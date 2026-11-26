//! Staging module oc736 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc736`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0017_a638,
        debug_menu_init_world_list,
        "DebugMenu_InitWorldList",
    ),
    (
        0x0017_acc8,
        debug_menu_load_backgrounds,
        "DebugMenu_LoadBackgrounds",
    ),
    (0x0017_b1b0, debug_menu_ctor, "DebugMenu_ctor"),
    (
        0x0018_1ef0,
        cfe_state_char_select_update_rider_preview,
        "cFEStateCharSelect_UpdateRiderPreview",
    ),
];

/// `void DebugMenu_InitWorldList(TrackFileMan* self)` (0x17a638, the C++
/// `cTrackFileMan_scanLocations`): clears the track list (count, BAM and
/// WC-BAM start counters, the 256 location records), asks the new-race
/// interface for the 22 location file names, lets `DebugMenu_CheckWorldFiles`
/// verify them and then adds one record per name that passed: file name copy,
/// the big-file pointer (`0x4a1290`), location id, BAM flag, and the WC-BAM
/// start flag when `DebugMenu_IsTrackSelectable` agrees.
pub fn debug_menu_init_world_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
        c.g[V0][0] = 0x8000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.g[A1][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[A2][0] = 0x8c00;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S2][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[RA][0]);
        c.sw(c.g[S1][0] as u32, 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc08), 0u64);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc04), 0u64);
        call_ptr(c, 0x003e6448, 0x0017a690)?;
        call_ptr(c, 0x0014dc80, 0x0017a698)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0017a6a4)?;
        c.g[FP][0] = c.g[V0][0];
        c.g[S0][0] = c.g[SP][0];
        'l_17a6b0: loop {
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[FP][0];
            call_ptr(c, 0x00144c48, 0x0017a6bc)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[V0][0] = ((c.g[S2][0] as i64) < 22i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0017a6b0);
                }
                continue 'l_17a6b0;
            } else {
                'b_17a800: {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0x16;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                    call_ptr(c, 0x0017a978, 0x0017a6e8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                    if t {
                        break 'b_17a800;
                    } else {
                        'b_17a7fc: {
                            c.cycles += 4;
                            c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                            c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[S4][0] = 0u64;
                            if t {
                                break 'b_17a7fc;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x88));
                                c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                                'l_17a718: loop {
                                    'b_17a7dc: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = 0x8c;
                                        if t {
                                            break 'b_17a7dc;
                                        } else {
                                            'b_17a7d4: {
                                                c.cycles += 38;
                                                c.g[V0][0] = 0x8000;
                                                c.mult(0, 16, 16, 19);
                                                c.g[S2][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.g[FP][0];
                                                c.g[A1][0] = c.g[S4][0];
                                                c.g[S6][0] = 0x1;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[S0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[S0][0] as u32));
                                                call_ptr(c, 0x00144c48, 0x0017a744)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x004166f4, 0x0017a750)?;
                                                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                                c.g[V0][0] = 0x4a0000;
                                                c.g[A1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x1290));
                                                c.g[A0][0] = c.g[S1][0];
                                                c.mult(0, 2, 3, 19);
                                                c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S1][0] as u32));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x44),
                                                    c.g[A1][0],
                                                );
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                c.mult(0, 3, 2, 19);
                                                c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.sw(c.g[V0][0] as u32, c.g[S4][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                                c.lo[0] = c.g[V0][0];
                                                c.madd(0, 3, 3, 19);
                                                c.sw(c.g[V1][0] as u32, c.g[S6][0]);
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0xc04));
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0xc04),
                                                    c.g[V0][0],
                                                );
                                                c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                                                c.mult(0, 2, 3, 19);
                                                c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                                call_ptr(c, 0x00179fc8, 0x0017a7b0)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                if t {
                                                    break 'b_17a7d4;
                                                } else {
                                                    c.cycles += 7;
                                                    c.mult(0, 3, 2, 19);
                                                    c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[S1][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x8c),
                                                        c.g[S6][0],
                                                    );
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0xc08)
                                                        );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                                    c.sw(
                                                        (c.g[S2][0] as u32).wrapping_add(0xc08),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                                                    break 'b_17a7d4;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                            break 'b_17a7dc;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                    c.g[V0][0] = ((c.g[S4][0] as i64) < 22i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_17a7fc;
                                    } else {
                                        c.cycles += 4;
                                        c.g[S0][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0017a718);
                                            }
                                            continue 'l_17a718;
                                        } else {
                                            break 'b_17a7fc;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x160));
                        break 'b_17a800;
                    }
                }
                c.cycles += 11;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xd0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `int DebugMenu_LoadBackgrounds(BackgroundMan* self)` (0x17acc8, the C++
/// `cBackgroundMan_loadImages`): loads `data/textures/backs.ssh` (`FILE_loadz`),
/// allocates `self->mpiImageTID[count]`, unpacks each shape and registers it
/// as a texture through the graphics manager's vtable, frees the unpacked copy
/// when it differs from the packed shape and finally frees the file. Returns 0
/// when the file is missing, else 1.
pub fn debug_menu_load_backgrounds(c: &mut Cpu) {
    guest(c, |c| {
        'b_17add4: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
            c.g[A1][0] = 0x100;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S4][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x31d0));
            call_ptr(c, 0x003e1908, 0x0017ad00)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = 0u64;
            if t {
                break 'b_17add4;
            } else {
                'b_17adc0: {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12a0));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A3][0] = 0u64;
                    c.g[S2][0] = 0u64;
                    call_ptr(c, 0x00317e30, 0x0017ad30)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw(c.g[S4][0] as u32, c.g[V0][0]);
                    if t {
                        break 'b_17adc0;
                    } else {
                        c.cycles += 3;
                        c.g[S6][0] = 0x4a0000;
                        c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                        'l_17ad48: loop {
                            'b_17adb0: {
                                'b_17adac: {
                                    c.cycles += 21;
                                    c.g[S0][0] = c.lw(c.g[S5][0] as u32);
                                    c.g[S0][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x003b3578, 0x0017ad58)?;
                                    c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x12a8));
                                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                                    c.g[A3][0] = 0u64;
                                    c.g[T0][0] = 0x1;
                                    c.g[T1][0] = 0xffffffffffffffff;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0017ad88)?;
                                    c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                                    c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    let t = c.g[S1][0] == c.g[S0][0];
                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_17adac;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[S1][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                            break 'b_17adb0;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x00317e98, 0x0017adac)?;
                                            break 'b_17adac;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                                break 'b_17adb0;
                            }
                            c.cycles += 4;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x8));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0017ad48);
                                }
                                continue 'l_17ad48;
                            } else {
                                break 'b_17adc0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[S3][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_17add4;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00317e98, 0x0017add0)?;
                    c.g[V0][0] = 0x1;
                    break 'b_17add4;
                }
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        Ok(())
    });
}

/// `DebugMenu* DebugMenu_ctor(DebugMenu* self)` (0x17b1b0, the C++
/// `cDebugMenuScreen` constructor): sets the vtable, builds the background and
/// world lists, configures the back-end interfaces (humans, game path/mode/event,
/// player controller ids and character, the audio playlist), picks the first
/// enabled location and returns `self`.
pub fn debug_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0x470000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2868));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0x8c10;
        c.g[S3][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S6][0] = 0x530000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.sw(c.g[S3][0] as u32, c.g[V0][0]);
        call_ptr(c, 0x0017abc8, 0x0017b1f8)?;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x0017a638, 0x0017b200)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0017acc8, 0x0017b208)?;
        c.g[S0][0] = 0x8000;
        call_ptr(c, 0x0014dc80, 0x0017b210)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x0017b21c)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A1][0] = 0x7;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014dd58, 0x0017b230)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc24), 0u64);
        call_ptr(c, 0x0014dc80, 0x0017b240)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc24));
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x1;
        c.sw(c.g[V0][0] as u32, c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc24), 0u64);
        call_ptr(c, 0x00144aa8, 0x0017b258)?;
        c.g[V0][0] = 0x8c40;
        c.g[A0][0] = 0x4;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = 0x2;
        'l_17b268: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x0017b268);
                }
                continue 'l_17b268;
            } else {
                'b_17b310: {
                    c.cycles += 15;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00145108, 0x0017b290)?;
                    c.g[S2][0] = 0x8000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0x4;
                    call_ptr(c, 0x00144df0, 0x0017b2a0)?;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0xc;
                    call_ptr(c, 0x001451e8, 0x0017b2b0)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), 0u64);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x704));
                    let t = c.g[V0][0] == 0u64;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x4b30), 0u64);
                    if t {
                        break 'b_17b310;
                    } else {
                        c.cycles += 20;
                        call_ptr(c, 0x00266cd8, 0x0017b2c8)?;
                        call_ptr(c, 0x00261008, 0x0017b2d0)?;
                        call_ptr(c, 0x0025b688, 0x0017b2d8)?;
                        call_ptr(c, 0x00255898, 0x0017b2e0)?;
                        call_ptr(c, 0x00255860, 0x0017b2e8)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                        c.g[A1][0] = 0x4a0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12b0));
                        call_ptr(c, 0x00255f20, 0x0017b2f8)?;
                        call_ptr(c, 0x0025b650, 0x0017b300)?;
                        call_ptr(c, 0x00260f80, 0x0017b308)?;
                        call_ptr(c, 0x00266ca0, 0x0017b310)?;
                        break 'b_17b310;
                    }
                }
                'b_17b360: {
                    'b_17b35c: {
                        'b_17b350: {
                            c.cycles += 6;
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc08));
                            let t = c.g[V1][0] == 0u64;
                            c.g[V0][0] = 0x8c;
                            if t {
                                break 'b_17b350;
                            } else {
                                c.cycles += 6;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc20));
                                c.mult(0, 3, 5, 2);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                                let t = c.g[V1][0] != 0u64;
                                c.g[V0][0] = 0x8000;
                                if t {
                                    break 'b_17b360;
                                } else {
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x0017a028, 0x0017b348)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc20), c.g[V0][0]);
                                    break 'b_17b35c;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0017d008, 0x0017b35c)?;
                        break 'b_17b35c;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x8000;
                    break 'b_17b360;
                }
                'b_17b3a4: {
                    c.cycles += 17;
                    c.g[A1][0] = 0x8c;
                    c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc20));
                    c.g[A0][0] = 0x1;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                    c.mult(0, 6, 2, 5);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc20));
                    c.mult(0, 5, 2, 5);
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                    if c.g[V0][0] == 0u64 {
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), 0u64);
                        break 'b_17b3a4;
                    } else {
                        break 'b_17b3a4;
                    }
                }
                'b_17b3c4: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.g[V1][0] = 0x4;
                    if t {
                        break 'b_17b3c4;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call_ptr(c, 0x00144b20, 0x0017b3c0)?;
                        c.g[V1][0] = 0x4;
                        break 'b_17b3c4;
                    }
                }
                c.cycles += 70;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5b20));
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc1c), c.g[V1][0]);
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4b30));
                c.g[V1][0] = 0xe4a;
                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0017b3f4)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc44), 0u64);
                c.g[V0][0] = 0xffffffffffffffff;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc34), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc30), c.g[V0][0]);
                call_ptr(c, 0x0014dc80, 0x0017b408)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0017b414)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.g[S1][0];
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x001472c8, 0x0017b428)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x001472c8, 0x0017b438)?;
                c.g[V0][0] = 0x2;
                c.g[A2][0] = 0x2;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc28), c.g[V0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00147448, 0x0017b450)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00147618, 0x0017b45c)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc2c), c.g[V0][0]);
                call_ptr(c, 0x0028b180, 0x0017b464)?;
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0028b180, 0x0017b46c)?;
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc34));
                c.g[V1][0] = 0x14;
                c.g[A0][0] = c.g[S0][0];
                c.mult(0, 5, 5, 3);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60a4));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, 0x00289520, 0x0017b488)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0017b49c)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0017b4b0)?;
                c.g[V0][0] = c.g[S3][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateCharSelect_UpdateRiderPreview(cFEStateCharSelect* self)`
/// (0x181ef0, the C++ `updateCharacter3DPos`): when the first human rider is
/// ready (state below 10, its animation counters set) and the preview flag is
/// on, builds the rider's preview rotation from the three axis-angle tables
/// (three `sincos`-style calls and quaternion multiplies on VU0), poses the
/// rider and its board, restarts its animation after a pose change and makes it
/// visible.
pub fn cfe_state_char_select_update_rider_preview(c: &mut Cpu) {
    guest(c, |c| {
        'b_181f5c: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0548, 0x00181f34)?;
            c.g[S6][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(c.g[S6][0] as u32);
            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0u64;
            if t {
                break 'b_181f5c;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb8));
                let t = c.g[V0][0] == 0u64;
                if t {
                    break 'b_181f5c;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb4));
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_181f5c;
                }
            }
        }
        'b_1821f4: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
            if t {
                break 'b_1821f4;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_1821f4;
                } else {
                    'b_1821b4: {
                        c.cycles += 132;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 3);
                        c.g[AT][0] = 0x43080000;
                        c.mtc1(1, 0);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                        c.g[AT][0] = 0xffffffffc37a0000;
                        c.mtc1(1, 1);
                        c.g[S0][0] = 0x500000;
                        c.g[AT][0] = 0xffffffffc2a40000;
                        c.mtc1(1, 2);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = 0x500000;
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xeb0));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                        c.g[S4][0] = 0x500000;
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
                        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xea0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(2, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lwc1(20, (c.g[S7][0] as u32).wrapping_add(0x60));
                        call_ptr(c, 0x0031be50, 0x00181fe4)?;
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x78));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7c));
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xec0));
                        c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(2, 0, 2);
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.mul_s(0, 0, 4);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x5c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mtc1(0, 12);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x0031be50, 0x00182060)?;
                        c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                        c.lwc1(1, (c.g[S2][0] as u32).wrapping_sub(0xeb0));
                        c.g[AT][0] = 0x42c80000;
                        c.mtc1(1, 3);
                        c.mul_s(2, 0, 2);
                        c.lwc1(4, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.add_s(20, 20, 3);
                        c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x5a14));
                        c.mul_s(0, 0, 4);
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.mul_s(20, 20, 5);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.g[AT][0] = 0x3f000000;
                        c.mtc1(1, 12);
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.mul_s(12, 20, 12);
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x0031be50, 0x001820f8)?;
                        c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[S1][0] = c.g[A1][0];
                        c.lwc1(3, (c.g[S4][0] as u32).wrapping_sub(0xea0));
                        c.g[A0][0] = c.g[S6][0];
                        c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(3, 0, 3);
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                        c.mul_s(0, 0, 4);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x0019f3e8, 0x00182174)?;
                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x54));
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'b_1821b4;
                        } else {
                            'b_1821b0: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                if c.g[S0][0] == 0u64 {
                                    c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), 0u64);
                                    break 'b_1821b0;
                                } else {
                                    c.cycles += 9;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00311a50, 0x00182194)?;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x1b2;
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x003128e8, 0x001821ac)?;
                                    c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), 0u64);
                                    break 'b_1821b0;
                                }
                            }
                            c.cycles += 1;
                            c.sw((c.g[S7][0] as u32).wrapping_add(0x54), 0u64);
                            break 'b_1821b4;
                        }
                    }
                    c.cycles += 16;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x5a10));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = c.g[S6][0];
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x3c));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x0019f548, 0x001821e4)?;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0019e538, 0x001821f0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                    break 'b_1821f4;
                }
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x120));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fill, put_words, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    const FX_CTRL: u32 = DATA + 0x20_0000;
    const FX_MAGIC: u32 = 0x4f43_3736;
    const FX_SELF: u32 = DATA + 0x21_0000;
    const FX_BGFILE: u32 = DATA + 0x22_0000;
    const FX_GRMAN: u32 = DATA + 0x23_0000;
    const FX_GRVT: u32 = DATA + 0x24_0000;
    const FX_WORLD: u32 = DATA + 0x25_0000;
    const FX_OBJ: u32 = DATA + 0x26_0000;
    const FX_RIDER: u32 = DATA + 0x27_0000;
    const FX_IFACE: u32 = DATA + 0x28_0000;
    const FX_IFVT: u32 = DATA + 0x29_0000;
    /// Per-callee forced results: one word per fake, at `FX_FORCE + (addr & 0xffffc)`.
    const FX_FORCE: u32 = DATA + 0x30_0000;

    /// The mailbox word of the fake for `addr`.
    fn oc736_mail(addr: u32) -> u32 {
        FX_FORCE + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (any value; `v` is stored +3).
    fn oc736_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc736_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    fn oc736_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc736_mail(addr), 1);
    }

    /// Make the fake for `addr` return its first argument.
    fn oc736_force_echo(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc736_mail(addr), 2);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    fn oc736_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc736_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc736_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            fn oc736_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc736_fakes!(
        (
            oc736_fake_144c48,
            OC736_PREV_144C48,
            ee::s01::OVERRIDE_00144C48,
            ee::s01::f_00144c48__generated,
            0x0014_4c48
        ),
        (
            oc736_fake_14dc80,
            OC736_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc736_fake_14dd58,
            OC736_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc736_fake_144aa8,
            OC736_PREV_144AA8,
            ee::s01::OVERRIDE_00144AA8,
            ee::s01::f_00144aa8__generated,
            0x0014_4aa8
        ),
        (
            oc736_fake_144b20,
            OC736_PREV_144B20,
            ee::s01::OVERRIDE_00144B20,
            ee::s01::f_00144b20__generated,
            0x0014_4b20
        ),
        (
            oc736_fake_144df0,
            OC736_PREV_144DF0,
            ee::s01::OVERRIDE_00144DF0,
            ee::s01::f_00144df0__generated,
            0x0014_4df0
        ),
        (
            oc736_fake_145108,
            OC736_PREV_145108,
            ee::s01::OVERRIDE_00145108,
            ee::s01::f_00145108__generated,
            0x0014_5108
        ),
        (
            oc736_fake_1451e8,
            OC736_PREV_1451E8,
            ee::s01::OVERRIDE_001451E8,
            ee::s01::f_001451e8__generated,
            0x0014_51e8
        ),
        (
            oc736_fake_1472c8,
            OC736_PREV_1472C8,
            ee::s01::OVERRIDE_001472C8,
            ee::s01::f_001472c8__generated,
            0x0014_72c8
        ),
        (
            oc736_fake_147448,
            OC736_PREV_147448,
            ee::s01::OVERRIDE_00147448,
            ee::s01::f_00147448__generated,
            0x0014_7448
        ),
        (
            oc736_fake_147618,
            OC736_PREV_147618,
            ee::s01::OVERRIDE_00147618,
            ee::s01::f_00147618__generated,
            0x0014_7618
        ),
        (
            oc736_fake_179fc8,
            OC736_PREV_179FC8,
            ee::s02::OVERRIDE_00179FC8,
            ee::s02::f_00179fc8__generated,
            0x0017_9fc8
        ),
        (
            oc736_fake_17a028,
            OC736_PREV_17A028,
            ee::s02::OVERRIDE_0017A028,
            ee::s02::f_0017a028__generated,
            0x0017_a028
        ),
        (
            oc736_fake_17d008,
            OC736_PREV_17D008,
            ee::s02::OVERRIDE_0017D008,
            ee::s02::f_0017d008__generated,
            0x0017_d008
        ),
        (
            oc736_fake_3e6448,
            OC736_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc736_fake_3e1908,
            OC736_PREV_3E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
        (
            oc736_fake_4166f4,
            OC736_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc736_fake_41aa88,
            OC736_PREV_41AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88
        ),
        (
            oc736_fake_4165a8,
            OC736_PREV_4165A8,
            ee::s15::OVERRIDE_004165A8,
            ee::s15::f_004165a8__generated,
            0x0041_65a8
        ),
        (
            oc736_fake_3182d8,
            OC736_PREV_3182D8,
            ee::s10::OVERRIDE_003182D8,
            ee::s10::f_003182d8__generated,
            0x0031_82d8
        ),
        (
            oc736_fake_3186d0,
            OC736_PREV_3186D0,
            ee::s10::OVERRIDE_003186D0,
            ee::s10::f_003186d0__generated,
            0x0031_86d0
        ),
        (
            oc736_fake_318260,
            OC736_PREV_318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
        (
            oc736_fake_318630,
            OC736_PREV_318630,
            ee::s10::OVERRIDE_00318630,
            ee::s10::f_00318630__generated,
            0x0031_8630
        ),
        (
            oc736_fake_3168b0,
            OC736_PREV_3168B0,
            ee::s10::OVERRIDE_003168B0,
            ee::s10::f_003168b0__generated,
            0x0031_68b0
        ),
        (
            oc736_fake_3168e8,
            OC736_PREV_3168E8,
            ee::s10::OVERRIDE_003168E8,
            ee::s10::f_003168e8__generated,
            0x0031_68e8
        ),
        (
            oc736_fake_317e30,
            OC736_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc736_fake_317e98,
            OC736_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc736_fake_311a50,
            OC736_PREV_311A50,
            ee::s10::OVERRIDE_00311A50,
            ee::s10::f_00311a50__generated,
            0x0031_1a50
        ),
        (
            oc736_fake_3128e8,
            OC736_PREV_3128E8,
            ee::s10::OVERRIDE_003128E8,
            ee::s10::f_003128e8__generated,
            0x0031_28e8
        ),
        (
            oc736_fake_31be50,
            OC736_PREV_31BE50,
            ee::s10::OVERRIDE_0031BE50,
            ee::s10::f_0031be50__generated,
            0x0031_be50
        ),
        (
            oc736_fake_3a6948,
            OC736_PREV_3A6948,
            ee::s13::OVERRIDE_003A6948,
            ee::s13::f_003a6948__generated,
            0x003a_6948
        ),
        (
            oc736_fake_3b3578,
            OC736_PREV_3B3578,
            ee::s13::OVERRIDE_003B3578,
            ee::s13::f_003b3578__generated,
            0x003b_3578
        ),
        (
            oc736_fake_28b180,
            OC736_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc736_fake_2b3eb0,
            OC736_PREV_2B3EB0,
            ee::s08::OVERRIDE_002B3EB0,
            ee::s08::f_002b3eb0__generated,
            0x002b_3eb0
        ),
        (
            oc736_fake_2a77f8,
            OC736_PREV_2A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc736_fake_2a7c68,
            OC736_PREV_2A7C68,
            ee::s08::OVERRIDE_002A7C68,
            ee::s08::f_002a7c68__generated,
            0x002a_7c68
        ),
        (
            oc736_fake_2a7a20,
            OC736_PREV_2A7A20,
            ee::s08::OVERRIDE_002A7A20,
            ee::s08::f_002a7a20__generated,
            0x002a_7a20
        ),
        (
            oc736_fake_2a7f90,
            OC736_PREV_2A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90
        ),
        (
            oc736_fake_2a7ff8,
            OC736_PREV_2A7FF8,
            ee::s08::OVERRIDE_002A7FF8,
            ee::s08::f_002a7ff8__generated,
            0x002a_7ff8
        ),
        (
            oc736_fake_2b3ee8,
            OC736_PREV_2B3EE8,
            ee::s08::OVERRIDE_002B3EE8,
            ee::s08::f_002b3ee8__generated,
            0x002b_3ee8
        ),
        (
            oc736_fake_2a7890,
            OC736_PREV_2A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc736_fake_19e538,
            OC736_PREV_19E538,
            ee::s03::OVERRIDE_0019E538,
            ee::s03::f_0019e538__generated,
            0x0019_e538
        ),
        (
            oc736_fake_19f3e8,
            OC736_PREV_19F3E8,
            ee::s03::OVERRIDE_0019F3E8,
            ee::s03::f_0019f3e8__generated,
            0x0019_f3e8
        ),
        (
            oc736_fake_19f548,
            OC736_PREV_19F548,
            ee::s03::OVERRIDE_0019F548,
            ee::s03::f_0019f548__generated,
            0x0019_f548
        ),
        (
            oc736_fake_1a0548,
            OC736_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
    );

    /// Machine state shared by the four tests: scrambled registers, fake
    /// hooks, the fakes' gate, stack junk below `sp` (half of its words are 0,
    /// so frame slots the callees fill later read as both set and clear), the
    /// world at `gp-0x848`, and the graphics manager (`gp-0x854`) with a vtable
    /// whose methods are fake calls.
    fn oc736_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(FX_CTRL, FX_MAGIC);
        for i in 1..=0x300u32 {
            let w = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32()
            };
            c.bus.write32(crate::difftest::SP - 4 * i, w);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), FX_WORLD);
        c.bus.write32(FX_WORLD + 0x7c, FX_OBJ);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), FX_GRMAN);
        c.bus.write32(FX_GRMAN + 0x10d8, FX_GRVT);
        // Every method slot the tests reach: {delta, fake address}.
        for off in [0x170u32, 0x190] {
            c.bus.write16(FX_GRVT + off, 0x10 * (seed % 3) as u16);
            c.bus.write32(FX_GRVT + off + 4, fakes::FAKE + 0x10);
        }
        // The back-end interface the ctor calls virtual methods of.
        c.bus.write32(FX_IFACE + 0xc, FX_IFVT);
        c.bus.write16(FX_IFVT + 8, 0x20);
        c.bus.write32(FX_IFVT + 0xc, fakes::FAKE + 0x20);
        oc736_force(c, 0x0014_dd58, FX_IFACE);
        r
    }

    /// `backs.ssh` for `LoadBackgrounds`: `seed % 5 == 0` the file is missing,
    /// else `seed % 4` images; the shape unpacker answers 0, its argument (no
    /// free) or its own pointer (free) by `(seed / 5) % 3`.
    fn oc736_backgrounds(c: &mut Cpu, seed: u32) {
        if seed % 5 == 0 {
            oc736_force_zero(c, 0x003e_1908);
        } else {
            oc736_force(c, 0x003e_1908, FX_BGFILE);
        }
        c.bus.write32(FX_BGFILE + 8, seed % 4);
        for i in 0..4u32 {
            c.bus.write32(FX_BGFILE + 0x14 + 8 * i, 0x100 + 0x40 * i);
        }
        match (seed / 5) % 3 {
            0 => {}
            1 => oc736_force_zero(c, 0x003b_3578),
            _ => oc736_force_echo(c, 0x003b_3578),
        }
    }

    /// The scan of `InitWorldList`: whether the file check (`0x3a6948`, the
    /// result of `CheckWorldFiles`) succeeds and `IsTrackSelectable` accepts.
    fn oc736_world_scan(c: &mut Cpu, seed: u32) {
        if seed % 3 == 0 {
            oc736_force_zero(c, 0x003a_6948);
        }
        if (seed / 3) % 2 == 0 {
            oc736_force_zero(c, 0x0017_9fc8);
        }
        c.bus.write32(0x004a_1290, 0x0044_0000 + 0x10 * seed);
    }

    /// `DebugMenu_InitWorldList`: failing and passing file checks, records
    /// accepted by 50 % of the junk `ok` words, selectable and not.
    #[test]
    fn debug_menu_init_world_list_matches() {
        oc736_install_fakes();
        fakes::covered(
            ee::s02::f_0017a638__generated,
            debug_menu_init_world_list,
            250,
            &|c, seed| {
                let mut r = oc736_fixture(c, seed, 0x0017_a638, 120);
                oc736_world_scan(c, seed);
                fill(c, &mut r, FX_SELF, 0x40);
                c.g[A0][0] = sx(FX_SELF);
                c.pc = 0x0017_a638;
            },
        );
    }

    /// `DebugMenu_LoadBackgrounds`: no file, 0 to 3 images, the unpacker's
    /// three answers.
    #[test]
    fn debug_menu_load_backgrounds_matches() {
        oc736_install_fakes();
        fakes::covered(
            ee::s02::f_0017acc8__generated,
            debug_menu_load_backgrounds,
            250,
            &|c, seed| {
                let mut r = oc736_fixture(c, seed, 0x0017_acc8, 40);
                oc736_backgrounds(c, seed);
                fill(c, &mut r, FX_SELF, 8);
                c.g[A0][0] = sx(FX_SELF);
                c.pc = 0x0017_acc8;
            },
        );
    }

    /// `DebugMenu_ctor`, with the real `InitBackgroundList`, `InitWorldList`
    /// (and its `CheckWorldFiles`), `LoadBackgrounds` and `0x289520` running
    /// on the fakes. The `EnableOnline` branch (`gp-0x704`) calls unported
    /// callees without a slot and stays off.
    #[test]
    fn debug_menu_ctor_matches() {
        oc736_install_fakes();
        fakes::covered(
            ee::s02::f_0017b1b0__generated,
            debug_menu_ctor,
            150,
            &|c, seed| {
                let mut r = oc736_fixture(c, seed, 0x0017_b1b0, 140);
                oc736_world_scan(c, seed);
                oc736_backgrounds(c, seed / 2);
                oc736_force_zero(c, 0x002a_7c68);
                oc736_force_zero(c, 0x0041_65a8);
                // The menu object: junk words (a third of them 0), then the
                // location record 0: its id (22 on every 4th seed) and flags.
                for i in 0..0x100u32 {
                    let w = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(FX_SELF + 4 * i, w);
                }
                c.bus
                    .write32(FX_SELF + 0x88, if seed % 4 == 0 { 0x16 } else { seed % 7 });
                c.bus.write32(FX_SELF + 0x8c, (seed / 4) % 2);
                c.bus.write32(FX_SELF + 0x90, (seed / 8) % 2);
                c.g[A0][0] = sx(FX_SELF);
                c.pc = 0x0017_b1b0;
            },
        );
    }

    /// `cFEStateCharSelect_UpdateRiderPreview`: the rider not ready (state
    /// 10 and up, no animation counters), ready with and without the pose
    /// change flag (`+0x54`), with and without the rider's animation object
    /// (`+0xc`), with the preview flag clear.
    #[test]
    fn cfe_state_char_select_update_rider_preview_matches() {
        oc736_install_fakes();
        fakes::covered(
            ee::s02::f_00181ef0__generated,
            cfe_state_char_select_update_rider_preview,
            250,
            &|c, seed| {
                let mut r = oc736_fixture(c, seed, 0x0018_1ef0, 10);
                oc736_force(c, 0x001a_0548, FX_RIDER);
                // The rider: state, animation counters, animation object.
                c.bus
                    .write32(FX_RIDER, if seed % 5 == 0 { 12 } else { seed % 10 });
                c.bus
                    .write32(FX_RIDER + 0xcb8, if (seed / 5) % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(FX_RIDER + 0xcb4, if (seed / 15) % 3 == 0 { 0 } else { 5 });
                c.bus
                    .write32(FX_RIDER + 0xc, if (seed / 3) % 2 == 0 { 0 } else { FX_OBJ });
                // The state object: rider index, pose flag, angle, preview flag.
                c.bus.write8(FX_SELF + 0x44, seed as u8);
                c.bus.write32(FX_SELF + 0x54, seed % 2);
                c.bus.write32(FX_SELF + 0x60, r.f32(3.0));
                c.bus
                    .write32(FX_SELF + 0x80, if (seed / 2) % 4 == 0 { 0 } else { 1 });
                // The axis tables (three vec3 + w at 0x4ff140) and two
                // gp-relative scale floats.
                let table: Vec<u32> = (0..12).map(|_| r.f32(3.0)).collect();
                put_words(c, 0x004f_f140, &table);
                c.bus.write32(GAME_GP.wrapping_sub(0x5a14), r.f32(3.0));
                c.bus.write32(GAME_GP.wrapping_sub(0x5a10), r.f32(3.0));
                c.g[A0][0] = sx(FX_SELF);
                c.pc = 0x0018_1ef0;
            },
        );
    }
}
