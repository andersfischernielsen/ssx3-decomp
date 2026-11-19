//! Lobby list storage and entry operations.

use crate::abi::{addu, r32, ret, set_word};
use crate::call::{Callee, Frame, call_ptr, guest};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::Cpu;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003e_e558,
        lobby_list_get_visible_count,
        "Lobby_ListGetVisibleCount",
    ),
    (0x003e_e550, lobby_list_get_count, "Lobby_ListGetCount"),
    (0x003e_e480, lobby_list_remove, "Lobby_ListRemove"),
    (0x003e_e468, lobby_list_get_data, "Lobby_ListGetData"),
    (0x003e_e3e0, lobby_list_update, "Lobby_ListUpdate"),
    (0x003e_e320, lobby_list_add, "Lobby_ListAdd"),
    (0x003e_e260, lobby_list_destroy, "Lobby_ListDestroy"),
    (0x003e_e1e8, lobby_list_create, "Lobby_ListCreate"),
    (
        0x003e_e1b0,
        lobby_list_compare_thunk,
        "Lobby_ListCompareThunk",
    ),
    (0x003e_e028, lobby_list_grow, "Lobby_ListGrow"),
];

mod callee {
    use super::{Callee, ee};

    pub(super) use crate::game::lib::libc::callee::MEMCPY;
    pub(super) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(super) const LOBBY_MEM_ALLOC: Callee = Callee {
        addr: 0x0025_91e8,
        f: ee::s06::Lobby_MemAlloc,
    };
    pub(super) const LOBBY_MEM_FREE: Callee = Callee {
        addr: 0x0025_9210,
        f: ee::s06::Lobby_MemFree,
    };
    pub(super) const LOBBY_LIST_GROW: Callee = Callee {
        addr: 0x003e_e028,
        f: ee::s14::Lobby_ListGrow,
    };
}

const LIST_INITIAL_SIZE: u32 = 0x00;
const LIST_GROW_PERCENT: u32 = 0x04;
const LIST_SORT_DIRTY: u32 = 0x08;
const LIST_FREE_COUNT: u32 = 0x10;
const LIST_VISIBLE_COUNT: u32 = 0x14;
const LIST_ENTRY_COUNT: u32 = 0x18;
const LIST_FILTER_ARG1: u32 = 0x1c;
const LIST_FILTER_ARG0: u32 = 0x20;
const LIST_FILTER: u32 = 0x24;
const LIST_SORT_ARG1: u32 = 0x28;
const LIST_SORT_ARG0: u32 = 0x2c;
const LIST_COMPARE: u32 = 0x30;
const LIST_FREE_HEAD: u32 = 0x38;
const LIST_CHUNKS: u32 = 0x3c;
const LIST_ENTRIES: u32 = 0x40;
const LIST_HEADER_BYTES: u64 = 0x44;
const LIST_ENTRY_FRAME: u32 = 0x40;
const LIST_STORAGE_FRAME: u32 = 0x50;
const LIST_SAVE_RA: u32 = 0;
const LIST_ENTRY_SAVE_S0: u32 = 0x30;
const LIST_ENTRY_SAVE_S1: u32 = 0x20;
const LIST_ENTRY_SAVE_S2: u32 = 0x10;
const LIST_STORAGE_SAVE_S0: u32 = 0x40;
const LIST_STORAGE_SAVE_S1: u32 = 0x30;
const LIST_STORAGE_SAVE_S2: u32 = 0x20;
const LIST_STORAGE_SAVE_S3: u32 = 0x10;
const LIST_ENTRY_SHIFT: u32 = LIST_ENTRY_POINTER_BYTES.trailing_zeros();
const LIST_ENTRY_POINTER_BYTES: u32 = 4;

/// Returns the number of visible entries in the lobby list.
pub fn lobby_list_get_visible_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(LIST_VISIBLE_COUNT).lw(c);
    ret(c);
}

/// Returns the total number of entries in the lobby list.
pub fn lobby_list_get_count(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = Addr(r32(c, A0)).off(LIST_ENTRY_COUNT).lw(c);
    ret(c);
}

/// Unlinks the requested lobby entry and returns its data.
pub fn lobby_list_remove(c: &mut Cpu) {
    const SEARCH_HEAD: u32 = 0x003e_e4a8;
    const SHIFT_HEAD: u32 = 0x003e_e508;
    c.cycles += 4;
    c.g[A3][0] = c.g[A0][0];
    c.g[A0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
    c.g[A2][0] = 0;
    let hit_first = if (c.g[A0][0] as i64) > 0 {
        c.cycles += 4;
        c.g[V1][0] = Addr(r32(c, A3)).off(LIST_ENTRIES).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
        if c.g[V0][0] == c.g[A1][0] {
            true
        } else {
            c.cycles += 2;
            set_word(c, A2, r32(c, A2).wrapping_add(1));
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                set_word(c, V1, r32(c, V1).wrapping_add(LIST_ENTRY_POINTER_BYTES));
                if c.g[V0][0] == 0 {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                if c.g[V0][0] == c.g[A1][0] {
                    break;
                }
                set_word(c, A2, r32(c, A2).wrapping_add(1));
                if event_due(c) {
                    deliver_event_here(c, SEARCH_HEAD);
                }
            }
            false
        }
    } else {
        false
    };
    if !hit_first {
        c.cycles += 1;
    }
    c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
    c.cycles += 2;
    if c.g[A2][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }

    c.g[V1][0] = Addr(r32(c, A3)).off(LIST_VISIBLE_COUNT).lw(c);
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
    set_word(c, V1, r32(c, V1).wrapping_sub(1));
    if c.g[V0][0] != 0 {
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A3)).off(LIST_SORT_DIRTY).lw(c);
        Addr(r32(c, A3)).off(LIST_VISIBLE_COUNT).sw(c, c.g[V1][0]);
        set_word(c, V0, r32(c, V0).wrapping_add(1));
        Addr(r32(c, A3)).off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
    set_word(c, V0, r32(c, V0).wrapping_sub(1));
    c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).sw(c, c.g[V0][0]);
    if c.g[V1][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRIES).lw(c);
        loop {
            c.cycles += 9;
            set_word(c, A0, r32(c, A2) << LIST_ENTRY_SHIFT);
            set_word(c, A2, r32(c, A2).wrapping_add(1));
            set_word(c, A0, r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V1][0] = Addr(r32(c, A0)).off(LIST_ENTRY_POINTER_BYTES).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRY_COUNT).lw(c);
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(r32(c, A3)).off(LIST_ENTRIES).lw(c);
            if event_due(c) {
                deliver_event_here(c, SHIFT_HEAD);
            }
        }
    }
    c.cycles += 8;
    c.g[V1][0] = Addr(r32(c, A3)).off(LIST_FREE_HEAD).lw(c);
    c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[V1][0]);
    Addr(r32(c, A3)).off(LIST_FREE_HEAD).sw(c, c.g[A1][0]);
    c.g[V1][0] = Addr(r32(c, A3)).off(LIST_FREE_COUNT).lw(c);
    set_word(c, V1, r32(c, V1).wrapping_add(1));
    Addr(r32(c, A3)).off(LIST_FREE_COUNT).sw(c, c.g[V1][0]);
    ret(c);
}

/// Returns the data carried by a lobby entry, or zero for a null entry.
pub fn lobby_list_get_data(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0;
    if c.g[A1][0] != 0 {
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, A1)).lw(c);
    }
    c.cycles += 2;
    ret(c);
}

fn lobby_list_close_entry_frame(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, LIST_ENTRY_SAVE_S0);
    frame.restore(c, S1, LIST_ENTRY_SAVE_S1);
    frame.restore(c, S2, LIST_ENTRY_SAVE_S2);
    frame.restore_ra(c, LIST_SAVE_RA);
    frame.close(c);
}

fn lobby_list_close_storage_frame(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, LIST_STORAGE_SAVE_S0);
    frame.restore(c, S1, LIST_STORAGE_SAVE_S1);
    frame.restore(c, S2, LIST_STORAGE_SAVE_S2);
    frame.restore(c, S3, LIST_STORAGE_SAVE_S3);
    frame.restore_ra(c, LIST_SAVE_RA);
    frame.close(c);
}

/// Replaces the entry data and marks the visible list dirty if either value passes the filter.
pub fn lobby_list_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, LIST_ENTRY_FRAME);
        frame.save(c, S0, LIST_ENTRY_SAVE_S0);
        frame.save(c, S1, LIST_ENTRY_SAVE_S1);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, LIST_ENTRY_SAVE_S2);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_FILTER).lw(c);
        c.g[S2][0] = c.g[A1][0];

        let mark = if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
            true
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG0).lw(c);
            c.g[A1][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG1).lw(c);
            let filter = r32(c, V0);
            c.g[A2][0] = Addr(r32(c, S2)).lw(c);
            call_ptr(c, filter, 0x003e_e418)?;
            if (c.g[V0][0] as i64) > 0 {
                c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
                true
            } else {
                c.cycles += 7;
                c.g[V0][0] = Addr(r32(c, S0)).off(LIST_FILTER).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.g[A0][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG0).lw(c);
                let filter = r32(c, V0);
                c.g[A1][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG1).lw(c);
                call_ptr(c, filter, 0x003e_e434)?;
                if (c.g[V0][0] as i64) > 0 {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
                    true
                } else {
                    false
                }
            }
        };
        if mark {
            c.cycles += 3;
            set_word(c, V0, r32(c, V0).wrapping_add(1));
            Addr(r32(c, S0)).off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
        }
        Addr(r32(c, S2)).sw(c, c.g[S1][0]);

        c.cycles += 6;
        lobby_list_close_entry_frame(c, frame);
        Ok(())
    });
}

/// Adds an entry, growing the free list if necessary, and returns its handle.
pub fn lobby_list_add(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, LIST_ENTRY_FRAME);
        frame.save(c, S0, LIST_ENTRY_SAVE_S0);
        frame.save(c, S2, LIST_ENTRY_SAVE_S2);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, LIST_ENTRY_SAVE_S1);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_FREE_HEAD).lw(c);
        c.g[S2][0] = c.g[A1][0];

        if c.g[V0][0] == 0 {
            c.cycles += 4;
            callee::LOBBY_LIST_GROW.call(c, 0x003e_e34c)?;
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.cycles += 6;
                lobby_list_close_entry_frame(c, frame);
                return Ok(());
            }
        } else {
            c.cycles += 1;
        }
        c.g[S1][0] = Addr(r32(c, S0)).off(LIST_FREE_HEAD).lw(c);

        c.cycles += 16;
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        Addr(r32(c, S0)).off(LIST_FREE_HEAD).sw(c, c.g[V0][0]);
        Addr(r32(c, S1)).sw(c, c.g[S2][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_ENTRY_COUNT).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(LIST_ENTRIES).lw(c);
        set_word(c, V0, r32(c, V0) << LIST_ENTRY_SHIFT);
        c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
        Addr(r32(c, V0)).sw(c, c.g[S1][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_ENTRY_COUNT).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(LIST_FREE_COUNT).lw(c);
        set_word(c, V0, r32(c, V0).wrapping_add(1));
        c.g[A3][0] = Addr(r32(c, S0)).off(LIST_FILTER).lw(c);
        set_word(c, V1, r32(c, V1).wrapping_sub(1));
        Addr(r32(c, S0)).off(LIST_ENTRY_COUNT).sw(c, c.g[V0][0]);
        Addr(r32(c, S0)).off(LIST_FREE_COUNT).sw(c, c.g[V1][0]);
        let mark = if c.g[A3][0] == 0 {
            true
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG0).lw(c);
            c.g[A1][0] = Addr(r32(c, S0)).off(LIST_FILTER_ARG1).lw(c);
            let filter = r32(c, A3);
            c.g[A2][0] = Addr(r32(c, S1)).lw(c);
            call_ptr(c, filter, 0x003e_e3b0)?;
            let visible = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = c.g[S1][0];
            visible
        };
        if mark {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S0)).off(LIST_SORT_DIRTY).lw(c);
            set_word(c, V0, r32(c, V0).wrapping_add(1));
            Addr(r32(c, S0)).off(LIST_SORT_DIRTY).sw(c, c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }

        c.cycles += 6;
        lobby_list_close_entry_frame(c, frame);
        Ok(())
    });
}

/// Releases the list's entry chunks, index array, and header.
pub fn lobby_list_destroy(c: &mut Cpu) {
    const DESTROY_HEAD: u32 = 0x003e_e280;
    const DESTROY_FRAME: u32 = 0x20;
    const DESTROY_SAVE_S0: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, DESTROY_FRAME);
        frame.save(c, S0, DESTROY_SAVE_S0);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
            loop {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                Addr(r32(c, S0)).off(LIST_CHUNKS).sw(c, c.g[V0][0]);
                callee::LOBBY_MEM_FREE.call(c, 0x003e_e28c)?;
                c.g[V0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S0)).off(LIST_CHUNKS).lw(c);
                if event_due(c) {
                    deliver_event_here(c, DESTROY_HEAD);
                }
            }
        }

        c.cycles += 8;
        c.g[A0][0] = Addr(r32(c, S0)).off(LIST_ENTRIES).lw(c);
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e2a0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MEM_FREE.call(c, 0x003e_e2a8)?;
        frame.restore(c, S0, DESTROY_SAVE_S0);
        frame.restore_ra(c, LIST_SAVE_RA);
        frame.close(c);
        Ok(())
    });
}

/// Allocates a lobby list and seeds its first chunk of reusable entries.
pub fn lobby_list_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, LIST_STORAGE_FRAME);
        frame.save(c, S1, LIST_STORAGE_SAVE_S1);
        frame.save(c, S2, LIST_STORAGE_SAVE_S2);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, LIST_STORAGE_SAVE_S3);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, LIST_STORAGE_SAVE_S0);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[A0][0] = LIST_HEADER_BYTES;
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e214)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = LIST_HEADER_BYTES;
        c.g[A0][0] = c.g[S0][0];
        callee::MEMSET.call(c, 0x003e_e228)?;
        Addr(r32(c, S0)).off(LIST_INITIAL_SIZE).sw(c, c.g[S1][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_LIST_GROW.call(c, 0x003e_e234)?;
        Addr(r32(c, S0)).off(LIST_INITIAL_SIZE).sw(c, c.g[S2][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(r32(c, S0)).off(LIST_GROW_PERCENT).sw(c, c.g[S3][0]);
        lobby_list_close_storage_frame(c, frame);
        Ok(())
    });
}

/// Passes entry data and the stored sort arguments to the list's comparator.
pub fn lobby_list_compare_thunk(c: &mut Cpu) {
    const COMPARE_FRAME: u32 = 0x10;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, COMPARE_FRAME);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A3][0] = Addr(r32(c, V0)).lw(c);
        c.g[A1][0] = Addr(r32(c, A0)).off(LIST_SORT_ARG1).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(LIST_COMPARE).lw(c);
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        let comparator = r32(c, V0);
        c.g[A0][0] = Addr(r32(c, A0)).off(LIST_SORT_ARG0).lw(c);
        call_ptr(c, comparator, 0x003e_e1d8)?;
        frame.restore_ra(c, LIST_SAVE_RA);
        frame.close(c);
        Ok(())
    });
}

/// Allocates and links a chunk of free entries, then grows the list's index array.
pub fn lobby_list_grow(c: &mut Cpu) {
    const PERCENT: u64 = 100;
    const CHUNK_WORDS_OVERHEAD: u32 = 2;
    const LIST_INDEX_SHIFT: u32 = 3;
    const GROW_HEAD: u32 = 0x003e_e128;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, LIST_STORAGE_FRAME);
        frame.save(c, S2, LIST_STORAGE_SAVE_S2);
        frame.save(c, S0, LIST_STORAGE_SAVE_S0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, LIST_STORAGE_SAVE_S1);
        frame.save(c, S3, LIST_STORAGE_SAVE_S3);
        frame.save_ra(c, LIST_SAVE_RA);
        c.g[A0][0] = Addr(r32(c, S2)).off(LIST_GROW_PERCENT).lw(c);
        if (c.g[A0][0] as i64) > 0 {
            c.g[A1][0] = Addr(r32(c, S2)).off(LIST_INITIAL_SIZE).lw(c);
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
            c.cycles += 3;
            c.g[V1][0] = PERCENT;
            if c.g[A1][0] == 0 {
                c.cycles += 5;
                c.mult(0, V0, V0, V1);
                c.div(0, V0, A0);
                c.g[V0][0] = c.lo[0];
                c.g[S1][0] = c.g[V0][0];
            } else {
                c.cycles += 7;
                c.mult(0, V0, V0, V1);
                c.div(0, V0, A0);
                c.g[V0][0] = c.lo[0];
                c.g[V0][0] = addu(c.g[A1][0], c.g[V0][0]);
                set_word(c, V1, r32(c, V0) >> 31);
                c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
                set_word(c, S1, ((r32(c, V0) as i32) >> 1) as u32);
            }
        } else {
            c.cycles += 5;
            c.g[S1][0] = Addr(r32(c, S2)).off(LIST_INITIAL_SIZE).lw(c);
            c.g[V1][0] = 1;
            c.g[V0][0] = ((c.g[S1][0] as i64) > 0) as u64;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = c.g[V1][0];
            }
        }

        c.cycles += 11;
        c.g[V0][0] = 1;
        c.g[V1][0] = 2;
        c.g[V0][0] = ((c.g[S1][0] as i64) > (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[V1][0];
        }
        set_word(c, S1, r32(c, S1).wrapping_add(CHUNK_WORDS_OVERHEAD));
        set_word(c, S3, r32(c, S1) << LIST_ENTRY_SHIFT);
        c.g[A0][0] = c.g[S3][0];
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e0d4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 7;
            lobby_list_close_storage_frame(c, frame);
            return Ok(());
        }
        c.g[A0][0] = c.g[S0][0];

        c.cycles += 14;
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[S3][0];
        c.g[S1][0] = 1;
        callee::MEMSET.call(c, 0x003e_e0f8)?;
        c.g[A0][0] = Addr(r32(c, S2)).off(LIST_CHUNKS).lw(c);
        c.g[V0][0] = u64::MAX;
        c.g[V1][0] = addu(c.g[S3][0], c.g[S0][0]);
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        Addr(r32(c, V1).wrapping_sub(LIST_ENTRY_POINTER_BYTES)).sw(c, c.g[V0][0]);
        Addr(r32(c, S2)).off(LIST_CHUNKS).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(r32(c, S0)).off(LIST_ENTRY_POINTER_BYTES).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            set_word(c, A0, r32(c, S0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
            loop {
                c.cycles += 7;
                c.g[V0][0] = Addr(r32(c, S2)).off(LIST_FREE_HEAD).lw(c);
                Addr(r32(c, A0)).sw(c, c.g[V0][0]);
                Addr(r32(c, S2)).off(LIST_FREE_HEAD).sw(c, c.g[A0][0]);
                set_word(c, A0, r32(c, A0).wrapping_add(LIST_ENTRY_POINTER_BYTES));
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                set_word(c, S1, r32(c, S1).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, GROW_HEAD);
                }
            }
            c.cycles += 1;
        }
        c.g[V0][0] = Addr(r32(c, S2)).off(LIST_FREE_COUNT).lw(c);

        c.cycles += 10;
        c.g[A0][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
        set_word(c, V0, r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = addu(c.g[V0][0], c.g[S1][0]);
        c.g[A0][0] = addu(c.g[A0][0], c.g[V0][0]);
        Addr(r32(c, S2)).off(LIST_FREE_COUNT).sw(c, c.g[V0][0]);
        set_word(c, A0, r32(c, A0) << LIST_INDEX_SHIFT);
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_e164)?;
        c.g[A1][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
        c.g[S0][0] = c.g[V0][0];
        if c.g[A1][0] != 0 {
            c.cycles += 6;
            c.g[A2][0] = Addr(r32(c, S2)).off(LIST_ENTRY_COUNT).lw(c);
            c.g[A0][0] = c.g[S0][0];
            set_word(c, A2, r32(c, A2) << LIST_ENTRY_SHIFT);
            callee::MEMCPY.call(c, 0x003e_e180)?;
            c.g[A0][0] = Addr(r32(c, S2)).off(LIST_ENTRIES).lw(c);
            callee::LOBBY_MEM_FREE.call(c, 0x003e_e188)?;
        }
        c.cycles += 2;
        Addr(r32(c, S2)).off(LIST_ENTRIES).sw(c, c.g[S0][0]);
        set_word(c, V0, r32(c, S1).wrapping_sub(1));
        c.cycles += 7;
        lobby_list_close_storage_frame(c, frame);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{Events, check_events, driven, schedule};
    use crate::difftest::fakes::{fall_back, hook, prev_cell};
    use crate::difftest::{DATA, Rng, check_paths, check_paths_lohi, fill, scramble};
    use ssx3_ee as ee;

    const LIST_TEST_ENTRY: u32 = DATA + 0x100;
    const LIST_TEST_SLOTS: u32 = DATA + 0x200;
    const LIST_TEST_REMOVE: u32 = 0x003e_e480;

    fn lobby_list_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut rng = Rng(0x4c69_7374 ^ u64::from(seed));
        scramble(c, &mut rng);
        fill(c, &mut rng, DATA, 0x100);
        c.g[A0][0] = sx(DATA);
        c.pc = entry;
        rng
    }

    #[test]
    fn lobby_list_get_visible_count_matches() {
        check_paths(
            ee::s14::f_003ee558__generated,
            lobby_list_get_visible_count,
            24,
            &[2],
            &|c, seed| {
                lobby_list_fixture(c, seed, 0x003e_e558);
                c.bus.write32(DATA + LIST_VISIBLE_COUNT, seed);
                c.bus.write32(DATA + LIST_ENTRY_COUNT, seed + 1);
            },
        );
    }

    #[test]
    fn lobby_list_get_count_matches() {
        check_paths(
            ee::s14::f_003ee550__generated,
            lobby_list_get_count,
            24,
            &[2],
            &|c, seed| {
                lobby_list_fixture(c, seed, 0x003e_e550);
                c.bus.write32(DATA + LIST_ENTRY_COUNT, seed);
                c.bus.write32(DATA + LIST_VISIBLE_COUNT, seed + 1);
            },
        );
    }

    fn lobby_list_remove_fixture(
        c: &mut Cpu,
        seed: u32,
        count: u32,
        index: u32,
        visible: u32,
    ) -> Rng {
        let rng = lobby_list_fixture(c, seed, LIST_TEST_REMOVE);
        c.bus.write32(DATA + LIST_ENTRY_COUNT, count);
        c.bus.write32(DATA + LIST_VISIBLE_COUNT, visible);
        c.bus.write32(DATA + LIST_ENTRIES, LIST_TEST_SLOTS);
        c.bus.write32(DATA + LIST_FREE_HEAD, LIST_TEST_ENTRY + 0x40);
        for i in 0..4 {
            c.bus.write32(
                LIST_TEST_SLOTS + i * LIST_ENTRY_POINTER_BYTES,
                LIST_TEST_ENTRY + i * 0x10,
            );
        }
        c.g[A1][0] = sx(LIST_TEST_ENTRY + index * 0x10);
        rng
    }

    #[test]
    fn lobby_list_remove_matches() {
        check_paths(
            ee::s14::f_003ee480__generated,
            lobby_list_remove,
            24,
            &[9, 18, 36, 26, 30, 54, 58, 47, 51],
            &|c, seed| {
                let (count, index) = match seed % 6 {
                    0 => (0, 0),
                    1 => (1, 4),
                    2 => (4, 4),
                    3 => (1, 0),
                    4 => (4, 1),
                    _ => (4, 3),
                };
                let visible = if seed / 6 % 2 == 0 { 0 } else { count };
                lobby_list_remove_fixture(c, seed, count, index, visible);
            },
        );
    }

    driven!(lobby_list_remove_driven, lobby_list_remove);

    #[test]
    fn lobby_list_remove_events() {
        check_events(
            ee::s14::f_003ee480__generated,
            lobby_list_remove_driven,
            12,
            &|c, seed| {
                let (index, delay) = if seed / 3 % 2 == 0 { (3, 14) } else { (0, 30) };
                let mut rng = lobby_list_remove_fixture(c, seed, 4, index, 4);
                let mode = schedule(c, &mut rng, seed, LIST_TEST_REMOVE);
                if mode != Events::Off {
                    c.next_event = c.cycles + delay;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_list_get_data_matches() {
        check_paths(
            ee::s14::f_003ee468__generated,
            lobby_list_get_data,
            24,
            &[4, 5],
            &|c, seed| {
                lobby_list_fixture(c, seed, 0x003e_e468);
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(LIST_TEST_ENTRY)
                };
            },
        );
    }

    const LIST_TEST_UPDATE: u32 = 0x003e_e3e0;
    const LIST_TEST_ADD: u32 = 0x003e_e320;
    const LIST_TEST_DESTROY: u32 = 0x003e_e260;
    const LIST_TEST_CREATE: u32 = 0x003e_e1e8;
    const LIST_TEST_COMPARE_THUNK: u32 = 0x003e_e1b0;
    const LIST_TEST_GROW: u32 = 0x003e_e028;
    const LIST_TEST_FILTER: u32 = 0x0041_65a8;
    const LIST_TEST_GATE: u32 = DATA + 0x5_4000;
    const LIST_TEST_MAGIC: u32 = 0x1e82_65d1;
    const LIST_TEST_CALLBACKS: u32 = LIST_TEST_GATE + 4;
    const LIST_TEST_UNWIND_AT: u32 = LIST_TEST_GATE + 8;
    const LIST_TEST_ALLOCS: u32 = LIST_TEST_GATE + 12;
    const LIST_TEST_ALLOC_RESULTS: u32 = LIST_TEST_GATE + 16;
    const LIST_TEST_FREED: u32 = LIST_TEST_GATE + 28;
    const LIST_TEST_CHUNK: u32 = DATA + 0x180;
    const LIST_TEST_NEW_CHUNK: u32 = DATA + 0x5_4100;
    const LIST_TEST_NEW_SLOTS: u32 = DATA + 0x5_4200;
    const LIST_TEST_NEW_LIST: u32 = DATA + 0x5_4300;

    fn lobby_list_test_callback(c: &mut Cpu) {
        if c.bus.read32(LIST_TEST_GATE) != LIST_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_list_test_callback),
                ee::s15::f_004165a8__generated,
            );
        }
        let index = c.bus.read32(LIST_TEST_CALLBACKS);
        c.bus.write32(LIST_TEST_CALLBACKS, index + 1);
        c.cycles += 3;
        c.g[V0][0] = c.g[A2][0];
        if index == c.bus.read32(LIST_TEST_UNWIND_AT) {
            c.pc = 0xdead_0040;
        } else {
            ret(c);
        }
    }

    fn lobby_list_test_alloc(c: &mut Cpu) {
        if c.bus.read32(LIST_TEST_GATE) != LIST_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_list_test_alloc),
                ee::s10::f_00317e30__generated,
            );
        }
        let index = c.bus.read32(LIST_TEST_ALLOCS);
        c.bus.write32(LIST_TEST_ALLOCS, index + 1);
        c.g[V0][0] = sx(c
            .bus
            .read32(LIST_TEST_ALLOC_RESULTS + index * LIST_ENTRY_POINTER_BYTES));
        c.cycles += 1;
        ret(c);
    }

    fn lobby_list_test_free(c: &mut Cpu) {
        if c.bus.read32(LIST_TEST_GATE) != LIST_TEST_MAGIC {
            return fall_back(
                c,
                prev_cell(lobby_list_test_free),
                ee::s10::f_00317e98__generated,
            );
        }
        let ptr = r32(c, A0);
        c.bus.write32(LIST_TEST_FREED, ptr);
        c.cycles += 1;
        ret(c);
    }

    fn lobby_list_test_hooks() {
        use std::ptr::addr_of_mut;
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            lobby_list_test_callback,
            prev_cell(lobby_list_test_callback),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobby_list_test_alloc,
            prev_cell(lobby_list_test_alloc),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobby_list_test_free,
            prev_cell(lobby_list_test_free),
        );
    }

    fn lobby_list_call_fixture(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let mut rng = lobby_list_fixture(c, seed, entry);
        c.bus.write32(LIST_TEST_GATE, LIST_TEST_MAGIC);
        c.bus.write32(LIST_TEST_UNWIND_AT, u32::MAX);
        c.bus.write32(DATA + LIST_FILTER_ARG0, 0x1234_5678);
        c.bus.write32(DATA + LIST_FILTER_ARG1, 0x8765_4321);
        fill(c, &mut rng, LIST_TEST_NEW_CHUNK, 0x100);
        rng
    }

    fn lobby_list_alloc_results(c: &mut Cpu, results: &[u32]) {
        for (index, &result) in results.iter().enumerate() {
            c.bus.write32(
                LIST_TEST_ALLOC_RESULTS + index as u32 * LIST_ENTRY_POINTER_BYTES,
                result,
            );
        }
    }

    #[test]
    fn lobby_list_update_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee3e0__generated,
            lobby_list_update,
            24,
            &[20, 28, 39, 35, 19, 29],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_UPDATE);
                let filter = seed % 4 != 0;
                let old = if seed % 4 == 1 { 1 } else { u32::MAX };
                let new = if seed % 4 == 2 { 1 } else { 0 };
                c.bus.write32(
                    DATA + LIST_FILTER,
                    if filter { LIST_TEST_FILTER } else { 0 },
                );
                c.bus.write32(LIST_TEST_ENTRY, old);
                c.g[A1][0] = sx(LIST_TEST_ENTRY);
                c.g[A2][0] = sx(new);
                match seed / 4 % 3 {
                    1 => c.bus.write32(LIST_TEST_UNWIND_AT, 0),
                    2 => c.bus.write32(LIST_TEST_UNWIND_AT, 1),
                    _ => {}
                }
            },
        );
    }

    #[test]
    fn lobby_list_add_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee320__generated,
            lobby_list_add,
            24,
            &[36, 45, 41, 35, 177, 67],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_ADD);
                c.bus.write32(DATA + LIST_FREE_HEAD, LIST_TEST_ENTRY);
                c.bus.write32(DATA + LIST_FREE_COUNT, 1);
                c.bus.write32(DATA + LIST_ENTRY_COUNT, 0);
                c.bus.write32(DATA + LIST_ENTRIES, LIST_TEST_SLOTS);
                c.bus.write32(DATA + LIST_CHUNKS, 0);
                c.bus.write32(DATA + LIST_INITIAL_SIZE, 0);
                c.bus.write32(DATA + LIST_GROW_PERCENT, 0);
                c.bus.write32(LIST_TEST_ENTRY, 0);
                c.bus.write32(
                    DATA + LIST_FILTER,
                    if matches!(seed % 6, 1..=3) {
                        LIST_TEST_FILTER
                    } else {
                        0
                    },
                );
                c.g[A1][0] = sx(if seed % 6 == 2 { u32::MAX } else { seed + 1 });
                if seed % 6 == 3 {
                    c.bus.write32(LIST_TEST_UNWIND_AT, 0);
                }
                if seed % 6 >= 4 {
                    c.bus.write32(DATA + LIST_FREE_HEAD, 0);
                    c.bus.write32(DATA + LIST_FREE_COUNT, 0);
                    c.bus.write32(DATA + LIST_ENTRIES, 0);
                    let chunk = if seed % 6 == 4 {
                        LIST_TEST_NEW_CHUNK
                    } else {
                        0
                    };
                    lobby_list_alloc_results(c, &[chunk, LIST_TEST_NEW_SLOTS]);
                }
            },
        );
    }

    fn lobby_list_destroy_fixture(c: &mut Cpu, seed: u32, chunks: u32, slots: bool) -> Rng {
        let rng = lobby_list_call_fixture(c, seed, LIST_TEST_DESTROY);
        c.bus.write32(
            DATA + LIST_CHUNKS,
            if chunks == 0 { 0 } else { LIST_TEST_CHUNK },
        );
        c.bus.write32(
            LIST_TEST_CHUNK,
            if chunks > 1 {
                LIST_TEST_CHUNK + 0x10
            } else {
                0
            },
        );
        c.bus.write32(LIST_TEST_CHUNK + 0x10, 0);
        c.bus
            .write32(DATA + LIST_ENTRIES, if slots { LIST_TEST_SLOTS } else { 0 });
        rng
    }

    #[test]
    fn lobby_list_destroy_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee260__generated,
            lobby_list_destroy,
            12,
            &[30, 33, 46, 49, 61, 64],
            &|c, seed| {
                lobby_list_destroy_fixture(c, seed, seed % 3, seed / 3 % 2 != 0);
            },
        );
    }

    driven!(lobby_list_destroy_driven, lobby_list_destroy);

    #[test]
    fn lobby_list_destroy_events() {
        lobby_list_test_hooks();
        check_events(
            ee::s14::f_003ee260__generated,
            lobby_list_destroy_driven,
            6,
            &|c, seed| {
                let mut rng = lobby_list_destroy_fixture(c, seed, 2, true);
                let mode = schedule(c, &mut rng, seed, LIST_TEST_DESTROY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 22;
                }
                mode
            },
        );
    }

    #[test]
    fn lobby_list_create_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee1e8__generated,
            lobby_list_create,
            12,
            &[159, 251, 272],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_CREATE);
                c.g[A0][0] = if seed % 2 == 0 { 0 } else { 4 };
                c.g[A1][0] = u64::from(seed + 2);
                c.g[A2][0] = u64::from(seed + 10);
                let chunk = if seed / 2 % 2 == 0 {
                    0
                } else {
                    LIST_TEST_NEW_CHUNK
                };
                lobby_list_alloc_results(c, &[LIST_TEST_NEW_LIST, chunk, LIST_TEST_NEW_SLOTS]);
            },
        );
    }

    #[test]
    fn lobby_list_compare_thunk_matches() {
        lobby_list_test_hooks();
        check_paths(
            ee::s14::f_003ee1b0__generated,
            lobby_list_compare_thunk,
            12,
            &[16],
            &|c, seed| {
                lobby_list_call_fixture(c, seed, LIST_TEST_COMPARE_THUNK);
                c.bus.write32(DATA + LIST_SORT_ARG0, 0x1122_3344);
                c.bus.write32(DATA + LIST_SORT_ARG1, 0xaabb_ccdd);
                c.bus.write32(DATA + LIST_COMPARE, LIST_TEST_FILTER);
                c.bus.write32(LIST_TEST_SLOTS, LIST_TEST_ENTRY);
                c.bus.write32(
                    LIST_TEST_SLOTS + LIST_ENTRY_POINTER_BYTES,
                    LIST_TEST_ENTRY + LIST_ENTRY_POINTER_BYTES,
                );
                c.bus.write32(LIST_TEST_ENTRY, seed.wrapping_sub(6));
                c.bus
                    .write32(LIST_TEST_ENTRY + LIST_ENTRY_POINTER_BYTES, seed + 3);
                c.g[A1][0] = sx(LIST_TEST_SLOTS);
                c.g[A2][0] = sx(LIST_TEST_SLOTS + LIST_ENTRY_POINTER_BYTES);
                if seed % 2 != 0 {
                    c.bus.write32(LIST_TEST_UNWIND_AT, 0);
                }
            },
        );
    }

    fn lobby_list_grow_fixture(c: &mut Cpu, seed: u32, kind: u32, fail: bool) -> Rng {
        let rng = lobby_list_call_fixture(c, seed, LIST_TEST_GROW);
        let (initial, percent, count) = match kind {
            0 => (0, 0, 0),
            1 => (4, 0, 0),
            2 => (0, 100, 1),
            _ => (4, 50, 1),
        };
        c.bus.write32(DATA + LIST_INITIAL_SIZE, initial);
        c.bus.write32(DATA + LIST_GROW_PERCENT, percent);
        c.bus.write32(DATA + LIST_ENTRY_COUNT, count);
        c.bus.write32(DATA + LIST_FREE_COUNT, 0);
        c.bus.write32(DATA + LIST_FREE_HEAD, 0);
        c.bus.write32(
            DATA + LIST_CHUNKS,
            if count == 0 { 0 } else { LIST_TEST_CHUNK },
        );
        c.bus.write32(
            DATA + LIST_ENTRIES,
            if count == 0 { 0 } else { LIST_TEST_SLOTS },
        );
        c.bus.write32(LIST_TEST_SLOTS, LIST_TEST_ENTRY);
        let chunk = if fail { 0 } else { LIST_TEST_NEW_CHUNK };
        lobby_list_alloc_results(c, &[chunk, LIST_TEST_NEW_SLOTS]);
        rng
    }

    #[test]
    fn lobby_list_grow_matches() {
        lobby_list_test_hooks();
        check_paths_lohi(
            ee::s14::f_003ee028__generated,
            lobby_list_grow,
            16,
            &[46, 51, 53, 138, 159, 196, 235],
            &|c, seed| {
                lobby_list_grow_fixture(c, seed, seed % 4, seed / 4 % 2 != 0);
            },
        );
    }

    driven!(lobby_list_grow_driven, lobby_list_grow);

    #[test]
    fn lobby_list_grow_events() {
        lobby_list_test_hooks();
        check_events(
            ee::s14::f_003ee028__generated,
            lobby_list_grow_driven,
            6,
            &|c, seed| {
                let mut rng = lobby_list_grow_fixture(c, seed, 0, false);
                let mode = schedule(c, &mut rng, seed, LIST_TEST_GROW);
                if mode != Events::Off {
                    c.next_event = c.cycles + 98;
                }
                mode
            },
        );
    }
}
