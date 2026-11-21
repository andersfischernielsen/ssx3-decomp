//! `src/ealib/seg_2dcae8.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
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
    (
        0x003e_4f08,
        ea_file_timer_set_deadline,
        "EATimer_SetDeadline",
    ),
    (
        0x003e_4f40,
        ea_file_timer_wait_deadline,
        "EATimer_WaitDeadline",
    ),
    (
        0x003e_4f80,
        ea_file_timer_deadline_passed,
        "EATimer_DeadlinePassed",
    ),
    (0x003e_4fb0, ea_file_timer_delay, "EATimer_Delay"),
    (
        0x003e_5008,
        ea_file_timer_get_tick_rate,
        "EATimer_GetTickRate",
    ),
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
];

/// Callees of the EA file, stream, timer and thread ports below:
/// re-exports of the homes the reorg tool gave their definitions.
pub(crate) mod callee {
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
    pub(crate) use crate::game::misc::callee::EA_ADD_SHUTDOWN_HANDLER;
    pub(crate) use crate::game::misc::callee::EA_FILE_SET_REQUEST_PRIORITY;
    pub(crate) use crate::game::misc::callee::EA_THREAD_DESTROY;
    pub(crate) use crate::game::misc::callee::EA_THREAD_YIELD_TICKS;
    pub(crate) use crate::game::misc::callee::EA_TIMER_DEADLINE_PASSED;
    pub(crate) use crate::game::misc::callee::EA_TIMER_GET_TICKS;
    pub(crate) use crate::game::misc::callee::EA_TIMER_SET_VIDEO_MODE;
    pub(crate) use crate::game::misc::callee::STREAM_FILE_ALLOC_REQUEST;
    pub(crate) use crate::game::misc::callee::STREAM_FILE_CALC_MEM_SIZE;
    pub(crate) use crate::game::misc::callee::STREAM_FILE_CANCEL_ALL;
    pub(crate) use crate::game::misc::callee::STREAM_FILE_ENQUEUE_REQUEST;
    pub(crate) use crate::game::misc::callee::STREAM_FILE_SET_STARVED;
    pub(crate) use crate::game::misc::callee::STREAM_FILE_START_NEXT_REQUEST;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_FILE_CLOSE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_SET;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ZERO;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MUTEX_DESTROY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MUTEX_INIT;
}

// ---------------------------------------------------------------------------
// EA timer.

/// The current timer tick (`0x450dd0`) and its copy taken by the service
/// pass (`0x450e04`).
pub(crate) const EA_TIMER_NOW: u32 = 0x0045_0dd0;

pub(crate) const EA_TIMER_SERVICED_AT: u32 = 0x0045_0e04;

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
// EA file, stream, timer and thread ports moved from misc.rs:
// the reorg tool leaves names it cannot map in misc; these belong to this unit.

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
                // Scan the channel list for a matching entry.
                c.cycles += 2;
                if (c.g[S0][0] as i64) > 0 {
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
                if !done {
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
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x20);
            } else {
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
                frame.restore(c, S1, 0x20);
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
                if c.g[V0][0] != c.g[V1][0] {
                    frame.restore(c, S0, 0x20);
                } else {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x003e0be4);
                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x174));
                    callee::EA_FILE_SET_REQUEST_PRIORITY.call(c, 0x003e0be4)?;
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
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S4][0] = 0x1;
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

/// `u32 StreamFile_Create(handle, channels, mode, arg3, arg4, arg5)`
/// (0x3e06d8): validates the configuration and lays out the stream
/// block pool; 0 when any check fails, otherwise the entries base.
pub fn stream_file_create(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_878: u32 = 0x003e_0878;
        const HEAD_8D0: u32 = 0x003e_08d0;
        const HEAD_8F8: u32 = 0x003e_08f8;
        c.cycles += 18;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_0710);
        c.g[S5][0] = c.g[T0][0];
        callee::STREAM_FILE_CALC_MEM_SIZE.call(c, 0x003e_0710)?;
        c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S4][0] as i64) < 6144) as u64;
        let small = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        let mut ok = !small;
        if ok {
            // Channel-count and range validation chain.
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
            let big_enough = c.g[V0][0] == 0;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 257) as u64;
            if !big_enough {
                c.cycles += 2;
                ok = false;
            } else {
                c.cycles += 2;
                let too_big = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                if too_big {
                    c.cycles += 2;
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_072c);
                    }
                    ok = false;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x10) as u64;
                    let fits = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if !fits {
                        ok = false;
                    } else {
                        c.cycles += 2;
                        let nonneg = (c.g[S3][0] as i64) > 0;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        if !nonneg {
                            c.cycles += 2;
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_072c);
                            }
                            ok = false;
                        } else {
                            c.cycles += 2;
                            let ordered = c.g[V0][0] != 0;
                            c.g[V0][0] = 0;
                            if ordered {
                                ok = false;
                            }
                        }
                    }
                }
            }
        }
        if !ok {
            // Every validation failure exits with V0 zero.
            c.g[V0][0] = 0;
        }
        if ok {
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
            // Entry-size ladder from the leftover budget.
            c.sw((c.g[S0][0] as u32).wrapping_add(0x16c), 0);
            c.g[V0][0] = ((c.g[S4][0] as i64) < 16384) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x800;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x7fff;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let big = c.g[V0][0] != 0;
                c.g[V0][0] = 0xffff;
                if big {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                    let big2 = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x10000;
                    if big2 {
                        c.cycles += 4;
                        c.g[V0][0] = c.g[V0][0] | 0xffff;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        let big3 = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x8000;
                        if !big3 {
                            c.cycles += 1;
                            c.g[V0][0] = 0x4000;
                        }
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = 0x2000;
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1000;
                }
            }
            c.cycles += 3;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
            let empty_ch = (c.g[S1][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if !empty_ch {
                c.cycles += 3;
                c.g[A3][0] = 0x124;
                c.g[A2][0] = 0;
                loop {
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
                    if c.g[A0][0] == 0 {
                        break;
                    }
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x124));
                    if event_due(c) {
                        deliver_event_here(c, HEAD_878);
                    }
                }
            }
            c.cycles += 6;
            c.g[V0][0] = 0x124;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
            c.mult(0, 2, 17, 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let empty_buf = (c.g[S2][0] as i64) <= 0;
            c.sw((c.g[V0][0] as u32).wrapping_sub(0x118), 0);
            if !empty_buf {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = 0x1;
                c.g[V1][0] = 0;
                loop {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.sw(c.g[V0][0] as u32, 0);
                    let more = c.g[A1][0] != 0;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD_8D0);
                    }
                }
            }
            c.cycles += 2;
            let empty_sub = (c.g[S3][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if !empty_sub {
                loop {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.g[A0][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0);
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    c.sw(c.g[V1][0] as u32, c.g[S0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, HEAD_8F8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
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
/// subscriber epoch word, or 0 when the stream is invalid.
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

/// `u32 StreamFile_GetChunkRequest(stream)` (0x3e15e8): finds the next
/// live chunk request, or 0 when the stream is invalid or drained.
pub fn stream_file_get_chunk_request(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_650: u32 = 0x003e_1650;
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        c.g[RA][0] = sx(0x003e_160c);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_160c)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            let live = c.g[V1][0] != 0;
            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            if live {
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
                // Walk the subscriber list for a live entry.
                loop {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    if c.g[A1][0] == 0 {
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                    let found = c.g[V0][0] == c.g[S3][0];
                    c.g[A2][0] = c.g[S2][0];
                    if found {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break;
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x120));
                    c.g[RA][0] = sx(0x003e_1678);
                    c.g[A1][0] = c.g[S0][0];
                    callee::STREAM_FILE_RING_IN_RANGE.call(c, 0x003e_1678)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        break;
                    }
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    if event_due(c) {
                        deliver_event_here(c, HEAD_650);
                    }
                }
                // Found (or list end): unlock and report.
                c.cycles += 3;
                c.g[RA][0] = sx(0x003e_168c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                callee::EA_MUTEX_UNLOCK.call(c, 0x003e_168c)?;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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

/// `u32 StreamFile_CancelAll(stream)` (0x3e1110): cancels every chunk
/// request on a stream and reclaims its buffers.
pub fn stream_file_cancel_all(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_1158: u32 = 0x003e_1158;
        const HEAD_1188: u32 = 0x003e_1188;
        const HEAD_11C8: u32 = 0x003e_11c8;
        const HEAD_1220: u32 = 0x003e_1220;
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A1][0] = c.g[SP][0];
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = 0;
        c.g[RA][0] = sx(0x003e_1134);
        c.g[A2][0] = c.g[SP][0] | 0x4;
        callee::STREAM_FILE_VALIDATE.call(c, 0x003e_1134)?;
        let invalid = c.g[V0][0] != 0;
        if invalid {
            frame.restore(c, S0, 0x30);
        } else {
            // Per-channel head: an empty head restores S0 and skips
            // straight to the shared epilogue (no drain/consume/fixup).
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
            let no_head = c.g[A1][0] == 0;
            if no_head {
                frame.restore(c, S0, 0x30);
            }
            if !no_head {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                loop {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x2) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A1][0] = c.lw(c.g[A1][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, HEAD_1158);
                    }
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003e_1160);
                    c.g[A0][0] = c.g[S0][0];
                    callee::STREAM_FILE_CANCEL_REQUEST.call(c, 0x003e_1160)?;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                }
                c.cycles += 2;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
            }
            if !no_head {
                // Drain live channel requests.
                loop {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x60));
                    let drained = c.g[V1][0] == c.g[V0][0];
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    if drained {
                        break;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x003e_1194);
                    callee::STREAM_FILE_FREE_REQUEST.call(c, 0x003e_1194)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    if event_due(c) {
                        deliver_event_here(c, HEAD_1188);
                    }
                }
                // Zero the buffer table.
                c.cycles += 8;
                c.g[A0][0] = 0x4;
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x60));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A0][0]);
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                if !empty {
                    loop {
                        c.cycles += 10;
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                        c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
                        c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, HEAD_11C8);
                        }
                    }
                }
                // Consume leftover bytes, then fix up the cursors.
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                c.g[RA][0] = sx(0x003e_11fc);
                callee::STREAM_FILE_CONSUME_BYTES.call(c, 0x003e_11fc)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let synced = c.g[A0][0] == c.g[V0][0];
                c.g[A1][0] = 0xff0000;
                if !synced {
                    c.cycles += 4;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[A1][0] = c.g[A1][0] | 0xffff;
                    c.g[A2][0] = 0xfffffffffffffffe;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    // Walk entries to the cursor.
                    loop {
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[A3][0] {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                        } else {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = c.g[V0][0] & c.g[A1][0];
                            c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                            let other = c.g[V0][0] != c.g[S3][0];
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            if other {
                                break;
                            }
                            c.cycles += 15;
                            c.g[RA][0] = sx(0x003e_1028);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            callee::EA_MUTEX_LOCK.call(c, 0x003e_1028)?;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S1][0] as u32));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_1044);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                            callee::EA_MUTEX_UNLOCK.call(c, 0x003e_1044)?;
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                            c.g[RA][0] = sx(0x003e_1050);
                            c.g[A1][0] = c.g[S1][0];
                            callee::STREAM_FILE_CONSUME_BYTES.call(c, 0x003e_1050)?;
                            c.sw(c.g[S0][0] as u32, c.g[S4][0]);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                            c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                        if c.g[A0][0] == c.g[V0][0] {
                            break;
                        }
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, HEAD_1220);
                        }
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.lw(c.g[SP][0] as u32);
                c.g[V1][0] = 0x2;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x38));
                let starved = c.g[V0][0] != c.g[V1][0];
                frame.restore(c, S0, 0x30);
                if !starved {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x30));
                    if c.g[A0][0] != c.g[V0][0] {
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
                    } else {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x54));
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x30), c.g[V1][0]);
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    }
                }
                c.cycles += 2;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0);
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

/// `void EATimer_Shutdown()` (0x3e4d68): stop the timer interrupt.
pub fn ea_timer_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = 0x450000;
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
            let hook = 0x0042_4880;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_4d98)?;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xddc));
            c.g[RA][0] = sx(0x003e_4da4);
            c.g[A0][0] = 0xa;
            let hook = 0x0042_3aa0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_4da4)?;
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
        c.g[S1][0] = 0x450000;
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
pub fn ea_file_timer_set_deadline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4f24);
        c.g[S1][0] = 0x450000;
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
pub fn ea_file_timer_wait_deadline(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_50: u32 = 0x003e_4f50;
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        'wait: loop {
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_4f68);
            callee::EA_TIMER_DEADLINE_PASSED.call(c, 0x003e_4f68)?;
            let done = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if done {
                break 'wait;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD_50);
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_4f58);
            callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_4f58)?;
            c.g[RA][0] = sx(0x003e_4f60);
            c.g[A0][0] = 0;
            callee::EA_THREAD_YIELD.call(c, 0x003e_4f60)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_DeadlinePassed()` (0x3e4f80): nonzero once now is past
/// the deadline.
pub fn ea_file_timer_deadline_passed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4f90);
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4f90)?;
        c.g[A0][0] = 0x450000;
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
pub fn ea_file_timer_delay(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_D0: u32 = 0x003e_4fd0;
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_4fc4);
        c.g[S0][0] = c.g[A0][0];
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4fc4)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        'delay: loop {
            c.cycles += 5;
            c.g[RA][0] = sx(0x003e_4fe8);
            callee::EA_TIMER_GET_TICKS.call(c, 0x003e_4fe8)?;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
            let wait = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = 0;
            if !wait {
                break 'delay;
            }
            if event_due(c) {
                deliver_event_here(c, HEAD_D0);
            }
            c.cycles += 4;
            c.g[RA][0] = sx(0x003e_4fd8);
            callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_4fd8)?;
            c.g[RA][0] = sx(0x003e_4fe0);
            c.g[A0][0] = 0;
            callee::EA_THREAD_YIELD.call(c, 0x003e_4fe0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EATimer_GetTickRate()` (0x3e5008): cached ticks-per-second.
pub fn ea_file_timer_get_tick_rate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        c.g[V1][0] = 0x450000;
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
        let hook = 0x0042_3c90;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_5028)?;
        c.g[V1][0] = 0x50530000;
        c.g[A0][0] = 0x450000;
        c.g[V1][0] = c.g[V1][0] | 0x3254;
        c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdf8));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xdf8), c.g[V1][0]);
        c.g[A1][0] = 0x64;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x003e_504c);
        c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        let hook = 0x0042_3c30;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_504c)?;
        c.g[A0][0] = 0x520000;
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
        c.g[A0][0] = 0x520000;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_507c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1278));
        callee::EA_MUTEX_DESTROY.call(c, 0x003e_507c)?;
        c.g[A0][0] = 0x450000;
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
            // Jump table on the loaded handler: each arm names the same
            // priority slot gen dispatches to. The table is fixed game
            // data, so the fallback never fires on fixture.
            c.g[V1][0] = match c.g[A0][0] as u32 {
                0x003e_5108 => 0x61,
                0x003e_5110 => 0x62,
                0x003e_5118 => 0x63,
                0x003e_5120 => 0x65,
                0x003e_5128 => 0x66,
                0x003e_5130 => 0x67,
                0x003e_5138 => 0x67,
                t => {
                    c.jump_out(t, 0x003e_50c8, 0x003e_519c);
                    return Ok(());
                }
            };
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
            let hook = 0x0042_3ba0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5160)?;
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
            let hook = 0x0042_3bc0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5184)?;
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
                let hook = 0x0042_3c90;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_51c4)?;
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
                c.g[V1][0] = 0x450000;
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
                let hook = 0x0042_3cb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_51ec)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e5204;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x003e_5200);
            c.g[A1][0] = c.g[SP][0];
            let hook = 0x0042_3ca0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5200)?;
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
                        let hook = 0x0042_3c00;
                        c.pc = hook;
                        call_ptr(c, hook, 0x003e_5220)?;
                        break 'b_3e5230;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003e_5230);
                    c.g[A0][0] = c.g[S0][0];
                    let hook = 0x0042_3bf0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_5230)?;
                    break 'b_3e5230;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_5238);
                c.g[A0][0] = c.g[S0][0];
                let hook = 0x0042_3bb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_5238)?;
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
            let hook = 0x0042_3bb0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5254)?;
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
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5278);
        callee::EA_THREAD_DESTROY.call(c, 0x003e_5278)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EAThread_WakeupAlarm(arg)` (0x3e5290): wake the alarm thread.
pub fn ea_thread_wakeup_alarm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_52a0);
        c.g[A0][0] = c.g[A2][0];
        let hook = 0x0042_4c50;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_52a0)?;
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
        let hook = 0x0042_3c90;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_5454)?;
        let this = c.g[S0][0] != 0;
        c.g[A0][0] = c.g[V0][0];
        'b_3e5484: {
            'b_3e547c: {
                'b_3e5468: {
                    if this {
                        break 'b_3e5468;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = 0x450000;
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
                c.g[V0][0] = 0x450000;
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
            let hook = 0x0042_3c90;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_54c8)?;
            c.g[A0][0] = c.g[V0][0];
            break 'b_3e54cc;
        }
        'b_3e54f8: {
            'b_3e54ec: {
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V1][0] == 0;
                if no_main {
                    break 'b_3e54ec;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_54e4);
                c.g[A1][0] = c.g[SP][0];
                let hook = 0x0042_3cb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_54e4)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                break 'b_3e54f8;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_54f4);
            c.g[A1][0] = c.g[SP][0];
            let hook = 0x0042_3ca0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_54f4)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
            break 'b_3e54f8;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `u32 EAThread_WaitForExit(thread, timeout)` (0x3e55e0): wait for
/// the thread to finish, or until `timeout` ticks elapse.
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
                    let hook = 0x0042_3ca0;
                    c.pc = hook;
                    call_ptr(c, hook, 0x003e_5618)?;
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
                    let done = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if done {
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
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5688);
        let hook = 0x0042_3be0;
        c.pc = hook;
        call_ptr(c, hook, 0x003e_5688)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EATimer_ClearCallbacks()` (0x3e57d0): drop all timer
/// callbacks.
pub fn ea_timer_clear_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x520000;
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
        c.g[V0][0] = 0x520000;
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
                c.g[V0][0] = 0x450000;
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
            let hook = 0x0042_3c90;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_5540)?;
            c.g[A0][0] = c.g[V0][0];
            break 'b_3e5544;
        }
        'b_3e5570: {
            'b_3e5564: {
                c.cycles += 4;
                c.g[V0][0] = 0x450000;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xde4));
                let no_main = c.g[V1][0] == 0;
                if no_main {
                    break 'b_3e5564;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_555c);
                c.g[A1][0] = c.g[S0][0];
                let hook = 0x0042_3c40;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_555c)?;
                frame.restore(c, S0, 0x10);
                break 'b_3e5570;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_556c);
            c.g[A1][0] = c.g[S0][0];
            let hook = 0x0042_3c30;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_556c)?;
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
        c.g[V1][0] = 0x450000;
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
                let hook = 0x0042_3cb0;
                c.pc = hook;
                call_ptr(c, hook, 0x003e_55a0)?;
                c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                break 'b_3e55b8;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x003e_55b4);
            c.g[A1][0] = c.g[SP][0];
            let hook = 0x0042_3ca0;
            c.pc = hook;
            call_ptr(c, hook, 0x003e_55b4)?;
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
/// until `ticks` elapse; returns the last nonzero callback result.
pub fn ea_timer_service_for(c: &mut Cpu) {
    guest(c, |c| {
        const HEAD_38: u32 = 0x003e_5a38;
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003e_5a2c);
        c.g[S1][0] = 0;
        callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5a2c)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        'service: loop {
            // Head 0x3e5a44: re-read the clock, exit on expiry or result.
            c.cycles += 5;
            c.g[RA][0] = sx(0x003e_5a4c);
            callee::EA_TIMER_GET_TICKS.call(c, 0x003e_5a4c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            let expired = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[S1][0];
            if expired {
                break 'service;
            }
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                break 'service;
            }
            // Heads 0x3e5a34/0x3e5a38: pump the callbacks once.
            // (Gen's 5a34 `+1` is dead: the entry always jumps to 5a44.)
            c.cycles += 3;
            c.g[RA][0] = sx(0x003e_5a40);
            c.g[A0][0] = 0;
            callee::EA_TIMER_SERVICE_CALLBACKS.call(c, 0x003e_5a40)?;
            c.g[S1][0] = c.g[V0][0];
            if event_due(c) {
                deliver_event_here(c, HEAD_38);
            }
            continue 'service;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
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
            c.g[V1][0] = 0x450000;
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
            c.g[V1][0] = 0x450000;
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
        c.g[V1][0] = 0x450000;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::events::driven;
    use crate::difftest::fakes::covered;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    pub(crate) use crate::game::fe::feasyncfile::tests::ea_install_oc1123_fakes;
    pub(crate) use crate::game::fe::feasyncfile::tests::EAF_RET_ENQ;
    pub(crate) use crate::game::fe::feasyncfile::tests::EAF_RET_NEXT;
    pub(crate) use crate::game::lib::eafile::tests::ea_install_oc1124_fakes;
    pub(crate) use crate::game::lib::eafile::tests::put_ascii_3;
    pub(crate) use crate::game::lib::eafile::tests::stream_block;
    pub(crate) use crate::game::lib::eafile::tests::zero_block;
    pub(crate) use crate::game::lib::eafile::tests::EAF_REQ;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_ALLOC;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_SETCB;
    pub(crate) use crate::game::lib::eafile::tests::EAF_RET_TYPE;
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
        const TABLE: u32 = DATA + 0x2_0000;
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
                c.bus.write32(0x0051_9ad8, TABLE);
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

    driven!(gen_stream_file_cancel_all, ee::s14::f_003e1110__generated);

    driven!(hand_stream_file_cancel_all, stream_file_cancel_all);

    /// Invalid streams; empty heads, short walks and immediate exits;
    /// always with terminating request lists.
    #[test]
    fn stream_file_cancel_all_matches() {
        check(
            gen_stream_file_cancel_all,
            hand_stream_file_cancel_all,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1110, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream, stream);
                // Request list head: null skips the loop, otherwise a
                // two-node chain ending in a count >= 3.
                let n1 = DATA + 0x500;
                let n2 = DATA + 0x540;
                let head_mode = seed % 3;
                c.bus
                    .write32(stream + 0x64, if head_mode == 0 { 0 } else { n1 });
                c.bus.write32(n1, n2);
                c.bus.write32(n1 + 4, [0u32, 0, 5][head_mode as usize]);
                c.bus.write32(n2, 0);
                c.bus.write32(n2 + 4, 5);
                // Free-list: single link resolving in one step.
                c.bus.write32(stream + 0x5c, EAF_REQ);
                c.bus.write32(EAF_REQ + 0xc, stream + 0x60);
                c.bus.write32(stream + 0x60, stream + 0x60);
                // Buffer table: empty or one bounded entry.
                c.bus
                    .write32(stream + 0x28, [0u32, 1][seed as usize / 3 % 2]);
                c.bus.write32(stream + 0x24, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 8, 0);
                // Cursor words kept in sync so the fix-up walk is skipped
                // (it needs another thread's writes to terminate).
                c.bus.write32(stream + 0x50, 0x9200 + seed % 3);
                c.bus.write32(stream + 0x54, 0x9200 + seed % 3);
                c.bus
                    .write32(stream + 0x38, [2u32, 0x9300][seed as usize / 6 % 2]);
                c.bus
                    .write32(stream + 0x30, [stream, 0][seed as usize / 12 % 2]);
                c.bus.write32(stream + 0x2c, 0x9400 + seed % 3);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_1110;
            },
        );
    }

    driven!(hand_stream_file_cancel_request, stream_file_cancel_request);

    /// Invalid streams; lookup miss/hit; mark, free and match arms;
    /// empty and single-step scan lists.
    #[test]
    fn stream_file_cancel_request_matches() {
        check(
            gen_stream_file_cancel_request,
            hand_stream_file_cancel_request,
            128,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0e90, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 5 == 0) as u32);
                // Self-describing stream: [stream] points back at it, so
                // the validate fake resolves [SP] to the stream.
                c.bus.write32(stream, stream);
                let table = DATA + 0x700;
                let mode = seed % 6;
                // Lookup table: count + one entry slot.
                c.bus.write32(stream + 0x18, if mode == 1 { 0 } else { 1 });
                c.bus.write32(stream + 0x14, table);
                c.bus.write32(table, 0x40 + (seed / 6 % 4));
                c.bus
                    .write32(table + 4, [0u32, 1, 4, 9][seed as usize / 24 % 4]);
                c.bus.write32(table + 0xc, 0);
                c.bus.write32(stream + 0x50, 0x9000 + seed % 3);
                c.bus.write32(stream + 0x5c, 0x9100 + seed % 3);
                c.bus.write32(stream + 0x54, 0x9200 + seed % 5);
                c.bus
                    .write32(stream + 0x28, [0u32, 1][seed as usize / 120 % 2]);
                c.bus.write32(stream + 0x24, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 8, 0);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = if mode == 0 {
                    0x40 + (seed / 240 % 4) as u64
                } else {
                    (seed / 240 % 251) as u64
                };
                c.pc = 0x003e_0e90;
            },
        );
    }

    /// Invalid streams; cursor pairs across the wrap point.
    #[test]
    fn stream_file_get_buffer_size_matches() {
        check(
            gen_stream_file_get_buffer_size,
            hand_stream_file_get_buffer_size,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_1580, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x2c, 0x1000 + (seed / 3 % 7) * 0x100);
                c.bus
                    .write32(stream + 0x34, 0x1000 + (seed / 21 % 7) * 0x100);
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
                ea_install_oc1124_fakes();
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

    /// Invalid streams; empty and populated subscriber lists.
    #[test]
    fn stream_file_get_chunk_request_matches() {
        check(
            gen_stream_file_get_chunk_request,
            hand_stream_file_get_chunk_request,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_15e8, 0);
                ea_install_oc1124_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream, stream);
                c.bus
                    .write32(stream + 0x8, [0u32, 1][seed as usize / 4 % 2]);
                c.bus.write32(stream + 0x50, 0x9500 + seed % 3);
                c.bus.write32(stream + 0x5c, 0x9600 + seed % 3);
                // Subscriber node: +0xc link, +0x4 serial.
                let node = DATA + 0x500;
                c.bus.write32(stream + 0x4, node);
                c.bus.write32(node, 0x1234);
                c.bus
                    .write32(node + 4, [0u32, 1, 2][seed as usize / 12 % 3]);
                c.bus.write32(node + 8, 0x2000 + seed % 5);
                c.bus
                    .write32(node + 0xc, if seed / 36 % 2 == 0 { 0 } else { node });
                c.bus.write32(node + 0x120, 0x9700 + seed % 3);
                c.bus.write32(stream + 0x54, 0x9800 + seed % 3);
                c.g[A0][0] = stream as u64;
                c.pc = 0x003e_15e8;
            },
        );
    }

    driven!(hand_stream_file_get_read_state, stream_file_get_read_state);

    /// Invalid streams; present and absent epoch words.
    #[test]
    fn stream_file_get_read_state_matches() {
        check(
            gen_stream_file_get_read_state,
            hand_stream_file_get_read_state,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_14f8, 0);
                ea_install_oc1124_fakes();
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
                ea_install_oc1124_fakes();
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
                c.g[A1][0] = (seed / 60 % 200) as u64;
                c.pc = 0x003e_16b0;
            },
        );
    }

    /// Past, present and future deadlines.
    #[test]
    fn ea_file_timer_deadline_passed_matches() {
        check(
            gen_ea_file_timer_deadline_passed,
            hand_ea_file_timer_deadline_passed,
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

    driven!(gen_ea_file_timer_delay, ee::s14::f_003e4fb0__generated);

    driven!(hand_ea_file_timer_delay, ea_file_timer_delay);

    /// Zero delay exits at once (larger delays spin on the live
    /// timer and cannot terminate on frozen test ticks).
    #[test]
    fn ea_file_timer_delay_matches() {
        check(
            gen_ea_file_timer_delay,
            hand_ea_file_timer_delay,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4fb0, 0);
                c.g[A0][0] = 0;
                c.pc = 0x003e_4fb0;
            },
        );
    }

    /// Pure cached-word load.
    #[test]
    fn ea_file_timer_get_tick_rate_matches() {
        check(
            gen_ea_file_timer_get_tick_rate,
            hand_ea_file_timer_get_tick_rate,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5008, 0);
                c.bus.write32(0x0045_0dc8, 100 + (seed % 3));
                c.pc = 0x003e_5008;
            },
        );
    }

    driven!(hand_ea_file_timer_set_deadline, ea_file_timer_set_deadline);

    /// Zero and nonzero deltas.
    #[test]
    fn ea_file_timer_set_deadline_matches() {
        check(
            gen_ea_file_timer_set_deadline,
            hand_ea_file_timer_set_deadline,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f08, 0);
                c.g[A0][0] = [0u32, 1, 0x1e0, 0x10000][seed as usize % 4] as u64;
                c.bus.write32(0x0045_0de8, 0);
                c.pc = 0x003e_4f08;
            },
        );
    }

    /// Deadline already past exits without pumping.
    #[test]
    fn ea_file_timer_wait_deadline_matches() {
        check(
            gen_ea_file_timer_wait_deadline,
            hand_ea_file_timer_wait_deadline,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_4f40, 0);
                c.bus.write32(0x0045_0de8, 0);
                c.pc = 0x003e_4f40;
            },
        );
    }

    driven!(gen_ea_thread_create, ee::s14::f_003e50c8__generated);

    driven!(hand_ea_thread_create, ea_thread_create);

    /// Default and tabled priority slots.
    #[test]
    fn ea_thread_create_matches() {
        check(
            gen_ea_thread_create,
            hand_ea_thread_create,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_50c8, 0);
                zero_block(c, DATA + 0x500, 16);
                c.g[A0][0] = sx(DATA + 0x500);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[T1][0] = [0u64, 0x61, 0x64, 0x67][seed as usize % 4];
                c.pc = 0x003e_50c8;
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

    driven!(gen_ea_thread_wait_for_exit, ee::s14::f_003e55e0__generated);

    driven!(hand_ea_thread_wait_for_exit, ea_thread_wait_for_exit);

    /// Dead exits at once, gone exits at once, live with a live deadline
    /// yields once then exits. (Live with an infinite timeout would spin
    /// on frozen test ticks.)
    #[test]
    fn ea_thread_wait_for_exit_matches() {
        check(
            gen_ea_thread_wait_for_exit,
            hand_ea_thread_wait_for_exit,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_55e0, 0);
                zero_block(c, DATA + 0x500, 8);
                c.bus.write32(DATA + 0x504, 0x1234);
                c.g[A0][0] = sx(DATA + 0x500);
                let status = [0x10u32, 0, 0x20][seed as usize % 3];
                // Status word the ReferThreadStatus fake leaves behind.
                let sp = c.g[SP][0] as u32;
                c.bus.write32(sp - 0x70, status);
                c.g[A1][0] = if status == 0x20 { 100 } else { 0 };
                c.pc = 0x003e_55e0;
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
                    c.bus.write32(TABLE + 4 * i, 0xdead_0000 + seed + i);
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

    /// Fn in slot 0, in a later slot, or absent.
    #[test]
    fn ea_timer_remove_callback_matches() {
        check(
            gen_ea_timer_remove_callback,
            hand_ea_timer_remove_callback,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_58c8, 0);
                zero_block(c, TABLE, 64);
                let f = DATA + 0x400;
                put_ascii_3(c, "cb", f);
                match seed % 3 {
                    0 => {
                        c.bus.write32(TABLE, f);
                    }
                    1 => {
                        c.bus.write32(TABLE + 3 * 0x10, f);
                        c.bus.write32(TABLE + 3 * 0x10 + 4, 2);
                    }
                    _ => {}
                }
                c.g[A0][0] = sx(f);
                c.pc = 0x003e_58c8;
            },
        );
    }

    driven!(gen_ea_timer_service_for, ee::s14::f_003e5a10__generated);

    driven!(hand_ea_timer_service_for, ea_timer_service_for);

    /// Zero duration exits at once; a live fake callback that returns
    /// nonzero ends the wait with its result. (A zero-returning callback
    /// would spin on frozen test ticks.)
    #[test]
    fn ea_timer_service_for_matches() {
        check(
            gen_ea_timer_service_for,
            hand_ea_timer_service_for,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_5a10, 0);
                zero_block(c, TABLE, 64);
                c.bus.write32(0x0045_0dd0, 1000);
                if seed % 2 == 0 {
                    c.g[A0][0] = 0;
                } else {
                    // Due callback through the miss-hook fake.
                    c.bus.write32(TABLE, FAKE);
                    c.bus.write32(TABLE + 4, 1);
                    c.bus.write32(TABLE + 8, 0);
                    c.g[A0][0] = 0x1_0000;
                }
                c.pc = 0x003e_5a10;
            },
        );
    }

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
                c.g[A1][0] = (seed / 17 % 300) as u64;
                c.g[A2][0] = (seed / 5100 % 8) as u64;
                c.pc = 0x003e_06b0;
            },
        );
    }

    driven!(gen_stream_file_create, ee::s14::f_003e06d8__generated);

    driven!(hand_stream_file_create, stream_file_create);

    /// Valid configs lay out the pool and report the entries base;
    /// short channels, oversized counts, tiny budgets and inverted
    /// ranges take the zero exit.
    #[test]
    fn stream_file_create_matches() {
        check(
            gen_stream_file_create,
            hand_stream_file_create,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_06d8, 0);
                let stream = DATA + 0x400;
                for i in 0..256u32 {
                    c.bus.write32(stream + 4 * i, 0);
                }
                let (ch, s2, s3, s5) = match seed % 8 {
                    4 => (0, 4, 2, 0x20000),
                    5 => (300, 4, 2, 0x20000),
                    6 => (3, 4, 2, 0),
                    7 => (3, 1, 2, 0x20000),
                    _ => (2 + seed % 4, 4, 2, 0x20000),
                };
                c.g[A0][0] = ch as u64;
                c.g[A1][0] = s2 as u64;
                c.g[A2][0] = s3 as u64;
                c.g[A3][0] = stream as u64;
                c.g[T0][0] = s5 as u64;
                c.pc = 0x003e_06d8;
            },
        );
    }

    driven!(gen_stream_file_destroy, ee::s14::f_003e0a28__generated);

    driven!(hand_stream_file_destroy, stream_file_destroy);

    /// Invalid streams (early exit); live streams with an empty V0 slot
    /// (the wait loop needs another thread, so it stays uncovered).
    #[test]
    fn stream_file_destroy_matches() {
        check(
            gen_stream_file_destroy,
            hand_stream_file_destroy,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0a28, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.g[A0][0] = (seed / 3 % 251) as u64;
                c.pc = 0x003e_0a28;
            },
        );
    }

    driven!(hand_stream_file_get_channel_id, stream_file_get_channel_id);

    /// Invalid streams; present and absent channel ids.
    #[test]
    fn stream_file_get_channel_id_matches() {
        check(
            gen_stream_file_get_channel_id,
            hand_stream_file_get_channel_id,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0c58, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus
                    .write32(stream + 0x80 + 0x4, [0u32, 0x42][seed as usize / 3 % 2]);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = 0;
                c.pc = 0x003e_0c58;
            },
        );
    }

    driven!(gen_stream_file_get_channel, ee::s14::f_003e0bf8__generated);

    driven!(hand_stream_file_get_channel, stream_file_get_channel);

    /// Invalid streams; empty and populated channel tables.
    #[test]
    fn stream_file_get_channel_matches() {
        check(
            gen_stream_file_get_channel,
            hand_stream_file_get_channel,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0bf8, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus
                    .write32(stream + 0x28, [0u32, 1, 4][seed as usize / 4 % 3]);
                c.bus.write32(stream + 0x24, 0x8000 + seed % 5);
                c.bus.write32(stream + 0x1c, 0x8100 + seed % 5);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = [0u32, 1, 4, 9][seed as usize / 16 % 4] as u64;
                c.pc = 0x003e_0bf8;
            },
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
                c.g[A1][0] = (seed / 64 % 300) as u64;
                c.pc = 0x003d_fdd0;
            },
        );
    }

    driven!(gen_stream_file_queue_file, ee::s14::f_003e0d80__generated);

    driven!(hand_stream_file_queue_file, stream_file_queue_file);

    /// Invalid streams; null and live alloc results; matching and
    /// scanning channel lists; empty and live subscriber slots.
    #[test]
    fn stream_file_queue_file_matches() {
        check(
            gen_stream_file_queue_file,
            hand_stream_file_queue_file,
            128,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0d80, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 5 == 0) as u32);
                c.bus
                    .write32(EAF_RET_ALLOC, if seed % 5 == 1 { 0 } else { EAF_REQ });
                c.bus.write32(EAF_RET_ENQ, (seed / 5 % 2) as u32);
                c.bus.write32(EAF_RET_NEXT, (seed / 10 % 2) as u32);
                c.bus.write32(EAF_RET_TYPE, (seed / 20 % 2) as u32);
                c.bus.write32(EAF_RET_SETCB, (seed / 40 % 2) as u32);
                // Channel list walk starts at the stream word: [stream] is
                // the head id, [stream+4] links to a node holding the
                // target, so the walk terminates in at most one step
                // (a zero link with a mismatched head would spin the
                // fixed-word loop identically on both sides).
                let head = DATA + 0x500;
                c.bus.write32(stream, head);
                let target = if seed / 1920 % 2 == 0 { 0x1234 } else { 0x5678 };
                c.bus
                    .write32(head, if seed / 80 % 2 == 0 { target } else { 0x5678 });
                c.bus.write32(stream + 4, 0x20);
                c.bus.write32(stream + 0x20, target);
                c.bus.write32(EAF_REQ, 0);
                c.bus.write32(EAF_REQ + 4, 0);
                c.bus.write32(0x0051_9ae4, 0);
                c.g[A0][0] = 0;
                c.g[A1][0] = stream as u64;
                c.g[A2][0] = [0u32, 0x10, 0x1234][seed as usize / 640 % 3] as u64;
                c.g[A3][0] = target as u64;
                c.pc = 0x003e_0d80;
            },
        );
    }

    /// Invalid streams, full and mismatched tables, set and unset slots.
    #[test]
    fn stream_file_set_channel_filter_matches() {
        check(
            gen_stream_file_set_channel_filter,
            hand_stream_file_set_channel_filter,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0948, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 5 == 0) as u32);
                c.bus
                    .write32(stream + 0x20, [0u32, 1, 4, 9][seed as usize / 5 % 4]);
                c.bus.write32(stream + 0x28, 0x10 + (seed / 20 % 4));
                c.bus.write32(stream + 0x1c, 0x5000 + seed % 7);
                c.bus.write32(stream + 0x4, 0x6000 + seed % 3);
                c.bus.write32(stream + 0x38, 0x7000 + seed % 3);
                c.g[A0][0] = (seed / 80 % 251) as u64;
                c.g[A1][0] = (seed / 320 % 9) as u64;
                c.g[A2][0] = (0x9000 + seed % 5) as u64;
                c.g[A3][0] = (0xa000 + seed % 5) as u64;
                c.g[T0][0] = [0i64 as u64, 1, 0xffff_ffff, 5][seed as usize / 1440 % 4];
                c.pc = 0x003e_0948;
            },
        );
    }

    driven!(hand_stream_file_set_low_water, stream_file_set_low_water);

    /// Invalid streams; low-water marks above and below the starved and
    /// current levels (the re-arm path calls back into SetStarved).
    #[test]
    fn stream_file_set_low_water_matches() {
        check(
            gen_stream_file_set_low_water,
            hand_stream_file_set_low_water,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0b28, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                // [SP] stream's level words: +0x44 current, +0x4c starved.
                c.bus
                    .write32(stream + 0x44, [0u32, 5, 10][seed as usize / 4 % 3]);
                c.bus
                    .write32(stream + 0x4c, [0u32, 5, 10][seed as usize / 12 % 3]);
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = [0u32, 5, 10][seed as usize / 36 % 3] as u64;
                c.pc = 0x003e_0b28;
            },
        );
    }

    driven!(gen_stream_file_set_priority, ee::s14::f_003e0ad8__generated);

    driven!(hand_stream_file_set_priority, stream_file_set_priority);

    /// Invalid streams and stored priority pairs.
    #[test]
    fn stream_file_set_priority_matches() {
        check(
            gen_stream_file_set_priority,
            hand_stream_file_set_priority,
            64,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0ad8, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 3 == 0) as u32);
                c.bus.write32(stream + 0x3c, 0x100 + seed % 3);
                c.bus.write32(stream + 0x40, 0x200 + seed % 3);
                c.g[A0][0] = (seed / 3 % 251) as u64;
                c.g[A1][0] = (seed / 9 % 7) as u64;
                c.g[A2][0] = (seed / 63 % 7) as u64;
                c.pc = 0x003e_0ad8;
            },
        );
    }

    driven!(gen_stream_file_set_starved, ee::s14::f_003e0b98__generated);

    driven!(hand_stream_file_set_starved, stream_file_set_starved);

    /// Invalid streams; cleared and set flags; live and missing priority
    /// callbacks (null targets miss identically on both sides).
    #[test]
    fn stream_file_set_starved_matches() {
        check(
            gen_stream_file_set_starved,
            hand_stream_file_set_starved,
            96,
            &|c, seed| {
                let _r = machine(c, seed, 0x3e_0b98, 0);
                ea_install_oc1123_fakes();
                let stream = DATA + 0x400;
                stream_block(c, stream);
                c.bus.write32(EAF_RET_VALID, (seed % 4 == 0) as u32);
                c.bus.write32(stream + 0x38, (seed / 4 % 2) as u32);
                c.bus.write32(stream + 0x40, 0x300 + seed % 3);
                c.bus.write32(
                    stream + 0x174,
                    if seed / 12 % 2 == 0 { 0 } else { 0x00c0_0000 },
                );
                c.g[A0][0] = stream as u64;
                c.g[A1][0] = (seed / 24 % 3) as u64;
                c.pc = 0x003e_0b98;
            },
        );
    }

    /// The timer callback table (16 slots of fn/mode/due/fC).
    pub(crate) const TABLE: u32 = 0x0051_ed98;
    driven!(
        gen_ea_file_timer_deadline_passed,
        ee::s14::f_003e4f80__generated
    );

    driven!(
        gen_ea_file_timer_get_tick_rate,
        ee::s14::f_003e5008__generated
    );

    driven!(
        gen_ea_file_timer_set_deadline,
        ee::s14::f_003e4f08__generated
    );

    driven!(
        gen_ea_file_timer_wait_deadline,
        ee::s14::f_003e4f40__generated
    );

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
        gen_stream_file_get_chunk_request,
        ee::s14::f_003e15e8__generated
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
        gen_stream_file_set_channel_filter,
        ee::s14::f_003e0948__generated
    );

    driven!(
        gen_stream_file_set_low_water,
        ee::s14::f_003e0b28__generated
    );

    driven!(
        hand_ea_file_timer_deadline_passed,
        ea_file_timer_deadline_passed
    );

    driven!(
        hand_ea_file_timer_get_tick_rate,
        ea_file_timer_get_tick_rate
    );

    driven!(
        hand_ea_file_timer_wait_deadline,
        ea_file_timer_wait_deadline
    );

    driven!(
        hand_stream_file_get_buffer_size,
        stream_file_get_buffer_size
    );

    driven!(
        hand_stream_file_get_buffered_bytes,
        stream_file_get_buffered_bytes
    );

    driven!(
        hand_stream_file_get_chunk_request,
        stream_file_get_chunk_request
    );

    driven!(
        hand_stream_file_get_request_status,
        stream_file_get_request_status
    );

    driven!(
        hand_stream_file_set_channel_filter,
        stream_file_set_channel_filter
    );
}
