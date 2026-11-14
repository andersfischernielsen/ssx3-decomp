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
}
