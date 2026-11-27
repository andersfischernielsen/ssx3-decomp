//! `src/wscript/wscriptman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::SAVE_STATE_STREAM_WRITE;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerman::OC429_DELETE;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p_3;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::intersect::worldsphtree::forward_to_arg_vtable;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::ssxapp::oc429_at;
pub(crate) use crate::game::movie::movieplayer::add32;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_cdf8, playback_man_stop, "PlaybackMan_Stop"),
    (
        0x0030_7d78,
        wscript_process_terminate,
        "WScriptProcess_Terminate",
    ),
    (
        0x0030_9568,
        wscript_man_create_mission,
        "WScriptMan_CreateMission",
    ),
    (
        0x0030_9750,
        w_script_man_start_process,
        "WScriptMan_StartProcess",
    ),
    (
        0x0030_9798,
        w_script_man_stop_process,
        "WScriptMan_StopProcess",
    ),
    (
        0x0030_97e0,
        wscript_man_post_trigger_event,
        "WScriptMan_PostTriggerEvent",
    ),
    (
        0x0030_9848,
        wscript_man_post_jump_event,
        "WScriptMan_PostJumpEvent",
    ),
    (
        0x0030_9918,
        wscript_man_post_trick_event,
        "WScriptMan_PostTrickEvent",
    ),
    (
        0x0030_9990,
        wscript_man_post_wipeout_event,
        "WScriptMan_PostWipeoutEvent",
    ),
    (
        0x0030_99f8,
        wscript_man_post_reset_event,
        "WScriptMan_PostResetEvent",
    ),
    (
        0x0030_9a60,
        wscript_man_activate_process,
        "WScriptMan_ActivateProcess",
    ),
    (
        0x0030_9aa0,
        wscript_man_deactivate_process,
        "WScriptMan_DeactivateProcess",
    ),
    (
        0x0030_9b00,
        w_script_man_deactivate_mission,
        "WScriptMan_DeactivateMission",
    ),
    (
        0x0030_9ba8,
        w_script_man_get_hud_target_height,
        "WScriptMan_GetHudTargetHeight",
    ),
    (
        0x0030_9dd0,
        w_script_man_table_has_function,
        "WScriptMan_TableHasFunction",
    ),
    (
        0x0030_9e50,
        wscript_man_call_table_function,
        "WScriptMan_CallTableFunction",
    ),
    (
        0x0030_a270,
        wscript_man_has_obj_script1,
        "WScriptMan_HasObjScript1",
    ),
    (
        0x0030_a310,
        wscript_man_is_scripted_instance,
        "WScriptMan_IsScriptedInstance",
    ),
    (
        0x0030_a598,
        wscript_man_has_kill_script,
        "WScriptMan_HasKillScript",
    ),
    (
        0x0030_a610,
        wscript_man_run_obj_script0,
        "WScriptMan_RunObjScript0",
    ),
    (
        0x0030_a6d8,
        wscript_man_jump_to_task,
        "WScriptMan_JumpToTask",
    ),
    (0x0030_a700, wscript_man_goto_task, "WScriptMan_GotoTask"),
    (
        0x0030_aeb8,
        wscript_man_get_current_mission,
        "WScriptMan_GetCurrentMission",
    ),
    (
        0x0030_b058,
        wscript_process_list_ctor,
        "WScriptProcessList_ctor",
    ),
    (
        0x0030_b068,
        w_script_process_list_dtor,
        "WScriptProcessList_dtor",
    ),
    (
        0x0030_b0e8,
        wscript_process_list_delete_by_section,
        "WScriptProcessList_DeleteBySection",
    ),
    (
        0x0030_b1b8,
        wscript_process_list_delete_all,
        "WScriptProcessList_DeleteAll",
    ),
    (
        0x0030_b208,
        wscript_process_list_pop_front,
        "WScriptProcessList_PopFront",
    ),
    (
        0x0030_b260,
        wscript_process_list_push_front,
        "WScriptProcessList_PushFront",
    ),
    (
        0x0030_b270,
        wscript_process_list_push_back,
        "WScriptProcessList_PushBack",
    ),
    (
        0x0030_b2c0,
        wscript_process_list_push_front_2,
        "WScriptProcessList_PushFront_2",
    ),
    (
        0x0030_b2d0,
        wscript_process_list_remove,
        "WScriptProcessList_Remove",
    ),
    (
        0x0030_b320,
        wscript_process_list_find_by_id,
        "WScriptProcessList_FindById",
    ),
    (
        0x0030_b388,
        wscript_process_list_find_mission_by_id,
        "WScriptProcessList_FindMissionById",
    ),
    (
        0x0030_b428,
        wscript_process_list_contains,
        "WScriptProcessList_Contains",
    ),
    (
        0x0030_b520,
        wscript_man_set_pending_rider_config,
        "WScriptMan_SetPendingRiderConfig",
    ),
    (
        0x0030_b540,
        wscript_man_request_mission_start,
        "WScriptMan_RequestMissionStart",
    ),
    (
        0x0030_b658,
        wscript_man_request_mission_cancel,
        "WScriptMan_RequestMissionCancel",
    ),
    (
        0x0030_b6f8,
        w_script_man_request_mission_restart,
        "WScriptMan_RequestMissionRestart",
    ),
    (
        0x0030_b7f8,
        wscript_man_deactivate_all,
        "WScriptMan_DeactivateAll",
    ),
    (
        0x0030_b928,
        wscript_man_define_collect_set,
        "WScriptMan_DefineCollectSet",
    ),
    (
        0x0030_b9a0,
        w_script_man_collect_item,
        "WScriptMan_CollectItem",
    ),
    (
        0x0030_ba80,
        wscript_man_get_collected_count,
        "WScriptMan_GetCollectedCount",
    ),
    (
        0x0030_bac8,
        wscript_man_add_collect_items,
        "WScriptMan_AddCollectItems",
    ),
    (
        0x0030_bee8,
        wscriptman_load_process_list,
        "WScriptMan_LoadProcessList",
    ),
    (
        0x0030_c390,
        wscript_man_reset_collect_set,
        "WScriptMan_ResetCollectSet",
    ),
    (
        0x0030_c3e0,
        wscript_man_collect_set_mark_item,
        "WScriptMan_CollectSetMarkItem",
    ),
    (
        0x0030_c468,
        wscript_man_collect_set_get_count,
        "WScriptMan_CollectSetGetCount",
    ),
    (
        0x0030_c700,
        wscript_man_collect_set_load,
        "WScriptMan_CollectSetLoad",
    ),
    (
        0x0030_c738,
        wscript_man_cmd_queue_ctor,
        "WScriptMan_CmdQueueCtor",
    ),
    (
        0x0030_c760,
        wscript_man_reset_command_queue,
        "WScriptMan_ResetCommandQueue",
    ),
    (
        0x0030_c790,
        wscript_man_event_queue_reset,
        "WScriptMan_EventQueueReset",
    ),
    (
        0x0030_c7c0,
        wscript_man_event_queue_push,
        "WScriptMan_EventQueuePush",
    ),
    (
        0x0030_d420,
        wscript_man_sub_obj_dtor_d420,
        "WScriptMan_SubObjDtor_d420",
    ),
    (0x0030_d4b8, rider_model_ctor, "RiderModel_ctor"),
    (
        0x0030_d840,
        rider_model_alloc_model_refs,
        "RiderModel_AllocModelRefs",
    ),
    (0x0035_06d8, dead_node_ctor, "DeadNode_ctor"),
    (0x0030_9c88, wscript_man_call_func, "WScriptMan_CallFunc"),
    (
        0x0030_a060,
        wscript_man_on_object_trigger,
        "WScriptMan_OnObjectTrigger",
    ),
    (
        0x0030_a460,
        wscript_man_focus_leave,
        "WScriptMan_FocusLeave",
    ),
    (
        0x0030_a3a0,
        wscript_man_focus_enter,
        "WScriptMan_FocusEnter",
    ),
    (0x0030_bb10, wscript_man_save_state, "WScriptMan_SaveState"),
    (
        0x0030_bc80,
        wscript_man_save_process_list,
        "WScriptMan_SaveProcessList",
    ),
    (
        0x0030_a298,
        wscript_man_run_obj_script1,
        "WScriptMan_RunObjScript1",
    ),
    (
        0x0030_98b0,
        wscript_man_post_air_event,
        "WScriptMan_PostAirEvent",
    ),
    (
        0x0030_b228,
        wscript_process_list_count,
        "WScriptProcessList_Count",
    ),
    (
        0x0030_c6c8,
        wscript_man_collect_set_save,
        "WScriptMan_CollectSetSave",
    ),
    (
        0x0030_a5c0,
        wscript_man_run_kill_script,
        "WScriptMan_RunKillScript",
    ),
    (
        0x0030_a868,
        wscript_man_mission_command,
        "WScriptMan_MissionCommand",
    ),
    (
        0x0030_c4a8,
        wscript_man_collect_set_add_items,
        "WScriptMan_CollectSetAddItems",
    ),
    (0x0030_d540, rider_model_dtor, "RiderModel_dtor"),
    (0x0030_9f18, wscript_man_shutdown, "WScriptMan_Shutdown"),
    (
        0x0030_96f0,
        wscript_man_unlink_process,
        "WScriptMan_UnlinkProcess",
    ),
    (
        0x0030_9d20,
        wscript_man_call_func_on_instance_2,
        "WScriptMan_CallFuncOnInstance",
    ),
    (
        0x0030_a660,
        wscript_man_has_obj_script5,
        "WScriptMan_HasObjScript5",
    ),
    (
        0x0030_a728,
        wscript_man_process_command,
        "WScriptMan_ProcessCommand",
    ),
    (
        0x0030_ac98,
        wscript_man_call_process_func,
        "WScriptMan_CallProcessFunc",
    ),
    (
        0x0030_ada8,
        wscript_man_call_task_func,
        "WScriptMan_CallTaskFunc",
    ),
    (
        0x0030_af08,
        wscript_man_get_event_arg,
        "WScriptMan_GetEventArg",
    ),
    (
        0x0030_b350,
        wscript_process_list_find_by_name,
        "WScriptProcessList_FindByName",
    ),
    (
        0x0030_b3f0,
        wscript_process_list_find_mission_by_name,
        "WScriptProcessList_FindMissionByName",
    ),
    (
        0x0030_b458,
        wscript_man_find_mission_by_id,
        "WScriptMan_FindMissionById",
    ),
    (
        0x0030_b4c0,
        wscript_man_set_pending_request_a,
        "WScriptMan_SetPendingRequestA",
    ),
    (
        0x0030_b4d8,
        wscript_man_set_pending_request_b,
        "WScriptMan_SetPendingRequestB",
    ),
    (
        0x0030_b4f0,
        wscript_man_set_pending_request_c,
        "WScriptMan_SetPendingRequestC",
    ),
    (
        0x0030_b508,
        wscript_man_set_hud_height_target,
        "WScriptMan_SetHudHeightTarget",
    ),
    (
        0x0030_b608,
        wscript_man_request_mission_abort,
        "WScriptMan_RequestMissionAbort",
    ),
    (
        0x0030_b758,
        wscript_man_request_mission_end_2,
        "WScriptMan_RequestMissionEnd",
    ),
    (
        0x0030_b898,
        wscript_man_is_process_active,
        "WScriptMan_IsProcessActive",
    ),
    (
        0x0030_c898,
        wscriptman_debugprintf,
        "WScriptMan_DebugPrintf",
    ),
    (
        0x0030_c8d8,
        wscript_man_static_init,
        "WScriptMan_StaticInit",
    ),
    (0x0030_d348, wscriptman_pop_event, "WScriptMan_PopEvent"),
    (0x0030_d368, wscriptman_peek_event, "WScriptMan_PeekEvent"),
    (
        0x0030_d3e0,
        wscriptprocess_run_callback,
        "WScriptProcess_RunCallback",
    ),
    (
        0x0030_d400,
        wscripttask_run_callback,
        "WScriptTask_RunCallback",
    ),
    (
        0x0030_d498,
        wscriptman_global_ctors,
        "WScriptMan_GlobalCtors",
    ),
    (
        0x0030_94c8,
        wscript_man_create_process_2,
        "WScriptMan_CreateProcess",
    ),
    (
        0x0030_9618,
        wscript_man_delete_process_by_id,
        "WScriptMan_DeleteProcessById",
    ),
    (
        0x0030_b590,
        wscript_man_request_mission_playback_2,
        "WScriptMan_RequestMissionPlayback",
    ),
    (
        0x0030_bfc0,
        wscript_man_check_rider_gate,
        "WScriptMan_CheckRiderGate",
    ),
    (0x0030_bd20, wscript_man_load_state, "WScriptMan_LoadState"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::bx::bxstringctor::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstringctor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT_3;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;

    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR_3;

    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;

    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;

    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_CONTAINS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT as WSCRIPT_PROCESS_LIST_PUSH_FRONT_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE as WSCRIPT_PROCESS_LIST_REMOVE_2;

    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;

    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;

    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;

    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT1;
    pub(crate) use crate::game::object::debouncenode::callee::HAS_OBJECT_SCRIPT3;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;

    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;

    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;

    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;

    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;

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
pub(crate) mod g {}

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
pub(crate) mod addr {}

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
    c.ldr(2, c.g[A1][0] as u32);
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
    guest(c, |c| {
        forward_to_arg_vtable(c, 0x0030_c728, 0x10, 0x10c, None)
    })
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
        c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
        let _idx = abi::arg(c, 1);
        let _value = abi::arg(c, 2);
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
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[V0][0] = c.g[A2][0] & 0xff;
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
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
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
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
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[RA][0] = sx(0x00309c4c);
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x00309c4c {
                        return Err(crate::call::Unwound);
                    }
                    c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sq(3, c.g[SP][0] as u32);
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
        c.pc = t;
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
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        'l_30b9c0: loop {
            'b_30ba6c: {
                'b_30ba60: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
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
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
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
                    if event_due(c) {
                        deliver_event_here(c, 0x0030b9c0);
                    }
                    continue 'l_30b9c0;
                } else {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    break 'b_30ba6c;
                }
            }
            c.cycles += 5;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = t;
            return Ok(());
        }
    });
}

// ---------------------------------------------------------------------------
// WScriptMan_CallFunc (0x309c88, wscriptman.cpp)

/// WScriptMan fields: the world (script cache owner), the per-section
/// script tables (`cLunoValue*[]`, indexed by the handle's low byte).
pub(crate) const WSM_WORLD: u32 = 0x28c;

pub(crate) const WSM_SECTION_TABLES: u32 = 0x3cc;

/// Callees (all through the function table).
pub(crate) const WSM_WORLD_GET_SECTION_SCRIPT_FUNC: u32 = 0x003a_6d18;

pub(crate) const WSM_LUNO_VM_REGISTER_SET_RAW: u32 = 0x0022_6600;

pub(crate) const WSM_LUNO_VM_PASS_THROUGH: u32 = 0x0022_24a8;

pub(crate) const WSM_LUNO_VM_CALL: u32 = 0x0022_24b8;

pub(crate) const WSM_LUNO_VM_REGISTER_DTOR: u32 = 0x0022_5b90;

/// `void WScriptMan_CallFunc(WScriptMan* self, ScriptHandle* handle)`
/// (0x309c88): looks up the scripted function `handle` names in the world's
/// script cache, wraps it in a Luno value and calls it on the section's
/// script table with an empty result value, which is destroyed afterwards.
pub fn wscript_man_call_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        // func = World_GetSectionScriptFunc(self->world, handle)
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(WSM_WORLD)).lw(c);
        call_ptr(c, WSM_WORLD_GET_SECTION_SCRIPT_FUNC, 0x0030_9cac)?;
        // arg = {func, 0} at sp
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        call_ptr(c, WSM_LUNO_VM_REGISTER_SET_RAW, 0x0030_9cbc)?;
        let section = Addr(r32(c, S1)).lbu(c);
        c.g[V0][0] = section;
        c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
        Addr(sp(c).wrapping_add(0x18)).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, 0);
        c.g[V0][0] = sx((section as u32) << 2);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(c.g[V0][0] as u32));
        c.g[S1][0] = Addr(r32(c, S0).wrapping_add(WSM_SECTION_TABLES)).lw(c);
        call_ptr(c, WSM_LUNO_VM_PASS_THROUGH, 0x0030_9ce0)?;
        // LunoVM_Call(&call, &arg, table, &result)
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, WSM_LUNO_VM_CALL, 0x0030_9cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, WSM_LUNO_VM_REGISTER_DTOR, 0x0030_9d04)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_FocusLeave (0x30a460, wscriptman.cpp)

/// World object: its script controller (or 0).
pub(crate) const OC264_WOBJ_CTL: u32 = 0xc;

/// Script controller: vtable, kind (6: no focus scripts), owner object.
pub(crate) const OC264_WCTL_VTABLE: u32 = 0xc;

pub(crate) const OC264_WCTL_KIND: u32 = 0x10;

pub(crate) const OC264_WCTL_OWNER: u32 = 0x18;

pub(crate) const OC264_WCTL_KIND_PASSIVE: u64 = 6;

/// Controller vtable entries (this-adjust halfword, method): v20 "handles
/// focus itself", v22 "focus leave", v35 "lost focus".
pub(crate) const OC264_WCTL_V20: u32 = 0xa0;

pub(crate) const OC264_WCTL_V22: u32 = 0xb0;

pub(crate) const OC264_WCTL_V35: u32 = 0x118;

pub(crate) const OC264_WSCRIPT_HAS_OBJ_SCRIPT3: Callee = Callee {
    addr: 0x0030_a2e8,
    f: ee::s10::WScriptMan_HasObjScript3,
};

pub(crate) const OC264_WSCRIPT_RUN_OBJ_SCRIPT3: Callee = Callee {
    addr: 0x0030_a548,
    f: ee::s10::WScriptMan_RunObjScript3,
};

/// Call virtual method `slot` (this-adjust halfword at `slot`, function
/// at `slot + 4`) of the controller in `ctl_reg`, loading the vtable into
/// `vt_reg` and the method into `fn_reg` as the original does.
pub(crate) fn oc264_wctl_call(
    c: &mut Cpu,
    ctl_reg: usize,
    vt_reg: usize,
    fn_reg: usize,
    slot: u32,
    ret: u32,
) -> crate::call::Flow {
    let ctl = r32(c, ctl_reg);
    let vtable = Addr(ctl.wrapping_add(OC264_WCTL_VTABLE)).lw(c);
    c.g[vt_reg][0] = vtable;
    let adjust = Addr((vtable as u32).wrapping_add(slot)).lh(c);
    let method = Addr((vtable as u32).wrapping_add(slot + 4)).lw(c);
    c.g[fn_reg][0] = method;
    c.g[A0][0] = sx(ctl.wrapping_add(adjust as u32));
    call_ptr(c, method as u32, ret)
}

/// `void WScriptMan_FocusLeave(WScriptMan* self, WorldObj* obj)` (0x30a460):
/// the camera/focus leaves `obj`. A scripted controller that handles focus
/// itself (v20) gets v22; a passive one (kind 6) nothing; otherwise the
/// controller's owner runs its "focus leave" object script (slot 3) if it
/// has one, else the controller's v35. Without a controller the object's
/// own slot-3 script runs if present.
pub fn wscript_man_focus_leave(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
        c.g[S1][0] = c.g[A0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 7;
            oc264_wctl_call(c, A1, V1, V0, OC264_WCTL_V20, 0x0030_a494)?;
            let handles = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if handles {
                c.cycles += 8;
                c.g[A2][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                oc264_wctl_call(c, A2, V1, V0, OC264_WCTL_V22, 0x0030_a4b4)?;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                let ctl = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                c.g[A1][0] = ctl;
                c.g[V1][0] = OC264_WCTL_KIND_PASSIVE;
                c.g[V0][0] = Addr((ctl as u32).wrapping_add(OC264_WCTL_KIND)).lh(c);
                let passive = c.g[V0][0] == c.g[V1][0];
                c.g[A0][0] = c.g[S1][0];
                if passive {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 5;
                    c.g[S0][0] = Addr((ctl as u32).wrapping_add(OC264_WCTL_OWNER)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    OC264_WSCRIPT_HAS_OBJ_SCRIPT3.call(c, 0x0030_a4dc)?;
                    let has = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if has {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S0][0];
                        OC264_WSCRIPT_RUN_OBJ_SCRIPT3.call(c, 0x0030_a4ec)?;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                        oc264_wctl_call(c, A1, V0, V1, OC264_WCTL_V35, 0x0030_a50c)?;
                    }
                    frame.restore(c, S0, 0x20);
                }
            }
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            OC264_WSCRIPT_HAS_OBJ_SCRIPT3.call(c, 0x0030_a520)?;
            let has = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if has {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                OC264_WSCRIPT_RUN_OBJ_SCRIPT3.call(c, 0x0030_a530)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_OnObjectTrigger (0x30a060)

/// Script manager fields: the current object id (+0), the cache (+0x28c),
/// the trigger being run (+0x290, from the trigger volume's +0x50), its
/// source (+0x294) and volume (+0x298), and the script owner (+0x2a4) or
/// owning instance (+0x2a8, whose owner is at +0x3c).
pub(crate) const WSM_CURRENT_ID: u32 = 0x0;

pub(crate) const WSM_CACHE: u32 = 0x28c;

pub(crate) const WSM_TRIGGER: u32 = 0x290;

pub(crate) const WSM_SOURCE: u32 = 0x294;

pub(crate) const WSM_VOLUME: u32 = 0x298;

pub(crate) const WSM_OWNER: u32 = 0x2a4;

pub(crate) const WSM_INSTANCE: u32 = 0x2a8;

pub(crate) const WSM_ARGS: u32 = 0x24;

pub(crate) const WSM_POST_TRIGGER_EVENT: Callee = Callee {
    addr: 0x0030_97e0,
    f: ee::s10::WScriptMan_PostTriggerEvent,
};

pub(crate) const WSM_CALL_FUNC: Callee = Callee {
    addr: 0x0030_9c88,
    f: ee::s10::WScriptMan_CallFunc,
};

pub(crate) const WSM_FIND_FUNC: u32 = 0x003a_6b78;

pub(crate) const WSM_FUNC_ENTRY: u32 = 0x003a_6d18;

pub(crate) const WSM_VALUE_FROM_FUNC: u32 = 0x0022_6600;

pub(crate) const WSM_CALL_INIT: u32 = 0x0022_24a8;

pub(crate) const WSM_CALL_RUN: u32 = 0x0022_24b8;

pub(crate) const WSM_ALLOC: u32 = 0x0031_7d70;

pub(crate) const WSM_TABLE_CTOR: u32 = 0x0022_4da0;

pub(crate) const WSM_TABLE_DTOR: u32 = 0x0022_4df0;

pub(crate) const WSM_VALUE_RELEASE: u32 = 0x0022_5b90;

/// The allocation name (`0x4898d8`).
pub(crate) const WSM_TABLE_NAME: u32 = 0x0049_0000 - 0x6728;

/// `obj->vtable[slot](obj)`: `lw vt; lh delta; lw method` with the
/// registers `vt`/`m` the original uses, returning to `ret`.
pub(crate) fn wsm_virtual(
    c: &mut Cpu,
    obj: usize,
    slot: u32,
    vt: usize,
    m: usize,
    ret: u32,
) -> crate::call::Flow {
    let this = r32(c, obj);
    c.g[vt][0] = Addr(this).lw(c);
    let table = r32(c, vt);
    c.g[A0][0] = Addr(table).off(slot).lh(c);
    c.g[m][0] = Addr(table).off(slot + 4).lw(c);
    let method = r32(c, m);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void WScriptMan_OnObjectTrigger(WScriptMan *self, void *source,
/// TriggerVolume *volume, Object *obj)` (0x30a060): an object entered a
/// trigger volume. Makes the object current (its id, vtable slot 0x38),
/// records the trigger, and for an object that says so (slot 0x40) posts
/// the trigger event, unless it is a rider who is already finished outside
/// event mode 4. Then runs the trigger's script function (`id != -1`):
/// directly without an owner, else through a Luno call with the owner's
/// table (or a fresh one, freed after). Finally the trigger's effect, if
/// any, is played at 0.5 (slot 0x148), and the current id and trigger
/// state are restored / cleared.
pub fn wscript_man_on_object_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A1][0]; // source
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A2][0]; // volume
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A0][0]; // self
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[A3][0]; // obj
        frame.save_ra(c, 0x30);
        // self->current = obj->GetId(); s4 = the previous one.
        let obj = r32(c, S3);
        c.g[V1][0] = Addr(obj).lw(c);
        c.g[S4][0] = Addr(r32(c, S2)).off(WSM_CURRENT_ID).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(0x38).lh(c);
        c.g[V0][0] = Addr(vt).off(0x3c).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0030_a0a4)?;
        let wsm = Addr(r32(c, S2));
        let id = c.g[V0][0];
        wsm.off(WSM_CURRENT_ID).sw(c, id);
        c.g[V0][0] = Addr(r32(c, S1)).off(0x50).lw(c);
        let (source, trigger, volume) = (c.g[S0][0], c.g[V0][0], c.g[S1][0]);
        wsm.off(WSM_SOURCE).sw(c, source);
        wsm.off(WSM_TRIGGER).sw(c, trigger);
        wsm.off(WSM_VOLUME).sw(c, volume);
        wsm_virtual(c, S3, 0x40, V1, V0, 0x0030_a0cc)?;
        let posts = c.g[V0][0] != 0;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        let mut run = true;
        if posts {
            c.cycles += 5;
            c.g[A1][0] = 4;
            c.g[V0][0] = Addr(r32(c, A0)).off(0x84).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x214).lw(c);
            let event_mode = c.g[V1][0] == 4;
            // The rider that owns obj (its interface at +0x6c0).
            c.g[V0][0] = sx(r32(c, S3).wrapping_sub(0x6c0));
            if !event_mode {
                c.cycles += 4;
                if c.g[S3][0] == 0 {
                    c.g[V0][0] = 0;
                }
                c.g[V1][0] = Addr(r32(c, V0)).off(0x480).lw(c);
                if c.g[V1][0] != 0 {
                    let prev = c.g[S4][0];
                    Addr(r32(c, S2)).sw(c, prev);
                    run = false;
                }
            }
            if run {
                c.cycles += 4;
                c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = Addr(r32(c, V0)).off(0x78).lw(c);
                WSM_POST_TRIGGER_EVENT.call(c, 0x0030_a108)?;
            }
        }
        if run {
            wsm_run_trigger_func(c)?;
            // The trigger's effect (+0xc), played at 0.5.
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 8;
                let fx = r32(c, A1);
                c.g[V0][0] = Addr(fx).off(0xc).lw(c);
                c.g[AT][0] = 0x3f00_0000;
                c.f[12] = 0x3f00_0000;
                let vt = r32(c, V0);
                c.g[A0][0] = Addr(vt).off(0x148).lh(c);
                c.g[V1][0] = Addr(vt).off(0x14c).lw(c);
                let method = r32(c, V1);
                c.g[A0][0] = sx(fx.wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x0030_a240)?;
            }
            let prev = c.g[S4][0];
            Addr(r32(c, S2)).sw(c, prev);
        }

        c.cycles += 11;
        let wsm = Addr(r32(c, S2));
        wsm.off(WSM_TRIGGER).sw(c, 0);
        wsm.off(WSM_SOURCE).sw(c, 0);
        wsm.off(WSM_VOLUME).sw(c, 0);
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// Find and run the trigger's script function; leaves `v0 = self->trigger`.
/// Stack: the argument value at sp+0, the result at sp+0x10, the Luno call
/// at sp+0x20, the function id at sp+0x24.
pub(crate) fn wsm_run_trigger_func(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    c.cycles += 8;
    c.g[A1][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
    c.g[A2][0] = 2;
    c.g[A0][0] = Addr(r32(c, S2)).off(WSM_CACHE).lw(c);
    call_ptr(c, WSM_FIND_FUNC, 0x0030_a118)?;
    c.g[A1][0] = sx(s + WSM_ARGS);
    c.g[V1][0] = !c.g[V0][0];
    let id = c.g[V0][0];
    Addr(s + WSM_ARGS).sw(c, id);
    if c.g[V1][0] == 0 {
        // No function (-1).
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S2)).off(WSM_OWNER).lw(c);
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S2)).off(WSM_INSTANCE).lw(c);
        if c.g[V0][0] == 0 {
            // No owner: a plain call.
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            WSM_CALL_FUNC.call(c, 0x0030_a148)?;
            c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
            return Ok(());
        }
    }
    // A Luno call with the owner's table as `self`.
    c.cycles += 14;
    c.g[A0][0] = Addr(r32(c, S2)).off(WSM_CACHE).lw(c);
    call_ptr(c, WSM_FUNC_ENTRY, 0x0030_a158)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    call_ptr(c, WSM_VALUE_FROM_FUNC, 0x0030_a168)?;
    c.g[A0][0] = sx(s + 0x20);
    Addr(s + 0x18).sw(c, 0);
    c.g[S3][0] = c.g[A0][0];
    Addr(s + 0x10).sw(c, 0);
    call_ptr(c, WSM_CALL_INIT, 0x0030_a17c)?;
    c.g[V0][0] = Addr(r32(c, S2)).off(WSM_INSTANCE).lw(c);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = Addr(r32(c, S2)).off(WSM_OWNER).lw(c);
    } else {
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, V0)).off(0x3c).lw(c);
    }
    c.cycles += 3;
    c.g[S1][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
    c.g[V0][0] = sx(s + 0x10);
    if c.g[S1][0] != 0 {
        c.cycles += 8;
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, WSM_CALL_RUN, 0x0030_a1b0)?;
        c.g[A0][0] = c.g[S0][0];
    } else {
        // No table yet: a temporary one, released after the call.
        c.cycles += 18;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(WSM_TABLE_NAME);
        c.g[A0][0] = 4;
        call_ptr(c, WSM_ALLOC, 0x0030_a1d0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, WSM_TABLE_CTOR, 0x0030_a1dc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(s + 0x10);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, WSM_CALL_RUN, 0x0030_a1f8)?;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call_ptr(c, WSM_TABLE_DTOR, 0x0030_a208)?;
        }
        c.cycles += 1;
        c.g[A0][0] = c.g[S0][0];
    }
    // Release the result value.
    c.cycles += 2;
    c.g[A1][0] = 2;
    call_ptr(c, WSM_VALUE_RELEASE, 0x0030_a214)?;
    c.cycles += 1;
    c.g[V0][0] = Addr(r32(c, S2)).off(WSM_TRIGGER).lw(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// WScriptMan_FocusEnter (0x30a3a0)

/// Controller vtable slot v21 ("focus enter") and the object-type table
/// entry size; type 3 objects run their slot-1 object script.
pub(crate) const WSF_CTL_V21: u32 = 0xa8;

pub(crate) const WSF_MAN_CACHE: u32 = 0x28c;

pub(crate) const WSF_OBJ_TYPE: u32 = 0x78;

pub(crate) const WSF_TYPE_ENTRY: u32 = 0xc;

pub(crate) const WSF_TYPE_SCRIPTED: u64 = 3;

pub(crate) const WSF_HAS_OBJ_SCRIPT1: Callee = Callee {
    addr: 0x0030_a270,
    f: ee::s10::WScriptMan_HasObjScript1,
};

pub(crate) const WSF_RUN_OBJ_SCRIPT1: Callee = Callee {
    addr: 0x0030_a298,
    f: ee::s10::WScriptMan_RunObjScript1,
};

/// `void WScriptMan_FocusEnter(WScriptMan* self, WorldObj* obj)` (0x30a3a0):
/// the camera/focus enters `obj`. A controller that handles focus itself
/// (v20 non-zero) gets v21(obj); without a controller, an object whose type
/// (from the script cache's type table) is 3 runs its slot-1 object script
/// if it has one.
pub fn wscript_man_focus_enter(c: &mut Cpu) {
    use crate::game::wscript::wscriptman::{oc264_wctl_call, OC264_WCTL_V20, OC264_WOBJ_CTL};
    use crate::guest::Addr;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
        c.g[S1][0] = c.g[A0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 7;
            oc264_wctl_call(c, A1, V1, V0, OC264_WCTL_V20, 0x0030_a3d4)?;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                c.g[A2][0] = Addr(r32(c, S0).wrapping_add(OC264_WOBJ_CTL)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                oc264_wctl_call(c, A2, V1, V0, WSF_CTL_V21, 0x0030_a3f8)?;
            }
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 11;
            // self->cache->+4->+4: the object-type table (12-byte entries).
            let cache = Addr(r32(c, S1).wrapping_add(WSF_MAN_CACHE)).lw(c);
            c.g[A0][0] = cache;
            c.g[A1][0] = WSF_TYPE_ENTRY as u64;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WSF_OBJ_TYPE)).lbu(c);
            c.g[V0][0] = Addr((cache as u32).wrapping_add(4)).lw(c);
            c.mult(0, V1, V1, A1);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            c.g[V1][0] = crate::abi::addu(c.g[V1][0], c.g[A0][0]);
            c.g[V0][0] = Addr(r32(c, V1)).lh(c) ^ WSF_TYPE_SCRIPTED;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                WSF_HAS_OBJ_SCRIPT1.call(c, 0x0030_a438)?;
                let has = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if has {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    WSF_RUN_OBJ_SCRIPT1.call(c, 0x0030_a448)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod wsm_save {

    /// The plain state block written first (0x28c bytes from `self`).
    pub const STATE_SIZE: u64 = 0x28c;

    /// The world (`world + 4` is its script cache).
    pub const WORLD: u32 = 0x28c;

    /// The two process lists.
    pub const LIST_A: u32 = 0x2b8;

    pub const LIST_B: u32 = 0x2bc;

    /// The collect-set table (one, 0x10c bytes).
    pub const COLLECT_SET: u32 = 0x2c0;

    pub const COLLECT_SET_SIZE: u32 = 0x10c;

    /// The 50 Luno object tables.
    pub const OBJS: u32 = 0x3cc;

    pub const OBJ_COUNT: u64 = 50;
}

// ---------------------------------------------------------------------------
// WScriptMan_SaveProcessList (0x30bc80)

/// WScriptProcess: next in its list, script id.
pub(crate) const WSCRIPT_SAVE_LIST_NEXT: u32 = 0x18;

pub(crate) const WSCRIPT_SAVE_LIST_ID: u32 = 0x20;

pub(crate) const WSCRIPT_SAVE_LIST_COUNT: Callee = Callee {
    addr: 0x0030_b228,
    f: ee::s10::WScriptProcessList_Count,
};

pub(crate) const WSCRIPT_SAVE_PROCESS: Callee = Callee {
    addr: 0x0030_8038,
    f: ee::s10::WScriptProcess_SaveState,
};

/// `stream->Write(stream, a1, a2)` (s1 = the stream) with `v1` = the stream's vtable
/// (already loaded by the caller), returning to `ret`.
pub(crate) fn wscript_save_list_write_word(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    let stream = c.g[S1][0];
    let vt = c.g[V1][0];
    let slot = VSlot::read(c, vt, SAVE_STATE_STREAM_WRITE);
    c.g[A0][0] = slot.delta;
    c.g[V0][0] = slot.func;
    c.g[A0][0] = slot.this(stream);
    call_ptr(c, slot.func as u32, ret)
}

/// `void WScriptMan::SaveProcessList(WScriptMan*, WScriptProcessList* list,
/// Stream* s)` (0x30bc80): writes the number of processes, then for each
/// process its script id and its state (`WScriptProcess_SaveState`). The
/// list walk keeps the node in s0 and the stream in s1, as the original.
pub fn wscript_man_save_process_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0]; // list
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A2][0]; // stream
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        WSCRIPT_SAVE_LIST_COUNT.call(c, 0x0030_bca0)?;

        // The count.
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        let count = c.g[V0][0];
        Addr(sp(c)).sw(c, count);
        c.g[A2][0] = 4;
        wscript_save_list_write_word(c, 0x0030_bcc0)?;

        // Each process: its id, then its state.
        c.g[S0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            loop {
                c.cycles += 14;
                c.g[A1][0] = sx(sp(c).wrapping_add(4));
                c.g[A3][0] = Addr(r32(c, S0).wrapping_add(WSCRIPT_SAVE_LIST_ID)).lw(c);
                c.g[A2][0] = 4;
                let id = c.g[A3][0];
                Addr(sp(c).wrapping_add(4)).sw(c, id);
                wscript_save_list_write_word(c, 0x0030_bcf0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                WSCRIPT_SAVE_PROCESS.call(c, 0x0030_bcfc)?;
                c.g[S0][0] = Addr(r32(c, S0).wrapping_add(WSCRIPT_SAVE_LIST_NEXT)).lw(c);
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0030_bcd0);
                }
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const WSM_COLLECT_SET_SAVE: Callee = Callee {
    addr: 0x0030_c6c8,
    f: ee::s10::WScriptMan_CollectSetSave,
};

pub(crate) const WSM_SAVE_PROCESS_LIST: Callee = Callee {
    addr: 0x0030_bc80,
    f: ee::s10::WScriptMan_SaveProcessList,
};

/// `cWScriptCache_GetData` and `cLunoTable_Write` (cross-shard: the table).
pub(crate) const WSM_CACHE_GET_DATA: u32 = 0x003a_ca38;

pub(crate) const WSM_LUNO_TABLE_WRITE: u32 = 0x0022_57e0;

/// `stream->Write(...)` through the stream's vtable slot 8 (`{delta, fn}`):
/// the vtable is loaded into `vt`, the method into `func`, `a0 = stream +
/// delta`.
pub(crate) fn wsm_save_stream_write(
    c: &mut Cpu,
    vt: usize,
    func: usize,
    ret: u32,
) -> crate::call::Flow {
    let stream = r32(c, S2);
    c.g[vt][0] = c.lw(stream);
    c.g[A0][0] = c.lh(r32(c, vt).wrapping_add(0x8));
    c.g[func][0] = c.lw(r32(c, vt).wrapping_add(0xc));
    c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
    let target = r32(c, func);
    call_ptr(c, target, ret)
}

/// `void WScriptMan_SaveState(WScriptMan* self, cPbStream* stream)`
/// (0x30bb10): write the script manager's state to a save stream: its first
/// 0x28c bytes, the collect set, both process lists, then the number of
/// live Luno object tables and, for each, its index and the table (with the
/// script cache entry of that index).
pub fn wscript_man_save_state(c: &mut Cpu) {
    use wsm_save as f;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = f::STATE_SIZE;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A0][0]; // self
        c.g[A1][0] = c.g[S3][0];
        c.g[S0][0] = sx(r32(c, S3).wrapping_add(f::COLLECT_SET));
        wsm_save_stream_write(c, V1, V0, 0x0030_bb54)?;

        // The collect sets (a loop over one).
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        WSM_COLLECT_SET_SAVE.call(c, 0x0030_bb64)?;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(f::COLLECT_SET_SIZE));

        // Both process lists.
        c.cycles += 14;
        let this = r32(c, S3);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(this.wrapping_add(f::LIST_A));
        c.g[A2][0] = c.g[S2][0];
        c.g[S0][0] = sx(this.wrapping_add(f::OBJS));
        WSM_SAVE_PROCESS_LIST.call(c, 0x0030_bb84)?;
        let this = r32(c, S3);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(this.wrapping_add(f::LIST_B));
        c.g[A2][0] = c.g[S2][0];
        WSM_SAVE_PROCESS_LIST.call(c, 0x0030_bb94)?;

        // count = number of live object tables (local at sp+0).
        c.sw(sp(c), 0);
        c.g[V1][0] = c.g[S0][0];
        c.g[A0][0] = f::OBJ_COUNT - 1;
        c.g[V0][0] = c.lw(r32(c, V1));
        loop {
            c.cycles += 3;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
            let empty = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if !empty {
                c.cycles += 3;
                c.g[V0][0] = c.lw(sp(c));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                c.sw(sp(c), c.g[V0][0]);
            }
            c.cycles += 2;
            if (c.g[A0][0] as i64) < 0 {
                break;
            }
            c.g[V0][0] = c.lw(r32(c, V1));
            if event_due(c) {
                deliver_event_here(c, 0x0030_bba8);
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw(r32(c, S2));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        wsm_save_stream_write(c, V0, V1, 0x0030_bbe4)?;

        // Each live table: its index j (local at sp+4), then the table.
        c.sw(sp(c).wrapping_add(4), 0);
        c.g[A0][0] = 0;
        loop {
            c.cycles += 6;
            c.g[V0][0] = sx(r32(c, A0) << 2);
            c.g[A1][0] = sx(sp(c).wrapping_add(4));
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            c.g[V1][0] = c.lw(r32(c, V0));
            c.g[A2][0] = 4;
            if c.g[V1][0] != 0 {
                c.cycles += 16;
                wsm_save_stream_write(c, V0, V1, 0x0030_bc1c)?;
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(f::WORLD));
                c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(4));
                call_ptr(c, WSM_CACHE_GET_DATA, 0x0030_bc2c)?;
                c.g[V1][0] = c.lw(sp(c).wrapping_add(4));
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = c.lw(r32(c, V1));
                call_ptr(c, WSM_LUNO_TABLE_WRITE, 0x0030_bc48)?;
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(sp(c).wrapping_add(4));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = ((c.g[A0][0] as i64) < f::OBJ_COUNT as i64) as u64;
            let more = c.g[V1][0] != 0;
            c.sw(sp(c).wrapping_add(4), c.g[V0][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_bbf0);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x0030a298 WScriptMan_RunObjScript1

/// Script manager fields: the world, the object whose script runs.
pub(crate) const WSR1_WORLD: u32 = 0x28c;

pub(crate) const WSR1_RUNNING: u32 = 0x290;

pub(crate) const WSR1_CALL_FUNC: Callee = Callee {
    addr: 0x0030_9c88,
    f: ee::s10::WScriptMan_CallFunc,
};

/// `void WScriptMan_RunObjScript1(WScriptMan* self, WObj* obj)` (0x30a298):
/// with `obj` as the running object, look up its script function 1 in the
/// world's script cache and call it when there is one (the handle on the
/// stack), then clear the running object.
pub fn wscript_man_run_obj_script1(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        c.cycles += 12;
        Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        let obj = c.g[V0][0];
        Addr(this).off(WSR1_RUNNING).sw(c, obj);
        c.g[A2][0] = 1;
        c.g[A0][0] = Addr(this).off(WSR1_WORLD).lw(c);
        call_ptr(c, 0x003a_6b78, 0x0030_a2bc)?; // World_GetObjectScriptFunc(world, obj, 1)
        c.g[V1][0] = !c.g[V0][0];
        let handle = c.g[V0][0];
        Addr(sp(c)).sw(c, handle);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            WSR1_CALL_FUNC.call(c, 0x0030_a2d4)?;
        }
        c.cycles += 5;
        Addr(r32(c, S0)).off(WSR1_RUNNING).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptProcessList_Count (0x30b228)

/// `cWScriptListNode`: the next process (`+0x18`).
pub(crate) const OC305_WSCRIPT_PROCESS_NEXT: u32 = 0x18;

/// `int WScriptProcessList_Count(WScriptProcessList* self)` (0x30b228): the
/// number of processes in the list (head at `+0`, linked through `+0x18`).
pub fn wscript_process_list_count(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(r32(c, A0)).lw(c);
    c.g[V0][0] = 0;
    if c.g[A0][0] != 0 {
        c.cycles += 1;
        loop {
            // The node and count live in a0/v0 (an event at the loop head
            // sees them there).
            c.cycles += 7;
            c.g[A0][0] = Addr(r32(c, A0)).off(OC305_WSCRIPT_PROCESS_NEXT).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_b238);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// WScriptMan_PostAirEvent (0x3098b0)

/// The script manager's op list: count at +0x40, then 12-byte ops from
/// +0x44 (`{ int op; int a; int b; }`).
pub(crate) const POST_AIR_OP_COUNT: u32 = 0x40;

pub(crate) const POST_AIR_OP_SIZE: u64 = 0xc;

/// The "air" op code.
pub(crate) const POST_AIR_OP: u64 = 0x7_0002;

/// `void WScriptMan_PostAirEvent(WScriptMan* self)` (0x3098b0): appends
/// op 0x70002 to the manager's op list. The 12-byte op is built in a stack
/// temporary whose other two words are never written (stack leftovers are
/// copied along), copied once on the stack, then into the list with
/// unaligned doubleword moves; the count is bumped. Leaf, frame 0x20.
pub fn wscript_man_post_air_event(c: &mut Cpu) {
    c.cycles += 25;
    let man = r32(c, A0);
    c.g[A1][0] = Addr(man.wrapping_add(POST_AIR_OP_COUNT)).lw(c);
    c.g[V1][0] = POST_AIR_OP_SIZE;
    let frame = Frame::open(c, 0x20);
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = POST_AIR_OP;
    let tmp = sp(c);
    Addr(tmp).sw(c, POST_AIR_OP);
    c.g[A2][0] = sx(man.wrapping_add(POST_AIR_OP_COUNT));
    // The op temporary, copied to sp+0x10.
    let head = Addr(tmp).read_u64_unaligned(c);
    let tail = Addr(tmp + 8).lw(c);
    c.g[V0][0] = head;
    c.g[A3][0] = tail;
    Addr(tmp + 0x10).write_u64_unaligned(c, head);
    Addr(tmp + 0x18).sw(c, tail);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    let slot = r32(c, V1).wrapping_add(r32(c, A2));
    c.g[V1][0] = sx(slot);
    // ... and into the list slot.
    let head = Addr(tmp + 0x10).read_u64_unaligned(c);
    let tail = Addr(tmp + 0x18).lw(c);
    c.g[V0][0] = head;
    c.g[A2][0] = tail;
    Addr(slot.wrapping_add(4)).write_u64_unaligned(c, head);
    Addr(slot.wrapping_add(0xc)).sw(c, tail);
    let count = c.g[A1][0];
    Addr(man.wrapping_add(POST_AIR_OP_COUNT)).sw(c, count);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// WScriptMan_CollectSetSave (0x30c6c8)

/// The saved object's vtable and the stream/collector interface slot.
pub(crate) const WSCRIPT_SET_SAVE_VTABLE: u32 = 0x00;

pub(crate) const WSCRIPT_SET_SAVE_SLOT: u32 = 0x08;

/// `int WScriptMan_CollectSetSave(void* stream, cObj* obj)` (0x30c6c8):
/// `obj->v01(stream, 0x10c)`, i.e. the collector's save entry with the
/// `SaveSet` tag. Frame 0x10 (ra +0).
pub fn wscript_man_collect_set_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0]; // obj
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0]; // stream
        c.g[A2][0] = 0x10c;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(WSCRIPT_SET_SAVE_VTABLE));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(WSCRIPT_SET_SAVE_SLOT));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(WSCRIPT_SET_SAVE_SLOT + 4));
        let save = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0030_c6f0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, save, 0x0030_c6f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_RunKillScript (0x30a5c0)

/// Script-manager fields: the running "kill" script handle and the script
/// object, plus the world's script-function lookup.
pub(crate) const RKS_RUNNING: u32 = 0x290;

pub(crate) const RKS_SCRIPT: u32 = 0x28c;

pub(crate) const RKS_GET_OBJECT_SCRIPT_FUNC: u32 = 0x003a_6b78;

pub(crate) const RKS_CALL_FUNC: Callee = Callee {
    addr: 0x0030_9c88,
    f: ee::s10::WScriptMan_CallFunc,
};

/// `void WScriptMan_RunKillScript(WScriptMan* self, int id)` (0x30a5c0): look
/// `id` up as an object script function (slot 4) of the manager's script
/// object and, when found (`!= -1`), call it with the handle; the handle is
/// published in `self+0x290` for the duration.
pub fn wscript_man_run_kill_script(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.sw(r32(c, S0).wrapping_add(RKS_RUNNING), c.g[V0][0]);
        c.g[A2][0] = 0x4;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(RKS_SCRIPT)).lw(c);
        call_ptr(c, RKS_GET_OBJECT_SCRIPT_FUNC, 0x0030_a5e4)?;
        c.g[V1][0] = !(0u64 | c.g[V0][0]);
        let not_found = c.g[V1][0] == 0;
        // Delay slot: the handle is the argument for the call.
        c.sw(sp(c), c.g[V0][0]);
        if !not_found {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            RKS_CALL_FUNC.call(c, 0x0030_a5fc)?;
        }
        c.cycles += 5;
        c.sw(r32(c, S0).wrapping_add(RKS_RUNNING), 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_MissionCommand
// ---------------------------------------------------------------------------

/// `WScriptMan` fields: `+0` the current context, `+0x1c4` the event queue,
/// `+0x2ac` the mission mode, `+0x2b4`/`+0x2b8` the process lists.
pub(crate) const MISSION_CMD_QUEUE: u32 = 0x1c4;

pub(crate) const MISSION_CMD_MODE: u32 = 0x2ac;

pub(crate) const MISSION_CMD_LIST_B: u32 = 0x2b4;

pub(crate) const MISSION_CMD_LIST_A: u32 = 0x2b8;

/// Mission object fields: `+0x20` its id, `+0x5c` its vtable.
pub(crate) const MISSION_CMD_ID: u32 = 0x20;

pub(crate) const MISSION_CMD_VT: u32 = 0x5c;

/// The two `switch (cmd)` jump tables (19 and 20 entries).
pub(crate) const MISSION_CMD_TABLE1: u32 = 0x0048_9930;

pub(crate) const MISSION_CMD_TABLE2: u32 = 0x0048_9980;

pub(crate) const MISSION_CMD_PROCESS: Callee = Callee {
    addr: 0x0030_a728,
    f: ee::s10::WScriptMan_ProcessCommand,
};

pub(crate) const MISSION_CMD_QUEUE_PUSH: Callee = Callee {
    addr: 0x0030_c7c0,
    f: ee::s10::WScriptMan_EventQueuePush,
};

pub(crate) const MISSION_CMD_CURRENT: Callee = Callee {
    addr: 0x0030_aeb8,
    f: ee::s10::WScriptMan_GetCurrentMission,
};

pub(crate) const MISSION_CMD_FIND: Callee = Callee {
    addr: 0x0030_b388,
    f: ee::s10::WScriptProcessList_FindMissionById,
};

pub(crate) const MISSION_CMD_TYPE2_ACTIVE: Callee = Callee {
    addr: 0x0030_b8c0,
    f: ee::s10::WScriptMan_IsMissionType2Active,
};

/// `int WScriptMan_MissionCommand(WScriptMan* m, int id, int cmd, int ctx)`
/// (0x30a868): mission command `cmd` on mission `id` (-1: the current
/// mission) under context `ctx` (-1: keep). Unless `cmd` is 15 or 19 it
/// needs the mission game mode and an active race. Commands 0-2/18 go to
/// ProcessCommand, 8/9/11 to the event queue (unless the queue is busy);
/// 3/4/5/17 call the mission's start/stop/abort/restart methods; 12-16/19
/// ask the BE mission interface (completed, unlocked, available, ...).
/// The context is put back; returns the answer (0/1).
pub fn wscript_man_mission_command(c: &mut Cpu) {
    guest(c, mission_command_body);
}

pub(crate) fn mission_command_body(c: &mut Cpu) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = 0xf;
    frame.save(c, S0, 0x70);
    frame.save(c, S1, 0x60);
    frame.save(c, S2, 0x50);
    frame.save(c, S3, 0x40);
    frame.save(c, S4, 0x30);
    frame.save(c, S5, 0x20);
    frame.save(c, S6, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = 0;
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A2][0];
    c.g[S3][0] = c.g[A0][0];
    c.g[S4][0] = c.g[A3][0];
    // s0 = forced (cmd 15 or 19).
    if c.g[S2][0] != 0xf {
        c.cycles += 3;
        c.g[V0][0] = 0x13;
        if c.g[S2][0] == 0x13 {
            c.cycles += 1;
            c.g[S0][0] = 1;
        }
    } else {
        c.cycles += 1;
        c.g[S0][0] = 1;
    }
    c.cycles += 7;
    call_ptr(c, 0x0014_dc80, 0x0030_a8c0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0030_a8cc)?;
    c.g[V0][0] = u64::MAX;
    'run: {
        if c.g[S0][0] == 0 {
            // Mission mode and a race running, or return 0.
            c.cycles += 5;
            c.g[A0][0] = 4;
            c.g[V1][0] = c.lb(0x0053_5c10);
            c.g[V0][0] = 0;
            if c.g[V1][0] != 4 {
                break 'run;
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x84));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xc));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x7c));
            c.g[V0][0] = 0;
            if c.g[A0][0] != 1 {
                break 'run;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        // Switch to the context (the old one in s6).
        c.cycles += 2;
        c.g[S6][0] = c.lw(r32(c, S3));
        if c.g[S4][0] != c.g[V0][0] {
            c.cycles += 1;
            let ctx = c.g[S4][0];
            c.sw(r32(c, S3), ctx);
        }
        mission_command_first(c)?;

        // obj = the current mission; s4 = the BE mission interface.
        c.cycles += 11;
        c.g[S5][0] = 0;
        MISSION_CMD_CURRENT.call(c, 0x0030_a994)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0030_a99c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x0014_dd58, 0x0030_a9a8)?;
        c.g[S4][0] = c.g[V0][0];
        let in_table = c.g[S2][0] < 0x14;
        c.g[V0][0] = 0x49_0000;
        let restore = if in_table {
            c.cycles += 6;
            let target = mission_command_table(c, MISSION_CMD_TABLE2);
            match target {
                0x0030_ac68 => true,
                0x0030_a9d0 => mission_command_method(c, 0x88)?,
                0x0030_aa3c => mission_command_method(c, 0x10)?,
                0x0030_aa70 => mission_command_method(c, 0x98)?,
                0x0030_aac4 => mission_command_method(c, 0xa0)?,
                0x0030_ab20 => {
                    mission_command_ask(c, 0x0015_3e28, 4, 0x0030_ab48, MissionAsk::NonZero)?
                }
                0x0030_ab50 => {
                    mission_command_ask(c, 0x0015_3dd0, 4, 0x0030_ab78, MissionAsk::NonZero)?
                }
                0x0030_ab80 => mission_command_available(c)?,
                0x0030_abd8 => {
                    mission_command_ask(c, 0x0015_3fd8, 5, 0x0030_ac00, MissionAsk::Last)?
                }
                0x0030_ac0c => {
                    mission_command_ask(c, 0x0015_3e80, 4, 0x0030_ac34, MissionAsk::NonZero)?
                }
                0x0030_ac3c => {
                    mission_command_ask(c, 0x0015_3d78, 3, 0x0030_ac64, MissionAsk::NonZero)?
                }
                _ => {
                    c.jump_out(target, 0x0030_a868, 0x0030_ac98);
                    return Ok(());
                }
            }
        } else {
            true
        };
        if restore {
            c.cycles += 1;
            let old = c.g[S6][0];
            c.sw(r32(c, S3), old);
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S5][0];
    }
    c.cycles += 10;
    for (reg, off) in [
        (S0, 0x70),
        (S1, 0x60),
        (S2, 0x50),
        (S3, 0x40),
        (S4, 0x30),
        (S5, 0x20),
        (S6, 0x10),
    ] {
        frame.restore(c, reg, off);
    }
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `a0 = table[cmd]` (the jump target, left in `a0` as the original does).
pub(crate) fn mission_command_table(c: &mut Cpu, table: u32) -> u32 {
    c.g[V1][0] = sx(r32(c, S2) << 2);
    c.g[V0][0] = sx(table);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = c.lw(r32(c, V1));
    r32(c, A0)
}

/// The first switch: ProcessCommand or the event queue.
pub(crate) fn mission_command_first(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    let in_table = c.g[S2][0] < 0x13;
    c.g[V0][0] = 0x49_0000;
    if in_table {
        c.cycles += 6;
        let target = mission_command_table(c, MISSION_CMD_TABLE1);
        match target {
            0x0030_a938 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                MISSION_CMD_PROCESS.call(c, 0x0030_a948)?;
                c.g[A0][0] = c.g[S3][0];
                return Ok(());
            }
            0x0030_a950 => {
                // Queue the command unless the queue is busy (1..9).
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x28));
                c.g[A0][0] = c.lw(r32(c, A0));
                c.g[A1][0] = 0;
                if c.g[A0][0] != 0 {
                    c.cycles += 1;
                    c.g[A1][0] = ((c.g[A0][0] as i64) < 10) as u64;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[A1][0] != 0 {
                    return Ok(());
                }
                c.cycles += 5;
                c.g[A3][0] = c.lw(r32(c, S3));
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_QUEUE));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                MISSION_CMD_QUEUE_PUSH.call(c, 0x0030_a988)?;
            }
            0x0030_a988 => {}
            _ => {
                c.jump_out(target, 0x0030_a868, 0x0030_ac98);
                return Err(crate::call::Unwound);
            }
        }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S3][0];
    Ok(())
}

/// Commands 3/4/5/17: find mission `id` (in list A, and for 3/17 then list
/// B; -1: the current one) and call its vtable method `slot`; 3 and 5 then
/// tell the sound system. Stores the context back itself (false: the
/// caller's tail does not).
pub(crate) fn mission_command_method(c: &mut Cpu, slot: u32) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    let current = c.g[S1][0] == c.g[V0][0];
    // The vtable already loaded into v0 (3/17's first lookup found it).
    let mut loaded = false;
    match slot {
        0x88 => {
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_A));
            if !current {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                MISSION_CMD_FIND.call(c, 0x0030_a9e8)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_VT));
                    loaded = true;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_B));
                    c.g[A1][0] = c.g[S1][0];
                    MISSION_CMD_FIND.call(c, 0x0030_aa00)?;
                    c.g[S0][0] = c.g[V0][0];
                }
            }
        }
        0xa0 => {
            if !current {
                c.cycles += 6;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_A));
                c.g[A1][0] = c.g[S1][0];
                MISSION_CMD_FIND.call(c, 0x0030_aae0)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_VT));
                    loaded = true;
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_B));
                    MISSION_CMD_FIND.call(c, 0x0030_aaf8)?;
                    c.g[S0][0] = c.g[V0][0];
                }
            }
        }
        _ => {
            c.g[A1][0] = c.g[S1][0];
            if !current {
                c.cycles += 3;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(MISSION_CMD_LIST_A));
                MISSION_CMD_FIND.call(
                    c,
                    if slot == 0x10 {
                        0x0030_aa54
                    } else {
                        0x0030_aa88
                    },
                )?;
                c.g[S0][0] = c.g[V0][0];
            }
        }
    }
    if !loaded {
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            let old = c.g[S6][0];
            c.sw(r32(c, S3), old);
            return Ok(false);
        }
        c.cycles += match slot {
            0x10 => 4,
            0x98 => 12,
            _ => 1,
        };
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_VT));
    }
    // obj->method(): a0 = obj + delta.
    let ret = match slot {
        0x88 => {
            c.cycles += 11;
            0x0030_aa20
        }
        0x98 => 0x0030_aaa8,
        _ => {
            if slot == 0xa0 {
                c.cycles += 2;
            }
            c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(slot));
            c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(slot + 4));
            c.cycles += 4;
            0x0030_ab18
        }
    };
    if slot == 0x88 || slot == 0x98 {
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(slot));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(slot + 4));
    }
    let method = r32(c, V1);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)?;
    match slot {
        0x88 => {
            // Sound: mission started (id).
            call_ptr(c, 0x0028_b180, 0x0030_aa28)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0029_d6e0, 0x0030_aa34)?;
        }
        0x98 => {
            // Sound: mission aborted.
            call_ptr(c, 0x0028_b180, 0x0030_aab0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0029_dbb0, 0x0030_aabc)?;
        }
        _ => {}
    }
    let old = c.g[S6][0];
    c.sw(r32(c, S3), old);
    Ok(false)
}

/// How a BE mission query's result becomes the answer.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum MissionAsk {
    /// `answer = result != 0`.
    NonZero,
    /// `answer = result == -1` (no next mission).
    Last,
}

/// `id` (-1: the current mission's id, if there is one); false (and v0
/// = -1, a0 = iface) when there is none.
pub(crate) fn mission_command_resolve_id(c: &mut Cpu) -> bool {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    c.g[A0][0] = c.g[S4][0];
    if c.g[S1][0] != c.g[V0][0] {
        return true;
    }
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_ID));
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S4][0];
    c.g[S1][0] != c.g[V0][0]
}

/// Commands 12/13/14/15/19: `answer = query(iface, id)`.
pub(crate) fn mission_command_ask(
    c: &mut Cpu,
    query: u32,
    cost: u64,
    ret: u32,
    how: MissionAsk,
) -> Result<bool, crate::call::Unwound> {
    if mission_command_resolve_id(c) {
        c.cycles += cost;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, query, ret)?;
        if how == MissionAsk::Last {
            c.g[V0][0] = !c.g[V0][0];
            c.g[S5][0] = (c.g[V0][0] < 1) as u64;
        } else {
            c.g[S5][0] = (c.g[V0][0] != 0) as u64;
        }
    }
    Ok(true)
}

/// Command 16: available (not in mode 1, no type-2 mission running).
pub(crate) fn mission_command_available(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    if c.g[S1][0] == c.g[V0][0] {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(MISSION_CMD_ID));
        }
        c.cycles += 2;
        if c.g[S1][0] == c.g[V0][0] {
            let old = c.g[S6][0];
            c.sw(r32(c, S3), old);
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(MISSION_CMD_MODE));
    c.cycles += 3;
    c.g[S0][0] = 1;
    if c.g[V0][0] == c.g[S0][0] {
        let old = c.g[S6][0];
        c.sw(r32(c, S3), old);
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    MISSION_CMD_TYPE2_ACTIVE.call(c, 0x0030_abb8)?;
    if c.g[V0][0] != 0 {
        let old = c.g[S6][0];
        c.sw(r32(c, S3), old);
        return Ok(false);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x0015_3ed8, 0x0030_abcc)?;
    c.g[S5][0] = 0;
    if c.g[V0][0] != 0 {
        c.g[S5][0] = c.g[S0][0];
    }
    Ok(true)
}

/// `void WScriptMan::CollectSetAddItems(pool, int* ids, int n)` (0x30c4a8): append
/// each valid (not -1) instance id of `ids` to the entry pool (`count` at +0,
/// ids from +0xc) and, depending on the race path (new race DB flag), either
/// respawn the instance straight away or only when `func_001538E8` says it
/// is not already collected.
pub fn wscript_man_collect_set_add_items(c: &mut Cpu) {
    guest(c, |c| {
        ssx3_ee_rt::prof::enter(c, 0x0030c4a8);
        'b_30c6a0: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0];
            call_ptr(c, 0x0014dc80, 0x0030c4dc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x0030c4e8)?;
            let t = (c.g[S0][0] as i64) <= 0;
            c.g[S5][0] = c.g[V0][0];
            if t {
                break 'b_30c6a0;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x530000;
                c.g[S3][0] = c.g[S0][0];
                c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                c.g[S4][0] = 0x490000;
                'l_30c500: loop {
                    'b_30c698: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[V0][0] = !(0u64 | c.g[V1][0]);
                        if c.g[V0][0] == 0u64 {
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                            break 'b_30c698;
                        } else {
                            'b_30c68c: {
                                'b_30c688: {
                                    'b_30c5f4: {
                                        c.cycles += 11;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                        call_ptr(c, 0x0014dc80, 0x0030c524)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x0014dd58, 0x0030c530)?;
                                        c.g[V1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x49));
                                        let t = c.g[V1][0] != 0u64;
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                                        if t {
                                            break 'b_30c5f4;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A2][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A3][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x001538e8, 0x0030c550)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                                            if t {
                                                break 'b_30c688;
                                            } else {
                                                'b_30c5a8: {
                                                    'b_30c584: {
                                                        c.cycles += 9;
                                                        c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                        c.g[V0][0] = c.g[A1][0] & 0xff;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[V1][0] as u32));
                                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                        if c.g[V1][0] != 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            break 'b_30c584;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[S0][0] = 0u64;
                                                            if t {
                                                                break 'b_30c5a8;
                                                            } else {
                                                                break 'b_30c584;
                                                            }
                                                        }
                                                    }
                                                    'b_30c5a4: {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = 0u64;
                                                        if t {
                                                            break 'b_30c5a4;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[A1][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            break 'b_30c5a4;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[S0][0] = c.g[A1][0];
                                                    break 'b_30c5a8;
                                                }
                                                'b_30c5d0: {
                                                    c.cycles += 3;
                                                    c.g[A2][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[A2][0] == 0u64;
                                                    c.g[A0][0] = 0x1c;
                                                    if t {
                                                        break 'b_30c5d0;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A1][0] = 0x3;
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0030c5cc)?;
                                                        c.g[A0][0] = 0x1c;
                                                        break 'b_30c5d0;
                                                    }
                                                }
                                                c.cycles += 9;
                                                c.g[A1][0] =
                                                    sx((c.g[S4][0] as u32).wrapping_sub(0x6948));
                                                c.g[A2][0] = 0x20000000;
                                                c.g[A3][0] = 0u64;
                                                Callee {
                                                    addr: 0x00317d70,
                                                    f: ee::s10::EA_MemAllocNamed,
                                                }
                                                .call(c, 0x0030c5e0)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S0][0];
                                                call_ptr(c, 0x003506d8, 0x0030c5ec)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_30c68c;
                                                } else {
                                                    break 'b_30c5f4;
                                                }
                                            }
                                        }
                                    }
                                    'b_30c644: {
                                        'b_30c620: {
                                            c.cycles += 9;
                                            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                            c.g[V0][0] = c.g[A1][0] & 0xff;
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            if c.g[V1][0] != 0u64 {
                                                c.g[A0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                                break 'b_30c620;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[S0][0] = 0u64;
                                                if t {
                                                    break 'b_30c644;
                                                } else {
                                                    break 'b_30c620;
                                                }
                                            }
                                        }
                                        'b_30c640: {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0u64;
                                            if t {
                                                break 'b_30c640;
                                            } else {
                                                c.cycles += 1;
                                                c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                                                break 'b_30c640;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[S0][0] = c.g[A1][0];
                                        break 'b_30c644;
                                    }
                                    'b_30c66c: {
                                        c.cycles += 3;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        if c.g[A2][0] == 0u64 {
                                            c.g[A0][0] = 0x1c;
                                            break 'b_30c66c;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                                            c.g[A1][0] = 0x3;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0030c668)?;
                                            c.g[A0][0] = 0x1c;
                                            break 'b_30c66c;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6948));
                                    c.g[A2][0] = 0x20000000;
                                    c.g[A3][0] = 0u64;
                                    Callee {
                                        addr: 0x00317d70,
                                        f: ee::s10::EA_MemAllocNamed,
                                    }
                                    .call(c, 0x0030c67c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x003506d8, 0x0030c688)?;
                                    break 'b_30c688;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                break 'b_30c68c;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                            break 'b_30c698;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S3][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030c500);
                        }
                        continue 'l_30c500;
                    } else {
                        break 'b_30c6a0;
                    }
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    })
}

/// `EA_MemFree(void*)`.
pub(crate) const OC429_MEM_FREE: u32 = 0x0031_7e98;

/// `if (a0 != 0) EA_MemFree(a0)`: the compiled null check on `a0` (2 cycles,
/// 3 more with the call), returning to `ret`.
#[inline(always)]
pub(crate) fn oc429_free_a0(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        call_ptr(c, OC429_MEM_FREE, ret)?;
    }
    Ok(())
}

/// `cAnimModel` (RiderModel) field offsets used by the destructor.
pub(crate) const RIDERMODEL_NUM_VARIANTS: u32 = 0x4;

pub(crate) const RIDERMODEL_NUM_GROUPS: u32 = 0x8;

pub(crate) const RIDERMODEL_GROUPS: u32 = 0xc;

pub(crate) const RIDERMODEL_TEX_COUNT: u32 = 0x12c;

pub(crate) const RIDERMODEL_TEX_LIST: u32 = 0x130;

/// `void RiderModel_dtor(RiderModel* self, int flags)` (0x30d540,
/// `cAnimModel::~cAnimModel`): free the model's allocations (`+0x38`, the four
/// slot blocks at `+0x44 + 8 * k`, per group/variant buffers, the texture
/// list entries via the graphics manager's vtable `+0x380` / `+0x190`, then
/// the plain buffers) and `__builtin_delete(self)` when `flags & 1`.
pub fn rider_model_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 22, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, 23, 0x30);
        frame.save(c, 30, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x38));
        let t = c.g[A0][0] == 0;
        c.g[S6][0] = c.lw(oc429_at(c.g[GP][0], 0x2a90));
        if !t {
            c.cycles += 2;
            call_ptr(c, OC429_MEM_FREE, 0x0030_d588)?;
        }

        'slots: {
            c.cycles += 4;
            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
            c.g[S0][0] &= 0x1;
            let t = c.g[V0][0] == 0;
            c.sw(oc429_at(c.g[SP][0], 4), c.g[S0][0]);
            if t {
                break 'slots;
            }
            c.cycles += 4;
            c.g[V0][0] = sx(oc429_at(c.g[S2][0], 0x44));
            c.g[S0][0] = 0;
            c.sw(sp(c), c.g[V0][0]);
            c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
            // k = 0..3 (slot blocks).
            loop {
                c.cycles += 6;
                c.g[V1][0] = c.lw(sp(c));
                c.g[S5][0] = c.g[V0][0];
                c.g[V0][0] = sx(oc429_at(c.g[V1][0], c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    call_ptr(c, OC429_MEM_FREE, 0x0030_d5c8)?;
                }
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_GROUPS));
                'groups: {
                    c.cycles += 4;
                    c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                    c.sw(oc429_at(c.g[SP][0], 8), c.g[S0][0]);
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S3][0] = 0;
                    if t {
                        break 'groups;
                    }
                    c.cycles += 3;
                    c.g[FP][0] = 0x58;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_VARIANTS));
                    // j = 0..num_groups.
                    loop {
                        'variants: {
                            c.cycles += 3;
                            c.g[S1][0] = 0;
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[S7][0] = sx(oc429_at(c.g[S3][0], 1));
                            if t {
                                break 'variants;
                            }
                            c.cycles += 3;
                            c.mult(0, S4, S3, FP);
                            c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                            // variant = 0..num_variants.
                            loop {
                                c.cycles += 8;
                                c.g[S0][0] = sx((c.g[S1][0] as u32) << 4);
                                c.g[V1][0] = sx(oc429_at(c.g[S4][0], c.g[V1][0] as u32));
                                c.g[V1][0] = sx(oc429_at(c.g[V1][0], c.g[S5][0] as u32));
                                c.g[V0][0] = c.lw(oc429_at(c.g[V1][0], 0x1c));
                                c.g[V0][0] = sx(oc429_at(c.g[S0][0], c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0xc));
                                if c.g[A0][0] != 0 {
                                    c.cycles += 3;
                                    call_ptr(c, OC429_MEM_FREE, 0x0030_d628)?;
                                }
                                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                                // gfx->vtbl[0x380](gfx, entry->f0)
                                c.cycles += 18;
                                c.g[A1][0] = c.lw(oc429_at(c.g[S6][0], 0x10d8));
                                c.g[V0][0] = sx(oc429_at(c.g[S4][0], c.g[V0][0] as u32));
                                c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S5][0] as u32));
                                c.g[A0][0] = c.lh(oc429_at(c.g[A1][0], 0x380));
                                c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x1c));
                                c.g[A2][0] = c.lw(oc429_at(c.g[A1][0], 0x384));
                                c.g[A0][0] = sx(oc429_at(c.g[S6][0], c.g[A0][0] as u32));
                                c.g[V1][0] = sx(oc429_at(c.g[S0][0], c.g[V1][0] as u32));
                                let target = c.g[A2][0] as u32;
                                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                call_ptr(c, target, 0x0030_d654)?;
                                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                                c.g[V0][0] = sx(oc429_at(c.g[S4][0], c.g[V0][0] as u32));
                                c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S5][0] as u32));
                                c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x1c));
                                c.g[V1][0] = sx(oc429_at(c.g[S0][0], c.g[V1][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                if c.g[A0][0] != 0 {
                                    c.cycles += 3;
                                    call_ptr(c, OC429_MEM_FREE, 0x0030_d67c)?;
                                }
                                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_VARIANTS));
                                c.cycles += 4;
                                c.g[S1][0] = sx(oc429_at(c.g[S1][0], 1));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0 {
                                    c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0030_d600);
                                    }
                                    continue;
                                }
                                break;
                            }
                        }
                        // Free the group's variant array.
                        c.cycles += 7;
                        c.mult(0, V0, S3, FP);
                        c.g[V1][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_GROUPS));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[V1][0] as u32));
                        c.g[V0][0] = sx(oc429_at(c.g[V0][0], c.g[S5][0] as u32));
                        c.g[A0][0] = c.lw(oc429_at(c.g[V0][0], 0x1c));
                        if c.g[A0][0] != 0 {
                            c.cycles += 3;
                            call_ptr(c, OC429_MEM_FREE, 0x0030_d6b4)?;
                        }
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_GROUPS));
                        c.cycles += 4;
                        c.g[S3][0] = c.g[S7][0];
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_NUM_VARIANTS));
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_d5e8);
                            }
                            continue;
                        }
                        break;
                    }
                }
                c.cycles += 4;
                c.g[S0][0] = c.lw(oc429_at(c.g[SP][0], 8));
                c.g[V0][0] = ((c.g[S0][0] as i64) < 4) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 3);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0030_d5a8);
                    }
                    continue;
                }
                break;
            }
        }

        // Texture list: release each entry with id >= 0 through the gfx
        // vtable `+0x190`.
        'tex: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_LIST));
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x30));
                break 'tex;
            }
            'texdone: {
                c.cycles += 3;
                c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_COUNT));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if t {
                    break 'texdone;
                }
                c.cycles += 2;
                c.g[S1][0] = 0;
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_LIST));
                    c.g[V0][0] = sx(oc429_at(c.g[S1][0], c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    if (c.g[A1][0] as i64) < 0 {
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_COUNT));
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S6][0], 0x10d8));
                        c.g[A0][0] = c.lh(oc429_at(c.g[V0][0], 0x190));
                        c.g[V1][0] = c.lw(oc429_at(c.g[V0][0], 0x194));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx(oc429_at(c.g[S6][0], c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0030_d720)?;
                        c.g[V0][0] = c.lw(oc429_at(c.g[S2][0], RIDERMODEL_TEX_COUNT));
                    }
                    c.cycles += 4;
                    c.g[S0][0] = sx(oc429_at(c.g[S0][0], 1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S1][0] = sx(oc429_at(c.g[S1][0], 0xc));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0030_d6f8);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], 0x30));
        }

        // The plain buffers, each freed if non-null.
        for (next, ret) in [
            (0x34u32, 0x0030_d748u32),
            (0x2c, 0x0030_d75c),
            (0xc, 0x0030_d770),
            (0x1c, 0x0030_d784),
            (0x3c, 0x0030_d798),
            (0x130, 0x0030_d7ac),
            (0x24, 0x0030_d7c0),
            (0x28, 0x0030_d7d4),
            (0x20, 0x0030_d7e8),
        ] {
            oc429_free_a0(c, ret)?;
            c.g[A0][0] = c.lw(oc429_at(c.g[S2][0], next));
        }
        // Last buffer (the next load is `flags & 1` from the frame).
        c.cycles += 2;
        let t = c.g[A0][0] == 0;
        c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 4));
        if !t {
            c.cycles += 3;
            call_ptr(c, OC429_MEM_FREE, 0x0030_d7fc)?;
            c.g[V0][0] = c.lw(oc429_at(c.g[SP][0], 4));
        }
        // flags & 1: delete the object itself.
        c.cycles += 2;
        let t = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xa0);
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, OC429_DELETE, 0x0030_d810)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, 22, 0x40);
        frame.restore(c, 23, 0x30);
        frame.restore(c, 30, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_Shutdown(self)` (0x309f18): destroys all Avalanche
/// objects (`0x2d9b40`), resets the collect sets (the table at +0x2c0, one
/// entry of 0x10c bytes, only one is walked), then runs the global shutdown
/// scripts of every world section of type 3: `self + 0x28c` is the world;
/// section `i` (`info->sections + i * 0xc`) of type 3 yields
/// `World_GetSectionShutdownFuncs` (`globals`) and their count, and each
/// function is run with `WScriptMan_CallFunc(self, fn)`. The script id slot
/// `self + 0` is set to -1 meanwhile (restored at the end) and the counters
/// at +0x290, +0x294, +0x298, +0x2a4, +0x2a8 are cleared before and after.
/// Frame 0x80 (s0-s6).
pub fn wscript_man_shutdown(c: &mut Cpu) {
    const RESET_COLLECT_SET: Callee = Callee {
        addr: 0x0030_c390,
        f: ee::s10::f_0030c390,
    };
    const CALL_FUNC: Callee = Callee {
        addr: 0x0030_9c88,
        f: ee::s10::f_00309c88,
    };
    const WORLD: u32 = 0x28c;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x2c0));
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        call_ptr(c, 0x002d_9b40, 0x0030_9f4c)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            RESET_COLLECT_SET.call(c, 0x0030_9f5c)?;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0030_9f50);
            }
        }
        // Clear the busy state, remembering the current script id.
        c.cycles += 16;
        let this = r32(c, S2);
        c.g[S6][0] = c.lw(this);
        c.g[V0][0] = u64::MAX;
        c.sw(this, c.g[V0][0]);
        c.g[S0][0] = 0;
        for off in [0x290, 0x294, 0x298, 0x2a4, 0x2a8] {
            c.sw(this.wrapping_add(off), 0);
        }
        c.g[V0][0] = c.lw(this.wrapping_add(WORLD));
        c.g[A0][0] = c.lw(r32(c, V0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
        call_ptr(c, 0x003a_9800, 0x0030_9f98)?;
        c.g[S4][0] = c.g[V0][0];
        let no_sections = (c.g[S4][0] as i64) <= 0;
        c.g[S5][0] = 0xc;
        if !no_sections {
            c.cycles += 1;
            c.g[A1][0] = c.lw(this.wrapping_add(WORLD));
            loop {
                'next: {
                    'skip: {
                        c.cycles += 8;
                        c.mult(0, A0, S0, S5);
                        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(4));
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        c.g[V0][0] = c.lh(r32(c, A0));
                        c.g[V0][0] ^= 3;
                        let not_type3 = c.g[V0][0] != 0;
                        c.g[S3][0] = sx(r32(c, S0).wrapping_add(1));
                        if not_type3 {
                            break 'skip;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[A1][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_6bd8, 0x0030_9fd4)?;
                        c.g[S1][0] = c.g[V0][0];
                        if c.g[S1][0] == 0 {
                            c.g[S0][0] = c.g[S3][0];
                            break 'next;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(this.wrapping_add(WORLD));
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a_6ba8, 0x0030_9fec)?;
                        let none = (c.g[V0][0] as i64) <= 0;
                        c.g[S0][0] = c.g[V0][0];
                        if none {
                            break 'skip;
                        }
                        c.cycles += 1;
                        loop {
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S2][0];
                            CALL_FUNC.call(c, 0x0030_a004)?;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                            let more = c.g[S0][0] != 0;
                            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                            if !more {
                                break 'skip;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0030_9ff8);
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[S0][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw(this.wrapping_add(WORLD));
                if event_due(c) {
                    deliver_event_here(c, 0x0030_9fa8);
                }
            }
        }
        c.cycles += 16;
        c.sw(this, c.g[S6][0]);
        for off in [0x290, 0x294, 0x298, 0x2a4, 0x2a8] {
            c.sw(this.wrapping_add(off), 0);
        }
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

/// `void WScriptMan_UnlinkProcess(WScriptMan* self, WScriptProcess* node)`
/// (0x3096f0): take `node` out of the first of the active, idle and add lists
/// that holds it.
pub fn wscript_man_unlink_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b8));
        call_ptr(c, 0x0030_b2d0, 0x0030_9710)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_b2d0, 0x0030_9724)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2bc));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0030_b2d0, 0x0030_9738)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_CallFuncOnInstance(WScriptMan* self, int id, u8* handle,
/// int value)` (0x309d20): run a world script function with `self->Inst`
/// (+0x290) set to `value`, through a fresh Luno register and table, then
/// restore the instance.
pub fn wscript_man_call_func_on_instance_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x30);
        c.g[S4][0] = Addr(r32(c, S0).wrapping_add(0x290)).lw(c);
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x28c)).lw(c);
        let value = c.g[A2][0];
        Addr(r32(c, S0).wrapping_add(0x290)).sw(c, value);
        call_ptr(c, 0x003a_6d18, 0x0030_9d54)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0022_6600, 0x0030_9d64)?;
        c.g[V0][0] = Addr(r32(c, S1)).lbu(c);
        c.g[S3][0] = sx(sp(c).wrapping_add(0x20));
        Addr(sp(c).wrapping_add(0x18)).sw(c, 0);
        c.g[A0][0] = c.g[S3][0];
        Addr(sp(c).wrapping_add(0x10)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        c.g[S2][0] = Addr(r32(c, V0).wrapping_add(0x3cc)).lw(c);
        call_ptr(c, 0x0022_24a8, 0x0030_9d88)?;
        c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0022_24b8, 0x0030_9da0)?;
        let inst_saved = c.g[S4][0];
        Addr(r32(c, S0).wrapping_add(0x290)).sw(c, inst_saved);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0022_5b90, 0x0030_9db0)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `bool WScriptMan_HasObjScript5(WScriptMan* self, int id)` (0x30a660): true
/// when the world's object script lookup (kind 5) finds an entry.
pub fn wscript_man_has_obj_script5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 5;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x28c)).lw(c);
        call_ptr(c, 0x003a_6b78, 0x0030_a674)?;
        c.g[V0][0] = !c.g[V0][0];
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// The lookup at the top of each `WScriptMan_ProcessCommand` case:
/// `obj = FindById(self + list, id)`, except that `id == -1` keeps the current
/// process (`A1`).
pub(crate) fn oc557_command_lookup(c: &mut Cpu, list: u32, ret_addr: u32) -> Flow {
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    if c.g[V1][0] != c.g[V0][0] {
        c.cycles += 4;
        c.g[A1][0] = c.g[V1][0];
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(list));
        call_ptr(c, 0x0030_b320, ret_addr)?;
        c.g[A1][0] = c.g[V0][0];
    }
    Ok(())
}

/// `void WScriptMan_ProcessCommand(WScriptMan* self, u32 id, int kind)`
/// (0x30a728): kind 1 starts an idle process, 0 stops an active one, 2 ends
/// its task (`NextTask`), 18 terminates it. `id == -1` acts on the current
/// process.
pub fn wscript_man_process_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let kind = c.g[A2][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x2a4)).lw(c);
        if kind == 1 {
            // Start: the idle list, then WScriptMan_StartProcess.
            oc557_command_lookup(c, 0x2b4, 0x0030_a798)?;
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0030_9750, 0x0030_a7ac)?;
                frame.restore(c, S0, 0x10);
            }
        } else {
            c.cycles += 3;
            if (kind as i64) < 2 {
                c.g[V0][0] = 1;
                c.cycles += 2;
                if kind == 0 {
                    // Stop: the active list, then WScriptMan_StopProcess.
                    oc557_command_lookup(c, 0x2b8, 0x0030_a7d0)?;
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        frame.restore(c, S0, 0x10);
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0030_9798, 0x0030_a7e4)?;
                        frame.restore(c, S0, 0x10);
                    }
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                }
            } else {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let two = kind == 2;
                c.g[V0][0] = 0x12;
                if two {
                    // End task: the active list, then WScriptProcess_NextTask.
                    oc557_command_lookup(c, 0x2b8, 0x0030_a808)?;
                    c.cycles += 2;
                    frame.restore(c, S0, 0x10);
                    if c.g[A1][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[A1][0];
                        call_ptr(c, 0x0030_7e10, 0x0030_a81c)?;
                        frame.restore(c, S0, 0x10);
                    }
                } else {
                    c.cycles += 2;
                    if kind == 0x12 {
                        // Terminate: the active list, then WScriptProcess_Terminate.
                        oc557_command_lookup(c, 0x2b8, 0x0030_a840)?;
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        if c.g[A1][0] != 0 {
                            c.cycles += 3;
                            c.g[A0][0] = c.g[A1][0];
                            call_ptr(c, 0x0030_7d78, 0x0030_a854)?;
                            frame.restore(c, S0, 0x10);
                        }
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of `WScriptMan_CallProcessFunc` (0x30ac98) and
/// `WScriptMan_CallTaskFunc` (0x30ada8): run a Luno function with `obj` as
/// the current process (or task) at `self + cur_off`, on the owner's local
/// table; a missing table is created for the call and destroyed after it.
/// `task` reads the table through the task's process (+0x3c).
pub(crate) fn oc557_run_script(
    c: &mut Cpu,
    cur_off: u32,
    task: bool,
    prologue: u64,
    rets: [u32; 8],
) -> Flow {
    let [r_setraw, r_pass, r_call, r_alloc, r_ctor, r_call2, r_dtor1, r_end] = rets;
    c.cycles += prologue;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, S1, 0x80);
    frame.save(c, S2, 0x70);
    frame.save(c, S3, 0x60);
    frame.save(c, S0, 0x90);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S4, 0x50);
    c.g[A0][0] = c.g[SP][0];
    frame.save_ra(c, 0x30);
    frame.save(c, S5, 0x40);
    c.g[S5][0] = Addr(r32(c, S3).wrapping_add(cur_off)).lw(c);
    let obj = c.g[A1][0];
    Addr(r32(c, S3).wrapping_add(cur_off)).sw(c, obj);
    c.g[A1][0] = c.g[A2][0];
    c.g[S4][0] = Addr(r32(c, S3).wrapping_add(0x290)).lw(c);
    c.g[A2][0] = 0;
    call_ptr(c, 0x0022_6600, r_setraw)?;
    c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
    Addr(sp(c).wrapping_add(0x18)).sw(c, 0);
    Addr(sp(c).wrapping_add(0x10)).sw(c, 0);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0022_24a8, r_pass)?;
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(cur_off)).lw(c);
    if task {
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
        c.g[S1][0] = Addr(r32(c, V0).wrapping_add(0x1c)).lw(c);
    } else {
        c.g[S1][0] = Addr(r32(c, V1).wrapping_add(0x1c)).lw(c);
    }
    let has_table = c.g[S1][0] != 0;
    c.g[V0][0] = sx(sp(c).wrapping_add(0x10));
    if has_table {
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A3][0] = c.g[V0][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, 0x0022_24b8, r_call)?;
        let inst = c.g[S4][0];
        Addr(r32(c, S3).wrapping_add(0x290)).sw(c, inst);
    } else {
        // No table: allocate one, run the function on it, destroy it.
        c.cycles += 18;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x6728));
        c.g[A0][0] = 4;
        call_ptr(c, 0x0031_7d70, r_alloc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x0022_4da0, r_ctor)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S0][0];
        call_ptr(c, 0x0022_24b8, r_call2)?;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S1][0] != 0 {
            c.g[A1][0] = 3;
            call_ptr(c, 0x0022_4df0, r_dtor1)?;
        }
        c.cycles += 1;
        let inst = c.g[S4][0];
        Addr(r32(c, S3).wrapping_add(0x290)).sw(c, inst);
    }
    c.cycles += 13;
    c.g[A0][0] = c.g[S0][0];
    let saved = c.g[S5][0];
    Addr(r32(c, S3).wrapping_add(cur_off)).sw(c, saved);
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x0022_5b90, r_end)?;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
    Ok(())
}

/// `void WScriptMan_CallProcessFunc(WScriptMan* self, WScriptProcess* obj,
/// u8* data)` (0x30ac98): `oc557_run_script` with the current process.
pub fn wscript_man_call_process_func(c: &mut Cpu) {
    guest(c, |c| {
        oc557_run_script(
            c,
            0x2a4,
            false,
            25,
            [
                0x0030_acd8,
                0x0030_acec,
                0x0030_ad14,
                0x0030_ad34,
                0x0030_ad40,
                0x0030_ad5c,
                0x0030_ad6c,
                0x0030_ad80,
            ],
        )
    })
}

/// `void WScriptMan_CallTaskFunc(WScriptMan* self, WScriptTask* obj, u8* data)`
/// (0x30ada8): `oc557_run_script` with the current task.
pub fn wscript_man_call_task_func(c: &mut Cpu) {
    guest(c, |c| {
        oc557_run_script(
            c,
            0x2a8,
            true,
            26,
            [
                0x0030_ade8,
                0x0030_adfc,
                0x0030_ae28,
                0x0030_ae48,
                0x0030_ae54,
                0x0030_ae70,
                0x0030_ae80,
                0x0030_ae94,
            ],
        )
    })
}

/// One `WScriptMan_GetEventArg` case reached through the table: `V1 =
/// *(self + 0x29c)`, `V0 = lw(V1 + off)`, then `>> shift` (32-bit,
/// sign-extended) and `& mask`.
pub(crate) fn oc557_event_field(c: &mut Cpu, cycles: u64, off: u32, shift: u32, mask: u64) -> u64 {
    c.cycles += cycles;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(off)).lw(c);
    if shift != 0 {
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> shift);
    }
    c.g[V0][0] & mask
}

/// `int WScriptMan_GetEventArg(WScriptMan* self, int which)` (0x30af08): the
/// field `which` (0..15) of the current trick's flag record, selected through
/// the game's table of bit-field extractors (one case per entry). An unknown
/// entry, `which >= 16` or no record gives 0.
pub fn wscript_man_get_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        c.g[A1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
        let no_record = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[A2][0] < 0x10) as u64;
        if !no_record {
            c.cycles += 2;
            let in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0x0049_0000;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x6630));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                let target = c.g[A0][0] as u32;
                let handled: Option<u64> = match target {
                    0x0030_af38 => Some(oc557_event_field(c, 4, 0, 0, 0x7)),
                    0x0030_af48 => Some(oc557_event_field(c, 5, 0, 3, 0xf)),
                    0x0030_af5c => Some(oc557_event_field(c, 5, 0, 7, 0x7)),
                    0x0030_af70 => Some(oc557_event_field(c, 5, 0, 10, 0x3)),
                    0x0030_af84 => Some(oc557_event_field(c, 5, 0, 12, 0xf)),
                    0x0030_af98 => {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2)).lhu(c);
                        Some(c.g[V0][0] & 0x7)
                    }
                    0x0030_afa8 => Some(oc557_event_field(c, 5, 0, 19, 0x7)),
                    0x0030_afbc => Some(oc557_event_field(c, 5, 0, 22, 0x3f)),
                    0x0030_afd0 => Some(oc557_event_field(c, 4, 0, 28, u64::MAX)),
                    0x0030_afe0 => Some(oc557_event_field(c, 4, 0x4, 0, 0x7)),
                    0x0030_aff0 => Some(oc557_event_field(c, 5, 0x4, 3, 0x7f)),
                    0x0030_b004 => Some(oc557_event_field(c, 5, 0x4, 10, 0x1)),
                    0x0030_b018 => Some(oc557_event_field(c, 5, 0x4, 18, 0x3)),
                    0x0030_b02c => {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x29c)).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x7)).lbu(c);
                        Some(c.g[V0][0] & 0x7)
                    }
                    0x0030_b03c => Some(oc557_event_field(c, 5, 0x4, 20, 0xf)),
                    _ => None,
                };
                match handled {
                    Some(v) => {
                        c.g[V0][0] = v;
                        ret(c);
                        return Ok(());
                    }
                    None if target == 0x0030_b050 => {}
                    None => {
                        c.jump_out(target, 0x0030_af08, 0x0030_b058);
                        return Ok(());
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        Ok(())
    })
}

/// `WScriptProcess* WScriptProcessList_FindByName(WScriptProcessList* self,
/// const char* name)` (0x30b350): find by the name's hash.
pub fn wscript_process_list_find_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7670, 0x0030_b368)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0030_b320, 0x0030_b374)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess* WScriptProcessList_FindMissionByName(self, name)`
/// (0x30b3f0): as `WScriptProcessList_FindByName`, over the mission lookup.
pub fn wscript_process_list_find_mission_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, 0x0031_7670, 0x0030_b408)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0030_b388, 0x0030_b414)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess* WScriptMan_FindMissionById(WScriptMan* self, int id)`
/// (0x30b458): the mission `id` in the active list, else the idle list, else
/// the add list.
pub fn wscript_man_find_mission_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b8));
        call_ptr(c, 0x0030_b388, 0x0030_b478)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2b4));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0030_b388, 0x0030_b48c)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x2bc));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0030_b388, 0x0030_b4a0)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_SetPendingRequestA(WScriptMan* self, int a, int b)`
/// (0x30b4c0): queue a request (flag at +4, values at +8 and +0xc).
pub fn wscript_man_set_pending_request_a(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let b = c.g[A2][0];
    Addr(this.wrapping_add(0xc)).sw(c, b);
    Addr(this.wrapping_add(0x4)).sw(c, 1);
    let a = c.g[A1][0];
    Addr(this.wrapping_add(0x8)).sw(c, a);
    ret(c);
}

/// `void WScriptMan_SetPendingRequestB(WScriptMan* self, int a, int b)`
/// (0x30b4d8): as request A, with the fields at +0x10..+0x18.
pub fn wscript_man_set_pending_request_b(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let b = c.g[A2][0];
    Addr(this.wrapping_add(0x18)).sw(c, b);
    Addr(this.wrapping_add(0x10)).sw(c, 1);
    let a = c.g[A1][0];
    Addr(this.wrapping_add(0x14)).sw(c, a);
    ret(c);
}

/// `void WScriptMan_SetPendingRequestC(WScriptMan* self, int a, int b)`
/// (0x30b4f0): as request A, with the fields at +0x1c..+0x24.
pub fn wscript_man_set_pending_request_c(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let b = c.g[A2][0];
    Addr(this.wrapping_add(0x24)).sw(c, b);
    Addr(this.wrapping_add(0x1c)).sw(c, 1);
    let a = c.g[A1][0];
    Addr(this.wrapping_add(0x20)).sw(c, a);
    ret(c);
}

/// `void WScriptMan_SetHudHeightTarget(WScriptMan* self, int target, Hud* hud)`
/// (0x30b508): set the height target flag and value, and copy the hud's
/// word at +0x78.
pub fn wscript_man_set_hud_height_target(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 1;
    let this = r32(c, A0);
    let target = c.g[A1][0];
    Addr(this.wrapping_add(0x30)).sw(c, target);
    Addr(this.wrapping_add(0x28)).sw(c, 1);
    c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x78)).lw(c);
    c.sw(r32(c, A0).wrapping_add(0x2c), c.g[V0][0]);
    ret(c);
}

/// The playback-manager calls shared by the mission request functions: the
/// world's playback manager (`*(gp - 0x848) + 0x84`) and replay.
pub(crate) const OC557_GP_WORLD: u32 = 0x848;

/// `void WScriptMan_RequestMissionAbort(WScriptMan* self)` (0x30b608): stop
/// the playback, clear its saved world state, and queue the abort command.
pub fn wscript_man_request_mission_abort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b628)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cbb0, 0x0030_b638)?;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x2a0)).lw(c);
        c.g[V0][0] = 0x6;
        c.sw(r32(c, S0).wrapping_add(0x2ac), c.g[V0][0]);
        c.sw(r32(c, S0).wrapping_add(0x2b0), c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_RequestMissionEnd(WScriptMan* self)` (0x30b758): stop and
/// clear the playback, reset and advance the replay, force the rider reset,
/// and queue the end command with the current mission id.
pub fn wscript_man_request_mission_end_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cdf8, 0x0030_b778)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x34)).lw(c);
        call_ptr(c, 0x0026_cbb0, 0x0030_b788)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f228, 0x0030_b798)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f7b8, 0x0030_b7a8)?;
        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(OC557_GP_WORLD)).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0012_35f8, 0x0030_b7c8)?;
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x2a0)).lw(c);
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = 0x4;
        c.sw(r32(c, S0).wrapping_add(0x2a0), c.g[V0][0]);
        c.sw(r32(c, S0).wrapping_add(0x2ac), c.g[V1][0]);
        let mission = c.g[A0][0];
        Addr(r32(c, S0).wrapping_add(0x2b0)).sw(c, mission);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool WScriptMan_IsProcessActive(WScriptMan* self, WScriptProcess* proc)`
/// (0x30b898): whether the process's id is in the active list.
pub fn wscript_man_is_process_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2b8));
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, A1).wrapping_add(0x20)).lw(c);
        call_ptr(c, 0x0030_b320, 0x0030_b8ac)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_StaticInit(int init, int prio)` (0x30c8d8): when `init` is
/// nonzero and `prio` is 0xFFFF, builds wscriptman.cpp's static colour
/// constants. The 56 four-float tables at 0x4fc080..0x4fc3f0 get their values
/// (the unit's own pair at 0x4fc400 and 0x4fc410 too), the key at 0x4a58c8
/// gets -1 as index and id, the pair at 0x4a58d0 is zeroed, and the 14 keys at
/// 0x4a58d8..0x4a5940 are numbered 0..13. Nothing else happens, and there is
/// no destruction branch.
///
/// This is a flat table of about 600 generated operations: the float constants
/// are loaded from the gp-relative pool and stored into the tables, and each
/// address is formed from the 0x500000 base in a temporary register. Those
/// temporaries, the saved registers spilled to the stack and the cycle count
/// are kept exactly as generated, so the body is transcribed in its generated
/// order rather than retyped as data.
pub fn wscript_man_static_init(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0030c8d8);
    'b_30d208: {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0xffff;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[A1][0] != c.g[V0][0];
        c.sq(30, c.g[SP][0] as u32);
        if t {
            break 'b_30d208;
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[T5][0] = 0x500000;
            if t {
                break 'b_30d208;
            } else {
                c.cycles += 574;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x335c));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3358));
                c.g[T6][0] = 0x500000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3354));
                c.g[T7][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3350));
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x3f50));
                c.swc1(8, (c.g[T5][0] as u32).wrapping_sub(0x3f70));
                c.g[S1][0] = 0x500000;
                c.swc1(9, (c.g[T6][0] as u32).wrapping_sub(0x3f60));
                c.g[S2][0] = 0x500000;
                c.swc1(10, (c.g[T7][0] as u32).wrapping_sub(0x3f50));
                c.g[S3][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x334c));
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x3f20));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3348));
                c.g[S5][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3344));
                c.g[S4][0] = 0x500000;
                c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[T3][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3340));
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x3f80));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x3f70));
                c.swc1(7, (c.g[S1][0] as u32).wrapping_sub(0x3f40));
                c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x3f60));
                c.swc1(4, (c.g[S2][0] as u32).wrapping_sub(0x3f30));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x3f40));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_sub(0x3f20));
                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3f30));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x333c));
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x3f10));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3338));
                c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x3f00));
                c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[T9][0] = 0x500000;
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 13);
                c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(0x3ef0));
                c.swc1(3, (c.g[S4][0] as u32).wrapping_sub(0x3f10));
                c.g[S6][0] = 0x500000;
                c.swc1(1, (c.g[T3][0] as u32).wrapping_sub(0x3f00));
                c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x3ec0));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_sub(0x3f80));
                c.g[S2][0] = 0x500000;
                c.swc1(13, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3ea0));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S3][0] = 0x500000;
                c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x3334));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x3e90));
                c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x3330));
                c.g[T8][0] = 0x500000;
                c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x500000;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x332c));
                c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3e80));
                c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x3328));
                c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub(0x3ee0));
                c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S7][0] = 0x500000;
                c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x500000;
                c.swc1(12, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3e70));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x3ed0));
                c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x3324));
                c.g[V1][0] = 0x500000;
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3320));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3e30));
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3e10));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x331c));
                c.g[S0][0] = 0x500000;
                c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e20));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e00));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3df0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3de0));
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x500000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x3318));
                c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3db0));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3314));
                c.g[A2][0] = 0x500000;
                c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3dd0));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3310));
                c.g[A2][0] = 0x500000;
                c.swc1(2, (c.g[T9][0] as u32).wrapping_sub(0x3ef0));
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3da0));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x330c));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3d80));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3308));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0xc));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3304));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3300));
                c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T8][0] as u32).wrapping_sub(0x3ee0));
                c.g[T2][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x3d20));
                c.swc1(0, (c.g[S7][0] as u32).wrapping_sub(0x3ed0));
                c.g[T2][0] = 0x500000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x32fc));
                c.swc1(0, (c.g[S6][0] as u32).wrapping_sub(0x3ec0));
                c.swc1(6, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3eb0));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3eb0));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x32f8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x3ea0));
                c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3d00));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x32f4));
                c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3cf0));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x32f0));
                c.g[S0][0] = 0x500000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x32ec));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3e90));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3e80));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
                c.g[T5][0] = 0x500000;
                c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x3eb0));
                c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[T6][0] = 0x500000;
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c70));
                c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                c.g[S1][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x3c90));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T6][0] = 0xffffffffffff0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
                c.g[S2][0] = 0x500000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
                c.g[T5][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3c60));
                c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S2][0] = 0x500000;
                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = 0x500000;
                c.swc1(1, (c.g[S3][0] as u32).wrapping_sub(0x3e10));
                c.g[T6][0] = c.g[T6][0] | 0xff00;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3e70));
                c.g[S3][0] = 0x500000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x3e60));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3e60));
                c.swc1(13, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x3c50));
                c.g[S0][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x32e8));
                c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3c40));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x32e4));
                c.g[S0][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x32e0));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3e50));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3e50));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x32dc));
                c.swc1(13, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x32d8));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3e40));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3e40));
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x32d4));
                c.swc1(13, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x32d0));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3e30));
                c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x32cc));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x3e20));
                c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3e60));
                c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.g[V1][0] = 0x500000;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x32c8));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x3e00));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3df0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3e50));
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3e40));
                c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.swc1(13, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x32c4));
                c.g[FP][0] = 0x500000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x3de0));
                c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[FP][0] = 0x500000;
                c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x32c0));
                c.g[A0][0] = c.g[V1][0];
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x32bc));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3dc0));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x3dd0));
                c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[FP][0];
                c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d90));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x3dc0));
                c.g[A1][0] = 0x500000;
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x3db0));
                c.g[A0][0] = 0x500000;
                c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x500000;
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3da0));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3dc0));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = 0x500000;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x3d90));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x500000;
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x32b8));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3d10));
                c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x32b4));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x3d80));
                c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x32b0));
                c.g[A2][0] = c.g[A0][0];
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x3d70));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3d70));
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = 0x500000;
                c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A0][0];
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3d50));
                c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V0][0];
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x32ac));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3d60));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x32a8));
                c.g[A2][0] = 0x500000;
                c.swc1(4, (c.g[S0][0] as u32).wrapping_sub(0x3d30));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x32a4));
                c.g[S0][0] = c.g[A2][0];
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3d40));
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x32a0));
                c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
                c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
                c.g[FP][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x329c));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3d30));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x3d60));
                c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = 0x500000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x3d50));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x3d40));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3d10));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = 0x500000;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A2][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3298));
                c.g[S0][0] = 0x500000;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3294));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3d60));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_sub(0x3d20));
                c.g[FP][0] = 0x500000;
                c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3d50));
                c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = 0x500000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3290));
                c.g[A3][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x3d10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d40));
                c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x500000;
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x328c));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3d30));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x3d00));
                c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x500000;
                c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3cd0));
                c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3288));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x3cf0));
                c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x3ce0));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x3c80));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x500000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V0][0] = c.g[FP][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3cb0));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A3][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ce0));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x500000;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3284));
                c.g[A3][0] = 0x500000;
                c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[S0][0] = c.g[T2][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3280));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3cc0));
                c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0x500000;
                c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
                c.g[A2][0] = 0x500000;
                c.swc1(0, (c.g[A3][0] as u32).wrapping_sub(0x3cd0));
                c.g[A0][0] = c.g[V1][0];
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x3cc0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ca0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A3][0] = c.g[A2][0];
                c.swc1(0, (c.g[FP][0] as u32).wrapping_sub(0x3cb0));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3c80));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[T2][0] = 0x500000;
                c.swc1(0, (c.g[V1][0] as u32).wrapping_sub(0x3ca0));
                c.g[S0][0] = 0x500000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
                c.g[FP][0] = 0x500000;
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x327c));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3ce0));
                c.swc1(0, (c.g[A1][0] as u32).wrapping_sub(0x3c90));
                c.g[V0][0] = 0x500000;
                c.swc1(3, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3cd0));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3278));
                c.g[V1][0] = 0x500000;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x3c80));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3cc0));
                c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = 0x500000;
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3274));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3cb0));
                c.swc1(0, (c.g[T2][0] as u32).wrapping_sub(0x3c70));
                c.g[A1][0] = 0x500000;
                c.swc1(5, (c.g[T7][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ca0));
                c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
                c.g[A2][0] = 0xffffffffffffffff;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3270));
                c.g[T2][0] = 0x2;
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x326c));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3c60));
                c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x500000;
                c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = 0x1;
                c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x27d8), c.g[A2][0]);
                c.g[T5][0] = 0x500000;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x27e8), 0u64);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x3c30));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3268));
                c.g[V0][0] = c.g[A2][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x27f0), c.g[A3][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
                c.sb((c.g[GP][0] as u32).wrapping_add(0x27f8), c.g[T2][0]);
                c.g[T6][0] = 0x500000;
                c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x27e8));
                c.g[V1][0] = c.g[A3][0] & 0xff;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3264));
                c.g[A2][0] = 0x500000;
                c.swc1(2, (c.g[S3][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = c.g[A2][0];
                c.swc1(1, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3c10));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3260));
                c.g[V1][0] = c.g[V1][0] | 0x100;
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x325c));
                c.g[A0][0] = c.g[T2][0] & 0xff;
                c.swc1(0, (c.g[T6][0] as u32).wrapping_sub(0x3c50));
                c.g[A0][0] = c.g[A0][0] | 0x200;
                c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[T2][0] = 0x3;
                c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = 0x5;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3258));
                c.g[S2][0] = 0x500000;
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3254));
                c.swc1(0, (c.g[S0][0] as u32).wrapping_sub(0x3c40));
                c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = 0x6;
                c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0x8));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3250));
                c.g[S3][0] = 0x500000;
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x324c));
                c.g[FP][0] = c.g[S3][0];
                c.swc1(0, (c.g[S2][0] as u32).wrapping_sub(0x3c30));
                c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x3c20));
                c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.g[S2][0] = 0x7;
                c.swc1(8, (c.g[T5][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[S3][0] as u32).wrapping_sub(0x3c20));
                c.g[T5][0] = 0x4;
                c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
                c.g[S3][0] = 0x8;
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x3c10));
                c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x27d8), c.g[V0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x27e8), c.g[A1][0]);
                c.g[V0][0] = 0xa;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x27f0), c.g[V1][0]);
                c.g[A1][0] = 0xd;
                c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
                c.g[V1][0] = 0xb;
                c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[FP][0] = 0x9;
                c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x27e4), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x27e0), 0u64);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x27f8), c.g[A0][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2800), c.g[T2][0]);
                c.g[A0][0] = 0xc;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2808), c.g[T5][0]);
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2810), c.g[T6][0]);
                c.g[T1][0] = c.g[T2][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2838), c.g[V0][0]);
                c.g[T3][0] = c.g[T5][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2840), c.g[V1][0]);
                c.g[T4][0] = c.g[T6][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2830), c.g[FP][0]);
                c.g[T1][0] = c.g[T1][0] | 0x300;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2848), c.g[A0][0]);
                c.g[T3][0] = c.g[T3][0] | 0x400;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2850), c.g[A1][0]);
                c.g[A0][0] = c.g[V1][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2828), c.g[S3][0]);
                c.g[A1][0] = c.g[V0][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2818), c.g[S0][0]);
                c.g[A2][0] = c.g[FP][0] & 0xff;
                c.sb((c.g[GP][0] as u32).wrapping_add(0x2820), c.g[S2][0]);
                c.g[A3][0] = c.g[S3][0] & 0xff;
                c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2848));
                c.g[A2][0] = c.g[A2][0] | 0x900;
                c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2850));
                c.g[T2][0] = c.g[S0][0] & 0xff;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2830), c.g[A2][0]);
                c.g[T0][0] = c.g[S2][0] & 0xff;
                c.g[A3][0] = c.g[A3][0] | 0x800;
                c.g[T2][0] = c.g[T2][0] | 0x600;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2828), c.g[A3][0]);
                c.g[A2][0] = 0x500000;
                c.g[T4][0] = c.g[T4][0] | 0x500;
                c.g[T0][0] = c.g[T0][0] | 0x700;
                c.g[A1][0] = c.g[A1][0] | 0xa00;
                c.g[A0][0] = c.g[A0][0] | 0xb00;
                c.g[V1][0] = c.g[V1][0] | 0xc00;
                c.g[V0][0] = c.g[V0][0] | 0xd00;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2818), c.g[T2][0]);
                c.g[A3][0] = c.g[A2][0];
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2800), c.g[T1][0]);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x3c00));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2808), c.g[T3][0]);
                c.g[T2][0] = 0x500000;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2810), c.g[T4][0]);
                c.g[T5][0] = c.g[T2][0];
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2820), c.g[T0][0]);
                c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_sub(0x3bf0));
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2838), c.g[A1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2840), c.g[A0][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2848), c.g[V1][0]);
                c.sw((c.g[GP][0] as u32).wrapping_add(0x2850), c.g[V0][0]);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3248));
                c.swc1(0, (c.g[A2][0] as u32).wrapping_sub(0x3c00));
                c.swc1(1, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3244));
                c.swc1(13, (c.g[T2][0] as u32).wrapping_sub(0x3bf0));
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3240));
                c.swc1(2, (c.g[T5][0] as u32).wrapping_add(0xc));
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[T5][0] as u32).wrapping_add(0x4));
                c.swc1(2, (c.g[T5][0] as u32).wrapping_add(0x8));
                break 'b_30d208;
            }
        }
    }
    c.cycles += 11;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lq(30, c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = t;
    return;
}

/// `int WScriptMan_DebugPrintf(WScriptMan* self, ...)` (0x30c898): the release
/// build's printf stub. It spills the variadic argument registers into its
/// frame (a1..a3 and t0..t3 as 64-bit words, f12..f18 as float varargs) and
/// returns `self`.
pub fn wscriptman_debugprintf(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    c.g[V0][0] = c.g[A0][0];
    for (i, reg) in [A1, A2, A3, T0, T1, T2, T3].into_iter().enumerate() {
        let v = c.g[reg][0];
        let a = frame.at(c, 0x48 + 8 * i as u32);
        c.sd(a, v);
    }
    for (i, f) in [12usize, 14, 16, 18].into_iter().enumerate() {
        let a = frame.at(c, 0x38 + 4 * i as u32);
        c.swc1(f, a);
    }
    frame.close(c);
}

/// The shape of the two event-queue forwards: a 0x10 frame holding `ra`, `a0`
/// moved to the queue at `self + 0x1c4`, one call returning to `ret`, then
/// `ra` restored and the frame closed.
pub(crate) fn wscriptman_event_queue_forward(c: &mut Cpu, callee: Callee, ret: u32) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c4));
    guest(c, |c| {
        callee.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_PopEvent(WScriptMan* self)` (0x30d348): pop the next
/// script event off the event queue (`WScriptMan_EventQueuePop`).
pub fn wscriptman_pop_event(c: &mut Cpu) {
    let callee = Callee {
        addr: 0x0030_c820,
        f: ee::s10::f_0030c820,
    };
    wscriptman_event_queue_forward(c, callee, 0x0030_d358);
}

/// `void WScriptMan_PeekEvent(WScriptMan* self)` (0x30d368): look at the next
/// script event without removing it (`WScriptMan_EventQueuePeek`).
pub fn wscriptman_peek_event(c: &mut Cpu) {
    let callee = Callee {
        addr: 0x0030_c870,
        f: ee::s10::f_0030c870,
    };
    wscriptman_event_queue_forward(c, callee, 0x0030_d378);
}

/// `void WScriptProcess_runScriptCode1`-style forward (0x30d3e0): run the
/// manager's script code (`self->Manager`, at `self + 0x10`).
pub fn wscriptprocess_run_callback(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(0x10),
        Target::Direct(Callee {
            addr: 0x0030_ac98,
            f: ee::s10::f_0030ac98,
        }),
        0x0030_d3f0,
    );
}

/// `WScriptTask_RunCallback` (0x30d400): run the task function of the
/// manager (`self + 0x10`), the same forward as the process one.
pub fn wscripttask_run_callback(c: &mut Cpu) {
    tail_wrapper(
        c,
        Some(0x10),
        Target::Direct(Callee {
            addr: 0x0030_ada8,
            f: ee::s10::f_0030ada8,
        }),
        0x0030_d410,
    );
}

/// The shape of the three `*_GlobalCtors` stubs: a 0x10 frame holding `ra`,
/// then `StaticInit(1, 0xffff)` (the static constructors of one unit), and
/// return what that returns.
pub(crate) fn static_ctor_stub(c: &mut Cpu, init: Callee, ret: u32) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A0][0] = 0x1;
    c.g[A1][0] = 0xffff;
    guest(c, |c| {
        init.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void WScriptMan_GlobalCtors(void)` (0x30d498).
pub fn wscriptman_global_ctors(c: &mut Cpu) {
    let init = Callee {
        addr: 0x0030_c8d8,
        f: ee::s10::f_0030c8d8,
    };
    static_ctor_stub(c, init, 0x0030_d4ac);
}

/// `EA_MemAllocNamed` (0x317d70).
pub(crate) const WSM_EA_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::f_00317d70,
};

/// `WScriptProcess_ctor` (0x307598).
pub(crate) const WSM_PROCESS_CTOR: Callee = Callee {
    addr: 0x0030_7598,
    f: ee::s10::f_00307598,
};

/// `WScriptProcessList_PushFront` (0x30b260).
pub(crate) const WSM_PUSH_FRONT: Callee = Callee {
    addr: 0x0030_b260,
    f: ee::s10::f_0030b260,
};

/// `WScriptProcessList_FindById` (0x30b320).
pub(crate) const WSM_FIND_BY_ID: Callee = Callee {
    addr: 0x0030_b320,
    f: ee::s10::f_0030b320,
};

/// `WScriptProcessList_Remove` (0x30b2d0).
pub(crate) const WSM_LIST_REMOVE: Callee = Callee {
    addr: 0x0030_b2d0,
    f: ee::s10::f_0030b2d0,
};

/// `void* WScriptMan_CreateProcess(man, sectionId, data, front)` (0x3094c8):
/// allocates a 0x60-byte process, constructs it, and pushes it on the add
/// list (`front` set) or the idle list. Returns the process. Frame 0x50.
pub fn wscript_man_create_process_2(c: &mut Cpu) {
    guest(c, wsm_create_body)
}

pub(crate) fn wsm_create_body(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 16, 0x40);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, 17, 0x30);
    frame.save(c, 18, 0x20);
    c.g[S1][0] = c.g[A2][0];
    frame.save(c, 19, 0x10);
    c.g[S2][0] = c.g[A3][0];
    c.g[S3][0] = c.g[A0][0];
    c.g[A1][0] = 0x0049_0000;
    frame.save_ra(c, 0);

    c.g[A0][0] = 0x60;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x66f8));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0u64;
    WSM_EA_ALLOC.call(c, 0x0030_9508)?;
    c.g[A2][0] = c.g[S0][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    WSM_PROCESS_CTOR.call(c, 0x0030_951c)?;
    let t = c.g[S2][0] == 0u64;
    c.g[S0][0] = c.g[V0][0];
    if !t {
        c.cycles += 5;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2bc));
        c.g[A1][0] = c.g[S0][0];
        WSM_PUSH_FRONT.call(c, 0x0030_9530)?;
    } else {
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2b4));
        c.g[A1][0] = c.g[S0][0];
        WSM_PUSH_FRONT.call(c, 0x0030_9544)?;
    }
    c.g[V0][0] = c.g[S0][0];

    c.cycles += 7;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 16, 0x40);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int WScriptMan_DeleteProcessById(man, id)` (0x309618): finds the process
/// with `id` on the active list, else the add list, else the idle list,
/// removes it, and calls its virtual destructor with flag 3 (`vt[+0x0c]`).
/// Frame 0x50.
pub fn wscript_man_delete_process_by_id(c: &mut Cpu) {
    guest(c, wsm_delete_body)
}

pub(crate) fn wsm_delete_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 17, 0x30);
    frame.save(c, 18, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, 19, 0x10);
    c.g[S2][0] = c.g[A1][0];
    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b8));
    frame.save(c, 16, 0x40);
    frame.save_ra(c, 0);

    'b_3096d8: {
        'b_309694: {
            'b_30966c: {
                c.g[A0][0] = c.g[S3][0];
                WSM_FIND_BY_ID.call(c, 0x0030_9644)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] != 0u64;
                c.g[A0][0] = c.g[S3][0];
                if t {
                    break 'b_30966c;
                }
                c.cycles += 7;
                c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2bc));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S3][0];
                WSM_FIND_BY_ID.call(c, 0x0030_9660)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A0][0] = c.g[S3][0];
                if t {
                    break 'b_309694;
                }
            }
            // Found on the active or the add list: remove it and destroy it.
            c.cycles += 10;
            c.g[A1][0] = c.g[S0][0];
            WSM_LIST_REMOVE.call(c, 0x0030_9674)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            c.g[A1][0] = 0x3;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let destroy = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, destroy, 0x0030_968c)?;
            frame.restore(c, 16, 0x40);
            break 'b_3096d8;
        }
        // Not on the active or add list: the idle list.
        c.cycles += 7;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        WSM_FIND_BY_ID.call(c, 0x0030_96a4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
            frame.restore(c, 16, 0x40);
            break 'b_3096d8;
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        WSM_LIST_REMOVE.call(c, 0x0030_96bc)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        c.g[A1][0] = 0x3;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let destroy = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, destroy, 0x0030_96d4)?;
        frame.restore(c, 16, 0x40);
    }

    c.cycles += 6;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int WScriptMan_RequestMissionPlayback(man)` (0x30b590): stops the
/// playback, resets the replay, and sets up the preview of the local mission
/// (`LocalMissionId`). On success it queues command 5 for that mission and
/// returns 1; otherwise 0. Frame 0x20: saves ra and s0.
pub fn wscript_man_request_mission_playback_2(c: &mut Cpu) {
    guest(c, wsm_preview_body)
}

pub(crate) fn wsm_preview_body(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    frame.save(c, 16, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];

    // SSXApp->Game->PlaybackMan->Stop(); SSXApp->Game->Replay->Reset().
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
    call_ptr(c, 0x0026_cdf8, 0x0030_b5b0)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
    call_ptr(c, 0x0026_f228, 0x0030_b5c0)?;

    // PlaybackMan->setupPreview(LocalMissionId).
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a0));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
    call_ptr(c, 0x0026_caf8, 0x0030_b5d4)?;

    if c.g[V0][0] != 0u64 {
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a0));
        c.cycles += 4;
        c.g[V0][0] = 0x5;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2ac), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2b0), c.g[V1][0]);
        c.g[V0][0] = 0x1;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0u64;
    }

    c.cycles += 4;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `|a - b|` on VU0: `Vec(a) - Vec(b)` stored at `dst` (and through `tmp`),
/// then the square root of the squared length into Q (`cfc2 v1, vi22`). The
/// distance helper `vu0Length(vu0Sub(a, b))` of the rider gate.
pub(crate) fn gate_distance(c: &mut Cpu, a: u32, b: u32, dst: u32, tmp: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, dst);
    c.lq(2, dst);
    c.sq(2, tmp);
    c.lqc2(3, tmp);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
}

/// `int WScriptMan_CheckRiderGate(int* rider, int id1, int id2, int id3)`
/// (0x30bfc0): looks up up to three script objects (`id`: low byte indexes
/// the table behind `gp+0x16c8`, `id >> 8` the entry's table, `-1` = none) and
/// decides whether the rider passes the gate between them. Returns 1 when the
/// rider's own first query (vtable +0x18) is set, else compares the rider's
/// distance to the first object (scaled by `gp-0x3364`, else 1.5) with the
/// distance to the other two: 0, or -1 when it is outside.
pub fn wscript_man_check_rider_gate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xa0);
        c.g[T0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[T1][0] = c.g[A0][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[S4][0] = 0;
        frame.save(c, S1, 0x80);
        c.g[V0][0] = !(0u64 | c.g[T0][0]);
        let none = c.g[V0][0] == 0;
        frame.save_ra(c, 0x40);
        'b_30c040: {
            if none {
                break 'b_30c040;
            }
            'b_30c03c: {
                'b_30c038: {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                    c.g[V0][0] = c.g[T0][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    let null = c.g[V1][0] == 0;
                    c.g[V0][0] = sx(r32(c, T0) >> 8);
                    if null {
                        break 'b_30c038;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if found {
                        break 'b_30c03c;
                    }
                    break 'b_30c038;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'b_30c03c;
            }
            c.cycles += 1;
            c.g[S3][0] = c.g[V0][0];
        }
        'b_30c09c: {
            c.cycles += 3;
            c.g[V0][0] = !(0u64 | c.g[A2][0]);
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            if none {
                break 'b_30c09c;
            }
            'b_30c098: {
                'b_30c074: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                        break 'b_30c074;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    break 'b_30c098;
                }
                'b_30c094: {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A2) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if found {
                        break 'b_30c094;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[S2][0] = c.g[A1][0];
        }
        'b_30c0ec: {
            c.cycles += 3;
            c.g[V0][0] = !(0u64 | c.g[A3][0]);
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            if none {
                break 'b_30c0ec;
            }
            'b_30c0e8: {
                'b_30c0e4: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A3][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    let null = c.g[V1][0] == 0;
                    c.g[V0][0] = sx(r32(c, A3) >> 8);
                    if null {
                        break 'b_30c0e4;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if found {
                        break 'b_30c0e8;
                    }
                    break 'b_30c0e4;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[S4][0] = c.g[V0][0];
        }
        'b_30c36c: {
            c.cycles += 16;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(r32(c, T1));
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x84));
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x6c0));
            c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x6c0));
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x18));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0030_c124)?;
            let gated = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if gated {
                break 'b_30c36c;
            }
            let spv = frame.at(c, 0);
            'b_30c200: {
                'b_30c1a4: {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x6c0));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x28));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x0030_c140)?;
                    let no_first = c.g[S3][0] == 0;
                    c.g[A0][0] = c.g[V0][0];
                    if no_first {
                        break 'b_30c1a4;
                    }
                    // Rider position vs the first object, scaled by gp-0x3364.
                    c.cycles += 23;
                    c.lq(2, r32(c, S3).wrapping_add(0x40));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3364));
                    c.sq(2, spv + 0x10);
                    let pos = r32(c, A0);
                    gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                    c.mtc1(3, 2);
                    c.lq(2, r32(c, S2).wrapping_add(0x40));
                    c.sq(2, spv + 0x10);
                    c.lq(3, r32(c, S3).wrapping_add(0x40));
                    break 'b_30c200;
                }
                // No first object: scale 1.5, second object instead.
                c.cycles += 23;
                c.lq(2, r32(c, S2).wrapping_add(0x40));
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 1);
                c.sq(2, spv + 0x10);
                let pos = r32(c, A0);
                gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                c.mtc1(3, 2);
                c.lq(2, r32(c, S4).wrapping_add(0x40));
                c.sq(2, spv + 0x10);
                c.lq(3, r32(c, S2).wrapping_add(0x40));
            }
            c.cycles += 22;
            c.sq(3, spv + 0x20);
            gate_distance(c, spv + 0x10, spv + 0x20, spv + 0x30, spv);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 1);
            c.c_le_s(2, 0);
            let inside = c.fcc();
            c.g[V0][0] = 0;
            if inside {
                break 'b_30c36c;
            }
            'b_30c310: {
                'b_30c2b8: {
                    c.cycles += 2;
                    let no_third = c.g[S4][0] == 0;
                    c.lq(2, r32(c, S2).wrapping_add(0x40));
                    if no_third {
                        break 'b_30c2b8;
                    }
                    c.cycles += 22;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3360));
                    c.sq(2, spv + 0x10);
                    let pos = r32(c, A0);
                    gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                    c.mtc1(3, 2);
                    c.lq(2, r32(c, S4).wrapping_add(0x40));
                    c.sq(2, spv + 0x10);
                    c.lq(3, r32(c, S2).wrapping_add(0x40));
                    break 'b_30c310;
                }
                c.cycles += 22;
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 1);
                c.sq(2, spv + 0x10);
                let pos = r32(c, A0);
                gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                c.mtc1(3, 2);
                c.lq(2, r32(c, S2).wrapping_add(0x40));
                c.sq(2, spv + 0x10);
                c.lq(3, r32(c, S3).wrapping_add(0x40));
            }
            c.cycles += 22;
            c.sq(3, spv + 0x20);
            gate_distance(c, spv + 0x10, spv + 0x20, spv + 0x30, spv);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 1);
            c.c_le_s(2, 0);
            let inside = c.fcc();
            c.g[V0][0] = 0;
            if !inside {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `WScriptMan_LoadState(self, stream)` (0x0030bd20): restore the script
/// manager from a replay frame. Reads the 652-byte header and the collect
/// pool (`CollectSetLoad`), moves every process of the active and add lists
/// back to the idle list (`PopFront` / `PushFront`), loads both process lists
/// from the stream, empties the global Luno tables, then reads `count`
/// `(id, table)` pairs (`cWScriptCache_GetData`, `cLunoTable_Read`) and clears
/// the current-call fields. Resumable body: the normal entry is hand-ported,
/// any other `pc` (resume after an unwound callee or event) runs the
/// generated body.
pub fn wscript_man_load_state(c: &mut Cpu) {
    if c.pc != 0x0030bd20 {
        return ee::s10::f_0030bd20__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0030bd20;
        loop {
            match pc {
                0x0030bd20 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[A2][0] = 0x28c;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[S1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S2][0] = 0u64;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0030bd6c);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030bd6c)?;
                    pc = 0x0030bd70;
                    continue;
                }
                0x0030bd70 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0030bd7c);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0030c700,
                        f: ee::s10::WScriptMan_CollectSetLoad,
                    }
                    .call(c, 0x0030bd7c)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10c));
                    if t {
                        if back_edge_unwinds(c, 0x0030bd70) {
                            return Ok(());
                        }
                        pc = 0x0030bd70;
                        continue;
                    }
                    pc = 0x0030bd88;
                    continue;
                }
                0x0030bd88 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b8));
                    c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2bc));
                    let t = true;
                    c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3cc));
                    if t {
                        pc = 0x0030bda4;
                        continue;
                    }
                    pc = 0x0030bd98;
                    continue;
                }
                0x0030bd98 => {
                    c.cycles += 3;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x0030bda4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
                    Callee {
                        addr: 0x0030b260,
                        f: ee::s10::WScriptProcessList_PushFront,
                    }
                    .call(c, 0x0030bda4)?;
                    pc = 0x0030bda4;
                    continue;
                }
                0x0030bda4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0030bdac);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0030b208,
                        f: ee::s10::WScriptProcessList_PopFront,
                    }
                    .call(c, 0x0030bdac)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x0030bd98) {
                            return Ok(());
                        }
                        pc = 0x0030bd98;
                        continue;
                    }
                    pc = 0x0030bdb4;
                    continue;
                }
                0x0030bdb4 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0030bdcc;
                        continue;
                    }
                    pc = 0x0030bdbc;
                    continue;
                }
                0x0030bdbc => {
                    c.cycles += 1;
                    pc = 0x0030bdc0;
                    continue;
                }
                0x0030bdc0 => {
                    c.cycles += 3;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x0030bdcc);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
                    Callee {
                        addr: 0x0030b260,
                        f: ee::s10::WScriptProcessList_PushFront,
                    }
                    .call(c, 0x0030bdcc)?;
                    pc = 0x0030bdcc;
                    continue;
                }
                0x0030bdcc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0030bdd4);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x0030b208,
                        f: ee::s10::WScriptProcessList_PopFront,
                    }
                    .call(c, 0x0030bdd4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x0030bdc0) {
                            return Ok(());
                        }
                        pc = 0x0030bdc0;
                        continue;
                    }
                    pc = 0x0030bddc;
                    continue;
                }
                0x0030bddc => {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0030bdf0);
                    c.g[S0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0030bee8,
                        f: ee::s10::WScriptMan_LoadProcessList,
                    }
                    .call(c, 0x0030bdf0)?;
                    c.g[S2][0] = 0x31;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0030be04);
                    c.g[A2][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0030bee8,
                        f: ee::s10::WScriptMan_LoadProcessList,
                    }
                    .call(c, 0x0030be04)?;
                    pc = 0x0030be08;
                    continue;
                }
                0x0030be08 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[A0][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x0030be1c;
                        continue;
                    }
                    pc = 0x0030be14;
                    continue;
                }
                0x0030be14 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0030be1c);
                    call_ptr(c, 0x00224f68, 0x0030be1c)?;
                    pc = 0x0030be1c;
                    continue;
                }
                0x0030be1c => {
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0030be08) {
                            return Ok(());
                        }
                        pc = 0x0030be08;
                        continue;
                    }
                    pc = 0x0030be24;
                    continue;
                }
                0x0030be24 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x4;
                    c.g[S0][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0030be44);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030be44)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);

                    if (c.g[V0][0] as i64) <= 0 {
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x2a8), 0u64);
                        pc = 0x0030beb4;
                        continue;
                    }
                    pc = 0x0030be50;
                    continue;
                }
                0x0030be50 => {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x0030be58;
                    continue;
                }
                0x0030be58 => {
                    c.cycles += 22;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0030be74);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030be74)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0030be84);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x003aca38, 0x0030be84)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x0030bea0);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x002259a8, 0x0030bea0)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        if back_edge_unwinds(c, 0x0030be58) {
                            return Ok(());
                        }
                        pc = 0x0030be58;
                        continue;
                    }
                    pc = 0x0030beb0;
                    continue;
                }
                0x0030beb0 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2a8), 0u64);
                    pc = 0x0030beb4;
                    continue;
                }
                0x0030beb4 => {
                    c.cycles += 13;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x290), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x294), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x298), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2a4), 0u64);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030bd20, 0x0030bee8);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::game::main::game::tests::{
        fake_streamer_section_count, PREV_STREAMER_SECTION_COUNT,
    };

    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as TEST_RA;

    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_6;
    pub(crate) use crate::game::be::beprepoststate::tests::belib;
    pub(crate) use crate::game::be::beprepoststate::tests::w;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_7;
    pub(crate) use crate::game::bx::appman::tests::ptr as ptr_2;
    pub(crate) use crate::game::bx::appman::tests::rng;
    pub(crate) use crate::game::bx::appman::tests::SEEDS;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base as alloc_base_2;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_6 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::EV;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8;

    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::globals;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::intersect::worldsphtree::tests::forward_setup;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_9;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;

    pub(crate) use crate::game::movie::movieplayer::tests::ret_slot;

    pub(crate) use crate::game::movie::movieplayer::tests::RET_ZERO;

    pub(crate) use crate::game::object::debouncenode::tests::base;
    pub(crate) use crate::game::object::debouncenode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::sound::bankmanager::tests::be_library;
    pub(crate) use crate::game::sound::bankmanager::tests::be_library_interface;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4;
    pub(crate) use crate::game::sound::soundsys::tests::base_9;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;

    pub(crate) use crate::game::world::world::tests::put_object_slots;
    pub(crate) use crate::game::world::world::tests::put_script_cache;
    pub(crate) use crate::game::world::world::tests::BASE;
    pub(crate) use crate::game::world::world::tests::SLOT1;
    pub(crate) use crate::game::world::world::tests::SLOT2;
    pub(crate) use crate::game::world::world::tests::TABLE;

    use ssx3_ee as ee;

    use ssx3_ee::s10;

    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn wscript_process_list_pop_front_matches() {
        check(
            s10::f_0030b208__generated,
            wscript_process_list_pop_front,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 4);
                scramble(c, &mut r);
                let list = ptr_2(&mut r, 0);
                let node = ptr_2(&mut r, 1);
                c.g[A0][0] = sx(list);
                let head = if seed % 2 == 0 { 0 } else { node };
                c.bus.write32(list, head);
                c.bus.write32(node + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_process_list_push_front_matches() {
        check(
            s10::f_0030b260__generated,
            wscript_process_list_push_front,
            SEEDS,
            &|c, seed| {
                let mut r = rng(seed, 5);
                scramble(c, &mut r);
                let list = ptr_2(&mut r, 0);
                let node = ptr_2(&mut r, 1);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = sx(node);
                c.bus
                    .write32(list, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn wscript_man_post_trigger_event_matches() {
        check_lohi(
            ssx3_ee::s10::f_003097e0,
            wscript_man_post_trigger_event,
            200,
            &|c, seed| {
                let mut r = Rng(0x2002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x40, seed % 4);
            },
        );
    }

    #[test]
    fn wscript_man_post_jump_event_matches() {
        check_lohi(
            ssx3_ee::s10::f_00309848,
            wscript_man_post_jump_event,
            200,
            &|c, seed| {
                let mut r = Rng(0x2003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x40, seed % 4);
            },
        );
    }

    #[test]
    fn wscript_man_post_trick_event_matches() {
        check_lohi(
            ssx3_ee::s10::f_00309918,
            wscript_man_post_trick_event,
            200,
            &|c, seed| {
                let mut r = Rng(0x2004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                c.bus.write32(DATA + 0x40, seed % 4);
                for i in 0..2 {
                    c.bus.write32(DATA + 0x200 + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn wscript_man_post_wipeout_event_matches() {
        check_lohi(
            ssx3_ee::s10::f_00309990,
            wscript_man_post_wipeout_event,
            200,
            &|c, seed| {
                let mut r = Rng(0x2005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x40, seed % 4);
            },
        );
    }

    #[test]
    fn wscript_man_post_reset_event_matches() {
        check_lohi(
            ssx3_ee::s10::f_003099f8,
            wscript_man_post_reset_event,
            200,
            &|c, seed| {
                let mut r = Rng(0x2006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x40, seed % 4);
            },
        );
    }

    #[test]
    fn wscript_process_list_ctor_matches() {
        check(
            ssx3_ee::s10::f_0030b058,
            wscript_process_list_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_process_list_push_front_2_matches() {
        check(
            ssx3_ee::s10::f_0030b2c0,
            wscript_process_list_push_front_2,
            150,
            &|c, seed| {
                let mut r = Rng(0x2008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 0x100 + 0x18, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_man_event_queue_push_matches() {
        check_lohi(
            ssx3_ee::s10::f_0030c7c0,
            wscript_man_event_queue_push,
            250,
            &|c, seed| {
                let mut r = Rng(0x2009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 4, (seed % 20) as u32);
            },
        );
    }

    #[test]
    fn wscript_man_is_scripted_instance_matches() {
        check(
            ee::s10::f_0030a310__generated,
            wscript_man_is_scripted_instance,
            SEEDS_2,
            &|c, seed| {
                let _r = base(c, seed, 0x30a3);
                let (man, cache, obj) = (DATA, DATA + 0x100, DATA + 0x140);
                let id = (seed % 3) as u8;
                let kind = if seed % 8 == 1 { 5 } else { 3 };
                put_script_cache(c, man, cache, obj, id, kind);
                let slot = TABLE + 12 * id as u32;
                c.bus
                    .write32(obj + 0xc, if seed % 8 == 0 { DATA + 0x700 } else { 0 });
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
            },
        );
    }

    #[test]
    fn wscript_man_has_obj_script1_matches() {
        check(
            ssx3_ee::s10::f_0030a270__generated,
            wscript_man_has_obj_script1,
            200,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_rider_config_matches() {
        check(
            ee::s10::f_0030b520__generated,
            wscript_man_set_pending_rider_config,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_b520, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x801) as u64; // deliberately unaligned
                for i in 0..3u32 {
                    c.bus.write32(DATA + 0x800 + 4 * i, r.next_u32());
                }
                c.bus.write32(DATA + 0x34, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_man_has_kill_script_matches() {
        check(
            s10::f_0030a598__generated,
            wscript_man_has_kill_script,
            120,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn wscript_man_jump_to_task_matches() {
        check(
            s10::f_0030a6d8__generated,
            wscript_man_jump_to_task,
            120,
            &|c, seed| {
                machine(c, seed, 0x30a6d8, 1);
                c.g[A1][0] = 0;
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x100);
                c.g[A0][0] = sx(OBJ);
                // No process / a process with no commands (the callee loops over
                // the command count at +0x4).
                let proc = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(OBJ + 0x2a4, proc);
            },
        );
    }

    #[test]
    fn wscript_man_goto_task_matches() {
        check(
            s10::f_0030a700__generated,
            wscript_man_goto_task,
            120,
            &|c, seed| {
                machine(c, seed, 0x30a700, 1);
                c.g[A1][0] = 0;
                clear(c, OBJ, 0x40);
                clear(c, CHILD, 0x100);
                c.g[A0][0] = sx(OBJ);
                let proc = if seed % 3 == 0 { 0 } else { CHILD };
                c.bus.write32(OBJ + 0x2a4, proc);
            },
        );
    }

    #[test]
    fn wscript_man_cmd_queue_ctor_matches() {
        check(
            s10::f_0030c738__generated,
            wscript_man_cmd_queue_ctor,
            120,
            &|c, seed| {
                machine(c, seed, 0x30c738, 1);
                clear(c, OBJ, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.bus.write32(OBJ, seed as u32 & 0xff);
            },
        );
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
        check(
            ee::s10::f_0030b320__generated,
            wscript_process_list_find_by_id,
            SEEDS_3,
            &setup,
        );
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
        check(
            ee::s10::f_0030b428__generated,
            wscript_process_list_contains,
            SEEDS_3,
            &setup,
        );
    }

    #[test]
    fn wscript_man_reset_command_queue_matches() {
        check(
            ee::s10::f_0030c760__generated,
            wscript_man_reset_command_queue,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30c760, 0);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x40u32 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn wscript_man_event_queue_reset_matches() {
        check(
            ee::s10::f_0030c790__generated,
            wscript_man_event_queue_reset,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30c790, 0);
                c.g[A0][0] = DATA as u64;
                for i in 0..0x40u32 {
                    c.bus.write32(DATA + 4 * i, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------- destructors (batch)

    #[test]
    fn wscript_man_sub_obj_dtor_d420_matches() {
        dtor_case(
            ee::s10::f_0030d420__generated,
            wscript_man_sub_obj_dtor_d420,
            0x30d420,
        );
    }

    #[test]
    fn wscript_man_collect_set_load_matches() {
        check(
            ee::s10::f_0030c700__generated,
            wscript_man_collect_set_load,
            120,
            &|c, seed| {
                forward_setup(c, seed, 0x30c7_00, 0x10);
            },
        );
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
                c.bus
                    .write32(crate::difftest::GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
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
        check(
            ee::s10::f_00309750__generated,
            w_script_man_start_process,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3097_50, 1);
                let man = DATA + 0x100;
                let process = DATA + 0x300;
                c.g[A0][0] = sx(man);
                c.g[A1][0] = sx(process);
                c.bus.write32(man + 0x2a4, ptr(&mut r));
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(process + 0x5c, vt);
            },
        );
    }

    #[test]
    fn w_script_man_stop_process_matches() {
        check(
            ee::s10::f_00309798__generated,
            w_script_man_stop_process,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3097_98, 1);
                let man = DATA + 0x100;
                let process = DATA + 0x300;
                c.g[A0][0] = sx(man);
                c.g[A1][0] = sx(process);
                c.bus.write32(man + 0x2a4, ptr(&mut r));
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(process + 0x5c, vt);
            },
        );
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
                c.bus
                    .write32(this + 0x1c, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
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
                c.bus
                    .write32(this + 0x2a4, if seed % 2 == 0 { 0 } else { mission });
                // The mission's vtable slot: `lh(+0x18)` delta, `lw(+0x1c)` fn.
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(mission + 0x5c, vt);
                c.bus
                    .write32(LOG_RESULTS, if seed % 4 < 2 { 0x1234 } else { 0 });
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
        check(
            ee::s10::f_00309aa0__generated,
            wscript_man_deactivate_process,
            SEEDS_7,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn w_script_man_request_mission_restart_matches() {
        check(
            ee::s10::f_0030b6f8__generated,
            w_script_man_request_mission_restart,
            32,
            &|c, seed| {
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
            },
        );
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
        check(
            ee::s11::f_003506d8__generated,
            dead_node_ctor,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x3506d8, 4);
                be_library_2(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.g[A2][0] = (DATA + 0x500) as u64;
                w(c, this + 0x18, DATA + 0x600);
            },
        );
    }

    #[test]
    fn wscript_man_define_collect_set_matches() {
        check(
            ee::s10::f_0030b928__generated,
            wscript_man_define_collect_set,
            SEEDS_9,
            &|c, seed| {
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
                c.g[A1][0] = if seed % 3 == 0 {
                    id as u64
                } else {
                    (r.next_u32() % 8) as u64
                };
                c.g[A2][0] = r.next_u32() as u64;
                w(c, slotaddr, id);
                w(c, slotaddr + 4, 0);
            },
        );
    }

    #[test]
    fn rider_model_alloc_model_refs_matches() {
        check_lohi(
            ee::s10::f_0030d840__generated,
            rider_model_alloc_model_refs,
            SEEDS_9,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30d840, 2);
                be_library_3(c);
                heap_init(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
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
        check(
            ee::s10::f_0030d4b8__generated,
            rider_model_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x30d4b8, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
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
        check(
            ssx3_ee::s10::f_0030b658__generated,
            wscript_man_request_mission_cancel,
            8,
            &|c, seed| {
                let (mut r, obj) = mission_world(c, seed, 0x0030_b658);
                w(c, obj + 0x2a0, r.next_u32());
                w(c, obj + 0x2ac, r.next_u32());
                w(c, obj + 0x2b0, r.next_u32());
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn wscript_man_deactivate_all_matches() {
        check(
            ssx3_ee::s10::f_0030b7f8__generated,
            wscript_man_deactivate_all,
            8,
            &|c, seed| {
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
                w(
                    c,
                    man + 0x2a0,
                    if seed % 2 == 0 {
                        0xffff_ffff
                    } else {
                        (1 + seed % 4) as u32
                    },
                );
                c.pc = 0x0030_b7f8;
                c.g[A0][0] = sx(man);
            },
        );
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
        diff(
            ee::s10::f_00309ba8__generated,
            w_script_man_get_hud_target_height,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x30_9ba8);
                c.g[A0][0] = sx(DATA);
                w(
                    c,
                    DATA + 0x28,
                    if seed % 3 == 0 { 0 } else { DATA + 0x2000 },
                );
                w(c, DATA + 0x2c, r.next_u32());
                w(c, DATA + 0x30, r.next_u32());
                let t = DATA + 0x2000;
                c.f[12] = uf(&mut r, 100.0);
                w(c, t + 0x28c, r.next_u32());
                w(c, t + 0x6c0, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn w_script_man_collect_item_matches() {
        diff(
            ee::s10::f_0030b9a0__generated,
            w_script_man_collect_item,
            30,
            &|c, seed| {
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
            },
        );
    }

    use crate::difftest::fakes::{self};

    pub(crate) use crate::game::animation::animsequencer::tests::asl_install_fakes;
    pub(crate) use crate::game::animation::animsequencer::tests::ASL_CTRL;
    pub(crate) use crate::game::animation::animsequencer::tests::ASL_MAGIC;

    /// WScriptMan_CallFunc: a handle naming section 0..3 and script id
    /// 0..3 resolved through a world script cache (table entries of kind 3)
    /// to a Luno script whose function returns at once; the real
    /// LunoVM_PassThrough / LunoVM_Call / LunoVMRegister_dtor run on the pool
    /// allocator and a dlmalloc arena (as `luno_vm_call_matches` sets them up).
    #[test]
    fn wscript_man_call_func_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x20_0000;
        const HANDLE: u32 = SELF + 0x1000;
        const WORLD: u32 = SELF + 0x1100;
        const CACHE: u32 = SELF + 0x1200;
        const ENTRIES: u32 = SELF + 0x1300;
        const FUNCS: u32 = SELF + 0x1400;
        const SCRIPT: u32 = SELF + 0x2000;
        const TABLES: u32 = SELF + 0x3000;
        check(
            ee::s10::f_00309c88__generated,
            wscript_man_call_func,
            120,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x30_9c88);
                c.pc = 0x0030_9c88;
                let section = seed % 4;
                let id = (seed / 4) % 4;
                c.bus.write32(HANDLE, id << 8 | section);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(HANDLE);
                // World_GetSectionScriptFunc -> tWScriptCache_GetTableEntry.
                c.bus.write32(SELF + WSM_WORLD, WORLD);
                c.bus.write32(WORLD + 4, CACHE);
                c.bus.write32(CACHE + 4, ENTRIES);
                let entry = ENTRIES + 0xc * section;
                c.bus.write16(entry, 3);
                c.bus.write32(entry + 8, FUNCS);
                c.bus.write32(FUNCS + 0x3c, FUNCS + 0x40);
                c.bus.write32(FUNCS + 0x40 + 4 * id, SCRIPT);
                // The script: code offset 0x10, start pc 0, n registers, 0 locals.
                c.bus.write32(SCRIPT + 4, 0x10);
                c.bus.write32(SCRIPT + 0x14, 0);
                c.bus.write32(SCRIPT + 0x18, (seed / 16) % 4);
                c.bus.write32(SCRIPT + 0x1c, 0);
                // Per-section tables: a cLunoValue whose word 0 is a table
                // (refcount at +8).
                for i in 0..4u32 {
                    let value = TABLES + 0x40 * i;
                    let table = TABLES + 0x400 + 0x40 * i;
                    c.bus.write32(SELF + WSM_SECTION_TABLES + 4 * i, value);
                    c.bus.write32(value, table);
                    c.bus.write32(table + 8, 100);
                }
                // LunoVM_Call's default value: a plain word.
                let d = 0x004c_9098;
                c.bus.write32(d, r.next_u32());
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, 2);
            },
        );
    }

    /// WScriptMan_FocusLeave: no controller / a controller whose v20 is a
    /// fake (non-zero: v22) or the real zero leaf 0x1406c0, kinds 6 and
    /// other; the real HasObjScript3/RunObjScript3 and script cache lookup
    /// run. The "has script" path is reached by pointing the object's cache
    /// entry id at `man + 0x290`: RunObjScript3 stores the object there
    /// before its own lookup, which then finds -1 and skips the script call.
    #[test]
    fn wscript_man_focus_leave_matches() {
        const MAN: u32 = DATA;
        const TABLE: u32 = DATA + 0x400;
        const CACHE: u32 = DATA + 0xc00;
        const SECTION: u32 = DATA + 0x1200;
        const NO_SCRIPT: u32 = DATA + 0x1400;
        const OBJ: u32 = DATA + 0x800;
        const OWNER: u32 = DATA + 0x900;
        const CTL: u32 = DATA + 0xa00;
        const OWNER_CTL: u32 = DATA + 0xb00;
        const VTABLE: u32 = DATA + 0x1800;
        const FUNCS: u32 = DATA + 0x1_0000;
        const ZERO_LEAF: u32 = 0x0014_06c0;
        // Function entry `id` of slot 3.
        let func_at = |id: u32| FUNCS + (id >> 8) * 0x18 + 12;
        check(
            ee::s10::f_0030a460__generated,
            wscript_man_focus_leave,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_a460, 2);
                c.pc = 0x0030_a460;
                c.g[A0][0] = sx(MAN);
                c.g[A1][0] = sx(OBJ);
                // Script cache: man -> table -> cache -> section -> functions.
                c.bus.write32(MAN + 0x28c, TABLE);
                c.bus.write32(TABLE + 4, DATA + 0x600);
                c.bus.write32(DATA + 0x604, CACHE);
                c.bus.write32(CACHE + 8, SECTION);
                c.bus.write32(SECTION + 0x1c, FUNCS);
                c.bus.write32(NO_SCRIPT + 8, u32::MAX);
                c.bus.write32(MAN + 0x290, 0x100);
                c.bus.write32(func_at(0x100), r.next_u32() & 0x7fff_ffff);
                c.bus.write32(func_at(OBJ), u32::MAX);
                c.bus.write32(func_at(OWNER), u32::MAX);
                let has_script = seed % 2 == 0;
                for o in [OBJ, OWNER] {
                    c.bus.write8(o + 0x78, 0);
                    let entry = if has_script {
                        MAN + 0x290 - 8
                    } else {
                        NO_SCRIPT
                    };
                    c.bus.write32(o + 0x88, entry);
                }
                // Controllers.
                let has_ctl = seed % 6 >= 2;
                c.bus
                    .write32(OBJ + OC264_WOBJ_CTL, if has_ctl { CTL } else { 0 });
                c.bus.write32(OWNER + OC264_WOBJ_CTL, OWNER_CTL);
                for ctl in [CTL, OWNER_CTL] {
                    c.bus.write32(ctl + OC264_WCTL_VTABLE, VTABLE);
                }
                let kind = if seed % 6 == 4 { 6 } else { r.next_u32() % 6 };
                c.bus.write16(CTL + OC264_WCTL_KIND, kind as u16);
                c.bus.write32(CTL + OC264_WCTL_OWNER, OWNER);
                let v20 = if seed % 3 == 0 { FAKE } else { ZERO_LEAF };
                for (slot, f) in [
                    (OC264_WCTL_V20, v20),
                    (OC264_WCTL_V22, FAKE + 0x10),
                    (OC264_WCTL_V35, FAKE + 0x20),
                ] {
                    c.bus.write16(VTABLE + slot, (r.next_u32() % 0x20) as u16);
                    c.bus.write32(VTABLE + slot + 4, f);
                }
            },
        );
    }

    // ------------------------------------------------ WScriptMan_OnObjectTrigger

    pub(crate) const WSM_CTRL: u32 = DATA + 0x5_b000;

    pub(crate) const WSM_MAGIC: u32 = 0x30a0_60a7;

    /// The function lookup: a logged fake returning the id the fixture
    /// picks at `WSM_CTRL + 4` (-1 = none).
    pub(crate) fn wsm_fake_find_func(c: &mut Cpu) {
        if c.bus.read32(WSM_CTRL) != WSM_MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(wsm_fake_find_func),
                ee::s13::f_003a6b78__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, WSM_FIND_FUNC, ra);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(WSM_CTRL + 4));
        }
    }

    macro_rules! wsm_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(WSM_CTRL) != WSM_MAGIC {
                    return crate::difftest::fakes::fall_back(
                        c,
                        crate::difftest::fakes::prev_cell($name),
                        $gen,
                    );
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    wsm_gated!(wsm_fake_post, 0x0030_97e0, ee::s10::f_003097e0__generated);

    wsm_gated!(
        wsm_fake_value,
        WSM_VALUE_FROM_FUNC,
        ee::s05::f_00226600__generated
    );

    wsm_gated!(
        wsm_fake_call_init,
        WSM_CALL_INIT,
        ee::s05::f_002224a8__generated
    );

    wsm_gated!(
        wsm_fake_call_run,
        WSM_CALL_RUN,
        ee::s05::f_002224b8__generated
    );

    wsm_gated!(
        wsm_fake_table_ctor,
        WSM_TABLE_CTOR,
        ee::s05::f_00224da0__generated
    );

    wsm_gated!(
        wsm_fake_release,
        WSM_VALUE_RELEASE,
        ee::s05::f_00225b90__generated
    );

    wsm_gated!(
        wsm_fake_cache_entry,
        0x003a_dc50,
        ee::s13::f_003adc50__generated
    );

    wsm_gated!(wsm_fake_free, 0x0031_7e50, ee::s10::f_00317e50__generated);

    pub(crate) fn wsm_install_fakes() {
        // The allocator's slot is shared with the PushLoopingAnim fakes.
        asl_install_fakes();
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003A6B78),
            wsm_fake_find_func,
            crate::difftest::fakes::prev_cell(wsm_fake_find_func),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_003097E0),
            wsm_fake_post,
            crate::difftest::fakes::prev_cell(wsm_fake_post),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_00226600),
            wsm_fake_value,
            crate::difftest::fakes::prev_cell(wsm_fake_value),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_002224A8),
            wsm_fake_call_init,
            crate::difftest::fakes::prev_cell(wsm_fake_call_init),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_002224B8),
            wsm_fake_call_run,
            crate::difftest::fakes::prev_cell(wsm_fake_call_run),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_00224DA0),
            wsm_fake_table_ctor,
            crate::difftest::fakes::prev_cell(wsm_fake_table_ctor),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s05::OVERRIDE_00225B90),
            wsm_fake_release,
            crate::difftest::fakes::prev_cell(wsm_fake_release),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003ADC50),
            wsm_fake_cache_entry,
            crate::difftest::fakes::prev_cell(wsm_fake_cache_entry),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            wsm_fake_free,
            crate::difftest::fakes::prev_cell(wsm_fake_free),
        );
    }

    /// Objects that post the trigger event or not (a real leaf returning 0),
    /// event mode 4 or not, finished riders, no rider; no function (-1), no
    /// owner (the real `CallFunc`), an owner or an instance with a table or
    /// without (a temporary one, the real table release); with and without a
    /// trigger effect. Script-VM callees are logged fakes (one may unwind).
    #[test]
    fn wscript_man_on_object_trigger_matches() {
        wsm_install_fakes();
        covered(
            ee::s10::f_0030a060__generated,
            wscript_man_on_object_trigger,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_a060, 10);
                c.pc = 0x0030_a060;
                c.bus.write32(WSM_CTRL, WSM_MAGIC);
                c.bus.write32(ASL_CTRL, ASL_MAGIC);
                let wsm = DATA + 0x5_0000;
                let volume = DATA + 0x5_1000;
                let trigger = DATA + 0x5_1100;
                let rider = DATA + 0x5_2000;
                let obj = rider + 0x6c0;
                let obj_vt = DATA + 0x5_3000;
                let world = DATA + 0x5_4000;
                let events = DATA + 0x5_4100;
                let owner = DATA + 0x5_5000;
                let instance = DATA + 0x5_5100;
                let fx = DATA + 0x5_6000;
                let fx_vt = DATA + 0x5_6100;
                let cache = DATA + 0x5_7000;
                fill(c, &mut r, wsm, 4);
                c.bus.write32(wsm + WSM_CACHE, cache);
                c.bus.write32(cache + 4, DATA + 0x5_7100);
                for i in 0..0x40 {
                    c.bus.write32(wsm + 0x3cc + 4 * i, r.next_u32());
                }
                c.bus.write32(volume + 0x50, trigger);
                c.bus.write32(trigger + 0x78, r.next_u32());
                c.bus
                    .write32(trigger + 0xc, if r.next_u32() % 3 == 0 { 0 } else { fx });
                c.bus.write32(fx + 0xc, fx_vt);
                c.bus
                    .write16(fx_vt + 0x148, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(fx_vt + 0x14c, FAKE + 0x20);
                // obj: id (slot 0x38) and "posts the event" (slot 0x40).
                c.bus.write32(obj, obj_vt);
                c.bus
                    .write16(obj_vt + 0x38, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(obj_vt + 0x3c, FAKE);
                c.bus.write16(obj_vt + 0x40, 0);
                c.bus.write32(
                    obj_vt + 0x44,
                    if seed % 3 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x10
                    },
                );
                c.bus.write32(rider + 0x480, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, events);
                c.bus
                    .write32(events + 0x214, if r.next_u32() % 3 == 0 { 4 } else { 1 });
                // The function and its owner.
                c.bus.write32(
                    WSM_CTRL + 4,
                    if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 0x400
                    },
                );
                let (o, i) = match (seed / 3) % 4 {
                    0 => (0, 0),
                    1 => (owner, 0),
                    2 => (0, instance),
                    _ => (owner, instance),
                };
                c.bus.write32(wsm + WSM_OWNER, o);
                c.bus.write32(wsm + WSM_INSTANCE, i);
                c.bus.write32(instance + 0x3c, owner);
                c.bus.write32(
                    owner + 0x1c,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32()
                    },
                );
                c.g[A0][0] = sx(wsm);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(volume);
                c.g[A3][0] = sx(if r.next_u32() % 8 == 0 { 0 } else { obj });
            },
        );
    }

    /// WScriptMan_FocusEnter: no controller (object type 3 or not, with or
    /// without a slot-1 script) or a controller whose v20 is a fake
    /// (non-zero: v21 is called) or the real zero leaf 0x1406c0. The real
    /// HasObjScript1/RunObjScript1 and script cache lookup run; as in the
    /// FocusLeave test the "has script" entry points at `man + 0x290`, which
    /// RunObjScript1 overwrites with the object (whose function is -1), so
    /// the script call itself is skipped.
    #[test]
    fn wscript_man_focus_enter_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        use crate::game::wscript::wscriptman::{OC264_WCTL_V20, OC264_WCTL_VTABLE, OC264_WOBJ_CTL};
        const MAN: u32 = DATA;
        const TABLE: u32 = DATA + 0x400;
        const CACHE: u32 = DATA + 0xc00;
        const SECTION: u32 = DATA + 0x1200;
        const NO_SCRIPT: u32 = DATA + 0x1400;
        const OBJ: u32 = DATA + 0x800;
        const CTL: u32 = DATA + 0xa00;
        const VTABLE: u32 = DATA + 0x1800;
        const FUNCS: u32 = DATA + 0x1_0000;
        const ZERO_LEAF: u32 = 0x0014_06c0;
        // Function entry `id` of slot 1.
        let func_at = |id: u32| FUNCS + (id >> 8) * 0x18 + 4;
        check(
            ee::s10::f_0030a3a0__generated,
            wscript_man_focus_enter,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_a3a0, 2);
                c.pc = 0x0030_a3a0;
                c.g[A0][0] = sx(MAN);
                c.g[A1][0] = sx(OBJ);
                // Script cache: man -> table -> cache (type table) -> section.
                c.bus.write32(MAN + 0x28c, TABLE);
                c.bus.write32(TABLE + 4, DATA + 0x600);
                c.bus.write32(DATA + 0x604, CACHE);
                c.bus.write32(CACHE + 8, SECTION);
                c.bus.write32(SECTION + 0x1c, FUNCS);
                c.bus.write32(NO_SCRIPT + 8, u32::MAX);
                c.bus.write32(MAN + 0x290, 0x100);
                c.bus.write32(func_at(0x100), r.next_u32() & 0x7fff_ffff);
                c.bus.write32(func_at(OBJ), u32::MAX);
                c.bus.write8(OBJ + 0x78, 0);
                let ty = if seed % 4 == 1 { r.next_u32() % 8 } else { 3 };
                c.bus.write16(CACHE, ty as u16);
                let has_script = seed % 2 == 0;
                let entry = if has_script {
                    MAN + 0x290 - 8
                } else {
                    NO_SCRIPT
                };
                c.bus.write32(OBJ + 0x88, entry);
                // Controller.
                let has_ctl = seed % 6 >= 3;
                c.bus
                    .write32(OBJ + OC264_WOBJ_CTL, if has_ctl { CTL } else { 0 });
                c.bus.write32(CTL + OC264_WCTL_VTABLE, VTABLE);
                let v20 = if seed % 3 == 0 { FAKE } else { ZERO_LEAF };
                for (slot, f) in [(OC264_WCTL_V20, v20), (WSF_CTL_V21, FAKE + 0x10)] {
                    c.bus.write16(VTABLE + slot, (r.next_u32() % 0x20) as u16);
                    c.bus.write32(VTABLE + slot + 4, f);
                }
            },
        );
    }

    /// `WScriptMan_SaveProcessList`: lists of 0..4 processes; the stream's
    /// Write virtual is a fake (sometimes unwinding); the real
    /// `WScriptProcessList_Count` and `WScriptProcess_SaveState` run (no
    /// tasks, no attached object; the real script-cache lookup on a small
    /// table).
    #[test]
    fn wscript_man_save_process_list_matches() {
        use crate::difftest::fakes::vtable;
        const LIST: u32 = DATA;
        const NODES: u32 = DATA + 0x1000;
        const CACHE_HOLDER: u32 = DATA + 0x3000;
        const CACHE: u32 = DATA + 0x3100;
        const TABLE: u32 = DATA + 0x3200;
        const STREAM: u32 = DATA + 0x20_0000;
        covered(
            ee::s10::f_0030bc80__generated,
            wscript_man_save_process_list,
            300,
            &|c, seed| {
                let n = seed % 5;
                let mut r = machine(c, seed, 0x0030_bc80, 1 + 3 * n);
                c.pc = 0x0030_bc80;
                let vt = vtable(c, &mut r, &[SAVE_STATE_STREAM_WRITE]);
                c.bus.write32(STREAM, vt);
                c.bus.write32(GAME_GP + 0x16c8, CACHE_HOLDER);
                c.bus.write32(CACHE_HOLDER + 4, CACHE);
                c.bus.write32(CACHE + 4, TABLE);
                fill(c, &mut r, TABLE, 16 * 3);
                c.bus.write32(LIST, if n == 0 { 0 } else { NODES });
                for i in 0..n {
                    let node = NODES + i * 0x100;
                    fill(c, &mut r, node, 0x30 / 4);
                    c.bus.write32(node + 0x4, 0); // no tasks
                    c.bus.write32(node + 0x1c, 0); // no attached object
                    c.bus.write32(node + 0x24, r.next_u32() % 16);
                    let next = if i + 1 < n { node + 0x100 } else { 0 };
                    c.bus.write32(node + WSCRIPT_SAVE_LIST_NEXT, next);
                }
                c.g[A0][0] = sx(DATA + 0x8000);
                c.g[A1][0] = sx(LIST);
                c.g[A2][0] = sx(STREAM);
            },
        );
    }

    /// WScriptMan_SaveState: a fake stream (its Write unwinds at some calls),
    /// empty process lists (the real WScriptMan_SaveProcessList), and 0..50
    /// live object tables whose real cLunoTable_Write takes its no-hash path;
    /// the real cWScriptCache_GetData reads the world's cache.
    #[test]
    fn wscript_man_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const SELF: u32 = DATA;
        const WORLD: u32 = DATA + 0x1000;
        const CACHE: u32 = DATA + 0x1100;
        const CACHE_DATA: u32 = DATA + 0x1200;
        const OBJ: u32 = DATA + 0x2000;
        const TABLE: u32 = DATA + 0x2100;
        const STREAM: u32 = DATA + 0x3000;
        covered(
            ee::s10::f_0030bb10__generated,
            wscript_man_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30_bb10, 8);
                c.pc = 0x0030_bb10;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(STREAM, vt);
                c.bus.write32(SELF + wsm_save::WORLD, WORLD);
                c.bus.write32(SELF + wsm_save::LIST_A, 0);
                c.bus.write32(SELF + wsm_save::LIST_B, 0);
                c.bus.write32(WORLD + 4, CACHE);
                c.bus.write32(CACHE + 4, CACHE_DATA);
                for i in 0..50 {
                    c.bus.write32(CACHE_DATA + 0xc * i + 8, r.next_u32());
                }
                // Each live object is a reference to a table without a hash
                // part (modulus -1).
                c.bus.write32(OBJ, TABLE);
                c.bus.write32(TABLE + 0xc, u32::MAX);
                let density = [0, 1, 4, 16][(seed % 4) as usize];
                for i in 0..50 {
                    let live = density != 0 && r.next_u32() % 16 < density;
                    c.bus
                        .write32(SELF + wsm_save::OBJS + 4 * i, if live { OBJ } else { 0 });
                }
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// `WScriptMan_RunObjScript1`: an object without a script slot, a slot
    /// whose id is -1, and a found script (handle `id << 8 | section`) run
    /// by the real `WScriptMan_CallFunc` on the Luno VM (the fixture of
    /// `wscript_man_call_func_matches`).
    #[test]
    fn wscript_man_run_obj_script1_matches() {
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        use crate::game::wscript::wscriptman::WSM_SECTION_TABLES;
        const SELF: u32 = DATA + 0x20_0000;
        const OBJ: u32 = SELF + 0x1000;
        const WORLD: u32 = SELF + 0x1100;
        const CACHE: u32 = SELF + 0x1200;
        const ENTRIES: u32 = SELF + 0x1300;
        const FUNCS: u32 = SELF + 0x1400;
        const SLOT: u32 = SELF + 0x1800;
        const OBJ_FUNCS: u32 = SELF + 0x1900;
        const SCRIPT: u32 = SELF + 0x2000;
        const TABLES: u32 = SELF + 0x3000;
        check(
            ee::s10::f_0030a298__generated,
            wscript_man_run_obj_script1,
            120,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x30_a298);
                c.pc = 0x0030_a298;
                let section = seed % 4;
                let id = (seed / 4) % 4;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OBJ);
                c.bus.write32(SELF + WSR1_WORLD, WORLD);
                c.bus.write32(SELF + WSR1_RUNNING, r.next_u32());
                c.bus.write32(WORLD + 4, CACHE);
                c.bus.write32(CACHE + 4, ENTRIES);
                for k in 0..4u32 {
                    let entry = ENTRIES + 0xc * k;
                    c.bus.write16(entry, 3);
                    c.bus.write32(entry + 8, FUNCS);
                }
                // The object's script slot: none, id -1, or function 1 of
                // object-script entry `k` = the handle.
                c.bus.write8(OBJ + 0x78, (r.next_u32() % 4) as u8);
                c.bus
                    .write32(OBJ + 0x88, if seed % 5 == 0 { 0 } else { SLOT });
                let k = r.next_u32() % 3;
                c.bus.write32(
                    SLOT + 8,
                    if seed % 5 == 1 {
                        u32::MAX
                    } else {
                        k << 8 | 0x5a
                    },
                );
                c.bus.write32(FUNCS + 0x1c, OBJ_FUNCS);
                c.bus.write32(OBJ_FUNCS + 0x18 * k + 4, id << 8 | section);
                // WScriptMan_CallFunc: the section function and its script.
                c.bus.write32(FUNCS + 0x3c, FUNCS + 0x40);
                c.bus.write32(FUNCS + 0x40 + 4 * id, SCRIPT);
                c.bus.write32(SCRIPT + 4, 0x10);
                c.bus.write32(SCRIPT + 0x14, 0);
                c.bus.write32(SCRIPT + 0x18, (seed / 16) % 4);
                c.bus.write32(SCRIPT + 0x1c, 0);
                for i in 0..4u32 {
                    let value = TABLES + 0x40 * i;
                    let table = TABLES + 0x400 + 0x40 * i;
                    c.bus.write32(SELF + WSM_SECTION_TABLES + 4 * i, value);
                    c.bus.write32(value, table);
                    c.bus.write32(table + 8, 100);
                }
                let d = 0x004c_9098;
                c.bus.write32(d, r.next_u32());
                c.bus.write32(d + 4, r.next_u32());
                c.bus.write32(d + 8, 2);
            },
        );
    }

    /// `WScriptProcessList_Count`: empty lists and lists of up to 9
    /// processes; events off, delivered, or unwinding at the loop head.
    #[test]
    fn wscript_process_list_count_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(
            ee::s10::f_0030b228__generated,
            wscript_process_list_count,
            240,
            &|c, seed| {
                let mut r = crate::difftest::Rng(0x30b2_2800 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let list = crate::difftest::DATA;
                let n = seed / 3 % 10;
                let node_at = |i: u32| crate::difftest::DATA + 0x1000 + 0x40 * i;
                for i in 0..n {
                    crate::difftest::fill(c, &mut r, node_at(i), 0x20);
                    let next = if i + 1 < n { node_at(i + 1) } else { 0 };
                    c.bus.write32(node_at(i) + OC305_WSCRIPT_PROCESS_NEXT, next);
                }
                c.bus.write32(list, if n > 0 { node_at(0) } else { 0 });
                c.g[A0][0] = sx(list);
                let mode = schedule(c, &mut r, seed, 0x0030_b228);
                if mode != Events::Off {
                    c.next_event = c.cycles + (r.next_u32() % 20) as u64;
                }
                mode
            },
        );
    }

    /// `WScriptMan_PostAirEvent`: random op counts (slots at every
    /// alignment mod 8, as the 12-byte stride gives) and random stack
    /// leftovers in the op temporary.
    #[test]
    fn wscript_man_post_air_event_matches() {
        crate::difftest::check_lohi(
            ee::s10::f_003098b0__generated,
            wscript_man_post_air_event,
            300,
            &|c, seed| {
                let mut r = crate::difftest::Rng(0x30_98b0 ^ (seed as u64) << 12);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0030_98b0;
                let man = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                c.g[A0][0] = sx(man);
                let count = seed % 64;
                c.bus.write32(man + POST_AIR_OP_COUNT, count);
                fill(c, &mut r, man + 0x44, 0x100);
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
            },
        );
    }

    // --------------------------------------- WScriptMan_CollectSetSave

    /// `WScriptMan_CollectSetSave`: a fake vtable slot 0x8 method.
    #[test]
    fn wscript_man_collect_set_save_matches() {
        covered(
            ee::s10::f_0030c6c8__generated,
            wscript_man_collect_set_save,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_c6c8, 1);
                c.pc = 0x0030_c6c8;
                let obj = DATA + 0x100;
                let vt = vtable(c, &mut r, &[WSCRIPT_SET_SAVE_SLOT]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = obj as u64;
            },
        );
    }

    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    /// Magic word that turns the gated test fakes on. It lives in each test
    /// machine's own RAM (`DATA + 0xf0000`, below the fakes' call log), so the
    /// rest of the suite reaches the fakes' chained-to bodies instead.
    pub(crate) const OC330_CTRL: u32 = DATA + 0x0f_0000;

    pub(crate) const OC330_MAGIC: u32 = 0x4f43_3330;

    /// The bodies the slots held before a test fake was installed.
    pub(crate) static OC330_PREV: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];

    /// Replace a generated slot with `fake` (once), remembering what it held.
    pub(crate) fn oc330_install(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu), idx: usize) {
        crate::difftest::fakes::hook(slot, fake, &OC330_PREV[idx]);
    }

    /// Run the body a slot held before our fake (if any).
    pub(crate) fn oc330_chain(c: &mut Cpu, idx: usize, mine: fn(&mut Cpu)) {
        let p = OC330_PREV[idx].load(Ordering::Relaxed);
        if p != 0 && p != mine as usize {
            let f: GuestFn = unsafe { std::mem::transmute(p) };
            f(c);
        }
    }

    pub(crate) fn oc330_active(c: &mut Cpu) -> bool {
        c.bus.read32(OC330_CTRL) == OC330_MAGIC
    }

    // -- 0x30a5c0 WScriptMan_RunKillScript ---------------------------------

    /// A gated fake for the world's script-function lookup: `OC330_CTRL+4` chooses
    /// "not found" (-1) or the handle at `OC330_CTRL+8`.
    pub(crate) fn oc330_fake_get_object_script_func(c: &mut Cpu) {
        if !oc330_active(c) {
            return oc330_chain(c, 0, oc330_fake_get_object_script_func);
        }
        let missing = c.bus.read32(OC330_CTRL + 4) != 0;
        c.g[V0][0] = if missing { u64::MAX } else { 5 };
        c.g[V1][0] = 0;
        c.cycles += 6;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    /// A gated fake for the script call: records the handle argument in `v0`.
    pub(crate) fn oc330_fake_call_func(c: &mut Cpu) {
        if !oc330_active(c) {
            return oc330_chain(c, 1, oc330_fake_call_func);
        }
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
        c.cycles += 4;
        let ra = c.g[RA][0] as u32;
        c.pc = ra;
    }

    /// The handle published in `self+0x290`, the script object lookup's
    /// "found" and "not found" branches, and the script call.
    #[test]
    fn wscript_man_run_kill_script_matches() {
        oc330_install(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_003A6B78),
            oc330_fake_get_object_script_func,
            0,
        );
        oc330_install(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00309C88),
            oc330_fake_call_func,
            1,
        );
        check(
            ee::s10::f_0030a5c0__generated,
            wscript_man_run_kill_script,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_a5c0, 1);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(this + RKS_SCRIPT, 0);
                c.bus.write32(OC330_CTRL, OC330_MAGIC);
                c.bus.write32(OC330_CTRL + 4, (seed % 2 == 0) as u32);
                c.bus.write32(OC330_CTRL + 8, 0);
                c.pc = 0x0030_a5c0;
            },
        );
    }

    use crate::difftest::put_words;

    pub(crate) use crate::game::lib::snd::tests::oc375_machine;
    pub(crate) use crate::game::lib::snd::tests::oc375_script;
    pub(crate) use crate::game::lib::snd::tests::OC375_COIN;
    pub(crate) use crate::game::lib::snd::tests::OC375_SMALL;

    /// WScriptMan_MissionCommand: every command 0..21 (both jump tables are
    /// written), forced (15/19) or gated by the game mode and the race,
    /// with a context or -1, the event queue busy or free, mission id or
    /// -1 (current mission or none), lookups that find the mission or not,
    /// and the BE queries' answers. ProcessCommand runs for real (with no
    /// process to touch); cmd 3's sound call (0x29d6e0, no override slot)
    /// is never reached: the fake before it unwinds.
    #[test]
    fn wscript_man_mission_command_matches() {
        covered(
            ee::s10::f_0030a868__generated,
            wscript_man_mission_command,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x0030_a868, 10);
                c.pc = 0x0030_a868;
                put_words(
                    c,
                    MISSION_CMD_TABLE1,
                    &[
                        0x30a938, 0x30a938, 0x30a938, 0x30a988, 0x30a988, 0x30a988, 0x30a988,
                        0x30a988, 0x30a950, 0x30a950, 0x30a988, 0x30a950, 0x30a988, 0x30a988,
                        0x30a988, 0x30a988, 0x30a988, 0x30a988, 0x30a938,
                    ],
                );
                put_words(
                    c,
                    MISSION_CMD_TABLE2,
                    &[
                        0x30ac68, 0x30ac68, 0x30ac68, 0x30a9d0, 0x30aa3c, 0x30aa70, 0x30ac68,
                        0x30ac68, 0x30ac68, 0x30ac68, 0x30ac68, 0x30ac68, 0x30ab50, 0x30ab20,
                        0x30ac0c, 0x30ac3c, 0x30ab80, 0x30aac4, 0x30ac68, 0x30abd8,
                    ],
                );
                let man = DATA + 0x100;
                let world = DATA + 0x400;
                let p = DATA + 0x500;
                let q = DATA + 0x600;
                let busy = DATA + 0x700;
                let obj = DATA + 0x3000;
                c.bus.write32(man, r.next_u32());
                c.bus.write32(man + 0x2a4, 0);
                c.bus
                    .write32(man + MISSION_CMD_MODE, (seed / 7 % 3 == 0) as u32);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, p);
                c.bus.write32(p + 0xc, q);
                c.bus.write32(q + 0x7c, (seed % 11 != 0) as u32);
                c.bus.write32(p + 0x28, busy);
                c.bus.write32(busy, [0, 5, 20][(seed / 3 % 3) as usize]);
                c.bus
                    .write8(0x0053_5c10, if seed % 13 == 0 { 3 } else { 4 });
                let vt = fakes::vtable(c, &mut r, &[0x88, 0x10, 0x98, 0xa0]);
                c.bus.write32(obj + MISSION_CMD_VT, vt);
                c.bus.write32(
                    obj + MISSION_CMD_ID,
                    if seed / 5 % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 40
                    },
                );
                let cmd = seed % 22;
                let any_id = seed / 22 % 3 != 0;
                let current = seed / 2 % 4 != 0;
                let found = seed / 4 % 3 != 0;
                c.g[A0][0] = sx(man);
                c.g[A1][0] = if any_id {
                    (r.next_u32() % 40) as u64
                } else {
                    u64::MAX
                };
                c.g[A2][0] = cmd as u64;
                c.g[A3][0] = if seed % 2 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() % 8) as u64
                };
                oc375_script(c, 0x0030_aeb8, if current { obj } else { 0 });
                oc375_script(c, 0x0030_b388, if found { obj } else { 0 });
                oc375_script(c, 0x0030_b320, 0);
                oc375_script(c, 0x0030_b8c0, OC375_COIN);
                for query in [
                    0x0015_3e28,
                    0x0015_3dd0,
                    0x0015_3ed8,
                    0x0015_3e80,
                    0x0015_3d78,
                ] {
                    oc375_script(c, query, OC375_COIN);
                }
                oc375_script(c, 0x0015_3fd8, OC375_SMALL);
                // Cmd 3 with a mission: unwind at its method or the sound-system
                // getter, so the real 0x29d6e0 never runs.
                if cmd == 3 {
                    let at = (if any_id { 6 } else { 5 }) + seed / 3 % 2;
                    c.bus.write32(fakes::LOG_UNWIND_AT, at);
                }
            },
        );
    }

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_queue;

    pub(crate) use crate::game::main::gamemode::tests::oc427_vtable;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_locator_code;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc427_world_cache;

    /// `WScriptMan_CollectSetAddItems`: ids that are -1, valid or point at
    /// missing models, both race paths (the byte at `0x535c11`), the
    /// `IsCollected` answers, bones with and without an object whose vtable
    /// method (a fake) is called before the object is rebuilt.
    #[test]
    fn wscript_man_collect_set_add_items_matches() {
        covered(
            ee::s10::f_0030c4a8__generated,
            wscript_man_collect_set_add_items,
            160,
            &|c, seed| {
                let mut r = oc427_base(c, seed, 0x30c4a8, 20, 0x0030_c4a8);
                let pool = DATA + 0x1000;
                let ids = DATA + 0x2000;
                let bones = DATA + 0x7000;
                let size = r.next_u32() % 4;
                c.bus.write8(0x0053_5c11, (r.next_u32() % 2) as u8);
                c.bus.write32(pool + 8, r.next_u32() % 4);
                for k in 0..4 {
                    c.bus.write32(ids + 4 * k, oc427_locator_code(&mut r));
                }
                oc427_world_cache(c, &mut r, bones);
                // bones: a +0xc object (or none) with a vtable at +0xc of it
                for b in 0..3u32 {
                    let bone = bones + 0x100 * b;
                    let obj = DATA + 0x7800 + 0x100 * b;
                    let vt = DATA + 0x7c00 + 0x100 * b;
                    c.bus
                        .write32(bone + 0xc, if r.next_u32() % 3 == 0 { 0 } else { obj });
                    c.bus.write32(obj + 0xc, vt);
                    oc427_vtable(c, vt, 8, (r.next_u32() as u16) & !3);
                }
                let mut collected = [0u64; 8];
                for v in collected.iter_mut() {
                    *v = (r.next_u32() % 2) as u64;
                }
                oc427_queue(c, 0x0015_38e8, &collected);
                oc427_queue(
                    c,
                    0x0031_7d70,
                    &[
                        (DATA + 0x1_0000) as u64,
                        (DATA + 0x1_1000) as u64,
                        (DATA + 0x1_2000) as u64,
                        (DATA + 0x1_3000) as u64,
                    ],
                );
                c.g[A0][0] = sx(pool);
                c.g[A1][0] = sx(ids);
                c.g[A2][0] = sx(size);
            },
        );
    }

    pub(crate) use crate::game::main::ssxapp::ssx_load_net_modules;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_gp;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_machine;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_queue_base;
    pub(crate) use crate::game::main::ssxapp::tests::oc429_w;

    pub(crate) use crate::game::main::ssxapp::tests::OC429_CTRL;
    pub(crate) use crate::game::main::ssxapp::tests::OC429_FX;
    pub(crate) use crate::game::main::ssxapp::tests::OC429_MAGIC;

    use ssx3_ee_rt::funcs::register_functions;
    use ssx3_ee_rt::funcs::FnEntry;
    use std::ptr::addr_of_mut;

    pub(crate) fn oc429_pop(c: &mut Cpu, addr: u32) -> Option<u64> {
        let q = oc429_queue_base(addr);
        let head = c.bus.read32(q);
        if head < c.bus.read32(q + 4) {
            c.bus.write32(q, head + 1);
            Some(c.bus.read64(q + 8 + 8 * head))
        } else {
            None
        }
    }

    /// A gated callee fake: logs the call like `fakes::fake`, then returns the
    /// next queued value (if any) in v0.
    pub(crate) fn oc429_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if let Some(v) = oc429_pop(c, addr) {
            c.g[V0][0] = v;
        }
    }

    macro_rules! oc429_fakes {
        ($(($name:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident)),* $(,)?) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC429_CTRL) != OC429_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), ee::$shard::$gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc429_fake_call(c, $addr, ra);
                }
            )*

            /// Hooks the gated fakes into the callees' override slots (once
            /// per process; with the gate off they run what the slots held).
            pub(crate) fn oc429_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc429_fakes! {
        (oc429_fake_0010f1c0, 0x0010_f1c0, s00, OVERRIDE_0010F1C0, f_0010f1c0__generated),
        (oc429_fake_00144bc0, 0x0014_4bc0, s01, OVERRIDE_00144BC0, f_00144bc0__generated),
        (oc429_fake_0014dc80, 0x0014_dc80, s01, OVERRIDE_0014DC80, f_0014dc80__generated),
        (oc429_fake_0014dd58, 0x0014_dd58, s01, OVERRIDE_0014DD58, f_0014dd58__generated),
        (oc429_fake_00154278, 0x0015_4278, s01, OVERRIDE_00154278, f_00154278__generated),
        (oc429_fake_00154588, 0x0015_4588, s01, OVERRIDE_00154588, f_00154588__generated),
        (oc429_fake_00156a60, 0x0015_6a60, s01, OVERRIDE_00156A60, f_00156a60__generated),
        (oc429_fake_001589b0, 0x0015_89b0, s01, OVERRIDE_001589B0, f_001589b0__generated),
        (oc429_fake_00158a50, 0x0015_8a50, s01, OVERRIDE_00158A50, f_00158a50__generated),
        (oc429_fake_00169340, 0x0016_9340, s02, OVERRIDE_00169340, f_00169340__generated),
        (oc429_fake_001694a8, 0x0016_94a8, s02, OVERRIDE_001694A8, f_001694a8__generated),
        (oc429_fake_001694b8, 0x0016_94b8, s02, OVERRIDE_001694B8, f_001694b8__generated),
        (oc429_fake_00169cf8, 0x0016_9cf8, s02, OVERRIDE_00169CF8, f_00169cf8__generated),
        (oc429_fake_0016a458, 0x0016_a458, s02, OVERRIDE_0016A458, f_0016a458__generated),
        (oc429_fake_0016ad38, 0x0016_ad38, s02, OVERRIDE_0016AD38, f_0016ad38__generated),
        (oc429_fake_0026d628, 0x0026_d628, s07, OVERRIDE_0026D628, f_0026d628__generated),
        (oc429_fake_0026e950, 0x0026_e950, s07, OVERRIDE_0026E950, f_0026e950__generated),
        (oc429_fake_00272488, 0x0027_2488, s07, OVERRIDE_00272488, f_00272488__generated),
        (oc429_fake_002729a8, 0x0027_29a8, s07, OVERRIDE_002729A8, f_002729a8__generated),
        (oc429_fake_00273938, 0x0027_3938, s07, OVERRIDE_00273938, f_00273938__generated),
        (oc429_fake_002739b8, 0x0027_39b8, s07, OVERRIDE_002739B8, f_002739b8__generated),
        (oc429_fake_002743c8, 0x0027_43c8, s07, OVERRIDE_002743C8, f_002743c8__generated),
        (oc429_fake_00275718, 0x0027_5718, s07, OVERRIDE_00275718, f_00275718__generated),
        (oc429_fake_002757f0, 0x0027_57f0, s07, OVERRIDE_002757F0, f_002757f0__generated),
        (oc429_fake_00275b98, 0x0027_5b98, s07, OVERRIDE_00275B98, f_00275b98__generated),
        (oc429_fake_00275cf8, 0x0027_5cf8, s07, OVERRIDE_00275CF8, f_00275cf8__generated),
        (oc429_fake_00277450, 0x0027_7450, s07, OVERRIDE_00277450, f_00277450__generated),
        (oc429_fake_002776e0, 0x0027_76e0, s07, OVERRIDE_002776E0, f_002776e0__generated),
        (oc429_fake_00277dd8, 0x0027_7dd8, s07, OVERRIDE_00277DD8, f_00277dd8__generated),
        (oc429_fake_002797a0, 0x0027_97a0, s07, OVERRIDE_002797A0, f_002797a0__generated),
        (oc429_fake_00282338, 0x0028_2338, s07, OVERRIDE_00282338, f_00282338__generated),
        (oc429_fake_00282bf0, 0x0028_2bf0, s07, OVERRIDE_00282BF0, f_00282bf0__generated),
        (oc429_fake_00282c88, 0x0028_2c88, s07, OVERRIDE_00282C88, f_00282c88__generated),
        (oc429_fake_00282cb0, 0x0028_2cb0, s07, OVERRIDE_00282CB0, f_00282cb0__generated),
        (oc429_fake_00283bb8, 0x0028_3bb8, s07, OVERRIDE_00283BB8, f_00283bb8__generated),
        (oc429_fake_00283d28, 0x0028_3d28, s07, OVERRIDE_00283D28, f_00283d28__generated),
        (oc429_fake_00283d70, 0x0028_3d70, s07, OVERRIDE_00283D70, f_00283d70__generated),
        (oc429_fake_00309568, 0x0030_9568, s10, OVERRIDE_00309568, f_00309568__generated),
        (oc429_fake_00317d70, 0x0031_7d70, s10, OVERRIDE_00317D70, f_00317d70__generated),
        (oc429_fake_00317e30, 0x0031_7e30, s10, OVERRIDE_00317E30, f_00317e30__generated),
        (oc429_fake_00317e50, 0x0031_7e50, s10, OVERRIDE_00317E50, f_00317e50__generated),
        (oc429_fake_00317e98, 0x0031_7e98, s10, OVERRIDE_00317E98, f_00317e98__generated),
        (oc429_fake_00318260, 0x0031_8260, s10, OVERRIDE_00318260, f_00318260__generated),
        (oc429_fake_003182d8, 0x0031_82d8, s10, OVERRIDE_003182D8, f_003182d8__generated),
        (oc429_fake_00318448, 0x0031_8448, s10, OVERRIDE_00318448, f_00318448__generated),
        (oc429_fake_003184e8, 0x0031_84e8, s10, OVERRIDE_003184E8, f_003184e8__generated),
        (oc429_fake_00319028, 0x0031_9028, s10, OVERRIDE_00319028, f_00319028__generated),
        (oc429_fake_003190a8, 0x0031_90a8, s10, OVERRIDE_003190A8, f_003190a8__generated),
        (oc429_fake_003284b8, 0x0032_84b8, s10, OVERRIDE_003284B8, f_003284b8__generated),
        (oc429_fake_00328c20, 0x0032_8c20, s10, OVERRIDE_00328C20, f_00328c20__generated),
        (oc429_fake_00328f28, 0x0032_8f28, s10, OVERRIDE_00328F28, f_00328f28__generated),
        (oc429_fake_003581b8, 0x0035_81b8, s11, OVERRIDE_003581B8, f_003581b8__generated),
        (oc429_fake_003a6c68, 0x003a_6c68, s13, OVERRIDE_003A6C68, f_003a6c68__generated),
        (oc429_fake_003a6c98, 0x003a_6c98, s13, OVERRIDE_003A6C98, f_003a6c98__generated),
        (oc429_fake_003a9800, 0x003a_9800, s13, OVERRIDE_003A9800, f_003a9800__generated),
        (oc429_fake_003e2580, 0x003e_2580, s14, OVERRIDE_003E2580, f_003e2580__generated),
        (oc429_fake_003e6448, 0x003e_6448, s14, OVERRIDE_003E6448, f_003e6448__generated),
        (oc429_fake_004166f4, 0x0041_66f4, s15, OVERRIDE_004166F4, f_004166f4__generated),
        (oc429_fake_0041aa88, 0x0041_aa88, s15, OVERRIDE_0041AA88, f_0041aa88__generated),
    }

    /// `RiderModel_dtor`: null and non-null buffers, 0..3 groups x variants,
    /// textures with negative and valid ids, `flags & 1`, unwinds in any call.
    #[test]
    fn rider_model_dtor_matches() {
        covered(
            ee::s10::f_0030d540__generated,
            rider_model_dtor,
            120,
            &|c, seed| {
                let mut r = oc429_machine(c, seed, 0x30d540, 80, 0x0030_d540);
                let this = OC429_FX + 0x400;
                let gfx = OC429_FX + 0x800;
                let gfx_vt = OC429_FX + 0x900;
                let groups = OC429_FX + 0x2000;
                let tex = OC429_FX + 0x6000;
                oc429_w(c, GAME_GP + 0x2a90, gfx);
                oc429_w(c, gfx + 0x10d8, gfx_vt);
                c.bus.write16(gfx_vt + 0x380, (r.next_u32() as u16) & !3);
                oc429_w(c, gfx_vt + 0x384, fakes::FAKE);
                c.bus.write16(gfx_vt + 0x190, (r.next_u32() as u16) & !3);
                oc429_w(c, gfx_vt + 0x194, fakes::FAKE + 0x10);
                let nv = seed % 3;
                let ng = (seed / 3) % 3;
                // Plain buffers: each present or absent by seed bit.
                for (i, off) in [0x1cu32, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c]
                    .iter()
                    .enumerate()
                {
                    let present = (r.next_u32() >> i) & 1 == 1 || seed % 7 == 0;
                    oc429_w(
                        c,
                        this + off,
                        if present {
                            OC429_FX + 0x100 + 0x10 * i as u32
                        } else {
                            0
                        },
                    );
                }
                oc429_w(c, this + RIDERMODEL_NUM_VARIANTS, nv);
                oc429_w(c, this + RIDERMODEL_NUM_GROUPS, ng);
                let has_groups = seed % 5 != 0;
                oc429_w(
                    c,
                    this + RIDERMODEL_GROUPS,
                    if has_groups { groups } else { 0 },
                );
                for k in 0..4u32 {
                    let nonnull = r.next_u32() & 1;
                    oc429_w(
                        c,
                        this + 0x44 + 8 * k,
                        nonnull * (OC429_FX + 0x180 + 0x10 * k),
                    );
                }
                // Per group j, slot k: variant array at +0x1c; entries 0x10 apart.
                for j in 0..3u32 {
                    for k in 0..4u32 {
                        let arr = OC429_FX + 0x3000 + (j * 4 + k) * 0x40;
                        let slot = groups + j * 0x58 + k * 8;
                        oc429_w(c, slot + 0x1c, if r.next_u32() & 3 == 0 { 0 } else { arr });
                        for v in 0..3u32 {
                            oc429_w(
                                c,
                                arr + 0x10 * v,
                                if r.next_u32() & 1 == 0 {
                                    0
                                } else {
                                    OC429_FX + 0x500
                                },
                            );
                            oc429_w(
                                c,
                                arr + 0x10 * v + 0xc,
                                if r.next_u32() & 1 == 0 {
                                    0
                                } else {
                                    OC429_FX + 0x540
                                },
                            );
                        }
                    }
                }
                let ntex = (seed / 2) % 4;
                oc429_w(c, this + RIDERMODEL_TEX_COUNT, ntex);
                oc429_w(
                    c,
                    this + RIDERMODEL_TEX_LIST,
                    if seed % 4 == 1 { 0 } else { tex },
                );
                for i in 0..4u32 {
                    let id = if r.next_u32() & 1 == 0 {
                        r.next_u32() | 0x8000_0000
                    } else {
                        r.next_u32() & 0x7fff
                    };
                    oc429_w(c, tex + 0xc * i, id);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (r.next_u32() & 0xffff_fffe | (seed & 1)) as u64;
            },
        );
    }

    /// Set in the child process of the tests that need an empty function
    /// table (every callee then lands in the logging fake).
    pub(crate) const OC429_CHILD_ENV: &str = "SSX3_OC429_CHILD";

    /// Result queue of the module-loader stub: `[head, len, values: u64 ...]`.
    pub(crate) const OC429_LOAD_Q: u32 = DATA + 0x20_1000;

    /// `0x319718` (the IOP module loader): logs the call, then returns the
    /// next queued result (negative = load failed).
    pub(crate) fn oc429_stub_load(c: &mut Cpu) {
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_9718, ret);
        if c.pc != ret {
            return;
        }
        let head = c.bus.read32(OC429_LOAD_Q);
        if head < c.bus.read32(OC429_LOAD_Q + 4) {
            c.bus.write32(OC429_LOAD_Q, head + 1);
            c.g[V0][0] = c.bus.read64(OC429_LOAD_Q + 8 + 8 * head);
        }
    }

    pub(crate) static OC429_STUBS: [FnEntry; 1] = [FnEntry {
        start: 0x0031_9718,
        end: 0x0031_971c,
        f: oc429_stub_load,
    }];

    /// Run test `name` of this module in a child process (the same test
    /// binary, `--exact`) where `body` runs against the stub function table:
    /// a table swap in the shared test process would race with the other
    /// tests' `fresh()`.
    pub(crate) fn oc429_in_child(name: &str, body: fn()) {
        if std::env::var_os(OC429_CHILD_ENV).is_some() {
            return body();
        }
        let path = format!(
            "{}::{name}",
            module_path!().split_once("::").map_or("", |p| p.1)
        );
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([path.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC429_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child failed:\n{text}"
        );
    }

    /// Queue `n` loader results, each failing (-1) with the given chance.
    pub(crate) fn oc429_load_results(c: &mut Cpu, r: &mut Rng, n: u32) {
        c.bus.write32(OC429_LOAD_Q, 0);
        c.bus.write32(OC429_LOAD_Q + 4, n);
        for i in 0..n {
            let v: u64 = if r.next_u32() % 3 == 0 { u64::MAX } else { 5 };
            c.bus.write64(OC429_LOAD_Q + 8 + 8 * i, v);
        }
    }

    pub(crate) fn oc429_lnm_checked() {
        covered(
            ee::s05::f_00228ee0__generated,
            ssx_load_net_modules,
            100,
            &|c, seed| {
                register_functions(&OC429_STUBS);
                let mut r = machine(c, seed, 0x228ee0, 30);
                c.pc = 0x0022_8ee0;
                oc429_w(c, oc429_gp(0xda4), r.next_u32());
                oc429_load_results(c, &mut r, 20);
            },
        );
    }

    /// `ssx_LoadNetModules`: every combination of failing SMAP / SPDUART
    /// loads (PPPOE, the voice modules and their start/stop), unwinds in any
    /// of the ~25 calls. Runs in a child process whose function table holds
    /// only the loader stub (all other callees are unported).
    #[test]
    fn ssx_load_net_modules_matches() {
        oc429_in_child("ssx_load_net_modules_matches", oc429_lnm_checked);
    }

    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;

    oc432_fake!(
        fake_avalanche_destroy_all,
        PREV_AVALANCHE_DESTROY_ALL,
        ee::s09::f_002d9b40__generated,
        0x002d_9b40
    );

    oc432_fake!(
        fake_wscript_reset_collect_set,
        PREV_WSCRIPT_RESET_COLLECT_SET,
        ee::s10::f_0030c390__generated,
        0x0030_c390
    );

    oc432_fake!(
        fake_wscript_call_func,
        PREV_WSCRIPT_CALL_FUNC,
        ee::s10::f_00309c88__generated,
        0x0030_9c88
    );

    /// `WScriptMan_Shutdown`: 0..3 world sections of type 3 or not, with or
    /// without a script cache, 0..3 global shutdown functions each (the two
    /// world accessors run real on the fixture's tables).
    #[test]
    fn wscript_man_shutdown_matches() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D9B40),
            fake_avalanche_destroy_all,
            &PREV_AVALANCHE_DESTROY_ALL,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030C390),
            fake_wscript_reset_collect_set,
            &PREV_WSCRIPT_RESET_COLLECT_SET,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00309C88),
            fake_wscript_call_func,
            &PREV_WSCRIPT_CALL_FUNC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A9800),
            fake_streamer_section_count,
            &PREV_STREAMER_SECTION_COUNT,
        );
        fakes::covered(
            ee::s10::f_00309f18__generated,
            wscript_man_shutdown,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x309f18, 12);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                let nsect = r.next_u32() % 4;
                c.bus.write32(OC432_CTRL + 4, nsect);
                let this = DATA + 0x5_0000;
                let world = DATA + 0x5_1000;
                let view = DATA + 0x5_2000;
                let info = DATA + 0x5_3000;
                let sections = DATA + 0x5_4000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x28c, world);
                c.bus.write32(world, view);
                c.bus.write32(world + 4, info);
                c.bus.write32(info + 4, sections);
                for i in 0..3u32 {
                    let sect = sections + i * 0xc;
                    let cache = DATA + 0x5_5000 + i * 0x40;
                    let ty = if r.next_u32() % 4 == 0 {
                        r.next_u32() & 0xffff
                    } else {
                        3
                    };
                    c.bus.write16(sect, ty as u16);
                    c.bus
                        .write32(sect + 8, if r.next_u32() % 5 == 0 { 0 } else { cache });
                    c.bus.write32(cache + 0x10, r.next_u32() % 4);
                    c.bus.write32(
                        cache + 0x14,
                        if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            DATA + 0x5_6000 + i * 0x40
                        },
                    );
                }
                c.bus.write32(this, r.next_u32());
            },
        );
    }

    /// A process list at `head` with one node per id: nodes at `base + 0x40 * i`,
    /// id at +0x20, next at +0x18. Returns the node addresses.
    pub(crate) fn oc557_chain(c: &mut Cpu, head: u32, base: u32, ids: &[u32]) -> Vec<u32> {
        let nodes: Vec<u32> = (0..ids.len() as u32).map(|i| base + 0x40 * i).collect();
        for (i, &node) in nodes.iter().enumerate() {
            c.bus.write32(node + 0x20, ids[i]);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            c.bus.write32(node + 0x18, next);
        }
        c.bus.write32(head, nodes.first().copied().unwrap_or(0));
        nodes
    }

    /// The world pointer at `gp - 0x848` with its playback block (+0x84),
    /// whose +0x34 is the playback manager and +0x28 the replay.
    pub(crate) fn oc557_world(c: &mut Cpu) {
        let world = DATA + 0x2000;
        let block = DATA + 0x2100;
        c.bus.write32(GAME_GP - OC557_GP_WORLD, world);
        c.bus.write32(world + 0x84, block);
        c.bus.write32(block + 0x34, DATA + 0x2200);
        c.bus.write32(block + 0x28, DATA + 0x2300);
    }

    #[test]
    fn wscript_man_unlink_process_matches() {
        check(
            ee::s10::f_003096f0__generated,
            wscript_man_unlink_process,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x3096_f0, 0);
                c.pc = 0x0030_96f0;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let act = oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[1, 2]);
                let idle = oc557_chain(c, this + 0x2b4, DATA + 0x1200, &[3]);
                let _add = oc557_chain(c, this + 0x2bc, DATA + 0x1400, &[4, 5]);
                let victim = match seed % 6 {
                    0 => act.first().copied(),
                    1 => act.get(1).copied(),
                    2 => idle.first().copied(),
                    3 => _add.get(1).copied(),
                    4 => None,
                    _ => act.first().copied(),
                };
                c.g[A1][0] = sx(victim.unwrap_or(DATA + 0x1800));
            },
        );
    }

    #[test]
    fn wscript_man_call_func_on_instance_matches() {
        check(
            ee::s10::f_00309d20__generated,
            wscript_man_call_func_on_instance_2,
            160,
            &|c, seed| {
                let _ = machine(c, seed, 0x309d20, 0);
                c.pc = 0x0030_9d20;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let handle = DATA + 0x900;
                c.bus.write8(handle, (seed % 4) as u8);
                c.g[A1][0] = sx(handle);
                c.g[A2][0] = sx(DATA + 0x700 + seed % 3);
                c.bus.write32(this + 0x28c, DATA + 0x400);
                c.bus.write32(this + 0x290, DATA + 0x500);
            },
        );
    }

    #[test]
    fn wscript_man_has_obj_script5_matches() {
        check(
            ee::s10::f_0030a660__generated,
            wscript_man_has_obj_script5,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30a660, 0);
                c.pc = 0x0030_a660;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed);
                c.bus.write32(this + 0x28c, DATA + 0x400);
                fill(c, &mut r, DATA + 0x400, 0x40);
            },
        );
    }

    #[test]
    fn wscript_man_process_command_matches() {
        check(
            ee::s10::f_0030a728__generated,
            wscript_man_process_command,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x30a728, 0);
                c.pc = 0x0030_a728;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let id: u32 = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    1 + seed % 3
                };
                c.g[A1][0] = sx(id);
                let kinds: [u64; 8] = [0, 1, 2, 18, u64::MAX, 3, 5, 1];
                c.g[A2][0] = kinds[(seed % 8) as usize];
                oc557_chain(c, this + 0x2b4, DATA + 0x1200, &[2, 3]);
                oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[1, 2]);
                let obj = if seed % 4 == 0 { 0 } else { DATA + 0x1800 };
                c.bus.write32(this + 0x2a4, obj);
            },
        );
    }

    /// Fixture for the two script-call functions: the current owner slot at
    /// `this + cur_off`, the object's table pointer and the saved instance.
    pub(crate) fn oc557_script_setup(c: &mut Cpu, seed: u32, salt: u64, cur_off: u32, task: bool) {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        let obj = DATA + 0x1800;
        c.g[A1][0] = sx(obj);
        c.g[A2][0] = sx(DATA + 0x1900);
        c.bus.write32(this + 0x290, r.next_u32());
        c.bus.write32(this + cur_off, r.next_u32());
        let table = if seed % 2 == 0 { 0 } else { DATA + 0x1a00 };
        if task {
            let process = DATA + 0x1c00;
            c.bus.write32(obj + 0x3c, process);
            c.bus.write32(process + 0x1c, table);
        } else {
            c.bus.write32(obj + 0x1c, table);
        }
        if seed % 2 == 0 {
            crate::game::bx::bxstring::tests::alloc_ready(c);
        }
    }

    #[test]
    fn wscript_man_call_process_func_matches() {
        check(
            ee::s10::f_0030ac98__generated,
            wscript_man_call_process_func,
            160,
            &|c, seed| {
                oc557_script_setup(c, seed, 0x30ac98, 0x2a4, false);
                c.pc = 0x0030_ac98;
            },
        );
    }

    #[test]
    fn wscript_man_call_task_func_matches() {
        check(
            ee::s10::f_0030ada8__generated,
            wscript_man_call_task_func,
            160,
            &|c, seed| {
                oc557_script_setup(c, seed, 0x30ada8, 0x2a8, true);
                c.pc = 0x0030_ada8;
            },
        );
    }

    #[test]
    fn wscript_man_get_event_arg_matches() {
        // The table of bit-field extractors at 0x490000 - 0x6630, one entry
        // per `which` (0..15), each naming one case of the port.
        const CASES: [u32; 16] = [
            0x0030_af38,
            0x0030_af48,
            0x0030_af5c,
            0x0030_af70,
            0x0030_af84,
            0x0030_af98,
            0x0030_afa8,
            0x0030_afbc,
            0x0030_afd0,
            0x0030_afe0,
            0x0030_aff0,
            0x0030_b004,
            0x0030_b018,
            0x0030_b02c,
            0x0030_b03c,
            0x0030_b050,
        ];
        check(
            ee::s10::f_0030af08__generated,
            wscript_man_get_event_arg,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30af08, 0);
                c.pc = 0x0030_af08;
                let table = 0x0049_0000u32.wrapping_sub(0x6630);
                for (i, &case) in CASES.iter().enumerate() {
                    c.bus.write32(table + 4 * i as u32, case);
                }
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A2][0] = (seed % 17) as u64;
                let record = if seed % 5 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(this + 0x29c, record);
                fill(c, &mut r, DATA + 0x400, 0x10);
            },
        );
    }

    #[test]
    fn wscript_process_list_find_by_name_matches() {
        check(
            ee::s10::f_0030b350__generated,
            wscript_process_list_find_by_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b350, 0);
                c.pc = 0x0030_b350;
                let this = DATA + 0x100;
                oc557_chain(c, this, DATA + 0x1000, &[7, 8]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 4);
            },
        );
    }

    #[test]
    fn wscript_process_list_find_mission_by_name_matches() {
        check(
            ee::s10::f_0030b3f0__generated,
            wscript_process_list_find_mission_by_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b3f0, 0);
                c.pc = 0x0030_b3f0;
                let this = DATA + 0x100;
                oc557_chain(c, this, DATA + 0x1000, &[7, 8]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 4);
            },
        );
    }

    #[test]
    fn wscript_man_find_mission_by_id_matches() {
        check(
            ee::s10::f_0030b458__generated,
            wscript_man_find_mission_by_id,
            160,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b458, 0);
                c.pc = 0x0030_b458;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(7 + seed % 2);
                oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[7]);
                oc557_chain(c, this + 0x2b4, DATA + 0x1200, &[8]);
                oc557_chain(c, this + 0x2bc, DATA + 0x1400, &[9]);
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_request_a_matches() {
        check(
            ee::s10::f_0030b4c0__generated,
            wscript_man_set_pending_request_a,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b4c0, 0);
                c.pc = 0x0030_b4c0;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(seed);
                c.g[A2][0] = sx(seed.wrapping_mul(3));
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_request_b_matches() {
        check(
            ee::s10::f_0030b4d8__generated,
            wscript_man_set_pending_request_b,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b4d8, 0);
                c.pc = 0x0030_b4d8;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(seed);
                c.g[A2][0] = sx(seed.wrapping_mul(5));
            },
        );
    }

    #[test]
    fn wscript_man_set_pending_request_c_matches() {
        check(
            ee::s10::f_0030b4f0__generated,
            wscript_man_set_pending_request_c,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b4f0, 0);
                c.pc = 0x0030_b4f0;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(seed);
                c.g[A2][0] = sx(seed.wrapping_mul(7));
            },
        );
    }

    #[test]
    fn wscript_man_set_hud_height_target_matches() {
        check(
            ee::s10::f_0030b508__generated,
            wscript_man_set_hud_height_target,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b508, 0);
                c.pc = 0x0030_b508;
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(DATA + 0x200);
                fill(c, &mut r, DATA + 0x200, 0x20);
            },
        );
    }

    #[test]
    fn wscript_man_request_mission_abort_matches() {
        check(
            ee::s10::f_0030b608__generated,
            wscript_man_request_mission_abort,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b608, 0);
                c.pc = 0x0030_b608;
                oc557_world(c);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x2a0, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_man_request_mission_end_matches() {
        check(
            ee::s10::f_0030b758__generated,
            wscript_man_request_mission_end_2,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30b758, 0);
                c.pc = 0x0030_b758;
                oc557_world(c);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x2a0, r.next_u32());
                c.bus.write32(this, seed % 3);
                c.bus.write32(DATA + 0x2400 + 0xc, DATA + 0x2500);
                c.bus.write32(DATA + 0x2500, r.next_u32());
            },
        );
    }

    #[test]
    fn wscript_man_is_process_active_matches() {
        check(
            ee::s10::f_0030b898__generated,
            wscript_man_is_process_active,
            160,
            &|c, seed| {
                let _ = machine(c, seed, 0x30b898, 0);
                c.pc = 0x0030_b898;
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                oc557_chain(c, this + 0x2b8, DATA + 0x1000, &[2, 5, 6]);
                let proc_ = DATA + 0x1800;
                c.bus
                    .write32(proc_ + 0x20, if seed % 2 == 0 { 5 } else { 9 });
                c.g[A1][0] = sx(proc_);
            },
        );
    }

    /// Covers the three paths: a wrong priority (no-op), a zero `init`
    /// (no-op), and the full table build.
    #[test]
    fn wscript_man_static_init_matches() {
        check(
            ssx3_ee::s10::f_0030c8d8__generated,
            wscript_man_static_init,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c8d_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = if seed % 3 == 0 {
                    0x0001_0000 + seed as u64
                } else {
                    0xffff
                };
                c.g[A0][0] = if seed % 5 == 1 { 0 } else { 1 + seed as u64 };
            },
        );
    }

    #[test]
    fn wscriptman_debugprintf_matches() {
        check(
            ee::s10::f_0030c898__generated,
            wscriptman_debugprintf,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c89_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_c898;
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn wscriptman_pop_event_matches() {
        check(
            ee::s10::f_0030d348__generated,
            wscriptman_pop_event,
            60,
            &|c, seed| {
                let mut r = Rng(0x0d34_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d348;
                c.g[A0][0] = DATA as u64;
                // An empty queue (head == tail): the callee's early return.
                let q = r.next_u32();
                c.bus.write32(DATA + 0x1c4, q);
                c.bus.write32(DATA + 0x1c8, q);
            },
        );
    }

    #[test]
    fn wscriptman_peek_event_matches() {
        check(
            ee::s10::f_0030d368__generated,
            wscriptman_peek_event,
            60,
            &|c, seed| {
                let mut r = Rng(0x0d36_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d368;
                c.g[A0][0] = DATA as u64;
                let q = r.next_u32();
                c.bus.write32(DATA + 0x1c4, q);
                c.bus.write32(DATA + 0x1c8, q);
            },
        );
    }

    #[test]
    fn wscriptprocess_run_callback_matches() {
        check(
            ee::s10::f_0030d3e0__generated,
            wscriptprocess_run_callback,
            30,
            &|c, seed| {
                let mut r = Rng(0x0d3e_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d3e0;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, DATA + 0x200);
            },
        );
    }

    #[test]
    fn wscripttask_run_callback_matches() {
        check(
            ee::s10::f_0030d400__generated,
            wscripttask_run_callback,
            30,
            &|c, seed| {
                let mut r = Rng(0x0d40_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d400;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, DATA + 0x200);
            },
        );
    }

    #[test]
    fn wscriptman_global_ctors_matches() {
        check(
            ee::s10::f_0030d498__generated,
            wscriptman_global_ctors,
            10,
            &|c, seed| {
                let mut r = Rng(0x0d49_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0030_d498;
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    /// Gate word: the fakes below act only while it holds `OC655_MAGIC`.
    pub(crate) const OC655_CTRL: u32 = DATA + 0x4_f000;

    pub(crate) const OC655_MAGIC: u32 = 0x6f63_0655;

    /// Bit `n` set: the fake with bit `n` returns 0 (instead of its random value).
    pub(crate) const OC655_ZERO: u32 = DATA + 0x4_f008;

    /// The FindById hits: bit `k` set, the list at slot `k` (idle, active,
    /// add) holds the process.
    pub(crate) const OC655_HITS: u32 = DATA + 0x4_f020;

    /// The process manager, and the process object a FindById hit returns.
    pub(crate) const OC655_MAN: u32 = DATA + 0x5_0000;

    pub(crate) const OC655_OBJ: u32 = DATA + 0x5_3000;

    pub(crate) const OC655_OBJ_VT: u32 = DATA + 0x5_3100;

    /// A gated fake of a callee: with the gate set it logs the call and
    /// returns (`fake_call`), with bit `$bit` of the zero word set it returns
    /// 0; with the gate off it runs the body it replaced.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path, $bit:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC655_CTRL) != OC655_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                fake_call(c, $target);
                if (c.bus.read32(OC655_ZERO) >> $bit) & 1 != 0 {
                    c.g[V0][0] = 0;
                }
            }
        };
    }

    gated_fake!(
        oc655_get_instance,
        OC655_PREV_GET_INSTANCE,
        0x0027_2cc0,
        ee::s07::f_00272cc0__generated,
        0
    );

    gated_fake!(
        oc655_run_to_end,
        OC655_PREV_RUN_TO_END,
        0x0027_4868,
        ee::s07::f_00274868__generated,
        1
    );

    gated_fake!(
        oc655_cmdq_count,
        OC655_PREV_CMDQ_COUNT,
        0x0028_3d28,
        ee::s07::f_00283d28__generated,
        2
    );

    gated_fake!(
        oc655_pop_entries,
        OC655_PREV_POP_ENTRIES,
        0x0027_7598,
        ee::s07::f_00277598__generated,
        3
    );

    gated_fake!(
        oc655_list_prepare,
        OC655_PREV_LIST_PREPARE,
        0x0027_6270,
        ee::s07::f_00276270__generated,
        4
    );

    gated_fake!(
        oc655_cmdq_at,
        OC655_PREV_CMDQ_AT,
        0x0028_3d70,
        ee::s07::f_00283d70__generated,
        5
    );

    gated_fake!(
        oc655_is_movie,
        OC655_PREV_IS_MOVIE,
        0x0027_7dd8,
        ee::s07::f_00277dd8__generated,
        6
    );

    gated_fake!(
        oc655_kill_process,
        OC655_PREV_KILL_PROCESS,
        0x0028_2540,
        ee::s07::f_00282540__generated,
        7
    );

    gated_fake!(
        oc655_movie_stop,
        OC655_PREV_MOVIE_STOP,
        0x0028_3818,
        ee::s07::f_00283818__generated,
        8
    );

    gated_fake!(
        oc655_set_paused,
        OC655_PREV_SET_PAUSED,
        0x0027_7c08,
        ee::s07::f_00277c08__generated,
        9
    );

    gated_fake!(
        oc655_prepare_entry,
        OC655_PREV_PREPARE_ENTRY,
        0x0027_7450,
        ee::s07::f_00277450__generated,
        10
    );

    gated_fake!(
        oc655_list_stop,
        OC655_PREV_LIST_STOP,
        0x0027_6868,
        ee::s07::f_00276868__generated,
        11
    );

    gated_fake!(
        oc655_start_transition,
        OC655_PREV_START_TRANSITION,
        0x0027_7980,
        ee::s07::f_00277980__generated,
        12
    );

    gated_fake!(
        oc655_try_advance,
        OC655_PREV_TRY_ADVANCE,
        0x0027_7800,
        ee::s07::f_00277800__generated,
        13
    );

    gated_fake!(
        oc655_add_choice,
        OC655_PREV_ADD_CHOICE,
        0x0027_8e50,
        ee::s07::f_00278e50__generated,
        14
    );

    gated_fake!(
        oc655_nis_prepare,
        OC655_PREV_NIS_PREPARE,
        0x0027_8f38,
        ee::s07::f_00278f38__generated,
        15
    );

    gated_fake!(
        oc655_nis_start,
        OC655_PREV_NIS_START,
        0x0027_8f68,
        ee::s07::f_00278f68__generated,
        16
    );

    gated_fake!(
        oc655_audio_get,
        OC655_PREV_AUDIO_GET,
        0x0028_b180,
        ee::s07::f_0028b180__generated,
        17
    );

    gated_fake!(
        oc655_playback_stop,
        OC655_PREV_PLAYBACK_STOP,
        0x0026_cdf8,
        ee::s07::f_0026cdf8__generated,
        18
    );

    gated_fake!(
        oc655_replay_reset,
        OC655_PREV_REPLAY_RESET,
        0x0026_f228,
        ee::s07::f_0026f228__generated,
        19
    );

    gated_fake!(
        oc655_load_playback,
        OC655_PREV_LOAD_PLAYBACK,
        0x0026_caf8,
        ee::s07::f_0026caf8__generated,
        20
    );

    gated_fake!(
        oc655_task_load,
        OC655_PREV_TASK_LOAD,
        0x0030_8798,
        ee::s10::f_00308798__generated,
        21
    );

    gated_fake!(
        oc655_process_ctor,
        OC655_PREV_PROCESS_CTOR,
        0x0030_7598,
        ee::s10::f_00307598__generated,
        22
    );

    gated_fake!(
        oc655_ea_alloc,
        OC655_PREV_EA_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated,
        23
    );

    gated_fake!(
        oc655_push_front,
        OC655_PREV_PUSH_FRONT,
        0x0030_b260,
        ee::s10::f_0030b260__generated,
        24
    );

    gated_fake!(
        oc655_list_remove,
        OC655_PREV_LIST_REMOVE,
        0x0030_b2d0,
        ee::s10::f_0030b2d0__generated,
        25
    );

    gated_fake!(
        oc655_cache_get,
        OC655_PREV_CACHE_GET,
        0x003a_ca38,
        ee::s13::f_003aca38__generated,
        26
    );

    gated_fake!(
        oc655_luno_dtor,
        OC655_PREV_LUNO_DTOR,
        0x0022_4df0,
        ee::s05::f_00224df0__generated,
        27
    );

    gated_fake!(
        oc655_luno_ctor,
        OC655_PREV_LUNO_CTOR,
        0x0022_4da0,
        ee::s05::f_00224da0__generated,
        28
    );

    pub(crate) static OC655_PREV_FIND_BY_ID: AtomicUsize = AtomicUsize::new(0);

    /// FindById: the process is found on the list whose slot bit is set in
    /// `OC655_HITS` (slots: idle +0x2b4, active +0x2b8, add +0x2bc).
    pub(crate) fn oc655_find_by_id(c: &mut Cpu) {
        if c.bus.read32(OC655_CTRL) != OC655_MAGIC {
            return fall_back(c, &OC655_PREV_FIND_BY_ID, ee::s10::f_0030b320__generated);
        }
        let list = c.g[A0][0] as u32;
        let slot = list.wrapping_sub(OC655_MAN + 0x2b4) / 4;
        let hit = (c.bus.read32(OC655_HITS) >> (slot & 31)) & 1 != 0;
        fake_call(c, 0x0030_b320);
        c.g[V0][0] = if hit { OC655_OBJ as u64 } else { 0 };
    }

    /// Puts the gated fakes of this module in their override slots (once per
    /// process; with the gate off each runs the body it replaced).
    pub(crate) fn oc655_hook_all() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00272CC0),
            oc655_get_instance,
            &OC655_PREV_GET_INSTANCE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274868),
            oc655_run_to_end,
            &OC655_PREV_RUN_TO_END,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283D28),
            oc655_cmdq_count,
            &OC655_PREV_CMDQ_COUNT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277598),
            oc655_pop_entries,
            &OC655_PREV_POP_ENTRIES,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276270),
            oc655_list_prepare,
            &OC655_PREV_LIST_PREPARE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283D70),
            oc655_cmdq_at,
            &OC655_PREV_CMDQ_AT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277DD8),
            oc655_is_movie,
            &OC655_PREV_IS_MOVIE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282540),
            oc655_kill_process,
            &OC655_PREV_KILL_PROCESS,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283818),
            oc655_movie_stop,
            &OC655_PREV_MOVIE_STOP,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277C08),
            oc655_set_paused,
            &OC655_PREV_SET_PAUSED,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277450),
            oc655_prepare_entry,
            &OC655_PREV_PREPARE_ENTRY,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            oc655_list_stop,
            &OC655_PREV_LIST_STOP,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277980),
            oc655_start_transition,
            &OC655_PREV_START_TRANSITION,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277800),
            oc655_try_advance,
            &OC655_PREV_TRY_ADVANCE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278E50),
            oc655_add_choice,
            &OC655_PREV_ADD_CHOICE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F38),
            oc655_nis_prepare,
            &OC655_PREV_NIS_PREPARE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278F68),
            oc655_nis_start,
            &OC655_PREV_NIS_START,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            oc655_audio_get,
            &OC655_PREV_AUDIO_GET,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026CDF8),
            oc655_playback_stop,
            &OC655_PREV_PLAYBACK_STOP,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F228),
            oc655_replay_reset,
            &OC655_PREV_REPLAY_RESET,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026CAF8),
            oc655_load_playback,
            &OC655_PREV_LOAD_PLAYBACK,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00308798),
            oc655_task_load,
            &OC655_PREV_TASK_LOAD,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00307598),
            oc655_process_ctor,
            &OC655_PREV_PROCESS_CTOR,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc655_ea_alloc,
            &OC655_PREV_EA_ALLOC,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B260),
            oc655_push_front,
            &OC655_PREV_PUSH_FRONT,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B2D0),
            oc655_list_remove,
            &OC655_PREV_LIST_REMOVE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_0030B320),
            oc655_find_by_id,
            &OC655_PREV_FIND_BY_ID,
        );
        hook(
            addr_of_mut!(ee::s13::OVERRIDE_003ACA38),
            oc655_cache_get,
            &OC655_PREV_CACHE_GET,
        );
        hook(
            addr_of_mut!(ee::s05::OVERRIDE_00224DF0),
            oc655_luno_dtor,
            &OC655_PREV_LUNO_DTOR,
        );
        hook(
            addr_of_mut!(ee::s05::OVERRIDE_00224DA0),
            oc655_luno_ctor,
            &OC655_PREV_LUNO_CTOR,
        );
    }

    /// The gate on, and random zero/hit words for this machine.
    pub(crate) fn oc655_gates(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC655_CTRL, OC655_MAGIC);
        c.bus.write32(OC655_ZERO, r.next_u32());
        c.bus.write32(OC655_HITS, r.next_u32() & 7);
    }

    /// The rider-independent vtable slot `{delta, fn}` at `off` of `vt`.
    pub(crate) fn oc655_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, func: u32) {
        c.bus.write16(vt + off, ((r.next_u32() as i16) & !3) as u16);
        c.bus.write32(vt + off + 4, func);
    }

    #[test]
    fn wscript_man_create_process_matches() {
        oc655_hook_all();
        covered(
            ee::s10::f_003094c8__generated,
            wscript_man_create_process_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_94c8, 3);
                oc655_gates(c, &mut r);
                c.g[A0][0] = OC655_MAN as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn wscript_man_delete_process_by_id_matches() {
        oc655_hook_all();
        covered(
            ee::s10::f_00309618__generated,
            wscript_man_delete_process_by_id,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_9618, 4);
                oc655_gates(c, &mut r);
                // The found process: its vtable's destructor slot (+0x0c, delta +8).
                c.bus.write32(OC655_OBJ + 0x5c, OC655_OBJ_VT);
                oc655_slot(c, &mut r, OC655_OBJ_VT, 0x8, FAKE);
                c.g[A0][0] = OC655_MAN as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn wscript_man_request_mission_playback_matches() {
        oc655_hook_all();
        covered(
            ee::s10::f_0030b590__generated,
            wscript_man_request_mission_playback_2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_b590, 3);
                oc655_gates(c, &mut r);
                let world = OC655_MAN + 0x1000;
                let game = OC655_MAN + 0x1100;
                let pm = OC655_MAN + 0x1200;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x34, pm);
                c.bus.write32(game + 0x28, OC655_MAN + 0x1300);
                c.bus.write32(OC655_MAN + 0x2a0, r.next_u32());
                c.g[A0][0] = OC655_MAN as u64;
            },
        );
    }

    pub(crate) use crate::game::animation::animsequencer::tests::oc684_w;

    /// `SphereTree_SortChildOrder`: random child centres and direction (some
    /// seeds with equal centres, so the keys tie) through the real shell sort.

    /// `SphereTree_CollidePlane`: compressed and plain tree info (AllocBuf
    /// fake), `worstCase` negative, zero, positive or -0 from the faked plane
    /// check, the real sort-axes call in between; fakes unwind at any call.

    /// `WScriptMan_CheckRiderGate`: the three ids each -1 (none), valid, or
    /// pointing at a null entry / null node (6^3 combinations), with the
    /// rider's first query either set (fake call, returns 1) or clear (real
    /// leaf returning 0, then the position from a second fake).
    #[test]
    fn wscript_man_check_rider_gate_matches() {
        fakes::covered(
            ee::s10::f_0030bfc0__generated,
            wscript_man_check_rider_gate,
            432,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_bfc0, 1);
                // The first fake call unwinds on a third of the seeds.
                let unwind = (seed / 7) % 3 == 0;
                oc684_w(c, fakes::LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
                const NODES: u32 = DATA + 0x7000;
                const POS: u32 = DATA + 0x7800;
                // Ref table: ctx -> A -> [+8] B[lo] -> entry -> [+0x1c] C[hi]
                // -> (node >> 2) << 8.
                let (ctx, ta, tb, entry, tc) = (
                    DATA + 0x5000,
                    DATA + 0x5100,
                    DATA + 0x5200,
                    DATA + 0x5300,
                    DATA + 0x5400,
                );
                oc684_w(c, GAME_GP + 0x16c8, ctx);
                oc684_w(c, ctx, ta);
                oc684_w(c, ta + 8, tb);
                for lo in 0..4u32 {
                    oc684_w(c, tb + 4 * lo, entry);
                }
                oc684_w(c, tb + 16, 0);
                oc684_w(c, entry + 0x1c, tc);
                for hi in 0..4u32 {
                    let node = NODES + 0x100 * hi;
                    let word = if hi == 3 {
                        0
                    } else {
                        ((node >> 2) << 8) | 0x55
                    };
                    oc684_w(c, tc + 4 * hi, word);
                    for k in 0..4 {
                        oc684_w(c, node + 0x40 + 4 * k, r.f32(10.0));
                    }
                }
                for k in 0..4 {
                    oc684_w(c, POS + 4 * k, r.f32(10.0));
                }
                // The rider: index at `idx`, object with the vtable at +0x6c0.
                let (world, manager, table, idx) =
                    (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200, DATA + 0x4400);
                let (rider, vt) = (DATA + 0x6900, DATA + 0x6800);
                let slot = seed % 3;
                oc684_w(c, GAME_GP - 0x848, world);
                oc684_w(c, world + 0x84, manager);
                oc684_w(c, manager + 0xc, table);
                oc684_w(c, idx, slot);
                oc684_w(c, table + 0x28 + 4 * slot, rider);
                oc684_w(c, rider + 0x6c0, vt);
                let delta = (r.next_u32() % 4) * 4;
                c.bus.write16(vt + 0x18, delta as u16);
                c.bus.write16(vt + 0x28, delta as u16);
                let first_set = seed / 216 == 0;
                // Slot 0x1c: the fake (non-zero result) or a leaf returning 0.
                oc684_w(
                    c,
                    vt + 0x1c,
                    if first_set { fakes::FAKE } else { 0x0014_06c0 },
                );
                oc684_w(c, vt + 0x2c, fakes::FAKE);
                // The second query's result: the rider position (fake call 0).
                oc684_w(c, fakes::LOG_RESULTS, POS);
                put_words(
                    c,
                    GAME_GP - 0x3364,
                    &[(1.0f32 + (seed % 3) as f32).to_bits()],
                );
                put_words(
                    c,
                    GAME_GP - 0x3360,
                    &[(1.0f32 + (seed / 3 % 3) as f32).to_bits()],
                );
                // Ids: -1, valid (hi 0..2, lo 0..2), null node (hi 3), null entry (lo 4).
                let ids = [u32::MAX, 0x000, 0x101, 0x202, 0x303, 0x004];
                let pick = |k: u32| sx(ids[((seed % 216) / 6u32.pow(k) % 6) as usize]);
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = pick(0);
                c.g[A2][0] = pick(1);
                c.g[A3][0] = pick(2);
            },
        );
    }

    use crate::difftest;

    pub(crate) use crate::game::visualfx::worldlightman::tests::ls_setup;

    /// Restoring the script manager from a replay frame: every combination of
    /// list lengths, read counts (none, some, negative) and unwinds in the
    /// stream reads and the table loads.
    #[test]
    fn wscript_man_load_state_matches() {
        difftest::check(
            ee::s10::f_0030bd20__generated,
            wscript_man_load_state,
            180,
            &ls_setup,
        );
    }
}
