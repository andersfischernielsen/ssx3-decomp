//! Staging module oc777 (wave wave245): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc777`.

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
    (0x002a_99d8, stream_buffer_ctor, "StreamBuffer_ctor"),
    (
        0x0020_2770,
        event_select_post_input,
        "EventSelect_PostInput",
    ),
    (
        0x0028_5fb0,
        ssx_audio_system_load_front_end_audio,
        "SSXAudioSystem_LoadFrontEndAudio",
    ),
    (
        0x002b_0088,
        speech_instance_load_speech_inf,
        "SpeechInstance_LoadSpeechInf",
    ),
    (
        0x0019_ff00,
        fe_module_rider_preview_request_bolt_models,
        "FEModule_RiderPreviewRequestBoltModels",
    ),
    (
        0x0029_f3f8,
        ssx_audio_system_load_grunt_bank,
        "SSXAudioSystem_LoadGruntBank",
    ),
];

/// `void cUITemplate_MAP::onInputAlways(cUITemplate_MAP* this, cUIInput* in)`
/// (0x202770, frame 0x30): the event-select map screen's per-frame input hook.
/// Asks the input object (`a1`) its two virtual predicates (`isUp`, `isDown`,
/// thiscall tables at +0x88/+0x8c and +0x90/+0x94 of its class); if neither is
/// set it does nothing. Otherwise, depending on the map state word at
/// `gp-0xb54` (0: peak list, 1: peak goals, 2: event list) it refreshes the
/// matching list (`EventSelect_RefreshPeakList` / `RefreshPeakGoals` +
/// `UpdateModePanels`, or `RefreshEventList`).
pub fn event_select_post_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_202814: {
            'b_202810: {
                'b_2027c0: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0020279c);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0020279c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        break 'b_2027c0;
                    } else {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002027b8);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x002027b8)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        if t {
                            break 'b_202810;
                        } else {
                            break 'b_2027c0;
                        }
                    }
                }
                'b_2027dc: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        break 'b_2027dc;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002027d0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00203d40, 0x002027d0)?;
                        c.g[RA][0] = sx(0x002027d8);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00202ad8, 0x002027d8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        break 'b_2027dc;
                    }
                }
                'b_2027fc: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                    if t {
                        break 'b_2027fc;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002027f0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00205098, 0x002027f0)?;
                        c.g[RA][0] = sx(0x002027f8);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00202ad8, 0x002027f8)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                        break 'b_2027fc;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_202814;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00202810);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00207430, 0x00202810)?;
                    break 'b_202810;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_202814;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}
/// `cStreamInstance* StreamBuffer_ctor(cStreamInstance* self, SndBankMonitor* p,
/// int heap, int extra)` (0x2a99d8, frame 0x50): resets the instance fields
/// (+0x74/+0x78/+0x80 = 0, handle -1, heap type +0x7c), picks up the volume
/// control pointer of the current monitor instance, and, when set, rescales
/// the instance's base volume by it (clamped to 0..127, `cvt.w.s`) and mirrors
/// the clamped value into the current-volume array. Then allocates the stream
/// buffer (`SNDSTRM_overhead(8, 300) + extra` bytes through
/// `operator_new_named`, 0x317e30), creates the stream (`SNDSTRM_create`) and
/// stores the handle at +8. Returns `self`.
pub fn stream_buffer_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a9b84: {
            c.cycles += 29;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0xffffffffffffffff;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A3][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.sw((c.g[S2][0] as u32).wrapping_add(0x74), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x78), 0u64);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[S3][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x80), 0u64);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[A1][0] == 0u64;
            c.sb((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            if t {
                break 'b_2a9b84;
            } else {
                'b_2a9b0c: {
                    'b_2a9b08: {
                        'b_2a9ab8: {
                            c.cycles += 16;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.lwc1(2, c.g[A1][0] as u32);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[AT][0] = 0x42fe0000;
                            c.mtc1(1, 3);
                            c.lwc1(0, c.g[V0][0] as u32);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.c_lt_s(0, 3);
                            let t = !c.fcc();
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            if t {
                                break 'b_2a9ab8;
                            } else {
                                c.cycles += 11;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                c.mtc1(0, 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.lwc1(0, c.g[V0][0] as u32);
                                c.cvt_s_w(0, 0);
                                c.mul_s(0, 0, 2);
                                c.c_le_s(1, 0);
                                let t = !c.fcc();
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_2a9b08;
                                } else {
                                    break 'b_2a9ab8;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.cvt_s_w(0, 0);
                        c.mul_s(0, 0, 2);
                        c.c_lt_s(0, 3);
                        let t = !c.fcc();
                        c.g[V0][0] = 0x7f;
                        if t {
                            break 'b_2a9b08;
                        } else {
                            c.cycles += 10;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.lwc1(0, c.g[V0][0] as u32);
                            c.cvt_s_w(0, 0);
                            c.mul_s(0, 0, 2);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            let t = true;
                            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_2a9b0c;
                            } else {
                                break 'b_2a9b08;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    break 'b_2a9b0c;
                }
                'b_2a9b80: {
                    'b_2a9b54: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x18;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.mult(0, 4, 4, 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        if t {
                            break 'b_2a9b54;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            let t = (c.g[V1][0] as i64) < 0;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_2a9b80;
                            } else {
                                break 'b_2a9b54;
                            }
                        }
                    }
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                    if c.g[V1][0] == 0u64 {
                        c.g[V0][0] = 0x7f;
                        break 'b_2a9b80;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                        c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        break 'b_2a9b80;
                    }
                }
                c.cycles += 1;
                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                break 'b_2a9b84;
            }
        }
        c.cycles += 31;
        c.g[A1][0] = 0x12c;
        c.g[RA][0] = sx(0x002a9b90);
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x003b7b00, 0x002a9b90)?;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3770));
        c.g[RA][0] = sx(0x002a9bac);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x00317e30, 0x002a9bac)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V1][0] = 0x18;
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[A1][0] = 0x8;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[A2][0] = 0x12c;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 4, 4, 3);
        c.g[RA][0] = sx(0x002a9bdc);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, 0x003b7818, 0x002a9bdc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[V0][0] = c.g[S2][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}
/// `void FEModule_RiderPreviewRequestBoltModels(FERider* self, BoltList* list,
/// BoltEntry* prev, int force, int keepLoaded)` (0x19ff00, frame 0xb0;
/// the reference's `cFERider::loadModels`). `self+0x4` is the profile index,
/// `self` the character id; the list has `count` (+0x28c) 4-byte entries from
/// +0x290 (id halfword, flags halfword). Pass 1 (when `force` is 0): entries
/// not flagged 0x10 whose model slot is mapped release their Bolt-file slot
/// (`BoltFile_Release`) and are unmapped (`SetModelSlot(-1)`). Pass 2: for
/// unmapped entries, when `force` or (flag 0x10 and `prev` unset / not flagged)
/// it looks up the part entry (`func_0014D448`, 0x38-byte records) and
/// acquires its file (`BoltFile_Acquire`, `keepLoaded` from the stack),
/// storing the slot and clearing `self+0xcb4/0xcbc`, or stores -1 when the
/// part has no file; `prev` advances per unmapped entry. Ends with
/// `BoltFile_PrioritizeModels`.
pub fn fe_module_rider_preview_request_bolt_models(c: &mut Cpu) {
    guest(c, |c| {
        'b_19ffd4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S3][0] = c.g[A2][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A1][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[FP][0] = c.g[A3][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S4][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x28c));
            c.g[RA][0] = sx(0x0019ff48);
            c.sw(c.g[SP][0] as u32, c.g[T0][0]);
            call_ptr(c, 0x0014bdb8, 0x0019ff48)?;
            c.g[S7][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1a70));
            if t {
                break 'b_19ffd4;
            } else {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
                c.g[S2][0] = c.g[S4][0];
                'l_19ff68: loop {
                    'b_19ffcc: {
                        c.cycles += 2;
                        if c.g[FP][0] != 0u64 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_19ffcc;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                            c.g[V0][0] = c.g[V0][0] & 0x10;
                            if c.g[V0][0] != 0u64 {
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                break 'b_19ffcc;
                            } else {
                                'b_19ffc8: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x0019ff90);
                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                    call_ptr(c, 0x0019cbe0, 0x0019ff90)?;
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    if t {
                                        break 'b_19ffc8;
                                    } else {
                                        c.cycles += 12;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[RA][0] = sx(0x0019ffa4);
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        call_ptr(c, 0x0019cbe0, 0x0019ffa4)?;
                                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x0019ffb4);
                                        c.g[A0][0] = c.g[S6][0];
                                        call_ptr(c, 0x0019dc20, 0x0019ffb4)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                        c.g[A3][0] = 0xffffffffffffffff;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        c.g[RA][0] = sx(0x0019ffc8);
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        call_ptr(c, 0x0019cc30, 0x0019ffc8)?;
                                        break 'b_19ffc8;
                                    }
                                }
                                c.cycles += 1;
                                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                break 'b_19ffcc;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019ff68);
                        }
                        continue 'l_19ff68;
                    } else {
                        break 'b_19ffd4;
                    }
                }
            }
        }
        'b_1a00c0: {
            c.cycles += 2;
            let t = (c.g[S4][0] as i64) <= 0;
            c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x290));
            if t {
                break 'b_1a00c0;
            } else {
                c.cycles += 1;
                c.g[S2][0] = c.g[S4][0];
                'l_19ffe0: loop {
                    'b_1a00b8: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x0019fff0);
                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                        call_ptr(c, 0x0019cbe0, 0x0019fff0)?;
                        if (c.g[V0][0] as i64) >= 0 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_1a00b8;
                        } else {
                            'b_1a00b0: {
                                'b_1a002c: {
                                    c.cycles += 2;
                                    if c.g[FP][0] != 0u64 {
                                        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                        break 'b_1a002c;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                                        c.g[V0][0] = c.g[V0][0] & 0x10;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_1a00b0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[S3][0] == 0u64 {
                                                c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                break 'b_1a002c;
                                            } else {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S3][0] as u32).wrapping_add(0x2));
                                                c.g[V0][0] = c.g[V0][0] & 0x10;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    break 'b_1a00b0;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                                    break 'b_1a002c;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_1a0058: {
                                    'b_1a0048: {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S7][0];
                                        c.g[RA][0] = sx(0x001a0038);
                                        c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                        call_ptr(c, 0x0014d448, 0x001a0038)?;
                                        let t = (c.g[V0][0] as i64) >= 0;
                                        c.g[A0][0] = c.g[V0][0];
                                        if t {
                                            break 'b_1a0048;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1a0058;
                                            } else {
                                                break 'b_1a0048;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = 0x38;
                                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4));
                                    c.mult(0, 2, 4, 2);
                                    c.g[V0][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    break 'b_1a0058;
                                }
                                'b_1a00ac: {
                                    'b_1a0098: {
                                        c.cycles += 3;
                                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                        let t = c.g[A2][0] == 0u64;
                                        c.g[A0][0] = c.g[S6][0];
                                        if t {
                                            break 'b_1a0098;
                                        } else {
                                            c.cycles += 13;
                                            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                            c.g[T0][0] = c.lw(c.g[SP][0] as u32);
                                            c.g[A3][0] = 0x8;
                                            c.g[RA][0] = sx(0x001a0078);
                                            c.g[T1][0] = 0x1;
                                            call_ptr(c, 0x0019da10, 0x001a0078)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                            c.g[A3][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                            c.g[RA][0] = sx(0x001a008c);
                                            c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                            call_ptr(c, 0x0019cc30, 0x001a008c)?;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0xcbc), 0u64);
                                            let t = true;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0xcb4), 0u64);
                                            if t {
                                                break 'b_1a00ac;
                                            } else {
                                                break 'b_1a0098;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                                    c.g[A3][0] = 0xffffffffffffffff;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                    c.g[RA][0] = sx(0x001a00ac);
                                    c.g[A2][0] = c.lh(c.g[S0][0] as u32);
                                    call_ptr(c, 0x0019cc30, 0x001a00ac)?;
                                    break 'b_1a00ac;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                                break 'b_1a00b0;
                            }
                            c.cycles += 2;
                            if c.g[S3][0] != 0 {
                                c.g[S3][0] = c.g[V0][0];
                            }
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                            break 'b_1a00b8;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S2][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019ffe0);
                        }
                        continue 'l_19ffe0;
                    } else {
                        break 'b_1a00c0;
                    }
                }
            }
        }
        c.cycles += 15;
        c.g[A1][0] = c.lw(c.g[S1][0] as u32);
        c.g[RA][0] = sx(0x001a00cc);
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x0019df20, 0x001a00cc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}
/// `void SSXAudioSystem_LoadGruntBank(SSXAudioSystem* this)` (0x29f3f8,
/// frame 0x90): finds the rider's grunt voice (`SSXAudioSystem_GetGruntVoice`
/// on `AI.Riders[0]`'s speech id), picks the bank file name of the 10 grunt
/// voices through a jump table (0x482f60; default `GRNT_none.bnk`), formats it
/// as `%s%s` with the audio data path and loads it into slot 0xe of the music
/// stream system's bank manager (`SndBankMan_LoadBank`), then does the same for
/// the shared `GRNT_AI.bnk` into slot 0xf. The resume entries inside the jump
/// table fall back to the generated `__entries` twin.
pub fn ssx_audio_system_load_grunt_bank(c: &mut Cpu) {
    if c.pc != 0x0029_f3f8 {
        return ee::s08::f_0029f3f8__entries(c);
    }
    guest(c, |c| {
        'b_29f57c: {
            'b_29f550: {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                c.g[RA][0] = sx(0x0029f418);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0028b1d8, 0x0029f418)?;
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[RA][0] = sx(0x0029f424);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0029f378, 0x0029f424)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t {
                    break 'b_29f550;
                } else {
                    'b_29f538: {
                        'b_29f524: {
                            'b_29f50c: {
                                'b_29f4f4: {
                                    'b_29f4dc: {
                                        'b_29f4c4: {
                                            'b_29f4ac: {
                                                'b_29f494: {
                                                    'b_29f47c: {
                                                        'b_29f464: {
                                                            c.cycles += 6;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x2f60));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x0029f44c => {
                                                                    c.cycles += 6;
                                                                    c.g[V0][0] = 0x4a0000;
                                                                    c.g[S0][0] = 0x480000;
                                                                    c.g[A3][0] = 0x480000;
                                                                    c.g[A0][0] = c.g[SP][0];
                                                                    let t = true;
                                                                    c.g[A3][0] = sx((c.g[A3][0]
                                                                        as u32)
                                                                        .wrapping_add(0x2e98));
                                                                    if t {
                                                                        break 'b_29f538;
                                                                    } else {
                                                                        break 'b_29f464;
                                                                    }
                                                                }
                                                                0x0029f464 => {
                                                                    break 'b_29f464;
                                                                }
                                                                0x0029f47c => {
                                                                    break 'b_29f47c;
                                                                }
                                                                0x0029f494 => {
                                                                    break 'b_29f494;
                                                                }
                                                                0x0029f4ac => {
                                                                    break 'b_29f4ac;
                                                                }
                                                                0x0029f4c4 => {
                                                                    break 'b_29f4c4;
                                                                }
                                                                0x0029f4dc => {
                                                                    break 'b_29f4dc;
                                                                }
                                                                0x0029f4f4 => {
                                                                    break 'b_29f4f4;
                                                                }
                                                                0x0029f50c => {
                                                                    break 'b_29f50c;
                                                                }
                                                                0x0029f524 => {
                                                                    break 'b_29f524;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x0029f3f8, 0x0029f5dc,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[V0][0] = 0x4a0000;
                                                        c.g[S0][0] = 0x480000;
                                                        c.g[A3][0] = 0x480000;
                                                        c.g[A0][0] = c.g[SP][0];
                                                        let t = true;
                                                        c.g[A3][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x2ea8));
                                                        if t {
                                                            break 'b_29f538;
                                                        } else {
                                                            break 'b_29f47c;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[V0][0] = 0x4a0000;
                                                    c.g[S0][0] = 0x480000;
                                                    c.g[A3][0] = 0x480000;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    let t = true;
                                                    c.g[A3][0] =
                                                        sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x2eb8));
                                                    if t {
                                                        break 'b_29f538;
                                                    } else {
                                                        break 'b_29f494;
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[V0][0] = 0x4a0000;
                                                c.g[S0][0] = 0x480000;
                                                c.g[A3][0] = 0x480000;
                                                c.g[A0][0] = c.g[SP][0];
                                                let t = true;
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x2ec8));
                                                if t {
                                                    break 'b_29f538;
                                                } else {
                                                    break 'b_29f4ac;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[V0][0] = 0x4a0000;
                                            c.g[S0][0] = 0x480000;
                                            c.g[A3][0] = 0x480000;
                                            c.g[A0][0] = c.g[SP][0];
                                            let t = true;
                                            c.g[A3][0] =
                                                sx((c.g[A3][0] as u32).wrapping_add(0x2ed8));
                                            if t {
                                                break 'b_29f538;
                                            } else {
                                                break 'b_29f4c4;
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[V0][0] = 0x4a0000;
                                        c.g[S0][0] = 0x480000;
                                        c.g[A3][0] = 0x480000;
                                        c.g[A0][0] = c.g[SP][0];
                                        let t = true;
                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2ee8));
                                        if t {
                                            break 'b_29f538;
                                        } else {
                                            break 'b_29f4dc;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[S0][0] = 0x480000;
                                    c.g[A3][0] = 0x480000;
                                    c.g[A0][0] = c.g[SP][0];
                                    let t = true;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2ef8));
                                    if t {
                                        break 'b_29f538;
                                    } else {
                                        break 'b_29f4f4;
                                    }
                                }
                                c.cycles += 6;
                                c.g[V0][0] = 0x4a0000;
                                c.g[S0][0] = 0x480000;
                                c.g[A3][0] = 0x480000;
                                c.g[A0][0] = c.g[SP][0];
                                let t = true;
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f08));
                                if t {
                                    break 'b_29f538;
                                } else {
                                    break 'b_29f50c;
                                }
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x4a0000;
                            c.g[S0][0] = 0x480000;
                            c.g[A3][0] = 0x480000;
                            c.g[A0][0] = c.g[SP][0];
                            let t = true;
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f18));
                            if t {
                                break 'b_29f538;
                            } else {
                                break 'b_29f524;
                            }
                        }
                        c.cycles += 5;
                        c.g[V0][0] = 0x4a0000;
                        c.g[S0][0] = 0x480000;
                        c.g[A3][0] = 0x480000;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f28));
                        break 'b_29f538;
                    }
                    c.cycles += 6;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x36e0));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2e88));
                    c.g[RA][0] = sx(0x0029f548);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x00417828, 0x0029f548)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
                    if t {
                        break 'b_29f57c;
                    } else {
                        break 'b_29f550;
                    }
                }
            }
            c.cycles += 11;
            c.g[S0][0] = 0x4a0000;
            c.g[S1][0] = 0x480000;
            c.g[A3][0] = 0x480000;
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x36e0));
            c.g[A0][0] = c.g[SP][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f38));
            c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2e88));
            c.g[RA][0] = sx(0x0029f574);
            c.g[S3][0] = c.g[S0][0];
            call_ptr(c, 0x00417828, 0x0029f574)?;
            c.g[S0][0] = c.g[S1][0];
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
            break 'b_29f57c;
        }
        c.cycles += 24;
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0u64;
        c.g[A1][0] = 0xe;
        c.g[RA][0] = sx(0x0029f590);
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, 0x0028bc58, 0x0029f590)?;
        c.g[A3][0] = 0x480000;
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x36e0));
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2e88));
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x0029f5a8);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f48));
        call_ptr(c, 0x00417828, 0x0029f5a8)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
        c.g[A1][0] = 0xf;
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = 0u64;
        c.g[RA][0] = sx(0x0029f5c0);
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        call_ptr(c, 0x0028bc58, 0x0029f5c0)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void SpeechInstance_LoadSpeechInf(SpeechInstance* self, int instance,
/// const char* section, int loadLangHead)` (0x2b0088, frame 0xae0): opens the
/// speech ini (`IniParser_ctor`), finds the section prefix and walks its keys:
/// `BIGFILE` sets the stream file of the stream system, `BASEPATH` stores the
/// base path (+0x6c), formats `%sheaders.big` and loads the events
/// (`SpeechInstance_LoadEvents`, stopping at the first failure), the
/// language head (`%slanghead.big`) when requested, and counts the event file
/// (+0xec), `BANK` formats `%s%s.bnk` and calls `SpeechInstance_AddBank`.
/// The key loop is a native loop with the event check at the back edge; a
/// resume at a block head (`c.pc != entry`) falls back to the generated
/// dispatcher.
pub fn speech_instance_load_speech_inf(c: &mut Cpu) {
    if c.pc != 0x002b_0088 {
        return ee::s08::f_002b0088__generated(c);
    }
    guest(c, |c| {
        'b_2b0258: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xae0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xab0));
            c.g[FP][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa80));
            c.g[S2][0] = c.g[A0][0];
            c.g[S5][0] = c.g[A2][0];
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = c.g[SP][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xad0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xac0));
            c.g[A2][0] = 0x100;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xaa0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa90));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa70));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa60));
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa40), c.g[RA][0]);
            c.g[RA][0] = sx(0x002b00d4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0xa30), c.g[A3][0]);
            call_ptr(c, 0x002a77f8, 0x002b00d4)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x002b00e0);
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, 0x002a7c68, 0x002b00e0)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[SP][0];
            if t {
                break 'b_2b0258;
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x002b00f0);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002a7a20, 0x002b00f0)?;
            c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x910));
            c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x950));
            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9d0));
            'l_2b0244: loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002b024c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x002b024c)?;
                let t = c.g[V0][0] == 0u64;
                if !t {
                    break 'l_2b0244;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b0100);
                }
                c.cycles += 10;
                c.g[RA][0] = sx(0x002b0108);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x002b0108)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x4a0000;
                c.g[A1][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x37c8));
                c.g[RA][0] = sx(0x002b0120);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041aa88, 0x002b0120)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x480000;
                if !t {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002b0138);
                    c.g[A2][0] = c.g[S7][0];
                    call_ptr(c, 0x002a7ff8, 0x002b0138)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
                    c.g[A2][0] = c.g[S7][0];
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x002b014c);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x002aab98, 0x002b014c)?;
                    c.g[V0][0] = 0x480000;
                }
                c.cycles += 6;
                c.g[A1][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28c0));
                c.g[RA][0] = sx(0x002b0160);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041aa88, 0x002b0160)?;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x4a0000;
                'b_2b01f4: {
                    if t {
                        break 'b_2b01f4;
                    }
                    c.cycles += 19;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002b017c);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x002a7ff8, 0x002b017c)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.g[V0][0] = 0x480000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x32d8));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002b0194);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, 0x00417828, 0x002b0194)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x002b01ac);
                    c.g[T0][0] = c.g[S5][0];
                    call_ptr(c, 0x002afd40, 0x002b01ac)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa30));
                    if t {
                        break 'l_2b0244;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x480000;
                    if !t {
                        c.cycles += 10;
                        c.g[A2][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002b01cc);
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x32e8));
                        call_ptr(c, 0x00417828, 0x002b01cc)?;
                        c.g[A2][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = 0u64;
                        c.g[A3][0] = c.g[FP][0];
                        c.g[RA][0] = sx(0x002b01e4);
                        c.g[T0][0] = c.g[S5][0];
                        call_ptr(c, 0x002afd40, 0x002b01e4)?;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xec));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xec), c.g[V0][0]);
                    c.g[V0][0] = 0x4a0000;
                }
                c.cycles += 6;
                c.g[A1][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x37d0));
                c.g[RA][0] = sx(0x002b0204);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0041aa88, 0x002b0204)?;
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = c.g[SP][0];
                if !t {
                    c.cycles += 14;
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.g[RA][0] = sx(0x002b0218);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x002a7ff8, 0x002b0218)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x36e0));
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c));
                    c.g[RA][0] = sx(0x002b0230);
                    c.g[A3][0] = c.g[S4][0];
                    call_ptr(c, 0x00417828, 0x002b0230)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x002b0244);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x002b0348, 0x002b0244)?;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[SP][0];
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x002b0260);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002a7890, 0x002b0260)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xad0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xac0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xab0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xaa0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa90));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa80));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa70));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa60));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xae0));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_LoadFrontEndAudio(SSXAudioSystem* this)` (0x285fb0,
/// frame 0xa20; the reference's `cSSXAudio_FrontEndLoad`): sets up the channel
/// gains and audio mode 1, opens the audio ini (`IniParser_ctor`, 0x2a77f8,
/// path from `gp+0x464`). It scans the `GLOBAL` section for its `BASEPATH`
/// (loop 1, `strcpy` of it), then the `FE` section: for each key equal to
/// `MAIN` (`strcasecmp`) it formats `%s%s%s.b` and loads that bank
/// synchronously (`SndBankMan_LoadBank`, slot 0). Then it ducks to the stream
/// (`SetFadeMode(3, 0.5, 1.0)`), sets the channel gain through the music
/// system's vtable entry 2 (`charsel`), sends message 301 (`0x2b49c0`), stores
/// 10 at +0x6290 and the radio mode at +0x6090, resumes the audio and
/// destroys the parser. Both key loops are native loops with the event check
/// at the back edge; a resume at a block head (`c.pc != entry`) falls back to
/// the generated dispatcher.
pub fn ssx_audio_system_load_front_end_audio(c: &mut Cpu) {
    if c.pc != 0x0028_5fb0 {
        return ee::s07::f_00285fb0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa20));
        c.g[A1][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x9e0));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa10));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x9c0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa00));
        c.g[S0][0] = 0x4a0000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x9f0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x9d0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x9b0), c.g[RA][0]);
        c.g[RA][0] = sx(0x00285fe4);
        c.g[S5][0] = 0x4a0000;
        call_ptr(c, 0x00288d18, 0x00285fe4)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x00285ff0);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x00285be0, 0x00285ff0)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x464));
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00286000);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x002a77f8, 0x00286000)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3570));
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00286018);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        call_ptr(c, 0x002a7bf8, 0x00286018)?;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        'b_286054: {
            if t {
                break 'b_286054;
            }
            c.cycles += 2;
            'l_286044: loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0028604c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x0028604c)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if !t {
                    break 'l_286044;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00286028);
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x00286030);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x00286030)?;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28c0));
                c.g[RA][0] = sx(0x00286044);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                call_ptr(c, 0x002a7ff8, 0x00286044)?;
            }
        }
        c.cycles += 7;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3578));
        c.g[A0][0] = c.g[SP][0];
        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
        c.g[RA][0] = sx(0x00286068);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x002a7bf8, 0x00286068)?;
        let t = c.g[V0][0] == 0u64;
        'b_286120: {
            if t {
                break 'b_286120;
            }
            c.cycles += 6;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x890));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
            c.g[RA][0] = sx(0x00286080);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x004166f4, 0x00286080)?;
            c.g[S2][0] = c.g[S0][0];
            'l_286110: loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00286118);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x00286118)?;
                let t = c.g[V0][0] == 0u64;
                if !t {
                    break 'l_286110;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00286088);
                }
                c.cycles += 15;
                c.g[RA][0] = sx(0x00286090);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x00286090)?;
                c.g[A1][0] = 0x480000;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28c0));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x002860a8);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x002a7ff8, 0x002860a8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3580));
                c.g[RA][0] = sx(0x002860bc);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0041aa88, 0x002860bc)?;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.g[S1][0];
                if !t {
                    c.cycles += 19;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x910));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002860d4);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x002a7ff8, 0x002860d4)?;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x970));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3590));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002860f8);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x002860f8)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00286110);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x0028bc58, 0x00286110)?;
                }
            }
        }
        c.cycles += 43;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S3][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 13);
        c.g[A1][0] = 0x3;
        c.g[RA][0] = sx(0x00286140);
        c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3598));
        call_ptr(c, 0x00287f00, 0x00286140)?;
        c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x0028615c);
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x00287968, 0x0028615c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = c.g[V0][0];
        c.g[A2][0] = 0xc;
        c.g[T0][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00286178);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, t, 0x00286178)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x00286184);
        c.g[A1][0] = 0x12d;
        call_ptr(c, 0x002b49c0, 0x00286184)?;
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x608c));
        c.g[V0][0] = 0xa;
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(0x6290), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028619c);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x6090), c.g[V1][0]);
        call_ptr(c, 0x00289bb8, 0x0028619c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x002861a8);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002a7890, 0x002861a8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa10));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa00));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x9f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x9e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x9d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x9c0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x9b0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa20));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and the fixture objects, clear of the fakes' call
    /// log (`DATA + 0x10_0000`) and pointer range (`DATA .. DATA + 0x4_0000`).
    const OC777_CTRL: u32 = DATA + 0x2c_0000;
    const OC777_MAGIC: u32 = 0x4f43_3737 + 0x77;
    /// Address whose gated fake unwinds (pc = 0xdead_xxxx) whenever it is
    /// called; 0 for none.
    const OC777_UNWIND_ADDR: u32 = OC777_CTRL + 4;
    /// Per fake address (index in `OC777_ADDRS`): result mask word and result
    /// value word. Call k (k < 32) of that address returns the value in `v0`
    /// when bit k of the mask is set.
    const OC777_PAT: u32 = DATA + 0x2c_1000;
    /// Per fake address: how many times it was called.
    const OC777_CNT: u32 = DATA + 0x2c_2000;
    const OC777_OBJ: u32 = DATA + 0x30_0000;
    /// The real leaf at 0x1406c0 returns 0.
    const OC777_ZERO_LEAF: u32 = 0x0014_06c0;

    /// Gated fakes for the callees that have an override slot. A fake logs
    /// the call (every register) like `fakes::fake_call`, then the fixture can
    /// script its result (`oc777_pat`) or make it unwind (`OC777_UNWIND_ADDR`).
    macro_rules! oc777_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            const OC777_ADDRS: &[u32] = &[$($addr),*];
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC777_CTRL) != OC777_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc777_fake_call(c, $addr);
                }
            )*
            fn oc777_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    fn oc777_index(addr: u32) -> u32 {
        OC777_ADDRS
            .iter()
            .position(|&a| a == addr)
            .expect("fake address") as u32
    }

    /// One gated fake call: the logged call of `fakes::fake_call`, then the
    /// scripted result and the scripted unwind.
    fn oc777_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let idx = oc777_index(addr);
        let k = c.bus.read32(OC777_CNT + 4 * idx);
        c.bus.write32(OC777_CNT + 4 * idx, k + 1);
        if k < 32 && (c.bus.read32(OC777_PAT + 8 * idx) >> k) & 1 != 0 {
            let v = c.bus.read32(OC777_PAT + 8 * idx + 4);
            c.g[V0][0] = sx(v);
        }
        if addr == c.bus.read32(OC777_UNWIND_ADDR) {
            let n = c.bus.read32(fakes::LOG_COUNT);
            c.pc = 0xdead_0000 | n << 4;
        }
    }

    /// Script the results of the fake at `addr`: calls k with bit k of `mask`
    /// set return `value` in `v0`.
    fn oc777_pat(c: &mut Cpu, addr: u32, mask: u32, value: u32) {
        let idx = oc777_index(addr);
        c.bus.write32(OC777_PAT + 8 * idx, mask);
        c.bus.write32(OC777_PAT + 8 * idx + 4, value);
    }

    /// Open the gate (both machines of a diff get their own).
    fn oc777_gate(c: &mut Cpu) {
        c.bus.write32(OC777_CTRL, OC777_MAGIC);
    }

    oc777_fakes!(
        (
            oc777_fake_00202ad8,
            OC777_PREV_00202AD8,
            ee::s05::OVERRIDE_00202AD8,
            ee::s05::f_00202ad8__generated,
            0x0020_2ad8
        ),
        (
            oc777_fake_00203d40,
            OC777_PREV_00203D40,
            ee::s05::OVERRIDE_00203D40,
            ee::s05::f_00203d40__generated,
            0x0020_3d40
        ),
        (
            oc777_fake_00317670,
            OC777_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc777_fake_00317e30,
            OC777_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc777_fake_00287920,
            OC777_PREV_00287920,
            ee::s07::OVERRIDE_00287920,
            ee::s07::f_00287920__generated,
            0x0028_7920
        ),
        (
            oc777_fake_00285be0,
            OC777_PREV_00285BE0,
            ee::s07::OVERRIDE_00285BE0,
            ee::s07::f_00285be0__generated,
            0x0028_5be0
        ),
        (
            oc777_fake_00287968,
            OC777_PREV_00287968,
            ee::s07::OVERRIDE_00287968,
            ee::s07::f_00287968__generated,
            0x0028_7968
        ),
        (
            oc777_fake_00287f00,
            OC777_PREV_00287F00,
            ee::s07::OVERRIDE_00287F00,
            ee::s07::f_00287f00__generated,
            0x0028_7f00
        ),
        (
            oc777_fake_00289bb8,
            OC777_PREV_00289BB8,
            ee::s07::OVERRIDE_00289BB8,
            ee::s07::f_00289bb8__generated,
            0x0028_9bb8
        ),
        (
            oc777_fake_0028b830,
            OC777_PREV_0028B830,
            ee::s07::OVERRIDE_0028B830,
            ee::s07::f_0028b830__generated,
            0x0028_b830
        ),
        (
            oc777_fake_0028a728,
            OC777_PREV_0028A728,
            ee::s07::OVERRIDE_0028A728,
            ee::s07::f_0028a728__generated,
            0x0028_a728
        ),
        (
            oc777_fake_0028b320,
            OC777_PREV_0028B320,
            ee::s07::OVERRIDE_0028B320,
            ee::s07::f_0028b320__generated,
            0x0028_b320
        ),
        (
            oc777_fake_0028b1d8,
            OC777_PREV_0028B1D8,
            ee::s07::OVERRIDE_0028B1D8,
            ee::s07::f_0028b1d8__generated,
            0x0028_b1d8
        ),
        (
            oc777_fake_002b4388,
            OC777_PREV_002B4388,
            ee::s08::OVERRIDE_002B4388,
            ee::s08::f_002b4388__generated,
            0x002b_4388
        ),
        (
            oc777_fake_002a77f8,
            OC777_PREV_002A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc777_fake_002a7890,
            OC777_PREV_002A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc777_fake_002a7bf8,
            OC777_PREV_002A7BF8,
            ee::s08::OVERRIDE_002A7BF8,
            ee::s08::f_002a7bf8__generated,
            0x002a_7bf8
        ),
        (
            oc777_fake_002a7f90,
            OC777_PREV_002A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90
        ),
        (
            oc777_fake_002a7ff8,
            OC777_PREV_002A7FF8,
            ee::s08::OVERRIDE_002A7FF8,
            ee::s08::f_002a7ff8__generated,
            0x002a_7ff8
        ),
        (
            oc777_fake_002b49c0,
            OC777_PREV_002B49C0,
            ee::s08::OVERRIDE_002B49C0,
            ee::s08::f_002b49c0__generated,
            0x002b_49c0
        ),
        (
            oc777_fake_002a78f8,
            OC777_PREV_002A78F8,
            ee::s08::OVERRIDE_002A78F8,
            ee::s08::f_002a78f8__generated,
            0x002a_78f8
        ),
        (
            oc777_fake_002a78e0,
            OC777_PREV_002A78E0,
            ee::s08::OVERRIDE_002A78E0,
            ee::s08::f_002a78e0__generated,
            0x002a_78e0
        ),
        (
            oc777_fake_002a7a20,
            OC777_PREV_002A7A20,
            ee::s08::OVERRIDE_002A7A20,
            ee::s08::f_002a7a20__generated,
            0x002a_7a20
        ),
        (
            oc777_fake_002a7c68,
            OC777_PREV_002A7C68,
            ee::s08::OVERRIDE_002A7C68,
            ee::s08::f_002a7c68__generated,
            0x002a_7c68
        ),
        (
            oc777_fake_002aab98,
            OC777_PREV_002AAB98,
            ee::s08::OVERRIDE_002AAB98,
            ee::s08::f_002aab98__generated,
            0x002a_ab98
        ),
        (
            oc777_fake_002b0348,
            OC777_PREV_002B0348,
            ee::s08::OVERRIDE_002B0348,
            ee::s08::f_002b0348__generated,
            0x002b_0348
        ),
        (
            oc777_fake_0029f378,
            OC777_PREV_0029F378,
            ee::s08::OVERRIDE_0029F378,
            ee::s08::f_0029f378__generated,
            0x0029_f378
        ),
        (
            oc777_fake_003b8160,
            OC777_PREV_003B8160,
            ee::s13::OVERRIDE_003B8160,
            ee::s13::f_003b8160__generated,
            0x003b_8160
        ),
        (
            oc777_fake_004166f4,
            OC777_PREV_004166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc777_fake_00417828,
            OC777_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc777_fake_0041aa88,
            OC777_PREV_0041AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88
        ),
        (
            oc777_fake_00418ef8,
            OC777_PREV_00418EF8,
            ee::s15::OVERRIDE_00418EF8,
            ee::s15::f_00418ef8__generated,
            0x0041_8ef8
        ),
        (
            oc777_fake_0014bdb8,
            OC777_PREV_0014BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8
        ),
        (
            oc777_fake_0014d448,
            OC777_PREV_0014D448,
            ee::s01::OVERRIDE_0014D448,
            ee::s01::f_0014d448__generated,
            0x0014_d448
        ),
        (
            oc777_fake_0019cbe0,
            OC777_PREV_0019CBE0,
            ee::s03::OVERRIDE_0019CBE0,
            ee::s03::f_0019cbe0__generated,
            0x0019_cbe0
        ),
        (
            oc777_fake_0019cc30,
            OC777_PREV_0019CC30,
            ee::s03::OVERRIDE_0019CC30,
            ee::s03::f_0019cc30__generated,
            0x0019_cc30
        ),
        (
            oc777_fake_0019da10,
            OC777_PREV_0019DA10,
            ee::s03::OVERRIDE_0019DA10,
            ee::s03::f_0019da10__generated,
            0x0019_da10
        ),
        (
            oc777_fake_0019dc20,
            OC777_PREV_0019DC20,
            ee::s03::OVERRIDE_0019DC20,
            ee::s03::f_0019dc20__generated,
            0x0019_dc20
        ),
        (
            oc777_fake_003e1908,
            OC777_PREV_003E1908,
            ee::s14::OVERRIDE_003E1908,
            ee::s14::f_003e1908__generated,
            0x003e_1908
        ),
    );

    /// The channel-volume jump table (0x482900, game data) that
    /// `SSXAudioSystem_SetChannelVolume` dispatches on.
    const OC777_CHANNEL_TABLE: [u32; 11] = [
        0x287760, 0x28776c, 0x28778c, 0x287794, 0x28779c, 0x2877a4, 0x2877ac, 0x2877b4, 0x2877bc,
        0x2877c4, 0x2877cc,
    ];
    /// The grunt-bank jump table (0x482f60, game data) of `LoadGruntBank`.
    const OC777_GRUNT_TABLE: [u32; 10] = [
        0x29f47c, 0x29f4ac, 0x29f524, 0x29f464, 0x29f4c4, 0x29f50c, 0x29f44c, 0x29f4f4, 0x29f494,
        0x29f4dc,
    ];

    fn oc777_tables(c: &mut Cpu) {
        for (i, w) in OC777_CHANNEL_TABLE.iter().enumerate() {
            c.bus.write32(0x0048_2900 + 4 * i as u32, *w);
        }
        for (i, w) in OC777_GRUNT_TABLE.iter().enumerate() {
            c.bus.write32(0x0048_2f60 + 4 * i as u32, *w);
        }
    }

    /// Fixture of `stream_buffer_ctor`: a stream instance object, and the
    /// bank monitor `p` (current instance at +0x1c, current-descriptor array
    /// at +0x20 (24-byte entries), base-volume array at +0x38, volume-control
    /// pointer array at +0x3c). A quarter of the seeds have no volume
    /// control; its float is an edge value. `unwind_alloc` makes the buffer
    /// allocation (`operator_new_named`, 0x317e30) unwind, so the real
    /// `SNDSTRM_create` is not reached.
    fn oc777_stream_fixture(c: &mut Cpu, seed: u32, unwind_alloc: bool) {
        let mut r = machine(c, seed, 0x777a, 1);
        c.pc = 0x002a_99d8;
        oc777_gate(c);
        c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        if unwind_alloc {
            c.bus.write32(OC777_UNWIND_ADDR, 0x0031_7e30);
        }
        let this = OC777_OBJ;
        let p = OC777_OBJ + 0x1000;
        let ctl_ptrs = OC777_OBJ + 0x2000;
        let base_vols = OC777_OBJ + 0x2100;
        let current = OC777_OBJ + 0x2200;
        let ctl_cells = OC777_OBJ + 0x2300;
        let inst = r.next_u32() % 4;
        c.bus.write32(p + 0x1c, inst);
        c.bus.write32(p + 0x20, current);
        c.bus.write32(p + 0x38, base_vols);
        c.bus.write32(p + 0x3c, ctl_ptrs);
        for i in 0..4u32 {
            let ctl = if i == inst && seed % 4 == 0 {
                0
            } else {
                ctl_cells + 4 * i
            };
            c.bus.write32(ctl_ptrs + 4 * i, ctl);
            let bits = r.edge_f32();
            c.bus.write32(ctl_cells + 4 * i, bits);
            let vol = match r.next_u32() % 4 {
                0 => r.next_u32(),
                1 => 126 + r.next_u32() % 4,
                _ => (r.next_u32() % 400).wrapping_sub(100),
            };
            c.bus.write32(base_vols + 4 * i, vol);
        }
        for w in 0..0x20u32 {
            c.bus.write32(current + 4 * w, r.next_u32());
        }
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(p);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(r.next_u32() % 0x400);
    }

    /// `StreamBuffer_ctor` up to the buffer allocation: field resets, the
    /// volume-control rescale and clamps (0..127, `cvt.w.s`), the current
    /// descriptor's volume byte, `SNDSTRM_overhead` (real), and the gated
    /// `operator_new_named` which unwinds.
    #[test]
    fn stream_buffer_ctor_matches() {
        oc777_install_fakes();
        check(
            ee::s08::f_002a99d8__generated,
            stream_buffer_ctor,
            80,
            &|c, seed| {
                oc777_stream_fixture(c, seed, true);
            },
        );
    }

    /// `StreamBuffer_ctor` through the real `SNDSTRM_create` on the
    /// allocated (zeroed) buffer. If the stream code runs away on the
    /// fixture, drop this test and keep the one above.
    #[test]
    fn stream_buffer_ctor_full_matches() {
        oc777_install_fakes();
        check(
            ee::s08::f_002a99d8__generated,
            stream_buffer_ctor,
            40,
            &|c, seed| {
                oc777_stream_fixture(c, seed, false);
            },
        );
    }

    /// `EventSelect_PostInput` (the MAP screen's `onInputAlways`): the input
    /// object's two virtual predicates are the real zero leaf or a fake
    /// (non-zero), the map state at `gp-0xb54` takes 0..3. State 0 runs the
    /// gated list/panel fakes; states 1 and 2 enter the real
    /// `RefreshPeakGoals`/`RefreshEventList`, whose first call (`0x317670`,
    /// a gated fake) is made to unwind.
    #[test]
    fn event_select_post_input_matches() {
        oc777_install_fakes();
        check(
            ee::s05::f_00202770__generated,
            event_select_post_input,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777b, 4);
                c.pc = 0x0020_2770;
                oc777_gate(c);
                let obj = OC777_OBJ;
                let input = OC777_OBJ + 0x100;
                let vt = OC777_OBJ + 0x200;
                let up_fake = seed & 1 == 0;
                let down_fake = seed & 2 == 0;
                let state = (seed >> 2) % 4;
                c.bus.write32(input + 8, vt);
                c.bus.write16(vt + 0x88, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(
                    vt + 0x8c,
                    if up_fake {
                        fakes::FAKE
                    } else {
                        OC777_ZERO_LEAF
                    },
                );
                c.bus.write16(vt + 0x90, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(
                    vt + 0x94,
                    if down_fake {
                        fakes::FAKE + 0x10
                    } else {
                        OC777_ZERO_LEAF
                    },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0xb54), state);
                if state == 1 || state == 2 {
                    // One fake predicate call comes first, then the callee's own
                    // first fake call (0x317670) is the one that unwinds.
                    let before = (up_fake || down_fake) as u32;
                    c.bus.write32(fakes::LOG_UNWIND_AT, before);
                }
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(input);
            },
        );
    }

    /// `SSXAudioSystem_LoadGruntBank`: the grunt voice (`0x29f378`, scripted
    /// to 0..11 so the jump table's ten cases and the default are all taken),
    /// `sprintf`, and the real `SndBankMan_LoadBank` (twice) over its gated
    /// callees.
    #[test]
    fn ssx_audio_system_load_grunt_bank_matches() {
        oc777_install_fakes();
        check(
            ee::s08::f_0029f3f8__generated,
            ssx_audio_system_load_grunt_bank,
            72,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777c, 10);
                c.pc = 0x0029_f3f8;
                oc777_gate(c);
                oc777_tables(c);
                let this = OC777_OBJ;
                let music = OC777_OBJ + 0x1000;
                let bank_man = OC777_OBJ + 0x1100;
                c.bus.write32(this + 0x118, music);
                c.bus.write32(music, bank_man);
                c.bus.write32(bank_man + 0xacc, r.next_u32());
                oc777_pat(c, 0x0029_f378, 0xffff_ffff, seed % 12);
                oc777_pat(c, 0x0028_b1d8, 0xffff_ffff, OC777_OBJ + 0x1200);
                oc777_pat(c, 0x0028_b830, r.next_u32() & 3, 0);
                c.bus.write32(OC777_OBJ + 0x1200 + 0x28, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// Shared fixture of the front-end audio load: the audio system object
    /// (`this`), its music system pointer (+0x118) and stream-system chain,
    /// the object at +0x5558 whose vtable entry 2 is a fake, and the game
    /// tables the channel-gain setup dispatches on.
    fn oc777_audio_fixture(c: &mut Cpu, r: &mut Rng) {
        oc777_tables(c);
        let this = OC777_OBJ;
        let music = OC777_OBJ + 0x1000;
        let bank_man = OC777_OBJ + 0x1100;
        let gain_obj = OC777_OBJ + 0x1200;
        c.bus.write32(this + 0x118, music);
        c.bus.write32(music, bank_man);
        c.bus.write32(bank_man + 0xacc, r.next_u32());
        c.bus.write32(this + 0x5558, gain_obj);
        c.bus.write16(gain_obj + 0x10, (r.next_u32() & 0x7c) as u16);
        c.bus.write32(gain_obj + 0x14, fakes::FAKE + 0x20);
        c.bus.write32(this + 0x608c, r.next_u32());
        c.bus.write32(GAME_GP + 0x464, r.next_u32());
        c.g[A0][0] = sx(this);
    }

    /// `SSXAudioSystem_LoadFrontEndAudio`: the ini parser calls are gated
    /// fakes with scripted results (section found or not, end-of-file after
    /// 0..4 keys, `strcasecmp` equal for some keys); the channel-gain setup,
    /// `SetChannelVolume`, `GetName` and `SndBankMan_LoadBank` run for real
    /// (the parser object is zeroed stack, so `GetName` ends at once).
    #[test]
    fn ssx_audio_system_load_front_end_audio_matches() {
        oc777_install_fakes();
        check(
            ee::s07::f_00285fb0__generated,
            ssx_audio_system_load_front_end_audio,
            72,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777d, 60);
                c.pc = 0x0028_5fb0;
                oc777_gate(c);
                oc777_audio_fixture(c, &mut r);
                oc777_pat(c, 0x002a_7bf8, r.next_u32() & 3, 0);
                oc777_pat(c, 0x002a_7f90, r.next_u32() & 0xf, 0);
                oc777_pat(c, 0x0041_aa88, r.next_u32() & 0xff, 0);
                oc777_pat(c, 0x0028_b830, r.next_u32() & 3, 0);
            },
        );
    }

    /// `SpeechInstance_LoadSpeechInf`: ini parser fakes with scripted
    /// results (section prefix found or not, 0..4 keys, `strcasecmp` hits for
    /// BIGFILE/BASEPATH/BANK). `SpeechInstance_LoadEvents` is real: its first
    /// call (`0x3e1908`) returns 0 (so it returns 0 and the key loop ends) or
    /// unwinds. The path through a successful `LoadEvents` is not reached.
    #[test]
    fn speech_instance_load_speech_inf_matches() {
        oc777_install_fakes();
        check(
            ee::s08::f_002b0088__generated,
            speech_instance_load_speech_inf,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777e, 40);
                c.pc = 0x002b_0088;
                oc777_gate(c);
                let this = OC777_OBJ;
                let stream = OC777_OBJ + 0x1400;
                c.bus.write32(this + 4, r.next_u32());
                c.bus.write32(this + 0xec, r.next_u32() % 4);
                c.bus.write32(GAME_GP + 0x6c0, r.next_u32());
                c.bus.write32(GAME_GP + 0x6c4, stream);
                c.bus.write32(stream, OC777_OBJ + 0x1500);
                oc777_pat(c, 0x002a_7c68, r.next_u32() & 1, 0);
                oc777_pat(c, 0x002a_7f90, r.next_u32() & 0x7, 0);
                oc777_pat(c, 0x0041_aa88, r.next_u32() & 0xfff, 0);
                oc777_pat(c, 0x003e_1908, 0xffff_ffff, 0);
                if seed % 5 == 0 {
                    c.bus.write32(OC777_UNWIND_ADDR, 0x003e_1908);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(r.next_u32() % 4);
                c.g[A2][0] = sx(OC777_OBJ + 0x1600);
                c.g[A3][0] = (seed / 2 % 2) as u64;
            },
        );
    }

    /// `FEModule_RiderPreviewRequestBoltModels`: a rider with 0..5 model
    /// entries (random ids and 0x10 flags), `force` and `prev` variants, the
    /// model-slot lookups scripted mapped/unmapped, the part database with
    /// parts that have a file or not, `BoltFile_PrioritizeModels` real (over a
    /// zeroed manager, its sort a gated fake).
    #[test]
    fn fe_module_rider_preview_request_bolt_models_matches() {
        oc777_install_fakes();
        check(
            ee::s03::f_0019ff00__generated,
            fe_module_rider_preview_request_bolt_models,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777f, 40);
                c.pc = 0x0019_ff00;
                oc777_gate(c);
                let this = OC777_OBJ + 0x1000;
                let list = OC777_OBJ + 0x1100;
                let prev = OC777_OBJ + 0x1300;
                let world = OC777_OBJ + 0x2000;
                let db = OC777_OBJ + 0x2100;
                let parts = OC777_OBJ + 0x2200;
                let fe = OC777_OBJ + 0x1_0000;
                c.bus.write32(this, r.next_u32() & 0xff);
                c.bus.write32(this + 4, r.next_u32() & 0xf);
                let n = r.next_u32() % 6;
                c.bus.write32(list + 0x28c, n);
                for i in 0..n {
                    c.bus
                        .write16(list + 0x290 + 4 * i, (r.next_u32() & 0x3f) as u16);
                    let flags =
                        (if r.next_u32() & 1 != 0 { 0x10u32 } else { 0 }) | (r.next_u32() & 0x3);
                    c.bus.write16(list + 0x292 + 4 * i, flags as u16);
                }
                c.bus
                    .write16(prev + 2, if r.next_u32() & 1 != 0 { 0x10 } else { 0 });
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, fe);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x1788), OC777_OBJ + 0x2300);
                c.bus.write32(db + 4, parts);
                for i in 0..4u32 {
                    let file = if r.next_u32() & 1 != 0 {
                        r.next_u32() | 1
                    } else {
                        0
                    };
                    c.bus.write32(parts + 0x38 * i + 0x28, file);
                }
                // The part-database lookup returns an index 0..2, or -1 on
                // a third of the seeds; the slot lookup answers "unmapped"
                // (-1) for some of its calls.
                oc777_pat(c, 0x0014_bdb8, 0xffff_ffff, db);
                let idx = if seed % 3 == 0 {
                    0xffff_ffff
                } else {
                    seed % 3 - 1 + (seed / 3) % 2
                };
                oc777_pat(c, 0x0014_d448, 0xffff_ffff, idx);
                oc777_pat(c, 0x0019_cbe0, r.next_u32(), 0xffff_ffff);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = if seed % 3 == 1 { 0 } else { sx(prev) };
                c.g[A3][0] = (seed % 4 == 0) as u64;
                c.g[T0][0] = sx(r.next_u32());
            },
        );
    }
}
