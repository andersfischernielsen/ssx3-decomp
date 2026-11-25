//! `src/lib/eafile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ealib::seg_2dcae8::EAFS_MUTEX_LOCK;
pub(crate) use crate::game::ealib::seg_2dcae8::EAFS_MUTEX_UNLOCK;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d_d310,
        ea_file_release_request,
        "EAFile_ReleaseRequest",
    ),
    (0x003e_2490, eafile_str_icmp, "EAFile_StrICmp"),
    (0x003d_d648, ea_file_queue_read, "EAFile_QueueRead"),
    (0x003d_e670, ea_file_alloc_request, "EAFile_AllocRequest"),
    (0x003d_d4e8, ea_file_queue_open, "EAFile_QueueOpen"),
    (0x003d_d5a0, ea_file_queue_close, "EAFile_QueueClose"),
    (
        0x003d_e800,
        ea_file_alloc_file_handle,
        "EAFile_AllocFileHandle",
    ),
    (
        0x003d_e8c0,
        ea_file_free_file_handle,
        "EAFile_FreeFileHandle",
    ),
    (0x003d_dfa8, eafile_service_queue, "EAFile_ServiceQueue"),
    (0x003d_cbd8, ea_file_startup, "EAFile_Startup"),
    (0x003d_cc88, ea_file_init, "EAFile_Init"),
    (0x003d_cd98, ea_file_calc_mem_size, "EAFile_CalcMemSize"),
    (
        0x003d_ce90,
        ea_file_get_request_status,
        "EAFile_GetRequestStatus",
    ),
    (
        0x003d_cf10,
        ea_file_get_request_result,
        "EAFile_GetRequestResult",
    ),
    (
        0x003d_cf40,
        ea_file_get_request_bytes,
        "EAFile_GetRequestBytes",
    ),
    (
        0x003d_cfc0,
        ea_file_set_request_priority,
        "EAFile_SetRequestPriority",
    ),
    (0x003d_d148, ea_file_cancel_request, "EAFile_CancelRequest"),
    (0x003d_d1d8, ea_file_wait_request, "EAFile_WaitRequest"),
    (0x003d_d438, ea_file_queue_exists, "EAFile_QueueExists"),
    (0x003d_d720, ea_file_queue_write, "EAFile_QueueWrite"),
    (0x003d_d7e0, ea_file_queue_get_size, "EAFile_QueueGetSize"),
    (0x003d_d878, ea_file_queue_nop, "EAFile_QueueNop"),
    (
        0x003d_d8e8,
        ea_file_on_big_header_read,
        "EAFile_OnBigHeaderRead",
    ),
    (0x003d_da10, ea_file_on_big_opened, "EAFile_OnBigOpened"),
    (0x003d_dac0, ea_file_queue_mount_big, "EAFile_QueueMountBig"),
    (
        0x003d_dc00,
        ea_file_on_big_unmounted,
        "EAFile_OnBigUnmounted",
    ),
    (
        0x003d_dc30,
        ea_file_queue_unmount_big,
        "EAFile_QueueUnmountBig",
    ),
    (
        0x003d_ddf0,
        ea_file_call_with_priority,
        "EAFile_CallWithPriority",
    ),
    (0x003d_de50, ea_file_set_priority, "EAFile_SetPriority"),
    (0x003d_de78, ea_file_set_root_path, "EAFile_SetRootPath"),
    (0x003d_df80, ea_file_get_root_path, "EAFile_GetRootPath"),
    (0x003d_e4d0, ea_file_parse_big_path, "EAFile_ParseBigPath"),
    (
        0x003d_e668,
        ea_file_queue_signal_stub,
        "EAFile_QueueSignalStub",
    ),
    (0x003d_e928, ea_file_get_settings, "EAFile_GetSettings"),
    (0x003d_e9b8, ea_file_set_settings, "EAFile_SetSettings"),
    (
        0x003d_ea48,
        ea_file_on_chunk_transferred,
        "EAFile_OnChunkTransferred",
    ),
    (
        0x003d_eb50,
        ea_file_transfer_chunked,
        "EAFile_TransferChunked",
    ),
    (0x003d_ebf0, ea_file_open, "EAFile_Open"),
    (0x003d_ec60, ea_file_read, "EAFile_Read"),
    (0x003d_eca0, ea_file_close, "EAFile_Close"),
    (0x003d_ecf8, ea_file_get_size, "EAFile_GetSize"),
    (0x003d_ed50, ea_file_mount_big, "EAFile_MountBig"),
    (0x003d_edc0, ea_file_unmount_big, "EAFile_UnmountBig"),
    (0x003d_ee18, ea_file_exists, "EAFile_Exists"),
    (0x003e_1700, ea_file_exists_default, "EAFile_ExistsDefault"),
    (
        0x003e_1728,
        ea_file_get_file_size_worker,
        "EAFile_GetFileSizeWorker",
    ),
    (
        0x003e_1798,
        ea_file_get_file_size_ex,
        "EAFile_GetFileSizeEx",
    ),
    (0x003e_17d8, ea_file_get_file_size, "EAFile_GetFileSize"),
    (
        0x003e_1810,
        ea_file_load_file_worker,
        "EAFile_LoadFileWorker",
    ),
    (0x003e_18c8, ea_file_load_file_ex, "EAFile_LoadFileEx"),
    (0x003e_1908, ea_file_load_file, "EAFile_LoadFile"),
    (
        0x003e_1948,
        ea_file_load_file_get_size_worker,
        "EAFile_LoadFileGetSizeWorker",
    ),
    (
        0x003e_1a10,
        ea_file_load_file_get_size_ex,
        "EAFile_LoadFileGetSizeEx",
    ),
    (
        0x003e_1a70,
        ea_file_load_file_get_size,
        "EAFile_LoadFileGetSize",
    ),
    (
        0x003e_1ad0,
        ea_file_load_file_into_worker,
        "EAFile_LoadFileIntoWorker",
    ),
    (
        0x003e_1b68,
        ea_file_load_file_into_ex,
        "EAFile_LoadFileIntoEx",
    ),
    (0x003e_1bb8, ea_file_load_file_into, "EAFile_LoadFileInto"),
    (
        0x003e_1c00,
        ea_file_load_files_worker,
        "EAFile_LoadFilesWorker",
    ),
    (0x003e_1c98, ea_file_load_files_ex, "EAFile_LoadFilesEx"),
    (0x003e_1d00, ea_file_load_files, "EAFile_LoadFiles"),
    (
        0x003e_1d68,
        ea_file_load_big_header_worker,
        "EAFile_LoadBigHeaderWorker",
    ),
    (
        0x003e_1ec8,
        ea_file_load_big_header_ex,
        "EAFile_LoadBigHeaderEx",
    ),
    (0x003e_1f08, ea_file_load_big_header, "EAFile_LoadBigHeader"),
    (
        0x003e_2130,
        ea_file_get_unpacked_size,
        "EAFile_GetUnpackedSize",
    ),
    (
        0x003e_2168,
        ea_file_read_unpacked_size,
        "EAFile_ReadUnpackedSize",
    ),
    (
        0x003e_2190,
        ea_file_load_decompressed,
        "EAFile_LoadDecompressed",
    ),
    (
        0x003e_22f0,
        ea_file_load_decompressed_thunk,
        "EAFile_LoadDecompressedThunk",
    ),
    (
        0x003e_2310,
        ea_file_load_decompressed_into,
        "EAFile_LoadDecompressedInto",
    ),
    (
        0x003e_2470,
        ea_file_load_decompressed_into_thunk,
        "EAFile_LoadDecompressedIntoThunk",
    ),
    (0x003e_3020, ea_file_platform_init, "EAFile_PlatformInit"),
    (0x003e_3098, ea_file_resolve_path, "EAFile_ResolvePath"),
    (0x003e_3478, ea_file_platform_write, "EAFile_PlatformWrite"),
    (
        0x003e_3538,
        ea_file_platform_get_lsn,
        "EAFile_PlatformGetLsn",
    ),
    (0x003e_3588, ea_file_alarm_handler, "EAFile_AlarmHandler"),
    (0x003e_35b0, ea_file_delay_ticks, "EAFile_DelayTicks"),
    (0x003e_36f8, ea_file_hash_name, "EAFile_HashName"),
    (
        0x003e_39a8,
        ea_file_cd_build_hash_table,
        "EAFile_CdBuildHashTable",
    ),
    (0x003e_3ad8, ea_file_cd_signal, "EAFile_CdSignal"),
    (0x003e_4000, ea_file_cd_remount, "EAFile_CdRemount"),
    (0x003e_4508, ea_file_cd_lookup, "EAFile_CdLookup"),
    (0x003e_4648, ea_file_cd_read, "EAFile_CdRead"),
    (0x003e_48d8, ea_file_cd_get_entry, "EAFile_CdGetEntry"),
    (0x003e_4968, ea_file_cd_find_entry, "EAFile_CdFindEntry"),
    (0x0031_7f38, ea_file_path_exists, "EAFile_PathExists"),
    (
        0x0031_7f60,
        ea_file_get_path_size_checked,
        "EAFile_GetPathSizeChecked",
    ),
    (0x0031_7f98, ea_file_get_path_size, "EAFile_GetPathSize"),
    (0x0031_7fc0, ea_file_empty_stub, "EAFile_EmptyStub"),
    (
        0x0031_7fc8,
        ea_file_empty_stub_wrapper,
        "EAFile_EmptyStub_Wrapper",
    ),
    (0x003d_ec80, eafile_write, "EAFile_Write"),
    (0x003e_3618, ea_file_cd_reset, "EAFile_CdReset"),
    (0x003e_3758, ea_file_cd_scan_dir, "EAFile_CdScanDir"),
    (0x003e_3968, ea_file_cd_compare_hash, "EAFile_CdCompareHash"),
    (0x003e_3be0, ea_file_cd_read_thread, "EAFile_CdReadThread"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    use super::ee;
    use crate::call::Callee;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_FILE_HANDLE;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_REQUEST;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CALC_MEM_SIZE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CLOSE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_PLATFORM_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_SET;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ZERO;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MUTEX_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRLEN;

    // `EAMutex_Lock`/`Unlock` (0x3e5700/0x3e5760), under the names the
    // other EA units import from here.
    pub(crate) use crate::game::ealib::seg_2dcae8::EAFS_MUTEX_LOCK as EA_MUTEX_LOCK;
    pub(crate) use crate::game::ealib::seg_2dcae8::EAFS_MUTEX_UNLOCK as EA_MUTEX_UNLOCK;

    pub(crate) const EA_THREAD_IS_CURRENT: Callee = Callee {
        addr: 0x003e_5440,
        f: ee::s14::EAThread_IsCurrent,
    };

    pub(crate) const EA_THREAD_YIELD: Callee = Callee {
        addr: 0x003e_5398,
        f: ee::s14::EAThread_Yield,
    };

    pub(crate) const EA_TIMER_SERVICE_CALLBACKS: Callee = Callee {
        addr: 0x003e_5928,
        f: ee::s14::EATimer_ServiceCallbacks,
    };

    pub(crate) const EA_FILE_RELEASE_REQUEST: Callee = Callee {
        addr: 0x003d_d310,
        f: ee::s14::EAFile_ReleaseRequest,
    };

    pub(crate) const EA_FILE_QUEUE_CLOSE: Callee = Callee {
        addr: 0x003d_d5a0,
        f: ee::s14::EAFile_QueueClose,
    };

    pub(crate) const EA_FILE_QUEUE_SIGNAL_STUB: Callee = Callee {
        addr: 0x003d_e668,
        f: ee::s14::EAFile_QueueSignalStub,
    };

    pub(crate) const EA_FILE_ALLOC_REQUEST: Callee = Callee {
        addr: 0x003d_e670,
        f: ee::s14::EAFile_AllocRequest,
    };

    pub(crate) const EA_FILE_ALLOC_FILE_HANDLE: Callee = Callee {
        addr: 0x003d_e800,
        f: ee::s14::EAFile_AllocFileHandle,
    };

    pub(crate) const EA_FILE_SERVICE_QUEUE: Callee = Callee {
        addr: 0x003d_dfa8,
        f: ee::s14::EAFile_ServiceQueue,
    };

    pub(crate) const EA_FILE_QUEUE_READ: Callee = Callee {
        addr: 0x003d_d648,
        f: ee::s14::EAFile_QueueRead,
    };

    pub(crate) const EA_FILE_QUEUE_OPEN: Callee = Callee {
        addr: 0x003d_d4e8,
        f: ee::s14::EAFile_QueueOpen,
    };

    pub(crate) const EA_FILE_SET_REQUEST_CALLBACK: Callee = Callee {
        addr: 0x003d_cf70,
        f: ee::s14::EAFile_SetRequestCallback,
    };

    pub(crate) const BIG_FILE_GET_TYPE: Callee = Callee {
        addr: 0x003e_2500,
        f: ee::s14::BigFile_GetType,
    };

    pub(crate) const BIG_FILE_GET_HEADER_SIZE: Callee = Callee {
        addr: 0x003e_2580,
        f: ee::s14::BigFile_GetHeaderSize,
    };

    pub(crate) const EA_FILE_PLATFORM_OPEN: Callee = Callee {
        addr: 0x003e_3208,
        f: ee::s14::EAFile_PlatformOpen,
    };

    pub(crate) const EA_FILE_BIG_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2768,
        f: ee::s14::EAFile_BigFindEntry,
    };

    pub(crate) const EA_FILE_PLATFORM_CLOSE: Callee = Callee {
        addr: 0x003e_32f8,
        f: ee::s14::EAFile_PlatformClose,
    };

    pub(crate) const EA_FILE_PLATFORM_GET_SIZE: Callee = Callee {
        addr: 0x003e_34e8,
        f: ee::s14::EAFile_PlatformGetSize,
    };

    /// `strncmp` (newlib), called through the function table.
    pub(crate) const STRNCMP: u32 = 0x0041_6b18;

    /// `strncpy` (newlib), called through the function table.
    pub(crate) const STRNCPY: u32 = 0x0041_6ce0;

    /// `strchr` (newlib), called through the function table.
    pub(crate) const STRCHR: u32 = 0x0041_6404;

    /// `strcpy` (newlib), called through the function table.
    pub(crate) const STRCPY: u32 = 0x0041_66f4;

    /// `strcmp` (newlib), called through the function table.
    pub(crate) const STRCMP: u32 = 0x0041_65a8;

    /// `strcat` (newlib), called through the function table.
    pub(crate) const STRCAT: u32 = 0x0041_62d0;

    /// `qsort` (newlib), called through the function table.
    pub(crate) const QSORT: u32 = 0x0041_8ef8;

    /// `bsearch` (newlib), called through the function table.
    pub(crate) const BSEARCH: u32 = 0x0041_87e8;

    /// `SignalSema` (kernel call stub), called through the function table.
    pub(crate) const SIGNAL_SEMA: u32 = 0x0042_3dc0;

    /// `RefPack_Unpack`, called through the function table.
    pub(crate) const REFPACK_UNPACK: u32 = 0x003b_4740;

    /// `EAC_GetUnpackedSize`, called through the function table.
    pub(crate) const EAC_GET_UNPACKED_SIZE: u32 = 0x003b_4818;

    /// `SetAlarm` (kernel call stub), called through the function table.
    pub(crate) const SET_ALARM: u32 = 0x0042_3b20;

    /// `CreateSema` (kernel call stub), called through the function table.
    pub(crate) const CREATE_SEMA: u32 = 0x0042_3da0;

    /// `DeleteSema` (kernel call stub), called through the function table.
    pub(crate) const DELETE_SEMA: u32 = 0x0042_3db0;

    /// `iSignalSema` (kernel call stub), called through the function table.
    pub(crate) const I_SIGNAL_SEMA: u32 = 0x0042_3dd0;

    /// `WaitSema` (kernel call stub), called through the function table.
    pub(crate) const WAIT_SEMA: u32 = 0x0042_3de0;

    /// `sceLseek`, called through the function table.
    pub(crate) const SCE_LSEEK: u32 = 0x0042_8168;

    /// `sceWrite`, called through the function table.
    pub(crate) const SCE_WRITE: u32 = 0x0042_8600;

    pub(crate) const EA_MEM_COPY: Callee = Callee {
        addr: 0x003e_6574,
        f: ee::s14::EA_MemCopy,
    };

    pub(crate) const EA_FILE_WAIT_REQUEST: Callee = Callee {
        addr: 0x003d_d1d8,
        f: ee::s14::EAFile_WaitRequest,
    };

    pub(crate) const EA_FILE_GET_REQUEST_STATUS: Callee = Callee {
        addr: 0x003d_ce90,
        f: ee::s14::EAFile_GetRequestStatus,
    };

    pub(crate) const EA_FILE_QUEUE_GET_SIZE: Callee = Callee {
        addr: 0x003d_d7e0,
        f: ee::s14::EAFile_QueueGetSize,
    };

    pub(crate) const EA_FILE_QUEUE_MOUNT_BIG: Callee = Callee {
        addr: 0x003d_dac0,
        f: ee::s14::EAFile_QueueMountBig,
    };

    pub(crate) const EA_FILE_QUEUE_UNMOUNT_BIG: Callee = Callee {
        addr: 0x003d_dc30,
        f: ee::s14::EAFile_QueueUnmountBig,
    };

    pub(crate) const EA_FILE_QUEUE_EXISTS: Callee = Callee {
        addr: 0x003d_d438,
        f: ee::s14::EAFile_QueueExists,
    };

    pub(crate) const EA_FILE_TRANSFER_CHUNKED: Callee = Callee {
        addr: 0x003d_eb50,
        f: ee::s14::EAFile_TransferChunked,
    };

    pub(crate) const EA_FILE_PARSE_BIG_PATH: Callee = Callee {
        addr: 0x003d_e4d0,
        f: ee::s14::EAFile_ParseBigPath,
    };

    pub(crate) const EA_FILE_EMPTY_STUB: Callee = Callee {
        addr: 0x0031_7fc0,
        f: ee::s10::EAFile_EmptyStub,
    };

    pub(crate) const ASYNC_FILE_POOL_PUSH: Callee = Callee {
        addr: 0x003d_ee70,
        f: ee::s14::AsyncFile_PoolPush,
    };

    pub(crate) const ASYNC_FILE_FREE: Callee = Callee {
        addr: 0x003d_efc0,
        f: ee::s14::AsyncFile_Free,
    };

    pub(crate) const ASYNC_FILE_FINISH: Callee = Callee {
        addr: 0x003d_f028,
        f: ee::s14::AsyncFile_Finish,
    };

    pub(crate) const STREAM_FILE_VALIDATE: Callee = Callee {
        addr: 0x003d_fbd0,
        f: ee::s14::StreamFile_Validate,
    };

    pub(crate) const STREAM_FILE_LOOKUP_REQUEST: Callee = Callee {
        addr: 0x003d_fdd0,
        f: ee::s14::StreamFile_LookupRequest,
    };

    pub(crate) const STREAM_FILE_FREE_REQUEST: Callee = Callee {
        addr: 0x003d_fe18,
        f: ee::s14::StreamFile_FreeRequest,
    };

    pub(crate) const STREAM_FILE_RING_IN_RANGE: Callee = Callee {
        addr: 0x003d_fc08,
        f: ee::s14::StreamFile_RingInRange,
    };

    pub(crate) const STREAM_FILE_CONSUME_BYTES: Callee = Callee {
        addr: 0x003d_fc48,
        f: ee::s14::StreamFile_ConsumeBytes,
    };

    pub(crate) const STREAM_FILE_RELEASE_CHUNK: Callee = Callee {
        addr: 0x003e_13e8,
        f: ee::s14::StreamFile_ReleaseChunk,
    };

    pub(crate) const STREAM_FILE_GET_CHUNK: Callee = Callee {
        addr: 0x003e_12e0,
        f: ee::s14::StreamFile_GetChunk,
    };

    /// Intra-module call: the generated body calls the raw body, so both
    /// sides run the generated code here.
    pub(crate) const STREAM_FILE_CANCEL_REQUEST: Callee = Callee {
        addr: 0x003e_0e90,
        f: ee::s14::StreamFile_CancelRequest,
    };

    pub(crate) const EA_FILE_OPEN: Callee = Callee {
        addr: 0x003d_ebf0,
        f: ee::s14::EAFile_Open,
    };

    pub(crate) const EA_FILE_READ: Callee = Callee {
        addr: 0x003d_ec60,
        f: ee::s14::EAFile_Read,
    };

    pub(crate) const EA_FILE_GET_SIZE: Callee = Callee {
        addr: 0x003d_ecf8,
        f: ee::s14::EAFile_GetSize,
    };

    pub(crate) const EA_FILE_EXISTS: Callee = Callee {
        addr: 0x003d_ee18,
        f: ee::s14::EAFile_Exists,
    };

    pub(crate) const EA_FILE_CALL_WITH_PRIORITY: Callee = Callee {
        addr: 0x003d_ddf0,
        f: ee::s14::EAFile_CallWithPriority,
    };

    pub(crate) const EA_FILE_LOAD_FILE: Callee = Callee {
        addr: 0x003e_1908,
        f: ee::s14::EAFile_LoadFile,
    };

    pub(crate) const EA_FILE_LOAD_FILE_INTO: Callee = Callee {
        addr: 0x003e_1bb8,
        f: ee::s14::EAFile_LoadFileInto,
    };

    pub(crate) const EA_FILE_LOAD_FILE_GET_SIZE: Callee = Callee {
        addr: 0x003e_1a70,
        f: ee::s14::EAFile_LoadFileGetSize,
    };

    pub(crate) const BIG_FILE_GET_FOOTER: Callee = Callee {
        addr: 0x003e_2610,
        f: ee::s14::BigFile_GetFooter,
    };

    pub(crate) const BIG_FILE_FIND_ENTRY: Callee = Callee {
        addr: 0x003e_2ce0,
        f: ee::s14::BigFile_FindEntry,
    };

    /// Intra-module calls: the generated bodies call the raw bodies, so
    /// both sides run the generated code here.
    pub(crate) const EA_FILE_LOAD_DECOMPRESSED: Callee = Callee {
        addr: 0x003e_2190,
        f: ee::s14::EAFile_LoadDecompressed,
    };

    pub(crate) const EA_FILE_LOAD_DECOMPRESSED_INTO: Callee = Callee {
        addr: 0x003e_2310,
        f: ee::s14::EAFile_LoadDecompressedInto,
    };

    pub(crate) const BIG_FILE_GET_ENTRY_OFFSET: Callee = Callee {
        addr: 0x003e_2d00,
        f: ee::s14::BigFile_GetEntryOffset,
    };

    pub(crate) const BIG_FILE_GET_ENTRY_OFFSET_2: Callee = Callee {
        addr: 0x003e_2d30,
        f: ee::s14::BigFile_GetEntryOffset_2,
    };

    pub(crate) const BIG_FILE_GET_ENTRY_DATA: Callee = Callee {
        addr: 0x003e_2df0,
        f: ee::s14::BigFile_GetEntryData,
    };

    pub(crate) const EA_FILE_CD_GET_ENTRY: Callee = Callee {
        addr: 0x003e_48d8,
        f: ee::s14::EAFile_CdGetEntry,
    };

    pub(crate) const EA_FILE_HASH_NAME: Callee = Callee {
        addr: 0x003e_36f8,
        f: ee::s14::EAFile_HashName,
    };

    pub(crate) const EA_FILE_COMPLETE_REQUEST: Callee = Callee {
        addr: 0x003d_e420,
        f: ee::s14::EAFile_CompleteRequest,
    };

    pub(crate) const EA_TIMER_ADD_CALLBACK: Callee = Callee {
        addr: 0x003e_57f8,
        f: ee::s14::EATimer_AddCallback,
    };

    pub(crate) const EA_FILE_CANCEL_REQUEST: Callee = Callee {
        addr: 0x003d_d148,
        f: ee::s14::EAFile_CancelRequest,
    };

    pub(crate) const EA_FILE_SET_REQUEST_PRIORITY: Callee = Callee {
        addr: 0x003d_cfc0,
        f: ee::s14::EAFile_SetRequestPriority,
    };

    pub(crate) const EA_FILE_CD_MOUNT_DISC: Callee = Callee {
        addr: 0x003e_3d78,
        f: ee::s14::EAFile_CdMountDisc,
    };

    pub(crate) const EA_FILE_CD_LOOKUP: Callee = Callee {
        addr: 0x003e_4508,
        f: ee::s14::EAFile_CdLookup,
    };
}

// ---------------------------------------------------------------------------
// EAFile_ReleaseRequest: a release path chosen from flags in the request and
// the request-state table at `.rodata 0x495d40`.

/// `int EAFile_ReleaseRequest(EAFileRequest* req)` (0x3dd310): clear the
/// request and, depending on its state and channel type, free the associated
/// file handle first. The request is found through a 256-entry hash of its
/// address's top byte (table pointer at 0x51ad98, 0x30-byte entries).
pub fn ea_file_release_request(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x003d_d310 => {
                    c.cycles += 15;
                    let frame = Frame::open(c, 0x40);
                    c.g[A1][0] = 0x30;
                    let req = c.g[A0][0];
                    let sp0 = frame.at(c, 0);
                    Addr(sp0).sw(c, req);
                    c.g[A0][0] = 0x52_0000;
                    frame.save(c, S0, 0x30);
                    c.g[V0][0] = Addr(frame.at(c, 0x3)).lbu(c);
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_sub(0x6528)).lw(c);
                    c.mult(0, V0, V0, A1);
                    frame.save(c, S1, 0x20);
                    frame.save_ra(c, 0x10);
                    c.g[S1][0] = 0;
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                    pc = if c.g[V1][0] == 0 {
                        0x003d_d414
                    } else {
                        0x003d_d34c
                    };
                }
                0x003d_d34c => {
                    c.cycles += 2;
                    let positive = (c.g[V1][0] as i64) > 0;
                    c.g[V0][0] = 1;
                    pc = if positive { 0x003d_d368 } else { 0x003d_d354 };
                }
                0x003d_d354 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < -2i64) as u64;
                    pc = if c.g[V0][0] != 0 {
                        0x003d_d414
                    } else {
                        0x003d_d360
                    };
                }
                0x003d_d360 => {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    pc = 0x003d_d3f0;
                }
                0x003d_d368 => {
                    c.cycles += 2;
                    pc = if c.g[V1][0] != c.g[V0][0] {
                        0x003d_d414
                    } else {
                        0x003d_d370
                    };
                }
                0x003d_d370 => {
                    c.cycles += 7;
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[V0][0] = c.g[V0][0] & 0xf;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                    c.g[V1][0] = (c.g[A0][0] < 0x9u64) as u64;
                    let out = c.g[V1][0] == 0;
                    c.g[V0][0] = 0x49_0000;
                    pc = if out { 0x003d_d414 } else { 0x003d_d38c };
                }
                0x003d_d38c => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d40));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x003d_d3a4 => 0x003d_d3a4,
                        0x003d_d3ac => 0x003d_d3ac,
                        0x003d_d3b8 => 0x003d_d3b8,
                        0x003d_d3c0 => 0x003d_d3c0,
                        0x003d_d3c8 => 0x003d_d3c8,
                        0x003d_d3e8 => 0x003d_d3e8,
                        0x003d_d3f0 => 0x003d_d3f0,
                        _ => {
                            c.jump_out(target, 0x003d_d310, 0x003d_d434);
                            return Ok(());
                        }
                    };
                }
                0x003d_d3a4 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3ac => {
                    c.cycles += 3;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3b8 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
                    pc = 0x003d_d414;
                }
                0x003d_d3c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        let a = c.g[A0][0] as u32;
                        let v = c.g[V1][0];
                        Addr(a.wrapping_add(0x4)).sw(c, v);
                        pc = 0x003d_d414;
                    } else {
                        pc = 0x003d_d3e8;
                    }
                }
                0x003d_d3e8 => {
                    c.cycles += 2;
                    pc = 0x003d_d414;
                }
                0x003d_d3f0 => {
                    c.cycles += 5;
                    c.g[V1][0] = 0x2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 20);
                    c.g[A0][0] = c.g[V0][0] & 0xf;
                    let is2 = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x9;
                    pc = if is2 { 0x003d_d40c } else { 0x003d_d404 };
                }
                0x003d_d404 => {
                    c.cycles += 2;
                    pc = if c.g[A0][0] != c.g[V0][0] {
                        0x003d_d414
                    } else {
                        0x003d_d40c
                    };
                }
                0x003d_d40c => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
                    callee::EA_FILE_FREE_FILE_HANDLE.call(c, 0x003d_d414)?;
                    pc = 0x003d_d414;
                }
                0x003d_d414 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S0][0];
                    callee::EA_FILE_FREE_REQUEST.call(c, 0x003d_d41c)?;
                    c.g[V0][0] = c.g[S1][0];
                    let frame = Frame { size: 0x40 };
                    frame.restore(c, 16, 0x30);
                    frame.restore(c, 17, 0x20);
                    frame.restore_ra(c, 0x10);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_d310, 0x003d_d434);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// EAFile_StrICmp (0x3e2490, seg_2DCAE8.c)

/// The ctype table indexed with a signed byte (0x4a0000 - 0x637f); bit 0 is
/// "uppercase".
pub(crate) const EAF_CTYPE: u32 = 0x0049_9c81;

/// `int EAFile_StrICmp(const char* a, const char* b)` (0x3e2490): compares
/// two strings ignoring case (uppercase letters per the ctype table are
/// lowered by +0x20); returns the first non-zero difference, else 0.
pub fn eafile_str_icmp(c: &mut Cpu) {
    c.cycles += 7;
    c.g[T2][0] = c.g[A0][0];
    c.g[T1][0] = c.g[A1][0];
    c.g[T0][0] = Addr(c.g[T2][0] as u32).lb(c);
    c.g[V0][0] = 0x4a_0000;
    c.g[A3][0] = Addr(c.g[T1][0] as u32).lb(c);
    c.g[T3][0] = sx(EAF_CTYPE);
    loop {
        // 0x3e24bc: lowered characters and their difference.
        c.cycles += 13;
        let ca = c.g[T0][0] as u32;
        let cb = c.g[A3][0] as u32;
        let table = c.g[T3][0] as u32;
        c.g[A0][0] = sx(ca.wrapping_add(table));
        c.g[A2][0] = sx(ca.wrapping_add(0x20));
        c.g[A1][0] = sx(cb.wrapping_add(table));
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c) & 1;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lbu(c) & 1;
        c.g[A0][0] = sx(cb.wrapping_add(0x20));
        if c.g[V0][0] == 0 {
            c.g[A2][0] = c.g[T0][0];
        }
        if c.g[V1][0] == 0 {
            c.g[A0][0] = c.g[A3][0];
        }
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if c.g[V0][0] != 0 {
            break;
        }
        // 0x3e24f0: equal so far; stop at the end of `a`.
        c.cycles += 2;
        let more = c.g[T0][0] != 0;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_24b0);
        }
        // 0x3e24b0: next pair.
        c.cycles += 3;
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
        c.g[T0][0] = Addr(c.g[T2][0] as u32).lb(c);
        c.g[A3][0] = Addr(c.g[T1][0] as u32).lb(c);
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// EAFile_AllocRequest (0x3de670)
// ---------------------------------------------------------------------------

/// The `lui` halves the EA library's code loads before an offset: its data
/// (`0x450000`: settings, allocator hooks, timer words) and its BSS
/// (`0x520000`: the request pool and queue, CD, async-file and timer state).
pub(crate) const EA_DATA_HI: u64 = 0x0045_0000;

pub(crate) const EA_BSS_HI: u64 = 0x0052_0000;

/// The EAFile request pool (`0x519ab0`): slot count at +0, its mutex at
/// +0x14, the slot array (0x30 bytes each) at +0x28.
#[cfg(test)]
pub(crate) const EAFILE_POOL: u32 = 0x0051_9ab0;

pub(crate) const EAFILE_POOL_SLOTS: u32 = 0x28;

pub(crate) const EAFILE_POOL_MUTEX: u32 = 0x0051_9ac4;

pub(crate) const EAFILE_REQ_SIZE: u32 = 0x30;

/// Slot word 0: bits 0-19 serial, 20-23 state (0 = free, 1 = allocated);
/// byte 3 = the slot's index.
pub(crate) const EAFILE_REQ_INDEX: u32 = 0x3;

/// The next request serial (wraps from 0xfffff to 1).
pub(crate) const EAFILE_REQ_SERIAL: u32 = 0x0045_0c0c;

/// Loop heads: the slot scan, and the spin when the pool is exhausted.
pub(crate) const EAFILE_ALLOC_SCAN_LOOP: u32 = 0x003d_e6d0;

pub(crate) const EAFILE_ALLOC_EXHAUSTED_LOOP: u32 = 0x003d_e768;

pub(crate) const EAFILE_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const EAFILE_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `EAFileRequest* EAFile_AllocRequest(void)` (0x3de670): under the pool
/// mutex, takes the first free request slot, marks it allocated, stores its
/// index and the next serial (1..0xfffff) in it; returns `&slots[index]`.
/// With every slot busy the original spins forever (pc 0x3de768); so does
/// this port.
pub fn ea_file_alloc_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x0052_0000;
        c.g[S2][0] = sx(EAFILE_POOL_MUTEX);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = c.g[S2][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        EAFILE_MUTEX_LOCK.call(c, 0x003d_e69c)?;
        let count = Addr(r32(c, S2).wrapping_sub(0x14)).lw(c);
        c.g[V0][0] = count;
        c.g[S3][0] = 0x0052_0000;
        if (count as i64) > 0 {
            c.cycles += 10;
            c.g[T0][0] = 0xffff_ffff_ff0f_ffff; // clears the state bits
            c.g[A2][0] = 0xf_ffff; // serial mask
            c.g[A0][0] = sx(r32(c, S2).wrapping_sub(0x14));
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = 0x10_0000; // state 1
            c.g[T3][0] = 0xffff_ffff_fff0_0000; // clears the serial
            c.g[T4][0] = 1;
            c.g[A3][0] = 0;
            loop {
                c.cycles += 7;
                let pool = r32(c, A0);
                let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                let slot = sx(r32(c, A3).wrapping_add(slots as u32));
                c.g[A1][0] = slot;
                let word = Addr(slot as u32).lw(c);
                c.g[V1][0] = word;
                let state = sx((word as u32) >> 20) & 0xf;
                c.g[V0][0] = Addr(r32(c, T1).wrapping_sub(0x6550)).lw(c);
                if state == 0 {
                    // Free: claim it.
                    c.cycles += 19;
                    let claimed = (word & c.g[T0][0]) | c.g[T2][0];
                    c.g[V1][0] = claimed;
                    c.g[T0][0] = 0x0045_0000;
                    Addr(slot as u32).sw(c, claimed);
                    let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                    let at = sx(r32(c, A3).wrapping_add(slots as u32));
                    let index = c.g[S1][0];
                    Addr((at as u32).wrapping_add(EAFILE_REQ_INDEX)).sb(c, index);
                    let slots = Addr(pool.wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
                    let serial = Addr(EAFILE_REQ_SERIAL).lw(c);
                    let req = sx(r32(c, A3).wrapping_add(slots as u32));
                    c.g[A0][0] = req;
                    let head = Addr(req as u32).lw(c);
                    let serial_bits = serial & c.g[A2][0];
                    let next = sx((serial as u32).wrapping_add(1));
                    let head = (head & c.g[T3][0]) | serial_bits;
                    let wrap = (c.g[A2][0] as i64) < (next as i64);
                    c.g[A1][0] = serial_bits;
                    c.g[V1][0] = next;
                    c.g[A2][0] = wrap as u64;
                    c.g[V0][0] = head;
                    Addr(req as u32).sw(c, head);
                    Addr(EAFILE_REQ_SERIAL).sw(c, next);
                    if wrap {
                        c.cycles += 2;
                        let one = c.g[T4][0];
                        Addr(EAFILE_REQ_SERIAL).sw(c, one);
                    }
                    break;
                }
                // Busy: next slot.
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let more = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(EAFILE_REQ_SIZE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, EAFILE_ALLOC_SCAN_LOOP);
                }
            }
        }
        c.cycles += 6;
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x653c));
        c.g[A0][0] = c.g[S0][0];
        EAFILE_MUTEX_UNLOCK.call(c, 0x003d_e75c)?;
        let count = Addr(r32(c, S0).wrapping_sub(0x14)).lw(c);
        c.g[V0][0] = EAFILE_REQ_SIZE as u64;
        if c.g[S1][0] == count {
            // No free slot: the original spins here forever.
            loop {
                c.cycles += 7;
                if event_due(c) {
                    deliver_event_here(c, EAFILE_ALLOC_EXHAUSTED_LOOP);
                }
            }
        }
        c.cycles += 11;
        c.g[A0][0] = sx(r32(c, S3).wrapping_sub(0x6550));
        let (index, stride) = (r32(c, S1), r32(c, V0));
        let offset = abi::mult(c, index, stride);
        c.g[V0][0] = sx(offset);
        let slots = Addr(r32(c, A0).wrapping_add(EAFILE_POOL_SLOTS)).lw(c);
        c.g[V1][0] = slots;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = sx((slots as u32).wrapping_add(offset));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_QueueRead.

/// `EAFileRequest` fields.
pub(crate) const EAFILE_QR_STATUS: u32 = 0xc;

pub(crate) const EAFILE_QR_CALLBACK_ARG: u32 = 0x10;

pub(crate) const EAFILE_QR_CALLBACK: u32 = 0x14;

pub(crate) const EAFILE_QR_OFFSET: u32 = 0x18;

pub(crate) const EAFILE_QR_SIZE: u32 = 0x1c;

pub(crate) const EAFILE_QR_BUFFER: u32 = 0x20;

pub(crate) const EAFILE_QR_FILE: u32 = 0x24;

/// `EAFile::size`.
pub(crate) const EAFILE_QR_FILE_SIZE: u32 = 0x4;

/// `int EAFile_QueueRead(EAFile* f, int offset, void* buf, int size, int
/// cbArg, void* cb)` (0x3dd648): allocate a request (`EAFile_AllocRequest`),
/// make it a read (command 4 in bits 20-23 of word 0), store the callback;
/// a null file marks the request failed (status 6, `EAFile_QueueSignalStub`).
/// Clamp the size to the end of the file, fill the request and hand it to
/// `EAFile_ServiceQueue`; returns the request's word 0 (its handle).
pub fn ea_file_queue_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[T1][0]; // callback
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[T0][0]; // callback arg
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A3][0]; // size
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0]; // file
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A1][0]; // offset
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[A2][0]; // buffer
        Callee {
            addr: 0x003d_e670,
            f: ee::s14::EAFile_AllocRequest,
        }
        .call(c, 0x003d_d688)?;
        c.g[S2][0] = c.g[V0][0];
        let req = c.g[S2][0] as u32;
        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
        c.g[V0][0] = Addr(req).lw(c);
        c.g[A0][0] = 0x0040_0000;
        c.sw(req.wrapping_add(EAFILE_QR_CALLBACK), c.g[S0][0]);
        c.g[V0][0] &= c.g[V1][0];
        c.sw(req.wrapping_add(EAFILE_QR_CALLBACK_ARG), c.g[S1][0]);
        c.g[V0][0] |= c.g[A0][0]; // command: read
        c.sw(req, c.g[V0][0]);
        if c.g[S4][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = 6;
            c.g[A0][0] = c.g[S2][0];
            c.sw(req.wrapping_add(EAFILE_QR_STATUS), c.g[V0][0]);
            Callee {
                addr: 0x003d_e668,
                f: ee::s14::EAFile_QueueSignalStub,
            }
            .call(c, 0x003d_d6c4)?;
        }

        c.cycles += 22;
        let req = c.g[S2][0] as u32;
        c.sw(req.wrapping_add(EAFILE_QR_FILE), c.g[S4][0]);
        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S3][0] as u32)); // end
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(EAFILE_QR_FILE_SIZE)).lw(c);
        c.sw(req.wrapping_add(EAFILE_QR_BUFFER), c.g[S6][0]);
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
        c.sw(req.wrapping_add(EAFILE_QR_OFFSET), c.g[S5][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S5][0] as u32));
        if c.g[V1][0] != 0 {
            c.g[S3][0] = c.g[V0][0]; // clamp to the file end
        }
        c.sw(req.wrapping_add(EAFILE_QR_SIZE), c.g[S3][0]);
        Callee {
            addr: 0x003d_dfa8,
            f: ee::s14::EAFile_ServiceQueue,
        }
        .call(c, 0x003d_d6f0)?;
        c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
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
// 0x003de800 EAFile_AllocFileHandle

/// The file-handle manager (`D_00519AB0`): `+4` the handle count, `+0x14`
/// its mutex (0x519ac4), `+0x2c` the handle array. Handles are 0x110 bytes;
/// their first word is the in-use flag.
#[cfg(test)]
pub(crate) const OC325_FILE_MANAGER: u32 = 0x0051_9ab0;

pub(crate) const OC325_FILE_COUNT: u32 = 0x4;

pub(crate) const OC325_FILE_MUTEX: u32 = 0x14;

pub(crate) const OC325_FILE_ARRAY: u32 = 0x2c;

pub(crate) const OC325_FILE_HANDLE: u32 = 0x110;

/// The manager's own mutex (`D_00519AC4`).
#[cfg(test)]
pub(crate) const OC325_FILE_MANAGER_MUTEX: u32 = 0x0051_9ac4;

pub(crate) const OC325_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const OC325_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

/// `EAFileHandle* EAFile_AllocFileHandle(void)` (0x3de800): take the file
/// manager's lock, claim the first free handle (marking it in use) and
/// return it; spin forever when every handle is taken.
pub fn ea_file_alloc_file_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[A0][0] = 0x0052_0000;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x653c));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        OC325_MUTEX_LOCK.call(c, 0x003d_e824)?;
        c.g[S2][0] = 0x0052_0000;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_COUNT)).lw(c);
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let all_taken = c.g[V0][0] == 0;
            c.g[V0][0] = OC325_FILE_HANDLE as u64;
            if all_taken {
                break;
            }
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_ARRAY)).lw(c);
            c.mult(0, V0, S1, V0); // v0 = 0x110 * i
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if c.g[V1][0] != 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, 0x003d_e830);
                }
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                break;
            }
        }
        c.cycles += 5;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(OC325_FILE_MUTEX));
        OC325_MUTEX_UNLOCK.call(c, 0x003d_e868)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(OC325_FILE_COUNT)).lw(c);
        let no_free = c.g[S1][0] != c.g[V0][0];
        c.g[V0][0] = OC325_FILE_HANDLE as u64;
        if !no_free {
            // Every handle is in use: the original spins here forever.
            loop {
                c.cycles += 7;
                if event_due(c) {
                    deliver_event_here(c, 0x003d_e878);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.mult(0, V0, S1, V0); // v0 = 0x110 * i
        c.g[V1][0] = Addr(r32(c, A0).wrapping_add(OC325_FILE_ARRAY)).lw(c);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_QueueOpen (0x3dd4e8)

/// `EAFileRequest` words QueueOpen fills: arg2 at `+0x10`, arg3 at `+0x14`,
/// arg1 at `+0x18`; its status at `+0xc` and its file handle at `+0x24`.
pub(crate) const OC326_QO_ARG2: u32 = 0x10;

pub(crate) const OC326_QO_ARG3: u32 = 0x14;

pub(crate) const OC326_QO_ARG1: u32 = 0x18;

pub(crate) const OC326_QO_STATUS: u32 = 0xc;

pub(crate) const OC326_QO_HANDLE: u32 = 0x24;

/// Where the file name goes inside the file handle.
pub(crate) const OC326_QO_HANDLE_NAME: u32 = 0x10;

/// The name buffer's length.
pub(crate) const OC326_QO_NAME_LEN: u64 = 0x100;

/// Command bits in the request's word 0: the mask that clears the state
/// field (bits 20-23) and the "open" state (2 << 20).
pub(crate) const OC326_QO_CMD_MASK: u32 = 0xff0f_ffff;

pub(crate) const OC326_QO_CMD_OPEN: u64 = 0x0020_0000;

/// Request status 2: failed / signalled.
pub(crate) const OC326_QO_STATUS_FAILED: u64 = 2;

pub(crate) const OC326_EA_ALLOC_REQUEST: Callee = Callee {
    addr: 0x003d_e670,
    f: ee::s14::EAFile_AllocRequest,
};

pub(crate) const OC326_EA_ALLOC_FILE_HANDLE: Callee = Callee {
    addr: 0x003d_e800,
    f: ee::s14::EAFile_AllocFileHandle,
};

pub(crate) const OC326_EA_QUEUE_SIGNAL_STUB: Callee = Callee {
    addr: 0x003d_e668,
    f: ee::s14::EAFile_QueueSignalStub,
};

pub(crate) const OC326_EA_SERVICE_QUEUE: Callee = Callee {
    addr: 0x003d_dfa8,
    f: ee::s14::EAFile_ServiceQueue,
};

/// `strncpy` (0x416ce0), through the function table.
pub(crate) const OC326_STRNCPY: u32 = 0x0041_6ce0;

/// `int EAFile_QueueOpen(const char* name, int a1, int a2, int a3)`
/// (0x3dd4e8): take a request (`EAFile_AllocRequest`), make it an open (the
/// command bits of word 0), store the three arguments, take a file handle for
/// it (`EAFile_AllocFileHandle`, word `+0x24`); a null handle marks the
/// request failed (status 2, signal stub). Copy the name (0x100 bytes) into
/// the handle at `+0x10`, hand the request to `EAFile_ServiceQueue`, and
/// return the request's word 0 (its handle).
pub fn ea_file_queue_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A0][0]; // name
        OC326_EA_ALLOC_REQUEST.call(c, 0x003d_d518)?;
        c.g[S3][0] = c.g[V0][0]; // request
        let req = r32(c, S3);
        c.g[A0][0] = OC326_QO_CMD_OPEN;
        c.g[V1][0] = Addr(req).lw(c);
        c.g[V0][0] = sx(OC326_QO_CMD_MASK); // 0xffff_ffff_ff0f_ffff
        c.g[V0][0] |= 0xffff;
        c.sw(req.wrapping_add(OC326_QO_ARG3), c.g[S0][0]);
        c.g[V1][0] &= c.g[V0][0];
        c.sw(req.wrapping_add(OC326_QO_ARG1), c.g[S1][0]);
        c.g[V1][0] |= c.g[A0][0];
        c.sw(req.wrapping_add(OC326_QO_ARG2), c.g[S2][0]);
        c.sw(req, c.g[V1][0]);
        OC326_EA_ALLOC_FILE_HANDLE.call(c, 0x003d_d548)?;
        let has_handle = c.g[V0][0] != 0;
        c.sw(req.wrapping_add(OC326_QO_HANDLE), c.g[V0][0]);
        if !has_handle {
            c.cycles += 4;
            c.g[V0][0] = OC326_QO_STATUS_FAILED;
            c.g[A0][0] = c.g[S3][0];
            c.sw(req.wrapping_add(OC326_QO_STATUS), c.g[V0][0]);
            OC326_EA_QUEUE_SIGNAL_STUB.call(c, 0x003d_d560)?;
        }
        c.cycles += 16;
        // strncpy(handle->name, name, 0x100)
        c.g[A0][0] = Addr(r32(c, S3)).off(OC326_QO_HANDLE).lw(c);
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = OC326_QO_NAME_LEN;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(OC326_QO_HANDLE_NAME));
        call_ptr(c, OC326_STRNCPY, 0x003d_d574)?;
        c.g[A0][0] = c.g[S3][0];
        OC326_EA_SERVICE_QUEUE.call(c, 0x003d_d57c)?;
        c.g[V0][0] = Addr(r32(c, S3)).lw(c);
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

// ---------------------------------------------------------------------------
// EAFile_QueueClose (0x3dd5a0)

/// The EAFile request pool (`0x519ab0`) and its request-list head (`D_00519AE4`,
/// reached as `0x520000 - 0x651c`).
#[cfg(test)]
pub(crate) const EAFILE_QC_LIST_HEAD: u32 = 0x0051_9ae4;

/// A request's words: the flags/state (+0), the queue link (+0xc), the
/// priority (+0x10), the second argument (+0x14) and the node id (+0x24).
pub(crate) const EAFILE_QC_LINK: u32 = 0xc;

pub(crate) const EAFILE_QC_PRIORITY: u32 = 0x10;

pub(crate) const EAFILE_QC_ARG2: u32 = 0x14;

pub(crate) const EAFILE_QC_ID: u32 = 0x24;

/// A request-list node: its id (+0xc) and next link (+0x14).
pub(crate) const EAFILE_QC_LIST_ID: u32 = 0xc;

pub(crate) const EAFILE_QC_LIST_NEXT: u32 = 0x14;

/// `int EAFile_QueueClose(int id, int arg1, int arg2)` (0x3dd5a0): take a
/// request from the pool, set its state to 3 ("close"), store the arguments
/// and the node id, and mark it signalled when a request-list node with that
/// id exists; then hand it to the service queue and return its state word.
/// Frame 0x50 (s0 = arg1, s1 = arg2, s2 = request, s3 = id).
pub fn ea_file_queue_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        Callee {
            addr: 0x003d_e670,
            f: ee::s14::EAFile_AllocRequest,
        }
        .call(c, 0x003d_d5c8)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0052_0000;
        c.g[V1][0] = Addr(r32(c, S2)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ff0f_ffff; // clear the state bits
        c.g[A0][0] = 0x30_0000; // state 3
        c.g[V1][0] &= c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, A1).wrapping_sub(0x651c)).lw(c);
        c.g[V1][0] |= c.g[A0][0];
        c.g[A2][0] = 3;
        Addr(r32(c, S2)).sw(c, c.g[V1][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_PRIORITY)).sw(c, c.g[S0][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_ARG2)).sw(c, c.g[S1][0]);
        Addr(r32(c, S2).wrapping_add(EAFILE_QC_ID)).sw(c, c.g[S3][0]);
        loop {
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A1).wrapping_add(EAFILE_QC_LIST_ID)).lw(c);
                if c.g[V0][0] != c.g[S3][0] {
                    c.g[A1][0] = Addr(r32(c, A1).wrapping_add(EAFILE_QC_LIST_NEXT)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_d600);
                    }
                    continue;
                }
                c.cycles += 3;
                Addr(r32(c, S2).wrapping_add(EAFILE_QC_LINK)).sw(c, c.g[A2][0]);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x003d_e668,
                    f: ee::s14::EAFile_QueueSignalStub,
                }
                .call(c, 0x003d_d620)?;
            }
            break;
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x003d_dfa8,
            f: ee::s14::EAFile_ServiceQueue,
        }
        .call(c, 0x003d_d628)?;
        c.g[V0][0] = Addr(r32(c, S2)).lw(c);
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
// EAFile_FreeFileHandle (0x3de8c0)

#[cfg(test)]
/// The EAFile file-handle pool mutex (`0x520000 - 0x653c`).
pub(crate) const EAF_HANDLE_MUTEX: u32 = 0x0051_9ac4;

/// Size of one EAFile file-handle record.
pub(crate) const EAF_HANDLE_SIZE: u64 = 0x110;

pub(crate) const EAF_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};

pub(crate) const EAF_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};

pub(crate) const EAF_MEMSET: Callee = Callee {
    addr: 0x003e_6448,
    f: ee::s14::EA_MemSet,
};

/// `void EAFile_FreeFileHandle(EAFileHandle* handle)` (0x3de8c0): under the
/// file-handle pool mutex, clear the 0x110-byte handle record (a free handle
/// is an all-zero record).
pub fn ea_file_free_file_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x0052_0000;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x653c));
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        EAF_MUTEX_LOCK.call(c, 0x003d_e8e4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = EAF_HANDLE_SIZE;
        EAF_MEMSET.call(c, 0x003d_e8f4)?;
        c.g[A0][0] = c.g[S0][0];
        EAF_MUTEX_UNLOCK.call(c, 0x003d_e8fc)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EAFile_ServiceQueue
// ---------------------------------------------------------------------------

/// The EA file system state (`0x519ab0`): `+0x8` highest priority served,
/// `+0xc` queued count, `+0x10` paused, `+0x14` mutex, `+0x24` the request
/// being served, `+0x30` the queue head, `+0x34` the open big files.
pub(crate) const EAFS_STATE: u32 = 0x0051_9ab0;

/// The command jump table (`.rodata`, commands 2..10).
pub(crate) const EAFS_JUMP_TABLE: u32 = 0x0049_5d80;

/// Request fields: `+0` command (bits 20-23), `+4` cancelled, `+0xc`
/// result, `+0x10` priority, `+0x18` flags / offset / result, `+0x1c`
/// size, `+0x20` buffer, `+0x24` file handle, `+0x2c` next.
pub(crate) const EAFS_REQ_PRIORITY: u32 = 0x10;

pub(crate) const EAFS_REQ_NEXT: u32 = 0x2c;

pub(crate) const EAFS_REQ_HANDLE: u32 = 0x24;

pub(crate) const EAFS_COMPLETE_REQUEST: Callee = Callee {
    addr: 0x003d_e420,
    f: ee::s14::f_003de420,
};

pub(crate) const EAFS_PLATFORM_OPEN: Callee = Callee {
    addr: 0x003e_3208,
    f: ee::s14::f_003e3208,
};

pub(crate) const EAFS_PLATFORM_CLOSE: Callee = Callee {
    addr: 0x003e_32f8,
    f: ee::s14::f_003e32f8,
};

pub(crate) const EAFS_PLATFORM_GET_SIZE: Callee = Callee {
    addr: 0x003e_34e8,
    f: ee::s14::f_003e34e8,
};

pub(crate) const EAFS_PLATFORM_GET_LSN: Callee = Callee {
    addr: 0x003e_3538,
    f: ee::s14::f_003e3538,
};

pub(crate) const EAFS_BIG_FIND_ENTRY: Callee = Callee {
    addr: 0x003e_2768,
    f: ee::s14::f_003e2768,
};

pub(crate) const EAFS_FREE_FILE_HANDLE: Callee = Callee {
    addr: 0x003d_e8c0,
    f: ee::s14::f_003de8c0,
};

pub(crate) const EAFS_PLATFORM_READ: Callee = Callee {
    addr: 0x003e_3350,
    f: ee::s14::f_003e3350,
};

pub(crate) const EAFS_PLATFORM_WRITE: Callee = Callee {
    addr: 0x003e_3478,
    f: ee::s14::f_003e3478,
};

/// libc through the table.
pub(crate) const EAFS_STRCHR: u32 = 0x0041_6404;

pub(crate) const EAFS_STRNCPY: u32 = 0x0041_6ce0;

pub(crate) const EAFS_STRCPY: u32 = 0x0041_66f4;

pub(crate) const EAFS_STRCMP: u32 = 0x0041_65a8;

/// `void EAFile_ServiceQueue(Request* req)` (0x3ddfa8, `iFILESYS_ExecCommand`):
/// queues `req` (if any) by priority, then, when nothing is being served
/// and the queue is not paused, takes the head (if its priority is allowed)
/// and executes it: open / exists (2, 8: a plain path, or `big|name` /
/// `|name` looked up in the open big files), close (3), read (4), write
/// (5), size (6), the others just complete.
pub fn eafile_service_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x270);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S0, 0x260);
        frame.save(c, S1, 0x250);
        c.g[S0][0] = sx(0x0051_9ac4);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x220);
        c.g[S4][0] = 0x52_0000;
        frame.save(c, S2, 0x240);
        frame.save(c, S3, 0x230);
        frame.save_ra(c, 0x210);
        c.g[A0][0] = c.g[S0][0];
        EAFS_MUTEX_LOCK.call(c, 0x003d_dfdc)?;
        let st = Addr(EAFS_STATE);
        c.g[A0][0] = 0;
        if c.g[S1][0] != 0 {
            // Insert behind every request of the same or higher priority.
            let req = Addr(c.g[S1][0] as u32);
            c.cycles += 3;
            let head = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
            c.g[V0][0] = head;
            req.off(EAFS_REQ_NEXT).sw(c, head);
            if head != 0 {
                c.cycles += 5;
                let hp = Addr(head as u32).off(EAFS_REQ_PRIORITY).lw(c);
                let prio = req.off(EAFS_REQ_PRIORITY).lw(c);
                c.g[V1][0] = prio;
                c.g[V0][0] = ((prio as i64) < (hp as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.g[A0][0] = req.off(EAFS_REQ_NEXT).lw(c);
                    loop {
                        c.cycles += 3;
                        let next = Addr(c.g[A0][0] as u32).off(EAFS_REQ_NEXT).lw(c);
                        c.g[V0][0] = next;
                        req.off(EAFS_REQ_NEXT).sw(c, next);
                        if next == 0 {
                            break;
                        }
                        c.cycles += 4;
                        let np = Addr(next as u32).off(EAFS_REQ_PRIORITY).lw(c);
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (np as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            break;
                        }
                        c.g[A0][0] = req.off(EAFS_REQ_NEXT).lw(c);
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_e008);
                        }
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = sx(EAFS_STATE);
            let prev = c.g[A0][0];
            if prev == 0 {
                c.cycles += 1;
                let v = c.g[S1][0];
                st.off(0x30).sw(c, v);
            } else {
                c.cycles += 2;
                let s1 = c.g[S1][0];
                Addr(prev as u32).off(EAFS_REQ_NEXT).sw(c, s1);
            }
            c.cycles += 4;
            c.g[V1][0] = sx(EAFS_STATE);
            let n = st.off(0xc).lw(c);
            c.g[V0][0] = sx((n as u32).wrapping_add(1));
            let v = c.g[V0][0];
            st.off(0xc).sw(c, v);
        }
        'out: {
            // Serve the head unless one is being served or the queue is paused.
            c.cycles += 4;
            c.g[A0][0] = sx(EAFS_STATE);
            let busy = st.off(0x24).lw(c);
            c.g[V0][0] = busy;
            let mut idle = false;
            if busy == 0 {
                c.cycles += 3;
                let paused = st.off(0x10).lw(c);
                c.g[V0][0] = paused;
                if paused == 0 {
                    c.g[S1][0] = st.off(0x30).lw(c);
                    idle = true;
                }
            }
            if !idle {
                c.cycles += 4;
                c.g[A0][0] = sx(EAFS_STATE + 0x14);
                EAFS_MUTEX_UNLOCK.call(c, 0x003d_e06c)?;
                frame.restore(c, S0, 0x260);
                break 'out;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                c.g[S1][0] = 0;
            } else {
                c.cycles += 5;
                let req = Addr(c.g[S1][0] as u32);
                c.g[V1][0] = req.off(EAFS_REQ_PRIORITY).lw(c);
                let max = st.off(0x8).lw(c);
                c.g[V0][0] = ((max as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = 0;
                } else {
                    c.cycles += 5;
                    let n = st.off(0xc).lw(c);
                    c.g[V1][0] = req.off(EAFS_REQ_NEXT).lw(c);
                    c.g[V0][0] = sx((n as u32).wrapping_sub(1));
                    let v = c.g[V0][0];
                    st.off(0xc).sw(c, v);
                    let v = c.g[V1][0];
                    st.off(0x30).sw(c, v);
                }
            }
            c.cycles += 6;
            c.g[V0][0] = sx(EAFS_STATE);
            c.g[A0][0] = sx(EAFS_STATE + 0x14);
            let v = c.g[S1][0];
            st.off(0x24).sw(c, v);
            EAFS_MUTEX_UNLOCK.call(c, 0x003d_e0b4)?;
            frame.restore(c, S0, 0x260);
            if c.g[S1][0] == 0 {
                break 'out;
            }
            let req = Addr(c.g[S1][0] as u32);
            // A cancelled request completes with 0.
            c.cycles += 3;
            c.g[V0][0] = req.off(4).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = 0;
                EAFS_COMPLETE_REQUEST.call(c, 0x003d_e0d0)?;
                frame.restore(c, S0, 0x260);
                break 'out;
            }
            c.g[V0][0] = req.lw(c);
            c.cycles += 6;
            let cmd = (c.g[V0][0] as u32 >> 20) & 0xf;
            c.g[V0][0] = cmd as u64;
            c.g[A0][0] = sx(cmd.wrapping_sub(2));
            c.g[V1][0] = (c.g[A0][0] < 9) as u64;
            c.g[V0][0] = 0x49_0000;
            if c.g[V1][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x260);
                break 'out;
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx(EAFS_JUMP_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(EAFS_JUMP_TABLE));
            let target = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = target;
            match target as u32 {
                0x003d_e108 => eafile_open_command(c, frame)?,
                0x003d_e308 => {
                    // Close: the platform file unless it is inside a big file.
                    c.cycles += 4;
                    c.g[V0][0] = 1;
                    let fh = req.off(EAFS_REQ_HANDLE).lw(c);
                    c.g[A0][0] = fh;
                    req.off(0xc).sw(c, 1);
                    if fh != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(fh as u32).off(8).lw(c);
                        if c.g[V0][0] != 0 {
                            req.off(0xc).sw(c, 0);
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = Addr(fh as u32).lw(c);
                            EAFS_PLATFORM_CLOSE.call(c, 0x003d_e32c)?;
                            let v0 = c.g[V0][0];
                            req.off(0xc).sw(c, v0);
                        }
                        c.cycles += 3;
                        c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                        EAFS_FREE_FILE_HANDLE.call(c, 0x003d_e338)?;
                        req.off(EAFS_REQ_HANDLE).sw(c, 0);
                    }
                    c.cycles += 5;
                    let res = req.off(0xc).lw(c);
                    c.g[A0][0] = (res < 1) as u64;
                    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e348)?;
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e350 => {
                    // Read: from the file, or at the entry's offset inside
                    // its big file.
                    c.cycles += 3;
                    c.g[A3][0] = req.off(0x1c).lw(c);
                    if (c.g[A3][0] as i64) <= 0 {
                        c.cycles += 4;
                        c.g[A0][0] = 1;
                        EAFS_COMPLETE_REQUEST.call(c, 0x003d_e3ac)?;
                    } else {
                        c.cycles += 4;
                        let fh = req.off(EAFS_REQ_HANDLE).lw(c);
                        c.g[V0][0] = fh;
                        c.g[A0][0] = Addr(fh as u32).off(8).lw(c);
                        if c.g[A0][0] == 0 {
                            c.g[A0][0] = Addr(fh as u32).lw(c);
                            c.cycles += 5;
                            c.g[A2][0] = req.off(0x18).lw(c);
                            c.g[A1][0] = req.off(0x20).lw(c);
                            EAFS_PLATFORM_READ.call(c, 0x003d_e39c)?;
                        } else {
                            c.cycles += 9;
                            let big = Addr(fh as u32).lw(c);
                            c.g[V0][0] = big;
                            c.g[A2][0] = req.off(0x18).lw(c);
                            c.g[V1][0] = Addr(big as u32).off(0xc).lw(c);
                            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[A1][0] = req.off(0x20).lw(c);
                            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                            EAFS_PLATFORM_READ.call(c, 0x003d_e388)?;
                        }
                    }
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e3b4 => {
                    c.cycles += 8;
                    c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                    c.g[A3][0] = req.off(0x1c).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = req.off(0x20).lw(c);
                    c.g[A2][0] = req.off(0x18).lw(c);
                    EAFS_PLATFORM_WRITE.call(c, 0x003d_e3cc)?;
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e3d4 => {
                    // Size: the handle's size into the request.
                    c.cycles += 7;
                    c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                    c.g[A0][0] = 1;
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
                    let v0 = c.g[V0][0];
                    req.off(0x18).sw(c, v0);
                    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e3e8)?;
                    frame.restore(c, S0, 0x260);
                }
                0x003d_e3f0 => {
                    c.cycles += 3;
                    let res = req.off(0xc).lw(c);
                    c.g[A0][0] = (res < 1) as u64;
                    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e3fc)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x260);
                }
                t => {
                    c.jump_out(t, 0x003d_dfa8, 0x003d_e41c);
                    return Err(crate::call::Unwound);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x250);
        frame.restore(c, S2, 0x240);
        frame.restore(c, S3, 0x230);
        frame.restore(c, S4, 0x220);
        frame.restore_ra(c, 0x210);
        frame.close(c);
        Ok(())
    })
}

/// The open / exists commands of [`eafile_service_queue`] (`s1` the
/// request): parse `big|name`, `|name` or a plain path, open the plain path
/// (exists: close it again; open: size and LSN), else look the name up in
/// the open big files; free the handle of an exists, complete.
pub(crate) fn eafile_open_command(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let req = Addr(c.g[S1][0] as u32);
    let sp = |c: &Cpu| c.g[SP][0] as u32;
    c.cycles += 7;
    let name_buf = sp(c).wrapping_add(0x100);
    Addr(name_buf).sb(c, 0);
    c.g[A1][0] = 0x7c;
    let fh = req.off(EAFS_REQ_HANDLE).lw(c);
    c.g[A0][0] = sx((fh as u32).wrapping_add(0x10));
    call_ptr(c, EAFS_STRCHR, 0x003d_e11c)?;
    let has_bar = c.g[V0][0] != 0;
    c.g[V1][0] = 0x7c;
    if !has_bar {
        // A plain path: open it if the flags ask (bit 0).
        c.cycles += 4;
        c.g[V0][0] = req.off(0x18).lw(c) & 1;
        c.g[S3][0] = 1;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
            c.g[S3][0] = 3;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x100));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            call_ptr(c, EAFS_STRCPY, 0x003d_e1c0)?;
        }
        c.cycles += 1;
        c.g[S2][0] = 0;
    } else {
        c.cycles += 4;
        c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x10).lb(c);
        if c.g[V0][0] != c.g[V1][0] {
            // `big|name`: the big file's name into the stack buffer.
            c.g[A1][0] = 0x7c;
            c.cycles += 15;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            call_ptr(c, EAFS_STRCHR, 0x003d_e144)?;
            c.g[S3][0] = 4;
            c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
            Addr(sp(c)).sb(c, 0);
            c.g[A0][0] = c.g[SP][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[A1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            call_ptr(c, EAFS_STRNCPY, 0x003d_e16c)?;
            c.g[V1][0] = sx(sp(c).wrapping_add(c.g[S0][0] as u32));
            Addr(c.g[V1][0] as u32).sb(c, 0);
            c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
        } else {
            // `|name`: any big file.
            c.cycles += 2;
            c.g[S3][0] = 2;
        }
        // The entry name after the bar.
        c.cycles += 9;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x100));
        c.g[A1][0] = 0x7c;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        call_ptr(c, EAFS_STRCHR, 0x003d_e188)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        call_ptr(c, EAFS_STRCPY, 0x003d_e194)?;
        c.g[S2][0] = 0;
    }
    // A plain path: the platform file.
    c.cycles += 3;
    c.g[V0][0] = c.g[S3][0] & 1;
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        let fh = req.off(EAFS_REQ_HANDLE).lw(c);
        c.g[A1][0] = req.off(0x18).lw(c);
        c.g[A2][0] = fh;
        c.g[A0][0] = sx((fh as u32).wrapping_add(0x10));
        EAFS_PLATFORM_OPEN.call(c, 0x003d_e1e4)?;
        let opened = c.g[V0][0] != 0;
        c.g[V1][0] = 8;
        if opened {
            c.cycles += 5;
            let cmd = (req.lw(c) as u32 >> 20) & 0xf;
            c.g[V0][0] = cmd as u64;
            c.g[S2][0] = 1;
            if cmd == 8 {
                c.cycles += 5;
                c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                EAFS_PLATFORM_CLOSE.call(c, 0x003d_e20c)?;
            } else {
                c.cycles += 10;
                c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                EAFS_PLATFORM_GET_SIZE.call(c, 0x003d_e220)?;
                c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                let v0 = c.g[V0][0];
                Addr(c.g[V1][0] as u32).off(4).sw(c, v0);
                c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                EAFS_PLATFORM_GET_LSN.call(c, 0x003d_e234)?;
                c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                let v0 = c.g[V0][0];
                Addr(c.g[V1][0] as u32).off(0xc).sw(c, v0);
            }
        }
    }
    // Not opened: search the open big files (bit 1: any, bit 2: the named one).
    c.cycles += 2;
    if c.g[S2][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 6;
        let search = c.g[V0][0] != 0;
        c.g[V0][0] = sx(EAFS_STATE);
        let mut found_none = true;
        if search {
            c.cycles += 3;
            c.g[S0][0] = Addr(EAFS_STATE).off(0x34).lw(c);
            c.g[S3][0] &= 4;
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                loop {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[SP][0];
                    let mut try_it = true;
                    if c.g[S3][0] != 0 {
                        c.cycles += 5;
                        let big = Addr(c.g[S0][0] as u32);
                        c.g[A0][0] = big.off(0xc).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        call_ptr(c, EAFS_STRCMP, 0x003d_e274)?;
                        try_it = c.g[V0][0] == 0;
                    }
                    if try_it {
                        c.cycles += 8;
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        c.g[A1][0] = sx(sp(c).wrapping_add(0x100));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = sx(sp(c).wrapping_add(0x200));
                        c.g[T0][0] = sx(sp(c).wrapping_add(0x204));
                        EAFS_BIG_FIND_ENTRY.call(c, 0x003d_e294)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 10;
                            c.g[V0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                            c.g[S2][0] = 1;
                            let s0 = c.g[S0][0];
                            Addr(c.g[V0][0] as u32).sw(c, s0);
                            c.g[V0][0] = Addr(sp(c).wrapping_add(0x204)).lw(c);
                            c.g[V1][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                            let v0 = c.g[V0][0];
                            Addr(c.g[V1][0] as u32).off(4).sw(c, v0);
                            c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
                            c.g[V0][0] = Addr(sp(c).wrapping_add(0x200)).lw(c);
                            let v0 = c.g[V0][0];
                            Addr(c.g[A0][0] as u32).off(8).sw(c, v0);
                        }
                    }
                    c.g[S0][0] = Addr(c.g[S0][0] as u32).off(0x14).lw(c);
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        found_none = false;
                        break;
                    }
                    c.cycles += 2;
                    if c.g[S2][0] != 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_e260);
                    }
                }
            }
        }
        if found_none {
            c.cycles += 1;
        }
    }
    c.g[V0][0] = req.lw(c);
    // An exists request frees its handle and keeps the answer.
    c.cycles += 5;
    c.g[V1][0] = 8;
    let cmd = (c.g[V0][0] as u32 >> 20) & 0xf;
    c.g[V0][0] = cmd as u64;
    if cmd == 8 {
        c.cycles += 3;
        c.g[A0][0] = req.off(EAFS_REQ_HANDLE).lw(c);
        EAFS_FREE_FILE_HANDLE.call(c, 0x003d_e2f4)?;
        let s2 = c.g[S2][0];
        req.off(0x18).sw(c, s2);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    EAFS_COMPLETE_REQUEST.call(c, 0x003d_e300)?;
    frame.restore(c, S0, 0x260);
    Ok(())
}

/// `int EAFile_Startup(n0, n1, n2)` (0x3dcbd8): unless the layer already
/// started (`[0x519ab0]` nonzero: returns 0), size it
/// ([`ea_file_calc_mem_size`]), allocate that through the allocator
/// callback (`[0x450c14]`, named by the string at `0x495d28`, alignment
/// 0x100), store the block at `0x519ae8` and return
/// [`ea_file_init`]`(n0, n1, n2, block)`.
pub fn ea_file_startup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = EA_BSS_HI;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6550));
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let fresh = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        if !fresh {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 20;
            c.g[S0][0] = EA_DATA_HI;
            c.g[S1][0] = 0x490000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc10));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5d28));
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003d_cc34);
            c.g[A2][0] = c.g[S3][0];
            callee::EA_FILE_CALC_MEM_SIZE.call(c, 0x003d_cc34)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            let alloc = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x003d_cc48);
            c.g[A2][0] = 0x100;
            c.pc = alloc;
            call_ptr(c, alloc, 0x003d_cc48)?;
            c.g[V1][0] = EA_BSS_HI;
            c.g[A0][0] = c.g[S4][0];
            c.sw((c.g[V1][0] as u32).wrapping_sub(0x6518), c.g[V0][0]);
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x003d_cc64);
            c.g[A3][0] = c.g[V0][0];
            callee::EA_FILE_INIT.call(c, 0x003d_cc64)?;
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

/// `int EAFile_Init(n0, n1, n2, mem)` (0x3dcc88): zero counts take the
/// defaults 0x10, 0x3e8 and 0x20. Unless the layer already started
/// (returns 0): `[0x519ab0] = n2`, `[+4] = n0`, the mutex, `mem` zeroed
/// for [`ea_file_calc_mem_size`] bytes with `n2` 0x30-byte request slots
/// first and the 0x110-byte handle slots after them (`[+0x2c]`); the CD
/// layer (`n1`, the area after `n0` handle slots) when `[0x450c60]` is
/// set, then the platform layer. Returns 1.
pub fn ea_file_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = EA_BSS_HI;
        frame.save(c, S1, 0x50);
        frame.save(c, S4, 0x20);
        c.g[S1][0] = 0x20;
        frame.save(c, S5, 0x10);
        c.g[S4][0] = 0x3e8;
        frame.save(c, S0, 0x60);
        c.g[S5][0] = 0x10;
        frame.save(c, S3, 0x30);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6550));
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        if c.g[A0][0] != 0 {
            c.g[S5][0] = c.g[A0][0];
        }
        if c.g[A1][0] != 0 {
            c.g[S4][0] = c.g[A1][0];
        }
        let started = c.g[V0][0] != 0;
        if c.g[A2][0] != 0 {
            c.g[S1][0] = c.g[A2][0];
        }
        if !started {
            c.cycles += 24;
            c.g[V0][0] = 0xff;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_sub(0x6550), c.g[S1][0]);
            c.g[RA][0] = sx(0x003d_ccec);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S5][0]);
            callee::EA_MUTEX_INIT.call(c, 0x003d_ccec)?;
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x003d_cd00);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S3][0]);
            callee::EA_FILE_CALC_MEM_SIZE.call(c, 0x003d_cd00)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[A2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003d_cd10);
            c.g[A1][0] = 0;
            callee::EA_MEM_SET.call(c, 0x003d_cd10)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_sub(0x6550));
            c.g[A0][0] = 0x30;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = EA_DATA_HI;
            c.mult(0, 2, 2, 4);
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc60));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let skip_cd = c.g[A1][0] == 0;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
            if !skip_cd {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x110;
                c.g[A0][0] = c.g[S4][0];
                c.mult(0, 3, 2, 5);
                c.g[RA][0] = sx(0x003d_cd4c);
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                callee::EA_FILE_CD_INIT.call(c, 0x003d_cd4c)?;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003d_cd64);
            c.g[A0][0] = c.g[S5][0];
            callee::EA_FILE_PLATFORM_INIT.call(c, 0x003d_cd64)?;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `u32 EAFile_CalcMemSize(a0, a1, a2)` (0x3dcd98): the request-area bytes
/// for the given queue counts, or the defaults (0x10/0x20/0x3e8) where an
/// argument is 0, plus the table and header overhead.
pub fn ea_file_calc_mem_size(c: &mut Cpu) {
    c.cycles += 18;
    c.g[V0][0] = EA_DATA_HI;
    c.g[A3][0] = 0x10;
    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc5c));
    if c.g[A0][0] != 0 {
        c.g[A3][0] = c.g[A0][0];
    }
    c.g[V0][0] = 0x20;
    c.g[V1][0] = 0x3e8;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = c.g[A2][0];
    }
    c.g[A0][0] = 0x30;
    c.g[A2][0] = 0x110;
    c.mult(0, 2, 2, 4);
    c.mult(0, 7, 7, 6);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = c.g[A1][0];
    }
    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
    c.mult(0, 3, 3, 8);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
    c.pc = ra;
}

/// `u32 EAFile_GetRequestStatus(request)` (0x3dce90): the status nibble of
/// `request` (bits 16..20 of the request word ^ the handle), or -3 when the
/// status word is clear; with the word at `sp+4` it then loads the
/// request's status field.
pub fn ea_file_get_request_status(c: &mut Cpu) {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A3][0] = c.g[A0][0];
    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
    c.g[V0][0] = 0;
    let null = c.g[A3][0] == 0;
    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
    if !null {
        c.cycles += 12;
        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = 0x30;
        c.g[A2][0] = EA_BSS_HI;
        c.g[A0][0] = 0xf0000;
        c.mult(0, 3, 3, 2);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x6528));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    }
    c.cycles += 2;
    let clear = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
    if !clear {
        c.cycles += 9;
        c.g[A1][0] = 0x30;
        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = EA_BSS_HI;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6528));
        c.mult(0, 3, 3, 5);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
    } else {
        c.cycles += 1;
        c.g[V0][0] = 0xfffffffffffffffd;
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `u32 EAFile_GetRequestResult(request)` (0x3dcf10): the result field of
/// `request` (found through the 0x30-byte request table), like
/// [`ea_file_get_request_status`] but loading `+0xc`.
pub fn ea_file_get_request_result(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A1][0] = 0x30;
    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
    c.g[V0][0] = EA_BSS_HI;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6528));
    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
    c.mult(0, 3, 3, 5);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `u32 EAFile_GetRequestBytes(request)` (0x3dcf40): the transferred-bytes
/// field of `request` (`+0x14`).
pub fn ea_file_get_request_bytes(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A1][0] = 0x30;
    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
    c.g[V0][0] = EA_BSS_HI;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6528));
    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
    c.mult(0, 3, 3, 5);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `void EAFile_SetRequestPriority(request, prio)` (0x3dcfc0): set the
/// priority of `request` and, when the queue is running and the request is
/// idle, unlink it and reinsert it in priority order.
pub fn ea_file_set_request_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = EA_BSS_HI;
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[A0][0] = 0;
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.mult(0, 2, 2, 7);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6550));
        c.g[S2][0] = c.g[A1][0];
        c.g[S3][0] = c.g[V1][0];
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
        let null = c.g[A2][0] == 0;
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !null {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0xf0000;
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x3));
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.mult(0, 2, 3, 7);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] ^ c.g[A2][0];
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            c.g[A0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 8;
            c.g[RA][0] = sx(0x003d_d048);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            EAFS_MUTEX_LOCK.call(c, 0x003d_d048)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let queued = c.g[V0][0] != 0;
            c.g[A0][0] = EA_BSS_HI;
            // Gen (`b_3dd120`): a queued request (state < 2) skips the
            // reinsert scan and goes straight to unlock; otherwise three
            // gate checks select the scan, and either scan outcome falls
            // into the second (HEAD2) block.
            let mut to_head2 = false;
            if !queued {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                if c.g[S0][0] != c.g[V0][0] {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[A1][0] = 0;
                        if c.g[V1][0] != c.g[S2][0] {
                            to_head2 = true;
                            const HEAD: u32 = 0x003d_d098;
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                            if c.g[V1][0] != 0 {
                                c.cycles += 2;
                                if c.g[V1][0] != c.g[S0][0] {
                                    c.cycles += 1;
                                    c.g[A1][0] = c.g[V1][0];
                                    loop {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                                        if c.g[V1][0] == 0 {
                                            break;
                                        }
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[S0][0] {
                                            break;
                                        }
                                        c.g[A1][0] = c.g[V1][0];
                                        if event_due(c) {
                                            deliver_event_here(c, HEAD);
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                            if c.g[A1][0] != 0 {
                                c.cycles += 2;
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
                            } else {
                                c.cycles += 2;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A2][0]);
                            }
                        }
                    }
                }
            }
            if to_head2 {
                c.cycles += 4;
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                let empty = c.g[V0][0] == 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                if !empty {
                    const HEAD2: u32 = 0x003d_d0f0;
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let after = c.g[V0][0] != 0;
                    if !after {
                        c.cycles += 2;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                            let end = c.g[V0][0] == 0;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                            if end {
                                break;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] != 0 {
                                break;
                            }
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                            if event_due(c) {
                                deliver_event_here(c, HEAD2);
                            }
                        }
                    }
                }
                c.cycles += 2;
                let no_prev2 = c.g[A1][0] == 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                if !no_prev2 {
                    c.cycles += 2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), c.g[S0][0]);
                } else {
                    c.cycles += 1;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[S0][0]);
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003d_d128);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x653c));
            EAFS_MUTEX_UNLOCK.call(c, 0x003d_d128)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_CancelRequest(request)` (0x3dd148): for a live request
/// (its slot's status word matches) of kind 3 or 0xa (bits 20-23) returns
/// 0xa; another live kind marks the slot's +4 word cancelled (1) and
/// returns the slot. A null or stale request returns `request >> 20`.
pub fn ea_file_cancel_request(c: &mut Cpu) {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[A3][0] = c.g[A0][0];
    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
    c.g[V0][0] = 0;
    let null = c.g[A3][0] == 0;
    c.sw(c.g[SP][0] as u32, c.g[A3][0]);
    if !null {
        c.cycles += 12;
        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[V0][0] = 0x30;
        c.g[A2][0] = EA_BSS_HI;
        c.g[A0][0] = 0xf0000;
        c.mult(0, 3, 3, 2);
        c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x6528));
        c.g[A0][0] = c.g[A0][0] | 0xffff;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    }
    c.cycles += 2;
    let clear = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[A3][0] as u32) >> 20);
    if !clear {
        c.cycles += 4;
        c.g[V1][0] = 0x3;
        c.g[A0][0] = c.g[V0][0] & 0xf;
        let is_kind3 = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = 0xa;
        if !is_kind3 {
            c.cycles += 2;
            let is_kind_a = c.g[A0][0] == c.g[V0][0];
            c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            if !is_kind_a {
                c.cycles += 8;
                c.g[A2][0] = 0x30;
                c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x3));
                c.g[V1][0] = EA_BSS_HI;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x6528));
                c.g[A1][0] = 0x1;
                c.mult(0, 2, 2, 6);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = ra;
}

/// `u32 EAFile_WaitRequest(request)` (0x3dd1d8): -3 when `request` is null
/// or its status bit is clear; the ready word when it is already set;
/// otherwise yield (or run the timer callbacks on the file thread) until
/// the word arrives, re-checking the bit each round.
pub fn ea_file_wait_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        c.g[A3][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = EA_BSS_HI;
        frame.save(c, S1, 0x60);
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6550));
        c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[A1][0] = c.g[A0][0];
        frame.save(c, S0, 0x70);
        c.g[A0][0] = 0;
        frame.save(c, S2, 0x50);
        c.mult(0, 2, 2, 7);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x28));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        let null = c.g[A1][0] == 0;
        c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if !null {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0xf0000;
            c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x3));
            c.g[A0][0] = c.g[A0][0] | 0xffff;
            c.mult(0, 2, 3, 7);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
            c.g[A0][0] = (c.g[V0][0] < 0x1u64) as u64;
        }
        c.cycles += 2;
        let clear = c.g[A0][0] == 0;
        c.g[V0][0] = 0xfffffffffffffffd;
        if !clear {
            const HEAD: u32 = 0x003d_d278;
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            let ready = c.g[V0][0] != 0;
            c.g[S0][0] = 0xf0000;
            // Gen (`b_3dd2e8`): every non-aborted exit reloads the wait
            // word and costs one more cycle, including ready-at-entry
            // (the `break 'b_3dd2e8` falls into the shared tail); only
            // the aborted spin exit skips it (`break 'b_3dd2ec`).
            let mut skip_tail = false;
            if !ready {
                c.cycles += 5;
                c.g[S5][0] = c.g[T0][0];
                c.g[S0][0] = c.g[S0][0] | 0xffff;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[S4][0] = 0x30;
                c.g[S3][0] = 0x1;
                let mut aborted = false;
                loop {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003d_d280);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_IS_CURRENT.call(c, 0x003d_d280)?;
                    let on_file_thread = c.g[V0][0] == 0;
                    if !on_file_thread {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003d_d290);
                        c.g[A0][0] = 0;
                        callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003d_d290)?;
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    } else {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003d_d2a0);
                        c.g[A0][0] = 0x1;
                        callee::EA_THREAD_YIELD.call(c, 0x003d_d2a0)?;
                        c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    }
                    c.cycles += 3;
                    c.g[A2][0] = 0;
                    let gone = c.g[A1][0] == 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    if !gone {
                        c.cycles += 9;
                        c.g[V1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x3));
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28));
                        c.mult(0, 3, 3, 20);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
                        c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
                        if c.g[V0][0] != 0 {
                            c.g[A2][0] = 0;
                        }
                    }
                    c.cycles += 2;
                    let done = c.g[A2][0] == 0;
                    c.g[V0][0] = 0xfffffffffffffffd;
                    if done {
                        aborted = true;
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let ready = c.g[V0][0] == 0;
                    if !ready {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
                skip_tail = aborted;
            }
            if !skip_tail {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueExists(handle, path, mode)` (0x3dd438): allocate a
/// request for the path and run the queue; the request word or, when the
/// handle is null, a stub signal.
pub fn ea_file_queue_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d460);
        c.g[S3][0] = c.g[A0][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d460)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = 0x800000;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[V0][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] = 0x1;
        c.sw(c.g[S2][0] as u32, c.g[V1][0]);
        c.g[RA][0] = sx(0x003d_d494);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        callee::EA_FILE_ALLOC_FILE_HANDLE.call(c, 0x003d_d494)?;
        let got_handle = c.g[V0][0] != 0;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        if !got_handle {
            c.cycles += 4;
            c.g[V0][0] = 0x2;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003d_d4ac);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            callee::EA_FILE_QUEUE_SIGNAL_STUB.call(c, 0x003d_d4ac)?;
        }
        c.cycles += 15;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x003d_d4c0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        c.pc = 0x0041_6ce0;
        call_ptr(c, callee::STRNCPY, 0x003d_d4c0)?;
        c.g[RA][0] = sx(0x003d_d4c8);
        c.g[A0][0] = c.g[S2][0];
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d4c8)?;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueWrite(handle, buf, size, off, mode)` (0x3dd720):
/// allocate a write request for the buffer and run the queue; a stub
/// signal when the handle is null.
pub fn ea_file_queue_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[T1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A3][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d760);
        c.g[S6][0] = c.g[A2][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d760)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = 0x500000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let has_handle = c.g[S2][0] != 0;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        if !has_handle {
            c.cycles += 4;
            c.g[V0][0] = 0x6;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_d798);
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            callee::EA_FILE_QUEUE_SIGNAL_STUB.call(c, 0x003d_d798)?;
        }
        c.cycles += 18;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x24), c.g[S2][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[S4][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
        c.g[RA][0] = sx(0x003d_d7b4);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[S6][0]);
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d7b4)?;
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
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

/// `u32 EAFile_QueueGetSize(handle, cb, cb_arg)` (0x3dd7e0): allocate a
/// size request (kind 6) with the callback; for a live handle mark it
/// queued with the handle's file word, else signal it at once (the stub).
/// Returns the request word.
pub fn ea_file_queue_get_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d808);
        c.g[S3][0] = c.g[A0][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d808)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = 0x600000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S1][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let no_entry = c.g[S3][0] == 0;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        if !no_entry {
            c.cycles += 5;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x1;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x6;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003d_d858);
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            callee::EA_FILE_QUEUE_SIGNAL_STUB.call(c, 0x003d_d858)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueNop(a0, a1, a2)` (0x3dd878): allocate a no-op request
/// and run the queue.
pub fn ea_file_queue_nop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_d898);
        c.g[S2][0] = c.g[A1][0];
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_d898)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x700000;
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
        c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003d_d8c8);
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d8c8)?;
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnBigHeaderRead(request, handle)` (0x3dd8e8): release the
/// header request; when the archive type is still unknown, free the buffer
/// through the allocator callback. Then, when the buffered bytes cover the
/// header, record the request and run the queue, else read the full header
/// into place first.
pub fn ea_file_on_big_header_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x60);
        c.g[A3][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A1][0] = EA_BSS_HI;
        frame.save(c, S0, 0x50);
        c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6550));
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.mult(0, 3, 3, 7);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[RA][0] = sx(0x003d_d93c);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V1][0]);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_d93c)?;
        c.g[RA][0] = sx(0x003d_d944);
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        callee::BIG_FILE_GET_TYPE.call(c, 0x003d_d944)?;
        let known = c.g[V0][0] != 0;
        c.g[V1][0] = EA_DATA_HI;
        if !known {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc18));
            let free = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_d958);
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.pc = free;
            call_ptr(c, free, 0x003d_d958)?;
        }
        c.cycles += 7;
        c.g[RA][0] = sx(0x003d_d960);
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        callee::BIG_FILE_GET_HEADER_SIZE.call(c, 0x003d_d960)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
        let need_header = c.g[V0][0] != 0;
        if !need_header {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
            c.cycles += 5;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[S1][0]);
            c.g[RA][0] = sx(0x003d_d9f0);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_d9f0)?;
        } else {
            c.cycles += 27;
            c.g[S0][0] = EA_DATA_HI;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let free = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_d988);
            c.g[A0][0] = c.lw(c.g[S1][0] as u32);
            c.pc = free;
            call_ptr(c, free, 0x003d_d988)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[S2][0]);
            c.g[A0][0] = 0x490000;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d68));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_d9a4);
            c.g[A1][0] = c.g[S2][0];
            c.pc = alloc;
            call_ptr(c, alloc, 0x003d_d9a4)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0;
            c.g[A3][0] = c.g[S2][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[T1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003d_d9c4);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_d9c4)?;
            c.g[A1][0] = 0x3e0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2718));
            c.g[RA][0] = sx(0x003d_d9d8);
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_d9d8)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnBigOpened(request, handle)` (0x3dda10): release the open
/// request; when it was the header fetch, queue the header read, else mark
/// the archive ready and run the queue.
pub fn ea_file_on_big_opened(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        c.g[T0][0] = 0x30;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = EA_BSS_HI;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x6528));
        c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
        frame.save(c, S0, 0x40);
        c.mult(0, 2, 2, 8);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[RA][0] = sx(0x003d_da50);
        c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_da50)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let is_header = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x4;
        if !is_header {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[RA][0] = sx(0x003d_da70);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_da70)?;
        } else {
            c.cycles += 12;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0;
            c.g[T0][0] = c.g[S2][0];
            c.g[T1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003d_da90);
            c.g[A3][0] = 0x800;
            callee::EA_FILE_QUEUE_READ.call(c, 0x003d_da90)?;
            c.g[A1][0] = 0x3e0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2718));
            c.g[RA][0] = sx(0x003d_daa4);
            c.sw(c.g[SP][0] as u32, c.g[V0][0]);
            callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_daa4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueMountBig(handle, path, mode)` (0x3ddac0): unless the
/// archive is already mounted (name match), allocate a buffer, open the
/// path through the queue and set the mount callback.
pub fn ea_file_queue_mount_big(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_db30;
        c.cycles += 26;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = EA_BSS_HI;
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_dafc);
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x651c));
        callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_dafc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xffffffffff0f0000;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.g[V1][0] | 0xffff;
        c.g[A0][0] = 0x900000;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[S5][0]);
        c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
        let none_mounted = c.g[S1][0] == 0;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        let mut matched = false;
        if !none_mounted {
            c.cycles += 2;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A2][0] = 0x100;
                c.g[RA][0] = sx(0x003d_db40);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                c.pc = 0x0041_6b18;
                call_ptr(c, callee::STRNCMP, 0x003d_db40)?;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003d_db58);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_db58)?;
                    c.g[V0][0] = c.g[S2][0];
                    matched = true;
                    break;
                }
                c.cycles += 2;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        if !matched {
            c.cycles += 29;
            c.g[S0][0] = EA_DATA_HI;
            c.g[A0][0] = c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc10));
            c.g[A1][0] = 0x18;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_db84);
            c.g[A2][0] = c.g[S4][0];
            c.pc = alloc;
            call_ptr(c, alloc, 0x003d_db84)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = 0x490000;
            c.g[V0][0] = 0x800;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d68));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[S4][0]);
            c.g[A2][0] = c.g[S4][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            let alloc2 = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003d_dbb0);
            c.g[A1][0] = 0x800;
            c.pc = alloc2;
            call_ptr(c, alloc2, 0x003d_dbb0)?;
            c.g[A1][0] = 0x1;
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x003d_dbc8);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::EA_FILE_QUEUE_OPEN.call(c, 0x003d_dbc8)?;
            c.g[A1][0] = 0x3e0000;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003d_dbd8);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x25f0));
            callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_dbd8)?;
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
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

/// `void EAFile_OnBigUnmounted(request, handle)` (0x3ddc00): release the
/// unmount request, then run the queue on its handle.
pub fn ea_file_on_big_unmounted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_dc14);
        c.g[S0][0] = c.g[A2][0];
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_dc14)?;
        c.g[RA][0] = sx(0x003d_dc1c);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_dc1c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_QueueUnmountBig(handle, path)` (0x3ddc30): allocate an
/// unmount request, unlink the matching mounted archive, free its slots
/// and run the queue; returns the request word.
///
/// Dispatcher-shaped in the original (loop heads `0x3ddcf0`, `0x3ddcf4`
/// and `0x3ddd40` are resume entries): the port keeps the dispatch and
/// its state in the registers.
pub fn ea_file_queue_unmount_big(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003d_dc30 => {
                    c.cycles += 32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S0][0] = EA_BSS_HI;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S1][0] = c.g[A2][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S6][0] = 0;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S7][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[RA][0] = sx(0x003d_dc78);
                    c.g[S2][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x34));
                    callee::EA_FILE_ALLOC_REQUEST.call(c, 0x003d_dc78)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A0][0] = 0xa00000;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[V0][0] = 0xffffffffff0f0000;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x2c));
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[S7][0]);
                    c.sw(c.g[S3][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    let no_big = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x7;
                    pc = if no_big { 0x003d_dcb4 } else { 0x003d_dcb0 };
                }
                0x003d_dcb0 => {
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    pc = 0x003d_dcb4;
                }
                0x003d_dcb4 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0;
                    c.g[A2][0] = EA_DATA_HI;
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = 0x3e0000;
                    pc = if empty { 0x003d_dd44 } else { 0x003d_dcc8 };
                }
                0x003d_dcc8 => {
                    c.cycles += 2;
                    let no_handle = c.g[A1][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    pc = if no_handle { 0x003d_dcf0 } else { 0x003d_dcd0 };
                }
                0x003d_dcd0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        pc = 0x003d_dcf4;
                    } else {
                        pc = 0x003d_dcdc;
                    }
                }
                0x003d_dcdc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let found = c.g[V0][0] == c.g[S4][0];
                    c.g[V0][0] = 0xfffffffffffffffe;
                    pc = if found { 0x003d_dd2c } else { 0x003d_dcec };
                }
                0x003d_dcec => {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    pc = 0x003d_dcf0;
                }
                0x003d_dcf0 => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d_dcf4;
                }
                0x003d_dcf4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x110));
                    pc = if done { 0x003d_dd44 } else { 0x003d_dd04 };
                }
                0x003d_dd04 => {
                    c.cycles += 2;
                    let empty = c.g[A1][0] == 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    if empty {
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dcf0) {
                            return Ok(());
                        }
                        pc = 0x003d_dcf0;
                    } else {
                        pc = 0x003d_dd0c;
                    }
                }
                0x003d_dd0c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dcf4) {
                            return Ok(());
                        }
                        pc = 0x003d_dcf4;
                    } else {
                        pc = 0x003d_dd18;
                    }
                }
                0x003d_dd18 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let found = c.g[V1][0] != c.g[S4][0];
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    if found {
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dcf0) {
                            return Ok(());
                        }
                        pc = 0x003d_dcf0;
                    } else {
                        pc = 0x003d_dd28;
                    }
                }
                0x003d_dd28 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    pc = 0x003d_dd2c;
                }
                0x003d_dd2c => {
                    c.cycles += 4;
                    c.g[V1][0] = 0x8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    pc = 0x003d_dd44;
                }
                0x003d_dd3c => {
                    c.cycles += 1;
                    pc = 0x003d_dd40;
                }
                0x003d_dd40 => {
                    c.cycles += 1;
                    c.g[S2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    pc = 0x003d_dd44;
                }
                0x003d_dd44 => {
                    c.cycles += 2;
                    let done = c.g[S2][0] == 0;
                    c.g[V0][0] = 0x5;
                    pc = if done { 0x003d_dd60 } else { 0x003d_dd4c };
                }
                0x003d_dd4c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != c.g[S4][0] {
                        c.g[S6][0] = c.g[S2][0];
                        if event_due(c) && back_edge_unwinds(c, 0x003d_dd40) {
                            return Ok(());
                        }
                        pc = 0x003d_dd40;
                    } else {
                        pc = 0x003d_dd58;
                    }
                }
                0x003d_dd58 => {
                    c.cycles += 2;
                    let found = c.g[S2][0] != 0;
                    c.g[V0][0] = 0x5;
                    pc = if found { 0x003d_dd64 } else { 0x003d_dd60 };
                }
                0x003d_dd60 => {
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    pc = 0x003d_dd64;
                }
                0x003d_dd64 => {
                    c.cycles += 2;
                    let linked = c.g[S6][0] == 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
                    pc = if linked { 0x003d_dd78 } else { 0x003d_dd6c };
                }
                0x003d_dd6c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    pc = 0x003d_dd84;
                }
                0x003d_dd78 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[A0][0]);
                    pc = 0x003d_dd84;
                }
                0x003d_dd84 => {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc10));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    let free = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003d_dd94);
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.pc = free;
                    call_ptr(c, free, 0x003d_dd94)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    let free2 = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003d_dda0);
                    c.g[A0][0] = c.g[S2][0];
                    c.pc = free2;
                    call_ptr(c, free2, 0x003d_dda0)?;
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003d_ddb0);
                    c.g[A1][0] = c.g[S7][0];
                    callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_ddb0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003d_ddbc);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x2400));
                    callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_ddbc)?;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_dc30, 0x003d_ddec);
                    return Ok(());
                }
            }
        }
    })
}

/// `char *EAFile_GetRootPath(char *out)` (0x3ddf80): copy the current
/// root (`strncpy`, 0x100 bytes) into `out`; returns `out`.
pub fn ea_file_get_root_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = EA_BSS_HI;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6510));
        c.g[RA][0] = sx(0x003d_df98);
        c.g[A2][0] = 0x100;
        call_ptr(c, callee::STRNCPY, 0x003d_df98)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ParseBigPath(path, a1, a2, out_size)` (0x3de4d0): split
/// `path` at `|`; with no pipe, open the plain file for its size, else
/// find the archive entry in a mounted big. Returns 1 with the size stored
/// (or 0).
pub fn ea_file_parse_big_path(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_e5e8;
        c.cycles += 18;
        let frame = Frame::open(c, 0x280);
        frame.save(c, S4, 0x230);
        frame.save(c, S1, 0x260);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S2, 0x250);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x220);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x270);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S3, 0x240);
        frame.save_ra(c, 0x210);
        c.g[A1][0] = 0x7c;
        c.sw(c.g[S4][0] as u32, 0);
        c.g[RA][0] = sx(0x003d_e510);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x100), 0);
        call_ptr(c, callee::STRCHR, 0x003d_e510)?;
        let no_pipe = c.g[V0][0] == 0;
        c.g[V0][0] = 0x7c;
        'b_3de598: {
            if !no_pipe {
                'b_3de554: {
                    'b_3de52c: {
                        c.cycles += 3;
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                        let not_first = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        if not_first {
                            break 'b_3de52c;
                        }
                        c.cycles += 2;
                        c.g[S3][0] = 0x2;
                        break 'b_3de554;
                    }
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x003d_e534);
                    c.g[A1][0] = 0x7c;
                    call_ptr(c, callee::STRCHR, 0x003d_e534)?;
                    c.g[S3][0] = 0x4;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003d_e54c);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, callee::STRNCPY, 0x003d_e54c)?;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.sb(c.g[V1][0] as u32, 0);
                    break 'b_3de554;
                }
                c.cycles += 9;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003d_e564);
                c.g[A1][0] = 0x7c;
                call_ptr(c, callee::STRCHR, 0x003d_e564)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003d_e570);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                call_ptr(c, callee::STRCPY, 0x003d_e570)?;
                c.g[V0][0] = c.g[S3][0] & 0x1;
                // (Generated: unconditional break to 3de598; the else is dead.)
                break 'b_3de598;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S5][0] & 0x1;
            let direct = c.g[V0][0] == 0;
            c.g[S3][0] = 0x1;
            if !direct {
                c.cycles += 4;
                c.g[S3][0] = 0x3;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = sx(0x003d_e594);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, callee::STRCPY, 0x003d_e594)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            break 'b_3de598;
        }
        'b_3de5c8: {
            c.cycles += 2;
            let done = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if done {
                break 'b_3de5c8;
            }
            c.cycles += 5;
            c.g[A1][0] = c.g[S5][0];
            c.g[RA][0] = sx(0x003d_e5ac);
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
            callee::EA_FILE_PLATFORM_OPEN.call(c, 0x003d_e5ac)?;
            let failed = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
            if failed {
                break 'b_3de5c8;
            }
            c.cycles += 5;
            c.g[RA][0] = sx(0x003d_e5bc);
            c.g[S2][0] = 0x1;
            callee::EA_FILE_PLATFORM_GET_SIZE.call(c, 0x003d_e5bc)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
            c.g[RA][0] = sx(0x003d_e5c8);
            c.sw(c.g[S4][0] as u32, c.g[V0][0]);
            callee::EA_FILE_PLATFORM_CLOSE.call(c, 0x003d_e5c8)?;
            break 'b_3de5c8;
        }
        'b_3de644: {
            c.cycles += 2;
            let have = c.g[S2][0] != 0;
            c.g[V0][0] = c.g[S2][0];
            if have {
                break 'b_3de644;
            }
            'b_3de640: {
                c.cycles += 3;
                c.g[V0][0] = c.g[S3][0] & 0x6;
                let plain = c.g[V0][0] == 0;
                c.g[V0][0] = EA_BSS_HI;
                if plain {
                    break 'b_3de640;
                }
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x651c));
                let none = c.g[S0][0] == 0;
                c.g[S1][0] = c.g[S3][0] & 0x4;
                if none {
                    break 'b_3de640;
                }
                loop {
                    c.cycles += 2;
                    let skip_name = c.g[S1][0] == 0;
                    c.g[A1][0] = c.g[SP][0];
                    let mut do_find = skip_name;
                    // Mismatch advances `S0` and skips the shared advance
                    // below (generated `break 'b_3de630`).
                    let mut mismatched = false;
                    if !skip_name {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[RA][0] = sx(0x003d_e5fc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.pc = 0x0041_65a8;
                        call_ptr(c, callee::STRCMP, 0x003d_e5fc)?;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            mismatched = true;
                        } else {
                            do_find = true;
                        }
                    }
                    if !mismatched {
                        if do_find {
                            c.cycles += 8;
                            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                            c.g[A2][0] = 0;
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x204));
                            c.g[RA][0] = sx(0x003d_e61c);
                            c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x208));
                            callee::EA_FILE_BIG_FIND_ENTRY.call(c, 0x003d_e61c)?;
                            let missing = c.g[V0][0] == 0;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x208));
                            if !missing {
                                c.cycles += 2;
                                c.g[S2][0] = 0x1;
                                c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                            }
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    }
                    c.cycles += 2;
                    let end = c.g[S0][0] == 0;
                    c.g[V0][0] = c.g[S2][0];
                    if end {
                        break 'b_3de644;
                    }
                    c.cycles += 2;
                    let found = c.g[S2][0] != 0;
                    if !found {
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                    } else {
                        break;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x270);
        frame.restore(c, S1, 0x260);
        frame.restore(c, S2, 0x250);
        frame.restore(c, S3, 0x240);
        frame.restore(c, S4, 0x230);
        frame.restore(c, S5, 0x220);
        frame.restore_ra(c, 0x210);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_QueueSignalStub()` (0x3de668): the 2-cycle queue-signal
/// placeholder.
pub fn ea_file_queue_signal_stub(c: &mut Cpu) {
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
}

/// `u32 EAFile_CallWithPriority(fn, _, prio, arg)` (0x3dddf0): call
/// `fn(prio, arg)` through the table with the queue's current priority
/// (`[0x519ab8]`) set to `prio`, restore it, run the queue and return
/// `fn`'s result.
pub fn ea_file_call_with_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = EA_BSS_HI;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6550));
        frame.save(c, S2, 0x10);
        c.g[A1][0] = c.g[A3][0];
        c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x003d_de24);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        c.pc = func;
        call_ptr(c, func, 0x003d_de24)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003d_de34);
        c.g[A0][0] = 0;
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_de34)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_SetPriority(prio)` (0x3dde50): store the queue priority
/// and run the queue once.
pub fn ea_file_set_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = EA_BSS_HI;
        let frame = Frame::open(c, 0x10);
        c.sw((c.g[V0][0] as u32).wrapping_sub(0x6548), c.g[A0][0]);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_de68);
        c.g[A0][0] = 0;
        callee::EA_FILE_SERVICE_QUEUE.call(c, 0x003d_de68)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_SetRootPath(path)` (0x3dde78): switch the file root to
/// `path` (creating it), keeping the `0x45c44` mode flags in sync.
pub fn ea_file_set_root_path(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = EA_DATA_HI;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc50));
        frame.save_ra(c, 0);
        'b_3ddf6c: {
            'b_3ddf68: {
                'b_3ddf20: {
                    'b_3ddf1c: {
                        'b_3ddee8: {
                            c.cycles += 18;
                            c.g[RA][0] = sx(0x003d_dea0);
                            call_ptr(c, callee::STRLEN, 0x003d_dea0)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = EA_DATA_HI;
                            c.g[A2][0] = 0x3;
                            c.g[RA][0] = sx(0x003d_deb8);
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc08));
                            call_ptr(c, callee::STRNCMP, 0x003d_deb8)?;
                            if c.g[V0][0] != 0 {
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc50));
                                break 'b_3ddee8;
                            }
                            c.cycles += 6;
                            c.g[V0][0] = EA_DATA_HI;
                            c.g[A0][0] = 0x2;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc48));
                            c.g[V1][0] = c.g[V1][0] & 0x3;
                            let keep = c.g[V1][0] == c.g[A0][0];
                            c.g[V1][0] = EA_DATA_HI;
                            if keep {
                                break 'b_3ddf68;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = 0x1;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3));
                            break 'b_3ddf1c;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003d_def4);
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, callee::STRNCMP, 0x003d_def4)?;
                        let differ = c.g[V0][0] != 0;
                        c.g[V0][0] = EA_BSS_HI;
                        if differ {
                            break 'b_3ddf20;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = EA_DATA_HI;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc48));
                        c.g[V0][0] = c.g[V0][0] & 0x2;
                        let no_cd = c.g[V0][0] == 0;
                        c.g[V1][0] = EA_DATA_HI;
                        if no_cd {
                            break 'b_3ddf68;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x2;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        break 'b_3ddf1c;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = EA_BSS_HI;
                    break 'b_3ddf20;
                }
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6510));
                c.g[RA][0] = sx(0x003d_df30);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::STRCPY, 0x003d_df30)?;
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xff));
                if c.g[V1][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'b_3ddf6c;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003d_df44);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::STRLEN, 0x003d_df44)?;
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                if empty {
                    break 'b_3ddf68;
                }
                c.cycles += 4;
                c.g[A0][0] = 0x2f;
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_sub(0x1));
                let slash = c.g[V1][0] == c.g[A0][0];
                c.g[A0][0] = c.g[S0][0];
                if slash {
                    break 'b_3ddf68;
                }
                c.cycles += 3;
                c.g[A1][0] = 0x490000;
                c.g[RA][0] = sx(0x003d_df68);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5d78));
                call_ptr(c, callee::STRCAT, 0x003d_df68)?;
                break 'b_3ddf68;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_3ddf6c;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetSettings(count_ptr)` (0x3de928): copies the active file
/// settings out through the settings globals; always 1.
pub fn ea_file_get_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[S1][0] as i64) < 37) as u64;
        let fits = c.g[V0][0] != 0;
        c.g[V0][0] = EA_DATA_HI;
        if !fits {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003d_e954);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_MEM_ZERO.call(c, 0x003d_e954)?;
            c.g[V0][0] = EA_DATA_HI;
        }
        c.cycles += 23;
        c.g[A1][0] = EA_DATA_HI;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc58));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc50));
        c.g[V0][0] = EA_DATA_HI;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003d_e99c);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A3][0]);
        callee::EA_MEM_COPY.call(c, 0x003d_e99c)?;
        c.g[V0][0] = 0x1;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_SetSettings(settings)` (0x3de9b8): installs `settings` as
/// the active file settings.
pub fn ea_file_set_settings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 36) as u64;
        let skip = c.g[V0][0] == 0;
        c.g[S1][0] = EA_DATA_HI;
        if !skip {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
            c.g[RA][0] = sx(0x003d_e9e8);
            c.g[A1][0] = 0x24;
            callee::EA_MEM_ZERO.call(c, 0x003d_e9e8)?;
        }
        c.cycles += 24;
        c.g[A1][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc10));
        c.g[A2][0] = c.lw(c.g[A1][0] as u32);
        c.g[RA][0] = sx(0x003d_e9fc);
        c.g[A0][0] = c.g[S0][0];
        callee::EA_MEM_COPY.call(c, 0x003d_e9fc)?;
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = EA_DATA_HI;
        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc58));
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = EA_DATA_HI;
        c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = 0x1;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw((c.g[A1][0] as u32).wrapping_add(0xc58), c.g[A3][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.sw((c.g[A2][0] as u32).wrapping_add(0xc50), c.g[T2][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[T1][0]);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_OnChunkTransferred(kind, request, result)` (0x3dea48):
/// accumulates a transferred chunk into the request and runs its callback.
pub fn ea_file_on_chunk_transferred(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ea64);
        c.g[S1][0] = c.g[A2][0];
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ea64)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 0x1;
        let not_chunk = c.g[S0][0] != c.g[V0][0];
        c.sw((c.g[S1][0] as u32).wrapping_add(0x20), 0);
        if not_chunk {
            c.cycles += 2;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0);
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 13;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V1][0] = ((c.g[A0][0] as i64)
                < (c.lw((c.g[S1][0] as u32).wrapping_add(0x14)) as i64))
                as u64;
            if c.g[V1][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            let finished = (c.g[V0][0] as i64) <= 0;
            frame.restore(c, S0, 0x20);
            if !finished {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x8000;
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let fits = c.g[V0][0] == 0;
                if !fits {
                    c.cycles += 3;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                }
                c.cycles += 11;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = c.g[S1][0];
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[T0][0] = c.lw(c.g[S1][0] as u32);
                let chunk_cb = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003d_eb18);
                c.pc = chunk_cb;
                call_ptr(c, chunk_cb, 0x003d_eb18)?;
                let missing = c.g[V0][0] == 0;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                if missing {
                    c.cycles += 2;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), 0);
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = 0x3e0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003d_eb30);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15b8));
                    callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_eb30)?;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_TransferChunked(...)` (0x3deb50): runs one chunk of a
/// split transfer through its callback, then waits for the request.
pub fn ea_file_transfer_chunked(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003d_ebc0;
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = c.g[T1][0];
        c.sw(c.g[SP][0] as u32, c.g[T0][0]);
        c.g[T1][0] = 0x8000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A3][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A3][0]);
        frame.save_ra(c, 0x30);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A2][0]);
        c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        if !fits {
            c.cycles += 1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[T1][0]);
        }
        'b_3debdc: {
            c.cycles += 7;
            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            c.g[T1][0] = c.g[SP][0];
            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
            let chunk_cb = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x003d_eba4);
            c.pc = chunk_cb;
            call_ptr(c, chunk_cb, 0x003d_eba4)?;
            let null_cb = c.g[V0][0] == 0;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            if !null_cb {
                c.cycles += 5;
                c.g[A1][0] = 0x3e0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x003d_ebbc);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x15b8));
                callee::EA_FILE_SET_REQUEST_CALLBACK.call(c, 0x003d_ebbc)?;
                loop {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003d_ebc8);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ebc8)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    let more_cb = c.g[V1][0] != 0;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                    if !more_cb {
                        c.cycles += 2;
                        let more_req = c.g[V0][0] != 0;
                        if !more_req {
                            break 'b_3debdc;
                        }
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
        }
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Open(path, mode, out_result)` (0x3debf0): queues an open,
/// waits, and reports status through `out_result`.
pub fn ea_file_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ec14);
        c.g[S2][0] = 0;
        callee::EA_FILE_QUEUE_OPEN.call(c, 0x003d_ec14)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.sw(c.g[S1][0] as u32, 0);
        } else {
            c.cycles += 9;
            c.g[RA][0] = sx(0x003d_ec28);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ec28)?;
            c.g[RA][0] = sx(0x003d_ec30);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_GET_REQUEST_STATUS.call(c, 0x003d_ec30)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003d_ec40);
            c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ec40)?;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Read(...)` (0x3dec60): a chunked transfer with the read
/// callback selected.
pub fn ea_file_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T1][0] = 0x3e0000;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ec74);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x29b8));
        callee::EA_FILE_TRANSFER_CHUNKED.call(c, 0x003d_ec74)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Close(handle)` (0x3deca0): queues a close, waits, and
/// reports whether anything was released.
pub fn ea_file_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ecbc);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_CLOSE.call(c, 0x003d_ecbc)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_ecd0);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ecd0)?;
            c.g[RA][0] = sx(0x003d_ecd8);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ecd8)?;
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetSize(handle)` (0x3decf8): queues a size query, waits,
/// and returns the released byte count.
pub fn ea_file_get_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ed14);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_GET_SIZE.call(c, 0x003d_ed14)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_ed28);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ed28)?;
            c.g[RA][0] = sx(0x003d_ed30);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ed30)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_MountBig(path, mode, out_result)` (0x3ded50): queues a big
/// mount, waits, and reports status through `out_result`.
pub fn ea_file_mount_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ed74);
        c.g[S2][0] = 0;
        callee::EA_FILE_QUEUE_MOUNT_BIG.call(c, 0x003d_ed74)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.sw(c.g[S1][0] as u32, 0);
        } else {
            c.cycles += 9;
            c.g[RA][0] = sx(0x003d_ed88);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ed88)?;
            c.g[RA][0] = sx(0x003d_ed90);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_GET_REQUEST_STATUS.call(c, 0x003d_ed90)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003d_eda0);
            c.g[S2][0] = (c.g[V0][0] < 0x1u64) as u64;
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_eda0)?;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_UnmountBig(handle)` (0x3dedc0): queues an unmount, waits,
/// and reports whether anything was released.
pub fn ea_file_unmount_big(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_eddc);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_UNMOUNT_BIG.call(c, 0x003d_eddc)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_edf0);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_edf0)?;
            c.g[RA][0] = sx(0x003d_edf8);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_edf8)?;
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_Exists(path)` (0x3dee18): queues an existence check, waits,
/// and reports whether anything was released.
pub fn ea_file_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003d_ee34);
        c.g[S1][0] = 0;
        callee::EA_FILE_QUEUE_EXISTS.call(c, 0x003d_ee34)?;
        c.g[S0][0] = c.g[V0][0];
        let null = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !null {
            c.cycles += 6;
            c.g[RA][0] = sx(0x003d_ee48);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_WAIT_REQUEST.call(c, 0x003d_ee48)?;
            c.g[RA][0] = sx(0x003d_ee50);
            c.g[A0][0] = c.g[S0][0];
            callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_ee50)?;
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ExistsDefault(path)` (0x3e1700): existence check with
/// the default settings word.
pub fn ea_file_exists_default(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = EA_DATA_HI;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_1718);
        callee::EA_FILE_EXISTS.call(c, 0x003e_1718)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetFileSizeWorker(path, ...)` (0x3e1728): opens the file,
/// reads its size and closes it; 0 when the open fails.
pub fn ea_file_get_file_size_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
        c.g[RA][0] = sx(0x003e_1750);
        c.g[A1][0] = 0x1;
        callee::EA_FILE_OPEN.call(c, 0x003e_1750)?;
        let opened = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        if opened {
            c.cycles += 8;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x003e_176c);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_GET_SIZE.call(c, 0x003e_176c)?;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003e_177c);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_CLOSE.call(c, 0x003e_177c)?;
            c.g[V0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetFileSizeEx(path, size_out)` (0x3e1798): size query
/// through the priority-call gate.
pub fn ea_file_get_file_size_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = 0x3e0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1728));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x003e_17c8);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_17c8)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetFileSize(path, size_out)` (0x3e17d8): size query
/// without the priority flag.
pub fn ea_file_get_file_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A1][0] = 0;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = 0x3e0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1728));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1804);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1804)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileWorker(path, ...)` (0x3e1810): opens the file,
/// allocates a buffer, reads it fully and closes; 0 on any failure.
pub fn ea_file_load_file_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003e_1840);
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        callee::EA_FILE_OPEN.call(c, 0x003e_1840)?;
        let failed = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        // The success path returns the buffer directly; every other
        // path runs the shared +1/null tail.
        let mut done = false;
        if !failed {
            c.cycles += 13;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x003e_1854);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_GET_SIZE.call(c, 0x003e_1854)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V1][0] = EA_DATA_HI;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc14));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003e_1870);
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            c.pc = alloc;
            call_ptr(c, alloc, 0x003e_1870)?;
            c.g[S0][0] = c.g[V0][0];
            let no_mem = c.g[S0][0] == 0;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            if no_mem {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003e_18ac);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_18ac)?;
            } else {
                c.cycles += 10;
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003e_1890);
                c.g[T0][0] = c.g[S1][0];
                callee::EA_FILE_READ.call(c, 0x003e_1890)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_189c);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_189c)?;
                c.g[V0][0] = c.g[S0][0];
                done = true;
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileEx(path, ...)` (0x3e18c8): load through the
/// priority-call gate with the allocate flag set.
pub fn ea_file_load_file_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1810));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[RA][0] = sx(0x003e_18fc);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_18fc)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFile(path, ...)` (0x3e1908): load through the
/// priority-call gate without the allocate flag.
pub fn ea_file_load_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1810));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1938);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1938)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileGetSizeWorker(path, ...)` (0x3e1948): opens the
/// file, allocates a buffer, reads it, records the size and closes.
pub fn ea_file_load_file_get_size_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003e_197c);
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        callee::EA_FILE_OPEN.call(c, 0x003e_197c)?;
        let failed = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        let mut done = false;
        if !failed {
            c.cycles += 13;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[RA][0] = sx(0x003e_1990);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_FILE_GET_SIZE.call(c, 0x003e_1990)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V1][0] = EA_DATA_HI;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc14));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
            let alloc = c.g[V0][0] as u32;
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x003e_19ac);
            c.pc = alloc;
            call_ptr(c, alloc, 0x003e_19ac)?;
            c.g[S0][0] = c.g[V0][0];
            let no_mem = c.g[S0][0] == 0;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            if no_mem {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003e_19ec);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_19ec)?;
            } else {
                c.cycles += 11;
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x003e_19cc);
                c.g[T0][0] = c.g[S1][0];
                callee::EA_FILE_READ.call(c, 0x003e_19cc)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_19d8);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_FILE_CLOSE.call(c, 0x003e_19d8)?;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                c.g[V0][0] = c.g[S0][0];
                done = true;
            }
        }
        if !done {
            c.cycles += 1;
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
    })
}

/// `u32 EAFile_LoadFileGetSizeEx(path, size_out, ...)` (0x3e1a10):
/// get-size-and-load through the priority-call gate.
pub fn ea_file_load_file_get_size_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x3e0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1948));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0x20);
        c.g[RA][0] = sx(0x003e_1a4c);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1a4c)?;
        let missing = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !missing {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileGetSize(path, size_out, ...)` (0x3e1a70): same
/// without the priority flag.
pub fn ea_file_load_file_get_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = c.g[SP][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x3e0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
        c.g[S0][0] = c.g[A1][0];
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1948));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1aa8);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1aa8)?;
        let missing = c.g[S0][0] == 0;
        c.g[V1][0] = c.g[V0][0];
        if !missing {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileIntoWorker(path, ...)` (0x3e1ad0): opens the
/// file, checks the caller's size and reads into its buffer.
pub fn ea_file_load_file_into_worker(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x7fff0000;
        frame.save(c, S0, 0x40);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A1][0] = 0x1;
        c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A3][0] = c.g[SP][0];
        c.g[V1][0] = ((0u64 as i64) < (c.g[S1][0] as i64)) as u64;
        c.g[RA][0] = sx(0x003e_1b14);
        if c.g[V1][0] == 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        callee::EA_FILE_OPEN.call(c, 0x003e_1b14)?;
        let opened = c.g[V0][0] != 0;
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        let mut done = false;
        if opened {
            c.cycles += 11;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = c.g[S1][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003e_1b3c);
            c.g[A1][0] = 0;
            callee::EA_FILE_READ.call(c, 0x003e_1b3c)?;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[S1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003e_1b4c);
            c.g[A1][0] = c.g[S0][0];
            callee::EA_FILE_CLOSE.call(c, 0x003e_1b4c)?;
            c.g[V0][0] = c.g[S1][0];
            done = true;
        }
        if !done {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileIntoEx(path, ...)` (0x3e1b68): into-buffer load
/// through the priority-call gate.
pub fn ea_file_load_file_into_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A3][0] = c.g[A2][0];
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = EA_DATA_HI;
        c.g[V1][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ad0));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[A3][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_1ba8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1ba8)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFileInto(path, dst, max)` (0x3e1bb8): runs
/// `EAFile_LoadFileIntoWorker` through [`ea_file_call_with_priority`] with
/// the default priority (`[0x450c38]`) and no flags.
pub fn ea_file_load_file_into(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A2][0];
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = EA_DATA_HI;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ad0));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A3][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        c.g[RA][0] = sx(0x003e_1bf4);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1bf4)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFilesWorker(list, ...)` (0x3e1c00): loads every file
/// in a count-prefixed list, recording each buffer; stops early on a
/// null buffer and reports whether all loads succeeded.
pub fn ea_file_load_files_worker(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003e_1c30;
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0x1;
        if !empty {
            c.cycles += 1;
            loop {
                c.cycles += 17;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[RA][0] = sx(0x003e_1c4c);
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                callee::EA_FILE_LOAD_FILE.call(c, 0x003e_1c4c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                if c.g[V1][0] == 0 {
                    c.g[S3][0] = 0;
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, HEAD);
                }
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFilesEx(count, ...)` (0x3e1c98): file-list load
/// through the priority-call gate.
pub fn ea_file_load_files_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T1][0] = c.g[A2][0];
        let empty = (c.g[T0][0] as i64) <= 0;
        c.g[T2][0] = c.g[A3][0];
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 16;
            c.g[V0][0] = EA_DATA_HI;
            c.g[V1][0] = 0x1;
            c.sw(c.g[SP][0] as u32, c.g[T0][0]);
            c.g[A0][0] = 0x3e0000;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c00));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.g[A1][0] = 0;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
            c.g[A3][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[T2][0]);
            c.g[RA][0] = sx(0x003e_1ce4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1ce4)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadFiles(count, ...)` (0x3e1d00): same without the
/// priority flag.
pub fn ea_file_load_files(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T0][0] = c.g[A1][0];
        c.g[T1][0] = c.g[A2][0];
        let empty = (c.g[V1][0] as i64) <= 0;
        c.g[T2][0] = c.g[A3][0];
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 15;
            c.g[V0][0] = EA_DATA_HI;
            c.g[A0][0] = 0x3e0000;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c00));
            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
            c.g[A1][0] = 0;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[T0][0]);
            c.g[A3][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[T2][0]);
            c.g[RA][0] = sx(0x003e_1d4c);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0);
            callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1d4c)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void *EAFile_LoadBigHeaderWorker(prio, params)` (0x3e1d68): opens the
/// file `params[0]`, reads the first 0xa90 bytes into a buffer from the
/// allocator callback (`params[2]` its third argument) and, when it is a
/// big file whose header is longer, moves it into a header-sized buffer
/// and reads the rest. Returns the buffer, or null on any failure.
///
/// Dispatcher-shaped in the original: the no-second-buffer exit is a back
/// edge to the free block `0x3e1df8`, a resume entry after an unwind.
pub fn ea_file_load_big_header_worker(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_1d68 => {
                    c.cycles += 17;
                    Frame::open(c, 0x80);
                    frame.save(c, S0, 0x70);
                    c.g[A3][0] = c.g[SP][0];
                    frame.save(c, S3, 0x40);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S1, 0x60);
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, S2, 0x50);
                    c.g[A1][0] = 0x1;
                    frame.save(c, S4, 0x30);
                    c.g[A2][0] = c.g[S0][0];
                    frame.save(c, S5, 0x20);
                    frame.save_ra(c, 0x10);
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    callee::EA_FILE_OPEN.call(c, 0x003e_1da4)?;
                    let failed = c.g[V0][0] == 0;
                    c.g[V0][0] = EA_DATA_HI;
                    pc = if failed { 0x003e_1ea0 } else { 0x003e_1dac };
                }
                0x003e_1dac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                    let alloc = c.g[V0][0] as u32;
                    c.g[A1][0] = 0xa90;
                    call_ptr(c, alloc, 0x003e_1dc4)?;
                    c.g[S1][0] = c.g[V0][0];
                    let no_buffer = c.g[S1][0] == 0;
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    pc = if no_buffer { 0x003e_1e90 } else { 0x003e_1dd0 };
                }
                0x003e_1dd0 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[S4][0] = c.g[T0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A3][0] = 0xa90;
                    callee::EA_FILE_READ.call(c, 0x003e_1de8)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::BIG_FILE_GET_TYPE.call(c, 0x003e_1df0)?;
                    pc = if c.g[V0][0] != 0 {
                        0x003e_1e10
                    } else {
                        0x003e_1df8
                    };
                }
                0x003e_1df8 => {
                    // Not a big file, or no header buffer: free the first
                    // buffer and close.
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    let free = c.g[V0][0] as u32;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, free, 0x003e_1e04)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.g[S4][0];
                    pc = 0x003e_1e98;
                }
                0x003e_1e10 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    callee::BIG_FILE_GET_HEADER_SIZE.call(c, 0x003e_1e18)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2705) as u64;
                    let fits = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    pc = if fits { 0x003e_1e80 } else { 0x003e_1e28 };
                }
                0x003e_1e28 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                    let alloc = c.g[V0][0] as u32;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    call_ptr(c, alloc, 0x003e_1e3c)?;
                    c.g[S0][0] = c.g[V0][0];
                    let no_header = c.g[S0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    if no_header {
                        if back_edge_unwinds(c, 0x003e_1df8) {
                            return Ok(());
                        }
                        pc = 0x003e_1df8;
                    } else {
                        pc = 0x003e_1e48;
                    }
                }
                0x003e_1e48 => {
                    c.cycles += 14;
                    c.g[A2][0] = 0xa90;
                    c.g[A0][0] = c.g[S0][0];
                    callee::EA_MEM_COPY.call(c, 0x003e_1e54)?;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S1][0];
                    let free = c.g[V0][0] as u32;
                    c.g[S1][0] = c.g[S0][0];
                    call_ptr(c, free, 0x003e_1e64)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xa90));
                    c.g[A1][0] = 0xa90;
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa90));
                    c.g[T0][0] = c.g[S4][0];
                    callee::EA_FILE_READ.call(c, 0x003e_1e7c)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_1e80;
                }
                0x003e_1e80 => {
                    // Close and return the header buffer.
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S4][0];
                    callee::EA_FILE_CLOSE.call(c, 0x003e_1e88)?;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x003e_1ea4;
                }
                0x003e_1e90 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    pc = 0x003e_1e98;
                }
                0x003e_1e98 => {
                    c.cycles += 2;
                    callee::EA_FILE_CLOSE.call(c, 0x003e_1ea0)?;
                    pc = 0x003e_1ea0;
                }
                0x003e_1ea0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = 0x003e_1ea4;
                }
                0x003e_1ea4 => {
                    c.cycles += 9;
                    frame.restore(c, S0, 0x70);
                    frame.restore(c, S1, 0x60);
                    frame.restore(c, S2, 0x50);
                    frame.restore(c, S3, 0x40);
                    frame.restore(c, S4, 0x30);
                    frame.restore(c, S5, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_1d68, 0x003e_1ec8);
                    return Ok(());
                }
            }
        }
    })
}

/// `u32 EAFile_LoadBigHeaderEx(path, ...)` (0x3e1ec8): header load
/// through the priority-call gate.
pub fn ea_file_load_big_header_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V1][0] = 0x1;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d68));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[RA][0] = sx(0x003e_1efc);
        c.g[A3][0] = c.g[SP][0];
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1efc)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadBigHeader(path, ...)` (0x3e1f08): same without the
/// priority flag.
pub fn ea_file_load_big_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = EA_DATA_HI;
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[A3][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
        c.g[A0][0] = 0x3e0000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc38));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d68));
        frame.save_ra(c, 0x20);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x003e_1f38);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0);
        callee::EA_FILE_CALL_WITH_PRIORITY.call(c, 0x003e_1f38)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressed(path, ...)` (0x3e2190): loads a
/// file, checks its unpacked size and inflates it into a fresh buffer.
pub fn ea_file_load_decompressed(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_98: u32 = 0x003e_2298;
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S5, 0x30);
        frame.save(c, S1, 0x70);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[S5][0] ^ 0x100;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S6, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_21cc);
        c.g[A2][0] = c.g[S1][0];
        callee::EA_FILE_LOAD_FILE_GET_SIZE.call(c, 0x003e_21cc)?;
        c.g[S0][0] = c.g[V0][0];
        let no_mem = c.g[S0][0] == 0;
        c.g[V0][0] = 0;
        'b_3e22c4: {
            if !no_mem {
                'b_3e22c0: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_21e0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, callee::EAC_GET_UNPACKED_SIZE, 0x003e_21e0)?;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    let no_size = c.g[A1][0] == 0;
                    c.g[S4][0] = c.g[V0][0];
                    if !no_size {
                        c.cycles += 2;
                        let empty = c.g[S4][0] == 0;
                        c.g[V0][0] = EA_DATA_HI;
                        if !empty {
                            'b_3e222c: {
                                c.cycles += 9;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[S6][0] = c.g[V0][0];
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                let alloc = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003e_220c);
                                c.g[A0][0] = c.g[S2][0];
                                c.pc = alloc;
                                call_ptr(c, alloc, 0x003e_220c)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                                if c.g[S1][0] != 0 {
                                    break 'b_3e222c;
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                let free = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x003e_2224);
                                c.g[A0][0] = c.g[S0][0];
                                c.pc = free;
                                call_ptr(c, free, 0x003e_2224)?;
                                c.g[V0][0] = 0;
                                break 'b_3e22c4;
                            }
                            'b_3e2288: {
                                'b_3e2284: {
                                    c.cycles += 14;
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x003e_2238);
                                    c.g[A0][0] = c.g[S1][0];
                                    callee::EA_MEM_COPY.call(c, 0x003e_2238)?;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                    let free = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2244);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.pc = free;
                                    call_ptr(c, free, 0x003e_2244)?;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S4][0];
                                    let alloc = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2258);
                                    c.g[A2][0] = c.g[S5][0];
                                    call_ptr(c, alloc, 0x003e_2258)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let no_buf = c.g[S0][0] == 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    if no_buf {
                                        break 'b_3e2284;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x003e_226c);
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, callee::REFPACK_UNPACK, 0x003e_226c)?;
                                    let ok = c.g[V0][0] != 0;
                                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc10));
                                    if ok {
                                        break 'b_3e2288;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                    let free3 = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2284);
                                    c.g[S0][0] = 0;
                                    c.pc = free3;
                                    call_ptr(c, free3, 0x003e_2284)?;
                                    break 'b_3e2284;
                                }
                                c.cycles += 1;
                                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0xc10));
                                break 'b_3e2288;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_2294);
                            c.g[A0][0] = c.g[S1][0];
                            c.pc = free;
                            call_ptr(c, free, 0x003e_2294)?;
                            c.g[T0][0] = 0x0;
                            'l_3e2298: loop {
                                c.cycles += 9;
                                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                                c.g[T1][0] = ((c.g[T0][0] as i64) < 4096) as u64;
                                if c.g[T1][0] != 0 {
                                    if event_due(c) {
                                        deliver_event_here(c, HEAD_98);
                                    }
                                    continue 'l_3e2298;
                                }
                                c.cycles += 1;
                                break 'b_3e22c0;
                            }
                        }
                    }
                }
            }
            if !no_mem {
                // A1==0, S4==0 and cleanup paths converge here.
                c.cycles += 1;
                c.g[V0][0] = c.g[S0][0];
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressedInto(path, buf, ...)` (0x3e2310):
/// loads a file and inflates it into the caller's buffer.
pub fn ea_file_load_decompressed_into(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_418: u32 = 0x003e_2418;
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S6, 0x20);
        c.g[S3][0] = c.g[A2][0];
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_234c);
        c.g[A2][0] = 0x100;
        callee::EA_FILE_LOAD_FILE_GET_SIZE.call(c, 0x003e_234c)?;
        c.g[S1][0] = c.g[V0][0];
        let no_mem = c.g[S1][0] == 0;
        c.g[V0][0] = 0;
        'b_3e2444: {
            if !no_mem {
                'b_3e23e0: {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003e_2364);
                    c.g[S5][0] = EA_DATA_HI;
                    call_ptr(c, callee::EAC_GET_UNPACKED_SIZE, 0x003e_2364)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    let empty = (c.g[V0][0] as i64) <= 0;
                    if empty {
                        break 'b_3e23e0;
                    }
                    c.cycles += 2;
                    let bad_size = (c.g[S2][0] as i64) <= 0;
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                    if bad_size {
                        break 'b_3e23e0;
                    }
                    'b_3e23c0: {
                        'b_3e23bc: {
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                            c.g[A1][0] = c.g[S2][0];
                            let alloc = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_2390);
                            c.g[A2][0] = 0x100;
                            c.pc = alloc;
                            call_ptr(c, alloc, 0x003e_2390)?;
                            c.g[S0][0] = c.g[V0][0];
                            let no_buf = c.g[S0][0] == 0;
                            c.g[A0][0] = c.g[S1][0];
                            if no_buf {
                                break 'b_3e23bc;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e_23a4);
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, callee::REFPACK_UNPACK, 0x003e_23a4)?;
                            let ok = c.g[V0][0] != 0;
                            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                            if ok {
                                break 'b_3e23c0;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_23bc);
                            c.g[S0][0] = 0;
                            c.pc = free;
                            call_ptr(c, free, 0x003e_23bc)?;
                            break 'b_3e23bc;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                        break 'b_3e23c0;
                    }
                    'b_3e23dc: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        let free = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x003e_23cc);
                        c.g[A0][0] = c.g[S1][0];
                        c.pc = free;
                        call_ptr(c, free, 0x003e_23cc)?;
                        if c.g[S0][0] != 0 {
                            c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                            break 'b_3e23dc;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        break 'b_3e2444;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = c.g[S0][0];
                    break 'b_3e23e0;
                }
                {
                    c.cycles += 2;
                    let empty = (c.g[S3][0] as i64) <= 0;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if !empty {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.sw(c.g[SP][0] as u32, c.g[S3][0]);
                        }
                    }
                }
                c.cycles += 9;
                c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x003e_2404);
                c.g[A1][0] = c.g[S1][0];
                callee::EA_MEM_COPY.call(c, 0x003e_2404)?;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                let free = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003e_2414);
                c.g[A0][0] = c.g[S1][0];
                c.pc = free;
                call_ptr(c, free, 0x003e_2414)?;
                c.g[T0][0] = 0x0;
                'l_3e2418: loop {
                    c.cycles += 9;
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
                    c.g[T1][0] = ((c.g[T0][0] as i64) < 4096) as u64;
                    if c.g[T1][0] != 0 {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_418);
                        }
                        continue 'l_3e2418;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    break 'b_3e2444;
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_GetUnpackedSize(path, ...)` (0x3e2130): loads a file
/// into a scratch buffer and reads its unpacked size; 0 on failure.
pub fn ea_file_get_unpacked_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x90);
        c.g[A2][0] = 0x80;
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_2144);
        callee::EA_FILE_LOAD_FILE_INTO.call(c, 0x003e_2144)?;
        let failed = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !failed {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_2154);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, callee::EAC_GET_UNPACKED_SIZE, 0x003e_2154)?;
            frame.restore_ra(c, 0x80);
        } else {
            c.cycles += 1;
            frame.restore_ra(c, 0x80);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ReadUnpackedSize(path, ...)` (0x3e2168): same reader
/// without the null check.
pub fn ea_file_read_unpacked_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x90);
        c.g[A2][0] = 0x80;
        frame.save_ra(c, 0x80);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_217c);
        callee::EA_FILE_LOAD_FILE_INTO.call(c, 0x003e_217c)?;
        c.g[RA][0] = sx(0x003e_2184);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, callee::EAC_GET_UNPACKED_SIZE, 0x003e_2184)?;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void *EAFile_LoadDecompressedThunk(path, flags)` (0x3e22f0): calls
/// `EAFile_LoadDecompressed` through its own frame (gcc's forwarding
/// wrapper).
pub fn ea_file_load_decompressed_thunk(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::EA_FILE_LOAD_DECOMPRESSED),
        0x003e_2300,
    );
}

/// `void *EAFile_LoadDecompressedIntoThunk(path, dst, max)` (0x3e2470):
/// the same wrapper for `EAFile_LoadDecompressedInto`.
pub fn ea_file_load_decompressed_into_thunk(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::EA_FILE_LOAD_DECOMPRESSED_INTO),
        0x003e_2480,
    );
}

/// `void EAFile_PlatformInit(use_cd)` (0x3e3020): timer callback setup
/// and boot flag writes.
pub fn ea_file_platform_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x3e0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33b0));
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = EA_DATA_HI;
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), 0);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x003e_3050);
        c.g[A2][0] = 0;
        callee::EA_TIMER_ADD_CALLBACK.call(c, 0x003e_3050)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc48));
        c.g[V0][0] = c.g[V1][0] | 0x2;
        let cold = c.g[S0][0] == 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), c.g[V0][0]);
        'b_3e3084: {
            'b_3e3078: {
                if cold {
                    break 'b_3e3078;
                }
                c.cycles += 6;
                c.g[V0][0] = c.g[V1][0] | 0x3;
                c.g[A0][0] = EA_DATA_HI;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), c.g[V1][0]);
                break 'b_3e3084;
            }
            c.cycles += 3;
            c.g[V1][0] = EA_DATA_HI;
            c.g[V0][0] = 0x2;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc44), c.g[V0][0]);
            break 'b_3e3084;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_PlatformWrite(fd, buf, offset, len)` (0x3e3478): seek
/// the host file (`fd & 0xffffff`) to `offset`, write `len` bytes of `buf`
/// and complete the request; returns 0.
pub fn ea_file_platform_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x003e_34b0);
        c.g[A2][0] = 0;
        call_ptr(c, callee::SCE_LSEEK, 0x003e_34b0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003e_34c0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::SCE_WRITE, 0x003e_34c0)?;
        c.g[RA][0] = sx(0x003e_34c8);
        c.g[A0][0] = 0x1;
        callee::EA_FILE_COMPLETE_REQUEST.call(c, 0x003e_34c8)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_PlatformGetLsn(drive_fd)` (0x3e3538): LSN of the disc
/// entry for host drives, 0 otherwise.
pub fn ea_file_platform_get_lsn(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[V0][0] = 0xff0000;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = 0x1;
        let host = c.g[A1][0] != c.g[V1][0];
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        'b_3e3578: {
            'b_3e3570: {
                if host {
                    break 'b_3e3570;
                }
                c.cycles += 6;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[RA][0] = sx(0x003e_3568);
                c.g[A3][0] = c.g[SP][0];
                callee::EA_FILE_CD_GET_ENTRY.call(c, 0x003e_3568)?;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e3578;
            }
            c.cycles += 2;
            c.sw(c.g[SP][0] as u32, 0);
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3e3578;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_AlarmHandler(arg)` (0x3e3588): host alarm ack plus
/// an EE interrupt enable.
pub fn ea_file_alarm_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3598);
        c.g[A0][0] = c.g[A2][0];
        call_ptr(c, callee::I_SIGNAL_SEMA, 0x003e_3598)?;
        c.ei();
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAFile_DelayTicks(ticks)` (0x3e35b0): timer-primitive wait.
pub fn ea_file_delay_ticks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0] & 0xffff;
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0);
        c.g[RA][0] = sx(0x003e_35dc);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0);
        call_ptr(c, callee::CREATE_SEMA, 0x003e_35dc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3e0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
        c.g[RA][0] = sx(0x003e_35f4);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::SET_ALARM, 0x003e_35f4)?;
        c.g[RA][0] = sx(0x003e_35fc);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::WAIT_SEMA, 0x003e_35fc)?;
        c.g[RA][0] = sx(0x003e_3604);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::DELETE_SEMA, 0x003e_3604)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_HashName(name, seed)` (0x3e36f8): `h = h * 33 + byte`
/// (signed bytes) from the last byte of `name` to the first, starting at
/// `seed`.
pub fn ea_file_hash_name(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_20: u32 = 0x003e_3720;
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3714);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, callee::STRLEN, 0x003e_3714)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        'l_3e3720: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lb(c.g[A2][0] as u32);
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 5);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = (c.g[A2][0] < c.g[S1][0]) as u64;
            let more = c.g[A0][0] == 0;
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if more {
                if event_due(c) {
                    deliver_event_here(c, HEAD_20);
                }
                continue 'l_3e3720;
            }
            c.cycles += 6;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S1, 0x10);
            frame.restore(c, S0, 0x20);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void EAFile_CdSignal(arg)` (0x3e3ad8): host signal ack.
pub fn ea_file_cd_signal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = EA_BSS_HI;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x63b4));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3af0);
        call_ptr(c, callee::I_SIGNAL_SEMA, 0x003e_3af0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_ResolvePath(src, dst, status_out)` (0x3e3098): split a
/// `host:name` path, compare the host prefix, and join the remainder.
pub fn ea_file_resolve_path(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_B8: u32 = 0x003e_31b8;
        c.cycles += 19;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        c.g[S5][0] = EA_DATA_HI;
        frame.save(c, S2, 0x50);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S4, 0x30);
        c.g[A1][0] = 0x3a;
        frame.save_ra(c, 0);
        c.sb(c.g[S0][0] as u32, 0);
        c.g[RA][0] = sx(0x003e_30dc);
        c.g[S3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc44));
        call_ptr(c, callee::STRCHR, 0x003e_30dc)?;
        c.g[S4][0] = EA_DATA_HI;
        'b_3e31e4: {
            'b_3e31e0: {
                'b_3e3150: {
                    if c.g[V0][0] == 0 {
                        break 'b_3e3150;
                    }
                    'b_3e3118: {
                        c.cycles += 10;
                        c.g[RA][0] = sx(0x003e_30ec);
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc50));
                        call_ptr(c, callee::STRLEN, 0x003e_30ec)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V0][0] = EA_DATA_HI;
                        c.g[A2][0] = 0x3;
                        c.g[RA][0] = sx(0x003e_3104);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc3c));
                        call_ptr(c, callee::STRNCMP, 0x003e_3104)?;
                        c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc50));
                        if c.g[V0][0] != 0 {
                            break 'b_3e3118;
                        }
                        c.cycles += 3;
                        c.g[S3][0] = 0x1;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3));
                        break 'b_3e3150;
                    }
                    'b_3e3138: {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x003e_3124);
                        c.g[A2][0] = c.g[S2][0];
                        call_ptr(c, callee::STRNCMP, 0x003e_3124)?;
                        c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc44));
                        if c.g[V0][0] != 0 {
                            break 'b_3e3138;
                        }
                        c.cycles += 3;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.g[S3][0] = 0x2;
                        break 'b_3e3150;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let is_boot = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x3a;
                    if is_boot {
                        break 'b_3e31e0;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x2));
                    if c.g[V1][0] != c.g[V0][0] {
                        frame.restore(c, S0, 0x70);
                        break 'b_3e31e4;
                    }
                    break 'b_3e3150;
                }
                {
                    // S3 != 2 skips the host combine; both arms converge.
                    c.cycles += 3;
                    c.g[V0][0] = 0x2;
                    if c.g[S3][0] != c.g[V0][0] {
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = EA_DATA_HI;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003e_316c);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc50));
                        call_ptr(c, callee::STRCAT, 0x003e_316c)?;
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
                    }
                }
                'b_3e3198: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5c;
                    let is_bs = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x2f;
                    if is_bs {
                        break 'b_3e3198;
                    }
                    c.cycles += 2;
                    let is_sl = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7e;
                    if is_sl {
                        break 'b_3e3198;
                    }
                    c.cycles += 2;
                    let is_tl = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = EA_BSS_HI;
                    if is_tl {
                        break 'b_3e3198;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e_3198);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6510));
                    call_ptr(c, callee::STRCAT, 0x003e_3198)?;
                    break 'b_3e3198;
                }
                'b_3e31d8: {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e_31a4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, callee::STRCAT, 0x003e_31a4)?;
                    c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                    let empty = c.g[V1][0] == 0;
                    c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                    if empty {
                        break 'b_3e31d8;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0x5c;
                    c.g[V1][0] = 0x2f;
                    'l_3e31b8: loop {
                        'b_3e31c8: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 24);
                            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                            if c.g[V0][0] == c.g[A1][0] {
                                c.sb(c.g[S0][0] as u32, c.g[V1][0]);
                            }
                            break 'b_3e31c8;
                        }
                        c.cycles += 4;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lbu(c.g[S0][0] as u32);
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, HEAD_B8);
                            }
                            continue 'l_3e31b8;
                        }
                        break 'b_3e31d8;
                    }
                }
                c.cycles += 2;
                if c.g[S6][0] != 0 {
                    c.sw(c.g[S6][0] as u32, c.g[S3][0]);
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 9;
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

/// `u32 EAFile_CdBuildHashTable(src, dst, n, seed)` (0x3e39a8): hash the
/// `n` source names (stride 0x108) into `dst` (hash, two words, and the
/// name when `[0x450c5c]` > 0), sort by hash, and redo it with a new seed
/// while two hashes collide; returns the seed used.
pub fn ea_file_cd_build_hash_table(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_A08: u32 = 0x003e_3a08;
        const HEAD_9F0: u32 = 0x003e_39f0;
        c.cycles += 18;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        frame.save(c, S5, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = EA_BSS_HI;
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = 0;
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x10);
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[S1][0] = c.g[S6][0];
        'l_3e39f0: loop {
            'b_3e3a54: {
                c.cycles += 2;
                let empty = (c.g[S5][0] as i64) <= 0;
                c.g[S0][0] = c.lw(c.g[SP][0] as u32);
                if empty {
                    break 'b_3e3a54;
                }
                c.cycles += 4;
                c.g[V0][0] = EA_DATA_HI;
                c.g[S2][0] = c.g[S5][0];
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
                'l_3e3a08: loop {
                    'b_3e3a40: {
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003e_3a14);
                        c.g[A1][0] = c.g[S3][0];
                        callee::EA_FILE_HASH_NAME.call(c, 0x003e_3a14)?;
                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x100));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x104));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        let no_copy = (c.g[V0][0] as i64) <= 0;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                        if no_copy {
                            break 'b_3e3a40;
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003e_3a3c);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, callee::STRCPY, 0x003e_3a3c)?;
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                        break 'b_3e3a40;
                    }
                    c.cycles += 5;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x108));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc));
                    let more = c.g[S2][0] != 0;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_A08);
                        }
                        continue 'l_3e3a08;
                    }
                    break 'b_3e3a54;
                }
            }
            'b_3e3a98: {
                'b_3e3a94: {
                    c.cycles += 13;
                    c.g[V1][0] = EA_DATA_HI;
                    c.g[V0][0] = 0x3e0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc58));
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3968));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S6][0];
                    c.sw((c.g[S7][0] as u32).wrapping_sub(0x1280), 0);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003e_3a7c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, callee::QSORT, 0x003e_3a7c)?;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_sub(0x1280));
                    let clean = c.g[V0][0] == 0;
                    c.g[V1][0] = 0x65;
                    if clean {
                        break 'b_3e3a94;
                    }
                    c.cycles += 3;
                    c.mult(0, 2, 19, 3);
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7));
                    break 'b_3e3a98;
                }
                c.cycles += 1;
                c.g[FP][0] = 0x1;
                break 'b_3e3a98;
            }
            c.cycles += 2;
            let retry = c.g[FP][0] == 0;
            c.g[S1][0] = c.g[S6][0];
            if retry {
                if event_due(c) {
                    deliver_event_here(c, HEAD_9F0);
                }
                continue 'l_3e39f0;
            }
            c.cycles += 13;
            c.g[V0][0] = c.g[S3][0];
            frame.restore(c, S0, 0xa0);
            frame.restore(c, S1, 0x90);
            frame.restore(c, S2, 0x80);
            frame.restore(c, S3, 0x70);
            frame.restore(c, S4, 0x60);
            frame.restore(c, S5, 0x50);
            frame.restore(c, S6, 0x40);
            frame.restore(c, S7, 0x30);
            frame.restore(c, FP, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `u32 EAFile_CdRemount()` (0x3e4000): remount the disc if the
/// present flag says it went away.
pub fn ea_file_cd_remount(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = EA_BSS_HI;
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x63c0));
        c.g[V1][0] = 0x6;
        c.g[V0][0] = c.g[A0][0] & 0x6;
        let no_remount = c.g[V0][0] != c.g[V1][0];
        frame.save_ra(c, 0);
        'b_3e402c: {
            if no_remount {
                break 'b_3e402c;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xfffffffffffffff9;
            c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[RA][0] = sx(0x003e_402c);
            c.sw((c.g[A1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
            callee::EA_FILE_CD_MOUNT_DISC.call(c, 0x003e_402c)?;
            break 'b_3e402c;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_CdGetEntry(index, name_out, lba_out, size_out)`
/// (0x3e48d8): table entry fields for a 1-based index.
pub fn ea_file_cd_get_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V1][0] = EA_DATA_HI;
        c.g[V0][0] = EA_BSS_HI;
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc5c));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x6388));
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
        frame.save(c, S0, 0x30);
        c.mult(0, 4, 4, 2);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A3][0];
        let no_name = c.g[A1][0] == 0;
        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        'b_3e4928: {
            if no_name {
                break 'b_3e4928;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[A1][0];
            c.g[A2][0] = c.g[T0][0];
            c.g[RA][0] = sx(0x003e_4928);
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            call_ptr(c, callee::STRNCPY, 0x003e_4928)?;
            break 'b_3e4928;
        }
        'b_3e4938: {
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                break 'b_3e4938;
            }
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            break 'b_3e4938;
        }
        'b_3e494c: {
            c.cycles += 2;
            if c.g[S2][0] == 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                break 'b_3e494c;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            break 'b_3e494c;
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

/// `entry *EAFile_CdFindEntry(name)` (0x3e4968): the table entry for
/// `name` ([`ea_file_cd_lookup`]), or 0 when absent.
pub fn ea_file_cd_find_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_497c);
        c.g[A2][0] = c.g[SP][0];
        callee::EA_FILE_CD_LOOKUP.call(c, 0x003e_497c)?;
        let found = c.g[V0][0] != 0;
        c.g[A0][0] = EA_DATA_HI;
        'b_3e49ac: {
            'b_3e498c: {
                if found {
                    break 'b_3e498c;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'b_3e49ac;
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc5c));
            c.g[A1][0] = EA_BSS_HI;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x6388));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
            c.mult(0, 2, 2, 3);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            break 'b_3e49ac;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_CdRead(entry, dst, offset, count)` (0x3e4648): set up a
/// read of `count` bytes at byte `offset` of CD entry `entry` (1-based)
/// into `dst`: the first pass (a partial sector, a short read, or whole
/// sectors), then either copy it straight from the cached sector and set
/// up the next pass (or complete the request), or wake the reader thread
/// (`SignalSema`). Returns the byte count clamped to the entry.
pub fn ea_file_cd_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        c.g[T0][0] = EA_DATA_HI;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc5c));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        frame.save(c, S0, 0x40);
        c.g[V0][0] = EA_BSS_HI;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.mult(0, 4, 4, 3);
        frame.save(c, S3, 0x10);
        c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
        frame.save_ra(c, 0);
        c.g[T1][0] = c.g[A2][0];
        c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7ff));
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x38));
        c.g[T3][0] = 0xffffffffffffffff;
        c.g[A2][0] = ((c.g[T3][0] as i64) < (c.g[T1][0] as i64)) as u64;
        c.g[V0][0] = c.g[T0][0];
        c.g[T4][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        if c.g[A2][0] != 0 {
            c.g[V0][0] = c.g[T1][0];
        }
        c.g[V1][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 11) as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 11);
        c.g[S0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[T1][0] as u32));
        c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[A0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        c.g[S2][0] = c.g[V1][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
        if c.g[A0][0] == 0 {
            c.g[S2][0] = c.g[A3][0];
        }
        let t = c.g[A2][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A1][0]);
        'b_3e4778: {
            'b_3e4708: {
                if t {
                    break 'b_3e4708;
                }
                'b_3e4700: {
                    'b_3e46fc: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                        c.g[A0][0] = 0x1;
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.g[V0][0] = c.g[V0][0] | 0x1;
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 2049) as u64;
                        let t = c.g[V1][0] != 0;
                        c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                        if t {
                            break 'b_3e46fc;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x800;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                        break 'b_3e4700;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S2][0];
                    break 'b_3e4700;
                }
                c.cycles += 2;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                let t = true;
                if t {
                    break 'b_3e4778;
                } else {
                    break 'b_3e4708;
                }
            }
            'b_3e472c: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 2048) as u64;
                let small = c.g[V0][0] == 0;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                if small {
                    break 'b_3e472c;
                }
                c.cycles += 6;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.g[V1][0] = c.g[V1][0] | 0x1;
                c.sw((c.g[T2][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                let t = true;
                if t {
                    break 'b_3e4778;
                } else {
                    break 'b_3e472c;
                }
            }
            'b_3e4754: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A1][0] & 0x3f;
                let aligned = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                if aligned {
                    break 'b_3e4754;
                }
                c.cycles += 7;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A0][0] = 0x800;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.sw((c.g[T2][0] as u32).wrapping_add(0x20), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                let t = true;
                if t {
                    break 'b_3e4778;
                } else {
                    break 'b_3e4754;
                }
            }
            c.cycles += 9;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
            c.g[V0][0] = 0xfffffffffffffffe;
            c.g[V1][0] = 0xfffffffffffff800;
            c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
            c.g[V1][0] = c.g[S2][0] & c.g[V1][0];
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[A0][0]);
            c.g[A1][0] = sx(((c.g[V1][0] as u32) as i32 >> 11) as u32);
            c.sw((c.g[T2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            break 'b_3e4778;
        }
        'b_3e48bc: {
            'b_3e48a0: {
                'b_3e4898: {
                    c.cycles += 13;
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    c.g[S3][0] = 0xffffffffffffffff;
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[T1][0] as i64)) as u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    if c.g[V1][0] != 0 {
                        c.g[T0][0] = c.g[T1][0];
                    }
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = sx(((c.g[T0][0] as u32) as i32 >> 11) as u32);
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    if c.g[V0][0] != c.g[A0][0] {
                        break 'b_3e4898;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 2049) as u64;
                    let multi = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    if multi {
                        break 'b_3e48a0;
                    }
                    'b_3e4878: {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3c));
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[RA][0] = sx(0x003e_47cc);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        callee::EA_MEM_COPY.call(c, 0x003e_47cc)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        let small = (c.g[A2][0] as i64) <= 0;
                        c.g[A1][0] = ((c.g[A2][0] as i64) < 2048) as u64;
                        if small {
                            break 'b_3e4878;
                        }
                        'b_3e4864: {
                            'b_3e4818: {
                                c.cycles += 9;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0);
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                let full = c.g[A1][0] == 0;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                                if full {
                                    break 'b_3e4818;
                                }
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                                c.g[V0][0] = 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                                c.g[V1][0] = c.g[V1][0] | 0x1;
                                c.sw(c.g[S0][0] as u32, c.g[A2][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                                break 'b_3e4864;
                            }
                            'b_3e4840: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[A0][0] & 0x3f;
                                let aligned = c.g[V0][0] == 0;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                                if aligned {
                                    break 'b_3e4840;
                                }
                                c.cycles += 7;
                                c.g[V1][0] = 0x1;
                                c.g[A0][0] = 0x800;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                c.g[V0][0] = c.g[V0][0] | 0x1;
                                c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                                break 'b_3e4864;
                            }
                            c.cycles += 9;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                            c.g[V0][0] = 0xfffffffffffffffe;
                            c.g[A0][0] = 0xfffffffffffff800;
                            c.g[A0][0] = c.g[A2][0] & c.g[A0][0];
                            c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
                            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                            c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 11) as u32);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                            c.sw(c.g[S0][0] as u32, c.g[A0][0]);
                            break 'b_3e4864;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        let t = true;
                        if t {
                            break 'b_3e4898;
                        } else {
                            break 'b_3e4878;
                        }
                    }
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    c.g[V1][0] = 0xfffffffffffffffd;
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[RA][0] = sx(0x003e_4890);
                    c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                    callee::EA_FILE_COMPLETE_REQUEST.call(c, 0x003e_4890)?;
                    c.g[V0][0] = c.g[S2][0];
                    let t = true;
                    if t {
                        break 'b_3e48bc;
                    } else {
                        break 'b_3e4898;
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                break 'b_3e48a0;
            }
            c.cycles += 7;
            c.g[V1][0] = 0x3;
            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = c.g[V0][0] | 0xa;
            c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[RA][0] = sx(0x003e_48b8);
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
            call_ptr(c, callee::SIGNAL_SEMA, 0x003e_48b8)?;
            c.g[V0][0] = c.g[S2][0];
            break 'b_3e48bc;
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

/// `u32 EAFile_CdLookup(name, _, u32 *index)` (0x3e4508): normalise
/// `name` (a leading `/`, upper case, `\` as `/`), hash it and binary-search
/// the sorted entry table; stores the 1-based index (0 when absent) and
/// returns 1 on a hit, else 0.
pub fn ea_file_cd_lookup(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_68: u32 = 0x003e_4568;
        c.cycles += 19;
        let frame = Frame::open(c, 0x160);
        c.g[A3][0] = c.g[A0][0];
        frame.save(c, S2, 0x130);
        c.g[V0][0] = 0x2f;
        frame.save(c, S3, 0x120);
        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x150);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1));
        frame.save(c, S1, 0x140);
        frame.save_ra(c, 0x110);
        c.sb(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lb(c.g[A3][0] as u32);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = c.g[V1][0] ^ 0x2f;
        if c.g[V1][0] != 0 {
            c.g[A3][0] = c.g[A0][0];
        }
        c.g[V0][0] = c.lb(c.g[A3][0] as u32);
        c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
        'b_3e45a8: {
            if c.g[V0][0] == 0 {
                break 'b_3e45a8;
            }
            c.cycles += 5;
            c.g[V0][0] = 0x4a0000;
            c.g[T2][0] = 0x5c;
            c.g[T1][0] = sx(EAF_CTYPE);
            c.g[T0][0] = 0x2f;
            'l_3e4568: loop {
                'b_3e4594: {
                    'b_3e4580: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 24);
                        c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                        let not_bs = c.g[A1][0] != c.g[T2][0];
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                        if not_bs {
                            break 'b_3e4580;
                        }
                        c.cycles += 2;
                        c.sb(c.g[A2][0] as u32, c.g[T0][0]);
                        break 'b_3e4594;
                    }
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = c.lbu(c.g[A0][0] as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x2;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[A1][0];
                    }
                    c.sb(c.g[A2][0] as u32, c.g[V1][0]);
                    break 'b_3e4594;
                }
                c.cycles += 5;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lb(c.g[A3][0] as u32);
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = c.lbu(c.g[A3][0] as u32);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, HEAD_68);
                    }
                    continue 'l_3e4568;
                }
                break 'b_3e45a8;
            }
        }
        'b_3e462c: {
            'b_3e4624: {
                c.cycles += 20;
                c.sb(c.g[A2][0] as u32, 0);
                c.g[V0][0] = EA_BSS_HI;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = EA_DATA_HI;
                c.g[RA][0] = sx(0x003e_45c8);
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
                callee::EA_FILE_HASH_NAME.call(c, 0x003e_45c8)?;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[T0][0] = 0x3e0000;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3968));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[RA][0] = sx(0x003e_45ec);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                call_ptr(c, callee::BSEARCH, 0x003e_45ec)?;
                c.g[A1][0] = c.g[V0][0];
                let null = c.g[A1][0] == 0;
                c.g[V0][0] = 0x1;
                if null {
                    break 'b_3e4624;
                }
                'b_3e4614: {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.divu(0, 3, 4);
                    // Gen tests A0==0 here, but A0 was just bumped by 0xc
                    // off a mapped word: both arms break out. Kept faithful.
                    if c.g[A0][0] == 0 {
                        break 'b_3e4614;
                    } else {
                        break 'b_3e4614;
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lo[0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw(c.g[S3][0] as u32, c.g[V1][0]);
                break 'b_3e462c;
            }
            c.cycles += 2;
            c.sw(c.g[S3][0] as u32, 0);
            c.g[V0][0] = 0;
            break 'b_3e462c;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00317f38 EAFile_PathExists

/// `int EAFile_PathExists(const char* path)` (0x317f38): whether the file
/// system knows `name`: ask `FILESYS_bypassqueuefileinfo` (0x3de4d0,
/// `EAFile_ParseBigPath`) for it and return whether it answered 1.
pub fn ea_file_path_exists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        callee::EA_FILE_PARSE_BIG_PATH.call(c, 0x0031_7f4c)?;
        let found = c.g[V0][0] ^ 1;
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = (found < 1) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00317f60 EAFile_GetPathSizeChecked

/// `int EAFile_GetPathSizeChecked(const char* path)` (0x317f60): the size of `name`
/// as reported by `FILESYS_bypassqueuefileinfo` (0x3de4d0), or 0 when it
/// does not answer 1. A pre-zeroed out-word keeps the miss path at 0.
pub fn ea_file_get_path_size_checked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        let out = c.g[SP][0] as u32;
        c.sw(out, 0);
        callee::EA_FILE_PARSE_BIG_PATH.call(c, 0x0031_7f78)?;
        let missing = c.g[V0][0] ^ 1;
        c.g[V0][0] = 0;
        if missing == 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(out).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00317f98 EAFile_GetPathSize

/// `int EAFile_GetPathSize(const char* path)` (0x317f98): the size of `name`
/// as reported by `FILESYS_bypassqueuefileinfo` (0x3de4d0), with no
/// answer check: a pre-zeroed out-word is returned as is.
pub fn ea_file_get_path_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0x10);
        c.g[A2][0] = c.g[SP][0];
        let out = c.g[SP][0] as u32;
        c.sw(out, 0);
        callee::EA_FILE_PARSE_BIG_PATH.call(c, 0x0031_7fb0)?;
        c.g[V0][0] = Addr(out).lw(c);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00317fc0 EAFile_EmptyStub

/// `void EAFile_EmptyStub(void)` (0x317fc0): empty EAFile stub; teardown
/// paths tail-call it.
pub fn ea_file_empty_stub(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// 0x00317fc8 EAFile_EmptyStub_Wrapper

/// `void EAFile_EmptyStub_Wrapper(void)` (0x317fc8): frame wrapper around the
/// empty `EAFile_EmptyStub` stub.
pub fn ea_file_empty_stub_wrapper(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::EA_FILE_EMPTY_STUB.call(c, 0x0031_7fd8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x003dec80 EAFile_Write

/// `void EAFile_Write(a, b, data, size, prio)` (0x3dec80): the chunked
/// file write: hand the caller's registers to the chunked transfer
/// `EAFile_TransferChunked` (0x3deb50) with the write-chunk callback
/// (0x3dd720) in `t1`. Returns the transfer's spare word (0 here).
pub fn eafile_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[T1][0] = 0x3e0000;
        frame.save_ra(c, 0);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x28e0));
        callee::EA_FILE_TRANSFER_CHUNKED.call(c, 0x003d_ec94)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// How often `EAFile_CdCompareHash` found two equal hashes (a collision
/// counter, `[0x51ed80]`).
pub(crate) const EA_FILE_CD_HASH_EQUAL_COUNT: u32 = 0x0051_ed80;

/// `int EAFile_CdCompareHash(const u32 *a, const u32 *b)` (0x3e3968): compare
/// two file-name hashes (`-1`, `1`, or `0` when equal). Equal hashes also
/// count a collision in [`EA_FILE_CD_HASH_EQUAL_COUNT`]. The 64-bit difference
/// of the zero-extended words is tested, so the full unsigned range compares
/// correctly.
pub fn ea_file_cd_compare_hash(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = c.lwu(c.g[A0][0] as u32);
    c.g[V0][0] = c.lwu(c.g[A1][0] as u32);
    c.g[V1][0] = c.g[V1][0].wrapping_sub(c.g[V0][0]);
    let diff = c.g[V1][0] as i64;
    c.g[V0][0] = u64::MAX;
    c.cycles += 2;
    if diff < 0 {
        ret(c);
        return;
    }
    c.g[A0][0] = 0x520000;
    if diff > 0 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V0][0] = 0;
    c.g[V1][0] = c.lw(EA_FILE_CD_HASH_EQUAL_COUNT);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    // The store sits in the return's delay slot.
    c.sw(EA_FILE_CD_HASH_EQUAL_COUNT, c.g[V1][0]);
    ret(c);
}

/// `char *EAFile_CdReset(void)` (0x3e3618): (re)read the CD sector at the
/// current LBA into the sector buffer `0x519c80` and return it. Retries up to
/// ten times on drive errors (a delay between polls); when the budget is
/// used up it re-initialises the drive (`sceCdInit(0)`, `sceCdMmode`) and
/// starts over. A `match pc` body (back edges deliver events): re-entered at
/// its block heads after an unwind.
pub fn ea_file_cd_reset(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e3618 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] = 0x450000;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = 0x520000;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = 0xa;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x63c0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S4][0] = 0x520000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    pc = 0x003e3650;
                    continue;
                }
                0x003e3650 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x003e3668;
                        continue;
                    }
                    pc = 0x003e3658;
                    continue;
                }
                0x003e3658 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e3660);
                    c.g[S2][0] = 0xa;
                    call_ptr(c, 0x00401718, 0x003e3660)?;
                    c.g[RA][0] = sx(0x003e3668);
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    call_ptr(c, 0x00402450, 0x003e3668)?;
                    pc = 0x003e3668;
                    continue;
                }
                0x003e3668 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e3670);
                    c.g[A0][0] = 0u64;
                    call_ptr(c, 0x00401bf0, 0x003e3670)?;
                    let t = true;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x003e3680;
                        continue;
                    }
                    pc = 0x003e3678;
                    continue;
                }
                0x003e3678 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e3680);
                    c.g[A0][0] = 0x1e0;
                    Callee {
                        addr: 0x003e35b0,
                        f: ee::s14::EAFile_DelayTicks,
                    }
                    .call(c, 0x003e3680)?;
                    pc = 0x003e3680;
                    continue;
                }
                0x003e3680 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6380));
                    c.g[RA][0] = sx(0x003e3694);
                    c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c));
                    call_ptr(c, 0x00401df8, 0x003e3694)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x003e3678) {
                            return Ok(());
                        }
                        pc = 0x003e3678;
                        continue;
                    }
                    pc = 0x003e369c;
                    continue;
                }
                0x003e369c => {
                    c.cycles += 1;
                    pc = 0x003e36a0;
                    continue;
                }
                0x003e36a0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x003e36a8);
                    c.g[A0][0] = 0x1;
                    call_ptr(c, 0x004013e8, 0x003e36a8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e36b4);
                    c.g[A0][0] = 0x1e0;
                    Callee {
                        addr: 0x003e35b0,
                        f: ee::s14::EAFile_DelayTicks,
                    }
                    .call(c, 0x003e36b4)?;
                    let t = c.g[S0][0] != 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x003e36a0) {
                            return Ok(());
                        }
                        pc = 0x003e36a0;
                        continue;
                    }
                    pc = 0x003e36bc;
                    continue;
                }
                0x003e36bc => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e36c4);
                    call_ptr(c, 0x00402300, 0x003e36c4)?;
                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    if c.g[V0][0] == 0 {
                        c.g[S2][0] = c.g[S1][0];
                    }
                    if t {
                        if back_edge_unwinds(c, 0x003e3650) {
                            return Ok(());
                        }
                        pc = 0x003e3650;
                        continue;
                    }
                    pc = 0x003e36d0;
                    continue;
                }
                0x003e36d0 => {
                    c.cycles += 10;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6380));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e3618, 0x003e36f8);
                    return Ok(());
                }
            }
        }
    });
}

/// `void EAFile_CdScanDir(int lba, int sectors)` (0x3e3758): walk the ISO9660
/// directory at `lba` (`sectors` sectors long), appending every file to the
/// file table (`[0x450c68] + n * 0x108`: path, LBA, size; count at
/// `[0x450c6c]`) and recursing into subdirectories with the sector buffer
/// and the path saved around the call. Re-entered at its block heads after
/// an unwind (`match pc` body).
pub fn ea_file_cd_scan_dir(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e3758 => {
                    c.cycles += 29;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x9c0));
                    c.g[V0][0] = 0x520000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x9a0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x990));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x980));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x970));
                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63a0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x960));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x950));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x940));
                    c.g[S6][0] = 0x450000;
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x930));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x9b0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x920), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x910), c.g[V0][0]);
                    c.g[V0][0] = 0x520000;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                    c.g[RA][0] = sx(0x003e37b0);
                    c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1380));
                    Callee {
                        addr: 0x003e3618,
                        f: ee::s14::EAFile_CdReset,
                    }
                    .call(c, 0x003e37b0)?;
                    c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                    c.g[V1][0] = 0x520000;
                    c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let t = true;
                    c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6380));
                    if t {
                        pc = 0x003e38fc;
                        continue;
                    }
                    pc = 0x003e37cc;
                    continue;
                }
                0x003e37cc => {
                    c.cycles += 1;
                    pc = 0x003e37d0;
                    continue;
                }
                0x003e37d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] & 0x2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x003e386c;
                        continue;
                    }
                    pc = 0x003e37dc;
                    continue;
                }
                0x003e37dc => {
                    c.cycles += 36;
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x003e37e8);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                    call_ptr(c, 0x0041605c, 0x003e37e8)?;
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A3][0] = 0x2f;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = c.g[S7][0];
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sb(c.g[V1][0] as u32, c.g[A3][0]);
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                    c.g[RA][0] = sx(0x003e3810);
                    c.sb((c.g[V0][0] as u32).wrapping_add(0x1), 0u64);
                    call_ptr(c, 0x004166f4, 0x003e3810)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e381c);
                    c.g[A0][0] = c.g[S7][0];
                    call_ptr(c, 0x004162d0, 0x003e381c)?;
                    c.g[A2][0] = 0x800;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x003e382c);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e382c)?;
                    c.lwl(5, (c.g[S1][0] as u32).wrapping_add(0xd));
                    c.lwr(5, (c.g[S1][0] as u32).wrapping_add(0xa));
                    c.lwl(4, (c.g[S1][0] as u32).wrapping_add(0x5));
                    c.lwr(4, (c.g[S1][0] as u32).wrapping_add(0x2));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7ff));
                    c.g[RA][0] = sx(0x003e3848);
                    c.g[A1][0] = sx((c.g[A1][0] as u32) >> 11);
                    Callee {
                        addr: 0x003e3758,
                        f: ee::s14::EAFile_CdScanDir,
                    }
                    .call(c, 0x003e3848)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003e3858);
                    c.g[A2][0] = 0x800;
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e3858)?;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[RA][0] = sx(0x003e3864);
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x004166f4, 0x003e3864)?;
                    let t = true;
                    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                    if t {
                        pc = 0x003e3900;
                        continue;
                    }
                    pc = 0x003e386c;
                    continue;
                }
                0x003e386c => {
                    c.cycles += 14;
                    c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21));
                    c.g[RA][0] = sx(0x003e387c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x2));
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e387c)?;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0x2e;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[SP][0] as u32));
                    c.sb((c.g[V1][0] as u32).wrapping_sub(0x2), 0u64);
                    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);

                    if c.g[V1][0] == c.g[A0][0] {
                        c.sb(c.g[V0][0] as u32, 0u64);
                        pc = 0x003e38a4;
                        continue;
                    }
                    pc = 0x003e38a4;
                    continue;
                }
                0x003e38a4 => {
                    c.cycles += 22;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc6c));
                    c.g[V1][0] = 0x108;
                    c.g[A0][0] = 0x450000;
                    c.mult(0, 2, 2, 3);
                    c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc68));
                    c.g[A0][0] = 0x520000;
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1380));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x003e38cc);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x004166f4, 0x003e38cc)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e38d8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x004162d0, 0x003e38d8)?;
                    c.lwl(4, (c.g[S1][0] as u32).wrapping_add(0x5));
                    c.lwr(4, (c.g[S1][0] as u32).wrapping_add(0x2));
                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc6c));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x100), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.lwl(2, (c.g[S1][0] as u32).wrapping_add(0xd));
                    c.lwr(2, (c.g[S1][0] as u32).wrapping_add(0xa));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.sw((c.g[S6][0] as u32).wrapping_add(0xc6c), c.g[V1][0]);
                    pc = 0x003e38fc;
                    continue;
                }
                0x003e38fc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
                    pc = 0x003e3900;
                    continue;
                }
                0x003e3900 => {
                    c.cycles += 1;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    pc = 0x003e3904;
                    continue;
                }
                0x003e3904 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S1][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x19));
                        if back_edge_unwinds(c, 0x003e37d0) {
                            return Ok(());
                        }
                        pc = 0x003e37d0;
                        continue;
                    }
                    pc = 0x003e3910;
                    continue;
                }
                0x003e3910 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x003e3930;
                        continue;
                    }
                    pc = 0x003e3918;
                    continue;
                }
                0x003e3918 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x003e3928);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    Callee {
                        addr: 0x003e3618,
                        f: ee::s14::EAFile_CdReset,
                    }
                    .call(c, 0x003e3928)?;
                    let t = true;
                    c.g[S1][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x003e3904) {
                            return Ok(());
                        }
                        pc = 0x003e3904;
                        continue;
                    }
                    pc = 0x003e3930;
                    continue;
                }
                0x003e3930 => {
                    c.cycles += 14;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x910));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x9b0));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x9a0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x990));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x980));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x970));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x960));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x950));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x940));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x930));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x920));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9c0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e3758, 0x003e3968);
                    return Ok(());
                }
            }
        }
    });
}

/// `void EAFile_CdReadThread(void)` (0x3e3be0): the CD read thread, which never
/// returns. Waits on the request semaphore, then handles a drive error or
/// an abort flag (re-issuing the read while retries last), the copy of the
/// sector just read into the caller's buffer, and the next sector run
/// (`EAFile_CdIssueRead`) or the completion of the request.
pub fn ea_file_cd_read_thread(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
        c.g[V0][0] = 0x520000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63a0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x520000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = 0x1;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S4][0] = 0xfffffffffffffffe;
        'l_3e3c18: loop {
            'b_3e3c7c: {
                'b_3e3c3c: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e3c20);
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x00423de0, 0x003e3c20)?;
                    c.g[RA][0] = sx(0x003e3c28);
                    call_ptr(c, 0x00402300, 0x003e3c28)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                    if t {
                        break 'b_3e3c3c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V1][0] & 0x8;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[V1][0] & 0x1;
                        if t {
                            break 'b_3e3c7c;
                        } else {
                            break 'b_3e3c3c;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = 0xfffffffffffffff7;
                c.g[A0][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] & 0x2;
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[A0][0]);
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e3c18);
                    }
                    continue 'l_3e3c18;
                } else {
                    'b_3e3c70: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                        if t {
                            break 'b_3e3c70;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e3c68);
                            Callee {
                                addr: 0x003e3b00,
                                f: ee::s14::EAFile_CdIssueRead,
                            }
                            .call(c, 0x003e3c68)?;
                            let t = true;

                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x003e3c18);
                                }
                                continue 'l_3e3c18;
                            } else {
                                break 'b_3e3c70;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] | 0x4;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e3c18);
                        }
                        continue 'l_3e3c18;
                    } else {
                        break 'b_3e3c7c;
                    }
                }
            }
            'b_3e3ca8: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = c.g[V1][0] & c.g[S4][0];
                if t {
                    break 'b_3e3ca8;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003e3ca8);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[A3][0]);
                    Callee {
                        addr: 0x003e6574,
                        f: ee::s14::EA_MemCopy,
                    }
                    .call(c, 0x003e3ca8)?;
                    break 'b_3e3ca8;
                }
            }
            'b_3e3d54: {
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = (c.g[A3][0] as i64) <= 0;
                c.g[A2][0] = ((c.g[A3][0] as i64) < 2048i64) as u64;
                if t {
                    break 'b_3e3d54;
                } else {
                    'b_3e3d38: {
                        'b_3e3cf4: {
                            c.cycles += 10;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            let t = c.g[A2][0] == 0u64;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                            if t {
                                break 'b_3e3cf4;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                                c.sw(c.g[S0][0] as u32, c.g[A3][0]);
                                c.g[V0][0] = c.g[V0][0] | 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                                if t {
                                    break 'b_3e3d38;
                                } else {
                                    break 'b_3e3cf4;
                                }
                            }
                        }
                        'b_3e3d18: {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] & 0x3f;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                            if t {
                                break 'b_3e3d18;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = 0x800;
                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                c.g[V1][0] = c.g[V1][0] | 0x1;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V1][0]);
                                if t {
                                    break 'b_3e3d38;
                                } else {
                                    break 'b_3e3d18;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
                        c.g[V1][0] = 0xfffffffffffff800;
                        c.g[V1][0] = c.g[A3][0] & c.g[V1][0];
                        c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
                        c.g[A0][0] = sx(((c.g[V1][0] as u32) as i32 >> 11) as u32);
                        c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
                        break 'b_3e3d38;
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x003e3d4c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    Callee {
                        addr: 0x003e3b00,
                        f: ee::s14::EAFile_CdIssueRead,
                    }
                    .call(c, 0x003e3d4c)?;
                    let t = true;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e3c18);
                        }
                        continue 'l_3e3c18;
                    } else {
                        break 'b_3e3d54;
                    }
                }
            }
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x63c0));
            c.g[V1][0] = 0xfffffffffffffffd;
            c.g[A0][0] = 0x1;
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.g[RA][0] = sx(0x003e3d6c);
            c.sw((c.g[S1][0] as u32).wrapping_sub(0x63c0), c.g[V0][0]);
            Callee {
                addr: 0x003de420,
                f: ee::s14::EAFile_CompleteRequest,
            }
            .call(c, 0x003e3d6c)?;
            let t = true;

            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x003e3c18);
                }
                continue 'l_3e3c18;
            } else {
                c.cycles += 1;
                c.pc = 0x003e_3d78;
                ee::s14::EAFile_CdMountDisc(c);
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::fake_check;

    // -------------------------------------------------- EAFile_ReleaseRequest

    #[test]
    fn ea_file_release_request_matches() {
        fake_check(
            ssx3_ee::s14::f_003dd310__generated,
            ea_file_release_request,
            44,
            &|c, _seed, r| {
                c.g[A0][0] = DATA as u64; // its top byte is 0
                c.g[SP][0] = (STACK - 0x100) as u64;
                // The hash table entry for top byte 0 points at the request.
                c.bus.write32(0x0051_ad98, DATA);
                // A positive state that equals itself in the compare.
                c.bus.write32(DATA + 8, 1);
                // Request flags: channel class 2..10, so the dispatch is taken.
                let lane = 2 + r.next_u32() % 9;
                c.bus
                    .write32(DATA, (lane << 20) | (r.next_u32() & 0x000f_ffff));
                // The state table (0x495d40) stays zero: default jump_out.
            },
        );
    }

    /// EAFile_StrICmp: strings of 0..5 characters over a mixed-case
    /// alphabet, often `b` a case-flipped copy of `a` (equal, end reached),
    /// sometimes a prefix; uppercase bits set in the ctype table.
    #[test]
    fn eafile_str_icmp_matches() {
        check(
            ee::s14::f_003e2490__generated,
            eafile_str_icmp,
            300,
            &|c, seed| {
                let mut r = Rng(0x003e_2490 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003e_2490;
                for ch in b'A'..=b'Z' {
                    c.bus.write8(EAF_CTYPE + ch as u32, 1);
                }
                const ALPHABET: [u8; 7] = [b'a', b'A', b'b', b'B', b'1', b'_', 0xc1];
                let a = DATA + 0x1000;
                let b = DATA + 0x2000;
                let n = seed % 6;
                for i in 0..n {
                    let ch = ALPHABET[(r.next_u32() % 7) as usize];
                    c.bus.write8(a + i, ch);
                }
                c.bus.write8(a + n, 0);
                let m = if seed % 7 == 0 {
                    n.saturating_sub(1)
                } else {
                    n
                };
                for i in 0..m {
                    let x = c.bus.read8(a + i);
                    let ch = if seed % 3 != 2 {
                        if x.is_ascii_lowercase() && r.next_u32() % 2 == 0 {
                            x - 32
                        } else {
                            x
                        }
                    } else {
                        ALPHABET[(r.next_u32() % 7) as usize]
                    };
                    c.bus.write8(b + i, ch);
                }
                c.bus.write8(b + m, 0);
                c.g[A0][0] = sx(a);
                c.g[A1][0] = sx(b);
            },
        );
    }

    /// EAFile_AllocRequest: pools of 1..6 slots with the first free one
    /// anywhere (or a negative count: no scan), serials near the wrap; the
    /// real EAMutex lock/unlock with fake thread-id/semaphore syscalls (some
    /// unwinding). The all-busy case spins forever in the original and is
    /// not run.
    #[test]
    fn ea_file_alloc_request_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const SLOTS: u32 = DATA + 0x1000;
        check(
            ssx3_ee::s14::f_003de670__generated,
            ea_file_alloc_request,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_e670, 6);
                c.pc = 0x003d_e670;
                let count: i32 = if seed % 7 == 0 {
                    -1 - (r.next_u32() % 3) as i32
                } else {
                    1 + (r.next_u32() % 6) as i32
                };
                c.bus.write32(EAFILE_POOL, count as u32);
                c.bus.write32(EAFILE_POOL + EAFILE_POOL_SLOTS, SLOTS);
                // Mutex: owner never a fake thread id, depth, semaphore.
                c.bus
                    .write32(EAFILE_POOL_MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(EAFILE_POOL_MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(EAFILE_POOL_MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = if count > 0 {
                    r.next_u32() % count as u32
                } else {
                    0
                };
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000 << (r.next_u32() % 4);
                    }
                    c.bus.write32(SLOTS + EAFILE_REQ_SIZE * i, w);
                }
                let serial = match seed % 3 {
                    0 => 0xf_fffe + r.next_u32() % 3,
                    1 => r.next_u32(),
                    _ => r.next_u32() & 0xf_ffff,
                };
                c.bus.write32(EAFILE_REQ_SERIAL, serial);
            },
        );
    }

    /// EAFile_QueueRead: the real EAFile_AllocRequest on a pool with a free
    /// slot (EAFile_AllocRequest's own fixture: mutex with fake syscalls,
    /// some unwinding), a null file or one whose size clamps the read or
    /// not, and the real EAFile_ServiceQueue on its short path (a request
    /// already active at pool+0x24: insert into the priority queue of 0..2
    /// requests, unlock, return).
    #[test]
    fn ea_file_queue_read_matches() {
        use crate::difftest::fakes::machine;
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const FILE: u32 = DATA + 0x2000;
        const QUEUED: u32 = DATA + 0x3000;
        check(
            ee::s14::f_003dd648__generated,
            ea_file_queue_read,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d648, 8);
                c.pc = 0x003d_d648;
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + 0x28, SLOTS);
                c.bus
                    .write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = r.next_u32() % count;
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000;
                    }
                    c.bus.write32(SLOTS + 0x30 * i, w);
                }
                c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
                // The service queue: a request active; 0..2 queued.
                c.bus.write32(POOL + 0x24, QUEUED + 0x100);
                let queued = seed % 3;
                c.bus
                    .write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
                for k in 0..queued {
                    let q = QUEUED + 0x40 * k;
                    c.bus.write32(q + 0x10, r.next_u32() % 8);
                    c.bus
                        .write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
                }
                c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
                c.bus
                    .write32(FILE + EAFILE_QR_FILE_SIZE, r.next_u32() & 0xf_ffff);
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { sx(FILE) };
                c.g[A1][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A2][0] = sx(DATA + 0x8000);
                c.g[A3][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[T0][0] = sx(r.next_u32() % 8);
                c.g[T1][0] = sx(r.next_u32());
            },
        );
    }

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::RA as SENTINEL;

    // -- 0x003de800 EAFile_AllocFileHandle -----------------------------------

    /// The file manager with 1..4 handles, at least one free (the original
    /// spins forever when every handle is taken), and its mutex owned by the
    /// thread id the fakes return (0x42), so both mutex calls take the fast
    /// path.
    pub(crate) fn oc325_file_alloc_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x003d_e800, 3);
        for i in 0..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * i, 0x42);
        }
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 4, 0x42);
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 8, 1);
        c.bus.write32(OC325_FILE_MANAGER_MUTEX + 0xc, 32);
        c.bus.write32(0x0045_0de4, 0);
        let count = 1 + r.next_u32() % 4;
        let array = DATA + 0x1_0000;
        c.bus.write32(OC325_FILE_MANAGER + OC325_FILE_COUNT, count);
        c.bus.write32(OC325_FILE_MANAGER + OC325_FILE_ARRAY, array);
        let free = r.next_u32() % count;
        for i in 0..count {
            let handle = array + OC325_FILE_HANDLE * i;
            c.bus.write32(handle, if i == free { 0 } else { 1 });
            c.bus.write32(handle + 4, r.next_u32());
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003d_e800;
    }

    /// The mutex fast path on both calls and a free handle at a random
    /// index; some mutex calls unwind (propagated).
    #[test]
    fn ea_file_alloc_file_handle_matches() {
        covered(
            ee::s14::f_003de800__generated,
            ea_file_alloc_file_handle,
            200,
            &|c, seed| oc325_file_alloc_setup(c, seed),
        );
    }

    // ------------------------------------------------------ EAFile_QueueOpen

    /// EAFile_QueueOpen: the real EAFile_AllocRequest (a pool with a free
    /// request slot; mutex with fake syscalls, some unwinding) and
    /// EAFile_AllocFileHandle (a free handle slot), a name buffer, and the
    /// real EAFile_ServiceQueue on its short path (a request already active
    /// at pool+0x24, 0..2 queued). The null-handle branch is not reached: the
    /// real AllocFileHandle spins when every handle is busy instead of
    /// returning 0.
    #[test]
    fn ea_file_queue_open_matches() {
        use crate::game::lib::eafile::{EAFILE_POOL_SLOTS, EAFILE_REQ_SIZE};
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const HANDLES: u32 = DATA + 0x4000;
        const QUEUED: u32 = DATA + 0x8000;
        const NAME: u32 = DATA + 0xc000;
        const HANDLE_COUNT: u32 = POOL + 0x4;
        const HANDLE_SLOTS: u32 = POOL + 0x2c;
        check(
            ee::s14::f_003dd4e8__generated,
            ea_file_queue_open,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d4e8, 8);
                c.pc = 0x003d_d4e8;
                // The request pool: 1..6 slots, the first free one anywhere.
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + EAFILE_POOL_SLOTS, SLOTS);
                // The handle pool: 1..6 slots, all free.
                let handles = 1 + r.next_u32() % 6;
                c.bus.write32(HANDLE_COUNT, handles);
                c.bus.write32(HANDLE_SLOTS, HANDLES);
                for i in 0..6u32 {
                    c.bus.write32(HANDLES + 0x110 * i, 0);
                }
                // Mutex: owner never a fake thread id, depth, semaphore.
                c.bus
                    .write32(MUTEX + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
                c.bus.write32(MUTEX + 0x8, r.next_u32() & 3);
                c.bus.write32(MUTEX + 0xc, r.next_u32() & 0xff);
                c.bus.write32(0x0045_0de4, r.next_u32() & 1);
                let free = r.next_u32() % count;
                for i in 0..8u32 {
                    let mut w = r.next_u32();
                    if i == free {
                        w &= !0x00f0_0000;
                    } else if i < free && w & 0x00f0_0000 == 0 {
                        w |= 0x0010_0000;
                    }
                    c.bus.write32(SLOTS + EAFILE_REQ_SIZE * i, w);
                }
                c.bus.write32(0x0045_0c0c, r.next_u32() & 0xf_ffff);
                // The service queue: a request active; 0..2 queued.
                c.bus.write32(POOL + 0x24, QUEUED + 0x100);
                let queued = seed % 3;
                c.bus
                    .write32(POOL + 0x30, if queued == 0 { 0 } else { QUEUED });
                for k in 0..queued {
                    let q = QUEUED + 0x40 * k;
                    c.bus.write32(q + 0x10, r.next_u32() % 8);
                    c.bus
                        .write32(q + 0x2c, if k + 1 < queued { q + 0x40 } else { 0 });
                }
                c.bus.write32(POOL + 0xc, r.next_u32() & 0xff);
                // The name: a short, NUL-terminated string.
                let len = r.next_u32() % 0x40;
                for i in 0..len {
                    c.bus.write8(NAME + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(NAME + len, 0);
                c.g[A0][0] = sx(NAME);
                c.g[A1][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A2][0] = sx(r.next_u32() & 0x7_ffff);
                c.g[A3][0] = sx(r.next_u32() & 0x7_ffff);
            },
        );
    }

    /// `EAFile_QueueClose`: a real request pool (two free slots), a
    /// request-list with no matching, a matching or a second matching node,
    /// and priorities that keep the real service queue on its short path.
    #[test]
    fn ea_file_queue_close_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const POOL: u32 = 0x0051_9ab0;
        const SLOTS: u32 = DATA + 0x1_0000;
        const NODE: u32 = DATA + 0x2_0000;
        const NODE2: u32 = DATA + 0x2_0100;
        const ID: u32 = 0x1234;
        check(
            ee::s14::f_003dd5a0__generated,
            ea_file_queue_close,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d5a0, 0);
                c.pc = 0x003d_d5a0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // The request pool: two free slots, a zeroed mutex and a zeroed
                // request queue (the real service queue then takes its short
                // path).
                for i in 0..0x10u32 {
                    w(c, POOL + 4 * i, 0);
                }
                w(c, POOL, 2);
                w(c, POOL + 0x28, SLOTS);
                for i in 0..0x20u32 {
                    w(c, SLOTS + 4 * i, 0);
                }
                // The request list.
                let head = if seed % 3 == 0 { 0 } else { NODE };
                w(c, EAFILE_QC_LIST_HEAD, head);
                fill(c, &mut r, NODE, 0x20);
                fill(c, &mut r, NODE2, 0x20);
                w(
                    c,
                    NODE + EAFILE_QC_LIST_ID,
                    if seed % 2 == 0 { ID } else { ID + 1 },
                );
                w(
                    c,
                    NODE + EAFILE_QC_LIST_NEXT,
                    if seed % 4 == 0 { NODE2 } else { 0 },
                );
                w(c, NODE2 + EAFILE_QC_LIST_ID, ID);
                w(c, NODE2 + EAFILE_QC_LIST_NEXT, 0);
                c.g[A0][0] = ID as u64;
                c.g[A1][0] = 1 + (r.next_u32() % 0x100) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    // --------------------------------------------------- EAFile_FreeFileHandle

    /// A real handle pool mutex (short path) and a handle record to clear.
    #[test]
    fn ea_file_free_file_handle_matches() {
        check(
            ee::s14::f_003de8c0__generated,
            ea_file_free_file_handle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_e8c0, 4);
                c.pc = 0x003d_e8c0;
                let owner = 0x40 + seed % 3;
                c.bus.write32(EAF_HANDLE_MUTEX + 4, owner);
                c.bus.write32(EAF_HANDLE_MUTEX + 8, 1 + seed % 2);
                c.bus.write32(EAF_HANDLE_MUTEX, r.next_u32() & 0xff);
                let handle = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                for i in 0..(EAF_HANDLE_SIZE / 4) as u32 {
                    c.bus.write32(handle + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(handle);
            },
        );
    }

    use crate::difftest::fill;
    use crate::difftest::put_words;

    pub(crate) use crate::game::sound::soundsys::tests::oc371_machine;
    pub(crate) use crate::game::sound::soundsys::tests::oc371_script;

    pub(crate) use crate::game::sound::soundsys::tests::oc371_install_fakes;

    /// A NUL-terminated string at `at`.
    pub(crate) fn oc371_put_str(c: &mut Cpu, at: u32, text: &str) {
        for (i, b) in text.bytes().chain([0]).enumerate() {
            c.bus.write8(at + i as u32, b);
        }
    }

    /// Queueing (empty queue, in front, behind several) and serving: busy,
    /// paused, priority too high, cancelled, every command but write (5:
    /// its real `EAFile_PlatformWrite` reaches the sceFs semaphores) and
    /// out-of-range ones. Open / exists with plain paths (open flag on /
    /// off, open fails or not), `big|name` and `|name` searched through
    /// big files (named match or not, found or not). libc runs for real;
    /// the EAFile callees are logged fakes (one may unwind).
    #[test]
    fn eafile_service_queue_matches() {
        oc371_install_fakes();
        covered(
            ee::s14::f_003ddfa8__generated,
            eafile_service_queue,
            400,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003d_dfa8, 6);
                c.pc = 0x003d_dfa8;
                put_words(
                    c,
                    EAFS_JUMP_TABLE,
                    &[
                        0x003d_e108,
                        0x003d_e308,
                        0x003d_e350,
                        0x003d_e3b4,
                        0x003d_e3d4,
                        0x003d_e3f0,
                        0x003d_e108,
                        0x003d_e3f0,
                        0x003d_e3f0,
                    ],
                );
                oc371_script(c, 0x003e_3208, (r.next_u32() % 2) as u32);
                oc371_script(c, 0x003e_2768, (r.next_u32() % 3 == 0) as u32);
                let st = EAFS_STATE;
                let n_queued = r.next_u32() % 4;
                let mut prio = r.next_u32() % 3;
                let cmds = [2, 2, 8, 8, 3, 3, 4, 4, 6, 7, 9, 10, 0, 12];
                let names = ["file.dat", "big|name", "|name", "other|name", "|"];
                let mut prev = 0u32;
                for i in 0..n_queued + 1 {
                    let req = DATA + 0x200 + 0x100 * i;
                    fill(c, &mut r, req, 0x10);
                    let cmd = cmds[(r.next_u32() % cmds.len() as u32) as usize];
                    let w = r.next_u32() & !0x00f0_0000 | cmd << 20;
                    c.bus.write32(req, w);
                    c.bus.write32(req + 4, (r.next_u32() % 8 == 0) as u32);
                    c.bus.write32(req + 0x18, r.next_u32() % 4);
                    c.bus.write32(req + 0x1c, r.next_u32() % 3);
                    prio += r.next_u32() % 3;
                    c.bus.write32(req + EAFS_REQ_PRIORITY, prio);
                    // The file handle and its path.
                    let fh = DATA + 0x1000 + 0x100 * i;
                    let handle = if r.next_u32() % 5 == 0 { 0 } else { fh };
                    c.bus.write32(req + EAFS_REQ_HANDLE, handle);
                    c.bus.write32(fh, DATA + 0x3000);
                    c.bus.write32(fh + 4, r.next_u32());
                    c.bus.write32(
                        fh + 8,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32() % 0x1000
                        },
                    );
                    c.bus.write32(fh + 0xc, r.next_u32());
                    let name = names[(r.next_u32() % names.len() as u32) as usize];
                    oc371_put_str(c, fh + 0x10, name);
                    c.bus.write32(req + EAFS_REQ_NEXT, 0);
                    if i < n_queued {
                        if prev == 0 {
                            c.bus.write32(st + 0x30, req);
                        } else {
                            c.bus.write32(prev + EAFS_REQ_NEXT, req);
                        }
                        prev = req;
                    }
                }
                if n_queued == 0 {
                    c.bus.write32(st + 0x30, 0);
                }
                // The new request (last one built), or none.
                let new_req = DATA + 0x200 + 0x100 * n_queued;
                c.bus.write32(new_req + EAFS_REQ_PRIORITY, r.next_u32() % 6);
                c.g[A0][0] = if r.next_u32() % 5 == 0 {
                    0
                } else {
                    sx(new_req)
                };
                c.bus.write32(DATA + 0x3000 + 0xc, DATA + 0x3100);
                c.bus.write32(st + 0x8, r.next_u32() % 8);
                c.bus.write32(st + 0xc, r.next_u32() % 8);
                c.bus.write32(st + 0x10, (r.next_u32() % 8 == 0) as u32);
                c.bus
                    .write32(st + 0x24, if r.next_u32() % 8 == 0 { DATA } else { 0 });
                // The open big files, named "big" or "other".
                let n_big = r.next_u32() % 4;
                let mut next = 0;
                for i in (0..n_big).rev() {
                    let big = DATA + 0x2000 + 0x40 * i;
                    c.bus.write32(big, r.next_u32());
                    c.bus.write32(big + 0xc, DATA + 0x2800 + 0x40 * i);
                    let bn = if r.next_u32() % 2 == 0 {
                        "big"
                    } else {
                        "other"
                    };
                    oc371_put_str(c, DATA + 0x2810 + 0x40 * i, bn);
                    c.bus.write32(big + 0x14, next);
                    next = big;
                }
                c.bus.write32(st + 0x34, next);
            },
        );
    }
    use crate::difftest::check_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;

    /// Started and fresh states; the fresh path sizes, allocates through
    /// the test fake and initialises (already-started, so it returns 0).
    #[test]
    fn ea_file_startup_matches() {
        check(
            ee::s14::f_003dcbd8__generated,
            ea_file_startup,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cbd8, 0);
                // Request area, allocator slot, started flag.
                c.bus.write32(0x0045_0c10, 0x1000);
                c.bus.write32(0x0045_0c14, 0x2000);
                c.bus.write32(0x0049_5d28, 0x3000);
                c.bus.write32(0x0051_9ab0, seed % 2);
                c.g[A0][0] = 0x10;
                c.g[A1][0] = 0x20;
                c.g[A2][0] = 0x30;
                c.pc = 0x003d_cbd8;
            },
        );
    }

    /// Already-started (0) and fresh (1) inits with default and given
    /// counts. The CD init arm (`[0x450c60]` set) is not reached: it runs
    /// the real CD driver init, which has no override slot to fake.
    #[test]
    fn ea_file_init_matches() {
        check(
            ee::s14::f_003dcc88__generated,
            ea_file_init,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cc88, 0);
                c.bus.write32(0x0051_9ab0, (seed % 2) as u32);
                c.bus.write32(0x0045_0c60, 0);
                c.bus.write32(0x0045_0c5c, 0x40);
                c.bus.write32(0x0051_9ad8, DATA + 0x800);
                for i in 0..16u32 {
                    c.bus.write32(DATA + 0x800 + 4 * i, 0);
                }
                // Zero counts take the defaults (0x10, 0x3e8, 0x20); the
                // started shortcut keys on odd seeds.
                c.g[A0][0] = [0u64, 0x10, 0x8][seed as usize % 3];
                c.g[A1][0] = [0u64, 0x20][seed as usize / 6 % 2];
                c.g[A2][0] = [0u64, 0x30][seed as usize / 12 % 2];
                c.g[A3][0] = (DATA + 0x900) as u64;
                c.pc = 0x003d_cc88;
            },
        );
    }

    /// Default and explicit queue counts; LO/HI compared too.
    #[test]
    fn ea_file_calc_mem_size_matches() {
        check_lohi(
            ee::s14::f_003dcd98__generated,
            ea_file_calc_mem_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cd98, 0);
                c.bus.write32(0x0045_0c5c, 0x40);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { 0x18 };
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { 0x400 };
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { 0x28 };
                c.pc = 0x003d_cd98;
            },
        );
    }

    /// Null requests (-3) and crafted handles hitting the table both ways.
    #[test]
    fn ea_file_get_request_status_matches() {
        check(
            ee::s14::f_003dce90__generated,
            ea_file_get_request_status,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ce90, 0);
                // V1 = byte 3 of the request times 0x30 (the `mult`
                // just above folds V0 = 0x30 in) plus the table word
                // below; the status word read is fully controlled.
                // i % 3 == 0 matches (status set), 1 is clear, 2 is the
                // `< 1` boundary ((word ^ req) & mask == 1).
                let tbl = DATA + 0x800;
                c.bus.write32(0x0051_9ad8, tbl);
                let mode = seed % 4;
                if mode == 0 {
                    c.g[A0][0] = 0;
                } else {
                    // Byte 3 selects the table slot (stride 0x30); the
                    // word there decides the status bit.
                    let i = seed % 8;
                    let slot = tbl + i * 0x30;
                    c.bus.write32(slot, [0x4_0000, 0, 0x4_0001][i as usize % 3]);
                    c.bus.write32(slot + 8, 0x1234_0000 + seed);
                    c.g[A0][0] = 0x34_0000 + ((i << 24) as u64);
                }
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_ce90;
            },
        );
    }

    /// Write `s` as bytes at `addr`, NUL-terminated.
    pub(crate) fn put_ascii(c: &mut Cpu, addr: u32, s: &str) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// The request hash table: every top byte selects `DATA + 0x1000`.
    pub(crate) fn table_setup(c: &mut Cpu) {
        c.bus.write32(0x0051_9ad8, DATA + 0x1000);
    }

    /// Queue-layer block (`0x519ab0`): count, head and the mutex.
    pub(crate) fn queue_setup(c: &mut Cpu, r: &mut Rng, count: u32, head: u32) {
        c.bus.write32(0x0051_9ab0 + 0x28, count);
        c.bus.write32(0x0051_9ab0 + 0x30, head);
        c.bus.write32(0x0051_9ab0 + 0xc, r.next_u32() & 0xff);
        c.bus.write32(0x0051_9ab0 + 0x24, 0);
        c.bus.write32(0x0051_9ab0 + 0x1c, r.next_u32() & 3);
        c.bus
            .write32(0x0051_9ac4 + 0x4, 0x7700_0000 | r.next_u32() & 0xffff);
        c.bus.write32(0x0051_9ac4 + 0x8, r.next_u32() & 3);
        c.bus.write32(0x0051_9ac4 + 0xc, r.next_u32() & 0xff);
        c.bus.write32(0x0051_9ad8, DATA + 0x1000);
    }

    /// The EA file test fakes: one gated fake per override slot, shared by
    /// every EA file, async file and stream test (`ea_install_fakes`). Each
    /// answers from its own result cell, so a fixture can make one callee
    /// fail while another succeeds. With the gate word off they run what the
    /// slot held before.
    pub(crate) const EAF_CTRL: u32 = DATA + 0x2_0000;

    pub(crate) const EAF_MAGIC: u32 = 0xeaf0_0001;

    /// `EAF_COST_TAG | n`: every EA fake (and, after `ea_fixed_costs`,
    /// every plain faked call, syscall and jump-out) charges exactly `n`
    /// cycles instead of `fakes::fake`'s 3 to 10, so a path's total cycle
    /// cost names the path (`check_paths`). Any other word: no change.
    pub(crate) const EAF_COST: u32 = DATA + 0x2_0078;

    pub(crate) const EAF_COST_TAG: u32 = 0xeaf0_c000;

    /// Charge `EAF_COST` cycles for a fake call that started at `before`.
    pub(crate) fn ea_fake_cost(c: &mut Cpu, before: u64) {
        let w = c.bus.read32(EAF_COST);
        if w & !0xff == EAF_COST_TAG {
            c.cycles = before + (w & 0xff) as u64;
        }
    }

    fn ea_fixed_miss(c: &mut Cpu, target: u32) {
        let before = c.cycles;
        fakes::fake_call(c, target);
        ea_fake_cost(c, before);
    }

    fn ea_fixed_syscall(c: &mut Cpu, at: u32) {
        let before = c.cycles;
        fakes::fake_syscall(c, at);
        ea_fake_cost(c, before);
    }

    fn ea_fixed_jump_out(c: &mut Cpu, pc: u32, start: u32, end: u32) {
        let before = c.cycles;
        fakes::fake_jump_out(c, pc, start, end);
        ea_fake_cost(c, before);
    }

    /// Fixed 3-cycle fakes (`EAF_COST`): call after `machine`.
    pub(crate) fn ea_fixed_costs(c: &mut Cpu) {
        c.bus.write32(EAF_COST, EAF_COST_TAG | 3);
        c.hooks.miss = ea_fixed_miss;
        c.hooks.syscall = ea_fixed_syscall;
        c.hooks.jump_out = ea_fixed_jump_out;
    }

    /// The request and file-handle blocks `ea_fixed_results` hands out.
    pub(crate) const EAF_REQ: u32 = DATA + 0x2000;

    pub(crate) const EAF_HANDLE: u32 = DATA + 0x3000;

    /// Result cells, one per faked callee.
    pub(crate) const EAF_RET_RELEASE: u32 = DATA + 0x2_0010;

    pub(crate) const EAF_RET_HDRSIZE: u32 = DATA + 0x2_0014;

    pub(crate) const EAF_RET_FIND: u32 = DATA + 0x2_0018;

    pub(crate) const EAF_RET_POPEN: u32 = DATA + 0x2_001c;

    pub(crate) const EAF_RET_PSIZE: u32 = DATA + 0x2_0020;

    pub(crate) const EAF_RET_SETCB: u32 = DATA + 0x2_0024;

    pub(crate) const EAF_RET_FH: u32 = DATA + 0x2_0028;

    pub(crate) const EAF_RET_VALID: u32 = DATA + 0x2_002c;

    pub(crate) const EAF_RET_ALLOC: u32 = DATA + 0x2_0030;

    pub(crate) const EAF_RET_ENQ: u32 = DATA + 0x2_0034;

    pub(crate) const EAF_RET_NEXT: u32 = DATA + 0x2_0038;

    pub(crate) const EAF_RET_SVCQ: u32 = DATA + 0x2_003c;

    pub(crate) const EAF_RET_QOPEN: u32 = DATA + 0x2_0040;

    pub(crate) const EAF_RET_QCLOSE: u32 = DATA + 0x2_0044;

    pub(crate) const EAF_RET_QREAD: u32 = DATA + 0x2_0048;

    pub(crate) const EAF_RET_BIGTYPE: u32 = DATA + 0x2_004c;

    pub(crate) const EAF_RET_PCLOSE: u32 = DATA + 0x2_0050;

    pub(crate) const EAF_RET_HANDLE: u32 = DATA + 0x2_0054;

    pub(crate) const EAF_RET_SALLOC: u32 = DATA + 0x2_0058;

    /// Nonzero: the service-queue fake returns its argument instead of
    /// `EAF_RET_SVCQ`.
    pub(crate) const EAF_SVCQ_ECHO: u32 = DATA + 0x2_005c;

    /// The cells of the queue/close/read/type callees (they shared one cell
    /// before; most fixtures still steer them together).
    pub(crate) fn ea_set_type_rets(c: &mut Cpu, v: u32) {
        for cell in [
            EAF_RET_RELEASE,
            EAF_RET_QOPEN,
            EAF_RET_QCLOSE,
            EAF_RET_QREAD,
            EAF_RET_BIGTYPE,
            EAF_RET_PCLOSE,
        ] {
            c.bus.write32(cell, v);
        }
    }

    /// The cells of the three allocators (request, file handle, stream
    /// request).
    pub(crate) fn ea_set_alloc_rets(c: &mut Cpu, v: u32) {
        for cell in [EAF_RET_ALLOC, EAF_RET_HANDLE, EAF_RET_SALLOC] {
            c.bus.write32(cell, v);
        }
    }

    /// The request allocator hands out `EAF_REQ`, the handle allocator
    /// `EAF_HANDLE`, and the service queue echoes its request.
    pub(crate) fn ea_fixed_results(c: &mut Cpu) {
        ea_set_alloc_rets(c, EAF_REQ);
        c.bus.write32(EAF_RET_HANDLE, EAF_HANDLE);
        c.bus.write32(EAF_SVCQ_ECHO, 1);
    }

    macro_rules! ea_fake_ret {
        ($name:ident, $gen:expr, $cell:expr, $id:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                let before = c.cycles;
                fakes::fake(c, $id, ra);
                ea_fake_cost(c, before);
                c.g[V0][0] = c.bus.read32($cell) as u64;
            }
        };
    }

    ea_fake_ret!(
        ea_fake_alloc_request,
        ee::s14::f_003de670__generated,
        EAF_RET_ALLOC,
        0xeaf0_1000
    );

    ea_fake_ret!(
        ea_fake_alloc_file_handle,
        ee::s14::f_003de800__generated,
        EAF_RET_HANDLE,
        0xeaf0_1001
    );

    ea_fake_ret!(
        ea_fake_release_request,
        ee::s14::f_003dd310__generated,
        EAF_RET_RELEASE,
        0xeaf0_1004
    );

    ea_fake_ret!(
        ea_fake_queue_open,
        ee::s14::f_003dd4e8__generated,
        EAF_RET_QOPEN,
        0xeaf0_1005
    );

    ea_fake_ret!(
        ea_fake_queue_close,
        ee::s14::f_003dd5a0__generated,
        EAF_RET_QCLOSE,
        0xeaf0_1006
    );

    ea_fake_ret!(
        ea_fake_queue_read,
        ee::s14::f_003dd648__generated,
        EAF_RET_QREAD,
        0xeaf0_1007
    );

    ea_fake_ret!(
        ea_fake_big_get_type,
        ee::s14::f_003e2500__generated,
        EAF_RET_BIGTYPE,
        0xeaf0_1008
    );

    ea_fake_ret!(
        ea_fake_big_get_header_size,
        ee::s14::f_003e2580__generated,
        EAF_RET_HDRSIZE,
        0xeaf0_1009
    );

    ea_fake_ret!(
        ea_fake_big_find_entry,
        ee::s14::f_003e2768__generated,
        EAF_RET_FIND,
        0xeaf0_100a
    );

    ea_fake_ret!(
        ea_fake_platform_open,
        ee::s14::f_003e3208__generated,
        EAF_RET_POPEN,
        0xeaf0_100b
    );

    ea_fake_ret!(
        ea_fake_platform_get_size,
        ee::s14::f_003e34e8__generated,
        EAF_RET_PSIZE,
        0xeaf0_100c
    );

    ea_fake_ret!(
        ea_fake_platform_close,
        ee::s14::f_003e32f8__generated,
        EAF_RET_PCLOSE,
        0xeaf0_100d
    );

    ea_fake_ret!(
        ea_fake_set_request_callback,
        ee::s14::f_003dcf70__generated,
        EAF_RET_SETCB,
        0xeaf0_100e
    );

    ea_fake_ret!(
        ea_fake_from_handle,
        ee::s14::f_003def70__generated,
        EAF_RET_FH,
        0xeaf0_100f
    );

    ea_fake_ret!(
        ea_fake_stream_alloc,
        ee::s14::f_003dfcd8__generated,
        EAF_RET_SALLOC,
        0xeaf0_1013
    );

    ea_fake_ret!(
        ea_fake_stream_enqueue,
        ee::s14::f_003dfd58__generated,
        EAF_RET_ENQ,
        0xeaf0_1014
    );

    ea_fake_ret!(
        ea_fake_stream_start_next,
        ee::s14::f_003e0270__generated,
        EAF_RET_NEXT,
        0xeaf0_1015
    );

    /// The service queue: `EAF_RET_SVCQ`, or its request with
    /// `EAF_SVCQ_ECHO` set.
    pub(crate) fn ea_fake_service_queue(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_service_queue),
                ee::s14::f_003ddfa8__generated,
            );
        }
        let req = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let before = c.cycles;
        fakes::fake(c, 0xeaf0_1002, ra);
        ea_fake_cost(c, before);
        c.g[V0][0] = if c.bus.read32(EAF_SVCQ_ECHO) != 0 {
            req
        } else {
            c.bus.read32(EAF_RET_SVCQ) as u64
        };
    }

    /// Yield completes the request block at `DATA + 0x1000` (its status
    /// word), so a wait loop ends after one pass.
    pub(crate) fn ea_fake_yield(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_yield),
                ee::s14::f_003e5398__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let before = c.cycles;
        fakes::fake(c, 0xeaf0_1003, ra);
        ea_fake_cost(c, before);
        c.bus.write32(DATA + 0x1008, 1);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_fake_mutex_lock(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_mutex_lock),
                ee::s14::f_003e5700__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let before = c.cycles;
        fakes::fake(c, 0xeaf0_1010, ra);
        ea_fake_cost(c, before);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_fake_mutex_unlock(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_mutex_unlock),
                ee::s14::f_003e5760__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let before = c.cycles;
        fakes::fake(c, 0xeaf0_1011, ra);
        ea_fake_cost(c, before);
        c.g[V0][0] = 0;
    }

    /// Validate answers `EAF_RET_VALID`; on success (0) it leaves the stream
    /// in the caller's `[SP+4]` and its first word in `[SP]`, like the real
    /// one.
    pub(crate) fn ea_fake_stream_validate(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_stream_validate),
                ee::s14::f_003dfbd0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        // The stream (A0), before `fakes::fake` clobbers the temporaries.
        let stream = c.g[A0][0] as u32;
        let before = c.cycles;
        fakes::fake(c, 0xeaf0_1012, ra);
        ea_fake_cost(c, before);
        let v = c.bus.read32(EAF_RET_VALID);
        c.g[V0][0] = v as u64;
        if v == 0 {
            c.bus.write32(c.g[SP][0] as u32 + 4, stream);
            let inner = c.bus.read32(stream);
            c.bus.write32(c.g[SP][0] as u32, inner);
        }
    }

    thread_local! {
        /// Entry `pc`s of the hand-written side (`ea_counted!`), so a
        /// dispatcher test can assert the hand port itself resumed at each
        /// loop head (`check_events` only prints both sides' resumes pooled).
        static EAF_ENTRIES: std::cell::RefCell<std::collections::BTreeMap<u32, u32>> =
            const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
    }

    pub(crate) fn ea_note_entry(c: &Cpu) {
        EAF_ENTRIES.with(|e| *e.borrow_mut().entry(c.pc).or_default() += 1);
    }

    /// Assert the hand side was entered at every head in `heads` since the
    /// last call, and clear the counts.
    pub(crate) fn ea_assert_entered(heads: &[u32]) {
        let seen = EAF_ENTRIES.with(|e| std::mem::take(&mut *e.borrow_mut()));
        for h in heads {
            assert!(
                seen.get(h).copied().unwrap_or(0) > 0,
                "hand side never resumed at {h:#x} (entries {seen:x?})"
            );
        }
    }

    /// `fn $name(c)`: note the entry `pc`, then run `$f` (wrap it in
    /// `driven!` for `check_events`).
    macro_rules! ea_counted {
        ($name:ident, $f:path) => {
            fn $name(c: &mut Cpu) {
                $crate::game::lib::eafile::tests::ea_note_entry(c);
                $f(c)
            }
        };
    }
    pub(crate) use ea_counted;

    /// Hook every EA file test fake (once per process; one fake per slot,
    /// so the order tests run in does not matter).
    pub(crate) fn ea_install_fakes() {
        use std::ptr::addr_of_mut;
        let hooks: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 22] = [
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DE670),
                ea_fake_alloc_request,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DE800),
                ea_fake_alloc_file_handle,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
                ea_fake_service_queue,
            ),
            (addr_of_mut!(ee::s14::OVERRIDE_003E5398), ea_fake_yield),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DD310),
                ea_fake_release_request,
            ),
            (addr_of_mut!(ee::s14::OVERRIDE_003DD4E8), ea_fake_queue_open),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DD5A0),
                ea_fake_queue_close,
            ),
            (addr_of_mut!(ee::s14::OVERRIDE_003DD648), ea_fake_queue_read),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E2500),
                ea_fake_big_get_type,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E2580),
                ea_fake_big_get_header_size,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E2768),
                ea_fake_big_find_entry,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E3208),
                ea_fake_platform_open,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E34E8),
                ea_fake_platform_get_size,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E32F8),
                ea_fake_platform_close,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
                ea_fake_set_request_callback,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DEF70),
                ea_fake_from_handle,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DFCD8),
                ea_fake_stream_alloc,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DFD58),
                ea_fake_stream_enqueue,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E0270),
                ea_fake_stream_start_next,
            ),
            (addr_of_mut!(ee::s14::OVERRIDE_003E5700), ea_fake_mutex_lock),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E5760),
                ea_fake_mutex_unlock,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DFBD0),
                ea_fake_stream_validate,
            ),
        ];
        for (slot, fake) in hooks {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// Gate of the load-worker fakes (`ea_install_worker_fakes`):
    /// `EAF_MAGIC` arms them. A gate of its own, so the tests of the real Open, Read and
    /// Close bodies (which arm `EAF_CTRL`) still reach those bodies.
    pub(crate) const EAF_WORKER: u32 = DATA + 0x2_0060;

    /// Result of the `EAFile_Open` fake.
    pub(crate) const EAF_RET_OPEN: u32 = DATA + 0x2_0064;

    /// The word the `EAFile_Open` fake stores through its out pointer (A3).
    pub(crate) const EAF_OPEN_OUT: u32 = DATA + 0x2_0068;

    /// Result of the `EAFile_Read` fake.
    pub(crate) const EAF_RET_READ: u32 = DATA + 0x2_006c;

    /// Result of the `EAFile_Close` fake.
    pub(crate) const EAF_RET_CLOSE: u32 = DATA + 0x2_0070;

    /// Index of the next `EAF_SEQ` cell the allocator-callback fake answers.
    pub(crate) const EAF_SEQ_NEXT: u32 = DATA + 0x2_0074;

    /// Eight results the allocator-callback fake answers in call order (a
    /// zero cell is a failed allocation).
    pub(crate) const EAF_SEQ: u32 = DATA + 0x2_0080;

    /// The address a fixture stores as the allocator callback
    /// (`[0x450c14]`) to reach the sequence fake: the slot of the 2-cycle
    /// `EAFile_QueueSignalStub`, which the fake takes over while
    /// `EAF_WORKER` is armed (so a fixture that arms it must not reach
    /// `EAFile_QueueExists`, the stub's caller).
    pub(crate) const EAF_SEQ_FN: u32 = 0x003d_e668;

    macro_rules! ea_worker_fake {
        ($name:ident, $gen:expr, $id:expr, $body:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(EAF_WORKER) != EAF_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                let args = [A0, A1, A2, A3].map(|r| c.g[r][0] as u32);
                let sp = c.g[SP][0] as u32;
                let before = c.cycles;
                fakes::fake(c, $id, ra);
                ea_fake_cost(c, before);
                let body: fn(&mut Cpu, [u32; 4], u32) = $body;
                body(c, args, sp);
            }
        };
    }

    ea_worker_fake!(
        ea_fake_open,
        ee::s14::f_003debf0__generated,
        0xeaf0_1020,
        |c, args, _| {
            let out = args[3];
            let v = c.bus.read32(EAF_RET_OPEN);
            c.g[V0][0] = v as u64;
            let w = if v != 0 {
                c.bus.read32(EAF_OPEN_OUT)
            } else {
                0
            };
            c.bus.write32(out, w);
        }
    );

    ea_worker_fake!(
        ea_fake_read,
        ee::s14::f_003dec60__generated,
        0xeaf0_1021,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_READ) as u64
    );

    ea_worker_fake!(
        ea_fake_close,
        ee::s14::f_003deca0__generated,
        0xeaf0_1022,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_CLOSE) as u64
    );

    ea_worker_fake!(
        ea_fake_alloc_seq,
        ee::s14::f_003de668__generated,
        0xeaf0_1023,
        |c, _, _| {
            c.g[V0][0] = ea_seq_next(c) as u64;
        }
    );

    /// The next `EAF_SEQ` result (0 past the eighth).
    pub(crate) fn ea_seq_next(c: &mut Cpu) -> u32 {
        let i = c.bus.read32(EAF_SEQ_NEXT);
        c.bus.write32(EAF_SEQ_NEXT, i + 1);
        if i < 8 {
            c.bus.read32(EAF_SEQ + 4 * i)
        } else {
            0
        }
    }

    /// The word the `EAFile_LoadFileGetSize` fake stores through its size
    /// pointer (A1) when it hands out a buffer.
    pub(crate) const EAF_GETSIZE_OUT: u32 = DATA + 0x2_007c;

    /// Passes left before the `EAFile_WaitRequest` fake ends a chunked
    /// transfer: 2 clears the caller's remaining-bytes word (`[sp+0xc]`),
    /// 1 also its current request (`[sp+0x20]`), as the chunk callback
    /// does when the last chunk lands. 0: the fake only returns.
    pub(crate) const EAF_WAIT_LEFT: u32 = DATA + 0x2_00a0;

    /// Result of the queue-call fakes (`EAFile_QueueGetSize`,
    /// `QueueMountBig`, `QueueUnmountBig`, `QueueExists`): the request, or
    /// 0 when nothing could be queued.
    pub(crate) const EAF_RET_QUEUE: u32 = DATA + 0x2_00a8;

    ea_worker_fake!(
        ea_fake_queue_get_size,
        ee::s14::f_003dd7e0__generated,
        0xeaf0_1028,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_QUEUE) as u64
    );

    ea_worker_fake!(
        ea_fake_queue_mount_big,
        ee::s14::f_003ddac0__generated,
        0xeaf0_1029,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_QUEUE) as u64
    );

    ea_worker_fake!(
        ea_fake_queue_unmount_big,
        ee::s14::f_003ddc30__generated,
        0xeaf0_102a,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_QUEUE) as u64
    );

    ea_worker_fake!(
        ea_fake_queue_exists,
        ee::s14::f_003dd438__generated,
        0xeaf0_102b,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_QUEUE) as u64
    );

    /// The 16 bytes the `EAFile_LoadFileInto` fake copies into its buffer
    /// (A1) when its `EAF_SEQ` result is nonzero.
    pub(crate) const EAF_INTO_BYTES: u32 = DATA + 0x2_00b0;

    ea_worker_fake!(
        ea_fake_load_file_into,
        ee::s14::f_003e1bb8__generated,
        0xeaf0_102c,
        |c, args, _| {
            let v = ea_seq_next(c);
            if v != 0 {
                for i in 0..16 {
                    let b = c.bus.read8(EAF_INTO_BYTES + i);
                    c.bus.write8(args[1] + i, b);
                }
            }
            c.g[V0][0] = v as u64;
        }
    );

    /// Result of the `EAFile_GetSize` fake.
    pub(crate) const EAF_RET_GETSIZE: u32 = DATA + 0x2_00a4;

    ea_worker_fake!(
        ea_fake_get_size,
        ee::s14::f_003decf8__generated,
        0xeaf0_1027,
        |c, _, _| c.g[V0][0] = c.bus.read32(EAF_RET_GETSIZE) as u64
    );

    ea_worker_fake!(
        ea_fake_load_file,
        ee::s14::f_003e1908__generated,
        0xeaf0_1024,
        |c, _, _| {
            c.g[V0][0] = ea_seq_next(c) as u64;
        }
    );

    ea_worker_fake!(
        ea_fake_load_file_get_size,
        ee::s14::f_003e1a70__generated,
        0xeaf0_1025,
        |c, args, _| {
            let v = ea_seq_next(c);
            if v != 0 {
                let size = c.bus.read32(EAF_GETSIZE_OUT);
                c.bus.write32(args[1], size);
            }
            c.g[V0][0] = v as u64;
        }
    );

    ea_worker_fake!(
        ea_fake_wait_request,
        ee::s14::f_003dd1d8__generated,
        0xeaf0_1026,
        |c, _, sp| {
            let n = c.bus.read32(EAF_WAIT_LEFT);
            if n != 0 {
                c.bus.write32(EAF_WAIT_LEFT, n - 1);
                if n <= 2 {
                    c.bus.write32(sp + 0xc, 0);
                }
                if n == 1 {
                    c.bus.write32(sp + 0x20, 0);
                }
            }
            c.g[V0][0] = 0;
        }
    );

    /// Hook the load-worker fakes (`EAF_WORKER`): the four queue calls
    /// (`EAF_RET_QUEUE`), Open, GetSize, Read, Close,
    /// WaitRequest, LoadFile, LoadFileGetSize and the sequence allocator at
    /// `EAF_SEQ_FN` (LoadFile, LoadFileGetSize, LoadFileInto and the
    /// allocator answer `EAF_SEQ` in call order).
    pub(crate) fn ea_install_worker_fakes() {
        use std::ptr::addr_of_mut;
        let hooks: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 13] = [
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E1BB8),
                ea_fake_load_file_into,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DD7E0),
                ea_fake_queue_get_size,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DDAC0),
                ea_fake_queue_mount_big,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DDC30),
                ea_fake_queue_unmount_big,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DD438),
                ea_fake_queue_exists,
            ),
            (addr_of_mut!(ee::s14::OVERRIDE_003DECF8), ea_fake_get_size),
            (addr_of_mut!(ee::s14::OVERRIDE_003E1908), ea_fake_load_file),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003E1A70),
                ea_fake_load_file_get_size,
            ),
            (
                addr_of_mut!(ee::s14::OVERRIDE_003DD1D8),
                ea_fake_wait_request,
            ),
            (addr_of_mut!(ee::s14::OVERRIDE_003DEBF0), ea_fake_open),
            (addr_of_mut!(ee::s14::OVERRIDE_003DEC60), ea_fake_read),
            (addr_of_mut!(ee::s14::OVERRIDE_003DECA0), ea_fake_close),
            (addr_of_mut!(ee::s14::OVERRIDE_003DE668), ea_fake_alloc_seq),
        ];
        for (slot, fake) in hooks {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// Arm the load-worker fakes with the allocator answering `seq` in
    /// order, and point the allocator callback (`[0x450c14]`) at it; the
    /// free callback (`[0x450c18]`) stays a plain faked call.
    pub(crate) fn ea_worker_fixture(c: &mut Cpu, seq: &[u32]) {
        ea_install_fakes();
        ea_install_worker_fakes();
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        c.bus.write32(EAF_WORKER, EAF_MAGIC);
        c.bus.write32(EAF_SEQ_NEXT, 0);
        for i in 0..8u32 {
            let v = seq.get(i as usize).copied().unwrap_or(0);
            c.bus.write32(EAF_SEQ + 4 * i, v);
        }
        c.bus.write32(0x0045_0c10, 0);
        c.bus.write32(0x0045_0c14, EAF_SEQ_FN);
        c.bus.write32(0x0045_0c18, 0x00c0_0010);
    }

    /// Debug aid for finding `check_paths` totals: panics with each
    /// (label, total) the generated side takes, its seed count and first
    /// seed. `label` names the arm from the final machine (`None`: the
    /// faked call targets and V0).
    #[allow(dead_code)]
    pub(crate) fn ea_probe_paths(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        label: Option<&dyn Fn(&mut Cpu) -> String>,
    ) {
        let mut seen: std::collections::BTreeMap<(String, u64), (u32, u32)> =
            std::collections::BTreeMap::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            let name = match label {
                Some(f) => f(&mut c),
                None => {
                    let n = c.bus.read32(fakes::LOG_COUNT);
                    let calls: Vec<u32> = (0..n.min(12))
                        .map(|i| c.bus.read32(fakes::LOG_START + 0x220 * i))
                        .collect();
                    format!("calls={calls:x?} v0={:x}", c.g[V0][0])
                }
            };
            let e = seen.entry((name, c.cycles - before)).or_insert((0, seed));
            e.0 += 1;
        }
        let lines: Vec<String> = seen
            .iter()
            .map(|((l, k), (n, s))| format!("{k}: n={n} seed={s} {l}"))
            .collect();
        panic!("probe paths:\n{}", lines.join("\n"));
    }

    /// A zeroed request block plus the table entry selecting it.
    pub(crate) fn req_fixture(c: &mut Cpu) {
        for i in 0..12u32 {
            c.bus.write32(EAF_REQ + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        ea_fixed_results(c);
        table_setup(c);
    }

    /// Result/bytes words with a matching status bit, or zeros.
    #[test]
    fn ea_file_get_request_result_matches() {
        check(
            ee::s14::f_003dcf10__generated,
            ea_file_get_request_result,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cf10, 0);
                table_setup(c);
                let i = seed % 4;
                let entry = DATA + 0x1000 + 0x30 * i;
                c.bus.write32(entry + 8, 0x34_0000 + (i << 24));
                c.bus.write32(entry + 0xc, 0x1000 + seed);
                c.g[A0][0] = 0x34_0000 + ((i << 24) as u64);
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_cf10;
            },
        );
    }

    #[test]
    fn ea_file_get_request_bytes_matches() {
        check(
            ee::s14::f_003dcf40__generated,
            ea_file_get_request_bytes,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_cf40, 0);
                table_setup(c);
                let i = seed % 4;
                let entry = DATA + 0x1000 + 0x30 * i;
                c.bus.write32(entry + 8, 0x34_0000 + (i << 24));
                c.bus.write32(entry + 0x14, 0x2000 + seed);
                c.g[A0][0] = 0x34_0000 + ((i << 24) as u64);
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_cf40;
            },
        );
    }

    driven!(
        gen_ea_file_set_request_priority,
        ee::s14::f_003dcfc0__generated
    );

    driven!(
        hand_ea_file_set_request_priority,
        ea_file_set_request_priority
    );

    /// The `EAFile_SetRequestPriority` fixture (see the `_matches` test).
    fn set_request_priority_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x3d_cfc0, 0);
        ea_fixed_costs(c);
        let mode = seed % 6;
        let entry = DATA + 0x1000;
        for i in 0..12u32 {
            c.bus.write32(entry + 4 * i, 0);
        }
        if mode == 0 {
            c.g[A0][0] = 0;
            c.g[A1][0] = seed as u64;
            queue_setup(c, &mut r, 0, 0);
            c.pc = 0x003d_cfc0;
            return r;
        }
        // Status word decides clear vs set.
        let status = if mode == 1 { 0 } else { 0x34_0000 };
        c.bus.write32(entry + 8, status);
        // The request's top byte selects the slot both reads use:
        // S0 = [S1+0x28] + top * 0x30. The table base below puts
        // that slot at DATA+0x400 for every top byte; the low byte
        // is nonzero (a heap address), so reading the wrong byte
        // selects another slot.
        let node = DATA + 0x2000;
        let node2 = DATA + 0x2040;
        for i in 0..12u32 {
            c.bus.write32(node + 4 * i, 0);
            c.bus.write32(node2 + 4 * i, 0);
        }
        let s0 = DATA + 0x400;
        // Service-path combo ((seed/6)%8; only seed%3==0 seeds reach
        // the service, and mode-0 ones returned above, so these are
        // exactly the mode-3 seeds). The old count-coupled indexing
        // starved half the paths (count<2 always queued), so each
        // combo sets its words directly; queue count itself is
        // clobbered by the X override below and read nowhere else.
        // 0 queued (skip all) | 1 gate1-hit | 2 gate2-hit |
        // 3 gate3-hit | 4 scan node->node2 | 5 scan node->s0 |
        // 6 scan at-head | 7 scan empty-head.
        let combo = (seed / 6) % 8;
        let (qval, head, link) = match combo {
            0 => ([0, 1][seed as usize / 48 % 2], node, node2),
            1 | 2 | 3 => ([2, 200][seed as usize / 48 % 2], node, node2),
            4 => ([2, 200][seed as usize / 48 % 2], node, node2),
            5 => ([2, 200][seed as usize / 48 % 2], node, s0),
            6 => ([2, 200][seed as usize / 48 % 2], s0, s0),
            _ => ([2, 200][seed as usize / 48 % 2], 0, node2),
        };
        queue_setup(c, &mut r, 0, head);
        // The status test reads the same slot (top byte, `lbu
        // sp+7`): ([slot] ^ request) & 0xfffff == 0 services; the
        // word there is the request (service), the request ^ 1
        // (the `< 1` boundary) or 0 (early exit).
        let top = r.next_u32() % 4;
        let req = (top << 24) | 0x34_0000 | 0x10 | (r.next_u32() & 0x2f);
        c.bus.write32(0x0051_9ab0 + 0x28, s0 - top * 0x30);
        c.bus.write32(s0, [req, req ^ 1, 0][seed as usize % 3]);
        // [S1+0xc] decides queued below: 0, 1 (queued) vs 2, 200.
        c.bus.write32(0x0051_9ab0 + 0xc, qval);
        // Gate1 ([S1+0x24]): S0 diverts to unlock (combo 1 only).
        if combo == 1 {
            c.bus.write32(0x0051_9ab0 + 0x24, s0);
        }
        // Gate2 ([S0+0x8]): nonzero diverts (combo 2 only).
        if combo == 2 {
            c.bus.write32(DATA + 0x408, 1);
        } else {
            c.bus.write32(DATA + 0x408, 0);
        }
        // P1 (initial [S0+0x10]) only feeds gate3 (V1 == S2): seed
        // diverts (combo 3); small values proceed. (The service
        // overwrites [S0+0x10] with S2 right after the lock, so the
        // HEAD2 compares are (seed < prio).)
        c.bus
            .write32(DATA + 0x410, if combo == 3 { seed } else { seed % 3 });
        // Slot link and C1 -> C2 -> 0 chain (always terminates).
        c.bus.write32(DATA + 0x42c, DATA + 0x450);
        c.bus.write32(DATA + 0x47c, DATA + 0x460);
        c.bus.write32(DATA + 0x48c, 0);
        // C1's priority word ([C1+0x10]): read by the HEAD2-head
        // compare when W == C1, as (seed < C1prio). Seed-scale
        // values cover ==, <, > for all three `<` mutants.
        let c1p = [seed, seed + 1, 0][seed as usize / 9 % 3];
        c.bus.write32(DATA + 0x460, c1p);
        // C2's priority word ([C2+0x10]): read by the HEAD2 loop as
        // (seed < P2), always false here (P2 small); the loop's ==
        // case comes from the s0 link ([s0+0x10] == seed).
        c.bus.write32(DATA + 0x470, seed / 3 % 3);
        // The request: priorities enumerate all 27 (entry, node,
        // node2) combos; node links wrap to S0 on some seeds so the
        // reinsert scan hits its wrap arm as well as its end.
        let pc3 = seed % 27;
        c.bus.write32(entry + 0x10, pc3 % 3);
        c.bus.write32(entry + 0x8 + 0, status);
        c.bus
            .write32(entry + 0x2c, if seed / 48 % 2 == 0 { node } else { 0 });
        c.bus.write32(node + 0x10, (pc3 / 3) % 3);
        c.bus.write32(node + 0x2c, link);
        c.bus.write32(node + 0x8, (seed / 96 % 2) as u32);
        c.bus.write32(node2 + 0x10, (pc3 / 9) % 3);
        c.bus.write32(node2 + 0x2c, 0);
        c.g[A0][0] = req as u64;
        c.g[A1][0] = seed as u64;
        c.pc = 0x003d_cfc0;
        r
    }

    /// Null, status-clear, and set requests; the set ones with short and
    /// long queues, matching/missing heads, busy and idle entries.
    #[test]
    fn ea_file_set_request_priority_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            set_request_priority_setup(c, seed);
        };
        crate::difftest::check_paths(
            gen_ea_file_set_request_priority,
            hand_ea_file_set_request_priority,
            200,
            &[28, 38, 121, 124, 127, 129, 176, 166, 150, 161],
            setup,
        );
    }

    /// The reinsert scans (the request and the queue walks) with events due:
    /// fixture seeds 6k+3 with scan combos 4-7 (`(seed / 6) % 8`), the
    /// event mode from the test seed.
    #[test]
    fn ea_file_set_request_priority_events() {
        check_events(
            gen_ea_file_set_request_priority,
            hand_ea_file_set_request_priority,
            96,
            &|c, seed| {
                let combo = 4 + seed / 3 % 4;
                let fixture = 3 + 6 * combo + 48 * (seed / 12 % 4);
                let mut r = set_request_priority_setup(c, fixture);
                schedule(c, &mut r, seed, 0x003d_cfc0)
            },
        );
    }

    /// Null (returns 0), stale (returns the request >> 20), and live
    /// requests of kinds 3 and 0xa (return 0xa) and another kind (marks
    /// the slot +4 cancelled, returns the slot). The request's top byte
    /// selects the table slot.
    #[test]
    fn ea_file_cancel_request_matches() {
        crate::difftest::check_paths(
            ee::s14::f_003dd148__generated,
            ea_file_cancel_request,
            72,
            &[10, 22, 26, 28, 36],
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d148, 0);
                table_setup(c);
                let mode = seed % 6;
                if mode == 0 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = seed as u64;
                    c.pc = 0x003d_d148;
                    return;
                }
                let entry = DATA + 0x1000;
                let top = seed / 6 % 4;
                let slot = entry + 0x30 * top;
                for i in 0..12u32 {
                    c.bus.write32(slot + 4 * i, 0);
                }
                // Kind is bits 20-23; the low 20 bits are the status match
                // ((word ^ request) & 0xfffff == 0 is live; mode 2 is the
                // `< 1` boundary, mode 5 a stale other kind).
                let kind = [0u32, 3, 3, 0xa, 7, 7][mode as usize];
                let handle = (top << 24) | (kind << 20) | 0x5_0000;
                c.bus.write32(0x0051_9ad8, entry);
                c.bus.write32(
                    slot,
                    [0, 0x5_0000, 0x5_0001, 0x5_0000, 0x5_0000, 0][mode as usize],
                );
                c.g[A0][0] = handle as u64;
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_d148;
            },
        );
    }

    driven!(gen_ea_file_wait_request, ee::s14::f_003dd1d8__generated);

    driven!(hand_ea_file_wait_request, ea_file_wait_request);

    /// Null, status-clear, already-ready, and spinning (the yield fake
    /// delivers the word) requests.
    #[test]
    fn ea_file_wait_request_events() {
        check_events(
            gen_ea_file_wait_request,
            hand_ea_file_wait_request,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d1d8, 0);
                ea_install_fakes();
                table_setup(c);
                let mode = seed % 5;
                let entry = DATA + 0x1000;
                for i in 0..12u32 {
                    c.bus.write32(entry + 4 * i, 0);
                }
                c.bus.write32(0x0051_9ab0 + 0x28, DATA + 0x1000);
                // The request table (`[0x519ad8]`, `table_setup`) starts at
                // entry; the handle's top byte is 0, so its slot is entry.
                if mode == 0 {
                    c.g[A0][0] = 0;
                } else {
                    let handle = 0x33_0000u64;
                    // Status bit from entry+0; the wait word is entry+8.
                    // Bit set iff the word differs in the low 20 bits.
                    // Modes 1 and 4 match (spin; 2's wait word is ready),
                    // 3 is the `< 1` boundary (diff == 1, early -3).
                    // Two spinning match modes keep the event-seed
                    // delivery rate above the check_events threshold.
                    let word = match mode {
                        1 | 2 | 4 => handle as u32 & 0xf_ffff,
                        3 => 0x3_0001,
                        _ => 0,
                    };
                    c.bus.write32(entry, word);
                    c.bus.write32(entry + 8, if mode == 2 { 0x42 } else { 0 });
                    c.bus.write32(EAF_CTRL, EAF_MAGIC);
                    c.g[A0][0] = handle;
                }
                c.g[A1][0] = 0;
                schedule(c, &mut r, seed, 0x003d_d1d8)
            },
        );
    }

    driven!(gen_ea_file_queue_exists, ee::s14::f_003dd438__generated);

    driven!(hand_ea_file_queue_exists, ea_file_queue_exists);

    /// Null and live handles; short and 100-char paths.
    #[test]
    fn ea_file_queue_exists_events() {
        check_events(
            gen_ea_file_queue_exists,
            hand_ea_file_queue_exists,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_d438, 0);
                ea_install_fakes();
                req_fixture(c);
                // Nonzero status word so the 0x0fffff mask below is live
                // (an `&`-for-`|` slip zeroes V1 and is caught).
                c.bus.write32(EAF_REQ, 0x1_2345);
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                let paths = [
                    "f.dat",
                    "",
                    "a/very/long/path/that/goes/on/for/a/while/file.dat",
                ];
                let path = paths[seed as usize % paths.len()];
                put_ascii(c, DATA + 0x400, path);
                c.g[A0][0] = if seed / 3 % 2 == 0 {
                    0
                } else {
                    EAF_HANDLE as u64
                };
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_d438)
            },
        );
    }

    driven!(gen_ea_file_queue_write, ee::s14::f_003dd720__generated);

    driven!(hand_ea_file_queue_write, ea_file_queue_write);

    /// Null and live handles; empty and 64-byte buffers.
    #[test]
    fn ea_file_queue_write_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_d720, 0);
            ea_install_fakes();
            req_fixture(c);
            let len = [0, 1, 64][seed as usize % 3];
            for i in 0..len {
                c.bus.write8(DATA + 0x400 + i as u32, b'a' + i as u8 % 26);
            }
            c.g[A0][0] = if seed / 3 % 2 == 0 {
                0
            } else {
                EAF_HANDLE as u64
            };
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = len as u64;
            c.g[A3][0] = seed as u64;
            c.g[T0][0] = (seed / 6) as u64;
            c.g[T1][0] = (seed / 48) as u64;
            c.pc = 0x003d_d720;
        };
        crate::difftest::check_paths(
            gen_ea_file_queue_write,
            hand_ea_file_queue_write,
            96,
            &[59, 70],
            setup,
        );
    }

    driven!(gen_ea_file_queue_get_size, ee::s14::f_003dd7e0__generated);

    driven!(hand_ea_file_queue_get_size, ea_file_queue_get_size);

    /// Missing and present entries.
    #[test]
    fn ea_file_queue_get_size_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_d7e0, 0);
            ea_install_fakes();
            req_fixture(c);
            if seed % 2 == 0 {
                c.g[A0][0] = 0;
            } else {
                c.g[A0][0] = sx(DATA + 0x800);
                c.bus.write32(DATA + 0x804, 0x400 + seed);
            }
            c.g[A1][0] = sx(DATA + 0x400);
            put_ascii(c, DATA + 0x400, "f.dat");
            c.g[A2][0] = seed as u64;
            c.pc = 0x003d_d7e0;
        };
        crate::difftest::check_paths(
            gen_ea_file_queue_get_size,
            hand_ea_file_queue_get_size,
            64,
            &[38, 40],
            setup,
        );
    }

    driven!(gen_ea_file_queue_nop, ee::s14::f_003dd878__generated);

    driven!(hand_ea_file_queue_nop, ea_file_queue_nop);

    /// Plain request allocation plus queue run.
    #[test]
    fn ea_file_queue_nop_matches() {
        check(
            gen_ea_file_queue_nop,
            hand_ea_file_queue_nop,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d878, 0);
                ea_install_fakes();
                req_fixture(c);
                c.g[A0][0] = seed as u64;
                c.g[A1][0] = (seed / 7) as u64;
                c.g[A2][0] = (seed / 13) as u64;
                c.pc = 0x003d_d878;
            },
        );
    }

    driven!(
        gen_ea_file_on_big_header_read,
        ee::s14::f_003dd8e8__generated
    );

    driven!(hand_ea_file_on_big_header_read, ea_file_on_big_header_read);

    /// Known and unknown archive types; buffered bytes below and above
    /// the header size (queue run vs full header read). The request's top
    /// byte selects a table slot whose +0x24 word (the waiting request) and
    /// +0x10 word (the handle, QueueRead's T0) differ from the waiting
    /// request's own +0x10; SetRequestCallback's result differs from the
    /// handle block's +0x14 word.
    #[test]
    fn ea_file_on_big_header_read_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3d_d8e8, 0);
            ea_fixed_costs(c);
            ea_install_fakes();
            req_fixture(c);
            let top = r.next_u32() % 4;
            let slot = DATA + 0x1000 + 0x30 * top;
            let waiting = DATA + 0xa00 + 0x40 * top;
            zero_block(c, waiting, 16);
            c.bus.write32(slot + 0x24, waiting);
            c.bus.write32(slot + 0x10, 0x5300 + seed);
            c.bus.write32(waiting + 0x10, 0x6600 + top);
            // The handle block (A2): buffer, buffered bytes, size arg,
            // and the request block (+0x10) whose +0x24 gets the
            // waiting request.
            let blk = DATA + 0x900;
            zero_block(c, blk, 8);
            c.bus.write32(blk, DATA + 0x4000);
            c.bus.write32(blk + 0x4, r.next_u32() % 0x40);
            c.bus.write32(blk + 0x8, 0x77);
            c.bus.write32(blk + 0x10, DATA + 0x800);
            c.bus.write32(blk + 0x14, 0x8800 + seed);
            zero_block(c, DATA + 0x800, 16);
            // Allocator callbacks (faked calls).
            c.bus.write32(0x0045_0c14, 0x00c0_0010);
            c.bus.write32(0x0045_0c18, 0x00c0_0000);
            ea_set_type_rets(c, r.next_u32() % 2);
            c.bus.write32(EAF_RET_HDRSIZE, r.next_u32() % 0x40);
            c.bus.write32(EAF_RET_SETCB, 0x7700 + seed);
            c.g[A0][0] = ((top << 24) | EAF_REQ) as u64;
            c.g[A1][0] = sx(DATA + 0x800);
            c.g[A2][0] = sx(blk);
            c.pc = 0x003d_d8e8;
        };
        crate::difftest::check_paths(
            gen_ea_file_on_big_header_read,
            hand_ea_file_on_big_header_read,
            96,
            &[55, 61, 86, 92],
            setup,
        );
    }

    driven!(gen_ea_file_on_big_opened, ee::s14::f_003dda10__generated);

    driven!(hand_ea_file_on_big_opened, ea_file_on_big_opened);

    /// Header fetch (s0 == 1: QueueRead with the slot's handle word as T0)
    /// and ready-mark paths. The request's top byte selects the table slot
    /// (`lbu sp+3`); every slot holds a distinct handle word and the low
    /// byte differs from the top byte.
    #[test]
    fn ea_file_on_big_opened_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3d_da10, 0);
            ea_fixed_costs(c);
            ea_install_fakes();
            req_fixture(c);
            for k in 0..0x40u32 {
                c.bus.write32(DATA + 0x1000 + 0x30 * k + 0x10, 0x5100 + k);
            }
            c.bus.write32(DATA + 0x800, EAF_REQ as u32);
            c.bus.write32(DATA + 0x810, EAF_HANDLE as u32);
            ea_set_type_rets(c, EAF_REQ as u32);
            let top = r.next_u32() % 4;
            let req = (top << 24) | EAF_REQ | 0x10 | (r.next_u32() & 0x2f);
            c.g[A0][0] = if seed % 3 == 0 { 0 } else { req as u64 };
            c.g[A1][0] = if seed % 2 == 0 { 1 } else { 2 };
            c.g[A2][0] = sx(DATA + 0x800);
            c.pc = 0x003d_da10;
        };
        crate::difftest::check_paths(
            gen_ea_file_on_big_opened,
            hand_ea_file_on_big_opened,
            64,
            &[37, 46],
            setup,
        );
    }

    driven!(gen_ea_file_queue_mount_big, ee::s14::f_003ddac0__generated);

    driven!(hand_ea_file_queue_mount_big, ea_file_queue_mount_big);

    /// No mounted archives, a name match (queue run), and a full scan
    /// with no match (mount path).
    #[test]
    fn ea_file_queue_mount_big_events() {
        check_events(
            gen_ea_file_queue_mount_big,
            hand_ea_file_queue_mount_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_dac0, 0);
                ea_install_fakes();
                req_fixture(c);
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                // Independent of the event mode (`schedule`: seed % 3).
                let mode = seed / 3 % 3;
                // Mounted list at DATA+0x900: each node holds a name
                // struct at +0xc whose string starts at +0x10.
                let names = ["other.big", "arch.big", "nomatch.big"];
                let want = ["arch.big", "missing.big", "arch.big"][mode as usize];
                put_ascii(c, DATA + 0x400, want);
                let mut prev = 0u32;
                if mode != 0 {
                    for (k, name) in names.iter().enumerate() {
                        let node = DATA + 0x900 + 64 * k as u32;
                        let st = DATA + 0xb00 + 0x40 * k as u32;
                        put_ascii(c, st + 0x10, name);
                        c.bus.write32(node + 0xc, st);
                        c.bus.write32(node + 0x14, prev);
                        c.bus.write32(node, 0x7000 + k as u32);
                        prev = node;
                    }
                }
                c.bus.write32(0x0051_9ae4, prev);
                // Allocator + root words the mount path reads.
                c.bus.write32(0x0045_0c10 + 0x4, 0x00c0_0000);
                c.bus.write32(0x0045_0c10, 0x00c0_0000);
                c.bus.write32(0x0049_5d68, 0);
                c.bus.write32(0x0049_5d28, 0x3000);
                ea_set_type_rets(c, EAF_REQ as u32);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x800);
                c.bus.write32(DATA + 0x800, EAF_HANDLE as u32);
                c.g[A2][0] = 0;
                c.g[A3][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_dac0)
            },
        );
    }

    driven!(gen_ea_file_on_big_unmounted, ee::s14::f_003ddc00__generated);

    driven!(hand_ea_file_on_big_unmounted, ea_file_on_big_unmounted);

    /// Release plus queue run.
    #[test]
    fn ea_file_on_big_unmounted_matches() {
        check(
            gen_ea_file_on_big_unmounted,
            hand_ea_file_on_big_unmounted,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_dc00, 0);
                ea_install_fakes();
                req_fixture(c);
                ea_set_type_rets(c, 0);
                c.g[A0][0] = EAF_REQ as u64;
                c.g[A2][0] = sx(DATA + 0x800);
                c.bus.write32(DATA + 0x800, EAF_HANDLE as u32);
                c.pc = 0x003d_dc00;
            },
        );
    }

    driven!(
        gen_ea_file_queue_unmount_big,
        ee::s14::f_003ddc30__generated
    );

    crate::game::lib::eafile::tests::ea_counted!(
        counted_ea_file_queue_unmount_big,
        ea_file_queue_unmount_big
    );

    driven!(
        hand_counted_ea_file_queue_unmount_big,
        counted_ea_file_queue_unmount_big
    );

    /// Both scans with events due. The first scan walks the open-file
    /// slots (`[0x519adc]`, `[0x519ab4]` of them, stride 0x110: +8 in use,
    /// +0 the file whose +0xc names its archive); the second walks the
    /// mounted list (`[0x519ae4]`, +0xc the archive handle, +0x14 next).
    /// `kind` (independent of the event mode `seed % 3`): 0 slots in use
    /// by other archives (the first unwind lands on `0x3ddcf0`), 1 unused
    /// slots (`0x3ddcf4`), 2 no slots and a long list of other archives
    /// (`0x3ddd40`), 3 a random mix with the archive's own slot or node
    /// (busy result, or unlink at the list head or behind another node).
    #[test]
    fn ea_file_queue_unmount_big_events() {
        check_events(
            gen_ea_file_queue_unmount_big,
            hand_counted_ea_file_queue_unmount_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_dc30, 0);
                ea_install_fakes();
                req_fixture(c);
                // A nonzero status word keeps the prologue's 0x0fffff
                // mask live.
                c.bus.write32(EAF_REQ, 0x1_2345);
                let kind = seed / 3 % 4;
                let handle = EAF_HANDLE;
                zero_block(c, handle, 12);
                c.bus.write32(handle + 8, r.next_u32() % 2);
                let other = |k: u32| DATA + 0x3100 + 0x10 * (k % 4);
                // Open-file slots.
                let slots = DATA + 0x4000;
                let count = match kind {
                    2 => 0,
                    3 => r.next_u32() % 6,
                    _ => 16,
                };
                for k in 0..count {
                    let slot = slots + 0x110 * k;
                    let file = DATA + 0xa000 + 0x20 * k;
                    let (used, owner) = match kind {
                        0 => (1, other(k)),
                        1 => (0, other(k)),
                        _ => (
                            r.next_u32() % 2,
                            if r.next_u32() % 4 == 0 {
                                handle
                            } else {
                                other(k)
                            },
                        ),
                    };
                    c.bus.write32(slot, file);
                    c.bus.write32(slot + 8, used);
                    c.bus.write32(file + 0xc, owner);
                }
                c.bus.write32(0x0051_9ab4, count);
                c.bus.write32(0x0051_9adc, slots);
                // Mounted list.
                let nodes = match kind {
                    2 => 24,
                    _ => r.next_u32() % 4,
                };
                let own = if kind == 3 { r.next_u32() % 5 } else { 99 };
                let mut prev = 0u32;
                for k in 0..nodes {
                    let node = DATA + 0x900 + 0x40 * k;
                    zero_block(c, node, 8);
                    c.bus.write32(node, 0x6000 + k);
                    c.bus
                        .write32(node + 0xc, if k == own { handle } else { other(k) });
                    c.bus.write32(node + 0x14, prev);
                    prev = node;
                }
                c.bus.write32(0x0051_9ae4, prev);
                c.bus.write32(0x0045_0c18, 0x00c0_0000);
                ea_set_type_rets(c, EAF_REQ as u32);
                c.g[A0][0] = handle as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                put_ascii(c, DATA + 0x400, "arch.big");
                c.g[A2][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_dc30)
            },
        );
        ea_assert_entered(&[0x003d_dc30, 0x003d_dcf0, 0x003d_dcf4, 0x003d_dd40]);
    }

    driven!(
        gen_ea_file_call_with_priority,
        ee::s14::f_003dddf0__generated
    );

    driven!(hand_ea_file_call_with_priority, ea_file_call_with_priority);

    /// The callback runs through the table; the pending count is saved
    /// and restored around it.
    #[test]
    fn ea_file_call_with_priority_matches() {
        check(
            gen_ea_file_call_with_priority,
            hand_ea_file_call_with_priority,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ddf0, 0);
                ea_install_fakes();
                req_fixture(c);
                c.bus.write32(0x0051_9ab0 + 0x8, 0x40 + seed % 8);
                c.g[A0][0] = 0x00c0_1000 + (seed % 2) as u64;
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = seed as u64;
                c.g[A3][0] = sx(DATA + 0x400);
                c.pc = 0x003d_ddf0;
            },
        );
    }

    driven!(gen_ea_file_set_priority, ee::s14::f_003dde50__generated);

    driven!(hand_ea_file_set_priority, ea_file_set_priority);

    /// Stores the priority word and runs the queue.
    #[test]
    fn ea_file_set_priority_events() {
        check_events(
            gen_ea_file_set_priority,
            hand_ea_file_set_priority,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_de50, 0);
                ea_install_fakes();
                c.bus.write32(0x0051_9ab0 + 0x8, 0);
                c.g[A0][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_de50)
            },
        );
    }

    driven!(gen_ea_file_set_root_path, ee::s14::f_003dde78__generated);

    driven!(hand_ea_file_set_root_path, ea_file_set_root_path);

    /// `EAFile_SetRootPath(path)`: a `cd:` path (mode bits 0-1 == 2 keep
    /// the root, else drop the prefix and set mode word 1), a path under
    /// the host root `[0x450c50]` (mode bit 1 drops the prefix and sets
    /// mode word 2, else nothing), any other path; the copied root
    /// (`0x519af0`, 0x100 bytes) gets a `/` appended unless it is empty,
    /// already ends in `/`, or fills the buffer (byte 0xff set).
    #[test]
    fn ea_file_set_root_path_matches() {
        check(
            gen_ea_file_set_root_path,
            hand_ea_file_set_root_path,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_de78, 0);
                elf_image(c);
                let roots = ["", "host:", "host:dir", "cd:dir", "cd:", "other/", "x"];
                let root = roots[seed as usize % roots.len()];
                put_ascii(c, DATA + 0x400, root);
                // The root buffer: old contents, and its last byte.
                fill(c, &mut r, 0x0051_9af0, 0x40);
                c.bus.write8(
                    0x0051_9af0 + 0xff,
                    [0u8, 0, 0x41][r.next_u32() as usize % 3],
                );
                put_ascii(c, DATA + 0x700, "host:");
                c.bus.write32(0x0045_0c50, DATA + 0x700);
                put_ascii(c, DATA + 0x720, "cd:");
                c.bus.write32(0x0045_0c08, DATA + 0x720);
                c.bus
                    .write32(0x0045_0c48, [0x2, 0x0, 0x3, 0x1][r.next_u32() as usize % 4]);
                c.bus.write32(0x0045_0c44, 0);
                c.g[A0][0] = sx(DATA + 0x400);
                c.pc = 0x003d_de78;
            },
        );
    }

    driven!(gen_ea_file_get_root_path, ee::s14::f_003ddf80__generated);

    driven!(hand_ea_file_get_root_path, ea_file_get_root_path);

    /// Copies the current root (`0x519af0`: empty, short, or filling the
    /// 0x100 bytes) into the caller's buffer.
    #[test]
    fn ea_file_get_root_path_events() {
        check_events(
            gen_ea_file_get_root_path,
            hand_ea_file_get_root_path,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_df80, 0);
                elf_image(c);
                zero_block(c, 0x0051_9af0, 0x41);
                match seed / 3 % 3 {
                    0 => {}
                    1 => put_ascii(c, 0x0051_9af0, "host:dir/"),
                    _ => fill(c, &mut r, 0x0051_9af0, 0x41),
                }
                fill(c, &mut r, DATA + 0x600, 0x41);
                c.g[A0][0] = sx(DATA + 0x600);
                schedule(c, &mut r, seed, 0x003d_df80)
            },
        );
    }

    driven!(gen_ea_file_parse_big_path, ee::s14::f_003de4d0__generated);

    driven!(hand_ea_file_parse_big_path, ea_file_parse_big_path);

    /// `EAFile_ParseBigPath` inputs: path A0, flags A1 (bit 0: also try
    /// the plain file), out size A2. Plain paths open the file (open
    /// fails, or size + close); `big|name` scans the mounted list (name
    /// match, mismatch, end of list) and finds the entry in a matching
    /// archive (found exits the scan, missing goes on); `|name` searches
    /// every archive. `events` schedules an event (`schedule`) for a
    /// `|name` search of a 40-archive list: that scan calls no `strcmp`
    /// (a dispatcher whose mid-call unwind would resume at a return
    /// address), and the first due event is put past the prologue's
    /// `strchr` calls, inside the scan.
    fn parse_big_path_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3d_e4d0, 0);
        ea_fixed_costs(c);
        elf_image(c);
        ea_install_fakes();
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        let paths = [
            "file.dat",
            "big|name",
            "|name",
            "arch.big|x",
            "other|y",
            "a|b|c",
        ];
        let path = if events {
            "|name"
        } else {
            paths[seed as usize % paths.len()]
        };
        put_ascii(c, DATA + 0x400, path);
        c.bus
            .write32(EAF_RET_POPEN, [0, 1, 1][r.next_u32() as usize % 3]);
        c.bus.write32(EAF_RET_PSIZE, 0x1234 + seed);
        let find = if events {
            (r.next_u32() % 8 == 0) as u32
        } else {
            r.next_u32() % 3 / 2
        };
        c.bus.write32(EAF_RET_FIND, find);
        // The out size, pre-filled so a missing store shows.
        c.bus.write32(DATA + 0x5000, 0xbeef_beef);
        // Mounted list: name structs hold the string at +0x10.
        let names = ["big", "arch.big", "other", "big", "x.big", "a"];
        let n = if events { 40 } else { r.next_u32() % 6 };
        let mut prev = 0u32;
        for k in 0..n {
            let node = DATA + 0x900 + 64 * k;
            let st = DATA + 0x6000 + 0x40 * k;
            put_ascii(c, st + 0x10, names[(k + seed / 6) as usize % names.len()]);
            c.bus.write32(node, 0x7000 + k);
            c.bus.write32(node + 0xc, st);
            c.bus.write32(node + 0x14, prev);
            prev = node;
        }
        c.bus.write32(0x0051_9ae4, prev);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = (r.next_u32() % 2) as u64;
        c.g[A2][0] = sx(DATA + 0x5000);
        c.g[A3][0] = seed as u64;
        if events {
            let mode = schedule(c, &mut r, seed, 0x003d_e4d0);
            if mode != Events::Off {
                c.next_event += 150;
            }
            mode
        } else {
            c.pc = 0x003d_e4d0;
            Events::Off
        }
    }

    /// Every arm of the parse (see `parse_big_path_setup`).
    #[test]
    fn ea_file_parse_big_path_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            parse_big_path_setup(c, seed, false);
        };
        // Totals with 3-cycle fakes: plain file open fails (148), sized
        // (155; 258 also trying archives); open fails, archive found (271)
        // or not (287); `|name` with no archives (186), one archive missing
        // (201) or found at the end (203), found with archives left (206),
        // two missing (219); `big|name` with no archives (346), a name
        // mismatch (394), a match found with archives left (412), a match
        // missing (509).
        crate::difftest::check_paths(
            gen_ea_file_parse_big_path,
            hand_ea_file_parse_big_path,
            120,
            &[
                148, 155, 258, 271, 287, 186, 201, 203, 206, 219, 346, 394, 412, 509,
            ],
            setup,
        );
    }

    /// The mounted-list scan with events due at its back edge.
    #[test]
    fn ea_file_parse_big_path_events() {
        check_events(
            gen_ea_file_parse_big_path,
            hand_ea_file_parse_big_path,
            120,
            &|c, seed| parse_big_path_setup(c, seed, true),
        );
    }

    driven!(
        gen_ea_file_queue_signal_stub,
        ee::s14::f_003de668__generated
    );

    driven!(hand_ea_file_queue_signal_stub, ea_file_queue_signal_stub);

    /// The 2-cycle return.
    #[test]
    fn ea_file_queue_signal_stub_matches() {
        check(
            gen_ea_file_queue_signal_stub,
            hand_ea_file_queue_signal_stub,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_e668, 0);
                c.pc = 0x003d_e668;
            },
        );
    }

    driven!(gen_ea_file_get_settings, ee::s14::f_003de928__generated);

    driven!(hand_ea_file_get_settings, ea_file_get_settings);

    /// Counts below and above the zero-fill threshold.
    #[test]
    fn ea_file_get_settings_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_e928, 0);
            let count = [0u32, 1, 36, 37, 64, 0xffff][seed as usize % 6];
            c.bus.write32(DATA + 0x100, count);
            c.bus.write32(0x0045_0c58, 0x11 + seed);
            c.bus.write32(0x0045_0c50, 0x22 + seed);
            c.bus.write32(0x0045_0c64, seed % 2);
            c.bus.write32(0x0045_0c5c, 0x44 + seed);
            for i in 0..8u32 {
                c.bus.write32(0x0045_0c10 + 4 * i, 0xbeef_0000 + seed);
            }
            c.g[A0][0] = sx(DATA + 0x100);
            c.pc = 0x003d_e928;
        };
        crate::difftest::check_paths(
            gen_ea_file_get_settings,
            hand_ea_file_get_settings,
            48,
            &[40, 49, 112, 167, 186, 22802],
            setup,
        );
    }

    driven!(gen_ea_file_set_settings, ee::s14::f_003de9b8__generated);

    driven!(hand_ea_file_set_settings, ea_file_set_settings);

    /// Counts below and above the zero-fill threshold, varied words.
    #[test]
    fn ea_file_set_settings_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_e9b8, 0);
            let count = [0u32, 1, 35, 36, 72][seed as usize % 5];
            c.bus.write32(DATA + 0x100, count);
            for i in 1..10u32 {
                c.bus.write32(DATA + 0x100 + 4 * i, 0x1000 * seed + i);
            }
            for i in 0..8u32 {
                c.bus.write32(0x0045_0c10 + 4 * i, 0xbeef_0000 + seed);
            }
            c.g[A0][0] = sx(DATA + 0x100);
            c.pc = 0x003d_e9b8;
        };
        crate::difftest::check_paths(
            gen_ea_file_set_settings,
            hand_ea_file_set_settings,
            48,
            &[108, 113, 117, 118, 173],
            setup,
        );
    }

    driven!(
        gen_ea_file_on_chunk_transferred,
        ee::s14::f_003dea48__generated
    );

    driven!(
        hand_ea_file_on_chunk_transferred,
        ea_file_on_chunk_transferred
    );

    /// Chunk and non-chunk kinds; byte counts below, at and above the
    /// remaining length; null and queue callbacks.
    #[test]
    fn ea_file_on_chunk_transferred_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_ea48, 0);
            ea_install_fakes();
            req_fixture(c);
            // Request block fields the chunk math walks.
            c.bus.write32(EAF_REQ + 0x4, 0x100 + seed);
            c.bus.write32(EAF_REQ + 0x8, 0x200 + seed);
            c.bus.write32(EAF_REQ + 0x10, 0x300 + seed);
            c.bus.write32(EAF_REQ + 0x18, 0x400 + seed);
            c.bus
                .write32(EAF_REQ + 0x14, [0u32, 0x10, 0x1000][seed as usize / 2 % 3]);
            // 0x8000 is the `(0x8000 < remaining)` boundary: it
            // takes the fits arm, unlike 0x9000 above it.
            c.bus.write32(
                EAF_REQ + 0xc,
                [0u32, 0x20, 0x8000, 0x9000][seed as usize / 6 % 4],
            );
            c.bus.write32(
                EAF_REQ + 0x1c,
                if seed / 18 % 2 == 0 { 0 } else { 0x003d_d878 },
            );
            ea_set_type_rets(c, (seed / 36 % 2) as u32);
            c.bus.write32(EAF_RET_SETCB, (seed / 72 % 2) as u32);
            c.g[A0][0] = (seed % 3) as u64;
            c.g[A1][0] = if seed % 2 == 0 { 1 } else { 0 };
            c.g[A2][0] = EAF_REQ as u64;
            c.pc = 0x003d_ea48;
        };
        crate::difftest::check_paths(
            gen_ea_file_on_chunk_transferred,
            hand_ea_file_on_chunk_transferred,
            120,
            &[24, 41, 74, 112],
            setup,
        );
    }

    driven!(gen_ea_file_transfer_chunked, ee::s14::f_003deb50__generated);

    driven!(hand_ea_file_transfer_chunked, ea_file_transfer_chunked);

    /// `EAFile_TransferChunked`: the chunk callback (`T1`, here the
    /// sequence fake) hands out a request or none (exit); the wait loop
    /// runs until the request's callback has cleared the remaining-bytes
    /// and request words (the WaitRequest fake does after 1-40 passes, the
    /// bytes word one pass first). Sizes (A3) below and above the 0x8000
    /// chunk clamp. `events` schedules events in the wait loop.
    fn transfer_chunked_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3d_eb50, 0);
        ea_fixed_costs(c);
        ea_install_fakes();
        req_fixture(c);
        let req = if !events && r.next_u32() % 4 == 0 {
            0
        } else {
            EAF_REQ
        };
        ea_worker_fixture(c, &[req]);
        c.bus.write32(EAF_RET_SETCB, r.next_u32() % 2);
        c.bus.write32(EAF_WAIT_LEFT, 1 + r.next_u32() % 40);
        c.g[A0][0] = 0x1000 + seed as u64;
        c.g[A1][0] = 0x2000 + seed as u64;
        c.g[A2][0] = 0x3000 + seed as u64;
        c.g[A3][0] = [0u32, 0x100, 0x8000, 0x9000][r.next_u32() as usize % 4] as u64;
        c.g[T0][0] = 0x4000 + seed as u64;
        c.g[T1][0] = EAF_SEQ_FN as u64;
        if events {
            schedule(c, &mut r, seed, 0x003d_eb50)
        } else {
            c.pc = 0x003d_eb50;
            Events::Off
        }
    }

    /// No request (exit) and the wait loop (`transfer_chunked_setup`).
    #[test]
    fn ea_file_transfer_chunked_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            transfer_chunked_setup(c, seed, false);
        };
        crate::difftest::check_paths(
            gen_ea_file_transfer_chunked,
            hand_ea_file_transfer_chunked,
            48,
            &[29, 47, 82],
            setup,
        );
    }

    /// The wait loop with events due at its back edges.
    #[test]
    fn ea_file_transfer_chunked_events() {
        check_events(
            gen_ea_file_transfer_chunked,
            hand_ea_file_transfer_chunked,
            48,
            &|c, seed| transfer_chunked_setup(c, seed, true),
        );
    }

    driven!(gen_ea_file_open, ee::s14::f_003debf0__generated);

    driven!(hand_ea_file_open, ea_file_open);

    /// Null and live queue results; varied status words.
    #[test]
    fn ea_file_open_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_ebf0, 0);
            ea_install_fakes();
            req_fixture(c);
            // WAIT/GET_STATUS read the request table (`[0x519ad8]`, set
            // to this slot by `table_setup`): S0 = EAF_REQ, whose low 20
            // bits are 0x2000, so this word matches and the wait word
            // below decides the status (0/1 drive both `< 1` outcomes).
            let slot = DATA + 0x1000;
            c.bus.write32(slot, 0x2_000);
            c.bus.write32(slot + 8, (seed % 2) as u32);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus.write32(EAF_REQ + 0x8, (seed % 3) as u32);
            ea_set_type_rets(c, if seed / 3 % 2 == 0 { 0 } else { EAF_REQ });
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = (seed / 6 % 4) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = sx(DATA + 0x500);
            c.pc = 0x003d_ebf0;
        };
        crate::difftest::check_paths(
            gen_ea_file_open,
            hand_ea_file_open,
            64,
            &[23, 116, 178],
            setup,
        );
    }

    /// Queue-open succeeds with a zero status word: the request word equals
    /// the handle low 24 (inner mask zero -> V1path), and the status word
    /// is zero, so GET_STATUS returns 0. Kills 4250 `< with <=`
    /// (status^1==1: orig S2=0 vs mutant S2=1; S2 is returned).
    /// Deterministic: the lbu reads the handle's top byte (0x00), fixing
    /// both the inner word (base+0) and the status word (base+8).
    #[test]
    fn ea_file_open_zero_status_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_ebf0, 0);
            ea_install_fakes();
            req_fixture(c);
            let slot = DATA + 0x1000;
            // Request word == EAF_REQ low 24 (inner mask zero).
            c.bus.write32(slot, EAF_REQ);
            c.bus.write32(slot + 8, 0);
            // Zero the table window every scratch-derived address reads.
            zero_block(c, slot + 0x10, 64);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus.write32(EAF_REQ + 0x8, 2);
            ea_set_type_rets(c, EAF_REQ);
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = sx(DATA + 0x500);
            c.pc = 0x003d_ebf0;
        };
        check(gen_ea_file_open, hand_ea_file_open, 32, setup);
    }

    driven!(gen_ea_file_read, ee::s14::f_003dec60__generated);

    driven!(hand_ea_file_read, ea_file_read);

    /// Read is a chunked transfer with the read callback; the callback is
    /// faked to null so the transfer exits (a live callback would spin
    /// the wait loop on fixture data; the replay gates cover it).
    #[test]
    fn ea_file_read_matches() {
        check(gen_ea_file_read, hand_ea_file_read, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3d_ec60, 0);
            ea_install_fakes();
            req_fixture(c);
            ea_set_type_rets(c, 0);
            c.bus.write32(EAF_RET_SETCB, (seed / 2 % 2) as u32);
            c.g[A0][0] = 0x1000 + seed as u64;
            c.g[A1][0] = sx(DATA + 0x600);
            c.g[A2][0] = 0x3000 + seed as u64;
            c.g[A3][0] = [0u32, 0x9000][seed as usize / 4 % 2] as u64;
            c.g[T0][0] = 0x4000 + seed as u64;
            // Chunk callback: an empty-pool pop returns 0, taking the
            // null-callback exit (see transfer_chunked).
            c.bus.write32(DATA + 0x700, 0);
            c.g[A0][0] = sx(DATA + 0x700);
            c.g[T1][0] = 0x003d_eed8;
            c.pc = 0x003d_ec60;
        });
    }

    driven!(gen_ea_file_close, ee::s14::f_003deca0__generated);

    driven!(hand_ea_file_close, ea_file_close);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_close_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_eca0, 0);
            ea_install_fakes();
            req_fixture(c);
            ea_set_type_rets(c, if seed % 3 == 0 { 0 } else { EAF_REQ });
            // A zero release returns 0 (nothing released).
            if seed % 2 == 0 {
                c.bus.write32(EAF_RET_RELEASE, 0);
            }
            c.g[A0][0] = EAF_HANDLE as u64;
            c.pc = 0x003d_eca0;
        };
        crate::difftest::check_paths(gen_ea_file_close, hand_ea_file_close, 64, &[18, 71], setup);
    }

    driven!(gen_ea_file_get_size, ee::s14::f_003decf8__generated);

    driven!(hand_ea_file_get_size, ea_file_get_size);

    /// `GetSize(handle)`: nothing queued (returns 0), or the queued size
    /// request waited for and released (its result returned). The queue
    /// call and the wait are worker fakes.
    #[test]
    fn ea_file_get_size_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3d_ecf8, 0);
            ea_install_fakes();
            req_fixture(c);
            ea_worker_fixture(c, &[]);
            c.bus.write32(
                EAF_RET_QUEUE,
                if r.next_u32() % 3 == 0 { 0 } else { EAF_REQ },
            );
            ea_set_type_rets(c, 0x1234 + seed);
            c.g[A0][0] = EAF_HANDLE as u64;
            c.pc = 0x003d_ecf8;
        };
        crate::difftest::check_paths(
            gen_ea_file_get_size,
            hand_ea_file_get_size,
            64,
            &[18, 30],
            setup,
        );
    }

    driven!(gen_ea_file_mount_big, ee::s14::f_003ded50__generated);

    driven!(hand_ea_file_mount_big, ea_file_mount_big);

    /// `MountBig(path, mode, _, out)`: nothing queued, or the queued mount
    /// waited for, its status read (`< 1` both ways) and released.
    #[test]
    fn ea_file_mount_big_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3d_ed50, 0);
            ea_install_fakes();
            req_fixture(c);
            // The request's status slot (`table_setup`; EAF_REQ's top
            // byte is 0): its word matches the request's low 20 bits
            // (0x2000) or not, and the word at +8 decides the status.
            let slot = DATA + 0x1000;
            c.bus.write32(slot, [0x2000, 0][seed as usize / 12 % 2]);
            c.bus.write32(slot + 8, (seed % 2) as u32);
            put_ascii(c, DATA + 0x400, "arch.big");
            c.bus.write32(EAF_REQ + 0x8, (seed % 3) as u32);
            ea_set_type_rets(c, EAF_REQ);
            // The queue call and the wait are worker fakes: nothing
            // queued (returns 0) or the request.
            ea_worker_fixture(c, &[]);
            c.bus
                .write32(EAF_RET_QUEUE, if seed / 3 % 2 == 0 { 0 } else { EAF_REQ });
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = (seed / 6 % 4) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = sx(DATA + 0x500);
            c.pc = 0x003d_ed50;
        };
        crate::difftest::check_paths(
            gen_ea_file_mount_big,
            hand_ea_file_mount_big,
            64,
            &[23, 67, 73],
            setup,
        );
    }

    driven!(gen_ea_file_unmount_big, ee::s14::f_003dedc0__generated);

    driven!(hand_ea_file_unmount_big, ea_file_unmount_big);

    /// `UnmountBig(handle, path)`: nothing queued (returns 0), or the
    /// queued unmount waited for and released (zero and nonzero
    /// releases). The queue call and the wait are worker fakes.
    #[test]
    fn ea_file_unmount_big_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3d_edc0, 0);
            ea_install_fakes();
            req_fixture(c);
            ea_worker_fixture(c, &[]);
            c.bus.write32(
                EAF_RET_QUEUE,
                if r.next_u32() % 3 == 0 { 0 } else { EAF_REQ },
            );
            ea_set_type_rets(c, [0, 0x10 + seed][r.next_u32() as usize % 2]);
            c.g[A0][0] = EAF_HANDLE as u64;
            c.g[A1][0] = sx(DATA + 0x400);
            put_ascii(c, DATA + 0x400, "arch.big");
            c.pc = 0x003d_edc0;
        };
        crate::difftest::check_paths(
            gen_ea_file_unmount_big,
            hand_ea_file_unmount_big,
            64,
            &[21, 40],
            setup,
        );
    }

    driven!(gen_ea_file_exists, ee::s14::f_003dee18__generated);

    driven!(hand_ea_file_exists, ea_file_exists);

    /// `Exists(path)`: nothing queued (returns 0), or the queued query
    /// waited for and released (zero and nonzero releases). The queue call
    /// and the wait are worker fakes.
    #[test]
    fn ea_file_exists_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3d_ee18, 0);
            ea_install_fakes();
            req_fixture(c);
            ea_worker_fixture(c, &[]);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus.write32(
                EAF_RET_QUEUE,
                if r.next_u32() % 3 == 0 { 0 } else { EAF_REQ },
            );
            ea_set_type_rets(c, [0, 0x20 + seed][r.next_u32() as usize % 2]);
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.pc = 0x003d_ee18;
        };
        crate::difftest::check_paths(
            gen_ea_file_exists,
            hand_ea_file_exists,
            64,
            &[25, 47],
            setup,
        );
    }

    /// A zeroed stream block with selectable header fields.
    pub(crate) fn stream_block(c: &mut Cpu, base: u32) {
        for i in 0..64u32 {
            c.bus.write32(base + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    driven!(gen_ea_file_exists_default, ee::s14::f_003e1700__generated);

    driven!(hand_ea_file_exists_default, ea_file_exists_default);

    /// The default-settings word selects null vs live queue results.
    #[test]
    fn ea_file_exists_default_matches() {
        check(
            gen_ea_file_exists_default,
            hand_ea_file_exists_default,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1700, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, [0u32, 1][seed as usize % 2]);
                ea_set_type_rets(c, (seed / 2 % 2) as u32);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                // QueueExists resolves a null alloc to the flags word
                // 0x800000 at [0]; pre-complete that pseudo-request so the
                // real WaitRequest exits immediately instead of yielding
                // forever (no other thread can complete it at unit level).
                // WaitRequest's S1 is NOT the request pointer: its prologue
                // sets S1=[0x519AD8]+top_byte(A0), so the pool word is
                // pinned to 0x800000 to land S1 on the pre-completed block
                // (a zero pool word spins in EAThread_Yield forever).
                c.bus.write32(0, 0);
                c.bus.write32(0x0080_0008, 1);
                c.bus.write32(0x0051_9AD8, 0x0080_0000);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.pc = 0x003e_1700;
            },
        );
    }

    driven!(
        gen_ea_file_get_file_size_worker,
        ee::s14::f_003e1728__generated
    );

    driven!(
        hand_ea_file_get_file_size_worker,
        ea_file_get_file_size_worker
    );

    /// The load workers' fixture (`worker(prio, params)`): `params` holds
    /// the path (+0), the destination (+4), a size word (+8, the
    /// get-size worker's out word) and the allocator's third argument
    /// (+0x10). `arm`: 0 the open fails, 1 the allocator has no buffer,
    /// 2 every call succeeds. Open, GetSize, Read and Close are the worker
    /// fakes; the file is 0x123 + seed bytes.
    fn load_worker_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32, arm: u32) {
        let mut r = machine(c, seed, salt, 0);
        ea_fixed_costs(c);
        let buf = DATA + 0x1_0000;
        let seq = [if arm == 1 { 0 } else { buf }];
        ea_worker_fixture(c, &seq);
        c.bus.write32(EAF_RET_OPEN, (arm != 0) as u32);
        c.bus.write32(EAF_OPEN_OUT, 0x4400 + seed);
        c.bus.write32(EAF_RET_GETSIZE, 0x123 + seed);
        c.bus.write32(EAF_RET_READ, r.next_u32() & 0xfff);
        c.bus.write32(EAF_RET_CLOSE, r.next_u32() % 2);
        let params = DATA + 0x400;
        put_ascii(c, DATA + 0x500, "file.dat");
        zero_block(c, params, 8);
        c.bus.write32(params, DATA + 0x500);
        c.bus.write32(params + 4, DATA + 0x1_8000);
        c.bus.write32(params + 8, 0xdddd_0000 + seed);
        c.bus.write32(params + 0x10, 0x77);
        c.g[A0][0] = (r.next_u32() % 5) as u64;
        c.g[A1][0] = sx(params);
        c.pc = entry;
    }

    /// The open-failed and loaded exits (`load_worker_setup`; this worker
    /// allocates nothing).
    #[test]
    fn ea_file_get_file_size_worker_matches() {
        crate::difftest::check_paths(
            gen_ea_file_get_file_size_worker,
            hand_ea_file_get_file_size_worker,
            48,
            &[22, 34],
            &|c, seed| load_worker_setup(c, seed, 0x3e_1728, 0x003e_1728, seed % 3),
        );
    }

    driven!(gen_ea_file_get_file_size_ex, ee::s14::f_003e1798__generated);

    driven!(hand_ea_file_get_file_size_ex, ea_file_get_file_size_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_get_file_size_ex_matches() {
        check(
            gen_ea_file_get_file_size_ex,
            hand_ea_file_get_file_size_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1798, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_1798;
            },
        );
    }

    driven!(gen_ea_file_get_file_size, ee::s14::f_003e17d8__generated);

    driven!(hand_ea_file_get_file_size, ea_file_get_file_size);

    /// Same without the priority flag.
    #[test]
    fn ea_file_get_file_size_matches() {
        check(
            gen_ea_file_get_file_size,
            hand_ea_file_get_file_size,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_17d8, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_17d8;
            },
        );
    }

    driven!(gen_ea_file_load_file_worker, ee::s14::f_003e1810__generated);

    driven!(hand_ea_file_load_file_worker, ea_file_load_file_worker);

    /// The open-failed, no-buffer and loaded exits (`load_worker_setup`).
    #[test]
    fn ea_file_load_file_worker_matches() {
        crate::difftest::check_paths(
            gen_ea_file_load_file_worker,
            hand_ea_file_load_file_worker,
            48,
            &[24, 48, 58],
            &|c, seed| load_worker_setup(c, seed, 0x3e_1810, 0x003e_1810, seed % 3),
        );
    }

    driven!(gen_ea_file_load_file_ex, ee::s14::f_003e18c8__generated);

    driven!(hand_ea_file_load_file_ex, ea_file_load_file_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_file_ex_matches() {
        check(
            gen_ea_file_load_file_ex,
            hand_ea_file_load_file_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_18c8, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_18c8;
            },
        );
    }

    driven!(gen_ea_file_load_file, ee::s14::f_003e1908__generated);

    driven!(hand_ea_file_load_file, ea_file_load_file);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_file_matches() {
        check(
            gen_ea_file_load_file,
            hand_ea_file_load_file,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1908, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1908;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_get_size_worker,
        ee::s14::f_003e1948__generated
    );

    driven!(
        hand_ea_file_load_file_get_size_worker,
        ea_file_load_file_get_size_worker
    );

    /// The open-failed, no-buffer and loaded exits (`load_worker_setup`).
    #[test]
    fn ea_file_load_file_get_size_worker_matches() {
        crate::difftest::check_paths(
            gen_ea_file_load_file_get_size_worker,
            hand_ea_file_load_file_get_size_worker,
            48,
            &[26, 50, 61],
            &|c, seed| load_worker_setup(c, seed, 0x3e_1948, 0x003e_1948, seed % 3),
        );
    }

    driven!(
        gen_ea_file_load_file_get_size_ex,
        ee::s14::f_003e1a10__generated
    );

    driven!(
        hand_ea_file_load_file_get_size_ex,
        ea_file_load_file_get_size_ex
    );

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_file_get_size_ex_matches() {
        check(
            gen_ea_file_load_file_get_size_ex,
            hand_ea_file_load_file_get_size_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1a10, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_1a10;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_get_size,
        ee::s14::f_003e1a70__generated
    );

    driven!(hand_ea_file_load_file_get_size, ea_file_load_file_get_size);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_file_get_size_matches() {
        check(
            gen_ea_file_load_file_get_size,
            hand_ea_file_load_file_get_size,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1a70, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.bus.write32(DATA + 0x500, 0xdddd_dddd);
                c.g[A0][0] = sx(DATA + 0x400);
                // A null size pointer skips the size store.
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(DATA + 0x500) };
                c.g[A2][0] = (seed / 4 % 7) as u64;
                c.pc = 0x003e_1a70;
            },
        );
    }

    driven!(
        gen_ea_file_load_file_into_worker,
        ee::s14::f_003e1ad0__generated
    );

    driven!(
        hand_ea_file_load_file_into_worker,
        ea_file_load_file_into_worker
    );

    /// The open-failed and loaded exits (`load_worker_setup`; this worker
    /// allocates nothing).
    #[test]
    fn ea_file_load_file_into_worker_matches() {
        crate::difftest::check_paths(
            gen_ea_file_load_file_into_worker,
            hand_ea_file_load_file_into_worker,
            48,
            &[30, 45],
            &|c, seed| load_worker_setup(c, seed, 0x3e_1ad0, 0x003e_1ad0, seed % 3),
        );
    }

    /// Size word 0: S1==0 takes the `V0`-store arm. Kills the 4873
    /// `< with <=` pair (orig V1=0 vs mutant V1=1: S1 stays 0 vs becomes
    /// 0x7fffffff).
    #[test]
    fn ea_file_load_file_into_worker_zero_size_matches() {
        crate::difftest::check(
            gen_ea_file_load_file_into_worker,
            hand_ea_file_load_file_into_worker,
            32,
            &|c, seed| {
                load_worker_setup(c, seed, 0x3e_1ad0, 0x003e_1ad0, 1);
                // Params block: [path, buf, size, ...]; force size 0.
                c.bus.write32(DATA + 0x400 + 8, 0);
            },
        );
    }

    driven!(
        gen_ea_file_load_file_into_ex,
        ee::s14::f_003e1b68__generated
    );

    driven!(hand_ea_file_load_file_into_ex, ea_file_load_file_into_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_file_into_ex_matches() {
        check(
            gen_ea_file_load_file_into_ex,
            hand_ea_file_load_file_into_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1b68, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1b68;
            },
        );
    }

    driven!(gen_ea_file_load_file_into, ee::s14::f_003e1bb8__generated);

    driven!(hand_ea_file_load_file_into, ea_file_load_file_into);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_file_into_matches() {
        check(
            gen_ea_file_load_file_into,
            hand_ea_file_load_file_into,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1bb8, 0);
                ea_install_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                ea_set_type_rets(c, 0);
                ea_set_alloc_rets(c, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1bb8;
            },
        );
    }

    /// Null queue/alloc/service results so the real Open/Close/Read
    /// workers take their early exits.
    pub(crate) fn ea_null_results(c: &mut Cpu) {
        ea_set_type_rets(c, 0);
        ea_set_alloc_rets(c, 0);
        c.bus.write32(EAF_RET_SVCQ, 0);
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    driven!(
        gen_ea_file_load_files_worker,
        ee::s14::f_003e1c00__generated
    );

    driven!(hand_ea_file_load_files_worker, ea_file_load_files_worker);

    /// `EAFile_LoadFilesWorker(_, list)`: `list` = count, path-pointer
    /// array, out-pointer array, LoadFile's second argument. Each LoadFile
    /// (faked, answering `EAF_SEQ` in order: a buffer, or 0, which clears
    /// the all-loaded result) stores into the out array (pre-filled, so a
    /// missing store shows). Empty, short and (`events`) 24-entry lists.
    fn load_files_worker_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_1c00, 0);
        ea_fixed_costs(c);
        let seq: Vec<u32> = (0..8)
            .map(|i| {
                if r.next_u32() % 4 == 0 {
                    0
                } else {
                    DATA + 0x1_0000 + 0x1000 * i
                }
            })
            .collect();
        ea_worker_fixture(c, &seq);
        let list = DATA + 0x400;
        let count = if events { 24 } else { r.next_u32() % 5 };
        let paths = DATA + 0x700;
        let out = DATA + 0x800;
        for i in 0..count {
            put_ascii(c, DATA + 0x1000 + i * 0x20, "f.dat");
            c.bus.write32(paths + 4 * i, DATA + 0x1000 + i * 0x20);
            c.bus.write32(out + 4 * i, 0xdddd_0000 + i);
        }
        c.bus.write32(list, count);
        c.bus.write32(list + 4, paths);
        c.bus.write32(list + 8, out);
        c.bus.write32(list + 12, r.next_u32() % 2);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(list);
        if events {
            schedule(c, &mut r, seed, 0x003e_1c00)
        } else {
            c.pc = 0x003e_1c00;
            Events::Off
        }
    }

    /// Empty and short lists, all loaded or some not
    /// (`load_files_worker_setup`).
    #[test]
    fn ea_file_load_files_worker_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            load_files_worker_setup(c, seed, false);
        };
        crate::difftest::check_paths(
            gen_ea_file_load_files_worker,
            hand_ea_file_load_files_worker,
            48,
            &[19, 40, 100],
            setup,
        );
    }

    /// A 24-entry list with events due at the loop's back edge.
    #[test]
    fn ea_file_load_files_worker_events() {
        check_events(
            gen_ea_file_load_files_worker,
            hand_ea_file_load_files_worker,
            48,
            &|c, seed| load_files_worker_setup(c, seed, true),
        );
    }

    driven!(gen_ea_file_load_files_ex, ee::s14::f_003e1c98__generated);

    driven!(hand_ea_file_load_files_ex, ea_file_load_files_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_files_ex_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3e_1c98, 0);
            ea_install_fakes();
            ea_null_results(c);
            c.bus.write32(0x0045_0c38, 0x99 + seed);
            c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
            put_ascii(c, DATA + 0x400, "f.dat");
            c.g[A0][0] = (seed % 5) as u64;
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x500);
            c.g[A3][0] = sx(DATA + 0x600);
            c.pc = 0x003e_1c98;
        };
        crate::difftest::check_paths(
            gen_ea_file_load_files_ex,
            hand_ea_file_load_files_ex,
            32,
            &[10, 177, 505],
            setup,
        );
    }

    /// Priority-call worker returns 1: the real LoadFilesWorker is driven
    /// (worker fn 0x3e1c00), and its per-file results come from the EAF_SEQ
    /// cells, so forcing those nonzero makes every V1 check pass and the
    /// worker return 1 (X = 1^1 = 0). Kills 5061 `^ with |` (orig X=0 vs
    /// mutant X=1) and 5062 `< with >` (orig (0<1)=1 vs mutant (0>1)=0).
    #[test]
    fn ea_file_load_files_ex_worker_one_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3e_1c98, 0);
            ea_install_fakes();
            ea_install_worker_fakes();
            ea_null_results(c);
            c.bus.write32(0x0045_0c38, 0x99 + seed);
            c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
            // Arm the load-worker fakes so LoadFile answers EAF_SEQ.
            c.bus.write32(EAF_WORKER, EAF_MAGIC);
            // LoadFile answers nonzero for every file in the list.
            for i in 0..4u32 {
                c.bus.write32(EAF_SEQ + 4 * i, 0x4400 + i);
            }
            put_ascii(c, DATA + 0x400, "f.dat");
            c.g[A0][0] = 1 + (seed % 4) as u64;
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x500);
            c.g[A3][0] = sx(DATA + 0x600);
            c.pc = 0x003e_1c98;
        };
        check(
            gen_ea_file_load_files_ex,
            hand_ea_file_load_files_ex,
            32,
            setup,
        );
    }

    driven!(gen_ea_file_load_files, ee::s14::f_003e1d00__generated);

    driven!(hand_ea_file_load_files, ea_file_load_files);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_files_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3e_1d00, 0);
            ea_install_fakes();
            ea_null_results(c);
            c.bus.write32(0x0045_0c38, 0x99 + seed);
            c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
            put_ascii(c, DATA + 0x400, "f.dat");
            c.g[A0][0] = (seed % 5) as u64;
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x500);
            c.g[A3][0] = sx(DATA + 0x600);
            c.pc = 0x003e_1d00;
        };
        crate::difftest::check_paths(
            gen_ea_file_load_files,
            hand_ea_file_load_files,
            32,
            &[11, 177, 505],
            setup,
        );
    }

    /// Same EAF_SEQ forcing for the non-Ex form. Kills 5101 `^ with |`
    /// and 5102 `< with >` (same X=0 divergence).
    #[test]
    fn ea_file_load_files_worker_one_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3e_1d00, 0);
            ea_install_fakes();
            ea_install_worker_fakes();
            ea_null_results(c);
            c.bus.write32(0x0045_0c38, 0x99 + seed);
            c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
            // Arm the load-worker fakes so LoadFile answers EAF_SEQ.
            c.bus.write32(EAF_WORKER, EAF_MAGIC);
            // LoadFile answers nonzero for every file in the list.
            for i in 0..4u32 {
                c.bus.write32(EAF_SEQ + 4 * i, 0x4400 + i);
            }
            put_ascii(c, DATA + 0x400, "f.dat");
            c.g[A0][0] = 1 + (seed % 4) as u64;
            c.g[A1][0] = sx(DATA + 0x400);
            c.g[A2][0] = sx(DATA + 0x500);
            c.g[A3][0] = sx(DATA + 0x600);
            c.pc = 0x003e_1d00;
        };
        check(gen_ea_file_load_files, hand_ea_file_load_files, 32, setup);
    }

    driven!(
        gen_ea_file_load_big_header_worker,
        ee::s14::f_003e1d68__generated
    );

    driven!(
        hand_ea_file_load_big_header_worker,
        ea_file_load_big_header_worker
    );

    /// `EAFile_LoadBigHeaderWorker(prio, params)`: `params` holds the
    /// path (+0) and the allocator's third argument (+8). `arm` picks the
    /// exit: 0 open fails, 1 no buffer, 2 not a big file (free + close),
    /// 3 header fits the first buffer (close, return it), 4 no second
    /// buffer (back edge to the free + close block), 5 full header (copy,
    /// free, read the rest, close). Open, Read, Close and the allocator
    /// are the worker fakes.
    fn load_big_header_worker_setup(c: &mut Cpu, seed: u32, arm: u32) -> Rng {
        let mut r = machine(c, seed, 0x3e_1d68, 0);
        ea_fixed_costs(c);
        let (buf1, buf2) = (DATA + 0x1_0000, DATA + 0x1_4000);
        let seq: &[u32] = match arm {
            0..=1 => &[0],
            2 | 3 => &[buf1],
            4 => &[buf1, 0],
            _ => &[buf1, buf2],
        };
        ea_worker_fixture(c, seq);
        fill(c, &mut r, buf1, 0x2a4);
        c.bus.write32(EAF_RET_OPEN, (arm != 0) as u32);
        c.bus.write32(EAF_OPEN_OUT, 0x4400 + seed);
        c.bus.write32(EAF_RET_READ, r.next_u32() & 0xfff);
        c.bus.write32(EAF_RET_CLOSE, r.next_u32() % 2);
        c.bus.write32(EAF_RET_BIGTYPE, (arm != 2) as u32);
        let hdr = match arm {
            3 => [0x10u32, 0xa90][r.next_u32() as usize % 2],
            _ => [0xa91u32, 0x1000][r.next_u32() as usize % 2],
        };
        c.bus.write32(EAF_RET_HDRSIZE, hdr);
        let params = DATA + 0x400;
        put_ascii(c, DATA + 0x500, "big.dat");
        zero_block(c, params, 4);
        c.bus.write32(params, DATA + 0x500);
        c.bus.write32(params + 8, 0x10 + seed % 7);
        c.g[A0][0] = (r.next_u32() % 5) as u64;
        c.g[A1][0] = sx(params);
        c.pc = 0x003e_1d68;
        r
    }

    /// Every exit of the worker (`load_big_header_worker_setup`).
    #[test]
    fn ea_file_load_big_header_worker_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            load_big_header_worker_setup(c, seed, seed % 6);
        };
        crate::difftest::check_paths(
            gen_ea_file_load_big_header_worker,
            hand_ea_file_load_big_header_worker,
            60,
            &[30, 49, 72, 73, 92, 560],
            setup,
        );
    }

    crate::game::lib::eafile::tests::ea_counted!(
        counted_ea_file_load_big_header_worker,
        ea_file_load_big_header_worker
    );

    driven!(
        hand_counted_ea_file_load_big_header_worker,
        counted_ea_file_load_big_header_worker
    );

    /// The no-second-buffer back edge (`0x3e1e28 -> 0x3e1df8`) with events
    /// due: an unwind there re-enters the hand port at the resume head.
    #[test]
    fn ea_file_load_big_header_worker_events() {
        check_events(
            gen_ea_file_load_big_header_worker,
            hand_counted_ea_file_load_big_header_worker,
            48,
            &|c, seed| {
                let mut r = load_big_header_worker_setup(c, seed, 4);
                schedule(c, &mut r, seed, 0x003e_1d68)
            },
        );
        ea_assert_entered(&[0x003e_1d68, 0x003e_1df8]);
    }

    driven!(
        gen_ea_file_load_big_header_ex,
        ee::s14::f_003e1ec8__generated
    );

    driven!(hand_ea_file_load_big_header_ex, ea_file_load_big_header_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_big_header_ex_matches() {
        check(
            gen_ea_file_load_big_header_ex,
            hand_ea_file_load_big_header_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1ec8, 0);
                ea_install_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii(c, DATA + 0x400, "big.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1ec8;
            },
        );
    }

    driven!(gen_ea_file_load_big_header, ee::s14::f_003e1f08__generated);

    driven!(hand_ea_file_load_big_header, ea_file_load_big_header);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_big_header_matches() {
        check(
            gen_ea_file_load_big_header,
            hand_ea_file_load_big_header,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1f08, 0);
                ea_install_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii(c, DATA + 0x400, "big.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1f08;
            },
        );
    }

    driven!(
        gen_ea_file_get_unpacked_size,
        ee::s14::f_003e2130__generated
    );

    driven!(hand_ea_file_get_unpacked_size, ea_file_get_unpacked_size);

    /// `GetUnpackedSize(path)`: the first 0x80 bytes of the file
    /// (LoadFileInto, a worker fake) or nothing (0); a refpack header
    /// (`10 fb`, a 24-bit big-endian size) or another header (0).
    #[test]
    fn ea_file_get_unpacked_size_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3e_2130, 0);
            let loaded = r.next_u32() % 3 != 0;
            ea_worker_fixture(c, &[if loaded { DATA + 0x1_0000 } else { 0 }]);
            let kind = [0x10u8, 0x11, 0x90, 0x00][r.next_u32() as usize % 4];
            let magic = [0xfbu8, 0xfb, 0xfb, 0x12][r.next_u32() as usize % 4];
            let n = r.next_u32();
            for (i, b) in [kind, magic, (n >> 16) as u8, (n >> 8) as u8, n as u8]
                .iter()
                .enumerate()
            {
                c.bus.write8(EAF_INTO_BYTES + i as u32, *b);
            }
            put_ascii(c, DATA + 0x400, "pack.dat");
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = sx(DATA + 0x500);
            c.pc = 0x003e_2130;
        };
        crate::difftest::check_paths(
            gen_ea_file_get_unpacked_size,
            hand_ea_file_get_unpacked_size,
            48,
            &[19, 30, 46],
            setup,
        );
    }

    driven!(
        gen_ea_file_read_unpacked_size,
        ee::s14::f_003e2168__generated
    );

    driven!(hand_ea_file_read_unpacked_size, ea_file_read_unpacked_size);

    /// Same reader without the null check.
    #[test]
    fn ea_file_read_unpacked_size_matches() {
        check(
            gen_ea_file_read_unpacked_size,
            hand_ea_file_read_unpacked_size,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2168, 0);
                ea_install_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, DATA + 0x400, "pack.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2168;
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed,
        ee::s14::f_003e2190__generated
    );

    driven!(hand_ea_file_load_decompressed, ea_file_load_decompressed);

    /// The load-decompressed fixture: `path` (A0) loads (LoadFileGetSize,
    /// faked) a refpack file at `DATA+0x6000` (`10 fb` header, 0x10
    /// unpacked bytes: one 16-byte literal run and the stop code), or
    /// nothing. `arm`: 0 no file, 1 a zero size, 2 a bad header (unpacked
    /// size 0), 3 no buffer for the packed copy, 4 no buffer for the output
    /// (then the 64-pass delay loop), 5 the full unpack (and the loop). The
    /// allocator answers `EAF_SEQ` after the load.
    fn load_decompressed_setup(c: &mut Cpu, seed: u32, arm: u32) -> Rng {
        let mut r = machine(c, seed, 0x3e_2190, 0);
        ea_fixed_costs(c);
        let pack = DATA + 0x6000;
        let (buf1, buf2) = (DATA + 0x1_0000, DATA + 0x1_4000);
        let seq: &[u32] = match arm {
            0 => &[0],
            1 | 2 => &[pack],
            3 => &[pack, 0],
            4 => &[pack, buf1, 0],
            _ => &[pack, buf1, buf2],
        };
        ea_worker_fixture(c, seq);
        zero_block(c, buf1, 16);
        zero_block(c, buf2, 16);
        let mut file = vec![0x10u8, if arm == 2 { 0x00 } else { 0xfb }, 0, 0, 0x10, 0xe3];
        file.extend((0..16).map(|i| b'a' + i));
        file.push(0xfc);
        for (i, b) in file.iter().enumerate() {
            c.bus.write8(pack + i as u32, *b);
        }
        c.bus.write32(
            EAF_GETSIZE_OUT,
            if arm == 1 { 0 } else { file.len() as u32 },
        );
        put_ascii(c, DATA + 0x400, "pack.dat");
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = [0u32, 0x100, 0x1234][r.next_u32() as usize % 3] as u64;
        c.g[A2][0] = c.g[A1][0];
        c.pc = 0x003e_2190;
        r
    }

    /// Every exit (`load_decompressed_setup`).
    #[test]
    fn ea_file_load_decompressed_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            load_decompressed_setup(c, seed, seed % 6);
        };
        crate::difftest::check_paths(
            gen_ea_file_load_decompressed,
            hand_ea_file_load_decompressed,
            60,
            &[31, 47, 71, 92, 777, 995],
            setup,
        );
    }

    /// The unpack paths with events due in the delay loop.
    #[test]
    fn ea_file_load_decompressed_events() {
        check_events(
            gen_ea_file_load_decompressed,
            hand_ea_file_load_decompressed,
            48,
            &|c, seed| {
                let mut r = load_decompressed_setup(c, seed, 4 + seed / 3 % 2);
                schedule(c, &mut r, seed, 0x003e_2190)
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed_thunk,
        ee::s14::f_003e22f0__generated
    );

    driven!(
        hand_ea_file_load_decompressed_thunk,
        ea_file_load_decompressed_thunk
    );

    /// Thunk forwards to the worker.
    #[test]
    fn ea_file_load_decompressed_thunk_matches() {
        check(
            gen_ea_file_load_decompressed_thunk,
            hand_ea_file_load_decompressed_thunk,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_22f0, 0);
                ea_install_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, DATA + 0x400, "pack.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_22f0;
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed_into,
        ee::s14::f_003e2310__generated
    );

    driven!(
        hand_ea_file_load_decompressed_into,
        ea_file_load_decompressed_into
    );

    /// Into-buffer variant: the output buffer (A1) and flags (A2), the
    /// same files and allocator answers (`load_decompressed_setup`).
    #[test]
    fn ea_file_load_decompressed_into_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            load_decompressed_setup(c, seed, seed % 6);
            zero_block(c, DATA + 0x1_8000, 16);
            c.g[A1][0] = sx(DATA + 0x1_8000);
            // A limit below the unpacked size (0x10) clamps it.
            if seed / 6 % 2 == 1 {
                c.g[A2][0] = 8;
            }
            c.pc = 0x003e_2310;
        };
        crate::difftest::check_paths(
            gen_ea_file_load_decompressed_into,
            hand_ea_file_load_decompressed_into,
            60,
            // ... and with the limit below the unpacked size: 675, 709, 974.
            &[31, 96, 672, 727, 1027, 675, 709, 974],
            setup,
        );
    }

    /// The into-buffer unpack paths with events due in the delay loop.
    #[test]
    fn ea_file_load_decompressed_into_events() {
        check_events(
            gen_ea_file_load_decompressed_into,
            hand_ea_file_load_decompressed_into,
            48,
            &|c, seed| {
                let mut r = load_decompressed_setup(c, seed, 4 + seed / 3 % 2);
                zero_block(c, DATA + 0x1_8000, 16);
                c.g[A1][0] = sx(DATA + 0x1_8000);
                schedule(c, &mut r, seed, 0x003e_2310)
            },
        );
    }

    driven!(
        gen_ea_file_load_decompressed_into_thunk,
        ee::s14::f_003e2470__generated
    );

    driven!(
        hand_ea_file_load_decompressed_into_thunk,
        ea_file_load_decompressed_into_thunk
    );

    /// Thunk forwards to the into-buffer worker.
    #[test]
    fn ea_file_load_decompressed_into_thunk_matches() {
        check(
            gen_ea_file_load_decompressed_into_thunk,
            hand_ea_file_load_decompressed_into_thunk,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2470, 0);
                ea_install_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii(c, DATA + 0x400, "pack.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_2470;
            },
        );
    }

    /// A zeroed data block.
    pub(crate) fn zero_block(c: &mut Cpu, base: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(base + 4 * i, 0);
        }
    }

    driven!(gen_ea_file_platform_init, ee::s14::f_003e3020__generated);

    driven!(hand_ea_file_platform_init, ea_file_platform_init);

    /// Cold boot skips the flag-3 write; warm boot takes it.
    #[test]
    fn ea_file_platform_init_matches() {
        check(
            gen_ea_file_platform_init,
            hand_ea_file_platform_init,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3020, 0);
                c.bus.write32(0x0045_0c48, (seed % 5) as u32);
                c.bus.write32(0x0045_0c44, 0);
                c.g[A1][0] = (seed / 5 % 2) as u64;
                c.pc = 0x003e_3020;
            },
        );
    }

    driven!(gen_ea_file_platform_write, ee::s14::f_003e3478__generated);

    driven!(hand_ea_file_platform_write, ea_file_platform_write);

    /// Host write pair; sizes drive the completion path.
    #[test]
    fn ea_file_platform_write_matches() {
        check(
            gen_ea_file_platform_write,
            hand_ea_file_platform_write,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3478, 0);
                zero_block(c, DATA + 0x500, 8);
                c.g[A0][0] = 0x1_0000 + (seed % 3) as u64;
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = [0u32, 1, 0x1000][seed as usize / 3 % 3] as u64;
                c.g[A3][0] = (seed / 9 % 2) as u64;
                c.pc = 0x003e_3478;
            },
        );
    }

    driven!(gen_ea_file_platform_get_lsn, ee::s14::f_003e3538__generated);

    driven!(hand_ea_file_platform_get_lsn, ea_file_platform_get_lsn);

    /// Host-drive bit selects the CdGetEntry arm or the zero store.
    #[test]
    fn ea_file_platform_get_lsn_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let _r = machine(c, seed, 0x3e_3538, 0);
            c.bus.write32(0x0045_0c5c, (seed % 7) as u32);
            c.bus.write32(0x0051_9c70, 0x100 + seed);
            // Drive byte (bits 24-31, an arithmetic shift): 1 is the
            // CD drive, negative bytes sign-extend.
            let drive = [0u32, 1, 0x81, 0xff][seed as usize % 4];
            c.g[A0][0] = ((drive << 24) | 0x34_5678) as u64;
            c.pc = 0x003e_3538;
        };
        crate::difftest::check_paths(
            gen_ea_file_platform_get_lsn,
            hand_ea_file_platform_get_lsn,
            48,
            &[13, 46],
            setup,
        );
    }

    driven!(gen_ea_file_alarm_handler, ee::s14::f_003e3588__generated);

    driven!(hand_ea_file_alarm_handler, ea_file_alarm_handler);

    /// Host alarm ack through the syscall fake.
    #[test]
    fn ea_file_alarm_handler_matches() {
        check(
            gen_ea_file_alarm_handler,
            hand_ea_file_alarm_handler,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3588, 0);
                c.g[A2][0] = (seed % 5) as u64;
                c.pc = 0x003e_3588;
            },
        );
    }

    driven!(gen_ea_file_delay_ticks, ee::s14::f_003e35b0__generated);

    driven!(hand_ea_file_delay_ticks, ea_file_delay_ticks);

    /// Timer-primitive wait; tick counts drive the config word.
    #[test]
    fn ea_file_delay_ticks_matches() {
        check(
            gen_ea_file_delay_ticks,
            hand_ea_file_delay_ticks,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_35b0, 0);
                c.g[A0][0] = [0u32, 1, 0x1e0, 0x1_0000][seed as usize % 4] as u64;
                c.pc = 0x003e_35b0;
            },
        );
    }

    driven!(gen_ea_file_hash_name, ee::s14::f_003e36f8__generated);

    driven!(hand_ea_file_hash_name, ea_file_hash_name);

    /// `EAFile_HashName(name, seed)`: empty, short and long names (each
    /// byte folds in from the last, `h * 33 + byte`); `events` uses
    /// 40-200 byte names so events fall due inside the fold.
    fn hash_name_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_36f8, 0);
        let names = ["", "a", "entry", "longer/name/here"];
        if events {
            let n = 40 + r.next_u32() % 160;
            let s: String = (0..n)
                .map(|i| (b'a' + (i * 7 % 26) as u8) as char)
                .collect();
            put_ascii(c, DATA + 0x400, &s);
        } else {
            put_ascii(c, DATA + 0x400, names[seed as usize % 4]);
        }
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = r.next_u32() as u64;
        if events {
            schedule(c, &mut r, seed, 0x003e_36f8)
        } else {
            c.pc = 0x003e_36f8;
            Events::Off
        }
    }

    /// Empty, short and long names (`hash_name_setup`).
    #[test]
    fn ea_file_hash_name_matches() {
        check(
            gen_ea_file_hash_name,
            hand_ea_file_hash_name,
            48,
            &|c, seed| {
                hash_name_setup(c, seed, false);
            },
        );
    }

    /// Long names with events due in the fold loop.
    #[test]
    fn ea_file_hash_name_events() {
        check_events(
            gen_ea_file_hash_name,
            hand_ea_file_hash_name,
            48,
            &|c, seed| hash_name_setup(c, seed, true),
        );
    }

    driven!(gen_ea_file_cd_signal, ee::s14::f_003e3ad8__generated);

    driven!(hand_ea_file_cd_signal, ea_file_cd_signal);

    /// Host signal ack through the syscall fake.
    #[test]
    fn ea_file_cd_signal_matches() {
        check(
            gen_ea_file_cd_signal,
            hand_ea_file_cd_signal,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3ad8, 0);
                c.bus.write32(0x0051_9c4c, seed);
                c.pc = 0x003e_3ad8;
            },
        );
    }

    driven!(gen_ea_file_resolve_path, ee::s14::f_003e3098__generated);

    driven!(hand_ea_file_resolve_path, ea_file_resolve_path);

    /// `ResolvePath(src, dst, status_out)`: `dst` = (host root when the
    /// path names the host) + (the root `0x519af0` unless the rest starts
    /// with `\`, `/` or `~`) + the rest, with `\` turned into `/`; the
    /// mode (1 cd, 2 host, else `[0x450c44]`) is stored through
    /// `status_out` when given. Paths: host, `cd:`, a drive-like `ab:`
    /// (third byte `:`), another device (early exit unless the mode word
    /// is 1, which exits after the status store), plain, `/`, `~` and `\`
    /// led. `events` uses a 150-300 byte path and delivers events (no
    /// unwinds: `strchr` is a dispatcher) in the slash loop.
    fn resolve_path_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_3098, 0);
        elf_image(c);
        let paths = [
            "host0:dir\\file",
            "cd:\\x\\y.big",
            "ab:x\\y",
            "q:file",
            "plain\\name",
            "/abs/x",
            "~home",
            "\\back\\slash",
        ];
        let path: String = if events {
            let n = 150 + r.next_u32() % 150;
            let mut p = String::from("host0:");
            p.extend((0..n).map(|i| if i % 5 == 4 { '\\' } else { 'a' }));
            p
        } else {
            paths[seed as usize % paths.len()].to_string()
        };
        put_ascii(c, DATA + 0x400, &path);
        zero_block(c, DATA + 0x800, 0x100);
        c.bus.write32(0x0045_0c44, r.next_u32() % 4);
        put_ascii(c, DATA + 0x600, "host0:");
        c.bus.write32(0x0045_0c50, DATA + 0x600);
        put_ascii(c, DATA + 0x620, "cd:");
        c.bus.write32(0x0045_0c3c, DATA + 0x620);
        let roots = ["", "root\\dir\\", "r/"];
        put_ascii(c, 0x0051_9af0, roots[r.next_u32() as usize % 3]);
        c.bus.write32(DATA + 0x700, 0xdddd_dddd);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = sx(DATA + 0x800);
        c.g[A2][0] = if r.next_u32() % 2 == 0 {
            0
        } else {
            sx(DATA + 0x700)
        };
        if events {
            let mode = schedule(c, &mut r, seed, 0x003e_3098);
            ea_deliver_only(c, mode)
        } else {
            c.pc = 0x003e_3098;
            Events::Off
        }
    }

    /// Every path kind (`resolve_path_setup`).
    #[test]
    fn ea_file_resolve_path_matches() {
        check(
            gen_ea_file_resolve_path,
            hand_ea_file_resolve_path,
            64,
            &|c, seed| {
                resolve_path_setup(c, seed, false);
            },
        );
    }

    /// A long host path with events delivered in the slash loop.
    #[test]
    fn ea_file_resolve_path_events() {
        check_events(
            gen_ea_file_resolve_path,
            hand_ea_file_resolve_path,
            48,
            &|c, seed| resolve_path_setup(c, seed, true),
        );
    }

    driven!(
        gen_ea_file_cd_build_hash_table,
        ee::s14::f_003e39a8__generated
    );

    driven!(
        hand_ea_file_cd_build_hash_table,
        ea_file_cd_build_hash_table
    );

    /// Turn an `Unwind` schedule into `Deliver` (no VBlank due): for loops
    /// followed by a dispatcher callee (`qsort`, `strchr`), where a pending
    /// unwind request would unwind that callee mid-call.
    fn ea_deliver_only(c: &mut Cpu, mode: Events) -> Events {
        if mode == Events::Unwind {
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            Events::Deliver
        } else {
            mode
        }
    }

    /// `CdBuildHashTable(src, dst, n, seed)`: hash `n` source entries
    /// (stride 0x108: name, then two words) into `dst` (stride
    /// `[0x450c5c] + 0xc`: hash, the two words, and the name when the
    /// stride is larger than 0xc), sort them, and retry with a new seed
    /// while two hashes collide. Empty and 1-4 entry tables; with `collide`
    /// the names `"\x22"` and `"\x01\x01"` hash equal under seed
    /// 0x0800_0000 (their difference is seed * 1056) and differ under the
    /// retry seed. `events` delivers events (no unwinds) in the loops.
    fn cd_build_hash_table_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_39a8, 0);
        elf_image(c);
        let collide = r.next_u32() % 3 == 0;
        let n = if events { 6 } else { r.next_u32() % 5 };
        let src = DATA + 0x700;
        zero_block(c, src, 0x108 * 6 / 4);
        zero_block(c, DATA + 0x1_0000, 0x200);
        let names = ["alpha", "betaa", "gamma!", "d", "eps.big", "z"];
        for i in 0..n {
            let name = match (collide, i) {
                (true, 0) => "\x22",
                (true, 1) => "\x01\x01",
                _ => names[i as usize],
            };
            put_ascii(c, src + i * 0x108, name);
            c.bus.write32(src + i * 0x108 + 0x100, 0x10 + i);
            c.bus.write32(src + i * 0x108 + 0x104, 0x20 + i);
        }
        c.bus
            .write32(0x0045_0c58 + 4, [0u32, 0x10][r.next_u32() as usize % 2]);
        c.g[A0][0] = sx(src);
        c.g[A1][0] = sx(DATA + 0x1_0000);
        c.g[A2][0] = n as u64;
        c.g[A3][0] = if collide {
            0x0800_0000
        } else {
            r.next_u32() as u64
        };
        if events {
            let mode = schedule(c, &mut r, seed, 0x003e_39a8);
            ea_deliver_only(c, mode)
        } else {
            c.pc = 0x003e_39a8;
            Events::Off
        }
    }

    /// Empty and small tables, name copies, and the collision retry
    /// (`cd_build_hash_table_setup`).
    #[test]
    fn ea_file_cd_build_hash_table_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_build_hash_table,
            hand_ea_file_cd_build_hash_table,
            48,
            &|c, seed| {
                cd_build_hash_table_setup(c, seed, false);
            },
        );
    }

    /// Six-entry tables with events delivered in the hash and retry loops.
    #[test]
    fn ea_file_cd_build_hash_table_events() {
        check_events(
            gen_ea_file_cd_build_hash_table,
            hand_ea_file_cd_build_hash_table,
            48,
            &|c, seed| cd_build_hash_table_setup(c, seed, true),
        );
    }

    driven!(gen_ea_file_cd_find_entry, ee::s14::f_003e4968__generated);

    driven!(hand_ea_file_cd_find_entry, ea_file_cd_find_entry);

    /// `CdFindEntry(name)`: the entry address for a name in the table
    /// (`cd_table_setup`), or 0 when absent.
    #[test]
    fn ea_file_cd_find_entry_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3e_4968, 0);
            elf_image(c);
            let names = ["entry", "/dir/file.dat", "x"];
            let name = names[seed as usize % names.len()];
            put_ascii(c, DATA + 0x400, name);
            let count = 1 + r.next_u32() % 4;
            let hit = r.next_u32() % 3 != 0;
            cd_table_setup(c, &mut r, name, count, hit);
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = 0;
            c.pc = 0x003e_4968;
        };
        crate::difftest::check_paths_lohi(
            gen_ea_file_cd_find_entry,
            hand_ea_file_cd_find_entry,
            48,
            &[217, 235],
            setup,
        );
    }

    driven!(gen_ea_file_cd_get_entry, ee::s14::f_003e48d8__generated);

    driven!(hand_ea_file_cd_get_entry, ea_file_cd_get_entry);

    /// `CdGetEntry(index, name_out, sector_out, size_out)`: each out is
    /// skipped when null (the name is `strncpy`d, `[0x450c5c]` bytes);
    /// returns the size.
    #[test]
    fn ea_file_cd_get_entry_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_get_entry,
            hand_ea_file_cd_get_entry,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_48d8, 0);
                // Three entries, stride [0x450c5c] + 0xc = 0x2c: +4 first
                // sector, +8 size, +0xc the name (0x20 bytes).
                c.bus.write32(0x0045_0c5c, 0x20);
                c.bus.write32(0x0051_9c78, DATA + 0x700);
                zero_block(c, DATA + 0x700, 0x30);
                for i in 0..3u32 {
                    let e = DATA + 0x700 + 0x2c * i;
                    c.bus.write32(e + 0x4, 0x10 + i);
                    c.bus.write32(e + 0x8, 0x40 + i);
                    put_ascii(c, e + 0xc, ["entry-one", "two", "third.big"][i as usize]);
                }
                // Out words pre-filled, so a missing store shows.
                for a in (DATA + 0x500..DATA + 0x600).step_by(4) {
                    c.bus.write32(a, 0xdddd_dddd);
                }
                c.g[A0][0] = (seed % 3 + 1) as u64;
                c.g[A1][0] = if seed / 3 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x500)
                };
                c.g[A2][0] = if seed / 6 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x580)
                };
                c.g[A3][0] = if seed / 12 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x5c0)
                };
                c.pc = 0x003e_48d8;
            },
        );
    }

    driven!(gen_ea_file_cd_lookup, ee::s14::f_003e4508__generated);

    driven!(hand_ea_file_cd_lookup, ea_file_cd_lookup);

    /// `HashName` of `s` with `seed` (fold from the last byte).
    fn ea_cd_hash(s: &[u8], seed: u32) -> u32 {
        s.iter().rev().fold(seed, |h, &b| {
            h.wrapping_mul(33).wrapping_add(b as i8 as i32 as u32)
        })
    }

    /// The CD entry table for `CdLookup`/`CdFindEntry`: `count` entries
    /// (stride 0x10: +0 the name hash, sorted unsigned) at `DATA+0x700`,
    /// one of them for `name` when `hit`. Writes the table words
    /// (`[0x519c78]` base, `[0x519c44]` count, `[0x450c5c]` stride - 0xc,
    /// `[0x519c58]` hash seed).
    fn cd_table_setup(c: &mut Cpu, r: &mut Rng, name: &str, count: u32, hit: bool) {
        let seed = r.next_u32();
        // CdLookup's normal form: a leading `/`, upper case, `\` as `/`.
        let body = name.strip_prefix('/').unwrap_or(name);
        let mut norm = vec![b'/'];
        norm.extend(body.bytes().map(|b| match b {
            b'\\' => b'/',
            b'a'..=b'z' => b - 0x20,
            b => b,
        }));
        let h = ea_cd_hash(&norm, seed);
        let mut keys: Vec<u32> = (0..count)
            .map(|_| r.next_u32() | 1)
            .filter(|&k| k != h)
            .collect();
        if hit && count > 0 {
            keys.truncate(count as usize - 1);
            keys.push(h);
        }
        keys.sort_unstable();
        zero_block(c, DATA + 0x700, 0x40);
        for (i, k) in keys.iter().enumerate() {
            c.bus.write32(DATA + 0x700 + 0x10 * i as u32, *k);
            c.bus
                .write32(DATA + 0x704 + 0x10 * i as u32, 0x100 + i as u32);
        }
        c.bus.write32(0x0051_9c78, DATA + 0x700);
        c.bus.write32(0x0051_9c44, keys.len() as u32);
        c.bus.write32(0x0045_0c5c, 4);
        c.bus.write32(0x0051_9c58, seed);
    }

    /// `CdLookup(name, _, out)`: empty, short, `/`-led and `\` names,
    /// found (index stored) or not (0 stored) in tables of 0-4 entries;
    /// `events` uses 40-120 byte names (normalize and hash loops) and a
    /// one-entry table that holds the name (the binary search, a
    /// dispatcher, then makes no back-edge check).
    fn cd_lookup_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_4508, 0);
        elf_image(c);
        let names = ["", "a", "entry", "/dir/file.dat", "dir\\sub\\x.big"];
        let name: String = if events {
            let n = 40 + r.next_u32() % 80;
            (0..n)
                .map(|i| (b'a' + (i * 5 % 26) as u8) as char)
                .collect()
        } else {
            names[seed as usize % names.len()].to_string()
        };
        put_ascii(c, DATA + 0x400, &name);
        let (count, hit) = if events {
            (1, true)
        } else {
            (r.next_u32() % 5, r.next_u32() % 3 != 0)
        };
        cd_table_setup(c, &mut r, &name, count, hit);
        c.bus.write32(DATA + 0x600, 0xdddd_dddd);
        c.g[A0][0] = sx(DATA + 0x400);
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(DATA + 0x600);
        c.g[A3][0] = 0;
        if events {
            schedule(c, &mut r, seed, 0x003e_4508)
        } else {
            c.pc = 0x003e_4508;
            Events::Off
        }
    }

    /// Found and missing names (`cd_lookup_setup`).
    #[test]
    fn ea_file_cd_lookup_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            cd_lookup_setup(c, seed, false);
        };
        crate::difftest::check_paths_lohi(
            gen_ea_file_cd_lookup,
            hand_ea_file_cd_lookup,
            60,
            &[147, 184],
            setup,
        );
    }

    /// Long names with events due in the normalize and hash loops.
    #[test]
    fn ea_file_cd_lookup_events() {
        check_events(
            gen_ea_file_cd_lookup,
            hand_ea_file_cd_lookup,
            48,
            &|c, seed| cd_lookup_setup(c, seed, true),
        );
    }

    /// A path that is exactly "/" at the top of kuseg (`0x7fff_ffff`, the
    /// scratchpad's last byte; its NUL is RAM[0] through kseg0): the
    /// pointer bump past the slash crosses the sign bit, so `A3` is the
    /// sign extension of `0x8000_0000`. Scratch registers are compared.
    #[test]
    fn ea_file_cd_lookup_slash_at_sign_bit() {
        for seed in 0..crate::difftest::seed_count(8) {
            let r = crate::difftest::compare(gen_ea_file_cd_lookup, hand_ea_file_cd_lookup, &|c| {
                cd_lookup_setup(c, seed, false);
                c.bus.write8(0x7fff_ffff, b'/');
                c.bus.write8(0, 0);
                c.g[A0][0] = sx(0x7fff_ffff);
            });
            assert!(r.hard.is_empty(), "seed {seed}: {:#?}", r.hard);
            let soft: Vec<&String> = r
                .soft
                .iter()
                .filter(|m| !m.starts_with("vu0 ctrl 1"))
                .collect();
            assert!(soft.is_empty(), "seed {seed} (soft): {soft:#?}");
        }
    }

    driven!(gen_ea_file_cd_read, ee::s14::f_003e4648__generated);

    driven!(hand_ea_file_cd_read, ea_file_cd_read);

    /// Queue a read of `count` (A3) bytes at byte `offset` (A2) of entry
    /// `lba` (A0, 1-based; entry stride 0x10, +4 first sector, +8 size)
    /// into `dest` (A1). First half: unaligned offset, short read,
    /// unaligned and aligned destination. Second half: a sector other
    /// than the cached one (`[0x519c54]`) issues the read (`0x423dc0`); the
    /// cached sector with a multi-sector pass issues it too, else the
    /// cached bytes are copied and the request completes (nothing left) or
    /// the next pass is set up (short tail, unaligned or aligned
    /// destination). Flags words carry bit 1 so the completion's clear
    /// shows.
    #[test]
    fn ea_file_cd_read_matches() {
        let setup: &dyn Fn(&mut Cpu, u32) = &|c, seed| {
            let mut r = machine(c, seed, 0x3e_4648, 0);
            ea_fixed_costs(c);
            let table = DATA + 0x700;
            // Entry stride is [0x450c5c] + 0xc.
            c.bus.write32(0x0045_0c5c, 4);
            c.bus.write32(0x0051_9c40 + 0x38, table);
            zero_block(c, table, 16);
            let lba = r.next_u32() % 3 + 1;
            let entry = table + 0x10 * (lba - 1);
            let first = 0x100 + 0x20 * lba;
            // Offset -1 and count 0x801 sit on the `-1 < offset` and
            // `remaining < 2049` boundaries.
            let offset = [0u32, 1, 0x40, 0x7ff, 0x800, 0x1234, 0xffff_ffff][(seed % 7) as usize];
            let count =
                [0u32, 0x10, 0x7ff, 0x800, 0x801, 0x1000, 0x1840, 0x4000][(seed / 7 % 8) as usize];
            let end = offset.wrapping_add(count);
            let size = end
                .wrapping_add([0u32, 0x100, 0][r.next_u32() as usize % 3])
                .wrapping_sub([0u32, 0, 0x8][r.next_u32() as usize % 3].min(end));
            c.bus.write32(entry + 4, first);
            c.bus.write32(entry + 8, size);
            // Cached sector: the one the read starts in, or another.
            let hit = r.next_u32() % 4 != 0;
            let sector = first + (offset >> 11);
            c.bus
                .write32(0x0051_9c40 + 0x14, if hit { sector } else { sector + 1 });
            // Cache buffer (copy source) and the flags word.
            c.bus.write32(0x0051_9c40 + 0x3c, DATA + 0x8000);
            fill(c, &mut r, DATA + 0x8000, 0x200);
            c.bus.write32(0x0051_9c40, (r.next_u32() & 0xf) | 2);
            c.bus.write32(0x0051_9c40 + 0xc, 0x5000 + seed);
            let dest = DATA + 0x4000 + [0u32, 4][(seed / 42 % 2) as usize];
            c.g[A0][0] = lba as u64;
            c.g[A1][0] = sx(dest);
            // Sign-extended, as a 32-bit value sits in a register (offset -1).
            c.g[A2][0] = sx(offset);
            c.g[A3][0] = count as u64;
            c.pc = 0x003e_4648;
        };
        // Totals with 3-cycle fakes: a read issued for another sector
        // after each first-half shape (79 short, 81 unaligned short, 83
        // unaligned long or unaligned destination, 85 aligned multi-sector),
        // the cached sector with a multi-sector pass (86); copy and complete
        // (105 nothing copied, 219 a short copy, 443 an aligned sector);
        // copy and set up the next pass (132 short tail, 135 unaligned
        // destination, 498 aligned); a 0x801-byte count (457, 493). Offset -1
        // (the `-1 < offset` boundary) takes the 81/83 shapes.
        crate::difftest::check_paths(
            gen_ea_file_cd_read,
            hand_ea_file_cd_read,
            240,
            &[79, 81, 83, 85, 86, 105, 219, 443, 132, 135, 498, 457, 493],
            setup,
        );
    }

    driven!(gen_ea_file_cd_remount, ee::s14::f_003e4000__generated);

    driven!(hand_ea_file_cd_remount, ea_file_cd_remount);

    /// Flag clear always takes the early exit (mount arm is replay-gated).
    #[test]
    fn ea_file_cd_remount_matches() {
        check(
            gen_ea_file_cd_remount,
            hand_ea_file_cd_remount,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4000, 0);
                c.bus.write32(0x0051_9c40, 0);
                c.pc = 0x003e_4000;
            },
        );
    }

    use crate::difftest::check_paths;

    /// The `EAFile_*` probe name: unaligned, so the real `strchr`/`strcpy`
    /// take their byte loops (7 cycles a byte, counted by hand).
    pub(crate) const BXNAME: u32 = DATA + 0x101;

    /// The `0x519ae4` request-list head the parse only walks when a big
    /// file is mounted: zeroed, so it never is.
    pub(crate) const PARSE_LIST_HEAD: u32 = 0x0051_9ae4;

    /// Fixture shared by the three `EAFile_*` probes: pipe-free or piped
    /// names at `BXNAME`, and the platform open/size answers steered by
    /// `seed` (`seed % 2`: miss vs hit).
    pub(crate) fn eafile_probe_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x31_7f38, 0);
        ea_install_fakes();
        ea_fixed_costs(c);
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        c.bus.write32(PARSE_LIST_HEAD, 0);
        if seed % 4 < 2 {
            put_ascii(c, BXNAME, "abc");
        } else {
            put_ascii(c, BXNAME, "|ab");
        }
        if seed % 2 == 0 {
            c.bus.write32(EAF_RET_POPEN, 0);
        } else {
            c.bus.write32(EAF_RET_POPEN, 1);
            c.bus.write32(EAF_RET_PSIZE, 0x1_2345);
        }
        c.bus.write32(EAF_RET_PCLOSE, 0);
        c.g[A0][0] = sx(BXNAME);
        c.g[RA][0] = SENTINEL as u64;
    }

    /// `EAFile_PathExists` generated totals over the plain/piped names and
    /// the miss/hit answers (piped names take the callee's `strncpy` branch).
    pub(crate) const PATHS_EAFILE_EXISTS: [u64; 3] = [116, 132, 136];

    /// `EAFile_PathExists`: miss vs hit through the real parse.
    #[test]
    fn ea_file_path_exists_matches() {
        check_paths(
            ee::s10::f_00317f38__generated,
            ea_file_path_exists,
            64,
            &PATHS_EAFILE_EXISTS,
            &|c, seed| {
                eafile_probe_setup(c, seed);
                c.pc = 0x0031_7f38;
            },
        );
    }

    /// `EAFile_GetPathSizeChecked` generated totals over the same shapes.
    pub(crate) const PATHS_EAFILE_CHECKED: [u64; 3] = [118, 134, 139];

    /// `EAFile_GetPathSizeChecked`: the size on hit, 0 on miss.
    #[test]
    fn ea_file_get_path_size_checked_matches() {
        check_paths(
            ee::s10::f_00317f60__generated,
            ea_file_get_path_size_checked,
            64,
            &PATHS_EAFILE_CHECKED,
            &|c, seed| {
                eafile_probe_setup(c, seed);
                c.pc = 0x0031_7f60;
            },
        );
    }

    /// `EAFile_GetPathSize` generated totals over the same shapes.
    pub(crate) const PATHS_EAFILE_SIZE: [u64; 3] = [116, 132, 136];

    /// `EAFile_GetPathSize`: the size on hit, the pre-zeroed word on miss.
    #[test]
    fn ea_file_get_path_size_matches() {
        check_paths(
            ee::s10::f_00317f98__generated,
            ea_file_get_path_size,
            64,
            &PATHS_EAFILE_SIZE,
            &|c, seed| {
                eafile_probe_setup(c, seed);
                c.pc = 0x0031_7f98;
            },
        );
    }

    /// The empty stub: one path, 2 cycles.
    #[test]
    fn ea_file_empty_stub_matches() {
        check_paths(
            ee::s10::f_00317fc0__generated,
            ea_file_empty_stub,
            64,
            &[2],
            &|c, seed| {
                let _r = machine(c, seed, 0x31_7fc0, 0);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_7fc0;
            },
        );
    }

    /// The stub wrapper: 7 cycles of frame plus the 2-cycle stub.
    #[test]
    fn ea_file_empty_stub_wrapper_matches() {
        check_paths(
            ee::s10::f_00317fc8__generated,
            ea_file_empty_stub_wrapper,
            64,
            &[9],
            &|c, seed| {
                let _r = machine(c, seed, 0x31_7fc8, 0);
                c.g[RA][0] = SENTINEL as u64;
                c.pc = 0x0031_7fc8;
            },
        );
    }

    /// Chunked-write steering: `seed` bit 0 nulls `a0` (the callback's
    /// signal-stub path), bit 1 doubles the wait passes, bit 2 overflows
    /// the 0x8000 chunk clamp.
    pub(crate) fn eafile_write_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3d_ec80, 0);
        ea_install_fakes();
        ea_install_worker_fakes();
        ea_fixed_costs(c);
        ea_fixed_results(c);
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
        c.bus.write32(EAF_WORKER, EAF_MAGIC);
        c.bus.write32(EAF_RET_SETCB, 0);
        c.bus.write32(EAF_REQ, 0x5150_0001);
        c.bus.write32(EAF_SEQ_NEXT, 0);
        for i in 0..8u32 {
            c.bus.write32(EAF_SEQ + 4 * i, 0);
        }
        c.bus
            .write32(EAF_WAIT_LEFT, if seed % 4 < 2 { 1 } else { 2 });
        c.g[A0][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x304) };
        c.g[A1][0] = sx(DATA + 0x404);
        c.g[A2][0] = sx(DATA + 0x508);
        c.g[A3][0] = if seed % 8 < 4 { 0x100 } else { 0x9000 };
        c.g[RA][0] = SENTINEL as u64;
        c.pc = 0x003d_ec80;
    }

    /// `EAFile_Write` generated totals: 102/109 (one wait pass, `a0`
    /// null/set), 112/119 (two passes), plus 1 each over the chunk clamp.
    pub(crate) const PATHS_EAFILE_WRITE: [u64; 8] = [102, 109, 112, 119, 103, 110, 113, 120];

    /// The chunked write: the transfer with its callback, wait and
    /// clamp paths, all through the shared EA fakes.
    #[test]
    fn eafile_write_matches() {
        check_paths(
            ee::s14::f_003dec80__generated,
            eafile_write,
            64,
            &PATHS_EAFILE_WRITE,
            &eafile_write_setup,
        );
    }

    use crate::game::ealib::seg_2dcae8::tests::oc697_hook_all;

    use crate::difftest::fakes::LOG_UNWIND_AT;

    // -------------------------------------------------------------------
    // Leaf and simple functions.

    /// Hash compare: less, greater and equal (with the collision counter),
    /// over the full unsigned range.
    #[test]
    fn ea_file_cd_compare_hash_matches() {
        check_paths(
            ee::s14::f_003e3968__generated,
            ea_file_cd_compare_hash,
            200,
            &[7, 9, 12],
            &|c, seed| {
                let mut r = Rng(0x003e_3968 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x003e_3968;
                let x = if seed % 4 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32()
                };
                let y = match seed % 3 {
                    0 => x,
                    1 => x.wrapping_add(1 + r.next_u32() % 4),
                    _ => x.wrapping_sub(1 + r.next_u32() % 4),
                };
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x110) as u64;
                c.bus.write32(DATA + 0x100, x);
                c.bus.write32(DATA + 0x110, y);
                c.bus.write32(EA_FILE_CD_HASH_EQUAL_COUNT, r.next_u32());
            },
        );
    }

    // -------------------------------------------------------------------
    // Gated slot fakes for the library calls of the CD functions.

    /// Gate word (in the machine's RAM): the fakes below only act when it
    /// holds [`OC697_MAGIC`]; with the gate off they run the body they
    /// replaced.
    pub(crate) const OC697_CTRL: u32 = DATA + 0x9_0000;

    pub(crate) const OC697_MAGIC: u32 = 0x6970_0697;

    /// Per-callee result scripts: `[count, next, v0...]` (0x40 bytes each).
    pub(crate) const OC697_SCRIPT: u32 = DATA + 0x9_0100;

    /// While nonzero, the fake `sceCdRead` copies a sector into its buffer:
    /// sector 0 on the first call, sector 1 on every later one.
    pub(crate) const OC697_SECT_ON: u32 = DATA + 0x9_0f00;

    pub(crate) const OC697_SECT_NEXT: u32 = DATA + 0x9_0f04;

    /// The two 0x800-byte sectors.
    pub(crate) const OC697_SECTORS: u32 = DATA + 0x2_0000;

    /// While nonzero, `SetAlarm` fails (-1).
    pub(crate) const OC697_FAIL_ALARM: u32 = DATA + 0x9_0f08;

    /// Script slot of a faked callee.
    pub(crate) fn oc697_slot(addr: u32) -> u32 {
        match addr {
            0x0040_1718 => 0,
            0x0040_2450 => 1,
            0x0040_1bf0 => 2,
            0x0040_1df8 => 3,
            0x0040_13e8 => 4,
            0x0040_2300 => 5,
            0x0042_3de0 => 6,
            0x003e_35b0 => 7,
            _ => 8,
        }
    }

    /// Give callee `addr` the results `vals` (cycled) instead of the fake's
    /// hash.
    pub(crate) fn oc697_script(c: &mut Cpu, addr: u32, vals: &[u32]) {
        let base = OC697_SCRIPT + oc697_slot(addr) * 0x40;
        c.bus.write32(base, vals.len() as u32);
        c.bus.write32(base + 4, 0);
        for (i, v) in vals.iter().enumerate() {
            c.bus.write32(base + 8 + 4 * i as u32, *v);
        }
    }

    /// The gate open, empty scripts and no sector copy; the fakes unwind at
    /// call `unwind` (`u32::MAX`: never).
    pub(crate) fn oc697_open(c: &mut Cpu, unwind: u32) {
        oc697_hook_all();
        c.bus.write32(OC697_CTRL, OC697_MAGIC);
        c.bus.write32(LOG_UNWIND_AT, unwind);
        for slot in 0..9 {
            c.bus.write32(OC697_SCRIPT + slot * 0x40, 0);
        }
        c.bus.write32(OC697_SECT_ON, 0);
        c.bus.write32(OC697_SECT_NEXT, 0);
        c.bus.write32(OC697_FAIL_ALARM, 0);
    }

    /// A test machine (fake syscalls and unknown callees) with the CD
    /// fakes open.
    pub(crate) fn oc697_machine(c: &mut Cpu, seed: u32, salt: u64, unwind: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        oc697_open(c, unwind);
        r
    }

    // -------------------------------------------------------------------
    // EAFile_CdReset

    /// Scripted drive for `EAFile_CdReset`: `Read` fails `reads` times first,
    /// `Sync(1)` reports busy once, `GetError` reports `errors` errors before
    /// success.
    pub(crate) fn oc697_reset_scripts(c: &mut Cpu, reads: u32, errors: u32) {
        let mut read = vec![0; reads as usize];
        read.push(1);
        oc697_script(c, 0x0040_1df8, &read);
        oc697_script(c, 0x0040_13e8, &[1, 0]);
        let mut err = vec![1; errors as usize];
        err.push(0);
        oc697_script(c, 0x0040_2300, &err);
    }

    /// `EAFile_CdReset`: read retries, busy syncs, drive errors (up to
    /// the ten-error budget that re-initialises the drive), and fakes that
    /// unwind in the middle.
    #[test]
    fn ea_file_cd_reset_matches() {
        covered(
            ee::s14::f_003e3618__generated,
            ea_file_cd_reset,
            200,
            &|c, seed| {
                let mut r = Rng(0x003e_3618 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let unwind = if seed % 4 == 0 {
                    r.next_u32() % 30
                } else {
                    u32::MAX
                };
                let mut g = oc697_machine(c, seed, 0x003e_3618, unwind);
                c.pc = 0x003e_3618;
                oc697_reset_scripts(c, seed % 3, (seed / 3) % 13);
                c.bus.write32(0x0045_0c60, g.next_u32());
            },
        );
    }

    // -------------------------------------------------------------------
    // EAFile_CdScanDir

    /// Write an ISO9660-style directory record at `at` and return its
    /// length: `len`, LBA at +2 and size at +10 (little endian, unaligned),
    /// flags at +0x19, name length at +0x20 and the name at +0x21.
    pub(crate) fn oc697_dir_record(
        c: &mut Cpu,
        at: u32,
        lba: u32,
        size: u32,
        flags: u8,
        name: &[u8],
    ) -> u32 {
        let len = (0x21 + name.len() as u32 + 1) & !1;
        c.bus.write8(at, len as u8);
        for i in 0..4 {
            c.bus.write8(at + 2 + i, (lba >> (8 * i)) as u8);
            c.bus.write8(at + 10 + i, (size >> (8 * i)) as u8);
        }
        c.bus.write8(at + 0x19, flags);
        c.bus.write8(at + 0x20, name.len() as u8);
        for (i, b) in name.iter().enumerate() {
            c.bus.write8(at + 0x21 + i as u32, *b);
        }
        len
    }

    /// The two sectors `sceCdRead` hands out: sector 0 lists "." then
    /// `entries` files and (when `dirs`) directories, sector 1 only files.
    /// Names end in ".;1", ";1" or a bare dot, to reach each branch of the
    /// name clean-up.
    pub(crate) fn oc697_sectors(c: &mut Cpu, r: &mut Rng, entries: u32, dirs: bool) {
        for s in 0..2u32 {
            let base = OC697_SECTORS + s * 0x800;
            let mut at = base;
            at += oc697_dir_record(c, at, 0x100, 0x800, 2, b".");
            let n = if s == 0 {
                entries
            } else {
                1 + r.next_u32() % 3
            };
            for j in 0..n {
                let dir = s == 0 && dirs && (j + r.next_u32()) % 2 == 0;
                let name: Vec<u8> = match r.next_u32() % 3 {
                    0 => b"AB.;1".to_vec(),
                    1 => b"CDE;1".to_vec(),
                    _ => b"FG.H;1".to_vec(),
                };
                let size = 0x7f0 + (r.next_u32() % 3) * 0x400;
                at += oc697_dir_record(
                    c,
                    at,
                    0x200 + 0x10 * j,
                    size,
                    if dir { 2 } else { 0 },
                    if dir { &b"SUBDIR"[..] } else { &name[..] },
                );
            }
            c.bus.write8(at, 0);
        }
    }

    /// The `ScanDir` fixture: the file table and its count, an empty path,
    /// the sector script and a quiet drive.
    pub(crate) fn oc697_scan_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        oc697_reset_scripts(c, 0, 0);
        oc697_script(c, 0x0040_1df8, &[1]);
        oc697_script(c, 0x0040_13e8, &[0]);
        c.bus.write32(OC697_SECT_ON, 1);
        c.bus.write32(0x0045_0c68, DATA + 0x6000);
        c.bus.write32(0x0045_0c6c, seed % 3);
        c.bus.write32(0x0051_9c6c, r.next_u32());
        let entries = 1 + seed % 5;
        oc697_sectors(c, r, entries, seed / 5 % 3 != 0);
        c.g[A0][0] = (0x100 + seed % 7) as u64;
        c.g[A1][0] = (1 + seed / 15 % 3) as u64;
    }

    /// `EAFile_CdScanDir`: a root directory with files and subdirectories
    /// (the recursion), one to three sectors, the sector buffer and path
    /// saved around the recursion.
    #[test]
    fn ea_file_cd_scan_dir_matches() {
        covered(
            ee::s14::f_003e3758__generated,
            ea_file_cd_scan_dir,
            120,
            &|c, seed| {
                let mut r = Rng(0x003e_3758 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                let unwind = if seed % 5 == 0 {
                    r.next_u32() % 60
                } else {
                    u32::MAX
                };
                oc697_machine(c, seed, 0x003e_3758, unwind);
                c.pc = 0x003e_3758;
                oc697_scan_setup(c, seed, &mut r);
            },
        );
    }

    // -------------------------------------------------------------------
    // EAFile_CdReadThread

    /// The CD request state: flags `0x519c40`, semaphore `+0xc`, retries
    /// `+0x10`, `+0x14`, the buffer base `+0x3c`; the current read at
    /// `0x519c60` (length, remaining, offset, sector, sector count, dest).
    pub(crate) const OC697_CD_STATE: u32 = 0x0051_9c40;

    pub(crate) const OC697_CD_READ: u32 = 0x0051_9c60;

    /// `EAFile_CdReadThread`: the thread loops forever, so each seed picks
    /// the call at which a fake unwinds. Random flags, retry counts,
    /// remaining lengths (none, a short tail, unaligned and aligned
    /// runs), drive errors, and failing re-reads.
    #[test]
    fn ea_file_cd_read_thread_matches() {
        covered(
            ee::s14::f_003e3be0__generated,
            ea_file_cd_read_thread,
            300,
            &|c, seed| {
                let mut r = Rng(0x003e_3be0 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                oc697_machine(c, seed, 0x003e_3be0, 1 + seed % 14);
                c.pc = 0x003e_3be0;
                let flags = r.next_u32() & 0xf;
                c.bus.write32(OC697_CD_STATE, flags);
                c.bus.write32(OC697_CD_STATE + 0xc, r.next_u32());
                c.bus.write32(OC697_CD_STATE + 0x10, r.next_u32() % 3);
                c.bus.write32(OC697_CD_STATE + 0x14, r.next_u32());
                c.bus.write32(OC697_CD_STATE + 0x3c, DATA + 0x6_0000);
                let len = [0, 1, 0x10, 0x7ff, 0x800][(r.next_u32() % 5) as usize];
                let remaining =
                    [0, 0x40, 0x7ff, 0x800, 0x1000, 0x1801][(r.next_u32() % 6) as usize];
                c.bus.write32(OC697_CD_READ, len);
                c.bus.write32(OC697_CD_READ + 4, remaining);
                c.bus.write32(OC697_CD_READ + 8, r.next_u32() % 0x40);
                c.bus.write32(OC697_CD_READ + 0xc, r.next_u32() % 0x1000);
                c.bus.write32(OC697_CD_READ + 0x10, r.next_u32() % 4);
                c.bus.write32(
                    OC697_CD_READ + 0x14,
                    DATA + 0x4_0000 + 0x40 * (r.next_u32() % 4),
                );
                let errors: Vec<u32> = (0..8).map(|_| (r.next_u32() % 3 == 0) as u32).collect();
                oc697_script(c, 0x0040_2300, &errors);
                let reads: Vec<u32> = (0..4).map(|_| (r.next_u32() % 2) as u32).collect();
                oc697_script(c, 0x0040_1df8, &reads);
                for w in 0..16 {
                    c.bus.write32(DATA + 0x6_0000 + 4 * w, r.next_u32());
                }
            },
        );
    }
}
