//! EE kernel syscall numbers and names.
//!
//! Transcribed from the PS2Recomp syscall database (`db-syscalls.md`), which is
//! cross-referenced with ps2tek. The EE enters the kernel with `syscall` and the
//! number in `$v1`; parameters follow the o32 convention in `$a0..$a3`, and the
//! result returns in `$v0`.
//!
//! `SLUS_207.72` calls `InitMainThread` (0x3C) and `InitHeap` (0x3D) directly from
//! its `entry`, which is the standard PS2 crt0 prologue.

/// Syscall numbers that SSX 3 and the PS2 SDK rely on.
pub mod nr {
    pub const RESET_EE: u32 = 0x01;
    pub const SET_GS_CRT: u32 = 0x02;
    pub const EXIT: u32 = 0x04;
    pub const LOAD_EXEC_PS2: u32 = 0x06;
    pub const EXEC_PS2: u32 = 0x07;

    pub const ADD_INTC_HANDLER: u32 = 0x10;
    pub const REMOVE_INTC_HANDLER: u32 = 0x11;
    pub const ADD_DMAC_HANDLER: u32 = 0x12;
    pub const REMOVE_DMAC_HANDLER: u32 = 0x13;
    pub const ENABLE_INTC: u32 = 0x14;
    pub const DISABLE_INTC: u32 = 0x15;
    pub const ENABLE_DMAC: u32 = 0x16;
    pub const DISABLE_DMAC: u32 = 0x17;

    pub const SET_ALARM: u32 = 0x18;
    pub const RELEASE_ALARM: u32 = 0x19;

    pub const CREATE_THREAD: u32 = 0x20;
    pub const DELETE_THREAD: u32 = 0x21;
    pub const START_THREAD: u32 = 0x22;
    pub const EXIT_THREAD: u32 = 0x23;
    pub const EXIT_DELETE_THREAD: u32 = 0x24;
    pub const TERMINATE_THREAD: u32 = 0x25;
    pub const CHANGE_THREAD_PRIORITY: u32 = 0x29;
    pub const GET_THREAD_ID: u32 = 0x2f;
    pub const REFER_THREAD_STATUS: u32 = 0x30;
    pub const SLEEP_THREAD: u32 = 0x32;
    pub const WAKEUP_THREAD: u32 = 0x33;
    pub const SUSPEND_THREAD: u32 = 0x37;
    pub const RESUME_THREAD: u32 = 0x39;

    pub const INIT_MAIN_THREAD: u32 = 0x3c;
    pub const INIT_HEAP: u32 = 0x3d;
    pub const END_OF_HEAP: u32 = 0x3e;
    pub const GET_HEAP_END: u32 = 0x3f;

    pub const CREATE_SEMA: u32 = 0x40;
    pub const DELETE_SEMA: u32 = 0x41;
    pub const SIGNAL_SEMA: u32 = 0x42;
    pub const WAIT_SEMA: u32 = 0x44;
    pub const POLL_SEMA: u32 = 0x45;
    pub const REFER_SEMA_STATUS: u32 = 0x47;

    pub const CREATE_EVENT_FLAG: u32 = 0x50;
    pub const DELETE_EVENT_FLAG: u32 = 0x51;
    pub const SET_EVENT_FLAG: u32 = 0x52;
    pub const CLEAR_EVENT_FLAG: u32 = 0x54;
    pub const WAIT_EVENT_FLAG: u32 = 0x56;
    pub const POLL_EVENT_FLAG: u32 = 0x57;
    pub const REFER_EVENT_FLAG_STATUS: u32 = 0x59;

    pub const GET_THREAD_TLS: u32 = 0x5b;

    pub const FLUSH_CACHE: u32 = 0x64;

    pub const GS_GET_IMR: u32 = 0x70;
    pub const GS_PUT_IMR: u32 = 0x71;
    pub const SET_VSYNC_FLAG: u32 = 0x73;

    pub const SET_SYSCALL: u32 = 0x74;
    pub const SIF_DMA_STAT: u32 = 0x76;
    pub const SIF_SET_DMA: u32 = 0x77;
    pub const SIF_SET_DCHAIN: u32 = 0x78;
    pub const GET_OSD_CONFIG_PARAM: u32 = 0x79;
    pub const GET_OSD_CONFIG_PARAM2: u32 = 0x7a;
    pub const EXEC_OSD: u32 = 0x7b;
    pub const SET_OSD_CONFIG_PARAM: u32 = 0x7e;
    pub const SET_OSD_CONFIG_PARAM2: u32 = 0x7f;
    pub const FIND_ADDRESS: u32 = 0x83;
    pub const SET_MEMORY_MODE: u32 = 0x85;

    pub const KPRINTF: u32 = 0xfc;
}

/// Human-readable name for a syscall number, or `None` if it is one of the
/// many undocumented slots.
pub fn name(n: u32) -> Option<&'static str> {
    Some(match n {
        nr::RESET_EE => "ResetEE",
        nr::SET_GS_CRT => "SetGsCrt",
        nr::EXIT => "Exit",
        nr::LOAD_EXEC_PS2 => "LoadExecPS2",
        nr::EXEC_PS2 => "ExecPS2",
        nr::ADD_INTC_HANDLER => "AddIntcHandler",
        nr::REMOVE_INTC_HANDLER => "RemoveIntcHandler",
        nr::ADD_DMAC_HANDLER => "AddDmacHandler",
        nr::REMOVE_DMAC_HANDLER => "RemoveDmacHandler",
        nr::ENABLE_INTC => "_EnableIntc",
        nr::DISABLE_INTC => "_DisableIntc",
        nr::ENABLE_DMAC => "_EnableDmac",
        nr::DISABLE_DMAC => "_DisableDmac",
        nr::SET_ALARM => "SetAlarm",
        nr::RELEASE_ALARM => "ReleaseAlarm",
        nr::CREATE_THREAD => "CreateThread",
        nr::DELETE_THREAD => "DeleteThread",
        nr::START_THREAD => "StartThread",
        nr::EXIT_THREAD => "ExitThread",
        nr::EXIT_DELETE_THREAD => "ExitDeleteThread",
        nr::TERMINATE_THREAD => "TerminateThread",
        nr::CHANGE_THREAD_PRIORITY => "ChangeThreadPriority",
        nr::GET_THREAD_ID => "GetThreadId",
        nr::REFER_THREAD_STATUS => "ReferThreadStatus",
        nr::SLEEP_THREAD => "SleepThread",
        nr::WAKEUP_THREAD => "WakeupThread",
        nr::SUSPEND_THREAD => "SuspendThread",
        nr::RESUME_THREAD => "ResumeThread",
        nr::INIT_MAIN_THREAD => "InitMainThread",
        nr::INIT_HEAP => "InitHeap",
        nr::END_OF_HEAP => "EndOfHeap",
        nr::GET_HEAP_END => "GetHeapEnd",
        nr::CREATE_SEMA => "CreateSema",
        nr::DELETE_SEMA => "DeleteSema",
        nr::SIGNAL_SEMA => "SignalSema",
        nr::WAIT_SEMA => "WaitSema",
        nr::POLL_SEMA => "PollSema",
        nr::REFER_SEMA_STATUS => "ReferSemaStatus",
        nr::CREATE_EVENT_FLAG => "CreateEventFlag",
        nr::DELETE_EVENT_FLAG => "DeleteEventFlag",
        nr::SET_EVENT_FLAG => "SetEventFlag",
        nr::CLEAR_EVENT_FLAG => "ClearEventFlag",
        nr::WAIT_EVENT_FLAG => "WaitEventFlag",
        nr::POLL_EVENT_FLAG => "PollEventFlag",
        nr::REFER_EVENT_FLAG_STATUS => "ReferEventFlagStatus",
        nr::GET_THREAD_TLS => "GetThreadTLS",
        nr::FLUSH_CACHE => "FlushCache",
        nr::GS_GET_IMR => "GsGetIMR",
        nr::GS_PUT_IMR => "GsPutIMR",
        nr::SET_VSYNC_FLAG => "SetVSyncFlag",
        nr::SET_SYSCALL => "SetSyscall",
        nr::SIF_DMA_STAT => "SifDmaStat",
        nr::SIF_SET_DMA => "SifSetDma",
        nr::SIF_SET_DCHAIN => "SifSetDChain",
        nr::GET_OSD_CONFIG_PARAM => "GetOsdConfigParam",
        nr::GET_OSD_CONFIG_PARAM2 => "GetOsdConfigParam2",
        nr::EXEC_OSD => "ExecOSD",
        nr::SET_OSD_CONFIG_PARAM => "SetOsdConfigParam",
        nr::SET_OSD_CONFIG_PARAM2 => "SetOsdConfigParam2",
        nr::FIND_ADDRESS => "FindAddress",
        nr::SET_MEMORY_MODE => "SetMemoryMode",
        nr::KPRINTF => "_kprintf",
        _ => return None,
    })
}
