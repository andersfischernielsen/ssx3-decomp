//! Staging module oc776 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc776`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_e960,
        fe_module_rider_preview_load_char,
        "FEModule_RiderPreviewLoadChar",
    ),
    (
        0x0037_39d0,
        bezier_patch_cache_reset,
        "BezierPatchCache_Reset",
    ),
    (0x0039_bed8, ui_pair_handle_input, "UIPair_HandleInput"),
    (
        0x0020_3a58,
        event_select_update_confirm_popup,
        "EventSelect_UpdateConfirmPopup",
    ),
    (
        0x0019_2488,
        cfe_state_rule_select_update_row_visibility,
        "cFEStateRuleSelect_UpdateRowVisibility",
    ),
    (
        0x0028_f140,
        ssx_audio_system_set_fe_music_screen,
        "SSXAudioSystem_SetFEMusicScreen",
    ),
];

/// `void SSXAudioSystem_SetFEMusicScreen(SSXAudioSystem* sys, int screen)` (0x28f140).
///
/// Remembers the front-end screen (`+0x6290`) and, when it changed, maps it
/// through a jump table (0x482a70, screens 0..=9) to a music bank (screen 0
/// picks bank 9 or 0 by the word at `+0x534`) and tells the music system
/// (`sys + 0x118`, vtable at `+0x5558`, slot `+0x20`) to switch to it.
pub fn ssx_audio_system_set_fe_music_screen(c: &mut Cpu) {
    if c.pc != 0x0028_f140 {
        return ee::s07::f_0028f140__entries(c);
    }
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(FE_MUSIC_SCREEN)).lw(c);
        let same = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[A1][0] < 0xa) as u64;
        if same {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.sw(
            (c.g[A2][0] as u32).wrapping_add(FE_MUSIC_SCREEN),
            c.g[A1][0],
        );
        if in_range {
            c.cycles += 7;
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2a70));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let target = c.g[A0][0] as u32;
            // Each case of the jump table: the bank it selects.
            let bank: u64 = match target {
                0x0028_f17c => {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x534)).lw(c);
                    c.g[A1][0] = 0x9;
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    }
                    c.g[A1][0]
                }
                0x0028_f190 => {
                    c.cycles += 2;
                    1
                }
                0x0028_f198 => {
                    c.cycles += 2;
                    3
                }
                0x0028_f1a0 => {
                    c.cycles += 2;
                    4
                }
                0x0028_f1a8 => {
                    c.cycles += 2;
                    5
                }
                0x0028_f1b0 => {
                    c.cycles += 2;
                    6
                }
                0x0028_f1b8 => {
                    c.cycles += 2;
                    7
                }
                0x0028_f1c0 => {
                    c.cycles += 2;
                    8
                }
                0x0028_f1c8 => {
                    c.cycles += 2;
                    2
                }
                _ => {
                    c.jump_out(target, 0x0028_f140, 0x0028_f200);
                    return Ok(());
                }
            };
            c.g[A1][0] = bank;
        } else {
            c.cycles += 1;
            c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 2;
        let no_bank = (c.g[A1][0] as i64) < 0;
        frame.restore_ra(c, 0);
        if !no_bank {
            c.cycles += 6;
            c.g[V1][0] = Addr((c.g[A2][0] as u32).wrapping_add(FE_MUSIC_SYS_VTABLE)).lw(c);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x118));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
            let method = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, method, 0x0028_f1f4)?;
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Row object (`cUIText`) virtual call: `obj->vt[off](obj + delta, a1)` with
/// `a1` already set by the caller. `obj` is a register index (s0 or s1).
fn rule_row_vcall(c: &mut Cpu, obj: usize, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[obj][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// Copies the 16-byte static colour (0x4c66c8) into `obj + 0x88`, with the
/// original's unaligned loads in registers `lo` and `lo + 1`.
fn rule_row_copy_colour(c: &mut Cpu, src_reg: usize, lo: usize, obj: usize) {
    c.ldl(lo, (c.g[src_reg][0] as u32).wrapping_add(0x7));
    c.ldr(lo, c.g[src_reg][0] as u32);
    c.ldl(lo + 1, (c.g[src_reg][0] as u32).wrapping_add(0xf));
    c.ldr(lo + 1, (c.g[src_reg][0] as u32).wrapping_add(0x8));
    c.sdl(lo, (c.g[obj][0] as u32).wrapping_add(0x8f));
    c.sdr(lo, (c.g[obj][0] as u32).wrapping_add(0x88));
    c.sdl(lo + 1, (c.g[obj][0] as u32).wrapping_add(0x97));
    c.sdr(lo + 1, (c.g[obj][0] as u32).wrapping_add(0x90));
}

/// `void cFEStateRuleSelect::UpdateRowVisibility(cFEStateRuleSelect* this)` (0x192488).
///
/// For each of the first `min(rowCount (+0x50), 11)` rows it looks up the
/// screen objects named "0b".."9b", "ab".. (list box) and "0t".."9t", "at"..
/// (text box) by hash (`this->screen (+0x40)`). When both exist: if the
/// row's flag byte (`this + rowData(+0x18) + 0x51`) is positive both get the
/// static colour (0x4c66c8) and are shown, otherwise they are hidden. The
/// names advance one character per row (digits, then `a`..).
pub fn cfe_state_rule_select_update_row_visibility(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.g[V0][0] = 0x4a0000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S3][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A0][0] = 0x4a0000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(RULE_ROW_COUNT));
        // name0 = "0b" (0x4a16a8), name1 = "0t" (0x4a16b0): 3 bytes each.
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
        c.g[A1][0] = c.lb(c.g[T0][0] as u32);
        c.g[A2][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x2));
        c.sb(c.g[SP][0] as u32, c.g[A1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[A3][0]);
        c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(RULE_ROW_COUNT));
        c.g[V1][0] = ((c.g[V1][0] as i64) < 12i64) as u64;
        c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x16b0));
        c.g[A1][0] = c.lb(c.g[T0][0] as u32);
        c.g[A2][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x1));
        c.g[A3][0] = c.lb((c.g[T0][0] as u32).wrapping_add(0x2));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A3][0]);
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0xb;
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[S4][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        let no_rows = (c.g[S4][0] as i64) <= 0;
        c.g[S2][0] = 0;
        if !no_rows {
            c.cycles += 3;
            c.g[S6][0] = 0x4c0000;
            c.g[S5][0] = 0x1000000;
            loop {
                c.cycles += 13;
                // Both objects by name hash.
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0031_7670, 0x0019_2528)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RULE_SCREEN));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0019_2534)?;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0031_7670, 0x0019_2540)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(RULE_SCREEN));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0019_254c)?;
                let no_list_box = c.g[S1][0] == 0;
                c.g[S0][0] = c.g[V0][0];
                if no_list_box {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                } else {
                    c.cycles += 2;
                    let no_text_box = c.g[S0][0] == 0;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                    if !no_text_box {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(RULE_ROW_DATA_OFF));
                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x51));
                        let hidden = (c.g[V1][0] as i64) <= 0;
                        c.g[A1][0] = 0x1;
                        if !hidden {
                            // Show both rows in the static colour.
                            c.cycles += 43;
                            rule_row_vcall(c, S0, 0xc8, 0x0019_2584)?;
                            c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x66c8));
                            rule_row_copy_colour(c, A1, 2, S0);
                            c.g[A1][0] = 0x1;
                            rule_row_vcall(c, S0, 0x40, 0x0019_25c0)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc0));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc4));
                            let method = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, method, 0x0019_25d8)?;
                            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x66c8));
                            rule_row_copy_colour(c, V0, 6, S1);
                            c.g[A1][0] = 0x1;
                            rule_row_vcall(c, S1, 0x40, 0x0019_2614)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                        } else {
                            // Hide both rows.
                            c.cycles += 24;
                            c.g[A1][0] = 0;
                            rule_row_vcall(c, S0, 0xc8, 0x0019_2634)?;
                            c.g[A1][0] = 0;
                            rule_row_vcall(c, S0, 0x40, 0x0019_264c)?;
                            c.g[A1][0] = 0;
                            // (the original loads v0 = [s1+8] before a1 = 0: same result)
                            rule_row_vcall(c, S1, 0xc0, 0x0019_2664)?;
                            c.g[A1][0] = 0;
                            rule_row_vcall(c, S1, 0x40, 0x0019_267c)?;
                            c.cycles += 1;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
                        }
                    }
                }
                // Advance the names: one character per row, then 'a'.. .
                c.cycles += 2;
                let past_digits = c.g[V0][0] == 0;
                c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
                if !past_digits {
                    c.cycles += 6;
                    c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                }
                c.cycles += 6;
                c.g[V0][0] = c.g[S5][0];
                c.g[V1][0] = 0x1000000;
                c.g[S2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_2520);
                    }
                    continue;
                }
                break;
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = ra;
        Ok(())
    })
}

/// `StrHash(name)` then `UIScreen_FindObject(screen, hash)` of the map UI
/// (`this` in s1, screen at `+0x2e8`); the object comes back in `v0`.
fn popup_find_by_name(c: &mut Cpu, name: u32, hash_ret: u32, find_ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, find_ret)
}

/// `cUIText_setUnicodeStringByID(obj (s2 or s0), StrHash(name))`: the hash
/// call, then the string set (0x3a1000) on the object in register `obj`.
fn popup_set_text(c: &mut Cpu, obj: usize, name: u32, hash_ret: u32, set_ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, 0x0031_7670, hash_ret)?;
    c.g[A0][0] = c.g[obj][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x003a_1000, set_ret)
}

/// Virtual call `obj->vt[off](obj + delta, a1)` on the object in s0 (`a1`
/// set by the caller).
fn popup_s0_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// `void EventSelect_UpdateConfirmPopup(cUITemplate_MAP* this)` (0x203a58)
/// (reference: `cUITemplate_MAP_setupPopup`).
///
/// Shows or hides the map screen's transport confirm popup, by the popup flag
/// at `+0x2cc`. Active: show "ConfirmPopup", select item 0 of "ConfirmMenu",
/// and (if still active) swap "Popup1"/"Popup2"/"maptab" and set the
/// "P2_TransportNow" text by the map state (gp-0xb54); then label the Yes
/// button and, by the popup type (`+0x2d0`), show or hide the No button.
/// Inactive: hide "ConfirmPopup", show "maptab".
pub fn event_select_update_confirm_popup(c: &mut Cpu) {
    if c.pc != 0x0020_3a58 {
        return ee::s05::f_00203a58__entries(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_ACTIVE));
        let inactive = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
        if inactive {
            c.cycles += 8;
            c.g[A1][0] = sx(0x0047_0a60);
            call_ptr(c, 0x0039_e8b8, 0x0020_3c8c)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
            c.g[A1][0] = sx(0x004a_25b0);
            call_ptr(c, 0x0039_e948, 0x0020_3c9c)?;
            frame.restore(c, 16, 0x40);
        } else {
            c.cycles += 11;
            c.g[A1][0] = sx(0x0047_0a60);
            call_ptr(c, 0x0039_e948, 0x0020_3a8c)?;
            popup_find_by_name(c, 0x0047_0a20, 0x0020_3a98, 0x0020_3aa4)?;
            let no_menu = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !no_menu {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_b960, 0x0020_3ab4)?;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_ACTIVE));
            let still_active = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
            if still_active {
                c.cycles += 21;
                c.g[A1][0] = sx(0x004a_2638);
                call_ptr(c, 0x0039_e8b8, 0x0020_3acc)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = sx(0x004a_2640);
                call_ptr(c, 0x0039_e948, 0x0020_3adc)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = sx(0x004a_25b0);
                call_ptr(c, 0x0039_e8b8, 0x0020_3aec)?;
                popup_find_by_name(c, 0x0047_0a70, 0x0020_3af8, 0x0020_3b04)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                let map_state_set = c.g[V0][0] != 0;
                c.cycles += 2;
                if c.g[S0][0] != 0 {
                    // Map state 0: "kT_OVRCMNGoPeakNow", else "kT_MAPTransArea".
                    if map_state_set {
                        c.cycles += 7;
                        popup_set_text(c, S0, 0x0047_0a98, 0x0020_3b50, 0x0020_3b5c)?;
                    } else {
                        c.cycles += 8;
                        popup_set_text(c, S0, 0x0047_0a80, 0x0020_3b28, 0x0020_3b34)?;
                    }
                }
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
            }
            // The Yes/No buttons of the confirm menu.
            c.cycles += 22;
            c.g[A1][0] = sx(0x0047_0700);
            call_ptr(c, 0x0039_e8b8, 0x0020_3b6c)?;
            popup_find_by_name(c, 0x0047_06b8, 0x0020_3b78, 0x0020_3b84)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 0x470000;
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c8));
            let no_button_name = c.g[S3][0] as u32;
            popup_find_by_name(c, no_button_name, 0x0020_3b98, 0x0020_3ba4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_TYPE));
            c.g[V0][0] = 0x1;
            let not_ok_only = c.g[V1][0] != c.g[V0][0];
            if !not_ok_only {
                // Type 1: "OK" label, hide and freeze the No button.
                c.cycles += 2;
                let no_title = c.g[S2][0] == 0;
                c.g[A0][0] = 0x470000;
                if !no_title {
                    c.cycles += 5;
                    popup_set_text(c, S2, 0x0046_faa0, 0x0020_3bc8, 0x0020_3bd4)?;
                }
                c.cycles += 2;
                let no_no_button = c.g[S0][0] == 0;
                c.g[A1][0] = 0;
                if !no_no_button {
                    c.cycles += 11;
                    popup_s0_vcall(c, 0x48, 0x0020_3bf0)?;
                    c.g[A1][0] = 0x1;
                    popup_s0_vcall(c, 0x40, 0x0020_3c08)?;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039_e8b8, 0x0020_3c14)?;
            } else {
                // Other types: "Yes" label, show the No button.
                c.cycles += 2;
                let no_title = c.g[S2][0] == 0;
                c.g[A0][0] = 0x470000;
                if !no_title {
                    c.cycles += 5;
                    popup_set_text(c, S2, 0x0047_0aa8, 0x0020_3c2c, 0x0020_3c38)?;
                }
                c.cycles += 2;
                let no_no_button = c.g[S0][0] == 0;
                c.g[A1][0] = 0x1;
                if !no_no_button {
                    c.cycles += 11;
                    popup_s0_vcall(c, 0x48, 0x0020_3c54)?;
                    c.g[A1][0] = 0;
                    popup_s0_vcall(c, 0x40, 0x0020_3c6c)?;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(POPUP_STATE));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039_e948, 0x0020_3c78)?;
            }
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const PAIR_GET_STATE_DATA: Callee = Callee {
    addr: 0x0039_ff50,
    f: ee::s13::f_0039ff50,
};
const PAIR_GET_EVENT_THREAD: Callee = Callee {
    addr: 0x0039_ff78,
    f: ee::s13::f_0039ff78,
};
const PAIR_START_THREAD: Callee = Callee {
    addr: 0x0039_c870,
    f: ee::s13::f_0039c870,
};
const PAIR_GET_USER_DATA: Callee = Callee {
    addr: 0x003a_0318,
    f: ee::s13::f_003a0318,
};

/// `in->vt[off](in + delta, a1 = pair)` on the input in s0 (a1 set by the
/// caller); `ret` is the return address of the call.
fn pair_input_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(off));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(off + 4));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// The focus-controller object (`pair->screen (+0x5c) + 0xd0`) virtual call
/// `vt[off](obj + delta, pair, a2)` with the generated's register order:
/// `a2` is set by the caller; `a1 = pair` (s1) stays.
fn pair_focus_vcall(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(off + 4));
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, ret)
}

/// A direction case of `UIPair_HandleInput` (previous 5 / next 6): notify the
/// focus controller (`a2 = dir`), move the focus (`a2 = dir - 5`), play the
/// keyframer event (`a1 = event slot`) if the move changed focus (0x100),
/// then fetch the move's sound (`GetUserData`, 1 if bit 0, 0 if bit 4) and
/// tell the sound listener (`s2`) about it. Leaves `v0 = 1`.
fn pair_direction_case(c: &mut Cpu, dir: u64, rets: [u32; 5]) -> Flow {
    c.cycles += 21;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
    c.g[A2][0] = dir;
    pair_focus_vcall(c, 0x98, rets[0])?;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = dir - 5;
    // (the original loads a3 from v0 here; `pair_focus_vcall` reads v1)
    c.g[V1][0] = c.g[V0][0];
    c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, rets[1])?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = c.g[S0][0] & 0x100;
    let unchanged = c.g[V0][0] == 0;
    c.g[A1][0] = dir - 3;
    let mut skip_plus1 = false;
    if !unchanged {
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        PAIR_GET_EVENT_THREAD.call(c, rets[2])?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0xffff;
        let no_event = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0] & 0x1;
        if no_event {
            skip_plus1 = true;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
            c.g[A2][0] = 0x1;
            PAIR_START_THREAD.call(c, rets[3])?;
        }
    }
    if !skip_plus1 {
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0] & 0x1;
    }
    // Which sound: bit 0 -> 1, else bit 4 -> 0, else none.
    c.cycles += 2;
    let bit0_clear = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S1][0];
    let mut fetch = false;
    if !bit0_clear {
        c.cycles += 2;
        c.g[A1][0] = 0x1;
        fetch = true;
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.g[S0][0] & 0x10;
        let bit4_clear = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S1][0];
        if !bit4_clear {
            c.cycles += 1;
            c.g[A1][0] = 0;
            fetch = true;
        }
    }
    if fetch {
        c.cycles += 3;
        PAIR_GET_USER_DATA.call(c, 0x0039_c074)?;
        c.g[S3][0] = c.g[V0][0];
    }
    // The listener's vt[2] (`+0x10`) with the sound.
    c.cycles += 2;
    let no_listener = c.g[S2][0] == 0;
    c.g[A1][0] = c.g[S3][0];
    if no_listener {
        c.cycles += 1;
        c.g[V0][0] = 0x1;
    } else {
        c.cycles += 7;
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, rets[4])?;
        c.g[V0][0] = 0x1;
    }
    Ok(())
}

/// `bool UIPair_HandleInput(UIPair* this, const UIInput* in)` (0x39bed8)
/// (reference: `cUIPair::onInput`).
///
/// A pair of widgets with one focus. Previous (input 5) and next (input 6)
/// move the focus ([`pair_direction_case`]); input 4 only notifies the focus
/// controller; anything else goes to the child widget (`+0x7c`), if any.
/// Returns 1 when the pair handled the input.
pub fn ui_pair_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
        frame.save_ra(c, 0);
        c.g[S3][0] = 0;
        PAIR_GET_STATE_DATA.call(c, 0x0039_bf04)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        if c.g[A1][0] != 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0039_bf20)?;
        let is_prev = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[S1][0];
        let handled_dir = if is_prev {
            pair_direction_case(
                c,
                5,
                [
                    0x0039_bf48,
                    0x0039_bf6c,
                    0x0039_bf84,
                    0x0039_bfa0,
                    0x0039_c094,
                ],
            )?;
            true
        } else {
            c.cycles += 7;
            pair_input_vcall(c, 0x30, 0x0039_bfc8)?;
            let is_next = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if is_next {
                pair_direction_case(
                    c,
                    6,
                    [
                        0x0039_bff0,
                        0x0039_c014,
                        0x0039_c02c,
                        0x0039_c048,
                        0x0039_c094,
                    ],
                )?;
                true
            } else {
                false
            }
        };
        if handled_dir {
            c.cycles += 1;
        } else {
            c.cycles += 7;
            pair_input_vcall(c, 0x20, 0x0039_c0b0)?;
            let is_focus = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            if is_focus {
                c.cycles += 10;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                c.g[A2][0] = 0x4;
                pair_focus_vcall(c, 0x98, 0x0039_c0d8)?;
                c.g[V0][0] = 0x1;
                c.cycles += 1;
            } else {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                let no_child = c.g[A2][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if no_child {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.cycles += 1;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x78));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0039_c100)?;
                    frame.restore(c, 16, 0x40);
                    return pair_epilogue(c, frame);
                }
            }
        }
        frame.restore(c, 16, 0x40);
        pair_epilogue(c, frame)
    })
}

/// The shared epilogue of `ui_pair_handle_input` (after `lq s0`).
fn pair_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// One free-list fill loop of `bezier_patch_cache_reset`: pushes the indices
/// `a1..count` (a1 starts at 0) onto the list at `list` (`cache + list_off`),
/// counting in the word at `cache + counter_off`.
fn bezier_fill_free_list(c: &mut Cpu, head: u32, count: usize, counter_off: u32, list_off: u32) {
    loop {
        c.cycles += 10;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(counter_off));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(list_off));
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[count][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(counter_off), c.g[V1][0]);
        if more {
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        break;
    }
}

/// `void BezierPatchCache_Reset(BezierPatchCache* cache)` (0x3739d0).
///
/// Three pool sizes (4x4, 6x6, 8x8). Zeroes the 12-byte elements of the
/// three arrays at `+0xc..` (counts `+0..+8`) with `MEM_fill`; rebuilds the
/// three free lists of element indices (`+0x30..`, fill counters `+0x3c..`);
/// resets every slot of the arrays at `+0x18..` (`f0 = 0`, `f4 = 0`,
/// `f8 = -1`; counts again `+0..+8`); and rebuilds the second three free
/// lists (counts `+0x1a8..`, lists `+0x1cc..`, counters `+0x1d8..`).
pub fn bezier_patch_cache_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = 0xc;
        frame.save(c, 19, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, 18, 0x30);
        c.g[S2][0] = c.lw(c.g[S1][0] as u32);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.mult(0, 0, 18, 16);
        c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A2][0] = c.lo[0];
        c.g[S4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x003e_6448, 0x0037_3a14)?;
        c.mult(0, 0, 19, 16);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = c.lo[0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_6448, 0x0037_3a28)?;
        c.mult(0, 0, 20, 16);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = c.lo[0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e_6448, 0x0037_3a3c)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), 0);
        c.g[A1][0] = 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), 0);
        let none = (c.g[S2][0] as i64) <= 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x44), 0);
        if !none {
            bezier_fill_free_list(c, 0x0037_3a50, S2, 0x3c, 0x30);
        }
        c.cycles += 2;
        let none = (c.g[S3][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            bezier_fill_free_list(c, 0x0037_3a80, S3, 0x40, 0x34);
        }
        c.cycles += 2;
        let none = (c.g[S4][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            bezier_fill_free_list(c, 0x0037_3ab0, S4, 0x44, 0x38);
        }
        // Reset the slots of the three pools: f8 = -1, f4 = 0, f0 = 0.
        c.cycles += 6;
        c.g[T4][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a8));
        c.g[V1][0] = 0;
        c.g[T5][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1ac));
        c.g[T6][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b0));
        c.g[A2][0] = sx((c.g[V1][0] as u32) << 2);
        loop {
            c.cycles += 5;
            c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let empty = (c.g[V0][0] as i64) <= 0;
            c.g[T0][0] = 0;
            if !empty {
                c.cycles += 5;
                c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
                c.g[T3][0] = 0xffff_ffff_ffff_ffff;
                c.g[A3][0] = 0;
                c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                loop {
                    c.cycles += 16;
                    c.g[A1][0] = c.g[A2][0];
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0);
                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw(c.g[V1][0] as u32, 0);
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0037_3b18);
                        }
                        continue;
                    }
                    break;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[T2][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
            if c.g[V0][0] != 0 {
                c.g[A2][0] = sx((c.g[V1][0] as u32) << 2);
                if event_due(c) {
                    deliver_event_here(c, 0x0037_3af0);
                }
                continue;
            }
            break;
        }
        // Free lists of the slot pools.
        c.cycles += 5;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d8), 0);
        c.g[A1][0] = 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1dc), 0);
        let none = (c.g[T4][0] as i64) <= 0;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e0), 0);
        if !none {
            c.cycles += 1;
            bezier_fill_free_list(c, 0x0037_3b80, T4, 0x1d8, 0x1cc);
        }
        c.cycles += 2;
        let none = (c.g[T5][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            bezier_fill_free_list(c, 0x0037_3bb0, T5, 0x1dc, 0x1d0);
        }
        c.cycles += 2;
        let none = (c.g[T6][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            bezier_fill_free_list(c, 0x0037_3be0, T6, 0x1e0, 0x1d4);
        }
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

const PREVIEW_UNLOAD: Callee = Callee {
    addr: 0x0019_eba0,
    f: ee::s03::f_0019eba0,
};
const PREVIEW_REQUEST_BOLT_MODELS: Callee = Callee {
    addr: 0x0019_ff00,
    f: ee::s03::f_0019ff00,
};
const PREVIEW_REQUEST_EXTRA_TEXTURES: Callee = Callee {
    addr: 0x0019_e7f0,
    f: ee::s03::f_0019e7f0,
};
const PREVIEW_REQUEST_BOLT_TEXTURES: Callee = Callee {
    addr: 0x0019_fbe0,
    f: ee::s03::f_0019fbe0,
};
const PREVIEW_UPDATE: Callee = Callee {
    addr: 0x0019_f138,
    f: ee::s03::f_0019f138,
};

/// `int FEModule_RiderPreviewLoadChar(Preview* this, int profile, int character, int slot)`
/// (0x19e960) (reference: `cFERider_init`).
///
/// Loads the front-end rider preview for `character` in outfit `slot`
/// (`profile` is the rider profile). Returns 1 at once when that character
/// and slot are already loaded; otherwise unloads the current one (if any),
/// resets the preview fields (character `+0`, profile `+4`, slot `+0x10`,
/// the dirty flag `+0xcc4`, ...), builds its effect objects and anim model,
/// requests the models and textures (the extra textures for slots other than
/// 0), updates the preview, and records the character's bit mask (`+0xcd8`,
/// from the jump table at 0x460b00 for characters 0..=9). Returns 1.
pub fn fe_module_rider_preview_load_char(c: &mut Cpu) {
    if c.pc != 0x0019_e960 {
        return ee::s03::f_0019e960__entries(c);
    }
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0019_e990)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x0014_dd58, 0x0019_e99c)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        if c.g[V0][0] == c.g[S2][0] {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT));
            let same_slot = c.g[V0][0] == c.g[S1][0];
            c.g[V0][0] = 0x1;
            if same_slot {
                // Already loaded.
                return preview_epilogue(c, frame);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        }
        c.cycles += 2;
        if (c.g[V0][0] as i64) < 0 {
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            PREVIEW_UNLOAD.call(c, 0x0019_e9cc)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc4));
        }
        // Reset the preview fields (the dirty flag keeps its two sources).
        c.cycles += 68;
        c.g[T0][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xcc8));
        c.g[A1][0] = 0x460000;
        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT), c.g[S1][0]);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S3][0]);
        c.g[V1][0] |= c.g[V0][0];
        c.sw(c.g[S0][0] as u32, c.g[S2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc28), 0);
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
        c.g[A3][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc8), 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xac0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb4), 0);
        c.g[A0][0] = 0xa8;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcb8), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcbc), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc0), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc24), c.g[T0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xcc4), c.g[V1][0]);
        // "AttachedEmitterFX" (168 bytes) and its constructor.
        call_ptr(c, 0x0031_7d70, 0x0019_ea2c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002f_1890, 0x0019_ea38)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xad8));
        c.g[A0][0] = 0xc8;
        c.g[A2][0] = 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc70), c.g[V0][0]);
        // "cBoltOnFXInterface" (200 bytes).
        call_ptr(c, 0x0031_7d70, 0x0019_ea54)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_ca68, 0x0019_ea64)?;
        c.g[A1][0] = 0x460000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xaf0));
        c.g[A2][0] = 0;
        c.g[A0][0] = 0x160;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc74), c.g[V0][0]);
        // "AnimModel" (352 bytes), constructor and init(100, 15).
        call_ptr(c, 0x0031_7d70, 0x0019_ea80)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0030_d4b8, 0x0019_ea88)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = 0xf;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x64;
        call_ptr(c, 0x0030_d840, 0x0019_ea9c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc70));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x002f_19f0, 0x0019_eaa8)?;
        // The rider's model list, then request models and textures.
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0014_ad28, 0x0019_eab8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0;
        c.g[T0][0] = 0x1;
        PREVIEW_REQUEST_BOLT_MODELS.call(c, 0x0019_ead4)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PREVIEW_SLOT));
        let slot_zero = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if !slot_zero {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            PREVIEW_REQUEST_EXTRA_TEXTURES.call(c, 0x0019_eae8)?;
            c.g[A1][0] = c.g[S1][0];
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        c.g[A3][0] = 0x1;
        PREVIEW_REQUEST_BOLT_TEXTURES.call(c, 0x0019_eafc)?;
        c.g[A0][0] = c.g[S0][0];
        PREVIEW_UPDATE.call(c, 0x0019_eb04)?;
        // The character's bit mask, by the jump table at 0x460b00.
        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
        c.g[V0][0] = (c.g[V1][0] < 0xa) as u64;
        let unknown = c.g[V0][0] == 0;
        c.g[V0][0] = 0x460000;
        if !unknown {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xb00));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let target = c.g[A0][0] as u32;
            // (mask, cycles) of each case of the jump table.
            let (mask, cycles) = match target {
                0x0019_eb2c => (0x1, 2),
                0x0019_eb34 => (0x4, 2),
                0x0019_eb3c => (0x20, 2),
                0x0019_eb44 => (0x2, 2),
                0x0019_eb4c => (0x8, 2),
                0x0019_eb54 => (0x100, 2),
                0x0019_eb5c => (0x200, 2),
                0x0019_eb64 => (0x80, 2),
                0x0019_eb6c => (0x10, 2),
                0x0019_eb74 => (0x40, 1),
                _ => {
                    c.jump_out(target, 0x0019_e960, 0x0019_eba0);
                    return Ok(());
                }
            };
            c.cycles += cycles;
            c.g[V0][0] = mask;
            c.cycles += 1;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xcd8), c.g[V0][0]);
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        preview_epilogue(c, frame)
    })
}

/// `FEModule_RiderPreviewLoadChar` fields.
pub(crate) const PREVIEW_SLOT: u32 = 0x10;

/// The epilogue of `fe_module_rider_preview_load_char`.
fn preview_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cUITemplate_MAP` fields of `event_select_update_confirm_popup`.
pub(crate) const POPUP_ACTIVE: u32 = 0x2cc;
pub(crate) const POPUP_TYPE: u32 = 0x2d0;
pub(crate) const POPUP_SCREEN: u32 = 0x2e8;
pub(crate) const POPUP_STATE: u32 = 0x2ec;

/// `cFEStateRuleSelect` fields of `cfe_state_rule_select_update_row_visibility`.
pub(crate) const RULE_ROW_COUNT: u32 = 0x50;
pub(crate) const RULE_SCREEN: u32 = 0x40;
pub(crate) const RULE_ROW_DATA_OFF: u32 = 0x18;

/// `SSXAudioSystem` fields of `ssx_audio_system_set_fe_music_screen`.
pub(crate) const FE_MUSIC_SCREEN: u32 = 0x6290;
pub(crate) const FE_MUSIC_SYS_VTABLE: u32 = 0x5558;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, put_words, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// `SSXAudioSystem_SetFEMusicScreen`: screens -1..=11 (out of range, all
    /// ten table cases), unchanged screen, music-system flag word, unwind in
    /// the bank call, and a corrupt table entry (jump out).
    #[test]
    fn ssx_audio_system_set_fe_music_screen_matches() {
        let table: [u32; 10] = [
            0x28f17c, 0x28f190, 0x28f1c8, 0x28f198, 0x28f1a0, 0x28f1a8, 0x28f1b0, 0x28f1b8,
            0x28f1c0, 0x28f1c8,
        ];
        fakes::covered(
            ee::s07::f_0028f140__generated,
            ssx_audio_system_set_fe_music_screen,
            420,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7761, 1);
                c.pc = 0x0028_f140;
                put_words(c, 0x0048_2a70, &table);
                let sys = DATA + 0x1_0000;
                let vt = DATA + 0x3_0000;
                c.g[A0][0] = sx(sys);
                let screen = (seed % 13).wrapping_sub(1);
                c.g[A1][0] = sx(screen);
                let old = if (seed / 13) % 4 == 0 {
                    screen
                } else {
                    r.next_u32()
                };
                c.bus.write32(sys + FE_MUSIC_SCREEN, old);
                let flag = if (seed / 13) % 2 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32()
                };
                c.bus.write32(sys + 0x534, flag);
                c.bus.write32(sys + FE_MUSIC_SYS_VTABLE, vt);
                c.bus
                    .write16(vt + 0x20, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x24, fakes::FAKE);
                if seed % 37 == 0 && screen < 10 {
                    c.bus.write32(0x0048_2a70 + 4 * screen, 0x0028_f150);
                }
            },
        );
    }

    pub(crate) const OC776_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC776_MAGIC: u32 = 0x0776_f4ce;
    pub(crate) const OC776_THIS: u32 = DATA + 0x1_0000;
    pub(crate) const OC776_ROWS: u32 = DATA + 0x2_0000;
    pub(crate) const OC776_VT: u32 = DATA + 0x4_0000;

    static OC776_PREV_HASH: AtomicUsize = AtomicUsize::new(0);
    static OC776_PREV_FIND: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `StrHash` (0x317670): the hash is the first two
    /// characters of the name (logged like any fake call).
    fn oc776_fake_hash(c: &mut Cpu) {
        if c.bus.read32(OC776_CTRL) != OC776_MAGIC {
            return fakes::fall_back(c, &OC776_PREV_HASH, ee::s10::f_00317670__generated);
        }
        let ra = c.g[RA][0] as u32;
        let name = c.g[A0][0] as u32;
        let hash = c.bus.read8(name) as u32 | (c.bus.read8(name + 1) as u32) << 8;
        fakes::fake_call(c, 0x0031_7670);
        if c.pc == ra {
            c.g[V0][0] = hash as u64;
        }
    }

    /// Gated fake of `UIScreen_FindObject` (0x39d860): the object of a row
    /// name hash ("0b".."9b", "ab", "bb" and the "t" twins), or null when the
    /// fixture's absence mask (`CTRL + 4`) has its bit.
    fn oc776_fake_find(c: &mut Cpu) {
        if c.bus.read32(OC776_CTRL) != OC776_MAGIC {
            return fakes::fall_back(c, &OC776_PREV_FIND, ee::s13::f_0039d860__generated);
        }
        let ra = c.g[RA][0] as u32;
        let hash = c.g[A1][0] as u32;
        let mask = c.bus.read32(OC776_CTRL + 4);
        let closes_popup = c.bus.read32(OC776_CTRL + 8) != 0 && hash & 0xff == b'1' as u32;
        fakes::fake_call(c, 0x0039_d860);
        if c.pc == ra {
            if closes_popup {
                c.bus.write32(OC776_THIS + POPUP_ACTIVE, 0);
            }
            let (c0, c1) = (hash & 0xff, (hash >> 8) & 0xff);
            let row = if c0 < 0x40 {
                c0.wrapping_sub(0x30)
            } else {
                c0.wrapping_sub(0x61) + 9
            };
            let n = row.wrapping_mul(2).wrapping_add((c1 == b't' as u32) as u32);
            c.g[V0][0] = if n >= 32 || mask >> n & 1 != 0 {
                0
            } else {
                (OC776_ROWS + n * 0x200) as u64
            };
        }
    }

    fn oc776_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc776_fake_hash,
            &OC776_PREV_HASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc776_fake_find,
            &OC776_PREV_FIND,
        );
    }

    /// `cFEStateRuleSelect_UpdateRowVisibility`: row counts -1..=100 (none,
    /// each length, capped at 11), missing list/text boxes, shown (flag > 0)
    /// and hidden rows, and the unwinds of its calls.
    #[test]
    fn cfe_state_rule_select_update_row_visibility_matches() {
        oc776_install_fakes();
        let counts: [i32; 10] = [-1, 0, 1, 2, 5, 9, 10, 11, 12, 100];
        fakes::covered(
            ee::s02::f_00192488__generated,
            cfe_state_rule_select_update_row_visibility,
            450,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7762, 40);
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                let mask = match seed % 3 {
                    0 => 0,
                    _ => r.next_u32() & r.next_u32(),
                };
                c.bus.write32(OC776_CTRL + 4, mask);
                c.g[A0][0] = sx(OC776_THIS);
                c.bus.write8(
                    OC776_THIS + RULE_ROW_COUNT,
                    counts[(seed % 10) as usize] as u8,
                );
                c.bus.write32(OC776_THIS + RULE_SCREEN, 0x1234);
                for (i, b) in b"0b\0".iter().enumerate() {
                    c.bus.write8(0x004a_16a8 + i as u32, *b);
                }
                for (i, b) in b"0t\0".iter().enumerate() {
                    c.bus.write8(0x004a_16b0 + i as u32, *b);
                }
                fill(c, &mut r, 0x004c_66c8, 4);
                for (i, slot) in [0xc8u32, 0xc0, 0x40].into_iter().enumerate() {
                    c.bus
                        .write16(OC776_VT + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus
                        .write32(OC776_VT + slot + 4, fakes::FAKE + 0x10 * i as u32);
                }
                for n in 0..32u32 {
                    let obj = OC776_ROWS + n * 0x200;
                    c.bus.write32(obj + 8, OC776_VT);
                    let data_off = 0x100 + 0x10 * (n / 2);
                    c.bus.write32(obj + RULE_ROW_DATA_OFF, data_off);
                    c.bus.write8(
                        OC776_THIS + data_off + 0x51,
                        (r.next_u32() % 5).wrapping_sub(2) as u8,
                    );
                }
            },
        );
    }

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc776_mail(addr: u32) -> u32 {
        OC776_CTRL + 0x1000 + (addr & 0xffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc776_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc776_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    fn oc776_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc776_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    /// Fake methods that return false (a real-looking 0) or the fixture's move result.
    pub(crate) const OC776_FALSE_FN: u32 = fakes::FAKE + 0x100;
    pub(crate) const OC776_MOVE_FN: u32 = fakes::FAKE + 0x110;

    fn oc776_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let move_result = c.bus.read32(OC776_CTRL + 0x10);
        fakes::fake_call(c, target);
        if c.pc == ra {
            if target == OC776_FALSE_FN {
                c.g[V0][0] = 0;
            } else if target == OC776_MOVE_FN {
                c.g[V0][0] = move_result as u64;
            }
        }
    }

    /// Plain gated fakes (logged calls) of the UI state/menu/text callees.
    macro_rules! oc776_plain_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC776_CTRL) != OC776_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc776_fake_call(c, $addr);
                }
            )*
            fn oc776_install_plain_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc776_plain_fakes!(
        (
            oc776_fk_14dc80,
            OC776_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc776_fk_14dd58,
            OC776_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc776_fk_14ad28,
            OC776_PREV_0014AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc776_fk_14bdb8,
            OC776_PREV_0014BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc776_fk_14d998,
            OC776_PREV_0014D998,
            ee::s01::OVERRIDE_0014D998,
            ee::s01::f_0014d998__generated,
            0x0014_d998
        ),
        (
            oc776_fk_14d988,
            OC776_PREV_0014D988,
            ee::s01::OVERRIDE_0014D988,
            ee::s01::f_0014d988__generated,
            0x0014_d988
        ),
        (
            oc776_fk_317d70,
            OC776_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc776_fk_2f1890,
            OC776_PREV_002F1890,
            ee::s09::OVERRIDE_002F1890,
            ee::s09::f_002f1890__generated,
            0x002f_1890
        ),
        (
            oc776_fk_2eca68,
            OC776_PREV_002ECA68,
            ee::s09::OVERRIDE_002ECA68,
            ee::s09::f_002eca68__generated,
            0x002e_ca68
        ),
        (
            oc776_fk_30d4b8,
            OC776_PREV_0030D4B8,
            ee::s10::OVERRIDE_0030D4B8,
            ee::s10::f_0030d4b8__generated,
            0x0030_d4b8
        ),
        (
            oc776_fk_30d840,
            OC776_PREV_0030D840,
            ee::s10::OVERRIDE_0030D840,
            ee::s10::f_0030d840__generated,
            0x0030_d840
        ),
        (
            oc776_fk_2f19f0,
            OC776_PREV_002F19F0,
            ee::s09::OVERRIDE_002F19F0,
            ee::s09::f_002f19f0__generated,
            0x002f_19f0
        ),
        (
            oc776_fk_19eba0,
            OC776_PREV_0019EBA0,
            ee::s03::OVERRIDE_0019EBA0,
            ee::s03::f_0019eba0__generated,
            0x0019_eba0
        ),
        (
            oc776_fk_39ff50,
            OC776_PREV_39FF50,
            ee::s13::OVERRIDE_0039FF50,
            ee::s13::f_0039ff50__generated,
            0x0039_ff50
        ),
        (
            oc776_fk_39ff78,
            OC776_PREV_39FF78,
            ee::s13::OVERRIDE_0039FF78,
            ee::s13::f_0039ff78__generated,
            0x0039_ff78
        ),
        (
            oc776_fk_39c870,
            OC776_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc776_fk_3a0318,
            OC776_PREV_3A0318,
            ee::s13::OVERRIDE_003A0318,
            ee::s13::f_003a0318__generated,
            0x003a_0318
        ),
        (
            oc776_fk_39b960,
            OC776_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc776_fk_39e948,
            OC776_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc776_fk_39e8b8,
            OC776_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc776_fk_3a1000,
            OC776_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
    );

    /// `EventSelect_UpdateConfirmPopup`: popup inactive/active, the menu
    /// found or not, the second activity check, map state 0 or not, the
    /// P2 text, Yes/No buttons present or missing, popup type 1 or not, and
    /// the unwinds of its calls.
    #[test]
    fn event_select_update_confirm_popup_matches() {
        oc776_install_fakes();
        oc776_install_plain_fakes();
        fakes::covered(
            ee::s05::f_00203a58__generated,
            event_select_update_confirm_popup,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7763, 30);
                c.pc = 0x0020_3a58;
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                // Objects by name: "1b" ConfirmMenu, "2b" P2_TransportNow,
                // "3b" ConfirmMenu_Yes, "4b" ConfirmMenu_No (object index = 2 * digit).
                for (name, digit) in [
                    (0x0047_0a20u32, b'1'),
                    (0x0047_0a70, b'2'),
                    (0x0047_06b8, b'3'),
                    (0x0047_06c8, b'4'),
                ] {
                    c.bus.write8(name, digit);
                    c.bus.write8(name + 1, b'b');
                }
                // Absent objects: one bit per object (2, 4, 6, 8).
                let absent = [0u32, 1 << 2, 1 << 4, 1 << 6, 1 << 8][((seed / 4) % 5) as usize]
                    | if seed % 7 == 0 { 1 << 6 | 1 << 8 } else { 0 };
                c.bus.write32(OC776_CTRL + 4, absent);
                c.g[A0][0] = sx(OC776_THIS);
                c.bus
                    .write32(OC776_THIS + POPUP_ACTIVE, if seed % 5 == 0 { 0 } else { 1 });
                // Finding the menu clears the popup flag (the popup closed meanwhile).
                c.bus.write32(OC776_CTRL + 8, (seed % 6 == 1) as u32);
                c.bus.write32(
                    OC776_THIS + POPUP_TYPE,
                    [1u32, 0, 2][((seed / 2) % 3) as usize],
                );
                c.bus.write32(OC776_THIS + POPUP_SCREEN, 0x4321);
                c.bus.write32(OC776_THIS + POPUP_STATE, 0x8765);
                c.bus.write32(
                    crate::difftest::GAME_GP - 0xb54,
                    if seed % 2 == 0 { 0 } else { 3 },
                );
                for (i, slot) in [0x48u32, 0x40].into_iter().enumerate() {
                    c.bus
                        .write16(OC776_VT + slot, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus
                        .write32(OC776_VT + slot + 4, fakes::FAKE + 0x10 * i as u32);
                }
                for n in 0..32u32 {
                    c.bus.write32(OC776_ROWS + n * 0x200 + 8, OC776_VT);
                }
            },
        );
    }

    /// `UIPair_HandleInput`: input 5 (previous), 6 (next), 4 (focus only) and
    /// anything else (with and without a child widget); move results with
    /// and without the changed (0x100), accepted (1) and refused (0x10) bits;
    /// event frame present or not; default or the owner's sound listener;
    /// and the unwinds of its calls.
    #[test]
    fn ui_pair_handle_input_matches() {
        oc776_install_plain_fakes();
        let moves: [u32; 7] = [0, 1, 0x10, 0x100, 0x101, 0x110, 0x111];
        fakes::covered(
            ee::s13::f_0039bed8__generated,
            ui_pair_handle_input,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7764, 9);
                c.hooks.miss = oc776_miss;
                c.pc = 0x0039_bed8;
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                let (this, input, screen, focus, focus_vt) = (
                    DATA + 0x1_0000,
                    DATA + 0x2_0000,
                    DATA + 0x3_0000,
                    DATA + 0x3_1000,
                    DATA + 0x3_2000,
                );
                let (input_vt, state, listener, listener_vt) = (
                    DATA + 0x2_1000,
                    DATA + 0x4_0000,
                    DATA + 0x4_1000,
                    DATA + 0x4_2000,
                );
                let (child, child_vt) = (DATA + 0x5_0000, DATA + 0x5_1000);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(input);
                let scenario = seed % 6;
                let slot = |c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, f: u32| {
                    c.bus
                        .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
                    c.bus.write32(vt + off + 4, f);
                };
                let pick = |on: bool, i: u32| {
                    if on {
                        fakes::FAKE + 0x10 * i
                    } else {
                        OC776_FALSE_FN
                    }
                };
                c.bus.write32(input + 8, input_vt);
                slot(
                    c,
                    &mut r,
                    input_vt,
                    0x28,
                    pick(matches!(scenario, 2 | 5), 1),
                );
                slot(c, &mut r, input_vt, 0x30, pick(scenario == 3, 2));
                slot(c, &mut r, input_vt, 0x20, pick(scenario == 4, 3));
                c.bus.write32(this + 0x5c, screen);
                c.bus
                    .write32(this + 0x7c, if scenario == 1 { 0 } else { child });
                c.bus.write32(screen + 0xd0, focus);
                c.bus.write32(focus + 8, focus_vt);
                slot(c, &mut r, focus_vt, 0x98, fakes::FAKE + 0x40);
                slot(c, &mut r, focus_vt, 0xa8, OC776_MOVE_FN);
                c.bus.write32(child + 8, child_vt);
                slot(c, &mut r, child_vt, 0x78, fakes::FAKE + 0x50);
                c.bus.write32(listener + 0, listener_vt);
                c.bus
                    .write32(crate::difftest::GAME_GP + 0x2968, listener_vt);
                slot(c, &mut r, listener_vt, 0x10, fakes::FAKE + 0x60);
                c.bus
                    .write32(state + 0x14, if (seed / 6) % 2 == 0 { listener } else { 0 });
                oc776_force(c, 0x0039_ff50, state);
                let event = if (seed / 12) % 2 == 0 { 0x1234 } else { 0xffff };
                oc776_force(c, 0x0039_ff78, event);
                c.bus
                    .write32(OC776_CTRL + 0x10, moves[((seed / 24) % 7) as usize]);
            },
        );
    }

    /// `BezierPatchCache_Reset`: pool counts of -1, 0 and a few (every
    /// combination of empty and non-empty loops, both pool sets), and the
    /// unwinds of the three `MEM_fill` calls.
    #[test]
    fn bezier_patch_cache_reset_matches() {
        oc776_install_plain_fakes();
        check(
            ee::s12::f_003739d0__generated,
            bezier_patch_cache_reset,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x7765, 3);
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                let cache = DATA + 0x1_0000;
                c.g[A0][0] = sx(cache);
                let count = |r: &mut Rng| ((r.next_u32() % 7) as i32 - 1) as u32;
                for k in 0..3u32 {
                    let (n, nb) = (count(&mut r), count(&mut r));
                    c.bus.write32(cache + 4 * k, n);
                    c.bus.write32(cache + 0x1a8 + 4 * k, nb);
                    // MEM_fill targets, slot arrays, free lists (A: +0x30.., B: +0x1cc..).
                    c.bus
                        .write32(cache + 0xc + 4 * k, DATA + 0x2_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x18 + 4 * k, DATA + 0x3_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x30 + 4 * k, DATA + 0x4_0000 + 0x400 * k);
                    c.bus
                        .write32(cache + 0x1cc + 4 * k, DATA + 0x5_0000 + 0x400 * k);
                    fill(c, &mut r, DATA + 0x3_0000 + 0x400 * k, 24);
                }
            },
        );
    }

    /// `FEModule_RiderPreviewLoadChar`: already loaded (same character and
    /// slot), a different slot of the same character, the first load
    /// (character -1: nothing to unload), a reload, characters 0..=9 (all
    /// bit masks) and out of range (the table's unknown case), slot 0 or not
    /// (extra textures), a corrupt table entry (jump out), and the unwinds
    /// of its calls. The real, unported request/update callees run on empty
    /// lists (`BE` counts forced to 0).
    #[test]
    fn fe_module_rider_preview_load_char_matches() {
        oc776_install_plain_fakes();
        let table: [u32; 10] = [
            0x19eb2c, 0x19eb34, 0x19eb3c, 0x19eb44, 0x19eb4c, 0x19eb54, 0x19eb5c, 0x19eb64,
            0x19eb6c, 0x19eb74,
        ];
        let zero_list = DATA + 0x6_0000;
        fakes::covered(
            ee::s03::f_0019e960__generated,
            fe_module_rider_preview_load_char,
            700,
            &|c, seed| {
                machine(c, seed, 0x7766, 12);
                c.pc = 0x0019_e960;
                c.bus.write32(OC776_CTRL, OC776_MAGIC);
                put_words(c, 0x0046_0b00, &table);
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                let character = (seed % 12) as u64;
                let slot = ((seed / 12) % 3) as u64;
                c.g[A1][0] = sx(0x77 + seed);
                c.g[A2][0] = character;
                c.g[A3][0] = slot;
                // The loaded character: the same (with the same or another slot),
                // none (-1), or another one.
                let old = match (seed / 36) % 4 {
                    0 | 1 => character as u32,
                    2 => u32::MAX,
                    _ => (seed % 5) + 3,
                };
                c.bus.write32(this, old);
                c.bus.write32(
                    this + PREVIEW_SLOT,
                    if (seed / 36) % 4 == 0 { slot as u32 } else { 9 },
                );
                c.bus.write32(this + 0xcc4, (seed % 3 == 0) as u32);
                c.bus.write32(this + 0xcc8, (seed % 5 == 0) as u32);
                // `PreviewUpdate` (real) leaves the model flow alone.
                c.bus.write32(this + 0xcd4, u32::MAX);
                oc776_force(c, 0x0014_bdb8, zero_list);
                oc776_force(c, 0x0014_d998, zero_list);
                oc776_force(c, 0x0014_d988, 0);
                oc776_force(c, 0x0014_ad28, zero_list);
                if seed % 41 == 0 && character < 10 {
                    c.bus
                        .write32(0x0046_0b00 + 4 * character as u32, 0x0019_e970);
                }
            },
        );
    }
}
