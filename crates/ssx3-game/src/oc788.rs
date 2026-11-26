//! Staging module oc788 (wave wave246): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc788`.

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
        0x0019_f780,
        fe_module_rider_preview_apply_bolt_materials,
        "FEModule_RiderPreviewApplyBoltMaterials",
    ),
    (
        0x0018_1308,
        cfe_state_char_select_on_activate,
        "cFEStateCharSelect_OnActivate",
    ),
    (
        0x002a_4a78,
        ssx_audio_system_schedule_intro_music,
        "SSXAudioSystem_ScheduleIntroMusic",
    ),
    (
        0x002a_31c0,
        ssx_audio_system_pa_say_race_intro,
        "SSXAudioSystem_PASayRaceIntro",
    ),
    (0x0028_b8d8, snd_bank_get_buffer, "SndBank_GetBuffer"),
    (
        0x0028_f108,
        ssx_audio_system_reset_music_fade_in,
        "SSXAudioSystem_ResetMusicFadeIn",
    ),
];

/// `int FEModule_RiderPreviewApplyBoltMaterials(RiderPreview* p)` (0x19f780).
///
/// Front-end rider preview: looks up the rider's model (`BE_GetRiderCharDBField`
/// style lookup through the BE with slot 9), and for every entry of its part
/// list whose flag `0x10` is set, finds the matching texture/material record
/// (0x14d448) and, when that record's variant byte (+0x10) is not negative,
/// applies it as a model part variant (0x30ec18) to the preview model
/// (`p+8`). Finally stores 1 through the pointer at `p+0xc74` and returns 1.
pub fn fe_module_rider_preview_apply_bolt_materials(c: &mut Cpu) {
    guest(c, |c| {
        'b_19f848: {
            c.cycles += 25;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x0019f7a4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x9;
            call_ptr(c, 0x0014dd58, 0x0019f7b0)?;
            c.g[S0][0] = c.g[V0][0];
            call_ptr(c, 0x0014bdb8, 0x0019f7b8)?;
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.lw(c.g[S2][0] as u32);
            c.g[S3][0] = c.g[V0][0];
            call_ptr(c, 0x0014ad28, 0x0019f7cc)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x290));
            c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28c));
            call_ptr(c, 0x0030eb60, 0x0019f7dc)?;
            let t = (c.g[S1][0] as i64) <= 0;
            c.g[S4][0] = 0x38;
            if t {
                break 'b_19f848;
            } else {
                c.cycles += 1;
                'l_19f7e8: loop {
                    'b_19f840: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                        c.g[V0][0] = c.g[V0][0] & 0x10;
                        if c.g[V0][0] == 0u64 {
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            break 'b_19f840;
                        } else {
                            'b_19f824: {
                                'b_19f818: {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                    call_ptr(c, 0x0014d448, 0x0019f808)?;
                                    let t = (c.g[V0][0] as i64) >= 0;
                                    c.g[V1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_19f818;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_19f824;
                                        } else {
                                            break 'b_19f818;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.mult(0, 3, 3, 20);
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                break 'b_19f824;
                            }
                            c.cycles += 3;
                            c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x10));
                            if (c.g[A1][0] as i64) < 0 {
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                break 'b_19f840;
                            } else {
                                c.cycles += 4;
                                c.g[A2][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x1));
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                call_ptr(c, 0x0030ec18, 0x0019f83c)?;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                break 'b_19f840;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S1][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019f7e8);
                        }
                        continue 'l_19f7e8;
                    } else {
                        break 'b_19f848;
                    }
                }
            }
        }
        c.cycles += 11;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc74));
        c.g[V0][0] = 0x1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        ret(c);
        return Ok(());
    })
}

/// `void cFEStateCharSelect_OnActivate(cFEStateCharSelect* self, int arg)` (0x181308).
///
/// Looks up the selected character index of the human player (`self+0x44`) in
/// the 10-entry table at 0x440f68; when found selects that entry of the
/// character menu (UI object by hash name), then registers the human
/// character with the rider manager, clears the first human's flag at +0xccc,
/// refreshes the 3D rider preview and chains to `cUIState_onGainFocus`.
pub fn cfe_state_char_select_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A1][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x0014dc80, 0x00181330)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0018133c)?;
        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x00147488, 0x00181348)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[S2][0] = 0x440000;
        'l_181350: loop {
            'b_18139c: {
                'b_181398: {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xf68));
                    if t {
                        break 'b_181398;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                        if c.g[V1][0] != c.g[A0][0] {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            if event_due(c) {
                                deliver_event_here(c, 0x00181350);
                            }
                            continue 'l_181350;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = 0x4a0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                            call_ptr(c, 0x00317670, 0x00181378)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00181384)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                break 'b_18139c;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S0][0] & 0xff;
                                call_ptr(c, 0x0039b960, 0x00181398)?;
                                break 'b_181398;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                break 'b_18139c;
            }
            c.cycles += 24;
            c.g[A3][0] = 0u64;
            c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0xf68));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0508, 0x001813b4)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0548, 0x001813c8)?;
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[V0][0] as u32).wrapping_add(0xccc), 0u64);
            call_ptr(c, 0x00181ef0, 0x001813d4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x0039e4c0, 0x001813e0)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
            ret(c);
            return Ok(());
        }
    })
}

/// `void SSXAudioSystem_ScheduleIntroMusic(SSXAudioSystem* sys)` (0x2a4a78).
///
/// Unless a music request is already pending (`sys+0x578c`) or the mute flag
/// byte (0x535c11) is set, adds a bank-monitor timer for the intro music:
/// for track group 5 a fixed one (after 0x28def0 with 13), else one with the
/// selected song's sed value (`MusicPlayer_GetSelectedSongSedValue`).
pub fn ssx_audio_system_schedule_intro_music(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a4b54: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x002a4a90)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002a4a9c)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x578c));
            if c.g[V1][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_2a4b54;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                if c.g[V1][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_2a4b54;
                } else {
                    'b_2a4b0c: {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002a3f90, 0x002a4ac0)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_2a4b0c;
                        } else {
                            c.cycles += 17;
                            c.g[A1][0] = 0xd;
                            call_ptr(c, 0x0028def0, 0x002a4ad0)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                            c.g[V0][0] = 0x4a0000;
                            c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x3657));
                            c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x3650));
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                            c.g[A3][0] = 0u64;
                            c.g[T0][0] = 0x2;
                            c.g[T1][0] = 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                            c.g[T2][0] = 0u64;
                            c.g[T3][0] = 0u64;
                            call_ptr(c, 0x002adca0, 0x002a4b04)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_2a4b54;
                            } else {
                                break 'b_2a4b0c;
                            }
                        }
                    }
                    c.cycles += 18;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
                    c.g[S1][0] = 0x4a0000;
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3650));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                    call_ptr(c, 0x002b4908, 0x002a4b28)?;
                    c.ldl(6, (c.g[S1][0] as u32).wrapping_add(0x7));
                    c.ldr(6, c.g[S1][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[T1][0] = c.g[V0][0];
                    c.g[A1][0] = 0x2;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0x4;
                    c.g[T2][0] = 0u64;
                    c.g[T3][0] = 0u64;
                    call_ptr(c, 0x002adca0, 0x002a4b50)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    break 'b_2a4b54;
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        return Ok(());
    })
}

/// `void SSXAudioSystem_PASayRaceIntro(SSXAudioSystem* sys)` (0x2a31c0).
///
/// When PA speech is on, not in event mode 4, not muted (0x535c11) and the
/// world's mode word is below 2, and no tracked sound is playing: sets the
/// speech event context 0x20c0 (event 11) and, if that succeeded, plays the
/// race intro line through the speech vtable method at `gp+0x173c` with the
/// language mask.
pub fn ssx_audio_system_pa_say_race_intro(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a32a0: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0029f128, 0x002a31d4)?;
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_2a32a0;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a4040, 0x002a31e4)?;
                if c.g[V0][0] != 0u64 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    break 'b_2a32a0;
                } else {
                    c.cycles += 9;
                    call_ptr(c, 0x0014dc80, 0x002a31f4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x002a3200)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_2a32a0;
                    } else {
                        'b_2a329c: {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                            c.g[V1][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_2a329c;
                            } else {
                                c.cycles += 5;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0u64;
                                call_ptr(c, 0x00295028, 0x002a3234)?;
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    break 'b_2a32a0;
                                } else {
                                    c.cycles += 11;
                                    c.mtc1(0, 12);
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
                                    c.g[A1][0] = 0u64;
                                    c.g[A2][0] = 0x20c0;
                                    c.g[A3][0] = 0u64;
                                    c.g[T0][0] = 0xb;
                                    c.g[T1][0] = 0u64;
                                    c.g[T2][0] = 0u64;
                                    call_ptr(c, 0x002b1458, 0x002a3260)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        break 'b_2a32a0;
                                    } else {
                                        c.cycles += 13;
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x0029f198, 0x002a3270)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x1;
                                        c.g[A1][0] = 0u64;
                                        c.g[A2][0] = 0x20c0;
                                        call_ptr(c, 0x003d8008, 0x002a3284)?;
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x2;
                                        let t = c.g[V1][0] as u32;
                                        c.g[A3][0] = 0x1;
                                        call_ptr(c, t, 0x002a329c)?;
                                        break 'b_2a329c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_2a32a0;
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        ret(c);
        return Ok(());
    })
}

/// `void* SndBank_GetBuffer(SndBank* bank, int size)` (0x28b8d8).
///
/// Lazily allocates the bank's 128-aligned buffer (`MEM_AllocNamed128`, name
/// at 0x4829a8) when `bank+8` is zero, remembering buffer (+0x10) and size
/// (+0x14); returns the buffer.
pub fn snd_bank_get_buffer(c: &mut Cpu) {
    guest(c, |c| {
        'b_28b90c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_28b90c;
            } else {
                c.cycles += 5;
                c.g[A0][0] = 0x480000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x29a8));
                call_ptr(c, 0x00252fa0, 0x0028b904)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
                break 'b_28b90c;
            }
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        return Ok(());
    })
}

/// `bool SSXAudioSystem_ResetMusicFadeIn(SSXAudioSystem* sys)` (0x28f108).
///
/// Clears the music fade-in word at `sys+0x628c`; returns whether the rider
/// rating tier is non-zero.
pub fn ssx_audio_system_reset_music_fade_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_28f12c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x00289c18, 0x0028f11c)?;
            c.g[V1][0] = c.g[V0][0];
            let t = c.g[V1][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_28f12c;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_28f12c;
            }
        }
        c.cycles += 5;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x628c), c.g[A0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        ret(c);
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The word that arms oc788's callee fakes in a machine's RAM.
    pub(crate) const OC788_CTRL: u32 = DATA + 0x30_0000;
    pub(crate) const OC788_MAGIC: u32 = 0x0788_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc788_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc788_arm(c: &mut Cpu) {
        c.bus.write32(OC788_CTRL, OC788_MAGIC);
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc788_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc788_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc788_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc788_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = sx(mail.wrapping_sub(3));
        }
    }

    macro_rules! oc788_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC788_CTRL) != OC788_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc788_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc788_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc788_fakes!(
        (
            oc788_fk_14dc80,
            OC788_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc788_fk_14dd58,
            OC788_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc788_fk_14bdb8,
            OC788_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc788_fk_14ad28,
            OC788_PREV_14AD28,
            ee::s01::OVERRIDE_0014AD28,
            ee::s01::f_0014ad28__generated,
            0x0014_ad28
        ),
        (
            oc788_fk_14d448,
            OC788_PREV_14D448,
            ee::s01::OVERRIDE_0014D448,
            ee::s01::f_0014d448__generated,
            0x0014_d448
        ),
        (
            oc788_fk_147488,
            OC788_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc788_fk_30eb60,
            OC788_PREV_30EB60,
            ee::s10::OVERRIDE_0030EB60,
            ee::s10::f_0030eb60__generated,
            0x0030_eb60
        ),
        (
            oc788_fk_30ec18,
            OC788_PREV_30EC18,
            ee::s10::OVERRIDE_0030EC18,
            ee::s10::f_0030ec18__generated,
            0x0030_ec18
        ),
        (
            oc788_fk_317670,
            OC788_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc788_fk_39d860,
            OC788_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc788_fk_39b960,
            OC788_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc788_fk_39e4c0,
            OC788_PREV_39E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc788_fk_1a0508,
            OC788_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc788_fk_1a0548,
            OC788_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
        (
            oc788_fk_181ef0,
            OC788_PREV_181EF0,
            ee::s02::OVERRIDE_00181EF0,
            ee::s02::f_00181ef0__generated,
            0x0018_1ef0
        ),
        (
            oc788_fk_29f128,
            OC788_PREV_29F128,
            ee::s08::OVERRIDE_0029F128,
            ee::s08::f_0029f128__generated,
            0x0029_f128
        ),
        (
            oc788_fk_2a4040,
            OC788_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040
        ),
        (
            oc788_fk_295028,
            OC788_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028
        ),
        (
            oc788_fk_2b1458,
            OC788_PREV_2B1458,
            ee::s08::OVERRIDE_002B1458,
            ee::s08::f_002b1458__generated,
            0x002b_1458
        ),
        (
            oc788_fk_2a3f90,
            OC788_PREV_2A3F90,
            ee::s08::OVERRIDE_002A3F90,
            ee::s08::f_002a3f90__generated,
            0x002a_3f90
        ),
        (
            oc788_fk_2adca0,
            OC788_PREV_2ADCA0,
            ee::s08::OVERRIDE_002ADCA0,
            ee::s08::f_002adca0__generated,
            0x002a_dca0
        ),
        (
            oc788_fk_2b4908,
            OC788_PREV_2B4908,
            ee::s08::OVERRIDE_002B4908,
            ee::s08::f_002b4908__generated,
            0x002b_4908
        ),
        (
            oc788_fk_3d8008,
            OC788_PREV_3D8008,
            ee::s14::OVERRIDE_003D8008,
            ee::s14::f_003d8008__generated,
            0x003d_8008
        ),
        (
            oc788_fk_28def0,
            OC788_PREV_28DEF0,
            ee::s07::OVERRIDE_0028DEF0,
            ee::s07::f_0028def0__generated,
            0x0028_def0
        ),
        (
            oc788_fk_289c18,
            OC788_PREV_289C18,
            ee::s07::OVERRIDE_00289C18,
            ee::s07::f_00289c18__generated,
            0x0028_9c18
        ),
        (
            oc788_fk_252fa0,
            OC788_PREV_252FA0,
            ee::s06::OVERRIDE_00252FA0,
            ee::s06::f_00252fa0__generated,
            0x0025_2fa0
        ),
    );

    /// `SndBank_GetBuffer`: buffer already there (no call) and not (allocator
    /// call, stores buffer and size), and the unwind of the call.
    #[test]
    fn snd_bank_get_buffer_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s07::f_0028b8d8__generated,
            snd_bank_get_buffer,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788a, 1);
                oc788_arm(c);
                c.pc = 0x0028_b8d8;
                let bank = DATA + 0x1000;
                c.g[A0][0] = sx(bank);
                c.g[A1][0] = sx(r.next_u32());
                let has = if seed % 2 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(bank + 8, has);
                c.bus.write32(bank + 0x10, r.next_u32());
                c.bus.write32(bank + 0x14, r.next_u32());
            },
        );
    }

    /// `SSXAudioSystem_ResetMusicFadeIn`: rating tier zero and non-zero, and
    /// the unwind of the tier call.
    #[test]
    fn ssx_audio_system_reset_music_fade_in_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s07::f_0028f108__generated,
            ssx_audio_system_reset_music_fade_in,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788b, 1);
                oc788_arm(c);
                c.pc = 0x0028_f108;
                let sys = DATA + 0x1000;
                c.g[A0][0] = sx(sys);
                let tier = if seed % 3 == 0 { 0 } else { 1 + seed % 5 };
                oc788_force(c, 0x0028_9c18, tier);
                c.bus.write32(sys + 0x628c, r.next_u32());
            },
        );
    }

    /// `FEModule_RiderPreviewApplyBoltMaterials`: empty and negative part
    /// counts, entries with and without the flag, a negative and non-negative
    /// texture index, a negative and non-negative variant byte, and unwinds.
    #[test]
    fn fe_module_rider_preview_apply_bolt_materials_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s03::f_0019f780__generated,
            fe_module_rider_preview_apply_bolt_materials,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788c, 14);
                oc788_arm(c);
                c.pc = 0x0019_f780;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                for off in [0, 4, 8] {
                    c.bus.write32(this + off, r.next_u32());
                }
                c.bus.write32(this + 0xc74, DATA + 0x1200 + 4 * (seed % 3));
                let rec = DATA + 0x2000;
                let count: u32 = [0, -1i32 as u32, 1, 3, 5][seed as usize % 5];
                c.bus.write32(rec + 0x28c, count);
                for i in 0..6 {
                    let w = r.next_u32();
                    c.bus.write16(rec + 0x290 + 4 * i, w as u16);
                    c.bus.write16(rec + 0x292 + 4 * i, (w >> 16) as u16);
                }
                let tbl = DATA + 0x3000;
                let elems = DATA + 0x4000;
                c.bus.write32(tbl + 4, elems);
                let idx: u32 = [-1i32 as u32, 0, 1, 3][(seed as usize / 5) % 4];
                if idx != u32::MAX {
                    let el = elems + idx * 0x38;
                    c.bus
                        .write8(el + 0x10, [0x00u8, 0xff, 0x05][(seed as usize / 20) % 3]);
                    c.bus.write8(el + 1, r.next_u32() as u8);
                }
                oc788_force(c, 0x0014_ad28, rec);
                oc788_force(c, 0x0014_bdb8, tbl);
                oc788_force(c, 0x0014_d448, idx);
            },
        );
    }

    /// `cFEStateCharSelect_OnActivate`: character found at every table slot
    /// and not found, menu object found and missing, and unwinds.
    #[test]
    fn cfe_state_char_select_on_activate_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s02::f_00181308__generated,
            cfe_state_char_select_on_activate,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788d, 12);
                oc788_arm(c);
                c.pc = 0x0018_1308;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32());
                c.bus.write8(this + 0x44, r.next_u32() as u8);
                c.bus.write32(this + 0x40, r.next_u32());
                let mut bytes = [0u8; 10];
                for (i, b) in bytes.iter_mut().enumerate() {
                    *b = (r.next_u32() & 0x3f) as u8;
                    c.bus.write8(0x0044_0f68 + i as u32, *b);
                }
                let k = seed as usize % 12;
                let target = if k < 10 { bytes[k] as u32 } else { 0x70 };
                oc788_force(c, 0x0014_7488, target);
                let menu = if (seed / 12) % 2 == 0 {
                    0
                } else {
                    DATA + 0x5000
                };
                oc788_force(c, 0x0039_d860, menu);
                let world = DATA + 0x6000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, DATA + 0x6100);
                oc788_force(c, 0x001a_0548, DATA + 0x6200);
            },
        );
    }

    /// `SSXAudioSystem_ScheduleIntroMusic`: music already scheduled, muted,
    /// track group 5 and the other groups, and unwinds.
    #[test]
    fn ssx_audio_system_schedule_intro_music_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s08::f_002a4a78__generated,
            ssx_audio_system_schedule_intro_music,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788e, 6);
                oc788_arm(c);
                c.pc = 0x002a_4a78;
                let sys = DATA + 0x1000;
                c.g[A0][0] = sx(sys);
                let path = seed % 4;
                c.bus
                    .write32(sys + 0x578c, if path == 0 { r.next_u32() | 1 } else { 0 });
                c.bus.write8(0x0053_5c11, if path == 1 { 1 } else { 0 });
                oc788_force(c, 0x002a_3f90, (path == 2) as u32);
                let x = DATA + 0x2000;
                c.bus.write32(sys + 0x118, x);
                c.bus.write32(x, DATA + 0x3000);
                c.bus.write32(0x004a_3650, r.next_u32());
                c.bus.write32(0x004a_3654, r.next_u32());
            },
        );
    }

    /// `SSXAudioSystem_PASayRaceIntro`: every early exit (speech off, event
    /// mode 4, muted, world mode >= 2, sound playing, no event context) and
    /// the full path through the speech vtable method, and unwinds.
    #[test]
    fn ssx_audio_system_pa_say_race_intro_matches() {
        oc788_install_fakes();
        fakes::covered(
            ee::s08::f_002a31c0__generated,
            ssx_audio_system_pa_say_race_intro,
            320,
            &|c, seed| {
                let mut r = machine(c, seed, 0x788f, 12);
                oc788_arm(c);
                c.pc = 0x002a_31c0;
                let sys = DATA + 0x1000;
                c.g[A0][0] = sx(sys);
                let path = seed % 8;
                oc788_force(c, 0x0029_f128, (path != 0) as u32);
                oc788_force(c, 0x002a_4040, (path == 1) as u32);
                c.bus.write8(0x0053_5c11, (path == 2) as u8);
                let world = DATA + 0x6000;
                let mode = DATA + 0x6100;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0xc0, mode);
                c.bus.write32(
                    mode,
                    if path == 3 {
                        2 + r.next_u32() % 5
                    } else {
                        r.next_u32() % 2
                    },
                );
                oc788_force(c, 0x0029_5028, (path == 4) as u32);
                oc788_force(c, 0x002b_1458, (path != 5) as u32);
                c.bus.write32(GAME_GP + 0x173c, fakes::FAKE);
            },
        );
    }
}
