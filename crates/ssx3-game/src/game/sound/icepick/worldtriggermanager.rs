//! `src/sound/icepick/worldtriggermanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::addu;
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
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::events;
use crate::call;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::intersect::worldsphtree::lw;
pub(crate) use crate::game::intersect::worldsphtree::at;
pub(crate) use crate::game::intersect::aifwddiff::gp;
pub(crate) use crate::game::intersect::worldsphtree::fpu;
pub(crate) use crate::game::intersect::worldsphtree::sw_reg;
pub(crate) use crate::game::intersect::worldsphtree::p;
pub(crate) use crate::game::replay::replay2::destroy_if_flag;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::debugfe::at as at_2;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::be::beintplayer::mul32;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::object::flexrailnode::at_3;
pub(crate) use crate::game::object::flexrailnode::r32v;
pub(crate) use crate::game::object::flexrailnode::spa;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_4b70, world_trigger_instance_ctor, "WorldTriggerInstance_ctor"),
    (0x002b_4be0, world_trigger_instance_dtor, "WorldTriggerInstance_dtor"),
    (0x002b_4c08, world_trigger_instance_matches, "WorldTriggerInstance_Matches"),
    (0x002b_5818, world_trigger_get_info, "WorldTrigger_GetInfo"),
    (0x002b_5838, world_trigger_is_type5, "WorldTrigger_IsType5"),
    (0x002b_5878, world_trigger_get_name, "WorldTrigger_GetName"),
    (0x002b_5898, world_trigger_instance_reset, "WorldTriggerInstance_Reset"),
    (0x002b_58d0, world_trigger_manager_ctor, "WorldTriggerManager_ctor"),
    (0x002b_5b90, world_trigger_manager_find_free_instance, "WorldTriggerManager_FindFreeInstance"),
    (0x002b_5bc0, world_trigger_manager_find_instance, "WorldTriggerManager_FindInstance"),
    (0x002b_5c68, world_trigger_manager_add_trigger, "WorldTriggerManager_AddTrigger"),
    (0x002b_5e68, world_trigger_manager_stop_all, "WorldTriggerManager_StopAll"),
    (0x002b_63d0, world_trigger_manager_get_trigger_info, "WorldTriggerManager_GetTriggerInfo"),
    (0x002b_6550, world_trigger_manager_get_special_trigger_info, "WorldTriggerManager_GetSpecialTriggerInfo"),
    (0x002b_6630, world_trigger_manager_count_active_others, "WorldTriggerManager_CountActiveOthers"),
    (0x002b_66e8, world_trigger_manager_is_duplicate_active, "WorldTriggerManager_IsDuplicateActive"),
    (0x002b_6740, world_trigger_manager_any_trigger_flagged, "WorldTriggerManager_AnyTriggerFlagged"),
    (0x002b_67d8, world_trigger_manager_is_trigger_flag4, "WorldTriggerManager_IsTriggerFlag4"),
    (0x002b_6808, world_trigger_manager_was_played, "WorldTriggerManager_WasPlayed"),
    (0x002b_6868, world_trigger_manager_mark_played, "WorldTriggerManager_MarkPlayed"),
    (0x002b_6900, world_trigger_manager_reset_played_list, "WorldTriggerManager_ResetPlayedList"),
    (0x002b_6988, world_trigger_manager_is_trigger_flag8, "WorldTriggerManager_IsTriggerFlag8"),
    (0x002b_69b8, world_trigger_manager_check_crossing, "WorldTriggerManager_CheckCrossing"),
    (0x002b_6a68, world_trigger_manager_flag_all_active, "WorldTriggerManager_FlagAllActive"),
    (0x002b_6aa0, world_sound_ctor, "cWorldSound_ctor"),
    (0x002b_6ae0, c_world_sound_dtor, "cWorldSound_dtor"),
    (0x002b_6b08, c_world_sound_on_section_loaded, "cWorldSound_OnSectionLoaded"),
    (0x002b_6b30, c_world_sound_on_section_unloaded, "cWorldSound_OnSectionUnloaded"),
    (0x002b_6c90, c_world_sound_insert_emitters, "cWorldSound_InsertEmitters"),
    (0x002b_77c0, cworldsound_remove_emitter, "cWorldSound_RemoveEmitter"),
    (0x002b_83f8, world_sound_query_tree_node, "cWorldSound_QueryTreeNode"),
];

/// Direct-call targets (dossier callees, found by address in `generated/ee`).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::intersect::worldsphtree::callee::COLLIDE_VOLUME_OBJECT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_PAINTER_LOOKUP_CELL;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_SOUND_QUERY_TREE_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::AI_FWD_DIFF_CACHE_GET;
    pub(crate) use crate::game::intersect::worldsphtree::callee::AI_FWD_DIFF_CACHE_COLLIDE_ENTRY;
    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;
    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;
    pub(crate) use crate::game::be::beintoption::callee::C_WORLD_SOUND_RELOCATE_EMITTERS;
    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;
    pub(crate) use crate::game::be::beintoption::callee::IRRADIANCE_REF_GET_ENTRY as IRRADIANCE_REF_GET_ENTRY_2;
    pub(crate) use crate::game::be::beintoption::callee::BE_OPTION_INT_GET_MAX_HANDICAP;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::be::bedbprofile::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::be::bedbprofile::callee::WORLD_TRIGGER_FIND_INSTANCE;
    pub(crate) use crate::game::be::bedbprofile::callee::WORLD_TRIGGER_FIND_FREE;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_NORMALIZE_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_CALC_IMAGE_SIZE;
    pub(crate) use crate::game::be::bedbprofile::callee::SHP_PALETTE_ENTRIES_FOR_BPP;
    pub(crate) use crate::game::be::bedbprofile::callee::CLOSEST_POINT_ON_SEGMENT;

    /// `EventSelect_HandleInput` — stub, returns 0.
    pub const EVENT_SELECT_HANDLE_INPUT: u32 = 0x0020_2768;

    /// `MCManager_GetInstance`.
    pub const MC_MANAGER_GET_INSTANCE: u32 = 0x0023_c808;

    /// `StatsProcess_UpdateStub`.
    pub const STATS_PROCESS_UPDATE_STUB: u32 = 0x002d_21b0;

    /// `StatsProcess_RenderStub`.
    pub const STATS_PROCESS_RENDER_STUB: u32 = 0x002d_21b8;

    /// `ScriptEngine_PushFreeNode`.
    pub const SCRIPT_ENGINE_PUSH_FREE_NODE: u32 = 0x0027_bd90;

    /// `ScriptObj_Deactivate`.
    pub const SCRIPT_OBJ_DEACTIVATE: u32 = 0x0028_3440;

    /// `ScriptDummyObj_ReadFields`.
    pub const SCRIPT_DUMMY_OBJ_READ_FIELDS: u32 = 0x0027_cbb0;

    /// `PathData_ReadFloat`.
    pub const PATH_DATA_READ_FLOAT: u32 = 0x0026_ba88;

    /// `EA_MemCopy`.
    pub const EA_MEM_COPY: u32 = 0x003e_6574;

    /// `Rand_NextMain`.
    pub const RAND_NEXT_MAIN: u32 = 0x0031_77f0;

    /// `Rand_Next`.
    pub const RAND_NEXT: u32 = 0x0031_7a08;

    /// `TexMan_SetAllocFlags`.
    pub const TEX_MAN_SET_ALLOC_FLAGS: u32 = 0x0036_6fe0;

    /// `GraphicsMan_EmptyHook`.
    pub const GRAPHICS_MAN_EMPTY_HOOK: u32 = 0x0036_9ff0;

    /// `UIStateMgr_RenderStub`.
    pub const UI_STATE_MGR_RENDER_STUB: u32 = 0x0039_eca8;

    /// `UIList_PushBack`.
    pub const UI_LIST_PUSH_BACK: u32 = 0x0039_7910;

    /// `UIList_First`.
    pub const UI_LIST_FIRST: u32 = 0x0039_7840;

    /// `UIListNode_Unlink`.
    pub const UI_LIST_NODE_UNLINK: u32 = 0x0039_7740;

    /// `World_ResetSections`.
    pub const WORLD_RESET_SECTIONS: u32 = 0x003a_6800;

    /// `strcpy`.
    pub const STRLEN_AND_STRCPY: u32 = 0x0041_66f4;

    /// `cBXString_assign_cstr`.
    pub const CBXSTRING_ASSIGN_CSTR: u32 = 0x0031_84e8;

    /// `LobbyMan_RequestStatsPage`.
    pub const LOBBY_MAN_REQUEST_STATS_PAGE: u32 = 0x0025_e348;

    /// `SSXApp_BootLoad`.
    pub const SSX_APP_BOOT_LOAD: u32 = 0x0022_82d0;

    /// `UStr_FromAscii_2`.
    pub const USTR_FROM_ASCII: u32 = 0x002c_2540;

    /// `PATH_SetTrackControlValue`.
    pub const PATH_SET_TRACK_CONTROL_VALUE: u32 = 0x003d_0d70;

    /// `WorldTriggerManager_GetTriggerInfo`.
    pub const WORLD_TRIGGER_GET_INFO: u32 = 0x002b_63d0;

    /// `WorldTriggerManager_GetTriggerName`.
    pub const WORLD_TRIGGER_GET_NAME: u32 = 0x002b_65b0;

    /// `StreamBuffer_GetStatus`.
    pub const STREAM_BUFFER_GET_STATUS: u32 = 0x002a_a020;

    /// `MenuSys_GetInputCode`.
    pub const MENU_SYS_GET_INPUT_CODE: u32 = 0x002c_c248;

    /// `IrradianceRef_GetEntry`.
    pub const IRRADIANCE_REF_GET_ENTRY: u32 = 0x002e_e010;

    /// `cWorldSound_UnlinkEmitters`.
    pub const C_WORLD_SOUND_UNLINK_EMITTERS: u32 = 0x002b_6c20;

    /// `RiderMetrix_BankCombo`.
    pub const RIDER_METRIX_BANK_COMBO: u32 = 0x0011_7718;

    /// `__builtin_delete`.
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}


/// `void WorldTriggerInstance_Reset(WorldTriggerInstance* t)` (0x2b5898):
/// clear the instance, set its state to -1, its timer field to -1.0 and arm
/// it from the global trigger flag.
pub fn world_trigger_instance_reset(c: &mut Cpu) {
    c.cycles += 14;
    let inst = Addr(abi::arg(c, 0));
    inst.sw(c, 0);
    inst.off(0x30).sw(c, u64::MAX);
    const GLOBAL_FLAG: u32 = 0x004a_35a8;
    let armed = Addr(GLOBAL_FLAG).lbu(c);
    inst.off(0x34).sw(c, 0);
    c.g[V1][0] = 0x004a_0000;
    c.f[0] = 0xbf80_0000; // -1.0f, as `lui 0xbf80` + `mtc1 $at, $f0`
    inst.off(0x68).write_fpu(c, FpuF32(0xbf80_0000));
    inst.off(0x40).sb(c, armed);
    c.g[V0][0] = armed;
    inst.off(0x6c).sw(c, 0);
    inst.off(0x60).sw(c, 0);
    inst.off(0x64).sw(c, 0);
    abi::ret(c);
}


/// `WorldTriggerInstance* WorldTriggerInstance_ctor(WorldTriggerInstance *self)`
/// (0x2b4b70): clear the instance, arm it from the global trigger flag and
/// set its timer to -1.0.
pub fn world_trigger_instance_ctor(c: &mut Cpu) {
    c.cycles += 27;
    let this = Addr(abi::arg(c, 0));
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = 0x3f80_0000;
    c.g[V0][0] = u64::MAX;
    this.off(0x30).sw(c, u64::MAX);
    c.g[A1][0] = 0x004a_0000;
    this.off(0x2c).write_fpu(c, FpuF32(0x3f80_0000));
    c.g[V0][0] = abi::arg64(c, 0);
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x20).sw(c, 0);
    this.off(0x24).sw(c, 0);
    this.off(0x28).sw(c, 0);
    this.off(0x34).sw(c, 0);
    this.off(0x38).sw(c, 0);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.f[0] = 0xbf80_0000;
    c.g[V1][0] = Addr(0x004a_0000 + 0x35a8).lbu(c);
    this.off(0x68).write_fpu(c, FpuF32(0xbf80_0000));
    this.off(0x40).sb(c, c.g[V1][0]);
    this.off(0x60).sw(c, 0);
    this.off(0x64).sw(c, 0);
    this.off(0x6c).sw(c, 0);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// World triggers.

/// `bool WorldTriggerInstance_Matches(WorldTriggerInstance* t, uint a, uint b,
/// uint c)` (0x2b4c08): whether the trigger's fields at +4/+8/+0x3c equal the
/// three arguments (0 if the first mismatch is at +4, 1 only if all match).
pub fn world_trigger_instance_matches(c: &mut Cpu) {
    let t = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = t.off(0x4).lw(c);
    if c.g[A1][0] != c.g[V0][0] {
        c.g[V0][0] = 0;
    } else {
        c.cycles += 3;
        c.g[V0][0] = t.off(0x8).lw(c);
        if c.g[A2][0] != c.g[V0][0] {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[V1][0] = t.off(0x3c).lw(c);
            if c.g[A3][0] == c.g[V1][0] {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void WorldTriggerManager_ResetPlayedList(WorldTriggerManager* m)`
/// (0x2b6900): clear four words of the played list (the last two set to -1).
pub fn world_trigger_manager_reset_played_list(c: &mut Cpu) {
    c.cycles += 6;
    let m = Addr(abi::arg(c, 0));
    c.g[V0][0] = u64::MAX;
    m.off(0x330).sw(c, 0);
    m.off(0x32c).sw(c, c.g[V0][0]);
    m.off(0x324).sw(c, 0);
    m.off(0x328).sw(c, c.g[V0][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// World sound octree.

/// The children visited after the first (`node+0`), as (key word changed,
/// change, child offset, return address): the key on the stack walks the
/// eight octants.
pub(crate) const OCTANTS: [(u32, i32, u32, u32); 7] = [
    (4, 1, 0x10, 0x002b_86ec),
    (8, 1, 0x18, 0x002b_8718),
    (4, -1, 0x08, 0x002b_8744),
    (0xc, 1, 0x0c, 0x002b_8770),
    (4, 1, 0x1c, 0x002b_879c),
    (8, -1, 0x14, 0x002b_87c8),
    (4, -1, 0x04, 0x002b_87f4),
];


/// One axis of the cell test: is `pos` above the cell's low edge
/// `(cell - margin) * size` (`c.lt.s pos, edge` fails)?
pub(crate) fn ws_above_low(c: &mut Cpu, cell: u32, pos: FpuF32) -> bool {
    let edge = (FpuF32::from_i32(cell as i32) - FpuF32(c.f[3])) * FpuF32(c.f[1]);
    c.f[0] = edge.0;
    let below = pos.lt(edge);
    abi::set_fcc(c, below);
    !below
}


/// The high edge `(cell + margin) * size` (here `cell` is already +1).
pub(crate) fn ws_below_high(c: &mut Cpu, cell: u32, pos: FpuF32) -> bool {
    let edge = (FpuF32::from_i32(cell as i32) + FpuF32(c.f[3])) * FpuF32(c.f[1]);
    c.f[0] = edge.0;
    let above = edge.lt(pos);
    abi::set_fcc(c, above);
    !above
}


/// `void cWorldSound_QueryTreeNode(a0, List** hits, List** all, Query* q,
/// Node* node, const Key* key)` (0x2b83f8): if the query position lies in
/// the node's cell (`key = {level, x, y, z}`, size `2^level`, margin
/// `gp-0x452c`), appends the node's sound emitters (list `+0x28`, type 5)
/// to `*hits` and, unless the level is 11, recurses into the eight
/// children (`+0..+0x1c`) with their keys built on the stack. (The
/// original's "whole subtree" branch for classification 0 is unreachable:
/// the classification is always 1 or 2.)
pub fn world_sound_query_tree_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x70);
        c.f[3] = fpu(c, gp(c, -0x452c)).0;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[T0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0x10);
        let key = r32(c, T1);
        c.g[T2][0] = lw(c, p(key, 4));
        c.g[V0][0] = lw(c, key);
        c.g[V1][0] = lw(c, r32(c, S3));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x7f) << 23);
        c.f[1] = r32(c, V0);
        c.g[S4][0] = c.g[A0][0];
        let pos = r32(c, V1);
        let px = fpu(c, pos);
        c.f[2] = px.0;
        let inside = 'cell: {
            if !ws_above_low(c, r32(c, T2), px) {
                c.cycles += 1;
                break 'cell false;
            }
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, T2).wrapping_add(1));
            if !ws_below_high(c, r32(c, V0), px) {
                break 'cell false;
            }
            c.cycles += 10;
            c.g[V0][0] = lw(c, p(key, 8));
            let py = fpu(c, p(pos, 4));
            c.f[2] = py.0;
            if !ws_above_low(c, r32(c, V0), py) {
                break 'cell false;
            }
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            if !ws_below_high(c, r32(c, V0), py) {
                break 'cell false;
            }
            c.cycles += 10;
            c.g[V0][0] = lw(c, p(key, 0xc));
            let pz = fpu(c, p(pos, 8));
            c.f[2] = pz.0;
            if !ws_above_low(c, r32(c, V0), pz) {
                break 'cell false;
            }
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            if !ws_below_high(c, r32(c, V0), pz) {
                c.cycles += 1;
                break 'cell false;
            }
            true
        };
        c.g[V1][0] = if inside { 2 } else { 1 };
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 2;
        if inside {
            // Append the node's emitters (type 5) to *hits.
            c.cycles += 3;
            c.g[A1][0] = lw(c, at(c, S0, 0x28));
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A2][0] = 5;
                c.g[V0][0] = lw(c, at(c, A1, 8));
                loop {
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A2][0] {
                        c.cycles += 8;
                        c.g[A0][0] = lw(c, at(c, S2, 0));
                        c.g[V1][0] = lw(c, r32(c, A0));
                        c.g[V0][0] = sx(r32(c, V1) << 2);
                        c.g[V0][0] = addu(c.g[A0][0], c.g[V0][0]);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                        sw_reg(c, at(c, V0, 4), A1);
                        sw_reg(c, r32(c, A0), V1);
                    }
                    c.g[A1][0] = lw(c, at(c, A1, 0));
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        c.cycles += 1;
                        break;
                    }
                    c.g[V0][0] = lw(c, at(c, A1, 8));
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_8638);
                    }
                }
            }
            c.g[A1][0] = lw(c, at(c, T1, 0));
            c.cycles += 3;
            c.g[V0][0] = 0xb;
            let last_level = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
            if !last_level {
                // The first child's key {level - 1, 2x, 2y, 2z} on the stack.
                c.cycles += 12;
                let key = r32(c, T1);
                let sp = sp(c);
                c.g[A0][0] = lw(c, p(key, 0xc));
                c.g[V0][0] = lw(c, p(key, 4));
                c.g[V1][0] = lw(c, p(key, 8));
                c.g[A0][0] = sx(r32(c, A0) << 1);
                c.g[T0][0] = lw(c, at(c, S0, 0));
                c.g[V0][0] = sx(r32(c, V0) << 1);
                c.g[V1][0] = sx(r32(c, V1) << 1);
                sw_reg(c, sp, A1);
                sw_reg(c, p(sp, 4), V0);
                sw_reg(c, p(sp, 8), V1);
                sw_reg(c, p(sp, 0xc), A0);
                if c.g[T0][0] != 0 {
                    c.cycles += 6;
                    world_sound_query_child(c, 0x002b_86c0)?;
                }
                for (word, change, child, ret) in OCTANTS {
                    c.cycles += 5;
                    let at_key = p(sp, word);
                    c.g[V0][0] = lw(c, at_key);
                    c.g[T0][0] = lw(c, at(c, S0, child));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(change as u32));
                    sw_reg(c, at_key, V0);
                    if c.g[T0][0] != 0 {
                        c.cycles += 6;
                        world_sound_query_child(c, ret)?;
                    }
                }
            }
            c.cycles += 1;
        }
        c.cycles += 7;
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


/// Recurse into the child in t0 with the key on the stack.
pub(crate) fn world_sound_query_child(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[T1][0] = c.g[SP][0];
    callee::WORLD_SOUND_QUERY_TREE_NODE.call(c, ret)
}


// ---------------------------------------------------------------------------
// Helpers.

/// A `gp_2`-relative global.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}


/// The shared shape of a one-call forwarding wrapper: charge `cycles`, open a
/// `size`-byte frame with `ra` saved at `size - 0x10`, run `adjust` (the
/// argument fix-ups the original does after the frame is open), call `target`
/// and return to `return_to`, then the epilogue. The callee's `v0` is the
/// result, as in the original.
pub(crate) fn forward(
    c: &mut Cpu,
    cycles: u64,
    size: u32,
    adjust: impl FnOnce(&mut Cpu),
    target: u32,
    return_to: u32,
) {
    guest(c, |c| {
        c.cycles += cycles;
        let frame = Frame::open(c, size);
        frame.save_ra(c, size - 0x10);
        adjust(c);
        call_ptr(c, target, return_to)?;
        frame.restore_ra(c, size - 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `int WorldTrigger_GetInfo(int idx, int *type, int *a, int *b, int c, int d)`
/// (0x002b5818): the singleton trigger manager's info query.
pub fn world_trigger_get_info(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = Addr(gp_2(c, 0x21e4)).lw(c),
        callee::WORLD_TRIGGER_GET_INFO,
        0x002b_582c,
    )
}


/// `int WorldTrigger_GetName(int idx, char *dst)` (0x002b5878): the singleton
/// trigger manager's name query.
pub fn world_trigger_get_name(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = Addr(gp_2(c, 0x21e4)).lw(c),
        callee::WORLD_TRIGGER_GET_NAME,
        0x002b_588c,
    )
}


/// `void cWorldSound_OnSectionUnloaded(this, void *section)` (0x002b6b30):
/// drop the section's emitters.
pub fn c_world_sound_on_section_unloaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let no_section = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_section {
            c.cycles += 2;
            call_ptr(c, callee::C_WORLD_SOUND_UNLINK_EMITTERS, 0x002b_6b44)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldTriggerInstance_dtor(WorldTriggerInstance *this, int flags)`
/// (0x2b4be0).
pub fn world_trigger_instance_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002b_4bf8))
}


/// `void cWorldSound_dtor(cWorldSound *this, int flags)` (0x2b6ae0).
pub fn c_world_sound_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002b_6af8))
}


/// `bool cWorldSound_OnSectionLoaded(cWorldSound *this, Section *s)`
/// (0x2b6b08): relocate the emitters when a section arrived; always true.
pub fn c_world_sound_on_section_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let section = c.g[A1][0];
        frame.save_ra(c, 0);
        if section != 0 {
            c.cycles += 2;
            callee::C_WORLD_SOUND_RELOCATE_EMITTERS.call(c, 0x002b_6b1c)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World triggers.

/// `WorldTriggerManager_FindFreeInstance(WorldTriggerManager* m)` (0x2b5b90):
/// the first instance (stride 0x70) whose `active` word is 0, or null after
/// 40 active instances.
pub fn world_trigger_manager_find_free_instance(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if active {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 40) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x70));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            if events::event_due(c) {
                events::deliver_event_here(c, 0x002b_5b98);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
    }
}


/// `WorldTriggerManager_IsTriggerFlag4(m, i)` (0x2b67d8): for an instance
/// index >= 2, whether the trigger object's word at `+0x4` is 1.
pub fn world_trigger_manager_is_trigger_flag4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x2) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    if small {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x33c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
    c.g[V0][0] ^= 1;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}


/// `WorldTriggerManager_IsTriggerFlag8(m, i)` (0x2b6988): the same for the
/// trigger object's word at `+0x8`.
pub fn world_trigger_manager_is_trigger_flag8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A1][0] < 0x2) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
    if small {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x33c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
    c.g[V0][0] ^= 1;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// WorldTriggerManager_FlagAllActive.

/// `void WorldTriggerManager_FlagAllActive(WorldTriggerManager* self)`
/// (0x2b6a68): for all 40 trigger slots (0x70 bytes each, the array at
/// `*self`), set the `+0x34` flag of every slot whose first word is 1.
pub fn world_trigger_manager_flag_all_active(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = 1;
    c.g[A2][0] = 0;
    c.g[A1][0] = 0x27;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    loop {
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let not_active = c.g[V0][0] != c.g[A3][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x70));
        if !not_active {
            c.cycles += 1;
            let slot = (c.g[V1][0] as u32).wrapping_add(0x34);
            let one = c.g[V0][0];
            c.sw(slot, one);
        }
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002b_6a78);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}


/// `int WorldTrigger_IsType5(int index)` (0x2B5838): look the trigger up in the
/// world trigger manager and report whether its type is 5.
pub fn world_trigger_is_type5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        let manager = c.lw_r_2(A0, gp_3(c, 0x21e4));
        let _ = manager;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = c.g[SP][0] | 4;
        c.g[T0][0] = c.g[SP][0] | 8;
        c.g[T1][0] = 0;
        c.g[T2][0] = 1;
        callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO.call(c, 0x002b_585c)?;
        let kind = c.lw_r_2(V0, r(c, SP));
        let ra = Addr(r(c, SP).wrapping_add(0x10)).ld(c);
        c.g[RA][0] = ra;
        let diff = (kind as u32) ^ 5;
        c.g[V0][0] = (diff < 1) as u64;
        frame.close(c);
        Ok(())
    })
}


/// `cWorldSound* cWorldSound_ctor(cWorldSound* self)` (0x2B6AA0): clear each of
/// the two 0x120-byte trigger slots and give it the shared trigger data
/// (`D_004FF120`) at +0x110.
pub fn world_sound_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0050_0000;
    c.g[V1][0] = c.g[A0][0];
    c.g[A3][0] = sx(0x0050_0000u32.wrapping_sub(0xee0));
    c.g[A1][0] = 1;
    c.g[A2][0] = u64::MAX;
    loop {
        c.cycles += 7;
        Addr(r(c, V1)).sw(c, 0);
        c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
        c.lq(V0, r(c, A3));
        let slot = r(c, V1).wrapping_add(0x110);
        c.sq(V0, slot);
        let more = c.g[A1][0] != c.g[A2][0];
        c.inc_r_2(V1, 0x120);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_6ab8);
        }
    }
}


/// `int WorldTriggerManager_IsDuplicateActive(Trigger* triggers, int a1, int
/// a2, Trigger* except)` (0x2b66e8): index of the first of the manager's 40
/// active triggers with the same `(a1, a2)` that is not `except`, or 0.
pub fn world_trigger_manager_is_duplicate_active(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = Addr(r32(c, A0)).lw(c);
    c.g[V1][0] = 0;
    c.g[T0][0] = 1;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        // `compare_done` marks the path that already computed v0 at the
        // compare; every other path charges one more cycle and recomputes it.
        let mut compare_done = false;
        if c.g[V0][0] != c.g[T0][0] {
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        } else {
            c.cycles += 2;
            let is_except = c.g[A0][0] == c.g[A3][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !is_except {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).off(0x4).lw(c);
                if c.g[V0][0] == c.g[A1][0] {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, A0)).off(0x8).lw(c);
                    let other_key = c.g[V0][0] != c.g[A2][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 40) as u64;
                    if other_key {
                        compare_done = true;
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        abi::ret(c);
                        return;
                    }
                }
            }
        }
        if !compare_done {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 40) as u64;
        }
        c.cycles += 2;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x70));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_66f8);
        }
    }
}


/// `void* WorldTriggerManager_GetSpecialTriggerInfo(WorldTriggerManager* m,
/// int id, int* a, int* b, int* c)` (0x2b6550): return the special trigger's
/// info of `id`, zeroing the three output words.
pub fn world_trigger_manager_get_special_trigger_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 0x4d;
        let t1 = c.g[T1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = t1;
        Addr(r32_2(c, A2)).sw(c, 0);
        Addr(r32_2(c, A3)).sw(c, 0);
        let other = c.g[A1][0] != c.g[V0][0];
        Addr(r32_2(c, T0)).sw(c, 0);
        if !other {
            c.cycles += 9;
            c.g[V0][0] = 0x1;
            Addr(r32_2(c, T0)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x0028_b180, 0x002b_6584)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x002a_34d0, 0x002b_6598)?;
            c.g[V1][0] = c.g[V0][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int WorldTriggerManager_WasPlayed(WorldTriggerManager* m, int id)`
/// (0x2b6808): mark `id` as the last played trigger and report whether it
/// had already been played.
pub fn world_trigger_manager_was_played(c: &mut Cpu) {
    c.cycles += 3;
    let mgr = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V0][0] = Addr(mgr.wrapping_add(0x32c)).lw(c);
    if id != c.g[V0][0] {
        c.g[V0][0] = Addr(mgr.wrapping_add(0x324)).lw(c);
        'scan: {
            c.cycles += 3;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let negative = (c.g[V1][0] as i64) < 0;
            Addr(mgr.wrapping_add(0x32c)).sw(c, id);
            if negative {
                break 'scan;
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(mgr));
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32_2(c, A2)).lw(c);
                let hit = c.g[V0][0] == id;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                if hit {
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0x1;
                    Addr(mgr.wrapping_add(0x330)).sw(c, c.g[V1][0]);
                    abi::ret(c);
                    return;
                }
                c.cycles += 2;
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(4));
                if !more {
                    break 'scan;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_6848);
                }
            }
        }
        c.cycles += 3;
        Addr(mgr.wrapping_add(0x330)).sw(c, 0);
        c.g[V0][0] = 0;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(mgr.wrapping_add(0x330)).lw(c);
        abi::ret(c);
    }
}


/// `WorldTriggerManager_MarkPlayed(WorldTriggerManager* self, int id)`
/// (0x2b6868): remember the id as played (append to the 200-entry ring,
/// invalidate the cached last id) unless it already was.
pub fn world_trigger_manager_mark_played(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let id = c.g[A1][0];
        c.g[S1][0] = id;
        call_ptr(c, 0x002b_6808, 0x002b_6884)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 4;
            c.g[V1][0] = lw_2(c, p(this as u32, 0x324));
            c.g[V0][0] = ((c.g[V1][0] as i64) < 200) as u64;
            let full = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if !full {
                c.cycles += 5;
                c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
                sw(c, p(c.g[V0][0] as u32, 4), id);
                c.g[V1][0] = lw_2(c, p(this as u32, 0x324));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                sw(c, p(this as u32, 0x324), c.g[V1][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw(c, p(this as u32, 0x32c), c.g[V0][0]);
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cWorldSound_InsertEmitters(cWorldSound* self, EmitterEntry* entries,
/// int unused, int n)` (0x2b6c90): insert each active emitter entry, 0x18
/// bytes apiece; always true.
pub fn c_world_sound_insert_emitters(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        let self_ = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = self_;
        frame.save(c, S1, 0x20);
        let n = c.g[A3][0];
        let none = (n as i64) <= 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            let mut entries = c.g[A1][0];
            c.g[S0][0] = entries;
            let mut count = n;
            c.g[S1][0] = count;
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, p(entries as u32, 0xc));
                c.g[A0][0] = self_;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = entries;
                    call_ptr(c, 0x002b_7318, 0x002b_6ccc)?;
                }
                c.cycles += 3;
                count = sx((count as u32).wrapping_sub(1));
                c.g[S1][0] = count;
                let more = count != 0;
                entries = sx((entries as u32).wrapping_add(0x18));
                c.g[S0][0] = entries;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_6cb8);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WorldTriggerManager_StopAll(WorldTriggerManager* m)` (0x2b5e68):
/// stop and clear every active trigger in the 40 trigger slots.
pub fn world_trigger_manager_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        sqsp(c, 16, 0x40);
        sqsp(c, 17, 0x30);
        c.g[S0][0] = 0;
        sqsp(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 19, 0x10);
        c.g[S2][0] = 0x27;
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        loop {
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[V1][0] = abi::addu(c.g[S0][0], c.g[V1][0]);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let active = c.g[V0][0] == c.g[S3][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            if active {
                c.cycles += 4;
                Addr(c.g[V1][0] as u32).sw(c, 0);
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = abi::addu(c.g[A0][0], c.g[S0][0]);
                call::call_ptr(c, 0x002b_5758, 0x002b_5eb4)?;
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5e90);
            }
        }
        c.cycles += 7;
        lqsp(c, 16, 0x40);
        lqsp(c, 17, 0x30);
        lqsp(c, 18, 0x20);
        lqsp(c, 19, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    });
}


/// `int WorldTriggerManager_CheckCrossing(self, trigger, state)`
/// (0x002b69b8): test a trigger against the world position and report a change.
pub fn world_trigger_manager_check_crossing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.swc1(20, frame.at(c, 0x20));
        c.g[A1][0] = Addr(at(c, A1, 0xc)).lw(c);
        let absent = c.g[A1][0] == 0;
        c.g[V0][0] = 0;
        if !absent {
            c.cycles += 10;
            c.g[V0][0] = Addr(at(c, A1, 0xc)).lw(c);
            c.lwc1(20, at(c, S0, 0x68));
            c.g[A0][0] = Addr(at(c, V0, 0xf0)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xf4)).lw(c);
            c.g[A0][0] = add32(word(c, A1), word(c, A0));
            call_ptr(c, word(c, V1), 0x002b_69f0)?;
            c.c_lt_s(0, 20);
            c.g[V1][0] = c.fcc() as u64;
            if !c.fcc() {
                c.cycles += 1;
                c.g[V1][0] = 0;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, S0, 0x6c)).lw(c);
            let was = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            if !was {
                c.cycles += 1;
                c.g[A0][0] = c.g[V1][0] ^ 1;
            }
            c.cycles += 3;
            Addr(at(c, S0, 0x6c)).sw(c, c.g[V1][0]);
            c.g[V0][0] = c.g[A0][0];
            c.swc1(0, at(c, S0, 0x68));
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.lwc1(20, frame.at(c, 0x20));
        frame.close(c);
        Ok(())
    })
}


/// `void cWorldSound_RemoveEmitter(self, id)` (0x2b77c0): clear the id from
/// both emitter tables (0x120 bytes each).
pub fn cworldsound_remove_emitter(c: &mut Cpu) {
    c.cycles += 6;
    c.g[T4][0] = c.g[A0][0];
    c.g[T2][0] = 0x120;
    c.g[A0][0] = 0;
    c.g[T1][0] = 0x120;
    mul32(c, V0, A0, T2);
    loop {
        c.cycles += 5;
        c.g[A2][0] = 0;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
        c.g[V0][0] = lw_at(c, c.g[V1][0] as u32);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if !empty {
            c.cycles += 7;
            mul32(c, V0, A0, T1);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
            c.g[V0][0] = c.g[V1][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            loop {
                c.cycles += 4;
                c.g[V0][0] = lw_at(c, c.g[V1][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                let other = c.g[V0][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                if !other {
                    c.cycles += 1;
                    c.sw(c.g[A0][0] as u32, 0);
                }
                c.cycles += 4;
                c.g[V0][0] = lw_at(c, c.g[A3][0] as u32);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_7808);
                }
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[T0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 2) as u64;
        let more = c.g[V0][0] != 0;
        mul32(c, V0, A0, T2);
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_77d8);
        }
    }
}


/// `void WorldTriggerManager_dtor(WorldTriggerManager* self, int flags)`
/// (0x2b5988): destroy the trigger instances backwards, free their array,
/// then delete the manager when bit 0 of `flags` says so.
pub fn world_trigger_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = flags;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(r32(c, S1));
        let empty = c.g[A1][0] == 0;
        c.g[V0][0] = 0x70;
        if !empty {
            c.cycles += 5;
            c.g[V1][0] = c.lw(r32(c, A1).wrapping_sub(0x10));
            c.mult(0, 4, 3, 2);
            c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = c.lw(r32(c, S1));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x70));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::WORLD_TRIGGER_INSTANCE_DTOR, 0x002b_59d4)?;
                    c.g[V0][0] = c.lw(r32(c, S1));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x70));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(r32(c, S1));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_59c8);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x10));
            call_ptr(c, callee::OPERATOR_DELETE, 0x002b_59ec)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let keep = c.g[V0][0] == 0;
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x002b_5a00)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `WorldTriggerManager_AnyTriggerFlagged` (0x2b6740): true when any of the
/// four trigger ids the world sound gives for `a1` has flag 4 set.
pub fn world_trigger_manager_any_trigger_flagged(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x002b_675c);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        call_ptr(c, callee::AUDIO_GET_GAME_MODULE, 0x002b_675c)?;
        c.g[A0][0] = Addr(p(r(c, V0), 0x4c)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = c.g[SP][0] | 4;
        c.g[T0][0] = c.g[SP][0] | 8;
        c.g[RA][0] = sx(0x002b_6778);
        c.g[T1][0] = c.g[SP][0] | 0xc;
        callee::WORLD_SOUND_GET_TRIGGER_IDS.call(c, 0x002b_6778)?;
        c.g[A1][0] = Addr(r(c, SP)).lw(c);
        c.g[RA][0] = sx(0x002b_6784);
        c.g[A0][0] = c.g[S1][0];
        callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_6784)?;
        let any = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !any {
            c.cycles += 5;
            c.g[A1][0] = Addr(p(r(c, SP), 4)).lw(c);
            c.g[RA][0] = sx(0x002b_6798);
            c.g[A0][0] = c.g[S1][0];
            callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_6798)?;
            let any = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !any {
                c.cycles += 5;
                c.g[A1][0] = Addr(p(r(c, SP), 8)).lw(c);
                c.g[RA][0] = sx(0x002b_67ac);
                c.g[A0][0] = c.g[S1][0];
                callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_67ac)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 4;
                    c.g[A1][0] = Addr(p(r(c, SP), 0xc)).lw(c);
                    c.g[RA][0] = sx(0x002b_67c0);
                    c.g[A0][0] = c.g[S1][0];
                    callee::WORLD_TRIGGER_IS_FLAG4.call(c, 0x002b_67c0)?;
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                } else {
                    c.g[V0][0] = 1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void* WorldTriggerManager_FindInstance(self, a, b, c)` (0x2b5bc0): walk
/// the manager's 40 instance slots and return the first one matching.
pub fn world_trigger_manager_find_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[A1][0];
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            let empty = c.g[V1][0] == 0;
            c.g[A1][0] = c.g[S5][0];
            let mut found = false;
            if !empty {
                c.cycles += 5;
                c.g[A2][0] = c.g[S4][0];
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x002b_4c08, 0x002b5c18)?;
                let miss = c.g[V0][0] == 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                if !miss {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    c.g[V0][0] = sx(p(r32(c, V0), r32(c, S0)));
                    found = true;
                }
            } else {
                c.cycles += 1;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            }
            if found {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 40) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x70));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5bf8);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int WorldTriggerManager_CountActiveOthers(self, int kind)` (0x2b6630):
/// count active triggers of the same kind whose name is not the excluded one.
pub fn world_trigger_manager_count_active_others(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = 0x27;
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = 0x4a0000;
        c.g[S6][0] = 0x1;
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if c.g[V1][0] != c.g[S6][0] {
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            } else {
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x35a8));
                call_ptr(c, 0x0041_65a8, 0x002b_6690)?;
                let same_name = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                if !same_name {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
                    c.g[V1][0] = c.g[V1][0] ^ c.g[S4][0];
                    if c.g[V1][0] != 0 {
                        c.g[S1][0] = c.g[A0][0];
                    }
                }
                c.cycles += 1;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
            }
            c.cycles += 2;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_6670);
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World trigger manager.

/// `cWorldTriggerManager* cWorldTriggerManager_ctor(cWorldTriggerManager* m)`
/// (0x2b58d0): allocate and construct the trigger instance array (0x28
/// entries), publish it and load the trigger definitions.
pub fn world_trigger_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = u64::MAX;
        frame.save(c, S1, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x50);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[A1][0] = sx(0x0048_35a0);
        frame.save(c, S4, 0x10);
        c.g[A0][0] = 0x1190;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        Addr(p(this, 0x324)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[S2][0] = 0x27;
        Addr(p(this, 0x330)).sw(c, 0);
        Addr(p(this, 0x32c)).sw(c, c.g[V1][0]);
        c.g[S4][0] = u64::MAX;
        Addr(p(this, 0x328)).sw(c, c.g[V1][0]);
        c.g[RA][0] = sx(0x002b_592c);
        Addr(p(this, 0x334)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x002b_592c)?;
        c.g[V1][0] = 0x28;
        c.g[S3][0] = sx(r(c, V0).wrapping_add(0x10));
        Addr(r(c, V0)).sw(c, c.g[V1][0]);
        c.g[S0][0] = c.g[S3][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002b_594c);
            c.g[S2][0] = sx(r(c, S2).wrapping_sub(1));
            callee::WORLD_TRIGGER_INSTANCE_CTOR.call(c, 0x002b_594c)?;
            let more = c.g[S2][0] != c.g[S4][0];
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0x70));
            if !more {
                c.cycles += 13;
                Addr(r(c, S1)).sw(c, c.g[S3][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002b_5964);
                Addr(p(r(c, S1), 0x338)).sw(c, 0);
                callee::WORLD_TRIGGER_MANAGER_LOAD.call(c, 0x002b_5964)?;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, 16, 0x50);
                frame.restore(c, 17, 0x40);
                frame.restore(c, 18, 0x30);
                frame.restore(c, 19, 0x20);
                frame.restore(c, 20, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_5940);
            }
        }
    })
}


// ---------------------------------------------------------------------------
// World triggers.

/// `void WorldTriggerManager_AddTrigger(WorldTriggerManager* mgr, int a1,
/// int a2, int a3, int count, int a5, Vec4* pos)` (0x2b5c68): update the
/// matching trigger instance's radius, or fill the first free instance from
/// the arguments, and bump the instance's use counter.
pub fn world_trigger_manager_add_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(r32v(c, SP).wrapping_sub(0xb0));
        c.sq(16, spa(c, 0x90));
        c.sq(17, spa(c, 0x80));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, spa(c, 0x70));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, spa(c, 0x60));
        c.g[S2][0] = c.g[A3][0];
        c.sq(20, spa(c, 0x50));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, spa(c, 0x40));
        c.g[S4][0] = c.g[T0][0];
        c.swc1(20, spa(c, 0xa0));
        c.g[S5][0] = c.g[T1][0];
        c.sd(spa(c, 0x30), c.g[RA][0]);
        c.mov_s(20, 12);
        // The position is passed by value; keep its 12 bytes on the frame.
        let mgr = r32v(c, S0);
        c.g[V0][0] = Addr(at_3(mgr, 0x334)).lw(c);
        c.ldl(3, at_3(r32v(c, T2), 7));
        c.ldr(3, at_3(r32v(c, T2), 0));
        c.g[A0][0] = Addr(at_3(r32v(c, T2), 8)).lw(c);
        c.sdl(3, spa(c, 7));
        c.sdr(3, spa(c, 0));
        let no_instances = c.g[V0][0] == 0;
        c.sw(spa(c, 8), c.g[A0][0]);
        if !no_instances {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x002b_5cd8);
            c.g[A2][0] = c.g[S1][0];
            callee::WORLD_TRIGGER_FIND_INSTANCE.call(c, 0x002b_5cd8)?;
            let found = c.g[V0][0];
            c.g[V1][0] = found;
            if found != 0 {
                // Existing instance: just take the new radius.
                c.swc1(20, at_3(found as u32, 0x10));
            } else {
                c.cycles += 24;
                c.lwc1(3, spa(c, 0));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(0, spa(c, 4));
                c.lwc1(1, spa(c, 8));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 2);
                c.swc1(3, spa(c, 0x20));
                c.swc1(0, spa(c, 0x24));
                c.swc1(1, spa(c, 0x28));
                c.swc1(2, spa(c, 0x2c));
                c.lq(2, spa(c, 0x20));
                c.g[RA][0] = sx(0x002b_5d18);
                c.sq(2, spa(c, 0x10));
                callee::WORLD_TRIGGER_FIND_FREE.call(c, 0x002b_5d18)?;
                let inst = c.g[V0][0] as u32;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                c.sw(at_3(inst, 4), c.g[S3][0]);
                c.sw(at_3(inst, 0), c.g[V0][0]);
                c.sw(at_3(inst, 8), c.g[S1][0]);
                c.sw(at_3(inst, 0x3c), c.g[S2][0]);
                c.sw(at_3(inst, 0xc), c.g[S4][0]);
                c.swc1(20, at_3(inst, 0x10));
                c.sw(at_3(inst, 0x14), c.g[S5][0]);
                c.lq(2, spa(c, 0x10));
                c.sq(2, at_3(inst, 0x20));
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            c.sw(at_3(r32v(c, V1), 0x34), c.g[V0][0]);
        }
        c.cycles += 10;
        c.lq(16, spa(c, 0x90));
        c.lq(17, spa(c, 0x80));
        c.lq(18, spa(c, 0x70));
        c.lq(19, spa(c, 0x60));
        c.lq(20, spa(c, 0x50));
        c.lq(21, spa(c, 0x40));
        c.g[RA][0] = c.ld(spa(c, 0x30));
        c.lwc1(20, spa(c, 0xa0));
        c.g[SP][0] = sx(r32v(c, SP).wrapping_add(0xb0));
        abi::ret(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// 0x002b63d0 WorldTriggerManager_GetTriggerInfo
// ---------------------------------------------------------------------------

/// `int WorldTriggerManager_GetTriggerInfo(WTM* self, int idx, int* type,
/// int* a, int* b, int c, int d)` (0x2b63d0).
///
/// Looks the trigger entry up by index: type 3 is a "disabled" entry, type 1
/// hands out the entry's two object fields and type 2 picks one of the
/// object's four random thresholds with `Audio_Rand`. Anything else is
/// delegated to `GetSpecialTriggerInfo` unless `d` says not to. Returns 1 when
/// the index was in range.
pub fn world_trigger_manager_get_trigger_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.sw(c.g[T0][0] as u32, 0);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x340));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A3][0];

        'ret: {
            if !in_range {
                c.cycles += 4;
                c.sw(c.g[A2][0] as u32, 0);
                c.g[V0][0] = 0;
                c.sw(c.g[S0][0] as u32, 0);
                break 'ret;
            }
            'found: {
                'not3: {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x33c));
                    c.g[S2][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[V0][0] = 3;
                    c.g[A0][0] = sx(c.g[S2][0] as u32 + c.g[V1][0] as u32);
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S3][0] = 1;
                    if t {
                        break 'not3;
                    } else {
                        c.cycles += 3;
                        c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                        c.sw(c.g[S0][0] as u32, 0);
                        break 'found;
                    }
                }
                'not1: {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[S3][0];
                    c.g[V0][0] = 2;
                    if t {
                        break 'not1;
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
                        c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                        c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                        c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x33c));
                        c.g[V0][0] = sx(c.g[S2][0] as u32 + c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 4));
                        c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                        c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                        break 'found;
                    }
                }
                'type2: {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    if t {
                        break 'type2;
                    }
                    c.cycles += 12;
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x10));
                    c.g[RA][0] = sx(0x002b_647c);
                    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
                    call_ptr(c, 0x0028_b180, 0x002b_647c)?;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x118));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[RA][0] = sx(0x002b_648c);
                    c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 0x1d8));
                    call_ptr(c, 0x002a_df60, 0x002b_648c)?;
                    c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x33c));
                    c.g[A0][0] = 0x7fff;

                    // The four threshold buckets, largest first.
                    let entry = (c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32);
                    c.cycles += 11;
                    c.g[V0][0] = c.g[V0][0] & 0x7fff;
                    c.g[V1][0] = sx(entry);
                    c.g[A1][0] = c.lw(p(c.g[V1][0] as u32, 4));
                    c.g[V1][0] = c.lw(p(c.g[A1][0] as u32, 0x34));
                    c.g[A2][0] = c.lw(p(c.g[A1][0] as u32, 0x24));
                    c.mult(0, 2, 2, 3);
                    c.divu(0, 2, 4);
                    c.g[V1][0] = c.lo[0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.lw(p(c.g[A1][0] as u32, 0x28));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x14));
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        break 'found;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(c.g[A0][0] as u32 + c.g[A2][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x2c));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x18));
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        break 'found;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(c.g[V0][0] as u32 + c.g[A0][0] as u32);
                    c.g[V0][0] = sx(c.g[V0][0] as u32 + c.g[A2][0] as u32);
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x20));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x1c));
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        break 'found;
                    }
                    c.cycles += 2;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    break 'found;
                }
                // Fallback entries: delegate unless `d` is set.
                c.cycles += 2;
                let t = c.g[T2][0] != 0;
                c.g[V0][0] = c.g[S3][0];
                if t {
                    break 'ret;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002b_652c);
                    c.g[A3][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_6550, 0x002b_652c)?;
                    c.g[S3][0] = c.g[V0][0];
                    break 'found;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }

        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
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
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use ssx3_ee as ee;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::*;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::intersect::worldsphtree::tests::SEEDS;
    pub(crate) use crate::game::intersect::worldsphtree::tests::f;
    pub(crate) use crate::game::intersect::worldsphtree::tests::uf;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::replay::replay2::tests::dtor_setup;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::render::particle::tests::clear;
    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::object::movenode::tests::OBJ4;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;
    pub(crate) use crate::game::movie::movieplayer::tests::wh;
    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_6;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ_2;
    pub(crate) use crate::game::fe::fereal::tests::SELF2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_7;
    pub(crate) use crate::game::object::flexrailnode::tests::base_3 as base_8;
    pub(crate) use crate::game::object::flexrailnode::tests::check_all;


    #[test]
    fn world_trigger_instance_reset_matches() {
        check(ssx3_ee::s08::f_002b5898__generated, world_trigger_instance_reset, 200, &|c, seed| {
            let mut r = Rng(0x7777 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(0x004a_35a8, (r.next_u32() & 0xff) as u8);
        });
    }


    #[test]
    fn world_trigger_instance_ctor_matches() {
        check(ssx3_ee::s08::f_002b4b70, world_trigger_instance_ctor, 200, &|c, seed| {
            let mut r = Rng(0x2b4b ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write8(0x004a_35a8, r.next_u32() as u8);
        });
    }


    #[test]
    fn world_trigger_instance_matches_matches() {
        check(ssx3_ee::s08::f_002b4c08, world_trigger_instance_matches, 300, &|c, seed| {
            let mut r = Rng(0x4c08 ^ seed as u64);
            scramble(c, &mut r);
            let a = r.next_u32();
            let b = r.next_u32();
            let d = r.next_u32();
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x4, a);
            c.bus.write32(DATA + 0x8, b);
            c.bus.write32(DATA + 0x3c, d);
            match seed % 4 {
                0 => {
                    c.g[A1][0] = sx(a).wrapping_add(1);
                    c.g[A2][0] = sx(b);
                    c.g[A3][0] = sx(d);
                }
                1 => {
                    c.g[A1][0] = sx(a);
                    c.g[A2][0] = sx(b).wrapping_add(1);
                    c.g[A3][0] = sx(d);
                }
                2 => {
                    c.g[A1][0] = sx(a);
                    c.g[A2][0] = sx(b);
                    c.g[A3][0] = sx(d);
                }
                _ => {
                    c.g[A1][0] = sx(a);
                    c.g[A2][0] = sx(b);
                    c.g[A3][0] = sx(d).wrapping_add(1);
                }
            }
        });
    }


    #[test]
    fn world_trigger_manager_reset_played_list_matches() {
        check(ssx3_ee::s08::f_002b6900, world_trigger_manager_reset_played_list, 50, &|c, seed| {
            let mut r = Rng(0x6900 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for k in 0..8u32 {
                c.bus.write32(DATA + 0x320 + 4 * k, r.next_u32());
            }
        });
    }


    #[test]
    fn world_sound_query_tree_node_matches() {
        check(ee::s08::f_002b83f8__generated, world_sound_query_tree_node, SEEDS, &|c, seed| {
            let mut r = Rng(0x002b_83f8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x452c), if seed % 2 == 0 { 0 } else { f(0.25) });
            let (key, query, pos, hits, all, root) =
                (DATA, DATA + 0x40, DATA + 0x60, DATA + 0x80, DATA + 0x90, DATA + 0x100);
            let level = [11u32, 2, 3, 1][(seed % 4) as usize];
            c.bus.write32(key, level);
            for k in 1..4 {
                c.bus.write32(key + 4 * k, r.next_u32() % 2);
            }
            c.bus.write32(query, pos);
            for k in 0..3 {
                c.bus.write32(pos + 4 * k, f(uf(&mut r, -2.0, 18.0)));
            }
            c.bus.write32(hits, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000, r.next_u32() % 4);
            // Root: up to eight leaf children, a few emitters.
            for i in 0..8 {
                let child = DATA + 0x200 + 0x40 * i;
                c.bus.write32(root + 4 * i, if r.next_u32() % 3 == 0 { 0 } else { child });
            }
            let mut next = 0;
            for i in 0..(r.next_u32() % 4) {
                let e = DATA + 0x1000 + 0x10 * i;
                c.bus.write32(e, next);
                c.bus.write32(e + 8, if r.next_u32() % 2 == 0 { 5 } else { 4 });
                next = e;
            }
            c.bus.write32(root + 0x28, next);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = sx(hits);
            c.g[A2][0] = sx(all);
            c.g[A3][0] = sx(query);
            c.g[T0][0] = sx(root);
            c.g[T1][0] = sx(key);
        });
    }


    /// A zeroed singleton trigger manager: every query is out of range.
    pub(crate) fn trigger_setup(c: &mut Cpu, seed: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_add(0x21e4), DATA + 0x1000);
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = 0;
        c.g[A2][0] = (DATA + 0x800) as u64;
        c.g[A3][0] = (DATA + 0x900) as u64;
    }


    #[test]
    fn world_trigger_get_info_matches() {
        check(ee::s08::f_002b5818__generated, world_trigger_get_info, 100, &|c, seed| {
            trigger_setup(c, seed, 0x2b_5818)
        });
    }


    #[test]
    fn world_trigger_get_name_matches() {
        check(ee::s08::f_002b5878__generated, world_trigger_get_name, 100, &|c, seed| {
            trigger_setup(c, seed, 0x2b_5878)
        });
    }


    #[test]
    fn c_world_sound_on_section_unloaded_matches() {
        check(ee::s08::f_002b6b30__generated, c_world_sound_on_section_unloaded, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2b_6b30, 0);
            c.g[A0][0] = (DATA + 0x1000) as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (DATA + 0x800) as u64 };
            c.bus.write32(DATA + 0x800 + 0xc, r.next_u32() % 2);
        });
    }


    #[test]
    fn world_trigger_instance_dtor_matches() {
        check(ee::s08::f_002b4be0__generated, world_trigger_instance_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2b4be0)
        });
    }


    #[test]
    fn c_world_sound_dtor_matches() {
        check(ee::s08::f_002b6ae0__generated, c_world_sound_dtor, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2b6ae0)
        });
    }


    #[test]
    fn c_world_sound_on_section_loaded_matches() {
        check(ee::s08::f_002b6b08__generated, c_world_sound_on_section_loaded, 80, &|c, seed| {
            let mut r = game(c, seed, 0x2b6b08, 1);
            c.g[A0][0] = SCRATCH as u64;
            // A null section skips the relocate call; a section whose name is
            // not empty makes the relocate return at once.
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (SCRATCH + 0x800) as u64 };
            c.g[A2][0] = r.next_u32() as u64;
            if seed % 2 != 0 {
                c.bus.write8(SCRATCH + 0x800, 0xff);
            }
        });
    }


    #[test]
    fn world_trigger_manager_find_free_instance_matches() {
        check(
            ee::s08::f_002b5b90__generated,
            world_trigger_manager_find_free_instance,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b5b90, 0);
                let base = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, base);
                // Cover a free slot early, a free slot late, and a full table.
                let full = seed % 5 == 0;
                for i in 0..40u32 {
                    let active = if full { 1 } else { ((i + seed) % 7 != 0) as u32 };
                    c.bus.write32(base + 0x70 * i, active);
                    c.bus.write32(base + 0x70 * i + 4, r.next_u32());
                }
            },
        );
    }


    #[test]
    fn world_trigger_manager_is_trigger_flag4_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x2b67d8, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 6) as u64;
            let refs = DATA + 0x400;
            c.bus.write32(DATA + 0x33c, refs);
            for i in 0..6u32 {
                let obj = DATA + 0x600 + 0x10 * i;
                c.bus.write32(refs + 8 * i, r.next_u32());
                c.bus.write32(refs + 8 * i + 4, obj);
                c.bus.write32(obj + 4, ((i + seed) % 3 == 0) as u32);
            }
        };
        check(ee::s08::f_002b67d8__generated, world_trigger_manager_is_trigger_flag4, SEEDS_2, &setup);
    }


    #[test]
    fn world_trigger_manager_is_trigger_flag8_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x2b6988, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 6) as u64;
            let refs = DATA + 0x400;
            c.bus.write32(DATA + 0x33c, refs);
            for i in 0..6u32 {
                let obj = DATA + 0x600 + 0x10 * i;
                c.bus.write32(refs + 8 * i, r.next_u32());
                c.bus.write32(refs + 8 * i + 4, obj);
                c.bus.write32(obj + 8, ((i + seed) % 3 == 0) as u32);
            }
        };
        check(ee::s08::f_002b6988__generated, world_trigger_manager_is_trigger_flag8, SEEDS_2, &setup);
    }


    // -----------------------------------------------------------------------
    // Looping helpers.

    pub(crate) fn trigger_setup_2(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x2b6a_68 ^ seed as u64);
        c.g[GP][0] = GAME_GP as u64;
        let array = DATA + 0x1000;
        c.bus.write32(DATA, array);
        c.g[A0][0] = DATA as u64;
        for i in 0..40u32 {
            let slot = array + i * 0x70;
            let active = match seed % 3 {
                0 => 1,
                1 => 2,
                _ => r.next_u32() % 3,
            };
            c.bus.write32(slot, active);
            c.bus.write32(slot + 0x34, r.next_u32());
        }
    }


    driven!(gen_flag_all_active, ssx3_ee::s08::f_002b6a68__generated);

    driven!(hand_flag_all_active, world_trigger_manager_flag_all_active);


    #[test]
    fn world_trigger_manager_flag_all_active_matches() {
        check(ssx3_ee::s08::f_002b6a68__generated, world_trigger_manager_flag_all_active, 48, &trigger_setup_2);
        check_events(gen_flag_all_active, hand_flag_all_active, 48, &|c, seed| {
            trigger_setup_2(c, seed);
            let mut r = Rng(0x6a68 ^ seed as u64);
            let mode = schedule(c, &mut r, seed, 0x002b_6a68);
            // The trigger sweep is 40 slots long, so a back edge is reached
            // within a few cycles whatever the event schedule says.
            c.next_event = c.cycles + 5;
            mode
        });
    }


    #[test]
    fn world_trigger_is_type5_matches() {
        check(ssx3_ee::s08::f_002b5838__generated, world_trigger_is_type5, 60, &|c, seed| {
            machine(c, seed, 0x2b58_38, 1);
            belib(c);
            c.g[A1][0] = (seed % 6) as u64;
            if seed % 2 == 0 {
                w(c, GAME_GP + 0x21e4, DATA + 0x200);
            }
        });
    }


    #[test]
    fn world_sound_ctor_matches() {
        check(ssx3_ee::s08::f_002b6aa0__generated, world_sound_ctor, SEEDS_3, &|c, seed| {
            machine(c, seed, 0x2b6a_a0, 0);
            belib(c);
            c.g[A0][0] = DATA as u64;
            put_vec(c, 0x004f_f120, [seed | 0x1000, 2, 3, 4]);
        });
    }


    #[test]
    fn world_trigger_manager_is_duplicate_active_matches() {
        check(ee::s08::f_002b66e8__generated, world_trigger_manager_is_duplicate_active, SEEDS_4, &|c, seed| {
            let mut r = base(c, seed, 0x2b66e8);
            let list = DATA + 0x1000;
            clear(c, list, 0x40 * 0x1c);
            let except = list + 0x70 * ((seed % 5) as u32);
            let a1 = r.next_u32() as u64;
            let a2 = r.next_u32() as u64;
            for i in 0..8u32 {
                let e = list + i * 0x70;
                // A few active entries with the queried keys, one of them the
                // exception, so both the found and the not-found exits run.
                c.bus.write32(e, if i % 3 == 0 { 1 } else { 0 });
                c.bus.write32(e + 4, a1 as u32);
                c.bus.write32(e + 8, if i % 2 == 0 { a2 as u32 } else { r.next_u32() });
            }
            c.g[A0][0] = sx(DATA + 0x900);
            c.bus.write32(DATA + 0x900, list);
            c.g[A1][0] = a1;
            c.g[A2][0] = a2;
            c.g[A3][0] = sx(except);
        });
    }


    // ----------------------------------------------------------- triggers

    #[test]
    fn world_trigger_manager_was_played_matches() {
        check(
            ssx3_ee::s08::f_002b6808__generated,
            world_trigger_manager_was_played,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2b6808, 0);
                let id = (seed % 5) as u32;
                let last = if seed % 2 == 0 { id } else { (seed + 1) % 5 };
                w_2(c, OBJ + 0x32c, last);
                w_2(c, OBJ + 0x330, (seed / 2) % 2);
                let count = (seed / 5) % 4;
                w_2(c, OBJ + 0x324, count);
                for i in 0..count {
                    w_2(
                        c,
                        OBJ + 4 + 4 * i,
                        if i + 1 == count { id } else { (seed + i) % 5 },
                    );
                }
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = id as u64;
            },
        );
    }


    #[test]
    fn world_trigger_manager_get_special_trigger_info_matches() {
        check(
            ssx3_ee::s08::f_002b6550__generated,
            world_trigger_manager_get_special_trigger_info,
            8,
            &|c, seed| {
                fakes::machine(c, seed, 0x2b6550, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0x4d
                } else {
                    seed as u64 % 0x4e
                };
                c.g[A2][0] = OBJ2 as u64;
                c.g[A3][0] = OBJ3 as u64;
                c.g[T0][0] = OBJ4 as u64;
                c.g[T1][0] = (OBJ2 + 0x100) as u64;
            },
        );
    }


    #[test]
    fn world_trigger_manager_mark_played_matches() {
        check(
            ssx3_ee::s08::f_002b6868__generated,
            world_trigger_manager_mark_played,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2b6868);
                c.g[A0][0] = DATA as u64;
                let id = if seed % 3 == 0 {
                    0x1111_2222
                } else {
                    r.next_u32()
                };
                c.g[A1][0] = id as u64;
                let count = seed % 4;
                c.bus.write32(DATA + 0x324, count);
                c.bus
                    .write32(DATA + 0x32c, if seed % 2 == 0 { id } else { 0x3333_4444 });
                c.bus.write32(DATA + 0x330, 0);
                for i in 0..count {
                    c.bus.write32(DATA + 4 + 4 * i, r.next_u32());
                }
            },
        );
    }


    #[test]
    fn c_world_sound_insert_emitters_matches() {
        check(
            ssx3_ee::s08::f_002b6c90__generated,
            c_world_sound_insert_emitters,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2b6c90);
                c.g[A0][0] = DATA as u64;
                // The entries pointer arrives in a2 and is moved into a1.
                c.g[A2][0] = (DATA + 0x200) as u64;
                let n = seed % 4;
                c.g[A3][0] = n as u64;
                for i in 0..n {
                    let e = DATA + 0x200 + i * 0x18;
                    // Only the active flag matters; `+0x10` stays null so the
                    // insert takes its early-out.
                    c.bus
                        .write32(e + 0xc, u32::from(seed % 2 == 0 || i % 2 == 0));
                    c.bus.write32(e + 0x10, 0);
                    c.bus.write32(e + 4, r.next_u32());
                }
            },
        );
    }


    #[test]
    fn world_trigger_manager_stop_all_matches() {
        check(ssx3_ee::s08::f_002b5e68__generated, world_trigger_manager_stop_all, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x2b5e68, 0);
            let m = DATA + 0x1000;
            let entries = DATA + 0x2000;
            c.g[A0][0] = m as u64;
            w32(c, m, entries);
            for i in 0..40u32 {
                let e = entries + i * 0x70;
                let active = if (seed as u32 + i) % 5 == 0 { 1 } else { 0 };
                w32(c, e, active);
                w32(c, e + 0x30, r.next_u32());
                w32(c, e + 0x38, 0); // WorldTriggerInstance_Stop early-out
            }
        });
    }


    #[test]
    fn world_trigger_manager_check_crossing_matches() {
        check(
            ee::s08::f_002b69b8__generated,
            world_trigger_manager_check_crossing,
            16,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2b69_b8, 4);
                let trigger = DATA + 0x1000;
                let obj = DATA + 0x1100;
                let vtable = DATA + 0x1200;
                let state = DATA + 0x2000;
                w(c, trigger + 0xc, obj);
                w(c, obj + 0xc, vtable);
                ret_slot(c, vtable, 0xf0, RET_ZERO);
                w(c, state + 0x68, r.edge_f32());
                w(c, state + 0x6c, (seed % 2) as u32);
                c.f[0] = r.edge_f32();
                c.g[A0][0] = (1 + seed) as u64;
                c.g[A1][0] = trigger as u64;
                c.g[A2][0] = state as u64;
            },
        );
    }


    #[test]
    fn cworldsound_remove_emitter_matches() {
        check(
            ee::s08::f_002b77c0__generated,
            cworldsound_remove_emitter,
            SEEDS_6,
            &|c, seed| {
                base_3(c, seed, 0xb77c0, 0);
                let s = DATA + 0x1000;
                c.g[A0][0] = sx(s);
                let id = (seed % 4) as u64;
                c.g[A1][0] = id;
                for t in 0..2u32 {
                    let base_ = s + 0x120 * t;
                    let count = ((seed / (1 + t)) % 4) as u32;
                    c.bus.write32(base_, count);
                    for j in 0..4u32 {
                        c.bus.write32(
                            base_ + 4 + 4 * j,
                            if (j + t) % 3 == 0 {
                                seed % 4
                            } else {
                                0x100 + j
                            },
                        );
                    }
                }
            },
        );
    }


    #[test]
    fn world_trigger_manager_any_trigger_flagged_matches() {
        check(
            ee::s08::f_002b6740__generated,
            world_trigger_manager_any_trigger_flagged,
            SEEDS_7,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let sound = DATA + 0x800;
                w(c, W + 0x1000 + 0x4c, sound);
                w(c, DATA + 0xc00 + 0x8c, DATA + 0x900);
                w(c, DATA + 0x900 + 0x10, DATA + 0xa00);
                w(c, DATA + 0xa00, 1);
                w(c, DATA + 0xa00 + 4, 1);
                w(c, this + 0x33c, DATA + 0x1000);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0xc00) as u64;
            }),
        );
    }


    #[test]
    fn world_trigger_manager_find_instance_matches() {
        check(
            ee::s08::f_002b5bc0__generated,
            world_trigger_manager_find_instance,
            6,
            &|c, seed| {
                let _ = base_5(c, seed, 0x2b5bc0, 2);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = 1;
                c.g[A2][0] = 2;
                c.g[A3][0] = 3;
                w(c, OBJ_2, SELF2);
                for i in 0..40u32 {
                    w(c, SELF2 + 0x70 * i, (i % 4 == 0) as u32);
                }
                let hit = (seed % 40) as u32;
                w(c, SELF2 + 0x70 * hit, 1);
                w(c, SELF2 + 0x70 * hit + 4, 1);
                w(c, SELF2 + 0x70 * hit + 8, 2);
                w(c, SELF2 + 0x70 * hit + 0x3c, 3);
            },
        );
    }


    #[test]
    fn world_trigger_manager_count_active_others_matches() {
        check(
            ee::s08::f_002b6630__generated,
            world_trigger_manager_count_active_others,
            6,
            &|c, seed| {
                let _ = base_6(c, seed, 0x002b_6630, 1);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, OBJ_3, OBJ2_2);
                for i in 0..40u32 {
                    let e = OBJ2_2 + i * 0x70;
                    w(c, e, if seed % 2 == 0 && i % 3 == 0 { 1 } else { 0 });
                    w(c, e + 0xc, (seed % 3) as u32);
                }
            },
        );
    }


    #[test]
    fn world_trigger_manager_ctor_matches() {
        check(ee::s08::f_002b58d0__generated, world_trigger_manager_ctor, 30, &|c, seed| {
            alloc_base(c, seed, 0x2b58, 5);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn world_trigger_manager_add_trigger_matches() {
        check_all(
            ee::s08::f_002b5c68__generated,
            world_trigger_manager_add_trigger,
            200,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x2b_5c68);
                let mgr = DATA;
                let a1 = 0x11u64;
                let a2 = 0x22u64;
                let a3 = 0x33u64;
                c.g[A0][0] = sx(mgr);
                c.g[A1][0] = a1;
                c.g[A2][0] = a2;
                c.g[A3][0] = a3;
                c.g[T0][0] = (r.next_u32() % 8) as u64;
                c.g[T1][0] = (r.next_u32() % 8) as u64;
                c.g[T2][0] = sx(DATA + 0x3000);
                for i in 0..4 {
                    c.bus.write32(DATA + 0x3000 + 4 * i, r.next_u32());
                }
                c.f[12] = r.f32(100.0);
                match seed % 4 {
                    0 => c.bus.write32(mgr + 0x334, 0),
                    _ => {
                        c.bus.write32(mgr + 0x334, DATA + 0x2000);
                        // Instance 0: 0x70 bytes; 0 means free.
                        for i in 0..(40 * 0x70 / 4) {
                            c.bus.write32(DATA + 0x2000 + 4 * i, 0);
                        }
                        if seed % 2 == 0 {
                            // A matching instance: the found path.
                            c.bus.write32(DATA + 0x2000 + 4, a1 as u32);
                            c.bus.write32(DATA + 0x2000 + 8, a2 as u32);
                            c.bus.write32(DATA + 0x2000 + 0x3c, a3 as u32);
                        } else {
                            c.bus.write32(DATA + 0x2000, 1);
                            c.bus.write32(DATA + 0x2000 + 4, 0x99);
                        }
                    }
                }
            },
        );
    }


    // -- 0x002b63d0 WorldTriggerManager_GetTriggerInfo ------------------

    #[test]
    fn world_trigger_manager_get_trigger_info_matches() {
        check(
            ssx3_ee::s08::f_002b63d0__generated,
            world_trigger_manager_get_trigger_info,
            120,
            &|c, seed| {
                let mut r = Rng(0x2b63d0 ^ (seed as u64) << 9);
                c.g[GP][0] = GAME_GP as u64;
                c.g[SP][0] = crate::difftest::SP as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                let self_ = DATA + 0x1000;
                let entries = DATA + 0x2000;
                let count = 4u32;
                w(c, self_ + 0x340, count);
                w(c, self_ + 0x33c, entries);
                c.g[A0][0] = sx(self_);
                let idx = (seed % 6) as u64;
                let ty = [1u32, 2, 3, 0, 3, 2][(seed % 6) as usize];
                c.g[A1][0] = sx(idx as u32);
                let typ = DATA + 0x3000;
                let av = DATA + 0x3100;
                let bv = DATA + 0x3200;
                c.g[A2][0] = sx(typ);
                c.g[A3][0] = sx(av);
                c.g[T0][0] = sx(bv);
                c.g[T1][0] = sx(r.next_u32());
                c.g[T2][0] = 1; // skip the special-trigger delegate
                for i in 0..8u32 {
                    w(c, entries + 8 * i, ty);
                    let obj = DATA + 0x4000 + 0x100 * i;
                    w(c, entries + 8 * i + 4, obj);
                    w(c, obj + 0x10, r.next_u32());
                    w(c, obj + 0x14, r.next_u32());
                    w(c, obj + 0x18, r.next_u32());
                    w(c, obj + 0x1c, r.next_u32());
                    w(c, obj + 0x20, r.next_u32());
                    w(c, obj + 0x24, r.next_u32() & 0x7fff);
                    w(c, obj + 0x28, r.next_u32() & 0x7fff);
                    w(c, obj + 0x2c, r.next_u32() & 0x7fff);
                    w(c, obj + 0x34, r.next_u32() & 0x7fff);
                }
                // The audio singleton chain the random threshold needs.
                w(c, GAME_GP + 0x410, DATA + 0x5000);
                w(c, DATA + 0x5000 + 0x118, DATA + 0x5100);
                w(c, DATA + 0x5100, DATA + 0x5200);
                w(c, DATA + 0x5200 + 0x1d8, 0x1234);
            },
        );
    }
}
