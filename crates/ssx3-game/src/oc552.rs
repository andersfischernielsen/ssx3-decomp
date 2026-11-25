//! Staging module oc552 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc552`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0028_08e8, script_audio_obj_apply, "ScriptAudioObj_Apply")];

/// `base + off` as a 32-bit guest address (the `addiu` on a pointer register).
#[inline(always)]
fn apply_ea(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

/// `a + b` on two 32-bit values, sign-extended as an `addu` leaves it.
#[inline(always)]
fn apply_sum(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `ScriptAudioObj_Apply` (0x2808e8; the reference calls it
/// `cScriptObjEvent_OnInSeqKeyEvent`). Runs one in-sequence key event on a
/// script audio object: `self` (a0) is the event engine, `obj` (a1) the
/// player entity, `when` (f12) the current frame and `flags` (a3) the event
/// bits. `ScriptAudio_ReadFields` (the game-event frame, a
/// `tSSXFrameGameEvent` at the new stack pointer) gives the lists:
///
/// - the owner is the transport section when the script header's first byte
///   is set, else the section of the new race's location;
/// - each world script event runs `WScriptMan_CallTableFunction`; its abort
///   hash (unless the no-script value) is recorded once per (hash, owner) in
///   the pair list at `+0x1c` (count `+0xec`);
/// - bit 8 clears and bit 4 sets the render-state indoor flag;
/// - each music event (id 0xff skipped) goes to the audio system; each speech
///   event goes to the speech system and sets `+0xf0`, except id 17 (skipped)
///   and id 16, which sets `+0xf4` (stop on speech end);
/// - bit 2: every trigger holding a sound gets its count set to 1, then the
///   triggers of the frame's sounds clear it;
/// - bit 1: walks the sound list until a sound starting after `when`; a
///   trigger that is free, or holds another sound (stopped first), starts it.
///
/// Triggers are `+0xbc`, 0xc bytes each (id, count, target).
pub fn script_audio_obj_apply(c: &mut Cpu) {
    guest(c, |c| {
        // Callees (generated addresses) and the frame's stack slots.
        const READ_FIELDS: u32 = 0x0027_cc28;
        const GET_BE: u32 = 0x0014_dc80;
        const GET_INTERFACE: u32 = 0x0014_dd58;
        const SCRIPT_DATA_INFO: u32 = 0x0027_c070;
        const ENGINE_WORLD_SECTION: u32 = 0x0027_a688;
        const BE_TRACK_INFO: u32 = 0x0014_4bc0;
        const SECTION_OWNER: u32 = 0x0022_e098;
        const WSCRIPT_CALL: u32 = 0x0030_9e50;
        const AUDIO_INSTANCE: u32 = 0x0028_b180;
        const MUSIC_EVENT: u32 = 0x0028_e8c0;
        const SPEECH_EVENT: u32 = 0x002a_19d8;
        const BANK_MONITOR_STOP: u32 = 0x002a_d5f0;
        const MEMSET: u32 = 0x0041_6210;
        const SOUND_BANK_ENTRY: u32 = 0x0028_1000;
        const CHANNEL_GAIN: u32 = 0x0028_7968;
        const PLAY_PUSHED_SOUND: u32 = 0x0029_06b8;

        c.cycles += 30;
        let frame = Frame::open(c, 0x100);
        frame.save(c, S0, 0xe0);
        frame.save(c, S6, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S6][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xf0);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S1, 0xd0);
        c.mov_s(20, 12);
        frame.save(c, S2, 0xc0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S4, 0xa0);
        frame.save(c, S5, 0x90);
        frame.save(c, S7, 0x70);
        frame.save(c, FP, 0x60);
        frame.save_ra(c, 0x50);

        // The game-event frame, and the owner section (S2).
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, READ_FIELDS, 0x0028_0934)?;
        call_ptr(c, GET_BE, 0x0028_093c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, GET_INTERFACE, 0x0028_0948)?;
        c.g[A0][0] = c.lw(apply_ea(c.g[S0][0], 0x8));
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, SCRIPT_DATA_INFO, 0x0028_0954)?;
        c.g[V1][0] = c.lb(c.g[V0][0] as u32);
        let header_zero = c.g[V1][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if !header_zero {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, ENGINE_WORLD_SECTION, 0x0028_0968)?;
            c.g[S2][0] = c.g[V0][0];
        } else {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[V1][0] = c.lw(apply_ea(c.g[V0][0], 0x84));
            c.g[S0][0] = c.lw(apply_ea(c.g[V1][0], 0x78));
            call_ptr(c, BE_TRACK_INFO, 0x0028_0980)?;
            c.g[A1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, SECTION_OWNER, 0x0028_098c)?;
            c.g[S2][0] = c.g[V0][0];
        }

        // World script events, the abort pair list, and the event bits 8/4.
        'b_280a94: {
            c.cycles += 9;
            c.g[V1][0] = c.g[S3][0] & 0x1;
            c.g[S7][0] = c.g[S3][0] & 0x8;
            c.g[FP][0] = c.g[S3][0] & 0x4;
            c.g[V0][0] = c.lw(c.g[SP][0] as u32);
            c.g[S3][0] = c.g[S3][0] & 0x2;
            c.sw(apply_ea(c.g[SP][0], 0x3c), c.g[V1][0]);
            c.g[S1][0] = 0;
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw(apply_ea(c.g[SP][0], 0x40), c.g[S3][0]);
            if t {
                break 'b_280a94;
            }
            c.cycles += 5;
            c.g[S5][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x216c));
            c.g[S4][0] = sx(apply_ea(c.g[S6][0], 0x1c));
            c.g[S3][0] = sx(apply_ea(c.g[S6][0], 0x20));
            c.g[S0][0] = c.lw(apply_ea(c.g[SP][0], 0x4));
            'l_2809c8: loop {
                'b_280a84: {
                    'b_280a80: {
                        c.cycles += 10;
                        c.g[V0][0] = sx((c.g[S1][0] as u32) << 3);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[S0][0] = apply_sum(c.g[S0][0], c.g[V0][0]);
                        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                        call_ptr(c, WSCRIPT_CALL, 0x0028_09e0)?;
                        c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x4));
                        c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                        let t = c.g[A2][0] == c.g[S5][0];
                        c.g[T1][0] = sx(apply_ea(c.g[S1][0], 0x1));
                        if t {
                            break 'b_280a80;
                        }
                        // Search the pair list for (id, owner).
                        'b_280a50: {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(apply_ea(c.g[S6][0], 0xec));
                            c.g[A3][0] = 0;
                            c.g[A1][0] = 0;
                            let t = (c.g[V0][0] as i64) <= 0;
                            c.g[T0][0] = c.g[V0][0];
                            if t {
                                break 'b_280a50;
                            }
                            'b_280a20: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(apply_ea(c.g[S6][0], 0x1c));
                                if c.g[A2][0] != c.g[V0][0] {
                                    c.g[A3][0] = sx(apply_ea(c.g[A3][0], 0x1));
                                    break 'b_280a20;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.lw(apply_ea(c.g[S6][0], 0x20));
                                if c.g[S2][0] == c.g[V0][0] {
                                    c.g[A1][0] = 0x1;
                                    break 'b_280a50;
                                }
                                c.cycles += 1;
                                c.g[A3][0] = sx(apply_ea(c.g[A3][0], 0x1));
                                break 'b_280a20;
                            }
                            'l_280a20: loop {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0;
                                c.g[A0][0] = sx((c.g[A3][0] as u32) << 3);
                                if t {
                                    break 'b_280a50;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = apply_sum(c.g[S4][0], c.g[A0][0]);
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                if c.g[A2][0] != c.g[V1][0] {
                                    c.g[A3][0] = sx(apply_ea(c.g[A3][0], 0x1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0028_0a20);
                                    }
                                    continue 'l_280a20;
                                }
                                c.cycles += 4;
                                c.g[V0][0] = apply_sum(c.g[S3][0], c.g[A0][0]);
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                if c.g[S2][0] != c.g[V1][0] {
                                    c.g[A3][0] = sx(apply_ea(c.g[A3][0], 0x1));
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0028_0a20);
                                    }
                                    continue 'l_280a20;
                                }
                                c.cycles += 1;
                                c.g[A1][0] = 0x1;
                                break 'b_280a50;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[A1][0] != 0;
                        c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                        if t {
                            break 'b_280a84;
                        }
                        // Not found: append (id, owner) to the pair list.
                        c.cycles += 10;
                        c.g[V0][0] = sx((c.g[T0][0] as u32) << 3);
                        c.g[V0][0] = apply_sum(c.g[S4][0], c.g[V0][0]);
                        c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                        c.g[V1][0] = c.lw(apply_ea(c.g[S6][0], 0xec));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                        c.g[V1][0] = apply_sum(c.g[S3][0], c.g[V1][0]);
                        c.sw(c.g[V1][0] as u32, c.g[S2][0]);
                        c.g[V0][0] = c.lw(apply_ea(c.g[S6][0], 0xec));
                        c.g[V0][0] = sx(apply_ea(c.g[V0][0], 0x1));
                        c.sw(apply_ea(c.g[S6][0], 0xec), c.g[V0][0]);
                        break 'b_280a80;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                }
                c.cycles += 4;
                c.g[S1][0] = c.g[T1][0];
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S0][0] = c.lw(apply_ea(c.g[SP][0], 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_09c8);
                    }
                    continue 'l_2809c8;
                }
                break 'b_280a94;
            }
        }

        // Event bit 8 clears and bit 4 sets the render-state indoor flag.
        c.cycles += 2;
        if c.g[S7][0] != 0 {
            c.sw((c.g[GP][0] as u32).wrapping_add(0xa70), 0);
        }
        c.cycles += 2;
        let t = c.g[FP][0] == 0;
        c.g[V0][0] = 0x1;
        if !t {
            c.cycles += 1;
            c.sw((c.g[GP][0] as u32).wrapping_add(0xa70), c.g[V0][0]);
        }

        // Music events (id 0xff skipped).
        'b_280afc: {
            c.cycles += 6;
            c.g[S1][0] = 0;
            call_ptr(c, AUDIO_INSTANCE, 0x0028_0ab0)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x8));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x10));
            if t {
                break 'b_280afc;
            }
            c.cycles += 2;
            c.g[S0][0] = 0xff;
            c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0xc));
            'l_280ac8: loop {
                'b_280ae4: {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 3);
                    c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[A1][0] == c.g[S0][0];
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        break 'b_280ae4;
                    }
                    c.cycles += 2;
                    c.g[A2][0] = 0;
                    call_ptr(c, MUSIC_EVENT, 0x0028_0ae4)?;
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x8));
                c.g[S1][0] = sx(apply_ea(c.g[S1][0], 0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0xc));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_0ac8);
                    }
                    continue 'l_280ac8;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x10));
                break 'b_280afc;
            }
        }

        // Speech events (id 17 skipped, 16 arms stop-on-speech-end).
        'b_280b5c: {
            c.cycles += 2;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if t {
                break 'b_280b5c;
            }
            c.cycles += 5;
            c.g[S3][0] = 0x11;
            c.g[S2][0] = 0x10;
            c.g[S0][0] = 0x1;
            c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x14));
            'l_280b18: loop {
                'b_280b4c: {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S1][0] as u32) << 3);
                    c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[A1][0] == c.g[S3][0];
                    c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x10));
                    if t {
                        break 'b_280b4c;
                    }
                    'b_280b48: {
                        'b_280b3c: {
                            c.cycles += 2;
                            let t = c.g[A1][0] != c.g[S2][0];
                            if t {
                                break 'b_280b3c;
                            }
                            c.cycles += 2;
                            c.sw(apply_ea(c.g[S6][0], 0xf4), c.g[S0][0]);
                            break 'b_280b48;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, SPEECH_EVENT, 0x0028_0b44)?;
                        c.sw(apply_ea(c.g[S6][0], 0xf0), c.g[S0][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x10));
                }
                c.cycles += 4;
                c.g[S1][0] = sx(apply_ea(c.g[S1][0], 0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = c.lw(apply_ea(c.g[SP][0], 0x14));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_0b18);
                    }
                    continue 'l_280b18;
                }
                break 'b_280b5c;
            }
        }

        // Bit 2: the triggers of the frame's sounds clear their end flag, the
        // other triggers with a sound get it set first.
        'b_280bd0: {
            'b_280bcc: {
                c.cycles += 3;
                c.g[A0][0] = c.lw(apply_ea(c.g[SP][0], 0x40));
                let t = c.g[A0][0] == 0;
                c.g[A1][0] = sx(apply_ea(c.g[S6][0], 0xec));
                if t {
                    break 'b_280bcc;
                }
                c.cycles += 4;
                c.g[A2][0] = 0x1;
                c.g[A0][0] = sx(apply_ea(c.g[S6][0], 0xc0));
                c.g[V1][0] = sx(apply_ea(c.g[S6][0], 0xbc));
                'l_280b78: loop {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    if (c.g[V0][0] as i64) >= 0 {
                        c.sw(c.g[A0][0] as u32, c.g[A2][0]);
                    }
                    c.cycles += 4;
                    c.g[V1][0] = sx(apply_ea(c.g[V1][0], 0xc));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0;
                    c.g[A0][0] = sx(apply_ea(c.g[A0][0], 0xc));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0028_0b78);
                        }
                        continue 'l_280b78;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(apply_ea(c.g[SP][0], 0x1c));
                    let t = c.g[V1][0] == 0;
                    c.g[A3][0] = c.lw(apply_ea(c.g[SP][0], 0x3c));
                    if t {
                        break 'b_280bd0;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = sx(apply_ea(c.g[S6][0], 0xc0));
                    c.g[V0][0] = c.lw(apply_ea(c.g[V1][0], 0xc));
                    'l_280bb0: loop {
                        c.cycles += 7;
                        c.mult(0, 5, 2, 4);
                        c.g[V0][0] = apply_sum(c.g[A1][0], c.g[A2][0]);
                        c.sw(c.g[V0][0] as u32, 0);
                        c.g[V1][0] = c.lw(apply_ea(c.g[V1][0], 0x20));
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = c.lw(apply_ea(c.g[V1][0], 0xc));
                            if event_due(c) {
                                deliver_event_here(c, 0x0028_0bb0);
                            }
                            continue 'l_280bb0;
                        }
                        break 'b_280bcc;
                    }
                }
            }
            c.cycles += 1;
            c.g[A3][0] = c.lw(apply_ea(c.g[SP][0], 0x3c));
        }

        // Bit 1: the sound list. Sounds start while their start frame is not
        // after `when`; a free or different trigger gets the new sound.
        'b_280fd0: {
            'b_280fcc: {
                c.cycles += 2;
                let t = c.g[A3][0] == 0;
                c.g[S5][0] = c.lw(apply_ea(c.g[SP][0], 0x1c));
                if t {
                    break 'b_280fcc;
                }
                c.cycles += 2;
                if c.g[S5][0] == 0 {
                    frame.restore(c, S0, 0xe0);
                    break 'b_280fd0;
                }
                c.cycles += 6;
                c.lwc1(0, apply_ea(c.g[S5][0], 0x4));
                c.cvt_s_w(0, 0);
                c.c_lt_s(0, 20);
                if c.fcc() {
                    frame.restore(c, S0, 0xe0);
                    break 'b_280fd0;
                }
                c.cycles += 4;
                c.g[T0][0] = sx(apply_ea(c.g[S6][0], 0xbc));
                c.g[FP][0] = sx(apply_ea(c.g[S6][0], 0xc4));
                c.sw(apply_ea(c.g[SP][0], 0x38), c.g[T0][0]);
                c.g[S7][0] = c.lw(apply_ea(c.g[S5][0], 0xc));
                'l_280c08: loop {
                    'b_280c50: {
                        c.cycles += 6;
                        c.g[V0][0] = 0xc;
                        c.mult(0, 3, 23, 2);
                        c.g[V0][0] = apply_sum(c.g[S6][0], c.g[V1][0]);
                        c.g[A1][0] = c.lw(apply_ea(c.g[V0][0], 0xbc));
                        let t = (c.g[A1][0] as i64) < 0;
                        c.g[A2][0] = 0x1;
                        if t {
                            break 'b_280c50;
                        }
                        c.cycles += 5;
                        c.g[V0][0] = apply_sum(c.g[FP][0], c.g[V1][0]);
                        c.g[A0][0] = c.lw(c.g[S5][0] as u32);
                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                        let t = c.g[V1][0] == c.g[A0][0];
                        c.g[A2][0] = 0;
                        if t {
                            break 'b_280c50;
                        }
                        // The trigger holds another sound: stop it first.
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(apply_ea(c.g[S4][0], 0x118));
                        c.g[A2][0] = 0x1;
                        c.mtc1(0, 12);
                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                        c.g[A0][0] = sx(apply_ea(c.g[A0][0], 0x1d8));
                        call_ptr(c, BANK_MONITOR_STOP, 0x0028_0c4c)?;
                        c.g[A2][0] = 0x1;
                    }
                    'b_280fa8: {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0;
                        c.g[A3][0] = sx(apply_ea(c.g[SP][0], 0x34));
                        if t {
                            break 'b_280fa8;
                        }
                        'b_280f2c: {
                            'b_280f00: {
                                c.cycles += 164;
                                c.g[A1][0] = c.lw(c.g[S5][0] as u32);
                                c.g[A2][0] = sx(apply_ea(c.g[SP][0], 0x30));
                                call_ptr(c, SOUND_BANK_ENTRY, 0x0028_0c68)?;
                                c.g[S0][0] = c.lw(c.g[S4][0] as u32);
                                c.g[A0][0] = 0x18;
                                c.g[S2][0] = c.lw(apply_ea(c.g[SP][0], 0x34));
                                c.g[A1][0] = 0;
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[S1][0] = sx(apply_ea(c.g[S0][0], 0x1d8));
                                c.g[S3][0] = c.lw(apply_ea(c.g[SP][0], 0x30));
                                c.g[A2][0] = 0x8;
                                c.g[V0][0] = sx(apply_ea(c.g[V0][0], 0x1));
                                c.sw(apply_ea(c.g[S0][0], 0x1f4), c.g[V0][0]);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x20));
                                c.mult(0, 3, 3, 4);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.ldl(7, apply_ea(c.g[S0][0], 0x1e3));
                                c.ldr(7, apply_ea(c.g[S0][0], 0x1dc));
                                c.ldl(8, apply_ea(c.g[S0][0], 0x1eb));
                                c.ldr(8, apply_ea(c.g[S0][0], 0x1e4));
                                c.ldl(2, apply_ea(c.g[S0][0], 0x1f3));
                                c.ldr(2, apply_ea(c.g[S0][0], 0x1ec));
                                c.sdl(7, apply_ea(c.g[V1][0], 0x7));
                                c.sdr(7, apply_ea(c.g[V1][0], 0x0));
                                c.sdl(8, apply_ea(c.g[V1][0], 0xf));
                                c.sdr(8, apply_ea(c.g[V1][0], 0x8));
                                c.sdl(2, apply_ea(c.g[V1][0], 0x17));
                                c.sdr(2, apply_ea(c.g[V1][0], 0x10));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[S0][0], c.g[V0][0]);
                                c.sw(apply_ea(c.g[V0][0], 0x1fc), 0);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x34));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(c.g[V1][0] as u32, 0);
                                c.g[V0][0] = 0x18;
                                c.g[A3][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[A0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[T0][0] = c.lw(apply_ea(c.g[S1][0], 0x20));
                                c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                                c.mult(0, 4, 4, 2);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x38));
                                c.g[A3][0] = apply_sum(c.g[A3][0], c.g[V0][0]);
                                c.g[A0][0] = apply_sum(c.g[A0][0], c.g[T0][0]);
                                c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                                c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x3c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, 0);
                                c.g[A0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                c.g[A0][0] = apply_sum(c.g[S1][0], c.g[A0][0]);
                                c.g[A0][0] = sx(apply_ea(c.g[A0][0], 0x40));
                                call_ptr(c, MEMSET, 0x0028_0d48)?;
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[A0][0] = 0x64;
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x60));
                                c.g[A1][0] = 0x5a;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A2][0] = 0x32;
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.g[A3][0] = 0x7f;
                                c.sw(c.g[V1][0] as u32, 0);
                                c.g[T0][0] = 0x1;
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x64));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, 0);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x68));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(c.g[V1][0] as u32, 0);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x6c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, 0);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x70));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(apply_ea(c.g[V1][0], 0x4), 0);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x70));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x70));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(apply_ea(c.g[V1][0], 0x8), c.g[A1][0]);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x70));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(apply_ea(c.g[V0][0], 0xc), c.g[A2][0]);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x74));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(c.g[V1][0] as u32, 0);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x80));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, 0);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x84));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(c.g[V1][0] as u32, 0);
                                c.g[V0][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[S1][0], 0x78));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S0][0], 0x1f4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S1][0], 0x7c));
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[V0][0]);
                                c.sw(c.g[V1][0] as u32, c.g[T0][0]);
                                c.g[A0][0] = c.lw(c.g[S4][0] as u32);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S4][0], 0x4));
                                c.g[V0][0] = c.lw(apply_ea(c.g[A0][0], 0x1f4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, c.g[S2][0]);
                                c.g[V1][0] = c.lw(c.g[S4][0] as u32);
                                c.g[A0][0] = c.lw(apply_ea(c.g[S4][0], 0x8));
                                c.g[V0][0] = c.lw(apply_ea(c.g[V1][0], 0x1f4));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[A0][0]);
                                c.sw(c.g[V0][0] as u32, c.g[S3][0]);
                                c.g[V1][0] = c.lw(apply_ea(c.g[S4][0], 0x118));
                                c.g[V0][0] = c.lw(apply_ea(c.g[S5][0], 0x1c));
                                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                c.g[A0][0] = c.lb(c.g[V0][0] as u32);
                                c.g[A2][0] = sx(apply_ea(c.g[A1][0], 0x1d8));
                                c.g[V0][0] = c.lw(apply_ea(c.g[A1][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[A2][0], 0x38));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[V1][0]);
                                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                c.g[A0][0] = c.lw(apply_ea(c.g[A1][0], 0x1f4));
                                c.g[V1][0] = c.lw(apply_ea(c.g[A2][0], 0x1c));
                                c.g[A1][0] = 0x18;
                                c.g[A3][0] = c.lw(apply_ea(c.g[A2][0], 0x38));
                                c.mult(0, 4, 4, 5);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[A1][0] = c.lw(apply_ea(c.g[A2][0], 0x20));
                                c.g[V1][0] = apply_sum(c.g[V1][0], c.g[A3][0]);
                                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
                                let t = c.g[V0][0] == 0;
                                c.g[A0][0] = apply_sum(c.g[A0][0], c.g[A1][0]);
                                if t {
                                    break 'b_280f00;
                                }
                                c.cycles += 6;
                                c.g[V0][0] = c.lw(apply_ea(c.g[A2][0], 0x1c));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] = apply_sum(c.g[V0][0], c.g[A3][0]);
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'b_280f2c;
                                }
                            }
                            c.cycles += 7;
                            c.g[V0][0] = c.lw(apply_ea(c.g[A2][0], 0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = apply_sum(c.g[V0][0], c.g[A3][0]);
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
                            if c.g[V1][0] == 0 {
                                c.g[V0][0] = 0x7f;
                                break 'b_280f2c;
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(apply_ea(c.g[A2][0], 0x1c));
                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                            c.g[V0][0] = apply_sum(c.g[V0][0], c.g[A3][0]);
                            c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                        }
                        c.cycles += 31;
                        c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                        c.g[A1][0] = 0x4;
                        c.g[A2][0] = 0;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[S0][0] = c.lw(apply_ea(c.g[S4][0], 0x118));
                        call_ptr(c, CHANNEL_GAIN, 0x0028_0f44)?;
                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = c.g[S4][0];
                        c.g[V1][0] = c.lw(apply_ea(c.g[A1][0], 0x1f4));
                        c.g[A2][0] = c.lw(apply_ea(c.g[A1][0], 0x214));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = apply_sum(c.g[V1][0], c.g[A2][0]);
                        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                        c.g[A1][0] = c.lw(apply_ea(c.g[S4][0], 0x118));
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[V0][0] = c.lw(apply_ea(c.g[V1][0], 0x1f4));
                        c.g[A1][0] = c.lw(apply_ea(c.g[V1][0], 0x23c));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V0][0] = apply_sum(c.g[V0][0], c.g[A1][0]);
                        c.sw(c.g[V0][0] as u32, 0);
                        call_ptr(c, PLAY_PUSHED_SOUND, 0x0028_0f80)?;
                        c.g[A3][0] = 0xc;
                        c.g[T0][0] = c.lw(apply_ea(c.g[SP][0], 0x38));
                        c.mult(0, 3, 23, 7);
                        c.g[A0][0] = apply_sum(c.g[T0][0], c.g[V1][0]);
                        c.g[A1][0] = apply_sum(c.g[FP][0], c.g[V1][0]);
                        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                        c.g[V1][0] = apply_sum(c.g[S6][0], c.g[V1][0]);
                        c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                        c.sw(apply_ea(c.g[V1][0], 0xc0), 0);
                    }
                    c.cycles += 3;
                    c.g[S5][0] = c.lw(apply_ea(c.g[S5][0], 0x20));
                    let t = c.g[S5][0] == 0;
                    frame.restore(c, S0, 0xe0);
                    if t {
                        break 'b_280fd0;
                    }
                    c.cycles += 6;
                    c.lwc1(0, apply_ea(c.g[S5][0], 0x4));
                    c.cvt_s_w(0, 0);
                    c.c_lt_s(0, 20);
                    if !c.fcc() {
                        c.g[S7][0] = c.lw(apply_ea(c.g[S5][0], 0xc));
                        if event_due(c) {
                            deliver_event_here(c, 0x0028_0c08);
                        }
                        continue 'l_280c08;
                    }
                    break 'b_280fcc;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0xe0);
        }

        // Epilogue.
        c.cycles += 12;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate: the fakes answer only when the machine holds `APPLY_MAGIC` at
    /// `APPLY_CTRL`; other tests run the bodies the fakes replaced.
    const APPLY_CTRL: u32 = DATA + 0x30_0000;
    const APPLY_MAGIC: u32 = 0x5eed_0552;
    const APPLY_THIS: u32 = DATA + 0x1_0000;
    const APPLY_OBJ: u32 = DATA + 0x2_0000;
    const APPLY_GAME: u32 = DATA + 0x3_0000;
    const APPLY_ENGINE: u32 = DATA + 0x4_0000;
    const APPLY_SECTION: u32 = DATA + 0x5_0000;
    const APPLY_WSCRIPT: u32 = DATA + 0x6_0000;
    const APPLY_INFO_SRC: u32 = APPLY_CTRL + 0x100;
    const APPLY_WORLD: u32 = APPLY_CTRL + 0x200;
    const APPLY_MUSIC: u32 = APPLY_CTRL + 0x400;
    const APPLY_SPEECH: u32 = APPLY_CTRL + 0x500;
    const APPLY_SOUNDS: u32 = APPLY_CTRL + 0x600;
    const APPLY_HEADER: u32 = APPLY_CTRL + 0x800;
    const APPLY_HASHES: u32 = APPLY_CTRL + 0x900;
    /// `D_004A525C`: the abort hash that means no script.
    const APPLY_NO_SCRIPT: u32 = 0x1234;

    /// A gated fake callee: logs the call (`fakes::fake_call`) when the
    /// machine carries the gate, else runs the body it replaced.
    macro_rules! apply_fake {
        ($fake:ident, $prev:ident, $gen:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(APPLY_CTRL) != APPLY_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
    }

    apply_fake!(
        apply_get_be,
        APPLY_PREV_GET_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );
    apply_fake!(
        apply_get_interface,
        APPLY_PREV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );
    apply_fake!(
        apply_script_data_info,
        APPLY_PREV_SCRIPT_INFO,
        ee::s07::f_0027c070__generated,
        0x0027_c070
    );
    apply_fake!(
        apply_engine_section,
        APPLY_PREV_ENGINE_SECTION,
        ee::s07::f_0027a688__generated,
        0x0027_a688
    );
    apply_fake!(
        apply_track_info,
        APPLY_PREV_TRACK_INFO,
        ee::s01::f_00144bc0__generated,
        0x0014_4bc0
    );
    apply_fake!(
        apply_section_owner,
        APPLY_PREV_SECTION_OWNER,
        ee::s05::f_0022e098__generated,
        0x0022_e098
    );
    apply_fake!(
        apply_wscript_call,
        APPLY_PREV_WSCRIPT,
        ee::s10::f_00309e50__generated,
        0x0030_9e50
    );
    apply_fake!(
        apply_audio_instance,
        APPLY_PREV_AUDIO,
        ee::s07::f_0028b180__generated,
        0x0028_b180
    );

    static APPLY_PREV_FRAME: AtomicUsize = AtomicUsize::new(0);

    /// `ScriptAudio_ReadFields` (the game-event frame): copies the fixture's
    /// `tSSXFrameGameEvent` (ten words) to `a1`, then logs the call.
    fn apply_frame_fake(c: &mut Cpu) {
        if c.bus.read32(APPLY_CTRL) != APPLY_MAGIC {
            return fakes::fall_back(c, &APPLY_PREV_FRAME, ee::s07::f_0027cc28__generated);
        }
        let out = c.g[A1][0] as u32;
        for k in 0..10u32 {
            let w = c.bus.read32(APPLY_INFO_SRC + 4 * k);
            c.bus.write32(out + 4 * k, w);
        }
        fakes::fake_call(c, 0x0027_cc28);
    }

    /// Hook the gated fakes on their override slots (once per process).
    fn apply_hook() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027CC28),
            apply_frame_fake,
            &APPLY_PREV_FRAME,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            apply_get_be,
            &APPLY_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            apply_get_interface,
            &APPLY_PREV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027C070),
            apply_script_data_info,
            &APPLY_PREV_SCRIPT_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027A688),
            apply_engine_section,
            &APPLY_PREV_ENGINE_SECTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
            apply_track_info,
            &APPLY_PREV_TRACK_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0022E098),
            apply_section_owner,
            &APPLY_PREV_SECTION_OWNER,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00309E50),
            apply_wscript_call,
            &APPLY_PREV_WSCRIPT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            apply_audio_instance,
            &APPLY_PREV_AUDIO,
        );
    }

    /// Two world events (hashes: no script, or one of two scripts), no music
    /// (its calls are not faked: music ids are 0xff, which are skipped),
    /// speech ids 16/17 (no call to the speech system), a sound list that
    /// stops at its first entry (its `when` is before the frame, so no sound
    /// is started), the flags and the four triggers at random. The game
    /// globals and the script header point into test data.
    #[test]
    fn script_audio_obj_apply_matches() {
        apply_hook();
        check(
            ee::s07::f_002808e8__generated,
            script_audio_obj_apply,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0028_08e8, 12);
                c.pc = 0x0028_08e8;
                c.bus.write32(APPLY_CTRL, APPLY_MAGIC);
                c.f[12] = 0;
                c.g[A0][0] = sx(APPLY_THIS);
                c.g[A1][0] = sx(APPLY_OBJ);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = (seed % 16) as u64;

                // Self: random words, then the pair list and the triggers.
                fill(c, &mut r, APPLY_THIS, 0x40);
                c.bus.write32(APPLY_THIS + 0xec, r.next_u32() % 4);
                for j in 0..4u32 {
                    let hash = [APPLY_NO_SCRIPT, 7, 9][(r.next_u32() % 3) as usize];
                    c.bus.write32(APPLY_THIS + 0x1c + 8 * j, hash);
                    c.bus.write32(APPLY_THIS + 0x20 + 8 * j, r.next_u32() % 3);
                }
                for k in 0..4u32 {
                    let t = APPLY_THIS + 0xbc + 0xc * k;
                    c.bus
                        .write32(t, [u32::MAX, 2, 5][(r.next_u32() % 3) as usize]);
                    c.bus.write32(t + 4, r.next_u32() % 2);
                    c.bus
                        .write32(t + 8, [2u32, 5, 9][(r.next_u32() % 3) as usize]);
                }

                // Globals: the no-script hash, the game, the script engine,
                // the section manager and the world-script manager.
                c.bus.write32(GAME_GP + 0x216c, APPLY_NO_SCRIPT);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), APPLY_GAME);
                c.bus.write32(GAME_GP.wrapping_sub(0x84c), APPLY_ENGINE);
                c.bus.write32(APPLY_GAME + 0x84, APPLY_SECTION);
                c.bus.write32(GAME_GP + 0xce8, APPLY_WSCRIPT);

                // The script header: its first byte picks the owner path.
                c.bus.write8(APPLY_HEADER, (seed % 2) as u8);
                c.bus.write32(fakes::LOG_RESULTS + 4 * 3, APPLY_HEADER);

                // The frame: counts and arrays.
                let world_count = r.next_u32() % 3;
                for k in 0..2u32 {
                    c.bus.write32(APPLY_WORLD + 8 * k, 3 + k);
                    c.bus.write32(APPLY_WORLD + 8 * k + 4, APPLY_HASHES + 4 * k);
                    let hash = [APPLY_NO_SCRIPT, 7, 9][(r.next_u32() % 3) as usize];
                    c.bus.write32(APPLY_HASHES + 4 * k, hash);
                }
                for k in 0..2u32 {
                    c.bus.write32(APPLY_MUSIC + 8 * k, 0xff);
                    let speech = if r.next_u32() % 2 == 0 { 16 } else { 17 };
                    c.bus.write32(APPLY_SPEECH + 8 * k, speech);
                }
                for k in 0..2u32 {
                    let e = APPLY_SOUNDS + 0x40 * k;
                    c.bus.write32(e, 3);
                    c.bus.write32(e + 4, (-5i32) as u32);
                    c.bus.write32(e + 0xc, k % 4);
                    c.bus.write32(e + 0x1c, APPLY_HEADER + 0x10);
                    c.bus
                        .write32(e + 0x20, if k == 0 { APPLY_SOUNDS + 0x40 } else { 0 });
                }
                let sound_list = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    APPLY_SOUNDS
                };
                let frame_words = [
                    world_count,
                    APPLY_WORLD,
                    r.next_u32() % 3,
                    APPLY_MUSIC,
                    r.next_u32() % 3,
                    APPLY_SPEECH,
                    0,
                    sound_list,
                    0,
                    0,
                ];
                for (k, w) in frame_words.iter().enumerate() {
                    c.bus.write32(APPLY_INFO_SRC + 4 * k as u32, *w);
                }
            },
        );
    }
}
