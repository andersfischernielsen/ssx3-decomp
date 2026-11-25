//! `src/ealib/seg_2dcae8.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Target;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::lib::eafile::EA_BSS_HI;
use crate::game::lib::eafile::EA_DATA_HI;
pub(crate) use crate::game::misc::add32;
pub(crate) use crate::game::ui::uitext::oc445_add;
pub(crate) use crate::game::ui::uitext::oc445_addu;
pub(crate) use crate::game::ui::uitext::oc445_lhu;
pub(crate) use crate::game::ui::uitext::oc445_lw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003e_5928,
        ea_timer_service_callbacks,
        "EATimer_ServiceCallbacks",
    ),
    (0x003d_e7b0, ea_file_free_request, "EAFile_FreeRequest"),
    (0x003e_2768, ea_file_big_find_entry, "EAFile_BigFindEntry"),
    (0x0041_6404, strchr, "strchr"),
    (0x003e_0118, stream_file_on_close, "StreamFile_OnClose"),
    (0x003e_03d0, streamfile_issue_read, "StreamFile_IssueRead"),
    (0x003e_5398, ea_thread_yield, "EAThread_Yield"),
    (0x003e_4af0, eatimer_init, "EATimer_Init"),
    (0x003e_57f8, eatimer_add_callback, "EATimer_AddCallback"),
    (
        0x003d_fdd0,
        stream_file_lookup_request,
        "StreamFile_LookupRequest",
    ),
    (
        0x003e_06b0,
        stream_file_calc_mem_size,
        "StreamFile_CalcMemSize",
    ),
    (0x003e_06d8, stream_file_create, "StreamFile_Create"),
    (
        0x003e_0948,
        stream_file_set_channel_filter,
        "StreamFile_SetChannelFilter",
    ),
    (0x003e_0a28, stream_file_destroy, "StreamFile_Destroy"),
    (
        0x003e_0ad8,
        stream_file_set_priority,
        "StreamFile_SetPriority",
    ),
    (
        0x003e_0b28,
        stream_file_set_low_water,
        "StreamFile_SetLowWater",
    ),
    (
        0x003e_0b98,
        stream_file_set_starved,
        "StreamFile_SetStarved",
    ),
    (
        0x003e_0bf8,
        stream_file_get_channel,
        "StreamFile_GetChannel",
    ),
    (
        0x003e_0c58,
        stream_file_get_channel_id,
        "StreamFile_GetChannelId",
    ),
    (0x003e_0d80, stream_file_queue_file, "StreamFile_QueueFile"),
    (
        0x003e_0e90,
        stream_file_cancel_request,
        "StreamFile_CancelRequest",
    ),
    (0x003e_1110, stream_file_cancel_all, "StreamFile_CancelAll"),
    (
        0x003e_14f8,
        stream_file_get_read_state,
        "StreamFile_GetReadState",
    ),
    (
        0x003e_1580,
        stream_file_get_buffer_size,
        "StreamFile_GetBufferSize",
    ),
    (
        0x003e_15b8,
        stream_file_get_buffered_bytes,
        "StreamFile_GetBufferedBytes",
    ),
    (
        0x003e_15e8,
        stream_file_get_chunk_request,
        "StreamFile_GetChunkRequest",
    ),
    (
        0x003e_16b0,
        stream_file_get_request_status,
        "StreamFile_GetRequestStatus",
    ),
    (0x003e_4d68, ea_timer_shutdown, "EATimer_Shutdown"),
    (0x003e_4ea8, ea_timer_get_delta, "EATimer_GetDelta"),
    (0x003e_4f08, ea_timer_set_deadline, "EATimer_SetDeadline"),
    (0x003e_4f40, ea_timer_wait_deadline, "EATimer_WaitDeadline"),
    (
        0x003e_4f80,
        ea_timer_deadline_passed,
        "EATimer_DeadlinePassed",
    ),
    (0x003e_4fb0, ea_timer_delay, "EATimer_Delay"),
    (0x003e_5008, ea_timer_get_tick_rate, "EATimer_GetTickRate"),
    (0x003e_5018, ea_thread_init_main, "EAThread_InitMain"),
    (0x003e_5068, ea_thread_shutdown, "EAThread_Shutdown"),
    (0x003e_50c8, ea_thread_create, "EAThread_Create"),
    (0x003e_51a0, ea_thread_destroy, "EAThread_Destroy"),
    (
        0x003e_5268,
        ea_thread_destroy_thunk,
        "EAThread_DestroyThunk",
    ),
    (0x003e_5290, ea_thread_wakeup_alarm, "EAThread_WakeupAlarm"),
    (0x003e_5440, ea_thread_is_current, "EAThread_IsCurrent"),
    (0x003e_5498, ea_thread_get_priority, "EAThread_GetPriority"),
    (0x003e_5508, ea_thread_set_priority, "EAThread_SetPriority"),
    (0x003e_5580, ea_thread_is_finished, "EAThread_IsFinished"),
    (0x003e_55e0, ea_thread_wait_for_exit, "EAThread_WaitForExit"),
    (0x003e_5678, ea_thread_exit, "EAThread_Exit"),
    (
        0x003e_57d0,
        ea_timer_clear_callbacks,
        "EATimer_ClearCallbacks",
    ),
    (
        0x003e_58c8,
        ea_timer_remove_callback,
        "EATimer_RemoveCallback",
    ),
    (0x003e_5a10, ea_timer_service_for, "EATimer_ServiceFor"),
    (0x003e_6690, ea_timer_set_video_mode, "EATimer_SetVideoMode"),
    (0x003e_52b0, thread_yieldticks, "THREAD_yieldticks"),
    (0x003e_5698, ea_mutex_init, "EAMutex_Init"),
    (0x003e_56e0, ea_mutex_destroy, "EAMutex_Destroy"),
];

/// Callees of the EA file, stream, timer and thread ports below (those
/// defined in other units are re-exported from there).
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::lib::eafile::callee::EA_FILE_SET_REQUEST_PRIORITY;
    pub(crate) use crate::game::lib::eafile::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::lib::eafile::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::lib::eafile::callee::EA_THREAD_IS_CURRENT;
    pub(crate) use crate::game::lib::eafile::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::lib::eafile::callee::EA_TIMER_SERVICE_CALLBACKS;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_CANCEL_REQUEST;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_CONSUME_BYTES;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_FREE_REQUEST;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_GET_CHUNK;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_LOOKUP_REQUEST;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_RING_IN_RANGE;
    pub(crate) use crate::game::lib::eafile::callee::STREAM_FILE_VALIDATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CLOSE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_SET;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ZERO;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MUTEX_DESTROY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MUTEX_INIT;

    pub(crate) const STREAM_FILE_CANCEL_ALL: Callee = Callee {
        addr: 0x003e_1110,
        f: ee::s14::StreamFile_CancelAll,
    };

    pub(crate) const STREAM_FILE_ALLOC_REQUEST: Callee = Callee {
        addr: 0x003d_fcd8,
        f: ee::s14::StreamFile_AllocRequest,
    };

    pub(crate) const STREAM_FILE_ENQUEUE_REQUEST: Callee = Callee {
        addr: 0x003d_fd58,
        f: ee::s14::StreamFile_EnqueueRequest,
    };

    pub(crate) const STREAM_FILE_START_NEXT_REQUEST: Callee = Callee {
        addr: 0x003e_0270,
        f: ee::s14::StreamFile_StartNextRequest,
    };

    /// Intra-module call: the generated body calls the slot target, so both
    /// sides run the installed replacement.
    pub(crate) const STREAM_FILE_CALC_MEM_SIZE: Callee = Callee {
        addr: 0x003e_06b0,
        f: ee::s14::StreamFile_CalcMemSize,
    };

    /// Intra-module call: the generated body calls the raw body.
    pub(crate) const STREAM_FILE_SET_STARVED: Callee = Callee {
        addr: 0x003e_0b98,
        f: ee::s14::StreamFile_SetStarved,
    };

    pub(crate) const EA_TIMER_GET_TICKS: Callee = Callee {
        addr: 0x003e_4e98,
        f: ee::s14::EATimer_GetTicks,
    };

    pub(crate) const EA_TIMER_DEADLINE_PASSED: Callee = Callee {
        addr: 0x003e_4f80,
        f: ee::s14::EATimer_DeadlinePassed,
    };

    pub(crate) const EA_THREAD_DESTROY: Callee = Callee {
        addr: 0x003e_51a0,
        f: ee::s14::EAThread_Destroy,
    };

    pub(crate) const EA_THREAD_YIELD_TICKS: Callee = Callee {
        addr: 0x003e_52b0,
        f: ee::s14::THREAD_yieldticks,
    };
}

/// EE kernel calls of the thread and timer ports (libkernel's syscall
/// stubs, called through the function table).
pub(crate) mod kernel {
    pub(crate) const REMOVE_INTC_HANDLER: u32 = 0x0042_3aa0;
    pub(crate) const CREATE_THREAD: u32 = 0x0042_3ba0;
    pub(crate) const DELETE_THREAD: u32 = 0x0042_3bb0;
    pub(crate) const START_THREAD: u32 = 0x0042_3bc0;
    pub(crate) const EXIT_DELETE_THREAD: u32 = 0x0042_3be0;
    pub(crate) const TERMINATE_THREAD: u32 = 0x0042_3bf0;
    pub(crate) const I_TERMINATE_THREAD: u32 = 0x0042_3c00;
    pub(crate) const CHANGE_THREAD_PRIORITY: u32 = 0x0042_3c30;
    pub(crate) const I_CHANGE_THREAD_PRIORITY: u32 = 0x0042_3c40;
    pub(crate) const GET_THREAD_ID: u32 = 0x0042_3c90;
    pub(crate) const REFER_THREAD_STATUS: u32 = 0x0042_3ca0;
    pub(crate) const I_REFER_THREAD_STATUS: u32 = 0x0042_3cb0;
    pub(crate) const DISABLE_INTC: u32 = 0x0042_4880;
    pub(crate) const I_WAKEUP_THREAD: u32 = 0x0042_4c50;
}

// ---------------------------------------------------------------------------
// EA timer.

/// The current timer tick (`0x450dd0`) and its copy taken by the service
/// pass (`0x450e04`).
pub(crate) const EA_TIMER_NOW: u32 = 0x0045_0dd0;

pub(crate) const EA_TIMER_SERVICED_AT: u32 = 0x0045_0e04;

/// The tick counter `EATimer_GetTicks` reads (`0x450dcc`) and the deadline
/// `EATimer_SetDeadline` stores (`0x450de8`).
#[cfg(test)]
pub(crate) const EA_TIMER_TICKS: u32 = 0x0045_0dcc;

#[cfg(test)]
pub(crate) const EA_TIMER_DEADLINE: u32 = 0x0045_0de8;

/// Sixteen timer callbacks of 0x10 bytes: `+0` the function, `+4` its
/// period, `+8` the tick it is next due, `+0xc` set while it runs.
pub(crate) const EA_TIMERS: u32 = 0x0051_ed98;

pub(crate) const EA_TIMER_COUNT: u64 = 16;

/// `int EATimer_ServiceCallbacks(void* arg)` (0x3e5928): run every timer
/// callback that is due and not already running, as `fn(arg, now - due)`,
/// and schedule it one period later (from the tick after the call). Returns
/// the callbacks' results OR'ed together. Walks the table from the end.
pub fn ea_timer_service_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S3, 0x40);
        c.g[A1][0] = 0x45_0000;
        c.g[S3][0] = c.g[V0][0];
        c.g[V1][0] = Addr(EA_TIMER_NOW).lw(c);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[V0][0] = sx(EA_TIMERS);
        frame.save(c, S2, 0x50);
        // s0 = &timer.running.
        c.g[S0][0] = sx(EA_TIMERS + 0xc);
        frame.save(c, S4, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S5, 0x20);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x10);
        c.g[S5][0] = 1;
        frame.save_ra(c, 0);
        c.g[S6][0] = c.g[S3][0];
        Addr(EA_TIMER_SERVICED_AT).sw(c, c.g[V1][0]);
        c.g[S1][0] = EA_TIMER_COUNT - 1;
        loop {
            // Head 0x3e5980.
            let running = r32(c, S0);
            c.cycles += 3;
            c.g[A2][0] = Addr(running - 0xc).lw(c);
            if c.g[A2][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = Addr(EA_TIMER_NOW).lw(c);
                c.g[V1][0] = Addr(running - 4).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(running).lw(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 12;
                        c.g[A1][0] = Addr(EA_TIMER_NOW).lw(c);
                        c.g[A0][0] = c.g[S4][0];
                        Addr(running).sw(c, c.g[S5][0]);
                        let callback = r32(c, A2);
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, V1)));
                        call_ptr(c, callback, 0x003e_59c0)?;
                        c.g[S2][0] |= c.g[V0][0];
                        c.g[V1][0] = Addr(EA_TIMER_NOW).lw(c);
                        c.g[V0][0] = Addr(running - 8).lw(c);
                        Addr(running).sw(c, 0);
                        c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                        Addr(running - 4).sw(c, c.g[V1][0]);
                    }
                }
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = add32(c.g[S0][0], 0x10);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_5980);
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S2][0];
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
    })
}

pub(crate) mod big_find_entry_callee {
    use super::ee;
    use super::Callee;

    pub(crate) const GET_FOOTER: Callee = Callee {
        addr: 0x003e_2610,
        f: ee::s14::BigFile_GetFooter,
    };

    pub(crate) const GET_HEADER_SIZE: Callee = Callee {
        addr: 0x003e_2580,
        f: ee::s14::BigFile_GetHeaderSize,
    };

    pub(crate) const GET_TYPE: Callee = Callee {
        addr: 0x003e_2500,
        f: ee::s14::BigFile_GetType,
    };

    pub(crate) const STR_ICMP: Callee = Callee {
        addr: 0x003e_2490,
        f: ee::s14::EAFile_StrICmp,
    };

    /// `strlen` (newlib), called through the function table.
    pub(crate) const STRLEN: u32 = 0x0041_6810;
}

/// Frame slots of [`ea_file_big_find_entry`]: the wanted entry index (a2)
/// and the `out_size` pointer (t0).
pub(crate) const BFE_INDEX: u32 = 0x00;

pub(crate) const BFE_OUT_SIZE: u32 = 0x04;

/// Where the index walk spills its constants 2/3/4 (t0..t2) around `strlen`.
pub(crate) const BFE_T0: u32 = 0x10;

pub(crate) const BFE_T1: u32 = 0x20;

pub(crate) const BFE_T2: u32 = 0x30;

/// BIG type 1 (`C0FB` magic): directory starts at +6 with 3-byte fields.
pub(crate) const BFE_TYPE_C0FB: u64 = 1;

/// BIG type 2 (`BIGF`): directory at +0x10, 4-byte fields (the defaults).
pub(crate) const BFE_TYPE_BIGF: u64 = 2;

/// BIG type 3 (`BIG<digit>`): the digit is the offset field's width.
pub(crate) const BFE_TYPE_BIGN: u64 = 3;

/// `char* EAFile_BigFindEntry(const u8* big /*a0*/, const char* name /*a1*/,
/// int index /*a2*/, u32* out_offset /*a3*/, u32* out_size /*t0*/)`
/// (0x3e2768, asm-only `BIG_locateentryz` in the reference): look an entry
/// up in a BIG archive's directory, by name (case-insensitive) or, when
/// `name` is null, by position. Each directory entry is a big-endian
/// offset field, a big-endian size field and a NUL-terminated name; the
/// field widths depend on the archive type (`C0FB` 3/3, `BIGF` 4/4,
/// `BIG<n>` n/min(n,4) with widths over 4 keeping the low 32 bits). The
/// directory runs from the start of entries to the header end minus the
/// optional 8-byte footer. Writes the entry's offset and size through the
/// non-null out pointers (0 when not found) and returns the entry's name
/// pointer, or null.
pub fn ea_file_big_find_entry(c: &mut Cpu) {
    use big_find_entry_callee::*;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S7, 0x60);
        frame.save(c, FP, 0x50);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x40);
        c.g[S3][0] = c.g[A0][0]; // big
        c.g[S7][0] = c.g[A1][0]; // name
        let index = c.g[A2][0];
        let at = frame.at(c, BFE_INDEX);
        c.sw(at, index);
        c.g[FP][0] = c.g[A3][0]; // out_offset
        let out_size = c.g[T0][0];
        let at = frame.at(c, BFE_OUT_SIZE);
        c.sw(at, out_size);
        // Defaults (BIGF): entries at +0x10, 4-byte offset and size.
        c.g[S1][0] = sx(r32(c, S3).wrapping_add(0x10));
        c.g[S2][0] = 4;
        c.g[S5][0] = 4;
        c.g[S6][0] = 0;
        // BigFile_GetFooter(big, 0, 0): the footer's size (8 or 0).
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        GET_FOOTER.call(c, 0x003e_27c4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        GET_HEADER_SIZE.call(c, 0x003e_27d0)?;
        // The directory ends where the footer starts.
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.g[S3][0];
        c.g[S4][0] = sx(r32(c, V0).wrapping_sub(r32(c, S0)));
        GET_TYPE.call(c, 0x003e_27e0)?;
        big_find_entry_field_widths(c);

        c.cycles += 2;
        let in_directory = c.g[S1][0] < c.g[S4][0];
        c.g[V0][0] = in_directory as u64;
        let found = if c.g[S7][0] != 0 {
            big_find_entry_by_name(c, in_directory)?
        } else {
            big_find_entry_by_index(c, in_directory)?
        };
        if !found {
            big_find_entry_not_found(c);
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// [`ea_file_big_find_entry`]: from the archive type (v0), the directory
/// start (s1) and the offset/size field widths (s2/s5). Leaves v0/v1 as the
/// type dispatch does.
pub(crate) fn big_find_entry_field_widths(c: &mut Cpu) {
    let kind = c.g[V0][0];
    c.g[V1][0] = kind;
    c.g[V0][0] = ((kind as i64) < 3) as u64;
    if kind == BFE_TYPE_BIGF {
        return;
    }
    c.cycles += 2;
    let below_3 = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if below_3 {
        c.cycles += 2;
        if kind == BFE_TYPE_C0FB {
            c.g[S1][0] = sx(r32(c, S3).wrapping_add(6));
            c.cycles += 2;
            c.g[S2][0] = 3;
            c.g[S5][0] = 3;
        } else {
            c.cycles += 2;
        }
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    c.g[V1][0] = 4;
    if kind == BFE_TYPE_BIGN {
        c.cycles += 6;
        // `BIG<digit>`: offset width = digit, size width = min(digit, 4).
        c.g[V0][0] = c.lb(r32(c, S3).wrapping_add(3));
        c.g[S5][0] = sx(r32(c, V0).wrapping_sub(0x30));
        c.g[S2][0] = c.g[S5][0];
        c.g[V0][0] = ((c.g[S5][0] as i64) < 5) as u64;
        if c.g[V0][0] == 0 {
            c.g[S5][0] = c.g[V1][0];
        }
    }
}

/// [`ea_file_big_find_entry`] by name: compare each entry's name (s3) with
/// `name` (s7), stepping over names with `strlen`. True when found (results
/// stored), false at the directory end.
pub(crate) fn big_find_entry_by_name(
    c: &mut Cpu,
    in_directory: bool,
) -> Result<bool, crate::call::Unwound> {
    use big_find_entry_callee::*;
    c.cycles += 2;
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
    if !in_directory {
        return Ok(false);
    }
    c.cycles += 1;
    c.g[S6][0] = 1;
    loop {
        c.cycles += 6;
        c.g[A1][0] = c.g[S7][0];
        c.g[S3][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
        c.g[A0][0] = c.g[S3][0];
        STR_ICMP.call(c, 0x003e_2858)?;
        if c.g[V0][0] == 0 {
            break;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, STRLEN, 0x003e_2a0c)?;
        // Next entry: past the name's NUL.
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
        c.g[S1][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            return Ok(false);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_2848);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 5) as u64;
    if c.g[FP][0] != 0 {
        big_find_entry_store_offset(c, true);
    }
    c.cycles += 3;
    let at = sp(c).wrapping_add(BFE_OUT_SIZE);
    c.g[V0][0] = c.lw(at);
    if c.g[V0][0] == 0 {
        c.g[V0][0] = c.g[S3][0];
        return Ok(true);
    }
    big_find_entry_store_size(c, true);
    Ok(true)
}

/// [`ea_file_big_find_entry`] by position: walk the entries counting in s6
/// until it equals the wanted index. True when found (results stored).
pub(crate) fn big_find_entry_by_index(
    c: &mut Cpu,
    in_directory: bool,
) -> Result<bool, crate::call::Unwound> {
    use big_find_entry_callee::*;
    c.cycles += 2;
    if !in_directory {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[S7][0] = 1;
    c.g[T0][0] = 2;
    c.g[T1][0] = 3;
    c.g[T2][0] = 4;
    let frame = Frame { size: 0xe0 };
    loop {
        c.cycles += 6;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
        c.g[S3][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
        c.g[S0][0] = c.g[V0][0];
        let at = frame.at(c, BFE_INDEX);
        c.g[V0][0] = c.lw(at);
        if c.g[S6][0] == c.g[V0][0] {
            break;
        }
        frame.save(c, T0, BFE_T0);
        c.cycles += 12;
        c.g[A0][0] = c.g[S3][0];
        frame.save(c, T1, BFE_T1);
        c.g[S6][0] = sx(r32(c, S6).wrapping_add(1));
        frame.save(c, T2, BFE_T2);
        call_ptr(c, STRLEN, 0x003e_2c78)?;
        c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, V0)));
        frame.restore(c, T0, BFE_T0);
        c.g[S1][0] = sx(r32(c, V0).wrapping_add(1));
        frame.restore(c, T1, BFE_T1);
        c.g[V0][0] = (c.g[S1][0] < c.g[S4][0]) as u64;
        frame.restore(c, T2, BFE_T2);
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003e_2a40);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 5) as u64;
    if c.g[FP][0] != 0 {
        big_find_entry_store_offset(c, false);
    }
    c.cycles += 3;
    let at = frame.at(c, BFE_OUT_SIZE);
    c.g[V1][0] = c.lw(at);
    c.g[V0][0] = c.g[S3][0];
    if c.g[V1][0] == 0 {
        return Ok(true);
    }
    big_find_entry_store_size(c, false);
    Ok(true)
}

/// Not found: zero the non-null out pointers, return null.
pub(crate) fn big_find_entry_not_found(c: &mut Cpu) {
    c.cycles += 2;
    if c.g[FP][0] != 0 {
        c.sw(r32(c, FP), 0);
    }
    c.cycles += 3;
    let at = sp(c).wrapping_add(BFE_OUT_SIZE);
    c.g[V0][0] = c.lw(at);
    if c.g[V0][0] != 0 {
        c.sw(r32(c, V0), 0);
    }
    c.cycles += 1;
    c.g[V0][0] = 0;
}

/// A big-endian field of `width` (1..=4) bytes at `at`, leaving v1/a0/a1
/// as the compiled byte assembly does; other widths read nothing and give 0.
pub(crate) fn big_find_entry_load_be(c: &mut Cpu, at: u32, width: u64) -> u32 {
    match width {
        1 => c.lbu(at) as u32,
        2 => {
            let b0 = c.lbu(at) as u32;
            c.g[V1][0] = c.lbu(at.wrapping_add(1));
            (b0 << 8) | c.g[V1][0] as u32
        }
        3 => {
            let b0 = c.lbu(at) as u32;
            let b1 = c.lbu(at.wrapping_add(1)) as u32;
            c.g[A0][0] = c.lbu(at.wrapping_add(2));
            c.g[V1][0] = sx(b1 << 8);
            (b0 << 16) | (b1 << 8) | c.g[A0][0] as u32
        }
        4 => {
            let b0 = c.lbu(at) as u32;
            let b1 = c.lbu(at.wrapping_add(1)) as u32;
            let b2 = c.lbu(at.wrapping_add(2)) as u32;
            c.g[A1][0] = c.lbu(at.wrapping_add(3));
            c.g[V1][0] = sx(b1 << 16);
            c.g[A0][0] = sx(b2 << 8);
            (b0 << 24) | (b1 << 16) | (b2 << 8) | c.g[A1][0] as u32
        }
        _ => 0,
    }
}

/// The width dispatch ahead of a short field read (1, 2, 3, 4 or none),
/// with its cycles; the by-name copy also sets v1 = 3 past width 1.
pub(crate) fn big_find_entry_short_field(c: &mut Cpu, at: u32, width: u64, by_name: bool) -> u32 {
    c.cycles += 2;
    if width == 1 {
        c.cycles += 2;
        return big_find_entry_load_be(c, at, 1);
    }
    c.cycles += 2;
    if by_name {
        c.g[V1][0] = 3;
    }
    if width == 2 {
        c.cycles += 5;
        return big_find_entry_load_be(c, at, 2);
    }
    c.cycles += 2;
    if width == 3 {
        c.cycles += 8;
        return big_find_entry_load_be(c, at, 3);
    }
    c.cycles += 2;
    if width == 4 {
        c.cycles += 10;
        return big_find_entry_load_be(c, at, 4);
    }
    0
}

/// Store the found entry's offset field (s2 bytes at s1) to `*out_offset`
/// (fp): widths over 4 read the high bytes, then the low 4 bytes, and keep
/// the low 32 bits.
pub(crate) fn big_find_entry_store_offset(c: &mut Cpu, by_name: bool) {
    let field = r32(c, S1);
    let width = c.g[S2][0];
    let value: u64;
    if (width as i64) < 5 {
        c.cycles += 2;
        let v = big_find_entry_short_field(c, field, width, by_name);
        c.cycles += 3;
        value = v as u64;
    } else {
        c.g[V0][0] = sx((width as u32).wrapping_sub(4));
        c.cycles += 2;
        let high_width = c.g[V0][0];
        let high = big_find_entry_short_field(c, field, high_width, false);
        c.cycles += 15;
        let low_at = r32(c, S0).wrapping_sub(4);
        c.g[A1][0] = sx(low_at);
        let b0 = c.lbu(low_at) as u32;
        let b1 = c.lbu(low_at.wrapping_add(1)) as u32;
        let b2 = c.lbu(low_at.wrapping_add(2)) as u32;
        c.g[A2][0] = c.lbu(low_at.wrapping_add(3));
        c.g[V1][0] = sx(b1 << 16);
        c.g[A0][0] = sx(b2 << 8);
        let low = (b0 << 24) | (b1 << 16) | (b2 << 8) | c.g[A2][0] as u32;
        c.g[A3][0] = (high as u64) << 32;
        value = c.g[A3][0] | low as u64;
    }
    c.cycles += 3;
    c.g[V0][0] = sx(value as u32);
    c.sw(r32(c, FP), c.g[V0][0]);
}

/// Store the found entry's size field (s5 bytes at s0) to `*out_size` and
/// return the entry's name (v0 = s3). The by-name and by-index copies of
/// the width dispatch differ only in cycles.
pub(crate) fn big_find_entry_store_size(c: &mut Cpu, by_name: bool) {
    let field = r32(c, S0);
    let width = c.g[S5][0];
    let value = if by_name {
        let cycles = match width {
            1 => 4,
            2 => 8,
            3 => 15,
            4 => 18,
            _ => 11,
        };
        c.cycles += cycles;
        big_find_entry_load_be(c, field, width)
    } else {
        big_find_entry_short_field(c, field, width, false)
    };
    c.cycles += 4;
    let at = sp(c).wrapping_add(BFE_OUT_SIZE);
    c.g[V1][0] = c.lw(at);
    c.sw(r32(c, V1), value as u64);
    c.g[V0][0] = c.g[S3][0];
}

// ---------------------------------------------------------------------------
// EAFile_FreeRequest (0x3de7b0)

/// The EAFile request-pool mutex (`0x520000 - 0x653c`).
pub(crate) const EAFILE_REQUEST_MUTEX: u32 = 0x0051_9ac4;

/// Size of one EAFile request record.
pub(crate) const EAFILE_REQUEST_SIZE: u64 = 0x30;

/// `void EAFile_FreeRequest(EAFileRequest* req)` (0x3de7b0): under the
/// request-pool mutex, clears the 0x30-byte request record (a free slot is
/// an all-zero record).
pub fn ea_file_free_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(EAFILE_REQUEST_MUTEX);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x003e_5700,
            f: ee::s14::EAMutex_Lock,
        }
        .call(c, 0x003d_e7d4)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = EAFILE_REQUEST_SIZE;
        Callee {
            addr: 0x003e_6448,
            f: ee::s14::EA_MemSet,
        }
        .call(c, 0x003d_e7e4)?;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x003e_5760,
            f: ee::s14::EAMutex_Unlock,
        }
        .call(c, 0x003d_e7ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x00416404 strchr

/// 0x0101..01 and 0x8080..80 as newlib builds them in a register.
pub(crate) const STRCHR_ONES: u64 = 0x0101_0101_0101_0101;

pub(crate) const STRCHR_HIGHS: u64 = 0x8080_8080_8080_8080;

/// `pxor`.
pub(crate) fn strchr_pxor(a: crate::mmi::Gpr, b: crate::mmi::Gpr) -> crate::mmi::Gpr {
    [a[0] ^ b[0], a[1] ^ b[1]]
}

/// `char* strchr(const char* s, int ch)` (0x416404, newlib): the first
/// `(char)ch` in `s` (the terminating NUL counts), or null.
///
/// From an 8-byte aligned `s` it skips whole blocks holding neither a NUL
/// nor `ch`: 16 bytes per iteration (MMI, per-byte lanes) when `s` is
/// 16-byte aligned, else 8 bytes (`(x - ONES) & !x & HIGHS` on `x` and on
/// `x ^ pattern`); the block with a hit, and unaligned strings, go byte by
/// byte. The loop back edges check events (no unwind).
pub fn strchr(c: &mut Cpu) {
    use crate::mmi::{lq, pand, pcpyld, pcpyud, pnot, psubb};
    const QUAD_HEAD: u32 = 0x0041_6468;
    const DOUBLE_HEAD: u32 = 0x0041_653c;
    const BYTE_HEAD: u32 = 0x0041_6578;
    c.cycles += 3;
    c.g[V0][0] = c.g[A0][0] & 7;
    c.g[A1][0] &= 0xff;
    // The wide paths end with v0 = the byte at a0 (the block with a hit).
    'bytes: {
        if c.g[V0][0] != 0 {
            // Unaligned: straight to the byte loop.
            c.cycles += 1;
            c.g[V0][0] = c.lbu(r32(c, A0));
            break 'bytes;
        }
        // The constants: a2 = ONES, t0 = HIGHS, a3 = ch in every byte.
        c.cycles += 22;
        let ch = c.g[A1][0];
        c.g[A2][0] = STRCHR_ONES;
        c.g[T0][0] = STRCHR_HIGHS;
        let pair = (ch << 8).wrapping_add(ch);
        c.g[V1][0] = c.g[A0][0] & 0xf;
        let quad = (pair << 16).wrapping_add(pair);
        c.g[V0][0] = quad;
        c.g[T2][0] = quad << 32;
        c.g[A3][0] = quad.wrapping_add(quad << 32);
        if c.g[V1][0] == 0 {
            // 16 bytes per iteration.
            loop {
                c.cycles += 12;
                c.g[T1] = lq(c, r32(c, A0));
                c.g[T2] = pcpyld(c.g[A2], c.g[A2]);
                c.g[V1] = pnot(c.g[T1]);
                c.g[V0] = psubb(c.g[T1], c.g[T2]);
                c.g[A2] = pcpyld(c.g[T0], c.g[T0]);
                c.g[V0] = pand(c.g[V0], c.g[V1]);
                c.g[T0] = pcpyld(c.g[A3], c.g[A3]);
                c.g[V0] = pand(c.g[V0], c.g[A2]);
                c.g[V1] = pcpyud(c.g[V0], c.g[A3]);
                c.g[V1][0] |= c.g[V0][0];
                if c.g[V1][0] != 0 {
                    // A NUL in this block.
                    c.g[V0][0] = c.lbu(r32(c, A0));
                    break 'bytes;
                }
                c.cycles += 23;
                c.g[V0] = strchr_pxor(c.g[T1], c.g[T0]);
                c.g[V1] = psubb(c.g[V0], c.g[T2]);
                c.g[V0] = pnot(c.g[V0]);
                c.g[T0][0] = STRCHR_HIGHS;
                c.g[V1] = pand(c.g[V1], c.g[V0]);
                c.g[V1] = pand(c.g[V1], c.g[A2]);
                c.g[A2][0] = STRCHR_ONES;
                c.g[V0] = pcpyud(c.g[V1], c.g[A1]);
                c.g[V1][0] |= c.g[V0][0];
                if c.g[V1][0] != 0 {
                    // `ch` in this block.
                    c.cycles += 2;
                    c.g[V0][0] = c.lbu(r32(c, A0));
                    break 'bytes;
                }
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x10));
                if event_due(c) {
                    deliver_event_here(c, QUAD_HEAD);
                }
            }
        }
        // The first 8 bytes.
        c.cycles += 7;
        c.g[T1][0] = c.ld(r32(c, A0));
        let x = c.g[T1][0];
        c.g[V1][0] = !x;
        c.g[V0][0] = x.wrapping_sub(c.g[A2][0]) & !x & c.g[T0][0];
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lbu(r32(c, A0));
            break 'bytes;
        }
        c.cycles += 7;
        let y = x ^ c.g[A3][0];
        c.g[V1][0] = y.wrapping_sub(c.g[A2][0]) & !y & c.g[T0][0];
        c.g[V0][0] = !y;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.lbu(r32(c, A0));
            break 'bytes;
        }
        // 8 bytes per iteration (t1 = ONES, a2 = the block).
        c.cycles += 2;
        c.g[T1][0] = c.g[A2][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        loop {
            c.cycles += 7;
            c.g[A2][0] = c.ld(r32(c, A0));
            let x = c.g[A2][0];
            c.g[V1][0] = !x;
            let nul = x.wrapping_sub(c.g[T1][0]) & !x & c.g[T0][0];
            c.g[V0][0] = x ^ c.g[A3][0];
            if nul != 0 {
                c.cycles += 1;
                c.g[V0][0] = c.lbu(r32(c, A0));
                break 'bytes;
            }
            c.cycles += 6;
            let y = c.g[V0][0];
            c.g[V1][0] = !y;
            c.g[V0][0] = y.wrapping_sub(c.g[T1][0]) & !y & c.g[T0][0];
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lbu(r32(c, A0));
                break 'bytes;
            }
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
            if event_due(c) {
                deliver_event_here(c, DOUBLE_HEAD);
            }
        }
    }
    // Byte by byte: v0 is the byte at a0.
    loop {
        c.cycles += 3;
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V1][0] = c.lbu(r32(c, A0));
            break;
        }
        if event_due(c) {
            deliver_event_here(c, BYTE_HEAD);
        }
        c.cycles += 2;
        if c.g[V0][0] == c.g[A1][0] {
            c.g[V1][0] = c.lbu(r32(c, A0));
            break;
        }
        c.cycles += 2;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        c.g[V0][0] = c.lbu(r32(c, A0));
    }
    c.cycles += 4;
    c.g[V1][0] ^= c.g[A1][0];
    c.g[V0][0] = if c.g[V1][0] == 0 { c.g[A0][0] } else { 0 };
    ret(c);
}

// ---------------------------------------------------------------------------
// StreamFile_OnClose (0x3e0118)

/// Stream file fields OnClose touches: the pending request (`+0x174`), the
/// refill priority (`+0x40`) and its name buffer (`+0x6c`).
pub(crate) const SFOC_REQUEST: u32 = 0x174;

pub(crate) const SFOC_PRIORITY: u32 = 0x40;

pub(crate) const SFOC_NAME: u32 = 0x6c;

/// The callback an opened stream sets on its request (`func_003E00D8`).
pub(crate) const SFOC_READ_CALLBACK: u32 = 0x003e_00d8;

pub(crate) const SFOC_RELEASE_REQUEST: Callee = Callee {
    addr: 0x003d_d310,
    f: ee::s14::EAFile_ReleaseRequest,
};

pub(crate) const SFOC_QUEUE_OPEN: Callee = Callee {
    addr: 0x003d_d4e8,
    f: ee::s14::EAFile_QueueOpen,
};

pub(crate) const SFOC_SET_REQUEST_CALLBACK: Callee = Callee {
    addr: 0x003d_cf70,
    f: ee::s14::EAFile_SetRequestCallback,
};

/// `void StreamFile_OnClose(int, int, StreamFile* st)` (0x3e0118): release
/// the current file request, then queue a fresh open of the stream's name
/// (`EAFile_QueueOpen(st + 0x6c, 1, st->priority, st)`) and, when that
/// produced a request, set the stream's read callback on it. The request
/// handle lives at `st + 0x174`.
pub fn stream_file_on_close(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SFOC_REQUEST)).lw(c);
        SFOC_RELEASE_REQUEST.call(c, 0x003e_0130)?;
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(SFOC_PRIORITY)).lw(c);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(SFOC_NAME));
        c.g[A1][0] = 1;
        c.g[A3][0] = c.g[S0][0];
        SFOC_QUEUE_OPEN.call(c, 0x003e_0144)?;
        let failed = c.g[V0][0] == 0;
        Addr(r32(c, S0)).off(SFOC_REQUEST).sw(c, c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = sx(SFOC_READ_CALLBACK);
            SFOC_SET_REQUEST_CALLBACK.call(c, 0x003e_015c)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const EAFS_MUTEX_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::f_003e5700,
};

pub(crate) const EAFS_MUTEX_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::f_003e5760,
};

// ---------------------------------------------------------------------------
// StreamFile_IssueRead
// ---------------------------------------------------------------------------

pub(crate) const SFIR_RING_IN_RANGE: Callee = Callee {
    addr: 0x003d_fc08,
    f: ee::s14::f_003dfc08,
};

pub(crate) const SFIR_FREE_REQUEST: Callee = Callee {
    addr: 0x003d_fe18,
    f: ee::s14::f_003dfe18,
};

pub(crate) const SFIR_MEM_COPY: Callee = Callee {
    addr: 0x003e_6574,
    f: ee::s14::f_003e6574,
};

pub(crate) const SFIR_ON_READ: Callee = Callee {
    addr: 0x003e_0170,
    f: ee::s14::f_003e0170,
};

pub(crate) const SFIR_QUEUE_READ: Callee = Callee {
    addr: 0x003d_d648,
    f: ee::s14::f_003dd648,
};

pub(crate) const SFIR_SET_REQUEST_CALLBACK: Callee = Callee {
    addr: 0x003d_cf70,
    f: ee::s14::f_003dcf70,
};

/// Ring markers: `-1` wrap to the start, `-2` skip `[1]` bytes.
pub(crate) const SFIR_WRAP: u64 = u64::MAX;

pub(crate) const SFIR_ENTRY: u32 = 0x003e_03d0;

pub(crate) const SFIR_SKIP: u32 = 0x003e_0400;

pub(crate) const SFIR_FREE: u32 = 0x003e_0450;

pub(crate) const SFIR_SCAN: u32 = 0x003e_0458;

/// `void StreamFile_IssueRead(StreamFile* sf, int priority)` (0x3e03d0):
/// moves the ring's read pointer (`+0x50`) past wrap / skip markers, frees
/// the finished requests that no longer overlap the free space (under the
/// file's mutex `+4`), then finds room for the next read at the write
/// pointer (`+0x58`): before the read pointer, at the end of the buffer
/// (`+0x34`), or by moving the pending data (`+0x54..+0x58`) back to the
/// start (`+0x2c`, 64-byte aligned unless memory-backed). Too little room
/// (`< +0x17c`): state 2 (`+0x38`). Otherwise copies from memory (source
/// `+0x60` kind 1) and completes, or queues a file read with this as its
/// callback. Dispatcher-shaped (both loops can unwind).
pub fn streamfile_issue_read(c: &mut Cpu) {
    guest(c, |c| {
        let s1 = |c: &Cpu| Addr(c.g[S1][0] as u32);
        let frame = Frame { size: 0x50 };
        let mut pc = c.pc;
        loop {
            match pc {
                SFIR_ENTRY => {
                    c.cycles += 12;
                    let frame = Frame::open(c, 0x50);
                    c.g[A2][0] = SFIR_WRAP;
                    frame.save(c, S1, 0x30);
                    frame.save(c, S3, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x40);
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, S2, 0x20);
                    c.g[A1][0] = sx(-2i32 as u32);
                    frame.save_ra(c, 0);
                    c.g[V1][0] = s1(c).off(0x54).lw(c);
                    pc = SFIR_SKIP;
                }
                SFIR_SKIP => {
                    // Skip the markers at the read pointer.
                    c.cycles += 3;
                    let rd = s1(c).off(0x50).lw(c);
                    c.g[V0][0] = rd;
                    c.g[A0][0] = rd;
                    if rd == c.g[V1][0] {
                        c.cycles += 1;
                        c.g[S0][0] = sx(s1(c).0.wrapping_add(4));
                        pc = 0x003e_0440;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(rd as u32).lw(c);
                    if c.g[V0][0] == c.g[A2][0] {
                        c.cycles += 3;
                        c.g[V0][0] = s1(c).off(0x30).lw(c);
                        let v0 = c.g[V0][0];
                        s1(c).off(0x50).sw(c, v0);
                    } else {
                        c.cycles += 2;
                        let skip = c.g[V0][0] == c.g[A1][0];
                        c.g[S0][0] = sx(s1(c).0.wrapping_add(4));
                        if !skip {
                            pc = 0x003e_0440;
                            continue;
                        }
                        c.cycles += 4;
                        let len = Addr(rd as u32).off(4).lw(c);
                        c.g[V0][0] = sx((rd as u32).wrapping_add(len as u32));
                        let v0 = c.g[V0][0];
                        s1(c).off(0x50).sw(c, v0);
                    }
                    if crate::events::back_edge_unwinds(c, SFIR_SKIP) {
                        return Ok(());
                    }
                }
                0x003e_0440 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    EAFS_MUTEX_LOCK.call(c, 0x003e_0448)?;
                    c.g[S2][0] = c.g[S0][0];
                    pc = SFIR_SCAN;
                }
                SFIR_FREE => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    SFIR_FREE_REQUEST.call(c, 0x003e_0458)?;
                    pc = SFIR_SCAN;
                }
                SFIR_SCAN => {
                    // Free the oldest request while it is done and outside
                    // the data still to be read.
                    c.cycles += 4;
                    c.g[S0][0] = s1(c).off(0x5c).lw(c);
                    let req = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
                    c.g[A0][0] = req;
                    c.g[V0][0] = 1;
                    let mut free = false;
                    if req != 0 {
                        c.cycles += 3;
                        c.g[V1][0] = Addr(req as u32).off(4).lw(c);
                        if c.g[V1][0] != 1 {
                            c.cycles += 7;
                            c.g[A2][0] = Addr(req as u32).off(0x120).lw(c);
                            c.g[A0][0] = s1(c).off(0x50).lw(c);
                            c.g[A1][0] = s1(c).off(0x58).lw(c);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                            SFIR_RING_IN_RANGE.call(c, 0x003e_0488)?;
                            free = c.g[V0][0] == 0;
                            c.g[A1][0] = c.g[S0][0];
                        }
                    }
                    if free {
                        if crate::events::back_edge_unwinds(c, SFIR_FREE) {
                            return Ok(());
                        }
                        pc = SFIR_FREE;
                        continue;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    EAFS_MUTEX_UNLOCK.call(c, 0x003e_0498)?;
                    streamfile_issue_read_room(c)?;
                    pc = 0x003e_0698;
                }
                0x003e_0698 => {
                    c.cycles += 6;
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, SFIR_ENTRY, 0x003e_06b0);
                    return Ok(());
                }
            }
        }
    })
}

/// [`streamfile_issue_read`] after the unlock: find the room, then fill it
/// (or state 2). Ends with `s0` restored.
pub(crate) fn streamfile_issue_read_room(c: &mut Cpu) -> crate::call::Flow {
    let sf = Addr(c.g[S1][0] as u32);
    let frame = Frame { size: 0x50 };
    let lt = |c: &Cpu, a: usize, b: usize| ((c.g[a][0] as i64) < (c.g[b][0] as i64)) as u64;
    c.g[T0][0] = sf.off(0x50).lw(c);
    c.g[A3][0] = sf.off(0x58).lw(c);
    let wrapped = c.g[A3][0] < c.g[T0][0];
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
    'room: {
        'fill: {
            if wrapped {
                // Room up to the read pointer.
                c.cycles += 2;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x41));
                break 'room;
            }
            // Room up to the end of the buffer.
            c.cycles += 8;
            c.g[V0][0] = sf.off(0x34).lw(c);
            c.g[V1][0] = sf.off(0x17c).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A3][0] as u32));
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
            c.g[T2][0] = c.g[V1][0];
            c.g[V1][0] = lt(c, A2, V1);
            c.g[A1][0] = 1;
            if c.g[V1][0] == 0 {
                break 'fill;
            }
            // Not enough: move the pending data to the start, if it fits
            // before the read pointer.
            c.cycles += 6;
            c.g[V1][0] = sf.off(0x60).lw(c);
            c.g[A0][0] = sf.off(0x54).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
            c.g[A2][0] = c.g[V1][0];
            let memory = c.g[V0][0] == c.g[A1][0];
            c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.g[V0][0] = sf.off(0x30).lw(c);
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            if memory {
                c.cycles += 6;
                c.g[V0][0] = lt(c, V0, V1);
                let full = c.g[V0][0] != 0;
                c.g[V0][0] = 2;
                if full {
                    c.cycles += 2;
                    return streamfile_issue_read_full(c, frame);
                }
            } else {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = lt(c, V0, V1);
                let full = c.g[V0][0] != 0;
                c.g[V0][0] = 2;
                if full {
                    return streamfile_issue_read_full(c, frame);
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[S0][0] & 0x3f;
            let aligned = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            let mut plain = aligned;
            if !aligned {
                c.cycles += 3;
                c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x10).lw(c);
                plain = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = u64::MAX;
            }
            if plain {
                c.cycles += 3;
                c.g[V0][0] = sf.off(0x2c).lw(c);
                let v0 = c.g[V0][0];
                sf.off(0x30).sw(c, v0);
            } else {
                // Keep the 64-byte phase: start - (used % 64 - 64).
                c.cycles += 10;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3f));
                c.g[V0][0] = lt(c, V0, S0);
                c.g[A0][0] = sf.off(0x2c).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S0][0];
                }
                c.g[V1][0] = sx((((c.g[V1][0] as u32) as i32 >> 6) as u32) << 6);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x40));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                let a0 = c.g[A0][0];
                sf.off(0x30).sw(c, a0);
            }
            c.cycles += 17;
            c.g[A0][0] = sf.off(0x30).lw(c);
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = sf.off(0x54).lw(c);
            SFIR_MEM_COPY.call(c, 0x003e_0584)?;
            // A wrap marker where the data was.
            let old = Addr(sf.off(0x54).lw(c) as u32);
            c.g[A0][0] = old.0 as i32 as u64;
            c.g[V1][0] = 8;
            c.g[V0][0] = SFIR_WRAP;
            old.off(4).sw(c, 8);
            old.sw(c, SFIR_WRAP);
            c.g[A0][0] = sf.off(0x30).lw(c);
            c.g[A1][0] = sf.off(0x50).lw(c);
            c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            let (a0, a2) = (c.g[A0][0], c.g[A2][0]);
            sf.off(0x54).sw(c, a0);
            sf.off(0x58).sw(c, a2);
            c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
            let at_wrap = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(a2 as u32));
            if at_wrap {
                c.cycles += 5;
                c.g[V0][0] = sf.off(0x34).lw(c);
                sf.off(0x50).sw(c, a0);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(a2 as u32));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x40));
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            }
            break 'room;
        }
        // Enough room at the end of the buffer.
        return streamfile_issue_read_fill(c, frame);
    }
    c.cycles += 5;
    c.g[V0][0] = sf.off(0x17c).lw(c);
    c.g[T2][0] = c.g[V0][0];
    c.g[V0][0] = lt(c, A2, V0);
    let full = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if full {
        return streamfile_issue_read_full(c, frame);
    }
    streamfile_issue_read_fill(c, frame)
}

/// [`streamfile_issue_read`]: no room, state 2 (`v0`).
pub(crate) fn streamfile_issue_read_full(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    let v0 = c.g[V0][0];
    Addr(c.g[S1][0] as u32).off(0x38).sw(c, v0);
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// [`streamfile_issue_read`]: `a2` bytes of room at the write pointer:
/// copy from memory and complete, or queue the file read.
pub(crate) fn streamfile_issue_read_fill(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    let sf = Addr(c.g[S1][0] as u32);
    c.cycles += 5;
    c.g[S0][0] = sf.off(0x60).lw(c);
    let src = Addr(c.g[S0][0] as u32);
    c.g[V1][0] = 1;
    c.g[V0][0] = src.off(0x10).lw(c);
    if c.g[V0][0] == 1 {
        // From memory: up to the end of the source.
        c.cycles += 6;
        c.g[A0][0] = sf.off(0x170).lw(c);
        c.g[V1][0] = src.off(0x118).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        let past = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        if past {
            c.cycles += 2;
            let v0 = c.g[V0][0];
            sf.off(0x178).sw(c, v0);
        } else {
            c.cycles += 1;
            let a2 = c.g[A2][0];
            sf.off(0x178).sw(c, a2);
        }
        c.cycles += 14;
        c.g[A0][0] = sf.off(0x58).lw(c);
        c.g[A1][0] = src.off(0x114).lw(c);
        c.g[A2][0] = sf.off(0x178).lw(c);
        SFIR_MEM_COPY.call(c, 0x003e_0638)?;
        let src = Addr(c.g[S0][0] as u32);
        let sf = Addr(c.g[S1][0] as u32);
        c.g[V0][0] = src.off(0x114).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[V1][0] = sf.off(0x178).lw(c);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let v0 = c.g[V0][0];
        src.off(0x114).sw(c, v0);
        SFIR_ON_READ.call(c, 0x003e_0658)?;
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    // From the file: queue the read, `StreamFile_OnRead` its callback.
    c.g[A0][0] = sf.off(0x16c).lw(c);
    c.cycles += 9;
    c.g[A3][0] = c.g[T2][0];
    c.g[A1][0] = sf.off(0x170).lw(c);
    c.g[T0][0] = c.g[S3][0];
    c.g[A2][0] = sf.off(0x58).lw(c);
    c.g[T1][0] = c.g[S1][0];
    let t2 = c.g[T2][0];
    sf.off(0x178).sw(c, t2);
    SFIR_QUEUE_READ.call(c, 0x003e_067c)?;
    let req = c.g[V0][0];
    Addr(c.g[S1][0] as u32).off(0x174).sw(c, req);
    if req != 0 {
        c.cycles += 4;
        c.g[A0][0] = req;
        c.g[A1][0] = sx(0x003e_0170);
        SFIR_SET_REQUEST_CALLBACK.call(c, 0x003e_0694)?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x40);
    Ok(())
}

// ---------------------------------------------------------------------------
// EAThread_Yield (0x3e5398)

/// The EA thread tick rate (ticks per second, `0x450dc8`) and the "in
/// interrupt context" flag (`0x450de4`).
pub(crate) const EATHREAD_TICK_RATE: u32 = 0x0045_0dc8;

pub(crate) const EATHREAD_IN_INTERRUPT: u32 = 0x0045_0de4;

/// `void EAThread_Yield(int ms)` (0x3e5398): sleep `ms * rate / 1000` ticks
/// (at least one) with `THREAD_yieldticks`; with `ms == 0`, rotate this
/// thread's priority ready queue instead (the `i` syscalls in interrupt
/// context).
pub fn ea_thread_yield(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        if c.g[A2][0] != 0 {
            c.cycles += 6 + 9;
            c.g[A1][0] = 0x45_0000;
            c.g[V1][0] = 1000;
            c.g[A0][0] = Addr(EATHREAD_TICK_RATE).lw(c);
            c.g[V0][0] = 0;
            c.g[A1][0] = 1;
            c.mult(0, A0, A2, A0);
            c.div(0, A0, V1);
            c.g[A0][0] = c.lo[0];
            let positive = 0 < (c.g[A0][0] as i64);
            c.g[V0][0] = positive as u64;
            if !positive {
                c.g[A0][0] = 1;
            }
            Callee {
                addr: 0x003e_52b0,
                f: ee::s14::THREAD_yieldticks,
            }
            .call(c, 0x003e_53dc)?;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            let in_interrupt = Addr(EATHREAD_IN_INTERRUPT).lw(c);
            c.g[V1][0] = in_interrupt;
            // (status call, rotate call, return addresses)
            let (refer, rotate, rets) = if in_interrupt != 0 {
                c.cycles += 9;
                (
                    0x0042_3cb0,
                    0x0042_4ce8,
                    [0x003e_53fc, 0x003e_5408, 0x003e_5410],
                )
            } else {
                c.cycles += 8;
                (
                    0x0042_3ca0,
                    0x0042_3c50,
                    [0x003e_5420, 0x003e_542c, 0x003e_5434],
                )
            };
            call_ptr(c, 0x0042_3c90, rets[0])?; // GetThreadId
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, refer, rets[1])?; // (i)ReferThreadStatus(id, &info)
            let priority = frame.at(c, 0x18);
            c.g[A0][0] = Addr(priority).lw(c);
            call_ptr(c, rotate, rets[2])?; // (i)RotateThreadReadyQueue(info.currentPriority)
        }
        frame.restore_ra(c, 0x30);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod oc445_callee {

    pub(crate) use crate::game::ui::uitext::oc445_callee::EA_ADD_SHUTDOWN_HANDLER;
    pub(crate) use crate::game::ui::uitext::oc445_callee::EA_TIMER_SET_VIDEO_MODE;
}

// ---------------------------------------------------------------------------
// EATimer_Init / EATimer_AddCallback.

/// `0x450ddc`: the timer 1 interrupt handler id (non-zero once started).
#[cfg(test)]
pub(crate) const EA_TIMER_READY: u32 = 0x0045_0ddc;

/// `0x450dc8`: ticks per interrupt (EATimer_Init's result).
#[cfg(test)]
pub(crate) const EA_TIMER_TICKS_PER_IRQ: u32 = 0x0045_0dc8;

/// `0x450da0`: the known video mode constant (0: not measured yet).
#[cfg(test)]
pub(crate) const EA_TIMER_VIDEO_MODE: u32 = 0x0045_0da0;

/// `0x450dcc`: the interrupt count (advanced by the timer handler).
#[cfg(test)]
pub(crate) const EA_TIMER_IRQ_COUNT: u32 = 0x0045_0dcc;

/// `int EATimer_Init(int hz)` (0x3e4af0): start the EE timer 1 interrupt
/// (compare, mode, `AddIntcHandler(10, Rcnt1Handler)`, `EnableIntc(10)`) at
/// `hz` (0 means 100) and return the ticks per interrupt (`0x450dc8`). A
/// second call just returns that value. The prescaler (`mode` 0/1/2 for
/// hz >= 3150 / >= 196 / lower) divides the bus clock. Unless the video
/// mode is already known (`0x450da0`) it then measures a frame with
/// timer 0 (spinning on the interrupt count `0x450dcc`) to choose the
/// video mode via `EATimer_SetVideoMode`. Registers `EATimer_Shutdown` as
/// shutdown handler last.
pub fn eatimer_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0x450000;
        frame.save(c, 18, 0x10);
        c.g[A3][0] = c.g[A0][0];
        c.g[V0][0] = oc445_lw(c, V1, 0xddc);
        c.g[S2][0] = c.g[V1][0];
        frame.save(c, 16, 0x30);
        frame.save(c, 17, 0x20);
        let uninitialised = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'done: {
            if !uninitialised {
                c.cycles += 3;
                c.g[V1][0] = 0x450000;
                c.g[V0][0] = oc445_lw(c, V1, 0xdc8);
                break 'done;
            }
            // hz -> compare value (+0xde0) and ticks per interrupt (+0xdc8).
            c.cycles += 5;
            c.g[V0][0] = 0x64;
            if c.g[A3][0] == 0 {
                c.g[A3][0] = c.g[V0][0];
            }
            c.g[V1][0] = ((c.g[A3][0] as i64) < 3150) as u64;
            let fast = c.g[V1][0] == 0;
            c.g[V0][0] = ((c.g[A3][0] as i64) < 196) as u64;
            if fast {
                // hz >= 3150: prescaler 0.
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[A3][0] as u32) >> 31);
                c.g[A1][0] = 0x8ca0000;
                c.g[V1][0] = oc445_addu(c, A3, V1);
                c.cycles += 14;
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                c.g[A0][0] = 0x450000;
                c.g[V1][0] = oc445_addu(c, V1, A1);
                c.g[A2][0] = 0x450000;
                c.div(0, V1, A3);
                c.g[S1][0] = c.g[A2][0];
                c.g[T0][0] = c.g[A0][0];
                c.g[A3][0] = 0;
                c.g[V1][0] = c.lo[0];
                Addr(oc445_add(c, A0, 0xde0) as u32).sh(c, c.g[V1][0]);
                c.g[V0][0] = c.g[V1][0] & 0xffff;
                c.div(0, A1, V0);
                c.g[A1][0] = c.lo[0];
                Addr(oc445_add(c, A2, 0xdc8) as u32).sw(c, c.g[A1][0]);
            } else {
                c.cycles += 2;
                let slow = c.g[V0][0] != 0;
                c.g[V1][0] = sx((c.g[A3][0] as u32) >> 31);
                if !slow {
                    // 196 <= hz < 3150: prescaler 1.
                    c.cycles += 6;
                    c.g[A0][0] = 0x8c0000;
                    c.g[V1][0] = oc445_addu(c, A3, V1);
                    c.g[A0][0] |= 0xa000;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                    c.cycles += 7;
                    c.g[V1][0] = oc445_addu(c, V1, A0);
                    c.g[A1][0] = 0x450000;
                    c.div(0, V1, A3);
                    c.g[A2][0] = 0x450000;
                    c.g[S1][0] = c.g[A2][0];
                    c.g[T0][0] = c.g[A1][0];
                    c.g[A3][0] = 0x1;
                } else {
                    // hz < 196: prescaler 2.
                    c.cycles += 6;
                    c.g[A0][0] = 0x80000;
                    c.g[V1][0] = oc445_addu(c, A3, V1);
                    c.g[A0][0] |= 0xca00;
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 1) as u32);
                    c.cycles += 8;
                    c.g[V1][0] = oc445_addu(c, V1, A0);
                    c.g[A1][0] = 0x450000;
                    c.div(0, V1, A3);
                    c.g[A2][0] = 0x450000;
                    c.g[S1][0] = c.g[A2][0];
                    c.g[T0][0] = c.g[A1][0];
                    c.g[A3][0] = 0x2;
                }
                c.cycles += 7;
                c.g[V1][0] = c.lo[0];
                Addr(oc445_add(c, A1, 0xde0) as u32).sh(c, c.g[V1][0]);
                c.g[V0][0] = c.g[V1][0] & 0xffff;
                c.div(0, A0, V0);
                c.g[A0][0] = c.lo[0];
                Addr(oc445_add(c, A2, 0xdc8) as u32).sw(c, c.g[A0][0]);
            }
            // Program timer 1: count 0, compare, mode | 0x180.
            c.cycles += 24;
            c.g[V0][0] = 0x10000000;
            c.g[A2][0] = 0x10000000;
            c.g[V0][0] |= 0x800;
            c.g[A2][0] |= 0x820;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = 0x10000000;
            c.g[V1][0] |= 0x810;
            c.g[A3][0] |= 0x180;
            c.g[V0][0] = oc445_lhu(c, T0, 0xde0);
            c.g[A1][0] = 0x3e0000;
            c.g[A1][0] = oc445_add(c, A1, 0x4db8);
            c.g[A0][0] = 0xa;
            Addr(c.g[A2][0] as u32).sw(c, c.g[V0][0]);
            c.g[S0][0] = 0x3e0000;
            Addr(c.g[V1][0] as u32).sw(c, c.g[A3][0]);
            c.g[A2][0] = 0;
            // AddIntcHandler(10, EATimer_Rcnt1Handler, 0)
            call_ptr(c, 0x0042_3a80, 0x003e_4c58)?;
            Addr(oc445_add(c, S2, 0xddc) as u32).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xa;
            // EnableIntc(10)
            call_ptr(c, 0x0042_48e8, 0x003e_4c64)?;
            c.g[V1][0] = 0x450000;
            c.g[V0][0] = oc445_lw(c, V1, 0xda0);
            let known = c.g[V0][0] != 0;
            c.g[V1][0] = 0x10000000;
            if !known {
                // Measure the frame length with timer 0 over one `ticks
                // per interrupt` wait on the interrupt count.
                c.cycles += 14;
                c.g[V0][0] = 0x10000000;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] |= 0x10;
                c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V1][0] = 0x83;
                Addr(c.g[V0][0] as u32).sw(c, c.g[V1][0]);
                c.g[A1][0] = 0x450000;
                c.g[A0][0] = oc445_lw(c, A1, 0xdcc);
                c.g[V1][0] = oc445_lw(c, S1, 0xdc8);
                c.g[V0][0] = oc445_lw(c, A1, 0xdcc);
                c.g[A0][0] = oc445_addu(c, A0, V1);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let wait = c.g[V0][0] != 0;
                c.g[V1][0] = c.g[A1][0];
                if wait {
                    c.cycles += 1;
                    loop {
                        c.cycles += 7;
                        c.g[V0][0] = oc445_lw(c, V1, 0xdcc);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        let again = c.g[V0][0] != 0;
                        if again {
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_4cb0);
                            }
                            continue;
                        }
                        break;
                    }
                }
                c.cycles += 7;
                c.g[V0][0] = 0x10000000;
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                Addr(c.g[V0][0] as u32).sw(c, c.g[A2][0]);
                Addr(c.g[V0][0] as u32).sw(c, c.g[A3][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                let nonneg = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x3d55));
                if !nonneg {
                    c.cycles += 3;
                    c.g[V0][0] = 0x10000;
                    c.g[V1][0] = oc445_addu(c, V1, V0);
                    c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x3d55));
                }
                // d - 15701 < 99: mode 1; d - 15576 < 99: mode 2;
                // d - 28801 < 3199: mode 3.
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                let not1 = c.g[V0][0] == 0;
                c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x3cd8));
                if !not1 {
                    c.cycles += 4;
                    c.g[A0][0] = 0x1;
                    oc445_callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d08)?;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x63) as u64;
                    let not2 = c.g[V0][0] == 0;
                    c.g[V0][0] = oc445_add(c, V1, 0u32.wrapping_sub(0x7081));
                    if !not2 {
                        c.cycles += 4;
                        c.g[A0][0] = 0x2;
                        oc445_callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d24)?;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[V0][0] < 0xc7f) as u64;
                        let not3 = c.g[V0][0] == 0;
                        if !not3 {
                            c.cycles += 2;
                            c.g[A0][0] = 0x3;
                            oc445_callee::EA_TIMER_SET_VIDEO_MODE.call(c, 0x003e_4d40)?;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = oc445_add(c, S0, 0x4d68);
            // EA_AddShutdownHandler(EATimer_Shutdown)
            oc445_callee::EA_ADD_SHUTDOWN_HANDLER.call(c, 0x003e_4d48)?;
            c.g[V0][0] = oc445_lw(c, S1, 0xdc8);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EA stream file, timer and thread ports.

/// `u32 StreamFile_LookupRequest(stream, handle)` (0x3dfdd0): finds the
/// queued request for `handle`, or 0 when absent or out of range.
pub fn stream_file_lookup_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.g[A1][0] & 0xff;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x124;
        if in_range {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
            c.mult(0, 2, 3, 2);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            if c.g[A1][0] != c.g[V1][0] {
                c.cycles += 2;
                c.g[V0][0] = 0;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0;
                }
            }
            c.pc = c.g[RA][0] as u32;
            return Ok(());
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `u32 StreamFile_CalcMemSize(channels, buffer, sample_count)`
/// (0x3e06b0): sizes a stream buffer in bytes.
pub fn stream_file_calc_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0xc;
        c.g[V1][0] = 0x124;
        c.mult(0, 5, 5, 2);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
        c.mult(0, 2, 4, 3);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x180));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
        // Tail call: returns directly to the caller's return address.
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `u32 StreamFile_SetChannelFilter(stream, channel, filter)`
/// (0x3e0948): installs a per-channel filter; nonzero when the channel
/// index is out of range or already has one.
pub fn stream_file_set_channel_filter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e097c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e097c)?;
        // Every early exit below restores S0 and jumps to the epilogue;
        // only the fallthrough runs the shared +1 tail.
        let mut done = false;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x50);
            done = true;
        } else {
            c.cycles += 2;
            let empty = (c.g[S1][0] as i64) <= 0;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            // An empty table skips straight to the shared tail; the
            // match/mismatch arms below always run the scan.
            let mut scanned = empty;
            if !empty {
                c.cycles += 5;
                c.g[A0][0] = c.g[V1][0];
                c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    // Channel table full: report the end of the table.
                    frame.restore(c, S0, 0x50);
                    done = true;
                    scanned = true;
                } else {
                    c.cycles += 2;
                    let mismatch = c.g[S1][0] != c.g[V1][0];
                    c.g[V0][0] = c.g[S2][0] | c.g[S3][0];
                    if !mismatch {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            // A filter is already installed.
                            frame.restore(c, S0, 0x50);
                            done = true;
                        }
                        // A clean match falls through to the scan below.
                    }
                }
            }
            if !done && !scanned {
                // Check the limit against the buffer count (-1 and -2 are
                // allowed too).
                c.cycles += 2;
                let positive = (c.g[S0][0] as i64) > 0;
                if positive {
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let neg_one = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 0xfffffffffffffffe;
                    if !neg_one {
                        c.cycles += 2;
                        if c.g[S0][0] != c.g[V0][0] {
                            frame.restore(c, S0, 0x50);
                            done = true;
                        }
                    }
                }
                if !done && !positive {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                }
            }
            if !done && !scanned {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x50);
                    done = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x50);
                        done = true;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = 0xc;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                        c.mult(0, 2, 17, 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                        c.sw(c.g[V1][0] as u32, c.g[S2][0]);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[S3][0]);
                    }
                }
            }
        }
        if !done {
            c.cycles += 1;
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

/// `u32 StreamFile_Destroy(handle)` (0x3e0a28): cancels all requests on
/// a stream and frees it; 0 unless the handle was missing.
pub fn stream_file_destroy(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003e_0a70;
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_0a44);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0a44)?;
        // The invalid-stream path restores S0 and returns V0 directly.
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 7;
            c.g[RA][0] = sx(0x003e_0a54);
            c.g[A0][0] = c.g[S0][0];
            callee::STREAM_FILE_CANCEL_ALL.call(c, 0x003e_0a54)?;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = 0x1;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
            if c.g[V1][0] != c.g[A0][0] {
                c.sw(c.g[V0][0] as u32, 0);
            } else {
                c.cycles += 2;
                c.g[S0][0] = 0x1;
                loop {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_0a78);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_IS_CURRENT.call(c, 0x003e_0a78)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x003e_0a88);
                        c.g[A0][0] = 0;
                        callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_0a88)?;
                    }
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e_0a90);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_YIELD.call(c, 0x003e_0a90)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                    if c.g[V1][0] != c.g[S0][0] {
                        c.cycles += 1;
                        c.sw(c.g[V0][0] as u32, 0);
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD);
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[RA][0] = sx(0x003e_0ab0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            callee::EA_MUTEX_DESTROY.call(c, 0x003e_0ab0)?;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x16c));
            let no_file = c.g[A0][0] == 0;
            frame.restore(c, S0, 0x20);
            if !no_file {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e_0ac8);
                c.g[A1][0] = 0x64;
                callee::EA_FILE_CLOSE.call(c, 0x003e_0ac8)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_SetPriority(stream, pri, sub)` (0x3e0ad8): stores
/// the stream priority pair; 0 unless validation failed.
pub fn stream_file_set_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e0afc);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e0afc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[S0][0]);
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[S1][0]);
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_SetLowWater(stream, level)` (0x3e0b28): lowers the
/// starvation threshold and re-arms starvation when it rises.
pub fn stream_file_set_low_water(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e0b4c);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e0b4c)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 9;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[S0][0]);
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.g[A1][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let same = c.g[V0][0] == c.g[A1][0];
            frame.restore(c, S0, 0x30);
            if !same {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e0b80);
                c.g[A0][0] = c.g[S1][0];
                callee::STREAM_FILE_SET_STARVED.call(c, 0x003e0b80)?;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_SetStarved(stream, starved)` (0x3e0b98): latches
/// the starved flag and re-arms the file's request priority.
pub fn stream_file_set_starved(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e0bb4);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e0bb4)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            let clear = c.g[S0][0] == 0;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[S0][0]);
            if clear {
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x38));
                let waiting = c.g[V0][0] == c.g[V1][0];
                frame.restore(c, S0, 0x20);
                if waiting {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x003e_0be8);
                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x174));
                    callee::EA_FILE_SET_REQUEST_PRIORITY.call(c, 0x003e_0be8)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetChannel(stream, index)` (0x3e0bf8): returns the
/// channel id at `index`, or 0 when out of range.
pub fn stream_file_get_channel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_0c14);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0c14)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            let empty = (c.g[S0][0] as i64) <= 0;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            if !empty {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let in_range = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 4);
                if in_range {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
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

/// `u32 StreamFile_GetChannelId(stream)` (0x3e0c58): returns the id of
/// the stream's current channel, or 0 when invalid.
pub fn stream_file_get_channel_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_0c6c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0c6c)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_QueueFile(stream, ...)` (0x3e0d80): allocates and
/// enqueues a stream request, then kicks the queue; nonzero on success.
pub fn stream_file_queue_file(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD: u32 = 0x003e_0de8;
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_0db4);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0db4)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 5;
            c.g[RA][0] = sx(0x003e_0dc4);
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            callee::STREAM_FILE_ALLOC_REQUEST.call(c, 0x003e_0dc4)?;
            c.g[S1][0] = c.g[V0][0];
            let queued = c.g[S1][0] == 0;
            c.g[V0][0] = 0;
            if !queued {
                c.cycles += 2;
                let has_offset = c.g[S0][0] != 0;
                c.g[S4][0] = 0x1;
                if !has_offset {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    let direct = c.g[V0][0] == c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    if !direct {
                        c.cycles += 1;
                        loop {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                            let more = c.g[V0][0] != c.g[S2][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, HEAD);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S4][0] = 0x1;
                }
                c.cycles += 14;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x118), c.g[S0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x114), c.g[S3][0]);
                c.g[A1][0] = c.g[S1][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x11c), c.g[S2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[S4][0]);
                c.g[RA][0] = sx(0x003e_0e2c);
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                callee::STREAM_FILE_ENQUEUE_REQUEST.call(c, 0x003e_0e2c)?;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_0e38);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_LOCK.call(c, 0x003e_0e38)?;
                c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x38));
                if c.g[S0][0] == 0 {
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[S4][0]);
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[RA][0] = sx(0x003e_0e54);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_UNLOCK.call(c, 0x003e_0e54)?;
                if c.g[S0][0] == 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[RA][0] = sx(0x003e_0e68);
                    c.g[A1][0] = 0;
                    callee::STREAM_FILE_START_NEXT_REQUEST.call(c, 0x003e_0e68)?;
                }
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
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

/// `u32 StreamFile_Create(channels, subs, buffers, block, size)`
/// (0x3e06d8): check the configuration (2-256 channels, 1-16 sub-channels,
/// 1..=subs buffers, at least 6144 bytes left for data) and lay out the
/// stream block, its channel, sub-channel and buffer tables in `block`;
/// returns the buffer table, or 0 when a check fails.
///
/// Dispatcher-shaped in the original: `0x3e072c` (the zero exit, reached
/// from two back edges) and the three table loops `0x3e0878`, `0x3e08d0`,
/// `0x3e08f8` are resume entries.
pub fn stream_file_create(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_06d8 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.g[A2][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A3][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x003e_0710);
                    c.g[S5][0] = c.g[T0][0];
                    callee::STREAM_FILE_CALC_MEM_SIZE.call(c, 0x003e_0710)?;
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 6144) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        pc = 0x003e_0924;
                        continue;
                    }
                    pc = 0x003e_0720;
                    continue;
                }
                0x003e_0720 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 257) as u64;
                    if t {
                        pc = 0x003e_0734;
                        continue;
                    }
                    pc = 0x003e_072c;
                    continue;
                }
                0x003e_072c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x003e_0924;
                    continue;
                }
                0x003e_0734 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x003e_072c) {
                            return Ok(());
                        }
                        pc = 0x003e_072c;
                        continue;
                    }
                    pc = 0x003e_073c;
                    continue;
                }
                0x003e_073c => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x10) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if t {
                        pc = 0x003e_0924;
                        continue;
                    }
                    pc = 0x003e_0748;
                    continue;
                }
                0x003e_0748 => {
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    if t {
                        if back_edge_unwinds(c, 0x003e_072c) {
                            return Ok(());
                        }
                        pc = 0x003e_072c;
                        continue;
                    }
                    pc = 0x003e_0750;
                    continue;
                }
                0x003e_0750 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        pc = 0x003e_0924;
                        continue;
                    }
                    pc = 0x003e_0758;
                    continue;
                }
                0x003e_0758 => {
                    c.cycles += 48;
                    c.g[V0][0] = 0x4d520000;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.g[V0][0] | 0x5453;
                    c.g[RA][0] = sx(0x003e_076c);
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    callee::EA_MUTEX_INIT.call(c, 0x003e_076c)?;
                    c.g[V0][0] = 0x124;
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x180));
                    c.mult(0, 3, 17, 2);
                    c.g[T0][0] = 0xffffffffffffffc0;
                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 4);
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[T1][0] = 0x96;
                    c.g[A3][0] = 0x32;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                    c.g[A1][0] = 0x100;
                    c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[S2][0]);
                    c.g[V1][0] = 0xc;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[S3][0]);
                    c.mult(0, 4, 18, 3);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x44), 0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0);
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x64), 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x34), c.g[T2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[T1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[A3][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[A2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[T3][0]);
                    c.g[RA][0] = sx(0x003e_080c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                    callee::EA_MEM_ZERO.call(c, 0x003e_080c)?;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 16384) as u64;
                    let t = c.g[V0][0] == 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x16c), 0);
                    if t {
                        pc = 0x003e_0820;
                        continue;
                    }
                    pc = 0x003e_0818;
                    continue;
                }
                0x003e_0818 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x800;
                    pc = 0x003e_0860;
                    continue;
                }
                0x003e_0820 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x7fff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0xffff;
                    if t {
                        pc = 0x003e_0838;
                        continue;
                    }
                    pc = 0x003e_0830;
                    continue;
                }
                0x003e_0830 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1000;
                    pc = 0x003e_0860;
                    continue;
                }
                0x003e_0838 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x10000;
                    if t {
                        pc = 0x003e_084c;
                        continue;
                    }
                    pc = 0x003e_0844;
                    continue;
                }
                0x003e_0844 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2000;
                    pc = 0x003e_0860;
                    continue;
                }
                0x003e_084c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x8000;
                    if t {
                        pc = 0x003e_0860;
                        continue;
                    }
                    pc = 0x003e_085c;
                    continue;
                }
                0x003e_085c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x4000;
                    pc = 0x003e_0860;
                    continue;
                }
                0x003e_0860 => {
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
                    let t = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = 0;
                    if t {
                        pc = 0x003e_08a8;
                        continue;
                    }
                    pc = 0x003e_086c;
                    continue;
                }
                0x003e_086c => {
                    c.cycles += 3;
                    c.g[A3][0] = 0x124;
                    c.g[A2][0] = 0;
                    pc = 0x003e_0878;
                    continue;
                }
                0x003e_0878 => {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x124));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    let t = c.g[A0][0] != 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x124));
                    if t {
                        if back_edge_unwinds(c, 0x003e_0878) {
                            return Ok(());
                        }
                        pc = 0x003e_0878;
                        continue;
                    }
                    pc = 0x003e_08a8;
                    continue;
                }
                0x003e_08a8 => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x124;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.mult(0, 2, 17, 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let t = (c.g[S2][0] as i64) <= 0;
                    c.sw((c.g[V0][0] as u32).wrapping_sub(0x118), 0);
                    if t {
                        pc = 0x003e_08f0;
                        continue;
                    }
                    pc = 0x003e_08c0;
                    continue;
                }
                0x003e_08c0 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = 0x1;
                    c.g[V1][0] = 0;
                    pc = 0x003e_08d0;
                    continue;
                }
                0x003e_08d0 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.sw(c.g[V0][0] as u32, 0);
                    let t = c.g[A1][0] != 0;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    if t {
                        if back_edge_unwinds(c, 0x003e_08d0) {
                            return Ok(());
                        }
                        pc = 0x003e_08d0;
                        continue;
                    }
                    pc = 0x003e_08f0;
                    continue;
                }
                0x003e_08f0 => {
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[A1][0] = 0;
                    if t {
                        pc = 0x003e_0920;
                        continue;
                    }
                    pc = 0x003e_08f8;
                    continue;
                }
                0x003e_08f8 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.g[A0][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                    let t = c.g[V0][0] != 0;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    if t {
                        if back_edge_unwinds(c, 0x003e_08f8) {
                            return Ok(());
                        }
                        pc = 0x003e_08f8;
                        continue;
                    }
                    pc = 0x003e_0920;
                    continue;
                }
                0x003e_0920 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    pc = 0x003e_0924;
                    continue;
                }
                0x003e_0924 => {
                    c.cycles += 9;
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
                    c.jump_out(pc, 0x003e_06d8, 0x003e_0948);
                    return Ok(());
                }
            }
        }
    })
}

/// Callback table: 16 records `{ fn, mode, due, f_c }` at `0x51ed98`.
#[cfg(test)]
pub(crate) const EA_TIMER_CALLBACKS: u32 = 0x0052_0000 - 0x1268;

/// Number of adds in progress (`+0xe00`).
#[cfg(test)]
pub(crate) const EA_TIMER_ADDING: u32 = 0x0045_0e00;

/// Current tick (`+0xdd0`).
#[cfg(test)]
pub(crate) const EA_TIMER_TICK: u32 = 0x0045_0dd0;

/// `void EATimer_AddCallback(void (*fn)(), int mode, int delay)` (0x3e57f8,
/// `SYNCTASK_add`): put `fn` into the 16-entry callback table, due `delay`
/// ticks from now. `mode` -1 is stored as 0, 0 as 1. An entry already
/// holding `fn` is replaced; otherwise the first free entry, after skipping
/// as many free entries as there are adds in progress (`+0xe00`, so a
/// nested add from a callback does not take the entry an outer add is
/// about to use). No room: nothing is added.
pub fn eatimer_add_callback(c: &mut Cpu) {
    c.cycles += 5;
    c.g[T1][0] = c.g[A0][0];
    c.g[T0][0] = c.g[A1][0];
    c.g[A0][0] = u64::MAX;
    let mode_given = c.g[T0][0] != c.g[A0][0];
    c.g[T4][0] = c.g[A2][0];
    c.cycles += 2;
    if mode_given {
        c.g[V0][0] = 0x1;
        if c.g[T0][0] == 0 {
            c.g[T0][0] = c.g[V0][0];
        }
    } else {
        c.g[T0][0] = 0;
    }
    c.cycles += 9;
    c.g[V0][0] = 0x450000;
    c.g[T3][0] = 0x520000;
    c.g[A1][0] = oc445_lw(c, V0, 0xe00);
    c.g[T5][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    c.g[T2][0] = u64::MAX;
    c.g[V1][0] = oc445_add(c, A1, 0x1);
    c.g[A3][0] = oc445_add(c, T3, 0u32.wrapping_sub(0x1268));
    Addr(oc445_add(c, V0, 0xe00) as u32).sw(c, c.g[V1][0]);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
        if c.g[V0][0] == c.g[T1][0] {
            // The entry already holds this function.
            c.g[A0][0] = c.g[A2][0];
            c.cycles += 1;
            c.g[A2][0] = oc445_add(c, A2, 0x1);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[A2][0] = oc445_add(c, A2, 0x1);
            } else {
                c.cycles += 2;
                if c.g[A0][0] != c.g[T2][0] {
                    c.g[A2][0] = oc445_add(c, A2, 0x1);
                } else {
                    c.cycles += 2;
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = oc445_add(c, A1, 0xffff_ffff);
                    }
                    c.cycles += 1;
                    c.g[A2][0] = oc445_add(c, A2, 0x1);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < 16) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A3][0] = oc445_add(c, A3, 0x10);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003e_5840);
            }
            continue;
        }
        break;
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    let no_room = c.g[A0][0] == c.g[V0][0];
    c.g[V1][0] = oc445_add(c, T3, 0u32.wrapping_sub(0x1268));
    if !no_room {
        c.cycles += 12;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
        c.g[A1][0] = 0x450000;
        c.g[A2][0] = oc445_addu(c, A0, V1);
        c.g[V0][0] = oc445_lw(c, A1, 0xdd0);
        c.g[A3][0] = oc445_addu(c, V1, A0);
        Addr(c.g[A2][0] as u32).sw(c, c.g[T1][0]);
        c.g[A1][0] = c.g[A3][0];
        Addr(oc445_add(c, A3, 0x4) as u32).sw(c, c.g[T0][0]);
        c.g[V0][0] = oc445_addu(c, V0, T4);
        Addr(oc445_add(c, A1, 0x8) as u32).sw(c, c.g[V0][0]);
        c.g[V1][0] = c.g[A1][0];
        Addr(oc445_add(c, V1, 0xc) as u32).sw(c, 0);
    }
    c.cycles += 4;
    c.g[V0][0] = oc445_lw(c, T5, 0xe00);
    c.g[V0][0] = oc445_add(c, V0, 0xffff_ffff);
    let ra = c.g[RA][0] as u32;
    Addr(oc445_add(c, T5, 0xe00) as u32).sw(c, c.g[V0][0]);
    c.pc = ra;
}

/// `u32 StreamFile_GetReadState(stream)` (0x3e14f8): returns the
/// stream's state word (`+0x38`), or 0 when the stream is invalid.
pub fn stream_file_get_read_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_150c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_150c)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x38));
            frame.restore_ra(c, 0x10);
        } else {
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetBufferSize(stream)` (0x3e1580): usable bytes
/// between the write and read cursors, or 0 when invalid.
pub fn stream_file_get_buffer_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_1594);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_1594)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetBufferedBytes(stream)` (0x3e15b8): bytes currently
/// buffered, or 0 when invalid.
pub fn stream_file_get_buffered_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003e_15cc);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_15cc)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetRequestStatus(stream, handle)` (0x3e16b0):
/// status word of a queued request, or 0 when absent/invalid.
pub fn stream_file_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_16cc);
        c.g[A1][0] = c.g[SP][0];
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_16cc)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 5;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            c.g[RA][0] = sx(0x003e_16e0);
            c.g[A1][0] = c.g[S0][0];
            callee::STREAM_FILE_LOOKUP_REQUEST.call(c, 0x003e_16e0)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `u32 StreamFile_GetChunkRequest(stream)` (0x3e15e8): find the first
/// request after the stream's current one that is taken (status 1) or
/// whose ring position is in range, under the stream mutex; 0 when the
/// stream is invalid or idle.
///
/// Dispatcher-shaped in the original: the walk head `0x3e1650` is a
/// resume entry.
pub fn stream_file_get_chunk_request(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_15e8 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = c.g[SP][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x003e_160c);
                    c.g[A2][0] = c.g[SP][0] | 0x4;
                    callee::STREAM_FILE_VALIDATE.call(c, 0x003e_160c)?;
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if t {
                        pc = 0x003e_1690;
                        continue;
                    }
                    pc = 0x003e_1614;
                    continue;
                }
                0x003e_1614 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    let t = c.g[V1][0] != 0;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x003e_162c;
                        continue;
                    }
                    pc = 0x003e_1624;
                    continue;
                }
                0x003e_1624 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x003e_1690;
                    continue;
                }
                0x003e_162c => {
                    c.cycles += 9;
                    c.g[S3][0] = 0x1;
                    c.g[RA][0] = sx(0x003e_1638);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    callee::EA_MUTEX_LOCK.call(c, 0x003e_1638)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    pc = 0x003e_1654;
                    continue;
                }
                0x003e_1650 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    pc = 0x003e_1654;
                    continue;
                }
                0x003e_1654 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        pc = 0x003e_1684;
                        continue;
                    }
                    pc = 0x003e_1660;
                    continue;
                }
                0x003e_1660 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == c.g[S3][0];
                    c.g[A2][0] = c.g[S2][0];
                    if t {
                        pc = 0x003e_1680;
                        continue;
                    }
                    pc = 0x003e_166c;
                    continue;
                }
                0x003e_166c => {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                    c.g[RA][0] = sx(0x003e_1678);
                    c.g[A1][0] = c.g[S0][0];
                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_1678)?;
                    if c.g[V0][0] == 0 {
                        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        if back_edge_unwinds(c, 0x003e_1650) {
                            return Ok(());
                        }
                        pc = 0x003e_1650;
                        continue;
                    }
                    pc = 0x003e_1680;
                    continue;
                }
                0x003e_1680 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_1684;
                    continue;
                }
                0x003e_1684 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003e_168c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    callee::EA_MUTEX_UNLOCK.call(c, 0x003e_168c)?;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    pc = 0x003e_1690;
                    continue;
                }
                0x003e_1690 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_15e8, 0x003e_16ac);
                    return Ok(());
                }
            }
        }
    })
}

/// `u32 StreamFile_CancelRequest(stream, handle)` (0x3e0e90): cancels
/// a queued chunk request, freeing it or marking it done.
pub fn stream_file_cancel_request(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_F98: u32 = 0x003e_0f98;
        const HEAD_1000: u32 = 0x003e_1000;
        const HEAD_1090: u32 = 0x003e_1090;
        c.cycles += 20;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        frame.save(c, S6, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = 0;
        frame.save(c, S1, 0x90);
        c.g[S7][0] = 0;
        frame.save(c, S2, 0x80);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_0ed8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0);
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_0ed8)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xa0);
        } else {
            'b_3e0f74: {
                'b_3e0f70: {
                    'b_3e0f18: {
                        'b_3e0f10: {
                            c.cycles += 9;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_0eec);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            callee::EA_MUTEX_LOCK.call(c, 0x003e_0eec)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_0ef8);
                            c.g[A1][0] = c.g[S0][0];
                            callee::STREAM_FILE_LOOKUP_REQUEST.call(c, 0x003e_0ef8)?;
                            c.g[A1][0] = c.g[V0][0];
                            let missing = c.g[A1][0] == 0;
                            c.g[V1][0] = 0x4;
                            if missing {
                                break 'b_3e0f10;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                            if c.g[V0][0] != c.g[V1][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_3e0f18;
                            }
                            break 'b_3e0f10;
                        }
                        c.cycles += 2;
                        c.g[S0][0] = 0x1;
                        break 'b_3e0f70;
                    }
                    'b_3e0f34: {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A0][0] {
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            break 'b_3e0f34;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        c.g[RA][0] = sx(0x003e_0f2c);
                        c.g[S0][0] = 0x1;
                        callee::STREAM_FILE_FREE_REQUEST.call(c, 0x003e_0f2c)?;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break 'b_3e0f74;
                    }
                    'b_3e0f50: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let tied = c.g[A1][0] == c.g[V0][0];
                        c.g[S6][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        if tied {
                            break 'b_3e0f50;
                        }
                        c.cycles += 1;
                        c.g[S6][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                        break 'b_3e0f50;
                    }
                    'b_3e0f6c: {
                        c.cycles += 3;
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                        if c.g[A1][0] == 0 {
                            c.g[S7][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                            break 'b_3e0f6c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                        if c.g[V0][0] != c.g[A0][0] {
                            c.g[S7][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                            break 'b_3e0f6c;
                        }
                        c.cycles += 1;
                        c.g[S7][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                        break 'b_3e0f6c;
                    }
                    c.cycles += 1;
                    c.g[S0][0] = 0;
                    break 'b_3e0f70;
                }
                c.cycles += 1;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e0f74;
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_0f7c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            callee::EA_MUTEX_UNLOCK.call(c, 0x003e_0f7c)?;
            if c.g[S0][0] != 0 {
                frame.restore(c, S0, 0xa0);
            } else {
                'b_3e10e0: {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = 0;
                    if empty {
                        break 'b_3e10e0;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    'l_3e0f98: loop {
                        'b_3e10cc: {
                            c.cycles += 8;
                            c.g[V0][0] = sx((c.g[A0][0] as u32) << 4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            if (c.g[V0][0] as i64) <= 0 {
                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                break 'b_3e10cc;
                            }
                            'b_3e10c8: {
                                'b_3e1070: {
                                    c.cycles += 6;
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[A1][0] = c.g[S6][0];
                                    c.g[RA][0] = sx(0x003e_0fc8);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_0fc8)?;
                                    let out = c.g[V0][0] == 0;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    if out {
                                        break 'b_3e1070;
                                    }
                                    c.cycles += 5;
                                    c.g[S0][0] = c.g[S6][0];
                                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    let synced = c.g[S6][0] == c.g[S7][0];
                                    c.g[S3][0] = sx((c.g[V0][0] as u32) << 24);
                                    if synced {
                                        break 'b_3e10c8;
                                    }
                                    c.cycles += 7;
                                    c.g[S2][0] = 0xff0000;
                                    c.g[FP][0] = 0xffffffffffffffff;
                                    c.g[S2][0] = c.g[S2][0] | 0xffff;
                                    c.g[S5][0] = 0xffffffffff000000;
                                    c.g[S4][0] = 0xfffffffffffffffe;
                                    c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                    'l_3e1000: loop {
                                        'b_3e1060: {
                                            'b_3e1010: {
                                                c.cycles += 2;
                                                if c.g[V0][0] != c.g[FP][0] {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    break 'b_3e1010;
                                                }
                                                c.cycles += 2;
                                                c.g[S0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                                                break 'b_3e1060;
                                            }
                                            'b_3e105c: {
                                                c.cycles += 4;
                                                c.g[S1][0] = c.g[V0][0] & c.g[S2][0];
                                                c.g[V0][0] = c.g[V0][0] & c.g[S5][0];
                                                let other = c.g[V0][0] != c.g[S3][0];
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                if other {
                                                    break 'b_3e105c;
                                                }
                                                c.cycles += 15;
                                                c.g[RA][0] = sx(0x003e_1028);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                callee::EA_MUTEX_LOCK.call(c, 0x003e_1028)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_sub(c.g[S1][0] as u32));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    c.g[V0][0],
                                                );
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                c.g[RA][0] = sx(0x003e_1044);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                                callee::EA_MUTEX_UNLOCK.call(c, 0x003e_1044)?;
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                c.g[RA][0] = sx(0x003e_1050);
                                                c.g[A1][0] = c.g[S1][0];
                                                callee::STREAM_FILE_CONSUME_BYTES
                                                    .call(c, 0x003e_1050)?;
                                                c.sw(c.g[S0][0] as u32, c.g[S4][0]);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x4),
                                                    c.g[S1][0],
                                                );
                                                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                                                break 'b_3e105c;
                                            }
                                            c.cycles += 1;
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                            break 'b_3e1060;
                                        }
                                        c.cycles += 2;
                                        if c.g[S0][0] != c.g[S7][0] {
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            if event_due(c) {
                                                deliver_event_here(c, HEAD_1000);
                                            }
                                            continue 'l_3e1000;
                                        }
                                        c.cycles += 2;
                                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                        break 'b_3e10cc;
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S6][0];
                                c.g[A1][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x003e_1084);
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_1084)?;
                                let out = c.g[V0][0] == 0;
                                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                                if out {
                                    break 'b_3e10cc;
                                }
                                c.cycles += 1;
                                'l_3e1090: loop {
                                    c.cycles += 9;
                                    c.g[RA][0] = sx(0x003e_1098);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    callee::STREAM_FILE_GET_CHUNK.call(c, 0x003e_1098)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x003e_10a4);
                                    c.g[A1][0] = c.g[V0][0];
                                    callee::STREAM_FILE_RELEASE_CHUNK.call(c, 0x003e_10a4)?;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                    let starved = (c.g[V0][0] as i64) <= 0;
                                    c.g[A0][0] = c.g[S6][0];
                                    if starved {
                                        break 'b_3e10c8;
                                    }
                                    c.cycles += 5;
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[RA][0] = sx(0x003e_10c0);
                                    c.g[A1][0] = c.g[S7][0];
                                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_10c0)?;
                                    if c.g[V0][0] == 0 {
                                        break 'b_3e10c8;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, HEAD_1090);
                                    }
                                    continue 'l_3e1090;
                                }
                            }
                            c.cycles += 1;
                            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                            break 'b_3e10cc;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            if event_due(c) {
                                deliver_event_here(c, HEAD_F98);
                            }
                            continue 'l_3e0f98;
                        }
                        break 'b_3e10e0;
                    }
                }
                c.cycles += 1;
                frame.restore(c, S0, 0xa0);
            }
        }
        c.cycles += 11;
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
        Ok(())
    })
}

/// `void StreamFile_CancelAll(stream)` (0x3e1110): cancel the requests
/// still listed at `+0x64`, free the queued ones (`+0x5c` up to `+0x60`),
/// clear the buffer table, consume the buffered bytes, free the ring
/// entries from the read cursor (`+0x50`) to the write cursor (`+0x54`),
/// and in state 2 move the write cursor back.
///
/// Dispatcher-shaped in the original: the loop heads `0x3e1158`,
/// `0x3e1188`, `0x3e11c8` and `0x3e1220` are resume entries.
pub fn stream_file_cancel_all(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_1110 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = c.g[SP][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S1][0] = 0;
                    c.g[RA][0] = sx(0x003e_1134);
                    c.g[A2][0] = c.g[SP][0] | 0x4;
                    callee::STREAM_FILE_VALIDATE.call(c, 0x003e_1134)?;
                    if c.g[V0][0] != 0 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        pc = 0x003e_12d0;
                        continue;
                    }
                    pc = 0x003e_113c;
                    continue;
                }
                0x003e_113c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    if c.g[A1][0] == 0 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        pc = 0x003e_12d0;
                        continue;
                    }
                    pc = 0x003e_114c;
                    continue;
                }
                0x003e_114c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    pc = 0x003e_116c;
                    continue;
                }
                0x003e_1158 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_1160);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STREAM_FILE_CANCEL_REQUEST.call(c, 0x003e_1160)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    pc = 0x003e_116c;
                    continue;
                }
                0x003e_116c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_1158) {
                            return Ok(());
                        }
                        pc = 0x003e_1158;
                        continue;
                    }
                    pc = 0x003e_117c;
                    continue;
                }
                0x003e_117c => {
                    c.cycles += 2;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_1198;
                    continue;
                }
                0x003e_1188 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x003e_1194);
                    callee::STREAM_FILE_FREE_REQUEST.call(c, 0x003e_1194)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_1198;
                    continue;
                }
                0x003e_1198 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x60));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x003e_1188) {
                            return Ok(());
                        }
                        pc = 0x003e_1188;
                        continue;
                    }
                    pc = 0x003e_11a8;
                    continue;
                }
                0x003e_11a8 => {
                    c.cycles += 8;
                    c.g[A0][0] = 0x4;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x60));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        pc = 0x003e_11f0;
                        continue;
                    }
                    pc = 0x003e_11c8;
                    continue;
                }
                0x003e_11c8 => {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        if back_edge_unwinds(c, 0x003e_11c8) {
                            return Ok(());
                        }
                        pc = 0x003e_11c8;
                        continue;
                    }
                    pc = 0x003e_11f0;
                    continue;
                }
                0x003e_11f0 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                    c.g[RA][0] = sx(0x003e_11fc);
                    callee::STREAM_FILE_CONSUME_BYTES.call(c, 0x003e_11fc)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    let t = c.g[A0][0] == c.g[V0][0];
                    c.g[A1][0] = 0xff0000;
                    if t {
                        pc = 0x003e_1254;
                        continue;
                    }
                    pc = 0x003e_1210;
                    continue;
                }
                0x003e_1210 => {
                    c.cycles += 4;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    c.g[A2][0] = 0xfffffffffffffffe;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    pc = 0x003e_1220;
                    continue;
                }
                0x003e_1220 => {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[A3][0] {
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        pc = 0x003e_1234;
                        continue;
                    }
                    pc = 0x003e_1228;
                    continue;
                }
                0x003e_1228 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_1248;
                    continue;
                }
                0x003e_1234 => {
                    c.cycles += 5;
                    c.sw(c.g[A0][0] as u32, c.g[A2][0]);
                    c.g[S1][0] = c.g[V0][0] & c.g[A1][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_1248;
                    continue;
                }
                0x003e_1248 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                    if c.g[A0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        if back_edge_unwinds(c, 0x003e_1220) {
                            return Ok(());
                        }
                        pc = 0x003e_1220;
                        continue;
                    }
                    pc = 0x003e_1254;
                    continue;
                }
                0x003e_1254 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V1][0] = 0x2;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x38));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        pc = 0x003e_12d0;
                        continue;
                    }
                    pc = 0x003e_1268;
                    continue;
                }
                0x003e_1268 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
                    let t = c.g[A0][0] != c.g[V0][0];
                    if t {
                        pc = 0x003e_128c;
                        continue;
                    }
                    pc = 0x003e_1274;
                    continue;
                }
                0x003e_1274 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x30), c.g[V1][0]);
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_12c8;
                    continue;
                }
                0x003e_128c => {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                    c.g[V1][0] = 0x40;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                    c.g[V0][0] = c.g[V0][0] & 0x3f;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                    c.g[V0][0] = c.g[V1][0] ^ 0x40;
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = 0;
                    }
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x54));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    pc = 0x003e_12c8;
                    continue;
                }
                0x003e_12c8 => {
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x003e_12d0;
                    continue;
                }
                0x003e_12d0 => {
                    c.cycles += 4;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_1110, 0x003e_12e0);
                    return Ok(());
                }
            }
        }
    })
}

/// `void EATimer_Shutdown()` (0x3e4d68): stop the timer interrupt.
pub fn ea_timer_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = EA_DATA_HI;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xddc));
        let live = c.g[V0][0] != 0;
        c.g[V0][0] = 0x10000000;
        'b_3e4da8: {
            if !live {
                break 'b_3e4da8;
            }
            c.cycles += 9;
            c.g[A0][0] = 0xa;
            c.g[V0][0] = c.g[V0][0] | 0x810;
            c.sw(c.g[V0][0] as u32, 0);
            c.g[RA][0] = sx(0x003e_4d98);
            call_ptr(c, kernel::DISABLE_INTC, 0x003e_4d98)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xddc));
            c.g[RA][0] = sx(0x003e_4da4);
            c.g[A0][0] = 0xa;
            call_ptr(c, kernel::REMOVE_INTC_HANDLER, 0x003e_4da4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xddc), 0);
            break 'b_3e4da8;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_GetDelta()` (0x3e4ea8): ticks since the last call.
pub fn ea_timer_get_delta(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = EA_DATA_HI;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdec));
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4ec8);
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4ec8)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S0][0] as u32));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xdec), c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `u32 EATimer_SetDeadline(dt)` (0x3e4f08): deadline = now + dt.
pub fn ea_timer_set_deadline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4f24);
        c.g[S1][0] = EA_DATA_HI;
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4f24)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void EATimer_WaitDeadline()` (0x3e4f40): pump callbacks and yield
/// until the deadline passes.
///
/// Dispatcher-shaped in the original: the loop head `0x3e4f50` is a
/// resume entry.
pub fn ea_timer_wait_deadline(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_4f40 => {
                    c.cycles += 3;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    pc = 0x003e_4f60;
                    continue;
                }
                0x003e_4f50 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_4f58);
                    callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_4f58)?;
                    c.g[RA][0] = sx(0x003e_4f60);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_YIELD.call(c, 0x003e_4f60)?;
                    pc = 0x003e_4f60;
                    continue;
                }
                0x003e_4f60 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_4f68);
                    callee::EA_TIMER_DEADLINE_PASSED.call(c, 0x003e_4f68)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = 0;
                    if t {
                        if back_edge_unwinds(c, 0x003e_4f50) {
                            return Ok(());
                        }
                        pc = 0x003e_4f50;
                        continue;
                    }
                    pc = 0x003e_4f70;
                    continue;
                }
                0x003e_4f70 => {
                    c.cycles += 3;
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_4f40, 0x003e_4f7c);
                    return Ok(());
                }
            }
        }
    })
}

/// `u32 EATimer_DeadlinePassed()` (0x3e4f80): nonzero once now is past
/// the deadline.
pub fn ea_timer_deadline_passed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4f90);
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4f90)?;
        c.g[A0][0] = EA_DATA_HI;
        c.g[A1][0] = 0xffffffffffffffff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xde8));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void EATimer_Delay(dt)` (0x3e4fb0): pump callbacks and yield for
/// `dt` ticks.
///
/// Dispatcher-shaped in the original: the loop head `0x3e4fd0` is a
/// resume entry.
pub fn ea_timer_delay(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_4fb0 => {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x003e_4fc4);
                    c.g[S0][0] = c.g[A0][0];
                    callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4fc4)?;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    pc = 0x003e_4fe0;
                    continue;
                }
                0x003e_4fd0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_4fd8);
                    callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_4fd8)?;
                    c.g[RA][0] = sx(0x003e_4fe0);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_YIELD.call(c, 0x003e_4fe0)?;
                    pc = 0x003e_4fe0;
                    continue;
                }
                0x003e_4fe0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_4fe8);
                    callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4fe8)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = 0;
                    if t {
                        if back_edge_unwinds(c, 0x003e_4fd0) {
                            return Ok(());
                        }
                        pc = 0x003e_4fd0;
                        continue;
                    }
                    pc = 0x003e_4ff4;
                    continue;
                }
                0x003e_4ff4 => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_4fb0, 0x003e_5004);
                    return Ok(());
                }
            }
        }
    })
}

/// `u32 EATimer_GetTickRate()` (0x3e5008): cached ticks-per-second.
pub fn ea_timer_get_tick_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V1][0] = EA_DATA_HI;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc8));
        c.pc = t;
        Ok(())
    })
}

/// `void EAThread_InitMain()` (0x3e5018): tag this thread in the main
/// thread slot.
pub fn ea_thread_init_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5028);
        call_ptr(c, kernel::GET_THREAD_ID, 0x003e_5028)?;
        c.g[V1][0] = 0x50530000;
        c.g[A0][0] = EA_DATA_HI;
        c.g[V1][0] = c.g[V1][0] | 0x3254;
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdf8));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xdf8), c.g[V1][0]);
        c.g[A1][0] = 0x64;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003e_504c);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        call_ptr(c, kernel::CHANGE_THREAD_PRIORITY, 0x003e_504c)?;
        c.g[A0][0] = EA_BSS_HI;
        c.g[RA][0] = sx(0x003e_5058);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1278));
        callee::EA_MUTEX_INIT.call(c, 0x003e_5058)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_Shutdown()` (0x3e5068): destroy the main-thread
/// mutex and clear its slot.
pub fn ea_thread_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = EA_BSS_HI;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_507c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1278));
        callee::EA_MUTEX_DESTROY.call(c, 0x003e_507c)?;
        c.g[A0][0] = EA_DATA_HI;
        c.g[A1][0] = 0x8;
        c.g[RA][0] = sx(0x003e_508c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdf8));
        callee::EA_MEM_ZERO.call(c, 0x003e_508c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_Create(kind, ...)` (0x3e50c8): create and start a
/// thread of the given kind (`T1` names the priority slot).
pub fn ea_thread_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x61));
        frame.save(c, S0, 0x50);
        c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        let tabled = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A2][0];
        'b_3e513c: {
            if !tabled {
                c.cycles += 1;
                c.g[V1][0] = 0x64;
                break 'b_3e513c;
            }
            c.cycles += 7;
            c.g[V0][0] = 0x490000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5e00));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            // Jump table (game data at 0x495e00): each arm loads one EE
            // thread priority.
            let (cost, priority) = match c.g[A0][0] as u32 {
                0x003e_5108 => (2, 0x61),
                0x003e_5110 => (2, 0x62),
                0x003e_5118 => (2, 0x63),
                0x003e_5120 => (2, 0x65),
                0x003e_5128 => (2, 0x66),
                0x003e_5130 => (2, 0x67),
                0x003e_5138 => (1, 0x64),
                t => {
                    c.jump_out(t, 0x003e_50c8, 0x003e_519c);
                    return Ok(());
                }
            };
            c.cycles += cost;
            c.g[V1][0] = priority;
            break 'b_3e513c;
        }
        'b_3e5188: {
            c.cycles += 13;
            c.g[V0][0] = 0x4a0000;
            c.g[A0][0] = c.g[SP][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30f0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[T0][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
            c.g[RA][0] = sx(0x003e_5160);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            call_ptr(c, kernel::CREATE_THREAD, 0x003e_5160)?;
            c.g[A0][0] = c.g[V0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
            let failed = c.g[A0][0] == 0;
            c.g[V0][0] = 0;
            if failed {
                break 'b_3e5188;
            }
            c.cycles += 6;
            c.g[V0][0] = 0x50530000;
            c.g[A1][0] = c.g[S1][0];
            c.g[V0][0] = c.g[V0][0] | 0x3254;
            c.g[RA][0] = sx(0x003e_5184);
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            call_ptr(c, kernel::START_THREAD, 0x003e_5184)?;
            c.g[V0][0] = 0x1;
            break 'b_3e5188;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_Destroy(thread)` (0x3e51a0): stop the thread unless
/// it is this one, then release it.
pub fn ea_thread_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xffffffffffff0000;
        frame.save(c, S0, 0x50);
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        frame.save(c, S1, 0x40);
        let is_self = c.g[A0][0] == c.g[V0][0];
        frame.save_ra(c, 0x30);
        'b_3e51d0: {
            'b_3e51cc: {
                if !is_self {
                    break 'b_3e51cc;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_51c4);
                call_ptr(c, kernel::GET_THREAD_ID, 0x003e_51c4)?;
                c.g[S0][0] = c.g[V0][0];
                break 'b_3e51d0;
            }
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            break 'b_3e51d0;
        }
        'b_3e5204: {
            'b_3e51f4: {
                c.cycles += 4;
                c.g[V1][0] = EA_DATA_HI;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V0][0] == 0;
                c.g[S1][0] = c.g[V1][0];
                if no_main {
                    break 'b_3e51f4;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x003e_51ec);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, kernel::I_REFER_THREAD_STATUS, 0x003e_51ec)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e5204;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003e_5200);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, kernel::REFER_THREAD_STATUS, 0x003e_5200)?;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3e5204;
        }
        'b_3e5258: {
            'b_3e5240: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0xf;
                let plain = c.g[V0][0] == 0;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xde4));
                if plain {
                    break 'b_3e5240;
                }
                'b_3e5230: {
                    'b_3e5228: {
                        c.cycles += 2;
                        let dead = c.g[V0][0] == 0;
                        if dead {
                            break 'b_3e5228;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x003e_5220);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, kernel::I_TERMINATE_THREAD, 0x003e_5220)?;
                        break 'b_3e5230;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_5230);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, kernel::TERMINATE_THREAD, 0x003e_5230)?;
                    break 'b_3e5230;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_5238);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, kernel::DELETE_THREAD, 0x003e_5238)?;
                frame.restore(c, S0, 0x50);
                break 'b_3e5258;
            }
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] & 0x10;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x50);
                break 'b_3e5258;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_5254);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, kernel::DELETE_THREAD, 0x003e_5254)?;
            frame.restore(c, S0, 0x50);
            break 'b_3e5258;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_DestroyThunk(thread)` (0x3e5268): destroy the
/// exiting thread.
pub fn ea_thread_destroy_thunk(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(callee::EA_THREAD_DESTROY),
        0x003e_5278,
    );
}

/// `void EAThread_WakeupAlarm(arg)` (0x3e5290): wake the alarm thread.
pub fn ea_thread_wakeup_alarm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_52a0);
        c.g[A0][0] = c.g[A2][0];
        call_ptr(c, kernel::I_WAKEUP_THREAD, 0x003e_52a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_IsCurrent(thread)` (0x3e5440): nonzero when the
/// caller runs on `thread` (or `thread` is null, matching anyone).
pub fn ea_thread_is_current(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5454);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, kernel::GET_THREAD_ID, 0x003e_5454)?;
        let this = c.g[S0][0] != 0;
        c.g[A0][0] = c.g[V0][0];
        'b_3e5484: {
            'b_3e547c: {
                'b_3e5468: {
                    if this {
                        break 'b_3e5468;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = EA_DATA_HI;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdfc));
                    break 'b_3e547c;
                }
                c.cycles += 4;
                c.g[V0][0] = 0xffffffffffff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[V0][0] = 0x1;
                    break 'b_3e5484;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                break 'b_3e547c;
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0] ^ c.g[V0][0];
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
            break 'b_3e5484;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_GetPriority(thread)` (0x3e5498): priority of the
/// thread, or of the caller when `thread` is null.
pub fn ea_thread_get_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x40);
        let given = c.g[A0][0] != 0;
        frame.save_ra(c, 0x30);
        'b_3e54cc: {
            'b_3e54b0: {
                if given {
                    break 'b_3e54b0;
                }
                c.cycles += 3;
                c.g[V0][0] = EA_DATA_HI;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdfc));
                break 'b_3e54cc;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                break 'b_3e54cc;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_54c8);
            call_ptr(c, kernel::GET_THREAD_ID, 0x003e_54c8)?;
            c.g[A0][0] = c.g[V0][0];
            break 'b_3e54cc;
        }
        'b_3e54f8: {
            'b_3e54ec: {
                c.cycles += 4;
                c.g[V0][0] = EA_DATA_HI;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V1][0] == 0;
                if no_main {
                    break 'b_3e54ec;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_54e4);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, kernel::I_REFER_THREAD_STATUS, 0x003e_54e4)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                break 'b_3e54f8;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_54f4);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, kernel::REFER_THREAD_STATUS, 0x003e_54f4)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            break 'b_3e54f8;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_WaitForExit(thread, timeout)` (0x3e55e0): poll the
/// thread's status, yielding between polls, until it is dead (returns 1)
/// or gone (returns 0). With a nonzero `timeout` a live thread is only
/// polled again while `start + timeout` is already past; before that the
/// call returns 0 after one poll.
pub fn ea_thread_wait_for_exit(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_10: u32 = 0x003e_5610;
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x30);
        c.g[RA][0] = sx(0x003e_5600);
        c.g[S1][0] = c.g[A1][0];
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5600)?;
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        'l_3e5610: loop {
            'b_3e565c: {
                'b_3e5658: {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x003e_5618);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, kernel::REFER_THREAD_STATUS, 0x003e_5618)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = c.g[V1][0] & 0x10;
                    let dead = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    if dead {
                        break 'b_3e5658;
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        frame.restore(c, S0, 0x60);
                        break 'b_3e565c;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e_5638);
                    c.g[A0][0] = 0;
                    callee::EA_THREAD_YIELD_TICKS.call(c, 0x003e_5638)?;
                    let infinite = c.g[S1][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if infinite {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_10);
                        }
                        continue 'l_3e5610;
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_5648);
                    callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5648)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let expired = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if expired {
                        if event_due(c) {
                            deliver_event_here(c, HEAD_10);
                        }
                        continue 'l_3e5610;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    break 'b_3e5658;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x60);
                break 'b_3e565c;
            }
            break 'l_3e5610;
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[V0][0] ^ 0x10;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_Exit()` (0x3e5678): stop this thread.
pub fn ea_thread_exit(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Table(kernel::EXIT_DELETE_THREAD),
        0x003e_5688,
    );
}

/// `void EATimer_ClearCallbacks()` (0x3e57d0): drop all timer
/// callbacks.
pub fn ea_timer_clear_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = EA_BSS_HI;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1268));
        c.g[RA][0] = sx(0x003e_57ec);
        c.g[A2][0] = 0x100;
        callee::EA_MEM_SET.call(c, 0x003e_57ec)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EATimer_RemoveCallback(fn)` (0x3e58c8): drop the callback
/// slot holding `fn`.
pub fn ea_timer_remove_callback(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_E8: u32 = 0x003e_58e8;
        c.cycles += 6;
        c.g[V0][0] = EA_BSS_HI;
        c.g[A2][0] = c.g[A0][0];
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x1268));
        c.g[A0][0] = c.g[V0][0];
        let present = c.g[V1][0] == c.g[A2][0];
        c.g[A1][0] = 0;
        'b_3e5920: {
            'b_3e5900: {
                if present {
                    break 'b_3e5900;
                }
                c.cycles += 2;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1268));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                'l_3e58e8: loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 16) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    if done {
                        break 'b_3e5920;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    if c.g[V0][0] != c.g[A2][0] {
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        if event_due(c) {
                            deliver_event_here(c, HEAD_E8);
                        }
                        continue 'l_3e58e8;
                    }
                    break 'b_3e5900;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 16) as u64;
            let missing = c.g[V0][0] == 0;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1268));
            if missing {
                break 'b_3e5920;
            }
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            if c.g[A0][0] == c.g[A2][0] {
                c.sw(c.g[V0][0] as u32, 0);
            }
            break 'b_3e5920;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void EAThread_SetPriority(thread, prio)` (0x3e5508): change the
/// thread's priority (own thread when null).
pub fn ea_thread_set_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let given = c.g[A0][0] != 0;
        c.g[S0][0] = c.g[A1][0];
        'b_3e5544: {
            'b_3e5528: {
                if given {
                    break 'b_3e5528;
                }
                c.cycles += 3;
                c.g[V0][0] = EA_DATA_HI;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdfc));
                break 'b_3e5544;
            }
            c.cycles += 4;
            c.g[V0][0] = 0xffffffffffff0000;
            c.g[V0][0] = c.g[V0][0] | 0xffff;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                break 'b_3e5544;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_5540);
            call_ptr(c, kernel::GET_THREAD_ID, 0x003e_5540)?;
            c.g[A0][0] = c.g[V0][0];
            break 'b_3e5544;
        }
        'b_3e5570: {
            'b_3e5564: {
                c.cycles += 4;
                c.g[V0][0] = EA_DATA_HI;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V1][0] == 0;
                if no_main {
                    break 'b_3e5564;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_555c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, kernel::I_CHANGE_THREAD_PRIORITY, 0x003e_555c)?;
                frame.restore(c, S0, 0x10);
                break 'b_3e5570;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_556c);
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, kernel::CHANGE_THREAD_PRIORITY, 0x003e_556c)?;
            frame.restore(c, S0, 0x10);
            break 'b_3e5570;
        }
        c.cycles += 4;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_IsFinished(thread)` (0x3e5580): nonzero once the
/// thread is done (a live thread reports its status word).
pub fn ea_thread_is_finished(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = EA_DATA_HI;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xde4));
        let no_main = c.g[V0][0] == 0;
        frame.save_ra(c, 0x30);
        'b_3e55b8: {
            'b_3e55a8: {
                if no_main {
                    break 'b_3e55a8;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[RA][0] = sx(0x003e_55a0);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, kernel::I_REFER_THREAD_STATUS, 0x003e_55a0)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e55b8;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x003e_55b4);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, kernel::REFER_THREAD_STATUS, 0x003e_55b4)?;
            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
            break 'b_3e55b8;
        }
        'b_3e55d4: {
            'b_3e55cc: {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x10;
                let dead = c.g[V0][0] != 0;
                c.g[A0][0] = 0;
                if dead {
                    break 'b_3e55cc;
                }
                c.cycles += 2;
                let live = c.g[V1][0] != 0;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                if live {
                    break 'b_3e55d4;
                }
                break 'b_3e55cc;
            }
            c.cycles += 2;
            c.g[A0][0] = 0x1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_3e55d4;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A0][0];
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `u32 EATimer_ServiceFor(ticks)` (0x3e5a10): service timer callbacks
/// until `ticks` elapse or a pass returns nonzero; returns that pass's
/// result.
///
/// Dispatcher-shaped in the original: the loop head `0x3e5a38` is a
/// resume entry.
pub fn ea_timer_service_for(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_5a10 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[RA][0] = sx(0x003e_5a2c);
                    c.g[S1][0] = 0;
                    callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5a2c)?;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    pc = 0x003e_5a44;
                    continue;
                }
                0x003e_5a38 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003e_5a40);
                    c.g[A0][0] = 0;
                    callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_5a40)?;
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x003e_5a44;
                    continue;
                }
                0x003e_5a44 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_5a4c);
                    callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5a4c)?;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[S1][0];
                    if t {
                        pc = 0x003e_5a60;
                        continue;
                    }
                    pc = 0x003e_5a58;
                    continue;
                }
                0x003e_5a58 => {
                    c.cycles += 2;
                    let t = c.g[S1][0] == 0;
                    if t {
                        if back_edge_unwinds(c, 0x003e_5a38) {
                            return Ok(());
                        }
                        pc = 0x003e_5a38;
                        continue;
                    }
                    pc = 0x003e_5a60;
                    continue;
                }
                0x003e_5a60 => {
                    c.cycles += 5;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_5a10, 0x003e_5a74);
                    return Ok(());
                }
            }
        }
    })
}

/// `void EATimer_SetVideoMode(mode)` (0x3e6690): record the display
/// mode for the timer interrupt.
pub fn ea_timer_set_video_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = 0x2;
        let pal = c.g[A0][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[A0][0] as i64) < 3) as u64;
        if pal {
            c.cycles += 4;
            c.g[V1][0] = EA_DATA_HI;
            c.g[V0][0] = 0x3d09;
            let t = c.g[RA][0] as u32;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xda0), c.g[V0][0]);
            c.pc = t;
            return Ok(());
        }
        c.cycles += 2;
        let hi = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if hi {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let ntsc = c.g[A0][0] == c.g[V0][0];
            c.g[V1][0] = EA_DATA_HI;
            if ntsc {
                c.cycles += 3;
                c.g[V0][0] = 0x7080;
                let t = c.g[RA][0] as u32;
                c.sw((c.g[V1][0] as u32).wrapping_add(0xda0), c.g[V0][0]);
                c.pc = t;
                return Ok(());
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.pc = t;
            return Ok(());
        }
        c.cycles += 2;
        let sixty = c.g[A0][0] == c.g[V0][0];
        c.g[V1][0] = EA_DATA_HI;
        if sixty {
            c.cycles += 3;
            c.g[V0][0] = 0x3d86;
            let t = c.g[RA][0] as u32;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xda0), c.g[V0][0]);
            c.pc = t;
            return Ok(());
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// Offsets in the semaphore-backed mutex (`MUTEX`): two counters the lock
/// code keeps (cleared at init) and the kernel semaphore id.
pub(crate) const EA_MUTEX_COUNT_A: u32 = 0x4;

pub(crate) const EA_MUTEX_COUNT_B: u32 = 0x8;

pub(crate) const EA_MUTEX_SEMA: u32 = 0xc;

/// `int EAMutex_Init(MUTEX *m)` (0x3e5698, `MUTEX_create`): clear the mutex's
/// two counters and create its kernel semaphore (initial and maximum count
/// 1, from a parameter block on the stack), keeping the id. Returns 0.
pub fn ea_mutex_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(EA_MUTEX_COUNT_B), 0u64);
        c.g[A0][0] = c.g[SP][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(EA_MUTEX_COUNT_A), 0u64);
        // `ee_sema_t`: max_count and init_count are 1 (count and the rest
        // are whatever the stack held).
        c.sw(frame.at(c, 8), c.g[V0][0]);
        c.g[RA][0] = sx(0x003e_56c4);
        c.sw(frame.at(c, 4), c.g[V0][0]);
        Callee {
            addr: 0x0042_3da0,
            f: ee::s15::CreateSema,
        }
        .call(c, 0x003e_56c4)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(EA_MUTEX_SEMA), c.g[V0][0]);
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void EAMutex_Destroy(MUTEX *m)` (0x3e56e0, `MUTEX_destroy`): delete the
/// mutex's kernel semaphore.
pub fn ea_mutex_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_56f0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(EA_MUTEX_SEMA));
        Callee {
            addr: 0x0042_3db0,
            f: ee::s15::DeleteSema,
        }
        .call(c, 0x003e_56f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void THREAD_yieldticks(int ticks)` (0x3e52b0): sleep `ticks` thread ticks:
/// an alarm for `ticks * rate / tick_rate` (masked to 16 bits) that wakes this
/// thread, then `SleepThread`. With `ticks <= 0` it rotates this thread's
/// ready queue instead (the `i` syscalls when in interrupt context).
pub fn thread_yieldticks(c: &mut Cpu) {
    guest(c, |c| {
        'b_3e5380: {
            'b_3e532c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = (c.g[S2][0] as i64) <= 0;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                if t {
                    break 'b_3e532c;
                } else {
                    'b_3e52e8: {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x003e52d4);
                        c.g[S1][0] = 0x3e0000;
                        Callee {
                            addr: 0x003e5008,
                            f: ee::s14::EATimer_GetTickRate,
                        }
                        .call(c, 0x003e52d4)?;
                        c.g[V1][0] = 0x450000;
                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xda0));
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5290));
                        if c.g[V0][0] == 0u64 {
                            break 'b_3e52e8;
                        } else {
                            break 'b_3e52e8;
                        }
                    }
                    'b_3e531c: {
                        c.cycles += 11;
                        c.mult(0, 16, 18, 16);
                        c.div(0, 16, 2);
                        c.g[S0][0] = c.lo[0];
                        c.g[RA][0] = sx(0x003e52fc);
                        c.g[S0][0] = c.g[S0][0] & 0xffff;
                        call_ptr(c, 0x00423c90, 0x003e52fc)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x003e530c);
                        c.g[A2][0] = c.g[V0][0];
                        call_ptr(c, 0x00423b20, 0x003e530c)?;
                        let t = (c.g[V0][0] as i64) >= 0;
                        c.g[A0][0] = 0x490000;
                        if t {
                            break 'b_3e531c;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x003e531c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5e20));
                            Callee {
                                addr: 0x003e6070,
                                f: ee::s14::EA_DebugMessage,
                            }
                            .call(c, 0x003e531c)?;
                            break 'b_3e531c;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003e5324);
                    call_ptr(c, 0x00423cc0, 0x003e5324)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_3e5380;
                    } else {
                        break 'b_3e532c;
                    }
                }
            }
            'b_3e5360: {
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let t = c.g[V1][0] == 0u64;

                if t {
                    break 'b_3e5360;
                } else {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x003e5344);
                    call_ptr(c, 0x00423c90, 0x003e5344)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e5350);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x00423cb0, 0x003e5350)?;
                    c.g[RA][0] = sx(0x003e5358);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00424ce8, 0x003e5358)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    if t {
                        break 'b_3e5380;
                    } else {
                        break 'b_3e5360;
                    }
                }
            }
            c.cycles += 8;
            c.g[RA][0] = sx(0x003e5368);
            call_ptr(c, 0x00423c90, 0x003e5368)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x003e5374);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x00423ca0, 0x003e5374)?;
            c.g[RA][0] = sx(0x003e537c);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x00423c50, 0x003e537c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_3e5380;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::lib::eafile::tests::ea_install_fakes;
    pub(crate) use crate::game::lib::eafile::tests::ea_set_alloc_rets;
    pub(crate) use crate::game::lib::eafile::tests::put_ascii;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_ENQ;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_NEXT;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::events::driven;
    use crate::difftest::fakes::covered;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    pub(crate) use crate::game::lib::eafile::tests::stream_block;
    pub(crate) use crate::game::lib::eafile::tests::zero_block;
    pub(crate) use crate::game::lib::eafile::tests::EAF_REQ;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_QCLOSE;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_VALID;

    /// `EATimer_ServiceCallbacks` over sixteen timers: empty, not yet due,
    /// already running, and due ones (fake callbacks, some of which unwind).
    #[test]
    fn ea_timer_service_callbacks_matches() {
        check(
            ee::s14::f_003e5928__generated,
            ea_timer_service_callbacks,
            150,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x3e_5928, 16);
                let now = r.next_u32() % 1000;
                c.bus.write32(EA_TIMER_NOW, now);
                for i in 0..16u32 {
                    let t = EA_TIMERS + 0x10 * i;
                    let kind = r.next_u32() % 4;
                    c.bus
                        .write32(t, if kind == 0 { 0 } else { FAKE + 0x10 * i });
                    c.bus.write32(t + 4, r.next_u32() % 100);
                    c.bus
                        .write32(t + 8, now.wrapping_add(r.next_u32() % 40).wrapping_sub(20));
                    c.bus.write32(t + 0xc, (kind == 1) as u32);
                }
                c.g[A0][0] = r.next_u32() as u64;
                c.pc = 0x003e_5928;
            },
        );
    }

    use crate::difftest::fakes::LOG_RESULTS;

    /// A BIG archive at DATA: the header kind (unknown, `C0FB`, `BIGF`,
    /// `BIG0`..`BIG:`), 1-5 entries with random offset/size bytes and
    /// letter names, an optional `L123` footer; the lookup by an existing
    /// name (exact or other case), a missing name, or by index (null name),
    /// with null or real out pointers.
    #[test]
    fn ea_file_big_find_entry_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        check(
            ee::s14::f_003e2768__generated,
            ea_file_big_find_entry,
            300,
            &|c, seed| {
                let mut r = Rng(0x0b16_f11e ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.pc = 0x003e_2768;
                // EAFile_StrICmp's ctype table: upper-case letters flagged.
                for b in b'A'..=b'Z' {
                    c.bus.write8(0x0049_9c81 + b as u32, 1);
                }
                let big = DATA;
                let kind = seed % 15;
                let (start, off_w, size_w): (u32, i32, i32) = match kind {
                    0 => {
                        c.bus.write32(big, 0x7856_3412);
                        (big + 0x10, 4, 4)
                    }
                    1 => {
                        c.bus.write8(big, 0xc0);
                        c.bus.write8(big + 1, 0xfb);
                        (big + 6, 3, 3)
                    }
                    2 | 14 => {
                        for (i, b) in b"BIGF".iter().enumerate() {
                            c.bus.write8(big + i as u32, *b);
                        }
                        (big + 0x10, 4, 4)
                    }
                    _ => {
                        let digit = (kind - 3) as i32;
                        for (i, b) in b"BIG".iter().enumerate() {
                            c.bus.write8(big + i as u32, *b);
                        }
                        c.bus.write8(big + 3, b'0' + digit as u8);
                        (big + 0x10, digit, digit.min(4))
                    }
                };
                // The directory.
                let count = 1 + r.next_u32() % 5;
                let mut names: Vec<Vec<u8>> = Vec::new();
                let mut at = start;
                for _ in 0..count {
                    for _ in 0..(off_w + size_w) {
                        c.bus.write8(at, r.next_u32() as u8);
                        at += 1;
                    }
                    let len = 1 + r.next_u32() % 8;
                    let mut name = Vec::new();
                    for _ in 0..len {
                        let ch = b'a' + (r.next_u32() % 26) as u8;
                        let ch = if r.next_u32() % 2 == 0 {
                            ch.to_ascii_uppercase()
                        } else {
                            ch
                        };
                        name.push(ch);
                        c.bus.write8(at, ch);
                        at += 1;
                    }
                    c.bus.write8(at, 0);
                    at += 1;
                    names.push(name);
                }
                if (seed / 15) % 2 == 0 {
                    for (i, b) in b"L123".iter().enumerate() {
                        c.bus.write8(at + i as u32, *b);
                    }
                    c.bus.write32(at + 4, r.next_u32());
                    at += 8;
                }
                let header = at - big;
                if kind == 1 {
                    c.bus.write8(big + 2, ((header - 4) >> 8) as u8);
                    c.bus.write8(big + 3, (header - 4) as u8);
                } else if kind != 0 {
                    c.bus.write32(big + 0xc, header.swap_bytes());
                }
                // The lookup.
                let name_at = DATA + 0x800;
                c.g[A0][0] = sx(big);
                c.g[A2][0] = sx(r.next_u32() % (count + 2));
                match (seed / 30) % 4 {
                    0 | 1 => {
                        let mut name = names[(r.next_u32() % count) as usize].clone();
                        if (seed / 30) % 4 == 1 {
                            for ch in name.iter_mut() {
                                *ch ^= 0x20;
                            }
                        }
                        for (i, b) in name.iter().enumerate() {
                            c.bus.write8(name_at + i as u32, *b);
                        }
                        c.bus.write8(name_at + name.len() as u32, 0);
                        c.g[A1][0] = sx(name_at);
                    }
                    2 => {
                        for (i, b) in b"q9\0".iter().enumerate() {
                            c.bus.write8(name_at + i as u32, *b);
                        }
                        c.g[A1][0] = sx(name_at);
                    }
                    _ => c.g[A1][0] = 0,
                }
                let out_offset = DATA + 0x900;
                let out_size = DATA + 0x904;
                c.bus.write32(out_offset, r.next_u32());
                c.bus.write32(out_size, r.next_u32());
                c.g[A3][0] = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    sx(out_offset)
                };
                c.g[T0][0] = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    sx(out_size)
                };
            },
        );
    }

    /// `EAFile_FreeRequest`: the real mutex lock/unlock (thread id from a
    /// fake GetThreadId: the owner, so the fast path, or another thread, so
    /// the semaphore path through fakes) and the real memset on a random
    /// request record.
    #[test]
    fn ea_file_free_request_matches() {
        covered(
            ee::s14::f_003de7b0__generated,
            ea_file_free_request,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_e7b0, 4);
                c.pc = 0x003d_e7b0;
                let owner = 0x40 + seed % 3;
                c.bus.write32(EAFILE_REQUEST_MUTEX + 4, owner);
                c.bus.write32(EAFILE_REQUEST_MUTEX + 8, 1 + seed % 2);
                c.bus.write32(EAFILE_REQUEST_MUTEX, r.next_u32() & 0xff);
                c.bus.write32(LOG_RESULTS, 0x41);
                c.bus.write32(LOG_RESULTS + 4, 0x41);
                c.bus.write32(LOG_RESULTS + 8, 0x41);
                c.bus.write32(LOG_RESULTS + 12, 0x41);
                let req = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                for i in 0..16 {
                    c.bus.write32(req - 4 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(req);
            },
        );
    }

    // ------------------------------------------------------- StreamFile_OnClose

    /// A real EAFile request pool, handle pool, mutex and service queue on
    /// their short paths, a stream whose old request releases cleanly
    /// (state 0 in the request table) and a NUL-terminated name.
    #[test]
    fn stream_file_on_close_matches() {
        const POOL: u32 = 0x0051_9ab0;
        const MUTEX: u32 = POOL + 0x14;
        const SLOTS: u32 = DATA + 0x1000;
        const HANDLES: u32 = DATA + 0x4000;
        const QUEUED: u32 = DATA + 0x8000;
        const FILE: u32 = DATA + 0x1_0000;
        const NAME: u32 = FILE + SFOC_NAME;
        // The request hash table (0x519ad8): zeroed entries mean "no state",
        // so the released request has nothing to free first.
        const EA_TIMERS: u32 = DATA + 0x2_0000;
        const OLD_REQUEST: u32 = DATA + 0x3_0000;
        const HANDLE_COUNT: u32 = POOL + 0x4;
        const HANDLE_SLOTS: u32 = POOL + 0x2c;
        check(
            ee::s14::f_003e0118__generated,
            stream_file_on_close,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_0118, 0);
                c.pc = 0x003e_0118;
                // The request pool: 1..6 slots, the first free one anywhere.
                let count = 1 + r.next_u32() % 6;
                c.bus.write32(POOL, count);
                c.bus.write32(POOL + 0x28, SLOTS);
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
                // The stream file: its old request, priority and name.
                // 0x519ad8 is the request table the EAFile release path
                // indexes by the request pointer's top byte (0 here).
                c.bus.write32(0x0051_9ad8, EA_TIMERS);
                for i in 0..0x10u32 {
                    c.bus.write32(OLD_REQUEST + 4 * i, 0);
                }
                c.bus.write32(FILE + SFOC_REQUEST, OLD_REQUEST);
                c.bus.write32(FILE + SFOC_PRIORITY, r.next_u32() & 0x7f);
                let len = r.next_u32() % 0x40;
                for i in 0..len {
                    c.bus.write8(NAME + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(NAME + len, 0);
                c.g[A2][0] = sx(FILE);
            },
        );
    }

    use crate::difftest::fill;

    pub(crate) use crate::game::sound::soundsys::tests::OC371_SEQ;

    pub(crate) use crate::game::sound::soundsys::tests::oc371_machine;
    pub(crate) use crate::game::sound::soundsys::tests::oc371_script;

    pub(crate) use crate::game::sound::soundsys::tests::oc371_install_fakes;

    /// Script a sequence of `v0` for calls to `target` (the last value
    /// repeats; at most 8; two sequences).
    pub(crate) fn oc371_sequence(c: &mut Cpu, target: u32, values: &[u32]) {
        let mut seq = OC371_SEQ;
        if c.bus.read32(seq) != 0 && c.bus.read32(seq) != target {
            seq += 0x40;
        }
        c.bus.write32(seq, target);
        c.bus.write32(seq + 4, 0);
        c.bus.write32(seq + 8, values.len().min(8) as u32);
        for (i, v) in values.iter().take(8).enumerate() {
            c.bus.write32(seq + 0x10 + 4 * i as u32, *v);
        }
    }

    /// Markers at the read pointer (wrap, skip, none, empty ring); 0-2
    /// finished requests freed (none, in use, overlapping); room before
    /// the read pointer, at the end, or after moving the pending data
    /// (aligned or not, memory source or not, fitting or not); filling from
    /// memory (clipped at the source's end or not) or queueing a file read
    /// (queued or refused). The real `EA_MemCopy` runs; the other callees
    /// are logged fakes (one may unwind).
    #[test]
    fn streamfile_issue_read_matches() {
        oc371_install_fakes();
        covered(
            ee::s14::f_003e03d0__generated,
            streamfile_issue_read,
            500,
            &|c, seed| {
                let mut r = oc371_machine(c, seed, 0x003e_03d0, 6);
                c.pc = 0x003e_03d0;
                let sf = DATA;
                let ring = DATA + 0x1_0000;
                let end = ring + 0x1000;
                c.g[A0][0] = sx(sf);
                c.g[A1][0] = r.next_u32() as u64 & 0xff;
                fill(c, &mut r, sf, 0x60);
                let start = ring + (r.next_u32() % 4) * 0x10;
                c.bus.write32(sf + 0x2c, ring);
                c.bus.write32(sf + 0x30, start);
                c.bus.write32(sf + 0x34, end);
                // Write pointer, pending data before it, read pointer anywhere.
                let wr = ring + 0x100 + (r.next_u32() % 0xe00 & !0xf);
                let used = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 0x100
                };
                let pend = wr - used;
                let rd = match r.next_u32() % 4 {
                    0 => pend,
                    1 => wr + 0x10 + (r.next_u32() % 0x200 & !0xf),
                    _ => start + 0x10 + (r.next_u32() % 0xc0 & !0xf),
                };
                c.bus.write32(sf + 0x50, rd);
                c.bus.write32(sf + 0x54, pend);
                c.bus.write32(sf + 0x58, wr);
                c.bus.write32(start, 0x1234);
                if rd != pend {
                    match r.next_u32() % 3 {
                        0 => c.bus.write32(rd, u32::MAX),
                        1 => {
                            c.bus.write32(rd, -2i32 as u32);
                            c.bus.write32(rd + 4, 0x20);
                            c.bus.write32(
                                rd + 0x20,
                                if r.next_u32() % 2 == 0 { u32::MAX } else { 7 },
                            );
                        }
                        _ => c.bus.write32(rd, 5),
                    }
                }
                c.bus
                    .write32(pend, if r.next_u32() % 2 == 0 { u32::MAX } else { 9 });
                // The request list.
                c.bus.write32(sf + 0x5c, DATA + 0x200);
                let req = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    DATA + 0x300
                };
                c.bus.write32(DATA + 0x200 + 0xc, req);
                c.bus
                    .write32(DATA + 0x300 + 4, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(DATA + 0x300 + 0x120, r.next_u32());
                let frees = r.next_u32() % 3;
                let seq: Vec<u32> = (0..frees).map(|_| 0).chain([1]).collect();
                oc371_sequence(c, 0x003d_fc08, &seq);
                // The source.
                let src = DATA + 0x400;
                c.bus.write32(sf + 0x60, src);
                c.bus
                    .write32(src + 0x10, if r.next_u32() % 2 == 0 { 1 } else { 2 });
                c.bus.write32(src + 0x114, DATA + 0x2_0000);
                let pos = r.next_u32() % 0x800;
                c.bus.write32(sf + 0x170, pos);
                c.bus.write32(src + 0x118, pos + r.next_u32() % 0x1000);
                c.bus.write32(sf + 0x17c, r.next_u32() % 0x900);
                oc371_script(
                    c,
                    0x003d_d648,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x500
                    },
                );
            },
        );
    }

    #[test]
    fn ea_thread_yield_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s14::f_003e5398__generated,
            ea_thread_yield,
            300,
            &|c, seed| {
                // Syscalls (thread status, alarms, sleep) are fakes.
                let mut r = machine(c, seed, 0x5398, 3);
                c.pc = 0x003e_5398;
                c.g[A0][0] = match seed % 4 {
                    0 => 0,
                    1 => (r.next_u32() % 100) as u64,
                    2 => sx(r.next_u32() % 4 + 0xffff_fffc),
                    _ => r.next_u32() as u64,
                };
                c.bus.write32(
                    EATHREAD_TICK_RATE,
                    [60, 1000, 0, 7][(seed / 4 % 4) as usize],
                );
                c.bus.write32(EATHREAD_IN_INTERRUPT, (seed / 16) % 2);
                c.bus.write32(0x0045_0da0, r.next_u32() % 2000);
            },
        );
    }

    use crate::difftest::check_lohi;
    use crate::difftest::fakes;

    pub(crate) use crate::game::ui::uitext::tests::OC445_CTRL;
    pub(crate) use crate::game::ui::uitext::tests::OC445_MAGIC;
    pub(crate) use crate::game::ui::uitext::tests::OC445_NEW_COUNT;
    pub(crate) use crate::game::ui::uitext::tests::OC445_RESULT_NEXT;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// What `UIList_GetAt` returns.
    pub(crate) const OC445_GET_AT: u32 = OC445_CTRL + 0xc;

    /// `UIScreen_CreateObject` results (16 words).
    pub(crate) const OC445_RESULTS: u32 = OC445_CTRL + 0x10;

    /// Where the fake `operator new` puts its blocks (0x1000 apart).
    pub(crate) const OC445_NEW_AREA: u32 = DATA + 0x8_0000;

    pub(crate) const OC445_K_PLAIN: u32 = 0;

    pub(crate) const OC445_K_NEW: u32 = 1;

    pub(crate) const OC445_K_RESULT: u32 = 2;

    pub(crate) const OC445_K_GET_AT: u32 = 3;

    /// The logging fake, then a result by kind.
    pub(crate) fn oc445_fake(c: &mut Cpu, addr: u32, kind: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match kind {
            OC445_K_NEW => {
                let n = c.bus.read32(OC445_NEW_COUNT);
                c.bus.write32(OC445_NEW_COUNT, n + 1);
                c.g[V0][0] = sx(OC445_NEW_AREA + 0x1000 * (n & 7));
            }
            OC445_K_RESULT => {
                let i = c.bus.read32(OC445_RESULT_NEXT);
                c.bus.write32(OC445_RESULT_NEXT, i + 1);
                let r = c.bus.read32(OC445_RESULTS + 4 * (i & 0xf));
                c.g[V0][0] = sx(r);
            }
            OC445_K_GET_AT => {
                let r = c.bus.read32(OC445_GET_AT);
                c.g[V0][0] = sx(r);
            }
            _ => {}
        }
    }

    macro_rules! oc445_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $kind:expr;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC445_CTRL) != OC445_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    oc445_fake(c, $addr, $kind)
                }
            )*
            pub(crate) fn oc445_install_fakes() {
                $(
                    fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev);
                )*
            }
        };
    }

    oc445_fakes! {
        oc445_f_sprintf OC445_P_SPRINTF 0x0041_7828 s15 OVERRIDE_00417828 f_00417828__generated OC445_K_PLAIN;
        oc445_f_new OC445_P_NEW 0x0031_7e30 s10 OVERRIDE_00317E30 f_00317e30__generated OC445_K_NEW;
        oc445_f_memset OC445_P_MEMSET 0x0041_6210 s15 OVERRIDE_00416210 f_00416210__generated OC445_K_PLAIN;
        oc445_f_free OC445_P_FREE 0x0025_23a8 s06 OVERRIDE_002523A8 f_002523a8__generated OC445_K_PLAIN;
        oc445_f_fill OC445_P_FILL 0x003e_6448 s14 OVERRIDE_003E6448 f_003e6448__generated OC445_K_PLAIN;
        oc445_f_free_strings OC445_P_FREE_STRINGS 0x003a_0b38 s13 OVERRIDE_003A0B38 f_003a0b38__generated OC445_K_PLAIN;
        oc445_f_create_object OC445_P_CREATE_OBJECT 0x0039_d078 s13 OVERRIDE_0039D078 f_0039d078__generated OC445_K_RESULT;
        oc445_f_push_back OC445_P_PUSH_BACK 0x0039_7910 s13 OVERRIDE_00397910 f_00397910__generated OC445_K_PLAIN;
        oc445_f_get_at OC445_P_GET_AT 0x0039_7870 s13 OVERRIDE_00397870 f_00397870__generated OC445_K_GET_AT;
    }

    /// `EATimer_Init`: already started, every prescaler, hz 0, and the
    /// measured-video-mode path (only with hz values whose wait count is
    /// below the compare, so the interrupt-count spin loop is not entered).
    #[test]
    fn eatimer_init_matches() {
        check_lohi(
            ee::s14::f_003e4af0__generated,
            eatimer_init,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e4af0, 0);
                // hz values; the last two give a zero compare value (the wait
                // loop is skipped for them), the others need a known video mode.
                let hzs: [i32; 14] = [
                    0,
                    1,
                    50,
                    100,
                    195,
                    196,
                    197,
                    3000,
                    3149,
                    3150,
                    5000,
                    1_000_000,
                    i32::MIN,
                    0x7fff_ffff,
                ];
                let hz = hzs[(seed % 14) as usize];
                let zero_compare = hz == i32::MIN || hz == 0x7fff_ffff;
                c.bus
                    .write32(EA_TIMER_READY, if seed % 5 == 0 { 0x1234 } else { 0 });
                c.bus.write32(EA_TIMER_TICKS_PER_IRQ, r.next_u32());
                c.bus.write32(
                    EA_TIMER_VIDEO_MODE,
                    if zero_compare && seed % 2 == 0 {
                        0
                    } else {
                        0x3d09
                    },
                );
                c.bus.write32(EA_TIMER_IRQ_COUNT, r.next_u32() >> 1);
                c.g[A0][0] = sx(hz as u32);
                c.pc = 0x003e_4af0;
            },
        );
    }

    /// `EATimer_AddCallback`: matching entries, free entries behind the
    /// skip count, a full table and the -1/0 mode conversions.
    #[test]
    fn eatimer_add_callback_matches() {
        check(
            ee::s14::f_003e57f8__generated,
            eatimer_add_callback,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e57f8, 0);
                let fns: [u32; 4] = [0, 0x0012_3450, 0x0012_3460, 0x0012_3470];
                for i in 0..16u32 {
                    let f = match r.next_u32() % 4 {
                        0 | 1 => 0,
                        k => fns[k as usize],
                    };
                    c.bus.write32(EA_TIMER_CALLBACKS + 0x10 * i, f);
                    c.bus
                        .write32(EA_TIMER_CALLBACKS + 0x10 * i + 4, r.next_u32());
                    c.bus
                        .write32(EA_TIMER_CALLBACKS + 0x10 * i + 8, r.next_u32());
                    c.bus
                        .write32(EA_TIMER_CALLBACKS + 0x10 * i + 0xc, r.next_u32());
                }
                if seed % 11 == 0 {
                    // A full table.
                    for i in 0..16u32 {
                        c.bus
                            .write32(EA_TIMER_CALLBACKS + 0x10 * i, 0x0012_3480 + 4 * i);
                    }
                }
                c.bus.write32(
                    EA_TIMER_ADDING,
                    [0, 0, 1, 2, 5, 20][(r.next_u32() % 6) as usize],
                );
                c.bus.write32(EA_TIMER_TICK, r.next_u32());
                c.g[A0][0] = sx(fns[(r.next_u32() % 4) as usize]);
                c.g[A1][0] = match r.next_u32() % 4 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 1,
                    _ => sx(r.next_u32() % 7),
                };
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    driven!(hand_stream_file_cancel_request, stream_file_cancel_request);

    /// Invalid streams; cursor pairs across the wrap point.
    #[test]
    fn stream_file_get_buffer_size_matches() {
        check(
            gen_stream_file_get_buffer_size,
            hand_stream_file_get_buffer_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1580, 0);
                ea_install_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x2c, 0x1000 + (seed / 3 % 7) * 0x100);
                c.bus
                    .write32(stream + 0x34, 0x1000 + (seed / 9 % 7) * 0x100);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_1580;
            },
        );
    }

    /// Invalid streams; empty and stocked level words.
    #[test]
    fn stream_file_get_buffered_bytes_matches() {
        check(
            gen_stream_file_get_buffered_bytes,
            hand_stream_file_get_buffered_bytes,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_15b8, 0);
                ea_install_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x4c, [0u32, 5, 0x100][seed as usize / 3 % 3]);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_15b8;
            },
        );
    }

    driven!(hand_stream_file_get_read_state, stream_file_get_read_state);

    /// Invalid streams; zero and nonzero state words.
    #[test]
    fn stream_file_get_read_state_matches() {
        check(
            gen_stream_file_get_read_state,
            hand_stream_file_get_read_state,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_14f8, 0);
                ea_install_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus.write32(stream + 0x38, 0x1234 + seed % 5);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_14f8;
            },
        );
    }

    /// Invalid streams; missing and live requests.
    #[test]
    fn stream_file_get_request_status_matches() {
        check(
            gen_stream_file_get_request_status,
            hand_stream_file_get_request_status,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_16b0, 0);
                ea_install_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream, stream);
                let table = DATA + 0x700;
                c.bus
                    .write32(stream + 0x18, [0u32, 1, 4][seed as usize / 4 % 3]);
                c.bus.write32(stream + 0x14, table);
                c.bus.write32(table, (seed / 12 % 200) as u32);
                c.bus.write32(table + 4, 0x4321 + seed % 5);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = (seed * 5 % 11) as u64;
                c.pc = 0x003e_16b0;
            },
        );
    }

    /// Past, present and future deadlines.
    #[test]
    fn ea_timer_deadline_passed_matches() {
        check(
            gen_ea_timer_deadline_passed,
            hand_ea_timer_deadline_passed,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f80, 0);
                c.bus.write32(
                    0x0045_0de8,
                    [0u32, 0x7fff_ffff, 0xffff_ffff][seed as usize % 3],
                );
                c.pc = 0x003e_4f80;
            },
        );
    }

    /// Deadline one tick in the future (ticks read 0 from [0x450dcc]):
    /// elapsed == -1, so orig (-1 < -1) is 0 while the `<=` mutant is 1.
    /// Kills 3915.
    #[test]
    fn ea_timer_deadline_passed_future_matches() {
        check(
            gen_ea_timer_deadline_passed,
            hand_ea_timer_deadline_passed,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f80, 0);
                c.bus.write32(0x0045_0de8, 1);
                c.pc = 0x003e_4f80;
            },
        );
    }

    /// Pure cached-word load.
    #[test]
    fn ea_timer_get_tick_rate_matches() {
        check(
            gen_ea_timer_get_tick_rate,
            hand_ea_timer_get_tick_rate,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5008, 0);
                c.bus.write32(0x0045_0dc8, 100 + (seed % 3));
                c.pc = 0x003e_5008;
            },
        );
    }

    driven!(hand_ea_timer_set_deadline, ea_timer_set_deadline);

    /// Zero and nonzero deltas.
    #[test]
    fn ea_timer_set_deadline_matches() {
        check(
            gen_ea_timer_set_deadline,
            hand_ea_timer_set_deadline,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f08, 0);
                c.g[A0][0] = [0u32, 1, 0x1e0, 0x10000][seed as usize % 4] as u64;
                c.bus.write32(0x0045_0de8, 0);
                c.pc = 0x003e_4f08;
            },
        );
    }

    driven!(gen_ea_thread_destroy, ee::s14::f_003e51a0__generated);

    driven!(hand_ea_thread_destroy, ea_thread_destroy);

    /// Self (-1) and other threads; live and dead main slots.
    #[test]
    fn ea_thread_destroy_matches() {
        check(
            gen_ea_thread_destroy,
            hand_ea_thread_destroy,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_51a0, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = if seed / 2 % 2 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    sx(DATA + 0x500)
                };
                c.pc = 0x003e_51a0;
            },
        );
    }

    driven!(gen_ea_thread_destroy_thunk, ee::s14::f_003e5268__generated);

    driven!(hand_ea_thread_destroy_thunk, ea_thread_destroy_thunk);

    /// Exiting-thread destroy through the syscall fakes.
    #[test]
    fn ea_thread_destroy_thunk_matches() {
        check(
            gen_ea_thread_destroy_thunk,
            hand_ea_thread_destroy_thunk,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5268, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(0x0045_0de4, 0);
                c.g[A0][0] = sx(DATA + 0x500);
                c.pc = 0x003e_5268;
            },
        );
    }

    driven!(gen_ea_thread_exit, ee::s14::f_003e5678__generated);

    driven!(hand_ea_thread_exit, ea_thread_exit);

    /// Exit through the syscall fake.
    #[test]
    fn ea_thread_exit_matches() {
        check(gen_ea_thread_exit, hand_ea_thread_exit, 48, &|c, seed| {
            let _r = machine(c, seed, 0x3e_5678, 0);
            c.pc = 0x003e_5678;
        });
    }

    driven!(gen_ea_thread_get_priority, ee::s14::f_003e5498__generated);

    driven!(hand_ea_thread_get_priority, ea_thread_get_priority);

    /// Null, self and explicit threads; live and dead main slots.
    #[test]
    fn ea_thread_get_priority_matches() {
        check(
            gen_ea_thread_get_priority,
            hand_ea_thread_get_priority,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5498, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x14);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = match seed / 2 % 3 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    _ => sx(DATA + 0x500),
                };
                c.pc = 0x003e_5498;
            },
        );
    }

    driven!(gen_ea_thread_init_main, ee::s14::f_003e5018__generated);

    driven!(hand_ea_thread_init_main, ea_thread_init_main);

    /// Main-thread tag through the syscall fakes.
    #[test]
    fn ea_thread_init_main_matches() {
        check(
            gen_ea_thread_init_main,
            hand_ea_thread_init_main,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5018, 0);
                c.pc = 0x003e_5018;
            },
        );
    }

    driven!(gen_ea_thread_is_current, ee::s14::f_003e5440__generated);

    driven!(hand_ea_thread_is_current, ea_thread_is_current);

    /// Null, self (-1) and other threads.
    #[test]
    fn ea_thread_is_current_matches() {
        check(
            gen_ea_thread_is_current,
            hand_ea_thread_is_current,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5440, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x9abc);
                c.g[A0][0] = match seed % 3 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    _ => sx(DATA + 0x500),
                };
                c.pc = 0x003e_5440;
            },
        );
    }

    /// Forced thread id 3 with [S0+4]=2: X = 3^2 = 1. Kills 4307 (`^`: orig
    /// 1 vs mutant 0, since A0&C = 3&2 != 0) and both 4308 `<=` rows (orig
    /// (1<1)=0 vs mutant (1<=1)=1).
    #[test]
    fn ea_thread_is_current_exact_matches() {
        check(
            gen_ea_thread_is_current,
            hand_ea_thread_is_current,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5440, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 2);
                // Force GET_THREAD_ID (first fake call) to return 3.
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, 3);
                c.g[A0][0] = sx(DATA + 0x500);
                c.pc = 0x003e_5440;
            },
        );
    }

    /// Forced thread id equal to the cached id (cur==cand!=0): X = 0.
    /// Kills 4307 (`^ with |`: orig (0<1)=1 via X=0 vs mutant (C<1)=0
    /// for C!=0; the `<=` rows need X==1 instead — separate fixture).
    #[test]
    fn ea_thread_is_current_equal_matches() {
        check(
            gen_ea_thread_is_current,
            hand_ea_thread_is_current,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5440, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                // Force GET_THREAD_ID (first fake call) to 0x1234.
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS, 0x1234);
                c.g[A0][0] = sx(DATA + 0x500);
                c.pc = 0x003e_5440;
            },
        );
    }

    driven!(gen_ea_thread_is_finished, ee::s14::f_003e5580__generated);

    driven!(hand_ea_thread_is_finished, ea_thread_is_finished);

    /// Dead (0x10), gone (0) and live status words; both main slots.
    #[test]
    fn ea_thread_is_finished_matches() {
        check(
            gen_ea_thread_is_finished,
            hand_ea_thread_is_finished,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5580, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = sx(DATA + 0x500);
                // Status word the ReferThreadStatus fake leaves behind.
                let sp = c.g[SP][0] as u32;
                c.bus
                    .write32(sp - 0x40, [0x10u32, 0, 0x20][seed as usize / 2 % 3]);
                c.pc = 0x003e_5580;
            },
        );
    }

    driven!(gen_ea_thread_set_priority, ee::s14::f_003e5508__generated);

    driven!(hand_ea_thread_set_priority, ea_thread_set_priority);

    /// Null, self and explicit threads; live and dead main slots.
    #[test]
    fn ea_thread_set_priority_matches() {
        check(
            gen_ea_thread_set_priority,
            hand_ea_thread_set_priority,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5508, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x14);
                c.bus.write32(0x0045_0de4, (seed % 2) as u32);
                c.g[A0][0] = match seed / 2 % 3 {
                    0 => 0,
                    1 => 0xffff_ffff_ffff_ffff,
                    _ => sx(DATA + 0x500),
                };
                c.g[A1][0] = [0u32, 1, 0x63][seed as usize / 6 % 3] as u64;
                c.pc = 0x003e_5508;
            },
        );
    }

    driven!(gen_ea_thread_shutdown, ee::s14::f_003e5068__generated);

    driven!(hand_ea_thread_shutdown, ea_thread_shutdown);

    /// Mutex destroy plus slot clear.
    #[test]
    fn ea_thread_shutdown_matches() {
        check(
            gen_ea_thread_shutdown,
            hand_ea_thread_shutdown,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5068, 0);
                c.pc = 0x003e_5068;
            },
        );
    }

    driven!(gen_ea_thread_wakeup_alarm, ee::s14::f_003e5290__generated);

    driven!(hand_ea_thread_wakeup_alarm, ea_thread_wakeup_alarm);

    /// Alarm wake through the syscall fake.
    #[test]
    fn ea_thread_wakeup_alarm_matches() {
        check(
            gen_ea_thread_wakeup_alarm,
            hand_ea_thread_wakeup_alarm,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5290, 0);
                c.g[A2][0] = [0u32, 1, 0x1234][seed as usize % 3] as u64;
                c.pc = 0x003e_5290;
            },
        );
    }

    driven!(gen_ea_timer_clear_callbacks, ee::s14::f_003e57d0__generated);

    driven!(hand_ea_timer_clear_callbacks, ea_timer_clear_callbacks);

    /// Planted table entries are all cleared.
    #[test]
    fn ea_timer_clear_callbacks_matches() {
        check(
            gen_ea_timer_clear_callbacks,
            hand_ea_timer_clear_callbacks,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_57d0, 0);
                for i in 0..64 {
                    c.bus.write32(EA_TIMERS + 4 * i, 0xdead_0000 + seed + i);
                }
                c.pc = 0x003e_57d0;
            },
        );
    }

    driven!(gen_ea_timer_get_delta, ee::s14::f_003e4ea8__generated);

    driven!(hand_ea_timer_get_delta, ea_timer_get_delta);

    /// Stale and fresh last-tick cells.
    #[test]
    fn ea_timer_get_delta_matches() {
        check(
            gen_ea_timer_get_delta,
            hand_ea_timer_get_delta,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4ea8, 0);
                c.bus
                    .write32(0x0045_0dec, [0u32, 1, 0x1000][seed as usize % 3]);
                c.pc = 0x003e_4ea8;
            },
        );
    }

    driven!(gen_ea_timer_remove_callback, ee::s14::f_003e58c8__generated);

    driven!(hand_ea_timer_remove_callback, ea_timer_remove_callback);

    driven!(gen_ea_timer_set_video_mode, ee::s14::f_003e6690__generated);

    driven!(hand_ea_timer_set_video_mode, ea_timer_set_video_mode);

    /// All five mode arms (PAL/NTSC/60Hz/fallthrough).
    #[test]
    fn ea_timer_set_video_mode_matches() {
        check(
            gen_ea_timer_set_video_mode,
            hand_ea_timer_set_video_mode,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_6690, 0);
                c.bus.write32(0x0045_0da0, 0);
                c.g[A0][0] = (seed % 5) as u64;
                c.pc = 0x003e_6690;
            },
        );
    }

    driven!(gen_ea_timer_shutdown, ee::s14::f_003e4d68__generated);

    driven!(hand_ea_timer_shutdown, ea_timer_shutdown);

    /// Set and clear interrupt cells both converge.
    #[test]
    fn ea_timer_shutdown_matches() {
        check(
            gen_ea_timer_shutdown,
            hand_ea_timer_shutdown,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4d68, 0);
                c.bus.write32(0x0045_0ddc, (seed % 2) as u32);
                c.pc = 0x003e_4d68;
            },
        );
    }

    /// Live timer (mem word != 0) with RAM[0] preset nonzero: orig stores
    /// 0 to the HW word [0x10000810] (ignored), the `&` mutant stores 0 to
    /// RAM[0] instead. Kills 3773 `&`.
    #[test]
    fn ea_timer_shutdown_live_ram0_matches() {
        check(
            gen_ea_timer_shutdown,
            hand_ea_timer_shutdown,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4d68, 0);
                c.bus.write32(0x0045_0ddc, 1);
                c.bus.write32(0, 0xA5A5_A5A5);
                c.pc = 0x003e_4d68;
            },
        );
    }

    driven!(hand_stream_file_calc_mem_size, stream_file_calc_mem_size);

    /// Channel counts and buffer sizes across the shift range.
    #[test]
    fn stream_file_calc_mem_size_matches() {
        check(
            gen_stream_file_calc_mem_size,
            hand_stream_file_calc_mem_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_06b0, 0);
                c.g[A0][0] = (seed % 17) as u64;
                c.g[A1][0] = (seed * 37 % 300) as u64;
                c.g[A2][0] = (seed / 8 % 8) as u64;
                c.pc = 0x003e_06b0;
            },
        );
    }

    driven!(hand_stream_file_get_channel_id, stream_file_get_channel_id);

    fn get_channel_id_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_0c58, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(stream, stream);
        c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
        c.bus
            .write32(stream + 0x4, [0u32, 0x42][seed as usize / 3 % 2]);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = 0;
        c.pc = 0x003e_0c58;
    }

    /// Invalid streams; present and absent channel ids.
    #[test]
    fn stream_file_get_channel_id_matches() {
        check_paths(
            gen_stream_file_get_channel_id,
            hand_stream_file_get_channel_id,
            48,
            &[17, 19],
            &|c, seed| get_channel_id_setup(c, seed),
        );
    }

    driven!(gen_stream_file_get_channel, ee::s14::f_003e0bf8__generated);

    driven!(hand_stream_file_get_channel, stream_file_get_channel);

    fn get_channel_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_0bf8, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(stream, stream);
        c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
        c.bus
            .write32(stream + 0x28, [0u32, 1, 4][seed as usize / 4 % 3]);
        c.bus
            .write32(stream + 0x24, DATA + 0x900 + 0x10 * (seed % 5));
        c.bus.write32(stream + 0x1c, 0x8100 + seed % 5);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = [0u32, 1, 4, 9][seed as usize / 16 % 4] as u64;
        c.pc = 0x003e_0bf8;
    }

    /// Invalid streams; empty and populated channel tables.
    #[test]
    fn stream_file_get_channel_matches() {
        check_paths(
            gen_stream_file_get_channel,
            hand_stream_file_get_channel,
            64,
            &[21, 23, 29, 30],
            &|c, seed| get_channel_setup(c, seed),
        );
    }

    driven!(hand_stream_file_lookup_request, stream_file_lookup_request);

    /// In-range and out-of-range handles, matching and mismatched words.
    #[test]
    fn stream_file_lookup_request_matches() {
        check(
            gen_stream_file_lookup_request,
            hand_stream_file_lookup_request,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_fdd0, 0);
                let base = DATA + 0x600;
                for i in 0..16u32 {
                    c.bus.write32(base + 4 * i, 0);
                }
                let count = [0u32, 1, 4, 300][seed as usize % 4];
                c.bus.write32(base + 0x18, count);
                c.bus.write32(base + 0x14, DATA + 0x700);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x700 + 4 * i, (seed / 4 + i) as u32);
                    c.bus.write32(DATA + 0x700 + 4 * i + 0x124, 0x1234 + i);
                }
                c.bus
                    .write32(DATA + 0x700 + 4 * (seed as u32 / 16 % 4), 0x1234 + seed % 4);
                c.g[A0][0] = base as u64;
                c.g[A1][0] = (seed * 13 % 300) as u64;
                c.pc = 0x003d_fdd0;
            },
        );
    }

    /// Matching nonzero entry word: A1 hits the entry key (entry =
    /// table + (A1&0xff)*0x124; the mult writes V0=LO as the index scale)
    /// and the status word is nonzero, so orig keeps the request pointer
    /// while the `==` mutant nulls it. Kills 1836.
    #[test]
    fn stream_file_lookup_request_hit_matches() {
        check(
            gen_stream_file_lookup_request,
            hand_stream_file_lookup_request,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d_fdd0, 0);
                let base = DATA + 0x600;
                for i in 0..16u32 {
                    c.bus.write32(base + 4 * i, 0);
                }
                c.bus.write32(base + 0x18, 4);
                c.bus.write32(base + 0x14, DATA + 0x700);
                // Entry for key index 2 (A1&0xff): key matches, status != 0.
                c.bus.write32(DATA + 0x700 + 2 * 0x124, 2);
                c.bus.write32(DATA + 0x700 + 2 * 0x124 + 4, 1);
                c.g[A0][0] = base as u64;
                c.g[A1][0] = 2;
                c.pc = 0x003d_fdd0;
            },
        );
    }

    driven!(hand_stream_file_set_low_water, stream_file_set_low_water);

    fn set_low_water_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_0b28, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(stream, stream);
        c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
        // [SP] stream's level words: +0x44 current, +0x4c starved.
        c.bus
            .write32(stream + 0x44, [0u32, 5, 10][seed as usize / 4 % 3]);
        c.bus
            .write32(stream + 0x4c, [0u32, 5, 10][seed as usize / 12 % 3]);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = [0u32, 5, 10][seed as usize / 36 % 3] as u64;
        c.pc = 0x003e_0b28;
    }

    /// Invalid streams; low-water marks above and below the starved and
    /// current levels (the re-arm path calls back into SetStarved).
    #[test]
    fn stream_file_set_low_water_matches() {
        check_paths(
            gen_stream_file_set_low_water,
            hand_stream_file_set_low_water,
            108,
            &[24, 33, 57, 61],
            &|c, seed| set_low_water_setup(c, seed),
        );
    }

    driven!(gen_stream_file_set_priority, ee::s14::f_003e0ad8__generated);

    driven!(hand_stream_file_set_priority, stream_file_set_priority);

    fn set_priority_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_0ad8, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(stream, stream);
        c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
        c.bus.write32(stream + 0x3c, 0x100 + seed % 3);
        c.bus.write32(stream + 0x40, 0x200 + seed % 3);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = (seed / 3 % 7) as u64;
        c.g[A2][0] = (seed / 21 % 3) as u64;
        c.pc = 0x003e_0ad8;
    }

    /// Invalid streams and stored priority pairs.
    #[test]
    fn stream_file_set_priority_matches() {
        check_paths(
            gen_stream_file_set_priority,
            hand_stream_file_set_priority,
            64,
            &[24, 29],
            &|c, seed| set_priority_setup(c, seed),
        );
    }

    driven!(gen_ea_timer_deadline_passed, ee::s14::f_003e4f80__generated);

    driven!(gen_ea_timer_get_tick_rate, ee::s14::f_003e5008__generated);

    driven!(gen_ea_timer_set_deadline, ee::s14::f_003e4f08__generated);

    driven!(
        gen_stream_file_calc_mem_size,
        ee::s14::f_003e06b0__generated
    );

    driven!(
        gen_stream_file_cancel_request,
        ee::s14::f_003e0e90__generated
    );

    driven!(
        gen_stream_file_get_buffer_size,
        ee::s14::f_003e1580__generated
    );

    driven!(
        gen_stream_file_get_buffered_bytes,
        ee::s14::f_003e15b8__generated
    );

    driven!(
        gen_stream_file_get_channel_id,
        ee::s14::f_003e0c58__generated
    );

    driven!(
        gen_stream_file_get_read_state,
        ee::s14::f_003e14f8__generated
    );

    driven!(
        gen_stream_file_get_request_status,
        ee::s14::f_003e16b0__generated
    );

    driven!(
        gen_stream_file_lookup_request,
        ee::s14::f_003dfdd0__generated
    );

    driven!(
        gen_stream_file_set_low_water,
        ee::s14::f_003e0b28__generated
    );

    driven!(hand_ea_timer_deadline_passed, ea_timer_deadline_passed);

    driven!(hand_ea_timer_get_tick_rate, ea_timer_get_tick_rate);

    driven!(
        hand_stream_file_get_buffer_size,
        stream_file_get_buffer_size
    );

    driven!(
        hand_stream_file_get_buffered_bytes,
        stream_file_get_buffered_bytes
    );

    driven!(
        hand_stream_file_get_request_status,
        stream_file_get_request_status
    );

    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::fake_syscall;
    use crate::game::lib::eafile::tests::ea_assert_entered;
    use crate::game::lib::eafile::tests::ea_counted;

    /// [`schedule`], with the due cycle spread over `span` cycles so the
    /// first unwind can land on any loop head of a longer function.
    fn sched(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32, span: u32) -> Events {
        let mode = schedule(c, r, seed, entry);
        if mode != Events::Off {
            c.next_event = c.cycles + (r.next_u32() % span) as u64;
        }
        mode
    }

    /// Plays the other thread for the wait loops: `[OTHER]` counts hooked
    /// calls and syscalls down and at zero stores `[OTHER + 8]` at
    /// `[OTHER + 4]`; every hooked call (a timer callback) also adds
    /// `[OTHER + 0xc]` to the timer ticks and to the timer's current tick,
    /// and returns `[OTHER + 0x10]`.
    const OTHER: u32 = DATA + 0x3_0000;

    fn other_thread(c: &mut Cpu, tick: bool) {
        let n = c.bus.read32(OTHER);
        if n > 0 {
            c.bus.write32(OTHER, n - 1);
            if n == 1 {
                let at = c.bus.read32(OTHER + 4);
                let v = c.bus.read32(OTHER + 8);
                c.bus.write32(at, v);
            }
        }
        let step = if tick { c.bus.read32(OTHER + 0xc) } else { 0 };
        for at in [EA_TIMER_TICKS, EA_TIMER_NOW] {
            let t = c.bus.read32(at);
            c.bus.write32(at, t.wrapping_add(step));
        }
    }

    fn other_call(c: &mut Cpu, target: u32) {
        other_thread(c, true);
        fake_call(c, target);
        c.g[V0][0] = c.bus.read32(OTHER + 0x10) as u64;
    }

    fn other_syscall(c: &mut Cpu, at: u32) {
        other_thread(c, false);
        fake_syscall(c, at);
    }

    /// Hook [`other_thread`] into the fake calls and syscalls: after `n`
    /// of them `[at] = v`; each adds `step` ticks.
    fn other_setup(c: &mut Cpu, n: u32, at: u32, v: u32, step: u32, ret: u32) {
        c.hooks.miss = other_call;
        c.hooks.syscall = other_syscall;
        for (i, w) in [n, at, v, step, ret].into_iter().enumerate() {
            c.bus.write32(OTHER + 4 * i as u32, w);
        }
    }

    /// A due timer callback at the fake address in slot 0 (period 0, so
    /// it is due again on every pass).
    fn fake_timer(c: &mut Cpu) {
        zero_block(c, EA_TIMERS, 64);
        c.bus.write32(EA_TIMERS, FAKE);
        c.bus.write32(EA_TIMERS + 4, 0);
        c.bus.write32(EA_TIMERS + 8, 0);
    }

    driven!(gen_stream_file_cancel_all, ee::s14::f_003e1110__generated);

    driven!(hand_stream_file_cancel_all, stream_file_cancel_all);

    ea_counted!(counted_stream_file_cancel_all, stream_file_cancel_all);

    driven!(
        hand_stream_file_cancel_all_ev,
        counted_stream_file_cancel_all
    );

    /// A stream for `StreamFile_CancelAll` at `DATA + 0x400`: a request
    /// list head (or none) whose status (+4) is 2 (CancelRequest marks it
    /// done after one pass) or out of range (3, 5), a drain chain of `0..=2` requests
    /// that `StreamFile_FreeRequest` walks to its end word, a buffer table
    /// of `0..=2` entries, and a cursor walk of `0..=2` entries from
    /// `+0x50` to `+0x54` (one of them `-1`, which jumps to `+0x30`).
    fn cancel_all_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_1110, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        zero_block(c, stream + 0x100, 0x80 / 4);
        c.bus.write32(EAF_RET_VALID, (r.next_u32() % 8 == 0) as u32);
        c.bus.write32(stream, stream);
        // Request table for the lookup CancelRequest makes: entry 1 is the
        // list head, its handle 1 and status 2 (marked 4 by the cancel).
        let table = DATA + 0x700;
        zero_block(c, table, 0x124 * 2 / 4);
        c.bus.write32(stream + 0x18, 2);
        c.bus.write32(stream + 0x14, table);
        let head = table + 0x124;
        c.bus.write32(head, 1);
        let head_mode = r.next_u32() % 4;
        c.bus.write32(head + 4, [0u32, 2, 5, 2][head_mode as usize]);
        c.bus
            .write32(stream + 0x64, if head_mode == 0 { 0 } else { head });
        // Status 3 on some seeds: just outside the `status - 1 < 2` loop
        // test (no cancel pass).
        if seed % 20 == 7 {
            c.bus.write32(head + 4, 3);
            c.bus.write32(stream + 0x64, head);
        }
        // Drain chain R0 -> R1 -> end.
        let end = DATA + 0xa00;
        zero_block(c, end, 8);
        let n = r.next_u32() % 3;
        let reqs = [DATA + 0xb00, DATA + 0xc00];
        for (i, &q) in reqs.iter().enumerate() {
            zero_block(c, q, 16);
            let next = if i as u32 + 1 >= n { end } else { reqs[i + 1] };
            c.bus.write32(q + 0xc, next);
        }
        c.bus
            .write32(stream + 0x5c, if n == 0 { end } else { reqs[0] });
        c.bus.write32(stream + 0x60, end);
        // Buffer table.
        c.bus.write32(stream + 0x28, r.next_u32() % 3);
        c.bus.write32(stream + 0x24, DATA + 0x900);
        zero_block(c, DATA + 0x900, 12);
        // Negative levels: CancelRequest's buffer scan skips them; CancelAll
        // clears them.
        c.bus.write32(DATA + 0x900 + 8, 0xffff_ff77);
        c.bus.write32(DATA + 0x910 + 8, 0xffff_ff78);
        c.bus.write32(stream + 0x4c, r.next_u32() % 0x100);
        // Cursor walk: entries of 0x20 bytes at 0xd00; the last of `k`
        // ends the walk; a `-1` entry jumps to `+0x30`.
        let ring = DATA + 0xd00;
        zero_block(c, ring, 0x60 / 4);
        let k = r.next_u32() % 3;
        for i in 0..2u32 {
            c.bus.write32(ring + 0x20 * i, 0x5a00 + i);
            c.bus.write32(ring + 0x20 * i + 4, 0x0300_0020);
        }
        let wrap = k == 2 && r.next_u32() % 2 == 0;
        if wrap {
            c.bus.write32(ring, 0xffff_ffff);
        }
        c.bus.write32(stream + 0x50, ring);
        c.bus.write32(stream + 0x54, ring + 0x20 * k);
        c.bus.write32(
            stream + 0x30,
            if wrap || r.next_u32() % 2 == 0 {
                ring + 0x20
            } else {
                ring + 0x20 * k
            },
        );
        c.bus
            .write32(stream + 0x38, [2u32, 2, 0x9300][r.next_u32() as usize % 3]);
        c.bus.write32(stream + 0x2c, 0x9400 + r.next_u32() % 0x80);
        c.g[A0][0] = stream as u64;
        if events {
            sched(c, &mut r, seed, 0x003e_1110, 300)
        } else {
            c.pc = 0x003e_1110;
            Events::Off
        }
    }

    /// Invalid streams; empty, done and cancelling request heads (and
    /// status 3, just outside the cancel loop's test); drain
    /// chains, buffer tables and cursor walks of zero to two steps.
    #[test]
    fn stream_file_cancel_all_matches() {
        check_paths(
            gen_stream_file_cancel_all,
            hand_stream_file_cancel_all,
            160,
            &[24, 28, 122, 135, 142, 175, 200, 217, 270, 362],
            &|c, seed| {
                cancel_all_setup(c, seed, false);
            },
        );
    }

    /// The same streams with events at the four loop heads.
    #[test]
    fn stream_file_cancel_all_events() {
        check_events(
            gen_stream_file_cancel_all,
            hand_stream_file_cancel_all_ev,
            240,
            &|c, seed| cancel_all_setup(c, seed, true),
        );
        ea_assert_entered(&[0x003e_1158, 0x003e_1188, 0x003e_11c8, 0x003e_1220]);
    }

    driven!(gen_stream_file_create, ee::s14::f_003e06d8__generated);

    driven!(hand_stream_file_create, stream_file_create);

    ea_counted!(counted_stream_file_create, stream_file_create);

    driven!(hand_stream_file_create_ev, counted_stream_file_create);

    /// `StreamFile_Create` arguments by mode: a budget below 6144 bytes,
    /// fewer than two or more than 256 channels (exactly 257 on some
    /// seeds), a sub-channel count of 0 or over 16, a non-positive or
    /// larger buffer count (each a zero exit), and valid layouts on each
    /// of the five budget rungs (fixed small sizes, and budgets exactly on
    /// the 0x7fff / 0xffff / 0x1ffff rung edges) or with two to six
    /// channels and up to four buffers.
    fn create_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_06d8, 0);
        let stream = DATA + 0x20_0000;
        zero_block(c, stream, 0x1000 / 4);
        let mode = r.next_u32() % 18;
        let (s2, s3) = {
            let s2 = 1 + r.next_u32() % 4;
            (s2, 1 + r.next_u32() % s2)
        };
        let ch = 2 + r.next_u32() % 5;
        let (ch, s2, s3) = match mode {
            1 => (r.next_u32() % 2, s2, s3),
            2 => (257 + r.next_u32() % 50, s2, s3),
            3 => (ch, 0, s3),
            4 => (ch, 17 + r.next_u32() % 4, s3),
            5 => (ch, s2, [0u32, 0xffff_ffff][r.next_u32() as usize % 2]),
            6 => (ch, s2, s2 + 1),
            7..=11 => (2, 1, 1),
            _ => (ch, s2, s3),
        };
        // Bytes left after the pool (S4): one value per rung.
        let rung = if (7..=11).contains(&mode) {
            mode - 7
        } else {
            r.next_u32() % 5
        };
        let left = match mode {
            0 => r.next_u32() % 6144,
            _ => {
                [6144, 0x4000, 0x8000, 0x1_0000, 0x2_0000][rung as usize]
                    + if mode >= 12 { r.next_u32() % 0x2000 } else { 0 }
            }
        };
        // Boundary seeds: exactly 257 channels (the too-many exit) and
        // budgets of exactly 0x7fff, 0xffff and 0x1ffff bytes (each `<`
        // of the size ladder false: the lower rung).
        let (mode, ch, s2, s3, left) = match seed % 25 {
            3 => (2, 257, 1, 1, 6144 + r.next_u32() % 0x100),
            8 => (12, 2, 1, 1, 0x7fff),
            13 => (12, 2, 1, 1, 0xffff),
            18 => (12, 2, 1, 1, 0x1_ffff),
            _ => (mode, ch, s2, s3, left),
        };
        let mem = ch
            .wrapping_mul(0x124)
            .wrapping_add(s2.wrapping_mul(12))
            .wrapping_add(0x180)
            .wrapping_add(s3.wrapping_mul(16))
            .wrapping_add(0x40);
        c.g[A0][0] = ch as u64;
        c.g[A1][0] = s2 as u64;
        c.g[A2][0] = sx(s3);
        c.g[A3][0] = stream as u64;
        c.g[T0][0] = mem.wrapping_add(left) as u64;
        if events {
            let mode_ev = schedule(c, &mut r, seed, 0x003e_06d8);
            // Valid layouts: due after the MemZero call (about 250 cycles
            // in), whose MemSet loop would unwind inside the callee.
            let k = match mode {
                0..=6 => r.next_u32() % 40,
                7..=11 => 250 + r.next_u32() % 80,
                _ => 250 + r.next_u32() % 170,
            };
            if mode_ev != Events::Off {
                c.next_event = c.cycles + k as u64;
            }
            mode_ev
        } else {
            c.pc = 0x003e_06d8;
            Events::Off
        }
    }

    /// Every zero exit and the five budget rungs (fixed sizes), plus
    /// varied valid layouts.
    #[test]
    fn stream_file_create_matches() {
        check_paths_lohi(
            gen_stream_file_create,
            hand_stream_file_create,
            150,
            &[37, 42, 44, 45, 49, 315, 319, 322, 324, 325],
            &|c, seed| {
                create_setup(c, seed, false);
            },
        );
    }

    /// The same arguments with events at the four loop heads.
    #[test]
    fn stream_file_create_events() {
        check_events(
            gen_stream_file_create,
            hand_stream_file_create_ev,
            300,
            &|c, seed| create_setup(c, seed, true),
        );
        ea_assert_entered(&[0x003e_072c, 0x003e_0878, 0x003e_08d0, 0x003e_08f8]);
    }

    driven!(
        gen_stream_file_set_channel_filter,
        ee::s14::f_003e0948__generated
    );

    driven!(
        hand_stream_file_set_channel_filter,
        stream_file_set_channel_filter
    );

    fn set_channel_filter_setup(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x3e_0948, 0);
        // Arms from seed digits: valid `% 6`, the stream words `/ 18 % 3`,
        // `/ 6 % 3`, `/ 54 % 3`, the channel `% 5`, the filter `% 7`, its
        // id `/ 5 % 5`, the limit `/ 7 % 7`. The buffer count (`/ 6 % 3`)
        // is independent of the limit (`/ 7 % 7`), so some seeds (71) reach
        // the `count < limit` compare with count == limit == 8, its only
        // boundary case.
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(EAF_RET_VALID, (seed % 6 == 0) as u32);
        c.bus.write32(stream, stream);
        c.bus
            .write32(stream + 0x20, [1u32, 2, 4][seed as usize / 18 % 3]);
        c.bus
            .write32(stream + 0x28, [0u32, 2, 8][seed as usize / 6 % 3]);
        c.bus.write32(stream + 0x38, (seed / 54 % 3 == 0) as u32);
        c.bus.write32(stream + 0x1c, DATA + 0x800);
        zero_block(c, DATA + 0x800, 16);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = [0u64, 1, 2, 4, 5][seed as usize % 5];
        let filter = matches!(seed % 7, 0 | 3);
        c.g[A2][0] = if filter { 0x9000 + seed / 5 % 5 } else { 0 } as u64;
        c.g[A3][0] = if filter && seed % 2 == 0 { 0xa000 } else { 0 };
        c.g[T0][0] =
            sx([0u32, 1, 3, 8, 0xffff_ffff, 0xffff_fffe, 0xffff_fffd][seed as usize / 7 % 7]);
        c.pc = 0x003e_0948;
    }

    /// Invalid streams; empty channel, out-of-range and already-filtered
    /// channels; positive, -1, -2 and other negative limits against the
    /// buffer count; busy streams; and the store.
    #[test]
    fn stream_file_set_channel_filter_matches() {
        check_paths_lohi(
            gen_stream_file_set_channel_filter,
            hand_stream_file_set_channel_filter,
            240,
            &[
                31, 34, 38, 42, 45, 47, 48, 49, 50, 52, 54, 57, 59, 61, 63, 65,
            ],
            &|c, seed| set_channel_filter_setup(c, seed),
        );
    }

    driven!(gen_stream_file_destroy, ee::s14::f_003e0a28__generated);

    driven!(hand_stream_file_destroy, stream_file_destroy);

    /// A stream with nothing for CancelAll to cancel, a state word of 0,
    /// 1 (busy: the wait loop runs until the other thread stores the new
    /// state after `n` calls) or 5, and with or without an open file.
    fn destroy_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_0a28, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        zero_block(c, stream + 0x100, 0x80 / 4);
        c.bus.write32(EAF_RET_VALID, (r.next_u32() % 6 == 0) as u32);
        c.bus.write32(EAF_RET_QCLOSE, 0);
        c.bus.write32(stream, stream);
        let state = [0u32, 1, 1, 5][r.next_u32() as usize % 4];
        c.bus.write32(stream + 0x38, state);
        c.bus.write32(
            stream + 0x16c,
            if r.next_u32() % 2 == 0 {
                0
            } else {
                DATA + 0x3000
            },
        );
        c.bus.write32(EA_TIMER_NOW, 1000);
        fake_timer(c);
        let n = 1 + r.next_u32() % 6;
        other_setup(
            c,
            n,
            stream + 0x38,
            [0u32, 2][r.next_u32() as usize % 2],
            0,
            0,
        );
        c.g[A0][0] = stream as u64;
        if events {
            sched(c, &mut r, seed, 0x003e_0a28, 200)
        } else {
            c.pc = 0x003e_0a28;
            Events::Off
        }
    }

    /// Invalid streams; idle, busy (one to six passes of the wait loop)
    /// and other states; with and without a file to close.
    #[test]
    fn stream_file_destroy_matches() {
        check_paths(
            gen_stream_file_destroy,
            hand_stream_file_destroy,
            96,
            &[
                20, 74, 97, 121, 148, 167, 191, 220, 241, 267, 288, 311, 338, 348, 370,
            ],
            &|c, seed| {
                destroy_setup(c, seed, false);
            },
        );
    }

    /// The wait loop with events.
    #[test]
    fn stream_file_destroy_events() {
        check_events(
            gen_stream_file_destroy,
            hand_stream_file_destroy,
            150,
            &|c, seed| destroy_setup(c, seed, true),
        );
    }

    driven!(gen_stream_file_set_starved, ee::s14::f_003e0b98__generated);

    driven!(hand_stream_file_set_starved, stream_file_set_starved);

    fn set_starved_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3e_0b98, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        zero_block(c, stream + 0x100, 0x80 / 4);
        c.bus.write32(EAF_RET_VALID, (r.next_u32() % 5 == 0) as u32);
        c.bus.write32(stream, stream);
        c.bus
            .write32(stream + 0x38, [0u32, 1, 1, 2][r.next_u32() as usize % 4]);
        c.bus.write32(stream + 0x40, 0x300 + r.next_u32() % 3);
        // The file's request: a zero priority-table word, so
        // SetRequestPriority finds no entry.
        c.bus.write32(stream + 0x174, DATA + 0x3000);
        zero_block(c, DATA + 0x3000, 16);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = (r.next_u32() % 3) as u64;
        c.pc = 0x003e_0b98;
    }

    /// Invalid streams; cleared and set flags; idle, waiting (the
    /// request-priority call) and other states.
    #[test]
    fn stream_file_set_starved_matches() {
        check_paths(
            gen_stream_file_set_starved,
            hand_stream_file_set_starved,
            96,
            &[20, 24, 28, 70],
            &|c, seed| set_starved_setup(c, seed),
        );
    }

    driven!(gen_stream_file_queue_file, ee::s14::f_003e0d80__generated);

    driven!(hand_stream_file_queue_file, stream_file_queue_file);

    /// A stream at `DATA + 0x600` and a channel list at `DATA + 0x800` of
    /// four 0x20-byte entries; the target id sits in entry `k`.
    fn queue_file_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_0d80, 0);
        ea_install_fakes();
        let stream = DATA + 0x600;
        stream_block(c, stream);
        c.bus.write32(stream, stream);
        // Arms from seed digits: valid `% 6`, the allocation `/ 6 % 6`,
        // enqueue `/ 36 % 2`, next `/ 2 % 2`, running `/ 4 % 2`, the walk
        // length `% 5`, the offset `/ 5 % 5`.
        c.bus.write32(EAF_RET_VALID, (seed % 6 == 0) as u32);
        ea_set_alloc_rets(c, if seed / 6 % 6 == 1 { 0 } else { EAF_REQ });
        zero_block(c, EAF_REQ, 0x130 / 4);
        c.bus.write32(EAF_RET_ENQ, seed / 36 % 2);
        c.bus.write32(EAF_RET_NEXT, seed / 2 % 2);
        c.bus
            .write32(stream + 0x38, [0u32, 3][seed as usize / 4 % 2]);
        let list = DATA + 0x800;
        let target = 0x1234;
        let k = [0u32, 1, 2, 3, 3][seed as usize % 5];
        for i in 0..4u32 {
            c.bus
                .write32(list + 0x20 * i, if i == k { target } else { 0x5600 + i });
            c.bus.write32(list + 0x20 * i + 4, 0x20);
        }
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = list as u64;
        c.g[A2][0] = [0u64, 0, 0, 0x10, 0x1234][seed as usize / 5 % 5];
        c.g[A3][0] = target as u64;
        if events {
            sched(c, &mut r, seed, 0x003e_0d80, 30)
        } else {
            c.pc = 0x003e_0d80;
            Events::Off
        }
    }

    /// Invalid streams; failed allocations; explicit offsets and channel
    /// walks of zero to three steps; idle and running streams.
    #[test]
    fn stream_file_queue_file_matches() {
        check_paths(
            gen_stream_file_queue_file,
            hand_stream_file_queue_file,
            128,
            &[30, 40, 83, 89, 90, 91, 97, 99, 104, 108, 120],
            &|c, seed| {
                queue_file_setup(c, seed, false);
            },
        );
    }

    /// The channel walk with events.
    #[test]
    fn stream_file_queue_file_events() {
        check_events(
            gen_stream_file_queue_file,
            hand_stream_file_queue_file,
            150,
            &|c, seed| queue_file_setup(c, seed, true),
        );
    }

    driven!(
        gen_stream_file_get_chunk_request,
        ee::s14::f_003e15e8__generated
    );

    driven!(
        hand_stream_file_get_chunk_request,
        stream_file_get_chunk_request
    );

    ea_counted!(
        counted_stream_file_get_chunk_request,
        stream_file_get_chunk_request
    );

    driven!(
        hand_stream_file_get_chunk_request_ev,
        counted_stream_file_get_chunk_request
    );

    /// A request list of up to four nodes after the head at `+0x5c`: each
    /// with a status (1 is taken at once) and a ring position the
    /// in-range test compares with the previous one and the stream's.
    fn get_chunk_request_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_15e8, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(EAF_RET_VALID, (r.next_u32() % 6 == 0) as u32);
        c.bus.write32(stream, stream);
        c.bus.write32(stream + 0x8, (r.next_u32() % 5 != 0) as u32);
        c.bus.write32(stream + 0x50, r.next_u32() % 8);
        c.bus.write32(stream + 0xc, r.next_u32() % 8);
        let nodes = [
            DATA + 0x1100,
            DATA + 0x1300,
            DATA + 0x1500,
            DATA + 0x1700,
            DATA + 0x1900,
        ];
        let len = 1 + r.next_u32() % 4;
        for (i, &n) in nodes.iter().enumerate() {
            zero_block(c, n, 0x130 / 4);
            let next = if (i as u32) < len { nodes[i + 1] } else { 0 };
            c.bus.write32(n, 0x4400 + i as u32);
            c.bus.write32(n + 0xc, next);
            c.bus
                .write32(n + 4, [0u32, 1, 2, 2, 2][r.next_u32() as usize % 5]);
            c.bus.write32(n + 0x120, r.next_u32() % 8);
        }
        c.bus.write32(stream + 0x5c, nodes[0]);
        c.g[A0][0] = stream as u64;
        if events {
            sched(c, &mut r, seed, 0x003e_15e8, 60)
        } else {
            c.pc = 0x003e_15e8;
            Events::Off
        }
    }

    /// Invalid and idle streams; walks that end at a taken node, an
    /// in-range one or the list end.
    #[test]
    fn stream_file_get_chunk_request_matches() {
        check_paths(
            gen_stream_file_get_chunk_request,
            hand_stream_file_get_chunk_request,
            128,
            &[
                24, 30, 63, 71, 72, 73, 74, 75, 78, 80, 81, 82, 83, 92, 94, 95, 96, 97, 98, 99,
                100, 118, 120, 133, 134,
            ],
            &|c, seed| {
                get_chunk_request_setup(c, seed, false);
            },
        );
    }

    /// The walk with events at its head.
    #[test]
    fn stream_file_get_chunk_request_events() {
        check_events(
            gen_stream_file_get_chunk_request,
            hand_stream_file_get_chunk_request_ev,
            200,
            &|c, seed| get_chunk_request_setup(c, seed, true),
        );
        ea_assert_entered(&[0x003e_1650]);
    }

    driven!(gen_ea_timer_service_for, ee::s14::f_003e5a10__generated);

    driven!(hand_ea_timer_service_for, ea_timer_service_for);

    ea_counted!(counted_ea_timer_service_for, ea_timer_service_for);

    driven!(hand_ea_timer_service_for_ev, counted_ea_timer_service_for);

    /// A due fake callback that adds a tick per pass and returns 0 (the
    /// wait runs out) or nonzero (ends the wait at once); durations 0-4.
    fn service_for_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_5a10, 0);
        fake_timer(c);
        c.bus.write32(EA_TIMER_NOW, 1000);
        c.bus.write32(EA_TIMER_TICKS, 0x500 + r.next_u32() % 0x100);
        let ret = [0u32, 0, 0, 7][r.next_u32() as usize % 4];
        other_setup(c, 0, 0, 0, 1, ret);
        c.g[A0][0] = (r.next_u32() % 5) as u64;
        if events {
            sched(c, &mut r, seed, 0x003e_5a10, 120)
        } else {
            c.pc = 0x003e_5a10;
            Events::Off
        }
    }

    /// Zero and short durations; callbacks returning 0 and nonzero.
    #[test]
    fn ea_timer_service_for_matches() {
        check_paths(
            gen_ea_timer_service_for,
            hand_ea_timer_service_for,
            64,
            &[27, 180, 182, 335, 492, 649],
            &|c, seed| {
                service_for_setup(c, seed, false);
            },
        );
    }

    /// The service loop with events at its head.
    #[test]
    fn ea_timer_service_for_events() {
        check_events(
            gen_ea_timer_service_for,
            hand_ea_timer_service_for_ev,
            150,
            &|c, seed| service_for_setup(c, seed, true),
        );
        ea_assert_entered(&[0x003e_5a38]);
    }

    driven!(gen_ea_timer_wait_deadline, ee::s14::f_003e4f40__generated);

    driven!(hand_ea_timer_wait_deadline, ea_timer_wait_deadline);

    ea_counted!(counted_ea_timer_wait_deadline, ea_timer_wait_deadline);

    driven!(
        hand_ea_timer_wait_deadline_ev,
        counted_ea_timer_wait_deadline
    );

    /// A deadline 0-4 ticks ahead (or passed); the fake callback adds a
    /// tick per pass.
    fn wait_deadline_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_4f40, 0);
        fake_timer(c);
        c.bus.write32(EA_TIMER_NOW, 1000);
        let now = 0x500 + r.next_u32() % 0x100;
        c.bus.write32(EA_TIMER_TICKS, now);
        c.bus.write32(EA_TIMER_DEADLINE, now + r.next_u32() % 5);
        other_setup(c, 0, 0, 0, 1, 0);
        if events {
            sched(c, &mut r, seed, 0x003e_4f40, 120)
        } else {
            c.pc = 0x003e_4f40;
            Events::Off
        }
    }

    /// Passed and near deadlines.
    #[test]
    fn ea_timer_wait_deadline_matches() {
        check_paths(
            gen_ea_timer_wait_deadline,
            hand_ea_timer_wait_deadline,
            48,
            &[26, 236, 448, 663, 866],
            &|c, seed| {
                wait_deadline_setup(c, seed, false);
            },
        );
    }

    /// The wait loop with events at its head.
    #[test]
    fn ea_timer_wait_deadline_events() {
        check_events(
            gen_ea_timer_wait_deadline,
            hand_ea_timer_wait_deadline_ev,
            150,
            &|c, seed| wait_deadline_setup(c, seed, true),
        );
        ea_assert_entered(&[0x003e_4f50]);
    }

    driven!(gen_ea_timer_delay, ee::s14::f_003e4fb0__generated);

    driven!(hand_ea_timer_delay, ea_timer_delay);

    ea_counted!(counted_ea_timer_delay, ea_timer_delay);

    driven!(hand_ea_timer_delay_ev, counted_ea_timer_delay);

    /// Delays of 0-4 ticks; the fake callback adds a tick per pass.
    fn delay_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_4fb0, 0);
        fake_timer(c);
        c.bus.write32(EA_TIMER_NOW, 1000);
        c.bus.write32(EA_TIMER_TICKS, 0x500 + r.next_u32() % 0x100);
        other_setup(c, 0, 0, 0, 1, 0);
        c.g[A0][0] = (r.next_u32() % 5) as u64;
        if events {
            sched(c, &mut r, seed, 0x003e_4fb0, 120)
        } else {
            c.pc = 0x003e_4fb0;
            Events::Off
        }
    }

    /// Zero and short delays.
    #[test]
    fn ea_timer_delay_matches() {
        check_paths(
            gen_ea_timer_delay,
            hand_ea_timer_delay,
            48,
            &[24, 227, 425, 622, 819],
            &|c, seed| {
                delay_setup(c, seed, false);
            },
        );
    }

    /// The delay loop with events at its head.
    #[test]
    fn ea_timer_delay_events() {
        check_events(
            gen_ea_timer_delay,
            hand_ea_timer_delay_ev,
            150,
            &|c, seed| delay_setup(c, seed, true),
        );
        ea_assert_entered(&[0x003e_4fd0]);
    }

    driven!(gen_ea_thread_wait_for_exit, ee::s14::f_003e55e0__generated);

    driven!(hand_ea_thread_wait_for_exit, ea_thread_wait_for_exit);

    /// A live thread whose status word (written by the ReferThreadStatus
    /// syscall into the frame) turns dead or gone after `n` syscalls;
    /// timeouts of 0 (wait forever), positive (one pass, also with the
    /// ticks exactly at the deadline) and negative (already expired: the
    /// loop goes on).
    fn wait_for_exit_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_55e0, 0);
        zero_block(c, DATA + 0x500, 8);
        c.bus.write32(DATA + 0x504, 0x1234);
        c.bus.write32(EA_TIMER_TICKS, 0x500);
        c.g[A0][0] = sx(DATA + 0x500);
        let status_at = c.g[SP][0] as u32 - 0x70;
        let first = [0x10u32, 0, 0x20, 0x20, 0x20][r.next_u32() as usize % 5];
        c.bus.write32(status_at, first);
        let n = 1 + r.next_u32() % 6;
        other_setup(
            c,
            n,
            status_at,
            [0x10u32, 0][r.next_u32() as usize % 2],
            0,
            0,
        );
        c.g[A1][0] = sx([0u32, 0, 100, 0xffff_ffff, 0xffff_fffd][r.next_u32() as usize % 5]);
        // Ticks exactly at the deadline on some seeds: a live thread, a
        // positive timeout, and the other thread setting the tick count to
        // the deadline at the first status call (`deadline < ticks` false:
        // one pass).
        if seed % 12 == 7 {
            let timeout = 1 + r.next_u32() % 200;
            c.bus.write32(status_at, 0x20);
            other_setup(c, 1, EA_TIMER_TICKS, 0x500 + timeout, 0, 0);
            c.g[A1][0] = timeout as u64;
        }
        if events {
            sched(c, &mut r, seed, 0x003e_55e0, 200)
        } else {
            c.pc = 0x003e_55e0;
            Events::Off
        }
    }

    /// Dead and gone threads exit at once; live ones wait out a timeout of
    /// zero or an expired one (both back edges) or leave after one pass.
    #[test]
    fn ea_thread_wait_for_exit_matches() {
        check_paths(
            gen_ea_thread_wait_for_exit,
            hand_ea_thread_wait_for_exit,
            96,
            &[40, 41, 110, 111, 112, 113, 117, 187, 225],
            &|c, seed| {
                wait_for_exit_setup(c, seed, false);
            },
        );
    }

    /// Status word flips to 0x11 mid-iteration: the other-thread hook
    /// writes it after n fires (n swept by seed), so the final load reads
    /// V0==0x11 while the dead-check saw a live status. Kills 4458
    /// (`< with <=`: orig (1<1)=0 vs mutant (1<=1)=1).
    #[test]
    fn ea_thread_wait_for_exit_status_11_matches() {
        check(
            gen_ea_thread_wait_for_exit,
            hand_ea_thread_wait_for_exit,
            16,
            &|c, seed| {
                let r = machine(c, seed, 0x3e_55e0, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.bus.write32(EA_TIMER_TICKS, 0x500);
                c.g[A0][0] = sx(DATA + 0x500);
                let status_at = c.g[SP][0] as u32 - 0x70;
                c.bus.write32(status_at, 0x20);
                other_setup(c, 1 + seed % 8, status_at, 0x11, 0, 0);
                c.g[A1][0] = 0x7fff_ffff;
                c.pc = 0x003e_55e0;
                let _ = r;
            },
        );
    }

    /// The wait loop with events.
    #[test]
    fn ea_thread_wait_for_exit_events() {
        check_events(
            gen_ea_thread_wait_for_exit,
            hand_ea_thread_wait_for_exit,
            150,
            &|c, seed| wait_for_exit_setup(c, seed, true),
        );
    }

    driven!(gen_ea_thread_create, ee::s14::f_003e50c8__generated);

    driven!(hand_ea_thread_create, ea_thread_create);

    /// The priority jump table at `0x495e00` (game data): priorities
    /// 0x61-0x67 map to these arms.
    const THREAD_PRIORITY_ARMS: [u32; 7] = [
        0x003e_5108,
        0x003e_5110,
        0x003e_5118,
        0x003e_5138,
        0x003e_5120,
        0x003e_5128,
        0x003e_5130,
    ];

    fn thread_create_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3e_50c8, 0);
        put_words(c, 0x0049_5e00, &THREAD_PRIORITY_ARMS);
        zero_block(c, DATA + 0x500, 16);
        c.g[A0][0] = sx(DATA + 0x500);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = sx(DATA + 0x500);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = [0u64, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]
            [r.next_u32() as usize % 10];
        c.pc = 0x003e_50c8;
    }

    /// Default priorities (below 0x61, above 0x67) and every table arm.
    #[test]
    fn ea_thread_create_matches() {
        check_paths(
            gen_ea_thread_create,
            hand_ea_thread_create,
            80,
            &[53, 66, 68],
            &|c, seed| thread_create_setup(c, seed),
        );
    }

    /// `StreamFile_CancelRequest` streams: a one-entry lookup table whose
    /// handle and status the seed picks, cursor words and a buffer table of
    /// zero or one entry.
    fn cancel_request_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        if let Some(v) = cancel_request_variant(seed) {
            return cancel_request_ring_setup(c, seed, v, events);
        }
        let mut r = machine(c, seed, 0x3e_0e90, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(EAF_RET_VALID, (r.next_u32() % 8 == 0) as u32);
        // Self-describing stream: [stream] points back at it, so
        // the validate fake resolves [SP] to the stream.
        c.bus.write32(stream, stream);
        let table = DATA + 0x700;
        let mode = r.next_u32() % 8;
        // Lookup table: count + one entry slot.
        c.bus.write32(stream + 0x18, if mode == 1 { 0 } else { 1 });
        c.bus.write32(stream + 0x14, table);
        // Handles with a zero low byte select entry 0 of the one-entry
        // table; the entry's word matches or not.
        let handle = 0x100 * (1 + r.next_u32() % 4);
        c.bus
            .write32(table, if r.next_u32() % 8 == 0 { 0x700 } else { handle });
        c.bus.write32(
            table + 4,
            [0u32, 1, 4, 9, 9, 9, 9, 9][r.next_u32() as usize % 8],
        );
        c.bus.write32(table + 0xc, 0);
        c.bus.write32(stream + 0x50, 0x9000 + r.next_u32() % 3);
        c.bus.write32(stream + 0x5c, 0x9100 + r.next_u32() % 3);
        c.bus.write32(stream + 0x54, 0x9200 + r.next_u32() % 5);
        // Buffer table of two or three idle (zero level) entries.
        c.bus.write32(stream + 0x28, 2 + r.next_u32() % 2);
        c.bus.write32(stream + 0x24, DATA + 0x900);
        zero_block(c, DATA + 0x900, 12);
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = if mode < 6 {
            handle as u64
        } else {
            (r.next_u32() % 251) as u64
        };
        if events {
            sched(c, &mut r, seed, 0x003e_0e90, 40)
        } else {
            c.pc = 0x003e_0e90;
            Events::Off
        }
    }

    /// The ring of `StreamFile_CancelRequest`'s buffer scan (0x40-byte
    /// chunks) and the request queued after the cancelled one.
    const CR_RING: u32 = DATA + 0x1000;

    const CR_NEXT: u32 = DATA + 0xa00;

    /// Every fourth seed is a ring seed: layouts 0-7 are fixed, 8-13
    /// random (see [`cancel_request_ring_setup`]).
    fn cancel_request_variant(seed: u32) -> Option<u32> {
        (seed % 4 == 3).then_some(seed / 4 % 14)
    }

    /// A marked request (status 9) whose stream has buffers with data on
    /// a ring of 0x40-byte chunks: `[stream+0x50]` = A, the request's
    /// position `[req+0x120]` = B (S6), and the scan end S7 =
    /// `[stream+0x54]`, or the next request's position when that one is
    /// not done (status 1). Buffers (`+4` tag, `+8` level, `+0xc` ring
    /// position) at a position in `[A, B)` walk the chunks from B to S7,
    /// marking those with their tag (`[e+4]` top byte) skipped; at a
    /// position in `[B, S7)` they take and release chunks
    /// (GetChunk/ReleaseChunk) until starved or out of `[B, S7)`; elsewhere
    /// or idle they are passed over. Linear layout: A 0, B 0x100, C 0x200;
    /// wrapped: A 0x200, B 0x300, C 0x80, with a `-1` word at 0x340 that
    /// sends the walk to the ring start. Fixed layouts:
    /// 0 a walk over matching and other chunks (and an idle buffer);
    /// 1 a wrapped walk through the `-1` word (and a buffer out of range);
    /// 2 S7 == B (no walk, no chunk loop);
    /// 3 one chunk, then starved (and a negative level);
    /// 4 two chunks (in range after the first), then a walk with no match;
    /// 5 two chunks, then out of range with data left;
    /// 6 / 7 a walk with a next request of status 1 / 9 (S7 from the
    /// stream / from that request).
    fn cancel_request_ring_setup(c: &mut Cpu, seed: u32, v: u32, events: bool) -> Events {
        const TW: u32 = 0x5a;
        const TC: u32 = 0xa5;
        const TX: u32 = 0x33;
        let mut r = machine(c, seed, 0x3e_0e90, 0);
        ea_install_fakes();
        let stream = DATA + 0x400;
        stream_block(c, stream);
        c.bus.write32(EAF_RET_VALID, 0);
        c.bus.write32(stream, stream);
        let table = DATA + 0x700;
        let bufs_at = DATA + 0x900;
        zero_block(c, table, 0x124 / 4);
        zero_block(c, CR_NEXT, 0x124 / 4);
        zero_block(c, CR_RING, 0x400 / 4);
        zero_block(c, bufs_at, 12);
        let handle = 0x100 * (1 + r.next_u32() % 4);
        c.bus.write32(stream + 0x14, table);
        c.bus.write32(stream + 0x18, 1);
        c.bus.write32(table, handle);
        c.bus.write32(table + 4, 9);
        c.bus.write32(stream + 0x5c, 0x9100);
        c.bus.write32(stream + 0x30, CR_RING);
        c.bus.write32(stream + 0x34, CR_RING + 0x400);
        c.bus.write32(stream + 0x24, bufs_at);
        let random = v >= 8;
        let wrap = v == 1 || (random && r.next_u32() % 2 == 0);
        // A, B, C, the chunks from B to C in walk order, a position in
        // neither range.
        let (a, b, end, path, out): (u32, u32, u32, [u32; 4], u32) = if wrap {
            (0x200, 0x300, 0x80, [0x300, 0x340, 0x000, 0x040], 0x100)
        } else {
            (0x000, 0x100, 0x200, [0x100, 0x140, 0x180, 0x1c0], 0x300)
        };
        let tags: [u32; 4] = match v {
            0 => [TW, TX, TW, TX],
            1 => [TW, 0, TX, TW],
            2..=5 => [TC; 4],
            6 => [TW, TW, TX, TW],
            7 => [TW, TX, TW, TW],
            _ => std::array::from_fn(|_| [TW, TC, TX][r.next_u32() as usize % 3]),
        };
        let put_chunk = |c: &mut Cpu, off: u32, tag: u32| {
            c.bus.write32(CR_RING + off, 0);
            c.bus.write32(CR_RING + off + 4, tag << 24 | 0x40);
        };
        for (i, &off) in path.iter().enumerate() {
            if wrap && i == 1 {
                c.bus.write32(CR_RING + off, 0xffff_ffff);
            } else {
                put_chunk(c, off, tags[i]);
            }
        }
        // GetChunk's look-ahead past the last chunk taken finds a chunk
        // buffer's tag.
        put_chunk(c, end, TC);
        put_chunk(c, end + 0x40, TC);
        let (s54, next) = match v {
            2 => (b, None),
            6 => (end, Some((1, path[2]))),
            7 => (end, Some((9, path[2]))),
            0..=5 => (end, None),
            _ => {
                let s54 = if r.next_u32() % 6 == 0 { b } else { end };
                let next = match r.next_u32() % 3 {
                    0 => None,
                    k => Some(([1, 9][k as usize - 1], path[2 + r.next_u32() as usize % 2])),
                };
                (s54, next)
            }
        };
        c.bus.write32(stream + 0x50, CR_RING + a);
        c.bus.write32(table + 0x120, CR_RING + b);
        c.bus.write32(stream + 0x54, CR_RING + s54);
        let s7 = match next {
            Some((status, at)) => {
                c.bus.write32(table + 0xc, CR_NEXT);
                c.bus.write32(CR_NEXT + 4, status);
                c.bus.write32(CR_NEXT + 0x120, CR_RING + at);
                if status == 1 {
                    s54
                } else {
                    at
                }
            }
            None => s54,
        };
        // Chunks in [B, S7) in walk order (B itself when that is empty).
        let chunks: Vec<u32> = path
            .iter()
            .copied()
            .filter(|&o| !(wrap && o == 0x340))
            .take_while(|&o| o != s7)
            .collect();
        let bufs: Vec<(u32, u32, u32)> = match v {
            0 => vec![(TW, 0x200, a + 0x40), (TW, 0, a)],
            1 => vec![(TW, 0x200, a + 0x40), (TC, 5, out)],
            2 => vec![(TW, 0x40, a + 0x40), (TC, 0x40, b)],
            3 => vec![(TC, 0x40, b), (TC, 0xffff_fffb, b)],
            4 => vec![(TC, 0x80, b), (TW, 0x100, a + 0x80)],
            5 => vec![(TC, 0x1000, b + 0x80)],
            6 => vec![(TW, 0x200, a + 0x10)],
            7 => vec![(TW, 0x200, a + 0x10), (TW, 0x30, a + 0x20)],
            _ => (0..2 + r.next_u32() % 2)
                .map(|_| match r.next_u32() % 4 {
                    0 => (TW, 1 + r.next_u32() % 0x200, a + 0x10 * (r.next_u32() % 16)),
                    1 => {
                        let at = if chunks.is_empty() {
                            b
                        } else {
                            chunks[r.next_u32() as usize % chunks.len()]
                        };
                        (TC, 1 + r.next_u32() % 0x100, at)
                    }
                    2 => (TX, 1 + r.next_u32() % 0x100, out),
                    _ => (TC, [0, 0xffff_ff00][r.next_u32() as usize % 2], b),
                })
                .collect(),
        };
        c.bus.write32(stream + 0x28, bufs.len() as u32);
        for (i, &(tag, level, at)) in bufs.iter().enumerate() {
            let buf = bufs_at + 0x10 * i as u32;
            c.bus.write32(buf, stream);
            c.bus.write32(buf + 4, 0x1200 | tag);
            c.bus.write32(buf + 8, level);
            c.bus.write32(buf + 0xc, CR_RING + at);
        }
        c.g[A0][0] = stream as u64;
        c.g[A1][0] = handle as u64;
        if events {
            let mode = schedule(c, &mut r, seed, 0x003e_0e90);
            if mode != Events::Off {
                // Fixed layouts: due at once, so the first event is at the
                // first back edge (the walk's in 0, 1, 6, 7, the chunk
                // loop's in 4, 5).
                let k = if random { r.next_u32() % 600 } else { 0 };
                c.next_event = c.cycles + k as u64;
            }
            mode
        } else {
            c.pc = 0x003e_0e90;
            Events::Off
        }
    }

    /// Invalid streams; lookup misses (empty table, other handle, other
    /// word, zero status); done, freed and marked requests; the mark
    /// arm's scan of two or three idle buffers; the ring seeds' scans
    /// (fixed layouts 0-7 by total, random ones besides).
    #[test]
    fn stream_file_cancel_request_matches() {
        check_paths(
            gen_stream_file_cancel_request,
            hand_stream_file_cancel_request,
            192,
            &[
                41, 76, 84, 86, 88, 114, 135, 148, 184, 310, 329, 343, 370, 399, 552, 602,
            ],
            &|c, seed| {
                cancel_request_setup(c, seed, false);
            },
        );
    }

    thread_local! {
        /// `pc` of the first event a generated body delivers.
        static FIRST_EVENT: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    }

    fn first_event_hook(c: &mut Cpu) -> bool {
        FIRST_EVENT.with(|f| {
            if f.get() == 0 {
                f.set(c.pc);
            }
        });
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    /// The scans with events; the fixed ring layouts' first event lands
    /// in the ring walk (0x3e1000) and in the chunk loop (0x3e1090).
    #[test]
    fn stream_file_cancel_request_events() {
        check_events(
            gen_stream_file_cancel_request,
            hand_stream_file_cancel_request,
            192,
            &|c, seed| cancel_request_setup(c, seed, true),
        );
        let mut first = std::collections::BTreeMap::new();
        for seed in 0..192 {
            let Some(v) = cancel_request_variant(seed).filter(|&v| v < 8) else {
                continue;
            };
            let mut c = crate::difftest::fresh();
            if cancel_request_setup(&mut c, seed, true) == Events::Off {
                continue;
            }
            FIRST_EVENT.with(|f| f.set(0));
            c.hooks.event = first_event_hook;
            crate::difftest::run_generated(&mut c, gen_stream_file_cancel_request);
            first
                .entry(FIRST_EVENT.with(|f| f.get()))
                .or_insert_with(Vec::new)
                .push(v);
        }
        assert!(
            first.contains_key(&0x003e_1000) && first.contains_key(&0x003e_1090),
            "first events (pc: layouts): {first:x?}"
        );
    }

    /// `EATimer_RemoveCallback` fixtures (see the `_matches` test).
    fn remove_callback_setup(c: &mut Cpu, seed: u32, events: bool) -> Events {
        let mut r = machine(c, seed, 0x3e_58c8, 0);
        zero_block(c, EA_TIMERS, 64);
        let f = DATA + 0x400;
        put_ascii(c, f, "cb");
        match r.next_u32() % 4 {
            0 => {
                c.bus.write32(EA_TIMERS, f);
            }
            1 => {
                c.bus.write32(EA_TIMERS + 3 * 0x10, f);
                c.bus.write32(EA_TIMERS + 3 * 0x10 + 4, 2);
            }
            2 => {
                c.bus.write32(EA_TIMERS + 12 * 0x10, f);
            }
            _ => {}
        }
        c.g[A0][0] = sx(f);
        if events {
            sched(c, &mut r, seed, 0x003e_58c8, 100)
        } else {
            c.pc = 0x003e_58c8;
            Events::Off
        }
    }

    /// Fn in slot 0, in a later slot, or absent.
    #[test]
    fn ea_timer_remove_callback_matches() {
        check(
            gen_ea_timer_remove_callback,
            hand_ea_timer_remove_callback,
            48,
            &|c, seed| {
                remove_callback_setup(c, seed, false);
            },
        );
    }

    /// The slot scan with events.
    #[test]
    fn ea_timer_remove_callback_events() {
        check_events(
            gen_ea_timer_remove_callback,
            hand_ea_timer_remove_callback,
            96,
            &|c, seed| remove_callback_setup(c, seed, true),
        );
    }

    use crate::difftest::elf_image;

    pub(crate) use crate::game::lib::eafile::tests::oc697_slot;
    pub(crate) use crate::game::lib::eafile::tests::OC697_CTRL;
    pub(crate) use crate::game::lib::eafile::tests::OC697_FAIL_ALARM;
    pub(crate) use crate::game::lib::eafile::tests::OC697_MAGIC;
    pub(crate) use crate::game::lib::eafile::tests::OC697_SCRIPT;
    pub(crate) use crate::game::lib::eafile::tests::OC697_SECTORS;
    pub(crate) use crate::game::lib::eafile::tests::OC697_SECT_NEXT;
    pub(crate) use crate::game::lib::eafile::tests::OC697_SECT_ON;

    pub(crate) use crate::game::lib::eafile::tests::oc697_open;
    pub(crate) use crate::game::lib::eafile::tests::oc697_reset_scripts;

    pub(crate) use crate::game::lib::cdvd::libcdvd_read_rpc_end_callback;
    pub(crate) use crate::game::lib::eafile::ea_file_cd_reset;
    pub(crate) use crate::game::lib::eafile::ea_file_cd_scan_dir;
    pub(crate) use crate::game::lib::eafile::tests::oc697_scan_setup;

    /// `EAMutex_Init`: the semaphore call is a fake (it unwinds on some
    /// seeds).
    #[test]
    fn ea_mutex_init_matches() {
        covered(
            ee::s14::f_003e5698__generated,
            ea_mutex_init,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003e_5698, 1);
                c.pc = 0x003e_5698;
                c.g[A0][0] = (DATA + 0x100 + 0x10 * (seed % 8)) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// `EAMutex_Destroy`: the delete call is a fake (it unwinds on some
    /// seeds).
    #[test]
    fn ea_mutex_destroy_matches() {
        covered(
            ee::s14::f_003e56e0__generated,
            ea_mutex_destroy,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003e_56e0, 1);
                c.pc = 0x003e_56e0;
                c.g[A0][0] = (DATA + 0x100 + 0x10 * (seed % 8)) as u64;
                for i in 0..4 {
                    c.bus.write32(DATA + 0x100 + 4 * i, r.next_u32());
                }
            },
        );
    }

    /// The logging fake (every register to the call log, maybe an unwind),
    /// then the scripted result and the sector copy.
    pub(crate) fn oc697_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let dest = c.g[A2][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let base = OC697_SCRIPT + oc697_slot(addr) * 0x40;
        let n = c.bus.read32(base);
        if n != 0 {
            let i = c.bus.read32(base + 4);
            c.bus.write32(base + 4, i.wrapping_add(1));
            let v = c.bus.read32(base + 8 + 4 * (i % n));
            c.g[V0][0] = sx(v);
        }
        if addr == 0x0040_1df8 && c.bus.read32(OC697_SECT_ON) != 0 {
            let i = c.bus.read32(OC697_SECT_NEXT);
            c.bus.write32(OC697_SECT_NEXT, i.wrapping_add(1));
            let src = OC697_SECTORS + if i == 0 { 0 } else { 0x800 };
            for w in 0..0x200 {
                let v = c.bus.read32(src + 4 * w);
                c.bus.write32(dest + 4 * w, v);
            }
        }
    }

    macro_rules! oc697_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC697_CTRL) != OC697_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc697_fake_call(c, $addr)
                }
            )*
            pub(crate) fn oc697_hook_all() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc697_fakes! {
        oc697_fake_cd_init, OC697_PREV_CD_INIT, 0x0040_1718,
            ee::s15::OVERRIDE_00401718, ee::s15::f_00401718__generated;
        oc697_fake_cd_mmode, OC697_PREV_CD_MMODE, 0x0040_2450,
            ee::s15::OVERRIDE_00402450, ee::s15::f_00402450__generated;
        oc697_fake_cd_disk_ready, OC697_PREV_CD_DISK_READY, 0x0040_1bf0,
            ee::s15::OVERRIDE_00401BF0, ee::s15::f_00401bf0__generated;
        oc697_fake_cd_read, OC697_PREV_CD_READ, 0x0040_1df8,
            ee::s15::OVERRIDE_00401DF8, ee::s15::f_00401df8__generated;
        oc697_fake_cd_sync, OC697_PREV_CD_SYNC, 0x0040_13e8,
            ee::s15::OVERRIDE_004013E8, ee::s15::f_004013e8__generated;
        oc697_fake_cd_get_error, OC697_PREV_CD_GET_ERROR, 0x0040_2300,
            ee::s15::OVERRIDE_00402300, ee::s15::f_00402300__generated;
        oc697_fake_wait_sema, OC697_PREV_WAIT_SEMA, 0x0042_3de0,
            ee::s15::OVERRIDE_00423DE0, ee::s15::f_00423de0__generated;
        oc697_fake_delay_ticks, OC697_PREV_DELAY_TICKS, 0x003e_35b0,
            ee::s14::OVERRIDE_003E35B0, ee::s14::f_003e35b0__generated;
        oc697_fake_complete_request, OC697_PREV_COMPLETE_REQUEST, 0x003d_e420,
            ee::s14::OVERRIDE_003DE420, ee::s14::f_003de420__generated;
    }

    /// The syscall hook with `SetAlarm` failing when the gate is set.
    pub(crate) fn oc697_syscall(c: &mut Cpu, at: u32) {
        fakes::fake_syscall(c, at);
        if at == 0x0042_3b24 && c.bus.read32(OC697_FAIL_ALARM) == OC697_MAGIC {
            c.g[V0][0] = u64::MAX;
        }
    }

    driven!(oc697_reset_gen, ee::s14::f_003e3618__generated);

    driven!(oc697_reset_hand, ea_file_cd_reset);

    /// `EAFile_CdReset` on a running thread with events due in the poll
    /// loops (read retry, busy sync, error retry).
    #[test]
    fn ea_file_cd_reset_events() {
        check_events(oc697_reset_gen, oc697_reset_hand, 96, &|c, seed| {
            let mut r = Rng(0x003e_3618 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            oc697_open(c, u32::MAX);
            oc697_reset_scripts(c, 1 + seed / 3 % 2, 2 + seed / 6 % 11);
            c.bus.write32(0x0045_0c60, r.next_u32());
            schedule(c, &mut r, seed, 0x003e_3618)
        });
    }

    driven!(oc697_scan_gen, ee::s14::f_003e3758__generated);

    driven!(oc697_scan_hand, ea_file_cd_scan_dir);

    /// `EAFile_CdScanDir` on a running thread with events due at its back
    /// edges (the record walk, the next sector).
    #[test]
    fn ea_file_cd_scan_dir_events() {
        check_events(oc697_scan_gen, oc697_scan_hand, 60, &|c, seed| {
            let mut r = Rng(0x003e_3758 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            oc697_open(c, u32::MAX);
            oc697_scan_setup(c, seed, &mut r);
            schedule(c, &mut r, seed, 0x003e_3758)
        });
    }

    // -------------------------------------------------------------------
    // THREAD_yieldticks

    /// `THREAD_yieldticks`: positive, zero and negative tick counts; tick
    /// rates including zero (the divide by zero); alarm failing (the debug
    /// message) or not; in interrupt context or not; fakes unwinding.
    #[test]
    fn thread_yieldticks_matches() {
        covered(
            ee::s14::f_003e52b0__generated,
            thread_yieldticks,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003e_52b0, 6);
                c.hooks.syscall = oc697_syscall;
                elf_image(c);
                c.pc = 0x003e_52b0;
                c.g[A0][0] = match seed % 5 {
                    0 => 0,
                    1 => sx(0xffff_ff00 | (r.next_u32() & 0xff)),
                    2 => 1 + (r.next_u32() % 100) as u64,
                    3 => 0x7fff,
                    _ => r.next_u32() as u64,
                };
                c.bus
                    .write32(0x0045_0dc8, [60, 1000, 0, 7][(seed / 5 % 4) as usize]);
                c.bus.write32(0x0045_0da0, r.next_u32() % 2000);
                c.bus.write32(0x0045_0de4, (seed / 20) % 2);
                if seed / 40 % 2 == 0 {
                    c.bus.write32(OC697_FAIL_ALARM, OC697_MAGIC);
                    // The debug message prints through [0x450dd8] (a fake) an
                    // empty string.
                    c.bus.write32(0x0045_0dd8, FAKE + 0x20);
                    c.bus.write8(0x0049_5e20, 0);
                }
            },
        );
    }

    // -------------------------------------------------------------------
    // libcdvd_ReadRpcEndCallback

    /// The read-RPC packet and the two destinations.
    pub(crate) const OC697_RPC: u32 = DATA + 0x100;

    pub(crate) const OC697_RPC_DST1: u32 = DATA + 0x400;

    pub(crate) const OC697_RPC_DST2: u32 = DATA + 0x600;

    /// Packet, destinations and the pending-command words the tail call
    /// reads.
    pub(crate) fn oc697_rpc_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        c.g[A0][0] = OC697_RPC as u64;
        let n1 = match seed % 4 {
            0 => 0,
            1 => 0xffff_ffff,
            _ => 1 + r.next_u32() % 0x40,
        };
        let n2 = match seed / 4 % 4 {
            0 => 0,
            1 => 0xffff_ffff,
            _ => 1 + r.next_u32() % 0x40,
        };
        c.bus.write32(OC697_RPC, n1);
        c.bus.write32(OC697_RPC + 4, n2);
        c.bus.write32(OC697_RPC + 8, OC697_RPC_DST1);
        c.bus.write32(OC697_RPC + 0xc, OC697_RPC_DST2);
        for w in 0..0x20 {
            c.bus.write32(OC697_RPC + 0x10 + 4 * w, r.next_u32());
        }
        c.bus.write32(
            0x0045_1658,
            if seed % 3 == 0 {
                0xb
            } else {
                r.next_u32() % 13
            },
        );
        c.bus.write32(0x0045_1628, r.next_u32() & 0xffff);
        c.bus.write32(0x0045_1620, r.next_u32() & 0xffff);
        c.bus
            .write32(0x0045_1614, if seed % 5 == 1 { 0 } else { 0x0052_0000 });
        c.bus
            .write32(0x0052_64c0, if seed % 7 == 2 { 0 } else { seed });
    }

    driven!(oc697_rpc_gen, ee::s15::f_00400b48__generated);

    driven!(oc697_rpc_hand, libcdvd_read_rpc_end_callback);

    /// `libcdvd_ReadRpcEndCallback` on a running thread with events due in
    /// the byte-copy loops.
    #[test]
    fn libcdvd_read_rpc_end_callback_events() {
        check_events(oc697_rpc_gen, oc697_rpc_hand, 96, &|c, seed| {
            let mut r = Rng(0x0040_0b48 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
            elf_image(c);
            // Copies of at least two bytes in both runs reach the back edges.
            oc697_rpc_setup(c, 2 + seed % 2 + 4 * (2 + seed / 3 % 2), &mut r);
            schedule(c, &mut r, seed, 0x0040_0b48)
        });
    }
}
