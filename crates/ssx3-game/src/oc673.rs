//! Staging module oc673 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc673`.

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
        0x0013_1d30,
        rider_railslide_sub_handle_input,
        "RiderRailslideSub_HandleInput",
    ),
    (
        0x0014_db40,
        bolts_find_table2_entries_by_flag,
        "Bolts_FindTable2EntriesByFlag",
    ),
    (0x0014_9bb8, belibrary_load_char_db, "BELibrary_LoadCharDB"),
    (0x0032_5f48, input_preproc_run, "InputPreproc_Run"),
    (0x0022_6900, ssx_app_init, "SSXApp_Init"),
    (0x0023_55c0, game_state_reset_enter, "GameStateReset_Enter"),
    (
        0x0023_3048,
        c_game_load_screen_load_pictures,
        "cGameLoadScreen_LoadPictures",
    ),
    (0x0014_9860, be_library_ctor, "BELibrary_ctor"),
    (0x0026_c458, playback_man_ctor, "PlaybackMan_ctor"),
];

/// `void RiderRailslideSub_HandleInput(RiderRailslideSub* self, Input* in)`
/// (0x131d30): per-tick input handling while grinding a rail. Early outs: the
/// reset path, `RiderRailslideSub_CheckAirborne`, prewind and the uber grind
/// (`RiderRailslideSub_SetRail`). Then the boost input, the slew 3/4/5 targets
/// and the timed-event and idle-look animations. With the rail animation in
/// category 0xe it waits for the board-press event flag, tries the board
/// press and chooses the rail spin from the spin input; otherwise it tries the
/// board press, picks the desired spin (spin input, else the board-press
/// input), and with no spin input steers the balance slew toward the rail
/// offset (x1.2, clamped to +-1) and starts the rail grind animation (18, 19
/// for rail mode 3, 20 for mode 4). The block structure of the generated body
/// is kept (it shares exits and the final spin choice).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn rider_railslide_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_13202c: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A2][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = c.lw((c.g[S1][0] as u32));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 12) as u32);
            c.g[A1][0] = c.g[A1][0] & 0x1;
            call_ptr(c, 0x00116120, 0x00131d68)?;
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_13202c;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x00132770, 0x00131d7c)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_13202c;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 13) as u32);
                    c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 14) as u32);
                    c.g[A2][0] = c.g[A2][0] & 0x1;
                    c.g[A1][0] = c.g[A1][0] & 0x1;
                    call_ptr(c, 0x001162c8, 0x00131da0)?;
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_13202c;
                    } else {
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 7);
                        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
                        call_ptr(c, 0x00132620, 0x00131dbc)?;
                        if c.g[V0][0] != 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_13202c;
                        } else {
                            'b_131f70: {
                                'b_131f58: {
                                    'b_131ec4: {
                                        c.cycles += 32;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 15) as u32);
                                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x75a8));
                                        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
                                        c.g[A2][0] = c.g[A2][0] & 0x1;
                                        c.mtc1(0, 21);
                                        c.g[A1][0] = c.g[A1][0] & 0x1;
                                        call_ptr(c, 0x00114130, 0x00131de8)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                        c.mtc1(2, 12);
                                        c.cvt_s_w(12, 12);
                                        c.mul_s(12, 12, 20);
                                        call_ptr(c, 0x00113f38, 0x00131e08)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.mov_s(13, 21);
                                        c.mov_s(12, 21);
                                        call_ptr(c, 0x00113f88, 0x00131e18)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        call_ptr(c, 0x00115b58, 0x00131e20)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        call_ptr(c, 0x00115d48, 0x00131e28)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x2;
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                                        call_ptr(c, 0x00311ae8, 0x00131e38)?;
                                        c.g[V1][0] = 0xe;
                                        if c.g[V0][0] != c.g[V1][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                            break 'b_131ec4;
                                        } else {
                                            c.cycles += 9;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0x2;
                                            c.g[A0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                                            call_ptr(c, 0x00311b20, 0x00131e54)?;
                                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb0));
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x001446a0, 0x00131e60)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                break 'b_13202c;
                                            } else {
                                                c.cycles += 10;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                                                c.g[V0][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                                c.mtc1(2, 12);
                                                c.cvt_s_w(12, 12);
                                                c.mul_s(12, 12, 20);
                                                call_ptr(c, 0x001161d0, 0x00131e88)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    break 'b_13202c;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
                                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32
                                                        >> 26)
                                                        as u32);
                                                    c.mtc1(2, 0);
                                                    c.cvt_s_w(0, 0);
                                                    c.mul_s(0, 0, 20);
                                                    c.c_eq_s(0, 21);
                                                    if c.fcc() {
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        break 'b_13202c;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.c_lt_s(0, 21);
                                                        let t = true;
                                                        c.g[A1][0] = 0x1;
                                                        if t {
                                                            break 'b_131f58;
                                                        } else {
                                                            break 'b_131ec4;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 9;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                    c.mtc1(2, 12);
                                    c.cvt_s_w(12, 12);
                                    c.mul_s(12, 12, 20);
                                    call_ptr(c, 0x001161d0, 0x00131ee0)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        break 'b_13202c;
                                    } else {
                                        'b_131f30: {
                                            c.cycles += 14;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 20);
                                            c.g[V1][0] =
                                                sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
                                            c.mtc1(3, 0);
                                            c.cvt_s_w(0, 0);
                                            c.g[V0][0] =
                                                sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
                                            c.mtc1(2, 1);
                                            c.cvt_s_w(1, 1);
                                            c.mul_s(2, 0, 20);
                                            c.c_eq_s(2, 21);
                                            let t = c.fcc();
                                            c.mul_s(0, 1, 20);
                                            if t {
                                                break 'b_131f30;
                                            } else {
                                                c.cycles += 4;
                                                c.c_eq_s(0, 21);
                                                if c.fcc() {
                                                    c.mov_s(0, 2);
                                                    break 'b_131f30;
                                                } else {
                                                    break 'b_131f30;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.mtc1(0, 20);
                                        c.c_eq_s(0, 20);
                                        let t = c.fcc();
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_131f70;
                                        } else {
                                            c.cycles += 5;
                                            c.c_lt_s(0, 20);
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x75a4));
                                            c.g[A1][0] = 0x1;
                                            c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x240));
                                            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x23c));
                                            break 'b_131f58;
                                        }
                                    }
                                }
                                'b_131f60: {
                                    c.cycles += 2;
                                    if !c.fcc() {
                                        c.g[A1][0] = 0u64;
                                        break 'b_131f60;
                                    } else {
                                        break 'b_131f60;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00132060, 0x00131f68)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_13202c;
                                } else {
                                    break 'b_131f70;
                                }
                            }
                            'b_132028: {
                                'b_131f9c: {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x2;
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                                    call_ptr(c, 0x00311ae8, 0x00131f7c)?;
                                    c.g[V1][0] = 0xa;
                                    if c.g[V0][0] != c.g[V1][0] {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_131f9c;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x75a0));
                                        c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x240));
                                        let t = true;
                                        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x23c));
                                        if t {
                                            break 'b_132028;
                                        } else {
                                            break 'b_131f9c;
                                        }
                                    }
                                }
                                'b_131fd0: {
                                    c.cycles += 12;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x759c));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x77c));
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 20);
                                    c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 2);
                                    c.mul_s(1, 1, 0);
                                    c.c_le_s(20, 1);
                                    let t = !c.fcc();
                                    c.mov_s(3, 20);
                                    if t {
                                        break 'b_131fd0;
                                    } else {
                                        c.cycles += 1;
                                        c.min_s(3, 1, 2);
                                        break 'b_131fd0;
                                    }
                                }
                                'b_132014: {
                                    c.cycles += 13;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7598));
                                    c.g[A1][0] = 0x2;
                                    c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x240));
                                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x23c));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
                                    call_ptr(c, 0x00312aa0, 0x00131fec)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x4;
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x328));
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[A1][0] = 0x14;
                                    if t {
                                        break 'b_132014;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.g[V1][0] ^ 0x3;
                                        c.g[A1][0] = 0x12;
                                        c.g[V0][0] = 0x13;
                                        if c.g[V1][0] == 0 {
                                            c.g[A1][0] = c.g[V0][0];
                                        }
                                        break 'b_132014;
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[A2][0] == c.g[A1][0];
                                c.mov_s(12, 20);
                                if t {
                                    break 'b_132028;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x784));
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x003128e8, 0x00132028)?;
                                    break 'b_132028;
                                }
                            }
                            c.cycles += 1;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_13202c;
                        }
                    }
                }
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `Econ* Bolts_FindTable2EntriesByFlag(Bolts* self, int index, int kind,
/// int want, int* out)` (0x14db40): walk the `table2[index]` entries (12 bytes
/// each: flag byte at +1, kind halfword at +2) and return the first one whose
/// kind equals `kind` and whose flag (`> 0`) equals `want`, counting every
/// match in `*out` (the list is sorted by kind: a larger kind ends the walk).
pub fn bolts_find_table2_entries_by_flag(c: &mut Cpu) {
    c.cycles += 7;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    Addr(c.g[T0][0] as u32).sw(c, 0);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20c));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T2][0] = Addr(c.g[V1][0] as u32).lw(c);
    let has_entries = (c.g[T2][0] as i64) > 0;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x284));
    if !has_entries {
        c.cycles += 2;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[T1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T2][0] as u32));
    c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let empty = c.g[V0][0] == 0;
    c.g[T3][0] = 0;
    'exit: {
        if empty {
            break 'exit;
        }
        c.cycles += 3;
        c.g[V0][0] = 0xc;
        c.g[T4][0] = c.g[V1][0];
        c.mult(0, 10, 9, 2);
        loop {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(8).lw(c);
            c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
            if c.g[V0][0] != c.g[A2][0] {
                // Other kind: past ours (the list is sorted) ends the walk.
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    break 'exit;
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(1).lb(c);
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                if c.g[V0][0] != c.g[A3][0] {
                    c.cycles += 2;
                    if c.g[T3][0] != 0 {
                        break 'exit;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
                    if c.g[T3][0] == 0 {
                        c.g[T3][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                    let count = c.g[V0][0];
                    Addr(c.g[T0][0] as u32).sw(c, count);
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                }
            }
            c.cycles += 6;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.g[V1][0] = Addr(c.g[T4][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0xc));
            if !more {
                break 'exit;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_db88);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T3][0];
    ret(c);
}

/// `void BELibrary::LoadCharDB()` (0x149bb8, `cBELibrary::loadData`): reset
/// the back-end library: the bolts DB, the reward DB, the race DB (6 ids), the
/// six rider slots, load `data/be/charDB.dbl` into the character table, build
/// the ten rows of the equipment value table (+5 of the base table),
/// initialise the three profile libraries and fill the grab-point score table
/// with its constants.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn belibrary_load_char_db(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[A0][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6750));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S3][0] = 0x530000;
        Callee {
            addr: 0x0014c6a0,
            f: ee::s01::Bolts_LoadDB,
        }
        .call(c, 0x00149be0)?;
        c.g[A0][0] = 0x4c0000;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3e98));
        Callee {
            addr: 0x0015ad58,
            f: ee::s01::RewardsFile_Load,
        }
        .call(c, 0x00149bf0)?;
        c.g[S0][0] = 0x530000;
        c.g[A1][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b0));
        c.g[A2][0] = 0x4c;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00416210, 0x00149c08)?;
        c.g[T4][0] = 0u64;
        'l_149c10: loop {
            c.cycles += 7;
            c.sw(c.g[S0][0] as u32, c.g[T4][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[T4][0] as i64) < 6i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00149c10);
                }
                continue 'l_149c10;
            } else {
                c.cycles += 9;
                c.g[S0][0] = 0x530000;
                c.g[A1][0] = 0u64;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5b20));
                c.g[A2][0] = 0xa8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416210, 0x00149c44)?;
                c.g[A0][0] = 0x4;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa8));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                'l_149c50: loop {
                    c.cycles += 8;
                    c.sb(c.g[S0][0] as u32, 0u64);
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb((c.g[S0][0] as u32).wrapping_add(0x11), c.g[A0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        if event_due(c) {
                            deliver_event_here(c, 0x00149c50);
                        }
                        continue 'l_149c50;
                    } else {
                        'b_149cac: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x460000;
                            c.g[A1][0] = 0x100;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5d20));
                            call_ptr(c, 0x003e18c8, 0x00149c80)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = 0x530000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x970));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = 0x550;
                            call_ptr(c, 0x0041605c, 0x00149c98)?;
                            let t = c.g[S0][0] == 0u64;
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_149cac;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x00149ca8)?;
                                c.g[V0][0] = 0x530000;
                                break 'b_149cac;
                            }
                        }
                        c.cycles += 7;
                        c.g[V1][0] = 0x440000;
                        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8d8));
                        c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf00));
                        c.g[T4][0] = 0u64;
                        c.g[T5][0] = 0xb;
                        c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0xd));
                        'l_149cc8: loop {
                            c.cycles += 35;
                            c.g[A0][0] = c.lbu(c.g[T2][0] as u32);
                            c.g[A2][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x1));
                            c.g[A3][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5));
                            c.g[A1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x5));
                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x5));
                            c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x6));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5));
                            c.sb(c.g[T1][0] as u32, c.g[T4][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x1), c.g[A0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x7), c.g[V0][0]);
                            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x2), c.g[A2][0]);
                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x7));
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x3), c.g[A3][0]);
                            c.g[V0][0] = ((c.g[T4][0] as i64) < 10i64) as u64;
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x4), c.g[T0][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x5), c.g[A1][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0x6), c.g[V1][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x4), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x2), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x5), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_sub(0x1), c.g[T5][0]);
                            c.sb((c.g[T3][0] as u32).wrapping_add(0x1), c.g[T5][0]);
                            c.sb(c.g[T3][0] as u32, c.g[T5][0]);
                            c.sb((c.g[T1][0] as u32).wrapping_add(0xa), c.g[T5][0]);
                            c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0xf));
                            let t = c.g[V0][0] != 0u64;
                            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xf));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00149cc8);
                                }
                                continue 'l_149cc8;
                            } else {
                                c.cycles += 13;
                                c.g[A0][0] = 0x530000;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5898));
                                c.g[A2][0] = 0x288;
                                call_ptr(c, 0x00416210, 0x00149d68)?;
                                c.g[S2][0] = 0x9b50;
                                c.g[V1][0] = 0x4a0000;
                                c.g[V0][0] = 0x10000;
                                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6ca8));
                                c.g[V0][0] = c.g[V0][0] | 0xd1f0;
                                c.g[S1][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.g[S0][0];
                                'l_149d88: loop {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001567b8,
                                        f: ee::s01::BE_ResetProfileCharacters,
                                    }
                                    .call(c, 0x00149d90)?;
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00149d88);
                                        }
                                        continue 'l_149d88;
                                    } else {
                                        c.cycles += 182;
                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x600));
                                        c.sw((c.g[S3][0] as u32).wrapping_add(0x600), 0u64);
                                        c.g[A1][0] = 0xc8;
                                        c.g[V1][0] = 0x190;
                                        c.g[A3][0] = 0x12c;
                                        c.g[T1][0] = 0xfa;
                                        c.g[T0][0] = 0x15e;
                                        c.g[A2][0] = 0x1f4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[T0][0]);
                                        c.g[A0][0] = 0xbb8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x48), c.g[T1][0]);
                                        c.g[T2][0] = 0x3e8;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                                        c.g[T0][0] = 0x7d0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
                                        c.g[T3][0] = 0x9c4;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                                        c.g[T4][0] = 0xfa0;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                                        c.g[T5][0] = 0x1388;
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x54), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x60), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x68), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[A3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[V1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x108), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xac), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xcc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xd8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xec), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf0), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf4), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x100), c.g[A1][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x104), c.g[A2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x148), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x158), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x168), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x16c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x150), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x154), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x160), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x164), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x170), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x174), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x178), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x17c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x188), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x198), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x19c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x180), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x184), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x190), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x194), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1bc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x118), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x11c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x128), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x200), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x138), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x13c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x120), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x124), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x130), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x134), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x140), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x144), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x208), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x218), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x21c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x228), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x22c), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x210), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x214), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x220), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x224), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x230), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x234), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d8), c.g[T2][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1dc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e8), c.g[T0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), c.g[T3][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1f4), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x204), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x238), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), c.g[T4][0]);
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a4), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x280), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x284), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x240), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x244), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x248), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x250), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x254), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x258), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x25c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x260), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x264), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x268), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x26c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x270), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x274), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x278), c.g[A0][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), c.g[T4][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x288), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x290), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x294), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x298), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), c.g[T5][0]);
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x2a0), c.g[T5][0]);
                                        let t = c.g[RA][0] as u32;
                                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
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
    })
}

/// `char* InputPreproc_Run(InputPreproc* self, const char* path, int* size)`
/// (0x325f48, `cInputPreProcessor::loadFileSize`): resets the preprocessor
/// (size 0, capacity 0x800, a 0x800-byte output buffer), splits `path` at its
/// last `/` into the directory (kept at `sp+0`, empty when there is no `/`)
/// and the file name (`sp+0x80`), runs `InputPreproc_ProcessFile` over it, copies
/// the output to a fresh buffer of exactly its length, frees the work buffer,
/// stores the length in `*size` and returns the copy.
pub fn input_preproc_run(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x800;
        frame.save(c, S3, 0xd0);
        c.g[A3][0] = 0;
        frame.save(c, S4, 0xc0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0xe0);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S5, 0xb0);
        c.g[A1][0] = 0x49_0000;
        frame.save(c, S0, 0x100);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0xf0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1c48));
        frame.save_ra(c, 0xa0);
        c.g[A0][0] = 0x800;
        c.sw(c.g[S3][0] as u32, 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S3][0] as u32).wrapping_add(4), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0032_5f98)?; // operator_new_named
        c.sw((c.g[S3][0] as u32).wrapping_add(8), c.g[V0][0]);
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0x2f;
        call_ptr(c, 0x0041_acc0, 0x0032_5fa8)?; // strrchr
        c.g[S2][0] = c.g[V0][0];
        let no_slash = c.g[S2][0] == 0;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
        if !no_slash {
            c.cycles += 13;
            c.g[A0][0] = c.g[SP][0];
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[S4][0] as u32));
            c.g[A1][0] = c.g[S4][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_6ce0, 0x0032_5fc8)?; // strncpy (directory)
            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sb(c.g[V1][0] as u32, 0);
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0041_66f4, 0x0032_5fe0)?; // strcpy (file name)
            c.g[A2][0] = c.g[S1][0];
        } else {
            c.cycles += 6;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.sb(c.g[SP][0] as u32, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, 0x0041_66f4, 0x0032_5ffc)?; // strcpy (file name)
            c.g[A2][0] = c.g[S0][0];
        }
        c.cycles += 16;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x0032_6078, 0x0032_600c)?; // InputPreproc_ProcessFile
        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0032_6020)?; // operator_new_named
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(8));
        c.g[A2][0] = c.lw(c.g[S3][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_605c, 0x0032_6034)?; // memcpy
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(8));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0032_6048)?; // EA_MemFree
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        }
        c.cycles += 11;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x100);
        c.sw(c.g[S5][0] as u32, c.g[V1][0]);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXApp_Init(cSSXApp* self, int argc, char** argv)` (0x226900): the
/// application's start-up: registers the app at `gp-0x848`, the state config
/// (`+0x9c`), the pad queue (`gp-0x850`, a 0x2efc-byte object), the graphics
/// manager (`gp-0x854`) with its vtable setup, the light and bezier managers,
/// the byte at `gp-0x844` and the two 0x2a4-byte pad records at `+0xa8`, the
/// command line, the graphics mode (slot 0x160, 0x100), the boot shape
/// (`SHPS_Find` + slot 0x170, into `+0xa0`), the 0x1c-byte white tint (`+0xa4`),
/// the pre-start screen (`+0xc0`) and the FE async manager (`+0x11c`).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn ssx_app_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A2][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa8));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = 0x1;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[S5][0] = 0x480000;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x848), c.g[S2][0]);
        call_ptr(c, 0x002c7440, 0x00226940)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65c0));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x2efc;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
        call_ptr(c, 0x00317d70, 0x0022695c)?;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        call_ptr(c, 0x00326a28, 0x0022696c)?;
        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x850), c.g[V0][0]);
        call_ptr(c, 0x00375a08, 0x00226978)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10d8));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x854), c.g[V0][0]);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00226990)?;
        call_ptr(c, 0x0038dbd0, 0x00226998)?;
        call_ptr(c, 0x0038aef8, 0x002269a0)?;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x1;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x65b0));
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317d70, 0x002269b8)?;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x844), c.g[V0][0]);
        'l_2269c0: loop {
            c.cycles += 10;
            c.g[A0][0] = 0x2a4;
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x65a0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x002269d4)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.sw(c.g[V0][0] as u32, 0u64);
            let t = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002269c0);
                }
                continue 'l_2269c0;
            } else {
                c.cycles += 94;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x84), 0u64);
                c.g[A2][0] = c.g[S3][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x80), 0u64);
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), 0u64);
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, 0x00227bf0, 0x00226a04)?;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A1][0] = 0x100;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00226a20)?;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A0][0] = 0x480000;
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x54c0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28b0));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x170));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x003b39b8, 0x00226a44)?;
                c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                c.g[A2][0] = 0x480000;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffffffffffffffff;
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6590));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[T2][0] as u32;
                c.g[A3][0] = 0u64;
                call_ptr(c, t, 0x00226a68)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                call_ptr(c, 0x00369ff8, 0x00226a78)?;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = 0x1c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6580));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x00226a90)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x480000;
                c.sw(c.g[V0][0] as u32, 0u64);
                c.g[A2][0] = 0x100;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.g[A3][0] = 0u64;
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6570));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S2][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x1f));
                c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x13));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = 0xc;
                call_ptr(c, 0x00317d70, 0x00226af0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00232328, 0x00226af8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00316ef8, 0x00226b04)?;
                call_ptr(c, 0x00237cf8, 0x00226b0c)?;
                c.g[A1][0] = 0x480000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6560));
                c.g[A3][0] = 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                c.g[A0][0] = 0x590;
                call_ptr(c, 0x00317d70, 0x00226b28)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3590, 0x00226b30)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x11c), c.g[V0][0]);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x001a3648, 0x00226b3c)?;
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
                return Ok(());
            }
        }
        Ok(())
    })
}

/// `void GameStateReset_Enter(GameStateReset* self)` (0x2355c0): enters the
/// "reset" game state: records whether the script engine is in a transport
/// sequence (`+0x1c`), clears the user awards, reads the race setup
/// (`0x535bc8`: byte `+0x49` game path, byte `+0x48` game mode), picks the
/// next state (`+0x0`, `+0x4`) from the course and race mode, switches the AI
/// state in mode 4, resets the BE state, restarts the missions and the global
/// scripts, enters the hub when the location type is 1, starts the NIS lists
/// (path set) or the idle list (`+0x14`), and re-enables the audio engine.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn game_state_reset_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_2355fc: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, 0x0027aa80, 0x002355f0)?; // call_indirect
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2355fc;
            } else {
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                break 'b_2355fc;
            }
        }
        'b_235684: {
            'b_235674: {
                'b_235670: {
                    c.cycles += 17;
                    call_ptr(c, 0x0014dc80, 0x00235604)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00235610)?; // call_indirect
                    c.g[S2][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x00235618)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x00235624)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00158e30, 0x0023562c)?; // call_indirect
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S4][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        break 'b_235670;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_235674;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x4;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                            let t = true;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_235684;
                            } else {
                                break 'b_235670;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_235674;
            }
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 0x1u64) as u64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[V0][0] = 0x4;
            break 'b_235684;
        }
        'b_2356a0: {
            c.cycles += 2;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00113b10, 0x0023569c)?; // call_indirect
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            }
        }
        'b_23571c: {
            c.cycles += 13;
            c.g[S5][0] = 0x1;
            call_ptr(c, 0x0014de68, 0x002356a8)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309030, 0x002356b0)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309f18, 0x002356b8)?; // call_indirect
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144be0, 0x002356c0)?; // call_indirect
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144cc0, 0x002356cc)?; // call_indirect
            let t = c.g[V0][0] != c.g[S5][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_23571c;
            } else {
                c.cycles += 18;
                call_ptr(c, 0x0014dc80, 0x002356dc)?; // call_indirect
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x002356e8)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00144be0, 0x002356f4)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e10, 0x00235704)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235718)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                break 'b_23571c;
            }
        }
        'b_2357b0: {
            'b_235798: {
                c.cycles += 11;
                c.g[V1][0] = c.g[S4][0] ^ 0x4;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0u64;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                call_ptr(c, 0x001f36d8, 0x00235740)?; // call_indirect
                if c.g[S3][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    break 'b_235798;
                } else {
                    c.cycles += 7;
                    c.sw(c.g[S1][0] as u32, c.g[S5][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_2357b0;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0x1;
                        call_ptr(c, 0x00278f68, 0x00235770)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2357b0;
                        } else {
                            c.cycles += 5;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x00278f68, 0x00235790)?; // call_indirect
                            let t = true;

                            if t {
                                break 'b_2357b0;
                            } else {
                                break 'b_235798;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if t {
                break 'b_2357b0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00278f68, 0x002357b0)?; // call_indirect
                break 'b_2357b0;
            }
        }
        c.cycles += 17;
        call_ptr(c, 0x0028b180, 0x002357b8)?; // call_indirect
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, 0x002b3a98, 0x002357c0)?; // call_indirect
        call_ptr(c, 0x0028b180, 0x002357c8)?; // call_indirect
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a4b68, 0x002357d0)?; // call_indirect
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
        Ok(())
    })
}

/// `void cGameLoadScreen_LoadPictures(cGameLoadScreen* self, int)` (0x233048):
/// loads the load screen's textures from a BigFile. Rider race (the byte at
/// `0x535c11` is 2): the two characters' pictures (`sprintf` of the character
/// ids of both players, `FILE_loadpackz`, then a texture load through the
/// graphics manager's vtable `+0x170` into `self + 0x10 + 4 * i`). Otherwise:
/// the course picture of the location id (0 when above 16) into `self + 0x10`.
/// The BigFile is closed with flag 2.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn c_game_load_screen_load_pictures(c: &mut Cpu) {
    guest(c, |c| {
        'b_233238: {
            'b_233180: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00233078)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00233084)?; // call_indirect
                c.g[S3][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x0023308c)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00233098)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = 0x2;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                let t = c.g[A0][0] != c.g[V1][0];
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_233180;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = 0x480000;
                    c.g[S7][0] = c.g[A0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4288));
                    c.g[A2][0] = 0x100;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003168b0, 0x002330cc)?; // call_indirect
                    c.g[S1][0] = 0u64;
                    c.g[V0][0] = 0x440000;
                    c.g[S5][0] = 0x480000;
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2928));
                    c.g[S4][0] = 0x480000;
                    'l_2330e0: loop {
                        'b_23315c: {
                            c.cycles += 29;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = (0u64 < c.g[S1][0]) as u64;
                            call_ptr(c, 0x00147488, 0x002330ec)?; // call_indirect
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x5b4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4270));
                            call_ptr(c, 0x00417828, 0x00233108)?; // call_indirect
                            c.g[A1][0] = 0x3000000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = c.g[A1][0] | 0x100;
                            call_ptr(c, 0x003e2190, 0x00233118)?; // call_indirect
                            c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x4260));
                            c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[T0][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                            c.g[T1][0] = 0xffffffffffffffff;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0023314c)?; // call_indirect
                            let t = c.g[S0][0] == 0u64;
                            c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_23315c;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00317e98, 0x0023315c)?; // call_indirect
                                break 'b_23315c;
                            }
                        }
                        c.cycles += 4;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002330e0);
                            }
                            continue 'l_2330e0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S7][0];
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003168e8, 0x00233178)?; // call_indirect
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                            if t {
                                break 'b_233238;
                            } else {
                                break 'b_233180;
                            }
                        }
                    }
                }
            }
            'b_233228: {
                c.cycles += 40;
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4250));
                c.g[A2][0] = 0x100;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003168b0, 0x00233194)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144be0, 0x0023319c)?; // call_indirect
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[A2][0] = 0x440000;
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2950));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 17i64) as u64;
                c.g[A1][0] = 0x4a0000;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = 0u64;
                }
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2c30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                call_ptr(c, 0x00417828, 0x002331d0)?; // call_indirect
                c.g[A1][0] = 0x3000000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[A1][0] | 0x100;
                call_ptr(c, 0x003e2190, 0x002331e0)?; // call_indirect
                c.g[T2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                c.g[A2][0] = 0x480000;
                c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10d8));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4238));
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A3][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x170));
                c.g[T0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x174));
                c.g[T1][0] = 0xffffffffffffffff;
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00233218)?; // call_indirect
                let t = c.g[S0][0] == 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_233228;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e98, 0x00233228)?; // call_indirect
                    break 'b_233228;
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x003168e8, 0x00233234)?; // call_indirect
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            break 'b_233238;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    })
}

/// `(game mode, game event, cycles)` that `SectionMan_OnEventTrigger` sets for
/// the section events 2..6 (the generated switch's cases).
pub(crate) fn oc673_section_event_modes(event: u32) -> Option<(u32, u32, u64)> {
    match event {
        2 => Some((0, 0, 8)),
        3 => Some((1, 1, 8)),
        4 => Some((2, 3, 8)),
        5 => Some((3, 2, 8)),
        6 => Some((5, 4, 6)),
        _ => None,
    }
}

/// `BELibrary* BELibrary_ctor(self)` (0x149860): loads the character DB
/// (`BELibrary_LoadCharDB`), copies the 0xa28-byte default block at 0x43fb28
/// to 0x535c18 (8-byte aligned: the aligned 32-byte copy loop always runs; 81
/// iterations plus 8 bytes) and creates every interface singleton, in order
/// (race, player, char DB, char, option, save, BAGT, network, score, bolt,
/// mission, econ, aggression, reward). Returns `self`. Frame 0x20 (s0).
pub fn be_library_ctor(c: &mut Cpu) {
    const LOAD_CHAR_DB: Callee = Callee {
        addr: 0x0014_9bb8,
        f: ee::s01::f_00149bb8,
    };
    const INSTANCES: [(Callee, u32); 14] = [
        (
            Callee {
                addr: 0x0014_48d8,
                f: ee::s01::f_001448d8,
            },
            0x0014_9928,
        ),
        (
            Callee {
                addr: 0x0014_5970,
                f: ee::s01::f_00145970,
            },
            0x0014_9930,
        ),
        (
            Callee {
                addr: 0x0014_edd8,
                f: ee::s01::f_0014edd8,
            },
            0x0014_9938,
        ),
        (
            Callee {
                addr: 0x0014_7f78,
                f: ee::s01::f_00147f78,
            },
            0x0014_9940,
        ),
        (
            Callee {
                addr: 0x0014_f250,
                f: ee::s01::f_0014f250,
            },
            0x0014_9948,
        ),
        (
            Callee {
                addr: 0x0015_2638,
                f: ee::s01::f_00152638,
            },
            0x0014_9950,
        ),
        (
            Callee {
                addr: 0x0014_f900,
                f: ee::s01::f_0014f900,
            },
            0x0014_9958,
        ),
        (
            Callee {
                addr: 0x0014_e0e0,
                f: ee::s01::f_0014e0e0,
            },
            0x0014_9960,
        ),
        (
            Callee {
                addr: 0x0015_4948,
                f: ee::s01::f_00154948,
            },
            0x0014_9968,
        ),
        (
            Callee {
                addr: 0x0014_abe0,
                f: ee::s01::f_0014abe0,
            },
            0x0014_9970,
        ),
        (
            Callee {
                addr: 0x0015_32a0,
                f: ee::s01::f_001532a0,
            },
            0x0014_9978,
        ),
        (
            Callee {
                addr: 0x0015_08d8,
                f: ee::s01::f_001508d8,
            },
            0x0014_9980,
        ),
        (
            Callee {
                addr: 0x0015_5a50,
                f: ee::s01::f_00155a50,
            },
            0x0014_9988,
        ),
        (
            Callee {
                addr: 0x0015_69c0,
                f: ee::s01::f_001569c0,
            },
            0x0014_9990,
        ),
    ];
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        LOAD_CHAR_DB.call(c, 0x0014_9874)?;
        c.g[V0][0] = 0x53_0000;
        c.g[V1][0] = 0x44_0000;
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(0x5c18));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x4d8));
        c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
        c.g[V0][0] &= 7;
        let unaligned = c.g[V0][0] != 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(0xa20));
        debug_assert!(!unaligned, "0x43fb28 and 0x535c18 are 8-byte aligned");
        loop {
            c.cycles += 11;
            let src = r32(c, V1);
            let dst = r32(c, A0);
            c.g[A1][0] = c.ld(src);
            c.g[A2][0] = c.ld(src.wrapping_add(8));
            c.g[A3][0] = c.ld(src.wrapping_add(0x10));
            c.g[T0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[A1][0]);
            c.sd(dst.wrapping_add(8), c.g[A2][0]);
            c.sd(dst.wrapping_add(0x10), c.g[A3][0]);
            c.sd(dst.wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0014_98e8);
            }
        }
        c.cycles += 36;
        let src = r32(c, V1);
        let dst = r32(c, A0);
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        for (callee, ret) in INSTANCES {
            callee.call(c, ret)?;
        }
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `PlaybackMan* PlaybackMan_ctor(PlaybackMan* self, int a1)` (0x26c458): build the
/// replay playback manager: the embedded `WorldStateStream` (`+0x14`), the world
/// and rider-table words copied from the game world, two allocated helpers (a
/// pointer cell and a pair set with `ValuePair_SetFirst/SetSecond`) and a
/// `BigFile` opened from a formatted name; returns `self`.
pub fn playback_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[S1][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee {
            addr: 0x0026d5e8,
            f: ee::s07::WorldStateStream_ctor,
        }
        .call(c, 0x0026c47c)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] = 0x2a4;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1550));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[T1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[T1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[S1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[T0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0u64);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xdf0));
        c.g[T0][0] = c.lw(c.g[V1][0] as u32);
        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[T0][0]);
        call_ptr(c, 0x00317d70, 0x0026c4e4)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V0][0]);
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0x20000000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3420));
        c.g[A0][0] = 0x8;
        c.sw(c.g[V0][0] as u32, 0u64);
        call_ptr(c, 0x00317d70, 0x0026c504)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00244400, 0x0026c514)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00244408, 0x0026c520)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[S1][0]);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x328));
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x320));
        call_ptr(c, 0x00417828, 0x0026c534)?;
        c.g[A1][0] = 0x4a0000;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3428));
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x00317d70, 0x0026c54c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00316898, 0x0026c554)?;
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x00316978, 0x0026c568)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC673_CTRL`.
    pub(crate) const OC673_CTRL: u32 = DATA + 0xf_e000;
    pub(crate) const OC673_MAGIC: u32 = 0x6730_73c1;
    /// `v0` of fake call `n` (0: first call) at `OC673_RESULTS + 4n`, or
    /// `OC673_NONE` for the fake's own pointer.
    pub(crate) const OC673_RESULTS: u32 = OC673_CTRL + 0x40;
    pub(crate) const OC673_NONE: u32 = 0xdead_beef;
    /// `(address, v0)` pairs: the result of every fake call to `address`.
    pub(crate) const OC673_BY_ADDR: u32 = OC673_CTRL + 0x100;

    /// A gated fake call: logs and clobbers like `fakes::fake`, then forces
    /// `v0` from the result table (unless the call unwound).
    pub(crate) fn oc673_fake_body(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc == ra && n < 32 {
            let v = c.bus.read32(OC673_RESULTS + 4 * n);
            if v != OC673_NONE {
                c.g[V0][0] = sx(v);
            }
            for k in 0..16 {
                let slot = OC673_BY_ADDR + 8 * k;
                if c.bus.read32(slot) == addr {
                    c.g[V0][0] = sx(c.bus.read32(slot + 4));
                }
            }
        }
    }

    /// Gated fakes of the callees of this module's functions (each `hook`ed
    /// once per process; with the gate off they run the body they replaced).
    macro_rules! oc673_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC673_CTRL) != OC673_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc673_fake_body(c, $addr)
                }
            )*
            pub(crate) fn oc673_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc673_fakes! {
        oc673_fake_00113b10, OC673_PREV_00113B10, 0x0011_3b10, ee::s00::OVERRIDE_00113B10, ee::s00::f_00113b10__generated;
        oc673_fake_00113f38, OC673_PREV_00113F38, 0x0011_3f38, ee::s00::OVERRIDE_00113F38, ee::s00::f_00113f38__generated;
        oc673_fake_00113f88, OC673_PREV_00113F88, 0x0011_3f88, ee::s00::OVERRIDE_00113F88, ee::s00::f_00113f88__generated;
        oc673_fake_00114130, OC673_PREV_00114130, 0x0011_4130, ee::s00::OVERRIDE_00114130, ee::s00::f_00114130__generated;
        oc673_fake_00115b58, OC673_PREV_00115B58, 0x0011_5b58, ee::s00::OVERRIDE_00115B58, ee::s00::f_00115b58__generated;
        oc673_fake_00115d48, OC673_PREV_00115D48, 0x0011_5d48, ee::s00::OVERRIDE_00115D48, ee::s00::f_00115d48__generated;
        oc673_fake_00116120, OC673_PREV_00116120, 0x0011_6120, ee::s00::OVERRIDE_00116120, ee::s00::f_00116120__generated;
        oc673_fake_001161d0, OC673_PREV_001161D0, 0x0011_61d0, ee::s00::OVERRIDE_001161D0, ee::s00::f_001161d0__generated;
        oc673_fake_001162c8, OC673_PREV_001162C8, 0x0011_62c8, ee::s00::OVERRIDE_001162C8, ee::s00::f_001162c8__generated;
        oc673_fake_0012b180, OC673_PREV_0012B180, 0x0012_b180, ee::s00::OVERRIDE_0012B180, ee::s00::f_0012b180__generated;
        oc673_fake_00132060, OC673_PREV_00132060, 0x0013_2060, ee::s01::OVERRIDE_00132060, ee::s01::f_00132060__generated;
        oc673_fake_00132620, OC673_PREV_00132620, 0x0013_2620, ee::s01::OVERRIDE_00132620, ee::s01::f_00132620__generated;
        oc673_fake_00132770, OC673_PREV_00132770, 0x0013_2770, ee::s01::OVERRIDE_00132770, ee::s01::f_00132770__generated;
        oc673_fake_001446a0, OC673_PREV_001446A0, 0x0014_46a0, ee::s01::OVERRIDE_001446A0, ee::s01::f_001446a0__generated;
        oc673_fake_001448d8, OC673_PREV_001448D8, 0x0014_48d8, ee::s01::OVERRIDE_001448D8, ee::s01::f_001448d8__generated;
        oc673_fake_00144be0, OC673_PREV_00144BE0, 0x0014_4be0, ee::s01::OVERRIDE_00144BE0, ee::s01::f_00144be0__generated;
        oc673_fake_00144cc0, OC673_PREV_00144CC0, 0x0014_4cc0, ee::s01::OVERRIDE_00144CC0, ee::s01::f_00144cc0__generated;
        oc673_fake_00144df0, OC673_PREV_00144DF0, 0x0014_4df0, ee::s01::OVERRIDE_00144DF0, ee::s01::f_00144df0__generated;
        oc673_fake_001451e8, OC673_PREV_001451E8, 0x0014_51e8, ee::s01::OVERRIDE_001451E8, ee::s01::f_001451e8__generated;
        oc673_fake_00145970, OC673_PREV_00145970, 0x0014_5970, ee::s01::OVERRIDE_00145970, ee::s01::f_00145970__generated;
        oc673_fake_00146e10, OC673_PREV_00146E10, 0x0014_6e10, ee::s01::OVERRIDE_00146E10, ee::s01::f_00146e10__generated;
        oc673_fake_00147488, OC673_PREV_00147488, 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc673_fake_00147f78, OC673_PREV_00147F78, 0x0014_7f78, ee::s01::OVERRIDE_00147F78, ee::s01::f_00147f78__generated;
        oc673_fake_0014abe0, OC673_PREV_0014ABE0, 0x0014_abe0, ee::s01::OVERRIDE_0014ABE0, ee::s01::f_0014abe0__generated;
        oc673_fake_0014c6a0, OC673_PREV_0014C6A0, 0x0014_c6a0, ee::s01::OVERRIDE_0014C6A0, ee::s01::f_0014c6a0__generated;
        oc673_fake_0014dc80, OC673_PREV_0014DC80, 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc673_fake_0014dd58, OC673_PREV_0014DD58, 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc673_fake_0014de68, OC673_PREV_0014DE68, 0x0014_de68, ee::s01::OVERRIDE_0014DE68, ee::s01::f_0014de68__generated;
        oc673_fake_0014e0e0, OC673_PREV_0014E0E0, 0x0014_e0e0, ee::s01::OVERRIDE_0014E0E0, ee::s01::f_0014e0e0__generated;
        oc673_fake_0014edd8, OC673_PREV_0014EDD8, 0x0014_edd8, ee::s01::OVERRIDE_0014EDD8, ee::s01::f_0014edd8__generated;
        oc673_fake_0014f250, OC673_PREV_0014F250, 0x0014_f250, ee::s01::OVERRIDE_0014F250, ee::s01::f_0014f250__generated;
        oc673_fake_0014f900, OC673_PREV_0014F900, 0x0014_f900, ee::s01::OVERRIDE_0014F900, ee::s01::f_0014f900__generated;
        oc673_fake_001508d8, OC673_PREV_001508D8, 0x0015_08d8, ee::s01::OVERRIDE_001508D8, ee::s01::f_001508d8__generated;
        oc673_fake_00152638, OC673_PREV_00152638, 0x0015_2638, ee::s01::OVERRIDE_00152638, ee::s01::f_00152638__generated;
        oc673_fake_001532a0, OC673_PREV_001532A0, 0x0015_32a0, ee::s01::OVERRIDE_001532A0, ee::s01::f_001532a0__generated;
        oc673_fake_00154948, OC673_PREV_00154948, 0x0015_4948, ee::s01::OVERRIDE_00154948, ee::s01::f_00154948__generated;
        oc673_fake_00155a50, OC673_PREV_00155A50, 0x0015_5a50, ee::s01::OVERRIDE_00155A50, ee::s01::f_00155a50__generated;
        oc673_fake_001567b8, OC673_PREV_001567B8, 0x0015_67b8, ee::s01::OVERRIDE_001567B8, ee::s01::f_001567b8__generated;
        oc673_fake_001569c0, OC673_PREV_001569C0, 0x0015_69c0, ee::s01::OVERRIDE_001569C0, ee::s01::f_001569c0__generated;
        oc673_fake_00158e30, OC673_PREV_00158E30, 0x0015_8e30, ee::s01::OVERRIDE_00158E30, ee::s01::f_00158e30__generated;
        oc673_fake_0015ad58, OC673_PREV_0015AD58, 0x0015_ad58, ee::s01::OVERRIDE_0015AD58, ee::s01::f_0015ad58__generated;
        oc673_fake_001a3590, OC673_PREV_001A3590, 0x001a_3590, ee::s03::OVERRIDE_001A3590, ee::s03::f_001a3590__generated;
        oc673_fake_001a3648, OC673_PREV_001A3648, 0x001a_3648, ee::s03::OVERRIDE_001A3648, ee::s03::f_001a3648__generated;
        oc673_fake_001f36d8, OC673_PREV_001F36D8, 0x001f_36d8, ee::s04::OVERRIDE_001F36D8, ee::s04::f_001f36d8__generated;
        oc673_fake_00227bf0, OC673_PREV_00227BF0, 0x0022_7bf0, ee::s05::OVERRIDE_00227BF0, ee::s05::f_00227bf0__generated;
        oc673_fake_00231250, OC673_PREV_00231250, 0x0023_1250, ee::s06::OVERRIDE_00231250, ee::s06::f_00231250__generated;
        oc673_fake_00232328, OC673_PREV_00232328, 0x0023_2328, ee::s06::OVERRIDE_00232328, ee::s06::f_00232328__generated;
        oc673_fake_00237cf8, OC673_PREV_00237CF8, 0x0023_7cf8, ee::s06::OVERRIDE_00237CF8, ee::s06::f_00237cf8__generated;
        oc673_fake_00238160, OC673_PREV_00238160, 0x0023_8160, ee::s06::OVERRIDE_00238160, ee::s06::f_00238160__generated;
        oc673_fake_00244400, OC673_PREV_00244400, 0x0024_4400, ee::s06::OVERRIDE_00244400, ee::s06::f_00244400__generated;
        oc673_fake_00244408, OC673_PREV_00244408, 0x0024_4408, ee::s06::OVERRIDE_00244408, ee::s06::f_00244408__generated;
        oc673_fake_0026d5e8, OC673_PREV_0026D5E8, 0x0026_d5e8, ee::s07::OVERRIDE_0026D5E8, ee::s07::f_0026d5e8__generated;
        oc673_fake_00278f68, OC673_PREV_00278F68, 0x0027_8f68, ee::s07::OVERRIDE_00278F68, ee::s07::f_00278f68__generated;
        oc673_fake_0027aa80, OC673_PREV_0027AA80, 0x0027_aa80, ee::s07::OVERRIDE_0027AA80, ee::s07::f_0027aa80__generated;
        oc673_fake_0028b180, OC673_PREV_0028B180, 0x0028_b180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated;
        oc673_fake_002a4b68, OC673_PREV_002A4B68, 0x002a_4b68, ee::s08::OVERRIDE_002A4B68, ee::s08::f_002a4b68__generated;
        oc673_fake_002b3a98, OC673_PREV_002B3A98, 0x002b_3a98, ee::s08::OVERRIDE_002B3A98, ee::s08::f_002b3a98__generated;
        oc673_fake_002c7440, OC673_PREV_002C7440, 0x002c_7440, ee::s08::OVERRIDE_002C7440, ee::s08::f_002c7440__generated;
        oc673_fake_00309030, OC673_PREV_00309030, 0x0030_9030, ee::s10::OVERRIDE_00309030, ee::s10::f_00309030__generated;
        oc673_fake_00309f18, OC673_PREV_00309F18, 0x0030_9f18, ee::s10::OVERRIDE_00309F18, ee::s10::f_00309f18__generated;
        oc673_fake_00311ae8, OC673_PREV_00311AE8, 0x0031_1ae8, ee::s10::OVERRIDE_00311AE8, ee::s10::f_00311ae8__generated;
        oc673_fake_00311b20, OC673_PREV_00311B20, 0x0031_1b20, ee::s10::OVERRIDE_00311B20, ee::s10::f_00311b20__generated;
        oc673_fake_003128e8, OC673_PREV_003128E8, 0x0031_28e8, ee::s10::OVERRIDE_003128E8, ee::s10::f_003128e8__generated;
        oc673_fake_00312aa0, OC673_PREV_00312AA0, 0x0031_2aa0, ee::s10::OVERRIDE_00312AA0, ee::s10::f_00312aa0__generated;
        oc673_fake_00316898, OC673_PREV_00316898, 0x0031_6898, ee::s10::OVERRIDE_00316898, ee::s10::f_00316898__generated;
        oc673_fake_003168b0, OC673_PREV_003168B0, 0x0031_68b0, ee::s10::OVERRIDE_003168B0, ee::s10::f_003168b0__generated;
        oc673_fake_003168e8, OC673_PREV_003168E8, 0x0031_68e8, ee::s10::OVERRIDE_003168E8, ee::s10::f_003168e8__generated;
        oc673_fake_00316978, OC673_PREV_00316978, 0x0031_6978, ee::s10::OVERRIDE_00316978, ee::s10::f_00316978__generated;
        oc673_fake_00316ef8, OC673_PREV_00316EF8, 0x0031_6ef8, ee::s10::OVERRIDE_00316EF8, ee::s10::f_00316ef8__generated;
        oc673_fake_00317d70, OC673_PREV_00317D70, 0x0031_7d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc673_fake_00317e30, OC673_PREV_00317E30, 0x0031_7e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc673_fake_00317e98, OC673_PREV_00317E98, 0x0031_7e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc673_fake_00326078, OC673_PREV_00326078, 0x0032_6078, ee::s10::OVERRIDE_00326078, ee::s10::f_00326078__generated;
        oc673_fake_00326a28, OC673_PREV_00326A28, 0x0032_6a28, ee::s10::OVERRIDE_00326A28, ee::s10::f_00326a28__generated;
        oc673_fake_00369ff8, OC673_PREV_00369FF8, 0x0036_9ff8, ee::s12::OVERRIDE_00369FF8, ee::s12::f_00369ff8__generated;
        oc673_fake_00375a08, OC673_PREV_00375A08, 0x0037_5a08, ee::s12::OVERRIDE_00375A08, ee::s12::f_00375a08__generated;
        oc673_fake_0038ae10, OC673_PREV_0038AE10, 0x0038_ae10, ee::s12::OVERRIDE_0038AE10, ee::s12::f_0038ae10__generated;
        oc673_fake_0038d638, OC673_PREV_0038D638, 0x0038_d638, ee::s12::OVERRIDE_0038D638, ee::s12::f_0038d638__generated;
        oc673_fake_003e18c8, OC673_PREV_003E18C8, 0x003e_18c8, ee::s14::OVERRIDE_003E18C8, ee::s14::f_003e18c8__generated;
        oc673_fake_003e2190, OC673_PREV_003E2190, 0x003e_2190, ee::s14::OVERRIDE_003E2190, ee::s14::f_003e2190__generated;
        oc673_fake_003e6448, OC673_PREV_003E6448, 0x003e_6448, ee::s14::OVERRIDE_003E6448, ee::s14::f_003e6448__generated;
        oc673_fake_0041605c, OC673_PREV_0041605C, 0x0041_605c, ee::s15::OVERRIDE_0041605C, ee::s15::f_0041605c__generated;
        oc673_fake_00416210, OC673_PREV_00416210, 0x0041_6210, ee::s15::OVERRIDE_00416210, ee::s15::f_00416210__generated;
        oc673_fake_004166f4, OC673_PREV_004166F4, 0x0041_66f4, ee::s15::OVERRIDE_004166F4, ee::s15::f_004166f4__generated;
        oc673_fake_00416ce0, OC673_PREV_00416CE0, 0x0041_6ce0, ee::s15::OVERRIDE_00416CE0, ee::s15::f_00416ce0__generated;
        oc673_fake_00417828, OC673_PREV_00417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc673_fake_0041acc0, OC673_PREV_0041ACC0, 0x0041_acc0, ee::s15::OVERRIDE_0041ACC0, ee::s15::f_0041acc0__generated;
    }

    /// A scrambled machine with the fakes on, the result table empty and
    /// `pc` at the function's entry; unwinds at one of `calls` fake calls.
    pub(crate) fn oc673_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc673_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.pc = entry;
        c.bus.write32(OC673_CTRL, OC673_MAGIC);
        for n in 0..32 {
            c.bus.write32(OC673_RESULTS + 4 * n, OC673_NONE);
        }
        r
    }

    /// Force the result of fake call `n`.
    pub(crate) fn oc673_result(c: &mut Cpu, n: u32, v: u32) {
        c.bus.write32(OC673_RESULTS + 4 * n, v);
    }

    /// Force the result of every fake call to the function at `addr`.
    pub(crate) fn oc673_result_at(c: &mut Cpu, addr: u32, v: u32) {
        for k in 0..16 {
            let slot = OC673_BY_ADDR + 8 * k;
            if c.bus.read32(slot) == 0 {
                c.bus.write32(slot, addr);
                c.bus.write32(slot + 4, v);
                return;
            }
        }
        panic!("oc673_result_at: table full");
    }

    /// A 6-bit signed input field: zero a third of the time.
    pub(crate) fn oc673_field(r: &mut Rng) -> u32 {
        match r.next_u32() % 3 {
            0 => 0,
            _ => r.next_u32() & 0x3f,
        }
    }

    /// `RiderRailslideSub_HandleInput` against the generated body: the four
    /// early-out checks (reset path, airborne, prewind, uber grind), the
    /// rail animation categories 0xe / 0xa / other, the board press and rail
    /// spin inputs (zero and nonzero, both signs), the rail-offset clamp, the
    /// animation switch and unwinds in every call. All callees are fakes.
    #[test]
    fn rider_railslide_sub_handle_input_matches() {
        fakes::covered(
            ee::s00::f_00131d30__generated,
            rider_railslide_sub_handle_input,
            300,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x131d30, 14, 0x0013_1d30);
                let me = DATA + 0x5_0000;
                let inp = DATA + 0x5_0100;
                let rider = DATA + 0x5_1000;
                let motion = DATA + 0x5_2000;
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(inp);
                c.bus.write32(me + 8, rider);
                let w0 = r.next_u32();
                c.bus.write32(inp, w0);
                let w1 = (r.next_u32() & !0xfff) | oc673_field(&mut r) | oc673_field(&mut r) << 6;
                c.bus.write32(inp + 4, w1);
                c.bus.write32(rider + 0x784, DATA + 0x5_3000);
                c.bus.write32(rider + 0x77c, motion);
                c.bus
                    .write32(rider + 0x328, [3, 4, 0, 7][(r.next_u32() % 4) as usize]);
                c.bus.write32(motion + 0xc8, r.f32(2.0));
                // The rate and scale constants at gp-0x75a8 .. gp-0x7598.
                for (off, v) in [
                    (0x75a8, 0.032_258_063_5f32),
                    (0x75a4, 0.066_666_67f32),
                    (0x75a0, 0.066_666_67f32),
                    (0x759c, 1.2f32),
                    (0x7598, 0.066_666_67f32),
                ] {
                    c.bus.write32(GAME_GP - off, v.to_bits());
                }
                for n in 0..4 {
                    let v = if r.next_u32() % 6 == 0 { 1 } else { 0 };
                    oc673_result(c, n, v);
                }
                // Call 9: the layer animation category (0xe, 0xa, other).
                let cat = [0xe, 0xe, 0xa, 0xa, 3][(r.next_u32() % 5) as usize];
                oc673_result(c, 9, cat);
                for n in 10..16 {
                    let v = [0, 0, 1, 0xa, 0x12, 0x13, 0x14][(r.next_u32() % 7) as usize];
                    oc673_result(c, n, v);
                }
            },
        );
    }

    /// `Bolts_FindTable2EntriesByFlag` against the generated body: empty and
    /// negative counts, empty ranges, kinds below / equal / above the wanted
    /// one, matching and non-matching flags, the match count at `*out`.
    #[test]
    fn bolts_find_table2_entries_by_flag_matches() {
        check(
            ee::s01::f_0014db40__generated,
            bolts_find_table2_entries_by_flag,
            300,
            &|c, seed| {
                let mut r = Rng(0x1234_5678 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0014_db40;
                let bolts = DATA + 0x6_0000;
                let entries = DATA + 0x6_1000;
                let out = DATA + 0x6_0800;
                let index = r.next_u32() % 4;
                c.g[A0][0] = sx(bolts);
                c.g[A1][0] = sx(index);
                c.g[A2][0] = sx(r.next_u32() % 4);
                c.g[A3][0] = sx(r.next_u32() % 2);
                c.g[T0][0] = sx(out);
                c.bus.write32(out, r.next_u32());
                c.bus.write32(bolts + 8, entries);
                let count = match r.next_u32() % 6 {
                    0 => 0,
                    1 => -2i32 as u32,
                    n => n,
                };
                c.bus.write32(bolts + 0x20c + 4 * index, count);
                let start = r.next_u32() % 3;
                c.bus.write32(bolts + 0x284 + 4 * index, start);
                for i in 0..12 {
                    let e = entries + 12 * i;
                    c.bus.write16(e + 2, (r.next_u32() % 4) as u16);
                    c.bus
                        .write8(e + 1, [0, 1, 1, 0x80, 5][(r.next_u32() % 5) as usize]);
                }
            },
        );
    }

    /// Write a C string at `at`.
    pub(crate) fn oc673_cstr(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// `BELibrary_LoadCharDB` against the generated body: the reward DB load,
    /// the race DB and rider slot clearing, the grab-point record loop, the
    /// character DB load with and without a result (the free is skipped
    /// without), the equipment value table, the three profile resets and the
    /// score constants. Every callee is a fake; unwinds in each call.
    #[test]
    fn belibrary_load_char_db_matches() {
        fakes::covered(
            ee::s01::f_00149bb8__generated,
            belibrary_load_char_db,
            100,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x149bb8, 12, 0x0014_9bb8);
                // The 10 rows of 7 bytes of the equipment value table.
                for i in 0..0x46 {
                    c.bus.write8(0x0044_0f00 + i, r.next_u32() as u8);
                }
                // The slot records at 0x535b20 (0x1c bytes each).
                for i in 0..0x2a {
                    c.bus.write32(0x0053_5b20 + 4 * i, r.next_u32());
                }
                // Call 4 is the character DB load (0: nothing loaded).
                if seed % 3 == 0 {
                    oc673_result(c, 4, 0);
                }
            },
        );
    }

    /// `InputPreproc_Run` against the generated body: a path with and without
    /// a directory part (the strrchr result forced to the last `/` or 0), the
    /// work buffer allocation failing (no free) or not, unwinds in each call.
    #[test]
    fn input_preproc_run_matches() {
        fakes::covered(
            ee::s10::f_00325f48__generated,
            input_preproc_run,
            100,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x325f48, 8, 0x0032_5f48);
                let this = DATA + 0x7_0000;
                let path = DATA + 0x7_1000;
                let size = DATA + 0x7_2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(path);
                c.g[A2][0] = sx(size);
                c.bus.write32(size, r.next_u32());
                if seed % 2 == 0 {
                    oc673_cstr(c, path, "data/dir/file.txt");
                    oc673_result_at(c, 0x0041_acc0, path + 8);
                } else {
                    oc673_cstr(c, path, "file.txt");
                    oc673_result_at(c, 0x0041_acc0, 0);
                }
                if seed % 5 == 0 {
                    // The first operator_new_named fails (the buffer is null).
                    oc673_result(c, 0, 0);
                }
            },
        );
    }

    /// `GameStateReset_Enter` against the generated body: the transport
    /// sequence flag, both race-setup branches (game path set or not, mode 4,
    /// 5, 6), the section id in 0xe..0x10, the NIS list branches, the audio
    /// re-enable, and unwinds in each call. Callees are fakes; the race
    /// interface object's vtable slot is a fake method.
    #[test]
    fn game_state_reset_enter_matches() {
        fakes::covered(
            ee::s06::f_002355c0__generated,
            game_state_reset_enter,
            300,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x2355c0, 26, 0x0023_55c0);
                let this = DATA + 0x8_1000;
                let app = DATA + 0x8_0000;
                let world = DATA + 0x8_2000;
                let x = DATA + 0x8_3000;
                let y = DATA + 0x8_4000;
                let m = DATA + 0x8_5000;
                let n = DATA + 0x8_5800;
                let iface = DATA + 0x8_6000;
                let obj = DATA + 0x8_6800;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x14, r.next_u32() % 3);
                c.bus.write32(GAME_GP - 0x84c, app);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(GAME_GP - 0x488, m);
                c.bus.write32(GAME_GP + 0xce8, r.next_u32());
                c.bus
                    .write32(app + 0x550, [2, 2, 3][(r.next_u32() % 3) as usize]);
                c.bus.write32(world + 0x84, x);
                c.bus.write32(x + 0x78, y);
                c.bus.write32(x + 0x94, r.next_u32());
                c.bus.write32(
                    y + 0x1bc,
                    [0xd, 0xe, 0xf, 0x10, 0x20][(r.next_u32() % 5) as usize],
                );
                c.bus.write32(m + 0xc, n);
                c.bus.write32(m + 0x28, r.next_u32());
                c.bus.write8(0x0053_5bc8 + 0x49, (r.next_u32() % 2) as u8);
                c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 7) as u8);
                c.bus.write32(iface + 0xc, obj);
                c.bus.write16(obj + 8, ((r.next_u32() % 64) * 4) as u16);
                c.bus.write32(obj + 0xc, FAKE);
                oc673_result_at(c, 0x0014_dd58, iface);
                oc673_result_at(c, 0x0027_aa80, r.next_u32() % 2);
                oc673_result_at(c, 0x0014_4cc0, [1, 1, 0, 2][(r.next_u32() % 4) as usize]);
            },
        );
    }

    /// `cGameLoadScreen_LoadPictures` against the generated body: the
    /// two-player branch (two pictures, the loaded file present or missing)
    /// and the single course picture (location id above 16 or not), unwinds
    /// in each call. The graphics manager's vtable slots are fake methods.
    #[test]
    fn c_game_load_screen_load_pictures_matches() {
        fakes::covered(
            ee::s06::f_00233048__generated,
            c_game_load_screen_load_pictures,
            200,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x233048, 17, 0x0023_3048);
                let this = DATA + 0x9_0000;
                let gm = DATA + 0x9_1000;
                let g = DATA + 0x9_2000;
                let file = DATA + 0x9_3000;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP - 0x854, gm);
                c.bus.write32(gm + 0x10d8, g);
                c.bus.write16(g + 0x170, ((r.next_u32() % 0x40) * 4) as u16);
                c.bus.write32(g + 0x174, FAKE);
                c.bus
                    .write8(0x0053_5c11, [2, 2, 3][(r.next_u32() % 3) as usize]);
                let loaded = if r.next_u32() % 4 == 0 { 0 } else { file };
                oc673_result_at(c, 0x003e_2190, loaded);
                c.bus.write32(file + 0x14, (r.next_u32() % 0x100) & !3);
                oc673_result_at(
                    c,
                    0x0014_4be0,
                    [5, 0x12, 0xff, 0x10, 0][(r.next_u32() % 5) as usize],
                );
            },
        );
    }

    /// `SectionMan_OnEventTrigger` against the generated body: skipped (game
    /// path, wrong mode, finale section, event 1) and taken; the AI state 4
    /// check; finale and the five table events and events outside the table;
    /// unwinds in each call. The race object's slot is a fake method.

    /// `BELibrary_ctor` against the generated body: the character DB load
    /// (its callees are fakes), the 0xa28-byte default block copy and the
    /// fourteen interface singleton creations, unwinds in each call.
    #[test]
    fn be_library_ctor_matches() {
        fakes::covered(
            ee::s01::f_00149860__generated,
            be_library_ctor,
            60,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x149860, 28, 0x0014_9860);
                c.g[A0][0] = sx(DATA + 0xc_0000);
                for i in 0..0x28a {
                    c.bus.write32(0x0043_fb28 + 4 * i, r.next_u32());
                }
                if seed % 3 == 0 {
                    oc673_result_at(c, 0x003e_18c8, 0);
                }
            },
        );
    }

    /// `PlaybackMan_ctor` against the generated body: the world and AI
    /// pointers copied in, the two allocations and the `cInput` setup, the
    /// formatted file name and the big file open; unwinds in each call.
    #[test]
    fn playback_man_ctor_matches() {
        fakes::covered(
            ee::s07::f_0026c458__generated,
            playback_man_ctor,
            100,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x26c458, 9, 0x0026_c458);
                let this = DATA + 0xb_0000;
                let world = DATA + 0xb_1000;
                let x = DATA + 0xb_2000;
                let y = DATA + 0xb_3000;
                let z = DATA + 0xb_4000;
                let p = DATA + 0xb_5000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, x);
                c.bus.write32(x + 0xc, y);
                c.bus.write32(y + 0x40, z);
                c.bus.write32(z + 0xdf0, p);
                c.bus.write32(p, r.next_u32());
                c.bus.write32(p + 4, r.next_u32());
                c.bus.write32(GAME_GP + 0x328, r.next_u32());
                c.bus.write32(GAME_GP + 0x320, r.next_u32());
            },
        );
    }

    /// `SSXApp_Init` against the generated body: the boot sequence with the
    /// graphics manager (a fixture whose vtable slots are fake methods), the
    /// pad queue, light and bezier managers (the real, bounded constructors
    /// run), the pad records, the command line, the boot shape and tint, and
    /// unwinds in each call.
    #[test]
    fn ssx_app_init_matches() {
        fakes::covered(
            ee::s05::f_00226900__generated,
            ssx_app_init,
            40,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x226900, 50, 0x0022_6900);
                // Above the reach of the real constructors' writes through the fake
                // allocator's pointers (DATA .. DATA + 0xe8000).
                let this = DATA + 0xf_2000;
                let gm = DATA + 0xf_0000;
                let g = DATA + 0xf_1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(DATA + 0xf_3000);
                c.bus.write32(gm + 0x10d8, g);
                for off in [0x14, 0x164, 0x174] {
                    c.bus.write32(g + off, FAKE);
                }
                for off in [0x10, 0x160, 0x170] {
                    c.bus.write16(g + off, ((r.next_u32() % 64) * 4) as u16);
                }
                oc673_result_at(c, 0x0037_5a08, gm);
            },
        );
    }
}
