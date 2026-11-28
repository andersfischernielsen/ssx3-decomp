//! `src/worldpainter/quadtree.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
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
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::be::beprepoststate::gp_rel;
pub(crate) use crate::game::bx::bxstring::gp as gp_4;
pub(crate) use crate::game::bx::bxstringctor::MESSAGE_MAP;
pub(crate) use crate::game::camera::camera::gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::p_8;
pub(crate) use crate::game::dirtysock::tagsunk::gp_9 as gp_3;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::gp_11 as gp_5;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::soundsys::p as p_7;
pub(crate) use crate::game::visualfx::renderstateman::STRLEN_AND_STRCPY;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_2;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002c_2088, quad_tree_flatten_node, "QuadTree_FlattenNode"),
    (
        0x002c_2210,
        mem_card_ascii_to_sjis_string,
        "MemCard_AsciiToSjisString",
    ),
    (
        0x002c_2268,
        memcard_sjis_to_ascii_string,
        "MemCard_SjisToAsciiString",
    ),
    (0x002c_22c8, c_ps2_mem_card_create, "cPS2MemCard_Create"),
    (0x002c_2300, c_ps2_mem_card_destroy, "cPS2MemCard_Destroy"),
    (
        0x002c_2338,
        memcard_ascii_char_to_sjis,
        "MemCard_AsciiCharToSjis",
    ),
    (
        0x002c_2430,
        memcard_sjis_char_to_ascii,
        "MemCard_SjisCharToAscii",
    ),
    (0x002c_24d0, ustr_len_2, "UStr_Len_2"),
    (0x002c_2540, ustr_from_ascii_2, "UStr_FromAscii_2"),
    (0x002c_25b8, ustr_copy_n, "UStr_CopyN"),
    (0x002c_2688, ustr_cat, "UStr_Cat"),
    (0x002c_2868, str_to_lower_in_place, "Str_ToLowerInPlace"),
    (0x002c_3fa8, c_ps2_mem_card_ctor, "cPS2MemCard_ctor"),
    (0x002c_3fe0, c_ps2_mem_card_dtor, "cPS2MemCard_dtor"),
    (
        0x002c_4198,
        c_ps2_mem_card_request_list_dir_w,
        "cPS2MemCard_RequestListDirW",
    ),
    (
        0x002c_41d8,
        c_ps2_mem_card_request_list_dir,
        "cPS2MemCard_RequestListDir",
    ),
    (
        0x002c_4210,
        c_ps2_mem_card_do_list_dir,
        "cPS2MemCard_DoListDir",
    ),
    (
        0x002c_42b0,
        cps2_mem_card_do_get_save_dir_entry,
        "cPS2MemCard_DoGetSaveDirEntry",
    ),
    (
        0x002c_4388,
        c_ps2_mem_card_do_format,
        "cPS2MemCard_DoFormat",
    ),
    (
        0x002c_4480,
        ps2_memcard_request_get_info,
        "cPS2MemCard_RequestGetInfo",
    ),
    (
        0x002c_44e0,
        c_ps2_mem_card_clear_port_changed,
        "cPS2MemCard_ClearPortChanged",
    ),
    (0x002c_4520, c_ps2_mem_card_v34, "cPS2MemCard_v34"),
    (0x002c_4578, c_ps2_mem_card_v35, "cPS2MemCard_v35"),
    (0x002c_45d0, cps2_mem_card_v37, "cPS2MemCard_v37"),
    (0x002c_4648, cps2_mem_card_v36, "cPS2MemCard_v36"),
    (0x002c_46c0, ps2_mem_card_v38, "cPS2MemCard_v38"),
    (
        0x002c_4748,
        c_ps2_mem_card_is_state34,
        "cPS2MemCard_IsState34",
    ),
    (
        0x002c_4788,
        cps2_mem_card_request_read_save,
        "cPS2MemCard_RequestReadSave",
    ),
    (0x002c_4800, c_ps2_mem_card_v29, "cPS2MemCard_v29"),
    (
        0x002c_4880,
        c_ps2_mem_card_is_state35,
        "cPS2MemCard_IsState35",
    ),
    (
        0x002c_50e0,
        ps2_memcard_do_get_info,
        "cPS2MemCard_DoGetInfo",
    ),
    (0x002c_5140, c_ps2_mem_card_is_busy, "cPS2MemCard_IsBusy"),
    (
        0x002c_5168,
        c_ps2_mem_card_is_port_card_error,
        "cPS2MemCard_IsPortCardError",
    ),
    (
        0x002c_51d0,
        cps2memcard_get_port_error,
        "cPS2MemCard_GetPortError",
    ),
    (
        0x002c_5250,
        cps2memcard_clear_port_info,
        "cPS2MemCard_ClearPortInfo",
    ),
    (
        0x002c_5278,
        ps2_mem_card_clear_dir_cache,
        "cPS2MemCard_ClearDirCache",
    ),
    (
        0x002c_52d8,
        c_ps2_mem_card_clear_dir_entry,
        "cPS2MemCard_ClearDirEntry",
    ),
    (
        0x002c_5300,
        c_ps2_mem_card_is_port_not_ps2_card,
        "cPS2MemCard_IsPortNotPS2Card",
    ),
    (
        0x002c_5320,
        cps2memcard_get_free_space_for_port,
        "cPS2MemCard_GetFreeSpaceForPort",
    ),
    (0x002c_5358, c_ps2_mem_card_v52, "cPS2MemCard_v52"),
    (
        0x002c_53b0,
        c_ps2_mem_card_is_port_formatted,
        "cPS2MemCard_IsPortFormatted",
    ),
    (
        0x002c_53c8,
        cps2_mem_card_get_port_changed_flag,
        "cPS2MemCard_GetPortChangedFlag",
    ),
    (
        0x002c_5528,
        c_ps2_mem_card_has_slot_name,
        "cPS2MemCard_HasSlotName",
    ),
    (
        0x002c_5558,
        c_ps2_mem_card_mark_slot_used,
        "cPS2MemCard_MarkSlotUsed",
    ),
    (
        0x002c_5570,
        cps2memcard_mark_save_invalid,
        "cPS2MemCard_MarkSaveInvalid",
    ),
    (
        0x002c_5588,
        c_ps2_mem_card_is_slot_empty,
        "cPS2MemCard_IsSlotEmpty",
    ),
    (
        0x002c_55c0,
        c_ps2_mem_card_slot_has_1ac,
        "cPS2MemCard_SlotHas1AC",
    ),
    (
        0x002c_6280,
        cps2memcard_mark_slots_without_space,
        "cPS2MemCard_MarkSlotsWithoutSpace",
    ),
    (0x002c_6360, c_ps2_mem_card_close, "cPS2MemCard_Close"),
    (
        0x002c_63e8,
        cps2memcard_is_mc_error,
        "cPS2MemCard_IsMcError",
    ),
    (
        0x002c_63f8,
        cps2_memcard_get_slot_name,
        "cPS2MemCard_GetSlotName",
    ),
    (0x002c_64a8, c_ps2_mem_card_v15, "cPS2MemCard_v15"),
    (0x002c_64d0, c_ps2_mem_card_v16, "cPS2MemCard_v16"),
    (0x002c_66b8, c_ps2_mem_card_v39, "cPS2MemCard_v39"),
    (
        0x002c_66d8,
        c_ps2_mem_card_is_port_result_m11,
        "cPS2MemCard_IsPortResultM11",
    ),
    (
        0x002c_6758,
        c_ps2_mem_card_calc_save_size_kb,
        "cPS2MemCard_CalcSaveSizeKB",
    ),
    (
        0x002c_6810,
        c_ps2_mem_card_request_list_dir_all,
        "cPS2MemCard_RequestListDirAll",
    ),
    (
        0x002c_6848,
        c_ps2_mem_card_do_list_dir_all,
        "cPS2MemCard_DoListDirAll",
    ),
    (
        0x002c_68c8,
        cps2_memcard_get_dir_entry_name,
        "cPS2MemCard_GetDirEntryName",
    ),
    (
        0x002c_68f0,
        cps2_mem_card_count_saves,
        "cPS2MemCard_CountSaves",
    ),
    (
        0x002c_6968,
        c_ps2_mem_card_request_open,
        "cPS2MemCard_RequestOpen",
    ),
    (0x002c_69a0, cps2_mem_card_do_open, "cPS2MemCard_DoOpen"),
    (0x002c_6a28, c_ps2_mem_card_do_close, "cPS2MemCard_DoClose"),
    (0x002c_6a88, c_ps2_mem_card_do_seek, "cPS2MemCard_DoSeek"),
    (0x002c_6b00, c_ps2_mem_card_do_flush, "cPS2MemCard_DoFlush"),
    (0x002c_6b78, c_ps2_mem_card_do_read, "cPS2MemCard_DoRead"),
    (0x002c_6bf8, c_ps2_mem_card_do_write, "cPS2MemCard_DoWrite"),
    (
        0x002c_6c50,
        c_ps2_mem_card_request_mkdir,
        "cPS2MemCard_RequestMkdir",
    ),
    (0x002c_6c88, c_ps2_mem_card_do_mkdir, "cPS2MemCard_DoMkdir"),
    (
        0x002c_6ce0,
        c_ps2_mem_card_request_delete,
        "cPS2MemCard_RequestDelete",
    ),
    (
        0x002c_6d18,
        c_ps2_mem_card_do_delete,
        "cPS2MemCard_DoDelete",
    ),
    (0x002c_6d70, loc_string_table_dtor, "LocStringTable_dtor"),
    (0x002c_6de0, loc_string_table_find, "LocStringTable_Find"),
    (0x002c_55d8, cps2_mem_card_update, "cPS2MemCard_Update"),
    (
        0x003d_bb68,
        loc_string_table_get_string_ptr,
        "LocStringTable_GetStringPtr",
    ),
    (0x003d_c450, ustr_vsprintf, "UStr_vsprintf"),
    (
        0x002c_2f18,
        surf_prop_cache_init_defaults,
        "SurfPropCache_InitDefaults",
    ),
    (0x003d_bd18, u_str_find, "UStr_Find"),
    (0x003d_bdb0, u_str_cmp, "UStr_Cmp"),
    (0x003d_bdd0, u_str_n_cmp, "UStr_NCmp"),
    (0x003d_be20, u_str_cpy, "UStr_Cpy"),
    (0x003d_be40, u_str_n_cpy, "UStr_NCpy"),
    (0x003d_be90, u_str_from_ascii, "UStr_FromAscii"),
    (
        0x003d_bed0,
        u_str_substitute_args_wrapper,
        "UStr_SubstituteArgs_Wrapper",
    ),
    (0x003d_bef0, u_str_substitute_args, "UStr_SubstituteArgs"),
    (0x003d_bfd8, u_str_format_u_int, "UStr_FormatUInt"),
    (0x003d_c038, u_str_format_int, "UStr_FormatInt"),
    (0x003d_c068, u_str_format_float, "UStr_FormatFloat"),
    (
        0x003d_c148,
        u_str_format_number_field,
        "UStr_FormatNumberField",
    ),
    (0x003d_c9f8, u_str_sprintf, "UStr_sprintf"),
    (
        0x003d_ca40,
        u_str_from_escaped_ascii,
        "UStr_FromEscapedAscii",
    ),
    (0x003d_cb20, u_str_to_escaped_ascii, "UStr_ToEscapedAscii"),
    (0x002c_28a0, game_link_get_ticks, "GameLink_GetTicks"),
    (0x002c_28c0, game_link_alloc, "GameLink_Alloc"),
    (0x002c_28f8, game_link_free, "GameLink_Free"),
    (0x002c_2918, game_link_mem_copy, "GameLink_MemCopy"),
    (0x002c_2940, game_link_realloc, "GameLink_Realloc"),
    (
        0x002c_29d0,
        game_link_create_thread,
        "GameLink_CreateThread",
    ),
    (
        0x002c_2aa8,
        game_link_destroy_thread,
        "GameLink_DestroyThread",
    ),
    (0x002c_2b08, game_link_yield, "GameLink_Yield"),
    (0x002c_2b28, game_link_create_mutex, "GameLink_CreateMutex"),
    (
        0x002c_2b68,
        game_link_destroy_mutex,
        "GameLink_DestroyMutex",
    ),
    (0x002c_2b98, game_link_lock_mutex, "GameLink_LockMutex"),
    (0x002c_2bb8, game_link_unlock_mutex, "GameLink_UnlockMutex"),
    (
        0x002c_2bd8,
        game_link_setup_platform_funcs,
        "GameLink_SetupPlatformFuncs",
    ),
    (0x002c_4050, c_ps2_mem_card_init, "cPS2MemCard_Init"),
    (0x002c_4290, c_ps2_mem_card_v76, "cPS2MemCard_v76"),
    (
        0x002c_4368,
        c_ps2_mem_card_request_format,
        "cPS2MemCard_RequestFormat",
    ),
    (0x002c_43e0, c_ps2_mem_card_v10, "cPS2MemCard_v10"),
    (
        0x002c_4410,
        c_ps2_mem_card_do_delete_save,
        "cPS2MemCard_DoDeleteSave",
    ),
    (0x002c_4720, c_ps2_mem_card_v41, "cPS2MemCard_v41"),
    (0x002c_4768, c_ps2_mem_card_v43, "cPS2MemCard_v43"),
    (0x002c_4858, c_ps2_mem_card_v30, "cPS2MemCard_v30"),
    (0x002c_48a0, c_ps2_mem_card_v32, "cPS2MemCard_v32"),
    (0x002c_48c0, cps2_mem_card_set_state, "cPS2MemCard_SetState"),
    (0x002c_5230, c_ps2_mem_card_v73, "cPS2MemCard_v73"),
    (
        0x002c_5338,
        c_ps2_mem_card_get_free_space,
        "cPS2MemCard_GetFreeSpace",
    ),
    (
        0x002c_53e0,
        c_ps2_mem_card_get_free_space_string,
        "cPS2MemCard_GetFreeSpaceString",
    ),
    (
        0x002c_54d0,
        c_ps2_mem_card_get_save_name,
        "cPS2MemCard_GetSaveName",
    ),
    (0x002c_5500, c_ps2_mem_card_v45, "cPS2MemCard_v45"),
    (
        0x002c_6128,
        c_ps2_mem_card_get_status_string,
        "cPS2MemCard_GetStatusString",
    ),
    (0x002c_64f8, c_ps2_mem_card_v17, "cPS2MemCard_v17"),
    (0x002c_6718, c_ps2_mem_card_v62, "cPS2MemCard_v62"),
    (0x002c_6778, c_ps2_mem_card_v23, "cPS2MemCard_v23"),
    (0x002c_67b0, c_ps2_mem_card_v51, "cPS2MemCard_v51"),
    (
        0x002c_67e8,
        c_ps2_mem_card_request_get_info_now,
        "cPS2MemCard_RequestGetInfoNow",
    ),
    (
        0x002c_6a08,
        c_ps2_mem_card_request_close,
        "cPS2MemCard_RequestClose",
    ),
    (
        0x002c_6a60,
        c_ps2_mem_card_request_seek,
        "cPS2MemCard_RequestSeek",
    ),
    (
        0x002c_6ae0,
        c_ps2_mem_card_request_flush,
        "cPS2MemCard_RequestFlush",
    ),
    (
        0x002c_6b50,
        c_ps2_mem_card_request_read,
        "cPS2MemCard_RequestRead",
    ),
    (
        0x002c_6bd0,
        c_ps2_mem_card_request_write,
        "cPS2MemCard_RequestWrite",
    ),
    (
        0x002d_3788,
        c_ps2_mem_card_base_dtor,
        "cPS2MemCard_BaseDtor",
    ),
    (0x002d_37e0, c_ps2_mem_card_v49, "cPS2MemCard_v49"),
    (0x002d_3810, c_ps2_mem_card_v53, "cPS2MemCard_v53"),
    (0x002d_3840, c_ps2_mem_card_v54, "cPS2MemCard_v54"),
    (0x002d_38f0, c_ps2_mem_card_v55, "cPS2MemCard_v55"),
    (0x002d_3930, c_ps2_mem_card_v63, "cPS2MemCard_v63"),
    (0x002d_3950, c_ps2_mem_card_v64, "cPS2MemCard_v64"),
    (0x002d_3980, c_ps2_mem_card_v71, "cPS2MemCard_v71"),
    (
        0x002d_39b8,
        c_ps2_mem_card_get_current_port_error,
        "cPS2MemCard_GetCurrentPortError",
    ),
    (
        0x002c_6dc0,
        loc_string_table_resolve_string,
        "LocStringTable_ResolveString",
    ),
    (0x002c_2580, ustr_to_ascii, "UStr_ToAscii"),
    (0x002c_6e98, loc_string_table_load, "LocStringTable_Load"),
    (0x002c_17d0, quad_tree_alloc_node, "QuadTree_AllocNode"),
    (0x002c_1950, quad_tree_insert, "QuadTree_Insert"),
    (0x002c_1da8, quad_tree_compact, "QuadTree_Compact"),
    (0x002c_2178, quad_tree_count_nodes, "QuadTree_CountNodes"),
    (
        0x002c_3fa0,
        game_module_unload_stub,
        "GameModule_UnloadStub",
    ),
    (0x002c_4400, ps2_mem_card_get_490, "cPS2MemCard_Get490"),
    (0x002c_4408, ps2_mem_card_set_24, "cPS2MemCard_Set24"),
    (
        0x002c_4758,
        ps2_mem_card_defer_state_18,
        "cPS2MemCard_DeferState18",
    ),
    (
        0x002c_4890,
        ps2_mem_card_defer_state_1e,
        "cPS2MemCard_DeferState1E",
    ),
    (0x002c_5520, ps2_mem_card_v46, "cPS2MemCard_v46"),
    (0x002c_6700, ps2_mem_card_v57, "cPS2MemCard_v57"),
    (0x002c_6708, ps2_mem_card_v58, "cPS2MemCard_v58"),
    (0x002c_6710, ps2_mem_card_v61, "cPS2MemCard_v61"),
    (
        0x002c_6748,
        ps2_mem_card_get_block_size,
        "cPS2MemCard_GetBlockSize",
    ),
    (0x002c_6750, ps2_mem_card_v21, "cPS2MemCard_v21"),
    (0x002c_67a8, ps2_mem_card_v50, "cPS2MemCard_v50"),
    (0x002c_67e0, ps2_mem_card_v25, "cPS2MemCard_v25"),
    (
        0x002d_37d8,
        ps2_mem_card_get_current_port,
        "cPS2MemCard_GetCurrentPort",
    ),
    (0x002d_3920, ps2_mem_card_v59, "cPS2MemCard_v59"),
    (0x002d_3928, ps2_mem_card_v60, "cPS2MemCard_v60"),
    (0x002d_3970, ps2_mem_card_get_14, "cPS2MemCard_Get14"),
    (0x002d_3978, ps2_mem_card_v69, "cPS2MemCard_v69"),
    (0x002d_39a8, ps2_mem_card_v72, "cPS2MemCard_v72"),
    (
        0x002d_39b0,
        ps2_mem_card_is_sync_done,
        "cPS2MemCard_IsSyncDone",
    ),
    (
        0x002c_1a00,
        quad_tree_insert_recursive_oc1599,
        "QuadTree_InsertRecursive",
    ),
    (0x002c_2d30, istr_alloc, "ISTR_Alloc"),
    (0x002c_2d68, istr_free, "ISTR_Free"),
    (0x002c_2d88, istr_mem_copy, "ISTR_MemCopy"),
    (0x002c_2db0, istr_realloc, "ISTR_Realloc"),
    (
        0x003d_b9d8,
        loc_string_table_get_info,
        "LocStringTable_GetInfo",
    ),
    (
        0x003d_bac8,
        loc_string_table_compare_id_003dbac8,
        "LocStringTable_CompareId",
    ),
    (
        0x003d_bae8,
        loc_string_table_remap_id,
        "LocStringTable_RemapId",
    ),
];

/// Direct and cross-shard callees, by guest address.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;

    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::C_PS2_MEM_CARD_SET_STATE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCE_MC_FORMAT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SPRINTF_2 as SPRINTF;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::STRCPY_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;

    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;

    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;

    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;

    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;

    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE as CPS2_MEM_CARD_SET_STATE_2;

    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_3;

    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;

    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;

    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;

    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;

    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;

    /// The engine's named-block allocator (`cMemMan_alloc`).
    pub const EA_MEM_ALLOC: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };

    pub const BUILTIN_DELETE_2: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const PS2_SOUND_MANAGER_CTOR: Callee = Callee {
        addr: 0x002a_8c20,
        f: ee::s08::PS2SoundManager_ctor,
    };

    pub const STREAM_BUFFER_SET_FILE: Callee = Callee {
        addr: 0x002a_9e78,
        f: ee::s08::StreamBuffer_SetFile,
    };

    pub const STREAM_BUFFER_STOP: Callee = Callee {
        addr: 0x002a_a408,
        f: ee::s08::StreamBuffer_Stop,
    };

    pub const SPEECH_INSTANCE_STOP_STREAM: Callee = Callee {
        addr: 0x002b_0320,
        f: ee::s08::SpeechInstance_StopStream,
    };

    pub const SONG_GET_STREAM_STATUS: Callee = Callee {
        addr: 0x002b_2578,
        f: ee::s08::SongInstance_GetStreamStatus,
    };

    pub const SONG_POST_EVENT: Callee = Callee {
        addr: 0x002b_20c8,
        f: ee::s08::SongInstance_PostEvent,
    };

    pub const SONG_FADE_OUT: Callee = Callee {
        addr: 0x002b_2418,
        f: ee::s08::SongInstance_FadeOut,
    };

    /// `SNDSTRM_GetRequestStatus` (0x3b7c40, another shard).
    pub const SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;

    pub const MEM_CARD_CTOR: Callee = Callee {
        addr: 0x002c_3fa8,
        f: ee::s08::cPS2MemCard_ctor,
    };

    pub const MEM_CARD_INIT: Callee = Callee {
        addr: 0x002c_4050,
        f: ee::s08::cPS2MemCard_Init,
    };

    pub const MEM_CARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::cPS2MemCard_SetState,
    };

    pub const CONSOLE_CONFIG_CTOR: Callee = Callee {
        addr: 0x002c_7478,
        f: ee::s08::cPS2ConsoleConfig_ctor,
    };

    /// `strcpy` (0x4166f4, s15).
    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub const MENU_ITEM_MEASURE_ROW: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::MenuSysItem_MeasureRow,
    };

    pub const STATS_MAN_CTOR: Callee = Callee {
        addr: 0x002d_21c0,
        f: ee::s09::SSXStatsMan_ctor,
    };

    pub const AI_VFX_MAN_CTOR: Callee = Callee {
        addr: 0x002f_03c8,
        f: ee::s09::PS2AIVisualEffectsMan_ctor,
    };

    pub const VFX_MAN_CTOR: Callee = Callee {
        addr: 0x002f_0548,
        f: ee::s09::PS2VisualEffectsMan_ctor,
    };

    /// `World_GetSectionScriptFunc` (0x3a6d18, s13).
    pub const WORLD_GET_SECTION_SCRIPT_FUNC: u32 = 0x003a_6d18;

    pub const WORLD_OBJ_CTOR: Callee = Callee {
        addr: 0x0034_fb90,
        f: ee::s11::WorldObj_ctor_2,
    };

    pub const MOD_OBJ_CTOR: Callee = Callee {
        addr: 0x0035_52d8,
        f: ee::s11::ModObj_ctor_2,
    };

    pub const GET_TRANSFORM_BOUNDS: Callee = Callee {
        addr: 0x0035_2b88,
        f: ee::s11::ModifierBlock_GetTransformBounds,
    };

    pub const INTC_REMOVE_HANDLER: Callee = Callee {
        addr: 0x0036_2008,
        f: ee::s12::IntcTable_RemoveHandler,
    };

    pub const EMITTER_RENDER: Callee = Callee {
        addr: 0x0037_08c0,
        f: ee::s12::Emitter_Render,
    };

    pub const GRAPHICS_ON_VIF1_DMA_END: Callee = Callee {
        addr: 0x0038_2688,
        f: ee::s12::PS2GraphicsMan_OnVif1DmaEnd,
    };

    /// `PS2GraphicsMan_ctor` (0x395288, s13).
    pub const GRAPHICS_MAN_CTOR: u32 = 0x0039_5288;

    pub(crate) const U_STR_LEN: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::UStr_Len,
    };

    pub(crate) const U_STR_N_CMP: Callee = Callee {
        addr: 0x003d_bdd0,
        f: ee::s14::UStr_NCmp,
    };

    pub(crate) const U_STR_CPY: Callee = Callee {
        addr: 0x003d_be20,
        f: ee::s14::UStr_Cpy,
    };

    pub(crate) const U_STR_N_CPY: Callee = Callee {
        addr: 0x003d_be40,
        f: ee::s14::UStr_NCpy,
    };

    pub(crate) const U_STR_SUBSTITUTE_ARGS: Callee = Callee {
        addr: 0x003d_bef0,
        f: ee::s14::UStr_SubstituteArgs,
    };

    pub(crate) const U_STR_FORMAT_U_INT: Callee = Callee {
        addr: 0x003d_bfd8,
        f: ee::s14::UStr_FormatUInt,
    };

    pub(crate) const U_STR_FORMAT_INT: Callee = Callee {
        addr: 0x003d_c038,
        f: ee::s14::UStr_FormatInt,
    };

    pub(crate) const U_STR_FORMAT_FLOAT: Callee = Callee {
        addr: 0x003d_c068,
        f: ee::s14::UStr_FormatFloat,
    };

    pub(crate) const U_STR_VSPRINTF: Callee = Callee {
        addr: 0x003d_c450,
        f: ee::s14::UStr_vsprintf,
    };

    pub(crate) const EA_FILE_CALC_MEM_SIZE: Callee = Callee {
        addr: 0x003d_cd98,
        f: ee::s14::EAFile_CalcMemSize,
    };

    pub(crate) const EA_FILE_INIT: Callee = Callee {
        addr: 0x003d_cc88,
        f: ee::s14::EAFile_Init,
    };

    pub(crate) const EA_FILE_CLOSE: Callee = Callee {
        addr: 0x003d_eca0,
        f: ee::s14::EAFile_Close,
    };

    pub(crate) const EA_MUTEX_INIT: Callee = Callee {
        addr: 0x003e_5698,
        f: ee::s14::EAMutex_Init,
    };

    pub(crate) const EA_MUTEX_DESTROY: Callee = Callee {
        addr: 0x003e_56e0,
        f: ee::s14::EAMutex_Destroy,
    };

    pub(crate) const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };

    pub(crate) const EA_MEM_ZERO: Callee = Callee {
        addr: 0x003e_6220,
        f: ee::s14::EA_MemZero,
    };

    pub(crate) const EA_FILE_CD_INIT: Callee = Callee {
        addr: 0x003e_4040,
        f: ee::s14::EAFile_CdInit,
    };

    pub(crate) const EA_FILE_PLATFORM_INIT: Callee = Callee {
        addr: 0x003e_3020,
        f: ee::s14::EAFile_PlatformInit,
    };

    /// `fcvt` (newlib), called through the function table.
    pub(crate) const FCVT: u32 = 0x0041_8d48;

    /// `strlen` (newlib), called through the function table.
    pub(crate) const STRLEN: u32 = 0x0041_6810;

    /// `__fixunsdfsi` (gcc), called through the function table.
    pub(crate) const FIXUNSDFSI: u32 = 0x0041_26a0;

    /// `__fixdfsi` (gcc), called through the function table.
    pub(crate) const FIXDFSI: u32 = 0x0041_2608;

    /// `__cmpdf2` (gcc), called through the function table.
    pub(crate) const CMPDF2: u32 = 0x0041_2500;
    pub(crate) use super::table::MEM_FREE;
    pub(crate) use crate::game::be::belibrary::EA_MEM_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;

    /// `cPS2MemCard_DoGetInfo` (0x2c50e0).
    pub(crate) const C_PS2_MEM_CARD_DO_GET_INFO: Callee = Callee {
        addr: 0x002c_50e0,
        f: ee::s08::cPS2MemCard_DoGetInfo,
    };

    /// `MemCard_AsciiToSjisString` (0x2c2210).
    pub(crate) const MEM_CARD_ASCII_TO_SJIS_STRING: Callee = Callee {
        addr: 0x002c_2210,
        f: ee::s08::MemCard_AsciiToSjisString,
    };

    /// `AsyncFile_Load` (0x3df690, s14), through the function table.
    pub(crate) const ASYNC_FILE_LOAD: u32 = 0x003d_f690;

    /// `sceMcOpen` (0x409d10, s15), through the function table.
    pub(crate) const SCE_MC_OPEN: u32 = 0x0040_9d10;

    /// `sceMcGetDir` (0x40a688, s15), through the function table.
    pub(crate) const SCE_MC_GET_DIR: u32 = 0x0040_a688;

    /// `cPS2MemCard_GetFreeSpaceForPort` (0x2c5320).
    pub(crate) const C_PS2_MEM_CARD_GET_FREE_SPACE_FOR_PORT: Callee = Callee {
        addr: 0x002c_5320,
        f: ee::s08::cPS2MemCard_GetFreeSpaceForPort,
    };

    /// `cPS2MemCard_GetPortChangedFlag` (0x2c53c8).
    pub(crate) const C_PS2_MEM_CARD_GET_PORT_CHANGED_FLAG: Callee = Callee {
        addr: 0x002c_53c8,
        f: ee::s08::cPS2MemCard_GetPortChangedFlag,
    };

    /// `cPS2MemCard_GetFreeSpaceString` (0x2c53e0).
    pub(crate) const C_PS2_MEM_CARD_GET_FREE_SPACE_STRING: Callee = Callee {
        addr: 0x002c_53e0,
        f: ee::s08::cPS2MemCard_GetFreeSpaceString,
    };
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;

    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstringctor::addr::MC_MANAGER_GET_INFO;

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
}

// ---------------------------------------------------------------------------
// Memory card / character conversion.

/// `short MemCard_AsciiCharToSjis(uchar ch)` (0x2c2338): map an ASCII byte to
/// its SJIS code through two static tables, 0 for unmapped bytes.
pub fn memcard_ascii_char_to_sjis(c: &mut Cpu) {
    c.cycles += 6;
    let ch = abi::arg(c, 0) & 0xff;
    c.g[A2][0] = ch as u64;
    c.g[V1][0] = 0;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(ch.wrapping_sub(0x20));
    c.g[V0][0] = (c.g[V0][0] < 0x10) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V1][0] = 0x1;
    } else {
        c.cycles += 4;
        c.g[V0][0] = sx(ch.wrapping_sub(0x30));
        let digits = (c.g[V0][0] < 0xa) as u64;
        c.g[V0][0] = sx(ch.wrapping_sub(0x3a));
        if digits == 0 {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x7) as u64;
            let not_punct = c.g[V0][0] == 0;
            c.g[V0][0] = sx(ch.wrapping_sub(0x41));
            if !not_punct {
                c.cycles += 2;
                c.g[V1][0] = 0xb;
            } else {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let not_upper = c.g[V0][0] == 0;
                c.g[V0][0] = sx(ch.wrapping_sub(0x5b));
                if !not_upper {
                    c.cycles += 2;
                    c.g[A0][0] = 0x1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x6) as u64;
                    let not_lbracket = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(ch.wrapping_sub(0x61));
                    if !not_lbracket {
                        c.cycles += 2;
                        c.g[V1][0] = 0x25;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                        let not_lower = c.g[V0][0] == 0;
                        c.g[V0][0] = sx(ch.wrapping_sub(0x7b));
                        if !not_lower {
                            c.cycles += 2;
                            c.g[A0][0] = 0x2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V0][0] < 0x4) as u64;
                            c.g[V1][0] = 0x3f;
                            if c.g[V0][0] == 0 {
                                c.cycles += 2;
                                c.g[V0][0] = 0;
                                abi::ret(c);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    // End of the mapping chain: byte-swap the mapped half-word.
    c.cycles += 2;
    if c.g[V1][0] == 0 {
        c.g[V1][0] = 0x440000;
        c.cycles += 9;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5940));
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2)).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(ch));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        c.g[V0][0] &= 0xffff;
    } else {
        c.cycles += 8;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
        c.g[V1][0] = 0x440000;
        c.g[V0][0] = sx(ch.wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5950));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lhu(c);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[V1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    abi::ret(c);
}

/// `uchar MemCard_SjisCharToAscii(ushort sjis)` (0x2c2430): map a two-byte SJIS
/// code back to ASCII.
pub fn memcard_sjis_char_to_ascii(c: &mut Cpu) {
    c.cycles += 6;
    let code = abi::arg(c, 0);
    let lead = (code >> 8) & 0xff;
    let trail = code & 0xff;
    c.g[V0][0] = sx(code >> 8);
    c.g[A2][0] = lead as u64;
    c.g[A0][0] = trail as u64;
    c.g[A1][0] = 0;
    if trail != 0x81 {
        c.cycles += 3;
        if trail != 0x82 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        let mut a1: u32 = 0;
        if lead.wrapping_sub(0x4f) >= 0xa {
            c.cycles += 4;
            if lead.wrapping_sub(0x60) < 0x1a {
                a1 = 1;
            } else {
                c.cycles += 6;
                let v = lead.wrapping_add(0x7f) & 0xff;
                a1 = if 0x19 < v { 0 } else { 2 };
            }
            c.cycles += 1;
            a1 <<= 1;
        }
        c.g[A1][0] = a1 as u64;
        c.cycles += 10;
        let gp = c.g[GP][0] as u32;
        c.g[V0][0] = sx(gp.wrapping_add(0x7a0).wrapping_add(a1));
        c.g[A0][0] = sx(gp.wrapping_add(0x7a1));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
        let a1b = a1.wrapping_add(gp.wrapping_add(0x7a1));
        c.g[A1][0] = sx(a1b);
        c.g[V0][0] = Addr(a1b).lbu(c);
        c.g[V1][0] = sx(lead.wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32)) & 0xff;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx(0x440000u32.wrapping_add(0x5998).wrapping_add(lead));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x40)).lbu(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory-card port accessors.

/// `int cPS2MemCard_GetPortError(cPS2MemCard* mc, int port)` (0x2c51d0):
/// classify the port's status word into an error code (-10 when unknown).
pub fn cps2memcard_get_port_error(c: &mut Cpu) {
    c.cycles += 8;
    let base = c.g[A0][0] as u32;
    let port = c.g[A1][0] as u32;
    c.g[V0][0] = 0x14;
    c.g[A2][0] = c.g[A0][0];
    let prod = abi::mult(c, port, 0x14);
    c.g[V1][0] = sx(prod);
    c.g[V0][0] = sx(prod.wrapping_add(base));
    let e = Addr(c.g[V0][0] as u32).off(0x184).lw(c);
    c.g[A0][0] = e;
    c.g[V1][0] = ((e as i64) < -7) as u64;
    c.g[V0][0] = ((e as i64) < -10) as u64;
    let retry = if c.g[V1][0] == 0 {
        // status >= -7
        c.cycles += 2;
        c.g[V1][0] = 0x14;
        if (e as i64) > 0 {
            false
        } else {
            c.cycles += 1;
            let p = abi::mult(c, c.g[A1][0] as u32, 0x14);
            c.g[V0][0] = sx(p);
            true
        }
    } else {
        c.cycles += 2;
        let v0_zero = (e as i64) >= -10;
        let below10k = (e as i64) < -10000;
        if v0_zero {
            false
        } else {
            c.cycles += 2;
            if !below10k {
                false
            } else {
                c.cycles += 2;
                c.g[V1][0] = 0x14;
                if (e as i64) < -10002 {
                    false
                } else {
                    c.cycles += 2;
                    let p = abi::mult(c, c.g[A1][0] as u32, 0x14);
                    c.g[V0][0] = sx(p);
                    true
                }
            }
        }
    };
    if !retry {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fff6;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x184).lw(c);
    abi::ret(c);
}

/// `void cPS2MemCard_ClearPortInfo(cPS2MemCard* mc)` (0x2c5250): clear the two
/// words at the current port's +0x17c/+0x180.
pub fn cps2memcard_clear_port_info(c: &mut Cpu) {
    c.cycles += 10;
    let base = c.g[A0][0] as u32;
    let h = Addr(base).off(0xc).lw(c);
    c.g[V0][0] = h;
    c.g[A1][0] = 0x14;
    let p1 = abi::mult(c, h as u32, 0x14);
    c.g[V1][0] = sx(p1);
    c.g[V0][0] = sx(p1.wrapping_add(base));
    Addr(c.g[V0][0] as u32).off(0x17c).sw(c, 0);
    let h2 = Addr(base).off(0xc).lw(c);
    c.g[V1][0] = h2;
    let p2 = abi::mult(c, h2 as u32, 0x14);
    c.g[V0][0] = sx(p2);
    c.g[V1][0] = sx(p2.wrapping_add(base));
    Addr(c.g[V1][0] as u32).off(0x180).sw(c, 0);
    abi::ret(c);
}

/// `int cPS2MemCard_GetFreeSpaceForPort(cPS2MemCard* mc, int port)`
/// (0x2c5320): read the port's free-space word at +0x180.
pub fn cps2memcard_get_free_space_for_port(c: &mut Cpu) {
    c.cycles += 5;
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = 0x14;
    let prod = abi::mult(c, c.g[A1][0] as u32, 0x14);
    c.g[V1][0] = sx(prod);
    c.g[A0][0] = sx(prod.wrapping_add(base));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x180).lw(c);
    abi::ret(c);
}

/// `void cPS2MemCard_MarkSaveInvalid(cPS2MemCard* mc, int port)`
/// (0x2c5570): set the port's invalid flag at +0x1a8.
pub fn cps2memcard_mark_save_invalid(c: &mut Cpu) {
    c.cycles += 6;
    let base = c.g[A0][0] as u32;
    c.g[V0][0] = 0x78;
    c.g[V1][0] = 0x1;
    let prod = abi::mult(c, c.g[A1][0] as u32, 0x78);
    c.g[A2][0] = sx(prod);
    c.g[A0][0] = sx(prod.wrapping_add(base));
    Addr(c.g[A0][0] as u32).off(0x1a8).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `bool cPS2MemCard_IsMcError(int err)` (0x2c63e8): whether `err` is in the
/// MC error range `(-0x270f, -0x2709)`.
pub fn cps2memcard_is_mc_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x270f));
    c.g[V0][0] = (c.g[V0][0] < 0x2706) as u64;
    abi::ret(c);
}

/// `int cPS2MemCard_GetPortChangedFlag(cPS2MemCard* m, int port)` (0x2c53c8):
/// whether a card port (0x14-byte slots, flag at +0x18c) changed.
pub fn cps2_mem_card_get_port_changed_flag(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0); // v1 = port * 0x14
    let selfp = c.g[A0][0] as u32;
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18c));
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `void cPS2MemCard_v39(this, const char *name, int flag)` (0x002c66b8):
/// cache `name` at `+0x14c0` when `flag == 0`.
pub fn c_ps2_mem_card_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let named = c.g[A2][0] != 0;
        frame.save_ra(c, 0);
        if !named {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
            call_ptr(c, callee::STRLEN_AND_STRCPY, 0x002c_66cc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_ClearDirEntry(cPS2MemCard *this)` (0x2c52d8): zero the
/// 0x40-byte directory entry at `this + 0x480`.
pub fn c_ps2_mem_card_clear_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x480));
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x40;
        call_ptr(c, table_callee::EA_MEMSET, 0x002c_52f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two `cPS2MemCard` string accessors: build the UTF-16
/// string for the ASCII text in `a2` (the `a0`/`a1` arguments are unused).
pub(crate) fn mem_card_string(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    c.g[A1][0] = MESSAGE_MAP as u64;
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A2][0];
    c.g[A1][0] = sx(MESSAGE_MAP.wrapping_add(0x3968));
    callee::USTR_FROM_ASCII_2.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `UStr cPS2MemCard_v15(cPS2MemCard *this, int unused, char *ascii)`
/// (0x2c64a8): `UStr_FromAscii_2(ascii, D_004A3968)`.
pub fn c_ps2_mem_card_v15(c: &mut Cpu) {
    guest(c, |c| mem_card_string(c, 0x002c_64c0))
}

/// `UStr cPS2MemCard_v16(cPS2MemCard *this, int unused, char *ascii)`
/// (0x2c64d0): `UStr_FromAscii_2(ascii, D_004A3968)`.
pub fn c_ps2_mem_card_v16(c: &mut Cpu) {
    guest(c, |c| mem_card_string(c, 0x002c_64e8))
}

// ---------------------------------------------------------------------------
// MemCard and menus.

/// `void cPS2MemCard_GetDirEntryName(self, int i, char* dst)` (0x2c68c8):
/// copy the `i`-th 64-byte directory entry name (at `+0x4e0`) into `dst`.
pub fn cps2_memcard_get_dir_entry_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(abi::arg(c, 0)));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        let src = c.g[A1][0] as u32;
        c.g[A1][0] = sx(at(src, 0x4e0));
        call_ptr(c, callee::STRLEN_COPY, 0x002c_68e4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Allocating constructors.

/// `new (size, name, flags)` then construct: the shape of every `Create` in
/// this module (`EA_MemAllocNamed` then the class constructor).
pub(crate) fn create_with<F>(
    c: &mut Cpu,
    size: u32,
    name: u32,
    flags: u32,
    ret: u32,
    ctor: F,
) -> Flow
where
    F: FnOnce(&mut Cpu, u32) -> Flow,
{
    c.cycles += 13;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = sx(name);
    c.g[A2][0] = sx(flags);
    c.g[A3][0] = 0;
    c.g[A0][0] = sx(size);
    callee::EA_MEM_ALLOC.call(c, ret)?;
    c.g[A0][0] = c.g[V0][0];
    ctor(c, ret.wrapping_add(8))?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// cPS2MemCard.

/// `void* cPS2MemCard_Create()` (0x2c22c8): allocate the 0x1500-byte memory
/// card object (`0x100` alignment) and construct it.
pub fn c_ps2_mem_card_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x1500, 0x0048_5ff0, 0x100, 0x002c_22e8, |c, ret| {
            callee::MEM_CARD_CTOR.call(c, ret)
        })
    })
}

/// `void cPS2MemCard_Destroy(cPS2MemCard* card)` (0x2c2300): the deleting
/// destructor through the object's vtable (slot +8, flag 3).
pub fn c_ps2_mem_card_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let obj = c.g[A0][0];
        c.g[A2][0] = obj;
        frame.save_ra(c, 0);
        if obj != 0 {
            c.cycles += 6;
            let vt = Addr(obj as u32).lw(c);
            c.g[V0][0] = vt;
            c.g[A1][0] = 3;
            let m = VSlot::read(c, vt, 8);
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            call_ptr(c, m.func as u32, 0x002c_2328)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cPS2MemCard_ctor(cPS2MemCard* card)` (0x2c3fa8): install the class
/// vtable and initialise the card state.
pub fn c_ps2_mem_card_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(0x0048_6f78);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        Addr(obj as u32).sw(c, c.g[V0][0]);
        callee::MEM_CARD_INIT.call(c, 0x002c_3fc8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestListDir(cPS2MemCard* card, const char* name)`
/// (0x2c41d8): save the directory name and switch the card to state 5.
pub fn c_ps2_mem_card_request_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((obj as u32).wrapping_add(0x139));
        callee::STRCPY.call(c, 0x002c_41f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 5;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_41fc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestDelete(cPS2MemCard* card, const char* name)`
/// (0x2c6ce0): save the name and switch the card to state 0x4d (delete).
pub fn c_ps2_mem_card_request_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let obj = c.g[A0][0];
        c.g[S0][0] = obj;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((obj as u32).wrapping_add(0xf9));
        callee::STRCPY.call(c, 0x002c_6cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4d;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_6d04)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Strings.

/// `int UStr_Len_2(const u16* s)` (0x2c24d0): the length in UTF-16 units
/// (`lhu`, two bytes per unit), counting the first unit and then while the
/// units are non-zero.
pub fn ustr_len_2(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_24e0;
    c.cycles += 4;
    let mut p = abi::arg(c, 0);
    let mut ch = c.lhu(p);
    c.g[V0][0] = ch;
    let mut n = 0u32;
    c.g[V1][0] = 0;
    p = p.wrapping_add(2);
    c.g[A0][0] = sx(p);
    if ch != 0 {
        loop {
            c.cycles += 7;
            ch = c.lhu(p);
            c.g[V0][0] = ch;
            n = n.wrapping_add(1);
            c.g[V1][0] = sx(n);
            let more = ch != 0;
            p = p.wrapping_add(2);
            c.g[A0][0] = sx(p);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
                p = r32(c, A0);
                n = r32(c, V1);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void Str_ToLowerInPlace(char* s)` (0x2c2868): lower-case the ASCII
/// letters of a zero-terminated string.
///
/// The original is a dispatcher: it tests the byte, folds `A`-`Z`, stores it
/// back and loads the next one, and an unwind after the back-edge event check
/// re-enters it at the folding block with the current byte in `v1` and the
/// pointer in `a0`.
pub fn str_to_lower_in_place(c: &mut Cpu) {
    const HEAD: u32 = 0x002c_2870;
    // Re-entered at the folding block after an event asked the thread to
    // unwind (`pc` is the loop head, `a0`/`v1` hold the loop state).
    let mut resumed = c.pc == HEAD;
    if !resumed {
        c.cycles += 2;
        c.g[V0][0] = Addr(abi::arg(c, 0)).lb(c);
    }
    loop {
        if !resumed {
            // The zero test and the (unsigned) reload of the current byte.
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = Addr(r32(c, A0)).lbu(c);
            if !more {
                break;
            }
            if back_edge_unwinds(c, HEAD) {
                return;
            }
        }
        resumed = false;
        // Fold `A`-`Z` (`v1 - 'A' < 26`), leaving `v1 + 0x20` in v0.
        c.cycles += 4;
        c.g[V0][0] = sx((r32(c, V1).wrapping_sub(0x41) < 0x1a) as u32);
        let upper = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x20));
        if upper {
            c.cycles += 1;
            Addr(r32(c, A0)).sb(c, c.g[V0][0]);
        }
        // Advance and load the next byte.
        c.cycles += 2;
        let p = r32(c, A0).wrapping_add(1);
        c.g[A0][0] = sx(p);
        c.g[V0][0] = Addr(p).lb(c);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `sceMcClose` (0x409e70).
pub(crate) const SCE_MC_CLOSE: u32 = 0x0040_9e70;

// ---------------------------------------------------------------------------
// cPS2MemCard request wrappers.

/// `void cPS2MemCard_RequestListDirAll(this, const char* name)` (0x2c6810):
/// clear the result field at `+0x54`, copy `name` to the work buffer at
/// `+0x139` and enter state 0x3c.
pub fn c_ps2_mem_card_request_list_dir_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.sw((this as u32).wrapping_add(0x54), 0);
        c.g[A0][0] = sx((this as u32).wrapping_add(0x139));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_682c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x3c;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6838)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestOpen(this, const char* name, int mode)`
/// (0x2c6968): store `mode` at `+0x14e0`, copy the name to `+0xf9` and enter
/// state 0x3f.
pub fn c_ps2_mem_card_request_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let mode = c.g[A2][0];
        c.sw((this as u32).wrapping_add(0x14e0), mode);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xf9));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_6984)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x3f;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6990)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoClose(this)` (0x2c6a28): close the file descriptor at
/// `+0x18`, keep the result in the `gp + 0x850` slot and enter state 0x4a.
pub fn c_ps2_mem_card_do_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[A0][0] = Addr((this as u32).wrapping_add(0x18)).lw(c);
        call_ptr(c, SCE_MC_CLOSE, 0x002c_6a40)?;
        c.g[A0][0] = c.g[S0][0];
        let result = c.g[V0][0];
        let slot = gp(c, 0x850);
        c.sw(slot, result);
        c.g[A1][0] = 0x4a;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6a50)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_RequestMkdir(this, const char* name, int mode)`
/// (0x2c6c50): store `mode` at `+0x20`, copy the name to `+0xf9` and enter
/// state 0x4b.
pub fn c_ps2_mem_card_request_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let mode = c.g[A2][0];
        c.sw((this as u32).wrapping_add(0x20), mode);
        c.g[A0][0] = sx((this as u32).wrapping_add(0xf9));
        call_ptr(c, STRLEN_AND_STRCPY, 0x002c_6c6c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x4b;
        callee::MEMCARD_SET_STATE.call(c, 0x002c_6c78)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `unsigned short* UStr_FromAscii_2(unsigned short* dst, const char* src)`
/// (0x2C2540): copy a NUL-terminated byte string into `dst` as 16-bit
/// characters (sign-extending each byte) and terminate it; returns `dst`.
pub fn ustr_from_ascii_2(c: &mut Cpu) {
    c.cycles += 4;
    let mut src = r(c, A1);
    let first = Addr(src).lb(c);
    c.g[V0][0] = first;
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = Addr(src).lbu(c);
    let mut dst = r(c, A0);
    if first != 0 {
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r(c, V1) << 24);
            src = src.wrapping_add(1);
            c.g[A1][0] = sx(src);
            c.g[V0][0] = sx(((r(c, V0) as i32) >> 24) as u32);
            let ch = c.g[V0][0];
            Addr(dst).sh(c, ch);
            dst = dst.wrapping_add(2);
            c.g[A0][0] = sx(dst);
            c.g[V0][0] = Addr(src).lb(c);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = Addr(src).lbu(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_2550);
            }
        }
    }
    c.cycles += 3;
    Addr(dst).sh(c, 0);
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `unsigned short* UStr_CopyN(unsigned short* dst, const unsigned short* src,
/// int n)` (0x2C25B8): copy at most `n` 16-bit characters and terminate.
pub fn ustr_copy_n(c: &mut Cpu) {
    c.cycles += 3;
    let mut src = r(c, A1);
    let first = Addr(src).lhu(c);
    c.g[V1][0] = first;
    let dst0 = c.g[A0][0];
    c.g[V0][0] = dst0;
    let mut dst = dst0 as u32;
    if first != 0 {
        c.cycles += 2;
        if (c.g[A2][0] as i64) > 0 {
            c.cycles += 1;
            let ch = c.g[V1][0];
            Addr(dst).sh(c, ch);
            loop {
                c.cycles += 5;
                src = src.wrapping_add(2);
                c.g[A1][0] = sx(src);
                c.g[V1][0] = Addr(src).lhu(c);
                dst = dst.wrapping_add(2);
                c.g[A0][0] = sx(dst);
                let at_end = c.g[V1][0] == 0;
                c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
                if at_end {
                    break;
                }
                c.cycles += 2;
                if (c.g[A2][0] as i64) > 0 {
                    let ch = c.g[V1][0];
                    Addr(dst).sh(c, ch);
                } else {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_25d0);
                }
            }
        }
    }
    c.cycles += 2;
    Addr(dst).sh(c, 0);
    abi::ret(c);
}

/// `void cPS2MemCard_RequestListDirW(cPS2MemCard* self, const u16* name)`
/// (0x2c4198): convert the name to ASCII and ask the virtual slot 6 of
/// `*self` for the directory listing.
pub fn c_ps2_mem_card_request_list_dir_w(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x410);
        frame.save(c, S0, 0x400);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x3f0);
        c.g[A0][0] = c.g[SP][0];
        callee::USTR_TO_ASCII.call(c, 0x002c_41b0)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = Addr(r32(c, V1)).off(0x30).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x34).lw(c);
        let func = r32(c, V0);
        c.g[RA][0] = sx(0x002c_41c8);
        c.g[A0][0] = add32(c.g[S0][0], c.g[A0][0]);
        c.pc = func;
        call_indirect(c, func);
        if c.pc != 0x002c_41c8 {
            return Err(crate::call::Unwound);
        }
        frame.restore(c, S0, 0x400);
        frame.restore_ra(c, 0x3f0);
        frame.close(c);
        Ok(())
    })
}

/// `void UStr_Cat(dst, src)` (0x2c2688): append the string `src` at the end of
/// the string `dst` (16-bit characters, walked by `lhu` in 2-byte steps).
pub fn ustr_cat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(abi::r32(c, A0)).lhu(c);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(2));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(abi::r32(c, A0)).lhu(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(2));
                if event_due(c) {
                    deliver_event_here(c, 0x002c_26a0);
                }
            }
        }
        c.cycles += 5;
        callee::USTR_COPY.call(c, 0x002c_26c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoFlush(void* self)` (0x2c6b00): flush the memcard and
/// move the object to the done (or error) state.
pub fn c_ps2_mem_card_do_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        call_ptr(c, table::SCE_MC_FLUSH, 0x002c_6b18)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        Addr(gp_2(c, 0x850)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        if ok {
            c.cycles += 4;
            c.g[A1][0] = 0x48;
            callee::CPS2_MEMCARD_SET_STATE.call(c, 0x002c_6b40)?;
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0x1;
            callee::CPS2_MEMCARD_SET_STATE.call(c, 0x002c_6b2c)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LocStringTable_dtor(LocStringTable* self, int flags)`
/// (0x2c6d70): free the owned table and, when `flags & 1`, the object.
pub fn loc_string_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::MEM_FREE, 0x002c_6d98)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002c_6dac)?;
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
// Memcards, menus and world.

/// `void MemCard_AsciiToSjisString(src, dst)` (0x2c2210): convert a
/// NUL-terminated ASCII string to Shift-JIS.
pub fn mem_card_ascii_to_sjis_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            loop {
                c.cycles += 7;
                c.g[A0][0] = Addr(r32(c, S0)).lbu(c);
                c.g[RA][0] = sx(0x002c_223c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                callee::MEM_CARD_ASCII_CHAR_TO_SJIS.call(c, 0x002c_223c)?;
                Addr(r32(c, S1)).sh(c, c.g[V0][0]);
                c.g[V0][0] = Addr(r32(c, S0)).lb(c);
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2230);
                }
            }
        }
        c.cycles += 6;
        Addr(r32(c, S1)).sh(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoFormat(self)` (0x2c4388): format the card and go to
/// the done/failed state.
pub fn c_ps2_mem_card_do_format(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x002c_43a4);
        c.g[A0][0] = Addr(r32(c, S0)).off(0xc).lw(c);
        call_ptr(c, callee::SCE_MC_FORMAT, 0x002c_43a4)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        Addr(gp_3(c, 0x850)).sw(c, c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002c_43cc);
            c.g[A1][0] = 0x12;
            callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_43cc)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002c_43b8);
            c.g[A1][0] = 1;
            callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_43b8)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v34(self, name)` (0x2c4520): copy the save name, format
/// the full path and go to state 0x14.
pub fn c_ps2_mem_card_v34(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xb9));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002c_4540);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::STRCPY_2, 0x002c_4540)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xf9));
        c.g[RA][0] = sx(0x002c_4554);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3950));
        call_ptr(c, callee::SPRINTF, 0x002c_4554)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002c_4560);
        c.g[A1][0] = 0x14;
        callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_4560)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v35(self, name)` (0x2c4578): convert the wide save name
/// to ASCII, format the full path and go to state 0x14.
pub fn c_ps2_mem_card_v35(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xb9));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002c_4598);
        c.g[A0][0] = c.g[S0][0];
        callee::USTR_TO_ASCII.call(c, 0x002c_4598)?;
        c.g[A1][0] = 0x4a_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xf9));
        c.g[RA][0] = sx(0x002c_45ac);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3950));
        call_ptr(c, callee::SPRINTF, 0x002c_45ac)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x002c_45b8);
        c.g[A1][0] = 0x14;
        callee::C_PS2_MEM_CARD_SET_STATE.call(c, 0x002c_45b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_v29(self, a1, a2, a3)` (0x2c4800): build the save name
/// and request a read of the save.
pub fn c_ps2_mem_card_v29(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.g[RA][0] = sx(0x002c_4828);
        c.g[A2][0] = sx(c.g[SP][0] as u32);
        callee::C_PS2_MEM_CARD_GET_SAVE_NAME.call(c, 0x002c_4828)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x002c_483c);
        c.g[A1][0] = sx(c.g[SP][0] as u32);
        callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE.call(c, 0x002c_483c)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoSeek(MemCard* self)` (0x2c6a88): start the seek phase,
/// then advance the state machine to the result.
pub fn c_ps2_mem_card_do_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x14e4).lw(c);
        c.g[A2][0] = Addr(this).off(0x14e8).lw(c);
        c.g[RA][0] = sx(0x002c_6aa8);
        call_ptr(c, callee::SCE_MC_SEEK, 0x002c_6aa8)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6abc);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6abc)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x42;
            c.g[RA][0] = sx(0x002c_6ad0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6ad0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoRead(MemCard* self)` (0x2c6b78): start the read phase.
pub fn c_ps2_mem_card_do_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x2c).lw(c);
        c.g[A2][0] = Addr(this).off(0x24).lw(c);
        c.g[RA][0] = sx(0x002c_6b98);
        call_ptr(c, callee::SCE_MC_READ, 0x002c_6b98)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6bac);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6bac)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x44;
            c.g[RA][0] = sx(0x002c_6bc0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6bc0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoWrite(MemCard* self)` (0x2c6bf8): start the write phase.
pub fn c_ps2_mem_card_do_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        c.g[A1][0] = Addr(this).off(0x2c).lw(c);
        c.g[A2][0] = Addr(this).off(0x24).lw(c);
        c.g[RA][0] = sx(0x002c_6c18);
        call_ptr(c, callee::SCE_MC_WRITE, 0x002c_6c18)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6c2c);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6c2c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x46;
            c.g[RA][0] = sx(0x002c_6c40);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6c40)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoMkdir(MemCard* self)` (0x2c6c88): start creating the
/// save directory.
pub fn c_ps2_mem_card_do_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A2][0] = sx(this.wrapping_add(0xf9));
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.g[RA][0] = sx(0x002c_6ca8);
        call_ptr(c, callee::SCE_MC_MKDIR, 0x002c_6ca8)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6cbc);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6cbc)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x4c;
            c.g[RA][0] = sx(0x002c_6cd0);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6cd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoDelete(MemCard* self)` (0x2c6d18): start deleting the
/// save directory.
pub fn c_ps2_mem_card_do_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[A2][0] = sx(this.wrapping_add(0xf9));
        c.g[A0][0] = Addr(this).off(0xc).lw(c);
        c.g[RA][0] = sx(0x002c_6d38);
        call_ptr(c, callee::SCE_MC_DELETE, 0x002c_6d38)?;
        let failed = (c.g[V0][0] as i64) < 0;
        Addr(gp_4(c, 0x850)).sw(c, c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 1;
            c.g[RA][0] = sx(0x002c_6d4c);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6d4c)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.g[A0][0] = sx(this);
            c.g[A1][0] = 0x4e;
            c.g[RA][0] = sx(0x002c_6d60);
            callee::CPS2_MEM_CARD_SET_STATE.call(c, 0x002c_6d60)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card port/slot state.

/// `void cPS2MemCard_ClearPortChanged(self)` (0x2c44e0): clear the port's
/// `+0x18c` field of the current level and, if it was -1, its `+0x184` field.
pub fn c_ps2_mem_card_clear_port_changed(c: &mut Cpu) {
    c.cycles += 13;
    let this = abi::arg(c, 0) as u32;
    c.g[V1][0] = Addr(this).off(0xc).lw(c);
    c.g[A1][0] = 0x14;
    c.g[A2][0] = sx(this.wrapping_add(0x184));
    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
    c.mult(0, V0, V1, A1);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
    let level = c.g[V1][0] as u32;
    Addr(level).off(0x18c).sw(c, 0);
    c.g[V0][0] = Addr(this).off(0xc).lw(c);
    c.mult(0, V0, V0, A1);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let word = c.g[A2][0] as u32;
    c.g[V1][0] = Addr(word).lw(c);
    if c.g[V1][0] == c.g[A3][0] {
        Addr(word).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsState34(self)` (0x2c4748): state word `+4` equals 0x34.
pub fn c_ps2_mem_card_is_state34(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    c.g[V0][0] ^= 0x34;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsState35(self)` (0x2c4880): state word `+4` equals 0x35.
pub fn c_ps2_mem_card_is_state35(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = this.off(0x4).lw(c);
    c.g[V0][0] ^= 0x35;
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsBusy(self)` (0x2c5140): busy while the state word at
/// `+4` or the flag at `+0x40` is non-zero.
pub fn c_ps2_mem_card_is_busy(c: &mut Cpu) {
    'busy: {
        c.cycles += 3;
        let this = Addr(abi::arg(c, 0));
        c.g[V0][0] = this.off(0x4).lw(c);
        if c.g[V0][0] != 0 {
            break 'busy;
        }
        c.cycles += 3;
        c.g[V0][0] = this.off(0x40).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        break 'busy;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortCardError(self)` (0x2c5168): whether the current
/// level's error word is one of -1, -3, -8..-10, -10001, -10002.
pub fn c_ps2_mem_card_is_port_card_error(c: &mut Cpu) {
    'outer: {
        'inner: {
            c.cycles += 8;
            let this = abi::arg(c, 0) as u32;
            c.g[V0][0] = Addr(this).off(0xc).lw(c);
            c.g[V1][0] = 0x14;
            c.mult(0, A1, V0, V1);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(this));
            let level = c.g[V0][0] as u32;
            c.g[A0][0] = Addr(level).off(0x184).lw(c);
            c.g[V1][0] = ((c.g[A0][0] as i64) < -7) as u64;
            let ge_minus7 = c.g[V1][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10) as u64;
            if ge_minus7 {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                let is_minus3 = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if is_minus3 {
                    break 'outer;
                }
                c.cycles += 2;
                let not_minus1 = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if not_minus1 {
                    break 'inner;
                }
                break 'outer;
            }
            c.cycles += 2;
            let ge_minus10 = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10000) as u64;
            if ge_minus10 {
                break 'outer;
            }
            c.cycles += 2;
            let ge_minus10000 = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -10002) as u64;
            if ge_minus10000 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'inner;
            }
            c.cycles += 2;
            let lt_minus10002 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if lt_minus10002 {
                break 'inner;
            }
            c.cycles += 2;
            c.g[V0][0] = 1;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortNotPS2Card(self, port)` (0x2c5300): the port's
/// `+0x184` word equals -10002.
pub fn c_ps2_mem_card_is_port_not_ps2_card(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x14;
    c.g[V1][0] = 0xffff_ffff_ffff_d8ee;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let level = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(level).off(0x184).lw(c);
    c.g[V0][0] ^= c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_v52(self, port)` (0x2c5358): the level error word is -7,
/// -10001 or in {-5..0}.
pub fn c_ps2_mem_card_v52(c: &mut Cpu) {
    'outer: {
        c.cycles += 7;
        let this = abi::arg(c, 0) as u32;
        c.g[V0][0] = 0x14;
        c.g[V1][0] = 0xffff_ffff_ffff_fff9;
        c.mult(0, A2, A1, V0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
        let level = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(level).off(0x184).lw(c);
        let is_minus7 = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < -6) as u64;
        if is_minus7 {
            break 'outer;
        }
        'inner: {
            'check: {
                c.cycles += 2;
                let ge_minus6 = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_d8ef;
                if ge_minus6 {
                    break 'check;
                }
                c.cycles += 2;
                let is_minus10001 = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0;
                if is_minus10001 {
                    break 'outer;
                }
                c.cycles += 2;
                break 'inner;
            }
            c.cycles += 2;
            let positive = (c.g[A0][0] as i64) > 0;
            c.g[V0][0] = 0;
            if positive {
                break 'inner;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A0][0] as i64) < -5) as u64;
            let lt_minus5 = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if lt_minus5 {
                break 'inner;
            }
            break 'outer;
        }
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortFormatted(self, port)` (0x2c53b0): the port's
/// `+0x188` word is non-zero.
pub fn c_ps2_mem_card_is_port_formatted(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x14;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    let level = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(level).off(0x188).lw(c);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_HasSlotName(self, slot)` (0x2c5528): whether the slot's
/// name at `+0x1b0` is non-empty.
pub fn c_ps2_mem_card_has_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0) as u32;
        c.g[V0][0] = 0x78;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        // V1 = slot * 0x78 (the slot stride); V0 keeps 0x78 as the multiplier.
        c.mult(0, V1, A1, V0);
        let sp = c.g[SP][0] as u32;
        c.sd(sp, c.g[RA][0]);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b0));
        call_ptr(c, 0x0041_6810, 0x002c_5544)?;
        c.g[RA][0] = c.ld(sp);
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        c.g[SP][0] = sx(sp.wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    });
}

/// `void cPS2MemCard_MarkSlotUsed(self, slot)` (0x2c5558): set the slot's
/// `+0x1a4` flag.
pub fn c_ps2_mem_card_mark_slot_used(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x78;
    c.g[V1][0] = 0x1;
    c.mult(0, A2, A1, V0);
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let slot = c.g[A0][0] as u32;
    c.g[V1][0] = 0x1;
    Addr(slot).off(0x1a4).sw(c, 1);
    abi::ret(c);
}

/// `bool cPS2MemCard_IsSlotEmpty(self, slot)` (0x2c5588): whether both the
/// `+0x1a8` and `+0x1a4` slot words are zero.
pub fn c_ps2_mem_card_is_slot_empty(c: &mut Cpu) {
    'taken: {
        c.cycles += 7;
        c.g[V1][0] = 0x78;
        c.g[A2][0] = c.g[A0][0];
        c.mult(0, A0, A1, V1);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let slot = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(slot).off(0x1a8).lw(c);
        if c.g[V1][0] != 0 {
            break 'taken;
        }
        c.cycles += 3;
        c.g[V1][0] = Addr(slot).off(0x1a4).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        break 'taken;
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `bool cPS2MemCard_SlotHas1AC(self, slot)` (0x2c55c0): the slot's `+0x1ac`
/// word is non-zero.
pub fn c_ps2_mem_card_slot_has_1ac(c: &mut Cpu) {
    c.cycles += 6;
    let this = abi::arg(c, 0) as u32;
    c.g[V0][0] = 0x78;
    c.mult(0, V1, A1, V0);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
    let slot = c.g[A0][0] as u32;
    c.g[V0][0] = Addr(slot).off(0x1ac).lw(c);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `bool cPS2MemCard_IsPortResultM11(self)` (0x2c66d8): the current level's
/// error word equals -11.
pub fn c_ps2_mem_card_is_port_result_m11(c: &mut Cpu) {
    c.cycles += 9;
    let this = abi::arg(c, 0) as u32;
    c.g[V1][0] = Addr(this).off(0xc).lw(c);
    c.g[V0][0] = 0x14;
    c.g[A1][0] = 0xffff_ffff_ffff_fff5;
    c.mult(0, A2, V1, V0);
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(this));
    let level = c.g[V1][0] as u32;
    c.g[V0][0] = Addr(level).off(0x184).lw(c);
    c.g[V0][0] ^= c.g[A1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `int cPS2MemCard_CalcSaveSizeKB(size)` (0x2c6758): the save size in KiB
/// (1 KiB blocks, plus 77 KiB of header/overhead).
pub fn c_ps2_mem_card_calc_save_size_kb(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3ff));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7fe));
    if c.g[V0][0] != 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[A1][0] = sx((((c.g[A1][0] as u32) as i32) >> 10) as u32);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4d));
    abi::ret(c);
}

/// `sw src, gp+off`.
#[inline(always)]
pub(crate) fn gp_sw(c: &mut Cpu, off: i32, v: u64) {
    c.sw(gp_rel(c, off), v);
}

/// `void MemCard_SjisToAsciiString(char* dst, char* src)` (0x2C2268): convert
/// the 16-bit SJIS string at `src` into single-byte characters at `dst`.
///
/// The original is a resumable loop (a delivered event may unwind with `pc`
/// at the conversion body), so the entry point also accepts that resume
/// address, as the generated `match pc` does.
pub fn memcard_sjis_to_ascii_string(c: &mut Cpu) {
    const ENTRY: u32 = 0x002c_2268;
    const CONVERT: u32 = 0x002c_2288;
    const TEST: u32 = 0x002c_22a0;
    const EXIT: u32 = 0x002c_22ac;

    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 7;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S0, 0x20);
                    frame.save(c, S1, 0x10);
                    c.g[S0][0] = c.g[A2][0];
                    frame.save_ra(c, 0);
                    c.g[S1][0] = c.g[A1][0];
                    pc = TEST;
                }
                CONVERT => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                    c.g[S0][0] = sx(ea(c.g[S0][0], 2));
                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
                    callee::MEMCARD_SJIS_CHAR_TO_ASCII.call(c, 0x002c_2298)?;
                    c.sb(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[S1][0] = sx(ea(c.g[S1][0], 1));
                    pc = TEST;
                }
                0x002c_2284 => {
                    // The generated block split (never branched to).
                    c.cycles += 1;
                    pc = CONVERT;
                }
                TEST => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lh(c.g[S0][0] as u32);
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lhu(c.g[S0][0] as u32);
                    if !more {
                        pc = EXIT;
                        continue;
                    }
                    if event_due(c) && crate::events::deliver_event(c, CONVERT) {
                        return Ok(());
                    }
                    pc = CONVERT;
                }
                EXIT => {
                    c.cycles += 6;
                    c.sb(c.g[S1][0] as u32, 0);
                    let frame = Frame { size: 0x30 };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, 0x002c_22c4);
                    return Ok(());
                }
            }
        }
    })
}

/// `void cPS2MemCard_RequestGetInfo(cPS2MemCard* self, int op)` (0x2C4480):
/// remember the pending operation and, when the card's `v11()` finds nothing,
/// restart the state machine at state 3.
pub fn ps2_memcard_request_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.sw(ea(c.g[S0][0], 0xc), c.g[S1][0]);
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.lh(ea(c.g[V1][0], 0x58));
        c.g[V0][0] = c.lw(ea(c.g[V1][0], 0x5c));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(ea(c.g[S0][0], c.g[A0][0] as u32));
        call_ptr(c, method, 0x002c_44b0)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.sw(ea(c.g[S0][0], 0x10), c.g[S1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 3;
            callee::PS2_MEMCARD_SET_STATE.call(c, 0x002c_44c8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2MemCard_DoGetInfo(cPS2MemCard* self)` (0x2C50E0): ask the memory
/// card manager for the card's info and record success at `+0x40`.
pub fn ps2_memcard_do_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        gp_sw(c, 0x848, 0);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.sw(ea(c.g[S0][0], 0x20), 0);
        c.g[A2][0] = sx(gp_rel(c, 0x848));
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 0xc));
        c.g[A3][0] = sx(gp_rel(c, 0x84c));
        c.g[T0][0] = sx(gp_rel(c, 0x854));
        gp_sw(c, 0x854, 0);
        gp_sw(c, 0x84c, 0);
        call_ptr(c, addr::MC_MANAGER_GET_INFO, 0x002c_5118)?;
        c.g[V1][0] = (c.g[V0][0] < 1) as u64;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 5;
        c.sw(ea(c.g[S0][0], 0x40), c.g[V1][0]);
        c.sw(ea(c.g[S0][0], 0x4c), c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card.

/// `void cPS2MemCard_v38(self, a1, a2, a3)` (0x2c46c0): build the save name
/// and call the virtual save slot.
pub fn ps2_mem_card_v38(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x40);
        c.g[S2][0] = c.g[A3][0];
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x002c_54d0, 0x002c_46e8)?;
        c.g[V1][0] = lw_2(c, c.g[S0][0] as u32);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = Addr(p(c.g[V1][0], 0x120)).lh(c);
        c.g[V0][0] = lw_2(c, p(c.g[V1][0], 0x124));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002c_4708)?;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `void cPS2MemCard_ClearDirCache(cPS2MemCard* self)` (0x2c5278): wipe the
/// directory cache buffers and the current directory entry.
pub fn ps2_mem_card_clear_dir_cache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x40;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(p(c.g[S0][0], 0x50)).sw(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x480));
        Addr(p(c.g[S0][0], 0x54)).sw(c, 0);
        call_ptr(c, 0x003e_6448, 0x002c_52a0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1a4));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x2d0;
        call_ptr(c, 0x003e_6448, 0x002c_52b0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c0));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x1000;
        call_ptr(c, 0x003e_6448, 0x002c_52c0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002c_52d8, 0x002c_52c8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Memory cards and cameras.

/// `void cPS2MemCard_DoOpen(cPS2MemCard* self)` (0x2c69a0): open the memory
/// card file and enter state 1 (failure) or 0x40 (ready).
pub fn cps2_mem_card_do_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = sx(p_2(c.g[S0][0] as u32, 0xf9));
        Addr(p_2(c.g[S0][0] as u32, 0x18)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p_2(c.g[S0][0] as u32, 0x0c)).lw(c);
        c.g[A3][0] = Addr(p_2(c.g[S0][0] as u32, 0x14e0)).lw(c);
        call_ptr(c, 0x0040_9d10, 0x002c_69cc)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.sw(gp(c, 0x850), c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x40;
            call_ptr(c, 0x002c_48c0, 0x002c_69f4)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x002c_48c0, 0x002c_69e0)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_dtor(self, flags)` (0x2c3fe0): close the card, free the buffer,
/// install the base vtable and free the object when asked.
pub fn c_ps2_mem_card_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f78));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x002c_6360, 0x002c_4008)?;
        c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x30)).lw(c);
        let no_buffer = c.g[A0][0] == 0;
        c.g[V0][0] = 0x480000;
        if !no_buffer {
            c.cycles += 3;
            call_ptr(c, 0x0025_23a8, 0x002c_401c)?;
            c.g[V0][0] = 0x480000;
        }
        c.cycles += 4;
        c.g[V1][0] = c.g[S1][0] & 0x1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71e8));
        let keep = c.g[V1][0] == 0;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        if !keep {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x002c_4038)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard_CountSaves` (0x2C68F0): count the memory card's save slots whose
/// name is non-empty.
pub fn cps2_mem_card_count_saves(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, S2, 0x14);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S1][0] = low(c, S2).wrapping_add(0x1b0);
            loop {
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[S3][0] = low(c, S3).wrapping_add(1);
                call_ptr(c, addr::STRLEN, 0x002c_692c)?;
                c.g[S1][0] = low(c, S1).wrapping_add(0x78);
                c.g[A0][0] = lw_at(c, S2, 0x14);
                c.g[V1][0] = low(c, S0).wrapping_add(1);
                c.g[A0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let more = c.g[A0][0] != 0;
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_6920);
                }
            }
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
// PS2 memcard save-name stubs (world-painter module).

/// Shared body of the three memcard stubs: build the ASCII copy of the name at
/// `+0xf9`, then record the two payload words and hand `state` to the memcard
/// state machine.
pub(crate) fn mem_card_save_name(
    c: &mut Cpu,
    a2: u64,
    a3: u64,
    state: u64,
    wide: bool,
    rets: [u32; 3],
) -> Flow {
    c.cycles += 30;
    c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x50));
    c.sq(16, sp_2(c).wrapping_add(0x40));
    c.g[S0][0] = c.g[A0][0];
    c.sq(17, sp_2(c).wrapping_add(0x30));
    c.sq(18, sp_2(c).wrapping_add(0x20));
    c.g[S1][0] = sx(low_2(c, S0).wrapping_add(0xb9));
    c.sq(19, sp_2(c).wrapping_add(0x10));
    c.g[S2][0] = a2;
    c.sd(sp_2(c), c.g[RA][0]);
    c.g[S3][0] = a3;
    if wide {
        c.g[A0][0] = c.g[S1][0];
        callee::USTR_TO_ASCII_2.call(c, rets[0])?;
    } else {
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRCPY_3, rets[0])?;
    }
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = 0x4a_0000;
    c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0xf9));
    c.g[A3][0] = c.g[A2][0];
    c.g[A1][0] = sx(low_2(c, A1).wrapping_add(0x3948));
    call_ptr(c, callee::SPRINTF, rets[1])?;
    sw_r(c, S0, 0x2c, c.g[S2][0]);
    c.g[A0][0] = c.g[S0][0];
    sw_r(c, S0, 0x20, c.g[S3][0]);
    c.g[A1][0] = state;
    callee::CPS2_MEM_CARD_SET_STATE_2.call(c, rets[2])?;
    c.lq(16, sp_2(c).wrapping_add(0x40));
    c.lq(17, sp_2(c).wrapping_add(0x30));
    c.lq(18, sp_2(c).wrapping_add(0x20));
    c.lq(19, sp_2(c).wrapping_add(0x10));
    c.g[RA][0] = Addr(sp_2(c)).ld(c);
    let t = low_2(c, RA);
    c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x50));
    c.pc = t;
    Ok(())
}

/// `void cPS2MemCard_v37(self, wname, a2, a3)` (0x2c45d0): wide-character name.
pub fn cps2_mem_card_v37(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(
            c,
            a2,
            a3,
            0x15,
            true,
            [0x002c_4600, 0x002c_4618, 0x002c_462c],
        )
    })
}

/// `void cPS2MemCard_v36(self, name, a2, a3)` (0x2c4648): narrow name.
pub fn cps2_mem_card_v36(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(
            c,
            a2,
            a3,
            0x15,
            false,
            [0x002c_4678, 0x002c_4690, 0x002c_46a4],
        )
    })
}

/// `void cPS2MemCard_RequestReadSave(self, name, a2, a3)` (0x2c4788): same,
/// entering the `0x1b` (read) state.
pub fn cps2_mem_card_request_read_save(c: &mut Cpu) {
    guest(c, |c| {
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        mem_card_save_name(
            c,
            a2,
            a3,
            0x1b,
            false,
            [0x002c_47b8, 0x002c_47d0, 0x002c_47e4],
        )
    })
}

/// `void cPS2MemCard_DoListDir(cPS2MemCard* m)` (0x002c4210): clear the
/// 0x1000-byte directory buffer and ask the IOP for the entries of the
/// current directory; state 1 on failure, state 6 on success.
pub fn c_ps2_mem_card_do_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x1000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c0));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x002c_4238)?; // EA_MemSet
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x139));
        c.g[A3][0] = 0;
        c.g[T0][0] = 6;
        call_ptr(c, 0x0040_a688, 0x002c_4254)?; // sceMcGetDir
        let failed = (c.g[V0][0] as i64) < 0;
        c.sw(gpa(c, 0x850), c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002c_48c0, 0x002c_4268)?; // cPS2MemCard_SetState
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 6;
            call_ptr(c, 0x002c_48c0, 0x002c_427c)?; // cPS2MemCard_SetState
        }
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `void cPS2MemCard_DoListDirAll(cPS2MemCard* m)` (0x002c6848): as
/// [`c_ps2_mem_card_do_list_dir`] but listing every directory (`t0 = 0x40`)
/// and entering state 0x3d on success.
pub fn c_ps2_mem_card_do_list_dir_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.g[A1][0] = 0;
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x1000;
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c0));
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x002c_6870)?; // EA_MemSet
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[T1][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x139));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x40;
        call_ptr(c, 0x0040_a688, 0x002c_688c)?; // sceMcGetDir
        let failed = (c.g[V0][0] as i64) < 0;
        c.sw(gpa(c, 0x850), c.g[V0][0]);
        if failed {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002c_48c0, 0x002c_68a0)?; // cPS2MemCard_SetState
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x3d;
            call_ptr(c, 0x002c_48c0, 0x002c_68b4)?; // cPS2MemCard_SetState
        }
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.cycles += 4;
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card.

/// `void cPS2MemCard_Close(cPS2MemCard* self)` (0x2c6360): finish the pending
/// card request (if any) and clear the card state.
pub fn c_ps2_mem_card_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_5(r32(c, S0), 0x40)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        let mut skip = false;
        if !idle {
            c.cycles += 8;
            Addr(gp_5(c, 0x850)).sw(c, 0);
            c.g[A2][0] = sx(gp_5(c, 0x850));
            c.g[RA][0] = sx(0x002c_638c);
            c.g[A1][0] = 0;
            call_ptr(c, 0x0040_a360, 0x002c_638c)?;
            c.g[V1][0] = Addr(gp_5(c, 0x850)).lw(c);
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] != c.g[V0][0] {
                Addr(p_5(r32(c, S0), 0x40)).sw(c, 0);
                skip = true;
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr(p_5(r32(c, S0), 0xc)).lw(c);
                c.g[V1][0] = 0x14;
                c.g[A0][0] = 0x1;
                c.mult(0, 5, 2, 3);
                c.g[V0][0] = sx(p_5(r32(c, A1), r32(c, S0)));
                Addr(p_5(r32(c, V0), 0x18c)).sw(c, c.g[A0][0]);
            }
        }
        if !skip {
            c.cycles += 1;
            Addr(p_5(r32(c, S0), 0x40)).sw(c, 0);
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S0][0];
        Addr(p_5(r32(c, S0), 0xc)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x17c)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x180)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x184)).sw(c, 0);
        Addr(p_5(r32(c, S0), 0x188)).sw(c, 0);
        c.g[RA][0] = sx(0x002c_63d8);
        Addr(p_5(r32(c, S0), 0x3c)).sw(c, 0);
        callee::PS2_MEM_CARD_CLEAR_DIR_CACHE.call(c, 0x002c_63d8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cPS2MemCard_GetSlotName` (0x2c63f8): format the memory-card slot number
// (n+1) for the card icon, with the slot's free-space suffix when it is one
// of the first two slots.
pub fn cps2_memcard_get_slot_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = 0x4a0000;
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3970));
        call_ptr(c, 0x002c_2540, 0x002c_6428)?;
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x002c_26d0, 0x002c_643c)?;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 2i64) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            let negative = (c.g[S0][0] as i64) < 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if !negative {
                c.cycles += 15;
                c.g[A0][0] = 0x480000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64e8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x002c_646c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x002c_647c)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x002c_26d0, 0x002c_648c)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x60));
        } else {
            c.lq(16, frame.at(c, 0x60));
        }
        c.cycles += 6;
        c.lq(17, frame.at(c, 0x50));
        c.lq(18, frame.at(c, 0x40));
        c.lq(19, frame.at(c, 0x30));
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Localisation.

/// `char* LocStringTable_Find(LocStringTable* t, int key)` (0x2c6de0): binary
/// search the sorted (key, string) table; 0 when the key is absent or the
/// table is empty.
pub fn loc_string_table_find(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(p_6(c.g[A0][0] as u32, 0xc)).lw(c);
    c.g[T2][0] = 0;
    let empty = c.g[V0][0] == 0;
    c.g[T0][0] = Addr(p_6(c.g[A0][0] as u32, 8)).lw(c);
    if empty {
        c.cycles += 2;
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
        return;
    }
    'miss: {
        c.cycles += 7;
        c.g[A0][0] = Addr(p_6(c.g[A0][0] as u32, 0x10)).lw(c);
        c.g[V1][0] = sx((c.g[T0][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        let past_end = c.g[V0][0] != 0;
        c.g[T1][0] = c.g[A0][0];
        if past_end {
            break 'miss;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
        c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
        let before_start = c.g[V0][0] == 0;
        c.g[A2][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
        if !before_start {
            break 'miss;
        }
        'found: {
            c.cycles += 5;
            c.g[A0][0] = sx((c.g[A2][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let hit = c.g[V1][0] == c.g[A1][0];
            c.g[A3][0] = c.g[T1][0];
            if hit {
                break 'found;
            }
            c.cycles += 2;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            loop {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.g[T2][0] = c.g[A2][0];
                } else {
                    c.cycles += 1;
                    c.g[T0][0] = c.g[A2][0];
                }
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                let converged = c.g[V0][0] == c.g[A2][0];
                c.g[A2][0] = c.g[V0][0];
                if converged {
                    break 'miss;
                }
                c.cycles += 5;
                c.g[A0][0] = sx((c.g[A2][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let different = c.g[V1][0] != c.g[A1][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                if !different {
                    break 'found;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002c_6e40);
                }
            }
        }
        c.cycles += 4;
        c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
        let ret = c.g[RA][0] as u32;
        c.g[V0][0] = Addr(p_6(c.g[V1][0] as u32, 4)).lw(c);
        c.pc = ret;
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    c.pc = c.g[RA][0] as u32;
}

/// `void cPS2MemCard_DoGetSaveDirEntry(self)` (0x2c42b0): issue the
/// sceMcGetDir for the save directory and update the state machine.
pub fn cps2_mem_card_do_get_save_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x78;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb9));
        c.sw(p_6(c.g[S2][0] as u32, 0x3c), 0);
        c.g[A1][0] = Addr(p_6(c.g[S2][0] as u32, 0x1c)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf9));
        c.mult(0, 3, 5, 2);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1b0));
        call_ptr(c, 0x0041_66f4, 0x002c_42f0)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3948));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_7828, 0x002c_4308)?;
        c.g[A0][0] = Addr(p_6(c.g[S2][0] as u32, 0xc)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x1;
        c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x480));
        call_ptr(c, 0x0040_a688, 0x002c_4324)?;
        let ok = (c.g[V0][0] as i64) >= 0;
        c.sw(p_6(r32(c, GP), 0x850), c.g[V0][0]);
        if ok {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x23;
            call_ptr(c, 0x002c_48c0, 0x002c_434c)?;
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x002c_48c0, 0x002c_4338)?;
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

// ---------------------------------------------------------------------------
// Memory card: slot marking.

/// `void cPS2MemCard_MarkSlotsWithoutSpace(self)` (0x2c6280): mark every slot
/// that cannot hold another save, given the remaining space and save size.
pub fn cps2memcard_mark_slots_without_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p_7(r32(c, V1), 0xa0)).lh(c);
        c.g[V0][0] = Addr(p_7(r32(c, V1), 0xa4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p_7(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x002c_62bc)?;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[S3][0] = c.g[V0][0];
        c.g[A1][0] = Addr(p_7(r32(c, S1), 0x24)).lw(c);
        c.g[A0][0] = Addr(p_7(r32(c, V1), 0xb0)).lh(c);
        c.g[V0][0] = Addr(p_7(r32(c, V1), 0xb4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(p_7(r32(c, S1), r32(c, A0)));
        call_ptr(c, f, 0x002c_62d8)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = Addr(p_7(r32(c, S1), 0x14)).lw(c);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S5][0] = 1;
        if none {
            c.cycles += 9;
            frame.restore(c, S0, 0x60);
            frame.restore(c, S1, 0x50);
            frame.restore(c, S2, 0x40);
            frame.restore(c, S3, 0x30);
            frame.restore(c, S4, 0x20);
            frame.restore(c, S5, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        c.g[S0][0] = sx(p_7(r32(c, S1), 0x1ac));
        loop {
            c.cycles += 9;
            Addr(r32(c, S0)).sw(c, 0);
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            c.g[A0][0] = Addr(p_7(r32(c, V1), 0x218)).lh(c);
            c.g[V0][0] = Addr(p_7(r32(c, V1), 0x21c)).lw(c);
            let f = r32(c, V0);
            c.g[A0][0] = sx(p_7(r32(c, S1), r32(c, A0)));
            call_ptr(c, f, 0x002c_630c)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p_7(r32(c, S1), 0x14)).lw(c);
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    Addr(r32(c, S0)).sw(c, c.g[S5][0]);
                } else {
                    c.cycles += 1;
                    c.g[S3][0] = sx(r32(c, S3).wrapping_sub(r32(c, S4)));
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(p_7(r32(c, S1), 0x14)).lw(c);
            }
            c.cycles += 4;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(p_7(r32(c, S0), 0x78));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_62f0);
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

/// `QuadTree_FlattenNode(self, out, count, map, node)` (0x2c2088): emit a
/// quadtree node into the flat array (`out[*count] = {child, payload}`),
/// record its index in `map` (`(node - self->base) >> 3`), then recurse into
/// the four children when the node's first child word has bit 0 set.
pub fn quad_tree_flatten_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x70);
        let this = abi::arg(c, 0);
        let _out = abi::arg(c, 1);
        let count = abi::arg(c, 2);
        let _map = abi::arg(c, 3);
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 21, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, 16, 0x60);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, 17, 0x50);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, count);
        c.g[A1][0] = lw(c, p_8(c.g[T0][0] as u32, 4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        c.sw(p_8(c.g[V0][0] as u32, 4), c.g[A1][0]);
        c.g[V1][0] = lw(c, count);
        c.g[A0][0] = lw(c, c.g[T0][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[A0][0]);
        c.g[V0][0] = lw(c, p_8(this, 0x20));
        c.g[V1][0] = lw(c, count);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
        c.g[V0][0] &= 0xffff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.sw(count, c.g[V1][0]);
        c.g[V0][0] = c.lhu(c.g[T0][0] as u32);
        c.g[V0][0] &= 1;
        c.g[S0][0] = c.g[T0][0];
        if c.g[V0][0] != 0 {
            // four children at base + (childWord >> 1)
            c.cycles += 2;
            c.g[S1][0] = 3;
            c.g[T0][0] = c.lhu(c.g[S0][0] as u32);
            loop {
                c.cycles += 13;
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = lw(c, p_8(c.g[S3][0] as u32, 0x20));
                c.g[A1][0] = c.g[S4][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32) >> 1);
                c.g[A2][0] = c.g[S2][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
                c.g[A3][0] = c.g[S5][0];
                c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[RA][0] = sx(0x002c_2148);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                c.pc = 0x002c_2088;
                quad_tree_flatten_node(c);
                if c.pc != 0x002c_2148 {
                    return Err(Unwound);
                }
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                if (c.g[S1][0] as i64) < 0 {
                    break;
                }
                c.g[T0][0] = c.lhu(c.g[S0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2120);
                }
            }
        }
        c.cycles += 9;
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

// ---------------------------------------------------------------------------
// UStr_vsprintf (0x3dc450)
// ---------------------------------------------------------------------------

/// `UStr_vsprintf`'s stack frame size.
pub(crate) const USTR_VSPRINTF_FRAME: u32 = 0x110;

/// Frame slot holding the output pointer (`char16* dst`), advanced per char.
pub(crate) const USTR_VSPRINTF_DST: u32 = 0x60;

/// Frame offset of the upper-case hex digit string (`fp`); the lower-case
/// one sits at `sp + 0`.
pub(crate) const USTR_VSPRINTF_UPPER_DIGITS: u32 = 0x30;

/// ASCII "0123456789abcdef" / "0123456789ABCDEF" (.rodata).
pub(crate) const USTR_VSPRINTF_LOWER_ASCII: u32 = 0x0049_5bc0;

pub(crate) const USTR_VSPRINTF_UPPER_ASCII: u32 = 0x0049_5bd8;

/// Flag-character jump table (' '..'0', 17 entries, .rodata).
pub(crate) const USTR_VSPRINTF_FLAG_TABLE: u32 = 0x0049_5bf0;

/// Conversion jump table ('C'..'x', 0x36 entries, .rodata).
pub(crate) const USTR_VSPRINTF_CONV_TABLE: u32 = 0x0049_5c40;

/// `__floatsidf` (integer arguments are passed on as doubles).
pub(crate) const USTR_VSPRINTF_FLOATSIDF: u32 = 0x0041_2550;

pub(crate) const USTR_VSPRINTF_START: u32 = 0x003d_c450;

pub(crate) const USTR_VSPRINTF_END: u32 = 0x003d_c9f8;

/// Main loop head (`s1` = current char, `s2` = the char after it).
pub(crate) const USTR_VSPRINTF_LOOP: u32 = 0x003d_c4b8;

pub(crate) const USTR_VSPRINTF_FROM_ASCII: Callee = Callee {
    addr: 0x003d_be90,
    f: ee::s14::UStr_FromAscii,
};

pub(crate) const USTR_VSPRINTF_FORMAT_FIELD: Callee = Callee {
    addr: 0x003d_c148,
    f: ee::s14::UStr_FormatNumberField,
};

/// Where a conversion continues in the main loop's tail.
pub(crate) enum UStrVsprintfTail {
    /// 0x3dc9a4: `dst += 2` (v0 holds the old `dst`), then as `Next`.
    Advance,
    /// 0x3dc9ac: `s1 = *s2`, then as `Loop`.
    Next,
    /// 0x3dc9b0: `s2 = a1`, back to the loop head.
    Loop,
}

/// The output-pointer slot `sp + 0x60`.
#[inline]
pub(crate) fn ustr_vsprintf_dst_slot(c: &Cpu) -> u32 {
    sp(c).wrapping_add(USTR_VSPRINTF_DST)
}

/// `s1 = *s2++` (the next format character).
#[inline]
pub(crate) fn ustr_vsprintf_next_char(c: &mut Cpu) {
    c.g[S1][0] = c.lhu(r32(c, S2));
    c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
}

/// `s3 += 8` (the next va_list slot); the slot's address.
#[inline]
pub(crate) fn ustr_vsprintf_next_arg(c: &mut Cpu) -> u32 {
    c.g[S3][0] = sx(r32(c, S3).wrapping_add(8));
    r32(c, S3).wrapping_sub(8)
}

/// The decimal field loops (width at 0x3dc5e8 into `s6`, precision at
/// 0x3dc670 into `s7`): `n = n * 10 + (ch - '0')` while digits follow.
pub(crate) fn ustr_vsprintf_digits(c: &mut Cpu, n: usize, head: u32) {
    loop {
        c.cycles += 8;
        c.mult(0, V0, n, V1);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x30));
        c.g[n][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.g[S1][0] = c.lhu(r32(c, S2));
        c.g[V0][0] = sx(r32(c, S1).wrapping_sub(0x30));
        c.g[V0][0] = (c.g[V0][0] < 10) as u64;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
        if c.g[V0][0] == 0 {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The integer conversions: `__floatsidf(va_arg(int))` (with `s4 |= set`
/// first), then the field arguments for `UStr_FormatNumberField`. `xor` is
/// the conversion letter whose digits are lower case (`t2 = sp`); any other
/// letter picks the upper-case digits (`t2 = fp`). `width` overrides `t0`.
pub(crate) fn ustr_vsprintf_int(
    c: &mut Cpu,
    cycles: u64,
    set: u64,
    ret: u32,
    base: u64,
    xor: Option<u64>,
    width: Option<u64>,
) -> crate::call::Flow {
    c.cycles += cycles;
    let slot = ustr_vsprintf_next_arg(c);
    c.g[S0][0] = sx(ustr_vsprintf_dst_slot(c));
    c.g[A0][0] = c.lw(slot);
    c.g[S4][0] |= set;
    call_ptr(c, USTR_VSPRINTF_FLOATSIDF, ret)?;
    if let Some(x) = xor {
        c.g[V1][0] = c.g[S1][0] ^ x;
    }
    c.g[T2][0] = c.g[SP][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = c.g[S4][0];
    c.g[T0][0] = match width {
        Some(w) => w,
        None => c.g[S6][0],
    };
    c.g[T1][0] = c.g[S7][0];
    if xor.is_some() && c.g[V1][0] != 0 {
        c.g[T2][0] = c.g[FP][0];
    }
    c.g[A2][0] = base;
    Ok(())
}

/// The float conversions (`%e`, `%g`, `%f`): the double straight from the
/// va_list, `flag` added to the flags, base 10.
pub(crate) fn ustr_vsprintf_float(c: &mut Cpu, flag: u64) {
    c.cycles += 9;
    let slot = ustr_vsprintf_next_arg(c);
    c.g[A3][0] = c.g[S4][0] | flag;
    c.g[A1][0] = c.ld(slot);
    c.g[T0][0] = c.g[S6][0];
    c.g[T1][0] = c.g[S7][0];
    c.g[A0][0] = sx(ustr_vsprintf_dst_slot(c));
    c.g[A2][0] = 10;
    c.g[T2][0] = c.g[SP][0];
}

/// `%s` (`wide`: UTF-16, `lhu`) and `%S` (bytes, sign-extended `lb`):
/// copy the string to `dst`; a null pointer prints "(null)".
pub(crate) fn ustr_vsprintf_string(c: &mut Cpu, wide: bool) -> UStrVsprintfTail {
    let (step, head) = if wide {
        (2, 0x003d_c8b8)
    } else {
        (1, 0x003d_c900)
    };
    c.cycles += 4;
    let slot = ustr_vsprintf_next_arg(c);
    c.g[V1][0] = c.lw(slot);
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
    if c.g[V1][0] == 0 {
        ustr_vsprintf_null(c);
        return UStrVsprintfTail::Next;
    }
    let load = |c: &mut Cpu| {
        let a = r32(c, V1);
        c.g[A0][0] = if wide { c.lhu(a) } else { c.lb(a) };
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(step));
    };
    c.cycles += 3;
    load(c);
    if c.g[A0][0] == 0 {
        return UStrVsprintfTail::Next;
    }
    c.cycles += 1;
    loop {
        c.cycles += 8;
        let dst_slot = ustr_vsprintf_dst_slot(c);
        c.g[V0][0] = c.lw(dst_slot);
        c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
        let ch = c.g[A0][0];
        c.sh(r32(c, V0), ch);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
        let v0 = c.g[V0][0];
        c.sw(dst_slot, v0);
        load(c);
        if c.g[A0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    c.cycles += 2;
    c.g[S1][0] = c.lhu(r32(c, S2));
    UStrVsprintfTail::Loop
}

/// "(null)" for a null `%s`/`%S` pointer (0x3dc928).
pub(crate) fn ustr_vsprintf_null(c: &mut Cpu) {
    c.cycles += 27;
    let slot = ustr_vsprintf_dst_slot(c);
    c.g[V0][0] = c.lw(slot);
    let d = r32(c, V0);
    c.sh(d, 0x28);
    let p = d.wrapping_add(2);
    c.g[V0][0] = sx(p);
    c.sw(slot, sx(p));
    c.sh(p, 0x6e);
    c.sw(slot, sx(p.wrapping_add(2)));
    c.sh(p.wrapping_add(2), 0x75);
    c.sw(slot, sx(p.wrapping_add(4)));
    c.sh(p.wrapping_add(4), 0x6c);
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(6));
    c.sw(slot, sx(p.wrapping_add(6)));
    c.sh(p.wrapping_add(6), 0x6c);
    c.sw(slot, sx(p.wrapping_add(8)));
    c.sh(p.wrapping_add(8), 0x29);
    c.sw(slot, sx(p.wrapping_add(10)));
    c.g[V1][0] = sx(p.wrapping_add(8));
    c.g[A0][0] = sx(p.wrapping_add(4));
    c.g[A2][0] = sx(p.wrapping_add(10));
    c.g[A3][0] = 0x6c;
    c.g[T0][0] = 0x29;
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
}

/// `void UStr_vsprintf(char16* dst, const char16* fmt, va_list args)`
/// (0x3dc450): printf into a UTF-16 string. Flags `-+ #0`, width and
/// precision (digits or `*`), conversions `c C d i o u x X p P e E f g G n
/// s S`; anything else is copied as is (`%%` prints `%`). Integer arguments
/// go through `__floatsidf` into `UStr_FormatNumberField`. Every va_list
/// slot is 8 bytes. Leaves the terminating 0 written and `dst` advanced
/// past it in the frame (`v0`).
pub fn ustr_vsprintf(c: &mut Cpu) {
    guest(c, ustr_vsprintf_body);
}

pub(crate) fn ustr_vsprintf_body(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 26;
    let frame = Frame::open(c, USTR_VSPRINTF_FRAME);
    frame.save(c, S2, 0xe0);
    c.g[S2][0] = c.g[A1][0];
    frame.save(c, S3, 0xd0);
    let dst = c.g[A0][0];
    let slot = ustr_vsprintf_dst_slot(c);
    c.sw(slot, dst);
    frame.save(c, S1, 0xf0);
    frame.save(c, S5, 0xb0);
    frame.save(c, FP, 0x80);
    c.g[S3][0] = c.g[A2][0];
    frame.save(c, S0, 0x100);
    frame.save(c, S4, 0xc0);
    frame.save(c, S6, 0xa0);
    frame.save(c, S7, 0x90);
    frame.save_ra(c, 0x70);
    // The hex digit strings: lower case at sp, upper case at fp = sp + 0x30.
    c.g[A1][0] = sx(USTR_VSPRINTF_LOWER_ASCII);
    c.g[A0][0] = c.g[SP][0];
    c.g[S5][0] = 0;
    USTR_VSPRINTF_FROM_ASCII.call(c, 0x003d_c49c)?;
    c.g[A0][0] = sx(sp(c).wrapping_add(USTR_VSPRINTF_UPPER_DIGITS));
    c.g[FP][0] = c.g[A0][0];
    c.g[A1][0] = sx(USTR_VSPRINTF_UPPER_ASCII);
    USTR_VSPRINTF_FROM_ASCII.call(c, 0x003d_c4b0)?;
    ustr_vsprintf_next_char(c);

    'main: loop {
        // 0x3dc4b8
        c.cycles += 2;
        c.g[V0][0] = 0x25;
        if c.g[S1][0] == 0 {
            break 'main;
        }
        c.cycles += 2;
        c.g[V1][0] = c.lw(ustr_vsprintf_dst_slot(c));
        if c.g[S1][0] != 0x25 {
            // Copy literal characters up to the next '%'.
            c.cycles += 2;
            c.g[A0][0] = 0x25;
            loop {
                c.cycles += 8;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                let ch = c.g[S1][0];
                c.sh(r32(c, V1), ch);
                let dst_slot = ustr_vsprintf_dst_slot(c);
                c.g[V0][0] = c.lw(dst_slot);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
                let v0 = c.g[V0][0];
                c.sw(dst_slot, v0);
                ustr_vsprintf_next_char(c);
                if c.g[S1][0] == 0 {
                    break 'main;
                }
                c.cycles += 2;
                c.g[V1][0] = c.lw(ustr_vsprintf_dst_slot(c));
                if c.g[S1][0] == c.g[A0][0] {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_c4d0);
                }
            }
        }
        // 0x3dc4f8: at a '%'.
        c.cycles += 2;
        c.g[A1][0] = 1;
        if c.g[S1][0] == 0 {
            break 'main;
        }
        c.cycles += 6;
        ustr_vsprintf_next_char(c);
        c.g[S4][0] = 0;
        c.g[S6][0] = 0;
        c.g[S7][0] = 6;
        c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x20));
        // Flags (0x3dc518), through the flag jump table.
        loop {
            c.cycles += 3;
            let in_table = c.g[V1][0] < 0x11;
            c.g[V0][0] = 0x0049_0000;
            if !in_table {
                c.cycles += 1;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(USTR_VSPRINTF_FLAG_TABLE - 0x0049_0000));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let flag = match r32(c, A0) {
                    0x003d_c53c => Some(0x10), // '-'
                    0x003d_c54c => Some(0x1),  // '+'
                    0x003d_c55c => Some(0x8),  // '0'
                    0x003d_c56c => Some(0x4),  // ' '
                    0x003d_c57c => Some(0x2),  // '#'
                    0x003d_c58c => None,
                    pc => {
                        c.jump_out(pc, USTR_VSPRINTF_START, USTR_VSPRINTF_END);
                        return Err(crate::call::Unwound);
                    }
                };
                match flag {
                    Some(f) => {
                        c.cycles += 4;
                        ustr_vsprintf_next_char(c);
                        c.g[S4][0] |= f;
                    }
                    None => {
                        c.cycles += 1;
                        c.g[A1][0] = 0;
                    }
                }
            }
            c.cycles += 2;
            c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x20));
            if c.g[A1][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003d_c518);
            }
        }
        // Width (0x3dc598) into s6.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 42) as u64;
        'width: {
            if c.g[S1][0] == 0x2a {
                c.cycles += 5;
                let slot = ustr_vsprintf_next_arg(c);
                c.g[S1][0] = c.lhu(r32(c, S2));
                c.g[S6][0] = c.lw(slot);
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
            } else {
                c.cycles += 2;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = 0x2e;
                if below {
                    break 'width;
                }
                c.cycles += 3;
                let above = c.g[S1][0] as i64 >= 58;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 49) as u64;
                if !above {
                    c.cycles += 2;
                    let below = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x2e;
                    if below {
                        break 'width;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 10) as u64;
                    c.g[S6][0] = 0;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[V1][0] = 10;
                        ustr_vsprintf_digits(c, S6, 0x003d_c5e8);
                    }
                }
            }
            // 0x3dc608
            c.cycles += 1;
            c.g[V0][0] = 0x2e;
        }
        // Precision (0x3dc60c) into s7, then the conversion index in v1.
        c.cycles += 2;
        'to698: {
            'to694: {
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x43));
                    break 'to694;
                }
                c.cycles += 4;
                ustr_vsprintf_next_char(c);
                c.g[V0][0] = 0x2a;
                if c.g[S1][0] == 0x2a {
                    c.cycles += 5;
                    let slot = ustr_vsprintf_next_arg(c);
                    c.g[S1][0] = c.lhu(r32(c, S2));
                    c.g[S7][0] = c.lw(slot);
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(2));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 42) as u64;
                    c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x43));
                    if c.g[V0][0] != 0 {
                        break 'to694;
                    }
                    c.cycles += 3;
                    let above = c.g[S1][0] as i64 >= 58;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 49) as u64;
                    if above {
                        break 'to694;
                    }
                    c.cycles += 2;
                    let below = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[V1][0] < 0x36) as u64;
                    if below {
                        break 'to698;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S1).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 10) as u64;
                    c.g[S7][0] = 0;
                    if c.g[V0][0] == 0 {
                        break 'to694;
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 10;
                    ustr_vsprintf_digits(c, S7, 0x003d_c670);
                }
                // 0x3dc690
                c.cycles += 1;
                c.g[V1][0] = sx(r32(c, S1).wrapping_sub(0x43));
            }
            // 0x3dc694
            c.cycles += 1;
            c.g[V0][0] = (c.g[V1][0] < 0x36) as u64;
        }
        // 0x3dc698: the conversion.
        c.cycles += 2;
        let known = c.g[V0][0] != 0;
        c.g[V0][0] = 0x0049_0000;
        let tail = if !known {
            ustr_vsprintf_literal(c)
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(USTR_VSPRINTF_CONV_TABLE - 0x0049_0000));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            match r32(c, A0) {
                // %c (UTF-16) / %C (signed byte)
                pc @ (0x003d_c6b8 | 0x003d_c6d8) => {
                    c.cycles += 8;
                    let slot = ustr_vsprintf_next_arg(c);
                    let dst_slot = ustr_vsprintf_dst_slot(c);
                    c.g[A0][0] = c.lw(dst_slot);
                    c.g[V1][0] = if pc == 0x003d_c6b8 {
                        c.lhu(slot)
                    } else {
                        c.lb(slot)
                    };
                    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
                    let ch = c.g[V1][0];
                    c.sh(r32(c, A0), ch);
                    c.g[V0][0] = c.lw(dst_slot);
                    UStrVsprintfTail::Advance
                }
                // %d %i
                0x003d_c6f8 => {
                    ustr_vsprintf_int(c, 12, 0, 0x003d_c708, 10, None, None)?;
                    ustr_vsprintf_field(c)?
                }
                // %o
                0x003d_c728 => {
                    ustr_vsprintf_int(c, 13, 0x20, 0x003d_c73c, 8, None, None)?;
                    ustr_vsprintf_field(c)?
                }
                // %u
                0x003d_c75c => {
                    ustr_vsprintf_int(c, 13, 0x20, 0x003d_c770, 10, None, None)?;
                    ustr_vsprintf_field(c)?
                }
                // %x %X
                0x003d_c790 => {
                    ustr_vsprintf_int(c, 15, 0x20, 0x003d_c7a4, 16, Some(0x78), None)?;
                    ustr_vsprintf_field(c)?
                }
                // %p %P: 8 hex digits
                0x003d_c84c => {
                    ustr_vsprintf_int(c, 14, 0x28, 0x003d_c860, 16, Some(0x70), Some(8))?;
                    ustr_vsprintf_field(c)?
                }
                // %e %E
                0x003d_c7cc => {
                    ustr_vsprintf_float(c, 0x80);
                    ustr_vsprintf_field(c)?
                }
                // %g %G
                0x003d_c7f0 => {
                    ustr_vsprintf_float(c, 0x100);
                    ustr_vsprintf_field(c)?
                }
                // %f
                0x003d_c814 => {
                    ustr_vsprintf_float(c, 0x40);
                    ustr_vsprintf_field(c)?
                }
                // %n: store the count so far
                0x003d_c838 => {
                    c.cycles += 5;
                    let slot = ustr_vsprintf_next_arg(c);
                    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
                    c.g[V0][0] = c.lw(slot);
                    let count = c.g[S5][0];
                    c.sw(r32(c, V0), count);
                    UStrVsprintfTail::Next
                }
                0x003d_c898 => ustr_vsprintf_string(c, true),
                0x003d_c8e0 => ustr_vsprintf_string(c, false),
                0x003d_c994 => ustr_vsprintf_literal(c),
                pc => {
                    c.jump_out(pc, USTR_VSPRINTF_START, USTR_VSPRINTF_END);
                    return Err(crate::call::Unwound);
                }
            }
        };
        if let UStrVsprintfTail::Advance = tail {
            c.cycles += 2;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
            let v0 = c.g[V0][0];
            c.sw(ustr_vsprintf_dst_slot(c), v0);
        }
        if !matches!(tail, UStrVsprintfTail::Loop) {
            c.cycles += 1;
            c.g[S1][0] = c.lhu(r32(c, S2));
        }
        // 0x3dc9b0
        c.cycles += 2;
        c.g[S2][0] = c.g[A1][0];
        if event_due(c) {
            deliver_event_here(c, USTR_VSPRINTF_LOOP);
        }
    }

    // 0x3dc9b8: terminate the string.
    c.cycles += 16;
    let dst_slot = ustr_vsprintf_dst_slot(c);
    c.g[V0][0] = c.lw(dst_slot);
    frame.restore(c, S0, 0x100);
    c.sh(r32(c, V0), 0);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(2));
    frame.restore(c, S1, 0xf0);
    frame.restore(c, S2, 0xe0);
    frame.restore(c, S3, 0xd0);
    frame.restore(c, S4, 0xc0);
    frame.restore(c, S5, 0xb0);
    frame.restore(c, S6, 0xa0);
    frame.restore(c, S7, 0x90);
    frame.restore(c, FP, 0x80);
    frame.restore_ra(c, 0x70);
    let v0 = c.g[V0][0];
    c.sw(dst_slot, v0);
    frame.close(c);
    Ok(())
}

/// An unknown conversion character is printed as is (0x3dc994).
pub(crate) fn ustr_vsprintf_literal(c: &mut Cpu) -> UStrVsprintfTail {
    c.cycles += 4;
    c.g[V0][0] = c.lw(ustr_vsprintf_dst_slot(c));
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(1));
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
    let ch = c.g[S1][0];
    c.sh(r32(c, V0), ch);
    UStrVsprintfTail::Advance
}

/// `count += UStr_FormatNumberField(...)` (0x3dc884).
pub(crate) fn ustr_vsprintf_field(c: &mut Cpu) -> Result<UStrVsprintfTail, crate::call::Unwound> {
    c.cycles += 5;
    USTR_VSPRINTF_FORMAT_FIELD.call(c, 0x003d_c88c)?;
    c.g[S5][0] = sx(r32(c, S5).wrapping_add(r32(c, V0)));
    c.g[A1][0] = sx(r32(c, S2).wrapping_add(2));
    Ok(UStrVsprintfTail::Next)
}

// ---------------------------------------------------------------------------
// cPS2MemCard_Update (0x2c55d8)

/// `cPS2MemCard` fields the per-frame update reads and writes.
pub(crate) const MCU_STATE: u32 = 0x4;

pub(crate) const MCU_PORT: u32 = 0xc;

pub(crate) const MCU_PORT2: u32 = 0x10;

pub(crate) const MCU_COUNT: u32 = 0x14;

pub(crate) const MCU_RESULT: u32 = 0x18;

pub(crate) const MCU_SLOT: u32 = 0x1c;

pub(crate) const MCU_SIZE: u32 = 0x20;

pub(crate) const MCU_EXPECT: u32 = 0x24;

pub(crate) const MCU_INDEX: u32 = 0x28;

pub(crate) const MCU_ASYNC_A: u32 = 0x30;

pub(crate) const MCU_ASYNC_B: u32 = 0x34;

pub(crate) const MCU_HANDLE: u32 = 0x38;

pub(crate) const MCU_ACTIVE: u32 = 0x3c;

pub(crate) const MCU_BUSY: u32 = 0x40;

/// Request flags: go to state 0x1e / 0x18 at the next update.
pub(crate) const MCU_REQ_1E: u32 = 0x44;

pub(crate) const MCU_REQ_18: u32 = 0x48;

pub(crate) const MCU_DIR_COUNT: u32 = 0x50;

pub(crate) const MCU_LIST_RESULT: u32 = 0x54;

pub(crate) const MCU_NAME: u32 = 0x58;

pub(crate) const MCU_PATH: u32 = 0x79;

pub(crate) const MCU_DIR: u32 = 0xb9;

/// Per-port info (stride 0x14): +0 type, +4 free, +8 status,
/// +0xc formatted, +0x10 changed.
pub(crate) const MCU_PORTS: u32 = 0x17c;

/// Save-slot entries (stride 0x78): +0..+8 sizes, +0xc name, +0x4c title,
/// +0x6e icon id.
pub(crate) const MCU_ENTRIES: u32 = 0x1a4;

/// Directory listing names (stride 0x40).
pub(crate) const MCU_DIRBUF: u32 = 0x4e0;

pub(crate) const MCU_ICON_ID: u32 = 0x488;

pub(crate) const MCU_CHECK: u32 = 0x490;

/// gp-relative `sceMcGetInfo` / `sceMcSync` outputs.
pub(crate) const MCU_GP_TYPE: u32 = 0x848;

pub(crate) const MCU_GP_FREE: u32 = 0x84c;

pub(crate) const MCU_GP_RESULT: u32 = 0x850;

pub(crate) const MCU_GP_FORMAT: u32 = 0x854;

/// The two state jump tables (.rodata) and the per-state check words.
pub(crate) const MCU_SYNC_TABLE: u32 = 0x0048_6210;

pub(crate) const MCU_STATE_TABLE: u32 = 0x0048_6350;

pub(crate) const MCU_CHECK_TABLE: u32 = 0x0048_7450;

pub(crate) const MCU_PATH_FMT: u32 = 0x004a_3948;

pub(crate) const MCU_PATH_EXT: u32 = 0x0048_7460;

pub(crate) const MCU_SJIS_TABLE: u32 = 0x0053_77b8;

pub(crate) const MCU_MC_SYNC: u32 = 0x0040_a360;

pub(crate) const MCU_MEM_FILL: u32 = 0x003e_6448;

pub(crate) const MCU_STRLEN: u32 = 0x0041_6810;

pub(crate) const MCU_STRCPY: u32 = 0x0041_66f4;

pub(crate) const MCU_SPRINTF: u32 = 0x0041_7828;

pub(crate) const MCU_ASYNC_IS_DONE: u32 = 0x003d_f980;

pub(crate) const MCU_ASYNC_RESULT: u32 = 0x003d_f9d8;

pub(crate) const MCU_FRAME: Frame = Frame { size: 0xb0 };

macro_rules! mcu_callee {
    ($name:ident, $addr:expr, $f:ident) => {
        const $name: Callee = Callee {
            addr: $addr,
            f: ee::s08::$f,
        };
    };
}

mcu_callee!(MCU_IS_MC_ERROR, 0x002c_63e8, cPS2MemCard_IsMcError);

mcu_callee!(MCU_SET_STATE, 0x002c_48c0, cPS2MemCard_SetState);

mcu_callee!(MCU_CLEAR_DIR_ENTRY, 0x002c_52d8, cPS2MemCard_ClearDirEntry);

mcu_callee!(MCU_DO_LIST_DIR, 0x002c_4210, cPS2MemCard_DoListDir);

mcu_callee!(
    MCU_DO_GET_SAVE_DIR_ENTRY,
    0x002c_42b0,
    cPS2MemCard_DoGetSaveDirEntry
);

mcu_callee!(MCU_SJIS_TO_ASCII, 0x002c_2268, MemCard_SjisToAsciiString);

mcu_callee!(
    MCU_MARK_SAVE_INVALID,
    0x002c_5570,
    cPS2MemCard_MarkSaveInvalid
);

mcu_callee!(
    MCU_MARK_SLOTS_WITHOUT_SPACE,
    0x002c_6280,
    cPS2MemCard_MarkSlotsWithoutSpace
);

mcu_callee!(MCU_DO_FORMAT, 0x002c_4388, cPS2MemCard_DoFormat);

mcu_callee!(MCU_DO_DELETE_SAVE, 0x002c_4410, cPS2MemCard_DoDeleteSave);

mcu_callee!(MCU_DO_LIST_DIR_ALL, 0x002c_6848, cPS2MemCard_DoListDirAll);

mcu_callee!(MCU_DO_OPEN, 0x002c_69a0, cPS2MemCard_DoOpen);

mcu_callee!(MCU_DO_CLOSE, 0x002c_6a28, cPS2MemCard_DoClose);

mcu_callee!(MCU_DO_SEEK, 0x002c_6a88, cPS2MemCard_DoSeek);

mcu_callee!(MCU_DO_FLUSH, 0x002c_6b00, cPS2MemCard_DoFlush);

mcu_callee!(MCU_DO_READ, 0x002c_6b78, cPS2MemCard_DoRead);

mcu_callee!(MCU_DO_WRITE, 0x002c_6bf8, cPS2MemCard_DoWrite);

mcu_callee!(MCU_DO_MKDIR, 0x002c_6c88, cPS2MemCard_DoMkdir);

mcu_callee!(MCU_DO_DELETE, 0x002c_6d18, cPS2MemCard_DoDelete);

mcu_callee!(MCU_GET_PORT_ERROR, 0x002c_51d0, cPS2MemCard_GetPortError);

mcu_callee!(MCU_CLEAR_PORT_INFO, 0x002c_5250, cPS2MemCard_ClearPortInfo);

mcu_callee!(MCU_CLEAR_DIR_CACHE, 0x002c_5278, cPS2MemCard_ClearDirCache);

/// `self` (s1) field `off`.
#[inline]
pub(crate) fn mcu_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S1).wrapping_add(off))
}

/// gp-relative global `off`.
#[inline]
pub(crate) fn mcu_gp(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, GP).wrapping_add(off))
}

/// `rd = sx(rs + s1)` (`addu rd, rs, s1`).
#[inline]
pub(crate) fn mcu_plus_self(c: &mut Cpu, rd: usize, rs: usize) {
    c.g[rd][0] = sx(r32(c, rs).wrapping_add(r32(c, S1)));
}

/// Where the update continues after its state handler.
#[derive(Clone, Copy)]
pub(crate) enum McuTail {
    /// `0x2c60fc`: s0 already restored, restore the rest and return.
    Epilogue,
    /// `0x2c60f8`: restore s0, then the epilogue.
    Restore,
    /// `0x2c6074`: load the port into a1, then the port checks.
    PortsLoad,
    /// `0x2c6078`: the port checks (a1 = the port).
    Ports,
}

/// `void cPS2MemCard_Update(cPS2MemCard* self)` (0x2c55d8): the memory card's
/// per-frame state machine. Collects the result of a finished `sceMc`
/// request (marking the port changed / unformatted, mapping card errors to
/// -10), polls the pending request of the waiting states, honours the two
/// request flags, then runs the current state's step: start the next
/// request (`Do*`) once the card is idle, match directory entries to save
/// slots, check save headers, finish async file reads, record results.
/// Finally drops the port's cached info / directory on a card error and
/// returns to state 0.
pub fn cps2_mem_card_update(c: &mut Cpu) {
    guest(c, |c| {
        let tail = mcu_body(c)?;
        mcu_tail(c, tail)
    })
}

pub(crate) fn mcu_body(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 19;
    let frame = Frame::open(c, 0xb0);
    frame.save(c, S1, 0x90);
    frame.save(c, S2, 0x80);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0xa0);
    c.g[S2][0] = 0x14;
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, S7, 0x30);
    frame.save(c, FP, 0x20);
    frame.save_ra(c, 0x10);
    c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
    c.mult(0, V1, V0, S2);
    mcu_plus_self(c, V0, V1);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0xc)).lw(c);
    let formatted = c.g[V1][0] != 0;
    c.g[S0][0] = 1;
    if !formatted {
        // No formatted card in the port: forget the directory.
        c.cycles += 7;
        mcu_field(c, MCU_DIR_COUNT).sw(c, 0);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(MCU_ENTRIES));
        mcu_field(c, MCU_LIST_RESULT).sw(c, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x2d0;
        call_ptr(c, MCU_MEM_FILL, 0x002c_563c)?;
        c.g[S0][0] = 1;
    }
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_BUSY).lw(c);
    let busy = c.g[V0][0] != 0;
    let one = c.g[S0][0];
    mcu_field(c, MCU_ACTIVE).sw(c, one);
    if busy && mcu_collect_request(c)? {
        return Ok(McuTail::Restore);
    }

    // The states that wait for their `sceMc` request first.
    c.cycles += 5;
    c.g[V0][0] = mcu_field(c, MCU_STATE).lw(c);
    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(2));
    c.g[V1][0] = (c.g[A0][0] < 0x4d) as u64;
    let in_table = c.g[V1][0] != 0;
    c.g[V0][0] = 0x48_0000;
    if in_table {
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(MCU_SYNC_TABLE);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let target = r32(c, A0);
        match target {
            0x002c_5794 => {
                if let Some(tail) = mcu_poll_request(c)? {
                    return Ok(tail);
                }
            }
            0x002c_58ac => {}
            _ => {
                c.jump_out(target, 0x002c_55d8, 0x002c_6128);
                return Err(Unwound);
            }
        }
    }

    // 0x2c58ac: the request flags.
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_REQ_1E).lw(c);
    c.g[A0][0] = c.g[S1][0];
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        mcu_field(c, MCU_REQ_1E).sw(c, 0);
        c.g[A1][0] = 0x1e;
        MCU_SET_STATE.call(c, 0x002c_58c4)?;
        MCU_FRAME.restore(c, S0, 0xa0);
        return Ok(McuTail::Epilogue);
    }
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_REQ_18).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        mcu_field(c, MCU_REQ_18).sw(c, 0);
        c.g[A1][0] = 0x18;
        MCU_SET_STATE.call(c, 0x002c_58e4)?;
        MCU_FRAME.restore(c, S0, 0xa0);
        return Ok(McuTail::Epilogue);
    }
    c.g[V0][0] = mcu_field(c, MCU_STATE).lw(c);

    // 0x2c58ec: the state's step.
    c.cycles += 4;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = (c.g[V0][0] < 0x4f) as u64;
    let in_table = c.g[V0][0] != 0;
    c.g[V0][0] = 0x48_0000;
    if !in_table {
        return Ok(McuTail::PortsLoad);
    }
    c.cycles += 6;
    c.g[V1][0] = sx(r32(c, A1) << 2);
    c.g[V0][0] = sx(MCU_STATE_TABLE);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    let target = r32(c, A0);
    mcu_step(c, target)
}

/// 0x2c564c..0x2c5768: the pending request finished? Store its result in
/// the port info. True when `sceMcSync` says it is still running (return).
pub(crate) fn mcu_collect_request(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 8;
    mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
    c.g[A0][0] = 1;
    let sp0 = sp(c);
    Addr(sp0).sw(c, 0);
    c.g[A2][0] = sx(r32(c, GP).wrapping_add(MCU_GP_RESULT));
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, MCU_MC_SYNC, 0x002c_5664)?;
    let running = c.g[V0][0] == 0;
    c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    if running {
        return Ok(true);
    }
    // -1 / -2: the card changed.
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    let mut changed = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = sx(0xffff_fffe);
    if !changed {
        c.cycles += 2;
        changed = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x13));
    }
    if changed {
        c.cycles += 6;
        c.g[V0][0] = mcu_field(c, MCU_PORT2).lw(c);
        c.mult(0, V1, V0, S2);
        mcu_plus_self(c, V0, V1);
        let s0 = c.g[S0][0];
        Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x10)).sw(c, s0);
        c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0x13));
    }
    // Results in -19..-11, -49..-40, -59..-50, -79..-70 become -10.
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 9) as u64;
    let mut card_error = c.g[V0][0] != 0;
    c.g[V0][0] = sx(0xffff_fff6);
    for bias in [0x31u32, 0x3b, 0x4f] {
        if card_error {
            break;
        }
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(bias));
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        card_error = c.g[V0][0] != 0;
        c.g[V0][0] = sx(0xffff_fff6);
    }
    if card_error {
        c.cycles += 1;
        let v0 = c.g[V0][0];
        mcu_gp(c, MCU_GP_RESULT).sw(c, v0);
    }

    c.cycles += 5;
    c.g[A1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    c.g[A0][0] = c.g[S1][0];
    MCU_IS_MC_ERROR.call(c, 0x002c_56e4)?;
    if c.g[V0][0] == 0 {
        // Not a card error: no PS2 card is -10002, unformatted -2.
        c.cycles += 4;
        c.g[V1][0] = mcu_gp(c, MCU_GP_TYPE).lw(c);
        c.g[V0][0] = 2;
        let not_ps2 = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = sx(0xffff_d8ee);
        let mut store = not_ps2;
        if !not_ps2 {
            c.cycles += 3;
            c.g[V0][0] = mcu_gp(c, MCU_GP_FORMAT).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(0xffff_fffe);
                store = true;
            }
        }
        if store {
            c.cycles += 2;
            let v0 = c.g[V0][0];
            mcu_gp(c, MCU_GP_RESULT).sw(c, v0);
        }
    }
    c.g[V1][0] = mcu_field(c, MCU_PORT2).lw(c);

    // 0x2c5714: record the request's port info.
    c.cycles += 21;
    c.g[A0][0] = 0x14;
    c.g[A1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    c.mult(0, V0, V1, A0);
    mcu_field(c, MCU_BUSY).sw(c, 0);
    c.g[A2][0] = mcu_gp(c, MCU_GP_TYPE).lw(c);
    c.g[A3][0] = mcu_gp(c, MCU_GP_FREE).lw(c);
    c.g[T0][0] = mcu_gp(c, MCU_GP_FORMAT).lw(c);
    mcu_plus_self(c, V1, V0);
    let a1 = c.g[A1][0];
    Addr(r32(c, V1).wrapping_add(MCU_PORTS + 0x8)).sw(c, a1);
    c.g[V0][0] = mcu_field(c, MCU_PORT2).lw(c);
    c.mult(0, V1, V0, A0);
    mcu_plus_self(c, V0, V1);
    let a2 = c.g[A2][0];
    Addr(r32(c, V0).wrapping_add(MCU_PORTS)).sw(c, a2);
    c.g[V1][0] = mcu_field(c, MCU_PORT2).lw(c);
    c.mult(0, V0, V1, A0);
    mcu_plus_self(c, V1, V0);
    let a3 = c.g[A3][0];
    Addr(r32(c, V1).wrapping_add(MCU_PORTS + 0x4)).sw(c, a3);
    c.g[V0][0] = mcu_field(c, MCU_PORT2).lw(c);
    c.mult(0, V1, V0, A0);
    mcu_plus_self(c, V0, V1);
    let t0 = c.g[T0][0];
    Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0xc)).sw(c, t0);
    Ok(false)
}

/// 0x2c5794..0x2c58ac: a waiting state polls its request (`sceMcSync`
/// without a command). Errors end the operation (state 1); some states
/// accept "no such file" (-4).
pub(crate) fn mcu_poll_request(c: &mut Cpu) -> Result<Option<McuTail>, Unwound> {
    c.cycles += 8;
    mcu_field(c, MCU_ACTIVE).sw(c, 0);
    c.g[A0][0] = 1;
    mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
    c.g[A2][0] = sx(r32(c, GP).wrapping_add(MCU_GP_RESULT));
    c.g[A1][0] = 0;
    call_ptr(c, MCU_MC_SYNC, 0x002c_57ac)?;
    let running = c.g[V0][0] == 0;
    c.g[A0][0] = 1;
    if running {
        return Ok(Some(McuTail::Restore));
    }
    c.cycles += 4;
    c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    c.g[A2][0] = u64::MAX;
    let changed = c.g[V0][0] == c.g[A2][0];
    let a0 = c.g[A0][0];
    mcu_field(c, MCU_ACTIVE).sw(c, a0);
    if changed {
        c.cycles += 5;
        c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
        c.g[V1][0] = 0x14;
        c.mult(0, A1, V0, V1);
        mcu_plus_self(c, V0, A1);
        let a0 = c.g[A0][0];
        Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x10)).sw(c, a0);
    }
    c.cycles += 3;
    c.g[A1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
    if (c.g[A1][0] as i64) < 0 {
        c.cycles += 4;
        c.g[V1][0] = mcu_field(c, MCU_STATE).lw(c);
        c.g[V0][0] = 0x24;
        let is_36 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x36));
        // Whether -4 counts as success (states 36 and 54..59).
        let mut allow_missing = is_36;
        if is_36 {
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_fffc);
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 6) as u64;
            allow_missing = c.g[V0][0] != 0;
            c.g[V0][0] = sx(0xffff_fffc);
        }
        if allow_missing {
            c.cycles += 2;
            if c.g[A1][0] == c.g[V0][0] {
                mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
            }
            c.cycles += 1;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
        } else {
            // States 8..15: a card error invalidates the save being read.
            c.cycles += 4;
            c.g[V0][0] = sx(r32(c, V1).wrapping_sub(8));
            c.g[V0][0] = (c.g[V0][0] < 8) as u64;
            let reading = c.g[V0][0] != 0;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            if reading {
                c.cycles += 2;
                if c.g[A1][0] != c.g[A2][0] {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    MCU_IS_MC_ERROR.call(c, 0x002c_5820)?;
                    let card_error = c.g[V0][0] != 0;
                    c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
                    if !card_error {
                        c.cycles += 8;
                        mcu_vcall_slot(c, V0, V1, 0x140, 0x002c_5840)?;
                        c.g[A0][0] = c.g[S1][0];
                        return mcu_invalid_exit(c).map(Some);
                    }
                }
            }
        }
        // 0x2c5858
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.cycles += 4;
            c.g[V1][0] = mcu_field(c, MCU_STATE).lw(c);
            c.g[V0][0] = 0x23;
            let mut clear = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 7;
            if !clear {
                c.cycles += 2;
                clear = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x13;
            }
            if clear {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                MCU_CLEAR_DIR_ENTRY.call(c, 0x002c_5880)?;
                c.g[V1][0] = mcu_field(c, MCU_STATE).lw(c);
                c.g[V0][0] = 0x13;
            }
            c.cycles += 2;
            let is_19 = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            if is_19 {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                mcu_field(c, MCU_SLOT).sw(c, u64::MAX);
            }
            return mcu_fail_exit(c).map(Some);
        }
    }
    c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
    // 0x2c589c: the port's status is clear.
    c.cycles += 4;
    c.g[V1][0] = 0x14;
    c.mult(0, A0, V0, V1);
    mcu_plus_self(c, V0, A0);
    Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x8)).sw(c, 0);
    Ok(None)
}

/// A virtual call on `self` through vtable slot `off` with the index:
/// `vt` = the vtable register, `func` = the function register.
pub(crate) fn mcu_vcall_slot(c: &mut Cpu, vt: usize, func: usize, off: u32, ret: u32) -> Flow {
    c.g[vt][0] = mcu_field(c, 0).lw(c);
    c.g[A1][0] = mcu_field(c, MCU_INDEX).lw(c);
    c.g[A0][0] = Addr(r32(c, vt).wrapping_add(off)).lh(c);
    c.g[func][0] = Addr(r32(c, vt).wrapping_add(off + 4)).lw(c);
    let target = r32(c, func);
    mcu_plus_self(c, A0, A0);
    call_ptr(c, target, ret)
}

/// The vtable's "card busy" query (slot 0x248), 7 cycles. True when busy.
pub(crate) fn mcu_card_busy(c: &mut Cpu, ret: u32) -> Result<bool, Unwound> {
    c.cycles += 7;
    c.g[V1][0] = mcu_field(c, 0).lw(c);
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x248)).lh(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x24c)).lw(c);
    let target = r32(c, V0);
    mcu_plus_self(c, A0, A0);
    call_ptr(c, target, ret)?;
    Ok(c.g[V0][0] != 0)
}

/// Busy: load the port and go to the port checks.
pub(crate) fn mcu_to_ports(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
    Ok(McuTail::Ports)
}

/// `cPS2MemCard_SetState(self, state)` and return (`cycles` for the block).
pub(crate) fn mcu_set_state_exit(
    c: &mut Cpu,
    cycles: u64,
    state: u64,
    ret: u32,
) -> Result<McuTail, Unwound> {
    c.cycles += cycles;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = state;
    MCU_SET_STATE.call(c, ret)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// A `Do*` request starter on `self`, then return.
pub(crate) fn mcu_do_exit(c: &mut Cpu, f: Callee, ret: u32) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    f.call(c, ret)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5bb4: the save is invalid, state 0x10 (a0 = self already).
pub(crate) fn mcu_invalid_exit(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A1][0] = 0x10;
    MCU_SET_STATE.call(c, 0x002c_5bbc)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5db4: the operation failed, state 1 (a0 = self already).
pub(crate) fn mcu_fail_exit(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A1][0] = 1;
    MCU_SET_STATE.call(c, 0x002c_5dbc)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5d48: on to the next state (a1 = the state, a0 = self).
pub(crate) fn mcu_next_state_exit(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 4;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    MCU_SET_STATE.call(c, 0x002c_5d50)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// Busy query, then a `Do*` request once the card is idle.
pub(crate) fn mcu_when_idle_do(
    c: &mut Cpu,
    busy_ret: u32,
    f: Callee,
    ret: u32,
) -> Result<McuTail, Unwound> {
    if mcu_card_busy(c, busy_ret)? {
        return mcu_to_ports(c);
    }
    mcu_do_exit(c, f, ret)
}

/// Busy query, then (with a non-negative result) a `Do*` file request.
pub(crate) fn mcu_when_idle_ok_do(
    c: &mut Cpu,
    busy_ret: u32,
    f: Callee,
    ret: u32,
) -> Result<McuTail, Unwound> {
    if mcu_card_busy(c, busy_ret)? {
        return mcu_to_ports(c);
    }
    c.cycles += 3;
    c.g[V0][0] = mcu_field(c, MCU_RESULT).lw(c);
    if (c.g[V0][0] as i64) < 0 {
        return mcu_to_ports(c);
    }
    mcu_do_exit(c, f, ret)
}

/// Busy query, then a state change once the card is idle.
pub(crate) fn mcu_when_idle_set(
    c: &mut Cpu,
    busy_ret: u32,
    state: u64,
    ret: u32,
) -> Result<McuTail, Unwound> {
    if mcu_card_busy(c, busy_ret)? {
        return mcu_to_ports(c);
    }
    mcu_set_state_exit(c, 5, state, ret)
}

/// The state's step (the second jump table's `target`; a1 = the state).
pub(crate) fn mcu_step(c: &mut Cpu, target: u32) -> Result<McuTail, Unwound> {
    match target {
        // 2: wait for the request, flag a changed card.
        0x002c_5914 => {
            c.cycles += 7;
            mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
            c.g[A0][0] = 1;
            c.g[A2][0] = sx(r32(c, GP).wrapping_add(MCU_GP_RESULT));
            c.g[A1][0] = 0;
            call_ptr(c, MCU_MC_SYNC, 0x002c_5928)?;
            let running = c.g[V0][0] == 0;
            c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            if running {
                return Ok(McuTail::Restore);
            }
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] != c.g[V0][0] {
                return mcu_to_ports(c);
            }
            c.cycles += 7;
            c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
            c.g[V1][0] = 0x14;
            c.g[A0][0] = 1;
            c.mult(0, A1, V0, V1);
            mcu_plus_self(c, V0, A1);
            Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0x10)).sw(c, 1);
            Ok(McuTail::PortsLoad)
        }
        // 3: go to 4.
        0x002c_5958 => {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 4;
            MCU_SET_STATE.call(c, 0x002c_5964)?;
            mcu_to_ports(c)
        }
        0x002c_596c => mcu_when_idle_do(c, 0x002c_5980, MCU_DO_LIST_DIR, 0x002c_5990),
        0x002c_5998 => mcu_when_idle_do(c, 0x002c_59ac, MCU_DO_GET_SAVE_DIR_ENTRY, 0x002c_59bc),
        // 6: match the directory listing to the save slots.
        0x002c_59c4 => mcu_match_slots(c),
        // 8: keep the result, go to 10.
        0x002c_5a98 => {
            c.cycles += 7;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0xa;
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_RESULT).sw(c, v0);
            MCU_SET_STATE.call(c, 0x002c_5aac)?;
            MCU_FRAME.restore(c, S0, 0xa0);
            Ok(McuTail::Epilogue)
        }
        // 10: convert the slot's title from Shift-JIS, go to 9.
        0x002c_5ab4 => {
            c.cycles += 14;
            c.g[A1][0] = mcu_field(c, MCU_INDEX).lw(c);
            c.g[V0][0] = 0x78;
            c.g[A2][0] = 0x53_0000;
            c.g[A0][0] = c.g[S1][0];
            c.mult(0, V1, A1, V0);
            c.g[A2][0] = sx(MCU_SJIS_TABLE);
            mcu_plus_self(c, A1, V1);
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(MCU_ENTRIES + 0x4c));
            MCU_SJIS_TO_ASCII.call(c, 0x002c_5ad8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 9;
            MCU_SET_STATE.call(c, 0x002c_5ae4)?;
            MCU_FRAME.restore(c, S0, 0xa0);
            Ok(McuTail::Epilogue)
        }
        // 9: go to 11.
        0x002c_5aec => mcu_set_state_exit(c, 5, 0xb, 0x002c_5af8),
        // 11: check the header, copy the icon id, go to 12.
        0x002c_5b00 => {
            c.cycles += 4;
            c.g[V1][0] = mcu_field(c, MCU_CHECK).lw(c);
            c.g[V0][0] = mcu_field(c, MCU_EXPECT).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 10;
                mcu_vcall_slot(c, V0, V1, 0x140, 0x002c_5b28)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x10;
                MCU_SET_STATE.call(c, 0x002c_5b34)?;
            }
            c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
            c.cycles += 12;
            c.g[V1][0] = 0x78;
            c.g[A0][0] = c.g[S1][0];
            c.mult(0, A2, V0, V1);
            mcu_plus_self(c, V0, A2);
            let s1 = r32(c, S1);
            c.ldl(A3, s1.wrapping_add(MCU_ICON_ID + 7));
            c.ldr(A3, s1.wrapping_add(MCU_ICON_ID));
            let dst = r32(c, V0);
            c.sdl(A3, dst.wrapping_add(MCU_ENTRIES + 0x6e + 7));
            c.sdr(A3, dst.wrapping_add(MCU_ENTRIES + 0x6e));
            c.g[A1][0] = 0xc;
            MCU_SET_STATE.call(c, 0x002c_5b60)?;
            MCU_FRAME.restore(c, S0, 0xa0);
            Ok(McuTail::Epilogue)
        }
        // 12, 13, 14: the check word must match the state's constant.
        0x002c_5b68 => {
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            let is_13 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = c.g[A1][0] ^ 0xe;
            if is_13 {
                c.cycles += 1;
                c.g[V1][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { 2 };
            }
            c.cycles += 8;
            c.g[V0][0] = 0x48_0000;
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = sx(MCU_CHECK_TABLE);
            c.g[A1][0] = mcu_field(c, MCU_CHECK).lw(c);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = Addr(r32(c, V1)).lw(c);
            if c.g[A1][0] == c.g[A0][0] {
                c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
                return mcu_next_slot(c);
            }
            c.cycles += 4;
            c.g[A1][0] = mcu_field(c, MCU_INDEX).lw(c);
            c.g[A0][0] = c.g[S1][0];
            MCU_MARK_SAVE_INVALID.call(c, 0x002c_5bb0)?;
            c.g[A0][0] = c.g[S1][0];
            mcu_invalid_exit(c)
        }
        // 15, 16: on to the next slot.
        0x002c_5be8 => {
            c.cycles += 1;
            c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
            mcu_next_slot(c)
        }
        // 7: all slots read.
        0x002c_5c00 => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            MCU_MARK_SLOTS_WITHOUT_SPACE.call(c, 0x002c_5c08)?;
            mcu_to_ports(c)
        }
        // 35: unless the result is 1, clear the directory entry.
        0x002c_5c10 => {
            c.cycles += 4;
            c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                MCU_CLEAR_DIR_ENTRY.call(c, 0x002c_5c28)?;
            }
            mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
            Ok(McuTail::PortsLoad)
        }
        // 17: format an unformatted card.
        0x002c_5c30 => {
            c.cycles += 7;
            c.g[V0][0] = mcu_field(c, MCU_PORT).lw(c);
            c.g[V1][0] = 0x14;
            c.mult(0, A0, V0, V1);
            mcu_plus_self(c, V0, A0);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MCU_PORTS + 0xc)).lw(c);
            if c.g[A0][0] != 0 {
                return mcu_to_ports(c);
            }
            mcu_do_exit(c, MCU_DO_FORMAT, 0x002c_5c54)
        }
        0x002c_5c5c => mcu_when_idle_do(c, 0x002c_5c70, MCU_DO_DELETE_SAVE, 0x002c_5c80),
        // 40, 44, 48: wait for the async file, take its result, next state.
        0x002c_5c88 => {
            c.cycles += 5;
            c.g[A0][0] = mcu_field(c, MCU_HANDLE).lw(c);
            call_ptr(c, MCU_ASYNC_IS_DONE, 0x002c_5c90)?;
            c.g[V1][0] = 1;
            let pending = c.g[V0][0] != c.g[V1][0];
            MCU_FRAME.restore(c, S0, 0xa0);
            if pending {
                return Ok(McuTail::Epilogue);
            }
            c.cycles += 8;
            c.g[A0][0] = mcu_field(c, MCU_HANDLE).lw(c);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(MCU_ASYNC_A));
            mcu_field(c, MCU_ASYNC_A).sw(c, 0);
            c.g[A2][0] = sx(r32(c, S1).wrapping_add(MCU_ASYNC_B));
            mcu_field(c, MCU_ASYNC_B).sw(c, 0);
            call_ptr(c, MCU_ASYNC_RESULT, 0x002c_5cb4)?;
            c.g[A1][0] = mcu_field(c, MCU_STATE).lw(c);
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            mcu_next_state_exit(c)
        }
        // 22, 28, 37, 41, 45, 49: keep the result, next state.
        0x002c_5cbc => {
            c.cycles += 5;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = mcu_field(c, MCU_STATE).lw(c);
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_RESULT).sw(c, v0);
            mcu_next_state_exit(c)
        }
        0x002c_5cd0 => mcu_when_idle_set(c, 0x002c_5ce4, 0x24, 0x002c_5cf8),
        0x002c_5d00 => mcu_when_idle_set(c, 0x002c_5d14, 0x1c, 0x002c_5d28),
        0x002c_5d30 => mcu_set_state_exit(c, 5, 0x3a, 0x002c_5d3c),
        // 36, 38, 39, 42, 46, 50, 54, 57, 58: next state.
        0x002c_5d44 => {
            c.cycles += 1;
            c.g[A0][0] = c.g[S1][0];
            mcu_next_state_exit(c)
        }
        0x002c_5d58 => mcu_when_idle_set(c, 0x002c_5d6c, 0x16, 0x002c_5d80),
        0x002c_5d88 => mcu_set_state_exit(c, 5, 0x34, 0x002c_5d94),
        // 29: the written size must match.
        0x002c_5d9c => {
            c.cycles += 4;
            c.g[V1][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            c.g[V0][0] = mcu_field(c, MCU_SIZE).lw(c);
            let same = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx(0xffff_d8ef);
            if same {
                return mcu_set_state_exit(c, 5, 0x35, 0x002c_5dd0);
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            mcu_gp(c, MCU_GP_RESULT).sw(c, sx(0xffff_d8ef));
            mcu_fail_exit(c)
        }
        // 25: fill in the new save's slot entry.
        0x002c_5dd8 => mcu_fill_slot(c),
        0x002c_5e48 => mcu_when_idle_do(c, 0x002c_5e5c, MCU_DO_LIST_DIR_ALL, 0x002c_5e6c),
        // 61: keep the listing's result.
        0x002c_5e74 => {
            c.cycles += 3;
            c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_LIST_RESULT).sw(c, v0);
            Ok(McuTail::PortsLoad)
        }
        0x002c_5e80 => mcu_when_idle_do(c, 0x002c_5e94, MCU_DO_OPEN, 0x002c_5ea4),
        0x002c_5eac => mcu_when_idle_ok_do(c, 0x002c_5ec0, MCU_DO_CLOSE, 0x002c_5edc),
        0x002c_5ee4 => mcu_when_idle_ok_do(c, 0x002c_5ef8, MCU_DO_SEEK, 0x002c_5f14),
        0x002c_5f1c => mcu_when_idle_ok_do(c, 0x002c_5f30, MCU_DO_FLUSH, 0x002c_5f4c),
        0x002c_5f54 => mcu_when_idle_ok_do(c, 0x002c_5f68, MCU_DO_READ, 0x002c_5f84),
        0x002c_5f8c => mcu_when_idle_ok_do(c, 0x002c_5fa0, MCU_DO_WRITE, 0x002c_5fbc),
        0x002c_5fc4 => mcu_when_idle_do(c, 0x002c_5fd8, MCU_DO_MKDIR, 0x002c_5fe8),
        0x002c_5ff0 => mcu_when_idle_do(c, 0x002c_6004, MCU_DO_DELETE, 0x002c_6014),
        // 64: done with the request's result; 74: done with -1.
        0x002c_601c | 0x002c_6024 => {
            if target == 0x002c_601c {
                c.cycles += 2;
                c.g[V0][0] = mcu_gp(c, MCU_GP_RESULT).lw(c);
            } else {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
            }
            c.cycles += 3;
            mcu_field(c, MCU_STATE).sw(c, 0);
            let v0 = c.g[V0][0];
            mcu_field(c, MCU_RESULT).sw(c, v0);
            Ok(McuTail::PortsLoad)
        }
        // 66, 68, 70, 72: done.
        0x002c_6070 => {
            c.cycles += 1;
            mcu_field(c, MCU_STATE).sw(c, 0);
            Ok(McuTail::PortsLoad)
        }
        // 0, 1, 4, ...: nothing to do.
        0x002c_6074 => Ok(McuTail::PortsLoad),
        // 52, 53: return at once.
        0x002c_60f8 => Ok(McuTail::Restore),
        _ => {
            c.jump_out(target, 0x002c_55d8, 0x002c_6128);
            Err(Unwound)
        }
    }
}

/// State 6 (0x2c59c4): clear the slot entries and give slot `i` the
/// directory entry whose name ends in the digit `i + 1`; then state 7.
pub(crate) fn mcu_match_slots(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 9;
    c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(MCU_ENTRIES));
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x2d0;
    let count = c.g[V0][0];
    mcu_field(c, MCU_DIR_COUNT).sw(c, count);
    call_ptr(c, MCU_MEM_FILL, 0x002c_59dc)?;
    c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A0][0] = 0;
    if !none {
        c.cycles += 2;
        c.g[FP][0] = 0x78;
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x31));
        loop {
            // 0x2c59f0: slot a0, digit s6.
            c.cycles += 5;
            c.g[V1][0] = mcu_field(c, MCU_DIR_COUNT).lw(c);
            c.g[S6][0] = c.g[V0][0] & 0xff;
            c.g[S2][0] = 0;
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[S7][0] = sx(r32(c, A0).wrapping_add(1));
            if empty {
                c.cycles += 1;
                c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
            } else {
                c.cycles += 7;
                c.mult(0, V0, A0, FP);
                c.g[S4][0] = sx(r32(c, S1).wrapping_add(MCU_DIRBUF));
                c.g[S3][0] = u64::MAX;
                c.g[S0][0] = c.g[S4][0];
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(MCU_ENTRIES));
                mcu_plus_self(c, S5, V0);
                loop {
                    // 0x2c5a20: directory entry s2 (name at s0).
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, MCU_STRLEN, 0x002c_5a28)?;
                    let blank = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                    if blank {
                        c.cycles += 1;
                        c.g[V0][0] = mcu_field(c, MCU_DIR_COUNT).lw(c);
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                        c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                        if c.g[V1][0] != c.g[S6][0] {
                            c.g[V0][0] = mcu_field(c, MCU_DIR_COUNT).lw(c);
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = sx(r32(c, S5).wrapping_add(0xc));
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, MCU_STRCPY, 0x002c_5a4c)?;
                            c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
                            break;
                        }
                    }
                    // 0x2c5a58
                    c.cycles += 5;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(0x40));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x40));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002c_5a20);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = mcu_field(c, MCU_COUNT).lw(c);
                    break;
                }
            }
            // 0x2c5a70
            c.cycles += 4;
            c.g[A0][0] = c.g[S7][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(0x31));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_59f0);
            }
        }
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S1][0];
    mcu_gp(c, MCU_GP_RESULT).sw(c, 0);
    c.g[A1][0] = 7;
    MCU_SET_STATE.call(c, 0x002c_5a90)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// 0x2c5bec: advance to the next slot that holds a save (vtable slot
/// 0x218) and build its path (state 8); past the last slot, state 7's
/// free-space marking. v0 = the current index.
pub(crate) fn mcu_next_slot(c: &mut Cpu) -> Result<McuTail, Unwound> {
    loop {
        c.cycles += 5;
        c.g[V1][0] = mcu_field(c, MCU_COUNT).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        let index = c.g[V0][0];
        mcu_field(c, MCU_INDEX).sw(c, index);
        if !more {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            MCU_MARK_SLOTS_WITHOUT_SPACE.call(c, 0x002c_5c08)?;
            return mcu_to_ports(c);
        }
        if event_due(c) {
            deliver_event_here(c, 0x002c_5bc8);
        }
        // 0x2c5bc8
        c.cycles += 8;
        mcu_vcall_slot(c, V1, V0, 0x218, 0x002c_5be0)?;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = mcu_field(c, MCU_INDEX).lw(c);
            return mcu_build_path(c);
        }
        c.cycles += 1;
        c.g[V0][0] = mcu_field(c, MCU_INDEX).lw(c);
    }
}

/// 0x2c6034: `sprintf(self->path, fmt, entries[a2].name, ext)`, state 8.
pub(crate) fn mcu_build_path(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 15;
    c.g[V0][0] = 0x78;
    c.g[A1][0] = 0x4a_0000;
    c.g[A3][0] = 0x48_0000;
    c.mult(0, V1, A2, V0);
    c.g[A1][0] = sx(MCU_PATH_FMT);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(MCU_PATH));
    c.g[A3][0] = sx(MCU_PATH_EXT);
    mcu_plus_self(c, A2, V1);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(MCU_ENTRIES + 0xc));
    call_ptr(c, MCU_SPRINTF, 0x002c_605c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 8;
    MCU_SET_STATE.call(c, 0x002c_6068)?;
    MCU_FRAME.restore(c, S0, 0xa0);
    Ok(McuTail::Epilogue)
}

/// State 25 (0x2c5dd8): the new save's slot entry gets zero sizes, the
/// directory name and the title.
pub(crate) fn mcu_fill_slot(c: &mut Cpu) -> Result<McuTail, Unwound> {
    c.cycles += 28;
    c.g[V1][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.g[S0][0] = 0x78;
    mcu_field(c, MCU_ACTIVE).sw(c, 0);
    c.g[A1][0] = sx(r32(c, S1).wrapping_add(MCU_DIR));
    c.mult(0, V0, V1, S0);
    mcu_plus_self(c, V1, V0);
    Addr(r32(c, V1).wrapping_add(MCU_ENTRIES + 4)).sw(c, 0);
    c.g[V0][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.mult(0, V1, V0, S0);
    mcu_plus_self(c, V0, V1);
    Addr(r32(c, V0).wrapping_add(MCU_ENTRIES)).sw(c, 0);
    c.g[V1][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.mult(0, V0, V1, S0);
    mcu_plus_self(c, V1, V0);
    Addr(r32(c, V1).wrapping_add(MCU_ENTRIES + 8)).sw(c, 0);
    c.g[A0][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.mult(0, V0, A0, S0);
    mcu_plus_self(c, A0, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MCU_ENTRIES + 0xc));
    call_ptr(c, MCU_STRCPY, 0x002c_5e28)?;
    c.g[A0][0] = mcu_field(c, MCU_SLOT).lw(c);
    c.g[A1][0] = sx(r32(c, S1).wrapping_add(MCU_NAME));
    c.mult(0, V0, A0, S0);
    mcu_plus_self(c, A0, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MCU_ENTRIES + 0x4c));
    call_ptr(c, MCU_STRCPY, 0x002c_5e40)?;
    mcu_to_ports(c)
}

/// The shared tails: port-error checks (drop the port's info on -10/-1,
/// its directory cache on -10/-2, back to state 0), then the epilogue.
pub(crate) fn mcu_tail(c: &mut Cpu, mut tail: McuTail) -> Flow {
    if let McuTail::PortsLoad = tail {
        c.cycles += 1;
        c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
        tail = McuTail::Ports;
    }
    if let McuTail::Ports = tail {
        for (first, second, wanted, clear, clear_ret) in [
            (
                0x002c_6080,
                0x002c_6098,
                u64::MAX,
                MCU_CLEAR_PORT_INFO,
                0x002c_60ac,
            ),
            (
                0x002c_60b8,
                0x002c_60d0,
                sx(0xffff_fffe),
                MCU_CLEAR_DIR_CACHE,
                0x002c_60e4,
            ),
        ] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            MCU_GET_PORT_ERROR.call(c, first)?;
            c.g[V1][0] = sx(0xffff_fff6);
            let mut hit = c.g[V0][0] == c.g[V1][0];
            if !hit {
                c.cycles += 6;
                c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
                c.g[A0][0] = c.g[S1][0];
                MCU_GET_PORT_ERROR.call(c, second)?;
                c.g[V1][0] = wanted;
                hit = c.g[V0][0] == c.g[V1][0];
            }
            if hit {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                clear.call(c, clear_ret)?;
            }
            if clear_ret == 0x002c_60ac {
                c.g[A1][0] = mcu_field(c, MCU_PORT).lw(c);
            } else {
                c.g[V0][0] = mcu_field(c, MCU_STATE).lw(c);
            }
        }
        c.cycles += 2;
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !idle {
            c.cycles += 2;
            c.g[A1][0] = 0;
            MCU_SET_STATE.call(c, 0x002c_60f8)?;
        }
        tail = McuTail::Restore;
    }
    if let McuTail::Restore = tail {
        c.cycles += 1;
        MCU_FRAME.restore(c, S0, 0xa0);
    }
    c.cycles += 11;
    MCU_FRAME.restore(c, S1, 0x90);
    MCU_FRAME.restore(c, S2, 0x80);
    MCU_FRAME.restore(c, S3, 0x70);
    MCU_FRAME.restore(c, S4, 0x60);
    MCU_FRAME.restore(c, S5, 0x50);
    MCU_FRAME.restore(c, S6, 0x40);
    MCU_FRAME.restore(c, S7, 0x30);
    MCU_FRAME.restore(c, FP, 0x20);
    MCU_FRAME.restore_ra(c, 0x10);
    MCU_FRAME.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// LocStringTable_GetStringPtr (0x3dbb68)

pub(crate) const LOC_STRING_TABLE_REMAP_ID: Callee = Callee {
    addr: 0x003d_bae8,
    f: ee::s14::LocStringTable_RemapId,
};

/// Read the unaligned little-endian word at `base` byte by byte as the
/// original does: byte 0 into `r0`, byte 1 (<< 8) into `r1`, byte 2 (<< 16)
/// into `r2`, and the word into `out` (which held byte 3 << 24).
pub(crate) fn loc_string_table_rd32(
    c: &mut Cpu,
    base: u32,
    r0: usize,
    r1: usize,
    r2: usize,
    out: usize,
) {
    c.g[r0][0] = Addr(base).lbu(c);
    c.g[out][0] = Addr(base.wrapping_add(3)).lbu(c);
    c.g[r2][0] = Addr(base.wrapping_add(2)).lbu(c);
    c.g[r1][0] = Addr(base.wrapping_add(1)).lbu(c);
    c.g[out][0] = sx(r32(c, out) << 24);
    c.g[r2][0] = sx(r32(c, r2) << 16);
    c.g[r1][0] = sx(r32(c, r1) << 8);
    c.g[out][0] |= c.g[r2][0] | c.g[r1][0] | c.g[r0][0];
}

/// `const u8* LocStringTable_GetStringPtr(LocStringTable* self, int id)`
/// (0x3dbb68): with the remap flag (`+8` bit 0) map `id` through the remap
/// table first; then, in the current language's string table (offset
/// `+0x10 + 4 * lang`, `lang` at `+0xe`), return the string `id` (offsets
/// relative to the table), or null when `id` is negative or out of range.
/// All fields are unaligned little-endian words.
pub fn loc_string_table_get_string_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(8)).lbu(c) & 1;
        c.g[T0][0] = c.g[A1][0]; // id
        if c.g[V0][0] != 0 {
            c.cycles += 15;
            c.g[V1][0] = sx(this.wrapping_add(4));
            c.g[A0][0] = c.g[T0][0] & 0xffff;
            loc_string_table_rd32(c, this.wrapping_add(4), A3, A2, V0, A1);
            c.g[A1][0] = sx(this.wrapping_add(r32(c, A1)));
            LOC_STRING_TABLE_REMAP_ID.call(c, 0x003d_bbc0)?;
            c.g[T0][0] = c.g[V0][0];
        }
        // The language's table.
        c.cycles += 19;
        c.g[V0][0] = Addr(this.wrapping_add(0xf)).lbu(c);
        c.g[V1][0] = Addr(this.wrapping_add(0xe)).lbu(c);
        c.g[V0][0] = sx(r32(c, V0) << 8) | c.g[V1][0];
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(this).wrapping_add(0x10));
        let entry = r32(c, V0);
        loc_string_table_rd32(c, entry, A2, A1, A0, V1);
        c.g[A3][0] = sx(this.wrapping_add(r32(c, V1)));
        let mut found = false;
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 14;
            let table = r32(c, A3);
            c.g[V1][0] = sx(table.wrapping_add(0xc));
            loc_string_table_rd32(c, table.wrapping_add(0xc), A2, A1, A0, V0);
            let in_range = c.g[T0][0] < c.g[V0][0];
            c.g[V0][0] = sx(r32(c, T0) << 2);
            if in_range {
                c.cycles += 14;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(table).wrapping_add(0x10));
                let entry = r32(c, V0);
                loc_string_table_rd32(c, entry, A2, A1, A0, V1);
                c.g[A3][0] = sx(table.wrapping_add(r32(c, V1)));
                found = true;
            }
        }
        if !found {
            c.cycles += 1;
            c.g[A3][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[A3][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Where a stored word comes from: an immediate the code builds in a
/// register (`lui`/`mtc1`) or a literal-pool float loaded from `gp + offset`.
#[derive(Clone, Copy)]
pub(crate) enum Src {
    Lit(u32),
    Pool(i16),
}

/// Size of one surface-property row, and the offset of its curve.
pub(crate) const SPIC_ROW_SIZE: u32 = 0xb0;

pub(crate) const SPIC_CURVE_OFF: u32 = 0x90;

pub(crate) const SPIC_ROWS: u32 = 19;

/// `memcpy`-like helper copying each row's tuned curve (0x20 bytes).
pub(crate) const SPIC_COPY: u32 = 0x003e_6574;

/// Rows copy their curve in this order (row 18 comes after row 7).
pub(crate) const SPIC_COPY_ORDER: [u32; 19] = [
    0, 1, 2, 3, 4, 5, 6, 7, 18, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
];

/// The value of `s` (the pool is read from memory at `gp`).
pub(crate) fn spic_val(c: &mut Cpu, gp: u32, s: Src) -> u32 {
    match s {
        Src::Lit(v) => v,
        Src::Pool(o) => c.bus.read32(gp.wrapping_add(o as i32 as u32)),
    }
}

/// `sw`/`swc1` of `s` to `at`.
pub(crate) fn spic_put(c: &mut Cpu, gp: u32, at: u32, s: Src) {
    let v = spic_val(c, gp, s);
    c.sw(at, v as u64);
}

/// `void SurfPropCache_InitDefaults(SurfPropCache* self)` (0x2c2f18): fills
/// the 19 surface-property rows (0xb0 bytes: physics tuning floats at
/// 0x00..0x8c and a four-point curve at 0x90) with the defaults, writes the
/// tuned fields of rows 0-17, and copies each row's tuned curve in from the
/// static tables at `gp + 0x7b4 ...` (`EA_MemCopy`, 0x20 bytes).
///
/// A flat table port of the straight-line stores: the registers the compiled
/// code leaves (GPR temporaries, FPRs, LO/HI) are set from a simulation of it,
/// per loop iteration (an event can arrive at each back edge) and at the
/// first call.
pub fn surf_prop_cache_init_defaults(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x002c_2f18);
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x90);
        let gp = c.g[GP][0] as u32;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        for k in 0..12 {
            frame.save_fpr(c, 31 - k, 0x88 - 8 * k as u32);
        }
        let mut i = 0u32;
        loop {
            c.cycles += 92;
            let s0 = c.g[S0][0] as u32;
            let row = s0.wrapping_add(i * SPIC_ROW_SIZE);
            c.g[V0][0] = SPIC_ROW_SIZE as u64;
            c.g[T7][0] = i as u64;
            c.mult(0, V0, T7, V0);
            for &(off, s) in SPIC_ROW_DEFAULTS {
                spic_put(c, gp, row.wrapping_add(off as u32), s);
            }
            // Each curve point goes through the frame's scratch doubleword
            // (`sp + 0` x, `sp + 4` y) and is stored as one unaligned 64-bit.
            let scratch = sp(c);
            for (k, &(x, y)) in SPIC_CURVE.iter().enumerate() {
                spic_put(c, gp, scratch, x);
                spic_put(c, gp, scratch.wrapping_add(4), y);
                let w = c.ld(scratch);
                Addr(row.wrapping_add(SPIC_CURVE_OFF + 8 * k as u32)).write_u64_unaligned(c, w);
            }
            spic_loop_regs(c, gp, s0, row, i + 1);
            if i + 1 < SPIC_ROWS {
                if event_due(c) {
                    deliver_event_here(c, 0x002c_2fa0);
                }
                i += 1;
                continue;
            }
            break;
        }
        c.cycles += 932;
        let s0 = c.g[S0][0] as u32;
        for &(off, s) in SPIC_TAIL_FIELDS {
            spic_put(c, gp, s0.wrapping_add(off as u32), s);
        }
        // Registers at the first copy call.
        let last = s0.wrapping_add(18 * SPIC_ROW_SIZE);
        c.g[AT][0] = 0x3f40_0000;
        c.g[T7][0] = 19;
        c.g[T5][0] = 0;
        c.g[T6][0] = 1;
        c.g[T8][0] = sx(s0.wrapping_add(0x6c));
        c.g[V0][0] = SPIC_ROW_SIZE as u64;
        for r in [V1, A3, T0, T1, T2, T3, T4] {
            c.g[r][0] = sx(last);
        }
        c.lwc1(0, gp.wrapping_sub(0x4004));
        c.f[1] = 0x40800000;
        c.f[2] = 0x3f800000;
        c.lwc1(3, gp.wrapping_sub(0x4010));
        c.lwc1(4, gp.wrapping_sub(0x400c));
        c.f[5] = 0x40a00000;
        c.lwc1(6, gp.wrapping_sub(0x4144));
        c.lwc1(7, gp.wrapping_sub(0x4018));
        c.lwc1(8, gp.wrapping_sub(0x4014));
        c.lwc1(9, gp.wrapping_sub(0x4068));
        c.f[10] = 0x3f000000;
        c.lwc1(11, gp.wrapping_sub(0x4064));
        c.lwc1(12, gp.wrapping_sub(0x4140));
        c.lwc1(13, gp.wrapping_sub(0x40c4));
        c.lwc1(14, gp.wrapping_sub(0x40c0));
        c.lwc1(15, gp.wrapping_sub(0x40bc));
        c.lwc1(16, gp.wrapping_sub(0x40b8));
        c.lwc1(17, gp.wrapping_sub(0x40b4));
        c.lwc1(18, gp.wrapping_sub(0x40b0));
        c.lwc1(19, gp.wrapping_sub(0x40ac));
        c.lwc1(20, gp.wrapping_sub(0x40a4));
        c.lwc1(21, gp.wrapping_sub(0x40a0));
        c.lwc1(22, gp.wrapping_sub(0x409c));
        c.lwc1(23, gp.wrapping_sub(0x40d0));
        c.lwc1(24, gp.wrapping_sub(0x40cc));
        c.lwc1(25, gp.wrapping_sub(0x4138));
        c.lwc1(26, gp.wrapping_sub(0x4134));
        c.lwc1(27, gp.wrapping_sub(0x4130));
        c.lwc1(28, gp.wrapping_sub(0x412c));
        c.f[29] = 0x42340000;
        c.f[30] = 0x41d80000;
        c.lwc1(31, gp.wrapping_sub(0x4154));
        for (k, &row) in SPIC_COPY_ORDER.iter().enumerate() {
            let s0 = c.g[S0][0] as u32;
            c.g[A1][0] = c.lw(gp.wrapping_add(0x7b4 + 8 * k as u32));
            c.g[A0][0] = sx(s0.wrapping_add(SPIC_CURVE_OFF + SPIC_ROW_SIZE * row));
            c.g[A2][0] = 0x20;
            call_ptr(c, SPIC_COPY, 0x002c_3e40 + 0x10 * k as u32)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        for k in 0..12 {
            frame.restore_fpr(c, 31 - k, 0x88 - 8 * k as u32);
        }
        frame.close(c);
        Ok(())
    });
}

/// Default fields of every row (row-relative byte offset, value), `f00`..`f8C` (the curve at `0x90` follows).
pub(crate) const SPIC_ROW_DEFAULTS: &[(u16, Src)] = &[
    (0x0, Src::Lit(0x44750000)),
    (0x4, Src::Lit(0x0)),
    (0x8, Src::Lit(0x3f800000)),
    (0xc, Src::Pool(-0x4150)),
    (0x10, Src::Lit(0x41a00000)),
    (0x14, Src::Lit(0x41200000)),
    (0x18, Src::Lit(0x41a00000)),
    (0x1c, Src::Lit(0x41f00000)),
    (0x20, Src::Lit(0x42520000)),
    (0x24, Src::Pool(-0x414c)),
    (0x28, Src::Pool(-0x4168)),
    (0x2c, Src::Pool(-0x4154)),
    (0x30, Src::Lit(0x3f800000)),
    (0x34, Src::Lit(0x41d80000)),
    (0x38, Src::Lit(0x41a00000)),
    (0x3c, Src::Lit(0x3f800000)),
    (0x40, Src::Lit(0x0)),
    (0x44, Src::Lit(0x0)),
    (0x48, Src::Lit(0x0)),
    (0x4c, Src::Lit(0x3f800000)),
    (0x50, Src::Lit(0x0)),
    (0x54, Src::Lit(0x42340000)),
    (0x58, Src::Lit(0x0)),
    (0x5c, Src::Lit(0x0)),
    (0x60, Src::Lit(0x0)),
    (0x64, Src::Lit(0x0)),
    (0x68, Src::Lit(0x0)),
    (0x6c, Src::Lit(0x0)),
    (0x70, Src::Lit(0x0)),
    (0x74, Src::Lit(0x0)),
    (0x78, Src::Lit(0x1)),
    (0x7c, Src::Pool(-0x4168)),
    (0x80, Src::Pool(-0x4168)),
    (0x84, Src::Lit(0x0)),
    (0x88, Src::Lit(0x0)),
    (0x8c, Src::Lit(0x0)),
];

/// Per-row overrides written after the loop (offset from the cache base, value): the tuned values of rows 0-17.
pub(crate) const SPIC_TAIL_FIELDS: &[(u16, Src)] = &[
    (0x8, Src::Pool(-0x4148)),
    (0x14, Src::Lit(0x3f000000)),
    (0xc4, Src::Lit(0x3f000000)),
    (0x28, Src::Pool(-0x4144)),
    (0x30, Src::Lit(0x3f800000)),
    (0x3c, Src::Lit(0x3f800000)),
    (0xd0, Src::Pool(-0x4140)),
    (0xd8, Src::Pool(-0x4144)),
    (0xe0, Src::Lit(0x3f800000)),
    (0xc, Src::Pool(-0x4128)),
    (0x20, Src::Pool(-0x4124)),
    (0xb8, Src::Pool(-0x4120)),
    (0xbc, Src::Pool(-0x411c)),
    (0x0, Src::Pool(-0x413c)),
    (0x10, Src::Pool(-0x4138)),
    (0x18, Src::Pool(-0x4134)),
    (0x1c, Src::Pool(-0x4130)),
    (0x24, Src::Pool(-0x412c)),
    (0x2c, Src::Pool(-0x4154)),
    (0x34, Src::Lit(0x41d80000)),
    (0x38, Src::Lit(0x41a00000)),
    (0xb0, Src::Pool(-0x413c)),
    (0xc0, Src::Pool(-0x4138)),
    (0xc8, Src::Pool(-0x4134)),
    (0xcc, Src::Pool(-0x4130)),
    (0xd4, Src::Pool(-0x412c)),
    (0xdc, Src::Pool(-0x4154)),
    (0xe4, Src::Lit(0x41d80000)),
    (0xe8, Src::Lit(0x41a00000)),
    (0x4, Src::Lit(0x0)),
    (0x40, Src::Lit(0x0)),
    (0x44, Src::Lit(0x0)),
    (0xb4, Src::Lit(0x0)),
    (0xec, Src::Lit(0x3f800000)),
    (0x160, Src::Pool(-0x4118)),
    (0x164, Src::Lit(0xbf800000)),
    (0x214, Src::Lit(0xbf800000)),
    (0x188, Src::Pool(-0x4144)),
    (0x190, Src::Lit(0x3f800000)),
    (0x238, Src::Pool(-0x4144)),
    (0x240, Src::Lit(0x3f800000)),
    (0x168, Src::Pool(-0x4114)),
    (0x16c, Src::Pool(-0x4110)),
    (0x170, Src::Pool(-0x410c)),
    (0x174, Src::Pool(-0x4108)),
    (0x178, Src::Pool(-0x4104)),
    (0x17c, Src::Pool(-0x4100)),
    (0x180, Src::Pool(-0x40fc)),
    (0x184, Src::Pool(-0x40f8)),
    (0x19c, Src::Pool(-0x40f4)),
    (0x1a0, Src::Pool(-0x40f0)),
    (0x210, Src::Pool(-0x40ec)),
    (0x218, Src::Pool(-0x40e8)),
    (0x21c, Src::Pool(-0x40e4)),
    (0x220, Src::Pool(-0x40e0)),
    (0x224, Src::Pool(-0x40dc)),
    (0x228, Src::Pool(-0x40d8)),
    (0x22c, Src::Pool(-0x40d4)),
    (0x230, Src::Pool(-0x40d0)),
    (0x234, Src::Pool(-0x40cc)),
    (0x18c, Src::Pool(-0x4154)),
    (0x194, Src::Lit(0x41d80000)),
    (0x198, Src::Lit(0x41a00000)),
    (0x23c, Src::Pool(-0x4154)),
    (0xf0, Src::Lit(0x0)),
    (0xf4, Src::Lit(0x0)),
    (0x1a4, Src::Lit(0x0)),
    (0x244, Src::Lit(0x41d80000)),
    (0x24c, Src::Pool(-0x40c8)),
    (0x2d4, Src::Lit(0x3f000000)),
    (0x384, Src::Lit(0x3f000000)),
    (0x2c0, Src::Pool(-0x40c0)),
    (0x2c4, Src::Lit(0x3f800000)),
    (0x2c8, Src::Pool(-0x40bc)),
    (0x2cc, Src::Pool(-0x40b8)),
    (0x2d0, Src::Pool(-0x40b4)),
    (0x2d8, Src::Pool(-0x40b0)),
    (0x2dc, Src::Pool(-0x40ac)),
    (0x2e8, Src::Pool(-0x4144)),
    (0x2f0, Src::Lit(0x3f800000)),
    (0x2fc, Src::Lit(0x3f800000)),
    (0x37c, Src::Pool(-0x40c4)),
    (0x390, Src::Pool(-0x4140)),
    (0x398, Src::Pool(-0x4144)),
    (0x250, Src::Pool(-0x4098)),
    (0x248, Src::Lit(0x41a00000)),
    (0x2e0, Src::Pool(-0x40a4)),
    (0x2e4, Src::Pool(-0x40a0)),
    (0x2ec, Src::Pool(-0x4154)),
    (0x2f4, Src::Pool(-0x409c)),
    (0x2f8, Src::Lit(0x41a00000)),
    (0x370, Src::Pool(-0x413c)),
    (0x378, Src::Pool(-0x40a8)),
    (0x380, Src::Pool(-0x4138)),
    (0x388, Src::Pool(-0x4134)),
    (0x38c, Src::Pool(-0x4130)),
    (0x394, Src::Pool(-0x412c)),
    (0x254, Src::Lit(0x0)),
    (0x300, Src::Lit(0x0)),
    (0x304, Src::Lit(0x0)),
    (0x374, Src::Lit(0x0)),
    (0x39c, Src::Pool(-0x4154)),
    (0x4d0, Src::Pool(-0x4094)),
    (0x434, Src::Lit(0x3f000000)),
    (0x4e4, Src::Lit(0x3f000000)),
    (0x4ec, Src::Lit(0x41f00000)),
    (0x3a0, Src::Lit(0x3f800000)),
    (0x3ac, Src::Lit(0x3f800000)),
    (0x42c, Src::Pool(-0x40c4)),
    (0x440, Src::Pool(-0x4140)),
    (0x448, Src::Pool(-0x4144)),
    (0x450, Src::Lit(0x3f800000)),
    (0x45c, Src::Lit(0x3f800000)),
    (0x4d8, Src::Pool(-0x4090)),
    (0x4dc, Src::Pool(-0x408c)),
    (0x4e0, Src::Pool(-0x4088)),
    (0x4e8, Src::Pool(-0x4084)),
    (0x4f0, Src::Pool(-0x4080)),
    (0x3a4, Src::Lit(0x41d80000)),
    (0x3a8, Src::Lit(0x41a00000)),
    (0x420, Src::Pool(-0x413c)),
    (0x428, Src::Pool(-0x40a8)),
    (0x430, Src::Pool(-0x4138)),
    (0x438, Src::Pool(-0x4134)),
    (0x43c, Src::Pool(-0x4130)),
    (0x444, Src::Pool(-0x412c)),
    (0x44c, Src::Pool(-0x4154)),
    (0x454, Src::Lit(0x41d80000)),
    (0x458, Src::Lit(0x41a00000)),
    (0x3b0, Src::Lit(0x0)),
    (0x3b4, Src::Lit(0x0)),
    (0x424, Src::Lit(0x0)),
    (0x460, Src::Lit(0x0)),
    (0x464, Src::Lit(0x0)),
    (0x4d4, Src::Lit(0x0)),
    (0x4f4, Src::Pool(-0x407c)),
    (0xc60, Src::Lit(0x44750000)),
    (0x4f8, Src::Pool(-0x4144)),
    (0xc6c, Src::Pool(-0x4078)),
    (0x500, Src::Lit(0x3f800000)),
    (0xc74, Src::Lit(0x41200000)),
    (0x50c, Src::Lit(0x3f800000)),
    (0xc7c, Src::Lit(0x41f00000)),
    (0xc68, Src::Lit(0x3f800000)),
    (0xc80, Src::Lit(0x42520000)),
    (0xc88, Src::Pool(-0x4144)),
    (0xc84, Src::Pool(-0x4074)),
    (0xc90, Src::Lit(0x3f800000)),
    (0xc9c, Src::Lit(0x3f800000)),
    (0x58c, Src::Pool(-0x40c4)),
    (0x4fc, Src::Pool(-0x4154)),
    (0x504, Src::Lit(0x41d80000)),
    (0x508, Src::Lit(0x41a00000)),
    (0xc70, Src::Lit(0x41a00000)),
    (0xc78, Src::Lit(0x41a00000)),
    (0xc8c, Src::Pool(-0x4154)),
    (0xc94, Src::Lit(0x41d80000)),
    (0xc98, Src::Lit(0x41a00000)),
    (0x580, Src::Pool(-0x413c)),
    (0x588, Src::Pool(-0x40a8)),
    (0x590, Src::Pool(-0x4138)),
    (0x594, Src::Lit(0x3f000000)),
    (0x598, Src::Pool(-0x4134)),
    (0x510, Src::Lit(0x0)),
    (0x514, Src::Lit(0x0)),
    (0xc64, Src::Lit(0x0)),
    (0xca0, Src::Lit(0x0)),
    (0xca4, Src::Lit(0x0)),
    (0x584, Src::Lit(0x0)),
    (0x59c, Src::Pool(-0x4130)),
    (0x5a0, Src::Pool(-0x4140)),
    (0x5a8, Src::Pool(-0x4144)),
    (0x5b0, Src::Lit(0x3f800000)),
    (0x5bc, Src::Lit(0x3f800000)),
    (0x63c, Src::Pool(-0x40c4)),
    (0x650, Src::Pool(-0x4140)),
    (0x658, Src::Pool(-0x4144)),
    (0x660, Src::Lit(0x3f800000)),
    (0x66c, Src::Lit(0x3f800000)),
    (0x6ec, Src::Pool(-0x40c4)),
    (0x5a4, Src::Pool(-0x412c)),
    (0x5ac, Src::Pool(-0x4154)),
    (0x5b4, Src::Lit(0x41d80000)),
    (0x5b8, Src::Lit(0x41a00000)),
    (0x630, Src::Pool(-0x413c)),
    (0x638, Src::Pool(-0x40a8)),
    (0x640, Src::Pool(-0x4138)),
    (0x644, Src::Lit(0x3f000000)),
    (0x648, Src::Pool(-0x4134)),
    (0x64c, Src::Pool(-0x4130)),
    (0x654, Src::Pool(-0x412c)),
    (0x65c, Src::Pool(-0x4154)),
    (0x664, Src::Lit(0x41d80000)),
    (0x668, Src::Lit(0x41a00000)),
    (0x6e0, Src::Pool(-0x413c)),
    (0x6e8, Src::Pool(-0x40a8)),
    (0x6f0, Src::Pool(-0x4138)),
    (0x5c0, Src::Lit(0x0)),
    (0x5c4, Src::Lit(0x0)),
    (0x634, Src::Lit(0x0)),
    (0x670, Src::Lit(0x0)),
    (0x674, Src::Lit(0x0)),
    (0x6e4, Src::Lit(0x0)),
    (0x6f4, Src::Lit(0x3f000000)),
    (0x7a4, Src::Lit(0x3f000000)),
    (0x790, Src::Pool(-0x40c0)),
    (0x798, Src::Pool(-0x40bc)),
    (0x79c, Src::Pool(-0x40b8)),
    (0x7a0, Src::Pool(-0x40b4)),
    (0x7a8, Src::Pool(-0x40b0)),
    (0x7ac, Src::Pool(-0x40ac)),
    (0x700, Src::Pool(-0x4140)),
    (0x708, Src::Pool(-0x4144)),
    (0x710, Src::Lit(0x3f800000)),
    (0x71c, Src::Lit(0x3f800000)),
    (0x794, Src::Lit(0x3f800000)),
    (0x7b8, Src::Pool(-0x4144)),
    (0x7c0, Src::Lit(0x3f800000)),
    (0x7cc, Src::Lit(0x3f800000)),
    (0x7b0, Src::Pool(-0x40a4)),
    (0x7b4, Src::Pool(-0x40a0)),
    (0x7c4, Src::Pool(-0x409c)),
    (0x840, Src::Pool(-0x413c)),
    (0x848, Src::Pool(-0x40a8)),
    (0x6f8, Src::Pool(-0x4134)),
    (0x6fc, Src::Pool(-0x4130)),
    (0x704, Src::Pool(-0x412c)),
    (0x70c, Src::Pool(-0x4154)),
    (0x714, Src::Lit(0x41d80000)),
    (0x718, Src::Lit(0x41a00000)),
    (0x7bc, Src::Pool(-0x4154)),
    (0x7c8, Src::Lit(0x41a00000)),
    (0x720, Src::Lit(0x0)),
    (0x724, Src::Lit(0x0)),
    (0x7d0, Src::Lit(0x0)),
    (0x7d4, Src::Lit(0x0)),
    (0x844, Src::Lit(0x0)),
    (0x84c, Src::Pool(-0x40c4)),
    (0x8f0, Src::Pool(-0x4070)),
    (0x910, Src::Pool(-0x406c)),
    (0x8f4, Src::Lit(0xbf800000)),
    (0x854, Src::Lit(0x3f000000)),
    (0x860, Src::Pool(-0x4140)),
    (0x90c, Src::Pool(-0x4068)),
    (0x914, Src::Pool(-0x4064)),
    (0x868, Src::Pool(-0x4144)),
    (0x870, Src::Lit(0x3f800000)),
    (0x87c, Src::Lit(0x3f800000)),
    (0x918, Src::Pool(-0x4144)),
    (0x920, Src::Lit(0x3f800000)),
    (0x92c, Src::Lit(0x3f800000)),
    (0x850, Src::Pool(-0x4138)),
    (0x858, Src::Pool(-0x4134)),
    (0x85c, Src::Pool(-0x4130)),
    (0x864, Src::Pool(-0x412c)),
    (0x8f8, Src::Pool(-0x4060)),
    (0x8fc, Src::Pool(-0x405c)),
    (0x900, Src::Pool(-0x4058)),
    (0x904, Src::Pool(-0x4054)),
    (0x908, Src::Pool(-0x4050)),
    (0x86c, Src::Pool(-0x4154)),
    (0x874, Src::Lit(0x41d80000)),
    (0x878, Src::Lit(0x41a00000)),
    (0x91c, Src::Pool(-0x4154)),
    (0x924, Src::Lit(0x41d80000)),
    (0x928, Src::Lit(0x41a00000)),
    (0x9a0, Src::Lit(0x44750000)),
    (0x880, Src::Lit(0x0)),
    (0x884, Src::Lit(0x0)),
    (0x930, Src::Lit(0x0)),
    (0x934, Src::Lit(0x0)),
    (0x9a4, Src::Lit(0x0)),
    (0x9ac, Src::Pool(-0x404c)),
    (0x9b4, Src::Lit(0x41200000)),
    (0x9bc, Src::Lit(0x41f00000)),
    (0x9c0, Src::Lit(0x42520000)),
    (0x9c4, Src::Pool(-0x4048)),
    (0xa50, Src::Lit(0x44750000)),
    (0xa5c, Src::Pool(-0x4044)),
    (0xa64, Src::Lit(0x41200000)),
    (0xa6c, Src::Lit(0x41f00000)),
    (0xa70, Src::Lit(0x42520000)),
    (0x9a8, Src::Lit(0x3f800000)),
    (0x9c8, Src::Pool(-0x4144)),
    (0x9d0, Src::Lit(0x3f800000)),
    (0x9dc, Src::Lit(0x3f800000)),
    (0xa58, Src::Lit(0x3f800000)),
    (0xa78, Src::Pool(-0x4144)),
    (0xa80, Src::Lit(0x3f800000)),
    (0xa8c, Src::Lit(0x3f800000)),
    (0x9b0, Src::Lit(0x41a00000)),
    (0x9b8, Src::Lit(0x41a00000)),
    (0x9cc, Src::Pool(-0x4154)),
    (0x9d4, Src::Lit(0x41d80000)),
    (0x9d8, Src::Lit(0x41a00000)),
    (0xa60, Src::Lit(0x41a00000)),
    (0xa68, Src::Lit(0x41a00000)),
    (0xa74, Src::Pool(-0x4040)),
    (0xa7c, Src::Pool(-0x4154)),
    (0xa84, Src::Lit(0x41d80000)),
    (0xa88, Src::Lit(0x41a00000)),
    (0x9e0, Src::Lit(0x0)),
    (0x9e4, Src::Lit(0x0)),
    (0xa54, Src::Lit(0x0)),
    (0xa90, Src::Lit(0x0)),
    (0xa94, Src::Lit(0x0)),
    (0xbb0, Src::Lit(0x44750000)),
    (0xbbc, Src::Pool(-0x403c)),
    (0xbc4, Src::Lit(0x41200000)),
    (0xbcc, Src::Lit(0x41f00000)),
    (0xbd0, Src::Lit(0x42520000)),
    (0xbd4, Src::Pool(-0x4038)),
    (0xb00, Src::Lit(0x44750000)),
    (0xb0c, Src::Pool(-0x4034)),
    (0xb14, Src::Lit(0x41200000)),
    (0xb1c, Src::Lit(0x41f00000)),
    (0xb20, Src::Lit(0x42520000)),
    (0xb08, Src::Lit(0x3f800000)),
    (0xb28, Src::Pool(-0x4144)),
    (0xb30, Src::Lit(0x3f800000)),
    (0xb3c, Src::Lit(0x3f800000)),
    (0xbb8, Src::Lit(0x3f800000)),
    (0xbd8, Src::Pool(-0x4144)),
    (0xbe0, Src::Lit(0x3f800000)),
    (0xbe4, Src::Lit(0x41d80000)),
    (0xbe8, Src::Lit(0x41a00000)),
    (0xb10, Src::Lit(0x41a00000)),
    (0xb18, Src::Lit(0x41a00000)),
    (0xb24, Src::Pool(-0x4030)),
    (0xb2c, Src::Pool(-0x4154)),
    (0xb34, Src::Lit(0x41d80000)),
    (0xb38, Src::Lit(0x41a00000)),
    (0xbc0, Src::Lit(0x41a00000)),
    (0xbc8, Src::Lit(0x41a00000)),
    (0xbdc, Src::Pool(-0x4154)),
    (0xb04, Src::Lit(0x0)),
    (0xb40, Src::Lit(0x0)),
    (0xb44, Src::Lit(0x0)),
    (0xbb4, Src::Lit(0x0)),
    (0xbec, Src::Lit(0x3f800000)),
    (0x5c, Src::Lit(0x40000000)),
    (0x68, Src::Pool(-0x402c)),
    (0x4c, Src::Lit(0x3f000000)),
    (0x48, Src::Lit(0x3f800000)),
    (0x58, Src::Lit(0x1)),
    (0x60, Src::Lit(0x1)),
    (0x54, Src::Lit(0x42340000)),
    (0xbf0, Src::Lit(0x0)),
    (0xbf4, Src::Lit(0x0)),
    (0x50, Src::Lit(0x0)),
    (0x64, Src::Lit(0x0)),
    (0x6c, Src::Lit(0x0)),
    (0x7c, Src::Pool(-0x4028)),
    (0x80, Src::Pool(-0x4024)),
    (0xfc, Src::Pool(-0x4020)),
    (0x10c, Src::Lit(0x40400000)),
    (0x124, Src::Pool(-0x401c)),
    (0x70, Src::Lit(0x3f800000)),
    (0x78, Src::Lit(0x1)),
    (0x88, Src::Lit(0x1)),
    (0xf8, Src::Lit(0x3f800000)),
    (0x100, Src::Lit(0x1)),
    (0x108, Src::Lit(0x1)),
    (0x110, Src::Lit(0x1)),
    (0x118, Src::Pool(-0x4018)),
    (0x120, Src::Pool(-0x4144)),
    (0x128, Src::Lit(0x1)),
    (0x74, Src::Pool(-0x4154)),
    (0x104, Src::Lit(0x42340000)),
    (0x84, Src::Lit(0x0)),
    (0x8c, Src::Lit(0x0)),
    (0x114, Src::Lit(0x0)),
    (0x11c, Src::Lit(0x0)),
    (0x12c, Src::Pool(-0x4014)),
    (0x1bc, Src::Lit(0x40800000)),
    (0x138, Src::Lit(0x1)),
    (0x1a8, Src::Lit(0x3f800000)),
    (0x1ac, Src::Lit(0x3f800000)),
    (0x1b0, Src::Lit(0x1)),
    (0x1b8, Src::Lit(0x1)),
    (0x1c0, Src::Lit(0x1)),
    (0x1c4, Src::Lit(0x1)),
    (0x1c8, Src::Pool(-0x4018)),
    (0x1d0, Src::Lit(0x3f800000)),
    (0x1d4, Src::Pool(-0x4010)),
    (0x1d8, Src::Lit(0x1)),
    (0x1dc, Src::Pool(-0x4144)),
    (0x1e0, Src::Pool(-0x4144)),
    (0x1e4, Src::Lit(0x1)),
    (0x1e8, Src::Lit(0x1)),
    (0x258, Src::Lit(0x3f800000)),
    (0x25c, Src::Pool(-0x400c)),
    (0x260, Src::Lit(0x1)),
    (0x268, Src::Lit(0x1)),
    (0x26c, Src::Lit(0x40a00000)),
    (0x270, Src::Lit(0x1)),
    (0x274, Src::Lit(0x1)),
    (0x278, Src::Pool(-0x4018)),
    (0x280, Src::Lit(0x3f800000)),
    (0x130, Src::Pool(-0x4008)),
    (0x1b4, Src::Lit(0x42340000)),
    (0x264, Src::Lit(0x42340000)),
    (0x134, Src::Lit(0x0)),
    (0x13c, Src::Lit(0x0)),
    (0x1cc, Src::Lit(0x0)),
    (0x1ec, Src::Lit(0x0)),
    (0x27c, Src::Lit(0x0)),
    (0x284, Src::Pool(-0x4010)),
    (0x288, Src::Lit(0x1)),
    (0x28c, Src::Pool(-0x4144)),
    (0x290, Src::Pool(-0x4144)),
    (0x294, Src::Lit(0x1)),
    (0x298, Src::Lit(0x1)),
    (0x330, Src::Lit(0x3f000000)),
    (0x33c, Src::Pool(-0x4144)),
    (0x340, Src::Pool(-0x4144)),
    (0x348, Src::Lit(0x1)),
    (0x3bc, Src::Lit(0x3f000000)),
    (0x314, Src::Lit(0x42340000)),
    (0x3c4, Src::Lit(0x42340000)),
    (0x29c, Src::Lit(0x0)),
    (0x308, Src::Lit(0x0)),
    (0x30c, Src::Lit(0x0)),
    (0x310, Src::Lit(0x0)),
    (0x318, Src::Lit(0x0)),
    (0x31c, Src::Lit(0x0)),
    (0x320, Src::Lit(0x0)),
    (0x324, Src::Lit(0x0)),
    (0x328, Src::Lit(0x0)),
    (0x32c, Src::Lit(0x0)),
    (0x334, Src::Lit(0x0)),
    (0x338, Src::Lit(0x0)),
    (0x344, Src::Lit(0x0)),
    (0x34c, Src::Lit(0x0)),
    (0x3b8, Src::Lit(0x0)),
    (0x3c0, Src::Lit(0x0)),
    (0x3c8, Src::Lit(0x0)),
    (0x3cc, Src::Lit(0x0)),
    (0x3d0, Src::Lit(0x0)),
    (0x3d4, Src::Lit(0x0)),
    (0x3d8, Src::Lit(0x0)),
    (0x3dc, Src::Lit(0x0)),
    (0x3e0, Src::Lit(0x3f800000)),
    (0x3e4, Src::Pool(-0x4010)),
    (0x3e8, Src::Lit(0x1)),
    (0x3ec, Src::Pool(-0x4144)),
    (0x3f0, Src::Pool(-0x4144)),
    (0x46c, Src::Pool(-0x4010)),
    (0x490, Src::Lit(0x3f800000)),
    (0x494, Src::Pool(-0x4010)),
    (0x498, Src::Lit(0x1)),
    (0x49c, Src::Pool(-0x4144)),
    (0x4a0, Src::Pool(-0x4144)),
    (0x51c, Src::Lit(0x3f800000)),
    (0x468, Src::Lit(0x3f400000)),
    (0x474, Src::Lit(0x42340000)),
    (0x524, Src::Lit(0x42340000)),
    (0x3f4, Src::Lit(0x0)),
    (0x3f8, Src::Lit(0x0)),
    (0x3fc, Src::Lit(0x0)),
    (0x470, Src::Lit(0x0)),
    (0x478, Src::Lit(0x0)),
    (0x47c, Src::Lit(0x0)),
    (0x480, Src::Lit(0x0)),
    (0x484, Src::Lit(0x0)),
    (0x488, Src::Lit(0x0)),
    (0x48c, Src::Lit(0x0)),
    (0x4a4, Src::Lit(0x0)),
    (0x4a8, Src::Lit(0x0)),
    (0x4ac, Src::Lit(0x0)),
    (0x518, Src::Lit(0x0)),
    (0x520, Src::Lit(0x0)),
    (0x528, Src::Lit(0x0)),
    (0x52c, Src::Lit(0x0)),
    (0x530, Src::Lit(0x0)),
    (0x534, Src::Lit(0x0)),
    (0x53c, Src::Pool(-0x4018)),
    (0x540, Src::Lit(0x3f800000)),
    (0x544, Src::Lit(0x3f800000)),
    (0x548, Src::Lit(0x1)),
    (0x54c, Src::Pool(-0x4144)),
    (0x550, Src::Pool(-0x4144)),
    (0x55c, Src::Lit(0x1)),
    (0xcac, Src::Lit(0x3f800000)),
    (0xccc, Src::Pool(-0x4018)),
    (0xcd0, Src::Lit(0x3f800000)),
    (0xcd4, Src::Lit(0x3f800000)),
    (0xcd8, Src::Lit(0x1)),
    (0xcdc, Src::Pool(-0x4144)),
    (0xce0, Src::Pool(-0x4144)),
    (0xcec, Src::Lit(0x1)),
    (0xcb4, Src::Lit(0x42340000)),
    (0x5d4, Src::Lit(0x42340000)),
    (0x538, Src::Lit(0x0)),
    (0x554, Src::Lit(0x0)),
    (0x558, Src::Lit(0x0)),
    (0xca8, Src::Lit(0x0)),
    (0xcb0, Src::Lit(0x0)),
    (0xcb8, Src::Lit(0x0)),
    (0xcbc, Src::Lit(0x0)),
    (0xcc0, Src::Lit(0x0)),
    (0xcc4, Src::Lit(0x0)),
    (0xcc8, Src::Lit(0x0)),
    (0xce4, Src::Lit(0x0)),
    (0xce8, Src::Lit(0x0)),
    (0x5c8, Src::Lit(0x0)),
    (0x5cc, Src::Lit(0x0)),
    (0x5d0, Src::Lit(0x0)),
    (0x5d8, Src::Lit(0x0)),
    (0x5dc, Src::Lit(0x0)),
    (0x5ec, Src::Pool(-0x4004)),
    (0x5f0, Src::Lit(0x3f800000)),
    (0x5f4, Src::Pool(-0x4010)),
    (0x5f8, Src::Lit(0x1)),
    (0x5fc, Src::Pool(-0x4144)),
    (0x600, Src::Pool(-0x4144)),
    (0x60c, Src::Lit(0x1)),
    (0x6a0, Src::Lit(0x3f800000)),
    (0x6a4, Src::Pool(-0x4010)),
    (0x6a8, Src::Lit(0x1)),
    (0x6ac, Src::Pool(-0x4144)),
    (0x6b0, Src::Pool(-0x4144)),
    (0x6b8, Src::Lit(0x1)),
    (0x684, Src::Lit(0x42340000)),
    (0x5e0, Src::Lit(0x0)),
    (0x5e4, Src::Lit(0x0)),
    (0x5e8, Src::Lit(0x0)),
    (0x604, Src::Lit(0x0)),
    (0x608, Src::Lit(0x0)),
    (0x678, Src::Lit(0x0)),
    (0x67c, Src::Lit(0x0)),
    (0x680, Src::Lit(0x0)),
    (0x688, Src::Lit(0x0)),
    (0x68c, Src::Lit(0x0)),
    (0x690, Src::Lit(0x0)),
    (0x694, Src::Lit(0x0)),
    (0x698, Src::Lit(0x0)),
    (0x69c, Src::Lit(0x0)),
    (0x6b4, Src::Lit(0x0)),
    (0x6bc, Src::Lit(0x0)),
    (0x728, Src::Lit(0x0)),
    (0x72c, Src::Lit(0x0)),
    (0x730, Src::Lit(0x0)),
    (0x734, Src::Lit(0x42340000)),
    (0x800, Src::Lit(0x3f000000)),
    (0x750, Src::Lit(0x3f800000)),
    (0x754, Src::Pool(-0x4010)),
    (0x758, Src::Lit(0x1)),
    (0x75c, Src::Pool(-0x4144)),
    (0x760, Src::Pool(-0x4144)),
    (0x76c, Src::Lit(0x1)),
    (0x80c, Src::Pool(-0x4144)),
    (0x810, Src::Pool(-0x4144)),
    (0x818, Src::Lit(0x1)),
    (0x7e4, Src::Lit(0x42340000)),
    (0x738, Src::Lit(0x0)),
    (0x73c, Src::Lit(0x0)),
    (0x740, Src::Lit(0x0)),
    (0x744, Src::Lit(0x0)),
    (0x748, Src::Lit(0x0)),
    (0x74c, Src::Lit(0x0)),
    (0x764, Src::Lit(0x0)),
    (0x768, Src::Lit(0x0)),
    (0x7d8, Src::Lit(0x0)),
    (0x7dc, Src::Lit(0x0)),
    (0x7e0, Src::Lit(0x0)),
    (0x7e8, Src::Lit(0x0)),
    (0x7ec, Src::Lit(0x0)),
    (0x7f0, Src::Lit(0x0)),
    (0x7f4, Src::Lit(0x0)),
    (0x7f8, Src::Lit(0x0)),
    (0x7fc, Src::Lit(0x0)),
    (0x804, Src::Lit(0x0)),
    (0x808, Src::Lit(0x0)),
    (0x814, Src::Lit(0x0)),
    (0x81c, Src::Lit(0x0)),
    (0x888, Src::Lit(0x0)),
    (0x88c, Src::Lit(0x0)),
    (0x93c, Src::Pool(-0x400c)),
    (0x94c, Src::Lit(0x40a00000)),
    (0x8b0, Src::Lit(0x3f800000)),
    (0x8b4, Src::Pool(-0x4010)),
    (0x8b8, Src::Lit(0x1)),
    (0x8bc, Src::Pool(-0x4144)),
    (0x8c0, Src::Pool(-0x4144)),
    (0x938, Src::Lit(0x3f800000)),
    (0x940, Src::Lit(0x1)),
    (0x948, Src::Lit(0x1)),
    (0x950, Src::Lit(0x1)),
    (0x954, Src::Lit(0x1)),
    (0x958, Src::Pool(-0x4018)),
    (0x960, Src::Lit(0x3f800000)),
    (0x964, Src::Pool(-0x4010)),
    (0x968, Src::Lit(0x1)),
    (0x96c, Src::Pool(-0x4144)),
    (0x970, Src::Pool(-0x4144)),
    (0x974, Src::Lit(0x1)),
    (0x978, Src::Lit(0x1)),
    (0x894, Src::Lit(0x42340000)),
    (0x944, Src::Lit(0x42340000)),
    (0x890, Src::Lit(0x0)),
    (0x898, Src::Lit(0x0)),
    (0x89c, Src::Lit(0x0)),
    (0x8a0, Src::Lit(0x0)),
    (0x8a4, Src::Lit(0x0)),
    (0x8a8, Src::Lit(0x0)),
    (0x8ac, Src::Lit(0x0)),
    (0x8c4, Src::Lit(0x0)),
    (0x8c8, Src::Lit(0x0)),
    (0x8cc, Src::Lit(0x0)),
    (0x95c, Src::Lit(0x0)),
    (0x97c, Src::Lit(0x0)),
    (0xa10, Src::Pool(-0x4010)),
    (0x9ec, Src::Lit(0x3f800000)),
    (0xa14, Src::Lit(0x3f800000)),
    (0xa1c, Src::Pool(-0x4144)),
    (0xa20, Src::Pool(-0x4144)),
    (0xa28, Src::Lit(0x1)),
    (0xa9c, Src::Lit(0x3f800000)),
    (0xac0, Src::Lit(0x3f800000)),
    (0xac4, Src::Lit(0x3f800000)),
    (0xac8, Src::Lit(0x1)),
    (0xacc, Src::Pool(-0x4144)),
    (0xad0, Src::Pool(-0x4144)),
    (0x9f4, Src::Lit(0x42340000)),
    (0xaa4, Src::Lit(0x42340000)),
    (0x9e8, Src::Lit(0x0)),
    (0x9f0, Src::Lit(0x0)),
    (0x9f8, Src::Lit(0x0)),
    (0x9fc, Src::Lit(0x0)),
    (0xa00, Src::Lit(0x0)),
    (0xa04, Src::Lit(0x0)),
    (0xa08, Src::Lit(0x0)),
    (0xa0c, Src::Lit(0x0)),
    (0xa18, Src::Lit(0x0)),
    (0xa24, Src::Lit(0x0)),
    (0xa2c, Src::Lit(0x0)),
    (0xa98, Src::Lit(0x0)),
    (0xaa0, Src::Lit(0x0)),
    (0xaa8, Src::Lit(0x0)),
    (0xaac, Src::Lit(0x0)),
    (0xab0, Src::Lit(0x0)),
    (0xab4, Src::Lit(0x0)),
    (0xab8, Src::Lit(0x0)),
    (0xabc, Src::Lit(0x0)),
    (0xad4, Src::Lit(0x0)),
    (0xb6c, Src::Pool(-0x4018)),
    (0xc24, Src::Lit(0x3f800000)),
    (0xb4c, Src::Lit(0x3f800000)),
    (0xb70, Src::Lit(0x3f800000)),
    (0xb74, Src::Lit(0x3f800000)),
    (0xb78, Src::Lit(0x1)),
    (0xb7c, Src::Pool(-0x4144)),
    (0xb80, Src::Pool(-0x4144)),
    (0xb8c, Src::Lit(0x1)),
    (0xbfc, Src::Lit(0x3f800000)),
    (0xc20, Src::Lit(0x3f800000)),
    (0xc28, Src::Lit(0x1)),
    (0xc04, Src::Lit(0x42340000)),
    (0xb54, Src::Lit(0x42340000)),
    (0xad8, Src::Lit(0x0)),
    (0xadc, Src::Lit(0x0)),
    (0xb48, Src::Lit(0x0)),
    (0xb50, Src::Lit(0x0)),
    (0xb58, Src::Lit(0x0)),
    (0xb5c, Src::Lit(0x0)),
    (0xb60, Src::Lit(0x0)),
    (0xb64, Src::Lit(0x0)),
    (0xb68, Src::Lit(0x0)),
    (0xb84, Src::Lit(0x0)),
    (0xb88, Src::Lit(0x0)),
    (0xbf8, Src::Lit(0x0)),
    (0xc00, Src::Lit(0x0)),
    (0xc08, Src::Lit(0x0)),
    (0xc0c, Src::Lit(0x0)),
    (0xc10, Src::Lit(0x0)),
    (0xc14, Src::Lit(0x0)),
    (0xc18, Src::Lit(0x0)),
    (0xc1c, Src::Lit(0x0)),
    (0xc2c, Src::Pool(-0x4144)),
    (0xc34, Src::Lit(0x0)),
    (0xc38, Src::Lit(0x0)),
    (0xc30, Src::Pool(-0x4144)),
    (0xc3c, Src::Lit(0x1)),
];

/// The four curve points (x, y) every row starts with.
pub(crate) const SPIC_CURVE: [(Src, Src); 4] = [
    (Src::Lit(0x0), Src::Pool(-0x4164)),
    (Src::Lit(0x42200000), Src::Pool(-0x4160)),
    (Src::Lit(0x42a00000), Src::Pool(-0x415c)),
    (Src::Lit(0x42f00000), Src::Pool(-0x4158)),
];

/// Registers at the end of a loop iteration (`row` = the row just filled, `next` = the next index).
pub(crate) fn spic_loop_regs(c: &mut Cpu, gp: u32, s0: u32, row: u32, next: u32) {
    c.g[AT][0] = 0x4252_0000;
    c.g[T7][0] = next as u64;
    c.g[T5][0] = (next < 19) as u64;
    c.g[T6][0] = 1;
    c.g[T8][0] = sx(s0.wrapping_add(0x6c));
    c.g[V0][0] = 0xb0;
    for r in [V1, A0, A1, A2, A3, T0, T1, T2, T3, T4] {
        c.g[r][0] = sx(row);
    }
    c.f[0] = 0x3f800000;
    c.lwc1(1, gp.wrapping_sub(0x4168));
    c.f[2] = 0x41a00000;
    c.lwc1(3, gp.wrapping_sub(0x4158));
    c.f[4] = 0x42f00000;
    c.lwc1(5, gp.wrapping_sub(0x415c));
    c.f[6] = 0x42a00000;
    c.lwc1(7, gp.wrapping_sub(0x4160));
    c.f[8] = 0x42200000;
    c.lwc1(9, gp.wrapping_sub(0x4164));
    c.lwc1(10, gp.wrapping_sub(0x414c));
    c.f[29] = 0x42340000;
    c.f[30] = 0x41d80000;
    c.lwc1(31, gp.wrapping_sub(0x4154));
}

/// `u16 *UStr_Find(haystack, needle)` (0x3dbd18): the first position in
/// `haystack` where `needle` occurs, in UTF-16 units from `haystack`, or
/// -1 when `needle` is empty or absent. Compares `strlen(needle)` units at
/// each haystack position starting with the haystack's own first unit.
pub fn u_str_find(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_bd58;
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[A0][0] = c.g[S2][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_bd48);
        c.g[S3][0] = 0;
        callee::U_STR_LEN.call(c, 0x003d_bd48)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[S0][0];
        let mut found = false;
        if !empty {
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003d_bd68);
                c.g[A2][0] = c.g[S4][0];
                callee::U_STR_N_CMP.call(c, 0x003d_bd68)?;
                let mismatch = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                if !mismatch {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S3][0];
                    found = true;
                    break;
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.lhu(c.g[S0][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0xffff;
        }
        c.cycles += 8;
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

/// `int UStr_Cmp(a, b)` (0x3dbdb0): compare two UTF-16 strings with no
/// effective length limit (0x7fffffff units).
pub fn u_str_cmp(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x7fff0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A2][0] | 0xffff;
        c.g[RA][0] = sx(0x003d_bdc4);
        callee::U_STR_N_CMP.call(c, 0x003d_bdc4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UStr_NCmp(a, b, max)` (0x3dbdd0): compare at most `max` UTF-16
/// units. 0 when they match (or `max` runs out); otherwise the first
/// difference, `a`-unit minus `b`-unit.
///
/// Dispatcher-shaped in the original (`0x3dbde8` is the loop head): it can
/// be re-entered there after an event unwind, so the port keeps the
/// dispatch and its state in the registers.
pub fn u_str_n_cmp(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003d_bdd0 => {
                c.cycles += 3;
                c.g[T0][0] = 0;
                let done = (c.g[A2][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if done { 0x003d_be14 } else { 0x003d_bddc };
            }
            0x003d_bddc => {
                c.cycles += 2;
                c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
                pc = 0x003d_be08;
            }
            0x003d_bde4 => {
                c.cycles += 1;
                pc = 0x003d_bde8;
            }
            0x003d_bde8 => {
                c.cycles += 5;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A2][0] as i64)) as u64;
                let done = c.g[V0][0] == 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                pc = if done { 0x003d_be14 } else { 0x003d_bdfc };
            }
            0x003d_bdfc => {
                c.cycles += 2;
                let done = c.g[A3][0] != 0;
                pc = if done { 0x003d_be14 } else { 0x003d_be04 };
            }
            0x003d_be04 => {
                c.cycles += 1;
                c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
                pc = 0x003d_be08;
            }
            0x003d_be08 => {
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                let more = c.g[V1][0] != 0;
                c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if more {
                    if event_due(c) && back_edge_unwinds(c, 0x003d_bde8) {
                        return;
                    }
                    pc = 0x003d_bde8;
                } else {
                    pc = 0x003d_be14;
                }
            }
            0x003d_be14 => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[A3][0];
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003d_bdd0, 0x003d_be1c);
                return;
            }
        }
    }
}

/// `u16 *UStr_Cpy(dst, src)` (0x3dbe20): copy a UTF-16 string with no
/// effective length limit (0x7fffffff units). Returns the end of the copy.
pub fn u_str_cpy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x7fff0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A2][0] | 0xffff;
        c.g[RA][0] = sx(0x003d_be34);
        callee::U_STR_N_CPY.call(c, 0x003d_be34)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_NCpy(dst, src, max)` (0x3dbe40): copy at most `max` UTF-16
/// units, stopping at the source terminator, and NUL-terminate the copy.
/// Returns the end of the copied units (`dst + 2 * units`).
pub fn u_str_n_cpy(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_be58;
    c.cycles += 2;
    let done = (c.g[A2][0] as i64) <= 0;
    c.g[A3][0] = 0;
    if !done {
        c.cycles += 3;
        c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[T0][0] = c.g[A0][0];
        if !empty {
            c.cycles += 1;
            c.g[V1][0] = c.lhu(c.g[A1][0] as u32);
            loop {
                c.cycles += 6;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.sh(c.g[T0][0] as u32, c.g[V1][0]);
                let full = c.g[V0][0] == 0;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2));
                if full {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lhu(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                    continue;
                } else {
                    break;
                }
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[V0][0] as u32, 0);
    c.pc = ra;
}

/// `u16 *UStr_FromAscii(dst, src)` (0x3dbe90): expand an ASCII (byte)
/// string to UTF-16, terminator included. Returns the end of the copy and
/// leaves the unit count in a2.
pub fn u_str_from_ascii(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_bea0;
    c.cycles += 3;
    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
    let empty = c.g[V0][0] == 0;
    c.g[A2][0] = 0;
    if !empty {
        c.cycles += 1;
        c.g[V1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sh(c.g[V1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[V0][0] as u32, 0);
    c.pc = ra;
}

/// `u16 *UStr_SubstituteArgs_Wrapper(dst, fmt, arg1, arg2)` (0x3dbed0):
/// substitute `%1`/`%2` in `fmt` with `arg1`/`arg2` (see
/// [`u_str_substitute_args`]), with the fourth slot cleared.
pub fn u_str_substitute_args_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x003d_bee0);
        callee::U_STR_SUBSTITUTE_ARGS.call(c, 0x003d_bee0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_SubstituteArgs(dst, fmt, arg1, arg2)` (0x3dbef0): copy the
/// UTF-16 `fmt` to `dst`, replacing `%1` with `arg1` and `%2` with `arg2`.
/// Any other `%x` pair copies the `%` and re-reads `x` next round.
/// Returns 0 (the terminating unit it stopped on).
pub fn u_str_substitute_args(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_bf38;
        c.cycles += 15;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S3][0] = c.g[A2][0];
        if !empty {
            c.cycles += 3;
            c.g[S6][0] = 0x25;
            c.g[S5][0] = 0x31;
            c.g[S4][0] = 0x32;
            loop {
                'b_3dbf9c: {
                    'b_3dbf90: {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[S6][0] {
                            c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                            break 'b_3dbf90;
                        }
                        'b_3dbf7c: {
                            'b_3dbf60: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                                let not_one = c.g[V0][0] != c.g[S5][0];
                                c.g[A0][0] = c.g[S0][0];
                                if not_one {
                                    break 'b_3dbf60;
                                }
                                c.cycles += 5;
                                c.g[RA][0] = sx(0x003d_bf54);
                                c.g[A1][0] = c.g[S3][0];
                                callee::U_STR_CPY.call(c, 0x003d_bf54)?;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S3][0];
                                break 'b_3dbf7c;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[S4][0] {
                                c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                                break 'b_3dbf90;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x003d_bf74);
                            c.g[A1][0] = c.g[S2][0];
                            callee::U_STR_CPY.call(c, 0x003d_bf74)?;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[S2][0];
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x003d_bf84);
                        callee::U_STR_LEN.call(c, 0x003d_bf84)?;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        break 'b_3dbf9c;
                    }
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                    c.sh(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                    break 'b_3dbf9c;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                let more = c.g[V0][0] != 0;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 11;
        c.sh(c.g[S0][0] as u32, 0);
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

/// `u16 *UStr_FormatUInt(dst, value, base, digits)` (0x3dbfd8): write
/// `value` in `base` at `dst`, going backwards, using the `digits` table at
/// a3 (indexed by twice the remainder). A zero `value` writes one digit.
/// Returns the start of the written units.
pub fn u_str_format_u_int(c: &mut Cpu) {
    c.cycles += 2;
    let nonzero = c.g[A1][0] != 0;
    c.g[T0][0] = 0;
    if !nonzero {
        c.cycles += 4;
        c.g[V0][0] = c.lhu(c.g[A3][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2));
        c.sh(c.g[A0][0] as u32, c.g[V0][0]);
    } else {
        const HEAD: u32 = 0x003d_bff8;
        c.cycles += 2;
        c.divu(0, 5, 6);
        loop {
            c.cycles += 2;
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2));
            c.g[V0][0] = c.hi[0];
            c.g[V1][0] = c.lo[0];
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
            c.sh(c.g[A0][0] as u32, c.g[V1][0]);
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                break;
            }
            c.divu(0, 5, 6);
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `u16 *UStr_FormatInt(dst, value, base, digits)` (0x3dc038): format the
/// absolute value (see [`u_str_format_u_int`]); the caller writes any sign.
pub fn u_str_format_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V1][0] = c.g[A1][0];
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_c058);
        if c.g[V0][0] == 0 {
            c.g[A1][0] = c.g[V1][0];
        }
        callee::U_STR_FORMAT_U_INT.call(c, 0x003d_c058)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_FormatFloat(dst, bits, precision)` (0x3dc068): convert the
/// float in `bits` with `fcvt`, then copy its decimal digits backwards to
/// `dst`, inserting `.` after `precision` digits and padding the integer
/// part with `0`. Returns the start of the written units (`dst` unchanged
/// when the value needs no digits).
pub fn u_str_format_float(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_c0d8;
        c.cycles += 25;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[A3][0] = c.g[SP][0] | 0x4;
        frame.save(c, S2, 0x40);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[A0][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S3, 0x30);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S4, 0x20);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003d_c0a8);
        c.g[S4][0] = 0;
        c.pc = 0x0041_8d48;
        call_ptr(c, callee::FCVT, 0x003d_c0a8)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003d_c0b4);
        c.g[A0][0] = c.g[S3][0];
        c.pc = 0x0041_6810;
        call_ptr(c, callee::STRLEN, 0x003d_c0b4)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.g[V0][0];
            c.g[T0][0] = 0x2e;
            c.g[A3][0] = 0x30;
            loop {
                c.cycles += 2;
                let at_end = c.g[S1][0] != c.g[S2][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[S4][0] as u32));
                if !at_end {
                    // Dot: leaves the loop directly, without the
                    // digit counter step below.
                    c.cycles += 3;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                    c.sh(c.g[S0][0] as u32, c.g[T0][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let no_more = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    if no_more {
                        c.cycles += 5;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        c.sh(c.g[S0][0] as u32, c.g[V0][0]);
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x2));
                        c.sh(c.g[S0][0] as u32, c.g[A3][0]);
                    }
                    c.cycles += 1;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                }
                c.cycles += 3;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                let more = c.g[S1][0] != c.g[A1][0];
                c.g[V0][0] = c.g[S0][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `UStr_FormatNumberField(u16 **cursor, value, base, flags, width, prec,
/// digits)` (0x3dc148): format one `printf`-style field at `*cursor` and
/// advance the cursor past it. Flags bit 0x40 selects
/// floating point (`fcvt` via [`u_str_format_float`], 0x80/0x100 pick the
/// call), otherwise 0x20 selects unsigned ([`u_str_format_u_int`] after
/// `__fixunsdfsi`) or signed ([`u_str_format_int`] after `__fixdfsi`).
/// Then zero padding, the sign/`0x`/space prefix, width padding and the
/// digits are written. Returns the flags word.
pub fn u_str_format_number_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x1a0);
        frame.save(c, S0, 0x190);
        frame.save(c, S1, 0x180);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x170);
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xfe));
        frame.save(c, S4, 0x150);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S5, 0x140);
        c.g[S4][0] = c.g[T2][0];
        frame.save(c, S6, 0x130);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, FP, 0x110);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S7, 0x120);
        c.g[FP][0] = c.g[A0][0];
        frame.save_ra(c, 0x100);
        c.g[A2][0] = c.g[T1][0];
        frame.save(c, S3, 0x160);
        c.g[V0][0] = c.g[S0][0] & 0x40;
        c.g[S3][0] = c.lw(c.g[FP][0] as u32);
        let int_path = c.g[V0][0] == 0;
        c.sh((c.g[SP][0] as u32).wrapping_add(0xfe), 0);
        if int_path {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x20;
            let signed = c.g[V0][0] == 0;
            if !signed {
                c.cycles += 9;
                c.g[RA][0] = sx(0x003d_c1fc);
                c.g[A0][0] = c.g[S2][0];
                c.pc = 0x0041_26a0;
                call_ptr(c, callee::FIXUNSDFSI, 0x003d_c1fc)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x003d_c210);
                c.g[A2][0] = c.g[S6][0];
                callee::U_STR_FORMAT_U_INT.call(c, 0x003d_c210)?;
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003d_c220);
                c.g[A0][0] = c.g[S2][0];
                c.pc = 0x0041_2608;
                call_ptr(c, callee::FIXDFSI, 0x003d_c220)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x003d_c234);
                c.g[A2][0] = c.g[S6][0];
                callee::U_STR_FORMAT_INT.call(c, 0x003d_c234)?;
                c.g[S1][0] = c.g[V0][0];
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x80;
            let no_float_first = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[S0][0] & 0x100;
            if !no_float_first {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_c1b8);
                c.g[A0][0] = c.g[S1][0];
                callee::U_STR_FORMAT_FLOAT.call(c, 0x003d_c1b8)?;
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                let no_float_second = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !no_float_second {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_c1d0);
                    c.g[A1][0] = c.g[S2][0];
                    callee::U_STR_FORMAT_FLOAT.call(c, 0x003d_c1d0)?;
                    c.g[S1][0] = c.g[V0][0];
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_c1e0);
                    c.g[A1][0] = c.g[S2][0];
                    callee::U_STR_FORMAT_FLOAT.call(c, 0x003d_c1e0)?;
                    c.g[S1][0] = c.g[V0][0];
                }
            }
        }
        // Zero padding to the width.
        c.cycles += 4;
        c.g[V1][0] = c.g[S0][0] & 0x18;
        c.g[V0][0] = 0x8;
        let no_zero_pad = c.g[V1][0] != c.g[V0][0];
        c.g[S7][0] = c.g[S0][0] & 0x10;
        if !no_zero_pad {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_c254);
            c.g[S4][0] = 0x30;
            callee::U_STR_LEN.call(c, 0x003d_c254)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S5][0] as i64)) as u64;
            let fits = c.g[V0][0] == 0;
            c.g[V0][0] = 0xa;
            if !fits {
                const HEAD: u32 = 0x003d_c268;
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                loop {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V1][0] != 0;
                    c.sh(c.g[S1][0] as u32, c.g[S4][0]);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0xa;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0xa;
        }
        // Sign / prefix.
        c.cycles += 2;
        let no_prefix = c.g[S6][0] != c.g[V0][0];
        c.g[V0][0] = 0x8;
        // Only the full chain below takes the block tail (cycles + 1,
        // v0 = 8); the skips jump straight to the `0` flag.
        let mut prefix_tail = !no_prefix;
        if !no_prefix {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x20;
            let unsigned = c.g[V0][0] != 0;
            c.g[V0][0] = 0x8;
            if !unsigned {
                c.cycles += 6;
                c.g[S4][0] = 0;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003d_c2ac);
                c.g[A1][0] = c.g[S4][0];
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c2ac)?;
                let non_negative = (c.g[V0][0] as i64) >= 0;
                c.g[A1][0] = c.g[S4][0];
                if !non_negative {
                    c.cycles += 3;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = 0x2d;
                    c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_c2c8);
                c.g[A0][0] = c.g[S2][0];
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c2c8)?;
                let positive = (c.g[V0][0] as i64) > 0;
                c.g[V0][0] = c.g[S0][0] & 0x1;
                if positive {
                    c.cycles += 2;
                    let no_plus = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x2b;
                    if !no_plus {
                        c.cycles += 2;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                        c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003d_c2ec);
                c.g[A1][0] = 0;
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c2ec)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !nonzero {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[S0][0] & 0x1;
                    let no_space = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x20;
                    if !no_space {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                        c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_c314);
                c.g[A1][0] = 0;
                c.pc = 0x0041_2500;
                call_ptr(c, callee::CMPDF2, 0x003d_c314)?;
                let negative = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = c.g[S0][0] & 0x5;
                if !negative {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let no_alt = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8;
                    if !no_alt {
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                        c.g[V0][0] = 0x20;
                        c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                    } else {
                        prefix_tail = false;
                    }
                }
            } else {
                prefix_tail = false;
            }
        }
        if prefix_tail {
            c.cycles += 1;
            c.g[V0][0] = 0x8;
        }
        // `0` flag without `0x` prefix: one `0`.
        c.cycles += 2;
        let no_zero = c.g[S6][0] != c.g[V0][0];
        c.g[V0][0] = 0x10;
        if !no_zero {
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x2;
            let no_zero_flag = c.g[V0][0] == 0;
            c.g[V0][0] = 0x30;
            if !no_zero_flag {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                c.sh(c.g[S1][0] as u32, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 0x10;
        }
        // `0x` prefix.
        c.cycles += 2;
        let no_hex = c.g[S6][0] != c.g[V0][0];
        c.g[V0][0] = c.g[S0][0] & 0x2;
        if !no_hex {
            c.cycles += 2;
            let no_hex_flag = c.g[V0][0] == 0;
            c.g[V0][0] = 0x78;
            if !no_hex_flag {
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                c.sh(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = 0x30;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                c.sh(c.g[S1][0] as u32, c.g[V1][0]);
            }
        }
        // Space padding to the width, then copy the digits in.
        c.cycles += 2;
        let no_space_pad = c.g[S7][0] != 0;
        c.g[V1][0] = c.g[S0][0] & 0x8;
        if !no_space_pad {
            c.cycles += 9;
            c.g[V0][0] = 0x20;
            c.g[S0][0] = 0x30;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_c398);
            if c.g[V1][0] == 0 {
                c.g[S0][0] = c.g[V0][0];
            }
            callee::U_STR_LEN.call(c, 0x003d_c398)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S5][0] as i64)) as u64;
            let fits = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if !fits {
                const HEAD: u32 = 0x003d_c3a8;
                loop {
                    c.cycles += 7;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let more = c.g[V1][0] != 0;
                    c.sh(c.g[S1][0] as u32, c.g[S0][0]);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x003d_c3cc);
        c.g[A0][0] = c.g[S1][0];
        callee::U_STR_LEN.call(c, 0x003d_c3cc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x003d_c3dc);
        c.g[A1][0] = c.g[S1][0];
        callee::U_STR_CPY.call(c, 0x003d_c3dc)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32) << 1);
        let pad_rest = c.g[S7][0] == 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !pad_rest {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S5][0] as i64)) as u64;
            let fits = c.g[V0][0] == 0;
            c.g[V1][0] = 0x20;
            if !fits {
                const HEAD: u32 = 0x003d_c3f8;
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                loop {
                    c.cycles += 7;
                    c.sh(c.g[S3][0] as u32, c.g[V1][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    let more = c.g[S0][0] != 0;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2));
                    if !more {
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S5][0];
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
        }
        c.cycles += 14;
        c.sw(c.g[FP][0] as u32, c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x190);
        frame.restore(c, S1, 0x180);
        frame.restore(c, S2, 0x170);
        frame.restore(c, S3, 0x160);
        frame.restore(c, S4, 0x150);
        frame.restore(c, S5, 0x140);
        frame.restore(c, S6, 0x130);
        frame.restore(c, S7, 0x120);
        frame.restore(c, FP, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_sprintf(dst, fmt, ...)` (0x3dc9f8): format into `dst` (see
/// `UStr_vsprintf`), spilling the register and FPU varargs onto the stack
/// for it. Returns `dst`.
pub fn u_str_sprintf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
        frame.save_ra(c, 0);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
        frame.save_fpr(c, 12, 0x40);
        frame.save_fpr(c, 14, 0x44);
        frame.save_fpr(c, 16, 0x48);
        c.g[RA][0] = sx(0x003d_ca30);
        frame.save_fpr(c, 18, 0x4c);
        callee::U_STR_VSPRINTF.call(c, 0x003d_ca30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u16 *UStr_FromEscapedAscii(dst, src)` (0x3dca40): expand ASCII `src`
/// to UTF-16, turning `\x` + up to 4 lowercase hex digits into the byte
/// they encode (a NUL or a non-hex digit ends the digits early; a lone
/// `\`, `\x` NUL or invalid digit copies the `\` and moves one byte).
/// Returns the end of the copy.
pub fn u_str_from_escaped_ascii(c: &mut Cpu) {
    const HEAD: u32 = 0x003d_ca58;
    const INNER: u32 = 0x003d_ca98;
    c.cycles += 3;
    c.g[V0][0] = c.lb(c.g[A1][0] as u32);
    let empty = c.g[V0][0] == 0;
    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
    if !empty {
        c.cycles += 3;
        c.g[T4][0] = 0x5c;
        c.g[T3][0] = 0x78;
        loop {
            c.cycles += 6;
            c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
            c.g[T1][0] = 0;
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[V1][0] = 0;
            let not_escape = c.g[V0][0] != c.g[T4][0];
            c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            if !not_escape {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x1));
                let not_x = c.g[V0][0] != c.g[T3][0];
                if !not_x {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x2));
                    let nul_after_x = c.g[V0][0] == 0;
                    c.g[T1][0] = 0x1;
                    if !nul_after_x {
                        c.cycles += 4;
                        c.g[A3][0] = c.g[A1][0];
                        c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A2][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                        loop {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                            let not_digit = c.g[V0][0] == 0;
                            c.g[V0][0] = c.g[A2][0] & 0xff;
                            if not_digit {
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x61));
                                c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                                if c.g[V0][0] == 0 {
                                    c.g[T1][0] = 0;
                                    break;
                                }
                                c.cycles += 2;
                                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x57));
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                            }
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                break;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x2));
                            if c.g[V0][0] != 0 {
                                c.g[A2][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x2));
                                if event_due(c) {
                                    deliver_event_here(c, INNER);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            if c.g[T1][0] == 0 {
                c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                c.cycles += 2;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.sh(c.g[A0][0] as u32, c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.sh(c.g[A0][0] as u32, c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6));
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[T2][0];
            c.g[V0][0] = c.lb(c.g[A1][0] as u32);
            let more = c.g[V0][0] != 0;
            c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.sh(c.g[A0][0] as u32, 0);
    c.pc = ra;
}

/// `char *UStr_ToEscapedAscii(dst, src)` (0x3dcb20): copy UTF-16 `src` to
/// ASCII `dst`, formatting every unit at or above 0x100 and every `\x`
/// pair with `sprintf`. Returns the end of the copy.
pub fn u_str_to_escaped_ascii(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_cb60;
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if !empty {
            c.cycles += 5;
            c.g[S4][0] = 0x5c;
            c.g[S3][0] = 0x78;
            c.g[S2][0] = 0x490000;
            c.g[V1][0] = c.g[V0][0] & 0xffff;
            loop {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x100) as u64;
                let wide = c.g[V0][0] == 0;
                let mut use_sprintf = wide;
                if !wide {
                    c.cycles += 2;
                    if c.g[V1][0] != c.g[S4][0] {
                        c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                        if c.g[V0][0] != c.g[S3][0] {
                            c.g[V0][0] = c.lbu(c.g[S1][0] as u32);
                        } else {
                            c.cycles += 1;
                            use_sprintf = true;
                        }
                    }
                }
                if use_sprintf {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.lhu(c.g[S1][0] as u32);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5d18));
                    c.g[RA][0] = sx(0x003d_cb94);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6));
                    call_ptr(c, callee::SPRINTF, 0x003d_cb94)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                } else {
                    c.cycles += 3;
                    c.sb(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2));
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[V0][0] & 0xffff;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 9;
        c.sb(c.g[S0][0] as u32, 0);
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

/// `GameLink_GetTicks` (0x2c28a0): the EA timer's tick count.
pub fn game_link_get_ticks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_4e98, 0x002c_28b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `GameLink_Alloc(size, zeroed)` (0x2c28c0, the reference's
/// `func_002C28C0`): `operator new` of `size` bytes named "GAMELINK" with the
/// zero-fill flag `0x100` (0 when `zeroed` is 0).
pub fn game_link_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = 0x100;
        let frame = Frame::open(c, 0x10);
        if c.g[A2][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6030));
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002c_28e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// Direct-call targets (the generated function, through its override slot when
// it has one).
pub(crate) const GL_ALLOC: Callee = Callee {
    addr: 0x002c_28c0,
    f: ee::s08::f_002c28c0,
};

pub(crate) const GL_FREE: Callee = Callee {
    addr: 0x002c_28f8,
    f: ee::s08::f_002c28f8,
};

/// `void GameLink_Free(void* block)` (0x2c28f8): `MEM_Free(block) != 0`.
pub fn game_link_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_23a8, 0x002c_2908)?;
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* GameLink_MemCopy(void* dst, ...)` (0x2c2918): `MEM_copy(...)`, then
/// returns its first argument.
pub fn game_link_mem_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_6574, 0x002c_292c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void* GameLink_Realloc(void* block, void* name, int size, int zeroFlag)`
/// (0x2c2940): no block allocates; a zero size frees and returns 0; otherwise
/// the block is resized under the memory-manager mutex.
pub fn game_link_realloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        let block_null = c.g[S1][0] == 0;
        c.g[A2][0] = c.g[A3][0];
        if block_null {
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            GL_ALLOC.call(c, 0x002c_2970)?;
            frame.restore(c, 16, 0x30);
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0x0054_0000;
            if c.g[S2][0] == 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                GL_FREE.call(c, 0x002c_29b4)?;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 11;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7500));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003e_5700, 0x002c_298c)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0031_9bd8, 0x002c_2998)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003e_5760, 0x002c_29a4)?;
                c.g[V0][0] = c.g[S1][0];
            }
            c.cycles += 1;
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

/// `void* GameLink_CreateThread(entry, arg, stackSize)` (0x2c29d0): a thread
/// block (`[0]` the stack, `+4` the THREAD_createparam block), returned when
/// both allocations and the thread creation succeed, else freed and 0.
pub fn game_link_create_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, 16, 0x40);
        c.g[A0][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xc;
        c.g[A2][0] = 0;
        GL_ALLOC.call(c, 0x002c_2a04)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0048_0000;
        Addr(r32(c, S0)).sw(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6088));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        GL_ALLOC.call(c, 0x002c_2a20)?;
        let stack_null = c.g[V0][0] == 0;
        let stack = c.g[V0][0];
        Addr(r32(c, S0)).sw(c, stack);
        if stack_null {
            // break to the shared tail below
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        } else {
            c.cycles += 9;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[T0][0] = c.g[S1][0];
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x4));
            c.g[T1][0] = 0x63;
            call_ptr(c, 0x003e_50c8, 0x002c_2a44)?;
            if c.g[V0][0] != 0 {
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            } else {
                c.cycles += 3;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                GL_FREE.call(c, 0x002c_2a54)?;
                Addr(r32(c, S0)).sw(c, 0);
                c.cycles += 1;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            }
        }
        // Shared tail: the thread is valid when its stack pointer is set.
        c.cycles += 2;
        let valid = c.g[A0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if !valid {
            c.cycles += 2;
            let thread_null = c.g[S0][0] == 0;
            c.g[V0][0] = 0;
            if !thread_null {
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 2;
                    GL_FREE.call(c, 0x002c_2a7c)?;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                GL_FREE.call(c, 0x002c_2a84)?;
                c.g[V0][0] = 0;
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

/// `void GameLink_DestroyThread(void** self)` (0x2c2aa8): frees the stack
/// (`*self`) and then the thread block itself, clearing both pointers.
pub fn game_link_destroy_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            GL_FREE.call(c, 0x002c_2acc)?;
            Addr(r32(c, S0)).sw(c, 0);
        }
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                GL_FREE.call(c, 0x002c_2aec)?;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            GL_FREE.call(c, 0x002c_2af4)?;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameLink_Yield()` (0x2c2b08): `THREAD_yield()`.
pub fn game_link_yield(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003e_5398), 0x002c_2b18)
}

/// `void* GameLink_CreateMutex()` (0x2c2b28): a 0x10-byte block, initialised
/// as a mutex, returned.
pub fn game_link_create_mutex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[A0][0] = 0x004a_0000;
        c.g[A1][0] = 0x10;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x3898));
        GL_ALLOC.call(c, 0x002c_2b48)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5698, 0x002c_2b54)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameLink_DestroyMutex(void* mutex)` (0x2c2b68): destroys the mutex,
/// then frees its block.
pub fn game_link_destroy_mutex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_56e0, 0x002c_2b7c)?;
        c.g[A0][0] = c.g[S0][0];
        GL_FREE.call(c, 0x002c_2b84)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameLink_LockMutex(void* mutex)` (0x2c2b98): `MUTEX_lock(mutex)`.
pub fn game_link_lock_mutex(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003e_5700), 0x002c_2ba8)
}

/// `void GameLink_UnlockMutex(void* mutex)` (0x2c2bb8): `MUTEX_unlock(mutex)`.
pub fn game_link_unlock_mutex(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x003e_5760), 0x002c_2bc8)
}

/// `void GameLink_SetupPlatformFuncs(GameLinkFuncs* t)` (0x2c2bd8): fills the
/// platform function table at `t` (allocator, thread, mutex and string
/// entries). Leaf: no calls.
pub fn game_link_setup_platform_funcs(c: &mut Cpu) {
    c.cycles += 85;
    let frame = Frame::open(c, 0x80);
    frame.save(c, 16, 0x70);
    frame.save(c, 17, 0x60);
    frame.save(c, 18, 0x50);
    frame.save(c, 19, 0x40);
    frame.save(c, 20, 0x30);
    frame.save(c, 21, 0x20);
    frame.save(c, 22, 0x10);
    frame.save(c, 23, 0x00);
    let t = r32(c, A0);
    Addr(t.wrapping_add(0xc)).sw(c, 0x0041_0000 + 0x65a8);
    Addr(t.wrapping_add(0x8)).sw(c, 0x0041_0000 + 0x66f4);
    Addr(t.wrapping_add(0x4)).sw(c, 0x0041_0000 + 0x6810);
    Addr(t.wrapping_add(0x20)).sw(c, 0x002c_0000 + 0x29d0);
    Addr(t.wrapping_add(0x24)).sw(c, 0x002c_0000 + 0x2aa8);
    Addr(t.wrapping_add(0x28)).sw(c, 0x002c_0000 + 0x2b08);
    Addr(t.wrapping_add(0x2c)).sw(c, 0x002c_0000 + 0x2b28);
    Addr(t.wrapping_add(0x30)).sw(c, 0x002c_0000 + 0x2b68);
    c.g[V1][0] = 0x002c_0000 + 0x28c0;
    c.g[A1][0] = 0x002c_0000 + 0x2940;
    c.g[A2][0] = 0x002c_0000 + 0x28f8;
    c.g[A3][0] = 0x0041_0000 + 0x6210;
    c.g[T0][0] = 0x0041_0000 + 0x610c;
    c.g[T1][0] = 0x0041_0000 + 0x5fc8;
    c.g[T2][0] = 0x0041_0000 + 0x605c;
    c.g[T3][0] = 0x002c_0000 + 0x28a0;
    c.g[T4][0] = 0x0042_0000 - 0x7860;
    c.g[T5][0] = 0x0042_0000 - 0x7840;
    c.g[T6][0] = 0x0042_0000 - 0x7818;
    c.g[T7][0] = 0x0042_0000 - 0x7108;
    c.g[T8][0] = 0x002c_0000 + 0x2b98;
    c.g[V0][0] = 0x002c_0000 + 0x2bb8;
    frame.restore(c, 16, 0x70);
    frame.restore(c, 17, 0x60);
    frame.restore(c, 18, 0x50);
    frame.restore(c, 19, 0x40);
    frame.restore(c, 20, 0x30);
    frame.restore(c, 21, 0x20);
    frame.restore(c, 22, 0x10);
    frame.restore(c, 23, 0x00);
    let (v0, v1, a1, a2, a3) = (c.g[V0][0], c.g[V1][0], c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    let (t0, t1, t2, t3) = (c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]);
    let (t4, t5, t6, t7, t8) = (c.g[T4][0], c.g[T5][0], c.g[T6][0], c.g[T7][0], c.g[T8][0]);
    Addr(t.wrapping_add(0x3c)).sw(c, v1);
    Addr(t.wrapping_add(0x38)).sw(c, v0);
    Addr(t.wrapping_add(0x40)).sw(c, a1);
    Addr(t.wrapping_add(0x44)).sw(c, a2);
    Addr(t.wrapping_add(0x48)).sw(c, a3);
    Addr(t.wrapping_add(0x50)).sw(c, t0);
    Addr(t.wrapping_add(0x54)).sw(c, t1);
    Addr(t.wrapping_add(0x4c)).sw(c, t2);
    Addr(t.wrapping_add(0x58)).sw(c, t3);
    Addr(t.wrapping_add(0x14)).sw(c, t4);
    Addr(t.wrapping_add(0x10)).sw(c, t5);
    Addr(t.wrapping_add(0x1c)).sw(c, t6);
    Addr(t.wrapping_add(0x18)).sw(c, t7);
    Addr(t.wrapping_add(0x34)).sw(c, t8);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// cPS2MemCard_SetState (0x2c48c0)

/// `cPS2MemCard_SetState`'s entry and the head of state 7's slot scan (the
/// one point it is re-entered at after an event).
pub(crate) const MC_SET_STATE_ENTRY: u32 = 0x002c_48c0;

pub(crate) const MC_SET_STATE_SCAN_HEAD: u32 = 0x002c_4e60;

/// The state jump table (.rodata, one word per state 1..=0x4e).
pub(crate) const MCU_SET_STATE_TABLE: u32 = 0x0048_60c0;

/// `cPS2MemCard_SetState`'s stack frame (s0..s4 and `ra`).
pub(crate) const SET_STATE_FRAME: Frame = Frame { size: 0xa0 };

/// `cPS2MemCard` field: the icon title's line-break offset (`OffsLF`; 0 or
/// less: after the whole title).
pub(crate) const MCU_TITLE_BREAK: u32 = 0x8;

/// `cPS2MemCard` field: the caller's buffer of the current card read/write.
pub(crate) const MCU_BUFFER: u32 = 0x2c;

/// `cPS2MemCard` field: the file or directory name a request names.
pub(crate) const MCU_FILE: u32 = 0xf9;

/// `cPS2MemCard` field: the 0x40-byte `sceMcGetDir` entry buffer.
pub(crate) const MCU_DIR_ENTRY: u32 = 0x480;

/// `cPS2MemCard` field: the directory the save's icon files are loaded from.
pub(crate) const MCU_ICON_DIR: u32 = 0x14c0;

/// `cPS2MemCard` vtable slot 0x218: `HasSlotName(slot)`, whether save slot
/// `slot` holds a named save.
pub(crate) const MCU_VT_HAS_SLOT_NAME: u32 = 0x218;

/// The shared `icon.sys` header buffer (`sceMcIconSys`, 0x3c4 bytes).
pub(crate) const MC_ICON_SYS: u32 = 0x0053_76f8;

pub(crate) const MC_ICON_SYS_SIZE: u32 = 0x3c4;

/// `"%s/%s"`: a file in a directory.
pub(crate) const MC_DIR_FMT: u32 = 0x004a_3958;

/// `"PS2D"`: the `icon.sys` magic (4 bytes and a NUL).
pub(crate) const MC_ICON_MAGIC: u32 = 0x004a_3960;

/// The three icon file names (view, copy, delete).
pub(crate) const MC_ICON_FILES: u32 = 0x0044_59f0;

/// The `icon.sys` colour and light constants: background colours (0x40
/// bytes), light directions (0x30), light colours (0x30), ambient (0x10).
pub(crate) const MC_ICON_BG: u32 = 0x0044_5a00;

pub(crate) const MC_ICON_LIGHT_DIR: u32 = 0x0044_5a40;

pub(crate) const MC_ICON_LIGHT_COLOR: u32 = 0x0044_5a70;

pub(crate) const MC_ICON_AMBIENT: u32 = 0x0044_5aa0;

/// `cPS2MemCard` states `cPS2MemCard_SetState` acts on.
///
/// State 1: the operation failed: record the result in the port's status,
/// drop the async buffer, close the open file (state 2 while that close
/// runs) and forget the request.
pub(crate) const MC_STATE_FAILED: u64 = 0x1;

/// State 2: waiting for the close state 1 started.
pub(crate) const MC_STATE_FAILED_CLOSING: u64 = 0x2;

/// State 3: refresh the card info now (`SetState` runs `DoGetInfo` at once).
pub(crate) const MC_STATE_GET_INFO_NOW: u64 = 0x3;

/// State 7: find the first slot that holds a save and open its `icon.sys`.
pub(crate) const MC_STATE_FIND_SAVE: u64 = 0x7;

/// State 8: open the slot's `icon.sys` for reading.
pub(crate) const MC_STATE_OPEN_ICON_SYS: u64 = 0x8;

/// State 9: close the slot's `icon.sys`.
pub(crate) const MC_STATE_CLOSE_ICON_SYS: u64 = 0x9;

/// State 0xa: read the slot's `icon.sys` into the shared header buffer.
pub(crate) const MC_STATE_READ_ICON_SYS: u64 = 0xa;

/// State 0xb: read the directory entry of the slot's save directory.
pub(crate) const MC_STATE_GET_SAVE_DIR: u64 = 0xb;

/// States 0xc-0xe: read the directory entry of icon file 0, 1, 2.
pub(crate) const MC_STATE_GET_ICON_DIR: [u64; 3] = [0xc, 0xd, 0xe];

/// State 0xf: close the `icon.sys` whose read failed.
pub(crate) const MC_STATE_CLOSE_BAD_SAVE: u64 = 0xf;

/// State 0x10: the slot's save is invalid.
pub(crate) const MC_STATE_SAVE_INVALID: u64 = 0x10;

/// State 0x16: create the file named at `+0xf9` for writing.
pub(crate) const MC_STATE_OPEN_FOR_WRITE: u64 = 0x16;

/// State 0x17: write the buffer to the open save file (`SetState` starts
/// `sceMcWrite` at once).
pub(crate) const MC_STATE_WRITE_SAVE_FILE: u64 = 0x17;

/// State 0x18: close the save file after a write (state 0x19 if none is open).
pub(crate) const MC_STATE_CLOSE_AFTER_WRITE: u64 = 0x18;

/// State 0x19: the save file is written and closed.
pub(crate) const MC_STATE_WRITE_DONE: u64 = 0x19;

/// State 0x1c: open the file named at `+0xf9` for reading.
pub(crate) const MC_STATE_OPEN_FOR_READ: u64 = 0x1c;

/// State 0x1d: read the open save file into the buffer (`SetState` starts
/// `sceMcRead` at once).
pub(crate) const MC_STATE_READ_SAVE_FILE: u64 = 0x1d;

/// State 0x1e: close the save file after a read (state 0x1f if none is open).
pub(crate) const MC_STATE_CLOSE_AFTER_READ: u64 = 0x1e;

/// State 0x1f: the save file is read and closed.
pub(crate) const MC_STATE_READ_DONE: u64 = 0x1f;

/// State 0x24: make the directory named at `+0xf9`.
pub(crate) const MC_STATE_MAKE_DIR: u64 = 0x24;

/// State 0x25: create `icon.sys` in that directory.
pub(crate) const MC_STATE_CREATE_ICON_SYS: u64 = 0x25;

/// State 0x26: build the `icon.sys` header and write it.
pub(crate) const MC_STATE_WRITE_ICON_SYS: u64 = 0x26;

/// State 0x27: close the new `icon.sys`.
pub(crate) const MC_STATE_CLOSE_NEW_ICON_SYS: u64 = 0x27;

/// States 0x28, 0x2c, 0x30: start loading icon file 0, 1, 2 from the icon
/// directory (async file load).
pub(crate) const MC_STATE_LOAD_ICON: [u64; 3] = [0x28, 0x2c, 0x30];

/// States 0x29, 0x2d, 0x31: create the icon file in the directory at `+0xb9`.
pub(crate) const MC_STATE_CREATE_ICON: [u64; 3] = [0x29, 0x2d, 0x31];

/// States 0x2a, 0x2e, 0x32: write the loaded icon to it.
pub(crate) const MC_STATE_WRITE_ICON: [u64; 3] = [0x2a, 0x2e, 0x32];

/// States 0x2b, 0x2f, 0x33: drop the loaded icon and close the file.
pub(crate) const MC_STATE_CLOSE_ICON: [u64; 3] = [0x2b, 0x2f, 0x33];

/// State 0x36: delete the file named at `+0xf9` (the port's status cleared).
pub(crate) const MC_STATE_DELETE_FILE: u64 = 0x36;

/// States 0x37-0x39: delete icon file 0, 1, 2 from the directory at `+0xb9`.
pub(crate) const MC_STATE_DELETE_ICON: [u64; 3] = [0x37, 0x38, 0x39];

/// State 0x3a: delete that directory's `icon.sys`.
pub(crate) const MC_STATE_DELETE_ICON_SYS: u64 = 0x3a;

/// State 0x3b: delete the directory at `+0xb9`.
pub(crate) const MC_STATE_DELETE_DIR: u64 = 0x3b;

/// States `SetState` only records (their work is done by the update). Every
/// other state of 1..=0x4e it has no step for, and any state out of range,
/// refreshes the card info unless a request is running.
pub(crate) const MC_SET_STATE_RECORD_ONLY: [u64; 21] = [
    0x2, 0x4, 0x6, 0x10, 0x12, 0x15, 0x19, 0x1b, 0x1f, 0x23, 0x34, 0x35, 0x3d, 0x40, 0x42, 0x44,
    0x46, 0x48, 0x4a, 0x4c, 0x4e,
];

/// Where a state step continues: the shared exit blocks, in code order
/// (each one falls through to the next one it names).
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum SetStateExit {
    /// 0x2c4e38: the request's result in v0; negative: state 1.
    Check,
    /// 0x2c4e44: state 1 (a0 = self).
    Fail,
    /// 0x2c5054: the save is invalid, state 0x10 (a0 = self).
    Invalid,
    /// 0x2c5064: the file is closed: forget its descriptor.
    Closed,
    /// 0x2c50ac: refresh the card info unless a request is running.
    Refresh,
    /// 0x2c50c0: restore s0, then the epilogue.
    Done,
    /// 0x2c50c4: s0 already restored.
    Epilogue,
}

/// `self` (s2) field `off`.
#[inline]
pub(crate) fn set_state_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S2).wrapping_add(off))
}

/// The address of `self` field `off`, as a register value.
#[inline]
pub(crate) fn set_state_addr(c: &Cpu, off: u32) -> u64 {
    sx(r32(c, S2).wrapping_add(off))
}

/// Keep the `sceMc*` result (v0) in the gp-relative result word.
#[inline]
pub(crate) fn set_state_keep_result(c: &mut Cpu) {
    let v0 = c.g[V0][0];
    Addr(r32(c, GP).wrapping_add(MCU_GP_RESULT)).sw(c, v0);
}

/// `cPS2MemCard_SetState(self, next)` returning to `ret`, then restore s0.
pub(crate) fn set_state_go(c: &mut Cpu, next: u64, ret: u32) -> Result<SetStateExit, Unwound> {
    c.g[A1][0] = next;
    callee::MEM_CARD_SET_STATE.call(c, ret)?;
    SET_STATE_FRAME.restore(c, S0, 0x90);
    Ok(SetStateExit::Epilogue)
}

/// `void cPS2MemCard_SetState(cPS2MemCard* self, int state)` (0x2c48c0):
/// enter `state` and start its first step: open, read, write, close or
/// delete the files of a save (`sceMc*`, results kept in the gp result
/// word; a failed request goes to state 1, a bad save to state 0x10), build
/// the `icon.sys` header, start loading the icon files, scan the slots for
/// a save (state 7), or refresh the card info. Re-entered at the slot scan's
/// head after an event.
pub fn cps2_mem_card_set_state(c: &mut Cpu) {
    let resumed = match c.pc {
        MC_SET_STATE_ENTRY => false,
        MC_SET_STATE_SCAN_HEAD => true,
        _ => return ee::s08::f_002c48c0__generated(c),
    };
    guest(c, |c| {
        let exit = if resumed {
            set_state_find_save_scan(c, true)?
        } else {
            set_state_enter(c)?
        };
        set_state_finish(c, exit)
    })
}

/// The prologue, the state store and the dispatch on the state.
pub(crate) fn set_state_enter(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 12;
    let frame = Frame::open(c, SET_STATE_FRAME.size);
    c.g[V1][0] = sx(r32(c, A1).wrapping_sub(1));
    frame.save(c, S2, 0x70);
    c.g[V0][0] = (c.g[V1][0] < 0x4e) as u64;
    frame.save(c, S0, 0x90);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S1, 0x80);
    frame.save(c, S3, 0x60);
    frame.save(c, S4, 0x50);
    frame.save_ra(c, 0x40);
    let in_table = c.g[V0][0] != 0;
    let a1 = c.g[A1][0];
    set_state_field(c, MCU_STATE).sw(c, a1);
    if !in_table {
        return Ok(SetStateExit::Refresh);
    }
    // The table word of the state (a0); the arms below are its targets.
    c.cycles += 7;
    c.g[V0][0] = sx(MCU_SET_STATE_TABLE);
    c.g[V1][0] = sx((r32(c, V1) << 2).wrapping_add(MCU_SET_STATE_TABLE));
    c.g[A0][0] = Addr(r32(c, V1)).lw(c);
    let state = r32(c, A1) as u64;
    match state {
        MC_STATE_FAILED => set_state_failed(c),
        MC_STATE_GET_INFO_NOW => {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            callee::C_PS2_MEM_CARD_DO_GET_INFO.call(c, 0x002c_4988)?;
            SET_STATE_FRAME.restore(c, S0, 0x90);
            Ok(SetStateExit::Epilogue)
        }
        MC_STATE_FIND_SAVE => set_state_find_save(c),
        MC_STATE_OPEN_ICON_SYS => set_state_open_icon_sys(c),
        MC_STATE_CLOSE_ICON_SYS => set_state_close_icon_sys(c),
        MC_STATE_READ_ICON_SYS => set_state_read_icon_sys(c),
        MC_STATE_GET_SAVE_DIR => set_state_get_save_dir(c),
        s if MC_STATE_GET_ICON_DIR.contains(&s) => set_state_get_icon_dir(c),
        MC_STATE_CLOSE_BAD_SAVE => set_state_close_bad_save(c),
        MC_STATE_OPEN_FOR_WRITE => set_state_open_for_write(c),
        MC_STATE_WRITE_SAVE_FILE => set_state_write_save_file(c),
        MC_STATE_CLOSE_AFTER_WRITE => {
            set_state_close_then(c, 0x002c_4cbc, MC_STATE_WRITE_DONE, 0x002c_4cdc)
        }
        MC_STATE_OPEN_FOR_READ => set_state_open_for_read(c),
        MC_STATE_READ_SAVE_FILE => set_state_read_save_file(c),
        MC_STATE_CLOSE_AFTER_READ => {
            set_state_close_then(c, 0x002c_4d48, MC_STATE_READ_DONE, 0x002c_4d68)
        }
        MC_STATE_MAKE_DIR => set_state_make_dir(c),
        MC_STATE_CREATE_ICON_SYS => set_state_create_icon_sys(c),
        MC_STATE_WRITE_ICON_SYS => set_state_write_icon_sys(c),
        MC_STATE_CLOSE_NEW_ICON_SYS => {
            c.cycles += 4;
            c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
            call_ptr(c, SCE_MC_CLOSE, 0x002c_4b08)?;
            Ok(set_state_handle_done(c))
        }
        s if MC_STATE_LOAD_ICON.contains(&s) => set_state_load_icon(c),
        s if MC_STATE_CREATE_ICON.contains(&s) => set_state_create_icon(c),
        s if MC_STATE_WRITE_ICON.contains(&s) => {
            c.cycles += 6;
            c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
            c.g[A1][0] = set_state_field(c, MCU_ASYNC_A).lw(c);
            c.g[A2][0] = set_state_field(c, MCU_ASYNC_B).lw(c);
            call_ptr(c, callee::SCE_MC_WRITE, 0x002c_4c1c)?;
            set_state_keep_result(c);
            Ok(SetStateExit::Check)
        }
        s if MC_STATE_CLOSE_ICON.contains(&s) => set_state_close_icon(c),
        MC_STATE_DELETE_FILE => {
            c.cycles += 9;
            c.g[V0][0] = set_state_field(c, MCU_PORT).lw(c);
            c.g[V1][0] = MCU_PORT_INFO_SIZE as u64;
            c.g[A1][0] = 0;
            c.g[A2][0] = set_state_addr(c, MCU_FILE);
            c.mult(0, A0, V0, V1);
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
            Addr(r32(c, V0).wrapping_add(MCU_PORTS + MCU_PORT_INFO_STATUS)).sw(c, 0);
            c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
            set_state_delete(c)
        }
        s if MC_STATE_DELETE_ICON.contains(&s) => {
            c.g[V1][0] = set_state_field(c, MCU_STATE).lw(c);
            set_state_icon_index(c, 4, MC_STATE_DELETE_ICON[1], MC_STATE_DELETE_ICON[2]);
            c.cycles += 15;
            set_state_icon_file(c);
            c.g[S0][0] = set_state_addr(c, MCU_PATH);
            c.g[A1][0] = sx(MCU_PATH_FMT);
            c.g[A3][0] = Addr(r32(c, V1)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = set_state_addr(c, MCU_DIR);
            call_ptr(c, MCU_SPRINTF, 0x002c_4de0)?;
            c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            set_state_delete(c)
        }
        MC_STATE_DELETE_ICON_SYS => {
            c.cycles += 12;
            c.g[S0][0] = set_state_addr(c, MCU_PATH);
            c.g[A1][0] = sx(MCU_PATH_FMT);
            c.g[A3][0] = sx(MCU_PATH_EXT);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = set_state_addr(c, MCU_DIR);
            call_ptr(c, MCU_SPRINTF, 0x002c_4e10)?;
            c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            set_state_delete(c)
        }
        MC_STATE_DELETE_DIR => {
            c.cycles += 3;
            c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = set_state_addr(c, MCU_DIR);
            set_state_delete(c)
        }
        s if MC_SET_STATE_RECORD_ONLY.contains(&s) => Ok(SetStateExit::Done),
        _ => Ok(SetStateExit::Refresh),
    }
}

/// The shared exit blocks and the epilogue.
pub(crate) fn set_state_finish(c: &mut Cpu, exit: SetStateExit) -> Flow {
    let mut exit = exit;
    if exit == SetStateExit::Check {
        c.cycles += 2;
        let ok = (c.g[V0][0] as i64) >= 0;
        // Delay slot: runs on both arms.
        SET_STATE_FRAME.restore(c, S0, 0x90);
        exit = if ok {
            SetStateExit::Epilogue
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            SetStateExit::Fail
        };
    }
    if exit == SetStateExit::Fail {
        c.cycles += 4;
        exit = set_state_go(c, MC_STATE_FAILED, 0x002c_4e4c)?;
    }
    if exit == SetStateExit::Invalid {
        c.cycles += 4;
        exit = set_state_go(c, MC_STATE_SAVE_INVALID, 0x002c_505c)?;
    }
    if exit == SetStateExit::Closed {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
        exit = SetStateExit::Done;
    }
    if exit == SetStateExit::Refresh {
        c.cycles += 3;
        c.g[V0][0] = set_state_field(c, MCU_BUSY).lw(c);
        let busy = c.g[V0][0] != 0;
        // Delay slot: runs on both arms.
        SET_STATE_FRAME.restore(c, S0, 0x90);
        exit = if busy {
            SetStateExit::Epilogue
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            callee::C_PS2_MEM_CARD_DO_GET_INFO.call(c, 0x002c_50c0)?;
            SetStateExit::Done
        };
    }
    if exit == SetStateExit::Done {
        c.cycles += 1;
        SET_STATE_FRAME.restore(c, S0, 0x90);
    }
    c.cycles += 7;
    SET_STATE_FRAME.restore(c, S1, 0x80);
    SET_STATE_FRAME.restore(c, S2, 0x70);
    SET_STATE_FRAME.restore(c, S3, 0x60);
    SET_STATE_FRAME.restore(c, S4, 0x50);
    SET_STATE_FRAME.restore_ra(c, 0x40);
    SET_STATE_FRAME.close(c);
    Ok(())
}

/// The handle is no longer needed (v0 = the `sceMc*` result): forget it;
/// a failure also goes to state 1.
pub(crate) fn set_state_handle_done(c: &mut Cpu) -> SetStateExit {
    let ok = (c.g[V0][0] as i64) >= 0;
    set_state_keep_result(c);
    if ok {
        return SetStateExit::Closed;
    }
    c.cycles += 4;
    c.g[V0][0] = u64::MAX;
    c.g[A0][0] = c.g[S2][0];
    set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
    SetStateExit::Fail
}

/// 0x2c4e2c: `sceMcDelete(a0, a1, a2)`, its result checked.
pub(crate) fn set_state_delete(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 3;
    call_ptr(c, callee::SCE_MC_DELETE, 0x002c_4e34)?;
    set_state_keep_result(c);
    Ok(SetStateExit::Check)
}

/// Which icon file a three-state step is on (v1 = the state, `cycles` the
/// block's charge): 1 for `second`, 2 for `third`, else 0, in v1.
pub(crate) fn set_state_icon_index(c: &mut Cpu, cycles: u64, second: u64, third: u64) {
    c.cycles += cycles;
    c.g[V0][0] = second;
    let is_second = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = c.g[V1][0] ^ third;
    if is_second {
        c.cycles += 1;
        c.g[V1][0] = 1;
    } else {
        c.cycles += 3;
        c.g[V1][0] = if c.g[V0][0] != 0 { 0 } else { 2 };
    }
}

/// v1 = the address of icon file name v1 (v0 = the name table).
pub(crate) fn set_state_icon_file(c: &mut Cpu) {
    c.g[V0][0] = sx(MC_ICON_FILES);
    c.g[V1][0] = sx((r32(c, V1) << 2).wrapping_add(MC_ICON_FILES));
}

/// State 1: record the result in the port's status, free the async buffer,
/// close the open file (state 2 when the close starts) and clear the slot,
/// size and buffer.
pub(crate) fn set_state_failed(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 9;
    c.g[V0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[V1][0] = MCU_PORT_INFO_SIZE as u64;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_add(MCU_GP_RESULT)).lw(c);
    c.mult(0, A1, V0, V1);
    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S2)));
    let result = c.g[A0][0];
    Addr(r32(c, V0).wrapping_add(MCU_PORTS + MCU_PORT_INFO_STATUS)).sw(c, result);
    c.g[A0][0] = set_state_field(c, MCU_ASYNC_A).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        call_ptr(c, callee::MEM_FREE, 0x002c_4938)?;
        set_state_field(c, MCU_ASYNC_A).sw(c, 0);
    }
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    c.cycles += 2;
    let none_open = (c.g[A0][0] as i64) < 0;
    c.g[V0][0] = u64::MAX;
    if !none_open {
        c.cycles += 4;
        call_ptr(c, SCE_MC_CLOSE, 0x002c_4950)?;
        let failed = (c.g[V0][0] as i64) < 0;
        set_state_keep_result(c);
        if !failed {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = MC_STATE_FAILED_CLOSING;
            callee::MEM_CARD_SET_STATE.call(c, 0x002c_4964)?;
        }
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
    }
    c.cycles += 4;
    set_state_field(c, MCU_SIZE).sw(c, 0);
    let v0 = c.g[V0][0];
    set_state_field(c, MCU_SLOT).sw(c, v0);
    set_state_field(c, MCU_BUFFER).sw(c, 0);
    Ok(SetStateExit::Done)
}

/// State 7: no slots: done. Else scan from slot 0.
pub(crate) fn set_state_find_save(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = set_state_field(c, MCU_COUNT).lw(c);
    let none = (c.g[V0][0] as i64) <= 0;
    set_state_field(c, MCU_INDEX).sw(c, 0);
    if none {
        return Ok(SetStateExit::Done);
    }
    set_state_find_save_scan(c, false)
}

/// State 7's scan (head 0x2c4e60): ask the vtable whether slot `+0x28`
/// holds a save; the first that does gets its `icon.sys` path and state 8.
/// Entered at the head after an event (`resumed`), a due event at the back
/// edge unwinds; on the first pass it is delivered and the scan goes on.
pub(crate) fn set_state_find_save_scan(
    c: &mut Cpu,
    resumed: bool,
) -> Result<SetStateExit, Unwound> {
    loop {
        c.cycles += 8;
        c.g[V1][0] = set_state_field(c, 0).lw(c);
        c.g[A1][0] = set_state_field(c, MCU_INDEX).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(MCU_VT_HAS_SLOT_NAME)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(MCU_VT_HAS_SLOT_NAME + 4)).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x002c_4e78)?;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = set_state_field(c, MCU_INDEX).lw(c);
            return set_state_open_found_save(c);
        }
        c.cycles += 6;
        c.g[V0][0] = set_state_field(c, MCU_INDEX).lw(c);
        c.g[V1][0] = set_state_field(c, MCU_COUNT).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        let more = c.g[V1][0] != 0;
        let next = c.g[V0][0];
        set_state_field(c, MCU_INDEX).sw(c, next);
        if !more {
            c.cycles += 2;
            SET_STATE_FRAME.restore(c, S0, 0x90);
            return Ok(SetStateExit::Epilogue);
        }
        if resumed {
            if back_edge_unwinds(c, MC_SET_STATE_SCAN_HEAD) {
                return Err(Unwound);
            }
        } else if event_due(c) {
            deliver_event_here(c, MC_SET_STATE_SCAN_HEAD);
        }
    }
}

/// 0x2c5070: slot a2 holds a save: path `/<slot name>/icon.sys`, state 8.
pub(crate) fn set_state_open_found_save(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 15;
    c.g[V0][0] = 0x78;
    c.g[A1][0] = sx(MCU_PATH_FMT);
    c.g[A3][0] = sx(MCU_PATH_EXT);
    c.mult(0, V1, A2, V0);
    c.g[A0][0] = set_state_addr(c, MCU_PATH);
    c.g[A2][0] = sx(r32(c, V1)
        .wrapping_add(r32(c, S2))
        .wrapping_add(MCU_ENTRIES + 0xc));
    call_ptr(c, MCU_SPRINTF, 0x002c_5098)?;
    c.g[A0][0] = c.g[S2][0];
    set_state_go(c, MC_STATE_OPEN_ICON_SYS, 0x002c_50a4)
}

/// State 8: open the path at `+0x79` for reading; on failure the slot's
/// save is marked invalid (state 0x10).
pub(crate) fn set_state_open_icon_sys(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 7;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A1][0] = 0;
    c.g[A2][0] = set_state_addr(c, MCU_PATH);
    c.g[A3][0] = 1;
    call_ptr(c, callee::SCE_MC_OPEN, 0x002c_4eb4)?;
    let ok = (c.g[V0][0] as i64) >= 0;
    set_state_keep_result(c);
    if ok {
        return Ok(SetStateExit::Closed);
    }
    c.cycles += 7;
    c.g[A1][0] = set_state_field(c, MCU_INDEX).lw(c);
    c.g[V0][0] = u64::MAX;
    c.g[A0][0] = c.g[S2][0];
    set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
    MCU_MARK_SAVE_INVALID.call(c, 0x002c_4ed0)?;
    c.g[A0][0] = c.g[S2][0];
    Ok(SetStateExit::Invalid)
}

/// State 0xa: read the header into the shared buffer; on failure the save
/// is marked invalid and its file closed (state 0xf).
pub(crate) fn set_state_read_icon_sys(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 7;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    c.g[A1][0] = sx(MC_ICON_SYS);
    c.g[A2][0] = MC_ICON_SYS_SIZE as u64;
    call_ptr(c, callee::SCE_MC_READ, 0x002c_4eec)?;
    let ok = (c.g[V0][0] as i64) >= 0;
    set_state_keep_result(c);
    if ok {
        return Ok(SetStateExit::Done);
    }
    c.cycles += 8;
    c.g[A1][0] = set_state_field(c, MCU_INDEX).lw(c);
    c.g[A0][0] = c.g[S2][0];
    MCU_MARK_SAVE_INVALID.call(c, 0x002c_4f00)?;
    c.g[A0][0] = c.g[S2][0];
    set_state_go(c, MC_STATE_CLOSE_BAD_SAVE, 0x002c_4f0c)
}

/// State 9: close the `icon.sys`; on failure the save is invalid.
pub(crate) fn set_state_close_icon_sys(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    call_ptr(c, SCE_MC_CLOSE, 0x002c_4f1c)?;
    let ok = (c.g[V0][0] as i64) >= 0;
    set_state_keep_result(c);
    if ok {
        return Ok(SetStateExit::Closed);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    Ok(SetStateExit::Invalid)
}

/// State 0xb: `sceMcGetDir` of `/<slot name>/<slot name>` (built at
/// `+0xf9`) into the entry buffer; on failure the save is invalid.
pub(crate) fn set_state_get_save_dir(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 26;
    c.g[A2][0] = set_state_field(c, MCU_INDEX).lw(c);
    c.g[V0][0] = 0x78;
    c.g[S1][0] = set_state_addr(c, MCU_FILE);
    set_state_field(c, MCU_ACTIVE).sw(c, 0);
    c.mult(0, V1, A2, V0);
    c.g[A1][0] = sx(MCU_PATH_FMT);
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = set_state_addr(c, MCU_DIR_ENTRY);
    c.g[A2][0] = sx(r32(c, V1)
        .wrapping_add(r32(c, S2))
        .wrapping_add(MCU_ENTRIES + 0xc));
    c.g[A3][0] = c.g[A2][0];
    call_ptr(c, MCU_SPRINTF, 0x002c_4f60)?;
    set_state_get_dir(c, 0x002c_4f70, 0x002c_4f8c)
}

/// The entry buffer cleared, then `sceMcGetDir(port, 0, s1, 0, 1, s0)`;
/// on failure the save is invalid.
pub(crate) fn set_state_get_dir(
    c: &mut Cpu,
    fill_ret: u32,
    ret: u32,
) -> Result<SetStateExit, Unwound> {
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x40;
    call_ptr(c, MCU_MEM_FILL, fill_ret)?;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A2][0] = c.g[S1][0];
    c.g[T1][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A3][0] = 0;
    c.g[T0][0] = 1;
    call_ptr(c, callee::SCE_MC_GET_DIR, ret)?;
    let ok = (c.g[V0][0] as i64) >= 0;
    set_state_keep_result(c);
    if ok {
        return Ok(SetStateExit::Done);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    Ok(SetStateExit::Invalid)
}

/// States 0xc-0xe: `sceMcGetDir` of `/<slot name>/<icon file>` (built at
/// `+0x79`) into the entry buffer.
pub(crate) fn set_state_get_icon_dir(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.g[V1][0] = set_state_field(c, MCU_STATE).lw(c);
    set_state_icon_index(c, 4, MC_STATE_GET_ICON_DIR[1], MC_STATE_GET_ICON_DIR[2]);
    c.cycles += 29;
    c.g[A2][0] = set_state_field(c, MCU_INDEX).lw(c);
    c.g[A0][0] = 0x78;
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.mult(0, A1, A2, A0);
    c.g[V0][0] = sx(MC_ICON_FILES);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(MC_ICON_FILES));
    c.g[S1][0] = set_state_addr(c, MCU_PATH);
    c.g[A3][0] = Addr(r32(c, V1)).lw(c);
    c.g[A0][0] = c.g[S1][0];
    c.g[S0][0] = set_state_addr(c, MCU_DIR_ENTRY);
    c.g[A2][0] = sx(r32(c, A1)
        .wrapping_add(r32(c, S2))
        .wrapping_add(MCU_ENTRIES + 0xc));
    c.g[A1][0] = sx(MCU_PATH_FMT);
    call_ptr(c, MCU_SPRINTF, 0x002c_4ffc)?;
    set_state_get_dir(c, 0x002c_500c, 0x002c_5028)
}

/// State 0xf: close the file whose header read failed; the save is invalid
/// either way.
pub(crate) fn set_state_close_bad_save(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    call_ptr(c, SCE_MC_CLOSE, 0x002c_5040)?;
    let ok = (c.g[V0][0] as i64) >= 0;
    set_state_keep_result(c);
    if ok {
        return Ok(SetStateExit::Closed);
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    c.g[A0][0] = c.g[S2][0];
    set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
    Ok(SetStateExit::Invalid)
}

/// State 0x16: `sceMcOpen(port, 0, +0xf9, CREAT | WRONLY)`.
pub(crate) fn set_state_open_for_write(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 7;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A1][0] = 0;
    c.g[A2][0] = set_state_addr(c, MCU_FILE);
    c.g[A3][0] = 0x203;
    call_ptr(c, callee::SCE_MC_OPEN, 0x002c_4c70)?;
    Ok(set_state_handle_done(c))
}

/// State 0x17: `sceMcWrite(fd, buffer, size)`; buffer and size are cleared.
pub(crate) fn set_state_write_save_file(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 8;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    c.g[A1][0] = set_state_field(c, MCU_BUFFER).lw(c);
    c.g[A2][0] = set_state_field(c, MCU_SIZE).lw(c);
    call_ptr(c, callee::SCE_MC_WRITE, 0x002c_4c98)?;
    set_state_keep_result(c);
    set_state_field(c, MCU_BUFFER).sw(c, 0);
    set_state_field(c, MCU_SIZE).sw(c, 0);
    Ok(SetStateExit::Check)
}

/// States 0x18 and 0x1e: close the open file; with none open, go straight
/// to `next` (`ret`: that call's return address).
pub(crate) fn set_state_close_then(
    c: &mut Cpu,
    close_ret: u32,
    next: u64,
    ret: u32,
) -> Result<SetStateExit, Unwound> {
    c.cycles += 3;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    if (c.g[A0][0] as i64) < 0 {
        c.g[A0][0] = c.g[S2][0];
        c.cycles += 4;
        return set_state_go(c, next, ret);
    }
    c.cycles += 4;
    call_ptr(c, SCE_MC_CLOSE, close_ret)?;
    Ok(set_state_handle_done(c))
}

/// State 0x1c: `sceMcOpen(port, 0, +0xf9, RDONLY)`; anything but 0 fails.
pub(crate) fn set_state_open_for_read(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 10;
    set_state_field(c, MCU_REQ_1E).sw(c, 0);
    c.g[A1][0] = 0;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A2][0] = set_state_addr(c, MCU_FILE);
    c.g[A3][0] = 1;
    call_ptr(c, callee::SCE_MC_OPEN, 0x002c_4cfc)?;
    c.g[V1][0] = u64::MAX;
    set_state_keep_result(c);
    let started = c.g[V0][0] == 0;
    set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
    if started {
        return Ok(SetStateExit::Done);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    Ok(SetStateExit::Fail)
}

/// State 0x1d: `sceMcRead(fd, buffer, size)`; anything but 0 fails.
pub(crate) fn set_state_read_save_file(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 6;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    c.g[A1][0] = set_state_field(c, MCU_BUFFER).lw(c);
    c.g[A2][0] = set_state_field(c, MCU_SIZE).lw(c);
    call_ptr(c, callee::SCE_MC_READ, 0x002c_4d24)?;
    let started = c.g[V0][0] == 0;
    set_state_keep_result(c);
    if started {
        return Ok(SetStateExit::Done);
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    Ok(SetStateExit::Fail)
}

/// State 0x24: `sceMcMkdir(port, 0, +0xf9)`; the pending close request
/// (`+0x48`) is dropped.
pub(crate) fn set_state_make_dir(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 7;
    set_state_field(c, MCU_REQ_18).sw(c, 0);
    c.g[A1][0] = 0;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A2][0] = set_state_addr(c, MCU_FILE);
    call_ptr(c, callee::SCE_MC_MKDIR, 0x002c_49a4)?;
    set_state_keep_result(c);
    Ok(SetStateExit::Check)
}

/// State 0x25: create `<+0xf9>/icon.sys` (path at `+0x79`).
pub(crate) fn set_state_create_icon_sys(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 17;
    c.g[S0][0] = set_state_addr(c, MCU_PATH);
    c.g[A1][0] = sx(MC_DIR_FMT);
    c.g[A3][0] = sx(MCU_PATH_EXT);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = set_state_addr(c, MCU_FILE);
    call_ptr(c, MCU_SPRINTF, 0x002c_49cc)?;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A3][0] = 0x203;
    call_ptr(c, callee::SCE_MC_OPEN, 0x002c_49e0)?;
    c.g[V1][0] = u64::MAX;
    set_state_keep_result(c);
    set_state_field(c, MCU_RESULT).sw(c, u64::MAX);
    Ok(SetStateExit::Check)
}

/// State 0x26: fill in the `icon.sys` header (magic, the title in SJIS
/// with its line break, transparency, colours and lights, the three icon
/// file names) and write it to the open file.
pub(crate) fn set_state_write_icon_sys(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 22;
    c.g[S0][0] = 0x53_0000;
    c.g[A1][0] = 0;
    c.g[S3][0] = sx(MC_ICON_SYS);
    c.g[A2][0] = MC_ICON_SYS_SIZE as u64;
    c.g[A0][0] = c.g[S3][0];
    c.g[S1][0] = set_state_addr(c, MCU_NAME);
    call_ptr(c, MCU_MEM_FILL, 0x002c_4a0c)?;
    c.g[S4][0] = c.g[S0][0];
    c.g[V0][0] = 0x4a_0000;
    c.g[A0][0] = c.g[S2][0];
    c.g[V1][0] = sx(MC_ICON_MAGIC);
    c.g[T1][0] = sx(r32(c, S0).wrapping_add(0x76f8));
    c.g[A3][0] = Addr(r32(c, V1)).lw(c);
    c.g[T0][0] = Addr(r32(c, V1).wrapping_add(4)).lb(c);
    let (magic, nul) = (c.g[A3][0], c.g[T0][0]);
    Addr(r32(c, T1)).sw(c, magic);
    Addr(r32(c, T1).wrapping_add(4)).sb(c, nul);
    // The title, at +0xc0.
    c.g[A1][0] = sx(r32(c, S3).wrapping_add(0xc0));
    c.g[A2][0] = c.g[S1][0];
    callee::MEM_CARD_ASCII_TO_SJIS_STRING.call(c, 0x002c_4a3c)?;
    c.g[V0][0] = set_state_field(c, MCU_TITLE_BREAK).lw(c);
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 2;
        c.g[V0][0] = set_state_field(c, MCU_TITLE_BREAK).lhu(c);
    } else {
        // After the whole title: two SJIS bytes per character.
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, MCU_STRLEN, 0x002c_4a58)?;
        c.g[V0][0] = sx(r32(c, V0) << 1);
    }
    c.cycles += 41;
    let line_break = c.g[V0][0];
    Addr(r32(c, S3).wrapping_add(6)).sh(c, line_break);
    c.g[S1][0] = sx(r32(c, S4).wrapping_add(0x76f8));
    c.g[V0][0] = 0x60;
    Addr(r32(c, S1).wrapping_add(0xc)).sw(c, 0x60);
    for (dst, src, len, ret) in [
        (0x10, MC_ICON_BG, 0x40, 0x002c_4a80),
        (0x50, MC_ICON_LIGHT_DIR, 0x30, 0x002c_4a94),
        (0x80, MC_ICON_LIGHT_COLOR, 0x30, 0x002c_4aa8),
        (0xb0, MC_ICON_AMBIENT, 0x10, 0x002c_4abc),
    ] {
        c.g[A1][0] = sx(src);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(dst));
        c.g[A2][0] = len;
        call_ptr(c, callee::EA_MEM_COPY, ret)?;
    }
    // The view, copy and delete icon file names (+0x104, +0x144, +0x184).
    c.g[S0][0] = sx(MC_ICON_FILES);
    for (k, ret) in [0x002c_4ad0u32, 0x002c_4adc, 0x002c_4ae8]
        .into_iter()
        .enumerate()
    {
        let k = k as u32;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x104 + 0x40 * k));
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(4 * k)).lw(c);
        call_ptr(c, MCU_STRCPY, ret)?;
    }
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = MC_ICON_SYS_SIZE as u64;
    call_ptr(c, callee::SCE_MC_WRITE, 0x002c_4af8)?;
    set_state_keep_result(c);
    Ok(SetStateExit::Check)
}

/// States 0x28, 0x2c, 0x30: drop the previous icon, then start loading
/// `<+0x14c0>/<icon file>` (path on the stack; the handle to `+0x38`).
pub(crate) fn set_state_load_icon(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 3;
    c.g[A0][0] = set_state_field(c, MCU_ASYNC_A).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        call_ptr(c, callee::MEM_FREE, 0x002c_4b34)?;
        set_state_field(c, MCU_ASYNC_A).sw(c, 0);
    }
    c.g[V1][0] = set_state_field(c, MCU_STATE).lw(c);
    set_state_icon_index(c, 3, MC_STATE_LOAD_ICON[1], MC_STATE_LOAD_ICON[2]);
    c.cycles += 15;
    set_state_icon_file(c);
    c.g[A0][0] = c.g[SP][0];
    c.g[A3][0] = Addr(r32(c, V1)).lw(c);
    c.g[A1][0] = sx(MC_DIR_FMT);
    c.g[A2][0] = set_state_addr(c, MCU_ICON_DIR);
    call_ptr(c, MCU_SPRINTF, 0x002c_4b80)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0x100;
    call_ptr(c, callee::ASYNC_FILE_LOAD, 0x002c_4b8c)?;
    let handle = c.g[V0][0];
    set_state_field(c, MCU_HANDLE).sw(c, handle);
    Ok(SetStateExit::Done)
}

/// States 0x29, 0x2d, 0x31: create `/<+0xb9>/<icon file>` (path at `+0x79`).
pub(crate) fn set_state_create_icon(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.g[V1][0] = set_state_field(c, MCU_STATE).lw(c);
    set_state_icon_index(c, 4, MC_STATE_CREATE_ICON[1], MC_STATE_CREATE_ICON[2]);
    c.cycles += 18;
    set_state_icon_file(c);
    c.g[S0][0] = set_state_addr(c, MCU_PATH);
    c.g[A3][0] = Addr(r32(c, V1)).lw(c);
    c.g[A1][0] = sx(MCU_PATH_FMT);
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = set_state_addr(c, MCU_DIR);
    call_ptr(c, MCU_SPRINTF, 0x002c_4be0)?;
    c.g[A0][0] = set_state_field(c, MCU_PORT).lw(c);
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    c.g[A3][0] = 0x203;
    call_ptr(c, callee::SCE_MC_OPEN, 0x002c_4bf4)?;
    Ok(set_state_handle_done(c))
}

/// States 0x2b, 0x2f, 0x33: free the loaded icon, close the icon file.
pub(crate) fn set_state_close_icon(c: &mut Cpu) -> Result<SetStateExit, Unwound> {
    c.cycles += 3;
    c.g[A0][0] = set_state_field(c, MCU_ASYNC_A).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        call_ptr(c, callee::MEM_FREE, 0x002c_4c38)?;
        set_state_field(c, MCU_ASYNC_A).sw(c, 0);
    }
    c.cycles += 4;
    c.g[A0][0] = set_state_field(c, MCU_RESULT).lw(c);
    call_ptr(c, SCE_MC_CLOSE, 0x002c_4c44)?;
    Ok(set_state_handle_done(c))
}

// ---------------------------------------------------------------------------
// cPS2MemCard layout used here (the fields the per-frame update names are
// the `MCU_*` constants above).

/// Size of one save-slot entry (`MCU_ENTRIES`, six of them).
pub(crate) const MCU_ENTRY_SIZE: u32 = 0x78;

/// Number of save slots.
pub(crate) const MCU_ENTRY_COUNT: u64 = 6;

/// Save-slot entry word set by `cPS2MemCard_MarkSlotUsed`.
pub(crate) const MCU_ENTRY_USED: u32 = 0x0;

/// Save-slot entry word set by `cPS2MemCard_MarkSaveInvalid`.
pub(crate) const MCU_ENTRY_INVALID: u32 = 0x4;

/// Save-slot entry word set by `cPS2MemCard_MarkSlotsWithoutSpace`.
pub(crate) const MCU_ENTRY_NO_SPACE: u32 = 0x8;

/// Save-slot entry: the save's directory name (ASCII).
pub(crate) const MCU_ENTRY_NAME: u32 = 0xc;

/// Save-slot entry: the ASCII text the status string shows for a saved slot
/// (inside the title at +0x4c).
pub(crate) const MCU_ENTRY_TITLE: u32 = 0x52;

/// Save-slot entry: the save's timestamp, laid out as `sceMcStDateTime`
/// (bytes: seconds, minutes, hours, day, month; then the year as a half).
pub(crate) const MCU_ENTRY_SEC: u32 = 0x6f;

pub(crate) const MCU_ENTRY_MIN: u32 = 0x70;

pub(crate) const MCU_ENTRY_HOUR: u32 = 0x71;

pub(crate) const MCU_ENTRY_DAY: u32 = 0x72;

pub(crate) const MCU_ENTRY_MONTH: u32 = 0x73;

pub(crate) const MCU_ENTRY_YEAR: u32 = 0x74;

/// Size of one per-port record (`MCU_PORTS`, two of them).
pub(crate) const MCU_PORT_INFO_SIZE: u32 = 0x14;

/// Per-port record: the card's status word.
pub(crate) const MCU_PORT_INFO_STATUS: u32 = 0x8;

/// Per-port record: the card's formatted flag.
pub(crate) const MCU_PORT_INFO_FORMATTED: u32 = 0xc;

/// The port status word of an unformatted card.
pub(crate) const MC_STATUS_NO_FORMAT: u32 = 0xffff_fffe;

/// `cPS2MemCard`: the name `RequestListDir` copies (0x40 bytes).
pub(crate) const MCU_LIST_NAME: u32 = 0x139;

/// `cPS2MemCard`: the directory listing table (0x1000 bytes of 0x40-byte
/// entries; the names sit at +0x20, `MCU_DIRBUF`).
pub(crate) const MCU_DIR_TABLE: u32 = 0x4c0;

/// `cPS2MemCard` vtable slots (`{delta, fn}` pairs; the base class's method in
/// each slot, read from its vtable at 0x486f78).
///
/// 0x180: `cPS2MemCard_IsPortNotPS2Card(port)`.
pub(crate) const MCU_VT_IS_PORT_NOT_PS2_CARD: u32 = 0x180;

/// 0x190: `cPS2MemCard_v50(port)` (returns 0).
pub(crate) const MCU_VT_V50: u32 = 0x190;

/// 0x1a0: `cPS2MemCard_v52(port)`.
pub(crate) const MCU_VT_V52: u32 = 0x1a0;

/// 0x1a8: `cPS2MemCard_v53()`, `v52` of the current port.
pub(crate) const MCU_VT_V53: u32 = 0x1a8;

/// 0x1b8: `cPS2MemCard_v55()`, `IsPortFormatted` of the current port.
pub(crate) const MCU_VT_V55: u32 = 0x1b8;

/// 0x1c0: `cPS2MemCard_IsPortFormatted(port)`.
pub(crate) const MCU_VT_IS_PORT_FORMATTED: u32 = 0x1c0;

/// The game object (`gp - 0x848`) and its string table at +0x8c, whose
/// vtable slot 0x20 returns the UTF-16 string for a key hash.
pub(crate) const MC_GAME_GP_OFF: u32 = 0x848;

pub(crate) const MC_GAME_STRINGS: u32 = 0x8c;

pub(crate) const MC_STRINGS_FIND: u32 = 0x20;

/// String-table keys of the card status texts: "kT_MEMFreePS2".
pub(crate) const MC_KEY_FREE: u32 = 0x0048_61f8;

/// "kT_MEMEmpty".
pub(crate) const MC_KEY_EMPTY: u32 = 0x0048_6490;

/// "kT_MEMUnavailable".
pub(crate) const MC_KEY_UNAVAILABLE: u32 = 0x0048_64a0;

/// "kT_MEMFileCorruptPS2".
pub(crate) const MC_KEY_FILE_CORRUPT: u32 = 0x0048_64b8;

/// "kT_MEMNoFreeSpace".
pub(crate) const MC_KEY_NO_FREE_SPACE: u32 = 0x0048_64d0;

/// The ASCII text (" ") of a port with no free-space figure.
pub(crate) const MC_BLANK_TEXT: u32 = 0x004a_3968;

/// `this->vt[slot](this + delta, a1, ...)` with `this` in register `this_reg`
/// (`v1` = the vtable, `a0` = the adjusted `this`, `v0` = the method; the
/// caller sets `a1`), returning to `ret`.
pub(crate) fn mem_card_vcall(c: &mut Cpu, this_reg: usize, slot: u32, ret: u32) -> Flow {
    let this = c.g[this_reg][0];
    c.g[V1][0] = Addr(this as u32).lw(c);
    let vt = c.g[V1][0];
    let m = VSlot::read(c, vt, slot);
    c.g[A0][0] = m.this(this);
    c.g[V0][0] = m.func;
    call_ptr(c, m.func as u32, ret)
}

/// The game object pointer at `gp - 0x848`.
pub(crate) fn mem_card_game(c: &mut Cpu) -> u64 {
    Addr(r32(c, GP).wrapping_sub(MC_GAME_GP_OFF)).lw(c)
}

/// Look up the localized string for the key in `a0` in the string table in
/// `s0`: `s1` = its vtable, `s0` += the slot's delta, `StrHash(key)` (returns
/// to `hashed`), then the table's find method (returns to `found`) with the
/// hash. `v0` = the UTF-16 string.
pub(crate) fn mem_card_localize(c: &mut Cpu, hashed: u32, found: u32) -> Flow {
    c.g[S1][0] = Addr(r32(c, S0)).off(4).lw(c);
    c.g[V0][0] = Addr(r32(c, S1)).off(MC_STRINGS_FIND).lh(c);
    c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
    call_ptr(c, callee::STR_HASH, hashed)?;
    c.g[V1][0] = Addr(r32(c, S1)).off(MC_STRINGS_FIND + 4).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    let find = r32(c, V1);
    call_ptr(c, find, found)
}

// ---------------------------------------------------------------------------
// cPS2MemCard_v17 (0x2c64f8)

/// Event check head of `cPS2MemCard_v17`'s slot scan.
pub(crate) const NEWEST_SAVE_HEAD: u32 = 0x002c_6568;

/// "No slot" (-1) in `cPS2MemCard_v17`.
pub(crate) const NEWEST_SAVE_NONE: u64 = u64::MAX;

/// How one slot of `cPS2MemCard_v17`'s scan ends: the index (`s2`) advances
/// in the branch delay slot, or in the 1-cycle join block at 0x2c666c.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum NewestSaveStep {
    Next,
    Join,
}

/// `int cPS2MemCard_v17(cPS2MemCard* self)` (0x2c64f8): the save slot with the
/// latest timestamp among the slots whose three flag words are clear and that
/// have a name (`HasSlotName`), or -1 (also when the current port's card is
/// not formatted, `v55`). Dates compare by year, month, day, then
/// `hours * 3600 + minutes * 60`, both sides adding the best slot's seconds.
pub fn c_ps2_mem_card_v17(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        c.g[S3][0] = NEWEST_SAVE_NONE;
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        mem_card_vcall(c, S1, MCU_VT_V55, 0x002c_6540)?;
        let formatted = c.g[V0][0] != 0;
        c.g[S2][0] = 0;
        if !formatted {
            c.cycles += 2;
            c.g[V0][0] = NEWEST_SAVE_NONE;
        } else {
            // fp = slot size, s7/s6 = seconds per hour/minute, s0 = self +
            // i * 0x78, s4 = slot i's entry, s5 = i * 0x78, s2 = i, s3 = best.
            c.cycles += 6;
            c.g[FP][0] = MCU_ENTRY_SIZE as u64;
            c.g[S7][0] = 3600;
            c.g[S6][0] = 60;
            c.g[S0][0] = c.g[S1][0];
            c.g[S4][0] = sx(r32(c, S1).wrapping_add(MCU_ENTRIES));
            c.g[S5][0] = 0;
            loop {
                if newest_save_visit(c)? == NewestSaveStep::Join {
                    c.cycles += 1;
                }
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.cycles += 5;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(MCU_ENTRY_SIZE));
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(MCU_ENTRY_SIZE));
                c.g[V0][0] = ((c.g[S2][0] as i64) < MCU_ENTRY_COUNT as i64) as u64;
                c.g[S5][0] = sx(r32(c, S5).wrapping_add(MCU_ENTRY_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, NEWEST_SAVE_HEAD);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Slot `s2` of `cPS2MemCard_v17`'s scan: skip it when a flag word is set or
/// it has no name; else it becomes the best slot (`s3`) if it is the first
/// candidate or later than the best one.
pub(crate) fn newest_save_visit(c: &mut Cpu) -> Result<NewestSaveStep, Unwound> {
    use NewestSaveStep::{Join, Next};
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S4)).off(MCU_ENTRY_USED).lw(c);
    if c.g[V0][0] != 0 {
        return Ok(Next);
    }
    c.cycles += 4;
    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S5)));
    c.g[V1][0] = Addr(r32(c, V0)).off(MCU_ENTRIES + MCU_ENTRY_INVALID).lw(c);
    if c.g[V1][0] != 0 {
        return Ok(Next);
    }
    c.cycles += 8;
    c.g[A1][0] = c.g[S2][0];
    mem_card_vcall(c, S1, MCU_VT_HAS_SLOT_NAME, 0x002c_659c)?;
    let named = c.g[V0][0] != 0;
    c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S5)));
    if !named {
        return Ok(Join);
    }
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, V0)).off(MCU_ENTRIES + MCU_ENTRY_NO_SPACE).lw(c);
    if c.g[V1][0] != 0 {
        return Ok(Next);
    }
    c.cycles += 3;
    c.g[V0][0] = NEWEST_SAVE_NONE;
    let first = c.g[S3][0] == c.g[V0][0];
    c.mult(0, V0, S3, FP); // v0 = best * slot size
    if first {
        c.cycles += 1;
        c.g[S3][0] = c.g[S2][0];
        return Ok(Join);
    }
    c.cycles += 22;
    newest_save_compare(c)
}

/// `cPS2MemCard_v17`: compare slot `s2`'s timestamp (entry at `s0`) with the
/// best slot's (`v0` = its offset) and take the later one.
///
/// Registers as the original leaves them: `t2`/`t0` the years, `a0`/`t1` the
/// months, `t4`/`t3` the days, `a2`/`v1` the times (candidate/best), `a3` the
/// year test, `a1` the best slot's seconds.
pub(crate) fn newest_save_compare(c: &mut Cpu) -> Result<NewestSaveStep, Unwound> {
    use NewestSaveStep::{Join, Next};
    let cand = r32(c, S0).wrapping_add(MCU_ENTRIES);
    let best = r32(c, V0)
        .wrapping_add(r32(c, S1))
        .wrapping_add(MCU_ENTRIES);
    c.g[A2][0] = Addr(cand).off(MCU_ENTRY_HOUR).lbu(c);
    c.g[A1][0] = Addr(cand).off(MCU_ENTRY_MIN).lbu(c);
    c.mult(0, A2, A2, S7); // hours * 3600
    c.g[T2][0] = Addr(cand).off(MCU_ENTRY_YEAR).lhu(c);
    c.mult(0, A1, A1, S6); // minutes * 60
    c.g[T4][0] = Addr(cand).off(MCU_ENTRY_DAY).lbu(c);
    c.g[V0][0] = Addr(best).off(MCU_ENTRY_HOUR).lbu(c);
    c.g[V1][0] = Addr(best).off(MCU_ENTRY_MIN).lbu(c);
    c.mult(0, V0, V0, S7);
    c.g[A2][0] = addu(c.g[A2][0], c.g[A1][0]);
    c.mult(0, V1, V1, S6);
    c.g[A1][0] = Addr(best).off(MCU_ENTRY_SEC).lbu(c);
    c.g[T0][0] = Addr(best).off(MCU_ENTRY_YEAR).lhu(c);
    c.g[T3][0] = Addr(best).off(MCU_ENTRY_DAY).lbu(c);
    c.g[A2][0] = addu(c.g[A2][0], c.g[A1][0]);
    c.g[T1][0] = Addr(best).off(MCU_ENTRY_MONTH).lbu(c);
    c.g[A3][0] = (c.g[T2][0] < c.g[T0][0]) as u64;
    c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
    c.g[A0][0] = Addr(cand).off(MCU_ENTRY_MONTH).lbu(c);
    c.g[V1][0] = addu(c.g[V0][0], c.g[A1][0]);
    // `rs < rt` into v0, 3 cycles per test after the first.
    let less = |c: &mut Cpu, rs: usize, rt: usize| {
        c.cycles += 3;
        c.g[V0][0] = (c.g[rs][0] < c.g[rt][0]) as u64;
        c.g[V0][0] != 0
    };
    // The candidate becomes the best slot.
    let take = |c: &mut Cpu| -> Result<NewestSaveStep, Unwound> {
        c.g[S3][0] = c.g[S2][0];
        Ok(Join)
    };
    if c.g[A3][0] != 0 {
        return Ok(Join); // older year
    }
    if less(c, T0, T2) {
        return take(c); // newer year
    }
    if less(c, A0, T1) {
        return Ok(Next); // older month
    }
    if less(c, T1, A0) {
        return take(c); // newer month
    }
    if less(c, T4, T3) {
        return Ok(Next); // older day
    }
    if less(c, T3, T4) {
        return take(c); // newer day
    }
    if less(c, A2, V1) {
        return Ok(Next); // earlier time
    }
    if less(c, V1, A2) {
        return take(c); // later time
    }
    Ok(Join) // the same time
}

// ---------------------------------------------------------------------------
// cPS2MemCard_GetStatusString (0x2c6128)

/// `cPS2MemCard_GetStatusString`'s stack buffer: 64 UTF-16 units at `sp`;
/// the unit at +0x40 (the 33rd) is cleared to cut the text to 32 units.
pub(crate) const STATUS_STRING_CUT: u32 = 0x40;

/// `void cPS2MemCard_GetStatusString(cPS2MemCard* self, int slot, u16* dst)`
/// (0x2c6128): the status text of save slot `slot` into `dst`, at most 32
/// UTF-16 units: "kT_MEMEmpty" when the current port's `v53` is false or the
/// slot has no name, "kT_MEMUnavailable" when the card is not formatted
/// (`v55`), "kT_MEMFileCorruptPS2" when the slot's used or invalid word is
/// set, "kT_MEMNoFreeSpace" for its no-space word (all through the game's
/// string table), else the slot's own text (entry +0x52, ASCII).
pub fn c_ps2_mem_card_get_status_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S2, 0xa0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0xb0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0x80);
        mem_card_vcall(c, S0, MCU_VT_V53, 0x002c_6160)?;
        // The string-table key of the message, or None for the slot's text
        // (`a1` = the slot's entry offset then). `v0` = the game object.
        let key = 'key: {
            let card_ok = c.g[V0][0] != 0;
            c.g[V0][0] = mem_card_game(c);
            if !card_ok {
                c.cycles += 2;
                break 'key Some(MC_KEY_EMPTY);
            }
            c.cycles += 7;
            mem_card_vcall(c, S0, MCU_VT_V55, 0x002c_617c)?;
            let formatted = c.g[V0][0] != 0;
            c.g[V0][0] = MCU_ENTRY_SIZE as u64;
            if !formatted {
                c.cycles += 4;
                c.g[V0][0] = mem_card_game(c);
                break 'key Some(MC_KEY_UNAVAILABLE);
            }
            c.cycles += 5;
            c.mult(0, S1, S2, V0); // s1 = slot * 0x78
            c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, S1)));
            c.g[V0][0] = Addr(r32(c, V1)).off(MCU_ENTRIES + MCU_ENTRY_USED).lw(c);
            let used = c.g[V0][0] != 0;
            c.g[V0][0] = mem_card_game(c);
            if !used {
                c.cycles += 4;
                c.g[V0][0] = c.g[V1][0];
                c.g[V1][0] = Addr(r32(c, V0)).off(MCU_ENTRIES + MCU_ENTRY_INVALID).lw(c);
                let invalid = c.g[V1][0] != 0;
                c.g[V0][0] = mem_card_game(c);
                if !invalid {
                    c.cycles += 4;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S1)));
                    c.g[V1][0] = Addr(r32(c, V0)).off(MCU_ENTRIES + MCU_ENTRY_NO_SPACE).lw(c);
                    let full = c.g[V1][0] != 0;
                    c.g[V0][0] = mem_card_game(c);
                    if full {
                        c.cycles += 3;
                        break 'key Some(MC_KEY_NO_FREE_SPACE);
                    }
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S2][0];
                    mem_card_vcall(c, S0, MCU_VT_HAS_SLOT_NAME, 0x002c_61f8)?;
                    let named = c.g[V0][0] != 0;
                    c.g[A1][0] = sx(r32(c, S1).wrapping_add(r32(c, S0)));
                    if named {
                        break 'key None;
                    }
                    // 1 cycle here, then the 2-cycle "empty" block.
                    c.cycles += 1 + 2;
                    c.g[V0][0] = mem_card_game(c);
                    break 'key Some(MC_KEY_EMPTY);
                }
            }
            c.cycles += 3;
            Some(MC_KEY_FILE_CORRUPT)
        };
        match key {
            Some(key) => {
                c.g[A0][0] = sx(key);
                c.cycles += 14;
                c.g[S0][0] = Addr(r32(c, V0)).off(MC_GAME_STRINGS).lw(c);
                mem_card_localize(c, 0x002c_6220, 0x002c_6230)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                callee::USTR_COPY.call(c, 0x002c_623c)?;
            }
            None => {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(MCU_ENTRIES + MCU_ENTRY_TITLE));
                callee::USTR_FROM_ASCII_2.call(c, 0x002c_6250)?;
            }
        }
        c.g[A0][0] = c.g[S3][0];
        c.cycles += 10;
        Addr(sp(c)).off(STATUS_STRING_CUT).sh(c, 0);
        c.g[A1][0] = c.g[SP][0];
        callee::USTR_COPY.call(c, 0x002c_6260)?;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2MemCard_Init (0x2c4050)

/// Event check head of `cPS2MemCard_Init`'s port-record loop.
pub(crate) const MC_INIT_PORTS_HEAD: u32 = 0x002c_4128;

/// `void cPS2MemCard_Init(cPS2MemCard* self)` (0x2c4050): reset the card
/// object: state, ports, request words and the `sceMc` result globals to 0,
/// the handle, result and slot to -1, the slot count to 6, then clear the
/// name and path buffers, both port records, the directory entry and table,
/// the save slots and the cached name.
pub fn c_ps2_mem_card_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x1c;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let card = r32(c, S0);
        let gp = r32(c, GP);
        let (word_8, none) = (c.g[V0][0], c.g[V1][0]);
        let zero = |c: &mut Cpu, at: u32| Addr(at).sw(c, 0);
        Addr(card).off(MCU_TITLE_BREAK).sw(c, word_8);
        zero(c, card.wrapping_add(MCU_STATE));
        zero(c, card.wrapping_add(MCU_PORT));
        zero(c, card.wrapping_add(MCU_PORT2));
        zero(c, card.wrapping_add(MCU_BUSY));
        zero(c, card.wrapping_add(0x4c)); // unknown word +0x4c
        zero(c, gp.wrapping_add(MCU_GP_TYPE));
        zero(c, gp.wrapping_add(MCU_GP_FREE));
        zero(c, gp.wrapping_add(MCU_GP_RESULT));
        Addr(card).off(MCU_HANDLE).sw(c, none);
        c.g[V0][0] = MCU_ENTRY_COUNT;
        Addr(card).off(MCU_COUNT).sw(c, MCU_ENTRY_COUNT);
        Addr(card).off(MCU_RESULT).sw(c, none);
        Addr(card).off(MCU_SLOT).sw(c, none);
        for field in [
            MCU_DIR_COUNT,
            MCU_LIST_RESULT,
            MCU_SIZE,
            MCU_EXPECT,
            0x2c, // the read/write buffer
            MCU_ASYNC_A,
            MCU_ASYNC_B,
            MCU_REQ_1E,
            MCU_REQ_18,
        ] {
            zero(c, card.wrapping_add(field));
        }
        zero(c, gp.wrapping_add(MCU_GP_FORMAT));
        zero(c, card.wrapping_add(MCU_INDEX));
        zero(c, card.wrapping_add(0x14e0)); // the open mode
        zero(c, card.wrapping_add(0x14e8)); // the seek origin
        zero(c, card.wrapping_add(0x14e4)); // the seek offset
        zero(c, card.wrapping_add(MCU_ACTIVE));
        for (field, ret) in [
            (MCU_DIR, 0x002c_40ec),
            (MCU_FILE, 0x002c_40fc),
            (MCU_PATH, 0x002c_410c),
            (MCU_LIST_NAME, 0x002c_411c),
        ] {
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(field));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x40;
            call_ptr(c, MCU_MEM_FILL, ret)?;
        }
        // Both port records (five words each); v0 walks the last word.
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(MCU_PORTS + 0x10));
        c.g[V1][0] = 1;
        loop {
            c.cycles += 8;
            let record = r32(c, V0).wrapping_sub(0x10);
            for word in 0..5 {
                zero(c, record.wrapping_add(4 * word));
            }
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(MCU_PORT_INFO_SIZE));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, MC_INIT_PORTS_HEAD);
            }
        }
        c.cycles += 20;
        for (field, len, ret) in [
            (MCU_DIR_ENTRY, 0x40, 0x002c_4158),
            (
                MCU_ENTRIES,
                MCU_ENTRY_SIZE * MCU_ENTRY_COUNT as u32,
                0x002c_4168,
            ),
            (MCU_DIR_TABLE, 0x1000, 0x002c_4178),
            (MCU_ICON_DIR, 0x20, 0x002c_4188),
        ] {
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(field));
            c.g[A1][0] = 0;
            c.g[A2][0] = len as u64;
            call_ptr(c, MCU_MEM_FILL, ret)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cPS2MemCard_GetFreeSpaceString (0x2c53e0)

/// `void cPS2MemCard_GetFreeSpaceString(cPS2MemCard* self, int port, u16* dst)`
/// (0x2c53e0): the free-space text of card port `port` into `dst`: a blank
/// (" ") unless the port's card is formatted and its status is 0; then
/// "kT_MEMFreePS2" from the game's string table formatted with
/// `GetFreeSpaceForPort(port)` (a negative value shows as 0).
pub fn c_ps2_mem_card_get_free_space_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = MCU_PORT_INFO_SIZE as u64;
        let frame = Frame::open(c, 0x50);
        c.mult(0, A3, A1, V0); // a3 = port * 0x14
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
        c.g[V1][0] = Addr(r32(c, V0))
            .off(MCU_PORTS + MCU_PORT_INFO_FORMATTED)
            .lw(c);
        let formatted = c.g[V1][0] != 0;
        c.g[S3][0] = c.g[A2][0];
        // The blank text's call site (its block's charge and return
        // address), or None for the free-space text.
        let blank = if !formatted {
            c.cycles += 1;
            Some((5, 0x002c_5444))
        } else {
            c.cycles += 5;
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
            c.g[V1][0] = sx(MC_STATUS_NO_FORMAT);
            c.g[V0][0] = Addr(r32(c, V0)).off(MCU_PORTS + MCU_PORT_INFO_STATUS).lw(c);
            if c.g[V0][0] == c.g[V1][0] {
                Some((5, 0x002c_5444))
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    None
                } else {
                    c.cycles += 2;
                    Some((4, 0x002c_54b4))
                }
            }
        };
        match blank {
            Some((charge, ret)) => {
                c.cycles += charge;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = sx(MC_BLANK_TEXT);
                callee::USTR_FROM_ASCII_2.call(c, ret)?;
            }
            None => {
                c.cycles += 23;
                c.g[S2][0] = 0;
                callee::C_PS2_MEM_CARD_GET_FREE_SPACE_FOR_PORT.call(c, 0x002c_5454)?;
                c.g[A1][0] = mem_card_game(c);
                c.g[V1][0] = (-1i64 < c.g[V0][0] as i64) as u64;
                c.g[A0][0] = sx(MC_KEY_FREE);
                c.g[S0][0] = Addr(r32(c, A1)).off(MC_GAME_STRINGS).lw(c);
                if c.g[V1][0] != 0 {
                    c.g[S2][0] = c.g[V0][0];
                }
                mem_card_localize(c, 0x002c_5480, 0x002c_5490)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                callee::U_STR_SPRINTF.call(c, 0x002c_54a0)?;
            }
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

// ---------------------------------------------------------------------------
// cPS2MemCard_v54 (0x2d3840)

/// `bool cPS2MemCard_v54(cPS2MemCard* self, int port)` (0x2d3840): whether
/// the card in `port` is usable: not a non-PS2 card (`IsPortNotPS2Card`),
/// `v50` false, formatted (`IsPortFormatted`), and `v52` true (all virtual).
pub fn c_ps2_mem_card_v54(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        'done: {
            mem_card_vcall(c, S0, MCU_VT_IS_PORT_NOT_PS2_CARD, 0x002d_3874)?;
            let not_ps2 = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S2][0];
            if not_ps2 {
                break 'done;
            }
            c.cycles += 8;
            c.g[A1][0] = c.g[S1][0];
            mem_card_vcall(c, S0, MCU_VT_V50, 0x002d_3894)?;
            let v50 = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S2][0];
            if v50 {
                break 'done;
            }
            c.cycles += 8;
            c.g[A1][0] = c.g[S1][0];
            mem_card_vcall(c, S0, MCU_VT_IS_PORT_FORMATTED, 0x002d_38b4)?;
            let formatted = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if formatted {
                c.cycles += 6;
                mem_card_vcall(c, S0, MCU_VT_V52, 0x002d_38d0)?;
                c.g[S2][0] = (0 < c.g[V0][0]) as u64;
            }
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

// ---------------------------------------------------------------------------
// cPS2MemCard_DoDeleteSave (0x2c4410)

/// `void cPS2MemCard_DoDeleteSave(cPS2MemCard* self)` (0x2c4410): start
/// deleting the save in slot `+0x1c`: clear the active word, copy the slot's
/// directory name to `+0xb9`, build the file path `/name/name` at `+0xf9` and
/// enter state 0x36 (`SetState`), the delete request for that path.
pub fn c_ps2_mem_card_do_delete_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = MCU_ENTRY_SIZE as u64;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let card = r32(c, S0);
        c.g[S1][0] = sx(card.wrapping_add(MCU_DIR));
        Addr(card).off(MCU_ACTIVE).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(card).off(MCU_SLOT).lw(c);
        c.mult(0, V1, A1, V0); // v1 = slot * 0x78
        c.g[A1][0] = sx(r32(c, V1)
            .wrapping_add(r32(c, S0))
            .wrapping_add(MCU_ENTRIES + MCU_ENTRY_NAME));
        call_ptr(c, MCU_STRCPY, 0x002c_4448)?;
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(MCU_FILE));
        c.g[A3][0] = c.g[A2][0];
        c.g[A1][0] = sx(MCU_PATH_FMT);
        call_ptr(c, MCU_SPRINTF, 0x002c_4460)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = MC_STATE_DELETE_FILE;
        callee::MEM_CARD_SET_STATE.call(c, 0x002c_446c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cPS2MemCard` field: the file offset of a seek request.
pub(crate) const MCU_SEEK_OFFSET: u32 = 0x14e4;

/// `cPS2MemCard` field: the origin (`sceMcSeek` mode) of a seek request.
pub(crate) const MCU_SEEK_ORIGIN: u32 = 0x14e8;

/// State 0x11: format the card if it is unformatted (`DoFormat`).
pub(crate) const MC_STATE_FORMAT: u64 = 0x11;

/// State 0x13: delete the save in slot `+0x1c` (`DoDeleteSave`).
pub(crate) const MC_STATE_DELETE_SAVE: u64 = 0x13;

/// State 0x22: read the directory entry of the save in slot `+0x1c`
/// (`DoGetSaveDirEntry`).
pub(crate) const MC_STATE_GET_SAVE_DIR_ENTRY: u64 = 0x22;

/// State 0x41: seek the open file (`DoSeek`).
pub(crate) const MC_STATE_SEEK: u64 = 0x41;

/// State 0x43: read the open file (`DoRead`).
pub(crate) const MC_STATE_READ: u64 = 0x43;

/// State 0x45: write the open file (`DoWrite`).
pub(crate) const MC_STATE_WRITE: u64 = 0x45;

/// State 0x47: flush the open file (`DoFlush`).
pub(crate) const MC_STATE_FLUSH: u64 = 0x47;

/// State 0x49: close the open file (`DoClose`).
pub(crate) const MC_STATE_CLOSE: u64 = 0x49;

/// The shape of the `cPS2MemCard` request wrappers: charge `charge`, open a
/// 0x10 frame holding `ra`, run the wrapper's own field stores (`fields`),
/// then `cPS2MemCard_SetState(this, state)` returning to `ret`, and return.
/// `this` stays in `a0` throughout.
pub(crate) fn mem_card_request(
    c: &mut Cpu,
    charge: u64,
    fields: impl FnOnce(&mut Cpu),
    state: u64,
    ret: u32,
) {
    guest(c, |c| {
        c.cycles += charge;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        fields(c);
        c.g[A1][0] = state;
        callee::MEM_CARD_SET_STATE.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The field stores of the two-argument requests (10 cycles): `v0 = this`,
/// then `this[first] = first_reg` and `this[second] = second_reg` (words), in
/// that order.
pub(crate) fn mem_card_request_store_pair(
    c: &mut Cpu,
    first: u32,
    first_reg: usize,
    second: u32,
    second_reg: usize,
) {
    let this = c.g[A0][0];
    c.g[V0][0] = this;
    let card = Addr(this as u32);
    let a = c.g[first_reg][0];
    card.off(first).sw(c, a);
    let b = c.g[second_reg][0];
    card.off(second).sw(c, b);
}

/// The field store of the slot requests (8 cycles): `this[+0x1c] = slot`.
pub(crate) fn mem_card_request_store_slot(c: &mut Cpu) {
    let slot = c.g[A1][0];
    Addr(c.g[A0][0] as u32).off(MCU_SLOT).sw(c, slot);
}

/// `void cPS2MemCard_RequestFormat(cPS2MemCard* this)` (0x2c4368): ask the
/// card state machine to format the current port's card (state 0x11: the
/// update runs `DoFormat` if the card is unformatted).
pub fn c_ps2_mem_card_request_format(c: &mut Cpu) {
    mem_card_request(c, 7, |_| {}, MC_STATE_FORMAT, 0x002c_4378);
}

/// `void cPS2MemCard_v32(cPS2MemCard* this)` (0x2c48a0): finish a save-file
/// read (state 0x1e: close the open file, or go on to 0x1f if none is open).
pub fn c_ps2_mem_card_v32(c: &mut Cpu) {
    mem_card_request(c, 7, |_| {}, MC_STATE_CLOSE_AFTER_READ, 0x002c_48b0);
}

/// `void cPS2MemCard_v43(cPS2MemCard* this)` (0x2c4768): finish a save-file
/// write (state 0x18: close the open file, or go on to 0x19 if none is open).
pub fn c_ps2_mem_card_v43(c: &mut Cpu) {
    mem_card_request(c, 7, |_| {}, MC_STATE_CLOSE_AFTER_WRITE, 0x002c_4778);
}

/// `void cPS2MemCard_RequestClose(cPS2MemCard* this)` (0x2c6a08): request
/// closing the open file (state 0x49: the update runs `DoClose`).
pub fn c_ps2_mem_card_request_close(c: &mut Cpu) {
    mem_card_request(c, 7, |_| {}, MC_STATE_CLOSE, 0x002c_6a18);
}

/// `void cPS2MemCard_RequestFlush(cPS2MemCard* this)` (0x2c6ae0): request
/// flushing the open file (state 0x47: the update runs `DoFlush`).
pub fn c_ps2_mem_card_request_flush(c: &mut Cpu) {
    mem_card_request(c, 7, |_| {}, MC_STATE_FLUSH, 0x002c_6af0);
}

/// `void cPS2MemCard_v76(cPS2MemCard* this, int slot)` (0x2c4290): store
/// `slot` at `+0x1c` and request that save's directory entry (state 0x22:
/// the update runs `DoGetSaveDirEntry`).
pub fn c_ps2_mem_card_v76(c: &mut Cpu) {
    mem_card_request(
        c,
        8,
        mem_card_request_store_slot,
        MC_STATE_GET_SAVE_DIR_ENTRY,
        0x002c_42a4,
    );
}

/// `void cPS2MemCard_v10(cPS2MemCard* this, int slot)` (0x2c43e0): store
/// `slot` at `+0x1c` and request deleting that save (state 0x13: the update
/// runs `DoDeleteSave`).
pub fn c_ps2_mem_card_v10(c: &mut Cpu) {
    mem_card_request(
        c,
        8,
        mem_card_request_store_slot,
        MC_STATE_DELETE_SAVE,
        0x002c_43f4,
    );
}

/// `void* cPS2MemCard_RequestWrite(cPS2MemCard* this, void* buffer, int size)`
/// (0x2c6bd0): store the buffer (`+0x2c`) and size (`+0x24`) and request
/// writing them to the open file (state 0x45: the update runs `DoWrite`).
/// Leaves `this` in `v0` until the call.
pub fn c_ps2_mem_card_request_write(c: &mut Cpu) {
    mem_card_request(
        c,
        10,
        |c| mem_card_request_store_pair(c, MCU_BUFFER, A1, MCU_EXPECT, A2),
        MC_STATE_WRITE,
        0x002c_6bec,
    );
}

/// `void* cPS2MemCard_RequestRead(cPS2MemCard* this, void* buffer, int size)`
/// (0x2c6b50): store the buffer (`+0x2c`) and size (`+0x24`) and request
/// reading the open file into it (state 0x43: the update runs `DoRead`).
pub fn c_ps2_mem_card_request_read(c: &mut Cpu) {
    mem_card_request(
        c,
        10,
        |c| mem_card_request_store_pair(c, MCU_BUFFER, A1, MCU_EXPECT, A2),
        MC_STATE_READ,
        0x002c_6b6c,
    );
}

/// `void* cPS2MemCard_RequestSeek(cPS2MemCard* this, int offset, int origin)`
/// (0x2c6a60): store the offset (`+0x14e4`) and origin (`+0x14e8`) and
/// request seeking the open file (state 0x41: the update runs `DoSeek`).
pub fn c_ps2_mem_card_request_seek(c: &mut Cpu) {
    mem_card_request(
        c,
        10,
        |c| mem_card_request_store_pair(c, MCU_SEEK_OFFSET, A1, MCU_SEEK_ORIGIN, A2),
        MC_STATE_SEEK,
        0x002c_6a7c,
    );
}

/// `void* cPS2MemCard_RequestGetInfoNow(cPS2MemCard* this, int port)`
/// (0x2c67e8): make `port` the current port (`+0x10`, then `+0xc`) and
/// refresh its card info at once (state 3: `SetState` runs `DoGetInfo`).
pub fn c_ps2_mem_card_request_get_info_now(c: &mut Cpu) {
    mem_card_request(
        c,
        10,
        |c| mem_card_request_store_pair(c, MCU_PORT2, A1, MCU_PORT, A1),
        MC_STATE_GET_INFO_NOW,
        0x002c_6804,
    );
}

/// `void* cPS2MemCard_v30(cPS2MemCard* this, void* buffer, int size)`
/// (0x2c4858): store the buffer (`+0x2c`) and size (`+0x20`) and read the
/// open save file into it (state 0x1d: `SetState` starts `sceMcRead`; the
/// update then checks the size read).
pub fn c_ps2_mem_card_v30(c: &mut Cpu) {
    mem_card_request(
        c,
        10,
        |c| mem_card_request_store_pair(c, MCU_BUFFER, A1, MCU_SIZE, A2),
        MC_STATE_READ_SAVE_FILE,
        0x002c_4874,
    );
}

/// `void* cPS2MemCard_v41(cPS2MemCard* this, void* buffer, int size)`
/// (0x2c4720): store the buffer (`+0x2c`) and size (`+0x20`) and write it to
/// the open save file (state 0x17: `SetState` starts `sceMcWrite`).
pub fn c_ps2_mem_card_v41(c: &mut Cpu) {
    mem_card_request(
        c,
        10,
        |c| mem_card_request_store_pair(c, MCU_BUFFER, A1, MCU_SIZE, A2),
        MC_STATE_WRITE_SAVE_FILE,
        0x002c_473c,
    );
}

// ---------------------------------------------------------------------------
// cPS2MemCard vtable slots the current-port thunks call (gcc 2.9x entries:
// `{ short delta; short; void (*fn)(); }`, 8 bytes each).

/// Slot 22: `cPS2MemCard_CalcSaveSizeKB(bytes)`.
pub(crate) const MCU_VT_CALC_SAVE_SIZE_KB: u32 = 0xb0;

/// Slot 61: `cPS2MemCard_v61(port)` (cPS2MemCard's own returns 1).
pub(crate) const MCU_VT_V61: u32 = 0x1e8;

/// Slot 70: `cPS2MemCard_GetSlotName(a1, a2, a3)`.
pub(crate) const MCU_VT_GET_SLOT_NAME: u32 = 0x230;

/// The memory-card base class's vtable (every method slot is
/// `__pure_virtual`), put back by the base destructor.
pub(crate) const MEM_CARD_BASE_VTABLE: u32 = 0x0048_71e8;

/// The shape of one current-port vtable thunk: what differs between them.
#[derive(Clone, Copy)]
pub(crate) struct MemCardVThunk {
    /// The block's cycle charge.
    pub(crate) cycles: u64,
    /// The `this` word passed in `a1` (`None`: the caller's `a1..a3` pass through).
    pub(crate) field: Option<u32>,
    /// The vtable slot called.
    pub(crate) slot: u32,
    /// Registers holding the vtable, the slot's `this` adjustment and its function.
    pub(crate) vt: usize,
    pub(crate) delta: usize,
    pub(crate) func: usize,
    /// The call's return address.
    pub(crate) ret: u32,
}

/// `this->vt[slot](this + delta, field)` in a 0x10 frame holding `ra`; returns
/// what the method returned.
pub(crate) fn mem_card_vthunk(c: &mut Cpu, thunk: MemCardVThunk) {
    guest(c, |c| {
        c.cycles += thunk.cycles;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        let vt = Addr(this).lw(c);
        c.g[thunk.vt][0] = vt;
        if let Some(off) = thunk.field {
            c.g[A1][0] = Addr(this).off(off).lw(c);
        }
        let entry = VSlot::read(c, vt, thunk.slot);
        c.g[thunk.delta][0] = entry.delta;
        c.g[thunk.func][0] = entry.func;
        c.g[A0][0] = entry.this(c.g[A0][0]);
        call_ptr(c, entry.func as u32, thunk.ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A thunk passing the current port (`+0xc`), registers `v1`/`a2`/`v0`.
pub(crate) const fn mem_card_port_vthunk(slot: u32, ret: u32) -> MemCardVThunk {
    MemCardVThunk {
        cycles: 11,
        field: Some(MCU_PORT),
        slot,
        vt: V1,
        delta: A2,
        func: V0,
        ret,
    }
}

/// `bool cPS2MemCard::v49()` (0x2d37e0): whether the card in the current port
/// is not a PS2 card (`this->IsPortNotPS2Card(this->port)`, virtual).
pub fn c_ps2_mem_card_v49(c: &mut Cpu) {
    mem_card_vthunk(
        c,
        mem_card_port_vthunk(MCU_VT_IS_PORT_NOT_PS2_CARD, 0x002d_3800),
    );
}

/// `int cPS2MemCard::v51()` (0x2c67b0): `this->v50(this->port)` (virtual) for
/// the current port.
pub fn c_ps2_mem_card_v51(c: &mut Cpu) {
    mem_card_vthunk(c, mem_card_port_vthunk(MCU_VT_V50, 0x002c_67d0));
}

/// `bool cPS2MemCard::v53()` (0x2d3810): `this->v52(this->port)` (virtual),
/// whether the current port's status word is a known card state.
pub fn c_ps2_mem_card_v53(c: &mut Cpu) {
    mem_card_vthunk(c, mem_card_port_vthunk(MCU_VT_V52, 0x002d_3830));
}

/// `bool cPS2MemCard::v55()` (0x2d38f0): whether the card in the current port
/// is formatted (`this->IsPortFormatted(this->port)`, virtual).
pub fn c_ps2_mem_card_v55(c: &mut Cpu) {
    mem_card_vthunk(
        c,
        mem_card_port_vthunk(MCU_VT_IS_PORT_FORMATTED, 0x002d_3910),
    );
}

/// `int cPS2MemCard::v23()` (0x2c6778): the card size in KB of the save whose
/// byte size is the word at `+0x24` (`this->CalcSaveSizeKB(..)`, virtual).
pub fn c_ps2_mem_card_v23(c: &mut Cpu) {
    mem_card_vthunk(
        c,
        MemCardVThunk {
            field: Some(MCU_EXPECT),
            ..mem_card_port_vthunk(MCU_VT_CALC_SAVE_SIZE_KB, 0x002c_6798)
        },
    );
}

/// `int cPS2MemCard::v62()` (0x2c6718): `this->v61(this->port)` (virtual) for
/// the current port.
pub fn c_ps2_mem_card_v62(c: &mut Cpu) {
    mem_card_vthunk(c, mem_card_port_vthunk(MCU_VT_V61, 0x002c_6738));
}

/// `cPS2MemCard::v71(a1, a2, a3)` (0x2d3980): forward to
/// `this->GetSlotName(a1, a2, a3)` (virtual), arguments unchanged.
pub fn c_ps2_mem_card_v71(c: &mut Cpu) {
    mem_card_vthunk(
        c,
        MemCardVThunk {
            cycles: 10,
            field: None,
            slot: MCU_VT_GET_SLOT_NAME,
            vt: T0,
            delta: V0,
            func: V1,
            ret: 0x002d_399c,
        },
    );
}

// ---------------------------------------------------------------------------
// Current-port getters: `f(this, this->port)`.

/// Open the 0x10 frame (`ra` at 0), charge `cycles`, pass the current port in
/// `a1` and call `target`; the caller closes the frame.
pub(crate) fn mem_card_call_with_port(
    c: &mut Cpu,
    cycles: u64,
    target: Callee,
    ret: u32,
) -> Result<Frame, Unwound> {
    c.cycles += cycles;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = Addr(r32(c, A0)).off(MCU_PORT).lw(c);
    target.call(c, ret)?;
    frame.restore_ra(c, 0);
    Ok(frame)
}

/// `bool cPS2MemCard::v63()` (0x2d3930): whether the current port's card
/// changed (`GetPortChangedFlag(this, this->port)`).
pub fn c_ps2_mem_card_v63(c: &mut Cpu) {
    guest(c, |c| {
        let frame = mem_card_call_with_port(
            c,
            7,
            callee::C_PS2_MEM_CARD_GET_PORT_CHANGED_FLAG,
            0x002d_3940,
        )?;
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard::v64(wchar_t* dst)` (0x2d3950): write the current port's
/// free-space label into `dst` (`GetFreeSpaceString(this, this->port, dst)`).
pub fn c_ps2_mem_card_v64(c: &mut Cpu) {
    guest(c, |c| {
        c.g[A2][0] = c.g[A1][0];
        let frame = mem_card_call_with_port(
            c,
            8,
            callee::C_PS2_MEM_CARD_GET_FREE_SPACE_STRING,
            0x002d_3964,
        )?;
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2MemCard::GetCurrentPortError()` (0x2d39b8): the current port's
/// error code (`GetPortError(this, this->port)`).
pub fn c_ps2_mem_card_get_current_port_error(c: &mut Cpu) {
    guest(c, |c| {
        let frame = mem_card_call_with_port(c, 7, MCU_GET_PORT_ERROR, 0x002d_39c8)?;
        frame.close(c);
        Ok(())
    })
}

/// `int cPS2MemCard::GetFreeSpace()` (0x2c5338): the current port's free space
/// (`GetFreeSpaceForPort(this, this->port)`).
pub fn c_ps2_mem_card_get_free_space(c: &mut Cpu) {
    guest(c, |c| {
        let frame = mem_card_call_with_port(
            c,
            7,
            callee::C_PS2_MEM_CARD_GET_FREE_SPACE_FOR_PORT,
            0x002c_5348,
        )?;
        frame.close(c);
        Ok(())
    })
}

/// `bool cPS2MemCard::v73()` (0x2c5230): whether the current port has a
/// non-zero error code (`GetPortError(this, this->port) != 0`).
pub fn c_ps2_mem_card_v73(c: &mut Cpu) {
    guest(c, |c| {
        let frame = mem_card_call_with_port(c, 8, MCU_GET_PORT_ERROR, 0x002c_5240)?;
        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Save names and the base destructor.

/// `char* cPS2MemCard::GetSaveName(int slot, char* dst)` (0x2c54d0): copy save
/// slot `slot`'s name into `dst` (returns `dst`, from `strcpy`).
pub fn c_ps2_mem_card_get_save_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[V0][0] = MCU_ENTRY_SIZE as u64;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0); // v1 = slot * 0x78
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        let name = r32(c, V1)
            .wrapping_add(this)
            .wrapping_add(MCU_ENTRIES + MCU_ENTRY_NAME);
        c.g[A0][0] = c.g[A2][0];
        c.g[A1][0] = sx(name);
        callee::STRCPY.call(c, 0x002c_54f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char* cPS2MemCard::v45(const wchar_t* title)` (0x2c5500): store `title` as
/// the ASCII save title at `+0x58` (`UStr_ToAscii`; returns the title buffer).
pub fn c_ps2_mem_card_v45(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(MCU_NAME));
        callee::USTR_TO_ASCII.call(c, 0x002c_5510)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cPS2MemCard_BaseDtor(cPS2MemCard* this, int flags)` (0x2d3788): the
/// base-class destructor: put the base vtable back and free the object when
/// `flags & 1`.
pub fn c_ps2_mem_card_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = sx(MEM_CARD_BASE_VTABLE);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let vt = c.g[V0][0];
        Addr(r32(c, A0)).sw(c, vt);
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            callee::BUILTIN_DELETE_2.call(c, 0x002d_37ac)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LocStringTable_ResolveString(LocStringTable* t)` (0x2c6dc0): a
/// forwarder. Calls `LocStringTable_GetStringPtr` (0x3dbb68) through the
/// function table with `a0 = t->table` (+4), in a 0x10 frame holding `ra`,
/// and returns to the caller at 0x2c6dd0 with that call's results.
pub fn loc_string_table_resolve_string(c: &mut Cpu) {
    crate::call::tail_wrapper(c, Some(0x4), Target::Table(0x003d_bb68), 0x002c_6dd0);
}

/// `void LocStringTable_Load(LocStringTable* t, const char* name, char lang)`
/// (0x2c6e98, frame 0x40): loads the locale file (`EAFile_LoadFileEx`), stores
/// the language (+0), file (+4), string count (`LocStringTable_GetInfo` 2,
/// +8), header (+0xc: file + the offset at file+4, zeroed when its 4-byte
/// magic does not match) and the table (+0x10, header + the offset at
/// header+4), then resolves every table entry's id to its string pointer
/// in place (`LocStringTable_ResolveString`, 8-byte entries).
pub fn loc_string_table_load(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c6f5c: {
            'b_2c6f08: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x002c6ec0);
                c.sb(c.g[S1][0] as u32, c.g[A2][0]);
                call_ptr(c, 0x003e18c8, 0x002c6ec0)?;
                c.g[A1][0] = 0x2;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002c6ed0);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                call_ptr(c, 0x003db9d8, 0x002c6ed0)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A1][0] = 0x4a0000;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3978));
                c.g[A2][0] = 0x4;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002c6ef8);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                call_ptr(c, 0x00416b18, 0x002c6ef8)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    break 'b_2c6f08;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0u64);
                    if t {
                        break 'b_2c6f5c;
                    } else {
                        break 'b_2c6f08;
                    }
                }
            }
            c.cycles += 6;
            c.g[S2][0] = 0u64;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let t = c.g[V1][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            if t {
                break 'b_2c6f5c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                'l_2c6f28: loop {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 3);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x002c6f40);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    Callee {
                        addr: 0x002c6dc0,
                        f: ee::s08::f_002c6dc0,
                    }
                    .call(c, 0x002c6f40)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        if event_due(c) {
                            deliver_event_here(c, 0x002c6f28);
                        }
                        continue 'l_2c6f28;
                    } else {
                        break 'b_2c6f5c;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `char* UStr_ToAscii(char* dst, const u16* src)` (0x2c2580): narrows a
/// zero-terminated UTF-16 string to bytes (low byte of each unit), writes the
/// terminator and returns `dst`.
pub fn ustr_to_ascii(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
    c.g[V1][0] = c.g[A0][0];
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        loop {
            c.cycles += 7;
            c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(2));
            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_2590);
            }
        }
    }
    c.cycles += 3;
    c.sb(c.g[A0][0] as u32, 0);
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[V1][0];
    c.pc = t;
}

/// `void QuadTree_Compact(QuadTree* self, void** out, int* size, ctx)` (0x2c1da8,
/// `cQuadTree_exportTree`): allocates the node map and the output block, flattens the
/// tree into it, renumbers each node's children and byte-swaps the header and nodes
/// through the context's fix functions. Transcription of the generated body (labels
/// kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn quad_tree_compact(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c1e54: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A1][0] = 0x480000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[T2][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S7][0] = c.g[T1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[FP][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5fb0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x002c1e1c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x002c1e1c)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.g[V0][0];
            if t {
                break 'b_2c1e54;
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = c.g[S4][0];
                'l_2c1e38: loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S2][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002c1e38);
                        }
                        continue 'l_2c1e38;
                    } else {
                        break 'b_2c1e54;
                    }
                }
            }
        }
        'b_2c1ff4: {
            c.cycles += 48;
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = 0x480000;
            c.g[A3][0] = 0u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5fc8));
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x002c1e70);
            c.g[S2][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x002c1e70)?;
            c.sw(c.g[S3][0] as u32, c.g[V0][0]);
            c.g[A1][0] = 0xffffffffdead0000;
            c.g[A1][0] = c.g[A1][0] | 0xbeef;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002c1e88);
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            call_ptr(c, 0x00416210, 0x002c1e88)?;
            c.g[S3][0] = c.lw(c.g[S3][0] as u32);
            c.g[A0][0] = c.g[S6][0];
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = c.g[S4][0];
            c.ldl(2, (c.g[S6][0] as u32).wrapping_add(0x7));
            c.ldr(2, c.g[S6][0] as u32);
            c.ldl(3, (c.g[S6][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[S6][0] as u32).wrapping_add(0x8));
            c.ldl(5, (c.g[S6][0] as u32).wrapping_add(0x17));
            c.ldr(5, (c.g[S6][0] as u32).wrapping_add(0x10));
            c.ldl(8, (c.g[S6][0] as u32).wrapping_add(0x1f));
            c.ldr(8, (c.g[S6][0] as u32).wrapping_add(0x18));
            c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[S3][0] as u32);
            c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0xf));
            c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x8));
            c.sdl(5, (c.g[S3][0] as u32).wrapping_add(0x17));
            c.sdr(5, (c.g[S3][0] as u32).wrapping_add(0x10));
            c.sdl(8, (c.g[S3][0] as u32).wrapping_add(0x1f));
            c.sdr(8, (c.g[S3][0] as u32).wrapping_add(0x18));
            c.ldl(2, (c.g[S6][0] as u32).wrapping_add(0x27));
            c.ldr(2, (c.g[S6][0] as u32).wrapping_add(0x20));
            c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x27));
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
            c.sdr(2, (c.g[S3][0] as u32).wrapping_add(0x20));
            c.sw(c.g[SP][0] as u32, 0u64);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
            c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
            c.g[RA][0] = sx(0x002c1f08);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            Callee {
                addr: 0x002c2088,
                f: ee::s08::QuadTree_FlattenNode,
            }
            .call(c, 0x002c1f08)?;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            if (c.g[V0][0] as i64) <= 0 {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                break 'b_2c1ff4;
            } else {
                c.cycles += 1;
                'l_2c1f18: loop {
                    'b_2c1fdc: {
                        'b_2c1fbc: {
                            c.cycles += 4;
                            c.g[T1][0] = c.lhu(c.g[S0][0] as u32);
                            c.g[V0][0] = c.g[T1][0] & 0x1;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T1][0] = sx((c.g[T1][0] as u32) >> 1);
                            if t {
                                break 'b_2c1fbc;
                            } else {
                                c.cycles += 37;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[T1][0] = sx((c.g[T1][0] as u32) << 2);
                                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[T1][0] =
                                    sx((c.g[T1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                                c.g[A2][0] = c.lhu(c.g[A0][0] as u32);
                                c.g[A1][0] = c.lhu(c.g[T1][0] as u32);
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A3][0] = c.lhu(c.g[V0][0] as u32);
                                c.g[RA][0] = sx(0x002c1f78);
                                c.g[T0][0] = c.lhu(c.g[V1][0] as u32);
                                Callee {
                                    addr: 0x002c16b8,
                                    f: ee::s08::QuadTreeNode_SetChildren,
                                }
                                .call(c, 0x002c1f78)?;
                                c.g[A0][0] = c.lhu(c.g[S0][0] as u32);
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1f84);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, t, 0x002c1f84)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                                c.g[A1][0] = c.g[S1][0];
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1f94);
                                c.sh(c.g[S0][0] as u32, c.g[V0][0]);
                                call_ptr(c, t, 0x002c1f94)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = c.g[S1][0];
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1fa4);
                                c.sh((c.g[S0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                call_ptr(c, t, 0x002c1fa4)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
                                c.g[A1][0] = c.g[S1][0];
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1fb4);
                                c.sh((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                call_ptr(c, t, 0x002c1fb4)?;
                                let t = true;
                                c.sh((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                                if t {
                                    break 'b_2c1fdc;
                                } else {
                                    break 'b_2c1fbc;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        let t = c.g[S7][0] as u32;
                        c.g[RA][0] = sx(0x002c1fc8);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, t, 0x002c1fc8)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = c.g[S1][0];
                        let t = c.g[S7][0] as u32;
                        c.g[RA][0] = sx(0x002c1fd8);
                        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                        call_ptr(c, t, 0x002c1fd8)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        break 'b_2c1fdc;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002c1f18);
                        }
                        continue 'l_2c1f18;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                        break 'b_2c1ff4;
                    }
                }
            }
        }
        c.cycles += 37;
        c.g[A1][0] = c.g[S1][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(0x24), 0u64);
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2004);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1c));
        call_ptr(c, t, 0x002c2004)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2014);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x18));
        call_ptr(c, t, 0x002c2014)?;
        c.lwc1(12, c.g[S3][0] as u32);
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2024);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, t, 0x002c2024)?;
        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2034);
        c.swc1(0, c.g[S3][0] as u32);
        call_ptr(c, t, 0x002c2034)?;
        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2044);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        call_ptr(c, t, 0x002c2044)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2054);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
        call_ptr(c, t, 0x002c2054)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    })
}

/// `QuadTree_InsertRecursive` (0x2c1a00).
pub(crate) const QT_INSERT_RECURSIVE: Callee = Callee {
    addr: 0x002c_1a00,
    f: ee::s08::QuadTree_InsertRecursive,
};

/// `QuadTree_CountNodes` (0x2c2178), called on itself for the children.
pub(crate) const QT_COUNT_NODES: Callee = Callee {
    addr: 0x002c_2178,
    f: ee::s08::QuadTree_CountNodes,
};

/// The register `off` bytes above the value of `reg`, as an address.
#[inline(always)]
pub(crate) fn oc1052_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `QuadTree_AllocNode(tree)` (0x2c17d0): hands out the next free node slot
/// of the tree. When the free list is empty it grows the node array (a fresh
/// allocation, the old contents copied over, then the old array freed) and
/// links the new nodes into the free list. Returns the node pointer.
pub fn quad_tree_alloc_node(c: &mut Cpu) {
    guest(c, quad_tree_alloc_node_body);
}

pub(crate) fn quad_tree_alloc_node_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let f = Frame::open(c, 0x30);
    f.save(c, S0, 0x20);
    f.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    f.save_ra(c, 0);
    c.g[V1][0] = oc1052_at(c, S0, 0x24).lw(c);
    c.g[V0][0] = oc1052_at(c, S0, 0x20).lw(c);
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V0][0] = oc1052_at(c, S0, 0x24).lw(c);
    } else {
        // 0x2c1830: allocate the grown array (size = count*8 + 0x400 bytes, 8-byte units).
        c.cycles += 11;
        c.g[A0][0] = oc1052_at(c, S0, 0x10).lw(c);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x5f98));
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x400));
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        call_ptr(c, 0x0031_7e30, 0x002c_1814)?;
        c.g[A1][0] = oc1052_at(c, S0, 0x20).lw(c);
        let no_old = c.g[A1][0] == 0;
        c.g[S1][0] = c.g[V0][0];
        if !no_old {
            // 0x2c1830 (copy the old array over).
            c.cycles += 4;
            c.g[A2][0] = oc1052_at(c, S0, 0x10).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 3);
            call_ptr(c, 0x0041_605c, 0x002c_1830)?;
        }
        // 0x2c186c: fill the new slots of the free list.
        c.cycles += 5;
        c.g[V1][0] = oc1052_at(c, S0, 0x10).lw(c);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3ff));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let grow = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
        if grow {
            c.cycles += 3;
            c.g[A1][0] = c.g[A0][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                let link = c.g[V0][0];
                Addr(c.g[A0][0] as u32).sh(c, link);
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(8));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_1850);
                    }
                    continue;
                }
                break;
            }
        }
        // 0x2c189c: publish the new array and release the old one.
        c.cycles += 10;
        c.g[V0][0] = oc1052_at(c, S0, 0x10).lw(c);
        c.g[A1][0] = oc1052_at(c, S0, 0x20).lw(c);
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x400));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        Addr((c.g[V1][0] as u32).wrapping_add(0x1ff8)).sh(c, 0);
        let first_free = c.g[A0][0];
        oc1052_at(c, S0, 0x24).sw(c, first_free);
        let old = c.g[A1][0];
        let grown_count = c.g[V0][0];
        oc1052_at(c, S0, 0x10).sw(c, grown_count);
        if old != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[A1][0];
            call_ptr(c, 0x0031_7e98, 0x002c_189c)?;
        }
        c.cycles += 2;
        let s1 = c.g[S1][0];
        oc1052_at(c, S0, 0x20).sw(c, s1);
        c.g[V0][0] = oc1052_at(c, S0, 0x24).lw(c);
    }
    // 0x2c18a4 (epilogue of the allocation): hand out the node at the head.
    c.cycles += 16;
    c.g[A1][0] = oc1052_at(c, S0, 0x20).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    c.g[A0][0] = oc1052_at(c, S0, 0xc).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    let used = c.g[A0][0];
    oc1052_at(c, S0, 0xc).sw(c, used);
    c.g[V0][0] &= 0xffff;
    let next_free = c.g[A1][0];
    oc1052_at(c, S0, 0x24).sw(c, next_free);
    f.restore(c, S0, 0x20);
    f.restore(c, S1, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `QuadTree_Insert(node, x, y, ...)` (0x2c1950): descends the quad tree from
/// `node` towards the point `(x, y)` (the node's bounds are the three floats
/// at +0, +4, +8), and inserts into the child cell that holds it. Leaves
/// with a full cell are split through `QuadTree_InsertRecursive`.
pub fn quad_tree_insert(c: &mut Cpu) {
    guest(c, quad_tree_insert_body);
}

pub(crate) fn quad_tree_insert_body(c: &mut Cpu) -> Flow {
    c.cycles += 20;
    let f = Frame::open(c, 0x10);
    c.g[V1][0] = c.g[A0][0];
    f.save_ra(c, 0);
    c.g[T2][0] = c.g[A2][0];
    c.g[A0][0] = 0x7fff;
    let node = c.g[V1][0] as u32;
    c.lwc1(2, node.wrapping_add(4));
    c.lwc1(1, node.wrapping_add(8));
    c.sub_s(12, 12, 2);
    c.lwc1(0, node);
    c.sub_s(13, 13, 1);
    c.mul_s(14, 14, 0);
    c.mul_s(12, 12, 0);
    c.mul_s(13, 13, 0);
    c.cvt_w_s(0, 12);
    c.mfc1(6, 0);
    c.cvt_w_s(0, 13);
    c.mfc1(7, 0);
    c.g[V0][0] = (c.g[A0][0] < c.g[A2][0]) as u64;
    let beyond_limit = c.g[V0][0] != 0;
    c.g[T1][0] = c.g[A1][0];
    if !beyond_limit {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A0][0] < c.g[A3][0]) as u64;
        let leaf_done = c.g[V0][0] != 0;
        f.restore_ra(c, 0);
        if leaf_done {
            return quad_tree_insert_end(c, f);
        }
        // 0x2c19d4: the split point (a 16-bit quotient of the halved limits).
        c.cycles += 10;
        c.cvt_w_s(0, 14);
        c.mfc1(2, 0);
        c.g[T0][0] = 0x8000;
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 1);
        c.g[A0][0] = c.g[V1][0];
        c.g[A2][0] = c.g[A2][0] & 0xfffe;
        c.div(0, T0, V0);
        c.cycles += 7;
        c.g[A1][0] = Addr(node.wrapping_add(0x14)).lhu(c);
        c.g[A3][0] &= 0xfffe;
        c.g[V0][0] = oc1052_at(c, V1, 0x20).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[T0][0] = c.lo[0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        QT_INSERT_RECURSIVE.call(c, 0x002c_19f0)?;
    }
    // 0x2c19f0: the node is handled; leave through the epilogue.
    c.cycles += 1;
    f.restore_ra(c, 0);
    quad_tree_insert_end(c, f)
}

/// 0x2c19f4 epilogue of `QuadTree_Insert`.
pub(crate) fn quad_tree_insert_end(c: &mut Cpu, f: Frame) -> Flow {
    c.cycles += 2;
    f.close(c);
    Ok(())
}

/// `int QuadTree_CountNodes(node)` (0x2c2178): the number of nodes under
/// `node`, counting the node itself (1 for a leaf). Recurses through the four
/// child slots of a node whose first halfword is odd.
pub fn quad_tree_count_nodes(c: &mut Cpu) {
    guest(c, quad_tree_count_nodes_body);
}

pub(crate) fn quad_tree_count_nodes_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let f = Frame::open(c, 0x50);
    f.save(c, S2, 0x20);
    f.save(c, S3, 0x10);
    c.g[S2][0] = c.g[A0][0];
    f.save(c, S0, 0x40);
    c.g[S3][0] = 1;
    f.save(c, S1, 0x30);
    let null_node = c.g[A1][0] == 0;
    f.save_ra(c, 0);
    if !null_node {
        c.cycles += 4;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lhu(c);
        c.g[V0][0] &= 1;
        let leaf = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !leaf {
            c.cycles += 1;
            c.g[S1][0] = 3;
            loop {
                c.cycles += 11;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lhu(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[V0][0] = oc1052_at(c, S2, 0x20).lw(c);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(2));
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                QT_COUNT_NODES.call(c, 0x002c_21d4)?;
                let more_children = (c.g[S1][0] as i64) >= 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                if more_children {
                    if event_due(c) {
                        deliver_event_here(c, 0x002c_21b0);
                    }
                    continue;
                }
                break;
            }
        }
    }
    // 0x2c21dc: return the count.
    c.cycles += 8;
    c.g[V0][0] = c.g[S3][0];
    f.restore(c, S0, 0x40);
    f.restore(c, S1, 0x30);
    f.restore(c, S2, 0x20);
    f.restore(c, S3, 0x10);
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// `GameModule_UnloadStub` (0x2c3fa0): an empty unload hook; returns at once.
pub fn game_module_unload_stub(c: &mut Cpu) {
    c.cycles += 2;
    c.pc = c.g[RA][0] as u32;
}

/// `int cPS2MemCard_DeferState18(cPS2MemCard *self)` (0x2c4758):
/// raises the deferred-state flag at `self+0x48`, returns 1.
pub fn ps2_mem_card_defer_state_18(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
    c.pc = t;
    return;
}

/// `int cPS2MemCard_DeferState1E(cPS2MemCard *self)` (0x2c4890):
/// raises the deferred-state flag at `self+0x44`, returns 1.
pub fn ps2_mem_card_defer_state_1e(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x44), c.g[V0][0]);
    c.pc = t;
    return;
}

/// `int cPS2MemCard_Get490(cPS2MemCard *self)` (0x2c4400): returns the
/// word at `self+0x490`.
pub fn ps2_mem_card_get_490(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x490));
    c.pc = t;
    return;
}

/// `void cPS2MemCard_Set24(cPS2MemCard *self, int v)` (0x2c4408): stores
/// `A1` at `self+0x24`.
pub fn ps2_mem_card_set_24(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[A1][0]);
    c.pc = t;
    return;
}

/// `void cPS2MemCard_v46(cPS2MemCard *self)` (0x2c5520): empty vtable
/// slot.
pub fn ps2_mem_card_v46(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v57(cPS2MemCard *self)` (0x2c6700): vtable slot
/// returning 1.
pub fn ps2_mem_card_v57(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0x1;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v58(cPS2MemCard *self)` (0x2c6708): vtable slot
/// returning 1.
pub fn ps2_mem_card_v58(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0x1;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v61(cPS2MemCard *self)` (0x2c6710): vtable slot
/// returning 1.
pub fn ps2_mem_card_v61(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0x1;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_GetBlockSize(cPS2MemCard *self)` (0x2c6748):
/// returns the card block size, 0x4000.
pub fn ps2_mem_card_get_block_size(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0x4000;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v21(cPS2MemCard *self)` (0x2c6750): vtable slot
/// returning -1.
pub fn ps2_mem_card_v21(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0xffffffffffffffff;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v50(cPS2MemCard *self)` (0x2c67a8): vtable slot
/// returning 0.
pub fn ps2_mem_card_v50(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v25(cPS2MemCard *self)` (0x2c67e0): vtable slot
/// returning -1.
pub fn ps2_mem_card_v25(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0xffffffffffffffff;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_GetCurrentPort(cPS2MemCard *self)` (0x2d37d8):
/// returns the current port word at `self+0xc`.
pub fn ps2_mem_card_get_current_port(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v59(cPS2MemCard *self)` (0x2d3920): vtable slot
/// returning 0.
pub fn ps2_mem_card_v59(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v60(cPS2MemCard *self)` (0x2d3928): vtable slot
/// returning 0.
pub fn ps2_mem_card_v60(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_Get14(cPS2MemCard *self)` (0x2d3970): returns the
/// word at `self+0x14`.
pub fn ps2_mem_card_get_14(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v69(cPS2MemCard *self)` (0x2d3978): vtable slot
/// returning 2.
pub fn ps2_mem_card_v69(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0x2;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_v72(cPS2MemCard *self)` (0x2d39a8): vtable slot
/// returning 0.
pub fn ps2_mem_card_v72(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = 0u64;
    c.pc = t;
    return;
}

/// `int cPS2MemCard_IsSyncDone(cPS2MemCard *self)` (0x2d39b0): returns the
/// sync-done word at `self+0x3c`.
pub fn ps2_mem_card_is_sync_done(c: &mut Cpu) {
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3c));
    c.pc = t;
    return;
}

/// `LocStringTable_GetInfo(table, kind, record)` (0x3db9d8): reads one
/// packed info word of a localised string record. Kind 1 and kind 0 read two
/// bytes each, kind 2 a halfword, kind 3 a flag bit, other kinds 0.
pub fn loc_string_table_get_info(c: &mut Cpu) {
    'tail: {
        'main: {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let kind_one = c.g[A1][0] == c.g[V0][0];
            c.g[A3][0] = c.g[A0][0];
            if kind_one {
                c.cycles += 3;
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xd));
                c.g[V1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xc));
                break 'tail;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                if c.g[A1][0] == 0 {
                    c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xf));
                    c.cycles += 1;
                    c.g[V1][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0xe));
                    break 'tail;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                ret(c);
                return;
            }
            c.g[V0][0] = 0x2;
            c.cycles += 2;
            if c.g[A1][0] == c.g[V0][0] {
                c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0xe));
                break 'main;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if c.g[A1][0] == c.g[V0][0] {
                c.g[V0][0] = c.lbu((c.g[A3][0] as u32).wrapping_add(0x8));
                c.cycles += 2;
                c.g[V0][0] = c.g[V0][0] & 0x1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            ret(c);
            return;
        }
        // The record's packed word: bytes at +0x3, +0x2, +0x1, +0x0 of the
        // entry at `a3 + 4 * v0 + 0x10`, then the next entry's +0xc.
        c.cycles += 26;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x3));
        c.g[A0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 24);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
        c.g[A2][0] = c.lbu(c.g[V0][0] as u32);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
        c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x2));
        c.g[A1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
        c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
        ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    ret(c);
}

/// `LocStringTable_CompareId` (0x003d_bac8): straight-line leaf: 3 load(s). Mechanical port of the generated body.
pub fn loc_string_table_compare_id_003dbac8(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[V1][0] = c.lbu(c.g[A1][0] as u32);
    c.g[A2][0] = c.lhu(c.g[A0][0] as u32);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    ret(c);
}

/// The string the ISTR allocations are tagged with (`str_ISTR_data`).
pub(crate) const ISTR_DATA: u32 = 0x0048_6098;

/// `ISTR_Alloc(name, size, zeroFlag)` (0x2c2d30): `new (str_ISTR_data,
/// zeroFlag ? 0x100 : 0, 0) char[size]`. The name is not used. Returns the
/// allocator's pointer.
pub fn istr_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = 0x100;
        let frame = Frame::open(c, 0x10);
        if c.g[A2][0] == 0 {
            c.g[V0][0] = 0u64;
        }
        c.g[A1][0] = sx(ISTR_DATA);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x0031_7e30, 0x002c_2d58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ISTR_Free(block)` (0x2c2d68): frees through `MEM_Free` and returns whether
/// that gave a non-zero result (`0 < v0`).
pub fn istr_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_23a8, 0x002c_2d78)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `ISTR_MemCopy(dst, src, size)` (0x2c2d88): `memcpy`, returns `dst`.
pub fn istr_mem_copy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x003e_6574, 0x002c_2d9c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ISTR_Realloc(block, name, size, zeroFlag)` (0x2c2db0): no block allocates
/// (`ISTR_Alloc`); size 0 frees the block (`ISTR_Free`); otherwise the block is
/// resized under the `cMemMan` mutex.
pub fn istr_realloc(c: &mut Cpu) {
    guest(c, realloc_body)
}

pub(crate) fn realloc_body(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x30);
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[A1][0];
    c.g[A2][0] = c.g[A3][0];
    if c.g[S1][0] == 0 {
        // No block yet: allocate `size` bytes.
        c.cycles += 4;
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x002c_2d30,
            f: ee::s08::f_002c2d30,
        }
        .call(c, 0x002c_2de0)?;
        frame.restore(c, S0, 0x30);
    } else {
        c.cycles += 2;
        let size_is_zero = c.g[S2][0] == 0;
        c.g[S0][0] = 0x540000;
        if size_is_zero {
            // Size 0: free the block.
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x002c_2d68,
                f: ee::s08::f_002c2d68,
            }
            .call(c, 0x002c_2e24)?;
            c.g[V0][0] = 0u64;
        } else {
            // Resize under the mutex (cMemMan_m_mutex at 0x538b00).
            c.cycles += 11;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x7500));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_5700, 0x002c_2dfc)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x0031_9bd8, 0x002c_2e08)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_5760, 0x002c_2e14)?;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
    }
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `LocStringTable_RemapId(short key, const u8 *index)` (0x3dbae8): binary
/// search (`bsearch` with `compare_BAE8`, 4-byte entries) of the key in the
/// table at `index + 0x10`, whose entry count is the byte word at `index + 8`.
/// Returns the 16-bit value stored after the found key, or -1 if it is absent.
pub fn loc_string_table_remap_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        let sp = c.g[SP][0] as u32;
        let key = c.g[A0][0];
        Addr(sp).sh(c, key);
        c.g[T0][0] = 0x3e0000;
        let ra = c.g[RA][0];
        Addr(sp.wrapping_add(0x10)).sd(c, ra);
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x4538));
        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x3)).lbu(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2)).lbu(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1)).lbu(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
        c.g[A3][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x8)).lbu(c);
        c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 8);
        c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = c.g[A2][0] | c.g[A3][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = 0x4;
        call_ptr(c, 0x0041_87e8, 0x003d_bb38)?;
        c.g[V1][0] = c.g[V0][0];
        let not_found = c.g[V1][0] == 0u64;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
        if not_found {
            c.cycles += 1;
            c.g[V0][0] = 0xffffffffffffffff;
        } else {
            c.cycles += 5;
            let value = c.g[V1][0] as u32;
            c.g[V0][0] = Addr(value.wrapping_add(0x1)).lbu(c);
            c.g[V1][0] = Addr(value).lbu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
        }
        c.cycles += 3;
        let frame = c.g[SP][0] as u32;
        c.g[RA][0] = Addr(frame.wrapping_add(0x10)).ld(c);
        c.g[SP][0] = sx(frame.wrapping_add(0x20));
        ret(c);
        Ok(())
    })
}

pub(crate) const OC1599_ALLOC_NODE: Callee = Callee {
    addr: 0x002c_17d0,
    f: ee::s08::QuadTree_AllocNode,
};

pub(crate) const OC1599_SET_CHILDREN: Callee = Callee {
    addr: 0x002c_16b8,
    f: ee::s08::QuadTreeNode_SetChildren,
};

pub(crate) const OC1599_SET_LEAF: Callee = Callee {
    addr: 0x002c_16a8,
    f: ee::s08::QuadTreeNode_SetLeaf,
};

pub(crate) const OC1599_FREE_CHILDREN: Callee = Callee {
    addr: 0x002c_18e8,
    f: ee::s08::QuadTree_FreeChildren,
};

/// The recursion goes through the generated alias (the override table).
pub(crate) const OC1599_INSERT_RECURSIVE: Callee = Callee {
    addr: 0x002c_1a00,
    f: ee::s08::QuadTree_InsertRecursive,
};

/// `void QuadTree_InsertRecursive(QuadTree* tree, QuadTreeNode* node, u16 x, u16 z,
/// int size, a, b)` (0x2c1a00): set the cell `(x, z)` of the quad tree, below `node`
/// and `size` cells wide, to the leaf value `(a, b)`. A leaf node that is larger than
/// one cell is first split into four children (allocated with `QuadTree_AllocNode`,
/// all holding the old leaf value), the insert recurses into the child that holds
/// the point, and afterwards four equal children are merged back into a leaf
/// (`QuadTree_FreeChildren`, `QuadTreeNode_SetLeaf`). At size < 2 the cell is set
/// directly (freeing its children first if it had any) and the merge check follows.
/// The node's low half-word bit 0 marks an internal node; children are 8-byte
/// node indices in the half-words (`>> 1`) relative to the array at `tree+0x20`.
#[allow(unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn quad_tree_insert_recursive_oc1599(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c1bc0: {
            'b_2c1b80: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                c.g[A2][0] = c.g[A2][0] & 0xffff;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[A3][0] = c.g[A3][0] & 0xffff;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[S1][0] = c.g[A1][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A0][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S6][0] = c.g[T1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[S7][0] = c.g[T2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = ((c.g[T0][0] as i64) < 2i64) as u64;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 3);
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.g[V1][0] & 0xffff;
                if t {
                    break 'b_2c1b80;
                } else {
                    'b_2c1b24: {
                        c.cycles += 14;
                        c.g[FP][0] = sx((c.g[V1][0] as u32) << 3);
                        c.g[V0][0] = c.lhu(c.g[S1][0] as u32);
                        c.g[V1][0] = sx((c.g[A2][0] as u32) << 1);
                        c.g[A0][0] = sx((c.g[A3][0] as u32) << 1);
                        c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 1) as u32);
                        c.g[A2][0] = sx((c.g[A2][0] as u32) >> 15);
                        c.g[A3][0] = sx((c.g[A3][0] as u32) >> 15);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                        let t = c.g[V0][0] != 0u64;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A3][0]);
                        if t {
                            break 'b_2c1b24;
                        } else {
                            c.cycles += 25;
                            c.g[S5][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A0][0] = c.g[S2][0];
                            c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[RA][0] = sx(0x002c1aa8);
                            c.g[S3][0] = 0x3;
                            OC1599_ALLOC_NODE.call(c, 0x002c1aa8)?;
                            c.sh(c.g[SP][0] as u32, c.g[V0][0]);
                            c.g[RA][0] = sx(0x002c1ab4);
                            c.g[A0][0] = c.g[S2][0];
                            OC1599_ALLOC_NODE.call(c, 0x002c1ab4)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x002c1ac0);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                            OC1599_ALLOC_NODE.call(c, 0x002c1ac0)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x002c1acc);
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            OC1599_ALLOC_NODE.call(c, 0x002c1acc)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                            c.sh((c.g[SP][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[FP][0] as u32));
                            c.g[A2][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x2));
                            c.g[A3][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.lhu(c.g[SP][0] as u32);
                            c.g[T0][0] = c.g[V0][0] & 0xffff;
                            c.g[RA][0] = sx(0x002c1af4);
                            c.g[S0][0] = c.g[S1][0];
                            OC1599_SET_CHILDREN.call(c, 0x002c1af4)?;
                            c.g[A0][0] = c.lhu(c.g[S0][0] as u32);
                            'l_2c1af8: loop {
                                c.cycles += 11;
                                c.g[A1][0] = c.g[S4][0];
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                c.g[A2][0] = c.g[S5][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                                c.g[RA][0] = sx(0x002c1b1c);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                OC1599_SET_LEAF.call(c, 0x002c1b1c)?;
                                if (c.g[S3][0] as i64) >= 0 {
                                    c.g[A0][0] = c.lhu(c.g[S0][0] as u32);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002c1af8);
                                    }
                                    continue 'l_2c1af8;
                                } else {
                                    break 'b_2c1b24;
                                }
                            }
                        }
                    }
                    c.cycles += 23;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.g[T1][0] = c.g[S6][0];
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[T2][0] = c.g[S7][0];
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.lhu(c.g[V0][0] as u32);
                    c.g[A2][0] = c.g[A0][0] & 0xfffe;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
                    c.g[A3][0] = c.g[A0][0] & 0xfffe;
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002c1b74);
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    OC1599_INSERT_RECURSIVE.call(c, 0x002c1b74)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                    let t = true;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    if t {
                        break 'b_2c1bc0;
                    } else {
                        break 'b_2c1b80;
                    }
                }
            }
            'b_2c1bb0: {
                c.cycles += 4;
                c.g[A1][0] = c.lhu(c.g[S1][0] as u32);
                c.g[V0][0] = c.g[A1][0] & 0x1;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 1);
                if t {
                    break 'b_2c1bb0;
                } else {
                    c.cycles += 8;
                    c.g[A2][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                    c.g[A3][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[T0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
                    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 1);
                    c.g[A3][0] = sx((c.g[A3][0] as u32) >> 1);
                    c.g[RA][0] = sx(0x002c1bb0);
                    c.g[T0][0] = sx((c.g[T0][0] as u32) >> 1);
                    OC1599_FREE_CHILDREN.call(c, 0x002c1bb0)?;
                    break 'b_2c1bb0;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S6][0];
            c.g[RA][0] = sx(0x002c1bc0);
            c.g[A2][0] = c.g[S7][0];
            OC1599_SET_LEAF.call(c, 0x002c1bc0)?;
            break 'b_2c1bc0;
        }
        'b_2c1ca8: {
            'b_2c1ca4: {
                c.cycles += 4;
                c.g[V1][0] = c.lhu(c.g[S1][0] as u32);
                c.g[V0][0] = c.g[V1][0] & 0x1;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = sx((c.g[V1][0] as u32) >> 1);
                if t {
                    break 'b_2c1ca4;
                } else {
                    'b_2c1c0c: {
                        c.cycles += 11;
                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x2));
                        c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
                        c.g[A2][0] = sx((c.g[V0][0] as u32) >> 1);
                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 3);
                        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[T0][0] = 0u64;
                        if t {
                            break 'b_2c1c0c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                            c.g[T0][0] = (c.g[V0][0] < 0x1u64) as u64;
                            break 'b_2c1c0c;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[T0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
                    if t {
                        break 'b_2c1ca4;
                    } else {
                        'b_2c1c48: {
                            c.cycles += 9;
                            c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                            c.g[A3][0] = sx((c.g[V1][0] as u32) >> 1);
                            c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = sx((c.g[A3][0] as u32) << 3);
                            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                            let t = c.g[A0][0] != c.g[V1][0];
                            c.g[T3][0] = 0u64;
                            if t {
                                break 'b_2c1c48;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(c.g[T2][0] as u32);
                                c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                c.g[T3][0] = (c.g[V0][0] < 0x1u64) as u64;
                                break 'b_2c1c48;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[T3][0] == 0u64;
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 3);
                        if t {
                            break 'b_2c1ca4;
                        } else {
                            'b_2c1c84: {
                                c.cycles += 9;
                                c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
                                c.g[T2][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                                c.g[T0][0] = sx((c.g[V1][0] as u32) >> 1);
                                c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = sx((c.g[T0][0] as u32) << 3);
                                c.g[T1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                                c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                                let t = c.g[A0][0] != c.g[V1][0];
                                c.g[T3][0] = 0u64;
                                if t {
                                    break 'b_2c1c84;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw(c.g[T2][0] as u32);
                                    c.g[V1][0] = c.lw(c.g[T1][0] as u32);
                                    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                                    c.g[T3][0] = (c.g[V0][0] < 0x1u64) as u64;
                                    break 'b_2c1c84;
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[T3][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                            if t {
                                break 'b_2c1ca8;
                            } else {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x002c1c94);
                                c.g[A0][0] = c.g[S2][0];
                                OC1599_FREE_CHILDREN.call(c, 0x002c1c94)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S6][0];
                                c.g[RA][0] = sx(0x002c1ca4);
                                c.g[A2][0] = c.g[S7][0];
                                OC1599_SET_LEAF.call(c, 0x002c1ca4)?;
                                break 'b_2c1ca4;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
            break 'b_2c1ca8;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;

    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_10;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_6;
    pub(crate) use crate::game::be::beintnetwork::tests::put_str;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_6;
    pub(crate) use crate::game::bx::bxstring::tests::string as string_2;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_4;
    pub(crate) use crate::game::bx::bxstringctor::tests::game;

    pub(crate) use crate::game::bx::bxstringctor::tests::slot_2;

    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstringctor::tests::VTS;

    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_7;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_3;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::string;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear_5;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_9;

    pub(crate) use crate::game::fe::fepopup::tests::b;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_8;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::object::debouncenode::tests::vtable_2;
    pub(crate) use crate::game::render::particle::tests::clear as clear_4;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_7;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_6;

    pub(crate) use crate::game::sound::soundsys::tests::wf;
    pub(crate) use crate::game::ui::uitext::tests::h;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear_2;

    use ssx3_ee as ee;

    use ssx3_ee::s08;

    /// Fill the two static SJIS conversion tables around 0x445940.
    pub(crate) fn sjis_tables(c: &mut Cpu, r: &mut Rng) {
        for k in 0..0x60u32 {
            c.bus.write16(0x0044_5940 + 2 * k, r.next_u32() as u16);
        }
    }

    #[test]
    fn memcard_ascii_char_to_sjis_matches() {
        check(
            ssx3_ee::s08::f_002c2338,
            memcard_ascii_char_to_sjis,
            300,
            &|c, seed| {
                let mut r = Rng(0x2338 ^ seed as u64);
                scramble(c, &mut r);
                sjis_tables(c, &mut r);
                let ch = match seed % 9 {
                    0 => 0x20 + (r.next_u32() % 0x10),
                    1 => 0x30 + (r.next_u32() % 0xa),
                    2 => 0x3a + (r.next_u32() % 7),
                    3 => 0x41 + (r.next_u32() % 0x1a),
                    4 => 0x5b + (r.next_u32() % 6),
                    5 => 0x61 + (r.next_u32() % 0x1a),
                    6 => 0x7b + (r.next_u32() % 4),
                    7 => r.next_u32() % 0x20,
                    _ => r.next_u32() & 0xff,
                };
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | ch as u64;
            },
        );
    }

    #[test]
    fn memcard_sjis_char_to_ascii_matches() {
        check(
            ssx3_ee::s08::f_002c2430,
            memcard_sjis_char_to_ascii,
            300,
            &|c, seed| {
                let mut r = Rng(0x2430 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                sjis_tables(c, &mut r);
                for k in 0..0x20u32 {
                    c.bus.write8(GAME_GP + 0x7a0 + k, r.next_u32() as u8);
                }
                let (lead, trail) = match seed % 5 {
                    0 => (r.next_u32() as u8, 0x81u8),
                    1 => (r.next_u32() as u8, 0x80u8),
                    _ => {
                        let lead = match seed % 6 {
                            0 => (0x4f + (r.next_u32() % 10)) as u8,
                            1 => (0x60 + (r.next_u32() % 0x1a)) as u8,
                            2 => (r.next_u32() % 0x4f) as u8,
                            3 => (0x59 + (r.next_u32() % 7)) as u8,
                            4 => (0x7a + (r.next_u32() % 6)) as u8,
                            _ => r.next_u32() as u8,
                        };
                        (lead, 0x82u8)
                    }
                };
                let code = ((lead as u32) << 8) | trail as u32;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | code as u64;
            },
        );
    }

    #[test]
    fn cps2memcard_get_port_error_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c51d0,
            cps2memcard_get_port_error,
            300,
            &|c, seed| {
                let mut r = Rng(0x51d0 ^ seed as u64);
                scramble(c, &mut r);
                let port = seed % 4;
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | DATA as u64;
                c.g[A1][0] = port as u64;
                let e = match seed % 12 {
                    0 => 0,
                    1 => 1,
                    2 => (-7i32) as u32,
                    3 => (-8i32) as u32,
                    4 => (-10i32) as u32,
                    5 => (-11i32) as u32,
                    6 => (-10000i32) as u32,
                    7 => (-10001i32) as u32,
                    8 => (-10002i32) as u32,
                    9 => (-10003i32) as u32,
                    _ => r.next_u32(),
                };
                c.bus.write32(DATA + port * 0x14 + 0x184, e);
            },
        );
    }

    #[test]
    fn cps2memcard_clear_port_info_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c5250,
            cps2memcard_clear_port_info,
            200,
            &|c, seed| {
                let mut r = Rng(0x5250 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn cps2memcard_get_free_space_for_port_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c5320,
            cps2memcard_get_free_space_for_port,
            200,
            &|c, seed| {
                let mut r = Rng(0x5320 ^ seed as u64);
                scramble(c, &mut r);
                let port = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = port as u64;
                c.bus.write32(DATA + port * 0x14 + 0x180, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2memcard_mark_save_invalid_matches() {
        check_lohi(
            ssx3_ee::s08::f_002c5570,
            cps2memcard_mark_save_invalid,
            200,
            &|c, seed| {
                let mut r = Rng(0x5570 ^ seed as u64);
                scramble(c, &mut r);
                let port = seed % 4;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = port as u64;
                c.bus.write32(DATA + port * 0x78 + 0x1a8, r.next_u32());
            },
        );
    }

    #[test]
    fn cps2memcard_is_mc_error_matches() {
        check(
            ssx3_ee::s08::f_002c63e8,
            cps2memcard_is_mc_error,
            300,
            &|c, seed| {
                let mut r = Rng(0x63e8 ^ seed as u64);
                scramble(c, &mut r);
                let v = match seed % 4 {
                    0 => (r.next_u32() % 0x3000).wrapping_sub(0x2800),
                    1 => 0xffff_d8f1, // -9999
                    2 => 0x7fff_ffff,
                    _ => r.next_u32(),
                };
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | v as u64;
            },
        );
    }

    #[test]
    fn cps2_mem_card_get_port_changed_flag_matches() {
        check_lohi(
            ee::s08::f_002c53c8__generated,
            cps2_mem_card_get_port_changed_flag,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002c_53c8, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let port = (seed % 8) as u32;
                c.g[A1][0] = mixed(&mut r, seed, port);
                c.bus.write32(DATA + port * 0x14 + 0x18c, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v39_matches() {
        check(
            ee::s08::f_002c66b8__generated,
            c_ps2_mem_card_v39,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_66b8, 0);
                c.g[A0][0] = DATA as u64;
                let name = DATA + 0x800;
                put_str(c, name, if seed % 2 == 0 { "slot1" } else { "" });
                c.g[A1][0] = name as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_clear_dir_entry_matches() {
        check(
            ee::s08::f_002c52d8__generated,
            c_ps2_mem_card_clear_dir_entry,
            60,
            &|c, seed| {
                let _r = game(c, seed, 0x2c52d8, 1);
                c.g[A0][0] = SCRATCH as u64;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v15_matches() {
        check(
            ee::s08::f_002c64a8__generated,
            c_ps2_mem_card_v15,
            80,
            &|c, seed| {
                let mut r = game(c, seed, 0x2c64a8, 1);
                c.g[A2][0] = (SCRATCH + 0x900) as u64;
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                // The ASCII source the second argument points at (D_004A3968).
                for (i, b) in b"memcard\0".iter().enumerate() {
                    c.bus.write8(0x004a_3968 + i as u32, *b);
                }
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v16_matches() {
        check(
            ee::s08::f_002c64d0__generated,
            c_ps2_mem_card_v16,
            80,
            &|c, seed| {
                let mut r = game(c, seed, 0x2c64d0, 1);
                c.g[A2][0] = (SCRATCH + 0x900) as u64;
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                for (i, b) in b"memcard\0".iter().enumerate() {
                    c.bus.write8(0x004a_3968 + i as u32, *b);
                }
            },
        );
    }

    // -----------------------------------------------------------------
    // MemCard and menus.

    #[test]
    fn cps2_memcard_get_dir_entry_name_matches() {
        check(
            s08::f_002c68c8__generated,
            cps2_memcard_get_dir_entry_name,
            150,
            &|c, seed| {
                machine(c, seed, 0x2c68c8, 1);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(DATA + 0x4000);
                clear(c, OBJ, 0x200);
                clear(c, DATA + 0x4000, 0x40);
                // A short NUL-terminated name in each of the eight entries.
                for i in 0..8u32 {
                    let name = b"mc0:dir\0";
                    for (k, b) in name.iter().enumerate() {
                        c.bus.write8(OBJ + (i << 6) + 0x4e0 + k as u32, *b);
                    }
                }
                // Scrambled destinations are fine, but keep strcpy on RAM: the
                // copy length comes from the source only.
                c.bus.write8(DATA + 0x4000 + 0x3f, 0);
            },
        );
    }

    /// The engine's size-class pools (`MEM_PoolAlloc`): allocations up to
    /// 0x200 bytes are popped from per-class free lists hanging off
    /// `gp + 0x2a60..`, but only when the "pools ready" flag at `gp + 0xdd8`
    /// is set; otherwise `MEM_PoolAlloc` forwards to `MEM_AllocInternal`, which
    /// asks the pools again — an endless recursion on a fresh test machine.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1); // pools ready
                                           // (pool pointer global, free-list head offset within the pool object,
                                           // block size of the class)
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),     // 8-byte blocks
            (0x2a60, 0x2000, 0x10), // 16
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            // 16 free nodes per class, spaced by the block size (the head is
            // updated on pop, so only an unpopped node's next pointer matters).
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    /// The main heap's dlmalloc arena over 256 KiB of test RAM, as
    /// `containers.rs`/`oc59` set up for the allocator tests: allocations
    /// larger than 0x200 bytes come from here.
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    pub(crate) fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    /// A machine for functions that allocate: fake callees for the misses,
    /// the pool free lists and a real heap arena.
    pub(crate) fn alloc_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        pool_allocator(c);
        heap_init(c);
        r
    }

    /// A vtable at a fixed test address with `{delta, fake}` at each slot.
    pub(crate) fn put_vtable(c: &mut Cpu, r: &mut Rng, at: u32, slots: &[u32]) -> u32 {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
        at
    }

    // -------------------------------------------------------- memory card

    #[test]
    fn c_ps2_mem_card_create_matches() {
        check(
            ee::s08::f_002c22c8__generated,
            c_ps2_mem_card_create,
            40,
            &|c, seed| {
                alloc_machine(c, seed, 0x2c22_c8);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_destroy_matches() {
        check(
            ee::s08::f_002c2300__generated,
            c_ps2_mem_card_destroy,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c23_00, 0);
                let obj = DATA + 0x100;
                let vt = put_vtable(c, &mut r, DATA + 0x800, &[8]);
                c.bus.write32(obj, vt);
                c.g[A0][0] = sx(if seed % 4 == 0 { 0 } else { obj });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_ctor_matches() {
        check(
            ee::s08::f_002c3fa8__generated,
            c_ps2_mem_card_ctor,
            40,
            &|c, seed| {
                alloc_machine(c, seed, 0x2c3f_a8);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_matches() {
        check(
            ee::s08::f_002c41d8__generated,
            c_ps2_mem_card_request_list_dir,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c41_d8, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.bus.write8(DATA + 0x2000, 0);
                let len = (seed % 16) as u32;
                for i in 0..len {
                    c.bus
                        .write8(DATA + 0x2000 + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(DATA + 0x2000 + len, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_delete_matches() {
        check(
            ee::s08::f_002c6ce0__generated,
            c_ps2_mem_card_request_delete,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c6c_e0, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                let len = (seed % 16) as u32;
                for i in 0..len {
                    c.bus
                        .write8(DATA + 0x2000 + i, b'A' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(DATA + 0x2000 + len, 0);
            },
        );
    }

    // -------------------------------------------------------- strings

    driven!(gen_ustr_len_2, ee::s08::f_002c24d0__generated);

    driven!(hand_ustr_len_2, ustr_len_2);

    #[test]
    fn ustr_len_2_matches() {
        check_events(gen_ustr_len_2, hand_ustr_len_2, 200, &|c, seed| {
            let mut r = Rng(0x7573_7472_6c65_6e32 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            let len = (seed % 24) as u32;
            for i in 0..len {
                let b = match r.next_u32() % 5 {
                    0 => [1u16, 0x7f, 0x80, 0xff, 0xfe][(r.next_u32() % 5) as usize],
                    _ => (r.next_u32() as u16).max(1),
                };
                c.bus.write16(DATA + 2 * i, b);
            }
            c.bus.write16(DATA + 2 * len, 0);
            c.g[A0][0] = sx(DATA);
            schedule(c, &mut r, seed, 0x002c_24d0)
        });
    }

    driven!(gen_str_to_lower, ee::s08::f_002c2868__generated);

    driven!(hand_str_to_lower, str_to_lower_in_place);

    #[test]
    fn str_to_lower_in_place_matches() {
        check_events(gen_str_to_lower, hand_str_to_lower, 200, &|c, seed| {
            let mut r = Rng(0x6c6f_7765_72 ^ (seed as u64) << 20);
            crate::difftest::scramble(c, &mut r);
            let len = (seed % 24) as u32;
            for i in 0..len {
                let b = match r.next_u32() % 6 {
                    0 => b'A' + (r.next_u32() % 26) as u8,
                    1 => [b'Z', b'a', b'z', b'@', b'[', 0x80, 0xff][(r.next_u32() % 7) as usize],
                    _ => (r.next_u32() as u8).max(1),
                };
                c.bus.write8(DATA + i, b);
            }
            c.bus.write8(DATA + len, 0);
            c.g[A0][0] = sx(DATA);
            schedule(c, &mut r, seed, 0x002c_2868)
        });
    }

    // -----------------------------------------------------------------------
    // Memcard request wrappers.

    pub(crate) fn memcard_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        let name = DATA + 0x800;
        match seed % 3 {
            0 => put_str(c, name, ""),
            1 => put_str(c, name, "BESLES-51234"),
            _ => put_str(c, name, "a much longer memory card file name"),
        }
        c.g[A1][0] = name as u64;
        c.g[A2][0] = (r.next_u32() % 4) as u64;
        // The state machine's dispatch table is zeroed: it lands in the
        // `jump_out` hook, which the test machine turns into a fake call.
        c.bus.write32(DATA + 0x18, 0x100);
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_all_matches() {
        check(
            ssx3_ee::s08::f_002c6810__generated,
            c_ps2_mem_card_request_list_dir_all,
            48,
            &|c, seed| {
                memcard_setup(c, seed, 0x2c68_10);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_open_matches() {
        check(
            ssx3_ee::s08::f_002c6968__generated,
            c_ps2_mem_card_request_open,
            48,
            &|c, seed| {
                memcard_setup(c, seed, 0x2c69_68);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_close_matches() {
        check(
            ssx3_ee::s08::f_002c6a28__generated,
            c_ps2_mem_card_do_close,
            48,
            &|c, seed| {
                // A zero `sceMcClose` global makes the firmware call take its
                // "not initialised" path and return at once.
                memcard_setup(c, seed, 0x2c6a_28);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_mkdir_matches() {
        check(
            ssx3_ee::s08::f_002c6c50__generated,
            c_ps2_mem_card_request_mkdir,
            48,
            &|c, seed| {
                memcard_setup(c, seed, 0x2c6c_50);
            },
        );
    }

    #[test]
    fn ustr_from_ascii_2_matches() {
        check(
            ssx3_ee::s08::f_002c2540__generated,
            ustr_from_ascii_2,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2c25_40, 0);
                belib(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let text: &[u8] = match seed % 4 {
                    0 => &b""[..],
                    1 => &b"Hello"[..],
                    2 => &[0x41, 0xc3, 0x80, 0x7f][..],
                    _ => &[0x80, 0x81, 0xff][..],
                };
                for (i, &ch) in text.iter().enumerate() {
                    b(c, DATA + 0x100 + i as u32, ch);
                }
                b(c, DATA + 0x100 + text.len() as u32, 0);
            },
        );
    }

    #[test]
    fn ustr_copy_n_matches() {
        check(
            ssx3_ee::s08::f_002c25b8__generated,
            ustr_copy_n,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2c25_b8, 0);
                belib(c);
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 3,
                    1 => 0,
                    _ => sx(0xffff_ffff),
                };
                let text: &[u16] = match seed % 4 {
                    0 => &[][..],
                    1 => &[0x41, 0x42, 0x43][..],
                    2 => &[0x1234, 0x8000][..],
                    _ => &[0x41][..],
                };
                for (i, &ch) in text.iter().enumerate() {
                    h(c, DATA + 0x100 + 2 * i as u32, ch);
                }
                h(c, DATA + 0x100 + 2 * text.len() as u32, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_request_list_dir_w_matches() {
        check(
            ee::s08::f_002c4198__generated,
            c_ps2_mem_card_request_list_dir_w,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4198, 0);
                let self_at = DATA + 0x200;
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write16(DATA + 0x300, b'd' as u16);
                c.bus.write16(DATA + 0x302, b'i' as u16);
                c.bus.write16(DATA + 0x304, b'r' as u16);
                c.bus.write16(DATA + 0x306, 0);
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                c.bus.write32(self_at, vt);
            },
        );
    }

    pub(crate) fn put_str16(c: &mut Cpu, at: u32, s: &[u16]) {
        for (i, h) in s.iter().enumerate() {
            c.bus.write16(at + 2 * i as u32, *h);
        }
    }

    #[test]
    fn ustr_cat_matches() {
        check(ee::s08::f_002c2688__generated, ustr_cat, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x2c26_88, 1);
            let dst = DATA + 0x200;
            let src = DATA + 0x300;
            c.g[A0][0] = sx(dst);
            c.g[A1][0] = sx(src);
            let empty = seed % 5 == 0;
            let empty_str = [0u16];
            let filled_str = [0x61, 0x62 + (seed as u16 % 3), 0];
            put_str16(
                c,
                dst,
                if empty {
                    &empty_str[..]
                } else {
                    &filled_str[..]
                },
            );
            put_str16(c, src, &[0x63 + (seed as u16 % 3), 0]);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn c_ps2_mem_card_do_flush_matches() {
        check(
            ee::s08::f_002c6b00__generated,
            c_ps2_mem_card_do_flush,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2c6b_00, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let card = DATA + 0x2000;
                clear_2(c, this, 0x80);
                clear_2(c, card, 0x100);
                c.bus.write32(this + 0x18, card);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn loc_string_table_dtor_matches() {
        check(
            ee::s08::f_002c6d70__generated,
            loc_string_table_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2c6d_70, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x20);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x40)
                };
                c.bus.write32(this + 4, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn mem_card_ascii_to_sjis_string_matches() {
        check(
            ee::s08::f_002c2210__generated,
            mem_card_ascii_to_sjis_string,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2c2210, 1);
                let src = DATA + 0x2000;
                let dst = DATA + 0x3000;
                clear_3(c, src, 0x40);
                clear_3(c, dst, 0x40);
                let s = match seed % 4 {
                    0 => "",
                    1 => "A",
                    2 => "SSX3",
                    _ => "name",
                };
                for (i, b) in s.bytes().enumerate() {
                    c.bus.write8(src + i as u32, b);
                }
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(dst);
                c.g[A2][0] = sx(src);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_format_matches() {
        check(
            ee::s08::f_002c4388__generated,
            c_ps2_mem_card_do_format,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x2c4388, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                c.bus.write32(this + 0xc, seed % 4);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v34_matches() {
        check(
            ee::s08::f_002c4520__generated,
            c_ps2_mem_card_v34,
            SEEDS_4,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4520, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                let name = DATA + 0x2000;
                clear_3(c, name, 0x40);
                c.bus.write8(name, b'S');
                c.bus.write8(name + 1, b'S');
                c.bus.write8(name + 2, b'X');
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v35_matches() {
        check(
            ee::s08::f_002c4578__generated,
            c_ps2_mem_card_v35,
            SEEDS_4,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4578, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                let name = DATA + 0x2000;
                clear_3(c, name, 0x40);
                c.bus.write16(name, b'S' as u16);
                c.bus.write16(name + 2, b'X' as u16);
                c.bus.write16(name + 4, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(name);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v29_matches() {
        check(
            ee::s08::f_002c4800__generated,
            c_ps2_mem_card_v29,
            SEEDS_4,
            &|c, seed| {
                elf_image(c);
                let _ = common(c, seed, 0x2c4800, 3);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x400);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x2000);
                c.g[A3][0] = sx(DATA + 0x2100);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_seek_matches() {
        check(
            ee::s08::f_002c6a88__generated,
            c_ps2_mem_card_do_seek,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6a88, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0x18, r.next_u32());
                c.bus.write32(this + 0x14e4, r.next_u32());
                c.bus.write32(this + 0x14e8, r.next_u32());
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_read_matches() {
        check(
            ee::s08::f_002c6b78__generated,
            c_ps2_mem_card_do_read,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6b78, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0x18, r.next_u32());
                c.bus.write32(this + 0x2c, DATA + 0x3000);
                c.bus.write32(this + 0x24, 0x200);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_write_matches() {
        check(
            ee::s08::f_002c6bf8__generated,
            c_ps2_mem_card_do_write,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6bf8, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0x18, r.next_u32());
                c.bus.write32(this + 0x2c, DATA + 0x3000);
                c.bus.write32(this + 0x24, 0x200);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_mkdir_matches() {
        check(
            ee::s08::f_002c6c88__generated,
            c_ps2_mem_card_do_mkdir,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6c88, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0xc, r.next_u32());
                c.bus.write8(this + 0xf9, b'B');
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_delete_matches() {
        check(
            ee::s08::f_002c6d18__generated,
            c_ps2_mem_card_do_delete,
            SEEDS_5,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2c6d18, 1);
                let this = DATA + 0x100;
                clear_4(c, this, 0x400);
                c.bus.write32(this + 0xc, r.next_u32());
                c.bus.write8(this + 0xf9, b'B');
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_clear_port_changed_matches() {
        check(
            ssx3_ee::s08::f_002c44e0__generated,
            c_ps2_mem_card_clear_port_changed,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c44_e000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let level = seed % 4;
                c.bus.write32(this + 0xc, level);
                for i in 0..4u32 {
                    c.bus.write32(this + 0x184 + i * 0x14, r.next_u32());
                    c.bus.write32(this + 0x18c + i * 0x14, r.next_u32());
                }
                c.bus.write32(
                    this + 0x184 + level * 0x14,
                    if seed % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_state34_matches() {
        check(
            ssx3_ee::s08::f_002c4748__generated,
            c_ps2_mem_card_is_state34,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c47_4800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0x34 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_state35_matches() {
        check(
            ssx3_ee::s08::f_002c4880__generated,
            c_ps2_mem_card_is_state35,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c48_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0x35 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_busy_matches() {
        check(
            ssx3_ee::s08::f_002c5140__generated,
            c_ps2_mem_card_is_busy,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c51_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 4, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(
                    DATA + 0x40,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_card_error_matches() {
        check(
            ssx3_ee::s08::f_002c5168__generated,
            c_ps2_mem_card_is_port_card_error,
            220,
            &|c, seed| {
                let mut r = Rng(0x2c51_6800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let level = seed % 3;
                c.bus.write32(this + 0xc, level);
                let values = [
                    0xffff_ffffu32,
                    0xffff_fffd,
                    0xffff_fff8,
                    0xffff_fff7,
                    0xffff_fff6,
                    0x0000_d8ef,
                    0x0000_d8ee,
                    0xffff_d8ef,
                    0xffff_d8ee,
                    0,
                    5,
                    0x8000_0000,
                ];
                c.bus.write32(
                    this + 0x184 + level * 0x14,
                    values[(seed as usize) % values.len()],
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_not_ps2_card_matches() {
        check(
            ssx3_ee::s08::f_002c5300__generated,
            c_ps2_mem_card_is_port_not_ps2_card,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c53_0000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let port = seed % 3;
                c.g[A1][0] = port as u64;
                c.bus.write32(
                    this + 0x184 + port * 0x14,
                    if seed % 2 == 0 {
                        0xffff_d8ee
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v52_matches() {
        check(
            ssx3_ee::s08::f_002c5358__generated,
            c_ps2_mem_card_v52,
            220,
            &|c, seed| {
                let mut r = Rng(0x2c53_5800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let port = seed % 3;
                c.g[A1][0] = port as u64;
                let values = [
                    0xffff_fff9u32, // -7
                    0xffff_d8ef,    // -10001
                    0xffff_fffb,    // -5
                    0xffff_fffc,    // -4
                    0xffff_fffd,    // -3
                    0xffff_fffe,    // -2
                    0xffff_ffff,    // -1
                    0,
                    0xffff_fff8, // -8
                    5,
                    0xffff_fffa, // -6
                    0x8000_0000,
                ];
                c.bus.write32(
                    this + 0x184 + port * 0x14,
                    values[(seed as usize) % values.len()],
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_formatted_matches() {
        check(
            ssx3_ee::s08::f_002c53b0__generated,
            c_ps2_mem_card_is_port_formatted,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c53_b000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let port = seed % 3;
                c.g[A1][0] = port as u64;
                c.bus.write32(
                    this + 0x188 + port * 0x14,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_has_slot_name_matches() {
        check(
            ssx3_ee::s08::f_002c5528__generated,
            c_ps2_mem_card_has_slot_name,
            120,
            &|c, seed| {
                let mut r = Rng(0x2c55_2800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                let name = this + slot * 0x78 + 0x1b0;
                let len = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 5,
                    _ => 12,
                };
                for i in 0..len {
                    c.bus.write8(name + i, b'A' + (seed % 26) as u8);
                }
                c.bus.write8(name + len, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_mark_slot_used_matches() {
        check(
            ssx3_ee::s08::f_002c5558__generated,
            c_ps2_mem_card_mark_slot_used,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c55_5800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(this + slot * 0x78 + 0x1a4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_slot_empty_matches() {
        check(
            ssx3_ee::s08::f_002c5588__generated,
            c_ps2_mem_card_is_slot_empty,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c55_8800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(
                    this + slot * 0x78 + 0x1a8,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    this + slot * 0x78 + 0x1a4,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_slot_has_1ac_matches() {
        check(
            ssx3_ee::s08::f_002c55c0__generated,
            c_ps2_mem_card_slot_has_1ac,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c55_c000 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let slot = r.next_u32() % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(
                    this + slot * 0x78 + 0x1ac,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_is_port_result_m11_matches() {
        check(
            ssx3_ee::s08::f_002c66d8__generated,
            c_ps2_mem_card_is_port_result_m11,
            160,
            &|c, seed| {
                let mut r = Rng(0x2c66_d800 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let level = seed % 3;
                c.bus.write32(this + 0xc, level);
                c.bus.write32(
                    this + 0x184 + level * 0x14,
                    if seed % 2 == 0 {
                        0xffff_fff5
                    } else {
                        r.next_u32()
                    },
                );
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_calc_save_size_kb_matches() {
        check(
            ssx3_ee::s08::f_002c6758__generated,
            c_ps2_mem_card_calc_save_size_kb,
            200,
            &|c, seed| {
                let mut r = Rng(0x2c67_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = sx(r.next_u32());
            },
        );
    }

    // ------------------------------------------------------- memcard

    #[test]
    fn memcard_sjis_to_ascii_string_matches() {
        check(
            ee::s08::f_002c2268__generated,
            memcard_sjis_to_ascii_string,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_2268, 1);
                belib(c);
                c.pc = 0x002c_2268;
                c.g[A1][0] = (DATA + 0x400) as u64;
                c.g[A2][0] = (DATA + 0x200) as u64;
                let n = 1 + seed % 6;
                for i in 0..n {
                    let ch = if seed % 5 == 0 {
                        0x8140 | (r.next_u32() & 0xff) as u16
                    } else if seed % 5 == 1 {
                        0x8260 | (r.next_u32() & 0x1f) as u16
                    } else {
                        (r.next_u32() & 0xff) as u16
                    };
                    h(c, DATA + 0x200 + 2 * i, ch);
                }
                h(c, DATA + 0x200 + 2 * n, 0);
            },
        );
    }

    #[test]
    fn ps2_memcard_request_get_info_matches() {
        check(
            ee::s08::f_002c4480__generated,
            ps2_memcard_request_get_info,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002c_4480, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                let vt = vtable(c, &mut r, &[0x58]);
                w(c, DATA, vt);
                if seed % 2 == 1 {
                    // A callee that answers 0 goes on to restart the state machine.
                    w(c, vt + 0x5c, 0x002a_41c8);
                    c.g[A1][0] = 0;
                }
            },
        );
    }

    #[test]
    fn ps2_memcard_do_get_info_matches() {
        check(
            ee::s08::f_002c50e0__generated,
            ps2_memcard_do_get_info,
            SEEDS_6,
            &|c, seed| {
                machine(c, seed, 0x002c_50e0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0xc, seed % 6);
            },
        );
    }

    #[test]
    fn ps2_mem_card_v38_matches() {
        check(
            ee::s08::f_002c46c0__generated,
            ps2_mem_card_v38,
            32,
            &|c, seed| {
                let mut r = base(c, seed, 0x2c46_c0);
                let s = DATA + 0x100;
                clear_5(c, s, 0x40);
                let vt = DATA + 0x400;
                vtable_2(c, vt, &[(0x120, FAKE + 0x10), (0x124, FAKE + 0x20)]);
                w_2(c, s, vt as u32);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ps2_mem_card_clear_dir_cache_matches() {
        check(
            ee::s08::f_002c5278__generated,
            ps2_mem_card_clear_dir_cache,
            32,
            &|c, seed| {
                let _ = base(c, seed, 0x2c52_78);
                let s = DATA + 0x1000;
                clear_5(c, s, 0x1000);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn cps2_mem_card_do_open_matches() {
        check(
            ee::s08::f_002c69a0__generated,
            cps2_mem_card_do_open,
            8,
            &|c, seed| {
                machine(c, seed, 0x002c_69a0, 2);
                be_library(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(DATA + 0x14e0, 0);
                c.bus.write32(0x0052_71c0 + 0x24, 0);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_dtor_matches() {
        check(
            ee::s08::f_002c3fe0__generated,
            c_ps2_mem_card_dtor,
            4,
            &|c, seed| {
                let _ = base_2(c, seed, 0x002c_3fe0, 2);
                c.g[A0][0] = OBJ as u64;
                // No buffer and no delete: a non-pool_2 pointer would walk the test
                // heap.
                c.g[A1][0] = 0;
                w(c, OBJ + 0x30, 0);
            },
        );
    }

    #[test]
    fn cps2_mem_card_count_saves_matches() {
        check(
            ee::s08::f_002c68f0__generated,
            cps2_mem_card_count_saves,
            SEEDS_7,
            &|c, seed| {
                machine(c, seed, 0x2c_68f0, 1);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x14, n);
                for i in 0..4u32 {
                    let name = DATA + 0x2000 + 0x1b0 + 0x78 * i;
                    if i % 2 == 0 && i < n {
                        string(c, name, "SAVE");
                    } else {
                        string(c, name, "");
                    }
                }
                c.pc = 0x002c_68f0;
            },
        );
    }

    /// A NUL-terminated UTF-16 string_2 at `a` (for the wide-char memcard name).
    pub(crate) fn wide_string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.encode_utf16().enumerate() {
            h(c, a + 2 * i as u32, ch);
        }
        h(c, a + 2 * s.encode_utf16().count() as u32, 0);
    }

    #[test]
    fn cps2_mem_card_v37_matches() {
        check(
            ee::s08::f_002c45d0__generated,
            cps2_mem_card_v37,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c45d0, 3);
                be_library_2(c);
                let obj = DATA;
                let name = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                wide_string(c, name, if seed % 2 == 0 { "SAVE01" } else { "" });
            },
        );
    }

    #[test]
    fn cps2_mem_card_v36_matches() {
        check(
            ee::s08::f_002c4648__generated,
            cps2_mem_card_v36,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4648, 3);
                be_library_2(c);
                let obj = DATA;
                let name = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                string_2(c, name, if seed % 2 == 0 { "SAVE01" } else { "" });
            },
        );
    }

    #[test]
    fn cps2_mem_card_request_read_save_matches() {
        check(
            ee::s08::f_002c4788__generated,
            cps2_mem_card_request_read_save,
            SEEDS_8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c4788, 3);
                be_library_2(c);
                let obj = DATA;
                let name = DATA + 0x800;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = name as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                string_2(c, name, if seed % 2 == 0 { "SLOT-A" } else { "" });
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_list_dir_matches() {
        check(
            ee::s08::f_002c4210__generated,
            c_ps2_mem_card_do_list_dir,
            40,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2c42, 3);
                let m = DATA;
                w(c, m + 0xc, seed);
                w(c, m + 0x139, 0x4142);
                c.g[A0][0] = sx(m);
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_do_list_dir_all_matches() {
        check(
            ee::s08::f_002c6848__generated,
            c_ps2_mem_card_do_list_dir_all,
            40,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2c68, 3);
                let m = DATA;
                w(c, m + 0xc, seed);
                w(c, m + 0x139, 0x4142);
                c.g[A0][0] = sx(m);
            },
        );
    }

    // ------------------------------------------------------------------
    // Memory card.

    #[test]
    fn c_ps2_mem_card_close_matches() {
        check(
            ssx3_ee::s08::f_002c6360__generated,
            c_ps2_mem_card_close,
            SEEDS_9,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c6360, 4);
                be_library_3(c);
                world_2(c);
                let card = DATA + 0x100;
                c.g[A0][0] = card as u64;
                w(c, card + 0x40, if seed % 2 == 0 { 0 } else { 1 });
                w(c, card + 0xc, r.next_u32() & 0x7);
                w(
                    c,
                    GAME_GP.wrapping_add(0x850),
                    if seed % 3 == 0 { 0xffff_ffff } else { 0 },
                );
            },
        );
    }

    #[test]
    fn cps2_memcard_get_slot_name_matches() {
        check(
            ee::s08::f_002c63f8__generated,
            cps2_memcard_get_slot_name,
            80,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x2c63);
                let dst = DATA + 0x2000;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(dst);
                slot_2(c, VTS, 0x20, 0);
                fill(c, &mut r, dst, 0x100);
                put_words(c, 0x004a_3970, &[0]);
            },
        );
    }

    pub(crate) const TBL: u32 = DATA + 0x2000;

    // ------------------------------------------------------------- localisation

    #[test]
    fn loc_string_table_find_matches() {
        check(
            ee::s08::f_002c6de0__generated,
            loc_string_table_find,
            18,
            &|c, seed| {
                let _ = base_5(c, seed, 0x002c_6de0, 0);
                c.g[A0][0] = OBJ as u64;
                let count = if seed % 7 == 0 { 0 } else { 1 + (seed % 6) };
                w(c, OBJ + 0xc, count);
                w(c, OBJ + 8, count.saturating_sub(1));
                w(c, OBJ + 0x10, TBL);
                let mut keys = [0u32; 8];
                for i in 0..count {
                    keys[i as usize] = 0x10 + i * 4;
                    w(c, TBL + i * 8, keys[i as usize]);
                    w(c, TBL + i * 8 + 4, 0x1000 + i);
                }
                let i = (seed as usize) % count.max(1) as usize;
                let delta = (seed % 5) as i64 - 2;
                c.g[A1][0] = (keys[i] as i64 + delta) as u32 as u64;
            },
        );
    }

    #[test]
    fn cps2_mem_card_do_get_save_dir_entry_matches() {
        check(
            ee::s08::f_002c42b0__generated,
            cps2_mem_card_do_get_save_dir_entry,
            4,
            &|c, seed| {
                let _ = base_5(c, seed, 0x002c_42b0, 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, OBJ + 0x1c, 0);
                w(c, OBJ + 0xc, 0);
                w(c, OBJ + 0x3c, 0);
                w(c, GAME_GP.wrapping_add(0x850), 0);
            },
        );
    }

    #[test]
    fn cps2memcard_mark_slots_without_space_matches() {
        check(
            ee::s08::f_002c6280__generated,
            cps2memcard_mark_slots_without_space,
            12,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2c6280, 3);
                let this = DATA;
                clear_6(c, this, 0x2000);
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[0xa0, 0xb0, 0x218]);
                w(c, this, vt);
                w(c, vt + 0x21c, 0x002c_c098);
                w(c, this + 0x24, (seed % 8) as u32);
                w(c, this + 0x14, 2);
                w(c, this + 0x150, (seed % 3) as u32);
                wf(c, this + 0x100, 2.0);
                wf(c, this + 0x104, 1.0);
                for i in 0..2u32 {
                    let e = this + 0x1ac + 0x78 * i;
                    w(c, e, 0);
                }
            },
        );
    }

    #[test]
    fn quad_tree_flatten_node_matches() {
        strict(
            ee::s08::f_002c2088__generated,
            quad_tree_flatten_node,
            SEEDS_10,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x2088, 0);
                let this = DATA + 0x1000;
                let base_7 = DATA + 0x4000;
                let count = DATA + 0x100;
                let out = DATA + 0x2000;
                let map = DATA + 0x3000;
                w(c, count, 0);
                w(c, this + 0x20, base_7);
                // the root's four children are leaves at base_7 + 0x100 + 0x10*k
                for k in 0..4u32 {
                    let child = base_7 + 0x100 + 0x10 * k;
                    let val = 2 * (0x100 + 0x10 * k) + 1;
                    c.bus.write16(base_7 + 2 * k, val as u16);
                    w(c, child, 0);
                    w(c, child + 8, r.next_u32());
                }
                w(c, base_7 + 8, r.next_u32());
                for k in 0..4u32 {
                    c.bus.write16(
                        base_7 + 8 + 2 * k,
                        if seed % 2 == 0 {
                            0
                        } else {
                            (0x201 + 0x20 * k) as u16
                        },
                    );
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(out);
                c.g[A2][0] = sx(count);
                c.g[A3][0] = sx(map);
                c.g[T0][0] = sx(base_7);
            },
        );
    }

    /// UTF-16 format string, va_list (8-byte slots), argument strings,
    /// `%n` targets and the output buffer.
    pub(crate) const USTR_VSPRINTF_T_FMT: u32 = DATA;

    pub(crate) const USTR_VSPRINTF_T_ARGS: u32 = DATA + 0x2000;

    pub(crate) const USTR_VSPRINTF_T_STRS: u32 = DATA + 0x4000;

    pub(crate) const USTR_VSPRINTF_T_NOUT: u32 = DATA + 0x6000;

    pub(crate) const USTR_VSPRINTF_T_OUT: u32 = DATA + 0x8000;

    /// Builds a random format string with matching arguments: literals,
    /// flags, digit and `*` widths and precisions (including `.` alone and
    /// the `.0` quirk that prints the `0`), every conversion of both jump
    /// tables, unknown conversions, `%%`, null `%s`/`%S` and sometimes a
    /// dangling `%` at the end.
    pub(crate) fn ustr_vsprintf_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x3dc4_5000_0000 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        let mut fmt: Vec<u16> = Vec::new();
        let mut args = USTR_VSPRINTF_T_ARGS;
        let mut strs = USTR_VSPRINTF_T_STRS;
        let mut nout = USTR_VSPRINTF_T_NOUT;
        // One 8-byte va_list slot: the value and junk in the upper word.
        let push = |c: &mut Cpu, args: &mut u32, lo: u32, hi: u32| {
            c.bus.write32(*args, lo);
            c.bus.write32(*args + 4, hi);
            *args += 8;
        };
        let lits: &[u16] = &[
            b'a' as u16,
            b'Z' as u16,
            b' ' as u16,
            b'.' as u16,
            b'*' as u16,
            b'7' as u16,
            b'0' as u16,
            b'x' as u16,
            0x263a,
            0xff21,
        ];
        let convs = b"cCdiouxXpPeEfgGnsS%qzhDAt";
        let tokens = 1 + r.next_u32() % 6;
        for _ in 0..tokens {
            if r.next_u32() % 4 == 0 {
                for _ in 0..1 + r.next_u32() % 4 {
                    fmt.push(lits[(r.next_u32() % lits.len() as u32) as usize]);
                }
                continue;
            }
            fmt.push(b'%' as u16);
            for _ in 0..r.next_u32() % 4 {
                fmt.push(b" #+-0"[(r.next_u32() % 5) as usize] as u16);
            }
            match r.next_u32() % 4 {
                1 => {
                    fmt.push(b'1' as u16 + (r.next_u32() % 9) as u16);
                    if r.next_u32() % 2 == 0 {
                        fmt.push(b'0' as u16 + (r.next_u32() % 3) as u16);
                    }
                }
                2 => {
                    fmt.push(b'*' as u16);
                    let w = r.next_u32() % 25;
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, w, hi);
                    }
                }
                _ => {}
            }
            let mut consumes = true;
            match r.next_u32() % 6 {
                1 => {
                    fmt.push(b'.' as u16);
                    fmt.push(b'1' as u16 + (r.next_u32() % 8) as u16);
                }
                2 => {
                    fmt.push(b'.' as u16);
                    fmt.push(b'*' as u16);
                    let p = r.next_u32() % 9;
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, p, hi);
                    }
                }
                3 => fmt.push(b'.' as u16),
                4 => {
                    // ".0": the '0' is printed and the letter is literal text.
                    fmt.push(b'.' as u16);
                    fmt.push(b'0' as u16);
                    consumes = false;
                }
                _ => {}
            }
            let conv = convs[(r.next_u32() % convs.len() as u32) as usize];
            fmt.push(conv as u16);
            if !consumes {
                continue;
            }
            match conv {
                b'c' | b'C' => {
                    let ch = r.next_u32();
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, ch, hi);
                    }
                }
                b'd' | b'i' | b'o' | b'u' | b'x' | b'X' | b'p' | b'P' => {
                    let v = match r.next_u32() % 3 {
                        0 => r.next_u32() % 100,
                        1 => r.next_u32(),
                        _ => (r.next_u32() % 100_000).wrapping_neg(),
                    };
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, v, hi);
                    }
                }
                b'e' | b'E' | b'f' | b'g' | b'G' => {
                    let v = if r.next_u32() % 6 == 0 {
                        0.0
                    } else {
                        (r.next_u32() % 2_000_000) as f64 / 64.0 - 15_625.0
                    };
                    let bits = v.to_bits();
                    push(c, &mut args, bits as u32, (bits >> 32) as u32);
                }
                b'n' => {
                    let at = nout;
                    nout += 4;
                    {
                        let hi = r.next_u32();
                        push(c, &mut args, at, hi);
                    }
                }
                b's' | b'S' => {
                    if r.next_u32() % 5 == 0 {
                        {
                            let hi = r.next_u32();
                            push(c, &mut args, 0, hi);
                        }
                    } else {
                        let at = strs;
                        let n = r.next_u32() % 7;
                        for i in 0..n {
                            if conv == b's' {
                                let ch = 0x20 + (r.next_u32() % 0x2000) as u16;
                                c.bus.write16(at + 2 * i, ch);
                            } else {
                                let ch = 1 + (r.next_u32() % 0xff) as u8;
                                c.bus.write8(at + i, ch);
                            }
                        }
                        if conv == b's' {
                            c.bus.write16(at + 2 * n, 0);
                        } else {
                            c.bus.write8(at + n, 0);
                        }
                        strs += 0x40;
                        {
                            let hi = r.next_u32();
                            push(c, &mut args, at, hi);
                        }
                    }
                }
                _ => {}
            }
        }
        if seed % 11 == 0 {
            fmt.push(b'%' as u16);
        }
        fmt.push(0);
        for (i, ch) in fmt.iter().enumerate() {
            c.bus.write16(USTR_VSPRINTF_T_FMT + 2 * i as u32, *ch);
        }
        c.g[A0][0] = sx(USTR_VSPRINTF_T_OUT);
        c.g[A1][0] = sx(USTR_VSPRINTF_T_FMT);
        c.g[A2][0] = sx(USTR_VSPRINTF_T_ARGS);
        c.pc = USTR_VSPRINTF_START;
    }

    /// `UStr_vsprintf` (printf into a UTF-16 string) against the generated
    /// body, with the real `UStr_FromAscii`, `__floatsidf` and
    /// `UStr_FormatNumberField` on the game's .rodata (jump tables, digits).
    #[test]
    fn ustr_vsprintf_matches() {
        check(
            ssx3_ee::s14::f_003dc450__generated,
            ustr_vsprintf,
            300,
            &ustr_vsprintf_setup,
        );
    }

    /// A leaf that returns 0 (the vtable's "not busy" / "no save").
    pub(crate) const MCU_T_RET0: u32 = 0x0014_06c0;

    /// `cPS2MemCard_SetState`'s jump table (zeroed: its calls become fakes).
    pub(crate) const MCU_T_SET_STATE_TABLE: u32 = 0x0048_60c0;

    /// `sceMcSync` state: the pending command, the RPC header, the result.
    pub(crate) const MCU_T_MC_CMD: u32 = 0x0045_33c0;

    pub(crate) const MCU_T_RPC: u32 = 0x0052_71c0;

    pub(crate) const MCU_T_MC_RESULT: u32 = 0x0051_8780;

    /// The async-file table pointer and count.
    pub(crate) const MCU_T_ASYNC_TABLE: u32 = 0x0045_0c34;

    pub(crate) const MCU_T_ASYNC_COUNT: u32 = 0x0051_9bf0;

    pub(crate) fn mcu_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// A memory card object at `DATA` in state `seed % 83`, with the
    /// `sceMcSync` result, the port infos, the directory listing, the
    /// vtable's busy / has-save answers and the async file chosen per seed.
    pub(crate) fn mcu_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c55d8, 4);
        elf_image(c);
        put_words(c, MCU_T_SET_STATE_TABLE, &[0; 0x4e]);
        c.pc = 0x002c_55d8;
        let me = DATA;
        c.g[A0][0] = me as u64;
        let state = seed % 83;
        let v = seed / 83;
        let w = |c: &mut Cpu, off: u32, x: u32| c.bus.write32(me + off, x);

        // The vtable: 0x140 (invalidate), 0x218 (slot has a save), 0x248 (busy).
        let vt = DATA + 0x4000;
        w(c, 0, vt);
        c.bus.write16(vt + 0x140, (r.next_u32() & 0x18) as u16);
        c.bus.write32(vt + 0x144, FAKE);
        c.bus.write16(vt + 0x218, 0);
        let has_save = if r.next_u32() % 2 == 0 {
            FAKE + 0x10
        } else {
            MCU_T_RET0
        };
        c.bus.write32(vt + 0x21c, has_save);
        c.bus.write16(vt + 0x248, 0);
        let busy = if r.next_u32().wrapping_add(v) % 3 == 0 {
            FAKE + 0x20
        } else {
            MCU_T_RET0
        };
        c.bus.write32(vt + 0x24c, busy);

        w(c, MCU_STATE, state);
        w(c, MCU_PORT, r.next_u32() % 2);
        w(c, MCU_PORT2, r.next_u32() % 2);
        let count = r.next_u32() % 5;
        w(c, MCU_COUNT, count);
        let result = [0u32, 5, 0xffff_ffff][(r.next_u32() % 3) as usize];
        w(c, MCU_RESULT, result);
        w(c, MCU_SLOT, r.next_u32() % 4);
        let written = r.next_u32() % 0x2000;
        w(c, MCU_SIZE, written);
        w(c, MCU_EXPECT, 0x11db8);
        let index = (r.next_u32() % (count + 1)).wrapping_sub(1);
        w(c, MCU_INDEX, index);
        w(c, MCU_BUSY, (r.next_u32() % 2 == 0) as u32);
        w(c, MCU_REQ_1E, (r.next_u32() % 13 == 0) as u32);
        w(c, MCU_REQ_18, (r.next_u32() % 13 == 0) as u32);
        w(c, MCU_DIR_COUNT, r.next_u32() % 4);
        w(c, MCU_LIST_RESULT, r.next_u32());
        let check_word = [0x11db8u32, 0, 7][(r.next_u32() % 3) as usize];
        w(c, MCU_CHECK, check_word);
        c.bus.write32(me + MCU_ICON_ID, r.next_u32());
        c.bus.write32(me + MCU_ICON_ID + 4, r.next_u32());
        mcu_put_str(c, me + MCU_NAME, b"SSX 3 save");
        mcu_put_str(c, me + MCU_DIR, b"BASLUS-20772SSX3");
        for p in 0..2u32 {
            let port = me + MCU_PORTS + p * 0x14;
            let status =
                [0u32, 0xffff_fff6, 0xffff_ffff, 0xffff_fffe, 3][(r.next_u32() % 5) as usize];
            c.bus.write32(port, 2);
            c.bus.write32(port + 4, r.next_u32() % 0x2000);
            c.bus.write32(port + 8, status);
            c.bus.write32(port + 0xc, (r.next_u32() % 3 != 0) as u32);
            c.bus.write32(port + 0x10, 0);
        }
        // Slot entries and the directory listing (names ending in a digit).
        for i in 0..5u32 {
            let e = me + MCU_ENTRIES + i * 0x78;
            mcu_put_str(c, e + 0xc, b"BASLUS-20772SSX31");
            mcu_put_str(c, e + 0x4c, b"SSX 3");
        }
        for j in 0..5u32 {
            let name = me + MCU_DIRBUF + j * 0x40;
            match r.next_u32() % 4 {
                0 => mcu_put_str(c, name, b""),
                1 => mcu_put_str(c, name, b"BADATA-SYSTEM"),
                _ => {
                    let digit = b'1' + (r.next_u32() % 4) as u8;
                    mcu_put_str(c, name, &[b'B', b'A', b'S', b'S', b'X', digit]);
                }
            }
        }

        // gp globals: card type, free space, formatted, last result.
        let gp = crate::difftest::GAME_GP;
        c.bus
            .write32(gp + MCU_GP_TYPE, if r.next_u32() % 3 == 0 { 1 } else { 2 });
        c.bus.write32(gp + MCU_GP_FREE, r.next_u32() % 0x2000);
        c.bus.write32(gp + MCU_GP_FORMAT, r.next_u32() % 2);
        c.bus.write32(gp + MCU_GP_RESULT, r.next_u32());

        // `sceMcSync`: no command (-1), still running (0), or finished with
        // a result (1).
        let results = [
            0u32,
            1,
            5,
            written,
            0xffff_ffff,
            0xffff_fffe,
            0xffff_fffc,
            0xffff_fff6,
            0xffff_fff1,
            0xffff_ffd3,
            0xffff_ffc9,
            0xffff_ffb5,
            0xffff_ff9c,
        ];
        c.bus
            .write32(MCU_T_MC_RESULT, results[(r.next_u32() % 13) as usize]);
        c.bus.write32(MCU_T_RPC, 0);
        match r.next_u32() % 6 {
            0 => c.bus.write32(MCU_T_MC_CMD, 0),
            1 => {
                c.bus.write32(MCU_T_MC_CMD, 0x10);
                let rpc = DATA + 0x6000;
                c.bus.write32(MCU_T_RPC, rpc);
                c.bus.write32(MCU_T_RPC + 4, 0x77);
                c.bus.write32(rpc + 0x18, 0x77);
                c.bus.write32(rpc + 0x10, 1);
            }
            _ => c.bus.write32(MCU_T_MC_CMD, 0x10 + r.next_u32() % 4),
        }

        // The async file: an unknown handle, pending, or done.
        let table = DATA + 0x5000;
        c.bus.write32(MCU_T_ASYNC_TABLE, table);
        c.bus.write32(MCU_T_ASYNC_COUNT, 4);
        let handle = 0x0300 | 2;
        let entry = table + 2 * 0x30;
        c.bus.write32(entry, handle);
        c.bus.write32(entry + 0x8, 0x40);
        c.bus.write32(entry + 0x10, 1);
        c.bus.write32(entry + 0x1c, (r.next_u32() % 3 == 0) as u32);
        c.bus.write32(entry + 0x2c, 0x140);
        w(
            c,
            MCU_HANDLE,
            if r.next_u32() % 4 == 0 { 7 } else { handle },
        );
    }

    #[test]
    fn cps2_mem_card_update_matches() {
        check(
            ee::s08::f_002c55d8__generated,
            cps2_mem_card_update,
            83 * 3,
            &mcu_fixture,
        );
    }

    /// LocStringTable_GetStringPtr: unaligned tables, three languages, ids
    /// negative, in and out of range, with and without the remap table (the
    /// real RemapId and bsearch, ids found or not).
    #[test]
    fn loc_string_table_get_string_ptr_matches() {
        check(
            ee::s14::f_003dbb68__generated,
            loc_string_table_get_string_ptr,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3dbb68, 0);
                c.pc = 0x003d_bb68;
                let this = DATA + 0x1000 + (seed % 4);
                let put = |c: &mut Cpu, at: u32, v: u32| {
                    for k in 0..4 {
                        c.bus.write8(at + k, (v >> (8 * k)) as u8);
                    }
                };
                let lang = seed % 3;
                let remap = 0x101u32;
                put(c, this + 4, remap);
                c.bus.write8(this + 8, (seed % 2) as u8 | 0x10);
                c.bus.write8(this + 0xe, lang as u8);
                c.bus.write8(this + 0xf, 0);
                for l in 0..3u32 {
                    let table = 0x203 + 0x100 * l;
                    put(c, this + 0x10 + 4 * l, table);
                    let count = 5 + l;
                    put(c, this + table + 0xc, count);
                    for i in 0..count {
                        put(c, this + table + 0x10 + 4 * i, 0x40 + 3 * i + l);
                    }
                }
                // The remap table: 6 sorted keys 10, 20, .. mapping to 0..7.
                put(c, this + remap + 8, 6);
                for k in 0..6u32 {
                    c.bus
                        .write16(this + remap + 0x10 + 4 * k, (10 * (k + 1)) as u16);
                    c.bus
                        .write16(this + remap + 0x12 + 4 * k, (r.next_u32() % 8) as u16);
                }
                c.g[A0][0] = sx(this);
                let id: i32 = match seed % 5 {
                    0 => -(1 + (r.next_u32() % 5) as i32),
                    1 => 10 * (1 + (r.next_u32() % 7) as i32),
                    _ => (r.next_u32() % 9) as i32,
                };
                c.g[A1][0] = id as i64 as u64;
            },
        );
    }

    use crate::difftest::RA as SENTINEL;

    /// A cache at a random 16-aligned address with random bytes around, a
    /// random literal pool at `gp - 0x4168 ..` (every pool offset the
    /// function reads is checked) and the 19 curve sources at `gp + 0x7b4`.
    pub(crate) fn spic_fixture(c: &mut Cpu, r: &mut Rng) {
        c.g[GP][0] = GAME_GP as u64;
        let gp = GAME_GP;
        fill(c, r, gp - 0x4170, 0x60);
        let s0 = DATA + 0x1000 + (r.next_u32() % 64) * 16;
        fill(c, r, s0, 0x360);
        for k in 0..SPIC_ROWS {
            let from = DATA + 0x20_0000 + k * 0x40;
            fill(c, r, from, 16);
            c.bus.write32(gp + 0x7b4 + 8 * k, from);
        }
        c.g[A0][0] = sx(s0);
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x002c_2f18;
    }

    /// `SurfPropCache_InitDefaults`: all rows, tuned fields, curve copies,
    /// the registers and cycles left behind (no events).
    #[test]
    fn surf_prop_cache_init_defaults_matches() {
        check(
            ee::s08::f_002c2f18__generated,
            surf_prop_cache_init_defaults,
            24,
            &|c, seed| {
                let mut r = Rng(0x2c2f_1800 ^ seed as u64);
                scramble(c, &mut r);
                spic_fixture(c, &mut r);
            },
        );
    }

    /// Write `units` as UTF-16 at `addr`.
    pub(crate) fn put_units(c: &mut Cpu, addr: u32, units: &[u16]) {
        for (i, &u) in units.iter().enumerate() {
            c.bus.write16(addr + 2 * i as u32, u);
        }
    }

    /// The digit table the formatters read (`0`-`9`, `a`-`f` as UTF-16
    /// units).
    fn put_digits(c: &mut Cpu, addr: u32) {
        let units: Vec<u16> = "0123456789abcdef".encode_utf16().collect();
        put_units(c, addr, &units);
    }

    /// Write `s` as bytes at `addr`, NUL-terminated.
    pub(crate) fn put_ascii(c: &mut Cpu, addr: u32, s: &str) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    driven!(gen_u_str_format_float, ee::s14::f_003dc068__generated);

    driven!(hand_u_str_format_float, u_str_format_float);

    /// Zero, integer, fractional, negative and large magnitudes with
    /// precisions 0..5; the destination is pre-filled.
    #[test]
    fn u_str_format_float_events() {
        check_events(
            gen_u_str_format_float,
            hand_u_str_format_float,
            300,
            &u_str_format_float_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_format_float,
            300,
            &[603, 690, 592, 600, 598, 594],
            &|c, seed| {
                u_str_format_float_events_setup(c, seed);
            },
        );
    }

    /// Gate + model callees for the `u_str_format_float` digit-loop bound
    /// fixture below. In-harness FCVT/STRLEN never produce a real digit
    /// string (strlen answers 0 and a callee clobbers the K slot), so the
    /// exact `no_more` equality never binds. FCVT answers a fixed 5-digit
    /// string with nonzero last digit and STRLEN answers its length; both
    /// leave the caller's stack slot and callee-saved registers alone.
    /// Gate off falls back, so no other test changes behaviour.
    pub(crate) const QT_GATE: u32 = DATA + 0x8_0004;
    pub(crate) const QT_GATE_MAGIC: u32 = 0x0f46_6374;
    pub(crate) const QT_DIGITS: u32 = DATA + 0x8_0000;
    pub(crate) static QT_PREV_FCVT: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static QT_PREV_STRLEN: AtomicUsize = AtomicUsize::new(0);

    fn qt_fake_fcvt(c: &mut Cpu) {
        if c.bus.read32(QT_GATE) != QT_GATE_MAGIC {
            return fakes::fall_back(c, &QT_PREV_FCVT, ee::s15::f_00418d48__generated);
        }
        let ra = c.g[RA][0] as u32;
        for (i, &b) in b"11945".iter().enumerate() {
            c.bus.write8(QT_DIGITS + i as u32, b);
        }
        c.bus.write8(QT_DIGITS + 5, 0);
        c.g[V0][0] = QT_DIGITS as u64;
        c.pc = ra;
    }

    fn qt_fake_strlen(c: &mut Cpu) {
        if c.bus.read32(QT_GATE) != QT_GATE_MAGIC {
            return fakes::fall_back(c, &QT_PREV_STRLEN, ee::s15::f_00416810__generated);
        }
        let ra = c.g[RA][0] as u32;
        let mut len = 0u32;
        while len < 64 && c.bus.read8((c.g[A0][0] as u32).wrapping_add(len)) != 0 {
            len += 1;
        }
        c.g[V0][0] = len as u64;
        c.pc = ra;
    }

    /// Hooks the model callees into their override slots (once per process).
    pub(crate) fn qt_install_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00418D48),
            qt_fake_fcvt,
            &QT_PREV_FCVT,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00416810),
            qt_fake_strlen,
            &QT_PREV_STRLEN,
        );
    }

    /// Sweep precisions 0..7, pinning the saved stack word to K=0x100 so
    /// the remaining-count V1 sweeps a wide window that always binds the
    /// exact no_more equality (A2==V1). The model FCVT/STRLEN fakes above
    /// answer a fixed 5-digit string with nonzero last digit (`qt_install_fakes`
    /// + gate; the harness callees answer 0 and clobber the K slot).
    /// Kills 6844 (`<=`: orig takes no_more at the bound, mutant loops once
    /// more). The nonzero digit bytes kill 6851 (`>>` zeroes what `<<`
    /// sign-extends).
    #[test]
    fn u_str_format_float_exact_matches() {
        check(
            gen_u_str_format_float,
            hand_u_str_format_float,
            32,
            &|c, seed| {
                qt_install_fakes();
                let _r = machine(c, seed, 0x3d_c068, 0);
                for i in 0..48u32 {
                    c.bus.write16(DATA + 2 * i, 0xbeef);
                }
                c.bus.write32(QT_GATE, QT_GATE_MAGIC);
                // Pin the saved word the prologue loads (V0=lw(SP)): shifts
                // the V1 window up so the exact boundary always binds. The
                // model fakes leave this slot alone.
                c.bus.write32(c.g[SP][0] as u32 - 0x70, 0x100);
                c.g[A0][0] = sx(DATA + 96);
                c.g[A1][0] = 0x42f6_e979u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.pc = 0x003d_c068;
            },
        );
    }

    /// Events delivered at the row loop's back edge (0x2c2fa0), delivered
    /// and unwind-requesting.
    #[test]
    fn surf_prop_cache_init_defaults_events_match() {
        check(
            ee::s08::f_002c2f18__generated,
            surf_prop_cache_init_defaults,
            90,
            &|c, seed| {
                let mut r = Rng(0x2c2f_1801 ^ seed as u64);
                scramble(c, &mut r);
                spic_fixture(c, &mut r);
                schedule(c, &mut r, seed, 0x002c_2f18);
            },
        );
    }

    fn u_str_format_float_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_c068, 0);
        let bits = match seed as usize / 6 % 8 {
            0 => 0x0000_0000,
            1 => 0x3f80_0000,
            2 => 0xbf80_0000,
            3 => 0x42f6_e979,
            4 => 0xc2f6_e979,
            5 => 0x4e6e_6b28,
            6 => 0x3a83_126f,
            _ => r.edge_f32(),
        };
        for i in 0..48u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA + 96);
        c.g[A1][0] = bits as u64;
        c.g[A2][0] = (seed % 6) as u64;
        schedule(c, &mut r, seed, 0x003d_c068)
    }

    driven!(
        gen_u_str_format_number_field,
        ee::s14::f_003dc148__generated
    );

    driven!(hand_u_str_format_number_field, u_str_format_number_field);

    /// The flag matrix (0x40 float vs int, 0x80/0x100 float choice, 0x20
    /// unsigned, 0x10/0x8/0x2/0x1 padding and prefix) over bases 8/10/16,
    /// widths, precisions and signed double magnitudes (integer fields;
    /// floating fields format 0.0).
    #[test]
    fn u_str_format_number_field_events() {
        check_events(
            gen_u_str_format_number_field,
            hand_u_str_format_number_field,
            400,
            &u_str_format_number_field_setup,
        );
    }

    fn u_str_format_number_field_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_c148, 0);
        let flags = [
            0x00u32,
            0x20,
            0x40,
            0x40 | 0x80,
            0x40 | 0x100,
            0x18 | 0x10,
            0x08 | 0x20,
            0x01,
            0x05,
            0x02,
            0x20 | 0x01,
            0x40 | 0x80 | 0x10,
            0x40 | 0x04,
        ];
        let bases = [8u32, 10, 16];
        let doubles = [
            0x0000_0000_0000_0000u64,
            0x3ff0_0000_0000_0000,
            0xbff0_0000_0000_0000,
            0x405e_d2f1_a9fc_9722,
            0xc05e_d2f1_a9fc_9722,
            0x432d_f8b5_5818_0000,
            0x3fe0_0000_0000_0000,
            0xbff0_0000_0000_0001,
        ];
        let flag = flags[seed as usize % flags.len()];
        let base = bases[seed as usize / 12 % 3];
        let width = r.next_u32() % 9;
        let prec = r.next_u32() % 7;
        // Floating-point fields format 0.0 only: other doubles run the
        // game's `fcvt` away on this fixture (cycle budget).
        let dbl = if flag & 0x40 != 0 {
            0
        } else {
            doubles[seed as usize / 36 % doubles.len()]
        };
        put_digits(c, DATA + 0x600);
        for i in 0..64u32 {
            c.bus.write16(DATA + 0x200 + 2 * i, 0xbeef);
        }
        // The write cursor: the field goes to `DATA + 0x200`.
        c.bus.write32(DATA + 0x100, DATA + 0x200);
        c.g[A0][0] = sx(DATA + 0x100);
        c.g[A1][0] = dbl;
        c.g[A2][0] = base as u64;
        c.g[A3][0] = flag as u64;
        c.g[T0][0] = width as u64;
        c.g[T1][0] = prec as u64;
        c.g[T2][0] = sx(DATA + 0x600);
        c.f[12] = dbl as u32;
        c.f[13] = (dbl >> 32) as u32;
        // Nonzero doubles go through `__fixdfsi`/`__cmpdf2`, whose loops would
        // take an unwind mid-call (the test machine cannot resume there):
        // deliver instead.
        let ev = if dbl != 0 && seed % 3 == 1 {
            seed + 1
        } else {
            seed
        };
        schedule(c, &mut r, ev, 0x003d_c148)
    }

    /// The fixture reaches the sign prefixes: `-` (signed, negative) and
    /// `+` (flag 0x1, positive).
    #[test]
    fn u_str_format_number_field_signs() {
        let (mut minus, mut plus) = (0, 0);
        for seed in 0..400u32 {
            let mut c = crate::difftest::fresh();
            u_str_format_number_field_setup(&mut c, seed);
            c.next_event = crate::difftest::CYCLE_BUDGET;
            crate::difftest::run_generated(&mut c, gen_u_str_format_number_field);
            let units: Vec<u16> = (0..64u32)
                .map(|i| c.bus.read16(DATA + 0x200 + 2 * i))
                .collect();
            minus += units.contains(&0x2d) as u32;
            plus += units.contains(&0x2b) as u32;
        }
        assert!(
            minus > 0 && plus > 0,
            "sign prefixes: {minus} `-`, {plus} `+`"
        );
    }

    driven!(gen_u_str_sprintf, ee::s14::f_003dc9f8__generated);

    driven!(hand_u_str_sprintf, u_str_sprintf);

    /// `%d`/`%u`/`%x`/`%s`/`%c` formats with register varargs; the
    /// destination is pre-filled and the game image is loaded for the
    /// formatter's table.
    #[test]
    fn u_str_sprintf_events() {
        check_events(
            gen_u_str_sprintf,
            hand_u_str_sprintf,
            300,
            &u_str_sprintf_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_sprintf,
            300,
            &[323, 353, 333, 363],
            &|c, seed| {
                u_str_sprintf_events_setup(c, seed);
            },
        );
    }

    fn u_str_sprintf_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_c9f8, 0);
        elf_image(c);
        let fmts = [
            "n=%d",
            "u=%u x=%x",
            "s=%s c=%c",
            "%d%d%d",
            "plain",
            "mix %s %d %x",
        ];
        let fmt = fmts[seed as usize % fmts.len()];
        put_ascii(c, DATA + 0x400, fmt);
        put_ascii(c, DATA + 0x800, "hi");
        for i in 0..64u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = seed as u64;
        c.g[A3][0] = sx(DATA + 0x800);
        c.g[T0][0] = (seed / 7) as u64;
        c.g[T1][0] = 0x41;
        c.g[T2][0] = (seed / 13 % 256) as u64;
        c.g[T3][0] = 0x1234_5678;
        c.f[12] = seed;
        schedule(c, &mut r, seed, 0x003d_c9f8)
    }

    driven!(gen_u_str_from_escaped_ascii, ee::s14::f_003dca40__generated);

    driven!(hand_u_str_from_escaped_ascii, u_str_from_escaped_ascii);

    /// Plain text, valid `\xHH` escapes (1-4 digits), lone backslashes,
    /// `\x` NUL, invalid hex and empty input.
    #[test]
    fn u_str_from_escaped_ascii_events() {
        check_events(
            gen_u_str_from_escaped_ascii,
            hand_u_str_from_escaped_ascii,
            300,
            &u_str_from_escaped_ascii_setup,
        );
        // Arm totals: plain, valid escapes, lone backslash, invalid hex,
        // empty, `\x` NUL, text then `\x` NUL, a hex run ended by `:` or `g`.
        crate::difftest::require_paths(
            gen_u_str_from_escaped_ascii,
            300,
            &[92, 186, 221, 138, 22, 29, 57, 97],
            &|c, seed| {
                u_str_from_escaped_ascii_setup(c, seed);
            },
        );
    }

    fn u_str_from_escaped_ascii_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_ca40, 0);
        let srcs = [
            "plain",
            "\\x41\\x42",
            "a\\x43b",
            "lone\\backslash",
            "bad\\xzz",
            "\\x41",
            "trail\\",
            "\\",
            "",
            "\\x41424344",
            "mix\\x41z\\x42",
            "\\x",
            "ab\\x",
            "\\x3:",
            "\\x1g",
        ];
        let mut src = srcs[seed as usize % srcs.len()].to_string();
        if seed / 13 % 3 == 0 && !src.ends_with('x') {
            src.push((b'a' + seed as u8 % 26) as char);
        }
        put_ascii(c, DATA + 0x400, &src);
        for i in 0..48u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        schedule(c, &mut r, seed, 0x003d_ca40)
    }

    driven!(gen_u_str_to_escaped_ascii, ee::s14::f_003dcb20__generated);

    driven!(hand_u_str_to_escaped_ascii, u_str_to_escaped_ascii);

    /// ASCII, wide units, `\x` pairs, lone backslashes and empty input;
    /// the game image is loaded for the `%`-format string.
    #[test]
    fn u_str_to_escaped_ascii_events() {
        check_events(
            gen_u_str_to_escaped_ascii,
            hand_u_str_to_escaped_ascii,
            300,
            &u_str_to_escaped_ascii_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_to_escaped_ascii,
            300,
            &[58, 871, 878, 50, 39, 20, 1713, 889],
            &|c, seed| {
                u_str_to_escaped_ascii_events_setup(c, seed);
            },
        );
    }

    fn u_str_to_escaped_ascii_events_setup(
        c: &mut Cpu,
        seed: u32,
    ) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_cb20, 0);
        elf_image(c);
        let srcs: &[&[u16]] = &[
            &[0x41, 0x42, 0x43],
            &[0x141, 0x42],
            &[0x5c, 0x78, 0x41],
            &[0x5c, 0x41],
            &[0x5c],
            &[],
            &[0x20, 0x100, 0x5c, 0x78],
            &[0x41, 0x5c, 0x78, 0x42],
        ];
        let src = srcs[seed as usize % srcs.len()];
        put_units(c, DATA + 0x400, src);
        c.bus.write16(DATA + 0x400 + 2 * src.len() as u32, 0);
        for i in 0..64u32 {
            c.bus.write8(DATA + i, 0xa5);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        schedule(c, &mut r, seed, 0x003d_cb20)
    }

    /// Haystack/needle pairs: found at 0, found late, absent, empty
    /// needle, empty haystack, needle longer than the haystack. No
    /// events: the call into the dispatcher-shaped `UStr_NCmp` cannot
    /// unwind mid-call in the test machine, so the wrapper is compared
    /// without events (its own loop still runs every arm; `UStr_NCmp`'s
    /// event test covers re-entry at its head).
    #[test]
    fn u_str_find_matches() {
        // Arm totals: empty haystack, empty needle, found at 0, found at
        // index 3, absent.
        crate::difftest::check_paths(
            ee::s14::f_003dbd18__generated,
            u_str_find,
            200,
            &[45, 42, 60, 135, 113],
            &u_str_find_setup,
        );
    }

    /// The scan with events due (fixture `seed / 3`, so every haystack
    /// shape meets every event mode). Off and Deliver only: an unwind
    /// inside `UStr_NCmp` (a dispatcher) cannot resume mid-call in the
    /// test machine, so Unwind seeds deliver instead.
    #[test]
    fn u_str_find_events() {
        check_events(
            ee::s14::f_003dbd18__generated,
            u_str_find,
            240,
            &|c, seed| {
                u_str_find_setup(c, seed / 3);
                let mut r = crate::difftest::Rng(0x3d_bd18 ^ (seed as u64) << 8);
                let ev = if seed % 3 == 1 { seed + 1 } else { seed };
                schedule(c, &mut r, ev, 0x003d_bd18)
            },
        );
    }

    fn u_str_find_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3d_bd18, 0);
        let hay = DATA;
        let ndl = DATA + 0x400;
        // Found at 0, found at the end, or (usually) absent.
        let mode = seed % 3;
        let hlen = seed / 3 % 21;
        let nlen = r.next_u32() % 9;
        let mut needle = vec![0u16; nlen as usize];
        for u in needle.iter_mut() {
            let b = r.next_u32();
            *u = (1 + b % 0x2ff) as u16;
        }
        let mut haystack = vec![0u16; hlen as usize];
        for u in haystack.iter_mut() {
            let b = r.next_u32();
            *u = (1 + b % 0x2ff) as u16;
        }
        if mode == 0 && nlen <= hlen && nlen > 0 {
            haystack[..nlen as usize].copy_from_slice(&needle);
        }
        if mode == 1 && nlen <= hlen && nlen > 0 {
            haystack[hlen as usize - nlen as usize..].copy_from_slice(&needle);
        }
        put_units(c, hay, &haystack);
        c.bus.write16(hay + 2 * hlen, 0);
        put_units(c, ndl, &needle);
        c.bus.write16(ndl + 2 * nlen, 0);
        // Pre-fill past the ends so an over-read shows.
        c.bus.write16(hay + 2 * hlen + 2, 0xbeef);
        c.bus.write16(ndl + 2 * nlen + 2, 0xbeef);
        c.g[A0][0] = sx(hay);
        c.g[A1][0] = sx(ndl);
        c.pc = 0x003d_bd18;
    }

    /// Equal, early/late difference, prefix, empty strings. Plain `check`
    /// for the same dispatcher-callee reason as [`u_str_find_matches`].
    #[test]
    fn u_str_cmp_matches() {
        crate::difftest::check_paths(
            ee::s14::f_003dbdb0__generated,
            u_str_cmp,
            200,
            &[18, 29, 25, 36],
            &u_str_cmp_matches_setup,
        );
    }

    fn u_str_cmp_matches_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3d_bdb0, 0);
        let len = seed % 24;
        let mode = seed / 24 % 5;
        let mut a = vec![0u16; len as usize];
        for u in a.iter_mut() {
            *u = (1 + r.next_u32() % 0x2ff) as u16;
        }
        let mut b = a.clone();
        match mode {
            1 => {
                if len > 0 {
                    b[0] = b[0].wrapping_add(1);
                }
            }
            2 => {
                if len > 0 {
                    b[len as usize - 1] = b[len as usize - 1].wrapping_add(1);
                }
            }
            3 => {
                b.push(0x41);
            }
            4 => {
                a.clear();
            }
            _ => {}
        }
        put_units(c, DATA, &a);
        c.bus.write16(DATA + 2 * a.len() as u32, 0);
        put_units(c, DATA + 0x400, &b);
        c.bus.write16(DATA + 0x400 + 2 * b.len() as u32, 0);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.pc = 0x003d_bdb0;
    }

    driven!(gen_u_str_n_cmp, ee::s14::f_003dbdd0__generated);
    crate::game::lib::eafile::tests::ea_counted!(counted_u_str_n_cmp, u_str_n_cmp);
    driven!(hand_u_str_n_cmp, counted_u_str_n_cmp);

    /// Bounded compares: max 0, max below/above the difference point,
    /// NUL inside the window, empty inputs.
    #[test]
    fn u_str_n_cmp_events() {
        check_events(
            gen_u_str_n_cmp,
            hand_u_str_n_cmp,
            300,
            &u_str_n_cmp_events_setup,
        );
        // The hand side itself resumes at the loop head.
        crate::game::lib::eafile::tests::ea_assert_entered(&[0x003d_bdd0, 0x003d_bde8]);
        crate::difftest::require_paths(gen_u_str_n_cmp, 300, &[5, 15, 17, 28, 10], &|c, seed| {
            u_str_n_cmp_events_setup(c, seed);
        });
    }

    fn u_str_n_cmp_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_bdd0, 0);
        let len = 1 + seed % 16;
        let max = [0, 1, 3, 7, 100][seed as usize / 17 % 5];
        let mode = seed / 60 % 5;
        let mut a = vec![0u16; len as usize];
        for u in a.iter_mut() {
            *u = (1 + r.next_u32() % 0x2ff) as u16;
        }
        let mut b = a.clone();
        match mode {
            1 => {
                b[0] = b[0].wrapping_add(1);
            }
            2 => {
                let i = len as usize / 2;
                b[i] = b[i].wrapping_add(1);
            }
            3 => {
                a[len as usize / 2] = 0;
            }
            4 => {
                a.clear();
                b.clear();
            }
            _ => {}
        }
        put_units(c, DATA, &a);
        c.bus.write16(DATA + 2 * a.len() as u32, 0);
        put_units(c, DATA + 0x400, &b);
        c.bus.write16(DATA + 0x400 + 2 * b.len() as u32, 0);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = max;
        schedule(c, &mut r, seed, 0x003d_bdd0)
    }

    driven!(gen_u_str_cpy, ee::s14::f_003dbe20__generated);
    driven!(hand_u_str_cpy, u_str_cpy);

    /// Copies of empty, short and 23-unit strings; the destination is
    /// pre-filled so a missing store shows.
    #[test]
    fn u_str_cpy_events() {
        check_events(gen_u_str_cpy, hand_u_str_cpy, 200, &u_str_cpy_events_setup);
        crate::difftest::require_paths(gen_u_str_cpy, 200, &[17, 27, 225], &|c, seed| {
            u_str_cpy_events_setup(c, seed);
        });
    }

    fn u_str_cpy_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_be20, 0);
        let len = seed % 24;
        let mut s = vec![0u16; len as usize];
        for u in s.iter_mut() {
            *u = (1 + r.next_u32() % 0x2ff) as u16;
        }
        put_units(c, DATA + 0x400, &s);
        c.bus.write16(DATA + 0x400 + 2 * len, 0);
        for i in 0..32u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        schedule(c, &mut r, seed, 0x003d_be20)
    }

    /// Max 0 (nothing copied) and max 5 with an empty source (the empty
    /// copy) or a short one. Split: edge arms under `check`, the looping
    /// arms under `check_events` so enough seeds reach the event check.
    #[test]
    fn u_str_n_cpy_edges() {
        // Arm totals: max 0, empty source, short sources.
        crate::difftest::check_paths(
            ee::s14::f_003dbe40__generated,
            u_str_n_cpy,
            32,
            &[6, 9, 52, 37],
            &u_str_n_cpy_edges_setup,
        );
    }

    fn u_str_n_cpy_edges_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3d_be40, 0);
        let len = 2 + seed % 6;
        let mut s = vec![0u16; len as usize];
        for u in s.iter_mut() {
            *u = (1 + r.next_u32() % 0x2ff) as u16;
        }
        if seed % 2 == 0 {
            s.clear();
        }
        put_units(c, DATA + 0x400, &s);
        c.bus.write16(DATA + 0x400 + 2 * s.len() as u32, 0);
        for i in 0..32u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = [0u64, 5][seed as usize / 2 % 2];
        c.pc = 0x003d_be40;
    }

    driven!(gen_u_str_n_cpy_loop, ee::s14::f_003dbe40__generated);
    driven!(hand_u_str_n_cpy_loop, u_str_n_cpy);

    /// Truncated, exact and over-long copies over 8..31 units; the
    /// destination is pre-filled.
    #[test]
    fn u_str_n_cpy_loop_events() {
        check_events(
            gen_u_str_n_cpy_loop,
            hand_u_str_n_cpy_loop,
            200,
            &u_str_n_cpy_loop_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_n_cpy_loop,
            200,
            &[43, 88, 82, 160],
            &|c, seed| {
                u_str_n_cpy_loop_events_setup(c, seed);
            },
        );
    }

    fn u_str_n_cpy_loop_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_be40, 1);
        let len = 8 + seed % 24;
        let max = [4u32, 9, 17, 100][seed as usize / 32 % 4];
        let mut s = vec![0u16; len as usize];
        for u in s.iter_mut() {
            *u = (1 + r.next_u32() % 0x2ff) as u16;
        }
        put_units(c, DATA + 0x400, &s);
        c.bus.write16(DATA + 0x400 + 2 * len, 0);
        for i in 0..40u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = max as u64;
        schedule(c, &mut r, seed, 0x003d_be40)
    }

    driven!(gen_u_str_from_ascii, ee::s14::f_003dbe90__generated);
    driven!(hand_u_str_from_ascii, u_str_from_ascii);

    /// Empty, short and 31-byte sources with high bytes; the destination
    /// is pre-filled.
    #[test]
    fn u_str_from_ascii_events() {
        check_events(
            gen_u_str_from_ascii,
            hand_u_str_from_ascii,
            200,
            &u_str_from_ascii_events_setup,
        );
        crate::difftest::require_paths(gen_u_str_from_ascii, 200, &[7, 15, 225], &|c, seed| {
            u_str_from_ascii_events_setup(c, seed);
        });
    }

    fn u_str_from_ascii_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_be90, 0);
        let len = seed % 32;
        for i in 0..len {
            let b = match r.next_u32() % 4 {
                0 => 0x80 + r.next_u32() % 0x80,
                _ => 1 + r.next_u32() % 0x7e,
            };
            c.bus.write8(DATA + 0x400 + i, b as u8);
        }
        c.bus.write8(DATA + 0x400 + len, 0);
        for i in 0..40u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        schedule(c, &mut r, seed, 0x003d_be90)
    }

    driven!(
        gen_u_str_substitute_args_wrapper,
        ee::s14::f_003dbed0__generated
    );
    driven!(
        hand_u_str_substitute_args_wrapper,
        u_str_substitute_args_wrapper
    );

    /// Formats with no escapes, `%1`/`%2`, `%x`, a trailing `%` and an
    /// empty format; the destinations are pre-filled.
    #[test]
    fn u_str_substitute_args_wrapper_events() {
        check_events(
            gen_u_str_substitute_args_wrapper,
            hand_u_str_substitute_args_wrapper,
            300,
            &u_str_substitute_args_wrapper_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_substitute_args_wrapper,
            300,
            &[92, 166, 168, 234, 129, 44, 131],
            &|c, seed| {
                u_str_substitute_args_wrapper_events_setup(c, seed);
            },
        );
    }

    fn u_str_substitute_args_wrapper_events_setup(
        c: &mut Cpu,
        seed: u32,
    ) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_bed0, 0);
        let fmts = [
            "score ",
            "p1=%1 ok",
            "%2/%1",
            "%1%2%1",
            "100%x done",
            "trailing %",
            "",
            "%%1",
        ];
        let fmt = fmts[seed as usize / 37 % fmts.len()];
        let mut f: Vec<u16> = fmt.encode_utf16().collect();
        if seed / 296 % 2 == 0 {
            f.push(b'x' as u16);
        }
        put_units(c, DATA + 0x400, &f);
        c.bus.write16(DATA + 0x400 + 2 * f.len() as u32, 0);
        put_units(c, DATA + 0x800, &[0x41, 0x42, 0]);
        put_units(c, DATA + 0xc00, &[0x58, 0x59, 0x5a, 0]);
        for i in 0..64u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = sx(DATA + 0x800);
        schedule(c, &mut r, seed, 0x003d_bed0)
    }

    driven!(gen_u_str_substitute_args, ee::s14::f_003dbef0__generated);
    driven!(hand_u_str_substitute_args, u_str_substitute_args);

    /// Same shapes as the wrapper, with a real second argument.
    #[test]
    fn u_str_substitute_args_events() {
        check_events(
            gen_u_str_substitute_args,
            hand_u_str_substitute_args,
            300,
            &u_str_substitute_args_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_substitute_args,
            300,
            &[77, 151, 203, 269, 114, 26, 134],
            &|c, seed| {
                u_str_substitute_args_events_setup(c, seed);
            },
        );
    }

    fn u_str_substitute_args_events_setup(
        c: &mut Cpu,
        seed: u32,
    ) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_bef0, 0);
        let fmts = [
            "score ",
            "p1=%1 ok",
            "%2/%1",
            "%1%2%1",
            "100%x done",
            "trailing %",
            "",
            "%%2",
        ];
        let fmt = fmts[seed as usize / 37 % fmts.len()];
        let f: Vec<u16> = fmt.encode_utf16().collect();
        put_units(c, DATA + 0x400, &f);
        c.bus.write16(DATA + 0x400 + 2 * f.len() as u32, 0);
        put_units(c, DATA + 0x800, &[0x41, 0x42, 0]);
        put_units(c, DATA + 0xc00, &[0x58, 0x59, 0x5a, 0]);
        for i in 0..64u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = sx(DATA + 0x800);
        c.g[A3][0] = sx(DATA + 0xc00);
        schedule(c, &mut r, seed, 0x003d_bef0)
    }

    driven!(gen_u_str_format_u_int, ee::s14::f_003dbfd8__generated);
    driven!(hand_u_str_format_u_int, u_str_format_u_int);

    /// Zero, small and 32-bit values in bases 8/10/16; the destination is
    /// pre-filled and the write runs backwards from its end.
    #[test]
    fn u_str_format_u_int_events() {
        check_events(
            gen_u_str_format_u_int,
            hand_u_str_format_u_int,
            300,
            &u_str_format_u_int_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_format_u_int,
            300,
            &[8, 20, 34, 118, 146, 160],
            &|c, seed| {
                u_str_format_u_int_events_setup(c, seed);
            },
        );
    }

    fn u_str_format_u_int_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_bfd8, 0);
        let bases = [8u32, 10, 16];
        let base = bases[seed as usize % 3];
        let value = match seed as usize / 3 % 6 {
            0 => 0,
            1 => seed,
            2 => 0xffff_ffff,
            3 => 0x8000_0000,
            4 => 123456789,
            _ => r.next_u32(),
        };
        put_digits(c, DATA + 0x400);
        for i in 0..40u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA + 80);
        c.g[A1][0] = value as u64;
        c.g[A2][0] = base as u64;
        c.g[A3][0] = sx(DATA + 0x400);
        schedule(c, &mut r, seed, 0x003d_bfd8)
    }

    driven!(gen_u_str_format_int, ee::s14::f_003dc038__generated);
    driven!(hand_u_str_format_int, u_str_format_int);

    /// Negative, zero and positive values (the sign is the caller's job;
    /// this formats the absolute value).
    #[test]
    fn u_str_format_int_events() {
        check_events(
            gen_u_str_format_int,
            hand_u_str_format_int,
            200,
            &u_str_format_int_events_setup,
        );
        crate::difftest::require_paths(
            gen_u_str_format_int,
            200,
            &[19, 31, 45, 157],
            &|c, seed| {
                u_str_format_int_events_setup(c, seed);
            },
        );
    }

    fn u_str_format_int_events_setup(c: &mut Cpu, seed: u32) -> crate::difftest::events::Events {
        let mut r = machine(c, seed, 0x3d_c038, 0);
        let value = match seed as usize % 6 {
            0 => 0,
            1 => seed as i32,
            2 => -(seed as i32),
            3 => i32::MIN,
            4 => -1,
            _ => i32::MAX,
        };
        put_digits(c, DATA + 0x400);
        for i in 0..40u32 {
            c.bus.write16(DATA + 2 * i, 0xbeef);
        }
        c.g[A0][0] = sx(DATA + 80);
        // Sign-extended, as a 32-bit value sits in a register.
        c.g[A1][0] = value as i64 as u64;
        c.g[A2][0] = 10;
        c.g[A3][0] = sx(DATA + 0x400);
        schedule(c, &mut r, seed, 0x003d_c038)
    }

    use crate::difftest::fakes;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc554_gate_on;

    #[test]
    fn game_link_get_ticks_matches() {
        check(
            ee::s08::f_002c28a0__generated,
            game_link_get_ticks,
            100,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0015);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn game_link_alloc_matches() {
        check(
            ee::s08::f_002c28c0__generated,
            game_link_alloc,
            200,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0016);
                c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) | 1
                };
            },
        );
    }

    /// Gate word in the machine's RAM: the callee fakes below run only when it
    /// is set (otherwise they run the callee they replaced).
    pub(crate) const OC555_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC555_MAGIC: u32 = 0x0c55_5a5a;

    /// The fake-call index whose result is forced to 0 (u32::MAX: none).
    pub(crate) const OC555_ZERO_AT: u32 = DATA + 0x20_0010;

    /// Fixtures: the process, its manager and the definition block.
    pub(crate) const OC555_SELF: u32 = DATA + 0x21_0000;

    /// Hooks the fakes in the callees' override slots (once per process).
    pub(crate) fn oc555_setup_fakes() {
        crate::game::wscript::wscriptprocess::tests::oc555_install_fakes();
    }

    /// Fake machine state for one seed: scrambled registers, the fake hooks,
    /// the gate on, and the forced-zero call index.
    pub(crate) fn oc555_gate(c: &mut Cpu, seed: u32, salt: u64, zero_at: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, 4);
        c.bus.write32(OC555_CTRL, OC555_MAGIC);
        c.bus.write32(OC555_ZERO_AT, zero_at);
        r
    }

    #[test]
    fn game_link_free_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c28f8__generated,
            game_link_free,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c28f8, u32::MAX);
                c.pc = 0x002c_28f8;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_mem_copy_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2918__generated,
            game_link_mem_copy,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2918, u32::MAX);
                c.pc = 0x002c_2918;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_realloc_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2940__generated,
            game_link_realloc,
            300,
            &|c, seed| {
                let zero_at = [u32::MAX, 0, 1, 2][(seed % 4) as usize];
                let mut r = oc555_gate(c, seed, 0x2c2940, zero_at);
                c.pc = 0x002c_2940;
                // block: null on a third of the seeds; size: zero on a quarter.
                let block = if seed % 3 == 0 { 0 } else { fakes::ptr(&mut r) };
                let size = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 0x1000
                };
                c.g[A0][0] = sx(block);
                c.g[A2][0] = sx(size);
            },
        );
    }

    #[test]
    fn game_link_create_thread_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c29d0__generated,
            game_link_create_thread,
            300,
            &|c, seed| {
                // 0: the thread block is null, 1: the stack is null,
                // 2: THREAD_createparam fails, otherwise nothing fails.
                let zero_at = [u32::MAX, 0, 1, 2][(seed % 4) as usize];
                let mut r = oc555_gate(c, seed, 0x2c29d0, zero_at);
                c.pc = 0x002c_29d0;
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(1 + r.next_u32() % 0x1000);
            },
        );
    }

    #[test]
    fn game_link_destroy_thread_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2aa8__generated,
            game_link_destroy_thread,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2aa8, u32::MAX);
                c.pc = 0x002c_2aa8;
                let this = if seed % 7 == 0 { 0 } else { OC555_SELF };
                // *self: the stack pointer, set or clear.
                let stack = if seed % 3 == 0 { 0 } else { fakes::ptr(&mut r) };
                if this != 0 {
                    c.bus.write32(this, stack);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn game_link_yield_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b08__generated,
            game_link_yield,
            100,
            &|c, seed| {
                let _r = oc555_gate(c, seed, 0x2c2b08, u32::MAX);
                c.pc = 0x002c_2b08;
            },
        );
    }

    #[test]
    fn game_link_create_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b28__generated,
            game_link_create_mutex,
            200,
            &|c, seed| {
                // 0: the mutex block is null (MUTEX_create runs on address 0x10).
                let zero_at = if seed % 4 == 0 { 0 } else { u32::MAX };
                let _r = oc555_gate(c, seed, 0x2c2b28, zero_at);
                c.pc = 0x002c_2b28;
            },
        );
    }

    #[test]
    fn game_link_destroy_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b68__generated,
            game_link_destroy_mutex,
            200,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2b68, u32::MAX);
                c.pc = 0x002c_2b68;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_lock_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2b98__generated,
            game_link_lock_mutex,
            100,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2b98, u32::MAX);
                c.pc = 0x002c_2b98;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_unlock_mutex_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2bb8__generated,
            game_link_unlock_mutex,
            100,
            &|c, seed| {
                let mut r = oc555_gate(c, seed, 0x2c2bb8, u32::MAX);
                c.pc = 0x002c_2bb8;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    #[test]
    fn game_link_setup_platform_funcs_matches() {
        oc555_setup_fakes();
        check(
            ee::s08::f_002c2bd8__generated,
            game_link_setup_platform_funcs,
            100,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2c2bd8, 0);
                c.pc = 0x002c_2bd8;
                c.g[A0][0] = sx(fakes::ptr(&mut r));
            },
        );
    }

    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fresh;
    use crate::difftest::require_paths;
    use crate::difftest::run_generated;
    use crate::difftest::SP as STACK_TOP;
    use std::collections::HashMap;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate words of the `cPS2MemCard_SetState` fakes: one 16-byte slot each
    /// from here, all holding this magic.
    pub(crate) const SET_STATE_GATES: u32 = DATA + 0xf_f000;

    pub(crate) const SET_STATE_MAGIC: u32 = 0x4d43_5353;

    /// What each faked callee and the slot query charge (fixed, so the path
    /// totals can be added up).
    pub(crate) const SET_STATE_FAKE_CYCLES: u64 = 5;

    pub(crate) const SET_STATE_QUERY_CYCLES: u64 = 4;

    /// The card object (fields at non-zero low bytes), its vtable and the
    /// slot query's answers (word i: slot i holds a save).
    pub(crate) const SET_STATE_SELF: u32 = DATA + 0x1238;

    pub(crate) const SET_STATE_VTABLE: u32 = DATA + 0x3468;

    pub(crate) const SET_STATE_SLOT_SAVES: u32 = DATA + 0x3800;

    /// The read/write buffer (`+0x2c`, size `+0x20`), the loaded icon
    /// (`+0x30`, size `+0x34`), the port and the open file's descriptor.
    pub(crate) const SET_STATE_BUFFER: u32 = DATA + 0x4123;

    pub(crate) const SET_STATE_BUFFER_SIZE: u32 = 0x25;

    pub(crate) const SET_STATE_ASYNC: u32 = DATA + 0x4561;

    pub(crate) const SET_STATE_ASYNC_SIZE: u32 = 0x13;

    pub(crate) const SET_STATE_PORT: u32 = 1;

    pub(crate) const SET_STATE_FD: u32 = 3;

    /// Three different icon file name pointers (the game's are all
    /// 0x4860a8, "ssx1.ico").
    pub(crate) const SET_STATE_ICON_FILES: [u32; 3] = [0x0048_60a8, DATA + 0x4a10, DATA + 0x4a50];

    /// libmc's RPC client (+0x14 the server's receive buffer, +0x24 nonzero
    /// once bound), libkernel's SIF RPC packet pool (+0 request counter, +4
    /// the packets, +8 their number), the IOP command buffer word, and two
    /// fixture packets.
    pub(crate) const SET_STATE_MC_CLIENT: u32 = 0x0052_71c0;

    pub(crate) const SET_STATE_SIF_RPC_POOL: u32 = 0x0052_d680;

    pub(crate) const SET_STATE_SIF_CMD_WORD: u32 = 0x0052_bce0;

    pub(crate) const SET_STATE_RPC_PACKETS: u32 = DATA + 0x6000;

    /// The states the fixture enters: 0..=0x4f, two more out of range
    /// (0xffff_ffff, 0x8000_0001) and 0x26 with junk in a1's high word.
    pub(crate) const SET_STATE_VALUES: u32 = 0x53;

    pub(crate) fn set_state_value(k: u32) -> u64 {
        match k {
            0..=0x4f => k as u64,
            0x50 => 0xffff_ffff,
            0x51 => 0x8000_0001,
            _ => 0x5a5a_0000_0000_0026,
        }
    }

    /// The fixture's variants: bit 0 libmc answered; 0-1 the title break and
    /// busy word off (title break 0 for 0, negative for 1), 2-5 on; a file
    /// open for 2-5 (descriptor 3 for 2, 3; 0 for 4, 5); an icon loaded for
    /// 1, 2 and 4 (so every pair of the two shows).
    pub(crate) const SET_STATE_VARIANTS: u32 = 6;

    /// Whether variant `variant` has an icon loaded (`+0x30`).
    pub(crate) fn set_state_loaded(variant: u32) -> bool {
        matches!(variant, 1 | 2 | 4)
    }

    /// State 7's slot scans by variant: (slot count, first slot with a save);
    /// the count is -1 for variant 3.
    pub(crate) const SET_STATE_SCANS: [(u32, Option<u32>); 6] = [
        (0, None),
        (1, Some(0)),
        (3, Some(2)),
        (u32::MAX, None),
        (2, Some(1)),
        (5, None),
    ];

    /// A faked callee: log the call (every register), return a pointer (or
    /// unwind at `LOG_UNWIND_AT`) and charge a fixed cost.
    pub(crate) fn set_state_fake_call(c: &mut Cpu, addr: u32) {
        let ra = r32(c, RA);
        let start = c.cycles;
        fakes::fake(c, addr, ra);
        c.cycles = start + SET_STATE_FAKE_CYCLES;
    }

    macro_rules! set_state_fake {
        ($name:ident, $k:expr, $addr:expr, $generated:path) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(SET_STATE_GATES + 0x10 * $k) != SET_STATE_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $generated);
                }
                set_state_fake_call(c, $addr);
            }
        };
    }

    set_state_fake!(
        set_state_fake_self,
        0,
        0x002c_48c0,
        ee::s08::f_002c48c0__generated
    );

    set_state_fake!(
        set_state_fake_get_info,
        1,
        0x002c_50e0,
        ee::s08::f_002c50e0__generated
    );

    set_state_fake!(
        set_state_fake_mark_invalid,
        2,
        0x002c_5570,
        ee::s08::f_002c5570__generated
    );

    set_state_fake!(
        set_state_fake_sjis,
        3,
        0x002c_2210,
        ee::s08::f_002c2210__generated
    );

    set_state_fake!(
        set_state_fake_sprintf,
        4,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    set_state_fake!(
        set_state_fake_mem_set,
        5,
        0x003e_6448,
        ee::s14::f_003e6448__generated
    );

    set_state_fake!(
        set_state_fake_mem_copy,
        6,
        0x003e_6574,
        ee::s14::f_003e6574__generated
    );

    set_state_fake!(
        set_state_fake_strcpy,
        7,
        0x0041_66f4,
        ee::s15::f_004166f4__generated
    );

    set_state_fake!(
        set_state_fake_strlen,
        8,
        0x0041_6810,
        ee::s15::f_00416810__generated
    );

    set_state_fake!(
        set_state_fake_mem_free,
        9,
        0x0025_23a8,
        ee::s06::f_002523a8__generated
    );

    set_state_fake!(
        set_state_fake_async_load,
        10,
        0x003d_f690,
        ee::s14::f_003df690__generated
    );

    set_state_fake!(
        set_state_fake_poll_sema,
        11,
        0x0042_3df0,
        ee::s15::f_00423df0__generated
    );

    pub(crate) const SET_STATE_FAKES: u32 = 12;

    pub(crate) fn set_state_hook(slot: *mut fn(&mut Cpu), fake: fn(&mut Cpu)) {
        fakes::hook(slot, fake, fakes::prev_cell(fake));
    }

    /// Every callee with a slot faked (gated on this machine): `SetState`
    /// itself (the recursive calls), the card's own helpers, the string and
    /// memory helpers, `MEM_Free`, `AsyncFile_Load`, and `PollSema` (the
    /// libmc sema is free).
    pub(crate) fn set_state_fakes_on(c: &mut Cpu) {
        set_state_hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C48C0),
            set_state_fake_self,
        );
        set_state_hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C50E0),
            set_state_fake_get_info,
        );
        set_state_hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C5570),
            set_state_fake_mark_invalid,
        );
        set_state_hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2210),
            set_state_fake_sjis,
        );
        set_state_hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            set_state_fake_sprintf,
        );
        set_state_hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            set_state_fake_mem_set,
        );
        set_state_hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6574),
            set_state_fake_mem_copy,
        );
        set_state_hook(
            addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            set_state_fake_strcpy,
        );
        set_state_hook(
            addr_of_mut!(ee::s15::OVERRIDE_00416810),
            set_state_fake_strlen,
        );
        set_state_hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            set_state_fake_mem_free,
        );
        set_state_hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DF690),
            set_state_fake_async_load,
        );
        set_state_hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DF0),
            set_state_fake_poll_sema,
        );
        for k in 0..SET_STATE_FAKES {
            c.bus.write32(SET_STATE_GATES + 0x10 * k, SET_STATE_MAGIC);
        }
    }

    /// The `miss` hook: the vtable's "slot holds a save" query (at `FAKE`),
    /// logged like a fake, answered from `SET_STATE_SLOT_SAVES[a1]`.
    pub(crate) fn set_state_slot_query(c: &mut Cpu, target: u32) {
        let slot = r32(c, A1);
        let ra = r32(c, RA);
        let start = c.cycles;
        fakes::fake(c, target, ra);
        c.cycles = start + SET_STATE_QUERY_CYCLES;
        c.g[V0][0] = c.bus.read32(SET_STATE_SLOT_SAVES + 4 * (slot & 0xf)) as u64;
    }

    /// libmc unbound (every `sceMc*` returns -100 at once) or answered: bound,
    /// with two free SIF RPC packets, so `sceSifCallRpc` (no wait) sends its
    /// command to plain IOP RAM and the call returns 0.
    pub(crate) fn set_state_libmc(c: &mut Cpu, answered: bool) {
        c.bus.write32(SET_STATE_MC_CLIENT + 0x24, answered as u32);
        if !answered {
            return;
        }
        put_words(c, SET_STATE_SIF_RPC_POOL, &[0x11, SET_STATE_RPC_PACKETS, 2]);
        // Flag word (+0x10) bit 0 clear: both packets free.
        for i in 0..0x20 {
            c.bus.write32(SET_STATE_RPC_PACKETS + 4 * i, 0x5a5a_5a5a);
        }
        c.bus.write32(SET_STATE_SIF_CMD_WORD, 0x0001_f000);
        c.bus.write32(SET_STATE_MC_CLIENT + 0x14, 0x0001_f400);
    }

    /// The open file's descriptor (`+0x18`) by variant: -1 (none), 3, or 0.
    pub(crate) fn set_state_fd(variant: u32) -> u32 {
        match variant {
            0 | 1 => u32::MAX,
            2 | 3 => SET_STATE_FD,
            _ => 0,
        }
    }

    pub(crate) fn set_state_async(loaded: bool) -> u32 {
        if loaded {
            SET_STATE_ASYNC
        } else {
            0
        }
    }

    /// State 7's scan: `count` slots, a save in slot `found` only.
    pub(crate) fn set_state_scan(c: &mut Cpu, count: u32, found: Option<u32>) {
        c.bus.write32(SET_STATE_SELF + MCU_COUNT, count);
        for i in 0..16 {
            c.bus
                .write32(SET_STATE_SLOT_SAVES + 4 * i, (found == Some(i)) as u32);
        }
    }

    /// `SetState(self, state)` on a card object. `variant` bit 0: libmc
    /// answered (the `sceMc*` calls return 0; else -100). Variants 2-5
    /// (`flag`): a file is open (`+0x18` = 3 for 2-3, 0 for 4-5; else -1),
    /// the title has a line break (`+0x8`; negative for 1, else 0), a
    /// request is running (`+0x40`). An icon is loaded (`+0x30`) by
    /// [`set_state_loaded`]. State 7 scans slots by `SET_STATE_SCANS[variant]`.
    pub(crate) fn set_state_fixture(c: &mut Cpu, seed: u32, state: u64, variant: u32) -> Rng {
        let mut r = Rng(0x002c_48c0 ^ (seed as u64) << 20 ^ (variant as u64) << 12);
        elf_image(c);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.hooks.miss = set_state_slot_query;
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        set_state_fakes_on(c);
        fill(c, &mut r, STACK_TOP - 0x100, 0x40);
        let flag = variant >= 2;
        let loaded = set_state_loaded(variant);
        let title_break = match variant {
            0 => 0,
            1 => 0xffff_ffe4,
            _ => 0x1c,
        };
        let this = SET_STATE_SELF;
        fill(c, &mut r, this, 0x1500 / 4);
        c.bus.write32(this, SET_STATE_VTABLE);
        c.bus
            .write16(SET_STATE_VTABLE + MCU_VT_HAS_SLOT_NAME, 0xfff8);
        c.bus
            .write32(SET_STATE_VTABLE + MCU_VT_HAS_SLOT_NAME + 4, FAKE + 0x10);
        for (off, v) in [
            (MCU_TITLE_BREAK, title_break),
            (MCU_PORT, SET_STATE_PORT),
            (MCU_RESULT, set_state_fd(variant)),
            (MCU_SIZE, SET_STATE_BUFFER_SIZE),
            (MCU_INDEX, 2),
            (MCU_BUFFER, SET_STATE_BUFFER),
            (MCU_ASYNC_A, set_state_async(loaded)),
            (MCU_ASYNC_B, SET_STATE_ASYNC_SIZE),
            (MCU_BUSY, flag as u32),
        ] {
            c.bus.write32(this + off, v);
        }
        for (off, s) in [
            (MCU_NAME, "SSX 3"),
            (MCU_PATH, "/BASLUS-20772/icon.sys"),
            (MCU_DIR, "BASLUS-20772"),
            (MCU_FILE, "BASLUS-20772SSX3"),
            (MCU_ICON_DIR, "cdrom0:\\DATA\\MC"),
        ] {
            mcu_put_str(c, this + off, s.as_bytes());
        }
        let (count, found) = SET_STATE_SCANS[variant as usize];
        set_state_scan(c, count, found);
        fill(c, &mut r, SET_STATE_BUFFER & !3, 0x10);
        fill(c, &mut r, SET_STATE_ASYNC & !3, 0x10);
        fill(c, &mut r, MC_ICON_SYS, MC_ICON_SYS_SIZE / 4);
        // The game's three icon file names are one pointer; on odd seeds
        // three different ones, so each state's name index shows.
        if seed & 1 != 0 {
            put_words(c, MC_ICON_FILES, &SET_STATE_ICON_FILES);
        }
        // The last libmc result (`gp + 0x850`, 0 in the image): non-zero, so
        // state 1's copy and every result store change RAM.
        c.bus
            .write32(GAME_GP + MCU_GP_RESULT, r.next_u32() | 0x8000_0001);
        set_state_libmc(c, variant & 1 != 0);
        // On the main guest thread (libkernel's interrupt helpers).
        let regs = c.g;
        ssx3_ps2::kernel::start_main_thread(c, MC_SET_STATE_ENTRY, 0, 0);
        c.g = regs;
        c.pc = MC_SET_STATE_ENTRY;
        // `self` is copied whole into s2 (and from there into the calls'
        // a0): junk in the high word on some seeds.
        let high = if r.next_u32() & 1 != 0 {
            (r.next_u32() as u64 | 1) << 32
        } else {
            0
        };
        c.g[A0][0] = high | this as u64;
        c.g[A1][0] = state;
        r
    }

    /// The cycles a real `sceMc*` call costs on the fixture (unbound or
    /// answered), entered at `addr` with `args` and the caller's `sp`.
    pub(crate) fn set_state_mc_cycles(addr: u32, answered: bool, args: &[(usize, u64)]) -> u64 {
        let mut c = fresh();
        set_state_fixture(&mut c, 0, 0, answered as u32);
        c.g[SP][0] = (STACK_TOP - SET_STATE_FRAME.size) as u64;
        for &(reg, v) in args {
            c.g[reg][0] = v;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        let start = c.cycles;
        ssx3_ee_rt::funcs::call_indirect(&mut c, addr);
        assert_eq!(c.pc, SENTINEL_RA, "sceMc call {addr:#x} did not return");
        c.cycles - start
    }

    /// [`set_state_mc_cycles`] by call.
    pub(crate) struct SetStateMcCosts(pub(crate) HashMap<(u32, bool, Vec<(usize, u64)>), u64>);

    impl SetStateMcCosts {
        pub(crate) fn of(&mut self, addr: u32, answered: bool, args: &[(usize, u64)]) -> u64 {
            *self
                .0
                .entry((addr, answered, args.to_vec()))
                .or_insert_with(|| set_state_mc_cycles(addr, answered, args))
        }
    }

    /// The cycle total of seed `seed`'s path (state `seed % 0x53`, variant
    /// `seed / 0x53 % 6`), added up from the generated body's block charges,
    /// the fakes' fixed costs and the measured `sceMc*` calls.
    pub(crate) fn set_state_expected(seed: u32, mc: &mut SetStateMcCosts) -> (u64, String) {
        let value = set_state_value(seed % SET_STATE_VALUES);
        let variant = seed / SET_STATE_VALUES % SET_STATE_VARIANTS;
        let ok = variant & 1 != 0;
        let flag = variant >= 2;
        let loaded = set_state_loaded(variant);
        let fd = sx(set_state_fd(variant));
        let at = |off: u32| sx(SET_STATE_SELF + off);
        let port = SET_STATE_PORT as u64;
        const FC: u64 = SET_STATE_FAKE_CYCLES;
        const QC: u64 = SET_STATE_QUERY_CYCLES;
        // The shared exits: 0x2c50c4 (epilogue), 0x2c50c0, 0x2c5064,
        // 0x2c4e44, 0x2c5054, 0x2c4e38, the close/open result, 0x2c50ac.
        let epilogue = 7;
        let done = 1 + epilogue;
        let closed = 3 + done;
        let fail = 4 + FC + epilogue;
        let invalid = 4 + FC + epilogue;
        let check = if ok { 2 + epilogue } else { 3 + fail };
        let handle_done = if ok { closed } else { 4 + fail };
        let refresh = 3 + if flag { epilogue } else { 2 + FC + done };
        let label = format!("state {value:#x} variant {variant}");
        if (value as u32).wrapping_sub(1) >= 0x4e {
            return (12 + refresh, label);
        }
        let mut m = |addr: u32, args: &[(usize, u64)]| mc.of(addr, ok, args);
        let close = [(A0, fd)];
        let open = |name: u32, mode: u64| [(A0, port), (A1, 0), (A2, at(name)), (A3, mode)];
        let delete = |name: u32| [(A0, port), (A1, 0), (A2, at(name))];
        let get_dir = |name: u32| {
            [
                (A0, port),
                (A1, 0),
                (A2, at(name)),
                (A3, 0),
                (T0, 1),
                (T1, at(MCU_DIR_ENTRY)),
            ]
        };
        let write = |buf: u32, size: u32| [(A0, fd), (A1, sx(buf)), (A2, sx(size))];
        let state = value as u32 as u64;
        let icon = |set: [u64; 3]| if state == set[1] { 1 } else { 3 };
        let arm = match state {
            MC_STATE_FAILED => {
                9 + (if loaded { 4 + FC } else { 0 })
                    + 2
                    + (if flag {
                        4 + m(SCE_MC_CLOSE, &close) + (if ok { 3 + FC } else { 0 }) + 3
                    } else {
                        0
                    })
                    + 4
                    + done
            }
            MC_STATE_GET_INFO_NOW => 4 + FC + epilogue,
            MC_STATE_FIND_SAVE => {
                3 + match SET_STATE_SCANS[variant as usize] {
                    (n, _) if n as i32 <= 0 => done,
                    (_, Some(k)) => k as u64 * (14 + QC) + 8 + QC + 15 + 2 * FC + epilogue,
                    (n, None) => n as u64 * (14 + QC) + 2 + epilogue,
                }
            }
            MC_STATE_OPEN_ICON_SYS => {
                7 + m(callee::SCE_MC_OPEN, &open(MCU_PATH, 1))
                    + if ok { closed } else { 7 + FC + invalid }
            }
            MC_STATE_CLOSE_ICON_SYS => {
                4 + m(SCE_MC_CLOSE, &close) + if ok { closed } else { 2 + invalid }
            }
            MC_STATE_READ_ICON_SYS => {
                7 + m(callee::SCE_MC_READ, &write(MC_ICON_SYS, MC_ICON_SYS_SIZE))
                    + if ok { done } else { 8 + 2 * FC + epilogue }
            }
            MC_STATE_GET_SAVE_DIR => {
                26 + 2 * FC
                    + m(callee::SCE_MC_GET_DIR, &get_dir(MCU_FILE))
                    + if ok { done } else { 2 + invalid }
            }
            s if MC_STATE_GET_ICON_DIR.contains(&s) => {
                4 + icon(MC_STATE_GET_ICON_DIR)
                    + 29
                    + 2 * FC
                    + m(callee::SCE_MC_GET_DIR, &get_dir(MCU_PATH))
                    + if ok { done } else { 2 + invalid }
            }
            MC_STATE_CLOSE_BAD_SAVE => {
                4 + m(SCE_MC_CLOSE, &close) + if ok { closed } else { 3 + invalid }
            }
            MC_STATE_OPEN_FOR_WRITE => {
                7 + m(callee::SCE_MC_OPEN, &open(MCU_FILE, 0x203)) + handle_done
            }
            MC_STATE_WRITE_SAVE_FILE => {
                8 + m(
                    callee::SCE_MC_WRITE,
                    &write(SET_STATE_BUFFER, SET_STATE_BUFFER_SIZE),
                ) + check
            }
            MC_STATE_CLOSE_AFTER_WRITE | MC_STATE_CLOSE_AFTER_READ => {
                3 + if flag {
                    4 + m(SCE_MC_CLOSE, &close) + handle_done
                } else {
                    4 + FC + epilogue
                }
            }
            MC_STATE_OPEN_FOR_READ => {
                10 + m(callee::SCE_MC_OPEN, &open(MCU_FILE, 1)) + if ok { done } else { 2 + fail }
            }
            MC_STATE_READ_SAVE_FILE => {
                6 + m(
                    callee::SCE_MC_READ,
                    &write(SET_STATE_BUFFER, SET_STATE_BUFFER_SIZE),
                ) + if ok { done } else { 2 + fail }
            }
            MC_STATE_MAKE_DIR => {
                7 + m(
                    callee::SCE_MC_MKDIR,
                    &[(A0, port), (A1, 0), (A2, at(MCU_FILE))],
                ) + check
            }
            MC_STATE_CREATE_ICON_SYS => {
                17 + FC + m(callee::SCE_MC_OPEN, &open(MCU_PATH, 0x203)) + check
            }
            MC_STATE_WRITE_ICON_SYS => {
                22 + 2 * FC
                    + (if flag { 2 } else { 3 + FC })
                    + 41
                    + 7 * FC
                    + m(callee::SCE_MC_WRITE, &write(MC_ICON_SYS, MC_ICON_SYS_SIZE))
                    + check
            }
            MC_STATE_CLOSE_NEW_ICON_SYS => 4 + m(SCE_MC_CLOSE, &close) + handle_done,
            s if MC_STATE_LOAD_ICON.contains(&s) => {
                3 + (if loaded { 4 + FC } else { 0 })
                    + 3
                    + icon(MC_STATE_LOAD_ICON)
                    + 15
                    + 2 * FC
                    + done
            }
            s if MC_STATE_CREATE_ICON.contains(&s) => {
                4 + icon(MC_STATE_CREATE_ICON)
                    + 18
                    + FC
                    + m(callee::SCE_MC_OPEN, &open(MCU_PATH, 0x203))
                    + handle_done
            }
            s if MC_STATE_WRITE_ICON.contains(&s) => {
                6 + m(
                    callee::SCE_MC_WRITE,
                    &write(set_state_async(loaded), SET_STATE_ASYNC_SIZE),
                ) + check
            }
            s if MC_STATE_CLOSE_ICON.contains(&s) => {
                3 + (if loaded { 3 + FC } else { 0 }) + 4 + m(SCE_MC_CLOSE, &close) + handle_done
            }
            MC_STATE_DELETE_FILE => 9 + 3 + m(callee::SCE_MC_DELETE, &delete(MCU_FILE)) + check,
            s if MC_STATE_DELETE_ICON.contains(&s) => {
                4 + icon(MC_STATE_DELETE_ICON)
                    + 15
                    + FC
                    + 3
                    + m(callee::SCE_MC_DELETE, &delete(MCU_PATH))
                    + check
            }
            MC_STATE_DELETE_ICON_SYS => {
                12 + FC + 3 + m(callee::SCE_MC_DELETE, &delete(MCU_PATH)) + check
            }
            MC_STATE_DELETE_DIR => 3 + 3 + m(callee::SCE_MC_DELETE, &delete(MCU_DIR)) + check,
            s if MC_SET_STATE_RECORD_ONLY.contains(&s) => done,
            _ => refresh,
        };
        // The prologue (12) and the table dispatch (7).
        (19 + arm, label)
    }

    /// Every state 0..=0x4f (and three more a1 values), each with libmc
    /// unbound / answered and the `flag` fields off / on (file descriptor
    /// -1, 3 or 0): all handler arms and sub-arms, the shared exits, the
    /// recursive calls (through the faked slot), state 7's scans (none, -1,
    /// first, second of 2, third of 3, all five empty).
    #[test]
    fn cps2_mem_card_set_state_matches() {
        let seeds = SET_STATE_VALUES * SET_STATE_VARIANTS;
        let mut costs = SetStateMcCosts(HashMap::new());
        let setup = |c: &mut Cpu, seed: u32| {
            set_state_fixture(
                c,
                seed,
                set_state_value(seed % SET_STATE_VALUES),
                seed / SET_STATE_VALUES % SET_STATE_VARIANTS,
            );
        };
        check_lohi(
            ee::s08::f_002c48c0__generated,
            cps2_mem_card_set_state,
            seeds,
            &setup,
        );
        // Many arms share a total, so each seed's own total is checked.
        for seed in 0..seeds {
            let (total, label) = set_state_expected(seed, &mut costs);
            let mut c = fresh();
            setup(&mut c, seed);
            let start = c.cycles;
            run_generated(&mut c, ee::s08::f_002c48c0__generated);
            assert_eq!(c.cycles - start, total, "seed {seed}: {label}");
        }
    }

    /// The return address of every call `cPS2MemCard_SetState` makes (each
    /// one can unwind: a slot fake, the vtable query, or a real `sceMc*`
    /// call reaching the faked `PollSema`).
    pub(crate) const SET_STATE_CALL_SITES: [u32; 57] = [
        0x002c_4938,
        0x002c_4950,
        0x002c_4964,
        0x002c_4988,
        0x002c_49a4,
        0x002c_49cc,
        0x002c_49e0,
        0x002c_4a0c,
        0x002c_4a3c,
        0x002c_4a58,
        0x002c_4a80,
        0x002c_4a94,
        0x002c_4aa8,
        0x002c_4abc,
        0x002c_4ad0,
        0x002c_4adc,
        0x002c_4ae8,
        0x002c_4af8,
        0x002c_4b08,
        0x002c_4b34,
        0x002c_4b80,
        0x002c_4b8c,
        0x002c_4be0,
        0x002c_4bf4,
        0x002c_4c1c,
        0x002c_4c38,
        0x002c_4c44,
        0x002c_4c70,
        0x002c_4c98,
        0x002c_4cbc,
        0x002c_4cdc,
        0x002c_4cfc,
        0x002c_4d24,
        0x002c_4d48,
        0x002c_4d68,
        0x002c_4de0,
        0x002c_4e10,
        0x002c_4e34,
        0x002c_4e4c,
        0x002c_4e78,
        0x002c_4eb4,
        0x002c_4ed0,
        0x002c_4eec,
        0x002c_4f00,
        0x002c_4f0c,
        0x002c_4f1c,
        0x002c_4f60,
        0x002c_4f70,
        0x002c_4f8c,
        0x002c_4ffc,
        0x002c_500c,
        0x002c_5028,
        0x002c_5040,
        0x002c_505c,
        0x002c_5098,
        0x002c_50a4,
        0x002c_50c0,
    ];

    /// `PollSema`'s return address in each `sceMc*` function, with that
    /// function's frame size and saved-`ra` offset.
    pub(crate) const SET_STATE_MC_FRAMES: [(u32, u32, u32); 6] = [
        (0x0040_9eac, 0x50, 0x40), // sceMcClose
        (0x0040_9d68, 0x90, 0x80), // sceMcOpen
        (0x0040_a1f8, 0x80, 0x70), // sceMcWrite
        (0x0040_a6ec, 0xa0, 0x90), // sceMcGetDir
        (0x0040_a0e4, 0x90, 0x80), // sceMcRead
        (0x0040_aab8, 0x80, 0x70), // sceMcDelete
    ];

    /// `sceMcMkdir`'s call of `sceMcOpen` returns here (its 0x10 frame holds
    /// `ra` at 0).
    pub(crate) const SET_STATE_MKDIR_OPEN_RET: u32 = 0x0040_9e48;

    /// The `SetState` call site of fake call `n`, from its log entry: the
    /// logged `ra`, or for `PollSema` inside a `sceMc*` call, the `ra` that
    /// call (and `sceMcMkdir` around `sceMcOpen`) saved on the stack.
    pub(crate) fn set_state_unwound_site(c: &mut Cpu, n: u32) -> u32 {
        let entry = fakes::LOG_START + 0x220 * n;
        let target = c.bus.read32(entry);
        let mut site = c.bus.read32(entry + 0x10 + 16 * RA as u32);
        if target == 0x0042_3df0 {
            let mut sp = c.bus.read32(entry + 0x10 + 16 * SP as u32);
            let &(_, frame, ra_at) = SET_STATE_MC_FRAMES
                .iter()
                .find(|f| f.0 == site)
                .unwrap_or_else(|| panic!("PollSema from {site:#x}"));
            site = c.bus.read32(sp + ra_at);
            if site == SET_STATE_MKDIR_OPEN_RET {
                sp += frame;
                site = c.bus.read32(sp);
            }
        }
        site
    }

    /// The unwind cases: for every state value `k` and variant (see
    /// [`set_state_fixture`]), one case per fake call `j` the path makes
    /// (counted on an undisturbed run of the generated body).
    pub(crate) fn set_state_unwind_cases() -> Vec<(u32, u32, u32)> {
        let mut cases = Vec::new();
        for k in 0..SET_STATE_VALUES {
            for variant in 0..4 {
                let mut c = fresh();
                set_state_fixture(&mut c, k, set_state_value(k), variant);
                run_generated(&mut c, ee::s08::f_002c48c0__generated);
                for j in 0..c.bus.read32(fakes::LOG_COUNT) {
                    cases.push((k, variant, j));
                }
            }
        }
        cases
    }

    /// Every fake call of every path unwinds in turn (case = state value,
    /// variant, call index), and every one of the 57 call sites is among the
    /// unwound ones.
    #[test]
    fn cps2_mem_card_set_state_unwinds() {
        let cases = set_state_unwind_cases();
        let setup = |c: &mut Cpu, seed: u32| {
            let (k, variant, j) = cases[seed as usize];
            set_state_fixture(c, k, set_state_value(k), variant);
            c.bus.write32(LOG_UNWIND_AT, j);
        };
        let seeds = cases.len() as u32;
        check_lohi(
            ee::s08::f_002c48c0__generated,
            cps2_mem_card_set_state,
            seeds,
            &setup,
        );
        let mut sites = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, ee::s08::f_002c48c0__generated);
            let case = cases[seed as usize];
            assert_eq!(
                c.pc,
                0xdead_0000 | case.2 << 4,
                "case {case:x?} did not unwind at its call"
            );
            sites.insert(set_state_unwound_site(&mut c, case.2));
        }
        for site in SET_STATE_CALL_SITES {
            assert!(
                sites.contains(&site),
                "call site {site:#x} never unwound (unwound: {sites:x?})"
            );
        }
    }

    driven!(set_state_gen_driven, ee::s08::f_002c48c0__generated);

    driven!(set_state_hand_driven, cps2_mem_card_set_state);

    /// State 7's slot scan with live events: 6-9 slots (`seed / 6 % 4`),
    /// none or the last holding a save (`seed / 24 % 2`). Entered at the
    /// start (`seed / 3 % 2 == 0`) or at the head 0x2c4e60 itself with the
    /// frame open and the scan at slot `seed / 48 % 3`. The event mode is
    /// `seed % 3` ([`schedule`]).
    pub(crate) fn set_state_scan_setup(c: &mut Cpu, seed: u32) -> Events {
        let config = seed / 6;
        let mut r = set_state_fixture(c, config, MC_STATE_FIND_SAVE, 0);
        let count = 6 + config % 4;
        let found = if config / 4 % 2 == 1 {
            Some(count - 1)
        } else {
            None
        };
        set_state_scan(c, count, found);
        let entry = if seed / 3 % 2 == 0 {
            MC_SET_STATE_ENTRY
        } else {
            let sp = STACK_TOP - SET_STATE_FRAME.size;
            c.g[SP][0] = sp as u64;
            fill(c, &mut r, sp + 0x50, 0x14);
            c.bus.write64(sp + 0x40, SENTINEL_RA as u64);
            c.g[S2][0] = SET_STATE_SELF as u64;
            c.bus.write32(SET_STATE_SELF + MCU_INDEX, config / 8 % 3);
            MC_SET_STATE_SCAN_HEAD
        };
        schedule(c, &mut r, seed, entry)
    }

    /// The scan with live events, driven: entered at the start, the event
    /// lands on the scan head and is delivered, never unwinding there;
    /// entered at the head, an unwinding event at the back edge returns and
    /// the drive resumes at the head.
    #[test]
    fn cps2_mem_card_set_state_events() {
        check_events(
            set_state_gen_driven,
            set_state_hand_driven,
            144,
            &|c, seed| set_state_scan_setup(c, seed),
        );
    }

    /// The scan entered at its head with an unwinding event (seed `i` is
    /// `set_state_scan_setup`'s seed `6i + 4`: resumed, `Events::Unwind`),
    /// not driven: the return at the back edge itself is compared (`pc` and
    /// the registers as left there), and some seeds must stop there (`pc` the
    /// head, the low bit masked off).
    #[test]
    fn cps2_mem_card_set_state_scan_unwinds() {
        let seeds = 48;
        let setup = |c: &mut Cpu, i: u32| {
            let mode = set_state_scan_setup(c, 6 * i + 4);
            assert_eq!(mode, Events::Unwind);
        };
        check_lohi(
            ee::s08::f_002c48c0__generated,
            cps2_mem_card_set_state,
            seeds,
            &setup,
        );
        let mut stopped = 0;
        for i in 0..seeds {
            let mut c = fresh();
            setup(&mut c, i);
            run_generated(&mut c, ee::s08::f_002c48c0__generated);
            if c.pc & !1 == MC_SET_STATE_SCAN_HEAD {
                stopped += 1;
            }
        }
        assert!(
            stopped > 0,
            "no seed stopped at the scan head (0 of {seeds})"
        );
    }

    // -----------------------------------------------------------------------
    // Card-query fakes shared by the tests below.

    /// Gate word of the card-query fakes; fake k's control word is at
    /// `CARD_QUERY_CTRL + 0x10 * k`.
    pub(crate) const CARD_QUERY_MAGIC: u32 = 0x4d43_4d44;

    pub(crate) const CARD_QUERY_CTRL: u32 = DATA + 0xf_f600;

    pub(crate) const CARD_QUERY_FAKES: u32 = 9;

    /// The v0 of faked call n is `sx(word n)` of this table.
    pub(crate) const CARD_QUERY_ANSWERS: u32 = DATA + 0xf_e000;

    pub(crate) const CARD_QUERY_MAX_CALLS: u32 = 64;

    /// Cycles every faked call charges: fixed, so each path has one total.
    pub(crate) const CARD_QUERY_CALL_COST: u64 = 6;

    /// Fixture layout: the card object (non-zero low bytes), its vtable, the
    /// destination string, the game object, its string table and the table's
    /// vtable.
    pub(crate) const CARD_QUERY_THIS: u32 = DATA + 0x1238;

    pub(crate) const CARD_QUERY_VTABLE: u32 = DATA + 0x3010;

    pub(crate) const CARD_QUERY_DST: u32 = DATA + 0x3a46;

    pub(crate) const CARD_QUERY_GAME: u32 = DATA + 0x4124;

    pub(crate) const CARD_QUERY_STRINGS: u32 = DATA + 0x4a38;

    pub(crate) const CARD_QUERY_STRINGS_VT: u32 = DATA + 0x5010;

    /// A faked call (the `miss` hook for the fixtures' vtable targets, and the
    /// body of the gated slot fakes): logged, clobbering and unwinding as
    /// `fakes::fake` does, but charging `CARD_QUERY_CALL_COST` and returning
    /// the fixture's answer for its call index.
    pub(crate) fn card_query_call(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(LOG_COUNT).min(CARD_QUERY_MAX_CALLS - 1);
        let ra = c.g[RA][0] as u32;
        let start = c.cycles;
        fakes::fake(c, target, ra);
        c.cycles = start + CARD_QUERY_CALL_COST;
        let answer = c.bus.read32(CARD_QUERY_ANSWERS + 4 * n);
        c.g[V0][0] = sx(answer);
    }

    /// A gated fake on an override slot: with the gate word at `$ctrl`, a
    /// `card_query_call` to `$addr`; otherwise the slot's previous body.
    macro_rules! card_query_slot_fake {
        ($fake:ident, $prev:ident, $ctrl:expr, $addr:expr, $generated:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);

            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32($ctrl) != CARD_QUERY_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                card_query_call(c, $addr);
            }
        };
    }

    card_query_slot_fake!(
        card_query_fake_str_hash,
        CARD_QUERY_PREV_STR_HASH,
        CARD_QUERY_CTRL,
        0x0031_7670,
        ee::s10::f_00317670__generated
    );

    card_query_slot_fake!(
        card_query_fake_ustr_copy,
        CARD_QUERY_PREV_USTR_COPY,
        CARD_QUERY_CTRL + 0x10,
        0x002c_2508,
        ee::s08::f_002c2508__generated
    );

    card_query_slot_fake!(
        card_query_fake_ustr_from_ascii,
        CARD_QUERY_PREV_USTR_FROM_ASCII,
        CARD_QUERY_CTRL + 0x20,
        0x002c_2540,
        ee::s08::f_002c2540__generated
    );

    card_query_slot_fake!(
        card_query_fake_mem_set,
        CARD_QUERY_PREV_MEM_SET,
        CARD_QUERY_CTRL + 0x30,
        0x003e_6448,
        ee::s14::f_003e6448__generated
    );

    card_query_slot_fake!(
        card_query_fake_free_space_for_port,
        CARD_QUERY_PREV_FREE_SPACE_FOR_PORT,
        CARD_QUERY_CTRL + 0x40,
        0x002c_5320,
        ee::s08::f_002c5320__generated
    );

    card_query_slot_fake!(
        card_query_fake_ustr_sprintf,
        CARD_QUERY_PREV_USTR_SPRINTF,
        CARD_QUERY_CTRL + 0x50,
        0x002c_26d0,
        ee::s08::f_002c26d0__generated
    );

    card_query_slot_fake!(
        card_query_fake_strcpy,
        CARD_QUERY_PREV_STRCPY,
        CARD_QUERY_CTRL + 0x60,
        0x0041_66f4,
        ee::s15::f_004166f4__generated
    );

    card_query_slot_fake!(
        card_query_fake_sprintf,
        CARD_QUERY_PREV_SPRINTF,
        CARD_QUERY_CTRL + 0x70,
        0x0041_7828,
        ee::s15::f_00417828__generated
    );

    card_query_slot_fake!(
        card_query_fake_set_state,
        CARD_QUERY_PREV_SET_STATE,
        CARD_QUERY_CTRL + 0x80,
        0x002c_48c0,
        ee::s08::f_002c48c0__generated
    );

    /// Hook the nine gated fakes (once per process).
    pub(crate) fn card_query_fakes_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            card_query_fake_str_hash,
            &CARD_QUERY_PREV_STR_HASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            card_query_fake_ustr_copy,
            &CARD_QUERY_PREV_USTR_COPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2540),
            card_query_fake_ustr_from_ascii,
            &CARD_QUERY_PREV_USTR_FROM_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E6448),
            card_query_fake_mem_set,
            &CARD_QUERY_PREV_MEM_SET,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C5320),
            card_query_fake_free_space_for_port,
            &CARD_QUERY_PREV_FREE_SPACE_FOR_PORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            card_query_fake_ustr_sprintf,
            &CARD_QUERY_PREV_USTR_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            card_query_fake_strcpy,
            &CARD_QUERY_PREV_STRCPY,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            card_query_fake_sprintf,
            &CARD_QUERY_PREV_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C48C0),
            card_query_fake_set_state,
            &CARD_QUERY_PREV_SET_STATE,
        );
    }

    /// The machine of these tests: scrambled registers (`machine`), the
    /// fakes hooked and gated on, and vtable calls answered by
    /// `card_query_call`.
    pub(crate) fn card_query_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        card_query_fakes_install();
        let r = machine(c, seed, salt, 0);
        c.hooks.miss = card_query_call;
        for k in 0..CARD_QUERY_FAKES {
            c.bus.write32(CARD_QUERY_CTRL + 0x10 * k, CARD_QUERY_MAGIC);
        }
        r
    }

    /// The faked calls' answers in call order, and the call that unwinds.
    pub(crate) fn card_query_calls(c: &mut Cpu, answers: &[u32], unwind_at: Option<u32>) {
        put_words(c, CARD_QUERY_ANSWERS, answers);
        c.bus.write32(LOG_UNWIND_AT, unwind_at.unwrap_or(u32::MAX));
    }

    /// The card object filled with junk (`words` words), its vtable's
    /// `slots` pointing at fake methods.
    pub(crate) fn card_query_card(c: &mut Cpu, r: &mut Rng, words: u32, slots: &[u32]) {
        fill(c, r, CARD_QUERY_THIS, words);
        let vt = put_vtable(c, r, CARD_QUERY_VTABLE, slots);
        c.bus.write32(CARD_QUERY_THIS, vt);
    }

    /// The game object at `gp - 0x848` and its string table, whose find
    /// method is a fake.
    pub(crate) fn card_query_strings(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - MC_GAME_GP_OFF, CARD_QUERY_GAME);
        c.bus
            .write32(CARD_QUERY_GAME + MC_GAME_STRINGS, CARD_QUERY_STRINGS);
        c.bus.write32(CARD_QUERY_STRINGS + 4, CARD_QUERY_STRINGS_VT);
        put_vtable(c, r, CARD_QUERY_STRINGS_VT, &[MC_STRINGS_FIND]);
    }

    /// Junk for the high half of a register the code copies whole (about
    /// half the time).
    pub(crate) fn card_query_high(r: &mut Rng) -> u64 {
        if r.next_u32() % 2 == 1 {
            ((r.next_u32() | 1) as u64) << 32
        } else {
            0
        }
    }

    /// A random pointer-like answer.
    pub(crate) fn card_query_ptr(r: &mut Rng) -> u32 {
        DATA + 0x8000 + (r.next_u32() & 0x7_fff0)
    }

    /// Some seeds return to the caller and some stop at a faked call's
    /// unwind address.
    pub(crate) fn card_query_require_unwinds(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let (mut returned, mut unwound) = (0, 0);
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            if c.pc == SENTINEL_RA {
                returned += 1;
            } else if c.pc & 0xffff_0000 == 0xdead_0000 {
                unwound += 1;
            }
        }
        assert!(
            returned > 0 && unwound > 0,
            "returned {returned}, unwound {unwound}"
        );
    }

    /// A flag word or a bool answer: non-zero when `set` (1, 0x100,
    /// 0x8000_0000 or junk; the game tests the whole word, so bit 0 and the
    /// low byte are clear on some seeds).
    pub(crate) fn card_query_flag(r: &mut Rng, set: bool) -> u32 {
        if !set {
            return 0;
        }
        match r.next_u32() % 4 {
            0 => 1,
            1 => 0x100,
            2 => 0x8000_0000,
            _ => r.next_u32() | 0x0200_0000,
        }
    }

    // -----------------------------------------------------------------------
    // cPS2MemCard_v17

    /// Arms of one slot of the scan: skipped for a flag word or no name, the
    /// first candidate, or compared with the best slot on year, month, day,
    /// time; and the scan skipped (card not formatted).
    pub(crate) const NS_USED: u32 = 0;

    pub(crate) const NS_INVALID: u32 = 1;

    pub(crate) const NS_NO_NAME: u32 = 2;

    pub(crate) const NS_NO_SPACE: u32 = 3;

    pub(crate) const NS_FIRST: u32 = 4;

    pub(crate) const NS_OLDER_YEAR: u32 = 5;

    pub(crate) const NS_NEWER_YEAR: u32 = 6;

    pub(crate) const NS_OLDER_MONTH: u32 = 7;

    pub(crate) const NS_NEWER_MONTH: u32 = 8;

    pub(crate) const NS_OLDER_DAY: u32 = 9;

    pub(crate) const NS_NEWER_DAY: u32 = 10;

    pub(crate) const NS_OLDER_TIME: u32 = 11;

    pub(crate) const NS_NEWER_TIME: u32 = 12;

    pub(crate) const NS_SAME_TIME: u32 = 13;

    pub(crate) const NS_NOT_FORMATTED: u32 = 14;

    /// Cycles of one slot per arm: the generated charges including the
    /// 5-cycle loop tail, without the name query's call.
    pub(crate) const NS_SLOT_CYCLES: [u64; 14] =
        [8, 12, 21, 23, 28, 49, 52, 54, 58, 60, 64, 66, 70, 70];

    /// A timestamp: seconds, minutes, hours, day, month, year.
    pub(crate) type NsDate = [u32; 6];

    pub(crate) fn ns_random_date(r: &mut Rng) -> NsDate {
        [
            r.next_u32() % 60,
            1 + r.next_u32() % 58,
            1 + r.next_u32() % 22,
            2 + r.next_u32() % 26,
            2 + r.next_u32() % 10,
            2001 + r.next_u32() % 28,
        ]
    }

    /// A date that compares with `best` the way the compare `arm` names: the
    /// fields above the deciding one equal, the deciding one one less or
    /// more, the rest random. Times decide on the minutes (the seconds of
    /// both sides are the best slot's); "same time" keeps random seconds.
    pub(crate) fn ns_date_against(arm: u32, best: NsDate, r: &mut Rng) -> NsDate {
        let mut d = ns_random_date(r);
        let (field, newer) = match arm {
            NS_OLDER_YEAR => (5, false),
            NS_NEWER_YEAR => (5, true),
            NS_OLDER_MONTH => (4, false),
            NS_NEWER_MONTH => (4, true),
            NS_OLDER_DAY => (3, false),
            NS_NEWER_DAY => (3, true),
            NS_OLDER_TIME => (1, false),
            NS_NEWER_TIME => (1, true),
            _ => (0, false),
        };
        d[field.max(1)..].copy_from_slice(&best[field.max(1)..]);
        if field > 0 {
            d[field] = if newer {
                best[field] + 1
            } else {
                best[field] - 1
            };
        }
        d
    }

    pub(crate) fn ns_put_date(c: &mut Cpu, entry: u32, d: NsDate) {
        for (off, v) in [
            (MCU_ENTRY_SEC, d[0]),
            (MCU_ENTRY_MIN, d[1]),
            (MCU_ENTRY_HOUR, d[2]),
            (MCU_ENTRY_DAY, d[3]),
            (MCU_ENTRY_MONTH, d[4]),
        ] {
            c.bus.write8(entry + off, v as u8);
        }
        c.bus.write16(entry + MCU_ENTRY_YEAR, d[5] as u16);
    }

    /// Seed digits: `seed % 15` the arm of slot 1 (slot 0 is the first
    /// candidate; for `NS_FIRST` slot 0 is used and slot 1 the first) or
    /// "not formatted"; `seed / 15 % 2` slots 2-5 used (invalid for
    /// `NS_FIRST`) or compared with rotating arms; `seed / 30 % 4` the call
    /// that unwinds (3: none); `seed / 120 % 2` junk in `a0`'s high half
    /// (`s1 = a0`, `s0 = s1` whole).
    pub(crate) fn newest_save_fixture(c: &mut Cpu, seed: u32) {
        let focus = seed % 15;
        let rest_compared = seed / 15 % 2 == 1;
        let unwind = seed / 30 % 4;
        let high = seed / 120 % 2 == 1;
        let mut r = card_query_machine(c, seed, 0x2c_64f8);
        card_query_card(c, &mut r, 0x480 / 4, &[MCU_VT_V55, MCU_VT_HAS_SLOT_NAME]);
        let mut arms = [NS_USED; 6];
        match focus {
            NS_FIRST => {
                arms[1] = NS_FIRST;
                arms[2..].fill(NS_INVALID);
            }
            NS_NOT_FORMATTED => {}
            arm => {
                arms[0] = NS_FIRST;
                arms[1] = arm;
            }
        }
        if rest_compared {
            for (i, arm) in arms.iter_mut().enumerate().skip(2) {
                *arm = NS_OLDER_YEAR + (focus + i as u32) % 9;
            }
        }
        let best = ns_random_date(&mut r);
        // v55, then one name query per slot that gets that far.
        let mut answers = vec![card_query_flag(&mut r, focus != NS_NOT_FORMATTED)];
        for (i, &arm) in arms.iter().enumerate() {
            let entry = CARD_QUERY_THIS + MCU_ENTRIES + MCU_ENTRY_SIZE * i as u32;
            for (word, set) in [
                (MCU_ENTRY_USED, arm == NS_USED),
                (MCU_ENTRY_INVALID, arm == NS_INVALID),
                (MCU_ENTRY_NO_SPACE, arm == NS_NO_SPACE),
            ] {
                let v = card_query_flag(&mut r, set);
                c.bus.write32(entry + word, v);
            }
            if arm >= NS_NO_NAME {
                answers.push(card_query_flag(&mut r, arm != NS_NO_NAME));
            }
            let date = match arm {
                NS_FIRST => best,
                a if a >= NS_OLDER_YEAR => ns_date_against(a, best, &mut r),
                _ => ns_random_date(&mut r),
            };
            ns_put_date(c, entry, date);
        }
        card_query_calls(c, &answers, (unwind < 3).then_some(unwind));
        let hi = if high {
            ((r.next_u32() | 1) as u64) << 32
        } else {
            0
        };
        c.g[A0][0] = hi | CARD_QUERY_THIS as u64;
    }

    /// The total of each focus arm with slots 2-5 used (invalid), no unwind.
    pub(crate) fn newest_save_totals() -> Vec<u64> {
        let k = CARD_QUERY_CALL_COST;
        let slot = |arm: u32| NS_SLOT_CYCLES[arm as usize];
        // 20 + the v55 call + 6 before the scan, 1 + 12 after it; `calls`
        // name queries.
        let scan = |slots: u64, calls: u64| 20 + 6 + 1 + 12 + slots + (1 + calls) * k;
        let mut totals = vec![20 + k + 2 + 12];
        for arm in NS_USED..=NS_SAME_TIME {
            totals.push(if arm == NS_FIRST {
                scan(slot(NS_USED) + slot(NS_FIRST) + 4 * slot(NS_INVALID), 1)
            } else {
                let queries = 1 + (arm >= NS_NO_NAME) as u64;
                scan(slot(NS_FIRST) + slot(arm) + 4 * slot(NS_USED), queries)
            });
        }
        totals
    }

    /// Every arm (`NS_NEWER_TIME` and `NS_SAME_TIME` share a total; slot 1
    /// reaches each arm as named, while the rotating slots 2-5 are dated
    /// against slot 0, not the running best), every unwinding call.
    #[test]
    fn c_ps2_mem_card_v17_matches() {
        check_paths_lohi(
            ee::s08::f_002c64f8__generated,
            c_ps2_mem_card_v17,
            240,
            &newest_save_totals(),
            &newest_save_fixture,
        );
        card_query_require_unwinds(ee::s08::f_002c64f8__generated, 240, &newest_save_fixture);
    }

    driven!(newest_save_generated, ee::s08::f_002c64f8__generated);

    driven!(newest_save_hand, c_ps2_mem_card_v17);

    /// Events at the scan's back edge (head 0x2c6568): the original delivers
    /// and carries on (no resume entry, nothing to resume). Every fixture
    /// scans and no call unwinds.
    #[test]
    fn c_ps2_mem_card_v17_events() {
        check_events(newest_save_generated, newest_save_hand, 120, &|c, seed| {
            let i = seed / 3;
            newest_save_fixture(c, i % 14 + 15 * (i / 14 % 2) + 30 * 3 + 120 * (i / 28 % 2));
            let mut r = Rng(0x64f8 ^ u64::from(seed));
            schedule(c, &mut r, seed, 0x002c_64f8)
        });
    }

    // -----------------------------------------------------------------------
    // cPS2MemCard_GetStatusString

    /// Arms: no card (`v53`), not formatted (`v55`), slot used, invalid, no
    /// space, no name, named.
    pub(crate) const STATUS_NO_CARD: u32 = 0;

    pub(crate) const STATUS_UNFORMATTED: u32 = 1;

    pub(crate) const STATUS_USED: u32 = 2;

    pub(crate) const STATUS_INVALID: u32 = 3;

    pub(crate) const STATUS_NO_SPACE: u32 = 4;

    pub(crate) const STATUS_NO_NAME: u32 = 5;

    pub(crate) const STATUS_NAMED: u32 = 6;

    /// Seed digits: `seed % 7` the arm, `seed / 7 % 8` the call that unwinds
    /// (past the last call: none), `seed / 56 % 3` the slot (0, 2 or 5).
    pub(crate) fn status_string_fixture(c: &mut Cpu, seed: u32) {
        let arm = seed % 7;
        let unwind = seed / 7 % 8;
        let slot = [0, 2, 5][(seed / 56 % 3) as usize];
        let mut r = card_query_machine(c, seed, 0x2c_6128);
        card_query_card(
            c,
            &mut r,
            0x480 / 4,
            &[MCU_VT_V53, MCU_VT_V55, MCU_VT_HAS_SLOT_NAME],
        );
        card_query_strings(c, &mut r);
        let entry = CARD_QUERY_THIS + MCU_ENTRIES + MCU_ENTRY_SIZE * slot;
        for (word, set) in [
            (MCU_ENTRY_USED, arm == STATUS_USED),
            (MCU_ENTRY_INVALID, arm == STATUS_INVALID),
            (MCU_ENTRY_NO_SPACE, arm == STATUS_NO_SPACE),
        ] {
            let v = card_query_flag(&mut r, set);
            c.bus.write32(entry + word, v);
        }
        // The card queries in call order, then the hash, the found string
        // and the copies.
        let mut answers = vec![card_query_flag(&mut r, arm != STATUS_NO_CARD)];
        if arm != STATUS_NO_CARD {
            answers.push(card_query_flag(&mut r, arm != STATUS_UNFORMATTED));
        }
        if arm >= STATUS_NO_NAME {
            answers.push(card_query_flag(&mut r, arm == STATUS_NAMED));
        }
        for _ in 0..4 {
            answers.push(card_query_ptr(&mut r));
        }
        card_query_calls(c, &answers, Some(unwind));
        c.g[A0][0] = card_query_high(&mut r) | CARD_QUERY_THIS as u64;
        c.g[A1][0] = card_query_high(&mut r) | slot as u64;
        c.g[A2][0] = card_query_high(&mut r) | CARD_QUERY_DST as u64;
    }

    /// Totals per arm: generated charges plus the faked calls (5, 6, 6, 6,
    /// 6, 7, 5 calls).
    #[test]
    fn c_ps2_mem_card_get_status_string_matches() {
        let k = CARD_QUERY_CALL_COST;
        check_paths_lohi(
            ee::s08::f_002c6128__generated,
            c_ps2_mem_card_get_status_string,
            168,
            &[
                42 + 5 * k,
                51 + 6 * k,
                55 + 6 * k,
                59 + 6 * k,
                63 + 6 * k,
                71 + 7 * k,
                58 + 5 * k,
            ],
            &status_string_fixture,
        );
        card_query_require_unwinds(ee::s08::f_002c6128__generated, 168, &status_string_fixture);
    }

    // -----------------------------------------------------------------------
    // cPS2MemCard_Init

    /// Seed digit: `seed % 9` the clearing call that unwinds (8: none).
    pub(crate) fn init_fixture(c: &mut Cpu, seed: u32) {
        let unwind = seed % 9;
        let mut r = card_query_machine(c, seed, 0x2c_4050);
        fill(c, &mut r, CARD_QUERY_THIS, 0x1500 / 4);
        fill(c, &mut r, GAME_GP + MCU_GP_TYPE, 4);
        let answers: Vec<u32> = (0..8).map(|_| card_query_ptr(&mut r)).collect();
        card_query_calls(c, &answers, (unwind < 8).then_some(unwind));
        c.g[A0][0] = card_query_high(&mut r) | CARD_QUERY_THIS as u64;
    }

    /// One path: 54 + two loop passes of 8 + 20, and eight faked clears.
    #[test]
    fn c_ps2_mem_card_init_matches() {
        check_paths(
            ee::s08::f_002c4050__generated,
            c_ps2_mem_card_init,
            108,
            &[54 + 2 * 8 + 20 + 8 * CARD_QUERY_CALL_COST],
            &init_fixture,
        );
        card_query_require_unwinds(ee::s08::f_002c4050__generated, 108, &init_fixture);
    }

    driven!(init_generated, ee::s08::f_002c4050__generated);

    driven!(init_hand, c_ps2_mem_card_init);

    /// Events at the port-record loop's back edge (head 0x2c4128, reached once
    /// after 86 cycles, so every event seed's event is due there): delivered,
    /// the loop carries on (no resume entry). No call unwinds.
    #[test]
    fn c_ps2_mem_card_init_events() {
        check_events(init_generated, init_hand, 60, &|c, seed| {
            init_fixture(c, 8 + 9 * (seed / 3));
            let mut r = Rng(0x4050 ^ u64::from(seed));
            schedule(c, &mut r, seed, 0x002c_4050)
        });
    }

    // -----------------------------------------------------------------------
    // cPS2MemCard_GetFreeSpaceString

    /// Arms: not formatted, status -2, status 0 with a negative / zero /
    /// positive free space, another status.
    pub(crate) const FREE_UNFORMATTED: u32 = 0;

    pub(crate) const FREE_NO_FORMAT: u32 = 1;

    pub(crate) const FREE_NEGATIVE: u32 = 2;

    pub(crate) const FREE_ZERO: u32 = 3;

    pub(crate) const FREE_POSITIVE: u32 = 4;

    pub(crate) const FREE_OTHER_STATUS: u32 = 5;

    /// Seed digits: `seed % 6` the arm, `seed / 6 % 5` the call that unwinds
    /// (4: none), `seed / 30 % 2` the port, `seed / 60 % 2` a negative free
    /// space of -1 (0) or -100 and below (1).
    pub(crate) fn free_space_fixture(c: &mut Cpu, seed: u32) {
        let arm = seed % 6;
        let unwind = seed / 6 % 5;
        let port = seed / 30 % 2;
        let minus_one = seed / 60 % 2 == 0;
        let mut r = card_query_machine(c, seed, 0x2c_53e0);
        fill(c, &mut r, CARD_QUERY_THIS, 0x480 / 4);
        card_query_strings(c, &mut r);
        let record = CARD_QUERY_THIS + MCU_PORTS + MCU_PORT_INFO_SIZE * port;
        let formatted = card_query_flag(&mut r, arm != FREE_UNFORMATTED);
        c.bus.write32(record + MCU_PORT_INFO_FORMATTED, formatted);
        let status = match arm {
            FREE_UNFORMATTED => r.next_u32(),
            FREE_NO_FORMAT => MC_STATUS_NO_FORMAT,
            FREE_OTHER_STATUS => [0xffff_ffff, 0xffff_d8ee, 5][(r.next_u32() % 3) as usize],
            _ => 0,
        };
        c.bus.write32(record + MCU_PORT_INFO_STATUS, status);
        // The free space (or the blank text call's result), then the hash,
        // the found format and the formatter's result.
        let first = match arm {
            FREE_NEGATIVE if minus_one => u32::MAX,
            FREE_NEGATIVE => 0xffff_ff9c - r.next_u32() % 0x1000,
            FREE_ZERO => 0,
            FREE_POSITIVE => 1 + r.next_u32() % 0x2000,
            _ => card_query_ptr(&mut r),
        };
        let mut answers = vec![first];
        for _ in 0..3 {
            answers.push(card_query_ptr(&mut r));
        }
        card_query_calls(c, &answers, (unwind < 4).then_some(unwind));
        c.g[A0][0] = card_query_high(&mut r) | CARD_QUERY_THIS as u64;
        c.g[A1][0] = card_query_high(&mut r) | port as u64;
        c.g[A2][0] = card_query_high(&mut r) | CARD_QUERY_DST as u64;
    }

    /// Totals: the two blank call sites (24 / 28 and 31 cycles, one call)
    /// and the free-space text (48, four calls; the free-space clamp is a
    /// conditional move, all three values share it).
    #[test]
    fn c_ps2_mem_card_get_free_space_string_matches() {
        let k = CARD_QUERY_CALL_COST;
        check_paths_lohi(
            ee::s08::f_002c53e0__generated,
            c_ps2_mem_card_get_free_space_string,
            240,
            &[24 + k, 28 + k, 31 + k, 48 + 4 * k],
            &free_space_fixture,
        );
        card_query_require_unwinds(ee::s08::f_002c53e0__generated, 240, &free_space_fixture);
    }

    // -----------------------------------------------------------------------
    // cPS2MemCard_v54

    /// Arms: not a PS2 card, `v50` true, not formatted, `v52` false / true.
    pub(crate) const USABLE_NOT_PS2: u32 = 0;

    pub(crate) const USABLE_V50: u32 = 1;

    pub(crate) const USABLE_UNFORMATTED: u32 = 2;

    pub(crate) const USABLE_V52_FALSE: u32 = 3;

    /// Seed digits: `seed % 5` the arm, `seed / 5 % 5` the call that unwinds
    /// (4: none), `seed / 25 % 3` what a true `v52` returns (1, a large odd
    /// word, a word that is negative once sign-extended).
    pub(crate) fn usable_port_fixture(c: &mut Cpu, seed: u32) {
        let arm = seed % 5;
        let unwind = seed / 5 % 5;
        let v52_true = [1, 0x0012_3457, 0x8000_0001][(seed / 25 % 3) as usize];
        let mut r = card_query_machine(c, seed, 0x2d_3840);
        card_query_card(
            c,
            &mut r,
            0x480 / 4,
            &[
                MCU_VT_IS_PORT_NOT_PS2_CARD,
                MCU_VT_V50,
                MCU_VT_IS_PORT_FORMATTED,
                MCU_VT_V52,
            ],
        );
        let yes = r.next_u32() | 1;
        let answers = match arm {
            USABLE_NOT_PS2 => vec![yes],
            USABLE_V50 => vec![0, yes],
            USABLE_UNFORMATTED => vec![0, 0, 0],
            USABLE_V52_FALSE => vec![0, 0, yes, 0],
            _ => vec![0, 0, yes, v52_true],
        };
        card_query_calls(c, &answers, (unwind < 4).then_some(unwind));
        c.g[A0][0] = card_query_high(&mut r) | CARD_QUERY_THIS as u64;
        c.g[A1][0] = card_query_high(&mut r) | (r.next_u32() % 2) as u64;
    }

    /// Totals: 21, 29, 38 and 44 cycles with 1, 2, 3 and 4 calls (`v52`
    /// false and true share the last).
    #[test]
    fn c_ps2_mem_card_v54_matches() {
        let k = CARD_QUERY_CALL_COST;
        check_paths(
            ee::s09::f_002d3840__generated,
            c_ps2_mem_card_v54,
            200,
            &[21 + k, 29 + 2 * k, 38 + 3 * k, 44 + 4 * k],
            &usable_port_fixture,
        );
        card_query_require_unwinds(ee::s09::f_002d3840__generated, 200, &usable_port_fixture);
    }

    // -----------------------------------------------------------------------
    // cPS2MemCard_DoDeleteSave

    /// Seed digits: `seed % 4` the call that unwinds (`strcpy`, `sprintf`,
    /// `SetState`; 3: none), `seed / 4 % 6` the slot.
    pub(crate) fn delete_save_fixture(c: &mut Cpu, seed: u32) {
        let unwind = seed % 4;
        let slot = seed / 4 % 6;
        let mut r = card_query_machine(c, seed, 0x2c_4410);
        fill(c, &mut r, CARD_QUERY_THIS, 0x480 / 4);
        c.bus.write32(CARD_QUERY_THIS + MCU_SLOT, slot);
        let answers: Vec<u32> = (0..3).map(|_| card_query_ptr(&mut r)).collect();
        card_query_calls(c, &answers, (unwind < 3).then_some(unwind));
        c.g[A0][0] = card_query_high(&mut r) | CARD_QUERY_THIS as u64;
    }

    /// One path: 28 cycles and three faked calls.
    #[test]
    fn c_ps2_mem_card_do_delete_save_matches() {
        check_paths_lohi(
            ee::s08::f_002c4410__generated,
            c_ps2_mem_card_do_delete_save,
            120,
            &[28 + 3 * CARD_QUERY_CALL_COST],
            &delete_save_fixture,
        );
        card_query_require_unwinds(ee::s08::f_002c4410__generated, 120, &delete_save_fixture);
    }

    /// Gate of the `cPS2MemCard_SetState` fake the request-wrapper tests use.
    pub(crate) const SET_STATE_WRAPPER_CTRL: u32 = DATA + 0xf_f520;

    pub(crate) const SET_STATE_WRAPPER_MAGIC: u32 = 0x4d43_5752;

    pub(crate) static SET_STATE_WRAPPER_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `cPS2MemCard_SetState` for the request-wrapper tests: log the call
    /// (every register: `a0` = this, `a1` = the state, the stored arguments,
    /// `v0`, `ra`) and return or unwind; with the gate off, the slot's
    /// previous body.
    pub(crate) fn fake_set_state_for_wrappers(c: &mut Cpu) {
        if c.bus.read32(SET_STATE_WRAPPER_CTRL) != SET_STATE_WRAPPER_MAGIC {
            return fakes::fall_back(c, &SET_STATE_WRAPPER_PREV, ee::s08::f_002c48c0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x002c_48c0, ra);
    }

    pub(crate) fn set_state_wrapper_install() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C48C0),
            fake_set_state_for_wrappers,
            &SET_STATE_WRAPPER_PREV,
        );
    }

    /// One request-wrapper call. Seed digits: `seed % 3` = 0 makes the
    /// `SetState` call unwind (else it returns); `seed / 3 % 5` places the
    /// card object (non-zero low bytes, pre-filled with junk so every store
    /// shows); `seed / 15 % 2` = 1 puts junk in the high half of `a0` (only
    /// its low word addresses the object; `v0 = a0` copies it whole). `a1`
    /// and `a2` keep the machine's full 64-bit junk: the stores keep their low
    /// words, and the fake logs what the wrapper leaves in each register.
    pub(crate) fn set_state_wrapper_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        c.bus
            .write32(SET_STATE_WRAPPER_CTRL, SET_STATE_WRAPPER_MAGIC);
        let unwind_at = if seed % 3 == 0 { 0 } else { u32::MAX };
        c.bus.write32(LOG_UNWIND_AT, unwind_at);
        let this = DATA + 0x1238 + (seed / 3 % 5) * 0x2_0134;
        fill(c, &mut r, this, 0x540);
        let high = if seed / 15 % 2 == 1 {
            (r.next_u32() as u64 | 1) << 32
        } else {
            0
        };
        c.g[A0][0] = high | this as u64;
    }

    /// Both arms of a request wrapper are taken: `SetState` returns (the
    /// wrapper returns to the caller's `ra`) and unwinds (the wrapper stops at
    /// the fake's unwind address).
    pub(crate) fn set_state_wrapper_arms(generated: GuestFn, salt: u64) {
        let (mut returned, mut unwound) = (0, 0);
        for seed in 0..30 {
            let mut c = fresh();
            set_state_wrapper_setup(&mut c, seed, salt);
            run_generated(&mut c, generated);
            if seed % 3 == 0 {
                assert_eq!(c.pc, 0xdead_0000, "seed {seed}: SetState did not unwind");
                unwound += 1;
            } else {
                assert_eq!(
                    c.pc,
                    crate::difftest::RA,
                    "seed {seed}: wrapper did not return"
                );
                returned += 1;
            }
        }
        assert!(returned > 0 && unwound > 0);
    }

    /// Differential test of one request wrapper against its generated body,
    /// with `SetState` faked (calls logged, unwinds on `seed % 3 == 0`).
    pub(crate) fn set_state_wrapper_check(generated: GuestFn, hand: GuestFn, salt: u64) {
        set_state_wrapper_install();
        set_state_wrapper_arms(generated, salt);
        fakes::covered(generated, hand, 300, &|c, seed| {
            set_state_wrapper_setup(c, seed, salt)
        });
    }

    #[test]
    fn c_ps2_mem_card_request_format_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c4368__generated,
            c_ps2_mem_card_request_format,
            0x2c_4368,
        );
    }

    #[test]
    fn c_ps2_mem_card_v32_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c48a0__generated,
            c_ps2_mem_card_v32,
            0x2c_48a0,
        );
    }

    #[test]
    fn c_ps2_mem_card_v43_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c4768__generated,
            c_ps2_mem_card_v43,
            0x2c_4768,
        );
    }

    #[test]
    fn c_ps2_mem_card_request_close_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c6a08__generated,
            c_ps2_mem_card_request_close,
            0x2c_6a08,
        );
    }

    #[test]
    fn c_ps2_mem_card_request_flush_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c6ae0__generated,
            c_ps2_mem_card_request_flush,
            0x2c_6ae0,
        );
    }

    #[test]
    fn c_ps2_mem_card_v76_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c4290__generated,
            c_ps2_mem_card_v76,
            0x2c_4290,
        );
    }

    #[test]
    fn c_ps2_mem_card_v10_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c43e0__generated,
            c_ps2_mem_card_v10,
            0x2c_43e0,
        );
    }

    #[test]
    fn c_ps2_mem_card_request_write_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c6bd0__generated,
            c_ps2_mem_card_request_write,
            0x2c_6bd0,
        );
    }

    #[test]
    fn c_ps2_mem_card_request_read_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c6b50__generated,
            c_ps2_mem_card_request_read,
            0x2c_6b50,
        );
    }

    #[test]
    fn c_ps2_mem_card_request_seek_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c6a60__generated,
            c_ps2_mem_card_request_seek,
            0x2c_6a60,
        );
    }

    #[test]
    fn c_ps2_mem_card_request_get_info_now_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c67e8__generated,
            c_ps2_mem_card_request_get_info_now,
            0x2c_67e8,
        );
    }

    #[test]
    fn c_ps2_mem_card_v30_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c4858__generated,
            c_ps2_mem_card_v30,
            0x2c_4858,
        );
    }

    #[test]
    fn c_ps2_mem_card_v41_matches() {
        set_state_wrapper_check(
            ee::s08::f_002c4720__generated,
            c_ps2_mem_card_v41,
            0x2c_4720,
        );
    }

    /// The gate word of this unit's port-getter fakes.
    pub(crate) const PORT_GETTER_FAKE_MAGIC: u32 = 0x4d43_5448;

    /// One gate word per faked callee.
    pub(crate) const PORT_GETTER_CTRL_CHANGED_FLAG: u32 = DATA + 0xf_f600;

    pub(crate) const PORT_GETTER_CTRL_PORT_ERROR: u32 = DATA + 0xf_f610;

    pub(crate) const PORT_GETTER_CTRL_FREE_SPACE: u32 = DATA + 0xf_f620;

    pub(crate) const PORT_GETTER_CTRL_SPRINTF: u32 = DATA + 0xf_f630;

    pub(crate) const PORT_GETTER_CTRL_STRCPY: u32 = DATA + 0xf_f640;

    pub(crate) const PORT_GETTER_CTRL_DELETE: u32 = DATA + 0xf_f650;

    /// The card object (word aligned, non-zero low bytes) and the size of
    /// the per-port info block (four ports of 0x14 bytes).
    pub(crate) const PORT_GETTER_THIS: u32 = DATA + 0x1238;

    pub(crate) const PORT_INFO_SIZE: u32 = 0x14;

    macro_rules! port_getter_fake {
        ($name:ident, $prev:ident, $ctrl:expr, $addr:expr, $gen:path) => {
            pub(crate) static $prev: AtomicUsize = AtomicUsize::new(0);
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32($ctrl) != PORT_GETTER_FAKE_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    port_getter_fake!(
        fake_changed_flag_for_port_getters,
        PREV_CHANGED_FLAG_FOR_PORT_GETTERS,
        PORT_GETTER_CTRL_CHANGED_FLAG,
        0x002c_53c8,
        ee::s08::f_002c53c8__generated
    );

    port_getter_fake!(
        fake_port_error_for_port_getters,
        PREV_PORT_ERROR_FOR_PORT_GETTERS,
        PORT_GETTER_CTRL_PORT_ERROR,
        0x002c_51d0,
        ee::s08::f_002c51d0__generated
    );

    port_getter_fake!(
        fake_free_space_for_port_getters,
        PREV_FREE_SPACE_FOR_PORT_GETTERS,
        PORT_GETTER_CTRL_FREE_SPACE,
        0x002c_5320,
        ee::s08::f_002c5320__generated
    );

    port_getter_fake!(
        fake_ustr_sprintf_for_port_getters,
        PREV_USTR_SPRINTF_FOR_PORT_GETTERS,
        PORT_GETTER_CTRL_SPRINTF,
        0x002c_26d0,
        ee::s08::f_002c26d0__generated
    );

    port_getter_fake!(
        fake_strcpy_for_port_getters,
        PREV_STRCPY_FOR_PORT_GETTERS,
        PORT_GETTER_CTRL_STRCPY,
        0x0041_66f4,
        ee::s15::f_004166f4__generated
    );

    port_getter_fake!(
        fake_delete_for_port_getters,
        PREV_DELETE_FOR_PORT_GETTERS,
        PORT_GETTER_CTRL_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    pub(crate) fn install_port_getter_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C53C8),
            fake_changed_flag_for_port_getters,
            &PREV_CHANGED_FLAG_FOR_PORT_GETTERS,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C51D0),
            fake_port_error_for_port_getters,
            &PREV_PORT_ERROR_FOR_PORT_GETTERS,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C5320),
            fake_free_space_for_port_getters,
            &PREV_FREE_SPACE_FOR_PORT_GETTERS,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            fake_ustr_sprintf_for_port_getters,
            &PREV_USTR_SPRINTF_FOR_PORT_GETTERS,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            fake_strcpy_for_port_getters,
            &PREV_STRCPY_FOR_PORT_GETTERS,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            fake_delete_for_port_getters,
            &PREV_DELETE_FOR_PORT_GETTERS,
        );
    }

    /// The one faked call unwinds (`LOG_UNWIND_AT` 0) or returns: a seed
    /// digit, not `machine`'s draw.
    pub(crate) fn unwind_first(c: &mut Cpu, unwind: bool) {
        c.bus
            .write32(LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
    }

    /// A pointer or index register: sign-extended, or with a junk upper half.
    pub(crate) fn junk_high(r: &mut Rng, low: u32, junk: bool) -> u64 {
        let high = if junk { (r.next_u32() as u64) << 32 } else { 0 };
        high | low as u64
    }

    /// A card whose vtable slot `slot` is `{delta, fake method}`: the delta is
    /// negative on even seeds and positive on odd ones (digit `seed % 2`);
    /// `a0`'s upper half is junk when `seed / 2 % 2` is 1. The fake unwinds
    /// when `seed / 4 % 3` is 0.
    pub(crate) fn mem_card_vthunk_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 1);
        unwind_first(c, seed / 4 % 3 == 0);
        let this = PORT_GETTER_THIS;
        let vt = DATA + 0x4564;
        let mag = (r.next_u32() % 0x7fff + 1) as i32;
        let delta = if seed % 2 == 0 { -mag } else { mag };
        c.bus.write16(vt + slot, delta as u16);
        c.bus.write16(vt + slot + 2, r.next_u32() as u16);
        c.bus.write32(vt + slot + 4, FAKE + 0x10);
        c.bus.write32(this, vt);
        fill(c, &mut r, this + 4, 0xc);
        c.g[A0][0] = junk_high(&mut r, this, seed / 2 % 2 == 1);
    }

    #[test]
    fn c_ps2_mem_card_v49_matches() {
        covered(
            ee::s09::f_002d37e0__generated,
            c_ps2_mem_card_v49,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2d_37e0, MCU_VT_IS_PORT_NOT_PS2_CARD),
        );
    }

    #[test]
    fn c_ps2_mem_card_v51_matches() {
        covered(
            ee::s08::f_002c67b0__generated,
            c_ps2_mem_card_v51,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2c_67b0, MCU_VT_V50),
        );
    }

    #[test]
    fn c_ps2_mem_card_v53_matches() {
        covered(
            ee::s09::f_002d3810__generated,
            c_ps2_mem_card_v53,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2d_3810, MCU_VT_V52),
        );
    }

    #[test]
    fn c_ps2_mem_card_v55_matches() {
        covered(
            ee::s09::f_002d38f0__generated,
            c_ps2_mem_card_v55,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2d_38f0, MCU_VT_IS_PORT_FORMATTED),
        );
    }

    #[test]
    fn c_ps2_mem_card_v23_matches() {
        covered(
            ee::s08::f_002c6778__generated,
            c_ps2_mem_card_v23,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2c_6778, MCU_VT_CALC_SAVE_SIZE_KB),
        );
    }

    #[test]
    fn c_ps2_mem_card_v62_matches() {
        covered(
            ee::s08::f_002c6718__generated,
            c_ps2_mem_card_v62,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2c_6718, MCU_VT_V61),
        );
    }

    #[test]
    fn c_ps2_mem_card_v71_matches() {
        covered(
            ee::s09::f_002d3980__generated,
            c_ps2_mem_card_v71,
            200,
            &|c, seed| mem_card_vthunk_setup(c, seed, 0x2d_3980, MCU_VT_GET_SLOT_NAME),
        );
    }

    /// A card at `PORT_GETTER_THIS` with random per-port info. Even seeds
    /// switch on the callee's fake (`gate`; any port, unwinding when
    /// `seed / 32 % 2` is 1); odd seeds run the real callee on port
    /// `seed / 2 % 4`. `a0`'s upper half is junk when `seed / 8 % 2` is 1.
    /// Returns the port's info block for the caller's per-callee words.
    pub(crate) fn port_getter_setup(c: &mut Cpu, seed: u32, salt: u64, gate: u32) -> (Rng, u32) {
        install_port_getter_fakes();
        let mut r = machine(c, seed, salt, 1);
        unwind_first(c, seed / 32 % 2 == 1);
        let this = PORT_GETTER_THIS;
        let faked = seed % 2 == 0;
        let port = if faked { r.next_u32() } else { seed / 2 % 4 };
        if faked {
            c.bus.write32(gate, PORT_GETTER_FAKE_MAGIC);
        }
        c.bus.write32(this + MCU_PORT, port);
        fill(c, &mut r, this + MCU_PORTS, 4 * PORT_INFO_SIZE / 4);
        c.g[A0][0] = junk_high(&mut r, this, seed / 8 % 2 == 1);
        let info = this + MCU_PORTS + (port % 4) * PORT_INFO_SIZE;
        (r, info)
    }

    #[test]
    fn c_ps2_mem_card_v63_matches() {
        // Real callee: 7 + 6 cycles.
        let setup = |c: &mut Cpu, seed: u32| {
            let (mut r, info) =
                port_getter_setup(c, seed, 0x2d_3930, PORT_GETTER_CTRL_CHANGED_FLAG);
            let changed = if seed / 16 % 2 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(info + 0x10, changed);
        };
        covered(
            ee::s09::f_002d3930__generated,
            c_ps2_mem_card_v63,
            200,
            &setup,
        );
        require_paths(ee::s09::f_002d3930__generated, 200, &[13], &setup);
    }

    #[test]
    fn c_ps2_mem_card_v64_matches() {
        // Real `GetFreeSpaceString` on each of its paths (digit `seed % 4`:
        // no save data, status -2, another status, status 0); on status 0 it
        // calls the localiser through a fake vtable slot and the gated
        // `UStr_Sprintf` fake; `seed / 32 % 3` the one that unwinds (2: none).
        let setup = |c: &mut Cpu, seed: u32| {
            install_port_getter_fakes();
            let mut r = machine(c, seed, 0x2d_3950, 2);
            let unwind = seed / 32 % 3;
            c.bus
                .write32(LOG_UNWIND_AT, if unwind < 2 { unwind } else { u32::MAX });
            c.bus
                .write32(PORT_GETTER_CTRL_SPRINTF, PORT_GETTER_FAKE_MAGIC);
            let this = PORT_GETTER_THIS;
            let port = seed / 4 % 4;
            c.bus.write32(this + MCU_PORT, port);
            fill(c, &mut r, this + MCU_PORTS, 4 * PORT_INFO_SIZE / 4);
            let info = this + MCU_PORTS + port * PORT_INFO_SIZE;
            let status = match seed % 4 {
                0 | 2 => match r.next_u32() {
                    0 | 0xffff_fffe => 5,
                    s => s,
                },
                1 => 0xffff_fffe,
                _ => 0,
            };
            let data = if seed % 4 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(info + 0xc, data);
            c.bus.write32(info + 8, status);
            let free = r.next_u32() & 0x7fff_ffff;
            let free = if seed / 16 % 2 == 0 {
                free | 0x8000_0000
            } else {
                free
            };
            c.bus.write32(info + 4, free);
            // The localiser object: *(gp-0x848) + 0x8c, its vtable at +4,
            // slot 0x20 = {delta, fake}.
            let world = DATA + 0x5674;
            let obj = DATA + 0x6788;
            let vt = DATA + 0x78a4;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x8c, obj);
            c.bus.write32(obj + 4, vt);
            let mag = (r.next_u32() % 0x7fff + 1) as i32;
            let delta = if seed / 32 % 2 == 0 { -mag } else { mag };
            c.bus.write16(vt + 0x20, delta as u16);
            c.bus.write32(vt + 0x24, FAKE + 0x20);
            let dst = DATA + 0x8a12;
            fill(c, &mut r, DATA + 0x8a10, 0x10);
            c.g[A0][0] = junk_high(&mut r, this, seed / 64 % 2 == 1);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | dst as u64;
        };
        covered(
            ee::s09::f_002d3950__generated,
            c_ps2_mem_card_v64,
            200,
            &setup,
        );
    }

    /// The port status word for the real `GetPortError` (digit
    /// `seed / 16 % 6`): 0 and 1 give fixed cycle totals.
    pub(crate) fn port_error_status(r: &mut Rng, seed: u32) -> u32 {
        match seed / 16 % 6 {
            0 => 0,
            1 => 1,
            2 => (-7i32) as u32,
            3 => (-10i32) as u32,
            4 => (-10001i32) as u32,
            _ => r.next_u32(),
        }
    }

    #[test]
    fn c_ps2_mem_card_get_current_port_error_matches() {
        // Real callee: 7 + 14 (status 0) and 7 + 12 (status 1).
        let setup = |c: &mut Cpu, seed: u32| {
            let (mut r, info) = port_getter_setup(c, seed, 0x2d_39b8, PORT_GETTER_CTRL_PORT_ERROR);
            let status = port_error_status(&mut r, seed);
            c.bus.write32(info + 8, status);
        };
        covered(
            ee::s09::f_002d39b8__generated,
            c_ps2_mem_card_get_current_port_error,
            200,
            &setup,
        );
        require_paths(ee::s09::f_002d39b8__generated, 200, &[21, 19], &setup);
    }

    #[test]
    fn c_ps2_mem_card_get_free_space_matches() {
        // Real callee: 7 + 5 cycles.
        let setup = |c: &mut Cpu, seed: u32| {
            port_getter_setup(c, seed, 0x2c_5338, PORT_GETTER_CTRL_FREE_SPACE);
        };
        covered(
            ee::s08::f_002c5338__generated,
            c_ps2_mem_card_get_free_space,
            200,
            &setup,
        );
        require_paths(ee::s08::f_002c5338__generated, 200, &[12], &setup);
    }

    #[test]
    fn c_ps2_mem_card_get_save_name_matches() {
        // Slot `seed % 6`; strcpy faked when `seed / 6 % 2` is 0 (unwinding
        // when `seed / 24 % 2` is 1), else real on a 0..7-character name;
        // slot and dst registers carry junk upper halves when `seed / 12 % 2`
        // is 1.
        let setup = |c: &mut Cpu, seed: u32| {
            install_port_getter_fakes();
            let mut r = machine(c, seed, 0x2c_54d0, 1);
            unwind_first(c, seed / 24 % 2 == 1);
            if seed / 6 % 2 == 0 {
                c.bus
                    .write32(PORT_GETTER_CTRL_STRCPY, PORT_GETTER_FAKE_MAGIC);
            }
            let this = PORT_GETTER_THIS;
            for i in 0..6 {
                let name = this + MCU_ENTRIES + MCU_ENTRY_NAME + i * MCU_ENTRY_SIZE;
                fill(c, &mut r, name, 4);
                let len = r.next_u32() % 8;
                for k in 0..len {
                    c.bus.write8(name + k, (r.next_u32() % 94 + 33) as u8);
                }
                c.bus.write8(name + len, 0);
            }
            let dst = DATA + 0x9a35;
            fill(c, &mut r, DATA + 0x9a30, 4);
            let junk = seed / 12 % 2 == 1;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = junk_high(&mut r, seed % 6, junk);
            c.g[A2][0] = junk_high(&mut r, dst, junk);
        };
        covered(
            ee::s08::f_002c54d0__generated,
            c_ps2_mem_card_get_save_name,
            240,
            &setup,
        );
        check_lohi(
            ee::s08::f_002c54d0__generated,
            c_ps2_mem_card_get_save_name,
            240,
            &setup,
        );
    }

    #[test]
    fn c_ps2_mem_card_v45_matches() {
        // Real UStr_ToAscii on a title of `seed % 6` characters: 7 + 6 for the
        // empty one, 7 + 7 + 7n otherwise.
        check_paths(
            ee::s08::f_002c5500__generated,
            c_ps2_mem_card_v45,
            200,
            &[13, 21, 28, 35, 42, 49],
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_5500, 0);
                let this = PORT_GETTER_THIS;
                fill(c, &mut r, this + MCU_NAME, 0x10);
                let title = DATA + 0x2346;
                let len = seed % 6;
                for k in 0..len {
                    c.bus.write16(title + 2 * k, (r.next_u32() as u16) | 1);
                }
                c.bus.write16(title + 2 * len, 0);
                c.g[A0][0] = junk_high(&mut r, this, seed / 6 % 2 == 1);
                c.g[A1][0] = title as u64;
            },
        );
    }

    #[test]
    fn c_ps2_mem_card_v73_matches() {
        // Real callee: 8 + 14 (status 0, result 0) and 8 + 12 (status 1).
        let setup = |c: &mut Cpu, seed: u32| {
            let (mut r, info) = port_getter_setup(c, seed, 0x2c_5230, PORT_GETTER_CTRL_PORT_ERROR);
            let status = port_error_status(&mut r, seed);
            c.bus.write32(info + 8, status);
        };
        covered(
            ee::s08::f_002c5230__generated,
            c_ps2_mem_card_v73,
            200,
            &setup,
        );
        require_paths(ee::s08::f_002c5230__generated, 200, &[22, 20], &setup);
    }

    #[test]
    fn c_ps2_mem_card_base_dtor_matches() {
        // Delete flag bit 0 from `seed % 2`: 7 + 3 cycles without the call;
        // with it the gated `__builtin_delete` fake returns or unwinds
        // (`seed / 4 % 2`).
        let setup = |c: &mut Cpu, seed: u32| {
            install_port_getter_fakes();
            let mut r = machine(c, seed, 0x2d_3788, 1);
            unwind_first(c, seed / 4 % 2 == 1);
            c.bus
                .write32(PORT_GETTER_CTRL_DELETE, PORT_GETTER_FAKE_MAGIC);
            let this = PORT_GETTER_THIS;
            fill(c, &mut r, this, 4);
            c.g[A0][0] = junk_high(&mut r, this, seed / 2 % 2 == 1);
            let flags = ((r.next_u32() as u64) << 32 | r.next_u32() as u64) & !1;
            c.g[A1][0] = flags | (seed % 2) as u64;
        };
        covered(
            ee::s09::f_002d3788__generated,
            c_ps2_mem_card_base_dtor,
            200,
            &setup,
        );
        require_paths(ee::s09::f_002d3788__generated, 200, &[10], &setup);
    }

    /// `LocStringTable_ResolveString`: the forwarder runs the real
    /// `LocStringTable_GetStringPtr` on the table pointer at +4.
    #[test]
    fn loc_string_table_resolve_string_matches() {
        const LT: u32 = DATA + 0x6000;
        const TABLE: u32 = DATA + 0x6100;
        check(
            ee::s08::f_002c6dc0__generated,
            loc_string_table_resolve_string,
            200,
            &|c, seed| {
                let mut r = Rng(0x2c6d_c0 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[RA][0] = 0x0012_3450;
                c.pc = 0x002c_6dc0;
                c.g[A0][0] = sx(LT);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(LT + 4, TABLE);
                fill(c, &mut r, TABLE, 0x40);
                c.bus.write8(TABLE + 8, (r.next_u32() & !1) as u8);
            },
        );
    }

    pub(crate) use crate::game::sound::soundsys::tests::oc769_install_fakes;
    pub(crate) use crate::game::sound::soundsys::tests::oc769_machine;
    pub(crate) use crate::game::sound::soundsys::tests::OC769_CTRL;
    pub(crate) use crate::game::sound::soundsys::tests::OC769_MAGIC;

    pub(crate) use crate::game::sound::soundsys::tests::OC769_THIS;

    /// Gated fakes for the callees that have an override slot (allocators,
    /// file loaders, the sound and song callees). `strlen`, `strcmp`,
    /// `strncmp`, `memset`, `UI_AsciiToWide`, `LocStringTable_GetInfo`,
    /// `SongInstance_Create`/`_ctor` and the syscall wrapper run for real.
    macro_rules! oc769_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC769_CTRL) != OC769_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc769_install_plain_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc769_fakes!(
        (
            oc769_fake_2ade88,
            OC769_PREV_2ADE88,
            ee::s08::OVERRIDE_002ADE88,
            ee::s08::f_002ade88__generated,
            0x002a_de88
        ),
        (
            oc769_fake_317e30,
            OC769_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc769_fake_317d70,
            OC769_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc769_fake_3ba020,
            OC769_PREV_3BA020,
            ee::s13::OVERRIDE_003BA020,
            ee::s13::f_003ba020__generated,
            0x003b_a020
        ),
        (
            oc769_fake_2c6dc0,
            OC769_PREV_2C6DC0,
            ee::s08::OVERRIDE_002C6DC0,
            ee::s08::f_002c6dc0__generated,
            0x002c_6dc0
        ),
        (
            oc769_fake_2b3ac0,
            OC769_PREV_2B3AC0,
            ee::s08::OVERRIDE_002B3AC0,
            ee::s08::f_002b3ac0__generated,
            0x002b_3ac0
        ),
        (
            oc769_fake_417828,
            OC769_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc769_fake_2b4a20,
            OC769_PREV_2B4A20,
            ee::s08::OVERRIDE_002B4A20,
            ee::s08::f_002b4a20__generated,
            0x002b_4a20
        ),
        (
            oc769_fake_2b21e0,
            OC769_PREV_2B21E0,
            ee::s08::OVERRIDE_002B21E0,
            ee::s08::f_002b21e0__generated,
            0x002b_21e0
        ),
        (
            oc769_fake_2b43d8,
            OC769_PREV_2B43D8,
            ee::s08::OVERRIDE_002B43D8,
            ee::s08::f_002b43d8__generated,
            0x002b_43d8
        ),
        (
            oc769_fake_2ae478,
            OC769_PREV_2AE478,
            ee::s08::OVERRIDE_002AE478,
            ee::s08::f_002ae478__generated,
            0x002a_e478
        ),
    );

    /// `LocStringTable_Load` on a fixture file with 0..3 strings, a header
    /// magic that matches (the table is walked and each id resolved by the
    /// fake) or not (every third seed: header cleared), and a fake unwinding
    /// in the file load or one of the resolves.
    #[test]
    fn loc_string_table_load_matches() {
        oc769_install_fakes();
        const FILE: u32 = DATA + 0x21_0000;
        const MAGIC_STR: u32 = 0x004a_3978;
        check(
            ee::s08::f_002c6e98__generated,
            loc_string_table_load,
            90,
            &|c, seed| {
                let mut r = oc769_machine(c, seed, 4);
                // The file loader returns FILE (forced result of fake call 0).
                c.bus.write32(fakes::LOG_RESULTS, FILE);
                c.bus.write32(OC769_CTRL + 8, seed % 4);
                // `GetInfo(file, 2)`: halfword at +0xe (0), or'ed with v1.
                c.bus.write16(FILE + 0xe, 0);
                // header at FILE + 0x40, table at header + 0x100.
                c.bus.write32(FILE + 4, 0x40);
                c.bus.write32(FILE + 0x40 + 4, 0x100);
                c.bus.write32(MAGIC_STR, u32::from_le_bytes(*b"LOCS"));
                let magic = if seed % 3 == 1 {
                    u32::from_le_bytes(*b"XOCS")
                } else {
                    u32::from_le_bytes(*b"LOCS")
                };
                c.bus.write32(FILE + 0x40, magic);
                for i in 0..4 {
                    c.bus.write32(FILE + 0x140 + 8 * i, r.next_u32());
                    c.bus.write32(FILE + 0x140 + 8 * i + 4, r.next_u32());
                }
                c.g[A0][0] = sx(OC769_THIS);
                c.g[A1][0] = sx(OC769_THIS + 0x1000);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x002c_6e98;
            },
        );
    }

    /// `UStr_ToAscii`: empty and 1..=9 unit strings (high bytes arbitrary, units nonzero).
    #[test]
    fn ustr_to_ascii_matches() {
        check(
            ee::s08::f_002c2580__generated,
            ustr_to_ascii,
            40,
            &|c, seed| {
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                let n = seed % 10;
                for i in 0..n {
                    let unit = 0x0100 * (i + 1) + 0x41 + (seed + i) % 20;
                    c.bus.write16(DATA + 0x2000 + 2 * i, unit as u16);
                }
                c.bus.write16(DATA + 0x2000 + 2 * n, 0);
                c.g[V0][0] = 0x1234;
                c.g[V1][0] = 0x5678;
            },
        );
    }

    pub(crate) use crate::game::sound::bankmonitor::tests::oc1043_box_r;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc1043_box_w;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc1043_fake;

    pub(crate) use crate::game::sound::bankmonitor::tests::OC1043_CTRL;
    pub(crate) use crate::game::sound::bankmonitor::tests::OC1043_MAGIC;

    // ----------------------------------------------------------------------
    // QuadTree_Compact.

    oc1043_fake!(
        oc1043_fake_qt_alloc,
        OC1043_PREV_QT_ALLOC,
        ee::s10::f_00317e30__generated,
        |c, a| {
            let p = oc1043_box_r(c, 0x30);
            oc1043_box_w(c, 0x30, (p + a[0] + 15) & !15);
            c.g[V0][0] = sx(p);
        }
    );

    oc1043_fake!(
        oc1043_fake_qt_flatten,
        OC1043_PREV_QT_FLATTEN,
        ee::s08::f_002c2088__generated,
        |c, a| {
            let n = oc1043_box_r(c, 0x40);
            c.bus.write32(a[2], n);
            for k in 0..n * 2 {
                let v = oc1043_box_r(c, 0x48 + k);
                c.bus.write32(a[1] + 4 * k, v);
            }
        }
    );

    oc1043_fake!(
        oc1043_fake_qt_set_children,
        OC1043_PREV_QT_SET_CHILDREN,
        ee::s08::f_002c16b8__generated,
        |_c, _a| {}
    );

    /// A tree of 0-4 nodes (leaf and inner records) exported through three
    /// callbacks (fixInt, fixShort, fixFloat at fake addresses).
    #[test]
    fn quad_tree_compact_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1043_fake_qt_alloc,
            &OC1043_PREV_QT_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2088),
            oc1043_fake_qt_flatten,
            &OC1043_PREV_QT_FLATTEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C16B8),
            oc1043_fake_qt_set_children,
            &OC1043_PREV_QT_SET_CHILDREN,
        );
        covered(
            ee::s08::f_002c1da8__generated,
            quad_tree_compact,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_1da8, 12);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let me = DATA + 0x2f_8000;
                fill(c, &mut r, me, 10);
                c.bus.write32(me + 0xc, r.next_u32() % 8);
                c.bus.write32(me + 0x10, r.next_u32() % 5);
                c.bus.write16(me + 0x14, (r.next_u32() % 4) as u16);
                c.bus.write32(me + 0x20, DATA + 0x2f_a000);
                oc1043_box_w(c, 0x30, DATA + 0x30_0000);
                let n = r.next_u32() % 5;
                oc1043_box_w(c, 0x40, n);
                for k in 0..8 {
                    // nodes alternate between inner (bit 0 set) and leaf records
                    let v = r.next_u32();
                    oc1043_box_w(
                        c,
                        0x48 + k,
                        if k % 2 == 0 && seed % 3 != 0 {
                            v | 1
                        } else {
                            v
                        },
                    );
                }
                c.g[A0][0] = me as u64;
                c.g[A1][0] = (DATA + 0x2f_9000) as u64;
                c.g[A2][0] = (DATA + 0x2f_9010) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = (FAKE + 0x100) as u64;
                c.g[T1][0] = (FAKE + 0x110) as u64;
                c.g[T2][0] = (FAKE + 0x120) as u64;
            },
        );
    }

    /// Gate word of this module's fakes (far above the fixtures' data).
    pub(crate) const OC1052_CTRL: u32 = DATA + 0x0020_0000;

    pub(crate) const OC1052_MAGIC: u32 = 0x1052_c0de;

    /// Machine with scrambled registers and the fakes' gate on.
    pub(crate) fn oc1052_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc1052_hooks();
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC1052_CTRL, OC1052_MAGIC);
        r
    }

    /// A callee fake: logs the call and returns (gate on), or runs the callee it
    /// replaced (gate off, another test in this binary).
    macro_rules! oc1052_fake {
        ($fake:ident, $prev:ident, $target:expr, $gen:path) => {
            static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1052_CTRL) != OC1052_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    oc1052_fake!(
        oc1052_fake_002b6cf8,
        OC1052_PREV_002B6CF8,
        0x002b_6cf8,
        ee::s08::f_002b6cf8__generated
    );

    oc1052_fake!(
        oc1052_fake_002b77c0,
        OC1052_PREV_002B77C0,
        0x002b_77c0,
        ee::s08::f_002b77c0__generated
    );

    oc1052_fake!(
        oc1052_fake_0028b1c8,
        OC1052_PREV_0028B1C8,
        0x0028_b1c8,
        ee::s07::f_0028b1c8__generated
    );

    oc1052_fake!(
        oc1052_fake_00328f28,
        OC1052_PREV_00328F28,
        0x0032_8f28,
        ee::s10::f_00328f28__generated
    );

    oc1052_fake!(
        oc1052_fake_003284b8,
        OC1052_PREV_003284B8,
        0x0032_84b8,
        ee::s10::f_003284b8__generated
    );

    oc1052_fake!(
        oc1052_fake_00317e50,
        OC1052_PREV_00317E50,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );

    oc1052_fake!(
        oc1052_fake_00317e98,
        OC1052_PREV_00317E98,
        0x0031_7e98,
        ee::s10::f_00317e98__generated
    );

    oc1052_fake!(
        oc1052_fake_00317e30,
        OC1052_PREV_00317E30,
        0x0031_7e30,
        ee::s10::f_00317e30__generated
    );

    oc1052_fake!(
        oc1052_fake_0011b978,
        OC1052_PREV_0011B978,
        0x0011_b978,
        ee::s00::f_0011b978__generated
    );

    oc1052_fake!(
        oc1052_fake_002c1a00,
        OC1052_PREV_002C1A00,
        0x002c_1a00,
        ee::s08::f_002c1a00__generated
    );

    oc1052_fake!(
        oc1052_fake_002c0b70,
        OC1052_PREV_002C0B70,
        0x002c_0b70,
        ee::s08::f_002c0b70__generated
    );

    /// Installs the fakes above (once per process; each is gated).
    pub(crate) fn oc1052_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6CF8),
            oc1052_fake_002b6cf8,
            &OC1052_PREV_002B6CF8,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B77C0),
            oc1052_fake_002b77c0,
            &OC1052_PREV_002B77C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B1C8),
            oc1052_fake_0028b1c8,
            &OC1052_PREV_0028B1C8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328F28),
            oc1052_fake_00328f28,
            &OC1052_PREV_00328F28,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003284B8),
            oc1052_fake_003284b8,
            &OC1052_PREV_003284B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1052_fake_00317e50,
            &OC1052_PREV_00317E50,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1052_fake_00317e98,
            &OC1052_PREV_00317E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1052_fake_00317e30,
            &OC1052_PREV_00317E30,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0011B978),
            oc1052_fake_0011b978,
            &OC1052_PREV_0011B978,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C1A00),
            oc1052_fake_002c1a00,
            &OC1052_PREV_002C1A00,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C0B70),
            oc1052_fake_002c0b70,
            &OC1052_PREV_002C0B70,
        );
    }

    #[test]
    fn quad_tree_alloc_node_matches() {
        check(
            ee::s08::f_002c17d0__generated,
            quad_tree_alloc_node,
            200,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0004);
                let tree = DATA + 0x2000;
                c.g[A0][0] = sx(tree);
                c.bus.write32(tree + 0x10, seed % 5);
                c.bus.write32(tree + 0xc, seed);
                // Free list present (the first branch), or empty with an old array or none.
                let (first, cap) = match seed % 3 {
                    0 => (DATA + 0x3000, DATA + 0x3008),
                    1 => (DATA + 0x3000, DATA + 0x3000),
                    _ => (DATA + 0x3000, 0),
                };
                c.bus.write32(tree + 0x24, first);
                c.bus.write32(tree + 0x20, cap);
                Addr(DATA + 0x3000).sh(c, 0x0102);
            },
        );
    }

    #[test]
    fn quad_tree_insert_matches() {
        check(
            ee::s08::f_002c1950__generated,
            quad_tree_insert,
            200,
            &|c, seed| {
                let mut r = oc1052_machine(c, seed, 0x1052_0005);
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(node);
                c.g[A1][0] = sx(DATA + 0x4000);
                // Limits chosen so the three branches of the descent are taken.
                let (a2, a3) = match seed % 3 {
                    0 => (0x8000u64, 0x7000u64),
                    1 => (0x7000, 0x8000),
                    _ => (0x7000, 0x7000),
                };
                c.g[A2][0] = a2;
                c.g[A3][0] = a3;
                c.bus.write32(node, r.f32(1000.0));
                c.bus.write32(node + 4, r.f32(1000.0));
                c.bus.write32(node + 8, r.f32(1000.0));
                c.bus.write32(node + 0x20, DATA + 0x5000);
                Addr(node + 0x14).sh(c, seed as u64 % 0x100);
                c.g[A3][0] = a3;
            },
        );
    }

    #[test]
    fn quad_tree_count_nodes_matches() {
        check(
            ee::s08::f_002c2178__generated,
            quad_tree_count_nodes,
            200,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0006);
                let tree = DATA + 0x1000;
                let base = DATA + 0x20000;
                c.g[A0][0] = sx(tree);
                c.bus.write32(tree + 0x20, base);
                // Node at DATA+0x30000: a leaf (even first halfword), or odd with
                // children whose halfwords (in the zeroed pool) make them leaves.
                let node = DATA + 0x30000;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { sx(node) };
                let first = if seed % 4 == 1 { 0u16 } else { 1 };
                Addr(node).sh(c, first as u64);
                for k in 1..4u32 {
                    Addr(node + 2 * k).sh(c, ((seed % 7 + k) * 2 + 1) as u64);
                }
            },
        );
    }

    #[test]
    fn game_module_unload_stub_matches() {
        check(
            ee::s08::f_002c3fa0__generated,
            game_module_unload_stub,
            50,
            &|c, seed| {
                oc1052_machine(c, seed, 0x1052_0007);
            },
        );
    }

    #[test]
    fn ps2_mem_card_defer_state_18_matches() {
        check_paths(
            ee::s08::f_002c4758__generated,
            ps2_mem_card_defer_state_18,
            40,
            &[3],
            &|c, seed| {
                machine(c, seed, 0x002c_4758, 0);
                let this = DATA + 0x1c1;
                c.bus.write32(this + 0x48, [0, 1][seed as usize % 2]);
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x002c_4758;
            },
        );
    }

    #[test]
    fn ps2_mem_card_defer_state_1e_matches() {
        check_paths(
            ee::s08::f_002c4890__generated,
            ps2_mem_card_defer_state_1e,
            40,
            &[3],
            &|c, seed| {
                machine(c, seed, 0x002c_4890, 0);
                let this = DATA + 0x1d1;
                c.bus.write32(this + 0x44, [0, 1][seed as usize % 2]);
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x002c_4890;
            },
        );
    }

    pub(crate) fn fixture_const(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x151) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = entry;
    }

    #[test]
    fn ps2_mem_card_get_490_matches() {
        check_paths(
            ee::s08::f_002c4400__generated,
            ps2_mem_card_get_490,
            40,
            &[2],
            &|c, seed| {
                machine(c, seed, 0x002c_4400, 0);
                let this = DATA + 0x161;
                c.bus.write32(this + 0x490, 0x1122_3300 | (seed + 7));
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x002c_4400;
            },
        );
    }

    #[test]
    fn ps2_mem_card_set_24_matches() {
        check_paths(
            ee::s08::f_002c4408__generated,
            ps2_mem_card_set_24,
            40,
            &[2],
            &|c, seed| {
                machine(c, seed, 0x002c_4408, 0);
                let this = DATA + 0x171;
                c.bus.write32(this + 0x24, 0xdead_beef);
                c.g[A0][0] = this as u64;
                // `A1` low byte nonzero so a byte-lane mix-up would show.
                c.g[A1][0] = (0x0102_0300 | (seed + 9)) as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x002c_4408;
            },
        );
    }

    #[test]
    fn ps2_mem_card_v46_matches() {
        check_paths(
            ee::s08::f_002c5520__generated,
            ps2_mem_card_v46,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x002c_5520, 0x002c_5520),
        );
    }

    #[test]
    fn ps2_mem_card_v57_matches() {
        check_paths(
            ee::s08::f_002c6700__generated,
            ps2_mem_card_v57,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x002c_6700, 0x002c_6700),
        );
    }

    #[test]
    fn ps2_mem_card_v58_matches() {
        check_paths(
            ee::s08::f_002c6708__generated,
            ps2_mem_card_v58,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x002c_6708, 0x002c_6708),
        );
    }

    #[test]
    fn ps2_mem_card_v61_matches() {
        check_paths(
            ee::s08::f_002c6710__generated,
            ps2_mem_card_v61,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x002c_6710, 0x002c_6710),
        );
    }

    #[test]
    fn ps2_mem_card_get_block_size_matches() {
        check_paths(
            ee::s08::f_002c6748__generated,
            ps2_mem_card_get_block_size,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x002c_6748, 0x002c_6748),
        );
    }

    #[test]
    fn ps2_mem_card_v21_matches() {
        check_paths(
            ee::s08::f_002c6750__generated,
            ps2_mem_card_v21,
            16,
            &[2],
            &|c, seed| fixture_const(c, seed, 0x002c_6750, 0x002c_6750),
        );
    }

    /// Fixture for the `+0x14` reader: `A0` has a nonzero low byte and the
    /// word varies per seed.
    pub(crate) fn fixture_14(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x002d_3970, 0);
        let this = DATA + 0x121;
        c.bus.write32(this + 0x14, 0x0a0b_0c00 | (seed + 3));
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x002d_3970;
    }

    /// Fixture for the `+0xc` reader.
    pub(crate) fn fixture_0c(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x002d_37d8, 0);
        let this = DATA + 0x131;
        c.bus.write32(this + 0xc, 0x0506_0700 | (seed + 5));
        c.g[A0][0] = this as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = 0x002d_37d8;
    }

    /// Fixture for the constant-return slots: nothing read, but `A0` still
    /// points at scratch so a stray load would fault.
    pub(crate) fn fixture_const_2(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        machine(c, seed, salt, 0);
        c.g[A0][0] = (DATA + 0x141) as u64;
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = entry;
    }

    #[test]
    fn ps2_mem_card_v50_matches() {
        check_paths(
            ee::s08::f_002c67a8__generated,
            ps2_mem_card_v50,
            16,
            &[2],
            &|c, seed| fixture_const_2(c, seed, 0x002c_67a8, 0x002c_67a8),
        );
    }

    #[test]
    fn ps2_mem_card_v25_matches() {
        check_paths(
            ee::s08::f_002c67e0__generated,
            ps2_mem_card_v25,
            16,
            &[2],
            &|c, seed| fixture_const_2(c, seed, 0x002c_67e0, 0x002c_67e0),
        );
    }

    #[test]
    fn ps2_mem_card_get_current_port_matches() {
        check_paths(
            ee::s09::f_002d37d8__generated,
            ps2_mem_card_get_current_port,
            40,
            &[2],
            &fixture_0c,
        );
    }

    #[test]
    fn ps2_mem_card_v59_matches() {
        check_paths(
            ee::s09::f_002d3920__generated,
            ps2_mem_card_v59,
            16,
            &[2],
            &|c, seed| fixture_const_2(c, seed, 0x002d_3920, 0x002d_3920),
        );
    }

    #[test]
    fn ps2_mem_card_v60_matches() {
        check_paths(
            ee::s09::f_002d3928__generated,
            ps2_mem_card_v60,
            16,
            &[2],
            &|c, seed| fixture_const_2(c, seed, 0x002d_3928, 0x002d_3928),
        );
    }

    #[test]
    fn ps2_mem_card_get_14_matches() {
        check_paths(
            ee::s09::f_002d3970__generated,
            ps2_mem_card_get_14,
            40,
            &[2],
            &fixture_14,
        );
    }

    #[test]
    fn ps2_mem_card_v69_matches() {
        check_paths(
            ee::s09::f_002d3978__generated,
            ps2_mem_card_v69,
            16,
            &[2],
            &|c, seed| fixture_const_2(c, seed, 0x002d_3978, 0x002d_3978),
        );
    }

    #[test]
    fn ps2_mem_card_v72_matches() {
        check_paths(
            ee::s09::f_002d39a8__generated,
            ps2_mem_card_v72,
            16,
            &[2],
            &|c, seed| fixture_const_2(c, seed, 0x002d_39a8, 0x002d_39a8),
        );
    }

    #[test]
    fn ps2_mem_card_is_sync_done_matches() {
        check_paths(
            ee::s09::f_002d39b0__generated,
            ps2_mem_card_is_sync_done,
            40,
            &[2],
            &|c, seed| {
                machine(c, seed, 0x002d_39b0, 0);
                let this = DATA + 0x111;
                c.bus.write32(this + 0x3c, 0x0102_0300 | (seed + 1));
                c.g[A0][0] = this as u64;
                c.g[RA][0] = SENTINEL_RA as u64;
                c.pc = 0x002d_39b0;
            },
        );
    }

    use crate::difftest::fakes::machine_rt;
    pub(crate) use crate::game::lib::snd::tests::leaf_fill;
    pub(crate) use crate::game::worldpainter::worldpaintermanquery::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::worldpainter::worldpaintermanquery::tests::SALT;

    /// `LocStringTable_GetInfo`: every kind (0, 1, 2, 3, other, negative).
    #[test]
    fn loc_string_table_get_info_matches() {
        check(
            ee::s14::f_003db9d8__generated,
            loc_string_table_get_info,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_b9d8, 0);
                let record = DATA + 0x1000;
                for i in 0..8u32 {
                    c.bus.write32(record + 4 * i, r.next_u32());
                }
                c.g[A0][0] = record as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => u64::MAX,
                    _ => 5,
                };
            },
        );
    }

    /// `LocStringTable_CompareId` (0x003d_bac8) against the generated body.
    #[test]
    fn loc_string_table_compare_id_003dbac8_matches() {
        check(
            ee::s14::f_003dbac8__generated,
            loc_string_table_compare_id_003dbac8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3dbac8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// The memcpy source.
    pub(crate) const SRC: u32 = DATA + 0x20_2000;

    /// Gate word: the gated fakes answer only while it holds `MAGIC`.
    pub(crate) const GATE: u32 = DATA + 0x20_8000;

    /// Mailbox: word 0 is the MEM_Free result, word 1 the resize result.
    pub(crate) const MAILBOX: u32 = GATE + 4;

    pub(crate) const MAGIC: u32 = 0x1552_0a5e;

    pub(crate) static PREV_MEM_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_RESIZE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_LOCK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_UNLOCK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn gated(c: &mut Cpu) -> bool {
        c.bus.read32(GATE) == MAGIC
    }

    /// MEM_Free while gated: returns the mailbox word (zero included).
    pub(crate) fn oc1552_fake_mem_free(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_MEM_FREE, ee::s06::f_002523a8__generated);
        }
        c.g[V0][0] = c.bus.read32(MAILBOX) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// The cMemMan resize while gated: returns the second mailbox word.
    pub(crate) fn oc1552_fake_resize(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_RESIZE, ee::s10::f_00319bd8__generated);
        }
        c.g[V0][0] = c.bus.read32(MAILBOX + 4) as u64;
        c.pc = c.g[RA][0] as u32;
    }

    /// The mutex lock and unlock while gated: they return at once.
    pub(crate) fn oc1552_fake_lock(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_LOCK, ee::s14::f_003e5700__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn oc1552_fake_unlock(c: &mut Cpu) {
        if !gated(c) {
            return fakes::fall_back(c, &PREV_UNLOCK, ee::s14::f_003e5760__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// Puts the gated fakes in their override slots (once per process).
    pub(crate) fn oc1552_hook_all() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002523A8),
            oc1552_fake_mem_free,
            &PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00319BD8),
            oc1552_fake_resize,
            &PREV_RESIZE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            oc1552_fake_lock,
            &PREV_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            oc1552_fake_unlock,
            &PREV_UNLOCK,
        );
    }

    /// Opens the gate and sets the mailbox; some seeds get a zero result.
    pub(crate) fn oc1552_gate_on(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.bus.write32(GATE, MAGIC);
        let free = if seed % 3 == 0 { 0 } else { r.next_u32() };
        let resize = if seed % 5 == 0 { 0 } else { r.next_u32() };
        c.bus.write32(MAILBOX, free);
        c.bus.write32(MAILBOX + 4, resize);
    }

    #[test]
    fn istr_alloc_matches() {
        // The allocator (0x317e30) is real: the runtime fixture gives it a heap.
        check(
            ee::s08::f_002c2d30__generated,
            istr_alloc,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, SALT, 0);
                c.g[A0][0] = sx(OBJ_2);
                c.g[A1][0] = sx(1 + (r.next_u32() & 0xff));
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn istr_free_matches() {
        oc1552_hook_all();
        check(
            ee::s08::f_002c2d68__generated,
            istr_free,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, SALT, 0);
                oc1552_gate_on(c, seed, &mut r);
                c.g[A0][0] = sx(OBJ_2);
            },
        );
    }

    #[test]
    fn istr_mem_copy_matches() {
        check(
            ee::s08::f_002c2d88__generated,
            istr_mem_copy,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, SALT, 0);
                fill(c, &mut r, SRC, 0x20);
                c.g[A0][0] = sx(OBJ_2);
                c.g[A1][0] = sx(SRC);
                c.g[A2][0] = (r.next_u32() % 0x40) as u64;
            },
        );
    }

    #[test]
    fn istr_realloc_matches() {
        oc1552_hook_all();
        // Seeds cover the three paths: no block (allocate), size 0 (free) and
        // a resize under the mutex.
        check(
            ee::s08::f_002c2db0__generated,
            istr_realloc,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, SALT, 0);
                oc1552_gate_on(c, seed, &mut r);
                c.g[A0][0] = sx(if seed % 3 == 0 { 0 } else { OBJ_2 + 0x40 });
                c.g[A1][0] = sx(OBJ_2);
                c.g[A2][0] = if seed % 4 == 0 {
                    0
                } else {
                    1 + (r.next_u32() & 0xff) as u64
                };
                c.g[A3][0] = (r.next_u32() & 1) as u64;
            },
        );
    }

    #[test]
    fn loc_string_table_remap_id_matches() {
        check(
            ee::s14::f_003dbae8__generated,
            loc_string_table_remap_id,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_bae8 ^ seed as u64);
                scramble(c, &mut r);
                let index = DATA + 0x3000;
                // entry count (0..6) in the byte word at +8, sorted 4-byte entries
                // at +0x10: key in the low half, value in the high half
                let count = r.next_u32() % 7;
                c.bus.write32(index + 8, count);
                for i in 0..count {
                    let value = r.next_u32() & 0xffff;
                    c.bus.write32(index + 0x10 + 4 * i, 10 * i | (value << 16));
                }
                // a key that is present on most seeds
                let key = if seed % 5 == 0 {
                    r.next_u32() & 0xffff
                } else {
                    10 * (r.next_u32() % 7)
                };
                c.g[A0][0] = sx(key);
                c.g[A1][0] = sx(index);
            },
        );
    }

    /// Quad tree at `DATA+0x1000`; its node array (8-byte nodes) at `DATA+0x10000`
    /// with a free list of 192 nodes from index 0x40 (so the real `AllocNode`,
    /// `SetChildren`, `SetLeaf` and `FreeChildren` all take their short paths).
    /// Root node index 1: a leaf, or an internal node with four leaf children
    /// whose values are all equal (merge) or drawn from two values.
    #[test]
    fn quad_tree_insert_recursive_matches() {
        const TREE: u32 = DATA + 0x1000;
        const BASE: u32 = DATA + 0x1_0000;
        check_paths(
            ee::s08::f_002c1a00__generated,
            quad_tree_insert_recursive_oc1599,
            400,
            &[],
            &|c, seed| {
                let mut r = machine(c, seed, 0x1599_0001, 0);
                c.bus.write32(TREE + 0x20, BASE);
                c.bus.write32(TREE + 0x24, BASE + 0x40 * 8);
                c.bus.write32(TREE + 0xc, seed % 9);
                c.bus.write32(TREE + 0x10, 0x400);
                for i in 0x40..0x100u32 {
                    c.bus.write16(BASE + i * 8, (i + 1) as u16);
                }
                let node = |i: u32| BASE + i * 8;
                let x_old = r.next_u32() & !1;
                let y_old = r.next_u32();
                let same = seed % 3 == 0;
                match seed % 4 {
                    // Leaf root.
                    0 | 2 => {
                        c.bus.write32(node(1), x_old);
                        c.bus.write32(node(1) + 4, y_old);
                    }
                    // Internal root, four leaf children.
                    _ => {
                        c.bus.write16(node(1), (2 << 1) | 1);
                        c.bus.write16(node(1) + 2, 3 << 1);
                        c.bus.write16(node(1) + 4, 4 << 1);
                        c.bus.write16(node(1) + 6, 5 << 1);
                        for k in 0..4u32 {
                            let alt = !same && (seed / 4 >> k) & 1 == 1;
                            c.bus
                                .write32(node(2 + k), if alt { x_old ^ 0x10 } else { x_old });
                            c.bus.write32(node(2 + k) + 4, y_old);
                        }
                    }
                }
                // New value: sometimes the old one (merges), sometimes odd (an
                // "internal" marker over garbage children).
                let (mut new_b, mut new_a) = match seed % 5 {
                    0 | 1 => (x_old, y_old),
                    _ => (r.next_u32() & !1, r.next_u32()),
                };
                if seed % 13 == 0 {
                    new_b = (new_b & 0xffff) | 1;
                    new_a &= 0xffff;
                }
                let sizes = [0u32, 1, 2, 3, 4, 8, 16];
                c.g[A0][0] = sx(TREE);
                c.g[A1][0] = sx(node(1));
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = sx(sizes[(seed as usize / 2) % 7]);
                c.g[T1][0] = sx(new_a);
                c.g[T2][0] = sx(new_b);
            },
        );
    }
}
