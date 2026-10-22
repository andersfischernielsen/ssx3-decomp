//! `src/scripter/bxscriptengine.cpp`: hand-written replacements (moved from the
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
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use ssx3_ee as ee;
use crate::abi::r32;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call;
use crate::call::Unwound;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::movie::movieplayer::at as at_3;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::main::gamestate::lw_at_3;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::fe::feasyncfile::at as at_4;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_5;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_3;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_2cc0, in_seq_get_instance, "InSeq_GetInstance"),
    (0x0027_d050, script_engine_remove_all_script_objs, "ScriptEngine_RemoveAllScriptObjs"),
    (0x0028_2150, script_engine_register_script, "ScriptEngine_RegisterScript"),
    (0x0028_2178, script_engine_unregister_script, "ScriptEngine_UnregisterScript"),
    (0x0028_21a0, script_engine_start_script, "ScriptEngine_StartScript"),
    (0x0028_22a0, script_engine_run_process, "ScriptEngine_RunProcess"),
    (0x0028_2338, script_engine_stop_process_seq, "ScriptEngine_StopProcessSeq"),
    (0x0028_2390, script_engine_stop_all_process_seqs, "ScriptEngine_StopAllProcessSeqs"),
    (0x0028_23f0, script_engine_kill_all_processes, "ScriptEngine_KillAllProcesses"),
    (0x0028_2450, script_engine_tick_process, "ScriptEngine_TickProcess"),
    (0x0028_26d8, script_engine_pause_process, "ScriptEngine_PauseProcess"),
    (0x0028_2720, script_engine_resume_process, "ScriptEngine_ResumeProcess"),
    (0x0028_2798, script_engine_get_process_state, "ScriptEngine_GetProcessState"),
    (0x0028_2838, script_engine_is_process_finished, "ScriptEngine_IsProcessFinished"),
    (0x0028_28b8, script_engine_reset_process_table, "ScriptEngine_ResetProcessTable"),
    (0x0028_2908, script_engine_launch_pending_process, "ScriptEngine_LaunchPendingProcess"),
    (0x0028_29d0, script_engine_seq_listener_on_start, "ScriptEngine_SeqListenerOnStart"),
    (0x0028_2a18, script_engine_seq_listener_on_stop, "ScriptEngine_SeqListenerOnStop"),
    (0x0028_2b40, script_engine_seq_listener_on_create, "ScriptEngine_SeqListenerOnCreate"),
    (0x0028_2b88, script_engine_seq_listener_on_destroy, "ScriptEngine_SeqListenerOnDestroy"),
    (0x0028_2bf0, script_engine_get_process_in_seq, "ScriptEngine_GetProcessInSeq"),
    (0x0028_2c38, script_engine_alloc_process, "ScriptEngine_AllocProcess"),
    (0x0028_2c88, script_engine_clear_process, "ScriptEngine_ClearProcess"),
    (0x0028_2cb0, script_engine_get_process, "ScriptEngine_GetProcess"),
    (0x0028_2cd0, c_transition_man_script_obj_ctor, "cTransitionMan_ScriptObjCtor"),
    (0x0028_2d30, c_transition_man_script_advance_timer, "cTransitionMan_ScriptAdvanceTimer"),
    (0x0028_2ef0, script_obj_cam_has_binding, "ScriptObjCam_HasBinding"),
    (0x0028_2f00, c_transition_man_script_pause, "cTransitionMan_ScriptPause"),
    (0x0028_2f30, c_transition_man_script_resume, "cTransitionMan_ScriptResume"),
    (0x0028_2f90, script_binding_clear_listener, "ScriptBinding_ClearListener"),
    (0x0028_2fb8, script_binding_bind, "ScriptBinding_Bind"),
    (0x0028_3000, script_binding_unbind, "ScriptBinding_Unbind"),
    (0x0028_3038, script_binding_pause_seq, "ScriptBinding_PauseSeq"),
    (0x0028_3060, script_binding_resume_seq, "ScriptBinding_ResumeSeq"),
    (0x0028_3088, script_cam_binding_ctor, "ScriptCamBinding_ctor"),
    (0x0028_30c8, script_cam_binding_bind, "ScriptCamBinding_Bind"),
    (0x0028_31b0, script_cam_binding_accept_command, "ScriptCamBinding_AcceptCommand"),
    (0x0028_3200, script_obj_ctor, "ScriptObj_ctor"),
    (0x0028_3228, script_obj_dtor, "ScriptObj_dtor"),
    (0x0028_3298, script_obj_register, "ScriptObj_Register"),
    (0x0028_3320, scriptobj_pause, "ScriptObj_Pause"),
    (0x0028_33a8, scriptobj_resume, "ScriptObj_Resume"),
    (0x0028_3430, script_obj_activate, "ScriptObj_Activate"),
    (0x0028_3440, script_obj_deactivate, "ScriptObj_Deactivate"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;
    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;
    pub(crate) use crate::game::be::beintoption::callee::C_WORLD_SOUND_RELOCATE_EMITTERS;
    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;
    pub(crate) use crate::game::be::beintoption::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintoption::callee::BE_OPTION_INT_GET_MAX_HANDICAP;
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
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE as IN_SEQ_INSTANCE_PAUSE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_4;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
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
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_4;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT as LOBBY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT as NET_FE_STATE_HANDLE_NET_EVENT_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_5;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS as SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR as SCRIPT_CMD_QUEUE_CLEAR_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
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
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
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
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_4;
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
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_3;
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
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_6;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
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
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
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
}


// ---------------------------------------------------------------------------
// Instrument-sequence instances.

/// `void* InSeq_GetInstance(Channel* ch, int index)` (0x272cc0): the instance
/// pointer `index * 0x58` into the table at `ch + 0x14`, returned as the record
/// plus 8 (null when the record's +0xc field is zero).
pub fn in_seq_get_instance(c: &mut Cpu) {
    const TABLE: u32 = 0x14;
    const RECORD: u32 = 0x58;
    const RECORD_FLAG: u32 = 0xc;
    c.cycles += 8;
    c.g[V1][0] = RECORD as u64;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let scaled = abi::mult(c, abi::arg(c, 1), RECORD);
    c.g[A1][0] = sx(scaled);
    let record = sx((table as u32).wrapping_add(scaled));
    c.g[V0][0] = record;
    let flag = Addr(record as u32).off(RECORD_FLAG).lw(c);
    c.g[V1][0] = flag;
    c.g[V0][0] = sx((record as u32).wrapping_add(0x8));
    if flag == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script engine: registration table.

/// `bool ScriptEngine_RegisterScript(ScriptEngine *engine, ScriptObj *obj,
/// int index)` (0x282150): register `obj` in the table at `engine + 0x2b0`
/// (8-byte entries); false when the slot is already occupied.
pub fn script_engine_register_script(c: &mut Cpu) {
    c.cycles += 6;
    const TABLE: u32 = 0x2b0;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V1][0] = table;
    let index = abi::arg(c, 2);
    c.g[A2][0] = sx(index << 3);
    let slot = (table as u32).wrapping_add(index << 3);
    c.g[A2][0] = sx(slot);
    let existing = Addr(slot).lw(c);
    c.g[V0][0] = existing;
    c.g[V0][0] = 0x1;
    if existing == 0 {
        c.cycles += 2;
        Addr(slot).sw(c, abi::arg64(c, 1));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `ScriptObj *ScriptEngine_UnregisterScript(ScriptEngine *engine, int index)`
/// (0x282178): clear the table slot at `engine + 0x2b0 + index*8`, returning the
/// old pointer (0 when the slot was empty).
pub fn script_engine_unregister_script(c: &mut Cpu) {
    c.cycles += 6;
    const TABLE: u32 = 0x2b0;
    let table = Addr(abi::arg(c, 0)).off(TABLE).lw(c);
    c.g[V0][0] = table;
    let index = abi::arg(c, 1);
    c.g[A1][0] = sx(index << 3);
    let slot = (table as u32).wrapping_add(index << 3);
    c.g[A1][0] = sx(slot);
    let old = Addr(slot).lw(c);
    c.g[V0][0] = old;
    if old == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
    } else {
        c.cycles += 2;
        Addr(slot).sw(c, 0);
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script engine: process records.

/// `void ScriptEngine_ClearProcess(ScriptEngine *engine, ScriptProcess *proc)`
/// (0x282c88): reset a process record's fields (the engine argument is unused).
pub fn script_engine_clear_process(c: &mut Cpu) {
    c.cycles += 10;
    let proc = Addr(abi::arg(c, 1));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    proc.off(0xc).sw(c, 0);
    proc.off(0x14).sw(c, c.g[V0][0]);
    proc.sw(c, 0);
    proc.off(0x4).sw(c, 0);
    proc.off(0x20).sw(c, 0);
    proc.off(0x10).sh(c, 0);
    proc.off(0x12).sh(c, 0);
    proc.off(0x18).sw(c, 0);
    abi::ret(c);
}


/// `ScriptProcess *ScriptEngine_GetProcess(ScriptEngine *engine, int index)`
/// (0x282cb0): the process at `engine + 0x2c0 + index*0x24`, or null when its
/// +0x20 field is zero.
pub fn script_engine_get_process(c: &mut Cpu) {
    c.cycles += 7;
    const PROCESSES: u32 = 0x2c0;
    const PROCESS_SIZE: u32 = 0x24;
    const PRESENT: u32 = 0x20;
    let engine = abi::arg(c, 0);
    let offset = abi::mult(c, abi::arg(c, 1), PROCESS_SIZE).wrapping_add(PROCESSES);
    c.g[A1][0] = sx(offset);
    let proc = engine.wrapping_add(offset);
    c.g[V0][0] = sx(proc);
    c.g[V1][0] = Addr(proc).off(PRESENT).lw(c);
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script objects.

/// `ScriptObj *ScriptObj_ctor(ScriptObj *obj, int handle)` (0x283200):
/// initialise the object: `obj[0] = handle`, `obj[+0x4] = -1`, `obj[+0x8] = 0`,
/// `obj[+0xc] = vtable`.
pub fn script_obj_ctor(c: &mut Cpu) {
    c.cycles += 9;
    let obj = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x0048_0000;
    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x2558));
    obj.sw(c, abi::arg64(c, 1));
    obj.off(0xc).sw(c, c.g[V0][0]);
    c.g[V0][0] = abi::arg64(c, 0);
    obj.off(0x4).sw(c, c.g[V1][0]);
    obj.off(0x8).sw(c, 0);
    abi::ret(c);
}


/// `void ScriptObj_Activate(ScriptObj *obj)` (0x283430): increment the
/// reference count at +0x8.
pub fn script_obj_activate(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let count = obj.off(0x8).lw(c);
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    obj.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `void ScriptObj_Deactivate(ScriptObj *obj)` (0x283440): decrement the
/// reference count at +0x8.
pub fn script_obj_deactivate(c: &mut Cpu) {
    c.cycles += 4;
    let obj = Addr(abi::arg(c, 0));
    let count = obj.off(0x8).lw(c);
    c.g[V0][0] = sx((count as u32).wrapping_sub(1));
    obj.off(0x8).sw(c, c.g[V0][0]);
    abi::ret(c);
}


/// `bool ScriptObjCam_HasBinding(ScriptObjCam* cam, int id)` (0x282ef0): true
/// when the binding id at +0x14 equals `id`.
pub fn script_obj_cam_has_binding(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x14)) ^ id;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}


/// `void ScriptEngine_SeqListenerOnDestroy(ScriptEngine *self, SeqListener *l)`
/// (0x282b88): take the listener's process (`l->+0xc`), mark it destroyed
/// (`state = 7` at `+0x4`) and clear it.
pub fn script_engine_seq_listener_on_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 7; // process state: destroyed
        frame.save_ra(c, 0);
        let ctx = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A1][0] = ctx;
        Addr((ctx as u32).wrapping_add(4)).sw(c, c.g[V0][0]);
        callee::SCRIPT_ENGINE_CLEAR_PROCESS.call(c, 0x0028_2ba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int* ScriptEngine_GetProcessInSeq(eng, seq)` (0x282bf0): the process
/// record of `seq`, or 0 when the slot is empty.
pub fn script_engine_get_process_in_seq(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_GET_PROCESS.call(c, 0x0028_2c00)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script bindings.

/// `void ScriptBinding_ClearListener(ScriptBinding* self)` (0x282f90):
/// detach the listener from the binding's channel.
pub fn script_binding_clear_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 0;
            callee::IN_SEQ_CHANNEL_SET_LISTENER.call(c, 0x0028_2fac)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptBinding_PauseSeq(ScriptBinding* self)` (0x283038): pause the
/// sequence instance behind the binding's channel.
pub fn script_binding_pause_seq(c: &mut Cpu) {
    instance_pause_resume(c, callee::IN_SEQ_INSTANCE_PAUSE, 0x0028_3054)
}


/// `void ScriptBinding_ResumeSeq(ScriptBinding* self)` (0x283060): resume it.
pub fn script_binding_resume_seq(c: &mut Cpu) {
    instance_pause_resume(c, callee::IN_SEQ_INSTANCE_RESUME, 0x0028_307c)
}


/// `if (self->channel) InstancePause/Resume(self->channel->instance)`.
pub(crate) fn instance_pause_resume(c: &mut Cpu, target: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(arg(c, 0).wrapping_add(0x8)).lw(c);
        let none = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8)).lw(c);
            target.call(c, ret)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// The `ScriptObjCam`/`cTransitionMan` script hooks: a virtual method on the
/// script binding at `*(this + 0x10)`, whose vtable is at `*(binding + 0xc)`.
pub(crate) fn script_binding_vcall(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let binding = lw_at(c, this as u64, 0x10);
        c.g[A1][0] = binding;
        let vt = lw_at(c, binding, 0xc);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, slot);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((binding as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cTransitionMan_ScriptPause(this)` 0x282f00: slot +0x10 of the script
/// binding's vtable.
pub fn c_transition_man_script_pause(c: &mut Cpu) {
    script_binding_vcall(c, 0x10, 0x0028_2f20)
}


/// `cTransitionMan_ScriptResume(this)` 0x282f30: slot +0x18 of the script
/// binding's vtable.
pub fn c_transition_man_script_resume(c: &mut Cpu) {
    script_binding_vcall(c, 0x18, 0x0028_2f50)
}


/// `void ScriptBinding_Unbind(ScriptBinding* binding)` (0x283000):
/// deactivate the binding in the script engine and mark it unbound.
pub fn script_binding_unbind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE.call(c, 0x0028_3014)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_DEACTIVATE_BINDING.call(c, 0x0028_3020)?;
        Addr((c.g[S0][0] as u32).wrapping_add(8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `ScriptCamBinding* ScriptCamBinding_ctor(ScriptCamBinding* self)`
/// (0x283088): base constructor, then the camera-binding vtable and the
/// camera index 9 / field 0.
pub fn script_cam_binding_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR.call(c, 0x0028_309c)?;
        c.g[V0][0] = 9;
        c.g[V1][0] = 0x0048_0000;
        c.sw(p(r32(c, S0), 0xc), c.g[V0][0]);
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x23e0));
        c.sw(r32(c, S0), c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sw(p(r32(c, S0), 0x10), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptObj_Register(ScriptObj* self)` (0x283298): register with the
/// script engine (virtual slot 2 at `engine->vtable+0x10`) and store the id
/// it returns at `+4`.
pub fn script_obj_register(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE_2.call(c, 0x0028_32ac)?;
        c.g[V1][0] = Addr(r32(c, V0)).off(0x2a8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(r32(c, V1)).off(0x10).lh(c);
        c.g[A2][0] = Addr(r32(c, V1)).off(0x14).lw(c);
        let func = r32(c, A2);
        c.g[RA][0] = sx(0x0028_32c4);
        c.g[A0][0] = add32(c.g[V0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x0028_32c4 {
            return Err(crate::call::Unwound);
        }
        c.sw(p(r32(c, S0), 4), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptEngine_TickProcess(self, index)` (0x282450): tick the sequence
/// process in slot `index` of `self` when it is running (state 3 or 4);
/// returns 1 when it was ticked.
pub fn script_engine_tick_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_GET_PROCESS_2.call(c, 0x0028_2460)?;
        c.g[A0][0] = c.g[V0][0];
        let missing = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if missing {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let process = abi::r32(c, A0);
        c.cycles += 5;
        c.g[V0][0] = Addr(process.wrapping_add(4)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if idle {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(process).lw(c);
        callee::IN_SEQ_INSTANCE_UPDATE.call(c, 0x0028_2488)?;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_PauseProcess(self, index)` (0x2826d8): pause the
/// sequence process in slot `index` (`self + index*0x24 + 0x2c0`) and mark it
/// stopped (`+0x4 = 5`) when the process has pending work (`process+0x2c`).
pub fn script_engine_pause_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x24;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A1, A1, V0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c0));
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        let slot = abi::r32(c, S0);
        c.g[A0][0] = Addr(slot).lw(c);
        callee::IN_SEQ_INSTANCE_PAUSE_2.call(c, 0x0028_26fc)?;
        c.g[V0][0] = Addr(slot).lw(c);
        let process = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(process.wrapping_add(0x2c)).lw(c);
        let pending = (c.g[V1][0] as i64) <= 0;
        c.g[V0][0] = 5;
        if !pending {
            c.cycles += 1;
            Addr(slot.wrapping_add(4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_SeqListenerOnStart(self, instance)` (0x2829d0): the
/// sequence's start callback: call the listener slot (`self+0x2a8`, entry 10)
/// with the instance's ids and mark the instance started (`+0x4 = 4`).
pub fn script_engine_seq_listener_on_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        let instance = c.g[V0][0] as u32;
        c.g[S0][0] = Addr(instance.wrapping_add(0xc)).lw(c);
        let state = abi::r32(c, S0);
        c.g[V1][0] = Addr(self_.wrapping_add(0x2a8)).lw(c);
        c.g[A2][0] = Addr(state.wrapping_add(8)).lw(c);
        let vtable = c.g[V1][0] as u32;
        c.g[A3][0] = Addr(vtable.wrapping_add(0x50)).lh(c);
        c.g[T0][0] = Addr(vtable.wrapping_add(0x54)).lw(c);
        let target = c.g[T0][0] as u32;
        c.g[A0][0] = sx(self_.wrapping_add(abi::r32(c, A3)));
        c.g[A3][0] = Addr(state.wrapping_add(0xc)).lw(c);
        call_ptr(c, target, 0x0028_2a00)?;
        c.g[V0][0] = 4;
        Addr(state.wrapping_add(4)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_SeqListenerOnCreate(self, obj)` (0x282b40): allocate a
/// script thread for `obj`, mark it "stop requested" (`+0x4 = 2`, handle
/// `+0x8 = -1`), store `obj` at `+0` and hand the thread to the instance.
pub fn script_engine_seq_listener_on_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_ENGINE_ALLOC_PROCESS.call(c, 0x0028_2b58)?;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = 2;
        let thread = abi::r32(c, V0);
        Addr(thread.wrapping_add(8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        Addr(thread.wrapping_add(4)).sw(c, c.g[V1][0]);
        Addr(thread).sw(c, c.g[S0][0]);
        callee::IN_SEQ_INSTANCE_SET_USER_DATA.call(c, 0x0028_2b78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptBinding_Bind(self, binding)` (0x282fb8): remember `binding`
/// at `self+8` and activate it with the instance's current key.
pub fn script_binding_bind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, S0);
        let binding = abi::r32(c, S1);
        Addr(self_.wrapping_add(8)).sw(c, c.g[S1][0]);
        callee::SCRIPT_ENGINE_GET_INSTANCE_3.call(c, 0x0028_2fd8)?;
        c.g[V1][0] = Addr(binding.wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        let state = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(state).lw(c);
        callee::SCRIPT_ENGINE_ACTIVATE_BINDING.call(c, 0x0028_2fec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A28A4`: the script-engine singleton.
pub(crate) const SCRIPT_ENGINE_GP: i32 = -0x84c;


/// `void ScriptEngine_ResetProcessTable(ScriptEngine* e)` (0x2828b8): clear
/// the process count at +0x2b8/+0x2bc and the sixteen 0x24-byte process
/// entries at +0x2c0: id = i, +0x2d4 = -1, everything else 0.
/// 6 + 16×12 + 2 cycles.
pub fn script_engine_reset_process_table(c: &mut Cpu) {
    c.cycles += 6;
    let base = r32(c, A0);
    c.sw(p(base, 0x2b8), 0);
    c.g[V1][0] = sx(base);
    c.sw(p(base, 0x2bc), 0);
    c.g[A1][0] = 0;
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    let mut slot = base;
    loop {
        c.cycles += 12;
        c.sw(p(slot, 0x2dc), c.g[A1][0]);
        c.sw(p(slot, 0x2e0), 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.sw(p(slot, 0x2c0), 0);
        c.g[V0][0] = ((c.g[A1][0] as i64) < 16i64) as u64;
        c.sw(p(slot, 0x2c4), 0);
        Addr(p(slot, 0x2d0)).sh(c, 0);
        Addr(p(slot, 0x2d2)).sh(c, 0);
        c.sw(p(slot, 0x2d8), 0);
        c.sw(p(slot, 0x2d4), c.g[A0][0]);
        let more = c.g[V0][0] != 0;
        slot += 0x24;
        c.g[V1][0] = sx(slot);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0028_28d0);
            }
            continue;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
}


/// `ScriptThread* ScriptEngine_AllocProcess(ScriptEngine* e, int type)`
/// (0x282c38): find the first of the sixteen process entries at +0x2c0 whose
/// type word (+0x2e0) is 0, set it to `type` (a type-1 process gets handle -1)
/// and return the entry; 0 when the table is full.
/// 6 + 3 (+4) + 2 / 2 (+1) + 2 cycles.
pub fn script_engine_alloc_process(c: &mut Cpu) {
    c.cycles += 6;
    let base = r32(c, A0);
    let mut type_slot = p(base, 0x2e0);
    c.g[V1][0] = sx(type_slot);
    let ty = c.g[A1][0];
    c.g[A2][0] = 0;
    c.g[T0][0] = 1;
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.g[A0][0] = sx(p(base, 0x2c0));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw(type_slot);
        if c.g[V0][0] != 0 {
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.cycles += 4;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x24));
            c.g[V0][0] = ((c.g[A2][0] as i64) < 16i64) as u64;
            let more = c.g[V0][0] != 0;
            type_slot += 0x24;
            c.g[V1][0] = sx(type_slot);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0028_2c50);
                }
                continue;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        let plain = ty != c.g[T0][0];
        c.sw(type_slot, c.g[A1][0]);
        if !plain {
            c.cycles += 1;
            c.sw(type_slot.wrapping_sub(0x18), c.g[A3][0]);
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
}


/// `bool ScriptCamBinding_AcceptCommand(ScriptCamBinding* b, void* cmd, int n)`
/// (0x2831b0): true when `n < 4` and the instance script found for
/// `cmd->+8` has the same owner word (+0xc) as the binding's `+0x10 -> 0xf8`.
/// 6 (+9) + 1 or +4 cycles plus the callee.
pub fn script_cam_binding_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = ((c.g[A2][0] as i64) < 4i64) as u64;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let too_many = c.g[A2][0] == 0;
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        if too_many {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw(gp(c, SCRIPT_ENGINE_GP));
            c.g[A1][0] = c.lw(p(r32(c, A1), 8));
            callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT.call(c, 0x0028_31d4)?;
            c.g[V1][0] = c.lw(p(self_, 0x10));
            c.g[V0][0] = c.lw(p(r32(c, V0), 0xc));
            c.g[A0][0] = c.lw(p(r32(c, V1), 0xf8));
            c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
            c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_StopProcessSeq(ScriptEngine* e, int a1)` (0x282338):
/// stop the sequence process bound to the process returned by
/// `ScriptEngine_GetProcess`.
pub fn script_engine_stop_process_seq(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    if callee::SCRIPT_ENGINE_GET_PROCESS_3
        .call(c, 0x0028_234c)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = c.g[V0][0];
    c.g[A1][0] = Addr(p_2(c, V1, 0)).lw(c);
    if c.g[A1][0] == 0 {
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(p_2(c, A1, 0x20)).lw(c);
    let skip_tail = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if !skip_tail {
        c.cycles += 3;
        c.g[V1][0] = Addr(p_2(c, V1, 0x20)).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_2(c, S0, 0x2b8)).lw(c);
        c.g[A1][0] = Addr(p_2(c, A1, 0)).lw(c);
        if callee::IN_SEQ_STOP.call(c, 0x0028_2380).is_err() {
            return;
        }
    }
    // Both exits of the inner block charge this one cycle.
    c.cycles += 1;
    let _ = skip_tail;
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void ScriptEngine_StopAllProcessSeqs(ScriptEngine* self)` (0x282390):
/// stop the sequence of each of the 16 active slots at `+0x2C0` (0x24 bytes
/// apart).
pub fn script_engine_stop_all_process_seqs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(ea(c.g[S1][0], 0x2c0));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xf;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(ea(c.g[S0][0], 0x1c));
                c.g[A0][0] = c.g[S1][0];
                callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ.call(c, 0x0028_23c8)?;
            }
            c.g[S2][0] = sx(ea(c.g[S2][0], 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(ea(c.g[S0][0], 0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_23b0);
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


/// `void ScriptEngine_KillAllProcesses(ScriptEngine* self)` (0x2823F0): kill
/// each of the 16 slots at `+0x2C0` whose state word at `+0x20` is set.
pub fn script_engine_kill_all_processes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(ea(c.g[S1][0], 0x2c0));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0xf;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x20));
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(ea(c.g[S0][0], 0x1c));
                c.g[A0][0] = c.g[S1][0];
                callee::SCRIPT_ENGINE_KILL_PROCESS.call(c, 0x0028_2428)?;
            }
            c.g[S2][0] = sx(ea(c.g[S2][0], 0xffff_ffff));
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx(ea(c.g[S0][0], 0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_2410);
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


/// `int ScriptEngine_GetProcessState(ScriptEngine* self, int id)` (0x282798):
/// a pending process is launched (and its new state returned); a live one
/// just reports its state word; no process is 0.
pub fn script_engine_get_process_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_4.call(c, 0x0028_27b0)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[V1][0] = c.lw(ea(c.g[S0][0], 4));
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(ea(c.g[S0][0], 4));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS.call(c, 0x0028_27dc)?;
                c.g[V0][0] = c.lw(ea(c.g[S0][0], 4));
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
// Transitions, memory and pads.

/// `void cTransitionMan_ScriptObjCtor(cTransitionMan_ScriptObj* self)`
/// (0x282cd0): construct the script-object base and clear the transition
/// fields.
pub fn c_transition_man_script_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        call_ptr(c, 0x0028_3200, 0x0028_2ce8)?;
        c.g[V1][0] = 0x482_060;
        c.g[A1][0] = u64::MAX;
        c.g[V1][0] = sx(0x0048_2060);
        c.g[A0][0] = 0x9;
        Addr(at(c, S0, 0xc)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(at(c, S0, 0x14)).sw(c, c.g[A1][0]);
        Addr(at(c, S0, 0x18)).sw(c, c.g[A0][0]);
        Addr(at(c, S0, 0x10)).sw(c, 0);
        Addr(at(c, S0, 0x2c)).sw(c, 0);
        Addr(at(c, S0, 0x1c)).sw(c, 0);
        Addr(at(c, S0, 0x20)).sw(c, 0);
        Addr(at(c, S0, 0x24)).sw(c, 0);
        Addr(at(c, S0, 0x28)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine listeners.

/// `ScriptEngine_SeqListenerOnStop(ScriptEngine* self, ScriptSeqMsg* msg)`
/// (0x282a18): tell the engine's sequence callback (vtable slot 11) that the
/// sequence stopped, mark the sequence object's state 2 and kill the process
/// when it is still running.
pub fn script_engine_seq_listener_on_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        frame.save_ra(c, 0);
        let msg = c.g[A1][0];
        c.g[V0][0] = lw(c, p(this as u32, 0x2a8));
        c.g[S0][0] = lw(c, p(msg as u32, 0xc));
        let seq = c.g[S0][0] as u32;
        c.g[A0][0] = lh(c, p(c.g[V0][0] as u32, 0x58));
        c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x5c));
        c.g[A2][0] = lw(c, p(seq, 8));
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V1][0] as u32;
        c.g[A3][0] = lw(c, p(seq, 0xc));
        call_ptr(c, target, 0x0028_2a4c)?;
        c.g[V0][0] = 2;
        c.g[V1][0] = lh(c, p(seq, 0x10));
        let done = c.g[V1][0] == 0;
        sw(c, p(seq, 4), c.g[V0][0]);
        if !done {
            c.cycles += 3;
            c.g[A1][0] = lw(c, p(seq, 0x1c));
            c.g[A0][0] = this;
            call_ptr(c, 0x0028_2540, 0x0028_2a68)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptObj_dtor(ScriptObj* self, int flags)` (0x283228): release the
/// script object (unregister it when its registration word is 0 or -1) and
/// optionally free it.
pub fn script_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x4825_58;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(at_2(c, S0, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(at_2(c, S0, 4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
        let t = c.g[V1][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !t {
            c.cycles += 4;
            call_ptr(c, 0x0028_32d8, 0x0028_3264)?;
            c.g[V0][0] = u64::MAX;
            Addr(at_2(c, S0, 4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0028_3280)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_RemoveAllScriptObjs(ScriptEngine* e)` (0x27d050):
/// remove the first 20 script objects, calling virtual slot 3 on each.
pub fn script_engine_remove_all_script_objs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 18, 0x10);
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        c.g[A1][0] = 0;
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !empty {
                c.cycles += 5;
                c.g[V1][0] = Addr(p_3(r32(c, S2), 0x2a8)).lw(c);
                c.g[A0][0] = Addr(p_3(c.g[V1][0] as u32, 0x18)).lh(c);
                c.g[V0][0] = Addr(p_3(c.g[V1][0] as u32, 0x1c)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = abi::addu(c.g[S2][0], c.g[A0][0]);
                call::call_ptr(c, target, 0x0027_d09c)?;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 20) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_d078);
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptEngine_ResumeProcess(ScriptEngine* e, int idx)` (0x282720):
/// resume the in-sequence instance in slot `idx` and, when it was stopped but
/// has re-entered a runnable state, move its slot state to 3 or 4.
pub fn script_engine_resume_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[V0][0] = 0x24;
        let frame = Frame::open(c, 0x20);
        c.mult(0, A1, A1, V0);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c0));
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[A1][0]);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        call::call_ptr(c, 0x0027_4a08, 0x0028_2744)?;
        c.g[V1][0] = Addr(p_3(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = 5;
        if c.g[V1][0] != c.g[V0][0] {
            lqsp(c, 16, 0x10);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V0][0] = Addr(p_3(c.g[V1][0] as u32, 0x2c)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                lqsp(c, 16, 0x10);
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_3(c.g[V1][0] as u32, 0x20)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(3));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 4;
                if !t {
                    c.cycles += 1;
                    c.g[V0][0] = 3;
                }
                c.cycles += 2;
                Addr(p_3(r32(c, S0), 4)).sw(c, c.g[V0][0]);
                lqsp(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}


/// `cTransitionMan_ScriptAdvanceTimer` (0x282D30): advance a script transition's
/// timer by the frame delta and stop the transition when the hold time has
/// elapsed.
pub fn c_transition_man_script_advance_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw_at_2(c, A0, 0x14);
        let no_timer = (c.g[A1][0] as i64) < 0;
        c.g[RA][0] = Addr(frame.at(c, 0)).ld(c);
        if !no_timer {
            c.cycles += 15;
            c.lwc1(1, r32(c, A0).wrapping_add(0x20));
            c.lwc1(0, r32(c, A0).wrapping_add(0x1c));
            c.lwc1(2, gp(c, -0x4780));
            c.add_s(0, 0, 1);
            c.lwc1(4, r32(c, A0).wrapping_add(0x24));
            c.mul_s(2, 12, 2);
            c.lwc1(1, r32(c, A0).wrapping_add(0x2c));
            c.lwc1(3, gp(c, -0x477c));
            c.add_s(0, 0, 4);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 3);
            c.c_le_s(0, 1);
            // The stop arm is the one where `(timer <= target)` holds: the
            // generated code breaks out when the comparison is false.
            let stop = c.fcc();
            c.swc1(1, r32(c, A0).wrapping_add(0x2c));
            if stop {
                c.cycles += 7;
                c.mtc1(ZERO, 12);
                c.g[A2][0] = 9;
                c.mov_s(13, 12);
                c.mov_s(14, 12);
                c.mov_s(15, 12);
                callee::STOP_TRANSITION.call(c, 0x0028_2d98)?;
                c.g[RA][0] = Addr(frame.at(c, 0)).ld(c);
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptEngine_IsProcessFinished(self, a1)` (0x00282838): after a
/// process reaches its end state, launch it and report completion.
pub fn script_engine_is_process_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_5.call(c, 0x0028_2850)?;
        c.g[S0][0] = c.g[V0][0];
        let absent = c.g[S0][0] == 0;
        c.g[V0][0] = 1;
        if !absent {
            c.cycles += 3;
            c.g[V1][0] = Addr(at_3(c, S0, 0x4)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = Addr(at_3(c, S0, 0x4)).lw(c);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS_2.call(c, 0x0028_2874)?;
                c.g[V0][0] = Addr(at_3(c, S0, 0x4)).lw(c);
            }
            c.cycles += 4;
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(4));
            c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 5;
                c.g[V1][0] = Addr(at_3(c, S0, 0)).lw(c);
                c.g[V0][0] = Addr(at_3(c, V1, 0x20)).lw(c);
                c.g[V0][0] ^= 4;
                c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptObj_Pause(self)` (0x283320): pause every running sequence of
/// the object's binding.
pub fn scriptobj_pause(c: &mut Cpu) {
    script_obj_pause_resume(
        c,
        ScriptLoop {
            get_count_ret: 0x0028_3350,
            each: callee::SCRIPT_BINDING_PAUSE_SEQ,
            each_ret: 0x0028_3380,
            seq_ret: 0x0028_3378,
            head: 0x0028_3360,
        },
    );
}


/// `void ScriptObj_Resume(self)` (0x2833a8): resume every paused sequence.
pub fn scriptobj_resume(c: &mut Cpu) {
    script_obj_pause_resume(
        c,
        ScriptLoop {
            get_count_ret: 0x0028_33d8,
            each: callee::SCRIPT_BINDING_RESUME_SEQ,
            each_ret: 0x0028_3408,
            seq_ret: 0x0028_3400,
            head: 0x0028_33e8,
        },
    );
}


/// The addresses that differ between the pause and resume variants.
pub(crate) struct ScriptLoop {
    get_count_ret: u32,
    each: Callee,
    each_ret: u32,
    seq_ret: u32,
    head: u32,
}


pub(crate) fn script_obj_pause_resume(c: &mut Cpu, l: ScriptLoop) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_at_3(c, p(r32(c, S1), 0xc));
        let func = vslot(c, c.g[V1][0], 0x30, V0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, l.get_count_ret)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[S2][0] as i64) <= 0 {
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw_at_3(c, p(r32(c, S1), 0xc));
            loop {
                c.cycles += 11;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let func = vslot(c, c.g[V1][0], 0x38, V0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, l.seq_ret)?;
                c.g[A0][0] = c.g[V0][0];
                l.each.call(c, l.each_ret)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = lw_at_3(c, p(r32(c, S1), 0xc));
                if event_due(c) {
                    deliver_event_here(c, l.head);
                }
            }
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_DestroyScriptDatabase(ScriptEngine* self)` (0x282020):
/// stop and kill everything, drop the script object list, the process table
/// and the sequence bank, and reset the process table.
pub fn script_engine_destroy_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[V0][0] = c.lw(at_4(c, S0, 0x2b0));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            call_ptr(c, callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2, 0x0028_2044)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES, 0x0028_204c)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2, 0x0028_2054)?;
            c.g[A0][0] = c.lw(at_4(c, S0, 0x2b0));
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.lw(at_4(c, S0, 0x2b8));
            } else {
                c.cycles += 3;
                call_ptr(c, callee::OPERATOR_DELETE, 0x0028_2068)?;
                c.g[A0][0] = c.lw(at_4(c, S0, 0x2b8));
            }
            c.cycles += 3;
            c.sw(at_4(c, S0, 0x2b0), 0);
            let no_seq = c.g[A0][0] == 0;
            c.sw(at_4(c, S0, 0x2b4), 0);
            if !no_seq {
                c.cycles += 2;
                c.g[A1][0] = 3;
                call_ptr(c, callee::IN_SEQ_DTOR, 0x0028_2080)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(at_4(c, S0, 0x2bc));
            let no_bank = c.g[A0][0] == 0;
            c.sw(at_4(c, S0, 0x2b8), 0);
            if !no_bank {
                c.cycles += 2;
                call_ptr(c, callee::OPERATOR_DELETE, 0x0028_2094)?;
            }
            c.cycles += 4;
            c.sw(at_4(c, S0, 0x2bc), 0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE, 0x0028_20a0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptEngine_RunProcess(self, arg, value, unused)` (0x2822a0): run a
/// script process: store the 16-bit value and, if the process is armed, play
/// its sequence.
pub fn script_engine_run_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_6.call(c, 0x0028_22c8)?;
        c.g[S0][0] = c.g[V0][0];
        let mut run = false;
        if c.g[S0][0] != 0 {
            c.g[V1][0] = lw_2(c, S0, 0x4);
            run = true;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        if run {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let wrong_state = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[V1][0];
            if !wrong_state {
                c.cycles += 4;
                c.g[V1][0] = lw_2(c, S0, 0x20);
                c.g[V0][0] = 1;
                let not_armed = c.g[V1][0] != c.g[V0][0];
                Addr(c.g[S0][0] as u32).off(0x10).sh(c, c.g[S1][0]);
                if !not_armed {
                    c.cycles += 7;
                    c.g[V0][0] = 3;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    Addr(c.g[S0][0] as u32).off(0x4).sw(c, c.g[V0][0]);
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A0][0] = lw_2(c, S2, 0x2b8);
                    c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                    callee::IN_SEQ_PLAY.call(c, 0x0028_2314)?;
                }
                c.cycles += 1;
                c.g[V0][0] = lw_2(c, S0, 0x4);
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


// ---------------------------------------------------------------------------
// Script camera binding.

/// `void ScriptCamBinding_Bind(ScriptCamBinding* b, void* channel)` (0x2830c8):
/// bind the script camera to the channel and copy the camera flags appropriate
/// to its parameter type (`+0x21` for manual/target/subject, `+0x1c` for an
/// algorithmic camera).
pub fn script_cam_binding_bind(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let channel = r(c, A1);
        c.g[S0][0] = sx(channel);
        let this = r(c, A0);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(this);
        callee::SCRIPT_BINDING_BIND.call(c, 0x0028_30e4)?;
        c.g[RA][0] = sx(0x0028_30ec);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0027_43c8, 0x0028_30ec)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 1;
        let manual = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        'outer: {
            'store: {
                'alg: {
                    'subj: {
                        'target: {
                            if manual {
                                break 'target;
                            }
                            'zero: {
                                'cmp: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0 {
                                        c.g[V0][0] = 2;
                                        break 'cmp;
                                    }
                                    c.cycles += 2;
                                    let is_zero = c.g[V1][0] == 0;
                                    if is_zero {
                                        break 'zero;
                                    }
                                    c.cycles += 2;
                                    frame.restore(c, 16, 0x20);
                                    break 'outer;
                                }
                                c.cycles += 2;
                                let is_two = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 3;
                                if is_two {
                                    break 'subj;
                                }
                                c.cycles += 2;
                                let is_three = c.g[V1][0] == c.g[V0][0];
                                if is_three {
                                    break 'alg;
                                }
                                c.cycles += 2;
                                frame.restore(c, 16, 0x20);
                                break 'outer;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0028_3134);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0027_c098, 0x0028_3134)?;
                            c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                            break 'store;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x0028_3144);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0027_c2a8, 0x0028_3144)?;
                        c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                        break 'store;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0028_3154);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_c4b8, 0x0028_3154)?;
                    c.g[V1][0] = Addr(p(r(c, V0), 0x21)).lb(c);
                    break 'store;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x0028_3164);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_c6a0, 0x0028_3164)?;
                c.g[V1][0] = Addr(p(r(c, V0), 0x1c)).lb(c);
                break 'store;
            }
            c.cycles += 2;
            Addr(p(r(c, S1), 0xc)).sw(c, c.g[V1][0]);
            frame.restore(c, 16, 0x20);
            break 'outer;
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptEngine_LaunchPendingProcess(this, script)` (0x282908): start the
/// script's pending sequence, bind the instance as the thread's owner and
/// bump the slot's use count.
pub fn script_engine_launch_pending_process(c: &mut Cpu) {
    'b_2829b0: {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2b0));
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw((c.g[S2][0] as u32));
        let t = c.g[A0][0] == 0u64;
        c.g[V1][0] = 0x2;
        if t {
            break 'b_2829b0;
        } else {
            'b_2829a8: {
                c.cycles += 21;
                c.g[V0][0] = 0xffffffffffffffff;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x12), 0u64);
                c.mtc1(0, 20);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2b8));
                c.g[RA][0] = sx(0x00282968);
                c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                c.pc = 0x00272938;
                ee::s07::InSeq_StartSequence(c);
                if c.pc != 0x00282968 {
                    return;
                }
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2b8));
                c.g[RA][0] = sx(0x00282974);
                c.g[A1][0] = c.g[V0][0];
                c.pc = 0x00272cc0;
                ee::s07::InSeq_GetInstance(c);
                if c.pc != 0x00282974 {
                    return;
                }
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00282984);
                c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                c.pc = 0x00274c10;
                ee::s07::InSeq_InstanceSetUserData(c);
                if c.pc != 0x00282984 {
                    return;
                }
                c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x18));
                c.c_lt_s(20, 12);
                if !c.fcc() {
                    c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x4));
                    break 'b_2829a8;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002829a0);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    c.pc = 0x00274808;
                    ee::s07::InSeq_InstanceSetSpeed(c);
                    if c.pc != 0x002829a0 {
                        return;
                    }
                    c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x4));
                    break 'b_2829a8;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            break 'b_2829b0;
        }
    }
    c.cycles += 7;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
    return;
}


/// `ScriptEngine_CreateScriptDatabase(self, count, a2, a3)` (0x281f30):
/// create the script database: allocate the header, construct the
/// instruction sequence, allocate one 8-byte record per script and clear
/// them.
pub fn scriptengine_create_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        let count = abi::arg(c, 1);
        let a2 = abi::arg(c, 2);
        let size = abi::arg(c, 3);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f48));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0028_1f70);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_1f70)?; // operator new(size, ...)
        c.g[A1][0] = 0x4a_0000;
        c.g[A0][0] = 0x20;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34f0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0028_1f8c);
        c.sw(p(this, 0x2bc), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0028_1f8c)?; // operator new(0x20, ...)
        c.g[A1][0] = lw_3(c, p(this, 0x2bc));
        c.g[T0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2ac));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0028_1fa4);
        c.g[A3][0] = c.g[S2][0];
        c.pc = 0x0027_2788;
        ee::s07::InSeq_ctor(c);
        if c.pc != 0x0028_1fa4 {
            return Err(Unwound);
        }
        c.sw(p(this, 0x2b4), c.g[S1][0]);
        c.g[A1][0] = 0x48_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f58));
        c.sw(p(this, 0x2b8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S1][0] as u32) << 3);
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x0028_1fc4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_1fc4)?; // operator new(count * 8, ...)
        c.g[V1][0] = lw_3(c, p(this, 0x2b4));
        c.g[A1][0] = 0;
        let t = (c.g[V1][0] as i64) <= 0;
        c.sw(p(this, 0x2b0), c.g[V0][0]);
        if !t {
            c.cycles += 1;
            c.g[V1][0] = lw_3(c, p(this, 0x2b0));
            loop {
                c.cycles += 11;
                c.g[A0][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw(c.g[V1][0] as u32, 0);
                c.g[V0][0] = lw_3(c, p(this, 0x2b0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh(p(c.g[A0][0] as u32, 4), 0);
                c.g[V0][0] = lw_3(c, p(this, 0x2b4));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = lw_3(c, p(this, 0x2b0));
                if event_due(c) {
                    deliver_event_here(c, 0x0028_1fd8);
                }
            }
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


/// `int func_002821A0(ScriptEngine* self, int key, void* arg)` (0x2821a0):
/// start the script bound to slot `key` of the engine's script table: allocate
/// a thread, and either queue it for a sequence (when the slot has data) or
/// hand it to the slot's start callback; a refused start tears the thread down
/// again and returns -1.
pub fn script_engine_start_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 20, 0x10);
        c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        c.g[A1][0] = 0x1;
        c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x2b0));
        c.g[RA][0] = sx(0x0028_21dc);
        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.pc = 0x0028_2c38;
        ee::s07::ScriptEngine_AllocProcess(c);
        if c.pc != 0x0028_21dc {
            return Err(Unwound);
        }
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S3][0] as u32); // slot data
        let has_data = c.g[V0][0] != 0;
        c.g[V0][0] = 0x2;
        let mut refused = false;
        if has_data {
            c.cycles += 16;
            c.sw(p(c.g[S0][0] as u32, 8), c.g[S2][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x2b8));
            c.g[RA][0] = sx(0x0028_224c);
            c.g[A1][0] = c.lw(c.g[S3][0] as u32);
            c.pc = 0x0027_2938;
            ee::s07::InSeq_StartSequence(c);
            if c.pc != 0x0028_224c {
                return Err(Unwound);
            }
            c.g[A0][0] = c.lw(p(c.g[S1][0] as u32, 0x2b8));
            c.g[RA][0] = sx(0x0028_2258);
            c.g[A1][0] = c.g[V0][0];
            c.pc = 0x0027_2cc0;
            ee::s07::InSeq_GetInstance(c);
            if c.pc != 0x0028_2258 {
                return Err(Unwound);
            }
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0028_2268);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.pc = 0x0027_4c10;
            ee::s07::InSeq_InstanceSetUserData(c);
            if c.pc != 0x0028_2268 {
                return Err(Unwound);
            }
            c.g[V1][0] = c.lhu(p(c.g[S3][0] as u32, 4));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.sh(p(c.g[S3][0] as u32, 4), c.g[V1][0]);
            c.sw(p(c.g[S0][0] as u32, 0xc), c.g[S4][0]);
        } else {
            c.cycles += 12;
            c.g[V0][0] = 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x14), c.g[S2][0]);
            c.sh(p(c.g[S0][0] as u32, 0x12), c.g[V0][0]);
            c.g[A1][0] = c.g[S2][0];
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x2a8));
            c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x38));
            c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x3c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0028_2214);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0028_2214)?;
            if c.g[V0][0] != 0 {
                c.sw(p(c.g[S0][0] as u32, 0xc), c.g[S4][0]);
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0x7;
                c.g[A0][0] = c.g[S1][0];
                c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
                c.g[RA][0] = sx(0x0028_2230);
                c.g[A1][0] = c.g[S0][0];
                c.pc = 0x0028_2c88;
                ee::s07::ScriptEngine_ClearProcess(c);
                if c.pc != 0x0028_2230 {
                    return Err(Unwound);
                }
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                refused = true;
            }
        }
        if !refused {
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x1c));
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
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
    use crate::difftest::put_vec;
    use crate::difftest::GAME_GP;
    use ssx3_ee as ee;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::*;
    use crate::difftest::SP;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::compare;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::run;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::object::effectlink::tests::alloc;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY as EMPTY_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::slot as slot_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_6;
    pub(crate) use crate::game::ai::rider::tests::fb;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_7;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::slot_2 as slot_3;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_4;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_2;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals as game_globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::be::beintplayer::tests::SEEDS_7 as SEEDS_9;
    pub(crate) use crate::game::be::beintplayer::tests::strict;
    pub(crate) use crate::game::be::beintplayer::tests::globals_5 as globals_3;
    pub(crate) use crate::game::be::beintplayer::tests::base_10 as base_7;


    #[test]
    fn in_seq_get_instance_matches() {
        check_lohi(ssx3_ee::s07::f_00272cc0, in_seq_get_instance, 200, &|c, seed| {
            let mut r = Rng(0x72cc0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = seed % 4;
            c.g[A1][0] = index as u64 | ((r.next_u32() as u64) << 32);
            let table = DATA + 0x1000;
            c.bus.write32(DATA + 0x14, table);
            let record = table + index * 0x58;
            c.bus.write32(record + 0xc, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
        });
    }


    #[test]
    fn script_engine_register_script_matches() {
        check(ssx3_ee::s07::f_00282150, script_engine_register_script, 200, &|c, seed| {
            let mut r = Rng(0x2821_50 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let index = (seed % 4) as u64;
            c.g[A2][0] = index;
            let table = DATA + 0x800;
            c.bus.write32(DATA + 0x2b0, table);
            let slot = table + index as u32 * 8;
            c.bus.write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }


    #[test]
    fn script_engine_unregister_script_matches() {
        check(ssx3_ee::s07::f_00282178, script_engine_unregister_script, 200, &|c, seed| {
            let mut r = Rng(0x2821_78 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 4) as u64;
            c.g[A1][0] = index;
            let table = DATA + 0x800;
            c.bus.write32(DATA + 0x2b0, table);
            let slot = table + index as u32 * 8;
            c.bus.write32(slot, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }


    #[test]
    fn script_engine_clear_process_matches() {
        check(ssx3_ee::s07::f_00282c88, script_engine_clear_process, 100, &|c, seed| {
            let mut r = Rng(0x282c_88 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = (DATA + 0x100) as u64;
            for i in 0..0x24 {
                c.bus.write8(DATA + 0x100 + i, r.next_u32() as u8);
            }
        });
    }


    #[test]
    fn script_engine_get_process_matches() {
        check_lohi(ssx3_ee::s07::f_00282cb0, script_engine_get_process, 200, &|c, seed| {
            let mut r = Rng(0x282c_b0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 4) as u64;
            c.g[A1][0] = index;
            let proc = DATA + 0x2c0 + index as u32 * 0x24;
            c.bus.write32(proc + 0x20, if seed % 3 == 0 { 0 } else { r.next_u32() });
        });
    }


    #[test]
    fn script_obj_ctor_matches() {
        check(ssx3_ee::s07::f_00283200, script_obj_ctor, 100, &|c, seed| {
            let mut r = Rng(0x2832_00 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn script_obj_activate_matches() {
        check(ssx3_ee::s07::f_00283430, script_obj_activate, 100, &|c, seed| {
            let mut r = Rng(0x2834_30 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let value = match seed % 4 {
                0 => 0,
                1 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0x8, value);
        });
    }


    #[test]
    fn script_obj_deactivate_matches() {
        check(ssx3_ee::s07::f_00283440, script_obj_deactivate, 100, &|c, seed| {
            let mut r = Rng(0x2834_40 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let value = match seed % 4 {
                0 => 0,
                1 => 0xffff_ffff,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 0x8, value);
        });
    }


    #[test]
    fn script_obj_cam_has_binding_matches() {
        check(ee::s07::f_00282ef0__generated, script_obj_cam_has_binding, SEEDS, &|c, seed| {
            let mut r = rng(0x0028_2ef0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            let id = r.next_u32();
            c.bus.write32(DATA + 0x14, id);
            c.g[A1][0] = match seed % 3 {
                0 => mixed(&mut r, seed, id),
                1 => id as u64,
                _ => r.next_u32() as u64,
            };
        });
    }


    #[test]
    fn script_engine_seq_listener_on_destroy_matches() {
        check(ee::s07::f_00282b88__generated, script_engine_seq_listener_on_destroy, 100, &|c, seed| {
            let mut r = game(c, seed, 0x282b88, 1);
            c.g[A0][0] = SCRATCH as u64;
            c.g[A1][0] = (SCRATCH + 0x2000) as u64;
            let ctx = ptr(&mut r);
            c.bus.write32(SCRATCH + 0x200c, ctx);
        });
    }


    #[test]
    fn script_engine_get_process_in_seq_matches() {
        check(ssx3_ee::s07::f_00282bf0__generated, script_engine_get_process_in_seq, 200, &|c, seed| {
            let mut r = rng_2(seed, 0x35);
            scramble(c, &mut r);
            let engine = DATA + 0x4000;
            let index = seed % 6;
            let proc = engine + 0x2c0 + index * 0x24;
            c.bus.write32(proc + 0x20, (seed % 2) as u32);
            c.bus.write32(proc, r.next_u32());
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = index as u64;
        });
    }


    #[test]
    fn script_binding_clear_listener_matches() {
        check(ssx3_ee::s07::f_00282f90__generated, script_binding_clear_listener, 150, &|c, seed| {
            let mut r = rng_2(seed, 0x36);
            scramble(c, &mut r);
            let binding = DATA + 0x4000;
            let channel = DATA + 0x5000;
            c.bus.write32(binding + 8, if seed % 2 == 0 { 0 } else { channel });
            // No listener: the channel just records the null listener.
            c.bus.write32(channel + 0x2c, 0);
            c.bus.write32(channel + 0x1c, DATA + 0x6000);
            c.bus.write8(DATA + 0x6003, r.next_u32() as u8);
            c.g[A0][0] = sx(binding);
        });
    }


    pub(crate) fn binding_pause_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let binding = DATA + 0x4000;
        let channel = DATA + 0x5000;
        let inst = DATA + 0x6000;
        c.bus.write32(binding + 8, if seed % 2 == 0 { 0 } else { channel });
        c.bus.write32(channel + 8, inst);
        c.bus.write32(inst + 0x20, (seed % 3 != 0) as u32);
        c.bus.write32(inst + 0x2c, match seed % 4 {
            0 => 0,
            1 => 1,
            _ => r.next_u32(),
        });
        c.g[A0][0] = sx(binding);
    }


    #[test]
    fn script_binding_pause_seq_matches() {
        check(ssx3_ee::s07::f_00283038__generated, script_binding_pause_seq, 200, &|c, seed| {
            binding_pause_setup(c, seed, 0x37)
        });
    }


    #[test]
    fn script_binding_resume_seq_matches() {
        check(ssx3_ee::s07::f_00283060__generated, script_binding_resume_seq, 200, &|c, seed| {
            binding_pause_setup(c, seed, 0x38)
        });
    }


    /// The four script-binding hooks (ScriptObjCam / cTransitionMan).
    pub(crate) fn script_binding_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
        });
    }


    #[test]
    fn c_transition_man_script_pause_matches() {
        script_binding_test(
            ssx3_ee::s07::f_00282f00__generated,
            c_transition_man_script_pause,
            0xcf,
        );
    }


    #[test]
    fn c_transition_man_script_resume_matches() {
        script_binding_test(
            ssx3_ee::s07::f_00282f30__generated,
            c_transition_man_script_resume,
            0xd0,
        );
    }


    #[test]
    fn script_binding_unbind_matches() {
        check(ssx3_ee::s07::f_00283000__generated, script_binding_unbind, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x283000, 1);
            let binding = DATA + 0x100;
            c.bus.write32(GAME_GP + 0x3ec, DATA + 0x2000);
            fill(c, &mut r, binding, 0x20);
            c.bus.write32(binding + 4, r.next_u32() % 8);
            c.g[A0][0] = sx(binding);
        });
    }


    #[test]
    fn script_cam_binding_ctor_matches() {
        check(
            ee::s07::f_00283088__generated,
            script_cam_binding_ctor,
            SEEDS_3,
            &|c, _seed| {
                let _ = machine(c, 0, 0x283088, 0);
                c.g[A0][0] = (DATA + 0x200) as u64;
            },
        );
    }


    #[test]
    fn script_obj_register_matches() {
        check(
            ee::s07::f_00283298__generated,
            script_obj_register,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x283298, 0);
                let engine = DATA + 0x400;
                let vt = vtable(c, &mut r, &[0x10]);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(GAME_GP + 0x3ec, engine);
                c.bus.write32(engine + 0x2a8, vt);
            },
        );
    }


    #[test]
    fn script_engine_tick_process_matches() {
        check(
            ee::s07::f_00282450__generated,
            script_engine_tick_process,
            32,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2824_50, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = 0;
                let slot = this + 0x2c0;
                c.bus
                    .write32(slot + 0x20, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(
                    slot + 4,
                    match seed % 4 {
                        0 => 3,
                        1 => 4,
                        2 => 2,
                        _ => r.next_u32() & 0xff,
                    },
                );
                c.bus.write32(slot, DATA + 0x400);
            },
        );
    }


    #[test]
    fn script_engine_pause_process_matches() {
        check(
            ee::s07::f_002826d8__generated,
            script_engine_pause_process,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2826_d8, 1);
                let this = DATA + 0x100;
                let index = seed % 3;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(index);
                let slot = this + 0x2c0 + 0x24 * index;
                c.bus.write32(slot, DATA + 0x400);
                c.bus.write32(slot + 4, r.next_u32());
                c.bus
                    .write32(DATA + 0x400 + 0x2c, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }


    #[test]
    fn script_engine_seq_listener_on_start_matches() {
        check(
            ee::s07::f_002829d0__generated,
            script_engine_seq_listener_on_start,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2829_d0, 1);
                let this = DATA + 0x100;
                let obj = DATA + 0x200;
                let state = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 0xc, state);
                c.bus.write32(state + 8, r.next_u32());
                c.bus.write32(state + 0xc, r.next_u32());
                c.bus.write32(this + 0x2a8, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 0x50, FAKE + 0x20);
            },
        );
    }


    #[test]
    fn script_engine_seq_listener_on_create_matches() {
        check(
            ee::s07::f_00282b40__generated,
            script_engine_seq_listener_on_create,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x282b_40, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x200);
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn script_binding_bind_matches() {
        check(ee::s07::f_00282fb8__generated, script_binding_bind, 40, &|c, seed| {
            let mut r = call_machine(c, seed, 0x282f_b8, 1);
            let this = DATA + 0x100;
            let binding = DATA + 0x200;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(binding);
            c.bus.write32(binding + 8, DATA + 0x300);
            c.bus.write32(DATA + 0x300, r.next_u32());
        });
    }


    #[test]
    fn script_engine_reset_process_table_matches() {
        check(ee::s07::f_002828b8__generated, script_engine_reset_process_table, 80, &|c, seed| {
            let mut r = Rng(0x2828_b811 ^ seed as u64);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x300);
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn script_engine_alloc_process_matches() {
        check(ee::s07::f_00282c38__generated, script_engine_alloc_process, 80, &|c, seed| {
            let mut r = Rng(0x282c_3811 ^ seed as u64);
            scramble(c, &mut r);
            let e = DATA;
            fill(c, &mut r, e + 0x2c0, 16 * 0x24 / 4);
            // Leave a few slots occupied so the search walks the table; make
            // slot 0 occupied for some seeds so the walk exhausts.
            let used = seed % 6;
            for i in 0..used {
                c.bus.write32(e + 0x2e0 + 0x24 * i, 1 + i);
            }
            c.bus.write32(e + 0x300, 0);
            c.g[A0][0] = sx(e);
            c.g[A1][0] = match seed % 3 {
                0 => 1u64,
                1 => 0,
                _ => (seed % 5) as u64,
            };
        });
    }


    #[test]
    fn script_cam_binding_accept_command_matches() {
        check(ee::s07::f_002831b0__generated, script_cam_binding_accept_command, 80, &|c, seed| {
            let mut r = base(c, seed, 0x2831);
            let b = DATA;
            let cmd = DATA + 0x400;
            c.bus.write32(b + 0x10, DATA + 0x800);
            c.bus.write32(DATA + 0x800 + 0xf8, r.next_u32());
            c.bus.write32(cmd + 8, r.next_u32());
            c.bus.write32(gp_at(-0x84c), DATA + 0x1000);
            fill(c, &mut r, DATA + 0x1000, 0x40);
            c.g[A0][0] = sx(b);
            c.g[A1][0] = sx(cmd);
            c.g[A2][0] = (seed % 6) as u64;
        });
    }


    #[test]
    fn script_engine_stop_process_seq_matches() {
        check(
            ssx3_ee::s07::f_00282338__generated,
            script_engine_stop_process_seq,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x282338, 2);
                // `ScriptEngine_GetProcess` returns 0 for a zeroed engine, so the
                // process pointer is null and the sequence stop is skipped.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }


    #[test]
    fn script_engine_stop_all_process_seqs_matches() {
        check(ee::s07::f_00282390__generated, script_engine_stop_all_process_seqs, 64, &|c, seed| {
            machine(c, seed, 0x0028_2390, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            for i in 0..16u32 {
                let slot = DATA + 0x2c0 + 0x24 * i;
                for k in 0..9u32 {
                    w(c, slot + 4 * k, 0);
                }
            }
            if seed % 2 == 1 {
                // One active sequence; its process id 0 resolves to the empty
                // slot 0, so `StopProcessSeq` returns at once.
                w(c, DATA + 0x2c0 + 0x24 * (seed % 16) as u32, 1);
            }
        });
    }


    #[test]
    fn script_engine_kill_all_processes_matches() {
        check(ee::s07::f_002823f0__generated, script_engine_kill_all_processes, 64, &|c, seed| {
            machine(c, seed, 0x0028_23f0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            for i in 0..16u32 {
                let slot = DATA + 0x2c0 + 0x24 * i;
                for k in 0..9u32 {
                    w(c, slot + 4 * k, 0);
                }
            }
        });
    }


    #[test]
    fn script_engine_get_process_state_matches() {
        check(ee::s07::f_00282798__generated, script_engine_get_process_state, SEEDS_4, &|c, seed| {
            machine(c, seed, 0x0028_2798, 2);
            belib(c);
            let id = seed % 6;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = id as u64;
            let slot = DATA + 0x2c0 + 0x24 * id;
            if seed % 2 == 0 {
                w(c, slot + 0x20, 0); // no such process
            } else {
                w(c, slot + 0x20, 1);
                w(c, slot + 4, 0); // already running: reports its state word
            }
        });
    }


    #[test]
    fn c_transition_man_script_obj_ctor_matches() {
        check(
            ssx3_ee::s07::f_00282cd0__generated,
            c_transition_man_script_obj_ctor,
            60,
            &|c, seed| {
                fakes::machine(c, seed, 0x282cd0, 0);
                c.bus.write32(OBJ, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }


    #[test]
    fn script_engine_seq_listener_on_stop_matches() {
        check(
            ssx3_ee::s07::f_00282a18__generated,
            script_engine_seq_listener_on_stop,
            90,
            &|c, seed| {
                let mut r = setup(c, seed, 0x282a18);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let vt = vtable(c, &mut r, &[0x58]);
                c.bus.write32(DATA + 0x2a8, vt);
                c.bus.write32(DATA + 0x10c, DATA + 0x200);
                c.bus.write32(DATA + 0x208, r.next_u32());
                c.bus.write32(DATA + 0x20c, r.next_u32());
                c.bus
                    .write16(DATA + 0x210, if seed % 2 == 0 { 0 } else { 3 });
                c.bus.write32(DATA + 0x21c, r.next_u32());
            },
        );
    }


    #[test]
    fn script_obj_dtor_matches() {
        check(ee::s07::f_00283228__generated, script_obj_dtor, 80, &|c, seed| {
            alloc_base(c, seed, 0x2832_28);
            let se = DATA + 0x2000;
            let vt = DATA + 0x2200;
            c.bus.write32(GAME_GP + 0x3ec, se);
            c.bus.write32(se + 0x2a8, vt);
            put_vtable(c, vt, &[0x18]);
            let obj = alloc(c, 0x30);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = [0u64, 2, 1][seed as usize % 3];
            c.bus
                .write32(obj + 4, [0u32, u32::MAX, 1, 5][seed as usize % 4]);
        });
    }


    #[test]
    fn script_engine_remove_all_script_objs_matches() {
        check(
            ssx3_ee::s07::f_0027d050__generated,
            script_engine_remove_all_script_objs,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27d050, 0);
                let e = DATA + 0x1000;
                c.g[A0][0] = e as u64;
                let vt = DATA + 0x5000;
                w32(c, e + 0x2a8, vt);
                slot_2(c, vt, 0x18, 0, EMPTY_2);
                for i in 0..20u32 {
                    w32(c, e + 4 * i, if (seed + i) % 4 == 0 { 0 } else { r.next_u32() });
                }
            },
        );
    }


    #[test]
    fn script_engine_resume_process_matches() {
        check(ssx3_ee::s07::f_00282720__generated, script_engine_resume_process, SEEDS_5, &|c, seed| {
            let mut r = machine(c, seed, 0x282720, 0);
            let e = DATA + 0x1000;
            let idx = (seed % 4) as u32;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = idx as u64;
            let slotp = e + idx * 0x24 + 0x2c0;
            let inst = DATA + 0x3000 + idx * 0x100;
            w32(c, slotp, inst);
            w32(c, slotp + 4, if seed % 3 == 0 { 5 } else { (seed % 7) as u32 });
            w32(c, inst + 0x20, if seed % 2 == 0 { 0 } else { 3 + (seed % 4) });
            w32(c, inst + 0x2c, if seed % 5 == 0 { 1 } else { 0 });
            let _ = &mut r;
        });
    }


    #[test]
    fn c_transition_man_script_advance_timer_matches() {
        check(ee::s07::f_00282d30__generated, c_transition_man_script_advance_timer, SEEDS_6, &|c, seed| {
            let mut r = machine(c, seed, 0x28_2d30, 2);
            c.f[12] = r.edge_f32();
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let (a, b_, d) = if seed % 2 == 0 {
                (fb(1.0), fb(2.0), fb(4.0))
            } else {
                (fb(100.0), fb(0.5), fb(0.25))
            };
            w(c, DATA + 0x2000 + 0x14, if seed % 3 == 0 { 0xffff_ffff } else { 3 });
            w(c, DATA + 0x2000 + 0x1c, a);
            w(c, DATA + 0x2000 + 0x20, fb(0.5));
            w(c, DATA + 0x2000 + 0x24, fb(0.25));
            w(c, DATA + 0x2000 + 0x2c, b_);
            let _ = d;
            c.pc = 0x0028_2d30;
        });
    }


    // ----------------------------------------------------------------- scripts

    #[test]
    fn script_engine_is_process_finished_matches() {
        check(
            ee::s07::f_00282838__generated,
            script_engine_is_process_finished,
            16,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2828_38, 4);
                let self_ = DATA;
                let idx = (seed % 6) as u32;
                let proc = self_ + 0x2c0 + idx * 0x24;
                w(c, proc + 0x20, if seed % 5 == 0 { 0 } else { 1 });
                w(c, proc + 0x4, (seed % 8) as u32);
                w(c, proc, DATA + 0x1000);
                w(c, DATA + 0x1000 + 0x20, (seed % 5) as u32);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = idx as u64;
            },
        );
    }


    #[test]
    fn scriptobj_pause_resume_matches() {
        for (gen, hand) in [
            (
                ee::s07::f_00283320__generated as fn(&mut Cpu),
                scriptobj_pause as fn(&mut Cpu),
            ),
            (ee::s07::f_002833a8__generated, scriptobj_resume),
        ] {
            check(gen, hand, SEEDS_7, &|c, seed| {
                base_4(c, seed, 0x83320, 0);
                let obj = DATA + 0x100;
                let binding = DATA + 0x200;
                c.g[A0][0] = sx(obj);
                c.bus.write32(obj + 0xc, binding);
                slot_3(c, binding, 0x30, 0, FAKE);
                slot_3(c, binding, 0x38, 0, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS, 1 + seed % 3);
            });
        }
    }


    // ------------------------------------------------------------------
    // Script / player / crowd.

    #[test]
    fn script_engine_run_process_matches() {
        check(
            ee::s07::f_002822a0__generated,
            script_engine_run_process,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x2822a0, 3);
                let eng = DATA + 0x1000;
                let id = (seed % 4) as u32;
                let proc = eng + 0x2c0 + 0x24 * id;
                c.g[A0][0] = eng as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                w(c, proc + 0x20, (seed / 4 % 2) as u32);
                w(c, proc + 0x4, (seed / 8 % 4) as u32);
                w(c, proc + 0x10, r.next_u32());
                w(c, proc, DATA + 0x2000);
                w(c, DATA + 0x2000, r.next_u32());
                w(c, eng + 0x2b8, DATA + 0x3000);
            },
        );
    }


    // ---------------------------------------------------------- script cam

    #[test]
    fn script_cam_binding_bind_matches() {
        check(ee::s07::f_002830c8__generated, script_cam_binding_bind, 30, &|c, seed| {
            let mut r = base_6(c, seed, 0x2830, 2);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            for i in 0..0x40u32 {
                w(c, DATA + 0x800 + 4 * i, r.next_u32());
            }
        });
    }


        #[test]
    fn script_engine_launch_pending_process_matches() {
        check_all(
            ee::s07::f_00282908__generated,
            script_engine_launch_pending_process,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2829, 3);
                game_globals_2(c);
                let this = DATA;
                let script = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = script as u64;
                let slots = DATA + 0x2400;
                c.bus.write32(this + 0x2b0, slots);
                c.bus.write32(this + 0x2b8, DATA + 0x2800);
                c.bus.write32(script + 0x14, seed % 4);
                if seed % 3 != 0 {
                    c.bus.write32(slots + 8 * (seed % 4), DATA + 0x2c00);
                }
                c.bus.write32(script + 0x18, r.f32(1.0));
            },
        );
    }


    /// A vtable at `at` whose slots are fake calls (all with delta 8).
    pub(crate) fn fake_table(c: &mut Cpu, at: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            c.bus.write16(at + off, 8u16);
            w(c, at + off + 4, crate::difftest::fakes::FAKE + 0x10 * i as u32);
        }
    }


        #[test]
    fn script_engine_start_script_matches() {
        // Only the "slot has no sequence data" arm: the sequence arm needs a
        // live sequence manager behind the engine.
        strict(ee::s07::f_002821a0__generated, script_engine_start_script, SEEDS_9, &|c, seed| {
            let mut r = base_7(c, seed, 0x21a0, 1);
            let engine = DATA + 0x1000;
            let slots = DATA + 0x4000;
            let callbacks = DATA + 0x5000;
            let key = (seed % 4) as u32;
            w(c, engine + 0x2b0, slots);
            w(c, engine + 0x2a8, callbacks);
            fake_table(c, callbacks, &[0x38]);
            w(c, slots + 8 * key, 0);
            c.bus.write16(slots + 8 * key + 4, r.next_u32() as u16);
            // the start callback's verdict: refuse on half the seeds
            w(
                c,
                crate::difftest::fakes::LOG_RESULTS,
                if seed % 2 == 0 { 0 } else { DATA + 0x7000 },
            );
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = key as u64;
            c.g[A2][0] = sx(DATA + 0x3000);
        });
    }
}
