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
];

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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::Rng;
    use crate::difftest::DATA;

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

    use crate::difftest::fakes::covered;

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
}
