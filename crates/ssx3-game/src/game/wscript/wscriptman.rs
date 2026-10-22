//! `src/wscript/wscriptman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::abi::r32;
use crate::events;
use crate::call::Flow;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::funcs::call_indirect;
use callee::*;
use crate::call::Unwound;
use crate::events::deliver_event;
pub(crate) use crate::game::sound::asyncsys::lw;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::world::wscriptcache::DELETE_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_5;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_cdf8, playback_man_stop, "PlaybackMan_Stop"),
    (0x0030_7d78, wscript_process_terminate, "WScriptProcess_Terminate"),
    (0x0030_9568, wscript_man_create_mission, "WScriptMan_CreateMission"),
    (0x0030_9750, w_script_man_start_process, "WScriptMan_StartProcess"),
    (0x0030_9798, w_script_man_stop_process, "WScriptMan_StopProcess"),
    (0x0030_97e0, wscript_man_post_trigger_event, "WScriptMan_PostTriggerEvent"),
    (0x0030_9848, wscript_man_post_jump_event, "WScriptMan_PostJumpEvent"),
    (0x0030_9918, wscript_man_post_trick_event, "WScriptMan_PostTrickEvent"),
    (0x0030_9990, wscript_man_post_wipeout_event, "WScriptMan_PostWipeoutEvent"),
    (0x0030_99f8, wscript_man_post_reset_event, "WScriptMan_PostResetEvent"),
    (0x0030_9a60, wscript_man_activate_process, "WScriptMan_ActivateProcess"),
    (0x0030_9aa0, wscript_man_deactivate_process, "WScriptMan_DeactivateProcess"),
    (0x0030_9b00, w_script_man_deactivate_mission, "WScriptMan_DeactivateMission"),
    (0x0030_9ba8, w_script_man_get_hud_target_height, "WScriptMan_GetHudTargetHeight"),
    (0x0030_9dd0, w_script_man_table_has_function, "WScriptMan_TableHasFunction"),
    (0x0030_9e50, wscript_man_call_table_function, "WScriptMan_CallTableFunction"),
    (0x0030_a270, wscript_man_has_obj_script1, "WScriptMan_HasObjScript1"),
    (0x0030_a310, wscript_man_is_scripted_instance, "WScriptMan_IsScriptedInstance"),
    (0x0030_a598, wscript_man_has_kill_script, "WScriptMan_HasKillScript"),
    (0x0030_a610, wscript_man_run_obj_script0, "WScriptMan_RunObjScript0"),
    (0x0030_a6d8, wscript_man_jump_to_task, "WScriptMan_JumpToTask"),
    (0x0030_a700, wscript_man_goto_task, "WScriptMan_GotoTask"),
    (0x0030_aeb8, wscript_man_get_current_mission, "WScriptMan_GetCurrentMission"),
    (0x0030_b058, wscript_process_list_ctor, "WScriptProcessList_ctor"),
    (0x0030_b068, w_script_process_list_dtor, "WScriptProcessList_dtor"),
    (0x0030_b0e8, wscript_process_list_delete_by_section, "WScriptProcessList_DeleteBySection"),
    (0x0030_b1b8, wscript_process_list_delete_all, "WScriptProcessList_DeleteAll"),
    (0x0030_b208, wscript_process_list_pop_front, "WScriptProcessList_PopFront"),
    (0x0030_b260, wscript_process_list_push_front, "WScriptProcessList_PushFront"),
    (0x0030_b270, wscript_process_list_push_back, "WScriptProcessList_PushBack"),
    (0x0030_b2c0, wscript_process_list_push_front_2, "WScriptProcessList_PushFront_2"),
    (0x0030_b2d0, wscript_process_list_remove, "WScriptProcessList_Remove"),
    (0x0030_b320, wscript_process_list_find_by_id, "WScriptProcessList_FindById"),
    (0x0030_b388, wscript_process_list_find_mission_by_id, "WScriptProcessList_FindMissionById"),
    (0x0030_b428, wscript_process_list_contains, "WScriptProcessList_Contains"),
    (0x0030_b520, wscript_man_set_pending_rider_config, "WScriptMan_SetPendingRiderConfig"),
    (0x0030_b540, wscript_man_request_mission_start, "WScriptMan_RequestMissionStart"),
    (0x0030_b658, wscript_man_request_mission_cancel, "WScriptMan_RequestMissionCancel"),
    (0x0030_b6f8, w_script_man_request_mission_restart, "WScriptMan_RequestMissionRestart"),
    (0x0030_b7f8, wscript_man_deactivate_all, "WScriptMan_DeactivateAll"),
    (0x0030_b928, wscript_man_define_collect_set, "WScriptMan_DefineCollectSet"),
    (0x0030_b9a0, w_script_man_collect_item, "WScriptMan_CollectItem"),
    (0x0030_ba80, wscript_man_get_collected_count, "WScriptMan_GetCollectedCount"),
    (0x0030_bac8, wscript_man_add_collect_items, "WScriptMan_AddCollectItems"),
    (0x0030_bee8, wscriptman_load_process_list, "WScriptMan_LoadProcessList"),
    (0x0030_c390, wscript_man_reset_collect_set, "WScriptMan_ResetCollectSet"),
    (0x0030_c3e0, wscript_man_collect_set_mark_item, "WScriptMan_CollectSetMarkItem"),
    (0x0030_c468, wscript_man_collect_set_get_count, "WScriptMan_CollectSetGetCount"),
    (0x0030_c700, wscript_man_collect_set_load, "WScriptMan_CollectSetLoad"),
    (0x0030_c738, wscript_man_cmd_queue_ctor, "WScriptMan_CmdQueueCtor"),
    (0x0030_c760, wscript_man_reset_command_queue, "WScriptMan_ResetCommandQueue"),
    (0x0030_c790, wscript_man_event_queue_reset, "WScriptMan_EventQueueReset"),
    (0x0030_c7c0, wscript_man_event_queue_push, "WScriptMan_EventQueuePush"),
    (0x0030_d420, wscript_man_sub_obj_dtor_d420, "WScriptMan_SubObjDtor_d420"),
    (0x0030_d4b8, rider_model_ctor, "RiderModel_ctor"),
    (0x0030_d840, rider_model_alloc_model_refs, "RiderModel_AllocModelRefs"),
    (0x0035_06d8, dead_node_ctor, "DeadNode_ctor"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT1;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT3;
    pub(crate) use crate::game::object::debouncenode::callee::GET_OBJECT_SLOT;
    pub(crate) use crate::game::object::debouncenode::callee::GET_PLAYER_INSTANCE;
    pub(crate) use crate::game::object::debouncenode::callee::GET_HUMAN_RIDER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_CHARACTER;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRACK_PEAK;
    pub(crate) use crate::game::object::debouncenode::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::object::debouncenode::callee::EVAL_SEGMENT;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
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
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR as ACTIVE_CAMERA_TRIGGER_LIST_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;
    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_CONTAINS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE as WSCRIPT_PROCESS_LIST_REMOVE_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT as WSCRIPT_PROCESS_LIST_PUSH_FRONT_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_KILL_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT as BUDDY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_3;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS as MENU_SYS_ITEM_ON_FOCUS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS as SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES as FLEX_BRIDGE_ALLOC_SPRING_NODES_2;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE as CBE_GET_BE_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT_3;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT as QSORT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_4;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN as USTR_LEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY as USTR_CPY_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;


    pub const SET_FADE_GAIN: Callee = Callee {
        addr: 0x0028_7a10,
        f: ee::s07::SSXAudioSystem_SetFadeGain,
    };

    pub const SND_BANK_UNLOAD: Callee = Callee {
        addr: 0x0028_b528,
        f: ee::s07::SndBank_Unload,
    };

    pub const BANK_MONITOR_ADD_TIMER: Callee = Callee {
        addr: 0x002a_dca0,
        f: ee::s08::BankMonitor_AddTimer,
    };

    pub const CROWD_AUDIO_INSTANCE_STOP_SOUND: Callee = Callee {
        addr: 0x002a_6848,
        f: ee::s08::CrowdAudio_InstanceStopSound,
    };

    pub const CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES: Callee = Callee {
        addr: 0x002a_6430,
        f: ee::s08::CrowdAudio_InstanceRemoveAllSources,
    };

    pub const BANK_MONITOR_INSTANCE_POLL_FREE: Callee = Callee {
        addr: 0x002a_bb80,
        f: ee::s08::BankMonitorInstance_PollFree,
    };

    pub const BANK_MONITOR_INSTANCE_SET_PITCH: Callee = Callee {
        addr: 0x002a_be78,
        f: ee::s08::BankMonitorInstance_SetPitch,
    };

    pub const CPS2_MEMCARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::cPS2MemCard_SetState,
    };

    pub const AVALANCHE_BAKE_NODE_HISTORY: Callee = Callee {
        addr: 0x002d_66a0,
        f: ee::s09::Avalanche_BakeNodeHistory,
    };

    pub const WSCRIPT_MAN_CALL_FUNC: Callee = Callee {
        addr: 0x0030_9c88,
        f: ee::s10::WScriptMan_CallFunc,
    };

    pub const CROWD_MAN3D_GROUP_FREE_MESHES: Callee = Callee {
        addr: 0x0034_4800,
        f: ee::s11::CrowdMan3d_GroupFreeMeshes,
    };

    pub const CROWD_MAN3D_GROUP_CREATE_MESHES_2: Callee = Callee {
        addr: 0x0034_43a8,
        f: ee::s11::CrowdMan3d_GroupCreateMeshes,
    };

    pub const FLEX_RAIL_UPDATE_SPACE_NODE: Callee = Callee {
        addr: 0x0034_8058,
        f: ee::s11::FlexRail_UpdateSpaceNode,
    };

    pub const C_FLAG_MANAGER_GROUP_LOAD: Callee = Callee {
        addr: 0x0034_c378,
        f: ee::s11::cFlagManager_GroupLoad,
    };

    pub const FLOATING_V49: Callee = Callee {
        addr: 0x0034_f048,
        f: ee::s11::Floating_v49,
    };

    pub const MOD_OBJ_DTOR: Callee = Callee {
        addr: 0x0035_53c0,
        f: ee::s11::ModObj_dtor,
    };

    pub const MODIFIER_LIST_UNLINK: Callee = Callee {
        addr: 0x0034_5760,
        f: ee::s11::ModifierList_Unlink,
    };

    pub const RAIL_MODIFIER_LIST_UNLINK: Callee = Callee {
        addr: 0x0035_b6d0,
        f: ee::s11::RailModifier_ListUnlink,
    };

    pub const DISABLE_INTC: Callee = Callee {
        addr: 0x0042_4880,
        f: ee::s15::DisableIntc,
    };

    pub const REMOVE_INTC_HANDLER: Callee = Callee {
        addr: 0x0042_3aa0,
        f: ee::s15::RemoveIntcHandler,
    };

    pub const TEX_MAN_LOAD_TEXTURE: Callee = Callee {
        addr: 0x0036_7440,
        f: ee::s12::TexMan_LoadTexture,
    };

    pub const LIGHT_MAN_EMPTY_HOOK_A: Callee = Callee {
        addr: 0x0038_dc50,
        f: ee::s12::LightMan_EmptyHookA,
    };

    pub const UI_OBJECT_GET_STATE_DATA: Callee = Callee {
        addr: 0x0039_ff50,
        f: ee::s13::UIObject_GetStateData,
    };

    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::UIListNode_dtorBody,
    };

    pub const CPS2_CHUNK_RESOLVE_CTOR: Callee = Callee {
        addr: 0x003a_a2f0,
        f: ee::s13::cPS2ChunkResolve_ctor,
    };

    pub const AV_PLAYER_INIT: Callee = Callee {
        addr: 0x003a_dec8,
        f: ee::s13::AV_Player_Init,
    };

    pub const FOCUS_MAN_ADD_FOCUS_BOX: Callee = Callee {
        addr: 0x0010_33b0,
        f: ee::s00::FocusMan_AddFocusBox,
    };

    pub const CAM_CONTROLLER_CLEAR_ALGORITHMS: Callee = Callee {
        addr: 0x0015_cd60,
        f: ee::s01::CamController_ClearAlgorithms,
    };

    pub const ALGORITHM_LIST_PUSH: Callee = Callee {
        addr: 0x0015_c988,
        f: ee::s01::AlgorithmList_Push,
    };

    pub const APP_MODULE_UPDATE_FADE: Callee = Callee {
        addr: 0x0023_1d18,
        f: ee::s06::AppModule_UpdateFade,
    };

    pub const DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL: Callee = Callee {
        addr: 0x0017_f2d0,
        f: ee::s02::DebugMenuModule_UpdateCursorTrail,
    };

    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: Callee = Callee {
        addr: 0x0019_4498,
        f: ee::s02::UIObject_ShowClampAlphaHalf,
    };

    pub const C_SETUP_MAN_SEND_QUIT: Callee = Callee {
        addr: 0x0026_5768,
        f: ee::s07::cSetupMan_SendQuit,
    };

    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER: Callee = Callee {
        addr: 0x001a_94d8,
        f: ee::s03::cFEStateOnlineCharacterSelect_EmptyChildHandler,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const UI_OBJECT_SET_USER_DATA: Callee = Callee {
        addr: 0x003a_0330,
        f: ee::s13::UIObject_SetUserData,
    };

    pub const EA_FILE_MOUNT_BIG: Callee = Callee {
        addr: 0x003d_ed50,
        f: ee::s14::EAFile_MountBig,
    };

    pub const NET_FE_STATE_CTOR: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };

    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
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

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {

    pub const IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;

    pub const GET_CHANNEL_VOLUME: u32 = 0x0028_7920;

    pub const SND_BANK_REMOVE: u32 = 0x003b_6300;

    pub const SND_MIDI_RESUME: u32 = 0x003b_9dc8;

    pub const SCE_MC_FLUSH: u32 = 0x0040_ab80;

    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;

    pub const BUILTIN_VEC_DELETE: u32 = 0x0031_7e98;

    pub const MEM_FREE: u32 = 0x0025_23a8;

    pub const WORLD_GET_OBJECT_SCRIPT_FUNC: u32 = 0x003a_6b78;

    pub const WSCRIPT_PROCESS_LIST_POP_FRONT: u32 = 0x0030_b208;

    pub const STR_HASH: u32 = 0x0031_7670;

    pub const SERIALIZE_READ_U32: u32 = 0x002d_2510;

    pub const SERIALIZE_READ_U32_2: u32 = 0x002d_2558;

    pub const BXFILE_EXISTS: u32 = 0x0031_7f38;

    pub const STREAM_FILE_DESTROY: u32 = 0x003e_0a28;

    pub const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
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
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
}


// ---------------------------------------------------------------- wscript list

/// `WScriptProcessNode *WScriptProcessList_PopFront(WScriptProcessList *l)`
/// (0x30b208): unlink and return the head node (its `+0x18` is the next
/// pointer), or `NULL` when the list is empty.
pub fn wscript_process_list_pop_front(c: &mut Cpu) {
    c.cycles += 3;
    let list = Addr(abi::arg(c, 0));
    let head = list.lw(c);
    c.g[V1][0] = head;
    if head != 0 {
        c.cycles += 2;
        let next = Addr(head as u32).off(0x18).lw(c);
        c.g[V0][0] = next;
        list.sw(c, next);
    }
    c.cycles += 2;
    c.g[V0][0] = head;
    abi::ret(c);
}


/// `void WScriptProcessList_PushFront(WScriptProcessList *l,
/// WScriptProcessNode *n)` (0x30b260): link `n` before the head (in
/// `n+0x18`); returns the previous head in `v0`.
pub fn wscript_process_list_push_front(c: &mut Cpu) {
    c.cycles += 4;
    let list = Addr(abi::arg(c, 0));
    let node = Addr(abi::arg(c, 1));
    let head = list.lw(c);
    c.g[V0][0] = head;
    node.off(0x18).sw(c, head);
    list.sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// `void WScriptMan_PostTriggerEvent(WScriptMan *this, int value)` (0x3097e0):
/// append a `0x70000` trigger event to the queue at `this+0x40`.
pub fn wscript_man_post_trigger_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).off(0x40).lw(c);
    c.g[V0][0] = 0xc;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.g[A2][0] = 0x7_0000;
    c.mult(0, V0, V1, V0);
    Addr(sp).off(0x4).sw(c, c.g[A1][0]);
    Addr(sp).sw(c, c.g[A2][0]);
    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(A2, sp.wrapping_add(0x7));
    c.ldr(A2, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(A2, sp.wrapping_add(0x17));
    c.sdr(A2, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.ldl(A1, sp.wrapping_add(0x17));
    c.ldr(A1, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(A1, (c.g[V0][0] as u32).wrapping_add(0xb));
    c.sdr(A1, (c.g[V0][0] as u32).wrapping_add(0x4));
    Addr((c.g[V0][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[V1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}


/// `void WScriptMan_PostJumpEvent(WScriptMan *this)` (0x309848): append a
/// `0x70001` jump event.
pub fn wscript_man_post_jump_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = c.g[V0][0] | 0x1;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.ldl(V0, sp.wrapping_add(0x17));
    c.ldr(V0, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}


/// `void WScriptMan_PostTrickEvent(WScriptMan *this, const void *trick)`
/// (0x309918): append a `0x70003` trick event carrying the first quad of the
/// trick record.
pub fn wscript_man_post_trick_event(c: &mut Cpu) {
    c.cycles += 29;
    let this = abi::arg(c, 0);
    let trick = c.g[A1][0] as u32;
    c.g[A2][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A2, V1);
    c.g[V0][0] = c.g[V0][0] | 0x3;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, trick.wrapping_add(0x7));
    c.ldr(V0, trick);
    c.sdl(V0, sp.wrapping_add(0xb));
    c.sdr(V0, sp.wrapping_add(0x4));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A1][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.ldl(T2, sp.wrapping_add(0x17));
    c.ldr(T2, sp.wrapping_add(0x10));
    c.g[V0][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(T2, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(T2, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[V0][0]);
    Addr(this).off(0x40).sw(c, c.g[A2][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}


/// `void WScriptMan_PostWipeoutEvent(WScriptMan *this)` (0x309990): append a
/// `0x70004` wipeout event.
pub fn wscript_man_post_wipeout_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = c.g[V0][0] | 0x4;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.ldl(V0, sp.wrapping_add(0x17));
    c.ldr(V0, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}


/// `void WScriptMan_PostResetEvent(WScriptMan *this)` (0x3099f8): append a
/// `0x70005` reset event.
pub fn wscript_man_post_reset_event(c: &mut Cpu) {
    c.cycles += 25;
    let this = abi::arg(c, 0);
    c.g[A1][0] = Addr(this).off(0x40).lw(c);
    c.g[V1][0] = 0xc;
    c.g[V0][0] = 0x7_0000;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    let sp = c.g[SP][0] as u32;
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = c.g[V0][0] | 0x5;
    Addr(sp).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.ldl(V0, sp.wrapping_add(0x7));
    c.ldr(V0, sp);
    c.g[A3][0] = Addr(sp).off(0x8).lw(c);
    c.sdl(V0, sp.wrapping_add(0x17));
    c.sdr(V0, sp.wrapping_add(0x10));
    Addr(sp).off(0x18).sw(c, c.g[A3][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.ldl(V0, sp.wrapping_add(0x17));
    c.ldr(V0, sp.wrapping_add(0x10));
    c.g[A2][0] = Addr(sp).off(0x18).lw(c);
    c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(0xb));
    c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(0x4));
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    Addr(this).off(0x40).sw(c, c.g[A1][0]);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    abi::ret(c);
}


/// `void WScriptProcessList_ctor(WScriptProcessList *this)` (0x30b058):
/// clear the list head.
pub fn wscript_process_list_ctor(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0];
    Addr(abi::arg(c, 0)).sw(c, 0);
    abi::ret(c);
}


/// `void WScriptProcessList_PushFront_2(WScriptProcessList *this,
/// WScriptProcess *item)` (0x30b2c0): link `item` at the front, chaining the
/// previous head at `item+0x18`.
pub fn wscript_process_list_push_front_2(c: &mut Cpu) {
    c.cycles += 4;
    let list = abi::arg(c, 0);
    c.g[V0][0] = Addr(list).lw(c);
    Addr(c.g[A1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
    Addr(list).sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// `void WScriptMan_EventQueuePush(WScriptMan *this, int a, int b, int c)`
/// (0x30c7c0): store three words at the ring slot `this+4` (12-byte records,
/// 16 per page) and advance the wrapped index.
pub fn wscript_man_event_queue_push(c: &mut Cpu) {
    c.cycles += 24;
    let this = abi::arg(c, 0);
    c.g[V1][0] = Addr(this).off(0x4).lw(c);
    c.g[T0][0] = 0xc;
    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V0, V1, T0);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(this).off(0x4).lw(c);
    c.mult(0, V1, V0, T0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr((c.g[V0][0] as u32).wrapping_add(0xc)).sw(c, c.g[A2][0]);
    c.g[V1][0] = Addr(this).off(0x4).lw(c);
    c.mult(0, V0, V1, T0);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[A3][0]);
    c.g[V0][0] = Addr(this).off(0x4).lw(c);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[T1][0] = ((c.g[T1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    if c.g[T1][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    Addr(this).off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// WScript.

/// `int WScriptMan_IsScriptedInstance(WScriptMan* self, WorldObj* obj)`
/// (0x30a310): 1 if the object is a scripted instance — it has a live
/// virtual at `+0xc` (a rail object with an update), or its 12-byte script
/// table entry (`*(*(self + 0x28c) + 4) + 4`, indexed by the object's
/// `+0x78` byte) has type 3 and the object has a script 1 or script 3 slot.
pub fn wscript_man_is_scripted_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        let obj = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(obj).off(0xc).lw(c);
        let has_virtual = c.g[V0][0] != 0;
        c.g[S1][0] = abi::arg64(c, 0);
        if has_virtual {
            // Falls through to the common "return 1" tail (+1 cycle).
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 11;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x28c).lw(c);
            c.g[A1][0] = 0xc;
            c.g[V1][0] = Addr(obj).off(0x78).lbu(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(4).lw(c);
            // v1 = id * 12 (the table's record stride), in the delay slot.
            c.mult(0, 3, 3, 5);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lh(c);
            c.g[V0][0] ^= 0x3;
            let not_type3 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if not_type3 {
                // Not a scripted type: return 0 (no extra cycle).
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::HAS_OBJECT_SCRIPT1.call(c, 0x0030_a368)?;
                let has_script1 = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if !has_script1 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S0][0];
                    callee::HAS_OBJECT_SCRIPT3.call(c, 0x0030_a37c)?;
                    let no_script3 = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if no_script3 {
                        // Return 0.
                    } else {
                        // Common return-1 tail: one extra cycle.
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    }
                }
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
// WScriptMan_HasObjScript1.

/// `bool WScriptMan_HasObjScript1(WScriptMan* m, WScriptObj* obj)`
/// (0x30a270): whether the object has a slot 1 script — the script manager's
/// object table (`m + 0x28c`) lookup returns anything but -1.
pub fn wscript_man_has_obj_script1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28c));
        call_ptr(c, 0x003a_6b78, 0x0030_a284)?;
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptMan_SetPendingRiderConfig(this, cWScriptPair *src)`
/// (0x0030b520): set the pending flag at `+0x34` and copy the pair into
/// `+0x38` (an unaligned 8-byte move).
pub fn wscript_man_set_pending_rider_config(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x34), c.g[V0][0]);
    c.ldl(2, (c.g[A1][0] as u32).wrapping_add(7));
    c.ldr(2, (c.g[A1][0] as u32));
    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x3f));
    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x38));
    abi::ret(c);
}


/// `void WScriptMan_HasKillScript(WScriptMan* self, int id)` (0x30a598): true
/// unless the object-script lookup (`self->slot at +0x28c`, class 4) returns
/// -1.
pub fn wscript_man_has_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 4;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x28c)).lw(c);
        call_ptr(c, callee::WORLD_GET_OBJECT_SCRIPT_FUNC, 0x0030_a5ac)?;
        c.g[V0][0] = !c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] = (c.g[V0][0] > 0) as u64;
        frame.close(c);
        Ok(())
    });
}


/// `void WScriptMan_JumpToTask(WScriptMan* self)` (0x30a6d8): make the
/// manager's script process (`+0x2a4`) jump to its current task.
pub fn wscript_man_jump_to_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x2a4)).lw(c);
        let proc = c.g[A0][0];
        frame.restore_ra(c, 0);
        if proc != 0 {
            c.cycles += 3;
            callee::WSCRIPT_PROCESS_JUMP_TO_TASK.call(c, 0x0030_a6f4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void WScriptMan_GotoTask(WScriptMan* self)` (0x30a700): the `GotoTask`
/// flavour of the above.
pub fn wscript_man_goto_task(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x2a4)).lw(c);
        let proc = c.g[A0][0];
        frame.restore_ra(c, 0);
        if proc != 0 {
            c.cycles += 3;
            callee::WSCRIPT_PROCESS_GOTO_TASK.call(c, 0x0030_a71c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `WScriptMan* WScriptMan_CmdQueueCtor(WScriptMan* self)` (0x30c738): reset
/// the command queue and return `self`.
pub fn wscript_man_cmd_queue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE.call(c, 0x0030_c74c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}


/// `WScriptMan_SubObjDtor_d420` (0x30d420): vtable at `+0x40`.
pub fn wscript_man_sub_obj_dtor_d420(c: &mut Cpu) {
    deleting_dtor(c, 0x40, 0x0048_9ae0, 0x0030_d444);
}


/// `WScriptProcessList_FindById(list, id)` (0x30b320): the node whose id
/// (`+0x20`) matches, following `+0x18`.
pub fn wscript_process_list_find_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x18).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0030_b330);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


/// `WScriptProcessList_Contains(list, node)` (0x30b428): whether `node` is in
/// the process list.
pub fn wscript_process_list_contains(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 2;
        if c.g[A0][0] != c.g[A1][0] {
            c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x18).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x0030_b438);
                }
                continue;
            }
            break;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


/// The command/event queue reset shared by both WScriptMan queues: clear the
/// two head words and zero the 0xc0-byte array at `+0x8`.
pub(crate) fn reset_queue(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        let this = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(8));
        Addr(this).sw(c, 0);
        Addr(this).off(4).sw(c, 0);
        c.g[A2][0] = 0xc0;
        call_ptr(c, 0x003e_6448, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `WScriptMan_ResetCommandQueue(WScriptMan* m)` (0x30c760).
pub fn wscript_man_reset_command_queue(c: &mut Cpu) {
    reset_queue(c, 0x0030_c784);
}


/// `WScriptMan_EventQueueReset(WScriptMan* m)` (0x30c790).
pub fn wscript_man_event_queue_reset(c: &mut Cpu) {
    reset_queue(c, 0x0030_c7b4);
}


/// `void WScriptMan_CollectSetLoad(WScriptMan* man, sWScriptStream* stream)`
/// (0x30c700).
pub fn wscript_man_collect_set_load(c: &mut Cpu) {
    guest(c, |c| forward_to_arg_vtable(c, 0x0030_c728, 0x10, 0x10c, None))
}


/// `void WScriptMan_ActivateProcess(WScriptMan* self, Node* node)`
/// (0x309a60): move the process from the waiting list (`+0x2b4`) to the
/// active list (`+0x2b8`).
pub fn wscript_man_activate_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x2b4));
        callee::WSCRIPT_PROCESS_LIST_REMOVE.call(c, 0x0030_9a80)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0x2b8));
        c.g[A1][0] = c.g[S1][0];
        callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT.call(c, 0x0030_9a8c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int WScriptMan_CollectSetGetCount(WScriptMan* self, Entry* entry)`
/// (0x30c468): the collected count of mission `entry->id` in the BE mission
/// interface (interface 10 of the BE library).
pub fn wscript_man_collect_set_get_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BE_LIBRARY_GET_INSTANCE_2.call(c, 0x0030_c47c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0030_c488)?;
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_MISSION_INT_GET_COLLECTED_COUNT.call(c, 0x0030_c498)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// WScript collect-set lookups (single-entry tables).

/// `int WScriptMan_GetCollectedCount(WScriptMan* m, uint id)` (0x30ba80): the
/// collected count of the single collect-set entry, or 0 when the id does not
/// match.
pub fn wscript_man_get_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x2c0));
        frame.save_ra(c, 0);
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, A0));
            let miss = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !miss {
                c.cycles += 4;
                callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT.call(c, 0x0030_baa4)?;
                break;
            }
            c.cycles += 2;
            let exhausted = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10c));
            if exhausted {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_ba90);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            break;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptMan_AddCollectItems(WScriptMan* m, uint id, int a1, int a2)`
/// (0x30bac8): add items to the single collect-set entry when the id matches.
pub fn wscript_man_add_collect_items(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x2c0));
        frame.save_ra(c, 0);
        c.g[V1][0] = 0;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, A0));
            let miss = c.g[V0][0] != c.g[A1][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            if !miss {
                c.cycles += 5;
                c.g[A1][0] = c.g[A2][0];
                c.g[A2][0] = c.g[A3][0];
                callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS.call(c, 0x0030_baf0)?;
                break;
            }
            c.cycles += 2;
            let exhausted = (c.g[V1][0] as i64) <= 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10c));
            if exhausted {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_bad8);
                }
                continue;
            }
            c.cycles += 1;
            break;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptMan_StartProcess(man, process)` (0x309750): make `process` the
/// manager's current process (`+0x2a4`), call its `start` slot (0x20/0x24),
/// and restore the previous current process.
pub fn w_script_man_start_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        let man = c.g[S0][0] as u32;
        c.g[S1][0] = Addr(man).off(0x2a4).lw(c);
        Addr(man).off(0x2a4).sw(c, c.g[A1][0]);
        let process = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(process).off(0x5c).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x20).lh(c);
        c.g[V1][0] = Addr(vt).off(0x24).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0030_9780);
        c.g[A0][0] = sx(process.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_9780)?;
        Addr(man).off(0x2a4).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptMan_StopProcess(man, process)` (0x309798): as
/// [`w_script_man_start_process`] but calls the process's `stop` slot
/// (vtable 0x30/0x34).
pub fn w_script_man_stop_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        let man = c.g[S0][0] as u32;
        c.g[S1][0] = Addr(man).off(0x2a4).lw(c);
        Addr(man).off(0x2a4).sw(c, c.g[A1][0]);
        let process = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(process).off(0x5c).lw(c);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x30).lh(c);
        c.g[V1][0] = Addr(vt).off(0x34).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0030_97c8);
        c.g[A0][0] = sx(process.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_97c8)?;
        Addr(man).off(0x2a4).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// WScript.

/// `void WScriptProcess_Terminate(WScriptProcess* self)` (0x307d78): mark the
/// process dead, detach it from its manager and release the owned object.
pub fn wscript_process_terminate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        Addr(this).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(this).off(0x10).lw(c);
        callee::WSCRIPT_MAN_DEACTIVATE_PROCESS.call(c, 0x0030_7d9c)?;
        c.g[A0][0] = Addr(this).off(0x1c).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_4df0, 0x0030_7db0)?;
        }
        Addr(this).off(0x1c).sw(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* WScriptMan_GetCurrentMission(WScriptMan* self)` (0x30aeb8): the
/// mission object when it accepts the current frame, else 0.
pub fn wscript_man_get_current_mission(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(this).off(0x2a4).lw(c);
        let none = c.g[A1][0] == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 7;
            c.g[V1][0] = Addr(r32(c, A1)).off(0x5c).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x18).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x1c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0030_aee8)?;
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if !zero {
                c.cycles += 1;
                c.g[V0][0] = Addr(this).off(0x2a4).lw(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptProcessList_PushBack(WScriptProcessList* self,
/// cWScriptListNode* node)` (0x30b270): append `node` to the list.
pub fn wscript_process_list_push_back(c: &mut Cpu) {
    c.cycles += 4;
    let node = c.g[A1][0];
    Addr(r32(c, A1)).off(0x18).sw(c, 0);
    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
    let empty = c.g[V1][0] == 0;
    if empty {
        c.cycles += 2;
        Addr(r32(c, A0)).sw(c, node);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1)).off(0x18).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[V1][0] = Addr(r32(c, V1)).off(0x18).lw(c);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, V1)).off(0x18).lw(c);
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, V1)).off(0x18).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_b290);
            }
        }
    }
    c.cycles += 2;
    Addr(r32(c, V1)).off(0x18).sw(c, node);
    abi::ret(c);
}


/// `int WScriptProcessList_Remove(WScriptProcessList* self,
/// cWScriptListNode* node)` (0x30b2d0): unlink `node`, 1 when it was found.
pub fn wscript_process_list_remove(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    if c.g[A1][0] != c.g[V0][0] {
        c.cycles += 2;
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if empty {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x18).lw(c);
            if c.g[V0][0] == c.g[A1][0] {
                c.g[V1][0] = Addr(r32(c, V0)).off(0x18).lw(c);
                c.cycles += 3;
                c.g[V0][0] = 1;
                Addr(r32(c, A0)).off(0x18).sw(c, c.g[V1][0]);
                abi::ret(c);
                return;
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[V0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_b300);
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A1)).off(0x18).lw(c);
    c.g[V0][0] = 1;
    Addr(r32(c, A0)).sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `void WScriptMan_ResetCollectSet(WScriptMan* self)` (0x30c390): reset the
/// collect-set bookkeeping and fill the 64 entries at `+0x0c..+0x108` with
/// -1.
pub fn wscript_man_reset_collect_set(c: &mut Cpu) {
    c.cycles += 10;
    let this = r32(c, A0);
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] |= 0xffff;
    Addr(this).off(4).sw(c, 0);
    Addr(this).sw(c, c.g[V0][0]);
    c.g[A1][0] |= 0xffff;
    Addr(this).off(8).sw(c, 0);
    c.g[V0][0] = sx(this.wrapping_add(0x108));
    c.g[V1][0] = 0x3f;
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, c.g[A1][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_c3b8);
        }
    }
    abi::ret(c);
}


/// `gp`-relative global (`lw(gp + off)` / `sw(gp + off, v)`).
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


/// `void WScriptMan_RunObjScript0(WScriptMan* self, int id)` (0x30a610): set
/// the running id, look the object script up in the script cache and run it
/// when it resolved.
pub fn wscript_man_run_obj_script0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(this, 0x290)).sw(c, c.g[V0][0]);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0030_a634);
        c.g[A0][0] = Addr(p(this, 0x28c)).lw(c);
        call_ptr(c, table::WORLD_GET_OBJECT_SCRIPT_FUNC, 0x0030_a634)?;
        c.g[V1][0] = !(0u64 | c.g[V0][0]);
        let found = c.g[V1][0] != 0;
        Addr(r32(c, SP)).sw(c, c.g[V0][0]);
        if found {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0030_a64c);
            c.g[A1][0] = c.g[SP][0];
            callee::WSCRIPT_MAN_CALL_FUNC.call(c, 0x0030_a64c)?;
        }
        c.cycles += 5;
        Addr(p(this, 0x290)).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptProcessList_DeleteAll(WScriptProcessList* self)`
/// (0x30b1b8): pop processes and delete each until the list is empty. The
/// loop head (0x30b1d0) is a resume entry: an event can unwind the thread
/// with the popped process still in `a2`.
pub fn wscript_process_list_delete_all(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        let list: u32;
        let mut pending: u64;
        // 0x30b1e4 pops the next process. The entry starts there (after the
        // prologue); the loop head 0x30b1d0 is entered with a process already
        // in `a2` (also the resume point of an unwound event).
        let mut need_pop;
        if c.pc == 0x0030_b1d0 {
            list = r32(c, S0);
            pending = c.g[A2][0];
            need_pop = false;
        } else if c.pc == 0x0030_b1b8 {
            c.cycles += 5;
            c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x20));
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            list = r32(c, A0);
            c.g[S0][0] = c.g[A0][0];
            pending = 0;
            need_pop = true;
        } else {
            c.jump_out(c.pc, 0x0030_b1b8, 0x0030_b208);
            return Ok(());
        }
        loop {
            if need_pop {
                // 0x30b1e4: pop the next process, then check for an event.
                c.cycles += 5;
                c.g[RA][0] = sx(0x0030_b1ec);
                c.g[A0][0] = sx(list);
                call_ptr(c, table::WSCRIPT_PROCESS_LIST_POP_FRONT, 0x0030_b1ec)?;
                c.g[A2][0] = c.g[V0][0];
                pending = c.g[V0][0];
                c.g[A1][0] = 0x3;
                if pending == 0 {
                    break;
                }
                if event_due(c) && crate::events::deliver_event(c, 0x0030_b1d0) {
                    return Ok(());
                }
                need_pop = false;
            }
            // 0x30b1d0: run the process's destructor through its vtable.
            c.cycles += 5;
            let obj = pending as u32;
            c.g[V0][0] = Addr(obj).off(0x5c).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0030_b1e4);
            c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0030_b1e4)?;
            need_pop = true;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void WScriptMan_RequestMissionStart(WScriptMan* self)` (0x30b540): stop
/// playback and the replay, then arm the mission-start request.
pub fn wscript_man_request_mission_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1)).off(0x84).lw(c);
        c.g[RA][0] = sx(0x0030_b560);
        c.g[A0][0] = Addr(r32(c, V0)).off(0x34).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b560)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
        c.g[RA][0] = sx(0x0030_b570);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x28).lw(c);
        call_ptr(c, 0x0026_f228, 0x0030_b570)?;
        c.g[V1][0] = Addr(p(this, 0x2a0)).lw(c);
        c.g[V0][0] = 0x1;
        Addr(p(this, 0x2ac)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x2b0)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Replay.

/// `void PlaybackMan_Stop(PlaybackMan* p_2)` (0x26cdf8): stop playback (state 1)
/// or recording (state 2); any other state is left alone.
pub fn playback_man_stop(c: &mut Cpu) {
    c.cycles += 6;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 1;
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p_2(c, A0, 0x70)).lw(c);
    let is1 = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
    if is1 {
        c.cycles += 4;
        if callee::PLAYBACK_MAN_STOP_PLAYBACK
            .call(c, 0x0026_ce34)
            .is_err()
        {
            return;
        }
    } else {
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            // States below 2 other than 1: nothing to do.
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let is2 = c.g[V1][0] == c.g[V0][0];
            if is2 {
                c.cycles += 3;
                if callee::PLAYBACK_MAN_STOP_RECORDING
                    .call(c, 0x0026_ce44)
                    .is_err()
                {
                    return;
                }
            } else {
                c.cycles += 2;
            }
        }
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void WScriptMan_DeactivateProcess(WScriptMan* self, void* node)`
/// (0x309AA0): move a node from the active list to the front of the pending
/// list, if it is active at all.
pub fn wscript_man_deactivate_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx(ea(c.g[S2][0], 0x2b8));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::WSCRIPT_PROCESS_LIST_CONTAINS.call(c, 0x0030_9ac8)?;
        let inactive = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !inactive {
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_REMOVE_2.call(c, 0x0030_9ad8)?;
            c.g[A0][0] = sx(ea(c.g[S2][0], 0x2b4));
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2.call(c, 0x0030_9ae4)?;
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


/// `void WScriptMan_RequestMissionRestart(WScriptMan* self)` (0x30b6f8):
/// reset the world/replay managers and arm the mission-restart request.
pub fn w_script_man_request_mission_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = lw_2(c, p_3(c.g[GP][0], -0x848i32 as u32));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p_3(c.g[V1][0], 0x84));
        c.g[A0][0] = lw_2(c, p_3(c.g[V0][0], 0x34));
        call_ptr(c, 0x0026_cdf8, 0x0030_b718)?;
        c.g[V0][0] = lw_2(c, p_3(c.g[GP][0], -0x848i32 as u32));
        c.g[V1][0] = lw_2(c, p_3(c.g[V0][0], 0x84));
        c.g[A0][0] = lw_2(c, p_3(c.g[V1][0], 0x28));
        call_ptr(c, 0x0026_f228, 0x0030_b728)?;
        c.g[V0][0] = lw_2(c, p_3(c.g[GP][0], -0x848i32 as u32));
        c.g[V1][0] = lw_2(c, p_3(c.g[V0][0], 0x84));
        c.g[A0][0] = lw_2(c, p_3(c.g[V1][0], 0x28));
        call_ptr(c, 0x0026_f7b8, 0x0030_b738)?;
        c.g[V1][0] = lw_2(c, p_3(c.g[S0][0], 0x2a0));
        c.g[V0][0] = 2;
        Addr(p_3(c.g[S0][0], 0x2ac)).sw(c, c.g[V0][0]);
        Addr(p_3(c.g[S0][0], 0x2b0)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void* WScriptProcessList_FindMissionById(WScriptProcessList* self, int id)`
/// (0x30b388).
pub fn wscript_process_list_find_mission_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.save(c, S1, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S0, 0x20);
        c.g[S1][0] = 0;
        let all = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let mut found = 0u64;
        let mut plus1 = true;
        if !all {
            c.cycles += 5;
            call_ptr(c, 0x0030_b320, 0x0030_b3b0)?;
            c.g[S0][0] = c.g[V0][0];
            let none = c.g[S0][0] == 0;
            c.g[V0][0] = c.g[S1][0];
            if none {
                plus1 = false;
            } else {
                c.cycles += 6;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x5c)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x18)).lh(c);
                c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x1c)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0030_b3d0)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = c.g[S0][0];
                }
                found = c.g[S1][0];
            }
        }
        if plus1 {
            c.cycles += 1;
        }
        c.g[V0][0] = found;
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `WScriptMan::DeactivateMission(self, node)` (0x309b00): move a still-active
/// node to the front of the process list and clear the current mission id.
pub fn w_script_man_deactivate_mission(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx(p(c.g[S2][0] as u32, 0x2b8));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0030_b428, 0x0030_9b28)?;
        let missing = c.g[V0][0] == 0;
        if !missing {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b2d0, 0x0030_9b3c)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(p(c.g[S2][0] as u32, 0x2b4));
            call_ptr(c, 0x0030_b260, 0x0030_9b48)?;
        }
        c.cycles += 9;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.restore(c, S1, 0x20);
        Addr(p(c.g[S2][0] as u32, 0x2a0)).sw(c, c.g[V0][0]);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_003506D8` (0x3506d8): the dead-node object constructor. Runs the
/// world object constructor (`WorldObj_ctor` with kind 8 / flags 6), installs
/// vtable `0x491b00` at `+0xc` and clears bits `0x60`/`0x2` of the node flags
/// word at `+0x8` of both the node and its script object while setting bit 2
/// (`0x4`) on the script object. Signature: `void ctor(this, a, b)`.
pub fn dead_node_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x8;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0035_06f8);
        c.g[A2][0] = 0x6;
        callee::WORLD_OBJ_CTOR_3.call(c, 0x0035_06f8)?;
        c.g[V1][0] = 0x49_0000;
        c.g[A2][0] = lw_r(c, S0, 0x18);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x1b00));
        c.g[A0][0] = 0xffff_ffff_ffff_0000;
        sw_r(c, S0, 0xc, c.g[V1][0]);
        c.g[A0][0] = c.g[A0][0] | 0xff9f;
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = lw_r(c, A2, 0x8);
        c.g[A1][0] = c.g[A1][0] | 0xfffd;
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        sw_r(c, A2, 0x8, c.g[V1][0]);
        c.g[A0][0] = lw_r(c, S0, 0x18);
        frame.restore(c, S0, 0x10);
        c.g[V1][0] = lw_r(c, A0, 0x8);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | 0x4;
        sw_r(c, A0, 0x8, c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


/// `void WScriptMan_DefineCollectSet(self, id, arg)` (0x30b928): look through
/// the collect-set slots at `+0x2c0` (stride 0x10c) for `id`; take a free
/// (`-1`) slot by resetting it, else skip.
pub fn wscript_man_define_collect_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.g[A3][0] = 0xffff_ffff_ffff_0000;
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.g[V1][0] = 0;
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = sx(low(c, A0).wrapping_add(0x2c0));
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        c.g[A3][0] = c.g[A3][0] | 0xffff;
        'outer: loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(low(c, S0)).lw(c);
            if c.g[V0][0] == c.g[S1][0] {
                c.lq(16, sp_2(c).wrapping_add(0x30));
                break 'outer;
            }
            c.cycles += 2;
            let free = c.g[V0][0] == c.g[A3][0];
            c.g[V1][0] = sx(low(c, V1).wrapping_add(1));
            if free {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::WSCRIPT_MAN_RESET_COLLECT_SET.call(c, 0x0030_b974)?;
                sw_r(c, S0, 0x4, c.g[S2][0]);
                sw_r(c, S0, 0, c.g[S1][0]);
                c.cycles += 1;
                c.lq(16, sp_2(c).wrapping_add(0x30));
                break 'outer;
            }
            c.cycles += 2;
            let more = (c.g[V1][0] as i64) <= 0;
            c.g[S0][0] = sx(low(c, S0).wrapping_add(0x10c));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_b958);
                }
                continue;
            }
            c.cycles += 1;
            c.lq(16, sp_2(c).wrapping_add(0x30));
            break;
        }
        c.cycles += 5;
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}


/// `void RiderModel_AllocModelRefs(self, count, unused)` (0x30d840): allocate
/// `count` 0x58-byte model references, zero them and store the block.
pub fn rider_model_alloc_model_refs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = c.g[A1][0];
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[A1][0] = 0x49_0000;
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = 0x58;
        c.sd(sp_2(c), c.g[RA][0]);
        c.mult(0, ZERO, V0, S1); // lo = count * 0x58
        sw_r(c, S0, 0x4, c.g[A2][0]);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(low(c, A1).wrapping_sub(0x6390));
        c.g[A0][0] = c.lo[0];
        sw_r(c, S0, 0, c.g[V0][0]);
        callee::OPERATOR_NEW_NAMED_2.call(c, 0x0030_d880)?;
        c.g[A2][0] = lw_r(c, S0, 0);
        c.g[A0][0] = c.g[V0][0];
        sw_r(c, S0, 0xc, c.g[V0][0]);
        c.mult(0, ZERO, A2, S1); // lo = count * 0x58
        c.g[A2][0] = c.lo[0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::MEMSET_3E6448, 0x0030_d89c)?;
        sw_r(c, S0, 0x60, 0);
        sw_r(c, S0, 0x8, 0);
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}


/// `void WScriptProcessList_dtor(self, flags)` (0x0030b068): tell every
/// process in the list to unlink itself, then free the list when asked to.
pub fn w_script_process_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = Addr(at_2(c, S1, 0)).lw(c);
        let empty = c.g[S0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 1;
            c.g[A2][0] = c.g[S0][0];
            loop {
                c.cycles += 2;
                let null = c.g[A2][0] == 0;
                c.g[S0][0] = Addr(at_2(c, S0, 0x18)).lw(c);
                if !null {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(at_2(c, A2, 0x5c)).lw(c);
                    c.g[A1][0] = 0x3;
                    c.g[A0][0] = Addr(at_2(c, V0, 0x8)).lh(c);
                    c.g[V1][0] = Addr(at_2(c, V0, 0xc)).lw(c);
                    c.g[A0][0] = add32(word(c, A2), word(c, A0));
                    call_ptr(c, word(c, V1), 0x0030_b0b0)?;
                }
                c.cycles += 2;
                let more = c.g[S0][0] != 0;
                c.g[A2][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_b090);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let keep = c.g[V0][0] == 0;
        c.lq(16, frame.at(c, 0x30));
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE.call(c, 0x0030_b0cc)?;
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `bool WScriptMan_TableHasFunction(WScriptMan* m, int table, UStr* name)`
/// (0x00309dd0): a script table slot with a function, whose registered name
/// is not the "special" marker.
pub fn w_script_man_table_has_function(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cc));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let have = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A2][0];
        if !have {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 15;
            c.g[V0][0] = 3;
            c.g[A0][0] = c.g[SP][0];
            c.sw(sp(c).wrapping_add(8), c.g[V0][0]);
            call_ptr(c, 0x0022_6620, 0x0030_9e10)?; // LunoVMRegister_AsId
            c.sw(c.g[V0][0] as u32, c.g[S1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            call_ptr(c, 0x0022_5248, 0x0030_9e20)?; // cLunoTable_Find
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 2;
            c.g[S0][0] = c.g[S0][0] ^ 5;
            c.g[S0][0] = (c.g[S0][0] < 1) as u64;
            call_ptr(c, 0x0022_5b90, 0x0030_9e38)?; // LunoVMRegister_dtor
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}


/// `int WScriptMan_CollectSetMarkItem(WScriptMan* self, int item)`
/// (0x30c3e0): find `item` in the collect-item table at `+0xc` and, when
/// found, mark it collected through the mission interface.
pub fn wscript_man_collect_set_mark_item(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        let this = c.g[S1][0];
        c.g[V0][0] = lw_at(c, this, 8);
        let n = c.g[V0][0] as i64;
        c.g[S0][0] = 0;
        let mut found = false;
        if n > 0 {
            c.cycles += 2;
            c.g[V1][0] = sx(at(this as u32, 0xc));
            loop {
                c.cycles += 3;
                let ent = c.g[V1][0] as u32;
                c.g[V0][0] = lw_at(c, ent as u64, 0);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[V0][0] = lw_at(c, this, 8);
                } else {
                    c.cycles += 12;
                    call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE_4, 0x0030_c41c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xa;
                    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE_2, 0x0030_c428)?;
                    let iface = c.g[V0][0];
                    c.g[A3][0] = lw_at(c, this, 0);
                    c.g[A0][0] = iface;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, callee::BE_MISSION_SET_COLLECTED, 0x0030_c43c)?;
                    c.lq(16, at(sp0, 0x20));
                    found = true;
                    break;
                }
                c.cycles += 4;
                c.g[S0][0] = sx(at(c.g[S0][0] as u32, 1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = sx(at(c.g[V1][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0030_c408);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}


/// `RiderModel* RiderModel_ctor(RiderModel* self)` (0x30d4b8): clear the
/// render fields and the four model-slot pointers at `+0x3c`, `+0x44`, ...
pub fn rider_model_ctor(c: &mut Cpu) {
    c.cycles += 24;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    open(c, 0x10);
    let sp0 = spv(c);
    let this = c.g[A0][0] as u32;
    c.sw(at(this, 0x38), 0);
    c.g[A1][0] = 3;
    c.swc1(0, at(sp0, 0xc));
    c.g[V1][0] = sx(at(this, 0x5c));
    c.swc1(0, sp0);
    c.swc1(0, at(sp0, 4));
    c.swc1(0, at(sp0, 8));
    c.sw(at(this, 0x30), 0);
    c.lq(2, sp0);
    c.sw(at(this, 0x34), 0);
    c.sq(2, at(this, 0x140));
    c.sw(at(this, 0x2c), 0);
    c.sw(at(this, 0xc), 0);
    c.sw(at(this, 0x1c), 0);
    c.sw(at(this, 0x3c), 0);
    c.sw(at(this, 8), 0);
    c.sw(at(this, 0x12c), 0);
    c.sw(at(this, 0x130), 0);
    c.sw(at(this, 0x24), 0);
    c.sw(at(this, 0x28), 0);
    c.sw(at(this, 0x20), 0);
    loop {
        c.cycles += 7;
        let slot = c.g[V1][0] as u32;
        c.sw(slot, 0);
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0xffff_ffff));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(at(slot, 0xffff_fff8));
        if !more {
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0];
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(sp0, 0x10));
            c.pc = ra;
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_d518);
        }
    }
}


/// `sp + off` (a frame slot).
#[inline(always)]
pub(crate) fn frame_at(c: &Cpu, off: u32) -> u32 {
    r32(c, SP).wrapping_add(off)
}


/// `lw_4` (sign-extended).
#[inline(always)]
pub(crate) fn lw_4(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}


/// `sw`.
#[inline(always)]
pub(crate) fn sw(c: &mut Cpu, a: u32, v: u64) {
    c.sw(a, v);
}


// ---------------------------------------------------------------------------
// WScript process manager.

/// `WScriptProcess* WScriptMan_CreateProcess(WScriptMan* self, char* name,
/// char* code, int flag)` (0x3094c8): allocate and construct a 0x60-byte
/// process, then push it on the mission list (`+0x2bc`) when `flag` is set,
/// else on the generic list (`+0x2b4`).
pub fn wscript_man_create_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x0049_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x60;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x66f8));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0030_9508)?; // EA_MemAllocNamed
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0030_7598, 0x0030_951c)?; // WScriptProcess_ctor
        let plain = c.g[S2][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !plain {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2bc));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b260, 0x0030_9530)?; // PushFront
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b260, 0x0030_9544)?;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void WScriptMan_RequestMissionEnd(WScriptMan* self, int rider)` (0x30b758)
/// (0x30b658 is the cancel variant): stop playback, clear its saved world
/// state, reset the replay to state 10, force-reset the rider and mark the
/// mission state as cancelled (3) or ended (4).
pub fn wscript_man_request_mission(c: &mut Cpu, state: u64) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = lw_4(c, gp(c, -0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x84));
        c.g[A0][0] = lw_4(c, p(c.g[V0][0] as u32, 0x34));
        call_ptr(c, 0x0026_cdf8, 0x0030_b678)?; // PlaybackMan_Stop
        c.g[V0][0] = lw_4(c, gp(c, -0x848));
        c.g[V1][0] = lw_4(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x34));
        call_ptr(c, 0x0026_cbb0, 0x0030_b688)?; // PlaybackMan_ClearWorldStateSaved
        c.g[V0][0] = lw_4(c, gp(c, -0x848));
        c.g[V1][0] = lw_4(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x28));
        call_ptr(c, 0x0026_f228, 0x0030_b698)?; // Replay_Reset
        c.g[V0][0] = lw_4(c, gp(c, -0x848));
        c.g[V1][0] = lw_4(c, p(c.g[V0][0] as u32, 0x84));
        c.g[A0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x28));
        call_ptr(c, 0x0026_f7b8, 0x0030_b6a8)?; // Replay_AdvanceFromState10
        c.g[V0][0] = lw_4(c, gp(c, -0x848));
        c.g[V1][0] = lw_4(c, c.g[S0][0] as u32);
        c.g[A0][0] = lw_4(c, p(c.g[V0][0] as u32, 0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = lw_4(c, p(c.g[A0][0] as u32, 0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = lw_4(c, p(c.g[V0][0] as u32, 0x28));
        call_ptr(c, 0x0012_35f8, 0x0030_b6c8)?; // Rider_ForceReset
        c.g[A0][0] = lw_4(c, p(c.g[S0][0] as u32, 0x2a0));
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V1][0] = state;
        c.g[V0][0] |= 0xffff;
        sw(c, p(c.g[S0][0] as u32, 0x2a0), c.g[V0][0]);
        sw(c, p(c.g[S0][0] as u32, 0x2ac), c.g[V1][0]);
        sw(c, p(c.g[S0][0] as u32, 0x2b0), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void WScriptMan_RequestMissionCancel(WScriptMan* self, int rider)`
/// (0x30b658).
pub fn wscript_man_request_mission_cancel(c: &mut Cpu) {
    wscript_man_request_mission(c, 3)
}


/// `void WScriptMan_RequestMissionEnd(WScriptMan* self, int rider)` (0x30b758).
pub fn wscript_man_request_mission_end(c: &mut Cpu) {
    wscript_man_request_mission(c, 4)
}


/// `void WScriptMan_DeactivateAll(WScriptMan* self)` (0x30b7f8): terminate the
/// current mission process, clear the active fields, move every queued process
/// back to the idle list and reset the event queue.
pub fn wscript_man_deactivate_all(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0030_b7f8 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, frame_at(c, 0x30));
                    c.sq(17, frame_at(c, 0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, frame_at(c, 0x10));
                    c.g[S1][0] = 0xffff_ffff_ffff_0000;
                    c.sd(frame_at(c, 0), c.g[RA][0]);
                    c.g[S1][0] |= 0xffff;
                    c.g[A1][0] = lw_4(c, p(c.g[S0][0] as u32, 0x2a0));
                    if c.g[A1][0] == c.g[S1][0] {
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
                        pc = 0x0030_b850;
                        continue;
                    }
                    pc = 0x0030_b824;
                    continue;
                }
                0x0030_b824 => {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
                    call_ptr(c, 0x0030_b388, 0x0030_b82c)?; // FindMissionById
                    if c.g[V0][0] == 0 {
                        sw(c, p(c.g[S0][0] as u32, 0x2a0), c.g[S1][0]);
                        pc = 0x0030_b840;
                        continue;
                    }
                    pc = 0x0030_b834;
                    continue;
                }
                0x0030_b834 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0030_7d78, 0x0030_b83c)?; // WScriptProcess_Terminate
                    sw(c, p(c.g[S0][0] as u32, 0x2a0), c.g[S1][0]);
                    pc = 0x0030_b840;
                    continue;
                }
                0x0030_b840 => {
                    c.cycles += 4;
                    sw(c, p(c.g[S0][0] as u32, 0x2ac), 0);
                    sw(c, p(c.g[S0][0] as u32, 0x2b0), c.g[S1][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b8));
                    pc = 0x0030_b850;
                    continue;
                }
                0x0030_b850 => {
                    c.cycles += 2;
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
                    pc = 0x0030_b864;
                    continue;
                }
                0x0030_b858 => {
                    c.cycles += 3;
                    sw(c, c.g[V0][0] as u32, 0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
                    call_ptr(c, 0x0030_b260, 0x0030_b864)?; // PushFront
                    pc = 0x0030_b864;
                    continue;
                }
                0x0030_b864 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0030_b208, 0x0030_b86c)?; // PopFront
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[V0][0] != 0 {
                        if event_due(c) {
                            c.pc = 0x0030_b858;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0030_b858;
                        continue;
                    }
                    pc = 0x0030_b874;
                    continue;
                }
                0x0030_b874 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0030_c790, 0x0030_b87c)?; // EventQueueReset
                    c.lq(16, frame_at(c, 0x30));
                    c.lq(17, frame_at(c, 0x20));
                    c.lq(18, frame_at(c, 0x10));
                    c.g[RA][0] = c.ld(frame_at(c, 0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030_b7f8, 0x0030_b894);
                    return Ok(());
                }
            }
        }
    });
}


// ---------------------------------------------------------------------------
// `WScriptMan_CreateMission` (0x309568): allocate and construct a mission
// script, push it on the front or back process list, and return it.
pub fn wscript_man_create_mission(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x10);
        c.g[S4][0] = c.g[A0][0];
        c.g[A1][0] = 0x490000;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x66e8));
        c.g[A0][0] = 0x78;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x0030_95b0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        callee::WSCRIPT_MISSION_CTOR.call(c, 0x0030_95c8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S3][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2bc));
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_3.call(c, 0x0030_95dc)?;
        } else {
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S0][0];
            callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_3.call(c, 0x0030_95f0)?;
        }
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 8;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `WScriptMan_CallFuncOnInstance` (0x309d20): call a script function on the
/// world section's instance by name, with an empty argument and result value.
/// Not listed: it needs a live Luno VM to run, which the test fixtures cannot
/// provide cheaply.
pub fn wscript_man_call_func_on_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x30);
        frame.save(c, S4, 0x40);
        c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x290));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28c));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x290), c.g[A2][0]);
        call_ptr(c, 0x003a_6d18, 0x0030_9d54)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x0022_6600, 0x0030_9d64)?;
        c.g[V0][0] = c.lbu((c.g[S1][0] as u32));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3cc));
        call_ptr(c, 0x0022_24a8, 0x0030_9d88)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0022_24b8, 0x0030_9da0)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x290), c.g[S4][0]);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0022_5b90, 0x0030_9db0)?;
        c.lq(16, frame.at(c, 0x80));
        c.lq(17, frame.at(c, 0x70));
        c.lq(18, frame.at(c, 0x60));
        c.lq(19, frame.at(c, 0x50));
        c.lq(20, frame.at(c, 0x40));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}


/// `WScriptMan_CallTableFunction(this, idx, value)` (0x309e50): marshal the
/// value into a script argument, look the scripted function up in the table
/// at +0x3cc, call it and destroy the temporaries.
pub fn wscript_man_call_table_function(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        let value = abi::arg(c, 2);
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 3;
        frame.save(c, 17, 0x70);
        frame.save(c, 16, 0x80);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x60);
        c.g[A0][0] = sp(c) as u64;
        frame.save(c, 19, 0x50);
        c.g[S2][0] = abi::arg64(c, 2);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = abi::arg64(c, 1);
        Addr(sp(c)).off(8).sw(c, c.g[V0][0]);

        // key = LunoVMRegister_AsId(&key); *key = value
        callee::LUNO_VM_REGISTER_AS_ID.call(c, 0x0030_9e84)?;
        Addr(abi::r32(c, V0)).sw(c, c.g[S2][0]);

        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[S1][0] = sx(this.wrapping_add(0x3cc));
        c.g[A1][0] = sp(c) as u64;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        callee::LUNO_TABLE_FIND.call(c, 0x0030_9ea0)?;

        // Copy the found argument onto the stack (an unaligned doubleword).
        c.ldl(3, abi::r32(c, V0).wrapping_add(7));
        c.ldr(3, abi::r32(c, V0));
        c.sdl(3, sp(c).wrapping_add(0x17));
        c.sdr(3, sp(c).wrapping_add(0x10));
        c.g[S2][0] = sx(sp(c).wrapping_add(0x30));
        c.g[S3][0] = sx(sp(c).wrapping_add(0x10));
        Addr(sp(c)).off(0x28).sw(c, 0);
        Addr(sp(c)).off(0x20).sw(c, 0);

        c.g[A0][0] = c.g[S2][0];
        callee::LUNO_VM_PASS_THROUGH.call(c, 0x0030_9ec8)?;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x20));
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S3][0];
        callee::LUNO_VM_CALL.call(c, 0x0030_9ee0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0030_9eec)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0030_9ef8)?;

        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptProcessList_DeleteBySection(WScriptProcessList *list,
/// int section)` (0x30b0e8): unlink and destroy every process whose section
/// matches, stopping at the first non-matching node. Re-entry points at
/// 0x30b118 and 0x30b168.
pub fn wscript_process_list_delete_by_section(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        // Opened at the entry arm, reused by the resume arms.
        let mut frame = Frame { size: 0x40 };
        loop {
            match pc {
                0x0030_b0e8 => {
                    c.cycles += 9;
                    frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    frame.save_ra(c, 0);
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[S2][0] = c.g[A1][0];
                    if empty {
                        pc = 0x0030_b19c;
                    } else {
                        pc = 0x0030_b10c;
                    }
                }
                0x0030_b10c => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x24)).lw(c);
                    let other = c.g[V0][0] != c.g[S2][0];
                    c.g[S0][0] = Addr(p_4(c.g[V1][0], 0x18)).lw(c);
                    if other {
                        pc = 0x0030_b150;
                    } else {
                        pc = 0x0030_b118;
                    }
                }
                0x0030_b118 => {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if none {
                        pc = 0x0030_b138;
                    } else {
                        pc = 0x0030_b124;
                    }
                }
                0x0030_b124 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_4(c.g[A2][0], 0x5c)).lw(c);
                    c.g[A0][0] = Addr(p_4(c.g[V0][0], 0x8)).lh(c);
                    c.g[V1][0] = Addr(p_4(c.g[V0][0], 0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_b138)?;
                    pc = 0x0030_b138;
                }
                0x0030_b138 => {
                    c.cycles += 3;
                    Addr(c.g[S1][0] as u32).sw(c, c.g[S0][0]);
                    let empty = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S0][0];
                    if empty {
                        pc = 0x0030_b19c;
                    } else {
                        pc = 0x0030_b144;
                    }
                }
                0x0030_b144 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_4(c.g[V0][0], 0x24)).lw(c);
                    let matches = c.g[V0][0] == c.g[S2][0];
                    c.g[S0][0] = Addr(p_4(c.g[S0][0], 0x18)).lw(c);
                    if matches {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0030_b118;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0030_b118;
                    } else {
                        pc = 0x0030_b150;
                    }
                }
                0x0030_b150 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let empty = c.g[A0][0] == 0;
                    c.lq(16, frame.at(c, 0x30));
                    if empty {
                        pc = 0x0030_b1a0;
                    } else {
                        pc = 0x0030_b15c;
                    }
                }
                0x0030_b15c => {
                    c.cycles += 2;
                    c.g[S0][0] = Addr(p_4(c.g[A0][0], 0x18)).lw(c);
                    pc = 0x0030_b184;
                }
                0x0030_b164 => {
                    c.cycles += 1;
                    pc = 0x0030_b168;
                }
                0x0030_b168 => {
                    c.cycles += 2;
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if none {
                        pc = 0x0030_b184;
                    } else {
                        pc = 0x0030_b170;
                    }
                }
                0x0030_b170 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_4(c.g[A2][0], 0x5c)).lw(c);
                    c.g[A0][0] = Addr(p_4(c.g[V0][0], 0x8)).lh(c);
                    c.g[V1][0] = Addr(p_4(c.g[V0][0], 0xc)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0030_b184)?;
                    pc = 0x0030_b184;
                }
                0x0030_b184 => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.lq(16, frame.at(c, 0x30));
                        pc = 0x0030_b1a0;
                    } else {
                        pc = 0x0030_b18c;
                    }
                }
                0x0030_b18c => {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[V0][0] = Addr(p_4(c.g[S0][0], 0x24)).lw(c);
                    let matches = c.g[V0][0] == c.g[S2][0];
                    c.g[S0][0] = Addr(p_4(c.g[A2][0], 0x18)).lw(c);
                    if matches {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0030_b168;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0030_b168;
                    } else {
                        pc = 0x0030_b19c;
                    }
                }
                0x0030_b19c => {
                    c.cycles += 1;
                    c.lq(16, frame.at(c, 0x30));
                    pc = 0x0030_b1a0;
                }
                0x0030_b1a0 => {
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030_b0e8, 0x0030_b1b4);
                    return Ok(());
                }
            }
        }
    });
}


// ---------------------------------------------------------------------------
// WScript.

/// `void WScriptMan_LoadProcessList(WScriptMan* self, WScriptList* list,
/// Stream* stream)` (0x30bee8): read a count and that many process ids from
/// the stream, move each process from the old list state into the new one.
pub fn wscriptman_load_process_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[A2][0] = 4;
        frame.save(c, S4, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[S2][0] = 0;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let read = r32(c, V1);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, read, 0x0030_bf30)?;
        c.g[V0][0] = Addr(r32(c, SP)).lw(c);
        c.g[S3][0] = sx(p(r32(c, S0), 0x2b4));
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 8;
            c.lq(S0, frame.at(c, 0x60));
            frame.restore(c, S1, 0x50);
            frame.restore(c, S2, 0x40);
            frame.restore(c, S3, 0x30);
            frame.restore(c, S4, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        loop {
            c.cycles += 24;
            c.g[A1][0] = sx(p(r32(c, SP), 4));
            c.g[A2][0] = 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
            let read = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, read, 0x0030_bf5c)?;
            c.g[A1][0] = Addr(p(r32(c, SP), 4)).lw(c);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0030_b320, 0x0030_bf68)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0030_b2d0, 0x0030_bf78)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_8118, 0x0030_bf84)?;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0030_b270, 0x0030_bf90)?;
            c.g[V0][0] = Addr(r32(c, SP)).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0030_bf40);
            }
        }
        c.cycles += 8;
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


// ---------------------------------------------------------------------------
// WScriptMan_GetHudTargetHeight (0x0000309ba8).

/// `WScriptMan_GetHudTargetHeight`: return the HUD target height for the current script target.
pub fn w_script_man_get_hud_target_height(c: &mut Cpu) {
    guest(c, |c| {
    'b_309c7c: {
    'b_309c74: {
    c.cycles += 7;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.g[A1][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28));
    let t = c.g[V0][0] == 0u64;

    if t {
    break 'b_309c74;
    } else {
    'b_309c18: {
    'b_309bf4: {
    c.cycles += 10;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28c));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    if c.g[V1][0] != 0u64 {
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
    break 'b_309bf4;
    } else {
    c.cycles += 2;
    let t = true;
    c.g[S0][0] = 0u64;
    if t {
    break 'b_309c18;
    } else {
    break 'b_309bf4;
    }
    }
    }
    'b_309c14: {
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    let t = c.g[V0][0] != 0u64;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if t {
    break 'b_309c14;
    } else {
    c.cycles += 1;
    c.g[V0][0] = 0u64;
    break 'b_309c14;
    }
    }
    c.cycles += 1;
    c.g[S0][0] = c.g[V0][0];
    break 'b_309c18;
    }
    c.cycles += 23;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
    c.g[RA][0] = sx(0x00309c4c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x00309c4c { return Err(crate::call::Unwound); }
    c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.sq(3, (c.g[SP][0] as u32));
    c.mtc1(0, 2);
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.sub_s(0, 0, 1);
    c.c_le_s(2, 0);
    let t = c.fcc();
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    if t {
    break 'b_309c7c;
    } else {
    break 'b_309c74;
    }
    }
    }
    c.cycles += 2;
    c.mtc1(0, 0);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    break 'b_309c7c;
    }
    c.cycles += 3;
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// WScriptMan_CollectItem (0x000030b9a0).

/// `WScriptMan_CollectItem`: collect a world script item: add it to the inventory and fire its events.
pub fn w_script_man_collect_item(c: &mut Cpu) {
    guest(c, |c| {
    c.cycles += 8;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c0));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = 0u64;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S2][0] = sx((c.g[A1][0] as u32) << 2);
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    'l_30b9c0: loop {
    'b_30ba6c: {
    'b_30ba60: {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
    let t = c.g[V0][0] != c.g[A2][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    if t {
    break 'b_30ba60;
    } else {
    c.cycles += 37;
    c.g[A1][0] = c.g[A3][0];
    call_ptr(c, 0x0030c3e0, 0x0030b9d4)?;
    call_ptr(c, 0x0028b180, 0x0030b9dc)?;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 12);
    c.g[A2][0] = 0u64;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x5;
    call_ptr(c, 0x0029ced8, 0x0030b9f4)?;
    call_ptr(c, 0x0014dc80, 0x0030b9fc)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0014dd58, 0x0030ba08)?;
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, 0x0014dc80, 0x0030ba10)?;
    c.g[A1][0] = 0xb;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014dd58, 0x0030ba1c)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[S1][0] = c.g[V0][0];
    call_ptr(c, 0x00144c98, 0x0030ba28)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x00151178, 0x0030ba38)?;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    call_ptr(c, 0x0010f338, 0x0030ba58)?;
    let t = true;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    if t {
    break 'b_30ba6c;
    } else {
    break 'b_30ba60;
    }
    }
    }
    c.cycles += 2;
    let t = (c.g[V1][0] as i64) <= 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10c));
    if t {
    if event_due(c) { deliver_event_here(c, 0x0030b9c0); } continue 'l_30b9c0;
    } else {
    c.cycles += 1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    break 'b_30ba6c;
    }
    }
    c.cycles += 5;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
    }
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::put_words;
    use crate::difftest::fakes::ptr;
    use ssx3_ee::s01;
    use ssx3_ee::s03;
    use ssx3_ee::s00;
    use ssx3_ee::s02;
    use ssx3_ee::s08;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use crate::difftest::fakes::vtable;
    use ssx3_ee as ee;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as RET;
    use crate::difftest::elf_image;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::SP;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::ptr as ptr_2;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::world::world::tests::WORLD;
    pub(crate) use crate::game::world::world::tests::TABLE;
    pub(crate) use crate::game::world::world::tests::SLOT1;
    pub(crate) use crate::game::world::world::tests::SLOT2;
    pub(crate) use crate::game::world::world::tests::BASE;
    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::world::world::tests::put_script_cache;
    pub(crate) use crate::game::world::world::tests::put_object_slots;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::worldpainter::quadtree::tests::put_vtable;
    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4;
    pub(crate) use crate::game::sound::bankmanager::tests::be_library;
    pub(crate) use crate::game::sound::bankmanager::tests::be_library_interface;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_7;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::EV;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_9;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;
    pub(crate) use crate::game::movie::movieplayer::tests::wh;
    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_6 as be_library_4;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_4;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_2;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_7;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_5;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_6;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_4;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_2;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;


    #[test]
    fn wscript_process_list_pop_front_matches() {
        check(s10::f_0030b208__generated, wscript_process_list_pop_front, SEEDS, &|c, seed| {
            let mut r = rng(seed, 4);
            scramble(c, &mut r);
            let list = ptr_2(&mut r, 0);
            let node = ptr_2(&mut r, 1);
            c.g[A0][0] = sx(list);
            let head = if seed % 2 == 0 { 0 } else { node };
            c.bus.write32(list, head);
            c.bus.write32(node + 0x18, r.next_u32());
        });
    }


    #[test]
    fn wscript_process_list_push_front_matches() {
        check(s10::f_0030b260__generated, wscript_process_list_push_front, SEEDS, &|c, seed| {
            let mut r = rng(seed, 5);
            scramble(c, &mut r);
            let list = ptr_2(&mut r, 0);
            let node = ptr_2(&mut r, 1);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(node);
            c.bus.write32(list, if seed % 2 == 0 { 0 } else { r.next_u32() });
        });
    }


    #[test]
    fn wscript_man_post_trigger_event_matches() {
        check_lohi(ssx3_ee::s10::f_003097e0, wscript_man_post_trigger_event, 200, &|c, seed| {
            let mut r = Rng(0x2002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }


    #[test]
    fn wscript_man_post_jump_event_matches() {
        check_lohi(ssx3_ee::s10::f_00309848, wscript_man_post_jump_event, 200, &|c, seed| {
            let mut r = Rng(0x2003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }


    #[test]
    fn wscript_man_post_trick_event_matches() {
        check_lohi(ssx3_ee::s10::f_00309918, wscript_man_post_trick_event, 200, &|c, seed| {
            let mut r = Rng(0x2004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
            for i in 0..2 {
                c.bus.write32(DATA + 0x200 + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn wscript_man_post_wipeout_event_matches() {
        check_lohi(ssx3_ee::s10::f_00309990, wscript_man_post_wipeout_event, 200, &|c, seed| {
            let mut r = Rng(0x2005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }


    #[test]
    fn wscript_man_post_reset_event_matches() {
        check_lohi(ssx3_ee::s10::f_003099f8, wscript_man_post_reset_event, 200, &|c, seed| {
            let mut r = Rng(0x2006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x40, seed % 4);
        });
    }


    #[test]
    fn wscript_process_list_ctor_matches() {
        check(ssx3_ee::s10::f_0030b058, wscript_process_list_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
        });
    }


    #[test]
    fn wscript_process_list_push_front_2_matches() {
        check(ssx3_ee::s10::f_0030b2c0, wscript_process_list_push_front_2, 150, &|c, seed| {
            let mut r = Rng(0x2008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 0x100 + 0x18, r.next_u32());
        });
    }


    #[test]
    fn wscript_man_event_queue_push_matches() {
        check_lohi(ssx3_ee::s10::f_0030c7c0, wscript_man_event_queue_push, 250, &|c, seed| {
            let mut r = Rng(0x2009 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.bus.write32(DATA + 4, (seed % 20) as u32);
        });
    }


    #[test]
    fn wscript_man_is_scripted_instance_matches() {
        check(ee::s10::f_0030a310__generated, wscript_man_is_scripted_instance, SEEDS_2, &|c, seed| {
            let _r = base(c, seed, 0x30a3);
            let (man, cache, obj) = (DATA, DATA + 0x100, DATA + 0x140);
            let id = (seed % 3) as u8;
            let kind = if seed % 8 == 1 { 5 } else { 3 };
            put_script_cache(c, man, cache, obj, id, kind);
            let slot = TABLE + 12 * id as u32;
            c.bus.write32(obj + 0xc, if seed % 8 == 0 { DATA + 0x700 } else { 0 });
            match seed % 8 {
                2 => c.bus.write32(obj + 0x88, 0),
                3 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0);
                }
                4 => {
                    c.bus.write32(obj + 0x88, SLOT1);
                    c.bus.write32(SLOT1 + 8, 0xffff_ffff);
                    c.bus.write32(slot + 8, SLOT2);
                    c.bus.write32(SLOT2 + 0x1c, BASE);
                }
                5 => put_object_slots(c, obj, slot, 0, 0x55),
                6 => put_object_slots(c, obj, slot, 7, 0),
                _ => put_object_slots(c, obj, slot, 7, 9),
            }
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(obj);
        });
    }


    #[test]
    fn wscript_man_has_obj_script1_matches() {
        check(ssx3_ee::s10::f_0030a270__generated, wscript_man_has_obj_script1, 200, &|c, seed| {
            machine(c, seed, 0x30a2_70, 0);
            let manager = DATA;
            let table = DATA + 0x400;
            let obj = DATA + 0x800;
            c.g[A0][0] = sx(manager);
            c.g[A1][0] = sx(obj);
            c.bus.write32(manager + 0x28c, table);
            c.bus.write8(obj + 0x78, 0);
            c.bus.write32(table + 4, DATA + 0x600);
            c.bus.write32(DATA + 0x604, DATA + 0xc00);
            if seed % 2 == 0 {
                // The object has no script cache entry: the lookup is -1.
                c.bus.write32(obj + 0x88, 0);
            } else {
                // A resident entry: the slot 1 script id comes back.
                c.bus.write32(obj + 0x88, DATA + 0x1000);
                c.bus.write32(DATA + 0x1008, 0x800);
                c.bus.write32(DATA + 0xc08, DATA + 0x1200);
                c.bus.write32(DATA + 0x121c, DATA + 0x1400);
                c.bus.write32(DATA + 0x140c, 0x1234_5678);
            }
        });
    }


    #[test]
    fn wscript_man_set_pending_rider_config_matches() {
        check(ee::s10::f_0030b520__generated, wscript_man_set_pending_rider_config, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x30_b520, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x801) as u64; // deliberately unaligned
            for i in 0..3u32 {
                c.bus.write32(DATA + 0x800 + 4 * i, r.next_u32());
            }
            c.bus.write32(DATA + 0x34, r.next_u32());
        });
    }


    #[test]
    fn wscript_man_has_kill_script_matches() {
        check(s10::f_0030a598__generated, wscript_man_has_kill_script, 120, &|c, seed| {
            machine(c, seed, 0x30a598, 1);
            globals(c);
            c.g[A0][0] = sx(OBJ);
            c.g[A1][0] = sx(CHILD + 0x80);
            clear(c, OBJ, 0x80);
            clear(c, CHILD, 0x80);
            // The script cache object (`+0x28c`) is null: the lookup falls
            // through to the id-only path.
            c.bus.write32(OBJ + 0x28c, 0);
            let _ = seed;
        });
    }


    #[test]
    fn wscript_man_jump_to_task_matches() {
        check(s10::f_0030a6d8__generated, wscript_man_jump_to_task, 120, &|c, seed| {
            machine(c, seed, 0x30a6d8, 1);
            c.g[A1][0] = 0;
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            // No process / a process with no commands (the callee loops over
            // the command count at +0x4).
            let proc = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x2a4, proc);
        });
    }


    #[test]
    fn wscript_man_goto_task_matches() {
        check(s10::f_0030a700__generated, wscript_man_goto_task, 120, &|c, seed| {
            machine(c, seed, 0x30a700, 1);
            c.g[A1][0] = 0;
            clear(c, OBJ, 0x40);
            clear(c, CHILD, 0x100);
            c.g[A0][0] = sx(OBJ);
            let proc = if seed % 3 == 0 { 0 } else { CHILD };
            c.bus.write32(OBJ + 0x2a4, proc);
        });
    }


    #[test]
    fn wscript_man_cmd_queue_ctor_matches() {
        check(s10::f_0030c738__generated, wscript_man_cmd_queue_ctor, 120, &|c, seed| {
            machine(c, seed, 0x30c738, 1);
            clear(c, OBJ, 0x40);
            c.g[A0][0] = sx(OBJ);
            c.bus.write32(OBJ, seed as u32 & 0xff);
        });
    }


    #[test]
    fn wscript_process_list_find_by_id_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x30b320, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 4 == 0 {
                c.bus.write32(DATA, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            let n2 = DATA + 0x180;
            c.bus.write32(DATA, n0);
            c.bus.write32(n0 + 0x18, n1);
            c.bus.write32(n1 + 0x18, n2);
            c.bus.write32(n2 + 0x18, 0);
            c.bus.write32(n0 + 0x20, 10);
            c.bus.write32(n1 + 0x20, 11);
            c.bus.write32(n2 + 0x20, 12);
            c.g[A1][0] = (8 + seed % 7) as u64;
            c.bus.write32(DATA + 0x1f0, r.next_u32());
        };
        check(ee::s10::f_0030b320__generated, wscript_process_list_find_by_id, SEEDS_3, &setup);
    }


    #[test]
    fn wscript_process_list_contains_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x30b428, 0);
            c.g[A0][0] = DATA as u64;
            if seed % 5 == 0 {
                c.bus.write32(DATA, 0);
                return;
            }
            let n0 = DATA + 0x100;
            let n1 = DATA + 0x140;
            c.bus.write32(DATA, n0);
            c.bus.write32(n0 + 0x18, n1);
            c.bus.write32(n1 + 0x18, 0);
            c.bus.write32(n0 + 0x20, r.next_u32());
            c.bus.write32(n1 + 0x20, r.next_u32());
            c.g[A1][0] = match seed % 4 {
                0 => n0,
                1 => n1,
                2 => DATA + 0x200,
                _ => 0,
            } as u64;
        };
        check(ee::s10::f_0030b428__generated, wscript_process_list_contains, SEEDS_3, &setup);
    }


    #[test]
    fn wscript_man_reset_command_queue_matches() {
        check(ee::s10::f_0030c760__generated, wscript_man_reset_command_queue, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x30c760, 0);
            c.g[A0][0] = DATA as u64;
            for i in 0..0x40u32 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn wscript_man_event_queue_reset_matches() {
        check(ee::s10::f_0030c790__generated, wscript_man_event_queue_reset, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x30c790, 0);
            c.g[A0][0] = DATA as u64;
            for i in 0..0x40u32 {
                c.bus.write32(DATA + 4 * i, r.next_u32());
            }
        });
    }


    // ------------------------------------------------------- destructors (batch)

    #[test]
    fn wscript_man_sub_obj_dtor_d420_matches() {
        dtor_case(ee::s10::f_0030d420__generated, wscript_man_sub_obj_dtor_d420, 0x30d420);
    }


    #[test]
    fn wscript_man_collect_set_load_matches() {
        check(ee::s10::f_0030c700__generated, wscript_man_collect_set_load, 120, &|c, seed| {
            forward_setup(c, seed, 0x30c7_00, 0x10);
        });
    }


    #[test]
    fn wscript_man_activate_process_matches() {
        check(
            ee::s10::f_00309a60__generated,
            wscript_man_activate_process,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x309a60, 0);
                let self_at = DATA + 0x200;
                let node = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = node as u64;
                c.bus.write32(node, (seed % 3) as u32);
                c.bus
                    .write32(node + 4, if seed % 2 == 0 { 0 } else { node });
            },
        );
    }


    #[test]
    fn wscript_man_collect_set_get_count_matches() {
        check(
            ee::s10::f_0030c468__generated,
            wscript_man_collect_set_get_count,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30c468, 0);
                let lib = DATA + 0x800;
                let obj = DATA + 0x900;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(GAME_GP - 0x1f04, lib); // BELibrary singleton
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, obj);
                c.bus.write32(obj + 4, vt);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x200, (seed % 20) as u32);
                c.bus.write32(GAME_GP + 0x8d8, 0);
            },
        );
    }


    #[test]
    fn wscript_man_get_collected_count_matches() {
        check(
            ee::s10::f_0030ba80__generated,
            wscript_man_get_collected_count,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_ba80, 0);
                // Zeroed BE library: no disc load when the script manager
                // reaches the behaviour-library singleton.
                c.bus.write32(crate::difftest::GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
                let m = DATA + 0x1000;
                let id = (r.next_u32() % 3) as u32;
                c.bus
                    .write32(m + 0x2c0, if seed % 2 == 0 { id } else { id ^ 1 });
                c.bus.write32(m + 0x2c0 + 0x10c, r.next_u32());
                c.g[A0][0] = sx(m);
                c.g[A1][0] = id as u64;
            },
        );
    }


    #[test]
    fn wscript_man_add_collect_items_matches() {
        covered(
            ee::s10::f_0030bac8__generated,
            wscript_man_add_collect_items,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_bac8, 1);
                be_library_interface(c, &mut r);
                let m = DATA + 0x1000;
                let id = (r.next_u32() % 3) as u32;
                c.bus
                    .write32(m + 0x2c0, if seed % 2 == 0 { id } else { id ^ 1 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = id as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // A non-positive item count makes the collect-set routine bail out
                // before touching the (absent) item table.
                c.g[A3][0] = 0;
            },
        );
    }


    // -----------------------------------------------------------------------
    // 0x309750 WScriptMan_StartProcess / 0x309798 WScriptMan_StopProcess.

    #[test]
    fn w_script_man_start_process_matches() {
        check(ee::s10::f_00309750__generated, w_script_man_start_process, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3097_50, 1);
            let man = DATA + 0x100;
            let process = DATA + 0x300;
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(process);
            c.bus.write32(man + 0x2a4, ptr(&mut r));
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(process + 0x5c, vt);
        });
    }


    #[test]
    fn w_script_man_stop_process_matches() {
        check(ee::s10::f_00309798__generated, w_script_man_stop_process, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3097_98, 1);
            let man = DATA + 0x100;
            let process = DATA + 0x300;
            c.g[A0][0] = sx(man);
            c.g[A1][0] = sx(process);
            c.bus.write32(man + 0x2a4, ptr(&mut r));
            let vt = vtable(c, &mut r, &[0x30]);
            c.bus.write32(process + 0x5c, vt);
        });
    }


    // ------------------------------------------------------------------
    // WScript.

    #[test]
    fn wscript_process_terminate_matches() {
        check(
            ee::s10::f_00307d78__generated,
            wscript_process_terminate,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x307d_78, 2);
                let this = DATA + 0x100;
                clear_2(c, this, 0x40);
                c.bus.write32(this + 0x10, DATA + 0x400);
                clear_2(c, DATA + 0x400, 0x60);
                c.bus.write32(this + 0x1c, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
                clear_2(c, DATA + 0x500, 0x20);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn wscript_man_get_current_mission_matches() {
        check(
            ee::s10::f_0030aeb8__generated,
            wscript_man_get_current_mission,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30ae_b8, 2);
                let this = DATA + 0x100;
                clear_2(c, this, 0x300);
                c.g[A0][0] = sx(this);
                let mission = DATA + 0x400;
                clear_2(c, mission, 0x80);
                c.bus.write32(this + 0x2a4, if seed % 2 == 0 { 0 } else { mission });
                // The mission's vtable slot: `lh(+0x18)` delta, `lw(+0x1c)` fn.
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mission + 0x5c, vt);
                c.bus.write32(LOG_RESULTS, if seed % 4 < 2 { 0x1234 } else { 0 });
            },
        );
    }


    #[test]
    fn wscript_process_list_push_back_matches() {
        check(
            ee::s10::f_0030b270__generated,
            wscript_process_list_push_back,
            SEEDS_5,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x30b2_70);
                let list = DATA + 0x100;
                clear_2(c, list, 0x40);
                // A chain of 0..3 nodes at +0x18.
                let n = seed % 4;
                let mut prev = 0;
                let mut first = 0;
                for i in 0..n {
                    let node = DATA + 0x200 + 0x20 * i;
                    clear_2(c, node, 8);
                    if i == 0 {
                        first = node;
                    } else {
                        c.bus.write32(prev + 0x18, node);
                    }
                    prev = node;
                }
                c.bus.write32(list, first);
                let node = DATA + 0x500;
                clear_2(c, node, 8);
                c.bus.write32(node + 0x18, r.next_u32());
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(node);
            },
        );
    }


    #[test]
    fn wscript_process_list_remove_matches() {
        check(
            ee::s10::f_0030b2d0__generated,
            wscript_process_list_remove,
            SEEDS_5,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x30b2_d0);
                let list = DATA + 0x100;
                clear_2(c, list, 0x40);
                // A chain of 0..3 nodes; remove the head, one of the others,
                // an absent node, or from an empty list.
                let n = seed % 4;
                let mut nodes = Vec::new();
                let mut prev = 0;
                for i in 0..n {
                    let node = DATA + 0x200 + 0x20 * i;
                    clear_2(c, node, 8);
                    if i == 0 {
                        c.bus.write32(list, node);
                    } else {
                        c.bus.write32(prev + 0x18, node);
                    }
                    prev = node;
                    nodes.push(node);
                }
                let victim = match seed % 5 {
                    0 => nodes.first().copied().unwrap_or(DATA + 0x600),
                    1 => nodes.last().copied().unwrap_or(DATA + 0x600),
                    2 => *nodes.get(1).unwrap_or(&(DATA + 0x600)),
                    3 => DATA + 0x600,
                    _ => nodes.first().copied().unwrap_or(DATA + 0x600),
                };
                clear_2(c, DATA + 0x600, 8);
                c.bus.write32(DATA + 0x600 + 0x18, r.next_u32());
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(victim);
            },
        );
    }


    #[test]
    fn wscript_man_reset_collect_set_matches() {
        check(
            ee::s10::f_0030c390__generated,
            wscript_man_reset_collect_set,
            SEEDS_5,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x30c3_90);
                let this = DATA + 0x100;
                for i in 0..0x50u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    pub(crate) const SEEDS_6: u32 = 120;


    /// A `gp`-relative global address.
    pub(crate) fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }


    /// Call a generated helper as a setup step (arguments in a0..t1).
    pub(crate) fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    /// Zero `words` words at `at`.
    pub(crate) fn clear_3(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }


    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1); // size-class pools ready
    }


    /// The dummy world at `world_at`: `+0x84` the game module, its `+0xc` the
    /// race manager. Returns the module address.
    pub(crate) fn dummy_world(c: &mut Cpu, world_at: u32) -> u32 {
        let module = DATA + 0x3000;
        let race = DATA + 0x3400;
        clear_3(c, world_at, 0x40);
        clear_3(c, module, 0x100);
        clear_3(c, race, 0x100);
        c.bus.write32(world_at + 0x84, module);
        c.bus.write32(module + 0xc, race);
        c.bus.write32(gp_at(-0x848), world_at);
        module
    }


    #[test]
    fn wscript_man_run_obj_script0_matches() {
        check(
            ee::s10::f_0030a610__generated,
            wscript_man_run_obj_script0,
            SEEDS_6,
            &|c, seed| {
                let _ = machine(c, seed, 0x30a6_10, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let cache = DATA + 0x6000;
                clear_3(c, this, 0x400);
                clear_3(c, cache, 0x40);
                c.bus.write32(this + 0x28c, cache);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }


    #[test]
    fn wscript_process_list_delete_all_matches() {
        check(
            ee::s10::f_0030b1b8__generated,
            wscript_process_list_delete_all,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b1_b8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x40);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x800 + 0x100 * i;
                    clear_3(c, node, 0x80);
                    c.bus.write32(node + 0x18, next);
                    let vt = vtable(c, &mut r, &[8]);
                    c.bus.write32(node + 0x5c, vt);
                    next = node;
                }
                c.bus.write32(this, next);
                c.g[A0][0] = sx(this);
                c.pc = 0x0030_b1b8;
            },
        );
    }


    #[test]
    fn wscript_man_request_mission_start_matches() {
        check(
            ee::s10::f_0030b540__generated,
            wscript_man_request_mission_start,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b5_40, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                let module = dummy_world(c, DATA + 0x2000);
                let playback = DATA + 0x4000;
                let replay = DATA + 0x4400;
                clear_3(c, playback, 0x40);
                clear_3(c, replay, 0x40);
                c.bus.write32(module + 0x34, playback);
                c.bus.write32(module + 0x28, replay);
                // The replay manager's state 14 makes `Replay_Reset` return at
                // once (its reset is not what this test exercises).
                c.bus.write32(replay, 14);
                c.bus.write32(this + 0x2a0, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }


    // ------------------------------------------------------------- replay

    #[test]
    fn playback_man_stop_matches() {
        check(
            ssx3_ee::s07::f_0026cdf8__generated,
            playback_man_stop,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x26cdf8, 2);
                let p_2 = DATA + 0x400;
                // States 0 and 3 do nothing; 2 stops recording (whose world-state
                // restore is a no-op with a null pointer at +0x10). State 1 needs
                // the replay stream itself and is not exercised.
                let states = [0u32, 2, 2, 3, 5, 0xffff_ffff];
                c.bus.write32(p_2 + 0x70, states[(seed % 6) as usize]);
                c.bus.write32(p_2 + 0x10, 0);
                c.g[A0][0] = p_2 as u64;
            },
        );
    }


    #[test]
    fn wscript_man_deactivate_process_matches() {
        check(ee::s10::f_00309aa0__generated, wscript_man_deactivate_process, SEEDS_7, &|c, seed| {
            machine(c, seed, 0x0030_9aa0, 3);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let node = DATA + 0x100;
            if seed % 2 == 0 {
                w(c, DATA + 0x2b8, 0); // inactive list
            } else {
                w(c, DATA + 0x2b8, node);
                w(c, node + 0x18, 0);
            }
        });
    }


    #[test]
    fn w_script_man_request_mission_restart_matches() {
        check(ee::s10::f_0030b6f8__generated, w_script_man_request_mission_restart, 32, &|c, seed| {
            let mut r = base_3(c, seed, 0x30b6_f8);
            let world = DATA + 0x1000;
            let man = DATA + 0x2000;
            clear_4(c, world, 0x100);
            clear_4(c, man, 0x100);
            w_2(c, gp_at(-0x848), world as u32);
            w_2(c, world + 0x84, man as u32);
            w_2(c, man + 0x34, DATA + 0x3000);
            w_2(c, man + 0x28, DATA + 0x3400);
            clear_4(c, DATA + 0x3000, 0x40);
            clear_4(c, DATA + 0x3400, 0x40);
            let s = DATA + 0x4000;
            clear_4(c, s, 0x400);
            w_2(c, s + 0x2a0, r.next_u32());
            c.g[A0][0] = sx(s);
        });
    }


    #[test]
    fn wscript_process_list_find_mission_by_id_matches() {
        check(
            ee::s10::f_0030b388__generated,
            wscript_process_list_find_mission_by_id,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_b388, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x20, 0x55);
                c.bus.write32(DATA + 0x1000 + 0x18, 0);
                c.bus.write32(DATA + 0x1000 + 0x5c, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x18, 0);
                c.bus.write32(DATA + 0x2000 + 0x1c, FAKE);
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffffu64,
                    1 => 0x55,
                    _ => (r.next_u32() & 0xff) as u64,
                };
            },
        );
    }


    // ----------------------------------------------------- script and riders

    #[test]
    fn w_script_man_deactivate_mission_matches() {
        check(
            ee::s10::f_00309b00__generated,
            w_script_man_deactivate_mission,
            6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0030_9b00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = EV as u64;
                w(c, OBJ + 0x2b8, if seed % 2 == 0 { 0 } else { EV });
                w(c, OBJ + 0x2b4, 0);
                w(c, EV + 0x18, 0);
            },
        );
    }


    #[test]
    fn dead_node_ctor_matches() {
        check(ee::s11::f_003506d8__generated, dead_node_ctor, SEEDS_8, &|c, seed| {
            machine(c, seed, 0x3506d8, 4);
            be_library_2(c);
            world(c);
            heap_init(c);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = (DATA + 0x500) as u64;
            w(c, this + 0x18, DATA + 0x600);
        });
    }


    #[test]
    fn wscript_man_define_collect_set_matches() {
        check(ee::s10::f_0030b928__generated, wscript_man_define_collect_set, SEEDS_9, &|c, seed| {
            let mut r = machine(c, seed, 0x30b928, 1);
            be_library_3(c);
            let obj = DATA;
            let slotaddr = obj + 0x2c0;
            let mut id = r.next_u32() % 8;
            if seed % 3 == 1 {
                id = 0xffff_ffff;
            }
            if seed % 3 == 2 {
                id = 0x55;
            }
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = if seed % 3 == 0 { id as u64 } else { (r.next_u32() % 8) as u64 };
            c.g[A2][0] = r.next_u32() as u64;
            w(c, slotaddr, id);
            w(c, slotaddr + 4, 0);
        });
    }


    #[test]
    fn rider_model_alloc_model_refs_matches() {
        check_lohi(ee::s10::f_0030d840__generated, rider_model_alloc_model_refs, SEEDS_9, &|c, seed| {
            let mut r = machine(c, seed, 0x30d840, 2);
            be_library_3(c);
            heap_init(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn w_script_process_list_dtor_matches() {
        check(
            ee::s10::f_0030b068__generated,
            w_script_process_list_dtor,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x30b0_68, 4);
                let self_ = DATA;
                let n0 = DATA + 0x1000;
                let n1 = DATA + 0x1100;
                if seed % 2 == 0 {
                    w(c, self_, 0);
                } else {
                    w(c, self_, n0);
                    w(c, n0 + 0x18, if seed % 4 == 3 { n1 } else { 0 });
                    w(c, n1 + 0x18, 0);
                    for n in [n0, n1] {
                        w(c, n + 0x5c, DATA + 0x2000);
                        ret_slot(c, DATA + 0x2000, 0x8, RET_ZERO);
                    }
                }
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn w_script_man_table_has_function_matches() {
        check(
            ee::s10::f_00309dd0__generated,
            w_script_man_table_has_function,
            60,
            &|c, seed| {
                let m = DATA;
                let idx = (seed % 4) as u32;
                w(
                    c,
                    m + 0x3cc + 4 * idx,
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                w(c, DATA + 0x400 + 8, 3);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(DATA + 0x200);
            },
        );
    }


    #[test]
    fn wscript_man_collect_set_mark_item_matches() {
        check(
            ee::s10::f_0030c3e0__generated,
            wscript_man_collect_set_mark_item,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30c3e0, 3);
                be_library_4(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x2100);
                c.g[A0][0] = sx(DATA);
                let n = 1 + seed % 3;
                c.bus.write32(DATA + 8, n);
                for i in 0..3u32 {
                    c.bus
                        .write32(DATA + 0xc + 4 * i, if i + 1 == n { 7 } else { 100 + i });
                }
                c.g[A1][0] = if seed % 2 == 0 { 7 } else { 200 };
            },
        );
    }


    #[test]
    fn rider_model_ctor_matches() {
        check(ee::s10::f_0030d4b8__generated, rider_model_ctor, 30, &|c, seed| {
            let _r = machine(c, seed, 0x30d4b8, 0);
            c.g[A0][0] = sx(DATA);
        });
    }


    pub(crate) fn clear_5(c: &mut Cpu, a: u32, len: u32) {
        for i in 0..len / 4 {
            c.bus.write32(a + 4 * i, 0);
        }
    }


    pub(crate) const BE_3: u32 = DATA + 0x10_0000;

    pub(crate) const WORLD_3: u32 = DATA + 0x11_0000;

    pub(crate) const GAME: u32 = WORLD_3 + 0x1000;


    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    pub(crate) fn globals_3(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_3);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD_3);
        let game = GAME;
        w(c, WORLD_3 + 0x84, game);
        let race = WORLD_3 + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


    pub(crate) fn base_6(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals_3(c);
        r
    }


    // ------------------------------------------------------ WScript manager

        pub(crate) fn mission_world(c: &mut Cpu, seed: u32, salt: u64) -> (Rng, u32) {
        let r = base_6(c, seed, salt);
        let playback = DATA + 0x6000;
        let replay = DATA + 0x7000;
        let riders = DATA + 0x3000;
        let rider = DATA + 0x4000;
        let state = DATA + 0x5000;
        clear_5(c, playback, 0x100);
        clear_5(c, replay, 0x100);
        clear_5(c, riders, 0x100);
        clear_5(c, rider, 0x800);
        clear_5(c, state, 0x400);
        w(c, GAME + 0x34, playback);
        w(c, GAME + 0x28, replay);
        w(c, replay, 0xe);
        w(c, GAME + 0xc, riders);
        for i in 0..3 {
            w(c, riders + 0x28 + 4 * i, rider);
        }
        w(c, rider + 0x77c, state);
        let self_obj = DATA + 0x100;
        clear_5(c, self_obj, 0x300);
        w(c, self_obj, (seed % 3) as u32);
        (r, self_obj)
    }


    #[test]
    fn wscript_man_request_mission_cancel_matches() {
        check(ssx3_ee::s10::f_0030b658__generated, wscript_man_request_mission_cancel, 8, &|c, seed| {
            let (mut r, obj) = mission_world(c, seed, 0x0030_b658);
            w(c, obj + 0x2a0, r.next_u32());
            w(c, obj + 0x2ac, r.next_u32());
            w(c, obj + 0x2b0, r.next_u32());
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = (seed % 3) as u64;
        });
    }


        #[test]
    fn wscript_man_deactivate_all_matches() {
        check(ssx3_ee::s10::f_0030b7f8__generated, wscript_man_deactivate_all, 8, &|c, seed| {
            let _ = base_6(c, seed, 0x0030_b7f8);
            let man = DATA + 0x100;
            let mission = DATA + 0x2000;
            let idle = DATA + 0x2100;
            let queue = DATA + 0x2200;
            clear_5(c, man, 0x400);
            clear_5(c, mission, 0x40);
            clear_5(c, idle, 0x40);
            clear_5(c, queue, 0x100);
            w(c, man + 0x2b8, mission);
            w(c, man + 0x2b4, idle);
            w(c, man + 0x1c4, queue);
            w(c, man + 0x2a0, if seed % 2 == 0 { 0xffff_ffff } else { (1 + seed % 4) as u32 });
            c.pc = 0x0030_b7f8;
            c.g[A0][0] = sx(man);
        });
    }


    #[test]
    fn wscript_man_create_mission_matches() {
        check(
            ee::s10::f_00309568__generated,
            wscript_man_create_mission,
            60,
            &|c, seed| {
                let _r = alloc_base_2(c, seed, 0x3095);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = 0x12;
                c.g[A2][0] = 0x34;
                c.g[A3][0] = 0x56;
                c.g[T0][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x2b4, 0);
                c.bus.write32(DATA + 0x2bc, 0);
            },
        );
    }


    #[test]
    fn wscript_man_call_table_function_matches() {
        check(
            ee::s10::f_00309e50__generated,
            wscript_man_call_table_function,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x309e, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let entry = DATA + 0x2000;
                for i in 0..4 {
                    c.bus.write32(this + 0x3cc + 4 * i, entry + 0x40 * i);
                }
                c.bus.write32(LOG_RESULTS, 0);
            },
        );
    }


    #[test]
    fn wscript_process_list_delete_by_section_matches() {
        check(
            ee::s10::f_0030b0e8__generated,
            wscript_process_list_delete_by_section,
            80,
            &|c, seed| {
                let _r = base_8(c, seed, 0x30b0);
                c.g[A0][0] = DATA as u64;
                let section = (seed % 4) as u64;
                c.g[A1][0] = section;
                let nodes = if seed % 3 == 0 {
                    0
                } else {
                    1 + (seed % 2) as u32
                };
                c.bus
                    .write32(DATA, if nodes == 0 { 0 } else { DATA + 0x1000 });
                for i in 0..nodes {
                    let node = DATA + 0x1000 + 0x100 * i;
                    c.bus.write32(node + 0x24, section as u32);
                    c.bus
                        .write32(node + 0x18, if i + 1 == nodes { 0 } else { node + 0x100 });
                    c.bus.write32(node + 0x5c, DATA + 0x5000);
                }
                c.bus.write16(DATA + 0x5000 + 0x8, 0);
                c.bus.write32(DATA + 0x5000 + 0xc, FAKE);
            },
        );
    }


    #[test]
    fn wscriptman_load_process_list_matches() {
        check(
            ee::s10::f_0030bee8__generated,
            wscriptman_load_process_list,
            12,
            &|c, seed| {
                let mut r = base_9(c, seed, 0x30bee8, 2);
                let this = DATA;
                clear_6(c, this, 0x400);
                let stream = DATA + 0x800;
                clear_6(c, stream, 0x100);
                let list = DATA + 0xc00;
                clear_6(c, list, 0x100);
                let vt = vtable(c, &mut r, &[0x10]);
                w(c, stream, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = list as u64;
                c.g[A2][0] = stream as u64;
                // the stream read leaves the count where it is written
                w(c, SP - 0x70, 0);
            },
        );
    }


    #[test]
    fn w_script_man_get_hud_target_height_matches() {
        diff(ee::s10::f_00309ba8__generated, w_script_man_get_hud_target_height, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x30_9ba8);
            c.g[A0][0] = sx(DATA);
            w(c, DATA + 0x28, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
            w(c, DATA + 0x2c, r.next_u32());
            w(c, DATA + 0x30, r.next_u32());
            let t = DATA + 0x2000;
            c.f[12] = uf(&mut r, 100.0);
            w(c, t + 0x28c, r.next_u32());
            w(c, t + 0x6c0, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        });
    }


    #[test]
    fn w_script_man_collect_item_matches() {
        diff(ee::s10::f_0030b9a0__generated, w_script_man_collect_item, 30, &|c, seed| {
            let mut r = fe_base(c, seed, 0x30_b9a0);
            let item = (seed % 4) as u32;
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            c.g[A2][0] = item as u64;
            c.g[A3][0] = sx(DATA + 0x2000);
            w(c, DATA + 0x2c0, if seed % 2 == 0 { item } else { item ^ 1 });
            for i in 0..4 {
                w(c, DATA + 0x2c0 + 0x10c * i, r.next_u32());
            }
        });
    }
}
