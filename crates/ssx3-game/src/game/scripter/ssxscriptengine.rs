//! `src/scripter/ssxscriptengine.cpp`: hand-written replacements (moved from the
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
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use crate::call::VSlot;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use crate::call;
use crate::events::deliver_event;
pub(crate) use crate::game::object::debouncenode::p;
pub(crate) use crate::game::visualfx::renderstateman::gp;
pub(crate) use crate::game::bx::memblockman::gp_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::main::gamestate::script_list_call;
pub(crate) use crate::game::scripter::bxscriptengine::script_binding_vcall;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_4;
pub(crate) use crate::game::scripter::scsndmgr::SCRIPT_OBJ_TABLE_GP;
pub(crate) use crate::game::scripter::scsndmgr::script_obj_dtor;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_4;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::main::gamestate::lw_at_3;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_6;
pub(crate) use crate::game::fe::festaterewards::lw_3;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_5;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_6;
pub(crate) use crate::game::fe::festaterewards::gp_4 as gp_7;
pub(crate) use crate::game::bx::bxstring2::gp_4 as gp_8;
pub(crate) use crate::game::fe::feasyncfile::lh_2;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_3;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_42e0, inseq_channel_get_block_payload, "InSeq_ChannelGetBlockPayload"),
    (0x0027_8720, script_engine_update, "ScriptEngine_Update"),
    (0x0027_89e0, script_engine_stop_lists, "ScriptEngine_StopLists"),
    (0x0027_8a10, script_engine_set_locator_table, "ScriptEngine_SetLocatorTable"),
    (0x0027_8a58, script_engine_clear_locator_table, "ScriptEngine_ClearLocatorTable"),
    (0x0027_8a98, script_engine_stop_all, "ScriptEngine_StopAll"),
    (0x0027_8ad0, script_engine_v06, "ScriptEngine_v06"),
    (0x0027_8cd8, script_engine_unload_world_scripts, "ScriptEngine_UnloadWorldScripts"),
    (0x0027_8d58, script_engine_is_load_done, "ScriptEngine_IsLoadDone"),
    (0x0027_8da0, script_engine_is_active_list_skippable, "ScriptEngine_IsActiveListSkippable"),
    (0x0027_8e20, script_engine_list_add, "ScriptEngine_ListAdd"),
    (0x0027_8e50, script_engine_list_add_choice, "ScriptEngine_ListAddChoice"),
    (0x0027_8e90, script_engine_list_add_movie, "ScriptEngine_ListAddMovie"),
    (0x0027_8ed0, script_engine_list_add_movie_by_id, "ScriptEngine_ListAddMovieById"),
    (0x0027_8f68, script_engine_list_start, "ScriptEngine_ListStart"),
    (0x0027_9040, script_engine_list_set_paused, "ScriptEngine_ListSetPaused"),
    (0x0027_9070, script_engine_list_skip, "ScriptEngine_ListSkip"),
    (0x0027_90f8, script_engine_stop_main_lists, "ScriptEngine_StopMainLists"),
    (0x0027_9148, script_engine_list_get_entry_type, "ScriptEngine_ListGetEntryType"),
    (0x0027_91d8, script_engine_list_get_state, "ScriptEngine_ListGetState"),
    (0x0027_9220, script_engine_list_get_play_status, "ScriptEngine_ListGetPlayStatus"),
    (0x0027_9248, script_engine_list_count, "ScriptEngine_ListCount"),
    (0x0027_9298, script_engine_get_active_list_field_b0, "ScriptEngine_GetActiveListFieldB0"),
    (0x0027_9370, script_engine_is_playing_script, "ScriptEngine_IsPlayingScript"),
    (0x0027_9488, script_engine_stop_aux_lists, "ScriptEngine_StopAuxLists"),
    (0x0027_9528, script_engine_spawn_chosen_script, "ScriptEngine_SpawnChosenScript"),
    (0x0027_95d0, script_engine_spawn_script, "ScriptEngine_SpawnScript"),
    (0x0027_9720, script_engine_play_script_instance, "ScriptEngine_PlayScriptInstance"),
    (0x0027_97a0, script_engine_find_script_index, "ScriptEngine_FindScriptIndex"),
    (0x0027_97c8, script_engine_load_script_bank, "ScriptEngine_LoadScriptBank"),
    (0x0027_9840, script_engine_cancel_script_bank, "ScriptEngine_CancelScriptBank"),
    (0x0027_99e0, script_engine_on_data_removed, "ScriptEngine_OnDataRemoved"),
    (0x0027_9ce0, script_engine_v12, "ScriptEngine_v12"),
    (0x0027_a688, script_engine_get_world_section2b, "ScriptEngine_GetWorldSection2B"),
    (0x0027_a6b0, script_engine_on_seq_start, "ScriptEngine_OnSeqStart"),
    (0x0027_a720, script_engine_on_seq_advance, "ScriptEngine_OnSeqAdvance"),
    (0x0027_a798, script_engine_on_seq_end, "ScriptEngine_OnSeqEnd"),
    (0x0027_a7f0, script_engine_on_seq_stopped, "ScriptEngine_OnSeqStopped"),
    (0x0027_aa80, script_engine_is_finish_script_active, "ScriptEngine_IsFinishScriptActive"),
    (0x0027_aaf8, script_engine_queue_intro_scripts, "ScriptEngine_QueueIntroScripts"),
    (0x0027_b370, script_engine_load_bank_async, "ScriptEngine_LoadBankAsync"),
    (0x0027_b410, script_engine_unload_bank, "ScriptEngine_UnloadBank"),
    (0x0027_b4b8, script_engine_cancel_bank_load, "ScriptEngine_CancelBankLoad"),
    (0x0027_b528, script_engine_find_free_bank_slot, "ScriptEngine_FindFreeBankSlot"),
    (0x0027_b578, script_engine_find_bank_slot_by_id, "ScriptEngine_FindBankSlotById"),
    (0x0027_b5d8, script_engine_on_bank_loaded, "ScriptEngine_OnBankLoaded"),
    (0x0027_b6b0, script_engine_on_filter_loaded, "ScriptEngine_OnFilterLoaded"),
    (0x0027_bd78, script_engine_pop_free_node, "ScriptEngine_PopFreeNode"),
    (0x0027_bd90, script_engine_push_free_node, "ScriptEngine_PushFreeNode"),
    (0x0027_bf50, script_engine_is_bank_slot_loaded, "ScriptEngine_IsBankSlotLoaded"),
    (0x0027_bf90, script_engine_exit_script_mode, "ScriptEngine_ExitScriptMode"),
    (0x0027_bfd0, script_dummy_obj_ctor, "ScriptDummyObj_ctor"),
    (0x0027_c020, script_dummy_obj_deactivate, "ScriptDummyObj_Deactivate"),
    (0x0027_c040, script_dummy_obj_accept_command, "ScriptDummyObj_AcceptCommand"),
    (0x0027_c050, script_dummy_obj_apply, "ScriptDummyObj_Apply"),
    (0x0027_c0f0, script_cam_read_manual_fields, "ScriptCam_ReadManualFields"),
    (0x0027_c300, script_cam_read_target_fields, "ScriptCam_ReadTargetFields"),
    (0x0027_c510, script_cam_read_subject_fields, "ScriptCam_ReadSubjectFields"),
    (0x0027_c6f8, script_cam_read_algorithmic_fields, "ScriptCam_ReadAlgorithmicFields"),
    (0x0027_c8e0, script_view_read_fields, "ScriptView_ReadFields"),
    (0x0027_cbb0, script_dummy_obj_read_fields, "ScriptDummyObj_ReadFields"),
    (0x0027_cc28, script_audio_read_fields, "ScriptAudio_ReadFields"),
    (0x0027_cdd0, script_engine_obj_table_ctor, "ScriptEngine_ObjTableCtor"),
    (0x0027_ce58, script_engine_obj_table_dtor, "ScriptEngine_ObjTableDtor"),
    (0x0027_cea8, script_engine_add_script_obj, "ScriptEngine_AddScriptObj"),
    (0x0027_cf88, script_engine_remove_script_obj, "ScriptEngine_RemoveScriptObj"),
    (0x0027_d0c0, script_engine_set_obj_binding_filter, "ScriptEngine_SetObjBindingFilter"),
    (0x0027_d170, script_engine_clear_obj_binding_filter, "ScriptEngine_ClearObjBindingFilter"),
    (0x0027_d2c8, script_engine_binding_has_process, "ScriptEngine_BindingHasProcess"),
    (0x0027_d2e8, script_engine_get_instance, "ScriptEngine_GetInstance"),
    (0x0027_d2f0, script_engine_get_binding_process, "ScriptEngine_GetBindingProcess"),
    (0x0027_d320, script_engine_get_script_obj, "ScriptEngine_GetScriptObj"),
    (0x0027_d390, script_engine_remove_binding, "ScriptEngine_RemoveBinding"),
    (0x0027_d400, script_engine_activate_binding, "ScriptEngine_ActivateBinding"),
    (0x0027_d4a0, script_engine_deactivate_binding, "ScriptEngine_DeactivateBinding"),
    (0x0027_d540, script_engine_set_binding_filter, "ScriptEngine_SetBindingFilter"),
    (0x0027_d558, script_engine_clear_binding_filter, "ScriptEngine_ClearBindingFilter"),
    (0x0027_d578, script_engine_collect_free_bindings, "ScriptEngine_CollectFreeBindings"),
    (0x0027_d5e8, script_obj_cam_ctor, "ScriptObjCam_ctor"),
    (0x0027_d688, script_obj_cam_set_binding, "ScriptObjCam_SetBinding"),
    (0x0027_d790, script_obj_cam_is_bound_to_process, "ScriptObjCam_IsBoundToProcess"),
    (0x0027_d7f0, script_obj_cam_v02, "ScriptObjCam_v02"),
    (0x0027_d820, script_obj_cam_v03, "ScriptObjCam_v03"),
    (0x0027_d850, script_obj_cam_activate, "ScriptObjCam_Activate"),
    (0x0027_d8d8, script_obj_cam_deactivate, "ScriptObjCam_Deactivate"),
    (0x0027_f968, rider_script_obj_ctor, "RiderScriptObj_ctor"),
    (0x0028_03f0, rider_script_obj_deactivate, "RiderScriptObj_Deactivate"),
    (0x0028_0458, rider_script_obj_accept_command, "RiderScriptObj_AcceptCommand"),
    (0x0028_0560, rider_script_obj_matches_rider_slot, "RiderScriptObj_MatchesRiderSlot"),
    (0x0028_05b8, script_audio_obj_ctor, "ScriptAudioObj_ctor"),
    (0x0028_0730, script_audio_obj_activate, "ScriptAudioObj_Activate"),
    (0x0028_08d8, script_audio_obj_accept_command, "ScriptAudioObj_AcceptCommand"),
    (0x0028_1100, script_view_obj_update, "ScriptViewObj_Update"),
    (0x0028_1170, script_view_obj_get_binding, "ScriptViewObj_GetBinding"),
    (0x0028_1190, script_view_obj_add_cam, "ScriptViewObj_AddCam"),
    (0x0028_12e0, script_view_obj_set_transition, "ScriptViewObj_SetTransition"),
    (0x0028_1a00, script_view_obj_accept_command, "ScriptViewObj_AcceptCommand"),
    (0x0028_1a50, script_view_obj_reset, "ScriptViewObj_Reset"),
    (0x0028_1c40, script_view_obj_get_slot_cam, "ScriptViewObj_GetSlotCam"),
    (0x0028_1e80, script_engine_script_db_ctor, "ScriptEngine_ScriptDbCtor"),
    (0x0028_1ed8, script_engine_script_db_dtor, "ScriptEngine_ScriptDbDtor"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::object::debouncenode::callee::TRANSITION_MAN_STOP;
    pub(crate) use crate::game::object::debouncenode::callee::SPLINE_MODIFIER_IS_NODE_VISIBLE;
    pub(crate) use crate::game::object::debouncenode::callee::SCRIPT_VIEW_OBJ_BIND_CAM_SLOT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
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
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_VISIBLE_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_FIND_ROSTER_ENTRY;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BUDDY_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BLOCKED_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_TEMP_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_ONLINE_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_SAME_PRODUCT_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_COUNT_MESSAGES_FROM_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_INSTANCE_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_ADD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_SKIP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_EMPTY_CROWD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_RIDER_SAY_20B9;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_TRICK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_FALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_STOP_ANTICIPATE;
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
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS as SCRIPT_ENGINE_STOP_MAIN_LISTS_2;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS as SCRIPT_ENGINE_STOP_AUX_LISTS_2;
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
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
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
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT as LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_3;
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
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;
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
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_3;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_3;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
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
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT as LOBBY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT as BUDDY_MAN_DISCONNECT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT as NET_FE_STATE_HANDLE_NET_EVENT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_4;
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
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS as RIDER_METRIX_RESET_STATS_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION_3;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_4;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF_4;
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
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_4;
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
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_5;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
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
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_5;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_6;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR as UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2 as UI_OBJECT_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_7;
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
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST as LOBBY_MAN_SEND_REQUEST_2;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_3;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_3;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE as SCRIPT_OBJ_DEACTIVATE_2;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_5;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT as SCRIPT_ENGINE_CHOOSE_SCRIPT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_8;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2;
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
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2 as IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2 as GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
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


    pub const OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO: Callee = Callee {
        addr: 0x0021_1380,
        f: ee::s05::OVScreenProfileSavePDA_RefreshCardInfo,
    };

    pub const OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST: Callee = Callee {
        addr: 0x0021_1270,
        f: ee::s05::OVScreenProfileSavePDA_ResetSlotList,
    };

    pub const GAME_STATE_GOTO_NEXT_3: Callee = Callee {
        addr: 0x0023_3aa0,
        f: ee::s06::GameState_GotoNext,
    };

    pub const VID_ENGINE_INSTALL_MEM_HOOKS: Callee = Callee {
        addr: 0x0025_3aa0,
        f: ee::s06::VidEngine_InstallMemHooks,
    };

    pub const VID_ENGINE_PLAYER_INIT_OPEN_PARAMS: Callee = Callee {
        addr: 0x0025_3390,
        f: ee::s06::VidEnginePlayer_InitOpenParams,
    };

    pub const BUDDY_MAN_CLEAR_LOGIN: Callee = Callee {
        addr: 0x0026_1058,
        f: ee::s06::BuddyMan_ClearLogin,
    };

    pub const BUDDY_MAN_CLEAR_MESSAGES: Callee = Callee {
        addr: 0x0026_1070,
        f: ee::s06::BuddyMan_ClearMessages,
    };

    pub const BUDDY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::BuddyMan_Disconnect,
    };

    pub const REPLAY_SAVE_EMIT: Callee = Callee {
        addr: 0x0027_1058,
        f: ee::s07::Replay_SaveEmit,
    };

    pub const SCRIPT_DATA_GET_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::ScriptData_GetInfo,
    };

    pub const SCRIPT_ENGINE_LIST_STOP_2: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };

    pub const SCRIPT_ENGINE_GET_INSTANCE_SCRIPT: Callee = Callee {
        addr: 0x0027_a668,
        f: ee::s07::ScriptEngine_GetInstanceScript,
    };

    pub const SCRIPT_BINDING_CTOR_2: Callee = Callee {
        addr: 0x0028_2f60,
        f: ee::s07::ScriptBinding_ctor,
    };

    pub const SCRIPT_OBJ_CTOR: Callee = Callee {
        addr: 0x0028_3200,
        f: ee::s07::ScriptObj_ctor,
    };

    pub const SSX_AUDIO_SYSTEM_DJ_SAY_20E5: Callee = Callee {
        addr: 0x002a_26f0,
        f: ee::s08::SSXAudioSystem_DJSay20E5,
    };

    pub const SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE: Callee = Callee {
        addr: 0x002b_1758,
        f: ee::s08::SpeechSystem_CancelPendingInstance,
    };

    pub const MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE: Callee = Callee {
        addr: 0x002b_4908,
        f: ee::s08::MusicPlayer_GetSelectedSongSedValue,
    };

    pub const BANK_MONITOR_RESET_INSTANCE: Callee = Callee {
        addr: 0x002a_d398,
        f: ee::s08::BankMonitor_ResetInstance,
    };

    pub const C_TRANSITION_MAN_START: Callee = Callee {
        addr: 0x002e_4370,
        f: ee::s09::cTransitionMan_Start,
    };

    pub const WSCRIPT_MAN_DEACTIVATE_PROCESS: Callee = Callee {
        addr: 0x0030_9aa0,
        f: ee::s10::WScriptMan_DeactivateProcess,
    };

    pub const WSCRIPT_MAN_UNLOAD_SECTION: Callee = Callee {
        addr: 0x0030_8f70,
        f: ee::s10::WScriptMan_UnloadSection,
    };

    pub const C_BX_STRING_COPY_BEFORE_WRITE: Callee = Callee {
        addr: 0x0031_8120,
        f: ee::s10::cBXString_CopyBeforeWrite,
    };

    pub const WORLD_OBJ_SAVE: Callee = Callee {
        addr: 0x0034_fe90,
        f: ee::s11::WorldObj_Save,
    };

    pub const SYNC_PROCESS_SAVE: Callee = Callee {
        addr: 0x0035_4980,
        f: ee::s11::SyncProcess_Save,
    };

    pub const ANIM_OBJECT_SAVE: Callee = Callee {
        addr: 0x0034_2150,
        f: ee::s11::AnimObject_Save,
    };

    pub const MOD_OBJ_CREATE_MODIFIER_BLOCK: Callee = Callee {
        addr: 0x0035_5468,
        f: ee::s11::ModObj_CreateModifierBlock,
    };

    pub const RAIL_MODIFIER_LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x0035_b670,
        f: ee::s11::RailModifier_ListPushFront,
    };

    pub const INTC_TABLE_ADD_HANDLER: Callee = Callee {
        addr: 0x0036_1f98,
        f: ee::s12::IntcTable_AddHandler,
    };


    pub const APP_VSYNC_CALLBACK: Callee = Callee {
        addr: 0x0031_7500,
        f: ee::s10::App_VSyncCallback,
    };

    pub const INPUT_PREPROC_WRITE: Callee = Callee {
        addr: 0x0032_6360,
        f: ee::s10::InputPreproc_Write,
    };

    pub const WORLD_OBJ_CTOR: Callee = Callee {
        addr: 0x0034_fb00,
        f: ee::s11::WorldObj_ctor,
    };

    pub const WORLD_OBJ_DTOR: Callee = Callee {
        addr: 0x0034_fbf0,
        f: ee::s11::WorldObj_dtor,
    };

    pub const MODIFIER_BLOCK_FREE_TEX_FLIP: Callee = Callee {
        addr: 0x0035_3150,
        f: ee::s11::ModifierBlock_FreeTexFlip,
    };

    pub const MODIFIER_BLOCK_FREE_BOOST: Callee = Callee {
        addr: 0x0035_3188,
        f: ee::s11::ModifierBlock_FreeBoost,
    };

    pub const MULTI_PARTICLE_DTOR: Callee = Callee {
        addr: 0x0035_7fa0,
        f: ee::s11::MultiParticle_dtor,
    };

    pub const DYNAMIC_EMITTER_FREE_DATA_2: Callee = Callee {
        addr: 0x0037_0cf8,
        f: ee::s12::DynamicEmitter_FreeData,
    };

    pub const RCMP_DECODER_RELEASE_CODEC: Callee = Callee {
        addr: 0x003b_0538,
        f: ee::s13::RCMP_DECODER_ReleaseCodec,
    };

    pub const RIDER_METRIX_AWARD_BONUS_POINTS: Callee = Callee {
        addr: 0x0011_9608,
        f: ee::s00::RiderMetrix_AwardBonusPoints,
    };

    pub const RIDER_ADD_BOOST: Callee = Callee {
        addr: 0x0010_e098,
        f: ee::s00::Rider_AddBoost,
    };

    pub const RIDER_METRIX_SHOW_HUD_NUMBER_2: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::RiderMetrix_ShowHUDNumber,
    };

    pub const RIDER_METRIX_AWARD_POINTS: Callee = Callee {
        addr: 0x0011_9ef8,
        f: ee::s00::RiderMetrix_AwardPoints,
    };

    pub const AI_CREATE_ACTOR_RIDERS: Callee = Callee {
        addr: 0x0012_9ff8,
        f: ee::s00::AI_CreateActorRiders,
    };

    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };

    pub const CHASE_CAM_CONTROLLER_RESET_STATE: Callee = Callee {
        addr: 0x0016_2138,
        f: ee::s01::ChaseCamController_ResetState,
    };

    pub const ACTIVE_CAMERA_TRIGGER_LIST_CLEAR: Callee = Callee {
        addr: 0x0016_c628,
        f: ee::s02::ActiveCameraTriggerList_Clear,
    };

    pub const CAMERA_TRIGGER_STACK_CLEAR_2: Callee = Callee {
        addr: 0x0016_d200,
        f: ee::s02::CameraTriggerStack_Clear,
    };

    pub const CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS: Callee = Callee {
        addr: 0x0016_cbe0,
        f: ee::s02::CameraTriggerMan_RemoveAllTriggers,
    };

    pub const CAMERA_TRIGGER_MAN_CLEAR_DATA: Callee = Callee {
        addr: 0x0016_bf20,
        f: ee::s02::CameraTriggerMan_ClearData,
    };

    pub const CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION: Callee = Callee {
        addr: 0x0018_d108,
        f: ee::s02::cFEStateOptionsController_SetVibration,
    };

    pub const FE_LOCALE_UNLOAD_ALL_TABLES: Callee = Callee {
        addr: 0x0019_5948,
        f: ee::s02::FELocale_UnloadAllTables,
    };

    pub const CFE_STATE_NET_ACCOUNT_MEM_SET_STATE: Callee = Callee {
        addr: 0x001b_70d0,
        f: ee::s03::cFEStateNetAccountMem_SetState,
    };

    pub const NET_FE_STATE_DTOR_3: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::NetFEState_dtor,
    };

    pub const CFE_LOAD_STATE_SETUP_RESULT_WIDGET: Callee = Callee {
        addr: 0x0024_7ab0,
        f: ee::s06::cFELoadState_SetupResultWidget,
    };

    pub const EVENT_SELECT_ON_WIDGET_ACTION: Callee = Callee {
        addr: 0x0020_18a8,
        f: ee::s05::EventSelect_OnWidgetAction,
    };

    pub const PLAYBACK_MAN_STOP_PLAYBACK: Callee = Callee {
        addr: 0x0026_cd20,
        f: ee::s07::PlaybackMan_StopPlayback,
    };

    pub const PLAYBACK_MAN_STOP_RECORDING: Callee = Callee {
        addr: 0x0026_cdd0,
        f: ee::s07::PlaybackMan_StopRecording,
    };

    pub const REPLAY_CACHE_DTOR: Callee = Callee {
        addr: 0x0026_d0e8,
        f: ee::s07::ReplayCache_dtor,
    };

    pub const SCRIPT_LIST_STOP_2: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::ScriptList_Stop,
    };

    pub const SCRIPT_ENGINE_EXIT_SCRIPT_MODE: Callee = Callee {
        addr: 0x0027_bf90,
        f: ee::s07::ScriptEngine_ExitScriptMode,
    };

    pub const SCRIPT_ENGINE_GET_SEQ_PROCESS_ID: Callee = Callee {
        addr: 0x0028_2be0,
        f: ee::s07::ScriptEngine_GetSeqProcessId,
    };

    pub const SCRIPT_LIST_ON_SEQ_END: Callee = Callee {
        addr: 0x0027_73a0,
        f: ee::s07::ScriptList_OnSeqEnd,
    };

    pub const SCRIPT_ENGINE_GET_TARGET_OBJECT: Callee = Callee {
        addr: 0x0027_9f18,
        f: ee::s07::ScriptEngine_GetTargetObject,
    };

    pub const SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2: Callee = Callee {
        addr: 0x0028_2020,
        f: ee::s07::ScriptEngine_DestroyScriptDatabase,
    };

    pub const SCRIPT_ENGINE_OBJ_TABLE_DTOR: Callee = Callee {
        addr: 0x0027_ce58,
        f: ee::s07::ScriptEngine_ObjTableDtor,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_2: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::ScriptEngine_GetProcess,
    };

    pub const IN_SEQ_STOP: Callee = Callee {
        addr: 0x0027_2b58,
        f: ee::s07::InSeq_Stop,
    };

    pub const SND_BANK_UNLOAD_MEMORY_2: Callee = Callee {
        addr: 0x0028_b7e0,
        f: ee::s07::SndBank_UnloadMemory,
    };


    pub const BUILD_BUDDY_LIST: Callee =
        Callee { addr: 0x001a_5898, f: ee::s03::cFEStateNetMessenger_BuildBuddyList };

    pub const REFRESH_PLAYER_LIST: Callee =
        Callee { addr: 0x001a_6978, f: ee::s03::cFEStateNetMessenger_RefreshPlayerList };

    pub const SET_SELECTED_USER: Callee =
        Callee { addr: 0x001a_9890, f: ee::s03::NetFEState_SetSelectedUser };

    pub const GET_SELECTED_USER: Callee =
        Callee { addr: 0x001a_98b0, f: ee::s03::NetFEState_GetSelectedUser };

    pub const SHOW_BUDDY_ACTION_POPUP: Callee =
        Callee { addr: 0x001a_b370, f: ee::s03::NetFEState_ShowBuddyActionPopup };

    pub const ON_CHILD_EVENT: Callee =
        Callee { addr: 0x001a_97b8, f: ee::s03::NetFEState_OnChildEvent };

    pub const GOTO_NEXT_STATE: Callee =
        Callee { addr: 0x001a_8a40, f: ee::s03::NetFEState_GotoNextState };

    pub const HANDLE_NET_EVENT: Callee =
        Callee { addr: 0x001a_8e40, f: ee::s03::NetFEState_HandleNetEvent };

    pub const ONLINE_CHARACTER_SELECT_CTOR: Callee =
        Callee { addr: 0x001b_0f60, f: ee::s03::cFEStateOnlineCharacterSelect_ctor };

    pub const CHANNEL_GET_TRACK: Callee =
        Callee { addr: 0x0027_42c8, f: ee::s07::InSeq_ChannelGetTrack };

    pub const LIST_GET_ENTRY_TYPE: Callee =
        Callee { addr: 0x0027_9148, f: ee::s07::ScriptEngine_ListGetEntryType };

    pub const FIND_FREE_BANK_SLOT: Callee =
        Callee { addr: 0x0027_b528, f: ee::s07::ScriptEngine_FindFreeBankSlot };

    pub const LOAD_BANK_ASYNC: Callee =
        Callee { addr: 0x0027_b370, f: ee::s07::ScriptEngine_LoadBankAsync };

    pub const REQUEST_IMAGE: Callee =
        Callee { addr: 0x001d_2150, f: ee::s04::cFEStatePreviewReward_RequestImage };

    pub const SEND_PACKET: Callee = Callee { addr: 0x0026_4be8, f: ee::s07::cSetupMan_SendPacket };

    pub const BANK_MONITOR_RESET: Callee =
        Callee { addr: 0x002a_bb38, f: ee::s08::BankMonitorInstance_Reset };

    pub const RIDER_SAY_20B9: Callee =
        Callee { addr: 0x002a_1400, f: ee::s08::SSXAudioSystem_RiderSay20B9 };

    pub const STOP_BOARD_LOOPS: Callee =
        Callee { addr: 0x0029_2b48, f: ee::s07::SSXAudioSystem_StopBoardLoops };

    pub const START_RIDER_BOARD_LOOPS: Callee =
        Callee { addr: 0x0029_2a50, f: ee::s07::SSXAudioSystem_StartRiderBoardLoops };

    pub const GET_RACE_MAN: Callee = Callee { addr: 0x0028_b1d8, f: ee::s07::SSXAudioSystem_GetRaceMan };

    pub const LEVEL_TO_VOLUME: Callee =
        Callee { addr: 0x0028_73d8, f: ee::s07::SSXAudioSystem_LevelToVolume };

    pub const SET_CHANNEL_VOLUME: Callee =
        Callee { addr: 0x0028_7700, f: ee::s07::SSXAudioSystem_SetChannelVolume };

    pub const GET_CHANNEL_VOLUME: Callee =
        Callee { addr: 0x0028_7920, f: ee::s07::SSXAudioSystem_GetChannelVolume };

    pub const AUDIO_SYSTEM_CTOR: Callee =
        Callee { addr: 0x0028_4c68, f: ee::s07::SSXAudioSystem_ctor };

    pub const STOP_TRANSITION: Callee =
        Callee { addr: 0x0028_2db0, f: ee::s07::cTransitionMan_StopTransition };

    pub const REPLAY_END_SAVE: Callee = Callee { addr: 0x0027_0ed8, f: ee::s07::Replay_EndSave };

    pub const REPLAY_SET_CONTROLLER_INDEX: Callee =
        Callee { addr: 0x0026_f898, f: ee::s07::Replay_SetControllerIndex };

    pub const UI_STATE_CTOR_2_2: Callee = Callee { addr: 0x0039_e318, f: ee::s13::UIState_ctor_2 };

    pub const UI_LIST_CTOR_2: Callee = Callee { addr: 0x0039_77e8, f: ee::s13::UIList_ctor };

    pub const UI_MENU_SET_SELECTION: Callee =
        Callee { addr: 0x0039_b960, f: ee::s13::UIMenu_SetSelection };

    pub const MEM_CARD_BASE_CTOR: Callee =
        Callee { addr: 0x001d_53b0, f: ee::s04::MemCardStateBase_ctor };

    pub const MEM_CARD_ON_CHILD_EVENT: Callee =
        Callee { addr: 0x001d_8700, f: ee::s04::MemCardStateBase_OnChildEvent };

    pub const RIDER_HAS_RENDER_SLOT: Callee =
        Callee { addr: 0x0011_c0e0, f: ee::s00::Rider_HasRenderSlot };


    pub const SETUP_MAN_SET_LOCAL_CHARACTER: Callee =
        Callee { addr: 0x0026_4cf0, f: ee::s07::f_00264cf0 };


    pub const CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES: Callee =
        Callee { addr: 0x002a_6430, f: ee::s08::f_002a6430 };


    pub const SCRIPT_LIST_START: Callee = Callee { addr: 0x0027_6388, f: ee::s07::f_00276388 };

    pub const SCRIPT_LIST_CAN_SKIP: Callee =
        Callee { addr: 0x0027_7060, f: ee::s07::f_00277060 };

    pub const SCRIPT_LIST_GET_PLAY_STATUS_2: Callee =
        Callee { addr: 0x0027_6bd8, f: ee::s07::f_00276bd8 };


    pub const SCRIPT_BINDING_CTOR_4: Callee = Callee { addr: 0x0028_2f60, f: ee::s07::f_00282f60 };

    pub const SCRIPT_OBJ_CTOR_3: Callee = Callee { addr: 0x0028_3200, f: ee::s07::f_00283200 };


    pub const TRANSITION_MAN_STOP_TRANSITION: Callee =
        Callee { addr: 0x0028_2db0, f: ee::s07::f_00282db0 };

    pub const TRANSITION_MAN_SCRIPT_SET_OWNER: Callee =
        Callee { addr: 0x0028_2da8, f: ee::s07::f_00282da8 };


    pub const PATH_PLAT_FIND_BANK_SLOT: Callee =
        Callee { addr: 0x002a_e230, f: ee::s08::f_002ae230 };


    pub const MENU_SYS_ITEM_DRAW_ROW_4: Callee = Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };


    pub const MATH_EVAL_CUBIC: Callee = Callee { addr: 0x0027_2570, f: ee::s07::f_00272570 };


    pub const LINE_STRIP_CLOSE: Callee = Callee { addr: 0x0038_4dc0, f: ee::s12::f_00384dc0 };

    pub const LINE_STRIP_ADD_VERTEX: Callee = Callee { addr: 0x0038_4fd0, f: ee::s12::f_00384fd0 };


    pub const KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW: Callee =
        Callee { addr: 0x001c_c8d8, f: ee::s04::f_001cc8d8 };

    pub const KEYBOARD_POPUP_MASK_TEXT: Callee =
        Callee { addr: 0x001c_ce98, f: ee::s04::f_001cce98 };

    pub const KEYBOARD_POPUP_UPDATE_CURSOR_2: Callee =
        Callee { addr: 0x001c_cc30, f: ee::s04::f_001ccc30 };


    pub const INPUT_SYNC_PACK_SAMPLE: Callee = Callee { addr: 0x0023_7b28, f: ee::s06::f_00237b28 };

    pub const INPUT_SYNC_UNPACK_SAMPLE: Callee = Callee { addr: 0x0023_7be8, f: ee::s06::f_00237be8 };


    pub const SND_BANK_IS_LOADED_WITH_NAME: Callee =
        Callee { addr: 0x0028_b830, f: ee::s07::f_0028b830 };

    pub const SND_LOAD_QUEUE_CANCEL_BANK: Callee =
        Callee { addr: 0x0028_a728, f: ee::s07::f_0028a728 };

    pub const SND_BANK_LOAD: Callee = Callee { addr: 0x0028_b320, f: ee::s07::f_0028b320 };


    pub const BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE: Callee =
        Callee { addr: 0x0015_7bf0, f: ee::s01::f_00157bf0 };


    pub const VEC3_NEG_DOT: Callee = Callee { addr: 0x0033_00f8, f: ee::s11::f_003300f8 };


    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT_3: Callee =
        Callee { addr: 0x0027_71c8, f: ee::s07::f_002771c8 };

    pub const SCRIPT_DATA_GET_INFO_4: Callee = Callee { addr: 0x0027_c070, f: ee::s07::f_0027c070 };

    pub const IN_SEQ_INSTANCE_GET_REMAINING_STEPS: Callee =
        Callee { addr: 0x0027_4d08, f: ee::s07::f_00274d08 };

    pub const SCRIPT_LIST_START_TRANSITION: Callee =
        Callee { addr: 0x0027_7980, f: ee::s07::f_00277980 };


    pub const CBX_STRING_CONCAT_COPY: Callee = Callee { addr: 0x0031_8540, f: ee::s10::f_00318540 };

    pub const CBX_STRING_CTOR_COPY: Callee = Callee { addr: 0x0031_7fe8, f: ee::s10::f_00317fe8 };

    pub const CBX_STRING_DTOR: Callee = Callee { addr: 0x0031_8260, f: ee::s10::f_00318260 };


    pub const ATTACHED_DYNAMIC_EMITTER_CREATE: Callee =
        Callee { addr: 0x002f_2130, f: ee::s09::f_002f2130 };

    pub const CROWD_MAN2D_CLEAR_SLOT: Callee = Callee { addr: 0x0022_9b90, f: ee::s05::f_00229b90 };

    pub const REPLAY_STREAM_REWIND_READ: Callee =
        Callee { addr: 0x0026_e9b0, f: ee::s07::f_0026e9b0 };

    pub const CAMERA_ALGORITHM_DEFAULT_FOV: Callee =
        Callee { addr: 0x0016_7e30, f: ee::s02::f_00167e30 };

    pub const BE_GET_RIDER_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ee::s01::f_0014a080 };


    pub const GAME_PROCESS_CTOR: Callee = Callee { addr: 0x0035_4648, f: ee::s11::f_00354648 };

    pub const C_FLAG_MANAGER_GROUP_CTOR: Callee =
        Callee { addr: 0x0034_af38, f: ee::s11::f_0034af38 };


    pub const MC_MANAGER_GET: Callee = Callee { addr: 0x0022_7f80, f: ee::s05::f_00227f80 };


    pub const WORLD_OBJ_REPLACE_WITH_DEAD_NODE: Callee =
        Callee { addr: 0x0034_fe28, f: ee::s11::f_0034fe28 };


    pub const FE_LOCALE_UNLOAD_TABLE: Callee =
        Callee { addr: 0x0019_5a50, f: ee::s02::f_00195a50 };


    pub const UI_LIST_FIRST_5: Callee = Callee { addr: 0x0039_7840, f: ee::s13::f_00397840 };

    pub const UI_ASCII_TO_WIDE: Callee = Callee { addr: 0x0039_7ad8, f: ee::s13::f_00397ad8 };

    pub const UI_TEXT_SET_WIDE_STRING_INTERNAL: Callee =
        Callee { addr: 0x003a_0eb0, f: ee::s13::f_003a0eb0 };


    pub const BELIBRARY_DTOR: Callee = Callee { addr: 0x0014_99a8, f: ee::s01::f_001499a8 };


    pub const MEM_MAN_FREE_ENTRY: Callee = Callee { addr: 0x0031_7e98, f: ee::s10::f_00317e98 };


    pub const UI_SCREEN_FIND_OBJECT_3: Callee =
        Callee { addr: 0x0039_d860, f: ee::s13::f_0039d860 };


    /// `InSeq_Channel* InSeq_ChannelGetTrack(InSeq_Channel*, int index)`.
    pub const IN_SEQ_CHANNEL_GET_TRACK: Callee =
        Callee { addr: 0x0027_42c8, f: ee::s07::f_002742c8 };


    /// `int CameraTrigger_ReadActionTypeN(CameraTrigger*, Action*)`.
    pub const TYPE0: Callee = Callee { addr: 0x0017_15a0, f: ee::s02::f_001715a0 };

    pub const TYPE1: Callee = Callee { addr: 0x0017_14e8, f: ee::s02::f_001714e8 };

    pub const TYPE2: Callee = Callee { addr: 0x0017_15f8, f: ee::s02::f_001715f8 };

    pub const TYPE3: Callee = Callee { addr: 0x0017_1680, f: ee::s02::f_00171680 };


    /// `void RiderCameraAlgorithm_ctor(RiderCameraAlgorithm*)`.
    pub const RIDER_CAMERA_ALGORITHM_CTOR: Callee =
        Callee { addr: 0x0016_22b0, f: ee::s01::f_001622b0 };
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
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;

    pub const STR_HASH: u32 = 0x0031_7670;

    pub const UI_ENGINE_CREATE_SCREEN: u32 = 0x0039_7cc0;

    pub const UI_SCREEN_START_THREAD: u32 = 0x0039_c870;

    pub const CLOSE_UNPINNED_POPUPS: u32 = 0x0039_f600;

    pub const STRCASECMP: u32 = 0x0041_aa88;

    /// `strcmp`.
    pub const STRCMP: u32 = 0x0041_65a8;

    pub const STRLEN: u32 = 0x0041_6810;

    pub const MEMSET: u32 = 0x003e_6448;

    pub const BE_GET_BE: u32 = 0x0014_dc80;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const CHASE_CAMERA_RESET_FOR_RIDER: u32 = 0x0016_6c60;

    pub const MENU_POP_PAGE: u32 = 0x002c_c460;

    pub const MENU_PUSH_PAGE: u32 = 0x002c_c3b8;

    pub const FE_NET_SEND: u32 = 0x0025_aae0;

    pub const MENU_DTOR_2: u32 = 0x002c_a280;

    pub const MENU_DTOR: u32 = 0x002c_aa80;

    pub const WORLD_GET: u32 = 0x0022_7f80;

    pub const MC_STATUS_TEXT: u32 = 0x0024_1e18;

    pub const RIDER_PREVIEW: u32 = 0x001a_0548;

    pub const RIDER_PREVIEW_BOLTS: u32 = 0x0019_fbe0;

    pub const BE_GET_INTERFACE_9: u32 = 0x0014_ad28;

    pub const HUD_SHOW_NUMBER_DELAYED: u32 = 0x0011_71a8;

    pub const SPATIAL_INSERT: u32 = 0x0032_91e0;

    pub const POSITION_MODIFIER_CTOR_3: u32 = 0x0035_6f68;

    pub const WORLD_PAINTER_2D: u32 = 0x002d_1be0;

    pub const UI_OBJECT_BY_HASH: u32 = 0x0039_d860;

    pub const BANK_MONITOR_ADD: u32 = 0x002a_dbf0;
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


// ---------------------------------------------------------------------------
// Script engine: free-node list.

/// `ScriptNode *ScriptEngine_PopFreeNode(ScriptEngine *engine)` (0x27bd78):
/// pop `engine->freeNodes` (+0x548), returning the node and clearing its link
/// (+0x10).
pub fn script_engine_pop_free_node(c: &mut Cpu) {
    c.cycles += 6;
    const FREE: u32 = 0x548;
    const NEXT: u32 = 0x10;
    let engine = Addr(abi::arg(c, 0));
    let node = engine.off(FREE).lw(c);
    c.g[V1][0] = node;
    let next = Addr(node as u32).off(NEXT).lw(c);
    c.g[A1][0] = next;
    c.g[V0][0] = node;
    engine.off(FREE).sw(c, next);
    Addr(node as u32).off(NEXT).sw(c, 0);
    abi::ret(c);
}


/// `void ScriptEngine_PushFreeNode(ScriptEngine *engine, ScriptNode *node)`
/// (0x27bd90): push `node` onto `engine->freeNodes` (+0x548), setting its
/// fields to -1 and threading the old head through +0x10.
pub fn script_engine_push_free_node(c: &mut Cpu) {
    c.cycles += 9;
    const FREE: u32 = 0x548;
    const NEXT: u32 = 0x10;
    let engine = Addr(abi::arg(c, 0));
    let node = Addr(abi::arg(c, 1));
    let head = engine.off(FREE).lw(c);
    c.g[V1][0] = head;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    node.off(0xc).sw(c, c.g[V0][0]);
    node.off(NEXT).sw(c, head);
    node.sw(c, c.g[V0][0]);
    node.off(0x4).sw(c, c.g[V0][0]);
    node.off(0x8).sw(c, c.g[V0][0]);
    engine.off(FREE).sw(c, abi::arg64(c, 1));
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Script engine: bindings.

/// `ScriptProcess *ScriptEngine_GetBindingProcess(ScriptEngine *engine,
/// const Binding *binding)` (0x27d2f0): the process of the binding at
/// `engine->bindings (+0x50) + binding->index (+0x4)*0x14`, returned only when
/// its state (+0xc) is 1, else -1.
pub fn script_engine_get_binding_process(c: &mut Cpu) {
    c.cycles += 9;
    const BINDINGS: u32 = 0x50;
    const BINDING_SIZE: u32 = 0x14;
    const STATE: u32 = 0xc;
    const PROCESS: u32 = 0x8;
    let engine = abi::arg(c, 0);
    let index = Addr(abi::arg(c, 1)).off(0x4).lw(c);
    c.g[V0][0] = index;
    c.g[V1][0] = BINDING_SIZE as u64;
    c.g[A1][0] = 0x1;
    let offset = abi::mult(c, index as u32, BINDING_SIZE);
    let binding = engine.wrapping_add(offset).wrapping_add(BINDINGS);
    c.g[V0][0] = sx(binding);
    c.g[A0][0] = sx(binding);
    c.g[V1][0] = Addr(binding).off(STATE).lw(c);
    let mut result = 0xffff_ffff_ffff_ffffu64;
    if c.g[V1][0] == 0x1 {
        c.cycles += 1;
        result = Addr(binding).off(PROCESS).lw(c);
    }
    c.g[V0][0] = result;
    c.cycles += 2;
    abi::ret(c);
}


/// `ScriptObj *ScriptEngine_GetScriptObj(ScriptEngine *engine, int index)`
/// (0x27d320): the script object pointer at `engine + index*4`.
pub fn script_engine_get_script_obj(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    abi::ret(c);
}


/// `void ScriptEngine_SetBindingFilter(ScriptEngine *engine, const Binding
/// *binding, int filter)` (0x27d540): store `filter` at
/// `engine + binding->index (+0x4)*0x14 + 0x60`.
pub fn script_engine_set_binding_filter(c: &mut Cpu) {
    c.cycles += 6;
    const BINDING_SIZE: u32 = 0x14;
    const FILTER: u32 = 0x60;
    let engine = abi::arg(c, 0);
    let index = Addr(abi::arg(c, 1)).off(0x4).lw(c);
    c.g[V0][0] = index;
    c.g[V1][0] = BINDING_SIZE as u64;
    let offset = abi::mult(c, index as u32, BINDING_SIZE);
    c.g[A1][0] = sx(offset);
    let binding = engine.wrapping_add(offset);
    c.g[V0][0] = sx(binding);
    Addr(binding).off(FILTER).sw(c, abi::arg64(c, 2));
    abi::ret(c);
}


/// `void ScriptEngine_ClearBindingFilter(ScriptEngine *engine, const Binding
/// *binding)` (0x27d558): store -1 at
/// `engine + binding->index (+0x4)*0x14 + 0x60`.
pub fn script_engine_clear_binding_filter(c: &mut Cpu) {
    c.cycles += 7;
    const BINDING_SIZE: u32 = 0x14;
    const FILTER: u32 = 0x60;
    let engine = abi::arg(c, 0);
    let index = Addr(abi::arg(c, 1)).off(0x4).lw(c);
    c.g[V0][0] = index;
    c.g[V1][0] = BINDING_SIZE as u64;
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    let offset = abi::mult(c, index as u32, BINDING_SIZE);
    c.g[A2][0] = sx(offset);
    let binding = engine.wrapping_add(offset);
    c.g[V0][0] = sx(binding);
    Addr(binding).off(FILTER).sw(c, 0xffff_ffff_ffff_ffff);
    abi::ret(c);
}

/// The game world (`D_004A28A8`, `gp-0x848`).
pub(crate) const WORLD: i32 = -0x848;


// ---------------------------------------------------------------------------
// ScriptViewObj_Update.

/// `void ScriptViewObj_Update(ScriptViewObj* self)` (0x281100): while the
/// object is active (`+8 > 0`), copy the bound rider's position (`rider+0x20`,
/// rider = `world->race->riders[+0xf8]`) to `+0x100`, and when it has no
/// camera yet (`+0x18 == 0`) bind one (`ScriptViewObj_BindCamSlot(self, 0, 0)`).
pub fn script_view_obj_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A2);
        c.g[V0][0] = Addr(this).off(8).lw(c);
        let idle = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = Addr(gp(c, WORLD)).lw(c);
        if !idle {
            c.cycles += 10;
            let world = r32(c, V0);
            c.g[V1][0] = Addr(this).off(0xf8).lw(c);
            c.g[A0][0] = Addr(world).off(0x84).lw(c);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[A1][0] = Addr(this).off(0x18).lw(c);
            c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[A0][0] = Addr(r32(c, V0)).off(4).lw(c);
            let rider = r32(c, A0);
            c.lq(3, p(rider, 0x20));
            let has_camera = c.g[A1][0] != 0;
            let dst = p(this, 0x100);
            c.sq(3, dst);
            if !has_camera {
                c.cycles += 4;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                callee::SCRIPT_VIEW_OBJ_BIND_CAM_SLOT.call(c, 0x0028_1150)?;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptEngine* ScriptEngine_GetInstance(void)` (0x27d2e8): the singleton
/// pointer at `D_004A34DC`.
pub fn script_engine_get_instance(c: &mut Cpu) {
    c.cycles += 2;
    let slot = gp_2(c, 0x3ec);
    c.g[V0][0] = c.lw(slot);
    abi::ret(c);
}


/// `int ScriptEngine_BindingHasProcess(this, int i, int process)`
/// (0x0027d2c8): whether binding `i`'s record at `+0x58` names `process`.
pub fn script_engine_binding_has_process(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0); // v1 = i * 20
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x58)).lw(c);
    let binding = c.g[V0][0] ^ c.g[A2][0];
    c.g[V0][0] = binding;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    c.pc = ra;
}


/// `void ScriptEngine_v12(this, unused, Node *node)` (0x00279ce0):
/// `ScriptEngine_PushFreeNode(this, node)`.
pub fn script_engine_v12(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A1][0] = c.g[A2][0],
        callee::SCRIPT_ENGINE_PUSH_FREE_NODE,
        0x0027_9cf0,
    )
}


/// `void ScriptDummyObj_Deactivate(this)` (0x0027c020):
/// `ScriptObj_Deactivate(&this->obj)` (`+0xc`).
pub fn script_dummy_obj_deactivate(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc)),
        callee::SCRIPT_OBJ_DEACTIVATE,
        0x0027_c030,
    )
}


/// `void ScriptDummyObj_Apply(this, void *args)` (0x0027c050): read the
/// scripted fields (`args`) into a stack buffer via `ScriptDummyObj_ReadFields`.
pub fn script_dummy_obj_apply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::SCRIPT_DUMMY_OBJ_READ_FIELDS, 0x0027_c064)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine lookups.

/// `int ScriptEngine_ListGetPlayStatus(ScriptEngine* eng, int list)`
/// (0x279220): the play status of list `list` (`&eng->lists[list]`).
pub fn script_engine_list_get_play_status(c: &mut Cpu) {
    script_list_lookup(c, callee::SCRIPT_LIST_GET_PLAY_STATUS, 0x0027_923c)
}


/// `int ScriptEngine_ListCount(ScriptEngine* eng, int list)` (0x279248): the
/// number of pending commands in list `list`.
pub fn script_engine_list_count(c: &mut Cpu) {
    script_list_lookup(c, callee::SCRIPT_LIST_COUNT, 0x0027_9264)
}


/// `callee(&eng->lists[list])` where `lists` is at `+0x54c` and each element
/// is 0xcc bytes.
pub(crate) fn script_list_lookup(c: &mut Cpu, target: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xcc;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        let base = Addr(arg(c, 0).wrapping_add(0x54c)).lw(c);
        c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
        target.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptEngine_FindScriptIndex(eng, category, ...)` (0x2797a0): the
/// chosen script's id, or -1.
pub fn script_engine_find_script_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SCRIPT_ENGINE_CHOOSE_SCRIPT.call(c, 0x0027_97b0)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
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


/// `int ScriptEngine_GetWorldSection2B(void)` (0x27a688): the race manager's
/// world section 0x2b.
pub fn script_engine_get_world_section2b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2b;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x78)).lw(c);
        call_ptr(c, callee::SECTION_MAN_GET_WORLD_SECTION_INDEX, 0x0027_a6a4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptViewObj_GetSlotCam(ScriptViewObj* self, int slot)` (0x281c40):
/// the camera object in view slot `slot` (`slot` indexes the 16-bit slot
/// table at `+0xdc`; negative entries mean "no camera").
pub fn script_view_obj_get_slot_cam(c: &mut Cpu) {
    c.cycles += 5;
    let obj = arg(c, 0);
    c.g[A1][0] = sx(arg(c, 1) << 2);
    c.g[A1][0] = sx(obj.wrapping_add(c.g[A1][0] as u32));
    c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xdc)).lw(c);
    let negative = (c.g[V1][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    if negative {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
    } else {
        c.cycles += 3;
        c.g[V1][0] = sx(obj.wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xd0)).lw(c);
        abi::ret(c);
    }
}


// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptEngine_StopLists(engine)` 0x2789e0: stop the main and auxiliary
/// script lists.
pub fn script_engine_stop_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let engine = abi::arg64(c, 0);
        c.g[S0][0] = engine;
        callee::SCRIPT_ENGINE_STOP_MAIN_LISTS.call(c, 0x0027_89f4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SCRIPT_ENGINE_STOP_AUX_LISTS.call(c, 0x0027_89fc)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptEngine_ListSetPaused(engine, i, paused)` 0x279040: pause/resume list
/// `i` (and nested lists).
pub fn script_engine_list_set_paused(c: &mut Cpu) {
    let second = abi::arg64(c, 2);
    script_list_call(
        c,
        0x0027_9060,
        Some(second),
        callee::SCRIPT_LIST_SET_PAUSED_NESTED,
    )
}


/// `ScriptEngine_ListGetEntryType(engine, i, a)` 0x279148: entry type of list
/// `i`'s entry `a`.
pub fn script_engine_list_get_entry_type(c: &mut Cpu) {
    let second = abi::arg64(c, 2);
    script_list_call(
        c,
        0x0027_9168,
        Some(second),
        callee::SCRIPT_LIST_GET_ENTRY_TYPE,
    )
}


/// `ScriptObjCam_v02(this)` 0x27d7f0: slot +0x10 of the script binding's
/// vtable (script camera stop).
pub fn script_obj_cam_v02(c: &mut Cpu) {
    script_binding_vcall(c, 0x10, 0x0027_d810)
}


/// `ScriptObjCam_v03(this)` 0x27d820: slot +0x18 of the script binding's
/// vtable (script camera start).
pub fn script_obj_cam_v03(c: &mut Cpu) {
    script_binding_vcall(c, 0x18, 0x0027_d840)
}


/// `void ScriptEngine_ListAdd(ScriptEngine *self, int index, void *cmd,
/// void *arg)` (0x278e20): append a command to list `index` (each list is
/// 0xcc bytes inside the array at `self+0x54c`).
pub fn script_engine_list_add(c: &mut Cpu) {
    /// The array of per-slot script lists.
    const LISTS: u32 = 0x54c;
    /// Size of one `ScriptList`.
    const LIST_SIZE: u32 = 0xcc;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = LIST_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LISTS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SCRIPT_LIST_ADD.call(c, 0x0027_8e44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_ListSkip(ScriptEngine *self, int index, void *arg)`
/// (0x279070): advance list `index` past `arg` entries (same list array as
/// [`script_engine_list_add`]).
pub fn script_engine_list_skip(c: &mut Cpu) {
    /// The array of per-slot script lists.
    const LISTS: u32 = 0x54c;
    /// Size of one `ScriptList`.
    const LIST_SIZE: u32 = 0xcc;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = LIST_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LISTS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SCRIPT_LIST_SKIP.call(c, 0x0027_9094)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_StopAll(ScriptEngine* self)` (0x278a98): stop the
/// main and auxiliary script lists and every process sequence.
pub fn script_engine_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_STOP_MAIN_LISTS_2.call(c, 0x0027_8aac)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SCRIPT_ENGINE_STOP_AUX_LISTS_2.call(c, 0x0027_8ab4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS.call(c, 0x0027_8abc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void* InSeq_ChannelGetBlockPayload(InSeqChannel* self, int block)`
/// (0x2742e0): payload of block `block` (`base + table[block] + 8`) when the
/// block is large enough, else 0.
pub fn inseq_channel_get_block_payload(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x14).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A2][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = Addr(c.g[A2][0] as u32).lhu(c);
    c.g[V0][0] = (c.g[V0][0] < 9u64) as u64;
    let small = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if small {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(8));
        abi::ret(c);
    }
}


/// `void ScriptEngine_ClearLocatorTable(ScriptEngine* self, int key)`
/// (0x278A58): clear the locator-table entry for a logical section (found in
/// the current world's section manager).
pub fn script_engine_clear_locator_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        let world = c.lw_r_2(V1, gp_3(c, -0x848));
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let engine = c.g[A0][0];
        c.g[S0][0] = engine;
        let manager = c.lw_r_2(V0, (world as u32).wrapping_add(0x84));
        let key = c.lw_r_2(A0, (manager as u32).wrapping_add(0x78));
        let _ = key;
        callee::SECTION_MAN_FIND_LOGICAL_SECTION.call(c, 0x0027_8a78)?;
        let index = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = index;
        frame.restore_ra(c, 0);
        let entry = sx((engine as u32).wrapping_add(index as u32));
        c.g[S0][0] = entry;
        Addr((entry as u32).wrapping_add(0x558)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_ListAddMovie(ScriptEngine* self, int list, tag a, tag b,
/// tag c, tag d, tag f)` (0x278E90): add a movie entry to list `list` (0xCC
/// bytes each at `*(self+0x54C) + list*0xCC`).
pub fn script_engine_list_add_movie(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        let index = abi::arg(c, 1);
        let lo = abi::mult(c, index, 0xcc);
        c.g[V1][0] = sx(lo);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        let lists = c.lw_r_2(A0, r(c, A0).wrapping_add(0x54c));
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        let entry = sx((lists as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = entry;
        c.g[T1][0] = c.g[T2][0];
        callee::SCRIPT_LIST_ADD_MOVIE.call(c, 0x0027_8ec0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_ListAddChoice(ScriptEngine* self, int index, a, b, c,
/// d, f, g)` (0x278e50): add a choice (0xcc bytes) to the script list at
/// `self+0x54c`.
pub fn script_engine_list_add_choice(c: &mut Cpu) {
    guest(c, |c| {
        let self_at = abi::arg(c, 0);
        c.cycles += 16;
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = Addr(self_at).off(0x54c).lw(c);
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[A0][0] = add32(c.g[A0][0], c.g[V1][0]);
        c.g[T1][0] = c.g[T2][0];
        c.g[T2][0] = c.g[T3][0];
        callee::SCRIPT_LIST_ADD_CHOICE.call(c, 0x0027_8e84)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `bool ScriptEngine_IsBankSlotLoaded(ScriptEngine* self, int slot)`
/// (0x27bf50): pump the async loaders, then report whether bank `slot`
/// (`self + 0x630 + slot*16`) is present.
pub fn script_engine_is_bank_slot_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS.call(c, 0x0027_bf6c)?;
        c.g[S1][0] = sx((r32(c, S1) as u32) << 4);
        frame.restore_ra(c, 0);
        c.g[S0][0] = add32(c.g[S0][0], c.g[S1][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x630).lw(c);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        leave(c, 0x30);
        Ok(())
    })
}


/// `void ScriptEngine_ExitScriptMode(ScriptEngine* self)` (0x27bf90): once,
/// set the mode to 2, end script music and disable the script surface mode.
pub fn script_engine_exit_script_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 2;
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).off(0x550).lw(c);
        let already = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 8;
            c.sw(p(self_at, 0x550), c.g[V1][0]);
            c.sw(p(self_at, 0x554), 0);
            callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x0027_bfb4)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC.call(c, 0x0027_bfbc)?;
            c.g[A0][0] = 0;
            callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE.call(c, 0x0027_bfc4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine accessors.

/// `bool ScriptEngine_IsLoadDone(ScriptEngine* self)` (0x278d58): true when
/// no async load is pending and bank slot 1 is not loaded.
pub fn script_engine_is_load_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2.call(c, 0x0027_8d6c)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x538));
        let pending = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if !pending {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED.call(c, 0x0027_8d84)?;
            c.g[V1][0] = (c.g[V0][0] < 1u64) as u64;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    });
}


/// `bool ScriptEngine_IsActiveListSkippable(ScriptEngine* self)` (0x278da0):
/// true unless the active script list says otherwise.
pub fn script_engine_is_active_list_skippable(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[V1][0] = c.lw(at(c, A1, 0x550));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0xcc;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, A1, 0x54c));
            c.mult(0, A0, V1, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE.call(c, 0x0027_8dcc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `int ScriptEngine_ListGetState(ScriptEngine* self, int index)` (0x2791d8):
/// the state of the active script list at `index`.
pub fn script_engine_list_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2.call(c, 0x0027_91f4)?;
        c.g[V0][0] = 0xcc;
        c.g[A0][0] = c.lw(at(c, S1, 0x54c));
        c.mult(0, S0, S0, V0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        callee::SCRIPT_LIST_GET_STATE.call(c, 0x0027_9208)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `int ScriptEngine_GetActiveListFieldB0(ScriptEngine* self)` (0x279298):
/// field 0xb0 of the current active script list (0 when idle).
pub fn script_engine_get_active_list_field_b0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[V1][0] = c.lw(at(c, A1, 0x550));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0xcc;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, A1, 0x54c));
            c.mult(0, A0, V1, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::SCRIPT_LIST_GET_FIELD_B0.call(c, 0x0027_92c4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `bool ScriptEngine_IsPlayingScript(ScriptEngine* self, int a)` (0x279370):
/// whether the current active script list is playing a script.
pub fn script_engine_is_playing_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 2;
        c.g[V1][0] = c.lw(at(c, A2, 0x550));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = 0xcc;
        if idle {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw(at(c, A2, 0x54c));
            c.mult(0, A0, V1, A0);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            callee::SCRIPT_LIST_IS_PLAYING_SCRIPT.call(c, 0x0027_939c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptEngine_CancelScriptBank(ScriptEngine* self, int id)`
/// (0x279840): find the bank slot for `id` and cancel its load.
pub fn script_engine_cancel_script_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 2;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID.call(c, 0x0027_9860)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD.call(c, 0x0027_9870)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptEngine_SetLocatorTable(self, key, value)` (0x278a10): store
/// `value` in the locator table slot the script manager's locator list gives
/// for `key` (`self + index*4 + 0x558`, index 50 when the key is absent).
pub fn script_engine_set_locator_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(gp_4(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        let world = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(world.wrapping_add(0x84)).lw(c);
        let manager = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(manager.wrapping_add(0x78)).lw(c);
        call_ptr(c, 0x0022_e0e0, 0x0027_8a38)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        let self_ = abi::r32(c, S0);
        let slot = self_.wrapping_add(c.g[V0][0] as u32);
        Addr(slot.wrapping_add(0x558)).sw(c, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_StopMainLists(ScriptEngine* self)` (0x2790f8): stop
/// both main script lists (0 and 1).
pub fn script_engine_stop_main_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::SCRIPT_ENGINE_LIST_STOP_2.call(c, 0x0027_9120)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_9118);
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


/// `int ScriptEngine_FindFreeBankSlot(ScriptEngine* self)` (0x27b528): first
/// free sound-bank slot in 2..9, else -1.
pub fn script_engine_find_free_bank_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x648));
    c.g[A1][0] = 2;
    c.g[A2][0] = 1;
    loop {
        c.cycles += 3;
        let slot = r32(c, A0);
        c.g[V0][0] = Addr(slot).off(8).lw(c);
        let busy = c.g[V0][0] != 0;
        c.g[V1][0] = 0;
        if !busy {
            c.cycles += 3;
            c.g[V0][0] = Addr(slot).lw(c);
            c.g[V1][0] = c.g[A2][0];
            if c.g[V0][0] != 0 {
                c.g[V1][0] = 0;
            }
        }
        c.cycles += 2;
        if c.g[V1][0] == 0 {
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
            if !more {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                abi::ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_b538);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A1][0];
            abi::ret(c);
            return;
        }
    }
}


/// `ScriptDummyObj* ScriptDummyObj_ctor(ScriptDummyObj* self)` (0x27bfd0):
/// construct the binding base and the 4-slot script object, then install the
/// two vtables.
pub fn script_dummy_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR_2.call(c, 0x0027_bfe4)?;
        c.g[A0][0] = sx(this.wrapping_add(0xc));
        c.g[A1][0] = 4;
        callee::SCRIPT_OBJ_CTOR.call(c, 0x0027_bff0)?;
        c.g[V1][0] = 0x480000;
        c.g[A0][0] = 0x480000;
        c.g[V1][0] = sx(0x0048_2318);
        c.g[A0][0] = sx(0x0048_22e0);
        Addr(this).off(0x18).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(this).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptViewObj_AcceptCommand(ScriptViewObj* self, void* arg, int
/// cmd)` (0x281a00): accept command 4 when the view object's script is the
/// engine's current one.
pub fn script_view_obj_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        let other = c.g[A2][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if other {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(r32(c, A1)).off(8).lw(c);
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT.call(c, 0x0028_1a24)?;
            c.g[V0][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            c.g[V1][0] = Addr(this).off(0xf8).lw(c);
            c.g[V0][0] ^= c.g[V1][0];
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_ObjTableDtor(ScriptEngine* e, int flags)` (0x27ce58).
pub fn script_engine_obj_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        script_obj_dtor(
            c,
            0x0048_2528,
            0x2a8,
            true,
            callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS,
            0x0027_ce80,
            5,
        )
    })
}


/// `void ScriptEngine_StopAuxLists(self)` (0x279488): stop both auxiliary
/// script lists of the rider.
pub fn script_engine_stop_aux_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 1;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x624).lw(c);
            c.g[A1][0] = 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[RA][0] = sx(0x0027_94bc);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            callee::SCRIPT_LIST_STOP.call(c, 0x0027_94bc)?;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xcc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_94a8);
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


/// `int ScriptViewObj_AddCam(self, cam)` (0x281190): put the camera in the
/// first free slot and make the view own it; 0 when both slots are taken.
pub fn script_view_obj_add_cam(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A2][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[V1][0] = sx(r32(c, A2).wrapping_add(0xd0));
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, V1));
            let occupied = c.g[V0][0] != 0;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            if !occupied {
                c.cycles += 5;
                Addr(r32(c, V1)).sw(c, c.g[A0][0]);
                c.g[RA][0] = sx(0x0028_11c0);
                c.g[A1][0] = c.g[A2][0];
                callee::SCRIPT_OBJ_CAM_SET_OWNER.call(c, 0x0028_11c0)?;
                c.g[V0][0] = 1;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_11a8);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* ScriptEngine_ScriptDbCtor(self)` (0x281e80): build the object table,
/// install the script-db vtables, register it globally and reset the process
/// table.
pub fn script_engine_script_db_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0028_1e94);
        callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR.call(c, 0x0028_1e94)?;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2450));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24b8));
        let this = r32(c, S0);
        Addr(p(this, 0x2ac)).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.g[S0][0];
        Addr(p(this, 0x2a8)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x3f8)).sw(c, c.g[S0][0]);
        Addr(p(this, 0x2b0)).sw(c, 0);
        c.g[RA][0] = sx(0x0028_1ec0);
        Addr(p(this, 0x2b4)).sw(c, 0);
        callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE.call(c, 0x0028_1ec0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Small register helpers.

/// The address `reg + off`.
#[inline(always)]
pub(crate) fn p_2(c: &Cpu, reg: usize, off: u32) -> u32 {
    abi::r32(c, reg).wrapping_add(off)
}


/// A `gp_5`-relative global address.
#[inline(always)]
pub(crate) fn gp_5(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_ListStop(ScriptEngine* e, int i)` (0x2790a0): stop
/// script list `i`; when it is the active list, leave script mode.
pub fn script_engine_list_stop(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = 0xcc;
    frame.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S1, 0x10);
    c.mult(0, V0, S0, V0);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[A1][0] = 1;
    c.g[A0][0] = Addr(p_2(c, S1, 0x54c)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if callee::SCRIPT_LIST_STOP_2.call(c, 0x0027_90d0).is_err() {
        return;
    }
    c.g[V0][0] = Addr(p_2(c, S1, 0x550)).lw(c);
    let not_active = c.g[V0][0] != c.g[S0][0];
    frame.restore(c, S0, 0x20);
    if !not_active {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        if callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE
            .call(c, 0x0027_90e4)
            .is_err()
        {
            return;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void ScriptEngine_OnSeqEnd(ScriptEngine* e, void* seq)` (0x27a798): tell
/// the active script list that the sequence ended.
pub fn script_engine_on_seq_end(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = 2;
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.g[V1][0] = Addr(p_2(c, A0, 0x550)).lw(c);
    let idle = c.g[V1][0] == c.g[V0][0];
    c.g[S1][0] = c.g[A1][0];
    if !idle {
        c.cycles += 9;
        c.g[V0][0] = 0xcc;
        c.g[S0][0] = Addr(p_2(c, A0, 0x54c)).lw(c);
        c.mult(0, V0, V1, V0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID
            .call(c, 0x0027_a7cc)
            .is_err()
        {
            return;
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        if callee::SCRIPT_LIST_ON_SEQ_END.call(c, 0x0027_a7dc).is_err() {
            return;
        }
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `bool RiderScriptObj_MatchesRiderSlot(void* self, void* rider, int id,
/// int a3)` (0x280560): true for slot 0, otherwise compare the target object
/// of rider `id - 1` with `rider`.
pub fn rider_script_obj_matches_rider_slot(c: &mut Cpu) {
    c.cycles += 5;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    let id = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    if id == 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        let negative = (id as i64) < 0;
        c.g[V0][0] = ((id as i64) < 22) as u64;
        if negative {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            let too_big = c.g[V0][0] == 0;
            c.g[A1][0] = sx((id as u32).wrapping_sub(1));
            if too_big {
                c.cycles += 1;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 6;
                c.g[A0][0] = Addr(gp_5(c, -0x84c)).lw(c);
                c.g[A2][0] = c.g[A3][0];
                if callee::SCRIPT_ENGINE_GET_TARGET_OBJECT
                    .call(c, 0x0028_0590)
                    .is_err()
                {
                    return;
                }
                c.g[V0][0] = c.g[V0][0] ^ c.g[S0][0];
                c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
            }
        }
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void ScriptEngine_ScriptDbDtor(ScriptEngine* e, int flags)` (0x281ed8):
/// the script database destructor: destroy the database, clear the global
/// engine pointer and destroy the object table.
pub fn script_engine_script_db_dtor(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = sx(0x0048_2450);
    frame.save(c, S0, 0x20);
    c.g[V1][0] = sx(0x0048_24b8);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    c.sw(p_2(c, S0, 0x2ac), c.g[V0][0]);
    c.sw(p_2(c, S0, 0x2a8), c.g[V1][0]);
    if callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2
        .call(c, 0x0028_1f0c)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.sw(gp_5(c, 0x3f8), 0);
    if callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR
        .call(c, 0x0028_1f1c)
        .is_err()
    {
        return;
    }
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `int ScriptDummyObj_AcceptCommand(void* self, int cmd, int which)`
/// (0x27c040): the dummy script object accepts command 6.
pub fn script_dummy_obj_accept_command(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0] ^ 6;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}


/// `int ScriptAudioObj_AcceptCommand(void* self, int cmd, int which)`
/// (0x2808d8): the audio script object accepts command 7.
pub fn script_audio_obj_accept_command(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.g[A2][0] ^ 7;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}


/// `void* ScriptViewObj_GetBinding(ScriptViewObj* this, int index)`
/// (0x281170): `this+0x10` for index 0, else the binding `this + 0x8 +
/// index*0x14`.
pub fn script_view_obj_get_binding(c: &mut Cpu) {
    c.cycles += 2;
    let this = abi::arg(c, 0);
    let index = c.g[A1][0];
    c.g[V0][0] = 0x14;
    if index == 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(this.wrapping_add(0x10));
    } else {
        c.cycles += 4;
        let index_off = abi::mult(c, index as u32, 0x14);
        c.g[V0][0] = sx(index_off);
        let binding_off = index_off.wrapping_add(8);
        c.g[V0][0] = sx(binding_off);
        c.g[V0][0] = sx(this.wrapping_add(binding_off));
    }
    abi::ret(c);
}


/// `bool ScriptObjCam_IsBoundToProcess(ScriptObjCam* self, int process)`
/// (0x27D790): does the binding at `+0x14` hold `process`? No binding is
/// false.
pub fn script_obj_cam_is_bound_to_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x14));
        let unbound = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if unbound {
            c.cycles += 2;
            c.g[V0][0] = 0;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 9;
            callee::SCRIPT_ENGINE_GET_INSTANCE_3.call(c, 0x0027_d7b8)?;
            c.g[V1][0] = c.lw(ea(c.g[S0][0], 0x14));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lw(ea(c.g[V1][0], 4));
            callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS.call(c, 0x0027_d7cc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptEngine_FindBankSlotById(BankSlot* slots, int key, int start)`
/// (0x27b578): scan the 10 bank slots from `start`, return the first free slot
/// whose key matches, or -1.
pub fn script_engine_find_bank_slot_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
    let out_of_range = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 4);
    if !out_of_range {
        c.cycles += 3;
        c.g[A3][0] = 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62c));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw_2(c, p_3(c.g[A0][0], 4));
            let taken = c.g[V0][0] != 0;
            c.g[V1][0] = 0;
            if !taken {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, (c.g[A0][0] as u32).wrapping_sub(4));
                c.g[V1][0] = c.g[A3][0];
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = 0;
                }
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, c.g[A0][0] as u32);
                if c.g[V0][0] != c.g[A1][0] {
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A2][0];
                    abi::ret(c);
                    return;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_b590);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    abi::ret(c);
}


/// `void ScriptEngine_ListAddMovieById(ScriptEngine* self, int key, ...)`
/// (0x278ed0): scan the movie list for a matching key and add it.
pub fn script_engine_list_add_movie_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[T4][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x52c)).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[A2][0] = 0;
        let mut matched = false;
        if !empty {
            c.cycles += 1;
            c.g[T3][0] = 0;
            loop {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x530)).lw(c);
                c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 4)).lw(c);
                if c.g[V1][0] == c.g[T4][0] {
                    c.cycles += 4;
                    call_ptr(c, 0x0027_8e90, 0x0027_8f0c)?;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    matched = true;
                    break;
                }
                c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 0x52c)).lw(c);
                c.cycles += 5;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x0c));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8ef0);
                }
            }
        }
        if !matched {
            c.cycles += 1;
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        }
        c.cycles += 2;
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}


/// `RiderScriptObj_Deactivate(obj)` 0x2803f0: deactivate the embedded script
/// object, copy the identity transform into `obj + 0x30` and clear the active
/// flags.
pub fn rider_script_obj_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(this, 0xc));
        call_ptr(c, 0x0028_3440, 0x0028_0408)?;
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(p(this, 0x30));
        c.g[V0][0] = sx(0x004f_f1a0);
        let src = c.g[V0][0] as u32;
        let dst = c.g[V1][0] as u32;
        for q in 0..4u32 {
            c.lqc2(q as usize + 1, p(src, q * 0x10));
        }
        for q in 0..4u32 {
            c.sqc2(q as usize + 1, p(dst, q * 0x10));
        }
        Addr(p(this, 0x20)).sw(c, 0);
        for off in [0x70, 0x74, 0x78, 0x1c] {
            Addr(p(this, off)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptEngine::CancelBankLoad(self, index)` (0x27b4b8): cancel and release
/// the async load in slot `index` and clear the slot.
pub fn script_engine_cancel_bank_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x628));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        let empty = c.g[A0][0] == 0;
        if !empty {
            c.cycles += 12;
            call_ptr(c, 0x003d_faf0, 0x0027_b4ec)?;
            c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x003d_f9d8, 0x0027_b4fc)?;
            c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0x620)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(p(c.g[S1][0] as u32, 0x620)).sw(c, c.g[V1][0]);
            call_ptr(c, 0x0027_7f08, 0x0027_b510)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_OnSeqStart(ScriptEngine* e, ScriptList* list)`
/// (0x27a6b0): tell the listener a sequence started, and if the engine is not
/// in "stopped" mode (state 2) also notify the script list entry `r` of the
/// sequence table (`+0x54c`, `0xcc` bytes each).
pub fn script_engine_on_seq_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SEQ_LISTENER_ON_START.call(c, 0x0027_a6d0)?;
        c.g[V1][0] = Addr(p_4(r32(c, S1), 0x550)).lw(c);
        c.g[V0][0] = 2;
        let stopped = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0xcc;
        if !stopped {
            c.cycles += 10;
            c.g[S0][0] = Addr(p_4(r32(c, S1), 0x54c)).lw(c);
            c.mult(0, V0, V1, V0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[S0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
            callee::GET_SEQ_PROCESS_ID.call(c, 0x0027_a6f8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            callee::SCRIPT_LIST_ON_SEQ_START.call(c, 0x0027_a708)?;
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}


/// `void ScriptEngine_OnSeqStopped(ScriptEngine* e, ScriptList* list)`
/// (0x27a7f0): like [`script_engine_on_seq_start`] but tells the listener the
/// sequence stopped, before notifying the script list.
pub fn script_engine_on_seq_stopped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 2;
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        sqsp(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p_4(r32(c, S1), 0x550)).lw(c);
        let stopped = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if !stopped {
            c.cycles += 9;
            c.g[V0][0] = 0xcc;
            c.g[S0][0] = Addr(p_4(r32(c, S1), 0x54c)).lw(c);
            c.mult(0, V0, V1, V0);
            c.g[S0][0] = abi::addu(c.g[S0][0], c.g[V0][0]);
            callee::GET_SEQ_PROCESS_ID.call(c, 0x0027_a82c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S2][0];
            callee::SCRIPT_LIST_ON_SEQ_STOPPED.call(c, 0x0027_a83c)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S2][0];
        callee::SEQ_LISTENER_ON_STOP.call(c, 0x0027_a848)?;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `int ScriptEngine_RemoveBinding(ScriptEngine* e, ScriptMessage* msg)`
/// (0x27d390): clear the binding for the variable index in `msg+4`, returning
/// 1.
pub fn script_engine_remove_binding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = c.g[A1][0];
        sqsp(c, 16, 0x10);
        c.g[V1][0] = 0x14;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(p_4(r32(c, A2), 4)).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = abi::addu(c.g[A0][0], c.g[V0][0]);
        c.g[V1][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
        if c.g[V1][0] != c.g[A1][0] {
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[A2][0];
            callee::SCRIPT_BINDING_CLEAR_LISTENER.call(c, 0x0027_d3cc)?;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        }
        c.cycles += 12;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        c.g[V0][0] = 1;
        Addr(p_4(c.g[A0][0] as u32, 4)).sw(c, c.g[V1][0]);
        Addr(p_4(r32(c, S0), 0x10)).sw(c, c.g[V1][0]);
        Addr(p_4(r32(c, S0), 4)).sw(c, c.g[V1][0]);
        Addr(c.g[S0][0] as u32).sw(c, 0);
        Addr(p_4(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        Addr(p_4(r32(c, S0), 0xc)).sw(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}


/// `int ScriptEngine_CollectFreeBindings(ScriptEngine* e, char* end,
/// void** out, int max)` (0x27d578): walk the 30 variable slots at `e+0x50`,
/// collecting live, non-type-1 variables that are unbound (`link < 0`) or
/// bound to `end`; returns how many were collected.
pub fn script_engine_collect_free_bindings(c: &mut Cpu) {
    c.cycles += 4;
    let mut a0 = (c.g[A0][0] as u32).wrapping_add(0x50);
    c.g[A0][0] = a0 as u64;
    let a1 = c.g[A1][0];
    let mut a2 = c.g[A2][0] as u32;
    let a3 = c.g[A3][0];
    let mut t1: u64 = 0;
    let mut t2: u64 = 0;
    if a3 != 0 {
        c.cycles += 2;
        let t3: u64 = 1;
        let mut t0 = a0;
        loop {
            c.cycles += 3;
            let v1 = Addr(a0).lw(c) as u64;
            c.g[V1][0] = v1;
            let mut tail = false;
            if v1 == 0 {
                t2 = sx((t2 as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                let ty = Addr(p_4(a0, 0xc)).lw(c) as u64;
                if ty == t3 {
                    t2 = sx((t2 as u32).wrapping_add(1));
                } else {
                    c.cycles += 3;
                    // `lw` sign-extends: the bound test and the `end`
                    // comparison are 64-bit, exactly as the original.
                    let link = Addr(p_4(a0, 0x10)).lw(c);
                    c.g[A0][0] = link;
                    if (link as i64) < 0 {
                        Addr(a2).sw(c, v1);
                        tail = true;
                    } else {
                        c.cycles += 2;
                        if link != a1 {
                            t2 = sx((t2 as u32).wrapping_add(1));
                        } else {
                            c.cycles += 1;
                            Addr(a2).sw(c, v1);
                            tail = true;
                        }
                    }
                }
            }
            if tail {
                c.cycles += 3;
                t1 = sx((t1 as u32).wrapping_add(1));
                a2 = a2.wrapping_add(4);
                c.g[A2][0] = a2 as u64;
                t2 = sx((t2 as u32).wrapping_add(1));
            }
            c.cycles += 3;
            let more_slots = (t2 as i64) < 30;
            t0 = t0.wrapping_add(0x14);
            if !more_slots {
                break;
            }
            c.cycles += 2;
            let cont = t1 != a3;
            a0 = t0;
            c.g[A0][0] = a0 as u64;
            if !cont {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_d590);
            }
        }
    }
    c.cycles += 2;
    c.g[T1][0] = t1;
    c.g[T2][0] = t2;
    c.g[V0][0] = t1;
    abi::ret(c);
}


/// `lw` at `reg + off`.
#[inline(always)]
pub(crate) fn lw_at_2(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(r32(c, reg).wrapping_add(off)).lw(c)
}


/// `sw` at `reg + off`.
#[inline(always)]
pub(crate) fn sw_at(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = r32(c, reg).wrapping_add(off);
    c.sw(a, v);
}


/// `sx(x as u32)` of a register's low word.
#[inline(always)]
pub(crate) fn low(c: &Cpu, reg: usize) -> u64 {
    sx(r32(c, reg))
}


// ---------------------------------------------------------------------------
// Script engine.

/// `ScriptEngine_LoadScriptBank` (0x2797C8): format the bank name and load it
/// asynchronously into the first free script bank slot. Returns 1.
pub fn script_engine_load_script_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0xb0);
        c.g[A2][0] = 0x48_0000;
        frame.save(c, S1, 0x90);
        c.g[A2][0] = low(c, A2).wrapping_add(0x1c78);
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0x80);
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A1][0] = low(c, A1).wrapping_add(0x1c68);
        call_ptr(c, addr::SPRINTF, 0x0027_97fc)?;
        c.g[A0][0] = c.g[S0][0];
        callee::FIND_FREE_BANK_SLOT.call(c, 0x0027_9804)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0x100;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        callee::LOAD_BANK_ASYNC.call(c, 0x0027_9824)?;
        frame.restore(c, S0, 0xa0);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptEngine_IsFinishScriptActive` (0x27AA80): true while the finish-line
/// script (entry type 0x12/0x0C/0x13/0x0D/0x14/0x0E) is the engine's current
/// script.
pub fn script_engine_is_finish_script_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 1;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at_2(c, A0, 0x550);
        let not_finish = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        if !not_finish {
            c.cycles += 7;
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            callee::LIST_GET_ENTRY_TYPE.call(c, 0x0027_aaa4)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = 0x12;
            c.g[A0][0] = 0;
            let mut hit = c.g[V1][0] == c.g[V0][0];
            if !hit {
                c.cycles += 3;
                c.g[V0][0] = 0xc;
                hit = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x13;
                if !hit {
                    c.cycles += 2;
                    hit = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xd;
                    if !hit {
                        c.cycles += 2;
                        hit = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = 0x14;
                        if !hit {
                            c.cycles += 2;
                            hit = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0xe;
                            if !hit {
                                c.cycles += 2;
                                let other = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = c.g[A0][0];
                                hit = !other;
                            }
                        }
                    }
                }
            }
            if hit {
                c.cycles += 2;
                c.g[A0][0] = 1;
                c.g[V0][0] = c.g[A0][0];
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptView_ReadFields` (0x27C8E0): read a script view's start/end times
/// from its channel's track object into `out[0]`/`out[1]`.
pub fn script_view_read_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_c900)?;
        c.g[V1][0] = lw_at_2(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x24)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_c914)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        sw_at(c, S1, 0, c.g[V1][0]);
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_c928)?;
        c.g[V1][0] = lw_at_2(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_c93c)?;
        sw_at(c, S1, 0x4, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptDummyObj_ReadFields` (0x27CBB0): the same start/end time read for a
/// dummy object (identical body).
pub fn script_dummy_obj_read_fields(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_cbd0)?;
        c.g[V1][0] = lw_at_2(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x20)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x24)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_cbe4)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        sw_at(c, S1, 0, c.g[V1][0]);
        callee::CHANNEL_GET_TRACK.call(c, 0x0027_cbf8)?;
        c.g[V1][0] = lw_at_2(c, V0, 0x4);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let get = r32(c, A1);
        c.g[A0][0] = abi::addu(c.g[V0][0], c.g[A0][0]);
        call_ptr(c, get, 0x0027_cc0c)?;
        sw_at(c, S1, 0x4, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptViewObj_Reset` (0x281A50): reset a script view object: clear both id
/// slots, set the "active" flag and an effectively infinite best distance,
/// store the zero vector + w=1 at +0x100, and clear the counters.
pub fn script_view_obj_reset(c: &mut Cpu) {
    c.cycles += 4;
    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
    c.g[A1][0] = 0xffff_ffff_ffff_ffff;
    c.g[V1][0] = 1;
    c.g[V0][0] = low(c, A0).wrapping_add(0xe0);
    loop {
        c.cycles += 7;
        c.sw(r32(c, V0), c.g[A1][0]);
        c.g[V1][0] = low(c, V1).wrapping_sub(1);
        let again = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = low(c, V0).wrapping_sub(4);
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x0028_1a60);
            }
            continue;
        }
        c.cycles += 18;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V1][0] = 1;
        c.sw(r32(c, SP), 0);
        c.swc1(0, sp(c).wrapping_add(0xc));
        c.sw(sp(c).wrapping_add(0x4), 0);
        c.sw(sp(c).wrapping_add(0x8), 0);
        c.lwc1(0, gp(c, -0x4788));
        c.lq(2, sp(c));
        sw_at(c, A0, 0xf0, c.g[V1][0]);
        c.swc1(0, r32(c, A0).wrapping_add(0xf4));
        c.sq(2, r32(c, A0).wrapping_add(0x100));
        for off in [0x110u32, 0xe4, 0xe8, 0xec] {
            sw_at(c, A0, off, 0);
        }
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ret;
        return;
    }
}


// ---------------------------------------------------------------------------
// Script / music / replay / AI.

/// `void ScriptEngine_OnSeqAdvance(self, a1, float x, float y)` (0x27a720):
/// unless the script slot at `+0x550` is already running (id 2), advance the
/// sequence list at `+0x54c + id*0xcc` to the process the engine reports for
/// `a1`, passing both floats through in the delay slots.
pub fn script_engine_on_seq_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x40));
        c.g[V0][0] = 0x2;
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.swc1(21, sp_2(c).wrapping_add(0x38));
        c.g[S1][0] = c.g[A1][0];
        c.swc1(20, sp_2(c).wrapping_add(0x30));
        c.mov_s(21, 13);
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[V1][0] = lw_r(c, A0, 0x550);
        let running = c.g[V1][0] == c.g[V0][0];
        c.mov_s(20, 12);
        if !running {
            c.cycles += 11;
            c.g[V0][0] = 0xcc;
            c.g[S0][0] = lw_r(c, A0, 0x54c);
            c.mult(0, V0, V1, V0); // lo = id * 0xcc
            c.g[S0][0] = sx(low_2(c, S0).wrapping_add(low_2(c, V0)));
            callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2.call(c, 0x0027_a764)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.mov_s(12, 20);
            c.mov_s(13, 21);
            callee::SCRIPT_LIST_ON_SEQ_ADVANCE.call(c, 0x0027_a77c)?;
        }
        c.cycles += 7;
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        c.lwc1(21, sp_2(c).wrapping_add(0x38));
        c.lwc1(20, sp_2(c).wrapping_add(0x30));
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}


/// `void ScriptEngine_UnloadWorldScripts(self)` (0x00278cd8): release the
/// world script file and unload the script bank.
pub fn script_engine_unload_world_scripts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(at(c, S0, 0x538)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 10;
            callee::ASYNC_FILE_RELEASE_1.call(c, 0x0027_8cfc)?;
            c.g[A0][0] = Addr(at(c, S0, 0x538)).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::ASYNC_FILE_RELEASE_2.call(c, 0x0027_8d0c)?;
            c.g[V1][0] = Addr(at(c, S0, 0x620)).lw(c);
            c.g[V1][0] = sx(word(c, V1).wrapping_sub(1));
            Addr(at(c, S0, 0x620)).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.g[A1][0] = 0x1;
        callee::SCRIPT_ENGINE_UNLOAD_BANK.call(c, 0x0027_8d24)?;
        c.g[A0][0] = Addr(at(c, S0, 0x53c)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            callee::MEM_FREE.call(c, 0x0027_8d38)?;
        }
        Addr(at(c, S0, 0x540)).sw(c, 0);
        c.cycles += 6;
        Addr(at(c, S0, 0x538)).sw(c, 0);
        Addr(at(c, S0, 0x53c)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_PlayScriptInstance(self, key, val, b, c)` (0x00279720):
/// look the process up, set its instance script's field and run it.
pub fn script_engine_play_script_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A3][0];
        c.g[S4][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0027_9758);
        c.g[A1][0] = c.g[S1][0];
        callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ.call(c, 0x0027_9758)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_3.call(c, 0x0027_9764)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x0027_977c);
        Addr(at(c, V0, 0xc)).sw(c, c.g[S2][0]);
        callee::SCRIPT_ENGINE_RUN_PROCESS.call(c, 0x0027_977c)?;
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


/// `void ScriptAudioObj_Activate(self)` (0x00280730): activate the script
/// object, resolve its data/bank and clear the four sound slots.
pub fn script_audio_obj_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(word(c, S0).wrapping_add(0xc));
        callee::SCRIPT_OBJ_ACTIVATE.call(c, 0x0028_0748)?;
        Addr(at(c, S0, 0xec)).sw(c, 0);
        Addr(at(c, S0, 0xf0)).sw(c, 0);
        Addr(at(c, S0, 0xf4)).sw(c, 0);
        c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x8)).lw(c);
        callee::SCRIPT_DATA_GET_INFO_3.call(c, 0x0028_0760)?;
        c.g[A1][0] = Addr(at(c, V0, 0x1c)).lw(c);
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        callee::SCRIPT_ENGINE_FIND_SOUND_BANK.call(c, 0x0028_076c)?;
        Addr(at(c, S0, 0xf8)).sw(c, c.g[V0][0]);
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[S0][0] = sx(word(c, S0).wrapping_add(0xbc));
        c.g[V0][0] = 0x3;
        loop {
            c.cycles += 7;
            Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx(word(c, V0).wrapping_sub(1));
            Addr(at(c, S0, 0)).sw(c, c.g[V1][0]);
            Addr(at(c, S0, 0x4)).sw(c, 0);
            let more = (c.g[V0][0] as i64) >= 0;
            c.g[S0][0] = sx(word(c, S0).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0028_0780);
                }
                continue;
            }
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `void ScriptEngine_v04(engine, obj, process)` (0x279e50): dispatch a
/// script object camera request by the object's opcode.
pub fn scriptengine_v04(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ.call(c, 0x0027_9e74)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = lw_at_3(c, c.g[A0][0] as u32);
        c.g[V0][0] = 1;
        let not_one = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 2;
        if !not_one {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS.call(c, 0x0027_9e90)?;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let is_two = c.g[V1][0] == c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        if !is_two {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS.call(c, 0x0027_9eac)?;
            frame.restore(c, 16, 0x30);
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A1][0] = c.g[S0][0];
        callee::SCRIPT_OBJ_CAM_HAS_BINDING.call(c, 0x0027_9ebc)?;
        frame.restore(c, 16, 0x30);
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script tables and objects.

/// `ScriptObjTable* ScriptEngine_ObjTableCtor(ScriptObjTable* self)`
/// (0x27cdd0): publish the vtable and clear the 20 free slots and the 30
/// 5-word records.
pub fn script_engine_obj_table_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x48_0000;
    c.g[V1][0] = 0x13;
    c.g[V0][0] = sx(0x0048_2528);
    Addr(p(r32(c, A0), 0x2a8)).sw(c, c.g[V0][0]);
    Addr(p(r32(c, GP), 0x3ec)).sw(c, c.g[A0][0]);
    c.g[V0][0] = sx(p(r32(c, A0), 0x4c));
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_cde8);
        }
    }
    c.cycles += 7;
    c.g[T1][0] = u64::MAX;
    c.g[V1][0] = sx(p(r32(c, A0), 0x60));
    c.g[T0][0] = sx(p(r32(c, A0), 0x5c));
    c.g[A3][0] = sx(p(r32(c, A0), 0x58));
    c.g[A2][0] = sx(p(r32(c, A0), 0x54));
    c.g[A1][0] = sx(p(r32(c, A0), 0x50));
    c.g[T2][0] = sx(p(r32(c, A0), 0x2b8));
    loop {
        c.cycles += 12;
        Addr(r32(c, A1)).sw(c, 0);
        Addr(r32(c, A2)).sw(c, c.g[T1][0]);
        c.g[A1][0] = sx(p(r32(c, A1), 0x14));
        Addr(r32(c, A3)).sw(c, c.g[T1][0]);
        c.g[A2][0] = sx(p(r32(c, A2), 0x14));
        Addr(r32(c, T0)).sw(c, 0);
        c.g[A3][0] = sx(p(r32(c, A3), 0x14));
        Addr(r32(c, V1)).sw(c, c.g[T1][0]);
        c.g[V1][0] = sx(p(r32(c, V1), 0x14));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[T0][0] = sx(p(r32(c, T0), 0x14));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0027_ce20);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}


/// `void ScriptObjCam_Activate(ScriptObjCam* self)` (0x27d850): activate the
/// script object and, when the world is running, build the matrix and start
/// the camera controller.
pub fn script_obj_cam_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0027_d864);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_OBJ_ACTIVATE_2.call(c, 0x0027_d864)?;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        let no_world = c.g[V1][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_world {
            c.cycles += 19;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A1][0] = Addr(p(r32(c, S0), 0x14)).lw(c);
            c.g[RA][0] = sx(0x0027_d888);
            c.swc1(0, p(r32(c, S0), 0x18));
            callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT.call(c, 0x0027_d888)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(p(r32(c, S0), 0x7c)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = Addr(p(r32(c, A0), 0x84)).lw(c);
            c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
            c.g[S0][0] = Addr(p(r32(c, V0), 4)).lw(c);
            c.g[RA][0] = sx(0x0027_d8b0);
            c.g[A0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
            call_ptr(c, 0x0016_9190, 0x0027_d8b0)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x18)).lw(c);
            let no_controller = c.g[V1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_controller {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0027_d8c8);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x0015_dfd8, 0x0027_d8c8)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptAudioObj* ScriptAudioObj_ctor(ScriptAudioObj* self)` (0x2805b8):
/// the binding/object constructors, the audio vtable and the four output
/// records.
pub fn script_audio_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_05cc);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR_3.call(c, 0x0028_05cc)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[RA][0] = sx(0x0028_05d8);
        c.g[A1][0] = 0x5;
        callee::SCRIPT_OBJ_CTOR_2.call(c, 0x0028_05d8)?;
        c.g[V0][0] = 0x48_0000;
        c.g[V1][0] = 0x48_0000;
        c.g[V0][0] = sx(0x0048_2298);
        c.g[V1][0] = sx(0x0048_2260);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = u64::MAX;
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(p(r32(c, S0), 0xbc));
        Addr(p(r32(c, S0), 0xec)).sw(c, 0);
        c.g[V1][0] = 0x3;
        Addr(p(r32(c, S0), 0xf0)).sw(c, 0);
        Addr(p(r32(c, S0), 0xf4)).sw(c, 0);
        Addr(p(r32(c, S0), 0xf8)).sw(c, 0);
        loop {
            c.cycles += 7;
            Addr(p(r32(c, V0), 8)).sw(c, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            Addr(r32(c, V0)).sw(c, c.g[A0][0]);
            Addr(p(r32(c, V0), 4)).sw(c, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(p(r32(c, V0), 0xc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_0610);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptEngine_ListStart(ScriptEngine* self, int offset, void* a,
/// void* b)` (0x278f68): start the script list at `self+0x54c+offset`, publish
/// the new list state and return the list's start result.
pub fn script_engine_list_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        Addr(p(r32(c, S3), 0x550)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0x1;
        call_ptr(c, 0x002e_f368, 0x0027_8f9c)?;
        c.g[V0][0] = 0xcc;
        c.g[A0][0] = Addr(p(r32(c, S3), 0x54c)).lw(c);
        c.mult(0, S0, S0, V0);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
        callee::SCRIPT_LIST_START.call(c, 0x0027_8fb8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = 0x3;
        if c.g[S0][0] == c.g[V0][0] {
            // Finished immediately.
        } else {
            c.cycles += 3;
            c.g[A0][0] = 0;
            c.g[V0][0] = 0x2;
            Addr(p(r32(c, S3), 0x550)).sw(c, c.g[V0][0]);
            call_ptr(c, 0x002e_f368, 0x0027_8fd4)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}



// ---------------------------------------------------------------------------
// HUD / script list state.

/// `int ScriptEngine_IsHUDAllowed(ScriptEngine* self)` (0x2792e0): the HUD may
/// draw unless the engine is in state 2 or in the middle of a skippable
/// script; the script's play status must also not be "done" (3).
pub fn script_engine_is_hud_allowed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 2;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, S0), 0x550)).lw(c);
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = 1;
        } else {
            c.g[V0][0] = Addr(p(r32(c, S0), 0x554)).lw(c);
            c.cycles += 2;
            c.g[S2][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                c.g[S1][0] = 0xcc;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x54c)).lw(c);
                c.mult(0, A0, A0, S1);
                c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                callee::SCRIPT_LIST_CAN_SKIP.call(c, 0x0027_932c)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.g[S2][0];
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = Addr(p(r32(c, S0), 0x550)).lw(c);
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x54c)).lw(c);
                    c.mult(0, A0, A0, S1);
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    callee::SCRIPT_LIST_GET_PLAY_STATUS_2.call(c, 0x0027_9348)?;
                    c.g[V0][0] = c.g[V0][0] ^ 0x3;
                    c.g[S2][0] = (0u64 < c.g[V0][0]) as u64;
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
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


// ---------------------------------------------------------------------------
// Script objects.

/// `RiderScriptObj* RiderScriptObj_ctor(RiderScriptObj* self)` (0x27f968):
/// build a rider script binding (a script binding plus a script object of
/// kind 3), publish its vtable, copy the identity matrix into the object's
/// transform and zero its counters.
pub fn rider_script_obj_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_BINDING_CTOR_4.call(c, 0x0027_f97c)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[A1][0] = 3;
        callee::SCRIPT_OBJ_CTOR_3.call(c, 0x0027_f988)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = 0x0048_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x1fd8));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x2018));
        c.g[A0][0] = 0x0050_0000;
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0xe60));
        Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
        c.g[V0][0] = sx(p(r32(c, S0), 0x30));
        Addr(p(r32(c, S0), 0x20)).sw(c, 0);
        c.lqc2(1, r32(c, A0));
        c.lqc2(2, p(r32(c, A0), 0x10));
        c.lqc2(3, p(r32(c, A0), 0x20));
        c.lqc2(4, p(r32(c, A0), 0x30));
        c.sqc2(1, r32(c, V0));
        c.sqc2(2, p(r32(c, V0), 0x10));
        c.sqc2(3, p(r32(c, V0), 0x20));
        c.sqc2(4, p(r32(c, V0), 0x30));
        c.g[V0][0] = c.g[S0][0];
        Addr(p(r32(c, S0), 0x70)).sw(c, 0);
        Addr(p(r32(c, S0), 0x74)).sw(c, 0);
        Addr(p(r32(c, S0), 0x78)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptViewObj_SetTransition(ScriptViewObj* self, void* obj)`
/// (0x2812e0): set (or, with a null `obj`, stop) the view's transition and
/// report whether it changed.
pub fn script_view_obj_set_transition(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(p(r32(c, S0), 0xd8)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, A0), 8)).lw(c);
            c.g[A1][0] = u64::MAX;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[A2][0] = 9;
                c.mov_s(13, 12);
                c.mov_s(14, 12);
                c.mov_s(15, 12);
                callee::TRANSITION_MAN_STOP_TRANSITION.call(c, 0x0028_1320)?;
                Addr(p(r32(c, S0), 0xe8)).sw(c, 0);
                c.g[A0][0] = Addr(p(r32(c, S0), 0xd8)).lw(c);
            }
            c.cycles += 5;
            c.g[A1][0] = 0;
            callee::TRANSITION_MAN_SCRIPT_SET_OWNER.call(c, 0x0028_1330)?;
            Addr(p(r32(c, S0), 0xd8)).sw(c, 0);
            c.g[V0][0] = 1;
        } else {
            c.cycles += 3;
            let running = Addr(p(r32(c, S0), 0xd8)).lw(c) != 0;
            c.g[V0][0] = 0;
            if running {
                // A transition is already running: refuse.
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V1][0];
                c.g[A1][0] = c.g[S0][0];
                Addr(p(r32(c, S0), 0xd8)).sw(c, c.g[V1][0]);
                callee::TRANSITION_MAN_SCRIPT_SET_OWNER.call(c, 0x0028_1358)?;
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_OnDataRemoved(self, obj, a2, type, id)` (0x2799e0):
/// release the script object's binding for `type` 0, the anim bank for 1 and
/// the sound bank for 2.
pub fn script_engine_on_data_removed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        let obj = abi::arg(c, 1);
        let kind = abi::arg64(c, 3);
        let id = abi::arg64(c, 4);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(obj);
        let is_one = kind == c.g[V0][0];
        c.g[A1][0] = id;
        if is_one {
            c.cycles += 4;
            c.g[A0][0] = sx(p(r32(c, S0), 0xa28));
            c.g[A1][0] = id;
            call_ptr(c, callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK, 0x0027_9a50)?;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((kind as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                if kind == c.g[V0][0] {
                    c.cycles += 3;
                    c.g[A0][0] = sx(p(r32(c, S0), 0xa38));
                    c.g[A1][0] = id;
                    call_ptr(c, callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK, 0x0027_9a60)?;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                if kind == 0 {
                    c.cycles += 7;
                    call_ptr(c, callee::SCRIPT_DATA_GET_BANK_LIST, 0x0027_9a34)?;
                    c.g[A1][0] = c.lw(at(c, V0, 0x18));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT, 0x0027_9a40)?;
                } else {
                    c.cycles += 2;
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptViewObj_Activate(ScriptViewObj* self, int seq)` (0x281370):
/// bind the object to the sequence, reset it, and if the sequence is playing
/// tell the view object that it was activated.
pub fn script_view_obj_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        let seq = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(self_);
        frame.save_ra(c, 0);
        c.g[S1][0] = seq;
        call_ptr(c, callee::SCRIPT_ENGINE_GET_INSTANCE_4, 0x0028_138c)?;
        c.g[A1][0] = c.lw(at(c, S0, 4));
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER, 0x0028_139c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::SCRIPT_VIEW_OBJ_RESET, 0x0028_13a4)?;
        c.g[A0][0] = c.lw(gp(c, -0x84c));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE, 0x0028_13b0)?;
        c.g[A0][0] = c.lw(gp(c, -0x84c));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT, 0x0028_13bc)?;
        let stopped = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gp(c, -0x848));
        if !stopped {
            c.cycles += 7;
            c.g[A1][0] = sx(p(r32(c, S0), 0x100));
            c.g[V0][0] = c.lw(at(c, A0, 0x84));
            c.g[V1][0] = c.lw(at(c, V0, 0xc));
            c.g[A0][0] = c.lw(at(c, V1, 0xa4));
            call_ptr(c, callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED, 0x0028_13d8)?;
            c.g[V0][0] = 1;
            c.sw(at(c, S0, 0x110), c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, callee::SCRIPT_OBJ_ACTIVATE_3, 0x0028_13ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptObjCam* ScriptObjCam_ctor(ScriptObjCam* self, u32 owner)` (0x27d5e8):
/// build the scripted camera object, copy the default 4x4 matrix and clear the
/// follow target.
pub fn script_obj_cam_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        callee::SCRIPT_OBJ_CTOR.call(c, 0x0027_d608)?;
        c.g[V1][0] = 0x480000;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20b8));
        c.g[V0][0] = 0x500000;
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
        Addr(c.g[S0][0] as u32).off(0x20).sw(c, c.g[A0][0]);
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x14).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x18).sw(c, 0);
        c.lqc2(1, c.g[V0][0] as u32);
        c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
        c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
        c.sqc2(1, c.g[V1][0] as u32);
        c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
        c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
        Addr(c.g[S0][0] as u32).off(0x7c).sw(c, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).off(0x70).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x74).sw(c, 0);
        Addr(c.g[S0][0] as u32).off(0x78).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptObjCam_Deactivate(self)` (0x27d8d8): detach the camera from its
/// rider and release the rider's scripted object when nothing else holds it.
pub fn script_obj_cam_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SCRIPT_OBJ_DEACTIVATE_2.call(c, 0x0027_d8ec)?;
        let world = Addr(gp_6(c, -0x848)).lw(c);
        c.g[A0][0] = world;
        c.g[V0][0] = Addr(world as u32).off(0x84).lw(c);
        c.g[A1][0] = c.g[S0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 13;
            Addr(c.g[S0][0] as u32).off(0x18).sw(c, 0);
            c.g[V1][0] = lw_3(c, S0, 0x7c);
            c.g[A0][0] = Addr(world as u32).off(0x84).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = lw_3(c, A0, 0x84);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = lw_3(c, V0, 0x4);
            c.g[A0][0] = lw_3(c, S0, 0xac);
            call_ptr(c, table_callee::RIDER_REMOVE_SCRIPT_OBJECT, 0x0027_d920)?;
            c.g[V0][0] = lw_3(c, S0, 0xac);
            c.g[V1][0] = lw_3(c, V0, 0x18);
            if c.g[V1][0] == 0 {
                c.cycles += 11;
                c.g[A1][0] = 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table_callee::RIDER_SET_SCRIPT_STATE, 0x0027_d93c)?;
                c.g[A0][0] = lw_3(c, S0, 0xa8);
                call_ptr(c, table_callee::RIDER_SCRIPT_CLEANUP, 0x0027_d944)?;
                c.g[A1][0] = lw_3(c, S0, 0xa0);
                c.g[V0][0] = lw_3(c, A1, 0x14);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
                c.g[V1][0] = lw_3(c, V0, 0x24);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0027_d95c)?;
            }
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_LoadBankAsync(ScriptEngine* self, int bank, char* path,
/// void* into, int size, int offset)` (0x27b370): store the offset in the
/// bank slot at `self + 0x628 + 16*bank`, start a load (into `into` when given,
/// else a plain async load), then account for a queued bank: bump `+0x620` if
/// the slot got a handle, else clear the slot.
pub fn script_engine_load_bank_async(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save(c, S1, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x628));
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        c.g[A1][0] = c.g[T1][0];
        c.g[A2][0] = c.g[T2][0];
        let plain = c.g[A1][0] == 0;
        sw(c, p(c.g[S0][0] as u32, 4), c.g[T0][0]);
        if !plain {
            c.cycles += 5;
            call_ptr(c, 0x003d_f748, 0x0027_b3ac)?; // AsyncFile_LoadInto
            sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
            sw(c, p(c.g[S0][0] as u32, 0xc), 0);
        } else {
            c.cycles += 5;
            c.g[A1][0] = c.g[A3][0];
            call_ptr(c, 0x003d_f690, 0x0027_b3c0)?; // AsyncFile_Load
            c.g[V1][0] = 1;
            sw(c, p(c.g[S0][0] as u32, 8), c.g[V0][0]);
            sw(c, p(c.g[S0][0] as u32, 0xc), c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 8));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0027_7f08, 0x0027_b3f8)?; // ScriptBankSlot_Clear
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x620));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            sw(c, p(c.g[S1][0] as u32, 0x620), c.g[V0][0]);
            c.g[V1][0] = lw(c, p(c.g[S0][0] as u32, 8));
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0027_7f08, 0x0027_b3f8)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptEngine_ClearObjBindingFilter(ScriptEngine* self, int filter)`
/// (0x27d170): for every binding in the container at `self + 4*filter`,
/// clear the filter slot through `ScriptEngine_ClearBindingFilter`.
pub fn script_engine_clear_obj_binding_filter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S1][0] = lw(c, c.g[A1][0] as u32);
        c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0xc));
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x30)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x34));
        c.g[V0][0] = sx(c.g[V0][0] as u32);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, 0x0027_d1b0)?;
        c.g[S2][0] = c.g[V0][0];
        if (c.g[S2][0] as i64) <= 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0xc));
            loop {
                c.cycles += 12;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x38)).lh(c);
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x3c));
                c.g[V0][0] = sx(c.g[V0][0] as u32);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x0027_d1d8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0027_d558, 0x0027_d1e4)?; // ScriptEngine_ClearBindingFilter
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0xc));
                    if event_due(c) {
                        deliver_event_here(c, 0x0027_d1c0);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break;
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptEngine_ActivateBinding(ScriptEngine* self, Binding* binding,
/// int value)` (0x27d400): run the binding's start method; when it returns 0,
/// also run the value table entry's start, then store the value and the
/// success flag in the binding's slot.
pub fn script_engine_activate_binding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x14;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 4));
        c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x2a8));
        c.mult(0, 3, 3, 2);
        c.g[A0][0] = Addr(p(c.g[A1][0] as u32, 0x20)).lh(c);
        c.g[V0][0] = lw(c, p(c.g[A1][0] as u32, 0x24));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx(c.g[V0][0] as u32);
        c.g[A1][0] = lw(c, p(c.g[S1][0] as u32, 4));
        call_ptr(c, c.g[V0][0] as u32, 0x0027_d448)?;
        let nonzero = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !nonzero {
            c.cycles += 11;
            c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 4));
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = lw(c, c.g[V0][0] as u32);
            c.g[A2][0] = c.g[V0][0];
            c.g[V1][0] = lw(c, p(c.g[A2][0] as u32, 0xc));
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x20)).lh(c);
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x24));
            c.g[V0][0] = sx(c.g[V0][0] as u32);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0027_d478)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 8;
        sw(c, p(c.g[S1][0] as u32, 8), c.g[S2][0]);
        sw(c, p(c.g[S1][0] as u32, 0xc), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ScriptEngine_OnFilterLoaded(self, file, data)` (0x27b6b0): take the
/// async filter result, bump the pending count and relocate each used
/// entry's offset.
pub fn script_engine_on_filter_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        sw_2(c, S0, 0x538, 0);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, table::ASYNC_FILE_GET_RESULT, 0x0027_b6d4)?;
        c.g[V1][0] = lw_4(c, S0, 0x620);
        c.g[V0][0] = Addr(c.g[SP][0] as u32).off(4).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let no_data = c.g[V0][0] == 0;
        sw_2(c, S0, 0x620, c.g[V1][0]);
        if !no_data {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            sw_2(c, S0, 0x53c, c.g[V0][0]);
            sw_2(c, S0, 0x540, c.g[A0][0]);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A2][0] = 0;
            if !empty {
                c.cycles += 1;
                c.g[V0][0] = lw_4(c, S0, 0x540);
                'entries: loop {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
                    let skip = c.g[A0][0] == 0;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    if !skip {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        Addr(c.g[A1][0] as u32).off(4).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 6;
                    c.g[V1][0] = lw_4(c, S0, 0x53c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw_4(c, S0, 0x540);
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_b708);
                        }
                        continue 'entries;
                    }
                    break 'entries;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_DeactivateBinding(self, key)` (0x27d4a0): clear the
/// binding slot for `key[1]` and notify its listeners.
pub fn script_engine_deactivate_binding(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x14;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(4).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x50));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S2][0] = lw_4(c, S0, 0x8);
        sw_2(c, S0, 0x8, c.g[A0][0]);
        sw_2(c, S0, 0xc, 0);
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = lw_4(c, S0, 0x4);
        c.g[V1][0] = lw_4(c, S1, 0x2a8);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = lw_4(c, V1, 0x24);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0027_d4f8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 11;
            c.g[V0][0] = lw_4(c, S0, 0x4);
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0xc).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
            c.g[V0][0] = lw_4(c, V1, 0x2c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_d528)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_v03(self, int idx)` (0x279da8): drop the script view
/// object of `idx` (removing its camera or setting its transition, by state)
/// and then remove the script object.
pub fn script_engine_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0027_d320, 0x00279dc4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = 1;
        let state1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        if state1 {
            c.cycles += 8;
            c.g[A0][0] = Addr(p(r32(c, A1), 0x7c)).lw(c);
            c.g[V0][0] = 0x120;
            c.mult(0, 4, 4, 2);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, 0x0028_11e8, 0x00279df0)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 12;
                c.g[V1][0] = Addr(p(r32(c, A1), 0xc)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
                c.g[V0][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
                let trans = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
                call_ptr(c, trans, 0x00279e14)?;
                c.g[V1][0] = 0x120;
                c.g[A1][0] = 0;
                c.mult(0, 2, 2, 3);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6d0));
                c.g[A0][0] = sx(p(r32(c, S0), r32(c, V0)));
                call_ptr(c, 0x0028_12e0, 0x00279e2c)?;
                c.g[A0][0] = c.g[S0][0];
            }
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0027_cf88, 0x00279e38)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ScriptEngine_UnloadBank(self, int index)` (0x27b410): release the
/// data instance (or the async file) held by bank slot `index` and clear the
/// slot.
pub fn script_engine_unload_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(r32(c, A1) << 4);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x628));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(r32(c, S0), r32(c, A1)));
        c.g[A1][0] = Addr(r32(c, S1)).lw(c);
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = sx(p(r32(c, S0), 0x51c));
            call_ptr(c, 0x0027_5a20, 0x0027b444)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0031_7e98, 0x0027b464)?;
                }
            }
        } else {
            c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 9;
                call_ptr(c, 0x003d_faf0, 0x0027b47c)?;
                c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x003d_f9d8, 0x0027b48c)?;
                c.g[V1][0] = Addr(p(r32(c, S0), 0x620)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                Addr(p(r32(c, S0), 0x620)).sw(c, c.g[V1][0]);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0027_7f08, 0x0027b4a0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine: spawn a chosen script.

/// `int ScriptEngine_SpawnChosenScript(engine, category, a, b, c)` (0x279528):
/// choose the script, take a free node and start it; on failure push the node
/// back and return the error.
pub fn script_engine_spawn_chosen_script(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'end: {
            c.cycles += 16;
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A2][0];
            frame.save(c, S5, 0x10);
            c.g[S4][0] = c.g[A3][0];
            frame.save(c, S1, 0x50);
            frame.save(c, S0, 0x60);
            frame.save_ra(c, 0);
            c.g[S5][0] = c.g[T0][0];
            callee::SCRIPT_ENGINE_CHOOSE_SCRIPT_2.call(c, 0x0027_955c)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0;
            c.g[V0][0] = sx(0xffff_ffff);
            if t {
                break 'end;
            }
            c.cycles += 13;
            c.g[RA][0] = sx(0x0027_9570);
            c.g[A0][0] = c.g[S2][0];
            callee::SCRIPT_ENGINE_POP_FREE_NODE.call(c, 0x0027_9570)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            Addr(c.g[S0][0] as u32).sw(c, c.g[S3][0]);
            c.g[A2][0] = c.g[S0][0];
            Addr(at_3(c.g[S0][0], 4)).sw(c, c.g[S4][0]);
            Addr(at_3(c.g[S0][0], 8)).sw(c, c.g[S5][0]);
            c.g[RA][0] = sx(0x0027_9590);
            c.g[A1][0] = lw_5(c, c.g[S1][0] as u32);
            callee::SCRIPT_ENGINE_START_SCRIPT.call(c, 0x0027_9590)?;
            c.g[S1][0] = c.g[V0][0];
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[V0][0] = c.g[S1][0];
            if !t {
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0027_95a8);
                c.g[A1][0] = c.g[S0][0];
                callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2.call(c, 0x0027_95a8)?;
                c.g[V0][0] = c.g[S1][0];
            }
            break 'end;
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


/// `int ScriptEngine_v02(engine, obj)` (0x279d00): add the script object and
/// either attach its camera or set its transition, returning the object.
pub fn script_engine_v02(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'end: {
            'plain: {
                c.cycles += 13;
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S2, 0x10);
                frame.save_ra(c, 0);
                c.g[RA][0] = sx(0x0027_9d20);
                c.g[S1][0] = c.g[A0][0];
                callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ.call(c, 0x0027_9d20)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = lw_5(c, c.g[S0][0] as u32);
                c.g[V0][0] = 1;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 2;
                if !t {
                    c.cycles += 9;
                    c.g[A0][0] = lw_5(c, at_3(c.g[S0][0], 0x7c));
                    c.g[V0][0] = 0x120;
                    c.g[A1][0] = c.g[S0][0];
                    c.mult(0, 4, 4, 2);
                    c.g[A0][0] = sx(at_3(c.g[A0][0], 0x6d0));
                    c.g[RA][0] = sx(0x0027_9d50);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    callee::SCRIPT_VIEW_OBJ_ADD_CAM.call(c, 0x0027_9d50)?;
                    c.g[V0][0] = c.g[S2][0];
                    break 'end;
                }
                break 'plain;
            }
            c.cycles += 2;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S2][0];
            if t {
                break 'end;
            }
            c.cycles += 12;
            c.g[V1][0] = lw_5(c, at_3(c.g[S0][0], 0xc));
            c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x48));
            c.g[V0][0] = lw_5(c, at_3(c.g[V1][0], 0x4c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0027_9d74);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_9d74)?;
            c.g[V1][0] = 0x120;
            c.g[A1][0] = c.g[S0][0];
            c.mult(0, 2, 2, 3);
            c.g[V0][0] = sx(at_3(c.g[V0][0], 0x6d0));
            c.g[RA][0] = sx(0x0027_9d8c);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            callee::SCRIPT_VIEW_OBJ_SET_TRANSITION.call(c, 0x0027_9d8c)?;
            c.g[V0][0] = c.g[S2][0];
            break 'end;
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

// ---------------------------------------------------------------------------
// Script engine: binding filter.

/// `void ScriptEngine_SetObjBindingFilter(engine, index, filter)` (0x27d0c0):
/// apply the filter to every binding of the indexed script object.
pub fn script_engine_set_obj_binding_filter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        'end: {
            c.cycles += 21;
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
            frame.save(c, S3, 0x20);
            frame.save(c, S0, 0x50);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S2, 0x30);
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
            frame.save(c, S4, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S4][0] = c.g[A2][0];
            frame.save(c, S1, 0x40);
            c.g[S1][0] = lw_5(c, c.g[A1][0] as u32);
            c.g[V1][0] = lw_5(c, at_3(c.g[S1][0], 0xc));
            c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x30));
            c.g[V0][0] = lw_5(c, at_3(c.g[V1][0], 0x34));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0027_d108);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0027_d108)?;
            c.g[S2][0] = c.g[V0][0];
            if (c.g[S2][0] as i64) <= 0 {
                frame.restore(c, S0, 0x50);
                break 'end;
            }
            c.cycles += 1;
            c.g[V1][0] = lw_5(c, at_3(c.g[S1][0], 0xc));
            loop {
                c.cycles += 13;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx(at_3(c.g[S0][0], 1));
                c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x38));
                c.g[V0][0] = lw_5(c, at_3(c.g[V1][0], 0x3c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0027_d130);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_d130)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                c.g[RA][0] = sx(0x0027_d140);
                c.g[A2][0] = c.g[S4][0];
                callee::SCRIPT_ENGINE_SET_BINDING_FILTER.call(c, 0x0027_d140)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x50);
                    break 'end;
                }
                c.g[V1][0] = lw_5(c, at_3(c.g[S1][0], 0xc));
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0027_d118);
                }
            }
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


/// `int ScriptEngine_v06(this, key)` (0x278ad0): tick the process with `key`
/// and, if it returned non-zero, update the two views attached to the
/// engine's slot array while their predicate holds.
pub fn script_engine_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let this = abi::arg(c, 0);
        let key = abi::arg(c, 1);
        let frame = Frame::open(c, 0x90);
        frame.save(c, 20, 0x40);
        frame.save(c, 22, 0x20);
        c.g[S4][0] = abi::arg64(c, 0);
        frame.save(c, 23, 0x10);
        frame.save(c, 16, 0x80);
        frame.save(c, 17, 0x70);
        frame.save(c, 18, 0x60);
        frame.save(c, 19, 0x50);
        frame.save(c, 21, 0x30);
        frame.save_ra(c, 0);
        c.g[S6][0] = abi::arg64(c, 1);

        callee::SCRIPT_ENGINE_TICK_PROCESS.call(c, 0x0027_8b04)?;
        let result = abi::r32(c, V0);
        c.g[S7][0] = c.g[V0][0];
        let started = c.g[S7][0] == 0;
        c.g[S3][0] = sx(this.wrapping_add(0x6d0));
        if !started {
            c.cycles += 2;
            c.g[S5][0] = 1;
            loop {
                c.cycles += 14;
                c.g[S1][0] = Addr(this).off(0x2a8).lw(c);
                c.g[A0][0] = c.g[S4][0];
                c.g[S2][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
                c.g[A1][0] = c.g[S6][0];
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                c.g[S0][0] = sx(this.wrapping_add(c.g[S0][0] as u32));
                callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3.call(c, 0x0027_8b34)?;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = Addr(abi::r32(c, V0)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, target, 0x0027_8b48)?;
                if c.g[V0][0] == 0 {
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    callee::SCRIPT_VIEW_OBJ_UPDATE.call(c, 0x0027_8b58)?;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
                }
                c.cycles += 2;
                let more = (c.g[S5][0] as i64) >= 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x120));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8b18);
                }
            }
        }

        c.cycles += 12;
        c.g[V0][0] = c.g[S7][0];
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ScriptEngine_RemoveScriptObj(this, idx)` (0x27cf88): drop the object
/// in slot `idx`, unbinding each of its bindings, then clear the slot.
pub fn script_engine_remove_script_obj(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let this = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 19, 0x20);
        c.g[S0][0] = sx((idx as u32) << 2);
        frame.save(c, 18, 0x30);
        c.g[S3][0] = abi::arg64(c, 0);
        frame.save(c, 20, 0x10);
        c.g[V0][0] = sx(this.wrapping_add(c.g[S0][0] as u32));
        frame.save_ra(c, 0);
        frame.save(c, 17, 0x40);
        c.g[S1][0] = Addr(c.g[V0][0] as u32).lw(c);

        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 8;
            frame.restore(c, 16, 0x50);
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            frame.restore(c, 19, 0x20);
            frame.restore(c, 20, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        c.cycles += 9;
        c.g[S4][0] = c.g[S0][0];
        c.g[S0][0] = 0;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0027_cfdc)?;
        c.g[S2][0] = c.g[V0][0];
        let none = (c.g[S2][0] as i64) <= 0;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
        if !none {
            c.cycles += 2;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            loop {
                c.cycles += 12;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x38).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x3c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_d008)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                callee::SCRIPT_ENGINE_REMOVE_BINDING.call(c, 0x0027_d014)?;
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V1][0] == 0 {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    break;
                }
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_cff0);
                }
            }
        }

        c.cycles += 2;
        c.g[V0][0] = 1;
        Addr(c.g[V1][0] as u32).sw(c, 0);
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


/// `RiderScriptObj_AcceptCommand(this, msg, type)` (0x280458): accept a
/// script command for the rider when it is loaded and its team mask matches.
pub fn rider_script_obj_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let this = abi::arg(c, 0);
        let msg = abi::arg(c, 1);
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 5;
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x40);
        c.g[S3][0] = abi::arg64(c, 1);
        frame.save(c, 17, 0x30);
        let not_five = c.g[A2][0] != c.g[V0][0];
        frame.save_ra(c, 0);

        if not_five {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
        } else {
            c.cycles += 10;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_0494)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            callee::RIDER_IS_LOADED.call(c, 0x0028_04a0)?;
            let not_loaded = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if not_loaded {
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x40));
            } else {
                c.cycles += 13;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_CHARACTER.call(c, 0x0028_04b0)?;
                c.g[V1][0] = 0x0048_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x1bb0));
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S1][0] = Addr(c.g[V0][0] as u32).lw(c);
                callee::SCRIPT_RIDER_GET_PARAMS.call(c, 0x0028_04cc)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
                let no_mask = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                if no_mask {
                    // -> the MatchesRiderSlot tail
                } else {
                    c.cycles += 2;
                    let masked_off = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if masked_off {
                        c.cycles += 1;
                        c.lq(16, frame.at(c, 0x40));
                        c.cycles += 6;
                        frame.restore(c, 17, 0x30);
                        frame.restore(c, 18, 0x20);
                        frame.restore(c, 19, 0x10);
                        frame.restore_ra(c, 0);
                        frame.close(c);
                        return Ok(());
                    }
                }
                c.cycles += 7;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lb(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = Addr(c.g[S3][0] as u32).off(8).lw(c);
                c.g[A1][0] = c.g[S0][0];
                callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT.call(c, 0x0028_04f8)?;
                c.lq(16, frame.at(c, 0x40));
            }
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine.

/// `void ScriptEngine_OnBankLoaded(ScriptEngine* self, int bank)` (0x27b5d8):
/// walk the ten script-bank slots; when one holds the loaded bank, read its
/// length, clear the slot, hand the data to the instance manager and record
/// the new data id.
pub fn script_engine_on_bank_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xf0);
        frame.save(c, S1, 0xd0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0xa0);
        c.g[S3][0] = 0x480000;
        frame.save_ra(c, 0x90);
        c.g[S4][0] = 0xa;
        c.g[S0][0] = sx(p(r32(c, S1), 0x628));
        let mut via_clear = false;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            if c.g[V0][0] != c.g[A0][0] {
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            } else {
                c.cycles += 8;
                c.g[A1][0] = sx(p(r32(c, SP), 0x80));
                c.g[A2][0] = sx(p(r32(c, SP), 0x84));
                call_ptr(c, 0x003d_f9d8, 0x0027_b628)?;
                c.g[V1][0] = Addr(p(r32(c, S1), 0x620)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, SP), 0x84)).lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                let pending = c.g[V0][0] != 0;
                Addr(p(r32(c, S1), 0x620)).sw(c, c.g[V1][0]);
                if !pending {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0027_7f08, 0x0027_b644)?;
                    via_clear = true;
                    break;
                }
                c.cycles += 14;
                c.g[A2][0] = Addr(p(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(p(r32(c, S3), 0x1eb0));
                call_ptr(c, 0x0041_7828, 0x0027_b65c)?;
                Addr(p(r32(c, S0), 8)).sw(c, 0);
                c.g[A0][0] = sx(p(r32(c, S1), 0x51c));
                Addr(p(r32(c, S1), 0xa48)).sw(c, c.g[S2][0]);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = Addr(p(r32(c, SP), 0x80)).lw(c);
                c.g[A3][0] = Addr(p(r32(c, SP), 0x84)).lw(c);
                call_ptr(c, 0x0027_5920, 0x0027_b678)?;
                Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                Addr(p(r32(c, S1), 0xa48)).sw(c, c.g[S4][0]);
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p(r32(c, S0), 0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_b610);
            }
        }
        c.lq(S0, frame.at(c, 0xe0));
        if !via_clear {
            c.cycles += 1;
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// ScriptEngine_AddScriptObj (0x000027cea8).

/// `ScriptEngine_AddScriptObj`: register a script owner in the first free of 20 slots and add its bindings.
pub fn script_engine_add_script_obj(c: &mut Cpu) {
    guest(c, |c| {
    'b_27cf04: {
    'b_27cee0: {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S1][0] = c.g[A1][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S4][0] = c.g[A0][0];
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S4][0] as u32));
    let t = c.g[V0][0] != 0u64;
    c.g[S3][0] = 0u64;
    if t {
    break 'b_27cee0;
    } else {
    c.cycles += 2;
    let t = true;
    c.sw((c.g[S4][0] as u32), c.g[S1][0]);
    if t {
    break 'b_27cf04;
    } else {
    break 'b_27cee0;
    }
    }
    }
    c.cycles += 1;
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
    'l_27cee4: loop {
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S3][0] as i64) < 20i64) as u64;
    let t = c.g[V0][0] == 0u64;
    c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
    if t {
    break 'b_27cf04;
    } else {
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
    if c.g[V1][0] != 0u64 {
    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
    if event_due(c) { deliver_event_here(c, 0x0027cee4); } continue 'l_27cee4;
    } else {
    c.cycles += 1;
    c.sw((c.g[V0][0] as u32), c.g[S1][0]);
    break 'b_27cf04;
    }
    }
    }
    }
    'b_27cf68: {
    c.cycles += 9;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    c.g[S0][0] = 0u64;
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0027cf1c);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0027cf1c { return Err(crate::call::Unwound); }
    c.g[S2][0] = c.g[V0][0];
    let t = (c.g[S2][0] as i64) <= 0;
    c.g[V0][0] = c.g[S3][0];
    if t {
    break 'b_27cf68;
    } else {
    c.cycles += 2;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    'l_27cf30: loop {
    c.cycles += 13;
    c.g[A1][0] = c.g[S0][0];
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x0027cf48);
    c.pc = t;
    ssx3_ee_rt::funcs::call_indirect(c, t);
    if c.pc != 0x0027cf48 { return Err(crate::call::Unwound); }
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S3][0];
    call_ptr(c, 0x0027d330, 0x0027cf58)?;
    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
    if c.g[V1][0] != 0u64 {
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    if event_due(c) { deliver_event_here(c, 0x0027cf30); } continue 'l_27cf30;
    } else {
    c.cycles += 1;
    c.g[V0][0] = c.g[S3][0];
    break 'b_27cf68;
    }
    }
    }
    }
    c.cycles += 8;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t; return Ok(());
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Script engine.

/// `int ScriptEngine_SpawnScript(ScriptEngine* self, int key, int a, int b,
/// int c)` (0x2795d0): allocate a script node, clamp the three rider indices
/// to `[-1, rider_count)`, fill the node and start the script; a failed start
/// returns the node to the free list.
pub fn script_engine_spawn_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A3][0];
        c.g[S3][0] = c.g[T0][0];
        c.g[S4][0] = c.g[A0][0];
        c.g[S5][0] = c.g[A1][0];
        frame.save_ra(c, 0);

        // rider_count = world->+0x84->+0xc->+0x78
        c.g[V0][0] = Addr(gp_7(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(r(c, V0)).off(0x84).lw(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0xc).lw(c);
        let first_negative = (c.g[S1][0] as i64) < 0;
        c.g[V1][0] = Addr(r(c, V0)).off(0x78).lw(c);
        if first_negative {
            c.cycles += 1;
            c.g[S1][0] = u64::MAX;
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if r(c, V0) == 0 {
                c.cycles += 1;
                c.g[S1][0] = u64::MAX;
            }
        }

        // a3
        c.cycles += 2;
        let negative = (c.g[S2][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let in_range = if negative {
            false
        } else {
            c.cycles += 2;
            r(c, V0) != 0
        };
        if !in_range {
            c.cycles += 1;
            c.g[S2][0] = u64::MAX;
        }

        // t0
        c.cycles += 2;
        let negative = (c.g[S3][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let in_range = if negative {
            false
        } else {
            c.cycles += 2;
            r(c, V0) != 0
        };
        if !in_range {
            c.cycles += 1;
            c.g[S3][0] = u64::MAX;
        }

        c.cycles += 13;
        c.g[A0][0] = c.g[S4][0];
        callee::SCRIPT_POP_FREE_NODE.call(c, 0x0027_9658)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S5][0];
        c.sw(r(c, S0), c.g[S1][0]);
        c.g[A0][0] = c.g[S4][0];
        c.sw(p(r(c, S0), 0x4), c.g[S2][0]);
        c.g[A2][0] = c.g[S0][0];
        c.sw(p(r(c, S0), 0x8), c.g[S3][0]);
        callee::SCRIPT_START_SCRIPT.call(c, 0x0027_9678)?;

        c.g[S1][0] = c.g[V0][0];
        let started = (c.g[S1][0] as i64) >= 0;
        c.g[V0][0] = c.g[S1][0];
        if !started {
            c.cycles += 4;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            callee::SCRIPT_PUSH_FREE_NODE.call(c, 0x0027_9690)?;
            c.g[V0][0] = c.g[S1][0];
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


/// `ScriptViewObj_Deactivate(self, a1)` (0x281400): clear the object's
/// camera bindings (two slots), stop its transition, drop its world binding,
/// then reset and deactivate the view object.
pub fn scriptviewobj_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x50);
        frame.save(c, 21, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, 18, 0x40);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xdc));
        frame.save(c, 19, 0x30);
        c.g[S2][0] = 1;
        frame.save(c, 20, 0x20);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_143c);
        c.g[S4][0] = sx(0xffff_ffff);
        c.pc = 0x0027_d2e8;
        ee::s07::ScriptEngine_GetInstance(c);
        if c.pc != 0x0028_143c {
            return Err(Unwound);
        }
        c.g[A1][0] = lw(c, p(c.g[S1][0] as u32, 4));
        c.g[RA][0] = sx(0x0028_1448);
        c.g[A0][0] = c.g[V0][0];
        c.pc = 0x0027_d170;
        ee::s07::ScriptEngine_ClearObjBindingFilter(c);
        if c.pc != 0x0028_1448 {
            return Err(Unwound);
        }
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[S0][0] as u32);
            let t = (c.g[V0][0] as i64) < 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !t {
                c.cycles += 5;
                c.g[A1][0] = 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[RA][0] = sx(0x0028_1464);
                c.g[A0][0] = lw(c, c.g[V0][0] as u32);
                c.pc = 0x0027_d688;
                ee::s07::ScriptObjCam_SetBinding(c);
                if c.pc != 0x0028_1464 {
                    return Err(Unwound);
                }
                c.sw(c.g[S0][0] as u32, c.g[S4][0]);
            }
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let t = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !t {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_1448);
            }
        }
        c.cycles += 3;
        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0xd8));
        let t = c.g[A0][0] == 0;
        c.g[A1][0] = sx(0xffff_ffff);
        if !t {
            c.cycles += 6;
            c.mtc1(0, 12);
            c.g[A2][0] = 9;
            c.mov_s(13, 12);
            c.mov_s(14, 12);
            c.g[RA][0] = sx(0x0028_1498);
            c.mov_s(15, 12);
            c.pc = 0x0028_2db0;
            ee::s07::cTransitionMan_StopTransition(c);
            if c.pc != 0x0028_1498 {
                return Err(Unwound);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x110));
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = lw(c, gp_8(c, -0x848));
        if !t {
            c.cycles += 6;
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x100));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x84));
            c.g[A2][0] = lw(c, p(c.g[V1][0] as u32, 0xc));
            c.g[RA][0] = sx(0x0028_14b8);
            c.g[A0][0] = lw(c, p(c.g[A2][0] as u32, 0xa4));
            call_ptr(c, 0x0010_33f8, 0x0028_14b8)?;
            c.sw(p(c.g[S1][0] as u32, 0x110), 0);
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x0028_14c4);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0028_1a50;
        ee::s07::ScriptViewObj_Reset(c);
        if c.pc != 0x0028_14c4 {
            return Err(Unwound);
        }
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0028_14d0);
        c.g[A1][0] = c.g[S5][0];
        c.pc = 0x0028_3440;
        ee::s07::ScriptObj_Deactivate(c);
        if c.pc != 0x0028_14d0 {
            return Err(Unwound);
        }
        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore(c, 20, 0x20);
        frame.restore(c, 21, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptObjCam_SetBinding(self, anim)` (0x27d688): bind a script-object
/// camera to an animation: tell the old binding process that its animation
/// is going away (`-1` for none), record the animation and its id from
/// `ScriptEngine_GetBindingProcess`, then tell the new process.
pub fn script_obj_cam_set_binding(c: &mut Cpu) {
    const GET_INSTANCE: Callee =
        Callee { addr: 0x0027_d2e8, f: ee::s07::ScriptEngine_GetInstance };
    const GET_BINDING_PROCESS: Callee =
        Callee { addr: 0x0027_d2f0, f: ee::s07::ScriptEngine_GetBindingProcess };

    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let no_anim = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !no_anim {
            c.cycles += 9;
            GET_INSTANCE.call(c, 0x0027_d6b0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            GET_BINDING_PROCESS.call(c, 0x0027_d6bc)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            if c.g[V0][0] == 0 {
                sw_3(c, p(c.g[S0][0] as u32, 0x14), c.g[S1][0]);
            } else {
                c.cycles += 7;
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
                c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
                c.g[A0][0] = lh_2(c, p(c.g[V0][0] as u32, 0x28));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0027_d6e4)?;
                sw_3(c, p(c.g[S0][0] as u32, 0x14), c.g[S1][0]);
            }
            c.cycles += 9;
            c.g[A1][0] = c.g[S2][0];
            sw_3(c, p(c.g[S0][0] as u32, 0x20), c.g[S2][0]);
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
            c.g[A0][0] = lh_2(c, p(c.g[V0][0] as u32, 0x20));
            c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x24));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0027_d704)?;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x14));
            let unbound = c.g[V0][0] == 0;
            c.g[V0][0] = 0x0050_0000;
            if !unbound {
                c.cycles += 22;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                c.lqc2(1, c.g[V0][0] as u32);
                c.lqc2(2, p(c.g[V0][0] as u32, 0x10));
                c.lqc2(3, p(c.g[V0][0] as u32, 0x20));
                c.lqc2(4, p(c.g[V0][0] as u32, 0x30));
                c.sqc2(1, c.g[V1][0] as u32);
                c.sqc2(2, p(c.g[V1][0] as u32, 0x10));
                c.sqc2(3, p(c.g[V1][0] as u32, 0x20));
                c.sqc2(4, p(c.g[V1][0] as u32, 0x30));
                c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0xc));
                sw_3(c, p(c.g[S0][0] as u32, 0x70), 0);
                sw_3(c, p(c.g[S0][0] as u32, 0x74), 0);
                sw_3(c, p(c.g[S0][0] as u32, 0x78), 0);
                c.g[A1][0] = lw(c, p(c.g[S0][0] as u32, 0x20));
                c.g[A0][0] = lh_2(c, p(c.g[V0][0] as u32, 0x28));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0027_d764)?;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                sw_3(c, p(c.g[S0][0] as u32, 0x14), 0);
                sw_3(c, p(c.g[S0][0] as u32, 0x20), c.g[V0][0]);
            }
            c.cycles += 1;
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


/// A virtual call: `vt = *(obj + 4)`, `fn = *(vt + slot + 4)`,
/// `this = obj + *(short *)(vt + slot)`, leaving `fn` in `a1` (the register
/// the original issues it from) and returning to `ret`.
#[inline]
pub(crate) fn call_slot(c: &mut Cpu, obj: u64, slot: u32, ret: u32) -> Flow {
    let vt = Addr((obj as u32).wrapping_add(4)).lw(c);
    c.g[V1][0] = vt;
    c.g[A0][0] = Addr((vt as u32).wrapping_add(slot)).lh(c);
    c.g[A1][0] = Addr((vt as u32).wrapping_add(slot + 4)).lw(c);
    let func = c.g[A1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}


/// Read track `index` of the sequencer channel `chan`.
#[inline]
pub(crate) fn channel_track(c: &mut Cpu, chan: u64, index: u32, ret: u32) -> Flow {
    c.g[A0][0] = chan;
    c.g[A1][0] = index as u64;
    callee::IN_SEQ_CHANNEL_GET_TRACK.call(c, ret)
}


// ---------------------------------------------------------------------------
// Script sequencer field readers.

/// `void ScriptCam_ReadSubjectFields(ScriptCam* self, float* out)`
/// (0x27c510): ten subject values; `out[i] = *track_i->v04()`.
pub fn script_cam_read_subject_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c544;
        c.cycles += 100;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..9u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void ScriptCam_ReadAlgorithmicFields(ScriptCam* self, float* out)`
/// (0x27c6f8): the same ten subject values as
/// [`script_cam_read_subject_fields`] but for the algorithmic camera.
pub fn script_cam_read_algorithmic_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c72c;
        c.cycles += 100;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..9u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void ScriptCam_ReadManualFields(ScriptCam* self, float* out)`
/// (0x27c0f0): ten manual-camera values.
pub fn script_cam_read_manual_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c110;
        c.cycles += 110;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..10u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void ScriptCam_ReadTargetFields(ScriptCam* self, float* out)`
/// (0x27c300): ten target-camera values.
pub fn script_cam_read_target_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_c320;
        c.cycles += 110;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..10u32 {
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.lwc1(0, src);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(4 * i));
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void ScriptAudio_ReadFields(ScriptAudio* self, int* out)` (0x27cc28):
/// five tracks, two ids each: `out[2i] = *track_i->v04()`,
/// `out[2i+1] = track_i->v05()`.
pub fn script_audio_read_fields(c: &mut Cpu) {
    guest(c, |c| {
        const RET: u32 = 0x0027_cc48;
        c.cycles += 105;
        let f = Frame::open(c, 0x30);
        f.save(c, S0, 0x20);
        f.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        f.save_ra(c, 0);
        for i in 0..5u32 {
            let out = c.g[S1][0] as u32;
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x20, RET)?;
            let src = c.g[V0][0] as u32;
            c.g[V1][0] = c.lw(src);
            c.sw(out.wrapping_add(8 * i), c.g[V1][0]);
            channel_track(c, c.g[S0][0], i, RET)?;
            let track = c.g[V0][0];
            call_slot(c, track, 0x28, RET)?;
            let v = c.g[V0][0];
            c.sw(out.wrapping_add(8 * i + 4), v);
        }
        c.lq(16, f.at(c, 0x20));
        c.lq(17, f.at(c, 0x10));
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void func_0027A860(void* engine, int online, int intro)` (0x27a860):
/// queue the script choices for the end of a run: choices 0xc..0xf (or
/// 0x12..0x15 when online), with the first one only when `intro`, then
/// prepare and start the choice list and tell the audio system (music event
/// 0x14) to stop the race music.
pub fn script_engine_queue_finish_scripts(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_27a9ac: {
        'b_27a98c: {
            'b_27a974: {
                'b_27a8f8: {
                    c.cycles += 5;
                    frame.save(c, S0, 0x10);
                    sd(c, r32(c, SP), c.g[RA][0]);
                    let t = c.g[A1][0] == 0u64;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'b_27a8f8;
                    } else {
                        'b_27a898: {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_27a898;
                            } else {
                                c.cycles += 7;
                                c.g[A2][0] = 0x12;
                                c.g[A3][0] = 0x1;
                                c.g[T0][0] = 0x1;
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x0027a898);
                                c.g[T3][0] = 0xffffffffffffffff;
                                if callee::C00278e50.call(c, 0x0027a898).is_err() {
                                    return;
                                }
                                break 'b_27a898;
                            }
                        }
                        'b_27a8e8: {
                            c.cycles += 16;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0x1;
                            c.g[A2][0] = 0x13;
                            c.g[A3][0] = 0u64;
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[T2][0] = 0xffffffffffffffff;
                            c.g[RA][0] = sx(0x0027a8bc);
                            c.g[T3][0] = 0xffffffffffffffff;
                            if callee::C00278e50.call(c, 0x0027a8bc).is_err() {
                                return;
                            }
                            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = lw(c, at(c, A0, 0x84));
                            c.g[V1][0] = lw(c, at(c, V0, 0xc));
                            c.g[A0][0] = lw(c, at(c, V1, 0x7c));
                            let t = c.g[A0][0] != c.g[A1][0];
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_27a8e8;
                            } else {
                                c.cycles += 4;
                                c.g[A2][0] = 0x14;
                                c.g[A3][0] = 0x8;
                                let t = true;
                                c.g[T0][0] = 0u64;
                                if t {
                                    break 'b_27a974;
                                } else {
                                    break 'b_27a8e8;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A2][0] = 0x15;
                        c.g[A3][0] = 0x8;
                        let t = true;
                        c.g[T0][0] = 0u64;
                        if t {
                            break 'b_27a974;
                        } else {
                            break 'b_27a8f8;
                        }
                    }
                }
                'b_27a920: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_27a920;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = 0x1;
                        c.g[A2][0] = 0xc;
                        c.g[A3][0] = 0x1;
                        c.g[T0][0] = 0x1;
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[T2][0] = 0xffffffffffffffff;
                        c.g[RA][0] = sx(0x0027a920);
                        c.g[T3][0] = 0xffffffffffffffff;
                        if callee::C00278e50.call(c, 0x0027a920).is_err() {
                            return;
                        }
                        break 'b_27a920;
                    }
                }
                c.cycles += 18;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0xd;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[RA][0] = sx(0x0027a944);
                c.g[T3][0] = 0xffffffffffffffff;
                if callee::C00278e50.call(c, 0x0027a944).is_err() {
                    return;
                }
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                sw(c, at(c, V1, 0xf44), c.g[A1][0]);
                c.g[V0][0] = lw(c, at(c, A0, 0x84));
                c.g[V1][0] = lw(c, at(c, V0, 0xc));
                c.g[A0][0] = lw(c, at(c, V1, 0x7c));
                let t = c.g[A0][0] != c.g[A1][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_27a98c;
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = 0xe;
                    c.g[A3][0] = 0x8;
                    c.g[T0][0] = 0x1;
                    break 'b_27a974;
                }
            }
            c.cycles += 6;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[RA][0] = sx(0x0027a984);
            c.g[T3][0] = 0xffffffffffffffff;
            if callee::C00278e50.call(c, 0x0027a984).is_err() {
                return;
            }
            let t = true;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_27a9ac;
            } else {
                break 'b_27a98c;
            }
        }
        c.cycles += 8;
        c.g[A2][0] = 0xf;
        c.g[A3][0] = 0x8;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0xffffffffffffffff;
        c.g[RA][0] = sx(0x0027a9a8);
        c.g[T3][0] = 0xffffffffffffffff;
        if callee::C00278e50.call(c, 0x0027a9a8).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S0][0];
        break 'b_27a9ac;
    }
    c.cycles += 17;
    c.g[RA][0] = sx(0x0027a9b4);
    c.g[A1][0] = 0x1;
    if callee::C00278f38.call(c, 0x0027a9b4).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x1;
    c.g[A2][0] = 0u64;
    c.g[RA][0] = sx(0x0027a9c8);
    c.g[A3][0] = 0x1;
    if callee::C00278f68.call(c, 0x0027a9c8).is_err() {
        return;
    }
    c.g[RA][0] = sx(0x0027a9d0);
    if callee::C0028b180.call(c, 0x0027a9d0).is_err() {
        return;
    }
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x14;
    c.g[RA][0] = sx(0x0027a9e0);
    c.g[A2][0] = 0x1;
    if callee::C0028e8c0.call(c, 0x0027a9e0).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x20));
    c.pc = t;
    return;
}


/// `void func_0027AAF8(void* engine, int state)` (0x27aaf8): queue the intro
/// script choices for the script-engine state `state`. State 0 walks the
/// `D_00445488[...]`/`D_00481E80` choice table (pairs `<kind>, <arg>`, ended
/// by a negative kind); states 1 and 2 queue choice 5, state 3 queues 4 and
/// then 5, and anything else does nothing.
pub fn script_engine_queue_intro_scripts(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_27ac58: {
        'b_27ac54: {
            'b_27ac08: {
                'b_27abe0: {
                    'b_27ab38: {
                        'b_27ab24: {
                            c.cycles += 5;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                            frame.save(c, S0, 0x10);
                            let t = c.g[V0][0] == 0u64;
                            sd(c, r32(c, SP), c.g[RA][0]);
                            if t {
                                break 'b_27ab24;
                            } else {
                                c.cycles += 2;
                                let t = (c.g[A1][0] as i64) > 0;
                                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
                                if t {
                                    break 'b_27abe0;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A1][0] == 0u64;
                                    frame.restore(c, S0, 0x10);
                                    if t {
                                        break 'b_27ab38;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[RA][0] = ld(c, r32(c, SP));
                                        if t {
                                            break 'b_27ac58;
                                        } else {
                                            break 'b_27ab24;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[A1][0] == c.g[V0][0];
                        frame.restore(c, S0, 0x10);
                        if t {
                            break 'b_27ac08;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[RA][0] = ld(c, r32(c, SP));
                            if t {
                                break 'b_27ac58;
                            } else {
                                break 'b_27ab38;
                            }
                        }
                    }
                    'b_27ab8c: {
                        c.cycles += 20;
                        c.g[RA][0] = sx(0x0027ab40);
                        if call_ptr(c, 0x0014dc80, 0x0027ab40).is_err() {
                            return;
                        }
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x0027ab4c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x0014dd58, 0x0027ab4c).is_err() {
                            return;
                        }
                        c.g[A0][0] = 0x530000;
                        c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                        c.g[V0][0] = lb(c, at(c, A0, 0x5c11));
                        c.g[V1][0] = 0x440000;
                        c.g[V1][0] = sx(at(c, V1, 0x5488));
                        c.g[A0][0] = lw(c, at(c, A1, 0x78));
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[A1][0] = 0x7;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[RA][0] = sx(0x0027ab78);
                        c.g[S0][0] = lw(c, r32(c, V0));
                        if call_ptr(c, 0x0014dd58, 0x0027ab78).is_err() {
                            return;
                        }
                        c.g[V1][0] = 0x530000;
                        c.g[V0][0] = lw(c, at(c, V1, 0x4b30));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x480000;
                        if t {
                            break 'b_27ab8c;
                        } else {
                            c.cycles += 1;
                            c.g[S0][0] = sx(at(c, V0, 0x1e80));
                            break 'b_27ab8c;
                        }
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0u64 {
                        frame.restore(c, S0, 0x10);
                        break 'b_27ac54;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, r32(c, S0));
                        if (c.g[V0][0] as i64) < 0 {
                            frame.restore(c, S0, 0x10);
                            break 'b_27ac54;
                        } else {
                            c.cycles += 2;
                            c.g[A2][0] = lw(c, r32(c, S0));
                            'l_27aba8: loop {
                                c.cycles += 12;
                                c.g[A1][0] = 0u64;
                                c.g[A3][0] = lw(c, at(c, S0, 0x4));
                                c.g[T0][0] = 0u64;
                                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x0027abc8);
                                c.g[T3][0] = 0xffffffffffffffff;
                                if callee::C00278e50.call(c, 0x0027abc8).is_err() {
                                    return;
                                }
                                c.g[S0][0] = sx(at(c, S0, 0x8));
                                c.g[V0][0] = lw(c, r32(c, S0));
                                if (c.g[V0][0] as i64) >= 0 {
                                    c.g[A2][0] = lw(c, r32(c, S0));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0027aba8);
                                    }
                                    continue 'l_27aba8;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    frame.restore(c, S0, 0x10);
                                    if t {
                                        break 'b_27ac54;
                                    } else {
                                        break 'b_27abe0;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 10;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0x5;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[T2][0] = 0xffffffffffffffff;
                c.g[RA][0] = sx(0x0027ac00);
                c.g[T3][0] = 0xffffffffffffffff;
                if callee::C00278e50.call(c, 0x0027ac00).is_err() {
                    return;
                }
                let t = true;
                frame.restore(c, S0, 0x10);
                if t {
                    break 'b_27ac54;
                } else {
                    break 'b_27ac08;
                }
            }
            c.cycles += 19;
            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x4;
            c.g[A3][0] = 0x3;
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[RA][0] = sx(0x0027ac2c);
            c.g[T3][0] = 0xffffffffffffffff;
            if callee::C00278e50.call(c, 0x0027ac2c).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x84c));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x5;
            c.g[A3][0] = 0u64;
            c.g[T0][0] = 0u64;
            c.g[T1][0] = 0xffffffffffffffff;
            c.g[T2][0] = 0xffffffffffffffff;
            c.g[RA][0] = sx(0x0027ac50);
            c.g[T3][0] = 0xffffffffffffffff;
            if callee::C00278e50.call(c, 0x0027ac50).is_err() {
                return;
            }
            frame.restore(c, S0, 0x10);
            break 'b_27ac54;
        }
        c.cycles += 1;
        c.g[RA][0] = ld(c, r32(c, SP));
        break 'b_27ac58;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x20));
    c.pc = t;
    return;
}

// ---------------------------------------------------------------------------
// Script engine update (tick rate).

/// The callees of [`script_engine_update`].
mod engine_update_callee {
    use super::{ee, Callee};
    pub(super) const LIST_UPDATE: Callee = Callee { addr: 0x0027_6cc8, f: ee::s07::ScriptList_Update };
    pub(super) const UPDATE_PROCESSES: Callee = Callee { addr: 0x0028_20b0, f: ee::s07::ScriptEngine_UpdateProcesses };
    pub(super) const MOVIE_UPDATE: Callee = Callee { addr: 0x0028_3aa0, f: ee::s07::ScriptMovie_Update };
    pub(super) const VIEW_UPDATE: Callee = Callee { addr: 0x0028_1100, f: ee::s07::ScriptViewObj_Update };
    pub(super) const AUDIO_UPDATE: Callee = Callee { addr: 0x0028_0640, f: ee::s07::ScriptAudioObj_UpdateSounds };
    pub(super) const CAN_SKIP: Callee = Callee { addr: 0x0027_7060, f: ee::s07::ScriptList_CanSkip };
    pub(super) const GET_STATE: Callee = Callee { addr: 0x0027_6b98, f: ee::s07::ScriptList_GetState };
    pub(super) const EXIT_SCRIPT_MODE: Callee = Callee { addr: 0x0027_bf90, f: ee::s07::ScriptEngine_ExitScriptMode };
    pub(super) const IS_PAUSED: Callee = Callee { addr: 0x0027_7298, f: ee::s07::ScriptList_IsPaused };
    pub(super) const SKIP_CURRENT: Callee = Callee { addr: 0x0027_6f48, f: ee::s07::ScriptList_SkipCurrent };
}

/// `bool InputBinding_EvalBool(binding, id)` (through the function table).
const INPUT_EVAL_BOOL: u32 = 0x0032_0c48;
/// The skip button's input id.
const INPUT_SKIP: u64 = 0x6f;
/// `GetHashValue32(name)`, the UI state stack lookup, the HUD flag set/clear
/// (through the function table).
const HASH_VALUE32: u32 = 0x0031_7670;
const UI_STATE_FIND: u32 = 0x0039_f9d8;
const HUD_FLAG_SET: u32 = 0x001f_36c0;
const HUD_FLAG_CLEAR: u32 = 0x001f_36d8;

/// Script engine fields.
const ENGINE_LISTS_A: u32 = 0x54c;
const ENGINE_LISTS_B: u32 = 0x624;
const ENGINE_ACTIVE_LIST: u32 = 0x550;
/// Updates (60 Hz frames) before a running script may be skipped
/// (`GameStateEventSelect_Update` sets 30 so the press that started the
/// event cannot skip its intro).
const ENGINE_SKIP_DELAY: u32 = 0x554;

/// Where [`script_engine_update`] continues (its loop heads are resume
/// points of the original's dispatcher: an event at a back edge may unwind
/// the thread there).
#[derive(Clone, Copy)]
enum EngineStage {
    /// 0x278750: the two lists at +0x54c (s1 = offset, s0 = count left).
    ListsA,
    /// 0x278778: the two lists at +0x624.
    ListsB,
    /// 0x2787a8: the two view objects at +0x6d0 (s0 = object, s1 = count left).
    Views,
    /// 0x278810: the riders' skip buttons (s0 = rider slot, s1 = index,
    /// s3 = count, s4 = the active list).
    Riders,
    /// 0x278850: the active list's state, the HUD flag.
    State,
}

/// `void ScriptEngine_Update(ScriptEngine* self)` (0x278720): one update of
/// the script engine: its four script lists, the processes, the movie, the
/// two view objects and the script sounds; then, while a list is active
/// (+0x550 != 2), a rider pressing skip skips its current entry (once the
/// skip delay is over), the engine leaves script mode when the list stopped
/// playing, and the HUD flag follows the list's pause. The skip delay
/// (+0x554) counts down 60 Hz frames: above 60 Hz one per 60 Hz frame
/// (`tick::frame60_due`), so it lasts the same wall time.
pub fn script_engine_update(c: &mut Cpu) {
    use engine_update_callee as callee;
    const FRAME: Frame = Frame { size: 0x70 };
    guest(c, |c| {
        let mut stage = match c.pc {
            0x0027_8720 => {
                c.cycles += 12;
                let f = Frame::open(c, 0x70);
                f.save(c, S0, 0x60);
                f.save(c, S1, 0x50);
                c.g[S0][0] = 1;
                f.save(c, S2, 0x40);
                c.g[S1][0] = 0;
                f.save(c, S3, 0x30);
                c.g[S2][0] = c.g[A0][0];
                f.save(c, S4, 0x20);
                f.save(c, S5, 0x10);
                f.save_ra(c, 0);
                EngineStage::ListsA
            }
            0x0027_8750 => EngineStage::ListsA,
            0x0027_8778 => EngineStage::ListsB,
            0x0027_87a8 => EngineStage::Views,
            0x0027_8810 => EngineStage::Riders,
            0x0027_8850 => EngineStage::State,
            pc => {
                c.jump_out(pc, 0x0027_8720, 0x0027_8914);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                EngineStage::ListsA | EngineStage::ListsB => {
                    let (lists, head, ret) = match stage {
                        EngineStage::ListsA => (ENGINE_LISTS_A, 0x0027_8750, 0x0027_8760),
                        _ => (ENGINE_LISTS_B, 0x0027_8778, 0x0027_8788),
                    };
                    loop {
                        c.cycles += 6;
                        c.g[A0][0] = Addr(r32(c, S2).wrapping_add(lists)).lw(c);
                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                        callee::LIST_UPDATE.call(c, ret)?;
                        let more = (c.g[S0][0] as i64) >= 0;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xcc));
                        if !more {
                            break;
                        }
                        if back_edge_unwinds(c, head) {
                            return Ok(());
                        }
                    }
                    if let EngineStage::ListsA = stage {
                        c.cycles += 4;
                        c.g[S3][0] = sx(r32(c, S2).wrapping_add(0x500));
                        c.g[S4][0] = sx(r32(c, S2).wrapping_add(0x92c));
                        c.g[S1][0] = 0;
                        c.g[S0][0] = 1;
                        EngineStage::ListsB
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x6d0));
                        callee::UPDATE_PROCESSES.call(c, 0x0027_879c)?;
                        c.g[S1][0] = 1;
                        c.g[A0][0] = c.g[S3][0];
                        callee::MOVIE_UPDATE.call(c, 0x0027_87a8)?;
                        EngineStage::Views
                    }
                }
                EngineStage::Views => {
                    loop {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                        callee::VIEW_UPDATE.call(c, 0x0027_87b4)?;
                        let more = (c.g[S1][0] as i64) >= 0;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x120));
                        if !more {
                            break;
                        }
                        if back_edge_unwinds(c, 0x0027_87a8) {
                            return Ok(());
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S4][0];
                    callee::AUDIO_UPDATE.call(c, 0x0027_87c4)?;
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(ENGINE_ACTIVE_LIST)).lw(c);
                    c.g[V0][0] = 2;
                    let idle = c.g[A1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xcc;
                    if idle {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(ENGINE_SKIP_DELAY)).lw(c);
                        return script_engine_update_leave(c, FRAME);
                    }
                    // The active list; can its current entry be skipped?
                    c.cycles += 8;
                    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(ENGINE_LISTS_A)).lw(c);
                    c.mult(0, V0, A1, V0);
                    c.g[S5][0] = 0x4a_0000;
                    c.g[S4][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A0][0] = c.g[S4][0];
                    callee::CAN_SKIP.call(c, 0x0027_87ec)?;
                    let skippable = c.g[V0][0] != 0;
                    c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                    if !skippable {
                        EngineStage::State
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                        c.g[V0][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                        c.g[S3][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
                        let none = (c.g[S3][0] as i64) <= 0;
                        c.g[S1][0] = 0;
                        if none {
                            EngineStage::State
                        } else {
                            c.cycles += 2;
                            c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x40));
                            EngineStage::Riders
                        }
                    }
                }
                EngineStage::Riders => {
                    // A rider with an input map, once the skip delay is over,
                    // pressing skip.
                    c.cycles += 6;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xdf0)).lw(c);
                    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    let mut pressed = false;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(ENGINE_SKIP_DELAY)).lw(c);
                        if c.g[V0][0] == 0 {
                            c.cycles += 4;
                            c.g[A1][0] = INPUT_SKIP;
                            call_ptr(c, INPUT_EVAL_BOOL, 0x0027_883c)?;
                            pressed = c.g[V0][0] != 0;
                        }
                    }
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    if pressed {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S4][0];
                        callee::SKIP_CURRENT.call(c, 0x0027_88c8)?;
                        if back_edge_unwinds(c, 0x0027_8850) {
                            return Ok(());
                        }
                        EngineStage::State
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                        if !more {
                            EngineStage::State
                        } else {
                            if back_edge_unwinds(c, 0x0027_8810) {
                                return Ok(());
                            }
                            EngineStage::Riders
                        }
                    }
                }
                EngineStage::State => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S4][0];
                    callee::GET_STATE.call(c, 0x0027_8858)?;
                    c.g[V1][0] = 3;
                    let playing = c.g[V0][0] == c.g[V1][0];
                    c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                    if !playing {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        callee::EXIT_SCRIPT_MODE.call(c, 0x0027_886c)?;
                        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                    }
                    // The HUD flag (when the HUD state is on the UI stack).
                    c.cycles += 10;
                    c.g[A0][0] = sx(r32(c, S5).wrapping_add(0x34a0));
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                    c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lw(c);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x18));
                    call_ptr(c, HASH_VALUE32, 0x0027_8884)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, UI_STATE_FIND, 0x0027_8890)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S4][0];
                        callee::IS_PAUSED.call(c, 0x0027_88a0)?;
                        let paused = c.g[V0][0] != 0;
                        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
                        c.g[A1][0] = 1;
                        if paused {
                            c.cycles += 6;
                            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x94)).lw(c);
                            call_ptr(c, HUD_FLAG_SET, 0x0027_88b8)?;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
                            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x94)).lw(c);
                            call_ptr(c, HUD_FLAG_CLEAR, 0x0027_88e0)?;
                            c.cycles += 1;
                        }
                    }
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(ENGINE_SKIP_DELAY)).lw(c);
                    return script_engine_update_leave(c, FRAME);
                }
            };
        }
    })
}

/// [`script_engine_update`]'s end (0x2788e4, v0 = the skip delay): the delay
/// goes down by one 60 Hz frame (above 60 only on a 60 Hz frame of its own),
/// then the epilogue.
fn script_engine_update_leave(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let delay = c.g[V0][0];
    c.g[V0][0] = sx((delay as u32).wrapping_sub(1));
    if (delay as i64) > 0 {
        c.cycles += 1;
        let field = r32(c, S2).wrapping_add(ENGINE_SKIP_DELAY);
        let left = if crate::tick::frame60_due(c, field, delay as u32) {
            (delay as u32).wrapping_sub(1)
        } else {
            delay as u32
        };
        Addr(field).sw(c, sx(left));
        crate::tick::frame60_ran(c, field, left);
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
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::fakes;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::GAME_GP;
    use crate::difftest::put_vec;
    use ssx3_ee as ee;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::events::driven;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::main::gamestate::tests::script_list_test;
    pub(crate) use crate::game::scripter::bxscriptengine::tests::script_binding_test;
    pub(crate) use crate::game::dirtysock::tags2::tests::SCRATCH;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::base as base_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_5;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::object::railmodifier::tests::SEEDS_7 as SEEDS_6;
    pub(crate) use crate::game::object::railmodifier::tests::rng as rng_3;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_7;
    pub(crate) use crate::game::animation::animmodel::tests::SEEDS_4 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::world as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_4;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_9;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_11;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::fe::fememcard2::tests::alloc as alloc_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_5;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_13;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_5;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_6;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_6;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_7;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::DST;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14;
    pub(crate) use crate::game::fe::debugfe::tests::FEW;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_10;
    pub(crate) use crate::game::main::gameload::tests::slot_to;
    pub(crate) use crate::game::main::gameload::tests::RET_ZERO;
    pub(crate) use crate::game::movie::movieplayer::tests::RET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_9;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_5;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_6;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_13;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_5;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_15;


    #[test]
    fn script_engine_pop_free_node_matches() {
        check(ssx3_ee::s07::f_0027bd78, script_engine_pop_free_node, 200, &|c, seed| {
            let mut r = Rng(0x27bd_78 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = DATA + 0x100;
            c.bus.write32(DATA + 0x548, if seed % 5 == 0 { 0 } else { node });
            c.bus.write32(node + 0x10, if seed % 4 == 0 { 0 } else { DATA + 0x140 });
        });
    }


    #[test]
    fn script_engine_push_free_node_matches() {
        check(ssx3_ee::s07::f_0027bd90, script_engine_push_free_node, 200, &|c, seed| {
            let mut r = Rng(0x27bd_90 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let node = DATA + 0x100;
            c.g[A1][0] = (node as u64) | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA + 0x548, r.next_u32());
            for i in 0..5 {
                c.bus.write32(node + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn script_engine_get_binding_process_matches() {
        check_lohi(ssx3_ee::s07::f_0027d2f0, script_engine_get_binding_process, 200, &|c, seed| {
            let mut r = Rng(0x27d2_f0 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            let index = seed % 8;
            c.bus.write32(DATA + 0x400 + 4, index);
            let binding = DATA + 0x50 + index * 0x14;
            c.bus.write32(binding + 0xc, if seed % 3 == 0 { 1 } else { r.next_u32() });
            c.bus.write32(binding + 0x8, r.next_u32());
        });
    }


    #[test]
    fn script_engine_get_script_obj_matches() {
        check(ssx3_ee::s07::f_0027d320, script_engine_get_script_obj, 200, &|c, seed| {
            let mut r = Rng(0x27d3_20 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let index = (seed % 16) as u64;
            c.g[A1][0] = index;
            c.bus.write32(DATA + index as u32 * 4, r.next_u32());
        });
    }


    #[test]
    fn script_engine_set_binding_filter_matches() {
        check_lohi(ssx3_ee::s07::f_0027d540, script_engine_set_binding_filter, 200, &|c, seed| {
            let mut r = Rng(0x27d5_40 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            let index = seed % 8;
            c.bus.write32(DATA + 0x400 + 4, index);
            let binding = DATA + index * 0x14;
            c.bus.write32(binding + 0x60, r.next_u32());
        });
    }


    #[test]
    fn script_engine_clear_binding_filter_matches() {
        check_lohi(ssx3_ee::s07::f_0027d558, script_engine_clear_binding_filter, 200, &|c, seed| {
            let mut r = Rng(0x27d5_58 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            let index = seed % 8;
            c.bus.write32(DATA + 0x400 + 4, index);
            let binding = DATA + index * 0x14;
            c.bus.write32(binding + 0x60, r.next_u32());
        });
    }


    // ------------------------------------------------------- ScriptViewObj_Update

    #[test]
    fn script_view_obj_update_matches() {
        check(ee::s07::f_00281100__generated, script_view_obj_update, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x0e50_00f6, 0);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + 8, if seed % 3 == 0 { 0 } else { 1 + seed % 5 });
            let idx = seed % 4;
            c.bus.write32(this + 0xf8, idx);
            c.bus.write32(this + 0x18, if seed % 2 == 0 { 0 } else { 1 });
            let world = DATA + 0x2000;
            let rm = DATA + 0x2100;
            let riders = DATA + 0x2200;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(world + 0x84, rm);
            c.bus.write32(rm + 0x84, riders);
            c.bus.write32(riders + 4 + 4 * idx, DATA + 0x3000);
            for i in 0..8 {
                c.bus.write32(DATA + 0x3000 + i * 4, r.next_u32());
            }
            // ScriptViewObj_BindCamSlot's short path: a negative slot at
            // +0xdc makes it bail out after scanning +0xd0/+0xd4.
            c.bus.write32(this + 0xdc, 0xffff_ffff);
            c.bus.write32(this + 0xd0, 0);
            c.bus.write32(this + 0xd4, 0);
        });
    }


    #[test]
    fn script_engine_get_instance_matches() {
        check(ee::s07::f_0027d2e8__generated, script_engine_get_instance, SEEDS, &|c, seed| {
            let mut r = rng(0x0027_d2e8, seed);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            if seed % 4 != 0 {
                c.bus.write32(0x004a_34dc, r.next_u32());
            }
        });
    }


    #[test]
    fn script_engine_v12_matches() {
        check(ee::s07::f_00279ce0__generated, script_engine_v12, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x27_9ce0, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (DATA + 0x900) as u64;
        });
    }


    #[test]
    fn script_dummy_obj_deactivate_matches() {
        check(ee::s07::f_0027c020__generated, script_dummy_obj_deactivate, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x27_c020, 0);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x14, r.next_u32());
        });
    }


    #[test]
    fn script_dummy_obj_apply_matches() {
        check(ee::s07::f_0027c050__generated, script_dummy_obj_apply, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x27_c050, 0);
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
        });
    }


    #[test]
    fn script_engine_binding_has_process_matches() {
        check_lohi(ee::s07::f_0027d2c8__generated, script_engine_binding_has_process, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x27_d2c8, 0);
            let i = r.next_u32() % 16;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = i as u64;
            let at = DATA + 0x58 + i * 0x14;
            let v = r.next_u32();
            c.bus.write32(at, v);
            c.g[A2][0] = if seed % 2 == 0 {
                v as u64
            } else {
                (v ^ (1 + r.next_u32() % 8)) as u64
            };
        });
    }


    // --------------------------------------------------------------- script

    pub(crate) fn script_list_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let engine = DATA + 0x4000;
        let list = DATA + 0x5000;
        c.bus.write32(engine + 0x54c, list);
        let index = seed % 4;
        let slot = list + index * 0xcc;
        // ScriptList_GetPlayStatus: state 3 with a non-zero "loaded" word
        // returns 3, anything else returns 0.
        c.bus.write32(slot + 0xa4, if seed % 2 == 0 { 0 } else { 3 });
        c.bus.write32(slot + 0xc4, 1);
        for i in 0..4u32 {
            c.bus.write32(slot + 0x8c + 4 * i, r.next_u32());
        }
        c.g[A0][0] = sx(engine);
        c.g[A1][0] = index as u64;
    }


    #[test]
    fn script_engine_list_get_play_status_matches() {
        check(ssx3_ee::s07::f_00279220__generated, script_engine_list_get_play_status, 100, &|c, seed| {
            script_list_setup(c, seed, 0x30)
        });
    }


    #[test]
    fn script_engine_list_count_matches() {
        check(ssx3_ee::s07::f_00279248__generated, script_engine_list_count, 100, &|c, seed| {
            script_list_setup(c, seed, 0x31)
        });
    }


    #[test]
    fn script_engine_find_script_index_matches() {
        check(ssx3_ee::s07::f_002797a0__generated, script_engine_find_script_index, 100, &|c, seed| {
            let mut r = rng_2(seed, 0x32);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA + 0x4000);
            // `ChooseScript` returns 0 for a category with no scripts, both
            // when the category table is empty and when the chosen entry is
            // null.
            let engine = DATA + 0x4000;
            c.bus.write32(engine + 0x53c, (seed % 2) as u32);
            c.bus.write32(engine + 0x540, DATA + 0x5000);
            c.g[A1][0] = (seed % 4) as u64;
        });
    }


    #[test]
    fn script_engine_get_world_section2b_matches() {
        check(ssx3_ee::s07::f_0027a688__generated, script_engine_get_world_section2b, 200, &|c, seed| {
            let mut r = rng_2(seed, 0x33);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x8000;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, DATA + 0x9000);
            c.bus.write32(DATA + 0x9000 + 0x78, DATA + 0xa000);
        });
    }


    #[test]
    fn script_view_obj_get_slot_cam_matches() {
        check(ssx3_ee::s07::f_00281c40__generated, script_view_obj_get_slot_cam, 300, &|c, seed| {
            let mut r = rng_2(seed, 0x34);
            scramble(c, &mut r);
            let obj = DATA + 0x4000;
            let slot = (seed % 8) as u32;
            let slot_value = if seed % 5 == 0 {
                u32::MAX - (seed % 8)
            } else {
                seed % 32
            };
            c.bus.write32(obj + 0xdc + slot * 4, slot_value);
            for i in 0..32u32 {
                c.bus.write32(obj + 0xd0 + 4 * i, r.next_u32());
            }
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = slot as u64;
        });
    }


    #[test]
    fn script_engine_stop_lists_matches() {
        check(
            ssx3_ee::s07::f_002789e0__generated,
            script_engine_stop_lists,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xc9, 2);
                globals(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }


    #[test]
    fn script_engine_list_set_paused_matches() {
        script_list_test(
            ssx3_ee::s07::f_00279040__generated,
            script_engine_list_set_paused,
            0xcb,
            true,
        );
    }


    #[test]
    fn script_engine_list_get_entry_type_matches() {
        script_list_test(
            ssx3_ee::s07::f_00279148__generated,
            script_engine_list_get_entry_type,
            0xcc,
            true,
        );
    }


    #[test]
    fn script_obj_cam_v02_matches() {
        script_binding_test(ssx3_ee::s07::f_0027d7f0__generated, script_obj_cam_v02, 0xcd);
    }


    #[test]
    fn script_obj_cam_v03_matches() {
        script_binding_test(ssx3_ee::s07::f_0027d820__generated, script_obj_cam_v03, 0xce);
    }


    /// A script engine at `SCRATCH` with the per-slot list array at `+0x54c`;
    /// the list's command queue is marked full (`+0xa0`) so `ScriptList_Add`
    /// returns without touching the queue, and the skip count (`a2`) is zero
    /// so `ScriptList_Skip` returns at once.
    pub(crate) fn script_engine_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 2);
        let lists = SCRATCH + 0x1000;
        let index = seed % 4;
        let list = lists + 0xcc * index;
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = index as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = (SCRATCH + 0x2000) as u64;
        c.bus.write32(SCRATCH + 0x54c, lists);
        c.bus.write32(list + 0xa0, 1); // ScriptCmdQueue_IsFull
        c.bus.write32(list + 0xc0, 0); // skip cursor
        c.bus.write32(list + 0xbc, 0);
    }


    #[test]
    fn script_engine_list_add_matches() {
        check(ee::s07::f_00278e20__generated, script_engine_list_add, 100, &|c, seed| {
            script_engine_setup(c, seed, 0x278e20)
        });
    }


    #[test]
    fn script_engine_list_skip_matches() {
        check(ee::s07::f_00279070__generated, script_engine_list_skip, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x279070, 2);
            let lists = SCRATCH + 0x1000;
            let index = seed % 4;
            let list = lists + 0xcc * index;
            c.g[A0][0] = SCRATCH as u64;
            c.g[A1][0] = index as u64;
            c.g[A2][0] = 0; // no entries to skip: Skip returns at once
            c.g[A3][0] = r.next_u32() as u64;
            c.bus.write32(SCRATCH + 0x54c, lists);
            c.bus.write32(list + 0xa0, 1);
            c.bus.write32(list + 0xc0, 0);
            c.bus.write32(list + 0xbc, 0);
        });
    }


    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_engine_stop_all_matches() {
        check(ssx3_ee::s07::f_00278a98__generated, script_engine_stop_all, SEEDS_2, &|c, seed| {
            let _r = machine(c, seed, 0x278a98, 0);
            world(c);
            let engine = DATA + 0x100;
            // Script mode 2: the list stop functions skip their exit path.
            c.bus.write32(engine + 0x550, 2);
            c.g[A0][0] = sx(engine);
        });
    }


    #[test]
    fn inseq_channel_get_block_payload_matches() {
        check(
            ee::s07::f_002742e0__generated,
            inseq_channel_get_block_payload,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2742_e0, 0);
                let chan = DATA + 0x400;
                zero(c, chan, 0x40);
                c.g[A0][0] = sx(chan);
                c.g[A1][0] = sx((seed % 4) as u32);
                c.bus.write32(chan + 0x14, DATA + 0x600);
                c.bus.write32(chan + 4, DATA + 0x700);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x600 + 4 * i, i * 0x20);
                }
                let hit = (seed % 4) as u32;
                let size = if seed % 2 == 0 {
                    (seed % 9) as u16
                } else {
                    9 + (seed % 8) as u16
                };
                c.bus.write16(DATA + 0x700 + hit * 0x20, size);
            },
        );
    }


    #[test]
    fn script_engine_clear_locator_table_matches() {
        check(ssx3_ee::s07::f_00278a58__generated, script_engine_clear_locator_table, 60, &|c, seed| {
            machine(c, seed, 0x278a_58, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            if seed % 2 == 0 {
                w(c, GAME_GP - 0x848, DATA + 0x200);
                w(c, DATA + 0x200 + 0x84, DATA + 0x300);
                w(c, DATA + 0x300 + 0x78, seed % 8);
            }
        });
    }


    #[test]
    fn script_engine_list_add_movie_matches() {
        check_lohi(ssx3_ee::s07::f_00278e90__generated, script_engine_list_add_movie, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x278e_90, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x54c, DATA + 0x1000);
            c.g[A1][0] = (seed % 3) as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = r.next_u32() as u64;
            c.g[T1][0] = r.next_u32() as u64;
            c.g[T2][0] = r.next_u32() as u64;
        });
    }


    // ------------------------------------------------------------------
    // Script engine.

    #[test]
    fn script_engine_list_add_choice_matches() {
        check_lohi(
            ee::s07::f_00278e50__generated,
            script_engine_list_add_choice,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x278e50, 0);
                let self_at = DATA + 0x200;
                let choices = DATA + 0x1000;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // The choice's other arguments: keep them null so the listener
                // does not chase scrambled pointers.
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                c.g[T3][0] = 0;
                c.bus.write32(self_at + 0x54c, choices);
            },
        );
    }


    #[test]
    fn script_engine_is_bank_slot_loaded_matches() {
        check(
            ee::s07::f_0027bf50__generated,
            script_engine_is_bank_slot_loaded,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x27bf50, 0);
                let self_at = DATA + 0x200;
                let slot = seed % 4;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = slot as u64;
                c.bus
                    .write32(self_at + 0x630 + 16 * slot, (seed % 2) as u32);
            },
        );
    }


    #[test]
    fn script_engine_exit_script_mode_matches() {
        check(
            ee::s07::f_0027bf90__generated,
            script_engine_exit_script_mode,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x27bf90, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(
                    self_at + 0x550,
                    if seed % 2 == 0 { 2 } else { (seed % 4) as u32 },
                );
            },
        );
    }


    #[test]
    fn script_engine_is_load_done_matches() {
        check(
            ssx3_ee::s07::f_00278d58__generated,
            script_engine_is_load_done,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x278d58, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A pending async load, or none (then bank slot 1 is checked).
                c.bus
                    .write32(this_ + 0x538, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }


    #[test]
    fn script_engine_is_active_list_skippable_matches() {
        check(
            ssx3_ee::s07::f_00278da0__generated,
            script_engine_is_active_list_skippable,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x278da0, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let list = DATA + 0x2000;
                c.bus
                    .write32(this_ + 0x550, if seed % 2 == 0 { 2 } else { 0 });
                c.bus.write32(this_ + 0x54c, list);
                // Not state 3: the list accessor answers without looking further.
                c.bus
                    .write32(list + 0xa4, if seed % 4 == 0 { 3 } else { 0 });
                c.bus.write32(list + 0x0c, list + 0x100);
                c.g[A0][0] = sx(this_);
            },
        );
    }


    #[test]
    fn script_engine_list_get_state_matches() {
        check(
            ssx3_ee::s07::f_002791d8__generated,
            script_engine_list_get_state,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2791d8, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let lists = DATA + 0x2000;
                c.bus.write32(this_ + 0x54c, lists);
                c.bus.write32(this_ + 0x620, 0);
                for i in 0..4u32 {
                    c.bus
                        .write32(lists + 0xcc * i + 0xa4, if i == 1 { 1 } else { 0 });
                }
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }


    #[test]
    fn script_engine_get_active_list_field_b0_matches() {
        check(
            ssx3_ee::s07::f_00279298__generated,
            script_engine_get_active_list_field_b0,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x279298, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let lists = DATA + 0x2000;
                c.bus
                    .write32(this_ + 0x550, if seed % 2 == 0 { 2 } else { 0 });
                c.bus.write32(this_ + 0x54c, lists);
                c.bus.write32(lists + 0xa4, 0);
                c.g[A0][0] = sx(this_);
            },
        );
    }


    #[test]
    fn script_engine_is_playing_script_matches() {
        check(
            ssx3_ee::s07::f_00279370__generated,
            script_engine_is_playing_script,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x279370, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                let lists = DATA + 0x2000;
                c.bus
                    .write32(this_ + 0x550, if seed % 2 == 0 { 2 } else { 0 });
                c.bus.write32(this_ + 0x54c, lists);
                c.bus.write32(lists + 0xa4, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }


    #[test]
    fn script_engine_cancel_script_bank_matches() {
        check(
            ssx3_ee::s07::f_00279840__generated,
            script_engine_cancel_script_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x279840, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                // A free slot table: the lookup returns -1 and the cancel is a
                // no-op (no file handle to release).
                for i in 0..10u32 {
                    c.bus.write32(this_ + 0x62c + 0x10 * i, 0);
                }
                c.bus.write32(this_ + 0x620, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 16) as u64;
            },
        );
    }


    #[test]
    fn script_engine_set_locator_table_matches() {
        check(
            ee::s07::f_00278a10__generated,
            script_engine_set_locator_table,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x278a_10, 1);
                let this = DATA + 0x100;
                let key = seed as u32 & 0xff;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(key);
                c.g[A2][0] = sx(r.next_u32());
                c.bus.write32(GAME_GP - 0x848, DATA + 0x1000);
                c.bus.write32(DATA + 0x1084, DATA + 0x2000);
                c.bus.write32(DATA + 0x2078, DATA + 0x3000);
                if seed % 2 == 0 {
                    // The locator table the lookup scans (0x44216c + i*0x10).
                    c.bus.write32(0x0044_216c + 0x20, key);
                }
            },
        );
    }


    pub(crate) const SEEDS_4: u32 = 150;


    /// Scrambled registers, the game's `gp`, the fake call hooks (for calls
    /// through pointer slots) and no forced unwind.
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        machine(c, seed, salt, 0)
    }


    /// A `gp`-relative global address.
    pub(crate) fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }


    /// Zero `words` words at `at`.
    pub(crate) fn clear(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }


    #[test]
    fn script_engine_stop_main_lists_matches() {
        check(
            ee::s07::f_002790f8__generated,
            script_engine_stop_main_lists,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2790_f8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x600);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn script_engine_find_free_bank_slot_matches() {
        check(
            ee::s07::f_0027b528__generated,
            script_engine_find_free_bank_slot,
            SEEDS_4,
            &|c, seed| {
                let mut r = base(c, seed, 0x27b5_28);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                // The probes are at +0x648, 0x10 bytes apart, for slots 2..9:
                // `+8` is the busy flag, `+0` the "in use" word.
                let taken = if seed % 7 == 0 { 0 } else { 2 + seed % 8 };
                for i in 2..10u32 {
                    let slot = this + 0x648 + 0x10 * (i - 2);
                    let is_target = i == taken;
                    // The slot just before the target is idle but in use, so
                    // the inner "if in use -> not free" path is taken too.
                    let idle = is_target || i + 1 == taken;
                    c.bus.write32(slot + 8, if idle { 0 } else { 1 });
                    c.bus.write32(slot, if is_target { 0 } else { 1 });
                    c.bus.write32(slot + 4, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn script_dummy_obj_ctor_matches() {
        check(
            ee::s07::f_0027bfd0__generated,
            script_dummy_obj_ctor,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x27bf_d0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x80);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn script_view_obj_accept_command_matches() {
        check(
            ee::s07::f_00281a00__generated,
            script_view_obj_accept_command,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x281a_00, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.bus.write32(this + 0xf8, r.next_u32());
                let arg = DATA + 0x300;
                clear(c, arg, 0x20);
                c.bus.write32(arg + 8, r.next_u32());
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), if seed % 2 == 0 { 0 } else { DATA + 0x400 });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(arg);
                c.g[A2][0] = [4u64, 3, 4, 5][(seed % 4) as usize];
            },
        );
    }


    #[test]
    fn script_engine_obj_table_dtor_matches() {
        check(ee::s07::f_0027ce58__generated, script_engine_obj_table_dtor, 60, &|c, seed| {
            base_2(c, seed, 0x27ce);
            let flags = if seed % 3 == 0 { 1u32 } else { 0 };
            // The delete path frees `self`: use a small (pooled) address.
            let obj = if flags == 1 { 0x100 } else { DATA };
            c.bus.write32(gp_at(0x3ec), 1);
            c.g[A0][0] = sx(obj);
            c.g[A1][0] = sx(flags);
        });
    }


    #[test]
    fn script_engine_stop_aux_lists_matches() {
        check(
            ee::s07::f_00279488__generated,
            script_engine_stop_aux_lists,
            SEEDS_5,
            &|c, seed| {
                let _ = common(c, seed, 0x279488, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                let lists = DATA + 0x2000;
                clear(c, lists, 0x200);
                c.bus.write32(this + 0x624, lists);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }


    #[test]
    fn script_view_obj_add_cam_matches() {
        check(
            ee::s07::f_00281190__generated,
            script_view_obj_add_cam,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x281190, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let cam = DATA + 0x2000;
                clear(c, cam, 0x40);
                let p0 = if seed % 4 == 0 { 0 } else { r.next_u32() };
                let p1 = if seed % 5 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(this + 0xd0, p0);
                c.bus.write32(this + 0xd4, p1);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(cam);
            },
        );
    }


    #[test]
    fn script_engine_script_db_ctor_matches() {
        check(
            ee::s07::f_00281e80__generated,
            script_engine_script_db_ctor,
            SEEDS_5,
            &|c, seed| {
                let _ = common(c, seed, 0x281e80, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// The main heap's struct and a 1 MiB arena (below the test data area and
    /// clear of the fake-call log at 0x900000).
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) const ARENA: u32 = 0x0010_0000;

    pub(crate) const ARENA_SIZE: u32 = 0x0010_0000;


    /// Runs a generated function to completion as a setup step.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }


    /// A fresh dlmalloc arena for the main heap, so allocations and frees in
    /// the callees are real (and identical on both machines).
    pub(crate) fn heap_init(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, ARENA as u64, ARENA_SIZE as u64],
        );
    }


    pub(crate) fn alloc(c: &mut Cpu, size: u64) -> u32 {
        run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size],
        ) as u32
    }


    // ------------------------------------------------------------- script

    #[test]
    fn script_engine_on_seq_end_matches() {
        check(
            ssx3_ee::s07::f_0027a798__generated,
            script_engine_on_seq_end,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x27a798, 2);
                let engine = DATA;
                let list = DATA + 0x1000;
                c.bus.write32(engine + 0x54c, list);
                c.bus.write32(
                    engine + 0x550,
                    match seed % 3 {
                        0 => 2,
                        1 => 0,
                        _ => 1,
                    },
                );
                let entry = list + 0xcc;
                c.bus.write32(entry + 0xc, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x1c, seed);
                c.g[A0][0] = engine as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }


    #[test]
    fn rider_script_obj_matches_rider_slot_matches() {
        check(
            ssx3_ee::s07::f_00280560__generated,
            rider_script_obj_matches_rider_slot,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0x280560, 0);
                let rider = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = rider as u64;
                // 0 (slot 0), a negative id, too large an id, or 1..21 (the
                // target-object lookup, which needs the script object table).
                c.g[A2][0] = match seed % 5 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    2 => 22,
                    3 => 0x7fff_ffff,
                    _ => 0x8000_0000,
                };
                c.g[A3][0] = 0;
            },
        );
    }


    // -------------------------------------------------------------- audio

    #[test]
    fn script_engine_script_db_dtor_matches() {
        check(
            ssx3_ee::s07::f_00281ed8__generated,
            script_engine_script_db_dtor,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x281ed8, 1);
                heap_init(c);
                let this = alloc(c, 0x300);
                // No live database: `DestroyScriptDatabase` returns at once, and
                // the object table below the engine is empty.
                c.bus.write32(this + 0x2b0, 0);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }


    #[test]
    fn script_dummy_obj_accept_command_matches() {
        check(
            ee::s07::f_0027c040__generated,
            script_dummy_obj_accept_command,
            SEEDS_6,
            &|c, seed| {
                let mut r = rng_3(0x0027_c040, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    6
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }


    #[test]
    fn script_audio_obj_accept_command_matches() {
        check(
            ee::s07::f_002808d8__generated,
            script_audio_obj_accept_command,
            SEEDS_6,
            &|c, seed| {
                let mut r = rng_3(0x0028_08d8, seed);
                scramble(c, &mut r);
                c.g[A2][0] = if seed % 2 == 0 {
                    7
                } else {
                    (r.next_u32() | 0x1000) as u64
                };
            },
        );
    }


    #[test]
    fn script_view_obj_get_binding_matches() {
        check_lohi(
            ee::s07::f_00281170__generated,
            script_view_obj_get_binding,
            SEEDS_6,
            &|c, seed| {
                let mut r = rng_3(0x0028_1170, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { (seed % 6) as u64 };
            },
        );
    }


    #[test]
    fn script_obj_cam_is_bound_to_process_matches() {
        check(ee::s07::f_0027d790__generated, script_obj_cam_is_bound_to_process, SEEDS_7, &|c, seed| {
            machine(c, seed, 0x0027_d790, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 5) as u64;
            let bound = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
            w(c, DATA + 0x14, bound);
            w(c, DATA + 0x104, (seed % 5) as u32);
        });
    }


    #[test]
    fn find_bank_slot_matches() {
        check(ee::s07::f_0027b578__generated, script_engine_find_bank_slot_by_id, SEEDS_8, &|c, seed| {
            let mut r = base_3(c, seed, 0x27b5_78);
            let base_3 = DATA + 0x100;
            clear(c, base_3, 0x300);
            let table = base_3 + 0x62c;
            let key = r.next_u32();
            let target = if seed % 7 == 0 { -1i32 } else { (seed % 10) as i32 };
            for i in 0..10u32 {
                let s = table + 0x10 * i;
                w_2(c, s + 4, if (seed + i) % 3 == 0 { 1 } else { 0 });
                w_2(c, s, if i as i32 == target { key } else { r.next_u32() });
            }
            c.g[A0][0] = sx(base_3);
            c.g[A1][0] = key as u64;
            c.g[A2][0] = if seed % 5 == 0 { 0 } else { (seed % 12) as u64 };
        });
    }


    #[test]
    fn script_engine_list_add_movie_by_id_matches() {
        check(
            ee::s07::f_00278ed0__generated,
            script_engine_list_add_movie_by_id,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_8ed0, 1);
                be_library(c);
                world_2(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x52c, DATA + 0x1000);
                c.bus.write32(DATA + 0x530, DATA + 0x1100);
                c.bus.write32(DATA + 0x1000, 3);
                for i in 0..3u32 {
                    let e = DATA + 0x1100 + 0xc * i;
                    c.bus.write32(e + 4, 0x100 + i);
                }
                let key = match seed % 4 {
                    0 => 0x100,
                    1 => 0x102,
                    2 => 0x999,
                    _ => (0x100 + r.next_u32() % 3) & 0x1ff,
                };
                c.g[A2][0] = key as u64;
                c.bus.write32(DATA + 0x54c, DATA + 0x2000);
            },
        );
    }


    #[test]
    fn rider_script_obj_deactivate_matches() {
        check(
            ee::s07::f_002803f0__generated,
            rider_script_obj_deactivate,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2803f0, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                w_3(c, OBJ + 0xc, if seed % 2 == 0 { 0 } else { 1 });
                // The identity-transform block the copy reads.
                for k in 0..4u32 {
                    w_3(c, 0x004f_f1a0 + k * 0x10, 0x3f80_0000);
                }
            },
        );
    }


    #[test]
    fn script_engine_cancel_bank_load_matches() {
        check(
            ee::s07::f_0027b4b8__generated,
            script_engine_cancel_bank_load,
            6,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x0027_b4b8, 4);
                c.g[A0][0] = OBJ_2 as u64;
                let idx = (r.next_u32() % 4) as u64;
                c.g[A1][0] = idx;
                let slot = OBJ_2 + idx as u32 * 16 + 0x628;
                w(c, slot + 8, if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
                w(c, OBJ_2 + 0x620, seed % 3);
                w(c, DATA + 0x4000, 0);
            },
        );
    }


    #[test]
    fn script_engine_on_seq_start_matches() {
        check(ssx3_ee::s07::f_0027a6b0__generated, script_engine_on_seq_start, SEEDS_9, &|c, seed| {
            let mut r = machine(c, seed, 0x27a6b0, 0);
            let e = DATA + 0x1000;
            let listener = DATA + 0x2000;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = listener as u64;
            let r_idx = if seed % 2 == 0 { 2 } else { (seed % 3) as u32 };
            w32(c, e + 0x550, r_idx);
            let table = DATA + 0x3000;
            w32(c, e + 0x54c, table);
            let list = table + r_idx * 0xcc;
            w32(c, list + 0xa4, 0); // ScriptList_IsPlayingScript -> false
            let o = DATA + 0x4000;
            w32(c, listener + 0xc, o);
            w32(c, o + 0x1c, r.next_u32());
            w32(c, o + 0x10, 0);
            let vt = DATA + 0x5000;
            w32(c, e + 0x2a8, vt);
            slot(c, vt, 0x50, 0, EMPTY);
        });
    }


    #[test]
    fn script_engine_on_seq_stopped_matches() {
        check(ssx3_ee::s07::f_0027a7f0__generated, script_engine_on_seq_stopped, SEEDS_9, &|c, seed| {
            let mut r = machine(c, seed, 0x27a7f0, 0);
            let e = DATA + 0x1000;
            let listener = DATA + 0x2000;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = listener as u64;
            let r_idx = if seed % 2 == 0 { 2 } else { (seed % 3) as u32 };
            w32(c, e + 0x550, r_idx);
            let table = DATA + 0x3000;
            w32(c, e + 0x54c, table);
            w32(c, table + r_idx * 0xcc + 0xa4, 0);
            let o = DATA + 0x4000;
            w32(c, listener + 0xc, o);
            w32(c, o + 0x10, 0);
            w32(c, o + 0x1c, r.next_u32());
            let vt = DATA + 0x5000;
            w32(c, e + 0x2a8, vt);
            slot(c, vt, 0x58, 0, EMPTY);
        });
    }


    #[test]
    fn script_engine_remove_binding_matches() {
        check(ssx3_ee::s07::f_0027d390__generated, script_engine_remove_binding, SEEDS_9, &|c, seed| {
            let mut r = machine(c, seed, 0x27d390, 0);
            let e = DATA + 0x1000;
            let msg = DATA + 0x2000;
            c.g[A0][0] = e as u64;
            c.g[A1][0] = msg as u64;
            let idx = (seed % 4) as u32;
            w32(c, msg + 4, idx);
            let s0 = e + idx * 0x14 + 0x50;
            w32(c, s0 + 0xc, if seed % 3 == 0 { 1 } else { r.next_u32() });
            w32(c, s0, if seed % 2 == 0 { DATA + 0x3000 } else { r.next_u32() });
            w32(c, msg + 8, 0); // ClearListener returns at once
            w32(c, DATA + 0x3010, r.next_u32());
        });
    }


    #[test]
    fn script_engine_collect_free_bindings_matches() {
        check(
            ssx3_ee::s07::f_0027d578__generated,
            script_engine_collect_free_bindings,
            SEEDS_9,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27d578, 0);
                let vars = DATA + 0x1000;
                let out = DATA + 0x4000;
                let end = DATA + 0x5000;
                c.g[A0][0] = vars as u64;
                c.g[A1][0] = end as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = (1 + seed % 4) as u64;
                for i in 0..30u32 {
                    let v = vars + 0x50 + i * 0x14;
                    w32(c, v, if (seed + i) % 5 == 0 { 0 } else { r.next_u32() });
                    w32(c, v + 0xc, if (seed + i) % 4 == 0 { 1 } else { 2 });
                    let link = match (seed + i) % 6 {
                        0 => 0x8000_0000u32,
                        1 => end,
                        2 => end + 0x14,
                        _ => r.next_u32() & 0x00ff_ffff,
                    };
                    w32(c, v + 0x10, link);
                }
            },
        );
    }


    pub(crate) const SEEDS_10: u32 = 20;


    /// The BELibrary singleton (`gp - 0x1f04`) pointed at a zeroed dummy.
    pub(crate) fn belib_2(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x8000);
    }


    /// The allocator's main heap header (`D_004FF250`) over 256 KiB of test
    /// RAM, as `DLHeap_InitArena` leaves it (the arena itself stays zeroed).
    pub(crate) fn heap_setup(c: &mut Cpu) {
        const HEAP: u32 = 0x004f_f250;
        let base = DATA + 0x40_0000;
        let start = base.wrapping_add(0xf) & !0xf;
        let rest = 0x4_0000u32.wrapping_sub(start - base) & !0xf;
        w(c, HEAP + 0xc, rest);
        w(c, HEAP + 4, start);
        w(c, HEAP, start);
        w(c, HEAP + 4, start + 0x380);
        w(c, HEAP + 0xc, rest - 0x380);
        w(c, HEAP + 8, start + 0x380);
    }


    // ------------------------------------------------------------ script engine

    #[test]
    fn script_engine_load_script_bank_matches() {
        check(ee::s07::f_002797c8__generated, script_engine_load_script_bank, 12, &|c, seed| {
            machine(c, seed, 0x27_97c8, 4);
            belib_2(c);
            heap_setup(c);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (seed % 32) as u64;
            // All bank slots busy: the finder returns -1 and the async load
            // takes its short path.
            for i in 0..8u32 {
                w(c, DATA + 0x2000 + 0x648 + 0x10 * i + 8, 1);
            }
            c.pc = 0x0027_97c8;
        });
    }


    #[test]
    fn script_engine_is_finish_script_active_matches() {
        check(ee::s07::f_0027aa80__generated, script_engine_is_finish_script_active, SEEDS_10, &|c, seed| {
            machine(c, seed, 0x27_aa80, 1);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            w(c, DATA + 0x2000 + 0x550, (seed % 2) as u32);
            w(c, DATA + 0x2000 + 0x54c, DATA + 0x2400);
            // The entry type the list query returns.
            w(c, DATA + 0x2400 + 0xcc, (seed % 20) as u32);
            c.pc = 0x0027_aa80;
        });
    }


    #[test]
    fn script_view_read_fields_matches() {
        check(ee::s07::f_0027c8e0__generated, script_view_read_fields, SEEDS_10, &|c, seed| {
            machine(c, seed, 0x27_c8e0, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2000 + 0x24, DATA + 0x2400);
            w(c, DATA + 0x2400 + 4, DATA + 0x2600);
            let (d1, f1) = (0x20u32, 0x24u32);
            let (d2, f2) = (0x28u32, 0x2c);
            w(c, DATA + 0x2600 + d1, 0);
            w(c, DATA + 0x2600 + f1, FAKE);
            w(c, DATA + 0x2600 + d2, 0);
            w(c, DATA + 0x2600 + f2, FAKE);
            c.pc = 0x0027_c8e0;
        });
    }


    #[test]
    fn script_dummy_obj_read_fields_matches() {
        check(ee::s07::f_0027cbb0__generated, script_dummy_obj_read_fields, SEEDS_10, &|c, seed| {
            machine(c, seed, 0x27_cbb0, 3);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.g[A1][0] = (DATA + 0x2200) as u64;
            w(c, DATA + 0x2000 + 0x24, DATA + 0x2400);
            w(c, DATA + 0x2400 + 4, DATA + 0x2600);
            w(c, DATA + 0x2600 + 0x20, 0);
            w(c, DATA + 0x2600 + 0x24, FAKE);
            w(c, DATA + 0x2600 + 0x28, 0);
            w(c, DATA + 0x2600 + 0x2c, FAKE);
            c.pc = 0x0027_cbb0;
        });
    }


    #[test]
    fn script_view_obj_reset_matches() {
        check(ee::s07::f_00281a50__generated, script_view_obj_reset, SEEDS_10, &|c, seed| {
            machine(c, seed, 0x28_1a50, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = 0x0028_1a50;
        });
    }


    #[test]
    fn script_engine_on_seq_advance_matches() {
        check(ee::s07::f_0027a720__generated, script_engine_on_seq_advance, SEEDS_11, &|c, seed| {
            machine(c, seed, 0x27a720, 0);
            be_library_2(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, obj + 0x550, if seed % 3 == 0 { 2 } else { 1 });
            w(c, obj + 0x54c, DATA + 0x1000);
        });
    }


    #[test]
    fn script_engine_unload_world_scripts_matches() {
        check(
            ee::s07::f_00278cd8__generated,
            script_engine_unload_world_scripts,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x278c_d8, 6);
                let self_ = DATA;
                w(
                    c,
                    self_ + 0x538,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                let blk = if seed % 3 == 0 { 0 } else { alloc_2(c, 0x40) };
                w(c, self_ + 0x53c, blk);
                w(c, self_ + 0x620, 1 + seed % 4 as u32);
                match seed % 4 {
                    0 => w(c, self_ + 0x638, 0),
                    _ => w(c, self_ + 0x638, 0),
                }
                w(c, self_ + 0x63c, 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }


    #[test]
    fn script_engine_play_script_instance_matches() {
        check(
            ee::s07::f_00279720__generated,
            script_engine_play_script_instance,
            12,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x2797_20, 4);
                let self_ = DATA;
                let obj = DATA + 0x1000;
                w(c, self_ + 0x2c0 + 0x20, (seed % 2) as u32);
                w(c, self_ + 0x2c0 + 0x24, 0);
                w(c, self_ + 0x2c0 + 0x4, (seed % 4) as u32);
                w(c, obj, 0);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (r.next_u32() % 8) as u64;
                c.g[A2][0] = (r.next_u32() % 8) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }


    #[test]
    fn script_audio_obj_activate_matches() {
        check(
            ee::s07::f_00280730__generated,
            script_audio_obj_activate,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2807_30, 6);
                let self_ = DATA;
                w(
                    c,
                    self_ + 0x8,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w(c, DATA + 0x1000 + 0x8, 0);
                w(c, DATA + 0x1000 + 0x1c, 0);
                w(c, GAME_GP.wrapping_sub(0x84c), 0);
                c.g[A0][0] = self_ as u64;
            },
        );
    }


    // ------------------------------------------------------------------
    // Script tables and objects.

    #[test]
    fn script_engine_obj_table_ctor_matches() {
        check(ssx3_ee::s07::f_0027cdd0__generated, script_engine_obj_table_ctor, SEEDS_12, &|c, seed| {
            let mut r = machine(c, seed, 0x27cdd0, 0);
            be_library_3(c);
            world_3(c);
            c.g[A0][0] = (DATA + 0x100) as u64;
            let _ = r.next_u32();
        });
    }


    #[test]
    fn script_obj_cam_activate_matches() {
        check(ssx3_ee::s07::f_0027d850__generated, script_obj_cam_activate, SEEDS_12, &|c, seed| {
            let mut r = machine(c, seed, 0x27d850, 6);
            be_library_3(c);
            world_3(c);
            let cam = DATA + 0x100;
            c.g[A0][0] = cam as u64;
            if seed % 4 == 0 {
                // No world_3: the short path.
                w(c, GAME_GP.wrapping_sub(0x848), 0);
            } else {
                world_3(c);
                let wld = DATA + 0x1000;
                let race = DATA + 0x1200;
                w(c, wld + 0x84, race);
                let arr = DATA + 0x1600;
                w(c, race + 0x84, arr);
                w(c, cam + 0x7c, (r.next_u32() % 4) as u32);
                let slot_obj = DATA + 0x1700;
                let slot_index = c.bus.read32(cam + 0x7c);
                w(c, arr + 4 * slot_index, slot_obj);
                w(c, slot_obj + 4, 0);
                w(c, slot_obj + 0xac, DATA + 0x1800);
                w(c, cam + 0x14, r.next_u32() & 0x7);
                w(c, cam + 0x18, 0);
                // The controller's vtable slot at +0x18.
                let ctl = DATA + 0x1800;
                w(c, ctl + 0x18, if seed % 3 == 0 { 0 } else { DATA + 0x1900 });
                let _ = ctl;
            }
        });
    }


    #[test]
    fn script_audio_obj_ctor_matches() {
        check(ssx3_ee::s07::f_002805b8__generated, script_audio_obj_ctor, SEEDS_12, &|c, seed| {
            let mut r = machine(c, seed, 0x2805b8, 0);
            be_library_3(c);
            world_3(c);
            c.g[A0][0] = (DATA + 0x100) as u64;
            let _ = r.next_u32();
        });
    }


    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc) and the world (`gp - 0x848`, with no riders).
    pub(crate) fn globals_4(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }


            #[test]
    fn rider_script_obj_ctor_matches() {
        check(ee::s07::f_0027f968__generated, rider_script_obj_ctor, 100, &|c, seed| {
            let _ = base(c, seed, 0x27_f968);
            globals_4(c);
            // The identity matrix at the global the original copies.
            for i in 0..4u32 {
                w(c, 0x004f_f1a0 + 0x10 * i, 0x3f80_0000);
            }
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn script_view_obj_set_transition_matches() {
        check(ee::s07::f_002812e0__generated, script_view_obj_set_transition, 150, &|c, seed| {
            let _ = base(c, seed, 0x28_12e0);
            globals_4(c);
            const TRANS: u32 = DATA + 0x4000;
            w(c, TRANS + 8, [0, 1, 2][seed as usize % 3]);
            w(c, TRANS + 0x10, 0x1111);
            match seed % 4 {
                0 => {
                    // No new object: the running transition is stopped.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = 0;
                    w(c, DATA + 0xd8, TRANS);
                }
                1 => {
                    // No new object and no transition.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = 0;
                    w(c, DATA + 0xd8, TRANS);
                }
                2 => {
                    // Install a new transition.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = sx(DATA + 0x5000);
                    w(c, DATA + 0xd8, 0);
                }
                _ => {
                    // A transition is already running: refuse.
                    c.g[A0][0] = sx(DATA);
                    c.g[A1][0] = sx(DATA + 0x5000);
                    w(c, DATA + 0xd8, TRANS);
                }
            }
        });
    }


        #[test]
    fn script_engine_list_start_matches() {
        check(ee::s07::f_00278f68__generated, script_engine_list_start, 200, &|c, seed| {
            let mut r = base(c, seed, 0x27_8f68);
            globals_4(c);
            // A list table at +0x54c; the list state at +0x550.
            w(c, DATA + 0x54c, DATA + 0x3000);
            for i in 0..4u32 {
                w(c, DATA + 0x3000 + 0xcc * i, 0);
            }
            w(c, DATA + 0x550, [0, 1, 2, 3][seed as usize % 4]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = [0u64, 1, 2][seed as usize % 3];
            c.g[A2][0] = [0u64, 1][seed as usize % 2];
            c.g[A3][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn script_engine_on_data_removed_matches() {
        check(
            ee::s07::f_002799e0__generated,
            script_engine_on_data_removed,
            SEEDS_13,
            &|c, seed| {
                machine(c, seed, 0x2799, 0);
                env(c);
                let this = DATA + 0x400;
                let obj = DATA + 0x800;
                let kind = (seed % 4) as u64;
                w32(c, obj + 0x18, 0x11);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(obj);
                c.g[A2][0] = 0;
                c.g[A3][0] = kind;
                c.g[T0][0] = 5;
            },
        );
    }


    #[test]
    fn script_obj_cam_ctor_matches() {
        check(ee::s07::f_0027d5e8__generated, script_obj_cam_ctor, 40, &|c, seed| {
            let mut r = base_6(c, seed, 0x27d5e8, 2);
            let obj = DATA + 0x1000;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            for i in 0..4u32 {
                w(c, 0x004f_f1a0 + 0x10 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn script_obj_cam_deactivate_matches() {
        check(
            ee::s07::f_0027d8d8__generated,
            script_obj_cam_deactivate,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x27d8d8, 3);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8_0000;
                let game = world + 0x1000;
                let riders = world + 0x3000;
                let rider = DATA + 0x5000;
                let scr = DATA + 0x6000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x7c, 0);
                w(c, game + 0x84, if seed % 4 == 0 { 0 } else { riders });
                // riders[1] is the rider `idx = 0` selects.
                w(c, riders + 4, rider);
                w(c, rider + 0xac, DATA + 0x7000);
                let sc = scrub(c, scr);
                w(c, rider + 0xa0, sc);
                w(c, DATA + 0x7000 + 0x18, if seed % 2 == 0 { 0 } else { 1 });
                let _ = r.next_u32();
            },
        );
    }


    /// A zeroed script-object dummy with the vtable slot_2 4 used by
    /// `ScriptObjCam_Deactivate`.
    pub(crate) fn scrub(c: &mut Cpu, at_2: u32) -> u32 {
        let vt = at_2 + 0x1000;
        w(c, at_2 + 0x14, vt);
        slot_2(c, vt, 0x20, 0, FAKE);
        at_2
    }


    // ----------------------------------------------------------------- roster

        // ------------------------------------------------------------ crowd audio

        // -------------------------------------------------------- script engine

    #[test]
    fn script_engine_load_bank_async_matches() {
        check(ssx3_ee::s07::f_0027b370__generated, script_engine_load_bank_async, 12, &|c, seed| {
            let mut r = base_7(c, seed, 0x0027_b370);
            let engine = DATA + 0x100;
            let bank = (seed % 4) as u32;
            let slot = engine + 0x628 + 16 * bank;
            clear_2(c, engine, 0x700);
            put_str(c, DATA + 0x3000, "bank.bin");
            w(c, slot + 4, r.next_u32());
            w(c, slot + 8, if seed % 3 == 0 { 1 } else { 0 });
            w(c, slot + 0xc, if seed % 3 == 0 { 0 } else { 1 });
            w(c, engine + 0x620, r.next_u32());
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = bank as u64;
            c.g[A2][0] = sx(DATA + 0x3000);
            c.g[A3][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x3100) };
            c.g[T0][0] = (seed % 0x1000) as u64;
            c.g[T1][0] = if seed % 4 == 0 { sx(DATA + 0x3200) } else { 0 };
            c.g[T2][0] = 0;
        });
    }


    #[test]
    fn script_engine_clear_obj_binding_filter_matches() {
        check(
            ssx3_ee::s07::f_0027d170__generated,
            script_engine_clear_obj_binding_filter,
            12,
            &|c, seed| {
                let _ = base_7(c, seed, 0x0027_d170);
                let engine = DATA + 0x100;
                let list = DATA + 0x2000;
                let vtable = DATA + 0x3000;
                let filter = (seed % 4) as u32;
                clear_2(c, engine, 0x1000);
                clear_2(c, list, 0x80);
                clear_2(c, vtable, 0x80);
                w(c, engine + 4 * filter, list);
                w(c, list + 0xc, vtable);
                c.bus.write16(vtable + 0x30, 0);
                w(c, vtable + 0x34, crate::difftest::fakes::FAKE);
                c.bus.write16(vtable + 0x38, 0);
                w(c, vtable + 0x3c, crate::difftest::fakes::FAKE + 0x10);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                w(c, crate::difftest::fakes::LOG_RESULTS + 4, 3);
                c.g[A0][0] = sx(engine);
                c.g[A1][0] = filter as u64;
                c.g[A2][0] = 0;
            },
        );
    }


    #[test]
    fn script_engine_activate_binding_matches() {
        check(ssx3_ee::s07::f_0027d400__generated, script_engine_activate_binding, 12, &|c, seed| {
            let _ = base_7(c, seed, 0x0027_d400);
            let engine = DATA + 0x100;
            let def = DATA + 0x2000;
            let slot = DATA + 0x3000;
            clear_2(c, engine, 0x400);
            clear_2(c, def, 0x80);
            clear_2(c, slot, 0x80);
            w(c, engine + 0x2a8, def);
            c.bus.write16(def + 0x20, 0);
            if seed % 2 == 0 {
                w(c, def + 0x24, crate::difftest::fakes::FAKE);
                w(c, crate::difftest::fakes::LOG_RESULTS, 1);
            } else {
                // Run the second path: the binding's start returns 0.
                w(c, def + 0x24, 0x002c_c098);
            }
            w(c, engine + 0x54, 0);
            if seed % 2 != 0 {
                // The binding start returns 0 (`MenuSys_IsActive` on a zeroed
                // first word), so the value table path runs and reads slot 0.
                w(c, engine, 0);
            } else {
                w(c, engine, slot);
            }
            c.bus.write16(slot + 0x20, 0);
            w(c, slot + 0x24, 0);
            c.g[A0][0] = sx(engine);
            c.g[A1][0] = sx(DATA + 0x3100);
            c.g[A2][0] = (seed % 5) as u64;
        });
    }


        #[test]
    fn script_engine_on_filter_loaded_matches() {
        check(ssx3_ee::s07::f_0027b6b0__generated, script_engine_on_filter_loaded, 20, &|c, seed| {
            let _r = base_8(c, seed, 0x27b6b0, 3);
            let this = DATA + 0x1000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0;
            w_4(c, this + 0x620, 2);
            let header = DATA + 0x3000;
            w_4(c, STACK - 0x30, if seed % 2 == 0 { 0 } else { header });
            w_4(c, STACK - 0x2c, if seed % 2 == 0 { 0 } else { 8 });
            let count = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => 2,
            };
            w_4(c, header, count);
            w_4(c, header + 4, 1);
            w_4(c, header + 8, 0x20);
            w_4(c, header + 0xc, 0);
            w_4(c, header + 0x10, 0x40);
        });
    }


    #[test]
    fn script_engine_deactivate_binding_matches() {
        check_lohi(ssx3_ee::s07::f_0027d4a0__generated, script_engine_deactivate_binding, 30, &|c, seed| {
            let _r = base_8(c, seed, 0x27d4a0, 2);
            let this = DATA + 0x1000;
            let key = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = key as u64;
            w_4(c, key + 4, (seed % 4) as u32);
            let slot = this + 0x50 + 0x14 * (seed % 4);
            w_4(c, slot + 4, 7);
            w_4(c, slot + 8, 0x55);
            w_4(c, slot + 0xc, 0);
            let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
            w_4(c, this + 0x2000, 0);
            w_4(c, this + 0x2a8, DATA + 0x3000);
            w_4(c, DATA + 0x3000, DATA + 0x3100);
            c.bus.write16(DATA + 0x3100 + 0x20, 0x2000);
            w_4(c, DATA + 0x3100 + 0x24, if seed % 2 == 0 { zero_fn } else { 0 });
            if seed % 3 == 0 {
                w_4(c, this, 0);
            } else {
                w_4(c, this, DATA + 0x4000);
                w_4(c, DATA + 0x4000 + 0xc, DATA + 0x4100);
                c.bus.write16(DATA + 0x4100 + 0x28, 0);
                w_4(c, DATA + 0x4100 + 0x2c, 0);
            }
        });
    }


        #[test]
    fn script_engine_unload_bank_matches() {
        check(
            ee::s07::f_0027b410__generated,
            script_engine_unload_bank,
            8,
            &|c, seed| {
                let _ = base_9(c, seed, 0x27b410, 3);
                c.g[A0][0] = OBJ_3 as u64;
                let idx = (seed % 3) as u32;
                c.g[A1][0] = idx as u64;
                let slot = OBJ_3 + idx * 16 + 0x628;
                match seed % 3 {
                    0 => {
                        w(c, slot, 0);
                        w(c, slot + 8, DST);
                    }
                    1 => {
                        w(c, slot, 0);
                        w(c, slot + 8, 0);
                    }
                    _ => {
                        w(c, slot, 0x77);
                        w(c, slot + 0xc, 1);
                    }
                }
                w(c, OBJ_3 + 0x620, 1);
                w(c, OBJ_3 + 0x51c, 0);
            },
        );
    }


    // ---------------------------------------------------------- script engine

    #[test]
    fn script_engine_spawn_chosen_script_matches() {
        check(
            ee::s07::f_00279528__generated,
            script_engine_spawn_chosen_script,
            FEW,
            &|c, seed| {
                let _ = base_10(c, seed, 0x2795_28, 8);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A2][0] = seed as u64;
                c.g[A3][0] = (seed ^ 0x55) as u64;
                c.g[T0][0] = (seed.wrapping_mul(3)) as u64;
                // A script node chosen for some seeds.
                w(c, obj + 0x100, if seed % 3 == 0 { 0 } else { obj + 0x1000 });
                w(c, obj + 0x1000, 0x100);
            },
        );
    }


    // ------------------------------------------------------ lobby list adds

                            // ------------------------------------------------------ binding filter

    #[test]
    fn script_engine_set_obj_binding_filter_matches() {
        check(
            ee::s07::f_0027d0c0__generated,
            script_engine_set_obj_binding_filter,
            SEEDS_14,
            &|c, seed| {
                let _ = base_10(c, seed, 0x27d0_c0, 8);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed ^ 0x3c) as u64;
                let so = obj + 0x1000;
                w(c, obj + 4 * (seed % 3), so);
                w(c, so + 0xc, obj + 0x1100);
                slot_to(
                    c,
                    obj + 0x1100,
                    0x30,
                    if seed % 2 == 0 { RET_ZERO } else { RET_ONE },
                );
                slot_to(c, obj + 0x1100, 0x38, RET_ONE);
                slot_3(c, obj + 0x1100, 0x3c, 0);
            },
        );
    }


    #[test]
    fn script_engine_v06_matches() {
        check(
            ee::s07::f_00278ad0__generated,
            script_engine_v06,
            SEEDS_15,
            &|c, seed| {
                let mut r = machine(c, seed, 0x278a, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let slot = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0x1000 + (seed % 4) as u64;
                c.bus.write32(this + 0x2a8, slot);
                c.bus.write32(this + 0x6d0, 1);
                c.bus.write32(this + 0x7f0, 2);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(slot + 0x24, vt.wrapping_add(4));
            },
        );
    }


    #[test]
    fn script_engine_remove_script_obj_matches() {
        check(
            ee::s07::f_0027cf88__generated,
            script_engine_remove_script_obj,
            SEEDS_15,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27cf, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let obj = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                if seed % 3 != 0 {
                    c.bus.write32(this + 4 * (seed % 4), obj);
                    let vt = vtable(c, &mut r, &[0x30, 0x38]);
                    c.bus.write32(obj + 0xc, vt);
                }
            },
        );
    }


    #[test]
    fn rider_script_obj_accept_command_matches() {
        check(
            ee::s07::f_00280458__generated,
            rider_script_obj_accept_command,
            SEEDS_15,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2804, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let msg = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = msg as u64;
                c.g[A2][0] = if seed % 3 == 0 { 5 } else { (seed % 9) as u64 };
                let obj = DATA + 0x2400;
                c.bus.write32(this, obj);
                c.bus.write32(obj + 0x30, 0);
                c.bus.write32(obj + 0x34, FAKE);
                c.bus.write32(msg + 8, seed % 5);
                let _ = &mut r;
            },
        );
    }


    #[test]
    fn script_engine_on_bank_loaded_matches() {
        check(
            ee::s07::f_0027b5d8__generated,
            script_engine_on_bank_loaded,
            16,
            &|c, seed| {
                let _ = base_11(c, seed, 0x27b5d8, 2);
                let this = DATA;
                clear_3(c, this, 0x1000);
                c.g[A0][0] = this as u64;
                let bank = (seed % 5) as u64;
                c.g[A1][0] = bank;
                w(c, this + 0x620, 1);
                for i in 0..10u32 {
                    let slot = this + 0x628 + 0x10 * i;
                    w(c, slot, (i + 1) as u32);
                    w(c, slot + 4, i);
                    w(
                        c,
                        slot + 8,
                        if seed % 2 == 0 && i == (seed % 10) {
                            bank as u32
                        } else {
                            0x99
                        },
                    );
                }
                w(c, SP - 0xf0 + 0x84, 0);
            },
        );
    }


        #[test]
    fn script_engine_add_script_obj_matches() {
        diff(ee::s07::f_0027cea8__generated, script_engine_add_script_obj, 40, &|c, seed| {
            let _r = fe_base(c, seed, 0x27_cea8);
            let obj = DATA + 0x2000;
            let vt = DATA + 0x3000;
            // The object's "binding count" virtual returns null: MenuSys_IsActive
            // on a zeroed first word.
            w(c, vt + 0x30, 0);
            w(c, vt + 0x34, 0x002c_c098);
            w(c, obj, 0);
            w(c, obj + 0xc, vt);
            for i in 0..20 {
                w(c, DATA + 4 * i, if i == 0 { 0 } else { 0 });
            }
            if seed % 2 == 0 {
                w(c, DATA, 0);
            } else {
                w(c, DATA, DATA + 0x5000);
            }
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(obj);
        });
    }


    // --------------------------------------------------------- script engine

    #[test]
    fn script_engine_spawn_script_matches() {
        strict(
            ee::s07::f_002795d0__generated,
            script_engine_spawn_script,
            90,
            &|c, seed| {
                let mut r = base_12(c, seed, 0x2795, 0);
                w(c, W + 0x2000 + 0x78, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 1;
                c.g[A2][0] = ((seed * 7) % 7) as u64;
                c.g[A3][0] = ((seed * 11) % 7) as u64;
                c.g[T0][0] = ((seed * 13) % 7) as u64;
                // Script process table: a non-empty entry for the given key
                // makes StartScript return 2 (already running).
                let table = DATA + 0x3000;
                w(c, DATA + 0x2b0, table);
                w(c, table, if seed % 2 == 0 { 1 } else { 0 });
                // AllocProcess: a free slot on some seeds, full otherwise.
                for i in 0..16u32 {
                    w(
                        c,
                        DATA + 0x2e0 + 0x24 * i,
                        if seed % 3 == 0 { 0 } else { (i + 1) as u32 },
                    );
                }
                // The free-list node.
                w(c, DATA + 0x548, DATA + 0x4000);
                w(c, DATA + 0x4000 + 0x10, 0);
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn script_obj_cam_set_binding_matches() {
        strict_2(
            ssx3_ee::s07::f_0027d688__generated,
            script_obj_cam_set_binding,
            SEEDS,
            &|c, seed| {
                let mut r = base_13(c, seed, 0xd688);
                let obj = DATA + 0x1000;
                let vt = DATA + 0x2000;
                let anim = DATA + 0x3000;
                let instance = DATA + 0x5000;
                c.bus.write16(vt + 0x20, 0);
                w(c, vt + 0x24, FAKE);
                c.bus.write16(vt + 0x28, 0);
                w(c, vt + 0x2c, FAKE);
                w(c, obj + 0xc, vt);
                w(c, obj + 0x14, if seed % 3 == 0 { 0 } else { anim });
                w(c, obj + 0x20, r.next_u32());
                w(c, anim + 4, 0);
                w(c, GAME_GP.wrapping_add(0x3ec), instance);
                w(c, instance + 0x5c, 0);
                w(c, instance + 8, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(if seed % 3 == 2 { 0 } else { anim });
            },
        );
    }


    /// A leaf that returns quickly and only reads its first argument: the
    /// stand-in for the virtual methods the originals call.
    pub(crate) const LEAF: u32 = 0x002c_c098;


    pub(crate) const BE_DUMMY: u32 = DATA + 0x7_0000;

    pub(crate) const WORLD_4: u32 = DATA + 0x1_0000;


    /// Scrambled machine with the game image loaded, the BELibrary singleton
    /// and the world pointer installed (real callees otherwise take long or
    /// runaway paths).
    pub(crate) fn base_14(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(salt ^ 0x9e37_79b9_7f4a_7c15 ^ seed as u64);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x1f04, BE_DUMMY);
        c.bus.write32(GAME_GP - 0x848, WORLD_4);
        r
    }


    /// Compare hard state, caller-saved state and LO/HI.
    pub(crate) fn check3(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(g, h, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed} hard: {:#?}", r.hard);
            assert!(r.soft.is_empty(), "seed {seed} soft: {:#?}", r.soft);
            assert!(r.lohi.is_empty(), "seed {seed} lohi: {:#?}", r.lohi);
        }
    }


    /// A sequencer channel whose ten tracks each have a vtable with the two
    /// slots the field readers use, plus a destination buffer.
    pub(crate) fn seq_channel(c: &mut Cpu, r: &mut Rng) -> (u32, u32) {
        let chan = DATA;
        let table = DATA + 0x100;
        c.bus.write32(chan + 0x24, table);
        for i in 0..10u32 {
            let track = DATA + 0x1000 + 0x40 * i;
            let vt = DATA + 0x2000 + 0x40 * i;
            c.bus.write32(table + 4 * i, track);
            c.bus.write32(track + 4, vt);
            c.bus.write32(track, r.next_u32());
            c.bus.write32(track + 0x54, r.next_u32());
            for slot in [0x20u32, 0x28] {
                c.bus.write16(vt + slot, 0);
                c.bus.write32(vt + slot + 4, LEAF);
            }
        }
        (chan, DATA + 0x400)
    }


    #[test]
    fn script_cam_read_subject_fields_matches() {
        check3(ee::s07::f_0027c510__generated, script_cam_read_subject_fields, 40, &|c, seed| {
            let mut r = base_14(c, seed, 0xc510);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }


    #[test]
    fn script_cam_read_algorithmic_fields_matches() {
        check3(ee::s07::f_0027c6f8__generated, script_cam_read_algorithmic_fields, 40, &|c, seed| {
            let mut r = base_14(c, seed, 0xc6f8);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }


    #[test]
    fn script_cam_read_manual_fields_matches() {
        check3(ee::s07::f_0027c0f0__generated, script_cam_read_manual_fields, 40, &|c, seed| {
            let mut r = base_14(c, seed, 0xc0f0);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }


    #[test]
    fn script_cam_read_target_fields_matches() {
        check3(ee::s07::f_0027c300__generated, script_cam_read_target_fields, 40, &|c, seed| {
            let mut r = base_14(c, seed, 0xc300);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }


    #[test]
    fn script_audio_read_fields_matches() {
        check3(ee::s07::f_0027cc28__generated, script_audio_read_fields, 40, &|c, seed| {
            let mut r = base_14(c, seed, 0xcc28);
            let (chan, out) = seq_channel(c, &mut r);
            c.g[A0][0] = sx(chan);
            c.g[A1][0] = sx(out);
        });
    }


    pub(crate) const INTRO_ENGINE: u32 = DATA + 0x1_0000;


    #[test]
    fn script_engine_queue_intro_scripts_matches() {
        check_all(
            ssx3_ee::s07::f_0027aaf8__generated,
            script_engine_queue_intro_scripts,
            50,
            &|c, seed| {
                let _ = base_15(c, seed, 0x27aa_f8, 6);
                c.g[A0][0] = sx(INTRO_ENGINE);
                c.g[A1][0] = sx(seed % 5);
                // State 0: `D_00534B30` set means the fixed intro table is used;
                // otherwise the table lookup comes from `D_00445488`.
                if seed % 2 == 0 {
                    c.bus.write32(0x0053_4b30, 1);
                    put_words(c, 0x0048_1e80, &[3, 4, 0xffff_ffff]);
                } else {
                    c.bus.write32(0x0053_4b30, 0);
                    put_words(c, 0x0044_5488, &[0]);
                }
            },
        );
    }


    // ------------------------------------------------------------------
    // Script engine update.

    const SE_ENGINE: u32 = DATA + 0x2_0000;
    const SE_LISTS_A: u32 = DATA + 0x2_1000;
    const SE_LISTS_B: u32 = DATA + 0x2_1400;
    const SE_WORLD: u32 = DATA + 0x2_2000;
    const SE_GAME: u32 = DATA + 0x2_2200;
    const SE_RACE: u32 = DATA + 0x2_2400;
    const SE_TABLE: u32 = DATA + 0x2_2600;
    const SE_HUD: u32 = DATA + 0x2_2700;
    const SE_RIDERS: u32 = DATA + 0x3_0000;
    const SE_MAP: u32 = DATA + 0x3_8000;
    use crate::game::camera::camera::tests::{INPUT_VM_STOP, INPUT_VM_TABLE, STOP_OPCODE};

    /// `GetHashValue32("")` (the HUD state's name is in the game image; the
    /// fixtures leave it empty).
    fn empty_name_hash() -> u32 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = sx(DATA + 0x7_f000);
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut c);
        assert_eq!(c.pc, TEST_RA);
        c.g[V0][0] as u32
    }

    /// A script engine at [`SE_ENGINE`]: four script lists (the active one,
    /// `+0x550`, playing and nested-paused, its current entry maybe
    /// skippable, the others idle), no processes, movie, views or script
    /// sounds; the race's riders with input maps whose skip binding stops the
    /// input VM at once; the UI state table maybe holding the HUD state.
    fn script_engine_fixture(c: &mut Cpu, seed: u32, hash: u32) -> Rng {
        let mut r = machine(c, seed, 0x278720, 0);
        clear(c, SE_ENGINE, 0x2a0);
        clear(c, SE_LISTS_A, 0x200);
        clear(c, SE_WORLD, 0x800);
        clear(c, SE_MAP, 0x120);
        let e = SE_ENGINE;
        w(c, e + 0x54c, SE_LISTS_A);
        w(c, e + 0x624, SE_LISTS_B);
        // The script sounds: four idle triggers.
        for i in 0..4 {
            w(c, e + 0x92c + 0xbc + 0xc * i, u32::MAX);
        }
        let active = [0u32, 1, 2][seed as usize % 3];
        w(c, e + 0x550, active);
        w(c, e + 0x554, [0, 0, 1, 30][(seed / 3 % 4) as usize]);
        // The lists.
        for i in 0..4u32 {
            let l = if i < 2 { SE_LISTS_A + 0xcc * i } else { SE_LISTS_B + 0xcc * (i - 2) };
            w(c, l + 0xbc, u32::MAX);
            if i == active {
                let playing = seed / 12 % 4 != 0;
                w(c, l + 0xa4, if playing { 3 } else { 0 });
                w(c, l + 0xc4, 1);
                w(c, l + 0xc8, 1);
                w(c, l + 0xa8, (seed / 48) % 2);
                // The current entry's skippable flag (bit 0).
                w(c, l + 0xc + 8, (seed / 12 % 2) | 2);
            }
        }
        // The world, the race and its riders.
        w(c, GAME_GP.wrapping_sub(0x848), SE_WORLD);
        w(c, SE_WORLD + 0x84, SE_GAME);
        w(c, SE_GAME + 0xc, SE_RACE);
        w(c, SE_GAME + 0x48, SE_TABLE);
        w(c, SE_GAME + 0x94, SE_HUD);
        w(c, SE_HUD + 4, r.next_u32() % 4);
        let riders = seed / 5 % 3;
        w(c, SE_RACE + 0x7c, riders);
        for i in 0..2u32 {
            let rider = SE_RIDERS + 0x1000 * i;
            let binding = rider + 0x100;
            let flag = rider + 0x200;
            w(c, SE_RACE + 0x40 + 4 * i, rider);
            w(c, rider + 0xdf0, binding);
            w(c, binding, flag);
            w(c, flag, (r.next_u32() % 3 != 0) as u32);
            w(c, binding + 4, SE_MAP);
        }
        // The input map: the skip binding (0x6f) is one VM stop.
        w(c, SE_MAP + 4, SE_MAP + 0x80);
        w(c, SE_MAP + 0x14, SE_MAP + 0x400);
        w(c, SE_MAP + 0x80 + 4 * 0x6f, 0);
        w(c, SE_MAP + 0x400, STOP_OPCODE);
        w(c, INPUT_VM_TABLE + 4 * STOP_OPCODE, INPUT_VM_STOP);
        // The VM's result (left in the evaluator's frame): pressed or not.
        w(c, SP - 0x70 - 0x10 - 0x1d0 + 8, if seed % 2 == 0 { 0 } else { 1.0f32.to_bits() });
        // The UI state table: empty, or the HUD state (named "").
        clear(c, SE_TABLE, 0x40);
        if seed % 4 == 1 {
            let node = SE_TABLE + 0x80;
            let sentinel = SE_TABLE + 0xc0;
            w(c, SE_TABLE + 0x18 + 4, node);
            w(c, node, sentinel);
            w(c, node + 4, sentinel);
            w(c, node + 0xc, hash);
            w(c, sentinel + 4, sentinel);
            w(c, sentinel, sentinel);
        }
        c.g[A0][0] = sx(e);
        c.pc = 0x0027_8720;
        r
    }

    #[test]
    fn script_engine_update_matches() {
        let hash = empty_name_hash();
        check(ee::s07::f_00278720__generated, script_engine_update, 192, &|c, seed| {
            let _ = script_engine_fixture(c, seed, hash);
        });
    }

    driven!(gen_engine_update, ee::s07::f_00278720__generated);

    driven!(hand_engine_update, script_engine_update);

    #[test]
    fn script_engine_update_events_match() {
        let hash = empty_name_hash();
        check_events(gen_engine_update, hand_engine_update, 48, &|c, seed| {
            let mut r = script_engine_fixture(c, seed, hash);
            schedule(c, &mut r, seed, 0x0027_8720)
        });
    }

    /// The skip delay counts 60 Hz frames: the 30 `GameStateEventSelect`
    /// sets lasts 30 updates at 60 Hz and the same time at 120 / 240 Hz.
    #[test]
    fn script_engine_update_skip_delay_follows_the_rate() {
        let updates = |hz: u32| {
            let mut c = crate::difftest::fresh();
            let c = &mut *c;
            // Seed 2: the engine idle (+0x550 = 2), straight to the delay.
            let _ = script_engine_fixture(c, 2, 0);
            w(c, SE_ENGINE + 0x554, 30);
            crate::tick::set_for_test(c, hz);
            let mut n = 0;
            while c.bus.read32(SE_ENGINE + 0x554) != 0 {
                n += 1;
                assert!(n < 1000);
                c.pc = 0x0027_8720;
                c.g[A0][0] = sx(SE_ENGINE);
                script_engine_update(c);
                assert_eq!(c.pc, TEST_RA, "{hz} Hz: update {n} did not return");
            }
            n
        };
        assert_eq!(updates(60), 30);
        for hz in [120u32, 240] {
            let q = hz / 60;
            assert_eq!(updates(hz), 30 * q - (q - 1), "{hz} Hz");
        }
    }
}
