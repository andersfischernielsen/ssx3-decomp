//! `src/object/multiparticle.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::abi::r32;
use crate::call::guest;
use callee::*;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
pub(crate) use crate::game::ai::aianimeventmap::BUILTIN_DELETE;
pub(crate) use crate::game::animation::rideranimbase::OPERATOR_NEW_NAMED2_ADDR;
pub(crate) use crate::game::animation::animmodel::EA_MEM_FREE_ADDR;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::intersect::worldsphtree::save_field_block;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::fe::festaterewards::at_4 as at;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::sound::soundsys::p as p_5;
pub(crate) use crate::game::sound::soundsys::gp_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::feasyncfile::addu;
pub(crate) use crate::game::fe::fepopup3::p as p_7;
pub(crate) use crate::game::fe::fepopup3::gp_6 as gp_5;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::fepopup::ld;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_8120, multi_particle_update, "MultiParticle_Update"),
    (0x0035_8140, multi_particle_remove_section_instances, "MultiParticle_RemoveSectionInstances"),
    (0x0035_81b8, multi_particle_add_instance, "MultiParticle_AddInstance"),
    (0x0035_81f0, multi_particle_remove_instance, "MultiParticle_RemoveInstance"),
    (0x0035_84f0, rail_man_ctor, "RailMan_ctor"),
    (0x0035_8588, railman_ctor_2, "RailMan_ctor_2"),
    (0x0035_8668, rail_man_dtor, "RailMan_dtor"),
    (0x0035_8700, railman_on_section_unload, "RailMan_OnSectionUnload"),
    (0x0035_8998, rail_man_enable_rail, "RailMan_EnableRail"),
    (0x0035_8c30, rail_man_save, "RailMan_Save"),
    (0x0035_8db0, anim_combo_ctor_2, "AnimCombo_ctor_2"),
    (0x0035_8e50, anim_combo_dtor, "AnimCombo_dtor"),
    (0x0035_8f90, anim_combo_v42, "AnimCombo_v42"),
    (0x0035_8fd0, anim_combo_v36, "AnimCombo_v36"),
    (0x0035_9030, anim_combo_snapshot_matrices, "AnimCombo_SnapshotMatrices"),
    (0x0035_90c8, anim_combo_kill, "AnimCombo_Kill"),
    (0x0035_90f0, anim_combo_save, "AnimCombo_Save"),
    (0x0035_9140, anim_combo_v52_2, "AnimCombo_v52"),
    (0x0035_9228, anim_delta_ctor, "AnimDelta_ctor"),
    (0x0035_93d0, anim_delta_v36, "AnimDelta_v36"),
    (0x0035_9410, anim_delta_save, "AnimDelta_Save"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

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
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_2;
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
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
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

    pub const ANIM_OBJECT_CTOR: Callee = Callee {
        addr: 0x0034_1aa0,
        f: ee::s11::f_00341aa0,
    };
}

// ---------------------------------------------------------------------------
// AnimCombo_v52 (0x00359140).

/// `cAnimCombo` fields used here.
pub(crate) mod anim_combo {

    /// `+0x14`/`+0x15`/`+0x17`: per-part enable bytes.
    pub const PART_A: u32 = 0x14;

    pub const PART_B: u32 = 0x15;

    pub const PART_C: u32 = 0x17;

    /// The object whose vertex arrays are blended.
    pub const MODEL: u32 = 0x60;

    /// The animation object and its per-part output matrices.
    pub const ANIM: u32 = 0x7c;

    pub const MATRICES: u32 = 0x84;
}


// ---------------------------------------------------------------------------
// Particles / modifiers.

/// `void MultiParticle_AddInstance(MultiParticle *p, void *inst)`
/// (0x3581b8): append `inst` to the instance array if there is room.
pub fn multi_particle_add_instance(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[A1][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

pub(crate) const ANIM_OBJECT_DTOR: Callee = Callee { addr: 0x0034_1cf0, f: ee::s11::f_00341cf0 };

pub(crate) const MOD_OBJ_V42: Callee = Callee { addr: 0x0035_6ae0, f: ee::s11::f_00356ae0 };

pub(crate) const WORLD_OBJ_KILL: Callee = Callee { addr: 0x0034_fd90, f: ee::s11::f_0034fd90 };

pub(crate) const ANIM_OBJECT_SAVE: Callee = Callee { addr: 0x0034_2150, f: ee::s11::f_00342150 };

pub(crate) const ANIM_OBJECT_V52: Callee = Callee { addr: 0x0034_e348, f: ee::s11::f_0034e348 };

pub(crate) const ANIM_OBJECT_CTOR_2: Callee = Callee { addr: 0x0034_1c80, f: ee::s11::f_00341c80 };

pub(crate) const ANIM_COMBO_SNAPSHOT: Callee = Callee { addr: 0x0035_9030, f: ee::s11::f_00359030 };

pub(crate) const INTERSECT_INFO_MUL_MATRIX: Callee = Callee { addr: 0x0035_1948, f: ee::s11::f_00351948 };


/// `AnimCombo_dtor(AnimCombo *this, uint flags)` (0x358e50): set the vtable at
/// +0x54, free the snapshot matrices (+0x84), destruct the `AnimObject` at
/// +0x18 and, when `flags & 1`, delete `this`.
pub fn anim_combo_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x49_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x49_0000u32.wrapping_sub(0x13d8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
        let no_snap = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_snap {
            c.cycles += 2;
            call_ptr(c, EA_MEM_FREE_ADDR, 0x0035_8e84)?;
        }
        c.cycles += 6;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = 0;
        ANIM_OBJECT_DTOR.call(c, 0x0035_8e90)?;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            BUILTIN_DELETE.call(c, 0x0035_8ea4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `AnimCombo_v42(SubObj *sub, void *out)` (0x358f90): call `ModObj_v42` on
/// `sub+0x48`, then copy the two fixed quads at 0x4ff120 to `out+0x20` and
/// `out+0x30`.
pub fn anim_combo_v42(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        MOD_OBJ_V42.call(c, 0x0035_8fa8)?;
        c.g[A0][0] = 0x50_0000;
        frame.restore_ra(c, 0);
        c.g[A0][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
        c.lq(2, c.g[A0][0] as u32);
        c.sq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
        c.lq(3, c.g[A0][0] as u32);
        c.sq(3, (c.g[S0][0] as u32).wrapping_add(0x30));
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `AnimCombo *AnimCombo_ctor_2(AnimCombo *this, void *a1, void *a2)`
/// (0x358db0): construct the `AnimObject` at +0x18, run the vtable slot at
/// `*a2 -> +0x10/+0x14`, allocate the snapshot buffer from the bone count and
/// snapshot the matrices.
pub fn anim_combo_ctor_2(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
        ANIM_OBJECT_CTOR_2.call(c, 0x0035_8dd8)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = sx(0x49_0000u32.wrapping_sub(0x13d8));
        c.g[A2][0] = 0x18;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, 0x0035_8e00)?;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x74)).lw(c);
        c.g[A1][0] = 0x49_0000;
        c.g[A1][0] = sx(0x49_0000u32.wrapping_sub(0x15e0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
        c.g[A3][0] = 0;
        call_ptr(c, OPERATOR_NEW_NAMED2_ADDR, 0x0035_8e1c)?;
        let empty = c.g[S0][0] == 0;
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = u64::MAX;
        }
        c.cycles += 10;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[S1][0];
        ANIM_COMBO_SNAPSHOT.call(c, 0x0035_8e34)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimCombo_Kill(AnimCombo *this)` (0x3590c8): kill the `WorldObj` at
/// +0x48 unless the alive flag at +0x15 is set.
pub fn anim_combo_kill(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x15)).lb(c);
        let alive = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !alive {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
            WORLD_OBJ_KILL.call(c, 0x0035_90e4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void AnimCombo_Save(AnimCombo *this, State *out)` (0x3590f0): save the
/// `AnimObject` at +0x18 and forward to `out->save` (`out+0 -> +8/+0xc`).
pub fn anim_combo_save(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
        ANIM_OBJECT_SAVE.call(c, 0x0035_9110)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x18;
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0035_912c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimCombo_v52(AnimCombo *this)` (0x359140): forward to
/// `AnimObject_v52(this+0x34)` and, when the bones are ready, multiply every
/// bone matrix into the snapshot buffer.
pub fn anim_combo_v52(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
        ANIM_OBJECT_V52.call(c, 0x0035_916c)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x60);
            anim_combo_v52_epilogue(c, &frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x14)).lb(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x17)).lb(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x60);
                anim_combo_v52_epilogue(c, &frame);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x15)).lb(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x60);
                anim_combo_v52_epilogue(c, &frame);
                return Ok(());
            }
            c.cycles += 1;
        }
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x60)).lw(c);
        c.cycles += 6;
        c.g[S5][0] = 0;
        c.g[S2][0] = 0;
        c.g[S3][0] = Addr(c.g[V1][0] as u32).off(0x80).lw(c);
        c.g[V0][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = Addr(c.g[S3][0] as u32).off(8).lw(c);
        if !empty {
            c.cycles += 2;
            c.g[S4][0] = 0;
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(8).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
                let skip = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                if !skip {
                    c.cycles += 7;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x7c)).lw(c);
                    c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
                    INTERSECT_INFO_MUL_MATRIX.call(c, 0x0035_91ec)?;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0035_91c0);
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x60);
        anim_combo_v52_epilogue(c, &frame);
        Ok(())
    })
}


/// The shared epilogue of [`anim_combo_v52`].
pub(crate) fn anim_combo_v52_epilogue(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void AnimCombo_SnapshotMatrices(AnimCombo *this)` (0x359030): copy each
/// bone's four quads (stepping 0xd0) into the snapshot buffer; VU0 `lqc2`/
/// `sqc2` set VF1-VF4 (the last copied bone is left in them).
pub fn anim_combo_snapshot_matrices(c: &mut Cpu) {
    c.cycles += 4;
    let t0 = c.g[A0][0];
    c.g[T0][0] = t0;
    c.g[V0][0] = Addr(t0 as u32).off(0x74).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[T2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = Addr(t0 as u32).off(0x60).lw(c);
    c.g[A2][0] = 0;
    c.g[A3][0] = Addr(c.g[V1][0] as u32).off(0x80).lw(c);
    c.g[V0][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
    let empty2 = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(8).lw(c);
    if empty2 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[T3][0] = sx((t0 as u32).wrapping_add(0x34));
    c.g[T1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[T2][0] as u32) << 6);
        if !skip {
            c.cycles += 15;
            c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x48).lw(c);
            c.g[A0][0] = Addr(t0 as u32).off(0x84).lw(c);
            c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x90));
            let src = c.g[V0][0] as u32;
            let dst = c.g[V1][0] as u32;
            c.lqc2(1, src);
            c.lqc2(2, src.wrapping_add(0x10));
            c.lqc2(3, src.wrapping_add(0x20));
            c.lqc2(4, src.wrapping_add(0x30));
            c.sqc2(1, dst);
            c.sqc2(2, dst.wrapping_add(0x10));
            c.sqc2(3, dst.wrapping_add(0x20));
            c.sqc2(4, dst.wrapping_add(0x30));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd0));
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        }
        c.cycles += 5;
        c.g[V0][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0035_9060);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void MultiParticle_Update(MultiParticle* p)` (0x358120): update the
/// particle system at `p->+0x10`, stepping it by the frame time at
/// `gp - 0x2a80`.
pub fn multi_particle_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.lwc1(12, r32(c, GP).wrapping_sub(0x2a80));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0).wrapping_add(0x10));
        call_ptr(c, 0x0037_0788, 0x0035_8134)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AnimDelta_v36(AnimObject* self, int id, float v)` (0x3593D0): id 0x8C
/// advances the object's delta at +0x6C by v/30, any other id is the generic
/// anim-object update.
pub fn anim_delta_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x8c;
        let id = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        if id != c.g[V0][0] {
            c.cycles += 2;
            callee::ANIM_OBJECT_V36.call(c, 0x0035_9400)?;
        } else {
            c.cycles += 6;
            c.ld_f(0, gp(c, -0x2a68));
            c.ld_f(1, (this as u32).wrapping_add(0x6c));
            c.mul_s(0, 12, 0);
            c.add_s(1, 1, 0);
            c.st_f(1, (this as u32).wrapping_add(0x6c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `AnimDelta* AnimDelta_ctor(this, a, b, c)` (0x359228): construct the
/// anim-object base (`AnimObject_ctor(this, a, 2, b, c)`), then install the
/// delta vtable and clear the delta state.
pub fn anim_delta_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[T0][0] = c.g[A3][0];
        frame.save(c, S0, 0x10);
        c.g[A3][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x2;
        ANIM_OBJECT_CTOR.call(c, 0x0035_9248)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(0x0049_0000u32.wrapping_sub(0x1590)); // &D_0048EA70
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x6c).sw(c, 0);
        Addr(this).off(0x3c).sw(c, c.g[V1][0]);
        Addr(this).off(4).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void RailMan_Save(SyncProcess* self, Stream* stream)` (0x358c30): 0x200
/// bytes at `+0x18`.
pub fn rail_man_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::SYNC_PROCESS_SAVE, 0x18, 0x200, 0x0035_8c4c, 0x0035_8c68)
    })
}


/// `void AnimDelta_Save(AnimDelta* self, Stream* stream)` (0x359410): 4 bytes
/// at `+0x6c`.
pub fn anim_delta_save(c: &mut Cpu) {
    guest(c, |c| {
        save_field_block(c, callee::ANIM_OBJECT_SAVE, 0x6c, 4, 0x0035_942c, 0x0035_9448)
    })
}


// ---------------------------------------------------------------------------
// Animation objects.

/// `void AnimCombo_v36(AnimCombo* self, int msg)` (0x358fd0): on the "snapshot"
/// message copy the current pose into the blend source; otherwise pass the
/// message to the animation object it embeds.
pub fn anim_combo_v36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x82;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let msg_is_snapshot = c.g[A1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !msg_is_snapshot {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0], 0x14)).lhu(c);
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 8;
                c.lwc1(0, p(c.g[S0][0], 0x38));
                c.swc1(0, p(c.g[S0][0], 0x10));
                call_ptr(c, 0x0035_9030, 0x0035_9000)?;
                c.lwc1(0, p(c.g[S0][0], 8));
                c.g[V0][0] = 1;
                Addr(p(c.g[S0][0], 0x14)).sb(c, c.g[V0][0]);
                c.swc1(0, p(c.g[S0][0], 4));
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0034_1fe8, 0x0035_901c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `AnimDelta* AnimDelta_ctor_2(AnimDelta* self, int a1, Stream* stream)`
/// (0x359270): base animation object, its vtable and the stream registration.
pub fn anim_delta_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0034_1c80, 0x0035_928c)?;
        c.g[V1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1590));
        c.g[A2][0] = 4;
        Addr(p(c.g[S0][0], 0x3c)).sw(c, c.g[V1][0]);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V0][0], 0x10)).lh(c);
        c.g[V1][0] = lw(c, p(c.g[V0][0], 0x14));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0035_92b4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void MultiParticle_RemoveInstance(MultiParticle* p_2, int key)`
/// (0x3581f0): remove `key` from the instance array (swap with the last) or
/// mark the slot empty.
pub fn multi_particle_remove_instance(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = Addr(p_2(r32(c, A0), 4)).lw(c);
    let none = (c.g[T0][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if none {
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
        return;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[T1][0] = Addr(p_2(r32(c, A0), 0xc)).lw(c);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[A2][0] = abi::addu(c.g[V0][0], c.g[T1][0]);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[T0][0] = Addr(p_2(r32(c, A0), 4)).lw(c);
            c.cycles += 4;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[T1][0] = Addr(p_2(r32(c, A0), 0xc)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0035_8208);
                }
                continue;
            }
            break;
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let last = c.g[V0][0] == 0;
            Addr(p_2(r32(c, A0), 4)).sw(c, c.g[V1][0]);
            if last {
                c.cycles += 2;
                Addr(c.g[A2][0] as u32).sw(c, c.g[T2][0]);
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = abi::addu(c.g[V0][0], c.g[T1][0]);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
            }
            let ra = c.g[RA][0] as u32;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}


/// `lbu off(reg)`.
#[inline]
pub(crate) fn lbu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lbu(c)
}


/// `func_00358140` (0x358140): remove every instance of section `a1` from a
/// multi-particle object. Walks the instance array (`+0xc`, count at `+4`) and
/// for each match replaces the slot with the last instance (and shrinks the
/// count), then retries the same index. The loop back edge carries an event
/// check. Signature: `void remove(MultiParticle* this, u8 section)`.
pub fn multi_particle_remove_section_instances(c: &mut Cpu) {
    c.cycles += 3;
    c.g[T0][0] = lw_r(c, A0, 0x4);
    let empty = (c.g[T0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[T2][0] = 0xffff_ffff_ffff_0000;
    c.g[T2][0] = c.g[T2][0] | 0xffff;
    c.g[T1][0] = lw_r(c, A0, 0xc);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(low(c, A2) << 2);
        c.g[A3][0] = sx(low(c, V0).wrapping_add(low(c, T1)));
        c.g[V1][0] = lbu_r(c, A3, 0);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[T0][0] = lw_r(c, A0, 0x4);
        } else {
            c.cycles += 4;
            c.g[V1][0] = sx(low(c, T0).wrapping_sub(1));
            let less = (c.g[A2][0] as i64) < (c.g[V1][0] as i64);
            c.g[V0][0] = less as u64;
            let last = c.g[V0][0] == 0;
            sw_r(c, A0, 0x4, c.g[V1][0]);
            if last {
                c.cycles += 1;
                sw_r(c, A3, 0, c.g[T2][0]);
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx(low(c, V1) << 2);
                c.g[A2][0] = sx(low(c, A2).wrapping_sub(1));
                c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, T1)));
                c.g[V1][0] = Addr(low(c, V0)).lw(c);
                sw_r(c, A3, 0, c.g[V1][0]);
            }
            c.cycles += 1;
            c.g[T0][0] = lw_r(c, A0, 0x4);
        }
        c.cycles += 4;
        c.g[A2][0] = sx(low(c, A2).wrapping_add(1));
        let more = (c.g[A2][0] as i64) < (c.g[T0][0] as i64);
        c.g[V0][0] = more as u64;
        if !more {
            break;
        }
        c.g[T1][0] = lw_r(c, A0, 0xc);
        if event_due(c) {
            deliver_event_here(c, 0x0035_8158);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void RailMan_OnSectionUnload(kind)` (0x358700): for every rail slot
/// (`D_004A4040`, 128 entries) with a kind tag matching `kind`, re-enable the
/// rail.
pub fn railman_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = lw_at(c, gw(c, 0xf50));
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 4;
            c.g[S1][0] = 0xffff_ffff_ffff_0000;
            c.g[S0][0] = 0;
            c.g[S1][0] |= 0xffff;
            let mut m = lw_at(c, gw(c, 0xf50));
            loop {
                c.cycles += 5;
                c.g[A0][0] = m;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                let slot = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = slot;
                c.g[V1][0] = lw_at(c, p_3(slot as u32, 0x18));
                let dead = c.g[V1][0] == c.g[S1][0];
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                if !dead {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(p_3(c.g[V0][0] as u32, 0x18)).lbu(c);
                    if c.g[V1][0] != c.g[S2][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0035_8998, 0x0035_8758)?;
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    }
                } else {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 128) as u64;
                c.g[A0][0] = lw_at(c, gw(c, 0xf50));
                if c.g[V0][0] == 0 {
                    break;
                }
                m = c.g[A0][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0035_8730);
                }
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


/// `void RailMan_dtor(self, flags)` (0x358668): free every live rail object and
/// chain to the base destructor.
pub fn rail_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x490000;
        frame.save(c, S2, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1220));
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        Addr(c.g[S2][0] as u32).off(0xc).sw(c, c.g[V0][0]);
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let unused = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            if !unused {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                callee::RAIL_MAN_ENABLE_RAIL.call(c, 0x0035_86bc)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 128) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_86a8);
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        Addr(gp_2(c, 0xf50)).sw(c, 0);
        callee::SYNC_PROCESS_DTOR.call(c, 0x0035_86dc)?;
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


/// `RailMan_ctor` (0x3584f0): construct the sync process and clear the rail
/// manager's two 128-entry tables.
pub fn rail_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x12;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_8508);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        callee::SYNC_PROCESS_CTOR.call(c, 0x0035_8508)?;
        c.g[V0][0] = 0x0049_0000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x1220));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p_4(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(this.wrapping_add(0x18));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8528);
            }
        }
        c.cycles += 5;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        Addr(gp_3(c, 0xf50)).sw(c, c.g[S0][0]);
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(this.wrapping_add(0x214));
        loop {
            c.cycles += 7;
            Addr(r(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8558);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        let ra = r(c, RA);
        c.g[SP][0] = sx(r(c, SP).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Rail manager.

/// `RailMan* RailMan_ctor_2(RailMan* self, Stream* stream, void* arg)`
/// (0x358588): init the sync process, clear the 128 rail handles, read the
/// handle table and disable every used rail.
pub fn railman_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0035_48c0, 0x0035_85ac)?;
        c.g[V0][0] = 0x490000;
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1220));
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p_5(r32(c, S2), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0x7f;
        c.g[V0][0] = sx(p_5(r32(c, S2), 0x18));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        loop {
            c.cycles += 7;
            Addr(r32(c, V0)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = c.g[V1][0] != c.g[A1][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_85d0);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A3][0] = sx(p_5(r32(c, S2), 0x18));
        Addr(gp_4(c, 0xf50)).sw(c, c.g[S2][0]);
        c.g[A1][0] = c.g[A3][0];
        c.g[A0][0] = Addr(p_5(r32(c, V0), 0x10)).lh(c);
        c.g[A2][0] = 0x200;
        c.g[V1][0] = Addr(p_5(r32(c, V0), 0x14)).lw(c);
        c.g[S3][0] = 0xffff_ffff_ffff_0000;
        c.g[A0][0] = sx(p_5(r32(c, S0), r32(c, A0)));
        let read = r32(c, V1);
        c.g[S1][0] = c.g[A3][0];
        call_ptr(c, read, 0x0035_8618)?;
        c.g[S0][0] = 0;
        c.g[S3][0] = c.g[S3][0] | 0xffff;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            let empty = c.g[V0][0] == c.g[S3][0];
            c.g[A0][0] = c.g[S2][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0035_8780, 0x0035_8634)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 128i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(p_5(r32(c, S1), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0035_8620);
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


/// `void AnimCombo_v52(cAnimCombo* self)`: update the underlying animation
/// object and, when the combo is playing, multiply every part's matrix with
/// the animation's transform.
pub fn anim_combo_v52_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_359204: {
            c.cycles += 14;
            c.g[SP][0] = sub(c.g[SP][0], 0x70);
            c.sq(16, p_6(c.g[SP][0] as u32, 0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, p_6(c.g[SP][0] as u32, 0x50));
            c.sq(18, p_6(c.g[SP][0] as u32, 0x40));
            c.sq(19, p_6(c.g[SP][0] as u32, 0x30));
            c.sq(20, p_6(c.g[SP][0] as u32, 0x20));
            c.sq(21, p_6(c.g[SP][0] as u32, 0x10));
            c.sd(p_6(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
            call_ptr(c, 0x0034_e348, 0x0035_916c)?;
            c.g[V0][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::ANIM));
            if c.g[V0][0] == 0 {
                c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
                break 'b_359204;
            }
            'b_3591a0: {
                c.cycles += 3;
                c.g[V0][0] = c.lb(p_6(c.g[S0][0] as u32, anim_combo::PART_A));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::MODEL));
                    break 'b_3591a0;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(p_6(c.g[S0][0] as u32, anim_combo::PART_C));
                if c.g[V0][0] == 0 {
                    c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
                    break 'b_359204;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(p_6(c.g[S0][0] as u32, anim_combo::PART_B));
                if c.g[V0][0] == 0 {
                    c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
                    break 'b_359204;
                }
                c.cycles += 1;
                c.g[V1][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::MODEL));
                break 'b_3591a0;
            }
            'b_359200: {
                c.cycles += 6;
                c.g[S5][0] = 0;
                c.g[S2][0] = 0;
                c.g[S3][0] = lw(c, p_6(c.g[V1][0] as u32, 0x80));
                c.g[V0][0] = lw(c, p_6(c.g[S3][0] as u32, 4));
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = lw(c, p_6(c.g[S3][0] as u32, 8));
                if none {
                    break 'b_359200;
                }
                c.cycles += 2;
                c.g[S4][0] = 0;
                'l_3591c0: loop {
                    'b_3591ec: {
                        c.cycles += 4;
                        c.g[V0][0] = lw(c, p_6(c.g[S1][0] as u32, 8));
                        c.g[V1][0] = lw(c, p_6(c.g[V0][0] as u32, 0x1c));
                        let empty = c.g[V1][0] == 0;
                        c.g[V0][0] = sx((c.g[S5][0] as u32) << 6);
                        if empty {
                            break 'b_3591ec;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::ANIM));
                        c.g[A1][0] = lw(c, p_6(c.g[S0][0] as u32, anim_combo::MATRICES));
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(1));
                        c.g[A0][0] = addu(c.g[A0][0], c.g[S4][0]);
                        c.g[A1][0] = addu(c.g[A1][0], c.g[V0][0]);
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
                        call_ptr(c, 0x0035_1948, 0x0035_91ec)?;
                        break 'b_3591ec;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, p_6(c.g[S3][0] as u32, 4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                    if !more {
                        break 'b_359200;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_91c0);
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, p_6(c.g[SP][0] as u32, 0x60));
            break 'b_359204;
        }
        c.cycles += 8;
        c.lq(17, p_6(c.g[SP][0] as u32, 0x50));
        c.lq(18, p_6(c.g[SP][0] as u32, 0x40));
        c.lq(19, p_6(c.g[SP][0] as u32, 0x30));
        c.lq(20, p_6(c.g[SP][0] as u32, 0x20));
        c.lq(21, p_6(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(p_6(c.g[SP][0] as u32, 0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}


/// `int func_003592D0(AnimDelta* s)` (0x3592d0): tick a particle/animation
/// delta: reset the two 1e10 guards, and when the object is not attached to a
/// live animation, count the delta down (clamped at 0), run the animation
/// object step once it reaches 0, and flag the parent; an attached object gets
/// its callback with the "0x30" payload. Returns 1 except when the callback
/// path ran.
pub fn anim_delta_v15(c: &mut Cpu) {
    /// What the tail of the function does.
    enum Tail {
        /// "+1; v0 = 1": report the object as alive.
        Alive,
        /// "+2; *(u16*)(s+0x42) |= 1; +1; v0 = 1".
        Flag,
        /// Return straight away (the callback path already set `v0`).
        Return,
    }
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.lwc1(0, gp_5(c, -0x2a6c)); // 1e10
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = c.g[S0][0] as u32;
        c.swc1(0, p_7(s, 0x28));
        c.g[V0][0] = c.lw(p_7(s, 8));
        let attached = c.g[V0][0] != 0;
        c.swc1(0, p_7(s, 0x24));
        let tail;
        if attached {
            tail = Tail::Alive;
        } else {
            c.cycles += 6;
            c.lwc1(1, p_7(s, 0x6c)); // v
            c.mtc1(0, 2); // f2 = 0.0
            c.c_lt_s(2, 1); // 0 < v
            let v_le_zero = !c.fcc();
            c.g[V0][0] = c.lw(p_7(s, 0xc)); // t
            if v_le_zero {
                // t > 0: count it down
                c.cycles += 2;
                let t_le_zero = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if !t_le_zero {
                    c.cycles += 1;
                    c.sw(p_7(s, 0xc), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p_7(s, 8));
                let no_parent = c.g[V0][0] == 0;
                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                if no_parent {
                    c.cycles += 1;
                    c.g[V0][0] = c.lhu(p_7(c.g[S0][0] as u32, 0x42));
                    tail = Tail::Flag;
                } else {
                    // the slot-0x110 callback: the object is attached elsewhere
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(p_7(c.g[S0][0] as u32, 0x3c));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh(p_7(c.g[V1][0] as u32, 0x110));
                    c.g[V0][0] = c.lw(p_7(c.g[V1][0] as u32, 0x114));
                    let func = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0035_93a4);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, func, 0x0035_93a4)?;
                    c.g[V0][0] = 0;
                    tail = Tail::Return;
                }
            } else if (c.g[V0][0] as i64) < 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lhu(p_7(c.g[S0][0] as u32, 0x42));
                tail = Tail::Flag;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    if !c.fcc() {
                        // t == 0 and v > 0: bleed the delta by |s+0x10|
                        // No 60/rate here: +0x10 is already a per-tick step
                        // (AnimObject msg 0x66 and the ctor divide the 30 fps
                        // value by the rate), so the budget drains in the same
                        // wall-clock time at every rate.
                        c.cycles += 7;
                        c.lwc1(0, p_7(c.g[S0][0] as u32, 0x10));
                        c.abs_s(0, 0);
                        c.sub_s(0, 1, 0);
                        c.c_lt_s(0, 2);
                        let negative = !c.fcc();
                        c.swc1(0, p_7(c.g[S0][0] as u32, 0x6c));
                        if !negative {
                            c.cycles += 1;
                            c.swc1(2, p_7(c.g[S0][0] as u32, 0x6c));
                        }
                    }
                }
                c.cycles += 8;
                c.g[RA][0] = sx(0x0035_934c);
                c.g[A0][0] = c.g[S0][0];
                c.pc = 0x0034_1d48;
                ee::s11::AnimObject_v15(c);
                if c.pc != 0x0035_934c {
                    return Err(Unwound);
                }
                c.lwc1(1, p_7(c.g[S0][0] as u32, 0x6c));
                c.mtc1(0, 0); // f0 = 0.0
                c.c_lt_s(0, 1); // 0 < v
                let stopped = !c.fcc(); // v <= 0 after the step
                c.g[V0][0] = 0x1;
                if stopped {
                    c.cycles += 2;
                    c.sw(p_7(c.g[S0][0] as u32, 4), 0);
                    tail = Tail::Alive;
                } else {
                    c.cycles += 2;
                    c.sw(p_7(c.g[S0][0] as u32, 4), c.g[V0][0]);
                    tail = Tail::Return;
                }
            }
        }
        match tail {
            Tail::Alive => {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            Tail::Flag => {
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.sh(p_7(c.g[S0][0] as u32, 0x42), c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 0x1;
            }
            Tail::Return => {}
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void func_00358998(void* railman, int idx)` (0x358998): enable rail
/// `idx`: fetch its part from the world, disable each collider node's box
/// (inserted into the spatial hash), then mark the reference unused
/// (`-1`).
pub fn rail_man_enable_rail(c: &mut Cpu) {
    let frame = Frame::open(c, 0xc0);
    'b_358a24: {
        'b_358a00: {
            c.cycles += 24;
            frame.save(c, S1, 0x90);
            frame.save(c, S6, 0x40);
            c.g[S1][0] = c.g[A1][0];
            c.g[S6][0] = c.g[A0][0];
            frame.save(c, S2, 0x80);
            frame.save(c, S3, 0x70);
            c.g[V0][0] = sx(r32(c, S1) << 2);
            frame.save(c, S4, 0x60);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S6)));
            frame.save(c, S5, 0x50);
            frame.save_ra(c, 0x30);
            c.swc1(20, at_2(c, SP, 0xb0));
            frame.save(c, S0, 0xa0);
            c.g[RA][0] = sx(0x003589d8);
            c.g[S0][0] = lw_3(c, at_2(c, V0, 0x18));
            if call_ptr(c, 0x002d1bd8, 0x003589d8).is_err() {
                return;
            }
            c.g[A0][0] = lw_3(c, r32(c, V0));
            c.g[V0][0] = c.g[S0][0] & 0xff;
            c.g[V1][0] = lw_3(c, at_2(c, A0, 0x8));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[V1][0] = lw_3(c, r32(c, V0));
            if c.g[V1][0] != 0u64 {
                c.g[A0][0] = lw_3(c, at_2(c, V1, 0x44));
                break 'b_358a00;
            } else {
                c.cycles += 2;
                let t = true;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_358a24;
                } else {
                    break 'b_358a00;
                }
            }
        }
        'b_358a20: {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, S0) >> 8);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = lw_3(c, r32(c, V0));
            c.g[V0][0] = sx(r32(c, V1) >> 8);
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            if t {
                break 'b_358a20;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_358a20;
            }
        }
        c.cycles += 1;
        c.g[S4][0] = c.g[V0][0];
        break 'b_358a24;
    }
    'b_358ae8: {
        c.cycles += 5;
        c.g[S5][0] = sx(r32(c, S1) << 2);
        c.g[V0][0] = lw_3(c, at_2(c, S4, 0x20));
        c.g[S1][0] = lw_3(c, at_2(c, S4, 0x24));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S2][0] = 0u64;
        if t {
            break 'b_358ae8;
        } else {
            c.cycles += 4;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.g[S3][0] = 0xffffffffffffffff;
            'l_358a48: loop {
                c.cycles += 40;
                c.lwc1(1, at_2(c, S1, 0x6c));
                c.g[S2][0] = sx(at_2(c, S2, 0x1));
                c.swc1(1, r32(c, SP));
                c.lwc1(0, at_2(c, S1, 0x70));
                c.swc1(0, at_2(c, SP, 0x4));
                c.lwc1(1, at_2(c, S1, 0x74));
                c.swc1(20, at_2(c, SP, 0xc));
                c.swc1(1, at_2(c, SP, 0x8));
                c.lwc1(0, at_2(c, S1, 0x78));
                c.swc1(0, at_2(c, SP, 0x10));
                c.lwc1(1, at_2(c, S1, 0x7c));
                c.swc1(1, at_2(c, SP, 0x14));
                c.lwc1(0, at_2(c, S1, 0x80));
                c.swc1(20, at_2(c, SP, 0x1c));
                c.g[RA][0] = sx(0x00358a88);
                c.swc1(0, at_2(c, SP, 0x18));
                if call_ptr(c, 0x002d1be0, 0x00358a88).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx(at_2(c, SP, 0x20));
                c.g[RA][0] = sx(0x00358a98);
                c.g[A1][0] = c.g[SP][0];
                if call_ptr(c, 0x00328f28, 0x00358a98).is_err() {
                    return;
                }
                c.g[V0][0] = lw_3(c, at_2(c, SP, 0x24));
                c.g[V1][0] = sx(at_2(c, S0, 0x50));
                c.g[A1][0] = lw_3(c, at_2(c, SP, 0x28));
                c.g[A2][0] = c.g[S1][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[T0][0] = lw_3(c, at_2(c, SP, 0x2c));
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[A1][0] = ((c.g[S3][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.g[A0][0] = sx(at_2(c, V1, 0x28));
                c.g[T0][0] = ((c.g[S3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                if c.g[A1][0] != 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[A3][0] = sx(at_2(c, SP, 0x20));
                c.g[V0][0] = sx(at_2(c, A0, 0x14));
                c.g[A1][0] = 0x2;
                c.g[RA][0] = sx(0x00358ad8);
                if c.g[T0][0] == 0 {
                    c.g[A0][0] = c.g[V0][0];
                }
                if call_ptr(c, 0x00328c20, 0x00358ad8).is_err() {
                    return;
                }
                c.g[V0][0] = lw_3(c, at_2(c, S4, 0x20));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S1][0] = lw_3(c, at_2(c, S1, 0x64));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00358a48);
                    }
                    continue 'l_358a48;
                } else {
                    break 'b_358ae8;
                }
            }
        }
    }
    c.cycles += 15;
    c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, S6)));
    c.g[V0][0] = 0xffffffffffff0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    c.g[RA][0] = ld(c, at_2(c, SP, 0x30));
    c.lwc1(20, at_2(c, SP, 0xb0));
    sw(c, at_2(c, V1, 0x18), c.g[V0][0]);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0xc0));
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
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::FAKE;
    use ssx3_ee as ee;
    use crate::difftest::fakes::*;
    use crate::difftest::SP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::compare;
    use crate::difftest::fill;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::main::gameload::tests::fb;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_4 as pool_allocator;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::intersect::worldsphtree::tests::fake_stream;
    pub(crate) use crate::game::intersect::worldsphtree::tests::save_test;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_7;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_2;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_6;


    #[test]
    fn multi_particle_add_instance_matches() {
        check(ssx3_ee::s11::f_003581b8, multi_particle_add_instance, 200, &|c, seed| {
            let mut r = Rng(0x1026 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((DATA + 0x100) as u64) | ((r.next_u32() as u64) << 32);
            let cap = 4 + (seed % 4);
            let count = if seed % 3 == 0 { cap } else { seed % 4 };
            c.bus.write32(DATA, cap);
            c.bus.write32(DATA + 4, count);
            c.bus.write32(DATA + 0xc, DATA + 0x200);
        });
    }


    #[test]
    fn anim_combo_ctor_2_matches() {
        check(ee::s11::f_00358db0, anim_combo_ctor_2, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8db0, 0);
            alloc_setup(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x200) as u64;
            let vt = vtable(c, &mut r, &[0x10]);
            c.bus.write32(DATA + 0x200, vt);
            c.bus.write32(this + 0x74, if seed % 3 == 0 { 0 } else { seed % 3 });
            crate::difftest::fill(c, &mut r, this + 0x18, 0x80);
            crate::difftest::fill(c, &mut r, DATA + 0x2000, 0x400 / 4);
        });
    }


    #[test]
    fn anim_combo_kill_matches() {
        check(ee::s11::f_003590c8, anim_combo_kill, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x90c8, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(DATA + 0x15, if seed % 2 == 0 { 0 } else { 1 });
            crate::difftest::fill(c, &mut r, DATA + 0x48, 0x40);
        });
    }


    #[test]
    fn anim_combo_save_matches() {
        check(ee::s11::f_003590f0, anim_combo_save, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x90f0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(DATA + 0x200, vt);
            crate::difftest::fill(c, &mut r, DATA + 0x18, 0x40);
        });
    }


    #[test]
    fn anim_combo_dtor_matches() {
        check(ee::s11::f_00358e50, anim_combo_dtor, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8e50, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.bus.write32(DATA + 0x84, if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        });
    }


    #[test]
    fn anim_combo_v42_matches() {
        check(ee::s11::f_00358f90, anim_combo_v42, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x8f90, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            crate::difftest::fill(c, &mut r, DATA + 0x48, 0x40);
            crate::difftest::fill(c, &mut r, 0x4ff120, 8);
        });
    }


    #[test]
    fn anim_combo_snapshot_matrices_matches() {
        check(ee::s11::f_00359030, anim_combo_snapshot_matrices, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x9030, 0);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + 0x74, if seed % 5 == 0 { 0 } else { 1 + (seed % 3) });
            c.bus.write32(this + 0x60, DATA + 0x100);
            c.bus.write32(DATA + 0x100 + 0x80, DATA + 0x300);
            let n = seed % 4;
            c.bus.write32(DATA + 0x300 + 4, n);
            c.bus.write32(DATA + 0x300 + 8, DATA + 0x400);
            for i in 0..n {
                c.bus.write32(DATA + 0x400 + i * 0x10 + 8, if (seed >> i) & 1 != 0 { 1 } else { 0 });
                c.bus.write32(DATA + 0x400 + i * 0x10 + 0xc, r.next_u32());
            }
            c.bus.write32(this + 0x7c, DATA + 0x1000);
            c.bus.write32(this + 0x84, DATA + 0x2000);
            crate::difftest::fill(c, &mut r, DATA + 0x1000, 0x400 / 4);
            crate::difftest::fill(c, &mut r, DATA + 0x2000, 0x400 / 4);
        });
    }


    #[test]
    fn multi_particle_update_matches() {
        crate::difftest::check(ee::s11::f_00358120__generated, multi_particle_update, 50, &|c, seed| {
            let mut r = machine(c, seed, 0x3581_20, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            // `p + 0x10` is the particle system; a null (or zeroed) system
            // makes `Emitter_Update` take its "descriptor time 0" exit.
            c.bus.write32(DATA + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(DATA + 0x1000, 0);
            c.bus.write32(GAME_GP.wrapping_sub(0x2a80), r.f32(1.0));
        });
    }
 // gp - 0x4618: the 0.01 gain step
    pub(crate) const GP_1_30: u32 = 0x4a06_88;


    #[test]
    fn anim_delta_v36_matches() {
        check(ssx3_ee::s11::f_003593d0__generated, anim_delta_v36, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x3593_d0, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0x8c } else { 0x10 };
            c.f[12] = r.edge_f32();
            w(c, DATA + 0x6c, r.edge_f32());
            w(c, GP_1_30, fb(1.0 / 30.0));
        });
    }


    /// A nonzero BELibrary singleton, so the BE init path in `0x14dc80` does
    /// not try to create the library (and allocate) from disc.
    pub(crate) fn be_library_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }


    // -----------------------------------------------------------------------
    // 0x359228 AnimDelta_ctor.

    #[test]
    fn anim_delta_ctor_matches() {
        check(ee::s11::f_00359228__generated, anim_delta_ctor, 8, &|c, seed| {
            let _r = machine(c, seed, 0x3592_28, 0);
            pool_allocator(c);
            be_library_2(c);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x800);
            c.g[A3][0] = sx(DATA + 0xc00);
        });
    }


    #[test]
    fn rail_man_save_matches() {
        save_test(ee::s11::f_00358c30__generated, rail_man_save, 0x358c_30, 0x18);
    }


    #[test]
    fn anim_delta_save_matches() {
        save_test(ee::s11::f_00359410__generated, anim_delta_save, 0x3594_10, 0x6c);
    }


    #[test]
    fn anim_combo_v36_matches() {
        check(ee::s11::f_00358fd0__generated, anim_combo_v36, SEEDS_4, &|c, seed| {
            let mut r = base(c, seed, 0x358f_d0);
            let s = DATA + 0x100;
            clear(c, s, 0x100);
            w_2(c, s + 0x14, if seed % 2 == 0 { 0 } else { 1 });
            w_2(c, s + 0x38, r.next_u32());
            w_2(c, s + 8, r.next_u32());
            w_2(c, s + 0x48, 0);
            w_2(c, s + 4, r.next_u32());
            c.f[12] = r.next_u32();
            c.g[A0][0] = sx(s);
            let msg = match seed % 6 {
                0 => 0x82,
                1 => 0x64,
                2 => 0x65,
                3 => 0x67,
                4 => 0x68,
                _ => 0xff,
            };
            c.g[A1][0] = msg as u64;
        });
    }


    #[test]
    fn multi_particle_remove_instance_matches() {
        check(ssx3_ee::s11::f_003581f0__generated, multi_particle_remove_instance, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x3581f0, 0);
            let p_2 = DATA + 0x1000;
            let entries = DATA + 0x2000;
            let count = (seed % 5) as u32;
            c.g[A0][0] = p_2 as u64;
            w32(c, p_2 + 4, count);
            w32(c, p_2 + 0xc, entries);
            for i in 0..5u32 {
                w32(c, entries + 4 * i, if i < count { r.next_u32() } else { 0 });
            }
            // Half of the seeds look for a present key.
            c.g[A1][0] = if seed % 2 == 0 && count > 0 {
                c.bus.read32(entries + 4 * ((seed % count) as u32)) as u64
            } else {
                r.next_u32() as u64
            };
        });
    }


    #[test]
    fn multi_particle_remove_section_instances_matches() {
        check(
            ee::s11::f_00358140__generated,
            multi_particle_remove_section_instances,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x358140, 2);
                be_library_3(c);
                world(c);
                let this = DATA;
                let arr = DATA + 0x400;
                c.g[A0][0] = this as u64;
                let target = (seed % 3) as u64;
                c.g[A1][0] = target;
                let n = seed % 6;
                w(c, this + 4, n);
                w(c, this + 0xc, arr);
                let mut r = Rng(0x3581_0000 ^ seed as u64);
                for i in 0..n {
                    b(c, arr + i, (r.next_u32() % 3) as u8);
                }
            },
        );
    }


    #[test]
    fn railman_on_section_unload_matches() {
        check(
            ee::s11::f_00358700__generated,
            railman_on_section_unload,
            SEEDS_7,
            &|c, seed| {
                base_2(c, seed, 0x8700, 0);
                let m = DATA + 0x800;
                c.g[A0][0] = (seed % 3) as u64;
                c.bus
                    .write32(GAME_GP + 0xf50, if seed % 7 == 0 { 0 } else { m });
                for i in 0..0x80u32 {
                    let w = match (i + seed) % 8 {
                        0 => 0xffff_ffff,
                        1 => seed % 3,
                        _ => 0x100 + i,
                    };
                    c.bus.write32(m + 0x18 + 4 * i, w);
                }
            },
        );
    }


        #[test]
    fn rail_man_dtor_matches() {
        check(ee::s11::f_00358668__generated, rail_man_dtor, 16, &|c, seed| {
            let mut r = base_3(c, seed, 0x358668, 3);
            let obj = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 2) as u64;
            // One live rail plus unused slots, so the free loop stays short.
            for i in 0..0x80u32 {
                w(c, obj + 0x18 + 4 * i, if i == 0 { 0 } else { 0xffff_ffff });
            }
            let _ = r.next_u32();
        });
    }


    #[test]
    fn rail_man_ctor_matches() {
        check(
            ee::s11::f_003584f0__generated,
            rail_man_ctor,
            SEEDS_8,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let _ = seed;
                c.g[A0][0] = this as u64;
            }),
        );
    }


    #[test]
    fn railman_ctor_2_matches() {
        check(ee::s11::f_00358588__generated, railman_ctor_2, 8, &|c, seed| {
            let mut r = base_5(c, seed, 0x358588, 2);
            let this = DATA;
            clear_2(c, this, 0x1000);
            let stream = DATA + 0x800;
            clear_2(c, stream, 0x100);
            let vt = vtable(c, &mut r, &[0x10]);
            w(c, stream, vt);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            c.g[A2][0] = (seed % 2) as u64;
            for i in 0..128u32 {
                w(c, this + 0x18 + 4 * i, 0xffff_ffff);
            }
        });
    }


    #[test]
    fn anim_combo_v52_matches() {
        diff(ee::s11::f_00359140__generated, anim_combo_v52_2, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x35_9140, 0);
            globals_3(c);
            let anim = DATA + 0x400;
            c.g[A0][0] = sx(DATA);
            // The animation object: no flags (early exit of AnimObject_v52).
            w(c, anim + 0x48, 0);
            w(c, DATA + 0x34, anim);
            w(c, DATA + anim_combo::ANIM, if seed % 5 == 0 { 0 } else { DATA + 0x4000 });
            c.bus.write8(DATA + anim_combo::PART_A, (seed % 2) as u8);
            c.bus.write8(DATA + anim_combo::PART_B, ((seed >> 1) % 2) as u8);
            c.bus.write8(DATA + anim_combo::PART_C, ((seed >> 2) % 2) as u8);
            w(c, DATA + anim_combo::MODEL, DATA + 0x6000);
            w(c, DATA + anim_combo::MATRICES, DATA + 0x5000);
            // The animation controller's part list.
            let ctrl = DATA + 0x6000;
            let count = (seed % 4) as u32;
            w(c, ctrl + 0x80, DATA + 0x7000);
            w(c, DATA + 0x7004, count);
            w(c, DATA + 0x7008, DATA + 0x7100);
            for i in 0..count {
                let part = DATA + 0x7100 + 0x10 * i;
                let obj = DATA + 0x7200 + 0x40 * i;
                w(c, part + 8, obj);
                w(c, obj + 0x1c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            }
            for i in 0..8 {
                w(c, DATA + 0x4000 + 4 * i, r.next_u32());
                w(c, DATA + 0x5000 + 4 * i, r.next_u32());
            }
        });
    }


        pub(crate) const RAIL_MAN: u32 = DATA + 0x1_0000;

    pub(crate) const RAIL_REF: u32 = DATA + 0x1_1000;

    pub(crate) const RAIL_PART: u32 = DATA + 0x1_2000;

    pub(crate) const RAIL_NODE: u32 = DATA + 0x1_3000;


    #[test]
    fn rail_man_enable_rail_matches() {
        check_all(ee::s11::f_00358998__generated, rail_man_enable_rail, 24, &|c, seed| {
            let mut r = base_6(c, seed, 0x3589_98, 8);
            c.g[A0][0] = sx(RAIL_MAN);
            c.g[A1][0] = sx(seed % 3);
            // `railman+0x18` is an array of 8-byte references whose object
            // (a multi-part) yields the collider list.
            c.bus.write32(RAIL_MAN + 0x18 + (seed % 3) * 8, RAIL_REF);
            c.bus.write32(RAIL_MAN + 0x1c + (seed % 3) * 8, 0);
            c.bus.write32(RAIL_PART, RAIL_NODE);
            c.bus
                .write32(RAIL_PART + 0x4, if seed % 2 == 0 { 0 } else { 2 });
            c.bus.write32(
                RAIL_NODE + 0x64,
                if seed % 2 == 0 { 0 } else { RAIL_NODE + 0x100 },
            );
            for i in 0..6 {
                c.bus.write32(RAIL_NODE + 0x6c + 4 * i, r.f32(10.0));
            }
        });
    }
}
