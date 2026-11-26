//! Staging module oc771 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc771`.

//! Not ported (resumable `match pc` dispatcher, left to the orchestrator):
//! `SSXAudioSystem_LoadVolumeConfig` (0x287fc8).

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
        0x0018_2c80,
        c_fe_state_char_setup_on_widget_action,
        "cFEStateCharSetup_OnWidgetAction",
    ),
    (
        0x002b_65b0,
        world_trigger_manager_get_trigger_name,
        "WorldTriggerManager_GetTriggerName",
    ),
    (
        0x0029_5950,
        ssx_audio_system_set_tracked_sound_expiry,
        "SSXAudioSystem_SetTrackedSoundExpiry",
    ),
    (
        0x0018_6f90,
        c_fe_state_mem_card_check_free_space,
        "cFEStateMemCard_CheckFreeSpace",
    ),
];

/// Offset of the engine pointer in a UI state.
pub(crate) const CHAR_SETUP_ENGINE: u32 = 0x10;
/// Offset of the state stack inside the UI engine.
pub(crate) const CHAR_SETUP_STATE_STACK: u32 = 0x18;

/// The shared epilogue of `cFEStateCharSetup_OnWidgetAction`: 3 cycles, `ra`
/// reloaded, frame popped.
fn char_setup_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStateCharSetup_OnWidgetAction(State* self, Item* item, int event)`
/// (0x182c80): the character-setup menu's widget handler. Event 5 / 6 ask the
/// UI factory for the next / previous state (vtable +0x20 / +0x28) and push
/// it on the engine's state stack, then hide the rider preview
/// (`SetRiderPreviewActive(self, 0)`; event 6 only when a state came back);
/// event 7 builds a `cFEStateOptions` (84 bytes, mode bits set to 1) and pushes
/// it immediately. Other events, or a null item, do nothing.
pub fn c_fe_state_char_setup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        'done: {
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 6;
            let is6 = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[A2][0] < 7u64) as u64;
            if is6 {
                // Event 6: previous state; a state is pushed (and the preview
                // hidden) only when the factory returned one.
                c.cycles += 11;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018_2d30)?;
                c.g[A1][0] = c.g[V0][0];
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CHAR_SETUP_STATE_STACK));
                call_ptr(c, 0x0039_f400, 0x0018_2d44)?;
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 2;
                let ge7 = c.g[V0][0] == 0;
                c.g[V0][0] = 0x5;
                if !ge7 {
                    c.cycles += 2;
                    if c.g[A2][0] != c.g[V0][0] {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        char_setup_exit(c, frame);
                        return Ok(());
                    }
                    // Event 5: next state, pushed when there is one; the
                    // preview is hidden either way.
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018_2cf0)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S0][0];
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CHAR_SETUP_STATE_STACK));
                        call_ptr(c, 0x0039_f400, 0x0018_2d04)?;
                        c.g[A0][0] = c.g[S0][0];
                    }
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x7;
                    let is7 = c.g[A2][0] == c.g[V0][0];
                    c.g[A1][0] = 0x46_0000;
                    if !is7 {
                        c.cycles += 2;
                        frame.restore(c, S0, 0x10);
                        char_setup_exit(c, frame);
                        return Ok(());
                    }
                    // Event 7: a new options state, pushed immediately.
                    c.cycles += 20;
                    c.g[A3][0] = 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                    c.g[A0][0] = 0x54;
                    c.g[A2][0] = 0x100;
                    call_ptr(c, 0x0031_7d70, 0x0018_2d6c)?; // alloc(84, "FEOptions", 0x100, 0)
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 1;
                    call_ptr(c, 0x0018_87a0, 0x0018_2d7c)?; // cFEStateOptions_ctor(mem, engine, 1)
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.g[A0][0] = 0xffff_ffff_ffff_c0ff;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = 0;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[A3][0] = 0;
                    c.g[V1][0] |= 0x100;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(CHAR_SETUP_ENGINE));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(CHAR_SETUP_STATE_STACK));
                    call_ptr(c, 0x0039_f320, 0x0018_2da8)?; // pushSpecial(stack, st, 0, 0)
                    break 'done;
                }
            }
            // Hide the rider preview, then leave.
            c.cycles += 4;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0018_31d0, 0x0018_2d50)?;
            frame.restore(c, S0, 0x10);
            char_setup_exit(c, frame);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        char_setup_exit(c, frame);
        Ok(())
    })
}

/// Offsets in the world trigger manager read by `GetTriggerName`.
pub(crate) const WTM_TRIGGER_LIST: u32 = 0x33c;
pub(crate) const WTM_TRIGGER_COUNT: u32 = 0x340;

/// `int WorldTriggerManager_GetTriggerName(Manager* m, int idx, char* dst)`
/// (0x2b65b0): the name of trigger `idx`. A type-3 trigger (8-byte entries
/// `{type, data}`) copies the string at `data + 0x10` to `dst` and returns 1;
/// a type-4 one asks `GetSpecialTriggerName` (0x2b6628, which returns 0);
/// anything else, or an index past the count, gives 0.
pub fn world_trigger_manager_get_trigger_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(WTM_TRIGGER_COUNT));
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 3);
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(WTM_TRIGGER_LIST));
            c.g[A0][0] = 0x3;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = c.lw(c.g[V1][0] as u32);
            let not3 = c.g[A3][0] != c.g[A0][0];
            c.g[V0][0] = 0x4;
            if !not3 {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x0041_66f4, 0x002b_65f4)?; // strcpy(dst, data + 0x10)
                c.g[V0][0] = 0x1;
            } else {
                c.cycles += 2;
                let not4 = c.g[A3][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if !not4 {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[T0][0];
                    Callee {
                        addr: 0x002b_6628,
                        f: ee::s08::f_002b6628,
                    }
                    .call(c, 0x002b_660c)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
            }
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Offsets of the tracked-sound history ring in the audio system.
pub(crate) const AUDIO_RING_SIZE: u32 = 0x597c;
#[cfg(test)]
pub(crate) const AUDIO_RING_BUF: u32 = 0x5980;
pub(crate) const AUDIO_RING_WRITE: u32 = 0x5984;
pub(crate) const AUDIO_RING_READ: u32 = 0x5988;

/// Read slot `R` of the ring and advance `R` (wrapping to 0 at the size). The
/// first pop loads `v0 = R` itself; the others had it loaded in a branch
/// delay slot.
fn audio_ring_advance(c: &mut Cpu, cycles: u64, load_v0: bool) {
    let t0 = c.g[T0][0] as u32;
    let t1 = c.g[T1][0] as u32;
    c.cycles += cycles;
    if load_v0 {
        c.g[V0][0] = c.lw(t1.wrapping_add(0xc));
    }
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
    c.g[A0][0] = c.lw(t1.wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(t0.wrapping_add(AUDIO_RING_SIZE));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let in_range = c.g[V1][0] != 0;
    c.sw(t0.wrapping_add(AUDIO_RING_READ), c.g[A1][0]);
    if !in_range {
        c.cycles += 1;
        c.sw(t0.wrapping_add(AUDIO_RING_READ), 0);
    }
}

/// Pops into `out` (T5, T2, T6) when the ring is not empty, else -1.
fn audio_ring_pop_mid(c: &mut Cpu, out: usize) {
    let t0 = c.g[T0][0] as u32;
    let t1 = c.g[T1][0] as u32;
    c.cycles += 2;
    if c.g[A0][0] != c.g[A1][0] {
        c.g[V0][0] = c.lw(t1.wrapping_add(0xc));
        audio_ring_advance(c, 9, false);
        c.cycles += 3;
        c.g[A0][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
        c.g[out][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(t0.wrapping_add(AUDIO_RING_READ));
    } else {
        c.cycles += 2;
        c.g[out][0] = 0xffff_ffff_ffff_ffff;
    }
}

/// Pushes `val` at `W` and advances `W` unless the ring is full (`(W+1) %
/// size == R`). `last` is the fifth push, which ends with the loop counter
/// decrement instead of reloading `W`.
fn audio_ring_push(c: &mut Cpu, val: usize, last: bool) {
    let t0 = c.g[T0][0] as u32;
    let t1 = c.g[T1][0] as u32;
    c.cycles += 5;
    c.g[V1][0] = c.lw(t0.wrapping_add(AUDIO_RING_SIZE));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.div(0, 2, 3);
    c.cycles += 4;
    c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_READ));
    c.g[A0][0] = c.hi[0];
    if c.g[A0][0] == c.g[V0][0] {
        if last {
            c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(1));
        } else {
            c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
        }
        return;
    }
    c.cycles += 11;
    c.g[V0][0] = c.lw(t1.wrapping_add(0x8));
    c.g[V1][0] = c.lw(t1.wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[val][0]);
    c.g[V1][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
    c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_SIZE));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let in_range = c.g[V0][0] != 0;
    c.sw(t0.wrapping_add(AUDIO_RING_WRITE), c.g[V1][0]);
    if !in_range {
        c.cycles += 1;
        c.sw(t0.wrapping_add(AUDIO_RING_WRITE), 0);
    }
    c.cycles += 1;
    if last {
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(1));
    } else {
        c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
    }
}

/// `void SSXAudioSystem_SetTrackedSoundExpiry(Audio* a, void* hist, int
/// idx, int time)` (0x295950, `cSSXAudio_PlayHistoryUpdateTime`): walks the
/// tracked-sound ring once (`count = (W - R + size) % size` records of five
/// words `{hist, handle, time, looping, hull}`): every record is popped and
/// pushed back, the `time` word replaced by the new time when its history
/// pointer is `hist` and its hull equals `idx`.
pub fn ssx_audio_system_set_tracked_sound_expiry(c: &mut Cpu) {
    c.cycles += 9;
    c.g[T0][0] = c.g[A0][0];
    c.g[T7][0] = c.g[A1][0];
    let t0 = c.g[T0][0] as u32;
    c.g[V1][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
    c.g[A0][0] = c.lw(t0.wrapping_add(AUDIO_RING_READ));
    c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_SIZE));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.cycles += 4;
    c.div(0, 3, 2);
    c.g[V1][0] = c.hi[0];
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[T4][0] = c.g[V1][0];
    if !empty {
        c.cycles += 3;
        c.g[T1][0] = sx(t0.wrapping_add(AUDIO_RING_SIZE));
        c.g[V1][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
        loop {
            // Pop 1 (hist pointer -> t3).
            c.cycles += 4;
            c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_READ));
            c.g[A0][0] = c.g[V1][0];
            let nonempty = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = c.g[V0][0];
            if nonempty {
                audio_ring_advance(c, 10, true);
                c.cycles += 3;
                c.g[A0][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
                c.g[T3][0] = c.g[V0][0];
                c.g[A1][0] = c.lw(t0.wrapping_add(AUDIO_RING_READ));
            } else {
                c.cycles += 2;
                c.g[T3][0] = 0xffff_ffff_ffff_ffff;
            }
            // Pops 2-4 (handle -> t5, time -> t2, looping -> t6).
            audio_ring_pop_mid(c, T5);
            audio_ring_pop_mid(c, T2);
            audio_ring_pop_mid(c, T6);
            // Pop 5 (hull -> a1).
            c.cycles += 2;
            if c.g[A0][0] != c.g[A1][0] {
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
                audio_ring_advance(c, 9, false);
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
            }
            // The record matches: the new time (a2) replaces the old one.
            c.cycles += 2;
            if c.g[T7][0] != c.g[T3][0] {
                c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.g[A3][0] ^ c.g[A1][0];
                if c.g[V0][0] == 0 {
                    c.g[T2][0] = c.g[A2][0];
                }
                c.g[V0][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
            }
            audio_ring_push(c, T3, false);
            audio_ring_push(c, T5, false);
            audio_ring_push(c, T2, false);
            audio_ring_push(c, T6, false);
            audio_ring_push(c, A1, true);
            c.cycles += 2;
            if c.g[T4][0] != 0 {
                c.g[V1][0] = c.lw(t0.wrapping_add(AUDIO_RING_WRITE));
                if event_due(c) {
                    deliver_event_here(c, 0x0029_5990);
                }
                continue;
            }
            break;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// Offsets in the mem-card UI state (`gp - 0x1c38` points at it).
pub(crate) const MEMCARD_FILES_FLAGS: u32 = 0x22c;
pub(crate) const MEMCARD_SPACE_NEEDED: u32 = 0x230;
pub(crate) const MEMCARD_SPACE_FAILED: u32 = 0x234;

/// `void cFEStateMemCard_CheckFreeSpace(void)` (0x186f90): the card-scan
/// step that checks that the memory card has room for the missing save files.
/// When the check has already failed (state +0x234 set) it only forwards to
/// the card vtable (+0x70) once the stage is 4 or more. Otherwise, unless the
/// card has a critical error, it sums the space needed (the profile file
/// when flag bit 0 is set) against the free space and the free directory
/// entries; on a shortage it sets +0x234 and switches the manager to state
/// 0x17; else it finishes the scan (`cFEStateMemCard_OnScanDone`).
pub fn c_fe_state_mem_card_check_free_space(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        let state_ptr = (c.g[GP][0] as u32).wrapping_sub(0x1c38);
        let stage_ptr = (c.g[GP][0] as u32).wrapping_sub(0x1c34);
        let app_ptr = (c.g[GP][0] as u32).wrapping_sub(0x848);
        c.g[V0][0] = c.lw(state_ptr);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(MEMCARD_SPACE_FAILED));
        let failed = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(stage_ptr);
        'epilogue: {
            if failed {
                // The check failed earlier: forward to the card once it is ready.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                let early = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x60);
                if early {
                    break 'epilogue;
                }
                c.cycles += 13;
                c.g[A0][0] = c.lw(app_ptr);
                call_ptr(c, 0x0022_7f80, 0x0018_6fd4)?;
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x434));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xf8));
                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                c.g[A2][0] = c.lw(state_ptr);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x9c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018_6ff8)?;
                frame.restore(c, S0, 0x60);
                break 'epilogue;
            }
            'scan_done: {
                c.cycles += 7;
                c.g[A0][0] = c.lw(app_ptr);
                call_ptr(c, 0x0022_7f80, 0x0018_7008)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0023_c8d0, 0x0018_7014)?; // critical card error?
                if c.g[V0][0] != 0 {
                    break 'scan_done;
                }
                c.cycles += 15;
                c.g[S5][0] = 0;
                call_ptr(c, 0x0014_dc80, 0x0018_7024)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x5;
                call_ptr(c, 0x0014_dd58, 0x0018_7030)?;
                c.g[S4][0] = 0;
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0024_19d8, 0x0018_7040)?; // free space of the device
                c.g[V1][0] = c.lw(state_ptr);
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(MEMCARD_FILES_FLAGS));
                c.g[V0][0] &= 0x1;
                let no_profile = c.g[V0][0] == 0;
                c.sw((c.g[V1][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED), 0);
                if !no_profile {
                    c.cycles += 14;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                    c.g[S4][0] = 0x1;
                    c.g[S1][0] = c.lw(c.g[S0][0] as u32);
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0xb0));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x0015_2ba8, 0x0018_7070)?; // save-profile size
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb4));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x0018_7080)?; // card space for that size
                    c.g[A0][0] = c.lw(state_ptr);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(
                        (c.g[A0][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED),
                        c.g[V1][0],
                    );
                }
                // Not enough space, or not enough directory entries?
                c.cycles += 5;
                c.g[V1][0] = c.lw(state_ptr);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(MEMCARD_SPACE_NEEDED));
                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[S5][0] = 0x1;
                } else {
                    c.cycles += 9;
                    memcard_free_dir_entries(c, 0x0018_70bc)?;
                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                    if c.g[V0][0] != c.g[V1][0] {
                        c.cycles += 9;
                        memcard_free_dir_entries(c, 0x0018_70e0)?;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S5][0] = 0x1;
                        }
                    }
                }
                c.cycles += 2;
                let ok = c.g[S5][0] == 0;
                c.g[A0][0] = c.lw(state_ptr);
                if ok {
                    break 'scan_done;
                }
                // Failed: remember it and switch the manager to state 0x17.
                c.cycles += 10;
                c.g[V1][0] = 0x1;
                c.g[A1][0] = 0x17;
                c.sw(
                    (c.g[A0][0] as u32).wrapping_add(MEMCARD_SPACE_FAILED),
                    c.g[V1][0],
                );
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x748));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018_7114)?;
                frame.restore(c, S0, 0x60);
                break 'epilogue;
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(state_ptr);
            call_ptr(c, 0x0018_7270, 0x0018_7124)?; // cFEStateMemCard_OnScanDone
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `mgr->memCard->vtable[0xc8]` call (free directory entries), returning to
/// `ret`: `a1 = mgr[+0x434]`, then the slot's delta and function.
fn memcard_free_dir_entries(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's fakes (in each machine's own RAM).
    const OC771_CTRL: u32 = DATA + 0x2e_0000;
    const OC771_MAGIC: u32 = 0x0771_0771;
    /// Base of the fixtures' data (clear of the fakes' call log at DATA+0x100000).
    const OC771_BASE: u32 = DATA + 0x22_0000;

    /// Force the result of callee `idx` (a gated fake's index, or 16 + the
    /// vtable slot number of a fake method) to `v`, sign-extended.
    fn oc771_mail(c: &mut Cpu, idx: u32, v: u32) {
        c.bus.write32(OC771_CTRL + 0x10 + 8 * idx, 1);
        c.bus.write32(OC771_CTRL + 0x14 + 8 * idx, v);
    }

    /// A fake callee: logs the call like `fakes::fake_call`, then, unless it
    /// unwound, returns the sign-extended word the fixture mailed for it.
    fn oc771_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc == ra && c.bus.read32(OC771_CTRL + 0x10 + 8 * idx) != 0 {
            let v = c.bus.read32(OC771_CTRL + 0x14 + 8 * idx);
            c.g[V0][0] = sx(v);
        }
    }

    /// The machine's miss hook: fake methods at `FAKE + 0x10 * k` are
    /// callee `16 + k`.
    fn oc771_miss(c: &mut Cpu, target: u32) {
        let idx = 16 + target.wrapping_sub(fakes::FAKE) / 0x10;
        oc771_fake_call(c, target, idx.min(31));
    }

    macro_rules! oc771_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC771_CTRL) != OC771_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc771_fake_call(c, $addr, $idx);
                }
            )*
            fn oc771_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc771_fakes!(
        (
            oc771_fake_push,
            OC771_PREV_PUSH,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400,
            0
        ),
        (
            oc771_fake_preview,
            OC771_PREV_PREVIEW,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0,
            1
        ),
        (
            oc771_fake_options_ctor,
            OC771_PREV_OPTIONS_CTOR,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x0018_87a0,
            2
        ),
        (
            oc771_fake_alloc,
            OC771_PREV_ALLOC,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            3
        ),
        (
            oc771_fake_push_special,
            OC771_PREV_PUSH_SPECIAL,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039_f320,
            4
        ),
        (
            oc771_fake_mc_manager,
            OC771_PREV_MC_MANAGER,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80,
            5
        ),
        (
            oc771_fake_critical,
            OC771_PREV_CRITICAL,
            ee::s06::OVERRIDE_0023C8D0,
            ee::s06::f_0023c8d0__generated,
            0x0023_c8d0,
            6
        ),
        (
            oc771_fake_get_be,
            OC771_PREV_GET_BE,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            7
        ),
        (
            oc771_fake_get_iface,
            OC771_PREV_GET_IFACE,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            8
        ),
        (
            oc771_fake_free_space,
            OC771_PREV_FREE_SPACE,
            ee::s06::OVERRIDE_002419D8,
            ee::s06::f_002419d8__generated,
            0x0024_19d8,
            9
        ),
        (
            oc771_fake_profile_size,
            OC771_PREV_PROFILE_SIZE,
            ee::s01::OVERRIDE_00152BA8,
            ee::s01::f_00152ba8__generated,
            0x0015_2ba8,
            10
        ),
        (
            oc771_fake_scan_done,
            OC771_PREV_SCAN_DONE,
            ee::s02::OVERRIDE_00187270,
            ee::s02::f_00187270__generated,
            0x0018_7270,
            11
        ),
    );

    /// `cFEStateCharSetup_OnWidgetAction`: events 5/6/7 and others (also with
    /// a 64-bit event word), a null item, the factory returning a state or
    /// null (vtable +0x24 / +0x2c), and the faked pushes, preview call,
    /// allocator and constructor (which may unwind).
    #[test]
    fn c_fe_state_char_setup_on_widget_action_matches() {
        oc771_install_fakes();
        const EVENTS: [u64; 12] = [
            5,
            6,
            7,
            5,
            6,
            7,
            5,
            6,
            7,
            1,
            0x1_0000_0005,
            0x8000_0000_0000_0007,
        ];
        covered(
            ee::s02::f_00182c80__generated,
            c_fe_state_char_setup_on_widget_action,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x182c80, 3);
                c.hooks.miss = oc771_miss;
                c.pc = 0x0018_2c80;
                c.bus.write32(OC771_CTRL, OC771_MAGIC);
                let this = OC771_BASE;
                let (engine, custom, vt, item) = (
                    OC771_BASE + 0x1000,
                    OC771_BASE + 0x2000,
                    OC771_BASE + 0x3000,
                    OC771_BASE + 0x4000,
                );
                c.bus.write32(this + CHAR_SETUP_ENGINE, engine);
                c.bus.write32(engine, custom);
                c.bus.write32(custom + 4, vt);
                for (slot, k) in [(0x20u32, 1u32), (0x28, 2)] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + 0x10 * k);
                }
                c.bus.write32(item + 0x18, r.next_u32());
                // The factory's next / previous state: null or a pointer.
                let state = if (seed / 12) % 2 == 0 {
                    0
                } else {
                    OC771_BASE + 0x5000
                };
                oc771_mail(c, 17, state);
                oc771_mail(
                    c,
                    18,
                    if (seed / 24) % 2 == 0 {
                        0
                    } else {
                        state | 0x10
                    },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { item as u64 };
                c.g[A2][0] = EVENTS[seed as usize % 12];
            },
        );
    }

    /// `WorldTriggerManager_GetTriggerName`: index in range or not (also
    /// negative), entry type 3 (the real `strcpy` of the name), 4 (the special
    /// name leaf) or other.
    #[test]
    fn world_trigger_manager_get_trigger_name_matches() {
        check(
            ee::s08::f_002b65b0__generated,
            world_trigger_manager_get_trigger_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x2b65_b0 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x002b_65b0;
                let (mgr, list, names, dst) =
                    (DATA + 0x1000, DATA + 0x2100, DATA + 0x3000, DATA + 0x4000);
                c.bus.write32(mgr + WTM_TRIGGER_COUNT, 3);
                c.bus.write32(mgr + WTM_TRIGGER_LIST, list);
                for i in 0..3u32 {
                    let ty = match (seed / 3 + i) % 4 {
                        0 | 1 => 3,
                        2 => 4,
                        _ => 7,
                    };
                    c.bus.write32(list + 8 * i, ty);
                    c.bus.write32(list + 8 * i + 4, names + 0x40 * i);
                    for k in 0..12u32 {
                        c.bus
                            .write8(names + 0x40 * i + 0x10 + k, 0x41 + ((i + k) % 20) as u8);
                    }
                    c.bus.write8(names + 0x40 * i + 0x10 + 4 + 2 * i, 0);
                }
                c.g[A0][0] = mgr as u64;
                c.g[A1][0] = match seed % 6 {
                    0..=2 => (seed % 3) as u64,
                    3 => 3,
                    4 => 0xffff_ffff_ffff_ffff,
                    _ => 1,
                };
                c.g[A2][0] = dst as u64;
            },
        );
    }

    /// `SSXAudioSystem_SetTrackedSoundExpiry`: the tracked-sound ring (aligned
    /// five-word records, or arbitrary read/write indices and ring sizes: empty
    /// and full rings, wrap-around) with matching and non-matching
    /// history/hull words.
    #[test]
    fn ssx_audio_system_set_tracked_sound_expiry_matches() {
        check(
            ee::s08::f_00295950__generated,
            ssx_audio_system_set_tracked_sound_expiry,
            300,
            &|c, seed| {
                let mut r = Rng(0x29_5950 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                c.pc = 0x0029_5950;
                let (audio, buf) = (DATA + 0x1000, DATA + 0x10000);
                let (evt, idx) = (0x0090_1230 + 4 * (seed & 7), 3 + (seed % 5));
                let (size, w, rd) = if seed % 2 == 0 {
                    let k = 1 + seed / 2 % 3;
                    (5 * k, 5 * (r.next_u32() % k), 5 * (r.next_u32() % k))
                } else {
                    let s = 1 + seed % 9;
                    (s, r.next_u32() % s, r.next_u32() % s)
                };
                c.bus.write32(audio + AUDIO_RING_SIZE, size);
                c.bus.write32(audio + AUDIO_RING_BUF, buf);
                c.bus.write32(audio + AUDIO_RING_WRITE, w);
                c.bus.write32(audio + AUDIO_RING_READ, rd);
                for i in 0..size.max(1) {
                    let word = match i % 5 {
                        0 => {
                            if r.next_u32() % 2 == 0 {
                                evt
                            } else {
                                0x0090_5550
                            }
                        }
                        4 => {
                            if r.next_u32() % 2 == 0 {
                                idx
                            } else {
                                99
                            }
                        }
                        _ => r.next_u32(),
                    };
                    c.bus.write32(buf + 4 * i, word);
                }
                c.g[A0][0] = audio as u64;
                c.g[A1][0] = evt as u64;
                c.g[A2][0] = 0x1000 + seed as u64;
                c.g[A3][0] = idx as u64;
            },
        );
    }

    /// `cFEStateMemCard_CheckFreeSpace`: the failed-before state (stage below
    /// or above 4), a critical card error, and the free-space / directory-entry
    /// checks (profile file needed or not, results of -1, 0, 2), with the
    /// faked manager, card and vtable calls (which may unwind).
    #[test]
    fn c_fe_state_mem_card_check_free_space_matches() {
        oc771_install_fakes();
        covered(
            ee::s02::f_00186f90__generated,
            c_fe_state_mem_card_check_free_space,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186f90, 8);
                c.hooks.miss = oc771_miss;
                c.pc = 0x0018_6f90;
                c.bus.write32(OC771_CTRL, OC771_MAGIC);
                let gp = GAME_GP;
                let (state, mgr, card, vt, vt2) = (
                    OC771_BASE + 0x100,
                    OC771_BASE + 0x1000,
                    OC771_BASE + 0x2000,
                    OC771_BASE + 0x3000,
                    OC771_BASE + 0x5000,
                );
                c.bus.write32(gp.wrapping_sub(0x1c38), state);
                c.bus.write32(gp.wrapping_sub(0x1c34), seed / 3 % 8);
                c.bus.write32(gp.wrapping_sub(0x848), OC771_BASE + 0x800);
                c.bus
                    .write32(state + MEMCARD_SPACE_FAILED, (seed % 3 == 0) as u32);
                c.bus.write32(
                    state + MEMCARD_FILES_FLAGS,
                    r.next_u32() & 0xf | (seed / 2) % 2,
                );
                c.bus.write32(mgr + 0x428, r.next_u32());
                c.bus.write32(mgr + 0xf8, r.next_u32());
                c.bus.write32(mgr + 0x434, card);
                c.bus.write32(mgr + 0x748, vt2);
                c.bus.write16(vt2 + 8, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(vt2 + 0xc, fakes::FAKE + 0x30);
                c.bus.write32(card, vt);
                for (slot, k) in [(0x70u32, 4u32), (0xb0, 1), (0xc8, 2)] {
                    c.bus.write16(vt + slot, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(vt + slot + 4, fakes::FAKE + 0x10 * k);
                }
                oc771_mail(c, 5, mgr);
                oc771_mail(c, 6, ((seed / 24) % 5 == 0) as u32);
                oc771_mail(c, 9, if (seed / 5) % 2 == 0 { 5 } else { 100 });
                oc771_mail(c, 10, 40);
                oc771_mail(c, 17, if (seed / 7) % 2 == 0 { 10 } else { 60 });
                oc771_mail(c, 18, [0xffff_ffff, 0, 2][(seed / 11) as usize % 3]);
            },
        );
    }
}
