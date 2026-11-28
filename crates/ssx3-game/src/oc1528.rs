//! Staging module oc1528 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1528`.

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
        0x0014_c068,
        bolts_build_child_tables,
        "Bolts_BuildChildTables",
    ),
    (
        0x0017_a378,
        debug_menu_parse_world_list_file,
        "DebugMenu_ParseWorldListFile",
    ),
    (
        0x0019_ad18,
        cfe_state_char_equip_scroll_item_list,
        "cFEStateCharEquip_ScrollItemList",
    ),
    (
        0x001a_6f70,
        cfe_state_net_messenger_refresh_roster_entries,
        "cFEStateNetMessenger_RefreshRosterEntries",
    ),
    (
        0x001c_e758,
        cfe_state_trophy_room_set_event_name,
        "cFEStateTrophyRoom_SetEventName",
    ),
];

/// `Bolts_BuildChildTables(BoltTable* self)` (0x14c068): allocates the child-bolt table (`0x5a * kBoltNum`
/// halfwords from `operator_new_named`), carves it into three arrays of 30 rows of `kBoltNum` halfwords
/// (count at `+0x11c`, first at `+0x194`, index at `+0x4dc`; the table pointer goes to `+0x18`), clears the
/// index rows (`EA_MemSet`), records each entry's position in `index[type][slot]`, clears the first and count
/// rows, then walks the entries (`self+4`, `0x38` bytes each, `self+0x1c` of them): for every entry with a parent
/// group (`+6`, not -1) it counts `count[type][group]` and notes the first entry of each run in
/// `first[type][group]`. Transcribed block by block from the generated body (every register and cycle kept).
#[allow(unreachable_code, unused_labels)]
pub fn bolts_build_child_tables(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = 0x5a;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A1][0] = 0x460000;
        c.g[S6][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x59f0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = 0u64;
        c.mult(0, 2, 22, 2);
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A3][0] = 0u64;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x11c));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0014c0bc);
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        call_ptr(c, 0x00317e30, 0x0014c0bc)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.g[A0][0] = 0x1d;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
        c.g[V0][0] = c.g[S7][0];
        'l_14c0d8: loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32), c.g[S0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            let t = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0014c0d8);
                }
                continue 'l_14c0d8;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x194));
                c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4dc));
                c.g[V1][0] = c.g[S4][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[A0][0] = 0x1d;
                'l_14c110: loop {
                    c.cycles += 7;
                    c.sw((c.g[V1][0] as u32), c.g[S0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    let t = (c.g[A0][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0014c110);
                        }
                        continue 'l_14c110;
                    } else {
                        c.cycles += 3;
                        c.g[S1][0] = c.g[S5][0];
                        c.g[S2][0] = 0x1d;
                        c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                        'l_14c138: loop {
                            c.cycles += 11;
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x0014c15c);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x003e6448, 0x0014c15c)?;
                            if (c.g[S2][0] as i64) >= 0 {
                                c.sw((c.g[S1][0] as u32), c.g[S0][0]);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0014c138);
                                }
                                continue 'l_14c138;
                            } else {
                                'b_14c1b4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_14c1b4;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A3][0] = c.g[S5][0];
                                        c.g[A2][0] = 0u64;
                                        'l_14c178: loop {
                                            c.cycles += 15;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V1][0] = sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.sh((c.g[A0][0] as u32), c.g[A1][0]);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                            c.g[V0][0] =
                                                ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x38));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0014c178);
                                                }
                                                continue 'l_14c178;
                                            } else {
                                                break 'b_14c1b4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[S1][0] = c.g[S7][0];
                                c.g[S0][0] = c.g[S4][0];
                                c.g[S2][0] = 0x1d;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                                'l_14c1c8: loop {
                                    c.cycles += 14;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                                    c.g[RA][0] = sx(0x0014c1e0);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                    call_ptr(c, 0x003e6448, 0x0014c1e0)?;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
                                    c.g[RA][0] = sx(0x0014c1f8);
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                    call_ptr(c, 0x003e6448, 0x0014c1f8)?;
                                    let t = (c.g[S2][0] as i64) >= 0;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f08));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0014c1c8);
                                        }
                                        continue 'l_14c1c8;
                                    } else {
                                        'b_14c280: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                            let t = (c.g[V0][0] as i64) <= 0;
                                            c.g[A3][0] = 0u64;
                                            if t {
                                                break 'b_14c280;
                                            } else {
                                                c.cycles += 5;
                                                c.g[T2][0] = c.g[S7][0];
                                                c.g[T1][0] = c.g[S4][0];
                                                c.g[T3][0] = 0xffffffffffffffff;
                                                c.g[T0][0] = 0u64;
                                                'l_14c220: loop {
                                                    'b_14c26c: {
                                                        c.cycles += 5;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[V0][0] = sx((c.g[T0][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x6));
                                                        let t = c.g[A0][0] == c.g[T3][0];
                                                        c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                                                        if t {
                                                            break 'b_14c26c;
                                                        } else {
                                                            c.cycles += 9;
                                                            c.g[A1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A2][0] =
                                                                sx((c.g[A0][0] as u32) << 1);
                                                            c.g[V0][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_add((c.g[A1][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                            c.g[V1][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            c.g[V0][0] = c.lhu((c.g[V1][0] as u32));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            let t = c.g[S6][0] == c.g[A0][0];
                                                            c.sh((c.g[V1][0] as u32), c.g[V0][0]);
                                                            if t {
                                                                break 'b_14c26c;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V1][0] = sx((c.g[T1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A1][0] as u32),
                                                                    ));
                                                                c.g[S6][0] = c.g[A0][0];
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.sh(
                                                                    (c.g[V0][0] as u32),
                                                                    c.g[A3][0],
                                                                );
                                                                break 'b_14c26c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                                    c.g[A3][0] =
                                                        sx((c.g[A3][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] = ((c.g[A3][0] as i64)
                                                        < (c.g[V0][0] as i64))
                                                        as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[T0][0] =
                                                        sx((c.g[T0][0] as u32).wrapping_add(0x38));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0014c220);
                                                        }
                                                        continue 'l_14c220;
                                                    } else {
                                                        break 'b_14c280;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 11;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                                        c.pc = t;
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })
}

/// `DebugMenu_ParseWorldListFile(DebugMenu* self, const char* path)` (0x17a378): loads the text file `path`
/// (`FILE_loadsizez`, 0x100 alignment), and scans it for file-name tokens (letters, digits, `_ - . /`, optionally
/// quoted, a `\` or `:` restarts the token after it, the extension is cut at the first `.`). Each token becomes an
/// `sStr`; if `DebugMenu_AddWorldIfMissing` accepts it, the name and tag are copied into
/// `self->entries[count]` (`0x8c` bytes at `+4`: name `+0`, tag `+0x40`, id `+0x80` = 0x16, flag `+0x84` = 0) and
/// `count` (`self+0`) is bumped. Frees the buffer afterwards.
#[allow(unreachable_code, unused_labels)]
pub fn debug_menu_parse_world_list_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_17a614: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
            c.g[A2][0] = 0x100;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x0017a3ac);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x003e1a70, 0x0017a3ac)?;
            c.g[S2][0] = c.g[V0][0];
            let t = c.g[S2][0] == 0u64;
            c.g[S0][0] = c.g[S2][0];
            if t {
                break 'b_17a614;
            } else {
                c.cycles += 4;
                c.g[S5][0] = 0x22;
                c.g[S4][0] = 0x27;
                c.g[S3][0] = 0x8c;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                'l_17a3c8: loop {
                    'b_17a60c: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                        c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_17a60c;
                        } else {
                            'b_17a408: {
                                'b_17a404: {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lb((c.g[S0][0] as u32));
                                    c.g[V0][0] = 0x20;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[A0][0] = c.lbu((c.g[S0][0] as u32));
                                    if t {
                                        break 'b_17a404;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x9;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0xa;
                                        if t {
                                            break 'b_17a404;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0xd;
                                            if t {
                                                break 'b_17a404;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];

                                                if t {
                                                    break 'b_17a408;
                                                } else {
                                                    break 'b_17a404;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A1][0] = 0x1;
                                break 'b_17a408;
                            }
                            c.cycles += 2;
                            if c.g[A1][0] != 0u64 {
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0017a3c8);
                                }
                                continue 'l_17a3c8;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[A0][0] as u32) << 24);
                                if t {
                                    break 'b_17a60c;
                                } else {
                                    'b_17a43c: {
                                        'b_17a438: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                            if c.g[V0][0] == c.g[S5][0] {
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                break 'b_17a438;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != c.g[S4][0];
                                                c.g[V1][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A2][0] as u32)));
                                                if t {
                                                    break 'b_17a43c;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                    break 'b_17a438;
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V1][0] =
                                            sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A2][0] as u32)));
                                        break 'b_17a43c;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = (c.g[S0][0] < c.g[V1][0]) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[T5][0] = c.g[A2][0];
                                    if t {
                                        break 'b_17a60c;
                                    } else {
                                        c.cycles += 6;
                                        c.g[T0][0] = c.g[V1][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[A2][0] = 0u64;
                                        c.g[T7][0] = 0x5c;
                                        c.g[T6][0] = 0x3a;
                                        'l_17a460: loop {
                                            'b_17a548: {
                                                'b_17a53c: {
                                                    'b_17a528: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[S0][0] < c.g[T0][0]) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = c.lbu((c.g[S0][0] as u32));
                                                        if t {
                                                            break 'b_17a528;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[T4][0] = 0x5f;
                                                            c.g[T3][0] = 0x2e;
                                                            c.g[T2][0] = 0x2d;
                                                            c.g[T1][0] = 0x2f;
                                                            c.g[A3][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_add((c.g[T5][0] as u32)));
                                                            c.g[V1][0] = c.lb((c.g[S0][0] as u32));
                                                            'l_17a488: loop {
                                                                'b_17a4dc: {
                                                                    'b_17a4d8: {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x61,
                                                                                ));
                                                                        c.g[V0][0] = (c.g[V0][0]
                                                                            < 0x1au64)
                                                                            as u64;
                                                                        let t = c.g[V0][0] != 0u64;
                                                                        c.g[A0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_17a4d8;
                                                                        } else {
                                                                            c.cycles += 4;
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x41,
                                                                                    ));
                                                                            c.g[V0][0] = (c.g[V0]
                                                                                [0]
                                                                                < 0x1au64)
                                                                                as u64;
                                                                            if c.g[V0][0] != 0u64 {
                                                                                c.g[A0][0] = 0x1;
                                                                                break 'b_17a4dc;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x30));
                                                                                c.g[V0][0] = (c.g
                                                                                    [V0][0]
                                                                                    < 0xau64)
                                                                                    as u64;
                                                                                if c.g[V0][0]
                                                                                    != 0u64
                                                                                {
                                                                                    c.g[A0][0] =
                                                                                        0x1;
                                                                                    break 'b_17a4dc;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V1][0]
                                                                                        == c.g[T4]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        break 'b_17a4dc;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V1]
                                                                                            [0]
                                                                                            == c.g
                                                                                                [T3]
                                                                                                [0]
                                                                                        {
                                                                                            c.g[A0][0] = 0x1;
                                                                                            break 'b_17a4dc;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            if c.g[V1][0] == c.g[T2][0] {
c.g[A0][0] = 0x1;
break 'b_17a4dc;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[T1][0];

if t {
break 'b_17a4dc;
} else {
break 'b_17a4d8;
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 1;
                                                                    c.g[A0][0] = 0x1;
                                                                    break 'b_17a4dc;
                                                                }
                                                                'b_17a510: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[A0][0] == 0u64;
                                                                    c.g[V0][0] = (c.g[S0][0]
                                                                        < c.g[T0][0])
                                                                        as u64;
                                                                    if t {
                                                                        break 'b_17a510;
                                                                    } else {
                                                                        'b_17a500: {
                                                                            c.cycles += 2;
                                                                            if c.g[A2][0] != 0u64 {
                                                                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                break 'b_17a500;
                                                                            } else {
                                                                                c.cycles += 5;
                                                                                c.g[V0][0] = c.lb(
                                                                                    (c.g[S0][0]
                                                                                        as u32),
                                                                                );
                                                                                c.g[A2][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[V0][0] = c.g
                                                                                    [V0][0]
                                                                                    ^ 0x2e;
                                                                                if c.g[V0][0] != 0 {
                                                                                    c.g[A2][0] =
                                                                                        0u64;
                                                                                }
                                                                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                break 'b_17a500;
                                                                            }
                                                                        }
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = (c.g[S0][0]
                                                                            < c.g[A3][0])
                                                                            as u64;
                                                                        if c.g[V0][0] != 0u64 {
                                                                            c.g[V1][0] = c
                                                                                .lb((c.g[S0][0]
                                                                                    as u32));
                                                                            if event_due(c) {
                                                                                deliver_event_here(
                                                                                    c, 0x0017a488,
                                                                                );
                                                                            }
                                                                            continue 'l_17a488;
                                                                        } else {
                                                                            c.cycles += 1;
                                                                            c.g[V0][0] = (c.g[S0]
                                                                                [0]
                                                                                < c.g[T0][0])
                                                                                as u64;
                                                                            break 'b_17a510;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[A0][0] =
                                                                    c.lbu((c.g[S0][0] as u32));
                                                                if t {
                                                                    break 'b_17a528;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        << 24);
                                                                    c.g[V0][0] = sx(((c.g[V0][0]
                                                                        as u32)
                                                                        as i32
                                                                        >> 24)
                                                                        as u32);
                                                                    if c.g[V0][0] == c.g[T7][0] {
                                                                        c.g[A1][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        break 'b_17a53c;
                                                                    } else {
                                                                        break 'b_17a528;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 24);
                                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32
                                                        >> 24)
                                                        as u32);
                                                    let t = c.g[V0][0] != c.g[T6][0];
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                    if t {
                                                        break 'b_17a548;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[A1][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_17a53c;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A2][0] = 0u64;
                                                let t = true;
                                                c.g[S0][0] = c.g[A1][0];
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0017a460);
                                                    }
                                                    continue 'l_17a460;
                                                } else {
                                                    break 'b_17a548;
                                                }
                                            }
                                            'b_17a570: {
                                                c.cycles += 4;
                                                c.g[V0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                if c.g[A2][0] == 0 {
                                                    c.g[A2][0] = c.g[S0][0];
                                                }
                                                if t {
                                                    break 'b_17a570;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 24);
                                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32
                                                        >> 24)
                                                        as u32);
                                                    if c.g[V0][0] == c.g[S5][0] {
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        break 'b_17a570;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[V0][0] == c.g[S4][0] {
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_17a570;
                                                        } else {
                                                            break 'b_17a570;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[A1][0] {
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0017a3c8);
                                                }
                                                continue 'l_17a3c8;
                                            } else {
                                                'b_17a5f8: {
                                                    c.cycles += 9;
                                                    c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_sub((c.g[A1][0] as u32)));
                                                    c.g[RA][0] = sx(0x0017a584);
                                                    c.g[A0][0] = c.g[SP][0];
                                                    call_ptr(c, 0x00318350, 0x0017a584)?;
                                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[RA][0] = sx(0x0017a594);
                                                    c.g[A2][0] = c.g[A1][0];
                                                    call_ptr(c, 0x0017a830, 0x0017a594)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                                    if t {
                                                        break 'b_17a5f8;
                                                    } else {
                                                        c.cycles += 23;
                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                        c.mult(0, 4, 4, 19);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[RA][0] = sx(0x0017a5b0);
                                                        c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, 0x004166f4, 0x0017a5b0)?;
                                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                                                        c.mult(0, 2, 4, 19);
                                                        c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[S1][0] as u32)));
                                                        c.g[RA][0] = sx(0x0017a5c8);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x44));
                                                        call_ptr(c, 0x004166f4, 0x0017a5c8)?;
                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                                                        c.g[A0][0] = 0x16;
                                                        c.mult(0, 2, 3, 19);
                                                        c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[S1][0] as u32)));
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x84),
                                                            c.g[A0][0],
                                                        );
                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                                        c.mult(0, 3, 2, 19);
                                                        c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[S1][0] as u32)));
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x88),
                                                            0u64,
                                                        );
                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                                                        break 'b_17a5f8;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[RA][0] = sx(0x0017a604);
                                                c.g[A1][0] = 0x2;
                                                call_ptr(c, 0x00318260, 0x0017a604)?;
                                                let t = true;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x0017a3c8);
                                                    }
                                                    continue 'l_17a3c8;
                                                } else {
                                                    break 'b_17a60c;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0017a614);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002523a8, 0x0017a614)?;
                    break 'b_17a614;
                }
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateCharEquip_ScrollItemList(cFEStateCharEquip* self, int n)` (0x19ad18): scrolls the six visible item
/// rows by `n` (negative: up). Does nothing without the menu (`+0x5c`). For `|n| > 6` it refills every row
/// (`FillBuyRows` when `+0xc8 == 1`, else `FillEquipRows`); otherwise it copies the rows that stay (row vtable
/// calls on the row objects at `+0x8c..` and the sound helper `0x3a0c28`/`0x3a0e90`) and refills the `|n|`
/// vacated rows with `SetBuyRow` / `SetEquipRow`.
#[allow(unreachable_code, unused_labels)]
pub fn cfe_state_char_equip_scroll_item_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_19afac: {
            'b_19afa8: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = c.g[A1][0];
                if t {
                    break 'b_19afa8;
                } else {
                    'b_19ad88: {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6));
                        c.g[V0][0] = (c.g[V0][0] < 0xdu64) as u64;
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_19ad88;
                        } else {
                            'b_19ad78: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc8));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_19ad78;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x0019ad70);
                                    call_ptr(c, 0x0019b598, 0x0019ad70)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    if t {
                                        break 'b_19afac;
                                    } else {
                                        break 'b_19ad78;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0019ad80);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x0019b518, 0x0019ad80)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_19afac;
                            } else {
                                break 'b_19ad88;
                            }
                        }
                    }
                    'b_19ae98: {
                        c.cycles += 2;
                        let t = (c.g[S3][0] as i64) >= 0;
                        c.g[V0][0] = sx(0u32.wrapping_sub((c.g[S3][0] as u32)));
                        if t {
                            break 'b_19ae98;
                        } else {
                            'b_19ae2c: {
                                c.cycles += 5;
                                c.g[S5][0] = 0x5;
                                c.g[S4][0] = c.g[V0][0];
                                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc8));
                                    break 'b_19ae2c;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                                    c.g[V0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa0));
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                                    'l_19adb8: loop {
                                        c.cycles += 28;
                                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x0019add8);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x0019add8)?;
                                        c.g[RA][0] = sx(0x0019ade0);
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                        call_ptr(c, 0x003a0c28, 0x0019ade0)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                        c.g[RA][0] = sx(0x0019adec);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a0e90, 0x0019adec)?;
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
                                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x0019ae1c);
                                        c.g[A1][0] = c.g[A1][0] & 0x1;
                                        call_ptr(c, t, 0x0019ae1c)?;
                                        c.g[V0][0] =
                                            ((c.g[S5][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0019adb8);
                                            }
                                            continue 'l_19adb8;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0xc8));
                                            break 'b_19ae2c;
                                        }
                                    }
                                }
                            }
                            'b_19ae68: {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_19ae68;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[S4][0] as i64) <= 0;
                                    c.g[S0][0] = 0u64;
                                    if t {
                                        break 'b_19afa8;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A1][0] = c.g[S0][0];
                                        'l_19ae48: loop {
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x0019ae50);
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x0019ba60, 0x0019ae50)?;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0019ae48);
                                                }
                                                continue 'l_19ae48;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                if t {
                                                    break 'b_19afac;
                                                } else {
                                                    break 'b_19ae68;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[S4][0] as i64) <= 0;
                            c.g[S0][0] = 0u64;
                            if t {
                                break 'b_19afa8;
                            } else {
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S0][0];
                                'l_19ae78: loop {
                                    c.cycles += 6;
                                    c.g[RA][0] = sx(0x0019ae80);
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x0019b7e0, 0x0019ae80)?;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0019ae78);
                                        }
                                        continue 'l_19ae78;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        if t {
                                            break 'b_19afac;
                                        } else {
                                            break 'b_19ae98;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = (c.g[S3][0] as i64) <= 0;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 6i64) as u64;
                    if t {
                        break 'b_19afa8;
                    } else {
                        'b_19af30: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S4][0] = c.g[S3][0];
                            if t {
                                break 'b_19af30;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                                c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8c));
                                c.g[S0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                                'l_19aec0: loop {
                                    c.cycles += 28;
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019aee0);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0019aee0)?;
                                    c.g[RA][0] = sx(0x0019aee8);
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    call_ptr(c, 0x003a0c28, 0x0019aee8)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                                    c.g[RA][0] = sx(0x0019aef4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a0e90, 0x0019aef4)?;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 6) as u32);
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019af24);
                                    c.g[A1][0] = c.g[A1][0] & 0x1;
                                    call_ptr(c, t, 0x0019af24)?;
                                    c.g[V0][0] = ((c.g[S4][0] as i64) < 6i64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0019aec0);
                                        }
                                        continue 'l_19aec0;
                                    } else {
                                        break 'b_19af30;
                                    }
                                }
                            }
                        }
                        'b_19af78: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc8));
                            c.g[V0][0] = c.g[V0][0] ^ 0x1;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0x6;
                            if t {
                                break 'b_19af78;
                            } else {
                                c.cycles += 4;
                                c.g[S0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub((c.g[S3][0] as u32)));
                                c.g[V1][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                                if c.g[V1][0] == 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    break 'b_19afac;
                                } else {
                                    c.cycles += 2;
                                    c.g[A1][0] = c.g[S0][0];
                                    'l_19af58: loop {
                                        c.cycles += 6;
                                        c.g[RA][0] = sx(0x0019af60);
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x0019ba60, 0x0019af60)?;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = c.g[S0][0];
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0019af58);
                                            }
                                            continue 'l_19af58;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_19afac;
                                            } else {
                                                break 'b_19af78;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S3][0] as u32)));
                        c.g[V1][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                        if c.g[V1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            break 'b_19afac;
                        } else {
                            c.cycles += 2;
                            c.g[A1][0] = c.g[S0][0];
                            'l_19af90: loop {
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x0019af98);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0019b7e0, 0x0019af98)?;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S0][0] as i64) < 6i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A1][0] = c.g[S0][0];
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019af90);
                                    }
                                    continue 'l_19af90;
                                } else {
                                    break 'b_19afa8;
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_19afac;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateNetMessenger_RefreshRosterEntries()` (0x1a6f70): for every roster entry `i` of the global session
/// (`gp+0x238`, created on demand through `0x260f80` / `0x261460` / `0x2613c8` / `0x25b800` before each use)
/// that has a name (`0x2625c8`) and is flagged (`0x262400`), it asks whether a record (`0x263128`) or messages
/// (`0x262a38`) exist and, when neither does, creates an entry (`0x261ef8(session, name, 2, buf)`).
#[allow(unreachable_code, unused_labels)]
pub fn cfe_state_net_messenger_refresh_roster_entries(c: &mut Cpu) {
    guest(c, |c| {
        'b_1a6f98: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[V0][0] != 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            if t {
                break 'b_1a6f98;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a6f98);
                call_ptr(c, 0x00260f80, 0x001a6f98)?;
                break 'b_1a6f98;
            }
        }
        'b_1a6fb4: {
            c.cycles += 4;
            c.g[RA][0] = sx(0x001a6fa0);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            call_ptr(c, 0x00261460, 0x001a6fa0)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            if t {
                break 'b_1a6fb4;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a6fb0);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a6fb0)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                break 'b_1a6fb4;
            }
        }
        'b_1a6fc8: {
            c.cycles += 2;
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            if t {
                break 'b_1a6fc8;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a6fc4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a6fc4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                break 'b_1a6fc8;
            }
        }
        'b_1a7194: {
            c.cycles += 5;
            c.g[RA][0] = sx(0x001a6fd0);
            c.g[S2][0] = 0u64;
            call_ptr(c, 0x00262598, 0x001a6fd0)?;
            c.g[S3][0] = c.g[V0][0];
            let t = (c.g[S3][0] as i64) <= 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_1a7194;
            } else {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                'l_1a6fe0: loop {
                    'b_1a6ff0: {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_1a6ff0;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x001a6ff0);
                            call_ptr(c, 0x00260f80, 0x001a6ff0)?;
                            break 'b_1a6ff0;
                        }
                    }
                    'b_1a700c: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001a6ff8);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x00261460, 0x001a6ff8)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1a700c;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001a7008);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x002613c8, 0x001a7008)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            break 'b_1a700c;
                        }
                    }
                    'b_1a7020: {
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1a7020;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001a701c);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            call_ptr(c, 0x0025b800, 0x001a701c)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            break 'b_1a7020;
                        }
                    }
                    'b_1a7184: {
                        'b_1a7180: {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x001a7028);
                            c.g[A1][0] = c.g[S2][0];
                            call_ptr(c, 0x002625c8, 0x001a7028)?;
                            c.g[S0][0] = c.g[V0][0];
                            let t = c.g[S0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1a7180;
                            } else {
                                'b_1a7044: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1a7044;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001a7044);
                                        call_ptr(c, 0x00260f80, 0x001a7044)?;
                                        break 'b_1a7044;
                                    }
                                }
                                'b_1a7060: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001a704c);
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x00261460, 0x001a704c)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1a7060;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x001a705c);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x002613c8, 0x001a705c)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        break 'b_1a7060;
                                    }
                                }
                                'b_1a7074: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1a7074;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x001a7070);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025b800, 0x001a7070)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        break 'b_1a7074;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001a707c);
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x00262400, 0x001a707c)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1a7180;
                                } else {
                                    'b_1a7094: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1a7094;
                                        } else {
                                            c.cycles += 2;
                                            c.g[RA][0] = sx(0x001a7094);
                                            call_ptr(c, 0x00260f80, 0x001a7094)?;
                                            break 'b_1a7094;
                                        }
                                    }
                                    'b_1a70b0: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001a709c);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x00261460, 0x001a709c)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a70b0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[RA][0] = sx(0x001a70ac);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x002613c8, 0x001a70ac)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1a70b0;
                                        }
                                    }
                                    'b_1a70c4: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a70c4;
                                        } else {
                                            c.cycles += 3;
                                            c.g[RA][0] = sx(0x001a70c0);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025b800, 0x001a70c0)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1a70c4;
                                        }
                                    }
                                    'b_1a70e4: {
                                        c.cycles += 6;
                                        c.g[RA][0] = sx(0x001a70cc);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x00263128, 0x001a70cc)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1a70e4;
                                        } else {
                                            c.cycles += 2;
                                            c.g[RA][0] = sx(0x001a70e4);
                                            call_ptr(c, 0x00260f80, 0x001a70e4)?;
                                            break 'b_1a70e4;
                                        }
                                    }
                                    'b_1a7100: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001a70ec);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x00261460, 0x001a70ec)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a7100;
                                        } else {
                                            c.cycles += 3;
                                            c.g[RA][0] = sx(0x001a70fc);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x002613c8, 0x001a70fc)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1a7100;
                                        }
                                    }
                                    'b_1a7114: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1a7114;
                                        } else {
                                            c.cycles += 3;
                                            c.g[RA][0] = sx(0x001a7110);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025b800, 0x001a7110)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1a7114;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001a711c);
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x00262a38, 0x001a711c)?;
                                    if c.g[S1][0] != 0u64 {
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        break 'b_1a7184;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                            break 'b_1a7184;
                                        } else {
                                            'b_1a7140: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1a7140;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[RA][0] = sx(0x001a7140);
                                                    call_ptr(c, 0x00260f80, 0x001a7140)?;
                                                    break 'b_1a7140;
                                                }
                                            }
                                            'b_1a715c: {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001a7148);
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                call_ptr(c, 0x00261460, 0x001a7148)?;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                if t {
                                                    break 'b_1a715c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x001a7158);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x238)
                                                        );
                                                    call_ptr(c, 0x002613c8, 0x001a7158)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x238)
                                                        );
                                                    break 'b_1a715c;
                                                }
                                            }
                                            'b_1a7170: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                if t {
                                                    break 'b_1a7170;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x001a716c);
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                    call_ptr(c, 0x0025b800, 0x001a716c)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x238)
                                                        );
                                                    break 'b_1a7170;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[A2][0] = 0x2;
                                            c.g[RA][0] = sx(0x001a7180);
                                            c.g[A3][0] = c.g[SP][0];
                                            call_ptr(c, 0x00261ef8, 0x001a7180)?;
                                            break 'b_1a7180;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        break 'b_1a7184;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001a6fe0);
                        }
                        continue 'l_1a6fe0;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1a7194;
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        Ok(())
    })
}

/// `cFEStateTrophyRoom_SetEventName(cUIText* text, int peak, int kind, int idx)` (0x1ce758): sets the label of a
/// trophy-room line. Kinds 3 and 2 use fixed localised strings (kind 2 by `idx`); otherwise the medals table
/// at `0x45aad8` (`peak * 40 + kind * 10 + idx * 2`: type, track) selects a formatted "<Peak> %s" caption
/// (type 4, through the BE interface and `sprintf`), an ASCII jam name (type 5) or a localised event name
/// (types 6..11), and any other type shows the track's location name.
#[allow(unreachable_code, unused_labels)]
pub fn cfe_state_trophy_room_set_event_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_1ce9c4: {
            'b_1ce9ac: {
                'b_1ce990: {
                    'b_1ce798: {
                        c.cycles += 13;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x230));
                        c.g[V0][0] = 0x3;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x210));
                        c.g[S0][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x200));
                        c.g[S1][0] = c.g[A2][0];
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                        c.g[S2][0] = c.g[A0][0];
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                        c.g[S3][0] = c.g[A3][0];
                        let t = c.g[S1][0] != c.g[V0][0];
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x1d0), c.g[RA][0]);
                        if t {
                            break 'b_1ce798;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6bb8));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce798;
                            }
                        }
                    }
                    'b_1ce7c0: {
                        c.cycles += 3;
                        c.g[S4][0] = 0x2;
                        let t = c.g[S1][0] != c.g[S4][0];

                        if t {
                            break 'b_1ce7c0;
                        } else {
                            'b_1ce7b8: {
                                c.cycles += 2;
                                if c.g[S3][0] != 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1ce7b8;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6bd0));
                                    if t {
                                        break 'b_1ce990;
                                    } else {
                                        break 'b_1ce7b8;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6be8));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce7c0;
                            }
                        }
                    }
                    'b_1ce8b8: {
                        c.cycles += 21;
                        c.g[RA][0] = sx(0x001ce7c8);
                        call_ptr(c, 0x0014dc80, 0x001ce7c8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001ce7d4);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001ce7d4)?;
                        c.g[T0][0] = 0xa;
                        c.g[A0][0] = 0x28;
                        c.mult(0, 5, 17, 8);
                        c.mult(0, 4, 16, 4);
                        c.g[A3][0] = c.g[V0][0];
                        c.g[V1][0] = 0x460000;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 1);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5528));
                        c.g[A2][0] = 0x4;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = c.lb((c.g[A1][0] as u32));
                        let t = c.g[A0][0] != c.g[A2][0];
                        c.g[A1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_1ce8b8;
                        } else {
                            'b_1ce864: {
                                'b_1ce84c: {
                                    'b_1ce83c: {
                                        'b_1ce82c: {
                                            c.cycles += 2;
                                            let t = c.g[S0][0] != 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1ce82c;
                                            } else {
                                                c.cycles += 4;
                                                c.g[S0][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x190));
                                                c.g[A1][0] = 0x460000;
                                                let t = true;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x6c00));
                                                if t {
                                                    break 'b_1ce83c;
                                                } else {
                                                    break 'b_1ce82c;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S0][0] != c.g[V0][0];
                                        c.g[A1][0] = 0x460000;
                                        if t {
                                            break 'b_1ce84c;
                                        } else {
                                            c.cycles += 2;
                                            c.g[S0][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x190));
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x6c10));
                                            break 'b_1ce83c;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001ce844);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x002c2540, 0x001ce844)?;
                                    let t = true;
                                    c.g[S3][0] = c.g[S0][0];
                                    if t {
                                        break 'b_1ce864;
                                    } else {
                                        break 'b_1ce84c;
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[S4][0];
                                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                                if t {
                                    break 'b_1ce864;
                                } else {
                                    c.cycles += 4;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x001ce864);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c20));
                                    call_ptr(c, 0x002c2540, 0x001ce864)?;
                                    break 'b_1ce864;
                                }
                            }
                            c.cycles += 21;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c30));
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x001ce884);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            call_ptr(c, 0x00317670, 0x001ce884)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001ce894);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x001ce894)?;
                            c.g[A1][0] = c.g[S3][0];
                            c.g[A0][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x001ce8a4);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x002c26d0, 0x001ce8a4)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001ce8b0);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x003a0e90, 0x001ce8b0)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                            if t {
                                break 'b_1ce9c4;
                            } else {
                                break 'b_1ce8b8;
                            }
                        }
                    }
                    'b_1ce920: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_1ce920;
                        } else {
                            'b_1ce8e4: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != 0u64;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_1ce8e4;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001ce8dc);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c40));
                                    call_ptr(c, 0x003a0c50, 0x001ce8dc)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                                    if t {
                                        break 'b_1ce9c4;
                                    } else {
                                        break 'b_1ce8e4;
                                    }
                                }
                            }
                            'b_1ce900: {
                                c.cycles += 2;
                                let t = c.g[S0][0] != c.g[V0][0];
                                c.g[A1][0] = 0x460000;
                                if t {
                                    break 'b_1ce900;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001ce8f8);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c50));
                                    call_ptr(c, 0x003a0c50, 0x001ce8f8)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                                    if t {
                                        break 'b_1ce9c4;
                                    } else {
                                        break 'b_1ce900;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = c.g[S0][0] != c.g[S4][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                            if t {
                                break 'b_1ce9c4;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001ce918);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c60));
                                call_ptr(c, 0x003a0c50, 0x001ce918)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                                if t {
                                    break 'b_1ce9c4;
                                } else {
                                    break 'b_1ce920;
                                }
                            }
                        }
                    }
                    'b_1ce934: {
                        c.cycles += 2;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V0][0] = 0x7;
                        if t {
                            break 'b_1ce934;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c70));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce934;
                            }
                        }
                    }
                    'b_1ce948: {
                        c.cycles += 2;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V0][0] = 0x8;
                        if t {
                            break 'b_1ce948;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c80));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce948;
                            }
                        }
                    }
                    'b_1ce95c: {
                        c.cycles += 2;
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[V0][0] = 0x9;
                        if t {
                            break 'b_1ce95c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c90));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce95c;
                            }
                        }
                    }
                    'b_1ce970: {
                        c.cycles += 2;
                        let t = c.g[A0][0] != c.g[V0][0];

                        if t {
                            break 'b_1ce970;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ca8));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce970;
                            }
                        }
                    }
                    'b_1ce984: {
                        c.cycles += 2;
                        let t = c.g[A0][0] != c.g[T0][0];
                        c.g[V0][0] = 0xb;
                        if t {
                            break 'b_1ce984;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6cb8));
                            if t {
                                break 'b_1ce990;
                            } else {
                                break 'b_1ce984;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_1ce9ac;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6cc8));
                        break 'b_1ce990;
                    }
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x001ce998);
                call_ptr(c, 0x00317670, 0x001ce998)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001ce9a4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001ce9a4)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                if t {
                    break 'b_1ce9c4;
                } else {
                    break 'b_1ce9ac;
                }
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x001ce9b4);
            c.g[A0][0] = c.g[A3][0];
            call_ptr(c, 0x00144c60, 0x001ce9b4)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001ce9c0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a0c50, 0x001ce9c0)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
            break 'b_1ce9c4;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1d0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x230));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of the oc1528 fakes (their own word and magic), the per-call
    /// "result is zero" mask (bit n = the n-th logged call), the result of the
    /// session count call, and the size / buffer the faked file loader returns.
    const OC1528_CTRL: u32 = DATA + 0xe_0000;
    const OC1528_MAGIC: u32 = 0x1528_c0de;
    const OC1528_ZERO: u32 = OC1528_CTRL + 4;
    const OC1528_COUNT: u32 = OC1528_CTRL + 8;
    const OC1528_SIZE: u32 = OC1528_CTRL + 12;
    const OC1528_BUF: u32 = OC1528_CTRL + 16;

    /// Fixture area (above the pool allocator and arena regions, below the control words).
    const OC1528_BASE: u32 = DATA + 0x9_0000;

    /// A gated fake call: logs the registers like `fakes::fake` and then
    /// shapes the result: the session count, the faked file loader (size
    /// through `a1`, buffer in `v0`), otherwise zero when the call's mask bit
    /// is set (a random non-zero pointer otherwise).
    fn oc1528_fake_call(c: &mut Cpu, addr: u32, ra: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let a1 = c.g[A1][0] as u32;
        fakes::fake(c, addr, ra);
        match addr {
            0x0026_2598 => c.g[V0][0] = c.bus.read32(OC1528_COUNT) as u64,
            0x003e_1a70 => {
                let size = c.bus.read32(OC1528_SIZE);
                c.bus.write32(a1, size);
                c.g[V0][0] = c.bus.read32(OC1528_BUF) as u64;
            }
            _ => {
                if n < 32 && (c.bus.read32(OC1528_ZERO) >> n) & 1 != 0 {
                    c.g[V0][0] = 0;
                }
            }
        }
    }

    /// Gated fakes for every callee with an override slot: with the gate word
    /// set they run `oc1528_fake_call`, with it clear they run what the slot held.
    macro_rules! oc1528_fakes {
        ($($fake:ident, $slot:path, $gen:path, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1528_CTRL) != OC1528_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    oc1528_fake_call(c, $addr, ra);
                }
            )*

            fn oc1528_hooks() {
                $(
                    fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
                )*
            }
        };
    }

    oc1528_fakes! {
        oc1528_fake_loadsizez, ee::s14::OVERRIDE_003E1A70, ee::s14::f_003e1a70__generated, 0x003e_1a70;
        oc1528_fake_str_ctor, ee::s10::OVERRIDE_00318350, ee::s10::f_00318350__generated, 0x0031_8350;
        oc1528_fake_add_world, ee::s02::OVERRIDE_0017A830, ee::s02::f_0017a830__generated, 0x0017_a830;
        oc1528_fake_strcpy, ee::s15::OVERRIDE_004166F4, ee::s15::f_004166f4__generated, 0x0041_66f4;
        oc1528_fake_str_dtor, ee::s10::OVERRIDE_00318260, ee::s10::f_00318260__generated, 0x0031_8260;
        oc1528_fake_free, ee::s06::OVERRIDE_002523A8, ee::s06::f_002523a8__generated, 0x0025_23a8;
        oc1528_fake_fill_buy, ee::s03::OVERRIDE_0019B598, ee::s03::f_0019b598__generated, 0x0019_b598;
        oc1528_fake_fill_equip, ee::s03::OVERRIDE_0019B518, ee::s03::f_0019b518__generated, 0x0019_b518;
        oc1528_fake_set_buy, ee::s03::OVERRIDE_0019BA60, ee::s03::f_0019ba60__generated, 0x0019_ba60;
        oc1528_fake_set_equip, ee::s03::OVERRIDE_0019B7E0, ee::s03::f_0019b7e0__generated, 0x0019_b7e0;
        oc1528_fake_ui_a, ee::s13::OVERRIDE_003A0C28, ee::s13::f_003a0c28__generated, 0x003a_0c28;
        oc1528_fake_ui_b, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90;
        oc1528_fake_sess_init, ee::s06::OVERRIDE_00260F80, ee::s06::f_00260f80__generated, 0x0026_0f80;
        oc1528_fake_sess_ready, ee::s06::OVERRIDE_00261460, ee::s06::f_00261460__generated, 0x0026_1460;
        oc1528_fake_sess_start, ee::s06::OVERRIDE_002613C8, ee::s06::f_002613c8__generated, 0x0026_13c8;
        oc1528_fake_sess_cfg, ee::s06::OVERRIDE_0025B800, ee::s06::f_0025b800__generated, 0x0025_b800;
        oc1528_fake_sess_count, ee::s07::OVERRIDE_00262598, ee::s07::f_00262598__generated, 0x0026_2598;
        oc1528_fake_sess_name, ee::s07::OVERRIDE_002625C8, ee::s07::f_002625c8__generated, 0x0026_25c8;
        oc1528_fake_sess_flag, ee::s07::OVERRIDE_00262400, ee::s07::f_00262400__generated, 0x0026_2400;
        oc1528_fake_sess_has, ee::s07::OVERRIDE_00263128, ee::s07::f_00263128__generated, 0x0026_3128;
        oc1528_fake_sess_msgs, ee::s07::OVERRIDE_00262A38, ee::s07::f_00262a38__generated, 0x0026_2a38;
        oc1528_fake_sess_add, ee::s07::OVERRIDE_00261EF8, ee::s07::f_00261ef8__generated, 0x0026_1ef8;
        oc1528_fake_get_be, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80;
        oc1528_fake_get_iface, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58;
        oc1528_fake_ustr_copy, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated, 0x002c_2540;
        oc1528_fake_hash, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670;
        oc1528_fake_sprintf, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0;
        oc1528_fake_text_id, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50;
        oc1528_fake_text_ustr, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000;
        oc1528_fake_loc_name, ee::s01::OVERRIDE_00144C60, ee::s01::f_00144c60__generated, 0x0014_4c60;
    }

    /// `fakes::machine` plus the gated fakes, the gate word and the result mask.
    fn oc1528_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = fakes::machine(c, seed, salt, calls);
        oc1528_hooks();
        c.bus.write32(OC1528_CTRL, OC1528_MAGIC);
        c.bus.write32(OC1528_ZERO, r.next_u32());
        r
    }

    // ---- Bolts_BuildChildTables ------------------------------------------

    /// `Bolts_BuildChildTables`: the allocation and `EA_MemSet` calls are the real
    /// ones (size-class allocator from `machine_rt`); the table object has
    /// `kBoltNum` (gp-0x1f08) of 2..7 and 0..6 random entries (type 0..29, slot,
    /// parent group -1..n-1) so every branch of the three entry loops runs.
    #[test]
    fn bolts_build_child_tables_matches() {
        check(
            ee::s01::f_0014c068__generated,
            bolts_build_child_tables,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x14c068, 0);
                let n = 2 + seed % 6;
                let obj = OC1528_BASE;
                let entries = OC1528_BASE + 0x1000;
                let n0 = (seed / 3) % 7;
                c.bus.write32(GAME_GP.wrapping_sub(0x1f08), n);
                c.bus.write32(obj + 4, entries);
                c.bus.write32(obj + 0x1c, n0);
                for i in 0..n0 {
                    let e = entries + 0x38 * i;
                    c.bus.write8(e, (r.next_u32() % 30) as u8);
                    c.bus.write16(e + 4, (r.next_u32() % n) as u16);
                    // Parent group: -1 (none) or 0..n-1; repeated parents are common with few bolts.
                    let group = (r.next_u32() % (n + 1)) as i32 - 1;
                    c.bus.write16(e + 6, group as u16);
                }
                c.pc = 0x0014_c068;
                c.g[A0][0] = sx(obj);
            },
        );
    }

    // ---- DebugMenu_ParseWorldListFile ------------------------------------

    /// Characters of the world list: file-name characters, quotes, separators
    /// and white space (no other character: such a byte makes the scanner
    /// spin forever in the original).
    const OC1528_WORLD_CHARS: &[u8] = b"abcXYZ019_-./ \t\r\n\"'\\:";

    /// `DebugMenu_ParseWorldListFile`: the file loader, the string
    /// constructor/destructor, `strcpy`, `AddWorldIfMissing` and the free are
    /// gated fakes; the text is random over the scanner's character classes.
    #[test]
    fn debug_menu_parse_world_list_file_matches() {
        fakes::check_reaching(
            ee::s02::f_0017a378__generated,
            debug_menu_parse_world_list_file,
            300,
            &[0x0017_a5b0, 0x0017_a604, 0x0017_a614],
            &|c, seed| {
                let mut r = oc1528_machine(c, seed, 0x17a378, 16);
                let obj = OC1528_BASE;
                let text = OC1528_BASE + 0x4000;
                let len = if seed % 13 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 80
                };
                for i in 0..len {
                    let ch = OC1528_WORLD_CHARS[r.next_u32() as usize % OC1528_WORLD_CHARS.len()];
                    c.bus.write8(text + i, ch);
                }
                c.bus.write32(OC1528_SIZE, len);
                c.bus
                    .write32(OC1528_BUF, if seed % 11 == 0 { 0 } else { text });
                c.bus.write32(obj, seed % 4);
                c.pc = 0x0017_a378;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(OC1528_BASE + 0x800);
            },
        );
    }

    // ---- cFEStateCharEquip_ScrollItemList --------------------------------

    /// `cFEStateCharEquip_ScrollItemList`: the row objects at `self+0x80..0xc4`
    /// point at six rows whose vtable calls go to the miss hook; the refill
    /// calls and the sound helpers are gated fakes. `n` runs over -12..12
    /// (both directions, 0, `|n| > 6`), the mode word over 0/1/2, and one seed
    /// in nine has no menu.
    #[test]
    fn cfe_state_char_equip_scroll_item_list_matches() {
        fakes::check_reaching(
            ee::s03::f_0019ad18__generated,
            cfe_state_char_equip_scroll_item_list,
            400,
            &[
                0x0019_ad70,
                0x0019_ad80,
                0x0019_ae1c,
                0x0019_ae50,
                0x0019_ae80,
                0x0019_af24,
                0x0019_af60,
                0x0019_af98,
            ],
            &|c, seed| {
                let mut r = oc1528_machine(c, seed, 0x19ad18, 40);
                let obj = OC1528_BASE;
                let rows = OC1528_BASE + 0x2000;
                let vt = OC1528_BASE + 0x3000;
                for k in 0..6u32 {
                    let row = rows + 0x40 * k;
                    for w in 0..16 {
                        c.bus.write32(row + 4 * w, r.next_u32());
                    }
                    c.bus.write32(row + 8, vt);
                }
                c.bus.write16(vt + 0x58, (r.next_u32() & 0x1c) as u16);
                c.bus.write32(vt + 0x5c, fakes::FAKE);
                c.bus.write16(vt + 0x48, (r.next_u32() & 0x1c) as u16);
                c.bus.write32(vt + 0x4c, fakes::FAKE + 0x10);
                for w in 0..0x11u32 {
                    c.bus
                        .write32(obj + 0x80 + 4 * w, rows + 0x40 * (r.next_u32() % 6));
                }
                c.bus.write32(
                    obj + 0x5c,
                    if seed % 9 == 0 {
                        0
                    } else {
                        OC1528_BASE + 0x2800
                    },
                );
                c.bus.write32(obj + 0xc8, seed % 3);
                let n = (seed % 25) as i32 - 12;
                c.pc = 0x0019_ad18;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(n as u32);
            },
        );
    }

    // ---- cFEStateNetMessenger_RefreshRosterEntries -----------------------

    /// `cFEStateNetMessenger_RefreshRosterEntries`: every session call is a
    /// gated fake. The session global (`gp+0x238`) is zero on one seed in
    /// five (the lazy-init calls run), the count is 0..4, and the per-call
    /// zero mask decides name / flag / record / message answers.
    #[test]
    fn cfe_state_net_messenger_refresh_roster_entries_matches() {
        fakes::check_reaching(
            ee::s03::f_001a6f70__generated,
            cfe_state_net_messenger_refresh_roster_entries,
            400,
            &[0x001a_6f98, 0x001a_7028, 0x001a_7180],
            &|c, seed| {
                let mut r = oc1528_machine(c, seed, 0x1a6f70, 14);
                c.bus
                    .write32(GAME_GP + 0x238, if seed % 5 == 0 { 0 } else { OC1528_BASE });
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), r.next_u32());
                c.bus
                    .write32(OC1528_COUNT, if seed % 7 == 0 { 0 } else { 1 + seed % 4 });
                c.pc = 0x001a_6f70;
            },
        );
    }

    // ---- cFEStateTrophyRoom_SetEventName ---------------------------------

    /// `cFEStateTrophyRoom_SetEventName`: text calls are gated fakes; the
    /// medals table (`0x45aad8`) is filled with random types 0..13 so every
    /// type arm runs, `peak` 0..3, `kind` 0..3 and `idx` 0..4; the world's
    /// BE interface vtable (`gp-0x848` -> `+0x8c` -> `+4`) calls the miss hook.
    #[test]
    fn cfe_state_trophy_room_set_event_name_matches() {
        fakes::check_reaching(
            ee::s04::f_001ce758__generated,
            cfe_state_trophy_room_set_event_name,
            500,
            &[0x001c_e884, 0x001c_e894, 0x001c_e9b4, 0x001c_e998],
            &|c, seed| {
                let mut r = oc1528_machine(c, seed, 0x1ce758, 12);
                for off in 0..0x90u32 {
                    let v = if off % 2 == 0 {
                        r.next_u32() % 14
                    } else {
                        r.next_u32() % 0x20
                    };
                    c.bus.write8(0x0045_aad8 + off, v as u8);
                }
                let world = OC1528_BASE;
                let iface = OC1528_BASE + 0x100;
                let vt = OC1528_BASE + 0x200;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x8c, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 0x20, (r.next_u32() & 0x1c) as u16);
                c.bus.write32(vt + 0x24, fakes::FAKE);
                c.pc = 0x001c_e758;
                c.g[A0][0] = sx(OC1528_BASE + 0x400);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = ((seed / 4) % 4) as u64;
                c.g[A3][0] = ((seed / 16) % 5) as u64;
            },
        );
    }
}
