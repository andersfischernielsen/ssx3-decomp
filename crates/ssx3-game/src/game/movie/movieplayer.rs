//! `src/movie/movieplayer.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
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
use crate::game::fe::fememcardcallbacks::memcard_text;
use crate::game::fe::fememcardcallbacks::MEMCARD_CARD_VT_SAVE_COUNT;
use crate::game::fe::fememcardcallbacks::MEMCARD_CARD_VT_SAVE_SIZE;
use crate::game::fe::fememcardcallbacks::MEMCARD_CARD_VT_SLOT_NAME;
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
    (
        0x0023_c808,
        mc_manager_get_instance,
        "MCManager_GetInstance",
    ),
    (0x0023_c8f0, mc_manager_begin_load, "MCManager_BeginLoad"),
    (
        0x0023_caa0,
        mc_manager_set_data_size,
        "MCManager_SetDataSize",
    ),
    (0x0023_cae8, mc_manager_begin_save, "MCManager_BeginSave"),
    (
        0x0023_cc58,
        mc_manager_set_data_size_wrapper,
        "MCManager_SetDataSize_Wrapper",
    ),
    (
        0x0023_cc78,
        mc_manager_format_save_desc,
        "MCManager_FormatSaveDesc",
    ),
    (
        0x0023_cdb0,
        mc_manager_build_save_title,
        "MCManager_BuildSaveTitle",
    ),
    (
        0x0023_d660,
        mc_manager_update_mode1,
        "MCManager_UpdateMode1",
    ),
    (
        0x0023_d7d8,
        mc_manager_update_states,
        "MCManager_UpdateStates",
    ),
    (0x0023_e268, mc_manager_state31, "MCManager_State31"),
    (0x0023_e2c0, mc_manager_state32, "MCManager_State32"),
    (0x0023_e4f0, mc_manager_state34, "MCManager_State34"),
    (
        0x0023_e8a0,
        mc_manager_classify_card_state,
        "MCManager_ClassifyCardState",
    ),
    (0x0023_f308, mc_manager_state0d, "MCManager_State0D"),
    (
        0x0023_f578,
        mc_manager_state11_save,
        "MCManager_State11_Save",
    ),
    (0x0023_fb18, mc_manager_set_slot, "MCManager_SetSlot"),
    (0x0023_fd40, mc_manager_state16, "MCManager_State16"),
    (0x0023_fff0, mc_manager_state1e, "MCManager_State1E"),
    (0x0024_09c8, mc_manager_state09, "MCManager_State09"),
    (0x0024_0cc0, mc_manager_state2b, "MCManager_State2B"),
    (
        0x0024_0d90,
        mc_manager_state2c_delete,
        "MCManager_State2C_Delete",
    ),
    (0x0024_0f10, mc_manager_state2f, "MCManager_State2F"),
    (
        0x0024_0fb0,
        mc_manager_write_save_by_name,
        "MCManager_WriteSaveByName",
    ),
    (
        0x0024_1050,
        mc_manager_write_save_by_slot,
        "MCManager_WriteSaveBySlot",
    ),
    (
        0x0024_10d0,
        mc_manager_clamp_file_index,
        "MCManager_ClampFileIndex",
    ),
    (0x0024_1400, mc_manager_next_slot, "MCManager_NextSlot"),
    (0x0024_1540, mc_manager_prev_slot, "MCManager_PrevSlot"),
    (0x0024_1678, mc_manager_prev_file, "MCManager_PrevFile"),
    (0x0024_16c0, mc_manager_next_file, "MCManager_NextFile"),
    (
        0x0024_1928,
        mc_manager_format_slot_free_space,
        "MCManager_FormatSlotFreeSpace",
    ),
    (
        0x0024_1fb0,
        mc_manager_read_current_dir_info,
        "MCManager_ReadCurrentDirInfo",
    ),
    (0x0023_cf38, mc_manager_v01, "MCManager_v01"),
    (
        0x0023_e540,
        mc_manager_state03_check,
        "MCManager_State03_Check",
    ),
    (
        0x0023_ec00,
        mc_manager_state06_check,
        "MCManager_State06_Check",
    ),
    (
        0x0023_f698,
        mc_manager_state07_check,
        "MCManager_State07_Check",
    ),
    (
        0x0024_0130,
        mc_manager_state20_load,
        "MCManager_State20_Load",
    ),
    (
        0x0024_02d0,
        mc_manager_state1d_check,
        "MCManager_State1D_Check",
    ),
    (
        0x0024_1710,
        mc_manager_get_card_error_text,
        "MCManager_GetCardErrorText",
    ),
    (
        0x0024_1b20,
        mc_manager_read_card_status,
        "MCManager_ReadCardStatus",
    ),
    (
        0x0024_1e18,
        mc_manager_get_status_text,
        "MCManager_GetStatusText",
    ),
    (
        0x0024_20c8,
        mc_manager_read_dir_info,
        "MCManager_ReadDirInfo",
    ),
    (0x0024_2288, mc_manager_ctor, "MCManager_ctor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_MANAGER_END_OPERATION_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::MC_MANAGER_REQUEST_CARD_INFO;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::OPERATOR_NEW_NAMED;

    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;

    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;

    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION_3;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_REQUEST_CARD_INFO_2;
    pub(crate) use crate::game::fe::fepopupmisc::callee::MC_MANAGER_SET_MODE;

    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;

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

    pub const MC_MANAGER_CTOR: Callee = Callee {
        addr: 0x0024_2288,
        f: ee::s06::MCManager_ctor,
    };

    pub const MC_MANAGER_SET_DATA_SIZE: Callee = Callee {
        addr: 0x0023_caa0,
        f: ee::s06::MCManager_SetDataSize,
    };

    pub const MC_MANAGER_RESET: Callee = Callee {
        addr: 0x0023_fb58,
        f: ee::s06::MCManager_Reset,
    };

    pub const MC_MANAGER_READ_DIR_INFO: Callee = Callee {
        addr: 0x0024_20c8,
        f: ee::s06::MCManager_ReadDirInfo,
    };

    pub const MC_MANAGER_CLAMP_FILE_INDEX: Callee = Callee {
        addr: 0x0024_10d0,
        f: ee::s06::MCManager_ClampFileIndex,
    };

    pub const MC_MANAGER_CARD_SET_STATE18: Callee = Callee {
        addr: 0x0023_cc20,
        f: ee::s06::MCManager_CardSetState18,
    };

    pub const MC_MANAGER_GET_PRODUCT_CODE: Callee = Callee {
        addr: 0x0024_1aa0,
        f: ee::s06::MCManager_GetProductCode,
    };

    pub const MC_MANAGER_GET_SAVE_TYPE_TAG: Callee = Callee {
        addr: 0x0024_1d40,
        f: ee::s06::MCManager_GetSaveTypeTag,
    };

    pub const MC_MANAGER_SET_SLOT: Callee = Callee {
        addr: 0x0023_fb18,
        f: ee::s06::MCManager_SetSlot,
    };

    pub const MC_MANAGER_REFRESH_SLOT: Callee = Callee {
        addr: 0x0023_fae0,
        f: ee::s06::MCManager_RefreshSlot,
    };

    pub const MC_MANAGER_GET_NUM_SLOTS: Callee = Callee {
        addr: 0x0024_1ac0,
        f: ee::s06::MCManager_GetNumSlots,
    };

    pub const MC_MANAGER_STATE31: Callee = Callee {
        addr: 0x0023_e268,
        f: ee::s06::MCManager_State31,
    };

    pub const MC_MANAGER_STATE33: Callee = Callee {
        addr: 0x0023_e498,
        f: ee::s06::MCManager_State33,
    };

    pub const MC_MANAGER_STATE36: Callee = Callee {
        addr: 0x0023_e320,
        f: ee::s06::MCManager_State36,
    };

    pub const MC_MANAGER_FORMAT_SAVE_DESC: Callee = Callee {
        addr: 0x0023_cc78,
        f: ee::s06::MCManager_FormatSaveDesc,
    };

    pub const BE_SAVE_INT_GET_PROFILE_SAVE_SIZE: Callee = Callee {
        addr: 0x0015_2ba8,
        f: ee::s01::BESaveInt_GetProfileSaveSize,
    };

    pub const BE_SAVE_INT_GET_SYSTEM_SAVE_SIZE: Callee = Callee {
        addr: 0x0015_2948,
        f: ee::s01::BESaveInt_GetSystemSaveSize,
    };

    pub const BE_NEW_PLAYER_INT_GET_NAME: Callee = Callee {
        addr: 0x0014_7170,
        f: ee::s01::BENewPlayerInt_GetName,
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;

    pub const MC_MANAGER_READ_CURRENT_DIR_INFO: Callee = Callee {
        addr: 0x0024_1fb0,
        f: ee::s06::f_00241fb0,
    };

    pub const MC_MANAGER_BUILD_SAVE_TITLE: Callee = Callee {
        addr: 0x0023_cdb0,
        f: ee::s06::f_0023cdb0,
    };

    /// `int UStr_Len_2(const u16* s)`: the length of a UTF-16 string.
    pub const USTR_LEN_2: Callee = Callee {
        addr: 0x002c_24d0,
        f: ee::s08::UStr_Len_2,
    };

    /// `MCManager_PollCardStatus(self)`: `MCManager_ReadCardStatus`, then 0
    /// when the flags equal `+0x43c` and the word at `+0x440` is set;
    /// otherwise stores the flags at `+0x43c` (when they changed, `+0x440` is
    /// first set from the hook at `+0xb0`) and returns `+0x440 ^ 1`.
    pub const MC_MANAGER_POLL_CARD_STATUS: Callee = Callee {
        addr: 0x0024_1cd8,
        f: ee::s06::f_00241cd8,
    };

    /// `MCManager_SetStateNoCard(self)`: post state 4.
    pub const MC_MANAGER_SET_STATE_NO_CARD: Callee = Callee {
        addr: 0x0023_eb50,
        f: ee::s06::f_0023eb50,
    };

    /// `MCManager_ReturnToSlotState(self)`: post state 2 or 3.
    pub const MC_MANAGER_RETURN_TO_SLOT_STATE: Callee = Callee {
        addr: 0x0024_2050,
        f: ee::s06::f_00242050,
    };

    /// `MCManager_FinishLoad(self)`: card driver slot 0x108, post 0x24, then
    /// the hook at `+0x2c`.
    pub const MC_MANAGER_FINISH_LOAD: Callee = Callee {
        addr: 0x0024_1240,
        f: ee::s06::f_00241240,
    };

    /// `MCManager_CardSetState1E(self)`: card driver slot 0x100.
    pub const MC_MANAGER_CARD_SET_STATE_1E: Callee = Callee {
        addr: 0x0023_ca70,
        f: ee::s06::f_0023ca70,
    };

    /// `UStr_Cmp(a, b)` (through the function table).
    pub const USTR_CMP: u32 = 0x003d_bdb0;

    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
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

    pub const VOIP_RPC_GET_STATUS: u32 = 0x003f_6ba0;

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

pub(crate) mod mcus {

    /// `mState`: the state machine's current state.
    pub const STATE: u32 = 0x130;

    /// `mSaveOk`, `mLoadOk`, `mDeleteOk`, `mCancelOk`: what the screen offers.
    pub const SAVE_OK: u32 = 0xcc;

    pub const LOAD_OK: u32 = 0xd0;

    pub const DELETE_OK: u32 = 0xd4;

    pub const CANCEL_OK: u32 = 0xd8;

    /// `mCardChanged`.
    pub const CARD_CHANGED: u32 = 0xc8;

    /// `mFileCursor`: the highlighted save file.
    pub const FILE_CURSOR: u32 = 0xf8;

    /// `mSave`, `mDelete`: an operation is pending.
    pub const SAVE: u32 = 0xbc;

    pub const DELETE: u32 = 0xc0;

    /// The word `MCManager_BuildSaveTitle` and `MCManager_FormatSaveDesc`
    /// switch on (0, 2 and other values); the constructor sets it to 2.
    pub const SAVE_MODE: u32 = 0xb4;

    /// The floats at +0x110 (set to 1.0) and +0x114 (set to one of the
    /// `gp - 0x5120..=0x5148` constants).
    pub const RAMP: u32 = 0x110;

    pub const RAMP_STEP: u32 = 0x114;

    /// Input callbacks (`bool (*)(void)`).
    pub const CB_SAVE_PRESSED: u32 = 0x64;

    pub const CB_DELETE_PRESSED: u32 = 0x68;

    pub const CB_DEVICE_SELECTED: u32 = 0x74;

    pub const CB_LOAD_PRESSED: u32 = 0x78;

    pub const CB_NEXT_DEVICE: u32 = 0x7c;

    pub const CB_PREV_DEVICE: u32 = 0x80;

    pub const CB_FORMAT_PRESSED: u32 = 0x88;

    /// `mCheckCard`, `mCheckDeviceTicks`: the periodic card poll.
    pub const CHECK_CARD: u32 = 0x338;

    pub const CHECK_TICKS: u32 = 0x33c;

    /// `mDevices[]`: 0xe0 bytes each; `bPluggedIn` at +0xc.
    pub const DEVICES: u32 = 0x344;

    pub const DEVICE_SIZE: u64 = 0xe0;

    pub const DEVICE_PLUGGED_IN: u32 = DEVICES + 0xc;

    /// Words of the device record that `MCManager_ReadDirInfo` sets to 1 and
    /// clears on the matching failure (the first word is `DEVICES` itself,
    /// cleared when the card is not detected).
    pub const DEVICE_FORMATTED: u32 = DEVICES + 4;

    pub const DEVICE_HAS_NAME: u32 = DEVICES + 8;

    /// The free-space count (`VT_FREE_SPACE`), read for the current slot.
    pub const DEVICE_FREE_SPACE: u32 = DEVICES + 0x10;

    /// The device name (UTF-16).
    pub const DEVICE_NAME: u32 = DEVICES + 0x18;

    pub const DEVICE_NAME_CHARS: u64 = 100;

    pub const DEVICE_NAME_BYTES: u64 = DEVICE_NAME_CHARS * 2;

    /// `mSelectedDevice`, `mCurrCheckDevice`, `mLastCheckDevice`.
    pub const SELECTED: u32 = 0x428;

    pub const CURR_CHECK: u32 = 0x42c;

    pub const LAST_CHECK: u32 = 0x430;

    /// `mpMemCard`: the card driver object (its vtable holds
    /// `{short delta; fn}` slots).
    pub const CARD: u32 = 0x434;

    /// The card status flags (`MCManager_ReadCardStatus`) as the last
    /// `MCManager_PollCardStatus` stored them.
    pub const CARD_STATUS: u32 = 0x43c;

    /// The message object: slot 8 of the vtable it points at posts a state.
    pub const MSG: u32 = 0x748;

    /// Card driver vtable slots: offsets into the vtable of the driver at
    /// `CARD` (+0x434).
    pub const VT_POLL: u32 = 0x18;

    pub const VT_BUSY: u32 = 0x58;

    pub const VT_SET_CHECK_DEVICE: u32 = 0x28;

    pub const VT_DEVICE_PRESENT: u32 = 0xd0;

    pub const VT_START_CHECK: u32 = 0xd8;

    pub const VT_CHANGED: u32 = 0x178;

    pub const VT_CAN_SAVE: u32 = 0x1a8;

    pub const VT_DEVICE_OK: u32 = 0x1a0;

    pub const VT_CAN_LOAD: u32 = 0x1b8;

    pub const VT_CARD_CHANGED: u32 = 0x1f8;

    pub const VT_FILE_EXISTS: u32 = 0x218;

    /// Any of these three true: the card is reported as not detected, and
    /// `MCManager_ReadDirInfo` clears the device's first word.
    pub const VT_NOT_DETECTED_A: u32 = 0x180;

    pub const VT_NOT_DETECTED_B: u32 = 0x190;

    pub const VT_NOT_DETECTED_C: u32 = 0x1e0;

    /// False: the card is reported as not formatted.
    pub const VT_FORMATTED: u32 = 0x1c0;

    /// True: the device's name is read (`VT_READ_NAME`) and shown.
    pub const VT_HAS_NAME: u32 = 0x1c8;

    /// `fn(card, slot, u16* name, int max_chars)`: fills the device's name.
    pub const VT_READ_NAME: u32 = 0x230;

    /// A count compared with the space a new save needs; kept in
    /// `DEVICE_FREE_SPACE` for the current slot.
    pub const VT_FREE_SPACE: u32 = 0xa0;

    /// The status flag slots of `MCManager_ReadCardStatus`: bit 0 is the low
    /// bit of `VT_STATUS_BIT0`, bit 1 is set when `VT_STATUS_BIT1_A` or
    /// `VT_STATUS_BIT1_B` is true, bit 4 when `VT_CAN_LOAD` or
    /// `VT_STATUS_BIT4` is false.
    pub const VT_STATUS_BIT0: u32 = 0x188;

    pub const VT_STATUS_BIT1_A: u32 = 0x198;

    pub const VT_STATUS_BIT1_B: u32 = 0x1d8;

    pub const VT_STATUS_BIT4: u32 = 0x1d0;

    /// The switch's jump table (states 2..=56) in `.rodata`.
    pub const JUMP_TABLE: u32 = 0x0047_c280;
}

pub(crate) mod mcus_callee {
    use super::ee;
    use super::Callee;

    pub const READ_DIR_INFO: Callee = Callee {
        addr: 0x0024_20c8,
        f: ee::s06::f_002420c8,
    };

    pub const NEXT_SLOT: Callee = Callee {
        addr: 0x0024_1400,
        f: ee::s06::f_00241400,
    };

    pub const PREV_SLOT: Callee = Callee {
        addr: 0x0024_1540,
        f: ee::s06::f_00241540,
    };

    pub const ON_CARD_SCANNED: Callee = Callee {
        addr: 0x0024_1fd0,
        f: ee::s06::f_00241fd0,
    };

    pub const GET_NUM_SLOTS: Callee = Callee {
        addr: 0x0024_1ac0,
        f: ee::s06::f_00241ac0,
    };

    pub const REQUEST_CARD_INFO: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::f_0023fb20,
    };

    pub const STATE38: Callee = Callee {
        addr: 0x0023_e428,
        f: ee::s06::f_0023e428,
    };

    /// `MEM_fill(dst, byte, n)` (reached through the function table).
    pub const MEM_FILL: u32 = 0x003e_6448;
}

/// First case block of the switch; the blocks are 0x10 apart, in this order
/// (the jump table maps a state to one of them). State 0x38's block
/// (0x23e240) and the no-handler target (0x23e248, state 8) are separate.
pub(crate) const MCUS_CASE_BASE: u32 = 0x0023_def0;

pub(crate) const MCUS_CASE_STATE38: u32 = 0x0023_e240;

pub(crate) const MCUS_CASE_NONE: u32 = 0x0023_e248;

/// The state handler each case block calls (`MCManager_StateNN`, NN = the state
/// in hex), by block.
pub(crate) const MCUS_CASES: [Callee; 53] = [
    Callee {
        addr: 0x0023_e268,
        f: ee::s06::f_0023e268,
    }, // 0x23def0 MCManager_State31
    Callee {
        addr: 0x0023_e2c0,
        f: ee::s06::f_0023e2c0,
    }, // 0x23df00 MCManager_State32
    Callee {
        addr: 0x0023_e2c8,
        f: ee::s06::f_0023e2c8,
    }, // 0x23df10 MCManager_State35
    Callee {
        addr: 0x0023_e320,
        f: ee::s06::f_0023e320,
    }, // 0x23df20 MCManager_State36
    Callee {
        addr: 0x0023_e498,
        f: ee::s06::f_0023e498,
    }, // 0x23df30 MCManager_State33
    Callee {
        addr: 0x0023_e4f0,
        f: ee::s06::f_0023e4f0,
    }, // 0x23df40 MCManager_State34
    Callee {
        addr: 0x0023_e4f8,
        f: ee::s06::f_0023e4f8,
    }, // 0x23df50 MCManager_State02
    Callee {
        addr: 0x0023_e540,
        f: ee::s06::f_0023e540,
    }, // 0x23df60 MCManager_State03_Check
    Callee {
        addr: 0x0023_e820,
        f: ee::s06::f_0023e820,
    }, // 0x23df70 MCManager_State04_NoCard
    Callee {
        addr: 0x0023_eb80,
        f: ee::s06::f_0023eb80,
    }, // 0x23df80 MCManager_State05
    Callee {
        addr: 0x0023_ec00,
        f: ee::s06::f_0023ec00,
    }, // 0x23df90 MCManager_State06_Check
    Callee {
        addr: 0x0023_ef68,
        f: ee::s06::f_0023ef68,
    }, // 0x23dfa0 MCManager_State0A_Format
    Callee {
        addr: 0x0023_f050,
        f: ee::s06::f_0023f050,
    }, // 0x23dfb0 MCManager_State0B_Format
    Callee {
        addr: 0x0023_f0f8,
        f: ee::s06::f_0023f0f8,
    }, // 0x23dfc0 MCManager_State0C
    Callee {
        addr: 0x0023_fd40,
        f: ee::s06::f_0023fd40,
    }, // 0x23dfd0 MCManager_State16
    Callee {
        addr: 0x0023_f2d8,
        f: ee::s06::f_0023f2d8,
    }, // 0x23dfe0 MCManager_State13
    Callee {
        addr: 0x0023_f150,
        f: ee::s06::f_0023f150,
    }, // 0x23dff0 MCManager_State14
    Callee {
        addr: 0x0023_f1d8,
        f: ee::s06::f_0023f1d8,
    }, // 0x23e000 MCManager_State15
    Callee {
        addr: 0x0023_f258,
        f: ee::s06::f_0023f258,
    }, // 0x23e010 MCManager_State17
    Callee {
        addr: 0x0023_f308,
        f: ee::s06::f_0023f308,
    }, // 0x23e020 MCManager_State0D
    Callee {
        addr: 0x0023_f3a0,
        f: ee::s06::f_0023f3a0,
    }, // 0x23e030 MCManager_State0E
    Callee {
        addr: 0x0023_f438,
        f: ee::s06::f_0023f438,
    }, // 0x23e040 MCManager_State0F_Save
    Callee {
        addr: 0x0023_f4f8,
        f: ee::s06::f_0023f4f8,
    }, // 0x23e050 MCManager_State10_Save
    Callee {
        addr: 0x0023_f578,
        f: ee::s06::f_0023f578,
    }, // 0x23e060 MCManager_State11_Save
    Callee {
        addr: 0x0023_f698,
        f: ee::s06::f_0023f698,
    }, // 0x23e070 MCManager_State07_Check
    Callee {
        addr: 0x0023_fbb8,
        f: ee::s06::f_0023fbb8,
    }, // 0x23e080 MCManager_State12_Save
    Callee {
        addr: 0x0023_ff00,
        f: ee::s06::f_0023ff00,
    }, // 0x23e090 MCManager_State22_Load
    Callee {
        addr: 0x0023_ffc0,
        f: ee::s06::f_0023ffc0,
    }, // 0x23e0a0 MCManager_State23
    Callee {
        addr: 0x0023_fc70,
        f: ee::s06::f_0023fc70,
    }, // 0x23e0b0 MCManager_State24
    Callee {
        addr: 0x0023_fe70,
        f: ee::s06::f_0023fe70,
    }, // 0x23e0c0 MCManager_State25
    Callee {
        addr: 0x0023_fff0,
        f: ee::s06::f_0023fff0,
    }, // 0x23e0d0 MCManager_State1E
    Callee {
        addr: 0x0024_00b0,
        f: ee::s06::f_002400b0,
    }, // 0x23e0e0 MCManager_State1F_Load
    Callee {
        addr: 0x0024_0130,
        f: ee::s06::f_00240130,
    }, // 0x23e0f0 MCManager_State20_Load
    Callee {
        addr: 0x0024_02d0,
        f: ee::s06::f_002402d0,
    }, // 0x23e100 MCManager_State1D_Check
    Callee {
        addr: 0x0024_05d0,
        f: ee::s06::f_002405d0,
    }, // 0x23e110 MCManager_State21_Load
    Callee {
        addr: 0x0023_fd08,
        f: ee::s06::f_0023fd08,
    }, // 0x23e120 MCManager_State26
    Callee {
        addr: 0x0024_0688,
        f: ee::s06::f_00240688,
    }, // 0x23e130 MCManager_State18
    Callee {
        addr: 0x0024_0768,
        f: ee::s06::f_00240768,
    }, // 0x23e140 MCManager_State19_Delete
    Callee {
        addr: 0x0024_0860,
        f: ee::s06::f_00240860,
    }, // 0x23e150 MCManager_State1A_Delete
    Callee {
        addr: 0x0024_0960,
        f: ee::s06::f_00240960,
    }, // 0x23e160 MCManager_State1B
    Callee {
        addr: 0x0024_0800,
        f: ee::s06::f_00240800,
    }, // 0x23e170 MCManager_State1C
    Callee {
        addr: 0x0024_09c8,
        f: ee::s06::f_002409c8,
    }, // 0x23e180 MCManager_State09
    Callee {
        addr: 0x0024_0ab0,
        f: ee::s06::f_00240ab0,
    }, // 0x23e190 MCManager_State27_Format
    Callee {
        addr: 0x0024_0b88,
        f: ee::s06::f_00240b88,
    }, // 0x23e1a0 MCManager_State28_Format
    Callee {
        addr: 0x0024_0c30,
        f: ee::s06::f_00240c30,
    }, // 0x23e1b0 MCManager_State29
    Callee {
        addr: 0x0024_0c90,
        f: ee::s06::f_00240c90,
    }, // 0x23e1c0 MCManager_State2A
    Callee {
        addr: 0x0024_0cc0,
        f: ee::s06::f_00240cc0,
    }, // 0x23e1d0 MCManager_State2B
    Callee {
        addr: 0x0024_0d90,
        f: ee::s06::f_00240d90,
    }, // 0x23e1e0 MCManager_State2C_Delete
    Callee {
        addr: 0x0024_0eb0,
        f: ee::s06::f_00240eb0,
    }, // 0x23e1f0 MCManager_State2D
    Callee {
        addr: 0x0024_0ee0,
        f: ee::s06::f_00240ee0,
    }, // 0x23e200 MCManager_State2E
    Callee {
        addr: 0x0024_0f10,
        f: ee::s06::f_00240f10,
    }, // 0x23e210 MCManager_State2F
    Callee {
        addr: 0x0024_0f30,
        f: ee::s06::f_00240f30,
    }, // 0x23e220 MCManager_State30
    Callee {
        addr: 0x0023_e3c0,
        f: ee::s06::f_0023e3c0,
    }, // 0x23e230 MCManager_State37
];

/// `m + off`.
#[inline]
pub(crate) fn mcus_at(m: u32, off: u32) -> Addr {
    Addr(m.wrapping_add(off))
}

/// A card driver method: `vt_r = *obj`, `a0 = obj + vt[off].delta`,
/// `fn_r = vt[off].fn`, call it returning to `ret`. `obj_r` holds the object.
pub(crate) fn mcus_card_call(
    c: &mut Cpu,
    obj_r: usize,
    vt_r: usize,
    fn_r: usize,
    off: u32,
    ret: u32,
) -> Flow {
    let obj = r32(c, obj_r);
    c.g[vt_r][0] = Addr(obj).lw(c);
    let vt = r32(c, vt_r);
    c.g[A0][0] = Addr(vt.wrapping_add(off)).lh(c);
    c.g[fn_r][0] = Addr(vt.wrapping_add(off + 4)).lw(c);
    c.g[A0][0] = sx(obj.wrapping_add(r32(c, A0)));
    let target = r32(c, fn_r);
    call_ptr(c, target, ret)
}

/// `a1 = m->mpMemCard` (the object register of most card calls).
#[inline]
pub(crate) fn mcus_load_card(c: &mut Cpu, m: u32, reg: usize) {
    c.g[reg][0] = mcus_at(m, mcus::CARD).lw(c);
}

/// The compiled `xori`/`beqz` chain that tests `st` against `list`: true at
/// the first of `list[..n-1]` that matches (the last one is tested by the
/// caller's tail). Leaves `v0` as the chain does (`st ^` the next constant,
/// loaded in each branch's delay slot).
pub(crate) fn mcus_state_chain(c: &mut Cpu, st: u64, list: &[u64]) -> bool {
    c.g[V0][0] = st ^ list[0];
    if st == list[0] {
        return true;
    }
    for i in 1..list.len() - 1 {
        c.cycles += if i == 1 { 3 } else { 2 };
        c.g[V0][0] = st ^ list[i + 1];
        if st == list[i] {
            return true;
        }
    }
    false
}

/// `void MCManager_UpdateStates(cMCOverlayManager* m)` (0x23d7d8,
/// `cMCOverlayManager::ManagerLoadSaveUpdate`): the memory card overlay's
/// per-frame state machine step.
///
/// Unless the manager is closed (state 1): clears the offered actions and
/// polls the card driver; while the card is busy (or the screen is one of the
/// save/load/delete selection states 7/0x1d) works out which of save, load,
/// delete and cancel are offered and latches a card change; when the card is
/// idle re-reads the info of the last checked slot (or clears it when the card
/// is gone, moving to the next slot if nothing else is happening); handles the
/// next/previous slot inputs; every few updates (`mCheckDeviceTicks`) scans the
/// slots for the next present card and starts a check on it; then runs the
/// current state's handler through the switch's jump table (read from guest
/// memory, as the compiled `jr` does).
pub fn mc_manager_update_states(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 1;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S1);
        c.g[V0][0] = mcus_at(m, mcus::STATE).lw(c);
        if c.g[V0][0] == 1 {
            frame.restore(c, S0, 0x40);
            mcus_epilogue(c, frame);
            return Ok(());
        }

        // Clear the offered actions and poll the card.
        c.cycles += 18;
        for off in [
            mcus::SAVE_OK,
            mcus::LOAD_OK,
            mcus::DELETE_OK,
            mcus::CANCEL_OK,
        ] {
            mcus_at(m, off).sw(c, 0);
        }
        mcus_load_card(c, m, A1);
        mcus_card_call(c, A1, V0, V1, mcus::VT_POLL, 0x0023_d82c)?;
        mcus_load_card(c, m, A1);
        mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_d844)?;

        'card_idle: {
            // Busy, or a state other than 0x14/0x24/0x25/0x1c/0xc: the
            // action and card-change section.
            if c.g[V0][0] != 0 {
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
            } else {
                c.cycles += 5;
                let st = mcus_at(m, mcus::STATE).lw(c);
                c.g[V1][0] = st;
                c.g[A0][0] = 0;
                if !mcus_state_chain(c, st, &[0x14, 0x24, 0x25, 0x1c, 0xc]) {
                    c.cycles += 1;
                    if st != 0xc {
                        c.g[A0][0] = c.g[S0][0];
                    }
                }
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    mcus_load_card(c, m, A1);
                    break 'card_idle;
                }
                c.cycles += 1;
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
            }
            c.cycles += 3;
            let st = c.g[V1][0];
            c.g[V0][0] = 0x1d;
            'card_changed: {
                if st != 7 {
                    c.cycles += 2;
                    if st != 0x1d {
                        mcus_load_card(c, m, A1);
                        break 'card_changed;
                    }
                }
                // Selection screen (7 or 0x1d): cancel is always offered.
                c.cycles += 12;
                c.g[S3][0] = 1;
                mcus_load_card(c, m, A1);
                mcus_at(m, mcus::CANCEL_OK).sw(c, 1);
                c.g[S2][0] = sx(m.wrapping_add(mcus::SAVE_OK));
                c.g[S0][0] = 0;
                mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0023_d8c8)?;
                'save_ok: {
                    if c.g[V0][0] == 0 {
                        break 'save_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x208, 0x0023_d8ec)?;
                    if c.g[V0][0] != 0 {
                        break 'save_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x210, 0x0023_d910)?;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[S3][0];
                    }
                }
                let save_ok = c.g[S0][0];
                Addr(r32(c, S2)).sw(c, save_ok);

                c.cycles += 10;
                c.g[S0][0] = sx(m.wrapping_add(mcus::LOAD_OK));
                c.g[S2][0] = 0;
                mcus_load_card(c, m, A1);
                mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_LOAD, 0x0023_d938)?;
                'load_ok: {
                    if c.g[V0][0] == 0 {
                        break 'load_ok;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, 0x1d0, 0x0023_d958)?;
                    if c.g[V0][0] == 0 {
                        break 'load_ok;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, 0x198, 0x0023_d978)?;
                    if c.g[V0][0] != 0 {
                        break 'load_ok;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, 0x1d8, 0x0023_d998)?;
                    if c.g[V0][0] != 0 {
                        break 'load_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_d9bc)?;
                    if c.g[V0][0] == 0 {
                        break 'load_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x208, 0x0023_d9e0)?;
                    if c.g[V0][0] != 0 {
                        break 'load_ok;
                    }
                    c.cycles += 9;
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, 0x210, 0x0023_da04)?;
                    c.g[S2][0] = (c.g[V0][0] < 1) as u64;
                }
                let load_ok = c.g[S2][0];
                Addr(r32(c, S0)).sw(c, load_ok);

                // Delete: never on the load screen, else if the file exists.
                c.cycles += 4;
                c.g[V1][0] = 0x1d;
                c.g[V0][0] = mcus_at(m, mcus::STATE).lw(c);
                if c.g[V0][0] != 0x1d {
                    mcus_load_card(c, m, A2);
                    c.cycles += 7;
                    c.g[A1][0] = mcus_at(m, mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_da3c)?;
                    let exists = c.g[V0][0];
                    mcus_at(m, mcus::DELETE_OK).sw(c, exists);
                } else {
                    c.cycles += 2;
                    mcus_at(m, mcus::DELETE_OK).sw(c, 0);
                }
                c.cycles += 1;
                mcus_load_card(c, m, A1);
            }
            // Latch a card change and tell the driver.
            c.cycles += 7;
            mcus_card_call(c, A1, V1, V0, mcus::VT_CARD_CHANGED, 0x0023_da58)?;
            let changed = c.g[V0][0] != 0;
            c.g[V1][0] = 1;
            if changed {
                c.cycles += 7;
                mcus_load_card(c, m, A1);
                mcus_at(m, mcus::CARD_CHANGED).sw(c, 1);
                mcus_card_call(c, A1, V0, V1, mcus::VT_CHANGED, 0x0023_da7c)?;
            }
            c.cycles += 1;
            mcus_load_card(c, m, A1);
        }

        // Card idle: refresh (or clear) the last checked slot.
        c.cycles += 7;
        mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_da94)?;
        'slot_done: {
            if c.g[V0][0] != 0 {
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            c.cycles += 5;
            let st = mcus_at(m, mcus::STATE).lw(c);
            c.g[V1][0] = st;
            c.g[A0][0] = 0;
            let list = [
                0x14, 0x24, 0x25, 0x1c, 0xc, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36,
            ];
            if !mcus_state_chain(c, st, &list) {
                c.cycles += 2;
                if st != 0x36 {
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 0) as u64;
                    c.g[A0][0] = c.g[V0][0] ^ 1;
                }
            }
            c.cycles += 2;
            c.g[S0][0] = mcus::DEVICE_SIZE;
            if c.g[A0][0] == 0 {
                c.cycles += 1;
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            c.cycles += 6;
            c.g[A1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
            c.mult(0, V1, A1, S0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(m));
            c.g[V1][0] = mcus_at(r32(c, V0), mcus::DEVICE_PLUGGED_IN).lw(c);
            let plugged_in = c.g[V1][0] != 0;
            mcus_load_card(c, m, A2);
            if plugged_in {
                c.cycles += 7;
                mcus_card_call(c, A2, V1, V0, mcus::VT_DEVICE_OK, 0x0023_dc68)?;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::READ_DIR_INFO.call(c, 0x0023_dc9c)?;
                    c.cycles += 1;
                    mcus_load_card(c, m, A1);
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    mcus_clear_device(c, m, 0x0023_dc8c)?;
                    mcus_load_card(c, m, A1);
                }
                break 'slot_done;
            }
            c.cycles += 7;
            mcus_card_call(c, A2, V1, V0, mcus::VT_DEVICE_OK, 0x0023_db44)?;
            if c.g[V0][0] != 0 {
                // A card arrived in the slot.
                c.cycles += 4;
                c.g[A1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                if c.g[A1][0] == c.g[V0][0] {
                    c.cycles += 8;
                    c.g[V0][0] = mcus_at(m, mcus::MSG).lw(c);
                    c.g[A1][0] = 0x31;
                    let vt = r32(c, V0);
                    c.g[A0][0] = Addr(vt.wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr(vt.wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = sx(m.wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0023_db74)?;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::READ_DIR_INFO.call(c, 0x0023_db84)?;
                }
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            // The card left the slot: clear it, and if it was the selected one
            // and nothing is going on, move to the next slot.
            c.g[A0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
            c.cycles += 10;
            mcus_clear_device(c, m, 0x0023_dba4)?;
            c.g[V1][0] = mcus_at(m, mcus::SELECTED).lw(c);
            c.g[V0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            for (cb, ret) in [
                (mcus::CB_DEVICE_SELECTED, 0x0023_dbc0),
                (mcus::CB_SAVE_PRESSED, 0x0023_dbd4),
                (mcus::CB_LOAD_PRESSED, 0x0023_dbe8),
                (mcus::CB_DELETE_PRESSED, 0x0023_dbfc),
                (mcus::CB_FORMAT_PRESSED, 0x0023_dc10),
            ] {
                c.cycles += 5;
                c.g[V0][0] = mcus_at(m, cb).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, ret)?;
                if c.g[V0][0] != 0 {
                    mcus_load_card(c, m, A1);
                    break 'slot_done;
                }
            }
            for flag in [mcus::SAVE, mcus::DELETE] {
                c.cycles += 3;
                c.g[V0][0] = mcus_at(m, flag).lw(c);
                if c.g[V0][0] != 0 {
                    mcus_load_card(c, m, A1);
                    break 'slot_done;
                }
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(m, mcus::CB_LOAD_PRESSED).lw(c);
            let target = r32(c, V0);
            call_ptr(c, target, 0x0023_dc3c)?;
            if c.g[V0][0] != 0 {
                mcus_load_card(c, m, A1);
                break 'slot_done;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            mcus_callee::NEXT_SLOT.call(c, 0x0023_dc4c)?;
            mcus_load_card(c, m, A1);
        }

        // Card idle: the check countdown and the next/previous slot inputs.
        c.cycles += 7;
        mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_dcb4)?;
        'inputs: {
            'inputs_done: {
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
                    break 'inputs_done;
                }
                c.cycles += 3;
                c.g[V0][0] = mcus_at(m, mcus::CHECK_CARD).lw(c);
                let checking = c.g[V0][0] != 0;
                c.g[V0][0] = 5;
                if checking {
                    c.cycles += 2;
                    mcus_at(m, mcus::CHECK_CARD).sw(c, 0);
                    mcus_at(m, mcus::CHECK_TICKS).sw(c, 5);
                }
                c.cycles += 4;
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
                let st = c.g[V1][0];
                c.g[V0][0] = 0x1d;
                if st != 7 {
                    c.cycles += 2;
                    c.g[V0][0] = 7;
                    if st != 0x1d {
                        break 'inputs;
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = mcus_at(m, mcus::CB_NEXT_DEVICE).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_dcf4)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    c.g[S0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::NEXT_SLOT.call(c, 0x0023_dd08)?;
                    c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    if c.g[S0][0] != c.g[V0][0] {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        mcus_callee::ON_CARD_SCANNED.call(c, 0x0023_dd1c)?;
                    }
                    c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
                    break 'inputs_done;
                }
                c.g[V0][0] = mcus_at(m, mcus::CB_PREV_DEVICE).lw(c);
                c.cycles += 4;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_dd2c)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    c.g[S0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::PREV_SLOT.call(c, 0x0023_dd40)?;
                    c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
                    if c.g[S0][0] != c.g[V0][0] {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        mcus_callee::ON_CARD_SCANNED.call(c, 0x0023_dd54)?;
                    }
                }
                c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
            }
            c.cycles += 1;
            c.g[V0][0] = 7;
        }

        // The slot scan: in the screens that watch the cards, every few
        // updates find the next present card and start a check on it.
        c.cycles += 2;
        let st = c.g[V1][0];
        c.g[A0][0] = 0;
        if mcus_scan_state(c, st) {
            c.cycles += 5;
            c.g[V0][0] = mcus_at(m, mcus::CHECK_TICKS).lw(c);
            c.g[V1][0] = mcus_at(m, mcus::CHECK_CARD).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let ticks = c.g[V0][0];
            mcus_at(m, mcus::CHECK_TICKS).sw(c, ticks);
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                if (ticks as i64) <= 0 {
                    c.cycles += 7;
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_dddc)?;
                    c.g[A0][0] = (c.g[V0][0] < 1) as u64;
                }
            }
        }
        c.cycles += 2;
        let go = c.g[A0][0] != 0;
        c.g[S2][0] = 0;
        if go {
            c.cycles += 3;
            c.g[S0][0] = 0;
            c.g[S3][0] = 1;
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                mcus_callee::GET_NUM_SLOTS.call(c, 0x0023_dea4)?;
                let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                mcus_load_card(c, m, A2);
                if event_due(c) {
                    deliver_event_here(c, 0x0023_ddf8);
                }
                c.cycles += 8;
                c.g[A1][0] = c.g[S0][0];
                mcus_card_call(c, A2, V1, V0, mcus::VT_DEVICE_PRESENT, 0x0023_de10)?;
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    continue;
                }
                c.cycles += 4;
                c.g[V0][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                let after_last = (c.g[V0][0] as i64) < (c.g[S0][0] as i64);
                c.g[V0][0] = after_last as u64;
                if !after_last {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    mcus_callee::GET_NUM_SLOTS.call(c, 0x0023_de30)?;
                    c.g[V1][0] = mcus_at(m, mcus::LAST_CHECK).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        continue;
                    }
                    c.cycles += 1;
                }
                let slot = c.g[S0][0];
                mcus_at(m, mcus::CURR_CHECK).sw(c, slot);
                // Check this slot.
                c.cycles += 22;
                c.g[S2][0] = c.g[S3][0];
                mcus_at(m, mcus::LAST_CHECK).sw(c, slot);
                c.g[A1][0] = slot;
                mcus_load_card(c, m, A3);
                c.g[V0][0] = slot ^ mcus_at(m, mcus::SELECTED).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[S2][0] = 0;
                }
                mcus_card_call(c, A3, A2, V1, mcus::VT_SET_CHECK_DEVICE, 0x0023_de74)?;
                let one = c.g[S3][0];
                mcus_at(m, mcus::CHECK_CARD).sw(c, one);
                mcus_load_card(c, m, A2);
                c.g[A1][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);
                mcus_card_call(c, A2, V0, V1, mcus::VT_START_CHECK, 0x0023_de94)?;
                break;
            }
            c.cycles += 2;
            if c.g[S2][0] != 0 {
                // The checked slot is not the selected one: refresh it.
                c.cycles += 3;
                c.g[A1][0] = mcus_at(m, mcus::SELECTED).lw(c);
                c.g[A0][0] = c.g[S1][0];
                mcus_callee::REQUEST_CARD_INFO.call(c, 0x0023_dec4)?;
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
        }
        c.g[V0][0] = mcus_at(m, mcus::STATE).lw(c);

        // switch (mState): states 2..=56 through the jump table.
        c.cycles += 4;
        c.g[A0][0] = sx(r32(c, V0).wrapping_sub(2));
        c.g[V1][0] = (c.g[A0][0] < 0x37) as u64;
        c.g[V0][0] = 0x48_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, A0) << 2);
            c.g[V0][0] = sx(mcus::JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(mcus::JUMP_TABLE));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let block = r32(c, A0);
            let index = block.wrapping_sub(MCUS_CASE_BASE);
            if block == MCUS_CASE_STATE38 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                mcus_callee::STATE38.call(c, block + 8)?;
            } else if block != MCUS_CASE_NONE {
                if index % 0x10 != 0 || (index / 0x10) as usize >= MCUS_CASES.len() {
                    c.jump_out(block, 0x0023_d7d8, 0x0023_e264);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                MCUS_CASES[(index / 0x10) as usize].call(c, block + 8)?;
                frame.restore(c, S0, 0x40);
                mcus_epilogue(c, frame);
                return Ok(());
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        mcus_epilogue(c, frame);
        Ok(())
    })
}

/// `MCManager *MCManager_GetInstance(void)` (0x23c808): the manager singleton
/// at `gp-0x47c`, creating it through `MCManager_ctor` on first use.
pub fn mc_manager_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let slot = Addr((c.g[GP][0] as u32).wrapping_sub(0x47c));
        c.g[V0][0] = slot.lw(c);
        let missing = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !missing {
            c.cycles += 4;
            c.g[V0][0] = slot.lw(c);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 9;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x74c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3ed8));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        // EA_MemAllocNamed(size 0x74c); the name pointer is in a1.
        callee::MEM_ALLOC_NAMED.call(c, 0x0023_c830)?;
        c.g[A0][0] = c.g[V0][0];
        callee::MC_MANAGER_CTOR.call(c, 0x0023_c838)?;
        slot.sw(c, c.g[V0][0]);
        c.cycles += 4;
        c.g[V0][0] = slot.lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MEM_fill(&m->mDevices[a0], 0, 0xe0)` (`a0` holds the slot index).
pub(crate) fn mcus_clear_device(c: &mut Cpu, m: u32, ret: u32) -> Flow {
    c.g[A1][0] = 0;
    c.g[A2][0] = mcus::DEVICE_SIZE;
    c.mult(0, A0, A0, S0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(mcus::DEVICES));
    c.g[A0][0] = sx(m.wrapping_add(r32(c, A0)));
    call_ptr(c, mcus_callee::MEM_FILL, ret)
}

/// The states that count `mCheckDeviceTicks` down (7, 0x16, 0x14, 0x1d, 4,
/// 0x17, 0x18, 9, 0xa), as the compiled compare chain tests them (`v0` = 7
/// on entry; leaves `v0` as the chain does).
pub(crate) fn mcus_scan_state(c: &mut Cpu, st: u64) -> bool {
    if st == 7 {
        return true;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x14;
    if st == 0x16 {
        return true;
    }
    for (cur, next) in [
        (0x14, 0x1d),
        (0x1d, 4),
        (4, 0x17),
        (0x17, 0x18),
        (0x18, 9),
        (9, 0xa),
    ] {
        c.cycles += 2;
        c.g[V0][0] = next;
        if st == cur {
            return true;
        }
    }
    c.cycles += 2;
    st == 0xa
}

/// Reload s1..s3 and `ra`, pop the 0x50 frame and return.
pub(crate) fn mcus_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void MCManager_SetSlot(MCManager *self, int slot)` (0x23fb18): record the
/// selected save slot at `self+0x428`.
pub fn mc_manager_set_slot(c: &mut Cpu) {
    c.cycles += 2;
    Addr(abi::arg(c, 0)).off(0x428).sw(c, c.g[A1][0]);
    ret(c);
}

/// `void *MCManager_SetDataSize_Wrapper(MCManager *self)` (0x23cc58): call
/// through to `MCManager_SetDataSize` and return.
pub fn mc_manager_set_data_size_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MC_MANAGER_SET_DATA_SIZE.call(c, 0x0023_cc68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *MCManager_State2F(MCManager *self)` (0x240f10): reset the manager
/// through `MCManager_Reset` and return.
pub fn mc_manager_state2f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::MC_MANAGER_RESET.call(c, 0x0024_0f20)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *MCManager_ReadCurrentDirInfo(MCManager *self)` (0x241fb0): reread
/// the directory info for the current slot (`self+0x42c`) through
/// `MCManager_ReadDirInfo` and return.
pub fn mc_manager_read_current_dir_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(abi::arg(c, 0)).off(0x42c).lw(c);
        callee::MC_MANAGER_READ_DIR_INFO.call(c, 0x0024_1fc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State32(MCManager *self)` (0x23e2c0): no-op state step,
/// returns immediately.
pub fn mc_manager_state32(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MCManager_State34(MCManager *self)` (0x23e4f0): no-op state step,
/// returns immediately.
pub fn mc_manager_state34(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void *MCManager_SetDataSize(MCManager *self, int size)` (0x23caa0): store
/// `size` as the pending data size at `self+0xf0` through the backend's size
/// hook (vtable at `self+0x434`, method `+0x98/0x9c`).
pub fn mc_manager_set_data_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_cad0)?;
        Addr(c.g[S1][0] as u32).off(0xf0).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_PrevFile(MCManager *self)` (0x241678): step the file index
/// at `+0xf8` down by one, or wrap to the clamped top when it is already 0
/// and the list at `+0x100` is non-empty.
pub fn mc_manager_prev_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
        let above_zero = (c.g[V0][0] as i64) > 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if above_zero {
            c.cycles += 2;
            Addr(c.g[S0][0] as u32).off(0xf8).sw(c, c.g[V0][0]);
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x100).lw(c);
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_CLAMP_FILE_INDEX.call(c, 0x0024_16a8)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.cycles += 2;
                Addr(c.g[S0][0] as u32).off(0xf8).sw(c, c.g[V0][0]);
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_NextFile(MCManager *self)` (0x2416c0): step the file index
/// at `+0xf8` up by one toward the clamped top, wrapping to 0 when it is
/// already there and the list at `+0x100` is non-empty.
pub fn mc_manager_next_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MC_MANAGER_CLAMP_FILE_INDEX.call(c, 0x0024_16d4)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let at_top = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if at_top {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x100).lw(c);
            if c.g[V0][0] != 0 {
                Addr(c.g[S0][0] as u32).off(0xf8).sw(c, 0);
            }
        } else {
            c.cycles += 2;
            Addr(c.g[S0][0] as u32).off(0xf8).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_WriteSaveByName(MCManager *self, int a, int b, int c)`
/// (0x240fb0): flag the save pending at `gp-0x478`, run the backend's
/// by-name write hook (vtable at `self+0x434`, method `+0x120/0x124`) and
/// keep `c` at `self+0xe4`.
pub fn mc_manager_write_save_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[T0][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x478)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[T0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x120).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x124).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0fe8)?;
        Addr(c.g[S1][0] as u32).off(0xe4).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_WriteSaveBySlot(MCManager *self, int a, int b, int c)`
/// (0x241050): flag the save pending at `gp-0x478`, run the backend's
/// by-slot write hook (vtable at `self+0x434`, method `+0x130/0x134`) and
/// keep `c` at `self+0xe4`.
pub fn mc_manager_write_save_by_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[T0][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x478)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[T0][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x130).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x134).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_1088)?;
        Addr(c.g[S1][0] as u32).off(0xe4).sw(c, c.g[S0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State31(MCManager *self)` (0x23e268): when the backend
/// at `+0x434` does not report the operation done, send message 0x32 to the
/// message sink at `+0x748` (slot `+8/+0xc`).
pub fn mc_manager_state31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e290)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
            c.g[A1][0] = 0x32;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e2b0)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MCManager_ClampFileIndex(MCManager *self)` (0x2410d0): the smaller
/// of the file count at `+0xec` and the backend's count (vtable at
/// `self+0x434`, method `+0x220/0x224`); the backend is asked again when
/// its first count is not larger.
pub fn mc_manager_clamp_file_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x220).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x224).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_10f8)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xec).lw(c);
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        // True when the +0xec count is the smaller one (nothing clamped).
        let fits = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if fits {
            c.cycles += 1;
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x220).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x224).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_1120)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State0D(MCManager *self)` (0x23f308): poll the backend
/// at the file index (vtable methods `+0x218/0x21c`, then `+0x208/0x20c`
/// only when the first poll returns 0); report message 0xf only when the
/// second poll also returns 0, and 0xe otherwise.
pub fn mc_manager_state0d(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x218).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x21c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f334)?;
        let mut second_done = false;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
        } else {
            c.cycles += 9;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x208).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x20c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_f358)?;
            second_done = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
        }
        let ret = if second_done {
            c.cycles += 6;
            c.g[A1][0] = 0xf;
            0x0023_f390
        } else {
            c.cycles += 7;
            c.g[A1][0] = 0xe;
            0x0023_f374
        };
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ret)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FormatSlotFreeSpace(MCManager *self, ...)` (0x241928):
/// format one memory-card device's free space into a wide buffer. The device
/// record is at `self + a1 * 0xe0`; a missing record, or one with any of
/// three optional strings null, leaves the destination untouched.
pub fn mc_manager_format_slot_free_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x450);
        c.g[V0][0] = 0xe0;
        frame.save(c, S1, 0x430);
        c.mult(0, 17, 5, 2);
        frame.save(c, S2, 0x420);
        frame.save(c, S3, 0x410);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x440);
        frame.save_ra(c, 0x400);
        let dev = (c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32);
        c.g[V0][0] = sx(dev);
        c.g[V1][0] = Addr(dev).off(0x350).lw(c);
        let missing = c.g[V1][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        // 7 cycles for the two skipped block tails, 6 for the middle one, 5
        // for the outer one (see the generated nesting).
        let mut tail = 7u64;
        // The generated body also restores s0 just after the +0x344 load
        // and s1 inside the +0x348 arm; the tail restores stay (the loads
        // are idempotent, and s0 is reused as scratch on the full path).
        'checks: {
            if missing {
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(dev).off(0x344).lw(c);
            let missing = c.g[V1][0] == 0;
            frame.restore(c, S0, 0x440);
            if missing {
                tail = 6;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(dev).off(0x348).lw(c);
            if c.g[V1][0] == 0 {
                frame.restore(c, S1, 0x430);
                tail = 5;
                break 'checks;
            }
            c.cycles += 3;
            c.g[V1][0] = Addr(dev).off(0x34c).lw(c);
            let missing = c.g[V1][0] == 0;
            c.g[A1][0] = 0x4a0000;
            if missing {
                tail = 6;
                break 'checks;
            }
            // Two wide strings then the formatted line.
            c.cycles += 14;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c90));
            callee::USTR_FROM_ASCII_2.call(c, 0x0024_198c)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c98));
            c.g[A0][0] = c.g[S0][0];
            callee::USTR_FROM_ASCII_2.call(c, 0x0024_19a0)?;
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x354).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            callee::U_STR_SPRINTF.call(c, 0x0024_19b8)?;
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

/// `void MCManager_State1E(MCManager *self)` (0x23fff0): when the backend
/// at `+0x434` says the operation is done, return at once; otherwise poll
/// the backend (vtable `+0x1a8/0x1ac`) and either run the pending hook at
/// `+0x78` or report to the message sink at `+0x748` (slot `+8/+0xc`).
pub fn mc_manager_state1e(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0018)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0038)?;
            let mut report = false;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x78).lw(c);
                c.cycles += 4;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0024_0054)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0x1f;
                if done {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0024_0084)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x1d;
                    if !done {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                        let target = r32(c, V1);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_00a0)?;
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    report = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A1][0] = 4;
                report = true;
            }
            if report {
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0070)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State2B(MCManager *self)` (0x240cc0): when the backend
/// at `+0x434` says the operation is done, return at once; otherwise poll
/// the backend (vtable `+0x1a8/0x1ac`) and either run the pending hook at
/// `+0x68` or report to the message sink at `+0x748` (slot `+8/+0xc`).
pub fn mc_manager_state2b(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0ce8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0d08)?;
            let mut report = false;
            let mut tail = false;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x68).lw(c);
                c.cycles += 4;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0024_0d24)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0x2c;
                if done {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0024_0d40)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = 6;
                    if done {
                        tail = true;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        report = true;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    report = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A1][0] = 4;
                report = true;
            }
            if report {
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0d5c)?;
                frame.restore(c, S0, 0x10);
            }
            if tail {
                c.cycles += 8;
                c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x428).lw(c);
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x2c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0d80)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State09(MCManager *self)` (0x2409c8): when the backend
/// at `+0x434` says the operation is done, return at once; otherwise poll
/// the backend (vtable `+0x1a8/0x1ac`) and either run the pending hook at
/// `+0x70` or report to the message sink at `+0x748` (slot `+8/+0xc`).
pub fn mc_manager_state09(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_09f0)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_0a10)?;
            let mut report = false;
            let mut tail = false;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.cycles += 7;
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0a38)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0x14;
                if done {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0024_0a54)?;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = 0xd;
                    if done {
                        tail = true;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        report = true;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    report = true;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                c.g[A1][0] = 4;
                report = true;
            }
            if report {
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0a70)?;
                frame.restore(c, S0, 0x10);
            }
            if tail {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                let target = r32(c, V0);
                call_ptr(c, target, 0x0024_0a84)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 1;
                if !done {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0aa0)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State11_Save(MCManager *self)` (0x23f578): count the
/// save down, update the progress fraction and either wait on the backend
/// at `+0x434` (vtable `+0x248/0x24c`) or finish the progress bar and
/// start the save.
pub fn mc_manager_state11_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xe4));
        c.cvt_s_w(0, 0);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x108).lw(c);
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xf0));
        c.cvt_s_w(1, 1);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(c.g[S0][0] as u32).off(0x108).sw(c, c.g[V0][0]);
        c.div_s(0, 0, 1);
        c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x10c));
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_f5c4)?;
        if c.g[V0][0] != 0 {
            c.g[A0][0] = c.g[S0][0];
            c.cycles += 4;
            c.g[A1][0] = 1;
            callee::MC_MANAGER_END_PROGRESS.call(c, 0x0023_f624)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 9;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x150).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x154).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_f5e4)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.cycles += 7;
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_f674)?;
                let done = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !done {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    callee::MC_MANAGER_END_PROGRESS.call(c, 0x0023_f684)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 5;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0xe4).lw(c);
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xf0).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let under = c.g[V0][0] == 0;
                if under {
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_CARD_SET_STATE18.call(c, 0x0023_f634)?;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(8).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_f640)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A1][0] = 0x12;
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_f658)?;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    call_ptr(c, target, 0x0023_f610)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x10);
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        c.cycles += 4;
                        c.g[A1][0] = 1;
                        callee::MC_MANAGER_END_PROGRESS.call(c, 0x0023_f624)?;
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

/// `void MCManager_State2C_Delete(MCManager *self)` (0x240d90): count the
/// delete down and either wait on the backend at `+0x434` (vtable
/// `+0x248/0x24c`) or advance the delete slot and report to the message
/// sink at `+0x748` (slot `+8/+0xc`).
pub fn mc_manager_state2c_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x108).lw(c);
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        Addr(c.g[S0][0] as u32).off(0x108).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0024_0dc4)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x108).lw(c);
            if (c.g[V0][0] as i64) > 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x248).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0024_0df0)?;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 0x2d;
                if done {
                    c.cycles += 10;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x68).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x6c).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0e2c)?;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        c.cycles += 5;
                        c.g[A1][0] = 0x2e;
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                        let target = r32(c, V1);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0e9c)?;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x208).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x20c).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0024_0e54)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 8;
                            c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x50).lh(c);
                            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x54).lw(c);
                            let target = r32(c, V1);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0024_0e78)?;
                        }
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x438).lw(c);
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        Addr(c.g[S0][0] as u32).off(0x438).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0024_0e0c)?;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_State16(MCManager *self)` (0x23fd40): when the backend
/// at `+0x434` says the operation is done, return at once; otherwise,
/// when no load is pending and the backend poll (vtable `+0x1a8/0x1ac`)
/// is ready, end the operation (or report it done).
pub fn mc_manager_state16(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_fd68)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            let mut report = false;
            let mut finish = false;
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x338).lw(c);
            let mut to_hook = false;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                to_hook = true;
            } else {
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_fd94)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6c).lw(c);
                    to_hook = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A1][0] = 4;
                    report = true;
                }
            }
            if to_hook {
                c.cycles += 4;
                let target = r32(c, V0);
                call_ptr(c, target, 0x0023_fdb0)?;
                let done = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if done {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x84).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_fdd4)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x68).lw(c);
                        c.cycles += 4;
                        let target = r32(c, V0);
                        call_ptr(c, target, 0x0023_fdf4)?;
                        let done = c.g[V0][0] == 0;
                        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x234));
                        if done {
                            c.cycles += 5;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x70).lw(c);
                            let target = r32(c, V0);
                            call_ptr(c, target, 0x0023_fe44)?;
                            let done = c.g[V0][0] == 0;
                            c.g[A1][0] = 0xd;
                            if !done {
                                c.cycles += 5;
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                                let target = r32(c, V1);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, target, 0x0023_fe60)?;
                            }
                            finish = true;
                        } else {
                            c.cycles += 9;
                            c.g[A3][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0xf8).lw(c);
                            c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x70).lh(c);
                            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x74).lw(c);
                            let target = r32(c, V1);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, target, 0x0023_fe18)?;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                            c.g[A1][0] = 0x19;
                            report = true;
                        }
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MC_MANAGER_ON_OPERATION_DONE.call(c, 0x0023_fde4)?;
                        frame.restore(c, S0, 0x10);
                    }
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = 0;
                    callee::MC_MANAGER_END_OPERATION_4.call(c, 0x0023_fdc0)?;
                    frame.restore(c, S0, 0x10);
                }
            }
            if report {
                c.cycles += 6;
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_fe30)?;
                frame.restore(c, S0, 0x10);
            }
            if finish {
                c.cycles += 1;
                frame.restore(c, S0, 0x10);
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
pub fn mc_manager_begin_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x640);
        frame.save(c, S2, 0x610);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x630);
        frame.save(c, S1, 0x620);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0x600);
        Addr(c.g[S2][0] as u32).off(0xb4).sw(c, c.g[A1][0]);
        c.g[A1][0] = 1;
        callee::MC_MANAGER_SET_MODE_2.call(c, 0x0023_cb10)?;
        c.g[V1][0] = 1;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        Addr(c.g[S2][0] as u32).off(0xfc).sw(c, c.g[S0][0]);
        let mode_nonzero = c.g[V0][0] != 0;
        Addr(c.g[S2][0] as u32).off(0xbc).sw(c, c.g[V1][0]);
        if !mode_nonzero {
            c.cycles += 5;
            c.g[V0][0] = 4;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x80000;
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_cb78)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        } else {
            c.cycles += 2;
            let mode_not_one = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 6;
            if mode_not_one {
                c.cycles += 15;
                Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V1][0]);
                callee::CBE_GET_BE.call(c, 0x0023_cb88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_cb94)?;
                callee::BE_SAVE_INT_GET_SYSTEM_SAVE_SIZE.call(c, 0x0023_cb9c)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_cbb8)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
            } else {
                c.cycles += 9;
                Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                callee::CBE_GET_BE.call(c, 0x0023_cb48)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_cb54)?;
                callee::BE_SAVE_INT_GET_PROFILE_SAVE_SIZE.call(c, 0x0023_cb5c)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = c.g[V0][0];
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_cb78)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
            }
        }
        c.cycles += 24;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        callee::USTR_FROM_ASCII_2.call(c, 0x0023_cbcc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x0023_cbd8)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0x100;
        callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_cbf0)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::U_STR_SPRINTF.call(c, 0x0023_cc04)?;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore(c, S2, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_BeginLoad(MCManager *self, int mode)` (0x23c8f0): the load
/// twin of `mc_manager_begin_save` (`-1` in the request slot, no cached
/// value, and the "loading" state word in the mode-2 path).
pub fn mc_manager_begin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x640);
        frame.save(c, S2, 0x610);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x630);
        frame.save(c, S1, 0x620);
        frame.save_ra(c, 0x600);
        Addr(c.g[S2][0] as u32).off(0xb4).sw(c, c.g[A1][0]);
        c.g[A1][0] = 1;
        callee::MC_MANAGER_SET_MODE_2.call(c, 0x0023_c914)?;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        Addr(c.g[S2][0] as u32).off(0xfc).sw(c, c.g[V0][0]);
        let mode_nonzero = c.g[V1][0] != 0;
        Addr(c.g[S2][0] as u32).off(0xbc).sw(c, 0);
        if !mode_nonzero {
            c.cycles += 5;
            c.g[V0][0] = 4;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x80000;
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_c984)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let mode_not_one = c.g[V1][0] != c.g[V0][0];
            if mode_not_one {
                c.cycles += 15;
                Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                callee::CBE_GET_BE.call(c, 0x0023_c994)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_c9a0)?;
                callee::BE_SAVE_INT_GET_SYSTEM_SAVE_SIZE.call(c, 0x0023_c9a8)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_c9c4)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
            } else {
                c.cycles += 10;
                c.g[V0][0] = 6;
                Addr(c.g[S2][0] as u32).off(0xec).sw(c, c.g[V0][0]);
                callee::CBE_GET_BE.call(c, 0x0023_c954)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 5;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_c960)?;
                callee::BE_SAVE_INT_GET_PROFILE_SAVE_SIZE.call(c, 0x0023_c968)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
                c.g[A1][0] = c.g[V0][0];
                c.cycles += 7;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x98).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x9c).lw(c);
                let target = r32(c, V1);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_c984)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
            }
        }
        c.cycles += 24;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c80));
        callee::USTR_FROM_ASCII_2.call(c, 0x0023_c9d8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x0023_c9e4)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = 0x100;
        callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_c9fc)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1b4));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::U_STR_SPRINTF.call(c, 0x0023_ca10)?;
        frame.restore(c, S0, 0x630);
        frame.restore(c, S1, 0x620);
        frame.restore(c, S2, 0x610);
        frame.restore_ra(c, 0x600);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_FormatSaveDesc(MCManager *self, int mode, char *dst)`
/// (0x23cc78): switch on the word at `self+0xb4` (the `mode` argument is
/// only passed on): values 0 and 1 run the same
/// `UStr_FromAscii`/`GetSaveTypeTag`/`UStr_Sprintf` sequence, 2 takes the
/// world-device hash path ending in `UStr_Copy`, and values of 3 or more
/// (and negative values) write nothing.
pub fn mc_manager_format_save_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x350);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x330);
        frame.save(c, S2, 0x320);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x310);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x340);
        frame.save_ra(c, 0x300);
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xb4).lw(c);
        let mode_is_one = c.g[V1][0] == c.g[V0][0];
        c.g[S3][0] = c.g[A1][0];
        if mode_is_one {
            c.cycles += 16;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c88));
            c.g[A0][0] = c.g[S0][0];
            callee::USTR_FROM_ASCII_2.call(c, 0x0023_cd6c)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0];
            c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0xb4).lw(c);
            c.g[A3][0] = 0x80;
            callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_cd80)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[A2][0] = c.g[SP][0];
            callee::U_STR_SPRINTF.call(c, 0x0023_cd94)?;
            frame.restore(c, S0, 0x340);
            c.cycles += 1;
            frame.restore(c, S1, 0x330);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 2;
                c.cycles += 2;
                let mode_is_two = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                if mode_is_two {
                    c.cycles += 16;
                    c.g[A0][0] = 0x480000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ec8));
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    callee::GET_HASH_VALUE_32.call(c, 0x0023_cd34)?;
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    let target = r32(c, V1);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, target, 0x0023_cd44)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[V0][0];
                    callee::USTR_COPY.call(c, 0x0023_cd50)?;
                    frame.restore(c, S0, 0x340);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x340);
                }
                c.cycles += 1;
                frame.restore(c, S1, 0x330);
            } else {
                c.cycles += 2;
                let mode_is_zero = c.g[V1][0] == 0;
                frame.restore(c, S0, 0x340);
                if mode_is_zero {
                    c.cycles += 17;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c88));
                    c.g[A0][0] = c.g[S0][0];
                    callee::USTR_FROM_ASCII_2.call(c, 0x0023_cce8)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0xb4).lw(c);
                    c.g[A3][0] = 0x80;
                    callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_ccfc)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    c.g[A2][0] = c.g[SP][0];
                    callee::U_STR_SPRINTF.call(c, 0x0023_cd10)?;
                    frame.restore(c, S0, 0x340);
                    c.cycles += 1;
                    frame.restore(c, S1, 0x330);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S1, 0x330);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x320);
        frame.restore(c, S3, 0x310);
        frame.restore_ra(c, 0x300);
        frame.close(c);
        Ok(())
    })
}

/// `void MCManager_UpdateMode1(MCManager *self)` (0x23d660): refresh the
/// saved-slot count when the load changed, then run the finished-movie hook
/// matching the mode word at `+0x130` (states 31, 33 and 36).
pub fn mc_manager_update_mode1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x18).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x1c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_d688)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x338).lw(c);
        let mut exited = false;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x424).lw(c);
        } else {
            c.cycles += 8;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_d6ac)?;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x10);
                exited = true;
            } else {
                c.cycles += 2;
                Addr(c.g[S0][0] as u32).off(0x338).sw(c, 0);
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x424).lw(c);
            }
        }
        if !exited {
            c.cycles += 3;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_GET_NUM_SLOTS.call(c, 0x0023_d6d0)?;
                Addr(c.g[S0][0] as u32).off(0x424).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
            }
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_d6ec)?;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                callee::MC_MANAGER_SET_MODE_2.call(c, 0x0023_d700)?;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
                c.g[V0][0] = c.g[V1][0] ^ 0x31;
                let is_31 = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                if !is_31 {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] ^ 0x32;
                    let is_32 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] ^ 0x33;
                    if !is_32 {
                        c.cycles += 2;
                        let is_33 = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[V1][0] ^ 0x34;
                        if !is_33 {
                            c.cycles += 2;
                            let is_34 = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[V1][0] ^ 0x36;
                            if !is_34 {
                                c.cycles += 1;
                                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let mode_set = c.g[A0][0] == 0;
                c.g[A1][0] = 3;
                if !mode_set {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                    let target = r32(c, V1);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_d74c)?;
                }
                c.cycles += 1;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
            }
            c.cycles += 3;
            c.g[V0][0] = 0x31;
            let not_31 = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x33;
            if !not_31 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_STATE31.call(c, 0x0023_d764)?;
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 2;
                let not_33 = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x36;
                if !not_33 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_STATE33.call(c, 0x0023_d77c)?;
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        frame.restore(c, S0, 0x10);
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MC_MANAGER_STATE36.call(c, 0x0023_d794)?;
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

/// `void MCManager_BuildSaveTitle(MCManager *self, ...)` (0x23cdb0): format
/// the save title into the 0x200-byte scratch buffers. The switch is the
/// word at `self+0xb4`: mode 0 and every other value name the world
/// device through its hash path (the other values copy the name with
/// `UStr_Copy`), mode 1 takes the "new player" path; `GetSaveTypeTag` is
/// only called inside `FormatSaveDesc`, which runs first.
pub fn mc_manager_build_save_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x870);
        frame.save(c, S2, 0x840);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        frame.save(c, S4, 0x820);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x830);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0x810);
        frame.save(c, S0, 0x860);
        frame.save(c, S1, 0x850);
        frame.save_ra(c, 0x800);
        c.g[S3][0] = c.g[A2][0];
        callee::MC_MANAGER_FORMAT_SAVE_DESC.call(c, 0x0023_cde4)?;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[A1][0] = 0x480000;
        c.g[S5][0] = c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3eb0));
        callee::USTR_FROM_ASCII_2.call(c, 0x0023_cdf8)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(0xb4).lw(c);
        let mode_is_zero = c.g[V1][0] == 0;
        c.g[V0][0] = 1;
        if mode_is_zero {
            c.cycles += 19;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            c.g[A0][0] = 0x480000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ea0));
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            callee::GET_HASH_VALUE_32.call(c, 0x0023_ce34)?;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = r32(c, V1);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, target, 0x0023_ce44)?;
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            callee::U_STR_SPRINTF.call(c, 0x0023_ce54)?;
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S4][0];
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            callee::U_STR_SPRINTF.call(c, 0x0023_cea0)?;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
        } else {
            c.cycles += 2;
            let mode_is_one = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
            if mode_is_one {
                c.cycles += 14;
                callee::CBE_GET_BE.call(c, 0x0023_ce68)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 1;
                callee::CBE_GET_INTERFACE.call(c, 0x0023_ce74)?;
                c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0xfc).lw(c);
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x600));
                c.g[A0][0] = c.g[V0][0];
                callee::BE_NEW_PLAYER_INT_GET_NAME.call(c, 0x0023_ce84)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                callee::USTR_FROM_ASCII_2.call(c, 0x0023_ce90)?;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S0][0];
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                callee::U_STR_SPRINTF.call(c, 0x0023_cea0)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0x480000;
                c.cycles += 18;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3ec8));
                c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
                c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                callee::GET_HASH_VALUE_32.call(c, 0x0023_cec0)?;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                c.g[A0][0] = c.g[S0][0];
                let target = r32(c, V1);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x0023_ced0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                callee::USTR_COPY.call(c, 0x0023_cedc)?;
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[A0][0] = c.g[SP][0];
                callee::U_STR_SPRINTF.call(c, 0x0023_ceec)?;
                c.g[A2][0] = Addr(c.g[S2][0] as u32).off(0x434).lw(c);
            }
        }
        c.cycles += 18;
        c.g[A1][0] = c.g[SP][0];
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x168).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x16c).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_cf08)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x234));
        c.g[A1][0] = c.g[SP][0];
        callee::USTR_COPY.call(c, 0x0023_cf14)?;
        frame.restore(c, S0, 0x860);
        frame.restore(c, S1, 0x850);
        frame.restore(c, S2, 0x840);
        frame.restore(c, S3, 0x830);
        frame.restore(c, S4, 0x820);
        frame.restore(c, S5, 0x810);
        frame.restore_ra(c, 0x800);
        frame.close(c);
        Ok(())
    })
}

/// `int MCManager_ClassifyCardState(MCManager *self, int arg)` (0x23e8a0):
/// walk the card-state vtable cascade (busy chain, then idle chain),
/// reporting 6, 7, 5, 4 or 0x1d through the message sink at `+0x748` along
/// the way, or finishing quietly. The state words at `+0xbc` and `+0x130`
/// are only compared (against 0 and 4), never returned; the result is the
/// last probe or sink call's return value.
pub fn mc_manager_classify_card_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1ac).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_e8d0)?;
        let busy = c.g[V0][0] != 0;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
        // 0 = report to the sink at +0x748, 1 = report 0x1d, 2 = finish quietly.
        let mut seg = 0u8;
        if busy {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1b8).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1bc).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e968)?;
            let mut after = false;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                after = true;
            } else {
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1d0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1d4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_e988)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                    after = true;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x198).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x19c).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_e9a8)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                        after = true;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lh(c);
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1dc).lw(c);
                        let target = r32(c, V0);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0023_e9c8)?;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0xbc).lw(c);
                            after = true;
                        } else {
                            c.cycles += 2;
                            let arg_is_zero = c.g[S1][0] == 0;
                            c.g[A1][0] = 6;
                            if arg_is_zero {
                                seg = 2;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                            }
                        }
                    }
                }
            }
            if after {
                c.cycles += 2;
                let done = c.g[V0][0] == 0;
                c.g[A1][0] = 7;
                if done {
                    seg = 1;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                }
            }
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x188).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x18c).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e8ec)?;
            let mut report5 = false;
            let mut after = false;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                report5 = true;
            } else {
                c.cycles += 8;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x198).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x19c).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_e90c)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                    report5 = true;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x434).lw(c);
                    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1dc).lw(c);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0023_e92c)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x130).lw(c);
                        after = true;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                        report5 = true;
                    }
                }
            }
            if report5 {
                c.cycles += 2;
                c.g[A1][0] = 5;
            }
            if after {
                c.cycles += 3;
                c.g[V0][0] ^= 4;
                let is_four = c.g[V0][0] == 0;
                c.g[A1][0] = 4;
                if is_four {
                    seg = 2;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
                }
            }
        }
        if seg == 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_e9fc)?;
            frame.restore(c, S0, 0x20);
        } else if seg == 1 {
            c.cycles += 6;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x748).lw(c);
            c.g[A1][0] = 0x1d;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = r32(c, V1);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_ea1c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
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

/// `void MCManager_NextSlot(MCManager *self)` (0x241400): step the chapter
/// at `+0x428` forward to the next chapter that has content (wrapping past
/// the end back to the current one), then record it and refresh the slot.
/// The back edge re-enters at 0x241430, so that address is an entry too.
pub fn mc_manager_next_slot(c: &mut Cpu) {
    // The frame size must equal the size `Frame::open` uses on the entry arm.
    let frame = Frame { size: 0x30 };
    // Entry, or re-entry at the loop head after a back-edge unwind (the
    // loop state is in the registers and the saved frame).
    let mut first = false;
    match c.pc {
        0x0024_1400 => {
            c.cycles += 9;
            Frame::open(c, 0x30);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame.save(c, S1, 0x10);
            frame.save(c, S0, 0x20);
            c.g[S1][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
            let no_slot = c.g[V1][0] == c.g[V0][0];
            c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
            if no_slot {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[V1][0];
            first = true;
        }
        0x0024_1430 => {}
        pc => {
            c.jump_out(pc, 0x0024_1400, 0x0024_1540);
            return;
        }
    }
    // Scan forward from the candidate in s0 until a chapter has content.
    'scan: loop {
        if first {
            first = false;
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            let mut probe = false;
            if c.g[S0][0] == c.g[V0][0] {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                probe = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                let list_done = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                if !list_done {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    probe = true;
                }
            }
            if probe {
                c.cycles += 2;
                let wrapped = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 1;
                if wrapped {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                    break 'scan;
                }
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                    break 'scan;
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
            c.cycles += 1;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        }
        c.cycles += 9;
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0024_149c).is_err() {
            return;
        }
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
            break 'scan;
        }
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
        if back_edge_unwinds(c, 0x0024_1430) {
            return;
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
    } else {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if c.g[S0][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
        } else {
            c.cycles += 9;
            c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if call_ptr(c, target, 0x0024_14dc).is_err() {
                return;
            }
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
            } else {
                c.cycles += 10;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_1504).is_err() {
                    return;
                }
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 0;
                }
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] == c.g[S0][0] {
        Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
    } else {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        if callee::MC_MANAGER_SET_SLOT.call(c, 0x0024_1520).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S1][0];
        if callee::MC_MANAGER_REFRESH_SLOT
            .call(c, 0x0024_1528)
            .is_err()
        {
            return;
        }
        Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void MCManager_PrevSlot(MCManager *self)` (0x241540): step the chapter
/// at `+0x428` back to the previous chapter that has content (wrapping
/// past the start back to the current one), then record it and refresh
/// the slot. The back edge re-enters at 0x241570, so that address is an
/// entry too.
pub fn mc_manager_prev_slot(c: &mut Cpu) {
    // The frame size must equal the size `Frame::open` uses on the entry arm.
    let frame = Frame { size: 0x40 };
    // Entry, or re-entry at the loop head after a back-edge unwind (the
    // loop state is in the registers and the saved frame).
    let mut first = false;
    match c.pc {
        0x0024_1540 => {
            c.cycles += 10;
            Frame::open(c, 0x40);
            c.g[V1][0] = 0xffff_ffff_ffff_ffff;
            frame.save(c, S1, 0x20);
            frame.save(c, S0, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S2, 0x10);
            frame.save_ra(c, 0);
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
            let no_slot = c.g[V0][0] == c.g[V1][0];
            c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
            if no_slot {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            c.cycles += 2;
            c.g[S2][0] = 1;
            first = true;
        }
        0x0024_1570 => {}
        pc => {
            c.jump_out(pc, 0x0024_1540, 0x0024_1674);
            return;
        }
    }
    // Scan backward from the candidate in s0 until a chapter has content.
    'scan: loop {
        if first {
            first = false;
        } else {
            c.cycles += 2;
            let mut probe = false;
            if c.g[S0][0] == 0 {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                probe = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                    probe = true;
                }
            }
            if probe {
                c.cycles += 2;
                let wrapped = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
                if wrapped {
                    break 'scan;
                }
                c.cycles += 2;
                if c.g[V0][0] != c.g[S2][0] {
                    break 'scan;
                }
            }
        }
        c.cycles += 2;
        if (c.g[S0][0] as i64) <= 0 {
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x424).lw(c);
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                c.cycles += 1;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
            }
        } else {
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
            c.cycles += 1;
            c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S0][0];
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
        let target = r32(c, V0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if call_ptr(c, target, 0x0024_15d4).is_err() {
            return;
        }
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x100).lw(c);
            break 'scan;
        }
        if back_edge_unwinds(c, 0x0024_1570) {
            return;
        }
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
    } else {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
        } else {
            c.cycles += 9;
            c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
            c.g[A1][0] = 0;
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if call_ptr(c, target, 0x0024_160c).is_err() {
                return;
            }
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
            } else {
                c.cycles += 10;
                c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x434).lw(c);
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x1a0).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a4).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                if call_ptr(c, target, 0x0024_1634).is_err() {
                    return;
                }
                if c.g[V0][0] == 0 {
                    c.g[S0][0] = 0;
                }
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x428).lw(c);
            }
        }
    }
    c.cycles += 2;
    if c.g[V0][0] == c.g[S0][0] {
        Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
    } else {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        if callee::MC_MANAGER_SET_SLOT.call(c, 0x0024_1650).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S1][0];
        if callee::MC_MANAGER_REFRESH_SLOT
            .call(c, 0x0024_1658)
            .is_err()
        {
            return;
        }
        Addr(c.g[S1][0] as u32).off(0xe0).sw(c, 0);
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

pub(crate) mod mc_v01 {

    /// The function's first instruction and its end (for `jump_out`).
    pub const ENTRY: u32 = 0x0023_cf38;

    pub const END: u32 = 0x0023_d570;

    /// The switch's jump table (states 1..=0x38) in `.rodata`.
    pub const JUMP_TABLE: u32 = 0x0047_c1a0;

    /// The word at `gp - 0x478` that most states set to 1 or clear.
    pub const GP_FLAG: u32 = 0x478;

    /// The jump table's length: states 1..=0x38, indexed by `state - 1`.
    pub const JUMP_TABLE_LEN: u64 = 0x38;

    /// The UStr buffer state 0xf prints the save's file name into.
    pub const SAVE_NAME: u32 = 0x134;

    /// `"%s-%s%04d"` and `"data/icon"` in `.rodata`.
    pub const NAME_FORMAT: u32 = 0x0047_c178;

    pub const ICON_PATH: u32 = 0x0047_c188;
}

/// The paths out of `MCManager_v01`'s switch, in the order the compiled
/// tails run them.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum McV01Tail {
    /// 0x23d548: clear the `gp - 0x478` word, then record the state.
    Clear,
    /// 0x23d54c: record the state at `mState` (+0x130).
    Record,
    /// 0x23d550: the state is already recorded (by the arm, often in a
    /// delay slot).
    AlreadyRecorded,
    /// The state is left as it was (s0 reloaded in the delay slot).
    Kept,
}

/// The word at `gp - 0x478`.
#[inline]
pub(crate) fn mcv01_flag(c: &Cpu) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_sub(mc_v01::GP_FLAG))
}

/// `mState = state` (the state is in s4).
#[inline]
pub(crate) fn mcv01_record(c: &mut Cpu) {
    let state = c.g[S4][0];
    mcus_at(r32(c, S3), mcus::STATE).sw(c, state);
}

/// `lui at, 0x3f80; mtc1 at, $f`: 1.0f in FPU register `f`.
#[inline]
pub(crate) fn mcv01_one(c: &mut Cpu, f: usize) {
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, f);
}

/// `lwc1 $f, -off(gp)`.
#[inline]
pub(crate) fn mcv01_gp_float(c: &mut Cpu, f: usize, off: u32) {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lwc1(f, a);
}

/// 0x23d404: f0 and f1 to +0x110 and +0x114.
pub(crate) fn mcv01_ramp(c: &mut Cpu) {
    c.cycles += 3;
    let a = mcus_at(r32(c, S3), mcus::RAMP).0;
    c.swc1(0, a);
    let a = mcus_at(r32(c, S3), mcus::RAMP_STEP).0;
    c.swc1(1, a);
}

/// 0x23d394: the card driver call shared by several states (the driver in a2,
/// its vtable in v0): `a0 = a2 + vt[off].delta`, call `vt[off].fn`, then record
/// the state.
pub(crate) fn mcv01_card_call(c: &mut Cpu, off: u32) -> Flow {
    c.cycles += 4;
    let vt = r32(c, V0);
    c.g[A0][0] = Addr(vt.wrapping_add(off)).lh(c);
    c.g[V1][0] = Addr(vt.wrapping_add(off + 4)).lw(c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x0023_d39c)?;
    mcv01_record(c);
    Ok(())
}

/// 0x23d35c: the state timer = v0, call the callback in v1, record the state.
pub(crate) fn mcv01_wait_call(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let target = r32(c, V1);
    let wait = c.g[V0][0];
    mcus_at(r32(c, S3), mc::WAIT).sw(c, wait);
    call_ptr(c, target, 0x0023_d364)?;
    mcv01_record(c);
    Ok(())
}

/// 0x23d3f0: call the callback in v0, then 1.0f and the `gp - 0x5138`
/// constant to +0x110/+0x114.
pub(crate) fn mcv01_call_ramp(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let target = r32(c, V0);
    call_ptr(c, target, 0x0023_d3f8)?;
    mcv01_one(c, 0);
    mcv01_gp_float(c, 1, 0x5138);
    mcv01_ramp(c);
    Ok(())
}

/// 0x23d540: call the callback in v0 (the `gp - 0x478` word is cleared after).
pub(crate) fn mcv01_call_clear(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let target = r32(c, V0);
    call_ptr(c, target, 0x0023_d548)
}

/// Call the callback word at `self + off` returning to `ret`, then record the
/// state.
pub(crate) fn mcv01_callback(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = mcus_at(r32(c, S3), off).lw(c);
    let target = r32(c, V0);
    call_ptr(c, target, ret)?;
    mcv01_record(c);
    Ok(())
}

/// State 0xf's format string: `s2 = self + 0x134`, `s1 = sp + 0x400`,
/// `UStr_FromAscii_2(s1, "%s-%s%04d")` returning to `ret`, then
/// `s0 = sp + 0x200`.
pub(crate) fn mcv01_name_format(c: &mut Cpu, ret: u32) -> Flow {
    c.g[S2][0] = sx(r32(c, S3).wrapping_add(mc_v01::SAVE_NAME));
    c.g[S1][0] = sx(sp(c).wrapping_add(0x400));
    c.g[A1][0] = sx(mc_v01::NAME_FORMAT);
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, callee::USTR_FROM_ASCII_2.addr, ret)?;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x200));
    Ok(())
}

/// The shared tails (0x23d548, 0x23d54c, 0x23d550) and the epilogue: reload
/// s0..s4 and `ra`, pop the 0x660 frame and return.
pub(crate) fn mcv01_finish(c: &mut Cpu, frame: Frame, tail: McV01Tail) {
    let mut tail = tail;
    if tail == McV01Tail::Clear {
        c.cycles += 1;
        mcv01_flag(c).sw(c, 0);
        tail = McV01Tail::Record;
    }
    if tail == McV01Tail::Record {
        c.cycles += 1;
        mcv01_record(c);
        tail = McV01Tail::AlreadyRecorded;
    }
    if tail == McV01Tail::AlreadyRecorded {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x650);
    c.cycles += 7;
    frame.restore(c, S1, 0x640);
    frame.restore(c, S2, 0x630);
    frame.restore(c, S3, 0x620);
    frame.restore(c, S4, 0x610);
    frame.restore_ra(c, 0x600);
    frame.close(c);
}

/// `void MCManager_v01(MCManager *self, int state)` (0x23cf38): the manager's
/// virtual method 1 (slot +8 of the vtable at `self+0x748`), which enters
/// `state`: it does that state's entry work, then records the state in
/// `mState` (+0x130).
///
/// The entry work is a switch over states 1..=0x38 through the jump table at
/// 0x47c1a0 (read from guest memory, as the compiled `jr` does): resetting the
/// pending-operation words and the update step (`MCManager_SetMode`, state 1),
/// starting the state timer (+0x108) and the floats at +0x110/+0x114, setting
/// or clearing the word at `gp - 0x478`, calling the screen callbacks stored
/// in the manager's first words, and calling the card driver at +0x434: the
/// busy poll, card info, the directory list, and the slots the format (0xb,
/// 0x28), delete (0xe, 0x19), save (0xf) and load (0x1f) states start with.
/// State 0xf first prints the save's file name into +0x134 as `"%s-%s%04d"`
/// (product code, save type tag, number) and passes `"data/icon"` to driver
/// slot 0x138, then the name to slot 0x118. States 0x32 and 0x34, and state 0x33
/// while the card is busy (it posts state 0x35 instead), leave `mState` as it
/// was. Other states, and values outside 1..=0x38, only record the state.
pub fn mc_manager_v01(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x660 };
        let tail = loop {
            c.cycles += 12;
            Frame::open(c, 0x660);
            frame.save(c, S4, 0x610);
            c.g[S4][0] = c.g[A1][0];
            frame.save(c, S3, 0x620);
            c.g[V1][0] = sx(r32(c, S4).wrapping_sub(1));
            frame.save(c, S0, 0x650);
            frame.save(c, S1, 0x640);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S2, 0x630);
            c.g[V0][0] = (c.g[V1][0] < mc_v01::JUMP_TABLE_LEN) as u64;
            let in_table = c.g[V0][0] != 0;
            frame.save_ra(c, 0x600);
            if !in_table {
                break McV01Tail::Record;
            }
            c.cycles += 7;
            c.g[V0][0] = 0x48_0000;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(mc_v01::JUMP_TABLE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            let block = r32(c, A0);
            let tail = match block {
                mc_v01::ENTRY => {
                    // A table word naming the entry restarts the function
                    // (prologue and all), after any due event.
                    if event_due(c) {
                        deliver_event_here(c, mc_v01::ENTRY);
                    }
                    continue;
                }
                0x0023_cf84 => {
                    // State 1: update step 1, clear the pending operations.
                    c.cycles += 10;
                    mcv01_flag(c).sw(c, 0);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = 1;
                    callee::MC_MANAGER_SET_MODE.call(c, 0x0023_cf94)?;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    mcus_at(r32(c, S3), 0xc4).sw(c, 0);
                    let none = c.g[V0][0];
                    mcus_at(r32(c, S3), 0xfc).sw(c, none);
                    mcus_at(r32(c, S3), mcus::SAVE).sw(c, 0);
                    mcus_at(r32(c, S3), mcus::DELETE).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_cfac => {
                    // State 0x31: ask for the selected slot's card info.
                    c.cycles += 6;
                    mcus_at(r32(c, S3), mcus::CARD_CHANGED).sw(c, 0);
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = mcus_at(r32(c, S3), mcus::SELECTED).lw(c);
                    callee::MC_MANAGER_REQUEST_CARD_INFO.call(c, 0x0023_cfbc)?;
                    c.g[V0][0] = 1;
                    c.cycles += 2;
                    let one = c.g[V0][0];
                    mcus_at(r32(c, S3), mcus::CHECK_CARD).sw(c, one);
                    McV01Tail::Record
                }
                0x0023_cfc4 => {
                    // State 0x32: reread the selected slot's directory; clear
                    // its plugged-in flag when driver slot 0x1a8 says no; run
                    // the callback at +0x90. mState is left alone.
                    c.cycles += 12;
                    c.g[V0][0] = mcus_at(r32(c, S3), mcus::SELECTED).lw(c);
                    c.g[A0][0] = c.g[S3][0];
                    let slot = c.g[V0][0];
                    mcus_at(r32(c, S3), mcus::CURR_CHECK).sw(c, slot);
                    callee::MC_MANAGER_READ_CURRENT_DIR_INFO.call(c, 0x0023_cfd4)?;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0023_cfec)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 6;
                        c.g[V0][0] = mcus_at(r32(c, S3), mcus::CURR_CHECK).lw(c);
                        c.g[V1][0] = mcus::DEVICE_SIZE;
                        c.mult(0, A0, V0, V1);
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                        Addr(r32(c, V0).wrapping_add(mcus::DEVICE_PLUGGED_IN)).sw(c, 0);
                    }
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x90).lw(c);
                    c.cycles += 4;
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d014)?;
                    McV01Tail::Kept
                }
                0x0023_d01c => {
                    // State 0x33: while the card is busy post state 0x35
                    // (mState left alone), else ask for the directory list.
                    c.cycles += 9;
                    mcus_at(r32(c, S3), mcus::CARD_CHANGED).sw(c, 0);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_d038)?;
                    let idle = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x35;
                    if !idle {
                        c.cycles += 7;
                        c.g[V0][0] = mcus_at(r32(c, S3), mcus::MSG).lw(c);
                        let vt = r32(c, V0);
                        c.g[A0][0] = Addr(vt.wrapping_add(8)).lh(c);
                        c.g[V1][0] = Addr(vt.wrapping_add(0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0023_d054)?;
                        McV01Tail::Kept
                    } else {
                        c.cycles += 6;
                        let m = r32(c, S3);
                        mcus_load_card(c, m, A2);
                        c.g[A1][0] = sx(r32(c, S3).wrapping_add(0x1b4));
                        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                        mcv01_card_call(c, 0x38)?;
                        McV01Tail::AlreadyRecorded
                    }
                }
                0x0023_d074 => {
                    // State 0x34: the callback at +0x94; mState is left alone.
                    c.cycles += 5;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x94).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d080)?;
                    McV01Tail::Kept
                }
                0x0023_d088 => {
                    // State 2: wait 90 updates.
                    c.cycles += 6;
                    mcv01_one(c, 0);
                    c.g[V0][0] = 0x5a;
                    mcv01_gp_float(c, 1, 0x5148);
                    let wait = c.g[V0][0];
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, wait);
                    mcv01_ramp(c);
                    McV01Tail::Record
                }
                0x0023_d0a0 => {
                    // State 3: unless the card is busy or a check is already
                    // pending, start a check of the selected slot.
                    c.cycles += 8;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_d0b8)?;
                    if c.g[V0][0] != 0 {
                        mcv01_record(c);
                        McV01Tail::AlreadyRecorded
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = mcus_at(r32(c, S3), mcus::CHECK_CARD).lw(c);
                        if c.g[V0][0] != 0 {
                            mcv01_record(c);
                            McV01Tail::AlreadyRecorded
                        } else {
                            c.cycles += 8;
                            let m = r32(c, S3);
                            mcus_load_card(c, m, A2);
                            c.g[A1][0] = mcus_at(r32(c, S3), mcus::SELECTED).lw(c);
                            mcus_card_call(c, A2, V0, V1, mcus::VT_SET_CHECK_DEVICE, 0x0023_d0e8)?;
                            c.g[V0][0] = 1;
                            c.cycles += 2;
                            let one = c.g[V0][0];
                            mcus_at(r32(c, S3), mcus::CHECK_CARD).sw(c, one);
                            McV01Tail::Record
                        }
                    }
                }
                0x0023_d0f4 => {
                    // State 0x18: delete pending; the callback at +0x54.
                    c.cycles += 11;
                    mcv01_one(c, 1);
                    c.g[V0][0] = 1;
                    mcv01_gp_float(c, 0, 0x5144);
                    c.g[V1][0] = mcus_at(r32(c, S3), 0x54).lw(c);
                    let one = c.g[V0][0];
                    mcus_at(r32(c, S3), mcus::DELETE).sw(c, one);
                    let a = mcus_at(r32(c, S3), mcus::RAMP).0;
                    c.swc1(1, a);
                    let a = mcus_at(r32(c, S3), mcus::RAMP_STEP).0;
                    c.swc1(0, a);
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0023_d118)?;
                    mcv01_record(c);
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d120 => {
                    // State 6: list the card's directory into +0x1b4.
                    c.cycles += 7;
                    mcus_at(r32(c, S3), mcus::CARD_CHANGED).sw(c, 0);
                    c.g[A1][0] = sx(r32(c, S3).wrapping_add(0x1b4));
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    mcv01_card_call(c, 0x38)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d13c => {
                    // State 0xb: start the format (driver slot 0x40), wait 120.
                    c.cycles += 16;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    c.g[V0][0] = 1;
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    mcus_card_call(c, A1, V1, V0, 0x40, 0x0023_d15c)?;
                    mcv01_one(c, 1);
                    c.g[V0][0] = 0x78;
                    mcv01_gp_float(c, 0, 0x5140);
                    let a = mcus_at(r32(c, S3), mcus::RAMP).0;
                    c.swc1(1, a);
                    let a = mcus_at(r32(c, S3), mcus::RAMP_STEP).0;
                    c.swc1(0, a);
                    let wait = c.g[V0][0];
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, wait);
                    McV01Tail::Record
                }
                0x0023_d17c => {
                    // State 0xd: build the save title for the file cursor,
                    // then (as state 0x23) clear the timer.
                    c.cycles += 7;
                    c.g[V0][0] = 1;
                    c.g[A1][0] = mcus_at(r32(c, S3), mcus::FILE_CURSOR).lw(c);
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    c.g[A0][0] = c.g[S3][0];
                    callee::MC_MANAGER_BUILD_SAVE_TITLE.call(c, 0x0023_d190)?;
                    mcus_at(r32(c, S3), 0x10c).sw(c, 0);
                    mcus_at(r32(c, S3), 0xe4).sw(c, 0);
                    c.cycles += 2;
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d198 => {
                    // State 0x23: clear the timer.
                    c.cycles += 2;
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d1a0 => {
                    // State 0xe: delete pending; driver slot 0x50 with the
                    // file cursor.
                    c.cycles += 7;
                    c.g[V1][0] = 1;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    let one = c.g[V1][0];
                    mcus_at(r32(c, S3), mcus::DELETE).sw(c, one);
                    c.g[A1][0] = mcus_at(r32(c, S3), mcus::FILE_CURSOR).lw(c);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    mcv01_flag(c).sw(c, one);
                    c.cycles += 2;
                    mcv01_card_call(c, 0x50)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d1bc => {
                    // State 0xf: print the save's file name into +0x134
                    // (`"%s-%s%04d"`: product code, the save type tag for
                    // the save mode, and the file cursor + 1; in save mode 2
                    // the number is 1 and the two strings trade scratch
                    // buffers), then driver slot 0x138 with `"data/icon"`
                    // and slot 0x118 with the name.
                    c.cycles += 3;
                    c.g[V1][0] = mcus_at(r32(c, S3), mcus::SAVE_MODE).lw(c);
                    let mode = c.g[V1][0];
                    c.g[V0][0] = 2;
                    if mode == 0 {
                        c.cycles += 13;
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A3][0] = 0x100;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = c.g[S3][0];
                        callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_d1dc)?;
                        mcv01_name_format(c, 0x0023_d1f4)?;
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = c.g[SP][0];
                        if mode != c.g[V0][0] {
                            c.cycles += 11;
                            c.g[A3][0] = 0x100;
                            c.g[A1][0] = 1;
                            c.g[A0][0] = c.g[S3][0];
                            callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_d268)?;
                            mcv01_name_format(c, 0x0023_d280)?;
                        } else {
                            c.cycles += 21;
                            c.g[S1][0] = sx(sp(c).wrapping_add(0x400));
                            c.g[A1][0] = sx(mc_v01::NAME_FORMAT);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, callee::USTR_FROM_ASCII_2.addr, 0x0023_d218)?;
                            c.g[S2][0] = sx(r32(c, S3).wrapping_add(mc_v01::SAVE_NAME));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = 0x200;
                            callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x0023_d228)?;
                            c.g[S0][0] = sx(sp(c).wrapping_add(0x200));
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = 2;
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = 0x200;
                            callee::MC_MANAGER_GET_SAVE_TYPE_TAG.call(c, 0x0023_d240)?;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[SP][0];
                            c.g[A0][0] = c.g[S2][0];
                            c.g[T0][0] = 1;
                        }
                    }
                    // Modes other than 2 continue at 0x23d284; mode 2 jumps
                    // past it.
                    if mode != 2 {
                        // 0x23d284: the product code, numbered cursor + 1.
                        c.cycles += 9;
                        c.g[A1][0] = 0x100;
                        c.g[A0][0] = c.g[S0][0];
                        callee::MC_MANAGER_GET_PRODUCT_CODE.call(c, 0x0023_d290)?;
                        c.g[T0][0] = mcus_at(r32(c, S3), mcus::FILE_CURSOR).lw(c);
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A3][0] = c.g[SP][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    }
                    // 0x23d2a8: UStr_Sprintf(self+0x134, fmt, a2, a3, t0).
                    c.cycles += 20;
                    call_ptr(c, callee::U_STR_SPRINTF.addr, 0x0023_d2b0)?;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A3);
                    c.g[A1][0] = sx(mc_v01::ICON_PATH);
                    c.g[A2][0] = 0;
                    mcus_card_call(c, A3, V0, V1, 0x138, 0x0023_d2d4)?;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V1][0] = 1;
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    let one = c.g[V1][0];
                    mcv01_flag(c).sw(c, one);
                    mcus_at(r32(c, S3), 0x118).sw(c, one);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    mcv01_card_call(c, 0x118)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d2f8 => {
                    // State 0x11: wait 60 updates; the callback at +0x0.
                    c.cycles += 3;
                    c.g[V0][0] = 0x3c;
                    c.g[V1][0] = mcus_at(r32(c, S3), 0).lw(c);
                    mcv01_wait_call(c)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d304 => {
                    // State 0x1f: driver slot 0x70 with the file cursor and
                    // +0x234; word 0x4d at +0xe8; the callback at +0x14.
                    c.cycles += 20;
                    mcv01_one(c, 1);
                    c.g[A2][0] = sx(r32(c, S3).wrapping_add(0x234));
                    mcv01_gp_float(c, 0, 0x513c);
                    let a = mcus_at(r32(c, S3), mcus::RAMP).0;
                    c.swc1(1, a);
                    let a = mcus_at(r32(c, S3), mcus::RAMP_STEP).0;
                    c.swc1(0, a);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A3);
                    c.g[A1][0] = mcus_at(r32(c, S3), mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A3, V0, V1, MEMCARD_CARD_VT_SLOT_NAME, 0x0023_d338)?;
                    c.g[A0][0] = 0x4d;
                    mcus_at(r32(c, S3), 0x10c).sw(c, 0);
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x14).lw(c);
                    let target = r32(c, V0);
                    let word = c.g[A0][0];
                    mcus_at(r32(c, S3), 0xe8).sw(c, word);
                    call_ptr(c, target, 0x0023_d34c)?;
                    mcv01_record(c);
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d354 => {
                    // State 0x20: wait 60 updates; the callback at +0x18.
                    c.cycles += 2;
                    c.g[V0][0] = 0x3c;
                    c.g[V1][0] = mcus_at(r32(c, S3), 0x18).lw(c);
                    mcv01_wait_call(c)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d36c => {
                    // State 0x19: delete pending, wait 60 updates; driver
                    // slot 0x50 with the file cursor.
                    c.cycles += 8;
                    c.g[V0][0] = 1;
                    c.g[V1][0] = 0x3c;
                    let one = c.g[V0][0];
                    mcus_at(r32(c, S3), mcus::DELETE).sw(c, one);
                    mcv01_flag(c).sw(c, one);
                    let wait = c.g[V1][0];
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, wait);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A2);
                    c.g[A1][0] = mcus_at(r32(c, S3), mcus::FILE_CURSOR).lw(c);
                    c.g[V0][0] = Addr(r32(c, A2)).lw(c);
                    c.cycles += 2;
                    mcv01_card_call(c, 0x50)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d3a4 => {
                    // State 0x1b: no delete pending.
                    c.cycles += 2;
                    mcus_at(r32(c, S3), mcus::DELETE).sw(c, 0);
                    McV01Tail::Clear
                }
                0x0023_d3ac => {
                    // State 0x1c: no delete pending; the callback at +0x34.
                    c.cycles += 6;
                    mcus_at(r32(c, S3), mcus::DELETE).sw(c, 0);
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x34).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d3bc)?;
                    mcv01_flag(c).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d3c4 => {
                    // State 0x13: word 1 at +0xdc, -1 at +0xfc.
                    c.cycles += 6;
                    mcv01_flag(c).sw(c, 0);
                    c.g[V0][0] = 1;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    let one = c.g[V0][0];
                    mcus_at(r32(c, S3), 0xdc).sw(c, one);
                    let none = c.g[V1][0];
                    mcus_at(r32(c, S3), 0xfc).sw(c, none);
                    McV01Tail::Record
                }
                0x0023_d3dc => {
                    // State 0xa: the callback at +0x48, then the floats.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x48).lw(c);
                    mcv01_call_ramp(c)?;
                    McV01Tail::Record
                }
                0x0023_d3e4 => {
                    // State 0xc: the callback at +0x4c.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x4c).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d3ec => {
                    // State 9: the callback at +0x44, then the floats.
                    c.cycles += 1;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x44).lw(c);
                    mcv01_call_ramp(c)?;
                    McV01Tail::Record
                }
                0x0023_d410 => {
                    // State 0x14: the callback at +0x10.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x10).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d418 => {
                    // State 0x15: the callback at +0x60.
                    mcv01_callback(c, 0x60, 0x0023_d424)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d42c => {
                    // State 0x17: the callback at +0x5c.
                    mcv01_callback(c, 0x5c, 0x0023_d438)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d440 => {
                    // State 4: the callback at +0x3c.
                    mcv01_callback(c, 0x3c, 0x0023_d44c)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d454 => {
                    // State 0x24: the callback at +0x24, clear the timer.
                    c.cycles += 5;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x24).lw(c);
                    let target = r32(c, V0);
                    call_ptr(c, target, 0x0023_d460)?;
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d468 => {
                    // State 0x25: the callback at +0x40.
                    mcv01_callback(c, 0x40, 0x0023_d474)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d47c => {
                    // State 0x16: the callback at +0x8c.
                    mcv01_callback(c, 0x8c, 0x0023_d488)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d490 => {
                    // State 0x27: the callback at +0x48.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x48).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d498 => {
                    // State 0x28: wait 120 updates; driver slot 0x40.
                    c.cycles += 17;
                    c.g[V0][0] = 1;
                    mcv01_one(c, 0);
                    mcv01_gp_float(c, 1, 0x5134);
                    c.g[V1][0] = 0x78;
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    let a = mcus_at(r32(c, S3), mcus::RAMP).0;
                    c.swc1(0, a);
                    let a = mcus_at(r32(c, S3), mcus::RAMP_STEP).0;
                    c.swc1(1, a);
                    let wait = c.g[V1][0];
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, wait);
                    let m = r32(c, S3);
                    mcus_load_card(c, m, A1);
                    mcus_card_call(c, A1, V0, V1, 0x40, 0x0023_d4d4)?;
                    mcv01_record(c);
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d4dc => {
                    // State 0x29: the callback at +0x4c.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x4c).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d4e4 => {
                    // State 0x2a: the callback at +0x50.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x50).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d4ec => {
                    // State 0x2b: the callback at +0x54.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0x54).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d4f4 => {
                    // State 0x2c: wait 60 updates, clear the word at +0x438.
                    c.cycles += 11;
                    c.g[V0][0] = 1;
                    mcv01_one(c, 0);
                    mcv01_gp_float(c, 1, 0x5130);
                    c.g[V1][0] = 0x3c;
                    let one = c.g[V0][0];
                    mcv01_flag(c).sw(c, one);
                    let a = mcus_at(r32(c, S3), mcus::RAMP).0;
                    c.swc1(0, a);
                    let a = mcus_at(r32(c, S3), mcus::RAMP_STEP).0;
                    c.swc1(1, a);
                    let wait = c.g[V1][0];
                    mcus_at(r32(c, S3), mc::WAIT).sw(c, wait);
                    mcus_at(r32(c, S3), 0x438).sw(c, 0);
                    McV01Tail::Record
                }
                0x0023_d520 => {
                    // States 5 and 0x30: the callback at +0x58.
                    mcv01_callback(c, 0x58, 0x0023_d52c)?;
                    McV01Tail::AlreadyRecorded
                }
                0x0023_d534 => {
                    // State 0x37: the callback at +0xa0.
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0xa0).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                0x0023_d53c => {
                    // State 0x38: the callback at +0xa8.
                    c.cycles += 1;
                    c.g[V0][0] = mcus_at(r32(c, S3), 0xa8).lw(c);
                    mcv01_call_clear(c)?;
                    McV01Tail::Clear
                }
                // States 0x2d, 0x2e.
                0x0023_d548 => McV01Tail::Clear,
                // States without entry work.
                0x0023_d54c => McV01Tail::Record,
                _ => {
                    c.jump_out(block, mc_v01::ENTRY, mc_v01::END);
                    return Ok(());
                }
            };
            break tail;
        };
        mcv01_finish(c, frame, tail);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Card status texts.

/// Text ids (ASCII, hashed and looked up in the application's text database).
pub(crate) const MC_TEXT_NOT_DETECTED: u32 = 0x0047_c360;

pub(crate) const MC_TEXT_NOT_FORMATTED: u32 = 0x0047_c378;

pub(crate) const MC_TEXT_MEM_DEVICE: u32 = 0x0047_c390;

pub(crate) const MC_TEXT_CHECKING_CARD: u32 = 0x0047_c3b8;

pub(crate) const MC_TEXT_FORMATTING: u32 = 0x0047_c3d0;

pub(crate) const MC_TEXT_SAVING: u32 = 0x0047_c3e8;

pub(crate) const MC_TEXT_LOADING: u32 = 0x0047_c408;

pub(crate) const MC_TEXT_DELETING: u32 = 0x0047_c420;

/// An empty ASCII string (the status text starts out empty).
pub(crate) const MC_TEXT_EMPTY: u32 = 0x004a_29e8;

/// `MCManager_GetStatusText`'s switch: `MC_STATUS_TEXT_TABLE_LEN` code
/// addresses, one per state.
pub(crate) const MC_STATUS_TEXT_TABLE: u32 = 0x0047_c440;

pub(crate) const MC_STATUS_TEXT_TABLE_LEN: u64 = 0x38;

// ---------------------------------------------------------------------------
// Card error text.

/// `void MCManager_GetCardErrorText(MCManager* m, int slot, u16* dst)`
/// (0x241710, `cMCOverlayManager::GetDeviceDisplayString`): the text shown
/// for the card in `slot`. Not detected (slot `mcus::VT_DEVICE_OK` false, or
/// any of the three not-detected slots true) and not formatted give the
/// localised message with the slot name ("1", "2", ...) formatted in; a
/// formatted card with a name gives the device name (`mcus::DEVICE_NAME`,
/// +0x18 of the device record), or the "memory device" message when that name
/// is empty; a
/// formatted card without one leaves `dst` alone.
pub fn mc_manager_get_card_error_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x240);
        frame.save(c, S0, 0x230);
        frame.save(c, S1, 0x220);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x210);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[S2][0] = c.g[A2][0];
        let m = r32(c, S1);
        mcus_load_card(c, m, A3);
        mcus_card_call(c, A3, V1, V0, mcus::VT_DEVICE_OK, 0x0024_1748)?;
        let device_ok = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[S0][0];
        if !device_ok {
            c.cycles += 20;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_18cc)?;
            mc_card_error_sprintf(
                c,
                frame,
                MC_TEXT_NOT_DETECTED,
                [0x0024_18ec, 0x0024_18fc, 0x0024_190c],
            )?;
            c.cycles += 1;
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }

        // Any of the three: not detected.
        c.cycles += 8;
        let mut not_detected = false;
        for (i, (slot, ret_at)) in [
            (mcus::VT_NOT_DETECTED_A, 0x0024_1768),
            (mcus::VT_NOT_DETECTED_B, 0x0024_178c),
            (mcus::VT_NOT_DETECTED_C, 0x0024_17b0),
        ]
        .into_iter()
        .enumerate()
        {
            let m = r32(c, S1);
            mcus_load_card(c, m, A2);
            if i > 0 {
                c.cycles += 9;
                c.g[A1][0] = c.g[S0][0];
            }
            mcus_card_call(c, A2, V1, V0, slot, ret_at)?;
            let hit = c.g[V0][0] != 0;
            c.g[A2][0] = c.g[S0][0];
            if hit {
                not_detected = true;
                break;
            }
        }
        if not_detected {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_17c4)?;
            c.cycles += 15;
            mc_card_error_sprintf(
                c,
                frame,
                MC_TEXT_NOT_DETECTED,
                [0x0024_1884, 0x0024_1894, 0x0024_18a4],
            )?;
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 9;
        let m = r32(c, S1);
        mcus_load_card(c, m, A2);
        c.g[A1][0] = c.g[S0][0];
        mcus_card_call(c, A2, V1, V0, mcus::VT_FORMATTED, 0x0024_17f0)?;
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1808)?;
            c.cycles += 15;
            mc_card_error_sprintf(
                c,
                frame,
                MC_TEXT_NOT_FORMATTED,
                [0x0024_1884, 0x0024_1894, 0x0024_18a4],
            )?;
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }
        let m = r32(c, S1);
        mcus_load_card(c, m, A2);

        c.cycles += 8;
        c.g[A1][0] = c.g[S0][0];
        mcus_card_call(c, A2, V1, V0, mcus::VT_HAS_NAME, 0x0024_1830)?;
        let has_name = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if !has_name {
            c.cycles += 1;
            frame.restore(c, S0, 0x230);
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 11;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1844)?;
        c.g[V0][0] = mcus::DEVICE_SIZE;
        c.mult(0, V1, S0, V0);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(mcus::DEVICE_NAME));
        c.g[A0][0] = c.g[S0][0];
        callee::USTR_LEN_2.call(c, 0x0024_185c)?;
        let named = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if named {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            callee::USTR_COPY.call(c, 0x0024_18b4)?;
            frame.restore(c, S0, 0x230);
            mc_card_error_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 3;
        c.cycles += 15;
        mc_card_error_sprintf(
            c,
            frame,
            MC_TEXT_MEM_DEVICE,
            [0x0024_1884, 0x0024_1894, 0x0024_18a4],
        )?;
        mc_card_error_epilogue(c, frame);
        Ok(())
    })
}

/// `UStr_Sprintf(dst, text(key), slot_name)` into `dst` (`s2`), the slot
/// name in the frame's buffer; then reload `s0` as the compiled tail does.
/// `rets`: the hash, lookup and format calls' return addresses.
pub(crate) fn mc_card_error_sprintf(c: &mut Cpu, frame: Frame, key: u32, rets: [u32; 3]) -> Flow {
    memcard_text(c, key, [rets[0], rets[1]])?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[SP][0];
    callee::U_STR_SPRINTF.call(c, rets[2])?;
    frame.restore(c, S0, 0x230);
    Ok(())
}

/// Reload s1, s2 and `ra`, pop the 0x240 frame and return.
pub(crate) fn mc_card_error_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    frame.restore(c, S1, 0x220);
    frame.restore(c, S2, 0x210);
    frame.restore_ra(c, 0x200);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Card status flags.

/// `u32 MCManager_ReadCardStatus(MCManager* m)` (0x241b20): the card's status
/// flags in `v0`. While the checked slot (`+0x42c`) is not the selected one
/// (`+0x428`) the word at `+0x43c`; otherwise, from the card driver: bit 0 the
/// low bit of slot 0x188, bit 1 slot 0x198 or 0x1d8, bit 2 "no saves and the
/// free count below the space a save needs", bit 3 the inverted low bit of
/// `mcus::VT_CAN_SAVE`, bit 4 `mcus::VT_CAN_LOAD` or slot 0x1d0 false; bits 5-8
/// clear.
pub fn mc_manager_read_card_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S2);
        c.g[V1][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);
        c.g[V0][0] = mcus_at(m, mcus::SELECTED).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            c.g[V0][0] = mcus_at(m, mcus::CARD_STATUS).lw(c);
            mc_card_status_epilogue(c, frame);
            return Ok(());
        }
        mcus_load_card(c, m, A1);

        // Bits 0 and 1.
        c.cycles += 15;
        c.g[S1][0] = 0;
        mcus_card_call(c, A1, V1, V0, mcus::VT_STATUS_BIT0, 0x0024_1b6c)?;
        let m = r32(c, S2);
        mcus_load_card(c, m, A1);
        c.g[S0][0] = c.g[V0][0] & 1;
        mcus_card_call(c, A1, V1, V0, mcus::VT_STATUS_BIT1_A, 0x0024_1b88)?;
        if c.g[V0][0] != 0 {
            c.g[S1][0] = 1;
            c.cycles += 1;
        } else {
            c.cycles += 8;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_STATUS_BIT1_B, 0x0024_1ba8)?;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                c.g[S1][0] = 1;
                c.cycles += 1;
            }
        }
        let m = r32(c, S2);
        mcus_load_card(c, m, A1);

        // Bit 2: no saves, and less free than a save needs.
        c.cycles += 10;
        c.g[V0][0] = sx(r32(c, S1) << 1);
        c.g[S1][0] = c.g[S0][0] | c.g[V0][0];
        c.g[S0][0] = 0;
        mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_COUNT, 0x0024_1bd8)?;
        if c.g[V0][0] == 0 {
            c.cycles += 15;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_FREE_SPACE, 0x0024_1bf8)?;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            c.g[S0][0] = c.g[V0][0];
            mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_SIZE, 0x0024_1c14)?;
            c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        }
        let m = r32(c, S2);
        mcus_load_card(c, m, A2);

        // Bits 3 and 4.
        c.cycles += 24;
        c.g[V0][0] = 0xffff_ffff_ffff_fffb;
        c.g[S1][0] &= c.g[V0][0];
        c.g[V1][0] = sx(r32(c, S0) << 2);
        c.g[S1][0] |= c.g[V1][0];
        c.g[S3][0] = 0;
        mcus_card_call(c, A2, A1, V0, mcus::VT_CAN_SAVE, 0x0024_1c44)?;
        let m = r32(c, S2);
        mcus_load_card(c, m, A2);
        c.g[V0][0] = (c.g[V0][0] ^ 1) & 1;
        c.g[V1][0] = 0xffff_ffff_ffff_fff7;
        c.g[V0][0] = sx(r32(c, V0) << 3);
        c.g[S1][0] &= c.g[V1][0];
        c.g[S0][0] = c.g[S1][0] | c.g[V0][0];
        mcus_card_call(c, A2, A1, V0, mcus::VT_CAN_LOAD, 0x0024_1c74)?;
        if c.g[V0][0] == 0 {
            c.g[S3][0] = 1;
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffef;
        } else {
            c.cycles += 8;
            let m = r32(c, S2);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_STATUS_BIT4, 0x0024_1c94)?;
            let ok = c.g[V0][0] != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffef;
            if !ok {
                c.cycles += 1;
                c.g[S3][0] = 1;
                c.cycles += 1;
            }
        }
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, S3) << 4);
        c.g[S1][0] = c.g[S0][0] & c.g[V0][0];
        c.g[V0][0] = 0xffff_ffff_ffff_fe1f;
        c.g[S1][0] |= c.g[V1][0];
        c.g[V0][0] = c.g[S1][0] & c.g[V0][0];
        mc_card_status_epilogue(c, frame);
        Ok(())
    })
}

/// Reload s0..s3 and `ra`, pop the 0x50 frame and return.
pub(crate) fn mc_card_status_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 7;
    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Status text.

/// `void MCManager_GetStatusText(MCManager* m, u16* out)` (0x241e18): the
/// text for the manager's state (`+0x130`): "checking card" (states 0, 2, 3,
/// 6, 7, 13, 29, 49..=54), "not detected" (4), "formatting" (10, 11, 39, 40),
/// "saving" (15..=18), "loading" (31..=34), each with the selected slot's name
/// (`+0x428`) formatted in, or "deleting" (25, 26, 44) as is; every other
/// state leaves `out` empty. The state picks a code address from the switch's
/// table in `.rodata`, read from guest memory as the compiled `jr` does.
pub fn mc_manager_get_status_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x450);
        frame.save(c, S3, 0x410);
        frame.save(c, S0, 0x440);
        c.g[S3][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x430);
        c.g[A0][0] = c.g[S3][0];
        frame.save(c, S2, 0x420);
        frame.save_ra(c, 0x400);
        c.g[A1][0] = sx(MC_TEXT_EMPTY);
        callee::USTR_FROM_ASCII_2.call(c, 0x0024_1e48)?;
        let m = r32(c, S0);
        c.g[A2][0] = mcus_at(m, mcus::SELECTED).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1e58)?;
        let m = r32(c, S0);
        c.g[V1][0] = mcus_at(m, mcus::STATE).lw(c);
        let in_table = c.g[V1][0] < MC_STATUS_TEXT_TABLE_LEN;
        c.g[V0][0] = 0x48_0000;
        if !in_table {
            c.cycles += 1;
            frame.restore(c, S0, 0x440);
            mc_status_text_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V0][0] = sx(MC_STATUS_TEXT_TABLE);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let block = r32(c, A0);
        let key = match block {
            0x0024_1e80 => {
                c.cycles += 4;
                MC_TEXT_CHECKING_CARD
            }
            0x0024_1e90 => {
                // Not detected: the slot name goes to a second buffer.
                c.cycles += 23;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = sx(sp(c).wrapping_add(0x200));
                let m = r32(c, A0);
                c.g[A2][0] = mcus_at(m, mcus::SELECTED).lw(c);
                c.g[A1][0] = c.g[S2][0];
                callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0024_1ea4)?;
                mc_status_text_sprintf(
                    c,
                    frame,
                    MC_TEXT_NOT_DETECTED,
                    S2,
                    [0x0024_1ec4, 0x0024_1ed4, 0x0024_1ee4],
                )?;
                mc_status_text_epilogue(c, frame);
                return Ok(());
            }
            0x0024_1eec => {
                c.cycles += 4;
                MC_TEXT_FORMATTING
            }
            0x0024_1efc => {
                c.cycles += 4;
                MC_TEXT_SAVING
            }
            0x0024_1f0c => {
                c.cycles += 3;
                MC_TEXT_LOADING
            }
            0x0024_1f54 => {
                // Deleting: the text as is.
                c.cycles += 15;
                memcard_text(c, MC_TEXT_DELETING, [0x0024_1f74, 0x0024_1f84])?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                callee::USTR_COPY.call(c, 0x0024_1f90)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x440);
                mc_status_text_epilogue(c, frame);
                return Ok(());
            }
            0x0024_1f90 => {
                c.cycles += 1;
                frame.restore(c, S0, 0x440);
                mc_status_text_epilogue(c, frame);
                return Ok(());
            }
            _ => {
                c.jump_out(block, 0x0024_1e18, 0x0024_1fac);
                return Ok(());
            }
        };
        c.cycles += 15;
        mc_status_text_sprintf(c, frame, key, SP, [0x0024_1f2c, 0x0024_1f3c, 0x0024_1f4c])?;
        mc_status_text_epilogue(c, frame);
        Ok(())
    })
}

/// `UStr_Sprintf(out, text(key), slot_name)` into `out` (`s3`), the slot name
/// in the buffer that `name_reg` points at; then reload `s0` as the compiled
/// tails do. `rets`: the hash, lookup and format calls' return addresses.
pub(crate) fn mc_status_text_sprintf(
    c: &mut Cpu,
    frame: Frame,
    key: u32,
    name_reg: usize,
    rets: [u32; 3],
) -> Flow {
    memcard_text(c, key, [rets[0], rets[1]])?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = c.g[name_reg][0];
    callee::U_STR_SPRINTF.call(c, rets[2])?;
    frame.restore(c, S0, 0x440);
    Ok(())
}

/// Reload s1..s3 and `ra`, pop the 0x450 frame and return.
pub(crate) fn mc_status_text_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0x430);
    frame.restore(c, S2, 0x420);
    frame.restore(c, S3, 0x410);
    frame.restore_ra(c, 0x400);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Device record.

/// `void MCManager_ReadDirInfo(MCManager* m, int slot)` (0x2420c8): refresh the
/// device record of `slot` (`mcus::DEVICES + slot * 0xe0`). Sets its first
/// four words to 1 and clears its name, then asks the card driver: any
/// not-detected slot true clears the first word, not formatted clears
/// `mcus::DEVICE_FORMATTED`, no name clears `mcus::DEVICE_HAS_NAME` (each stops
/// there); otherwise stores the free count when `slot` is the current one
/// (`+0x42c`) and reads the device name (at most 100 units).
pub fn mc_manager_read_dir_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x70);
        c.g[A3][0] = 1;
        frame.save(c, S1, 0x50);
        c.g[A2][0] = mcus::DEVICE_NAME_BYTES;
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S5][0] = mcus::DEVICE_SIZE;
        frame.save(c, S0, 0x60);
        c.mult(0, V1, S1, S5);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(mcus::DEVICE_FORMATTED));
        frame.save(c, S3, 0x30);
        c.g[T0][0] = sx(r32(c, S0).wrapping_add(mcus::DEVICE_HAS_NAME));
        frame.save(c, S4, 0x20);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
        Addr(r32(c, V0)).off(mcus::DEVICES).sw(c, c.g[A3][0]);
        c.g[S2][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
        c.g[S3][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
        Addr(r32(c, S2)).sw(c, c.g[A3][0]);
        c.g[S4][0] = sx(r32(c, V0).wrapping_add(mcus::DEVICE_NAME));
        c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V1)));
        Addr(r32(c, S3)).sw(c, c.g[A3][0]);
        c.g[A0][0] = c.g[S4][0];
        Addr(r32(c, V1))
            .off(mcus::DEVICE_PLUGGED_IN)
            .sw(c, c.g[A3][0]);
        // EA_MemSet(name, 0, 200)
        call_ptr(c, callee::EA_MEM_SET.addr, 0x0024_213c)?;

        // Any of the three: not detected.
        let mut not_detected = false;
        for (i, (slot, ret_at)) in [
            (mcus::VT_NOT_DETECTED_A, 0x0024_2158),
            (mcus::VT_NOT_DETECTED_B, 0x0024_217c),
            (mcus::VT_NOT_DETECTED_C, 0x0024_21a0),
        ]
        .into_iter()
        .enumerate()
        {
            if i > 0 {
                c.cycles += 9;
            }
            let m = r32(c, S0);
            mcus_load_card(c, m, A2);
            c.g[A1][0] = c.g[S1][0];
            mcus_card_call(c, A2, V1, V0, slot, ret_at)?;
            let hit = c.g[V0][0] != 0;
            c.g[V0][0] = mcus::DEVICE_SIZE;
            if hit {
                not_detected = true;
                break;
            }
        }
        if not_detected {
            c.cycles += 4;
            c.mult(0, V1, S1, V0);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S0)));
            Addr(r32(c, V0)).off(mcus::DEVICES).sw(c, 0);
            mc_read_dir_info_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 9;
        let m = r32(c, S0);
        mcus_load_card(c, m, A2);
        c.g[A1][0] = c.g[S1][0];
        mcus_card_call(c, A2, V1, V0, mcus::VT_FORMATTED, 0x0024_21d4)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            Addr(r32(c, S2)).sw(c, 0);
            mc_read_dir_info_epilogue(c, frame);
            return Ok(());
        }
        let m = r32(c, S0);
        mcus_load_card(c, m, A2);

        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        mcus_card_call(c, A2, V1, V0, mcus::VT_HAS_NAME, 0x0024_21fc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            Addr(r32(c, S3)).sw(c, 0);
            mc_read_dir_info_epilogue(c, frame);
            return Ok(());
        }
        let m = r32(c, S0);
        c.g[V0][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);

        c.cycles += 2;
        if c.g[S1][0] == c.g[V0][0] {
            // The current slot: keep its free count.
            c.cycles += 11;
            let m = r32(c, S0);
            mcus_load_card(c, m, A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_FREE_SPACE, 0x0024_222c)?;
            let m = r32(c, S0);
            c.g[V1][0] = mcus_at(m, mcus::CURR_CHECK).lw(c);
            c.mult(0, A0, V1, S5);
            c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
            Addr(r32(c, V1))
                .off(mcus::DEVICE_FREE_SPACE)
                .sw(c, c.g[V0][0]);
        }
        let m = r32(c, S0);
        mcus_load_card(c, m, T0);

        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S4][0];
        c.g[A3][0] = mcus::DEVICE_NAME_CHARS;
        mcus_card_call(c, T0, V0, V1, mcus::VT_READ_NAME, 0x0024_2260)?;
        mc_read_dir_info_epilogue(c, frame);
        Ok(())
    })
}

/// Reload s0..s5 and `ra`, pop the 0x70 frame and return.
pub(crate) fn mc_read_dir_info_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// Post the code in `a1` to the manager's message sink: `v0` holds the sink
/// (`self->+0x748`); its slot 8 (`{short delta; fn}`) is called with
/// `self + delta`, `self` in `this`.
pub(crate) fn mc_check_report(c: &mut Cpu, this: usize, ret: u32) -> Flow {
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, this).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// The "quit" test the check states share: `quit = mSave (+0xbc) || mDelete
/// (+0xc0) || the load-pressed hook (+0x78)()`, as 0/1 into `quit`. The caller
/// has loaded `v0 = self->+0xbc` and charged its cycles; leaves `v0 =
/// self->+0x128`. `self` is in `this`.
pub(crate) fn mc_check_quit(c: &mut Cpu, this: usize, quit: usize, ret: u32) -> Flow {
    c.g[quit][0] = 0;
    'set: {
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, this), mcus::DELETE).lw(c);
            if c.g[V0][0] != 0 {
                c.g[quit][0] = 1;
                c.cycles += 1;
                break 'set;
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(r32(c, this), mcus::CB_LOAD_PRESSED).lw(c);
            let hook = r32(c, V0);
            call_ptr(c, hook, ret)?;
            if c.g[V0][0] == 0 {
                break 'set;
            }
        }
        c.cycles += 2;
        c.g[quit][0] = 1;
    }
    c.g[V0][0] = mcus_at(r32(c, this), 0x128).lw(c);
    Ok(())
}

/// The card-ready probe of the check states: backend slots 0x1b8 and 0x1d0
/// answer non-zero, 0x198 and 0x1d8 zero, stopping at the first that does
/// not. `a1` holds the backend (`self->+0x434`) and the first call's cycles
/// are charged by the caller; each later probe reloads it. A failed probe
/// loads `v0 = self->fail` when given. `self` is in `this`; `rets` are the
/// four return addresses.
pub(crate) fn mc_check_card_ready(
    c: &mut Cpu,
    this: usize,
    rets: [u32; 4],
    fail: Option<u32>,
) -> Result<bool, Unwound> {
    let probes = [
        (mcus::VT_CAN_LOAD, true),
        (mcus::VT_STATUS_BIT4, true),
        (mcus::VT_STATUS_BIT1_A, false),
        (mcus::VT_STATUS_BIT1_B, false),
    ];
    for (i, (slot, want)) in probes.into_iter().enumerate() {
        if i > 0 {
            c.cycles += 8;
            mcus_load_card(c, r32(c, this), A1);
        }
        mcus_card_call(c, A1, V1, V0, slot, rets[i])?;
        if (c.g[V0][0] != 0) != want {
            if let Some(off) = fail {
                c.g[V0][0] = mcus_at(r32(c, this), off).lw(c);
            }
            return Ok(false);
        }
    }
    Ok(true)
}

/// How `MCManager_State03_Check` leaves its body.
pub(crate) enum State03End {
    /// Straight to the epilogue.
    Return,
    /// Post `a1` to the sink in `v0`.
    Report,
    /// Post 0x1d ("no card") to the sink in `v0`.
    ReportNoCard,
}

/// `void MCManager_State03_Check(MCManager *self)` (0x23e540): the card check
/// after the boot wait. Returns at once while the backend (`+0x434`, slot
/// 0x58) is busy. `quit` = save pending (`+0xbc`), delete pending (`+0xc0`) or
/// the load-pressed hook (`+0x78`). With the word at `+0x128` set, a changed
/// card (`+0xc8`) while quitting posts 0x38. Then `MCManager_PollCardStatus`;
/// when it returns non-zero the status flags at `+0x43c` (old flags kept in
/// `s2`) decide: flag 8 with `quit` or the word at `+0x124` →
/// `MCManager_SetStateNoCard`; flag 1 → post 5; flag 0x10 with the word at
/// `+0x120` → post 0x27; flag 8 dropped, or all flags cleared while others
/// than 4 were set → `MCManager_OnCardScanned`. Without `quit`, a changed
/// card → clear `+0xc8` and `MCManager_ReturnToSlotState`, flag 8 → post
/// 0x31. Last the card-ready probe: ready → post 6, else post 7 with a save
/// pending or 0x1d without.
pub fn mc_manager_state03_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let end = 'body: {
            mcus_load_card(c, r32(c, S0), A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_e570)?;
            if c.g[V0][0] != 0 {
                break 'body State03End::Return;
            }
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::SAVE).lw(c);
            mc_check_quit(c, S0, S1, 0x0023_e59c)?;
            // A changed card while quitting, with +0x128 set: post 0x38.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_CHANGED).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = 0x38;
                    if c.g[S1][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                        break 'body State03End::Report;
                    }
                    c.cycles += 1;
                }
            }
            // The flags before the poll.
            c.g[S2][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0023_e5dc)?;
            if c.g[V0][0] != 0 {
                // The new flags go to v1; `release` arms leave v0 = !4.
                let scan = 'flags: {
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S0), 0x124).lw(c);
                    let mut no_card_check = c.g[V0][0] != 0;
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
                    if !no_card_check {
                        c.cycles += 2;
                        no_card_check = c.g[S1][0] != 0;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] &= 8;
                    if no_card_check {
                        let no_card = c.g[V0][0] != 0;
                        c.g[V0][0] = c.g[V1][0] & 1;
                        if no_card {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0023_e610)?;
                            break 'body State03End::Return;
                        }
                    } else {
                        let no_card = c.g[V0][0] != 0;
                        c.g[V0][0] = !4;
                        if no_card {
                            break 'flags false;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.g[V1][0] & 1;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 5;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                        break 'body State03End::Report;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S0), 0x120).lw(c);
                    let skip = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0] & 0x10;
                    if !skip {
                        c.cycles += 2;
                        c.g[A1][0] = 0x27;
                        if c.g[V0][0] != 0 {
                            c.cycles += 2;
                            c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                            break 'body State03End::Report;
                        }
                    }
                    // Dead test: flag 8 is clear on every path to here.
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 8;
                    let no_card = c.g[V0][0] != 0;
                    c.g[V0][0] = !4;
                    if no_card {
                        break 'flags false;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S2][0] & 8;
                    let lost = c.g[V0][0] != 0;
                    c.g[V0][0] = !4;
                    lost
                };
                // `release`: no new flags while some old ones besides 4 were set.
                let mut scan = scan;
                if !scan {
                    c.cycles += 2;
                    let any = c.g[V1][0] != 0;
                    c.g[V0][0] &= c.g[S2][0];
                    if !any {
                        c.cycles += 2;
                        scan = c.g[V0][0] != 0;
                    }
                }
                if scan {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_ON_CARD_SCANNED.call(c, 0x0023_e688)?;
                    break 'body State03End::Return;
                }
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_CHANGED).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    mcus_at(r32(c, S0), mcus::CARD_CHANGED).sw(c, 0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_RETURN_TO_SLOT_STATE.call(c, 0x0023_e6b0)?;
                    break 'body State03End::Return;
                }
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
                c.cycles += 3;
                c.g[V0][0] &= 8;
                c.g[A1][0] = 0x31;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                    break 'body State03End::Report;
                }
                c.cycles += 1;
            }
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::SAVE).lw(c);
            c.cycles += 2;
            let saving = c.g[V0][0] != 0;
            mcus_load_card(c, r32(c, S0), A1);
            c.cycles += 7;
            if saving {
                let rets = [0x0023_e6ec, 0x0023_e70c, 0x0023_e72c, 0x0023_e74c];
                if mc_check_card_ready(c, S0, rets, Some(mcus::MSG))? {
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                    c.cycles += 1;
                    c.g[A1][0] = 6;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = 7;
                }
                break 'body State03End::Report;
            }
            let rets = [0x0023_e770, 0x0023_e790, 0x0023_e7b0, 0x0023_e7d0];
            if mc_check_card_ready(c, S0, rets, Some(mcus::MSG))? {
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                c.cycles += 1;
                c.g[A1][0] = 6;
                break 'body State03End::Report;
            }
            State03End::ReportNoCard
        };
        match end {
            State03End::Return => {}
            State03End::Report => {
                c.cycles += 6;
                mc_check_report(c, S0, 0x0023_e7ec)?;
            }
            State03End::ReportNoCard => {
                c.cycles += 6;
                c.g[A1][0] = 0x1d;
                mc_check_report(c, S0, 0x0023_e808)?;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// How `MCManager_State06_Check` leaves its body (`s0` is reloaded on every
/// way out but `Return`, which reloaded it before).
pub(crate) enum State06End {
    /// Straight to the epilogue.
    Return,
    /// Still counting down: one more cycle.
    Wait,
    /// Post `a1` to the sink in `v0`.
    Report,
    /// `MCManager_SetStateNoCard`.
    NoCard,
    /// Reread the slot's directory and post 0x1d.
    Fail,
}

/// `void MCManager_State06_Check(MCManager *self)` (0x23ec00): the card check
/// after a directory listing. Polls the card (`MCManager_PollCardStatus`,
/// kept in `s2`), counts the wait at `+0x108` down and returns while it was
/// above 0 or the backend (`+0x434`, slot 0x58) is busy. A changed card
/// (`+0xc8`): `quit` (save, delete pending or the load-pressed hook at
/// `+0x78`) with the word at `+0x128` set posts 0x38; a ready card (the
/// probe) posts 6. With a save pending (`+0xbc`) the flags at `+0x43c`
/// decide: flag 8 → `MCManager_SetStateNoCard` when the poll was non-zero, else
/// the slot's directory is reread and 7 posted; flag 1 → 0x14; a ready card
/// that the backend says has no room (slot 0x68 zero, slot 0xa0 below slot
/// 0xb8 or slot 0xc8 zero) → 0x15; otherwise reread and post 7. Without a
/// save pending: flag 8 → `MCManager_SetStateNoCard` when quitting or the
/// word at `+0x124` is set; flag 1 → 0x24 or 5 by the load-pressed hook;
/// else reread the directory and post 0x1d.
pub fn mc_manager_state06_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0023_ec1c)?;
        c.g[S2][0] = c.g[V0][0];
        let end = 'body: {
            let wait = mcus_at(r32(c, S1), mc::WAIT);
            c.g[V0][0] = wait.lw(c);
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let waiting = (c.g[V1][0] as i64) > 0;
            let left = c.g[V0][0];
            wait.sw(c, left);
            if waiting {
                break 'body State06End::Wait;
            }
            c.cycles += 8;
            mcus_load_card(c, r32(c, S1), A1);
            mcus_card_call(c, A1, V1, V0, mcus::VT_BUSY, 0x0023_ec4c)?;
            let busy = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x30);
            if busy {
                break 'body State06End::Return;
            }
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_CHANGED).lw(c);
            let changed = c.g[V0][0] != 0;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::SAVE).lw(c);
            if changed {
                c.cycles += 2;
                mc_check_quit(c, S1, S0, 0x0023_ec80)?;
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = 0x38;
                    if c.g[S0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                        break 'body State06End::Report;
                    }
                    c.cycles += 1;
                }
                mcus_load_card(c, r32(c, S1), A1);
                c.cycles += 7;
                let rets = [0x0023_ecc0, 0x0023_ece0, 0x0023_ed00, 0x0023_ed20];
                if mc_check_card_ready(c, S1, rets, Some(mcus::SAVE))? {
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    c.g[A1][0] = 6;
                    break 'body State06End::Report;
                }
            }
            // v0 = the save-pending word.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                'reread: {
                    c.cycles += 4;
                    c.g[V1][0] = mcus_at(r32(c, S1), mcus::CARD_STATUS).lw(c);
                    c.g[V0][0] = c.g[V1][0] & 8;
                    let no_card = c.g[V0][0] != 0;
                    c.g[V0][0] = c.g[V1][0] & 1;
                    if no_card {
                        c.cycles += 2;
                        if c.g[S2][0] != 0 {
                            break 'body State06End::NoCard;
                        }
                        break 'reread;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0x14;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                        break 'body State06End::Report;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, r32(c, S1), A1);
                    let rets = [0x0023_ed74, 0x0023_ed94, 0x0023_edb4, 0x0023_edd4];
                    if !mc_check_card_ready(c, S1, rets, None)? {
                        break 'reread;
                    }
                    c.cycles += 8;
                    mcus_load_card(c, r32(c, S1), A1);
                    mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_COUNT, 0x0023_edf4)?;
                    if c.g[V0][0] != 0 {
                        break 'reread;
                    }
                    // No room: slot 0xa0 below slot 0xb8, or slot 0xc8 zero.
                    c.cycles += 16;
                    mcus_load_card(c, r32(c, S1), A1);
                    mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_SIZE, 0x0023_ee14)?;
                    mcus_load_card(c, r32(c, S1), A1);
                    c.g[S0][0] = c.g[V0][0];
                    mcus_card_call(c, A1, V1, V0, mcus::VT_FREE_SPACE, 0x0023_ee30)?;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    } else {
                        c.cycles += 8;
                        mcus_load_card(c, r32(c, S1), A1);
                        mcus_card_call(c, A1, V1, V0, 0xc8, 0x0023_ee54)?;
                        if c.g[V0][0] != 0 {
                            break 'reread;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0x15;
                    break 'body State06End::Report;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                callee::MC_MANAGER_READ_CURRENT_DIR_INFO.call(c, 0x0023_ee78)?;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                c.g[A1][0] = 7;
                break 'body State06End::Report;
            }
            // No save pending: `quit` = delete pending or the hook.
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::DELETE).lw(c);
            c.cycles += 2;
            let mut pending = c.g[V0][0] != 0;
            c.g[S0][0] = 0;
            if !pending {
                c.cycles += 5;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_LOAD_PRESSED).lw(c);
                let hook = r32(c, V0);
                call_ptr(c, hook, 0x0023_ee98)?;
                pending = c.g[V0][0] != 0;
            }
            if pending {
                c.cycles += 2;
                c.g[S0][0] = 1;
            }
            c.g[V0][0] = mcus_at(r32(c, S1), 0x124).lw(c);
            c.cycles += 2;
            let mut no_card_check = c.g[V0][0] != 0;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_STATUS).lw(c);
            if !no_card_check {
                c.cycles += 2;
                no_card_check = c.g[S0][0] != 0;
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] &= 8;
            let no_card = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[V1][0] & 1;
            if no_card {
                if no_card_check {
                    break 'body State06End::NoCard;
                }
                break 'body State06End::Fail;
            }
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                break 'body State06End::Fail;
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_LOAD_PRESSED).lw(c);
            let hook = r32(c, V0);
            call_ptr(c, hook, 0x0023_eefc)?;
            let released = c.g[V0][0] == 0;
            c.g[A1][0] = 0x24;
            if !released {
                c.cycles += 2;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                break 'body State06End::Report;
            }
            c.cycles += 2;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
            c.g[A1][0] = 5;
            State06End::Report
        };
        match end {
            State06End::Return => {}
            State06End::Wait => {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
            State06End::Report => {
                c.cycles += 6;
                mc_check_report(c, S1, 0x0023_ef24)?;
                frame.restore(c, S0, 0x30);
            }
            State06End::NoCard => {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0023_eed0)?;
                frame.restore(c, S0, 0x30);
            }
            State06End::Fail => {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                callee::MC_MANAGER_READ_CURRENT_DIR_INFO.call(c, 0x0023_ef34)?;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                c.g[A1][0] = 0x1d;
                mc_check_report(c, S1, 0x0023_ef4c)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// How `MCManager_State07_Check` leaves its body.
pub(crate) enum State07End {
    /// Straight to the epilogue (`s0` already reloaded).
    Return,
    /// Post `a1` to the sink in `v0`.
    Report,
    /// The delete-pressed tail (`v0` holds the hook at `+0x68`).
    DeleteCheck,
}

/// `void MCManager_State07_Check(MCManager *self)` (0x23f698): the save/load
/// screen's card check. Nothing while the periodic card poll (`+0x338`) runs.
/// With the word at `+0x128` set, a changed card posts 0x38. A non-zero
/// `MCManager_PollCardStatus` with the save-pressed hook quiet: status flag 8
/// → `MCManager_SetStateNoCard`, else flags 0x10/2/1/4 post 0x27/0x30/5/0x17.
/// A changed card → clear it and `MCManager_ReturnToSlotState`; the hook at
/// `+0x6c` → post 1. Save pressed sets the ramp floats; with saving allowed:
/// card not ready → 0xa, no room for a new file → 0x15, another used slot
/// named as the one at `+0x276` → 0x37, the cursor's slot used → its name into
/// `+0x234` and 9, else 0xd; without: driver slot 0x208 → 0x16, 0x210 → 0x17,
/// `VT_CAN_SAVE` zero → 0x14. Last, delete pressed with deleting allowed: the
/// cursor's name into `+0x234`, post 0x18.
pub fn mc_manager_state07_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x230);
        frame.save(c, S1, 0x210);
        frame.save(c, S0, 0x220);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CHECK_CARD).lw(c);
        let checking = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x220);
        if !checking {
            let end = mc_state07_body(c, frame)?;
            match end {
                State07End::Return => {}
                State07End::Report => {
                    c.cycles += 6;
                    mc_check_report(c, S1, 0x0023_fa34)?;
                    frame.restore(c, S0, 0x220);
                }
                State07End::DeleteCheck => {
                    c.cycles += 4;
                    let hook = r32(c, V0);
                    call_ptr(c, hook, 0x0023_fa80)?;
                    let pressed = c.g[V0][0] != 0;
                    frame.restore(c, S0, 0x220);
                    if pressed {
                        c.cycles += 3;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::DELETE_OK).lw(c);
                        let allowed = c.g[V0][0] != 0;
                        c.g[A2][0] = sx(r32(c, S1).wrapping_add(0x234));
                        if allowed {
                            c.cycles += 14;
                            mcus_load_card(c, r32(c, S1), A3);
                            c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                            mcus_card_call(c, A3, V0, V1, MEMCARD_CARD_VT_SLOT_NAME, 0x0023_fab0)?;
                            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                            c.g[A1][0] = 0x18;
                            mc_check_report(c, S1, 0x0023_fac8)?;
                            frame.restore(c, S0, 0x220);
                        }
                    }
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// The head of `MCManager_State07_Check`'s slot-name loop.
pub(crate) const MC_STATE07_NAME_LOOP: u32 = 0x0023_f8ec;

/// [`mc_manager_state07_check`] after the card-poll test: returns how it ends.
pub(crate) fn mc_state07_body(c: &mut Cpu, frame: Frame) -> Result<State07End, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = mcus_at(r32(c, S1), 0x128).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_CHANGED).lw(c);
        let changed = c.g[V0][0] != 0;
        c.g[A1][0] = 0x38;
        if changed {
            c.cycles += 2;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
            return Ok(State07End::Report);
        }
    }
    c.cycles += 8;
    c.g[A0][0] = c.g[S1][0];
    callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0023_f6e0)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_SAVE_PRESSED).lw(c);
    let hook = r32(c, V0);
    call_ptr(c, hook, 0x0023_f6f0)?;
    'flags: {
        if c.g[V0][0] != 0 {
            break 'flags;
        }
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            break 'flags;
        }
        c.cycles += 4;
        c.g[V1][0] = mcus_at(r32(c, S1), mcus::CARD_STATUS).lw(c);
        c.g[V0][0] = c.g[V1][0] & 8;
        let no_card = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] & 0x10;
        if no_card {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0023_f718)?;
            frame.restore(c, S0, 0x220);
            return Ok(State07End::Return);
        }
        c.cycles += 2;
        c.g[A1][0] = 0x27;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
            return Ok(State07End::Report);
        }
        for (flag, code) in [(2, 0x30), (1, 5), (4, 0x17)] {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & flag;
            c.g[A1][0] = code;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                return Ok(State07End::Report);
            }
        }
        c.cycles += 1;
    }
    c.g[V0][0] = mcus_at(r32(c, S1), mcus::CARD_CHANGED).lw(c);
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        mcus_at(r32(c, S1), mcus::CARD_CHANGED).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::MC_MANAGER_RETURN_TO_SLOT_STATE.call(c, 0x0023_f784)?;
        frame.restore(c, S0, 0x220);
        return Ok(State07End::Return);
    }
    c.g[V0][0] = mcus_at(r32(c, S1), 0x6c).lw(c);
    c.cycles += 4;
    let hook = r32(c, V0);
    call_ptr(c, hook, 0x0023_f794)?;
    let quiet = c.g[V0][0] == 0;
    c.g[A1][0] = 1;
    if !quiet {
        c.cycles += 2;
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
        return Ok(State07End::Report);
    }
    c.cycles += 5;
    c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_SAVE_PRESSED).lw(c);
    let hook = r32(c, V0);
    call_ptr(c, hook, 0x0023_f7b0)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_DELETE_PRESSED).lw(c);
        return Ok(State07End::DeleteCheck);
    }
    'name_clash: {
        'post: {
            'not_allowed: {
                c.cycles += 7;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 0);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5124));
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::SAVE_OK).lw(c);
                c.swc1(0, mcus_at(r32(c, S1), mcus::RAMP).0);
                let allowed = c.g[V0][0] != 0;
                c.swc1(1, mcus_at(r32(c, S1), mcus::RAMP_STEP).0);
                if !allowed {
                    break 'not_allowed;
                }
                'not_ready: {
                    c.cycles += 8;
                    mcus_load_card(c, r32(c, S1), A1);
                    let rets = [0x0023_f7ec, 0x0023_f80c, 0x0023_f82c, 0x0023_f84c];
                    if !mc_check_card_ready(c, S1, rets, Some(mcus::MSG))? {
                        break 'not_ready;
                    }
                    // Room for a new file: the cursor's slot used, or slot
                    // 0xa0 not below slot 0xb8 and slot 0xc8 non-zero.
                    c.cycles += 9;
                    mcus_load_card(c, r32(c, S1), A2);
                    c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_f870)?;
                    let used = c.g[V0][0] != 0;
                    c.g[S0][0] = 0;
                    if !used {
                        c.cycles += 16;
                        mcus_load_card(c, r32(c, S1), A1);
                        mcus_card_call(c, A1, V1, V0, MEMCARD_CARD_VT_SAVE_SIZE, 0x0023_f890)?;
                        mcus_load_card(c, r32(c, S1), A1);
                        c.g[S0][0] = c.g[V0][0];
                        mcus_card_call(c, A1, V1, V0, mcus::VT_FREE_SPACE, 0x0023_f8ac)?;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                        let full = if c.g[V0][0] != 0 {
                            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                            true
                        } else {
                            c.cycles += 8;
                            mcus_load_card(c, r32(c, S1), A1);
                            mcus_card_call(c, A1, V1, V0, 0xc8, 0x0023_f8d0)?;
                            let room = c.g[V0][0] != 0;
                            c.g[S0][0] = 0;
                            if !room {
                                c.cycles += 1;
                                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                            }
                            !room
                        };
                        if full {
                            c.cycles += 2;
                            c.g[A1][0] = 0x15;
                            return Ok(State07End::Report);
                        }
                        c.cycles += 1;
                    }
                    c.cycles += 1;
                    mcus_load_card(c, r32(c, S1), A2);
                    // Slots 0..5 (s0): a used one other than the cursor with
                    // the same name clashes.
                    loop {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S0][0];
                        mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_f904)?;
                        let free = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[S0][0];
                        // A free slot, or the cursor's own name: one cycle more.
                        let mut next = free;
                        if !free {
                            c.cycles += 12;
                            mcus_load_card(c, r32(c, S1), A3);
                            c.g[A2][0] = c.g[SP][0];
                            mcus_card_call(c, A3, V0, V1, MEMCARD_CARD_VT_SLOT_NAME, 0x0023_f928)?;
                            c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x276));
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, callee::USTR_CMP, 0x0023_f934)?;
                            if c.g[V0][0] == 0 {
                                c.cycles += 3;
                                c.g[V0][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                                    break 'name_clash;
                                }
                                next = true;
                            }
                        }
                        if next {
                            c.cycles += 1;
                        }
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < 6) as u64;
                        let more = c.g[V0][0] != 0;
                        mcus_load_card(c, r32(c, S1), A2);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, MC_STATE07_NAME_LOOP);
                        }
                    }
                    c.cycles += 8;
                    c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                    mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0023_f970)?;
                    let free = c.g[V0][0] == 0;
                    c.g[A2][0] = sx(r32(c, S1).wrapping_add(0x234));
                    if !free {
                        c.cycles += 10;
                        mcus_load_card(c, r32(c, S1), A3);
                        c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
                        mcus_card_call(c, A3, V0, V1, MEMCARD_CARD_VT_SLOT_NAME, 0x0023_f994)?;
                        c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                        c.g[A1][0] = 9;
                        return Ok(State07End::Report);
                    }
                    c.cycles += 3;
                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    c.g[A1][0] = 0xd;
                    break 'post;
                }
                c.cycles += 2;
                c.g[A1][0] = 0xa;
                return Ok(State07End::Report);
            }
            // Saving not allowed.
            c.cycles += 9;
            mcus_load_card(c, r32(c, S1), A2);
            c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x208, 0x0023_f9d0)?;
            let quiet = c.g[V0][0] == 0;
            c.g[A1][0] = 0x16;
            if !quiet {
                c.cycles += 2;
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                break 'post;
            }
            c.cycles += 9;
            mcus_load_card(c, r32(c, S1), A2);
            c.g[A1][0] = mcus_at(r32(c, S1), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x210, 0x0023_f9fc)?;
            let quiet = c.g[V0][0] == 0;
            c.g[A1][0] = 0x17;
            if quiet {
                c.cycles += 8;
                mcus_load_card(c, r32(c, S1), A1);
                mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0023_fa54)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 7;
                    c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
                    c.g[A1][0] = 0x14;
                    mc_check_report(c, S1, 0x0023_fa74)?;
                }
                c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_DELETE_PRESSED).lw(c);
                return Ok(State07End::DeleteCheck);
            }
            c.cycles += 1;
            c.g[V0][0] = mcus_at(r32(c, S1), mcus::MSG).lw(c);
        }
        c.cycles += 6;
        mc_check_report(c, S1, 0x0023_fa18)?;
        c.g[V0][0] = mcus_at(r32(c, S1), mcus::CB_DELETE_PRESSED).lw(c);
        return Ok(State07End::DeleteCheck);
    }
    c.cycles += 1;
    c.g[A1][0] = 0x37;
    Ok(State07End::Report)
}

/// `void MCManager_State20_Load(MCManager *self)` (0x240130): one step of a
/// load. Counts the wait at `+0x108` down and stores the progress (backend
/// slot 0x90 over the word at `+0xf0`, as floats) at `+0x10c`. A finished
/// backend (slot 0x248) posts 0x24 and runs the hook at `+0x2c`. Otherwise,
/// when backend slot 0xf8 answers 1: while `+0xe8` is below `+0xf0` the hook
/// at `+0x1c` reads the rest (`+0xf0 - +0xe8`) and `MCManager_FinishLoad`
/// follows when it returns 0; else the hook at `+0x20` returning 0 → backend
/// slot 0x140 for the cursor `+0xf8`, then `MCManager_FinishLoad`; non-zero
/// → `MCManager_CardSetState1E` and post 0x21. Any other answer: when the
/// backend has finished meanwhile, run the hook at `+0x2c` and post 0x24.
pub fn mc_manager_state20_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let wait = mcus_at(r32(c, S0), mc::WAIT);
        c.g[V0][0] = wait.lw(c);
        mcus_load_card(c, r32(c, S0), A1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let left = c.g[V0][0];
        wait.sw(c, left);
        mcus_card_call(c, A1, V1, V0, 0x90, 0x0024_0164)?;
        // Progress = slot 0x90 / +0xf0.
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        mcus_load_card(c, r32(c, S0), A1);
        c.lwc1(1, r32(c, S0).wrapping_add(0xf0));
        c.cvt_s_w(1, 1);
        c.div_s(0, 0, 1);
        c.swc1(0, r32(c, S0).wrapping_add(0x10c));
        mcus_card_call(c, A1, V1, V0, 0x248, 0x0024_0194)?;
        let finished = c.g[V0][0] != 0;
        c.g[A1][0] = 0x24;
        if finished {
            c.cycles += 10;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
            mc_check_report(c, S0, 0x0024_01b0)?;
            c.g[V0][0] = mcus_at(r32(c, S0), 0x2c).lw(c);
            let hook = r32(c, V0);
            call_ptr(c, hook, 0x0024_01bc)?;
        } else {
            'other: {
                c.cycles += 9;
                mcus_load_card(c, r32(c, S0), A1);
                mcus_card_call(c, A1, V1, V0, 0xf8, 0x0024_01dc)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] != c.g[V1][0] {
                    mcus_load_card(c, r32(c, S0), A1);
                    break 'other;
                }
                'set_1e: {
                    'finish: {
                        c.cycles += 5;
                        c.g[V1][0] = mcus_at(r32(c, S0), 0xe8).lw(c);
                        c.g[A0][0] = mcus_at(r32(c, S0), 0xf0).lw(c);
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.cycles += 5;
                            c.g[V0][0] = mcus_at(r32(c, S0), 0x1c).lw(c);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
                            let hook = r32(c, V0);
                            call_ptr(c, hook, 0x0024_0208)?;
                            if c.g[V0][0] != 0 {
                                frame.restore(c, S0, 0x10);
                                mc_state20_epilogue(c, frame);
                                return Ok(());
                            }
                            c.cycles += 2;
                            break 'finish;
                        }
                        c.g[V0][0] = mcus_at(r32(c, S0), 0x20).lw(c);
                        c.cycles += 4;
                        let hook = r32(c, V0);
                        call_ptr(c, hook, 0x0024_0220)?;
                        if c.g[V0][0] != 0 {
                            break 'set_1e;
                        }
                        c.cycles += 7;
                        mcus_load_card(c, r32(c, S0), A2);
                        c.g[A1][0] = mcus_at(r32(c, S0), mcus::FILE_CURSOR).lw(c);
                        mcus_card_call(c, A2, V0, V1, 0x140, 0x0024_0244)?;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_FINISH_LOAD.call(c, 0x0024_024c)?;
                    frame.restore(c, S0, 0x10);
                    mc_state20_epilogue(c, frame);
                    return Ok(());
                }
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_CARD_SET_STATE_1E.call(c, 0x0024_025c)?;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                c.g[A1][0] = 0x21;
                mc_check_report(c, S0, 0x0024_0274)?;
                frame.restore(c, S0, 0x10);
                mc_state20_epilogue(c, frame);
                return Ok(());
            }
            // a1 = the backend: finished by now?
            c.cycles += 7;
            mcus_card_call(c, A1, V1, V0, 0x248, 0x0024_0290)?;
            if c.g[V0][0] != 0 {
                c.cycles += 10;
                c.g[V0][0] = mcus_at(r32(c, S0), 0x2c).lw(c);
                let hook = r32(c, V0);
                call_ptr(c, hook, 0x0024_02a4)?;
                c.g[V1][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
                c.g[A1][0] = 0x24;
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0024_02bc)?;
            }
        }
        frame.restore(c, S0, 0x10);
        mc_state20_epilogue(c, frame);
        Ok(())
    })
}

/// [`mc_manager_state20_load`]'s epilogue: `ra`, pop the 0x20 frame, return.
pub(crate) fn mc_state20_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// Reload `s0` and leave through the 0x50 frame's shared epilogue.
pub(crate) fn mc_state1d_leave(c: &mut Cpu, frame: Frame) -> Flow {
    frame.restore(c, S0, 0x40);
    mcus_epilogue(c, frame);
    Ok(())
}

/// The shared report tail (0x240578): charge `cycles` for the branch that
/// got here, load the sink from `+0x748`, post `a1` and leave.
pub(crate) fn mc_state1d_report(c: &mut Cpu, frame: Frame, cycles: u64) -> Flow {
    c.cycles += cycles;
    c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
    c.cycles += 6;
    mc_check_report(c, S0, 0x0024_0588)?;
    mc_state1d_leave(c, frame)
}

/// `void MCManager_State1D_Check(MCManager *self)` (0x2402d0): the card check
/// of state 0x1d. Nothing while the periodic card poll (`+0x338`) runs. `quit`
/// = save or delete pending, or the load-pressed hook. A non-zero
/// `MCManager_PollCardStatus` with the load-pressed hook quiet: status flag 8
/// → `MCManager_SetStateNoCard` when the word at `+0x124`, the load- or
/// delete-pressed hook or `quit` asks; else 0x27 (flag 0x10, word `+0x120`), 5
/// (flag 1), 0x17 (flag 4, word `+0x11c`), or `MCManager_OnCardScanned` as in
/// State03. Unless quitting, a changed card → clear it and
/// `MCManager_ReturnToSlotState`, then flag 8 → post 0x31. Then the hook at
/// `+0xa4`; still in state 0x1d: load pressed → 0x1f with loading allowed, else
/// 0x24/0x25 by driver slots `VT_CAN_SAVE`, `VT_FILE_EXISTS` and 0x208; delete
/// pressed with deleting allowed → the cursor's name into `+0x234`, post 0x18;
/// the hook at `+0x6c` → post 1.
pub fn mc_manager_state1d_check(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = mcus_at(r32(c, S0), mcus::CHECK_CARD).lw(c);
        if c.g[V0][0] != 0 {
            return mc_state1d_leave(c, frame);
        }

        // s2: quit. `+0xbc` and the hook set it in one shared block, `+0xc0`
        // in its branch's delay slot.
        c.cycles += 3;
        c.g[V0][0] = mcus_at(r32(c, S0), mcus::SAVE).lw(c);
        c.g[S2][0] = 0;
        let mut shared_set = c.g[V0][0] != 0;
        let mut quit = shared_set;
        if !quit {
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::DELETE).lw(c);
            if c.g[V0][0] != 0 {
                c.g[S2][0] = 1;
                quit = true;
            } else {
                c.cycles += 5;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CB_LOAD_PRESSED).lw(c);
                call_ptr(c, r32(c, V0), 0x0024_031c)?;
                shared_set = c.g[V0][0] != 0;
                quit = shared_set;
            }
        }
        if shared_set {
            c.cycles += 1;
            c.g[S2][0] = 1;
        }
        if quit {
            c.cycles += 1;
        }
        c.g[S3][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);

        // s1: the card is ready.
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        callee::MC_MANAGER_POLL_CARD_STATUS.call(c, 0x0024_0334)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = mcus_at(r32(c, S0), mcus::CB_LOAD_PRESSED).lw(c);
        call_ptr(c, r32(c, V0), 0x0024_0344)?;
        'card: {
            if c.g[V0][0] != 0 {
                break 'card;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                break 'card;
            }
            c.cycles += 5;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] &= 8;
            if c.g[V0][0] != 0 {
                // Flag 8: no card, unless nothing asks for the no-card state.
                c.cycles += 3;
                c.g[V0][0] = mcus_at(r32(c, S0), 0x124).lw(c);
                let mut no_card = c.g[V0][0] != 0;
                if !no_card {
                    c.cycles += 5;
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::CB_LOAD_PRESSED).lw(c);
                    call_ptr(c, r32(c, V0), 0x0024_0380)?;
                    no_card = c.g[V0][0] != 0;
                }
                if !no_card {
                    c.cycles += 5;
                    c.g[V0][0] = mcus_at(r32(c, S0), mcus::CB_DELETE_PRESSED).lw(c);
                    call_ptr(c, r32(c, V0), 0x0024_0394)?;
                    no_card = c.g[V0][0] != 0;
                }
                if !no_card {
                    c.cycles += 2;
                    no_card = c.g[S2][0] != 0;
                    if !no_card {
                        c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
                    }
                }
                if no_card {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    callee::MC_MANAGER_SET_STATE_NO_CARD.call(c, 0x0024_03ac)?;
                    return mc_state1d_leave(c, frame);
                }
            } else {
                c.g[V0][0] = mcus_at(r32(c, S0), 0x120).lw(c);
                // Word +0x120 and flag 0x10: post 0x27.
                c.cycles += 2;
                let off = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                if !off {
                    c.cycles += 2;
                    let clear = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x27;
                    if !clear {
                        return mc_state1d_report(c, frame, 2);
                    }
                }
                // Flag 1: post 5.
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 1;
                let clear = c.g[V0][0] == 0;
                c.g[A1][0] = 5;
                if !clear {
                    return mc_state1d_report(c, frame, 2);
                }
                // Word +0x11c and flag 4: post 0x17.
                c.cycles += 3;
                c.g[V0][0] = mcus_at(r32(c, S0), 0x11c).lw(c);
                let off = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V1][0] & 4;
                if !off {
                    c.cycles += 2;
                    let clear = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x17;
                    if !clear {
                        return mc_state1d_report(c, frame, 2);
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
            }

            // Rescan when flag 8 went away during the poll, or when the flags
            // are now 0 and the old ones had a flag other than 4.
            c.cycles += 4;
            c.g[V1][0] = c.g[V0][0];
            let no_card_now = c.g[V0][0] & 8 != 0;
            c.g[V0][0] = 0xffff_ffff_ffff_fffb;
            let mut rescan = false;
            if !no_card_now {
                c.cycles += 3;
                rescan = c.g[S3][0] & 8 != 0;
            }
            if !rescan {
                c.cycles += 2;
                let flags_set = c.g[V1][0] != 0;
                c.g[V0][0] = c.g[S3][0] & c.g[V0][0];
                if flags_set {
                    break 'card;
                }
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    break 'card;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::MC_MANAGER_ON_CARD_SCANNED.call(c, 0x0024_0434)?;
            return mc_state1d_leave(c, frame);
        }

        c.cycles += 2;
        if c.g[S2][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_CHANGED).lw(c);
            if c.g[V0][0] != 0 {
                // The card changed: back to the slot state.
                c.cycles += 4;
                mcus_at(r32(c, S0), mcus::CARD_CHANGED).sw(c, 0);
                c.g[A0][0] = c.g[S0][0];
                callee::MC_MANAGER_RETURN_TO_SLOT_STATE.call(c, 0x0024_045c)?;
            }
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::CARD_STATUS).lw(c);
            c.cycles += 3;
            c.g[V0][0] &= 8;
            let clear = c.g[V0][0] == 0;
            c.g[A1][0] = 0x31;
            if !clear {
                return mc_state1d_report(c, frame, 2);
            }
            c.cycles += 1;
        }
        c.g[V1][0] = mcus_at(r32(c, S0), 0xa4).lw(c);
        c.cycles += 6;
        call_ptr(c, r32(c, V1), 0x0024_0480)?;
        c.g[V0][0] = mcus_at(r32(c, S0), mcus::STATE).lw(c) ^ 0x1d;
        if c.g[V0][0] != 0 {
            return mc_state1d_leave(c, frame);
        }

        c.cycles += 5;
        c.g[V0][0] = mcus_at(r32(c, S0), mcus::CB_LOAD_PRESSED).lw(c);
        call_ptr(c, r32(c, V0), 0x0024_049c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::LOAD_OK).lw(c);
            let clear = c.g[V0][0] == 0;
            c.g[A1][0] = 0x1f;
            if !clear {
                return mc_state1d_report(c, frame, 2);
            }
            c.cycles += 8;
            c.g[A1][0] = mcus_at(r32(c, S0), mcus::CARD).lw(c);
            mcus_card_call(c, A1, V1, V0, mcus::VT_CAN_SAVE, 0x0024_04d0)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0x24;
                return mc_state1d_report(c, frame, 3);
            }
            c.g[A2][0] = mcus_at(r32(c, S0), mcus::CARD).lw(c);
            c.cycles += 8;
            c.g[A1][0] = mcus_at(r32(c, S0), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A2, V1, V0, mcus::VT_FILE_EXISTS, 0x0024_04fc)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0x25;
                return mc_state1d_report(c, frame, 3);
            }
            c.g[A2][0] = mcus_at(r32(c, S0), mcus::CARD).lw(c);
            c.cycles += 8;
            c.g[A1][0] = mcus_at(r32(c, S0), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A2, V1, V0, 0x208, 0x0024_0528)?;
            let clear = c.g[V0][0] == 0;
            c.g[A1][0] = 0x25;
            if clear {
                c.cycles += 1;
                return mc_state1d_leave(c, frame);
            }
            return mc_state1d_report(c, frame, 2);
        }

        c.g[V0][0] = mcus_at(r32(c, S0), mcus::CB_DELETE_PRESSED).lw(c);
        c.cycles += 4;
        call_ptr(c, r32(c, V0), 0x0024_0540)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::DELETE_OK).lw(c);
            let clear = c.g[V0][0] == 0;
            c.g[A2][0] = sx(r32(c, S0).wrapping_add(0x234));
            if clear {
                c.cycles += 1;
                return mc_state1d_leave(c, frame);
            }
            c.cycles += 9;
            c.g[A3][0] = mcus_at(r32(c, S0), mcus::CARD).lw(c);
            c.g[A1][0] = mcus_at(r32(c, S0), mcus::FILE_CURSOR).lw(c);
            mcus_card_call(c, A3, V0, V1, MEMCARD_CARD_VT_SLOT_NAME, 0x0024_0570)?;
            c.g[A1][0] = 0x18;
            return mc_state1d_report(c, frame, 0);
        }

        c.g[V0][0] = mcus_at(r32(c, S0), 0x6c).lw(c);
        c.cycles += 4;
        call_ptr(c, r32(c, V0), 0x0024_0598)?;
        let clear = c.g[V0][0] == 0;
        c.g[A1][0] = 1;
        if !clear {
            c.cycles += 5;
            c.g[V0][0] = mcus_at(r32(c, S0), mcus::MSG).lw(c);
            mc_check_report(c, S0, 0x0024_05b4)?;
        }
        c.cycles += 1;
        mc_state1d_leave(c, frame)
    })
}

/// The manager's vtable, which the constructor stores at `+0x748` (its slot 8,
/// `MCManager_v01`, enters a state).
pub(crate) const MC_MANAGER_VTABLE: u32 = 0x0047_cbd8;

/// `MCManager_NullCallback`: the constructor's default hook, does nothing.
pub(crate) const MC_MANAGER_NULL_CALLBACK: u32 = 0x0024_4860;
/// `MCManager_NullCallbackRet0`: default hook that returns 0.
pub(crate) const MC_MANAGER_NULL_CALLBACK_RET0: u32 = 0x0024_4868;
/// `MCManager_NullCallbackRet1`: default hook that returns 1.
pub(crate) const MC_MANAGER_NULL_CALLBACK_RET1: u32 = 0x0024_4870;
/// `MCManager_NullChunkCallback`: default chunk hook that returns 0.
pub(crate) const MC_MANAGER_NULL_CHUNK_CALLBACK: u32 = 0x0024_4878;

/// Hook words the constructor fills after `+0x1c`/`+0x34`, in the compiled
/// order, with the default stub each one gets. Each stub sits in one register
/// (`v0`, `v1`, `a0`, `a1`) for the whole run of stores, so storing the stub
/// is storing that register.
pub(crate) const MC_MANAGER_CTOR_HOOKS: [(u32, u32); 43] = [
    (0x00, MC_MANAGER_NULL_CALLBACK),
    (0x04, MC_MANAGER_NULL_CHUNK_CALLBACK),
    (0x08, MC_MANAGER_NULL_CALLBACK),
    (0x0c, MC_MANAGER_NULL_CALLBACK),
    (0x10, MC_MANAGER_NULL_CALLBACK),
    (0x14, MC_MANAGER_NULL_CALLBACK),
    (0x18, MC_MANAGER_NULL_CALLBACK),
    (0x20, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x24, MC_MANAGER_NULL_CALLBACK),
    (0x28, MC_MANAGER_NULL_CALLBACK),
    (0x2c, MC_MANAGER_NULL_CALLBACK),
    (0x30, MC_MANAGER_NULL_CALLBACK_RET1),
    (0x38, MC_MANAGER_NULL_CALLBACK),
    (0x3c, MC_MANAGER_NULL_CALLBACK),
    (0x40, MC_MANAGER_NULL_CALLBACK),
    (0x44, MC_MANAGER_NULL_CALLBACK),
    (0x48, MC_MANAGER_NULL_CALLBACK),
    (0x4c, MC_MANAGER_NULL_CALLBACK),
    (0x50, MC_MANAGER_NULL_CALLBACK),
    (0x54, MC_MANAGER_NULL_CALLBACK),
    (0x58, MC_MANAGER_NULL_CALLBACK),
    (0x5c, MC_MANAGER_NULL_CALLBACK),
    (0x60, MC_MANAGER_NULL_CALLBACK),
    (0x64, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x68, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x6c, MC_MANAGER_NULL_CALLBACK_RET0),
    (0xac, MC_MANAGER_NULL_CALLBACK),
    (0xb0, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x70, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x74, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x78, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x7c, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x80, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x84, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x88, MC_MANAGER_NULL_CALLBACK_RET0),
    (0x8c, MC_MANAGER_NULL_CALLBACK),
    (0x90, MC_MANAGER_NULL_CALLBACK),
    (0x94, MC_MANAGER_NULL_CALLBACK),
    (0x98, MC_MANAGER_NULL_CALLBACK),
    (0x9c, MC_MANAGER_NULL_CALLBACK),
    (0xa0, MC_MANAGER_NULL_CALLBACK),
    (0xa4, MC_MANAGER_NULL_CALLBACK),
    (0xa8, MC_MANAGER_NULL_CALLBACK),
];

/// `MCManager *MCManager_ctor(MCManager *self)` (0x242288): set up a new
/// memory-card manager and return it. Clears the save/delete pending words and
/// `+0xc4`, sets the save mode to 2 and `+0x748` to the manager's vtable,
/// enters mode 1 (`MCManager_SetMode`), then clears the state, the file
/// cursor, the screen's offer words and most other words up to `+0x12c`; sets
/// `+0xfc`, `+0x424` and `+0x444` to -1, `+0x128` to 1 and the ramp floats to
/// 1.0 and the constant at `gp-0x5120`; clears the first device record (the
/// compiled one-pass loop clears it again), the 0x42 bytes at `+0x234` and the
/// slot, card and check words, sets the check ticks to 5, clears the word at
/// `gp-0x478`, and points the 45 hook words `+0x00`..`+0xb0` at the default
/// stubs.
pub fn mc_manager_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 128;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S2, 0x30);
        c.g[V1][0] = 2;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[V0][0] = sx(MC_MANAGER_VTABLE);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S1][0] = 0xffff_ffff_ffff_ffff;
        mcus_at(r32(c, S2), mcus::SAVE).sw(c, 0);
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(mcus::DEVICES));
        mcus_at(r32(c, S2), mcus::DELETE).sw(c, 0);
        c.g[S3][0] = c.g[S0][0];
        mcus_at(r32(c, S2), 0xc4).sw(c, 0);
        c.g[S4][0] = 0;
        mcus_at(r32(c, S2), mcus::SAVE_MODE).sw(c, c.g[V1][0]);
        mcus_at(r32(c, S2), mcus::MSG).sw(c, c.g[V0][0]);
        callee::MC_MANAGER_SET_MODE.call(c, 0x0024_22e0)?;

        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.g[V0][0] = 1;
        let k = r32(c, GP).wrapping_sub(0x5120);
        c.lwc1(0, k);
        c.g[A0][0] = c.g[S0][0];
        mcus_at(r32(c, S2), mcus::STATE).sw(c, 0);
        c.g[A1][0] = 0;
        mcus_at(r32(c, S2), mcus::FILE_CURSOR).sw(c, 0);
        c.g[A2][0] = mcus::DEVICE_SIZE;
        mcus_at(r32(c, S2), 0xfc).sw(c, c.g[S1][0]);
        for off in [
            0x10c,
            0x104,
            mc::WAIT,
            mcus::CARD_CHANGED,
            mcus::SAVE_OK,
            mcus::LOAD_OK,
            mcus::DELETE_OK,
            mcus::CANCEL_OK,
            0xe0,
            0xf0,
            0xe8,
            0xe4,
            0xf4,
            0xdc,
            0xec,
        ] {
            mcus_at(r32(c, S2), off).sw(c, 0);
        }
        mcus_at(r32(c, S2), 0x424).sw(c, c.g[S1][0]);
        for off in [0x100, 0x12c, 0x118, 0x11c, 0x120, 0x124] {
            mcus_at(r32(c, S2), off).sw(c, 0);
        }
        mcus_at(r32(c, S2), 0x128).sw(c, c.g[V0][0]);
        let at = mcus_at(r32(c, S2), mcus::RAMP).0;
        c.swc1(1, at);
        let at = mcus_at(r32(c, S2), mcus::RAMP_STEP).0;
        c.swc1(0, at);
        // EA_MemSet(&self->+0x344, 0, 0xe0)
        call_ptr(c, callee::EA_MEM_SET.addr, 0x0024_2370)?;

        mcus_at(r32(c, S2), 0x444).sw(c, c.g[S1][0]);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x234));
        mcus_at(r32(c, S2), mcus::SELECTED).sw(c, 0);
        c.g[A1][0] = 0;
        mcus_at(r32(c, S2), mcus::CURR_CHECK).sw(c, 0);
        c.g[A2][0] = 0x42;
        mcus_at(r32(c, S2), mcus::CARD_STATUS).sw(c, 0);
        mcus_at(r32(c, S2), 0x440).sw(c, 0);
        // EA_MemSet(&self->+0x234, 0, 0x42)
        call_ptr(c, callee::EA_MEM_SET.addr, 0x0024_2394)?;

        c.g[V1][0] = 5;
        mcus_at(r32(c, S2), mcus::CARD).sw(c, 0);
        mcus_at(r32(c, S2), mcus::CHECK_TICKS).sw(c, c.g[V1][0]);
        c.g[V0][0] = 0x24_0000;
        mcus_at(r32(c, S2), 0x340).sw(c, 0);
        c.g[A0][0] = 0x24_0000;
        mcus_at(r32(c, S2), mcus::CHECK_CARD).sw(c, 0);
        c.g[V1][0] = 0x24_0000;
        c.g[A1][0] = 0x24_0000;
        c.g[V0][0] = sx(MC_MANAGER_NULL_CALLBACK);
        Addr(r32(c, GP).wrapping_sub(mc_v01::GP_FLAG)).sw(c, 0);
        c.g[V1][0] = sx(MC_MANAGER_NULL_CALLBACK_RET0);
        c.g[A0][0] = sx(MC_MANAGER_NULL_CHUNK_CALLBACK);
        c.g[A1][0] = sx(MC_MANAGER_NULL_CALLBACK_RET1);
        mcus_at(r32(c, S2), 0x1c).sw(c, c.g[A0][0]);
        mcus_at(r32(c, S2), 0x34).sw(c, c.g[A1][0]);
        mcus_at(r32(c, S2), 0x438).sw(c, 0);
        for (off, stub) in MC_MANAGER_CTOR_HOOKS {
            mcus_at(r32(c, S2), off).sw(c, sx(stub));
        }
        c.g[A0][0] = c.g[S3][0];

        // The compiled slot loop (0x242488): its counter s4 starts at 0, so
        // it runs once and its back edge is never taken. It clears the slot at
        // +0x344 again (a0 still holds it; s3 moves on to +0x424 first).
        c.cycles += 7;
        c.g[A1][0] = 0;
        c.g[A2][0] = mcus::DEVICE_SIZE;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(mcus::DEVICE_SIZE as u32));
        call_ptr(c, callee::EA_MEM_SET.addr, 0x0024_2498)?;
        c.g[S4][0] = sx(r32(c, S4).wrapping_sub(1));
        c.g[A0][0] = c.g[S3][0];

        c.cycles += 9;
        c.g[V0][0] = c.g[S2][0];
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

    /// The manager, its card driver object, the driver's vtable and the
    /// message vtable.
    pub(crate) const MCUS_M: u32 = DATA + 0x1_0000;

    pub(crate) const MCUS_CARD: u32 = DATA + 0x2_0000;

    pub(crate) const MCUS_VT: u32 = DATA + 0x2_1000;

    pub(crate) const MCUS_MSG_VT: u32 = DATA + 0x2_2000;

    /// Real leaves: `return 0` and `return 1`.
    pub(crate) const MCUS_RET_ZERO: u32 = 0x001a_8828;

    pub(crate) const MCUS_RET_ONE: u32 = 0x001c_5dd0;

    /// Case blocks whose handlers stay short on this fixture (State13, 23,
    /// 2A, 2D, 2E, 37, 1B, 38), the no-handler target and an address outside
    /// the function (the `jump_out` path).
    pub(crate) const MCUS_TEST_BLOCKS: [u32; 10] = [
        0x0023_dfe0,
        0x0023_e0a0,
        0x0023_e1c0,
        0x0023_e1f0,
        0x0023_e200,
        0x0023_e230,
        0x0023_e160,
        0x0023_e240,
        0x0023_e248,
        0x0000_5000,
    ];

    /// States whose real jump-table entry is one of those blocks (8 has no
    /// handler), or outside the table.
    pub(crate) const MCUS_REAL_TABLE_STATES: [u32; 13] = [
        8,
        0x13,
        0x23,
        0x2a,
        0x2d,
        0x2e,
        0x37,
        0x1b,
        0x38,
        0,
        1,
        0x39,
        0xffff_ffff,
    ];

    /// States the earlier sections test for.
    pub(crate) const MCUS_BRANCH_STATES: [u32; 20] = [
        7, 0x1d, 0x14, 0x24, 0x25, 0x1c, 0xc, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x16, 4, 0x17,
        0x18, 9, 0xa, 2,
    ];

    /// The game's jump table for the switch (55 words from the ELF).
    pub(crate) fn mcus_real_table() -> &'static [u32] {
        static TABLE: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
        TABLE.get_or_init(|| {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../orig/SLUS_207.72");
            let d = std::fs::read(path).expect("orig/SLUS_207.72");
            let u32_at = |o: usize| u32::from_le_bytes(d[o..o + 4].try_into().unwrap());
            let ph = u32_at(0x1c) as usize;
            let (off, vaddr) = (u32_at(ph + 4) as usize, u32_at(ph + 8));
            let base = off + (mcus::JUMP_TABLE - vaddr) as usize;
            (0..55).map(|i| u32_at(base + 4 * i)).collect()
        })
    }

    /// A method result: 0, 1 or a (logged) fake.
    pub(crate) fn mcus_pick(r: &mut Rng) -> u32 {
        match r.next_u32() % 3 {
            0 => MCUS_RET_ZERO,
            1 => MCUS_RET_ONE,
            _ => FAKE,
        }
    }

    /// An input callback: mostly "not pressed".
    pub(crate) fn mcus_pick_input(r: &mut Rng) -> u32 {
        if r.next_u32() % 3 != 0 {
            MCUS_RET_ZERO
        } else {
            mcus_pick(r)
        }
    }

    pub(crate) fn mcus_slot(c: &mut Cpu, vt: u32, off: u32, delta: u32, target: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, target);
    }

    /// `MCManager_UpdateStates` against the generated body: every section's
    /// branches (busy/idle card, the state chains, the slot refresh, the
    /// next/previous inputs, the slot scan) and the switch through the real
    /// jump table (for states whose handlers stay short) or a test table of
    /// short handlers, the no-handler entry and an outside address.
    #[test]
    fn mc_manager_update_states_matches() {
        fakes::covered(
            ee::s06::f_0023d7d8__generated,
            mc_manager_update_states,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0023_d7d8, 24);
                c.pc = 0x0023_d7d8;
                let m = MCUS_M;
                c.g[A0][0] = sx(m);

                // The card driver: every slot a fake, then the ones this
                // function (and its short callees) branch on.
                c.bus.write32(m + mcus::CARD, MCUS_CARD);
                c.bus.write32(MCUS_CARD, MCUS_VT);
                for off in (0..0x240).step_by(8) {
                    let delta = r.next_u32() & 0x1c;
                    mcus_slot(c, MCUS_VT, off, delta, FAKE);
                }
                let max_devices = if seed % 3 == 0 {
                    (r.next_u32() % 3 + 1) as i32
                } else {
                    -1
                };
                for off in [
                    mcus::VT_CAN_SAVE,
                    0x208,
                    0x210,
                    mcus::VT_CAN_LOAD,
                    0x1d0,
                    0x198,
                    0x1d8,
                    mcus::VT_FILE_EXISTS,
                    mcus::VT_CARD_CHANGED,
                    mcus::VT_DEVICE_PRESENT,
                ] {
                    let t = mcus_pick(&mut r);
                    mcus_slot(c, MCUS_VT, off, 0, t);
                }
                let busy = if r.next_u32() % 2 == 0 {
                    MCUS_RET_ZERO
                } else {
                    mcus_pick(&mut r)
                };
                mcus_slot(c, MCUS_VT, mcus::VT_BUSY, 0, busy);
                // With a known slot count the slot stepping searches: keep it
                // finding a slot at once.
                let device_ok = if max_devices >= 0 {
                    if r.next_u32() % 2 == 0 {
                        MCUS_RET_ONE
                    } else {
                        FAKE
                    }
                } else {
                    mcus_pick(&mut r)
                };
                mcus_slot(c, MCUS_VT, mcus::VT_DEVICE_OK, 0, device_ok);
                c.bus.write32(m + mcus::MSG, MCUS_MSG_VT);
                mcus_slot(c, MCUS_MSG_VT, 8, r.next_u32() & 0x1c, FAKE);

                // Input callbacks and flags.
                for cb in [
                    mcus::CB_SAVE_PRESSED,
                    mcus::CB_DELETE_PRESSED,
                    mcus::CB_DEVICE_SELECTED,
                    mcus::CB_LOAD_PRESSED,
                    mcus::CB_NEXT_DEVICE,
                    mcus::CB_PREV_DEVICE,
                    mcus::CB_FORMAT_PRESSED,
                    0xac,
                    0x30,
                ] {
                    let t = mcus_pick_input(&mut r);
                    c.bus.write32(m + cb, t);
                }
                c.bus.write32(m + 0x6c, MCUS_RET_ZERO);
                for flag in [mcus::SAVE, mcus::DELETE] {
                    let v = if r.next_u32() % 3 == 0 { 1 } else { 0 };
                    c.bus.write32(m + flag, v);
                }
                c.bus.write32(m + mcus::FILE_CURSOR, r.next_u32() % 6);
                c.bus.write32(m + mcus::CHECK_CARD, r.next_u32() % 2);
                c.bus
                    .write32(m + mcus::CHECK_TICKS, (r.next_u32() % 5).wrapping_sub(1));
                c.bus.write32(m + 0x424, max_devices as u32);
                c.bus.write32(m + mcus::SELECTED, r.next_u32() % 3);
                c.bus
                    .write32(m + mcus::LAST_CHECK, (r.next_u32() % 4).wrapping_sub(1));
                for i in -1i32..4 {
                    let at = m
                        .wrapping_add(mcus::DEVICE_PLUGGED_IN)
                        .wrapping_add((i * 0xe0) as u32);
                    c.bus.write32(at, r.next_u32() % 2);
                }
                c.bus.write32(m + 0x100, r.next_u32() % 2);
                c.bus.write32(m + 0x444, r.next_u32() % 3);

                // The state and the switch's table.
                let real = seed % 4 == 0;
                let state = if real {
                    MCUS_REAL_TABLE_STATES[(r.next_u32() % 13) as usize]
                } else {
                    MCUS_BRANCH_STATES[(r.next_u32() % 20) as usize]
                };
                c.bus.write32(m + mcus::STATE, state);
                for i in 0..55u32 {
                    let entry = if real {
                        mcus_real_table()[i as usize]
                    } else {
                        MCUS_TEST_BLOCKS[(r.next_u32() % 10) as usize]
                    };
                    c.bus.write32(mcus::JUMP_TABLE + 4 * i, entry);
                }
            },
        );
    }
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    /// Zeroed manager at `DATA+0x400` with a backend holder/vtable and a
    /// crowd-manager vtable, like the movieplayer `mc_setup_2` fixture but
    /// with the first poll result steered by `poll` (`RET_ZERO` goes on to
    /// poll, `FAKE` reports finished).
    pub(crate) fn mgr(c: &mut Cpu, seed: u32, salt: u64, poll: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        globals(c);
        let this = MC_FIXTURE_THIS;
        zero(c, this, 0x800);
        let holder = DATA + 0x1000;
        let vt = MC_FIXTURE_CARD_VT;
        w32(c, this + mcus::CARD, holder);
        w32(c, holder, vt);
        ret_slot(c, vt, mcus::VT_BUSY, poll);
        let crowd = DATA + 0x1200;
        w32(c, this + mcus::MSG, crowd);
        ret_slot(c, crowd, 8, FAKE);
        ret_slot(c, crowd, 0xc, FAKE);
        c.g[A0][0] = sx(this);
        r
    }

    #[test]
    fn mc_manager_state32_matches() {
        check(
            ee::s06::f_0023e2c0__generated,
            mc_manager_state32,
            32,
            &|c, seed| {
                let mut r = Rng(0x23e2_c0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn mc_manager_state34_matches() {
        check(
            ee::s06::f_0023e4f0__generated,
            mc_manager_state34,
            32,
            &|c, seed| {
                let mut r = Rng(0x23e4_f0 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn mc_manager_set_slot_matches() {
        check(
            ee::s06::f_0023fb18__generated,
            mc_manager_set_slot,
            32,
            &|c, seed| {
                let mut r = Rng(0x23fb_18 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 7,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn mc_manager_set_data_size_wrapper_matches() {
        check(
            ee::s06::f_0023cc58__generated,
            mc_manager_set_data_size_wrapper,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0xcc58, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 0x200,
                    _ => 0x8000_0000,
                };
            },
        );
    }

    #[test]
    fn mc_manager_state2f_matches() {
        check(
            ee::s06::f_00240f10__generated,
            mc_manager_state2f,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x40f10, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_read_current_dir_info_matches() {
        check(
            ee::s06::f_00241fb0__generated,
            mc_manager_read_current_dir_info,
            40,
            &|c, seed| {
                let _ = mgr(c, seed, 0x41fb0, FAKE);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x42c,
                    match seed % 4 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 5,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_get_instance_matches() {
        use crate::game::dirtysock::tagsunk::tests::alloc_ready;
        use crate::game::movie::movieplayer::tests::allocator_mutex;
        check(
            ee::s06::f_0023c808__generated,
            mc_manager_get_instance,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23c808, 0);
                alloc_ready(c);
                allocator_mutex(c);
                // Null slot takes the create path, set slot returns it.
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x47c),
                    match seed % 3 {
                        0 => 0,
                        1 => DATA + 0x500,
                        _ => r.next_u32() & 0x00ff_ffff,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_set_data_size_matches() {
        check(
            ee::s06::f_0023caa0__generated,
            mc_manager_set_data_size,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0xcaa0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 0x200,
                    _ => 0x8000_0000,
                };
            },
        );
    }

    #[test]
    fn mc_manager_prev_file_matches() {
        check(
            ee::s06::f_00241678__generated,
            mc_manager_prev_file,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x41678, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0xf8, (seed % 5) as u32);
                w32(c, this + 0x100, if seed % 3 == 0 { 0 } else { 1 });
                w32(c, this + 0xec, (seed % 4) as u32);
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x220, FAKE);
            },
        );
    }

    #[test]
    fn mc_manager_next_file_matches() {
        check(
            ee::s06::f_002416c0__generated,
            mc_manager_next_file,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x416c0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0xf8, (seed % 5) as u32);
                w32(c, this + 0x100, if seed % 3 == 0 { 0 } else { 1 });
                w32(c, this + 0xec, (seed % 4) as u32);
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                // A small clamped top reaches the at-top wrap to 0.
                ret_slot(
                    c,
                    DATA + 0x1100,
                    0x220,
                    match seed % 3 {
                        0 => RET_ZERO,
                        1 => RET_ONE,
                        _ => FAKE,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_write_save_by_name_matches() {
        check(
            ee::s06::f_00240fb0__generated,
            mc_manager_write_save_by_name,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x40fb0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x120, FAKE);
                c.g[A3][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0xabcd,
                };
            },
        );
    }

    #[test]
    fn mc_manager_write_save_by_slot_matches() {
        check(
            ee::s06::f_00241050__generated,
            mc_manager_write_save_by_slot,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x41050, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x130, FAKE);
                c.g[A3][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0xabcd,
                };
            },
        );
    }

    #[test]
    fn mc_manager_state31_matches() {
        check(
            ee::s06::f_0023e268__generated,
            mc_manager_state31,
            64,
            &|c, seed| {
                let poll = if seed % 2 == 0 { RET_ZERO } else { FAKE };
                let _ = mgr(c, seed, 0x23e268, poll);
                c.pc = 0x0023_e268;
            },
        );
    }

    #[test]
    fn mc_manager_clamp_file_index_matches() {
        check(
            ee::s06::f_002410d0__generated,
            mc_manager_clamp_file_index,
            48,
            &|c, seed| {
                let _ = mgr(c, seed, 0x410d0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0xec, (seed % 5) as u32);
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                // A count at or below +0xec reaches the second backend call.
                ret_slot(
                    c,
                    DATA + 0x1100,
                    0x220,
                    match seed % 3 {
                        0 => RET_ZERO,
                        1 => RET_ONE,
                        _ => FAKE,
                    },
                );
            },
        );
    }

    #[test]
    fn mc_manager_state0d_matches() {
        check(
            ee::s06::f_0023f308__generated,
            mc_manager_state0d,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23f308, FAKE);
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                w32(c, this + 0xf8, (seed % 3) as u32);
                // First poll done or not, second poll done or not.
                ret_slot(c, vt, 0x218, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x208, if seed % 4 == 0 { RET_ZERO } else { FAKE });
            },
        );
    }

    #[test]
    fn mc_manager_format_slot_free_space_matches() {
        check(
            ee::s06::f_00241928__generated,
            mc_manager_format_slot_free_space,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x41928, 0);
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
                c.bus.write32(dev + 0x354, DATA + 0x3000);
                for i in (0..0x40).step_by(4) {
                    c.bus.write32(DATA + 0x3000 + i, r.next_u32());
                }
                for i in (0..0x100).step_by(4) {
                    c.bus.write32(dst + i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn mc_manager_state1e_matches() {
        check(
            ee::s06::f_0023fff0__generated,
            mc_manager_state1e,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23fff0,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                // RET_ZERO hooks make the "done" branches run (a FAKE hook
                // can never return 0).
                w32(
                    c,
                    DATA + 0x400 + 0x78,
                    if seed % 8 == 2 { RET_ZERO } else { FAKE },
                );
                w32(
                    c,
                    DATA + 0x400 + 0x6c,
                    if seed % 16 == 2 { RET_ZERO } else { FAKE },
                );
                c.pc = 0x0023_fff0;
            },
        );
    }

    #[test]
    fn mc_manager_state2b_matches() {
        check(
            ee::s06::f_00240cc0__generated,
            mc_manager_state2b,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x40cc0,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x28, FAKE);
                w32(
                    c,
                    DATA + 0x400 + 0x68,
                    if seed % 8 == 2 { RET_ZERO } else { FAKE },
                );
                w32(
                    c,
                    DATA + 0x400 + 0x6c,
                    if seed % 16 == 2 { RET_ZERO } else { FAKE },
                );
                w32(c, DATA + 0x400 + 0x428, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn mc_manager_state09_matches() {
        check(
            ee::s06::f_002409c8__generated,
            mc_manager_state09,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x409c8,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x248, if seed % 8 == 2 { RET_ZERO } else { FAKE });
                // Different moduli for the two tail hooks: seed%32==2 keeps
                // both at 0 (quiet tail), seed%32==18 leaves +0x6c non-zero
                // (report 1), seed%16==10 leaves +0x70 non-zero with the
                // same poll (report 0xd).
                w32(
                    c,
                    DATA + 0x400 + 0x70,
                    if seed % 32 == 2 || seed % 32 == 18 {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                w32(
                    c,
                    DATA + 0x400 + 0x6c,
                    if seed % 32 == 2 { RET_ZERO } else { FAKE },
                );
            },
        );
    }

    #[test]
    fn mc_manager_state11_save_matches() {
        check(
            ee::s06::f_0023f578__generated,
            mc_manager_state11_save,
            64,
            &|c, seed| {
                let mut r = mgr(c, seed, 0x23f578, FAKE);
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x248, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                // RET_ONE at +0x150 reaches the == 1 branch (CardSetState18,
                // the +4 hook and both report calls below it).
                ret_slot(
                    c,
                    vt,
                    0x150,
                    match seed % 4 {
                        0 => RET_ZERO,
                        1 => RET_ONE,
                        _ => FAKE,
                    },
                );
                w32(c, this + 0x108, (seed % 3) as u32);
                // seed%16==1: e4 >= f0 (the CardSetState18 path);
                // seed%16==5: e4 < f0 (the +4 hook path, hook returns 0);
                // seed%16==13: e4 < f0 with the hook non-zero (early return);
                // seed%16==9: e4 == f0 (progress at its total, the
                // CardSetState18 path).
                let (e4, f0) = if seed % 16 == 1 {
                    (5, 3)
                } else if seed % 16 == 9 {
                    (4, 4)
                } else if seed % 16 == 5 || seed % 16 == 13 {
                    (3, 5)
                } else {
                    (r.next_u32(), r.next_u32())
                };
                c.bus.write32(this + 0xe4, e4);
                c.bus.write32(this + 0xf0, f0);
                w32(c, this + 0x4, if seed % 16 == 5 { RET_ZERO } else { FAKE });
            },
        );
    }

    #[test]
    fn mc_manager_state2c_delete_matches() {
        check(
            ee::s06::f_00240d90__generated,
            mc_manager_state2c_delete,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x40d90,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                // Per seed%16 (even seeds pass the first poll): 0 countdown
                // still running, 2 report 0x2d, 4 report 0x2e (RET_ZERO
                // count), 6 the 0x208 == 0 skip, 8/10 the 0x208 != 0 arm
                // with the slot-0x50 call.
                let (s248, s68, s208, n108, n438) = match seed % 16 {
                    0 => (FAKE, FAKE, FAKE, 2, 0),
                    2 => (FAKE, FAKE, FAKE, 0, 0),
                    4 => (RET_ZERO, RET_ZERO, FAKE, 0, 0),
                    6 => (RET_ZERO, FAKE, RET_ZERO, 1, 1),
                    8 => (RET_ZERO, FAKE, FAKE, 0, 2),
                    10 => (RET_ZERO, FAKE, FAKE, 1, 0),
                    _ => (
                        if seed % 4 < 2 { RET_ZERO } else { FAKE },
                        FAKE,
                        FAKE,
                        (seed % 3) as u32,
                        (seed % 3) as u32,
                    ),
                };
                ret_slot(c, vt, 0x248, s248);
                ret_slot(c, vt, 0x208, s208);
                ret_slot(c, vt, 0x68, s68);
                ret_slot(c, vt, 0x50, FAKE);
                w32(c, this + 0x108, n108);
                w32(c, this + 0x438, n438);
            },
        );
    }

    #[test]
    fn mc_manager_state16_matches() {
        check(
            ee::s06::f_0023fd40__generated,
            mc_manager_state16,
            64,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23fd40,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(c, vt, 0x1a8, if seed % 4 < 2 { RET_ZERO } else { FAKE });
                ret_slot(c, vt, 0x70, FAKE);
                w32(c, this + 0x338, if seed % 3 == 0 { 0 } else { 1 });
                // RET_ZERO hooks make the "done" chain under `done` run.
                // seed%16==10 stops at +0x84 (OnOperationDone); seed%16==14
                // leaves +0x68 non-zero (the +0x70 call, report 0x19).
                w32(
                    c,
                    this + 0x6c,
                    if matches!(seed % 16, 2 | 6 | 10 | 14) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                w32(
                    c,
                    this + 0x84,
                    if matches!(seed % 16, 2 | 6 | 14) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                w32(
                    c,
                    this + 0x68,
                    if matches!(seed % 16, 2 | 6) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                w32(c, this + 0x70, if seed % 8 == 2 { FAKE } else { RET_ZERO });
            },
        );
    }

    #[test]
    fn mc_manager_begin_save_matches() {
        check(
            ee::s06::f_0023cae8__generated,
            mc_manager_begin_save,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23cae8, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0x200,
                };
            },
        );
    }

    driven!(gen_prev_slot, ee::s06::f_00241540__generated);
    driven!(hand_prev_slot, mc_manager_prev_slot);

    #[test]
    fn mc_manager_prev_slot_matches() {
        check_events(gen_prev_slot, hand_prev_slot, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x41540, 0);
            globals(c);
            let this = DATA + 0x400;
            zero(c, this, 0x800);
            let holder = DATA + 0x1000;
            let vt = DATA + 0x1100;
            w32(c, this + 0x434, holder);
            w32(c, holder, vt);
            // (idx, count, list) restricted to combos that terminate under an
            // always-zero hasChapter (verified with a stage-level simulator;
            // other combos spin forever even in the generated body).
            // hasChapter returns 0 on even seeds (loop + events) and FAKE on
            // odd seeds (immediate exit).
            let (idx, count, list) = match seed % 8 {
                0 => (0, -1, 0),
                1 => (0, 2, 1),
                2 => (1, 2, 0),
                3 => (0, 3, 1),
                4 => (1, 4, 0),
                5 => (2, 4, 0),
                6 => (3, 4, 0),
                _ => (1, 3, 1),
            };
            ret_slot(c, vt, 0x1a0, if seed % 2 == 0 { RET_ZERO } else { FAKE });
            w32(c, this + 0x428, idx as u32);
            w32(
                c,
                this + 0x424,
                if count < 0 { 0xffff_ffff } else { count as u32 },
            );
            w32(c, this + 0x100, list as u32);
            c.g[A0][0] = sx(this);
            schedule(c, &mut r, seed, 0x0024_1540)
        });
    }

    #[test]
    fn mc_manager_begin_load_matches() {
        check(
            ee::s06::f_0023c8f0__generated,
            mc_manager_begin_load,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23c8f0, FAKE);
                let this = DATA + 0x400;
                w32(c, this + 0x434, DATA + 0x1000);
                w32(c, DATA + 0x1000, DATA + 0x1100);
                ret_slot(c, DATA + 0x1100, 0x98, FAKE);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_format_save_desc_matches() {
        check(
            ee::s06::f_0023cc78__generated,
            mc_manager_format_save_desc,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23cc78, FAKE);
                // The body switches on self+0xb4, not on the mode argument.
                w32(c, DATA + 0x400 + 0xb4, seed % 4);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x5000);
                zero(c, DATA + 0x5000, 0x200);
            },
        );
    }

    driven!(gen_next_slot, ee::s06::f_00241400__generated);
    driven!(hand_next_slot, mc_manager_next_slot);

    #[test]
    fn mc_manager_next_slot_matches() {
        check_events(gen_next_slot, hand_next_slot, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x41400, 0);
            globals(c);
            let this = DATA + 0x400;
            zero(c, this, 0x800);
            let holder = DATA + 0x1000;
            let vt = DATA + 0x1100;
            w32(c, this + 0x434, holder);
            w32(c, holder, vt);
            // (idx, count, list) restricted to combos that terminate under an
            // always-zero hasChapter (verified with a stage-level simulator;
            // other combos spin forever even in the generated body).
            // hasChapter returns 0 on even seeds (loop + events) and FAKE on
            // odd seeds (immediate exit).
            let (idx, count, list) = match seed % 8 {
                0 => (0, -1, 0),
                1 => (0, 2, 1),
                2 => (1, 2, 0),
                3 => (0, 3, 1),
                4 => (1, 4, 0),
                5 => (2, 4, 0),
                6 => (3, 4, 0),
                _ => (1, 3, 1),
            };
            ret_slot(c, vt, 0x1a0, if seed % 2 == 0 { RET_ZERO } else { FAKE });
            w32(c, this + 0x428, idx as u32);
            w32(
                c,
                this + 0x424,
                if count < 0 { 0xffff_ffff } else { count as u32 },
            );
            w32(c, this + 0x100, list as u32);
            c.g[A0][0] = sx(this);
            schedule(c, &mut r, seed, 0x0024_1400)
        });
    }

    #[test]
    fn mc_manager_update_mode1_matches() {
        // One digit per steering value: seed%2 the first poll, seed/2%2 the
        // slot-0x58 result, seed/4%2 the +0x338 reload flag, seed/8%2 a -1
        // slot count at +0x424 (the GetNumSlots branch) and seed/16%6 the
        // mode word at +0x130 (every compared mode, 0x34 posting nothing,
        // and one that matches none).
        const SEEDS: u32 = 96;
        assert_eq!(
            SEEDS,
            2 * 2 * 2 * 2 * 6,
            "poll x slot 0x58 x reload x count x mode"
        );
        check(
            ee::s06::f_0023d660__generated,
            mc_manager_update_mode1,
            SEEDS,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23d660,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let this = DATA + 0x400;
                let vt = DATA + 0x1100;
                ret_slot(
                    c,
                    vt,
                    0x58,
                    if (seed / 2) % 2 == 0 { RET_ZERO } else { FAKE },
                );
                ret_slot(c, vt, 0x18, FAKE);
                w32(c, this + 0x338, (seed / 4) % 2);
                w32(
                    c,
                    this + 0x424,
                    if (seed / 8) % 2 == 0 { 2 } else { 0xffff_ffff },
                );
                w32(
                    c,
                    this + 0x130,
                    [0x31, 0x32, 0x33, 0x34, 0x36, 0x40][((seed / 16) % 6) as usize],
                );
            },
        );
    }

    #[test]
    fn mc_manager_build_save_title_matches() {
        check(
            ee::s06::f_0023cdb0__generated,
            mc_manager_build_save_title,
            64,
            &|c, seed| {
                let _ = mgr(c, seed, 0x23cdb0, FAKE);
                ret_slot(c, DATA + 0x1100, 0x168, FAKE);
                // The body switches on self+0xb4 (modes 0, 1 and >= 2).
                w32(c, DATA + 0x400 + 0xb4, seed % 3);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn mc_manager_classify_card_state_matches() {
        check(
            ee::s06::f_0023e8a0__generated,
            mc_manager_classify_card_state,
            80,
            &|c, seed| {
                let _ = mgr(
                    c,
                    seed,
                    0x23e8a0,
                    if seed % 2 == 0 { RET_ZERO } else { FAKE },
                );
                let vt = DATA + 0x1100;
                let this = DATA + 0x400;
                // Steer the cascade per seed%16 so every leaf runs in both
                // chains: busy 0x1b8/0x1d0/0x198/0x1d8 finishing at +0xbc
                // (== 0: report 0x1d, != 0: report 7), busy fall-through
                // (arg 0: quiet, arg != 0: report 6), idle 0x188/0x198/0x1d8
                // (report 5) and idle +0x130 (== 4: quiet, != 4: report 4).
                let combo = seed % 16;
                let busy = combo <= 9 || combo == 15;
                ret_slot(c, vt, 0x1a8, if busy { FAKE } else { RET_ZERO });
                ret_slot(
                    c,
                    vt,
                    0x1b8,
                    if matches!(combo, 2 | 3) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                ret_slot(
                    c,
                    vt,
                    0x1d0,
                    if matches!(combo, 4 | 5) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                ret_slot(
                    c,
                    vt,
                    0x198,
                    if matches!(combo, 0 | 1 | 8 | 9 | 12 | 13 | 14 | 15) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                ret_slot(
                    c,
                    vt,
                    0x1d8,
                    if matches!(combo, 0 | 1 | 13 | 14 | 15) {
                        RET_ZERO
                    } else {
                        FAKE
                    },
                );
                ret_slot(c, vt, 0x188, if combo == 10 { FAKE } else { RET_ZERO });
                w32(
                    c,
                    this + 0xbc,
                    if matches!(combo, 3 | 5 | 7 | 9) { 3 } else { 0 },
                );
                w32(
                    c,
                    this + 0x130,
                    match combo {
                        13 => 4,
                        14 => 2,
                        _ => (seed % 3) as u32,
                    },
                );
                c.g[A1][0] = if combo == 0 || combo == 15 {
                    0
                } else if combo == 1 {
                    1
                } else {
                    (seed % 2) as u64
                };
            },
        );
    }
    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::require_paths;
    use crate::game::fe::fememcardcallbacks::MEMCARD_APP_GP;
    use crate::game::fe::fememcardcallbacks::MEMCARD_APP_TEXT_DB;
    use crate::game::fe::fememcardcallbacks::MEMCARD_TEXT_DB_VT_FIND;

    /// Extra `(state, variant)` cases after states 0..=0x39 (variant 0): the
    /// inner branches of states 3, 0xd, 0xf, 0x32 and 0x33, two more
    /// values outside the table, and state 0x21 with its table word pointing
    /// outside the function (the `jump_out` exit).
    pub(crate) const MCV01_EXTRA: [(u32, u32); 11] = [
        (3, 1),
        (3, 2),
        (0xd, 1),
        (0xd, 2),
        (0xf, 1),
        (0xf, 2),
        (0x32, 1),
        (0x33, 1),
        (0xffff_ffff, 0),
        (0x8000_0000, 0),
        (0x21, 1),
    ];

    /// The block state 0x21's table word names in case `(0x21, 1)`: no
    /// block of the function, so both bodies leave through `jump_out` (the
    /// fakes treat it as a call returning to the caller).
    pub(crate) const MCV01_FOREIGN_BLOCK: u32 = FAKE + 0x800;

    /// Cases per pass: states 0..=0x39 plus [`MCV01_EXTRA`].
    pub(crate) const MCV01_CASES: u32 = 0x3a + MCV01_EXTRA.len() as u32;

    /// `(state, variant)` for case `i` (`seed % MCV01_CASES`).
    pub(crate) fn mcv01_case(i: u32) -> (u32, u32) {
        if i < 0x3a {
            (i, 0)
        } else {
            MCV01_EXTRA[(i - 0x3a) as usize]
        }
    }

    /// The manager fixture (`mgr`) with the game image loaded (the jump table,
    /// the format strings, the `gp` floats), every callback word and driver
    /// slot pointed at `RET_ZERO`/`RET_ONE` (`leaf`: fixed cycle costs) or at
    /// the fakes (calls logged, some unwinding), and every word the function
    /// writes pre-filled non-zero.
    ///
    /// Variants: state 3 busy (0) / idle with a check pending (1) / idle
    /// (2); state 0xd save mode 0/1/2; state 0xf save mode 0 (0) / 2 (1) /
    /// other (2); state 0x32 driver slot 0x1a8 true (0) / false (1); state
    /// 0x33 busy (0) / idle (1).
    pub(crate) fn mcv01_fixture(c: &mut Cpu, seed: u32, salt: u64, leaf: bool) {
        let (state, variant) = mcv01_case(seed % MCV01_CASES);
        let yes = if leaf { RET_ONE } else { FAKE };
        let any = if leaf { RET_ZERO } else { FAKE };
        let mut r = mgr(c, seed, salt, RET_ZERO);
        elf_image(c);
        globals(c);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let crowd = DATA + 0x1200;
        // The seven ramp floats the states load (`gp - 0x5130..=0x5148`; the
        // image has the same word in all seven): a different word each, so a
        // wrong offset shows.
        for i in 0..7 {
            w32(c, GAME_GP - 0x5130 - 4 * i, (r.next_u32() & !7) | i);
        }
        for off in [
            0x28, 0x38, 0x40, 0x50, 0x70, 0x118, 0x138, 0x168, 0x180, 0x190, 0x1a8, 0x1e0,
        ] {
            ret_slot(c, vt, off, any);
        }
        ret_slot(c, crowd, 8, any);
        // The screen callbacks in the first words.
        for off in (0..=0xa8).step_by(4) {
            w32(c, this + off, if leaf { RET_ZERO } else { FAKE + off });
        }
        // Words the states write.
        for off in [
            0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xdc, 0xe4, 0xe8, 0xfc, 0x108, 0x10c, 0x110, 0x114,
            0x118, 0x130, 0x338, 0x42c, 0x438,
        ] {
            w32(c, this + off, r.next_u32() | 2);
        }
        for i in 0..4 {
            w32(
                c,
                this + mcus::DEVICE_PLUGGED_IN + 0xe0 * i,
                r.next_u32() | 2,
            );
        }
        fill(c, &mut r, this + mc_v01::SAVE_NAME, 0x20);
        let top = c.g[SP][0] as u32;
        fill(c, &mut r, top - 0x660, 0x180);
        w32(c, GAME_GP - mc_v01::GP_FLAG, r.next_u32() | 2);
        // Data: the file cursor (printed as a number), the selected slot.
        w32(c, this + mcus::FILE_CURSOR, r.next_u32() % 100);
        w32(c, this + mcus::SELECTED, r.next_u32() % 4);
        match (state, variant) {
            (3, 0) | (0x33, 0) => ret_slot(c, vt, mcus::VT_BUSY, yes),
            (3, 2) => w32(c, this + mcus::CHECK_CARD, 0),
            (0x32, 0) => ret_slot(c, vt, mcus::VT_CAN_SAVE, yes),
            (0x32, 1) => ret_slot(c, vt, mcus::VT_CAN_SAVE, RET_ZERO),
            (0xd, v) => {
                w32(c, this + mcus::SAVE_MODE, v);
                // Save mode 1 reads +0xfc as the player index.
                w32(c, this + 0xfc, 0);
            }
            (0xf, v) => w32(c, this + mcus::SAVE_MODE, [0, 2, 1][v as usize]),
            (0x21, 1) => w32(c, mc_v01::JUMP_TABLE + 4 * (0x21 - 1), MCV01_FOREIGN_BLOCK),
            _ => {}
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (c.g[A1][0] & !0xffff_ffff) | state as u64;
        c.pc = mc_v01::ENTRY;
    }

    /// Every table state, the inner branches and out-of-range states, with
    /// leaf callees so each path has a fixed cycle total. The totals below
    /// are every path except those through the string, directory and title
    /// callees (states 0xd, 0xf, 0x32), computed from the generated blocks
    /// (prologue 12 + dispatch 7, leaf call 2, tails 7/8/9/10):
    ///
    /// | cycles | states |
    /// | --- | --- |
    /// | 21 | out of range (0, 0x39, -1, 0x80000000) |
    /// | 28 | no entry work (7, 8, 0x10, 0x12, 0x1a, 0x1d, 0x1e, 0x21, 0x22, 0x26, 0x2f, 0x35, 0x36) |
    /// | 29 | 0x2d, 0x2e |
    /// | 30 | 0x23 |
    /// | 31 | 0x1b |
    /// | 33 | 0x34 |
    /// | 34 | 0x13, 4, 5, 0x15, 0x16, 0x17, 0x25, 0x30, 0x38 |
    /// | 35 | 0xc, 0x14, 0x20, 0x24, 0x27, 0x29, 0x2a, 0x2b, 0x37 |
    /// | 36 | 0x11, 0x1c |
    /// | 37 | 2, 3 busy |
    /// | 39 | 9, 0x2c |
    /// | 40 | 0xa, 3 check pending, 6, 0x18 |
    /// | 42 | 0xe |
    /// | 43 | 0x19 |
    /// | 46 | 0xb, 0x28, 0x33 busy |
    /// | 50 | 0x33 idle |
    /// | 51 | 1, 0x31, 0x1f |
    /// | 53 | 3 idle |
    ///
    /// The `jump_out` case runs the hook's fake (no fixed cost) and is not
    /// listed. LO/HI are compared too (state 0x32 multiplies).
    #[test]
    fn mc_manager_v01_matches() {
        check_paths_lohi(
            ee::s06::f_0023cf38__generated,
            mc_manager_v01,
            MCV01_CASES,
            &[
                21, 28, 29, 30, 31, 33, 34, 35, 36, 37, 39, 40, 42, 43, 46, 50, 51, 53,
            ],
            &|c, seed| mcv01_fixture(c, seed, 0x23cf38, true),
        );
    }

    /// The same cases with the callbacks and driver slots on the fakes (their
    /// arguments are logged to RAM and compared), each case run with no
    /// unwind, an unwind at the first fake call and one at the second
    /// (`seed / MCV01_CASES % 3`).
    #[test]
    fn mc_manager_v01_calls_match() {
        fakes::covered(
            ee::s06::f_0023cf38__generated,
            mc_manager_v01,
            3 * MCV01_CASES,
            &|c, seed| {
                mcv01_fixture(c, seed, 0x23cf39, false);
                let unwind_at = match seed / MCV01_CASES % 3 {
                    0 => u32::MAX,
                    n => n - 1,
                };
                c.bus.write32(LOG_UNWIND_AT, unwind_at);
            },
        );
    }

    /// `_close` (a newlib stub): a translated leaf returning -1 in 2 cycles.
    pub(crate) const RET_MINUS_ONE: u32 = crate::game::lib::libc::CLOSE.addr;

    /// A translated leaf returning 2 in 2 cycles.
    pub(crate) const RET_TWO: u32 = crate::game::object::movenode::tests::RET_TWO;

    /// The manager and its card driver vtable in the `mgr` fixture.
    pub(crate) const MC_FIXTURE_THIS: u32 = DATA + 0x400;

    pub(crate) const MC_FIXTURE_CARD_VT: u32 = DATA + 0x1100;

    /// The text the lookup fake returns: `"<%s>"` (UTF-16).
    pub(crate) const MC_TEXT_FIXTURE_FORMAT: u32 = DATA + 0x3200;

    /// Write `s` as a NUL-terminated UTF-16 string.
    pub(crate) fn mc_text_fixture_put(c: &mut Cpu, at: u32, s: &str) {
        let units: Vec<u16> = s.encode_utf16().collect();
        for (i, u) in units.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *u);
        }
        c.bus.write16(at + 2 * units.len() as u32, 0);
    }

    /// The application's text database for the card texts: the world's
    /// `+0x8c` points at a database whose find slot (`+0x20` of its vtable,
    /// `this` adjusted by `delta`) is the logging fake, so each lookup logs the
    /// id's hash and returns `"<%s>"`; with `unwind` the lookup unwinds.
    pub(crate) fn mc_text_fixture_db(c: &mut Cpu, delta: u16, unwind: bool) {
        let world = c.bus.read32(GAME_GP.wrapping_add(MEMCARD_APP_GP));
        let db = DATA + 0x3000;
        let vt = DATA + 0x3100;
        w32(c, world + MEMCARD_APP_TEXT_DB, db);
        w32(c, db + 4, vt);
        wh(c, vt + MEMCARD_TEXT_DB_VT_FIND, delta);
        w32(c, vt + MEMCARD_TEXT_DB_VT_FIND + 4, FAKE);
        mc_text_fixture_put(c, MC_TEXT_FIXTURE_FORMAT, "<%s>");
        w32(c, LOG_RESULTS, MC_TEXT_FIXTURE_FORMAT);
        w32(c, LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
    }

    /// The card driver slots `MCManager_GetCardErrorText` asks, in call order.
    pub(crate) const GET_CARD_ERROR_TEXT_SLOTS: [u32; 6] = [
        mcus::VT_DEVICE_OK,
        mcus::VT_NOT_DETECTED_A,
        mcus::VT_NOT_DETECTED_B,
        mcus::VT_NOT_DETECTED_C,
        mcus::VT_FORMATTED,
        mcus::VT_HAS_NAME,
    ];

    /// Their answers per arm (`seed % 8`); slots an arm never reaches answer 0.
    pub(crate) const GET_CARD_ERROR_TEXT_ARMS: [[u32; 6]; 8] = [
        // 0: no device: not detected.
        [RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO],
        // 1..=3: one of the three not-detected slots.
        [RET_ONE, RET_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_ZERO],
        [
            RET_ONE,
            RET_ZERO,
            RET_MINUS_ONE,
            RET_ZERO,
            RET_ZERO,
            RET_ZERO,
        ],
        [RET_ONE, RET_ZERO, RET_ZERO, RET_ONE, RET_ZERO, RET_ZERO],
        // 4: not formatted.
        [
            RET_MINUS_ONE,
            RET_ZERO,
            RET_ZERO,
            RET_ZERO,
            RET_ZERO,
            RET_ZERO,
        ],
        // 5: formatted, no name: `dst` untouched.
        [
            RET_ONE,
            RET_ZERO,
            RET_ZERO,
            RET_ZERO,
            RET_MINUS_ONE,
            RET_ZERO,
        ],
        // 6: an empty name: the memory device text.
        [RET_ONE, RET_ZERO, RET_ZERO, RET_ZERO, RET_ONE, RET_ONE],
        // 7: the name is copied.
        [
            RET_ONE,
            RET_ZERO,
            RET_ZERO,
            RET_ZERO,
            RET_ONE,
            RET_MINUS_ONE,
        ],
    ];

    /// The card error text test's steering: the arm (`seed % 8`), the slot
    /// (`seed / 8 % 3`), the name and the lookup's `this` delta (`seed / 24 %
    /// 2`), the unwind (`seed / 48`: none, or in the leaf pass the text
    /// lookup; in the logged pass call 0, 1 or 2). `logged`: the slots that
    /// answer non-zero are the logging fake (their arguments are compared),
    /// and every fake call answers the format text.
    pub(crate) fn mc_card_error_text_setup(c: &mut Cpu, seed: u32, logged: bool) {
        elf_image(c);
        let mut r = mgr(c, seed, 0x24_1710, RET_ZERO);
        let arm = (seed % 8) as usize;
        for (off, target) in GET_CARD_ERROR_TEXT_SLOTS
            .into_iter()
            .zip(GET_CARD_ERROR_TEXT_ARMS[arm])
        {
            let target = if logged && target != RET_ZERO {
                FAKE
            } else {
                target
            };
            ret_slot(c, MC_FIXTURE_CARD_VT, off, target);
        }
        let slot = [0u32, 2, 3][(seed / 8 % 3) as usize];
        if arm == 7 {
            let name = MC_FIXTURE_THIS + mcus::DEVICE_NAME + slot * mcus::DEVICE_SIZE as u32;
            mc_text_fixture_put(c, name, ["AB", "Card 1"][(seed / 24 % 2) as usize]);
        }
        let unwind = seed / 48;
        mc_text_fixture_db(
            c,
            [0, 0x40][(seed / 24 % 2) as usize],
            !logged && unwind == 1,
        );
        if logged {
            for n in 0..8 {
                w32(c, LOG_RESULTS + 4 * n, MC_TEXT_FIXTURE_FORMAT);
            }
            w32(
                c,
                LOG_UNWIND_AT,
                if unwind == 0 { u32::MAX } else { unwind - 1 },
            );
        }
        let dst = DATA + 0x5000;
        fill(c, &mut r, dst, 0x40);
        c.g[A0][0] = sx(MC_FIXTURE_THIS);
        c.g[A1][0] = slot as u64;
        c.g[A2][0] = sx(dst);
        c.pc = 0x0024_1710;
    }

    #[test]
    fn mc_manager_get_card_error_text_matches() {
        check_lohi(
            ee::s06::f_00241710__generated,
            mc_manager_get_card_error_text,
            96,
            &|c, seed| mc_card_error_text_setup(c, seed, false),
        );
        fakes::covered(
            ee::s06::f_00241710__generated,
            mc_manager_get_card_error_text,
            192,
            &|c, seed| mc_card_error_text_setup(c, seed, true),
        );
    }

    /// The card status test's steering: bits 0/1 (`seed % 3`), bit 2
    /// (`seed / 3 % 6`), bits 3/4 (`seed / 18 % 3`); seeds 54.. return the
    /// cached word. `logged`: slots answering "yes" are the logging fake (so
    /// their arguments are compared) and some runs unwind; otherwise every
    /// slot is a 2-cycle leaf, so the path totals are fixed.
    pub(crate) fn mc_read_card_status_setup(c: &mut Cpu, seed: u32, logged: bool) {
        let _ = mgr(c, seed, 0x24_1b20, RET_ZERO);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let yes = if logged {
            FAKE
        } else {
            [RET_ONE, RET_MINUS_ONE][(seed / 2 % 2) as usize]
        };
        // Bits 0 and 3 take the answer's low bit: 2 tells `& 1` from `!= 0`.
        let any = [RET_ZERO, RET_ONE, RET_MINUS_ONE, RET_TWO];
        ret_slot(c, vt, mcus::VT_STATUS_BIT0, any[(seed / 5 % 4) as usize]);
        let (bit1_a, bit1_b) = match seed % 3 {
            0 => (yes, RET_ZERO),
            1 => (RET_ZERO, RET_ZERO),
            _ => (RET_ZERO, yes),
        };
        ret_slot(c, vt, mcus::VT_STATUS_BIT1_A, bit1_a);
        ret_slot(c, vt, mcus::VT_STATUS_BIT1_B, bit1_b);
        // Saves on the card, else the signed `free < size` compare.
        let (count, free, size) = match seed / 3 % 6 {
            0 => (yes, RET_ZERO, RET_ZERO),
            1 => (RET_MINUS_ONE, RET_ZERO, RET_ZERO),
            2 => (RET_ZERO, RET_ZERO, RET_ONE),
            3 => (RET_ZERO, RET_MINUS_ONE, RET_ZERO),
            4 => (RET_ZERO, RET_ZERO, RET_MINUS_ONE),
            _ => (RET_ZERO, RET_ONE, RET_ONE),
        };
        ret_slot(c, vt, MEMCARD_CARD_VT_SAVE_COUNT, count);
        ret_slot(c, vt, mcus::VT_FREE_SPACE, free);
        ret_slot(c, vt, MEMCARD_CARD_VT_SAVE_SIZE, size);
        ret_slot(c, vt, mcus::VT_CAN_SAVE, any[(seed / 7 % 4) as usize]);
        let (can_load, bit4) = match seed / 18 % 3 {
            0 => (RET_ZERO, RET_ZERO),
            1 => (yes, yes),
            _ => (yes, RET_ZERO),
        };
        ret_slot(c, vt, mcus::VT_CAN_LOAD, can_load);
        ret_slot(c, vt, mcus::VT_STATUS_BIT4, bit4);
        let slot = seed / 11 % 3;
        w32(c, this + mcus::SELECTED, slot);
        let checked = if seed / 54 % 2 == 1 { slot + 1 } else { slot };
        w32(c, this + mcus::CURR_CHECK, checked);
        w32(
            c,
            this + mcus::CARD_STATUS,
            [0x8000_001f, 0x1234_5678, 0][(seed / 11 % 3) as usize],
        );
        if logged {
            w32(c, LOG_UNWIND_AT, if seed % 4 == 3 { 0 } else { u32::MAX });
        }
        c.g[A0][0] = sx(this);
        c.pc = 0x0024_1b20;
    }

    /// Path totals with every slot a 2-cycle leaf: 82 plus bits 0/1 (1, 10 or
    /// 12), bit 2 (0 or 19) and bits 3/4 (1, 10 or 12); 20 for the cached word.
    pub(crate) const MC_READ_CARD_STATUS_PATHS: [u64; 13] =
        [20, 84, 93, 95, 102, 104, 106, 103, 112, 114, 121, 123, 125];

    #[test]
    fn mc_manager_read_card_status_matches() {
        check(
            ee::s06::f_00241b20__generated,
            mc_manager_read_card_status,
            64,
            &|c, seed| mc_read_card_status_setup(c, seed, true),
        );
        check_paths(
            ee::s06::f_00241b20__generated,
            mc_manager_read_card_status,
            64,
            &MC_READ_CARD_STATUS_PATHS,
            &|c, seed| mc_read_card_status_setup(c, seed, false),
        );
    }

    /// The states of each switch target (`seed % 8`), from the game's table,
    /// and three past its end.
    pub(crate) const GET_STATUS_TEXT_STATES: [&[u32]; 8] = [
        // Checking card.
        &[0, 2, 3, 6, 7, 13, 29, 49, 50, 51, 52, 53, 54],
        // Not detected (the name in the second buffer).
        &[4],
        // Formatting, saving, loading.
        &[10, 11, 39, 40],
        &[15, 16, 17, 18],
        &[31, 32, 33, 34],
        // Deleting (copied as is).
        &[25, 26, 44],
        // No text.
        &[
            1, 5, 8, 9, 12, 14, 19, 20, 21, 22, 23, 24, 27, 28, 30, 35, 36, 37, 38, 41, 42, 43, 45,
            46, 47, 48, 55,
        ],
        // Past the table.
        &[0x38, 0x1000, 0xffff_ffff],
    ];

    #[test]
    fn mc_manager_get_status_text_matches() {
        check(
            ee::s06::f_00241e18__generated,
            mc_manager_get_status_text,
            96,
            &|c, seed| {
                // The switch's table and the empty start string are the game's.
                elf_image(c);
                let mut r = mgr(c, seed, 0x24_1e18, RET_ZERO);
                let states = GET_STATUS_TEXT_STATES[(seed % 8) as usize];
                let state = states[(seed / 8) as usize % states.len()];
                w32(c, MC_FIXTURE_THIS + mcus::STATE, state);
                // Slot (seed / 8 % 3), the lookup's `this` delta (seed / 24
                // % 2) and the unwind (seeds 48..) are separate digits, so each
                // slot's text is formatted with and without an unwind.
                w32(
                    c,
                    MC_FIXTURE_THIS + mcus::SELECTED,
                    [0, 1, 3][(seed / 8 % 3) as usize],
                );
                mc_text_fixture_db(c, [0, 0x40][(seed / 24 % 2) as usize], seed / 48 % 2 == 1);
                let out = DATA + 0x5000;
                fill(c, &mut r, out, 0x40);
                c.g[A0][0] = sx(MC_FIXTURE_THIS);
                c.g[A1][0] = sx(out);
                c.pc = 0x0024_1e18;
            },
        );
    }

    /// The directory read test's steering: the arm (`seed % 7`) is where the
    /// card driver answers stop it: 0..=2 one of the not-detected slots, 3
    /// not formatted, 4 no name, 5 the name read for another slot than the
    /// current one, 6 for the current one (free count stored). `logged`: as
    /// in [`mc_read_card_status_setup`].
    pub(crate) fn mc_read_dir_info_setup(c: &mut Cpu, seed: u32, logged: bool) {
        let mut r = mgr(c, seed, 0x24_20c8, RET_ZERO);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let yes = if logged {
            FAKE
        } else {
            [RET_ONE, RET_MINUS_ONE][(seed / 21 % 2) as usize]
        };
        let arm = seed % 7;
        let answer = |on: bool| if on { yes } else { RET_ZERO };
        ret_slot(c, vt, mcus::VT_NOT_DETECTED_A, answer(arm == 0));
        ret_slot(c, vt, mcus::VT_NOT_DETECTED_B, answer(arm == 1));
        ret_slot(c, vt, mcus::VT_NOT_DETECTED_C, answer(arm == 2));
        ret_slot(c, vt, mcus::VT_FORMATTED, answer(arm >= 4));
        ret_slot(c, vt, mcus::VT_HAS_NAME, answer(arm >= 5));
        ret_slot(
            c,
            vt,
            mcus::VT_FREE_SPACE,
            [RET_ZERO, RET_ONE, RET_MINUS_ONE][(seed / 21 % 3) as usize],
        );
        ret_slot(
            c,
            vt,
            mcus::VT_READ_NAME,
            if logged { FAKE } else { RET_ZERO },
        );
        let slot = [0u32, 2, 3][(seed / 7 % 3) as usize];
        w32(
            c,
            this + mcus::CURR_CHECK,
            if arm == 6 { slot } else { slot + 1 },
        );
        // The record (and its name) non-zero, so every store shows.
        fill(
            c,
            &mut r,
            this + mcus::DEVICES + slot * mcus::DEVICE_SIZE as u32,
            0x38,
        );
        if logged {
            w32(
                c,
                LOG_UNWIND_AT,
                if seed % 4 == 3 {
                    seed / 4 % 3
                } else {
                    u32::MAX
                },
            );
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = slot as u64;
        c.pc = 0x0024_20c8;
    }

    /// Cycles `EA_MemSet` takes to clear a device name (200 bytes at the
    /// fixture's alignment, the same for every slot), part of every path.
    pub(crate) fn mc_read_dir_info_memset_cycles() -> u64 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = sx(MC_FIXTURE_THIS + mcus::DEVICE_NAME);
        c.g[A1][0] = 0;
        c.g[A2][0] = mcus::DEVICE_NAME_BYTES;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = mcus_callee::MEM_FILL;
        let before = c.cycles;
        ee::s14::EA_MemSet(&mut c);
        assert_eq!(c.pc, SENTINEL_RA);
        c.cycles - before
    }

    /// Path totals without the clear, per arm 0..=6.
    pub(crate) const MC_READ_DIR_INFO_PATHS: [u64; 7] = [53, 64, 75, 84, 94, 104, 117];

    #[test]
    fn mc_manager_read_dir_info_matches() {
        check_lohi(
            ee::s06::f_002420c8__generated,
            mc_manager_read_dir_info,
            63,
            &|c, seed| mc_read_dir_info_setup(c, seed, true),
        );
        let clear = mc_read_dir_info_memset_cycles();
        let paths: Vec<u64> = MC_READ_DIR_INFO_PATHS.iter().map(|p| p + clear).collect();
        check_paths_lohi(
            ee::s06::f_002420c8__generated,
            mc_manager_read_dir_info,
            63,
            &paths,
            &|c, seed| mc_read_dir_info_setup(c, seed, false),
        );
    }

    /// The check states' manager at `DATA+0x400`: `mgr`, with every backend
    /// slot below 0x260 answering 0 (a real leaf), the message sink moved off
    /// the backend vtable (the `mgr` sink vtable sits on its slots 0x100..)
    /// to `DATA+0x1800` with slot 8 calling `sink`, and the manager hooks the
    /// handlers and their callees call answering 0.
    pub(crate) fn mc_check_mgr(c: &mut Cpu, seed: u32, salt: u64, sink: u32) -> Rng {
        let r = mgr(c, seed, salt, RET_ZERO);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        for off in (0..0x260).step_by(8) {
            ret_slot(c, vt, off, RET_ZERO);
        }
        let msg = DATA + 0x1800;
        w32(c, this + mcus::MSG, msg);
        ret_slot(c, msg, 8, sink);
        for hook in [
            0x1c,
            0x20,
            0x2c,
            mcus::CB_SAVE_PRESSED,
            mcus::CB_DELETE_PRESSED,
            0x6c,
            mcus::CB_LOAD_PRESSED,
            0xac,
            0xb0,
        ] {
            w32(c, this + hook, RET_ZERO);
        }
        r
    }

    /// The card-ready probe answers for `rd`: fail at the first, second, third
    /// or fourth probe, or (4) ready. `one` is the non-zero answer.
    pub(crate) fn mc_check_ready_slots(c: &mut Cpu, rd: u32, one: u32) {
        let vt = MC_FIXTURE_CARD_VT;
        let [a, b, x, y] = match rd {
            0 => [RET_ZERO, one, RET_ZERO, RET_ZERO],
            1 => [one, RET_ZERO, RET_ZERO, RET_ZERO],
            2 => [one, one, one, RET_ZERO],
            3 => [one, one, RET_ZERO, one],
            _ => [one, one, RET_ZERO, RET_ZERO],
        };
        ret_slot(c, vt, mcus::VT_CAN_LOAD, a);
        ret_slot(c, vt, mcus::VT_STATUS_BIT4, b);
        ret_slot(c, vt, mcus::VT_STATUS_BIT1_A, x);
        ret_slot(c, vt, mcus::VT_STATUS_BIT1_B, y);
    }

    /// Run `generated` once per `(seed, total)` and require that exact cycle
    /// total, so each arm is pinned by its own seed and two arms that cost the
    /// same cannot stand in for each other.
    pub(crate) fn mc_require_arms(
        generated: GuestFn,
        arms: &[(u32, u64)],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for &(seed, total) in arms {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            assert_eq!(c.cycles - before, total, "arm seed {seed}");
        }
    }

    /// The unwind point of a check-state pass (`seed / per_pass`): pass 0 runs
    /// real leaves, pass 1 the logging fakes without an unwind, passes 2..=4
    /// unwind at the fake's call 0, 1 or 2.
    pub(crate) fn mc_check_pass(c: &mut Cpu, pass: u32) {
        if pass >= 1 {
            w32(
                c,
                LOG_UNWIND_AT,
                if pass == 1 { u32::MAX } else { pass - 2 },
            );
        }
    }

    /// Seeds per pass of the State03 test: `row = seed % 18`, `d = seed / 18 % 5`.
    pub(crate) const STATE03_PASS: u32 = 18 * 5;

    /// The State03 fixture: `row` (the arm), `d` (quit kind and probe
    /// answers), the pass (`seed / STATE03_PASS`, see [`mc_check_pass`]):
    /// from pass 1 on the non-zero answers and the sink are the logging fake.
    pub(crate) fn mc_state03_setup(c: &mut Cpu, seed: u32) {
        let row = seed % 18;
        let d = seed / 18 % 5;
        let pass = seed / STATE03_PASS;
        let fakes = pass >= 1;
        let one = if fakes { FAKE } else { RET_ONE };
        let _ = mc_check_mgr(c, seed, 0x23e540, if fakes { FAKE } else { RET_ZERO });
        mc_check_pass(c, pass);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        // Quit kinds: 0 none, 1 delete pending, 2 the load-pressed
        // hook, 3 save pending.
        let free = d % 4;
        let any = 1 + d % 3;
        // (quit, +0x128, +0xc8, +0x124, +0x120, flags, +0x440, computed
        // flags, poll hook answers 1)
        let (q, f128, c8, f124, f120, flags, f440, computed, hook_one) = match row {
            0 => (free, 0, 0, 0, 0, 0, 0, false, false),
            1 => (any, 1, 1, 0, 0, 0, 0, false, false),
            2 => (0, 1, 1, 0, 0, 0, 1, false, false),
            3 => (any, 0, 0, 0, 0, 8, 0, false, false),
            4 => (0, 0, 0, 1, 0, 9, 0, false, false),
            5 => (any, 0, 0, 0, 0, 1, 0, false, false),
            6 => (0, 0, 0, 0, 0, 1, 0, false, false),
            7 => (free, 0, 0, 0, 1, 0x10, 0, false, false),
            8 => (free, 0, 0, 0, 0, 0x10, 0, false, false),
            9 => (free, 0, 0, 0, 1, 4, 0, false, false),
            10 => (0, 0, 0, 0, 0, 8, 0, false, false),
            11 => (free, 0, 0, 0, 0, 0, 0, false, false),
            12 => (free, 0, 0, 0, 0, 8, 0, true, false),
            13 => (free, 0, 0, 0, 0, 1, 0, true, false),
            14 => (free, 0, 0, 0, 0, 8, 0, true, true),
            15 => (0, 0, 1, 0, 0, 0, 1, false, false),
            16 => (3, 0, 0, 0, 0, 0, 1, false, false),
            _ => (1, 1, 0, 0, 0, 0, 1, false, false),
        };
        if row == 0 {
            ret_slot(c, vt, mcus::VT_BUSY, one);
        }
        match q {
            1 => w32(c, this + mcus::DELETE, 1),
            2 => w32(c, this + mcus::CB_LOAD_PRESSED, one),
            3 => w32(c, this + mcus::SAVE, 1),
            _ => {}
        }
        w32(c, this + 0x128, f128);
        w32(c, this + mcus::CARD_CHANGED, c8);
        w32(c, this + 0x124, f124);
        w32(c, this + 0x120, f120);
        w32(c, this + mcus::CARD_STATUS, flags);
        w32(c, this + 0x440, f440);
        // ReadCardStatus keeps the old flags unless the current check
        // slot is the selected one.
        w32(c, this + mcus::SELECTED, 0);
        w32(c, this + mcus::CURR_CHECK, if computed { 0 } else { 1 });
        // Computed flags: bit 0 from slot 0x188, 3 from !0x1a8, 2 from
        // !0x68 (here 0); bits 1 and 4 follow the probe answers.
        ret_slot(c, vt, mcus::VT_STATUS_BIT0, RET_ZERO);
        ret_slot(c, vt, mcus::VT_CAN_SAVE, RET_ONE);
        ret_slot(c, vt, MEMCARD_CARD_VT_SAVE_COUNT, one);
        // Row 13 needs all computed flags clear: the "ready" answers.
        mc_check_ready_slots(c, if row == 13 { 4 } else { d }, one);
        if hook_one {
            w32(c, this + 0xb0, RET_ONE);
        }
        c.pc = 0x0023_e540;
    }

    /// One seed per arm (pass 0: real leaves) and its cycle total. Charges:
    /// prologue 14, busy call 2, epilogue 5; quit test 10/4/12/2 (none,
    /// delete, hook, save); poll 39 (+0x440 clear) or 37 (set) on the cached
    /// flags, 25 + ReadCardStatus (93 at d 0, 102 ready) on changed ones; a
    /// post costs 13 (+6, sink 2, epilogue); ReturnToSlotState 27,
    /// SetStateNoCard 13, OnCardScanned 31; probe k fails after 2 + 10(k-1),
    /// ready 32.
    pub(crate) const STATE03_ARMS: &[(u32, u64)] = &[
        // Busy: 14+2+5.
        (0, 21),
        // Post 0x38, quit by +0xc0 / the hook / +0xbc: 14+2+3+q+2+7+13.
        (1, 45),
        (1 + 18, 53),
        (1 + 18 * 2, 43),
        // ReturnToSlotState after +0x128: 14+2+3+10+2+6 +4+37 +2+3+5+27+5.
        (2, 120),
        // SetStateNoCard, quitting / by +0x124: 25+4+39+9+4+13+5; 31+43+7+22.
        (3, 99),
        (4, 103),
        // Post 5, quitting / not: 68+9+4+13; 74+9+5+13.
        (5, 94),
        (6, 101),
        // Post 0x27: 74+10+9+13.
        (7, 106),
        // Release with flags 0x10 (84+11+2 +18+2+13) and 4 (delete pending:
        // 68+9+13+2 +11+12+13).
        (8, 130),
        (9 + 18, 128),
        // Post 0x31: 74+9+2+10+13.
        (10, 108),
        // Release with no flags: 84+11+4 +18+2+13.
        (11, 132),
        // OnCardScanned, flag 8 lost / all flags cleared: 31+4+118+10+11+40;
        // 31+4+127+10+11+4+40.
        (12, 214),
        (13, 227),
        // Poll false after a change: 31+4+118 +18+2+13.
        (14, 186),
        // ReturnToSlotState: 31+4+37 +10+27+5.
        (15, 114),
        // Save pending, the probe failing 1st..4th, ready: 64+11 +2+10(k-1)
        // +2+13; 64+11+32+1+13.
        (16, 92),
        (16 + 18, 102),
        (16 + 18 * 2, 112),
        (16 + 18 * 3, 122),
        (16 + 18 * 4, 121),
        // Delete pending, no save: the same probes, 69+11 +2+10(k-1) +13;
        // 69+11+32+1+13.
        (17, 95),
        (17 + 18, 105),
        (17 + 18 * 2, 115),
        (17 + 18 * 3, 125),
        (17 + 18 * 4, 126),
    ];

    #[test]
    fn mc_manager_state03_check_matches() {
        fakes::covered(
            ee::s06::f_0023e540__generated,
            mc_manager_state03_check,
            5 * STATE03_PASS,
            &mc_state03_setup,
        );
        mc_require_arms(
            ee::s06::f_0023e540__generated,
            STATE03_ARMS,
            &mc_state03_setup,
        );
    }

    /// Seeds per pass of the State06 test: `row = seed % 19`, `d = seed / 19 % 5`.
    pub(crate) const STATE06_PASS: u32 = 19 * 5;

    /// The State06 fixture: `row` (the arm), `d` (quit kind and probe
    /// answers), the pass (`seed / STATE06_PASS`, see [`mc_check_pass`]).
    pub(crate) fn mc_state06_setup(c: &mut Cpu, seed: u32) {
        let row = seed % 19;
        let d = seed / 19 % 5;
        let pass = seed / STATE06_PASS;
        let fakes = pass >= 1;
        let one = if fakes { FAKE } else { RET_ONE };
        let _ = mc_check_mgr(c, seed, 0x23ec00, if fakes { FAKE } else { RET_ZERO });
        mc_check_pass(c, pass);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let any = 1 + d % 3;
        // (quit kind as in State03, +0xc8, +0x128, +0x124, flags,
        // +0x440, ready probe answers)
        let (q, c8, f128, f124, flags, f440, rd) = match row {
            0 | 1 => (0, 0, 0, 0, 0, 0, d),
            2 => (any, 1, 1, 0, 0, 0, d),
            3 => (0, 1, 1, 0, 0, 0, d),
            4 => (any, 1, 0, 0, 0, 0, d),
            5 => (3, 0, 0, 0, 8, 0, d),
            6 => (3, 0, 0, 0, 8, 1, d),
            7 => (3, 0, 0, 0, 1, 0, d),
            8 => (3, 0, 0, 0, 0, 0, d),
            9..=11 => (3, 0, 0, 0, 0, 0, 4),
            12 => (1, 0, 0, 0, 8, 0, d),
            13 => (2, 0, 0, 0, 8, 0, d),
            14 => (0, 0, 0, 1, 8, 0, d),
            15 => (0, 0, 0, 0, 8, 0, d),
            16 => (d % 2, 0, 0, 0, 1, 0, d),
            17 => (2, 0, 0, 0, 1, 0, d),
            _ => (d % 2, 0, 0, 0, 0, 0, d),
        };
        // The wait above 0 for row 0; 0 or below elsewhere.
        w32(
            c,
            this + mc::WAIT,
            if row == 0 {
                1
            } else {
                0u32.wrapping_sub(d % 2)
            },
        );
        if row == 1 {
            ret_slot(c, vt, mcus::VT_BUSY, one);
        }
        match q {
            1 => w32(c, this + mcus::DELETE, 1),
            2 => w32(c, this + mcus::CB_LOAD_PRESSED, one),
            3 => w32(c, this + mcus::SAVE, 1),
            _ => {}
        }
        w32(c, this + mcus::CARD_CHANGED, c8);
        w32(c, this + 0x128, f128);
        w32(c, this + 0x124, f124);
        w32(c, this + mcus::CARD_STATUS, flags);
        w32(c, this + 0x440, f440);
        // The poll keeps the flags; the directory reread is slot 1.
        w32(c, this + mcus::SELECTED, 0);
        w32(c, this + mcus::CURR_CHECK, 1);
        mc_check_ready_slots(c, rd, one);
        // Slot 0x68 set (row 8), or the room test: 0xa0 below 0xb8
        // (row 9), not below with slot 0xc8 set (10) or zero (11).
        ret_slot(
            c,
            vt,
            MEMCARD_CARD_VT_SAVE_COUNT,
            if row == 8 { one } else { RET_ZERO },
        );
        ret_slot(
            c,
            vt,
            MEMCARD_CARD_VT_SAVE_SIZE,
            if row == 9 { RET_ONE } else { RET_ZERO },
        );
        ret_slot(c, vt, 0xc8, if row == 10 { one } else { RET_ZERO });
        c.pc = 0x0023_ec00;
    }

    /// One seed per arm (pass 0) and its cycle total. Charges: prologue and
    /// poll 52 (13 + 39 on the cached flags; 37 with +0x440 set), not waiting
    /// and idle 65; a post 13; SetStateNoCard 22 with its call and epilogue;
    /// the directory reread and post 0x1d 188 (ReadCurrentDirInfo 172), the
    /// reread and post 7 190; probe fails at k after 2 + 10(k-1), ready 32.
    pub(crate) const STATE06_ARMS: &[(u32, u64)] = &[
        // Still waiting: 13+39+1+5.
        (0, 58),
        // Busy: 13+39+8+2+5.
        (1, 67),
        // Post 0x38, quit by +0xc0 / the hook / +0xbc: 65+2+q+6+13.
        (2, 90),
        (2 + 19, 98),
        (2 + 19 * 2, 88),
        // A changed card with +0x128 set, not quitting: ready (post 6:
        // 65+2+10+5+7+32+3+13), or not (no-save failure: 65+2+10+5+7+2+2+17
        // +2+188).
        (3 + 19 * 4, 137),
        (3, 300),
        // A changed card, quitting: ready by the hook (65+2+12+2+7+32+3+13),
        // not ready on delete (65+2+4+2+7+2+2+12+2+188), not ready on save (on
        // to the save path: 65+2+2+2+7+22+2+4+2+8+22+190).
        (4 + 19 * 4, 136),
        (4, 286),
        (4 + 19 * 2, 328),
        // Save pending: SetStateNoCard (65+2+4+2+22); flag 8 without a change
        // (13+37+8+2+3+2+4+2+190); post 0x14 (65+2+4+2+2+13).
        (5, 95),
        (6, 261),
        (7, 88),
        // The probe failing 1st..4th, slot 0x68 set: 65+2+4+2+8+P+190.
        (8, 273),
        (8 + 19, 283),
        (8 + 19 * 2, 293),
        (8 + 19 * 3, 303),
        (8 + 19 * 4, 313),
        // No room by slot 0xa0 (post 0x15: 65+2+4+2+8+32+8+2+16+2+2+2+13), slot
        // 0xc8 set (...+16+2+2+8+2+190), no room by slot 0xc8 (...+16+2+2+8+2
        // +1+2+13).
        (9, 158),
        (10, 343),
        (11, 169),
        // No save pending: SetStateNoCard on delete (65+2+12+22), by the hook
        // (65+2+19+22), by +0x124 (65+2+15+22).
        (12, 101),
        (13, 108),
        (14, 104),
        // Flag 8 without quit (65+2+17+188), post 5 (delete pending:
        // 65+2+12+2+5+2+2+13), post 0x24 (65+2+19+2+5+2+2+13), no flags
        // (65+2+17+2+188).
        (15, 272),
        (16 + 19, 103),
        (17, 110),
        (18, 274),
    ];

    #[test]
    fn mc_manager_state06_check_matches() {
        fakes::covered(
            ee::s06::f_0023ec00__generated,
            mc_manager_state06_check,
            5 * STATE06_PASS,
            &mc_state06_setup,
        );
        mc_require_arms(
            ee::s06::f_0023ec00__generated,
            STATE06_ARMS,
            &mc_state06_setup,
        );
    }

    /// The State07 fixture: `row` (the arm), `d` (probe answers, the delete
    /// tail: hook at +0x68 quiet, pressed, pressed with deleting allowed),
    /// `fakes` (FAKE for the non-zero answers and the sink).
    pub(crate) fn mc_state07_fixture(c: &mut Cpu, seed: u32, row: u32, d: u32, fakes: bool) -> Rng {
        let one = if fakes { FAKE } else { RET_ONE };
        let mut r = mc_check_mgr(c, seed, 0x23f698, if fakes { FAKE } else { RET_ZERO });
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let (mut f338, mut f128, mut c8, mut save, mut f440, mut flags) = (0, 0, 0, one, 0, 0);
        let (mut hook6c, mut allowed, mut rd, mut used, mut name, mut cursor) =
            (RET_ZERO, 1, d, RET_ZERO, 0u16, 0);
        let (mut b8, mut room, mut s208, mut s210, mut s1a8) =
            (RET_ZERO, one, RET_ZERO, RET_ZERO, RET_ZERO);
        match row {
            0 => f338 = 1,
            1 => (f128, c8) = (1, 1),
            2 => (f128, allowed, s208) = (1, 0, one),
            3 => (save, flags) = (RET_ZERO, 8),
            4 => (save, flags) = (RET_ZERO, 0x10),
            5 => (save, flags) = (RET_ZERO, 2),
            6 => (save, flags) = (RET_ZERO, 1),
            7 => (save, flags) = (RET_ZERO, 4),
            8 => save = RET_ZERO,
            9 => (save, f440, f128) = (RET_ZERO, 1, 1),
            10 => c8 = 1,
            11 => (save, hook6c) = (RET_ZERO, one),
            12 => {}
            13 => (rd, b8) = (4, RET_ONE),
            14 => (rd, room) = (4, RET_ZERO),
            15 => (rd, used) = (4, one),
            16 => (rd, used, name) = (4, one, 0x41),
            17 => (rd, used, cursor) = (4, one, 3),
            18 => (allowed, s210) = (0, one),
            19 => (allowed, s1a8) = (0, one),
            _ => allowed = 0,
        }
        w32(c, this + mcus::CHECK_CARD, f338);
        w32(c, this + 0x128, f128);
        w32(c, this + mcus::CARD_CHANGED, c8);
        w32(c, this + mcus::CB_SAVE_PRESSED, save);
        w32(c, this + 0x440, f440);
        w32(c, this + mcus::CARD_STATUS, flags);
        w32(c, this + 0x6c, hook6c);
        w32(c, this + mcus::SAVE_OK, allowed);
        w32(c, this + mcus::FILE_CURSOR, cursor);
        c.bus.write16(this + 0x276, name);
        // The save block stores 1.0 and the `gp - 0x5124` float at
        // +0x110/+0x114: three different words before, so a dropped store or
        // a wrong offset shows.
        w32(c, GAME_GP - 0x5124, (r.next_u32() & !3) | 1);
        w32(c, this + mcus::RAMP, (r.next_u32() & !3) | 2);
        w32(c, this + mcus::RAMP_STEP, (r.next_u32() & !3) | 3);
        // The poll keeps the flags.
        w32(c, this + mcus::SELECTED, 0);
        w32(c, this + mcus::CURR_CHECK, 1);
        mc_check_ready_slots(c, rd, one);
        ret_slot(c, vt, mcus::VT_FILE_EXISTS, used);
        ret_slot(c, vt, MEMCARD_CARD_VT_SAVE_SIZE, b8);
        ret_slot(c, vt, 0xc8, room);
        ret_slot(c, vt, 0x208, s208);
        ret_slot(c, vt, 0x210, s210);
        ret_slot(c, vt, mcus::VT_CAN_SAVE, s1a8);
        let dc = d % 3;
        w32(
            c,
            this + mcus::CB_DELETE_PRESSED,
            if dc == 0 { RET_ZERO } else { one },
        );
        w32(c, this + mcus::DELETE_OK, (dc == 2) as u32);
        c.pc = 0x0023_f698;
        r
    }

    /// Seeds per pass of the State07 test: `row = seed % 21`, `d = seed / 21 % 5`.
    pub(crate) const STATE07_PASS: u32 = 21 * 5;

    /// The State07 test's setup: the fixture for `seed`'s row and `d`, the
    /// pass (`seed / STATE07_PASS`, see [`mc_check_pass`]).
    pub(crate) fn mc_state07_setup(c: &mut Cpu, seed: u32) {
        let pass = seed / STATE07_PASS;
        let _ = mc_state07_fixture(c, seed, seed % 21, seed / 21 % 5, pass >= 1);
        mc_check_pass(c, pass);
    }

    /// One seed per arm (pass 0) and its cycle total. Charges: prologue 8,
    /// epilogue 4, a post 12 (+6, sink 2, epilogue); poll 39 (+0x440 clear)
    /// or 37 (set) on the cached flags; SetStateNoCard 13, ReturnToSlotState
    /// 27; `UStr_Cmp` 18 on two empty names, 25 on "A" against an empty one;
    /// the delete tail (d % 3) quiet +4, pressed +3+4, pressed and allowed
    /// +3+14+2+2+4 after the arm's own charges.
    pub(crate) const STATE07_ARMS: &[(u32, u64)] = &[
        // Checking the card: 8+4.
        (0, 12),
        // Post 0x38: 8+3+3+2+12.
        (1, 28),
        // Post 0x16, the three delete tails: 112 then +4; +3+4; +3+14+2+2+4.
        (2, 116),
        (2 + 21, 119),
        (2 + 21 * 2, 137),
        // SetStateNoCard (flag 8): 60+2+4+4+13+4.
        (3, 87),
        // Posts 0x27 (60+2+4+2+2+12), 0x30 (68+3+2+12), 5 (68+3+3+2+12),
        // 0x17 (68+3+3+3+2+12).
        (4, 82),
        (5, 85),
        (6, 88),
        (7, 91),
        // No flag, delete tail quiet (99+4) and pressed and allowed (99+3+14
        // +2+2+4).
        (8, 103),
        (8 + 21 * 2, 124),
        // Poll unchanged (+0x440 set), pressed and allowed: 84+3+14+2+2+4.
        (9 + 21 * 2, 109),
        // ReturnToSlotState: 60+2+5+27+4.
        (10, 98),
        // Post 1 (the +0x6c hook): 60+18+2+4+2+2+12.
        (11, 100),
        // Not ready, the probe failing 1st..4th: 92 + 10(k-1) +2+12.
        (12, 106),
        (12 + 21, 116),
        (12 + 21 * 2, 126),
        (12 + 21 * 3, 136),
        // Ready: the name loop over six free slots, post 0xd: 133+16+2+2
        // +8+2+1+1 +6*14 +8+2+3+6+2+4+2+3+4.
        (12 + 21 * 4, 283),
        // No room by slot 0xa0 (153+2+12), by slot 0xc8 (163+1+2+12).
        (13, 167),
        (14, 178),
        // A clash after the cursor's own slot (134+49+45+1+12), six used
        // slots with other names (post 9: 134+6*52+8+2+10+2+12), a clash at
        // slot 0 (134+45+1+12).
        (15, 241),
        (16, 480),
        (17, 192),
        // Post 0x17 (slot 0x210): tail quiet (119+4), pressed and allowed
        // (119+3+14+2+2+4).
        (18, 123),
        (18 + 21 * 2, 144),
        // Nothing (slot 0x1a8 set): pressed (120+3+4), pressed and allowed
        // (120+3+14+2+2+4).
        (19 + 21, 127),
        (19 + 21 * 2, 145),
        // Post 0x14: tail quiet (129+4), pressed and allowed (129+3+14+2+2+4).
        (20, 133),
        (20 + 21 * 2, 154),
    ];

    #[test]
    fn mc_manager_state07_check_matches() {
        fakes::covered(
            ee::s06::f_0023f698__generated,
            mc_manager_state07_check,
            5 * STATE07_PASS,
            &mc_state07_setup,
        );
        mc_require_arms(
            ee::s06::f_0023f698__generated,
            STATE07_ARMS,
            &mc_state07_setup,
        );
    }

    driven!(gen_state07_check, ee::s06::f_0023f698__generated);

    driven!(hand_state07_check, mc_manager_state07_check);

    /// Events at the name loop's back edge (0x23f8ec): six free slots, or
    /// the cursor's slot 0 used (its empty name matches; the next slot
    /// clashes after one back edge). The names stay empty so `UStr_NCmp`'s
    /// own back edge never runs.
    #[test]
    fn mc_manager_state07_check_events() {
        check_events(gen_state07_check, hand_state07_check, 60, &|c, seed| {
            let row = if seed / 3 % 2 == 0 { 12 } else { 15 };
            let mut r = mc_state07_fixture(c, seed, row, 4, false);
            schedule(c, &mut r, seed, 0x0023_f698)
        });
    }

    /// Real leaves for a backend whose "finished" answer (slot 0x248) turns
    /// non-zero after slot 0xf8 runs: `[a0 + 0x2c] = a1`
    /// (`cFEStateBraggingRights_SetRowMask`) and `return [a0 + 0xc]`
    /// (`FEUIInput_v41`).
    pub(crate) const STATE20_SETTER: u32 = 0x0019_2968;

    pub(crate) const STATE20_GETTER: u32 = 0x001d_d560;

    /// Cycle totals of the arms (deterministic seeds): finished at once;
    /// slot 0xf8 not 1 and not finished; finished meanwhile; the read hook
    /// busy; read done (FinishLoad); the +0x20 hook quiet (slot 0x140,
    /// FinishLoad); the +0x20 hook set (CardSetState1E, post 0x21).
    pub(crate) const STATE20_PATHS: &[u64] = &[48, 54, 68, 57, 92, 98, 81];

    /// Seeds per pass of the State20 test: `row = seed % 8`, `d = seed / 8 % 4`.
    pub(crate) const STATE20_PASS: u32 = 8 * 4;

    /// The State20 fixture: `row` (the arm), `d` (the read window, the file
    /// cursor, the progress answer), the pass (`seed / STATE20_PASS`, see
    /// [`mc_check_pass`]).
    pub(crate) fn mc_state20_setup(c: &mut Cpu, seed: u32) {
        let row = seed % 8;
        let d = seed / 8 % 4;
        let pass = seed / STATE20_PASS;
        let fakes = pass >= 1;
        let one = if fakes { FAKE } else { RET_ONE };
        let mut r = mc_check_mgr(c, seed, 0x240130, if fakes { FAKE } else { RET_ZERO });
        mc_check_pass(c, pass);
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        // (+0xe8, +0xf0): below for rows 3 and 4, not below elsewhere.
        let (e8, f0): (i32, i32) = if row == 3 || row == 4 {
            [(0, 1), (-3, 2), (1, 5), (-1, 0)][d as usize]
        } else {
            [(1, 1), (5, 2), (0, -4), (0, 0)][d as usize]
        };
        w32(c, this + 0xe8, e8 as u32);
        w32(c, this + 0xf0, f0 as u32);
        w32(c, this + mc::WAIT, r.next_u32());
        w32(c, this + mcus::FILE_CURSOR, d);
        // The progress numerator, and the float there before.
        ret_slot(c, vt, 0x90, if d % 2 == 0 { RET_ZERO } else { one });
        w32(c, this + 0x10c, r.next_u32());
        ret_slot(c, vt, 0x248, if row == 0 { one } else { RET_ZERO });
        ret_slot(
            c,
            vt,
            0xf8,
            match row {
                0..=2 => RET_ZERO,
                7 => FAKE,
                _ => RET_ONE,
            },
        );
        if row == 2 {
            // Slot 0xf8 stores the backend at backend+0x42c, which
            // slot 0x248 reads.
            c.bus.write16(vt + 0xf8, 0x400);
            w32(c, vt + 0xfc, STATE20_SETTER);
            c.bus.write16(vt + 0x248, 0x420);
            w32(c, vt + 0x24c, STATE20_GETTER);
        }
        w32(c, this + 0x1c, if row == 3 { one } else { RET_ZERO });
        w32(c, this + 0x20, if row == 6 { one } else { RET_ZERO });
        ret_slot(c, vt, 0x140, if fakes { FAKE } else { RET_ZERO });
        c.pc = 0x0024_0130;
    }

    /// Every seed, the fakes and unwinds included, against the generated
    /// body; the path totals on the leaf pass only (a fake's cost varies, so
    /// it could stand in for a listed total).
    #[test]
    fn mc_manager_state20_load_matches() {
        fakes::covered(
            ee::s06::f_00240130__generated,
            mc_manager_state20_load,
            5 * STATE20_PASS,
            &mc_state20_setup,
        );
        require_paths(
            ee::s06::f_00240130__generated,
            STATE20_PASS,
            STATE20_PATHS,
            &mc_state20_setup,
        );
    }

    // ------------------------------------------------- MCManager_State1D_Check

    /// Stand-in ids: stand-in `k` is the call target `FAKE + 0x10 * k`.
    pub(crate) const STATE1D_CB78: usize = 0;

    pub(crate) const STATE1D_CB68: usize = 1;

    pub(crate) const STATE1D_CB6C: usize = 2;

    pub(crate) const STATE1D_TICK: usize = 3;

    pub(crate) const STATE1D_STATUS: usize = 4;

    pub(crate) const STATE1D_SCANNED: usize = 5;

    pub(crate) const STATE1D_SINK: usize = 6;

    pub(crate) const STATE1D_BUSY: usize = 7;

    pub(crate) const STATE1D_VT1A8: usize = 8;

    pub(crate) const STATE1D_VT218: usize = 9;

    pub(crate) const STATE1D_VT208: usize = 10;

    pub(crate) const STATE1D_VT70: usize = 11;

    /// `MCManager_ReadCardStatus`'s backend slots 0x188, 0x198, 0x1d8,
    /// 0x68, 0xa0, 0xb8, 0x1b8, 0x1d0 (its 0x1a8 is `STATE1D_VT1A8`).
    pub(crate) const STATE1D_RCS: usize = 12;

    pub(crate) const STATE1D_STAND_INS: usize = 20;

    /// The backend vtable slots and their stand-ins.
    pub(crate) const STATE1D_SLOTS: [(u32, usize); 13] = [
        (mcus::VT_BUSY, STATE1D_BUSY),
        (mcus::VT_CAN_SAVE, STATE1D_VT1A8),
        (mcus::VT_FILE_EXISTS, STATE1D_VT218),
        (0x208, STATE1D_VT208),
        (MEMCARD_CARD_VT_SLOT_NAME, STATE1D_VT70),
        (mcus::VT_STATUS_BIT0, STATE1D_RCS),
        (mcus::VT_STATUS_BIT1_A, STATE1D_RCS + 1),
        (mcus::VT_STATUS_BIT1_B, STATE1D_RCS + 2),
        (MEMCARD_CARD_VT_SAVE_COUNT, STATE1D_RCS + 3),
        (mcus::VT_FREE_SPACE, STATE1D_RCS + 4),
        (MEMCARD_CARD_VT_SAVE_SIZE, STATE1D_RCS + 5),
        (mcus::VT_CAN_LOAD, STATE1D_RCS + 6),
        (mcus::VT_STATUS_BIT4, STATE1D_RCS + 7),
    ];

    /// `[script, calls]` per stand-in, at `STATE1D_SCRIPTS + 8 * k`.
    pub(crate) const STATE1D_SCRIPTS: u32 = DATA + 0x3000;

    /// Cycles every stand-in call costs.
    pub(crate) const STATE1D_CALL_COST: u64 = 4;

    /// The miss hook of this fixture: logs the call like [`fake_call`], then
    /// gives stand-in `k` a fixed cost and answers its n-th call with bit n
    /// of its script word, so every arm has one cycle total.
    pub(crate) fn state1d_stand_in(c: &mut Cpu, target: u32) {
        let at = c.cycles;
        fake_call(c, target);
        c.cycles = at + STATE1D_CALL_COST;
        let k = target.wrapping_sub(FAKE) / 0x10;
        if (k as usize) < STATE1D_STAND_INS {
            let word = STATE1D_SCRIPTS + 8 * k;
            let n = c.bus.read32(word + 4);
            c.bus.write32(word + 4, n + 1);
            c.g[V0][0] = ((c.bus.read32(word) >> n.min(31)) & 1) as u64;
        }
    }

    /// One arm: the manager words it needs (0 elsewhere), the hooks' and
    /// backend slots' script words (bit n: the n-th call's result) and the
    /// generated body's cycle total for it.
    #[derive(Clone, Copy)]
    pub(crate) struct State1dCase {
        /// `+0x338`, `+0xbc`, `+0xc0`.
        check: u32,
        save: u32,
        del: u32,
        /// Scripts of the `+0x78`, `+0x68` and `+0x6c` hooks.
        cb78: u32,
        cb68: u32,
        cb6c: u32,
        /// `+0x43c` before the poll; how the poll reads the card: 0 does not
        /// (`+0x42c != +0x428`, the flags stay), 1 reads flags 0x12, 2 reads 0.
        held: u32,
        card: u32,
        /// `+0x440` (non-zero: the card is not ready unless the flags change).
        status: u32,
        w11c: u32,
        w120: u32,
        w124: u32,
        /// `+0xc8`, `+0x130`, `+0xd0`, `+0xd4`.
        changed: u32,
        state: u32,
        d0: u32,
        d4: u32,
        /// Scripts of backend slots 0x1a8, 0x218 and 0x208.
        vt1a8: u32,
        vt218: u32,
        vt208: u32,
        cycles: u64,
    }

    pub(crate) const STATE1D_NONE: State1dCase = State1dCase {
        check: 0,
        save: 0,
        del: 0,
        cb78: 0,
        cb68: 0,
        cb6c: 0,
        held: 0,
        card: 0,
        status: 0,
        w11c: 0,
        w120: 0,
        w124: 0,
        changed: 0,
        state: 0,
        d0: 0,
        d4: 0,
        vt1a8: 0,
        vt218: 0,
        vt208: 0,
        cycles: 0,
    };

    pub(crate) const STATE1D_ALL: u32 = 0xffff_ffff;

    /// Every arm, by `seed % 28`.
    pub(crate) const STATE1D_CASES: [State1dCase; 28] = [
        // 0: the periodic check is running.
        State1dCase {
            check: 1,
            cycles: 16,
            ..STATE1D_NONE
        },
        // 1-4: flag 8, no card by +0x124, the second +0x78 call, +0x68, quit.
        State1dCase {
            held: 8,
            w124: 1,
            cycles: 111,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 8,
            cb78: 0b100,
            cycles: 120,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 8,
            cb68: 1,
            cycles: 129,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 8,
            save: 1,
            cycles: 121,
            ..STATE1D_NONE
        },
        // 5-6: flag 8 kept: post 0x31 (6: after a card change).
        State1dCase {
            held: 8,
            cycles: 138,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 8,
            changed: 1,
            cycles: 173,
            ..STATE1D_NONE
        },
        // 7-10: posts 0x27, 5 (+0x120 set and not), 0x17.
        State1dCase {
            held: 0x10,
            w120: 1,
            cycles: 105,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 1,
            w120: 1,
            cycles: 108,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 1,
            cycles: 106,
            ..STATE1D_NONE
        },
        State1dCase {
            held: 4,
            w11c: 1,
            w120: 1,
            cycles: 113,
            ..STATE1D_NONE
        },
        // 11: flags set, +0x11c without flag 4: no rescan.
        State1dCase {
            held: 0x10,
            w11c: 1,
            cycles: 128,
            ..STATE1D_NONE
        },
        // 12: flags 0 before and after: no rescan, then a card change.
        State1dCase {
            changed: 1,
            cycles: 163,
            ..STATE1D_NONE
        },
        // 13: flag 8 went away (8 -> 0x12): rescan.
        State1dCase {
            held: 8,
            card: 1,
            cycles: 228,
            ..STATE1D_NONE
        },
        // 14: flags dropped to 0 (2 -> 0): rescan.
        State1dCase {
            held: 2,
            card: 2,
            cycles: 254,
            ..STATE1D_NONE
        },
        // 15: the card is not ready.
        State1dCase {
            status: 1,
            cycles: 101,
            ..STATE1D_NONE
        },
        // 16: +0xc0 pending and the +0x78 hook fires: quit, the state moved on.
        State1dCase {
            del: 1,
            cb78: 1,
            cycles: 86,
            ..STATE1D_NONE
        },
        // 17-21: still state 0x1d with the +0x78 hook firing: post 0x1f,
        // 0x24, 0x25 (slot 0x218), nothing (slot 0x208), 0x25.
        State1dCase {
            del: 1,
            cb78: STATE1D_ALL,
            state: 0x1d,
            d0: 1,
            cycles: 110,
            ..STATE1D_NONE
        },
        State1dCase {
            cb78: STATE1D_ALL,
            state: 0x1d,
            cycles: 133,
            ..STATE1D_NONE
        },
        State1dCase {
            cb78: STATE1D_ALL,
            state: 0x1d,
            vt1a8: 1,
            cycles: 145,
            ..STATE1D_NONE
        },
        State1dCase {
            del: 1,
            cb78: STATE1D_ALL,
            state: 0x1d,
            vt1a8: 1,
            vt218: 1,
            cycles: 135,
            ..STATE1D_NONE
        },
        State1dCase {
            cb78: STATE1D_ALL,
            state: 0x1d,
            vt1a8: 1,
            vt218: 1,
            vt208: 1,
            cycles: 156,
            ..STATE1D_NONE
        },
        // 22-25: state 0x1d, +0x78 quiet: +0x68 without and with +0xd4 (slot
        // 0x70, post 0x18), +0x6c (post 1), nothing.
        State1dCase {
            del: 1,
            status: 1,
            cb68: 1,
            state: 0x1d,
            cycles: 107,
            ..STATE1D_NONE
        },
        State1dCase {
            save: 1,
            status: 1,
            cb68: 1,
            state: 0x1d,
            d4: 1,
            cycles: 127,
            ..STATE1D_NONE
        },
        State1dCase {
            save: 1,
            status: 1,
            cb6c: 1,
            state: 0x1d,
            cycles: 119,
            ..STATE1D_NONE
        },
        State1dCase {
            del: 1,
            status: 1,
            state: 0x1d,
            cycles: 112,
            ..STATE1D_NONE
        },
        // 26-27: no quit, state 0x1d: nothing fires; +0x78 fires from its
        // second call on (post 0x1f).
        State1dCase {
            state: 0x1d,
            cycles: 154,
            ..STATE1D_NONE
        },
        State1dCase {
            cb78: 0b110,
            state: 0x1d,
            d0: 1,
            cycles: 125,
            ..STATE1D_NONE
        },
    ];

    /// The manager at `DATA+0x400` (the `mgr` fixture) with every hook and
    /// backend slot a scripted stand-in, set up for arm `seed % 28`;
    /// `seed / 28 % 2` adds `this` deltas to the backend and sink slots and
    /// moves the file cursor; `seed / 56` is no unwind (0) or the stand-in
    /// call that unwinds (1..=3: call 0..=2; the stand-ins log and unwind
    /// through [`fake_call`]).
    pub(crate) fn state1d_fixture(c: &mut Cpu, seed: u32) {
        let n = STATE1D_CASES.len() as u32;
        let case = STATE1D_CASES[(seed % n) as usize];
        let variant = seed / n % 2;
        let unwind = seed / (2 * n);
        let mut r = mgr(c, seed, 0x2402d0, FAKE);
        w32(
            c,
            LOG_UNWIND_AT,
            if unwind == 0 { u32::MAX } else { unwind - 1 },
        );
        c.hooks.miss = state1d_stand_in;
        let stand_in = |k: usize| FAKE + 0x10 * k as u32;
        let this = MC_FIXTURE_THIS;
        let vt = MC_FIXTURE_CARD_VT;
        let sink = DATA + 0x1200;
        // Words neither the state nor the callees it reaches read, non-zero
        // so a store at a wrong offset shows.
        for off in [
            mcus::SAVE_MODE,
            mc::UPDATE_STEP,
            mcus::SAVE_OK,
            mcus::CANCEL_OK,
            mc::WAIT,
            0x10c,
            mcus::RAMP,
            mcus::RAMP_STEP,
            mcus::LAST_CHECK,
        ] {
            w32(c, this + off, r.next_u32() | 2);
        }
        for (off, k) in [
            (mcus::CB_LOAD_PRESSED, STATE1D_CB78),
            (mcus::CB_DELETE_PRESSED, STATE1D_CB68),
            (0x6c, STATE1D_CB6C),
            (0xa4, STATE1D_TICK),
            (0xb0, STATE1D_STATUS),
            (0xac, STATE1D_SCANNED),
        ] {
            c.bus.write32(this + off, stand_in(k));
        }
        for (off, k) in STATE1D_SLOTS {
            c.bus.write16(vt + off, (variant * 8) as u16);
            c.bus.write32(vt + off + 4, stand_in(k));
        }
        c.bus.write16(sink + 8, (variant * 0x10) as u16);
        c.bus.write32(sink + 0xc, stand_in(STATE1D_SINK));

        let mut script = [0u32; STATE1D_STAND_INS];
        script[STATE1D_CB78] = case.cb78;
        script[STATE1D_CB68] = case.cb68;
        script[STATE1D_CB6C] = case.cb6c;
        // The card read's own 0x1a8 call comes first (bit 0 = 1: flag 8 clear).
        script[STATE1D_VT1A8] = case.vt1a8 | (case.card != 0) as u32;
        script[STATE1D_VT218] = case.vt218;
        script[STATE1D_VT208] = case.vt208;
        // The card read: flag 1 = 0x188, flag 2 = 0x198 || 0x1d8, flag 4 =
        // !0x68 && 0xa0 < 0xb8, flag 0x10 = !0x1b8 || !0x1d0. Case 1 gives
        // 0x12, case 2 gives 0.
        script[STATE1D_RCS + 1] = (case.card == 1) as u32;
        script[STATE1D_RCS + 3] = 1;
        script[STATE1D_RCS + 6] = (case.card == 2) as u32;
        script[STATE1D_RCS + 7] = 1;
        for (k, bits) in script.iter().enumerate() {
            c.bus.write32(STATE1D_SCRIPTS + 8 * k as u32, *bits);
            c.bus.write32(STATE1D_SCRIPTS + 8 * k as u32 + 4, 0);
        }

        for (off, v) in [
            (mcus::CHECK_CARD, case.check),
            (mcus::SAVE, case.save),
            (mcus::DELETE, case.del),
            (mcus::CARD_STATUS, case.held),
            (mcus::CURR_CHECK, (case.card == 0) as u32),
            (0x440, case.status),
            (0x11c, case.w11c),
            (0x120, case.w120),
            (0x124, case.w124),
            (mcus::CARD_CHANGED, case.changed),
            (mcus::STATE, case.state),
            (mcus::LOAD_OK, case.d0),
            (mcus::DELETE_OK, case.d4),
            (mcus::FILE_CURSOR, 2 + variant),
        ] {
            c.bus.write32(this + off, v);
        }
        c.pc = 0x0024_02d0;
    }

    /// Both variants of every arm with no unwind and with an unwind at the
    /// first three stand-in calls; each arm's total on its own seeds (no
    /// unwind).
    #[test]
    fn mc_manager_state1d_check_matches() {
        let n = STATE1D_CASES.len() as u32;
        fakes::covered(
            ee::s06::f_002402d0__generated,
            mc_manager_state1d_check,
            8 * n,
            &state1d_fixture,
        );
        let arms: Vec<(u32, u64)> = (0..2 * n)
            .map(|seed| (seed, STATE1D_CASES[(seed % n) as usize].cycles))
            .collect();
        mc_require_arms(ee::s06::f_002402d0__generated, &arms, &state1d_fixture);
    }

    // --------------------------------------------------------- MCManager_ctor

    #[test]
    fn mc_manager_ctor_matches() {
        check(
            ee::s06::f_00242288__generated,
            mc_manager_ctor,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x242288, 0);
                // Junk over the object and around it, so every store shows.
                let this = MC_FIXTURE_THIS + (seed % 4) * 0x40;
                fill(c, &mut r, DATA + 0x300, 0x260);
                // The floats SetMode and the constructor load, and the global it
                // clears.
                fill(c, &mut r, GAME_GP.wrapping_sub(0x5130), 8);
                c.bus.write32(GAME_GP.wrapping_sub(0x478), r.next_u32() | 1);
                c.g[A0][0] = sx(this);
                c.pc = 0x0024_2288;
            },
        );
    }
}
