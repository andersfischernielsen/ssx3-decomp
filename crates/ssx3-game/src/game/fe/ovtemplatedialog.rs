//! `src/fe/ovtemplatedialog.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::world::wscriptcache::p;
pub(crate) use crate::game::fe::festateloadhint::accept_key_6;
pub(crate) use crate::game::fe::festaterewards2::handle_list_key;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::movie::movieplayer::gp;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0020_a380, cov_state_overlay_init, "cOVStateOverlay_Init"),
    (0x0020_a8b0, game_ui_get_results_screen_id, "GameUI_GetResultsScreenId"),
    (0x0020_cbe8, game_ui_can_open_pause_menu, "GameUI_CanOpenPauseMenu"),
    (0x0020_cc60, game_ui_open_finish_screen, "GameUI_OpenFinishScreen"),
    (0x0020_d110, ov_screen_pda_template_open, "OVScreenPDATemplate_Open"),
    (0x0020_d308, ov_screen_overlay_menu_on_widget_notify, "OVScreenOverlayMenu_OnWidgetNotify"),
    (0x0020_d318, ov_screen_overlay_menu_on_widget_action, "OVScreenOverlayMenu_OnWidgetAction"),
    (0x0020_d4f0, ov_screen_yn_dialog_on_widget_created, "OVScreenYNDialog_OnWidgetCreated"),
    (0x0020_d568, ov_screen_yn_dialog_on_widget_notify, "OVScreenYNDialog_OnWidgetNotify"),
    (0x002c_26d0, ustr_sprintf, "UStr_Sprintf"),
    (0x0030_b8c0, w_script_man_is_mission_type2_active, "WScriptMan_IsMissionType2Active"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF;
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
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
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
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
}


/// `int GameUI_GetResultsScreenId(uint state, int mode)` (0x20a8b0): maps a
/// race state to the results screen id (0x16, 0xb or 0xa).
pub fn game_ui_get_results_screen_id(c: &mut Cpu) {
    let state = c.g[A0][0];
    let mode = c.g[A1][0];
    c.cycles += 4;
    c.g[V0][0] = sx((state as u32).wrapping_sub(6));
    c.g[V0][0] = (c.g[V0][0] < 6) as u64;
    let not_six_to_eleven = c.g[V0][0] == 0;
    c.g[V0][0] = sx((state as u32).wrapping_sub(4));
    if not_six_to_eleven {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            let is_zero = state == 0;
            c.g[V0][0] = 3;
            if !is_zero {
                c.cycles += 2;
                if mode != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = 0xa;
                    abi::ret(c);
                    return;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0xb;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x16;
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Formatting and the MPEG2 bit reader.

/// `int UStr_Sprintf(wchar_t* dst, const wchar_t* fmt, ...)` (0x2c26d0): the
/// varargs shim: spill `a2..a3`, `t0..t3` (at `+0x50`) and `f12`, `f14`,
/// `f16`, `f18` (at `+0x40`) into the frame, then `USTR_vsprintf(dst, fmt,
/// &a2)`.
pub fn ustr_sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.sd(p(sp(c), 0x50), c.g[A2][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(p(sp(c), 0x50));
        c.sd(p(sp(c), 0x58), c.g[A3][0]);
        c.sd(p(sp(c), 0x60), c.g[T0][0]);
        c.sd(p(sp(c), 0x68), c.g[T1][0]);
        c.sd(p(sp(c), 0x70), c.g[T2][0]);
        c.sd(p(sp(c), 0x78), c.g[T3][0]);
        c.swc1(12, p(sp(c), 0x40));
        c.swc1(14, p(sp(c), 0x44));
        c.swc1(16, p(sp(c), 0x48));
        c.swc1(18, p(sp(c), 0x4c));
        call_ptr(c, 0x003d_c450, 0x002c_2708)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int OVScreenOverlayMenu_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x0020d308).
pub fn ov_screen_overlay_menu_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}


/// `int OVScreenYNDialog_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x0020d568).
pub fn ov_screen_yn_dialog_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}


/// `void OVScreenOverlayMenu_OnWidgetAction(OVScreen*, void* item, int action)`
/// (0x0020d318): picking item action 5 sets the global overlay-menu flag
/// (`gp-0x9f4`, 0x4a26fc).
pub fn ov_screen_overlay_menu_on_widget_action(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 5;
    if abi::arg64(c, 1) != 0 {
        c.cycles += 2;
        let action = abi::arg64(c, 2);
        c.g[V0][0] = 1;
        if action == 5 {
            c.cycles += 1;
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `WScriptMan_IsMissionType2Active(man)` 0x30b8c0: walk the node list at
/// `+0x2b8` and report whether any active node has mission type 2.
pub fn w_script_man_is_mission_type2_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        let this = abi::arg(c, 0);
        c.g[S0][0] = lw_at(c, this, 0x2b8);
        c.g[S1][0] = 0;
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x5c)).lw(c);
            loop {
                c.cycles += 6;
                let vt = r32(c, V1);
                c.g[A0][0] = Addr(p(vt, 0x18)).lh(c);
                c.g[V0][0] = lw_at(c, vt, 0x1c);
                let func = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, func, 0x0030_b8f0)?;
                let active = c.g[V0][0] != 0;
                c.g[V1][0] = sx(r32(c, S1).wrapping_add(1));
                if active {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x60)).lw(c);
                    c.g[V0][0] ^= 2;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = c.g[V1][0];
                    }
                }
                c.cycles += 3;
                c.g[S0][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(p(r32(c, S0), 0x5c)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_b8e0);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = (0u64 < c.g[S1][0]) as u64;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_0020CBE8` (0x20cbe8): `GameUI_CanOpenPauseMenu`. False while the
/// pause menu is open; otherwise looks up the widget at
/// `((world->+0x84)->+0x48) + 0x18` (`0x39fa88`) and returns true only if the
/// widget exists, is not disabled (`+0x1c & 1`) and its `vtable[0xc8]`
/// predicate says so. Signature: `bool can_open(world)`.
pub fn game_ui_can_open_pause_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_r(c, V0, 0x84);
        c.g[RA][0] = sx(0x0020_cc04);
        c.g[S0][0] = lw_r(c, V1, 0x48);
        call_ptr(c, 0x0020_cba0, 0x0020_cc04)?;
        let open = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        let mut tail_charge = true;
        if !open {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0020_cc14);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(0x18));
            call_ptr(c, 0x0039_fa88, 0x0020_cc14)?;
            c.g[A1][0] = c.g[V0][0];
            let missing = c.g[A1][0] == 0;
            c.g[V0][0] = 0x1;
            if !missing {
                c.cycles += 4;
                c.g[V0][0] = lw_r(c, A1, 0x1c);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let disabled = c.g[V0][0] != 0;
                c.g[V0][0] = 0;
                if !disabled {
                    c.cycles += 7;
                    c.g[V1][0] = lw_r(c, A1, 0x8);
                    c.g[A0][0] = lh_r(c, V1, 0xc8);
                    c.g[V0][0] = lw_r(c, V1, 0xcc);
                    let target = low(c, V0);
                    c.g[RA][0] = sx(0x0020_cc44);
                    c.g[A0][0] = sx(low(c, A1).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x0020_cc44)?;
                    tail_charge = false;
                }
            }
        }
        if tail_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_0020D4F0` (0x20d4f0): `OVScreenYNDialog::OnWidgetCreated`. If the
/// created widget (`a1+0x38`) is the yes/no dialog's button ("0x46e818"),
/// enables it when `this->+0x9c == 6` (`0x39b960(widget, 0)`) or leaves it
/// disabled otherwise (`0x39b960(widget, 1)`).
/// Signature: `void on_widget_created(this, widget)`.
pub fn ov_screen_yn_dialog_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x47_0000;
        c.g[S0][0] = lw_r(c, S1, 0x38);
        c.g[RA][0] = sx(0x0020_d51c);
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0x17e8));
        call_ptr(c, 0x0031_7670, 0x0020_d51c)?;
        let other = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x30);
        if other {
            c.cycles += 5;
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V1][0] = lw_r(c, S2, 0x9c);
        c.g[V0][0] = 0x6;
        let enabled = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if enabled {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0020_d54c);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0039_b960, 0x0020_d54c)?;
        } else {
            c.cycles += 4;
            c.g[RA][0] = sx(0x0020_d53c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0039_b960, 0x0020_d53c)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenPDATemplate_Open(void)` (0x0020d110): once per process, add
/// the PDA template background state.
pub fn ov_screen_pda_template_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp(c, -0x9e4)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let already = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !already {
            c.cycles += 20;
            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A1][0] = sx(0x470000);
            c.g[A1][0] = sx(0x470000u32.wrapping_add(0x1a10));
            c.g[A0][0] = 0x48;
            c.g[V0][0] = Addr(at(c, V1, 0x84)).lw(c);
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[S1][0] = Addr(at(c, V0, 0x48)).lw(c);
            callee::MEM_ALLOC_NAMED.call(c, 0x0020_d14c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            callee::UI_STATE_CTOR.call(c, 0x0020_d15c)?;
            c.g[V1][0] = sx(0x470000);
            c.g[A0][0] = sx(word(c, S1).wrapping_add(0x18));
            c.g[V1][0] = sx(0x470000u32.wrapping_add(0x43f0));
            c.g[A1][0] = c.g[S0][0];
            Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
            Addr(gp(c, -0x9e4)).sw(c, c.g[S0][0]);
            callee::UI_STATE_MGR_ADD_BACKGROUND_STATE.call(c, 0x0020_d178)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void GameUI_OpenFinishScreen(void)` (0x20cc60): open the finish screen that
/// matches the current race mode and player count.
pub fn game_ui_open_finish_screen(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table_callee::BE_GET, 0x0020_cc70)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x0020_cc7c)?;
        c.g[V1][0] = 0x530000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x49).lb(c);
        c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0x48).lb(c);
        let no_mode = c.g[V1][0] == 0;
        c.g[A0][0] = 2;
        let mut stage = 0u8;
        if no_mode {
            stage = 1;
        } else {
            c.cycles += 2;
            let two_players = c.g[V0][0] != c.g[A0][0];
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            stage = if two_players { 2 } else { 1 };
        }
        if stage == 2 {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            let one_to_three = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if one_to_three {
                stage = 3;
            } else {
                c.cycles += 2;
                let not_six = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 4;
                stage = if not_six { 4 } else { 3 };
            }
        }
        match stage {
            1 => {
                c.cycles += 4;
                c.g[A0][0] = 0xc;
                callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_cca4)?;
                frame.restore_ra(c, 0);
            }
            3 => {
                c.cycles += 4;
                c.g[A0][0] = 0xd;
                callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_ccc8)?;
                frame.restore_ra(c, 0);
            }
            _ => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[A0][0] = 0xc;
                    callee::GAME_UI_OPEN_SCREEN.call(c, 0x0020_cce8)?;
                } else {
                    c.cycles += 2;
                    Addr(gp_2(c, -0x9f4)).sw(c, c.g[A0][0]);
                }
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `OVScreenYNDialogQuitGame_Update` (0x20dbd0): the quit-game yes/no dialog
/// update: run the overlay update and toggle the option thread's value.
pub fn ov_screen_yn_dialog_quit_game_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0020_dbe4);
        c.g[S0][0] = c.g[A0][0];
        callee::OVERLAY_UPDATE_CALLEE.call(c, 0x0020_dbe4)?;
        c.g[V1][0] = Addr(gp_3(c, -0x204)).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, V1), 0x68)).lw(c);
            let zero68 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if zero68 {
                c.cycles += 1;
                c.g[A0][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, V1), 0x64)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[A0][0] = 1;
                }
            }
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 10;
                c.g[A0][0] = 0x0047_0000;
                c.g[RA][0] = sx(0x0020_dc20);
                c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x1fb0));
                call_ptr(c, callee::FE_STRING_HASH, 0x0020_dc20)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x0020_dc2c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_LOOKUP_VALUE, 0x0020_dc2c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let is_ffff = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !is_ffff {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                    c.g[RA][0] = sx(0x0020_dc48);
                    c.g[A2][0] = 1;
                    call_ptr(c, callee::UI_SCREEN_LAUNCH_THREAD, 0x0020_dc48)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0020_dc58);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_SCREEN_SET_OBJECT_VALUE, 0x0020_dc58)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cOVStateOverlay_Init(self)` (0x20a380): initialise the online
/// overlay state, asking the current race manager's overlay object (if any)
/// to take effect.
pub fn cov_state_overlay_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
        c.sw(p(this as u32, 0x50), 0);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x84)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0x64)).lw(c);
            c.g[S0][0] = Addr(p(c.g[A0][0] as u32, 0x10)).lw(c);
            if c.g[S0][0] == 0 {
                c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
            } else {
                c.cycles += 5;
                call_ptr(c, 0x002e_a820, 0x0020_a3c0)?;
                c.g[V1][0] = 0x1;
                let one = c.g[S0][0] == c.g[V1][0];
                c.g[V0][0] = 0x3;
                let mut enable = one;
                if !one {
                    c.cycles += 2;
                    if c.g[S0][0] == c.g[V0][0] {
                        enable = true;
                    } else {
                        c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
                    }
                }
                if enable {
                    c.cycles += 2;
                    c.sw(p(this as u32, 0x50), c.g[V1][0]);
                    c.g[S0][0] = Addr(p(this as u32, 0x10)).lw(c);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0031_7670, 0x0020_a3ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_f9d8, 0x0020_a3f8)?;
        c.g[A3][0] = c.g[V0][0];
        let missing = c.g[A3][0] == 0;
        c.g[A1][0] = 0x3;
        if !missing {
            c.cycles += 6;
            c.g[V1][0] = Addr(p(c.g[A3][0] as u32, 8)).lw(c);
            c.g[A2][0] = 0;
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0xc0)).lh(c);
            c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0xc4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_a41c)?;
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
// Replay start.

/// `void Replay_BeginPlayback(Replay* self, int a1, int a2, int a3)`
/// (0x26f8a0): start the replay from a snapshot unless one is already
/// running, then switch the replay cameras and tell the audio system.
pub fn replay_begin_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xe));
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let busy = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A3][0];
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x608)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                call_ptr(c, 0x0026_f7f8, 0x0026_f8f0)?;
            }
            c.cycles += 13;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0026_f4a0, 0x0026_f8fc)?;
            Addr(p(r32(c, S0), 0x618)).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            Addr(p(r32(c, S0), 0x61c)).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
            Addr(p(r32(c, S0), 0x620)).sw(c, c.g[S1][0]);
            call_ptr(c, 0x0027_02f8, 0x0026_f914)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0027_03f0, 0x0026_f920)?;
            if c.g[S1][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ef30, 0x0026_f930)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_ee68, 0x0026_f940)?;
            }
            c.cycles += 2;
            let have_a2 = c.g[S1][0] != 0;
            c.lq(S0, frame.at(c, 0x40));
            if have_a2 {
                c.cycles += 1;
                c.lq(S1, frame.at(c, 0x30));
            } else {
                c.cycles += 2;
                let have_a3 = c.g[S3][0] != 0;
                c.lq(S1, frame.at(c, 0x30));
                if !have_a3 {
                    c.cycles += 4;
                    call_ptr(c, 0x0028_b180, 0x0026_f958)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028_f200, 0x0026_f960)?;
                    c.cycles += 1;
                    c.lq(S0, frame.at(c, 0x40));
                    c.cycles += 1;
                    c.lq(S1, frame.at(c, 0x30));
                }
            }
        } else {
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x40));
            c.cycles += 1;
            c.lq(S1, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
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
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes;
    use ssx3_ee as ee;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::SP;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::ovstatemap::tests::run;
    pub(crate) use crate::game::fe::ovstatemap::tests::str_hash;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::fe::festaterewards::tests::w as w_4;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_2;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_3;


    #[test]
    fn game_ui_get_results_screen_id_matches() {
        check(ssx3_ee::s05::f_0020a8b0, game_ui_get_results_screen_id, 400, &|c, seed| {
            let mut r = Rng(0x0c10_0011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = match seed % 6 {
                0 => 6 + (seed as u64 % 6),
                1 => 4 + (seed as u64 % 2),
                2 => 0,
                3 => 2,
                4 => (1u64 << 32) | 2,
                _ => (r.next_u32() % 16) as u64,
            };
            c.g[A1][0] = if seed % 2 == 0 { 3 } else { (r.next_u32() % 5) as u64 };
        });
    }


    #[test]
    fn ustr_sprintf_matches() {
        check(ee::s08::f_002c26d0__generated, ustr_sprintf, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2c_26d0, 0);
            let dst = DATA + 0x1000;
            let fmt = DATA + 0x2000;
            let spec = [0x25u16, 0x64, 0x20, 0x25, 0x69, 0x0a, 0];
            for (i, w) in spec.iter().enumerate() {
                c.bus.write16(fmt + 2 * i as u32, *w);
            }
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(fmt);
            c.g[A2][0] = (r.next_u32() % 0x1000) as u64;
            c.g[A3][0] = (r.next_u32() % 0x1000) as u64;
            c.f[12] = r.edge_f32();
        });
    }


    #[test]
    fn ov_screen_overlay_menu_on_widget_action_matches() {
        check(ssx3_ee::s05::f_0020d318__generated, ov_screen_overlay_menu_on_widget_action, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x20d318, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x10) };
            c.g[A2][0] = if seed % 2 == 0 { 5 } else { (seed % 8) as u64 };
        });
    }


    /// Tiny generated leaves used as deterministic vtable targets.
    pub(crate) const ZERO_STUB: u32 = 0x0020_2768;
 // `EventSelect_HandleInput`: returns 0
    pub(crate) const ONE_STUB: u32 = 0x0021_cb68;


    #[test]
    fn w_script_man_is_mission_type2_active_matches() {
        check(
            ee::s10::f_0030b8c0__generated,
            w_script_man_is_mission_type2_active,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x30b8c0, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                if seed % 4 == 0 {
                    w(c, OBJ + 0x2b8, 0);
                } else {
                    w(c, OBJ + 0x2b8, OBJ2);
                    w(c, OBJ2 + 0x5c, VT);
                    slot(
                        c,
                        VT,
                        0x18,
                        0,
                        if seed % 2 == 0 { ONE_STUB } else { ZERO_STUB },
                    );
                    w(c, OBJ2 + 0x60, if seed % 3 == 0 { 2 } else { 3 });
                    let more = seed % 4 == 1;
                    w(c, OBJ2 + 0x18, if more { OBJ3 } else { 0 });
                    w(c, OBJ3 + 0x5c, VT);
                    slot(c, VT + 0x40, 0x18, 0, ONE_STUB);
                    w(c, OBJ3 + 0x60, 2);
                    w(c, OBJ3 + 0x18, 0);
                }
            },
        );
    }


    #[test]
    fn game_ui_can_open_pause_menu_matches() {
        check(
            ee::s05::f_0020cbe8__generated,
            game_ui_can_open_pause_menu,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x20cbe8, 5);
                be_library(c);
                heap_init(c);
                world(c);
                let race = DATA + 0x1200;
                if seed % 2 == 0 {
                    w_2(c, race + 0x48, 0);
                } else {
                    w_2(c, race + 0x48, DATA + 0x400);
                }
            },
        );
    }


    #[test]
    fn ov_screen_yn_dialog_on_widget_created_matches() {
        check(
            ee::s05::f_0020d4f0__generated,
            ov_screen_yn_dialog_on_widget_created,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x20d4f0, 5);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let widget = DATA + 0x400;
                // `StrHash` clobbers the argument registers: hash first.
                let want = if seed % 2 == 0 {
                    str_hash(c, 0x0046_e818)
                } else {
                    0x1234
                };
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                w_2(c, widget + 0x38, want);
                w_2(c, this + 0x9c, if seed % 3 == 0 { 6 } else { 5 });
            },
        );
    }


    #[test]
    fn ov_screen_pda_template_open_matches() {
        check(
            ee::s05::f_0020d110__generated,
            ov_screen_pda_template_open,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x20d1_10, 4);
                w_3(
                    c,
                    GAME_GP.wrapping_sub(0x9e4),
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                let game = W + 0x1000;
                w_3(c, game + 0x48, DATA + 0x2000);
                c.g[A0][0] = (seed + 1) as u64;
            },
        );
    }


            #[test]
    fn game_ui_open_finish_screen_matches() {
        check(
            ee::s05::f_0020cc60__generated,
            game_ui_open_finish_screen,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x20cc60, 4);
                w_4(c, 0x0053_5bc8 + 0x48, (seed % 8) as u32);
                w_4(c, 0x0053_5bc8 + 0x49, (seed / 8 % 4) as u32);
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn cov_state_overlay_init_matches() {
        check(ee::s05::f_0020a380__generated, cov_state_overlay_init, 6, &|c, seed| {
            let _ = base_3(c, seed, 0x0020_a380, 3);
            c.g[A0][0] = OBJ_2 as u64;
            w_5(c, OBJ_2 + 0x50, 0);
            w_5(c, OBJ_2 + 0x10, OBJ2_2);
            let game = WORLD + 0x1000;
            w_5(c, game + 0x64, if seed % 3 == 0 { 0 } else { OBJ3_2 });
            w_5(c, OBJ3_2 + 0x10, match seed % 4 {
                0 => 1,
                1 => 3,
                _ => OBJ4_2,
            });
            w_5(c, OBJ2_2 + 0x18, 0);
        });
    }
}
