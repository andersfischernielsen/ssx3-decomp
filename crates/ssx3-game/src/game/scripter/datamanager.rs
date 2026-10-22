//! `src/scripter/datamanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::Flow;
use crate::abi::r32;
use crate::call::sp;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use crate::call;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::dirtysock::tags2::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_2;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_5828, data_instance_man_ctor, "DataInstanceMan_ctor"),
    (0x0027_58c0, data_instance_man_dtor, "DataInstanceMan_dtor"),
    (0x0027_5b08, data_instance_man_remove_all, "DataInstanceMan_RemoveAll"),
    (0x0027_5b98, data_instance_man_add_big_entries, "DataInstanceMan_AddBigEntries"),
    (0x0027_5cd0, data_instance_man_free_info, "DataInstanceMan_FreeInfo"),
    (0x0027_5cf8, data_instance_man_alloc_info, "DataInstanceMan_AllocInfo"),
    (0x0027_5d10, script_list_ctor, "ScriptList_ctor"),
    (0x0027_5d90, script_list_dtor, "ScriptList_dtor"),
    (0x0027_5dd8, scriptlist_add, "ScriptList_Add"),
    (0x0027_6998, script_list_get_entry_type, "ScriptList_GetEntryType"),
    (0x0027_6b98, script_list_get_state, "ScriptList_GetState"),
    (0x0027_7060, script_list_can_skip, "ScriptList_CanSkip"),
    (0x0027_70c0, script_list_is_current_skippable, "ScriptList_IsCurrentSkippable"),
    (0x0027_71c8, scriptlist_is_playing_script, "ScriptList_IsPlayingScript"),
    (0x0027_7298, script_list_is_paused, "ScriptList_IsPaused"),
    (0x0027_72c0, script_list_on_seq_start, "ScriptList_OnSeqStart"),
    (0x0027_7310, script_list_on_seq_advance, "ScriptList_OnSeqAdvance"),
    (0x0027_73a0, script_list_on_seq_end, "ScriptList_OnSeqEnd"),
    (0x0027_7400, script_list_on_seq_stopped, "ScriptList_OnSeqStopped"),
    (0x0027_76e0, script_list_check_next_ready, "ScriptList_CheckNextReady"),
    (0x0027_7778, script_list_poll_loading, "ScriptList_PollLoading"),
    (0x0027_7dd8, script_list_is_movie_entry, "ScriptList_IsMovieEntry"),
    (0x0027_7f08, script_bank_slot_clear, "ScriptBankSlot_Clear"),
    (0x0027_80b8, script_engine_dtor, "ScriptEngine_dtor"),
    (0x0027_8308, script_engine_unload_master, "ScriptEngine_UnloadMaster"),
    (0x0028_3818, script_movie_stop, "ScriptMovie_Stop"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_DISCONNECT_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_CACHE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT as LOBBY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT as BUDDY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT as NET_FE_STATE_HANDLE_NET_EVENT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_3;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_3;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED as MENU_SYS_ITEM_IS_DISABLED_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR as SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
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

pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_INTERFACE;
    pub(crate) use crate::game::bx::bxstring2::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstring2::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstring2::addr::SPEECH_BANK_FREE_CALLBACK;
    pub(crate) use crate::game::bx::bxstring2::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::bx::bxstring2::addr::SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::addr::FMODF;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE_NOSIZE;
    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;
}


// ---------------------------------------------------------------------------
// Data instance manager.

/// `void DataInstanceMan_FreeInfo(DataInstanceMan *man, DataInfo *info)`
/// (0x275cd0): push `info` onto the free list `man->free` (+0x8), clearing the
/// record's fields and threading the old head through `info + 0x14`.
pub fn data_instance_man_free_info(c: &mut Cpu) {
    c.cycles += 9;
    const FREE: u32 = 0x8;
    const NEXT: u32 = 0x14;
    let man = Addr(abi::arg(c, 0));
    let info = Addr(abi::arg(c, 1));
    info.off(0x8).sw(c, 0);
    info.off(0xc).sw(c, 0);
    info.off(0x10).sw(c, 0);
    info.sw(c, 0);
    info.off(0x4).sw(c, 0);
    let head = man.off(FREE).lw(c);
    c.g[V0][0] = head;
    info.off(NEXT).sw(c, head);
    man.off(FREE).sw(c, abi::arg64(c, 1));
    abi::ret(c);
}


/// `DataInfo *DataInstanceMan_AllocInfo(DataInstanceMan *man)` (0x275cf8):
/// pop the free list `man->free` (+0x8), returning the node and clearing its
/// next pointer (+0x14).
pub fn data_instance_man_alloc_info(c: &mut Cpu) {
    c.cycles += 6;
    const FREE: u32 = 0x8;
    const NEXT: u32 = 0x14;
    let man = Addr(abi::arg(c, 0));
    let info = man.off(FREE).lw(c);
    c.g[V1][0] = info;
    let next = Addr(info as u32).off(NEXT).lw(c);
    c.g[A1][0] = next;
    c.g[V0][0] = info;
    man.off(FREE).sw(c, next);
    Addr(info as u32).off(NEXT).sw(c, 0);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script lists.

/// `bool ScriptList_IsPaused(ScriptList *list)` (0x277298): true when the state
/// at +0xa4 is 3 and the pending count at +0xa8 is non-zero.
pub fn script_list_is_paused(c: &mut Cpu) {
    c.cycles += 4;
    const STATE: u32 = 0xa4;
    const PENDING: u32 = 0xa8;
    const PAUSED: u64 = 3;
    let list = Addr(abi::arg(c, 0));
    c.g[V1][0] = list.off(STATE).lw(c);
    c.g[V0][0] = PAUSED;
    let mut result = 0u64;
    if c.g[V1][0] == PAUSED {
        c.cycles += 2;
        let pending = list.off(PENDING).lw(c);
        c.g[V0][0] = pending;
        result = (0u64 < pending) as u64;
    }
    c.g[A1][0] = result;
    c.cycles += 2;
    c.g[V0][0] = result;
    abi::ret(c);
}


/// `bool ScriptList_IsMovieEntry(int *entry)` (0x277dd8): true when the signed
/// word at `entry` is at least 29 (i.e. not a movie entry when below 29).
pub fn script_list_is_movie_entry(c: &mut Cpu) {
    c.cycles += 4;
    let value = Addr(abi::arg(c, 1)).lw(c);
    c.g[V0][0] = ((value as i64) < 29) as u64;
    c.g[V0][0] ^= 0x1;
    abi::ret(c);
}


/// `void ScriptBankSlot_Clear(ScriptBankSlot *slot)` (0x277f08): reset a bank
/// slot: id -1 at +0x4, fields +0x0/+0x8/+0xc cleared.
pub fn script_bank_slot_clear(c: &mut Cpu) {
    c.cycles += 6;
    let slot = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    slot.off(0xc).sw(c, 0);
    slot.off(0x4).sw(c, c.g[V0][0]);
    slot.sw(c, 0);
    slot.off(0x8).sw(c, 0);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptList_GetState(ScriptList* self)` (0x276B98): while the list is
/// loading (state +0xA4 == 1) poll the load, then return the state.
pub fn script_list_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let state = c.lw_r_2(V0, r(c, S0).wrapping_add(0xa4));
        if state == c.g[V1][0] {
            c.cycles += 4;
            callee::SCRIPT_LIST_POLL_LOADING.call(c, 0x0027_6bc0)?;
            let state = c.lw_r_2(V0, r(c, S0).wrapping_add(0xa4));
            let _ = state;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptList_IsCurrentSkippable(ScriptList* self)` (0x2770C0): only a
/// running list (state 3) can be skipped, and only when the current command's
/// flags at +0x8 have bit 1 set.
pub fn script_list_is_current_skippable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 3;
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let state = c.lw_r_2(V0, this.wrapping_add(0xa4));
        if state != c.g[V1][0] {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(this.wrapping_add(0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT.call(c, 0x0027_70e4)?;
            let cmd = c.lw_r_2(V0, r(c, V0).wrapping_add(8));
            let flags = sx(((cmd as u32) as i32 >> 2) as u32);
            c.g[V0][0] = flags & 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptList_dtor(self, flags)` (0x275d90): stop the script list, clear
/// its head and free it when `flags & 1`.
pub fn script_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_LIST_STOP.call(c, 0x0027_5db0)?;
        let self_ = abi::r32(c, S1);
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        Addr(self_).sw(c, 0);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0027_5dc4)?;
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
// Script engine.

/// `void ScriptList_OnSeqStopped(ScriptList* self, void* list, void*
/// script)` (0x277400): when `script` is still playing and its block exists,
/// notify the render-state manager with 0.
pub fn script_list_on_seq_stopped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S0][0];
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_7418)?;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            callee::SCRIPT_DATA_GET_INFO.call(c, 0x0027_7428)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            let no_flag = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x10);
            if !no_flag {
                c.cycles += 3;
                c.g[A0][0] = 0;
                call_ptr(c, 0x002e_f378, 0x0027_743c)?;
                frame.restore(c, S0, 0x10);
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script list / data manager.

/// `void ScriptList_OnSeqStart(ScriptList* list, void* seq, ScriptObj* obj)`
/// (0x2772c0): when the sequence's script is playing and it is a named script,
/// post the "sequence started" message from `D_004A34A8`.
/// 8 (+6, +2) + 1 + 3 cycles plus the callees.
pub fn script_list_on_seq_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        let obj = c.g[A2][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A1][0] = obj;
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2.call(c, 0x0027_72d8)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::SCRIPT_DATA_GET_INFO_2.call(c, 0x0027_72e8)?;
            c.g[V1][0] = Addr(r32(c, V0)).lb(c);
            c.g[V0][0] = sx(0x004a_0000);
            let unnamed = c.g[V1][0] == 0;
            c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x34a8));
            if !unnamed {
                c.cycles += 2;
                call_ptr(c, callee::MESSAGE_CENTER_MESSAGE, 0x0027_7300)?;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_UnloadMaster(ScriptEngine* e)` (0x278308): destroy the
/// script database, free the two object-table blocks at +0x52c and +0x534 and
/// clear them (and the +0x554 word). 10 (+3) + 7 cycles plus the callees.
pub fn script_engine_unload_master(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE.call(c, 0x0027_831c)?;
        c.g[A0][0] = c.lw(p(self_, 0x52c));
        call_ptr(c, callee::MEMORY_FREE, 0x0027_8324)?;
        let second = c.lw(p(self_, 0x534));
        c.g[A0][0] = second;
        if second == 0 {
            c.sw(p(self_, 0x554), 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::MEMORY_FREE, 0x0027_8338)?;
            c.sw(p(self_, 0x554), 0);
        }
        c.cycles += 7;
        c.sw(p(self_, 0x52c), 0);
        c.sw(p(self_, 0x530), 0);
        c.sw(p(self_, 0x534), 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void DataInstanceMan_dtor(DataInstanceMan* self, int flags)` (0x2758C0):
/// install the vtable, free the owned block at `+4`, and delete when bit 0 of
/// `flags` is set.
pub fn data_instance_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(ea(c.g[V0][0], 0x25a0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0xc), c.g[V0][0]);
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 4));
        let no_block = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_block {
            c.cycles += 2;
            call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x0027_58f4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x0027_5908)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script data manager.

/// `void ScriptList_OnSeqEnd(ScriptList* self, int a1, void* obj)` (0x2773a0):
/// if the list's current command is a movie entry that is not playing, hand
/// over to the next ready script; a paused script is resumed first.
pub fn script_list_on_seq_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        call_ptr(c, 0x0027_71c8, 0x0027_73c0)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p_2(c.g[S0][0], 0xb8));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0028_26d8, 0x0027_73e0)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_76e0, 0x0027_73e8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptList_CanSkip(ScriptList* list)` (0x277060): true when the
/// current command is skippable and the list has no pending delay.
pub fn script_list_can_skip(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let list = c.g[A0][0];
        c.g[S0][0] = list;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x3;
        c.g[V0][0] = Addr(at(c, S0, 0xa4)).lw(c);
        let not_script = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !not_script {
            'check: {
                c.cycles += 7;
                c.g[A0][0] = sx(at(c, S0, 0xc));
                c.g[A1][0] = 0;
                call_ptr(c, 0x0028_3d70, 0x0027_7090)?;
                c.g[V1][0] = Addr(r32_2(c, V0)).off(0x8).lw(c);
                c.g[V1][0] = c.g[V1][0] & 0x1;
                let not_skippable = c.g[V1][0] == 0;
                c.g[S1][0] = 0;
                if not_skippable {
                    break 'check;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(at(c, S0, 0xa8)).lw(c);
                c.g[S1][0] = (c.g[V0][0] < 0x1) as u64;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptList_GetEntryType(ScriptList* self, int i)` (0x276998): the
/// command type at_2 index `i` when it is in range and below 0x1c, else 0x1c.
pub fn script_list_get_entry_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = sx(at_2(c, A0, 0xc));
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_3d28, 0x0027_69b8)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        let mut result = 0x1c;
        if in_range {
            c.cycles += 6;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0028_3d70, 0x0027_69cc)?;
            c.g[V1][0] = Addr(at_2(c, V0, 0)).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < 28) as u64;
            let low = c.g[V1][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if low {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0028_3d70, 0x0027_69e4)?;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                result = c.g[V0][0] as u32;
            } else {
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = sx(result);
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void DataInstanceMan_RemoveAll(DataInstanceMan* m)` (0x275b08): remove
/// every live instance of the manager.
pub fn data_instance_man_remove_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_3(r32(c, S0), 4)).lw(c);
                c.g[A1][0] = abi::addu(c.g[S2][0], c.g[V0][0]);
                c.g[V1][0] = Addr(p_3(c.g[A1][0] as u32, 0x10)).lw(c);
                if c.g[V1][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call::call_ptr(c, 0x0027_5a20, 0x0027_5b4c)?;
                }
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5b30);
                }
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


/// `void ScriptList_ctor(self)` (0x00275d10): the script list constructor.
pub fn script_list_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(word(c, S0).wrapping_add(0xc));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::SCRIPT_CMD_QUEUE_CTOR.call(c, 0x0027_5d30)?;
        c.g[A0][0] = c.g[S1][0];
        callee::SCRIPT_CMD_QUEUE_CLEAR.call(c, 0x0027_5d38)?;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[V1][0] = 0x1;
        Addr(at_3(c, S0, 0xbc)).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at_3(c, S0, 0x4)).sw(c, c.g[V1][0]);
        for off in [0xa4, 0xa8, 0xc8, 0xb0, 0xac, 0xb8, 0xc0, 0xc4] {
            Addr(at_3(c, S0, off)).sw(c, 0);
        }
        Addr(at_3(c, S0, 0)).sw(c, 0);
        Addr(at_3(c, S0, 0xb4)).sw(c, 0);
        Addr(at_3(c, S0, 0x8)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script lists.

/// `int ScriptList_IsPlayingScript(self, process, script_obj_id)` (0x2771c8):
/// 0 unless the list is in state 3 and its current command is not a movie,
/// then whether `process` is the one running the command's script.
pub fn scriptlist_is_playing_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 3;
        frame.save(c, 16, 0x30);
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        let state = lw_at(c, p(r32(c, S0), 0xa4));
        c.g[V1][0] = state;
        let not_ready = state != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if not_ready {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = sx(p(r32(c, S0), 0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT_2.call(c, 0x0027_71fc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY.call(c, 0x0027_720c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 6;
                c.g[A1][0] = lw_at(c, p(r32(c, S1), 0xc));
                c.g[A0][0] = lw_at(c, gw(c, -0x84c));
                callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2.call(c, 0x0027_7220)?;
                c.g[V0][0] ^= c.g[S2][0];
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script list.

/// `void ScriptList_PollLoading(ScriptList* self)` (0x277778): while the list
/// waits for its movie/process command (state 1), start the command at the
/// queue head; the state becomes 2 when the command is a movie entry or its
/// process reports state 2.
pub fn script_list_poll_loading(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S0), 0xa4)).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 9;
            c.g[A0][0] = sx(p(r32(c, S0), 0xc));
            c.g[A1][0] = 0;
            callee::SCRIPT_CMD_QUEUE_AT_3.call(c, 0x0027_77a8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2.call(c, 0x0027_77b8)?;
            let movie = c.g[V0][0] != 0;
            c.g[V0][0] = 0x2;
            if movie {
                c.cycles += 1;
                Addr(p(r32(c, S0), 0xa4)).sw(c, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 7;
                c.g[A1][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                callee::SCRIPT_ENGINE_GET_PROCESS_STATE.call(c, 0x0027_77cc)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    Addr(p(r32(c, S0), 0xa4)).sw(c, c.g[V1][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                } else {
                    // The process is still pending: leave the list alone.
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script lists.

/// `void ScriptList_OnSeqAdvance(ScriptList* self, ScriptObj* obj)`
/// (0x277310): when the object is the list's playing script with no pending
/// transition and its remaining steps have run out, start the transition.
pub fn script_list_on_seq_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S1][0];
        callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3.call(c, 0x0027_7334)?;
        frame.restore(c, S0, 0x30);
        // Two of the four paths jump straight to the epilogue (their `lq16`
        // and `lq17` happen before the +1 cycle charge).
        let mut short_epilogue = false;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S2), 4)).lw(c);
            if c.g[V0][0] == 0 {
                short_epilogue = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x1c)).lw(c);
                if c.g[V0][0] != 0 {
                    short_epilogue = true;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    callee::SCRIPT_DATA_GET_INFO_4.call(c, 0x0027_735c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS.call(c, 0x0027_7368)?;
                    c.g[V1][0] = Addr(p(r32(c, S0), 0x12)).lh(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let exhausted = c.g[V1][0] != 0;
                    frame.restore(c, S0, 0x30);
                    if !exhausted {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        callee::SCRIPT_LIST_START_TRANSITION.call(c, 0x0027_7384)?;
                    }
                }
            }
        }
        if !short_epilogue {
            c.cycles += 1;
        }
        frame.restore(c, S1, 0x20);
        c.cycles += 4;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `DataInstanceMan_ctor` (0x275828): allocate and clear a table of `count`
/// 0x18-byte data-instance records.
pub fn data_instance_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S1, 0x20);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x25a0));
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = 0x18;
        c.mult(0, 0, 16, 4);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        Addr(this).sw(c, c.g[S0][0]);
        c.g[A1][0] = 0x0048_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x1ab8));
        c.g[A0][0] = c.lo[0];
        c.g[RA][0] = sx(0x0027_5874);
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW.call(c, 0x0027_5874)?;
        Addr(p(this, 4)).sw(c, c.g[V0][0]);
        let empty = (c.g[S0][0] as i64) <= 0;
        Addr(p(this, 8)).sw(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 7;
                c.g[A1][0] = Addr(p(this, 4)).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx(r(c, S0).wrapping_sub(1));
                c.g[RA][0] = sx(0x0027_589c);
                c.g[A1][0] = sx(r(c, A1).wrapping_add(r(c, S2)));
                callee::DATA_INSTANCE_FREE_INFO.call(c, 0x0027_589c)?;
                let more = c.g[S0][0] != 0;
                c.g[S2][0] = sx(r(c, S2).wrapping_add(0x18));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_5888);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptList_CheckNextReady` (0x2776e0): if the next script cmd is a movie
/// entry or its process is paused, pause the list.
pub fn script_list_check_next_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(p(this, 0xbc)).lw(c);
        let invalid = (c.g[A1][0] as i64) < 0;
        c.g[S2][0] = sx(this.wrapping_add(0xc));
        if !invalid {
            c.cycles += 8;
            c.g[RA][0] = sx(0x0027_770c);
            c.g[A0][0] = c.g[S2][0];
            callee::SCRIPT_CMD_QUEUE_AT_4.call(c, 0x0027_770c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0027_771c);
            c.g[A1][0] = c.g[S1][0];
            callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3.call(c, 0x0027_771c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                c.cycles += 1;
                c.g[V0][0] = sx(r(c, V0) >> 31);
            } else {
                c.cycles += 9;
                c.g[A1][0] = Addr(p(r(c, S0), 0xbc)).lw(c);
                c.g[RA][0] = sx(0x0027_7730);
                c.g[A0][0] = c.g[S2][0];
                callee::SCRIPT_CMD_QUEUE_AT_4.call(c, 0x0027_7730)?;
                c.g[A1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                c.g[RA][0] = sx(0x0027_773c);
                c.g[A0][0] = Addr(r(c, S0)).lw(c);
                callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2.call(c, 0x0027_773c)?;
                c.g[V0][0] = c.g[V0][0] ^ 1;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            let ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !ready {
                c.cycles += 3;
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0027_7760);
                c.g[A2][0] = 1;
                callee::SCRIPT_LIST_SET_PAUSED.call(c, 0x0027_7760)?;
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


/// `void ScriptMovie_Stop(ScriptMovie *self)` (0x283818): stop the movie
/// player and release its resources.
pub fn script_movie_stop(c: &mut Cpu) {
    guest(c, |c| {
        'b_2838c8: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
            if c.g[A0][0] == 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                break 'b_2838c8;
            } else {
                'b_2838c4: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0028383c);
                    c.g[A1][0] = 0x3;
                    call_ptr(c, 0x00253418, 0x0028383c)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                    if t {
                        break 'b_2838c4;
                    } else {
                        c.cycles += 26;
                        c.g[A0][0] = 0x530000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                        c.g[A1][0] = 0x440000;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V0][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[RA][0] = sx(0x00283880);
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                        call_ptr(c, 0x0028b180, 0x00283880)?;
                        c.g[RA][0] = sx(0x00283888);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0029ce70, 0x00283888)?;
                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                        vslot(c, c.g[V0][0], 0x140, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002838a4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002838a4)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        if c.g[V0][0] == 0u64 {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                            break 'b_2838c8;
                        } else {
                            c.cycles += 5;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            c.g[RA][0] = sx(0x002838c4);
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                            call_ptr(c, 0x002ea860, 0x002838c4)?;
                            break 'b_2838c4;
                        }
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                break 'b_2838c8;
            }
        }
        c.cycles += 7;
        c.sw((c.g[S0][0] as u32), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}


/// `ScriptList_Add(self, cmd, arg)` (0x275dd8): queue a script command on
/// the list: push a command record, and when the pending list is in its
/// running state, prepare the next entry and advance it. Returns 1 when
/// queued, 0 when the command queue is full.
pub fn scriptlist_add(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x50);
        frame.save(c, 18, 0x40);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A2][0];
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0027_5e08);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0028_3c58;
        ee::s07::ScriptCmdQueue_IsFull(c);
        if c.pc != 0x0027_5e08 {
            return Err(Unwound);
        }
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 14;
            c.g[V0][0] = sx(0xffff_ffff);
            c.sw(sp(c), c.g[S3][0]);
            c.sw(p(sp(c), 8), c.g[S2][0]);
            c.g[A0][0] = c.g[S1][0];
            c.sw(p(sp(c), 0x18), c.g[V0][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sw(p(sp(c), 4), c.g[V0][0]);
            c.sw(p(sp(c), 0xc), c.g[V0][0]);
            c.sw(p(sp(c), 0x10), c.g[V0][0]);
            c.g[RA][0] = sx(0x0027_5e3c);
            c.sw(p(sp(c), 0x14), c.g[V0][0]);
            c.pc = 0x0028_3c80;
            ee::s07::ScriptCmdQueue_Push(c);
            if c.pc != 0x0027_5e3c {
                return Err(Unwound);
            }
            c.g[V1][0] = lw_2(c, p(c.g[S0][0] as u32, 0xa4));
            let t = c.g[V1][0] == 0;
            c.g[V0][0] = 1;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xbc));
                let t = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if !t {
                    let mut direct = false;
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x0027_5e5c);
                    c.g[A0][0] = c.g[S1][0];
                    c.pc = 0x0028_3d28;
                    ee::s07::ScriptCmdQueue_Count(c);
                    if c.pc != 0x0027_5e5c {
                        return Err(Unwound);
                    }
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[RA][0] = sx(0x0027_5e6c);
                    c.g[A2][0] = 0;
                    c.pc = 0x0027_7450;
                    ee::s07::ScriptList_PrepareEntry(c);
                    if c.pc != 0x0027_5e6c {
                        return Err(Unwound);
                    }
                    c.g[A0][0] = lw_2(c, p(c.g[S0][0] as u32, 0xa4));
                    c.g[V1][0] = 3;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw(p(c.g[S0][0] as u32, 0xbc), c.g[V0][0]);
                    if !t {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0027_5e88);
                        c.g[A1][0] = 0;
                        c.pc = 0x0028_3d70;
                        ee::s07::ScriptCmdQueue_At(c);
                        if c.pc != 0x0027_5e88 {
                            return Err(Unwound);
                        }
                        c.g[A1][0] = lw_2(c, p(c.g[V0][0] as u32, 0xc));
                        c.g[RA][0] = sx(0x0027_5e94);
                        c.g[A0][0] = lw_2(c, c.g[S0][0] as u32);
                        c.pc = 0x0028_2bf0;
                        ee::s07::ScriptEngine_GetProcessInSeq(c);
                        if c.pc != 0x0027_5e94 {
                            return Err(Unwound);
                        }
                        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 0x20));
                        c.g[V1][0] ^= 4;
                        let t = c.g[V1][0] != 0;
                        c.g[V0][0] = 1;
                        if t {
                            direct = true;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x0027_5eac);
                            c.g[A0][0] = c.g[S0][0];
                            c.pc = 0x0027_76e0;
                            ee::s07::ScriptList_CheckNextReady(c);
                            if c.pc != 0x0027_5eac {
                                return Err(Unwound);
                            }
                        }
                    }
                    if !direct {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    }
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}


/// `sDmNode* func_00275B98(void* self, char* big, sDmNode* prev)` (0x275b98):
/// append every entry of the BIG archive `big` to the data-instance manager's
/// list, recursing into entries that are themselves archives, and emit each
/// added node through vtable slot 2.
pub fn data_instance_man_add_big_entries(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    'b_275ca0: {
        c.cycles += 25;
        frame.save(c, S3, 0x60);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S7, 0x20);
        c.g[A0][0] = c.g[S5][0];
        frame.save(c, S0, 0x90);
        c.g[A1][0] = 0u64;
        frame.save(c, S1, 0x80);
        c.g[A2][0] = 0u64;
        frame.save(c, S6, 0x30);
        c.g[T0][0] = c.g[SP][0] | 0x4;
        frame.save_ra(c, 0x10);
        c.g[S7][0] = 0x1;
        sw(c, r32(c, SP), 0u64);
        c.g[RA][0] = sx(0x00275bf0);
        sw(c, at_4(c, SP, 0x4), 0u64);
        if call_ptr(c, 0x003e2768, 0x00275bf0).is_err() {
            return;
        }
        c.g[S2][0] = c.g[V0][0];
        let t = c.g[S2][0] == 0u64;
        c.g[S6][0] = 0x450000;
        if t {
            break 'b_275ca0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = lw_3(c, r32(c, SP));
            'l_275c00: loop {
                'b_275c78: {
                    'b_275c34: {
                        c.cycles += 7;
                        c.g[S0][0] = lw_3(c, at_4(c, S6, 0xdd4));
                        c.g[S1][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
                        sw(c, at_4(c, S6, 0xdd4), 0u64);
                        c.g[RA][0] = sx(0x00275c14);
                        c.g[A0][0] = c.g[S1][0];
                        if call_ptr(c, 0x003e2580, 0x00275c14).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        sw(c, at_4(c, S6, 0xdd4), c.g[S0][0]);
                        if t {
                            break 'b_275c34;
                        } else {
                            c.cycles += 6;
                            c.g[A2][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00275c2c);
                            c.g[A0][0] = c.g[S4][0];
                            if callee::C00275b98.call(c, 0x00275c2c).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[S3][0] = c.g[V0][0];
                            if t {
                                break 'b_275c78;
                            } else {
                                break 'b_275c34;
                            }
                        }
                    }
                    c.cycles += 17;
                    c.g[RA][0] = sx(0x00275c3c);
                    c.g[A0][0] = c.g[S4][0];
                    if callee::C00275cf8.call(c, 0x00275c3c).is_err() {
                        return;
                    }
                    sw(c, at_4(c, V0, 0x8), c.g[S1][0]);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V1][0] = lw_3(c, at_4(c, SP, 0x4));
                    sw(c, at_4(c, V0, 0xc), c.g[V1][0]);
                    sw(c, at_4(c, S3, 0x14), c.g[V0][0]);
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = lw_3(c, at_4(c, S4, 0xc));
                    c.g[T0][0] = c.g[S3][0];
                    c.g[A3][0] = lw_3(c, at_4(c, SP, 0x4));
                    c.g[T1][0] = sx(at_4(c, S3, 0x4));
                    c.g[A0][0] = lh(c, at_4(c, V1, 0x10));
                    c.g[V0][0] = lw_3(c, at_4(c, V1, 0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00275c78);
                    c.g[A0][0] = sx(r32(c, S4).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x00275c78).is_err() {
                        return;
                    }
                    break 'b_275c78;
                }
                c.cycles += 10;
                c.g[A2][0] = c.g[S7][0];
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = 0u64;
                c.g[A3][0] = c.g[SP][0];
                c.g[T0][0] = 0u64;
                c.g[RA][0] = sx(0x00275c94);
                c.g[S7][0] = sx(at_4(c, S7, 0x1));
                if call_ptr(c, 0x003e2768, 0x00275c94).is_err() {
                    return;
                }
                c.g[S2][0] = c.g[V0][0];
                let t = c.g[S2][0] != 0u64;
                c.g[V0][0] = lw_3(c, r32(c, SP));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00275c00);
                    }
                    continue 'l_275c00;
                } else {
                    break 'b_275ca0;
                }
            }
        }
    }
    c.cycles += 12;
    c.g[V0][0] = c.g[S3][0];
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    c.g[RA][0] = ld(c, at_4(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0xa0));
    c.pc = t;
    return;
}


/// `void func_002780B8(void* self, int flags)` (0x2780b8): destroying
/// destructor of the script engine: install the base vtables, unload the
/// master script list, tear down the sound-bank, movie and script-object
/// managers, clear the per-frame state slots, and finally free the engine.
pub fn script_engine_dtor(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_278120: {
        c.cycles += 24;
        c.g[V0][0] = 0x480000;
        frame.save(c, S1, 0x50);
        c.g[V1][0] = 0x480000;
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[A0][0] = 0x480000;
        frame.save(c, S2, 0x40);
        c.g[A2][0] = 0x480000;
        frame.save(c, S3, 0x30);
        c.g[V0][0] = sx(at_4(c, V0, 0x2100));
        frame.save(c, S4, 0x20);
        c.g[V1][0] = sx(at_4(c, V1, 0x21a0));
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A0][0] = sx(at_4(c, A0, 0x2168));
        c.g[A2][0] = sx(at_4(c, A2, 0x21b8));
        c.g[S5][0] = c.g[A1][0];
        c.g[A3][0] = lw_3(c, at_4(c, S1, 0x52c));
        sw(c, at_4(c, S1, 0x2ac), c.g[V0][0]);
        sw(c, at_4(c, S1, 0x518), c.g[V1][0]);
        sw(c, at_4(c, S1, 0x528), c.g[A0][0]);
        let t = c.g[A3][0] == 0u64;
        sw(c, at_4(c, S1, 0x2a8), c.g[A2][0]);
        if t {
            break 'b_278120;
        } else {
            c.cycles += 2;
            c.g[RA][0] = sx(0x00278120);
            c.g[A0][0] = c.g[S1][0];
            if callee::C00278308.call(c, 0x00278120).is_err() {
                return;
            }
            break 'b_278120;
        }
    }
    'b_2781cc: {
        c.cycles += 31;
        c.g[A0][0] = sx(at_4(c, S1, 0xa38));
        c.g[A1][0] = 0x2;
        sw(c, at_4(c, GP, 0x3c0), 0u64);
        c.g[RA][0] = sx(0x00278134);
        c.g[S0][0] = sx(at_4(c, S1, 0x910));
        if callee::C00283518.call(c, 0x00278134).is_err() {
            return;
        }
        c.g[S2][0] = sx(at_4(c, S1, 0x6d0));
        c.g[A0][0] = sx(at_4(c, S1, 0xa28));
        c.g[RA][0] = sx(0x00278144);
        c.g[A1][0] = 0x2;
        if callee::C00275498.call(c, 0x00278144).is_err() {
            return;
        }
        c.g[S4][0] = sx(at_4(c, S1, 0x51c));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx(at_4(c, V0, 0x2298));
        c.g[V1][0] = sx(at_4(c, V1, 0x2260));
        sw(c, at_4(c, S1, 0x944), c.g[V0][0]);
        c.g[A0][0] = sx(at_4(c, S1, 0x938));
        sw(c, at_4(c, S1, 0x92c), c.g[V1][0]);
        c.g[RA][0] = sx(0x0027816c);
        c.g[A1][0] = 0u64;
        if callee::C00283228.call(c, 0x0027816c).is_err() {
            return;
        }
        c.g[S3][0] = sx(at_4(c, S1, 0x500));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = sx(at_4(c, V0, 0x2318));
        c.g[V1][0] = sx(at_4(c, V1, 0x22e0));
        sw(c, at_4(c, S1, 0x928), c.g[V0][0]);
        c.g[A0][0] = sx(at_4(c, S1, 0x91c));
        sw(c, at_4(c, S1, 0x910), c.g[V1][0]);
        c.g[RA][0] = sx(0x00278194);
        c.g[A1][0] = 0u64;
        if callee::C00283228.call(c, 0x00278194).is_err() {
            return;
        }
        let t = c.g[S2][0] == 0u64;
        c.g[A0][0] = c.g[S4][0];
        if t {
            break 'b_2781cc;
        } else {
            c.cycles += 2;
            let t = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x120));
            if t {
                break 'b_2781cc;
            } else {
                c.cycles += 1;
                'l_2781a8: loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = lw_3(c, at_4(c, S0, 0xc));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x8));
                    c.g[V1][0] = lw_3(c, at_4(c, V0, 0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002781c0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x002781c0).is_err() {
                        return;
                    }
                    let t = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x120));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002781a8);
                        }
                        continue 'l_2781a8;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S4][0];
                        break 'b_2781cc;
                    }
                }
            }
        }
    }
    c.cycles += 17;
    c.g[RA][0] = sx(0x002781d4);
    c.g[A1][0] = 0u64;
    if callee::C002758c0.call(c, 0x002781d4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S3][0];
    c.g[RA][0] = sx(0x002781e0);
    c.g[A1][0] = 0u64;
    if callee::C002837c8.call(c, 0x002781e0).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x002781ec);
    c.g[A1][0] = c.g[S5][0];
    if callee::C00281ed8.call(c, 0x002781ec).is_err() {
        return;
    }
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x70));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_vec;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::ptr;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::compare;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::run;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_4;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE as BE_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_8;
    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_9;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_10;


    #[test]
    fn data_instance_man_free_info_matches() {
        check(ssx3_ee::s07::f_00275cd0, data_instance_man_free_info, 200, &|c, seed| {
            let mut r = Rng(0x275c_d0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA + 0x8, r.next_u32());
            for i in 0..6 {
                c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn data_instance_man_alloc_info_matches() {
        check(ssx3_ee::s07::f_00275cf8, data_instance_man_alloc_info, 200, &|c, seed| {
            let mut r = Rng(0x275c_f8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let head = DATA + 0x100;
            c.bus.write32(DATA + 0x8, if seed % 5 == 0 { 0 } else { head });
            c.bus.write32(head + 0x14, if seed % 4 == 0 { 0 } else { DATA + 0x140 });
        });
    }


    #[test]
    fn script_list_is_paused_matches() {
        check(ssx3_ee::s07::f_00277298, script_list_is_paused, 200, &|c, seed| {
            let mut r = Rng(0x2772_98 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0xa4, if seed % 3 == 0 { 3 } else { r.next_u32() });
            c.bus.write32(DATA + 0xa8, if seed % 4 == 0 { 0 } else { r.next_u32() });
        });
    }


    #[test]
    fn script_list_is_movie_entry_matches() {
        check(ssx3_ee::s07::f_00277dd8, script_list_is_movie_entry, 200, &|c, seed| {
            let mut r = Rng(0x277d_d8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            let value = match seed % 4 {
                0 => 28,
                1 => 29,
                2 => 0x8000_0000,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA, value);
        });
    }


    #[test]
    fn script_bank_slot_clear_matches() {
        check(ssx3_ee::s07::f_00277f08, script_bank_slot_clear, 100, &|c, seed| {
            let mut r = Rng(0x277f_08 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    // ------------------------------------------------------ script engine

    #[test]
    fn script_list_get_state_matches() {
        check(ssx3_ee::s07::f_00276b98__generated, script_list_get_state, 60, &|c, seed| {
            machine(c, seed, 0x276b_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0xa4, if seed % 2 == 0 { 1 } else { 2 });
        });
    }


    #[test]
    fn script_list_is_current_skippable_matches() {
        check(ssx3_ee::s07::f_002770c0__generated, script_list_is_current_skippable, 60, &|c, seed| {
            machine(c, seed, 0x2770_c0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0xa4, if seed % 2 == 0 { 3 } else { 0 });
        });
    }


    #[test]
    fn script_list_dtor_matches() {
        check(ee::s07::f_00275d90__generated, script_list_dtor, 60, &|c, seed| {
            let mut r = call_machine(c, seed, 0x275d_90, 2);
            let flags = seed % 2;
            // The free branch runs on a null list (a bogus pointer would be
            // handed to the heap).
            let this = if flags != 0 { 0 } else { DATA + 0x100 };
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(flags);
            c.bus.write32(this + 0x10, r.next_u32());
        });
    }


    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_list_on_seq_stopped_matches() {
        check(
            ee::s07::f_00277400__generated,
            script_list_on_seq_stopped,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2774_00, 2);
                let list = DATA + 0x100;
                clear(c, list, 0x100);
                // Playing only when the list sits in play mode 3.
                c.bus.write32(list + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), 0);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = 0;
            },
        );
    }


    // ----------------------------------------------------------- script list

    #[test]
    fn script_list_on_seq_start_matches() {
        check(ee::s07::f_002772c0__generated, script_list_on_seq_start, 40, &|c, seed| {
            let mut r = base(c, seed, 0x2772);
            let list = DATA;
            let obj = DATA + 0x1000;
            c.bus.write32(list + 0xa4, if seed % 3 == 0 { 0 } else { 3 });
            c.bus.write32(obj + 0x14, DATA + 0x2000);
            c.bus.write8(DATA + 0x2000, if seed % 4 == 0 { 0 } else { b'x' });
            c.bus.write32(DATA + 0x2004, r.next_u32());
            c.bus.write8(0x004a_34a8, 0);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = sx(obj);
        });
    }


    // --------------------------------------------------------- script engine

    #[test]
    fn script_engine_unload_master_matches() {
        check(ee::s07::f_00278308__generated, script_engine_unload_master, 60, &|c, seed| {
            let mut r = base(c, seed, 0x2783);
            let e = DATA;
            // The database destroy is a no-op without a live database.
            c.bus.write32(e + 0x2b0, 0);
            fill(c, &mut r, e + 0x400, 0x100);
            c.bus.write32(e + 0x52c, if seed % 3 == 0 { 0 } else { 0x40 });
            c.bus.write32(e + 0x534, if seed % 2 == 0 { 0 } else { 0x80 });
            c.g[A0][0] = sx(e);
        });
    }


    #[test]
    fn data_instance_man_dtor_matches() {
        check(ee::s07::f_002758c0__generated, data_instance_man_dtor, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x0027_58c0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 4, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
    }


    // ------------------------------------------------------------------
    // Script.

    #[test]
    fn script_list_on_seq_end_matches() {
        check(ee::s07::f_002773a0__generated, script_list_on_seq_end, SEEDS_3, &|c, seed| {
            let mut r = base_2(c, seed, 0x2773_a0);
            let list = DATA + 0x100;
            clear(c, list, 0x100);
            w_2(c, list + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
            w_2(c, list + 0xb8, if seed % 4 < 2 { 0 } else { 1 });
            w_2(c, gp_at(-0x84c), 0);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = sx(DATA);
        });
    }


    #[test]
    fn script_list_can_skip_matches() {
        check(
            ssx3_ee::s07::f_00277060__generated,
            script_list_can_skip,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x277060, 0);
                w_3(c, OBJ + 0xa4, if seed % 3 == 0 { 3 } else { seed % 4 });
                w_3(c, OBJ + 0x8c, seed);
                w_3(c, OBJ + 0xa8, if seed % 2 == 0 { 0 } else { seed % 4 });
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    #[test]
    fn script_list_get_entry_type_matches() {
        check(
            ee::s07::f_00276998__generated,
            script_list_get_entry_type,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x2769_98);
                let list = DATA + 0xc;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 7) as u64;
                c.bus
                    .write32(list + 0x94, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write32(list + 0x8c, (seed % 5) as u32);
                c.bus.write32(list + 0x90, 5);
                for slot in 0..5u32 {
                    let v = if seed % 3 == 0 {
                        r.next_u32() % 64
                    } else {
                        (r.next_u32() % 30) + 2
                    };
                    c.bus.write32(list + slot * 0x1c, v);
                }
            },
        );
    }


    #[test]
    fn data_instance_man_remove_all_matches() {
        check(ssx3_ee::s07::f_00275b08__generated, data_instance_man_remove_all, SEEDS_4, &|c, seed| {
            let mut r = machine(c, seed, 0x275b08, 0);
            let m = DATA + 0x1000;
            let entries = DATA + 0x2000;
            let count = (seed % 5) as u32;
            c.g[A0][0] = m as u64;
            w32(c, m, count);
            w32(c, m + 4, entries);
            for i in 0..4u32 {
                w32(c, entries + i * 0x18 + 0x10, 0);
                w32(c, entries + i * 0x18, r.next_u32());
            }
        });
    }


    #[test]
    fn script_list_ctor_matches() {
        check(ee::s07::f_00275d10__generated, script_list_ctor, 12, &|c, seed| {
            let _ = alloc_base(c, seed, 0x275d_10, 4);
            c.g[A0][0] = DATA as u64;
        });
    }


    // -----------------------------------------------------------------------
    // Script lists.

    #[test]
    fn scriptlist_is_playing_script_matches() {
        check(
            ee::s07::f_002771c8__generated,
            scriptlist_is_playing_script,
            SEEDS_5,
            &|c, seed| {
                base_5(c, seed, 0x71c8, 0);
                let engine = DATA + 0x8800;
                let s0 = DATA + 0x100;
                c.g[A0][0] = sx(s0);
                c.g[A1][0] = (seed % 3) as u64;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(s0 + 0xa4, if seed % 4 == 0 { 0 } else { 3 });
                c.bus.write32(s0 + 0xc + 0x8c, 0);
                c.bus.write32(s0 + 0xc, if seed % 2 == 0 { 5 } else { 40 });
                c.bus.write32(s0 + 0xc + 0xc, (seed / 4 % 3) as u32);
                for id in 0..3u32 {
                    let p = DATA + 0x8900 + 0x40 * id;
                    c.bus.write32(engine + 0x2c0 + 0x24 * id + 0x20, p);
                    c.bus.write32(p, id * 7 + 1);
                }
            },
        );
    }


    // ------------------------------------------------------------------
    // Script list.

    #[test]
    fn script_list_poll_loading_matches() {
        check(ssx3_ee::s07::f_00277778__generated, script_list_poll_loading, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x2777_78, 4);
            let list = DATA + 0x100;
            let engine = DATA + 0x400;
            w(c, list, engine);
            let state = if seed % 4 == 0 { 0 } else { 1 };
            w(c, list + 0xa4, state);
            // ScriptCmdQueue_At(list+0xc, 0): read cursor at +0x8c, 28-byte slots.
            let read = r.next_u32() % 5;
            w(c, list + 0xc + 0x8c, read);
            let entry = list + 0xc + read * 0x1c;
            // ScriptList_IsMovieEntry reads the first word of the entry.
            let movie = seed % 5 == 0;
            w(c, entry, if movie { 29 + r.next_u32() % 4 } else { r.next_u32() % 29 });
            // ScriptEngine_GetProcessState(engine, entry[+0xc]) indexes a
            // 0x24-byte process record at engine+0x2c0; present at +0x20.
            let id = (r.next_u32() % 3) as u32;
            w(c, entry + 0xc, id);
            let proc = engine + 0x2c0 + id * 0x24;
            w(c, proc + 0x20, 1);
            w(c, proc + 4, if seed % 2 == 0 { 2 } else { 3 });
        });
    }


    #[test]
    fn script_list_on_seq_advance_matches() {
        check(ee::s07::f_00277310__generated, script_list_on_seq_advance, 120, &|c, seed| {
            let _ = base_6(c, seed, 0x27_7310);
            globals(c);
            // self+0x4 gates the whole thing; obj+0x1c is the pending
            // transition.
            w(c, DATA + 0x4, if seed % 2 == 0 { 0 } else { 1 });
            w(c, DATA + 0xb0, 0);
            w(c, DATA + 0xac, 0);
            w(c, DATA + 0x2000 + 0x1c, [0, 0, 1][seed as usize % 3]);
            w(c, DATA + 0x2000 + 0x12, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x2000);
            c.g[A2][0] = sx(DATA + 0x2000);
        });
    }


    #[test]
    fn data_instance_man_ctor_matches() {
        check(
            ee::s07::f_00275828__generated,
            data_instance_man_ctor,
            SEEDS_7,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (seed % 4) as u64;
            }),
        );
    }


    #[test]
    fn script_list_check_next_ready_matches() {
        check(
            ee::s07::f_002776e0__generated,
            script_list_check_next_ready,
            SEEDS_7,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(
                    c,
                    this + 0xbc,
                    if seed & 1 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 5) as u32
                    },
                );
                w(c, this + 0x8c, 0);
                w(c, this + 0x90, 0);
                w(c, this, DATA + 0x1100);
                let cmd = DATA + 0x1200;
                for i in 0..8 {
                    w(c, cmd + 4 * i, if i == 0 { 0x10 } else { 0 });
                }
                w(c, this + 0xc, cmd);
                c.g[A0][0] = this as u64;
            }),
        );
    }


    #[test]
    fn script_movie_stop_matches() {
        strict(
            ee::s07::f_00283818__generated,
            script_movie_stop,
            40,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2838);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x10, seed as u32);
                c.bus.write32(DATA + 0x14, (seed % 4 == 0) as u32);
                c.bus.write32(gp_at(-0x69c), (seed % 4) as u32);
                c.bus.write32(gp_at(-0x6a0), 0);
            },
        );
    }


    #[test]
    fn scriptlist_add_matches() {
        strict_2(ee::s07::f_00275dd8__generated, scriptlist_add, SEEDS_8, &|c, seed| {
            let mut r = base_9(c, seed, 0x5dd8, 0);
            let this = DATA + 0x100;
            // no pending list state: the call queues and returns 1
            w(c, this + 0xa4, 0);
            w(c, this + 0xbc, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = sx(r.next_u32());
        });
    }


    pub(crate) const DMAN: u32 = DATA + 0x1_0000;

    pub(crate) const BIG: u32 = DATA + 0x1_1000;


    #[test]
    fn data_instance_man_add_big_entries_matches() {
        check_all(
            ee::s07::f_00275b98__generated,
            data_instance_man_add_big_entries,
            20,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x275b_98, 10);
                c.g[A0][0] = sx(DMAN);
                c.g[A1][0] = sx(BIG);
                c.g[A2][0] = 0;
                // The archive walker: an empty archive stops immediately; one
                // seed gets a single non-archive entry.
                if seed % 2 == 0 {
                    put_words(c, BIG, &[0, 0, 0, 0]);
                } else {
                    put_words(c, BIG, &[1, 0, 0x20, 0]);
                }
                let vt5 = vtable(c, &mut r, &[0x10, 0x14]);
                c.bus.write32(DMAN + 0xc, vt5);
            },
        );
    }


    pub(crate) const ENGINE: u32 = DATA + 0x1_0000;


    #[test]
    fn script_engine_dtor_matches() {
        check_all(ee::s07::f_002780b8__generated, script_engine_dtor, 20, &|c, seed| {
            let _ = base_10(c, seed, 0x2780_b8, 16);
            c.g[A0][0] = sx(ENGINE);
            c.g[A1][0] = sx(seed % 4); // destructor flags
            c.bus.write32(ENGINE + 0x52c, (seed % 2) as u32);
            // The per-frame state slots at_4 +0x6d0..+0x910: no vtables, so the
            // slot destructor loop only decrements.
            c.bus.write32(ENGINE + 0x910, 0);
        });
    }
}
