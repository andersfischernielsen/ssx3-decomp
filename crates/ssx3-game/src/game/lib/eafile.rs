//! `src/lib/eafile.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
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
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    use super::ee;
    use crate::call::Callee;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_FILE_HANDLE;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_REQUEST;
    pub(crate) use crate::game::misc::callee::EA_FILE_CD_LOOKUP;
    pub(crate) use crate::game::misc::callee::EA_FILE_CD_MOUNT_DISC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CALC_MEM_SIZE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CLOSE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_PLATFORM_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_SET;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ZERO;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MUTEX_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRLEN;

    pub(crate) const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };

    pub(crate) const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };

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

/// `int EAFile_Startup(a0, a1, a2)` (0x3dcbd8): size the file layer (see
/// [`ea_file_calc_mem_size`]), allocate it through the callback at `+4` of
/// the `0x519ab0` block, and initialise it (see [`ea_file_init`]).
/// Returns 0 when the `0x519ab0` block says it already started.
pub fn ea_file_startup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x520000;
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
            c.g[S0][0] = 0x450000;
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
            c.g[V1][0] = 0x520000;
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

/// `int EAFile_Init(a0, a1, a2, a3)` (0x3dcc88): set up the `0x519ab0`
/// file-layer block (mutex, allocator sizes, zeroed request area) and the
/// CD layer unless `0x450c60` says to skip it. Returns 1, or 0 when the
/// block says it already started.
pub fn ea_file_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = 0x520000;
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
            c.g[V1][0] = 0x450000;
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
    c.g[V0][0] = 0x450000;
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
        c.g[A2][0] = 0x520000;
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
        c.g[V0][0] = 0x520000;
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
    c.g[V0][0] = 0x520000;
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
    c.g[V0][0] = 0x520000;
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
        c.g[V1][0] = 0x520000;
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
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
            callee::EA_MUTEX_LOCK.call(c, 0x003d_d048)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
            let queued = c.g[V0][0] != 0;
            c.g[A0][0] = 0x520000;
            if queued {
                const HEAD: u32 = 0x003d_d098;
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                let empty = c.g[V1][0] == 0;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
                if !empty {
                    c.cycles += 2;
                    let at_head = c.g[V1][0] == c.g[S0][0];
                    if !at_head {
                        c.cycles += 1;
                        c.g[A1][0] = c.g[V1][0];
                        loop {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                            let end = c.g[V1][0] == 0;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                            if end {
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
                let no_prev = c.g[A1][0] == 0;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                if !no_prev {
                    c.cycles += 2;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
                } else {
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A2][0]);
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6550));
                }
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
            callee::EA_MUTEX_UNLOCK.call(c, 0x003d_d128)?;
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

/// `u32 EAFile_CancelRequest(request)` (0x3dd148): when the status bit is
/// set, mark the request slot `+4` as cancelled (1) for a matching kind
/// (3 or 0xa return 0xa); otherwise return the request word shifted down.
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
        c.g[A2][0] = 0x520000;
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
                c.g[V1][0] = 0x520000;
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
        c.g[V1][0] = 0x520000;
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
                if !aborted {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                }
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
        let no_handle = c.g[V0][0] != 0;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        if !no_handle {
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
        let no_handle = c.g[S2][0] != 0;
        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        if !no_handle {
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

/// `u32 EAFile_QueueGetSize(handle, path)` (0x3dd7e0): allocate a size
/// request for the path and run the queue; a stub signal when the queue
/// entry is missing.
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
        c.g[A1][0] = 0x520000;
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
        c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
        c.g[RA][0] = sx(0x003d_d93c);
        c.sw((c.g[A1][0] as u32).wrapping_add(0x24), c.g[V1][0]);
        callee::EA_FILE_RELEASE_REQUEST.call(c, 0x003d_d93c)?;
        c.g[RA][0] = sx(0x003d_d944);
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        callee::BIG_FILE_GET_TYPE.call(c, 0x003d_d944)?;
        let known = c.g[V0][0] != 0;
        c.g[V1][0] = 0x450000;
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
            c.g[S0][0] = 0x450000;
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
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
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
        c.g[A3][0] = 0x520000;
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_sub(0x6528));
        c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
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
        c.g[V0][0] = 0x520000;
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
            c.g[S0][0] = 0x450000;
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
                    c.g[S0][0] = 0x520000;
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
                    c.g[A2][0] = 0x450000;
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

/// `char *EAFile_GetRootPath()` (0x3ddf80): copy the current root into a
/// scratch buffer; returns the buffer.
pub fn ea_file_get_root_path(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x520000;
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
                c.g[V0][0] = 0x520000;
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
                        c.cycles += 1;
                        c.g[V0][0] = c.g[S2][0];
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

/// `void EAFile_CallWithPriority(prio, a1, fn, arg)` (0x3dddf0): stash the
/// queue's pending count, call `fn(arg)` through the table, restore the
/// count and run the queue with `prio`.
pub fn ea_file_call_with_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x520000;
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
        c.g[V0][0] = 0x520000;
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
        c.g[S2][0] = 0x450000;
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
                            c.g[V0][0] = 0x450000;
                            c.g[A2][0] = 0x3;
                            c.g[RA][0] = sx(0x003d_deb8);
                            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc08));
                            call_ptr(c, callee::STRNCMP, 0x003d_deb8)?;
                            if c.g[V0][0] != 0 {
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc50));
                                break 'b_3ddee8;
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x450000;
                            c.g[A0][0] = 0x2;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc48));
                            c.g[V1][0] = c.g[V1][0] & 0x3;
                            let keep = c.g[V1][0] == c.g[A0][0];
                            c.g[V1][0] = 0x450000;
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
                        c.g[V0][0] = 0x520000;
                        if differ {
                            break 'b_3ddf20;
                        }
                        c.cycles += 5;
                        c.g[V1][0] = 0x450000;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc48));
                        c.g[V0][0] = c.g[V0][0] & 0x2;
                        let no_cd = c.g[V0][0] == 0;
                        c.g[V1][0] = 0x450000;
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
                    c.g[V0][0] = 0x520000;
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
        c.g[V0][0] = 0x450000;
        if !fits {
            c.cycles += 3;
            c.g[RA][0] = sx(0x003d_e954);
            c.g[A1][0] = c.g[S1][0];
            callee::EA_MEM_ZERO.call(c, 0x003d_e954)?;
            c.g[V0][0] = 0x450000;
        }
        c.cycles += 23;
        c.g[A1][0] = 0x450000;
        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc58));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc58));
        c.g[A3][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc50));
        c.g[V0][0] = 0x450000;
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
        c.g[S1][0] = 0x450000;
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
        c.g[A1][0] = 0x450000;
        c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc58));
        c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x450000;
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
        c.g[V0][0] = 0x450000;
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
        c.g[V0][0] = 0x450000;
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
        c.g[V0][0] = 0x450000;
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
            c.g[V1][0] = 0x450000;
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
        c.g[V1][0] = 0x450000;
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
        c.g[V0][0] = 0x450000;
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
            c.g[V1][0] = 0x450000;
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
        c.g[V0][0] = 0x450000;
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
        if missing {
            // Fall through to the shared tail below.
        } else {
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
        c.g[V0][0] = 0x450000;
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
        if missing {
            // Fall through to the shared tail below.
        } else {
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
        c.g[V0][0] = 0x450000;
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

/// `u32 EAFile_LoadFileInto(path, ...)` (0x3e1bb8): into-buffer load
/// without the priority flag.
pub fn ea_file_load_file_into(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = c.g[A2][0];
        c.sw(c.g[SP][0] as u32, c.g[A0][0]);
        c.g[V0][0] = 0x450000;
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
            c.g[V0][0] = 0x450000;
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
            c.g[V0][0] = 0x450000;
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

/// `u32 EAFile_LoadBigHeaderWorker(path, ...)` (0x3e1d68): opens the
/// file, allocates a header buffer and reads the big-file header into
/// it; null on any failure.
pub fn ea_file_load_big_header_worker(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_F8: u32 = 0x003e_1df8;
        c.cycles += 17;
        let frame = Frame::open(c, 0x80);
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
        c.g[RA][0] = sx(0x003e_1da4);
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        callee::EA_FILE_OPEN.call(c, 0x003e_1da4)?;
        let failed = c.g[V0][0] == 0;
        c.g[V0][0] = 0x450000;
        let mut done = false;
        if failed {
            // Open failed: report null.
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.lw(c.g[S3][0] as u32);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc10));
            c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
            let alloc = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x003e_1dc4);
            c.g[A1][0] = 0xa90;
            c.pc = alloc;
            call_ptr(c, alloc, 0x003e_1dc4)?;
            c.g[S1][0] = c.g[V0][0];
            let no_mem = c.g[S1][0] == 0;
            c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
            if no_mem {
                c.cycles += 2;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                c.g[RA][0] = sx(0x003e_1e98);
                callee::EA_FILE_CLOSE.call(c, 0x003e_1e98)?;
            } else {
                c.cycles += 10;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[S4][0] = c.g[T0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003e_1de8);
                c.g[A3][0] = 0xa90;
                callee::EA_FILE_READ.call(c, 0x003e_1de8)?;
                c.g[RA][0] = sx(0x003e_1df0);
                c.g[A0][0] = c.g[S1][0];
                callee::BIG_FILE_GET_TYPE.call(c, 0x003e_1df0)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e_1e18);
                    c.g[A0][0] = c.g[S1][0];
                    callee::BIG_FILE_GET_HEADER_SIZE.call(c, 0x003e_1e18)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2705) as u64;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003e_1e88);
                        c.g[A1][0] = c.g[S4][0];
                        callee::EA_FILE_CLOSE.call(c, 0x003e_1e88)?;
                        c.g[V0][0] = c.g[S1][0];
                        done = true;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                        let alloc2 = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x003e_1e3c);
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        c.pc = alloc2;
                        call_ptr(c, alloc2, 0x003e_1e3c)?;
                        c.g[S0][0] = c.g[V0][0];
                        let no_mem2 = c.g[S0][0] == 0;
                        c.g[A1][0] = c.g[S1][0];
                        if no_mem2 {
                            if event_due(c) {
                                deliver_event_here(c, HEAD_F8);
                            }
                            // Retry from the free-callback block below.
                        } else {
                            c.cycles += 14;
                            c.g[A2][0] = 0xa90;
                            c.g[RA][0] = sx(0x003e_1e54);
                            c.g[A0][0] = c.g[S0][0];
                            callee::EA_MEM_COPY.call(c, 0x003e_1e54)?;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.g[S1][0];
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_1e64);
                            c.g[S1][0] = c.g[S0][0];
                            c.pc = free;
                            call_ptr(c, free, 0x003e_1e64)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xa90));
                            c.g[A1][0] = 0xa90;
                            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0xa90));
                            c.g[RA][0] = sx(0x003e_1e7c);
                            c.g[T0][0] = c.g[S4][0];
                            callee::EA_FILE_READ.call(c, 0x003e_1e7c)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x003e_1e88);
                            c.g[A1][0] = c.g[S4][0];
                            callee::EA_FILE_CLOSE.call(c, 0x003e_1e88)?;
                            c.g[V0][0] = c.g[S1][0];
                            done = true;
                        }
                        if !done {
                            // Free the first buffer and close out.
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                            let free = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x003e_1e04);
                            c.g[A0][0] = c.g[S1][0];
                            c.pc = free;
                            call_ptr(c, free, 0x003e_1e04)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[A1][0] = c.g[S4][0];
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x003e_1e98);
                            callee::EA_FILE_CLOSE.call(c, 0x003e_1e98)?;
                        }
                    }
                } else {
                    // Not a big file: free the buffer and close out.
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    let free = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e_1e04);
                    c.g[A0][0] = c.g[S1][0];
                    c.pc = free;
                    call_ptr(c, free, 0x003e_1e04)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.g[S4][0];
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_1e98);
                    callee::EA_FILE_CLOSE.call(c, 0x003e_1e98)?;
                }
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `u32 EAFile_LoadBigHeaderEx(path, ...)` (0x3e1ec8): header load
/// through the priority-call gate.
pub fn ea_file_load_big_header_ex(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x450000;
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
        c.g[V0][0] = 0x450000;
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
                    let hook = 0x003b_4818;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_21e0)?;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    let no_size = c.g[A1][0] == 0;
                    c.g[S4][0] = c.g[V0][0];
                    if !no_size {
                        c.cycles += 2;
                        let empty = c.g[S4][0] == 0;
                        c.g[V0][0] = 0x450000;
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
                                    let free2 = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x003e_2258);
                                    c.g[A2][0] = c.g[S5][0];
                                    c.pc = free2;
                                    call_ptr(c, free2, 0x003e_2258)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let no_buf = c.g[S0][0] == 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    if no_buf {
                                        break 'b_3e2284;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x003e_226c);
                                    c.g[A1][0] = c.g[S0][0];
                                    let hook = 0x003b_4740;
                                    c.pc = hook;
                                    call_ptr(c, hook, 0x003e_226c)?;
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
                    c.g[S5][0] = 0x450000;
                    let hook = 0x003b_4818;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_2364)?;
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
                            let hook = 0x003b_4740;
                            c.pc = hook;
                            call_ptr(c, hook, 0x003e_23a4)?;
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
            let hook = 0x003b_4818;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_2154)?;
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
        let hook = 0x003b_4818;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_2184)?;
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressedThunk(path, ...)` (0x3e22f0): tail call
/// to the worker with a fresh frame.
pub fn ea_file_load_decompressed_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2300);
        callee::EA_FILE_LOAD_DECOMPRESSED.call(c, 0x003e_2300)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_LoadDecompressedIntoThunk(path, ...)` (0x3e2470): tail
/// call to the into-buffer worker.
pub fn ea_file_load_decompressed_into_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_2480);
        callee::EA_FILE_LOAD_DECOMPRESSED_INTO.call(c, 0x003e_2480)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
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
        c.g[S1][0] = 0x450000;
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
                c.g[A0][0] = 0x450000;
                c.g[V1][0] = 0x1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0xc48), c.g[V0][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xc44), c.g[V1][0]);
                break 'b_3e3084;
            }
            c.cycles += 3;
            c.g[V1][0] = 0x450000;
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

/// `u32 EAFile_PlatformWrite(fd_gate, buf, len, gate)` (0x3e3478):
/// host write pair plus the completion callback.
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
        let hook = 0x0042_8168;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_34b0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x003e_34c0);
        c.g[A0][0] = c.g[S0][0];
        let hook = 0x0042_8600;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_34c0)?;
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
        c.g[A1][0] = sx(((c.g[A0][0] as u32) >> 24) as u32);
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
        let hook = 0x0042_3dd0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3598)?;
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
        let hook = 0x0042_3da0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_35dc)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3e0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
        c.g[RA][0] = sx(0x003e_35f4);
        c.g[A0][0] = c.g[S1][0];
        let hook = 0x0042_3b20;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_35f4)?;
        c.g[RA][0] = sx(0x003e_35fc);
        c.g[A0][0] = c.g[S0][0];
        let hook = 0x0042_3de0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_35fc)?;
        c.g[RA][0] = sx(0x003e_3604);
        c.g[A0][0] = c.g[S0][0];
        let hook = 0x0042_3db0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3604)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAFile_HashName(seed, path)` (0x3e36f8): trailing-slash
/// tolerant `h = h*33 + c` fold over the name.
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
        let hook = 0x0041_6810;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3714)?;
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
        c.g[V0][0] = 0x520000;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x63b4));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_3af0);
        let hook = 0x0042_3dd0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_3af0)?;
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
        c.g[S5][0] = 0x450000;
        frame.save(c, S2, 0x50);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S4, 0x30);
        c.g[A1][0] = 0x3a;
        frame.save_ra(c, 0);
        c.sb(c.g[S0][0] as u32, 0);
        c.g[RA][0] = sx(0x003e_30dc);
        c.g[S3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc44));
        let hook = 0x0041_6404;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_30dc)?;
        c.g[S4][0] = 0x450000;
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
                        let hook = 0x0041_6810;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_30ec)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V0][0] = 0x450000;
                        c.g[A2][0] = 0x3;
                        c.g[RA][0] = sx(0x003e_3104);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc3c));
                        let hook = 0x0041_6b18;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_3104)?;
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
                        let hook = 0x0041_6b18;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_3124)?;
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
                        c.g[V0][0] = 0x450000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x003e_316c);
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc50));
                        let hook = 0x0041_62d0;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_316c)?;
                        c.g[V1][0] = c.lb(c.g[S1][0] as u32);
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
                    c.g[A1][0] = 0x520000;
                    if is_tl {
                        break 'b_3e3198;
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e_3198);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6510));
                    let hook = 0x0041_62d0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_3198)?;
                    break 'b_3e3198;
                }
                'b_3e31d8: {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e_31a4);
                    c.g[A1][0] = c.g[S1][0];
                    let hook = 0x0041_62d0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_31a4)?;
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

/// `u32 EAFile_CdBuildHashTable(dst, n, src, seed)` (0x3e39a8):
/// hash every name, then sort the table.
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
        c.g[S7][0] = 0x520000;
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
                c.g[V0][0] = 0x450000;
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
                        let hook = 0x0041_66f4;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_3a3c)?;
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
                    c.g[V1][0] = 0x450000;
                    c.g[V0][0] = 0x3e0000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc58));
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3968));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S6][0];
                    c.sw((c.g[S7][0] as u32).wrapping_sub(0x1280), 0);
                    c.g[A1][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x003e_3a7c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc));
                    let hook = 0x0041_8ef8;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_3a7c)?;
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
        c.g[A1][0] = 0x520000;
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x63c0));
        c.g[V1][0] = 0x6;
        c.g[V0][0] = c.g[A0][0] & 0x6;
        let gone = c.g[V0][0] != c.g[V1][0];
        frame.save_ra(c, 0);
        'b_3e402c: {
            if gone {
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
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = 0;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `u32 EAFile_CdGetEntry(index, name_out, lba_out, size_out)`
/// (0x3e48d8): table entry fields for a 1-based index.
pub fn ea_file_cd_get_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = 0x520000;
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
            let hook = 0x0041_6ce0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_4928)?;
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

/// `u32 EAFile_CdFindEntry(index)` (0x3e4968): entry address through
/// the index lookup, or 0 when absent.
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
        c.g[A0][0] = 0x450000;
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
            c.g[A1][0] = 0x520000;
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

/// `u32 EAFile_CdRead(lba, count, ...)` (0x3e4648): queue a CD read.
/// The `0x423dc0` gate is a syscall (faked in tests); the rest is
/// same-module calls.
pub fn ea_file_cd_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        c.g[T0][0] = 0x450000;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc5c));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
        frame.save(c, S0, 0x40);
        c.g[V0][0] = 0x520000;
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
                    if c.g[V0][0] == 0 {
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
                                if c.g[A1][0] == 0 {
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
                                if c.g[V0][0] == 0 {
                                    break 'b_3e4840;
                                }
                                c.cycles += 7;
                                c.g[V1][0] = 0x1;
                                c.g[A0][0] = 0x800;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                c.g[V0][0] = c.g[V0][0] | 0x1;
                                c.sw(c.g[S0][0] as u32, c.g[A0][0]);
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
                    c.g[V1][0] = 0xffffffffffffffff;
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
            let hook = 0x0042_3dc0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_48b8)?;
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

/// `u32 EAFile_CdLookup(index, out0, out1, out2)` (0x3e4508): entry
/// address from a 1-based index into the sorted table.
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
        c.g[A3][0] = sx(c.g[A3][0] as u32).wrapping_add(0x1);
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
            c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x637f));
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
                c.g[V0][0] = 0x520000;
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x63c0));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = 0x450000;
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
                let hook = 0x0041_87e8;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_45ec)?;
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

    /// Already-started (0) and fresh (1, without the CD init: the CD
    /// path spins on Sony hardware calls, so it stays unreached at unit
    /// level and is covered by the replay gates instead).
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
                c.g[A0][0] = 0x10;
                c.g[A1][0] = 0x20;
                c.g[A2][0] = 0x30;
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
                c.bus.write32(0x0051_9ad8, DATA + 0x800);
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x800 + 8 * i, 0);
                    c.bus.write32(DATA + 0x800 + 8 * i + 4, 0);
                }
                let mode = seed % 4;
                if mode == 0 {
                    c.g[A0][0] = 0;
                } else {
                    // Byte 3 selects the table slot; the word there
                    // decides the status bit.
                    let i = seed % 8;
                    c.bus.write32(DATA + 0x800 + 8 * i, 0x100 << (i % 3));
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

    /// Gated fakes for the queue tests: the allocator and the service
    /// queue answer from fixtures; anywhere else they fall back to the
    /// installed bodies.
    pub(crate) const EAF_CTRL: u32 = DATA + 0x2_0000;

    pub(crate) const EAF_MAGIC: u32 = 0xeaf0_0001;

    pub(crate) const EAF_REQ: u32 = DATA + 0x2000;

    pub(crate) const EAF_HANDLE: u32 = DATA + 0x3000;

    /// Fixed-result variants for the queue tests: the allocator answers
    /// with the request block, the handle allocator with the handle
    /// block, and service-queue completion echoes its argument. (The
    /// `ea_fake_ret!` twins below answer from the EAF_RET_* cells for
    /// tests that steer results per seed.)
    pub(crate) fn ea_fake_alloc_request_fixed(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_alloc_request_fixed),
                ee::s14::f_003de670__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1000, ra);
        c.g[V0][0] = EAF_REQ as u64;
    }

    pub(crate) fn ea_fake_alloc_file_handle_fixed(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_alloc_file_handle_fixed),
                ee::s14::f_003de800__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1001, ra);
        c.g[V0][0] = EAF_HANDLE as u64;
    }

    pub(crate) fn ea_fake_service_queue_fixed(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_service_queue_fixed),
                ee::s14::f_003ddfa8__generated,
            );
        }
        let req = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1002, ra);
        c.g[V0][0] = req;
    }

    pub(crate) fn ea_fake_yield(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_yield),
                ee::s14::f_003e5398__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1003, ra);
        c.bus.write32(DATA + 0x1008, 1);
        c.g[V0][0] = 0;
    }

    /// Fixture-word returns for the small query callees.
    pub(crate) const EAF_RET_TYPE: u32 = DATA + 0x2_0010;

    pub(crate) const EAF_RET_HDRSIZE: u32 = DATA + 0x2_0014;

    pub(crate) const EAF_RET_FIND: u32 = DATA + 0x2_0018;

    pub(crate) const EAF_RET_POPEN: u32 = DATA + 0x2_001c;

    pub(crate) const EAF_RET_PSIZE: u32 = DATA + 0x2_0020;

    macro_rules! ea_fake_ret {
        ($name:ident, $slot:expr, $gen:expr, $cell:expr, $id:expr) => {
            pub(crate) fn $name(c: &mut Cpu) {
                if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $id, ra);
                c.g[V0][0] = c.bus.read32($cell) as u64;
            }
        };
    }

    ea_fake_ret!(
        ea_fake_release_request,
        0,
        ee::s14::f_003dd310__generated,
        EAF_RET_TYPE,
        0xeaf0_1004
    );

    ea_fake_ret!(
        ea_fake_queue_open,
        0,
        ee::s14::f_003dd4e8__generated,
        EAF_RET_TYPE,
        0xeaf0_1005
    );

    ea_fake_ret!(
        ea_fake_queue_close,
        0,
        ee::s14::f_003dd5a0__generated,
        EAF_RET_TYPE,
        0xeaf0_1006
    );

    ea_fake_ret!(
        ea_fake_queue_read,
        0,
        ee::s14::f_003dd648__generated,
        EAF_RET_TYPE,
        0xeaf0_1007
    );

    ea_fake_ret!(
        ea_fake_big_get_type,
        0,
        ee::s14::f_003e2500__generated,
        EAF_RET_TYPE,
        0xeaf0_1008
    );

    ea_fake_ret!(
        ea_fake_big_get_header_size,
        0,
        ee::s14::f_003e2580__generated,
        EAF_RET_HDRSIZE,
        0xeaf0_1009
    );

    ea_fake_ret!(
        ea_fake_big_find_entry,
        0,
        ee::s14::f_003e2768__generated,
        EAF_RET_FIND,
        0xeaf0_100a
    );

    ea_fake_ret!(
        ea_fake_platform_open,
        0,
        ee::s14::f_003e3208__generated,
        EAF_RET_POPEN,
        0xeaf0_100b
    );

    ea_fake_ret!(
        ea_fake_platform_get_size,
        0,
        ee::s14::f_003e34e8__generated,
        EAF_RET_PSIZE,
        0xeaf0_100c
    );

    ea_fake_ret!(
        ea_fake_platform_close,
        0,
        ee::s14::f_003e32f8__generated,
        EAF_RET_TYPE,
        0xeaf0_100d
    );

    pub(crate) fn ea_install_queue_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            ea_fake_alloc_request_fixed,
            fakes::prev_cell(ea_fake_alloc_request_fixed),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE800),
            ea_fake_alloc_file_handle_fixed,
            fakes::prev_cell(ea_fake_alloc_file_handle_fixed),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            ea_fake_service_queue_fixed,
            fakes::prev_cell(ea_fake_service_queue_fixed),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5398),
            ea_fake_yield,
            fakes::prev_cell(ea_fake_yield),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD310),
            ea_fake_release_request,
            fakes::prev_cell(ea_fake_release_request),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            ea_fake_queue_open,
            fakes::prev_cell(ea_fake_queue_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD5A0),
            ea_fake_queue_close,
            fakes::prev_cell(ea_fake_queue_close),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD648),
            ea_fake_queue_read,
            fakes::prev_cell(ea_fake_queue_read),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E2500),
            ea_fake_big_get_type,
            fakes::prev_cell(ea_fake_big_get_type),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E2580),
            ea_fake_big_get_header_size,
            fakes::prev_cell(ea_fake_big_get_header_size),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E2768),
            ea_fake_big_find_entry,
            fakes::prev_cell(ea_fake_big_find_entry),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E3208),
            ea_fake_platform_open,
            fakes::prev_cell(ea_fake_platform_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E34E8),
            ea_fake_platform_get_size,
            fakes::prev_cell(ea_fake_platform_get_size),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E32F8),
            ea_fake_platform_close,
            fakes::prev_cell(ea_fake_platform_close),
        );
    }

    /// A zeroed request block plus the table entry selecting it.
    pub(crate) fn req_fixture(c: &mut Cpu) {
        for i in 0..12u32 {
            c.bus.write32(EAF_REQ + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
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

    /// Null, status-clear, and set requests; the set ones with short and
    /// long queues, matching/missing heads, busy and idle entries.
    #[test]
    fn ea_file_set_request_priority_matches() {
        check(
            gen_ea_file_set_request_priority,
            hand_ea_file_set_request_priority,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_cfc0, 0);
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
                    return;
                }
                // Status word decides clear vs set.
                let status = if mode == 1 { 0 } else { 0x34_0000 };
                c.bus.write32(entry + 8, status);
                // Queue: count 0..3, head present or not.
                let count = seed / 6 % 4;
                let node = DATA + 0x2000;
                let node2 = DATA + 0x2040;
                for i in 0..12u32 {
                    c.bus.write32(node + 4 * i, 0);
                    c.bus.write32(node2 + 4 * i, 0);
                }
                let head = if count >= 2 && seed / 24 % 2 == 0 {
                    node
                } else {
                    0
                };
                queue_setup(c, &mut r, count, head);
                // The request: priority seed%3, busy flag, next link.
                c.bus.write32(entry + 0x10, seed % 3);
                c.bus.write32(entry + 0x8 + 0, status);
                c.bus
                    .write32(entry + 0x2c, if seed / 48 % 2 == 0 { node } else { 0 });
                c.bus.write32(node + 0x10, (seed / 96 % 3) as u32);
                c.bus.write32(node + 0x2c, node2);
                c.bus.write32(node + 0x8, (seed / 288 % 2) as u32);
                c.bus.write32(node2 + 0x10, (seed / 576 % 3) as u32);
                c.bus.write32(node2 + 0x2c, 0);
                c.g[A0][0] = 0x34_0000;
                c.g[A1][0] = seed as u64;
                c.pc = 0x003d_cfc0;
            },
        );
    }

    /// Null (0), status-clear (the shifted handle), and set requests of
    /// kinds 3, 0xa and other (slot store).
    #[test]
    fn ea_file_cancel_request_matches() {
        check(
            ee::s14::f_003dd148__generated,
            ea_file_cancel_request,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d148, 0);
                table_setup(c);
                let mode = seed % 5;
                if mode == 0 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = seed as u64;
                    c.pc = 0x003d_d148;
                    return;
                }
                let entry = DATA + 0x1000;
                for i in 0..12u32 {
                    c.bus.write32(entry + 4 * i, 0);
                }
                // Low nibble of byte 2 is the kind; byte 3 selects entry 0.
                // Mode 1 clears the bit with a matching word.
                let kind = [0u32, 3, 3, 0xa, 7][mode as usize];
                let handle = (kind << 16) as u64;
                // Clear iff (word ^ handle) & 0xfffff == 0.
                let word = if mode == 1 {
                    handle as u32 & 0xf_ffff
                } else {
                    0
                };
                c.bus.write32(entry + 8, word);
                c.g[A0][0] = handle;
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
                ea_install_queue_fakes();
                table_setup(c);
                let mode = seed % 4;
                let entry = DATA + 0x1000;
                for i in 0..12u32 {
                    c.bus.write32(entry + 4 * i, 0);
                }
                c.bus.write32(0x0051_9ab0 + 0x28, DATA + 0x1000);
                if mode == 0 {
                    c.g[A0][0] = 0;
                } else {
                    let handle = 0x33_0000u64;
                    // Status bit from entry+0; the wait word is entry+8.
                    // Bit set iff the word differs in the low 20 bits.
                    let word = if mode == 1 {
                        handle as u32 & 0xf_ffff
                    } else {
                        0
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
                ea_install_queue_fakes();
                req_fixture(c);
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
        check(
            gen_ea_file_queue_write,
            hand_ea_file_queue_write,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d720, 0);
                ea_install_queue_fakes();
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
            },
        );
    }

    driven!(gen_ea_file_queue_get_size, ee::s14::f_003dd7e0__generated);

    driven!(hand_ea_file_queue_get_size, ea_file_queue_get_size);

    /// Missing and present entries.
    #[test]
    fn ea_file_queue_get_size_matches() {
        check(
            gen_ea_file_queue_get_size,
            hand_ea_file_queue_get_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d7e0, 0);
                ea_install_queue_fakes();
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
            },
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
                ea_install_queue_fakes();
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
    /// the header size.
    #[test]
    fn ea_file_on_big_header_read_matches() {
        check(
            gen_ea_file_on_big_header_read,
            hand_ea_file_on_big_header_read,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_d8e8, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                // A1: block whose +0x10 selects the handle; entry words.
                c.bus.write32(DATA + 0x800, EAF_REQ as u32 + 0x20);
                c.bus.write32(DATA + 0x810, EAF_HANDLE as u32);
                c.bus.write32(DATA + 0x804, seed % 0x10);
                c.bus.write32(EAF_REQ + 0x4, (seed / 16 % 0x20) as u32);
                c.bus.write32(EAF_REQ + 0x8, EAF_HANDLE as u32);
                c.bus.write32(EAF_REQ + 0xc, 0);
                // Allocator free callback (unknown-type path).
                c.bus.write32(0x0045_0c18, 0x00c0_0000);
                c.bus.write32(EAF_RET_TYPE, (seed / 32 % 2) as u32);
                c.bus.write32(EAF_RET_HDRSIZE, (seed / 64 % 0x30) as u32);
                c.g[A0][0] = EAF_REQ as u64;
                c.g[A1][0] = sx(DATA + 0x800);
                c.g[A2][0] = sx(DATA + 0x900);
                c.pc = 0x003d_d8e8;
            },
        );
    }

    driven!(gen_ea_file_on_big_opened, ee::s14::f_003dda10__generated);

    driven!(hand_ea_file_on_big_opened, ea_file_on_big_opened);

    /// Header fetch (s0 == 1) and ready-mark paths.
    #[test]
    fn ea_file_on_big_opened_matches() {
        check(
            gen_ea_file_on_big_opened,
            hand_ea_file_on_big_opened,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_da10, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus.write32(DATA + 0x800, EAF_REQ as u32);
                c.bus.write32(DATA + 0x810, EAF_HANDLE as u32);
                c.bus.write32(EAF_RET_TYPE, EAF_REQ as u32);
                c.g[A0][0] = if seed % 3 == 0 { 0 } else { EAF_REQ as u64 };
                c.g[A1][0] = if seed % 2 == 0 { 1 } else { 2 };
                c.g[A2][0] = sx(DATA + 0x800);
                c.pc = 0x003d_da10;
            },
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
                ea_install_queue_fakes();
                req_fixture(c);
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                let mode = seed % 3;
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
                c.bus.write32(EAF_RET_TYPE, EAF_REQ as u32);
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
                ea_install_queue_fakes();
                req_fixture(c);
                c.bus.write32(EAF_RET_TYPE, 0);
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

    driven!(hand_ea_file_queue_unmount_big, ea_file_queue_unmount_big);

    /// Empty and populated mount lists, matching and missing handles.
    #[test]
    fn ea_file_queue_unmount_big_events() {
        check_events(
            gen_ea_file_queue_unmount_big,
            hand_ea_file_queue_unmount_big,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_dc30, 0);
                ea_install_queue_fakes();
                req_fixture(c);
                let mode = seed % 4;
                // Mounted archives at DATA+0x900: +0xc owner word,
                // +0x14 next; the loop compares owner +0xc to the handle.
                let handles = [0x7000u32, 0x8000, 0x9000];
                let mut prev = 0u32;
                if mode != 0 {
                    for (k, &h) in handles.iter().enumerate() {
                        let node = DATA + 0x900 + 64 * k as u32;
                        c.bus.write32(node + 0xc, h);
                        c.bus.write32(node + 0x14, prev);
                        c.bus.write32(node, 0x6000 + k as u32);
                        prev = node;
                    }
                }
                c.bus.write32(0x0051_9ae4, prev);
                let handle = [0u32, 0x8000, 0x1234, 0x9000][mode as usize];
                for i in 0..12u32 {
                    c.bus.write32(EAF_HANDLE + 4 * i, 0);
                }
                c.bus.write32(EAF_RET_TYPE, EAF_REQ as u32);
                c.g[A0][0] = handle as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                put_ascii(c, DATA + 0x400, "arch.big");
                c.g[A2][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_dc30)
            },
        );
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
                ea_install_queue_fakes();
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
                ea_install_queue_fakes();
                c.bus.write32(0x0051_9ab0 + 0x8, 0);
                c.g[A0][0] = seed as u64;
                schedule(c, &mut r, seed, 0x003d_de50)
            },
        );
    }

    driven!(gen_ea_file_set_root_path, ee::s14::f_003dde78__generated);

    driven!(hand_ea_file_set_root_path, ea_file_set_root_path);

    /// Same, shorter, and different roots; cd and non-cd modes.
    #[test]
    fn ea_file_set_root_path_matches() {
        check(
            gen_ea_file_set_root_path,
            hand_ea_file_set_root_path,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_de78, 0);
                elf_image(c);
                let roots = ["", "host:", "host:dir", "cdrom:", "other:"];
                let root = roots[seed as usize % roots.len()];
                put_ascii(c, DATA + 0x400, root);
                // Current root buffer + mode flags.
                for i in 0..8u32 {
                    c.bus.write8(0x0051_9ef0 + i, 0);
                }
                put_ascii(
                    c,
                    0x0051_9ef0,
                    ["", "host:", "cdrom:"][seed as usize / 5 % 3],
                );
                c.bus.write32(0x0045_0c50, 0x3000);
                c.bus.write32(0x0045_0c08, 0x495d40);
                put_ascii(c, 0x0049_5d40, "host:");
                c.bus
                    .write32(0x0045_0c48, [0x2, 0x0, 0x3][seed as usize / 15 % 3]);
                c.bus.write32(0x0045_0c44, 0);
                c.g[A0][0] = sx(DATA + 0x400);
                c.pc = 0x003d_de78;
            },
        );
    }

    driven!(gen_ea_file_get_root_path, ee::s14::f_003ddf80__generated);

    driven!(hand_ea_file_get_root_path, ea_file_get_root_path);

    /// Copies the current root (empty and short).
    #[test]
    fn ea_file_get_root_path_events() {
        check_events(
            gen_ea_file_get_root_path,
            hand_ea_file_get_root_path,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_df80, 0);
                for i in 0..8u32 {
                    c.bus.write8(0x0051_9ef0 + i, 0);
                }
                if seed % 2 == 0 {
                    put_ascii(c, 0x0051_9ef0, "host:");
                }
                schedule(c, &mut r, seed, 0x003d_df80)
            },
        );
    }

    driven!(gen_ea_file_parse_big_path, ee::s14::f_003de4d0__generated);

    driven!(hand_ea_file_parse_big_path, ea_file_parse_big_path);

    /// Plain paths, `big|name` paths, direct and archive modes.
    #[test]
    fn ea_file_parse_big_path_matches() {
        check(
            gen_ea_file_parse_big_path,
            hand_ea_file_parse_big_path,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_de4d0, 0);
                elf_image(c);
                ea_install_queue_fakes();
                let paths = ["file.dat", "big|name", "|name", "big|", "a|b|c", ""];
                let path = paths[seed as usize % paths.len()];
                put_ascii(c, DATA + 0x400, path);
                c.bus.write32(
                    EAF_RET_POPEN,
                    if seed / 6 % 3 == 0 { 0 } else { DATA + 0x800 },
                );
                c.bus.write32(EAF_RET_PSIZE, 0x1234 + seed);
                c.bus.write32(EAF_RET_FIND, (seed / 18 % 2) as u32);
                // BigFindEntry's out word on the stack: pre-fill so a
                // missing store shows (sp is fixed per test machine).
                c.bus.write32(DATA + 0x5000, 0xbeef_beef);
                // Mounted list for the entry scan.
                let mut prev = 0u32;
                if seed / 36 % 2 == 0 {
                    for k in 0..2u32 {
                        let node = DATA + 0x900 + 64 * k;
                        put_ascii(c, DATA + 0xb00 + 0x40 * k, "arch.big");
                        c.bus.write32(node + 0xc, DATA + 0xb00 + 0x40 * k);
                        c.bus.write32(node + 0x14, prev);
                        prev = node;
                    }
                }
                c.bus.write32(0x0051_9ae4, prev);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = (seed / 72 % 2) as u64;
                c.g[A2][0] = (seed / 144 % 3) as u64;
                c.g[A3][0] = sx(DATA + 0x5000);
                c.pc = 0x003d_de4d0;
            },
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
                let _r = machine(c, seed, 0x3d_de68, 0);
                c.pc = 0x003d_de68;
            },
        );
    }

    pub(crate) const EAF_RET_SETCB: u32 = DATA + 0x2_0024;

    pub(crate) fn ea_fake_mutex_lock(c: &mut Cpu) {
        // Gated like the other fakes: without magic the real lock runs
        // (its wait is what delivers events for no-magic tests).
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_mutex_lock),
                ee::s14::f_003e5700__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1010, ra);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_fake_mutex_unlock(c: &mut Cpu) {
        // Gated: see `ea_fake_mutex_lock`.
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_mutex_unlock),
                ee::s14::f_003e5760__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1011, ra);
        c.g[V0][0] = 0;
    }

    ea_fake_ret!(
        ea_fake_set_request_callback,
        0,
        ee::s14::f_003dcf70__generated,
        EAF_RET_SETCB,
        0xeaf0_100e
    );

    pub(crate) fn ea_install_queue_fakes_2() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            ea_fake_alloc_request_fixed,
            fakes::prev_cell(ea_fake_alloc_request_fixed),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE800),
            ea_fake_alloc_file_handle_fixed,
            fakes::prev_cell(ea_fake_alloc_file_handle_fixed),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            ea_fake_service_queue_fixed,
            fakes::prev_cell(ea_fake_service_queue_fixed),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5398),
            ea_fake_yield,
            fakes::prev_cell(ea_fake_yield),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD310),
            ea_fake_release_request,
            fakes::prev_cell(ea_fake_release_request),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            ea_fake_queue_open,
            fakes::prev_cell(ea_fake_queue_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD5A0),
            ea_fake_queue_close,
            fakes::prev_cell(ea_fake_queue_close),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD648),
            ea_fake_queue_read,
            fakes::prev_cell(ea_fake_queue_read),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
            ea_fake_set_request_callback,
            fakes::prev_cell(ea_fake_set_request_callback),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            ea_fake_mutex_lock,
            fakes::prev_cell(ea_fake_mutex_lock),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            ea_fake_mutex_unlock,
            fakes::prev_cell(ea_fake_mutex_unlock),
        );
    }

    driven!(gen_ea_file_get_settings, ee::s14::f_003de928__generated);

    driven!(hand_ea_file_get_settings, ea_file_get_settings);

    /// Counts below and above the zero-fill threshold.
    #[test]
    fn ea_file_get_settings_matches() {
        check(
            gen_ea_file_get_settings,
            hand_ea_file_get_settings,
            48,
            &|c, seed| {
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
            },
        );
    }

    driven!(gen_ea_file_set_settings, ee::s14::f_003de9b8__generated);

    driven!(hand_ea_file_set_settings, ea_file_set_settings);

    /// Counts below and above the zero-fill threshold, varied words.
    #[test]
    fn ea_file_set_settings_matches() {
        check(
            gen_ea_file_set_settings,
            hand_ea_file_set_settings,
            48,
            &|c, seed| {
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
            },
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
        check(
            gen_ea_file_on_chunk_transferred,
            hand_ea_file_on_chunk_transferred,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ea48, 0);
                ea_install_queue_fakes_2();
                req_fixture(c);
                // Request block fields the chunk math walks.
                c.bus.write32(EAF_REQ + 0x4, 0x100 + seed);
                c.bus.write32(EAF_REQ + 0x8, 0x200 + seed);
                c.bus.write32(EAF_REQ + 0x10, 0x300 + seed);
                c.bus.write32(EAF_REQ + 0x18, 0x400 + seed);
                c.bus
                    .write32(EAF_REQ + 0x14, [0u32, 0x10, 0x1000][seed as usize / 2 % 3]);
                c.bus
                    .write32(EAF_REQ + 0xc, [0u32, 0x20, 0x9000][seed as usize / 6 % 3]);
                c.bus.write32(
                    EAF_REQ + 0x1c,
                    if seed / 18 % 2 == 0 { 0 } else { 0x003d_d878 },
                );
                c.bus.write32(EAF_RET_TYPE, (seed / 36 % 2) as u32);
                c.bus.write32(EAF_RET_SETCB, (seed / 72 % 2) as u32);
                c.g[A0][0] = (seed / 144 % 3) as u64;
                c.g[A1][0] = if seed % 2 == 0 { 1 } else { 0 };
                c.g[A2][0] = EAF_REQ as u64;
                c.pc = 0x003d_ea48;
            },
        );
    }

    driven!(gen_ea_file_transfer_chunked, ee::s14::f_003deb50__generated);

    driven!(hand_ea_file_transfer_chunked, ea_file_transfer_chunked);

    /// Clamp on and off; null chunk callback (a live callback would spin
    /// the wait loop on fixture data, so the replay gates cover it).
    #[test]
    fn ea_file_transfer_chunked_matches() {
        check(
            gen_ea_file_transfer_chunked,
            hand_ea_file_transfer_chunked,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_eb50, 0);
                ea_install_queue_fakes_2();
                req_fixture(c);
                c.bus.write32(EAF_RET_TYPE, (seed % 2) as u32);
                c.bus.write32(EAF_RET_SETCB, (seed / 2 % 2) as u32);
                c.g[A0][0] = 0x1000 + seed as u64;
                c.g[A1][0] = 0x2000 + seed as u64;
                c.g[A2][0] = 0x3000 + seed as u64;
                c.g[A3][0] = [0u32, 0x9000][seed as usize / 4 % 2] as u64;
                c.g[T0][0] = 0x4000 + seed as u64;
                // Chunk callback: an empty-pool pop returns 0, taking the
                // null-callback exit (a live callback would spin the wait
                // loop on fixture data; the replay gates cover it).
                c.bus.write32(DATA + 0x700, 0);
                c.g[A0][0] = sx(DATA + 0x700);
                c.g[T1][0] = 0x003d_eed8;
                c.pc = 0x003d_eb50;
            },
        );
    }

    driven!(gen_ea_file_open, ee::s14::f_003debf0__generated);

    driven!(hand_ea_file_open, ea_file_open);

    /// Null and live queue results; varied status words.
    #[test]
    fn ea_file_open_matches() {
        check(gen_ea_file_open, hand_ea_file_open, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_ebf0, 0);
            ea_install_queue_fakes_2();
            req_fixture(c);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus.write32(EAF_REQ + 0x8, (seed % 3) as u32);
            c.bus
                .write32(EAF_RET_TYPE, if seed / 3 % 2 == 0 { 0 } else { EAF_REQ });
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = (seed / 6 % 4) as u64;
            c.g[A2][0] = 0;
            c.g[A3][0] = sx(DATA + 0x500);
            c.pc = 0x003d_ebf0;
        });
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
            ea_install_queue_fakes_2();
            req_fixture(c);
            c.bus.write32(EAF_RET_TYPE, 0);
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

    // NOTE: no `EAFile_Write` test or OVERRIDES row. Write hard-codes its
    // chunk callback to the real queue writer (no hook slot), which always
    // returns nonzero on fixture (it stores then reloads its own masked
    // result word): TransferChunked's wait loop then spins forever on both
    // sides (identical behavior, verified bit-equal in a debug run), so no
    // bounded fixture exists. The port below is a straight-line wrapper
    // identical in shape to the passing Read port; the replay gates cover it.

    driven!(gen_ea_file_close, ee::s14::f_003deca0__generated);

    driven!(hand_ea_file_close, ea_file_close);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_close_matches() {
        check(gen_ea_file_close, hand_ea_file_close, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_eca0, 0);
            ea_install_queue_fakes_2();
            req_fixture(c);
            c.bus
                .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { EAF_REQ });
            c.g[A0][0] = EAF_HANDLE as u64;
            c.pc = 0x003d_eca0;
        });
    }

    driven!(gen_ea_file_get_size, ee::s14::f_003decf8__generated);

    driven!(hand_ea_file_get_size, ea_file_get_size);

    /// Null and live queue results; the release value passes through.
    #[test]
    fn ea_file_get_size_matches() {
        check(
            gen_ea_file_get_size,
            hand_ea_file_get_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ecf8, 0);
                ea_install_queue_fakes_2();
                req_fixture(c);
                c.bus
                    .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { 0x1234 + seed });
                c.g[A0][0] = EAF_HANDLE as u64;
                c.pc = 0x003d_ecf8;
            },
        );
    }

    driven!(gen_ea_file_mount_big, ee::s14::f_003ded50__generated);

    driven!(hand_ea_file_mount_big, ea_file_mount_big);

    /// Null and live queue results; varied status words.
    #[test]
    fn ea_file_mount_big_matches() {
        check(
            gen_ea_file_mount_big,
            hand_ea_file_mount_big,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_ed50, 0);
                ea_install_queue_fakes_2();
                req_fixture(c);
                put_ascii(c, DATA + 0x400, "arch.big");
                c.bus.write32(EAF_REQ + 0x8, (seed % 3) as u32);
                c.bus
                    .write32(EAF_RET_TYPE, if seed / 3 % 2 == 0 { 0 } else { EAF_REQ });
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = (seed / 6 % 4) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = sx(DATA + 0x500);
                c.pc = 0x003d_ed50;
            },
        );
    }

    driven!(gen_ea_file_unmount_big, ee::s14::f_003dedc0__generated);

    driven!(hand_ea_file_unmount_big, ea_file_unmount_big);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_unmount_big_matches() {
        check(
            gen_ea_file_unmount_big,
            hand_ea_file_unmount_big,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_edc0, 0);
                ea_install_queue_fakes_2();
                req_fixture(c);
                c.bus
                    .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { 0x10 + seed });
                c.g[A0][0] = EAF_HANDLE as u64;
                c.pc = 0x003d_edc0;
            },
        );
    }

    driven!(gen_ea_file_exists, ee::s14::f_003dee18__generated);

    driven!(hand_ea_file_exists, ea_file_exists);

    /// Null and live queue results; zero and nonzero releases.
    #[test]
    fn ea_file_exists_matches() {
        check(gen_ea_file_exists, hand_ea_file_exists, 64, &|c, seed| {
            let _r = machine(c, seed, 0x3d_ee18, 0);
            ea_install_queue_fakes_2();
            req_fixture(c);
            put_ascii(c, DATA + 0x400, "file.dat");
            c.bus
                .write32(EAF_RET_TYPE, if seed % 3 == 0 { 0 } else { 0x20 + seed });
            c.g[A0][0] = sx(DATA + 0x400);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.pc = 0x003d_ee18;
        });
    }

    pub(crate) const EAF_RET_VALID: u32 = DATA + 0x2_002c;

    pub(crate) const EAF_RET_ALLOC: u32 = DATA + 0x2_0030;

    pub(crate) const EAF_RET_SVCQ: u32 = DATA + 0x2_003c;

    /// A zeroed stream block with selectable header fields.
    pub(crate) fn stream_block(c: &mut Cpu, base: u32) {
        for i in 0..64u32 {
            c.bus.write32(base + 4 * i, 0);
        }
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    /// Validate passes its stream through `[SP]/[SP+4]` like the real one.
    pub(crate) fn ea_fake_stream_validate(c: &mut Cpu) {
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_stream_validate),
                ee::s14::f_003dfbd0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_1012, ra);
        let v = c.bus.read32(EAF_RET_VALID);
        c.g[V0][0] = v as u64;
        if v == 0 {
            c.bus.write32(c.g[SP][0] as u32 + 4, c.g[A0][0] as u32);
            let inner = c.bus.read32(c.g[A0][0] as u32);
            c.bus.write32(c.g[SP][0] as u32, inner);
        }
    }

    ea_fake_ret!(
        ea_fake_alloc_request,
        0,
        ee::s14::f_003de670__generated,
        EAF_RET_ALLOC,
        0xeaf0_1000
    );

    ea_fake_ret!(
        ea_fake_alloc_file_handle,
        0,
        ee::s14::f_003de800__generated,
        EAF_RET_ALLOC,
        0xeaf0_1001
    );

    ea_fake_ret!(
        ea_fake_service_queue,
        0,
        ee::s14::f_003ddfa8__generated,
        EAF_RET_SVCQ,
        0xeaf0_1017
    );

    pub(crate) fn ea_install_oc1124_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DFBD0),
            ea_fake_stream_validate,
            fakes::prev_cell(ea_fake_stream_validate),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            ea_fake_alloc_request,
            fakes::prev_cell(ea_fake_alloc_request),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE800),
            ea_fake_alloc_file_handle,
            fakes::prev_cell(ea_fake_alloc_file_handle),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            ea_fake_queue_open,
            fakes::prev_cell(ea_fake_queue_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD5A0),
            ea_fake_queue_close,
            fakes::prev_cell(ea_fake_queue_close),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD648),
            ea_fake_queue_read,
            fakes::prev_cell(ea_fake_queue_read),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            ea_fake_service_queue,
            fakes::prev_cell(ea_fake_service_queue),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            ea_fake_mutex_lock,
            fakes::prev_cell(ea_fake_mutex_lock),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            ea_fake_mutex_unlock,
            fakes::prev_cell(ea_fake_mutex_unlock),
        );
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, [0u32, 1][seed as usize % 2]);
                c.bus.write32(EAF_RET_TYPE, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_RET_ALLOC, 0);
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

    /// Null and live opens; the size callback is faked to null.
    #[test]
    fn ea_file_get_file_size_worker_matches() {
        check(
            gen_ea_file_get_file_size_worker,
            hand_ea_file_get_file_size_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1728, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1728;
            },
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_SETCB, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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

    /// Null opens plus pool-allocator outcomes; reads never start.
    #[test]
    fn ea_file_load_file_worker_matches() {
        check(
            gen_ea_file_load_file_worker,
            hand_ea_file_load_file_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1810, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1810;
            },
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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

    /// Null opens plus pool-allocator outcomes; reads never start.
    #[test]
    fn ea_file_load_file_get_size_worker_matches() {
        check(
            gen_ea_file_load_file_get_size_worker,
            hand_ea_file_load_file_get_size_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1948, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1948;
            },
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
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

    /// Null opens plus short sizes; reads never start.
    #[test]
    fn ea_file_load_file_into_worker_matches() {
        check(
            gen_ea_file_load_file_into_worker,
            hand_ea_file_load_file_into_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1ad0, 0);
                ea_install_oc1124_fakes();
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
                c.bus.write32(EAF_CTRL, EAF_MAGIC);
                put_ascii(c, DATA + 0x400, "file.dat");
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1ad0;
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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
                ea_install_oc1124_fakes();
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_TYPE, 0);
                c.bus.write32(EAF_RET_ALLOC, 0);
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

    /// Write `s` as bytes at `addr`, NUL-terminated.
    pub(crate) fn put_ascii_2(c: &mut Cpu, s: &str, addr: u32) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    pub(crate) fn ea_fake_mutex_lock_2(c: &mut Cpu) {
        // Gated like the other fakes: without magic the real lock runs
        // (its wait is what delivers events for no-magic tests).
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_mutex_lock_2),
                ee::s14::f_003e5700__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_2010, ra);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_fake_mutex_unlock_2(c: &mut Cpu) {
        // Gated: see `ea_fake_mutex_lock_2`.
        if c.bus.read32(EAF_CTRL) != EAF_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(ea_fake_mutex_unlock_2),
                ee::s14::f_003e5760__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0xeaf0_2011, ra);
        c.g[V0][0] = 0;
    }

    pub(crate) fn ea_install_oc1125_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE670),
            ea_fake_alloc_request,
            fakes::prev_cell(ea_fake_alloc_request),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DE800),
            ea_fake_alloc_file_handle,
            fakes::prev_cell(ea_fake_alloc_file_handle),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            ea_fake_queue_open,
            fakes::prev_cell(ea_fake_queue_open),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD5A0),
            ea_fake_queue_close,
            fakes::prev_cell(ea_fake_queue_close),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD648),
            ea_fake_queue_read,
            fakes::prev_cell(ea_fake_queue_read),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DDFA8),
            ea_fake_service_queue,
            fakes::prev_cell(ea_fake_service_queue),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            ea_fake_mutex_lock_2,
            fakes::prev_cell(ea_fake_mutex_lock_2),
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            ea_fake_mutex_unlock_2,
            fakes::prev_cell(ea_fake_mutex_unlock_2),
        );
    }

    /// Null queue/alloc/service results so the real Open/Close/Read
    /// workers take their early exits.
    pub(crate) fn ea_null_results(c: &mut Cpu) {
        c.bus.write32(EAF_RET_TYPE, 0);
        c.bus.write32(EAF_RET_ALLOC, 0);
        c.bus.write32(EAF_RET_SVCQ, 0);
        c.bus.write32(EAF_CTRL, EAF_MAGIC);
    }

    driven!(
        gen_ea_file_load_files_worker,
        ee::s14::f_003e1c00__generated
    );

    driven!(hand_ea_file_load_files_worker, ea_file_load_files_worker);

    /// Empty, single and multi-entry lists; null buffers clear the flag.
    /// Layout: count, path-pointer array, out stride, LoadFile A1.
    #[test]
    fn ea_file_load_files_worker_matches() {
        check(
            gen_ea_file_load_files_worker,
            hand_ea_file_load_files_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1c00, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                let list = DATA + 0x400;
                let count = [0u32, 1, 2][seed as usize % 3];
                c.bus.write32(list, count);
                c.bus.write32(list + 4, DATA + 0x700);
                for i in 0..2u32 {
                    put_ascii_2(c, "f.dat", DATA + 0x500 + i * 0x40);
                    c.bus.write32(DATA + 0x700 + i * 4, DATA + 0x500 + i * 0x40);
                }
                c.bus
                    .write32(list + 8, [0u32, 0x100][seed as usize / 3 % 2]);
                c.bus.write32(list + 12, seed as u32 / 6 % 2);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(list);
                c.pc = 0x003e_1c00;
            },
        );
    }

    driven!(gen_ea_file_load_files_ex, ee::s14::f_003e1c98__generated);

    driven!(hand_ea_file_load_files_ex, ea_file_load_files_ex);

    /// The Ex form runs the worker through the priority gate.
    #[test]
    fn ea_file_load_files_ex_matches() {
        check(
            gen_ea_file_load_files_ex,
            hand_ea_file_load_files_ex,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1c98, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii_2(c, "f.dat", DATA + 0x400);
                c.g[A0][0] = (seed % 5) as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.g[A3][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1c98;
            },
        );
    }

    driven!(gen_ea_file_load_files, ee::s14::f_003e1d00__generated);

    driven!(hand_ea_file_load_files, ea_file_load_files);

    /// Same without the priority flag.
    #[test]
    fn ea_file_load_files_matches() {
        check(
            gen_ea_file_load_files,
            hand_ea_file_load_files,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1d00, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii_2(c, "f.dat", DATA + 0x400);
                c.g[A0][0] = (seed % 5) as u64;
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.g[A3][0] = sx(DATA + 0x600);
                c.pc = 0x003e_1d00;
            },
        );
    }

    driven!(
        gen_ea_file_load_big_header_worker,
        ee::s14::f_003e1d68__generated
    );

    driven!(
        hand_ea_file_load_big_header_worker,
        ea_file_load_big_header_worker
    );

    /// Null opens report null; the settings/allocator words select the
    /// header-size and free-callback slots.
    #[test]
    fn ea_file_load_big_header_worker_matches() {
        check(
            gen_ea_file_load_big_header_worker,
            hand_ea_file_load_big_header_worker,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1d68, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                let hdr = DATA + 0x400;
                stream_block(c, hdr);
                c.bus.write32(hdr + 8, 0xa90 + (seed / 3 % 3) * 4);
                put_ascii_2(c, "big.dat", DATA + 0x500);
                c.bus.write32(hdr, sx(DATA + 0x500) as u32);
                c.g[A0][0] = sx(hdr);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_1d68;
            },
        );
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
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii_2(c, "big.dat", DATA + 0x400);
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
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(EAF_RET_SVCQ, (seed / 2 % 2) as u32);
                put_ascii_2(c, "big.dat", DATA + 0x400);
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

    /// Null loads report zero; live loads read the helper result.
    #[test]
    fn ea_file_get_unpacked_size_matches() {
        check(
            gen_ea_file_get_unpacked_size,
            hand_ea_file_get_unpacked_size,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2130, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii_2(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x003e_2130;
            },
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
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c38, 0x99 + seed);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii_2(c, "pack.dat", DATA + 0x400);
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

    /// Null size queries report null; live ones run the unpack helper
    /// and allocator/free callbacks through the jump-table fakes.
    #[test]
    fn ea_file_load_decompressed_matches() {
        check(
            gen_ea_file_load_decompressed,
            hand_ea_file_load_decompressed,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2190, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii_2(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = [0u32, 0x100, 0x1234][seed as usize / 3 % 3] as u64;
                c.pc = 0x003e_2190;
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
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii_2(c, "pack.dat", DATA + 0x400);
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

    /// Into-buffer variant with a caller size word.
    #[test]
    fn ea_file_load_decompressed_into_matches() {
        check(
            gen_ea_file_load_decompressed_into,
            hand_ea_file_load_decompressed_into,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_2310, 0);
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii_2(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = [0u32, 0x100, 0x1234][seed as usize / 3 % 3] as u64;
                c.pc = 0x003e_2310;
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
                ea_install_oc1125_fakes();
                ea_null_results(c);
                c.bus.write32(0x0045_0c10, 0x450000 + seed % 3);
                c.bus.write32(0x0045_0c14, 0x7700_0000 + seed);
                put_ascii_2(c, "pack.dat", DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.pc = 0x003e_2470;
            },
        );
    }

    /// Write `s` as bytes at `addr`, NUL-terminated.
    pub(crate) fn put_ascii_3(c: &mut Cpu, s: &str, addr: u32) {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
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
        check(
            gen_ea_file_platform_get_lsn,
            hand_ea_file_platform_get_lsn,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3538, 0);
                c.bus.write32(0x0045_0c5c, (seed % 7) as u32);
                c.bus.write32(0x0051_9c70, 0x100 + seed);
                c.g[A0][0] = (((seed % 2) << 24 | 0x34_5678) & 0xffff_ffff) as u64;
                c.pc = 0x003e_3538;
            },
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

    /// Empty names exit at once; longer ones fold every byte.
    #[test]
    fn ea_file_hash_name_matches() {
        check(
            gen_ea_file_hash_name,
            hand_ea_file_hash_name,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_36f8, 0);
                let names = ["", "a", "entry", "longer/name/here"];
                put_ascii_3(c, names[seed as usize % 4], DATA + 0x400);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = (seed / 4 % 251) as u64;
                c.pc = 0x003e_36f8;
            },
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

    /// Colon split, dot handling and slash normalization across seeds.
    #[test]
    fn ea_file_resolve_path_matches() {
        check(
            gen_ea_file_resolve_path,
            hand_ea_file_resolve_path,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_3098, 0);
                let paths = [
                    "host0:dir/file",
                    "host0:dir\\sub\\file",
                    "host0:..\\up",
                    "plainname",
                ];
                put_ascii_3(c, paths[seed as usize % 4], DATA + 0x400);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(0x0045_0c44, (seed / 4 % 2) as u32);
                put_ascii_3(c, "host0:", DATA + 0x600);
                c.bus.write32(0x0045_0c50, DATA + 0x600);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = if seed / 8 % 2 == 0 {
                    0
                } else {
                    sx(DATA + 0x700)
                };
                c.pc = 0x003e_3098;
            },
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

    /// Empty tables skip the walk; small tables hash, copy and sort.
    #[test]
    fn ea_file_cd_build_hash_table_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_build_hash_table,
            hand_ea_file_cd_build_hash_table,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_39a8, 0);
                let n = seed % 3;
                let src = DATA + 0x700;
                zero_block(c, src, 64);
                zero_block(c, DATA + 0x500, 64);
                for i in 0..n {
                    put_ascii_3(c, ["alpha", "betaa", "gamma!"][i as usize], src + i * 0x108);
                    c.bus.write32(src + i * 0x108 + 0x100, 0x10 + i);
                    c.bus.write32(src + i * 0x108 + 0x104, 0x20 + i);
                }
                c.bus.write32(0x0045_0c58 + 4, 0);
                c.g[A0][0] = sx(src);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = n as u64;
                c.g[A3][0] = (seed / 3 % 251) as u64;
                c.pc = 0x003e_39a8;
            },
        );
    }

    driven!(gen_ea_file_cd_find_entry, ee::s14::f_003e4968__generated);

    driven!(hand_ea_file_cd_find_entry, ea_file_cd_find_entry);

    /// Index lookup through the table, or zero when absent.
    #[test]
    fn ea_file_cd_find_entry_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_find_entry,
            hand_ea_file_cd_find_entry,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4968, 0);
                c.bus.write32(0x0045_0c5c, 0x20);
                c.bus.write32(0x0051_9c78, DATA + 0x700);
                zero_block(c, DATA + 0x700, 32);
                put_ascii_2(c, "entry-one", DATA + 0x700 + 0x20 + 0xc);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x8, 0x40);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x4, 0x10);
                c.g[A0][0] = (seed % 3 + 1) as u64;
                c.g[A1][0] = 0;
                c.pc = 0x003e_4968;
            },
        );
    }

    driven!(gen_ea_file_cd_get_entry, ee::s14::f_003e48d8__generated);

    driven!(hand_ea_file_cd_get_entry, ea_file_cd_get_entry);

    /// Null outs skip the copy; live outs take it.
    #[test]
    fn ea_file_cd_get_entry_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_get_entry,
            hand_ea_file_cd_get_entry,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_48d8, 0);
                c.bus.write32(0x0045_0c5c, 0x20);
                c.bus.write32(0x0051_9c78, DATA + 0x700);
                zero_block(c, DATA + 0x700, 32);
                put_ascii_2(c, "entry-one", DATA + 0x700 + 0x20 + 0xc);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x8, 0x40);
                c.bus.write32(DATA + 0x700 + 0x20 + 0x4, 0x10);
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

    /// Empty names skip the walk; a name walks the normalizer.
    /// (Zeroed table misses; hits need a coherent table.)
    #[test]
    fn ea_file_cd_lookup_matches() {
        crate::difftest::check_lohi(
            gen_ea_file_cd_lookup,
            hand_ea_file_cd_lookup,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4508, 0);
                let names = ["", "a", "entry", "host0:dir/file"];
                put_ascii_2(c, names[seed as usize % 4], DATA + 0x400);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(0x0045_0c58, DATA + 0x700);
                zero_block(c, DATA + 0x700, 16);
                c.bus.write32(0x0051_9c40 + 0x18, 0);
                c.bus.write32(0x0051_9c40 + 0x4, 0);
                c.bus.write32(0x0051_9c40 + 0x38, 0);
                c.g[A0][0] = sx(DATA + 0x400);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x600);
                c.g[A3][0] = 0;
                c.pc = 0x003e_4508;
            },
        );
    }

    driven!(gen_ea_file_cd_read, ee::s14::f_003e4648__generated);

    driven!(hand_ea_file_cd_read, ea_file_cd_read);

    /// Zeroed table with a live count drives the queue path.
    #[test]
    fn ea_file_cd_read_matches() {
        check(gen_ea_file_cd_read, hand_ea_file_cd_read, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3e_4648, 0);
            // Point the entry table into controlled memory.
            c.bus.write32(0x0045_0c5c, DATA + 0x700 + 0x63c0 - 0xc);
            c.bus.write32(0x0051_9c40 + 0x38, DATA + 0x700);
            zero_block(c, DATA + 0x700, 32);
            c.bus.write32(0x0051_9c40, 0);
            c.g[A0][0] = (seed % 3 + 1) as u64;
            c.g[A1][0] = sx(DATA + 0x500);
            c.g[A2][0] = [0u32, 1, 0x800][seed as usize / 3 % 3] as u64;
            c.g[A3][0] = [0u32, 0x800, 0x1000][seed as usize / 9 % 3] as u64;
            c.pc = 0x003e_4648;
        });
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
}
