//! The shared-login store (`SLogin_*`), its memory-card layer (`SLoginMc_*`)
//! and record store (`SLoginStore_*`), the CRC-32 checksum service
//! (`Crc32_*`) and the DNAS relay loader (`DnasRel_*`): hand-written
//! replacements. The reference has no source unit for these, so they are
//! grouped here by hand (`tools/reorg_modules.py` would put them in `misc.rs`).

use crate::abi;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `lui` page of this unit's code: the step and done callbacks it stores as
/// function pointers.
const CODE_PAGE: u64 = 0x003f_0000;

/// `lui` page of the unit's read-only data: the store template, step and
/// dispatch tables and the section-name strings at 0x497xxx.
const DATA_PAGE: u64 = 0x0049_0000;

/// `lui` page of the login globals at 0x4a4bc8..0x4a4be0.
const GLOBALS_PAGE: u64 = 0x004a_0000;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003f_6c50, slogin_set_active, "SLogin_SetActive"),
    (0x003f_6c60, slogin_clear_active, "SLogin_ClearActive"),
    (0x003f_6c70, slogin_alloc_state, "SLogin_AllocState"),
    (0x003f_6d98, slogin_free_state, "SLogin_FreeState"),
    (0x003f_6dc8, slogin_init_store, "SLogin_InitStore"),
    (0x003f_6e70, slogin_shutdown_store, "SLogin_ShutdownStore"),
    (0x003f_6e98, slogin_start_op, "SLogin_StartOp"),
    (0x003f_6ee0, slogin_op_update, "SLogin_OpUpdate"),
    (0x003f_6f68, slogin_step_update, "SLogin_StepUpdate"),
    (0x003f_71b8, slogin_on_step_error, "SLogin_OnStepError"),
    (0x003f_71f8, slogin_on_final_error, "SLogin_OnFinalError"),
    (0x003f_7218, slogin_on_step_done, "SLogin_OnStepDone"),
    (0x003f_72c8, slogin_on_final_done, "SLogin_OnFinalDone"),
    (
        0x003f_72f8,
        slogin_on_error_close_done,
        "SLogin_OnErrorCloseDone",
    ),
    (0x003f_7328, slogin_begin_sequence, "SLogin_BeginSequence"),
    (0x003f_73a8, slogin_step_create, "SLogin_StepCreate"),
    (0x003f_73f0, slogin_step_open, "SLogin_StepOpen"),
    (0x003f_7438, slogin_step_delete, "SLogin_StepDelete"),
    (0x003f_7478, slogin_step_write_acct, "SLogin_StepWriteAcct"),
    (0x003f_7528, slogin_step_write_pass, "SLogin_StepWritePass"),
    (0x003f_75d8, slogin_step_read_acct, "SLogin_StepReadAcct"),
    (0x003f_7658, slogin_step_read_pass, "SLogin_StepReadPass"),
    (0x003f_76d8, slogin_step_close, "SLogin_StepClose"),
    (0x003f_7708, slogin_step_poll, "SLogin_StepPoll"),
    (
        0x003f_77f8,
        slogin_fetch_store_info,
        "SLogin_FetchStoreInfo",
    ),
    (0x003f_7820, slogin_on_open_done, "SLogin_OnOpenDone"),
    (
        0x003f_7840,
        slogin_on_read_acct_done,
        "SLogin_OnReadAcctDone",
    ),
    (
        0x003f_7890,
        slogin_on_read_pass_done,
        "SLogin_OnReadPassDone",
    ),
    (
        0x003f_78e0,
        slogin_on_write_acct_done,
        "SLogin_OnWriteAcctDone",
    ),
    (
        0x003f_7900,
        slogin_on_write_pass_done,
        "SLogin_OnWritePassDone",
    ),
    (
        0x003f_7920,
        slogin_free_acct_record,
        "SLogin_FreeAcctRecord",
    ),
    (
        0x003f_7958,
        slogin_free_pass_record,
        "SLogin_FreePassRecord",
    ),
    (0x003f_7990, slogin_reset_for_op, "SLogin_ResetForOp"),
    (0x003f_7a30, slogin_finish_op, "SLogin_FinishOp"),
    (0x003f_7a60, slogin_set_step, "SLogin_SetStep"),
    (0x003f_7bd0, slogin_end_sequence, "SLogin_EndSequence"),
    (0x003f_7c00, slogin_init_step_funcs, "SLogin_InitStepFuncs"),
    (
        0x003f_7d98,
        slogin_clear_step_funcs,
        "SLogin_ClearStepFuncs",
    ),
    (
        0x003f_7de0,
        slogin_verify_icon_checksum,
        "SLogin_VerifyIconChecksum",
    ),
    (0x003f_7e90, slogin_str_copy_n, "SLogin_StrCopyN"),
    (0x003f_7ee0, slogin_init, "SLogin_Init"),
    (0x003f_7f18, slogin_shutdown, "SLogin_Shutdown"),
    (0x003f_7f48, slogin_create, "SLogin_Create"),
    (0x003f_8000, slogin_load, "SLogin_Load"),
    (0x003f_8048, slogin_update, "SLogin_Update"),
    (0x003f_80c0, slogin_delete, "SLogin_Delete"),
    (0x003f_80e0, slogin_poll, "SLogin_Poll"),
    (0x003f_8178, slogin_map_store_error, "SLogin_MapStoreError"),
    (
        0x003f_8230,
        slogin_get_record_alloc_size,
        "SLogin_GetRecordAllocSize",
    ),
    (0x003f_8258, slogin_setup_icon_sys, "SLogin_SetupIconSys"),
    (
        0x003f_8550,
        slogin_mc_first_device_in_mask,
        "SLoginMc_FirstDeviceInMask",
    ),
    (
        0x003f_8588,
        slogin_mc_last_device_in_mask,
        "SLoginMc_LastDeviceInMask",
    ),
    (
        0x003f_85c0,
        slogin_mc_next_device_in_mask,
        "SLoginMc_NextDeviceInMask",
    ),
    (
        0x003f_86b8,
        slogin_mc_count_devices_in_mask,
        "SLoginMc_CountDevicesInMask",
    ),
    (0x003f_86f0, slogin_mc_set_error, "SLoginMc_SetError"),
    (0x003f_8710, slogin_mc_get_error, "SLoginMc_GetError"),
    (
        0x003f_8720,
        slogin_mc_abort_at_flush,
        "SLoginMc_AbortAtFlush",
    ),
    (
        0x003f_8768,
        slogin_mc_abort_at_close,
        "SLoginMc_AbortAtClose",
    ),
    (
        0x003f_87b0,
        slogin_mc_abort_at_finish,
        "SLoginMc_AbortAtFinish",
    ),
    (0x003f_8c38, slogin_mc_open_step, "SLoginMc_OpenStep"),
    (0x003f_8ef8, slogin_mc_io_step, "SLoginMc_IoStep"),
    (0x003f_8ff0, slogin_mc_close_step, "SLoginMc_CloseStep"),
    (0x003f_9130, slogin_mc_abort_step, "SLoginMc_AbortStep"),
    (
        0x003f_9208,
        slogin_mc_is_initialized,
        "SLoginMc_IsInitialized",
    ),
    (0x003f_9218, slogin_mc_init, "SLoginMc_Init"),
    (0x003f_9530, slogin_mc_shutdown, "SLoginMc_Shutdown"),
    (
        0x003f_95e8,
        slogin_mc_get_file_state,
        "SLoginMc_GetFileState",
    ),
    (0x003f_9698, slogin_mc_start_delete, "SLoginMc_StartDelete"),
    (0x003f_9818, slogin_mc_start_create, "SLoginMc_StartCreate"),
    (0x003f_99b8, slogin_mc_start_open, "SLoginMc_StartOpen"),
    (0x003f_9b38, slogin_mc_start_close, "SLoginMc_StartClose"),
    (0x003f_9c48, slogin_mc_start_seek, "SLoginMc_StartSeek"),
    (0x003f_9dc0, slogin_mc_start_read, "SLoginMc_StartRead"),
    (0x003f_9f00, slogin_mc_start_write, "SLoginMc_StartWrite"),
    (0x003f_a040, slogin_mc_poll, "SLoginMc_Poll"),
    (0x003f_a1d0, slogin_mc_get_port_slot, "SLoginMc_GetPortSlot"),
    (0x003f_a240, slogin_mc_set_icon_sys, "SLoginMc_SetIconSys"),
    (
        0x003f_a360,
        slogin_mc_is_available_wrapper,
        "SLoginMc_IsAvailable_Wrapper",
    ),
    (0x003f_a380, slogin_mc_ascii_to_sjis, "SLoginMc_AsciiToSjis"),
    (
        0x003f_a6a8,
        slogin_mc_calc_save_size,
        "SLoginMc_CalcSaveSize",
    ),
    (
        0x003f_a6c8,
        slogin_mc_get_checksum_iface,
        "SLoginMc_GetChecksumIface",
    ),
    (
        0x003f_a6e8,
        slogin_mc_get_data_offset,
        "SLoginMc_GetDataOffset",
    ),
    (
        0x003f_a708,
        slogin_mc_split_device_index,
        "SLoginMc_SplitDeviceIndex",
    ),
    (
        0x003f_a7a0,
        slogin_mc_build_dir_path,
        "SLoginMc_BuildDirPath",
    ),
    (
        0x003f_a818,
        slogin_mc_build_file_paths,
        "SLoginMc_BuildFilePaths",
    ),
    (0x003f_a8f8, slogin_mc_create_step, "SLoginMc_CreateStep"),
    (0x003f_ae50, slogin_mc_delete_step, "SLoginMc_DeleteStep"),
    (0x003f_b278, slogin_mc_is_available, "SLoginMc_IsAvailable"),
    (
        0x003f_b280,
        slogin_mc_calc_save_size_impl,
        "SLoginMc_CalcSaveSizeImpl",
    ),
    (
        0x003f_b2c8,
        slogin_mc_get_checksum_iface_impl,
        "SLoginMc_GetChecksumIfaceImpl",
    ),
    (0x003f_b2f8, slogin_mc_set_busy, "SLoginMc_SetBusy"),
    (
        0x003f_b300,
        slogin_mc_get_data_offset_impl,
        "SLoginMc_GetDataOffsetImpl",
    ),
    (
        0x003f_b308,
        slogin_mc_validate_icon_sys,
        "SLoginMc_ValidateIconSys",
    ),
    (0x003f_b318, crc32_table_entry, "Crc32_TableEntry"),
    (0x003f_b358, crc32_build_table, "Crc32_BuildTable"),
    (0x003f_b3a0, crc32_init, "Crc32_Init"),
    (0x003f_b430, crc32_shutdown, "Crc32_Shutdown"),
    (0x003f_b5e8, crc32_get_interface, "Crc32_GetInterface"),
    (
        0x003f_b800,
        slogin_store_get_cipher_iface,
        "SLoginStore_GetCipherIface",
    ),
    (
        0x003f_b810,
        slogin_store_map_mc_error,
        "SLoginStore_MapMcError",
    ),
    (
        0x003f_b828,
        slogin_store_map_cipher_error,
        "SLoginStore_MapCipherError",
    ),
    (
        0x003f_b840,
        slogin_store_map_hash_error,
        "SLoginStore_MapHashError",
    ),
    (
        0x003f_b858,
        slogin_store_get_alignment,
        "SLoginStore_GetAlignment",
    ),
    (
        0x003f_b898,
        slogin_store_calc_alloc_size,
        "SLoginStore_CalcAllocSize",
    ),
    (
        0x003f_b910,
        slogin_store_get_block_header,
        "SLoginStore_GetBlockHeader",
    ),
    (
        0x003f_b958,
        slogin_store_set_record_header,
        "SLoginStore_SetRecordHeader",
    ),
    (
        0x003f_b9a0,
        slogin_store_write_sentinel,
        "SLoginStore_WriteSentinel",
    ),
    (
        0x003f_b9d0,
        slogin_store_get_record_header,
        "SLoginStore_GetRecordHeader",
    ),
    (
        0x003f_ba20,
        slogin_store_find_record,
        "SLoginStore_FindRecord",
    ),
    (
        0x003f_ba78,
        slogin_store_set_record_crc,
        "SLoginStore_SetRecordCrc",
    ),
    (
        0x003f_bad0,
        slogin_store_add_record,
        "SLoginStore_AddRecord",
    ),
    (0x003f_bb50, slogin_store_hash, "SLoginStore_Hash"),
    (
        0x003f_bbe0,
        slogin_store_hash_directory,
        "SLoginStore_HashDirectory",
    ),
    (0x003f_bc40, slogin_store_encrypt, "SLoginStore_Encrypt"),
    (0x003f_bcd8, slogin_store_decrypt, "SLoginStore_Decrypt"),
    (
        0x003f_bd70,
        slogin_store_begin_open,
        "SLoginStore_BeginOpen",
    ),
    (
        0x003f_bdb8,
        slogin_store_seek_to_start,
        "SLoginStore_SeekToStart",
    ),
    (
        0x003f_bdf0,
        slogin_store_read_dir_header,
        "SLoginStore_ReadDirHeader",
    ),
    (
        0x003f_be40,
        slogin_store_load_dir_entry,
        "SLoginStore_LoadDirEntry",
    ),
    (
        0x003f_bed0,
        slogin_store_write_record,
        "SLoginStore_WriteRecord",
    ),
    (
        0x003f_bf80,
        slogin_store_read_record,
        "SLoginStore_ReadRecord",
    ),
    (
        0x003f_bfe0,
        slogin_store_verify_record,
        "SLoginStore_VerifyRecord",
    ),
    (
        0x003f_c0d8,
        slogin_store_is_initialized,
        "SLoginStore_IsInitialized",
    ),
    (0x003f_c0e8, slogin_store_init, "SLoginStore_Init"),
    (0x003f_c260, slogin_store_shutdown, "SLoginStore_Shutdown"),
    (0x003f_c390, slogin_store_create, "SLoginStore_Create"),
    (0x003f_c3c0, slogin_store_delete, "SLoginStore_Delete"),
    (0x003f_c4c8, slogin_store_open, "SLoginStore_Open"),
    (0x003f_c4e8, slogin_store_close, "SLoginStore_Close"),
    (0x003f_c548, slogin_store_write, "SLoginStore_Write"),
    (0x003f_c708, slogin_store_read, "SLoginStore_Read"),
    (
        0x003f_c850,
        slogin_store_count_records,
        "SLoginStore_CountRecords",
    ),
    (
        0x003f_c8f8,
        slogin_store_get_record_size,
        "SLoginStore_GetRecordSize",
    ),
    (0x003f_c990, slogin_store_get_info, "SLoginStore_GetInfo"),
    (0x003f_ca40, slogin_store_poll, "SLoginStore_Poll"),
    (
        0x003f_cdc8,
        slogin_store_set_icon_sys,
        "SLoginStore_SetIconSys",
    ),
    (
        0x003f_ce10,
        slogin_store_alloc_block,
        "SLoginStore_AllocBlock",
    ),
    (
        0x003f_cf00,
        slogin_store_free_block,
        "SLoginStore_FreeBlock",
    ),
    (
        0x003f_cf98,
        slogin_store_get_block_size,
        "SLoginStore_GetBlockSize",
    ),
    (0x003f_cfd8, dnas_rel_load, "DnasRel_Load"),
    (0x003f_d0d8, dnas_rel_call_init, "DnasRel_CallInit"),
    (0x003f_d100, dnas_rel_call_update, "DnasRel_CallUpdate"),
    (0x003f_d128, dnas_rel_call_exit, "DnasRel_CallExit"),
    (0x003f_d150, dnas_rel_run, "DnasRel_Run"),
    (0x003f_d1d8, dnas_rel_is_elf, "DnasRel_IsElf"),
    (
        0x003f_d1f0,
        dnas_rel_get_section_header,
        "DnasRel_GetSectionHeader",
    ),
    (0x003f_d210, dnas_rel_find_section, "DnasRel_FindSection"),
    (
        0x003f_d2b8,
        dnas_rel_get_section_addr,
        "DnasRel_GetSectionAddr",
    ),
    (
        0x003f_d2f8,
        dnas_rel_relocate_section,
        "DnasRel_RelocateSection",
    ),
    (0x003f_d558, dnas_rel_elf_open, "DnasRel_ElfOpen"),
    (0x003f_d660, dnas_rel_relocate_all, "DnasRel_RelocateAll"),
    (0x003f_d6e0, dnas_rel_get_symbol, "DnasRel_GetSymbol"),
    (0x003f_d758, dnas_rel_find_symbol, "DnasRel_FindSymbol"),
    (0x003f_77f0, slogin_step_nop, "SLogin_StepNop"),
    (
        0x003f_87f8,
        slogin_mc_sync_result_cmd00,
        "SLoginMc_SyncResult_Cmd00",
    ),
    (
        0x003f_8800,
        slogin_mc_sync_result_cmd01,
        "SLoginMc_SyncResult_Cmd01",
    ),
    (
        0x003f_8838,
        slogin_mc_sync_result_cmd02,
        "SLoginMc_SyncResult_Cmd02",
    ),
    (
        0x003f_8880,
        slogin_mc_sync_result_cmd07,
        "SLoginMc_SyncResult_Cmd07",
    ),
    (
        0x003f_8908,
        slogin_mc_sync_result_cmd08,
        "SLoginMc_SyncResult_Cmd08",
    ),
    (
        0x003f_8968,
        slogin_mc_sync_result_cmd09,
        "SLoginMc_SyncResult_Cmd09",
    ),
    (
        0x003f_89c8,
        slogin_mc_sync_result_cmd1a,
        "SLoginMc_SyncResult_Cmd1A",
    ),
    (
        0x003f_8a08,
        slogin_mc_sync_result_cmd05,
        "SLoginMc_SyncResult_Cmd05",
    ),
    (
        0x003f_8a58,
        slogin_mc_sync_result_cmd06,
        "SLoginMc_SyncResult_Cmd06",
    ),
    (
        0x003f_8ac0,
        slogin_mc_sync_result_cmd13,
        "SLoginMc_SyncResult_Cmd13",
    ),
    (
        0x003f_8b28,
        slogin_mc_sync_result_cmd12,
        "SLoginMc_SyncResult_Cmd12",
    ),
    (
        0x003f_8b78,
        slogin_mc_sync_result_cmd03,
        "SLoginMc_SyncResult_Cmd03",
    ),
    (
        0x003f_8bb0,
        slogin_mc_sync_result_cmd04,
        "SLoginMc_SyncResult_Cmd04",
    ),
    (
        0x003f_8be8,
        slogin_mc_sync_result_cmd17,
        "SLoginMc_SyncResult_Cmd17",
    ),
    (0x003f_8c30, slogin_mc_idle_step, "SLoginMc_IdleStep"),
];

pub(crate) mod callee {
    use super::*;

    pub(crate) const SLOGIN_RESET_FOR_OP: Callee = Callee {
        addr: 0x003f_7990,
        f: ee::s14::SLogin_ResetForOp,
    };

    pub(crate) const SLOGIN_END_SEQUENCE: Callee = Callee {
        addr: 0x003f_7bd0,
        f: ee::s14::SLogin_EndSequence,
    };

    pub(crate) const SLOGIN_SET_STEP: Callee = Callee {
        addr: 0x003f_7a60,
        f: ee::s14::SLogin_SetStep,
    };

    pub(crate) const SLOGIN_MAP_STORE_ERROR: Callee = Callee {
        addr: 0x003f_8178,
        f: ee::s14::SLogin_MapStoreError,
    };

    pub(crate) const SLOGIN_CLEAR_STEP_FUNCS: Callee = Callee {
        addr: 0x003f_7d98,
        f: ee::s14::SLogin_ClearStepFuncs,
    };

    pub(crate) const SLOGIN_FETCH_STORE_INFO: Callee = Callee {
        addr: 0x003f_77f8,
        f: ee::s14::SLogin_FetchStoreInfo,
    };

    pub(crate) const SLOGIN_FREE_ACCT_RECORD: Callee = Callee {
        addr: 0x003f_7920,
        f: ee::s14::SLogin_FreeAcctRecord,
    };

    pub(crate) const SLOGIN_FREE_PASS_RECORD: Callee = Callee {
        addr: 0x003f_7958,
        f: ee::s14::SLogin_FreePassRecord,
    };

    pub(crate) const SLOGIN_INIT_STEP_FUNCS: Callee = Callee {
        addr: 0x003f_7c00,
        f: ee::s14::SLogin_InitStepFuncs,
    };

    pub(crate) const SLOGIN_START_OP: Callee = Callee {
        addr: 0x003f_6e98,
        f: ee::s14::SLogin_StartOp,
    };

    pub(crate) const SLOGIN_STR_COPY_N: Callee = Callee {
        addr: 0x003f_7e90,
        f: ee::s14::SLogin_StrCopyN,
    };

    pub(crate) const SLOGIN_SHUTDOWN_STORE: Callee = Callee {
        addr: 0x003f_6e70,
        f: ee::s14::SLogin_ShutdownStore,
    };

    pub(crate) const SLOGIN_FREE_STATE: Callee = Callee {
        addr: 0x003f_6d98,
        f: ee::s14::SLogin_FreeState,
    };

    pub(crate) const SLOGIN_CLEAR_ACTIVE: Callee = Callee {
        addr: 0x003f_6c60,
        f: ee::s14::SLogin_ClearActive,
    };

    pub(crate) const SLOGIN_FINISH_OP: Callee = Callee {
        addr: 0x003f_7a30,
        f: ee::s14::SLogin_FinishOp,
    };

    pub(crate) const SLOGIN_VERIFY_ICON_CHECKSUM: Callee = Callee {
        addr: 0x003f_7de0,
        f: ee::s14::SLogin_VerifyIconChecksum,
    };

    pub(crate) const SLOGIN_SETUP_ICON_SYS: Callee = Callee {
        addr: 0x003f_8258,
        f: ee::s14::SLogin_SetupIconSys,
    };

    pub(crate) const SLOGIN_SET_ACTIVE: Callee = Callee {
        addr: 0x003f_6c50,
        f: ee::s14::SLogin_SetActive,
    };

    pub(crate) const SLOGIN_ALLOC_STATE: Callee = Callee {
        addr: 0x003f_6c70,
        f: ee::s14::SLogin_AllocState,
    };

    pub(crate) const SLOGIN_INIT_STORE: Callee = Callee {
        addr: 0x003f_6dc8,
        f: ee::s14::SLogin_InitStore,
    };

    pub(crate) const SLOGIN_MC_IS_INITIALIZED: Callee = Callee {
        addr: 0x003f_9208,
        f: ee::s14::SLoginMc_IsInitialized,
    };

    pub(crate) const SLOGIN_MC_GET_ERROR: Callee = Callee {
        addr: 0x003f_8710,
        f: ee::s14::SLoginMc_GetError,
    };

    pub(crate) const SLOGIN_MC_SET_ERROR: Callee = Callee {
        addr: 0x003f_86f0,
        f: ee::s14::SLoginMc_SetError,
    };

    pub(crate) const SLOGIN_MC_ABORT_AT_FLUSH: Callee = Callee {
        addr: 0x003f_8720,
        f: ee::s14::SLoginMc_AbortAtFlush,
    };

    pub(crate) const SLOGIN_MC_ABORT_AT_CLOSE: Callee = Callee {
        addr: 0x003f_8768,
        f: ee::s14::SLoginMc_AbortAtClose,
    };

    pub(crate) const SLOGIN_MC_ABORT_AT_FINISH: Callee = Callee {
        addr: 0x003f_87b0,
        f: ee::s14::SLoginMc_AbortAtFinish,
    };

    pub(crate) const SLOGIN_MC_GET_DATA_OFFSET: Callee = Callee {
        addr: 0x003f_a6e8,
        f: ee::s14::SLoginMc_GetDataOffset,
    };

    pub(crate) const SLOGIN_MC_CALC_SAVE_SIZE: Callee = Callee {
        addr: 0x003f_a6a8,
        f: ee::s14::SLoginMc_CalcSaveSize,
    };

    pub(crate) const SLOGIN_MC_COUNT_DEVICES_IN_MASK: Callee = Callee {
        addr: 0x003f_86b8,
        f: ee::s14::SLoginMc_CountDevicesInMask,
    };

    pub(crate) const SLOGIN_MC_FIRST_DEVICE_IN_MASK: Callee = Callee {
        addr: 0x003f_8550,
        f: ee::s14::SLoginMc_FirstDeviceInMask,
    };

    pub(crate) const SLOGIN_MC_LAST_DEVICE_IN_MASK: Callee = Callee {
        addr: 0x003f_8588,
        f: ee::s14::SLoginMc_LastDeviceInMask,
    };

    pub(crate) const SLOGIN_MC_SPLIT_DEVICE_INDEX: Callee = Callee {
        addr: 0x003f_a708,
        f: ee::s14::SLoginMc_SplitDeviceIndex,
    };

    pub(crate) const SLOGIN_MC_GET_CHECKSUM_IFACE_IMPL: Callee = Callee {
        addr: 0x003f_b2c8,
        f: ee::s15::SLoginMc_GetChecksumIfaceImpl,
    };

    pub(crate) const SLOGIN_MC_GET_DATA_OFFSET_IMPL: Callee = Callee {
        addr: 0x003f_b300,
        f: ee::s15::SLoginMc_GetDataOffsetImpl,
    };

    pub(crate) const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };

    pub(crate) const MEMCPY: Callee = Callee {
        addr: 0x0041_605c,
        f: ee::s15::memcpy,
    };

    pub(crate) const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };

    pub(crate) const TIB_EXT_MEM_ALLOC: Callee = Callee {
        addr: 0x0026_6728,
        f: ee::s07::TibExtMemAlloc,
    };

    pub(crate) const TIB_EXT_MEM_FREE: Callee = Callee {
        addr: 0x0026_6760,
        f: ee::s07::TibExtMemFree,
    };

    pub(crate) const CRC32_GET_INTERFACE: Callee = Callee {
        addr: 0x003f_b5e8,
        f: ee::s15::Crc32_GetInterface,
    };

    pub(crate) const CRC32_TABLE_ENTRY: Callee = Callee {
        addr: 0x003f_b318,
        f: ee::s15::Crc32_TableEntry,
    };

    pub(crate) const CRC32_BUILD_TABLE: Callee = Callee {
        addr: 0x003f_b358,
        f: ee::s15::Crc32_BuildTable,
    };

    pub(crate) const SLOGIN_MC_START_OPEN: Callee = Callee {
        addr: 0x003f_99b8,
        f: ee::s14::SLoginMc_StartOpen,
    };

    pub(crate) const SLOGIN_MC_START_SEEK: Callee = Callee {
        addr: 0x003f_9c48,
        f: ee::s14::SLoginMc_StartSeek,
    };

    pub(crate) const SLOGIN_MC_START_READ: Callee = Callee {
        addr: 0x003f_9dc0,
        f: ee::s14::SLoginMc_StartRead,
    };

    pub(crate) const SLOGIN_MC_START_WRITE: Callee = Callee {
        addr: 0x003f_9f00,
        f: ee::s14::SLoginMc_StartWrite,
    };

    pub(crate) const SLOGIN_STORE_MAP_MC_ERROR: Callee = Callee {
        addr: 0x003f_b810,
        f: ee::s15::SLoginStore_MapMcError,
    };

    pub(crate) const SLOGIN_STORE_MAP_CIPHER_ERROR: Callee = Callee {
        addr: 0x003f_b828,
        f: ee::s15::SLoginStore_MapCipherError,
    };

    pub(crate) const SLOGIN_STORE_MAP_HASH_ERROR: Callee = Callee {
        addr: 0x003f_b840,
        f: ee::s15::SLoginStore_MapHashError,
    };

    pub(crate) const SLOGIN_STORE_CALC_ALLOC_SIZE: Callee = Callee {
        addr: 0x003f_b898,
        f: ee::s15::SLoginStore_CalcAllocSize,
    };

    pub(crate) const SLOGIN_STORE_GET_BLOCK_HEADER: Callee = Callee {
        addr: 0x003f_b910,
        f: ee::s15::SLoginStore_GetBlockHeader,
    };

    pub(crate) const SLOGIN_STORE_SET_RECORD_HEADER: Callee = Callee {
        addr: 0x003f_b958,
        f: ee::s15::SLoginStore_SetRecordHeader,
    };

    pub(crate) const SLOGIN_STORE_GET_RECORD_HEADER: Callee = Callee {
        addr: 0x003f_b9d0,
        f: ee::s15::SLoginStore_GetRecordHeader,
    };

    pub(crate) const SLOGIN_STORE_ADD_RECORD: Callee = Callee {
        addr: 0x003f_bad0,
        f: ee::s15::SLoginStore_AddRecord,
    };

    pub(crate) const SLOGIN_STORE_HASH: Callee = Callee {
        addr: 0x003f_bb50,
        f: ee::s15::SLoginStore_Hash,
    };

    pub(crate) const SLOGIN_STORE_ENCRYPT: Callee = Callee {
        addr: 0x003f_bc40,
        f: ee::s15::SLoginStore_Encrypt,
    };

    pub(crate) const SLOGIN_STORE_DECRYPT: Callee = Callee {
        addr: 0x003f_bcd8,
        f: ee::s15::SLoginStore_Decrypt,
    };

    pub(crate) const SLOGIN_MC_INIT: Callee = Callee {
        addr: 0x003f_9218,
        f: ee::s14::SLoginMc_Init,
    };

    pub(crate) const SLOGIN_MC_SHUTDOWN: Callee = Callee {
        addr: 0x003f_9530,
        f: ee::s14::SLoginMc_Shutdown,
    };

    pub(crate) const SLOGIN_MC_START_CREATE: Callee = Callee {
        addr: 0x003f_9818,
        f: ee::s14::SLoginMc_StartCreate,
    };

    pub(crate) const SLOGIN_MC_START_DELETE: Callee = Callee {
        addr: 0x003f_9698,
        f: ee::s14::SLoginMc_StartDelete,
    };

    pub(crate) const SLOGIN_MC_START_CLOSE: Callee = Callee {
        addr: 0x003f_9b38,
        f: ee::s14::SLoginMc_StartClose,
    };

    pub(crate) const SLOGIN_MC_GET_CHECKSUM_IFACE: Callee = Callee {
        addr: 0x003f_a6c8,
        f: ee::s14::SLoginMc_GetChecksumIface,
    };

    pub(crate) const SLOGIN_STORE_IS_INITIALIZED: Callee = Callee {
        addr: 0x003f_c0d8,
        f: ee::s15::SLoginStore_IsInitialized,
    };

    pub(crate) const SLOGIN_STORE_WRITE_SENTINEL: Callee = Callee {
        addr: 0x003f_b9a0,
        f: ee::s15::SLoginStore_WriteSentinel,
    };

    pub(crate) const SLOGIN_STORE_FIND_RECORD: Callee = Callee {
        addr: 0x003f_ba20,
        f: ee::s15::SLoginStore_FindRecord,
    };

    pub(crate) const SLOGIN_STORE_SET_RECORD_CRC: Callee = Callee {
        addr: 0x003f_ba78,
        f: ee::s15::SLoginStore_SetRecordCrc,
    };

    pub(crate) const SLOGIN_STORE_HASH_DIRECTORY: Callee = Callee {
        addr: 0x003f_bbe0,
        f: ee::s15::SLoginStore_HashDirectory,
    };

    pub(crate) const SLOGIN_STORE_BEGIN_OPEN: Callee = Callee {
        addr: 0x003f_bd70,
        f: ee::s15::SLoginStore_BeginOpen,
    };

    pub(crate) const SLOGIN_STORE_ALLOC_BLOCK: Callee = Callee {
        addr: 0x003f_ce10,
        f: ee::s15::SLoginStore_AllocBlock,
    };

    pub(crate) const SLOGIN_STORE_FREE_BLOCK: Callee = Callee {
        addr: 0x003f_cf00,
        f: ee::s15::SLoginStore_FreeBlock,
    };

    pub(crate) const MEMCMP: Callee = Callee {
        addr: 0x0041_5fc8,
        f: ee::s15::memcmp,
    };

    pub(crate) const DNAS_REL_RELOCATE_SECTION: Callee = Callee {
        addr: 0x003f_d2f8,
        f: ee::s15::DnasRel_RelocateSection,
    };

    pub(crate) const DNAS_REL_GET_SECTION_ADDR: Callee = Callee {
        addr: 0x003f_d2b8,
        f: ee::s15::DnasRel_GetSectionAddr,
    };

    pub(crate) const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub(crate) const DNAS_REL_IS_ELF: Callee = Callee {
        addr: 0x003f_d1d8,
        f: ee::s15::DnasRel_IsElf,
    };

    pub(crate) const DNAS_REL_FIND_SECTION: Callee = Callee {
        addr: 0x003f_d210,
        f: ee::s15::DnasRel_FindSection,
    };

    pub(crate) const STRCMP: Callee = Callee {
        addr: 0x0041_65a8,
        f: ee::s15::strcmp,
    };

    pub(crate) const DNAS_REL_GET_SYMBOL: Callee = Callee {
        addr: 0x003f_d6e0,
        f: ee::s15::DnasRel_GetSymbol,
    };

    pub(crate) const SLOGIN_MC_BUILD_DIR_PATH: Callee = Callee {
        addr: 0x003f_a7a0,
        f: ee::s14::SLoginMc_BuildDirPath,
    };

    pub(crate) const SLOGIN_MC_NEXT_DEVICE_IN_MASK: Callee = Callee {
        addr: 0x003f_85c0,
        f: ee::s14::SLoginMc_NextDeviceInMask,
    };

    pub(crate) const SLOGIN_MC_BUILD_FILE_PATHS: Callee = Callee {
        addr: 0x003f_a818,
        f: ee::s14::SLoginMc_BuildFilePaths,
    };

    pub(crate) const SLOGIN_MC_SET_ICON_SYS: Callee = Callee {
        addr: 0x003f_a240,
        f: ee::s14::SLoginMc_SetIconSys,
    };

    pub(crate) const SLOGIN_STORE_GET_ALIGNMENT: Callee = Callee {
        addr: 0x003f_b858,
        f: ee::s15::SLoginStore_GetAlignment,
    };

    pub(crate) const DNAS_REL_ELF_OPEN: Callee = Callee {
        addr: 0x003f_d558,
        f: ee::s15::DnasRel_ElfOpen,
    };

    pub(crate) const DNAS_REL_FIND_SYMBOL: Callee = Callee {
        addr: 0x003f_d758,
        f: ee::s15::DnasRel_FindSymbol,
    };

    pub(crate) const DNAS_REL_RELOCATE_ALL: Callee = Callee {
        addr: 0x003f_d660,
        f: ee::s15::DnasRel_RelocateAll,
    };

    pub(crate) const FLUSH_CACHE: Callee = Callee {
        addr: 0x0042_4020,
        f: ee::s15::FlushCache,
    };

    pub(crate) const SLOGIN_MC_POLL: Callee = Callee {
        addr: 0x003f_a040,
        f: ee::s14::SLoginMc_Poll,
    };

    pub(crate) const SLOGIN_MC_GET_FILE_STATE: Callee = Callee {
        addr: 0x003f_95e8,
        f: ee::s14::SLoginMc_GetFileState,
    };

    pub(crate) const SLOGIN_STORE_WRITE_RECORD: Callee = Callee {
        addr: 0x003f_bed0,
        f: ee::s15::SLoginStore_WriteRecord,
    };

    pub(crate) const SLOGIN_STORE_READ_RECORD: Callee = Callee {
        addr: 0x003f_bf80,
        f: ee::s15::SLoginStore_ReadRecord,
    };

    pub(crate) const SLOGIN_STORE_VERIFY_RECORD: Callee = Callee {
        addr: 0x003f_bfe0,
        f: ee::s15::SLoginStore_VerifyRecord,
    };

    pub(crate) const SLOGIN_STORE_SEEK_TO_START: Callee = Callee {
        addr: 0x003f_bdb8,
        f: ee::s15::SLoginStore_SeekToStart,
    };

    pub(crate) const SLOGIN_STORE_READ_DIR_HEADER: Callee = Callee {
        addr: 0x003f_bdf0,
        f: ee::s15::SLoginStore_ReadDirHeader,
    };

    pub(crate) const SLOGIN_STORE_LOAD_DIR_ENTRY: Callee = Callee {
        addr: 0x003f_be40,
        f: ee::s15::SLoginStore_LoadDirEntry,
    };

    pub(crate) const SLOGIN_STORE_GET_CIPHER_IFACE: Callee = Callee {
        addr: 0x003f_b800,
        f: ee::s15::SLoginStore_GetCipherIface,
    };

    pub(crate) const SLOGIN_STORE_INIT: Callee = Callee {
        addr: 0x003f_c0e8,
        f: ee::s15::SLoginStore_Init,
    };

    pub(crate) const SLOGIN_STORE_SHUTDOWN: Callee = Callee {
        addr: 0x003f_c260,
        f: ee::s15::SLoginStore_Shutdown,
    };

    pub(crate) const SLOGIN_STORE_WRITE: Callee = Callee {
        addr: 0x003f_c548,
        f: ee::s15::SLoginStore_Write,
    };

    pub(crate) const SLOGIN_STORE_READ: Callee = Callee {
        addr: 0x003f_c708,
        f: ee::s15::SLoginStore_Read,
    };

    pub(crate) const SLOGIN_STORE_CLOSE: Callee = Callee {
        addr: 0x003f_c4e8,
        f: ee::s15::SLoginStore_Close,
    };

    pub(crate) const SLOGIN_STORE_POLL: Callee = Callee {
        addr: 0x003f_ca40,
        f: ee::s15::SLoginStore_Poll,
    };

    pub(crate) const SLOGIN_STORE_GET_INFO: Callee = Callee {
        addr: 0x003f_c990,
        f: ee::s15::SLoginStore_GetInfo,
    };

    pub(crate) const SLOGIN_STORE_GET_BLOCK_SIZE: Callee = Callee {
        addr: 0x003f_cf98,
        f: ee::s15::SLoginStore_GetBlockSize,
    };

    pub(crate) const SLOGIN_STORE_SET_ICON_SYS: Callee = Callee {
        addr: 0x003f_cdc8,
        f: ee::s15::SLoginStore_SetIconSys,
    };

    pub(crate) const SLOGIN_MC_ASCII_TO_SJIS: Callee = Callee {
        addr: 0x003f_a380,
        f: ee::s14::SLoginMc_AsciiToSjis,
    };

    pub(crate) const SLOGIN_MC_VALIDATE_ICON_SYS: Callee = Callee {
        addr: 0x003f_b308,
        f: ee::s15::SLoginMc_ValidateIconSys,
    };

    pub(crate) const SLOGIN_MC_SET_BUSY: Callee = Callee {
        addr: 0x003f_b2f8,
        f: ee::s15::SLoginMc_SetBusy,
    };

    pub(crate) const SLOGIN_MC_IS_AVAILABLE: Callee = Callee {
        addr: 0x003f_b278,
        f: ee::s15::SLoginMc_IsAvailable,
    };

    pub(crate) const SLOGIN_MC_CALC_SAVE_SIZE_IMPL: Callee = Callee {
        addr: 0x003f_b280,
        f: ee::s15::SLoginMc_CalcSaveSizeImpl,
    };

    pub(crate) const DNAS_REL_GET_SECTION_HEADER: Callee = Callee {
        addr: 0x003f_d1f0,
        f: ee::s15::DnasRel_GetSectionHeader,
    };

    pub(crate) const SLOGIN_ON_READ_ACCT_DONE: Callee = Callee {
        addr: 0x003f_7840,
        f: ee::s14::SLogin_OnReadAcctDone,
    };

    pub(crate) const SLOGIN_ON_READ_PASS_DONE: Callee = Callee {
        addr: 0x003f_7890,
        f: ee::s14::SLogin_OnReadPassDone,
    };

    pub(crate) const SLOGIN_ON_WRITE_ACCT_DONE: Callee = Callee {
        addr: 0x003f_78e0,
        f: ee::s14::SLogin_OnWriteAcctDone,
    };

    pub(crate) const SLOGIN_ON_WRITE_PASS_DONE: Callee = Callee {
        addr: 0x003f_7900,
        f: ee::s14::SLogin_OnWritePassDone,
    };

    pub(crate) const SLOGIN_ON_OPEN_DONE: Callee = Callee {
        addr: 0x003f_7820,
        f: ee::s14::SLogin_OnOpenDone,
    };

    pub(crate) const SLOGIN_STORE_CREATE: Callee = Callee {
        addr: 0x003f_c390,
        f: ee::s15::SLoginStore_Create,
    };

    pub(crate) const SLOGIN_STORE_DELETE: Callee = Callee {
        addr: 0x003f_c3c0,
        f: ee::s15::SLoginStore_Delete,
    };

    pub(crate) const SLOGIN_STORE_OPEN: Callee = Callee {
        addr: 0x003f_c4c8,
        f: ee::s15::SLoginStore_Open,
    };
}

/// The shared login's `gp` words and the login-state fields the bodies name.
pub(crate) mod login {
    /// `gp` word: nonzero while the shared login is active.
    pub(crate) const ACTIVE_GP: u32 = 0x1ad0;

    /// `gp` word: the login state block.
    pub(crate) const STATE_GP: u32 = 0x1ad4;

    /// Word at +0xcc: the caller's record buffer (`SLogin_Load`'s argument);
    /// a finished account or password read copies the record there.
    pub(crate) const RECORD: u32 = 0xcc;

    /// Word at +0xd0: the operation in progress (`SLogin_ResetForOp` stores
    /// it; `SLogin_Poll` reports 4 as finished).
    pub(crate) const OPERATION: u32 = 0xd0;

    /// Word at +0xd8: the current step, an index into the step tables (< 9).
    pub(crate) const STEP: u32 = 0xd8;
    /// Word at +0xfc: the callback `SLogin_StepPoll` runs when the card-store
    /// step reports done (cleared after it runs).
    pub(crate) const DONE_CALLBACK: u32 = 0xfc;
    /// Word at +0x100: the callback `SLogin_StepPoll` runs on its other exit
    /// (cleared after it runs).
    pub(crate) const RELEASE_CALLBACK: u32 = 0x100;
    /// Word at +0x140: the card store the record calls go to.
    pub(crate) const STORE: u32 = 0x140;
    /// Word at +0x144: the card-store block of the record being read or written.
    pub(crate) const BLOCK: u32 = 0x144;

    /// Word at +0x148: the error a failed step keeps until the error close
    /// reports it.
    pub(crate) const FINAL_ERROR: u32 = 0x148;
}

/// The memory-card layer's `gp` words, the card-state fields the bodies name
/// and the file names the card steps pass.
pub(crate) mod card {
    /// `gp` word: the card state block.
    pub(crate) const STATE_GP: u32 = 0x1ae8;

    /// `gp` word: the card interface (method table and device mask).
    pub(crate) const IFACE_GP: u32 = 0x1aec;

    /// Word at +4: the phase of the card sequence in progress.
    pub(crate) const PHASE: u32 = 0x04;

    /// Word at +8: the last card error (`SLoginMc_SetError`/`GetError`).
    pub(crate) const ERROR: u32 = 0x08;

    /// Word at +0xc: the open file's handle.
    pub(crate) const FILE_HANDLE: u32 = 0x0c;

    /// Word at +0x10: the selected device.
    pub(crate) const DEVICE: u32 = 0x10;

    /// The two path buffers `SLoginMc_BuildFilePaths` fills; the first is
    /// the save's path passed to the card callbacks.
    pub(crate) const FILE_PATH: u32 = 0x14;

    pub(crate) const DIR_PATH: u32 = 0x34;

    /// The directory name `SLoginMc_BuildDirPath` and `BuildFilePaths` append.
    pub(crate) const DIR_NAME: u32 = 0x54;

    /// Word at +0x6c: passed as `SLoginMc_NextDeviceInMask`'s direction.
    pub(crate) const SCAN_DIRECTION: u32 = 0x6c;

    /// `"view.ico"` and `"icon.sys"`.
    pub(crate) const VIEW_ICON_NAME: u32 = 0x0049_7410;

    pub(crate) const ICON_SYS_NAME: u32 = 0x0049_7420;

    /// Size of an icon.sys file.
    pub(crate) const ICON_SYS_BYTES: u64 = 0x3c4;

    /// The open mode the create steps pass to the card's open callback.
    pub(crate) const OPEN_CREATE_MODE: u64 = 0x203;

    /// Sizes of the interface and state blocks `SLoginMc_Init` allocates
    /// (and `SLoginMc_Shutdown` frees).
    pub(crate) const IFACE_BYTES: u64 = 0x42c;

    pub(crate) const STATE_BYTES: u64 = 0x74;

    /// Bytes a new card request clears from `FILE_HANDLE` on (up to
    /// `SCAN_DIRECTION`).
    pub(crate) const REQUEST_BYTES: u64 = 0x60;

    /// `SLoginMc_StartCreate` refuses names of this many bytes or more.
    pub(crate) const NAME_LIMIT: u64 = 0x18;
}

/// The login store's `gp` word, its state fields and the config words
/// `SLoginStore_Init` reads.
pub(crate) mod store {
    /// `gp` word: the store state block.
    pub(crate) const STATE_GP: u32 = 0x2a30;

    /// Word at +0x60: the directory hash. The first 0x64 bytes of the state
    /// are the directory header `SLoginStore_Write`/`_Read` copy in;
    /// `SLoginStore_Close` copies the hash back and `SLoginStore_GetInfo`
    /// copies the 0x60 bytes before it, then the hash word.
    pub(crate) const DIR_HASH: u32 = 0x60;

    /// The cipher key and its length, and the cipher and hash interfaces.
    pub(crate) const CIPHER_KEY: u32 = 0x64;

    pub(crate) const CIPHER_KEY_LEN: u32 = 0x68;

    pub(crate) const CIPHER_IFACE: u32 = 0x6c;

    pub(crate) const HASH_IFACE: u32 = 0x70;

    /// Word at +0x74: the store operation in progress.
    pub(crate) const OPERATION: u32 = 0x74;

    /// Word at +0x78: the memory-card step of that operation.
    pub(crate) const STEP: u32 = 0x78;

    /// The record payload buffer, the directory buffer and the payload size.
    pub(crate) const RECORD_BUFFER: u32 = 0x7c;

    pub(crate) const DIR_BUFFER: u32 = 0x80;

    pub(crate) const RECORD_SIZE: u32 = 0x84;

    /// The record's CRC and kind words.
    pub(crate) const RECORD_CRC: u32 = 0x88;

    pub(crate) const RECORD_KIND: u32 = 0x8c;

    /// Words at +0x90 / +0x94: directory entries the store holds (from the
    /// config) / entries in use or loaded so far.
    pub(crate) const RECORD_CAPACITY: u32 = 0x90;

    pub(crate) const RECORD_COUNT: u32 = 0x94;

    pub(crate) const DATA_END: u32 = 0x98;

    /// Word at +0x9c: the directory entries (`RECORD_STRIDE` bytes each).
    pub(crate) const RECORDS: u32 = 0x9c;

    pub(crate) const RECORD_STRIDE: u32 = 0x10;

    /// Word at +0xa0: the heap handle from the config. `SLoginStore_Init`
    /// stores it last, after its directory-block `SLoginStore_AllocBlock` has
    /// passed the word; `SLoginStore_Shutdown` passes it to its frees.
    pub(crate) const HEAP: u32 = 0xa0;

    /// Word at +0xa4: `SLoginStore_Read` puts the found record's offset there.
    pub(crate) const SEEK_OFFSET: u32 = 0xa4;

    /// Byte at +0xa8: set when a create (operation 1) succeeds on the card;
    /// the seek step after it then finishes without reading the directory
    /// header.
    pub(crate) const CREATED: u32 = 0xa8;

    /// Size of the store state block `SLoginStore_Init` allocates.
    pub(crate) const STATE_BYTES: u64 = 0xac;

    /// Words of the config `SLoginStore_Init` reads.
    pub(crate) const CONFIG_MAX_RECORDS: u32 = 0x0;

    pub(crate) const CONFIG_DEVICES: u32 = 0x4;

    pub(crate) const CONFIG_MC_FUNCS: u32 = 0x8;

    pub(crate) const CONFIG_KEY: u32 = 0xc;

    pub(crate) const CONFIG_KEY_LEN: u32 = 0x10;

    pub(crate) const CONFIG_CIPHER: u32 = 0x14;

    pub(crate) const CONFIG_HEAP: u32 = 0x18;
}

/// The store configuration template `SLogin_InitStore` copies.
pub(crate) const LOGIN_STORE_TEMPLATE: u32 = 0x0049_7060;

/// The step-function table `SLogin_StepUpdate` indexes by step.
pub(crate) const STEP_TABLE: u32 = 0x0049_7080;

/// The address of the `gp`-relative word at `off` (the `STATE_GP` and
/// `IFACE_GP` words of [`login`], [`card`] and [`store`]).
pub(crate) fn gp_slot(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off)
}

/// Mark the shared login as active and return success.
pub fn slogin_set_active(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 1;
    c.g[V0][0] = 0;
    let active = (c.g[GP][0] as u32).wrapping_add(login::ACTIVE_GP);
    let active_value = c.g[V1][0];
    Addr(active).sb(c, active_value);
    ret(c);
}

/// Clear the shared login's active flag and return success.
pub fn slogin_clear_active(c: &mut Cpu) {
    c.cycles += 3;
    let active = (c.g[GP][0] as u32).wrapping_add(login::ACTIVE_GP);
    Addr(active).sb(c, 0);
    c.g[V0][0] = 0;
    ret(c);
}

/// Allocate and initialize the shared-login state from its startup settings.
pub fn slogin_alloc_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x14c;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 4;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
        call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_6c90)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        let state = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).sw(c, state);
        c.g[A2][0] = 0x14c;
        call_ptr(c, callee::MEMSET.addr, 0x003f_6ca4)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        let original_store = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(login::STORE)).sw(c, original_store);
        c.g[A0][0] = c.g[V1][0];
        c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = c.g[S0][0] | c.g[A0][0];
        c.g[V0][0] &= 7;
        let aligned = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
        if !aligned {
            loop {
                c.cycles += 19;
                let source = c.g[S0][0] as u32;
                let destination = c.g[A0][0] as u32;
                c.ldl(V1, source.wrapping_add(7));
                c.ldr(V1, source);
                c.ldl(A1, source.wrapping_add(15));
                c.ldr(A1, source.wrapping_add(8));
                c.ldl(A2, source.wrapping_add(23));
                c.ldr(A2, source.wrapping_add(16));
                c.ldl(A3, source.wrapping_add(31));
                c.ldr(A3, source.wrapping_add(24));
                c.sdl(V1, destination.wrapping_add(7));
                c.sdr(V1, destination);
                c.sdl(A1, destination.wrapping_add(15));
                c.sdr(A1, destination.wrapping_add(8));
                c.sdl(A2, destination.wrapping_add(23));
                c.sdr(A2, destination.wrapping_add(16));
                c.sdl(A3, destination.wrapping_add(31));
                c.sdr(A3, destination.wrapping_add(24));
                c.g[S0][0] = sx(source.wrapping_add(0x20));
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = sx(destination.wrapping_add(0x20));
                if !more {
                    c.cycles += 2;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_6cc8);
                }
            }
        } else {
            loop {
                c.cycles += 11;
                let source = c.g[S0][0] as u32;
                let destination = c.g[A0][0] as u32;
                c.g[V1][0] = Addr(source).ld(c);
                c.g[A1][0] = Addr(source.wrapping_add(8)).ld(c);
                c.g[A2][0] = Addr(source.wrapping_add(16)).ld(c);
                c.g[A3][0] = Addr(source.wrapping_add(24)).ld(c);
                let v1 = c.g[V1][0];
                let a1 = c.g[A1][0];
                let a2 = c.g[A2][0];
                let a3 = c.g[A3][0];
                Addr(destination).sd(c, v1);
                Addr(destination.wrapping_add(8)).sd(c, a1);
                Addr(destination.wrapping_add(16)).sd(c, a2);
                Addr(destination.wrapping_add(24)).sd(c, a3);
                c.g[S0][0] = sx(source.wrapping_add(0x20));
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = sx(destination.wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_6d1c);
                }
            }
        }
        c.cycles += 19;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A3][0] = 5;
        let trailing = c.g[V0][0];
        Addr(c.g[A0][0] as u32).sw(c, trailing);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[A1][0] = 4;
        c.g[A2][0] = 10;
        c.g[A0][0] = u64::MAX;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        let login = c.g[V1][0] as u32;
        let a0 = c.g[A0][0];
        let a1 = c.g[A1][0];
        let a2 = c.g[A2][0];
        let a3 = c.g[A3][0];
        Addr(login.wrapping_add(0x13c)).sw(c, a0);
        Addr(login.wrapping_add(0x104)).sw(c, a1);
        Addr(login.wrapping_add(login::STEP)).sw(c, a2);
        Addr(login.wrapping_add(0xec)).sw(c, a3);
        Addr(login.wrapping_add(login::OPERATION)).sw(c, a1);
        Addr(login.wrapping_add(login::FINAL_ERROR)).sw(c, 0);
        Addr(login.wrapping_add(0x138)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Release the shared-login state allocation and clear its global pointer.
pub fn slogin_free_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x14c;
        c.g[A3][0] = 4;
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(login::STORE)).lw(c);
        call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_6db4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).sw(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Initialize the shared-login record store from its static configuration.
pub fn slogin_init_store(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = DATA_PAGE;
        c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7060));
        let template = LOGIN_STORE_TEMPLATE;
        c.ldl(A3, template.wrapping_add(7));
        c.ldr(A3, template);
        c.ldl(T0, template.wrapping_add(15));
        c.ldr(T0, template.wrapping_add(8));
        c.ldl(T1, template.wrapping_add(23));
        c.ldr(T1, template.wrapping_add(16));
        let config = frame.at(c, 0);
        c.sdl(A3, config.wrapping_add(7));
        c.sdr(A3, config);
        c.sdl(T0, config.wrapping_add(15));
        c.sdr(T0, config.wrapping_add(8));
        c.sdl(T1, config.wrapping_add(23));
        c.sdr(T1, config.wrapping_add(16));
        c.g[A3][0] = Addr(template.wrapping_add(24)).lb(c);
        c.g[T0][0] = Addr(template.wrapping_add(25)).lb(c);
        let a3 = c.g[A3][0];
        let t0 = c.g[T0][0];
        Addr(config.wrapping_add(24)).sb(c, a3);
        Addr(config.wrapping_add(25)).sb(c, t0);
        c.g[A2][0] = 0x1a;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x138));
        c.g[A0][0] = 2;
        let a0 = c.g[A0][0];
        let v0 = c.g[V0][0];
        let a1 = c.g[A1][0];
        let a2 = c.g[A2][0];
        let stack = c.g[SP][0];
        Addr(frame.at(c, 0x20)).sw(c, a0);
        Addr(frame.at(c, 0x24)).sw(c, v0);
        Addr(frame.at(c, 0x28)).sw(c, a1);
        Addr(frame.at(c, 0x30)).sw(c, a2);
        Addr(frame.at(c, 0x2c)).sw(c, stack);
        call_ptr(c, callee::SLOGIN_STORE_GET_CIPHER_IFACE.addr, 0x003f_6e40)?;
        let cipher_iface = c.g[V0][0];
        Addr(frame.at(c, 0x34)).sw(c, cipher_iface);
        c.g[A0][0] = sx(frame.at(c, 0x20));
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(login::STORE)).lw(c);
        let cache = c.g[V1][0];
        Addr(frame.at(c, 0x38)).sw(c, cache);
        call_ptr(c, callee::SLOGIN_STORE_INIT.addr, 0x003f_6e58)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_6e60)?;
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// Shut down the shared-login record store and translate its final error.
pub fn slogin_shutdown_store(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, callee::SLOGIN_STORE_SHUTDOWN.addr, 0x003f_6e80)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_6e88)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Start a shared-login operation when the store is in its ready state.
pub fn slogin_start_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let login = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[A1][0] = login;
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 4;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((login as u32).wrapping_add(login::OPERATION)).lw(c);
        c.g[S0][0] = 0;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 4;
            callee::SLOGIN_RESET_FOR_OP.call(c, 0x003f_6ec0)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[S0][0] = 0x16;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Poll a shared-login operation, returning its current or terminal status.
pub fn slogin_op_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        let result_address = c.g[A0][0] as u32;
        Addr(result_address).sw(c, 0);
        frame.save_ra(c, 0x10);
        let has_result = c.g[A0][0] != 0;
        Addr(frame.at(c, 0)).sw(c, 0);
        if !has_result {
            c.cycles += 2;
            c.g[V0][0] = 7;
            let error = c.g[V0][0];
            Addr(frame.at(c, 0)).sw(c, error);
        }
        c.cycles += 5;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(login::OPERATION)).lw(c);
        c.g[V0][0] = (c.g[V0][0] < 4) as u64;
        let operation_active = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        if !operation_active {
            c.cycles += 3;
            c.g[V0][0] = 15;
            let error = c.g[V0][0];
            Addr(frame.at(c, 0)).sw(c, error);
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        }
        c.cycles += 2;
        let has_error = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        if !has_error {
            c.cycles += 5;
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(login::STEP)).lw(c);
            c.g[V0][0] = (c.g[V0][0] < 9) as u64;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe8)).lw(c);
                c.cycles += 2;
                let target = c.g[V0][0] as u32;
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, target, 0x003f_6f54)?;
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xdc)).lw(c);
                let target = c.g[V0][0] as u32;
                call_ptr(c, target, 0x003f_6f44)?;
                let result = c.g[V0][0];
                Addr(frame.at(c, 0)).sw(c, result);
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Copy the three step-result doublewords at login +0xec to +0x120 through
/// the `scratch` registers.
pub(crate) fn slogin_copy_step_snapshot(c: &mut Cpu, login: u32, scratch: [usize; 3]) {
    for (index, register) in scratch.into_iter().enumerate() {
        let source = login.wrapping_add(0xec + 8 * index as u32);
        c.ldl(register, source.wrapping_add(7));
        c.ldr(register, source);
    }
    for (index, register) in scratch.into_iter().enumerate() {
        let destination = login.wrapping_add(0x120 + 8 * index as u32);
        c.sdl(register, destination.wrapping_add(7));
        c.sdr(register, destination);
    }
}

/// Poll the shared-login step callbacks and dispatch completion or error handling.
pub fn slogin_step_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        Addr(frame.at(c, 0x0c)).sw(c, 0);
        let has_result = c.g[S0][0] != 0;
        Addr(c.g[S0][0] as u32).sw(c, 0);
        if !has_result {
            c.cycles += 2;
            c.g[V0][0] = 7;
            Addr(frame.at(c, 0x0c)).sw(c, 7);
        }

        c.cycles += 5;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(login::STEP)).lw(c);
        c.g[V1][0] = (c.g[A0][0] < 9) as u64;
        let valid_step = c.g[V1][0] != 0;
        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
        if !valid_step {
            c.cycles += 3;
            c.g[V0][0] = 10;
            let not_terminal = c.g[A0][0] != c.g[V0][0];
            c.g[V0][0] = 13;
            if !not_terminal {
                c.cycles += 1;
                c.g[V0][0] = 12;
            }
            c.cycles += 2;
            let error = c.g[V0][0];
            Addr(frame.at(c, 0x0c)).sw(c, error);
            c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
        }

        c.cycles += 2;
        let mut has_error = c.g[V0][0] != 0;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        if !has_error {
            c.cycles += 5;
            c.g[A1][0] = c.g[V1][0];
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xec)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 6) as u64;
            let valid_phase = c.g[V0][0] != 0;
            c.g[V0][0] = DATA_PAGE;
            if !valid_phase {
                c.cycles += 1;
                c.g[V0][0] = 17;
                let error = c.g[V0][0];
                c.cycles += 1;
                Addr(frame.at(c, 0x0c)).sw(c, error);
                c.cycles += 1;
                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
            } else {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx(STEP_TABLE);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                match target {
                    0x003f_6fec => {
                        c.cycles += 17;
                        let login = c.g[A1][0] as u32;
                        slogin_copy_step_snapshot(c, login, [V0, V1, A0]);
                        c.g[V1][0] = 1;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                        Addr((c.g[V0][0] as u32).wrapping_add(0xec)).sw(c, 0);
                        Addr(c.g[S0][0] as u32).sw(c, 1);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    0x003f_7030 => {
                        c.cycles += 6;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xf0)).lw(c);
                        let target = c.g[V0][0] as u32;
                        call_ptr(c, target, 0x003f_7040)?;
                        let failed = c.g[V0][0] != 0;
                        let result = c.g[V0][0];
                        Addr(frame.at(c, 0x0c)).sw(c, result);
                        if !failed {
                            c.cycles += 18;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                            c.g[A0][0] = 1;
                            let login = c.g[V0][0] as u32;
                            slogin_copy_step_snapshot(c, login, [A3, T0, V1]);
                            c.g[V1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                            Addr((c.g[V1][0] as u32).wrapping_add(0xec)).sw(c, 1);
                            Addr(c.g[S0][0] as u32).sw(c, 1);
                            c.cycles += 1;
                            c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                            c.g[A1][0] = 2;
                            c.g[A0][0] = c.g[SP][0];
                            slogin_step_failure_cleanup(c, frame)?;
                        }
                    }
                    0x003f_70a0 => {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xf4)).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = c.g[SP][0] | 4;
                        call_ptr(c, target, 0x003f_70ac)?;
                        let failed = c.g[V0][0] != 0;
                        let result = c.g[V0][0];
                        Addr(frame.at(c, 0x0c)).sw(c, result);
                        if !failed {
                            c.cycles += 5;
                            c.g[V1][0] = 1;
                            c.g[A0][0] = Addr(frame.at(c, 4)).lw(c);
                            c.g[V0][0] = 2;
                            let incomplete = c.g[A0][0] != c.g[V0][0];
                            Addr(c.g[S0][0] as u32).sw(c, 1);
                            if incomplete {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                            } else {
                                c.cycles += 4;
                                let completed = c.g[A0][0];
                                Addr(c.g[S0][0] as u32).sw(c, completed);
                                callee::SLOGIN_CLEAR_STEP_FUNCS.call(c, 0x003f_70d0)?;
                                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                            c.g[A1][0] = 2;
                            c.g[A0][0] = c.g[SP][0] | 8;
                            slogin_step_failure_cleanup(c, frame)?;
                        }
                    }
                    0x003f_7134 => {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xf8)).lw(c);
                        let target = c.g[V0][0] as u32;
                        call_ptr(c, target, 0x003f_7140)?;
                        let result = c.g[V0][0];
                        Addr(frame.at(c, 0x0c)).sw(c, result);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    0x003f_7148 => {
                        c.cycles += 2;
                        c.g[V0][0] = 16;
                        c.cycles += 1;
                        Addr(frame.at(c, 0x0c)).sw(c, 16);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    0x003f_7150 => {
                        c.cycles += 1;
                        c.g[V0][0] = 17;
                        c.cycles += 1;
                        Addr(frame.at(c, 0x0c)).sw(c, 17);
                        c.cycles += 1;
                        c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                    }
                    target => {
                        c.jump_out(target, 0x003f_6f68, 0x003f_71b4);
                        return Ok(());
                    }
                }
            }
            c.cycles += 2;
            has_error = c.g[V0][0] != 0;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            if !has_error {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[V0][0] = 2;
                let complete = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                if complete {
                    c.cycles += 7;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe0)).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A1][0] = c.g[SP][0] | 0x0c;
                    call_ptr(c, target, 0x003f_7188)?;
                    c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
                }
            }
        }
        if has_error {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xe4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[SP][0] | 0x0c;
            call_ptr(c, target, 0x003f_71a0)?;
            c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The failed-step tail of `SLogin_StepUpdate`: snapshot the step result, call
/// the login's +0xdc callback, then clear the step callbacks.
pub(crate) fn slogin_step_failure_cleanup(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 20;
    let login = c.g[V0][0] as u32;
    slogin_copy_step_snapshot(c, login, [A2, A3, T0]);
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xdc)).lw(c);
    let target = c.g[V0][0] as u32;
    let phase = c.g[A1][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0xec)).sw(c, phase);
    call_ptr(c, target, 0x003f_7124)?;
    callee::SLOGIN_CLEAR_STEP_FUNCS.call(c, 0x003f_712c)?;
    c.g[V0][0] = Addr(frame.at(c, 0x0c)).lw(c);
    Ok(())
}

/// Save a login-step error and switch the sequence to its error step.
pub fn slogin_on_step_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A1][0];
        let step_result = c.g[A0][0] as u32;
        let in_progress = c.g[V0][0];
        Addr(step_result).sw(c, in_progress);
        c.g[A1][0] = 0;
        c.g[A0][0] = 7;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        let state = c.g[V1][0] as u32;
        let error = c.g[V0][0];
        Addr(state.wrapping_add(login::FINAL_ERROR)).sw(c, error);
        let error_address = c.g[A2][0] as u32;
        Addr(error_address).sw(c, 0);
        callee::SLOGIN_SET_STEP.call(c, 0x003f_71e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// End a shared-login sequence after its final step reports an error.
pub fn slogin_on_final_error(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::SLOGIN_END_SEQUENCE),
        0x003f_7208,
    );
}

/// Advance the login sequence after one store step finishes.
pub fn slogin_on_step_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        let result_address = c.g[A2][0] as u32;
        Addr(result_address).sw(c, 1);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(login::STEP)).lw(c);
        c.g[V1][0] = (c.g[A0][0] < 6) as u64;
        c.g[V0][0] = DATA_PAGE;

        let mut set_step = false;
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = 12;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70a0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x003f_7258 => {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    set_step = true;
                }
                0x003f_7260 => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(login::OPERATION)).lw(c);
                    c.g[V1][0] = (c.g[A0][0] < 2) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = 2;
                        set_step = true;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 2;
                        let is_other_operation = c.g[A0][0] != c.g[V0][0];
                        c.g[V0][0] = 12;
                        if !is_other_operation {
                            c.cycles += 2;
                            c.g[A0][0] = 4;
                            set_step = true;
                        }
                    }
                }
                0x003f_7288 => {
                    c.cycles += 2;
                    c.g[A0][0] = 3;
                    set_step = true;
                }
                0x003f_7290 => {
                    c.cycles += 2;
                    c.g[A0][0] = 5;
                    set_step = true;
                }
                0x003f_7298 => {
                    c.cycles += 1;
                    c.g[A0][0] = 6;
                    set_step = true;
                }
                target => {
                    c.jump_out(target, 0x003f_7218, 0x003f_72c4);
                    return Ok(());
                }
            }
        }
        if set_step {
            c.cycles += 4;
            c.g[A1][0] = 1;
            callee::SLOGIN_SET_STEP.call(c, 0x003f_72a4)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            let error_address = c.g[A1][0] as u32;
            let error = c.g[V0][0];
            Addr(error_address).sw(c, error);
            let result_address = c.g[A2][0] as u32;
            Addr(result_address).sw(c, 0);
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Complete the final login step and record a successful result.
pub fn slogin_on_final_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_72dc)?;
        c.g[V1][0] = 2;
        frame.restore_ra(c, 0);
        let result_address = c.g[S0][0] as u32;
        let result_value = c.g[V1][0];
        Addr(result_address).sw(c, result_value);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Forward a failed close result and end the shared-login sequence.
pub fn slogin_on_error_close_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(login::FINAL_ERROR)).lw(c);
        let result_address = c.g[A1][0] as u32;
        let error = c.g[V1][0];
        Addr(result_address).sw(c, error);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        Addr((c.g[V0][0] as u32).wrapping_add(login::FINAL_ERROR)).sw(c, 0);
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_7318)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Begin the next shared-login step when the previous sequence has finished.
pub fn slogin_begin_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = 9;
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(login::STEP)).lw(c);
        let ready = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 12;
        if !ready {
            c.cycles += 1;
            let failure_address = c.g[A1][0] as u32;
            let error = c.g[V0][0];
            Addr(failure_address).sw(c, error);
            c.cycles += 2;
            let result_address = c.g[A0][0] as u32;
            Addr(result_address).sw(c, 0);
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 7;
            c.g[V0][0] = 1;
            let result_address = c.g[A0][0] as u32;
            let in_progress = c.g[V0][0];
            Addr(result_address).sw(c, in_progress);
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(login::OPERATION)).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 3) as u64;
            let below_three = c.g[V0][0] != 0;
            c.g[V0][0] = 3;
            if below_three {
                c.cycles += 2;
                let first_op = c.g[V1][0] == 0;
                c.g[A0][0] = 0;
                if !first_op {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                }
                c.cycles += 4;
                c.g[A1][0] = 1;
                callee::SLOGIN_SET_STEP.call(c, 0x003f_738c)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                let third_op = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 12;
                if third_op {
                    c.cycles += 1;
                    c.g[A0][0] = 8;
                    c.cycles += 4;
                    c.g[A1][0] = 1;
                    callee::SLOGIN_SET_STEP.call(c, 0x003f_738c)?;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                    let failure_address = c.g[A1][0] as u32;
                    let error = c.g[V0][0];
                    Addr(failure_address).sw(c, error);
                    c.cycles += 2;
                    let result_address = c.g[A0][0] as u32;
                    Addr(result_address).sw(c, 0);
                    frame.restore_ra(c, 0);
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// The shared body of the create, open and delete steps: set or clear the done
/// callback, then run one card-store file call and map its result.
pub(crate) fn slogin_file_step(
    c: &mut Cpu,
    callback: Option<u32>,
    store_call: u32,
    return_at: u32,
    map_at: u32,
) {
    guest(c, |c| {
        c.cycles += if callback.is_some() { 17 } else { 15 };
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        if let Some(callback_address) = callback {
            c.g[V1][0] = CODE_PAGE;
            frame.save_ra(c, 0);
            c.g[V1][0] = sx(callback_address);
            c.g[A0][0] = GLOBALS_PAGE;
            let login = c.g[V0][0] as u32;
            let callback_value = c.g[V1][0];
            Addr(login.wrapping_add(login::DONE_CALLBACK)).sw(c, callback_value);
        } else {
            frame.save_ra(c, 0);
            c.g[A0][0] = GLOBALS_PAGE;
        }
        c.g[A1][0] = u64::MAX;
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4bc8));
        let login = c.g[V0][0] as u32;
        Addr(login.wrapping_add(login::RELEASE_CALLBACK)).sw(c, 0);
        if callback.is_none() {
            Addr(login.wrapping_add(login::DONE_CALLBACK)).sw(c, 0);
        }
        call_ptr(c, store_call, return_at)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, map_at)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Start creating the memory-card record for the shared login.
pub fn slogin_step_create(c: &mut Cpu) {
    slogin_file_step(
        c,
        Some(callee::SLOGIN_FETCH_STORE_INFO.addr),
        callee::SLOGIN_STORE_CREATE.addr,
        0x003f_73d8,
        0x003f_73e0,
    );
}

/// Start opening the memory-card record for the shared login.
pub fn slogin_step_open(c: &mut Cpu) {
    slogin_file_step(
        c,
        Some(callee::SLOGIN_ON_OPEN_DONE.addr),
        callee::SLOGIN_STORE_OPEN.addr,
        0x003f_7420,
        0x003f_7428,
    );
}

/// Start deleting the shared-login memory-card record.
pub fn slogin_step_delete(c: &mut Cpu) {
    slogin_file_step(
        c,
        None,
        callee::SLOGIN_STORE_DELETE.addr,
        0x003f_7460,
        0x003f_7468,
    );
}

/// The shared body of the account and password write steps.
pub(crate) fn slogin_write_record_step(
    c: &mut Cpu,
    release: u32,
    done_callback: u32,
    data_offset: u32,
    tag: u32,
    first_return: u32,
    first_map_return: u32,
    write_return: u32,
    final_map_return: u32,
) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
        frame.save_ra(c, 0);
        c.g[A2][0] = CODE_PAGE;
        c.g[A2][0] = sx(release);
        c.g[V1][0] = CODE_PAGE;
        c.g[V1][0] = sx(done_callback);
        let login = c.g[V0][0] as u32;
        let release_value = c.g[A2][0];
        Addr(login.wrapping_add(login::RELEASE_CALLBACK)).sw(c, release_value);
        c.g[A1][0] = Addr(login.wrapping_add(login::STORE)).lw(c);
        c.g[A0][0] = sx(login.wrapping_add(login::BLOCK));
        let done_value = c.g[V1][0];
        Addr(login.wrapping_add(login::DONE_CALLBACK)).sw(c, done_value);
        c.g[A2][0] = 0x11;
        c.g[A3][0] = 0;
        call_ptr(c, callee::SLOGIN_STORE_ALLOC_BLOCK.addr, first_return)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, first_map_return)?;
        let failed = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !failed {
            c.cycles += 23;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[A1][0] = (tag & 0xffff_0000) as u64;
            c.g[A1][0] |= (tag & 0xffff) as u64;
            c.g[A2][0] = 0;
            let login = c.g[V1][0] as u32;
            c.g[V0][0] = Addr(login.wrapping_add(login::BLOCK)).lw(c);
            c.g[T0][0] = 0x11;
            c.ldl(A0, login.wrapping_add(data_offset + 7));
            c.ldr(A0, login.wrapping_add(data_offset));
            c.ldl(A3, login.wrapping_add(data_offset + 15));
            c.ldr(A3, login.wrapping_add(data_offset + 8));
            c.g[T1][0] = Addr(login.wrapping_add(data_offset + 16)).lb(c);
            let block = c.g[V0][0] as u32;
            c.sdl(A0, block.wrapping_add(7));
            c.sdr(A0, block);
            c.sdl(A3, block.wrapping_add(15));
            c.sdr(A3, block.wrapping_add(8));
            let trailing_byte = c.g[T1][0];
            Addr(block.wrapping_add(16)).sb(c, trailing_byte);
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[A3][0] = Addr((c.g[A0][0] as u32).wrapping_add(login::BLOCK)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
            call_ptr(c, callee::SLOGIN_STORE_WRITE.addr, write_return)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_MAP_STORE_ERROR.call(c, final_map_return)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Write the shared-login account record to the memory card.
pub fn slogin_step_write_acct(c: &mut Cpu) {
    slogin_write_record_step(
        c,
        callee::SLOGIN_FREE_ACCT_RECORD.addr,
        callee::SLOGIN_ON_WRITE_ACCT_DONE.addr,
        0xa8,
        0x4143_4354,
        0x003f_74b0,
        0x003f_74b8,
        0x003f_7510,
        0x003f_7518,
    );
}

/// Write the shared-login password record to the memory card.
pub fn slogin_step_write_pass(c: &mut Cpu) {
    slogin_write_record_step(
        c,
        callee::SLOGIN_FREE_PASS_RECORD.addr,
        callee::SLOGIN_ON_WRITE_PASS_DONE.addr,
        0xb9,
        0x5041_5353,
        0x003f_7560,
        0x003f_7568,
        0x003f_75c0,
        0x003f_75c8,
    );
}

/// The shared body of the account and password read steps.
pub(crate) fn slogin_step_read_record(
    c: &mut Cpu,
    release: u32,
    on_done: u32,
    tag: u32,
    returns: [u32; 4],
) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(release);
        c.g[V1][0] = sx(on_done);
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(login::RELEASE_CALLBACK),
            c.g[A2][0],
        );
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::STORE));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(login::BLOCK));
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(login::DONE_CALLBACK),
            c.g[V1][0],
        );
        c.g[A2][0] = 0x11;
        c.g[A3][0] = 0;
        call_ptr(c, callee::SLOGIN_STORE_ALLOC_BLOCK.addr, returns[0])?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, returns[1])?;
        let mapped_error = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !mapped_error {
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
            c.g[A1][0] = ((tag >> 16) as u64) << 16;
            c.g[A1][0] |= (tag & 0xffff) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(login::BLOCK));
            c.g[T0][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
            call_ptr(c, callee::SLOGIN_STORE_READ.addr, returns[2])?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_MAP_STORE_ERROR.call(c, returns[3])?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// Start reading the shared login account record from the card store.
pub fn slogin_step_read_acct(c: &mut Cpu) {
    slogin_step_read_record(
        c,
        callee::SLOGIN_FREE_ACCT_RECORD.addr,
        callee::SLOGIN_ON_READ_ACCT_DONE.addr,
        0x4143_4354,
        [0x003f_7610, 0x003f_7618, 0x003f_7640, 0x003f_7648],
    );
}

/// Start reading the shared login password record from the card store.
pub fn slogin_step_read_pass(c: &mut Cpu) {
    slogin_step_read_record(
        c,
        callee::SLOGIN_FREE_PASS_RECORD.addr,
        callee::SLOGIN_ON_READ_PASS_DONE.addr,
        0x5041_5353,
        [0x003f_7690, 0x003f_7698, 0x003f_76c0, 0x003f_76c8],
    );
}

/// Close the shared login's current card-store file and map its result.
pub fn slogin_step_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
        c.sw((c.g[V0][0] as u32).wrapping_add(login::DONE_CALLBACK), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(login::RELEASE_CALLBACK), 0);
        call_ptr(c, callee::SLOGIN_STORE_CLOSE.addr, 0x003f_76f4)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_76fc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The shared epilogue of `SLogin_StepPoll`: return `a0`.
pub(crate) fn slogin_poll_finish(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    c.g[V0][0] = c.g[A0][0];
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// Poll the current card-store step and run its completion or error callback.
pub fn slogin_step_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 7;
        frame.save_ra(c, 0x10);
        if c.g[S1][0] != 0 {
            c.g[A0][0] = 0;
        }
        c.sw(frame.at(c, 0), 0);

        if c.g[A0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[SP][0] | 4;
            call_ptr(c, callee::SLOGIN_STORE_POLL.addr, 0x003f_7738)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_7740)?;
            c.g[A0][0] = c.g[V0][0];
            let mapped_error = c.g[A0][0] != 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
            if !mapped_error {
                c.cycles += 4;
                c.g[S0][0] = c.lw(frame.at(c, 0));
                c.g[V0][0] = 1;
                c.sw(c.g[S1][0] as u32, 0);
                if c.g[S0][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                let no_result = c.g[S0][0] == 0;
                c.g[V0][0] = 2;
                if no_result {
                    c.cycles += 2;
                    c.g[A0][0] = 0x17;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                let finished = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
                if !finished {
                    c.cycles += 2;
                    c.g[A0][0] = 0x12;
                    c.cycles += 2;
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }

                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::DONE_CALLBACK));
                if c.g[V0][0] != 0 {
                    c.cycles += 5;
                    let callback = c.g[V0][0] as u32;
                    call_ptr(c, callback, 0x003f_7790)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
                    c.g[A0][0] = c.g[V0][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(login::DONE_CALLBACK), 0);
                }
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    slogin_poll_finish(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                slogin_poll_finish(c, frame);
                return Ok(());
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::RELEASE_CALLBACK));
        frame.restore(c, S0, 0x30);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            let callback = c.g[V0][0] as u32;
            call_ptr(c, callback, 0x003f_77cc)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(login::RELEASE_CALLBACK), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        slogin_poll_finish(c, frame);
        Ok(())
    });
}

/// `int SLogin_FetchStoreInfo(void)` (0x3f77f8): `SLoginStore_GetInfo` on the
/// store header at login+0x44; returns its result through
/// `SLogin_MapStoreError`.
pub fn slogin_fetch_store_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44));
        call_ptr(c, callee::SLOGIN_STORE_GET_INFO.addr, 0x003f_780c)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_7814)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Continue the login sequence after the card-store file opens.
pub fn slogin_on_open_done(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::SLOGIN_FETCH_STORE_INFO),
        0x003f_7830,
    );
}

/// Copy the account record into the login state, then release its card-store block.
pub fn slogin_on_read_acct_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::BLOCK));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::RECORD));
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.ldl(A2, (c.g[A1][0] as u32).wrapping_add(15));
        c.ldr(A2, (c.g[A1][0] as u32).wrapping_add(8));
        c.g[A3][0] = c.lb((c.g[A1][0] as u32).wrapping_add(16));
        c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(7));
        c.sdr(V0, c.g[V1][0] as u32);
        c.sdl(A2, (c.g[V1][0] as u32).wrapping_add(15));
        c.sdr(A2, (c.g[V1][0] as u32).wrapping_add(8));
        c.sb((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
        callee::SLOGIN_FREE_ACCT_RECORD.call(c, 0x003f_7884)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Copy the password record into the login state, then release its card-store block.
pub fn slogin_on_read_pass_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::BLOCK));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::RECORD));
        c.ldl(V0, (c.g[A1][0] as u32).wrapping_add(7));
        c.ldr(V0, c.g[A1][0] as u32);
        c.ldl(A2, (c.g[A1][0] as u32).wrapping_add(15));
        c.ldr(A2, (c.g[A1][0] as u32).wrapping_add(8));
        c.g[A3][0] = c.lb((c.g[A1][0] as u32).wrapping_add(16));
        c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(24));
        c.sdr(V0, (c.g[V1][0] as u32).wrapping_add(17));
        c.sdl(A2, (c.g[V1][0] as u32).wrapping_add(32));
        c.sdr(A2, (c.g[V1][0] as u32).wrapping_add(25));
        c.sb((c.g[V1][0] as u32).wrapping_add(33), c.g[A3][0]);
        c.g[A0][0] = 0;
        callee::SLOGIN_FREE_PASS_RECORD.call(c, 0x003f_78d4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Release the account record after a completed card-store write.
pub fn slogin_on_write_acct_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        callee::SLOGIN_FREE_ACCT_RECORD.call(c, 0x003f_78f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Release the password record after a completed card-store write.
pub fn slogin_on_write_pass_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        callee::SLOGIN_FREE_PASS_RECORD.call(c, 0x003f_7910)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Free the login's card-store block and map the store result (both record kinds).
pub(crate) fn slogin_free_record(c: &mut Cpu, first_return: u32, second_return: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x11;
        c.g[A3][0] = 0;
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::STORE));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::BLOCK));
        call_ptr(c, callee::SLOGIN_STORE_FREE_BLOCK.addr, first_return)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_MAP_STORE_ERROR.call(c, second_return)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Free the account card-store block and map the store's return code.
pub fn slogin_free_acct_record(c: &mut Cpu) {
    slogin_free_record(c, 0x003f_7940, 0x003f_7948);
}

/// Free the password card-store block and map the store's return code.
pub fn slogin_free_pass_record(c: &mut Cpu) {
    slogin_free_record(c, 0x003f_7978, 0x003f_7980);
}

/// Finish the active login operation and end its step sequence.
pub fn slogin_finish_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[V1][0] = 4;
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(login::OPERATION),
            c.g[V1][0],
        );
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xd4), 0);
        callee::SLOGIN_END_SEQUENCE.call(c, 0x003f_7a50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Select a shared login step, preserving the previous step when requested.
pub fn slogin_set_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        frame.save_ra(c, 0);
        c.g[A1][0] &= 0xff;
        c.g[A2][0] = 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(login::STEP), c.g[A0][0]);
        if c.g[A1][0] != 0 {
            c.cycles += 22;
            c.ldl(V1, (c.g[V0][0] as u32).wrapping_add(0xdf));
            c.ldr(V1, (c.g[V0][0] as u32).wrapping_add(0xd8));
            c.ldl(A0, (c.g[V0][0] as u32).wrapping_add(0xe7));
            c.ldr(A0, (c.g[V0][0] as u32).wrapping_add(0xe0));
            c.ldl(A1, (c.g[V0][0] as u32).wrapping_add(0xef));
            c.ldr(A1, (c.g[V0][0] as u32).wrapping_add(0xe8));
            c.ldl(A3, (c.g[V0][0] as u32).wrapping_add(0xf7));
            c.ldr(A3, (c.g[V0][0] as u32).wrapping_add(0xf0));
            c.sdl(V1, (c.g[V0][0] as u32).wrapping_add(0x113));
            c.sdr(V1, (c.g[V0][0] as u32).wrapping_add(0x10c));
            c.sdl(A0, (c.g[V0][0] as u32).wrapping_add(0x11b));
            c.sdr(A0, (c.g[V0][0] as u32).wrapping_add(0x114));
            c.sdl(A1, (c.g[V0][0] as u32).wrapping_add(0x123));
            c.sdr(A1, (c.g[V0][0] as u32).wrapping_add(0x11c));
            c.sdl(A3, (c.g[V0][0] as u32).wrapping_add(0x12b));
            c.sdr(A3, (c.g[V0][0] as u32).wrapping_add(0x124));
            c.ldl(V1, (c.g[V0][0] as u32).wrapping_add(0xff));
            c.ldr(V1, (c.g[V0][0] as u32).wrapping_add(0xf8));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::RELEASE_CALLBACK));
            c.sdl(V1, (c.g[V0][0] as u32).wrapping_add(0x133));
            c.sdr(V1, (c.g[V0][0] as u32).wrapping_add(0x12c));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[A0][0]);
        }

        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(login::STEP));
        c.g[V0][0] = (c.g[V0][0] < 9) as u64;
        let valid_step = c.g[V0][0] != 0;
        c.g[V0][0] = CODE_PAGE;
        if valid_step {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f68));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xdc), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[A2][0] = 13;
        }

        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        if c.g[A2][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(login::STEP));
            c.g[V0][0] = (c.g[A0][0] < 9) as u64;
            c.g[A1][0] = c.g[V1][0];
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[V0][0] = DATA_PAGE;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70c0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let target = c.g[A0][0] as u32;
                let callback = match target {
                    0x003f_7b2c => 0x003f_7218,
                    0x003f_7b3c => 0x003f_72c8,
                    0x003f_7b4c => 0x003f_72f8,
                    0x003f_7b5c => {
                        c.cycles += 1;
                        c.g[A2][0] = 13;
                        0
                    }
                    _ => {
                        c.jump_out(target, 0x003f_7a60, 0x003f_7bcc);
                        return Ok(());
                    }
                };
                if callback != 0 {
                    c.cycles += 4;
                    c.g[V0][0] = CODE_PAGE;
                    c.g[V0][0] = sx(callback);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                }
            } else {
                c.cycles += 1;
                c.g[A2][0] = 13;
            }

            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
            if c.g[A2][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(login::STEP));
                c.g[V0][0] = (c.g[A0][0] < 9) as u64;
                c.g[A1][0] = c.g[V1][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 7;
                    c.g[V0][0] = DATA_PAGE;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70f0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let target = c.g[A0][0] as u32;
                    match target {
                        0x003f_7b94 => {
                            c.cycles += 3;
                            c.g[V0][0] = sx(0x003f_71b8);
                        }
                        0x003f_7ba0 => {
                            c.cycles += 3;
                            c.g[V0][0] = sx(0x003f_71f8);
                        }
                        0x003f_7bac => {
                            c.cycles += 2;
                            c.g[V0][0] = sx(0x003f_72f8);
                        }
                        _ => {
                            c.jump_out(target, 0x003f_7a60, 0x003f_7bcc);
                            return Ok(());
                        }
                    }
                    c.cycles += 1;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0xe4), c.g[V0][0]);
                }
            }
        }

        c.cycles += 5;
        callee::SLOGIN_INIT_STEP_FUNCS.call(c, 0x003f_7bc0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Mark the login step sequence complete and clear its callbacks.
pub fn slogin_end_sequence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = 10;
        c.sw((c.g[V0][0] as u32).wrapping_add(login::STEP), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), 0);
        callee::SLOGIN_CLEAR_STEP_FUNCS.call(c, 0x003f_7bf4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Install the callbacks for the selected login card-store step.
pub fn slogin_init_step_funcs(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
    c.g[V1][0] = 4;
    c.g[A1][0] = 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[V1][0]);
    let login = c.g[V0][0] as u32;
    slogin_copy_step_snapshot(c, login, [A0, A2, A3]);
    c.g[V1][0] = c.g[A1][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    loop {
        c.cycles += 7;
        c.g[V0][0] = (c.g[V1][0] < 3) as u64;
        let again = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003f_7c48);
            }
            continue;
        }
        break;
    }

    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(login::STEP));
    c.g[V1][0] = (c.g[A0][0] < 9) as u64;
    let valid_step = c.g[V1][0] != 0;
    c.g[V0][0] = DATA_PAGE;
    if valid_step {
        c.cycles += 6;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7120));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        let step_callback = match target {
            0x003f_7c90 => 0x003f_73a8,
            0x003f_7ca4 => 0x003f_73f0,
            0x003f_7cb8 => 0x003f_7478,
            0x003f_7ccc => 0x003f_7528,
            0x003f_7ce0 => 0x003f_75d8,
            0x003f_7cf4 => 0x003f_7658,
            0x003f_7d08 => 0x003f_76d8,
            0x003f_7d1c => 0x003f_7438,
            _ => {
                c.jump_out(target, 0x003f_7c00, 0x003f_7d98);
                return;
            }
        };
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        c.g[V0][0] = CODE_PAGE;
        c.g[V0][0] = sx(step_callback);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xf0), c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.g[A1][0] = 13;
    }

    c.cycles += 2;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
    if c.g[A1][0] == 0 {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(login::STEP));
        c.g[V0][0] = (c.g[V0][0] < 9) as u64;
        let valid_poll = c.g[V0][0] != 0;
        c.g[V0][0] = CODE_PAGE;
        if valid_poll {
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7708));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xf4), c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[A1][0] = 13;
        }
        c.cycles += 2;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        if c.g[A1][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(login::STEP));
            c.g[V0][0] = (c.g[V0][0] < 9) as u64;
            let valid_nop = c.g[V0][0] != 0;
            c.g[V0][0] = CODE_PAGE;
            if valid_nop {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x77f0));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
            } else {
                c.cycles += 1;
                c.g[A1][0] = 13;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        }
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[A1][0];
    c.sw((c.g[V1][0] as u32).wrapping_add(login::RELEASE_CALLBACK), 0);
    c.sw((c.g[V1][0] as u32).wrapping_add(login::DONE_CALLBACK), 0);
    ret(c);
}

/// Reset the shared login state for a requested operation and clear its pending callbacks.
pub fn slogin_reset_for_op(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
    c.g[V0][0] = (c.g[A0][0] < 4) as u64;
    c.g[A2][0] = 0;
    c.sw(
        (c.g[V1][0] as u32).wrapping_add(login::OPERATION),
        c.g[A0][0],
    );
    c.sw((c.g[V1][0] as u32).wrapping_add(0x104), c.g[A0][0]);
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = sx(0x003f_6ee0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0xd4), c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.g[A2][0] = 15;
    }

    c.cycles += 13;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
    c.g[V1][0] = sx(0x003f_7328);
    c.g[A0][0] = 9;
    c.g[A1][0] = 4;
    c.sw((c.g[V0][0] as u32).wrapping_add(login::STEP), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[A1][0]);
    c.g[A0][0] = 0;
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[V1][0]);
    for off in [0xdc, 0xe0, 0xe4] {
        c.sw((c.g[V0][0] as u32).wrapping_add(off), 0);
    }
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    loop {
        c.cycles += 7;
        c.g[V0][0] = (c.g[A0][0] < 3) as u64;
        let again = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003f_79f0);
            }
            continue;
        }
        c.cycles += 8;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        c.g[V0][0] = c.g[A2][0];
        for off in [0x100, 0xf0, 0xf4, 0xf8, 0xfc] {
            c.sw((c.g[V1][0] as u32).wrapping_add(off), 0);
        }
        ret(c);
        return;
    }
}

/// Clear the shared login's step callbacks and pending completion handlers.
pub fn slogin_clear_step_funcs(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
    c.g[V0][0] = 5;
    c.g[A0][0] = 0;
    c.sw((c.g[V1][0] as u32).wrapping_add(0xec), c.g[V0][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf0));
    loop {
        c.cycles += 7;
        c.sw(c.g[V1][0] as u32, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[V0][0] = (c.g[A0][0] < 3) as u64;
        let again = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        if again {
            if event_due(c) {
                deliver_event_here(c, 0x003f_7db0);
            }
            continue;
        }
        c.cycles += 5;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(login::STATE_GP));
        c.g[V0][0] = 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(login::RELEASE_CALLBACK), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(login::DONE_CALLBACK), 0);
        ret(c);
        return;
    }
}

/// `int SLogin_VerifyIconChecksum(const void* icon)` (0x3f7de0): CRC-32 the
/// first 0x296e bytes of `icon` through the checksum interface (begin,
/// update, final) and compare the digest word with 0x0836c71d. Returns 0 on
/// a match, 0x15 on a mismatch and 0x18 when an interface call fails.
pub fn slogin_verify_icon_checksum(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S2][0] = 0;
        call_ptr(c, callee::CRC32_GET_INTERFACE.addr, 0x003f_7e00)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        let first = c.g[V0][0] as u32;
        call_ptr(c, first, 0x003f_7e10)?;
        c.g[A2][0] = c.g[V0][0];
        let first_failed = c.g[A2][0] != 0;
        c.g[V0][0] = 0x18;
        if !first_failed {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x296e;
            let second = c.g[V0][0] as u32;
            call_ptr(c, second, 0x003f_7e2c)?;
            c.g[A2][0] = c.g[V0][0];
            let second_failed = c.g[A2][0] != 0;
            c.g[V0][0] = 0x18;
            if !second_failed {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                let third = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, third, 0x003f_7e44)?;
                c.g[A2][0] = c.g[V0][0];
                let third_failed = c.g[A2][0] != 0;
                c.g[V0][0] = 0x18;
                if !third_failed {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = 0x0836_c71d;
                    c.g[S2][0] = 0x15;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(4));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] ^= c.g[A0][0];
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = 0;
                    }
                    c.g[V0][0] = 0x18;
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        if c.g[A2][0] == 0 {
            c.g[V0][0] = c.g[S2][0];
        }
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// Copy a bounded, zero-terminated login string, returning the destination pointer.
pub fn slogin_str_copy_n(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.cycles += 2;
                c.g[V1][0] = c.g[A0][0];
                if c.g[A1][0] != c.g[A0][0] {
                    loop {
                        c.cycles += 3;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                        if c.g[A2][0] == 0 {
                            c.cycles += 2;
                            c.sb(c.g[V1][0] as u32, 0);
                            break;
                        }
                        c.g[V0][0] = c.lbu(c.g[A1][0] as u32);
                        c.cycles += 5;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                        c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        let again = c.g[V0][0] != 0;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003f_7eb0);
                        }
                    }
                }
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `int SLogin_Init(void* arg)` (0x3f7ee0): mark the shared login active,
/// allocate its state (`SLogin_AllocState(arg)`), then start the record
/// store; returns `SLogin_InitStore`'s result.
pub fn slogin_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_SET_ACTIVE.call(c, 0x003f_7ef4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::SLOGIN_ALLOC_STATE.call(c, 0x003f_7efc)?;
        callee::SLOGIN_INIT_STORE.call(c, 0x003f_7f04)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Create(const void* icon, const char* account_and_password)`
/// (0x3f7f48): copy login fields when needed, validate and stage the save
/// icon, then start creating the account record.
pub fn slogin_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        let has_icon = c.g[S2][0] != 0;
        frame.save_ra(c, 0);
        let copy_fields = if !has_icon {
            c.cycles += 2;
            let has_fields = c.g[S0][0] != 0;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            if has_fields {
                true
            } else {
                c.cycles += 2;
                c.g[S1][0] = 7;
                false
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            true
        };
        if copy_fields {
            c.cycles += 9;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa8));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_7f90)?;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb9));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_7fa4)?;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        if c.g[S1][0] == 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            callee::SLOGIN_VERIFY_ICON_CHECKSUM.call(c, 0x003f_7fb4)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
            if c.g[S1][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                callee::SLOGIN_SETUP_ICON_SYS.call(c, 0x003f_7fc8)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.g[S1][0];
                if c.g[S1][0] == 0 {
                    c.cycles += 4;
                    c.g[A0][0] = 0;
                    callee::SLOGIN_START_OP.call(c, 0x003f_7fdc)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V0][0] = c.g[S1][0];
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Poll(int* out_state)` (0x3f80e0): poll the current login
/// operation, finish it when its step reports completion, and return its error.
pub fn slogin_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = 0;
        let has_output = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        let mut outer_exit = false;
        if !has_output {
            c.cycles += 2;
            c.g[S2][0] = 7;
        } else {
            c.cycles += 6;
            c.sw(c.g[S0][0] as u32, 0);
            c.g[V1][0] = 4;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(login::OPERATION)).lw(c);
            let finished = c.g[V0][0] == c.g[V1][0];
            c.g[V0][0] = c.g[S2][0];
            if finished {
                outer_exit = true;
            } else {
                c.cycles += 6;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xd4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, target, 0x003f_812c)?;
                c.g[S2][0] = c.g[V0][0];
                let step_error = c.g[S2][0] != 0;
                c.g[V0][0] = 2;
                if step_error {
                    c.cycles += 2;
                    callee::SLOGIN_FINISH_OP.call(c, 0x003f_815c)?;
                } else {
                    c.cycles += 3;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).lw(c);
                    let step_done = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if step_done {
                        c.cycles += 4;
                        callee::SLOGIN_FINISH_OP.call(c, 0x003f_814c)?;
                        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    } else {
                        outer_exit = true;
                    }
                }
            }
        }
        if !outer_exit {
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
    });
}

/// `void SLogin_Shutdown()` (0x3f7f18): close the shared login store,
/// release its state, and clear the active login flag.
pub fn slogin_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_SHUTDOWN_STORE.call(c, 0x003f_7f28)?;
        callee::SLOGIN_FREE_STATE.call(c, 0x003f_7f30)?;
        callee::SLOGIN_CLEAR_ACTIVE.call(c, 0x003f_7f38)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Update(const char* account_and_password)` (0x3f8048):
/// copy the two fixed length login fields, then begin a store update.
pub fn slogin_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let has_fields = c.g[S0][0] != 0;
        c.g[S1][0] = 0;
        if has_fields {
            c.cycles += 10;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa8));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_8080)?;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
            c.g[A2][0] = 0x11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb9));
            callee::SLOGIN_STR_COPY_N.call(c, 0x003f_8094)?;
        } else {
            c.cycles += 2;
            c.g[S1][0] = 7;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[S1][0];
        if c.g[S1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = 1;
            callee::SLOGIN_START_OP.call(c, 0x003f_80a4)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Load(void* account)` (0x3f8000): save the account record
/// pointer and begin the login store's load operation.
pub fn slogin_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0;
        let has_account = c.g[A0][0] != 0;
        frame.save_ra(c, 0);
        if has_account {
            c.cycles += 2;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(login::STATE_GP)).lw(c);
            c.sw((c.g[V0][0] as u32).wrapping_add(login::RECORD), c.g[A0][0]);
        } else {
            c.cycles += 2;
            c.g[V1][0] = 7;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = 2;
            callee::SLOGIN_START_OP.call(c, 0x003f_8030)?;
            c.g[V1][0] = c.g[V0][0];
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_Delete()` (0x3f80c0): start deletion of the saved account
/// record in the shared login store.
pub fn slogin_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 3;
        callee::SLOGIN_START_OP.call(c, 0x003f_80d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_MapStoreError(int store_error)` (0x3f8178): translate a
/// shared login store error into the code returned to the login caller.
pub fn slogin_map_store_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 0x73) as u64;
    let outside_table = c.g[V0][0] == 0;
    c.g[V0][0] = DATA_PAGE;
    if outside_table {
        c.cycles += 2;
        c.g[V0][0] = 0x18;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7150));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let target = c.g[A0][0] as u32;
    let result = match target {
        0x003f_819c => 0,
        0x003f_81a4 => 1,
        0x003f_81ac => 2,
        0x003f_81b4 => 3,
        0x003f_81bc => 4,
        0x003f_81c4 => 5,
        0x003f_81cc => 6,
        0x003f_81d4 => 0x13,
        0x003f_81dc => 0x14,
        0x003f_81e4 => 7,
        0x003f_81ec => 0x0b,
        0x003f_81f4 => 0x18,
        _ => {
            c.jump_out(target, 0x003f_8178, 0x003f_81fc);
            return;
        }
    };
    c.cycles += 2;
    c.g[V0][0] = result;
    ret(c);
}

/// `int SLogin_GetRecordAllocSize(int* out)` (0x3f8230): ask the shared
/// login store for its record block size, then return success.
pub fn slogin_get_record_alloc_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x22;
        call_ptr(c, callee::SLOGIN_STORE_GET_BLOCK_SIZE.addr, 0x003f_8244)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_FirstDeviceInMask(unsigned mask)` (0x3f8550): return the
/// first of the eight memory card devices selected by the low mask byte.
pub fn slogin_mc_first_device_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] &= 0xffff;
    c.g[V0][0] = c.g[A0][0] & 1;
    c.g[V1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        loop {
            c.cycles += 3;
            let in_range = (c.g[V1][0] as i64) < 8;
            c.g[V0][0] = in_range as u64;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            if !in_range {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x003f_8568);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `int SLoginMc_LastDeviceInMask(unsigned mask)` (0x3f8588): return the
/// last selected memory card device among the eight slots.
pub fn slogin_mc_last_device_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A0][0] &= 0xffff;
    c.g[V0][0] = c.g[A0][0] & 0x80;
    c.g[V1][0] = 7;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[A1][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        loop {
            c.cycles += 2;
            let below_zero = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = sx((c.g[A1][0] as u32) << ((c.g[V1][0] as u32) & 31));
            if below_zero {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            if event_due(c) {
                deliver_event_here(c, 0x003f_85a0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    ret(c);
}

/// `int SLoginMc_NextDeviceInMask(unsigned mask, int direction)` (0x3f85c0):
/// scan from the current card device to the next selected device in the
/// requested direction; return -1 at either end or 18 for an invalid mode.
pub fn slogin_mc_next_device_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
    c.g[A0][0] &= 0xffff;
    c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
    if c.g[A1][0] == 0 {
        c.cycles += 6;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        c.g[V1][0] = ((c.g[A2][0] as i64) < 8) as u64;
        c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] &= 1;
            let selected = c.g[V0][0] != 0;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
            if !selected {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 16);
                c.g[V0][0] = 0x1_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x1_0000;
                c.g[V0][0] = c.g[V1][0];
                loop {
                    c.cycles += 4;
                    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                    let exhausted = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if exhausted {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
                    c.g[V0][0] &= 1;
                    let absent = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0];
                    if !absent {
                        c.cycles += 1;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_8608);
                    }
                }
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 8) as u64;
        }
        c.cycles += 3;
        c.g[A0][0] = u64::MAX;
        if c.g[V0][0] != 0 {
            c.g[A0][0] = c.g[A2][0];
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let bad_mode = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        if bad_mode {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
            ret(c);
            return;
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
        let below_zero = (c.g[A2][0] as i64) < 0;
        c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
        if !below_zero {
            c.cycles += 3;
            c.g[V0][0] &= 1;
            if c.g[V0][0] == 0 {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 16);
                c.g[V0][0] = 0xffff_ffff_ffff_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0xffff_ffff_ffff_0000;
                c.g[V0][0] = c.g[V1][0];
                loop {
                    c.cycles += 3;
                    c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    let exhausted = (c.g[A2][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    if exhausted {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] =
                        sx(((c.g[A0][0] as u32) as i32 >> ((c.g[A2][0] as u32) & 31)) as u32);
                    c.g[V0][0] &= 1;
                    let absent = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[V1][0];
                    if !absent {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_8678);
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[A2][0];
            } else {
                c.g[A0][0] = c.g[A2][0];
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.g[A2][0];
        }
        c.cycles += 4;
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `int SLoginMc_CountDevicesInMask(unsigned mask)` (0x3f86b8): count the
/// enabled devices in the low byte of a memory card device mask.
pub fn slogin_mc_count_devices_in_mask(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = 0;
    c.g[A3][0] = c.g[A0][0] & 0xffff;
    c.g[A1][0] = 0;
    c.g[T0][0] = 1;
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[T0][0] as u32) << ((c.g[A1][0] as u32) & 31));
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.g[V0][0] &= c.g[A3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V1][0] = ((c.g[A1][0] as i64) < 8) as u64;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = c.g[A0][0];
        }
        if c.g[V1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0];
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003f_86c8);
        }
    }
}

/// `void SLoginMc_SetError(int error)` (0x3f86f0): store a card error only
/// when the card state is outside the three reserved states 11 through 13.
pub fn slogin_mc_set_error(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(11));
    c.g[V0][0] = (c.g[V0][0] < 3) as u64;
    if c.g[V0][0] == 0 {
        let error = c.g[A0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(card::ERROR)).sw(c, error);
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_GetError()` (0x3f8710): read the last memory card error.
pub fn slogin_mc_get_error(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(card::ERROR)).lw(c);
    ret(c);
}

/// `int SLoginMc_AbortAtFlush(int, int* done)` (0x3f8720): set `*done = 1`,
/// put the card state in state 11, phase 0x18, and call the interface method
/// at +0x38 with the open file's handle. Returns 0.
pub fn slogin_mc_abort_at_flush(c: &mut Cpu) {
    slogin_mc_abort_file(c, callee::SLOGIN_MC_ABORT_AT_FLUSH.addr, 11, 0x18, 0x38);
}

/// `int SLoginMc_AbortAtClose(int, int* done)` (0x3f8768): set `*done = 1`,
/// put the card state in state 12, phase 0x19, and call the interface method
/// at +0x1c with the open file's handle. Returns 0.
pub fn slogin_mc_abort_at_close(c: &mut Cpu) {
    slogin_mc_abort_file(c, callee::SLOGIN_MC_ABORT_AT_CLOSE.addr, 12, 0x19, 0x1c);
}

/// The shared body of `SLoginMc_AbortAtFlush` and `SLoginMc_AbortAtClose`:
/// enter `state`/`phase` and call interface method `method` on the open file.
fn slogin_mc_abort_file(c: &mut Cpu, entry: u32, state: u64, phase: u64, method: u32) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = state;
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = phase;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(card::PHASE), c.g[V1][0]);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(method)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(card::FILE_HANDLE)).lw(c);
        call_ptr(c, target, entry + 0x34)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_AbortAtFinish(int arg, int* done)` (0x3f87b0): set
/// `*done = 1`, put the card state in state 13, phase 0x1a, and call the
/// interface method at +0x14 with `arg`. Returns 0.
pub fn slogin_mc_abort_at_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A3][0] = 13;
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[A2][0] = 0x1a;
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
        c.sw((c.g[V1][0] as u32).wrapping_add(card::PHASE), c.g[A2][0]);
        c.sw(c.g[V1][0] as u32, c.g[A3][0]);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, 0x003f_87e4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `int SLogin_SetupIconSys(const void* icon)` (0x3f8258): build the
/// memory card icon metadata and hand it to the login store.
pub fn slogin_setup_icon_sys(c: &mut Cpu) {
    c.cycles += 85;
    let frame = Frame::open(c, 0x520);
    // Reserve the title buffer and icon.sys payload in the guest frame.
    c.g[A2][0] = 0x40;
    frame.save(c, FP, 0x490);
    c.g[A1][0] = 0u64;
    c.g[FP][0] = c.g[A0][0];
    frame.save(c, S0, 0x510);
    frame.save(c, S1, 0x500);
    frame.save(c, S2, 0x4f0);
    frame.save(c, S3, 0x4e0);
    frame.save(c, S4, 0x4d0);
    frame.save(c, S5, 0x4c0);
    frame.save(c, S6, 0x4b0);
    frame.save(c, S7, 0x4a0);
    frame.save_ra(c, 0x480);
    c.g[RA][0] = sx(0x003f8298);
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3d0));
    if call_ptr(c, callee::MEMSET.addr, 0x003f_8298).is_err() {
        return;
    }
    c.g[V1][0] = 0x80;
    c.g[V0][0] = DATA_PAGE;
    // Three static icon.sys template runs are copied to the guest frame.
    c.sw((c.g[SP][0] as u32).wrapping_add(0x3e8), c.g[V1][0]);
    c.g[A0][0] = DATA_PAGE;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x3d8), c.g[V1][0]);
    c.g[A1][0] = DATA_PAGE;
    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7320));
    c.ldl(V1, (c.g[T0][0] as u32).wrapping_add(0x7));
    c.ldr(V1, c.g[T0][0] as u32);
    c.ldl(A2, (c.g[T0][0] as u32).wrapping_add(0xf));
    c.ldr(A2, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.ldl(A3, (c.g[T0][0] as u32).wrapping_add(0x17));
    c.ldr(A3, (c.g[T0][0] as u32).wrapping_add(0x10));
    c.sdl(V1, (c.g[SP][0] as u32).wrapping_add(0x417));
    c.sdr(V1, (c.g[SP][0] as u32).wrapping_add(0x410));
    c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x41f));
    c.sdr(A2, (c.g[SP][0] as u32).wrapping_add(0x418));
    c.sdl(A3, (c.g[SP][0] as u32).wrapping_add(0x427));
    c.sdr(A3, (c.g[SP][0] as u32).wrapping_add(0x420));
    c.ldl(V1, (c.g[T0][0] as u32).wrapping_add(0x1f));
    c.ldr(V1, (c.g[T0][0] as u32).wrapping_add(0x18));
    c.ldl(A2, (c.g[T0][0] as u32).wrapping_add(0x27));
    c.ldr(A2, (c.g[T0][0] as u32).wrapping_add(0x20));
    c.ldl(A3, (c.g[T0][0] as u32).wrapping_add(0x2f));
    c.ldr(A3, (c.g[T0][0] as u32).wrapping_add(0x28));
    c.sdl(V1, (c.g[SP][0] as u32).wrapping_add(0x42f));
    c.sdr(V1, (c.g[SP][0] as u32).wrapping_add(0x428));
    c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x437));
    c.sdr(A2, (c.g[SP][0] as u32).wrapping_add(0x430));
    c.sdl(A3, (c.g[SP][0] as u32).wrapping_add(0x43f));
    c.sdr(A3, (c.g[SP][0] as u32).wrapping_add(0x438));
    c.g[V0][0] = 0x7;
    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7350));
    c.ldl(V1, (c.g[T0][0] as u32).wrapping_add(0x7));
    c.ldr(V1, c.g[T0][0] as u32);
    c.ldl(A2, (c.g[T0][0] as u32).wrapping_add(0xf));
    c.ldr(A2, (c.g[T0][0] as u32).wrapping_add(0x8));
    c.ldl(A3, (c.g[T0][0] as u32).wrapping_add(0x17));
    c.ldr(A3, (c.g[T0][0] as u32).wrapping_add(0x10));
    c.sdl(V1, (c.g[SP][0] as u32).wrapping_add(0x447));
    c.sdr(V1, (c.g[SP][0] as u32).wrapping_add(0x440));
    c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x44f));
    c.sdr(A2, (c.g[SP][0] as u32).wrapping_add(0x448));
    c.sdl(A3, (c.g[SP][0] as u32).wrapping_add(0x457));
    c.sdr(A3, (c.g[SP][0] as u32).wrapping_add(0x450));
    c.ldl(V1, (c.g[T0][0] as u32).wrapping_add(0x1f));
    c.ldr(V1, (c.g[T0][0] as u32).wrapping_add(0x18));
    c.ldl(A2, (c.g[T0][0] as u32).wrapping_add(0x27));
    c.ldr(A2, (c.g[T0][0] as u32).wrapping_add(0x20));
    c.ldl(A3, (c.g[T0][0] as u32).wrapping_add(0x2f));
    c.ldr(A3, (c.g[T0][0] as u32).wrapping_add(0x28));
    c.sdl(V1, (c.g[SP][0] as u32).wrapping_add(0x45f));
    c.sdr(V1, (c.g[SP][0] as u32).wrapping_add(0x458));
    c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x467));
    c.sdr(A2, (c.g[SP][0] as u32).wrapping_add(0x460));
    c.sdl(A3, (c.g[SP][0] as u32).wrapping_add(0x46f));
    c.sdr(A3, (c.g[SP][0] as u32).wrapping_add(0x468));
    if c.g[FP][0] != 0 {
        c.g[V0][0] = 0u64;
    }
    c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7380));
    c.ldl(V1, (c.g[A3][0] as u32).wrapping_add(0x7));
    c.ldr(V1, c.g[A3][0] as u32);
    c.ldl(A0, (c.g[A3][0] as u32).wrapping_add(0xf));
    c.ldr(A0, (c.g[A3][0] as u32).wrapping_add(0x8));
    c.sdl(V1, (c.g[SP][0] as u32).wrapping_add(0x477));
    c.sdr(V1, (c.g[SP][0] as u32).wrapping_add(0x470));
    c.sdl(A0, (c.g[SP][0] as u32).wrapping_add(0x47f));
    c.sdr(A0, (c.g[SP][0] as u32).wrapping_add(0x478));
    let t = c.g[V0][0] != 0u64;
    frame.restore(c, S0, 0x510);
    // A null icon has no metadata to install.
    if !t {
        c.cycles += 94;
        // Convert the title to Shift JIS and assemble the icon.sys header.
        c.g[A0][0] = c.g[SP][0] | 0x8;
        c.g[A2][0] = card::ICON_SYS_BYTES;
        c.g[RA][0] = sx(0x003f83bc);
        c.g[A1][0] = 0u64;
        if call_ptr(c, callee::MEMSET.addr, 0x003f_83bc).is_err() {
            return;
        }
        c.g[V1][0] = GLOBALS_PAGE;
        c.g[A0][0] = DATA_PAGE;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4bd0));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4bd0));
        c.g[A2][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7390));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[RA][0] = sx(0x003f83e4);
        c.sb((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        if callee::SLOGIN_MC_ASCII_TO_SJIS
            .call(c, 0x003f_83e4)
            .is_err()
        {
            return;
        }
        c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[T7][0] = 0x60;
        c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x408));
        c.g[S1][0] = 0x10;
        c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[T1][0] = DATA_PAGE;
        c.g[A2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x418));
        c.g[S7][0] = 0x296e;
        c.g[A3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x420));
        c.g[S6][0] = 0x22;
        c.g[T0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x428));
        c.g[A0][0] = c.g[SP][0];
        c.g[S5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3d0));
        c.g[S4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3d8));
        c.g[S3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3e0));
        c.g[S2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3e8));
        c.g[S0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3f0));
        c.g[T6][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3f8));
        c.g[T2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x430));
        c.g[T3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x438));
        c.g[T4][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x440));
        c.g[T5][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x448));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T0][0]);
        c.sh((c.g[SP][0] as u32).wrapping_add(0xe), c.g[S1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T7][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S5][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[S3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[S2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[S0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T6][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T4][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T5][0]);
        c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x450));
        c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x458));
        c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x460));
        c.g[A2][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x468));
        c.g[A3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x470));
        c.g[T0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x478));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x98), c.g[V0][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[A2][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xb8), c.g[A3][0]);
        c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[T0][0]);
        c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x73a0));
        c.ldl(T0, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(T0, c.g[A1][0] as u32);
        c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(T0, (c.g[SP][0] as u32).wrapping_add(0x113));
        c.sdr(T0, (c.g[SP][0] as u32).wrapping_add(0x10c));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x114), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x73a0));
        c.ldl(A2, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(A2, c.g[V0][0] as u32);
        c.g[A3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x8));
        c.sdl(A2, (c.g[SP][0] as u32).wrapping_add(0x153));
        c.sdr(A2, (c.g[SP][0] as u32).wrapping_add(0x14c));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x154), c.g[A3][0]);
        c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x73a0));
        c.ldl(V1, (c.g[A3][0] as u32).wrapping_add(0x7));
        c.ldr(V1, c.g[A3][0] as u32);
        c.g[A1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x8));
        c.sdl(V1, (c.g[SP][0] as u32).wrapping_add(0x193));
        c.sdr(V1, (c.g[SP][0] as u32).wrapping_add(0x18c));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x194), c.g[A1][0]);
        c.sw(c.g[SP][0] as u32, c.g[FP][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[S7][0]);
        c.g[RA][0] = sx(0x003f8518);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x3cc), c.g[S6][0]);
        // The store owns the final metadata validation and error mapping.
        if call_ptr(c, callee::SLOGIN_STORE_SET_ICON_SYS.addr, 0x003f_8518).is_err() {
            return;
        }
        c.g[RA][0] = sx(0x003f8520);
        c.g[A0][0] = c.g[V0][0];
        if callee::SLOGIN_MAP_STORE_ERROR.call(c, 0x003f_8520).is_err() {
            return;
        }
        frame.restore(c, S0, 0x510);
    }
    c.cycles += 11;
    frame.restore(c, S1, 0x500);
    frame.restore(c, S2, 0x4f0);
    frame.restore(c, S3, 0x4e0);
    frame.restore(c, S4, 0x4d0);
    frame.restore(c, S5, 0x4c0);
    frame.restore(c, S6, 0x4b0);
    frame.restore(c, S7, 0x4a0);
    frame.restore(c, FP, 0x490);
    frame.restore_ra(c, 0x480);
    frame.close(c);
}

/// `int SLoginMc_OpenStep(int result, int* done, int* card_reply)`
/// (0x3f8c38): advance the memory card open sequence through directory,
/// file, and completion phases. The dispatcher also accepts an event resume.
pub fn slogin_mc_open_step(c: &mut Cpu) {
    // Only the start and the card-device retry head can be entered externally.
    let frame = Frame { size: 0xa0 };
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            // Validate the caller's completion and card reply pointers.
            0x003f8c38 => {
                c.cycles += 9;
                Frame::open(c, 0xa0);
                frame.save(c, S0, 0x90);
                frame.save(c, S1, 0x80);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x70);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x60);
                let t = c.g[S1][0] == 0u64;
                frame.save_ra(c, 0x50);
                if t {
                    pc = 0x003f8c80;
                    continue;
                }
                pc = 0x003f8c5c;
                continue;
            }
            0x003f8c5c => {
                c.cycles += 2;
                let t = c.g[A2][0] == 0u64;
                c.g[V0][0] = 0x12;
                if t {
                    pc = 0x003f8edc;
                    continue;
                }
                pc = 0x003f8c64;
                continue;
            }
            0x003f8c64 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[V1][0] = 0x3;
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                let t = c.g[A0][0] == c.g[V1][0];
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x003f8c88;
                    continue;
                }
                pc = 0x003f8c78;
                continue;
            }
            0x003f8c78 => {
                c.cycles += 2;
                let t = c.g[A0][0] == c.g[V0][0];
                c.g[S3][0] = 0x1;
                if t {
                    pc = 0x003f8c8c;
                    continue;
                }
                pc = 0x003f8c80;
                continue;
            }
            0x003f8c80 => {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8c88 => {
                c.cycles += 1;
                c.g[S3][0] = 0x1;
                pc = 0x003f8c8c;
                continue;
            }
            // Select the current card operation phase.
            0x003f8c8c => {
                c.cycles += 6;
                c.g[S2][0] = 0x2;
                c.sw(c.g[S1][0] as u32, c.g[S3][0]);
                c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] == c.g[S2][0];
                c.g[V0][0] = (c.g[V1][0] < 0x3u64) as u64;
                if t {
                    pc = 0x003f8d78;
                    continue;
                }
                pc = 0x003f8ca4;
                continue;
            }
            0x003f8ca4 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x16;
                if t {
                    pc = 0x003f8cbc;
                    continue;
                }
                pc = 0x003f8cac;
                continue;
            }
            0x003f8cac => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[S3][0];
                c.g[V1][0] = 0x2;
                if t {
                    pc = 0x003f8cd4;
                    continue;
                }
                pc = 0x003f8cb4;
                continue;
            }
            0x003f8cb4 => {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                pc = 0x003f8ed0;
                continue;
            }
            0x003f8cbc => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x1a;
                if t {
                    pc = 0x003f8e0c;
                    continue;
                }
                pc = 0x003f8cc4;
                continue;
            }
            0x003f8cc4 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V1][0] = 0x2;
                if t {
                    pc = 0x003f8e5c;
                    continue;
                }
                pc = 0x003f8ccc;
                continue;
            }
            0x003f8ccc => {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                pc = 0x003f8ed0;
                continue;
            }
            // Begin opening the save directory or handle its response.
            0x003f8cd4 => {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;
                c.g[V0][0] = 0x3;
                if t {
                    pc = 0x003f8d14;
                    continue;
                }
                pc = 0x003f8cdc;
                continue;
            }
            0x003f8cdc => {
                c.cycles += 14;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x54));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x003f8cf0);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
                if callee::SLOGIN_MC_BUILD_DIR_PATH
                    .call(c, 0x003f_8cf0)
                    .is_err()
                {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003f8d0c);
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8d0c {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8d14 => {
                c.cycles += 2;

                if c.g[S0][0] != c.g[V0][0] {
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    pc = 0x003f8ec8;
                    continue;
                }
                pc = 0x003f8d1c;
                continue;
            }
            0x003f8d1c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                let t = c.g[V0][0] != c.g[S0][0];
                c.g[V1][0] = 0x2;
                if t {
                    pc = 0x003f8d6c;
                    continue;
                }
                pc = 0x003f8d28;
                continue;
            }
            0x003f8d28 => {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x6c));
                c.g[RA][0] = sx(0x003f8d38);
                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                if callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK
                    .call(c, 0x003f_8d38)
                    .is_err()
                {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = 0xffffffffffffffff;
                let t = c.g[A0][0] == c.g[V0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                if t {
                    pc = 0x003f8d68;
                    continue;
                }
                pc = 0x003f8d4c;
                continue;
            }
            0x003f8d4c => {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003f8d60);
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8d60 {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8d68 => {
                c.cycles += 1;
                c.g[V1][0] = 0x2;
                pc = 0x003f8d6c;
                continue;
            }
            0x003f8d6c => {
                c.cycles += 3;
                c.g[V0][0] = c.g[S0][0];
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                pc = 0x003f8edc;
                continue;
            }
            // The directory is open: request the account file.
            0x003f8d78 => {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;

                if t {
                    pc = 0x003f8e3c;
                    continue;
                }
                pc = 0x003f8d80;
                continue;
            }
            0x003f8d80 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                let t = c.g[A2][0] != c.g[S3][0];
                c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x34));
                if t {
                    pc = 0x003f8dd0;
                    continue;
                }
                pc = 0x003f8d8c;
                continue;
            }
            0x003f8d8c => {
                c.cycles += 17;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x54));
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x14));
                c.g[RA][0] = sx(0x003f8da0);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x50));
                if callee::SLOGIN_MC_BUILD_FILE_PATHS
                    .call(c, 0x003f_8da0)
                    .is_err()
                {
                    return;
                }
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[V0][0] = 0x16;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.g[A3][0] = 0x3;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003f8dc8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x34));
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8dc8 {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8dd0 => {
                c.cycles += 2;

                if c.g[A2][0] != 0u64 {
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    pc = 0x003f8e04;
                    continue;
                }
                pc = 0x003f8dd8;
                continue;
            }
            0x003f8dd8 => {
                c.cycles += 11;
                c.g[RA][0] = sx(0x003f8de0);
                c.g[A0][0] = 0x4;
                if callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_8de0).is_err() {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[V0][0] = 0x1a;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003f8dfc);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.pc = t;
                ssx3_ee_rt::funcs::call_indirect(c, t);
                if c.pc != 0x003f8dfc {
                    return;
                }
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            0x003f8e04 => {
                c.cycles += 2;
                c.g[V0][0] = 0xa;
                pc = 0x003f8edc;
                continue;
            }
            // A pending card operation has returned its result word.
            0x003f8e0c => {
                c.cycles += 2;
                let t = c.g[S0][0] != 0u64;

                if t {
                    pc = 0x003f8e3c;
                    continue;
                }
                pc = 0x003f8e14;
                continue;
            }
            0x003f8e14 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw(c.g[A2][0] as u32);
                let t = (c.g[A2][0] as i64) < 0;
                c.g[V0][0] = 0u64;
                if t {
                    pc = 0x003f8e2c;
                    continue;
                }
                pc = 0x003f8e20;
                continue;
            }
            0x003f8e20 => {
                c.cycles += 3;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                pc = 0x003f8edc;
                continue;
            }
            0x003f8e2c => {
                c.cycles += 4;
                c.sw((c.g[T0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                c.g[V0][0] = 0xa;
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                pc = 0x003f8edc;
                continue;
            }
            0x003f8e3c => {
                c.cycles += 8;
                c.g[RA][0] = sx(0x003f8e44);
                c.g[A0][0] = c.g[S0][0];
                if callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_8e44).is_err() {
                    return;
                }
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003f8e54);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                if callee::SLOGIN_MC_ABORT_AT_FINISH
                    .call(c, 0x003f_8e54)
                    .is_err()
                {
                    return;
                }
                frame.restore(c, S0, 0x90);
                pc = 0x003f8ee0;
                continue;
            }
            // Retry another device after a card error, or finish this open.
            0x003f8e5c => {
                c.cycles += 2;

                if c.g[S0][0] != 0u64 {
                    c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                    pc = 0x003f8ec8;
                    continue;
                }
                pc = 0x003f8e64;
                continue;
            }
            0x003f8e64 => {
                c.cycles += 4;
                c.g[A0][0] = c.lw(c.g[T0][0] as u32);
                c.g[V0][0] = 0x4;
                let t = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 0x6;
                if t {
                    pc = 0x003f8eb8;
                    continue;
                }
                pc = 0x003f8e74;
                continue;
            }
            0x003f8e74 => {
                c.cycles += 2;
                let t = c.g[A0][0] == c.g[V0][0];

                if t {
                    pc = 0x003f8eb8;
                    continue;
                }
                pc = 0x003f8e7c;
                continue;
            }
            0x003f8e7c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003f8e84);
                if callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_8e84).is_err() {
                    return;
                }
                let t = c.g[V0][0] != 0u64;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                if t {
                    pc = 0x003f8e94;
                    continue;
                }
                pc = 0x003f8e8c;
                continue;
            }
            0x003f8e8c => {
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                pc = 0x003f8ed8;
                continue;
            }
            0x003f8e94 => {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6c));
                c.g[RA][0] = sx(0x003f8ea4);
                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                if callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK
                    .call(c, 0x003f_8ea4)
                    .is_err()
                {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = 0xffffffffffffffff;
                let t = c.g[A0][0] != c.g[V0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                if t {
                    if back_edge_unwinds(c, 0x003f_8d4c) {
                        return;
                    }
                    pc = 0x003f8d4c;
                    continue;
                }
                pc = 0x003f8eb8;
                continue;
            }
            0x003f8eb8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003f8ec0);
                c.sw(c.g[S1][0] as u32, c.g[S2][0]);
                if callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_8ec0).is_err() {
                    return;
                }
                frame.restore(c, S0, 0x90);
                pc = 0x003f8ee0;
                continue;
            }
            0x003f8ec8 => {
                c.cycles += 2;
                c.g[V0][0] = c.g[S0][0];
                pc = 0x003f8edc;
                continue;
            }
            0x003f8ed0 => {
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                pc = 0x003f8edc;
                continue;
            }
            0x003f8ed8 => {
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                pc = 0x003f8edc;
                continue;
            }
            // Shared return path, also reached after a resumed retry.
            0x003f8edc => {
                c.cycles += 1;
                frame.restore(c, S0, 0x90);
                pc = 0x003f8ee0;
                continue;
            }
            0x003f8ee0 => {
                c.cycles += 6;
                frame.restore(c, S1, 0x80);
                frame.restore(c, S2, 0x70);
                frame.restore(c, S3, 0x60);
                frame.restore_ra(c, 0x50);
                frame.close(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003f8c38, 0x003f8ef8);
                return;
            }
        }
    }
}

/// `SLoginMc_SetIconSys` (0x3fa240): install a memory-card save icon and
/// calculate the space its assets require.
pub fn slogin_mc_set_icon_sys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let valid_icon = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        if !valid_icon {
            c.cycles += 2;
            c.g[V0][0] = 12;
        } else {
            c.cycles += 4;
            callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_a264)?;
            let initialized = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
            if !initialized {
                c.cycles += 2;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = (c.g[S0][0] | c.g[V1][0]) & 7;
                let aligned = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c0));
                if aligned {
                    loop {
                        c.cycles += 11;
                        c.g[A1][0] = c.ld(c.g[A0][0] as u32);
                        c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(8));
                        c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(16));
                        c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(24));
                        c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(8), c.g[A2][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
                        c.sd((c.g[V1][0] as u32).wrapping_add(24), c.g[T0][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                        let again = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003f_a2e4);
                        }
                    }
                } else {
                    loop {
                        c.cycles += 19;
                        c.ldl(A1, (c.g[A0][0] as u32).wrapping_add(7));
                        c.ldr(A1, c.g[A0][0] as u32);
                        c.ldl(A2, (c.g[A0][0] as u32).wrapping_add(15));
                        c.ldr(A2, (c.g[A0][0] as u32).wrapping_add(8));
                        c.ldl(A3, (c.g[A0][0] as u32).wrapping_add(23));
                        c.ldr(A3, (c.g[A0][0] as u32).wrapping_add(16));
                        c.ldl(T0, (c.g[A0][0] as u32).wrapping_add(31));
                        c.ldr(T0, (c.g[A0][0] as u32).wrapping_add(24));
                        c.sdl(A1, (c.g[V1][0] as u32).wrapping_add(7));
                        c.sdr(A1, c.g[V1][0] as u32);
                        c.sdl(A2, (c.g[V1][0] as u32).wrapping_add(15));
                        c.sdr(A2, (c.g[V1][0] as u32).wrapping_add(8));
                        c.sdl(A3, (c.g[V1][0] as u32).wrapping_add(23));
                        c.sdr(A3, (c.g[V1][0] as u32).wrapping_add(16));
                        c.sdl(T0, (c.g[V1][0] as u32).wrapping_add(31));
                        c.sdr(T0, (c.g[V1][0] as u32).wrapping_add(24));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                        let again = c.g[A0][0] != c.g[V0][0];
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                        if !again {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003f_a28c);
                        }
                    }
                    c.cycles += 2;
                }
                c.cycles += 15;
                c.ldl(V0, (c.g[A0][0] as u32).wrapping_add(7));
                c.ldr(V0, c.g[A0][0] as u32);
                c.ldl(A1, (c.g[A0][0] as u32).wrapping_add(15));
                c.ldr(A1, (c.g[A0][0] as u32).wrapping_add(8));
                c.sdl(V0, (c.g[V1][0] as u32).wrapping_add(7));
                c.sdr(V0, c.g[V1][0] as u32);
                c.sdl(A1, (c.g[V1][0] as u32).wrapping_add(15));
                c.sdr(A1, (c.g[V1][0] as u32).wrapping_add(8));
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x424));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x420));
                callee::SLOGIN_MC_CALC_SAVE_SIZE.call(c, 0x003f_a344)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::SLOGIN_MC_VALIDATE_ICON_SYS.addr, 0x003f_a34c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_Init` (0x3f9218): install the card interface, build its device
/// mask, and allocate login card state.
pub fn slogin_mc_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9248)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        let mut valid = !initialized;
        let mut special_epilogue = false;
        if valid {
            c.cycles += 2;
            valid = c.g[S3][0] != 0;
            c.g[V0][0] = 12;
        }
        if valid {
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[S3][0] as u32);
            c.g[S4][0] = u64::MAX;
            valid = c.g[V0][0] != c.g[S4][0];
            c.g[V0][0] = 12;
        }
        if valid {
            c.cycles += 2;
            valid = c.g[S0][0] != 0;
            if !valid {
                frame.restore(c, S0, 0x50);
                special_epilogue = true;
            }
        }
        if valid {
            for slot in 0..16u32 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(slot * 4));
                valid = c.g[V0][0] != 0;
                c.g[V0][0] = 12;
                if !valid {
                    break;
                }
            }
        }
        if valid {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            valid = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
            if !valid {
                c.cycles += 2;
                c.g[V0][0] = 12;
            }
        }
        if valid {
            c.cycles += 2;
            valid = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
        }
        if valid {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
            valid = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
        }
        if valid {
            c.cycles += 6;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = card::IFACE_BYTES;
            c.g[A2][0] = 4;
            call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_9360)?;
            let allocated_iface = c.g[V0][0] != 0;
            c.sw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP), c.g[V0][0]);
            if allocated_iface {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = card::STATE_BYTES;
                c.g[A2][0] = 4;
                call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_9378)?;
                valid = c.g[V0][0] != 0;
                c.sw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP), c.g[V0][0]);
            } else {
                valid = false;
            }
            if !valid {
                c.cycles += 2;
                c.g[V0][0] = 7;
            }
        }
        if valid {
            c.cycles += 7;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
            c.g[A0][0] = c.g[S3][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x428), c.g[S2][0]);
            c.sh(c.g[V1][0] as u32, 0);
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            let terminated = c.g[V0][0] == c.g[S4][0];
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
            if !terminated {
                c.cycles += 5;
                c.g[T3][0] = c.g[V1][0];
                c.g[T4][0] = 9;
                c.g[T7][0] = 8;
                c.g[T6][0] = 1;
                c.g[T5][0] = u64::MAX;
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = (c.g[V1][0] < 8) as u64;
                    let in_range = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[V1][0];
                    if !in_range {
                        c.cycles += 2;
                        let all_devices = c.g[A1][0] == c.g[T4][0];
                        c.g[V0][0] = 9;
                        if !all_devices {
                            valid = false;
                            break;
                        }
                    }
                    c.cycles += 2;
                    let list_full = c.g[S1][0] == c.g[T7][0];
                    c.g[V0][0] = 12;
                    if list_full {
                        valid = false;
                        break;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[T4][0] {
                        c.cycles += 8;
                        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[A0][0] = 0;
                        c.g[T2][0] = 1;
                        c.g[A2][0] = 0x100_0000;
                        c.g[T1][0] = 0x100_0000;
                        loop {
                            c.cycles += 9;
                            c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                            c.g[V1][0] = sx((c.g[T2][0] as u32) << ((c.g[A0][0] as u32) & 31));
                            c.g[A0][0] = c.g[A2][0];
                            c.g[V0][0] |= c.g[V1][0];
                            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                            c.sh(c.g[A1][0] as u32, c.g[V0][0]);
                            c.g[V0][0] = ((c.g[A0][0] as i64) < 8) as u64;
                            let again = c.g[V0][0] != 0;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T1][0] as u32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9408);
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = c.g[T0][0];
                    } else {
                        c.g[V1][0] = c.lhu(c.g[A0][0] as u32);
                        c.cycles += 7;
                        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                        c.g[V0][0] = c.lhu(c.g[T3][0] as u32);
                        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        c.g[V1][0] = sx((c.g[T6][0] as u32) << ((c.g[V1][0] as u32) & 31));
                        c.g[V0][0] |= c.g[V1][0];
                        c.sh(c.g[T3][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[T0][0];
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    let again = c.g[V0][0] != c.g[T5][0];
                    c.g[S1][0] = c.g[A3][0] & 0xff;
                    if !again {
                        c.cycles += 1;
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_93c0);
                    }
                }
            }
            if valid {
                c.cycles += 44;
                c.g[A3][0] = 0x45_0000;
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.g[A2][0] = 0x45_0000;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1468));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x14a0));
                c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
                c.g[V0][0] = 0;
                for i in 1..17u32 {
                    if i & 1 == 1 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4 * i));
                        c.sw((c.g[A1][0] as u32).wrapping_add(4 + 4 * i), c.g[V1][0]);
                    } else {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4 * i));
                        c.sw((c.g[A1][0] as u32).wrapping_add(4 + 4 * i), c.g[A0][0]);
                    }
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x48), c.g[A3][0]);
                c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                c.sw(c.g[V1][0] as u32, 0);
            }
        }
        if !special_epilogue {
            c.cycles += 1;
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
    });
}

/// `SLoginMc_StartCreate` (0x3f9818): queue creation of a named memory-card
/// file after validating its title and card.
pub fn slogin_mc_start_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_983c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_name = c.g[S2][0] != 0;
            c.g[V0][0] = 12;
            if valid_name {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, callee::STRLEN.addr, 0x003f_9854)?;
                let valid_length = c.g[V0][0] < card::NAME_LIMIT;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if !valid_length {
                    c.cycles += 2;
                    c.g[V0][0] = 12;
                } else {
                    c.cycles += 3;
                    let valid_index = c.g[V0][0] < 9;
                    c.g[V0][0] = u64::MAX;
                    if !valid_index {
                        c.cycles += 2;
                        c.g[V0][0] = 9;
                    } else {
                        c.cycles += 2;
                        let auto_device = c.g[S0][0] == c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                        let mut allowed = true;
                        if auto_device {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = 1;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S0][0] as u32) & 31));
                            c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                            c.g[V0][0] &= c.g[V1][0];
                            allowed = c.g[V0][0] != 0;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                        }
                        if !allowed {
                            c.cycles += 2;
                            c.g[V0][0] = 11;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            let busy = c.g[V0][0] != 0;
                            c.g[V0][0] = 13;
                            if !busy {
                                c.cycles += 7;
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(card::FILE_HANDLE));
                                c.g[A1][0] = 0;
                                c.g[A2][0] = card::REQUEST_BYTES;
                                call_ptr(c, callee::MEMSET.addr, 0x003f_98bc)?;
                                c.g[V1][0] = u64::MAX;
                                let explicit_device = c.g[S0][0] != c.g[V1][0];
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                let mut start = true;
                                if explicit_device {
                                    c.cycles += 3;
                                    c.g[V0][0] = 4;
                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                                } else {
                                    c.cycles += 6;
                                    c.g[V0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                    c.g[S0][0] = 1;
                                    c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                    callee::SLOGIN_MC_COUNT_DEVICES_IN_MASK.call(c, 0x003f_98d8)?;
                                    let single = c.g[V0][0] == c.g[S0][0];
                                    c.g[V0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                    if single {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                        callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                            .call(c, 0x003f_98e8)?;
                                        c.g[A0][0] =
                                            c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                        c.g[V1][0] = 4;
                                        c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                                        c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S1][0] == 0 {
                                            c.cycles += 6;
                                            c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                            callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                                .call(c, 0x003f_990c)?;
                                            c.g[V1][0] = c
                                                .lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::STATE_GP));
                                            c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x10),
                                                c.g[V0][0],
                                            );
                                        } else {
                                            c.cycles += 2;
                                            let last = c.g[S1][0] == c.g[S0][0];
                                            c.g[V0][0] = 12;
                                            if !last {
                                                start = false;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::IFACE_GP));
                                                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                                callee::SLOGIN_MC_LAST_DEVICE_IN_MASK
                                                    .call(c, 0x003f_9930)?;
                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::STATE_GP));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[V0][0],
                                                );
                                                c.sw(c.g[V1][0] as u32, c.g[S1][0]);
                                            }
                                        }
                                    }
                                }
                                if start {
                                    c.cycles += 19;
                                    c.g[A0][0] = 0;
                                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_995c)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54));
                                    call_ptr(c, callee::STRCPY.addr, 0x003f_996c)?;
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.g[A0][0] = 1;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                                    call_ptr(c, callee::SLOGIN_MC_SET_BUSY.addr, 0x003f_997c)?;
                                    c.g[V0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                    let target = c.g[A1][0] as u32;
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                    call_ptr(c, target, 0x003f_9990)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.g[V1][0] = 1;
                                    c.g[V0][0] = 0;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V1][0]);
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_StartDelete` (0x3f9698): queue deletion of a named memory-card
/// file on a selected card.
pub fn slogin_mc_start_delete(c: &mut Cpu) {
    slogin_mc_start_named(c, false);
}

/// `SLoginMc_StartOpen` (0x3f99b8): queue opening a named memory-card file on a
/// selected card.
pub fn slogin_mc_start_open(c: &mut Cpu) {
    slogin_mc_start_named(c, true);
}

/// The shared body of `SLoginMc_StartDelete` and `SLoginMc_StartOpen`.
pub(crate) fn slogin_mc_start_named(c: &mut Cpu, open: bool) {
    guest(c, |c| {
        // StartOpen's return addresses sit at the same offsets from its start
        // as StartDelete's.
        let shift = if open {
            callee::SLOGIN_MC_START_OPEN.addr - callee::SLOGIN_MC_START_DELETE.addr
        } else {
            0
        };
        let phase_single = if open { 6 } else { 5 };
        let phase_search = if open { 3 } else { 2 };
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_96bc + shift)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_name = c.g[S2][0] != 0;
            c.g[V0][0] = 12;
            if valid_name {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                let valid_index = c.g[V0][0] < 9;
                c.g[V0][0] = u64::MAX;
                if !valid_index {
                    c.cycles += 2;
                    c.g[V0][0] = 9;
                } else {
                    c.cycles += 2;
                    let auto_device = c.g[S0][0] == c.g[V0][0];
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                    let mut allowed = true;
                    if auto_device {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = 1;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S0][0] as u32) & 31));
                        c.g[V0][0] = c.lhu(c.g[A0][0] as u32);
                        c.g[V0][0] &= c.g[V1][0];
                        allowed = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    }
                    if !allowed {
                        c.cycles += 2;
                        c.g[V0][0] = 11;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let busy = c.g[V0][0] != 0;
                        c.g[V0][0] = 13;
                        if !busy {
                            c.cycles += 7;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(card::FILE_HANDLE));
                            c.g[A1][0] = 0;
                            c.g[A2][0] = card::REQUEST_BYTES;
                            call_ptr(c, callee::MEMSET.addr, 0x003f_972c + shift)?;
                            c.g[V1][0] = u64::MAX;
                            let explicit_device = c.g[S0][0] != c.g[V1][0];
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                            let mut start = true;
                            if explicit_device {
                                c.cycles += 3;
                                c.g[V0][0] = phase_single;
                                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                callee::SLOGIN_MC_COUNT_DEVICES_IN_MASK
                                    .call(c, 0x003f_9744 + shift)?;
                                c.g[V1][0] = 1;
                                let single = c.g[V0][0] == 1;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                if single {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                    callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                        .call(c, 0x003f_9758 + shift)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.g[V1][0] = phase_single;
                                } else {
                                    c.cycles += 2;
                                    if c.g[S1][0] == 0 {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                        callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK
                                            .call(c, 0x003f_9774 + shift)?;
                                        c.g[A0][0] =
                                            c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    } else {
                                        c.cycles += 2;
                                        let last = c.g[S1][0] == c.g[V1][0];
                                        c.g[V0][0] = 12;
                                        if !last {
                                            start = false;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] = c
                                                .lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::IFACE_GP));
                                            c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
                                            callee::SLOGIN_MC_LAST_DEVICE_IN_MASK
                                                .call(c, 0x003f_9790 + shift)?;
                                            c.g[A0][0] = c
                                                .lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::STATE_GP));
                                        }
                                    }
                                    if start {
                                        c.cycles += 1;
                                        c.g[V1][0] = phase_search;
                                    }
                                }
                                if start {
                                    c.cycles += 3;
                                    c.sw(c.g[A0][0] as u32, c.g[V1][0]);
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                }
                            }
                            if start {
                                c.cycles += 19;
                                c.g[A0][0] = 0;
                                callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_97b8 + shift)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x54));
                                call_ptr(c, callee::STRCPY.addr, 0x003f_97c8 + shift)?;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                c.g[A0][0] = 1;
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
                                call_ptr(c, callee::SLOGIN_MC_SET_BUSY.addr, 0x003f_97d8 + shift)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                let target = c.g[A1][0] as u32;
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                call_ptr(c, target, 0x003f_97ec + shift)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                c.g[V1][0] = 1;
                                c.g[V0][0] = 0;
                                c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V1][0]);
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_StartSeek` (0x3f9c48): queue a seek on the open card file,
/// applying the save-data offset for data seeks.
pub fn slogin_mc_start_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9c6c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_file = c.g[S0][0] != 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
            if !valid_file {
                c.cycles += 2;
                c.g[V0][0] = 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 13;
                if !busy {
                    c.cycles += 13;
                    c.g[V1][0] = 8;
                    c.g[V0][0] = 0x1b;
                    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
                    c.g[A0][0] = 0;
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_9ca8)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[A0][0] | c.g[V1][0]) & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    if aligned {
                        loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[A0][0] as u32);
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(8));
                            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(16));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(24));
                            c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(8), c.g[A2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(24), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9d18);
                            }
                        }
                    } else {
                        loop {
                            c.cycles += 19;
                            c.ldl(A1, (c.g[A0][0] as u32).wrapping_add(7));
                            c.ldr(A1, c.g[A0][0] as u32);
                            c.ldl(A2, (c.g[A0][0] as u32).wrapping_add(15));
                            c.ldr(A2, (c.g[A0][0] as u32).wrapping_add(8));
                            c.ldl(A3, (c.g[A0][0] as u32).wrapping_add(23));
                            c.ldr(A3, (c.g[A0][0] as u32).wrapping_add(16));
                            c.ldl(T0, (c.g[A0][0] as u32).wrapping_add(31));
                            c.ldr(T0, (c.g[A0][0] as u32).wrapping_add(24));
                            c.sdl(A1, (c.g[V1][0] as u32).wrapping_add(7));
                            c.sdr(A1, c.g[V1][0] as u32);
                            c.sdl(A2, (c.g[V1][0] as u32).wrapping_add(15));
                            c.sdr(A2, (c.g[V1][0] as u32).wrapping_add(8));
                            c.sdl(A3, (c.g[V1][0] as u32).wrapping_add(23));
                            c.sdr(A3, (c.g[V1][0] as u32).wrapping_add(16));
                            c.sdl(T0, (c.g[V1][0] as u32).wrapping_add(31));
                            c.sdr(T0, (c.g[V1][0] as u32).wrapping_add(24));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9cc4);
                            }
                        }
                        c.cycles += 2;
                    }
                    c.cycles += 2;
                    let relative_to_data = c.g[S1][0] == 0;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    if relative_to_data {
                        c.cycles += 14;
                        callee::SLOGIN_MC_GET_DATA_OFFSET.call(c, 0x003f_9d54)?;
                        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                        callee::SLOGIN_MC_GET_DATA_OFFSET.call(c, 0x003f_9d68)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        let target = c.g[V1][0] as u32;
                        c.g[A2][0] = 0;
                        call_ptr(c, target, 0x003f_9d7c)?;
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                        c.g[A2][0] = c.g[S1][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[S2][0]);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x34));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, target, 0x003f_9da0)?;
                        c.g[V0][0] = 0;
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_StartClose` (0x3f9b38): queue closure of the selected memory-card
/// file and save its file record.
pub fn slogin_mc_start_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9b4c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let valid_file = c.g[S0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
            if !valid_file {
                c.cycles += 2;
                c.g[V0][0] = 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 13;
                if !busy {
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[S0][0] | c.g[A0][0]) & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[V1][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                    if aligned {
                        loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(8));
                            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(16));
                            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(24));
                            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(8), c.g[A2][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(16), c.g[A3][0]);
                            c.sd((c.g[A0][0] as u32).wrapping_add(24), c.g[T0][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            let again = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9be0);
                            }
                        }
                        c.cycles += 1;
                    } else {
                        loop {
                            c.cycles += 19;
                            c.ldl(A1, (c.g[V1][0] as u32).wrapping_add(7));
                            c.ldr(A1, c.g[V1][0] as u32);
                            c.ldl(A2, (c.g[V1][0] as u32).wrapping_add(15));
                            c.ldr(A2, (c.g[V1][0] as u32).wrapping_add(8));
                            c.ldl(A3, (c.g[V1][0] as u32).wrapping_add(23));
                            c.ldr(A3, (c.g[V1][0] as u32).wrapping_add(16));
                            c.ldl(T0, (c.g[V1][0] as u32).wrapping_add(31));
                            c.ldr(T0, (c.g[V1][0] as u32).wrapping_add(24));
                            c.sdl(A1, (c.g[A0][0] as u32).wrapping_add(7));
                            c.sdr(A1, c.g[A0][0] as u32);
                            c.sdl(A2, (c.g[A0][0] as u32).wrapping_add(15));
                            c.sdr(A2, (c.g[A0][0] as u32).wrapping_add(8));
                            c.sdl(A3, (c.g[A0][0] as u32).wrapping_add(23));
                            c.sdr(A3, (c.g[A0][0] as u32).wrapping_add(16));
                            c.sdl(T0, (c.g[A0][0] as u32).wrapping_add(31));
                            c.sdr(T0, (c.g[A0][0] as u32).wrapping_add(24));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            let again = c.g[V1][0] != c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x003f_9b88);
                            }
                        }
                        c.cycles += 2;
                    }
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.cycles += 9;
                    c.g[A1][0] = 7;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                    c.g[V0][0] = 0x18;
                    c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    call_ptr(c, target, 0x003f_9c30)?;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_Poll` (0x3fa040): poll the card interface and dispatch completed
/// shared-login card phases.
pub fn slogin_mc_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_a064)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
        if !initialized {
            c.cycles += 2;
            c.g[V0][0] = 2;
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 13;
            } else {
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    c.g[V0][0] = 12;
                } else {
                    c.cycles += 2;
                    let valid_result = c.g[S2][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if !valid_result {
                        c.cycles += 2;
                        c.g[V0][0] = 12;
                    } else {
                        c.cycles += 5;
                        let target = c.g[V0][0] as u32;
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, target, 0x003f_a0a8)?;
                        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                        let active_status = c.g[V1][0] != 0;
                        c.g[S0][0] = c.g[V0][0];
                        let mut dispatch = active_status;
                        if !active_status {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let busy = c.g[V1][0] != 0;
                            c.g[V0][0] = 15;
                            dispatch = !busy;
                        }
                        if dispatch {
                            c.cycles += 4;
                            c.g[S3][0] = c.lw(c.g[S1][0] as u32);
                            c.g[V0][0] = 2;
                            let completed = c.g[S3][0] == c.g[V0][0];
                            c.g[V0][0] = c.g[S0][0];
                            if completed {
                                c.cycles += 15;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                c.g[A1][0] = 2;
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
                                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                let target = c.g[V0][0] as u32;
                                c.g[A2][0] = c.lw(c.g[S2][0] as u32);
                                call_ptr(c, target, 0x003f_a100)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[V0][0] = 10;
                                if c.g[S0][0] == 10 {
                                    c.cycles += 4;
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                                    c.sw(c.g[V1][0] as u32, 0);
                                } else {
                                    c.g[V0][0] = 16;
                                    c.cycles += 2;
                                    let still_active = c.g[S0][0] == 16;
                                    c.g[A3][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                    if !still_active {
                                        c.cycles += 13;
                                        c.g[V0][0] =
                                            c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                        let target = c.g[V0][0] as u32;
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, target, 0x003f_a150)?;
                                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                                        let unchanged_status = c.g[A0][0] == c.g[S3][0];
                                        c.g[S0][0] = c.g[V0][0];
                                        if unchanged_status {
                                            c.cycles += 5;
                                            c.g[V0][0] = c
                                                .lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::STATE_GP));
                                            c.g[V1][0] = 5;
                                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                                            let stage = c.g[A2][0];
                                            let clear_busy = if stage == 5 {
                                                true
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] = 7;
                                                if stage == c.g[A0][0] {
                                                    true
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[V0][0] = 11;
                                                    if stage == 7 {
                                                        true
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[V0][0] = 12;
                                                        if stage == 11 {
                                                            true
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[V0][0] = 13;
                                                            if stage == 12 {
                                                                true
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[V0][0] = c.lw((c.g[GP][0]
                                                                    as u32)
                                                                    .wrapping_add(card::STATE_GP));
                                                                stage == 13
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            if clear_busy {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0;
                                                call_ptr(
                                                    c,
                                                    callee::SLOGIN_MC_SET_BUSY.addr,
                                                    0x003f_a1a0,
                                                )?;
                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                    .wrapping_add(card::STATE_GP));
                                            }
                                            c.cycles += 2;
                                            c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
                                            c.sw(c.g[V0][0] as u32, 0);
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.g[S0][0];
                                    }
                                }
                            }
                        }
                    }
                }
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
    });
}

/// `SLoginMc_AsciiToSjis` (0x3fa380): convert a card icon's ASCII title into
/// the two-byte Shift-JIS save header.
pub fn slogin_mc_ascii_to_sjis(c: &mut Cpu) {
    c.cycles += 30;
    c.g[A2][0] = c.g[A0][0];
    let frame = Frame::open(c, 0x30);
    c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
    c.g[V1][0] = DATA_PAGE;
    c.g[T1][0] = c.g[A1][0];
    c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x73b0));
    c.ldl(A0, (c.g[T2][0] as u32).wrapping_add(7));
    c.ldr(A0, c.g[T2][0] as u32);
    c.ldl(A3, (c.g[T2][0] as u32).wrapping_add(15));
    c.ldr(A3, (c.g[T2][0] as u32).wrapping_add(8));
    c.ldl(T0, (c.g[T2][0] as u32).wrapping_add(23));
    c.ldr(T0, (c.g[T2][0] as u32).wrapping_add(16));
    c.sdl(A0, sp(c).wrapping_add(7));
    c.sdr(A0, sp(c));
    c.sdl(A3, sp(c).wrapping_add(15));
    c.sdr(A3, sp(c).wrapping_add(8));
    c.sdl(T0, sp(c).wrapping_add(23));
    c.sdr(T0, sp(c).wrapping_add(16));
    c.ldl(A0, (c.g[T2][0] as u32).wrapping_add(31));
    c.ldr(A0, (c.g[T2][0] as u32).wrapping_add(24));
    c.g[A3][0] = c.lb((c.g[T2][0] as u32).wrapping_add(32));
    c.sdl(A0, sp(c).wrapping_add(31));
    c.sdr(A0, sp(c).wrapping_add(24));
    c.sb(sp(c).wrapping_add(32), c.g[A3][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
    c.g[A1][0] = 0;
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
    c.g[A0][0] = c.g[V0][0] & 0xffff;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        c.g[A3][0] = 0x8100;
        c.g[T2][0] = 0x8220;
        c.g[T0][0] = 0x821f;
        loop {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x81));
            let double_byte = c.g[V0][0] < 0x18;
            c.g[V0][0] = double_byte as u64;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 8);
            if double_byte {
                c.cycles += 7;
                c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = c.g[V0][0] & 0xffff;
            } else {
                c.cycles += 3;
                let below_z = c.g[A0][0] < 0x7b;
                c.g[V0][0] = (c.g[A0][0] < 0x61) as u64;
                if !below_z {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x5e));
                    c.cycles += 1;
                    c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                } else {
                    c.cycles += 2;
                    let below_a = c.g[V0][0] != 0;
                    c.g[V0][0] = (c.g[A0][0] < 0x5b) as u64;
                    if !below_a {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                        c.g[A0][0] = c.g[V0][0] & 0xffff;
                    } else {
                        c.cycles += 2;
                        let below_bracket = c.g[V0][0] != 0;
                        c.g[V0][0] = (c.g[A0][0] < 0x41) as u64;
                        if !below_bracket {
                            c.cycles += 3;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x44));
                            c.cycles += 1;
                            c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                        } else {
                            c.cycles += 2;
                            let below_upper = c.g[V0][0] != 0;
                            c.g[V0][0] = (c.g[A0][0] < 0x3a) as u64;
                            if !below_upper {
                                c.cycles += 3;
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.g[A0][0] = c.g[V0][0] & 0xffff;
                            } else {
                                c.cycles += 2;
                                let below_colon = c.g[V0][0] != 0;
                                c.g[V0][0] = (c.g[A0][0] < 0x30) as u64;
                                if !below_colon {
                                    c.cycles += 3;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x2a));
                                    c.cycles += 1;
                                    c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                                } else {
                                    c.cycles += 2;
                                    let below_digit = c.g[V0][0] != 0;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(sp(c)));
                                    if below_digit {
                                        c.cycles += 1;
                                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x20));
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                        c.g[A0][0] = c.g[V0][0] & 0xffff;
                                    }
                                    if below_digit {
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[V1][0] | c.g[A3][0];
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 15;
            c.g[V0][0] = c.g[A0][0] & 0xff;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
            c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
            c.g[A0][0] |= c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.sh(c.g[V1][0] as u32, c.g[A0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[A1][0] &= 0xff;
            c.g[V0][0] = c.lbu(c.g[A2][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
            c.g[A0][0] = c.g[V0][0] & 0xffff;
            let again = c.g[A0][0] != 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            if !again {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_a408);
            }
        }
    }
    c.cycles += 2;
    frame.close(c);
}

/// `SLoginMc_StartRead` (0x3f9dc0): begin reading from the open memory-card
/// file into the caller's buffer.
pub fn slogin_mc_start_read(c: &mut Cpu) {
    slogin_mc_start_transfer(c, false);
}

/// `SLoginMc_StartWrite` (0x3f9f00): begin writing the caller's buffer into the
/// open memory-card file.
pub fn slogin_mc_start_write(c: &mut Cpu) {
    slogin_mc_start_transfer(c, true);
}

/// The shared body of `SLoginMc_StartWrite` and `SLoginMc_StartRead`.
pub(crate) fn slogin_mc_start_transfer(c: &mut Cpu, write: bool) {
    guest(c, |c| {
        let initialized_ret = if write { 0x003f_9f24 } else { 0x003f_9de4 };
        let error_ret = if write { 0x003f_9f60 } else { 0x003f_9e20 };
        let method_ret = if write { 0x003f_a020 } else { 0x003f_9ee0 };
        let unaligned_head = if write { 0x003f_9f7c } else { 0x003f_9e3c };
        let aligned_head = if write { 0x003f_9fd4 } else { 0x003f_9e94 };
        let phase = if write { 9 } else { 10 };
        let subphase = if write { 0x1c } else { 0x1d };
        let method_offset = if write { 0x30 } else { 0x2c };
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, initialized_ret)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 2;
            let has_file = c.g[S0][0] != 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
            if !has_file {
                c.cycles += 2;
                c.g[V0][0] = 12;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 13;
                if !busy {
                    c.cycles += 12;
                    c.g[V1][0] = phase;
                    c.g[V0][0] = subphase;
                    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V0][0]);
                    c.g[A0][0] = 0;
                    c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    callee::SLOGIN_MC_SET_ERROR.call(c, error_ret)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = (c.g[S0][0] | c.g[V1][0]) & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x60));
                    if aligned {
                        loop {
                            c.cycles += 11;
                            c.g[A1][0] = c.ld(c.g[A0][0] as u32);
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(8));
                            c.g[A3][0] = c.ld((c.g[A0][0] as u32).wrapping_add(16));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(24));
                            c.sd(c.g[V1][0] as u32, c.g[A1][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(8), c.g[A2][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(16), c.g[A3][0]);
                            c.sd((c.g[V1][0] as u32).wrapping_add(24), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, aligned_head);
                            }
                        }
                        c.cycles += 1;
                    } else {
                        loop {
                            c.cycles += 19;
                            c.ldl(A1, (c.g[A0][0] as u32).wrapping_add(7));
                            c.ldr(A1, c.g[A0][0] as u32);
                            c.ldl(A2, (c.g[A0][0] as u32).wrapping_add(15));
                            c.ldr(A2, (c.g[A0][0] as u32).wrapping_add(8));
                            c.ldl(A3, (c.g[A0][0] as u32).wrapping_add(23));
                            c.ldr(A3, (c.g[A0][0] as u32).wrapping_add(16));
                            c.ldl(T0, (c.g[A0][0] as u32).wrapping_add(31));
                            c.ldr(T0, (c.g[A0][0] as u32).wrapping_add(24));
                            c.sdl(A1, (c.g[V1][0] as u32).wrapping_add(7));
                            c.sdr(A1, c.g[V1][0] as u32);
                            c.sdl(A2, (c.g[V1][0] as u32).wrapping_add(15));
                            c.sdr(A2, (c.g[V1][0] as u32).wrapping_add(8));
                            c.sdl(A3, (c.g[V1][0] as u32).wrapping_add(23));
                            c.sdr(A3, (c.g[V1][0] as u32).wrapping_add(16));
                            c.sdl(T0, (c.g[V1][0] as u32).wrapping_add(31));
                            c.sdr(T0, (c.g[V1][0] as u32).wrapping_add(24));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
                            let again = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
                            if !again {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, unaligned_head);
                            }
                        }
                        c.cycles += 2;
                    }
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                    c.g[A2][0] = c.g[S1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x70), c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(method_offset));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    call_ptr(c, target, method_ret)?;
                    c.g[V0][0] = 0;
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_CloseStep` (0x3f8ff0): advance memory-card close, flush, or finish
/// callbacks after an I/O operation.
pub fn slogin_mc_close_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let valid_status = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        let mut direct_epilogue = false;
        if !valid_status {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
        } else {
            c.cycles += 2;
            let valid_data = c.g[A2][0] != 0;
            c.g[V0][0] = 0x12;
            if valid_data {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[A0][0] = 7;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let valid_phase = c.g[V1][0] == c.g[A0][0];
                c.g[V0][0] = 1;
                if !valid_phase {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x19;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                    let stage = c.g[V1][0];
                    c.g[V0][0] = (stage < 0x1a) as u64;
                    let mut unknown_stage = false;
                    if stage == 0x19 {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1a;
                        if c.g[A3][0] == 0 {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                            c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                            let target = c.g[V0][0] as u32;
                            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                            call_ptr(c, target, 0x003f_90d0)?;
                            c.g[V0][0] = 0;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[A3][0];
                            callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_90e0)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                            callee::SLOGIN_MC_ABORT_AT_FINISH.call(c, 0x003f_90f0)?;
                            frame.restore(c, S0, 0x10);
                            direct_epilogue = true;
                        }
                    } else {
                        c.cycles += 2;
                        let at_or_past_finish = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x18;
                        if !at_or_past_finish {
                            c.cycles += 2;
                            let at_close = stage == 0x18;
                            c.g[V1][0] = 2;
                            if at_close {
                                c.cycles += 2;
                                if c.g[A3][0] == 0 {
                                    c.cycles += 7;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[A1][0]);
                                    c.g[V1][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                    let target = c.g[V0][0] as u32;
                                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                    call_ptr(c, target, 0x003f_908c)?;
                                    c.g[V0][0] = 0;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[A3][0];
                                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_909c)?;
                                    c.g[A0][0] =
                                        c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                                    callee::SLOGIN_MC_ABORT_AT_CLOSE.call(c, 0x003f_90ac)?;
                                    frame.restore(c, S0, 0x10);
                                    direct_epilogue = true;
                                }
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1a;
                            let at_finish = stage == 0x1a;
                            c.g[V1][0] = 2;
                            if at_finish {
                                c.cycles += 2;
                                let has_error = c.g[A3][0] != 0;
                                c.g[V0][0] = c.g[A3][0];
                                if has_error {
                                    c.cycles += 2;
                                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = 2;
                                    c.g[V0][0] = 0;
                                    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                                }
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        }
                    }
                    if unknown_stage {
                        // An unknown stage marks the request finished (status word = 2).
                        c.cycles += 1;
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    }
                }
            }
        }
        if !direct_epilogue {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginMc_IoStep(int result, int* done, const int* reply)` (0x3f8ef8):
/// finish a seek, write or read request (card state 8, 9 or 10): set
/// `*done = 1`; the seek phase (0x1b) returns 0, and the write and read
/// phases (0x1c, 0x1d) return 0 when `*reply` equals the requested byte count
/// (card state +0x70), else 10. A non-zero `result` is stored with
/// `SLoginMc_SetError` and the request is aborted at the flush; otherwise
/// `*done = 2`. Returns 0x12 for a null pointer or another request, and for
/// an unknown phase (with `*done = 2`).
pub fn slogin_mc_io_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let valid_status = c.g[S0][0] != 0;
        frame.save_ra(c, 0);
        let mut restored_s0 = false;
        if !valid_status {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
        } else {
            c.cycles += 2;
            let valid_data = c.g[A2][0] != 0;
            c.g[V0][0] = 0x12;
            if valid_data {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                c.g[V1][0] = 8;
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                let phase = c.g[A0][0];
                c.g[V0][0] = 10;
                let valid_phase = if phase == 8 {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    true
                } else {
                    c.cycles += 2;
                    let is_ten = phase == 10;
                    c.g[V0][0] = 9;
                    if is_ten {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                        true
                    } else {
                        c.cycles += 2;
                        let is_nine = phase == 9;
                        c.g[V0][0] = 1;
                        is_nine
                    }
                };
                if !valid_phase {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = 0x1b;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                    let stage = c.g[V1][0];
                    c.g[V0][0] = (stage < 0x1b) as u64;
                    let mut has_error = false;
                    let mut unknown_stage = false;
                    if stage == 0x1b {
                        c.cycles += 2;
                        has_error = c.g[A3][0] != 0;
                        c.g[V1][0] = 2;
                        if !has_error {
                            c.cycles += 2;
                            c.g[V0][0] = 0;
                        }
                    } else {
                        c.cycles += 2;
                        if stage < 0x1b {
                            c.g[V1][0] = 2;
                            unknown_stage = true;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (stage < 0x1e) as u64;
                            c.g[V1][0] = 2;
                            if stage >= 0x1e {
                                unknown_stage = true;
                            } else {
                                c.cycles += 4;
                                has_error = c.g[A3][0] != 0;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                                if !has_error {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x70));
                                    let matching_count = c.g[V1][0] == c.g[A0][0];
                                    c.g[V1][0] = 2;
                                    c.cycles += 2;
                                    c.g[V0][0] = if matching_count { 0 } else { 10 };
                                }
                            }
                        }
                    }
                    if has_error {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[A3][0];
                        callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_8fbc)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                        callee::SLOGIN_MC_ABORT_AT_FLUSH.call(c, 0x003f_8fcc)?;
                        frame.restore(c, S0, 0x10);
                        restored_s0 = true;
                    } else {
                        if unknown_stage {
                            c.cycles += 1;
                            c.g[V0][0] = 0x12;
                        }
                        // The step is done either way: status word = 2.
                        c.cycles += 1;
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                    }
                }
            }
        }
        if !restored_s0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_AbortStep` (0x3f9130): advance a memory-card abort after a close
/// or flush callback completes.
pub fn slogin_mc_abort_step(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let mut direct_epilogue = false;
        let valid_status = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if !valid_status {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
        } else {
            c.cycles += 2;
            let valid_data = c.g[A2][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
            if !valid_data {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(11));
                let valid_phase = c.g[V0][0] < 3;
                c.g[V0][0] = 1;
                if !valid_phase {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                } else {
                    c.cycles += 6;
                    c.g[A2][0] = 0x19;
                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                    let stage = c.g[V1][0];
                    c.g[V0][0] = 0x1a;
                    let mut unknown_stage = false;
                    if stage == 0x19 {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                        c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                        let target = c.g[V0][0] as u32;
                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
                        call_ptr(c, target, 0x003f_91d8)?;
                        c.g[V0][0] = 0;
                    } else {
                        c.cycles += 3;
                        let below_finish = stage < 0x1a;
                        c.g[V0][0] = 0x18;
                        if below_finish {
                            c.cycles += 2;
                            c.g[V1][0] = 2;
                            if stage == 0x18 {
                                c.cycles += 7;
                                c.sw((c.g[A0][0] as u32).wrapping_add(4), c.g[A2][0]);
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                let target = c.g[V0][0] as u32;
                                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                                call_ptr(c, target, 0x003f_91bc)?;
                                c.g[V0][0] = 0;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x1a;
                            c.g[V1][0] = 2;
                            if stage == 0x1a {
                                c.cycles += 5;
                                c.g[V0][0] = 2;
                                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_91ec)?;
                                direct_epilogue = true;
                            } else {
                                c.cycles += 2;
                                c.g[V0][0] = 0x12;
                                unknown_stage = true;
                            }
                        }
                    }
                    if unknown_stage {
                        // An unknown stage marks the request finished (status word = 2).
                        c.cycles += 1;
                        c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                    }
                }
            }
        }
        if !direct_epilogue {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_IsInitialized` (0x3f9208): report whether the shared-login
/// memory-card interface has been installed.
pub fn slogin_mc_is_initialized(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    ret(c);
}

/// `SLoginMc_Shutdown` (0x3f9530): shut down the shared-login memory-card state
/// and release its two allocations.
pub fn slogin_mc_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_9540)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
        if initialized {
            c.cycles += 2;
            let iface = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
            if iface {
                c.cycles += 2;
                if c.g[V1][0] != 0 {
                    c.sw((c.g[V0][0] as u32).wrapping_add(4), 0);
                    c.cycles += 31;
                    c.g[A1][0] = c.g[V1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(8), 0);
                    c.g[A2][0] = card::STATE_BYTES;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0);
                    c.g[A3][0] = 4;
                    for off in (0x10..=0x44).step_by(4) {
                        c.sw((c.g[V0][0] as u32).wrapping_add(off), 0);
                    }
                    c.sw(c.g[V1][0] as u32, 0);
                    c.sw((c.g[V1][0] as u32).wrapping_add(4), 0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x428));
                    call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_95bc)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP));
                    c.g[A2][0] = card::IFACE_BYTES;
                    c.sw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP), 0);
                    c.g[A3][0] = 4;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                    call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_95d4)?;
                    c.sw((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP), 0);
                    c.g[V0][0] = 0;
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 2;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_GetFileState` (0x3f95e8): copy the memory-card file state into the
/// caller's output record.
pub fn slogin_mc_get_file_state(c: &mut Cpu) {
    c.cycles += 2;
    let valid_output = c.g[A0][0] != 0;
    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(card::STATE_GP));
    if !valid_output {
        c.cycles += 2;
        c.g[V0][0] = 12;
        ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = (c.g[V1][0] | c.g[A0][0]) & 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c));
    if aligned {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(c.g[V1][0] as u32);
            c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(8));
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(16));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(24));
            c.sd(c.g[A0][0] as u32, c.g[A1][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(8), c.g[A2][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(16), c.g[A3][0]);
            c.sd((c.g[A0][0] as u32).wrapping_add(24), c.g[T0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
            let again = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
            if !again {
                c.cycles += 3;
                c.g[V0][0] = 0;
                ret(c);
                return;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_9660);
            }
        }
    }
    loop {
        c.cycles += 19;
        c.ldl(A1, (c.g[V1][0] as u32).wrapping_add(7));
        c.ldr(A1, c.g[V1][0] as u32);
        c.ldl(A2, (c.g[V1][0] as u32).wrapping_add(15));
        c.ldr(A2, (c.g[V1][0] as u32).wrapping_add(8));
        c.ldl(A3, (c.g[V1][0] as u32).wrapping_add(23));
        c.ldr(A3, (c.g[V1][0] as u32).wrapping_add(16));
        c.ldl(T0, (c.g[V1][0] as u32).wrapping_add(31));
        c.ldr(T0, (c.g[V1][0] as u32).wrapping_add(24));
        c.sdl(A1, (c.g[A0][0] as u32).wrapping_add(7));
        c.sdr(A1, c.g[A0][0] as u32);
        c.sdl(A2, (c.g[A0][0] as u32).wrapping_add(15));
        c.sdr(A2, (c.g[A0][0] as u32).wrapping_add(8));
        c.sdl(A3, (c.g[A0][0] as u32).wrapping_add(23));
        c.sdr(A3, (c.g[A0][0] as u32).wrapping_add(16));
        c.sdl(T0, (c.g[A0][0] as u32).wrapping_add(31));
        c.sdr(T0, (c.g[A0][0] as u32).wrapping_add(24));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(32));
        let again = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(32));
        if !again {
            c.cycles += 2;
            c.g[V0][0] = 0;
            ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003f_960c);
        }
    }
}

/// `SLoginMc_IsAvailable_Wrapper` (0x3fa360): forward the memory-card
/// availability query to the card interface.
pub fn slogin_mc_is_available_wrapper(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Table(callee::SLOGIN_MC_IS_AVAILABLE.addr),
        0x003f_a370,
    );
}

/// `SLoginMc_CalcSaveSize` (0x3fa6a8): compute the memory-card space needed for
/// a shared-login save.
pub fn slogin_mc_calc_save_size(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Table(callee::SLOGIN_MC_CALC_SAVE_SIZE_IMPL.addr),
        0x003f_a6b8,
    );
}

/// `SLoginMc_GetPortSlot` (0x3fa1d0): resolve a shared-login memory-card device
/// index to its port and slot pair.
pub fn slogin_mc_get_port_slot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 12;
        } else {
            c.cycles += 4;
            callee::SLOGIN_MC_IS_INITIALIZED.call(c, 0x003f_a1fc)?;
            let initialized = c.g[V0][0] != 0;
            c.g[V0][0] = (c.g[S0][0] < 8) as u64;
            if !initialized {
                c.cycles += 2;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                let valid_device = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if valid_device {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    callee::SLOGIN_MC_SPLIT_DEVICE_INDEX.call(c, 0x003f_a21c)?;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 9;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The phase dispatch table of `SLoginMc_CreateStep`.
pub(crate) const CREATE_CALLBACK_TABLE: u32 = 0x0049_7430;

#[derive(Clone, Copy)]
pub(crate) enum CreateExit {
    Normal,
    InvalidPhase,
    InputError,
    BadResult,
    CloseCallback,
    FinishAbort,
    FlushAbort,
    CloseAbort,
    GetError,
    FinishDevice,
}

/// The shared epilogue of `SLoginMc_CreateStep`, with or without `s0`.
pub(crate) fn create_step_epilogue(c: &mut Cpu, frame: Frame, restore_s0: bool) {
    if restore_s0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
}

/// `SLoginMc_CreateStep` (0x3fa8f8): advance the shared-login memory-card
/// creation operation after a card RPC. `a0` is the RPC result, `a1` receives
/// completion status, and `a2` points to the RPC's returned word. The phase
/// word at +4 of the card state (`gp+0x1ae8`) selects the next directory,
/// capacity, file, icon, flush, close, or device-retry action.
pub fn slogin_mc_create_step(c: &mut Cpu) {
    guest(c, |c| {
        loop {
            c.cycles += 7;
            let frame = Frame::open(c, 0x80);
            c.g[A3][0] = c.g[A0][0];
            frame.save(c, S0, 0x70);
            frame.save(c, S1, 0x60);
            c.g[S0][0] = c.g[A1][0];
            frame.save_ra(c, 0x50);
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x12;
                create_step_epilogue(c, frame, true);
                return Ok(());
            }
            c.cycles += 2;
            c.g[V0][0] = 0x12;
            if c.g[A2][0] == 0 {
                create_step_epilogue(c, frame, true);
                return Ok(());
            }
            c.cycles += 5;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
            c.g[V1][0] = 1;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = 4;
            if c.g[A0][0] == c.g[V1][0] {
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                let valid = c.g[A0][0] == c.g[V0][0];
                c.g[V0][0] = 1;
                if !valid {
                    c.cycles += 2;
                    c.g[V0][0] = 0x12;
                    create_step_epilogue(c, frame, true);
                    return Ok(());
                }
            }

            c.cycles += 7;
            let store_at = c.g[S0][0] as u32;
            let store_value = c.g[V0][0];
            Addr(store_at).sw(c, store_value);
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(card::PHASE)).lw(c);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            c.g[V1][0] = (c.g[A0][0] < 26) as u64;
            c.g[V0][0] = DATA_PAGE;
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[V1][0] = 2;
                c.g[V0][0] = 0x12;
                let store_at = c.g[S0][0] as u32;
                let store_value = c.g[V1][0];
                Addr(store_at).sw(c, store_value);
                create_step_epilogue(c, frame, true);
                return Ok(());
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CREATE_CALLBACK_TABLE & 0xffff));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            let tail = match target {
                0x003f_a8f8 => {
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_a8f8);
                    }
                    continue;
                }
                // Start a directory lookup, or move to the next selected card.
                0x003f_a978 => {
                    c.cycles += 2;
                    c.g[V0][0] = 3;
                    if c.g[A3][0] == 0 {
                        c.cycles += 16;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(card::DIR_NAME));
                        callee::SLOGIN_MC_BUILD_DIR_PATH.call(c, 0x003f_a998)?;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                        c.g[V0][0] = 2;
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[SP][0];
                        let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
                        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_a9b8)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    } else {
                        c.cycles += 2;
                        let retry = c.g[A3][0] == c.g[V0][0];
                        c.g[V1][0] = 2;
                        if !retry {
                            CreateExit::InputError
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                            c.g[V0][0] = 1;
                            c.g[S1][0] = Addr(c.g[V1][0] as u32).lw(c);
                            if c.g[S1][0] != c.g[V0][0] {
                                c.g[V1][0] = 2;
                                c.cycles += 3;
                                c.g[V0][0] = 3;
                                let store_at = c.g[S0][0] as u32;
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                CreateExit::Normal
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                                c.g[A1][0] =
                                    Addr((c.g[V1][0] as u32).wrapping_add(card::SCAN_DIRECTION))
                                        .lw(c);
                                c.g[A0][0] = Addr(c.g[V0][0] as u32).lhu(c);
                                callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.call(c, 0x003f_a9ec)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[V0][0] = u64::MAX;
                                let exhausted = c.g[A0][0] == c.g[V0][0];
                                let store_at = (c.g[V1][0] as u32).wrapping_add(card::DEVICE);
                                let store_value = c.g[A0][0];
                                Addr(store_at).sw(c, store_value);
                                if exhausted {
                                    c.cycles += 1;
                                    c.g[V0][0] = 2;
                                    CreateExit::GetError
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] =
                                        Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP))
                                            .lw(c);
                                    let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                                    let store_value = c.g[S1][0];
                                    Addr(store_at).sw(c, store_value);
                                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lw(c);
                                    let callback = c.g[V1][0] as u32;
                                    call_ptr(c, callback, 0x003f_aa14)?;
                                    c.g[V0][0] = 0;
                                    CreateExit::Normal
                                }
                            }
                        }
                    }
                }
                // Check whether the directory exists.
                0x003f_aa28 => {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[V0][0] = 1;
                        if c.g[A1][0] == 0 {
                            c.cycles += 9;
                            c.g[V1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                            c.g[V0][0] = 3;
                            c.g[A1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                            let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                            let store_value = c.g[V0][0];
                            Addr(store_at).sw(c, store_value);
                            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(8)).lw(c);
                            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                            let callback = c.g[V0][0] as u32;
                            call_ptr(c, callback, 0x003f_aa58)?;
                            c.g[V0][0] = 0;
                            CreateExit::Normal
                        } else {
                            c.cycles += 2;
                            c.g[V1][0] = 2;
                            if c.g[A1][0] != c.g[V0][0] {
                                CreateExit::BadResult
                            } else {
                                c.cycles += 4;
                                c.g[A0][0] = 8;
                                callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_aa70)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                CreateExit::CloseCallback
                            }
                        }
                    }
                }
                // Check available space for the directory.
                0x003f_aa78 => {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        if (c.g[A1][0] as i64) < 0 {
                            c.cycles += 1;
                            c.g[V1][0] = 2;
                            CreateExit::BadResult
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x420)).lw(c);
                            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                            let short = c.g[V0][0] != 0;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                            if short {
                                c.cycles += 4;
                                c.g[A0][0] = 14;
                                callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_aac0)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                CreateExit::CloseCallback
                            } else {
                                c.cycles += 7;
                                c.g[V1][0] = 4;
                                let store_at = (c.g[V0][0] as u32).wrapping_add(card::PHASE);
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x0c)).lw(c);
                                c.g[A0][0] =
                                    Addr((c.g[V0][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                                let callback = c.g[V1][0] as u32;
                                call_ptr(c, callback, 0x003f_aab0)?;
                                c.g[V0][0] = 0;
                                CreateExit::Normal
                            }
                        }
                    }
                }
                // Check available space for the file, then construct paths.
                0x003f_aac8 => {
                    c.cycles += 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        if (c.g[A1][0] as i64) < 0 {
                            c.cycles += 1;
                            c.g[V1][0] = 2;
                            CreateExit::BadResult
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x424)).lw(c);
                            c.g[V0][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
                            let short = c.g[V0][0] != 0;
                            c.g[A1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                            if short {
                                c.cycles += 4;
                                c.g[A0][0] = 14;
                                callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_ab44)?;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                CreateExit::CloseCallback
                            } else {
                                c.cycles += 16;
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x50));
                                c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(card::DIR_NAME));
                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(card::FILE_PATH));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(card::DIR_PATH));
                                callee::SLOGIN_MC_BUILD_FILE_PATHS.call(c, 0x003f_ab10)?;
                                c.g[V0][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                c.g[V1][0] = 5;
                                c.g[A3][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                                let store_at = (c.g[V0][0] as u32).wrapping_add(card::PHASE);
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(card::FILE_PATH));
                                c.g[A2][0] =
                                    Addr((c.g[V0][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                                c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x20)).lw(c);
                                c.g[A1][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x420)).lw(c);
                                let callback = c.g[V0][0] as u32;
                                call_ptr(c, callback, 0x003f_ab34)?;
                                c.g[V0][0] = 0;
                                CreateExit::Normal
                            }
                        }
                    }
                }
                // Open the new data file.
                0x003f_ab4c => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = 14;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A3][0] = card::OPEN_CREATE_MODE;
                        let store_at = (c.g[V0][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[V1][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(card::FILE_PATH));
                        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
                        c.g[A1][0] = c.g[A0][0];
                        let callback = c.g[V1][0] as u32;
                        call_ptr(c, callback, 0x003f_ab78)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    }
                }
                // Read the file handle; a negative result aborts at flush.
                0x003f_ab80 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    if c.g[A3][0] != 0 {
                        CreateExit::FlushAbort
                    } else {
                        c.cycles += 3;
                        c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                        if (c.g[A2][0] as i64) < 0 {
                            c.cycles += 2;
                            c.g[A0][0] = 0;
                            CreateExit::FlushAbort
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = 16;
                            c.g[A1][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                            c.g[A0][0] = c.g[A2][0];
                            let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                            let store_value = c.g[V0][0];
                            Addr(store_at).sw(c, store_value);
                            let store_at = (c.g[V1][0] as u32).wrapping_add(card::FILE_HANDLE);
                            let store_value = c.g[A2][0];
                            Addr(store_at).sw(c, store_value);
                            c.cycles += 5;
                            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
                            let callback = c.g[V0][0] as u32;
                            call_ptr(c, callback, 0x003f_acd8)?;
                            c.g[V0][0] = 0;
                            CreateExit::Normal
                        }
                    }
                }
                // Close after the first file flush.
                0x003f_abb4 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::CloseAbort
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 17;
                        CreateExit::FinishDevice
                    }
                }
                // Open the icon file after closing the first one.
                0x003f_abc4 => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = 6;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A1][0] = DATA_PAGE;
                        let store_at = (c.g[A0][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A1][0] =
                            sx((c.g[A1][0] as u32).wrapping_add(card::VIEW_ICON_NAME & 0xffff));
                        CreateExit::FinishDevice
                    }
                }
                // Save the first icon handle and write its bytes.
                0x003f_abe4 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A2][0] = 7;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[A1][0];
                        let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[A2][0];
                        Addr(store_at).sw(c, store_value);
                        let store_at = (c.g[V1][0] as u32).wrapping_add(card::FILE_HANDLE);
                        let store_value = c.g[A1][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x54)).lw(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x30)).lw(c);
                        c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x50)).lw(c);
                        let callback = c.g[V1][0] as u32;
                        call_ptr(c, callback, 0x003f_ac14)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    }
                }
                // Flush the first icon file.
                0x003f_ac1c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    if c.g[A3][0] != 0 {
                        CreateExit::FlushAbort
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                        c.g[V0][0] = 8;
                        CreateExit::FinishDevice
                    }
                }
                // Close the first icon file.
                0x003f_ac30 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::CloseAbort
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 9;
                        CreateExit::FinishDevice
                    }
                }
                // Open the system icon file.
                0x003f_ac40 => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 10;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A1][0] = DATA_PAGE;
                        let store_at = (c.g[A0][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A1][0] =
                            sx((c.g[A1][0] as u32).wrapping_add(card::ICON_SYS_NAME & 0xffff));
                        CreateExit::FinishDevice
                    }
                }
                // Save the system icon handle and write its bytes.
                0x003f_ac78 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A3][0] = 11;
                        c.g[A2][0] = card::ICON_SYS_BYTES;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        c.g[A0][0] = c.g[A1][0];
                        let store_at = (c.g[V0][0] as u32).wrapping_add(card::FILE_HANDLE);
                        let store_value = c.g[A1][0];
                        Addr(store_at).sw(c, store_value);
                        let store_at = (c.g[V0][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[A3][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30)).lw(c);
                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x58));
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_aca8)?;
                        c.g[V0][0] = 0;
                        CreateExit::Normal
                    }
                }
                // Flush the system icon file.
                0x003f_acb0 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    if c.g[A3][0] != 0 {
                        CreateExit::FlushAbort
                    } else {
                        c.cycles += 2;
                        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                        c.g[V0][0] = 12;
                        CreateExit::FinishDevice
                    }
                }
                // Close the system icon file.
                0x003f_ad00 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::CloseAbort
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 13;
                        CreateExit::FinishDevice
                    }
                }
                // Finish the memory-card transaction.
                0x003f_ad48 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    if c.g[A3][0] != 0 {
                        CreateExit::FinishAbort
                    } else {
                        CreateExit::CloseCallback
                    }
                }
                // Report completion or retry on another selected card.
                0x003f_ad90 => {
                    c.cycles += 2;
                    c.g[V1][0] = 2;
                    if c.g[A3][0] != 0 {
                        CreateExit::InputError
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                        c.g[A0][0] = 4;
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        let mode_four = c.g[V1][0] == c.g[A0][0];
                        c.g[V0][0] = 2;
                        if mode_four {
                            CreateExit::GetError
                        } else {
                            c.cycles += 4;
                            callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_adb4)?;
                            let failed = c.g[V0][0] != 0;
                            c.g[V0][0] =
                                Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                            if !failed {
                                c.cycles += 4;
                                c.g[V1][0] = 2;
                                c.g[V0][0] = 0;
                                let store_at = c.g[S0][0] as u32;
                                let store_value = c.g[V1][0];
                                Addr(store_at).sw(c, store_value);
                                CreateExit::Normal
                            } else {
                                c.cycles += 9;
                                c.g[V1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                c.g[A0][0] = Addr(c.g[V0][0] as u32).lhu(c);
                                c.g[A1][0] =
                                    Addr((c.g[V1][0] as u32).wrapping_add(card::SCAN_DIRECTION))
                                        .lw(c);
                                callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.call(c, 0x003f_addc)?;
                                c.g[A1][0] =
                                    Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[V0][0] = u64::MAX;
                                let exhausted = c.g[A0][0] == c.g[V0][0];
                                let store_at = (c.g[A1][0] as u32).wrapping_add(card::DEVICE);
                                let store_value = c.g[A0][0];
                                Addr(store_at).sw(c, store_value);
                                if exhausted {
                                    c.cycles += 1;
                                    c.g[V0][0] = 2;
                                    CreateExit::GetError
                                } else {
                                    c.cycles += 8;
                                    c.g[V1][0] =
                                        Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP))
                                            .lw(c);
                                    c.g[V0][0] = 1;
                                    let store_at = (c.g[A1][0] as u32).wrapping_add(card::PHASE);
                                    let store_value = c.g[V0][0];
                                    Addr(store_at).sw(c, store_value);
                                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x10)).lw(c);
                                    let callback = c.g[V0][0] as u32;
                                    call_ptr(c, callback, 0x003f_ae08)?;
                                    c.g[V0][0] = 0;
                                    CreateExit::Normal
                                }
                            }
                        }
                    }
                }
                0x003f_ae30 => CreateExit::InvalidPhase,
                _ => {
                    c.jump_out(target, 0x003f_a8f8, 0x003f_ae50);
                    return Ok(());
                }
            };

            let mut restore_s0 = true;
            match tail {
                CreateExit::Normal => {}
                CreateExit::InvalidPhase => {
                    c.cycles += 3;
                    c.g[V1][0] = 2;
                    c.g[V0][0] = 0x12;
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V1][0];
                    Addr(store_at).sw(c, store_value);
                }
                CreateExit::InputError => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A3][0];
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V1][0];
                    Addr(store_at).sw(c, store_value);
                }
                CreateExit::BadResult => {
                    c.cycles += 3;
                    c.g[V0][0] = 10;
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V1][0];
                    Addr(store_at).sw(c, store_value);
                }
                CreateExit::CloseCallback => {
                    c.cycles += 8;
                    c.g[V0][0] = 26;
                    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                    let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                    let store_value = c.g[V0][0];
                    Addr(store_at).sw(c, store_value);
                    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x14)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                    let callback = c.g[V0][0] as u32;
                    call_ptr(c, callback, 0x003f_ad68)?;
                    c.g[V0][0] = 0;
                }
                CreateExit::FinishAbort => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A3][0];
                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_ad78)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                    callee::SLOGIN_MC_ABORT_AT_FINISH.call(c, 0x003f_ad88)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::FlushAbort => {
                    c.cycles += 8;
                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_ace8)?;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(card::FILE_HANDLE));
                    callee::SLOGIN_MC_ABORT_AT_FLUSH.call(c, 0x003f_acf8)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::CloseAbort => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A3][0];
                    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_ad30)?;
                    c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::STATE_GP)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(card::FILE_HANDLE));
                    callee::SLOGIN_MC_ABORT_AT_CLOSE.call(c, 0x003f_ad40)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::GetError => {
                    c.cycles += 4;
                    let store_at = c.g[S0][0] as u32;
                    let store_value = c.g[V0][0];
                    Addr(store_at).sw(c, store_value);
                    callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_ae1c)?;
                    frame.restore(c, S0, 0x70);
                    restore_s0 = false;
                }
                CreateExit::FinishDevice => {
                    // Three shared continuations: icon open, handle close, and flush.
                    if target == 0x003f_abc4 || target == 0x003f_ac40 {
                        c.cycles += 7;
                        c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(card::DEVICE)).lw(c);
                        c.g[A3][0] = card::OPEN_CREATE_MODE;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(card::FILE_PATH));
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_ac70)?;
                    } else if target == 0x003f_abb4
                        || target == 0x003f_ac30
                        || target == 0x003f_ad00
                    {
                        c.cycles += 7;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x1c)).lw(c);
                        c.g[A0][0] =
                            Addr((c.g[V1][0] as u32).wrapping_add(card::FILE_HANDLE)).lw(c);
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_ad20)?;
                    } else {
                        // The flush callback takes the saved file handle.
                        c.cycles += 3;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(card::IFACE_GP)).lw(c);
                        let store_at = (c.g[V1][0] as u32).wrapping_add(card::PHASE);
                        let store_value = c.g[V0][0];
                        Addr(store_at).sw(c, store_value);
                        c.g[A0][0] =
                            Addr((c.g[V1][0] as u32).wrapping_add(card::FILE_HANDLE)).lw(c);
                        c.cycles += 5;
                        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x38)).lw(c);
                        let callback = c.g[V0][0] as u32;
                        call_ptr(c, callback, 0x003f_acd8)?;
                    }
                    c.g[V0][0] = 0;
                }
            }
            create_step_epilogue(c, frame, restore_s0);
            return Ok(());
        }
    })
}

/// `const ChecksumIface* SLoginMc_GetChecksumIface(const ChecksumIface** out)`
/// (0x3fa6c8): forward to `SLoginMc_GetChecksumIfaceImpl`.
pub fn slogin_mc_get_checksum_iface(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Table(callee::SLOGIN_MC_GET_CHECKSUM_IFACE_IMPL.addr),
        0x003f_a6d8,
    );
}

/// `int SLoginMc_GetDataOffset(void)` (0x3fa6e8): forward to
/// `SLoginMc_GetDataOffsetImpl` (the offset of the login data in the save).
pub fn slogin_mc_get_data_offset(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Table(callee::SLOGIN_MC_GET_DATA_OFFSET_IMPL.addr),
        0x003f_a6f8,
    );
}

/// The device-index jump table (8 words in the game's data).
pub(crate) const SPLIT_DEVICE_INDEX_TABLE: u32 = 0x0049_73e0;

/// The `port` byte at `out[0]` and `slot` byte at `out[1]`, written in the
/// order the arm writes them (`slot_first`).
pub(crate) fn split_device_index_store(c: &mut Cpu, slot_first: bool, port: u64, slot: u64) {
    let out = c.g[A1][0] as u32;
    if slot_first {
        c.sb(out.wrapping_add(1), slot);
        c.sb(out, port);
    } else {
        c.sb(out, port);
        c.sb(out.wrapping_add(1), slot);
    }
}

/// `void SLoginMc_SplitDeviceIndex(uint device, u8 out[2])` (0x3fa708): split
/// a memory-card device index 0..7 into port (`device / 4`, at `out[0]`) and
/// slot (`device % 4`, at `out[1]`) through the game's 8-entry jump table;
/// larger indices leave `out` alone.
pub fn slogin_mc_split_device_index(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 8) as u64;
    let in_range = c.g[V0][0] != 0;
    c.g[V0][0] = DATA_PAGE;
    if !in_range {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SPLIT_DEVICE_INDEX_TABLE & 0xffff));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    match target {
        // Port 0, slot 0.
        0x003f_a72c => {
            c.cycles += 3;
            split_device_index_store(c, true, 0, 0);
        }
        // Port 0, slots 1..3 (shared tail at 0x3fa73c).
        0x003f_a738 | 0x003f_a748 | 0x003f_a750 => {
            let (cycles, slot) = match target {
                0x003f_a738 => (1, 1),
                0x003f_a748 => (2, 2),
                _ => (2, 3),
            };
            c.cycles += cycles;
            c.g[V0][0] = slot;
            c.cycles += 3;
            split_device_index_store(c, false, 0, slot);
        }
        // Port 1, slot 0.
        0x003f_a758 => {
            c.cycles += 4;
            c.g[V0][0] = 1;
            split_device_index_store(c, true, 1, 0);
        }
        // Port 1, slot 1.
        0x003f_a768 => {
            c.cycles += 4;
            c.g[V0][0] = 1;
            split_device_index_store(c, true, 1, 1);
        }
        // Port 1, slots 2..3 (shared tail at 0x3fa78c).
        0x003f_a778 | 0x003f_a784 => {
            let (cycles, slot) = if target == 0x003f_a778 {
                (3, 2)
            } else {
                (2, 3)
            };
            c.cycles += cycles;
            c.g[V1][0] = 1;
            c.g[V0][0] = slot;
            c.cycles += 2;
            split_device_index_store(c, true, 1, slot);
            c.cycles += 2;
        }
        _ => {
            c.jump_out(target, 0x003f_a708, 0x003f_a79c);
            return;
        }
    }
    ret(c);
}

/// The path lead byte (a global byte at 0x4a4be0).
pub(crate) const BUILD_DIR_PATH_LEAD: u32 = 0x004a_4be0;

/// Bytes each path buffer is cleared to before it is built.
pub(crate) const SLOGIN_MC_PATH_BYTES: u64 = 0x20;

/// `char* SLoginMc_BuildDirPath(char* out, int, const char* name)` (0x3fa7a0):
/// clear 32 bytes of `out`, write the path lead byte (global at 0x4a4be0)
/// followed by `name`, and return the end of the copied name.
pub fn slogin_mc_build_dir_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = SLOGIN_MC_PATH_BYTES;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        call_ptr(c, callee::MEMSET.addr, 0x003f_a7cc)?;
        c.g[V1][0] = GLOBALS_PAGE;
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = c.lbu(BUILD_DIR_PATH_LEAD);
        c.sb(c.g[S0][0] as u32, c.g[V0][0]);
        call_ptr(c, callee::STRLEN.addr, 0x003f_a7e0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, callee::MEMCPY.addr, 0x003f_a7f0)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a7f8)?;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The 9-byte file-path prefix (the game's product code) in the game's data.
pub(crate) const BUILD_FILE_PATHS_PREFIX: u32 = 0x0049_7400;

pub(crate) const BUILD_FILE_PATHS_PREFIX_BYTES: u32 = 9;

/// Write the 9-byte file-path prefix at `dst` (8 unaligned bytes, then the
/// 9th byte through `v1`), addressed from the `lui` half kept in `s3`.
pub(crate) fn build_file_paths_prefix(c: &mut Cpu, dst: u32) {
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(BUILD_FILE_PATHS_PREFIX & 0xffff));
    let src = c.g[A1][0] as u32;
    c.ldl(V0, src.wrapping_add(7));
    c.ldr(V0, src);
    c.g[V1][0] = c.lb(src.wrapping_add(8));
    c.sdl(V0, dst.wrapping_add(7));
    c.sdr(V0, dst);
    c.sb(dst.wrapping_add(8), c.g[V1][0]);
}

/// `int SLoginMc_BuildFilePaths(char* path_a, char* path_b, int,
/// const char* name)` (0x3fa818): clear 32 bytes of each buffer and build
/// the product-code prefix + `name` in both; return `strlen(name)`.
pub fn slogin_mc_build_file_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 56;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = SLOGIN_MC_PATH_BYTES;
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[S3][0] = sx(BUILD_FILE_PATHS_PREFIX & 0xffff_0000);
        call_ptr(c, callee::MEMSET.addr, 0x003f_a84c)?;
        let path_a = c.g[S0][0] as u32;
        build_file_paths_prefix(c, path_a);
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(BUILD_FILE_PATHS_PREFIX_BYTES));
        call_ptr(c, callee::STRLEN.addr, 0x003f_a874)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(BUILD_FILE_PATHS_PREFIX_BYTES));
        call_ptr(c, callee::MEMCPY.addr, 0x003f_a888)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a890)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = SLOGIN_MC_PATH_BYTES;
        call_ptr(c, callee::MEMSET.addr, 0x003f_a8a0)?;
        let path_b = c.g[S2][0] as u32;
        build_file_paths_prefix(c, path_b);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a8c4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::MEMCPY.addr, 0x003f_a8d4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRLEN.addr, 0x003f_a8dc)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool SLoginMc_IsAvailable(void)` (0x3fb278): the card back end is always
/// available (returns 1).
pub fn slogin_mc_is_available(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 1;
    ret(c);
}

/// Bytes the save reserves on top of the two rounded payloads.
pub(crate) const CALC_SAVE_SIZE_OVERHEAD: u32 = 0x1400;

/// The two payload sizes it rounds (words at +0x3cc and +0x4).
pub(crate) const CALC_SAVE_SIZE_FIRST: u32 = 0x3cc;

pub(crate) const CALC_SAVE_SIZE_SECOND: u32 = 0x4;

/// `void SLoginMc_CalcSaveSizeImpl(const void* icon, uint* bytes,
/// uint* files)` (0x3fb280): round the words at +0x3cc and +0x4 up to whole
/// KiB, add 5 KiB, store that size in `*bytes` and 3 in `*files`.
pub fn slogin_mc_calc_save_size_impl(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(CALC_SAVE_SIZE_FIRST));
    c.g[V0][0] = c.g[V1][0] & 0x3ff;
    let first_whole = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 10);
    if !first_whole {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    }
    c.cycles += 5;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 10);
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(CALC_SAVE_SIZE_SECOND));
    c.g[V0][0] = c.g[A0][0] & 0x3ff;
    let second_whole = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 10);
    if !second_whole {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    }
    c.cycles += 7;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 10);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CALC_SAVE_SIZE_OVERHEAD));
    c.g[V1][0] = 3;
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
    ret(c);
}

/// `void SLoginMc_GetChecksumIfaceImpl(const ChecksumIface** out)`
/// (0x3fb2c8): store the CRC-32 interface in `*out` when `out` is non-null.
pub fn slogin_mc_get_checksum_iface_impl(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CRC32_GET_INTERFACE.call(c, 0x003f_b2dc)?;
        if c.g[S0][0] != 0 {
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SLoginMc_SetBusy(...)` (0x3fb2f8): does nothing on this back end.
pub fn slogin_mc_set_busy(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_GetDataOffsetImpl(void)` (0x3fb300): the login data starts
/// at offset 0.
pub fn slogin_mc_get_data_offset_impl(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// `int SLoginMc_ValidateIconSys(const void* icon_sys)` (0x3fb308): 0 when an
/// icon is given, else error 12.
pub fn slogin_mc_validate_icon_sys(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 12;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = 0;
    }
    ret(c);
}

/// The reflected CRC-32 polynomial (sign-extended as `lui`/`ori` leave it).
pub(crate) const CRC32_POLYNOMIAL: u32 = 0xedb8_8320;

/// `uint Crc32_TableEntry(uint n)` (0x3fb318): the CRC-32 table entry for
/// byte `n`: eight shift-right steps, xoring the reflected polynomial
/// 0xedb88320 whenever the bit shifted out is set.
pub fn crc32_table_entry(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = sx(CRC32_POLYNOMIAL);
    c.g[A1][0] = 8;
    c.g[V0][0] = c.g[A0][0] & 1;
    loop {
        c.cycles += 2;
        let low_clear = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 1);
        if low_clear {
            c.cycles += 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0] ^ c.g[V1][0];
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = c.g[V0][0] & 0xff;
        let more = c.g[A1][0] != 0;
        c.g[V0][0] = c.g[A0][0] & 1;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003f_b328);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// Entries in the CRC-32 table.
pub(crate) const CRC32_TABLE_ENTRIES: u64 = 0x100;

/// `void Crc32_BuildTable(uint table[256])` (0x3fb358): fill the 256-word
/// CRC-32 lookup table with `Crc32_TableEntry(0..255)`.
pub fn crc32_build_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            callee::CRC32_TABLE_ENTRY.call(c, 0x003f_b378)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = (c.g[S0][0] < CRC32_TABLE_ENTRIES) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_b370);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The CRC-32 service state: `{ uint* table; int +0x4; int +0x8;
/// uint +0xc; void* heap +0x10 }`.
pub(crate) const CRC32_STATE: u32 = 0x0045_1518;

/// `lui` half of [`CRC32_STATE`] (kept in `s0`) and its low half.
pub(crate) const CRC32_STATE_HI: u32 = CRC32_STATE & 0xffff_0000;

pub(crate) const CRC32_STATE_LO: u32 = CRC32_STATE & 0xffff;

pub(crate) const CRC32_STATE_TABLE: u32 = 0x0;

pub(crate) const CRC32_STATE_WORD_4: u32 = 0x4;

pub(crate) const CRC32_STATE_WORD_8: u32 = 0x8;

pub(crate) const CRC32_STATE_WORD_C: u32 = 0xc;

pub(crate) const CRC32_STATE_HEAP: u32 = 0x10;

/// Size and alignment of the table allocation.
pub(crate) const CRC32_TABLE_BYTES: u64 = 0x400;

pub(crate) const CRC32_TABLE_ALIGN: u64 = 4;

/// `Crc32_Init` results: already running, out of memory.
pub(crate) const CRC32_ERR_ALREADY_INIT: u64 = 5;

pub(crate) const CRC32_ERR_NO_MEMORY: u64 = 1;

/// `Crc32_Shutdown` result when not running.
pub(crate) const CRC32_ERR_NOT_INIT: u64 = 6;

/// `int Crc32_Init(void* heap)` (0x3fb3a0): allocate the 1 KiB CRC-32 table
/// from `heap` and build it, setting the state words (+0x4 = 4, +0x8 = 0,
/// +0xc = 0xffffffff, +0x10 = heap). Returns 0, 5 if already initialised, or
/// 1 if the allocation fails.
pub fn crc32_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = sx(CRC32_STATE_HI);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO + CRC32_STATE_TABLE));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO));
        let running = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if running {
            c.cycles += 2;
            c.g[V0][0] = CRC32_ERR_ALREADY_INIT;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = CRC32_TABLE_BYTES;
            c.g[A2][0] = CRC32_TABLE_ALIGN;
            call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_b3e0)?;
            let failed = c.g[V0][0] == 0;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO + CRC32_STATE_TABLE),
                c.g[V0][0],
            );
            if failed {
                c.cycles += 1;
                c.g[V0][0] = CRC32_ERR_NO_MEMORY;
            } else {
                c.cycles += 11;
                c.g[A0][0] = c.g[V0][0];
                c.sw(
                    (c.g[S2][0] as u32).wrapping_add(CRC32_STATE_HEAP),
                    c.g[S1][0],
                );
                callee::CRC32_BUILD_TABLE.call(c, 0x003f_b3f4)?;
                c.g[V0][0] = u64::MAX;
                c.g[V1][0] = 4;
                c.sw(
                    (c.g[S2][0] as u32).wrapping_add(CRC32_STATE_WORD_4),
                    c.g[V1][0],
                );
                c.sw(
                    (c.g[S2][0] as u32).wrapping_add(CRC32_STATE_WORD_C),
                    c.g[V0][0],
                );
                c.g[V0][0] = 0;
                c.sw((c.g[S2][0] as u32).wrapping_add(CRC32_STATE_WORD_8), 0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int Crc32_Shutdown(void)` (0x3fb430): free the CRC-32 table back to the
/// heap it came from and clear the table pointer; 6 if not initialised.
pub fn crc32_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = sx(CRC32_STATE_HI);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO + CRC32_STATE_TABLE));
        let running = c.g[A1][0] != 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO));
        if running {
            c.cycles += 7;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(CRC32_STATE_HEAP));
            c.g[A2][0] = CRC32_TABLE_BYTES;
            c.g[A3][0] = CRC32_TABLE_ALIGN;
            call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_b45c)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(CRC32_STATE_LO + CRC32_STATE_TABLE),
                0,
            );
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            c.g[V0][0] = CRC32_ERR_NOT_INIT;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The CRC-32 checksum interface (a table of method pointers in the game's
/// data: Init, Shutdown, ...).
pub(crate) const CRC32_INTERFACE: u32 = 0x0049_7508;

/// `const ChecksumIface* Crc32_GetInterface(void)` (0x3fb5e8): the CRC-32
/// checksum interface at 0x497508.
pub fn crc32_get_interface(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = DATA_PAGE;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(CRC32_INTERFACE & 0xffff));
    ret(c);
}

/// The login store's cipher interface (method pointers in the game's data).
pub(crate) const SLOGIN_STORE_CIPHER_INTERFACE: u32 = 0x0049_7528;

/// `const CipherIface* SLoginStore_GetCipherIface(void)` (0x3fb800): the
/// cipher interface at 0x497528.
pub fn slogin_store_get_cipher_iface(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = DATA_PAGE;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(SLOGIN_STORE_CIPHER_INTERFACE & 0xffff));
    ret(c);
}

/// Shared shape of the store's error mappers: 0 stays 0, anything else
/// moves up by `base`.
pub(crate) fn slogin_store_map_error(c: &mut Cpu, base: u32) {
    c.cycles += 2;
    let error = c.g[A0][0];
    c.g[V0][0] = 0;
    if error != 0 {
        c.cycles += 1;
        c.g[V0][0] = sx((error as u32).wrapping_add(base));
    }
    c.cycles += 2;
    ret(c);
}

/// Store error range base of memory-card errors.
pub(crate) const SLOGIN_STORE_MC_ERROR_BASE: u32 = 100;

/// Store error range base of cipher errors.
pub(crate) const SLOGIN_STORE_CIPHER_ERROR_BASE: u32 = 200;

/// Store error range base of checksum errors.
pub(crate) const SLOGIN_STORE_HASH_ERROR_BASE: u32 = 300;

/// `int SLoginStore_MapMcError(int mc_error)` (0x3fb810): translate a
/// memory-card error into the login store's error range (`+ 100`, 0 stays 0).
pub fn slogin_store_map_mc_error(c: &mut Cpu) {
    slogin_store_map_error(c, SLOGIN_STORE_MC_ERROR_BASE);
}

/// `int SLoginStore_MapCipherError(int cipher_error)` (0x3fb828): translate
/// a cipher error into the login store's error range (`+ 200`, 0 stays 0).
pub fn slogin_store_map_cipher_error(c: &mut Cpu) {
    slogin_store_map_error(c, SLOGIN_STORE_CIPHER_ERROR_BASE);
}

/// `int SLoginStore_MapHashError(int hash_error)` (0x3fb840): translate a
/// checksum service error into the login store's error range (`+ 300`, 0
/// stays 0).
pub fn slogin_store_map_hash_error(c: &mut Cpu) {
    slogin_store_map_error(c, SLOGIN_STORE_HASH_ERROR_BASE);
}

/// `void SLoginStore_GetAlignment(int format, int* first, int* unit)`
/// (0x3fb858): write 4 to `*first` and 1 to `*unit`. Formats 0..4 go through
/// the game's table (all five rows take the same arm) and others take the
/// defaults, so every format gives (4, 1). `SLoginStore_CalcAllocSize`
/// rounds by `*unit`.
pub fn slogin_store_get_alignment(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = 4;
    c.g[V0][0] = (c.g[A0][0] < 5) as u64;
    let valid_format = c.g[V0][0] != 0;
    c.g[V1][0] = 1;
    if valid_format {
        c.cycles += 7;
        c.g[V0][0] = DATA_PAGE;
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7580));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        if target == 0x003f_b884 {
            c.cycles += 2;
            c.g[A3][0] = 4;
            c.g[V1][0] = 1;
        } else {
            c.jump_out(target, 0x003f_b858, 0x003f_b898);
            return;
        }
    }
    c.cycles += 3;
    c.sw(c.g[A1][0] as u32, c.g[A3][0]);
    c.sw(c.g[A2][0] as u32, c.g[V1][0]);
    ret(c);
}

/// `uint SLoginStore_CalcAllocSize(uint payload_bytes, int format)`
/// (0x3fb898): include the 12-byte block header and round up to a multiple
/// of the second word `SLoginStore_GetAlignment` gives for `format` (1 for
/// every format).
pub fn slogin_store_calc_alloc_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 1;
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.g[A1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(4), c.g[V1][0]);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 4;
        callee::SLOGIN_STORE_GET_ALIGNMENT.call(c, 0x003f_b8c8)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(4));
        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(12));
        c.g[V0][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        } else {
            c.cycles += 3;
            c.divu(0, 3, 4);
            c.cycles += 6;
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.divu(0, 3, 4);
            c.g[V0][0] = c.hi[0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(12));
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const BLOCK_HEADER_BYTES: u32 = 12;

pub(crate) const BLOCK_HEADER_MAGIC: u32 = 0x0049_7550;

/// `BlockHeader* SLoginStore_GetBlockHeader(void* payload, bool validate)`
/// (0x3fb910): return the 12-byte header before a record payload, rejecting
/// it when requested and its magic bytes differ from the store template.
pub fn slogin_store_get_block_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] &= 0xff;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let validate = c.g[A1][0] != 0;
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_sub(BLOCK_HEADER_BYTES));
        if validate {
            c.cycles += 6;
            c.g[A1][0] = DATA_PAGE;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(BLOCK_HEADER_MAGIC & 0xffff));
            c.g[A2][0] = BLOCK_HEADER_BYTES as u64;
            call_ptr(c, callee::MEMCMP.addr, 0x003f_b93c)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0;
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void SLoginStore_SetRecordHeader(RecordHeader* header, uint kind,
/// uint size, const uint* crc)` (0x3fb958): write the kind, the size and the
/// CRC word into the 12-byte record header with the guest's unaligned copies.
pub fn slogin_store_set_record_header(c: &mut Cpu) {
    c.cycles += 17;
    let frame = Frame::open(c, 0x10);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(4), c.g[A2][0]);
    c.lwl(V0, (c.g[SP][0] as u32).wrapping_add(3));
    c.lwr(V0, c.g[SP][0] as u32);
    c.swl(V0, (c.g[A0][0] as u32).wrapping_add(3));
    c.swr(V0, c.g[A0][0] as u32);
    c.lwl(V0, (c.g[SP][0] as u32).wrapping_add(7));
    c.lwr(V0, (c.g[SP][0] as u32).wrapping_add(4));
    c.swl(V0, (c.g[A0][0] as u32).wrapping_add(7));
    c.swr(V0, (c.g[A0][0] as u32).wrapping_add(4));
    c.lwl(V0, (c.g[A3][0] as u32).wrapping_add(3));
    c.lwr(V0, c.g[A3][0] as u32);
    c.swl(V0, (c.g[A0][0] as u32).wrapping_add(11));
    c.swr(V0, (c.g[A0][0] as u32).wrapping_add(8));
    frame.close(c);
    ret(c);
}

/// `void SLoginStore_WriteSentinel(void* block)` (0x3fb9a0): write the
/// store's 12-byte sentinel pattern at the start of an allocated block.
pub fn slogin_store_write_sentinel(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = DATA_PAGE;
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(BLOCK_HEADER_MAGIC & 0xffff));
    c.ldl(V1, (c.g[A3][0] as u32).wrapping_add(7));
    c.ldr(V1, c.g[A3][0] as u32);
    c.lwl(A1, (c.g[A3][0] as u32).wrapping_add(11));
    c.lwr(A1, (c.g[A3][0] as u32).wrapping_add(8));
    c.sdl(V1, (c.g[A0][0] as u32).wrapping_add(7));
    c.sdr(V1, c.g[A0][0] as u32);
    c.swl(A1, (c.g[A0][0] as u32).wrapping_add(11));
    c.swr(A1, (c.g[A0][0] as u32).wrapping_add(8));
    ret(c);
}

/// `void SLoginStore_GetRecordHeader(const RecordHeader* header, uint* kind,
/// uint* size, uint* crc)` (0x3fb9d0): copy each requested record-header
/// word (kind, size, CRC) to its optional output pointer.
pub fn slogin_store_get_record_header(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        c.cycles += 4;
        c.lwl(V0, (c.g[A0][0] as u32).wrapping_add(3));
        c.lwr(V0, c.g[A0][0] as u32);
        c.swl(V0, (c.g[A1][0] as u32).wrapping_add(3));
        c.swr(V0, c.g[A1][0] as u32);
    }
    c.cycles += 2;
    if c.g[A2][0] != 0 {
        c.cycles += 4;
        c.lwl(V0, (c.g[A0][0] as u32).wrapping_add(7));
        c.lwr(V0, (c.g[A0][0] as u32).wrapping_add(4));
        c.swl(V0, (c.g[A2][0] as u32).wrapping_add(3));
        c.swr(V0, c.g[A2][0] as u32);
    }
    c.cycles += 2;
    if c.g[A3][0] != 0 {
        c.cycles += 4;
        c.lwl(T1, (c.g[A0][0] as u32).wrapping_add(11));
        c.lwr(T1, (c.g[A0][0] as u32).wrapping_add(8));
        c.swl(T1, (c.g[A3][0] as u32).wrapping_add(3));
        c.swr(T1, c.g[A3][0] as u32);
    }
    c.cycles += 2;
    ret(c);
}

/// `Record* SLoginStore_FindRecord(uint kind, uint occurrence)` (0x3fba20):
/// scan the store directory for the requested occurrence of a record kind.
pub fn slogin_store_find_record(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(store::RECORD_COUNT));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(store::RECORDS));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
    let has_records = c.g[V0][0] != 0;
    c.g[A3][0] = 0;
    if has_records {
        c.cycles += 2;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        loop {
            c.cycles += 2;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(store::RECORD_STRIDE));
            } else {
                c.cycles += 2;
                let other_occurrence = c.g[A1][0] != c.g[A3][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                if other_occurrence {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(store::RECORD_STRIDE));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V1][0];
                    ret(c);
                    return;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < c.g[A2][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003f_ba48);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

/// Directory entry words (`store::RECORD_STRIDE` bytes each).
pub(crate) const RECORD_OFFSET: u32 = 4;

pub(crate) const RECORD_SIZE: u32 = 8;

pub(crate) const RECORD_CRC: u32 = 0xc;

/// Checksum interface slots (`Crc32_GetInterface`'s table).
pub(crate) const HASH_BEGIN: u32 = 0xc;

pub(crate) const HASH_UPDATE: u32 = 0x10;

pub(crate) const HASH_FINAL: u32 = 0x14;

/// Cipher interface slots (`SLoginStore_GetCipherIface`'s table).
pub(crate) const CIPHER_SET_KEY: u32 = 0xc;

pub(crate) const CIPHER_ENCRYPT: u32 = 0x10;

pub(crate) const CIPHER_DECRYPT: u32 = 0x14;

pub(crate) const CIPHER_RESET: u32 = 0x18;

/// `void SLoginStore_SetRecordCrc(uint kind, uint occurrence, const uint* crc)`
/// (0x3fba78): store `*crc` as the CRC word of the requested occurrence of a
/// record kind; the whole directory is scanned either way.
pub fn slogin_store_set_record_crc(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(store::RECORD_COUNT));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(store::RECORDS));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
    let has_records = c.g[V0][0] != 0;
    c.g[T0][0] = 0;
    if has_records {
        c.cycles += 2;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        loop {
            c.cycles += 2;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(store::RECORD_STRIDE));
            } else {
                c.cycles += 2;
                let other_occurrence = c.g[A1][0] != c.g[T0][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                if !other_occurrence {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.sw((c.g[V1][0] as u32).wrapping_add(RECORD_CRC), c.g[V0][0]);
                }
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(store::RECORD_STRIDE));
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[V1][0] < c.g[A3][0]) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003f_baa0);
            }
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `uint SLoginStore_AddRecord(uint kind, uint size, const uint* crc)`
/// (0x3fbad0): append a directory entry (kind, data offset, size, CRC) after
/// the last one, advance the store's data end by the record's 12-byte header
/// plus its size, and return the new entry's data offset. A full directory
/// returns the current data end instead.
pub fn slogin_store_add_record(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(store::RECORD_COUNT));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(store::RECORD_CAPACITY));
    let has_room = c.g[V1][0] != c.g[V0][0];
    c.g[T0][0] = c.g[A1][0];
    if !has_room {
        c.cycles += 2;
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(store::DATA_END));
        ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(store::RECORDS));
    c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
    let first = c.g[V1][0] == 0;
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    if first {
        c.cycles += 1;
        c.sw((c.g[A1][0] as u32).wrapping_add(RECORD_OFFSET), 0);
    }
    c.cycles += 17;
    c.g[A3][0] = c.g[V0][0];
    c.g[V1][0] = 1;
    c.sw(c.g[A3][0] as u32, c.g[A0][0]);
    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(BLOCK_HEADER_BYTES));
    c.sw((c.g[A3][0] as u32).wrapping_add(RECORD_SIZE), c.g[T0][0]);
    if c.g[A1][0] == 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
    c.sw((c.g[A3][0] as u32).wrapping_add(RECORD_CRC), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORD_COUNT));
    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORD_CAPACITY));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.sw(
        (c.g[V1][0] as u32).wrapping_add(store::RECORD_COUNT),
        c.g[A0][0],
    );
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(RECORD_OFFSET));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let now_full = c.g[A0][0] == c.g[A2][0];
    c.sw(
        (c.g[V1][0] as u32).wrapping_add(store::DATA_END),
        c.g[V0][0],
    );
    if !now_full {
        c.cycles += 1;
        c.sw(
            (c.g[A3][0] as u32).wrapping_add(store::RECORD_STRIDE + RECORD_OFFSET),
            c.g[V0][0],
        );
    }
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(RECORD_OFFSET));
    ret(c);
}

/// `int SLoginStore_Hash(const void* data, uint size, void* out)` (0x3fbb50):
/// run the store's checksum interface (begin, update over `data`, final into
/// `out`); the first failing step's error is mapped into the store's range,
/// success returns 0.
pub fn slogin_store_hash(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::HASH_IFACE));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(HASH_BEGIN));
        c.g[S3][0] = c.g[A2][0];
        let begin = c.g[V0][0] as u32;
        call_ptr(c, begin, 0x003f_bb84)?;
        let mut failed = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if !failed {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(HASH_UPDATE));
            c.g[A1][0] = c.g[S2][0];
            let update = c.g[V0][0] as u32;
            call_ptr(c, update, 0x003f_bb98)?;
            failed = c.g[V0][0] != 0;
            if !failed {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(HASH_FINAL));
                c.g[A0][0] = c.g[S3][0];
                let finish = c.g[V0][0] as u32;
                call_ptr(c, finish, 0x003f_bbac)?;
                failed = c.g[V0][0] != 0;
                if !failed {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
            }
        }
        if failed {
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_HASH_ERROR.call(c, 0x003f_bbc4)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SLoginStore_HashDirectory(uint* out)` (0x3fbbe0): checksum the
/// whole directory (16 bytes per entry) and copy the 4-byte result to `out`
/// (through the pointer at +4 of the object the final step leaves in the
/// out slot; the hash's error code is dropped); an empty directory stores 0.
pub fn slogin_store_hash_directory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORD_COUNT));
        c.g[A1][0] = sx((c.g[V0][0] as u32) << 4);
        let empty = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if empty {
            c.cycles += 2;
            c.sw(c.g[S0][0] as u32, 0);
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORDS));
            c.g[A2][0] = c.g[SP][0];
            callee::SLOGIN_STORE_HASH.call(c, 0x003f_bc14)?;
            c.g[V1][0] = c.lw(sp(c));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
            c.lwl(V1, (c.g[V0][0] as u32).wrapping_add(3));
            c.lwr(V1, c.g[V0][0] as u32);
            c.swl(V1, (c.g[S0][0] as u32).wrapping_add(3));
            c.swr(V1, c.g[S0][0] as u32);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// One pass of the store cipher over `data` (`SLoginStore_Encrypt` and
/// `SLoginStore_Decrypt` differ only in the transform slot and their return
/// addresses: set-key, transform, reset, error map).
pub(crate) fn slogin_store_cipher_pass(c: &mut Cpu, transform_slot: u32, rets: [u32; 4]) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::CIPHER_IFACE));
        let no_cipher = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut failed = false;
        if !no_cipher {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(CIPHER_SET_KEY));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::CIPHER_KEY_LEN));
            let set_key = c.g[V0][0] as u32;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::CIPHER_KEY));
            call_ptr(c, set_key, rets[0])?;
            failed = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !failed {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::CIPHER_IFACE));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(transform_slot));
                let transform = c.g[V0][0] as u32;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, transform, rets[1])?;
                failed = c.g[V0][0] != 0;
                if !failed {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::CIPHER_IFACE));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(CIPHER_RESET));
                    let reset = c.g[V0][0] as u32;
                    call_ptr(c, reset, rets[2])?;
                    failed = c.g[V0][0] != 0;
                }
            }
        }
        if failed {
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, rets[3])?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Encrypt(void* data, uint size)` (0x3fbc40): encrypt
/// `data` in place with the store cipher (set key, encrypt, reset); no
/// cipher means success. Errors are mapped into the store's range.
pub fn slogin_store_encrypt(c: &mut Cpu) {
    slogin_store_cipher_pass(
        c,
        CIPHER_ENCRYPT,
        [0x003f_bc74, 0x003f_bc90, 0x003f_bcac, 0x003f_bcc4],
    );
}

/// `int SLoginStore_Decrypt(void* data, uint size)` (0x3fbcd8): decrypt
/// `data` in place with the store cipher (set key, decrypt, reset); no
/// cipher means success. Errors are mapped into the store's range.
pub fn slogin_store_decrypt(c: &mut Cpu) {
    slogin_store_cipher_pass(
        c,
        CIPHER_DECRYPT,
        [0x003f_bd0c, 0x003f_bd28, 0x003f_bd44, 0x003f_bd5c],
    );
}

/// `int SLoginStore_BeginOpen(const char* name, int device, uint arg2,
/// uint arg3)` (0x3fbd70): start
/// opening the store file on the memory card (`SLoginMc_StartOpen` with the
/// caller's first three arguments), then record `a3` at +0x74, set pending
/// operation 4 and empty the directory. Returns the mapped card error.
pub fn slogin_store_begin_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A3][0];
        call_ptr(c, callee::SLOGIN_MC_START_OPEN.addr, 0x003f_bd84)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[V0][0];
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(store::OPERATION),
            c.g[S0][0],
        );
        c.sw((c.g[V1][0] as u32).wrapping_add(store::STEP), c.g[A1][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(store::DATA_END), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(store::RECORD_COUNT), 0);
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bda4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_SeekToStart(void)` (0x3fbdb8): set pending operation 9
/// and start a card seek of the store file to offset 0. Returns the mapped
/// card error.
pub fn slogin_store_seek_to_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        frame.save_ra(c, 0);
        c.g[V1][0] = 9;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(store::STEP), c.g[V1][0]);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_bddc)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bde4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_ReadDirHeader(void)` (0x3fbdf0): set pending operation 7
/// and start a card read of one bare record header (12 bytes, rounded to
/// format 1) into the directory buffer. Returns the mapped card error.
pub fn slogin_store_read_dir_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 7;
        frame.save(c, S0, 0x10);
        c.g[A0][0] = 0;
        c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(store::STEP), c.g[V0][0]);
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_be14)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::DIR_BUFFER));
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        call_ptr(c, callee::SLOGIN_MC_START_READ.addr, 0x003f_be24)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_be2c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_LoadDirEntry(void)` (0x3fbe40): set pending operation 11,
/// decrypt the 12-byte record header in the directory buffer and add it to
/// the directory (its CRC through +0x88). Returns 5 when the new entry's
/// offset equals the data end (directory full), else starts a card seek to
/// the data end and returns the mapped card error; a decrypt error is
/// returned as is.
pub fn slogin_store_load_dir_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        c.sw(sp(c).wrapping_add(4), 0);
        c.g[V1][0] = 0xb;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = BLOCK_HEADER_BYTES as u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(store::STEP), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::DIR_BUFFER));
        callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_be64)?;
        let failed = c.g[V0][0] != 0;
        frame.restore_ra(c, 0x10);
        if !failed {
            c.cycles += 15;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
            c.g[A2][0] = c.g[SP][0] | 4;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::DIR_BUFFER));
            c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(store::RECORD_CRC));
            callee::SLOGIN_STORE_GET_RECORD_HEADER.call(c, 0x003f_be84)?;
            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
            c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
            c.g[A0][0] = c.lw(sp(c));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(store::RECORD_CRC));
            callee::SLOGIN_STORE_ADD_RECORD.call(c, 0x003f_be98)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(store::DATA_END));
            let at_end = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 5;
            if at_end {
                c.cycles += 1;
                frame.restore_ra(c, 0x10);
            } else {
                c.cycles += 6;
                c.g[A2][0] = 0;
                call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_beb0)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_beb8)?;
                frame.restore_ra(c, 0x10);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_WriteRecord(void)` (0x3fbed0): set pending operation 6,
/// fill the validated block header before the record buffer (kind +0x8c,
/// size +0x84, CRC +0x88), encrypt the header and then the payload, and start
/// a card write of the whole block (size rounded to format 2). Returns 4 for
/// a buffer without a valid block header, a cipher error, or the mapped card
/// error.
pub fn slogin_store_write_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 6;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.sw((c.g[V0][0] as u32).wrapping_add(store::STEP), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
        callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_bef4)?;
        'done: {
            'started: {
                c.g[S0][0] = c.g[V0][0];
                let no_header = c.g[S0][0] == 0;
                c.g[V0][0] = 4;
                if no_header {
                    break 'started;
                }
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_KIND));
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(store::RECORD_CRC));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
                callee::SLOGIN_STORE_SET_RECORD_HEADER.call(c, 0x003f_bf18)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = BLOCK_HEADER_BYTES as u64;
                callee::SLOGIN_STORE_ENCRYPT.call(c, 0x003f_bf24)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'done;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
                callee::SLOGIN_STORE_ENCRYPT.call(c, 0x003f_bf3c)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'done;
                }
                c.cycles += 10;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
                c.g[A1][0] = 2;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
                callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_bf54)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, callee::SLOGIN_MC_START_WRITE.addr, 0x003f_bf64)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bf6c)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_ReadRecord(void)` (0x3fbf80): set pending operation 10
/// and start a card read of the record block (header before the record
/// buffer, size +0x84 rounded to format 1). Returns the mapped card error.
pub fn slogin_store_read_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        frame.save(c, S0, 0x10);
        c.g[V1][0] = 0xa;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.sw((c.g[V0][0] as u32).wrapping_add(store::STEP), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
        callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_bfa4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORD_SIZE));
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_bfb8)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::SLOGIN_MC_START_READ.addr, 0x003f_bfc8)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_bfd0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_VerifyRecord(void)` (0x3fbfe0): check the record just
/// read into the store's record buffer: decrypt its 12-byte header, compare
/// the header's kind and size words with the expected ones, decrypt the
/// payload, hash it and compare the hash with the header's CRC word. Returns
/// 0 when it matches, 0xb for a wrong kind or hash length, 0x6e for a wrong
/// size, 0xa for a CRC mismatch, or the cipher/hash error.
pub fn slogin_store_verify_record(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V0][0] = c.lw(slot);
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S1, 0x20);
        c.sw(sp(c).wrapping_add(8), 0);
        c.g[S1][0] = 0xa;
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
        c.sw(sp(c).wrapping_add(4), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(store::STEP), c.g[S1][0]);
        callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_c010)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = BLOCK_HEADER_BYTES as u64;
        c.g[A0][0] = c.g[S0][0];
        callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_c020)?;
        'epilogue: {
            'result: {
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'epilogue;
                }
                c.cycles += 11;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[A3][0] = c.lw(slot);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = c.g[SP][0] | 4;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(store::RECORD_CRC));
                callee::SLOGIN_STORE_GET_RECORD_HEADER.call(c, 0x003f_c040)?;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[A0][0] = c.lw(slot);
                c.g[V1][0] = c.lw(sp(c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(store::RECORD_KIND));
                let other_kind = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0xb;
                if other_kind {
                    break 'result;
                }
                c.cycles += 4;
                c.g[A1][0] = c.lw(sp(c).wrapping_add(4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(store::RECORD_SIZE));
                let other_size = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x6e;
                if other_size {
                    break 'result;
                }
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(store::RECORD_BUFFER));
                callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_c06c)?;
                let failed = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x30);
                if failed {
                    break 'epilogue;
                }
                c.cycles += 7;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                c.g[A2][0] = c.g[SP][0] | 8;
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
                callee::SLOGIN_STORE_HASH.call(c, 0x003f_c088)?;
                let failed = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x30);
                if failed {
                    break 'epilogue;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(sp(c).wrapping_add(8));
                c.g[V1][0] = 4;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[V0][0] = 0xb;
                    break 'epilogue;
                }
                c.cycles += 7;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[A1][0] = c.lw(slot);
                c.g[A2][0] = 4;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(4));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(store::RECORD_CRC));
                callee::MEMCMP.call(c, 0x003f_c0b8)?;
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = 0;
                }
                c.g[V0][0] = c.g[S1][0];
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `bool SLoginStore_IsInitialized(void)` (0x3fc0d8): whether the login
/// store's state block exists.
pub fn slogin_store_is_initialized(c: &mut Cpu) {
    c.cycles += 3;
    let slot = gp_slot(c, store::STATE_GP);
    c.g[V0][0] = c.lw(slot);
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    ret(c);
}

/// `int SLoginStore_Init(const StoreConfig* config)` (0x3fc0e8): start the
/// memory-card layer, allocate the store state from the config's heap, set up
/// the checksum interface and the optional cipher (with a copy of its key),
/// allocate the record directory and the directory block. Returns 0, 1 when
/// already initialized, 6 when out of memory, the mapped card, hash or
/// cipher error, or a failing `SLoginStore_AllocBlock`'s error unmapped.
pub fn slogin_store_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c0fc)?;
        'epilogue: {
            'restore: {
                let initialized = c.g[V0][0] != 0;
                c.g[V0][0] = 1;
                if initialized {
                    break 'restore;
                }
                c.cycles += 6;
                let config = c.g[S0][0] as u32;
                c.g[A0][0] = c.lw(config.wrapping_add(store::CONFIG_DEVICES));
                c.g[A1][0] = c.lw(config.wrapping_add(store::CONFIG_MC_FUNCS));
                c.g[A2][0] = c.lw(config.wrapping_add(store::CONFIG_HEAP));
                call_ptr(c, callee::SLOGIN_MC_INIT.addr, 0x003f_c114)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c124)?;
                    frame.restore(c, S0, 0x10);
                    break 'epilogue;
                }
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_HEAP));
                'no_memory: {
                    c.cycles += 5;
                    c.g[A1][0] = store::STATE_BYTES;
                    c.g[A2][0] = 4;
                    call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_c138)?;
                    let failed = c.g[V0][0] == 0;
                    let slot = gp_slot(c, store::STATE_GP);
                    c.sw(slot, c.g[V0][0]);
                    if failed {
                        break 'no_memory;
                    }
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(store::HASH_IFACE));
                    call_ptr(c, callee::SLOGIN_MC_GET_CHECKSUM_IFACE.addr, 0x003f_c148)?;
                    let slot = gp_slot(c, store::STATE_GP);
                    c.g[V0][0] = c.lw(slot);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::HASH_IFACE));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let hash_init = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_HEAP));
                    call_ptr(c, hash_init, 0x003f_c15c)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V0][0];
                        callee::SLOGIN_STORE_MAP_HASH_ERROR.call(c, 0x003f_c16c)?;
                        frame.restore(c, S0, 0x10);
                        break 'epilogue;
                    }
                    let slot = gp_slot(c, store::STATE_GP);
                    c.g[V0][0] = c.lw(slot);
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_CIPHER));
                    let no_cipher = c.g[V1][0] == 0;
                    c.sw(
                        (c.g[V0][0] as u32).wrapping_add(store::CIPHER_IFACE),
                        c.g[V1][0],
                    );
                    if !no_cipher {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        let cipher_init = c.g[V0][0] as u32;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_HEAP));
                        call_ptr(c, cipher_init, 0x003f_c18c)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[V0][0];
                            callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, 0x003f_c19c)?;
                            frame.restore(c, S0, 0x10);
                            break 'epilogue;
                        }
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_KEY_LEN));
                        c.cycles += 9;
                        c.g[A2][0] = 4;
                        let slot = gp_slot(c, store::STATE_GP);
                        c.g[V0][0] = c.lw(slot);
                        c.g[A1][0] = c.g[V1][0];
                        c.sw(
                            (c.g[V0][0] as u32).wrapping_add(store::CIPHER_KEY_LEN),
                            c.g[V1][0],
                        );
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_HEAP));
                        call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_c1bc)?;
                        let slot = gp_slot(c, store::STATE_GP);
                        c.g[V1][0] = c.lw(slot);
                        let failed = c.g[V0][0] == 0;
                        c.sw(
                            (c.g[V1][0] as u32).wrapping_add(store::CIPHER_KEY),
                            c.g[V0][0],
                        );
                        if failed {
                            break 'no_memory;
                        }
                        c.cycles += 4;
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::CIPHER_KEY_LEN));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_KEY));
                        callee::MEMCPY.call(c, 0x003f_c1d8)?;
                    }
                    c.cycles += 13;
                    let slot = gp_slot(c, store::STATE_GP);
                    c.g[V0][0] = c.lw(slot);
                    c.g[A2][0] = 4;
                    let config = c.g[S0][0] as u32;
                    c.g[V1][0] = c.lw(config.wrapping_add(store::CONFIG_MAX_RECORDS));
                    let state = c.g[V0][0] as u32;
                    c.sw(state.wrapping_add(store::DATA_END), 0);
                    c.sw(state.wrapping_add(store::RECORD_CAPACITY), c.g[V1][0]);
                    c.sw(state.wrapping_add(store::RECORD_COUNT), 0);
                    c.g[A1][0] = c.lw(config.wrapping_add(store::CONFIG_MAX_RECORDS));
                    c.g[A0][0] = c.lw(config.wrapping_add(store::CONFIG_HEAP));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
                    call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_c200)?;
                    let slot = gp_slot(c, store::STATE_GP);
                    c.g[A0][0] = c.lw(slot);
                    let allocated = c.g[V0][0] != 0;
                    c.sw((c.g[A0][0] as u32).wrapping_add(store::RECORDS), c.g[V0][0]);
                    if allocated {
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(store::HEAP));
                        c.g[A2][0] = 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(store::DIR_BUFFER));
                        c.g[A3][0] = 1;
                        callee::SLOGIN_STORE_ALLOC_BLOCK.call(c, 0x003f_c228)?;
                        if c.g[V0][0] != 0 {
                            frame.restore(c, S0, 0x10);
                            break 'epilogue;
                        }
                        c.cycles += 8;
                        let slot = gp_slot(c, store::STATE_GP);
                        c.g[V1][0] = c.lw(slot);
                        c.g[V0][0] = 0;
                        let state = c.g[V1][0] as u32;
                        c.g[A0][0] = c.lw(state.wrapping_add(store::DIR_BUFFER));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(BLOCK_HEADER_BYTES));
                        c.sw(state.wrapping_add(store::DIR_BUFFER), c.g[A0][0]);
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::CONFIG_HEAP));
                        c.sb(state.wrapping_add(store::CREATED), 0);
                        c.sw(state.wrapping_add(store::HEAP), c.g[A1][0]);
                        break 'restore;
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 6;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Shutdown(void)` (0x3fc260): write the sentinel over the
/// directory block and free it, free the record directory, shut down the
/// cipher (freeing its key) and the checksum interface, shut down the
/// memory-card layer and free the store state. Returns 0, 2 when not
/// initialized, the block-free error, or the mapped cipher, hash or card
/// error.
pub fn slogin_store_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c270)?;
        'epilogue: {
            'restore_ra: {
                let initialized = c.g[V0][0] != 0;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                if !initialized {
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    break 'restore_ra;
                }
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::DIR_BUFFER));
                callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_c288)?;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V1][0] = c.lw(slot);
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                let state = c.g[V1][0] as u32;
                c.g[V0][0] = c.lw(state.wrapping_add(store::DIR_BUFFER));
                c.g[A1][0] = c.lw(state.wrapping_add(store::HEAP));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(BLOCK_HEADER_BYTES));
                c.g[A0][0] = c.g[V0][0];
                c.sw(state.wrapping_add(store::DIR_BUFFER), c.g[V0][0]);
                callee::SLOGIN_STORE_FREE_BLOCK.call(c, 0x003f_c2ac)?;
                let failed = c.g[V0][0] != 0;
                frame.restore_ra(c, 0);
                if failed {
                    break 'epilogue;
                }
                c.cycles += 11;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                c.g[A3][0] = 4;
                let state = c.g[V0][0] as u32;
                c.g[A2][0] = c.lw(state.wrapping_add(store::RECORD_CAPACITY));
                c.g[A1][0] = c.lw(state.wrapping_add(store::RECORDS));
                c.g[A0][0] = c.lw(state.wrapping_add(store::HEAP));
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
                call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_c2d0)?;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::CIPHER_IFACE));
                let no_cipher = c.g[V1][0] == 0;
                c.sw((c.g[V0][0] as u32).wrapping_add(store::RECORDS), 0);
                if !no_cipher {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                    let cipher_shutdown = c.g[V0][0] as u32;
                    call_ptr(c, cipher_shutdown, 0x003f_c2ec)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V0][0];
                        callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, 0x003f_c2fc)?;
                        frame.restore_ra(c, 0);
                        break 'epilogue;
                    }
                    let slot = gp_slot(c, store::STATE_GP);
                    c.g[V0][0] = c.lw(slot);
                    c.cycles += 7;
                    c.g[A3][0] = 4;
                    let state = c.g[V0][0] as u32;
                    c.g[A2][0] = c.lw(state.wrapping_add(store::CIPHER_KEY_LEN));
                    c.g[A0][0] = c.lw(state.wrapping_add(store::HEAP));
                    c.g[A1][0] = c.lw(state.wrapping_add(store::CIPHER_KEY));
                    call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_c318)?;
                    let slot = gp_slot(c, store::STATE_GP);
                    c.g[V0][0] = c.lw(slot);
                    c.sw((c.g[V0][0] as u32).wrapping_add(store::CIPHER_KEY), 0);
                }
                c.cycles += 8;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::HASH_IFACE));
                c.sw((c.g[V0][0] as u32).wrapping_add(store::CIPHER_IFACE), 0);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(4));
                let hash_shutdown = c.g[V0][0] as u32;
                call_ptr(c, hash_shutdown, 0x003f_c338)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[V0][0];
                    callee::SLOGIN_STORE_MAP_HASH_ERROR.call(c, 0x003f_c348)?;
                    frame.restore_ra(c, 0);
                    break 'epilogue;
                }
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                c.cycles += 4;
                c.sw((c.g[V0][0] as u32).wrapping_add(store::HASH_IFACE), 0);
                call_ptr(c, callee::SLOGIN_MC_SHUTDOWN.addr, 0x003f_c358)?;
                let failed = c.g[V0][0] != 0;
                let slot = gp_slot(c, store::STATE_GP);
                c.g[A1][0] = c.lw(slot);
                if !failed {
                    c.cycles += 7;
                    c.g[A2][0] = store::STATE_BYTES;
                    c.g[A3][0] = 4;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::HEAP));
                    call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_c370)?;
                    let slot = gp_slot(c, store::STATE_GP);
                    c.sw(slot, 0);
                    c.g[V0][0] = 0;
                    break 'restore_ra;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c384)?;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// The shared shape of `SLoginStore_Create` and `SLoginStore_Delete`: mark
/// the store operation `op` (both the words at +0x74 and +0x78), start the
/// card operation at `start` with the caller's arguments and map its result.
pub(crate) fn slogin_store_start_card_op(
    c: &mut Cpu,
    op: u64,
    start: u32,
    start_ret: u32,
    map_ret: u32,
) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V1][0] = c.lw(slot);
        frame.save_ra(c, 0);
        c.g[V0][0] = op;
        c.sw((c.g[V1][0] as u32).wrapping_add(store::STEP), c.g[V0][0]);
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(store::OPERATION),
            c.g[V0][0],
        );
        call_ptr(c, start, start_ret)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, map_ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Create(a0, a1, a2)` (0x3fc390): mark store operation 1
/// and start creating the save file on the card (`SLoginMc_StartCreate` with
/// the same arguments); returns the mapped card error.
pub fn slogin_store_create(c: &mut Cpu) {
    slogin_store_start_card_op(
        c,
        1,
        callee::SLOGIN_MC_START_CREATE.addr,
        0x003f_c3ac,
        0x003f_c3b4,
    );
}

/// `int SLoginStore_Delete(a0, a1, a2)` (0x3fc3c0): mark store operation 2
/// and start deleting the save file (`SLoginMc_StartDelete` with the same
/// arguments); returns the mapped card error.
pub fn slogin_store_delete(c: &mut Cpu) {
    slogin_store_start_card_op(
        c,
        2,
        callee::SLOGIN_MC_START_DELETE.addr,
        0x003f_c3dc,
        0x003f_c3e4,
    );
}

/// `int SLoginStore_Open(a0, a1, a2)` (0x3fc4c8): `SLoginStore_BeginOpen`
/// with the same arguments and 4 as its fourth.
pub fn slogin_store_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = 4;
        callee::SLOGIN_STORE_BEGIN_OPEN.call(c, 0x003f_c4d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Close(Directory* dir)` (0x3fc4e8): hash the directory
/// into its word at +0x60 and copy that hash into the store state, mark store
/// operation 3, clear the record count and data end, and start closing the
/// card file (`SLoginMc_StartClose(dir)`); returns the mapped card error.
pub fn slogin_store_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(store::DIR_HASH));
        callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_c500)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(store::DIR_HASH));
        c.g[A2][0] = 3;
        let slot = gp_slot(c, store::STATE_GP);
        c.g[A1][0] = c.lw(slot);
        c.g[A0][0] = c.g[S0][0];
        c.sw(
            (c.g[A1][0] as u32).wrapping_add(store::DIR_HASH),
            c.g[V1][0],
        );
        c.g[V0][0] = c.lw(slot);
        let state = c.g[V0][0] as u32;
        c.sw(state.wrapping_add(store::STEP), c.g[A2][0]);
        c.sw(state.wrapping_add(store::OPERATION), c.g[A2][0]);
        c.sw(state.wrapping_add(store::RECORD_COUNT), 0);
        c.sw(state.wrapping_add(store::DATA_END), 0);
        call_ptr(c, callee::SLOGIN_MC_START_CLOSE.addr, 0x003f_c52c)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c534)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Write(Directory* dir, uint kind, uint occurrence,
/// void* buffer, uint size)` (0x3fc548): copy the directory header (0x64
/// bytes) into the store state, mark store operations 6/5, remember the
/// record buffer, size and kind, hash the payload, then update the CRC of the
/// existing record or add a new one, rehash the directory and start seeking
/// to the record's offset (`SLoginMc_StartSeek(dir, offset, 0)`). Returns the
/// hash error or the mapped card error.
pub fn slogin_store_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V1][0] = c.lw(slot);
        frame.save(c, S3, 0x30);
        c.g[T1][0] = 6;
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S4, 0x20);
        c.g[T2][0] = c.g[V1][0];
        frame.save(c, S1, 0x50);
        c.g[A0][0] = 5;
        frame.save(c, S2, 0x40);
        c.g[V0][0] = c.g[S3][0] | c.g[T2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.sw(sp(c), 0);
        c.g[S4][0] = c.g[A2][0];
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(store::OPERATION),
            c.g[T1][0],
        );
        c.g[V0][0] &= 7;
        c.sw((c.g[V1][0] as u32).wrapping_add(store::STEP), c.g[A0][0]);
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[S3][0];
        // Copy the directory header, 0x20 bytes per pass (src in v1, the
        // store state in t2).
        c.cycles += 2;
        c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(store::DIR_HASH));
        c.g[V0][0] = c.g[S1][0];
        if aligned {
            loop {
                c.cycles += 11;
                let (src, dst) = (c.g[V1][0] as u32, c.g[T2][0] as u32);
                c.g[A0][0] = c.ld(src);
                c.g[A1][0] = c.ld(src.wrapping_add(8));
                c.g[A2][0] = c.ld(src.wrapping_add(0x10));
                c.g[T1][0] = c.ld(src.wrapping_add(0x18));
                c.sd(dst, c.g[A0][0]);
                c.sd(dst.wrapping_add(8), c.g[A1][0]);
                c.sd(dst.wrapping_add(0x10), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T1][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c600);
                }
            }
        } else {
            loop {
                c.cycles += 19;
                let (src, dst) = (c.g[V1][0] as u32, c.g[T2][0] as u32);
                c.ldl(A0, src.wrapping_add(7));
                c.ldr(A0, src);
                c.ldl(A1, src.wrapping_add(0xf));
                c.ldr(A1, src.wrapping_add(8));
                c.ldl(A2, src.wrapping_add(0x17));
                c.ldr(A2, src.wrapping_add(0x10));
                c.ldl(T1, src.wrapping_add(0x1f));
                c.ldr(T1, src.wrapping_add(0x18));
                c.sdl(A0, dst.wrapping_add(7));
                c.sdr(A0, dst);
                c.sdl(A1, dst.wrapping_add(0xf));
                c.sdr(A1, dst.wrapping_add(8));
                c.sdl(A2, dst.wrapping_add(0x17));
                c.sdr(A2, dst.wrapping_add(0x10));
                c.sdl(T1, dst.wrapping_add(0x1f));
                c.sdr(T1, dst.wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let more = c.g[V1][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c5a4);
                }
            }
            c.cycles += 2;
        }
        c.cycles += 17;
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.sw(c.g[T2][0] as u32, c.g[V0][0]);
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V0][0] = c.lw(slot);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S4][0];
        let state = c.g[V0][0] as u32;
        c.sw(state.wrapping_add(store::RECORD_SIZE), c.g[T0][0]);
        c.sw(state.wrapping_add(store::RECORD_BUFFER), c.g[A3][0]);
        c.sw(state.wrapping_add(store::RECORD_KIND), c.g[S0][0]);
        callee::SLOGIN_STORE_FIND_RECORD.call(c, 0x003f_c650)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A2][0] = c.g[SP][0];
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V0][0] = c.lw(slot);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
        callee::SLOGIN_STORE_HASH.call(c, 0x003f_c668)?;
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            // The hash result's CRC word (`*(result->crc)`) goes to the
            // store's record CRC.
            c.g[V0][0] = c.lw(sp(c));
            let slot = gp_slot(c, store::STATE_GP);
            c.g[A0][0] = c.lw(slot);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
            c.lwl(A3, (c.g[V1][0] as u32).wrapping_add(3));
            c.lwr(A3, c.g[V1][0] as u32);
            c.swl(A3, (c.g[A0][0] as u32).wrapping_add(store::RECORD_CRC + 3));
            c.swr(A3, (c.g[A0][0] as u32).wrapping_add(store::RECORD_CRC));
            let found = c.g[S2][0] != 0;
            c.g[A2][0] = c.lw(slot);
            if !found {
                c.cycles += 8;
                c.g[V0][0] = c.lw(slot);
                c.g[A0][0] = c.g[S0][0];
                let state = c.g[V0][0] as u32;
                c.g[A1][0] = c.lw(state.wrapping_add(store::RECORD_SIZE));
                c.g[A2][0] = sx(state.wrapping_add(store::RECORD_CRC));
                c.g[S0][0] = c.lw(state.wrapping_add(store::DATA_END));
                callee::SLOGIN_STORE_ADD_RECORD.call(c, 0x003f_c6ac)?;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(RECORD_OFFSET));
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(store::RECORD_CRC));
                callee::SLOGIN_STORE_SET_RECORD_CRC.call(c, 0x003f_c6c8)?;
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[S1][0];
            callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_c6d0)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0;
            call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_c6e0)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c6e8)?;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_Read(Directory* dir, uint kind, uint occurrence,
/// void* buffer, uint size)` (0x3fc708): copy the directory header into the
/// store state, mark store operations 8/5, remember the record buffer, size
/// and kind, find the record and, when its size matches, start seeking to
/// its offset (`SLoginMc_StartSeek(dir, offset, 0)`). Returns 7 when the
/// record is missing, 0x6e for a size mismatch, or the mapped card error.
pub fn slogin_store_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V1][0] = c.lw(slot);
        frame.save(c, S0, 0x20);
        c.g[T1][0] = 5;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 8;
        c.g[T2][0] = c.g[V1][0];
        c.g[T3][0] = c.g[A1][0];
        c.g[V0][0] = c.g[S0][0] | c.g[T2][0];
        c.sw(
            (c.g[V1][0] as u32).wrapping_add(store::OPERATION),
            c.g[A0][0],
        );
        c.g[A1][0] = c.g[A2][0];
        c.g[S1][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] &= 7;
        let aligned = c.g[V0][0] == 0;
        c.sw((c.g[V1][0] as u32).wrapping_add(store::STEP), c.g[T1][0]);
        // Copy the directory header, 0x20 bytes per pass (src in a0, the
        // store state in t2).
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(store::DIR_HASH));
        if aligned {
            loop {
                c.cycles += 11;
                let (src, dst) = (c.g[A0][0] as u32, c.g[T2][0] as u32);
                c.g[V1][0] = c.ld(src);
                c.g[A2][0] = c.ld(src.wrapping_add(8));
                c.g[T0][0] = c.ld(src.wrapping_add(0x10));
                c.g[T1][0] = c.ld(src.wrapping_add(0x18));
                c.sd(dst, c.g[V1][0]);
                c.sd(dst.wrapping_add(8), c.g[A2][0]);
                c.sd(dst.wrapping_add(0x10), c.g[T0][0]);
                c.sd(dst.wrapping_add(0x18), c.g[T1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                let more = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c7ac);
                }
            }
        } else {
            loop {
                c.cycles += 19;
                let (src, dst) = (c.g[A0][0] as u32, c.g[T2][0] as u32);
                c.ldl(V1, src.wrapping_add(7));
                c.ldr(V1, src);
                c.ldl(A2, src.wrapping_add(0xf));
                c.ldr(A2, src.wrapping_add(8));
                c.ldl(T0, src.wrapping_add(0x17));
                c.ldr(T0, src.wrapping_add(0x10));
                c.ldl(T1, src.wrapping_add(0x1f));
                c.ldr(T1, src.wrapping_add(0x18));
                c.sdl(V1, dst.wrapping_add(7));
                c.sdr(V1, dst);
                c.sdl(A2, dst.wrapping_add(0xf));
                c.sdr(A2, dst.wrapping_add(8));
                c.sdl(T0, dst.wrapping_add(0x17));
                c.sdr(T0, dst.wrapping_add(0x10));
                c.sdl(T1, dst.wrapping_add(0x1f));
                c.sdr(T1, dst.wrapping_add(0x18));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                let more = c.g[A0][0] != c.g[V0][0];
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c754);
                }
            }
            c.cycles += 2;
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
        c.sw(c.g[T2][0] as u32, c.g[V0][0]);
        let slot = gp_slot(c, store::STATE_GP);
        c.g[V0][0] = c.lw(slot);
        c.g[A0][0] = c.g[T3][0];
        let state = c.g[V0][0] as u32;
        c.sw(state.wrapping_add(store::RECORD_BUFFER), c.g[A3][0]);
        c.sw(state.wrapping_add(store::RECORD_SIZE), c.g[S1][0]);
        c.sw(state.wrapping_add(store::RECORD_KIND), c.g[T3][0]);
        callee::SLOGIN_STORE_FIND_RECORD.call(c, 0x003f_c7f8)?;
        'epilogue: {
            'failed: {
                c.g[V1][0] = c.g[V0][0];
                let missing = c.g[V1][0] == 0;
                c.g[V0][0] = 7;
                if missing {
                    break 'failed;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(RECORD_SIZE));
                let other_size = c.g[S1][0] != c.g[V0][0];
                c.g[V0][0] = 0x6e;
                if other_size {
                    break 'failed;
                }
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(RECORD_OFFSET));
                c.g[A0][0] = c.g[S0][0];
                let slot = gp_slot(c, store::STATE_GP);
                c.g[V0][0] = c.lw(slot);
                c.g[A2][0] = 0;
                c.g[A1][0] = c.g[V1][0];
                c.sw(
                    (c.g[V0][0] as u32).wrapping_add(store::SEEK_OFFSET),
                    c.g[V1][0],
                );
                call_ptr(c, callee::SLOGIN_MC_START_SEEK.addr, 0x003f_c82c)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_c834)?;
                frame.restore(c, S0, 0x20);
                break 'epilogue;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The DNAS relay block and its init, update and exit entry slots.
pub(crate) const DNAS_RELAY_BASE: u32 = 0x0045_0000;

pub(crate) const DNAS_INIT_SLOT: u32 = 0x152c;

pub(crate) const DNAS_UPDATE_SLOT: u32 = 0x1530;

pub(crate) const DNAS_EXIT_SLOT: u32 = 0x1534;

/// `"\x7fELF"` as a little-endian word.
pub(crate) const ELF_MAGIC: u64 = 0x464c_457f;

/// ELF header word `e_shoff` and the section header size.
pub(crate) const ELF_SECTION_OFFSET: u32 = 0x20;

pub(crate) const ELF_SECTION_SIZE: u64 = 0x28;

/// The last ordinary section index (`SHN_LORESERVE - 1`) and `SHN_ABS`.
pub(crate) const ELF_SPECIAL_SECTION_END: u64 = 0xfeff;

pub(crate) const ELF_SECTION_ABS: u64 = 0xfff1;

/// `".text"`, `".symtab"` and `".strtab"`: the sections `DnasRel_ElfOpen` records.
pub(crate) const DNAS_TEXT_NAME: u32 = 0x0049_75f0;

pub(crate) const DNAS_SYMTAB_NAME: u32 = 0x0049_75f8;

pub(crate) const DNAS_STRTAB_NAME: u32 = 0x0049_7600;

/// `".rel.text"`, `".data"`, `".rel.data"`, `".rodata"`, `".rel.rodata"`.
pub(crate) const DNAS_TEXT_RELOCS: u32 = 0x0049_7608;

pub(crate) const DNAS_DATA_NAME: u32 = 0x0049_7618;

pub(crate) const DNAS_DATA_RELOCS: u32 = 0x0049_7620;

pub(crate) const DNAS_RODATA_NAME: u32 = 0x0049_7630;

pub(crate) const DNAS_RODATA_RELOCS: u32 = 0x0049_7638;

/// ELF header word +0x10: `e_type` ET_REL (1) and `e_machine` EM_MIPS (8).
pub(crate) const ELF_RELAY_TYPE: u64 = 0x0008_0001;

/// ELF header halfwords: `e_shnum`, the section count, and `e_shstrndx`, the
/// index of the section-name string table.
pub(crate) const ELF_SECTION_COUNT: u32 = 0x30;

pub(crate) const ELF_SECTION_NAMES_INDEX: u32 = 0x32;

/// Section header words: name (`sh_name`), file offset (`sh_offset`) and size
/// in bytes (`sh_size`).
pub(crate) const ELF_SECTION_NAME: u32 = 0;

pub(crate) const ELF_SECTION_FILE_OFFSET: u32 = 0x10;

pub(crate) const ELF_SECTION_BYTES: u32 = 0x14;

/// One `Elf32_Rel` entry: `r_offset`, `r_info` (symbol << 8 | type).
pub(crate) const REL_ENTRY_SIZE: u32 = 8;

pub(crate) const REL_TYPE_32: u64 = 2;

pub(crate) const REL_TYPE_26: u64 = 4;

pub(crate) const REL_TYPE_HI16: u64 = 5;

pub(crate) const REL_TYPE_LO16: u64 = 6;

/// Waiting HI16 entries: at most 31 {word index, original word} pairs from
/// `sp + 0x100`; the entry count lives at `sp + 0x200`.
pub(crate) const REL_PENDING_LIMIT: u64 = 31;

pub(crate) const REL_PENDING_OFFSET: u32 = 0x100;

pub(crate) const REL_COUNT_SLOT: u32 = 0x200;

/// Call the relay entry stored in `slot` of the DNAS relay block.
pub(crate) fn dnas_relay_call(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V1][0] = DNAS_RELAY_BASE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(DNAS_RELAY_BASE + slot).lw(c);
        frame.save_ra(c, 0);
        let target = c.g[V0][0] as u32;
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Calls the update entry in the loaded DNAS relay.
pub fn dnas_rel_call_update(c: &mut Cpu) {
    dnas_relay_call(c, DNAS_UPDATE_SLOT, 0x003f_d118);
}

/// Calls the exit entry in the loaded DNAS relay.
pub fn dnas_rel_call_exit(c: &mut Cpu) {
    dnas_relay_call(c, DNAS_EXIT_SLOT, 0x003f_d140);
}

/// Runs the loaded DNAS relay around the caller's update callback.
pub fn dnas_rel_run(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = DNAS_RELAY_BASE as u64;
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        c.g[V1][0] = Addr(DNAS_RELAY_BASE + DNAS_INIT_SLOT).lw(c);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = 0x0102_0000;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 1;
        frame.save_ra(c, 0);
        let init = c.g[V1][0] as u32;
        c.g[S3][0] = DNAS_RELAY_BASE as u64;
        call_ptr(c, init, 0x003f_d190)?;
        loop {
            c.cycles += 7;
            let user_update = c.g[S1][0] as u32;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, user_update, 0x003f_d198)?;
            c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(DNAS_UPDATE_SLOT)).lw(c);
            let relay_update = c.g[V1][0] as u32;
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            call_ptr(c, relay_update, 0x003f_d1a4)?;
            let repeat = c.g[V0][0] == c.g[S2][0];
            c.g[V1][0] = DNAS_RELAY_BASE as u64;
            if repeat {
                if event_due(c) {
                    deliver_event_here(c, 0x003f_d190);
                }
                continue;
            }
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(DNAS_EXIT_SLOT)).lw(c);
            let relay_exit = c.g[V0][0] as u32;
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, relay_exit, 0x003f_d1b8)?;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore(c, S4, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    });
}

/// Reports whether the given buffer begins with an ELF file signature.
pub fn dnas_rel_is_elf(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = ELF_MAGIC;
    c.g[V0][0] = ((c.g[V0][0] ^ c.g[V1][0]) < 1) as u64;
    abi::ret(c);
}

/// Locates an ELF section header by index in the relay image.
pub fn dnas_rel_get_section_header(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = ELF_SECTION_SIZE;
    c.mult(0, 5, 5, 3);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(ELF_SECTION_OFFSET)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `u32 DnasRel_GetSectionAddr(DnasRel* rel, uint index)` (0x3fd2b8): the
/// file offset (`sh_offset`) of section `index`; for the reserved indices
/// above 0xfeff, 0xfff00000 for 0xfff1 (`SHN_ABS`) and 0 for the others.
pub fn dnas_rel_get_section_addr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = ELF_SPECIAL_SECTION_END;
        c.g[V0][0] = (c.g[V0][0] < c.g[A1][0]) as u64;
        let special = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if special {
            c.cycles += 3;
            c.g[V0][0] = c.g[A1][0] ^ ELF_SECTION_ABS;
            c.g[V1][0] = 0xffff_ffff_fff0_0000;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = 0;
            }
        } else {
            c.cycles += 4;
            callee::DNAS_REL_GET_SECTION_HEADER.call(c, 0x003f_d2d4)?;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.g[V1][0];
        frame.close(c);
        Ok(())
    });
}

/// Finds a named section header in the DNAS relay ELF image.
pub fn dnas_rel_find_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = ELF_SECTION_SIZE;
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_NAMES_INDEX)).lhu(c);
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_OFFSET)).lw(c);
        c.mult(0, 2, 2, 6);
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_COUNT)).lhu(c);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        let no_sections = c.g[A0][0] == 0;
        c.g[S3][0] = Addr((c.g[V0][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
        if no_sections {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 1;
            loop {
                c.cycles += 7;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(ELF_SECTION_NAME)).lw(c);
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                callee::STRCMP.call(c, 0x003f_d274)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_COUNT)).lhu(c);
                c.cycles += 4;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(ELF_SECTION_SIZE as u32));
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_d260);
                }
            }
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
    });
}

/// Opens a DNAS relay ELF image and records its text section, symbol table and string table.
pub fn dnas_rel_elf_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S4][0] = 0;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S3][0];
        callee::DNAS_REL_IS_ELF.call(c, 0x003f_d58c)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0x80000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr((c.g[S3][0] as u32).wrapping_add(0x10)).lw(c);
            c.g[V0][0] = ELF_RELAY_TYPE;
            let wrong_type = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S4][0];
            if !wrong_type {
                c.cycles += 19;
                c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[S1][0]);
                c.g[A1][0] = DNAS_TEXT_NAME as u64;
                c.sw(c.g[S0][0] as u32, c.g[S3][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[S4][0] = 1;
                callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d5c0)?;
                c.g[A1][0] = DNAS_SYMTAB_NAME as u64;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d5d4)?;
                c.g[A1][0] = DNAS_STRTAB_NAME as u64;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
                callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d5e8)?;
                let text_missing = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[V0][0];
                if !text_missing {
                    c.cycles += 5;
                    c.g[V0][0] =
                        Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[V0][0]);
                    c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_BYTES)).lw(c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                }
                c.cycles += 2;
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] =
                        Addr((c.g[A0][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                }
                c.cycles += 2;
                c.g[V0][0] = c.g[S4][0];
                if c.g[S2][0] != 0 {
                    c.cycles += 6;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(ELF_SECTION_BYTES)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    c.g[V1][0] =
                        Addr((c.g[S2][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S4][0];
                }
            }
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
    });
}

/// `void DnasRel_RelocateSection(DnasRel* rel, const char* section,
/// const char* rel_section, const u32* remap)` (0x3fd2f8): applies the MIPS
/// relocation entries of the relay section named `rel_section` to the
/// section named `section` (nothing when either is missing). Each entry's
/// symbol address comes from `DnasRel_GetSymbol`; an address inside the
/// window starting at `remap[1]` (`remap[0]` bytes long) is replaced by the
/// word at the same offset in the table at `remap[2]`. R_MIPS_32 adds the
/// address to the word, R_MIPS_26 adds it shifted right by 2 (unmasked);
/// R_MIPS_HI16 entries wait in a list of up to 31 on the stack until the
/// next R_MIPS_LO16, which patches its own low half and the high half of
/// every waiting word (with the carry from bit 15), then empties the list.
/// Other types are skipped.
pub fn dnas_rel_relocate_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x2b0);
        frame.save(c, FP, 0x220);
        frame.save(c, S0, 0x2a0);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S7, 0x230);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S1, 0x290);
        frame.save(c, S2, 0x280);
        frame.save(c, S3, 0x270);
        frame.save(c, S4, 0x260);
        frame.save(c, S5, 0x250);
        frame.save(c, S6, 0x240);
        frame.save_ra(c, 0x210);
        c.g[S7][0] = c.g[A3][0];
        callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d334)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[FP][0];
        callee::DNAS_REL_FIND_SECTION.call(c, 0x003f_d344)?;
        c.g[V1][0] = c.g[V0][0];
        'done: {
            if c.g[S1][0] == 0 {
                break 'done;
            }
            c.cycles += 2;
            c.g[S6][0] = 0;
            if c.g[V1][0] == 0 {
                break 'done;
            }
            // s2 = the relocation entries, s5 = the section they patch.
            c.cycles += 10;
            c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(ELF_SECTION_BYTES)).lw(c);
            c.g[S3][0] = 0;
            c.g[V0][0] = Addr(c.g[FP][0] as u32).lw(c);
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(ELF_SECTION_FILE_OFFSET)).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) >> 3);
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            let count_slot = (c.g[SP][0] as u32).wrapping_add(REL_COUNT_SLOT);
            c.sw(count_slot, c.g[V1][0]);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 1;
            loop {
                // s4 = word index, s1 = type, s0 = the word as it is now.
                c.cycles += 16;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[A0][0] = c.g[FP][0];
                c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(4)).lw(c);
                c.g[A2][0] = c.g[SP][0];
                c.g[S4][0] = sx((c.g[V0][0] as u32) >> 2);
                c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                c.g[S1][0] = c.g[A1][0] & 0xff;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 8);
                c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
                callee::DNAS_REL_GET_SYMBOL.call(c, 0x003f_d3ac)?;

                // a2 = the symbol address, through the remap window.
                c.g[A0][0] = Addr((c.g[S7][0] as u32).wrapping_add(4)).lw(c);
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[A2][0] < c.g[A0][0]) as u64;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = REL_TYPE_26;
                if !below {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[S7][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
                    let inside = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    if inside {
                        c.cycles += 3;
                        c.g[V1][0] = Addr((c.g[S7][0] as u32).wrapping_add(8)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = REL_TYPE_26;
                }

                // Dispatch on the type (t1 = next index, t0 = next entry).
                c.cycles += 2;
                let is_26 = c.g[S1][0] == c.g[V0][0];
                c.g[V0][0] = (c.g[S1][0] < 5) as u64;
                if is_26 {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[A2][0] as u32) >> 2);
                    c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                } else {
                    c.cycles += 2;
                    let low_type = c.g[V0][0] != 0;
                    c.g[V0][0] = REL_TYPE_32;
                    if low_type {
                        c.cycles += 2;
                        let is_32 = c.g[S1][0] == c.g[V0][0];
                        c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                        if is_32 {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                        } else {
                            c.cycles += 2;
                        }
                        c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = REL_TYPE_HI16;
                        let is_hi16 = c.g[S1][0] == c.g[V0][0];
                        c.g[V0][0] = REL_TYPE_LO16;
                        if is_hi16 {
                            c.cycles += 4;
                            c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                            c.g[V0][0] = (c.g[S6][0] < REL_PENDING_LIMIT) as u64;
                            c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                            if c.g[V0][0] != 0 {
                                c.cycles += 7;
                                c.g[V0][0] = sx((c.g[S6][0] as u32) << 3);
                                c.g[V1][0] =
                                    sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
                                c.g[A0][0] = c.g[V1][0];
                                c.sw(
                                    (c.g[V1][0] as u32).wrapping_add(REL_PENDING_OFFSET),
                                    c.g[S4][0],
                                );
                                c.sw(
                                    (c.g[A0][0] as u32).wrapping_add(REL_PENDING_OFFSET + 4),
                                    c.g[S0][0],
                                );
                            }
                        } else {
                            c.cycles += 2;
                            let is_lo16 = c.g[S1][0] == c.g[V0][0];
                            c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                            if is_lo16 {
                                dnas_rel_relocate_lo16(c);
                            } else {
                                c.cycles += 2;
                                c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
                            }
                        }
                    }
                }

                c.cycles += 5;
                let count_slot = (c.g[SP][0] as u32).wrapping_add(REL_COUNT_SLOT);
                c.g[V1][0] = Addr(count_slot).lw(c);
                c.g[S3][0] = c.g[T1][0];
                c.g[V0][0] = (c.g[S3][0] < c.g[V1][0]) as u64;
                c.g[S2][0] = c.g[T0][0];
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_d380);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x2a0);
        frame.restore(c, S1, 0x290);
        frame.restore(c, S2, 0x280);
        frame.restore(c, S3, 0x270);
        frame.restore(c, S4, 0x260);
        frame.restore(c, S5, 0x250);
        frame.restore(c, S6, 0x240);
        frame.restore(c, S7, 0x230);
        frame.restore(c, FP, 0x220);
        frame.restore_ra(c, 0x210);
        frame.close(c);
        Ok(())
    });
}

/// The R_MIPS_LO16 arm of `DnasRel_RelocateSection`: adds the symbol address (a2) plus the
/// addend made of the first waiting HI16 word's low half and this word's signed low half,
/// stores the low half of the sum into this word, the rounded high half into every waiting
/// word, and empties the waiting list (s6).
pub(crate) fn dnas_rel_relocate_lo16(c: &mut Cpu) {
    c.cycles += 18;
    let first_pending = (c.g[SP][0] as u32).wrapping_add(REL_PENDING_OFFSET + 4);
    c.g[V1][0] = Addr(first_pending).lhu(c);
    c.g[V0][0] = sx((c.g[S0][0] as u32) << 16);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
    c.g[A1][0] = 0xffff_ffff_ffff_0000;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[A1][0] &= c.g[S0][0];
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[S4][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
    c.g[V0][0] = c.g[A2][0] & 0xffff;
    c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> 15) as u32);
    c.g[A1][0] |= c.g[V0][0];
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(REL_ENTRY_SIZE));
    c.g[V1][0] &= 1;
    c.g[A3][0] = 0;
    if c.g[S6][0] != 0 {
        // a2 = high half rounded by bit 15; a1 walks the saved words.
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[A2][0] as u32) >> 16);
        c.g[T2][0] = 0xffff_ffff_ffff_0000;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(REL_PENDING_OFFSET + 4));
        loop {
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_sub(4)).lw(c);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[A0][0] = (c.g[A3][0] < c.g[S6][0]) as u64;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            c.g[V1][0] &= c.g[T2][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
            c.g[V1][0] |= c.g[A2][0];
            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_d4e0);
            }
        }
    }
    c.cycles += 1;
    c.g[S6][0] = 0;
}

/// Relocates the relay's text, data and read-only data sections in order.
pub fn dnas_rel_relocate_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = DNAS_TEXT_RELOCS as u64;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = DNAS_TEXT_NAME as u64;
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S1][0];
        callee::DNAS_REL_RELOCATE_SECTION.call(c, 0x003f_d690)?;

        c.g[A1][0] = DNAS_DATA_NAME as u64;
        c.g[A2][0] = DNAS_DATA_RELOCS as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        callee::DNAS_REL_RELOCATE_SECTION.call(c, 0x003f_d6ac)?;

        c.g[A1][0] = DNAS_RODATA_NAME as u64;
        c.g[A2][0] = DNAS_RODATA_RELOCS as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        callee::DNAS_REL_RELOCATE_SECTION.call(c, 0x003f_d6c8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `u32 DnasRel_GetSymbol(DnasRel* rel, uint index, char* name)` (0x3fd6e0):
/// copy symbol `index`'s name to `name` and return its address in the loaded
/// image (image + its section's file offset + `st_value`). Returns 0 when the
/// relay has no symbol or string table.
pub fn dnas_rel_get_symbol(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        let no_symbols = c.g[A1][0] == 0;
        c.g[A0][0] = c.g[A2][0];
        if !no_symbols {
            c.cycles += 3;
            c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lw(c);
            let no_names = c.g[A3][0] == 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 4);
            if !no_names {
                c.cycles += 11;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                callee::STRCPY.call(c, 0x003f_d724)?;
                c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xe)).lhu(c);
                c.g[A0][0] = c.g[S1][0];
                callee::DNAS_REL_GET_SECTION_ADDR.call(c, 0x003f_d730)?;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `u32 DnasRel_FindSymbol(DnasRel* rel, const char* name)` (0x3fd758): scan
/// the symbol table for `name` and return the first match's address in the
/// loaded image (image + its section's file offset + `st_value`). Returns 0
/// when the relay has no symbol or string table or no symbol matches.
pub fn dnas_rel_find_symbol(c: &mut Cpu) {
    guest(c, |c| {
        let mut at_increment = c.pc == 0x003f_d7a0;
        if !at_increment && c.pc != 0x003f_d758 {
            c.jump_out(c.pc, 0x003f_d758, 0x003f_d80c);
            return Ok(());
        }
        let frame = if at_increment {
            // Resumed at the back edge: the frame is already open.
            Frame { size: 0x60 }
        } else {
            c.cycles += 12;
            Frame::open(c, 0x60)
        };
        if !at_increment {
            frame.save(c, S1, 0x40);
            frame.save(c, S3, 0x20);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x10);
            c.g[S3][0] = 0;
            frame.save(c, S0, 0x50);
            frame.save(c, S2, 0x30);
            frame.save_ra(c, 0);
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
        }
        let no_symbols = !at_increment && c.g[A0][0] == 0;
        if !at_increment {
            c.g[S4][0] = c.g[A1][0];
        }
        if !no_symbols {
            if !at_increment {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lw(c);
                c.g[S0][0] = c.g[A0][0];
            }
            let no_names = !at_increment && c.g[V0][0] == 0;
            if !no_names {
                if !at_increment {
                    c.cycles += 2;
                    c.g[S2][0] = 0;
                }
                loop {
                    if at_increment {
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 4;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14)).lw(c);
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    let exhausted = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S4][0];
                    if exhausted {
                        break;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x20)).lw(c);
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    callee::STRCMP.call(c, 0x003f_d7c4)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 7;
                        c.g[A1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xe)).lhu(c);
                        c.g[A0][0] = c.g[S1][0];
                        callee::DNAS_REL_GET_SECTION_ADDR.call(c, 0x003f_d7d8)?;
                        c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(4)).lw(c);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        break;
                    }
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    if back_edge_unwinds(c, 0x003f_d7a0) {
                        return Ok(());
                    }
                    at_increment = true;
                }
            }
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SLoginMc_DeleteStep` entry and end (its `jump_out` range).
pub(crate) const DELETE_ENTRY: u32 = 0x003f_ae50;

/// The back-edge head an unwind resumes at (restart on the next card).
pub(crate) const DELETE_RETRY_HEAD: u32 = 0x003f_af60;

pub(crate) const DELETE_END: u32 = 0x003f_b274;

/// The phase dispatch table (phases 1..=26).
pub(crate) const DELETE_DISPATCH: u32 = 0x0049_74a0;

/// Where a delete step goes after its phase handler.
#[derive(Clone, Copy)]
pub(crate) enum DeleteExit {
    /// 0x3fb260: restore s0, then the shared epilogue.
    Return,
    /// 0x3fb1c0: abort at finish (v0 holds the card state), then return.
    AbortAtFinish,
    /// The `SLoginMc_GetError` tails: s0 comes back in the call's delay
    /// slot, so the 0x3fb260 charge is skipped.
    AfterGetError,
    /// Already left (jump out, or an unwind at the retry head).
    Left,
}

/// `int SLoginMc_DeleteStep(int result, int* done, int* card_reply)`
/// (0x3fae50): advance the memory card delete sequence after a card RPC.
/// `a0` is the RPC result, `a1` receives the completion status and `a2`
/// points at the RPC's returned word. The phase word at +4 of the card state
/// (`gp+0x1ae8`) picks the step: open the save directory (1), check it (2),
/// remove `icon.sys` (19), `view.ico` (20), the data file (21) and the
/// directory (18), then finish or retry on the next selected card (26).
/// Returns 0 while pending, 0x12 for bad arguments or an unknown phase.
/// Dispatcher-shaped: re-entered at [`DELETE_RETRY_HEAD`] after an unwind at
/// the retry back edge.
pub fn slogin_mc_delete_step(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        let exit = match c.pc {
            DELETE_ENTRY => {
                c.cycles += 7;
                Frame::open(c, 0x80);
                frame.save(c, S0, 0x70);
                frame.save(c, S1, 0x60);
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0x50);
                delete_step_phase(c)?
            }
            DELETE_RETRY_HEAD => {
                delete_retry_device(c)?;
                DeleteExit::Return
            }
            pc => {
                c.jump_out(pc, DELETE_ENTRY, DELETE_END);
                return Ok(());
            }
        };
        match exit {
            DeleteExit::Return => {
                c.cycles += 1;
                frame.restore(c, S0, 0x70);
            }
            DeleteExit::AbortAtFinish => {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(card::DEVICE));
                callee::SLOGIN_MC_ABORT_AT_FINISH.call(c, 0x003f_b1cc)?;
                frame.restore(c, S0, 0x70);
            }
            DeleteExit::AfterGetError => frame.restore(c, S0, 0x70),
            DeleteExit::Left => return Ok(()),
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// Argument and mode checks, then the phase dispatch through the game's table.
pub(crate) fn delete_step_phase(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if c.g[S1][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0x12;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 2;
    c.g[V0][0] = 0x12;
    if c.g[A2][0] == 0 {
        return Ok(DeleteExit::Return);
    }
    // Only card modes 2 and 5 delete.
    c.cycles += 5;
    c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
    c.g[V1][0] = 2;
    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
    c.g[V0][0] = 5;
    if c.g[A0][0] == c.g[V1][0] {
        c.cycles += 1;
        c.g[V0][0] = 1;
    } else {
        c.cycles += 2;
        let mode_five = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !mode_five {
            c.cycles += 2;
            c.g[V0][0] = 0x12;
            return Ok(DeleteExit::Return);
        }
    }
    c.cycles += 7;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw(gp_slot(c, card::STATE_GP));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(card::PHASE));
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = (c.g[A0][0] < 26) as u64;
    c.g[V0][0] = DATA_PAGE;
    if c.g[V1][0] == 0 {
        return Ok(delete_invalid_phase(c));
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(DELETE_DISPATCH & 0xffff));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    let target = c.g[A0][0] as u32;
    match target {
        0x003f_aed0 => delete_open_dir(c),
        0x003f_af98 => delete_check_dir(c),
        0x003f_b00c => delete_after_icon_sys(c),
        0x003f_b080 => delete_after_view_icon(c),
        0x003f_b0f0 => delete_after_data_file(c),
        0x003f_b15c => delete_after_dir(c),
        0x003f_b1d4 => delete_finish(c),
        0x003f_b254 => Ok(delete_invalid_phase(c)),
        _ => {
            c.jump_out(target, DELETE_ENTRY, DELETE_END);
            Ok(DeleteExit::Left)
        }
    }
}

/// 0x3fb254: unknown phase; done = 2, return 0x12.
pub(crate) fn delete_invalid_phase(c: &mut Cpu) -> DeleteExit {
    c.cycles += 3;
    c.g[V1][0] = 2;
    c.g[V0][0] = 0x12;
    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
    DeleteExit::Return
}

/// 0x3fb248: done = v1 (2), return the RPC result.
pub(crate) fn delete_result_error(c: &mut Cpu) -> DeleteExit {
    c.cycles += 3;
    c.g[V0][0] = c.g[S0][0];
    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
    DeleteExit::Return
}

/// 0x3faf60, also the resume head: back to phase 1 on the next selected
/// card through the interface's +0x10 callback (`a1` = card state).
pub(crate) fn delete_retry_device(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 8;
    c.g[V1][0] = c.lw(gp_slot(c, card::IFACE_GP));
    c.g[V0][0] = 1;
    c.sw((c.g[A1][0] as u32).wrapping_add(card::PHASE), c.g[V0][0]);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
    let callback = c.g[V0][0] as u32;
    call_ptr(c, callback, 0x003f_af78)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// 0x3fb068: remove the file named by `a1` in the save's path through the
/// interface's +0x24 callback (`a0` = card state, `v1` = interface).
pub(crate) fn delete_remove_file(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 6;
    c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(card::DEVICE));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
    let callback = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(card::FILE_PATH));
    call_ptr(c, callback, 0x003f_b078)?;
    c.g[V0][0] = 0;
    Ok(())
}

/// 0x3fb1b4: `SLoginMc_SetError(a0)`, then v0 = card state.
pub(crate) fn delete_set_error_reload(c: &mut Cpu) -> Result<(), Unwound> {
    c.cycles += 3;
    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_b1bc)?;
    c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
    Ok(())
}

/// Phase 1: clear the error, then open the save directory (result 0); after
/// result 3, in mode 2, move to the next selected card.
pub(crate) fn delete_open_dir(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 4;
    c.g[A0][0] = 0;
    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_aed8)?;
    let failed = c.g[S0][0] != 0;
    c.g[V0][0] = 3;
    if !failed {
        c.cycles += 16;
        c.g[A1][0] = c.lw(gp_slot(c, card::IFACE_GP));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.lw(gp_slot(c, card::STATE_GP));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(card::DIR_NAME));
        callee::SLOGIN_MC_BUILD_DIR_PATH.call(c, 0x003f_aef8)?;
        c.g[V1][0] = c.lw(gp_slot(c, card::STATE_GP));
        c.g[V0][0] = 2;
        c.g[A2][0] = c.lw(gp_slot(c, card::IFACE_GP));
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(card::PHASE), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(4));
        let callback = c.g[V0][0] as u32;
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(card::DEVICE));
        call_ptr(c, callback, 0x003f_af18)?;
        c.g[V0][0] = 0;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 2;
    c.g[V1][0] = 2;
    if c.g[S0][0] != c.g[V0][0] {
        return Ok(delete_result_error(c));
    }
    c.cycles += 5;
    c.g[V1][0] = c.lw(gp_slot(c, card::STATE_GP));
    c.g[S0][0] = 2;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    if c.g[V0][0] != c.g[S0][0] {
        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
        c.cycles += 2;
        c.g[V0][0] = 3;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 9;
    c.g[V0][0] = c.lw(gp_slot(c, card::IFACE_GP));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(card::SCAN_DIRECTION));
    c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
    callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.call(c, 0x003f_af4c)?;
    c.g[A1][0] = c.lw(gp_slot(c, card::STATE_GP));
    c.g[A0][0] = c.g[V0][0];
    c.g[V0][0] = u64::MAX;
    let exhausted = c.g[A0][0] == c.g[V0][0];
    c.sw((c.g[A1][0] as u32).wrapping_add(card::DEVICE), c.g[A0][0]);
    if exhausted {
        c.cycles += 4;
        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
        callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_af88)?;
        return Ok(DeleteExit::AfterGetError);
    }
    delete_retry_device(c)?;
    Ok(DeleteExit::Return)
}

/// Phase 2: the directory lookup's reply. 1: enter it and remove
/// `icon.sys` (phase 19); 0: error 4 and abort; else done with 10.
pub(crate) fn delete_check_dir(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 2;
    c.g[V0][0] = 1;
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_b004)?;
        c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 3;
    c.g[A2][0] = c.lw(c.g[A2][0] as u32);
    let found = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = c.lw(gp_slot(c, card::IFACE_GP));
    if found {
        c.cycles += 11;
        c.g[A0][0] = c.lw(gp_slot(c, card::STATE_GP));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let callback = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(card::FILE_PATH));
        call_ptr(c, callback, 0x003f_afbc)?;
        c.g[A0][0] = c.lw(gp_slot(c, card::STATE_GP));
        c.g[V0][0] = 0x13;
        c.g[V1][0] = c.lw(gp_slot(c, card::IFACE_GP));
        c.g[A1][0] = DATA_PAGE;
        c.sw((c.g[A0][0] as u32).wrapping_add(card::PHASE), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(card::ICON_SYS_NAME & 0xffff));
        delete_remove_file(c)?;
        return Ok(DeleteExit::Return);
    }
    c.cycles += 2;
    c.g[V1][0] = 2;
    if c.g[A2][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        return Ok(DeleteExit::Return);
    }
    c.cycles += 4;
    c.g[A0][0] = 4;
    callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_afe8)?;
    c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
    Ok(DeleteExit::AbortAtFinish)
}

/// The result check shared by phases 19-21. Result 3: abort at finish
/// (with error 3 unless one is already set); true, v0 = card state. Else:
/// unless the result is 4, record it as the error when none is set; false,
/// `state_reg` = card state. `rets`: the return addresses of the two
/// `SLoginMc_GetError` calls and of the `SLoginMc_SetError` call.
pub(crate) fn delete_removal_result(
    c: &mut Cpu,
    state_reg: usize,
    rets: [u32; 3],
) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let three = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = 4;
    if three {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, rets[0])?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
        if !failed {
            c.cycles += 2;
            c.g[A0][0] = 3;
            delete_set_error_reload(c)?;
        }
        return Ok(true);
    }
    c.cycles += 2;
    let four = c.g[S0][0] == c.g[V0][0];
    c.g[state_reg][0] = c.lw(gp_slot(c, card::STATE_GP));
    if !four {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, rets[1])?;
        let failed = c.g[V0][0] != 0;
        c.g[state_reg][0] = c.lw(gp_slot(c, card::STATE_GP));
        if !failed {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::SLOGIN_MC_SET_ERROR.call(c, rets[2])?;
            c.g[state_reg][0] = c.lw(gp_slot(c, card::STATE_GP));
        }
    }
    Ok(false)
}

/// Phase 19: `icon.sys` is gone; remove `view.ico` (phase 20).
pub(crate) fn delete_after_icon_sys(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if delete_removal_result(c, A0, [0x003f_b020, 0x003f_b040, 0x003f_b050])? {
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 5;
    c.g[V0][0] = 0x14;
    c.g[V1][0] = c.lw(gp_slot(c, card::IFACE_GP));
    c.g[A1][0] = DATA_PAGE;
    c.sw((c.g[A0][0] as u32).wrapping_add(card::PHASE), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(card::VIEW_ICON_NAME & 0xffff));
    delete_remove_file(c)?;
    Ok(DeleteExit::Return)
}

/// Phase 20: `view.ico` is gone; remove the data file (phase 21).
pub(crate) fn delete_after_view_icon(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if delete_removal_result(c, V0, [0x003f_b094, 0x003f_b0b4, 0x003f_b0c4])? {
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 10;
    c.g[V1][0] = 0x15;
    c.g[A1][0] = c.lw(gp_slot(c, card::IFACE_GP));
    c.sw((c.g[V0][0] as u32).wrapping_add(card::PHASE), c.g[V1][0]);
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(card::FILE_PATH));
    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(card::DEVICE));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
    let callback = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[A0][0];
    call_ptr(c, callback, 0x003f_b0e8)?;
    c.g[V0][0] = 0;
    Ok(DeleteExit::Return)
}

/// Phase 21: the data file is gone; remove the directory through the
/// interface's +0x28 callback (phase 18).
pub(crate) fn delete_after_data_file(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    if delete_removal_result(c, V1, [0x003f_b104, 0x003f_b124, 0x003f_b134])? {
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 9;
    c.g[V0][0] = 0x12;
    c.g[A2][0] = c.lw(gp_slot(c, card::IFACE_GP));
    c.sw((c.g[V1][0] as u32).wrapping_add(card::PHASE), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(card::FILE_PATH));
    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x28));
    let callback = c.g[V0][0] as u32;
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(card::DEVICE));
    call_ptr(c, callback, 0x003f_b154)?;
    c.g[V0][0] = 0;
    Ok(DeleteExit::Return)
}

/// Phase 18: the directory is gone. A failed removal aborts at finish
/// (recording the result unless an error is set); else clear the error
/// when none is set and finish through the interface's +0x14 callback
/// (phase 26).
pub(crate) fn delete_after_dir(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 2;
    if c.g[S0][0] != 0 {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b1a8)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
        if !failed {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            delete_set_error_reload(c)?;
        }
        return Ok(DeleteExit::AbortAtFinish);
    }
    c.cycles += 4;
    callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b16c)?;
    let failed = c.g[V0][0] != 0;
    c.g[V1][0] = c.lw(gp_slot(c, card::STATE_GP));
    if !failed {
        c.cycles += 3;
        c.g[A0][0] = 0;
        callee::SLOGIN_MC_SET_ERROR.call(c, 0x003f_b17c)?;
        c.g[V1][0] = c.lw(gp_slot(c, card::STATE_GP));
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1a;
    c.g[A1][0] = c.lw(gp_slot(c, card::IFACE_GP));
    c.sw((c.g[V1][0] as u32).wrapping_add(card::PHASE), c.g[V0][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    let callback = c.g[V0][0] as u32;
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(card::DEVICE));
    call_ptr(c, callback, 0x003f_b198)?;
    c.g[V0][0] = 0;
    Ok(DeleteExit::Return)
}

/// Phase 26: the transaction finished. Mode 5, or no further selected card,
/// reports done (2) and returns the error; mode 2 without an error is done
/// with 0; mode 2 with an error retries on the next selected card (the back
/// edge to [`DELETE_RETRY_HEAD`]).
pub(crate) fn delete_finish(c: &mut Cpu) -> Result<DeleteExit, Unwound> {
    c.cycles += 2;
    c.g[V1][0] = 2;
    if c.g[S0][0] != 0 {
        return Ok(delete_result_error(c));
    }
    c.cycles += 5;
    c.g[V0][0] = c.lw(gp_slot(c, card::STATE_GP));
    c.g[A0][0] = 5;
    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
    let mode_five = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = 2;
    if !mode_five {
        c.cycles += 4;
        callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b1f8)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp_slot(c, card::IFACE_GP));
        if !failed {
            c.cycles += 4;
            c.g[V1][0] = 2;
            c.g[V0][0] = 0;
            c.sw(c.g[S1][0] as u32, c.g[V1][0]);
            return Ok(DeleteExit::Return);
        }
        c.cycles += 9;
        c.g[V1][0] = c.lw(gp_slot(c, card::STATE_GP));
        c.g[A0][0] = c.lhu(c.g[V0][0] as u32);
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(card::SCAN_DIRECTION));
        callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.call(c, 0x003f_b220)?;
        c.g[A1][0] = c.lw(gp_slot(c, card::STATE_GP));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        let more = c.g[A0][0] != c.g[V0][0];
        c.sw((c.g[A1][0] as u32).wrapping_add(card::DEVICE), c.g[A0][0]);
        if more {
            if back_edge_unwinds(c, DELETE_RETRY_HEAD) {
                return Ok(DeleteExit::Left);
            }
            delete_retry_device(c)?;
            return Ok(DeleteExit::Return);
        }
        c.cycles += 1;
        c.g[V0][0] = 2;
    }
    c.cycles += 4;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    callee::SLOGIN_MC_GET_ERROR.call(c, 0x003f_b240)?;
    Ok(DeleteExit::AfterGetError)
}

/// `int SLoginStore_CountRecords(void* store, uint kind, int* count)`
/// (0x3fc850): count the directory records of `kind` into `*count`.
/// Returns 2 when the store is not initialized, 4 when `store` or `count` is
/// null, else 0.
pub fn slogin_store_count_records(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c874)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        let mut s0_restored = false;
        'done: {
            if !initialized {
                break 'done;
            }
            c.cycles += 2;
            let no_store = c.g[S1][0] == 0;
            c.g[V0][0] = 4;
            if no_store {
                break 'done;
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                frame.restore(c, S0, 0x30);
                s0_restored = true;
                break 'done;
            }
            c.sw(c.g[S0][0] as u32, 0);
            c.cycles += 8;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORD_COUNT));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORDS));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if empty {
                break 'done;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            loop {
                c.cycles += 2;
                let other_kind = c.g[S2][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(store::RECORD_STRIDE));
                if !other_kind {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    break 'done;
                }
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                if event_due(c) {
                    deliver_event_here(c, 0x003f_c8b8);
                }
            }
        }
        if !s0_restored {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_GetRecordSize(void* store, uint kind, uint occurrence,
/// uint* size)` (0x3fc8f8): find the given occurrence of a record kind and
/// write its size word (record + 8) to `*size`, which is cleared first.
/// Returns 2 when the store is not initialized, 4 when `store` or `size` is
/// null, 7 when no such record exists, else 0.
pub fn slogin_store_get_record_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = 0;
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_c92c)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        let mut s0_restored = false;
        'done: {
            if !initialized {
                break 'done;
            }
            c.cycles += 2;
            let no_store = c.g[S1][0] == 0;
            c.g[V0][0] = 4;
            if no_store {
                break 'done;
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                c.cycles += 2;
                frame.restore(c, S0, 0x50);
                s0_restored = true;
                break 'done;
            }
            c.sw(c.g[S0][0] as u32, 0);
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            callee::SLOGIN_STORE_FIND_RECORD.call(c, 0x003f_c958)?;
            if c.g[V0][0] == 0 {
                c.g[S4][0] = 7;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(RECORD_SIZE));
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S4][0];
        }
        if !s0_restored {
            c.cycles += 1;
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
    });
}

/// `int SLoginStore_GetInfo(void* out)` (0x3fc990): copy the first 0x64
/// bytes of the store state into `out` (doubleword copies when both
/// pointers are 8-byte aligned, unaligned `ldl`/`sdl` pairs otherwise).
/// Returns 4 when `out` is null, else 0.
pub fn slogin_store_get_info(c: &mut Cpu) {
    c.cycles += 2;
    let has_out = c.g[A0][0] != 0;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(store::STATE_GP));
    if !has_out {
        c.cycles += 2;
        c.g[V0][0] = 4;
        ret(c);
        return;
    }
    c.cycles += 4;
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] &= 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(store::DIR_HASH));
    if aligned {
        loop {
            c.cycles += 11;
            let src = c.g[V1][0] as u32;
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            let dst = c.g[A0][0] as u32;
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_ca04);
            }
        }
    } else {
        loop {
            c.cycles += 19;
            let src = c.g[V1][0] as u32;
            c.ldl(A1, src.wrapping_add(7));
            c.ldr(A1, src);
            c.ldl(A2, src.wrapping_add(0xf));
            c.ldr(A2, src.wrapping_add(8));
            c.ldl(A3, src.wrapping_add(0x17));
            c.ldr(A3, src.wrapping_add(0x10));
            c.ldl(T0, src.wrapping_add(0x1f));
            c.ldr(T0, src.wrapping_add(0x18));
            let dst = c.g[A0][0] as u32;
            c.sdl(A1, dst.wrapping_add(7));
            c.sdr(A1, dst);
            c.sdl(A2, dst.wrapping_add(0xf));
            c.sdr(A2, dst.wrapping_add(8));
            c.sdl(A3, dst.wrapping_add(0x17));
            c.sdr(A3, dst.wrapping_add(0x10));
            c.sdl(T0, dst.wrapping_add(0x1f));
            c.sdr(T0, dst.wrapping_add(0x18));
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_c9b0);
            }
        }
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[V0][0] = 0;
    ret(c);
}

/// `int SLoginStore_SetIconSys(const IconSys* icon)` (0x3fcdc8): hand the
/// save icon description to the memory-card layer and map its result into
/// the store's error range. Returns 2 when the store is not initialized.
pub fn slogin_store_set_icon_sys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_cddc)?;
        let initialized = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if initialized {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::SLOGIN_MC_SET_ICON_SYS.addr, 0x003f_cdec)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_cdf4)?;
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_AllocBlock(void** out, Heap* heap, uint payload_bytes,
/// int format)` (0x3fce10): allocate a zeroed block for a record payload
/// (header included, rounded to the format's alignment), stamp the 12-byte
/// header pattern at its start and store the payload pointer (block + 12)
/// in `*out`. Returns 4 when `out` is null, 6 when the allocation fails,
/// else 0.
pub fn slogin_store_alloc_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.sw(sp(c), c.g[V0][0]);
        c.g[A2][0] = c.g[SP][0] | 4;
        c.sw(sp(c).wrapping_add(4), c.g[V1][0]);
        callee::SLOGIN_STORE_GET_ALIGNMENT.call(c, 0x003f_ce58)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_ce64)?;
        let has_out = c.g[S2][0] != 0;
        c.g[S0][0] = c.g[V0][0];
        'done: {
            if !has_out {
                c.cycles += 2;
                c.g[V0][0] = 4;
                break 'done;
            }
            c.cycles += 6;
            c.g[A2][0] = c.lw(sp(c));
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, callee::TIB_EXT_MEM_ALLOC.addr, 0x003f_ce84)?;
            let allocated = c.g[V0][0] != 0;
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            if !allocated {
                c.cycles += 2;
                c.g[V0][0] = 6;
                break 'done;
            }
            c.cycles += 19;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::MEMSET.call(c, 0x003f_cea4)?;
            c.g[A1][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = DATA_PAGE;
            c.g[V0][0] = 0;
            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(BLOCK_HEADER_MAGIC & 0xffff));
            let pattern = c.g[T0][0] as u32;
            c.ldl(V1, pattern.wrapping_add(7));
            c.ldr(V1, pattern);
            c.lwl(A2, pattern.wrapping_add(11));
            c.lwr(A2, pattern.wrapping_add(8));
            let block = c.g[A1][0] as u32;
            c.sdl(V1, block.wrapping_add(7));
            c.sdr(V1, block);
            c.swl(A2, block.wrapping_add(11));
            c.swr(A2, block.wrapping_add(8));
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(BLOCK_HEADER_BYTES));
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_FreeBlock(void* payload, Heap* heap, uint payload_bytes,
/// int format)` (0x3fcf00): free a block made by `SLoginStore_AllocBlock`
/// (its start is 12 bytes before the payload), passing the same rounded
/// size and alignment. Returns 4 when `payload` is null, else 0.
pub fn slogin_store_free_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        c.sw(sp(c), c.g[V0][0]);
        c.sw(sp(c).wrapping_add(4), c.g[V1][0]);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0] | 4;
        callee::SLOGIN_STORE_GET_ALIGNMENT.call(c, 0x003f_cf48)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cf54)?;
        let no_payload = c.g[S2][0] == 0;
        c.g[A3][0] = c.lw(sp(c));
        if no_payload {
            c.cycles += 1;
            c.g[V0][0] = 4;
        } else {
            c.cycles += 6;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_sub(BLOCK_HEADER_BYTES));
            call_ptr(c, callee::TIB_EXT_MEM_FREE.addr, 0x003f_cf6c)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `int SLoginStore_GetBlockSize(uint payload_bytes, uint* size)`
/// (0x3fcf98): write the allocation size of a payload in store format 2 to
/// `*size`. Returns 4 when `size` is null, else 0.
pub fn slogin_store_get_block_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        let no_out = c.g[S0][0] == 0;
        frame.save_ra(c, 0);
        if no_out {
            c.cycles += 1;
            c.g[V0][0] = 4;
        } else {
            c.cycles += 5;
            c.g[A1][0] = 2;
            callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cfb4)?;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int DnasRel_Load(void* image, void* arg)` (0x3fcfd8): open the DNAS relay
/// ELF image, relocate it through its `_reloctable` symbol, record its
/// `DirtyDnasRelInit`/`Updt`/`Exit` entry points in the relay slots and
/// flush the caches. Returns 0, or -1 (not a relay ELF), -2 (no
/// `_reloctable`), -3/-4/-5 (init/update/exit entry missing).
pub fn dnas_rel_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A2][0] = c.g[A1][0];
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[A0][0] = c.g[SP][0];
        callee::DNAS_REL_ELF_OPEN.call(c, 0x003f_cff4)?;
        let opened = c.g[V0][0] != 0;
        c.g[A1][0] = DATA_PAGE;
        'done: {
            if !opened {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7598));
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d010)?;
            c.g[A0][0] = c.g[V0][0];
            let no_reloc_table = c.g[A0][0] == 0;
            c.g[V0][0] = sx(-2i32 as u32);
            if no_reloc_table {
                break 'done;
            }
            c.cycles += 18;
            c.g[V0][0] = 0x40_0000;
            c.g[V1][0] = 0x40_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x27f0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26f8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.sw(sp(c).wrapping_add(0x34), c.g[A0][0]);
            c.g[A1][0] = sx(sp(c).wrapping_add(0x30));
            c.g[A0][0] = c.g[SP][0];
            c.sw(sp(c).wrapping_add(0x30), c.g[V0][0]);
            c.g[S0][0] = DNAS_RELAY_BASE as u64;
            c.sw(sp(c).wrapping_add(0x38), c.g[V1][0]);
            callee::DNAS_REL_RELOCATE_ALL.call(c, 0x003f_d04c)?;
            c.g[A1][0] = DATA_PAGE;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75a8));
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d05c)?;
            let found = c.g[V0][0] != 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(DNAS_INIT_SLOT), c.g[V0][0]);
            if !found {
                c.cycles += 2;
                c.g[V0][0] = sx(-3i32 as u32);
                break 'done;
            }
            c.cycles += 7;
            c.g[A1][0] = DATA_PAGE;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75c0));
            c.g[S0][0] = DNAS_RELAY_BASE as u64;
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d080)?;
            let found = c.g[V0][0] != 0;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(DNAS_UPDATE_SLOT),
                c.g[V0][0],
            );
            if !found {
                c.cycles += 2;
                c.g[V0][0] = sx(-4i32 as u32);
                break 'done;
            }
            c.cycles += 7;
            c.g[A1][0] = DATA_PAGE;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x75d8));
            c.g[S0][0] = DNAS_RELAY_BASE as u64;
            callee::DNAS_REL_FIND_SYMBOL.call(c, 0x003f_d0a4)?;
            let missing = c.g[V0][0] == 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(DNAS_EXIT_SLOT), c.g[V0][0]);
            if missing {
                c.cycles += 1;
                c.g[V0][0] = sx(-5i32 as u32);
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = 0;
            callee::FLUSH_CACHE.call(c, 0x003f_d0b4)?;
            c.g[A0][0] = 2;
            callee::FLUSH_CACHE.call(c, 0x003f_d0bc)?;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// `void DnasRel_CallInit(void* arg)` (0x3fd0d8): call the loaded relay's
/// init entry (relay slot +0x152c) with `arg` and 0x1020000.
pub fn dnas_rel_call_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = DNAS_RELAY_BASE as u64;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(DNAS_INIT_SLOT));
        frame.save_ra(c, 0);
        let init = c.g[V1][0] as u32;
        c.g[A1][0] = 0x0102_0000;
        call_ptr(c, init, 0x003f_d0f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Where a step of `SLoginStore_Poll` leaves for the shared epilogue.
#[derive(Clone, Copy)]
pub(crate) enum StorePollExit {
    /// Translate the card result in `s0` (`SLoginStore_MapMcError`).
    MapMcError,
    /// Return `v0`, restoring `s0` first.
    RestoreS0,
    /// Return `v0`; `s0` is already restored.
    Epilogue,
}

/// `int SLoginStore_Poll(int* status, int* detail)` (0x3fca40): the login
/// store's per-frame step. It polls the memory card (`SLoginMc_Poll`); once
/// the card reports done (`*status == 2`) it finishes the card step named by
/// the store's operation (+0x74) and step (+0x78) words: writes or reads a
/// record, checks a read record's size and decrypts it, rewinds, reads the
/// directory header and loads directory entries, then compares the directory
/// hash. A step that starts another card request sets `*status = 1` and
/// returns that request's result; otherwise the card result is mapped into
/// the store's error range. Returns 2 when the store is not initialised and
/// 4 for a null argument.
pub fn slogin_store_poll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A1][0];
        callee::SLOGIN_STORE_IS_INITIALIZED.call(c, 0x003f_ca60)?;
        let exit = store_poll_steps(c, frame)?;
        if let StorePollExit::MapMcError = exit {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::SLOGIN_STORE_MAP_MC_ERROR.call(c, 0x003f_cdb0)?;
        }
        if !matches!(exit, StorePollExit::Epilogue) {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The body of `SLoginStore_Poll` after `SLoginStore_IsInitialized`
/// returned: `s1` is `status`, `s2` is `detail`, `s0` takes the card result.
pub(crate) fn store_poll_steps(c: &mut Cpu, frame: Frame) -> Result<StorePollExit, Unwound> {
    let not_ready = c.g[V0][0] == 0;
    c.g[V0][0] = 2;
    if not_ready {
        return Ok(StorePollExit::RestoreS0);
    }
    c.cycles += 2;
    let no_status = c.g[S1][0] == 0;
    c.g[V0][0] = 4;
    if no_status {
        return Ok(StorePollExit::RestoreS0);
    }
    c.cycles += 2;
    let has_detail = c.g[S2][0] != 0;
    c.g[A0][0] = c.g[S1][0];
    if !has_detail {
        c.cycles += 2;
        frame.restore(c, S0, 0x40);
        return Ok(StorePollExit::Epilogue);
    }

    // Poll the card; a pending card status with the store busy is error 8.
    c.cycles += 6;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, callee::SLOGIN_MC_POLL.addr, 0x003f_ca88)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
    } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::OPERATION));
        let busy = c.g[V1][0] != 0;
        c.g[V0][0] = 8;
        if busy {
            return Ok(StorePollExit::RestoreS0);
        }
        c.cycles += 1;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
    }
    c.cycles += 3;
    c.g[V0][0] = 2;
    if c.g[V1][0] != c.g[V0][0] {
        return Ok(StorePollExit::MapMcError);
    }

    // A card error only matters to operations 4 and 5.
    c.cycles += 2;
    let card_ok = c.g[S0][0] == 0;
    c.g[A1][0] = c.lw(gp_slot(c, store::STATE_GP));
    if !card_ok {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::OPERATION));
        c.g[V0][0] = 4;
        let op_four = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 5;
        if !op_four {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                return Ok(StorePollExit::MapMcError);
            }
        }
    }

    // Operation 1 sets the flag byte on success; every operation but 4
    // clears it otherwise.
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::OPERATION));
    c.g[V0][0] = 1;
    let mut clear_flag = true;
    if c.g[A0][0] != c.g[V0][0] {
        c.g[A0][0] = c.lw(gp_slot(c, store::STATE_GP));
    } else {
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            c.sb((c.g[A1][0] as u32).wrapping_add(store::CREATED), c.g[A0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::OPERATION));
        let still_one = c.g[V1][0] == c.g[A0][0];
        c.g[A0][0] = c.lw(gp_slot(c, store::STATE_GP));
        if still_one {
            clear_flag = false;
        }
    }
    if clear_flag {
        c.cycles += 4;
        c.g[V1][0] = 4;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(store::OPERATION));
        if c.g[V0][0] != c.g[V1][0] {
            c.sb((c.g[A0][0] as u32).wrapping_add(store::CREATED), 0);
        }
    }

    // Operation 6: write the record (step 5), or check the size of the
    // record just read and decrypt it (step 6).
    c.cycles += 5;
    c.g[V1][0] = c.lw(gp_slot(c, store::STATE_GP));
    c.g[V0][0] = 6;
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::OPERATION));
    let op_six = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
    if op_six {
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::STEP));
        c.g[V0][0] = 5;
        let write_step = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if write_step {
            c.cycles += 4;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::SLOGIN_STORE_WRITE_RECORD.call(c, 0x003f_cb3c)?;
            frame.restore(c, S0, 0x40);
            return Ok(StorePollExit::Epilogue);
        }
        c.cycles += 2;
        let decode_step = c.g[A0][0] == c.g[A1][0];
        c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
        if decode_step {
            c.cycles += 12;
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::RECORD_BUFFER));
            c.g[A1][0] = 0;
            callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_cb58)?;
            c.g[A0][0] = c.g[V0][0];
            callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_cb60)?;
            c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
            c.g[A1][0] = 2;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
            callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cb70)?;
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            let wrong_size = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0x6e;
            if wrong_size {
                return Ok(StorePollExit::RestoreS0);
            }
            c.cycles += 9;
            c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_BUFFER));
            callee::SLOGIN_STORE_DECRYPT.call(c, 0x003f_cb98)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                callee::SLOGIN_STORE_MAP_CIPHER_ERROR.call(c, 0x003f_cba8)?;
                frame.restore(c, S0, 0x40);
                return Ok(StorePollExit::Epilogue);
            }
            c.cycles += 2;
            let card_failed = c.g[S0][0] != 0;
            c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
            if !card_failed {
                c.cycles += 4;
                c.g[A0][0] = c.lw(gp_slot(c, store::STATE_GP));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(store::DIR_HASH));
                callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_cbc4)?;
                c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
            }
        }
    }

    // Operation 8: read the record (step 5), or check its size and verify
    // it (step 10).
    c.cycles += 4;
    c.g[A0][0] = 8;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::OPERATION));
    let op_eight = c.g[V1][0] == c.g[A0][0];
    c.g[A1][0] = c.g[V0][0];
    if op_eight {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::STEP));
        c.g[V0][0] = 5;
        let read_step = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0xa;
        if read_step {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::SLOGIN_STORE_READ_RECORD.call(c, 0x003f_cbf4)?;
            frame.restore(c, S0, 0x40);
            return Ok(StorePollExit::Epilogue);
        }
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::OPERATION));
        } else {
            c.cycles += 11;
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::RECORD_BUFFER));
            c.g[A1][0] = 0;
            callee::SLOGIN_STORE_GET_BLOCK_HEADER.call(c, 0x003f_cc10)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
            callee::SLOGIN_STORE_CALC_ALLOC_SIZE.call(c, 0x003f_cc24)?;
            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
            let wrong_size = c.g[V0][0] != c.g[V1][0];
            c.g[S0][0] = 0x6e;
            if !wrong_size {
                c.cycles += 5;
                c.g[V0][0] = c.lw(gp_slot(c, store::STATE_GP));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(store::RECORD_SIZE));
                c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                callee::SLOGIN_STORE_VERIFY_RECORD.call(c, 0x003f_cc40)?;
                c.g[S0][0] = c.g[V0][0];
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_cc4c)?;
            c.g[V0][0] = c.g[S0][0];
            return Ok(StorePollExit::RestoreS0);
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::OPERATION));
    }

    // Any operation but 4 and 5 ends here; a finished card status resets
    // the operation and step words.
    c.cycles += 4;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let directory_op = c.g[V0][0] != 0;
    c.g[V0][0] = 4;
    if !directory_op {
        c.cycles += 4;
        c.g[V1][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 2;
            c.sw((c.g[A1][0] as u32).wrapping_add(store::STEP), 0);
            c.sw((c.g[A1][0] as u32).wrapping_add(store::OPERATION), 0);
        }
        return Ok(StorePollExit::MapMcError);
    }

    // Operations 4 and 5, step 4: take the card's file state and rewind.
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::STEP));
    let rewind_step = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 9;
    if rewind_step {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.sb((c.g[A1][0] as u32).wrapping_add(store::CREATED), 0);
            return Ok(StorePollExit::MapMcError);
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[A1][0];
        call_ptr(c, callee::SLOGIN_MC_GET_FILE_STATE.addr, 0x003f_cc84)?;
        c.g[V1][0] = 1;
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        callee::SLOGIN_STORE_SEEK_TO_START.call(c, 0x003f_cc90)?;
        frame.restore(c, S0, 0x40);
        return Ok(StorePollExit::Epilogue);
    }

    // Step 9: done if the flag byte was set, else read the directory header.
    c.cycles += 2;
    let header_step = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = 7;
    if header_step {
        c.cycles += 2;
        if c.g[S0][0] != 0 {
            c.sb((c.g[A1][0] as u32).wrapping_add(store::CREATED), 0);
            return Ok(StorePollExit::MapMcError);
        }
        c.cycles += 3;
        c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(store::CREATED));
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = 2;
        if flagged {
            c.cycles += 4;
            c.sb((c.g[A1][0] as u32).wrapping_add(store::CREATED), 0);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V0][0] = 0;
            return Ok(StorePollExit::RestoreS0);
        }
        c.cycles += 6;
        c.sb((c.g[A1][0] as u32).wrapping_add(store::CREATED), 0);
        c.g[V0][0] = 1;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        callee::SLOGIN_STORE_READ_DIR_HEADER.call(c, 0x003f_ccd4)?;
        frame.restore(c, S0, 0x40);
        return Ok(StorePollExit::Epilogue);
    }

    // Step 7: load the next directory entry, or (all loaded, or card
    // result 10) hash the directory and store or compare the hash.
    c.cycles += 2;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 2;
        let card_ok = c.g[S0][0] == 0;
        c.g[V0][0] = 0xa;
        let mut load_entry = false;
        if !card_ok {
            c.cycles += 2;
            let no_more = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = 2;
            if !no_more {
                c.cycles += 2;
                c.sb((c.g[A1][0] as u32).wrapping_add(store::CREATED), 0);
                return Ok(StorePollExit::MapMcError);
            }
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::RECORD_CAPACITY));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::RECORD_COUNT));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if more {
                load_entry = true;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 2;
            }
        }
        if load_entry {
            c.cycles += 4;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::SLOGIN_STORE_LOAD_DIR_ENTRY.call(c, 0x003f_cd70)?;
            frame.restore(c, S0, 0x40);
            return Ok(StorePollExit::Epilogue);
        }
        c.cycles += 11;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lw(gp_slot(c, store::STATE_GP));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(store::DIR_BUFFER));
        callee::SLOGIN_STORE_WRITE_SENTINEL.call(c, 0x003f_cd20)?;
        c.g[A0][0] = c.g[SP][0];
        callee::SLOGIN_STORE_HASH_DIRECTORY.call(c, 0x003f_cd28)?;
        c.g[A1][0] = c.lw(gp_slot(c, store::STATE_GP));
        c.g[V1][0] = 5;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(store::OPERATION));
        let verify = c.g[V0][0] == c.g[V1][0];
        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
        if !verify {
            c.cycles += 3;
            c.g[V0][0] = 0;
            c.sw(
                (c.g[A1][0] as u32).wrapping_add(store::DIR_HASH),
                c.g[V1][0],
            );
            return Ok(StorePollExit::RestoreS0);
        }
        c.cycles += 8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(store::DIR_HASH));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        callee::MEMCMP.call(c, 0x003f_cd4c)?;
        c.g[V1][0] = 9;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = c.g[V1][0];
        return Ok(StorePollExit::RestoreS0);
    }

    // Any other step: read the directory header after a good card result.
    c.cycles += 2;
    let card_failed = c.g[S0][0] != 0;
    c.g[V0][0] = 1;
    if card_failed {
        return Ok(StorePollExit::MapMcError);
    }
    c.cycles += 4;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    callee::SLOGIN_STORE_READ_DIR_HEADER.call(c, 0x003f_cd88)?;
    frame.restore(c, S0, 0x40);
    Ok(StorePollExit::Epilogue)
}

/// `int SLogin_StepNop(void)` (0x3f77f0): a login step that does nothing
/// and reports 0.
pub fn slogin_step_nop(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

// ---------------------------------------------------------------------------
// `SLoginMc_SyncResult_Cmd00/Cmd01` (0x3f87f8/0x3f8800) use the shared
// card-state constants below (`SLOGIN_MC_SYNC_RESULT_STATE_GP`,
// `SLOGIN_MC_SYNC_RESULT_PHASE`).

/// `int SLoginMc_SyncResult_Cmd00(int result)` (0x3f87f8): report a memory
/// card sync result for step 0: echo the result.
pub fn slogin_mc_sync_result_cmd00(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd01(int result)` (0x3f8800): report a memory
/// card sync result for step 1: 0x12 while the step is not running,
/// otherwise echo the result (accepted: 0, 3, 0x11; 0x10 for anything
/// else).
pub fn slogin_mc_sync_result_cmd01(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
    c.g[V1][0] = 0x1;
    c.g[V0][0] = Addr(c.g[A1][0] as u32)
        .off(SLOGIN_MC_SYNC_RESULT_PHASE)
        .lw(c);
    if c.g[V0][0] != c.g[V1][0] {
        c.g[V0][0] = 0x12;
    } else {
        'result: {
            c.cycles += 2;
            if c.g[A0][0] == 0u64 {
                c.g[V0][0] = 0x3;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x3;
                if c.g[A0][0] == c.g[V0][0] {
                    c.g[V1][0] = 0x11;
                } else {
                    c.cycles += 2;
                    c.g[V1][0] = 0x11;
                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[V0][0] = 0x10;
                        break 'result;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A0][0];
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `gp` word holding the SLogin card-state block pointer, shared by the
/// `SLoginMc_SyncResult_CmdNN` translators below.
pub(crate) const SLOGIN_MC_SYNC_RESULT_STATE_GP: u32 = 0x1ae8;

/// Card-state word holding the in-progress command phase the translators gate on.
pub(crate) const SLOGIN_MC_SYNC_RESULT_PHASE: u32 = 0x04;

/// Result code while the card layer is in another command's phase.
pub(crate) const SLOGIN_MC_SYNC_RESULT_WRONG_PHASE: u32 = 0x12;

/// Result code for a sync result the translator does not recognize.
pub(crate) const SLOGIN_MC_SYNC_RESULT_BAD_RESULT: u32 = 0x10;

/// A pass-through result the translators echo back unchanged.
pub(crate) const SLOGIN_MC_SYNC_RESULT_PASS: u32 = 0x11;

/// `int SLoginMc_SyncResult_Cmd02(int result)` (0x3f8838): report the
/// memory-card sync result for command 0x02. Returns 0x12 while the card
/// layer is in another command's phase, 0x10 for an unrecognized result,
/// and echoes the result otherwise (accepted: 0, 4, 3, 9, 0x11).
pub fn slogin_mc_sync_result_cmd02(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x2;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x4;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x3;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd07(int result)` (0x3f8880): report the
/// memory-card sync result for command 0x07. The card phase must be one of
/// 0x1b/0x1d/0x1c/0x07/0x0b/0x18, else 0x12. Accepted results echo back
/// (0, 3, 4, 9, 0x0e, 0x0d, 0x11, 0x0a); anything else is 0x10.
pub fn slogin_mc_sync_result_cmd07(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x1b;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_1b = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x1d;
            if gate_1b {
                break 'gate;
            }
            c.cycles += 2;
            let gate_1d = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x1c;
            if gate_1d {
                break 'gate;
            }
            c.cycles += 2;
            let gate_1c = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x7;
            if gate_1c {
                break 'gate;
            }
            c.cycles += 2;
            let gate_07 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xb;
            if gate_07 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0b = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x18;
            if gate_0b {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0e = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_0e {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x11;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_11 = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = 0xa;
            if is_11 {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd08(int result)` (0x3f8908): report the
/// memory-card sync result for command 0x08. The card phase must be one of
/// 0x18/0x08/0x0c/0x10, else 0x12. Accepted results echo back
/// (0, 3, 4, 9, 0x11); anything else is 0x10.
pub fn slogin_mc_sync_result_cmd08(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x18;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_18 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x8;
            if gate_18 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_08 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xc;
            if gate_08 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0c = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x10;
            if gate_0c {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd09(int result)` (0x3f8968): report the
/// memory-card sync result for command 0x09. The card phase must be one of
/// 0x19/0x09/0x0d/0x11, else 0x12. Accepted results echo back
/// (0, 3, 4, 9, 0x11); anything else is 0x10.
pub fn slogin_mc_sync_result_cmd09(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x19;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_19 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if gate_19 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_09 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if gate_09 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0d = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x11;
            if gate_0d {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd1A(int result)` (0x3f89c8): report the
/// memory-card sync result for command 0x1a. The card phase must be 0x1a,
/// else 0x12. Accepted results echo back (0, 3, 9, 0x11); anything else
/// is 0x10.
pub fn slogin_mc_sync_result_cmd1a(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x1a;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd05(int result)` (0x3f8a08): report the
/// memory-card sync result for command 0x05. The card phase must be 0x05,
/// else 0x12. Accepted results echo back (0, 3, 9, 0x0e, 8, 0x11);
/// anything else is 0x10.
pub fn slogin_mc_sync_result_cmd05(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x5;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0e = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x8;
            if is_0e {
                break 'result;
            }
            c.cycles += 2;
            let is_eight = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_eight {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd06(int result)` (0x3f8a58): report the
/// memory-card sync result for command 0x06. The card phase must be one of
/// 0x16/0x06/0x0e/0x0a, else 0x12. Accepted results echo back
/// (0, 3, 4, 9, 0x0d, 0x11); anything else is 0x10.
pub fn slogin_mc_sync_result_cmd06(c: &mut Cpu) {
    'sync: {
        'gate: {
            c.cycles += 5;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = 0x16;
            c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let gate_16 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if gate_16 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_06 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xe;
            if gate_06 {
                break 'gate;
            }
            c.cycles += 2;
            let gate_0e = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0xa;
            if gate_0e {
                break 'gate;
            }
            c.cycles += 2;
            let wrong_phase = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            } else {
                break 'gate;
            }
        }
        'result: {
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd13(int result)` (0x3f8ac0): report the
/// memory-card sync result for command 0x13. The card phase must be
/// 0x13-0x15, else 0x12. Accepted results echo back
/// (0, 3, 4, 9, 0x0d, 6, 5, 0x11); anything else is 0x10.
pub fn slogin_mc_sync_result_cmd13(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 6;
            c.g[V1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x13));
            c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
            let wrong_phase = c.g[V0][0] == 0;
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_six = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x5;
            if is_six {
                break 'result;
            }
            c.cycles += 2;
            let is_five = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_five {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd12(int result)` (0x3f8b28): report the
/// memory-card sync result for command 0x12. The card phase must be 0x12,
/// else 0x12. Accepted results echo back (0, 3, 4, 9, 0x0d, 0x11);
/// anything else is 0x10.
pub fn slogin_mc_sync_result_cmd12(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x12;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0xd;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_0d = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_0d {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd03(int result)` (0x3f8b78): translate the
/// memory-card sync result for command 0x03. The card phase must be 0x03,
/// else 0x12. Results 0 and 3 echo back, 0x11 echoes, anything else is 0x10.
pub fn slogin_mc_sync_result_cmd03(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
    c.g[V0][0] = 0x3;
    c.g[V1][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
    let wrong_phase = c.g[V1][0] != c.g[V0][0];
    c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
    if wrong_phase {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let is_zero = c.g[A0][0] == 0;
    c.g[V0][0] = c.g[A0][0];
    if is_zero {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let is_phase = c.g[A0][0] == c.g[V1][0];
    c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
    if is_phase {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 2;
    let is_other = c.g[A0][0] != c.g[V1][0];
    c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
    if is_other {
        c.cycles += 2;
        ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[A0][0];
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd04(int result)` (0x3f8bb0): report the
/// memory-card sync result for command 0x04. The card phase must be 0x04,
/// else 0x12. Accepted results echo back (0, 3, 0x11); anything else
/// is 0x10.
pub fn slogin_mc_sync_result_cmd04(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x4;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_SyncResult_Cmd17(int result)` (0x3f8be8): report the
/// memory-card sync result for command 0x17. The card phase must be 0x17,
/// else 0x12. Accepted results echo back (0, 3, 4, 9, 0x11); anything
/// else is 0x10.
pub fn slogin_mc_sync_result_cmd17(c: &mut Cpu) {
    'sync: {
        'result: {
            c.cycles += 5;
            c.g[A1][0] =
                Addr((c.g[GP][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_STATE_GP)).lw(c);
            c.g[V1][0] = 0x17;
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(SLOGIN_MC_SYNC_RESULT_PHASE)).lw(c);
            let wrong_phase = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_WRONG_PHASE as u64;
            if wrong_phase {
                break 'sync;
            }
            c.cycles += 2;
            let is_zero = c.g[A0][0] == 0;
            c.g[V0][0] = 0x3;
            if is_zero {
                break 'result;
            }
            c.cycles += 2;
            let is_three = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            if is_three {
                break 'result;
            }
            c.cycles += 2;
            let is_four = c.g[A0][0] == c.g[V0][0];
            c.g[V0][0] = 0x9;
            if is_four {
                break 'result;
            }
            c.cycles += 2;
            let is_nine = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = SLOGIN_MC_SYNC_RESULT_PASS as u64;
            if is_nine {
                break 'result;
            }
            c.cycles += 2;
            let is_other = c.g[A0][0] != c.g[V1][0];
            c.g[V0][0] = SLOGIN_MC_SYNC_RESULT_BAD_RESULT as u64;
            if is_other {
                break 'sync;
            } else {
                break 'result;
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[A0][0];
        break 'sync;
    }
    c.cycles += 2;
    ret(c);
}

/// `int SLoginMc_IdleStep(int result)` (0x3f8c30): accept any result while
/// the card layer idles; echo it back to the caller.
pub fn slogin_mc_idle_step(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::require_paths;
    use crate::difftest::run_generated;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as STORE_POLL_TEST_RETURN;
    use std::sync::atomic::AtomicUsize;

    /// The control block of the SLogin_* test fakes in the test machine's RAM.
    /// Word 0 gates the shared `TibExtMemAlloc` fake (`ALLOC_MAGIC`); word 1
    /// is its result switch (0: the allocation fails with a null pointer,
    /// otherwise the fake's pointer or the forced `LOG_RESULTS` word); words
    /// 2 and 3 are the pass-through gate and unwind target
    /// (`SLOGIN_PASS_CTRL`, `SLOGIN_UNWIND_TARGET`).
    pub(crate) const ALLOC_CTRL: u32 = DATA + 0x2ff00;

    pub(crate) const ALLOC_MAGIC: u32 = 0x736c_6f67;

    /// The one gated `TibExtMemAlloc` fake of the SLogin_* tests: with the
    /// gate word set it logs the call (`fakes::fake_call`) and returns the
    /// fake's pointer, or null when `ALLOC_CTRL + 4` is 0; otherwise it runs
    /// what the slot held before.
    pub(crate) fn slogin_fake_alloc(c: &mut Cpu) {
        let previous = fakes::prev_cell(slogin_fake_alloc);
        if c.bus.read32(ALLOC_CTRL) != ALLOC_MAGIC {
            return fakes::fall_back(c, previous, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, 0x0026_6728);
        if c.bus.read32(ALLOC_CTRL + 4) == 0 {
            c.g[V0][0] = 0;
        }
    }

    /// Put `slogin_fake_alloc` in the `TibExtMemAlloc` slot (once per process).
    /// Every setup that writes `ALLOC_MAGIC` calls it first.
    pub(crate) fn hook_slogin_alloc() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            slogin_fake_alloc,
            fakes::prev_cell(slogin_fake_alloc),
        );
    }

    /// gp holds the game's value and the active byte at gp + 0x1ad0 starts
    /// at 0x80 | seed (never 1), so the store of 1 and its offset show.
    #[test]
    fn slogin_set_active_matches() {
        check(
            ee::s14::f_003f6c50__generated,
            slogin_set_active,
            32,
            &|c, seed| {
                let mut r = Rng(0x3f6c_50 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus
                    .write32(GAME_GP + login::ACTIVE_GP, 0x5a5a_5a80 | (seed & 0x7f));
                c.bus
                    .write32(GAME_GP + login::ACTIVE_GP + 4, 0xa5a5_0000 | seed);
                c.pc = 0x003f_6c50;
            },
        );
    }

    /// gp holds the game's value and the active byte at gp + 0x1ad0 starts
    /// non-zero (low byte 1..=0xff), so the store of 0 and its offset show.
    #[test]
    fn slogin_clear_active_matches() {
        check(
            ee::s14::f_003f6c60__generated,
            slogin_clear_active,
            32,
            &|c, seed| {
                let mut r = Rng(0x3f6c_60 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus
                    .write32(GAME_GP + login::ACTIVE_GP, 0x5a5a_5a00 | (seed % 0xff + 1));
                c.bus
                    .write32(GAME_GP + login::ACTIVE_GP + 4, 0xa5a5_0000 | seed);
                c.pc = 0x003f_6c60;
            },
        );
    }

    /// Independent digits: the allocation unwinds (seed % 2), the settings
    /// source sits at DATA + 0x1000 + [0, 1, 4][seed / 2 % 3] and the
    /// allocated state at DATA + 0x2000 + [0, 4][seed / 6 % 2]. Source and
    /// state both 4 mod 8 (seeds 10, 22) take the unaligned copy although
    /// their difference is 8-aligned, so `(source | state) & 7` and
    /// `(source ^ state) & 7` pick different loops there.
    #[test]
    fn slogin_alloc_state_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f6c70__generated,
            slogin_alloc_state,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6c_70, 1);
                c.pc = 0x003f_6c70;
                c.bus
                    .write32(LOG_UNWIND_AT, if seed % 2 == 1 { 0 } else { u32::MAX });
                let config = DATA + 0x300;
                let source = DATA + 0x1000 + [0, 1, 4][(seed / 2 % 3) as usize];
                let state = DATA + 0x2000 + [0, 4][(seed / 6 % 2) as usize];
                c.g[A0][0] = sx(config);
                c.bus.write32(config, source);
                c.bus.write32(config + 4, DATA + 0x4000);
                for index in 0..0x44 {
                    c.bus.write8(source + index, (index ^ seed) as u8);
                }
                c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
                c.bus.write32(ALLOC_CTRL + 4, 1);
                c.bus.write32(LOG_RESULTS, state);
            },
        );
    }

    /// A ready store (operation 4) or a busy one (7) is seed / 6 % 2, the
    /// requested operation seed % 6 (0-3 install the update callback, 4 and 5
    /// are refused by `SLogin_ResetForOp`). The login words at +0xc0..0x150
    /// start non-zero, so the reset's stores show.
    #[test]
    fn slogin_start_op_matches() {
        check(
            ee::s14::f_003f6e98__generated,
            slogin_start_op,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6e_98, 0);
                c.pc = 0x003f_6e98;
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = (seed % 6) as u64;
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0x200 + 0xc0 + 4 * i, 0x4c00_0000 | i << 8 | seed);
                }
                c.bus.write32(
                    DATA + 0x200 + login::OPERATION,
                    if seed / 6 % 2 == 0 { 4 } else { 7 },
                );
            },
        );
    }

    /// Null result pointer (seed % 2 == 0, error 7), operation [0, 4, 5][seed
    /// / 2 % 3] (4, the idle value `SLogin_FinishOp` leaves, and 5 are past
    /// 3: error 15, overwriting 7 when both) and step 9 (seed / 6 % 2: the
    /// 0xe8 callback, else 0xdc) are independent digits; the result word
    /// starts non-zero so the store of 0 shows. Operation 4 with a result
    /// pointer (seeds 3, 9, 15, 21) separates `< 4` from `<= 4`.
    #[test]
    fn slogin_op_update_matches() {
        check(
            ee::s14::f_003f6ee0__generated,
            slogin_op_update,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6e_e0, 1);
                c.pc = 0x003f_6ee0;
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x500) };
                c.bus.write32(DATA + 0x500, 0x5a5a_5a5a ^ seed);
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
                c.bus.write32(
                    DATA + 0x200 + login::OPERATION,
                    [0, 4, 5][(seed / 2 % 3) as usize],
                );
                c.bus.write32(
                    DATA + 0x200 + login::STEP,
                    if seed / 6 % 2 == 1 { 9 } else { 0 },
                );
                c.bus.write32(DATA + 0x200 + 0xdc, FAKE);
                c.bus.write32(DATA + 0x200 + 0xe8, FAKE + 0x10);
            },
        );
    }

    /// On the game's image (the step table at 0x497080: stages 0-5 run the
    /// 0xf0 call, the 0xf4 call, the 0xf8 call, error 17, the snapshot copy
    /// and error 16; stage 6 is past the table, error 17). Independent digits:
    /// null result pointer (seed % 2, error 7), step (seed / 2 % 3: 0, 9 ->
    /// error 13, 10 -> error 12), stage (seed / 6 % 7), the stage calls'
    /// result (seed / 42 % 2: a game function returning 0, or fakes returning
    /// non-zero), the 0xf4 call's completion word (seed / 84 % 2: 2 completes
    /// the step, else 1) and the fake call that unwinds (seed / 168 % 4: none,
    /// 0, 1, 2). The result word and the snapshot words at login + 0x120
    /// start non-zero, so the stores of 0/1/2 and the snapshot copy show.
    #[test]
    fn slogin_step_update_matches() {
        check(
            ee::s14::f_003f6f68__generated,
            slogin_step_update,
            672,
            &|c, seed| {
                elf_image(c);
                let _r = machine(c, seed, 0x3f6f_68, 4);
                c.pc = 0x003f_6f68;
                c.bus.write32(
                    LOG_UNWIND_AT,
                    [u32::MAX, 0, 1, 2][(seed / 168 % 4) as usize],
                );
                c.g[A0][0] = if seed % 2 == 1 { 0 } else { sx(DATA + 0x500) };
                c.bus.write32(DATA + 0x500, 0x5a5a_5a5a ^ seed);
                for i in 0..8u32 {
                    c.bus
                        .write32(DATA + 0x200 + 0x120 + 4 * i, 0x7c00_0000 | i << 8 | seed);
                }
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
                c.bus.write32(
                    DATA + 0x200 + login::STEP,
                    [0, 9, 10][(seed / 2 % 3) as usize],
                );
                c.bus.write32(DATA + 0x200 + login::OPERATION, 0);
                c.bus.write32(DATA + 0x200 + 0xec, seed / 6 % 7);
                let fails = seed / 42 % 2 == 1;
                for (i, slot) in [0xf0u32, 0xf4, 0xf8].into_iter().enumerate() {
                    c.bus.write32(
                        DATA + 0x200 + slot,
                        if fails {
                            FAKE + 0x10 * i as u32
                        } else {
                            0x0014_06c0
                        },
                    );
                }
                c.bus.write32(DATA + 0x200 + 0xfc, 0x7d00_0000 | seed);
                c.bus.write32(DATA + 0x200 + 0x100, 0x7e00_0000 | seed);
                c.bus.write32(DATA + 0x200 + 0xdc, FAKE + 0x30);
                c.bus.write32(DATA + 0x200 + 0xe0, FAKE + 0x40);
                c.bus.write32(DATA + 0x200 + 0xe4, FAKE + 0x50);
                c.bus.write32(
                    crate::difftest::SP - 0x30 + 4,
                    if seed / 84 % 2 == 1 { 2 } else { 1 },
                );
            },
        );
    }

    #[test]
    fn slogin_free_state_matches() {
        check(
            ee::s14::f_003f6d98__generated,
            slogin_free_state,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6d_98, 0);
                c.pc = 0x003f_6d98;
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x140, 0);
            },
        );
    }

    /// On the game's image (the 0x1a-byte template at 0x497060 and the
    /// store-error table are the game's), with the store already initialised
    /// (1 -> 0x13). The config copy's
    /// destination (the frame's first 0x1a bytes) starts with bit 7 set in
    /// every byte, so each copied byte (ASCII or the NUL) and its offset show.
    #[test]
    fn slogin_init_store_matches() {
        check(
            ee::s14::f_003f6dc8__generated,
            slogin_init_store,
            16,
            &|c, seed| {
                elf_image(c);
                let _r = machine(c, seed, 0x3f6d_c8, 0);
                c.pc = 0x003f_6dc8;
                for i in 0..0x20u32 {
                    c.bus.write8(
                        crate::difftest::SP - 0x50 + i,
                        (0x80 | (i * 7 + seed) & 0x7f) as u8,
                    );
                }
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + 0x140, DATA + 0x500);
                c.bus.write32(GAME_GP + 0x2a30, DATA + 0x800); // Store already initialized.
            },
        );
    }

    #[test]
    fn slogin_shutdown_store_matches() {
        check(
            ee::s14::f_003f6e70__generated,
            slogin_shutdown_store,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f6e_70, 0);
                c.pc = 0x003f_6e70;
                c.bus.write32(GAME_GP + 0x2a30, 0);
            },
        );
    }

    #[test]
    fn slogin_on_final_error_matches() {
        check(
            ee::s14::f_003f71f8__generated,
            slogin_on_final_error,
            24,
            &|c, seed| {
                step_result_fixture(c, seed, 0x003f_71f8);
            },
        );
    }

    /// On the game's image, so the real `SLogin_SetStep` (step 7) runs on the
    /// game's tables and returns through its own frame: the reload of `ra`
    /// after it shows. The result word, the error word and the login words
    /// at +0xd0..0x150 start non-zero, so every store shows.
    #[test]
    fn slogin_on_step_error_matches() {
        check(
            ee::s14::f_003f71b8__generated,
            slogin_on_step_error,
            24,
            &|c, seed| {
                elf_image(c);
                step_result_fixture(c, seed, 0x003f_71b8);
                for i in 0..0x20u32 {
                    c.bus
                        .write32(DATA + 0x200 + 0xd0 + 4 * i, 0x6b00_0000 | i << 8 | seed);
                }
                c.bus.write32(DATA + 0x500, 0x5a5a_5a5a ^ seed);
                c.bus.write32(DATA + 0x504, seed ^ 0x7777);
            },
        );
    }

    #[test]
    fn slogin_on_final_done_matches() {
        check(
            ee::s14::f_003f72c8__generated,
            slogin_on_final_done,
            24,
            &|c, seed| {
                step_result_fixture(c, seed, 0x003f_72c8);
            },
        );
    }

    /// The game's step table at 0x4970a0 (step 0 -> next 1, 1 -> next 2 for
    /// operations 0/1, 4 for operation 2, error 12 otherwise, 2 -> 3, 3 -> 6,
    /// 4 -> 5, 5 -> 6) and the real `SLogin_SetStep` with the game's tables:
    /// step = seed % 7 (6 is past the table: error 12), operation = seed / 7
    /// % 4 crosses every step. The result and error words start non-zero.
    #[test]
    fn slogin_on_step_done_matches() {
        check(
            ee::s14::f_003f7218__generated,
            slogin_on_step_done,
            56,
            &|c, seed| {
                elf_image(c);
                step_result_fixture(c, seed, 0x003f_7218);
                for i in 0..0x20u32 {
                    c.bus
                        .write32(DATA + 0x200 + 0xd0 + 4 * i, 0x6b00_0000 | i << 8 | seed);
                }
                c.bus.write32(DATA + 0x500, 0x5a5a_5a5a);
                c.bus.write32(DATA + 0x504, 0xa5a5_a5a5);
                c.bus.write32(DATA + 0x200 + login::STEP, seed % 7);
                c.bus.write32(DATA + 0x200 + login::OPERATION, seed / 7 % 4);
            },
        );
    }

    #[test]
    fn slogin_on_error_close_done_matches() {
        check(
            ee::s14::f_003f72f8__generated,
            slogin_on_error_close_done,
            24,
            &|c, seed| {
                step_result_fixture(c, seed, 0x003f_72f8);
            },
        );
    }

    /// On the game's image, so the real `SLogin_SetStep` runs on the game's
    /// tables and returns through its own frame (the reload of `ra` after it
    /// shows). Independent digits: step (seed % 2: 9 is ready, 8 is not) and
    /// operation (seed / 2 % 5: 0 starts step 0, 1 and 2 step 1, 3 step 8, 4
    /// is refused with error 12). The result and error words and the login
    /// words at +0xd0..0x150 start non-zero, so every store shows.
    #[test]
    fn slogin_begin_sequence_matches() {
        check(
            ee::s14::f_003f7328__generated,
            slogin_begin_sequence,
            30,
            &|c, seed| {
                elf_image(c);
                step_result_fixture(c, seed, 0x003f_7328);
                for i in 0..0x20u32 {
                    c.bus
                        .write32(DATA + 0x200 + 0xd0 + 4 * i, 0x6b00_0000 | i << 8 | seed);
                }
                c.bus.write32(DATA + 0x500, 0x5a5a_5a5a ^ seed);
                c.bus.write32(DATA + 0x504, 0xa5a5_a5a5 ^ seed);
                c.bus.write32(
                    DATA + 0x200 + login::STEP,
                    if seed % 2 == 1 { 9 } else { 8 },
                );
                c.bus.write32(DATA + 0x200 + login::OPERATION, seed / 2 % 5);
            },
        );
    }

    /// Login at DATA + 0x200 with an initialised store and no memory-card
    /// manager. The login's +0xfc callback slot and +0x100 word start
    /// non-zero, so the stores of 0 (and the callback store) show.
    pub(crate) fn file_step_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
        c.bus.write32(GAME_GP + 0x2a30, DATA + 0x800);
        c.bus.write32(GAME_GP + 0x1aec, 0); // Memory-card manager not initialized.
        c.bus.write32(DATA + 0x200 + 0xfc, 0x6b6b_0000 | seed);
        c.bus.write32(DATA + 0x200 + 0x100, 0x6c6c_0000 | seed);
    }

    #[test]
    fn slogin_step_create_matches() {
        check(
            ee::s14::f_003f73a8__generated,
            slogin_step_create,
            16,
            &|c, seed| {
                file_step_fixture(c, seed, 0x003f_73a8);
            },
        );
    }

    #[test]
    fn slogin_step_open_matches() {
        check(
            ee::s14::f_003f73f0__generated,
            slogin_step_open,
            16,
            &|c, seed| {
                file_step_fixture(c, seed, 0x003f_73f0);
            },
        );
    }

    #[test]
    fn slogin_step_delete_matches() {
        check(
            ee::s14::f_003f7438__generated,
            slogin_step_delete,
            16,
            &|c, seed| {
                file_step_fixture(c, seed, 0x003f_7438);
            },
        );
    }

    #[test]
    fn slogin_step_write_acct_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f7478__generated,
            slogin_step_write_acct,
            24,
            &|c, seed| {
                write_record_step_fixture(c, seed, 0x003f_7478);
            },
        );
    }

    #[test]
    fn slogin_step_write_pass_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f7528__generated,
            slogin_step_write_pass,
            24,
            &|c, seed| {
                write_record_step_fixture(c, seed, 0x003f_7528);
            },
        );
    }

    pub(crate) const LOGIN_TEST_MAGIC_AT: u32 = DATA + 0x3f0;

    pub(crate) const LOGIN_TEST_MAGIC: u32 = 0x53_4c_4f_47;

    pub(crate) static PREV_LOGIN_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn login_free_fake(c: &mut Cpu) {
        if c.bus.read32(LOGIN_TEST_MAGIC_AT) != LOGIN_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_LOGIN_FREE, ee::s07::f_00266760__generated);
        }
        slogin_arm_unwind(c, callee::TIB_EXT_MEM_FREE.addr);
        fakes::fake_call(c, 0x0026_6760);
    }

    pub(crate) fn login_mc_poll_fake(c: &mut Cpu, target: u32) {
        let out_status = c.g[A0][0] as u32;
        let out_detail = c.g[A1][0] as u32;
        fakes::fake_call(c, target);
        if target == fakes::FAKE + 0x50 {
            let mode = c.bus.read32(DATA + 0x3ec);
            c.bus.write32(out_status, mode);
            c.bus.write32(out_detail, 0);
            c.g[V0][0] = 0;
        } else if target == fakes::FAKE + 0x60 || target == fakes::FAKE + 0x70 {
            c.g[V0][0] = 0;
        }
    }

    /// The login at DATA with an account record block at DATA + 0x800, the
    /// shared allocation fake (result switch seed % 4: 0 fails) and the gated
    /// `TibExtMemFree` fake; the harness unwinds at fake call seed % 6 when
    /// that is below 3.
    pub(crate) fn login_store_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        hook_slogin_alloc();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            login_free_fake,
            &PREV_LOGIN_FREE,
        );
        let _r = fakes::machine(c, seed, entry as u64, 5);
        c.pc = entry;
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed % 6 < 3 { seed % 6 } else { u32::MAX },
        );
        c.bus.write32(LOGIN_TEST_MAGIC_AT, LOGIN_TEST_MAGIC);
        c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
        c.bus.write32(ALLOC_CTRL + 4, seed % 4);
        c.bus.write32(GAME_GP + login::STATE_GP, DATA);
        c.bus.write32(DATA + 0x140, 0x11);
        c.bus.write32(DATA + 0x144, DATA + 0x800);
        c.bus.write32(DATA + 0xcc, DATA + 0xa00);
        for i in 0..17u32 {
            c.bus.write8(DATA + 0x800 + i, (0x41 + i) as u8);
        }
        c.bus.write32(DATA + 0x44, 0);
        c.bus.write32(DATA + 0x48, 0);
        c.bus.write32(DATA + 0x4c, 0);
    }

    #[test]
    fn slogin_step_read_acct_matches() {
        check(
            ee::s14::f_003f75d8__generated,
            slogin_step_read_acct,
            60,
            &|c, seed| {
                step_read_record_fixture(c, seed, 0x003f_75d8);
            },
        );
    }

    #[test]
    fn slogin_step_read_pass_matches() {
        check(
            ee::s14::f_003f7658__generated,
            slogin_step_read_pass,
            60,
            &|c, seed| {
                step_read_record_fixture(c, seed, 0x003f_7658);
            },
        );
    }

    /// The login's +0xfc and +0x100 words (DATA + 0xfc / + 0x100) start
    /// non-zero, so both stores of 0 show.
    #[test]
    fn slogin_step_close_matches() {
        check(
            ee::s14::f_003f76d8__generated,
            slogin_step_close,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_76d8);
                c.bus.write32(DATA + 0xfc, 0x7b7b_0000 | seed);
                c.bus.write32(DATA + 0x100, 0x7c7c_0000 | seed);
            },
        );
    }

    /// `step_poll_fixture` with arm = seed % 8 and the login + 0x100 callback
    /// seed / 8 % 3, crossing arms 0 and 1.
    #[test]
    fn slogin_step_poll_matches() {
        check(
            ee::s14::f_003f7708__generated,
            slogin_step_poll,
            120,
            &|c, seed| {
                step_poll_fixture(c, seed, seed % 8, seed / 8 % 3);
            },
        );
    }

    #[test]
    fn slogin_fetch_store_info_matches() {
        check(
            ee::s14::f_003f77f8__generated,
            slogin_fetch_store_info,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_77f8);
            },
        );
    }

    #[test]
    fn slogin_on_open_done_matches() {
        check(
            ee::s14::f_003f7820__generated,
            slogin_on_open_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7820);
            },
        );
    }

    /// The 17-byte record copy lands on a non-zero-filled login field buffer
    /// (DATA + 0xa00, account at +0, password at +17).
    #[test]
    fn slogin_on_read_acct_done_matches() {
        check(
            ee::s14::f_003f7840__generated,
            slogin_on_read_acct_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7840);
                for i in 0..0x30u32 {
                    c.bus
                        .write8(DATA + 0xa00 + i, (0xb0 + (i + seed) % 0x40) as u8);
                }
            },
        );
    }

    /// As the account test, for the password half (DATA + 0xa00 + 17).
    #[test]
    fn slogin_on_read_pass_done_matches() {
        check(
            ee::s14::f_003f7890__generated,
            slogin_on_read_pass_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7890);
                for i in 0..0x30u32 {
                    c.bus
                        .write8(DATA + 0xa00 + i, (0xb0 + (i + seed) % 0x40) as u8);
                }
            },
        );
    }

    #[test]
    fn slogin_on_write_acct_done_matches() {
        check(
            ee::s14::f_003f78e0__generated,
            slogin_on_write_acct_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_78e0);
            },
        );
    }

    #[test]
    fn slogin_on_write_pass_done_matches() {
        check(
            ee::s14::f_003f7900__generated,
            slogin_on_write_pass_done,
            60,
            &|c, seed| {
                login_store_fixture(c, seed, 0x003f_7900);
            },
        );
    }

    #[test]
    fn slogin_free_acct_record_matches() {
        fakes::covered(
            ee::s14::f_003f7920__generated,
            slogin_free_acct_record,
            60,
            &|c, seed| {
                free_record_fixture(c, seed, 0x003f_7920);
            },
        );
    }

    #[test]
    fn slogin_free_pass_record_matches() {
        fakes::covered(
            ee::s14::f_003f7958__generated,
            slogin_free_pass_record,
            60,
            &|c, seed| {
                free_record_fixture(c, seed, 0x003f_7958);
            },
        );
    }

    /// The login words at DATA + 0xc0..0x150 start non-zero, so every
    /// field store (zeros included) shows.
    #[test]
    fn slogin_finish_op_matches() {
        check(
            ee::s14::f_003f7a30__generated,
            slogin_finish_op,
            60,
            &|c, seed| {
                login_state_fixture(c, seed, 0x003f_7a30);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0xc0 + 4 * i, 0x4c00_0000 | i << 8 | seed);
                }
            },
        );
    }

    /// Step = seed % 11 through the game's done/error/step tables (0x4970c0,
    /// 0x4970f0, 0x497120, loaded from the image), keep flag = [0, 1, 0x100,
    /// 0x1ff][seed / 11 % 4] (0x100 has a zero low byte: no copy). The words
    /// at login + 0xd8..0x140 start non-zero, so the saved copy at 0x10c and
    /// the callback stores show. Cycle totals (with `SLogin_InitStepFuncs`):
    /// step past the table 71 (copy 93); steps 0-6 and 8 132 (copy 154);
    /// step 7 (third error target, one cycle less) 131 (copy 153).
    #[test]
    fn slogin_set_step_matches() {
        check_paths(
            ee::s14::f_003f7a60__generated,
            slogin_set_step,
            132,
            &[71, 93, 131, 132, 153, 154],
            &|c, seed| {
                elf_image(c);
                login_state_fixture(c, seed, 0x003f_7a60);
                c.g[A0][0] = (seed % 11) as u64;
                c.g[A1][0] = [0, 1, 0x100, 0x1ff][(seed / 11 % 4) as usize];
                for i in 0..0x1au32 {
                    c.bus
                        .write32(DATA + 0xd8 + 4 * i, 0x7654_0000 + (i << 8) + seed);
                }
            },
        );
    }

    /// Step = seed % 11 through the game's step table at 0x497120 (loaded
    /// from the image); the callback words and the snapshot at login + 0x120
    /// start non-zero. Cycle totals: step past the table 51, any table step
    /// 78 (all eight targets cost the same; each is the only word for its
    /// steps, so seed % 11 reaches all of them).
    #[test]
    fn slogin_init_step_funcs_matches() {
        check_paths(
            ee::s14::f_003f7c00__generated,
            slogin_init_step_funcs,
            110,
            &[51, 78],
            &|c, seed| {
                elf_image(c);
                login_state_fixture(c, seed, 0x003f_7c00);
                for i in 0..0x14u32 {
                    c.bus
                        .write32(DATA + 0xec + 4 * i, 0x6100_0000 + (i << 8) + seed);
                }
                c.bus.write32(DATA + 0xd8, seed % 11);
            },
        );
    }

    #[test]
    fn slogin_init_step_funcs_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f7c00__generated,
            slogin_init_step_funcs,
            90,
            &|c, seed| {
                elf_image(c);
                login_state_fixture(c, seed, 0x003f_7c00);
                c.bus.write32(DATA + 0xd8, seed % 11);
                let mut r = Rng(0x003f_7c00 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_7c00)
            },
        );
    }

    pub(crate) fn checksum_method_fake(c: &mut Cpu, target: u32) {
        let mode = c.bus.read32(DATA + 0x3e4);
        let output_slot = c.g[A0][0] as u32;
        slogin_arm_unwind(c, target);
        fakes::fake_call(c, target);
        let status = match target {
            x if x == fakes::FAKE + 0x10 => (mode == 0) as u64,
            x if x == fakes::FAKE + 0x20 => (mode == 1) as u64,
            x if x == fakes::FAKE + 0x30 => (mode == 2) as u64,
            _ => return,
        };
        if target == fakes::FAKE + 0x30 && status == 0 {
            c.bus.write32(output_slot, DATA + 0x1600);
        }
        c.g[V0][0] = status;
    }

    /// The checksum interface's three methods are fakes; mode = seed % 5: the
    /// first, second or third method fails (0x18), the digest matches (0) or
    /// differs (0x15). With the pass-through fakes on, the unwinding call is
    /// seed / 5 % 5: none, `Crc32_GetInterface`, or the first, second or third
    /// method (each crossed with every mode that reaches it).
    #[test]
    fn slogin_verify_icon_checksum_matches() {
        fakes::covered(
            ee::s14::f_003f7de0__generated,
            slogin_verify_icon_checksum,
            100,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_7de0, 3);
                c.pc = 0x003f_7de0;
                c.hooks.miss = checksum_method_fake;
                slogin_unwind_at(
                    c,
                    [
                        0,
                        callee::CRC32_GET_INTERFACE.addr,
                        fakes::FAKE + 0x10,
                        fakes::FAKE + 0x20,
                        fakes::FAKE + 0x30,
                    ][(seed / 5 % 5) as usize],
                );
                c.bus.write32(DATA + 0x3e4, seed % 5);
                c.g[A0][0] = (DATA + 0x1200) as u64;
                c.bus.write32(0x0049_7508 + 0xc, fakes::FAKE + 0x10);
                c.bus.write32(0x0049_7508 + 0x10, fakes::FAKE + 0x20);
                c.bus.write32(0x0049_7508 + 0x14, fakes::FAKE + 0x30);
                c.bus.write32(DATA + 0x1604, DATA + 0x1800);
                c.bus.write32(
                    DATA + 0x1800,
                    if seed % 5 == 3 {
                        0x0836_c71d
                    } else {
                        0x1234_5678
                    },
                );
            },
        );
    }

    /// The login words at DATA + 0xc0..0x150 start non-zero, so every
    /// field store (zeros included) shows.
    #[test]
    fn slogin_end_sequence_matches() {
        check(
            ee::s14::f_003f7bd0__generated,
            slogin_end_sequence,
            60,
            &|c, seed| {
                login_state_fixture(c, seed, 0x003f_7bd0);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0xc0 + 4 * i, 0x4c00_0000 | i << 8 | seed);
                }
            },
        );
    }

    pub(crate) fn login_state_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x3f75_0000 ^ seed as u64 ^ entry as u64);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + login::STATE_GP, DATA);
    }

    /// The login words at DATA + 0xc0..0x150 start non-zero, so every
    /// field store (zeros included) shows. Operation = seed % 6.
    #[test]
    fn slogin_reset_for_op_matches() {
        check(
            ee::s14::f_003f7990__generated,
            slogin_reset_for_op,
            60,
            &|c, seed| {
                login_state_fixture(c, seed, 0x003f_7990);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0xc0 + 4 * i, 0x4c00_0000 | i << 8 | seed);
                }
                c.g[A0][0] = (seed % 6) as u64;
            },
        );
    }

    /// The login words at DATA + 0xc0..0x150 start non-zero, so every
    /// field store (zeros included) shows.
    #[test]
    fn slogin_clear_step_funcs_matches() {
        check(
            ee::s14::f_003f7d98__generated,
            slogin_clear_step_funcs,
            40,
            &|c, seed| {
                login_state_fixture(c, seed, 0x003f_7d98);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0xc0 + 4 * i, 0x4c00_0000 | i << 8 | seed);
                }
            },
        );
    }

    #[test]
    fn slogin_clear_step_funcs_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f7d98__generated,
            slogin_clear_step_funcs,
            90,
            &|c, seed| {
                login_state_fixture(c, seed, 0x003f_7d98);
                let mut r = Rng(0x003f_7d98 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_7d98)
            },
        );
    }

    /// seed % 7: null source, null destination, zero count, source ==
    /// destination, then counts 1, 4 and 16 over an invented 7-letter name;
    /// the destination starts non-zero so the terminator stores show.
    #[test]
    fn slogin_str_copy_n_matches() {
        check(
            ee::s14::f_003f7e90__generated,
            slogin_str_copy_n,
            120,
            &|c, seed| {
                let mut r = Rng(0x003f_7e90 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_7e90;
                let dst = DATA + 0x400;
                let src = DATA + 0x600;
                for (i, byte) in b"alice42\0".iter().enumerate() {
                    c.bus.write8(src + i as u32, *byte);
                }
                for i in 0..0x14u32 {
                    c.bus.write8(dst + i, (0xd0 + i) as u8);
                }
                match seed % 7 {
                    0 => {
                        c.g[A1][0] = 0;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 8;
                    }
                    1 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = 0;
                        c.g[A2][0] = 8;
                    }
                    2 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 0;
                    }
                    3 => {
                        c.g[A1][0] = dst as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 8;
                    }
                    4 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 1;
                    }
                    5 => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 4;
                    }
                    _ => {
                        c.g[A1][0] = src as u64;
                        c.g[A0][0] = dst as u64;
                        c.g[A2][0] = 16;
                    }
                }
            },
        );
    }

    /// On the game's image (the store-error table at 0x497150 is the game's:
    /// word 1 = 0x3f81d4), with the store already up: SetActive, AllocState
    /// (the shared allocation fake), then InitStore returns 1 -> 0x13.
    #[test]
    fn slogin_init_matches() {
        hook_slogin_alloc();
        check(
            ee::s14::f_003f7ee0__generated,
            slogin_init,
            30,
            &|c, seed| {
                elf_image(c);
                let mut r = fakes::machine(c, seed, 0x3f7e_e000, 1);
                c.pc = 0x003f_7ee0;
                c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
                c.bus.write32(ALLOC_CTRL + 4, 1);
                c.g[A0][0] = sx(DATA + 0x200);
                c.bus.write32(DATA + 0x200, DATA + 0x300);
                c.bus.write32(DATA + 0x204, 0);
                for i in 0..16 {
                    c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
                }
                c.bus.write32(GAME_GP + 0x2a30, DATA + 0x500); // store already up
            },
        );
    }

    /// mode = seed % 3: no icon and no fields (7), fields only, icon and
    /// fields; the checksum methods are plain fakes (never 0), so the icon
    /// check fails (0x18). The login's field buffers start non-zero.
    #[test]
    fn slogin_create_matches() {
        check(
            ee::s14::f_003f7f48__generated,
            slogin_create,
            60,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f7f_4800, 3);
                c.pc = 0x003f_7f48;
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                for i in 0..0x24u32 {
                    c.bus.write8(DATA + 0x400 + 0xa8 + i, (0xe0 + i) as u8);
                }
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.bus.write32(0x0049_7508 + 0x0c, FAKE);
                c.bus.write32(0x0049_7508 + 0x10, FAKE);
                c.bus.write32(0x0049_7508 + 0x14, FAKE);
                let mode = seed % 3;
                c.g[A0][0] = if mode == 2 { sx(DATA + 0xa00) } else { 0 };
                c.g[A1][0] = if mode == 0 { 0 } else { sx(DATA + 0x800) };
                for (i, b) in b"wren42\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + i as u32, *b);
                }
                for (i, b) in b"pw1234\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x811 + i as u32, *b);
                }
            },
        );
    }

    pub(crate) fn create_checksum_fake(c: &mut Cpu, target: u32) {
        let mode = c.bus.read32(DATA + 0x3e4);
        let output_slot = c.g[A0][0] as u32;
        fakes::fake_call(c, target);
        let status = match target {
            x if x == FAKE + 0x10 => (mode == 1) as u64,
            x if x == FAKE + 0x20 => 0,
            x if x == FAKE + 0x30 => 0,
            _ => return,
        };
        if target == FAKE + 0x30 {
            c.bus.write32(output_slot, DATA + 0x1600);
        }
        c.g[V0][0] = status;
    }

    /// On the game's image (icon.sys templates, title and store-error table
    /// are the game's): mode = seed % 3 is a good icon, a failing first
    /// checksum method (0x18) or a checksum mismatch (0x15). store = seed / 3
    /// % 3 crosses it: store and card interface up (good icon: icon.sys
    /// installed, 0, then `SLogin_StartOp`: operation 0 is not idle, 0x16),
    /// store not initialised (good icon: SetupIconSys 2 -> 0x14, the
    /// setup != 0 return) or store up without a card interface (good icon:
    /// card error -> 0x18, the setup != 0 return). The login's field buffers
    /// start non-zero.
    #[test]
    fn slogin_create_checksum_success_matches() {
        check(
            ee::s14::f_003f7f48__generated,
            slogin_create,
            18,
            &|c, seed| {
                elf_image(c);
                let _r = fakes::machine(c, seed, 0x3f7f_48ee, 0);
                let mode = seed % 3;
                let store = seed / 3 % 3;
                c.pc = 0x003f_7f48;
                c.hooks.miss = create_checksum_fake;
                c.bus.write32(DATA + 0x3e4, mode);
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                for i in 0..0x24u32 {
                    c.bus.write8(DATA + 0x400 + 0xa8 + i, (0xe0 + i) as u8);
                }
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.bus
                    .write32(GAME_GP + 0x2a30, if store == 1 { 0 } else { DATA + 0x1700 });
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if store == 2 { 0 } else { DATA + 0x1800 },
                );
                c.bus.write32(0x0049_7508 + 0x0c, FAKE + 0x10);
                c.bus.write32(0x0049_7508 + 0x10, FAKE + 0x20);
                c.bus.write32(0x0049_7508 + 0x14, FAKE + 0x30);
                c.bus.write32(DATA + 0x1604, DATA + 0x1900);
                c.bus
                    .write32(DATA + 0x1900, if mode == 2 { 0 } else { 0x0836_c71d });
                c.g[A0][0] = sx(DATA + 0xa00);
                c.g[A1][0] = sx(DATA + 0x800);
                for (i, b) in b"wren42\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + i as u32, *b);
                }
                for (i, b) in b"pw1234\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x811 + i as u32, *b);
                }
            },
        );
    }

    pub(crate) fn slogin_poll_fake(c: &mut Cpu, target: u32) {
        if target != FAKE || c.bus.read32(DATA + 0x100) == 2 {
            return fakes::fake_call(c, target);
        }
        let mode = c.bus.read32(DATA + 0x100);
        c.bus
            .write32(c.g[A0][0] as u32, if mode == 3 { 2 } else { 1 });
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
    }

    /// mode = seed % 5: null state pointer, idle operation (4), step error,
    /// step complete (the step fake writes 2), step running (writes 1). The state
    /// word and the login words at DATA + 0x4c0..0x550 start non-zero, so the
    /// store of 0 and the finish stores show.
    #[test]
    fn slogin_poll_matches() {
        check(
            ee::s14::f_003f80e0__generated,
            slogin_poll,
            60,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_e000, 1);
                let mode = seed % 5;
                c.hooks.miss = slogin_poll_fake;
                c.pc = 0x003f_80e0;
                c.bus.write32(DATA + 0x100, mode);
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0x400 + 0xc0 + 4 * i, 0x5000_0000 | i << 8 | seed);
                }
                c.bus
                    .write32(DATA + 0x400 + 0xd0, if mode == 1 { 4 } else { 0 });
                c.bus.write32(DATA + 0x400 + 0xd4, FAKE);
                c.bus.write32(DATA + 0x200, 0x5a5a_0000 | seed);
                c.g[A0][0] = if mode == 0 { 0 } else { sx(DATA + 0x200) };
            },
        );
    }

    #[test]
    fn slogin_shutdown_matches() {
        check(
            ee::s14::f_003f7f18__generated,
            slogin_shutdown,
            30,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f7f_1800, 0);
                // The store-error table MapStoreError dispatches through is the game's.
                elf_image(c);
                c.pc = 0x003f_7f18;
                c.bus.write32(GAME_GP + 0x2a30, 0); // no open store
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0x140, 0); // no allocated owner block
            },
        );
    }

    #[test]
    fn slogin_load_matches() {
        check(
            ee::s14::f_003f8000__generated,
            slogin_load,
            48,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_0000, 0);
                c.pc = 0x003f_8000;
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x800) };
            },
        );
    }

    #[test]
    fn slogin_delete_matches() {
        check(
            ee::s14::f_003f80c0__generated,
            slogin_delete,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_c000, 0);
                c.pc = 0x003f_80c0;
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                c.bus.write32(DATA + 0x400 + 0xd0, seed % 4);
            },
        );
    }

    /// No fields (seed % 3 == 0, 7) or invented account and password copied
    /// into the login's non-zero-filled field buffers, then `SLogin_StartOp`.
    #[test]
    fn slogin_update_matches() {
        check(
            ee::s14::f_003f8048__generated,
            slogin_update,
            48,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x3f80_4800, 0);
                c.pc = 0x003f_8048;
                c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x400);
                for i in 0..0x24u32 {
                    c.bus.write8(DATA + 0x400 + 0xa8 + i, (0xe0 + i) as u8);
                }
                c.bus.write32(DATA + 0x400 + 0xd0, 0);
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x800) };
                for (i, b) in b"wren42\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x800 + i as u32, *b);
                }
                for (i, b) in b"pw1234\0".iter().enumerate() {
                    c.bus.write8(DATA + 0x811 + i as u32, *b);
                }
            },
        );
    }

    /// Every error code against the game's own table at 0x497150 (seed %
    /// 0x78 walks all 0x73 entries, which hold all twelve targets, and five
    /// codes past the end); the second pass sets the upper word on the
    /// out-of-range codes. Cycle totals: out of range 5, any table target 11
    /// (all twelve targets cost the same, so the totals only tell the range
    /// check apart; the walk over every index reaches each target).
    #[test]
    fn slogin_map_store_error_matches() {
        check_paths(
            ee::s14::f_003f8178__generated,
            slogin_map_store_error,
            240,
            &[5, 11],
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x3f81_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8178;
                let error = (seed % 0x78) as u64;
                c.g[A0][0] = if error >= 0x73 && seed / 0x78 % 2 == 1 {
                    error | 0xffff_ffff_0000_0000
                } else {
                    error
                };
            },
        );
    }

    #[test]
    fn slogin_get_record_alloc_size_matches() {
        check(
            ee::s14::f_003f8230__generated,
            slogin_get_record_alloc_size,
            40,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3f82_3000, 0);
                c.pc = 0x003f_8230;
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { DATA as u64 };
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn slogin_mc_first_device_in_mask_matches() {
        check(
            ee::s14::f_003f8550__generated,
            slogin_mc_first_device_in_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x3f85_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8550;
                c.g[A0][0] = if seed % 9 == 0 { 0 } else { 1u64 << (seed % 8) };
            },
        );
    }

    #[test]
    fn slogin_mc_last_device_in_mask_matches() {
        check(
            ee::s14::f_003f8588__generated,
            slogin_mc_last_device_in_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x3f85_8800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8588;
                c.g[A0][0] = if seed % 9 == 0 { 0 } else { 1u64 << (seed % 8) };
            },
        );
    }

    #[test]
    fn slogin_mc_count_devices_in_mask_matches() {
        check(
            ee::s14::f_003f86b8__generated,
            slogin_mc_count_devices_in_mask,
            64,
            &|c, seed| {
                let mut r = Rng(0x3f86_b800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_86b8;
                c.g[A0][0] = ((seed * 0x31) & 0xff) as u64;
            },
        );
    }

    #[test]
    fn slogin_mc_next_device_in_mask_matches() {
        check(
            ee::s14::f_003f85c0__generated,
            slogin_mc_next_device_in_mask,
            300,
            &slogin_mc_next_device_in_mask_fixture,
        );
        // Forward (seed % 3 == 0): next index past the end (16), its bit set
        // (18), scan exhausted at once (27) and after a back edge (35), found on
        // the first step (32) and after a back edge (40).
        require_paths(
            ee::s14::f_003f85c0__generated,
            100,
            &[16, 18, 27, 32, 35, 40],
            &|c, s| slogin_mc_next_device_in_mask_fixture(c, 3 * s),
        );
        // Backward: next index below 0 (18), its bit set (20), scan exhausted at
        // once (30) and after a back edge (37), found on the first step (34) and
        // after a back edge (41).
        require_paths(
            ee::s14::f_003f85c0__generated,
            100,
            &[18, 20, 30, 34, 37, 41],
            &|c, s| slogin_mc_next_device_in_mask_fixture(c, 3 * s + 1),
        );
        // Neither direction (9).
        require_paths(ee::s14::f_003f85c0__generated, 100, &[9], &|c, s| {
            slogin_mc_next_device_in_mask_fixture(c, 3 * s + 2)
        });
    }

    /// `seed % 7` picks the card state word: 0, 10, 14 and 0xffffffff store the
    /// error, the reserved states 11..13 keep the old one. Both arms cost 8
    /// cycles; the pre-filled error word tells them apart.
    #[test]
    fn slogin_mc_set_error_matches() {
        check(
            ee::s14::f_003f86f0__generated,
            slogin_mc_set_error,
            70,
            &|c, seed| {
                let mut r = Rng(0x3f86_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_86f0;
                c.g[GP][0] = GAME_GP as u64;
                let state = DATA + 0x400 + seed / 7 % 4 * 0x40;
                c.g[A0][0] = (r.next_u32() as u64) << 32 | (seed + 100) as u64;
                c.bus.write32(GAME_GP + card::STATE_GP, state);
                c.bus.write32(
                    state,
                    [0, 10, 11, 12, 13, 14, u32::MAX][(seed % 7) as usize],
                );
                c.bus.write32(state + 4, 0x4444_4444);
                c.bus.write32(state + card::ERROR, 0xa5a5_a5a5);
                c.bus.write32(state + 0xc, 0x5a5a_5a5a);
            },
        );
    }

    /// The card state moves with `seed % 4`; the words around the error differ,
    /// and `seed / 4 % 2` sets the error's sign bit (the load sign-extends).
    #[test]
    fn slogin_mc_get_error_matches() {
        check(
            ee::s14::f_003f8710__generated,
            slogin_mc_get_error,
            40,
            &|c, seed| {
                let mut r = Rng(0x3f87_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8710;
                c.g[GP][0] = GAME_GP as u64;
                let state = DATA + 0x400 + seed % 4 * 0x40;
                c.bus.write32(GAME_GP + card::STATE_GP, state);
                for i in 0..8u32 {
                    c.bus.write32(state + 4 * i, 0x1111_1111 * (i + 1));
                }
                let error = if seed / 4 % 2 == 1 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(state + card::ERROR, error);
            },
        );
    }

    pub(crate) fn mc_abort_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        entry: u32,
        callback_offset: u32,
    ) {
        let mut r = fakes::machine(c, seed, salt, 1);
        c.pc = entry;
        c.g[A1][0] = sx(DATA + 0x200);
        c.bus.write32(DATA + 0x200, r.next_u32());
        c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x400);
        c.bus.write32(GAME_GP + card::IFACE_GP, DATA + 0x500);
        c.bus.write32(DATA + 0x400, r.next_u32());
        c.bus
            .write32(DATA + 0x400 + card::FILE_HANDLE, DATA + 0x800);
        c.bus.write32(DATA + 0x500 + callback_offset, FAKE);
    }

    #[test]
    fn slogin_mc_abort_at_flush_matches() {
        fakes::covered(
            ee::s14::f_003f8720__generated,
            slogin_mc_abort_at_flush,
            60,
            &|c, seed| {
                mc_abort_fixture(c, seed, 0x3f87_2000, 0x003f_8720, 0x38);
            },
        );
    }

    #[test]
    fn slogin_mc_abort_at_close_matches() {
        fakes::covered(
            ee::s14::f_003f8768__generated,
            slogin_mc_abort_at_close,
            60,
            &|c, seed| {
                mc_abort_fixture(c, seed, 0x3f87_6800, 0x003f_8768, 0x1c);
            },
        );
    }

    #[test]
    fn slogin_mc_abort_at_finish_matches() {
        fakes::covered(
            ee::s14::f_003f87b0__generated,
            slogin_mc_abort_at_finish,
            60,
            &|c, seed| {
                mc_abort_fixture(c, seed, 0x3f87_b000, 0x003f_87b0, 0x14);
            },
        );
    }

    /// On the game's image (icon.sys templates, title and the store-error
    /// table are the game's): icon = seed % 2 (null -> 7); store = seed / 2 %
    /// 3: not initialised (2 -> 0x14), initialised without a card manager
    /// (card error 2 -> 102 -> 0x18), or with one (icon.sys installed into the
    /// card interface, 0). The frame below SP - 0x100 and the card
    /// interface's icon.sys copy start non-zero, so both clears and the copy
    /// show.
    #[test]
    fn slogin_setup_icon_sys_matches() {
        check(
            ee::s14::f_003f8258__generated,
            slogin_setup_icon_sys,
            24,
            &|c, seed| {
                elf_image(c);
                let _r = fakes::machine(c, seed, 0x3f82_5800, 0);
                c.pc = 0x003f_8258;
                for i in 0..0x110u32 {
                    c.bus.write32(
                        crate::difftest::SP - 0x540 + 4 * i,
                        0x1c00_0000 | i << 8 | seed,
                    );
                }
                for i in 0..0x110u32 {
                    c.bus
                        .write32(DATA + 0x1800 + 4 * i, 0x1d00_0000 | i << 8 | seed);
                }
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x1000) };
                let store = seed / 2 % 3;
                c.bus
                    .write32(GAME_GP + 0x2a30, if store == 0 { 0 } else { DATA + 0x1700 });
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if store == 2 { DATA + 0x1800 } else { 0 },
                );
            },
        );
    }

    /// Every steering value crossed (`mc_open_step_fixture`, 864 seeds) with
    /// the machine's random unwind at the one fake call a path makes. The path
    /// proofs run with unwinds off, one phase (or phase and a0) per slice, so
    /// each listed total comes from one arm within its slice (state 6 costs one
    /// cycle more than state 3; 27 is a state other than 3 and 6):
    /// - phase 0: 38/39 (`*done` = 2, 0x12);
    /// - phase 1, a0 3: 45 state 6 != a0, 78 next device found, 129 none;
    /// - phase 1, a0 4: 40/41 (a0 neither 0 nor 3, return a0);
    /// - phase 2: 39/40 `*a2` -1 (0xa), 59/60 `*a2` 0 (SetError 4, phase
    ///   0x1a), 67/68 a0 != 0 (SetError, AbortAtFinish);
    /// - phase 0x16: 42/43 `*a2` >= 0, 43/44 `*a2` < 0, 71/72 a0 != 0;
    /// - phase 0x1a: 40/41 a0 != 0, 51 state 6 (GetError), 54 error 0, 88
    ///   error with another device (back edge), 141 error and no device;
    /// - phase 0xff: 40/41.
    /// Not listed: phase 1 with a0 0 (directory open) and phase 2 with `*a2`
    /// 1 (file request); their totals include the real memset/strlen/copy
    /// callees and were not worked out by hand (the phase-2 one is at least
    /// 111, above every phase-2 total listed). A null a1 or a2 (18 cycles
    /// each) runs as its own leg.
    #[test]
    fn slogin_mc_open_step_matches() {
        let generated = ee::s14::f_003f8c38__generated;
        check(generated, slogin_mc_open_step, 864, &mc_open_step_fixture);
        let quiet = |c: &mut Cpu, seed: u32| {
            mc_open_step_fixture(c, seed);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        };
        require_paths(generated, 144, &[27, 38, 39], &|c, i| quiet(c, 6 * i));
        require_paths(generated, 48, &[27, 45, 78, 129], &|c, i| {
            quiet(c, mc_open_step_slice(1, 1, i))
        });
        require_paths(generated, 48, &[27, 40, 41], &|c, i| {
            quiet(c, mc_open_step_slice(1, 2, i))
        });
        require_paths(generated, 144, &[27, 39, 40, 59, 60, 67, 68], &|c, i| {
            quiet(c, 6 * i + 2)
        });
        require_paths(generated, 144, &[27, 42, 43, 44, 71, 72], &|c, i| {
            quiet(c, 6 * i + 3)
        });
        require_paths(generated, 144, &[27, 40, 41, 51, 54, 88, 141], &|c, i| {
            quiet(c, 6 * i + 4)
        });
        require_paths(generated, 144, &[27, 40, 41], &|c, i| quiet(c, 6 * i + 5));
        // A null done pointer (seeds 0..24) or card reply pointer (24..48)
        // returns 0x12 before any state is read.
        check_paths(generated, slogin_mc_open_step, 48, &[18], &|c, seed| {
            mc_open_step_fixture(c, seed);
            if seed / 24 % 2 == 0 {
                c.g[A1][0] = 0;
            } else {
                c.g[A2][0] = 0;
            }
        });
    }

    // Both sides re-enter at the retry head after an unwind, as the trampoline does.
    crate::difftest::events::driven!(
        driven_generated_mc_open_step,
        ee::s14::f_003f8c38__generated
    );

    crate::difftest::events::driven!(driven_slogin_mc_open_step, slogin_mc_open_step_counted);

    /// The card reports an error after open, so the state machine advances to
    /// another device through the retry head 0x3f8d4c, whose back edge is the
    /// first event check. The hand side itself must unwind at that head and
    /// be re-entered there, as the generated body is.
    #[test]
    fn slogin_mc_open_step_events() {
        use crate::difftest::events::{check_events, schedule};
        mc_hand_resumes().with(|n| n.set(0));
        check_events(
            driven_generated_mc_open_step,
            driven_slogin_mc_open_step,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3f8c_38ee, 0);
                c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x400);
                c.bus.write32(GAME_GP + card::IFACE_GP, DATA + 0x800);
                c.bus.write32(DATA + 0x400, 3);
                c.bus.write32(DATA + 0x400 + card::PHASE, 0x1a);
                c.bus.write32(DATA + 0x400 + card::ERROR, 4);
                c.bus.write32(DATA + 0x400 + card::SCAN_DIRECTION, 0);
                c.bus.write16(DATA + 0x800, 0xff);
                c.bus.write32(DATA + 0x800 + 0x10, FAKE);
                let mode = schedule(c, &mut r, seed, 0x003f_8c38);
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1100);
                mode
            },
        );
        let resumed = mc_hand_resumes().with(|n| n.get());
        assert!(resumed > 0, "the hand side never resumed at the retry head");
    }

    pub(crate) const MC_TEST_CTRL: u32 = DATA + 0x3d0;

    pub(crate) const MC_TEST_MAGIC: u32 = 0x4d43_534c;

    pub(crate) static PREV_MC_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PREV_MC_ALLOC: AtomicUsize = AtomicUsize::new(0);

    pub(crate) const MC_INIT_MAGIC: u32 = 0x4d43_494e;

    pub(crate) fn mc_free_fake(c: &mut Cpu) {
        if c.bus.read32(MC_TEST_CTRL) != MC_TEST_MAGIC {
            return fakes::fall_back(c, &PREV_MC_FREE, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, 0x0026_6760);
    }

    pub(crate) fn mc_init_alloc_fake(c: &mut Cpu) {
        if c.bus.read32(MC_TEST_CTRL) != MC_INIT_MAGIC {
            return fakes::fall_back(c, &PREV_MC_ALLOC, ee::s07::f_00266728__generated);
        }
        fakes::fake_call(c, 0x0026_6728);
        let nth = c.bus.read32(fakes::LOG_COUNT) - 1;
        c.g[V0][0] = if nth == c.bus.read32(MC_TEST_CTRL + 4) {
            0
        } else if nth == 0 {
            (DATA + 0x8000) as u64
        } else {
            (DATA + 0x9000) as u64
        };
    }

    pub(crate) fn mc_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x003f_a000 ^ entry as u64 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
    }

    #[test]
    fn slogin_mc_is_initialized_matches() {
        check(
            ee::s14::f_003f9208__generated,
            slogin_mc_is_initialized,
            60,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_9208);
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if seed % 2 == 0 { 0 } else { DATA },
                );
            },
        );
    }

    /// `seed % 16` picks the class (an existing interface, null arguments, a
    /// failing allocation, a null method slot, device lists),
    /// `[none, 0, 1][seed / 16 % 3]` the fake call that unwinds and, for
    /// class 4, `seed / 48 % 17` the null method slot: three independent
    /// digits.
    #[test]
    fn slogin_mc_init_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            mc_init_alloc_fake,
            &PREV_MC_ALLOC,
        );
        fakes::covered(
            ee::s14::f_003f9218__generated,
            slogin_mc_init,
            16 * 3 * 17,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9218, 2);
                c.pc = 0x003f_9218;
                let class = seed % 16;
                c.bus.write32(MC_TEST_CTRL, MC_INIT_MAGIC);
                c.bus.write32(
                    MC_TEST_CTRL + 4,
                    match class {
                        6 => 0,
                        7 => 1,
                        _ => u32::MAX,
                    },
                );
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    [u32::MAX, 0, 1][(seed / 16 % 3) as usize],
                );
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if class == 0 { DATA + 0x8000 } else { 0 },
                );
                c.bus.write32(
                    GAME_GP + card::STATE_GP,
                    if class == 5 { DATA + 0x9000 } else { 0 },
                );
                c.g[A0][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x6000) as u64
                };
                c.g[A1][0] = if class == 3 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = (DATA + 0xa000) as u64;
                for i in 0..17u32 {
                    c.bus.write32(
                        DATA + 0x7000 + 4 * i,
                        if class == 4 && i == seed / 48 % 17 {
                            0
                        } else {
                            fakes::FAKE + 0x10 * i
                        },
                    );
                }
                // The interface and state the allocations return start
                // non-zero, so the mask reset and the zero stores show.
                for i in 0..0x42cu32 {
                    c.bus.write8(DATA + 0x8000 + i, 0xa5);
                }
                for i in 0..0x74u32 {
                    c.bus.write8(DATA + 0x9000 + i, 0x5a);
                }
                let words: &[u32] = match class {
                    2 => &[u32::MAX],
                    10 => &[9, u32::MAX],
                    11 => &[8, u32::MAX],
                    12 => &[0, 1, 2, 3, 4, 5, 6, 7, 0, u32::MAX],
                    13 => &[0, 9, u32::MAX],
                    14 => &[9, 0, u32::MAX],
                    15 => &[0, 7, u32::MAX],
                    _ => &[0, u32::MAX],
                };
                for (i, value) in words.iter().enumerate() {
                    c.bus.write32(DATA + 0x6000 + 4 * i as u32, *value);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_init_events() {
        use crate::difftest::events::{check_events, schedule};
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            mc_init_alloc_fake,
            &PREV_MC_ALLOC,
        );
        check_events(
            ee::s14::f_003f9218__generated,
            slogin_mc_init,
            90,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9218, 2);
                c.pc = 0x003f_9218;
                c.bus.write32(MC_TEST_CTRL, MC_INIT_MAGIC);
                c.bus.write32(MC_TEST_CTRL + 4, u32::MAX);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                c.bus.write32(GAME_GP + card::IFACE_GP, 0);
                c.bus.write32(GAME_GP + card::STATE_GP, 0);
                c.g[A0][0] = (DATA + 0x6000) as u64;
                c.g[A1][0] = (DATA + 0x7000) as u64;
                c.g[A2][0] = (DATA + 0xa000) as u64;
                for i in 0..17u32 {
                    c.bus.write32(DATA + 0x7000 + 4 * i, fakes::FAKE + 0x10 * i);
                }
                // The interface and state the allocations return start
                // non-zero, so the mask reset and the zero stores show.
                for i in 0..0x42cu32 {
                    c.bus.write8(DATA + 0x8000 + i, 0xa5);
                }
                for i in 0..0x74u32 {
                    c.bus.write8(DATA + 0x9000 + i, 0x5a);
                }
                let list: &[u32] = if seed % 2 == 0 {
                    &[9, u32::MAX]
                } else {
                    &[0, 1, u32::MAX]
                };
                for (i, value) in list.iter().enumerate() {
                    c.bus.write32(DATA + 0x6000 + 4 * i as u32, *value);
                }
                let mut r = Rng(0x003f_9218 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_9218)
            },
        );
    }

    /// `seed % 8`: no status pointer, no data pointer, a card state outside
    /// 11..13 (`[0, 10, 14][seed / 8 % 3]`), or one of the stages 0x18, 0x19,
    /// 0x1a, 0x17 and 0x1b with the state 11..13 (`seed / 8 % 3`);
    /// `seed / 8 % 2`: the method call unwinds or not.
    #[test]
    fn slogin_mc_abort_step_matches() {
        fakes::covered(
            ee::s14::f_003f9130__generated,
            slogin_mc_abort_step,
            128,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_9130, 1);
                c.pc = 0x003f_9130;
                let class = seed % 8;
                c.g[A0][0] = (DATA + 0x6000) as u64;
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x8000) as u64
                };
                c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x6000);
                c.bus.write32(GAME_GP + card::IFACE_GP, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x14, fakes::FAKE);
                c.bus.write32(DATA + 0x5000 + 0x1c, fakes::FAKE + 0x10);
                // The card state: 11..13 for the stage classes; class 2 tries
                // the states around them (0, 10 below, 14 above).
                c.bus.write32(
                    DATA + 0x6000,
                    if class == 2 {
                        [0, 10, 14][(seed / 8 % 3) as usize]
                    } else {
                        11 + seed / 8 % 3
                    },
                );
                // The status word starts non-zero, so a store of 1 or 2 shows.
                c.bus.write32(DATA + 0x7000, 0xa5a5_a5a5);
                c.bus.write32(
                    DATA + 0x6000 + 4,
                    match class {
                        3 => 0x18,
                        4 => 0x19,
                        5 => 0x1a,
                        6 => 0x17,
                        _ => 0x1b,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 8, seed ^ 0x1234);
                c.bus.write32(DATA + 0x6000 + 0xc, DATA + 0x9000);
                c.bus.write32(DATA + 0x6000 + 0x10, DATA + 0x9100);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 8 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    #[test]
    fn slogin_mc_io_step_matches() {
        fakes::covered(
            ee::s14::f_003f8ef8__generated,
            slogin_mc_io_step,
            160,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_8ef8, 1);
                c.pc = 0x003f_8ef8;
                let class = seed % 10;
                c.g[A0][0] = if class == 5 || class == 8 { 0x44 } else { 0 };
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x8000) as u64
                };
                c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x6000);
                c.bus.write32(GAME_GP + card::IFACE_GP, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x38, fakes::FAKE);
                c.bus.write32(
                    DATA + 0x6000,
                    if class == 2 { 7 } else { 8 + seed / 10 % 3 },
                );
                c.bus.write32(
                    DATA + 0x6000 + 4,
                    match class {
                        3 => 0x1a,
                        4 | 5 => 0x1b,
                        6 | 7 => 0x1c,
                        8 => 0x1d,
                        _ => 0x1e,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 0xc, DATA + 0x9100);
                c.bus.write32(
                    DATA + 0x6000 + 0x70,
                    if class == 7 { 0x1235 } else { 0x1234 },
                );
                c.bus.write32(DATA + 0x8000, 0x1234);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 10 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    #[test]
    fn slogin_mc_close_step_matches() {
        fakes::covered(
            ee::s14::f_003f8ff0__generated,
            slogin_mc_close_step,
            160,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_8ff0, 1);
                c.pc = 0x003f_8ff0;
                let class = seed % 10;
                c.g[A0][0] = if class == 4 || class == 6 || class == 8 {
                    0x33
                } else {
                    0
                };
                c.g[A1][0] = if class == 0 {
                    0
                } else {
                    (DATA + 0x7000) as u64
                };
                c.g[A2][0] = if class == 1 {
                    0
                } else {
                    (DATA + 0x8000) as u64
                };
                c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x6000);
                c.bus.write32(GAME_GP + card::IFACE_GP, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 0x14, fakes::FAKE);
                c.bus.write32(DATA + 0x5000 + 0x1c, fakes::FAKE + 0x10);
                c.bus.write32(DATA + 0x6000, if class == 2 { 0 } else { 7 });
                c.bus.write32(
                    DATA + 0x6000 + 4,
                    match class {
                        3 | 4 => 0x18,
                        5 | 6 => 0x19,
                        7 | 8 => 0x1a,
                        9 if seed / 20 % 2 == 1 => 0x1b,
                        _ => 0x17,
                    },
                );
                c.bus.write32(DATA + 0x6000 + 0xc, DATA + 0x9100);
                c.bus.write32(DATA + 0x6000 + 0x10, DATA + 0x9200);
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 10 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    /// `seed % 6`: no interface (not initialised), no file record, card
    /// busy, the record and the state's copy area (state + 0xc) both 8-byte
    /// aligned, the record 1 byte off, and both 4 bytes off (state at +0
    /// instead of +4: the alignment test ORs the two, so only the unaligned
    /// copy is right). `seed / 6 % 2`: the method call unwinds or not.
    pub(crate) fn mc_transfer_fixture(c: &mut Cpu, seed: u32, write: bool) {
        let entry = if write { 0x003f_9f00 } else { 0x003f_9dc0 };
        let _r = fakes::machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let class = seed % 6;
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        let state = if class == 5 {
            DATA + 0x6000
        } else {
            DATA + 0x6004
        };
        c.bus.write32(GAME_GP + card::STATE_GP, state);
        // The state record starts non-zero, so the copy and the stored
        // words show; the source bytes (0x10..0x6f) never match it.
        for i in 0..0x20u32 {
            c.bus.write32(DATA + 0x6000 + 4 * i, 0x5a5a_5a00 | (i + 1));
        }
        c.bus.write32(state, if class == 2 { 1 } else { 0 });
        c.bus
            .write32(DATA + 0x5000 + if write { 0x30 } else { 0x2c }, fakes::FAKE);
        c.g[A0][0] = match class {
            1 => 0,
            4 => (DATA + 0x7001) as u64,
            5 => (DATA + 0x7004) as u64,
            _ => (DATA + 0x7000) as u64,
        };
        c.g[A1][0] = (DATA + 0x8000) as u64;
        c.g[A2][0] = (seed ^ 0x1234) as u64;
        for i in 0..0x68u32 {
            c.bus
                .write8(DATA + 0x7000 + i, (0x10 + (i ^ seed) % 0x60) as u8);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 6 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_read_matches() {
        fakes::covered(
            ee::s14::f_003f9dc0__generated,
            slogin_mc_start_read,
            120,
            &|c, seed| mc_transfer_fixture(c, seed, false),
        );
    }

    #[test]
    fn slogin_mc_start_write_matches() {
        fakes::covered(
            ee::s14::f_003f9f00__generated,
            slogin_mc_start_write,
            120,
            &|c, seed| mc_transfer_fixture(c, seed, true),
        );
    }

    /// Both copy loops of StartRead and StartWrite (aligned head 0x3f9e94 /
    /// 0x3f9fd4: class 3; unaligned head 0x3f9e3c / 0x3f9f7c: classes 4 and
    /// 5, `seed / 3 % 3`); three trips each, the first event lands on a back
    /// edge of the copy.
    #[test]
    fn slogin_mc_start_transfer_events() {
        use crate::difftest::events::{check_events, schedule};
        for (generated, hand, entry, write) in [
            (
                ee::s14::f_003f9dc0__generated as GuestFn,
                slogin_mc_start_read as GuestFn,
                0x003f_9dc0,
                false,
            ),
            (
                ee::s14::f_003f9f00__generated as GuestFn,
                slogin_mc_start_write as GuestFn,
                0x003f_9f00,
                true,
            ),
        ] {
            check_events(generated, hand, 60, &|c, seed| {
                mc_transfer_fixture(c, seed * 6 + 3 + seed / 3 % 3, write);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(entry as u64 ^ seed as u64);
                schedule(c, &mut r, seed, entry)
            });
        }
    }

    pub(crate) fn mc_sjis_fixture(c: &mut Cpu, seed: u32) {
        mc_fixture(c, seed, 0x003f_a380);
        c.g[A0][0] = (DATA + 0x7000) as u64;
        c.g[A1][0] = (DATA + 0x8000) as u64;
        // The output starts non-zero, so a missing or extra halfword shows.
        for i in 0..0x40u32 {
            c.bus.write8(DATA + 0x8000 + i, 0xa5);
        }
        for i in 0..33u32 {
            c.bus.write8(0x0049_73b0 + i, (i ^ seed) as u8);
        }
        const INPUTS: [u8; 10] = [0, 0x20, 0x30, 0x3a, 0x41, 0x5b, 0x61, 0x7b, 0x81, 0x98];
        let ch = INPUTS[(seed % 10) as usize];
        c.bus.write8(DATA + 0x7000, ch);
        c.bus
            .write8(DATA + 0x7001, if ch >= 0x81 { 0x40 } else { 0 });
        c.bus.write8(DATA + 0x7002, 0);
    }

    #[test]
    fn slogin_mc_ascii_to_sjis_matches() {
        check(
            ee::s14::f_003fa380__generated,
            slogin_mc_ascii_to_sjis,
            100,
            &|c, seed| mc_sjis_fixture(c, seed),
        );
    }

    #[test]
    fn slogin_mc_ascii_to_sjis_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003fa380__generated,
            slogin_mc_ascii_to_sjis,
            90,
            &|c, seed| {
                mc_sjis_fixture(c, seed * 10 + 8);
                let mut r = Rng(0x003f_a380 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_a380)
            },
        );
    }

    pub(crate) const MC_POLL_CTRL: u32 = DATA + 0x3e0;

    pub(crate) const MC_POLL_MAGIC: u32 = 0x504f_4c4c;

    /// The status call (fake call 0) stores `MC_POLL_CTRL + 4` in its status
    /// word (`a0`); the stage call (fake call 2) stores `MC_POLL_CTRL + 8` in
    /// its status word (`a1`) when that is non-zero.
    pub(crate) fn mc_poll_fake(c: &mut Cpu, target: u32) {
        if c.bus.read32(MC_POLL_CTRL) == MC_POLL_MAGIC {
            match c.bus.read32(fakes::LOG_COUNT) {
                0 => {
                    let status = c.bus.read32(MC_POLL_CTRL + 4);
                    c.bus.write32(c.g[A0][0] as u32, status);
                }
                2 if c.bus.read32(MC_POLL_CTRL + 8) != 0 => {
                    let status = c.bus.read32(MC_POLL_CTRL + 8);
                    c.bus.write32(c.g[A1][0] as u32, status);
                }
                _ => {}
            }
        }
        fakes::fake_call(c, target);
    }

    pub(crate) fn mc_poll_jump_fake(c: &mut Cpu, target: u32, _start: u32, _end: u32) {
        mc_poll_fake(c, target);
    }

    /// `seed % 13` picks the class: 0 not initialized (23 cycles), 1 no status
    /// method (26), 2 no status word (26), 3 no result word (30), 4 idle, 5 idle
    /// but busy, 6 active, 7..12 completed: 7 result 10, 8 result 16, 9 the
    /// stage call changes the status, 10 stage 5, 11 stage 0, 12 stage
    /// `[2, 7, 11, 12, 13][seed / 52 % 5]`. `seed / 13 % 4` picks the unwind
    /// (none, at call 0, 1 or 2), so every class meets every unwind and the
    /// stage chain (no unwind) meets every stage over 260 seeds. Classes 4..12
    /// call fakes, whose cost varies. Class 7 (the only class that reaches the
    /// result-10 clear) starts the stage word non-zero; its path never reads
    /// it, so the clear's store of 0 shows.
    #[test]
    fn slogin_mc_poll_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _r = fakes::machine(c, seed, 0x003f_a040, 3);
            c.pc = 0x003f_a040;
            c.hooks.miss = mc_poll_fake;
            c.hooks.jump_out = mc_poll_jump_fake;
            let class = seed % 13;
            c.bus.write32(MC_POLL_CTRL, MC_POLL_MAGIC);
            c.bus.write32(
                MC_POLL_CTRL + 4,
                match class {
                    4 | 5 => 0,
                    6 => 1,
                    _ => 2,
                },
            );
            c.bus
                .write32(MC_POLL_CTRL + 8, if class == 9 { 3 } else { 0 });
            c.bus.write32(
                GAME_GP + card::IFACE_GP,
                if class == 0 { 0 } else { DATA + 0x5000 },
            );
            c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x6000);
            c.bus.write32(
                DATA + 0x5000 + 0x40,
                if class == 1 { 0 } else { fakes::FAKE },
            );
            c.bus.write32(DATA + 0x5000 + 0x48, DATA + 0x8000);
            c.bus.write32(DATA + 0x5000 + 0x4c, DATA + 0x8100);
            c.g[A0][0] = if class == 2 {
                0
            } else {
                (DATA + 0x7000) as u64
            };
            c.g[A1][0] = if class == 3 {
                0
            } else {
                (DATA + 0x7100) as u64
            };
            c.bus.write32(DATA + 0x7000, 0xa5a5_a5a5);
            c.bus.write32(DATA + 0x7100, seed ^ 0x44);
            c.bus.write32(
                DATA + 0x6000,
                match class {
                    5 | 10 => 5,
                    12 => [2, 7, 11, 12, 13][(seed / 52 % 5) as usize],
                    7 => 0x7a00_0000 | seed,
                    _ => 0,
                },
            );
            c.bus.write32(DATA + 0x6000 + 4, 1 + seed % 7);
            // Distinct methods per row, so a wrong index calls a wrong target.
            for i in 0..16u32 {
                c.bus
                    .write32(DATA + 0x8000 + 4 * i, fakes::FAKE + 0x200 + 0x10 * i);
                c.bus
                    .write32(DATA + 0x8100 + 4 * i, fakes::FAKE + 0x100 + 0x10 * i);
            }
            c.bus.write32(fakes::LOG_RESULTS, 4);
            c.bus.write32(
                fakes::LOG_RESULTS + 4,
                match class {
                    7 => 10,
                    8 => 16,
                    _ => 5,
                },
            );
            c.bus.write32(fakes::LOG_RESULTS + 8, 6);
            c.bus.write32(
                fakes::LOG_UNWIND_AT,
                [u32::MAX, 0, 1, 2][(seed / 13 % 4) as usize],
            );
        };
        fakes::covered(ee::s14::f_003fa040__generated, slogin_mc_poll, 260, &setup);
        require_paths(ee::s14::f_003fa040__generated, 260, &[23, 26, 30], &setup);
    }

    /// `seed % 6` as in `mc_transfer_fixture`: not initialised, no record,
    /// busy, aligned, record 1 byte off, record and state copy both 4 bytes
    /// off. `seed / 6 % 2`: the close method unwinds or not.
    pub(crate) fn mc_close_fixture(c: &mut Cpu, seed: u32) {
        let _r = fakes::machine(c, seed, 0x003f_9b38, 1);
        c.pc = 0x003f_9b38;
        let class = seed % 6;
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        let state = if class == 5 {
            DATA + 0x6000
        } else {
            DATA + 0x6004
        };
        c.bus.write32(GAME_GP + card::STATE_GP, state);
        // The state record starts non-zero, so the copy and the stored
        // words show; the source bytes (0x10..0x6f) never match it.
        for i in 0..0x20u32 {
            c.bus.write32(DATA + 0x6000 + 4 * i, 0x5a5a_5a00 | (i + 1));
        }
        c.bus.write32(state, if class == 2 { 1 } else { 0 });
        c.bus.write32(DATA + 0x5000 + 0x38, fakes::FAKE);
        c.g[A0][0] = match class {
            1 => 0,
            4 => (DATA + 0x7001) as u64,
            5 => (DATA + 0x7004) as u64,
            _ => (DATA + 0x7000) as u64,
        };
        for i in 0..0x68u32 {
            c.bus
                .write8(DATA + 0x7000 + i, (0x10 + (seed ^ i) % 0x60) as u8);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 6 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_close_matches() {
        fakes::covered(
            ee::s14::f_003f9b38__generated,
            slogin_mc_start_close,
            120,
            &|c, seed| mc_close_fixture(c, seed),
        );
    }

    /// Both copy loops (classes 3, 4 and 5, `seed / 3 % 3`).
    #[test]
    fn slogin_mc_start_close_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f9b38__generated,
            slogin_mc_start_close,
            60,
            &|c, seed| {
                mc_close_fixture(c, seed * 6 + 3 + seed / 3 % 3);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(0x003f_9b38 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_9b38)
            },
        );
    }

    /// `seed % 8`: not initialised, no record, busy, aligned, record 1 byte
    /// off (classes 0-4 seek relative to the data offset, a2 = 0); aligned and
    /// 1 byte off with a2 = 1 (5, 6); record and state copy both 4 bytes off,
    /// a2 = 1 (7). `seed / 8 % 2`: the seek method unwinds or not.
    pub(crate) fn mc_seek_fixture(c: &mut Cpu, seed: u32) {
        let _r = fakes::machine(c, seed, 0x003f_9c48, 1);
        c.pc = 0x003f_9c48;
        let class = seed % 8;
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        let state = if class == 7 {
            DATA + 0x6000
        } else {
            DATA + 0x6004
        };
        c.bus.write32(GAME_GP + card::STATE_GP, state);
        // The state record starts non-zero, so the copy and the stored
        // words show; the source bytes (0x10..0x6f) never match it.
        for i in 0..0x20u32 {
            c.bus.write32(DATA + 0x6000 + 4 * i, 0x5a5a_5a00 | (i + 1));
        }
        c.bus.write32(state, if class == 2 { 1 } else { 0 });
        c.bus.write32(DATA + 0x5000 + 0x34, fakes::FAKE);
        c.g[A0][0] = match class {
            1 => 0,
            4 | 6 => (DATA + 0x7001) as u64,
            7 => (DATA + 0x7004) as u64,
            _ => (DATA + 0x7000) as u64,
        };
        c.g[A1][0] = (seed ^ 0x1020) as u64;
        c.g[A2][0] = if class < 5 { 0 } else { 1 };
        for i in 0..0x68u32 {
            c.bus
                .write8(DATA + 0x7000 + i, (0x10 + (i ^ seed) % 0x60) as u8);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 8 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_seek_matches() {
        fakes::covered(
            ee::s14::f_003f9c48__generated,
            slogin_mc_start_seek,
            128,
            &|c, seed| mc_seek_fixture(c, seed),
        );
    }

    /// Both copy loops (classes 3-7, `seed / 3 % 5`).
    #[test]
    fn slogin_mc_start_seek_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f9c48__generated,
            slogin_mc_start_seek,
            75,
            &|c, seed| {
                mc_seek_fixture(c, seed * 8 + 3 + seed / 3 % 5);
                c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                let mut r = Rng(0x003f_9c48 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_9c48)
            },
        );
    }

    pub(crate) fn mc_named_fixture(c: &mut Cpu, seed: u32, open: bool) {
        let entry = if open { 0x003f_99b8 } else { 0x003f_9698 };
        let _r = fakes::machine(c, seed, entry as u64, 1);
        c.pc = entry;
        let class = seed % 11;
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if class == 0 { 0 } else { DATA + 0x5000 },
        );
        c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x6000);
        c.bus.write16(
            DATA + 0x5000,
            match class {
                3 => 0,
                6 => 1,
                10 => 0x80,
                _ => 3,
            },
        );
        c.bus.write32(DATA + 0x5000 + 0x10, fakes::FAKE);
        // The state record starts non-zero, so the clear, the name copy and
        // the stored mode, phase, error and device show.
        for i in 0..0x20u32 {
            c.bus.write32(DATA + 0x6000 + 4 * i, 0x5a5a_5a00 | (i + 1));
        }
        c.bus.write32(DATA + 0x6000, if class == 4 { 1 } else { 0 });
        c.g[A0][0] = if class == 1 {
            0
        } else {
            (DATA + 0x7000) as u64
        };
        c.g[A1][0] = match class {
            2 => 8,
            6 | 7 | 8 | 9 => u64::MAX,
            10 => 7,
            _ => 0,
        };
        c.g[A2][0] = match class {
            8 => 1,
            9 => 2,
            _ => 0,
        };
        for (i, ch) in b"alice.save\0".iter().enumerate() {
            c.bus.write8(DATA + 0x7000 + i as u32, *ch);
        }
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 11 % 2 == 0 { u32::MAX } else { 0 },
        );
    }

    #[test]
    fn slogin_mc_start_delete_matches() {
        fakes::covered(
            ee::s14::f_003f9698__generated,
            slogin_mc_start_delete,
            132,
            &|c, seed| mc_named_fixture(c, seed, false),
        );
    }

    #[test]
    fn slogin_mc_start_open_matches() {
        fakes::covered(
            ee::s14::f_003f99b8__generated,
            slogin_mc_start_open,
            132,
            &|c, seed| mc_named_fixture(c, seed, true),
        );
    }

    #[test]
    fn slogin_mc_start_create_matches() {
        fakes::covered(
            ee::s14::f_003f9818__generated,
            slogin_mc_start_create,
            144,
            &|c, seed| {
                let class = seed % 12;
                let mapped = if class == 11 { 5 } else { class };
                mc_named_fixture(c, seed / 12 * 11 + mapped, false);
                c.pc = 0x003f_9818;
                if class == 11 {
                    for i in 0..24u32 {
                        c.bus.write8(DATA + 0x7000 + i, b'X');
                    }
                    c.bus.write8(DATA + 0x7000 + 24, 0);
                }
            },
        );
    }

    /// `seed % 3`: not initialized (14 cycles), no card state (18), or clear
    /// both and free them (two fake frees, whose cost varies); `seed / 3 % 3`
    /// unwinds at no call, the first free or the second. The interface and
    /// state start non-zero so the cleared words show.
    #[test]
    fn slogin_mc_shutdown_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            mc_free_fake,
            &PREV_MC_FREE,
        );
        let setup = |c: &mut Cpu, seed: u32| {
            let _r = fakes::machine(c, seed, 0x003f_9530, 2);
            c.pc = 0x003f_9530;
            c.bus.write32(MC_TEST_CTRL, MC_TEST_MAGIC);
            c.bus.write32(
                fakes::LOG_UNWIND_AT,
                [u32::MAX, 0, 1][(seed / 3 % 3) as usize],
            );
            let class = seed % 3;
            let iface = DATA + 0x3000;
            let state = DATA + 0x4000;
            for i in 0..0x10cu32 {
                c.bus.write32(iface + 4 * i, 0x5a5a_5a00 | ((i & 0x7f) + 1));
            }
            for i in 0..4u32 {
                c.bus.write32(state + 4 * i, 0x6b6b_6b00 | (i + 1));
            }
            c.bus.write32(iface + 0x428, DATA + 0x5000);
            c.bus
                .write32(GAME_GP + card::IFACE_GP, if class == 0 { 0 } else { iface });
            c.bus
                .write32(GAME_GP + card::STATE_GP, if class == 2 { state } else { 0 });
        };
        fakes::covered(
            ee::s14::f_003f9530__generated,
            slogin_mc_shutdown,
            90,
            &setup,
        );
        require_paths(ee::s14::f_003f9530__generated, 90, &[14, 18], &setup);
    }

    #[test]
    fn slogin_mc_get_file_state_matches() {
        check_paths(
            ee::s14::f_003f95e8__generated,
            slogin_mc_get_file_state,
            120,
            &[4, 43, 66],
            &mc_get_file_state_fixture,
        );
    }

    #[test]
    fn slogin_mc_is_available_wrapper_matches() {
        check(
            ee::s14::f_003fa360__generated,
            slogin_mc_is_available_wrapper,
            40,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_a360);
            },
        );
    }

    #[test]
    fn slogin_mc_calc_save_size_matches() {
        check(
            ee::s14::f_003fa6a8__generated,
            slogin_mc_calc_save_size,
            100,
            &|c, seed| {
                mc_fixture(c, seed, 0x003f_a6a8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = (DATA + 0x1004) as u64;
                c.bus
                    .write32(DATA + 0x3cc, if seed % 2 == 0 { 1024 } else { 1025 });
                c.bus
                    .write32(DATA + 4, if seed % 4 < 2 { 2048 } else { 2049 });
            },
        );
    }

    /// `seed % 4`: no output (14 cycles), not initialized (21), a device index
    /// of 8 or more (22; `[8, 9, 1 << 32 | 3, -1][seed / 4 % 4]`, the compare is
    /// 64-bit), or device `seed / 4 % 8` split through the game's table (37 for
    /// device 0, 38 for 1/4/5, 39 for 2/3, 40 for 7, 41 for 6). The output
    /// starts non-zero.
    #[test]
    fn slogin_mc_get_port_slot_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            mc_fixture(c, seed, 0x003f_a1d0);
            elf_image(c);
            let class = seed % 4;
            let out = DATA + 0x1000;
            c.bus.write32(out, 0xcccc_cccc);
            c.g[A1][0] = if class == 0 { 0 } else { sx(out) };
            c.g[A0][0] = if class == 2 {
                [8, 9, 1 << 32 | 3, u64::MAX][(seed / 4 % 4) as usize]
            } else {
                (seed / 4 % 8) as u64
            };
            c.bus.write32(
                GAME_GP + card::IFACE_GP,
                if class <= 1 { 0 } else { DATA + 0x2000 },
            );
        };
        check_paths(
            ee::s14::f_003fa1d0__generated,
            slogin_mc_get_port_slot,
            128,
            &[14, 21, 22, 37, 38, 39, 40, 41],
            &setup,
        );
    }

    pub(crate) const CREATE_TEST_STATE: u32 = DATA + 0x2000;

    pub(crate) const CREATE_TEST_IFACE: u32 = DATA + 0x3000;

    pub(crate) const CREATE_TEST_STATUS: u32 = DATA + 0x1000;

    pub(crate) const CREATE_TEST_RESULT: u32 = DATA + 0x1004;

    pub(crate) fn create_step_fixture(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x003f_a8f8, 2);
        // The game's image holds the phase dispatch table and the icon file
        // names; it also covers the gp words, so it goes in first.
        elf_image(c);
        assert_eq!(c.bus.read32(CREATE_CALLBACK_TABLE), 0x003f_a978);
        c.pc = 0x003f_a8f8;
        c.g[A0][0] = 0;
        c.g[A1][0] = sx(CREATE_TEST_STATUS);
        c.g[A2][0] = sx(CREATE_TEST_RESULT);
        c.bus.write32(
            (c.g[GP][0] as u32).wrapping_add(card::STATE_GP),
            CREATE_TEST_STATE,
        );
        c.bus.write32(
            (c.g[GP][0] as u32).wrapping_add(card::IFACE_GP),
            CREATE_TEST_IFACE,
        );
        // The status word and every state word the step may store (phase,
        // device, handle, both paths) start non-zero.
        c.bus.write32(CREATE_TEST_STATUS, 0xa5a5_a5a5);
        for i in 0..0x20u32 {
            c.bus
                .write32(CREATE_TEST_STATE + 4 * i, 0x5a5a_5a00 | (i + 1));
        }
        c.bus.write32(CREATE_TEST_STATE, 1);
        c.bus.write32(CREATE_TEST_STATE + 8, 0);
        c.bus.write32(CREATE_TEST_STATE + card::DEVICE, 0);
        c.bus.write32(CREATE_TEST_STATE + card::FILE_HANDLE, 0x42);
        c.bus.write32(CREATE_TEST_STATE + card::SCAN_DIRECTION, 0);
        c.bus.write16(CREATE_TEST_IFACE, 3);
        c.bus.write32(CREATE_TEST_IFACE + 0x420, 64);
        c.bus.write32(CREATE_TEST_IFACE + 0x424, 128);
        for (i, offset) in [4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x30, 0x38]
            .into_iter()
            .enumerate()
        {
            c.bus
                .write32(CREATE_TEST_IFACE + offset, FAKE + 0x10 * i as u32);
        }
        for (i, byte) in b"beta\0".iter().enumerate() {
            c.bus.write8(CREATE_TEST_IFACE + 0x50 + i as u32, *byte);
        }
        for (i, byte) in b"demo\0".iter().enumerate() {
            c.bus
                .write8(CREATE_TEST_STATE + card::DIR_NAME + i as u32, *byte);
        }

        // Each residue is one state and one side of its result predicates.
        let scenario = seed % CREATE_STEP_SCENARIOS;
        let (phase, result, data) = match scenario {
            0 => {
                c.g[A1][0] = 0;
                (1, 0, 0)
            }
            1 => {
                c.g[A2][0] = 0;
                (1, 0, 0)
            }
            2 => {
                c.bus.write32(CREATE_TEST_STATE, 2);
                (1, 0, 0)
            }
            3 => (0, 0, 0),
            4 => (1, 0, 0),
            5 => (1, 2, 0),
            6 => {
                c.bus.write32(CREATE_TEST_STATE, 4);
                (1, 3, 0)
            }
            7 => {
                c.bus.write16(CREATE_TEST_IFACE, 0);
                (1, 3, 0)
            }
            8 => (1, 3, 0),
            9 => (2, 0, 0),
            10 => (2, 0, 1),
            11 => (2, 0, 2),
            12 => (2, 2, 0),
            13 => (3, 0, u32::MAX),
            14 => (3, 0, 32),
            15 => (3, 0, 96),
            16 => (3, 2, 0),
            17 => (4, 0, u32::MAX),
            18 => (4, 0, 64),
            19 => (4, 0, 256),
            20 => (4, 2, 0),
            21 => (5, 0, 0),
            22 => (5, 2, 0),
            23 => (14, 0, 12),
            24 => (14, 0, u32::MAX),
            25 => (14, 2, 0),
            26 => (16, 0, 0),
            27 => (16, 2, 0),
            28 => (17, 0, 0),
            29 => (17, 2, 0),
            30 => (6, 0, 12),
            31 => (6, 2, 0),
            32 => (7, 0, 0),
            33 => (7, 2, 0),
            34 => (8, 0, 0),
            35 => (8, 2, 0),
            36 => (9, 0, 0),
            37 => (9, 2, 0),
            38 => (10, 0, 12),
            39 => (10, 2, 0),
            40 => (11, 0, 0),
            41 => (11, 2, 0),
            42 => (12, 0, 0),
            43 => (12, 2, 0),
            44 => (13, 0, 0),
            45 => (13, 2, 0),
            46 => (26, 2, 0),
            47 => {
                c.bus.write32(CREATE_TEST_STATE, 4);
                (26, 0, 0)
            }
            48 => (26, 0, 0),
            49 => {
                c.bus.write32(CREATE_TEST_STATE + 8, 7);
                c.bus.write16(CREATE_TEST_IFACE, 0);
                (26, 0, 0)
            }
            50 => {
                c.bus.write32(CREATE_TEST_STATE + 8, 7);
                (26, 0, 0)
            }
            51 => (15, 0, 0),
            52 => {
                // A phase-1 row outside the function leaves through jump_out.
                c.bus.write32(CREATE_CALLBACK_TABLE, FAKE);
                (1, 0, 0)
            }
            53 => (27, 0, 0),
            54..=61 => (18 + scenario - 54, 0, 0),
            // The reply on each side of the space checks: no space at all
            // and exactly the space needed, for the directory (phase 3, needs
            // +0x420 = 64) and the file (phase 4, needs +0x424 = 128); and the
            // phase-14 open reply 0 (a valid handle).
            62 => (3, 0, 0),
            63 => (3, 0, 64),
            64 => (4, 0, 0),
            65 => (4, 0, 128),
            66 => (14, 0, 0),
            _ => unreachable!(),
        };
        c.g[A0][0] = result;
        c.bus.write32(CREATE_TEST_STATE + card::PHASE, phase);
        c.bus.write32(CREATE_TEST_RESULT, data);
    }

    /// Every scenario twice. The others call FAKE callbacks (directly or
    /// through the abort helpers), whose cost varies with their arguments, so
    /// only the fixed scenarios are held to their totals.
    #[test]
    fn slogin_mc_create_step_matches() {
        check(
            ee::s14::f_003fa8f8__generated,
            slogin_mc_create_step,
            2 * CREATE_STEP_SCENARIOS,
            &create_step_fixture,
        );
        require_paths(
            ee::s14::f_003fa8f8__generated,
            CREATE_STEP_FIXED_SCENARIOS.len() as u32,
            &[14, 23, 30, 36, 38, 40, 42, 43, 46, 47, 51, 133, 138],
            &|c, s| create_step_fixture(c, CREATE_STEP_FIXED_SCENARIOS[s as usize]),
        );
    }

    /// Scrambled registers and memory around the stack, entry at `entry`.
    pub(crate) fn slogin_mc_leaf_machine(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(entry as u64 ^ (seed as u64) << 24);
        scramble(c, &mut r);
        c.pc = entry;
        r
    }

    #[test]
    fn slogin_mc_get_checksum_iface_matches() {
        check(
            ee::s14::f_003fa6c8__generated,
            slogin_mc_get_checksum_iface,
            32,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a6c8);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x10) };
                c.bus.write32(DATA + 0x10, 0x5a5a_5a5a);
            },
        );
    }

    #[test]
    fn slogin_mc_get_data_offset_matches() {
        check(
            ee::s14::f_003fa6e8__generated,
            slogin_mc_get_data_offset,
            16,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a6e8);
            },
        );
    }

    #[test]
    fn slogin_mc_split_device_index_matches() {
        check(
            ee::s14::f_003fa708__generated,
            slogin_mc_split_device_index,
            80,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a708);
                elf_image(c);
                // The game's table must hold the eight arms this port knows.
                for (i, target) in SPLIT_DEVICE_INDEX_TARGETS.iter().enumerate() {
                    assert_eq!(
                        c.bus.read32(SPLIT_DEVICE_INDEX_TABLE + 4 * i as u32),
                        *target
                    );
                }
                let out = DATA + 0x200 + seed / 10 % 2;
                c.g[A0][0] = match seed % 10 {
                    d @ 0..=7 => d as u64,
                    8 => 8 + (seed / 20 % 4) as u64,
                    _ => 1u64 << 32 | (seed / 20 % 8) as u64,
                };
                c.g[A1][0] = sx(out);
                c.bus.write32(DATA + 0x200, 0xcccc_cccc);
            },
        );
    }

    /// A NUL-terminated name of `len` letters at `at`, with bytes after it.
    pub(crate) fn slogin_mc_path_name(c: &mut Cpu, at: u32, len: u32, salt: u32) {
        for i in 0..len {
            c.bus.write8(at + i, b'A' + ((i + salt) % 26) as u8);
        }
        c.bus.write8(at + len, 0);
        c.bus.write32(at + len + 1, 0x7777_7777);
    }

    /// The name length `seed % 8 * 3`, the output's misalignment `seed / 8 % 4`
    /// and the lead byte with or without its top bit (`lbu`) `seed / 32 % 2`,
    /// fully crossed (64 seeds).
    #[test]
    fn slogin_mc_build_dir_path_matches() {
        check(
            ee::s14::f_003fa7a0__generated,
            slogin_mc_build_dir_path,
            64,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a7a0);
                let out = DATA + 0x400 + seed / 8 % 4;
                for i in 0..0x30 {
                    c.bus.write8(DATA + 0x400 + i, 0xa5);
                }
                c.g[A0][0] = sx(out);
                c.g[A2][0] = sx(DATA + 0x500);
                slogin_mc_path_name(c, DATA + 0x500, seed % 8 * 3, seed);
                // Lead byte with and without its top bit (`lbu`).
                c.bus.write8(
                    BUILD_DIR_PATH_LEAD,
                    if seed / 32 % 2 == 0 { b'*' } else { 0xb0 },
                );
            },
        );
    }

    /// Every digit independent (512 seeds): the name length `seed % 8 * 3`,
    /// path A's misalignment `seed / 8 % 8`, path B's `seed / 64 % 4`, and
    /// the 9th prefix byte with its top bit set or not (`lb` into v1),
    /// `seed / 256 % 2`.
    #[test]
    fn slogin_mc_build_file_paths_matches() {
        check(
            ee::s14::f_003fa818__generated,
            slogin_mc_build_file_paths,
            512,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_a818);
                elf_image(c);
                for i in 0..0x30 {
                    c.bus.write8(DATA + 0x600 + i, 0xa5);
                    c.bus.write8(DATA + 0x700 + i, 0x5a);
                }
                c.g[A0][0] = sx(DATA + 0x600 + seed / 8 % 8);
                c.g[A1][0] = sx(DATA + 0x700 + seed / 64 % 4);
                c.g[A3][0] = sx(DATA + 0x800);
                slogin_mc_path_name(c, DATA + 0x800, seed % 8 * 3, seed);
                if seed / 256 % 2 == 1 {
                    c.bus.write8(BUILD_FILE_PATHS_PREFIX + 8, 0xb8);
                }
            },
        );
    }

    #[test]
    fn slogin_mc_is_available_matches() {
        check(
            ee::s15::f_003fb278__generated,
            slogin_mc_is_available,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b278);
            },
        );
    }

    #[test]
    fn slogin_mc_calc_save_size_impl_matches() {
        check(
            ee::s15::f_003fb280__generated,
            slogin_mc_calc_save_size_impl,
            48,
            &|c, seed| {
                let mut r = slogin_mc_leaf_machine(c, seed, 0x003f_b280);
                let login = DATA + 0x100;
                c.g[A0][0] = sx(login);
                c.g[A1][0] = sx(DATA + 0x900);
                c.g[A2][0] = sx(DATA + 0x904);
                c.bus.write32(DATA + 0x900, 0x1111_1111);
                c.bus.write32(DATA + 0x904, 0x2222_2222);
                let kib = |r: &mut Rng, partial: bool| {
                    let whole = r.next_u32() & 0x7fff_fc00;
                    if partial {
                        whole | 1 + seed / 4 % 0x3ff
                    } else {
                        whole
                    }
                };
                let first = kib(&mut r, seed % 2 == 1);
                let second = kib(&mut r, seed / 2 % 2 == 1);
                c.bus.write32(
                    login + CALC_SAVE_SIZE_FIRST,
                    if seed % 12 == 0 { 0 } else { first },
                );
                c.bus.write32(login + CALC_SAVE_SIZE_SECOND, second);
            },
        );
    }

    #[test]
    fn slogin_mc_get_checksum_iface_impl_matches() {
        check(
            ee::s15::f_003fb2c8__generated,
            slogin_mc_get_checksum_iface_impl,
            32,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b2c8);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x10) };
                c.bus.write32(DATA + 0x10, 0x5a5a_5a5a);
            },
        );
    }

    #[test]
    fn slogin_mc_set_busy_matches() {
        check(
            ee::s15::f_003fb2f8__generated,
            slogin_mc_set_busy,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b2f8);
            },
        );
    }

    #[test]
    fn slogin_mc_get_data_offset_impl_matches() {
        check(
            ee::s15::f_003fb300__generated,
            slogin_mc_get_data_offset_impl,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b300);
            },
        );
    }

    #[test]
    fn slogin_mc_validate_icon_sys_matches() {
        check(
            ee::s15::f_003fb308__generated,
            slogin_mc_validate_icon_sys,
            32,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b308);
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => sx(DATA),
                    2 => 1u64 << 40, // low word zero: the 64-bit test still sees it
                    _ => 1,
                };
            },
        );
    }

    /// `n` for shape `shape` (0 the bare byte, 1 sign-extended with bit 31
    /// set, 2 upper bits set: all 32 bits shift through the loop) of byte
    /// `digit * 0x9d % 256` (a permutation of the digit, so the bytes spread
    /// over 0..=0xff).
    pub(crate) fn crc32_table_entry_fixture(c: &mut Cpu, seed: u32, shape: u32, digit: u32) {
        let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b318);
        let byte = digit.wrapping_mul(0x9d) % 256;
        c.g[A0][0] = match shape {
            0 => byte as u64,
            1 => 0xffff_ffff_0000_0000 | byte as u64 | 0x8000_0000,
            _ => (seed as u64) << 40 | byte as u64 | (seed as u64 % 16) << 20,
        };
    }

    /// Shape seed % 3, byte digit seed / 3.
    #[test]
    fn crc32_table_entry_matches() {
        check(
            ee::s15::f_003fb318__generated,
            crc32_table_entry,
            96,
            &|c, seed| {
                crc32_table_entry_fixture(c, seed, seed % 3, seed / 3);
            },
        );
    }

    /// Event mode seed % 3, shape seed / 3 % 3, byte digit seed / 9.
    #[test]
    fn crc32_table_entry_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fb318__generated,
            crc32_table_entry,
            90,
            &|c, seed| {
                crc32_table_entry_fixture(c, seed, seed / 3 % 3, seed / 9);
                let mut r = Rng(0x003f_b318 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_b318)
            },
        );
    }

    pub(crate) fn crc32_build_table_fixture(c: &mut Cpu, seed: u32) {
        let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b358);
        let table = DATA + 0x1000 + 4 * (seed % 4);
        for i in 0..0x110 {
            c.bus.write32(DATA + 0x1000 + 4 * i, 0xc3c3_c3c3);
        }
        c.g[A0][0] = sx(table);
    }

    #[test]
    fn crc32_build_table_matches() {
        check(
            ee::s15::f_003fb358__generated,
            crc32_build_table,
            8,
            &|c, seed| {
                crc32_build_table_fixture(c, seed);
            },
        );
    }

    #[test]
    fn crc32_build_table_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fb358__generated,
            crc32_build_table,
            45,
            &|c, seed| {
                crc32_build_table_fixture(c, seed);
                let mut r = Rng(0x003f_b358 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_b358)
            },
        );
    }

    /// The service state words, non-zero, with the table pointer `table`.
    pub(crate) fn crc32_state(c: &mut Cpu, table: u32) {
        for i in 0..6 {
            c.bus.write32(CRC32_STATE + 4 * i, 0x9e00_0000 | i);
        }
        c.bus.write32(CRC32_STATE + CRC32_STATE_TABLE, table);
    }

    /// seed % 3: 0 running (5), 1 the allocation fails (1), 2 builds the
    /// table; the allocator unwinds when seed / 3 % 4 = 0. The store's heap
    /// fakes: allocation 0 returns the table block or 0.
    #[test]
    fn crc32_init_matches() {
        slogin_store_hook_heap_fakes();
        fakes::covered(
            ee::s15::f_003fb3a0__generated,
            crc32_init,
            72,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_b3a0, 1);
                c.pc = 0x003f_b3a0;
                let mode = seed % 3;
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 3 % 4 == 0 { 0 } else { u32::MAX },
                );
                c.bus.write32(STORE_FAKE_CTRL, STORE_FAKE_MAGIC);
                c.bus.write32(
                    STORE_FAKE_ALLOC_RESULTS,
                    if mode == 2 { DATA + 0x2000 } else { 0 },
                );
                for i in 0..0x104 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, 0xc3c3_c3c3);
                }
                crc32_state(c, if mode == 0 { DATA + 0x2000 } else { 0 });
                c.g[A0][0] = sx(DATA + 0x3000 + seed % 4 * 0x10);
            },
        );
    }

    /// seed % 2: not running (6) or frees the table through the store's
    /// heap fake, which unwinds when seed / 2 % 3 = 0.
    #[test]
    fn crc32_shutdown_matches() {
        slogin_store_hook_heap_fakes();
        fakes::covered(
            ee::s15::f_003fb430__generated,
            crc32_shutdown,
            48,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_b430, 1);
                c.pc = 0x003f_b430;
                let running = seed % 2 == 1;
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 2 % 3 == 0 { 0 } else { u32::MAX },
                );
                c.bus.write32(STORE_FAKE_CTRL, STORE_FAKE_MAGIC);
                crc32_state(c, if running { DATA + 0x2000 } else { 0 });
                c.bus.write32(
                    CRC32_STATE + CRC32_STATE_HEAP,
                    DATA + 0x3000 + seed % 4 * 0x10,
                );
            },
        );
    }

    #[test]
    fn crc32_get_interface_matches() {
        check(
            ee::s15::f_003fb5e8__generated,
            crc32_get_interface,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b5e8);
            },
        );
    }

    #[test]
    fn slogin_store_get_cipher_iface_matches() {
        check(
            ee::s15::f_003fb800__generated,
            slogin_store_get_cipher_iface,
            8,
            &|c, seed| {
                let _r = slogin_mc_leaf_machine(c, seed, 0x003f_b800);
            },
        );
    }

    pub(crate) fn slogin_store_map_error_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = slogin_mc_leaf_machine(c, seed, entry);
        c.g[A0][0] = match seed % 5 {
            0 => 0,
            1 => 1,
            2 => u32::MAX as u64,
            3 => 1u64 << 36, // low word zero: the 64-bit test still sees it
            _ => sx(r.next_u32() | 0x8000_0000),
        };
    }

    #[test]
    fn slogin_store_map_mc_error_matches() {
        check(
            ee::s15::f_003fb810__generated,
            slogin_store_map_mc_error,
            40,
            &|c, seed| {
                slogin_store_map_error_fixture(c, seed, 0x003f_b810);
            },
        );
    }

    #[test]
    fn slogin_store_map_cipher_error_matches() {
        check(
            ee::s15::f_003fb828__generated,
            slogin_store_map_cipher_error,
            40,
            &|c, seed| {
                slogin_store_map_error_fixture(c, seed, 0x003f_b828);
            },
        );
    }

    #[test]
    fn slogin_store_map_hash_error_matches() {
        check(
            ee::s15::f_003fb840__generated,
            slogin_store_map_hash_error,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fb8_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b840;
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => u32::MAX as u64,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    /// Arms: format seed % 9 = 0..4 table dispatch (16 cycles; the game's
    /// table at 0x497580 from `elf_image`), 5/6 past the table and 7/8
    /// (1 << 32, -1: the compare is 64-bit) the defaults (7). Output words
    /// pre-filled.
    #[test]
    fn slogin_store_get_alignment_matches() {
        check_paths(
            ee::s15::f_003fb858__generated,
            slogin_store_get_alignment,
            63,
            &[7, 16],
            &|c, seed| {
                let mut r = Rng(0x3fb8_5800 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                c.pc = 0x003f_b858;
                c.g[A0][0] = [0, 1, 2, 3, 4, 5, 6, 1 << 32, u64::MAX][(seed % 9) as usize];
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x104);
                c.bus.write32(DATA + 0x100, 0x5a5a_5a5a);
                c.bus.write32(DATA + 0x104, 0xa5a5_a5a5);
            },
        );
    }

    /// Payload seed % 7 (0xfffffff4 at 4: payload + 12 wraps to 0, the
    /// `sltu` arm; 0xfffffff5 at 5: payload + 12 is 1, equal to the unit, so
    /// the divide arm), format seed / 7 % 7 through the real
    /// `SLoginStore_GetAlignment` and the game's table (`elf_image`), and
    /// random upper payload bits when seed / 49 % 2 = 1. Paths: default
    /// format + wrap (28 cycles), table format + divide (46); 37 is either
    /// table format + wrap or default format + divide. LO/HI compared (the
    /// body runs `divu`).
    #[test]
    fn slogin_store_calc_alloc_size_matches() {
        crate::difftest::check_paths_lohi(
            ee::s15::f_003fb898__generated,
            slogin_store_calc_alloc_size,
            98,
            &[28, 37, 46],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3fb8_9800, 0);
                elf_image(c);
                c.pc = 0x003f_b898;
                let payload = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 4,
                    3 => 0xfff,
                    4 => 0u32.wrapping_sub(BLOCK_HEADER_BYTES),
                    5 => 1u32.wrapping_sub(BLOCK_HEADER_BYTES),
                    _ => r.next_u32(),
                } as u64;
                let high = if seed / 49 % 2 == 1 {
                    (r.next_u32() as u64) << 32
                } else {
                    0
                };
                c.g[A0][0] = payload | high;
                c.g[A1][0] = (seed / 7 % 7) as u64;
            },
        );
    }

    #[test]
    fn slogin_store_get_block_header_matches() {
        check(
            ee::s15::f_003fb910__generated,
            slogin_store_get_block_header,
            45,
            &|c, seed| {
                let _r = crate::difftest::fakes::machine(c, seed, 0x3fb9_1000, 0);
                let mode = seed % 3;
                c.pc = 0x003f_b910;
                c.g[A0][0] = sx(DATA + 0x20);
                c.g[A1][0] = if mode == 0 { 0 } else { 1 };
                for i in 0..BLOCK_HEADER_BYTES {
                    let b = (i * 17 + 7) as u8;
                    c.bus.write8(BLOCK_HEADER_MAGIC + i, b);
                    c.bus.write8(
                        DATA + 0x14 + i,
                        if mode == 2 && i == 4 { b ^ 0xff } else { b },
                    );
                }
            },
        );
    }

    /// Header misaligned by seed % 4 and the CRC source by seed / 4 % 4
    /// (independent: every pair of unaligned copies runs); the header bytes
    /// are pre-filled.
    #[test]
    fn slogin_store_set_record_header_matches() {
        check(
            ee::s15::f_003fb958__generated,
            slogin_store_set_record_header,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fb9_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b958;
                let header = DATA + 0x300 + seed % 4;
                let crc = DATA + 0x500 + seed / 4 % 4;
                for i in 0..0x14 {
                    c.bus.write8(DATA + 0x300 + i, 0xa5);
                }
                c.g[A0][0] = sx(header);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(crc);
                c.bus.write32(crc, r.next_u32());
            },
        );
    }

    /// Block misaligned by seed % 4 and pre-filled; the 12 pattern bytes at
    /// 0x497550 are random (filled completely).
    #[test]
    fn slogin_store_write_sentinel_matches() {
        check(
            ee::s15::f_003fb9a0__generated,
            slogin_store_write_sentinel,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fb9_a000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b9a0;
                for i in 0..0x14 {
                    c.bus.write8(DATA + 0x700 + i, 0xa5);
                }
                c.g[A0][0] = sx(DATA + 0x700 + seed % 4);
                for i in 0..12 {
                    c.bus.write8(BLOCK_HEADER_MAGIC + i, r.next_u32() as u8);
                }
            },
        );
    }

    /// Outputs present by bits of seed % 8 (kind, size, CRC), the header
    /// misaligned by seed / 8 % 4 and the outputs by seed / 32 % 4 (every
    /// presence/alignment combination); outputs pre-filled. Paths: 8 cycles
    /// plus 4 per output present.
    #[test]
    fn slogin_store_get_record_header_matches() {
        check_paths(
            ee::s15::f_003fb9d0__generated,
            slogin_store_get_record_header,
            128,
            &[8, 12, 16, 20],
            &|c, seed| {
                let mut r = Rng(0x3fb9_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_b9d0;
                let present = seed % 8;
                let header = DATA + 0x900 + seed / 8 % 4;
                let out = seed / 32 % 4;
                for i in 0..0x30 {
                    c.bus.write8(DATA + 0xa00 + i, 0xa5);
                }
                c.g[A0][0] = sx(header);
                for (bit, (reg, base)) in [(A1, 0xa00), (A2, 0xa10), (A3, 0xa20)]
                    .into_iter()
                    .enumerate()
                {
                    c.g[reg][0] = if present >> bit & 1 == 0 {
                        0
                    } else {
                        sx(DATA + base + out)
                    };
                }
                for i in 0..12 {
                    c.bus.write8(header + i, r.next_u32() as u8);
                }
            },
        );
    }

    /// Directory kinds 5, 7, 5. Arms by seed % 5: 0 empty directory (10
    /// cycles), 1 kind 5 occurrence 0: first record (16), 2 kind 5
    /// occurrence 1: skips the other occurrence and a kind miss, found third
    /// (29), 3 absent kind (9, or 0x1_0000_0005 by seed / 5 % 2: the compare
    /// is 64-bit): scans to the end (27), 4 kind 7: second record (21).
    #[test]
    fn slogin_store_find_record_matches() {
        check_paths(
            ee::s15::f_003fba20__generated,
            slogin_store_find_record,
            75,
            &[10, 16, 21, 27, 29],
            &|c, seed| {
                let mut r = Rng(0x3fba_2000 ^ seed as u64);
                scramble(c, &mut r);
                let mode = seed % 5;
                c.pc = 0x003f_ba20;
                let kinds: &[u32] = if mode == 0 { &[] } else { &[5, 7, 5] };
                slogin_store_find_record_directory(c, &mut r, kinds);
                c.g[A0][0] = match mode {
                    3 => [9u64, 0x1_0000_0005][(seed / 5 % 2) as usize],
                    4 => 7,
                    _ => 5,
                };
                c.g[A1][0] = if mode == 2 { 1 } else { 0 };
            },
        );
    }

    pub(crate) const STORE_TEST_STATE: u32 = DATA + 0x2000;

    pub(crate) const STORE_TEST_HASH_IFACE: u32 = DATA + 0x2200;

    pub(crate) const STORE_TEST_CIPHER_IFACE: u32 = DATA + 0x2300;

    pub(crate) const STORE_TEST_MC_MASK: u32 = DATA + 0x2400;

    pub(crate) const STORE_TEST_MC_STATE: u32 = DATA + 0x2500;

    pub(crate) const STORE_TEST_RECORDS: u32 = DATA + 0x3000;

    /// Memory-card layer state for the real `SLoginMc_Start*` short paths:
    /// 0 uninitialised (error 2); 1 initialised and busy (error 13);
    /// 2 initialised, idle, device mask empty (open: error 11).
    pub(crate) fn slogin_store_mc_fixture(c: &mut Cpu, mode: u32) {
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if mode == 0 { 0 } else { STORE_TEST_MC_MASK },
        );
        c.bus.write32(GAME_GP + card::STATE_GP, STORE_TEST_MC_STATE);
        c.bus
            .write16(STORE_TEST_MC_MASK, if mode == 2 { 0 } else { 0xffff });
        c.bus
            .write32(STORE_TEST_MC_STATE, if mode == 1 { 1 } else { 0 });
    }

    /// A directory of `count` entries (kinds 5, 7, 5, 9, 5, 5 repeating) with
    /// random offset, size and CRC words.
    pub(crate) fn slogin_store_set_record_crc_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        count: u32,
        kind: u64,
        occurrence: u64,
    ) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + store::STATE_GP, STORE_TEST_STATE);
        c.bus.write32(STORE_TEST_STATE + store::RECORD_COUNT, count);
        c.bus
            .write32(STORE_TEST_STATE + store::RECORDS, STORE_TEST_RECORDS);
        fill(c, r, STORE_TEST_RECORDS, 4 * 16);
        for i in 0..count {
            c.bus.write32(
                STORE_TEST_RECORDS + i * store::RECORD_STRIDE,
                [5, 7, 5, 9, 5, 5][(i % 6) as usize],
            );
        }
        c.bus.write32(DATA + 0x3800, r.next_u32() | 1);
        c.g[A0][0] = kind;
        c.g[A1][0] = occurrence;
        c.g[A2][0] = sx(DATA + 0x3800);
    }

    #[test]
    fn slogin_store_set_record_crc_matches() {
        check(
            ee::s15::f_003fba78__generated,
            slogin_store_set_record_crc,
            96,
            &|c, seed| {
                let mut r = Rng(0x3fba_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_ba78;
                // Kind 0x1_0000_0005 never matches (the compare is 64-bit).
                let kind = [5u64, 7, 0x1_0000_0005][(seed / 4 % 3) as usize];
                slogin_store_set_record_crc_fixture(
                    c,
                    &mut r,
                    [0, 1, 3, 6][(seed % 4) as usize],
                    kind,
                    (seed / 12 % 4) as u64,
                );
            },
        );
    }

    #[test]
    fn slogin_store_set_record_crc_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fba78__generated,
            slogin_store_set_record_crc,
            72,
            &|c, seed| {
                let mut r = Rng(0x3fba_78ee ^ seed as u64);
                scramble(c, &mut r);
                let mode = schedule(c, &mut r, seed, 0x003f_ba78);
                slogin_store_set_record_crc_fixture(c, &mut r, 16, 5, (seed / 3 % 4) as u64);
                mode
            },
        );
    }

    #[test]
    fn slogin_store_add_record_matches() {
        check(
            ee::s15::f_003fbad0__generated,
            slogin_store_add_record,
            54,
            &|c, seed| {
                let mut r = Rng(0x3fba_d000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_bad0;
                c.g[GP][0] = GAME_GP as u64;
                let count = [0, 1, 3][(seed % 3) as usize];
                // Full now, full after this entry, or room to spare.
                let capacity = count + [0, 1, 4][(seed / 3 % 3) as usize];
                // 0xffff_fff4 makes size + 12 zero (the `movz` arm).
                let size = [r.next_u32() & 0xfff, 0xffff_fff4, 0][(seed / 9 % 3) as usize];
                c.bus.write32(GAME_GP + store::STATE_GP, STORE_TEST_STATE);
                fill(c, &mut r, STORE_TEST_STATE, 0x28);
                c.bus.write32(STORE_TEST_STATE + store::RECORD_COUNT, count);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_CAPACITY, capacity);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORDS, STORE_TEST_RECORDS);
                fill(c, &mut r, STORE_TEST_RECORDS, 4 * 8);
                c.bus.write32(DATA + 0x3800, r.next_u32());
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(size);
                c.g[A2][0] = sx(DATA + 0x3800);
            },
        );
    }

    #[test]
    fn slogin_store_hash_matches() {
        fakes::covered(
            ee::s15::f_003fbb50__generated,
            slogin_store_hash,
            64,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbb_5000);
                c.pc = 0x003f_bb50;
                // Begin fails, update fails, final fails, all succeed.
                c.bus
                    .write32(STORE_IO_IFACE_FAIL_MASK, [1, 2, 4, 0][(seed % 4) as usize]);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 1, 2, u32::MAX][(seed / 4 % 4) as usize]);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = (r.next_u32() & 0xfff) as u64;
                c.g[A2][0] = sx(DATA + 0x4100);
            },
        );
    }

    #[test]
    fn slogin_store_hash_directory_matches() {
        fakes::covered(
            ee::s15::f_003fbbe0__generated,
            slogin_store_hash_directory,
            72,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbb_e000);
                c.pc = 0x003f_bbe0;
                // 0x1000_0000 entries is 0 bytes after the shift: the empty arm.
                let count = [0, 3, 0x1000_0000][(seed % 3) as usize];
                c.bus.write32(STORE_TEST_STATE + store::RECORD_COUNT, count);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORDS, STORE_TEST_RECORDS);
                c.bus.write32(
                    STORE_IO_IFACE_FAIL_MASK,
                    [1, 2, 4, 0][(seed / 3 % 4) as usize],
                );
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 2, u32::MAX][(seed / 12 % 3) as usize]);
                // Unaligned out pointer or unaligned digest, per seed digit.
                let (out_off, digest_off) = [(0, 1), (3, 0)][(seed / 36 % 2) as usize];
                fill(c, &mut r, DATA + 0x4200, 4);
                fill(c, &mut r, DATA + 0x4400, 4);
                // The object the final step leaves in the caller's out slot.
                c.bus.write32(crate::difftest::SP - 0x30, DATA + 0x4300);
                c.bus.write32(DATA + 0x4304, DATA + 0x4400 + digest_off);
                c.g[A0][0] = sx(DATA + 0x4200 + out_off);
            },
        );
    }

    /// Encrypt/decrypt fixture: no cipher, set-key fails, transform fails,
    /// reset fails, or all succeed; unwinding at fake call 0, 1, 2 or never.
    pub(crate) fn slogin_store_cipher_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = store_io_machine(c, seed, entry as u64);
        c.pc = entry;
        let arm = seed % 5;
        if arm == 0 {
            c.bus.write32(STORE_TEST_STATE + store::CIPHER_IFACE, 0);
        }
        c.bus
            .write32(STORE_IO_IFACE_FAIL_MASK, [0, 1, 2, 4, 0][arm as usize]);
        c.bus
            .write32(LOG_UNWIND_AT, [0, 1, 2, u32::MAX][(seed / 5 % 4) as usize]);
        c.g[A0][0] = sx(DATA + 0x4000);
        c.g[A1][0] = (r.next_u32() & 0xfff) as u64;
    }

    #[test]
    fn slogin_store_encrypt_matches() {
        fakes::covered(
            ee::s15::f_003fbc40__generated,
            slogin_store_encrypt,
            60,
            &|c, seed| {
                slogin_store_cipher_fixture(c, seed, 0x003f_bc40);
            },
        );
    }

    #[test]
    fn slogin_store_decrypt_matches() {
        fakes::covered(
            ee::s15::f_003fbcd8__generated,
            slogin_store_decrypt,
            60,
            &|c, seed| {
                slogin_store_cipher_fixture(c, seed, 0x003f_bcd8);
            },
        );
    }

    #[test]
    fn slogin_store_begin_open_matches() {
        check(
            ee::s15::f_003fbd70__generated,
            slogin_store_begin_open,
            40,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbd_7000);
                c.pc = 0x003f_bd70;
                // StartOpen: uninitialised (2), no name (12), bad device (9),
                // device not in the mask (11), busy (13).
                let mode = seed % 5;
                slogin_store_mc_fixture(
                    c,
                    match mode {
                        0 => 0,
                        3 => 2,
                        _ => 1,
                    },
                );
                c.g[A0][0] = if mode == 1 { 0 } else { sx(DATA + 0x4000) };
                // Device 8 is out of range; -1 is the automatic device.
                c.g[A1][0] = match mode {
                    2 => 8,
                    4 if seed / 5 % 2 == 1 => u64::MAX,
                    _ => 2,
                };
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn slogin_store_seek_to_start_matches() {
        check(
            ee::s15::f_003fbdb8__generated,
            slogin_store_seek_to_start,
            16,
            &|c, seed| {
                let _r = store_io_machine(c, seed, 0x3fbd_b800);
                c.pc = 0x003f_bdb8;
                slogin_store_mc_fixture(c, seed % 2);
            },
        );
    }

    #[test]
    fn slogin_store_read_dir_header_matches() {
        check(
            ee::s15::f_003fbdf0__generated,
            slogin_store_read_dir_header,
            16,
            &|c, seed| {
                let _r = store_io_machine(c, seed, 0x3fbd_f000);
                elf_image(c);
                c.pc = 0x003f_bdf0;
                slogin_store_mc_fixture(c, seed % 2);
            },
        );
    }

    #[test]
    fn slogin_store_load_dir_entry_matches() {
        fakes::covered(
            ee::s15::f_003fbe40__generated,
            slogin_store_load_dir_entry,
            72,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbe_4000);
                c.pc = 0x003f_be40;
                // 0: decrypt fails; 1: directory full (returns 5); 2: entry added
                // after others (seek); 3: first entry (seek), no cipher.
                let arm = seed % 4;
                if arm == 1 || arm == 3 {
                    c.bus.write32(STORE_TEST_STATE + store::CIPHER_IFACE, 0);
                }
                let fail = if arm == 0 {
                    [1, 2, 4][(seed / 4 % 3) as usize]
                } else {
                    0
                };
                c.bus.write32(STORE_IO_IFACE_FAIL_MASK, fail);
                let (count, capacity) = [(2, 6), (3, 3), (2, 6), (0, 6)][arm as usize];
                c.bus.write32(STORE_TEST_STATE + store::RECORD_COUNT, count);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_CAPACITY, capacity);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORDS, STORE_TEST_RECORDS);
                c.bus
                    .write32(STORE_TEST_STATE + store::DIR_BUFFER, DATA + 0x4000);
                fill(c, &mut r, STORE_TEST_RECORDS, 4 * 8);
                fill(c, &mut r, DATA + 0x4000, 3);
                c.bus.write32(DATA + 0x4004, r.next_u32() & 0xfff);
                slogin_store_mc_fixture(c, seed / 12 % 2);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 1, u32::MAX][(seed / 24 % 3) as usize]);
            },
        );
    }

    /// seed % 5: 0 block header invalid (4), 1 header encrypt fails, 2
    /// payload encrypt fails, 3 written through the cipher, 4 no cipher;
    /// seed / 5 % 12: the corrupted header byte (arm 0) or, mod 3, the
    /// failing cipher call (arms 1 and 2); seed / 60 % 2: card layer not
    /// initialized or busy; seed / 120 % 3: unwind at fake call 0, 4 or none.
    #[test]
    fn slogin_store_write_record_matches() {
        fakes::covered(
            ee::s15::f_003fbed0__generated,
            slogin_store_write_record,
            360,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbe_d000);
                elf_image(c);
                c.pc = 0x003f_bed0;
                let arm = seed % 5;
                let digit = seed / 5 % 12;
                let step = digit % 3;
                if arm == 4 {
                    c.bus.write32(STORE_TEST_STATE + store::CIPHER_IFACE, 0);
                }
                let fail = match arm {
                    1 => 1 << step,
                    2 => 8 << step,
                    _ => 0,
                };
                c.bus.write32(STORE_IO_IFACE_FAIL_MASK, fail);
                let buffer = DATA + 0x5010;
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_BUFFER, buffer);
                for i in 0..BLOCK_HEADER_BYTES {
                    let b = c.bus.read8(BLOCK_HEADER_MAGIC + i);
                    let corrupt = arm == 0 && i == digit;
                    c.bus.write8(
                        buffer - BLOCK_HEADER_BYTES + i,
                        if corrupt { b ^ 0x20 } else { b },
                    );
                }
                fill(c, &mut r, buffer, 8);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_SIZE, r.next_u32() & 0xfff);
                slogin_store_mc_fixture(c, seed / 60 % 2);
                c.bus
                    .write32(LOG_UNWIND_AT, [0, 4, u32::MAX][(seed / 120 % 3) as usize]);
            },
        );
    }

    #[test]
    fn slogin_store_read_record_matches() {
        check(
            ee::s15::f_003fbf80__generated,
            slogin_store_read_record,
            32,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbf_8000);
                elf_image(c);
                c.pc = 0x003f_bf80;
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_BUFFER, DATA + 0x5010);
                let size = [0, 13, r.next_u32() & 0xfff, 0xffff_fff4][(seed / 2 % 4) as usize];
                c.bus.write32(STORE_TEST_STATE + store::RECORD_SIZE, size);
                slogin_store_mc_fixture(c, seed % 2);
            },
        );
    }

    // Control block of the store fakes (in each test machine's RAM).
    pub(crate) const STORE_FAKE_CTRL: u32 = DATA + 0xf000;

    pub(crate) const STORE_FAKE_MAGIC: u32 = 0x534c_5354;

    /// Allocations made so far, and the pointer each one returns (0 fails).
    pub(crate) const STORE_FAKE_ALLOC_COUNT: u32 = STORE_FAKE_CTRL + 4;

    pub(crate) const STORE_FAKE_ALLOC_RESULTS: u32 = STORE_FAKE_CTRL + 0x10;

    pub(crate) const STORE_FAKE_HASH_RESULT: u32 = STORE_FAKE_CTRL + 0x3c;

    /// Fake checksum and cipher interface tables (slot i is `FAKE + base +
    /// 0x10 * i`); slot 5 of the checksum table (+0x14) is the hash's final
    /// call, which writes its result pointer.
    pub(crate) const STORE_FAKE_HASH_TABLE: u32 = DATA + 0xa000;

    pub(crate) const STORE_FAKE_CIPHER_TABLE: u32 = DATA + 0xa100;

    pub(crate) const STORE_FAKE_HASH_FINAL: u32 = FAKE + 0x100 + 0x50;

    pub(crate) const STORE_LIFECYCLE_TEST_STATE: u32 = DATA + 0x1000;

    pub(crate) const STORE_TEST_HASH_RESULT: u32 = DATA + 0xb000;

    pub(crate) const STORE_TEST_HASH_CRC: u32 = DATA + 0xb010;

    pub(crate) static STORE_ALLOC_PREV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static STORE_FREE_PREV: AtomicUsize = AtomicUsize::new(0);

    /// `TibExtMemAlloc` for the store, CRC-32 and block tests (gate
    /// `STORE_FAKE_CTRL`): logged, returning allocation k's pointer from
    /// `STORE_FAKE_ALLOC_RESULTS` (0: it fails).
    pub(crate) fn slogin_store_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(STORE_FAKE_CTRL) != STORE_FAKE_MAGIC {
            return fakes::fall_back(c, &STORE_ALLOC_PREV, ee::s07::f_00266728__generated);
        }
        let k = c.bus.read32(STORE_FAKE_ALLOC_COUNT);
        c.bus.write32(STORE_FAKE_ALLOC_COUNT, k + 1);
        fakes::fake_call(c, callee::TIB_EXT_MEM_ALLOC.addr);
        c.g[V0][0] = if k < 8 {
            c.bus.read32(STORE_FAKE_ALLOC_RESULTS + 4 * k) as u64
        } else {
            0
        };
    }

    /// `TibExtMemFree` for the same tests: logged only.
    pub(crate) fn slogin_store_fake_free(c: &mut Cpu) {
        if c.bus.read32(STORE_FAKE_CTRL) != STORE_FAKE_MAGIC {
            return fakes::fall_back(c, &STORE_FREE_PREV, ee::s07::f_00266760__generated);
        }
        fakes::fake_call(c, callee::TIB_EXT_MEM_FREE.addr);
    }

    pub(crate) fn slogin_store_hook_heap_fakes() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266728),
            slogin_store_fake_alloc,
            &STORE_ALLOC_PREV,
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00266760),
            slogin_store_fake_free,
            &STORE_FREE_PREV,
        );
    }

    #[test]
    fn slogin_store_verify_record_matches() {
        const HEADER: u32 = DATA + 0x2000;
        fakes::covered(
            ee::s15::f_003fbfe0__generated,
            slogin_store_verify_record,
            144,
            &|c, seed| {
                // seed % 8: the arm; seed / 8 % 2: cipher present (arms 0 and 3
                // always); seed / 16 % 3: which interface call of the failing
                // step fails; seed / 48: no unwind, or unwind in the first or
                // second group of three interface calls.
                let class = seed % 8;
                let cipher = matches!(class, 0 | 3) || seed / 8 % 2 == 1;
                let step = seed / 16 % 3;
                let unwind_at = match seed / 48 {
                    0 => u32::MAX,
                    g => 3 * (g - 1) + step,
                };
                let mut r = store_lifecycle_machine(c, seed, 0x3fbf_e000, 0x003f_bfe0, unwind_at);
                let state = STORE_LIFECYCLE_TEST_STATE;
                if cipher {
                    c.bus
                        .write32(state + store::CIPHER_IFACE, STORE_FAKE_CIPHER_TABLE);
                }
                let cipher_calls = if cipher { 3 } else { 0 };
                c.bus.write32(
                    STORE_LIFECYCLE_IFACE_FAIL_AT,
                    match class {
                        0 => step,
                        3 => 3 + step,
                        4 => 2 * cipher_calls + step,
                        _ => u32::MAX,
                    },
                );
                let kind = 0x11 + seed % 5;
                let size = 0x40 + seed % 7;
                let crc = 0xc0de_0000 | seed;
                c.bus
                    .write32(state + store::RECORD_BUFFER, HEADER + BLOCK_HEADER_BYTES);
                c.bus.write32(state + store::RECORD_KIND, kind);
                c.bus.write32(state + store::RECORD_SIZE, size);
                c.bus
                    .write32(HEADER, if class == 1 { kind ^ 1 } else { kind });
                c.bus
                    .write32(HEADER + 4, if class == 2 { size + 1 } else { size });
                c.bus.write32(HEADER + 8, crc);
                fill(c, &mut r, HEADER + BLOCK_HEADER_BYTES, 0x48 / 4);
                c.bus
                    .write32(STORE_TEST_HASH_RESULT, if class == 5 { 5 } else { 4 });
                c.bus.write32(
                    STORE_TEST_HASH_CRC,
                    if class == 7 { crc ^ 0x100 } else { crc },
                );
            },
        );
    }

    #[test]
    fn slogin_store_is_initialized_matches() {
        check(
            ee::s15::f_003fc0d8__generated,
            slogin_store_is_initialized,
            48,
            &|c, seed| {
                let mut r = Rng(0x3fc0_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_c0d8;
                c.g[GP][0] = GAME_GP as u64;
                let state = match seed % 3 {
                    0 => 0,
                    1 => DATA,
                    _ => 0x8000_0000,
                };
                c.bus.write32(GAME_GP + store::STATE_GP, state);
            },
        );
    }

    #[test]
    fn slogin_store_init_matches() {
        const CONFIG: u32 = DATA + 0x3000;
        const DEVICES: u32 = DATA + 0x3100;
        const MC_FUNCS: u32 = DATA + 0x3200;
        const KEY: u32 = DATA + 0x3300;
        const HEAP: u32 = DATA + 0x3400;
        slogin_store_hook_heap_fakes();
        fakes::covered(
            ee::s15::f_003fc0e8__generated,
            slogin_store_init,
            120,
            &|c, seed| {
                // seed % 10: the arm (0 already initialized, 1 card layer fails,
                // 2 state allocation fails, 3 checksum init fails, 4 cipher init
                // fails, 5 key allocation fails, 6 directory allocation fails,
                // 7 block allocation fails, 8 success, 9 success with a cipher);
                // seed / 10 % 2: cipher present for the arms where it is free;
                // seed / 20: unwind at fake call 1..5 (0: none).
                let class = seed % 10;
                let cipher = matches!(class, 4 | 5 | 9) || (class != 8 && seed / 10 % 2 == 1);
                let unwind_at = match seed / 20 % 6 {
                    0 => u32::MAX,
                    u => u,
                };
                elf_image(c);
                let mut r = store_lifecycle_machine(c, seed, 0x3fc0_e800, 0x003f_c0e8, unwind_at);
                // The state block the allocation hands back: its cipher slot
                // holds junk, so the body's store of the config's cipher (0 in
                // the no-cipher seeds) is compared.
                c.bus.write32(
                    STORE_LIFECYCLE_TEST_STATE + store::CIPHER_IFACE,
                    0x5a5a_0000 | seed,
                );
                c.bus.write32(
                    GAME_GP + store::STATE_GP,
                    if class == 0 {
                        STORE_LIFECYCLE_TEST_STATE
                    } else {
                        0
                    },
                );
                c.bus.write32(GAME_GP + card::IFACE_GP, 0);
                c.bus.write32(GAME_GP + card::STATE_GP, 0);
                // Crc32_GetInterface's table: the checksum interface Init sets up.
                for i in 0..8 {
                    c.bus
                        .write32(CRC32_INTERFACE + 4 * i, FAKE + 0x100 + 0x10 * i);
                }
                c.bus.write32(
                    STORE_LIFECYCLE_IFACE_FAIL_AT,
                    match class {
                        3 => 0,
                        4 => 1,
                        _ => u32::MAX,
                    },
                );
                c.g[A0][0] = sx(CONFIG);
                c.bus
                    .write32(CONFIG + store::CONFIG_MAX_RECORDS, 2 + seed % 3);
                c.bus.write32(
                    CONFIG + store::CONFIG_DEVICES,
                    if class == 1 { 0 } else { DEVICES },
                );
                c.bus.write32(CONFIG + store::CONFIG_MC_FUNCS, MC_FUNCS);
                c.bus.write32(CONFIG + store::CONFIG_KEY, KEY);
                c.bus
                    .write32(CONFIG + store::CONFIG_KEY_LEN, 0x10 + seed % 0x20);
                c.bus.write32(
                    CONFIG + store::CONFIG_CIPHER,
                    if cipher { STORE_FAKE_CIPHER_TABLE } else { 0 },
                );
                c.bus.write32(CONFIG + store::CONFIG_HEAP, HEAP);
                c.bus.write32(DEVICES, 0);
                c.bus.write32(DEVICES + 4, u32::MAX);
                for i in 0..17 {
                    c.bus.write32(MC_FUNCS + 4 * i, FAKE + 0x300 + 0x10 * i);
                }
                fill(c, &mut r, KEY, 0x30 / 4);
                let mut results = vec![DATA + 0x4000, DATA + 0x4800];
                results.push(if class == 2 {
                    0
                } else {
                    STORE_LIFECYCLE_TEST_STATE
                });
                if cipher {
                    results.push(if class == 5 { 0 } else { DATA + 0x5000 });
                }
                results.push(if class == 6 { 0 } else { DATA + 0x5400 });
                results.push(if class == 7 { 0 } else { DATA + 0x5800 });
                for (k, p) in results.iter().enumerate() {
                    c.bus.write32(STORE_FAKE_ALLOC_RESULTS + 4 * k as u32, *p);
                }
                fill(c, &mut r, DATA + 0x5000, 0x40 / 4);
                fill(c, &mut r, DATA + 0x5800, 0x40 / 4);
            },
        );
    }

    /// seed % 7: the arm (0 not initialized, 1 cipher shutdown fails,
    /// 2 checksum shutdown fails, 3 card layer shutdown fails, 4 success,
    /// 5 success with a cipher, 6 FreeBlock fails through the gated fake);
    /// seed / 7 % 2: cipher present for arms 2 and 3, and for arm 6 the
    /// error (4, or 0x80000004 sign-extended); seed / 14: unwind at fake
    /// call 0..6 (0: none). Fakes vary the cycle totals, so the arm set is
    /// not checked by path costs; the FreeBlock-failed arm is asserted
    /// through the fake's counter.
    #[test]
    fn slogin_store_shutdown_matches() {
        const BLOCK: u32 = DATA + 0x6000;
        slogin_store_hook_heap_fakes();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FCF00),
            store_shutdown_free_block_fake,
            &PREV_STORE_SHUTDOWN_FREE_BLOCK,
        );
        let fails_before = STORE_SHUTDOWN_FREE_FAILS.load(std::sync::atomic::Ordering::Relaxed);
        fakes::covered(
            ee::s15::f_003fc260__generated,
            slogin_store_shutdown,
            112,
            &|c, seed| {
                let class = seed % 7;
                let cipher =
                    matches!(class, 1 | 5) || (matches!(class, 2 | 3) && seed / 7 % 2 == 1);
                let unwind_at = match seed / 14 % 8 {
                    0 => u32::MAX,
                    u => u - 1,
                };
                elf_image(c);
                let mut r = store_lifecycle_machine(c, seed, 0x3fc2_6000, 0x003f_c260, unwind_at);
                let state = STORE_LIFECYCLE_TEST_STATE;
                if class == 0 {
                    c.bus.write32(GAME_GP + store::STATE_GP, 0);
                }
                if class == 6 {
                    c.bus
                        .write32(STORE_SHUTDOWN_FREE_CTRL, STORE_SHUTDOWN_FREE_MAGIC);
                    c.bus.write32(
                        STORE_SHUTDOWN_FREE_ERROR,
                        if seed / 7 % 2 == 0 { 4 } else { 0x8000_0004 },
                    );
                }
                c.bus.write32(state + store::DIR_BUFFER, BLOCK);
                fill(c, &mut r, BLOCK, 0x20 / 4);
                c.bus.write32(state + store::HEAP, DATA + 0x3400);
                c.bus.write32(state + store::RECORD_CAPACITY, 2 + seed % 4);
                c.bus.write32(state + store::RECORDS, DATA + 0x5400);
                if cipher {
                    c.bus
                        .write32(state + store::CIPHER_IFACE, STORE_FAKE_CIPHER_TABLE);
                }
                c.bus.write32(
                    STORE_LIFECYCLE_IFACE_FAIL_AT,
                    match class {
                        1 => 0,
                        2 => cipher as u32,
                        _ => u32::MAX,
                    },
                );
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if class == 3 { 0 } else { DATA + 0x4000 },
                );
                c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x4800);
                c.bus.write32(DATA + 0x4000 + 0x428, DATA + 0x3400);
            },
        );
        // Seed 6 is the first FreeBlock-failed seed (no unwind).
        assert!(
            crate::difftest::seed_count(112) < 7
                || STORE_SHUTDOWN_FREE_FAILS.load(std::sync::atomic::Ordering::Relaxed)
                    > fails_before,
            "the FreeBlock-failed arm was never taken"
        );
    }

    /// The card layer's short paths: not initialized (`SLoginMc_*` return 2),
    /// or initialized and busy (the state word at `gp+0x1ae8` points at a
    /// non-zero word: the seek and close starts return 0xd).
    pub(crate) fn slogin_store_card_busy(c: &mut Cpu, busy: bool) {
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if busy { DATA + 0x4000 } else { 0 },
        );
        c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x4800);
        c.bus.write32(DATA + 0x4800, 1);
    }

    #[test]
    fn slogin_store_create_matches() {
        check(
            ee::s15::f_003fc390__generated,
            slogin_store_create,
            48,
            &|c, seed| {
                let _r = store_lifecycle_machine(c, seed, 0x3fc3_9000, 0x003f_c390, u32::MAX);
                // seed % 2: card layer not initialized (2), or initialized with
                // a null first argument (0xc).
                let not_init = seed % 2 == 0;
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if not_init { 0 } else { DATA + 0x4000 },
                );
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn slogin_store_delete_matches() {
        check(
            ee::s15::f_003fc3c0__generated,
            slogin_store_delete,
            48,
            &|c, seed| {
                let _r = store_lifecycle_machine(c, seed, 0x3fc3_c000, 0x003f_c3c0, u32::MAX);
                let not_init = seed % 2 == 0;
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if not_init { 0 } else { DATA + 0x4000 },
                );
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn slogin_store_open_matches() {
        check(
            ee::s15::f_003fc4c8__generated,
            slogin_store_open,
            48,
            &|c, seed| {
                let _r = store_lifecycle_machine(c, seed, 0x3fc4_c800, 0x003f_c4c8, u32::MAX);
                let not_init = seed % 2 == 0;
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if not_init { 0 } else { DATA + 0x4000 },
                );
                c.g[A0][0] = 0;
            },
        );
    }

    #[test]
    fn slogin_store_close_matches() {
        const DIR: u32 = DATA + 0x7000;
        check(
            ee::s15::f_003fc4e8__generated,
            slogin_store_close,
            72,
            &|c, seed| {
                // seed % 2: empty directory (no hash) or hashed; seed / 2 % 2:
                // card layer not initialized or busy; seed / 6 % 4: unwind at
                // hash call 0..2 (3: none).
                let hashed = seed % 2 == 1;
                let unwind_at = match seed / 6 % 4 {
                    3 => u32::MAX,
                    u => u,
                };
                let mut r = store_lifecycle_machine(c, seed, 0x3fc4_e800, 0x003f_c4e8, unwind_at);
                c.bus.write32(
                    STORE_LIFECYCLE_TEST_STATE + store::RECORD_COUNT,
                    if hashed { 1 + seed % 3 } else { 0 },
                );
                c.bus
                    .write32(STORE_LIFECYCLE_TEST_STATE + store::RECORDS, DATA + 0x5400);
                fill(c, &mut r, DIR, 0x70 / 4);
                slogin_store_card_busy(c, seed / 2 % 2 == 1);
                c.g[A0][0] = sx(DIR);
            },
        );
    }

    /// The directory (3 records: kinds 5, 7, 5), junk directory-header bytes
    /// at `DATA + 0x7000` and the card layer's short path.
    pub(crate) fn slogin_store_record_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        capacity: u32,
        mc_busy: bool,
    ) {
        const RECORDS: u32 = DATA + 0x8000;
        let state = c.bus.read32(GAME_GP + store::STATE_GP);
        c.bus.write32(state + store::RECORD_COUNT, 3);
        c.bus.write32(state + store::RECORD_CAPACITY, capacity);
        c.bus.write32(state + store::RECORDS, RECORDS);
        c.bus.write32(state + store::DATA_END, 0x2000);
        fill(c, r, RECORDS, 0x50 / 4);
        for (i, kind) in [5u32, 7, 5].iter().enumerate() {
            let rec = RECORDS + 0x10 * i as u32;
            c.bus.write32(rec, *kind);
            c.bus.write32(rec + RECORD_OFFSET, 0x400 * (i as u32 + 1));
            c.bus.write32(rec + RECORD_SIZE, 0x40 + i as u32);
        }
        fill(c, r, DATA + 0x7000, 0x70 / 4);
        slogin_store_card_busy(c, mc_busy);
    }

    /// Store state and directory-header addresses for the alignment case
    /// `case` (0..5): both 8-aligned, the source 4- or 1-off, the state
    /// 4-off, or both 4-off (the `|` of the two is misaligned, their `^` is
    /// not: the unaligned copy loop).
    pub(crate) fn slogin_store_copy_addrs(case: u32) -> (u32, u32) {
        match case {
            0 => (STORE_LIFECYCLE_TEST_STATE, DATA + 0x7000),
            1 => (STORE_LIFECYCLE_TEST_STATE, DATA + 0x7004),
            2 => (STORE_LIFECYCLE_TEST_STATE, DATA + 0x7001),
            3 => (STORE_LIFECYCLE_TEST_STATE + 4, DATA + 0x7000),
            _ => (STORE_LIFECYCLE_TEST_STATE + 4, DATA + 0x7004),
        }
    }

    pub(crate) fn slogin_store_write_fixture(c: &mut Cpu, seed: u32, unwind_at: u32) -> Rng {
        // seed % 5: alignment; seed / 5 % 3: hash fails, record found,
        // record missing; seed / 15 % 3: which hash call fails / which
        // occurrence; seed / 45 % 2: card layer not initialized or busy;
        // seed / 90 % 2: directory full or not.
        let mut r = store_lifecycle_machine(c, seed, 0x3fc5_4800, 0x003f_c548, unwind_at);
        let (state, src) = slogin_store_copy_addrs(seed % 5);
        c.bus.write32(GAME_GP + store::STATE_GP, state);
        c.bus
            .write32(state + store::HASH_IFACE, STORE_FAKE_HASH_TABLE);
        let arm = seed / 5 % 3;
        let digit = seed / 15 % 3;
        slogin_store_record_fixture(c, &mut r, 3 + seed / 90 % 2, seed / 45 % 2 == 1);
        c.bus.write32(
            STORE_LIFECYCLE_IFACE_FAIL_AT,
            if arm == 0 { digit } else { u32::MAX },
        );
        c.g[A0][0] = sx(src);
        c.g[A1][0] = match arm {
            1 => 5,
            _ => {
                if digit == 0 {
                    9
                } else {
                    5
                }
            }
        };
        c.g[A2][0] = match arm {
            1 => (digit % 2) as u64,
            _ => 2,
        };
        c.g[A3][0] = sx(DATA + 0x9000);
        c.g[T0][0] = (0x40 + seed % 9) as u64;
        r
    }

    /// The fixture's 180 digit combinations, crossed with seed / 180 % 7:
    /// unwind at hash call 0..5, or (6) none.
    #[test]
    fn slogin_store_write_matches() {
        check(
            ee::s15::f_003fc548__generated,
            slogin_store_write,
            180 * 7,
            &|c, seed| {
                let unwind_at = match seed / 180 % 7 {
                    u if u < 6 => u,
                    _ => u32::MAX,
                };
                let _r = slogin_store_write_fixture(c, seed, unwind_at);
            },
        );
    }

    crate::difftest::events::driven!(driven_slogin_store_write, slogin_store_write);

    #[test]
    fn slogin_store_write_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc548__generated,
            driven_slogin_store_write,
            90,
            &|c, seed| {
                // seed % 3: the event mode, crossed with the fixture's digits.
                let mut r = slogin_store_write_fixture(c, seed, u32::MAX);
                schedule(c, &mut r, seed, 0x003f_c548)
            },
        );
    }

    pub(crate) fn slogin_store_read_fixture(c: &mut Cpu, seed: u32) -> Rng {
        // seed % 5: alignment; seed / 5 % 3: record missing, size mismatch,
        // found; seed / 15 % 2: card layer not initialized or busy;
        // seed / 30 % 2: which occurrence of kind 5.
        let mut r = store_lifecycle_machine(c, seed, 0x3fc7_0800, 0x003f_c708, u32::MAX);
        let (state, src) = slogin_store_copy_addrs(seed % 5);
        c.bus.write32(GAME_GP + store::STATE_GP, state);
        let arm = seed / 5 % 3;
        let occurrence = seed / 30 % 2;
        slogin_store_record_fixture(c, &mut r, 3, seed / 15 % 2 == 1);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = if arm == 0 { 9 } else { 5 };
        c.g[A2][0] = occurrence as u64;
        c.g[A3][0] = sx(DATA + 0x9000);
        let size = 0x40 + 2 * occurrence;
        c.g[T0][0] = (if arm == 1 { size + 1 } else { size }) as u64;
        r
    }

    #[test]
    fn slogin_store_read_matches() {
        check(
            ee::s15::f_003fc708__generated,
            slogin_store_read,
            120,
            &|c, seed| {
                let _r = slogin_store_read_fixture(c, seed);
            },
        );
    }

    crate::difftest::events::driven!(driven_slogin_store_read, slogin_store_read);

    #[test]
    fn slogin_store_read_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc708__generated,
            driven_slogin_store_read,
            60,
            &|c, seed| {
                // seed % 3: the event mode, crossed with the fixture's digits.
                let mut r = slogin_store_read_fixture(c, seed);
                schedule(c, &mut r, seed, 0x003f_c708)
            },
        );
    }

    /// The first special section index (`SHN_LORESERVE`).
    pub(crate) const ELF_SPECIAL_SECTION_MIN: u64 = 0xff00;

    #[test]
    fn dnas_rel_call_update_matches() {
        fakes::covered(
            ee::s15::f_003fd100__generated,
            dnas_rel_call_update,
            96,
            &|c, seed| {
                dnas_relay_call_fixture(c, seed, DNAS_UPDATE_SLOT);
            },
        );
    }

    #[test]
    fn dnas_rel_call_exit_matches() {
        fakes::covered(
            ee::s15::f_003fd128__generated,
            dnas_rel_call_exit,
            96,
            &|c, seed| {
                dnas_relay_call_fixture(c, seed, DNAS_EXIT_SLOT);
            },
        );
    }

    /// Fake call indices: 0 the relay's init, then per trip the user update
    /// (unless it is the real `DnasRel_IsElf`) and the relay's update, then
    /// its exit. seed % 3 is the route: 0 fake user update, the relay update
    /// returns 2 at once (calls: init, user, update, exit); 1 the real
    /// `DnasRel_IsElf` as user update (init, update, exit); 2 the relay
    /// update returns 1 once, so a second trip (init, user, update, user,
    /// update, exit). seed / 3 % 5 unwinds at no call or at fake call 0..3,
    /// so every `?` site unwinds on some route: the init (0), the user
    /// update (1 on routes 0/2, 3 on route 2 after the back edge), the relay
    /// update (2 on routes 0/2, 1 on route 1) and the exit (3 on route 0,
    /// 2 on route 1).
    #[test]
    fn dnas_rel_run_matches() {
        fakes::covered(
            ee::s15::f_003fd150__generated,
            dnas_rel_run,
            90,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_d150, 0);
                let route = seed % 3;
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = if route == 1 {
                    callee::DNAS_REL_IS_ELF.addr
                } else {
                    fakes::FAKE + 0x40
                } as u64;
                c.g[A3][0] = sx(DATA);
                c.bus.write32(DATA, 0);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_INIT_SLOT, fakes::FAKE + 0x10);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_UPDATE_SLOT, fakes::FAKE + 0x20);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_EXIT_SLOT, fakes::FAKE + 0x30);
                // The relay update's results (1 repeats, 2 ends the loop).
                match route {
                    0 => c.bus.write32(fakes::LOG_RESULTS + 8, 2),
                    1 => c.bus.write32(fakes::LOG_RESULTS + 4, 2),
                    _ => {
                        c.bus.write32(fakes::LOG_RESULTS + 8, 1);
                        c.bus.write32(fakes::LOG_RESULTS + 16, 2);
                    }
                }
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    [u32::MAX, 0, 1, 2, 3][(seed / 3 % 5) as usize],
                );
            },
        );
    }

    /// The header word (seed % 4): the magic, the magic with bit 0 flipped
    /// (the `< 1` compare's boundary), 0 or all ones.
    #[test]
    fn dnas_rel_is_elf_matches() {
        check(
            ee::s15::f_003fd1d8__generated,
            dnas_rel_is_elf,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d1d8 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = callee::DNAS_REL_IS_ELF.addr;
                c.g[A0][0] = sx(DATA);
                let word = match seed % 4 {
                    0 => ELF_MAGIC as u32,
                    1 => ELF_MAGIC as u32 ^ 1,
                    2 => 0,
                    _ => 0xffff_ffff,
                };
                c.bus.write32(DATA, word);
            },
        );
    }

    #[test]
    fn dnas_rel_get_section_header_matches() {
        check_lohi(
            ee::s15::f_003fd1f0__generated,
            dnas_rel_get_section_header,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d1f0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx((seed % 5) * 4);
                c.bus.write32(DATA, DATA + 0x100);
                c.bus.write32(DATA + 0x120, (seed % 7) * 0x28);
            },
        );
    }

    /// Section index seed % 4: 0 and `ELF_SPECIAL_SECTION_END` (the last
    /// ordinary index) take the section-header path (both headers hold a
    /// non-zero `sh_offset`), `SHN_ABS` gives 0xfff00000 and `SHN_LORESERVE`
    /// gives 0.
    #[test]
    fn dnas_rel_get_section_addr_matches() {
        check(
            ee::s15::f_003fd2b8__generated,
            dnas_rel_get_section_addr,
            96,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d2b8, 1);
                c.pc = 0x003f_d2b8;
                let image = DATA + 0x100;
                let headers = image + 0x40;
                let index = [
                    0,
                    ELF_SPECIAL_SECTION_END,
                    ELF_SECTION_ABS,
                    ELF_SPECIAL_SECTION_MIN,
                ][(seed % 4) as usize];
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = index;
                c.bus.write32(DATA, image);
                c.bus.write32(image + ELF_SECTION_OFFSET, headers - image);
                for i in [0, ELF_SPECIAL_SECTION_END as u32] {
                    c.bus.write32(
                        headers + i * ELF_SECTION_SIZE as u32 + ELF_SECTION_FILE_OFFSET,
                        fakes::ptr(&mut r),
                    );
                }
            },
        );
    }

    /// Arms: seed % 4 = 0 no section headers (every lookup fails), 1 all six
    /// sections (three relocation passes), 2 the target section of pass
    /// seed / 4 % 3 renamed, 3 that pass's relocation section renamed;
    /// symbol shift seed / 12 % 6. All callees are real.
    #[test]
    fn dnas_rel_relocate_all_matches() {
        check(
            ee::s15::f_003fd660__generated,
            dnas_rel_relocate_all,
            72,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d660, 0);
                let pass = seed / 4 % 3;
                let (count, renamed) = match seed % 4 {
                    0 => (0, None),
                    1 => (7, None),
                    2 => (7, Some(2 * pass)),
                    _ => (7, Some(2 * pass + 1)),
                };
                dnas_rel_relocate_all_fixture(c, &mut r, count, renamed, seed / 12 % 6);
            },
        );
    }

    /// Arms: seed % 3 = 0 no symbol table, 1 no string table, 2 both: symbol
    /// seed / 3 % 4 (four entries with distinct names, values and sections:
    /// 0 and 1 through their section headers, `SHN_ABS`, `SHN_LORESERVE`) is
    /// copied into the pre-filled buffer at DATA+0x400 and its address
    /// resolved.
    #[test]
    fn dnas_rel_get_symbol_matches() {
        check(
            ee::s15::f_003fd6e0__generated,
            dnas_rel_get_symbol,
            96,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d6e0, 2);
                c.pc = 0x003f_d6e0;
                let image = DATA + 0x1000;
                let headers = image + 0x100;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed / 3 % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x400);
                for i in 0..0x10 {
                    c.bus.write8(DATA + 0x400 + i, 0xa5);
                }
                c.bus.write32(DATA, image);
                c.bus
                    .write32(DATA + 0x10, if seed % 3 == 0 { 0 } else { DATA + 0x500 });
                c.bus
                    .write32(DATA + 0x20, if seed % 3 == 1 { 0 } else { DATA + 0x700 });
                let sections = [0, 1, ELF_SECTION_ABS as u16, ELF_SPECIAL_SECTION_MIN as u16];
                for (i, section) in sections.into_iter().enumerate() {
                    let i = i as u32;
                    let sym = DATA + 0x500 + 0x10 * i;
                    c.bus.write32(sym, 4 * i);
                    c.bus.write32(sym + 4, fakes::ptr(&mut r));
                    c.bus.write16(sym + 0xe, section);
                    // Names "abc", "bcd", "cde", "def".
                    c.bus
                        .write32(DATA + 0x700 + 4 * i, 0x0063_6261 + 0x0001_0101 * i);
                }
                c.bus.write32(image + ELF_SECTION_OFFSET, headers - image);
                for i in 0..2 {
                    c.bus.write32(
                        headers + i * ELF_SECTION_SIZE as u32 + ELF_SECTION_FILE_OFFSET,
                        0x200 + 0x100 * i,
                    );
                }
            },
        );
    }

    /// The image has four sections ("x" names them); `.text`, `.symtab` and `.strtab` (the
    /// game's own strings, `elf_image`, copied) are each present or renamed by their own seed
    /// digit, so every found/missing pair of arms runs.
    #[test]
    fn dnas_rel_elf_open_matches() {
        check(
            ee::s15::f_003fd558__generated,
            dnas_rel_elf_open,
            60,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d558, 0);
                elf_image(c);
                let image = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(image);
                c.g[A2][0] = sx(fakes::ptr(&mut r));
                crate::difftest::fill(c, &mut r, DATA, 0xc);
                c.bus
                    .write32(image, if seed % 5 == 0 { 0 } else { ELF_MAGIC as u32 });
                c.bus.write32(
                    image + 0x10,
                    if seed % 5 == 1 {
                        0
                    } else {
                        ELF_RELAY_TYPE as u32
                    },
                );
                c.bus.write32(image + ELF_SECTION_OFFSET, 0x100);
                c.bus.write16(image + ELF_SECTION_COUNT, 4);
                c.bus.write16(image + ELF_SECTION_NAMES_INDEX, 0);
                let names = image + 0x40;
                dnas_put_cstr(c, names, b"x");
                let mut at = 2;
                let mut name_at = [0; 3];
                for (k, &name) in [DNAS_TEXT_NAME, DNAS_SYMTAB_NAME, DNAS_STRTAB_NAME]
                    .iter()
                    .enumerate()
                {
                    let len = dnas_load_copy_cstr(c, name, names + at);
                    if seed / (5 << k) % 2 == 1 {
                        c.bus.write8(names + at, b'-');
                    }
                    name_at[k] = at;
                    at += len + 1;
                }
                let headers = image + 0x100;
                let sections = [
                    (0, 0x40, 0x18),
                    (name_at[0], 0x200 + (seed % 3) * 0x10, 0x80 + seed),
                    (name_at[1], 0x300, 0x30 + seed % 16),
                    (name_at[2], 0x380, 0x20),
                ];
                for (i, (name, offset, size)) in sections.into_iter().enumerate() {
                    let h = headers + i as u32 * ELF_SECTION_SIZE as u32;
                    c.bus.write32(h + ELF_SECTION_NAME, name);
                    c.bus.write32(h + ELF_SECTION_FILE_OFFSET, offset);
                    c.bus.write32(h + ELF_SECTION_BYTES, size);
                }
            },
        );
    }

    #[test]
    fn dnas_rel_find_section_matches() {
        check_lohi(
            ee::s15::f_003fd210__generated,
            dnas_rel_find_section,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d210 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x1020, 0x100);
                c.bus.write16(
                    DATA + 0x1030,
                    if seed % 4 == 0 {
                        0
                    } else if seed % 4 == 1 {
                        1
                    } else {
                        2
                    },
                );
                c.bus.write16(DATA + 0x1032, 0);
                c.bus.write32(DATA + 0x1100, 0);
                c.bus.write32(DATA + 0x1110, 0x300);
                c.bus.write32(DATA + 0x1128, 8);
                c.bus.write32(DATA + 0x1300, 0x0000_0061);
                c.bus.write32(DATA + 0x1308, 0x0000_0062);
                c.bus.write32(
                    DATA + 0x500,
                    match seed % 4 {
                        1 => 0x61,
                        2 => 0x62,
                        _ => 0x63,
                    },
                );
            },
        );
    }

    /// Arms: seed % 6 = 0 no symbol table, 1 no string table, 2 no symbols,
    /// 3 the first symbol matches, 4 the second, 5 none. Entered at the
    /// start (the body dispatches on `pc`).
    #[test]
    fn dnas_rel_find_symbol_matches() {
        check_lohi(
            ee::s15::f_003fd758__generated,
            dnas_rel_find_symbol,
            96,
            &|c, seed| {
                let mut r = Rng(0x003f_d758 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_d758;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus
                    .write32(DATA + 0x10, if seed % 6 == 0 { 0 } else { DATA + 0x200 });
                c.bus
                    .write32(DATA + 0x14, if seed % 6 == 2 { 0 } else { 2 });
                c.bus
                    .write32(DATA + 0x20, if seed % 6 == 1 { 0 } else { DATA + 0x400 });
                c.bus.write32(DATA + 0x200, 0);
                c.bus.write32(DATA + 0x204, 0x20);
                c.bus.write16(DATA + 0x20e, ELF_SECTION_ABS as u16);
                c.bus.write32(DATA + 0x210, 6);
                c.bus.write32(DATA + 0x214, 0x40);
                c.bus.write16(DATA + 0x21e, ELF_SECTION_ABS as u16);
                c.bus.write32(DATA + 0x400, 0x6870_6c61);
                c.bus.write32(DATA + 0x404, 0x0000_0061);
                c.bus.write32(DATA + 0x406, 0x6174_6562);
                c.bus.write8(DATA + 0x40a, 0);
                c.bus.write32(
                    DATA + 0x500,
                    match seed % 6 {
                        3 => 0x6870_6c61,
                        4 => 0x6174_6562,
                        _ => 0x6d6d_6167,
                    },
                );
                c.bus
                    .write32(DATA + 0x504, if seed % 6 == 3 { 0x61 } else { 0 });
            },
        );
    }

    crate::difftest::events::driven!(driven_dnas_rel_find_symbol, dnas_rel_find_symbol_counted);

    crate::difftest::events::driven!(
        driven_dnas_rel_find_symbol_generated,
        ee::s15::f_003fd758__generated
    );

    /// Both sides re-entered at the 0x3fd7a0 head after an unwind. The symbol names are empty
    /// and odd-aligned, so `strcmp` returns before its own back edge and the first due event
    /// always lands on this function's head (an unwind inside `strcmp` would resume there,
    /// which `drive` cannot follow). The hand side itself must unwind at that head and be
    /// re-entered there, as the generated body is.
    #[test]
    fn dnas_rel_find_symbol_events() {
        use crate::difftest::events::{check_events, schedule};
        use std::sync::atomic::Ordering;
        let before = DNAS_FIND_SYMBOL_HAND_RESUMES.load(Ordering::Relaxed);
        check_events(
            driven_dnas_rel_find_symbol_generated,
            driven_dnas_rel_find_symbol,
            36,
            &|c, seed| {
                let mut r = Rng(0x003f_d758 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.bus.write32(DATA, DATA + 0x1000);
                c.bus.write32(DATA + 0x10, DATA + 0x200);
                c.bus.write32(DATA + 0x14, 2 + seed / 3 % 4);
                c.bus.write32(DATA + 0x20, DATA + 0x400);
                for i in 0..6 {
                    c.bus.write32(DATA + 0x200 + i * 0x10, 1);
                }
                c.bus.write8(DATA + 0x400, 0x7a);
                c.bus.write8(DATA + 0x401, 0);
                c.bus.write32(DATA + 0x500, 0x6d6d_6167);
                c.bus.write8(DATA + 0x504, 0x61);
                c.bus.write8(DATA + 0x505, 0);
                schedule(c, &mut r, seed, 0x003f_d758)
            },
        );
        let resumed = DNAS_FIND_SYMBOL_HAND_RESUMES.load(Ordering::Relaxed) - before;
        assert!(resumed > 0, "the hand side never resumed at 0x3fd7a0");
    }

    /// Relay layout for `dnas_rel_relocate_section`: the relay at `DATA` (image, symbol table,
    /// string table), its ELF image with sections "x" (section names), "t" (the patched words)
    /// and "r" (the entries), the symbols and the remap window.
    pub(crate) const DNAS_RELOC_IMAGE: u32 = DATA + 0x1000;

    pub(crate) const DNAS_RELOC_WORDS: u32 = DNAS_RELOC_IMAGE + 0x400;

    pub(crate) const DNAS_RELOC_ENTRIES: u32 = DNAS_RELOC_IMAGE + 0x800;

    pub(crate) const DNAS_RELOC_SYMTAB: u32 = DATA + 0x2000;

    pub(crate) const DNAS_RELOC_STRTAB: u32 = DATA + 0x2400;

    pub(crate) const DNAS_RELOC_REMAP: u32 = DATA + 0x3000;

    pub(crate) const DNAS_RELOC_REMAP_WORDS: u32 = DATA + 0x3100;

    pub(crate) const DNAS_RELOC_NAMES: u32 = DATA + 0x500;

    /// One relocation fixture. `entries` are (type, symbol) pairs patching words 0, 1, ...;
    /// `extra` bytes after the entries test the size's `>> 3`; `found` says whether the target
    /// and the relocation section are found; `named` gives symbols 1, 3 and 5 a one-letter
    /// name (otherwise every name is empty, so `strcpy` makes no back edge).
    /// Symbols: 0 below the remap window, 1 inside it (`inside` words in), 2 at its end,
    /// 3 relative to section 1 (below), 4 absolute (below), 5 far above.
    pub(crate) fn dnas_rel_relocate_section_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        entries: &[(u32, u32)],
        extra: u32,
        found: [bool; 2],
        inside: u32,
        named: bool,
    ) {
        let [target_found, rel_found] = found;
        c.pc = 0x003f_d2f8;
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DNAS_RELOC_NAMES + 1);
        c.g[A2][0] = sx(DNAS_RELOC_NAMES + 3);
        c.g[A3][0] = sx(DNAS_RELOC_REMAP);
        // Odd addresses: `strcmp` takes its byte path.
        c.bus
            .write8(DNAS_RELOC_NAMES + 1, if target_found { b't' } else { b'q' });
        c.bus.write8(DNAS_RELOC_NAMES + 2, 0);
        c.bus
            .write8(DNAS_RELOC_NAMES + 3, if rel_found { b'r' } else { b'q' });
        c.bus.write8(DNAS_RELOC_NAMES + 4, 0);
        c.bus.write32(DATA, DNAS_RELOC_IMAGE);
        c.bus.write32(DATA + 0x10, DNAS_RELOC_SYMTAB);
        c.bus.write32(DATA + 0x14, 6);
        c.bus.write32(DATA + 0x20, DNAS_RELOC_STRTAB);
        c.bus.write32(DNAS_RELOC_IMAGE + 0x20, 0x100);
        c.bus.write16(DNAS_RELOC_IMAGE + 0x30, 3);
        c.bus.write16(DNAS_RELOC_IMAGE + 0x32, 0);
        for (i, &b) in b"x\0t\0r\0".iter().enumerate() {
            c.bus.write8(DNAS_RELOC_IMAGE + 0x41 + i as u32, b);
        }
        let headers = DNAS_RELOC_IMAGE + 0x100;
        let sections = [
            (0, 0x41, 6),
            (2, DNAS_RELOC_WORDS - DNAS_RELOC_IMAGE, 0x100),
            (
                4,
                DNAS_RELOC_ENTRIES - DNAS_RELOC_IMAGE,
                entries.len() as u32 * REL_ENTRY_SIZE + extra,
            ),
        ];
        for (i, (name, offset, size)) in sections.into_iter().enumerate() {
            let h = headers + i as u32 * ELF_SECTION_SIZE as u32;
            c.bus.write32(h + ELF_SECTION_NAME, name);
            c.bus.write32(h + ELF_SECTION_FILE_OFFSET, offset);
            c.bus.write32(h + ELF_SECTION_BYTES, size);
        }
        for (i, &(kind, sym)) in entries.iter().enumerate() {
            let i = i as u32;
            // The low two bits of r_offset are dropped (word index).
            c.bus.write32(DNAS_RELOC_ENTRIES + i * 8, i * 4 + i % 4);
            c.bus
                .write32(DNAS_RELOC_ENTRIES + i * 8 + 4, sym << 8 | kind);
        }
        crate::difftest::fill(c, r, DNAS_RELOC_WORDS, 0x40);
        let name = if named { 3 } else { 1 };
        let symbols = [
            (1, 0x5f0, 0xff00),
            (name, 0x600 + inside * 4, 0xff00),
            (1, 0x640, 0xff00),
            (name, 0x10, 1),
            (1, 0x8000, ELF_SECTION_ABS as u16),
            (name, 0x7000, 0xff00),
        ];
        for (i, (name, value, section)) in symbols.into_iter().enumerate() {
            let sym = DNAS_RELOC_SYMTAB + i as u32 * 0x10;
            c.bus.write32(sym, name);
            c.bus.write32(sym + 4, value);
            c.bus.write16(sym + 0xe, section);
        }
        for (i, &b) in b"z\0zk\0".iter().enumerate() {
            c.bus.write8(DNAS_RELOC_STRTAB + i as u32, b);
        }
        c.bus.write32(DNAS_RELOC_REMAP, 0x40);
        c.bus
            .write32(DNAS_RELOC_REMAP + 4, DNAS_RELOC_IMAGE + 0x600);
        c.bus.write32(DNAS_RELOC_REMAP + 8, DNAS_RELOC_REMAP_WORDS);
        crate::difftest::fill(c, r, DNAS_RELOC_REMAP_WORDS, 0x10);
        // The frame: name buffer, waiting list (read stale by a LO16 with none waiting).
        crate::difftest::fill(c, r, crate::difftest::SP - 0x2b0, 0x6c);
    }

    /// Types in the order the entries come (rotated per seed): HI16 pairs and a triple
    /// before LO16s, a LO16 with nothing waiting, R_MIPS_32/26 and skipped types 0, 1, 3, 7, 0xff.
    pub(crate) const DNAS_RELOC_TYPE_CYCLE: [u32; 18] =
        [5, 2, 5, 6, 4, 6, 0, 5, 5, 5, 6, 7, 3, 2, 0xff, 1, 4, 6];

    /// `seed % 7`: 0 section missing, 1 relocation section missing, 2 no whole entry,
    /// 3 a full waiting list (34 HI16s, the last 3 dropped), else the type cycle rotated by
    /// `seed / 7 % 18`. Symbol `(i + seed / 126 % 6) % 6` for entry `i`, so every type meets
    /// every symbol; symbol 1's word in the remap window (0..16) and the extra bytes after the
    /// entries (0..8) are random data, not arm digits.
    /// `check`, and both bodies must make the same sequence of logged calls (`check` lets a
    /// different sequence pass unless `SSX3_DIFF_STRICT=1`; a callee reached through its
    /// `__generated` body instead of its slot shows up only there).
    pub(crate) fn dnas_rel_relocate_section_check(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        for seed in 0..crate::difftest::seed_count(seeds) {
            let r = crate::difftest::compare(generated, hand, &|c| setup(c, seed));
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            assert!(
                crate::difftest::strict_ok(&r.soft),
                "seed {seed} (scratch): {:#?}",
                r.soft
            );
            let calls: Vec<&String> = r.soft.iter().filter(|m| m.starts_with("calls")).collect();
            assert!(calls.is_empty(), "seed {seed} (call sequence): {calls:#?}");
        }
    }

    #[test]
    fn dnas_rel_relocate_section_matches() {
        dnas_rel_relocate_section_check(
            ee::s15::f_003fd2f8__generated,
            dnas_rel_relocate_section,
            7 * 18 * 6,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d2f8, 0);
                let mode = seed % 7;
                let sym = |i: u32| (i + seed / 126 % 6) % 6;
                let entries: Vec<(u32, u32)> = match mode {
                    2 => Vec::new(),
                    3 => (0..36)
                        .map(|i| {
                            (
                                if i < 34 {
                                    5
                                } else if i == 34 {
                                    6
                                } else {
                                    2
                                },
                                sym(i),
                            )
                        })
                        .collect(),
                    _ => (0..18)
                        .map(|i| {
                            (
                                DNAS_RELOC_TYPE_CYCLE[((i + seed / 7) % 18) as usize],
                                sym(i),
                            )
                        })
                        .collect(),
                };
                let extra = r.next_u32() % 8;
                let inside = r.next_u32() % 16;
                dnas_rel_relocate_section_fixture(
                    c,
                    &mut r,
                    &entries,
                    extra,
                    [mode != 0, mode != 1],
                    inside,
                    true,
                );
            },
        );
    }

    /// Events at the two back edges (0x3fd380 per entry, 0x3fd4e0 per waiting HI16): twelve
    /// HI16s then a LO16 and an R_MIPS_32, all on symbol 0 with empty names. The event is
    /// made due after both section lookups (~221 cycles, so never inside `strcmp`):
    /// `seed / 3 % 2 == 0` spreads it over the entries (outer head), `== 1` puts it in the
    /// LO16 entry (~1349..1573 cycles: inner head). Plain functions on both sides: the loops
    /// never unwind (`deliver_event_here`).
    #[test]
    fn dnas_rel_relocate_section_events() {
        use crate::difftest::events::{check_events, schedule, Events};
        check_events(
            ee::s15::f_003fd2f8__generated,
            dnas_rel_relocate_section,
            90,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d2f8, 0);
                let entries: Vec<(u32, u32)> = (0..14)
                    .map(|i| {
                        (
                            if i < 12 {
                                5
                            } else if i == 12 {
                                6
                            } else {
                                2
                            },
                            0,
                        )
                    })
                    .collect();
                dnas_rel_relocate_section_fixture(c, &mut r, &entries, 0, [true, true], 0, false);
                let mode = schedule(c, &mut r, seed, 0x003f_d2f8);
                if mode != Events::Off {
                    let step = seed / 6 % 13;
                    let due = if seed / 3 % 2 == 0 {
                        230 + step * 90
                    } else {
                        1380 + step * 14
                    };
                    c.next_event = c.cycles + due as u64;
                }
                mode
            },
        );
    }

    pub(crate) const DELETE_TEST_STATUS: u32 = DATA + 0x1000;

    pub(crate) const DELETE_TEST_REPLY: u32 = DATA + 0x1004;

    pub(crate) const DELETE_TEST_STATE: u32 = DATA + 0x2000;

    pub(crate) const DELETE_TEST_IFACE: u32 = DATA + 0x3000;

    /// The card error word `SLoginMc_GetError`/`SetError` use.
    pub(crate) const DELETE_TEST_ERROR: u32 = DELETE_TEST_STATE + 8;

    /// The card state (mode word, phase, error, device 0, direction 0, path
    /// and directory name), the interface (device mask halfword, fake
    /// callbacks, card name) and a non-zero completion word; the names are
    /// invented, with the lengths of the game's. The game image supplies the
    /// dispatch table and the file names.
    pub(crate) fn delete_step_card(c: &mut Cpu, mode: u32, phase: u32, error: u32, mask: u16) {
        elf_image(c);
        c.bus.write32(GAME_GP + card::STATE_GP, DELETE_TEST_STATE);
        c.bus.write32(GAME_GP + card::IFACE_GP, DELETE_TEST_IFACE);
        c.bus.write32(DELETE_TEST_STATE, mode);
        c.bus.write32(DELETE_TEST_STATE + card::PHASE, phase);
        c.bus.write32(DELETE_TEST_ERROR, error);
        c.bus.write32(DELETE_TEST_STATE + card::DEVICE, 0);
        c.bus.write32(DELETE_TEST_STATE + card::SCAN_DIRECTION, 0);
        c.bus.write16(DELETE_TEST_IFACE, mask);
        for (i, off) in [4u32, 0x10, 0x14, 0x24, 0x28, 0x44].into_iter().enumerate() {
            c.bus
                .write32(DELETE_TEST_IFACE + off, FAKE + 0x10 * i as u32);
        }
        for (i, b) in b"beta\0".iter().enumerate() {
            c.bus.write8(DELETE_TEST_IFACE + 0x50 + i as u32, *b);
        }
        for (i, b) in b"/alice/bob\0".iter().enumerate() {
            c.bus
                .write8(DELETE_TEST_STATE + card::FILE_PATH + i as u32, *b);
        }
        for (i, b) in b"demo\0".iter().enumerate() {
            c.bus
                .write8(DELETE_TEST_STATE + card::DIR_NAME + i as u32, *b);
        }
        c.bus.write32(DELETE_TEST_STATUS, 0xa5a5_a5a5);
    }

    /// `seed % 14` picks the case (a phase, or an argument/mode/table edge),
    /// `seed / 14 % 4` the RPC result (0, 3, 4, 7), `seed / 56 % 4` the card:
    /// (mode, error, mask, reply) = (2, 0, found, 1), (2, 5, found, 0),
    /// (2, 5, none, -1), (5, 0, none, 2).
    pub(crate) fn delete_step_fixture(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3fae_5000, 2);
        c.pc = DELETE_ENTRY;
        let case = seed % 14;
        let result = [0u64, 3, 4, 7][(seed / 14 % 4) as usize];
        let card = (seed / 56 % 4) as usize;
        let (mode, error, mask) = [(2, 0, 0xff), (2, 5, 0xff), (2, 5, 0), (5, 0, 0)][card];
        let reply = [1u32, 0, u32::MAX, 2][card];
        let phase = match case {
            0 => 1,
            1 => 2,
            2 => 18,
            3 => 19,
            4 => 20,
            5 => 21,
            6 | 13 => 26,
            // Table rows that point at the unknown-phase exit.
            7 => [3, 9, 17, 22][card],
            // Outside the table's 26 rows.
            8 => [0, 27, 28, 0x8000_0000][card],
            _ => 1,
        };
        // Case 11: a card mode other than 2 and 5.
        let mode = if case == 11 {
            [0, 1, 3, 11][card]
        } else {
            mode
        };
        delete_step_card(c, mode, phase, error, mask);
        c.bus.write32(DELETE_TEST_REPLY, reply);
        c.g[A0][0] = result;
        c.g[A1][0] = if case == 9 { 0 } else { sx(DELETE_TEST_STATUS) };
        c.g[A2][0] = if case == 10 { 0 } else { sx(DELETE_TEST_REPLY) };
        if case == 12 {
            // A phase-1 row outside the function leaves through jump_out.
            c.bus.write32(DELETE_DISPATCH, FAKE + 0x100);
        }
        if case == 13 {
            // Resumed at the retry head inside the frame (a1 = card state).
            c.pc = DELETE_RETRY_HEAD;
            let frame_sp = crate::difftest::SP - 0x80;
            c.g[SP][0] = sx(frame_sp);
            c.bus.write64(frame_sp + 0x50, crate::difftest::RA as u64);
            c.g[A1][0] = sx(DELETE_TEST_STATE);
        }
    }

    #[test]
    fn slogin_mc_delete_step_matches() {
        check(
            ee::s14::f_003fae50__generated,
            slogin_mc_delete_step,
            224,
            &delete_step_fixture,
        );
        // Cycle totals of the fake-free arms: argument checks (14), bad mode
        // (23), phase outside the table (30), unknown-phase row (36), phase
        // 26 bad result (38), phase 2 odd reply (43), phase 26 mode 5 (47),
        // phase 1 bad result (50), phase 26 done (51), phase 1 mode 5 (55),
        // phase 26 no further card (138), phase 1 no further card (142).
        require_paths(
            ee::s14::f_003fae50__generated,
            224,
            &[14, 23, 30, 36, 38, 43, 47, 50, 51, 55, 138, 142],
            &delete_step_fixture,
        );
    }

    // Both sides re-enter at the retry head after an unwind, as the trampoline does.
    crate::difftest::events::driven!(
        driven_generated_mc_delete_step,
        ee::s14::f_003fae50__generated
    );

    crate::difftest::events::driven!(driven_slogin_mc_delete_step, slogin_mc_delete_step_counted);

    /// Phase 26 in mode 2 with an error and another selected card: the back
    /// edge to the retry head 0x3faf60 is the first event check (the device
    /// scan finds card 1 without looping). The hand side itself must unwind
    /// at that head and be re-entered there, as the generated body is.
    #[test]
    fn slogin_mc_delete_step_events() {
        use crate::difftest::events::{check_events, schedule};
        mc_hand_resumes().with(|n| n.set(0));
        check_events(
            driven_generated_mc_delete_step,
            driven_slogin_mc_delete_step,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3fae_50ee, 0);
                delete_step_card(c, 2, 26, 5, 0xff);
                let mode = schedule(c, &mut r, seed, DELETE_ENTRY);
                c.g[A0][0] = 0;
                c.g[A1][0] = sx(DELETE_TEST_STATUS);
                c.g[A2][0] = sx(DELETE_TEST_REPLY);
                mode
            },
        );
        let resumed = mc_hand_resumes().with(|n| n.get());
        assert!(resumed > 0, "the hand side never resumed at the retry head");
    }

    pub(crate) const STORE_QUERY_TEST_STATE: u32 = DATA + 0xc00;

    pub(crate) const STORE_QUERY_TEST_RECORDS: u32 = DATA + 0xd00;

    pub(crate) const STORE_TEST_OUT: u32 = DATA + 0x80;

    /// Scrambled registers, the game's gp and `pc` at `entry`.
    pub(crate) fn slogin_store_query_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut r = Rng(entry as u64 ^ (seed as u64) << 24);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    /// An initialized store whose directory holds one record per `kinds`
    /// entry (size word 0x100 + 0x10 * index).
    pub(crate) fn slogin_store_query_directory(c: &mut Cpu, kinds: &[u32]) {
        c.bus
            .write32(GAME_GP + store::STATE_GP, STORE_QUERY_TEST_STATE);
        c.bus.write32(
            STORE_QUERY_TEST_STATE + store::RECORD_COUNT,
            kinds.len() as u32,
        );
        c.bus.write32(
            STORE_QUERY_TEST_STATE + store::RECORDS,
            STORE_QUERY_TEST_RECORDS,
        );
        for (i, kind) in kinds.iter().enumerate() {
            let rec = STORE_QUERY_TEST_RECORDS + i as u32 * store::RECORD_STRIDE;
            c.bus.write32(rec, *kind);
            c.bus.write32(rec + 4, 0x1111 * (i as u32 + 1));
            c.bus.write32(rec + RECORD_SIZE, 0x100 + 0x10 * i as u32);
            c.bus.write32(rec + 0xc, 0x2222 * (i as u32 + 1));
        }
    }

    pub(crate) const COUNT_RECORDS_KINDS: [u32; 6] = [5, 7, 5, 9, 5, 7];

    /// Arms: seed % 5 = 0 store not initialized (20 cycles), 1 null store
    /// (22), 2 null count (25), 3 empty directory (32), 4 records; kind
    /// seed / 5 % 3 and record count 1 + seed / 15 % 4 cross the loop's
    /// match and skip arms (one record: 39 skip, 42 match).
    #[test]
    fn slogin_store_count_records_matches() {
        check_paths(
            ee::s15::f_003fc850__generated,
            slogin_store_count_records,
            120,
            &[20, 22, 25, 32, 39, 42],
            &|c, seed| {
                let _r = slogin_store_query_fixture(c, seed, 0x003f_c850);
                let arm = seed % 5;
                let records = (if arm == 3 { 0 } else { 1 + seed / 15 % 4 }) as usize;
                slogin_store_query_directory(c, &COUNT_RECORDS_KINDS[..records]);
                if arm == 0 {
                    c.bus.write32(GAME_GP + store::STATE_GP, 0);
                }
                c.g[A0][0] = if arm == 1 { 0 } else { sx(DATA + 0x40) };
                c.g[A1][0] = [5, 7, 11][(seed / 5 % 3) as usize];
                c.g[A2][0] = if arm == 2 { 0 } else { sx(STORE_TEST_OUT) };
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
            },
        );
    }

    crate::difftest::events::driven!(
        driven_slogin_store_count_records,
        slogin_store_count_records
    );

    /// Event seeds (seed % 3) on the one loop head (0x3fc8b8), eight records,
    /// kind seed / 3 % 2.
    #[test]
    fn slogin_store_count_records_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc850__generated,
            driven_slogin_store_count_records,
            60,
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_c850);
                slogin_store_query_directory(c, &[5, 7, 5, 9, 5, 7, 7, 5]);
                c.g[A0][0] = sx(DATA + 0x40);
                c.g[A1][0] = [5, 7][(seed / 3 % 2) as usize];
                c.g[A2][0] = sx(STORE_TEST_OUT);
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
                schedule(c, &mut r, seed, 0x003f_c850)
            },
        );
    }

    /// Arms: seed % 5 = 0 store not initialized (26 cycles), 1 null store
    /// (28), 2 null size (31), 3 empty directory: not found (46), 4 three
    /// records; kind seed / 5 % 3 and occurrence seed / 15 % 3 cross found
    /// (first record: 54) and not found.
    #[test]
    fn slogin_store_get_record_size_matches() {
        check_paths(
            ee::s15::f_003fc8f8__generated,
            slogin_store_get_record_size,
            90,
            &[26, 28, 31, 46, 54],
            &|c, seed| {
                let _r = slogin_store_query_fixture(c, seed, 0x003f_c8f8);
                let arm = seed % 5;
                let kinds: &[u32] = if arm == 3 { &[] } else { &[5, 7, 5] };
                slogin_store_query_directory(c, kinds);
                if arm == 0 {
                    c.bus.write32(GAME_GP + store::STATE_GP, 0);
                }
                c.g[A0][0] = if arm == 1 { 0 } else { sx(DATA + 0x40) };
                c.g[A1][0] = [5, 7, 9][(seed / 5 % 3) as usize];
                c.g[A2][0] = (seed / 15 % 3) as u64;
                c.g[A3][0] = if arm == 2 { 0 } else { sx(STORE_TEST_OUT) };
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
            },
        );
    }

    pub(crate) const GET_INFO_OUT: u32 = DATA + 0x2000;

    /// The store state (0x70 random bytes) at `STORE_QUERY_TEST_STATE + state_skew`
    /// and a non-zero output buffer.
    pub(crate) fn slogin_store_get_info_fixture(c: &mut Cpu, r: &mut Rng, state_skew: u32) {
        c.bus.write32(
            GAME_GP + store::STATE_GP,
            STORE_QUERY_TEST_STATE + state_skew,
        );
        for i in 0..0x70 {
            c.bus
                .write8(STORE_QUERY_TEST_STATE + i, r.next_u32() as u8 | 1);
            c.bus.write8(GET_INFO_OUT + i, 0xc3);
        }
    }

    /// Arms: seed % 5 = 0 null output (4 cycles), 1 both aligned (43),
    /// 2 output misaligned by 1 + seed / 5 % 7, 3 state misaligned by
    /// 1 + seed / 5 % 7, 4 both misaligned by that same amount (their `|`
    /// is misaligned, their `^` is not) (2-4: 69).
    #[test]
    fn slogin_store_get_info_matches() {
        check_paths(
            ee::s15::f_003fc990__generated,
            slogin_store_get_info,
            105,
            &[4, 43, 69],
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_c990);
                let arm = seed % 5;
                let skew = 1 + seed / 5 % 7;
                slogin_store_get_info_fixture(
                    c,
                    &mut r,
                    if arm == 3 || arm == 4 { skew } else { 0 },
                );
                c.g[A0][0] = match arm {
                    0 => 0,
                    2 | 4 => sx(GET_INFO_OUT + skew),
                    _ => sx(GET_INFO_OUT),
                };
            },
        );
    }

    crate::difftest::events::driven!(driven_slogin_store_get_info, slogin_store_get_info);

    /// Event seeds (seed % 3) on both loop heads: seed / 3 % 2 = 0 aligned
    /// (0x3fca04), 1 misaligned (0x3fc9b0).
    #[test]
    fn slogin_store_get_info_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fc990__generated,
            driven_slogin_store_get_info,
            72,
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_c990);
                slogin_store_get_info_fixture(c, &mut r, 0);
                let skew = if seed / 3 % 2 == 0 {
                    0
                } else {
                    1 + seed / 6 % 7
                };
                c.g[A0][0] = sx(GET_INFO_OUT + skew);
                schedule(c, &mut r, seed, 0x003f_c990)
            },
        );
    }

    pub(crate) const SET_ICON_SYS_ICON: u32 = DATA + 0x1000;

    pub(crate) const SET_ICON_SYS_MC_IFACE: u32 = DATA + 0x4000;

    /// Real callees throughout. Arms: seed % 4 = 0 store not initialized
    /// (14 cycles), 1 null icon (card layer 0xc, mapped 0x70: 35), 2 card
    /// layer not initialized (2, mapped 0x66: 42), 3 icon accepted (0); the
    /// icon's alignment seed / 4 % 2 steers the card layer's copy.
    #[test]
    fn slogin_store_set_icon_sys_matches() {
        check_paths(
            ee::s15::f_003fcdc8__generated,
            slogin_store_set_icon_sys,
            48,
            &[14, 35, 42],
            &|c, seed| {
                let mut r = slogin_store_query_fixture(c, seed, 0x003f_cdc8);
                let arm = seed % 4;
                let icon = SET_ICON_SYS_ICON + (seed / 4 % 2) * 4;
                c.bus.write32(
                    GAME_GP + store::STATE_GP,
                    if arm == 0 { 0 } else { STORE_QUERY_TEST_STATE },
                );
                c.bus.write32(
                    GAME_GP + card::IFACE_GP,
                    if arm == 2 { 0 } else { SET_ICON_SYS_MC_IFACE },
                );
                c.g[A0][0] = if arm == 1 { 0 } else { sx(icon) };
                for i in 0..0x3e0 {
                    c.bus.write8(SET_ICON_SYS_ICON + i, r.next_u32() as u8);
                }
                for i in 0..0x430 {
                    c.bus.write8(SET_ICON_SYS_MC_IFACE + i, 0xa5);
                }
            },
        );
    }

    pub(crate) const STORE_BLOCK_TEST_BLOCK: u32 = DATA + 0x2000;

    pub(crate) const STORE_BLOCK_TEST_OUT: u32 = DATA + 0x80;

    /// Fake hooks, the game image (the alignment jump table at 0x497580 and
    /// the header pattern at 0x497550) and the store heap fakes' gate.
    pub(crate) fn store_block_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        slogin_store_hook_heap_fakes();
        let r = fakes::machine(c, seed, entry as u64, 1);
        c.pc = entry;
        elf_image(c);
        c.bus.write32(STORE_FAKE_CTRL, STORE_FAKE_MAGIC);
        r
    }

    /// Arms: seed % 4 = 0 null `out` (4), 1 allocation fails (6), 2 small
    /// payload, 3 payload 0xfffffff4 (the size check's overflow arm);
    /// format seed / 4 % 7 (table and out-of-range formats), the allocator
    /// unwinds when seed / 28 % 2 = 1, block misaligned by 3 when
    /// seed / 56 % 2 = 1.
    #[test]
    fn slogin_store_alloc_block_matches() {
        fakes::covered(
            ee::s15::f_003fce10__generated,
            slogin_store_alloc_block,
            112,
            &|c, seed| {
                let mut r = store_block_fixture(c, seed, 0x003f_ce10);
                let arm = seed % 4;
                let block = STORE_BLOCK_TEST_BLOCK + (seed / 56 % 2) * 3;
                c.g[A0][0] = if arm == 0 {
                    0
                } else {
                    sx(STORE_BLOCK_TEST_OUT)
                };
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = if arm == 3 {
                    0xffff_fff4
                } else {
                    (r.next_u32() % 0x80) as u64
                };
                c.g[A3][0] = (seed / 4 % 7) as u64;
                c.bus
                    .write32(STORE_FAKE_ALLOC_RESULTS, if arm == 1 { 0 } else { block });
                c.bus.write32(STORE_BLOCK_TEST_OUT, 0x5a5a_5a5a);
                for i in 0..0xa0 {
                    c.bus.write8(STORE_BLOCK_TEST_BLOCK + i, 0xc3);
                }
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 28 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    /// Arms: seed % 2 = 0 null payload (4), 1 freed (0); format
    /// seed / 2 % 7, the free unwinds when seed / 14 % 2 = 1.
    #[test]
    fn slogin_store_free_block_matches() {
        fakes::covered(
            ee::s15::f_003fcf00__generated,
            slogin_store_free_block,
            56,
            &|c, seed| {
                let mut r = store_block_fixture(c, seed, 0x003f_cf00);
                c.g[A0][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(STORE_BLOCK_TEST_BLOCK + 12)
                };
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = (r.next_u32() % 0x80) as u64;
                c.g[A3][0] = (seed / 2 % 7) as u64;
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 14 % 2 == 0 { u32::MAX } else { 0 },
                );
            },
        );
    }

    /// Arms: seed % 3 = 0 null `size` (10 cycles), 1 payload 0xfffffff4
    /// (the size check's overflow arm: 51), 2 payload seed / 3 (60).
    #[test]
    fn slogin_store_get_block_size_matches() {
        check_paths(
            ee::s15::f_003fcf98__generated,
            slogin_store_get_block_size,
            48,
            &[10, 51, 60],
            &|c, seed| {
                let _r = slogin_store_query_fixture(c, seed, 0x003f_cf98);
                elf_image(c);
                let arm = seed % 3;
                c.g[A0][0] = if arm == 1 {
                    0xffff_fff4
                } else {
                    (seed / 3) as u64
                };
                c.g[A1][0] = if arm == 0 { 0 } else { sx(STORE_TEST_OUT) };
                c.bus.write32(STORE_TEST_OUT, 0x5a5a_5a5a);
            },
        );
    }

    pub(crate) const DNAS_LOAD_IMAGE: u32 = DATA + 0x1000;

    /// The game image (`elf_image`: the section and symbol names the code
    /// compares against) and a relocatable MIPS ELF with three sections
    /// (section-name strings, `.symtab`, `.strtab`, the names copied from
    /// the game image) and four symbols in section 1; the first `present`
    /// carry the looked-up names (copied from the game image), the rest a
    /// decoy name. No `.text`/`.rel.*` sections, so relocation does nothing.
    /// `header`: 0 valid, 1 bad magic, 2 wrong type/machine word.
    pub(crate) fn dnas_load_image(c: &mut Cpu, present: usize, header: u32) {
        elf_image(c);
        let img = DNAS_LOAD_IMAGE;
        c.bus.write32(
            img,
            if header == 1 {
                ELF_MAGIC as u32 ^ 0x7f
            } else {
                ELF_MAGIC as u32
            },
        );
        c.bus.write32(
            img + 0x10,
            if header == 2 {
                ELF_RELAY_TYPE as u32 + 1
            } else {
                ELF_RELAY_TYPE as u32
            },
        );
        c.bus.write32(img + ELF_SECTION_OFFSET, 0x100);
        c.bus.write16(img + ELF_SECTION_COUNT, 3);
        c.bus.write16(img + ELF_SECTION_NAMES_INDEX, 0);
        dnas_put_cstr(c, img + 0x200, b"");
        let symtab_at = 1;
        let len = dnas_load_copy_cstr(c, DNAS_SYMTAB_NAME, img + 0x200 + symtab_at);
        let strtab_at = symtab_at + len + 1;
        dnas_load_copy_cstr(c, DNAS_STRTAB_NAME, img + 0x200 + strtab_at);
        let sections = img + 0x100;
        for (i, (name, offset, size)) in [
            (0, 0x200, 0x20),
            (symtab_at, 0x300, 0x40),
            (strtab_at, 0x400, 0x80),
        ]
        .iter()
        .enumerate()
        {
            let sh = sections + ELF_SECTION_SIZE as u32 * i as u32;
            c.bus.write32(sh + ELF_SECTION_NAME, *name);
            c.bus.write32(sh + ELF_SECTION_FILE_OFFSET, *offset);
            c.bus.write32(sh + ELF_SECTION_BYTES, *size);
        }
        dnas_put_cstr(c, img + 0x400, b"");
        dnas_put_cstr(c, img + 0x401, b"Xnone");
        let mut name_at = 7u32;
        for (i, &name) in DNAS_LOAD_SYMBOL_NAMES.iter().enumerate() {
            let len = dnas_load_copy_cstr(c, name, img + 0x400 + name_at);
            let sym = img + 0x300 + 0x10 * i as u32;
            c.bus.write32(sym, if i < present { name_at } else { 1 });
            c.bus.write32(sym + 4, 0x10 * (i as u32 + 1));
            c.bus.write32(sym + 8, 0);
            c.bus.write16(sym + 0xc, 0);
            c.bus.write16(sym + 0xe, 1);
            name_at += len + 1;
        }
    }

    /// Real ELF callees; the two `FlushCache` syscalls are faked. Arms:
    /// seed % 6 = 0 not a relay ELF (-1; bad magic or bad type by
    /// seed / 6 % 2), 1 no `_reloctable` (-2), 2 no init (-3), 3 no update
    /// (-4), 4 no exit (-5), 5 loaded (0); in arm 5, seed / 6 % 3 = 1/2
    /// unwinds at the first/second `FlushCache`.
    #[test]
    fn dnas_rel_load_matches() {
        fakes::covered(
            ee::s15::f_003fcfd8__generated,
            dnas_rel_load,
            72,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_cfd8, 2);
                c.pc = 0x003f_cfd8;
                let arm = seed % 6;
                let present = (arm as usize).saturating_sub(1);
                dnas_load_image(c, present, if arm == 0 { 1 + seed / 6 % 2 } else { 0 });
                c.g[A0][0] = sx(DNAS_LOAD_IMAGE);
                c.g[A1][0] = sx(DATA + 0x3000);
                for slot in [DNAS_INIT_SLOT, DNAS_UPDATE_SLOT, DNAS_EXIT_SLOT] {
                    c.bus.write32(DNAS_RELAY_BASE + slot, 0x1357_9bdf);
                }
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    match seed / 6 % 3 {
                        0 => u32::MAX,
                        n => n - 1,
                    },
                );
            },
        );
    }

    /// The relay's init entry is a fake (one of three addresses by
    /// seed % 3) that unwinds when seed / 3 % 3 = 0.
    #[test]
    fn dnas_rel_call_init_matches() {
        fakes::covered(
            ee::s15::f_003fd0d8__generated,
            dnas_rel_call_init,
            72,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x003f_d0d8, 1);
                c.pc = 0x003f_d0d8;
                c.bus.write32(
                    DNAS_RELAY_BASE + DNAS_INIT_SLOT,
                    fakes::FAKE + (seed % 3) * 0x10,
                );
                c.bus.write32(
                    fakes::LOG_UNWIND_AT,
                    if seed / 3 % 3 == 0 { 0 } else { u32::MAX },
                );
            },
        );
    }

    pub(crate) const STORE_POLL_TEST_STATUS: u32 = DATA + 0x1000;

    pub(crate) const STORE_POLL_TEST_DETAIL: u32 = DATA + 0x1010;

    pub(crate) const STORE_POLL_TEST_MC_IFACE: u32 = DATA + 0x3000;

    pub(crate) const STORE_POLL_TEST_MC_STATE: u32 = DATA + 0x3400;

    pub(crate) const STORE_POLL_TEST_FINISH_TABLE: u32 = DATA + 0x3800;

    pub(crate) const STORE_POLL_TEST_STEP_TABLE: u32 = DATA + 0x3900;

    pub(crate) const STORE_POLL_TEST_CIPHER: u32 = DATA + 0x3c00;

    /// What the card fakes report: status word, then the results of the
    /// card poll, the finish handler and the step handler.
    pub(crate) const STORE_POLL_TEST_CTRL: u32 = DATA + 0x3f00;

    pub(crate) const STORE_POLL_TEST_STORE: u32 = DATA + 0x4000;

    pub(crate) const STORE_POLL_TEST_RECORD: u32 = DATA + 0x5010;

    pub(crate) const STORE_POLL_TEST_DIR: u32 = DATA + 0x5410;

    pub(crate) const STORE_POLL_TEST_RECORDS: u32 = DATA + 0x6000;

    /// Fake addresses of the card interface's poll and handlers.
    pub(crate) const STORE_POLL_FAKE_CARD_POLL: u32 = FAKE + 0x40;

    pub(crate) const STORE_POLL_FAKE_STEP: u32 = FAKE + 0x48;

    pub(crate) const STORE_POLL_FAKE_FINISH: u32 = FAKE + 0x4c;

    /// Fixture cases; the next seed digit picks the fake call that unwinds.
    pub(crate) const STORE_POLL_CASES: u32 = 36;

    /// No unwind, or at fake call 0..3 (the card poll, its handlers, a later
    /// callee).
    pub(crate) const STORE_POLL_UNWIND_AT: [u32; 5] = [u32::MAX, 0, 1, 2, 3];

    /// The `miss` hook: a logged fake call; the card poll also writes the
    /// fixture's status word, and the poll and handlers return the fixture's
    /// results (0 included, which plain fakes cannot return).
    pub(crate) fn store_poll_card_fake(c: &mut Cpu, target: u32) {
        let status_at = c.g[A0][0] as u32;
        fakes::fake_call(c, target);
        let result_at = match target {
            STORE_POLL_FAKE_CARD_POLL => {
                let status = c.bus.read32(STORE_POLL_TEST_CTRL);
                c.bus.write32(status_at, status);
                STORE_POLL_TEST_CTRL + 4
            }
            STORE_POLL_FAKE_FINISH => STORE_POLL_TEST_CTRL + 8,
            STORE_POLL_FAKE_STEP => STORE_POLL_TEST_CTRL + 0xc,
            _ => return,
        };
        c.g[V0][0] = sx(c.bus.read32(result_at));
    }

    /// One fixture case: the card's answers and the store's state.
    pub(crate) struct StorePollCase {
        initialized: bool,
        status_null: bool,
        detail_null: bool,
        /// `detail` points at `status` (a size write then changes the status).
        detail_is_status: bool,
        card_status: u32,
        first: u32,
        finish: u32,
        step_result: u32,
        op: u32,
        step: u32,
        flag: u8,
        entry_total: u32,
        dir_hash: u32,
        cipher_fails: bool,
        size_matches: bool,
        record_size: u32,
    }

    pub(crate) fn store_poll_case(case: u32) -> StorePollCase {
        let mut k = StorePollCase {
            initialized: true,
            status_null: false,
            detail_null: false,
            detail_is_status: false,
            card_status: 2,
            first: 0x33,
            finish: 0x21,
            step_result: 0,
            op: 0,
            step: 0,
            flag: 0x5a,
            entry_total: 0,
            dir_hash: 0x1357_9bdf,
            cipher_fails: false,
            size_matches: true,
            record_size: 0x100,
        };
        match case {
            0 => k.initialized = false,
            1 => k.status_null = true,
            2 => k.detail_null = true,
            3 => {
                k.card_status = 0;
                k.op = 3;
            }
            4 => k.card_status = 0,
            5 => {
                k.card_status = 0;
                k.first = 0;
            }
            6 => k.card_status = 1,
            7 => {
                k.step_result = 0x40;
                k.op = 3;
            }
            8 => {
                k.step_result = 0x40;
                k.op = 4;
                k.step = 4;
            }
            9 => {
                k.step_result = 0x40;
                k.op = 5;
                k.step = 9;
            }
            10 => {
                k.step_result = 0x40;
                k.op = 4;
                k.step = 7;
            }
            11 => {
                k.step_result = 0x40;
                k.op = 5;
                k.step = 2;
            }
            12 => k.op = 1,
            13 => {
                k.op = 6;
                k.step = 5;
            }
            14 => {
                k.op = 6;
                k.step = 6;
                k.size_matches = false;
            }
            15 => {
                k.op = 6;
                k.step = 6;
                k.cipher_fails = true;
            }
            16 => {
                k.op = 6;
                k.step = 6;
            }
            17 => {
                k.op = 6;
                k.step = 6;
                k.detail_is_status = true;
                k.record_size = 0xffff_fff6;
            }
            18 => {
                k.op = 6;
                k.step = 7;
            }
            19 => {
                k.op = 8;
                k.step = 5;
            }
            20 => {
                k.op = 8;
                k.step = 10;
                k.size_matches = false;
            }
            21 => {
                k.op = 8;
                k.step = 10;
            }
            22 => {
                k.op = 8;
                k.step = 3;
            }
            23 => {
                k.op = 4;
                k.step = 4;
            }
            24 => {
                k.op = 4;
                k.step = 9;
            }
            25 => {
                k.op = 5;
                k.step = 9;
            }
            26 => {
                k.op = 4;
                k.step = 7;
                k.entry_total = 3;
            }
            27 => {
                k.op = 4;
                k.step = 7;
            }
            28 => {
                k.op = 5;
                k.step = 7;
                k.dir_hash = 0;
            }
            29 => {
                k.op = 5;
                k.step = 7;
            }
            30 => {
                k.op = 4;
                k.step = 7;
                k.finish = 0xa;
            }
            31 => {
                k.op = 5;
                k.step = 7;
                k.finish = 0xa;
            }
            32 => {
                k.op = 4;
                k.step = 2;
            }
            33 => k.op = 0,
            34 => k.op = 7,
            35 => {
                k.op = 4;
                k.step = 4;
                k.finish = 0x10;
            }
            _ => unreachable!(),
        }
        k
    }

    /// `(v0, *status)` the generated body returns for a case with no unwind
    /// (`None`: the result comes from a callee path not pinned here).
    pub(crate) fn store_poll_expected(case: u32) -> Option<(u64, u32)> {
        let untouched = 0x5555;
        Some(match case {
            0 | 1 | 2 => (if case == 0 { 2 } else { 4 }, untouched),
            3 => (8, 0),
            4 => (0x97, 0),
            5 => (0, 0),
            6 => (0x97, 1),
            7..=11 => (0xa4, 2),
            12 => (0, 2),
            13 => (4, 1),
            14 | 20 => (0x6e, 2),
            15 | 26 => return None,
            16 | 18 | 22 | 24 | 27 | 28 | 30 | 33 | 34 => (0, 2),
            17 => (0, 0xffff_fff6),
            19 | 23 | 25 | 32 => (0, 1),
            21 => (0xb, 2),
            29 | 31 => (9, 2),
            35 => (0x74, 2),
            _ => unreachable!(),
        })
    }

    pub(crate) fn store_poll_fixture(c: &mut Cpu, seed: u32) {
        let case = seed % STORE_POLL_CASES;
        let k = store_poll_case(case);
        let _r = machine(c, seed, 0x003f_ca40, 4);
        elf_image(c);
        c.hooks.miss = store_poll_card_fake;
        c.bus.write32(
            LOG_UNWIND_AT,
            STORE_POLL_UNWIND_AT[(seed / STORE_POLL_CASES % 5) as usize],
        );
        c.pc = 0x003f_ca40;

        // Arguments and output words (pre-filled so every store shows).
        c.g[A0][0] = if k.status_null {
            0
        } else {
            sx(STORE_POLL_TEST_STATUS)
        };
        c.g[A1][0] = if k.detail_null {
            0
        } else if k.detail_is_status {
            sx(STORE_POLL_TEST_STATUS)
        } else {
            sx(STORE_POLL_TEST_DETAIL)
        };
        c.bus.write32(STORE_POLL_TEST_STATUS, 0x5555);
        let size_check = k.record_size.wrapping_add(0xc);
        c.bus.write32(
            STORE_POLL_TEST_DETAIL,
            if k.size_matches { size_check } else { 0x77 },
        );
        c.bus.write32(STORE_POLL_TEST_CTRL, k.card_status);
        c.bus.write32(STORE_POLL_TEST_CTRL + 4, k.first);
        c.bus.write32(STORE_POLL_TEST_CTRL + 8, k.finish);
        c.bus.write32(STORE_POLL_TEST_CTRL + 0xc, k.step_result);

        // The memory-card layer: idle state, an interface of fakes.
        c.bus
            .write32(GAME_GP + card::STATE_GP, STORE_POLL_TEST_MC_STATE);
        c.bus
            .write32(GAME_GP + card::IFACE_GP, STORE_POLL_TEST_MC_IFACE);
        for i in 0..0x20 {
            c.bus
                .write32(STORE_POLL_TEST_MC_STATE + 4 * i, 0x6100_0000 + i);
        }
        c.bus.write32(STORE_POLL_TEST_MC_STATE, 0);
        c.bus.write32(STORE_POLL_TEST_MC_STATE + 4, 0);
        for off in [0x2c, 0x30, 0x34] {
            c.bus.write32(STORE_POLL_TEST_MC_IFACE + off, FAKE + off);
        }
        c.bus
            .write32(STORE_POLL_TEST_MC_IFACE + 0x40, STORE_POLL_FAKE_CARD_POLL);
        c.bus
            .write32(STORE_POLL_TEST_MC_IFACE + 0x48, STORE_POLL_TEST_STEP_TABLE);
        c.bus.write32(
            STORE_POLL_TEST_MC_IFACE + 0x4c,
            STORE_POLL_TEST_FINISH_TABLE,
        );
        c.bus
            .write32(STORE_POLL_TEST_STEP_TABLE, STORE_POLL_FAKE_STEP);
        c.bus
            .write32(STORE_POLL_TEST_FINISH_TABLE, STORE_POLL_FAKE_FINISH);

        // The login store.
        let store = STORE_POLL_TEST_STORE;
        c.bus.write32(
            GAME_GP + store::STATE_GP,
            if k.initialized { store } else { 0 },
        );
        for i in 0..0x30 {
            c.bus.write32(store + 4 * i, 0x5300_0000 + i);
        }
        c.bus.write32(store + store::DIR_HASH, k.dir_hash);
        c.bus.write32(
            store + store::CIPHER_IFACE,
            if k.cipher_fails {
                STORE_POLL_TEST_CIPHER
            } else {
                0
            },
        );
        for off in [CIPHER_SET_KEY, CIPHER_DECRYPT, CIPHER_RESET] {
            c.bus
                .write32(STORE_POLL_TEST_CIPHER + off, FAKE + 0x80 + off);
        }
        c.bus.write32(store + store::OPERATION, k.op);
        c.bus.write32(store + store::STEP, k.step);
        c.bus
            .write32(store + store::RECORD_BUFFER, STORE_POLL_TEST_RECORD);
        c.bus
            .write32(store + store::DIR_BUFFER, STORE_POLL_TEST_DIR);
        c.bus.write32(store + store::RECORD_SIZE, k.record_size);
        c.bus.write32(store + store::RECORD_KIND, 0x11);
        c.bus.write32(store + store::RECORD_CAPACITY, k.entry_total);
        c.bus.write32(store + store::RECORD_COUNT, 0);
        c.bus
            .write32(store + store::RECORDS, STORE_POLL_TEST_RECORDS);
        c.bus.write8(store + store::CREATED, k.flag);
        // Record and directory blocks with headers that are not the
        // sentinel, so header writes and the header check show.
        for i in 0..0x40 {
            c.bus
                .write8(STORE_POLL_TEST_RECORD - 0xc + i, (0xa0 + i) as u8);
            c.bus
                .write8(STORE_POLL_TEST_DIR - 0xc + i, (0x30 + i) as u8);
        }
        for i in 0..8 {
            c.bus
                .write32(STORE_POLL_TEST_RECORDS + 4 * i, 0x5200_0000 + i);
        }
    }

    /// `SLoginStore_Poll`: every operation/step arm, card results 0, 10, 16
    /// and others, size match and mismatch, cipher failure, flagged and
    /// unflagged step 9, matching and differing directory hashes, and
    /// unwinds in the card poll, its handlers and a later callee.
    #[test]
    fn slogin_store_poll_matches() {
        check(
            ee::s15::f_003fca40__generated,
            slogin_store_poll,
            STORE_POLL_CASES * 5,
            &store_poll_fixture,
        );
        // The cases reach their arms: the generated body's results.
        for case in 0..STORE_POLL_CASES {
            let mut c = fresh();
            store_poll_fixture(&mut c, case);
            run_generated(&mut c, ee::s15::f_003fca40__generated);
            assert_eq!(c.pc, STORE_POLL_TEST_RETURN, "case {case}: returned");
            if let Some((v0, status)) = store_poll_expected(case) {
                assert_eq!(c.g[V0][0], v0, "case {case}: v0");
                assert_eq!(
                    c.bus.read32(STORE_POLL_TEST_STATUS),
                    status,
                    "case {case}: status"
                );
            }
        }
    }

    /// Event seeds (seed % 3) on the loop head 0x3fd260 (a structured loop:
    /// both sides deliver in place). 6 + seed / 3 % 6 sections whose names
    /// are all empty and odd-aligned, against the target "q": the real
    /// `strcmp` returns at its first byte test (before its own event check)
    /// and every section is scanned, so the events land on this function's
    /// head.
    #[test]
    fn dnas_rel_find_section_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fd210__generated,
            dnas_rel_find_section,
            36,
            &|c, seed| {
                let mut r = Rng(0x003f_d2ee ^ seed as u64);
                scramble(c, &mut r);
                let image = DATA + 0x1000;
                let sections = 6 + seed / 3 % 6;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x501);
                c.bus.write32(DATA, image);
                c.bus.write32(image + ELF_SECTION_OFFSET, 0x100);
                c.bus.write16(image + ELF_SECTION_COUNT, sections as u16);
                c.bus.write16(image + ELF_SECTION_NAMES_INDEX, 0);
                for i in 0..sections {
                    let h = image + 0x100 + i * ELF_SECTION_SIZE as u32;
                    c.bus.write32(h + ELF_SECTION_NAME, 1 + 2 * i);
                    c.bus.write32(h + ELF_SECTION_FILE_OFFSET, 0x300);
                    c.bus.write8(image + 0x300 + 2 * i, 0x78);
                    c.bus.write8(image + 0x301 + 2 * i, 0);
                }
                c.bus.write8(DATA + 0x501, b'q');
                c.bus.write8(DATA + 0x502, 0);
                schedule(c, &mut r, seed, 0x003f_d210)
            },
        );
    }

    /// Where `DnasRel_Load` finds the four symbol names it looks up (in the
    /// game image), in lookup order: the relocation table, then the init,
    /// update and exit entry points.
    pub(crate) const DNAS_LOAD_SYMBOL_NAMES: [u32; 4] =
        [0x0049_7598, 0x0049_75a8, 0x0049_75c0, 0x0049_75d8];

    /// Copies the NUL-terminated string at `from` (the loaded game image) to
    /// `to`; returns its length.
    pub(crate) fn dnas_load_copy_cstr(c: &mut Cpu, from: u32, to: u32) -> u32 {
        let mut n = 0;
        loop {
            let b = c.bus.read8(from + n);
            c.bus.write8(to + n, b);
            if b == 0 {
                return n;
            }
            n += 1;
        }
    }

    /// Event seeds (seed % 3) on the loop head 0x3fd190 (a structured loop:
    /// both sides deliver in place). No fake unwinds. The user update is a
    /// fake or the real `DnasRel_IsElf` (seed / 3 % 2; its buffer holds the
    /// ELF magic or 0 by seed / 12 % 2); the relay update returns 1 (repeat)
    /// for 2 or 3 trips with the fake user update, 3 or 6 with the real one
    /// (seed / 6 % 2), then 2 (or an unforced pointer), so every seed takes
    /// at least two back edges before the exit call.
    #[test]
    fn dnas_rel_run_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fd150__generated,
            dnas_rel_run,
            48,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x003f_d15e, 0);
                let user_fake = seed / 3 % 2 == 0;
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = if user_fake {
                    fakes::FAKE + 0x40
                } else {
                    callee::DNAS_REL_IS_ELF.addr
                } as u64;
                c.g[A3][0] = sx(DATA);
                c.bus.write32(
                    DATA,
                    if seed / 12 % 2 == 0 {
                        ELF_MAGIC as u32
                    } else {
                        0
                    },
                );
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_INIT_SLOT, fakes::FAKE + 0x10);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_UPDATE_SLOT, fakes::FAKE + 0x20);
                c.bus
                    .write32(DNAS_RELAY_BASE + DNAS_EXIT_SLOT, fakes::FAKE + 0x30);
                // Fake call indices: 0 init, then per trip (user,) relay.
                let (stride, repeats) = if user_fake {
                    (2, [2, 3][(seed / 6 % 2) as usize])
                } else {
                    (1, [3, 6][(seed / 6 % 2) as usize])
                };
                for trip in 0..=repeats {
                    let n = stride * (trip + 1);
                    if n < 8 {
                        c.bus.write32(
                            fakes::LOG_RESULTS + 4 * n,
                            if trip < repeats { 1 } else { 2 },
                        );
                    }
                }
                schedule(c, &mut r, seed, 0x003f_d150)
            },
        );
    }

    /// Event seeds (seed % 3) on the loop head 0x3fba48 (a structured loop:
    /// both sides deliver in place). Sixteen records (kinds 5, 7, 5, 9
    /// repeating); kind seed / 3 % 3 (5, 9, or the 64-bit miss
    /// 0x1_0000_0005) and occurrence 2 + seed / 9 % 4, so the scan makes 4
    /// to 16 trips (found at record 4..15, or not found).
    #[test]
    fn slogin_store_find_record_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s15::f_003fba20__generated,
            slogin_store_find_record,
            72,
            &|c, seed| {
                let mut r = Rng(0x3fba_20ee ^ seed as u64);
                scramble(c, &mut r);
                let kinds: Vec<u32> = (0..16).map(|i| [5, 7, 5, 9][i % 4]).collect();
                slogin_store_find_record_directory(c, &mut r, &kinds);
                c.g[A0][0] = [5u64, 9, 0x1_0000_0005][(seed / 3 % 3) as usize];
                c.g[A1][0] = (2 + seed / 9 % 4) as u64;
                schedule(c, &mut r, seed, 0x003f_ba20)
            },
        );
    }

    /// The store state at `gp+0x2a30` (random words, gp set) with a
    /// directory of `kinds.len()` records whose offset/size/CRC words are
    /// random.
    pub(crate) fn slogin_store_find_record_directory(c: &mut Cpu, r: &mut Rng, kinds: &[u32]) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + store::STATE_GP, STORE_TEST_STATE);
        fill(c, r, STORE_TEST_STATE, 0x28);
        c.bus
            .write32(STORE_TEST_STATE + store::RECORD_COUNT, kinds.len() as u32);
        c.bus
            .write32(STORE_TEST_STATE + store::RECORDS, STORE_TEST_RECORDS);
        fill(c, r, STORE_TEST_RECORDS, 4 * (kinds.len() as u32 + 1));
        for (i, kind) in kinds.iter().enumerate() {
            c.bus
                .write32(STORE_TEST_RECORDS + i as u32 * store::RECORD_STRIDE, *kind);
        }
    }

    /// Event delivery at the settings-copy loop heads (structured loops:
    /// `deliver_event_here`, so plain functions on both sides). Mode = seed
    /// % 3, source alignment seed / 3 % 2 (unaligned head 0x3f6cc8, aligned
    /// head 0x3f6d1c). The copy runs two trips and checks events once, after
    /// the first: the event is due 160 + seed / 6 % 12 cycles after entry,
    /// past the last check in the 0x14c-byte state clear (entry 22 + fake
    /// alloc at most 10 + memset check at 127 = 159) and no later than the
    /// loop check (at least 22 + 3 + 136 + 11 = 172).
    #[test]
    fn slogin_alloc_state_events_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        hook_slogin_alloc();
        check_events(
            ee::s14::f_003f6c70__generated,
            slogin_alloc_state,
            72,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f6c_70ee, 0);
                let config = DATA + 0x300;
                let source = DATA + 0x1000 + seed / 3 % 2;
                c.bus.write32(config, source);
                c.bus.write32(config + 4, DATA + 0x4000);
                for index in 0..0x44 {
                    c.bus.write8(source + index, (0x80 | (index ^ seed)) as u8);
                }
                c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
                c.bus.write32(ALLOC_CTRL + 4, 1);
                c.bus.write32(LOG_RESULTS, DATA + 0x2000);
                c.g[A0][0] = sx(config);
                let mode = schedule(c, &mut r, seed, 0x003f_6c70);
                if mode != Events::Off {
                    c.next_event = c.cycles + 160 + (seed / 6 % 12) as u64;
                }
                mode
            },
        );
    }

    /// `login_store_fixture` on the game's image (the store-error table at
    /// 0x497150 and the block header pattern are the game's) with an
    /// initialised, empty store state at DATA + 0x4000 and the allocation
    /// pinned to a non-zero-filled block at DATA + 0x6000. seed % 4 == 0
    /// fails the allocation (error 6 -> 0x0b, the error return); other seeds
    /// allocate (0 -> 0), read the record (not found: 7 -> 5) and map again,
    /// unless the harness unwinds at the allocation (seed % 6 == 0).
    pub(crate) fn step_read_record_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        elf_image(c);
        login_store_fixture(c, seed, entry);
        c.bus.write32(GAME_GP + 0x2a30, DATA + 0x4000);
        c.bus.write32(LOG_RESULTS, DATA + 0x6000);
        for i in 0..0x10u32 {
            c.bus
                .write32(DATA + 0x6000 + 4 * i, 0x3c00_0000 | i << 8 | seed);
        }
    }

    /// A login at DATA + 0x200 with its store state at DATA + 0x800 and no
    /// card manager, on the game's image (alignment table
    /// 0x497580, store-error table 0x497150 and block header pattern are the
    /// game's): the account and password fields (login + 0xa8, 34 bytes) hold
    /// invented non-zero bytes and the allocation (DATA + 0x2000) starts
    /// non-zero, so the block clear and the 17-byte copy both show. seed % 2
    /// == 1 allocates (copy, write, map), 0 fails it (error 6 -> 0x0b).
    pub(crate) fn write_record_step_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        elf_image(c);
        let _r = machine(c, seed, entry as u64, 4);
        c.pc = entry;
        c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
        c.bus.write32(DATA + 0x200 + 0x140, DATA + 0x500);
        c.bus.write32(GAME_GP + 0x2a30, DATA + 0x800);
        c.bus.write32(GAME_GP + 0x1aec, 0);
        for i in 0..34u32 {
            c.bus
                .write8(DATA + 0x200 + 0xa8 + i, (0x61 + i + seed) as u8);
        }
        for i in 0..0x20u32 {
            c.bus
                .write32(DATA + 0x2000 + 4 * i, 0x2d00_0000 | i << 8 | seed);
        }
        c.bus.write32(ALLOC_CTRL, ALLOC_MAGIC);
        c.bus.write32(ALLOC_CTRL + 4, seed % 2);
        c.bus.write32(LOG_RESULTS, DATA + 0x2000);
    }

    /// The `create_step_fixture` scenarios that make no callback, so their
    /// cycle totals are fixed: argument checks (0, 1: 14), bad mode (2: 23),
    /// phase outside the table (3, 53: 30), phase 1 input error (5: 40), phase 1
    /// retry in mode 4 (6: 46) and with no further card (7: 133), bad results
    /// (11: 43; 13, 17: 42), phase 26 input error (46: 38), in mode 4 (47: 47),
    /// done (48: 51) and with no further card (49: 138), and the unknown-phase
    /// rows (51, 54..61: 36).
    pub(crate) const CREATE_STEP_FIXED_SCENARIOS: [u32; 24] = [
        0, 1, 2, 3, 5, 6, 7, 11, 13, 17, 46, 47, 48, 49, 51, 53, 54, 55, 56, 57, 58, 59, 60, 61,
    ];

    /// Scrambled registers, the game's gp and card state. `seed % 3` picks the
    /// direction (forward, backward, invalid), `seed / 3 % 10` the current device
    /// (-1..8) and `seed / 30 % 10` the device mask (none, all, or one of the
    /// eight device bits), so 300 seeds cross all three. Bits above the low
    /// halfword are noise the function drops.
    pub(crate) fn slogin_mc_next_device_in_mask_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x3f85_c000 ^ seed as u64);
        scramble(c, &mut r);
        c.pc = 0x003f_85c0;
        c.g[GP][0] = GAME_GP as u64;
        let state = DATA + 0x400;
        c.bus.write32(GAME_GP + card::STATE_GP, state);
        let current = (seed / 3 % 10) as i16 - 1;
        c.bus.write16(state + 0x10, current as u16);
        c.bus.write16(state + 0x12, 0x7777);
        let mask: u64 = match seed / 30 % 10 {
            0 => 0,
            1 => 0xff,
            k => 1 << (k - 2),
        };
        c.g[A0][0] = mask | (r.next_u32() & 0xff00) as u64 | (r.next_u32() as u64) << 16;
        c.g[A1][0] = match seed % 3 {
            2 if seed / 30 % 2 == 1 => 1 << 32 | 1,
            d => d as u64,
        };
    }

    /// `seed % 7`: no icon, card not initialized, icon and card copy both
    /// 8-byte aligned, icon 4 bytes off, card copy 4 bytes off, both off, and an
    /// icon pointer whose low word is zero (the null check is 64-bit; the copy
    /// then reads RAM from 0). Both payload sizes the save-size callee rounds
    /// end partway into a KiB; the card's copy area starts non-zero.
    pub(crate) fn slogin_mc_set_icon_sys_fixture(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x003f_a240, 0);
        c.pc = 0x003f_a240;
        let class = seed % 7;
        let icon = DATA + 0x1000 + if matches!(class, 3 | 5) { 4 } else { 0 };
        let iface = DATA + 0x2000 + if matches!(class, 4 | 5) { 4 } else { 0 };
        c.g[A0][0] = match class {
            0 => 0,
            6 => 1 << 40,
            _ => sx(icon),
        };
        c.bus
            .write32(GAME_GP + card::IFACE_GP, if class == 1 { 0 } else { iface });
        fill(c, &mut r, DATA + 0x1000, 0x100);
        c.bus.write32(icon + CALC_SAVE_SIZE_FIRST, 0x0001_2345);
        c.bus.write32(icon + CALC_SAVE_SIZE_SECOND, 0x0000_0801);
        for i in 0..0x110u32 {
            c.bus.write32(DATA + 0x2000 + 4 * i, 0xa5a5_a5a5);
        }
    }

    /// Arms by cycle total: no icon (11), not initialized (18), the aligned
    /// copy (395; 393 for the icon at RAM 0, whose sizes are whole KiB) and
    /// the misaligned copy (637), each loop running its 30 trips.
    #[test]
    fn slogin_mc_set_icon_sys_matches() {
        check_paths(
            ee::s14::f_003fa240__generated,
            slogin_mc_set_icon_sys,
            56,
            &[11, 18, 393, 395, 637],
            &slogin_mc_set_icon_sys_fixture,
        );
    }

    /// The gate of `store_shutdown_free_block_fake` (its own word, so the
    /// other store tests, which set `STORE_FAKE_CTRL`, keep the real
    /// `SLoginStore_FreeBlock`).
    pub(crate) const STORE_SHUTDOWN_FREE_CTRL: u32 = STORE_FAKE_CTRL + 0x80;

    pub(crate) const STORE_SHUTDOWN_FREE_MAGIC: u32 = 0x5346_4246;

    /// The error the gated fake returns (sign-extended into v0).
    pub(crate) const STORE_SHUTDOWN_FREE_ERROR: u32 = STORE_SHUTDOWN_FREE_CTRL + 4;

    pub(crate) static PREV_STORE_SHUTDOWN_FREE_BLOCK: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake calls that came back with the error (the Shutdown
    /// FreeBlock-failed arm was taken).
    pub(crate) static STORE_SHUTDOWN_FREE_FAILS: AtomicUsize = AtomicUsize::new(0);

    /// `SLoginStore_FreeBlock` for the Shutdown test: with the gate set,
    /// logged like any fake and returning the error at
    /// `STORE_SHUTDOWN_FREE_ERROR`; otherwise the previous body.
    pub(crate) fn store_shutdown_free_block_fake(c: &mut Cpu) {
        if c.bus.read32(STORE_SHUTDOWN_FREE_CTRL) != STORE_SHUTDOWN_FREE_MAGIC {
            return fakes::fall_back(
                c,
                &PREV_STORE_SHUTDOWN_FREE_BLOCK,
                ee::s15::f_003fcf00__generated,
            );
        }
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, 0x003f_cf00, ret);
        c.g[V0][0] = sx(c.bus.read32(STORE_SHUTDOWN_FREE_ERROR));
        if c.pc == ret {
            STORE_SHUTDOWN_FREE_FAILS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    /// The miss hook of the open-step tests: the shared fake at a fixed cost
    /// of 3 cycles (the shared fake adds 3 to 10 by a hash), so every path
    /// through a card interface method has a fixed total for `require_paths`.
    pub(crate) fn mc_open_step_fake(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        c.cycles = before + 3;
    }

    /// One seed of the open-step cross (864 seeds), one independent digit per
    /// steering value: phase `[0, 1, 2, 0x16, 0x1a, 0xff][seed % 6]`, card
    /// state `[3, 6, 0, 4][seed / 6 % 4]`, a0 (the card result)
    /// `[0, 3, 4][seed / 24 % 3]`, `*a2` (the card reply)
    /// `[0, 1, -1][seed / 72 % 3]`, card error `[0, 4][seed / 216 % 2]`,
    /// device mask `[0xff, 0][seed / 432 % 2]`. The current device
    /// (state+0x10) and the search direction (state+0x6c) are 0, so
    /// NextDeviceInMask (real) finds device 1 (mask 0xff) or none (mask 0).
    /// The status word and every other state word start non-zero; the card
    /// interface's four methods are distinct fake addresses, so a wrong slot
    /// shows in the fake log.
    pub(crate) fn mc_open_step_fixture(c: &mut Cpu, seed: u32) {
        let _r = fakes::machine(c, seed, 0x3f8c_3800, 2);
        c.pc = 0x003f_8c38;
        c.hooks.miss = mc_open_step_fake;
        let phase = [0u32, 1, 2, 0x16, 0x1a, 0xff][(seed % 6) as usize];
        let state = [3u32, 6, 0, 4][(seed / 6 % 4) as usize];
        let result = [0u64, 3, 4][(seed / 24 % 3) as usize];
        let reply = [0u32, 1, u32::MAX][(seed / 72 % 3) as usize];
        let error = [0u32, 4][(seed / 216 % 2) as usize];
        let mask: u16 = if seed / 432 % 2 == 0 { 0xff } else { 0 };
        c.bus.write32(GAME_GP + card::STATE_GP, DATA + 0x400);
        c.bus.write32(GAME_GP + card::IFACE_GP, DATA + 0x800);
        c.bus.write32(DATA + 0x1000, 0xa5a5_a5a5);
        for i in 0..0x20u32 {
            c.bus.write32(DATA + 0x400 + 4 * i, 0x5a5a_5a00 | (i + 1));
        }
        c.bus.write32(DATA + 0x400, state);
        c.bus.write32(DATA + 0x404, phase);
        c.bus.write32(DATA + 0x410, 0);
        c.bus.write32(DATA + 0x400 + card::ERROR, error);
        c.bus.write32(DATA + 0x400 + 0x6c, 0);
        for (i, b) in b"demo\0".iter().enumerate() {
            c.bus.write8(DATA + 0x400 + 0x54 + i as u32, *b);
        }
        c.bus.write16(DATA + 0x800, mask);
        for (k, off) in [4u32, 0x10, 0x14, 0x18].iter().enumerate() {
            c.bus.write32(DATA + 0x800 + *off, FAKE + 0x10 * k as u32);
        }
        for (i, b) in b"beta\0".iter().enumerate() {
            c.bus.write8(DATA + 0x800 + 0x50 + i as u32, *b);
        }
        c.g[A0][0] = result;
        c.g[A1][0] = sx(DATA + 0x1000);
        c.g[A2][0] = sx(DATA + 0x1100);
        c.bus.write32(DATA + 0x1100, reply);
    }

    /// The `mc_open_step_fixture` seed with phase row `phase` and a0 row
    /// `result` fixed; `i` in 0..48 runs over the state, reply, error and
    /// mask rows.
    pub(crate) fn mc_open_step_slice(phase: u32, result: u32, i: u32) -> u32 {
        phase + 6 * (i % 4 + 4 * (result + 3 * (i / 4)))
    }

    /// Word 2 of the `ALLOC_CTRL` block: the gate of the SLogin_* pass-through
    /// fakes (`SLOGIN_PASS_MAGIC` turns them on; `slogin_unwind_at` sets it).
    pub(crate) const SLOGIN_PASS_CTRL: u32 = ALLOC_CTRL + 8;

    pub(crate) const SLOGIN_PASS_MAGIC: u32 = 0x736c_7074;

    /// Word 3 of the `ALLOC_CTRL` block: the call target whose first call
    /// unwinds while the pass-through gate is on (0: none).
    pub(crate) const SLOGIN_UNWIND_TARGET: u32 = ALLOC_CTRL + 0xc;

    /// Turn the pass-through fakes on for this machine and make the first
    /// call to `target` (a real callee's address or a fake's, 0 for none) the
    /// only fake call that unwinds.
    pub(crate) fn slogin_unwind_at(c: &mut Cpu, target: u32) {
        hook_slogin_passes();
        c.bus.write32(SLOGIN_PASS_CTRL, SLOGIN_PASS_MAGIC);
        c.bus.write32(SLOGIN_UNWIND_TARGET, target);
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
    }

    /// Called by a fake just before it logs a call to `target`: when that is
    /// the setup's unwind target, the coming fake call is the one that unwinds.
    pub(crate) fn slogin_arm_unwind(c: &mut Cpu, target: u32) {
        if slogin_pass_on(c, false) && c.bus.read32(SLOGIN_UNWIND_TARGET) == target {
            let next = c.bus.read32(fakes::LOG_COUNT);
            c.bus.write32(LOG_UNWIND_AT, next);
        }
    }

    /// A gated pass-through on a real callee's override slot. The call log
    /// records `(callee, a0-a3)` only, so a wrong return address to a real
    /// callee never shows there. With the gate on, the call is logged as a
    /// fake call (every register, `ra` included, lands in the RAM log both
    /// sides are compared on) and counts as one, so it unwinds there when
    /// armed (`slogin_arm_unwind`) or at the setup's `LOG_UNWIND_AT` index
    /// (the `?` after the call is taken); otherwise the body the slot held
    /// before runs with the registers and cycles it was called with. With the
    /// gate off it only runs that body.
    pub(crate) fn slogin_pass_through(
        c: &mut Cpu,
        fake: fn(&mut Cpu),
        target: u32,
        generated: fn(&mut Cpu),
        card_layer: bool,
    ) {
        let previous = fakes::prev_cell(fake);
        if !slogin_pass_on(c, card_layer) {
            return fakes::fall_back(c, previous, generated);
        }
        let (g, f, cycles) = (c.g, c.f, c.cycles);
        let ret = c.g[RA][0] as u32;
        slogin_arm_unwind(c, target);
        fakes::fake(c, target, ret);
        if c.pc != ret {
            slogin_pass_unwound().with(|u| u.borrow_mut().insert(target));
            return;
        }
        (c.g, c.f, c.cycles) = (g, f, cycles);
        c.pc = target;
        fakes::fall_back(c, previous, generated)
    }

    pub(crate) fn slogin_map_store_error_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_map_store_error_pass,
            callee::SLOGIN_MAP_STORE_ERROR.addr,
            ee::s14::f_003f8178__generated,
            false,
        );
    }

    pub(crate) fn slogin_store_poll_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_store_poll_pass,
            callee::SLOGIN_STORE_POLL.addr,
            ee::s15::f_003fca40__generated,
            false,
        );
    }

    pub(crate) fn crc32_get_interface_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            crc32_get_interface_pass,
            callee::CRC32_GET_INTERFACE.addr,
            ee::s15::f_003fb5e8__generated,
            false,
        );
    }

    /// Put the pass-through fakes in their callees' slots (once per process).
    pub(crate) fn hook_slogin_passes() {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 17] = [
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F8178),
                slogin_map_store_error_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FCA40),
                slogin_store_poll_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FB5E8),
                crc32_get_interface_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F9208),
                slogin_mc_is_initialized_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_00416210),
                slogin_memset_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004166F4),
                slogin_strcpy_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F86B8),
                slogin_mc_count_devices_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F8550),
                slogin_mc_first_device_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F8588),
                slogin_mc_last_device_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F85C0),
                slogin_mc_next_device_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F86F0),
                slogin_mc_set_error_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F8710),
                slogin_mc_get_error_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FB2F8),
                slogin_mc_set_busy_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003FA7A0),
                slogin_mc_build_dir_path_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003FA818),
                slogin_mc_build_file_paths_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003FA380),
                slogin_mc_ascii_to_sjis_pass,
            ),
            (
                std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FCDC8),
                slogin_store_set_icon_sys_pass,
            ),
        ];
        for (slot, fake) in slots {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// The step callbacks' and `SLogin_BeginSequence`'s shared setup: the
    /// login at DATA + 0x200 (final error word non-zero), the result and
    /// error pointers in a0/a1 (DATA + 0x500 and + 0x504), no fake unwinds.
    pub(crate) fn step_result_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(GAME_GP + login::STATE_GP, DATA + 0x200);
        c.g[A0][0] = sx(DATA + 0x500);
        c.g[A1][0] = sx(DATA + 0x504);
        c.bus
            .write32(DATA + 0x200 + login::FINAL_ERROR, seed ^ 0x1234);
    }

    /// Event delivery at the callback-clear loop head 0x3f79f0 (a structured
    /// loop: `deliver_event_here`, so plain functions on both sides). Mode =
    /// seed % 3, operation seed / 3 % 6 (both arms before the loop). The loop
    /// checks events at its two back edges, 30 and 37 cycles after entry (27
    /// and 34 for operations past 3), so most of the 0..48-cycle event delays
    /// land inside it. The login words at +0xc0..0x150 start non-zero.
    #[test]
    fn slogin_reset_for_op_events_matches() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f7990__generated,
            slogin_reset_for_op,
            90,
            &|c, seed| {
                login_state_fixture(c, seed, 0x003f_7990);
                for i in 0..0x24u32 {
                    c.bus
                        .write32(DATA + 0xc0 + 4 * i, 0x4c00_0000 | i << 8 | seed);
                }
                c.g[A0][0] = (seed / 3 % 6) as u64;
                let mut r = Rng(0x003f_7990 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_7990)
            },
        );
    }

    /// Event delivery at the copy loop head 0x3f7eb0 (a structured loop:
    /// `deliver_event_here`, so plain functions on both sides). Mode = seed %
    /// 3; the source string's length is [3, 0x28][seed / 3 % 2] and the count
    /// [6, 0x20][seed / 6 % 2], so the copy stops at the terminator or at the
    /// count. Each copied byte is followed by an event check, 16 + 8n cycles
    /// after entry; the destination starts non-zero so every store shows.
    #[test]
    fn slogin_str_copy_n_events_matches() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f7e90__generated,
            slogin_str_copy_n,
            90,
            &|c, seed| {
                let mut r = Rng(0x003f_7eb0 ^ seed as u64);
                scramble(c, &mut r);
                let (dst, src) = (DATA + 0x400, DATA + 0x600);
                let length = [3, 0x28][(seed / 3 % 2) as usize];
                for i in 0..0x30u32 {
                    let byte = if i == length {
                        0
                    } else {
                        0x41 + (i * 7 + seed) % 26
                    };
                    c.bus.write8(src + i, byte as u8);
                    c.bus.write8(dst + i, (0xd0 + i) as u8);
                }
                c.g[A0][0] = dst as u64;
                c.g[A1][0] = src as u64;
                let mode = schedule(c, &mut r, seed, 0x003f_7e90);
                c.g[A2][0] = [6, 0x20][(seed / 6 % 2) as usize];
                mode
            },
        );
    }

    /// `login_store_fixture` on the game's image (the store's alignment table
    /// and the store-error table are the game's), with the pass-through fakes
    /// on. Independent digits: the unwinding call (seed % 3: none, the
    /// `TibExtMemFree` fake inside `SLoginStore_FreeBlock`, or
    /// `SLogin_MapStoreError`) and a null record (seed / 3 % 2: no free, error
    /// 4 -> mapped).
    pub(crate) fn free_record_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        elf_image(c);
        login_store_fixture(c, seed, entry);
        slogin_unwind_at(
            c,
            [
                0,
                callee::TIB_EXT_MEM_FREE.addr,
                callee::SLOGIN_MAP_STORE_ERROR.addr,
            ][(seed % 3) as usize],
        );
        if seed / 3 % 2 == 1 {
            c.bus.write32(DATA + 0x144, 0);
        }
    }

    /// On the game's image (store-error table at 0x497150), `arm`: 0 no status
    /// pointer (poll skipped); 1 store not initialised (poll returns 2 ->
    /// 0x14, the mapped-error arm); 2-7 an initialised store whose card poll
    /// (fake card interface) reports status 0 (-> 0x17), 1 (pending), 3 (->
    /// 0x12) or 2 with no done callback, a callback returning non-zero, or
    /// one returning 0. `after` picks the login + 0x100 callback (none, a
    /// fake, one returning 0).
    pub(crate) fn step_poll_fixture(c: &mut Cpu, seed: u32, arm: u32, after: u32) {
        elf_image(c);
        login_store_fixture(c, seed, 0x003f_7708);
        c.g[A0][0] = if arm == 0 { 0 } else { (DATA + 0x2200) as u64 };
        c.bus.write32(DATA + 0x2200, 0x5a5a_0000 | seed);
        c.bus.write32(
            DATA + 0x100,
            [0, fakes::FAKE + 0x40, 0x0014_06c0][after as usize],
        );
        if arm >= 2 {
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
            c.hooks.miss = login_mc_poll_fake;
            c.bus.write32(GAME_GP + 0x2a30, DATA + 0x4000);
            c.bus.write32(GAME_GP + 0x1aec, DATA + 0x3000);
            c.bus.write32(GAME_GP + 0x1ae8, DATA + 0x3400);
            c.bus.write32(DATA + 0x3000 + 0x40, fakes::FAKE + 0x50);
            c.bus.write32(DATA + 0x3000 + 0x4c, DATA + 0x3800);
            c.bus.write32(DATA + 0x3000 + 0x48, DATA + 0x3900);
            c.bus.write32(DATA + 0x3800, fakes::FAKE + 0x60);
            c.bus.write32(DATA + 0x3900, fakes::FAKE + 0x70);
            c.bus.write32(DATA + 0x3400, 0);
            c.bus.write32(DATA + 0x3404, 0);
            c.bus.write32(DATA + 0x4000 + 0x74, 0);
            c.bus
                .write32(DATA + 0x3ec, [0, 1, 3, 2, 2, 2][(arm - 2) as usize]);
            c.bus.write32(
                DATA + 0xfc,
                match arm {
                    6 => fakes::FAKE + 0x80,
                    7 => 0x0014_06c0,
                    _ => 0,
                },
            );
        }
    }

    /// The card fake of `step_poll_fixture`, unwinding at the setup's target.
    pub(crate) fn step_poll_unwind_fake(c: &mut Cpu, target: u32) {
        slogin_arm_unwind(c, target);
        login_mc_poll_fake(c, target);
    }

    /// Each of the four calls unwinds in turn (site = seed % 4), with the
    /// pass-through fakes on so the two real callees can unwind too: the
    /// store poll and `SLogin_MapStoreError` (arms 1-7, seed / 4 % 7 + 1), the
    /// done callback at login + 0xfc (arm 6) and the login + 0x100 callback
    /// (arms 0 and 1, seed / 4 % 2, with the fake callback). The other sites'
    /// + 0x100 callback is seed / 28 % 3.
    #[test]
    fn slogin_step_poll_unwinds_matches() {
        fakes::covered(
            ee::s14::f_003f7708__generated,
            slogin_step_poll,
            84,
            &|c, seed| {
                let site = seed % 4;
                let (arm, after) = match site {
                    0 | 1 => (seed / 4 % 7 + 1, seed / 28 % 3),
                    2 => (6, seed / 28 % 3),
                    _ => (seed / 4 % 2, 1),
                };
                step_poll_fixture(c, seed, arm, after);
                c.hooks.miss = step_poll_unwind_fake;
                slogin_unwind_at(
                    c,
                    [
                        callee::SLOGIN_STORE_POLL.addr,
                        callee::SLOGIN_MAP_STORE_ERROR.addr,
                        fakes::FAKE + 0x80,
                        fakes::FAKE + 0x40,
                    ][site as usize],
                );
            },
        );
    }

    /// Scenarios of `create_step_fixture` (`seed % CREATE_STEP_SCENARIOS`).
    pub(crate) const CREATE_STEP_SCENARIOS: u32 = 67;

    /// Every scenario with the real callees logged ([`slogin_mc_pass_covered`]) and
    /// the unwind index `seed / CREATE_STEP_SCENARIOS % 3` (0, 1 or 2), so
    /// the `?` after each real callee is taken: BuildDirPath (scenario 4,
    /// call 0), NextDeviceInMask (8, call 0), GetError (7: no further card,
    /// call 1), SetError (10, call 0), BuildFilePaths (19, call 0).
    #[test]
    fn slogin_mc_create_step_callees_matches() {
        slogin_mc_pass_covered(
            ee::s14::f_003fa8f8__generated,
            slogin_mc_create_step,
            3 * CREATE_STEP_SCENARIOS,
            &[
                callee::SLOGIN_MC_BUILD_DIR_PATH.addr,
                callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.addr,
                callee::SLOGIN_MC_GET_ERROR.addr,
                callee::SLOGIN_MC_SET_ERROR.addr,
                callee::SLOGIN_MC_BUILD_FILE_PATHS.addr,
            ],
            &|c, seed| {
                create_step_fixture(c, seed);
                c.bus
                    .write32(LOG_UNWIND_AT, seed / CREATE_STEP_SCENARIOS % 3);
            },
        );
    }

    /// Every `delete_step_fixture` seed (224) with the real callees logged
    /// ([`slogin_mc_pass_covered`]) and the unwind index `seed / 224 % 3` (0, 1 or
    /// 2), so the `?` after each real callee is taken: phase 1 calls SetError
    /// first (call 0), then BuildDirPath (result 0, call 1) or, after result
    /// 3 in mode 2, NextDeviceInMask (call 1) and, with no further card,
    /// GetError (call 2).
    #[test]
    fn slogin_mc_delete_step_callees_matches() {
        slogin_mc_pass_covered(
            ee::s14::f_003fae50__generated,
            slogin_mc_delete_step,
            3 * 224,
            &[
                callee::SLOGIN_MC_SET_ERROR.addr,
                callee::SLOGIN_MC_BUILD_DIR_PATH.addr,
                callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.addr,
                callee::SLOGIN_MC_GET_ERROR.addr,
            ],
            &|c, seed| {
                delete_step_fixture(c, seed);
                c.bus.write32(LOG_UNWIND_AT, seed / 224 % 3);
            },
        );
    }

    /// The hand `SLoginMc_DeleteStep`, counting its own re-entries at the
    /// retry head (`drive`'s resume count pools both sides).
    pub(crate) fn slogin_mc_delete_step_counted(c: &mut Cpu) {
        if c.pc == DELETE_RETRY_HEAD {
            mc_hand_resumes().with(|n| n.set(n.get() + 1));
        }
        slogin_mc_delete_step(c);
    }

    /// The scan past bit 0 runs seven trips (mask 0, or only bit 7) or six
    /// (only bit 6) with an event check after each, `seed / 3 % 3`; bits
    /// above the low halfword are noise the function drops.
    #[test]
    fn slogin_mc_first_device_in_mask_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f8550__generated,
            slogin_mc_first_device_in_mask,
            60,
            &|c, seed| {
                let mut r = Rng(0x3f85_68ee ^ seed as u64);
                scramble(c, &mut r);
                let mask = [0u64, 0x80, 0x40][(seed / 3 % 3) as usize];
                c.g[A0][0] = mask | (r.next_u32() as u64) << 16;
                schedule(c, &mut r, seed, 0x003f_8550)
            },
        );
    }

    /// The scan down from bit 6 runs seven trips (mask 0) or six or five
    /// (only bit 0 or 1) with an event check after each, `seed / 3 % 3`.
    #[test]
    fn slogin_mc_last_device_in_mask_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f8588__generated,
            slogin_mc_last_device_in_mask,
            60,
            &|c, seed| {
                let mut r = Rng(0x3f85_a0ee ^ seed as u64);
                scramble(c, &mut r);
                let mask = [0u64, 1, 2][(seed / 3 % 3) as usize];
                c.g[A0][0] = mask | (r.next_u32() as u64) << 16;
                schedule(c, &mut r, seed, 0x003f_8588)
            },
        );
    }

    /// Every mask runs the eight trips (seven event checks).
    #[test]
    fn slogin_mc_count_devices_in_mask_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f86b8__generated,
            slogin_mc_count_devices_in_mask,
            60,
            &|c, seed| {
                let mut r = Rng(0x3f86_c8ee ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = ((seed / 3 * 0x31) & 0xff) as u64 | (r.next_u32() as u64) << 16;
                schedule(c, &mut r, seed, 0x003f_86b8)
            },
        );
    }

    /// Both scan loops, `seed / 3 % 2`: forward (head 0x3f8608) from device
    /// -1, or backward (head 0x3f8678) from device 8; `seed / 6 % 2`: no
    /// device in the way, or only the far end's (bit 7 forward, bit 0
    /// backward). Six or seven trips either way.
    #[test]
    fn slogin_mc_next_device_in_mask_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f85c0__generated,
            slogin_mc_next_device_in_mask,
            60,
            &|c, seed| {
                let mut r = Rng(0x3f86_08ee ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let state = DATA + 0x400;
                c.bus.write32(GAME_GP + card::STATE_GP, state);
                let backward = seed / 3 % 2 == 1;
                let far = seed / 6 % 2 == 1;
                let current: i16 = if backward { 8 } else { -1 };
                c.bus.write16(state + card::DEVICE, current as u16);
                c.bus.write16(state + card::DEVICE + 2, 0x7777);
                let mask: u64 = match (backward, far) {
                    (_, false) => 0,
                    (false, true) => 0x80,
                    (true, true) => 1,
                };
                c.g[A0][0] = mask | (r.next_u32() as u64) << 16;
                c.g[A1][0] = backward as u64;
                schedule(c, &mut r, seed, 0x003f_85c0)
            },
        );
    }

    /// `seed % 4`: no output record (4 cycles); the aligned copy (43: the
    /// state's copy area at state + 0xc and the output both 8-byte aligned);
    /// the misaligned copy (66) with the output 1 byte off, or with the copy
    /// area and the output both 4 bytes off (the alignment test ORs the two,
    /// so only the unaligned copy is right). Three trips each. The output
    /// starts at 0xa5 bytes and the source bytes (0x10..0x6f) never match it.
    pub(crate) fn mc_get_file_state_fixture(c: &mut Cpu, seed: u32) {
        mc_fixture(c, seed, 0x003f_95e8);
        let class = seed % 4;
        let state = if class == 1 {
            DATA + 0x2004
        } else {
            DATA + 0x2000
        };
        c.bus.write32(GAME_GP + card::STATE_GP, state);
        c.g[A0][0] = match class {
            0 => 0,
            1 => (DATA + 0x1000) as u64,
            2 => (DATA + 0x1001) as u64,
            _ => (DATA + 0x1004) as u64,
        };
        for i in 0..0x70u32 {
            c.bus.write8(DATA + 0x1000 + i, 0xa5);
        }
        for i in 0..0x60u32 {
            c.bus
                .write8(state + 0xc + i, (0x10 + (i ^ seed) % 0x60) as u8);
        }
    }

    /// Both copy loops (aligned head 0x3f9660: class 1; unaligned head
    /// 0x3f960c: classes 2 and 3, `seed / 3 % 3`), three trips each.
    #[test]
    fn slogin_mc_get_file_state_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003f95e8__generated,
            slogin_mc_get_file_state,
            60,
            &|c, seed| {
                mc_get_file_state_fixture(c, seed * 4 + 1 + seed / 3 % 3);
                let mut r = Rng(0x003f_95e8 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_95e8)
            },
        );
    }

    /// Both icon copy loops, `slogin_mc_set_icon_sys_fixture` classes 2..5
    /// (`seed / 3 % 4`): the aligned copy (head 0x3fa2e4) and the misaligned
    /// one (head 0x3fa28c), 30 trips each.
    #[test]
    fn slogin_mc_set_icon_sys_events() {
        use crate::difftest::events::{check_events, schedule};
        check_events(
            ee::s14::f_003fa240__generated,
            slogin_mc_set_icon_sys,
            60,
            &|c, seed| {
                slogin_mc_set_icon_sys_fixture(c, seed * 7 + 2 + seed / 3 % 4);
                let mut r = Rng(0x003f_a240 ^ seed as u64);
                schedule(c, &mut r, seed, 0x003f_a240)
            },
        );
    }

    /// The jump targets of the game's table at `SPLIT_DEVICE_INDEX_TABLE`
    /// (device 0..7: port 0 slots 0-3, port 1 slots 0-3), the eight arms the
    /// port handles.
    pub(crate) const SPLIT_DEVICE_INDEX_TARGETS: [u32; 8] = [
        0x003f_a72c,
        0x003f_a738,
        0x003f_a748,
        0x003f_a750,
        0x003f_a758,
        0x003f_a768,
        0x003f_a778,
        0x003f_a784,
    ];

    /// Phase 3, which the dispatch's `phase < 3` compare puts with the
    /// unknown phases: states 3 and 6 reach the dispatch and take the
    /// unknown-phase exit (`*done` = 2, 0x12; 40 and 41 cycles, 2 more than
    /// the phase-0 arm the compare would pick with `<=`); states 0 and 4 leave
    /// before it (27). The seeds run over `mc_open_step_fixture`'s phase-0
    /// slice (every state, a0, reply, error and mask row).
    #[test]
    fn slogin_mc_open_step_phase_three_matches() {
        check_paths(
            ee::s14::f_003f8c38__generated,
            slogin_mc_open_step,
            144,
            &[27, 40, 41],
            &|c, i| {
                mc_open_step_fixture(c, 6 * i);
                c.bus.write32(DATA + 0x400 + card::PHASE, 3);
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            },
        );
    }

    /// The open step's retry head (the next device after an error), where an
    /// unwind at its back edge resumes.
    pub(crate) const MC_OPEN_STEP_RETRY_HEAD: u32 = 0x003f_8d4c;

    /// Re-entries of a hand dispatcher at its own resume head (this test
    /// thread; `drive`'s resume count pools both sides).
    pub(crate) fn mc_hand_resumes() -> &'static std::thread::LocalKey<std::cell::Cell<u32>> {
        thread_local! {
            static RESUMES: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
        }
        &RESUMES
    }

    /// The hand `SLoginMc_OpenStep`, counting its own re-entries at the retry
    /// head.
    pub(crate) fn slogin_mc_open_step_counted(c: &mut Cpu) {
        if c.pc == MC_OPEN_STEP_RETRY_HEAD {
            mc_hand_resumes().with(|n| n.set(n.get() + 1));
        }
        slogin_mc_open_step(c);
    }

    /// The pass-through gate's second value: the SLogin_* slots and also the
    /// card-layer and libc callees of the `SLoginMc_*` tests (their
    /// `*_callees_matches`), which stay off under `SLOGIN_PASS_MAGIC` so the
    /// SLogin_* tests' call counts do not change.
    pub(crate) const SLOGIN_PASS_ALL_MAGIC: u32 = 0x736c_7061;

    /// Is the pass-through gate on for a slot (`card_layer`: one of the
    /// `SLOGIN_PASS_ALL_MAGIC`-only slots)?
    pub(crate) fn slogin_pass_on(c: &mut Cpu, card_layer: bool) -> bool {
        let gate = c.bus.read32(SLOGIN_PASS_CTRL);
        gate == SLOGIN_PASS_ALL_MAGIC || (gate == SLOGIN_PASS_MAGIC && !card_layer)
    }

    /// Targets at which a pass-through fake unwound (this test thread).
    pub(crate) fn slogin_pass_unwound(
    ) -> &'static std::thread::LocalKey<std::cell::RefCell<std::collections::BTreeSet<u32>>> {
        thread_local! {
            static UNWOUND: std::cell::RefCell<std::collections::BTreeSet<u32>> =
                const { std::cell::RefCell::new(std::collections::BTreeSet::new()) };
        }
        &UNWOUND
    }

    pub(crate) fn slogin_mc_is_initialized_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_is_initialized_pass,
            callee::SLOGIN_MC_IS_INITIALIZED.addr,
            ee::s14::f_003f9208__generated,
            true,
        );
    }

    pub(crate) fn slogin_memset_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_memset_pass,
            callee::MEMSET.addr,
            ee::s15::f_00416210__generated,
            true,
        );
    }

    pub(crate) fn slogin_strcpy_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_strcpy_pass,
            callee::STRCPY.addr,
            ee::s15::f_004166f4__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_count_devices_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_count_devices_pass,
            callee::SLOGIN_MC_COUNT_DEVICES_IN_MASK.addr,
            ee::s14::f_003f86b8__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_first_device_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_first_device_pass,
            callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK.addr,
            ee::s14::f_003f8550__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_last_device_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_last_device_pass,
            callee::SLOGIN_MC_LAST_DEVICE_IN_MASK.addr,
            ee::s14::f_003f8588__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_next_device_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_next_device_pass,
            callee::SLOGIN_MC_NEXT_DEVICE_IN_MASK.addr,
            ee::s14::f_003f85c0__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_set_error_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_set_error_pass,
            callee::SLOGIN_MC_SET_ERROR.addr,
            ee::s14::f_003f86f0__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_get_error_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_get_error_pass,
            callee::SLOGIN_MC_GET_ERROR.addr,
            ee::s14::f_003f8710__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_set_busy_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_set_busy_pass,
            callee::SLOGIN_MC_SET_BUSY.addr,
            ee::s15::f_003fb2f8__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_build_dir_path_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_build_dir_path_pass,
            callee::SLOGIN_MC_BUILD_DIR_PATH.addr,
            ee::s14::f_003fa7a0__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_build_file_paths_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_build_file_paths_pass,
            callee::SLOGIN_MC_BUILD_FILE_PATHS.addr,
            ee::s14::f_003fa818__generated,
            true,
        );
    }

    pub(crate) fn slogin_mc_ascii_to_sjis_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_mc_ascii_to_sjis_pass,
            callee::SLOGIN_MC_ASCII_TO_SJIS.addr,
            ee::s14::f_003fa380__generated,
            true,
        );
    }

    pub(crate) fn slogin_store_set_icon_sys_pass(c: &mut Cpu) {
        slogin_pass_through(
            c,
            slogin_store_set_icon_sys_pass,
            callee::SLOGIN_STORE_SET_ICON_SYS.addr,
            ee::s15::f_003fcdc8__generated,
            true,
        );
    }

    /// `fakes::covered` with every pass-through on (`SLOGIN_PASS_ALL_MAGIC`,
    /// no armed target, so the setup's `LOG_UNWIND_AT` index picks the call
    /// that unwinds), then require that each of `targets` unwound on some
    /// seed (its `?` exit was taken). The last check is skipped when
    /// `SSX3_DIFF_SEEDS` cuts the seeds.
    pub(crate) fn slogin_mc_pass_covered(
        generated: fn(&mut Cpu),
        hand: fn(&mut Cpu),
        seeds: u32,
        targets: &[u32],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        hook_slogin_passes();
        slogin_pass_unwound().with(|u| u.borrow_mut().clear());
        fakes::covered(generated, hand, seeds, &|c, seed| {
            setup(c, seed);
            c.bus.write32(SLOGIN_PASS_CTRL, SLOGIN_PASS_ALL_MAGIC);
            c.bus.write32(SLOGIN_UNWIND_TARGET, 0);
        });
        if crate::difftest::seed_count(seeds) < seeds {
            return;
        }
        let unwound = slogin_pass_unwound().with(|u| u.borrow().clone());
        for t in targets {
            assert!(
                unwound.contains(t),
                "no seed unwound at callee {t:#x} (unwound at {unwound:x?})"
            );
        }
    }

    /// `slogin_setup_icon_sys_matches`' state (icon `seed % 2`, store
    /// `seed / 2 % 3`) with the unwind index `[none, 0, .., 5][seed / 6 % 7]`
    /// over the logged calls: memset (0), and with an icon memset (1),
    /// AsciiToSjis (2), SLoginStore_SetIconSys (3; it calls the logged
    /// SLoginMc_IsInitialized (4) once the store is initialised) and
    /// SLogin_MapStoreError (4 or 5).
    pub(crate) fn slogin_setup_icon_sys_callees_fixture(c: &mut Cpu, seed: u32) {
        elf_image(c);
        let _r = fakes::machine(c, seed, 0x3f82_5800, 0);
        c.pc = 0x003f_8258;
        for i in 0..0x110u32 {
            c.bus.write32(
                crate::difftest::SP - 0x540 + 4 * i,
                0x1c00_0000 | i << 8 | seed,
            );
        }
        for i in 0..0x110u32 {
            c.bus
                .write32(DATA + 0x1800 + 4 * i, 0x1d00_0000 | i << 8 | seed);
        }
        c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x1000) };
        let store = seed / 2 % 3;
        c.bus.write32(
            GAME_GP + store::STATE_GP,
            if store == 0 { 0 } else { DATA + 0x1700 },
        );
        c.bus.write32(
            GAME_GP + card::IFACE_GP,
            if store == 2 { DATA + 0x1800 } else { 0 },
        );
        let at = seed / 6 % 7;
        c.bus
            .write32(LOG_UNWIND_AT, if at == 0 { u32::MAX } else { at - 1 });
    }

    /// SetupIconSys's real callees logged with their return addresses
    /// ([`slogin_mc_pass_covered`]); each of the four direct ones unwinds on some
    /// seed, so the early return after each is taken.
    #[test]
    fn slogin_setup_icon_sys_callees_matches() {
        slogin_mc_pass_covered(
            ee::s14::f_003f8258__generated,
            slogin_setup_icon_sys,
            42,
            &[
                callee::MEMSET.addr,
                callee::SLOGIN_MC_ASCII_TO_SJIS.addr,
                callee::SLOGIN_STORE_SET_ICON_SYS.addr,
                callee::SLOGIN_MAP_STORE_ERROR.addr,
            ],
            &slogin_setup_icon_sys_callees_fixture,
        );
    }

    /// `mc_named_fixture` (class `seed % 11`) with the unwind index
    /// `[none, 0, 1, .., 7][seed / 11 % 9]`, so the call sequence
    /// (IsInitialized, memset, Count/First/LastDeviceInMask, SetError, strcpy,
    /// SetBusy, the interface's +0x10 method) stops at each of its calls in
    /// turn. Run through [`slogin_mc_pass_covered`], which logs the real callees
    /// with their return addresses.
    pub(crate) fn mc_named_callees_fixture(c: &mut Cpu, seed: u32, open: bool) {
        mc_named_fixture(c, seed, open);
        let at = seed / 11 % 9;
        c.bus
            .write32(LOG_UNWIND_AT, if at == 0 { u32::MAX } else { at - 1 });
    }

    /// The real callees of StartDelete/StartOpen, every one reached by some
    /// class: IsInitialized (all), memset, SetError, strcpy and SetBusy
    /// (classes 5-8, 10), CountDevicesInMask (6-9), FirstDeviceInMask (6:
    /// one device; 7: several, a2 = 0), LastDeviceInMask (8).
    pub(crate) const MC_NAMED_REAL_CALLEES: [u32; 8] = [
        callee::SLOGIN_MC_IS_INITIALIZED.addr,
        callee::MEMSET.addr,
        callee::SLOGIN_MC_COUNT_DEVICES_IN_MASK.addr,
        callee::SLOGIN_MC_FIRST_DEVICE_IN_MASK.addr,
        callee::SLOGIN_MC_LAST_DEVICE_IN_MASK.addr,
        callee::SLOGIN_MC_SET_ERROR.addr,
        callee::STRCPY.addr,
        callee::SLOGIN_MC_SET_BUSY.addr,
    ];

    /// Every real call's return address is in the compared log, and each real
    /// callee unwinds on some seed (the `?` after it).
    #[test]
    fn slogin_mc_start_delete_callees_matches() {
        slogin_mc_pass_covered(
            ee::s14::f_003f9698__generated,
            slogin_mc_start_delete,
            198,
            &MC_NAMED_REAL_CALLEES,
            &|c, seed| mc_named_callees_fixture(c, seed, false),
        );
    }

    /// As for StartDelete; StartOpen's return addresses are StartDelete's
    /// shifted by 0x320, so a wrong shift shows in the log here.
    #[test]
    fn slogin_mc_start_open_callees_matches() {
        slogin_mc_pass_covered(
            ee::s14::f_003f99b8__generated,
            slogin_mc_start_open,
            198,
            &MC_NAMED_REAL_CALLEES,
            &|c, seed| mc_named_callees_fixture(c, seed, true),
        );
    }

    /// The relay's entry `slot` points at fake `seed % 3` (three call
    /// targets); the call unwinds when seed / 3 % 3 = 0.
    pub(crate) fn dnas_relay_call_fixture(c: &mut Cpu, seed: u32, slot: u32) {
        let _r = fakes::machine(c, seed, 0x003f_d100, 1);
        c.bus
            .write32(DNAS_RELAY_BASE + slot, fakes::FAKE + (seed % 3) * 0x10);
        c.bus.write32(
            fakes::LOG_UNWIND_AT,
            if seed / 3 % 3 == 0 { 0 } else { u32::MAX },
        );
    }

    /// Gate of the `SLoginStore_GetAlignment` fake (its override slot): with
    /// the magic word here the call is a logged fake that writes neither
    /// output word, so the caller's own stores of 4 and 1 to `sp` and
    /// `sp + 4` (the real callee writes the same values over them) stay in
    /// RAM and the second one is read back as the unit.
    pub(crate) const STORE_ALIGNMENT_FAKE_CTRL: u32 = DATA + 0xf200;

    pub(crate) const STORE_ALIGNMENT_FAKE_MAGIC: u32 = 0x5341_4c47;

    pub(crate) fn store_alignment_fake(c: &mut Cpu) {
        if c.bus.read32(STORE_ALIGNMENT_FAKE_CTRL) != STORE_ALIGNMENT_FAKE_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(store_alignment_fake),
                ee::s15::f_003fb858__generated,
            );
        }
        fakes::fake_call(c, callee::SLOGIN_STORE_GET_ALIGNMENT.addr);
    }

    /// `SLoginStore_CalcAllocSize` with `GetAlignment` faked (no output
    /// writes): the frame words around `sp` start random (`fakes::machine`),
    /// so a dropped `sw sp = 4` shows in RAM and a dropped `sw sp+4 = 1`
    /// in the result. Payload seed % 4 (0, 0xfffffff4, 0xfffffff5, random:
    /// both arms), the fake unwinds when seed / 4 % 3 = 0 (the `?` after
    /// the call), format seed / 12 % 2 (table or default; unused by the fake).
    #[test]
    fn slogin_store_calc_alloc_size_preset_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_003FB858),
            store_alignment_fake,
            fakes::prev_cell(store_alignment_fake),
        );
        fakes::covered(
            ee::s15::f_003fb898__generated,
            slogin_store_calc_alloc_size,
            48,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3fb8_9801, 0);
                c.pc = 0x003f_b898;
                c.bus
                    .write32(STORE_ALIGNMENT_FAKE_CTRL, STORE_ALIGNMENT_FAKE_MAGIC);
                c.bus
                    .write32(LOG_UNWIND_AT, if seed / 4 % 3 == 0 { 0 } else { u32::MAX });
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => 0u32.wrapping_sub(BLOCK_HEADER_BYTES),
                    2 => 1u32.wrapping_sub(BLOCK_HEADER_BYTES),
                    _ => r.next_u32(),
                } as u64;
                c.g[A1][0] = [1, 7][(seed / 12 % 2) as usize];
            },
        );
    }

    /// Gate of the card-layer start fakes (the override slots of
    /// `SLoginMc_StartOpen`, `StartSeek`, `StartRead` and `StartWrite`): with
    /// the magic word here a start is a logged fake call (`fakes::fake_call`:
    /// it returns a pointer, the result forced at `LOG_RESULTS`, or unwinds
    /// at `LOG_UNWIND_AT`), else the slot's previous body runs.
    pub(crate) const STORE_CARD_START_CTRL: u32 = DATA + 0xf100;

    pub(crate) const STORE_CARD_START_MAGIC: u32 = 0x5343_5354;

    /// The shared body of the four start fakes (the callee is `c.pc`, set by
    /// both call forms before the slot runs).
    pub(crate) fn store_card_start_fake(c: &mut Cpu, fake: fn(&mut Cpu), generated: fn(&mut Cpu)) {
        if c.bus.read32(STORE_CARD_START_CTRL) != STORE_CARD_START_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(fake), generated);
        }
        let target = c.pc;
        fakes::fake_call(c, target);
    }

    pub(crate) fn store_card_start_open_fake(c: &mut Cpu) {
        store_card_start_fake(
            c,
            store_card_start_open_fake,
            ee::s14::f_003f99b8__generated,
        );
    }

    pub(crate) fn store_card_start_seek_fake(c: &mut Cpu) {
        store_card_start_fake(
            c,
            store_card_start_seek_fake,
            ee::s14::f_003f9c48__generated,
        );
    }

    pub(crate) fn store_card_start_read_fake(c: &mut Cpu) {
        store_card_start_fake(
            c,
            store_card_start_read_fake,
            ee::s14::f_003f9dc0__generated,
        );
    }

    pub(crate) fn store_card_start_write_fake(c: &mut Cpu) {
        store_card_start_fake(
            c,
            store_card_start_write_fake,
            ee::s14::f_003f9f00__generated,
        );
    }

    /// Hooks the four start fakes and arms them: the start is fake call
    /// `card_call`, which unwinds when `unwind` is set and otherwise returns
    /// `result` (0: an unforced pointer).
    pub(crate) fn store_card_start_arm(c: &mut Cpu, card_call: u32, unwind: bool, result: u32) {
        let slots: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 4] = [
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F99B8),
                store_card_start_open_fake,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F9C48),
                store_card_start_seek_fake,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F9DC0),
                store_card_start_read_fake,
            ),
            (
                std::ptr::addr_of_mut!(ee::s14::OVERRIDE_003F9F00),
                store_card_start_write_fake,
            ),
        ];
        for (slot, fake) in slots {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
        c.bus.write32(STORE_CARD_START_CTRL, STORE_CARD_START_MAGIC);
        c.bus
            .write32(LOG_UNWIND_AT, if unwind { card_call } else { u32::MAX });
        c.bus.write32(LOG_RESULTS + 4 * card_call, result);
    }

    /// `SLoginStore_BeginOpen` with `SLoginMc_StartOpen` faked: it unwinds
    /// (seed % 2 = 0: the `?` after the start), or returns an unforced
    /// pointer or 0xd (seed / 2 % 2) for the state stores and the error map.
    #[test]
    fn slogin_store_begin_open_card_unwind_matches() {
        fakes::covered(
            ee::s15::f_003fbd70__generated,
            slogin_store_begin_open,
            24,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbd_7001);
                c.pc = 0x003f_bd70;
                store_card_start_arm(c, 0, seed % 2 == 0, [0, 0xd][(seed / 2 % 2) as usize]);
                c.g[A0][0] = sx(DATA + 0x4000);
                c.g[A1][0] = 2;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// `SLoginStore_SeekToStart` with `SLoginMc_StartSeek` faked: unwinds
    /// (seed % 2 = 0) or returns a pointer or 0xd (seed / 2 % 2).
    #[test]
    fn slogin_store_seek_to_start_card_unwind_matches() {
        fakes::covered(
            ee::s15::f_003fbdb8__generated,
            slogin_store_seek_to_start,
            24,
            &|c, seed| {
                let _r = store_io_machine(c, seed, 0x3fbd_b801);
                c.pc = 0x003f_bdb8;
                store_card_start_arm(c, 0, seed % 2 == 0, [0, 0xd][(seed / 2 % 2) as usize]);
            },
        );
    }

    /// `SLoginStore_ReadDirHeader` with `SLoginMc_StartRead` faked (after
    /// the real `CalcAllocSize`, `elf_image`): unwinds (seed % 2 = 0) or
    /// returns a pointer or 0xd (seed / 2 % 2).
    #[test]
    fn slogin_store_read_dir_header_card_unwind_matches() {
        fakes::covered(
            ee::s15::f_003fbdf0__generated,
            slogin_store_read_dir_header,
            24,
            &|c, seed| {
                let _r = store_io_machine(c, seed, 0x3fbd_f001);
                elf_image(c);
                c.pc = 0x003f_bdf0;
                store_card_start_arm(c, 0, seed % 2 == 0, [0, 0xd][(seed / 2 % 2) as usize]);
            },
        );
    }

    /// `SLoginStore_ReadRecord` with `SLoginMc_StartRead` faked (after the
    /// real `GetBlockHeader` and `CalcAllocSize`): unwinds (seed % 2 = 0) or
    /// returns a pointer or 0xd (seed / 2 % 2); record size seed / 4 % 3.
    #[test]
    fn slogin_store_read_record_card_unwind_matches() {
        fakes::covered(
            ee::s15::f_003fbf80__generated,
            slogin_store_read_record,
            36,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbf_8001);
                elf_image(c);
                c.pc = 0x003f_bf80;
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_BUFFER, DATA + 0x5010);
                let size = [0, 13, r.next_u32() & 0xfff][(seed / 4 % 3) as usize];
                c.bus.write32(STORE_TEST_STATE + store::RECORD_SIZE, size);
                store_card_start_arm(c, 0, seed % 2 == 0, [0, 0xd][(seed / 2 % 2) as usize]);
            },
        );
    }

    /// `SLoginStore_LoadDirEntry` past the decrypt and `AddRecord` (an entry
    /// added after two others, so the seek starts) with `SLoginMc_StartSeek`
    /// faked: it unwinds (seed % 2 = 0) or returns a pointer or 0xd
    /// (seed / 2 % 2); the cipher is present (three fake calls first, the
    /// start is call 3) or absent (call 0) by seed / 4 % 2.
    #[test]
    fn slogin_store_load_dir_entry_card_unwind_matches() {
        fakes::covered(
            ee::s15::f_003fbe40__generated,
            slogin_store_load_dir_entry,
            32,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbe_4001);
                c.pc = 0x003f_be40;
                let cipher = seed / 4 % 2 == 0;
                if !cipher {
                    c.bus.write32(STORE_TEST_STATE + store::CIPHER_IFACE, 0);
                }
                c.bus.write32(STORE_TEST_STATE + store::RECORD_COUNT, 2);
                c.bus.write32(STORE_TEST_STATE + store::RECORD_CAPACITY, 6);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORDS, STORE_TEST_RECORDS);
                c.bus
                    .write32(STORE_TEST_STATE + store::DIR_BUFFER, DATA + 0x4000);
                fill(c, &mut r, STORE_TEST_RECORDS, 4 * 8);
                fill(c, &mut r, DATA + 0x4000, 3);
                c.bus.write32(DATA + 0x4004, r.next_u32() & 0xfff);
                store_card_start_arm(
                    c,
                    if cipher { 3 } else { 0 },
                    seed % 2 == 0,
                    [0, 0xd][(seed / 2 % 2) as usize],
                );
            },
        );
    }

    /// `SLoginStore_WriteRecord` past the header and both encrypts (a valid
    /// block header from the game's template, `elf_image`) with
    /// `SLoginMc_StartWrite` faked: it unwinds (seed % 2 = 0) or returns a
    /// pointer or 0xd (seed / 2 % 2); the cipher is present (six fake calls
    /// first, the start is call 6) or absent (call 0) by seed / 4 % 2.
    #[test]
    fn slogin_store_write_record_card_unwind_matches() {
        fakes::covered(
            ee::s15::f_003fbed0__generated,
            slogin_store_write_record,
            32,
            &|c, seed| {
                let mut r = store_io_machine(c, seed, 0x3fbe_d001);
                elf_image(c);
                c.pc = 0x003f_bed0;
                let cipher = seed / 4 % 2 == 0;
                if !cipher {
                    c.bus.write32(STORE_TEST_STATE + store::CIPHER_IFACE, 0);
                }
                let buffer = DATA + 0x5010;
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_BUFFER, buffer);
                for i in 0..BLOCK_HEADER_BYTES {
                    let b = c.bus.read8(BLOCK_HEADER_MAGIC + i);
                    c.bus.write8(buffer - BLOCK_HEADER_BYTES + i, b);
                }
                fill(c, &mut r, buffer, 8);
                c.bus
                    .write32(STORE_TEST_STATE + store::RECORD_SIZE, r.next_u32() & 0xfff);
                store_card_start_arm(
                    c,
                    if cipher { 6 } else { 0 },
                    seed % 2 == 0,
                    [0, 0xd][(seed / 2 % 2) as usize],
                );
            },
        );
    }

    /// Where the `DnasRel_RelocateAll` fixture puts the target sections
    /// (`.text`, `.data`, `.rodata`: 0x80 apart), their relocation sections
    /// (0x40 apart) and the section-name table, as image offsets.
    pub(crate) const DNAS_RELOCATE_ALL_TARGETS: u32 = 0x400;

    pub(crate) const DNAS_RELOCATE_ALL_RELS: u32 = 0x600;

    pub(crate) const DNAS_RELOCATE_ALL_NAMES: u32 = 0x40;

    /// A relay for `DnasRel_RelocateAll` with `count` section headers
    /// (0, or 7: the name table and the six sections the body looks up, in
    /// its order `.text`, `.rel.text`, `.data`, `.rel.data`, `.rodata`,
    /// `.rel.rodata`). The names are the game's (`elf_image`), copied into
    /// the image's name table; section `renamed` (if any) gets a different
    /// first byte so its lookup fails. Each relocation section holds an
    /// R_MIPS_32, an R_MIPS_26, a HI16 and a LO16 entry on symbols
    /// `(entry + section + shift) % 6` (the six symbols of
    /// `dnas_rel_relocate_section_fixture`: below, inside and at the end of
    /// the remap window, section-relative, absolute, far above).
    pub(crate) fn dnas_rel_relocate_all_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        count: u16,
        renamed: Option<u32>,
        shift: u32,
    ) {
        elf_image(c);
        c.pc = 0x003f_d660;
        let image = DNAS_RELOC_IMAGE;
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DNAS_RELOC_REMAP);
        c.bus.write32(DATA, image);
        c.bus.write32(DATA + 0x10, DNAS_RELOC_SYMTAB);
        c.bus.write32(DATA + 0x14, 6);
        c.bus.write32(DATA + 0x20, DNAS_RELOC_STRTAB);
        c.bus.write32(image + ELF_SECTION_OFFSET, 0x100);
        c.bus.write16(image + ELF_SECTION_COUNT, count);
        c.bus.write16(image + ELF_SECTION_NAMES_INDEX, 0);
        let headers = image + 0x100;
        let header = |i: u32| headers + i * ELF_SECTION_SIZE as u32;
        let names = image + DNAS_RELOCATE_ALL_NAMES;
        c.bus.write8(names, 0);
        c.bus.write32(header(0) + ELF_SECTION_NAME, 0);
        c.bus
            .write32(header(0) + ELF_SECTION_FILE_OFFSET, DNAS_RELOCATE_ALL_NAMES);
        let order = [
            DNAS_TEXT_NAME,
            DNAS_TEXT_RELOCS,
            DNAS_DATA_NAME,
            DNAS_DATA_RELOCS,
            DNAS_RODATA_NAME,
            DNAS_RODATA_RELOCS,
        ];
        let mut at = 1;
        for (j, &name) in order.iter().enumerate() {
            let j = j as u32;
            let len = dnas_load_copy_cstr(c, name, names + at);
            if renamed == Some(j) {
                c.bus.write8(names + at, b'-');
            }
            let h = header(1 + j);
            c.bus.write32(h + ELF_SECTION_NAME, at);
            let k = j / 2;
            let (offset, size) = if j % 2 == 0 {
                (DNAS_RELOCATE_ALL_TARGETS + 0x80 * k, 0x40)
            } else {
                (DNAS_RELOCATE_ALL_RELS + 0x40 * k, 4 * REL_ENTRY_SIZE)
            };
            c.bus.write32(h + ELF_SECTION_FILE_OFFSET, offset);
            c.bus.write32(h + ELF_SECTION_BYTES, size);
            at += len + 1;
        }
        crate::difftest::fill(c, r, image + DNAS_RELOCATE_ALL_TARGETS, 0x180 / 4);
        let kinds = [REL_TYPE_32, REL_TYPE_26, REL_TYPE_HI16, REL_TYPE_LO16];
        for k in 0..3 {
            let rel = image + DNAS_RELOCATE_ALL_RELS + 0x40 * k;
            for (i, &kind) in kinds.iter().enumerate() {
                let i = i as u32;
                let sym = (i + k + shift) % 6;
                c.bus.write32(rel + i * REL_ENTRY_SIZE, 4 * i);
                c.bus
                    .write32(rel + i * REL_ENTRY_SIZE + 4, sym << 8 | kind as u32);
            }
        }
        let symbols = [
            (0x5f0, 0xff00),
            (0x600 + 4 * (shift % 16), 0xff00),
            (0x640, 0xff00),
            (0x10, 1),
            (0x8000, ELF_SECTION_ABS as u16),
            (0x7000, 0xff00),
        ];
        for (i, (value, section)) in symbols.into_iter().enumerate() {
            let sym = DNAS_RELOC_SYMTAB + i as u32 * 0x10;
            c.bus.write32(sym, 1);
            c.bus.write32(sym + 4, value);
            c.bus.write16(sym + 0xe, section);
        }
        // Every symbol name is the empty string at offset 1.
        c.bus.write8(DNAS_RELOC_STRTAB, b'z');
        c.bus.write8(DNAS_RELOC_STRTAB + 1, 0);
        c.bus.write32(DNAS_RELOC_REMAP, 0x40);
        c.bus
            .write32(DNAS_RELOC_REMAP + 4, DNAS_RELOC_IMAGE + 0x600);
        c.bus.write32(DNAS_RELOC_REMAP + 8, DNAS_RELOC_REMAP_WORDS);
        crate::difftest::fill(c, r, DNAS_RELOC_REMAP_WORDS, 0x10);
        // Both frames (RelocateAll's 0x30, RelocateSection's 0x2b0 below it).
        crate::difftest::fill(c, r, crate::difftest::SP - 0x2e0, 0x2e0 / 4);
    }

    /// Writes the NUL-terminated `bytes` at `at`.
    pub(crate) fn dnas_put_cstr(c: &mut Cpu, at: u32, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + bytes.len() as u32, 0);
    }

    /// For `store_io_machine`: bit n set makes fake call n (any fake, counted
    /// by `fakes::LOG_COUNT`) fail with its non-zero pointer `v0`, else the
    /// interface call returns 0.
    pub(crate) const STORE_IO_IFACE_FAIL_MASK: u32 = DATA + 0x3f0;

    /// The `miss` hook of `store_io_machine` for the checksum/cipher
    /// interface slots: the logging fake, returning 0 unless the fixture's
    /// fail mask names this call.
    pub(crate) fn store_io_iface_fake(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let fail = n < 32 && c.bus.read32(STORE_IO_IFACE_FAIL_MASK) >> n & 1 != 0;
        fakes::fake_call(c, target);
        if !fail {
            c.g[V0][0] = 0;
        }
    }

    /// The record-I/O tests' machine (checksum, cipher, card I/O steps):
    /// scrambled, with a random store state at `STORE_TEST_STATE` (`gp+0x2a30`)
    /// whose checksum and cipher interfaces are slot tables of fakes, and
    /// failures chosen per call by the bit mask `STORE_IO_IFACE_FAIL_MASK`
    /// (several calls may fail). `store_lifecycle_machine` is the other
    /// store machine: its own state block, flat interface tables, one
    /// failing call with a chosen error, and the hash result write; the
    /// tests were written against one layout each, so they are not merged.
    pub(crate) fn store_io_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = fakes::machine(c, seed, salt, 0);
        c.hooks.miss = store_io_iface_fake;
        c.bus.write32(GAME_GP + store::STATE_GP, STORE_TEST_STATE);
        fill(c, &mut r, STORE_TEST_STATE, 0x28);
        c.bus
            .write32(STORE_TEST_STATE + store::HASH_IFACE, STORE_TEST_HASH_IFACE);
        c.bus.write32(
            STORE_TEST_STATE + store::CIPHER_IFACE,
            STORE_TEST_CIPHER_IFACE,
        );
        for (i, slot) in [HASH_BEGIN, HASH_UPDATE, HASH_FINAL]
            .into_iter()
            .enumerate()
        {
            c.bus
                .write32(STORE_TEST_HASH_IFACE + slot, FAKE + 0x10 * i as u32);
        }
        for (i, slot) in [CIPHER_SET_KEY, CIPHER_ENCRYPT, CIPHER_DECRYPT, CIPHER_RESET]
            .into_iter()
            .enumerate()
        {
            c.bus.write32(
                STORE_TEST_CIPHER_IFACE + slot,
                FAKE + 0x40 + 0x10 * i as u32,
            );
        }
        c.bus.write32(STORE_IO_IFACE_FAIL_MASK, 0);
        r
    }

    /// For `store_lifecycle_machine`: interface calls made so far (heap
    /// fakes not counted), the index of the one that fails (with the error
    /// below), and the hash result `HASH_FINAL` hands back.
    pub(crate) const STORE_LIFECYCLE_IFACE_COUNT: u32 = STORE_FAKE_CTRL + 0x30;

    pub(crate) const STORE_LIFECYCLE_IFACE_FAIL_AT: u32 = STORE_FAKE_CTRL + 0x34;

    pub(crate) const STORE_LIFECYCLE_IFACE_ERROR: u32 = STORE_FAKE_CTRL + 0x38;

    /// The `miss` hook of `store_lifecycle_machine` for the fake interface
    /// slots: log the call, return 0 (or the error at the chosen interface
    /// call index), and for the hash's final call write the result pointer
    /// through its first argument.
    pub(crate) fn store_lifecycle_iface_fake(c: &mut Cpu, target: u32) {
        let out = c.g[A0][0] as u32;
        let n = c.bus.read32(STORE_LIFECYCLE_IFACE_COUNT);
        c.bus.write32(STORE_LIFECYCLE_IFACE_COUNT, n + 1);
        fakes::fake_call(c, target);
        c.g[V0][0] = if n == c.bus.read32(STORE_LIFECYCLE_IFACE_FAIL_AT) {
            c.bus.read32(STORE_LIFECYCLE_IFACE_ERROR) as u64
        } else {
            0
        };
        if target == STORE_FAKE_HASH_FINAL {
            let result = c.bus.read32(STORE_FAKE_HASH_RESULT);
            c.bus.write32(out, result);
        }
    }

    /// The lifecycle tests' machine (Init, Shutdown, Create..Close, Write,
    /// Read, VerifyRecord; see `store_io_machine` for the other one): fakes
    /// armed (unwinding at fake call `unwind_at`), interface tables, the state
    /// block at
    /// `STORE_LIFECYCLE_TEST_STATE` filled with junk and pointed to by `gp+0x2a30`,
    /// and the hash result `{len, &crc}` at `STORE_TEST_HASH_RESULT`.
    pub(crate) fn store_lifecycle_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        entry: u32,
        unwind_at: u32,
    ) -> Rng {
        let mut r = fakes::machine(c, seed, salt, 0);
        c.pc = entry;
        c.hooks.miss = store_lifecycle_iface_fake;
        c.bus.write32(fakes::LOG_UNWIND_AT, unwind_at);
        c.bus.write32(STORE_FAKE_CTRL, STORE_FAKE_MAGIC);
        c.bus.write32(STORE_LIFECYCLE_IFACE_FAIL_AT, u32::MAX);
        c.bus.write32(STORE_LIFECYCLE_IFACE_ERROR, 3 + seed % 5);
        c.bus
            .write32(STORE_FAKE_HASH_RESULT, STORE_TEST_HASH_RESULT);
        for i in 0..8 {
            c.bus
                .write32(STORE_FAKE_HASH_TABLE + 4 * i, FAKE + 0x100 + 0x10 * i);
            c.bus
                .write32(STORE_FAKE_CIPHER_TABLE + 4 * i, FAKE + 0x200 + 0x10 * i);
        }
        fill(c, &mut r, STORE_LIFECYCLE_TEST_STATE, 0xb0 / 4);
        c.bus
            .write32(GAME_GP + store::STATE_GP, STORE_LIFECYCLE_TEST_STATE);
        c.bus.write32(
            STORE_LIFECYCLE_TEST_STATE + store::HASH_IFACE,
            STORE_FAKE_HASH_TABLE,
        );
        c.bus
            .write32(STORE_LIFECYCLE_TEST_STATE + store::CIPHER_IFACE, 0);
        c.bus.write32(STORE_TEST_HASH_RESULT, 4);
        c.bus
            .write32(STORE_TEST_HASH_RESULT + 4, STORE_TEST_HASH_CRC);
        c.bus.write32(STORE_TEST_HASH_CRC, 0xc0de_0000 | seed);
        r
    }

    /// The back-edge head `DnasRel_FindSymbol` resumes at after an unwind.
    pub(crate) const DNAS_FIND_SYMBOL_HEAD: u32 = 0x003f_d7a0;

    /// Re-entries of the hand `DnasRel_FindSymbol` at its head (`drive`'s
    /// resume count pools both sides).
    pub(crate) static DNAS_FIND_SYMBOL_HAND_RESUMES: AtomicUsize = AtomicUsize::new(0);

    /// The hand body, counting its own re-entries at the head.
    pub(crate) fn dnas_rel_find_symbol_counted(c: &mut Cpu) {
        if c.pc == DNAS_FIND_SYMBOL_HEAD {
            DNAS_FIND_SYMBOL_HAND_RESUMES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        dnas_rel_find_symbol(c);
    }
    // ------------------------------------------------------------------
    // `SLogin_StepNop` (0x3f77f0), `SLoginMc_SyncResult_Cmd00` (0x3f87f8),
    // `SLoginMc_SyncResult_Cmd01` (0x3f8800).

    /// The no-op login step reports 0.
    #[test]
    fn slogin_step_nop_matches() {
        check(
            ee::s14::f_003f77f0__generated,
            slogin_step_nop,
            100,
            &|c, seed| {
                machine(c, seed, 0x3f77_f0, 0);
            },
        );
    }

    pub(crate) const SLOGIN_STATE: u32 = DATA + 0x1000;

    /// Step 0 echoes the sync result.
    #[test]
    fn slogin_mc_sync_result_cmd00_matches() {
        check(
            ee::s14::f_003f87f8__generated,
            slogin_mc_sync_result_cmd00,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f87_f8, 0);
                c.g[A0][0] =
                    [0, 1, 3, 0x10, 0x11, 0xffff_ffffu32.cast_signed()][seed as usize % 6] as u64;
            },
        );
    }

    /// Step 1 reports 0x12 while the step is not running, otherwise echoes
    /// the result.
    #[test]
    fn slogin_mc_sync_result_cmd01_matches() {
        check(
            ee::s14::f_003f8800__generated,
            slogin_mc_sync_result_cmd01,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f88_00, 0);
                c.g[A0][0] =
                    [0, 3, 0x10, 0x11, 7, 0xffff_ffffu32.cast_signed()][seed as usize % 6] as u64;
                c.bus
                    .write32(GAME_GP + SLOGIN_MC_SYNC_RESULT_STATE_GP, SLOGIN_STATE);
                c.bus
                    .write32(SLOGIN_STATE + SLOGIN_MC_SYNC_RESULT_PHASE, seed / 6 % 3);
            },
        );
    }

    /// Card-state block the sync-result fixtures point `gp + 0x1ae8` at.
    /// Word +0/+8 hold seed-varying fillers; the phase at +4 drives the gate.
    #[cfg(test)]
    pub(crate) const SLOGIN_MC_SYNC_RESULT_TEST_STATE: u32 = DATA + 0x1000;

    /// Point the card-state word at the scratch block and write `phase`,
    /// with seed-varying nonzero fillers around it.
    pub(crate) fn slogin_mc_sync_result_fixture(c: &mut Cpu, r: &mut Rng, phase: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.bus
            .write32(SLOGIN_MC_SYNC_RESULT_TEST_STATE, r.next_u32());
        c.bus.write32(
            SLOGIN_MC_SYNC_RESULT_TEST_STATE + SLOGIN_MC_SYNC_RESULT_PHASE,
            phase,
        );
        c.bus.write32(
            SLOGIN_MC_SYNC_RESULT_TEST_STATE + SLOGIN_MC_SYNC_RESULT_PHASE + 4,
            r.next_u32(),
        );
        c.bus.write32(
            GAME_GP + SLOGIN_MC_SYNC_RESULT_STATE_GP,
            SLOGIN_MC_SYNC_RESULT_TEST_STATE,
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd02_matches() {
        const PHASES: [u32; 7] = [0x2, 0x2, 0, 1, 3, 0x11, 0xffff_fff2];
        const RESULTS: [u64; 10] = [0, 4, 3, 9, 0x11, 0x10, 1, 2, 0x12, 0x7f];
        check(
            ee::s14::f_003f8838__generated,
            slogin_mc_sync_result_cmd02,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f88_3800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8838;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                let result = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
                c.g[A0][0] = if seed / 70 % 2 == 1 {
                    result | 0xffff_ffff_0000_0000
                } else {
                    result
                };
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd07_matches() {
        const PHASES: [u32; 8] = [0x1b, 0x1d, 0x1c, 0x7, 0xb, 0x18, 0, 0x5a];
        const RESULTS: [u64; 11] = [0, 3, 4, 9, 0xe, 0xd, 0x11, 0xa, 1, 0x10, 0x7f];
        check(
            ee::s14::f_003f8880__generated,
            slogin_mc_sync_result_cmd07,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f88_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8880;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                let result = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
                c.g[A0][0] = if seed / 88 % 2 == 1 {
                    result | 0xffff_ffff_0000_0000
                } else {
                    result
                };
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd08_matches() {
        const PHASES: [u32; 6] = [0x18, 0x8, 0xc, 0x10, 0, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0x11, 0x10, 1, 0x12];
        check(
            ee::s14::f_003f8908__generated,
            slogin_mc_sync_result_cmd08,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f89_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8908;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd09_matches() {
        const PHASES: [u32; 6] = [0x19, 0x9, 0xd, 0x11, 0, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0x11, 0x10, 2, 0x12];
        check(
            ee::s14::f_003f8968__generated,
            slogin_mc_sync_result_cmd09,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f89_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8968;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd1a_matches() {
        const PHASES: [u32; 5] = [0x1a, 0x1a, 0, 0x19, 0x5a];
        const RESULTS: [u64; 7] = [0, 3, 9, 0x11, 0x10, 1, 0x1a];
        check(
            ee::s14::f_003f89c8__generated,
            slogin_mc_sync_result_cmd1a,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f89_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_89c8;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd05_matches() {
        const PHASES: [u32; 5] = [0x5, 0x5, 0, 4, 0x5a];
        const RESULTS: [u64; 9] = [0, 3, 9, 0xe, 8, 0x11, 0x10, 1, 5];
        check(
            ee::s14::f_003f8a08__generated,
            slogin_mc_sync_result_cmd05,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8a_0800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8a08;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd06_matches() {
        const PHASES: [u32; 6] = [0x16, 0x6, 0xe, 0xa, 0, 0x5a];
        const RESULTS: [u64; 9] = [0, 3, 4, 9, 0xd, 0x11, 0x10, 1, 6];
        check(
            ee::s14::f_003f8a58__generated,
            slogin_mc_sync_result_cmd06,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8a_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8a58;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd13_matches() {
        const PHASES: [u32; 7] = [0x13, 0x14, 0x15, 0x16, 0x12, 0, 0x5a];
        const RESULTS: [u64; 11] = [0, 3, 4, 9, 0xd, 6, 5, 0x11, 0x10, 1, 0x13];
        check(
            ee::s14::f_003f8ac0__generated,
            slogin_mc_sync_result_cmd13,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8a_c000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8ac0;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd12_matches() {
        const PHASES: [u32; 5] = [0x12, 0x12, 0, 0x11, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0xd, 0x11, 0x10, 0x12];
        check(
            ee::s14::f_003f8b28__generated,
            slogin_mc_sync_result_cmd12,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_2800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8b28;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd03_matches() {
        const PHASES: [u32; 5] = [0x3, 0x3, 0, 2, 0x5a];
        const RESULTS: [u64; 7] = [0, 3, 0x11, 1, 0x10, 2, 0x12];
        check(
            ee::s14::f_003f8b78__generated,
            slogin_mc_sync_result_cmd03,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_7800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8b78;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd04_matches() {
        const PHASES: [u32; 5] = [0x4, 0x4, 0, 3, 0x5a];
        const RESULTS: [u64; 6] = [0, 3, 0x11, 1, 0x10, 4];
        check(
            ee::s14::f_003f8bb0__generated,
            slogin_mc_sync_result_cmd04,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8bb0;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_sync_result_cmd17_matches() {
        const PHASES: [u32; 5] = [0x17, 0x17, 0, 0x16, 0x5a];
        const RESULTS: [u64; 8] = [0, 3, 4, 9, 0x11, 0x10, 1, 0x17];
        check(
            ee::s14::f_003f8be8__generated,
            slogin_mc_sync_result_cmd17,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8b_e800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8be8;
                slogin_mc_sync_result_fixture(c, &mut r, PHASES[(seed as usize) % PHASES.len()]);
                c.g[A0][0] = RESULTS[((seed / PHASES.len() as u32) as usize) % RESULTS.len()];
            },
        );
    }

    #[test]
    fn slogin_mc_idle_step_matches() {
        check(
            ee::s14::f_003f8c30__generated,
            slogin_mc_idle_step,
            300,
            &|c, seed| {
                let mut r = Rng(0x3f8c_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_8c30;
                c.g[A0][0] = if seed % 5 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) | ((r.next_u32() as u64) << 32)
                };
            },
        );
    }
}
