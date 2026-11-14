//! Staging module oc359 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc359`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002b_4c38,
    world_trigger_instance_update,
    "WorldTriggerInstance_Update",
)];

// ---------------------------------------------------------------------------
// WorldTriggerInstance_Update (0x2b4c38).

/// `WorldTriggerInstance` fields read by `WorldTriggerInstance_Update`.
/// Trigger id (CheckCrossing / WasPlayed key).
pub(crate) const WTI_UPD_ID: u32 = 0x4;
/// Second key of the duplicate check.
pub(crate) const WTI_UPD_KEY: u32 = 0x8;
/// Trigger index in the manager.
pub(crate) const WTI_UPD_INDEX: u32 = 0xc;
/// Volume, a float 0..1 (sent as `volume * 127`).
pub(crate) const WTI_UPD_VOLUME: u32 = 0x10;
/// Non-zero: the sound follows the position at `WTI_UPD_TARGET`.
pub(crate) const WTI_UPD_HAS_TARGET: u32 = 0x14;
/// The tracked position (its address is handed to the voice).
pub(crate) const WTI_UPD_TARGET: u32 = 0x20;
/// The playing handle: -1 = not started, -9999 = nothing playing.
pub(crate) const WTI_UPD_HANDLE: u32 = 0x30;
/// Non-zero: a crowd trigger (type 5), played through the crowd mixer.
pub(crate) const WTI_UPD_CROWD: u32 = 0x38;
/// The owning rider id.
pub(crate) const WTI_UPD_OWNER: u32 = 0x3c;
/// The trigger's sound-bank name (a C string).
pub(crate) const WTI_UPD_BANK_NAME: u32 = 0x40;
/// The sound's bank and sample, kept for restarts.
pub(crate) const WTI_UPD_BANK: u32 = 0x60;
pub(crate) const WTI_UPD_SAMPLE: u32 = 0x64;

/// Handle value: not started yet.
const WTI_UPD_IDLE: u64 = u64::MAX;
/// Handle value: nothing playing (-9999).
const WTI_UPD_NO_HANDLE: u64 = 0xffff_ffff_ffff_d8f1;
/// The world trigger manager pointer (gp-relative).
const WTI_UPD_GP_MANAGER: u32 = 0x21e4;
/// `.lit4` (gp - 0x4548): the fade time `BankMonitor_StopHandle` gets in f12.
const WTI_UPD_GP_FADE: u32 = 0x4548;
/// The empty bank name (`D_004A35A8`).
const WTI_UPD_EMPTY_NAME: u32 = 0x004a_35a8;
/// The trigger type whose bank is loaded on demand.
const WTI_UPD_BANKED_TYPE: u64 = 3;

const WTI_UPD_IS_TYPE5: Callee = Callee {
    addr: 0x002b_5838,
    f: ee::s08::WorldTrigger_IsType5,
};
const WTI_UPD_IS_DUPLICATE_ACTIVE: Callee = Callee {
    addr: 0x002b_66e8,
    f: ee::s08::WorldTriggerManager_IsDuplicateActive,
};
const WTI_UPD_GET_INFO: Callee = Callee {
    addr: 0x002b_5818,
    f: ee::s08::WorldTrigger_GetInfo,
};
const WTI_UPD_RESET: Callee = Callee {
    addr: 0x002b_5898,
    f: ee::s08::WorldTriggerInstance_Reset,
};
const WTI_UPD_COUNT_ACTIVE_OTHERS: Callee = Callee {
    addr: 0x002b_6630,
    f: ee::s08::WorldTriggerManager_CountActiveOthers,
};
const WTI_UPD_GET_NAME: Callee = Callee {
    addr: 0x002b_5878,
    f: ee::s08::WorldTrigger_GetName,
};
const WTI_UPD_LOAD_TRIGGER_BANK: Callee = Callee {
    addr: 0x0029_b818,
    f: ee::s08::SSXAudioSystem_LoadTriggerBank,
};
const WTI_UPD_IS_FLAG4: Callee = Callee {
    addr: 0x002b_67d8,
    f: ee::s08::WorldTriggerManager_IsTriggerFlag4,
};
const WTI_UPD_WAS_PLAYED: Callee = Callee {
    addr: 0x002b_6808,
    f: ee::s08::WorldTriggerManager_WasPlayed,
};
const WTI_UPD_IS_FLAG8: Callee = Callee {
    addr: 0x002b_6988,
    f: ee::s08::WorldTriggerManager_IsTriggerFlag8,
};
const WTI_UPD_CHECK_CROSSING: Callee = Callee {
    addr: 0x002b_69b8,
    f: ee::s08::WorldTriggerManager_CheckCrossing,
};
const WTI_UPD_PUSH_PLAY_PARAMS: Callee = Callee {
    addr: 0x002a_7468,
    f: ee::s08::CrowdAudio_PushPlayParams,
};
const WTI_UPD_ADD_SOURCE: Callee = Callee {
    addr: 0x002a_7678,
    f: ee::s08::CrowdAudio_AddSource,
};
const WTI_UPD_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};
const WTI_UPD_SET_SOURCE_PARAM: Callee = Callee {
    addr: 0x002a_7340,
    f: ee::s08::CrowdAudio_SetSourceParam,
};
const WTI_UPD_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::BankMonitor_FindInstance,
};
/// Called through the function table.
const WTI_UPD_GET_AUDIO: u32 = 0x0028_b180; // SSXAudioSystem_GetInstance
const WTI_UPD_FIND_RIDER: u32 = 0x0028_5d98; // SSXAudioSystem_FindRider
const WTI_UPD_STRCMP: u32 = 0x0041_65a8;
const WTI_UPD_GAIN_PTR: u32 = 0x0028_7968; // SSXAudioSystem_GetChannelGainPtr
const WTI_UPD_MEMSET: u32 = 0x0041_6210;
const WTI_UPD_PLAY_PUSHED: u32 = 0x0029_06b8; // Audio_PlayPushedSound

/// The address of field `off` of the instance (kept in s6).
#[inline(always)]
fn wti_upd_field(c: &Cpu, off: u32) -> u32 {
    (c.g[S6][0] as u32).wrapping_add(off)
}

/// `lw a0, 0x21e4(gp)`: the world trigger manager.
#[inline(always)]
fn wti_upd_load_manager(c: &mut Cpu) {
    let a = (c.g[GP][0] as u32).wrapping_add(WTI_UPD_GP_MANAGER);
    c.g[A0][0] = c.lw(a);
}

/// `void WorldTriggerInstance_Update(WorldTriggerInstance* self)` (0x2b4c38):
/// the per-frame update of one active world sound trigger.
///
/// Not started yet (handle -1): check that the trigger may play (not a
/// duplicate, its info found, for bank-loaded type-3 triggers the owning
/// rider, no other instance active, the bank loaded; for flag-4 triggers
/// `WasPlayed`), then start it when the rider crosses it: a crowd
/// trigger through the crowd mixer, any other one by queueing the sound on
/// the audio system's push queue and playing it. Failing a check resets the
/// instance. Already started: restart the sound on a new crossing (stopping
/// the old one), else update the playing sound's volume.
pub fn world_trigger_instance_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = WTI_UPD_IDLE;
        frame.save(c, S6, 0x20);
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_HANDLE));
        if c.g[V0][0] != c.g[V1][0] {
            wti_upd_load_manager(c);
            wti_upd_running(c, frame)?;
        } else {
            wti_upd_start(c, frame)?;
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The common exit of the play paths (`lq s0` before the epilogue).
fn wti_upd_done(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x80);
}

/// The trigger may not play: `WorldTriggerInstance_Reset(self)`.
fn wti_upd_reset(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.g[S6][0];
    WTI_UPD_RESET.call(c, 0x002b_527c)?;
    frame.restore(c, S0, 0x80);
    Ok(())
}

/// Store v0 as the handle and leave.
fn wti_upd_store_handle(c: &mut Cpu, frame: Frame) {
    c.cycles += 2;
    c.sw(wti_upd_field(c, WTI_UPD_HANDLE), c.g[V0][0]);
    wti_upd_done(c, frame);
}

/// Handle -1: the trigger has not started yet.
fn wti_upd_start(c: &mut Cpu, frame: Frame) -> Flow {
    // Crowd triggers are type 5.
    c.cycles += 4;
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));
    WTI_UPD_IS_TYPE5.call(c, 0x002b_4c78)?;
    let crowd = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if crowd {
        c.cycles += 2;
        c.sw(wti_upd_field(c, WTI_UPD_CROWD), 1);
    } else {
        c.cycles += 1;
        c.sw(wti_upd_field(c, WTI_UPD_CROWD), 0);
    }

    // Sound triggers: not while the same sound is already active.
    c.cycles += 3;
    c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_CROWD));
    if c.g[V0][0] == 0 {
        c.cycles += 7;
        wti_upd_load_manager(c);
        c.g[A3][0] = c.g[S6][0];
        c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_ID));
        c.g[A2][0] = c.lw(wti_upd_field(c, WTI_UPD_KEY));
        WTI_UPD_IS_DUPLICATE_ACTIVE.call(c, 0x002b_4cac)?;
        if c.g[V0][0] != 0 {
            return wti_upd_reset(c, frame);
        }
        c.cycles += 1;
    }
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));

    // The trigger's info: type, sample, deferred flag (on the frame).
    c.cycles += 12;
    c.g[A0][0] = c.g[S6][0];
    c.g[T1][0] = c.lw(wti_upd_field(c, WTI_UPD_OWNER));
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = c.g[SP][0] | 0x4;
    c.g[T0][0] = c.g[SP][0] | 0x8;
    c.g[T2][0] = 0;
    WTI_UPD_GET_INFO.call(c, 0x002b_4cd4)?;
    c.g[A0][0] = c.lw(sp(c));
    c.g[V1][0] = c.lw(sp(c).wrapping_add(0x4));
    c.sw(wti_upd_field(c, WTI_UPD_BANK), c.g[A0][0]);
    let found = c.g[V0][0] != 0;
    c.sw(wti_upd_field(c, WTI_UPD_SAMPLE), c.g[V1][0]);
    if !found {
        return wti_upd_reset(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x8));
    let deferred = c.g[V0][0] != 0;
    c.g[V0][0] = WTI_UPD_NO_HANDLE;
    if deferred {
        wti_upd_store_handle(c, frame);
        return Ok(());
    }

    // Type 3: the owning rider's trigger only, alone, its bank loaded (s0:
    // the load result, -1 for other types).
    c.cycles += 3;
    c.g[V0][0] = WTI_UPD_BANKED_TYPE;
    let banked = c.g[A0][0] == c.g[V0][0];
    c.g[S0][0] = u64::MAX;
    if banked {
        c.cycles += 9;
        call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_4d08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = u64::MAX;
        call_ptr(c, WTI_UPD_FIND_RIDER, 0x002b_4d14)?;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x870));
        c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_OWNER));
        let other_rider = c.g[V1][0] != c.g[V0][0];
        wti_upd_load_manager(c);
        if other_rider {
            return wti_upd_reset(c, frame);
        }
        c.cycles += 4;
        c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));
        WTI_UPD_COUNT_ACTIVE_OTHERS.call(c, 0x002b_4d2c)?;
        let others = (c.g[V0][0] as i64) > 0;
        c.g[A0][0] = sx(wti_upd_field(c, WTI_UPD_BANK_NAME));
        if others {
            return wti_upd_reset(c, frame);
        }
        c.cycles += 6;
        c.g[A1][0] = WTI_UPD_EMPTY_NAME as u64;
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, WTI_UPD_STRCMP, 0x002b_4d44)?;
        let named = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S6][0];
        if !named {
            // No bank name yet: ask the manager for it.
            c.cycles += 5;
            c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));
            c.g[A2][0] = c.g[S0][0];
            WTI_UPD_GET_NAME.call(c, 0x002b_4d58)?;
            if c.g[V0][0] == 0 {
                return wti_upd_reset(c, frame);
            }
        }
        c.cycles += 7;
        call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_4d68)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[SP][0];
        WTI_UPD_LOAD_TRIGGER_BANK.call(c, 0x002b_4d78)?;
        c.g[S0][0] = c.g[V0][0];
    }

    // Flag-4 triggers: only when `WasPlayed` (which also marks the id as
    // the last one) reports the id played.
    c.cycles += 5;
    wti_upd_load_manager(c);
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));
    WTI_UPD_IS_FLAG4.call(c, 0x002b_4d88)?;
    let one_shot = c.g[V0][0] != 0;
    wti_upd_load_manager(c);
    if one_shot {
        c.cycles += 4;
        c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_ID));
        WTI_UPD_WAS_PLAYED.call(c, 0x002b_4d98)?;
        if c.g[V0][0] == 0 {
            return wti_upd_reset(c, frame);
        }
    }

    // Type 3 waits until the bank is loaded (LoadTriggerBank returned 1).
    c.cycles += 4;
    c.g[V1][0] = c.lw(sp(c));
    c.g[V0][0] = WTI_UPD_BANKED_TYPE;
    let banked = c.g[V1][0] == c.g[V0][0];
    wti_upd_load_manager(c);
    if banked {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let not_loaded = c.g[S0][0] != c.g[V0][0];
        frame.restore(c, S0, 0x80);
        if not_loaded {
            return Ok(());
        }
        c.cycles += 1;
        wti_upd_load_manager(c);
    }

    // Flag-8 triggers start only when crossed.
    c.cycles += 4;
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));
    WTI_UPD_IS_FLAG8.call(c, 0x002b_4dc8)?;
    let on_crossing = c.g[V0][0] != 0;
    wti_upd_load_manager(c);
    if !on_crossing {
        c.cycles += 1;
        c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_CROWD));
    } else {
        c.cycles += 5;
        c.g[A2][0] = c.g[S6][0];
        c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_ID));
        WTI_UPD_CHECK_CROSSING.call(c, 0x002b_4ddc)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_CROWD));
        } else {
            c.cycles += 1;
            c.g[V0][0] = WTI_UPD_NO_HANDLE;
            wti_upd_store_handle(c, frame);
            return Ok(());
        }
    }

    c.cycles += 2;
    if c.g[V0][0] != 0 {
        // Crowd: push the play parameters, set up the voice, add a source.
        c.cycles += 31;
        call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_4e04)?;
        c.g[A0][0] = c.g[V0][0];
        WTI_UPD_PUSH_PLAY_PARAMS.call(c, 0x002b_4e0c)?;
        call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_4e14)?;
        c.g[A2][0] = 0x18;
        wti_upd_volume(c, A3, T0, A2, 0);
        wti_upd_voice_tail(c, 0x002b_4ebc, 16)?;
        c.g[A1][0] = c.lw(sp(c).wrapping_add(0x4));
        c.g[A0][0] = c.g[V0][0];
        WTI_UPD_ADD_SOURCE.call(c, 0x002b_4fa0)?;
    } else {
        // Sound: queue the info's bank and sample, play it.
        wti_upd_queue_sound(c, 0x002b_4fa8, true)?;
        wti_upd_volume(c, A2, A3, S3, 2);
        wti_upd_voice_tail(c, 0x002b_55ac, 15)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, WTI_UPD_PLAY_PUSHED, 0x002b_568c)?;
    }
    c.sw(wti_upd_field(c, WTI_UPD_HANDLE), c.g[V0][0]);
    wti_upd_done(c, frame);
    Ok(())
}

/// Handle != -1 (a0 = the manager): restart on a new crossing, else follow
/// the volume.
fn wti_upd_running(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_INDEX));
    WTI_UPD_IS_FLAG8.call(c, 0x002b_528c)?;
    let on_crossing = c.g[V0][0] != 0;
    wti_upd_load_manager(c);
    if on_crossing {
        c.cycles += 5;
        c.g[A2][0] = c.g[S6][0];
        c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_ID));
        WTI_UPD_CHECK_CROSSING.call(c, 0x002b_52a0)?;
        let crossed = c.g[V0][0] != 0;
        c.g[V0][0] = WTI_UPD_NO_HANDLE;
        if crossed {
            // Stop the old sound (0.1 s fade), queue and play it again.
            c.cycles += 3;
            c.g[V1][0] = c.lw(wti_upd_field(c, WTI_UPD_HANDLE));
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 9;
                call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_52bc)?;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
                c.g[A2][0] = 1;
                c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_HANDLE));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                let fade = (c.g[GP][0] as u32).wrapping_sub(WTI_UPD_GP_FADE);
                c.lwc1(12, fade);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                WTI_UPD_STOP_HANDLE.call(c, 0x002b_52d8)?;
            }
            wti_upd_queue_sound(c, 0x002b_52d8, false)?;
            wti_upd_volume(c, A2, A3, S3, 0);
            wti_upd_voice_tail(c, 0x002b_55ac, 15)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, WTI_UPD_PLAY_PUSHED, 0x002b_568c)?;
            c.sw(wti_upd_field(c, WTI_UPD_HANDLE), c.g[V0][0]);
            wti_upd_done(c, frame);
            return Ok(());
        }
    }

    // Not (re)crossed: follow the volume of what is playing.
    c.cycles += 3;
    c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_CROWD));
    let crowd = c.g[V0][0] != 0;
    c.g[V0][0] = WTI_UPD_NO_HANDLE;
    c.cycles += 3;
    c.g[V1][0] = c.lw(wti_upd_field(c, WTI_UPD_HANDLE));
    let silent = c.g[V1][0] == c.g[V0][0];
    frame.restore(c, S0, 0x80);
    if silent {
        return Ok(());
    }
    if crowd {
        c.cycles += 13;
        call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_56b4)?;
        c.g[AT][0] = 0x42fe_0000; // 127.0
        c.mtc1(AT, 1);
        c.g[A0][0] = c.g[V0][0];
        c.lwc1(0, wti_upd_field(c, WTI_UPD_VOLUME));
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(A2, 1);
        c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_HANDLE));
        WTI_UPD_SET_SOURCE_PARAM.call(c, 0x002b_56d8)?;
        frame.restore(c, S0, 0x80);
        return Ok(());
    }
    c.cycles += 10;
    call_ptr(c, WTI_UPD_GET_AUDIO, 0x002b_56f4)?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
    c.g[A2][0] = 1;
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_HANDLE));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    WTI_UPD_FIND_INSTANCE.call(c, 0x002b_570c)?;
    let found = c.g[V0][0] != 0;
    frame.restore(c, S0, 0x80);
    if !found {
        return Ok(());
    }
    // The instance's volume byte (+0x64) = volume * 127.
    c.cycles += 6;
    c.lwc1(0, wti_upd_field(c, WTI_UPD_VOLUME));
    c.g[AT][0] = 0x42fe_0000;
    c.mtc1(AT, 1);
    c.mul_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x64));
    wti_upd_done(c, frame);
    Ok(())
}

/// One `table[cur] = value` store of the push queue set-up: `idx` gets the
/// queue's current slot (`s0 + 0x1f4`), `base` the table at `s1 + off`.
fn wti_upd_queue_store(
    c: &mut Cpu,
    idx: usize,
    base: usize,
    off: u32,
    shift: u32,
    field: u32,
    value: u64,
) {
    c.g[idx][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[base][0] = c.lw((c.g[S1][0] as u32).wrapping_add(off));
    c.g[idx][0] = sx((c.g[idx][0] as u32) << shift);
    c.g[idx][0] = sx((c.g[idx][0] as u32).wrapping_add(c.g[base][0] as u32));
    c.sw((c.g[idx][0] as u32).wrapping_add(field), value);
}

/// Open a new slot on the audio system's push queue (the inlined
/// `SFX_PushDefault`): copy the 24-byte default entry, reset the slot's
/// per-sound tables (fade 100/90/50, volume 127, flag 1) and set its bank
/// and sample (from the info on the frame, or kept in the instance for a
/// restart). `base` is the compiled block's address (the return addresses
/// follow from it).
fn wti_upd_queue_sound(c: &mut Cpu, base: u32, from_frame: bool) -> Flow {
    c.cycles += 164;
    c.g[S3][0] = 0x18;
    call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x8)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[S4][0] = if from_frame {
        c.lw(sp(c))
    } else {
        c.lw(wti_upd_field(c, WTI_UPD_BANK))
    };
    c.g[S0][0] = c.lw(c.g[S2][0] as u32);
    c.g[A1][0] = 0;
    c.g[S5][0] = if from_frame {
        c.lw(sp(c).wrapping_add(0x4))
    } else {
        c.lw(wti_upd_field(c, WTI_UPD_SAMPLE))
    };
    c.g[A2][0] = 0x8;
    let mgr = c.g[S0][0] as u32;
    c.g[V0][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[S1][0] = sx(mgr.wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sw(mgr.wrapping_add(0x1f4), c.g[V0][0]);
    // entries[cur] = the default entry (24 bytes, unaligned copy).
    c.g[V1][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, V1, V1, S3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(V0, mgr.wrapping_add(0x1e3));
    c.ldr(V0, mgr.wrapping_add(0x1dc));
    c.ldl(A0, mgr.wrapping_add(0x1eb));
    c.ldr(A0, mgr.wrapping_add(0x1e4));
    c.ldl(A3, mgr.wrapping_add(0x1f3));
    c.ldr(A3, mgr.wrapping_add(0x1ec));
    let entry = c.g[V1][0] as u32;
    c.sdl(V0, entry.wrapping_add(0x7));
    c.sdr(V0, entry);
    c.sdl(A0, entry.wrapping_add(0xf));
    c.sdr(A0, entry.wrapping_add(0x8));
    c.sdl(A3, entry.wrapping_add(0x17));
    c.sdr(A3, entry.wrapping_add(0x10));
    // The target pointer (mgr + 0x1fc + 4 cur) and table 0x34.
    c.g[V0][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(mgr.wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
    wti_upd_queue_store(c, V1, V0, 0x34, 2, 0, 0);
    // values[cur] = entries[cur].volume (signed byte).
    c.g[A3][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[A0][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S3);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    wti_upd_queue_store(c, V0, V1, 0x3c, 2, 0, 0);
    // memset(&queue[0x40 + 8 cur], 0, 8).
    c.g[A0][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, WTI_UPD_MEMSET, base + 0xe4)?;
    // Constants of the stores below (fade 100/90/50, volume 127, flag 1).
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    let mgr = c.g[S0][0] as u32;
    c.g[V1][0] = c.lw(mgr.wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0);
    c.g[T0][0] = 1;
    wti_upd_queue_store(c, V0, V1, 0x64, 2, 0, 0);
    wti_upd_queue_store(c, V1, V0, 0x68, 2, 0, 0);
    wti_upd_queue_store(c, V0, V1, 0x6c, 2, 0, 0);
    wti_upd_queue_store(c, V1, V0, 0x70, 4, 0x4, 0);
    let (fade0, fade1, fade2) = (c.g[A0][0], c.g[A1][0], c.g[A2][0]);
    wti_upd_queue_store(c, V0, V1, 0x70, 4, 0x0, fade0);
    wti_upd_queue_store(c, V1, V0, 0x70, 4, 0x8, fade1);
    wti_upd_queue_store(c, V0, V1, 0x70, 4, 0xc, fade2);
    wti_upd_queue_store(c, V1, V0, 0x74, 2, 0, 0);
    wti_upd_queue_store(c, V0, V1, 0x80, 2, 0, 0);
    wti_upd_queue_store(c, V1, V0, 0x84, 2, 0, 0);
    let (volume, flag) = (c.g[A3][0], c.g[T0][0]);
    wti_upd_queue_store(c, V0, V1, 0x78, 2, 0, volume);
    wti_upd_queue_store(c, V1, V0, 0x7c, 2, 0, flag);
    // The system's bank[cur] and sample[cur].
    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S4][0]);
    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S5][0]);
    call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x230)?;
    Ok(())
}

/// `v0 = &values[cur]` (the queue in `sub`, the values table in `table`).
fn wti_upd_value_slot(c: &mut Cpu, sub: usize, table: usize) -> u32 {
    c.g[V0][0] = c.lw((c.g[sub][0] as u32).wrapping_add(0x1c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[table][0] as u32));
    c.g[V0][0] as u32
}

/// The queued sound's volume (v0 = the audio system): `values[cur] =
/// (int)(volume * 127)`, then a0 = `&entries[cur]` and v0 = that value
/// clamped to 0..127 (stored as the entry's byte by the caller). `sub` and
/// `table` are the registers the compiled copy keeps the queue and the
/// values table in, `stride` the one holding 0x18, `high_cycles` what the
/// 127 clamp costs in this copy.
fn wti_upd_volume(c: &mut Cpu, sub: usize, table: usize, stride: usize, high_cycles: u64) {
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
    c.lwc1(0, wti_upd_field(c, WTI_UPD_VOLUME));
    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
    c.g[AT][0] = 0x42fe_0000; // 127.0
    c.mtc1(AT, 1);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[sub][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
    c.g[V1][0] = c.lw((c.g[sub][0] as u32).wrapping_add(0x38));
    c.mul_s(0, 0, 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.cvt_w_s(1, 0);
    c.swc1(1, c.g[V0][0] as u32);
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[sub][0] as u32).wrapping_add(0x1c));
    c.g[table][0] = c.lw((c.g[sub][0] as u32).wrapping_add(0x38));
    c.mult(0, A0, A0, stride);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = c.lw((c.g[sub][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[table][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let high = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    if !high {
        c.cycles += 6;
        let slot = wti_upd_value_slot(c, sub, table);
        c.g[V1][0] = c.lw(slot);
        let negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
        if negative {
            return;
        }
    }
    c.cycles += 7;
    let slot = wti_upd_value_slot(c, sub, table);
    c.g[V1][0] = c.lw(slot);
    c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
    if c.g[V1][0] == 0 {
        c.cycles += high_cycles;
        c.g[V0][0] = 0x7f;
    } else {
        c.cycles += 4;
        let slot = wti_upd_value_slot(c, sub, table);
        c.g[V0][0] = c.lbu(slot);
    }
}

/// The voice set-up after the volume (the inlined `voiceTail`): store the
/// entry's volume byte, the channel gain, for tracked sounds the target
/// pointer and its flags, the owner; return with v0 = the audio system.
/// `base` is the compiled block's address, `tail_cycles` the cost of its
/// last block.
fn wti_upd_voice_tail(c: &mut Cpu, base: u32, tail_cycles: u64) -> Flow {
    c.cycles += 17;
    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
    call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x8)?;
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
    call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x10)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 5;
    c.g[A2][0] = 0;
    call_ptr(c, WTI_UPD_GAIN_PTR, base + 0x20)?;
    // gains[cur] = the channel-5 gain pointer.
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[V0][0] = c.lw(wti_upd_field(c, WTI_UPD_HAS_TARGET));
    if c.g[V0][0] != 0 {
        c.cycles += 26;
        call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x4c)?;
        // targets[cur] = &self.target.
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
        c.g[A1][0] = sx(wti_upd_field(c, WTI_UPD_TARGET));
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A1][0]);
        call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x6c)?;
        // tracked[cur] = 1.
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
        c.g[A1][0] = 1;
        c.g[V1][0] = c.lw(c.g[A0][0] as u32);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        call_ptr(c, WTI_UPD_GET_AUDIO, base + 0x90)?;
        // table 0x7c [cur] = 0.
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0);
    }
    c.cycles += tail_cycles;
    call_ptr(c, WTI_UPD_GET_AUDIO, base + 0xb4)?;
    // owners[cur] = self.owner.
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x118));
    c.g[A1][0] = c.lw(wti_upd_field(c, WTI_UPD_OWNER));
    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
    call_ptr(c, WTI_UPD_GET_AUDIO, base + 0xd8)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, fall_back, hook, LOG_UNWIND_AT};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use crate::game::sound::soundsys::tests::audio;
    use std::sync::atomic::AtomicUsize;

    /// Fake control block: gate word, strcmp call count, results per callee
    /// (`+0x10 + 4 k`), the info record GetInfo writes (`+0x80`).
    const OC359_CTRL: u32 = DATA + 0x28_3000;
    const OC359_MAGIC: u32 = 0x0c35_9a11;
    const OC359_SELF: u32 = DATA + 0x28_0000;
    const OC359_RIDER: u32 = DATA + 0x28_1000;
    const OC359_SOUND: u32 = DATA + 0x28_2000;
    const OC359_MGR: u32 = DATA + 0x28_4000;

    const OC359_IS_TYPE5: u32 = 0;
    const OC359_IS_DUPLICATE: u32 = 1;
    const OC359_GET_INFO: u32 = 2;
    const OC359_FIND_RIDER: u32 = 3;
    const OC359_COUNT_OTHERS: u32 = 4;
    const OC359_STRCMP: u32 = 5;
    const OC359_GET_NAME: u32 = 6;
    const OC359_FLAG4: u32 = 7;
    const OC359_WAS_PLAYED: u32 = 8;
    const OC359_FLAG8: u32 = 9;
    const OC359_CROSSING: u32 = 10;
    const OC359_PUSH_PARAMS: u32 = 11;
    const OC359_STOP_HANDLE: u32 = 12;
    const OC359_SET_PARAM: u32 = 13;
    const OC359_FIND_INSTANCE: u32 = 14;
    /// strcmp after the first call (inside LoadTriggerBank and below).
    const OC359_STRCMP_LATER: u32 = 15;

    /// A gated fake: logs the call (`fakes::fake`), then returns the result
    /// the fixture scripted for this callee; GetInfo also writes the info
    /// record through its three pointer arguments.
    fn oc359_fake_call(c: &mut Cpu, addr: u32, slot: u32) {
        let record = [c.g[A2][0] as u32, c.g[A3][0] as u32, c.g[T0][0] as u32];
        let ra = c.g[RA][0] as u32;
        fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let mut slot = slot;
        if slot == OC359_STRCMP {
            let n = c.bus.read32(OC359_CTRL + 4);
            c.bus.write32(OC359_CTRL + 4, n + 1);
            if n > 0 {
                slot = OC359_STRCMP_LATER;
            }
        }
        c.g[V0][0] = sx(c.bus.read32(OC359_CTRL + 0x10 + 4 * slot));
        if slot == OC359_GET_INFO {
            for (i, p) in record.into_iter().enumerate() {
                let v = c.bus.read32(OC359_CTRL + 0x80 + 4 * i as u32);
                c.bus.write32(p, v);
            }
        }
    }

    macro_rules! oc359_fake {
        ($name:ident, $prev:ident, $addr:expr, $slot:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC359_CTRL) != OC359_MAGIC {
                    return fall_back(c, &$prev, $gen);
                }
                oc359_fake_call(c, $addr, $slot);
            }
        };
    }

    oc359_fake!(
        oc359_fake_is_type5,
        OC359_PREV_IS_TYPE5,
        0x002b_5838,
        OC359_IS_TYPE5,
        ee::s08::f_002b5838__generated
    );
    oc359_fake!(
        oc359_fake_is_duplicate,
        OC359_PREV_IS_DUPLICATE,
        0x002b_66e8,
        OC359_IS_DUPLICATE,
        ee::s08::f_002b66e8__generated
    );
    oc359_fake!(
        oc359_fake_get_info,
        OC359_PREV_GET_INFO,
        0x002b_5818,
        OC359_GET_INFO,
        ee::s08::f_002b5818__generated
    );
    oc359_fake!(
        oc359_fake_find_rider,
        OC359_PREV_FIND_RIDER,
        0x0028_5d98,
        OC359_FIND_RIDER,
        ee::s07::f_00285d98__generated
    );
    oc359_fake!(
        oc359_fake_count_others,
        OC359_PREV_COUNT_OTHERS,
        0x002b_6630,
        OC359_COUNT_OTHERS,
        ee::s08::f_002b6630__generated
    );
    oc359_fake!(
        oc359_fake_strcmp,
        OC359_PREV_STRCMP,
        0x0041_65a8,
        OC359_STRCMP,
        ee::s15::f_004165a8__generated
    );
    oc359_fake!(
        oc359_fake_get_name,
        OC359_PREV_GET_NAME,
        0x002b_5878,
        OC359_GET_NAME,
        ee::s08::f_002b5878__generated
    );
    oc359_fake!(
        oc359_fake_flag4,
        OC359_PREV_FLAG4,
        0x002b_67d8,
        OC359_FLAG4,
        ee::s08::f_002b67d8__generated
    );
    oc359_fake!(
        oc359_fake_was_played,
        OC359_PREV_WAS_PLAYED,
        0x002b_6808,
        OC359_WAS_PLAYED,
        ee::s08::f_002b6808__generated
    );
    oc359_fake!(
        oc359_fake_flag8,
        OC359_PREV_FLAG8,
        0x002b_6988,
        OC359_FLAG8,
        ee::s08::f_002b6988__generated
    );
    oc359_fake!(
        oc359_fake_crossing,
        OC359_PREV_CROSSING,
        0x002b_69b8,
        OC359_CROSSING,
        ee::s08::f_002b69b8__generated
    );
    oc359_fake!(
        oc359_fake_push_params,
        OC359_PREV_PUSH_PARAMS,
        0x002a_7468,
        OC359_PUSH_PARAMS,
        ee::s08::f_002a7468__generated
    );
    oc359_fake!(
        oc359_fake_stop_handle,
        OC359_PREV_STOP_HANDLE,
        0x002a_d5f0,
        OC359_STOP_HANDLE,
        ee::s08::f_002ad5f0__generated
    );
    oc359_fake!(
        oc359_fake_set_param,
        OC359_PREV_SET_PARAM,
        0x002a_7340,
        OC359_SET_PARAM,
        ee::s08::f_002a7340__generated
    );
    oc359_fake!(
        oc359_fake_find_instance,
        OC359_PREV_FIND_INSTANCE,
        0x002a_d550,
        OC359_FIND_INSTANCE,
        ee::s08::f_002ad550__generated
    );

    fn oc359_hook_all() {
        use std::ptr::addr_of_mut;
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B5838),
            oc359_fake_is_type5,
            &OC359_PREV_IS_TYPE5,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B66E8),
            oc359_fake_is_duplicate,
            &OC359_PREV_IS_DUPLICATE,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B5818),
            oc359_fake_get_info,
            &OC359_PREV_GET_INFO,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285D98),
            oc359_fake_find_rider,
            &OC359_PREV_FIND_RIDER,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6630),
            oc359_fake_count_others,
            &OC359_PREV_COUNT_OTHERS,
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            oc359_fake_strcmp,
            &OC359_PREV_STRCMP,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B5878),
            oc359_fake_get_name,
            &OC359_PREV_GET_NAME,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B67D8),
            oc359_fake_flag4,
            &OC359_PREV_FLAG4,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6808),
            oc359_fake_was_played,
            &OC359_PREV_WAS_PLAYED,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6988),
            oc359_fake_flag8,
            &OC359_PREV_FLAG8,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B69B8),
            oc359_fake_crossing,
            &OC359_PREV_CROSSING,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A7468),
            oc359_fake_push_params,
            &OC359_PREV_PUSH_PARAMS,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD5F0),
            oc359_fake_stop_handle,
            &OC359_PREV_STOP_HANDLE,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002A7340),
            oc359_fake_set_param,
            &OC359_PREV_SET_PARAM,
        );
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD550),
            oc359_fake_find_instance,
            &OC359_PREV_FIND_INSTANCE,
        );
    }

    /// The audio test world (real `GetInstance`, memset, gain lookup,
    /// `LoadTriggerBank` on its bank-state path, `CrowdAudio_AddSource` with
    /// no crowd mixer, `Audio_PlayPushedSound` with no bank), the trigger
    /// manager queries and the crowd/bank-monitor calls faked with scripted
    /// results, and an instance not started (half the seeds), silent
    /// (-9999) or playing.
    fn oc359_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = audio::world(c, seed, 0x2b_4c38, 0x002b_4c38);
        c.bus.write32(GAME_GP + 0x410, audio::SYS); // SSXAudioSystem_GetInstance
        c.bus.write32(GAME_GP + WTI_UPD_GP_MANAGER, OC359_MGR);
        c.bus.write32(audio::SYS + 0x18, 0); // no crowd mixer: AddSource returns -1
        if seed % 5 == 0 {
            c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 12);
        }
        // Scripted callee results.
        c.bus.write32(OC359_CTRL, OC359_MAGIC);
        c.bus.write32(OC359_CTRL + 4, 0);
        let owner = r.next_u32() % 4;
        let results: [u32; 16] = [
            r.next_u32() % 2,                                       // IsType5
            (r.next_u32() % 5 == 0) as u32,                         // IsDuplicateActive
            (r.next_u32() % 8 != 0) as u32,                         // GetInfo
            OC359_RIDER,                                            // FindRider
            [0, 0, 0, u32::MAX, 1, 5][(r.next_u32() % 6) as usize], // CountActiveOthers
            r.next_u32() % 2,                                       // strcmp (bank name vs "")
            (r.next_u32() % 5 != 0) as u32,                         // GetName
            r.next_u32() % 2,                                       // IsTriggerFlag4
            (r.next_u32() % 4 != 0) as u32,                         // WasPlayed
            (r.next_u32() % 3 != 0) as u32,                         // IsTriggerFlag8
            (r.next_u32() % 4 != 0) as u32,                         // CheckCrossing
            r.next_u32(),                                           // PushPlayParams
            r.next_u32(),                                           // StopHandle
            r.next_u32(),                                           // SetSourceParam
            if r.next_u32() % 3 == 0 {
                0
            } else {
                OC359_SOUND
            }, // FindInstance
            0,                                                      // strcmp later: same bank
        ];
        for (k, v) in results.into_iter().enumerate() {
            c.bus.write32(OC359_CTRL + 0x10 + 4 * k as u32, v);
        }
        // The info record: type (3 = bank-loaded), sample, deferred flag.
        let kind = if r.next_u32() % 2 == 0 {
            3
        } else {
            r.next_u32() % 3
        };
        c.bus.write32(OC359_CTRL + 0x80, kind);
        c.bus.write32(OC359_CTRL + 0x84, r.next_u32() % 0x100);
        c.bus.write32(
            OC359_CTRL + 0x88,
            if r.next_u32() % 6 == 0 {
                r.next_u32() | 1
            } else {
                0
            },
        );
        // LoadTriggerBank's bank state for type 3: loaded (1), failed (2, -1).
        c.bus.write32(
            audio::TABLES + 0x1180 + 3 * 0x60,
            [1, 1, 1, 2, u32::MAX][(r.next_u32() % 5) as usize],
        );
        // The owning rider.
        c.bus.write32(
            OC359_RIDER + 0x870,
            if r.next_u32() % 5 == 0 {
                owner ^ 1
            } else {
                owner
            },
        );
        // The instance.
        let s = OC359_SELF;
        c.bus.write32(s + WTI_UPD_ID, r.next_u32() % 16);
        c.bus.write32(s + WTI_UPD_KEY, r.next_u32() % 16);
        c.bus.write32(s + WTI_UPD_INDEX, r.next_u32() % 16);
        let volume = match r.next_u32() % 6 {
            0 => 0xbf00_0000, // -0.5
            1 => 0x3f80_0000, // 1.0
            2 => 0x3fc0_0000, // 1.5
            3 => 0,
            _ => r.f32(1.0),
        };
        c.bus.write32(s + WTI_UPD_VOLUME, volume);
        c.bus.write32(s + WTI_UPD_HAS_TARGET, r.next_u32() % 2);
        let handle = if seed % 2 == 0 {
            u32::MAX
        } else if (seed / 2) % 3 == 0 {
            WTI_UPD_NO_HANDLE as u32
        } else {
            r.next_u32() % 0x40
        };
        c.bus.write32(s + WTI_UPD_HANDLE, handle);
        c.bus.write32(s + WTI_UPD_CROWD, r.next_u32() % 2);
        c.bus.write32(s + WTI_UPD_OWNER, owner);
        c.bus.write32(s + WTI_UPD_BANK_NAME, 0x6b6e_6162); // "bank"
        c.bus.write32(s + WTI_UPD_BANK_NAME + 4, 0);
        c.bus.write32(s + WTI_UPD_BANK, r.next_u32() % 4);
        c.bus.write32(s + WTI_UPD_SAMPLE, r.next_u32() % 0x100);
        c.g[A0][0] = sx(s);
    }

    /// `WorldTriggerInstance_Update`: not started (duplicates, no info,
    /// deferred, type 3 with the wrong rider / other instances / no name /
    /// the bank loaded or not, one-shots played or not, crossings or not,
    /// crowd and sound starts with volumes below 0, inside and above 127,
    /// tracked or not), and started (restart on a crossing with and without
    /// a handle to stop, crowd volume, sound-instance volume found or not,
    /// silent). Callees unwind at random calls on a fifth of the seeds.
    #[test]
    fn world_trigger_instance_update_matches() {
        oc359_hook_all();
        check(
            ee::s08::f_002b4c38__generated,
            world_trigger_instance_update,
            240,
            &oc359_update_setup,
        );
    }
}
