//! Staging module oc355 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc355`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0028_d988,
    ssx_audio_system_handle_music_trigger,
    "SSXAudioSystem_HandleMusicTrigger",
)];

// ---------------------------------------------------------------------------
// SSXAudioSystem_HandleMusicTrigger (0x28d988)

/// Trigger object: vtable pointer.
pub(crate) const HMT_TRIGGER_VT: u32 = 0x4;
/// Trigger vtable slot: `int* GetState()`.
pub(crate) const HMT_SLOT_STATE: u32 = 0x8;
/// Trigger vtable slot: `int* GetValue()` (the music cue).
pub(crate) const HMT_SLOT_VALUE: u32 = 0x10;
/// Audio system: custom-music flag (0 plays the default music for cue 0).
pub(crate) const HMT_CUSTOM_MUSIC: u32 = 0x530;
/// Audio system: music player sub-object; its vtable pointer is at 0x5558.
pub(crate) const HMT_MUSIC: u32 = 0x118;
pub(crate) const HMT_MUSIC_VT: u32 = 0x5558;
/// Audio system: a queued cue (flags and cue number).
pub(crate) const HMT_PENDING_ON: u32 = 0x5740;
pub(crate) const HMT_PENDING_FLAG2: u32 = 0x5750;
pub(crate) const HMT_PENDING_SET: u32 = 0x5758;
pub(crate) const HMT_PENDING_CUE: u32 = 0x575c;
/// Audio system: fade the cue (BankMonitor timer flag).
pub(crate) const HMT_FADE_MODE: u32 = 0x5790;
/// Audio system: last trigger state / value seen.
pub(crate) const HMT_LAST_STATE: u32 = 0x6264;
pub(crate) const HMT_LAST_VALUE: u32 = 0x6268;
/// Audio system: music may switch on the next cue.
pub(crate) const HMT_ARMED: u32 = 0x6270;
pub(crate) const HMT_PAUSED: u32 = 0x62b8;

const HMT_GET_INSTANCE: Callee = Callee {
    addr: 0x0028_b180,
    f: ee::s07::f_0028b180,
};
const HMT_CLEAR_TRIGGER: Callee = Callee {
    addr: 0x0028_d7d8,
    f: ee::s07::f_0028d7d8,
};
const HMT_FIND_RIDER: Callee = Callee {
    addr: 0x0028_5d98,
    f: ee::s07::f_00285d98,
};
const HMT_PLAY_MUSIC: Callee = Callee {
    addr: 0x0028_cf98,
    f: ee::s07::f_0028cf98,
};
const HMT_SET_MUSIC_STATE: Callee = Callee {
    addr: 0x0028_def0,
    f: ee::s07::f_0028def0,
};
const HMT_START_NEXT_SONG: Callee = Callee {
    addr: 0x0028_df18,
    f: ee::s07::f_0028df18,
};
const HMT_START_TRIGGER: Callee = Callee {
    addr: 0x0028_d630,
    f: ee::s07::f_0028d630,
};
const HMT_CONFIRM_TRIGGER: Callee = Callee {
    addr: 0x0028_d740,
    f: ee::s07::f_0028d740,
};
/// `SSXAudioSystem_IsRaceType6to11` (cross-shard: through the table).
const HMT_IS_RACE_TYPE_6_TO_11: u32 = 0x002a_42c8;
/// `SSXAudioSystem_IsEventMode4`.
const HMT_IS_EVENT_MODE_4: u32 = 0x002a_4040;
/// `BankMonitor_AddTimer`.
const HMT_ADD_TIMER: u32 = 0x002a_dca0;

/// The audio system (s0).
fn hmt_self(c: &Cpu) -> Addr {
    Addr(c.g[S0][0] as u32)
}

/// self + 0x118 (the music player), as `addiu` leaves it.
fn hmt_music(c: &Cpu) -> u64 {
    sx((c.g[S0][0] as u32).wrapping_add(HMT_MUSIC))
}

/// Call the trigger object's (s1) vtable method at `slot` (`{delta, fn}`),
/// loading the vtable into `vt` (v1, or a1 at the end), returning to `ret`.
fn hmt_trigger_call(c: &mut Cpu, vt: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let obj = c.g[S1][0] as u32;
    c.g[vt][0] = Addr(obj).off(HMT_TRIGGER_VT).lw(c);
    let table = c.g[vt][0] as u32;
    c.g[A0][0] = Addr(table).off(slot).lh(c);
    c.g[V0][0] = Addr(table).off(slot + 4).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// Call the music player's method at `slot` of the vtable in v1 (a2 =
/// self+0x118 and a1 already set).
fn hmt_music_call(c: &mut Cpu, slot: u32, ret: u32) -> crate::call::Flow {
    let table = c.g[V1][0] as u32;
    c.g[A0][0] = Addr(table).off(slot).lh(c);
    c.g[V0][0] = Addr(table).off(slot + 4).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `BankMonitor_AddTimer(*music + 0x1d8, 1500, k(0x4a3650), 0, fade, 0, 0,
/// 0)` with v0 = the music pointer (self+0x118, already loaded), then
/// disarm.
fn hmt_fade_timer(c: &mut Cpu, fade: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 14;
    c.g[V1][0] = 0x4a_0000;
    c.ldl(A2, 0x004a_3657);
    c.ldr(A2, 0x004a_3650);
    c.g[A1][0] = 0x5dc;
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A3][0] = 0;
    c.g[T0][0] = fade;
    c.g[T1][0] = 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    c.g[T2][0] = 0;
    c.g[T3][0] = 0;
    call_ptr(c, HMT_ADD_TIMER, ret)?;
    hmt_self(c).off(HMT_ARMED).sw(c, 0);
    Ok(())
}

/// How the trigger compares with the last state/value seen.
#[derive(Clone, Copy, PartialEq, Eq)]
enum HmtCmp {
    StateDiffers,
    ValueDiffers,
    Same,
}

/// Compare the trigger with what was seen last: loads the state (call
/// returning to `ret_state`) and compares it with self+0x6264, then (only if
/// equal; 9 more cycles) the value with self+0x6268. Leaves v0/v1 as the
/// last compare does.
fn hmt_compare_last(
    c: &mut Cpu,
    ret_state: u32,
    ret_value: u32,
) -> Result<HmtCmp, crate::call::Unwound> {
    hmt_trigger_call(c, V1, HMT_SLOT_STATE, ret_state)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = hmt_self(c).off(HMT_LAST_STATE).lw(c);
    if c.g[V0][0] != c.g[V1][0] {
        return Ok(HmtCmp::StateDiffers);
    }
    c.cycles += 9;
    hmt_trigger_call(c, V1, HMT_SLOT_VALUE, ret_value)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = hmt_self(c).off(HMT_LAST_VALUE).lw(c);
    Ok(if c.g[V0][0] == c.g[V1][0] {
        HmtCmp::Same
    } else {
        HmtCmp::ValueDiffers
    })
}

/// The epilogue (`extra`: the 1-cycle block before the `lq s0` on the
/// normal path).
fn hmt_exit(c: &mut Cpu, frame: Frame, extra: u64) -> crate::call::Flow {
    c.cycles += extra;
    frame.restore(c, S0, 0x40);
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SSXAudioSystem_HandleMusicTrigger(SSXAudioSystem* self, Rider* rider,
/// MusicTrigger* trigger)` (0x28d988): react to a course music trigger.
///
/// The trigger's state (vtable getter at 0x8) selects: 1 starts a music
/// trigger, 2 confirms one, -1 clears it (when the last state was 1 or 2) and
/// re-arms the music; 0 (only for the rider the audio follows) applies the
/// trigger's music cue (getter at 0x10): 0 plays the default music or sends
/// cue 0, 0xb (race end) and 0x12 (event mode 4) switch the music state and
/// start the next song or schedule timers, 0xd and 0xe..0x11 switch state
/// and queue the cue, 0x2a sends a one-shot cue once, others go straight to
/// the music player. Unless the clear call returned 0, the trigger's state
/// and value are remembered at 0x6264/0x6268. Frame 0x50 (ra at 0, s3..s0
/// at 0x10..0x40).
pub fn ssx_audio_system_handle_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        if c.g[S1][0] == 0 {
            return hmt_exit(c, frame, 1);
        }
        'done: {
            c.cycles += 9;
            c.g[S3][0] = 1;
            hmt_trigger_call(c, V1, HMT_SLOT_STATE, 0x0028_d9c8)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            let state = c.g[V1][0];
            if state != 0 {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if (state as i64) > 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    if state == c.g[S3][0] {
                        // State 1: start the trigger's cue.
                        c.cycles += 11;
                        hmt_trigger_call(c, V1, HMT_SLOT_VALUE, 0x0028_de5c)?;
                        c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A0][0] = c.g[S0][0];
                        HMT_START_TRIGGER.call(c, 0x0028_de6c)?;
                        break 'done;
                    }
                    c.cycles += 2;
                    if state != 2 {
                        c.cycles += 2;
                        break 'done;
                    }
                    // State 2: confirm it.
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).off(HMT_TRIGGER_VT).lw(c);
                    c.cycles += 8;
                    hmt_trigger_call(c, V1, HMT_SLOT_VALUE, 0x0028_de84)?;
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    HMT_CONFIRM_TRIGGER.call(c, 0x0028_de94)?;
                    break 'done;
                }
                c.cycles += 2;
                if state != u64::MAX {
                    c.cycles += 2;
                    break 'done;
                }
                // State -1: clear a started/confirmed trigger, re-arm.
                c.g[V0][0] = hmt_self(c).off(HMT_LAST_STATE).lw(c);
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                let was_active = c.g[V0][0] < 2;
                c.g[V0][0] = 1;
                if was_active {
                    c.cycles += 7;
                    HMT_GET_INSTANCE.call(c, 0x0028_da1c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S2][0];
                    HMT_CLEAR_TRIGGER.call(c, 0x0028_da28)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = 1;
                }
                c.cycles += 2;
                hmt_self(c).off(HMT_ARMED).sw(c, c.g[V0][0]);
                break 'done;
            }
            // State 0: only for the rider the audio follows.
            c.g[A0][0] = c.g[S0][0];
            c.cycles += 4;
            c.g[A1][0] = u64::MAX;
            HMT_FIND_RIDER.call(c, 0x0028_da40)?;
            if c.g[S2][0] != c.g[V0][0] {
                return hmt_exit(c, frame, 0);
            }
            c.cycles += 8;
            hmt_trigger_call(c, V1, HMT_SLOT_VALUE, 0x0028_da5c)?;
            c.g[S2][0] = Addr(c.g[V0][0] as u32).lw(c);
            let cue = c.g[S2][0];
            c.g[V0][0] = 0xb;
            // Ends in the music player's slot 0x20 call (0x28de30; v1 =
            // vtable, a2 = player, a1 = cue).
            'music: {
                if cue == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = hmt_self(c).off(HMT_CUSTOM_MUSIC).lw(c);
                    c.g[A2][0] = hmt_music(c);
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = u64::MAX;
                        c.g[T0][0] = 0;
                        HMT_PLAY_MUSIC.call(c, 0x0028_da8c)?;
                        break 'done;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                    c.g[A1][0] = 0;
                    break 'music;
                }
                c.cycles += 2;
                c.g[V0][0] = 0xd;
                // Ends in the state switch shared by cues 0xb and 0x12
                // (0x28dd4c; a0 = self, a1 = cue).
                'switch: {
                    if cue == 0xb {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dab0)?;
                        let race = c.g[V0][0] != 0;
                        c.g[V0][0] = hmt_self(c).off(HMT_ARMED).lw(c);
                        if race {
                            c.cycles += 2;
                            c.g[A1][0] = 0xb;
                            if c.g[V0][0] == 0 {
                                break 'done;
                            }
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            break 'switch;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xb;
                        HMT_SET_MUSIC_STATE.call(c, 0x0028_dacc)?;
                        let ok = c.g[V0][0] != 0;
                        c.g[A2][0] = hmt_music(c);
                        if !ok {
                            break 'done;
                        }
                        c.cycles += 30;
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                        c.g[A1][0] = 0xb;
                        hmt_music_call(c, 0x20, 0x0028_daec)?;
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC).lw(c);
                        c.g[V0][0] = 0x4a_0000;
                        c.g[T0][0] = hmt_self(c).off(HMT_PAUSED).lw(c);
                        c.g[T1][0] = 0;
                        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[A1][0] = 0x7d0;
                        if c.g[T0][0] != 0 {
                            c.g[T1][0] = c.g[S3][0];
                        }
                        c.ldl(A2, 0x004a_364f);
                        c.ldr(A2, 0x004a_3648);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        c.g[A3][0] = 0;
                        c.g[T0][0] = 0;
                        c.g[T2][0] = 0;
                        c.g[T3][0] = 0;
                        call_ptr(c, HMT_ADD_TIMER, 0x0028_db28)?;
                        match hmt_compare_last(c, 0x0028_db3c, 0x0028_db60)? {
                            HmtCmp::Same => {
                                hmt_self(c).off(HMT_ARMED).sw(c, 0);
                                break 'done;
                            }
                            HmtCmp::ValueDiffers => c.cycles += 1,
                            HmtCmp::StateDiffers => {}
                        }
                        // 0x28db74: a 1.5 s timer, fading or not.
                        let fade = hmt_self(c).off(HMT_FADE_MODE).lw(c);
                        c.g[V0][0] = fade;
                        c.cycles += 2;
                        c.g[V0][0] = hmt_self(c).off(HMT_MUSIC).lw(c);
                        if fade == 0 {
                            hmt_fade_timer(c, 0, 0x0028_dbac)?;
                        } else {
                            hmt_fade_timer(c, 1, 0x0028_dbe4)?;
                        }
                        break 'done;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((cue as u32).wrapping_sub(0xe));
                    if cue == 0xd {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xd;
                        HMT_SET_MUSIC_STATE.call(c, 0x0028_dc10)?;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 9;
                        if hmt_compare_last(c, 0x0028_dc2c, 0x0028_dc50)? != HmtCmp::Same {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dc68)?;
                            if c.g[V0][0] == 0 {
                                hmt_self(c).off(HMT_PENDING_FLAG2).sw(c, c.g[S3][0]);
                            }
                            c.cycles += 1;
                        }
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                        c.cycles += 3;
                        c.g[A2][0] = hmt_music(c);
                        c.g[A1][0] = 0xd;
                        break 'music;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                    let mut queued = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !queued {
                        c.cycles += 4;
                        c.g[V0][0] = sx((cue as u32).wrapping_sub(0x10));
                        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                        queued = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x12;
                    }
                    if queued {
                        // Cues 0xe..0x11: switch state and queue the cue.
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S2][0];
                        HMT_SET_MUSIC_STATE.call(c, 0x0028_dca4)?;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 9;
                        if hmt_compare_last(c, 0x0028_dcc0, 0x0028_dce4)? == HmtCmp::Same {
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dcfc)?;
                        if c.g[V0][0] != 0 {
                            break 'done;
                        }
                        c.cycles += 4;
                        hmt_self(c).off(HMT_PENDING_CUE).sw(c, c.g[S2][0]);
                        hmt_self(c).off(HMT_PENDING_SET).sw(c, c.g[S3][0]);
                        hmt_self(c).off(HMT_PENDING_ON).sw(c, c.g[S3][0]);
                        break 'done;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x2a;
                    if cue == 0x12 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_EVENT_MODE_4, 0x0028_dd24)?;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, HMT_IS_RACE_TYPE_6_TO_11, 0x0028_dd34)?;
                        if c.g[V0][0] != 0 {
                            break 'done;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = hmt_self(c).off(HMT_ARMED).lw(c);
                        c.g[A1][0] = 0x12;
                        if c.g[V0][0] == 0 {
                            break 'done;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                        break 'switch;
                    }
                    c.cycles += 2;
                    if cue != 0x2a {
                        c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                        c.cycles += 2;
                        c.g[A2][0] = hmt_music(c);
                        c.g[A1][0] = c.g[S2][0];
                        break 'music;
                    }
                    // Cue 0x2a: a one-shot cue (slot 0x18) when it changed.
                    c.cycles += 9;
                    match hmt_compare_last(c, 0x0028_ddd0, 0x0028_ddf4)? {
                        HmtCmp::Same => break 'done,
                        HmtCmp::ValueDiffers => c.cycles += 1,
                        HmtCmp::StateDiffers => {}
                    }
                    c.g[V1][0] = hmt_self(c).off(HMT_MUSIC_VT).lw(c);
                    c.cycles += 8;
                    c.g[A2][0] = hmt_music(c);
                    c.g[A1][0] = 0x2a;
                    hmt_music_call(c, 0x18, 0x0028_de20)?;
                    break 'done;
                }
                // 0x28dd4c: switch the music state; start the next song if
                // the trigger changed; disarm.
                c.cycles += 4;
                HMT_SET_MUSIC_STATE.call(c, 0x0028_dd54)?;
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                c.cycles += 9;
                let same = hmt_compare_last(c, 0x0028_dd70, 0x0028_dd94)? == HmtCmp::Same;
                c.g[A0][0] = c.g[S0][0];
                if !same {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    HMT_START_NEXT_SONG.call(c, 0x0028_ddac)?;
                }
                c.cycles += 2;
                hmt_self(c).off(HMT_ARMED).sw(c, 0);
                break 'done;
            }
            // 0x28de30: the music player's slot 0x20 with the cue.
            c.cycles += 6;
            hmt_music_call(c, 0x20, 0x0028_de40)?;
        }
        // 0x28de94: remember the trigger's state and value.
        c.cycles += 2;
        if c.g[S3][0] == 0 {
            return hmt_exit(c, frame, 0);
        }
        c.cycles += 14;
        hmt_trigger_call(c, V1, HMT_SLOT_STATE, 0x0028_deb0)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        hmt_self(c).off(HMT_LAST_STATE).sw(c, c.g[V1][0]);
        hmt_trigger_call(c, A1, HMT_SLOT_VALUE, 0x0028_decc)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        hmt_self(c).off(HMT_LAST_VALUE).sw(c, c.g[V1][0]);
        hmt_exit(c, frame, 1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------- SSXAudioSystem_HandleMusicTrigger

    use crate::difftest::fakes::{covered, fake, fall_back, hook, machine, FAKE};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the `HandleMusicTrigger` test fakes (in each test machine's
    /// own RAM), followed by the result each faked callee returns.
    const HMT_TEST_CTRL: u32 = DATA + 0x0f_3550;
    const HMT_TEST_MAGIC: u32 = 0x484d_5452;
    const HMT_TEST_RESULTS: u32 = HMT_TEST_CTRL + 4;
    /// Words the trigger's getters return pointers to.
    const HMT_TEST_STATE: u32 = DATA + 0x500;
    const HMT_TEST_VALUE: u32 = DATA + 0x510;
    /// Fake methods: the trigger's getters, the music player, the rider.
    const HMT_TEST_GET_STATE: u32 = FAKE + 0x100;
    const HMT_TEST_GET_VALUE: u32 = FAKE + 0x110;

    /// A gated fake for a callee of `HandleMusicTrigger`: logs the call and
    /// returns the result the fixture chose (`id`), or unwinds.
    macro_rules! hmt_fake {
        ($name:ident, $prev:ident, $addr:expr, $gen:path, $id:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(HMT_TEST_CTRL) != HMT_TEST_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                let ra = c.g[RA][0] as u32;
                fake(c, $addr, ra);
                if c.pc == ra {
                    c.g[V0][0] = sx(c.bus.read32(HMT_TEST_RESULTS + 4 * $id));
                }
            }
        };
    }

    hmt_fake!(
        hmt_fake_set_state,
        HMT_PREV_SET_STATE,
        0x0028_def0,
        ee::s07::f_0028def0__generated,
        0
    );
    hmt_fake!(
        hmt_fake_confirm,
        HMT_PREV_CONFIRM,
        0x0028_d740,
        ee::s07::f_0028d740__generated,
        1
    );
    hmt_fake!(
        hmt_fake_race_6_11,
        HMT_PREV_RACE_6_11,
        0x002a_42c8,
        ee::s08::f_002a42c8__generated,
        2
    );
    hmt_fake!(
        hmt_fake_event_4,
        HMT_PREV_EVENT_4,
        0x002a_4040,
        ee::s08::f_002a4040__generated,
        3
    );
    hmt_fake!(
        hmt_fake_add_timer,
        HMT_PREV_ADD_TIMER,
        0x002a_dca0,
        ee::s08::f_002adca0__generated,
        4
    );
    hmt_fake!(
        hmt_fake_2a10c0,
        HMT_PREV_2A10C0,
        0x002a_10c0,
        ee::s08::f_002a10c0__generated,
        5
    );

    /// The `miss` hook: the trigger getters return the fixture's words.
    fn hmt_test_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fake(c, target, ra);
        if c.pc == ra {
            match target {
                HMT_TEST_GET_STATE => c.g[V0][0] = sx(HMT_TEST_STATE),
                HMT_TEST_GET_VALUE => c.g[V0][0] = sx(HMT_TEST_VALUE),
                _ => {}
            }
        }
    }

    /// A vtable at `vt` with `{delta, fn}` slots.
    fn hmt_test_vtable(c: &mut Cpu, r: &mut crate::difftest::Rng, vt: u32, slots: &[(u32, u32)]) {
        for &(off, f) in slots {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, f);
        }
    }

    /// `HandleMusicTrigger` with the music-state callees faked and the
    /// small ones real (GetInstance, FindRider, ClearMusicTrigger,
    /// StartMusicTrigger, PlayMusic and StartNextSong on short paths): no
    /// trigger, every trigger state (1, 2, -1 after 1/2 or not, other
    /// positive and negative), and for state 0 another rider or the followed
    /// one with every cue (0 with and without custom music, 0xb in and out of
    /// race types 6-11, 0xd, 0xe..0x11, 0x12, 0x2a, others), the results of
    /// the state switch and mode checks, unchanged and changed triggers, and
    /// unwinds at every faked call.
    #[test]
    fn ssx_audio_system_handle_music_trigger_matches() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028DEF0),
            hmt_fake_set_state,
            &HMT_PREV_SET_STATE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028D740),
            hmt_fake_confirm,
            &HMT_PREV_CONFIRM,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A42C8),
            hmt_fake_race_6_11,
            &HMT_PREV_RACE_6_11,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A4040),
            hmt_fake_event_4,
            &HMT_PREV_EVENT_4,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            hmt_fake_add_timer,
            &HMT_PREV_ADD_TIMER,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A10C0),
            hmt_fake_2a10c0,
            &HMT_PREV_2A10C0,
        );
        covered(
            ee::s07::f_0028d988__generated,
            ssx_audio_system_handle_music_trigger,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_d988, 6);
                c.hooks.miss = hmt_test_miss;
                c.pc = 0x0028_d988;
                c.bus.write32(HMT_TEST_CTRL, HMT_TEST_MAGIC);
                let pick = |r: &mut crate::difftest::Rng, set: &[u32]| {
                    set[(r.next_u32() as usize) % set.len()]
                };

                let audio = DATA + 0x10000;
                let trigger = DATA + 0x1000;
                let trigger_vt = DATA + 0x1100;
                let music_vt = DATA + 0x1200;
                let rider_vt = DATA + 0x1300;
                let rider = if seed % 8 == 7 { 0 } else { DATA + 0x2000 };
                let music = DATA + 0x3000;
                let world = DATA + 0x4000;
                let module = DATA + 0x4100;
                let race_man = DATA + 0x4200;
                let replay = DATA + 0x4300;
                let followed = DATA + 0x4400;

                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = if seed % 16 == 15 { 0 } else { sx(trigger) };

                // The trigger: state and value getters.
                c.bus.write32(trigger + HMT_TRIGGER_VT, trigger_vt);
                hmt_test_vtable(
                    c,
                    &mut r,
                    trigger_vt,
                    &[
                        (HMT_SLOT_STATE, HMT_TEST_GET_STATE),
                        (HMT_SLOT_VALUE, HMT_TEST_GET_VALUE),
                    ],
                );
                let state = match seed % 8 {
                    0..=3 => 0,
                    4 => pick(&mut r, &[1, 1, 2, 2, 3, 7]),
                    _ => pick(&mut r, &[u32::MAX, u32::MAX, u32::MAX, (-2i32) as u32]),
                };
                let cue = pick(
                    &mut r,
                    &[
                        0, 0, 0xb, 0xb, 0xb, 0xd, 0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x12, 0x12,
                        0x2a, 0x2a, 5, 0x13,
                    ],
                );
                c.bus.write32(HMT_TEST_STATE, state);
                c.bus.write32(HMT_TEST_VALUE, cue);
                let last_state = match r.next_u32() % 4 {
                    0 | 1 => state,
                    2 => pick(&mut r, &[1, 2]),
                    _ => pick(&mut r, &[0, 3, u32::MAX]),
                };
                let last_value = if r.next_u32() % 2 == 0 {
                    cue
                } else {
                    cue ^ 0x40
                };
                c.bus.write32(audio + HMT_LAST_STATE, last_state);
                c.bus.write32(audio + HMT_LAST_VALUE, last_value);

                // The audio system.
                let flag = |r: &mut crate::difftest::Rng| if r.next_u32() % 2 == 0 { 0 } else { 1 };
                c.bus.write32(GAME_GP + 0x410, audio);
                c.bus.write32(audio + HMT_MUSIC, music);
                c.bus.write32(music, r.next_u32());
                c.bus.write32(audio + HMT_MUSIC_VT, music_vt);
                hmt_test_vtable(
                    c,
                    &mut r,
                    music_vt,
                    &[(0x18, FAKE + 0x200), (0x20, FAKE + 0x210)],
                );
                for off in [
                    HMT_CUSTOM_MUSIC,
                    HMT_ARMED,
                    HMT_FADE_MODE,
                    HMT_PAUSED,
                    0x5fd4,
                    0x5fd8,
                ] {
                    let v = flag(&mut r);
                    c.bus.write32(audio + off, v);
                }
                // StartMusicTrigger / ClearMusicTrigger: current cue, active rider.
                let current = pick(&mut r, &[0, 3, cue]);
                c.bus.write32(audio + 0x623c, current);
                let other = r.next_u32();
                let active = pick(&mut r, &[u32::MAX, u32::MAX, other]);
                c.bus.write32(audio + 0x62a0, active);
                c.bus.write32(audio + 0x6294, 0);
                if rider != 0 {
                    c.bus.write32(rider + 0x6c0, rider_vt);
                    hmt_test_vtable(c, &mut r, rider_vt, &[(0x38, FAKE + 0x300)]);
                }

                // FindRider: a replay in progress, followed rider = this one or not.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, module);
                c.bus.write32(module + 0x28, module + 0x80);
                c.bus.write32(module + 0x80, 1 + r.next_u32() % 9);
                c.bus.write32(module + 0xc, race_man);
                c.bus.write32(race_man + 0x40, replay);
                let found = if r.next_u32() % 4 == 0 {
                    followed
                } else {
                    rider
                };
                c.bus.write32(replay + 0x18, found);

                // Faked results: SetMusicState, ConfirmMusicTrigger,
                // IsRaceType6to11, IsEventMode4, AddTimer, 0x2a10c0.
                let set_state = if r.next_u32() % 4 == 0 { 0 } else { 1 };
                let race = flag(&mut r);
                let event4 = if r.next_u32() % 4 == 0 { 0 } else { 1 };
                let results = [set_state, r.next_u32(), race, event4, r.next_u32(), 1];
                for (i, v) in results.into_iter().enumerate() {
                    c.bus.write32(HMT_TEST_RESULTS + 4 * i as u32, v);
                }
            },
        );
    }
}
