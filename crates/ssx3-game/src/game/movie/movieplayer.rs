//! `src/movie/movieplayer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::delete_if_flag;
pub(crate) use crate::game::ai::rider::fr_3 as fr_2;
pub(crate) use crate::game::ai::rider::set_fr;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_4;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::dirtysock::tagsunk::p_2;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::lh_2 as lh_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_5;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_5;
pub(crate) use crate::game::fe::feasyncfile::u32_at;
pub(crate) use crate::game::fe::fepopupmisc::gp_2;
pub(crate) use crate::game::fe::fepopupmisc::lh;
pub(crate) use crate::game::fe::fepopupmisc::lw_2;
pub(crate) use crate::game::fe::fepopupmisc::p_2 as p_4;
pub(crate) use crate::game::fe::fepopupmisc::sw as sw_2;
pub(crate) use crate::game::fe::festaterewards::lw_3;
pub(crate) use crate::game::fe::festateselectmp::epilogue;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::fe::ovstates::at as at_5;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at_3;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
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
    (0x0023_c850, mc_manager_exists, "MCManager_Exists"),
    (0x0023_c860, mc_manager_destroy, "MCManager_Destroy"),
    (0x0023_c898, mc_manager_is_idle, "MCManager_IsIdle"),
    (
        0x0023_c8d0,
        mc_manager_has_status_error,
        "MCManager_HasStatusError",
    ),
    (
        0x0023_ca28,
        mc_manager_read_save_by_slot,
        "MCManager_ReadSaveBySlot",
    ),
    (
        0x0023_ca70,
        mc_manager_card_set_state_1e,
        "MCManager_CardSetState1E",
    ),
    (
        0x0023_cc20,
        mc_manager_card_set_state18,
        "MCManager_CardSetState18",
    ),
    (0x0023_d5a8, mc_manager_create_card, "MCManager_CreateCard"),
    (
        0x0023_d5e8,
        mc_manager_destroy_card,
        "MCManager_DestroyCard",
    ),
    (0x0023_d618, mc_manager_update, "MCManager_Update"),
    (0x0023_e2c8, mc_manager_state35, "MCManager_State35"),
    (0x0023_e320, mc_manager_state36, "MCManager_State36"),
    (0x0023_e3c0, mc_manager_state37, "MCManager_State37"),
    (0x0023_e428, mc_manager_state38, "MCManager_State38"),
    (0x0023_e498, mc_manager_state33, "MCManager_State33"),
    (0x0023_e4f8, mc_manager_state02, "MCManager_State02"),
    (
        0x0023_e820,
        mc_manager_state04_no_card,
        "MCManager_State04_NoCard",
    ),
    (
        0x0023_ea30,
        mc_manager_end_operation,
        "MCManager_EndOperation",
    ),
    (
        0x0023_ea90,
        mc_manager_on_operation_done,
        "MCManager_OnOperationDone",
    ),
    (
        0x0023_eb50,
        mc_manager_set_state_no_card,
        "MCManager_SetStateNoCard",
    ),
    (0x0023_eb80, mc_manager_state05, "MCManager_State05"),
    (
        0x0023_ef68,
        mc_manager_state0a_format,
        "MCManager_State0A_Format",
    ),
    (
        0x0023_f050,
        mc_manager_state0b_format,
        "MCManager_State0B_Format",
    ),
    (0x0023_f0f8, mc_manager_state_0c, "MCManager_State0C"),
    (0x0023_f150, mc_manager_state14, "MCManager_State14"),
    (0x0023_f1d8, mc_manager_state15, "MCManager_State15"),
    (0x0023_f258, mc_manager_state17, "MCManager_State17"),
    (0x0023_f2d8, mc_manager_state_13, "MCManager_State13"),
    (0x0023_f3a0, mc_manager_state0e, "MCManager_State0E"),
    (
        0x0023_f438,
        mc_manager_state0f_save,
        "MCManager_State0F_Save",
    ),
    (
        0x0023_f4f8,
        mc_manager_state10_save,
        "MCManager_State10_Save",
    ),
    (
        0x0023_fae0,
        mc_manager_refresh_slot,
        "MCManager_RefreshSlot",
    ),
    (
        0x0023_fb20,
        mc_manager_request_card_info,
        "MCManager_RequestCardInfo",
    ),
    (0x0023_fb58, mc_manager_reset, "MCManager_Reset"),
    (
        0x0023_fbb8,
        mc_manager_state12_save,
        "MCManager_State12_Save",
    ),
    (0x0023_fc70, mc_manager_state_24, "MCManager_State24"),
    (0x0023_fd08, mc_manager_state26, "MCManager_State26"),
    (0x0023_fe70, mc_manager_state25, "MCManager_State25"),
    (
        0x0023_ff00,
        mc_manager_state22_load,
        "MCManager_State22_Load",
    ),
    (0x0023_ffc0, mc_manager_state_23, "MCManager_State23"),
    (
        0x0024_00b0,
        mc_manager_state1f_load,
        "MCManager_State1F_Load",
    ),
    (
        0x0024_05d0,
        mc_manager_state21_load,
        "MCManager_State21_Load",
    ),
    (0x0024_0688, mcmanager_state18, "MCManager_State18"),
    (
        0x0024_0768,
        mc_manager_state19_delete,
        "MCManager_State19_Delete",
    ),
    (0x0024_0800, mc_manager_state1c, "MCManager_State1C"),
    (
        0x0024_0860,
        mcmanager_state1a_delete,
        "MCManager_State1A_Delete",
    ),
    (0x0024_0960, mc_manager_state1b, "MCManager_State1B"),
    (
        0x0024_0ab0,
        mcmanager_state27_format,
        "MCManager_State27_Format",
    ),
    (
        0x0024_0b88,
        mc_manager_state28_format,
        "MCManager_State28_Format",
    ),
    (0x0024_0c30, mc_manager_state29, "MCManager_State29"),
    (0x0024_0c90, mc_manager_state_2a, "MCManager_State2A"),
    (0x0024_0eb0, mc_manager_state_2d, "MCManager_State2D"),
    (0x0024_0ee0, mc_manager_state_2e, "MCManager_State2E"),
    (0x0024_0f30, mc_manager_state30, "MCManager_State30"),
    (
        0x0024_1000,
        mc_manager_write_save_by_name_w,
        "MCManager_WriteSaveByNameW",
    ),
    (0x0024_10a0, mc_manager_close_card, "MCManager_CloseCard"),
    (
        0x0024_1138,
        mc_manager_write_save_chunk,
        "MCManager_WriteSaveChunk",
    ),
    (
        0x0024_1180,
        mc_manager_end_progress,
        "MCManager_EndProgress",
    ),
    (
        0x0024_1200,
        mc_manager_read_save_chunk,
        "MCManager_ReadSaveChunk",
    ),
    (0x0024_1240, mc_manager_finish_load, "MCManager_FinishLoad"),
    (
        0x0024_12a0,
        mc_manager_begin_replay_write,
        "MCManager_BeginReplayWrite",
    ),
    (
        0x0024_1380,
        mc_manager_begin_slot_select,
        "MCManager_BeginSlotSelect",
    ),
    (
        0x0024_19d8,
        mc_manager_get_slot_free_space,
        "MCManager_GetSlotFreeSpace",
    ),
    (
        0x0024_19f0,
        mc_manager_format_slot_total_space,
        "MCManager_FormatSlotTotalSpace",
    ),
    (
        0x0024_1aa0,
        mc_manager_get_product_code,
        "MCManager_GetProductCode",
    ),
    (
        0x0024_1ac0,
        mc_manager_get_num_slots,
        "MCManager_GetNumSlots",
    ),
    (
        0x0024_1cd8,
        mc_manager_poll_card_status,
        "MCManager_PollCardStatus",
    ),
    (
        0x0024_1d40,
        mc_manager_get_save_type_tag,
        "MCManager_GetSaveTypeTag",
    ),
    (
        0x0024_1dc8,
        mc_manager_format_slot_name,
        "MCManager_FormatSlotName",
    ),
    (
        0x0024_1fd0,
        mc_manager_on_card_scanned,
        "MCManager_OnCardScanned",
    ),
    (
        0x0024_2050,
        mc_manager_return_to_slot_state,
        "MCManager_ReturnToSlotState",
    ),
    (0x0024_24c8, mc_manager_dtor, "MCManager_dtor"),
    (0x0024_2500, occlusion_man_ctor, "OcclusionMan_ctor"),
    (0x0024_2518, occlusion_man_dtor, "OcclusionMan_dtor"),
    (
        0x0024_2540,
        occlusion_man_add_occluder,
        "OcclusionMan_AddOccluder",
    ),
    (
        0x0024_2570,
        occlusion_man_remove_occluder,
        "OcclusionMan_RemoveOccluder",
    ),
    (
        0x0024_25c0,
        occlusion_man_build_visible,
        "OcclusionMan_BuildVisible",
    ),
    (
        0x0024_2978,
        occlusion_man_compare_by_distance,
        "OcclusionMan_CompareByDistance",
    ),
    (
        0x0024_29b0,
        occlusion_man_build_occluder_planes,
        "OcclusionMan_BuildOccluderPlanes",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CBE_GET_BE as CBE_GET_BE_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::CBE_GET_INTERFACE as CBE_GET_INTERFACE_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::F_002C26D0;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_MANAGER_END_OPERATION_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_SET_MODE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::OPERATOR_NEW_NAMED;

    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;

    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION_3;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_REQUEST_CARD_INFO_2;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_SET_MODE;

    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;

    /// `MEM_PoolNNFree(base, block)`: push the block onto the size class's
    /// free list.
    pub const POOL8_FREE: Callee = Callee {
        addr: 0x0032_0488,
        f: ee::s10::MEM_Pool8Free,
    };

    pub const POOL16_FREE: Callee = Callee {
        addr: 0x0032_0498,
        f: ee::s10::MEM_Pool16Free,
    };

    pub const POOL32_FREE: Callee = Callee {
        addr: 0x0032_04a8,
        f: ee::s10::MEM_Pool32Free,
    };

    pub const POOL64_FREE: Callee = Callee {
        addr: 0x0032_04b8,
        f: ee::s10::MEM_Pool64Free,
    };

    pub const POOL128_FREE: Callee = Callee {
        addr: 0x0032_04d0,
        f: ee::s10::MEM_Pool128Free,
    };

    pub const POOL256_FREE: Callee = Callee {
        addr: 0x0032_04e8,
        f: ee::s10::MEM_Pool256Free,
    };

    pub const POOL512_FREE: Callee = Callee {
        addr: 0x0032_0500,
        f: ee::s10::MEM_Pool512Free,
    };

    pub const REMOVE_HUD_MESSAGE: Callee = Callee {
        addr: 0x0011_79e0,
        f: ee::s00::RiderMetrix_RemoveHUDMessage,
    };

    pub const SET_TIMED_EVENT: Callee = Callee {
        addr: 0x0010_e028,
        f: ee::s00::Rider_SetTimedEvent,
    };

    pub const PACK_POSITION: Callee = Callee {
        addr: 0x0038_5a38,
        f: ee::s12::cPS2GridMesh_PackPosition,
    };

    pub const BUILD_OCCLUDER_PLANES: Callee = Callee {
        addr: 0x0024_29b0,
        f: ee::s06::OcclusionMan_BuildOccluderPlanes,
    };

    pub const MATH_DET3X3: Callee = Callee {
        addr: 0x0031_b098,
        f: ee::s10::Math_Det3x3,
    };

    /// Cross-shard calls the generator emitted through the function table.
    pub const SHOW_HUD_NUMBER: u32 = 0x0011_7b88;

    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BE_CHAR_ATTRIB_FACTOR: u32 = 0x0014_9690;

    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    pub const AUDIO_RESET_BOOST_PITCH: u32 = 0x0029_ab08;

    /// `qsort`
    pub const QSORT: u32 = 0x0041_8ef8;

    pub const VU0_UPLOAD_OCCLUDER: u32 = 0x0022_9f88;

    pub const AIR_APPLY_WIND: u32 = 0x0012_5970;

    pub const AIR_PREDICTOR_UPDATE: u32 = 0x0011_3648;

    pub const RIDER_GET_SUBSTATE: u32 = 0x0011_fee8;

    pub const RIDER_UPDATE_ORIENTATION: u32 = 0x0012_1aa0;

    pub const RIDER_ORIENTATION_IMPLICIT: u32 = 0x0011_e098;

    pub const OVERLAP_LIST_FIND_CLOSEST: u32 = 0x0033_42d0;

    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::f_00261460,
    };

    pub const LOBBY_MAN: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::f_00260f80,
    };

    pub const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::f_002613c8,
    };

    pub const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::f_0025b800,
    };

    pub const BUDDY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::f_00261408,
    };

    pub const NET_FE_SET_QUALITY_ICONS: Callee = Callee {
        addr: 0x001a_ad50,
        f: ee::s03::f_001aad50,
    };

    pub const GET_HASH_VALUE_32: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };

    pub const UI_SCREEN_FIND_OBJECT: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::f_0039d860,
    };

    pub const UI_TEXT_SET_UNICODE_STRING_BY_ID: Callee = Callee {
        addr: 0x003a_1000,
        f: ee::s13::f_003a1000,
    };

    pub const NET_FE_STATE_HANDLE_NET_EVENT: Callee = Callee {
        addr: 0x001a_8e40,
        f: ee::s03::f_001a8e40,
    };

    pub const LOBBY_MENU_ON_CREATE_ROOM_RESULT: Callee = Callee {
        addr: 0x001b_da88,
        f: ee::s03::f_001bda88,
    };

    pub const LOBBY_MENU_ON_JOIN_ROOM_RESULT: Callee = Callee {
        addr: 0x001b_d8c8,
        f: ee::s03::f_001bd8c8,
    };

    pub const NET_FE_STATE_CTOR_5: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };

    pub const UI_ENGINE_ADD_SCREEN_BY_HASH_NAME: Callee = Callee {
        addr: 0x0039_7cc0,
        f: ee::s13::f_00397cc0,
    };

    pub const UI_SCREEN_PLAY_FRAME: Callee = Callee {
        addr: 0x0039_c870,
        f: ee::s13::f_0039c870,
    };

    pub const NET_FE_STATE_SHOW_DISCONNECTED_POPUP: Callee = Callee {
        addr: 0x001a_9930,
        f: ee::s03::f_001a9930,
    };

    pub const TAG_FIELD_GET_STRING: Callee = Callee {
        addr: 0x003e_d378,
        f: ee::s14::f_003ed378,
    };

    pub const STRCPY_2: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::f_004166f4,
    };

    pub const ASYNC_FILE_RELEASE_1: Callee = Callee {
        addr: 0x003d_faf0,
        f: ee::s14::f_003dfaf0,
    };

    pub const ASYNC_FILE_RELEASE_2: Callee = Callee {
        addr: 0x003d_f9d8,
        f: ee::s14::f_003df9d8,
    };

    pub const SCRIPT_ENGINE_UNLOAD_BANK: Callee = Callee {
        addr: 0x0027_b410,
        f: ee::s07::f_0027b410,
    };

    pub const MEM_FREE: Callee = Callee {
        addr: 0x0025_23a8,
        f: ee::s06::f_002523a8,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };

    pub const SCRIPT_ENGINE_GET_INSTANCE_SCRIPT: Callee = Callee {
        addr: 0x0027_a668,
        f: ee::s07::f_0027a668,
    };

    pub const SCRIPT_ENGINE_RUN_PROCESS: Callee = Callee {
        addr: 0x0028_22a0,
        f: ee::s07::f_002822a0,
    };

    pub const SCRIPT_OBJ_ACTIVATE: Callee = Callee {
        addr: 0x0028_3430,
        f: ee::s07::f_00283430,
    };

    pub const SCRIPT_DATA_GET_INFO: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::f_0027c070,
    };

    pub const SCRIPT_ENGINE_FIND_SOUND_BANK: Callee = Callee {
        addr: 0x0027_9ef8,
        f: ee::s07::f_00279ef8,
    };

    pub const MENU_SYS_ITEM_ON_FOCUS: Callee = Callee {
        addr: 0x002c_a368,
        f: ee::s09::f_002ca368,
    };

    pub const MENU_SYS_PAGE_SET_SELECTION: Callee = Callee {
        addr: 0x002c_be68,
        f: ee::s09::f_002cbe68,
    };

    pub const MENU_SYS_PAGE_ON_ACTIVATE: Callee = Callee {
        addr: 0x002c_bf60,
        f: ee::s09::f_002cbf60,
    };

    pub const BE_SCORE_INT_UPDATE_BEST_STATS: Callee = Callee {
        addr: 0x0015_4a58,
        f: ee::s01::f_00154a58,
    };

    pub const RIDER_METRIX_RESET_STATS: Callee = Callee {
        addr: 0x0011_75b8,
        f: ee::s00::f_001175b8,
    };

    pub const MC_MANAGER_ON_OPERATION_DONE: Callee = Callee {
        addr: 0x0023_ea90,
        f: ee::s06::f_0023ea90,
    };

    pub const MC_MANAGER_END_OPERATION_4: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::f_0023ea30,
    };

    pub const MC_MANAGER_SET_MODE_2: Callee = Callee {
        addr: 0x0023_d570,
        f: ee::s06::f_0023d570,
    };

    pub const FE_POPUP_CONFIRM_RESET_LAYOUT: Callee = Callee {
        addr: 0x001c_8a38,
        f: ee::s03::f_001c8a38,
    };

    pub const FE_POPUP_CONFIRM_APPLY_CONFIG: Callee = Callee {
        addr: 0x001c_6b08,
        f: ee::s03::f_001c6b08,
    };

    pub const FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE: Callee = Callee {
        addr: 0x001c_7738,
        f: ee::s03::f_001c7738,
    };

    pub const BUILTIN_DELETE_4: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    pub const PARENT_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_7038,
        f: ee::s11::f_00357038,
    };

    pub const ROLLER_MODIFIER_CTOR: Callee = Callee {
        addr: 0x0035_da70,
        f: ee::s11::f_0035da70,
    };

    pub const MOD_OBJ_SET_TRANSFORM_MODIFIER: Callee = Callee {
        addr: 0x0035_54b0,
        f: ee::s11::f_003554b0,
    };

    pub const MULTI_SPLINE_MODIFIER_ADD_REF: Callee = Callee {
        addr: 0x0035_aad0,
        f: ee::s11::f_0035aad0,
    };

    pub const MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };

    pub const MEM_FREE_2: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS: Callee = Callee {
        addr: 0x0028_2908,
        f: ee::s07::f_00282908,
    };

    pub const C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW: Callee = Callee {
        addr: 0x0019_b7e0,
        f: ee::s03::f_0019b7e0,
    };

    pub const C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW: Callee = Callee {
        addr: 0x0019_ba60,
        f: ee::s03::f_0019ba60,
    };

    pub const STREAM_BUFFER_CTOR: Callee = Callee {
        addr: 0x002a_99d8,
        f: ee::s08::f_002a99d8,
    };

    pub const WSCRIPT_MAN_CREATE_PROCESS: Callee = Callee {
        addr: 0x0030_94c8,
        f: ee::s10::f_003094c8,
    };

    /// `WScriptMan` wrappers that turn the manager into the cache pointer
    /// (`lw(lw(man+4)+4) + section`) and call the cache accessors.
    pub const WSCRIPT_CACHE_GET_PROCESS_COUNT: Callee = Callee {
        addr: 0x003a_6c08,
        f: ee::s13::f_003a6c08,
    };

    pub const WSCRIPT_CACHE_GET_PROCESSES: Callee = Callee {
        addr: 0x003a_6c38,
        f: ee::s13::f_003a6c38,
    };

    pub const RIDER_METRIX_ON_SPLIT_MARKER: Callee = Callee {
        addr: 0x0011_94c0,
        f: ee::s00::f_001194c0,
    };

    pub const SCRIPT_CMD_QUEUE_CTOR: Callee = Callee {
        addr: 0x0028_3c30,
        f: ee::s07::f_00283c30,
    };

    pub const SCRIPT_CMD_QUEUE_CLEAR: Callee = Callee {
        addr: 0x0028_3c20,
        f: ee::s07::f_00283c20,
    };

    pub const BX_STRING_ALLOC_COPY: Callee = Callee {
        addr: 0x0031_81e8,
        f: ee::s10::f_003181e8,
    };

    pub const BX_STRING_CTOR_COPY: Callee = Callee {
        addr: 0x0031_7fe8,
        f: ee::s10::f_00317fe8,
    };

    pub const BX_STRING_DTOR: Callee = Callee {
        addr: 0x0031_8260,
        f: ee::s10::f_00318260,
    };

    pub const FONT_PARSE_GLYPHS: Callee = Callee {
        addr: 0x0039_1788,
        f: ee::s12::f_00391788,
    };

    pub const SPRINTF_2: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };

    pub const FILE_LOAD: Callee = Callee {
        addr: 0x003e_18c8,
        f: ee::s14::f_003e18c8,
    };

    pub const UI_ENGINE_STORE_WORD: Callee = Callee {
        addr: 0x0039_7138,
        f: ee::s13::f_00397138,
    };

    pub const UI_ENGINE_LOAD_TEXTURES: Callee = Callee {
        addr: 0x0039_8138,
        f: ee::s13::f_00398138,
    };

    pub const FLEX_BRIDGE_ALLOC_SPRING_NODES: Callee = Callee {
        addr: 0x0035_3f00,
        f: ee::s11::f_00353f00,
    };

    pub const CBE_GET_BE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const CBE_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const C_OV_STATE_OVERLAY_INIT: Callee = Callee {
        addr: 0x0020_a380,
        f: ee::s05::f_0020a380,
    };

    pub const EVENT_SELECT_INIT: Callee = Callee {
        addr: 0x0020_0700,
        f: ee::s05::f_00200700,
    };

    pub const C_FLAG_MANAGER_GROUP_INIT_KEY: Callee = Callee {
        addr: 0x0034_af20,
        f: ee::s11::f_0034af20,
    };

    pub const OV_SCREEN_PDA_TEMPLATE_OPEN: Callee = Callee {
        addr: 0x0020_d110,
        f: ee::s05::f_0020d110,
    };

    pub const OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS: Callee = Callee {
        addr: 0x0021_2080,
        f: ee::s05::f_00212080,
    };

    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee {
        addr: 0x0023_1250,
        f: ee::s06::f_00231250,
    };

    pub const GAME_MODULE_REQUEST_OVERLAY_STATE: Callee = Callee {
        addr: 0x0023_1278,
        f: ee::s06::f_00231278,
    };

    pub const UI_STATE_CTOR_2: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::f_0039e2a0,
    };

    pub const UI_STATE_MGR_ADD_BACKGROUND_STATE: Callee = Callee {
        addr: 0x0039_f238,
        f: ee::s13::f_0039f238,
    };

    pub const BEREWARD_INT_IS_PEAK_COMPLETE: Callee = Callee {
        addr: 0x0015_7a78,
        f: ee::s01::f_00157a78,
    };
}

// ---------------------------------------------------------------------------
// Occlusion manager.

/// `OcclusionMan` fields.
pub(crate) mod occl {

    /// Non-zero when the manager collects views at all.
    pub const ENABLED: u32 = 0x04;

    /// The visible list: `{ const View* view; float dist; }` entries.
    pub const ENTRIES: u32 = 0x08;

    /// Number of entries collected this frame.
    pub const COUNT: u32 = 0x108;

    /// Number of views that get occluder planes (at most two).
    pub const N_VISIBLE: u32 = 0x10c;

    /// Bit `i` set once view `i` was uploaded.
    pub const MASK: u32 = 0x110;
}

/// `View` fields used here: the position (w = 1) and the outward normal.
pub(crate) mod view {

    pub const POS: u32 = 0x00;

    pub const NORMAL: u32 = 0x50;

    pub const MIN: u32 = 0xa0;

    pub const MAX: u32 = 0xac;

    /// Next view in the manager's list.
    pub const NEXT: u32 = 0xbc;

    /// The four occluder planes the upload writes.
    pub const PLANES: u32 = 0x60;
}

/// Cross-shard calls the generator emitted as `call_indirect` (no alias).
pub(crate) mod table {

    pub const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;

    pub const UI_STATE_SHUTDOWN: u32 = 0x0039_e4a0;

    pub const UI_LIST_SET_FIRST_FLAGS: u32 = 0x0039_f190;

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;

    pub const SSX_AUDIO_SYSTEM_SET_GAME_STATE: u32 = 0x0028_fc58;

    pub const LOBBY_LIST_GET_NAME: u32 = 0x003e_e560;

    pub const VOIP_RPC_CALL: u32 = 0x003f_6ba0;

    pub const MC_CARD_DESTROY: u32 = 0x002c_2300;
}

// ---------------------------------------------------------------------------
// Memory-card manager.

/// `bool MCManager_Exists(void)` (0x23c850): whether the global manager pointer
/// at `gp-0x47c` is non-zero.
pub fn mc_manager_exists(c: &mut Cpu) {
    c.cycles += 3;
    let p = Addr((c.g[GP][0] as u32).wrapping_sub(0x47c)).lw(c);
    c.g[V0][0] = (0u64 < p) as u64;
    abi::ret(c);
}

/// `bool MCManager_IsIdle(MCManager *self)` (0x23c898): true when the state
/// word at `+0x130` is exactly 1.
pub fn mc_manager_is_idle(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(abi::arg(c, 0)).off(0x130).lw(c);
    c.g[V0][0] = ((state ^ 1) < 1) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Occlusion manager.

/// `OcclusionMan *OcclusionMan_ctor(OcclusionMan *self)` (0x242500): zero the
/// list head (+0), count (+4), visible count (+0x108) and occluder count
/// (+0x10c); returns `self`.
pub fn occlusion_man_ctor(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.off(0x108).sw(c, 0);
    this.off(0x10c).sw(c, 0);
    this.sw(c, 0);
    abi::ret(c);
}

/// `void OcclusionMan_AddOccluder(OcclusionMan *man, Occluder *occ)`
/// (0x242540): push `occ` onto the doubly linked list at `man+0`, setting
/// `occ->next = 0` and `occ->prev = old head`, then increment the count.
pub fn occlusion_man_add_occluder(c: &mut Cpu) {
    c.cycles += 6;
    let man = Addr(abi::arg(c, 0));
    let occ = abi::arg(c, 1);
    let head = man.lw(c);
    c.g[V0][0] = head;
    Addr(occ).off(0xc0).sw(c, 0);
    Addr(occ).off(0xbc).sw(c, head);
    let head = man.lw(c);
    c.g[V0][0] = head;
    if head != 0 {
        Addr(head as u32).off(0xc0).sw(c, occ as u64);
    }
    c.cycles += 5;
    let count = man.off(0x4).lw(c);
    c.g[V0][0] = count;
    man.sw(c, occ as u64);
    c.g[V0][0] = sx((count as u32).wrapping_add(1));
    man.off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void OcclusionMan_RemoveOccluder(OcclusionMan *man, Occluder *occ)`
/// (0x242570): unlink `occ` from the list (fixing neighbours and the head)
/// and decrement the count.
pub fn occlusion_man_remove_occluder(c: &mut Cpu) {
    c.cycles += 3;
    let man = Addr(abi::arg(c, 0));
    let occ = abi::arg(c, 1);
    let mut v1 = Addr(occ).off(0xc0).lw(c);
    if v1 == 0 {
        v1 = Addr(occ).off(0xbc).lw(c);
    } else {
        c.cycles += 3;
        let prev = Addr(occ).off(0xbc).lw(c);
        Addr(v1 as u32).off(0xbc).sw(c, prev);
        v1 = Addr(occ).off(0xbc).lw(c);
    }
    c.g[V1][0] = v1;
    c.cycles += 2;
    let mut v0;
    if v1 == 0 {
        v0 = man.lw(c);
    } else {
        c.cycles += 3;
        let next = Addr(occ).off(0xc0).lw(c);
        Addr(v1 as u32).off(0xc0).sw(c, next);
        v0 = man.lw(c);
    }
    c.cycles += 2;
    if c.g[A1][0] != v0 {
        v0 = man.off(0x4).lw(c);
    } else {
        c.cycles += 3;
        let prev = Addr(occ).off(0xbc).lw(c);
        man.sw(c, prev);
        v0 = man.off(0x4).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = sx((v0 as u32).wrapping_sub(1));
    man.off(0x4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// FPU register `f`.
#[inline(always)]
pub(crate) fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// `lwc1 f, a`, returning the value.
#[inline(always)]
pub(crate) fn ldf(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}

/// `c.le.s a, b` leaving the condition bit set.
#[inline(always)]
pub(crate) fn le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

/// Camera singleton (`gp - 0x854`) and its vtable.
pub(crate) const CAMERA: i32 = -0x854;

pub(crate) const CAMERA_VT: u32 = 0x10d8;

/// `int OcclusionMan_BuildVisible(OcclusionMan* self, const Box* bb)`
/// (0x2425c0): collect the views whose box overlaps `bb` and that are in
/// front of the camera, sort them by distance and build the occluder planes
/// of the nearest two.
pub fn occlusion_man_build_visible(c: &mut Cpu) {
    c.cycles += 13;
    let frame = Frame::open(c, 0xd0);
    frame.save(c, S2, 0x90);
    frame.save(c, S3, 0x80);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0xb0);
    c.g[S3][0] = c.g[A1][0];
    frame.save(c, S1, 0xa0);
    frame.save_ra(c, 0x70);
    frame.save_fpr(c, 20, 0xc0);
    let this = r32(c, S2);
    let bbox = r32(c, S3);
    c.sw(p(this, occl::N_VISIBLE), 0);
    c.g[V0][0] = c.lw(p(this, occl::ENABLED));
    let disabled = c.g[V0][0] == 0;
    c.sw(p(this, occl::MASK), 0);
    if !disabled {
        c.cycles += 90;
        // The camera's matrix, written to the frame by its own method.
        c.g[A2][0] = c.lw(gp(c, CAMERA));
        c.g[A0][0] = c.g[SP][0];
        let cam = r32(c, A2);
        c.g[V0][0] = c.lw(p(cam, CAMERA_VT));
        c.g[A1][0] = c.lh(p(r32(c, V0), 0x158));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x15c));
        let target = r32(c, V1);
        c.g[A1][0] = sx(cam.wrapping_add(r32(c, A1)));
        if call_ptr(c, target, 0x0024_2610).is_err() {
            return;
        }
        // The camera's row position, scaled by -1.
        c.g[A1][0] = c.lw(gp(c, CAMERA));
        let cam = r32(c, A1);
        c.g[V1][0] = c.lw(p(cam, CAMERA_VT));
        c.g[A0][0] = c.lh(p(r32(c, V1), 0x118));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x11c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(cam.wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0024_2628).is_err() {
            return;
        }
        c.lq(3, p(r32(c, V0), 0x30));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.sq(3, p(sp(c), 0x50));
        c.mfc1(4, 0);
        c.lqc2(4, p(sp(c), 0x50));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, p(sp(c), 0x60));
        c.lq(2, p(sp(c), 0x60));
        c.g[A1][0] = c.lw(gp(c, CAMERA));
        c.sq(2, p(sp(c), 0x40));
        let cam = r32(c, A1);
        c.g[V1][0] = c.lw(p(cam, CAMERA_VT));
        c.g[A0][0] = c.lh(p(r32(c, V1), 0x118));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x11c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(cam.wrapping_add(r32(c, A0)));
        if call_ptr(c, target, 0x0024_266c).is_err() {
            return;
        }
        // Rotate the (negated) row position into world space and keep it on
        // the frame as the view position.
        c.lwc1(6, p(r32(c, V0), 0x00));
        c.lwc1(1, p(r32(c, V0), 0x04));
        c.lwc1(5, p(r32(c, V0), 0x10));
        c.lwc1(10, p(r32(c, V0), 0x14));
        c.lwc1(4, p(r32(c, V0), 0x30));
        c.lwc1(11, p(r32(c, V0), 0x34));
        c.lwc1(0, p(sp(c), 0x40));
        c.lwc1(2, p(sp(c), 0x44));
        c.lwc1(7, p(r32(c, V0), 0x20));
        c.mul_s(4, 0, 4);
        c.lwc1(15, p(r32(c, V0), 0x24));
        c.mul_s(1, 2, 1);
        c.mul_s(11, 2, 11);
        c.lwc1(13, p(r32(c, V0), 0x08));
        c.mul_s(6, 0, 6);
        c.lwc1(12, p(r32(c, V0), 0x18));
        c.mul_s(5, 0, 5);
        c.lwc1(9, p(r32(c, V0), 0x38));
        c.mul_s(10, 2, 10);
        c.lwc1(3, p(sp(c), 0x48));
        c.mul_s(0, 0, 7);
        c.lwc1(16, p(r32(c, V0), 0x28));
        c.mul_s(2, 2, 15);
        c.lwc1(14, p(r32(c, V0), 0x3c));
        c.add_s(6, 6, 1);
        c.lwc1(8, p(r32(c, V0), 0x0c));
        c.mul_s(9, 3, 9);
        c.lwc1(7, p(r32(c, V0), 0x1c));
        c.mul_s(13, 3, 13);
        c.lwc1(1, p(sp(c), 0x4c));
        c.mul_s(12, 3, 12);
        c.lwc1(15, p(r32(c, V0), 0x2c));
        c.add_s(5, 5, 10);
        c.sw(p(this, occl::COUNT), 0);
        c.add_s(0, 0, 2);
        c.g[S1][0] = c.lw(this);
        c.add_s(4, 4, 11);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 17);
        c.mul_s(3, 3, 16);
        c.mul_s(14, 1, 14);
        c.mul_s(8, 1, 8);
        c.mul_s(7, 1, 7);
        c.add_s(6, 6, 13);
        c.add_s(5, 5, 12);
        c.add_s(0, 0, 3);
        c.add_s(4, 4, 9);
        c.mul_s(1, 1, 15);
        c.add_s(6, 6, 8);
        c.add_s(5, 5, 7);
        c.add_s(0, 0, 1);
        c.add_s(4, 4, 14);
        c.swc1(6, p(sp(c), 0x50));
        c.swc1(5, p(sp(c), 0x54));
        c.swc1(0, p(sp(c), 0x58));
        c.swc1(4, p(sp(c), 0x5c));
        c.lq(2, p(sp(c), 0x50));
        c.sq(2, p(sp(c), 0x40));
        let no_views = c.g[S1][0] == 0;
        c.swc1(17, p(sp(c), 0x4c));
        if !no_views {
            c.cycles += 3;
            c.mov_s(20, 17);
            c.g[S0][0] = c.g[S1][0];
            loop {
                // The view's box against `bb`, all six planes.
                c.cycles += 19;
                c.lwc1(1, bbox);
                c.lwc1(3, p(r32(c, S0), view::MIN));
                c.g[V0][0] = 0;
                c.g[S1][0] = c.lw(p(r32(c, S1), view::NEXT));
                c.swc1(3, p(sp(c), 0x50));
                c.lwc1(4, p(r32(c, S0), view::MIN + 4));
                c.swc1(4, p(sp(c), 0x54));
                c.lwc1(5, p(r32(c, S0), view::MIN + 8));
                c.swc1(20, p(sp(c), 0x5c));
                c.swc1(5, p(sp(c), 0x58));
                c.lwc1(0, p(r32(c, S0), view::MAX));
                c.swc1(0, p(sp(c), 0x60));
                c.c_le_s(1, 0);
                c.lwc1(1, p(r32(c, S0), view::MAX + 4));
                c.swc1(1, p(sp(c), 0x64));
                c.lwc1(2, p(r32(c, S0), view::MAX + 8));
                c.swc1(20, p(sp(c), 0x6c));
                let miss = !c.fcc();
                c.swc1(2, p(sp(c), 0x68));
                'overlap: {
                    if miss {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x10));
                    let f3 = fr(c, 3);
                    if !le(c, f3, f0) {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x04));
                    let f1 = fr(c, 1);
                    if !le(c, f0, f1) {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x14));
                    let f4 = fr(c, 4);
                    if !le(c, f4, f0) {
                        break 'overlap;
                    }
                    c.cycles += 4;
                    let f0 = ldf(c, 0, p(bbox, 0x08));
                    let f2 = fr(c, 2);
                    if !le(c, f0, f2) {
                        break 'overlap;
                    }
                    c.cycles += 5;
                    let f0 = ldf(c, 0, p(bbox, 0x18));
                    let f5 = fr(c, 5);
                    if le(c, f5, f0) {
                        c.g[V0][0] = 1;
                    }
                }
                // Cull test and the normal's facing test.
                'visible: {
                    c.cycles += 2;
                    let miss = c.g[V0][0] == 0;
                    c.g[T0][0] = c.lw(gp(c, CAMERA));
                    if miss {
                        break 'visible;
                    }
                    c.cycles += 11;
                    c.g[A1][0] = sx(p(sp(c), 0x50));
                    c.g[A2][0] = sx(p(sp(c), 0x60));
                    c.g[A3][0] = c.g[SP][0];
                    let cam = r32(c, T0);
                    c.g[V1][0] = c.lw(p(cam, CAMERA_VT));
                    c.g[A0][0] = c.lh(p(r32(c, V1), 0x2e8));
                    c.g[V0][0] = c.lw(p(r32(c, V1), 0x2ec));
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(cam.wrapping_add(r32(c, A0)));
                    if call_ptr(c, target, 0x0024_2830).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.g[V0][0] ^ 1;
                    if c.g[V0][0] == 0 {
                        break 'visible;
                    }
                    c.cycles += 13;
                    c.mtc1(0, 0);
                    c.lqc2(3, p(sp(c), 0x40));
                    c.lqc2(5, p(r32(c, S0), view::NORMAL));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 1);
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        break 'visible;
                    }
                    // Keep the view and its squared distance from the camera.
                    c.cycles += 20;
                    c.g[V1][0] = c.lw(p(this, occl::COUNT));
                    c.lqc2(3, p(sp(c), 0x40));
                    c.lqc2(4, p(r32(c, S0), view::POS));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 7, 5, Src::Vec(5));
                    c.vu0().adda(0b1000, 7, Src::Bc(7, Y));
                    c.vu0().madd(0b1000, 7, 6, Src::Bc(7, Z));
                    c.qmfc2(2, 7);
                    c.mtc1(2, 0);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                    c.g[V1][0] = sx(this.wrapping_add(r32(c, V1)));
                    c.sw(p(r32(c, V1), 0x08), c.g[S0][0]);
                    c.g[V0][0] = c.lw(p(this, occl::COUNT));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V0][0] = sx(this.wrapping_add(r32(c, V0)));
                    c.swc1(0, p(r32(c, V0), 0x0c));
                    c.g[V1][0] = c.lw(p(this, occl::COUNT));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    c.sw(p(this, occl::COUNT), c.g[V1][0]);
                }
                c.cycles += 2;
                let more = c.g[S1][0] != 0;
                c.g[S0][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0024_2768);
                }
            }
        }
        // Sort by distance and build the planes of the nearest two.
        c.cycles += 3;
        c.g[A1][0] = c.lw(p(this, occl::COUNT));
        let empty = c.g[A1][0] == 0;
        c.g[S0][0] = sx(p(this, occl::ENTRIES));
        if !empty {
            c.cycles += 12;
            c.g[A3][0] = 0x24_0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2978));
            c.g[A2][0] = 8;
            c.g[S1][0] = 0;
            if call_ptr(c, callee::QSORT, 0x0024_28ec).is_err() {
                return;
            }
            c.g[V0][0] = c.lw(p(this, occl::COUNT));
            c.g[A0][0] = 2;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.g[A0][0];
            }
            let none = (c.g[V0][0] as i64) <= 0;
            c.sw(p(this, occl::N_VISIBLE), c.g[V0][0]);
            if !none {
                c.cycles += 3;
                c.g[S3][0] = 1;
                c.g[V0][0] = c.lw(p(this, occl::MASK));
                loop {
                    c.cycles += 18;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) << ((c.g[S1][0] as u32) & 31));
                    c.g[A1][0] = sx(p(sp(c), 0x40));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw(p(this, occl::MASK), c.g[V0][0]);
                    c.g[A2][0] = c.lw(r32(c, S0));
                    if callee::BUILD_OCCLUDER_PLANES.call(c, 0x0024_292c).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.lw(r32(c, S0));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(view::PLANES));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(view::NORMAL));
                    if call_ptr(c, callee::VU0_UPLOAD_OCCLUDER, 0x0024_2948).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.lw(p(this, occl::N_VISIBLE));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = c.lw(p(this, occl::MASK));
                    if event_due(c) {
                        deliver_event_here(c, 0x0024_2910);
                    }
                }
            }
        }
    }
    c.cycles += 8;
    frame.restore(c, S0, 0xb0);
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore(c, S3, 0x80);
    frame.restore_ra(c, 0x70);
    frame.restore_fpr(c, 20, 0xc0);
    frame.close(c);
}

/// `int MCManager_GetSlotFreeSpace(MCManager* m, int slot)` (0x2419d8): the
/// free-space word of a memory-card slot (0xe0-byte slots, field at +0x354).
pub fn mc_manager_get_slot_free_space(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xe0;
    c.mult(0, V1, A1, V0); // v1 = slot * 0xe0
    let selfp = c.g[A0][0] as u32;
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x354));
    abi::ret(c);
}

/// `int MCManager_HasStatusError(this)` (0x0023c8d0): whether the status word
/// at `+0x43c` has any bit outside `0x4 | 0x1e0`.
pub fn mc_manager_has_status_error(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x43c)).lw(c);
    c.g[V1][0] = 0xffff_ffff_ffff_fffb;
    c.g[A0][0] = 0xffff_ffff_ffff_fe1f;
    let v = c.g[V0][0] & c.g[V1][0];
    let v = v & c.g[A0][0];
    c.g[V0][0] = v;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = (0u64 < v) as u64;
    c.pc = ra;
}

/// `void MCManager_GetProductCode(this)` (0x00241aa0): decode the product
/// code string at `D_0047C3A8` into `this`.
pub fn mc_manager_get_product_code(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A1][0] = sx(0x48_0000u32.wrapping_sub(0x3c58)),
        callee::USTR_FROM_ASCII,
        0x0024_1ab4,
    )
}

/// `void OcclusionMan_dtor(OcclusionMan* m, int flags)` (0x242518).
pub fn occlusion_man_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0024_2530))
}

/// `lw` at `base + off` (sign-extended).
#[inline(always)]
pub(crate) fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

// ---------------------------------------------------------------------------
// Memory-card manager state hooks.

/// `MCManager_CardSetState1E(this)` 0x23ca70: call entry 0x20 of the card
/// object's vtable (`*(this + 0x434)`).
pub fn mc_manager_card_set_state_1e(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let card = lw_at(c, this as u64, 0x434);
        c.g[A1][0] = card;
        let vt = c.lw(card as u32);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x100);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((card as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0023_ca90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_DestroyCard(this)` 0x23d5e8: destroy the active card object and
/// forget it.
pub fn mc_manager_destroy_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.g[A0][0] = c.lw((this as u32).wrapping_add(0x434));
        call_ptr(c, table::MC_CARD_DESTROY, 0x0023_d600)?;
        c.sw((this as u32).wrapping_add(0x434), 0);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The `MCManager_State*` entry wrappers: pass `state` as the second argument
/// to the vtable method at slot +8 of `*(this + 0x748)` (the card state
/// machine).
pub(crate) fn mc_manager_state_entry(c: &mut Cpu, state: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = state;
        let this = abi::arg(c, 0);
        frame.save_ra(c, 0);
        let vt = lw_at(c, this as u64, 0x748);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 8);
        c.g[V1][0] = m.delta;
        c.g[A2][0] = m.func;
        c.g[A0][0] = sx(this.wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_SetStateNoCard(this)` 0x23eb50: request state 4.
pub fn mc_manager_set_state_no_card(c: &mut Cpu) {
    mc_manager_state_entry(c, 4, 0x0023_eb70)
}

/// `MCManager_State13(this)` 0x23f2d8: request state 1.
pub fn mc_manager_state_13(c: &mut Cpu) {
    mc_manager_state_entry(c, 1, 0x0023_f2f8)
}

/// `MCManager_State23(this)` 0x23ffc0: request state 1.
pub fn mc_manager_state_23(c: &mut Cpu) {
    mc_manager_state_entry(c, 1, 0x0023_ffe0)
}

/// `MCManager_State2A(this)` 0x240c90: request state 6.
pub fn mc_manager_state_2a(c: &mut Cpu) {
    mc_manager_state_entry(c, 6, 0x0024_0cb0)
}

/// `MCManager_State2D(this)` 0x240eb0: request state 1.
pub fn mc_manager_state_2d(c: &mut Cpu) {
    mc_manager_state_entry(c, 1, 0x0024_0ed0)
}

/// `MCManager_State2E(this)` 0x240ee0: request state 6.
pub fn mc_manager_state_2e(c: &mut Cpu) {
    mc_manager_state_entry(c, 6, 0x0024_0f00)
}

/// `MCManager_CloseCard(this)` 0x2410a0: call slot +0x60 of the vtable at
/// `*(*(this + 0x434))` (the card's close method).
pub fn mc_manager_close_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let card = lw_at(c, this as u64, 0x434);
        c.g[A1][0] = card;
        let vt = c.lw(card as u32);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x60);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((card as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0024_10c0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MC (memory card) manager.

/// `void MCManager_RequestCardInfo(MCManager* self, int slot)` (0x23fb20):
/// remember the slot and ask the card device (virtual slot +0x2c) for its
/// info.
pub fn mc_manager_request_card_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        Addr((c.g[A0][0] as u32).wrapping_add(0x42c)).sw(c, c.g[V1][0]);
        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x434)).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_fb48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_State26(MCManager* self)` (0x23fd08): end the current
/// operation, then run the state's virtual completion hook at +0x38.
pub fn mc_manager_state26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MC_MANAGER_END_OPERATION.call(c, 0x0023_fd20)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x38)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_fd2c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_dtor(MCManager* self, int del)` (0x2424c8): install the
/// dtor vtable, clear the manager global and (when `del & 1`) free `self`.
pub fn mc_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3428));
        Addr((c.g[A0][0] as u32).wrapping_add(0x748)).sw(c, c.g[V0][0]);
        c.g[A1][0] &= 1;
        let keep = c.g[A1][0] == 0;
        Addr((c.g[GP][0] as u32).wrapping_sub(0x478)).sw(c, 0);
        if !keep {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x0024_24f0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_Destroy(void)` (0x23c860): run the manager's vtable
/// destructor and clear the global.
pub fn mc_manager_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x47c)).lw(c);
        let absent = c.g[A2][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x748).lw(c);
            c.g[A1][0] = 3;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_c888)?;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x47c)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_CardSetState18(MCManager* self)` (0x23cc20): if a card
/// object is present, call its vtable slot 0x158.
pub fn mc_manager_card_set_state18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x118).lw(c);
        let no_card = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_card {
            c.cycles += 7;
            c.g[A1][0] = Addr(abi::arg(c, 0)).off(0x434).lw(c);
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x158).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x15c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_cc4c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_RefreshSlot(MCManager* self)` (0x23fae0): reset the slot
/// cache when the current slot changed and request its card info.
pub fn mc_manager_refresh_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(abi::arg(c, 0)).off(0x428).lw(c);
        c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x444).lw(c);
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 3;
            let a0 = abi::arg(c, 0);
            let v1 = c.g[V1][0];
            Addr(a0).off(0x444).sw(c, v1);
            Addr(a0).off(0x43c).sw(c, 0);
            Addr(a0).off(0x440).sw(c, 0);
        }
        c.cycles += 5;
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(0x428).lw(c);
        callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0023_fb0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_ReadSaveChunk(MCManager* self, char* buf, int n)` (0x241200):
/// account `n` more bytes read, then pass the chunk on to the current save
/// source (virtual at `*(self+0x434)`, slot `+0xF0`).
pub fn mc_manager_read_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        let done = c.lw_r_2(V0, (this as u32).wrapping_add(0xe8));
        let source = c.lw_r_2(A3, (this as u32).wrapping_add(0x434));
        let total = sx((done as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = total;
        Addr((this as u32).wrapping_add(0xe8)).sw(c, total);
        let vt = c.lw_r_2(V1, source as u32);
        let delta = c.lh_r(A0, (vt as u32).wrapping_add(0xf0));
        let target = c.lw_r_2(V0, (vt as u32).wrapping_add(0xf4));
        let t = target as u32;
        let a0 = sx((source as u32).wrapping_add(delta as u32));
        c.g[A0][0] = a0;
        call_ptr(c, t, 0x0024_1230)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_CreateCard(MCManager* self)` (0x23d5a8): create the
/// memcard object into `+0x434` and reset the download state (`+0x104` = 0,
/// `+0x110` = 1.0, `+0x114` = the timer step at gp-0x5128).
pub fn mc_manager_create_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::PS2_MEMCARD_CREATE.call(c, 0x0023_d5bc)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lwc1(0, gp(c, -0x5128));
        c.sw(p(r32(c, S0), 0x434), c.g[V0][0]);
        c.swc1(0, p(r32(c, S0), 0x114));
        c.swc1(1, p(r32(c, S0), 0x110));
        c.sw(p(r32(c, S0), 0x104), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State02(MCManager* self)` (0x23e4f8): keep the memory-card
/// frame counter above zero; when it runs out, have the movie player show
/// its "insert card" screen.
pub fn mc_manager_state02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, A2);
        c.g[V0][0] = c.lw(p_2(this, 0x108));
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let left = (c.g[V1][0] as i64) > 0;
        sw(c, this, 0x108, c.g[V0][0]);
        if !left {
            c.cycles += 6;
            c.g[V0][0] = c.lw(p_2(this, 0x748));
            c.g[A1][0] = 3;
            c.g[A0][0] = c.lh(p_2(r32(c, V0), 0x08));
            c.g[V1][0] = c.lw(p_2(r32(c, V0), 0x0c));
            let target = r32(c, V1);
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e530)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_WriteSaveChunk(MCManager* self, int size /*a2*/)`
/// (0x241138): advance the written-byte counter, mark the save active and
/// have the movie player write the chunk.
pub fn mc_manager_write_save_chunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 1;
        c.g[V0][0] = c.lw(at(c, A0, 0xe4));
        c.g[T0][0] = c.lw(at(c, A0, 0x434));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        sw(c, r32(c, A0), 0xe4, c.g[V0][0]);
        sw(c, gp(c, -0x478), 0, c.g[A3][0]);
        c.g[V0][0] = c.lw(r32(c, T0));
        c.g[A0][0] = c.lh(p_2(r32(c, V0), 0x148));
        c.g[V1][0] = c.lw(p_2(r32(c, V0), 0x14c));
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1170)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void MCManager_ReadSaveBySlot(self, slot, size, dest, ret)` (0x23ca28):
/// ask the memory card manager's v24 read-through (`self+0x434` is the card
/// object, slot 0xe8 of its vtable) for the save in `slot` into `dest`, and
/// remember `dest` at `+0xe8`.
pub fn mc_manager_read_save_by_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        let self_ = abi::r32(c, S1);
        c.g[T0][0] = Addr(self_.wrapping_add(0x434)).lw(c);
        let card = abi::r32(c, T0);
        c.g[V0][0] = Addr(card).lw(c);
        let vtable = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vtable.wrapping_add(0xe8)).lh(c);
        c.g[V1][0] = Addr(vtable.wrapping_add(0xec)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(card.wrapping_add(abi::r32(c, A0)));
        call_ptr(c, target, 0x0023_ca58)?;
        Addr(self_.wrapping_add(0xe8)).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager` fields read by the short card screens option.
mod mc {
    /// Selects the update step; the state machine runs while it is 0.
    pub const UPDATE_STEP: u32 = 0xb8;
    /// The manager's general state timer, in updates.
    pub const WAIT: u32 = 0x108;
    /// Current state of the state machine.
    pub const STATE: u32 = 0x130;

    /// The states the option caps.
    pub mod state {
        /// `MCManager_State02` (0x23e4f8): waits 90 updates, then enters state
        /// 3, which runs the real card check.
        pub const PRE_CHECK_WAIT: u32 = 0x02;
        /// `MCManager_State06_Check` (0x23ec00): after the countdown it
        /// inherited, polls the card list that SetState(6) requested, then
        /// acts on it.
        pub const LIST_DIR: u32 = 0x06;
        /// `MCManager_State12_Save` (0x23fbb8): waits for the card to go idle
        /// and for the rest of the 60-update hold state 0x11 started, then
        /// enters state 0x13.
        pub const SAVE_FINISH: u32 = 0x12;
        /// `MCManager_State21_Load` (0x2405d0): waits for the card to go idle
        /// and for the rest of the 60-update hold state 0x20 started, then
        /// enters state 0x22.
        pub const LOAD_FINISH: u32 = 0x21;
    }
}

/// The longest these states wait with the option on: 0.25 s of updates.
const SHORT_CARD_WAIT_UPDATES: i32 = 15;

fn short_card_screens() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        ssx3_core::config::var("SSX3_SHORT_MEMCARD_SCREENS")
            .is_some_and(|v| !v.is_empty() && v != "0")
    })
}

/// The wait to use in `state`: capped in four of the states, unchanged
/// elsewhere. Negative: already expired, leave it.
fn short_card_wait(state: u32, count: i32) -> i32 {
    if matches!(
        state,
        mc::state::PRE_CHECK_WAIT
            | mc::state::LIST_DIR
            | mc::state::SAVE_FINISH
            | mc::state::LOAD_FINISH
    ) {
        count.min(SHORT_CARD_WAIT_UPDATES)
    } else {
        count
    }
}

/// Cap the manager's wait for the option, while the state machine is running.
fn shorten_card_wait(c: &mut Cpu, manager: Addr) {
    if manager.off(mc::UPDATE_STEP).lw(c) != 0 {
        return;
    }
    let state = manager.off(mc::STATE).lw(c) as u32;
    let wait = manager.off(mc::WAIT).lw(c) as i32;
    let shortened = short_card_wait(state, wait);
    if shortened != wait {
        manager.off(mc::WAIT).sw(c, shortened as u64);
    }
}

/// `void MCManager_Update(self)` (0x23d618): the memory card manager's
/// per-frame update; `+0xb8` selects the update step (0: the state machine,
/// 1: mode 1, anything else: nothing).
///
/// The states wait in 60 Hz frames (`+0x108`: 60, 90, 120 set by
/// `MCManager_v01`, counted down or up by the state handlers), so above 60 Hz
/// the update runs once per `rate / 60` ticks (`tick::frame60_due`, one clock
/// per manager): the boot "Checking for memory card" screen and the save and
/// load messages stay up as long as at 60. Every call runs at 60.
///
/// With `SSX3_SHORT_MEMCARD_SCREENS=1` (the engine's default; the headless
/// runner and the tests leave it off), states 0x02/0x06/0x12/0x21 wait at
/// most 15 more updates (about 0.25 s); the handlers still wait for the card
/// where they do card I/O.
pub fn mc_manager_update(c: &mut Cpu) {
    let self_ = abi::r32(c, A0);
    if !crate::tick::frame60_due(c, self_, 0) {
        abi::ret(c);
        return;
    }
    if short_card_screens() {
        shorten_card_wait(c, Addr(self_));
    }
    mc_manager_update_step(c);
}

/// The step of [`mc_manager_update`] (the original).
fn mc_manager_update_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V1][0] = Addr(self_.wrapping_add(0xb8)).lw(c);
        let step0 = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        if step0 {
            c.cycles += 4;
            callee::MC_MANAGER_UPDATE_STATES.call(c, 0x0023_d644)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        let step1 = c.g[V1][0] == c.g[V0][0];
        c.cycles += 2;
        frame.restore_ra(c, 0);
        if !step1 {
            // The unknown-step block charges 2 cycles and then breaks out of
            // the outer label, which lands on the epilogue's own charge: it
            // pays both.
            c.cycles += 4;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        callee::MC_MANAGER_UPDATE_MODE1.call(c, 0x0023_d654)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FormatSlotName(char* dst, int slot, int max)`
/// (0x241dc8): write the slot name (from the wide-char table at 0x4a2c28)
/// into a stack buffer, then format it into `dst` (`max + 1` chars).
pub fn mc_manager_format_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x230);
        frame.save(c, S0, 0x220);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x210);
        c.g[A1][0] = 0x4a0000;
        frame.save_ra(c, 0x200);
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = sx(0x004a_2c28);
        call_ptr(c, 0x002c_2540, 0x0024_1df0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(r32(c, S1).wrapping_add(1));
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x002c_26d0, 0x0024_1e00)?;
        frame.restore(c, S0, 0x220);
        frame.restore(c, S1, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Save-game writing.

/// `void MCManager_WriteSaveByNameW(MCManager* m, int a1, int a2, int name)`
/// (0x241000): mark the save in progress (`D_004A2C78 = 1`) and forward to the
/// memory-card manager object at `m+0x434` (vtable slot 0x128); remember the
/// name word at +0xe4. 20 cycles plus the call.
pub fn mc_manager_write_save_by_name_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let arg3 = c.g[A3][0];
        c.g[S0][0] = arg3;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let obj = c.lw(p(self_, 0x434));
        c.g[T0][0] = obj;
        c.sw(gp(c, -0x478), c.g[V0][0]);
        let vt = c.lw(obj as u32);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x12c));
        let m = VSlot::read(c, vt, 0x128);
        c.g[A0][0] = m.this(obj);
        call_ptr(c, m.func as u32, 0x0024_1038)?;
        c.sw(p(self_, 0xe4), c.g[S0][0]);
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State0C(self)` (0x23f0f8): when the operation has finished,
/// end it, request the card info and move to state 1.
pub fn mc_manager_state_0c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x6c).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0023_f114);
        call_ptr(c, target, 0x0023_f114)?;
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0023_f128);
            c.g[A1][0] = 0;
            callee::MC_MANAGER_END_OPERATION_2.call(c, 0x0023_f128)?;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x428).lw(c);
            c.g[RA][0] = sx(0x0023_f134);
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0023_f134)?;
            c.g[V0][0] = 1;
            Addr(p(r32(c, S0), 0x338)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State35(MCManager* m)` (0x23e2c8): when the movie is
/// finished, tell the 3D crowd manager to stop (message 0x33).
pub fn mc_manager_state35(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(p_3(c, S0, 0x434)).lw(c);
    c.g[V1][0] = Addr(p_3(c, A1, 0)).lw(c);
    c.g[A0][0] = Addr(p_3(c, V1, 0x58)).lh(c);
    c.g[V0][0] = Addr(p_3(c, V1, 0x5c)).lw(c);
    let target = abi::r32(c, V0);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if call_ptr(c, target, 0x0023_e2f0).is_err() {
        return;
    }
    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(p_3(c, S0, 0x748)).lw(c);
        c.g[A1][0] = 0x33;
        c.g[A0][0] = Addr(p_3(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(p_3(c, V0, 0xc)).lw(c);
        let target = abi::r32(c, V1);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0023_e310).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void MCManager_State33(MCManager* m)` (0x23e498): the same "movie
/// finished" step, with message 0x34.
pub fn mc_manager_state33(c: &mut Cpu) {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(p_3(c, S0, 0x434)).lw(c);
    c.g[V1][0] = Addr(p_3(c, A1, 0)).lw(c);
    c.g[A0][0] = Addr(p_3(c, V1, 0x58)).lh(c);
    c.g[V0][0] = Addr(p_3(c, V1, 0x5c)).lw(c);
    let target = abi::r32(c, V0);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    if call_ptr(c, target, 0x0023_e4c0).is_err() {
        return;
    }
    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(p_3(c, S0, 0x748)).lw(c);
        c.g[A1][0] = 0x34;
        c.g[A0][0] = Addr(p_3(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(p_3(c, V0, 0xc)).lw(c);
        let target = abi::r32(c, V1);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0023_e4e0).is_err() {
            return;
        }
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Memory-card manager states.

/// `void MCManager_EndOperation(MCManager* self)` (0x23ea30): clear the
/// operation flags, notify the movie player and start loading.
pub fn mc_manager_end_operation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0xbc));
        let idle = c.g[V0][0] == 0;
        sw_2(c, gp_2(c, -0x478), 0);
        if !idle {
            c.cycles += 2;
            sw_2(c, p_4(c.g[S0][0], 0xbc), 0);
            callee::MC_MANAGER_END_PROGRESS.call(c, 0x0023_ea54)?;
        }
        c.cycles += 14;
        c.g[V0][0] = 1;
        sw_2(c, p_4(c.g[S0][0], 0xc0), 0);
        sw_2(c, p_4(c.g[S0][0], 0x440), c.g[V0][0]);
        c.g[A1][0] = 1;
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x748));
        sw_2(c, p_4(c.g[S0][0], 0xc4), 0);
        c.g[A0][0] = lh(c, p_4(c.g[V0][0], 8));
        c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_ea7c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_Reset(MCManager* self)` (0x23fb58): reset the operation
/// state and put the manager back in idle mode.
pub fn mc_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw_2(c, p_4(c.g[S0][0], 0x430), c.g[V0][0]);
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x748));
        sw_2(c, p_4(c.g[S0][0], 0x42c), 0);
        sw_2(c, p_4(c.g[S0][0], 0xbc), 0);
        sw_2(c, p_4(c.g[S0][0], 0xc0), 0);
        sw_2(c, p_4(c.g[S0][0], 0xc4), 0);
        c.g[A0][0] = lh(c, p_4(c.g[V0][0], 8));
        c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_fb98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        callee::MC_MANAGER_SET_MODE.call(c, 0x0023_fba4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State1C(MCManager* self)` (0x240800): when the overlay
/// reports no busy overlay, run the state's completion hook and end the
/// operation.
pub fn mc_manager_state1c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw_2(c, p_4(c.g[S0][0], 0x434));
        c.g[V1][0] = lw_2(c, p_4(c.g[A1][0], 0));
        c.g[A0][0] = lh(c, p_4(c.g[V1][0], 0x58));
        c.g[V0][0] = lw_2(c, p_4(c.g[V1][0], 0x5c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0828)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0024_083c)?;
            let nothing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !nothing {
                c.cycles += 2;
                c.g[A1][0] = 0;
                callee::MC_MANAGER_END_OPERATION_3.call(c, 0x0024_084c)?;
            }
            c.cycles += 1;
        }
        // The original reloads s0 on both paths, before the epilogue's block.
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State29(MCManager* self)` (0x240c30): when the overlay
/// manager reports work to do, notify the movie player and request the card
/// info for the pending slot.
pub fn mc_manager_state29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x6c));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0024_0c4c)?;
        let nothing = c.g[V0][0] == 0;
        c.g[A1][0] = 1;
        if !nothing {
            c.cycles += 10;
            c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x748));
            c.g[A0][0] = lh(c, p_4(c.g[V0][0], 8));
            c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0c68)?;
            c.g[A1][0] = lw_2(c, p_4(c.g[S0][0], 0x428));
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_REQUEST_CARD_INFO_2.call(c, 0x0024_0c74)?;
            c.g[V0][0] = 1;
            sw_2(c, p_4(c.g[S0][0], 0x338), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FinishLoad(MCManager* self)` (0x241240): tell the overlay
/// manager that the load finished, notify the movie player and run the state's
/// completion hook.
pub fn mc_manager_finish_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw_2(c, p_4(c.g[S0][0], 0x434));
        c.g[V0][0] = lw_2(c, p_4(c.g[A1][0], 0));
        c.g[A0][0] = lh(c, p_4(c.g[V0][0], 0x108));
        c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0x10c));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1268)?;
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x748));
        c.g[A1][0] = 0x24;
        c.g[A0][0] = lh(c, p_4(c.g[V0][0], 8));
        c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1280)?;
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x2c));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0024_128c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MCManager_GetNumSlots(MCManager* self)` (0x241ac0): the cached slot
/// count from the overlay manager, clamped to 1 when it reports 2 or more.
pub fn mc_manager_get_num_slots(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x424));
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 9;
            c.g[A1][0] = lw_2(c, p_4(c.g[S0][0], 0x434));
            c.g[V1][0] = lw_2(c, p_4(c.g[A1][0], 0));
            c.g[A0][0] = lh(c, p_4(c.g[V1][0], 0x228));
            c.g[V0][0] = lw_2(c, p_4(c.g[V1][0], 0x22c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_1af4)?;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let small = c.g[V1][0] != 0;
            sw_2(c, p_4(c.g[S0][0], 0x424), c.g[V0][0]);
            if !small {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw_2(c, p_4(c.g[S0][0], 0x424), c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0x424));
        }
        // The original reloads s0 on both paths, before the epilogue's block.
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sort comparator and stream helpers.

/// `OcclusionMan_CompareByDistance` (0x00242978): -1 / 0 / 1 comparison of
/// the float at `+0x4` of each entry (a `qsort`-style comparator).
pub fn occlusion_man_compare_by_distance(c: &mut Cpu) {
    c.cycles += 6;
    let a = abi::arg(c, 0);
    let b = abi::arg(c, 1);
    c.lwc1(1, p(a, 4));
    c.lwc1(0, p(b, 4));
    c.c_lt_s(1, 0);
    let less = c.fcc();
    c.g[V0][0] = sx(0xffff_ffff);
    if less {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.c_lt_s(0, 1);
    let greater = c.fcc();
    c.g[V0][0] = 1;
    if greater {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void MCManager_PollCardStatus(MCManager* self)` (0x241cd8).
pub fn mc_manager_poll_card_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0024_1b20, 0x0024_1cec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x440)).lw(c);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x43c)).lw(c);
        c.g[V0][0] = c.g[V0][0] ^ c.g[A0][0];
        let no_status = c.g[V1][0] == 0;
        c.g[V0][0] = (c.g[V0][0] != 0) as u64;
        let mut changed = c.g[V0][0] != 0;
        if !no_status {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
                changed = false;
            }
        }
        if no_status || changed {
            c.cycles += 2;
            Addr(p(c.g[S0][0] as u32, 0x43c)).sw(c, c.g[A0][0]);
            if changed {
                c.cycles += 4;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xb0)).lw(c);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0024_1d20)?;
                Addr(p(c.g[S0][0] as u32, 0x440)).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x440)).lw(c);
            c.g[V0][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The memory-card manager state hooks: poll the movie object at `+0x434` and,
/// when it is idle, whether `fn` at `self + fn_off` reports busy and then set
/// the load state `state` on the `+0x748` object.
pub(crate) fn mc_manager_state(c: &mut Cpu, fn_off: u32, state: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        let obj = u32_at(c, this, 0x434);
        c.g[A1][0] = sx(obj);
        let vt = u32_at(c, obj, 0);
        c.g[V1][0] = sx(vt);
        c.g[A0][0] = Addr(p(vt, 0x58)).lh(c);
        c.g[V0][0] = lw_at_2(c, vt, 0x5c);
        let func = r32(c, V0);
        c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
        call_ptr(c, func, ret)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at_2(c, this, fn_off);
            let func = r32(c, V0);
            call_ptr(c, func, ret + 0x14)?;
            let busy = c.g[V0][0] != 0;
            c.g[A1][0] = state;
            if busy {
                c.cycles += 5;
                c.g[V0][0] = lw_at_2(c, this, 0x748);
                let vt2 = r32(c, V0);
                c.g[A0][0] = Addr(p(vt2, 8)).lh(c);
                c.g[V1][0] = lw_at_2(c, vt2, 0xc);
                let func = r32(c, V1);
                c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
                call_ptr(c, func, ret + 0x30)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_State37(self)` 0x23e3c0.
pub fn mc_manager_state37(c: &mut Cpu) {
    mc_manager_state(c, 0x6c, 3, 0x0023_e3e8)
}

/// `MCManager_State1B(self)` 0x240960.
pub fn mc_manager_state1b(c: &mut Cpu) {
    mc_manager_state(c, 0x30, 6, 0x0024_0988)
}

/// `MCManager::State38(self)` (0x23e428): when the movie player is idle and
/// the operation finished, leave the operation and return to the slot state.
pub fn mc_manager_state38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x434)).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x58)).lh(c);
        c.g[V0][0] = Addr(p(c.g[V1][0] as u32, 0x5c)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e450)?;
        let busy = c.g[V0][0] != 0;
        if busy {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x6c)).lw(c);
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_e464)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_ea30, 0x0023_e478)?;
                Addr(p(c.g[S0][0] as u32, 0xc8)).sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0024_2050, 0x0023_e484)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_ReturnToSlotState` (0x242050): tell the movie player's active
/// object to return to the memory card's slot state (2 or 3).
pub fn mc_manager_return_to_slot_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw_at(c, S0, 0x338, 0);
        c.g[A1][0] = lw_at_3(c, S0, 0x434);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x58)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5c)).lw(c);
        let busy = r32(c, V0);
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, busy, 0x0024_207c)?;
        let returning = c.g[V0][0] != 0;
        c.g[V0][0] = lw_at_3(c, S0, 0x748);
        if returning {
            c.cycles += 6;
            c.g[A1][0] = 3;
        } else {
            c.cycles += 7;
            c.g[A1][0] = 2;
        }
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
        let set = r32(c, V1);
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        call_ptr(c, set, if returning { 0x0024_20b4 } else { 0x0024_2098 })?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A 32-bit `arg` (the low word of a GPR).
#[inline(always)]
pub(crate) fn word(c: &Cpu, reg: usize) -> u32 {
    r32(c, reg)
}

/// `addu dst, a, b` of two low words, sign-extended.
#[inline(always)]
pub(crate) fn add32(a: u32, b: u32) -> u64 {
    sx(a.wrapping_add(b))
}

/// The shared tail of `MCManager_State17`/`State30`: probe the card operation
/// (three virtual calls) and finish it with `MCManager_EndOperation(self, a1)`.
pub(crate) fn mc_manager_finish(
    c: &mut Cpu,
    end_arg: u64,
    ret: u32,
    done: u32,
    probe: u32,
    end: u32,
) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = Addr(at(c, S0, 0x434)).lw(c);
    c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
    c.g[A0][0] = Addr(at(c, V1, 0x58)).lh(c);
    c.g[V0][0] = Addr(at(c, V1, 0x5c)).lw(c);
    c.g[A0][0] = add32(word(c, A1), word(c, A0));
    call_ptr(c, word(c, V0), ret)?;
    if c.g[V0][0] != 0 {
        frame.restore(c, S0, 0x10);
    } else {
        c.cycles += 5;
        c.g[V0][0] = Addr(at(c, S0, 0x84)).lw(c);
        call_ptr(c, word(c, V0), probe)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_ON_OPERATION_DONE.call(c, done)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[V0][0] = Addr(at(c, S0, 0x6c)).lw(c);
            c.cycles += 4;
            call_ptr(c, word(c, V0), probe.wrapping_add(0x20))?;
            let finished = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !finished {
                c.cycles += 2;
                c.g[A1][0] = end_arg;
                callee::MC_MANAGER_END_OPERATION_4.call(c, end)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void MCManager_State17(self)` (0x0023f258): tick the memcard manager's
/// operation 17.
pub fn mc_manager_state17(c: &mut Cpu) {
    guest(c, |c| {
        mc_manager_finish(c, 0, 0x0023_f280, 0x0023_f2a4, 0x0023_f294, 0x0023_f2c4)
    })
}

/// `void MCManager_State30(self)` (0x00240f30): tick the memcard manager's
/// operation 30.
pub fn mc_manager_state30(c: &mut Cpu) {
    guest(c, |c| {
        mc_manager_finish(c, 1, 0x0024_0f58, 0x0024_0f7c, 0x0024_0f6c, 0x0024_0f9c)
    })
}

/// `void MCManager_EndProgress(self, flag)` (0x00241180): notify the card
/// operation's owner and switch to mode 0 with the progress text cleared.
pub fn mc_manager_end_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(at(c, S0, 0x118)).lw(c);
        let absent = c.g[V0][0] == 0;
        Addr(gp(c, -0x478)).sw(c, 0);
        if !absent {
            c.cycles += 6;
            c.g[A1][0] = Addr(at(c, S0, 0x434)).lw(c);
            c.g[V0][0] = Addr(at(c, A1, 0)).lw(c);
            c.g[A0][0] = Addr(at(c, V0, 0x160)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0x164)).lw(c);
            c.g[A0][0] = add32(word(c, A1), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_11bc)?;
        }
        c.cycles += 2;
        if c.g[S1][0] != 0 {
            c.cycles += 10;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::MC_MANAGER_SET_MODE_2.call(c, 0x0024_11d0)?;
            c.g[V0][0] = Addr(at(c, S0, 0x748)).lw(c);
            c.g[A1][0] = 0x14;
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_11e8)?;
        }
        c.lq(16, frame.at(c, 0x20));
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_OnCardScanned(self)` (0x00241fd0): after a card scan,
/// update the manager's 0x33/0x35 messages.
pub fn mc_manager_on_card_scanned(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(at(c, S0, 0xac)).lw(c);
        call_ptr(c, word(c, V0), 0x0024_1fec)?;
        c.g[A1][0] = Addr(at(c, S0, 0x434)).lw(c);
        c.g[V1][0] = Addr(at(c, A1, 0)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x58)).lh(c);
        c.g[V0][0] = Addr(at(c, V1, 0x5c)).lw(c);
        c.g[A0][0] = add32(word(c, A1), word(c, A0));
        call_ptr(c, word(c, V0), 0x0024_2004)?;
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(at(c, S0, 0x748)).lw(c);
        if !done {
            c.cycles += 7;
            c.g[A1][0] = 0x33;
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_2020)?;
            c.lq(16, frame.at(c, 0x10));
        } else {
            c.cycles += 6;
            c.g[A1][0] = 0x35;
            c.g[A0][0] = Addr(at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
            c.g[A0][0] = add32(word(c, S0), word(c, A0));
            call_ptr(c, word(c, V1), 0x0024_203c)?;
            c.lq(16, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State04_NoCard(MCManager* m)` (0x0023e820): when the card
/// operation is idle, end it (or report it done).
pub fn mc_manager_state04_no_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_e848)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_e85c)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_ea30, 0x0023_e86c)?; // MCManager_EndOperation
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_e880)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_e890)?; // MCManager_OnOperationDone
                }
            }
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State05(MCManager* m)` (0x0023eb80): as state 4 but ends
/// the operation with error 1.
pub fn mc_manager_state05(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_eba8)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_ebbc)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0023_ebcc)?; // MCManager_EndOperation
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_ebe0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_ebf0)?; // MCManager_OnOperationDone
                }
            }
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State15(MCManager* m)` (0x0023f1d8): as state 4 but ending
/// the operation with error 0.
pub fn mc_manager_state15(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_f200)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0023_f214)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_ea30, 0x0023_f224)?; // MCManager_EndOperation
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_f238)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_f248)?; // MCManager_OnOperationDone
                }
            }
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State10_Save(MCManager* m)` (0x0023f4f8): update the save
/// progress fraction, then finish the progress bar and kick the save (or
/// report that it finished).
pub fn mc_manager_state10_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
        c.cvt_s_w(0, 0);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xf0));
        c.cvt_s_w(1, 1);
        c.div_s(0, 0, 1);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0023_f538)?;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0024_1180, 0x0023_f54c)?; // MCManager_EndProgress
        } else {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.cycles += 6;
            c.g[A1][0] = 0x11;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x0023_f568)?;
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_State1F_Load(MCManager* m)` (0x002400b0): when the card is
/// ready, start the load and the progress bar; otherwise finish it.
pub fn mc_manager_state1f_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x248));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x0024_00d8)?;
        let not_ready = c.g[V0][0] == 0;
        c.g[A1][0] = 0x24;
        if not_ready {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A1][0] = 0x20;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x0024_0120)?;
        } else {
            c.cycles += 10;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let target = c.g[V1][0] as u32;
            call_ptr(c, target, 0x0024_00f4)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
            let target = c.g[V0][0] as u32;
            call_ptr(c, target, 0x0024_0100)?;
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `void MCManager_BeginSlotSelect(MCManager* m)` (0x00241380): reset the
/// slot-selection state, select slot 0 and enter the "choose a slot" state.
pub fn mc_manager_begin_slot_select(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[V0][0] = sx(0xffff_ffff);
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x340), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xdc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x42c), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x444), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x430), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xfc), c.g[V0][0]);
        call_ptr(c, 0x0023_fb18, 0x0024_13c0)?; // MCManager_SetSlot
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fae0, 0x0024_13c8)?; // MCManager_RefreshSlot
        c.g[V0][0] = 2;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x338), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x130), c.g[V0][0]);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let target = c.g[V1][0] as u32;
        call_ptr(c, target, 0x0024_13ec)?;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `char* MCManager_GetSaveTypeTag(MCManager* m, int type, UStr* out)`
/// (0x00241d40): the English tag for a save type ("new"/"current"/"other"),
/// built into `out`.
pub fn mc_manager_get_save_type_tag(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
        c.g[A0][0] = c.g[A2][0];
        c.g[V0][0] = 1;
        let one = c.g[A1][0] == c.g[V0][0];
        c.sd(sp(c), c.g[RA][0]);
        if one {
            // `break 'b_241d94`: the "current" tag.
            c.cycles += 5;
            c.g[A1][0] = sx(0x004a_2cb0);
            call_ptr(c, 0x002c_2540, 0x0024_1da0)?; // UStr_FromAscii_2
            c.g[RA][0] = c.ld(sp(c));
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let two = c.g[A1][0] == c.g[V0][0];
                c.g[RA][0] = c.ld(sp(c));
                if two {
                    // `break 'b_241da8`: the "other" tag.
                    c.cycles += 4;
                    c.g[A1][0] = sx(0x004a_2cb8);
                    call_ptr(c, 0x002c_2540, 0x0024_1db4)?; // UStr_FromAscii_2
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let zero = c.g[A1][0] == 0;
                c.g[RA][0] = c.ld(sp(c));
                if zero {
                    // `break 'b_241d80`: the "new" tag.
                    c.cycles += 5;
                    c.g[A1][0] = sx(0x004a_2ca8);
                    call_ptr(c, 0x002c_2540, 0x0024_1d8c)?; // UStr_FromAscii_2
                    c.g[RA][0] = c.ld(sp(c));
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card manager states.

/// `void MCManager_State14(MCManager* self)` (0x23f150): poll the card slot
/// (`+0x434`), then either end the pending operation (`+0x6c`) or report the
/// completed one (`+0x84`).
pub fn mc_manager_state14(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0];
        c.g[A1][0] = lw_at_4(c, this, 0x434);
        let mgr = c.g[A1][0];
        c.g[V1][0] = lw_at_4(c, mgr, 0);
        let vt = c.g[V1][0];
        c.g[A0][0] = lh_at(c, vt, 0x58);
        c.g[V0][0] = lw_at_4(c, vt, 0x5c);
        let func = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at_2(mgr as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0023_f178)?;
        if c.g[V0][0] != 0 {
            c.lq(16, at_2(sp0, 0x10));
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_at_4(c, this, 0x6c);
            let func = c.g[V0][0] as u32;
            call_ptr(c, func, 0x0023_f18c)?;
            let busy = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            let mut finished = false;
            if busy {
                c.cycles += 4;
                c.g[A1][0] = 0;
                callee::MC_END_OPERATION.call(c, 0x0023_f19c)?;
                c.lq(16, at_2(sp0, 0x10));
                finished = true;
            }
            if !finished {
                c.cycles += 5;
                c.g[V0][0] = lw_at_4(c, this, 0x84);
                let func = c.g[V0][0] as u32;
                call_ptr(c, func, 0x0023_f1b0)?;
                let ok = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                if !ok {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(at_2(this as u32, 0xbc), c.g[V0][0]);
                    callee::MC_ON_OPERATION_DONE.call(c, 0x0023_f1c4)?;
                    c.sw(at_2(this as u32, 0x440), 0);
                }
                c.cycles += 1;
                c.lq(16, at_2(sp0, 0x10));
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Movie player and misc state hooks.

/// `void MCManager_State25(MCManager* self)` (0x23fe70): poll the current
/// operation object; when it is finished, end the operation (or report it
/// done), otherwise call the pending callback.
pub fn mc_manager_state25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        c.g[A1][0] = c.lw(p(self_, 0x434));
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = c.lh(at(c, V1, 0x58));
        c.g[V0][0] = c.lw(at(c, V1, 0x5c));
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        let target = r32(c, V0);
        call_ptr(c, target, 0x0023_fe98)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            let mut done = false;
            c.cycles += 5;
            c.g[V0][0] = c.lw(at(c, S0, 0x6c));
            let target = r32(c, V0);
            call_ptr(c, target, 0x0023_feac)?;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if idle {
                c.cycles += 5;
                c.g[V0][0] = c.lw(at(c, S0, 0x84));
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_fed0)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(at(c, S0, 0x68));
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0023_fee0)?;
                    done = true;
                }
            } else {
                c.cycles += 4;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0023_febc)?;
                done = true;
            }
            if done {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 3;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_fef0)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Shared body of the two `MCManager_State*` movie checks: if the movie
/// finished and the second flag is set, send the "next" event.
pub(crate) fn mc_manager_state_check(c: &mut Cpu, entry: u32, first: u32, second: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[A1][0] = lw_3(c, S0, 0x434);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
    c.g[V0][0] = lw_3(c, V1, 0x5c);
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, c.g[V0][0] as u32, entry + 0x28)?;
    let mut late = false;
    if c.g[V0][0] == 0 {
        c.cycles += 8;
        c.g[A1][0] = lw_3(c, S0, 0x434);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
        c.g[V0][0] = lw_3(c, V1, 0x24c);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V0][0] as u32, entry + 0x48)?;
        c.g[A1][0] = sx(first);
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = lw_3(c, S0, 0x748);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
            c.g[V1][0] = lw_3(c, V0, 0xc);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, entry + 0x64)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        late = true;
    }
    if late {
        c.cycles += 7;
        c.g[V0][0] = lw_3(c, S0, 0x748);
        c.g[A1][0] = sx(second);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = lw_3(c, V0, 0xc);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[V1][0] as u32, entry + 0x84)?;
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void MCManager_State0E(self, void* unused)` (0x23f3a0): after the movie
/// finishes, ask the movie object for its sub-state and post event 0x14 or
/// 0xF.
pub fn mc_manager_state0e(c: &mut Cpu) {
    guest(c, |c| mc_manager_state_check(c, 0x0023_f3a0, 0x14, 0xf))
}

/// `void MCManager_State19_Delete(self, void* unused)` (0x240768): the delete
/// step's version of the same movie check (events 0x1C and 0x1A).
pub fn mc_manager_state19_delete(c: &mut Cpu) {
    guest(c, |c| mc_manager_state_check(c, 0x0024_0768, 0x1c, 0x1a))
}

/// `MCManager_State0D` (0x23f308): ask the MC manager's object two virtual
/// questions and send state 0xe (or 0xf) to the result object.
pub fn mc_manager_state_0d(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);

        c.g[A2][0] = Addr(p(this, 0x434)).lw(c);
        c.g[A1][0] = Addr(p(this, 0xf8)).lw(c);
        c.g[V1][0] = Addr(r(c, A2)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x218)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x21c)).lw(c);
        let first_target = r(c, V0);
        c.g[RA][0] = sx(0x0023_f334);
        c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
        call_ptr(c, first_target, 0x0023_f334)?;

        let handled = c.g[V0][0] != 0;
        let state = if handled {
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            0xe
        } else {
            c.cycles += 9;
            c.g[A2][0] = Addr(p(this, 0x434)).lw(c);
            c.g[A1][0] = Addr(p(this, 0xf8)).lw(c);
            c.g[V1][0] = Addr(r(c, A2)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V1), 0x208)).lh(c);
            c.g[V0][0] = Addr(p(r(c, V1), 0x20c)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0023_f358);
            c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0023_f358)?;
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            if c.g[V0][0] == 0 {
                0xf
            } else {
                0xe
            }
        };
        if state == 0xe {
            c.cycles += 7;
        } else {
            c.cycles += 6;
        }
        c.g[A1][0] = state;
        c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
        c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
        let target = r(c, V1);
        c.g[RA][0] = sx(if state == 0xe {
            0x0023_f374
        } else {
            0x0023_f390
        });
        c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
        call_ptr(
            c,
            target,
            if state == 0xe {
                0x0023_f374
            } else {
                0x0023_f390
            },
        )?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MCManager_State24` (0x23fc70): the manager's 0x24 state: poll the input
/// object, then send 3 or 6 to the result object.
pub fn mc_manager_state_24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A1][0] = Addr(p(this, 0x434)).lw(c);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fc98);
        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0023_fc98)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 5;
        c.g[V0][0] = Addr(p(this, 0x6c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fcac);
        call_ptr(c, target, 0x0023_fcac)?;
        let zero = c.g[V0][0] == 0;
        c.g[A1][0] = 3;
        if !zero {
            c.cycles += 7;
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
            let target = r(c, V1);
            c.g[RA][0] = sx(0x0023_fcc8);
            c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0023_fcc8)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 5;
        c.g[V0][0] = Addr(p(this, 0x84)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fcdc);
        call_ptr(c, target, 0x0023_fcdc)?;
        let zero = c.g[V0][0] == 0;
        c.g[A1][0] = 6;
        if !zero {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(this, 0x748)).lw(c);
            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
            let target = r(c, V1);
            c.g[RA][0] = sx(0x0023_fcf8);
            c.g[A0][0] = sx(this.wrapping_add(r(c, A0)));
            call_ptr(c, target, 0x0023_fcf8)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State36(self)` (0x23e320): if the mem-card movie has no
/// pending scan, ask it whether this card is in the movie; either report the
/// card as scanned or run the scan callback.
pub fn mc_manager_state36(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[A1][0] = lw_4(c, S0, 0x434);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = lw_4(c, V1, 0x5c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e348)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            c.g[A2][0] = lw_4(c, S0, 0x434);
            c.g[A1][0] = lw_4(c, S0, 0x428);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
            c.g[V0][0] = lw_4(c, V1, 0x1a4);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e36c)?;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = 2;
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_ON_CARD_SCANNED.call(c, 0x0023_e3a4)?;
                c.g[V0][0] = lw_4(c, S0, 0x98);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_e3b0)?;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 10;
                c.g[V0][0] = lw_4(c, S0, 0x748);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = lw_4(c, V0, 0xc);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_e388)?;
                c.g[V0][0] = lw_4(c, S0, 0x9c);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x0023_e394)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card manager formatting helpers.

/// `void MCManager_State0B_Format(MCManager* self)` (0x23f050): count the
/// outstanding formats down; at_4 zero, rebuild the status text and the prompt.
pub fn mc_manager_state0b_format(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid: {
                c.cycles += 9;
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = lw_5(c, at_4(c.g[S0][0], 0x108));
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = sx(at_4(c.g[V0][0], 0xffff_ffff));
                let t = (c.g[V1][0] as i64) > 0;
                Addr(at_4(c.g[S0][0], 0x108)).sw(c, c.g[V0][0]);
                if t {
                    break 'mid;
                }
                c.cycles += 8;
                c.g[A1][0] = lw_5(c, at_4(c.g[S0][0], 0x434));
                c.g[V1][0] = lw_5(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0x58));
                c.g[V0][0] = lw_5(c, at_4(c.g[V1][0], 0x5c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023_f08c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f08c)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                'retry: {
                    c.cycles += 8;
                    c.g[A1][0] = lw_5(c, at_4(c.g[S0][0], 0x434));
                    c.g[V1][0] = lw_5(c, c.g[A1][0] as u32);
                    c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0x48));
                    c.g[V0][0] = lw_5(c, at_4(c.g[V1][0], 0x4c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023_f0ac);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f0ac)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0xc;
                    if t {
                        break 'retry;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = lw_5(c, at_4(c.g[S0][0], 0x748));
                    c.g[A0][0] = lh(c, at_4(c.g[V0][0], 8));
                    c.g[V1][0] = lw_5(c, at_4(c.g[V0][0], 0xc));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023_f0c8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f0c8)?;
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 6;
                c.g[V0][0] = lw_5(c, at_4(c.g[S0][0], 0x748));
                c.g[A1][0] = 0xd;
                c.g[A0][0] = lh(c, at_4(c.g[V0][0], 8));
                c.g[V1][0] = lw_5(c, at_4(c.g[V0][0], 0xc));
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0023_f0e8);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f0e8)?;
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State28_Format(MCManager* self)` (0x240b88): the same for
/// the second card state (prompt ids 0x29 / 0x2a).
pub fn mc_manager_state28_format(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'mid: {
                c.cycles += 9;
                frame.save(c, S0, 0x10);
                frame.save_ra(c, 0);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = lw_5(c, at_4(c.g[S0][0], 0x108));
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = sx(at_4(c.g[V0][0], 0xffff_ffff));
                let t = (c.g[V1][0] as i64) > 0;
                Addr(at_4(c.g[S0][0], 0x108)).sw(c, c.g[V0][0]);
                if t {
                    break 'mid;
                }
                c.cycles += 8;
                c.g[A1][0] = lw_5(c, at_4(c.g[S0][0], 0x434));
                c.g[V1][0] = lw_5(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0x58));
                c.g[V0][0] = lw_5(c, at_4(c.g[V1][0], 0x5c));
                let target = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0024_0bc4);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0bc4)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                'retry: {
                    c.cycles += 8;
                    c.g[A1][0] = lw_5(c, at_4(c.g[S0][0], 0x434));
                    c.g[V1][0] = lw_5(c, c.g[A1][0] as u32);
                    c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0x48));
                    c.g[V0][0] = lw_5(c, at_4(c.g[V1][0], 0x4c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0024_0be4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0be4)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x29;
                    if t {
                        break 'retry;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = lw_5(c, at_4(c.g[S0][0], 0x748));
                    c.g[A0][0] = lh(c, at_4(c.g[V0][0], 8));
                    c.g[V1][0] = lw_5(c, at_4(c.g[V0][0], 0xc));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0024_0c00);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0c00)?;
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 6;
                c.g[V0][0] = lw_5(c, at_4(c.g[S0][0], 0x748));
                c.g[A1][0] = 0x2a;
                c.g[A0][0] = lh(c, at_4(c.g[V0][0], 8));
                c.g[V1][0] = lw_5(c, at_4(c.g[V0][0], 0xc));
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0024_0c20);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0c20)?;
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cMCOverlayManager_GetDeviceTotalString` (0x2419f0): format one memory-card
// device's total space into a wide buffer. The device record is at
// `self + a1 * 0xe0`; a missing record, or one with any of three optional
// strings null, leaves the destination untouched.
pub fn mc_manager_format_slot_total_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x450);
        frame.save(c, S1, 0x430);
        c.g[V0][0] = 0xe0;
        c.mult(0, 17, 5, 2);
        frame.save(c, S2, 0x420);
        frame.save(c, S3, 0x410);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x440);
        frame.save_ra(c, 0x400);
        let dev = (c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32);
        c.g[V0][0] = sx(dev);
        c.g[V1][0] = c.lw(dev.wrapping_add(0x350));
        let missing = c.g[V1][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        // 7 cycles for the two skipped block tails, 6 for the middle one, 5
        // for the outer one (see the generated nesting).
        let mut tail = 7u64;
        'checks: {
            if missing {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(dev.wrapping_add(0x344));
            let missing = c.g[V1][0] == 0;
            c.lq(16, frame.at(c, 0x440));
            if missing {
                tail = 6;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(dev.wrapping_add(0x348));
            if c.g[V1][0] == 0 {
                c.lq(17, frame.at(c, 0x430));
                tail = 5;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(dev.wrapping_add(0x34c));
            let missing = c.g[V1][0] == 0;
            c.g[A1][0] = 0x4a0000;
            if missing {
                tail = 6;
                break 'checks;
            }
            // Two wide strings then the formatted line.
            c.cycles += 14;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ca0));
            call_ptr(c, 0x002c2540, 0x0024_1a54)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c98));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c2540, 0x0024_1a68)?;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x358));
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, 0x002c26d0, 0x0024_1a80)?;
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x440);
        frame.restore(c, S1, 0x430);
        frame.restore(c, S2, 0x420);
        frame.restore(c, S3, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Media control manager state transitions.

/// `void MCManager_State12_Save(MCManager* m)` (0x23fbb8): drop the pending
/// action count, ask the device whether it is busy and otherwise mark the
/// save finished.
pub fn mc_manager_state12_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x108)).lw(c);
        c.g[A1][0] = Addr(p(this, 0x434)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        Addr(p(this, 0x108)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0023_fbec);
        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0023_fbec)?;
        'outer: {
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            'busy: {
                c.cycles += 8;
                c.g[A1][0] = Addr(p(r(c, S0), 0x434)).lw(c);
                c.g[V1][0] = Addr(r(c, A1)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V1), 0x248)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0x24c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0023_fc0c);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0023_fc0c)?;
                let idle = c.g[V0][0] == 0;
                c.g[A1][0] = 0x14;
                if idle {
                    break 'busy;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(p(r(c, S0), 0x748)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x0023_fc28);
                c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                call_ptr(c, target, 0x0023_fc28)?;
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, S0), 0x108)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0023_fc48);
            call_ptr(c, target, 0x0023_fc48)?;
            c.g[V1][0] = Addr(p(r(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 0x13;
            c.g[A0][0] = Addr(p(r(c, V1), 8)).lh(c);
            c.g[V0][0] = Addr(p(r(c, V1), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0023_fc60);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x0023_fc60)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State21_Load(MCManager* m)` (0x2405d0): the load-side twin
/// of [`mc_manager_state12_save`].
pub fn mc_manager_state21_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x108)).lw(c);
        c.g[A1][0] = Addr(p(this, 0x434)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_sub(1));
        Addr(p(this, 0x108)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
        let target = r(c, V0);
        c.g[RA][0] = sx(0x0024_0604);
        c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0024_0604)?;
        'outer: {
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            'busy: {
                c.cycles += 8;
                c.g[A1][0] = Addr(p(r(c, S0), 0x434)).lw(c);
                c.g[V1][0] = Addr(r(c, A1)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V1), 0x248)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0x24c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0024_0624);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0024_0624)?;
                let idle = c.g[V0][0] == 0;
                c.g[A1][0] = 0x24;
                if idle {
                    break 'busy;
                }
                c.cycles += 10;
                c.g[V0][0] = Addr(p(r(c, S0), 0x748)).lw(c);
                c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                let target = r(c, V1);
                c.g[RA][0] = sx(0x0024_0640);
                c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                call_ptr(c, target, 0x0024_0640)?;
                c.g[V0][0] = Addr(p(r(c, S0), 0x2c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0024_064c);
                call_ptr(c, target, 0x0024_064c)?;
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, S0), 0x108)).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                frame.restore(c, 16, 0x10);
                break 'outer;
            }
            c.cycles += 7;
            c.g[V0][0] = Addr(p(r(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 0x22;
            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
            let target = r(c, V1);
            c.g[RA][0] = sx(0x0024_0678);
            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
            call_ptr(c, target, 0x0024_0678)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State0F_Save(MCManager* self)` (0x23f438): ask the
/// movie/stream object at_5 +0x434 (three virtual predicates) whether a save is
/// due and forward the resulting state id (0x37 / 0x14 / 0x10) to the player
/// slot of `self->0x748`.
pub fn mc_manager_state0f_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        open(c, 0x20);
        c.sq(16, at_5(c.g[SP][0] as u32, 0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A1][0] = Addr(this as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f460)?;
        if c.g[V0][0] != 0 {
            c.lq(16, at_5(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A1][0] = Addr(this as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f480)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this as u32).off(0x748).lw(c);
            c.cycles += 6;
            c.g[A1][0] = 0x10;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_f4e4)?;
            c.lq(16, at_5(c.g[SP][0] as u32, 0x10));
            c.cycles += 3;
            epilogue(c, 0, 0x20);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A1][0] = Addr(this as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x250).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x254).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f4a0)?;
        let no_save = c.g[V0][0] == 0;
        c.g[A1][0] = 0x37;
        if no_save {
            c.g[V0][0] = Addr(this as u32).off(0x748).lw(c);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(this as u32).off(0x748).lw(c);
            c.g[A1][0] = 0x14;
        }
        c.cycles += 6;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f4c8)?;
        c.lq(16, at_5(c.g[SP][0] as u32, 0x10));
        c.cycles += 3;
        epilogue(c, 0, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MCManager_OnOperationDone` (0x0023ea90).
//
// `void MCManager::OnOperationDone(MCManager* self)`: finish a memory-card
// operation, refresh the slot and return to the slot state.

pub fn mc_manager_on_operation_done(c: &mut Cpu) {
    guest(c, |c| {
        'b_23eb44: {
            'b_23eb18: {
                'b_23eb00: {
                    'b_23eadc: {
                        c.cycles += 7;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[S0][0] = c.g[A0][0];
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        if c.g[V0][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x12c));
                            break 'b_23eadc;
                        } else {
                            'b_23ead0: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x478), 0u64);
                                if t {
                                    break 'b_23ead0;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                                    c.g[V1][0] = vslot(c, c.g[V0][0], 0x160, V1);
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0023ead0);
                                    c.g[A0][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0023ead0)?;
                                    break 'b_23ead0;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                            let t = true;
                            c.g[A1][0] = 0x2;
                            if t {
                                break 'b_23eb00;
                            } else {
                                break 'b_23eadc;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x440), 0u64);
                    if t {
                        break 'b_23eb18;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x0023eaf0);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023fb18, 0x0023eaf0)?;
                        c.g[RA][0] = sx(0x0023eaf8);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023fae0, 0x0023eaf8)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                        c.g[A1][0] = 0x31;
                        break 'b_23eb00;
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0023eb10);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0023eb10)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    break 'b_23eb44;
                } else {
                    break 'b_23eb18;
                }
            }
            'b_23eb38: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0023eb20);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023c8d0, 0x0023eb20)?;
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_23eb38;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0023eb30);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00241fd0, 0x0023eb30)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_23eb44;
                    } else {
                        break 'b_23eb38;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x0023eb40);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00242050, 0x0023eb40)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_23eb44;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MCManager_State22_Load` (0x0023ff00).
//
// `void MCManager::State22_Load(MCManager* self)`: poll the memory-card
// operation and either finish it or start the next step.

pub fn mc_manager_state22_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_23ffb4: {
            'b_23ffb0: {
                'b_23ff24: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x108));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    if t {
                        break 'b_23ff24;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                        if t {
                            break 'b_23ffb0;
                        } else {
                            break 'b_23ff24;
                        }
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = vslot(c, c.g[V1][0], 0x58, V0);
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023ff3c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0023ff3c)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    break 'b_23ffb4;
                } else {
                    'b_23ff8c: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[V0][0] = vslot(c, c.g[V1][0], 0x248, V0);
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0023ff5c);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0023ff5c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x24;
                        if t {
                            break 'b_23ff8c;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                            c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0023ff78);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0023ff78)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0023ff84);
                            call_ptr(c, t, 0x0023ff84)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_23ffb4;
                            } else {
                                break 'b_23ff8c;
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                    c.g[A1][0] = 0x23;
                    c.g[V1][0] = vslot(c, c.g[V0][0], 0x8, V1);
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0023ffa4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0023ffa4)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023ffb0);
                    call_ptr(c, t, 0x0023ffb0)?;
                    break 'b_23ffb0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_23ffb4;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card manager.

/// `void MCManager_State27_Format(MCManager* self)` (0x240ab0): state 27;
/// poll the card slot and step the format operation.
pub fn mcmanager_state27_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // slot 0x58 on the card interface
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x5c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_0ad8)?;
        if c.g[V0][0] != 0 {
            c.lq(S0, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        // slot 0x1a8
        c.cycles += 8;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x1a8)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x1ac)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_0af8)?;
        let mut join_b20 = false;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(p(r32(c, S0), 0x88)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 4;
            join_b20 = true;
        }
        if !join_b20 {
            c.cycles += 4;
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0b14)?;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
                join_b20 = true;
            } else {
                c.g[A1][0] = 0x28;
            }
        }
        if join_b20 {
            c.cycles += 6;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x0024_0b30)?;
            c.lq(S0, frame.at(c, 0x10));
        } else {
            // poll the 0x6c callback
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6c)).lw(c);
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0b44)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0024_0b54)?;
                c.lq(S0, frame.at(c, 0x10));
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x84)).lw(c);
                let f = r32(c, V0);
                call_ptr(c, f, 0x0024_0b68)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_ea90, 0x0024_0b78)?;
                }
                c.lq(S0, frame.at(c, 0x10));
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State18(MCManager* self)` (0x240688): state 18; poll the
/// card slot, then the pending-operation and completion callbacks.
pub fn mcmanager_state18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x58)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x5c)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_06b0)?;
        if c.g[V0][0] != 0 {
            c.lq(S0, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x1a8)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x1ac)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, f, 0x0024_06d0)?;
        let mut join = false;
        if c.g[V0][0] != 0 {
            c.g[A1][0] = Addr(p(r32(c, S0), 0x434)).lw(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
            c.g[A1][0] = 4;
            join = true;
        }
        if !join {
            c.cycles += 7;
            c.g[V1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0x248)).lh(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x24c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
            call_ptr(c, f, 0x0024_06f8)?;
            let zero = c.g[V0][0] == 0;
            c.g[A1][0] = 0x1c;
            if !zero {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
                join = true;
            }
        }
        if !join {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x68)).lw(c);
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0714)?;
            let zero = c.g[V0][0] == 0;
            c.g[A1][0] = 0x19;
            if !zero {
                c.cycles += 1;
                c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
                join = true;
            }
        }
        if join {
            c.cycles += 6;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x0024_0730)?;
            c.lq(S0, frame.at(c, 0x10));
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x6c)).lw(c);
            let f = r32(c, V0);
            call_ptr(c, f, 0x0024_0744)?;
            c.g[A0][0] = c.g[S0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 1;
                call_ptr(c, 0x0023_ea30, 0x0024_0754)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MCManager_BeginReplayWrite (0x00002412a0).

/// `MCManager_BeginReplayWrite`: compose the "please insert a memory card" text and start the replay write flow.
pub fn mc_manager_begin_replay_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x590));
        c.g[V0][0] = 0x1;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x570));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x580));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x560));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x550));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x540));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x530));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x520), c.g[RA][0]);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x478), c.g[V0][0]);
        call_ptr(c, 0x0014dc80, 0x002412d0)?;
        c.g[A1][0] = 0x5;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014dd58, 0x002412dc)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00152758, 0x002412e4)?;
        c.g[S5][0] = c.g[V0][0];
        call_ptr(c, 0x00152948, 0x002412ec)?;
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x320));
        c.g[A1][0] = 0x480000;
        c.g[S4][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3e88));
        call_ptr(c, 0x002c2540, 0x00241304)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[A3][0] = 0x82;
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00241d40, 0x0024131c)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[A1][0] = 0x200;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00241aa0, 0x0024132c)?;
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x002c26d0, 0x00241348)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x00241000, 0x0024135c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x580));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x570));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x560));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x550));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x540));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x530));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x520));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x590));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// MCManager_State0A_Format (0x000023ef68).

/// `MCManager_State0A_Format`: memory-card manager state 0xa: format the card and report the result.
pub fn mc_manager_state0a_format(c: &mut Cpu) {
    guest(c, |c| {
        'b_23f040: {
            'b_23f004: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x338));
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    break 'b_23f004;
                } else {
                    'b_23f000: {
                        'b_23efe8: {
                            'b_23efb0: {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[RA][0] = sx(0x0023ef9c);
                                c.pc = t;
                                ssx3_ee_rt::funcs::call_indirect(c, t);
                                if c.pc != 0x0023ef9c {
                                    return Err(crate::call::Unwound);
                                }
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                    break 'b_23efb0;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                    let t = true;
                                    c.g[A1][0] = 0x4;
                                    if t {
                                        break 'b_23efe8;
                                    } else {
                                        break 'b_23efb0;
                                    }
                                }
                            }
                            'b_23efd4: {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[RA][0] = sx(0x0023efc4);
                                c.pc = t;
                                ssx3_ee_rt::funcs::call_indirect(c, t);
                                if c.pc != 0x0023efc4 {
                                    return Err(crate::call::Unwound);
                                }
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0xc;
                                if t {
                                    break 'b_23efd4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                    if t {
                                        break 'b_23efe8;
                                    } else {
                                        break 'b_23efd4;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0xb;
                            if t {
                                break 'b_23f000;
                            } else {
                                c.cycles += 2;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                break 'b_23efe8;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[RA][0] = sx(0x0023eff8);
                        c.pc = t;
                        ssx3_ee_rt::funcs::call_indirect(c, t);
                        if c.pc != 0x0023eff8 {
                            return Err(crate::call::Unwound);
                        }
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_23f040;
                        } else {
                            break 'b_23f000;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    break 'b_23f004;
                }
            }
            'b_23f024: {
                c.cycles += 4;
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023f00c);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x0023f00c {
                    return Err(crate::call::Unwound);
                }
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_23f024;
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0023ea30, 0x0023f01c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_23f040;
                    } else {
                        break 'b_23f024;
                    }
                }
            }
            'b_23f03c: {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023f030);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x0023f030 {
                    return Err(crate::call::Unwound);
                }
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_23f03c;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    break 'b_23f03c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_23f040;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Memory card manager.

/// `void MCManager_State09(MCManager* m)` (0x2409c8): the memory-card
/// manager's "check" state. The first sub-object query sets the next state
/// directly; otherwise the idle/defer query runs, then the two condition
/// callbacks, and a non-zero result selects state `1` or `0xd`.
pub fn mcmanager_state09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];

        // if (m->sub434->v11()) return;
        c.g[A1][0] = Addr(r(c, S0)).off(0x434).lw(c);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x58).lh(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0x5c).lw(c);
        let target = r(c, V0);
        c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
        call_ptr(c, target, 0x0024_09f0)?;
        if r(c, V0) != 0 {
            return mcmanager_return(c, frame);
        }

        c.cycles += 8;
        c.g[A1][0] = Addr(r(c, S0)).off(0x434).lw(c);
        c.g[V1][0] = Addr(r(c, A1)).lw(c);
        c.g[A0][0] = Addr(r(c, V1)).off(0x1a8).lh(c);
        c.g[V0][0] = Addr(r(c, V1)).off(0x1ac).lw(c);
        let target = r(c, V0);
        c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
        call_ptr(c, target, 0x0024_0a10)?;
        if r(c, V0) != 0 {
            // v53() non-zero: the sub-object's v73() decides whether to wait.
            c.g[A1][0] = Addr(r(c, S0)).off(0x434).lw(c);
            c.cycles += 7;
            c.g[V1][0] = Addr(r(c, A1)).lw(c);
            c.g[A0][0] = Addr(r(c, V1)).off(0x248).lh(c);
            c.g[V0][0] = Addr(r(c, V1)).off(0x24c).lw(c);
            let target = r(c, V0);
            c.g[A0][0] = sx(p(r(c, A1), r(c, A0)));
            call_ptr(c, target, 0x0024_0a38)?;
            let idle = r(c, V0) == 0;
            c.g[A1][0] = 0x14;
            if !idle {
                c.cycles += 2;
                c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
            c.g[A1][0] = 4;
        }

        // if (m->v70() == 0) { if (m->v6c() != 0) m->v01(1); return; }
        c.cycles += 5;
        c.g[V0][0] = Addr(r(c, S0)).off(0x70).lw(c);
        let target = r(c, V0);
        call_ptr(c, target, 0x0024_0a54)?;
        let zero = r(c, V0) == 0;
        c.g[A1][0] = 0xd;
        if zero {
            c.cycles += 5;
            c.g[V0][0] = Addr(r(c, S0)).off(0x6c).lw(c);
            let target = r(c, V0);
            call_ptr(c, target, 0x0024_0a84)?;
            let zero = r(c, V0) == 0;
            c.g[A1][0] = 1;
            if !zero {
                c.cycles += 5;
                c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
                c.g[A0][0] = sx(p(r(c, S0), Addr(r(c, V0)).off(0x8).lh(c) as u32));
                let target = Addr(r(c, V0)).off(0xc).lw(c) as u32;
                call_ptr(c, target, 0x0024_0aa0)?;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(r(c, S0)).off(0x748).lw(c);
            c.cycles += 6;
            c.g[A0][0] = sx(p(r(c, S0), Addr(r(c, V0)).off(0x8).lh(c) as u32));
            let target = Addr(r(c, V0)).off(0xc).lw(c) as u32;
            call_ptr(c, target, 0x0024_0a70)?;
        }

        c.cycles += 1;
        mcmanager_return(c, frame)
    })
}

/// The `MCManager_State09` tail: `lq s0; ld ra; addiu sp; jr ra`.
pub(crate) fn mcmanager_return(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `MCManager_State1A_Delete(self)` (0x240860): delete the selected item of
/// state 1A: ask the screen (or, when the delete countdown is done, the
/// focused list) whether its confirm handler wants to run, otherwise step
/// the countdown and dispatch the item's delete method.
pub fn mcmanager_state1a_delete(c: &mut Cpu) {
    const SCREEN_SLOT: u32 = 0x58;
    const LIST_SLOT: u32 = 0x248;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(this, 0x108));
        let expired = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = lw(c, p(this, 0x434));
        let mut stage = 0u32; // 0 epilogue, 1 state 1c, 2 state 1b, 3 countdown
        if !expired {
            c.cycles += 7;
            c.g[V1][0] = lw(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh_3(c, p(c.g[V1][0] as u32, SCREEN_SLOT));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, SCREEN_SLOT + 4));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0024_0890)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, p(this, 0x108));
                stage = 3;
            } else {
                c.cycles += 8;
                c.g[A1][0] = lw(c, p(this, 0x434));
                c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh_3(c, p(c.g[V1][0] as u32, LIST_SLOT));
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, LIST_SLOT + 4));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x0024_08b0)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw(c, p(this, 0x748));
                    stage = 1;
                } else {
                    c.cycles += 8;
                    c.g[A2][0] = lw(c, p(this, 0x434));
                    c.g[A1][0] = lw(c, p(this, 0x428));
                    c.g[V0][0] = lw(c, c.g[A2][0] as u32);
                    c.g[A0][0] = lh_3(c, p(c.g[V0][0] as u32, 0x28));
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0x2c));
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0024_08d4)?;
                    c.g[V0][0] = lw(c, p(this, 0x108));
                    stage = 3;
                }
            }
        } else {
            c.cycles += 7;
            c.g[V1][0] = lw(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh_3(c, p(c.g[V1][0] as u32, SCREEN_SLOT));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, SCREEN_SLOT + 4));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0024_08f8)?;
            if c.g[V0][0] != 0 {
                c.lq(16, frame.at(c, 0x10));
            } else {
                c.cycles += 8;
                c.g[A1][0] = lw(c, p(this, 0x434));
                c.g[V1][0] = lw(c, c.g[A1][0] as u32);
                c.g[A0][0] = lh_3(c, p(c.g[V1][0] as u32, LIST_SLOT));
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, LIST_SLOT + 4));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x0024_0918)?;
                let zero = c.g[V0][0] == 0;
                c.g[V0][0] = lw(c, p(this, 0x748));
                stage = if zero { 2 } else { 1 };
            }
        }
        match stage {
            3 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                sw_5(c, p(this, 0x108), c.g[V0][0]);
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x10));
            }
            1 => {
                c.cycles += 7;
                c.g[A1][0] = 0x1c;
                c.g[A0][0] = lh_3(c, p(c.g[V0][0] as u32, 8));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0024_0934)?;
                c.lq(16, frame.at(c, 0x10));
            }
            2 => {
                c.cycles += 5;
                c.g[A1][0] = 0x1b;
                c.g[A0][0] = lh_3(c, p(c.g[V0][0] as u32, 8));
                c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0024_0950)?;
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x10));
            }
            _ => {
                c.lq(16, frame.at(c, 0x10));
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginSave(MCManager *self, int mode, int arg)` (0x23cae8):
/// start a save: record the mode, pick the "saving"/"loading" state word and
/// tell the movie object, then format the three 0x200-byte strings (title,
/// product code, save-type tag) into the request at `self+0x1b4`.
pub fn mcmanager_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x640);
        frame.save(c, 18, 0x610);
        let self_ = abi::arg64(c, 0);
        frame.save(c, 16, 0x630);
        frame.save(c, 17, 0x620);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save_ra(c, 0x600);
        let this = self_ as u32;
        c.sw(this.wrapping_add(0xb4), abi::arg64(c, 1));
        c.g[A1][0] = 1;
        callee::MC_SET_MODE.call(c, 0x0023_cb10)?;

        c.g[V1][0] = 1;
        c.g[V0][0] = c.lw(this.wrapping_add(0xb4));
        c.sw(this.wrapping_add(0xfc), c.g[S0][0]);
        let nonzero = c.g[V0][0] != 0;
        c.sw(this.wrapping_add(0xbc), c.g[V1][0]);
        // `buf` receives the format string; `desc`, `name` the other two.
        let fmt = (c.g[SP][0] as u32).wrapping_add(0x400);
        let desc = c.g[SP][0] as u32;
        let name = (c.g[SP][0] as u32).wrapping_add(0x200);
        if !nonzero {
            // mode 0: "saving", tell the movie object directly.
            c.cycles += 5;
            c.g[V0][0] = 4;
            c.g[A2][0] = c.lw(this.wrapping_add(0x434));
            c.sw(this.wrapping_add(0xec), c.g[V0][0]);
            c.g[A1][0] = 0x0008_0000;
            c.cycles += 7;
            movie_finish_step(c, 0x0023_cb78)?;
            c.g[S0][0] = sx(fmt);
        } else {
            c.cycles += 2;
            let mode_is_one = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = 6;
            if !mode_is_one {
                // mode 2+: "loading", ask the BAGT interface.
                c.cycles += 15;
                c.sw(this.wrapping_add(0xec), c.g[V1][0]);
                callee::CBE_GET_BE_2.call(c, 0x0023_cb88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE_2.call(c, 0x0023_cb94)?;
                callee::F_00152948.call(c, 0x0023_cb9c)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                movie_finish_step(c, 0x0023_cbb8)?;
                c.g[S0][0] = sx(fmt);
            } else {
                // mode 1: "saving", ask the player interface.
                c.cycles += 9;
                c.sw(this.wrapping_add(0xec), c.g[V0][0]);
                callee::CBE_GET_BE_2.call(c, 0x0023_cb48)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE_2.call(c, 0x0023_cb54)?;
                callee::F_00152BA8.call(c, 0x0023_cb5c)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                c.cycles += 7;
                movie_finish_step(c, 0x0023_cb78)?;
                c.g[S0][0] = sx(fmt);
            }
        }

        c.cycles += 24;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        callee::F_002C2540.call(c, 0x0023_cbcc)?;
        c.g[A0][0] = sx(desc);
        c.g[A1][0] = 0x200;
        callee::MC_GET_PRODUCT_CODE.call(c, 0x0023_cbd8)?;
        c.g[A1][0] = c.lw(this.wrapping_add(0xb4));
        c.g[A0][0] = self_;
        c.g[A2][0] = sx(name);
        c.g[A3][0] = 0x100;
        callee::MC_GET_SAVE_TYPE_TAG.call(c, 0x0023_cbf0)?;
        c.g[A0][0] = sx(this.wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = sx(name);
        c.g[A2][0] = sx(desc);
        callee::F_002C26D0.call(c, 0x0023_cc04)?;
        frame.restore(c, 16, 0x630);
        frame.restore(c, 17, 0x620);
        frame.restore(c, 18, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginLoad(MCManager *self, int mode)` (0x23c8f0): the load
/// twin of [`mcmanager_begin_save`] (`-1` in the request slot, no cached
/// value, and the "loading" state word in the mode-2 path).
pub fn mcmanager_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x640);
        frame.save(c, 18, 0x610);
        let self_ = abi::arg64(c, 0);
        frame.save(c, 16, 0x630);
        frame.save(c, 17, 0x620);
        frame.save_ra(c, 0x600);
        let this = self_ as u32;
        c.sw(this.wrapping_add(0xb4), abi::arg64(c, 1));
        c.g[A1][0] = 1;
        callee::MC_SET_MODE.call(c, 0x0023_c914)?;

        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = c.lw(this.wrapping_add(0xb4));
        c.sw(this.wrapping_add(0xfc), c.g[V0][0]);
        let nonzero = c.g[V1][0] != 0;
        c.sw(this.wrapping_add(0xbc), 0);
        let fmt = (c.g[SP][0] as u32).wrapping_add(0x400);
        let desc = c.g[SP][0] as u32;
        let name = (c.g[SP][0] as u32).wrapping_add(0x200);
        if !nonzero {
            // mode 0: "loading", tell the movie object directly.
            c.cycles += 5;
            c.g[V0][0] = 4;
            c.g[A2][0] = c.lw(this.wrapping_add(0x434));
            c.sw(this.wrapping_add(0xec), c.g[V0][0]);
            c.g[A1][0] = 0x0008_0000;
            c.cycles += 7;
            movie_finish_step(c, 0x0023_c984)?;
            c.g[S0][0] = sx(fmt);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                // mode 2+: "loading", ask the BAGT interface.
                c.cycles += 15;
                c.sw(this.wrapping_add(0xec), c.g[V0][0]);
                callee::CBE_GET_BE_2.call(c, 0x0023_c994)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE_2.call(c, 0x0023_c9a0)?;
                callee::F_00152948.call(c, 0x0023_c9a8)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                movie_finish_step(c, 0x0023_c9c4)?;
                c.g[S0][0] = sx(fmt);
            } else {
                // mode 1: "loading", ask the player interface.
                c.cycles += 10;
                c.g[V0][0] = 6;
                c.sw(this.wrapping_add(0xec), c.g[V0][0]);
                callee::CBE_GET_BE_2.call(c, 0x0023_c954)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE_2.call(c, 0x0023_c960)?;
                callee::F_00152BA8.call(c, 0x0023_c968)?;
                c.g[A2][0] = c.lw(this.wrapping_add(0x434));
                c.g[A1][0] = c.g[V0][0];
                c.cycles += 7;
                movie_finish_step(c, 0x0023_c984)?;
                c.g[S0][0] = sx(fmt);
            }
        }

        c.cycles += 24;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        callee::F_002C2540.call(c, 0x0023_c9d8)?;
        c.g[A0][0] = sx(desc);
        c.g[A1][0] = 0x200;
        callee::MC_GET_PRODUCT_CODE.call(c, 0x0023_c9e4)?;
        c.g[A1][0] = c.lw(this.wrapping_add(0xb4));
        c.g[A0][0] = self_;
        c.g[A2][0] = sx(name);
        c.g[A3][0] = 0x100;
        callee::MC_GET_SAVE_TYPE_TAG.call(c, 0x0023_c9fc)?;
        c.g[A0][0] = sx(this.wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = sx(name);
        c.g[A2][0] = sx(desc);
        callee::F_002C26D0.call(c, 0x0023_ca10)?;
        frame.restore(c, 16, 0x630);
        frame.restore(c, 17, 0x620);
        frame.restore(c, 18, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// The movie object's vtable slot 0x98/0x9c call shared by the save modes:/// `a2` (the movie object) and `a1` (the step argument) are already set.
pub(crate) fn movie_finish_step(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
    let f = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    crate::call::call_ptr(c, f, ret)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// OcclusionMan_BuildOccluderPlanes (0x002429b0)
// ---------------------------------------------------------------------------

/// Occluder fields: the four corners (+0x10..+0x40) and the four side
/// planes (+0x60..+0x90, `(n, -eye.n)`).
pub(crate) const OCC_CORNERS: u32 = 0x10;

pub(crate) const OCC_PLANES: u32 = 0x60;

/// `sp + off = v - eye` through VU0 (`sqc2` to `sp+0x40`, copied by `lq`/`sq`
/// to `sp+0x30` and `sp+off`).
pub(crate) fn occ_edge(c: &mut Cpu, s: u32, eye: u32, v: u32, off: u32) {
    c.lqc2(3, v);
    c.lqc2(4, eye);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x40);
    c.lq(V0, s + 0x40);
    c.sq(V0, s + 0x30);
    c.sq(V0, s + off);
}

/// One side plane through the eye and the corners `a`, `b`:
/// `n = normalize((a - eye) x (b - eye))`, `d = -(eye . n)`, stored at
/// `out`. The first plane keeps its values in a different register order.
pub(crate) fn occ_plane(c: &mut Cpu, s: u32, eye: u32, a: u32, b: u32, out: u32, first: bool) {
    occ_edge(c, s, eye, a, 0);
    occ_edge(c, s, eye, b, 0x10);
    c.lqc2(4, s);
    c.lqc2(5, s + 0x10);
    {
        let v = c.vu0();
        v.opmula(4, 5);
        v.opmsub(6, 5, 4);
        v.sub(0b0001, 6, 6, Src::Vec(6));
    }
    c.sqc2(6, s + 0x50);
    c.lq(V0, s + 0x50);
    c.sq(V0, s + 0x40);
    c.lqc2(3, s + 0x40);
    {
        let v = c.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(3));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
        v.rsqrt(0, 3, 4, 0);
        v.waitq();
        v.mul(0b1111, 5, 3, Src::Q);
    }
    c.sqc2(5, s + 0x50);
    c.lq(V0, s + 0x50);
    c.sq(V0, s + 0x20);
    c.sq(V0, s + 0x30);
    // eye . n
    c.lqc2(3, eye);
    c.lqc2(5, s + 0x20);
    {
        let v = c.vu0();
        v.add(0b1000, 6, 0, Src::Bc(0, W));
        v.mul(0b1111, 4, 3, Src::Vec(5));
        v.adda(0b1000, 4, Src::Bc(4, Y));
        v.madda(0b1000, 6, Src::Bc(4, Z));
        v.madd(0b1000, 4, 6, Src::Bc(4, W));
    }
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    let d = -fr_2(c, 0);
    set_fr(c, 0, d);
    Addr(s + 0x2c).write_fpu(c, d);
    let nx = Addr(s + 0x20).read_u32(c);
    let ny = Addr(s + 0x24).read_u32(c);
    let nz = Addr(s + 0x28).read_u32(c);
    c.f[1] = if first { nz } else { nx };
    c.f[2] = ny;
    if !first {
        c.f[3] = nz;
    }
    Addr(out).write_u32(c, nx);
    Addr(out + 4).write_u32(c, ny);
    Addr(out + 8).write_u32(c, nz);
    Addr(out + 0xc).write_fpu(c, d);
}

/// `void OcclusionMan_BuildOccluderPlanes(OcclusionMan* self,
/// const Vec4* eye, Occluder* occ)` (0x002429b0): the occluder's four side
/// planes from the eye through each pair of adjacent corners (normalised
/// on VU0), each `(n, -eye.n)`.
pub fn occlusion_man_build_occluder_planes(c: &mut Cpu) {
    c.cycles += 215;
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    let s = sp(c);
    let eye = r32(c, A1);
    let occ = r32(c, A2);
    let corner = |i: u32| occ + OCC_CORNERS + 0x10 * (i % 4);
    for i in 0..4u32 {
        let (a, b) = (corner(i + 1), corner(i));
        occ_plane(c, s, eye, a, b, occ + OCC_PLANES + 0x10 * i, i == 0);
    }
    c.g[SP][0] = sx(s.wrapping_add(0x60));
    abi::ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;

    pub(crate) use crate::game::ai::rider::tests::clear_2;

    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot_3;
    pub(crate) use crate::game::be::beintnetwork::tests::put_str;
    pub(crate) use crate::game::be::belibrary::tests::alloc_ready;

    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_9;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_8;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_method;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::world;

    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;

    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;

    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festatecredits::tests::RET_ZERO as RET_ZERO_2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_5;

    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_6;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ONE_STUB;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ZERO_STUB;
    pub(crate) use crate::game::main::gameload::tests::slot_to;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_11;

    pub(crate) use crate::game::ui::uivector::tests::start;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn mc_manager_exists_matches() {
        check(
            ssx3_ee::s06::f_0023c850,
            mc_manager_exists,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c11_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), v);
            },
        );
    }

    #[test]
    fn mc_manager_is_idle_matches() {
        check(
            ssx3_ee::s06::f_0023c898,
            mc_manager_is_idle,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c11_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let v = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + 0x130, v);
            },
        );
    }

    #[test]
    fn occlusion_man_ctor_matches() {
        check(
            ssx3_ee::s06::f_00242500,
            occlusion_man_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2425_0000 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + (seed % 4) * 4;
                c.g[A0][0] = base as u64;
                for off in [0u32, 4, 0x108, 0x10c] {
                    c.bus.write32(base + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn occlusion_man_add_occluder_matches() {
        check(
            ssx3_ee::s06::f_00242540,
            occlusion_man_add_occluder,
            200,
            &|c, seed| {
                let mut r = Rng(0x2425_4000 ^ seed as u64);
                scramble(c, &mut r);
                let occ = DATA + 0x200;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = occ as u64;
                let head = if seed % 2 == 0 { 0 } else { DATA + 0x300 };
                c.bus.write32(DATA, head);
                c.bus.write32(DATA + 4, r.next_u32());
                c.bus.write32(occ + 0xbc, r.next_u32());
                c.bus.write32(occ + 0xc0, r.next_u32());
                if head != 0 {
                    c.bus.write32(head + 0xc0, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn occlusion_man_remove_occluder_matches() {
        check(
            ssx3_ee::s06::f_00242570,
            occlusion_man_remove_occluder,
            300,
            &|c, seed| {
                let mut r = Rng(0x2425_7000 ^ seed as u64);
                scramble(c, &mut r);
                let n = DATA + 0x100;
                let o1 = DATA + 0x200;
                let o2 = DATA + 0x300;
                // Cover next == 0, prev == 0, occ == head and every combination.
                let (next, prev, head) = match seed % 6 {
                    0 => (0u32, 0u32, n),
                    1 => (0, 0, o1),
                    2 => (o1, 0, n),
                    3 => (o1, o2, n),
                    4 => (o1, o2, o1),
                    _ => (o1, 0, o1),
                };
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = n as u64;
                c.bus.write32(DATA, head);
                c.bus.write32(DATA + 4, r.next_u32());
                c.bus.write32(n + 0xbc, prev);
                c.bus.write32(n + 0xc0, next);
                for node in [o1, o2] {
                    c.bus.write32(node + 0xbc, r.next_u32());
                    c.bus.write32(node + 0xc0, r.next_u32());
                }
            },
        );
    }

    /// A `jr ra` stub the tests use for vtable slots whose result does not
    /// matter (`VfxVisualEffectsTest_Update`).
    pub(crate) const STUB: u32 = 0x0038_f7f8;

    #[test]
    fn occlusion_man_build_visible_matches() {
        check(
            ee::s06::f_002425c0__generated,
            occlusion_man_build_visible,
            160,
            &|c, seed| {
                let mut r = Rng(0x2425_c0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                let bbox = DATA + 0x400;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(bbox);
                c.bus
                    .write32(this + occl::ENABLED, if seed % 7 == 0 { 0 } else { 1 });
                c.bus.write32(this + occl::COUNT, 0);
                c.bus.write32(this + occl::N_VISIBLE, 0);
                c.bus.write32(this + occl::MASK, 0);
                for i in 0..6 {
                    c.bus.write32(bbox + i * 4, r.f32(100.0));
                }
                // Camera singleton: a matrix method that writes nothing (the frame
                // is already zero) and a row-position method with a zero result.
                let cam = DATA + 0x800;
                let vt = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x854, cam);
                c.bus.write32(cam + CAMERA_VT, vt);
                for off in [0x158u32, 0x118, 0x2e8] {
                    c.bus.write16(vt + off, 0);
                }
                for off in [0x15c, 0x11c, 0x2ec] {
                    c.bus.write32(vt + off, STUB);
                }
                // View list.
                let n = seed % 4;
                c.bus.write32(this, 0);
                let mut prev = 0u32;
                for i in 0..n {
                    let v = DATA + 0x2000 + i * 0x100;
                    if i == 0 {
                        c.bus.write32(this, v);
                    } else {
                        c.bus.write32(prev + view::NEXT, v);
                    }
                    c.bus.write32(v + view::NEXT, 0);
                    for k in 0..4 {
                        c.bus.write32(v + view::POS + k * 4, r.f32(50.0));
                        c.bus.write32(v + view::NORMAL + k * 4, r.f32(2.0));
                    }
                    for k in 0..3 {
                        c.bus.write32(v + view::MIN + k * 4, r.f32(100.0));
                        c.bus.write32(v + view::MAX + k * 4, r.f32(100.0));
                    }
                    prev = v;
                }
            },
        );
    }

    #[test]
    fn mc_manager_get_slot_free_space_matches() {
        check_lohi(
            ee::s06::f_002419d8__generated,
            mc_manager_get_slot_free_space,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0024_19d8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let slot = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, slot);
                c.bus.write32(DATA + slot * 0xe0 + 0x354, r.next_u32());
            },
        );
    }

    #[test]
    fn mc_manager_has_status_error_matches() {
        check(
            ee::s06::f_0023c8d0__generated,
            mc_manager_has_status_error,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_c8d0, 0);
                c.g[A0][0] = DATA as u64;
                let v = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + 0x43c, v);
            },
        );
    }

    #[test]
    fn mc_manager_get_product_code_matches() {
        check(
            ee::s06::f_00241aa0__generated,
            mc_manager_get_product_code,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x24_1aa0, 0);
                c.g[A0][0] = DATA as u64;
                let code = DATA + 0x800;
                match seed % 3 {
                    0 => put_str(c, code, ""),
                    1 => put_str(c, code, "SLUS-20772"),
                    _ => {
                        c.bus.write32(code, r.next_u32());
                        c.bus.write8(code + 4, 0);
                    }
                }
                // `D_0047C3A8`: the product-code string pointer.
                c.bus.write32(0x0047_c3a8, code);
            },
        );
    }

    #[test]
    fn occlusion_man_dtor_matches() {
        crate::difftest::check(
            ee::s06::f_00242518__generated,
            occlusion_man_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2425_18, 2);
                be_library(c);
                if seed % 2 == 1 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = 1;
                } else {
                    c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                    c.g[A1][0] = sx(r.next_u32() & !1);
                }
            },
        );
    }

    /// A non-null, zeroed stand-in for the BE-library singleton at
    /// `gp - 0x1f04` (its lazy constructor would otherwise load from disc).
    pub(crate) const BE_LIBRARY: u32 = DATA + 0x1_f000;

    /// Globals most of these wrappers read: the BE library, and a small zeroed
    /// world (`world -> +0x84 -> +0xc`, rider count at `+0x7c`).
    pub(crate) fn globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        let world = DATA + 0x2_0000;
        c.bus.write32(GAME_GP - 0x848, world);
        let game = world + 0x1000;
        c.bus.write32(world + 0x84, game);
        let race = world + 0x2000;
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
    }

    /// The eight memory-card manager hooks share the same fixture.
    pub(crate) fn mc_manager_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            // The card object at +0x434 and the state machine at +0x748 are
            // null for odd seeds (calls through the miss hook), zeroed objects
            // at DATA + 0x3000 for even ones.
            c.bus
                .write32(DATA + 0x434, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
            c.bus
                .write32(DATA + 0x748, if seed % 2 == 0 { DATA + 0x4000 } else { 0 });
        });
    }

    #[test]
    fn mc_manager_card_set_state_1e_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_0023ca70__generated,
            mc_manager_card_set_state_1e,
            0xb4,
        );
    }

    #[test]
    fn mc_manager_destroy_card_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_0023d5e8__generated,
            mc_manager_destroy_card,
            0xb5,
        );
    }

    #[test]
    fn mc_manager_set_state_no_card_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_0023eb50__generated,
            mc_manager_set_state_no_card,
            0xb6,
        );
    }

    #[test]
    fn mc_manager_state_13_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_0023f2d8__generated,
            mc_manager_state_13,
            0xb7,
        );
    }

    #[test]
    fn mc_manager_state_23_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_0023ffc0__generated,
            mc_manager_state_23,
            0xb8,
        );
    }

    #[test]
    fn mc_manager_state_2a_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_00240c90__generated,
            mc_manager_state_2a,
            0xb9,
        );
    }

    #[test]
    fn mc_manager_state_2d_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_00240eb0__generated,
            mc_manager_state_2d,
            0xba,
        );
    }

    #[test]
    fn mc_manager_state_2e_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_00240ee0__generated,
            mc_manager_state_2e,
            0xbb,
        );
    }

    #[test]
    fn mc_manager_close_card_matches() {
        mc_manager_test(
            ssx3_ee::s06::f_002410a0__generated,
            mc_manager_close_card,
            0xbc,
        );
    }

    // ------------------------------------------------------------------
    // Memory card manager.

    #[test]
    fn mc_manager_request_card_info_matches() {
        check(
            ssx3_ee::s06::f_0023fb20__generated,
            mc_manager_request_card_info,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fb20, 1);
                let self_ = DATA + 0x100;
                let dev = DATA + 0x500;
                let vt = DATA + 0x600;
                c.bus.write32(self_ + 0x434, dev);
                c.bus.write32(dev, vt);
                c.bus.write16(vt + 0x28, ((r.next_u32() % 0x40) * 4) as u16);
                c.bus.write32(vt + 0x2c, FAKE);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_state26_matches() {
        check(
            ssx3_ee::s06::f_0023fd08__generated,
            mc_manager_state26,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fd08, 1);
                let self_ = DATA + 0x100;
                fill(c, &mut r, self_, 0x60);
                c.bus.write32(self_ + 0xbc, 0);
                c.bus.write32(self_ + 0x38, FAKE);
                c.g[A0][0] = sx(self_);
            },
        );
    }

    #[test]
    fn mc_manager_dtor_matches() {
        check(
            ssx3_ee::s06::f_002424c8__generated,
            mc_manager_dtor,
            SEEDS_2,
            &|c, seed| {
                let _r = machine(c, seed, 0x424c8, 0);
                // `del & 1 == 0`: keep the object (the freeing path is not
                // exercised here; see the report).
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn mc_manager_destroy_matches() {
        check(
            ee::s06::f_0023c860__generated,
            mc_manager_destroy,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23c8_60, 0);
                let obj = DATA + 0x400;
                zero(c, obj, 0x800);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x47c),
                    if seed % 2 == 0 { 0 } else { obj },
                );
                c.bus.write32(obj + 0x748, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x10);
            },
        );
    }

    #[test]
    fn mc_manager_card_set_state18_matches() {
        check(
            ee::s06::f_0023cc20__generated,
            mc_manager_card_set_state18,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23cc_20, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x500);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x118, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(self_ + 0x434, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x158);
            },
        );
    }

    #[test]
    fn mc_manager_refresh_slot_matches() {
        check(
            ee::s06::f_0023fae0__generated,
            mc_manager_refresh_slot,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23fa_e0, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x500);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_ + 0x428, seed % 4);
                let cached = if seed % 2 == 0 {
                    seed % 4
                } else {
                    (seed + 1) % 4
                };
                c.bus.write32(self_ + 0x444, cached);
                c.bus.write32(self_ + 0x434, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x28);
            },
        );
    }

    #[test]
    fn mc_manager_read_save_chunk_matches() {
        check(
            ssx3_ee::s06::f_00241200__generated,
            mc_manager_read_save_chunk,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x2412_00, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0x10 } else { 0 };
            },
        );
    }

    /// The allocator mutex: pretend "this thread" already holds it so the
    /// real `EAMutex_Lock` takes its fast path.
    pub(crate) fn allocator_mutex(c: &mut Cpu) {
        c.bus.write32(0x0053_8b00 + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
    }

    #[test]
    fn mc_manager_create_card_matches() {
        check(
            ee::s06::f_0023d5a8__generated,
            mc_manager_create_card,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23d5a8, 0);
                alloc_ready(c);
                allocator_mutex(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.bus.write32(GAME_GP - 0x5128, r.next_u32());
            },
        );
    }

    #[test]
    fn mc_manager_state02_matches() {
        check(
            ssx3_ee::s06::f_0023e4f8__generated,
            mc_manager_state02,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x23e4f8, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                let vt = DATA + 0x2000;
                let left: [u32; 4] = [0, 1, 5, 0x8000_0000];
                c.bus.write32(this_ + 0x108, left[seed as usize % 4]);
                c.bus.write32(this_ + 0x748, vt);
                fake_method(c, vt, 0x08);
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn mc_manager_write_save_chunk_matches() {
        check(
            ssx3_ee::s06::f_00241138__generated,
            mc_manager_write_save_chunk,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x241138, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                let movie = DATA + 0x2000;
                let vt = DATA + 0x3000;
                c.bus.write32(this_ + 0xe4, (seed as u32).wrapping_mul(7));
                c.bus.write32(this_ + 0x434, movie);
                c.bus.write32(movie, vt);
                fake_method(c, vt, 0x148);
                c.g[A0][0] = sx(this_);
                c.g[A2][0] = (seed % 128) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_read_save_by_slot_matches() {
        check(
            ee::s06::f_0023ca28__generated,
            mc_manager_read_save_by_slot,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23ca_28, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() & 7);
                c.g[A2][0] = sx(r.next_u32() & 0xffff);
                c.g[A3][0] = sx(DATA + 0x300);
                let card = DATA + 0x400;
                c.bus.write32(this + 0x434, card);
                c.bus.write32(card, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 0xe8, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn mc_manager_update_matches() {
        check(
            ee::s06::f_0023d618__generated,
            mc_manager_update,
            32,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x23d6_18, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let step = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + r.next_u32() % 0x1e,
                };
                c.bus.write32(this + 0xb8, step);
            },
        );
    }

    const MESSAGE_STATES: [u32; 4] = [
        mc::state::PRE_CHECK_WAIT,
        mc::state::LIST_DIR,
        mc::state::SAVE_FINISH,
        mc::state::LOAD_FINISH,
    ];

    #[test]
    fn short_card_wait_caps_only_the_message_states() {
        // Negative: already expired, leave it.
        let pairs = [(90, 15), (16, 15), (15, 15), (14, 14), (0, 0), (-1, -1)];
        for state in MESSAGE_STATES {
            for (count, expected) in pairs {
                assert_eq!(
                    short_card_wait(state, count),
                    expected,
                    "{state:#x} {count}"
                );
            }
        }
        for state in [0, 1, 3, 5, 7, 0x11, 0x13, 0x20, 0x22] {
            for (count, _) in pairs {
                assert_eq!(short_card_wait(state, count), count, "{state:#x} {count}");
            }
        }
    }

    /// Runs `shorten_card_wait` on a manager and returns its wait afterwards.
    fn wait_after_shortening(state: u32, step: u32, wait: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        let this = DATA + 0x100;
        c.bus.write32(this + mc::STATE, state);
        c.bus.write32(this + mc::UPDATE_STEP, step);
        c.bus.write32(this + mc::WAIT, wait);
        shorten_card_wait(&mut c, Addr(this));
        c.bus.read32(this + mc::WAIT)
    }

    #[test]
    fn shorten_card_wait_caps_running_message_states() {
        for state in MESSAGE_STATES {
            assert_eq!(wait_after_shortening(state, 0, 90), 15);
            assert_eq!(wait_after_shortening(state, 0, 14), 14);
            assert_eq!(wait_after_shortening(state, 0, u32::MAX), u32::MAX);
            assert_eq!(wait_after_shortening(state, 0, 0x7fff_ffff), 15);
            // Negative as a signed count: already expired.
            assert_eq!(wait_after_shortening(state, 0, 0x8000_0000), 0x8000_0000);
            // The state machine isn't running; leave RAM untouched.
            assert_eq!(wait_after_shortening(state, 1, 90), 90);
        }
        assert_eq!(wait_after_shortening(3, 0, 90), 90);
    }

    /// The manager's update runs 60 times per second at 60, 120, 144 and
    /// 240 Hz (step 2 does nothing but charge its cycles).
    #[test]
    fn mc_manager_update_rate() {
        let run = |hz: u32, calls: u32| {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            // One manager per rate: the clocks are kept per address.
            let this = DATA + 0x100 + hz * 4;
            c.bus.write32(this + 0xb8, 2);
            let start = c.cycles;
            for _ in 0..calls {
                c.g[A0][0] = sx(this);
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0023_d618;
                mc_manager_update(&mut c);
            }
            c.cycles - start
        };
        let one = run(60, 1);
        assert!(one > 0);
        for hz in [60u32, 120, 144, 240] {
            assert_eq!(run(hz, hz), 60 * one, "{hz} Hz");
        }
    }

    #[test]
    fn mc_manager_format_slot_name_matches() {
        check(
            ee::s06::f_00241dc8__generated,
            mc_manager_format_slot_name,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x241d_c8, 0);
                clear(c, DATA, 0x100);
                // The wide-character table the name is read from.
                put_words(
                    c,
                    0x004a_2c28,
                    &[0x31 + (r.next_u32() % 26), 0x61 + (seed % 26), 0],
                );
                let dst = DATA + 0x800;
                clear(c, dst, 0x80);
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 40) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_write_save_by_name_w_matches() {
        check(
            ee::s06::f_00241000__generated,
            mc_manager_write_save_by_name_w,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x2410);
                let s = DATA;
                let card = DATA + 0x1000;
                let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x128]);
                c.bus.write32(s + 0x434, card);
                c.bus.write32(card, vt);
                c.bus.write32(s + 0xe4, 0xdead);
                fill(c, &mut r, card + 4, 16);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn mc_manager_state_0c_matches() {
        check(
            ee::s06::f_0023f0f8__generated,
            mc_manager_state_0c,
            SEEDS_5,
            &|c, seed| {
                let _ = common(c, seed, 0x23f0f8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x500);
                c.bus.write32(this + 0x6c, FAKE);
                c.bus.write32(this + 0x428, seed % 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mc_manager_state_matches() {
        for (gen, hand, salt) in [
            (
                ssx3_ee::s06::f_0023e2c8__generated as fn(&mut Cpu),
                mc_manager_state35 as fn(&mut Cpu),
                0x23e2c8u64,
            ),
            (
                ssx3_ee::s06::f_0023e498__generated,
                mc_manager_state33,
                0x23e498u64,
            ),
        ] {
            check(gen, hand, 150, &|c, seed| {
                let mut r = machine(c, seed, salt, 2);
                let mgr = DATA;
                let movie = DATA + 0x100;
                let ctl = vtable(c, &mut r, &[0x58]);
                c.bus.write32(mgr + 0x434, movie);
                c.bus.write32(movie, ctl);
                // The 3D crowd manager's method table lives at the object
                // itself: `{delta, fn}` at +8 / +0xc.
                let crowd = vtable(c, &mut r, &[8]);
                c.bus.write32(mgr + 0x748, crowd);
                c.g[A0][0] = mgr as u64;
            });
        }
    }

    // -------------------------------------------------------- memory card

    #[test]
    fn mc_manager_end_operation_matches() {
        check(
            ssx3_ee::s06::f_0023ea30__generated,
            mc_manager_end_operation,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x23ea30);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0xbc, (seed % 2) as u32);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
            },
        );
    }

    #[test]
    fn mc_manager_reset_matches() {
        check(
            ssx3_ee::s06::f_0023fb58__generated,
            mc_manager_reset,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x23fb58);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
            },
        );
    }

    #[test]
    fn mc_manager_state1c_matches() {
        check(
            ssx3_ee::s06::f_00240800__generated,
            mc_manager_state1c,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x240800);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x58]);
                c.bus.write32(DATA + 0x900, vt);
                c.bus.write32(this + 0x6c, FAKE);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
            },
        );
    }

    #[test]
    fn mc_manager_state29_matches() {
        check(
            ssx3_ee::s06::f_00240c30__generated,
            mc_manager_state29,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x240c30);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x6c, FAKE);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
                c.bus.write32(this + 0x428, r.next_u32());
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(DATA + 0x900, vt);
            },
        );
    }

    #[test]
    fn mc_manager_finish_load_matches() {
        check(
            ssx3_ee::s06::f_00241240__generated,
            mc_manager_finish_load,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x241240);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x108]);
                c.bus.write32(DATA + 0x900, vt);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0x748, vt);
                c.bus.write32(this + 0x2c, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_get_num_slots_matches() {
        check(
            ssx3_ee::s06::f_00241ac0__generated,
            mc_manager_get_num_slots,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x241ac0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(
                    this + 0x424,
                    match seed % 3 {
                        0 => 0,
                        1 => 0xffff_ffff,
                        _ => 1,
                    },
                );
                c.bus.write32(this + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x228]);
                c.bus.write32(DATA + 0x900, vt);
            },
        );
    }

    // ------------------------------------------------- comparator / streams

    #[test]
    fn occlusion_man_compare_by_distance_matches() {
        check(
            ssx3_ee::s06::f_00242978__generated,
            occlusion_man_compare_by_distance,
            200,
            &|c, seed| {
                let mut r = start(c, seed, 0x42978);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
                let (fa, fb) = match seed % 4 {
                    0 => (r.edge_f32(), r.edge_f32()),
                    1 => {
                        let v = r.edge_f32();
                        (v, v) // equal: returns 0
                    }
                    2 => (r.edge_f32(), r.edge_f32()),
                    _ => (r.edge_f32(), r.edge_f32()),
                };
                c.bus.write32(DATA + 0x104, fa);
                c.bus.write32(DATA + 0x204, fb);
            },
        );
    }

    #[test]
    fn mc_manager_poll_card_status_matches() {
        check(
            ee::s06::f_00241cd8__generated,
            mc_manager_poll_card_status,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0024_1cd8, 1);
                be_library_3(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x43c, r.next_u32());
                c.bus.write32(
                    DATA + 0x440,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus
                    .write32(DATA + 0xb0, if seed % 2 == 0 { 0 } else { FAKE });
            },
        );
    }

    pub(crate) fn mc_state_test(generated: GuestFn, hand: GuestFn, salt: u64, fn_off: u32) {
        check(generated, hand, 30, &|c, seed| {
            fakes::machine(c, seed, salt, 0);
            globals_2(c);
            c.g[A0][0] = OBJ as u64;
            w_2(c, OBJ + 0x434, OBJ2);
            w_2(c, OBJ2, VT);
            // The movie object's virtual "idle" query.
            slot_2(
                c,
                VT,
                0x58,
                0,
                if seed % 3 == 0 { ONE_STUB } else { ZERO_STUB },
            );
            // The state's "busy" function pointer.
            w_2(
                c,
                OBJ + fn_off,
                if seed % 2 == 0 { ZERO_STUB } else { ONE_STUB },
            );
            w_2(c, OBJ + 0x748, VT + 0x100);
            slot_2(c, VT + 0x100, 8, 0, fakes::FAKE);
        });
    }

    #[test]
    fn mc_manager_state37_matches() {
        mc_state_test(
            ee::s06::f_0023e3c0__generated,
            mc_manager_state37,
            0x23e3c0,
            0x6c,
        );
    }

    #[test]
    fn mc_manager_state1b_matches() {
        mc_state_test(
            ee::s06::f_00240960__generated,
            mc_manager_state1b,
            0x240960,
            0x30,
        );
    }

    #[test]
    fn mc_manager_state38_matches() {
        check(
            ee::s06::f_0023e428__generated,
            mc_manager_state38,
            8,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0023_e428, 2);
                c.g[A0][0] = OBJ_2 as u64;
                let holder = DATA + 0x3000;
                let vt = DATA + 0x3100;
                w(c, OBJ_2 + 0x434, holder);
                w(c, holder, vt);
                // Slot 0x58 returns a zero word from `holder + 4` (the movie player
                // is idle); the object's `+0x6c` hook is the same predicate.
                c.bus.write16(vt + 0x58, 0);
                w(c, vt + 0x5c, 0x002c_a3b0);
                w(c, OBJ_2 + 0x6c, 0x002c_a3b0);
                w(c, OBJ_2 + 4, if seed % 2 == 0 { 0 } else { 8 });
                w(c, OBJ_2 + 0xc8, 1);
            },
        );
    }

    #[test]
    fn mc_manager_return_to_slot_state_matches() {
        check(
            ee::s06::f_00242050__generated,
            mc_manager_return_to_slot_state,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x24_2050, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0x434, DATA + 0x2200);
                w(c, DATA + 0x2000 + 0x748, DATA + 0x2400);
                // The busy probe: itself (strcasecmp -> 0, i.e. not busy) or a fake
                // (non-zero, i.e. busy).
                slot_3(
                    c,
                    DATA + 0x2200,
                    0x58,
                    0,
                    if seed % 2 == 0 { 0x0041_aa88 } else { FAKE },
                );
                slot_3(c, DATA + 0x2400, 0x8, 0, FAKE);
                c.pc = 0x0024_2050;
            },
        );
    }

    /// The zeroed BELibrary dummy and the world.
    pub(crate) const BE_2: u32 = DATA + 0x1_0000;

    pub(crate) const W: u32 = DATA + 0x1_4000;

    /// Tiny translated functions used as virtual-method targets.
    pub(crate) const RET_ZERO: u32 = 0x001a_8828;

    pub(crate) const RET_ONE: u32 = 0x001c_5dd0;

    pub(crate) fn wh(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }

    /// A vtable slot `{ short delta = 0; void (*fn)(); }`.
    pub(crate) fn ret_slot(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }

    pub(crate) fn run_2(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        // A setup call may unwind through a fake hook; that is fine for setup,
        // the port under test is compared separately.
        c.g[V0][0]
    }

    pub(crate) fn heap_init_2(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run_2(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// Scrambled registers, the fake hooks, a game-like `gp`, the BE_2 singleton
    /// and a world with no riders.
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE_2);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, GAME_GP.wrapping_sub(0x848), W);
        let game = W + 0x1000;
        w(c, W + 0x84, game);
        let race = W + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        r
    }

    /// [`base_3`] plus a working pool allocator and dlmalloc arena.
    pub(crate) fn alloc_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = base_3(c, seed, salt, calls);
        pool_allocator(c);
        heap_init_2(c);
        r
    }

    // ------------------------------------------------------------- memcard

    pub(crate) fn mc_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = alloc_base(c, seed, salt, 4);
        let self_ = DATA;
        let obj = DATA + 0x1000;
        let vtable = DATA + 0x1200;
        w(c, self_ + 0x434, obj);
        w(c, obj, vtable);
        ret_slot(
            c,
            vtable,
            0x58,
            if seed % 4 == 0 { RET_ONE } else { RET_ZERO },
        );
        w(
            c,
            self_ + 0x84,
            match seed % 4 {
                1 => RET_ONE,
                _ => RET_ZERO,
            },
        );
        w(
            c,
            self_ + 0x6c,
            if seed % 4 == 3 { RET_ONE } else { RET_ZERO },
        );
        c.g[A0][0] = self_ as u64;
    }

    #[test]
    fn mc_manager_state17_matches() {
        check(
            ee::s06::f_0023f258__generated,
            mc_manager_state17,
            12,
            &|c, seed| {
                mc_setup(c, seed, 0x23f2_58);
            },
        );
    }

    #[test]
    fn mc_manager_state30_matches() {
        check(
            ee::s06::f_00240f30__generated,
            mc_manager_state30,
            12,
            &|c, seed| {
                mc_setup(c, seed, 0x240f_30);
            },
        );
    }

    #[test]
    fn mc_manager_end_progress_matches() {
        check(
            ee::s06::f_00241180__generated,
            mc_manager_end_progress,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2411_80, 4);
                let self_ = DATA;
                let obj = DATA + 0x1000;
                let widget = DATA + 0x2000;
                let vtable = DATA + 0x1200;
                w(c, self_ + 0x118, (seed % 2) as u32);
                w(c, self_ + 0x434, obj);
                w(c, obj, vtable);
                ret_slot(c, vtable, 0x160, RET_ZERO);
                w(c, self_ + 0x748, widget);
                ret_slot(c, widget, 0x8, RET_ZERO);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_on_card_scanned_matches() {
        check(
            ee::s06::f_00241fd0__generated,
            mc_manager_on_card_scanned,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x241f_d0, 4);
                let self_ = DATA;
                let obj = DATA + 0x1000;
                let widget = DATA + 0x2000;
                let vtable = DATA + 0x1200;
                w(
                    c,
                    self_ + 0xac,
                    match seed % 3 {
                        0 => RET_ONE,
                        _ => RET_ZERO,
                    },
                );
                w(c, self_ + 0x434, obj);
                w(c, obj, vtable);
                ret_slot(
                    c,
                    vtable,
                    0x58,
                    if seed % 2 == 0 { RET_ZERO } else { RET_ONE },
                );
                w(c, self_ + 0x748, widget);
                ret_slot(c, widget, 0x8, RET_ZERO);
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    /// A real function that returns 0 when its `+4` word is 0
    /// (`LocStringTable_ResolveString` over a zeroed object): used as a vtable
    /// slot_4 when a zero result drives a branch.
    pub(crate) const RET0: u32 = 0x002c_6dc0;

    #[test]
    fn mc_manager_state04_no_card_matches() {
        check(
            ee::s06::f_0023e820__generated,
            mc_manager_state04_no_card,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x23e8, 4);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                // The card object's vtable: real "return 0" for the idle branch.
                w(c, DATA + 0x400, DATA + 0x500);
                slot_4(
                    c,
                    DATA + 0x500,
                    0x58,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                slot_4(c, DATA + 0x500, 0x18, FAKE);
                w(c, m + 0x6c, if seed % 2 == 0 { 0 } else { RET0 });
                w(c, m + 0x84, if seed % 3 == 0 { 0 } else { RET0 });
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_state05_matches() {
        check(
            ee::s06::f_0023eb80__generated,
            mc_manager_state05,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x23eb, 4);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
                slot_4(
                    c,
                    DATA + 0x500,
                    0x58,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                slot_4(c, DATA + 0x500, 0x18, FAKE);
                w(c, m + 0x6c, if seed % 2 == 0 { 0 } else { RET0 });
                w(c, m + 0x84, if seed % 3 == 0 { 0 } else { RET0 });
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_state15_matches() {
        check(
            ee::s06::f_0023f1d8__generated,
            mc_manager_state15,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x23f1, 4);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
                slot_4(
                    c,
                    DATA + 0x500,
                    0x58,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                slot_4(c, DATA + 0x500, 0x18, FAKE);
                w(c, m + 0x6c, if seed % 2 == 0 { 0 } else { RET0 });
                w(c, m + 0x84, if seed % 3 == 0 { 0 } else { RET0 });
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_state10_save_matches() {
        check(
            ee::s06::f_0023f4f8__generated,
            mc_manager_state10_save,
            80,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x23f4, 3);
                let m = DATA;
                w(c, m + 0xe4, (seed % 100) as u32);
                w(c, m + 0xf0, 1 + (seed % 7) as u32);
                w(c, m + 0x434, DATA + 0x400);
                if seed % 4 != 0 {
                    w(c, m + 0x748, DATA + 0x600);
                    w(c, DATA + 0x600 + 8, 0);
                    w(c, DATA + 0x600 + 0xc, RET0);
                }
                w(c, DATA + 0x400, DATA + 0x500);
                slot_4(
                    c,
                    DATA + 0x500,
                    0x248,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                let _ = &mut r;
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_state1f_load_matches() {
        check(
            ee::s06::f_002400b0__generated,
            mc_manager_state1f_load,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2400, 3);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                if seed % 4 != 0 {
                    w(c, m + 0x748, DATA + 0x600);
                    w(c, DATA + 0x600 + 8, 0);
                    w(c, DATA + 0x600 + 0xc, RET0);
                }
                w(c, m + 0x2c, RET0);
                w(c, DATA + 0x400, DATA + 0x500);
                slot_4(
                    c,
                    DATA + 0x500,
                    0x248,
                    if seed % 2 == 0 { RET0 } else { FAKE },
                );
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_begin_slot_select_matches() {
        check(
            ee::s06::f_00241380__generated,
            mc_manager_begin_slot_select,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2413, 3);
                let m = DATA;
                w(c, m + 0x434, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
                slot_4(c, DATA + 0x500, 0x18, FAKE);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn mc_manager_get_save_type_tag_matches() {
        check(
            ee::s06::f_00241d40__generated,
            mc_manager_get_save_type_tag,
            80,
            &|c, seed| {
                let _ = base_4(c, seed, 0x241d, 1);
                let ty: u32 = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    _ => u32::MAX,
                };
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = ty as u64;
                c.g[A2][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn mc_manager_state14_matches() {
        check(
            ee::s06::f_0023f150__generated,
            mc_manager_state14,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x23f150, 4);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x434, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write16(DATA + 0x1100 + 0x58, 0);
                c.bus.write32(DATA + 0x84, FAKE);
                c.bus.write32(LOG_RESULTS, 0x11);
                match seed % 4 {
                    // Card absent, no pending operation: report the completion.
                    0 => {
                        c.bus.write32(DATA + 0x1100 + 0x5c, 0x002d_1bd8);
                        c.bus.write32(DATA + 0x6c, 0x002d_1bd8);
                        c.bus.write32(GAME_GP + 0x16c8, 0);
                    }
                    // Card absent, pending operation: end it.
                    2 => {
                        c.bus.write32(DATA + 0x1100 + 0x5c, 0x002d_1bd8);
                        c.bus.write32(DATA + 0x6c, FAKE);
                        c.bus.write32(GAME_GP + 0x16c8, 0);
                    }
                    // Card present: the poll returns nonzero and the state exits.
                    _ => {
                        c.bus.write32(DATA + 0x1100 + 0x5c, FAKE);
                        c.bus.write32(DATA + 0x6c, FAKE);
                    }
                }
            },
        );
    }

    #[test]
    fn mc_manager_state25_matches() {
        check(
            ee::s06::f_0023fe70__generated,
            mc_manager_state25,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x23fe, 0);
                env(c);
                let this = DATA + 0x400;
                let o = DATA + 0x800;
                let vt = DATA + 0x900;
                w32(c, this + 0x434, o);
                w32(c, o, vt);
                slot_5(c, vt, 0x58, if seed % 2 == 0 { EMPTY } else { RET_ZERO_2 });
                w32(c, this + 0x6c, [0, RET_ZERO_2][((seed / 2) % 2) as usize]);
                w32(c, this + 0x84, [0, RET_ZERO_2][((seed / 4) % 2) as usize]);
                w32(c, this + 0x68, [0, RET_ZERO_2][((seed / 8) % 2) as usize]);
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `V0 = *(a0 + 0xb0); jr ra` — a vtable slot_6 whose result the test
    /// controls through memory.
    pub(crate) const LOAD_B0: u32 = 0x0027_72b8;

    #[test]
    fn mc_manager_state0e_matches() {
        check(
            ee::s06::f_0023f3a0__generated,
            mc_manager_state0e,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x23f3a0, 4);
                let obj = DATA + 0x1000;
                let movie = DATA + 0x2000;
                let vt = DATA + 0x2_0000;
                let ui = DATA + 0x3000;
                let uivt = DATA + 0x3_0000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x434, movie);
                w(c, movie, vt);
                slot_6(c, vt, 0x58, 0, LOAD_B0);
                slot_6(c, vt, 0x248, 0x40, LOAD_B0);
                w(c, movie + 0xb0, (seed % 2) as u32);
                w(c, movie + 0xf0, if seed % 3 == 0 { 0 } else { 1 });
                w(c, obj + 0x748, ui);
                w(c, ui, uivt);
                slot_6(c, uivt, 0x8, 0, FAKE);
                w(c, ui + 0xb0, r.next_u32());
            },
        );
    }

    #[test]
    fn mc_manager_state19_delete_matches() {
        check(
            ee::s06::f_00240768__generated,
            mc_manager_state19_delete,
            60,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x240768, 4);
                let obj = DATA + 0x1000;
                let movie = DATA + 0x2000;
                let vt = DATA + 0x2_0000;
                let ui = DATA + 0x3000;
                let uivt = DATA + 0x3_0000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x434, movie);
                w(c, movie, vt);
                slot_6(c, vt, 0x58, 0, LOAD_B0);
                slot_6(c, vt, 0x248, 0x40, LOAD_B0);
                w(c, movie + 0xb0, (seed % 2) as u32);
                w(c, movie + 0xf0, if seed % 3 == 0 { 0 } else { 1 });
                w(c, obj + 0x748, ui);
                w(c, ui, uivt);
                slot_6(c, uivt, 0x8, 0, FAKE);
                w(c, ui + 0xb0, r.next_u32());
            },
        );
    }

    #[test]
    fn mc_manager_state_24_matches() {
        check(
            ee::s06::f_0023fc70__generated,
            mc_manager_state_24,
            SEEDS_8,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = DATA + 0x900;
                w(c, obj, vt);
                slot_7(c, vt, 0x58, if seed & 1 == 0 { RET_ZERO } else { RET_ONE });
                w(c, this + 0x434, obj);
                w(
                    c,
                    this + 0x6c,
                    if seed & 2 == 0 { RET_ZERO } else { RET_ONE },
                );
                w(
                    c,
                    this + 0x84,
                    if seed & 4 == 0 { RET_ZERO } else { RET_ONE },
                );
                let result = DATA + 0xa00;
                slot_7(c, result, 8, RET_ZERO);
                w(c, this + 0x748, result);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn mc_manager_state36_matches() {
        check(
            ssx3_ee::s06::f_0023e320__generated,
            mc_manager_state36,
            20,
            &|c, seed| {
                let _r = base_7(c, seed, 0x23e320, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                let movie = DATA + 0x2000;
                w_3(c, this + 0x434, movie);
                w_3(c, movie, DATA + 0x2100);
                let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
                c.bus.write16(DATA + 0x2100 + 0x58, 0x2000);
                w_3(
                    c,
                    DATA + 0x2100 + 0x5c,
                    if seed % 2 == 0 { zero_fn } else { 0 },
                );
                w_3(c, this + 0x428, 0);
                w_3(c, this + 0x748, DATA + 0x3000);
                w_3(c, DATA + 0x3000, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 8, 0);
                w_3(c, DATA + 0x3100 + 0xc, 0);
                w_3(c, this + 0x98, 0);
                w_3(c, this + 0x9c, 0);
            },
        );
    }

    // -------------------------------------------- memory-card format states

    pub(crate) fn mc_manager_fixture(c: &mut Cpu, obj: u32, seed: u32) {
        w(c, obj + 0x108, (seed % 3) as u32);
        let a = obj + 0x1000;
        w(c, obj + 0x434, a);
        let vt = obj + 0x1100;
        w(c, a, vt);
        if seed % 2 == 0 {
            slot_to(c, vt, 0x5c, RET_ZERO);
            slot_to(c, vt, 0x4c, RET_ONE);
        } else {
            slot_8(c, vt, 0x5c, 0);
            slot_8(c, vt, 0x4c, 1);
        }
        slot_to(c, vt, 0x58, RET_ZERO);
        slot_to(c, vt, 0x48, RET_ZERO);
        let vt2 = obj + 0x1200;
        w(c, obj + 0x748, vt2);
        slot_8(c, vt2, 8, 2);
        slot_8(c, vt2, 0xc, 3);
    }

    #[test]
    fn mc_manager_state0b_format_matches() {
        check(
            ee::s06::f_0023f050__generated,
            mc_manager_state0b_format,
            SEEDS_9,
            &|c, seed| {
                let _ = base_8(c, seed, 0x23f0_50, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                mc_manager_fixture(c, obj, seed);
            },
        );
    }

    #[test]
    fn mc_manager_state28_format_matches() {
        check(
            ee::s06::f_00240b88__generated,
            mc_manager_state28_format,
            SEEDS_9,
            &|c, seed| {
                let _ = base_8(c, seed, 0x240b_88, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                mc_manager_fixture(c, obj, seed);
            },
        );
    }

    #[test]
    fn mc_manager_format_slot_total_space_matches() {
        check(
            ee::s06::f_002419f0__generated,
            mc_manager_format_slot_total_space,
            SEEDS,
            &|c, seed| {
                let mut r = base_9(c, seed, 0x2419);
                let idx = (seed % 3) as u32;
                let dev = DATA + idx * 0xe0;
                let dst = DATA + 0x2000;
                let variant = seed / 3 % 5;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = sx(dst);
                c.bus.write32(dev + 0x350, if variant == 0 { 0 } else { 1 });
                c.bus.write32(dev + 0x344, if variant == 1 { 0 } else { 1 });
                c.bus.write32(dev + 0x348, if variant == 2 { 0 } else { 1 });
                c.bus.write32(dev + 0x34c, if variant == 3 { 0 } else { 1 });
                c.bus.write32(dev + 0x358, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                fill(c, &mut r, dst, 0x100);
            },
        );
    }

    // ------------------------------------------------------ song allocation

    // ---------------------------------------------------- media control mgr

    pub(crate) fn mc_setup_2(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = alloc_base_3(c, seed, salt, 4);
        let holder = DATA + 0x2_000;
        let vt = DATA + 0x2_100;
        w(c, holder, vt);
        slot_9(c, vt, 0x58, FAKE);
        slot_9(c, vt, 0x5c, FAKE);
        w(c, DATA + 0x434, holder);
        let mgr = DATA + 0x2_200;
        let mvt = DATA + 0x2_300;
        w(c, mgr, 0);
        w(c, DATA + 0x748, mvt);
        slot_9(c, mvt, 8, FAKE);
        slot_9(c, mvt, 0xc, FAKE);
        w(c, DATA + 0x108, seed % 3);
        let _ = r.next_u32();
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn mc_manager_state12_save_matches() {
        check(
            ee::s06::f_0023fbb8__generated,
            mc_manager_state12_save,
            60,
            &|c, seed| mc_setup_2(c, seed, 0x23fb),
        );
    }

    #[test]
    fn mc_manager_state21_load_matches() {
        check(
            ee::s06::f_002405d0__generated,
            mc_manager_state21_load,
            60,
            &|c, seed| mc_setup_2(c, seed, 0x2405),
        );
    }

    #[test]
    fn mc_manager_state0f_save_matches() {
        check(
            ee::s06::f_0023f438__generated,
            mc_manager_state0f_save,
            200,
            &|c, seed| {
                machine(c, seed, 0x23_f438, 4);
                let this = DATA;
                let obj = DATA + 0x1000;
                let vt = DATA + 0x2000;
                let vt2 = DATA + 0x2400;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x434, obj);
                c.bus.write32(obj, vt);
                // Which of the three predicates report "save needed".
                let zero: u32 = 0x0028_b180; // returns gp+0x410 (0)
                let mode = seed % 4;
                let slot = |m: u32| if m != 0 { FAKE as u64 } else { zero as u64 };
                c.bus.write16(vt + 0x58, 0);
                c.bus.write32(vt + 0x5c, slot(mode) as u32);
                c.bus.write16(vt + 0x248, 0);
                c.bus.write32(vt + 0x24c, slot(mode / 2) as u32);
                c.bus.write16(vt + 0x250, 0);
                c.bus.write32(vt + 0x254, slot(mode / 3) as u32);
                c.bus.write32(this + 0x748, vt2);
                c.bus.write16(vt2 + 8, 0);
                c.bus.write32(vt2 + 0xc, FAKE + 0x40);
            },
        );
    }

    #[test]
    fn mc_manager_on_operation_done_matches() {
        strict(
            ee::s06::f_0023ea90__generated,
            mc_manager_on_operation_done,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23ea, 10);
                let this = DATA;
                c.bus
                    .write32(this + 0xbc, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x12c, (seed % 3) as u32);
                c.bus.write32(this + 0x118, (r.next_u32() % 2) as u32);
                c.bus.write32(this + 0x434, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x160, 0);
                c.bus.write32(DATA + 0x2000 + 0x164, FAKE);
                c.bus.write32(this + 0x748, DATA + 0x3000);
                c.bus.write16(DATA + 0x3000 + 8, 0);
                c.bus.write32(DATA + 0x3000 + 0xc, FAKE);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mc_manager_state22_load_matches() {
        strict(
            ee::s06::f_0023ff00__generated,
            mc_manager_state22_load,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x23ff, 10);
                let this = DATA;
                c.bus
                    .write32(this + 0x108, [0u32, 1, 5][(seed % 3) as usize]);
                c.bus.write32(this + 0x434, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x58, 0);
                c.bus.write32(DATA + 0x2000 + 0x5c, FAKE);
                c.bus.write16(DATA + 0x2000 + 0x248, 0);
                c.bus.write32(DATA + 0x2000 + 0x24c, FAKE);
                c.bus.write32(this + 0x748, DATA + 0x3000);
                c.bus.write16(DATA + 0x3000 + 8, 0);
                c.bus.write32(DATA + 0x3000 + 0xc, FAKE);
                c.bus.write32(this + 0x2c, FAKE);
                c.bus.write32(this + 0x28, FAKE);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mcmanager_state27_format_matches() {
        check(
            ee::s06::f_00240ab0__generated,
            mcmanager_state27_format,
            12,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x240ab0, 3);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let iface = DATA + 0x800;
                clear_2(c, iface, 0x100);
                w(c, this + 0x434, iface);
                let vt = vtable(c, &mut r, &[0x58, 0x1a8]);
                w(c, iface, vt);
                w(c, this + 0x88, 0);
                w(c, this + 0x748, 0);
                w(c, this + 0x6c, 0);
                w(c, this + 0x84, 0);
            },
        );
    }

    #[test]
    fn mcmanager_state18_matches() {
        check(
            ee::s06::f_00240688__generated,
            mcmanager_state18,
            12,
            &|c, seed| {
                let mut r = base_11(c, seed, 0x240688, 3);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                let iface = DATA + 0x800;
                clear_2(c, iface, 0x100);
                w(c, this + 0x434, iface);
                let vt = vtable(c, &mut r, &[0x58, 0x1a8, 0x248]);
                w(c, iface, vt);
                w(c, this + 0x88, 0);
                w(c, this + 0x748, 0);
                w(c, this + 0x68, 0);
                w(c, this + 0x6c, 0);
            },
        );
    }

    #[test]
    fn mc_manager_begin_replay_write_matches() {
        diff(
            ee::s06::f_002412a0__generated,
            mc_manager_begin_replay_write,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x24_12a0);
                let _ = r.next_u32();
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0xf8, (seed % 3) as u32);
                w(c, GAME_GP.wrapping_sub(0x478), 0);
            },
        );
    }

    #[test]
    fn mc_manager_state0a_format_matches() {
        diff(
            ee::s06::f_0023ef68__generated,
            mc_manager_state0a_format,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x23_ef68);
                c.g[A0][0] = sx(DATA);
                w(
                    c,
                    DATA + 0x338,
                    if seed % 3 == 0 { 0 } else { r.next_u32() },
                );
                w(c, DATA + 0x434, DATA + 0x2000);
                w(c, DATA + 0x2000, 0);
                w(c, DATA + 0x6c, (seed % 4) as u32);
                w(c, DATA + 0x748, r.next_u32());
            },
        );
    }

    #[test]
    fn mcmanager_state1a_delete_matches() {
        strict_2(
            ssx3_ee::s06::f_00240860__generated,
            mcmanager_state1a_delete,
            SEEDS,
            &|c, seed| {
                let _ = base_12(c, seed, 0x0860);
                let this = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let list = DATA + 0x3000;
                let vt = DATA + 0x4000;
                let item = DATA + 0x5000;
                let vt2 = DATA + 0x6000;
                for off in [0x58, 0x248, 0x28] {
                    c.bus.write16(vt + off, 0);
                    w(c, vt + off + 4, FAKE);
                }
                w(c, screen, vt);
                w(c, list, 0);
                w(c, this + 0x434, screen);
                w(c, this + 0x428, list);
                w(c, this + 0x748, item);
                w(c, item, vt2);
                c.bus.write16(vt2 + 8, 0);
                w(c, vt2 + 0xc, FAKE);
                w(c, this + 0x108, if seed % 2 == 0 { 0 } else { 2 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `OcclusionMan_BuildOccluderPlanes`: random eyes and corners (some
    /// degenerate: a corner on the eye, two corners equal).
    #[test]
    fn occlusion_man_build_occluder_planes_matches() {
        check(
            ee::s06::f_002429b0__generated,
            occlusion_man_build_occluder_planes,
            300,
            &|c, seed| {
                let mut r = Rng(0x0024_29b0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0024_29b0;
                let eye = DATA;
                let occ = DATA + 0x100;
                let v = |r: &mut Rng| [r.f32(500.0), r.f32(500.0), r.f32(500.0), r.edge_f32()];
                let e = v(&mut r);
                put_vec(c, eye, e);
                for i in 0..4 {
                    let mut k = v(&mut r);
                    if seed % 11 == i {
                        k = e;
                    }
                    put_vec(c, occ + OCC_CORNERS + 0x10 * i, k);
                }
                if seed % 13 == 0 {
                    let k = c.bus.read128(occ + OCC_CORNERS);
                    c.bus.write128(occ + OCC_CORNERS + 0x10, k);
                }
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(eye);
                c.g[A2][0] = sx(occ);
            },
        );
    }
}
