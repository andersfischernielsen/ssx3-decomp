//! `src/fe/festatecharequipdetail.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::ps2f::FpuF32;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use ssx3_ee as ee;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use callee::*;
use crate::call::VSlot;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::fe::festatecharequip2::bolt_file_index_manager_get_node;
pub(crate) use crate::game::fe::festateload::glo;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::ai::ai::gp;
pub(crate) use crate::game::ai::ai::GAME_MODULE_GP;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_2;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::sb;
pub(crate) use crate::game::fe::fepopup::ld;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0019_9148, cfestate_charequip_dtor, "cFEStateCharEquip_dtor"),
    (0x0019_9350, cfe_state_char_equip_shutdown, "cFEStateCharEquip_Shutdown"),
    (0x0019_93a0, c_fe_state_char_equip_on_activate, "cFEStateCharEquip_OnActivate"),
    (0x0019_9420, fe_char_equip_on_deactivate, "cFEStateCharEquip_OnDeactivate"),
    (0x0019_9870, cfe_state_char_equip_on_child_event, "cFEStateCharEquip_OnChildEvent"),
    (0x0019_9a38, cfe_state_char_equip_handle_input, "cFEStateCharEquip_HandleInput"),
    (0x0019_a098, c_fe_state_char_equip_get_preview_texture, "cFEStateCharEquip_GetPreviewTexture"),
    (0x0019_a4e8, cfe_state_char_equip_request_preview, "cFEStateCharEquip_RequestPreview"),
    (0x0019_afd0, cfe_state_char_equip_category_has_flagged_item, "cFEStateCharEquip_CategoryHasFlaggedItem"),
    (0x0019_b098, c_fe_state_char_equip_category_all_flagged, "cFEStateCharEquip_CategoryAllFlagged"),
    (0x0019_bfe8, cfe_state_char_equip_update_preview_camera, "cFEStateCharEquip_UpdatePreviewCamera"),
    (0x0019_c880, bolt_file_index_manager_alloc_node, "BoltFileIndexManager_AllocNode"),
    (0x0019_c8d0, bolt_file_index_manager_get_prev_node, "BoltFileIndexManager_GetPrevNode"),
    (0x0019_c920, bolt_file_index_manager_node_index, "BoltFileIndexManager_NodeIndex"),
    (0x0019_ca70, bolt_file_index_manager_free_node, "BoltFileIndexManager_FreeNode"),
    (0x0019_cb38, bolt_file_index_manager_ctor, "BoltFileIndexManager_ctor"),
    (0x0019_cbe0, bolt_file_index_manager_get_model_slot, "BoltFileIndexManager_GetModelSlot"),
    (0x0019_cc30, bolt_file_index_manager_set_model_slot, "BoltFileIndexManager_SetModelSlot"),
    (0x0019_cce8, bolt_file_index_manager_get_texture_slot, "BoltFileIndexManager_GetTextureSlot"),
    (0x0019_cd38, bolt_file_index_manager_set_texture_slot, "BoltFileIndexManager_SetTextureSlot"),
    (0x0019_cdf0, bolt_file_cache_ctor, "BoltFile_CacheCtor"),
    (0x0019_ce20, bolt_file_cache_dtor, "BoltFile_CacheDtor"),
    (0x0019_d3b0, bolt_file_find_free_slot, "BoltFile_FindFreeSlot"),
    (0x0019_d428, bolt_file_find_slot_by_name, "BoltFile_FindSlotByName"),
    (0x0019_e2b0, bolt_file_get_data, "BoltFile_GetData"),
    (0x0019_e330, bolt_file_detach_data, "BoltFile_DetachData"),
    (0x0019_e498, fe_module_rider_preview_dtor, "FEModule_RiderPreviewDtor"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR as CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS as MPEG2_GET_BITS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT as MPEG2_MARKER_BIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET as BANK_MONITOR_INSTANCE_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
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

    pub const BOLT_FILE_INDEX_MANAGER_FIND_NODE: Callee = Callee {
        addr: 0x0019_c938,
        f: ee::s03::f_0019c938,
    };
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
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

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
}


/// `void* BoltFile_GetData(BoltFile *file, int outer, int inner)` (0x19e2b0):
/// 0 unless the state word at `file + outer*0x12000 + inner*0x120` is 1, else
/// the data pointer at `+0x104`.
pub fn bolt_file_get_data(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000;
    c.mult(0, A2, A2, V1);
    c.mult(0, A1, A1, V0);
    c.g[A3][0] = c.g[A0][0];
    c.g[A0][0] = 0x1;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let not_loaded = c.g[V1][0] != c.g[A0][0];
    c.g[V1][0] = c.g[V0][0];
    if not_loaded {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x104).lw(c);
    }
    abi::ret(c);
}


/// `void BoltFile_DetachData(BoltFile *file, int outer, int inner)`
/// (0x19e330): clears the state word and the data pointer of a loaded record,
/// and decrements the global live-data counter at `file + 0xb4000`.
pub fn bolt_file_detach_data(c: &mut Cpu) {
    c.cycles += 20;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000;
    c.mult(0, A2, A2, V1);
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x104));
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
    c.g[V0][0] = 0xb0000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4000).lw(c);
    c.g[A1][0] = Addr(c.g[A3][0] as u32).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x4000).sw(c, c.g[V1][0]);
    Addr(c.g[A3][0] as u32).sw(c, 0);
    abi::ret(c);
}


/// `Node* BoltFileIndexManager_GetPrevNode(Node *base, Node *node)` (0x19c8d0).
pub fn bolt_file_index_manager_get_prev_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node(c, 0xc);
}


/// `int BoltFileIndexManager_NodeIndex(Node *base, Node *node)` (0x19c920):
/// `(node - base) >> 4`, or -1 for a null node.
pub fn bolt_file_index_manager_node_index(c: &mut Cpu) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(abi::arg(c, 0)));
    if node != 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
    }
}


// ---------------------------------------------------------------------------
// Other constructors that clear their object.

/// `void* BoltFileIndexManager_ctor(void* self)` (0x19cb38): reset the two
/// arrays and return `self`.
pub fn bolt_file_index_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BOLT_FILE_INDEX_MANAGER_RESET.call(c, 0x0019_cb4c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Bolt file cache and rider previews.

/// `BoltFile_Cache* BoltFile_CacheCtor(BoltFile_Cache* self)` (0x19cdf0):
/// publish the cache singleton (`gp-0x1784`), initialise it and return it.
pub fn bolt_file_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let self_ = c.g[S0][0];
        Addr(glo(c, g::BOLT_FILE_CACHE)).sw(c, self_);
        callee::BOLT_FILE_CACHE_INIT.call(c, 0x0019_ce08)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Front-end screens.

/// `void cFEStateCharEquip_OnDeactivate(cFEStateCharEquip* self)`
/// (0x199420): hide the rider preview of the character selected in `+0x44`.
pub fn fe_char_equip_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = world;
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::GET_RIDER_PREVIEW.call(c, 0x0019_943c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_9448)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void BoltFile_CacheDtor(BoltFileCache* c, int flags)` (0x19ce20): clear the
/// global bolt-file cache slot, shut the cache down and, when `flags & 1`,
/// delete the object.
pub fn bolt_file_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.sw(gp(c, -0x1784), 0);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::BOLT_FILE_CACHE_SHUTDOWN.call(c, 0x0019_ce40)?;
        c.g[S0][0] &= 1;
        let keep = c.g[S0][0] == 0;
        // The original reloads s0 before the delete branch (see `dtor`).
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::DELETE.call(c, 0x0019_ce54)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The shared body of `BoltFileIndexManager_GetModelSlot` /
/// `GetTextureSlot`: find the index node for `(slot << 16) | index`, test bit
/// `slot` of the node's availability mask at `mask_off`, and return the value
/// at `value_off` (or -1).
pub(crate) fn bolt_slot_lookup(c: &mut Cpu, shift: u32, mask_off: u32, value_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        BOLT_FILE_INDEX_MANAGER_FIND_NODE.call(c, ret)?;
        c.g[V1][0] = c.g[V0][0];
        let node = c.g[V1][0] as u32;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if node != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(node).off(mask_off).lh(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> (shift & 31)) as u32);
            c.g[V0][0] &= 1;
            let available = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if available {
                c.cycles += 1;
                c.g[V0][0] = Addr(node).off(value_off).lh(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int BoltFileIndexManager_GetModelSlot(this, slot, index)` (0x19cbe0).
pub fn bolt_file_index_manager_get_model_slot(c: &mut Cpu) {
    let shift = c.g[A1][0] as u32;
    bolt_slot_lookup(c, shift, 6, 0xa, 0x0019_cbf8)
}


/// `int BoltFileIndexManager_GetTextureSlot(this, slot, index)` (0x19cce8).
pub fn bolt_file_index_manager_get_texture_slot(c: &mut Cpu) {
    let shift = c.g[A1][0] as u32;
    bolt_slot_lookup(c, shift, 4, 8, 0x0019_cd00)
}


/// `a0 = a0 + off` (the `addiu` the generated code uses for arguments).
#[inline(always)]
pub(crate) fn add_arg(c: &mut Cpu, reg: usize, off: u32) {
    c.g[reg][0] = sx(r32(c, reg).wrapping_add(off));
}


/// `void cFEStateCharEquip_Shutdown(cFEStateCharEquipDetail* s)` (0x199350):
/// invalidate the rider preview's card (-1 at +0xc2c) when a module manager is
/// present, then run the base shutdown. 8 (+6) + 6 cycles plus the callees.
pub fn cfe_state_char_equip_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let module = c.lw(gp_2(c, GAME_MODULE_GP));
        c.g[V0][0] = module;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let manager = c.lw(p(module as u32, 0x7c));
        c.g[A0][0] = manager;
        if manager != 0 {
            c.cycles += 6;
            c.g[A1][0] = Addr(p(self_, 0x44)).lb(c);
            add_arg(c, A0, 0xb0);
            callee::FEMODULE_GET_RIDER_PREVIEW.call(c, 0x0019_937c)?;
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            c.sw(p(r32(c, V0), 0xc2c), c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::UI_OBJECT_SET_VISIBLE, 0x0019_9390)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BoltFileIndexNode* BoltFileIndexManager_AllocNode(BoltFileIndexManager* m)`
/// (0x19c880): pop the free list at +0x1f40 and mark both the popped node's
/// and the new head's link words -1; return the popped node. 14 + 6 cycles
/// plus the callee.
pub fn bolt_file_index_manager_alloc_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        let node = c.lw(p(self_, 0x1f40));
        c.g[S1][0] = node;
        c.g[A1][0] = node;
        callee::BOLT_FILE_INDEX_GET_NEXT_NODE.call(c, 0x0019_c8a0)?;
        c.sw(p(self_, 0x1f40), c.g[V0][0]);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(node as u32, 0xe)).sh(c, c.g[V1][0]);
        c.g[V0][0] = c.lw(p(self_, 0x1f40));
        if c.g[V0][0] != 0 {
            Addr(p(r32(c, V0), 0xc)).sh(c, c.g[V1][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BoltFile_FindFreeSlot` (0x19D3B0): find a free bolt slot in `bank`'s 256
/// entries, preferring an unused-but-valid slot with the smallest stamp;
/// returns -1 when none.
pub fn bolt_file_find_free_slot(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x1_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] |= 0x2000;
    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V1, A1, V0);
    c.g[A1][0] = 0;
    c.g[A0][0] = low(c, V1).wrapping_add(low(c, A0));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        if c.g[V0][0] != 0 {
            c.g[A1][0] = low(c, A1).wrapping_add(1);
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw_at(c, A0, 0x104);
            let free = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[A1][0];
            if free {
                c.cycles += 2;
                c.g[V0][0] = c.g[A1][0];
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let no_best = (c.g[A2][0] as i64) < 0;
            c.g[V1][0] = lw_at(c, A0, 0x11c);
            let mut take_best = no_best;
            if !no_best {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = low(c, A1).wrapping_add(1);
                    c.cycles += 3;
                    c.g[A0][0] = low(c, A0).wrapping_add(0x120);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019_d3d0);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0];
                    abi::ret(c);
                    return;
                }
                take_best = true;
            }
            if take_best {
                c.cycles += 3;
                c.g[A3][0] = c.g[V1][0];
                c.g[A2][0] = c.g[A1][0];
                c.g[A1][0] = low(c, A1).wrapping_add(1);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 256) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = low(c, A0).wrapping_add(0x120);
        if !more {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0019_d3d0);
        }
    }
}


/// `void cFEStateCharEquip_OnActivate(cFEStateCharEquip* s, void* arg)`
/// (0x001993a0): show buy mode when the equip flag is set (else equip mode),
/// then refresh the stat bars and the item list and activate the base state.
pub fn c_fe_state_char_equip_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let equip = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if equip {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0019_a238, 0x0019_93dc)?; // cFEStateCharEquip_ShowEquipMode
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0019_a308, 0x0019_93cc)?; // cFEStateCharEquip_ShowBuyMode
        }
        c.cycles += 17;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019_bc90, 0x0019_93e4)?; // cFEStateCharEquip_UpdateStatBar
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0019_bd48, 0x0019_93f0)?; // cFEStateCharEquip_UpdateStatPreviewBar
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 1;
        c.g[A1][0] = 1;
        call_ptr(c, 0x0019_b618, 0x0019_9400)?; // cFEStateCharEquip_RefreshItemList
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0018_6518, 0x0019_940c)?; // cFEState_OnActivate
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}


/// `int BoltFile_FindSlotByName(EquipTable* self, int bank, const char* name)`
/// (0x19d428): index of the named slot in the bank's 256-entry table, or -1.
pub fn bolt_file_find_slot_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let bank = abi::arg64(c, 1);
        let name = abi::arg64(c, 2);
        c.g[V0][0] = 0x1_0000;
        c.g[V0][0] = c.g[V0][0] | 0x2000;
        frame.save(c, S0, 0x40);
        c.mult(0, 5, 5, 2);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = name;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(4));
        c.g[S1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lb(at(c, S1, 4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            let mut found = false;
            if empty {
                c.cycles += 1;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            } else {
                c.cycles += 4;
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, callee::STRCMP, 0x0019_d474)?;
                if c.g[V0][0] != 0 {
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    found = true;
                }
            }
            if found {
                break;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x120));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 256) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0x120));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_d460);
            }
        }
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


/// `void FEModule_RiderPreviewDtor(this, flags)` (0x19e498): unload the rider
/// preview, then release every occupied preview slot (0x100 of them).
pub fn fe_module_rider_preview_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = sx(this.wrapping_add(0x1c));
        frame.save_ra(c, 0);
        c.g[S4][0] = u64::MAX;
        callee::FEMODULE_RIDER_PREVIEW_UNLOAD.call(c, 0x0019_e4c8)?;
        c.g[S1][0] = 0xff;
        'slots: loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let free = (c.g[A1][0] as i64) < 0;
            c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
            if !free {
                c.cycles += 5;
                c.g[V0][0] = lw_2(c, A2, 0x10d8);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
                c.g[V1][0] = lw_2(c, V0, 0x194);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_e4f0)?;
            }
            c.cycles += 5;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S4][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            Addr(c.g[S0][0] as u32).off(4).sb(c, 0);
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0019_e4d0);
                }
                continue 'slots;
            }
            break 'slots;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x50);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0019_e518)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The shared body of [`bolt_file_index_manager_set_model_slot`] and
/// [`bolt_file_index_manager_set_texture_slot`]: set (`value >= 0`) or clear
/// a per-node slot bit, dropping the node when it has no more references.
pub(crate) fn bolt_set_slot(c: &mut Cpu, entry: u32, flags_off: u32, value_off: u32, ref_off: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    let manager = c.g[A0][0];
    c.g[S2][0] = manager;
    let bit = c.g[A1][0];
    c.g[S0][0] = bit;
    c.g[S1][0] = c.g[A3][0];
    c.g[A3][0] = !(0u64 | c.g[S1][0]);
    frame.save_ra(c, 0);
    c.g[A3][0] = (0u64 < c.g[A3][0]) as u64;
    call_ptr(c, 0x0019_c938, entry + 0x2c)?;
    c.g[A1][0] = c.g[V0][0];
    let mut skip_one = c.g[A1][0] == 0;
    if !skip_one {
        c.cycles += 2;
        let store = (c.g[S1][0] as i64) >= 0;
        c.g[V1][0] = Addr(p(c.g[A1][0] as u32, flags_off)).lhu(c);
        if !store {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[A1][0] as u32, flags_off)).lh(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> ((c.g[S0][0] as u32) & 31)) as u32);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            let clear = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1;
            if !clear {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
                c.g[V0][0] = !(0u64 | c.g[V0][0]);
                c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
                let other = c.g[V1][0] != 0;
                c.sh(p(c.g[A1][0] as u32, flags_off), c.g[V0][0]);
                if !other {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[A1][0] as u32, ref_off)).lh(c);
                    let referenced = (c.g[V0][0] as i64) >= 0;
                    if !referenced {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0019_ca70, entry + 0x24)?;
                        skip_one = true;
                    }
                }
            }
        } else {
            c.cycles += 5;
            c.g[V0][0] = 0x1;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
            c.sh(p(c.g[A1][0] as u32, value_off), c.g[S1][0]);
            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
            c.sh(p(c.g[A1][0] as u32, flags_off), c.g[V1][0]);
        }
    }
    if !skip_one {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x30);
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `void BoltFileIndexManager_SetModelSlot(self, int bit, int a2, int value)`
/// (0x19cc30): set or clear a model-slot bit on the node for (bit, a2).
pub fn bolt_file_index_manager_set_model_slot(c: &mut Cpu) {
    guest(c, |c| bolt_set_slot(c, 0x0019_cc30, 6, 0xa, 8))
}


/// `void BoltFileIndexManager_SetTextureSlot(self, int bit, int a2, int value)`
/// (0x19cd38): the same for texture slots.
pub fn bolt_file_index_manager_set_texture_slot(c: &mut Cpu) {
    guest(c, |c| bolt_set_slot(c, 0x0019_cd38, 4, 8, 0xa))
}


/// `cFEStateCharEquip_OnChildEvent(this, widget, msg)` (0x199870): keep the
/// equip screen's dirty flag in step with the selected item and buy it.
pub fn cfe_state_char_equip_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x15;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_15 = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = abi::arg64(c, 0);

        if is_15 {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::r32(c, A1)).off(0xc).lw(c);
            c.g[V0][0] = 7;
            let selected = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = sx(0xffff_fff7_u32);
            if !selected {
                c.cycles += 4;
                c.g[V0][0] = Addr(abi::r32(c, S0)).off(0x1c).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                Addr(abi::r32(c, S0)).off(0x1c).sw(c, c.g[V0][0]);
            }
            // 'b_199924 tail
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = 0x16;
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 2;
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // msg 0x16
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        c.cycles += 4;
        c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] | 8;
            Addr(c.g[S0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.g[V1][0] = Addr(abi::r32(c, A1)).off(0xc).lw(c);

        c.cycles += 3;
        c.g[V0][0] = 7;
        if c.g[V1][0] != c.g[V0][0] {
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(abi::r32(c, A1)).off(0x6c).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM.call(c, 0x0019_98f8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT.call(c, 0x0019_9900)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x5c).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x95).lbu(c);
        callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM.call(c, 0x0019_9910)?;
        let removed = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !removed {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST.call(c, 0x0019_9924)?;
        }

        c.cycles += 1;
        c.lq(16, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Character-equip item flags and bolt index nodes.

/// `int cFEStateCharEquip_CategoryHasFlaggedItem(self, categoryList, db,
/// int unused)` (0x19afd0): collect the category's items and report whether
/// any enabled one is flagged in the database.
pub fn cfe_state_char_equip_category_has_flagged_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x860);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x850);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0x840);
        c.g[S0][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[T0][0] = u64::MAX;
        c.g[A1][0] = Addr(p_2(c.g[V0][0], 0xbc)).lw(c);
        c.g[A3][0] = c.g[SP][0];
        c.g[T1][0] = 0x1;
        call_ptr(c, 0x0014_d7e8, 0x0019_b000)?;
        c.g[A3][0] = c.g[V0][0];
        let none = (c.g[A3][0] as i64) <= 0;
        c.g[A2][0] = 0;
        let mut found = false;
        if !none {
            c.cycles += 1;
            c.g[A1][0] = c.g[SP][0];
            loop {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V0][0] = Addr(p_2(c.g[A0][0], 0x34)).lw(c) & 0x4;
                if c.g[V0][0] == 0 {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(p_2(c.g[A0][0], 0x4)).lh(c);
                    c.g[V1][0] = Addr(p_2(c.g[S0][0], 0x288)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr(c.g[V0][0] as u32).lh(c);
                    let negative = (c.g[V0][0] as i64) < 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if negative {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr(p_2(c.g[V0][0], 0x2)).lhu(c) & 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(p_2(c.g[A0][0], 0x34)).lw(c) & 0x20;
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            found = true;
                            break;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b010);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x850);
        frame.restore_ra(c, 0x840);
        frame.close(c);
        Ok(())
    })
}


/// `void BoltFileIndexManager_FreeNode(BoltFileIndexManager *self, Node *n)`
/// (0x19ca70): unlink the node from the bucket list, clear it and push it on
/// the free list.
pub fn bolt_file_index_manager_free_node(c: &mut Cpu) {
    guest(c, |c| {
        'b_19cb20: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            let t = c.g[S0][0] == 0u64;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            if t {
                break 'b_19cb20;
            } else {
                'b_19caa0: {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V0][0] = 0x190;
                    if c.g[V0][0] == 0u64 {
                        break 'b_19caa0;
                    } else {
                        break 'b_19caa0;
                    }
                }
                'b_19cadc: {
                    'b_19cacc: {
                        c.cycles += 5;
                        c.div(0, 4, 2);
                        c.g[V1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[S2][0] = c.hi[0];
                        let t = (c.g[V1][0] as i64) >= 0;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_19cacc;
                        } else {
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x0019cabc);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0019c8f8, 0x0019cabc)?;
                            c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            let t = true;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x1900), c.g[V0][0]);
                            if t {
                                break 'b_19cadc;
                            } else {
                                break 'b_19cacc;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0019cad4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0019c8d0, 0x0019cad4)?;
                    c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xe));
                    c.sh((c.g[V0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
                    break 'b_19cadc;
                }
                'b_19caf8: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xe));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        break 'b_19caf8;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0019caf0);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0019c8f8, 0x0019caf0)?;
                        c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.sh((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                        break 'b_19caf8;
                    }
                }
                c.cycles += 10;
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A0][0] = c.g[S1][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0xa), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[RA][0] = sx(0x0019cb18);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1f40));
                call_ptr(c, 0x0019c920, 0x0019cb18)?;
                c.sh((c.g[S0][0] as u32).wrapping_add(0xe), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1f40), c.g[S0][0]);
                break 'b_19cb20;
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
    })
}


// ---------------------------------------------------------------------------
// cFEStateCharEquip_CategoryAllFlagged (0x000019b098).

/// `cFEStateCharEquip_CategoryAllFlagged`: report whether every item of a character-equipment category has the given flag.
pub fn c_fe_state_char_equip_category_all_flagged(c: &mut Cpu) {
    guest(c, |c| {
    'b_19b168: {
    'b_19b164: {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x870));
    c.g[T0][0] = 0xffffffffffffffff;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x850));
    c.g[T1][0] = 0x1;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x860));
    c.g[S1][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x840), c.g[RA][0]);
    c.g[A0][0] = c.g[A1][0];
    c.g[S0][0] = c.g[A2][0];
    c.g[A2][0] = c.g[A3][0];
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
    c.g[A3][0] = c.g[SP][0];
    call_ptr(c, 0x0014d7e8, 0x0019b0cc)?;
    c.g[A3][0] = c.g[V0][0];
    let t = (c.g[A3][0] as i64) <= 0;
    c.g[A2][0] = 0u64;
    if t {
    break 'b_19b164;
    } else {
    c.cycles += 2;
    c.g[A1][0] = c.g[SP][0];
    'l_19b0e0: loop {
    'b_19b158: {
    c.cycles += 5;
    c.g[A0][0] = c.lw((c.g[A1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
    c.g[V0][0] = c.g[V0][0] & 0x4;
    if c.g[V0][0] == 0u64 {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    'b_19b120: {
    'b_19b11c: {
    c.cycles += 7;
    c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x288));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V0][0] = c.lh((c.g[V0][0] as u32));
    let t = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if t {
    break 'b_19b11c;
    } else {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
    let t = true;
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    if t {
    break 'b_19b120;
    } else {
    break 'b_19b11c;
    }
    }
    }
    c.cycles += 1;
    c.g[V0][0] = 0u64;
    break 'b_19b120;
    }
    c.cycles += 4;
    c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2));
    c.g[V0][0] = c.g[V0][0] & 0x2;
    if c.g[V0][0] != 0u64 {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    c.cycles += 4;
    c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x2));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xcc));
    if c.g[V1][0] != c.g[V0][0] {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x34));
    c.g[V0][0] = c.g[V0][0] & 0x20;
    if c.g[V0][0] != 0u64 {
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
    break 'b_19b158;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[V0][0] = 0u64;
    if t {
    break 'b_19b168;
    } else {
    break 'b_19b158;
    }
    }
    }
    }
    }
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0019b0e0); } continue 'l_19b0e0;
    } else {
    break 'b_19b164;
    }
    }
    }
    }
    c.cycles += 1;
    c.g[V0][0] = 0x1;
    break 'b_19b168;
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x860));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x850));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x840));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x870));
    c.pc = t; return Ok(());
        Ok(())
    });
}


/// `cFEStateCharEquip_dtor(self, flags)` (0x199148): release the character
/// equipment screen's preview and its four bolt-file bindings, then run the
/// UI state destructor (and free the object when `flags & 1`).
pub fn cfestate_charequip_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let this = abi::arg(c, 0);
        c.g[V0][0] = 0x47_0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ab8));
        frame.save(c, 18, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A2][0] = lw(c, p(c.g[S0][0] as u32, 0xa64));
        let t = (c.g[A2][0] as i64) < 0;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 7;
            c.g[V0][0] = lw(c, gp(c, -0x848));
            c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xbc));
            c.g[A0][0] = lw(c, p(c.g[V0][0] as u32, 0x7c));
            c.g[RA][0] = sx(0x0019_918c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            c.pc = 0x0019_dc20;
            ee::s03::BoltFile_Release(c);
            if c.pc != 0x0019_918c {
                return Err(Unwound);
            }
            c.g[V0][0] = sx(0xffff_ffff);
            c.sw(p(c.g[S0][0] as u32, 0xa64), c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa68));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_91b4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_91b4 {
                return Err(Unwound);
            }
        }
        c.cycles += 7;
        c.g[S1][0] = sx(0xffff_ffff);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0019_91c4);
        c.sw(p(c.g[S0][0] as u32, 0xa68), c.g[S1][0]);
        c.pc = 0x0019_a498;
        ee::s03::cFEStateCharEquip_ReleasePreview(c);
        if c.pc != 0x0019_91c4 {
            return Err(Unwound);
        }
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa70));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_91e4);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_91e4 {
                return Err(Unwound);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0xa74));
        let t = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = lw(c, gp(c, -0x854));
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[A2][0] as u32, 0x10d8));
            c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x190));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x194));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0019_9204);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x0019_9204 {
                return Err(Unwound);
            }
        }
        c.cycles += 14;
        c.g[V0][0] = 0x47_0000;
        c.sw(p(c.g[S0][0] as u32, 0xa74), c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.sw(p(c.g[S0][0] as u32, 0xa70), c.g[S1][0]);
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0019_9224);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, 0x0039_e390, 0x0019_9224)?;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BoltFile_CompareModelSlots(row, id)` (0x19dd10): compare two model
/// slots of a bolt row by name: an empty/disabled slot sorts before a live
/// one; two live slots are compared with `strcasecmp`.
pub fn bolt_file_compare_model_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        let row = abi::arg(c, 0);
        let id = abi::arg(c, 1);
        frame.save_ra(c, 0);
        c.g[V0][0] = 0x0001_2000;
        c.g[T0][0] = 0x120;
        c.g[A3][0] = 3;
        c.g[V1][0] = lw(c, row);
        c.g[A2][0] = c.lhu(id);
        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
        c.mult(0, A0, A0, A2);
        c.g[V1][0] &= 0xffff;
        c.mult(0, V1, V1, T0);
        c.g[V0][0] = lw(c, gp(c, -0x1784));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.mult(0, V1, A2, T0);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = lw(c, c.g[A0][0] as u32);
        let mut check_other = false;
        let mut memcmp = false;
        let mut extra = 0u32; // 1: the `break b_19de08` path charges one more
        if c.g[V1][0] != c.g[A3][0] {
            c.g[A2][0] = lw(c, c.g[A1][0] as u32);
            check_other = true;
        } else {
            c.cycles += 4;
            c.g[A2][0] = lw(c, p(c.g[A0][0] as u32, 0x110));
            c.g[V0][0] = 8;
            if c.g[A2][0] != c.g[V0][0] {
                c.g[A2][0] = lw(c, c.g[A1][0] as u32);
                check_other = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x11c));
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = lw(c, c.g[A1][0] as u32);
                    check_other = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if t {
                        extra = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x110));
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        if t {
                            extra = 1;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x11c));
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            if t {
                                memcmp = true;
                            } else {
                                c.cycles += 2;
                            }
                        }
                    }
                }
            }
        }
        if check_other {
            // sb is not live: 0; otherwise the same tests as sa, then 1.
            c.cycles += 3;
            c.g[V0][0] = 3;
            let t = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if t {
                extra = 1;
            } else {
                c.cycles += 4;
                c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 0x110));
                c.g[V0][0] = 8;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if t {
                    extra = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x11c));
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        extra = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, c.g[A0][0] as u32);
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 1;
                        if t {
                            extra = 1;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x110));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[V0][0] = 1;
                            if t {
                                extra = 1;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = lw(c, p(c.g[A0][0] as u32, 0x11c));
                                let t = c.g[V0][0] != 0;
                                c.g[V0][0] = 1;
                                if t {
                                    extra = 1;
                                } else {
                                    memcmp = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        if memcmp {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
            call_ptr(c, 0x0041_aa88, 0x003e_de00)?; // strcasecmp
        } else {
            c.cycles += extra as u64;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int func_0019A098(void* self, const char* name)` (0x19a098): find the
/// equipment package entry called `name` for the character in `self` (equip
/// table at `world+0x7c + 0x1a70`, slots at `+0xbc`/`+0xa64`) and return its
/// model id. A missing package or a miss returns -1; a hit binds the entry's
/// data through slot 46 or 48 depending on whether an id was already stored
/// at `self+0xa68`.
pub fn c_fe_state_char_equip_get_preview_texture(c: &mut Cpu) {
    let frame = Frame::open(c, 0xa0);
    'b_19a1b0: {
        'b_19a1ac: {
            c.cycles += 14;
            frame.save(c, S3, 0x60);
            frame.save(c, S5, 0x40);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S4, 0x50);
            frame.save(c, S6, 0x30);
            frame.save(c, S7, 0x20);
            frame.save_ra(c, 0x10);
            c.g[V0][0] = lw(c, at(c, S3, 0xa60));
            let t = c.g[V0][0] == 0u64;
            c.g[S5][0] = c.g[A1][0];
            if t {
                break 'b_19a1ac;
            } else {
                c.cycles += 9;
                c.g[V0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                c.g[A1][0] = lw(c, at(c, S3, 0xbc));
                c.g[A0][0] = lw(c, at(c, V0, 0x7c));
                c.g[A2][0] = lw(c, at(c, S3, 0xa64));
                c.g[RA][0] = sx(0x0019a0e8);
                c.g[A0][0] = sx(at(c, A0, 0x1a70));
                if callee::C0019e2b0.call(c, 0x0019a0e8).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V0][0] = 0xffffffffffffffff;
                if t {
                    break 'b_19a1b0;
                } else {
                    c.cycles += 3;
                    c.g[S4][0] = lw(c, at(c, S1, 0x8));
                    let t = (c.g[S4][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        break 'b_19a1b0;
                    } else {
                        c.cycles += 4;
                        c.g[S6][0] = 0xffffffffffffffff;
                        c.g[S7][0] = 0x4a0000;
                        c.g[S2][0] = sx(at(c, S1, 0x14));
                        'l_19a110: loop {
                            'b_19a1a0: {
                                c.cycles += 10;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x0019a120);
                                c.g[A2][0] = c.g[SP][0];
                                if call_ptr(c, 0x003b3d00, 0x0019a120).is_err() {
                                    return;
                                }
                                sb(c, at(c, SP, 0x4), 0u64);
                                c.g[A0][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x0019a130);
                                c.g[A1][0] = c.g[S5][0];
                                if call_ptr(c, 0x004165a8, 0x0019a130).is_err() {
                                    return;
                                }
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx(at(c, S0, 0x1));
                                if t {
                                    break 'b_19a1a0;
                                } else {
                                    'b_19a198: {
                                        'b_19a17c: {
                                            c.cycles += 4;
                                            c.g[V0][0] = lw(c, r32(c, S2));
                                            c.g[A1][0] = lw(c, at(c, S3, 0xa68));
                                            let t = c.g[A1][0] != c.g[S6][0];
                                            c.g[A2][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            if t {
                                                break 'b_19a17c;
                                            } else {
                                                c.cycles += 13;
                                                c.g[T2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                                c.g[A1][0] = c.g[A2][0];
                                                c.g[A2][0] = sx(at(c, S7, 0x1950));
                                                c.g[A3][0] = 0u64;
                                                c.g[V1][0] = lw(c, at(c, T2, 0x10d8));
                                                c.g[T0][0] = 0x1;
                                                c.g[T1][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = lh(c, at(c, V1, 0x170));
                                                c.g[V0][0] = lw(c, at(c, V1, 0x174));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x0019a174);
                                                c.g[A0][0] =
                                                    sx(r32(c, T2).wrapping_add(r32(c, A0)));
                                                if call_ptr(c, t, 0x0019a174).is_err() {
                                                    return;
                                                }
                                                let t = true;
                                                sw_2(c, at(c, S3, 0xa68), c.g[V0][0]);
                                                if t {
                                                    break 'b_19a198;
                                                } else {
                                                    break 'b_19a17c;
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[T0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                        c.g[A3][0] = 0x1;
                                        c.g[V0][0] = lw(c, at(c, T0, 0x10d8));
                                        c.g[A0][0] = lh(c, at(c, V0, 0x180));
                                        c.g[V1][0] = lw(c, at(c, V0, 0x184));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x0019a198);
                                        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                                        if call_ptr(c, t, 0x0019a198).is_err() {
                                            return;
                                        }
                                        break 'b_19a198;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = lw(c, at(c, S3, 0xa68));
                                    if t {
                                        break 'b_19a1b0;
                                    } else {
                                        break 'b_19a1a0;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S2][0] = sx(at(c, S2, 0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0019a110);
                                }
                                continue 'l_19a110;
                            } else {
                                break 'b_19a1ac;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 0xffffffffffffffff;
        break 'b_19a1b0;
    }
    c.cycles += 11;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    c.g[RA][0] = ld(c, at(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0xa0));
    c.pc = t;
    return;
}


// ---------------------------------------------------------------------------
// Rider/item preview: camera, stick input, item preview request (tick rate).

/// Fields of `cFEStateCharEquip` used by the preview.
mod equip_preview {
    /// Mode word: 1 = the equip menu has the input (no preview control).
    pub const MODE: u32 = 0xc8;
    /// Preview zoom (0..1) and pitch (-1..1), driven by the right stick.
    pub const ZOOM: u32 = 0xaa4;
    pub const PITCH: u32 = 0xaa8;
    /// Preview yaw (degrees, 0..360), driven by the left stick.
    pub const YAW: u32 = 0xaac;
    /// Turntable angle of the preview rider (degrees, +1 per 60 Hz tick).
    pub const SPIN: u32 = 0xab0;
    /// The camera's two vectors (position-like and target-like) and its
    /// state (1/2: snap to a view, 3/4: glide to a view, then 2/1).
    pub const CAM_A: u32 = 0xac0;
    pub const CAM_B: u32 = 0xad0;
    pub const CAM_STATE: u32 = 0xae0;
    /// Item preview: load delay (ticks), item list, category.
    pub const PREVIEW_DELAY: u32 = 0x95c;
}

const PCAM_GET_RIDER_PREVIEW: Callee = Callee { addr: 0x001a_0548, f: ee::s03::f_001a0548 };
const PCAM_SET_TRANSFORM: Callee = Callee { addr: 0x0019_f3e8, f: ee::s03::f_0019f3e8 };
const PCAM_SET_TRANSFORM2: Callee = Callee { addr: 0x0019_f548, f: ee::s03::f_0019f548 };
const PREQ_RELEASE_PREVIEW: Callee = Callee { addr: 0x0019_a498, f: ee::s03::f_0019a498 };
const PCAM_MATH_SIN_COS: u32 = 0x0031_be50;
const PCAM_SNAP_CHASE_VIEW: u32 = 0x0015_e050;

/// `base(reg) + off`.
#[inline(always)]
fn pv_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    (c.g[reg][0] as u32).wrapping_add(off)
}

/// `gp - off`.
#[inline(always)]
fn pv_gp(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `lui at, hi; mtc1 at, f`.
#[inline(always)]
fn pv_lif(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(1, f);
}

/// The quaternion product the preview builds its rotations with:
/// `sp+0x40 = sp+0x10 * sp+0x30` (VU0 macro ops of the original).
fn pv_quat_mul(c: &mut Cpu) {
    let a = sp(c);
    c.lqc2(4, a.wrapping_add(0x10));
    c.lqc2(5, a.wrapping_add(0x30));
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, a.wrapping_add(0x40));
}

/// `mfc1 v1, f1` of the glide factor: at another rate the factor for the
/// rate (`1 - (1 - a)^(60/rate)`), so the glide keeps its time constant.
/// `f1` itself stays (it is also the settle threshold).
fn pv_glide_factor_to_v1(c: &mut Cpu) {
    if crate::tick::rate(c) == 60 {
        c.mfc1(3, 1);
    } else {
        let a = FpuF32(c.f[1]);
        let a = crate::game::ai::rider::smoothing_60(c, a);
        c.g[V1][0] = sx(a.0);
    }
}

/// States 3/4: both camera vectors move by `factor` (`.lit4` at `gp-k`) of
/// their gap to the view in `sp+0x20`/`sp+0x30`; returns whether the gap of
/// the vector at `sp+cmp` (before the move) was below the factor (the
/// settle test, `c.lt.s`).
fn pv_glide(c: &mut Cpu, k: u32, cmp: u32) -> bool {
    let s = sp(c);
    let at = |o: u32| s.wrapping_add(o);
    c.lqc2(3, at(0x20));
    c.lqc2(4, pv_at(c, S0, equip_preview::CAM_A));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(0x50));
    c.lwc1(1, pv_gp(c, k));
    c.lq(2, at(0x50));
    c.sq(2, at(0x40));
    c.lqc2(3, at(0x30));
    c.lqc2(4, pv_at(c, S0, equip_preview::CAM_B));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(0x60));
    c.lq(2, at(0x60));
    c.sq(2, at(0x50));
    pv_glide_factor_to_v1(c);
    c.lqc2(4, at(0x40));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(0x70));
    c.lq(2, at(0x70));
    c.sq(2, at(0x60));
    c.lqc2(3, pv_at(c, S0, equip_preview::CAM_A));
    c.lqc2(4, at(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, pv_at(c, S0, equip_preview::CAM_A));
    pv_glide_factor_to_v1(c);
    c.lqc2(4, at(0x50));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(0x70));
    c.lq(2, at(0x70));
    c.sq(2, at(0x60));
    c.lqc2(3, pv_at(c, S0, equip_preview::CAM_B));
    c.lqc2(4, at(0x60));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, pv_at(c, S0, equip_preview::CAM_B));
    c.lqc2(3, at(cmp));
    c.lqc2(5, at(cmp));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.c_lt_s(0, 1);
    c.fcc()
}

/// `void cFEStateCharEquip_UpdatePreviewCamera(cFEStateCharEquip* self)`
/// (0x19bfe8), once per tick on the equip screen: when the rider preview is
/// loaded, move the preview camera by its state (1/2 snap to the zoomed or
/// the overview, 3/4 glide there and settle), turn the rider by the stick
/// yaw, spin its turntable (+1 degree per 60 Hz tick, wrapped at 360), set
/// both preview transforms; then snap the game camera to the default view.
///
/// Tick rate: the spin step is `1 * 60/rate` and the glide factor
/// `1 - 0.8^(60/rate)` (the 0.2 settle threshold stays); exact at 60.
pub fn cfe_state_char_equip_update_preview_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x170);
        c.g[V0][0] = c.lw(pv_gp(c, 0x848));
        frame.save(c, S0, 0x130);
        frame.save(c, S4, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x120);
        frame.save(c, S2, 0x110);
        frame.save(c, S3, 0x100);
        frame.save(c, S5, 0xe0);
        frame.save(c, S6, 0xd0);
        frame.save(c, S7, 0xc0);
        frame.save_ra(c, 0xb0);
        frame.save_fpr(c, 24, 0x160);
        frame.save_fpr(c, 23, 0x158);
        frame.save_fpr(c, 22, 0x150);
        frame.save_fpr(c, 21, 0x148);
        frame.save_fpr(c, 20, 0x140);
        c.g[A0][0] = c.lw(pv_at(c, V0, 0x7c));
        c.g[A1][0] = c.lb(pv_at(c, S0, 0x44));
        c.g[A0][0] = sx(pv_at(c, A0, 0xb0));
        PCAM_GET_RIDER_PREVIEW.call(c, 0x0019_c03c)?;
        // The preview is ready: state < 10, loaded (+0xcb8) and shown (+0xcb4).
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S4][0] as u32);
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        c.g[V1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(pv_at(c, S4, 0xcb8));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(pv_at(c, S4, 0xcb4));
                c.g[V1][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[V1][0] != 0 {
            preview_camera_move(c);
            preview_transforms(c)?;
        }

        // Snap the game camera to its default chase view of the front end.
        c.cycles += 40;
        let s = sp(c);
        c.g[V0][0] = c.lw(pv_gp(c, 0x848));
        c.g[A1][0] = c.g[SP][0];
        pv_lif(c, 3, 0x3f80_0000);
        c.g[A2][0] = sx(s.wrapping_add(0x10));
        c.g[S0][0] = c.lw(pv_at(c, V0, 0x7c));
        pv_lif(c, 1, 0xc2dc_0000);
        pv_lif(c, 2, 0x43c5_0000);
        c.g[A0][0] = sx(pv_at(c, S0, 0x10));
        pv_lif(c, 0, 0xc20c_0000);
        c.swc1(1, s);
        c.swc1(0, s.wrapping_add(0x10));
        c.swc1(2, s.wrapping_add(4));
        c.swc1(3, s.wrapping_add(0x1c));
        c.sw(s.wrapping_add(8), 0);
        c.swc1(3, s.wrapping_add(0xc));
        c.sw(s.wrapping_add(0x14), 0);
        c.sw(s.wrapping_add(0x18), 0);
        call_ptr(c, PCAM_SNAP_CHASE_VIEW, 0x0019_c7a0)?;
        c.lwc1(0, pv_gp(c, 0x5974));
        frame.restore(c, S1, 0x120);
        c.swc1(0, pv_at(c, S0, 0x10));
        frame.restore(c, S0, 0x130);
        frame.restore(c, S2, 0x110);
        frame.restore(c, S3, 0x100);
        frame.restore(c, S4, 0xf0);
        frame.restore(c, S5, 0xe0);
        frame.restore(c, S6, 0xd0);
        frame.restore(c, S7, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.restore_fpr(c, 24, 0x160);
        frame.restore_fpr(c, 23, 0x158);
        frame.restore_fpr(c, 22, 0x150);
        frame.restore_fpr(c, 21, 0x148);
        frame.restore_fpr(c, 20, 0x140);
        frame.close(c);
        Ok(())
    })
}

/// The camera state step of [`cfe_state_char_equip_update_preview_camera`]
/// (0x19c09c..0x19c3a4): ends with `sp+0x10 = 0`.
fn preview_camera_move(c: &mut Cpu) {
    use equip_preview as f;
    let s = sp(c);
    let at = |o: u32| s.wrapping_add(o);
    c.cycles += 12;
    c.lwc1(2, pv_at(c, S0, f::ZOOM));
    c.lwc1(0, pv_at(c, S0, f::PITCH));
    pv_lif(c, 3, 0x4282_0000);
    c.mul_s(0, 0, 2);
    pv_lif(c, 1, 0x43a0_0000);
    c.g[V1][0] = c.lw(pv_at(c, S0, f::CAM_STATE));
    c.mul_s(2, 2, 1);
    c.lwc1(24, pv_at(c, S0, f::YAW));
    c.mul_s(4, 0, 3);

    // Whether the camera reached its view this tick (sp+0x10 cleared with 1
    // more cycle) or is still moving / idle (cleared directly).
    let settled_path: bool;
    if c.g[V1][0] == c.g[A1][0] {
        // State 1: snap to the zoomed view (zoom/pitch place it).
        c.cycles += 20;
        pv_lif(c, 0, 0xc2c8_0000);
        pv_lif(c, 1, 0xc2a0_0000);
        c.add_s(0, 2, 0);
        pv_lif(c, 3, 0x3f80_0000);
        c.add_s(1, 4, 1);
        pv_lif(c, 2, 0xc2d2_0000);
        c.swc1(3, at(0x2c));
        c.swc1(2, at(0x20));
        c.swc1(0, at(0x24));
        c.swc1(1, at(0x28));
        c.lwc1(0, pv_gp(c, 0x59a0));
        c.lwc1(1, pv_gp(c, 0x599c));
        pv_lif(c, 2, 0xc368_0000);
        c.lq(3, at(0x20));
        pv_snap(c);
        settled_path = true;
    } else {
        c.cycles += 3;
        c.g[A0][0] = 2;
        c.g[V0][0] = 3;
        if c.g[V1][0] == c.g[A0][0] {
            // State 2: snap to the overview.
            c.cycles += 18;
            pv_lif(c, 0, 0xc37a_0000);
            c.lwc1(1, pv_gp(c, 0x5998));
            pv_lif(c, 2, 0xc357_0000);
            pv_lif(c, 3, 0x3f80_0000);
            c.swc1(0, at(0x20));
            c.swc1(1, at(0x24));
            c.swc1(2, at(0x28));
            c.swc1(3, at(0x2c));
            pv_lif(c, 0, 0xc2c8_0000);
            pv_lif(c, 1, 0x4234_0000);
            pv_lif(c, 2, 0x4170_0000);
            c.lq(3, at(0x20));
            pv_snap(c);
            settled_path = true;
        } else {
            c.cycles += 2;
            let state3 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 4;
            if state3 {
                // State 3: glide to the overview, then state 2.
                c.cycles += 69;
                pv_lif(c, 6, 0x3f80_0000);
                pv_lif(c, 0, 0xc37a_0000);
                c.lwc1(1, pv_gp(c, 0x5994));
                pv_lif(c, 2, 0xc357_0000);
                pv_lif(c, 3, 0xc2c8_0000);
                pv_lif(c, 4, 0x4234_0000);
                pv_lif(c, 5, 0x4170_0000);
                c.swc1(0, at(0x20));
                c.swc1(1, at(0x24));
                c.swc1(2, at(0x28));
                c.swc1(3, at(0x30));
                c.swc1(4, at(0x34));
                c.swc1(5, at(0x38));
                c.swc1(6, at(0x3c));
                c.swc1(6, at(0x2c));
                if pv_glide(c, 0x5990, 0x50) {
                    c.sw(pv_at(c, S0, f::CAM_STATE), c.g[A0][0]);
                    settled_path = true;
                } else {
                    c.cycles += 2;
                    settled_path = false;
                }
            } else {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] {
                    settled_path = false;
                } else {
                    // State 4: glide to the zoomed view, then state 1.
                    c.cycles += 70;
                    pv_lif(c, 0, 0xc2c8_0000);
                    pv_lif(c, 1, 0xc2a0_0000);
                    c.add_s(0, 2, 0);
                    pv_lif(c, 6, 0x3f80_0000);
                    c.add_s(1, 4, 1);
                    pv_lif(c, 2, 0xc2d2_0000);
                    c.lwc1(3, pv_gp(c, 0x598c));
                    c.lwc1(4, pv_gp(c, 0x5988));
                    pv_lif(c, 5, 0xc368_0000);
                    c.swc1(0, at(0x24));
                    c.swc1(1, at(0x28));
                    c.swc1(2, at(0x20));
                    c.swc1(3, at(0x30));
                    c.swc1(4, at(0x34));
                    c.swc1(5, at(0x38));
                    c.swc1(6, at(0x3c));
                    c.swc1(6, at(0x2c));
                    if pv_glide(c, 0x5984, 0x40) {
                        c.sw(pv_at(c, S0, f::CAM_STATE), c.g[A1][0]);
                    }
                    settled_path = true;
                }
            }
        }
    }
    if settled_path {
        c.cycles += 1;
    }
    c.sw(at(0x10), 0);
}

/// States 1/2: both camera vectors take the view (`v1` = the first, the
/// second from f0..f2 over `sp+0x20`).
fn pv_snap(c: &mut Cpu) {
    let s = sp(c);
    c.cycles += 7;
    c.swc1(0, s.wrapping_add(0x20));
    c.swc1(1, s.wrapping_add(0x24));
    c.swc1(2, s.wrapping_add(0x28));
    c.sq(3, pv_at(c, S0, equip_preview::CAM_A));
    c.lq(2, s.wrapping_add(0x20));
    c.sq(2, pv_at(c, S0, equip_preview::CAM_B));
}

/// The two preview transforms (0x19c3a4..0x19c748): rotations built from
/// sin/cos of the angles (yaw from the stick, the turntable spin), the spin
/// step, both `SetTransform` calls.
fn preview_transforms(c: &mut Cpu) -> Flow {
    use equip_preview as f;
    let s = sp(c);
    let at = |o: u32| s.wrapping_add(o);
    c.cycles += 130;
    c.g[A0][0] = sx(at(0x80));
    pv_lif(c, 21, 0x3f80_0000);
    c.g[A1][0] = sx(at(0x84));
    c.lwc1(20, at(0x10));
    c.g[S7][0] = 0x50_0000;
    c.lq(2, pv_at(c, S0, f::CAM_A));
    c.g[S6][0] = sx(pv_at(c, S7, 0u32.wrapping_sub(0xec0)));
    c.swc1(21, at(0x1c));
    c.mov_s(12, 20);
    c.sq(2, s);
    c.g[S5][0] = 0x50_0000;
    c.sw(at(0x14), 0);
    c.g[S3][0] = sx(pv_at(c, S5, 0u32.wrapping_sub(0xeb0)));
    c.sw(at(0x18), 0);
    c.g[S2][0] = 0x50_0000;
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c3e8)?;
    c.lwc1(23, pv_gp(c, 0x5980));
    c.lwc1(1, pv_at(c, S6, 4));
    c.g[A0][0] = sx(at(0x88));
    c.lwc1(0, at(0x80));
    c.g[A1][0] = sx(at(0x8c));
    c.lwc1(2, pv_at(c, S7, 0u32.wrapping_sub(0xec0)));
    c.mov_s(12, 20);
    c.lwc1(4, pv_at(c, S6, 8));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0x84));
    c.mul_s(0, 0, 4);
    pv_lif(c, 22, 0x3f00_0000);
    c.swc1(3, at(0x3c));
    c.g[S1][0] = sx(pv_at(c, S2, 0u32.wrapping_sub(0xea0)));
    c.swc1(2, at(0x30));
    c.swc1(1, at(0x34));
    c.swc1(0, at(0x38));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c474)?;
    c.lwc1(0, pv_at(c, S3, 4));
    c.g[A0][0] = sx(at(0x90));
    c.lwc1(1, at(0x88));
    c.g[A1][0] = sx(at(0x94));
    c.lwc1(2, pv_at(c, S5, 0u32.wrapping_sub(0xeb0)));
    pv_lif(c, 12, 0x42a0_0000);
    c.mul_s(0, 1, 0);
    c.lwc1(4, pv_at(c, S3, 8));
    c.mul_s(2, 1, 2);
    c.add_s(12, 24, 12);
    c.lwc1(3, at(0x8c));
    c.mul_s(1, 1, 4);
    c.swc1(0, at(0x34));
    c.swc1(3, at(0x3c));
    c.mul_s(12, 12, 23);
    c.swc1(2, at(0x30));
    c.swc1(1, at(0x38));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.mul_s(12, 12, 22);
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c500)?;
    c.lwc1(0, pv_at(c, S1, 4));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(2, at(0x90));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(3, pv_at(c, S2, 0u32.wrapping_sub(0xea0)));
    c.lwc1(4, pv_at(c, S1, 8));
    c.mul_s(0, 2, 0);
    c.mul_s(3, 2, 3);
    c.lwc1(1, at(0x94));
    c.mul_s(2, 2, 4);
    c.swc1(1, at(0x3c));
    c.swc1(0, at(0x34));
    c.swc1(3, at(0x30));
    c.swc1(2, at(0x38));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    PCAM_SET_TRANSFORM.call(c, 0x0019_c578)?;

    // The turntable: +1 degree per 60 Hz tick (f21 = 1.0, also the w of
    // the identity rotation stored below), wrapped once at 360.
    c.lwc1(0, pv_at(c, S0, f::SPIN));
    pv_lif(c, 1, 0x43b4_0000);
    if crate::tick::rate(c) == 60 {
        c.add_s(0, 0, 21);
    } else {
        let step = crate::game::ai::rider::rate_step_60(c, FpuF32(c.f[21]));
        c.f[0] = (FpuF32(c.f[0]) + step).0;
    }
    c.lq(2, pv_at(c, S0, f::CAM_B));
    c.swc1(20, at(0x18));
    c.sq(2, s);
    c.c_le_s(1, 0);
    c.swc1(21, at(0x1c));
    c.swc1(20, at(0x10));
    c.swc1(20, at(0x14));
    let wrap = c.fcc();
    c.swc1(0, pv_at(c, S0, f::SPIN));
    if wrap {
        c.cycles += 2;
        c.sub_s(0, 0, 1);
        c.swc1(0, pv_at(c, S0, f::SPIN));
    }

    c.cycles += 101;
    c.lwc1(12, pv_gp(c, 0x597c));
    c.g[A0][0] = sx(at(0x98));
    c.g[A1][0] = sx(at(0x9c));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c5c4)?;
    c.lwc1(1, pv_at(c, S1, 8));
    c.g[A0][0] = sx(at(0xa0));
    c.lwc1(0, at(0x98));
    c.g[A1][0] = sx(at(0xa4));
    c.lwc1(2, pv_at(c, S2, 0u32.wrapping_sub(0xea0)));
    c.lwc1(4, pv_at(c, S1, 4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0x9c));
    c.mul_s(0, 0, 4);
    c.lwc1(12, pv_gp(c, 0x5978));
    c.swc1(3, at(0x3c));
    c.swc1(1, at(0x38));
    c.swc1(2, at(0x30));
    c.swc1(0, at(0x34));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c640)?;
    c.lwc1(1, pv_at(c, S3, 8));
    c.g[A0][0] = sx(at(0xa8));
    c.lwc1(0, at(0xa0));
    c.g[A1][0] = sx(at(0xac));
    c.lwc1(2, pv_at(c, S5, 0u32.wrapping_sub(0xeb0)));
    c.lwc1(4, pv_at(c, S3, 4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0xa4));
    c.mul_s(0, 0, 4);
    pv_lif(c, 5, 0x42b4_0000);
    c.swc1(3, at(0x3c));
    c.swc1(1, at(0x38));
    c.swc1(2, at(0x30));
    c.swc1(0, at(0x34));
    pv_quat_mul(c);
    c.lwc1(12, pv_at(c, S0, f::SPIN));
    c.lq(2, at(0x40));
    c.add_s(12, 12, 5);
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    c.mul_s(12, 12, 23);
    c.mul_s(12, 12, 22);
    call_ptr(c, PCAM_MATH_SIN_COS, 0x0019_c6d0)?;
    c.lwc1(1, pv_at(c, S6, 8));
    c.g[A0][0] = c.g[S4][0];
    c.lwc1(0, at(0xa8));
    c.g[A1][0] = c.g[SP][0];
    c.lwc1(2, pv_at(c, S7, 0u32.wrapping_sub(0xec0)));
    c.lwc1(4, pv_at(c, S6, 4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, at(0xac));
    c.mul_s(0, 0, 4);
    c.swc1(3, at(0x3c));
    c.swc1(1, at(0x38));
    c.swc1(2, at(0x30));
    c.swc1(0, at(0x34));
    pv_quat_mul(c);
    c.lq(2, at(0x40));
    c.sq(2, at(0x10));
    c.sq(2, at(0x20));
    PCAM_SET_TRANSFORM2.call(c, 0x0019_c748)?;
    Ok(())
}

/// `int cFEStateCharEquip_HandleInput(cFEStateCharEquip* self, Pad* pad)`
/// (0x199a38): unless the equip menu has the input (`+0xc8 == 1`), in the
/// camera states 1/4 the left stick turns the preview (`yaw += x * 3`,
/// wrapped to 0..360) and the right stick zooms (`+0xaa4`, 0..1) and
/// pitches (`+0xaa8`, -1..1) it by `-0.1` per tick. Returns whether
/// anything moved.
///
/// `UIState_Update` calls it every tick while the pad has input held (the
/// state's pad mask `+0x15` is set in its ctor), so it is a per-tick step:
/// the 3.0 becomes `3.0 * 60/rate` (the -0.1 steps are `tick::PER_TICK`
/// `.lit4` entries already). Exact at 60.
pub fn cfe_state_char_equip_handle_input(c: &mut Cpu) {
    use equip_preview as f;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(pv_at(c, S1, f::MODE)) ^ 1;
        c.g[S2][0] = 0;
        'out: {
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
                break 'out;
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(pv_at(c, S1, f::CAM_STATE));
            c.g[V0][0] = 1;
            let one = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 4;
            if !one {
                c.cycles += 2;
                let four = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = c.g[S2][0];
                if !four {
                    break 'out;
                }
            }

            // Left stick: yaw.
            c.cycles += 7;
            c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            pv_pad_call(c, V1, V0, 0x118, 0x0019_9a90)?;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            } else {
                c.cycles += 16;
                c.g[V0][0] = c.lw(pv_at(c, S0, 8));
                pv_pad_call(c, V0, V1, 0x130, 0x0019_9aac)?;
                pv_lif(c, 1, 0x4040_0000);
                crate::game::ai::rider::per_tick_60(c, 1);
                c.lwc1(2, pv_at(c, S1, f::YAW));
                c.mul_s(0, 0, 1);
                pv_lif(c, 3, 0x43b4_0000);
                c.add_s(2, 2, 0);
                c.c_le_s(3, 2);
                let wrap = c.fcc();
                c.swc1(2, pv_at(c, S1, f::YAW));
                if wrap {
                    const HEAD: u32 = 0x0019_9ae0;
                    c.cycles += 2;
                    c.mov_s(0, 3);
                    c.sub_s(2, 2, 0);
                    loop {
                        c.cycles += 4;
                        c.c_le_s(0, 2);
                        if !c.fcc() {
                            break;
                        }
                        c.sub_s(2, 2, 0);
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                    c.cycles += 1;
                    c.swc1(2, pv_at(c, S1, f::YAW));
                }
                c.cycles += 6;
                c.lwc1(1, pv_at(c, S1, f::YAW));
                c.mtc1(0, 2);
                c.c_lt_s(1, 2);
                if c.fcc() {
                    const HEAD: u32 = 0x0019_9b18;
                    c.cycles += 3;
                    pv_lif(c, 3, 0x43b4_0000);
                    loop {
                        c.cycles += 5;
                        c.add_s(0, 1, 3);
                        c.c_lt_s(0, 2);
                        let again = c.fcc();
                        c.mov_s(1, 0);
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    }
                    c.cycles += 2;
                    c.swc1(0, pv_at(c, S1, f::YAW));
                }
                c.g[S2][0] = 1;
                c.cycles += 1;
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            }

            // Right stick up/down: zoom.
            c.cycles += 6;
            pv_pad_call(c, V1, V0, 0x108, 0x0019_9b48)?;
            if c.g[V0][0] == 0 {
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            } else {
                c.cycles += 16;
                c.g[V0][0] = c.lw(pv_at(c, S0, 8));
                pv_pad_call(c, V0, V1, 0x120, 0x0019_9b64)?;
                c.lwc1(1, pv_gp(c, 0x59a8));
                c.lwc1(3, pv_at(c, S1, f::ZOOM));
                c.mul_s(0, 0, 1);
                pv_lif(c, 4, 0x3f80_0000);
                c.mtc1(0, 2);
                c.add_s(3, 3, 0);
                c.c_le_s(2, 3);
                let inside = c.fcc();
                c.mov_s(0, 2);
                if inside {
                    c.cycles += 1;
                    c.min_s(0, 3, 4);
                }
                c.cycles += 3;
                c.swc1(0, pv_at(c, S1, f::ZOOM));
                c.g[S2][0] = 1;
                c.g[V1][0] = c.lw(pv_at(c, S0, 8));
            }

            // Right stick left/right: pitch.
            c.cycles += 6;
            pv_pad_call(c, V1, V0, 0x110, 0x0019_9bb0)?;
            let none = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[S2][0];
            if none {
                break 'out;
            }
            c.cycles += 17;
            c.g[V0][0] = c.lw(pv_at(c, S0, 8));
            pv_pad_call(c, V0, V1, 0x128, 0x0019_9bcc)?;
            c.lwc1(1, pv_gp(c, 0x59a4));
            c.lwc1(3, pv_at(c, S1, f::PITCH));
            c.mul_s(0, 0, 1);
            pv_lif(c, 2, 0xbf80_0000);
            pv_lif(c, 4, 0x3f80_0000);
            c.add_s(3, 3, 0);
            c.c_le_s(2, 3);
            let inside = c.fcc();
            c.mov_s(1, 2);
            if inside {
                c.cycles += 1;
                c.min_s(1, 3, 4);
            }
            c.cycles += 2;
            c.swc1(1, pv_at(c, S1, f::PITCH));
            c.g[S2][0] = 1;
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
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

/// A pad method call `pad->slot()` (pad in `s0`): the vtable already in
/// `vt`, the method into `fnr` (the registers the original uses).
fn pv_pad_call(c: &mut Cpu, vt: usize, fnr: usize, slot: u32, ret: u32) -> Flow {
    let v = c.g[vt][0] as u32;
    c.g[A0][0] = c.lh(v.wrapping_add(slot));
    c.g[fnr][0] = c.lw(v.wrapping_add(slot + 4));
    let target = c.g[fnr][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `void cFEStateCharEquip_RequestPreview(cFEStateCharEquip* self, int
/// delayed)` (0x19a4e8): drop the current item preview (and the screen's
/// preview texture), then, for a selected item with a model name, start its
/// load after a delay of 20 frames when `delayed` (1 tick otherwise), and
/// build the file name `"<bank>_<name>.<digits>"`-style into `+0x960`.
///
/// Tick rate: the 20 frames are `ticks_from_frames60(20)` ticks
/// (`cFEStateCharEquip_UpdatePreview` counts the delay down per tick).
pub fn cfe_state_char_equip_request_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(pv_at(c, S1, 0x5c));
        c.g[S0][0] = c.g[A1][0];
        'tail: {
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                break 'tail;
            }
            c.cycles += 5;
            PREQ_RELEASE_PREVIEW.call(c, 0x0019_a518)?;
            c.g[A1][0] = c.lw(pv_at(c, S1, 0xa74));
            c.g[A2][0] = c.lw(pv_gp(c, 0x854));
            if (c.g[A1][0] as i64) >= 0 {
                // Release the screen's preview texture (a method of gp-0x854).
                c.cycles += 5;
                c.g[V0][0] = c.lw(pv_at(c, A2, 0x10d8));
                c.g[A0][0] = c.lh(pv_at(c, V0, 0x190));
                c.g[V1][0] = c.lw(pv_at(c, V0, 0x194));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx(pv_at(c, A2, c.g[A0][0] as u32));
                call_ptr(c, target, 0x0019_a538)?;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw(pv_at(c, S1, 0xa70));
            c.g[V1][0] = u64::MAX;
            c.sw(pv_at(c, S1, 0xa70), c.g[V1][0]);
            c.sw(pv_at(c, S1, 0xa74), c.g[V0][0]);
            c.g[V1][0] = c.lw(pv_at(c, S1, 0x5c));
            c.g[V1][0] = c.lbu(pv_at(c, V1, 0x95));
            // (`bltz` on the byte just loaded: never taken.)
            c.cycles += 4;
            c.g[V0][0] = c.lw(pv_at(c, S1, 0x958));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let past_end = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
            if past_end {
                c.cycles += 1;
                break 'tail;
            }
            c.cycles += 5;
            c.g[V0][0] = sx(pv_at(c, S1, c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(pv_at(c, V0, 0x124));
            c.g[S2][0] = c.lw(pv_at(c, V1, 0x30));
            if c.g[S2][0] == 0 {
                break 'tail;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = crate::tick::ticks_from_frames60(c, 20) as u64;
            }
            c.cycles += 24;
            c.sw(pv_at(c, S1, equip_preview::PREVIEW_DELAY), c.g[V0][0]);
            call_ptr(c, 0x0014_dc80, 0x0019_a590)?; // BELibrary_GetInstance
            c.g[S0][0] = sx(pv_at(c, S1, 0x960));
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 9;
            call_ptr(c, 0x0014_dd58, 0x0019_a5a0)?; // BELibrary_GetInterface(9)
            c.g[A1][0] = c.lw(pv_at(c, S1, 0xbc));
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_b700, 0x0019_a5ac)?; // BEBoltInt_DispatchCategory
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 0x2e;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_acc0, 0x0019_a5bc)?; // strrchr(path, '.')
            c.g[A1][0] = c.g[S1][0];
            c.g[S3][0] = c.g[V0][0];
            c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_6ce0, 0x0019_a5d8)?; // strncpy
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x4a_0000;
            c.g[S0][0] = sx(pv_at(c, S0, c.g[S1][0] as u32));
            c.sb(c.g[S0][0] as u32, 0);
            // Skip the name to its first digit.
            const HEAD: u32 = 0x0019_a5e8;
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lb(c.g[S2][0] as u32);
                c.g[V1][0] = c.lbu(c.g[S2][0] as u32);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                if c.g[V0][0] != 0 {
                    break;
                }
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
            c.cycles += 5;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1958));
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[A0][0];
            call_ptr(c, 0x0041_7828, 0x0019_a618)?; // sprintf("%s_%s%s")
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
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
    use ssx3_ee::s01;
    use ssx3_ee::s03;
    use ssx3_ee::s00;
    use ssx3_ee::s02;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use crate::difftest::fill;
    use ssx3_ee as ee;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    use crate::difftest::put_words;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::dirtysock::tags2::tests::cstr;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::ai::ai::tests::strict as strict_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_6;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_6;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_7;


    #[test]
    fn bolt_file_get_data_matches() {
        check_lohi(ssx3_ee::s03::f_0019e2b0, bolt_file_get_data, 200, &|c, seed| {
            let mut r = Rng(0x1111_0002 ^ seed as u64);
            scramble(c, &mut r);
            let outer = (seed % 2) as u32;
            let inner = ((seed / 2) % 4) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = outer as u64;
            c.g[A2][0] = inner as u64;
            let base = DATA
                .wrapping_add(outer.wrapping_mul(0x12000))
                .wrapping_add(inner.wrapping_mul(0x120));
            c.bus.write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
            c.bus.write32(base + 0x104, r.next_u32());
        });
    }


    #[test]
    fn bolt_file_detach_data_matches() {
        check_lohi(ssx3_ee::s03::f_0019e330, bolt_file_detach_data, 200, &|c, seed| {
            let mut r = Rng(0x1111_0003 ^ seed as u64);
            scramble(c, &mut r);
            let outer = (seed % 2) as u32;
            let inner = ((seed / 2) % 2) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = outer as u64;
            c.g[A2][0] = inner as u64;
            let d = outer.wrapping_mul(0x12000).wrapping_add(inner.wrapping_mul(0x120));
            let v1 = DATA.wrapping_add(0x104).wrapping_add(d);
            let a3 = DATA.wrapping_add(0x118).wrapping_add(d);
            let counter = DATA.wrapping_add(0xb4000);
            c.bus.write32(v1, r.next_u32());
            c.bus.write32(a3, r.next_u32());
            c.bus.write32(counter, r.next_u32());
        });
    }


    #[test]
    fn bolt_file_index_manager_get_prev_node_matches() {
        check(ssx3_ee::s03::f_0019c8d0, bolt_file_index_manager_get_prev_node, 200, &|c, seed| {
            let mut r = Rng(0x19c8 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.g[A1][0] = node as u64;
            if node != 0 {
                let val = if seed % 2 == 0 { r.next_u32() & 0x7fff } else { 0x8000 | (r.next_u32() & 0x7fff) };
                c.bus.write16(node + 0xc, val as u16);
            }
        });
    }


    #[test]
    fn bolt_file_index_manager_node_index_matches() {
        check(ssx3_ee::s03::f_0019c920, bolt_file_index_manager_node_index, 200, &|c, seed| {
            let mut r = Rng(0x19ca ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = if seed % 2 == 0 { 0 } else { (DATA + (r.next_u32() % 0x100) * 16) as u64 };
        });
    }


    #[test]
    fn bolt_file_index_manager_ctor_matches() {
        check(ssx3_ee::s03::f_0019cb38__generated, bolt_file_index_manager_ctor, 40, &|c, seed| {
            let mut r = rng(seed, 0x19);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x2000);
            for i in 0..4u32 {
                c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
            }
        });
    }


    // -----------------------------------------------------------------
    // Bolt file cache and rider previews.

    #[test]
    fn bolt_file_cache_ctor_matches() {
        check(s03::f_0019cdf0__generated, bolt_file_cache_ctor, 100, &|c, seed| {
            machine(c, seed, 0x19cdf0, 1);
            globals(c);
            clear(c, OBJ, 0x100);
            c.g[A0][0] = sx(OBJ);
            // An existing cache skips the allocation in `BoltFile_CacheInit`;
            // alternate so both paths run (the null path allocates from the
            // zeroed heap, which returns nothing).
            c.bus.write32(GAME_GP.wrapping_sub(0x1788), if seed % 3 == 0 { 0 } else { OBJ });
        });
    }


    /// A dummy world: `world -> +0x84 -> race manager -> +0x84 -> riders`
    /// (the rider count at the riders pointer is left 0, so rider loops stop).
    pub(crate) fn world(c: &mut Cpu) {
        let w = DATA + 0x4000;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), w);
        c.bus.write32(w + 0x7c, DATA + 0x5000);
        c.bus.write32(w + 0x84, DATA + 0x6000);
        c.bus.write32(DATA + 0x6000 + 0x84, DATA + 0x7000);
        // No cut-scene transition to force off.
        c.bus.write32(DATA + 0x6000 + 0x64, 0);
    }


    // ------------------------------------------------------------------
    // FE screens.

    #[test]
    fn fe_char_equip_on_deactivate_matches() {
        check(ssx3_ee::s03::f_00199420__generated, fe_char_equip_on_deactivate, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0x99420, 0);
            world(c);
            let self_ = DATA + 0x300;
            c.bus.write8(self_ + 0x44, (seed % 4) as u8);
            c.g[A0][0] = sx(self_);
        });
    }


    #[test]
    fn bolt_file_cache_dtor_matches() {
        // The cache teardown frees 10 * 256 slots for real, so keep the seed
        // count small: the body is a straight loop, not data dependent.
        check(ee::s03::f_0019ce20__generated, bolt_file_cache_dtor, 8, &|c, seed| {
            let mut r = machine(c, seed, 0x19_ce20, 0);
            let obj = DATA + 0x1000;
            c.bus.write32(obj, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 4 == 0) as u64;
        });
    }


    // -----------------------------------------------------------------------
    // 0x19cbe0 / 0x19cce8 BoltFileIndexManager slot lookups.

    pub(crate) fn bolt_slot_setup(c: &mut Cpu, seed: u32, salt: u64, mask_off: u32, value_off: u32) {
        let mut r = machine(c, seed, salt, 1);
        let index = DATA + 0x100;
        let slot = seed % 8;
        c.g[A0][0] = sx(index);
        c.g[A1][0] = slot as u64;
        c.g[A2][0] = 0;
        c.bus.write32(index + 0x1f40, 0); // no "always create" flag
        let key = slot << 16; // (slot << 16) | 0
        let bucket = key % 400;
        let node = DATA + 0x300;
        c.bus.write32(index + 0x1900 + bucket * 4, node);
        c.bus.write32(node, key);
        let mask = if seed % 3 == 0 {
            0
        } else {
            1u32 << (slot & 31)
        };
        c.bus.write16(node + mask_off, mask as u16);
        c.bus.write16(node + value_off, r.next_u32() as u16);
    }


    #[test]
    fn bolt_file_index_manager_get_model_slot_matches() {
        check(ee::s03::f_0019cbe0__generated, bolt_file_index_manager_get_model_slot, 24, &|c, seed| {
            bolt_slot_setup(c, seed, 0x19cb_e0, 6, 0xa)
        });
    }


    #[test]
    fn bolt_file_index_manager_get_texture_slot_matches() {
        check(ee::s03::f_0019cce8__generated, bolt_file_index_manager_get_texture_slot, 24, &|c, seed| {
            bolt_slot_setup(c, seed, 0x19cc_e8, 4, 8)
        });
    }


    #[test]
    fn cfe_state_char_equip_shutdown_matches() {
        check(ee::s03::f_00199350__generated, cfe_state_char_equip_shutdown, 60, &|c, seed| {
            let _r = base(c, seed, 0x1993);
            let module = DATA + 0x1000;
            let manager = DATA + 0x2000;
            c.bus.write32(gp_at(-0x848), if seed % 3 == 0 { 0 } else { module });
            c.bus.write32(module + 0x7c, manager);
            let preview = DATA + 0x3000;
            c.bus.write32(manager + 0xb0, preview);
            c.bus.write8(DATA + 0x44, (seed % 4) as u8);
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn bolt_file_index_manager_alloc_node_matches() {
        check(ee::s03::f_0019c880__generated, bolt_file_index_manager_alloc_node, 60, &|c, seed| {
            base(c, seed, 0x19c8);
            let m = DATA;
            let n = seed % 3;
            let head = if n == 0 { 0 } else { DATA + 0x1000 };
            c.bus.write32(m + 0x1f40, head);
            if n != 0 {
                // The next-node link at +0xe is either -1 (empty) or 0 (the
                // manager's own node, so a new head exists).
                c.bus.write16(head + 0xe, if seed % 2 == 0 { 0xbeef } else { 0 });
                c.bus.write16(head + 0xc, 0xdead);
                fill(c, &mut Rng(0x19c8_0000 ^ seed as u64), head, 3);
            }
            c.g[A0][0] = sx(m);
        });
    }


    #[test]
    fn bolt_file_find_free_slot_matches() {
        check(ee::s03::f_0019d3b0__generated, bolt_file_find_free_slot, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x19_d3b0, 0);
            c.g[A0][0] = DATA as u64;
            let bank = (seed % 2) as u32;
            c.g[A1][0] = bank as u64;
            let base = DATA + 0x1_2000 * bank;
            let free_at = if seed % 3 == 0 { 256 } else { 200 + seed % 40 };
            for i in 0..256u32 {
                let s = base + 0x120 * i;
                let empty = i >= free_at;
                let valid = if empty { 0 } else { 1 };
                let used = if empty { 0 } else if i % 4 == 0 { 0 } else { 1 };
                w(c, s, used);
                w(c, s + 0x104, valid);
                w(c, s + 0x11c, (i * 7) % 64);
            }
            c.pc = 0x0019_d3b0;
        });
    }


    #[test]
    fn c_fe_state_char_equip_on_activate_matches() {
        check(
            ee::s03::f_001993a0__generated,
            c_fe_state_char_equip_on_activate,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1993, 3);
                let s = DATA;
                w(c, s + 0xc8, (seed % 3) as u32);
                fill(c, &mut r, s + 0x20, 0x40);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }


    #[test]
    fn bolt_file_find_slot_by_name_matches() {
        check(
            ee::s03::f_0019d428__generated,
            bolt_file_find_slot_by_name,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x19d4, 0);
                env(c);
                let this = DATA + 0x400;
                let name = DATA + 0x800;
                cstr(c, name, "BOLT_B");
                // bank 1 selects the row at `this + 0x12000`.
                let row = this + 0x12000;
                match seed % 3 {
                    0 => {}
                    1 => cstr(c, row + 4, "BOLT_A"),
                    _ => {
                        cstr(c, row + 4, "BOLT_A");
                        cstr(c, row + 0x120 + 4, "BOLT_B");
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 1;
                c.g[A2][0] = sx(name);
            },
        );
    }


    #[test]
    fn fe_module_rider_preview_dtor_matches() {
        check(ssx3_ee::s03::f_0019e498__generated, fe_module_rider_preview_dtor, 8, &|c, seed| {
            let _r = base_3(c, seed, 0x19e498, 2);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            w_2(c, this + 8, 0);
            w_2(c, this + 0xc, 0);
            w_2(c, GAME_GP.wrapping_sub(0x854), DATA + 0x2000);
            w_2(c, DATA + 0x2000 + 0x10d8, DATA + 0x2100);
            c.bus.write16(DATA + 0x2100 + 0x190, 0);
            w_2(c, DATA + 0x2100 + 0x194, 0);
            for i in 0..256u32 {
                let slot = this + 0x1c + 0xc * i;
                let used = seed % 3 != 0 && i % 7 != 0;
                w_2(c, slot, if used { (i % 16) as u32 } else { 0xffff_ffff });
                c.bus.write8(slot + 4, 0xff);
            }
        });
    }


    /// A node for the (bit, a2) key in the index manager's hash bucket.
    pub(crate) fn bolt_node(c: &mut Cpu, seed: u32, flags_off: u32, value_off: u32, ref_off: u32) {
        let _ = base_4(c, seed, 0x0019_cc30, 3);
        let bit = (seed % 32) as u32;
        let a2 = (seed % 4) as u32;
        let hash = (bit << 16) | a2;
        let bucket = hash % 0x190;
        let node = OBJ3;
        c.g[A0][0] = OBJ as u64;
        c.g[A1][0] = bit as u64;
        c.g[A2][0] = a2 as u64;
        c.g[A3][0] = if seed & 1 == 0 {
            0
        } else {
            0xffff_ffff_ffff_ffff
        };
        w(c, OBJ + 0x1900 + bucket * 4, node);
        w(c, OBJ + 0x1f40, 0);
        w(c, node, hash);
        c.bus.write16(node + flags_off, if seed % 4 == 0 { 0 } else { 0xffff });
        c.bus.write16(node + value_off, (seed % 7) as u16);
        c.bus.write16(node + ref_off, if seed % 3 == 0 { 0xffff } else { 1 });
    }


    #[test]
    fn bolt_file_index_manager_set_model_slot_matches() {
        check(
            ee::s03::f_0019cc30__generated,
            bolt_file_index_manager_set_model_slot,
            6,
            &|c, seed| bolt_node(c, seed, 6, 0xa, 8),
        );
    }


    #[test]
    fn bolt_file_index_manager_set_texture_slot_matches() {
        check(
            ee::s03::f_0019cd38__generated,
            bolt_file_index_manager_set_texture_slot,
            6,
            &|c, seed| bolt_node(c, seed, 4, 8, 0xa),
        );
    }


    #[test]
    fn cfe_state_char_equip_on_child_event_matches() {
        check(
            ee::s03::f_00199870__generated,
            cfe_state_char_equip_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1998, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let widget = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 7,
                };
                c.bus.write32(widget + 0xc, (seed % 9) as u32);
                c.bus
                    .write32(widget + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x1c, r.next_u32());
                c.bus.write32(this + 0x5c, DATA + 0x2400);
                c.bus.write32(DATA + 0x2400 + 0x95, seed % 4);
            },
        );
    }


    #[test]
    fn cfe_state_char_equip_category_has_flagged_item_matches() {
        check(
            ee::s03::f_0019afd0__generated,
            cfe_state_char_equip_category_has_flagged_item,
            40,
            &|c, seed| {
                let _r = base_5(c, seed, 0x19af);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[A3][0] = 0;
                // Empty category: the collect call returns 0 immediately.
                c.bus.write32(DATA + 0xbc, 0);
                c.bus.write32(DATA + 0x1000 + 0xbc, 0);
            },
        );
    }


    #[test]
    fn bolt_file_index_manager_free_node_matches() {
        strict(
            ee::s03::f_0019ca70__generated,
            bolt_file_index_manager_free_node,
            80,
            &|c, seed| {
                let _r = base_5(c, seed, 0x19ca);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    (DATA + 0x1000) as u64
                };
                let node = DATA + 0x1000;
                c.bus.write32(node, (seed % 401) as u32);
                c.bus.write16(node + 8, 0xffff);
                c.bus.write16(node + 0xa, 0xffff);
                c.bus.write16(node + 0xc, 0xffff);
                c.bus.write16(node + 0xe, 0xffff);
                c.bus.write32(DATA + 0x1f40, 0);
            },
        );
    }


    #[test]
    fn char_equip_category_all_flagged_matches() {
        diff(
            ee::s03::f_0019b098__generated,
            c_fe_state_char_equip_category_all_flagged,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x19_b098);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = 0;
                c.g[A3][0] = (seed % 4) as u64;
                w(c, DATA + 0xbc, DATA + 0x3000);
                w(c, DATA + 0x3000, 0);
                w(c, DATA + 0x3004, r.next_u32());
            },
        );
    }


        #[test]
    fn cfestate_charequip_dtor_matches() {
        strict_2(ee::s03::f_00199148__generated, cfestate_charequip_dtor, SEEDS, &|c, seed| {
            let mut r = base_6(c, seed, 0x9148, 0);
            let this = DATA + 0x100;
            let world = DATA + 0x8_0000;
            w(c, world + 0x7c, DATA + 0x2000);
            // no bolt-file bindings: the release paths are skipped
            w(c, this + 0xa64, 0xffff_ffff);
            w(c, this + 0xa68, 0xffff_ffff);
            w(c, this + 0xa70, 0xffff_ffff);
            w(c, this + 0xa74, 0xffff_ffff);
            w(c, this + 0xbc, r.next_u32() & 3);
            w(c, GAME_GP.wrapping_sub(0x854), 0);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    pub(crate) const EQUIP: u32 = DATA + 0x1_0000;

    pub(crate) const EQUIP_TABLE: u32 = DATA + 0x1_1000;

    pub(crate) const EQUIP_NAME: u32 = DATA + 0x1_2000;


    #[test]
    fn c_fe_state_char_equip_get_preview_texture_matches() {
        check_all(
            ee::s03::f_0019a098__generated,
            c_fe_state_char_equip_get_preview_texture,
            24,
            &|c, seed| {
                let _ = base_7(c, seed, 0x19a0_98, 8);
                c.g[A0][0] = sx(EQUIP);
                c.g[A1][0] = sx(EQUIP_NAME);
                c.bus.write32(EQUIP + 0xa60, (seed % 2) as u32);
                c.bus.write32(EQUIP + 0xbc, seed % 4);
                c.bus.write32(EQUIP + 0xa64, (seed / 2) % 3);
                c.bus.write32(EQUIP + 0xa68, 0xffff_ffff);
                c.bus.write8(EQUIP_NAME, b'a' + (seed % 3) as u8);
                // The equip table the package lookup starts from.
                c.bus.write32(WORLD_3 + 0x7c, EQUIP_TABLE);
                if seed % 3 == 0 {
                    c.bus.write32(EQUIP_TABLE + 0x1a70, DATA + 0x1_3000);
                }
            },
        );
    }


    // ------------------------------------------- preview camera / input / request

    /// The equip state at `DATA`, its rider preview module through the world
    /// (`gp-0x848 -> +0x7c`, preview 0 at `module + 0xb0`), the camera
    /// constants at `gp-0x59a0..-0x5970` and the preview fields.
    fn preview_camera_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x19_bfe8, 0);
        let st = DATA;
        let world = DATA + 0x8_0000;
        let module = DATA + 0x2000;
        let preview = module + 0xb0;
        w(c, GAME_GP - 0x848, world);
        w(c, world + 0x7c, module);
        for k in (0x5970u32..=0x59a0).step_by(4) {
            w(c, GAME_GP - k, r.f32(300.0));
        }
        // The two 0.2 glide factors / settle thresholds.
        w(c, GAME_GP - 0x5990, 0x3e4c_cccd);
        w(c, GAME_GP - 0x5984, 0x3e4c_cccd);
        c.bus.write8(st + 0x44, 0);
        w(c, preview, if seed % 9 == 0 { 10 } else { seed % 4 });
        w(c, preview + 0xcb8, if seed % 11 == 1 { 0 } else { 1 });
        w(c, preview + 0xcb4, if seed % 13 == 2 { 0 } else { 1 });
        let state = seed % 6;
        w(c, st + equip_preview::CAM_STATE, state);
        w(c, st + equip_preview::ZOOM, r.f32(1.0));
        w(c, st + equip_preview::PITCH, r.f32(1.0));
        w(c, st + equip_preview::YAW, r.f32(360.0));
        // Near the wrap sometimes.
        let spin = if seed % 3 == 0 { 359.5f32.to_bits() } else { r.f32(360.0) };
        w(c, st + equip_preview::SPIN, spin);
        for i in 0..4 {
            w(c, st + equip_preview::CAM_A + 4 * i, r.f32(300.0));
            w(c, st + equip_preview::CAM_B + 4 * i, r.f32(300.0));
        }
        if seed % 4 == 1 {
            // Already at the view: the settle branches.
            if state == 3 {
                put_words(c, st + equip_preview::CAM_B, &[(-100f32).to_bits(), 45f32.to_bits(), 15f32.to_bits(), 1f32.to_bits()]);
            } else {
                w(c, st + equip_preview::ZOOM, 0);
                w(c, st + equip_preview::PITCH, 0);
                put_words(c, st + equip_preview::CAM_A, &[(-105f32).to_bits(), (-100f32).to_bits(), (-80f32).to_bits(), 1f32.to_bits()]);
            }
        }
        c.g[A0][0] = sx(st);
    }

    #[test]
    fn cfe_state_char_equip_update_preview_camera_matches() {
        check(
            ee::s03::f_0019bfe8__generated,
            cfe_state_char_equip_update_preview_camera,
            96,
            &|c, seed| preview_camera_setup(c, seed),
        );
    }

    /// Run a replacement once from the current state (returns to the
    /// sentinel `ra`).
    fn run_hand(c: &mut Cpu, f: GuestFn) {
        c.g[RA][0] = SENTINEL as u64;
        f(c);
        assert_eq!(c.pc, SENTINEL, "returned");
    }

    fn fbits(c: &mut Cpu, a: u32) -> f32 {
        f32::from_bits(c.bus.read32(a))
    }

    #[test]
    fn cfe_state_char_equip_update_preview_camera_rate() {
        // State 3 far from its view: one tick at 60 = two ticks at 120 for
        // the glide (both vectors) and the turntable spin.
        let run = |hz: u32| -> Vec<f32> {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            preview_camera_setup(c, 3);
            crate::tick::set_for_test(c, hz);
            let st = DATA;
            w(c, st + equip_preview::CAM_STATE, 3);
            w(c, st + equip_preview::SPIN, 10f32.to_bits());
            put_words(c, st + equip_preview::CAM_A, &[0, 0, 0, 1f32.to_bits()]);
            put_words(c, st + equip_preview::CAM_B, &[0, 0, 0, 1f32.to_bits()]);
            for _ in 0..hz / 60 {
                c.g[A0][0] = sx(st);
                run_hand(c, cfe_state_char_equip_update_preview_camera);
            }
            assert_eq!(c.bus.read32(st + equip_preview::CAM_STATE), 3, "still gliding");
            let mut v = vec![fbits(c, st + equip_preview::SPIN)];
            for i in 0..3 {
                v.push(fbits(c, st + equip_preview::CAM_A + 4 * i));
                v.push(fbits(c, st + equip_preview::CAM_B + 4 * i));
            }
            v
        };
        let a = run(60);
        assert!((a[0] - 11.0).abs() < 1e-5, "spin {}", a[0]);
        for hz in [120, 240] {
            let b = run(hz);
            for (x, y) in a.iter().zip(&b) {
                assert!((x - y).abs() <= 1e-3 * (1.0 + x.abs()), "{hz} Hz: {a:?} vs {b:?}");
            }
        }
    }

    /// Pad fakes for `HandleInput`: the vtable slots in the order
    /// [`PAD_SLOTS`] point at `FAKE + 0x10 * i`; the "active" queries
    /// (`0x118`, `0x108`, `0x110`) return 0/1, the stick reads -1..1 in f0.
    const PAD_SLOTS: [u32; 6] = [0x118, 0x130, 0x108, 0x120, 0x110, 0x128];

    fn pad_fake(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        match target.wrapping_sub(FAKE) {
            0x00 | 0x20 | 0x40 => c.g[V0][0] = (h >> 7) & 1,
            _ => c.f[0] = (((h >> 8) & 0xffff) as f32 / 32768.0 - 1.0).to_bits(),
        }
    }

    fn pv_pad(c: &mut Cpu, r: &mut Rng, at_: u32) {
        let vt = at_ + 0x200;
        w(c, at_ + 8, vt);
        for (i, off) in PAD_SLOTS.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3 & 0xff) as u32;
            c.bus.write16(vt + off, delta as u16);
            w(c, vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    #[test]
    fn cfe_state_char_equip_handle_input_matches() {
        check(ee::s03::f_00199a38__generated, cfe_state_char_equip_handle_input, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x19_9a38, 6);
            c.hooks.miss = pad_fake;
            let st = DATA;
            let p = DATA + 0x4000;
            pv_pad(c, &mut r, p);
            w(c, st + equip_preview::MODE, if seed % 7 == 0 { 1 } else { seed % 3 });
            w(c, st + equip_preview::CAM_STATE, [1, 4, 1, 4, 2, 0][(seed % 6) as usize]);
            w(c, st + equip_preview::YAW, r.f32(1000.0));
            w(c, st + equip_preview::ZOOM, r.f32(1.2));
            w(c, st + equip_preview::PITCH, r.f32(1.2));
            w(c, GAME_GP - 0x59a8, 0xbdcc_cccd);
            w(c, GAME_GP - 0x59a4, 0xbdcc_cccd);
            c.g[A0][0] = sx(st);
            c.g[A1][0] = sx(p);
        });
    }

    /// Only the left stick, held at 0.5.
    fn pad_yaw(c: &mut Cpu, target: u32) {
        match target.wrapping_sub(FAKE) {
            0x00 => c.g[V0][0] = 1,
            0x10 => c.f[0] = 0.5f32.to_bits(),
            _ => c.g[V0][0] = 0,
        }
        c.pc = c.g[RA][0] as u32;
    }

    #[test]
    fn cfe_state_char_equip_handle_input_rate() {
        // The yaw turns as far in one 60 Hz tick as in rate/60 ticks.
        let run = |hz: u32| -> f32 {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            let mut r = machine(c, 1, 0x19_9a38, 0);
            c.hooks.miss = pad_yaw;
            crate::tick::set_for_test(c, hz);
            let st = DATA;
            let p = DATA + 0x4000;
            pv_pad(c, &mut r, p);
            w(c, st + equip_preview::MODE, 0);
            w(c, st + equip_preview::CAM_STATE, 1);
            w(c, st + equip_preview::YAW, 10f32.to_bits());
            for _ in 0..hz / 60 {
                c.g[A0][0] = sx(st);
                c.g[A1][0] = sx(p);
                run_hand(c, cfe_state_char_equip_handle_input);
                assert_eq!(c.g[V0][0], 1);
            }
            fbits(c, st + equip_preview::YAW)
        };
        assert_eq!(run(60), 11.5);
        for hz in [120, 240] {
            assert!((run(hz) - 11.5).abs() < 1e-4, "{hz} Hz");
        }
    }

    /// The equip state at `EQUIP` with an item list, the item's model name,
    /// the BELibrary (`GetInterface(9)` a fake), the screen dummy at
    /// `gp-0x854` (its texture release a fake) and the game image (the bolt
    /// category path table and libc).
    fn request_preview_setup(c: &mut Cpu, seed: u32, calls: u32) {
        crate::difftest::elf_image(c);
        let mut r = crate::game::fe::fepopup::tests::base_14(c, seed, 0x19_a4e8, calls);
        use crate::game::fe::fepopup::tests::AUX;
        let st = EQUIP;
        let list = DATA + 0x1_4000;
        let item = DATA + 0x1_5000;
        let name = DATA + 0x1_6000;
        // BELibrary_GetInterface: index 0 of the interface table.
        w(c, GAME_GP - 0x1efc, 0);
        let iface = DATA + 0x1_7000;
        let vt = DATA + 0x1_7100;
        w(c, 0x0053_08a8, iface);
        w(c, iface + 4, vt);
        c.bus.write16(vt + 8, 0);
        w(c, vt + 0xc, FAKE);
        // The screen's texture release.
        let auxv = AUX + 0x2000;
        w(c, AUX + 0x10d8, auxv);
        c.bus.write16(auxv + 0x190, 0);
        w(c, auxv + 0x194, FAKE + 0x10);
        w(c, st + 0x5c, if seed % 7 == 0 { 0 } else { list });
        w(c, st + 0xa6c, u32::MAX);
        w(c, st + 0xa70, r.next_u32());
        w(c, st + 0xa74, if seed % 2 == 0 { u32::MAX } else { seed % 5 });
        let idx = seed % 4;
        c.bus.write8(list + 0x95, idx as u8);
        w(c, st + 0x958, 3);
        w(c, st + 0x124 + 4 * idx, item);
        w(c, item + 0x30, if seed % 6 == 1 { 0 } else { name });
        cstr(c, name, ["hat07", "abc", "", "x9y", "12"][(seed % 5) as usize]);
        w(c, st + 0xbc, seed % 10);
        c.g[A0][0] = sx(st);
        c.g[A1][0] = (seed % 3 != 0) as u64;
    }

    #[test]
    fn cfe_state_char_equip_request_preview_matches() {
        check(
            ee::s03::f_0019a4e8__generated,
            cfe_state_char_equip_request_preview,
            60,
            &|c, seed| request_preview_setup(c, seed, 2),
        );
    }

    #[test]
    fn cfe_state_char_equip_request_preview_rate() {
        // The 20-frame delay as ticks: 20 at 60, 40 at 120, 80 at 240.
        for hz in [60u32, 120, 240] {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            request_preview_setup(c, 5, 0);
            crate::tick::set_for_test(c, hz);
            c.g[A1][0] = 1;
            run_hand(c, cfe_state_char_equip_request_preview);
            assert_eq!(c.bus.read32(EQUIP + equip_preview::PREVIEW_DELAY), 20 * hz / 60, "{hz} Hz");
        }
    }
}
