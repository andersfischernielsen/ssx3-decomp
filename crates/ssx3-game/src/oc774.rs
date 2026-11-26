//! Staging module oc774 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc774`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_2928,
        cfe_state_char_setup_init,
        "cFEStateCharSetup_Init",
    ),
    (
        0x0020_ccf8,
        game_ui_handle_menu_result,
        "GameUI_HandleMenuResult",
    ),
    (0x003d_4bb0, path_find_map, "PATH_FindMap"),
    (0x003d_4c40, path_sort_maps, "PATH_SortMaps"),
    (0x0020_0700, event_select_init, "EventSelect_Init"),
    (
        0x0028_67e8,
        ssx_audio_system_enter_game,
        "SSXAudioSystem_EnterGame",
    ),
];

/// Map table of the path system: 4 records of 0x928 bytes at 0x517610.
#[cfg(test)]
pub(crate) const PATH_MAP_TABLE: u32 = 0x0051_7610;
/// Size of one map record.
pub(crate) const PATH_MAP_SIZE: u32 = 0x928;
/// Record field: sort key (mask).
pub(crate) const PATH_MAP_MASK: u32 = 0x30;
/// Record field: header pointer; 0 = unused record.
pub(crate) const PATH_MAP_HDR: u32 = 0x34;
/// Globals of `PATH_FindMap`: the found header (0x44f430) and record (0x44f420).
#[cfg(test)]
pub(crate) const PATH_FOUND_HDR: u32 = 0x0044_f430;

/// `int PATH_FindMap(int id)` (0x3d4bb0).
///
/// Looks for the record of the four-entry map table whose `+0x30` word equals
/// `id` and whose `+0x34` word is non-zero. Found: remembers the record in
/// 0x44f420 and its id in 0x44f430, returns the id. Not found: makes sure
/// 0x44f430 is set (to 0xffff_ffff when it was 0), clears it again and returns
/// the (zeroed) 0x44f430.
pub fn path_find_map(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = 0x510000;
    c.g[T0][0] = 0xffff_ffff_ffff_0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7610));
    c.g[A2][0] = 0;
    c.g[A3][0] = c.g[V0][0];
    c.g[T0][0] |= 0xffff;
    c.g[T3][0] = 0x450000;
    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
    c.g[T1][0] = 0x450000;
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(PATH_MAP_MASK));
    loop {
        'next: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0 {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                break 'next;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            if c.g[V1][0] != c.g[A0][0] {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                break 'next;
            }
            c.cycles += 4;
            c.g[A0][0] = 0x450000;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0xbd0));
            if c.g[V0][0] == 0 {
                c.sw((c.g[A0][0] as u32).wrapping_sub(0xbd0), c.g[T0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 0x4u64) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V0][0] = c.g[V1][0];
            if out_of_range {
                c.cycles += 2;
                if c.g[A2][0] != c.g[T2][0] {
                    c.sw((c.g[T1][0] as u32).wrapping_sub(0xbd0), 0);
                }
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_sub(0xbd0));
                ret(c);
                return;
            }
            c.cycles += 3;
            c.sw((c.g[T3][0] as u32).wrapping_sub(0xbe0), c.g[A3][0]);
            c.sw((c.g[A0][0] as u32).wrapping_sub(0xbd0), c.g[V1][0]);
            ret(c);
            return;
        }
        c.cycles += 4;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(PATH_MAP_SIZE));
        c.g[V0][0] = ((c.g[A2][0] as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(PATH_MAP_SIZE));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_4bd8);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
}

/// One 0x20-byte-per-iteration copy loop of `PATH_SortMaps` for aligned
/// pointers (`ld`/`sd`, 11 cycles per round). `t` are the four temporaries,
/// `src`/`dst` the pointer registers (advanced by 0x20 per round), `end` the
/// register holding the final `src`; `head` is the loop's pc.
fn path_sort_copy_aligned(
    c: &mut Cpu,
    t: [usize; 4],
    src: usize,
    dst: usize,
    end: usize,
    head: u32,
) {
    loop {
        c.cycles += 11;
        for (k, r) in t.iter().enumerate() {
            c.g[*r][0] = c.ld((c.g[src][0] as u32).wrapping_add(8 * k as u32));
        }
        for (k, r) in t.iter().enumerate() {
            c.sd((c.g[dst][0] as u32).wrapping_add(8 * k as u32), c.g[*r][0]);
        }
        c.g[src][0] = sx((c.g[src][0] as u32).wrapping_add(0x20));
        let more = c.g[src][0] != c.g[end][0];
        c.g[dst][0] = sx((c.g[dst][0] as u32).wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The same copy loop for unaligned pointers (`ldl`/`ldr`/`sdl`/`sdr`, 19
/// cycles per round).
fn path_sort_copy_unaligned(
    c: &mut Cpu,
    t: [usize; 4],
    src: usize,
    dst: usize,
    end: usize,
    head: u32,
) {
    loop {
        c.cycles += 19;
        for (k, r) in t.iter().enumerate() {
            let a = (c.g[src][0] as u32).wrapping_add(8 * k as u32);
            c.ldl(*r, a.wrapping_add(7));
            c.ldr(*r, a);
        }
        for (k, r) in t.iter().enumerate() {
            let a = (c.g[dst][0] as u32).wrapping_add(8 * k as u32);
            c.sdl(*r, a.wrapping_add(7));
            c.sdr(*r, a);
        }
        c.g[src][0] = sx((c.g[src][0] as u32).wrapping_add(0x20));
        let more = c.g[src][0] != c.g[end][0];
        c.g[dst][0] = sx((c.g[dst][0] as u32).wrapping_add(0x20));
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `void PATH_SortMaps(void)` (0x3d4c40).
///
/// Selection-style sort of the four map records (0x928 bytes each, at
/// 0x517610) by their sort key (`+0x30`, unsigned), skipping records whose
/// header (`+0x34`) is 0: for each used record i, every later used record j
/// with a smaller key is swapped in (copy i to the stack, j to i, the stack to
/// j; each copy is 0x920 bytes of 32-byte rounds plus an 8-byte tail, with
/// separate aligned and unaligned loops as the compiler emitted them).
pub fn path_sort_maps(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x980));
    c.g[V0][0] = 0x510000;
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
    c.g[T4][0] = 0;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x960));
    c.g[S0][0] = PATH_MAP_SIZE as u64;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x950));
    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7644));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x940));
    c.g[T9][0] = 0x510000;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x930));
    c.mult(0, 2, 12, 16);
    loop {
        'next_i: {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let unused = c.g[V0][0] == 0;
            c.g[T3][0] = sx((c.g[T4][0] as u32).wrapping_add(1));
            if unused {
                break 'next_i;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[T3][0] as i64) < 4) as u64;
            let last = c.g[V0][0] == 0;
            c.g[A3][0] = c.g[T3][0];
            if last {
                break 'next_i;
            }
            c.cycles += 7;
            c.g[T2][0] = PATH_MAP_SIZE as u64;
            c.g[T1][0] = sx((c.g[T9][0] as u32).wrapping_add(0x7610));
            c.mult(0, 2, 12, 10);
            c.g[T8][0] = sx((c.g[T1][0] as u32).wrapping_add(PATH_MAP_HDR));
            c.g[T6][0] = sx((c.g[T1][0] as u32).wrapping_add(PATH_MAP_MASK));
            c.g[T7][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.mult(0, 4, 7, 10);
            loop {
                'next_j: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T8][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let unused = c.g[V1][0] == 0;
                    c.g[T5][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    if unused {
                        break 'next_j;
                    }
                    c.cycles += 8;
                    c.mult(0, 5, 12, 10);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T6][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T6][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                    let not_smaller = c.g[A0][0] == 0;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    if not_smaller {
                        break 'next_j;
                    }
                    // Swap record i (v1) and record j: i -> stack.
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[SP][0];
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x920));
                    c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x920));
                    if aligned {
                        path_sort_copy_aligned(c, [S4, A0, A2, S2], V1, A1, V0, 0x003d_4d48);
                        c.cycles += 1;
                    } else {
                        path_sort_copy_unaligned(c, [A0, A2, S2, S3], V1, A1, V0, 0x003d_4cec);
                        c.cycles += 2;
                    }
                    c.mult(0, 2, 7, 10);
                    // Tail of the first copy; set up j -> i.
                    c.cycles += 10;
                    c.g[A2][0] = c.g[T7][0];
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    c.ldl(18, (c.g[V1][0] as u32).wrapping_add(7));
                    c.ldr(18, c.g[V1][0] as u32);
                    c.sdl(18, (c.g[A1][0] as u32).wrapping_add(7));
                    c.sdr(18, c.g[A1][0] as u32);
                    c.g[V0][0] = c.g[A0][0] | c.g[A2][0];
                    c.g[V0][0] &= 7;
                    let aligned = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x920));
                    if aligned {
                        path_sort_copy_aligned(c, [S4, V1, A1, S2], A0, A2, V0, 0x003d_4df4);
                        c.cycles += 1;
                    } else {
                        path_sort_copy_unaligned(c, [V1, A1, S2, S3], A0, A2, V0, 0x003d_4da0);
                        c.cycles += 2;
                    }
                    c.mult(0, 2, 7, 10);
                    // Tail of the second copy; set up stack -> j.
                    c.cycles += 8;
                    c.g[V1][0] = c.g[SP][0];
                    c.ldl(7, (c.g[A0][0] as u32).wrapping_add(7));
                    c.ldr(7, c.g[A0][0] as u32);
                    c.sdl(7, (c.g[A2][0] as u32).wrapping_add(7));
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                    c.g[V0][0] = c.g[A1][0] & 7;
                    let aligned = c.g[V0][0] == 0;
                    c.sdr(7, c.g[A2][0] as u32);
                    if aligned {
                        path_sort_copy_aligned(c, [S2, S3, S4, V0], V1, A1, T0, 0x003d_4e98);
                    } else {
                        path_sort_copy_unaligned(c, [V0, A0, A2, A3], V1, A1, T0, 0x003d_4e44);
                        c.cycles += 2;
                    }
                    c.cycles += 4;
                    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(7));
                    c.ldr(4, c.g[V1][0] as u32);
                    c.sdl(4, (c.g[A1][0] as u32).wrapping_add(7));
                    c.sdr(4, c.g[A1][0] as u32);
                }
                c.cycles += 4;
                c.g[A3][0] = c.g[T5][0];
                c.g[V0][0] = ((c.g[A3][0] as i64) < 4) as u64;
                let more = c.g[V0][0] != 0;
                c.mult(0, 4, 7, 10);
                if !more {
                    break 'next_i;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_4ca8);
                }
            }
        }
        c.cycles += 4;
        c.g[T4][0] = c.g[T3][0];
        c.g[V0][0] = ((c.g[T4][0] as i64) < 4) as u64;
        if c.g[V0][0] != 0 {
            c.mult(0, 2, 12, 16);
            if event_due(c) {
                deliver_event_here(c, 0x003d_4c70);
            }
            continue;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x960));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x950));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x940));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x930));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x980));
        ret(c);
        return;
    }
}

/// `void cFEStateCharSetup::Init(cFEStateCharSetup* this)` (0x182928).
///
/// Opens the "set character" screen of the front end: looks up the screen
/// (hash of its name, 0x317670, then `cUIEngine_addScreenByHashName`, 0x397cc0)
/// into `this + 0x40` and plays its first frame, shows the rider preview
/// (`cFEStateCharSetup_InitRiderPreview`, 0x182db8), reads the player's
/// character id from the back end (interface 1) for the user number at
/// `this + 0x44`, switches the audio mode (0x28f140), and finally looks up the
/// option and title widgets by name (0x39d860) and shows, hides or enables
/// them through their vtables (the `Option_*` buttons set to 0..3, the title
/// text of player 1 or 2 depending on the user number and game path).
pub fn cfe_state_char_setup_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_18297c: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2840));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00182950)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00182964)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_18297c;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x0018297c)?;
                break 'b_18297c;
            }
        }
        'b_182aa8: {
            c.cycles += 44;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00182db8, 0x00182984)?;
            call_ptr(c, 0x0014dc80, 0x0018298c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00182998)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00147488, 0x001829a8)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001829c0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0570, 0x001829d8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0508, 0x001829f4)?;
            call_ptr(c, 0x0028b180, 0x001829fc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0028f140, 0x00182a08)?;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1410));
            call_ptr(c, 0x00317670, 0x00182a14)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182a20)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A0][0] = 0x460000;
            if t {
                break 'b_182aa8;
            } else {
                c.cycles += 10;
                call_ptr(c, 0x0014dc80, 0x00182a34)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00182a40)?;
                c.g[V1][0] = 0x530000;
                c.g[A0][0] = 0x2;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                if c.g[V0][0] != c.g[A0][0] {
                    c.g[A0][0] = 0x460000;
                    break 'b_182aa8;
                } else {
                    'b_182a90: {
                        'b_182a70: {
                            c.cycles += 3;
                            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_182a70;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                if t {
                                    break 'b_182a90;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_182aa8;
                                    } else {
                                        break 'b_182a70;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2830));
                        call_ptr(c, 0x00317670, 0x00182a7c)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x00182a88)?;
                        let t = true;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_182aa8;
                        } else {
                            break 'b_182a90;
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2810));
                    call_ptr(c, 0x00317670, 0x00182a98)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00182aa4)?;
                    c.g[A0][0] = 0x460000;
                    break 'b_182aa8;
                }
            }
        }
        'b_182ae0: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27f0));
            call_ptr(c, 0x00317670, 0x00182ab0)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182abc)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182ae0;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182adc)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                break 'b_182ae0;
            }
        }
        'b_182b20: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27e0));
            call_ptr(c, 0x00317670, 0x00182aec)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182af8)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182b20;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182b18)?;
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_182b20;
            }
        }
        'b_182b60: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27d0));
            call_ptr(c, 0x00317670, 0x00182b2c)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182b38)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182b60;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182b58)?;
                c.g[V0][0] = 0x2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_182b60;
            }
        }
        'b_182ba0: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27c0));
            call_ptr(c, 0x00317670, 0x00182b6c)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182b78)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_182ba0;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182b98)?;
                c.g[V0][0] = 0x3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_182ba0;
            }
        }
        'b_182bf0: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x27b0));
            call_ptr(c, 0x00317670, 0x00182bac)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00182bb8)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_182bf0;
            } else {
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182bd8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00182bf0)?;
                break 'b_182bf0;
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
        return Ok(());
    });
}

/// `void GameUI_HandleMenuResult(void)` (0x20ccf8; the reference calls it
/// `cOVStateManager_gotoNextOverlay`).
///
/// Acts on the result of the overlay menu that just closed: clears the
/// overlay status (`gp - 0x9f4`) and looks at the id of the current UI state
/// (`+0x98`, 8..35, a jump table at 0x471a50) and the menu's `mMiscData`
/// (`gp - 0x9f0`, a second table at 0x471ac0). Depending on both it opens
/// another overlay screen (`GameUI_OpenScreen` with 0x10, 0x11, 0x13, 0x17,
/// 0x18, 0x21, 0x23 or the results screen), stops the replay, starts a game
/// state, leaves the game, or sets the overlay status to 2 or 4.
pub fn game_ui_handle_menu_result(c: &mut Cpu) {
    if c.pc != 0x0020_ccf8 {
        return ee::s05::f_0020ccf8__entries(c);
    }
    guest(c, |c| {
        'b_20cfc0: {
            'b_20cfbc: {
                'b_20cfb8: {
                    c.cycles += 25;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039fa88, 0x0020cd24)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x0020cd2c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0020cd38)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x98));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                    c.g[S1][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x5c12));
                    c.g[V0][0] = (c.g[V1][0] < 0x1cu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = c.lw(c.g[A0][0] as u32);
                    if t {
                        break 'b_20cfb8;
                    } else {
                        'b_20cfb0: {
                            'b_20cf9c: {
                                'b_20cf80: {
                                    'b_20cf44: {
                                        'b_20cf38: {
                                            'b_20cf20: {
                                                'b_20cf1c: {
                                                    'b_20ceb8: {
                                                        'b_20ce60: {
                                                            'b_20ce48: {
                                                                'b_20ce0c: {
                                                                    'b_20cdfc: {
                                                                        'b_20cdcc: {
                                                                            'b_20cda4: {
                                                                                c.cycles += 7;
                                                                                c.g[V0][0] =
                                                                                    0x470000;
                                                                                c.g[V1][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1a50));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                c.g[A0][0] = c
                                                                                    .lw(c.g[V1][0]
                                                                                        as u32);
                                                                                let t = c.g[A0][0]
                                                                                    as u32;
                                                                                match t {
                                                                                    0x0020cd78 => {
                                                                                        'l_20cd78: loop {
                                                                                            c.cycles += 5;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));
                                                                                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                                                            c.g[V1][0] = (c.g[A0][0] < 0x8u64) as u64;
                                                                                            let t = c.g[V1][0] == 0u64;
                                                                                            c.g[V0][0] = 0x470000;
                                                                                            if t {
                                                                                                break 'b_20cfb8;
                                                                                            } else {
                                                                                                c.cycles += 6;
                                                                                                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1ac0));
                                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                let t = c.g[A0][0] as u32;
                                                                                                match t {
0x0020cd78 => {
if event_due(c) { deliver_event_here(c, 0x0020cd78); } continue 'l_20cd78;
}
0x0020cda4 => {
break 'b_20cda4;
}
0x0020cdcc => {
break 'b_20cdcc;
}
0x0020cdfc => {
break 'b_20cdfc;
}
0x0020ce0c => {
break 'b_20ce0c;
}
0x0020ce48 => {
break 'b_20ce48;
}
0x0020ce60 => {
break 'b_20ce60;
}
0x0020ceb8 => {
break 'b_20ceb8;
}
0x0020cf1c => {
break 'b_20cf1c;
}
0x0020cf38 => {
break 'b_20cf38;
}
0x0020cf44 => {
break 'b_20cf44;
}
0x0020cf80 => {
break 'b_20cf80;
}
0x0020cf9c => {
break 'b_20cf9c;
}
0x0020cfb8 => {
break 'b_20cfb8;
}
_ => { c.jump_out(t, 0x0020ccf8, 0x0020cfd0); return Ok(()); }
}
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    0x0020cda4 => {
                                                                                        break 'b_20cda4;
                                                                                    }
                                                                                    0x0020cdcc => {
                                                                                        break 'b_20cdcc;
                                                                                    }
                                                                                    0x0020cdfc => {
                                                                                        break 'b_20cdfc;
                                                                                    }
                                                                                    0x0020ce0c => {
                                                                                        break 'b_20ce0c;
                                                                                    }
                                                                                    0x0020ce48 => {
                                                                                        break 'b_20ce48;
                                                                                    }
                                                                                    0x0020ce60 => {
                                                                                        break 'b_20ce60;
                                                                                    }
                                                                                    0x0020ceb8 => {
                                                                                        break 'b_20ceb8;
                                                                                    }
                                                                                    0x0020cf1c => {
                                                                                        break 'b_20cf1c;
                                                                                    }
                                                                                    0x0020cf38 => {
                                                                                        break 'b_20cf38;
                                                                                    }
                                                                                    0x0020cf44 => {
                                                                                        break 'b_20cf44;
                                                                                    }
                                                                                    0x0020cf80 => {
                                                                                        break 'b_20cf80;
                                                                                    }
                                                                                    0x0020cf9c => {
                                                                                        break 'b_20cf9c;
                                                                                    }
                                                                                    0x0020cfb8 => {
                                                                                        break 'b_20cfb8;
                                                                                    }
                                                                                    _ => {
                                                                                        c.jump_out(t, 0x0020ccf8, 0x0020cfd0);
                                                                                        return Ok(
                                                                                            (),
                                                                                        );
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 10;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x84,
                                                                                ));
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x28,
                                                                                ));
                                                                            call_ptr(
                                                                                c, 0x002706f0,
                                                                                0x0020cdb4,
                                                                            )?;
                                                                            c.g[V0][0] = 0x1;
                                                                            c.g[A0][0] = 0x21;
                                                                            c.sw(
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0xb5c,
                                                                                    ),
                                                                                c.g[V0][0],
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0020ab50,
                                                                                0x0020cdc4,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x30,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_20cfbc;
                                                                            } else {
                                                                                break 'b_20cdcc;
                                                                            }
                                                                        }
                                                                        c.cycles += 12;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[A0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x28));
                                                                        call_ptr(
                                                                            c, 0x002706f0,
                                                                            0x0020cddc,
                                                                        )?;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[A1][0] = 0xd;
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[A3][0] = 0x1;
                                                                        c.g[A0][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        call_ptr(
                                                                            c, 0x00231250,
                                                                            0x0020cdf4,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        if t {
                                                                            break 'b_20cfbc;
                                                                        } else {
                                                                            break 'b_20cdfc;
                                                                        }
                                                                    }
                                                                    c.cycles += 4;
                                                                    c.g[A0][0] = 0x23;
                                                                    call_ptr(
                                                                        c, 0x0020ab50, 0x0020ce04,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x30),
                                                                    );
                                                                    if t {
                                                                        break 'b_20cfbc;
                                                                    } else {
                                                                        break 'b_20ce0c;
                                                                    }
                                                                }
                                                                c.cycles += 15;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                call_ptr(
                                                                    c, 0x002706f0, 0x0020ce1c,
                                                                )?;
                                                                c.g[A0][0] = 0x11;
                                                                call_ptr(
                                                                    c, 0x0020ab50, 0x0020ce24,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A2][0] = 0u64;
                                                                c.g[A3][0] = 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                call_ptr(
                                                                    c, 0x0026f8a0, 0x0020ce40,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_20cfbc;
                                                                } else {
                                                                    break 'b_20ce48;
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[V0][0] = 0x1;
                                                            c.g[A0][0] = 0xf;
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x9f0),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x0020ab50, 0x0020ce58)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_20cfbc;
                                                            } else {
                                                                break 'b_20ce60;
                                                            }
                                                        }
                                                        'b_20cea8: {
                                                            c.cycles += 8;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            c.g[A1][0] = 0x7;
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x78));
                                                            call_ptr(c, 0x0014dd58, 0x0020ce70)?;
                                                            c.g[V1][0] = 0x530000;
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4b30));
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x204));
                                                            if t {
                                                                break 'b_20cea8;
                                                            } else {
                                                                'b_20ce94: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A0][0] == 0u64;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    if t {
                                                                        break 'b_20ce94;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A1][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00257610,
                                                                            0x0020ce90,
                                                                        )?;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        break 'b_20ce94;
                                                                    }
                                                                }
                                                                c.cycles += 5;
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                call_ptr(
                                                                    c, 0x00230488, 0x0020cea0,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_20cfbc;
                                                                } else {
                                                                    break 'b_20cea8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x18;
                                                        call_ptr(c, 0x0020ab50, 0x0020ceb0)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        if t {
                                                            break 'b_20cfbc;
                                                        } else {
                                                            break 'b_20ceb8;
                                                        }
                                                    }
                                                    'b_20ced8: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x9f0));
                                                        c.g[V0][0] = 0x13;
                                                        let t = c.g[V1][0] != c.g[V0][0];

                                                        if t {
                                                            break 'b_20ced8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A0][0] = 0x13;
                                                            call_ptr(c, 0x0020ab50, 0x0020ced0)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_20cfbc;
                                                            } else {
                                                                break 'b_20ced8;
                                                            }
                                                        }
                                                    }
                                                    'b_20cf0c: {
                                                        c.cycles += 10;
                                                        call_ptr(c, 0x0014dc80, 0x0020cee0)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0xd;
                                                        call_ptr(c, 0x0014dd58, 0x0020ceec)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00158f30, 0x0020cef8)?;
                                                        let t = (c.g[V0][0] as i64) > 0;

                                                        if t {
                                                            break 'b_20cf0c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_20cf20;
                                                            } else {
                                                                break 'b_20cf0c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[A0][0] = 0x10;
                                                    call_ptr(c, 0x0020ab50, 0x0020cf14)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_20cfbc;
                                                    } else {
                                                        break 'b_20cf1c;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] = c.g[S1][0];
                                                break 'b_20cf20;
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] = c.g[S2][0];
                                            call_ptr(c, 0x0020a8b0, 0x0020cf28)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0020ab50, 0x0020cf30)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_20cfbc;
                                            } else {
                                                break 'b_20cf38;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = true;
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                        if t {
                                            break 'b_20cfb8;
                                        } else {
                                            break 'b_20cf44;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9f0));
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 6i64) as u64;
                                    if t {
                                        break 'b_20cf9c;
                                    } else {
                                        'b_20cf6c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_20cf6c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_20cf80;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_20cfc0;
                                                    } else {
                                                        break 'b_20cf6c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x8;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_20cfb0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_20cfc0;
                                            } else {
                                                break 'b_20cf80;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                call_ptr(c, 0x002706f0, 0x0020cf90)?;
                                c.g[V0][0] = 0x4;
                                let t = true;
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                if t {
                                    break 'b_20cfb8;
                                } else {
                                    break 'b_20cf9c;
                                }
                            }
                            c.cycles += 5;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9e8), 0u64);
                            c.g[A0][0] = 0x17;
                            call_ptr(c, 0x0020ab50, 0x0020cfa8)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_20cfbc;
                            } else {
                                break 'b_20cfb0;
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x18;
                        call_ptr(c, 0x0020ab50, 0x0020cfb8)?;
                        break 'b_20cfb8;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_20cfbc;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_20cfc0;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void EventSelect_Init(sMapTmpl* this, void* screen, void* owner)` (0x200700).
///
/// Sets up the map template of the event select screen: stores `screen`
/// (`+0x2e8`) and `owner` (`+0x2ec`); opens the PDA template for map styles
/// 1, 3 and 4; clears the map state unless the front-end start of style 0;
/// resets the mode, popup and file fields, looks up the `MapPic` widget (hash
/// of its name) and hides it; then sets up the peak locks, flags, mode panels,
/// map image, popup and (style >= 2) the peak list, and finally hides or shows
/// the named widgets of the owner screen, which ones depending on the map
/// style (`gp - 0xb5c`).
pub fn event_select_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_200744: {
            'b_200738: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x1;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2ec), c.g[A2][0]);
                let t = c.g[V1][0] == c.g[V0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2e8), c.g[A1][0]);
                if t {
                    break 'b_200738;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x4;
                    if t {
                        break 'b_200738;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        if t {
                            break 'b_200744;
                        } else {
                            break 'b_200738;
                        }
                    }
                }
            }
            c.cycles += 3;
            call_ptr(c, 0x0020d110, 0x00200740)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
            break 'b_200744;
        }
        'b_200760: {
            'b_20075c: {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xb54), 0u64);
                    break 'b_20075c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb58));
                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = 0u64;
                        break 'b_200760;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xb54), 0u64);
                        break 'b_20075c;
                    }
                }
            }
            c.cycles += 1;
            c.g[A1][0] = 0u64;
            break 'b_200760;
        }
        'b_2007c4: {
            c.cycles += 19;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00203cb8, 0x00200768)?;
            c.g[V0][0] = 0xffffffffffffffff;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2cc), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d0), c.g[V0][0]);
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
            call_ptr(c, 0x001a3bf0, 0x00200780)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d4), 0u64);
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x25a8));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2d8), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x00200794)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2e8));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x002007a0)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x2e4), c.g[A2][0]);
            if t {
                break 'b_2007c4;
            } else {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x002007c4)?;
                break 'b_2007c4;
            }
        }
        'b_2008b8: {
            'b_200898: {
                'b_200818: {
                    c.cycles += 15;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x2f0), 0u64);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00201750, 0x002007d0)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00201720, 0x002007d8)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00202ad8, 0x002007e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00203990, 0x002007e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00203a58, 0x002007f0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                        break 'b_200818;
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00203d40, 0x00200808)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                        if t {
                            break 'b_200898;
                        } else {
                            break 'b_200818;
                        }
                    }
                }
                'b_200854: {
                    c.cycles += 10;
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4c8));
                    call_ptr(c, 0x0039e8b8, 0x00200824)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25b0));
                    call_ptr(c, 0x0039e8b8, 0x00200834)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                    if t {
                        break 'b_200854;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25b8));
                        call_ptr(c, 0x0039e8b8, 0x0020084c)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        if t {
                            break 'b_2008b8;
                        } else {
                            break 'b_200854;
                        }
                    }
                }
                c.cycles += 17;
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4d8));
                call_ptr(c, 0x0039e8b8, 0x00200860)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4e8));
                call_ptr(c, 0x0039e8b8, 0x00200870)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4f8));
                call_ptr(c, 0x0039e8b8, 0x00200880)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x508));
                call_ptr(c, 0x0039e8b8, 0x00200890)?;
                let t = true;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                if t {
                    break 'b_2008b8;
                } else {
                    break 'b_200898;
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25c0));
            call_ptr(c, 0x0039e8b8, 0x002008a4)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x518));
            call_ptr(c, 0x0039e948, 0x002008b4)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
            break 'b_2008b8;
        }
        'b_200904: {
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
            if t {
                break 'b_200904;
            } else {
                c.cycles += 16;
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x528));
                call_ptr(c, 0x0039e8b8, 0x002008d0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x538));
                call_ptr(c, 0x0039e8b8, 0x002008e0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25c8));
                call_ptr(c, 0x0039e8b8, 0x002008f0)?;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25d0));
                call_ptr(c, 0x0039e8b8, 0x00200900)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
                break 'b_200904;
            }
        }
        c.cycles += 51;
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25d8));
        call_ptr(c, 0x0039e8b8, 0x00200910)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x548));
        call_ptr(c, 0x0039e8b8, 0x00200920)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x558));
        call_ptr(c, 0x0039e8b8, 0x00200930)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x568));
        call_ptr(c, 0x0039e8b8, 0x00200940)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x578));
        call_ptr(c, 0x0039e8b8, 0x00200950)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x588));
        call_ptr(c, 0x0039e8b8, 0x00200960)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x598));
        call_ptr(c, 0x0039e8b8, 0x00200970)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5a8));
        call_ptr(c, 0x0039e8b8, 0x00200980)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5b8));
        call_ptr(c, 0x0039e8b8, 0x00200990)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5c8));
        call_ptr(c, 0x0039e8b8, 0x002009a0)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5d8));
        call_ptr(c, 0x0039e8b8, 0x002009b0)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5e8));
        call_ptr(c, 0x0039e8b8, 0x002009c0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_EnterGame(SSXAudioSystem* this)` (0x2867e8).
///
/// Starts the in-game audio: sets the radio mode and playlist from the back
/// end's reward interface, starts a song (`SelectRandomSong` + `PlayMusic`)
/// unless one is already playing (id 401), clears the rider air flags, fetches
/// the five world-painter queries (music trigger x2, ambience x2, mix x2) into
/// the system (`+0x625c`, `+0x6260`, `+0x5fa0`, `+0x5fa4`, `+0x6c80`,
/// `+0x6c84`), resets the fade/ducking fields, fills the 23 "first time on
/// track" flags at `+0x579c` (when the audio state byte at 0x535c11 is 0) and
/// finally resets the music state, runs `EnterGameInit` and 0x2a4a78.
pub fn ssx_audio_system_enter_game(c: &mut Cpu) {
    guest(c, |c| {
        'b_2868f4: {
            'b_28688c: {
                c.cycles += 36;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S4][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00286810)?;
                c.g[A1][0] = 0xd;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0028681c)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0028b1d8, 0x00286824)?;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                call_ptr(c, 0x00123128, 0x0028682c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x00158750, 0x00286840)?;
                c.g[A2][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00158700, 0x00286854)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0028bf78, 0x00286860)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0028c2d0, 0x0028686c)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x530));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                if t {
                    break 'b_28688c;
                } else {
                    c.cycles += 5;
                    c.g[S5][0] = 0x530000;
                    call_ptr(c, 0x002b49e0, 0x00286880)?;
                    c.g[V1][0] = 0x191;
                    let t = c.g[V0][0] == c.g[V1][0];

                    if t {
                        break 'b_2868f4;
                    } else {
                        break 'b_28688c;
                    }
                }
            }
            'b_2868dc: {
                c.cycles += 12;
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0028d488, 0x00286894)?;
                call_ptr(c, 0x0014dc80, 0x0028689c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002868a8)?;
                c.g[V1][0] = 0x530000;
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[S5][0] = c.g[V1][0];
                if t {
                    break 'b_2868dc;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x0028cf98, 0x002868d4)?;
                    let t = true;

                    if t {
                        break 'b_2868f4;
                    } else {
                        break 'b_2868dc;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0x24;
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0xffffffffffffffff;
            c.g[T0][0] = 0u64;
            call_ptr(c, 0x0028cf98, 0x002868f4)?;
            break 'b_2868f4;
        }
        'b_286a44: {
            c.cycles += 59;
            c.g[S1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2178));
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x002948a0, 0x00286900)?;
            c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2188));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[S1][0];
            c.g[S3][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2180));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c0a58, 0x00286924)?;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x625c), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286934)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6260), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286944)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x5fa0), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286954)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x5fa4), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286964)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6c80), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286974)?;
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[V1][0] = 0x32;
            c.g[A3][0] = 0x1;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 13);
            c.g[A0][0] = c.g[S4][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x626c), 0u64);
            c.g[A1][0] = 0x3;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6274), 0u64);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x5fc4), 0u64);
            c.sb((c.g[S4][0] as u32).wrapping_add(0x5fc0), c.g[V1][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6268), c.g[A2][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6270), c.g[A3][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6264), c.g[A2][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6c84), c.g[V0][0]);
            call_ptr(c, 0x00287f00, 0x002869bc)?;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6288), 0u64);
            call_ptr(c, 0x0014dc80, 0x002869c4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002869d0)?;
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            let t = c.g[V0][0] != 0u64;
            c.g[S2][0] = 0u64;
            if t {
                break 'b_286a44;
            } else {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x579c));
                'l_2869e8: loop {
                    c.cycles += 23;
                    call_ptr(c, 0x0014dc80, 0x002869f0)?;
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x002869fc)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028b1d8, 0x00286a04)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x00286a20)?;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00145d38, 0x00286a34)?;
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 23i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002869e8);
                        }
                        continue 'l_2869e8;
                    } else {
                        break 'b_286a44;
                    }
                }
            }
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0028df08, 0x00286a4c)?;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x00286e20, 0x00286a54)?;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002a4a78, 0x00286a5c)?;
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
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{
        check, check_lohi, elf_image, fill, put_words, Rng, DATA, GAME_GP, SP as TEST_SP,
    };
    use std::sync::atomic::AtomicUsize;

    /// `PATH_FindMap`: the id matches none, one or several records, records
    /// with and without header, and the found-header global set or not.
    #[test]
    fn path_find_map_matches() {
        check_lohi(
            ee::s14::f_003d4bb0__generated,
            path_find_map,
            400,
            &|c, seed| {
                let mut r = Rng(0x774a ^ seed as u64);
                c.pc = 0x003d_4bb0;
                for i in 0..4u32 {
                    let rec = PATH_MAP_TABLE + i * PATH_MAP_SIZE;
                    c.bus.write32(rec + PATH_MAP_MASK, r.next_u32() % 4);
                    let hdr = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(rec + PATH_MAP_HDR, hdr);
                }
                c.bus
                    .write32(PATH_FOUND_HDR, if seed % 2 == 0 { 0 } else { 0x1234 });
                c.g[A0][0] = sx(r.next_u32() % 5);
            },
        );
    }

    /// `PATH_SortMaps`: random keys and used/unused records over the four
    /// records (every swap pattern), a stack pointer that is 8-byte aligned or
    /// not (aligned and unaligned copy loops).
    #[test]
    fn path_sort_maps_matches() {
        check_lohi(
            ee::s14::f_003d4c40__generated,
            path_sort_maps,
            400,
            &|c, seed| {
                let mut r = Rng(0x774b ^ seed as u64);
                c.pc = 0x003d_4c40;
                for i in 0..4u32 {
                    let rec = PATH_MAP_TABLE + i * PATH_MAP_SIZE;
                    fill(c, &mut r, rec, PATH_MAP_SIZE / 4);
                    c.bus.write32(rec + PATH_MAP_MASK, r.next_u32() % 6);
                    let hdr = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(rec + PATH_MAP_HDR, hdr);
                }
                let skew = if seed % 5 == 0 { 4 } else { 0 };
                c.g[SP][0] = sx(TEST_SP - 0x100 + skew);
            },
        );
    }

    /// The word that arms oc774's callee fakes in a machine's RAM.
    pub(crate) const OC774_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC774_MAGIC: u32 = 0x0774_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc774_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc774_arm(c: &mut Cpu) {
        c.bus.write32(OC774_CTRL, OC774_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc774_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc774_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc774_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc774_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc774_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc774_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc774_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC774_CTRL) != OC774_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc774_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc774_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc774_fakes!(
        (
            oc774_fk_147488,
            OC774_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc774_fk_14dc80,
            OC774_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc774_fk_14dd58,
            OC774_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc774_fk_1a0508,
            OC774_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc774_fk_1a0570,
            OC774_PREV_1A0570,
            ee::s03::OVERRIDE_001A0570,
            ee::s03::f_001a0570__generated,
            0x001a_0570
        ),
        (
            oc774_fk_28b180,
            OC774_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc774_fk_317670,
            OC774_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc774_fk_397cc0,
            OC774_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc774_fk_39c870,
            OC774_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc774_fk_39d860,
            OC774_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc774_fk_3a1000,
            OC774_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc774_fk_14eec8,
            OC774_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc774_fk_15e050,
            OC774_PREV_15E050,
            ee::s01::OVERRIDE_0015E050,
            ee::s01::f_0015e050__generated,
            0x0015_e050
        ),
        (
            oc774_fk_1831d0,
            OC774_PREV_1831D0,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0
        ),
        (
            oc774_fk_182ec0,
            OC774_PREV_182EC0,
            ee::s02::OVERRIDE_00182EC0,
            ee::s02::f_00182ec0__generated,
            0x0018_2ec0
        ),
        (
            oc774_fk_158f30,
            OC774_PREV_158F30,
            ee::s01::OVERRIDE_00158F30,
            ee::s01::f_00158f30__generated,
            0x0015_8f30
        ),
        (
            oc774_fk_20a8b0,
            OC774_PREV_20A8B0,
            ee::s05::OVERRIDE_0020A8B0,
            ee::s05::f_0020a8b0__generated,
            0x0020_a8b0
        ),
        (
            oc774_fk_230488,
            OC774_PREV_230488,
            ee::s06::OVERRIDE_00230488,
            ee::s06::f_00230488__generated,
            0x0023_0488
        ),
        (
            oc774_fk_231250,
            OC774_PREV_231250,
            ee::s06::OVERRIDE_00231250,
            ee::s06::f_00231250__generated,
            0x0023_1250
        ),
        (
            oc774_fk_26f8a0,
            OC774_PREV_26F8A0,
            ee::s07::OVERRIDE_0026F8A0,
            ee::s07::f_0026f8a0__generated,
            0x0026_f8a0
        ),
        (
            oc774_fk_2706f0,
            OC774_PREV_2706F0,
            ee::s07::OVERRIDE_002706F0,
            ee::s07::f_002706f0__generated,
            0x0027_06f0
        ),
        (
            oc774_fk_39fa88,
            OC774_PREV_39FA88,
            ee::s13::OVERRIDE_0039FA88,
            ee::s13::f_0039fa88__generated,
            0x0039_fa88
        ),
        (
            oc774_fk_317d70,
            OC774_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc774_fk_39e2a0,
            OC774_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc774_fk_1a3bf0,
            OC774_PREV_1A3BF0,
            ee::s03::OVERRIDE_001A3BF0,
            ee::s03::f_001a3bf0__generated,
            0x001a_3bf0
        ),
        (
            oc774_fk_202ad8,
            OC774_PREV_202AD8,
            ee::s05::OVERRIDE_00202AD8,
            ee::s05::f_00202ad8__generated,
            0x0020_2ad8
        ),
        (
            oc774_fk_203990,
            OC774_PREV_203990,
            ee::s05::OVERRIDE_00203990,
            ee::s05::f_00203990__generated,
            0x0020_3990
        ),
        (
            oc774_fk_203cb8,
            OC774_PREV_203CB8,
            ee::s05::OVERRIDE_00203CB8,
            ee::s05::f_00203cb8__generated,
            0x0020_3cb8
        ),
        (
            oc774_fk_203d40,
            OC774_PREV_203D40,
            ee::s05::OVERRIDE_00203D40,
            ee::s05::f_00203d40__generated,
            0x0020_3d40
        ),
        (
            oc774_fk_20d110,
            OC774_PREV_20D110,
            ee::s05::OVERRIDE_0020D110,
            ee::s05::f_0020d110__generated,
            0x0020_d110
        ),
        (
            oc774_fk_39e8b8,
            OC774_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc774_fk_39e948,
            OC774_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc774_fk_39b960,
            OC774_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc774_fk_145f90,
            OC774_PREV_145F90,
            ee::s01::OVERRIDE_00145F90,
            ee::s01::f_00145f90__generated,
            0x0014_5f90
        ),
        (
            oc774_fk_146008,
            OC774_PREV_146008,
            ee::s01::OVERRIDE_00146008,
            ee::s01::f_00146008__generated,
            0x0014_6008
        ),
        (
            oc774_fk_123128,
            OC774_PREV_123128,
            ee::s00::OVERRIDE_00123128,
            ee::s00::f_00123128__generated,
            0x0012_3128
        ),
        (
            oc774_fk_145d38,
            OC774_PREV_145D38,
            ee::s01::OVERRIDE_00145D38,
            ee::s01::f_00145d38__generated,
            0x0014_5d38
        ),
        (
            oc774_fk_158700,
            OC774_PREV_158700,
            ee::s01::OVERRIDE_00158700,
            ee::s01::f_00158700__generated,
            0x0015_8700
        ),
        (
            oc774_fk_158750,
            OC774_PREV_158750,
            ee::s01::OVERRIDE_00158750,
            ee::s01::f_00158750__generated,
            0x0015_8750
        ),
        (
            oc774_fk_286e20,
            OC774_PREV_286E20,
            ee::s07::OVERRIDE_00286E20,
            ee::s07::f_00286e20__generated,
            0x0028_6e20
        ),
        (
            oc774_fk_287f00,
            OC774_PREV_287F00,
            ee::s07::OVERRIDE_00287F00,
            ee::s07::f_00287f00__generated,
            0x0028_7f00
        ),
        (
            oc774_fk_28b1d8,
            OC774_PREV_28B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8
        ),
        (
            oc774_fk_2948a0,
            OC774_PREV_2948A0,
            ee::s07::OVERRIDE_002948A0,
            ee::s07::f_002948a0__generated,
            0x0029_48a0
        ),
        (
            oc774_fk_2b49e0,
            OC774_PREV_2B49E0,
            ee::s08::OVERRIDE_002B49E0,
            ee::s08::f_002b49e0__generated,
            0x002b_49e0
        ),
        (
            oc774_fk_2c0a58,
            OC774_PREV_2C0A58,
            ee::s08::OVERRIDE_002C0A58,
            ee::s08::f_002c0a58__generated,
            0x002c_0a58
        ),
    );

    pub(crate) const OC774_OBJ: u32 = DATA + 0x1_0000;
    pub(crate) const OC774_WORLD: u32 = DATA + 0x2_0000;
    pub(crate) const OC774_MOD: u32 = DATA + 0x2_1000;
    pub(crate) const OC774_UI: u32 = DATA + 0x2_2000;
    pub(crate) const OC774_STATE: u32 = DATA + 0x3_0000;
    pub(crate) const OC774_REWARD: u32 = DATA + 0x3_1000;
    pub(crate) const OC774_AUDIO: u32 = DATA + 0x4_0000;

    /// `cFEStateCharSetup_Init`: the user number (0, 1, other) at `+0x44`,
    /// the game-path/audio flag at 0x535c11 (2 or not), widgets found or not
    /// (the name lookup returns 0), the screen lookup result zero or not, and
    /// the unwinds of its calls. The rider preview and the audio mode switch
    /// (0x182db8, 0x28f140) are the real, unported functions.
    #[test]
    fn cfe_state_char_setup_init_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s02::f_00182928__generated,
            cfe_state_char_setup_init,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774c, 30);
                elf_image(c);
                oc774_arm(c);
                c.pc = 0x0018_2928;
                c.g[A0][0] = sx(OC774_OBJ);
                c.bus.write32(OC774_OBJ + 0x10, OC774_UI);
                c.bus
                    .write8(OC774_OBJ + 0x44, [0u8, 1, 2][(seed % 3) as usize]);
                c.bus
                    .write8(0x0053_5c11, if (seed / 3) % 2 == 0 { 2 } else { 0 });
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x7c, OC774_MOD);
                if (seed / 6) % 2 == 0 {
                    oc774_force_zero(c, 0x0039_7cc0);
                }
                if (seed / 12) % 3 == 0 {
                    oc774_force_zero(c, 0x0039_d860);
                }
                // 0x28b180 returns the audio system whose current mode
                // (`+0x6290`) is 1 (no change) or something else.
                oc774_force(c, 0x0028_b180, OC774_AUDIO);
                c.bus.write32(
                    OC774_AUDIO + 0x6290,
                    if (seed / 4) % 2 == 0 {
                        1
                    } else {
                        r.next_u32() % 12
                    },
                );
            },
        );
    }

    /// `GameUI_HandleMenuResult`: every UI state id the table distinguishes
    /// (8..35 plus out-of-range ones) with every `mMiscData` (0..8, 19, an
    /// out-of-range one), reward interface with and without awards, network
    /// game or not; `GameUI_OpenScreen` and `0x257610` are the real functions.
    #[test]
    fn game_ui_handle_menu_result_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s05::f_0020ccf8__generated,
            game_ui_handle_menu_result,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774d, 3);
                elf_image(c);
                oc774_arm(c);
                c.pc = 0x0020_ccf8;
                let state = [8u32, 9, 10, 11, 15, 16, 20, 21, 22, 35, 12, 40][(seed % 12) as usize];
                let misc = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 19, 9, 10][((seed / 12) % 12) as usize];
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x84, OC774_MOD);
                c.bus.write32(OC774_WORLD + 0xc0, OC774_UI);
                c.bus.write32(OC774_MOD + 0x48, OC774_UI);
                c.bus.write32(OC774_STATE + 0x98, state);
                c.bus.write32(GAME_GP - 0x9f0, misc);
                c.bus.write8(0x0053_5c12, r.next_u32() as u8);
                c.bus.write32(0x0053_4b30, ((seed / 144) % 2) as u32);
                c.bus.write32(
                    GAME_GP - 0x204,
                    if (seed / 288) % 2 == 0 { 0 } else { OC774_UI },
                );
                oc774_force(c, 0x0039_fa88, OC774_STATE);
                oc774_force(c, 0x0014_dd58, OC774_REWARD);
                c.bus.write32(OC774_REWARD + 0x10, ((seed / 7) % 2) as u32);
                if (seed / 5) % 2 == 0 {
                    oc774_force_zero(c, 0x0015_8f30);
                } else {
                    oc774_force(c, 0x0015_8f30, 2);
                }
                oc774_force(c, 0x0020_a8b0, [1u32, 2, 3, 5, 7][(seed % 5) as usize]);
            },
        );
    }

    /// `EventSelect_Init`: map styles 0..5, the front-end start flag, the
    /// `MapPic` widget found or not, and the unwinds of its calls (the
    /// style-changed-by-refresh path needs a callee that changes `gp - 0xb5c`,
    /// which the fake does not).
    #[test]
    fn event_select_init_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s05::f_00200700__generated,
            event_select_init,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774e, 40);
                oc774_arm(c);
                c.pc = 0x0020_0700;
                c.g[A0][0] = sx(OC774_OBJ);
                c.g[A1][0] = sx(r.next_u32() | 1);
                c.g[A2][0] = sx(OC774_UI);
                c.bus.write32(GAME_GP - 0xb5c, (seed % 6) as u32);
                c.bus.write32(GAME_GP - 0xb58, ((seed / 6) % 2) as u32);
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x11c, r.next_u32());
                if (seed / 12) % 2 == 0 {
                    oc774_force_zero(c, 0x0039_d860);
                }
            },
        );
    }

    /// `SSXAudioSystem_EnterGame`: a song always playing with id 401 (see the
    /// fixture), the audio state byte (0x535c11) zero or not (the 23 first-time
    /// flags are filled), and the unwinds of its calls. The music functions
    /// (0x28bf78, 0x28c2d0, 0x28d488, 0x28cf98, 0x28df08, 0x2a4a78) are the
    /// real, unported ones, running on a zeroed audio system.
    #[test]
    fn ssx_audio_system_enter_game_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s07::f_002867e8__generated,
            ssx_audio_system_enter_game,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774f, 60);
                oc774_arm(c);
                c.pc = 0x0028_67e8;
                c.g[A0][0] = sx(OC774_AUDIO);
                // A song is always playing (id 401): the song-selection chain
                // (0x28d488 -> MusicPlayer_SelectNextSong 0x2b42b8, unported)
                // loops forever on a zeroed audio system, so the fixture never
                // reaches it. The two "not playing" branches stay untested.
                c.bus.write32(OC774_AUDIO + 0x530, 1);
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x84, OC774_MOD);
                c.bus.write32(OC774_MOD + 0x10, OC774_UI);
                c.bus.write32(OC774_UI + 0x8, OC774_STATE);
                c.bus.write8(0x0053_5c11, ((seed / 4) % 2) as u8);
                let _ = r.next_u32();
                oc774_force(c, 0x002b_49e0, 0x191);
            },
        );
    }
}
