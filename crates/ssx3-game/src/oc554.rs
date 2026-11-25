//! Staging module oc554 (wave wave232): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc554`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_0548,
        rider_script_obj_get_sound_bank_entry,
        "RiderScriptObj_GetSoundBankEntry",
    ),
    (
        0x0028_1000,
        script_audio_obj_get_sound_bank_entry,
        "ScriptAudioObj_GetSoundBankEntry",
    ),
    (
        0x0028_11e8,
        script_view_obj_remove_cam,
        "ScriptViewObj_RemoveCam",
    ),
    (
        0x0028_2020,
        script_engine_destroy_script_database,
        "ScriptEngine_DestroyScriptDatabase",
    ),
    (
        0x0028_2498,
        script_engine_set_process_speed,
        "ScriptEngine_SetProcessSpeed",
    ),
    (
        0x0028_2510,
        script_engine_set_process_user_data,
        "ScriptEngine_SetProcessUserData",
    ),
    (
        0x0028_2698,
        script_engine_pause_all_seqs,
        "ScriptEngine_PauseAllSeqs",
    ),
    (
        0x0028_26b8,
        script_engine_resume_all_seqs,
        "ScriptEngine_ResumeAllSeqs",
    ),
    (
        0x0028_27f8,
        script_engine_get_process_script_id,
        "ScriptEngine_GetProcessScriptId",
    ),
    (
        0x0028_2bc0,
        script_engine_get_process_user_data,
        "ScriptEngine_GetProcessUserData",
    ),
    (0x0028_39a8, script_movie_start, "ScriptMovie_Start"),
    (0x0028_48a0, script_view_obj_dtor, "ScriptViewObj_dtor"),
    (0x0028_48d8, script_dummy_obj_dtor, "ScriptDummyObj_dtor"),
    (0x0028_4950, script_audio_obj_dtor, "ScriptAudioObj_dtor"),
    (0x0028_4a28, script_obj_cam_dtor, "ScriptObjCam_dtor"),
    (
        0x0028_4aa8,
        c_transition_man_script_dtor,
        "cTransitionMan_ScriptDtor",
    ),
    (0x0028_4ae8, rider_script_obj_dtor, "RiderScriptObj_dtor"),
    (
        0x0028_4b80,
        script_engine_global_ctors,
        "ScriptEngine_GlobalCtors",
    ),
    (0x002c_28a0, game_link_get_ticks, "GameLink_GetTicks"),
    (0x002c_28c0, game_link_alloc, "GameLink_Alloc"),
];

/// `RiderScriptObj_GetSoundBankEntry` (0x280548): `*a3` gets the sound-bank
/// entry word of the object (`obj->+0x20`, then `+4`), and `*a2` gets `a1`.
pub fn rider_script_obj_get_sound_bank_entry(c: &mut Cpu) {
    copy_sound_bank_entry(c, 0x20);
}

/// `ScriptAudioObj_GetSoundBankEntry` (0x281000): as the rider version, with
/// the entry pointer at `obj->+0xf8`.
pub fn script_audio_obj_get_sound_bank_entry(c: &mut Cpu) {
    copy_sound_bank_entry(c, 0xf8);
}

/// The body both sound-bank getters share (they differ only in the offset
/// of the entry pointer).
fn copy_sound_bank_entry(c: &mut Cpu, entry_off: u32) {
    c.cycles += 5;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(entry_off).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4).lw(c);
    let entry = c.g[V1][0];
    Addr(c.g[A3][0] as u32).sw(c, entry);
    let a1 = c.g[A1][0];
    Addr(c.g[A2][0] as u32).sw(c, a1);
    ret(c);
}

/// `ScriptViewObj_RemoveCam(view, key)` (0x2811e8,
/// `cScriptObjCameraManager_RemoveCameraObject`): finds the camera object in
/// the view's two slots (`+0xd0`) whose `mObjID` equals `key->mObjID`. A
/// found camera loses its entity (when the view is in use, and its attached
/// slot is cleared), the manager is reset and the slot emptied. Returns 1
/// when a camera was removed, else 0.
pub fn script_view_obj_remove_cam(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        frame.save(c, 19, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, 20, 0x10);
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xdc));
        frame.save_ra(c, 0);
        c.g[S4][0] = u64::MAX;
        c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x4).lw(c);
        loop {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
            let mut found = false;
            if c.g[A0][0] == 0 {
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
                if c.g[V0][0] != c.g[V1][0] {
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                } else {
                    found = true;
                }
            }
            if found {
                remove_cam_entry(c)?;
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_1220);
            }
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
    });
}

/// The matched slot of [`script_view_obj_remove_cam`]: detaches the camera
/// (its entity when the view is in use, and its attached index), then resets
/// its manager. Leaves `v0 = 1`.
fn remove_cam_entry(c: &mut Cpu) -> Flow {
    let view = c.g[S0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = Addr(view).off(0x8).lw(c);
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x14).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A1][0] = 0;
            Callee {
                addr: 0x0027_d688,
                f: ee::s07::f_0027d688,
            }
            .call(c, 0x0028_1258)?;
            c.g[V0][0] = Addr(view).off(0xdc).lw(c);
            let other = c.g[V0][0] != c.g[S2][0];
            c.g[A0][0] = 0;
            if other {
                c.cycles += 1;
                c.g[A0][0] = 1;
                loop {
                    c.cycles += 3;
                    let inside = (c.g[A0][0] as i64) < 2;
                    c.g[V0][0] = inside as u64;
                    let past_end = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                    if past_end {
                        break;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                    let still_other = c.g[V1][0] != c.g[S2][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if still_other {
                        if event_due(c) {
                            deliver_event_here(c, 0x0028_1274);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    let attached = c.g[V0][0] as u32;
                    let minus_one = c.g[S4][0];
                    Addr(attached).sw(c, minus_one);
                    break;
                }
            } else {
                c.cycles += 2;
                let minus_one = c.g[S4][0];
                Addr(view).off(0xdc).sw(c, minus_one);
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        }
    }
    c.cycles += 5;
    c.g[A1][0] = 0;
    Callee {
        addr: 0x0027_d680,
        f: ee::s07::f_0027d680,
    }
    .call(c, 0x0028_12a0)?;
    let slot = c.g[S1][0] as u32;
    Addr(slot).sw(c, 0);
    c.g[V0][0] = 1;
    Ok(())
}

/// `ScriptEngine_DestroyScriptDatabase` (0x282020, the reference's
/// `cBXScriptEngine_PurgeBXEngine`): when a script database is loaded, stops
/// and kills every process, removes the script objects, frees the database
/// (`+0x2b0`, its size at `+0x2b4`) and the in-sequence object (`+0x2b8`),
/// frees its heap (`+0x2bc`) and resets the process table.
pub fn script_engine_destroy_script_database(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(this).off(0x2b0).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 9;
            Callee {
                addr: 0x0028_2390,
                f: ee::s07::f_00282390,
            }
            .call(c, 0x0028_2044)?;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0028_23f0,
                f: ee::s07::f_002823f0,
            }
            .call(c, 0x0028_204c)?;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0027_d050,
                f: ee::s07::f_0027d050,
            }
            .call(c, 0x0028_2054)?;
            c.g[A0][0] = Addr(this).off(0x2b0).lw(c);
            if c.g[A0][0] == 0 {
                c.g[A0][0] = Addr(this).off(0x2b8).lw(c);
            } else {
                c.cycles += 3;
                call_ptr(c, 0x0031_7e98, 0x0028_2068)?;
                c.g[A0][0] = Addr(this).off(0x2b8).lw(c);
            }
            c.cycles += 3;
            Addr(this).off(0x2b0).sw(c, 0);
            let has_in_seq = c.g[A0][0] != 0;
            Addr(this).off(0x2b4).sw(c, 0);
            if has_in_seq {
                c.cycles += 2;
                c.g[A1][0] = 3;
                Callee {
                    addr: 0x0027_28d0,
                    f: ee::s07::f_002728d0,
                }
                .call(c, 0x0028_2080)?;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(this).off(0x2bc).lw(c);
            let has_heap = c.g[A0][0] != 0;
            Addr(this).off(0x2b8).sw(c, 0);
            if has_heap {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x0028_2094)?;
            }
            c.cycles += 4;
            Addr(this).off(0x2bc).sw(c, 0);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0028_28b8,
                f: ee::s07::f_002828b8,
            }
            .call(c, 0x0028_20a0)?;
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptEngine_SetProcessSpeed` (0x282498, `cBXScriptEngine_SetScriptPlayRate`):
/// a rate below 0.01 is refused (returns 0). Otherwise the process of the
/// handle `a1` takes the rate: stored in the process while it is still
/// loading (state 1, returns 1), else handed to its in-sequence object.
pub fn script_engine_set_process_speed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4784));
        frame.save_fpr(c, 20, 0x10);
        c.mov_s(20, 12);
        let rate = FpuF32(c.f[20]);
        let threshold = FpuF32(c.f[0]);
        let below = abi::c_lt_s(c, rate, threshold);
        frame.save_ra(c, 0);
        let mut accepted = false;
        if !below {
            c.cycles += 5;
            Callee {
                addr: 0x0028_2cb0,
                f: ee::s07::f_00282cb0,
            }
            .call(c, 0x0028_24c0)?;
            c.g[A0][0] = c.g[V0][0];
            accepted = c.g[A0][0] != 0;
        }
        if accepted {
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] == c.g[V0][0] {
                Addr(c.g[A0][0] as u32).off(0x18).write_fpu(c, rate);
            } else {
                c.cycles += 4;
                c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
                abi::set_fpr(c, 12, rate);
                Callee {
                    addr: 0x0027_4808,
                    f: ee::s07::f_00274808,
                }
                .call(c, 0x0028_24ec)?;
                c.g[V0][0] = 1;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptEngine_SetProcessUserData` (0x282510, `cBXScriptEngine_SetRequestUserData1`):
/// swaps the user word (`+0xc`) of the process of handle `a1` for `a2`, and
/// returns the old word.
pub fn script_engine_set_process_user_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        Callee {
            addr: 0x0028_2cb0,
            f: ee::s07::f_00282cb0,
        }
        .call(c, 0x0028_2524)?;
        let process = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(process).off(0xc).lw(c);
        let value = c.g[S0][0];
        Addr(process).off(0xc).sw(c, value);
        frame.restore(c, 16, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptEngine_PauseAllSeqs` (0x282698): pauses the in-sequence object at
/// `self->+0x2b8`.
pub fn script_engine_pause_all_seqs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x2b8).lw(c);
        Callee {
            addr: 0x0027_2bf8,
            f: ee::s07::f_00272bf8,
        }
        .call(c, 0x0028_26a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptEngine_ResumeAllSeqs` (0x2826b8): resumes the in-sequence object at
/// `self->+0x2b8`.
pub fn script_engine_resume_all_seqs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x2b8).lw(c);
        Callee {
            addr: 0x0027_2c70,
            f: ee::s07::f_00272c70,
        }
        .call(c, 0x0028_26c8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptEngine_GetProcessScriptId` (0x2827f8, `cBXScriptEngine_GetRequestScriptIndex`):
/// the script index of the process of handle `a1`: the loading index in
/// state 1, else the anim script index. -1 without a process.
pub fn script_engine_get_process_script_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0028_2cb0,
            f: ee::s07::f_00282cb0,
        }
        .call(c, 0x0028_2808)?;
        c.g[A0][0] = c.g[V0][0];
        if c.g[A0][0] != 0 {
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x14).lw(c);
            } else {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x8).lw(c);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptEngine_GetProcessUserData` (0x282bc0, `cBXScriptEngine_GetRequestUserData1`):
/// the user word (`+0xc`) of the process of handle `a1`.
pub fn script_engine_get_process_user_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0028_2cb0,
            f: ee::s07::f_00282cb0,
        }
        .call(c, 0x0028_2bd0)?;
        c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptMovie_Start` (0x2839a8, `cVidEngine_PlayVideo`): with no movie
/// player returns 0, and 1 when one is already playing. Otherwise it starts
/// the player, pushes the pause state (10) on the pause stack, saves the TV
/// display format, moves the cut-scene manager to regular mode (remembering
/// whether it had to) and pauses the SFX. Returns 1.
pub fn script_movie_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let movie = c.g[S0][0] as u32;
        c.g[A0][0] = Addr(movie).lw(c);
        c.g[V0][0] = 0;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(movie).off(0x4).lw(c);
            let playing = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if !playing {
                start_movie(c)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The starting half of [`script_movie_start`]: the player is in `a0` and
/// `s0` is the movie object.
fn start_movie(c: &mut Cpu) -> Flow {
    let movie = c.g[S0][0] as u32;
    let gp = c.g[GP][0] as u32;
    c.cycles += 23;
    call_ptr(c, 0x0025_3860, 0x0028_39d8)?;
    c.g[A1][0] = Addr(gp.wrapping_sub(0x69c)).lw(c);
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
    c.g[A3][0] = 1;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    let playing_flag = c.g[A3][0];
    Addr(movie).off(0x4).sw(c, playing_flag);
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    c.g[A2][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = 0xa;
    let stack_slot = c.g[V0][0] as u32;
    let pause_state = c.g[A0][0];
    Addr(stack_slot).sw(c, pause_state);
    c.g[V1][0] = 0x440000;
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2918)).lw(c);
    c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x84).lw(c);
    let saved_flags = c.g[V0][0];
    Addr(gp.wrapping_sub(0x6a0)).sw(c, saved_flags);
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x64).lw(c);
    let stack_index = c.g[A1][0];
    Addr(gp.wrapping_sub(0x69c)).sw(c, stack_index);
    Addr(movie).off(0x14).sw(c, 0);
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
    let state_zero = c.g[V1][0] == 0;
    c.g[V0][0] = 2;
    if !state_zero {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            Addr(movie).off(0x14).sw(c, playing_flag);
        }
    }
    c.cycles += 22;
    call_ptr(c, 0x002e_a820, 0x0028_3a3c)?;
    c.g[A1][0] = Addr(gp.wrapping_sub(0x854)).lw(c);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x10d8).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x150).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x154).lw(c);
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x0028_3a54)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A1][0] = 0;
    c.g[A2][0] = Addr(gp.wrapping_sub(0x854)).lw(c);
    let display_format = c.g[V1][0];
    Addr(movie).off(0x10).sw(c, display_format);
    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x140).lh(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x144).lw(c);
    let set_method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, set_method, 0x0028_3a78)?;
    Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    }
    .call(c, 0x0028_3a80)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0029_ce28, 0x0028_3a88)?;
    c.g[V0][0] = 1;
    Ok(())
}

/// `ScriptViewObj_dtor` (0x2848a0): sets the view's two vtables and runs the
/// base `ScriptObj` destructor.
pub fn script_view_obj_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x2360));
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x2398));
        frame.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        let script_vtbl = c.g[V0][0];
        Addr(this).off(0x10).sw(c, script_vtbl);
        let vtbl = c.g[V1][0];
        Addr(this).off(0xc).sw(c, vtbl);
        Callee {
            addr: 0x0028_3228,
            f: ee::s07::f_00283228,
        }
        .call(c, 0x0028_48c4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The destructors of the `ScriptObj`-derived objects (0x2848d8, 0x284950,
/// 0x284ae8): set the object's two vtables, run the base `ScriptObj`
/// destructor on `obj+0xc`, and free the object when bit 0 of `flags` is set.
/// The three differ only in the vtable offsets and return addresses.
fn script_obj_derived_dtor(
    c: &mut Cpu,
    script_vtbl_off: u32,
    vtbl_off: u32,
    base_ret: u32,
    delete_ret: u32,
) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, 16, 0x20);
        c.g[V1][0] = 0x480000;
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(script_vtbl_off));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(vtbl_off));
        let this = c.g[S1][0] as u32;
        let script_vtbl = c.g[V0][0];
        Addr(this).off(0x18).sw(c, script_vtbl);
        c.g[A0][0] = sx(this.wrapping_add(0xc));
        let vtbl = c.g[V1][0];
        Addr(this).sw(c, vtbl);
        c.g[A1][0] = 0;
        Callee {
            addr: 0x0028_3228,
            f: ee::s07::f_00283228,
        }
        .call(c, base_ret)?;
        let delete = (c.g[S0][0] & 0x1) != 0;
        frame.restore(c, 16, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, delete_ret)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptDummyObj_dtor` (0x2848d8).
pub fn script_dummy_obj_dtor(c: &mut Cpu) {
    script_obj_derived_dtor(c, 0x2318, 0x22e0, 0x0028_4914, 0x0028_4928);
}

/// `ScriptAudioObj_dtor` (0x284950).
pub fn script_audio_obj_dtor(c: &mut Cpu) {
    script_obj_derived_dtor(c, 0x2298, 0x2260, 0x0028_498c, 0x0028_49a0);
}

/// `RiderScriptObj_dtor` (0x284ae8).
pub fn rider_script_obj_dtor(c: &mut Cpu) {
    script_obj_derived_dtor(c, 0x2018, 0x1fd8, 0x0028_4b24, 0x0028_4b38);
}

/// The destructors that only forward to the base `ScriptObj` destructor
/// (0x284a28, 0x284aa8), with a 0x10 frame holding `ra`.
fn script_obj_base_dtor_forward(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0028_3228,
            f: ee::s07::f_00283228,
        }
        .call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptObjCam_dtor` (0x284a28).
pub fn script_obj_cam_dtor(c: &mut Cpu) {
    script_obj_base_dtor_forward(c, 0x0028_4a38);
}

/// `cTransitionMan_ScriptDtor` (0x284aa8).
pub fn c_transition_man_script_dtor(c: &mut Cpu) {
    script_obj_base_dtor_forward(c, 0x0028_4ab8);
}

/// `ScriptEngine_GlobalCtors` (0x284b80): the static initialisation of the
/// scripter's globals (priority 1, key 0xffff).
pub fn script_engine_global_ctors(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0028_3e60,
            f: ee::s07::f_00283e60,
        }
        .call(c, 0x0028_4b94)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `GameLink_GetTicks` (0x2c28a0): the EA timer's tick count.
pub fn game_link_get_ticks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_4e98, 0x002c_28b0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `GameLink_Alloc(size, zeroed)` (0x2c28c0, the reference's
/// `func_002C28C0`): `operator new` of `size` bytes named "GAMELINK" with the
/// zero-fill flag `0x100` (0 when `zeroed` is 0).
pub fn game_link_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = 0x100;
        let frame = Frame::open(c, 0x10);
        if c.g[A2][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6030));
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002c_28e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fixture gate: the fakes below run their own code only when this word
    /// holds [`OC554_MAGIC`] in the test machine; otherwise they run the body
    /// they replaced (shared test binary).
    const OC554_GATE: u32 = DATA + 0xf_0000;
    /// The process handle lookup fake returns this pointer (0 for none).
    const OC554_CELL: u32 = DATA + 0xf_0004;
    const OC554_MAGIC: u32 = 0x0554_0ca5;

    /// A gated fake: logs the call through the difftest fakes (arguments and
    /// registers reach the RAM log) when the gate is on.
    macro_rules! oc554_gated {
        ($fake:ident, $prev:ident, $install:ident, $slot:path, $generated:path, $target:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC554_GATE) != OC554_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
            fn $install() {
                fakes::hook(addr_of_mut!($slot), $fake, &$prev);
            }
        };
    }

    oc554_gated!(
        oc554_fake_set_binding,
        OC554_PREV_SET_BINDING,
        oc554_install_set_binding,
        ee::s07::OVERRIDE_0027D688,
        ee::s07::f_0027d688__generated,
        0x0027_d688
    );
    oc554_gated!(
        oc554_fake_set_owner,
        OC554_PREV_SET_OWNER,
        oc554_install_set_owner,
        ee::s07::OVERRIDE_0027D680,
        ee::s07::f_0027d680__generated,
        0x0027_d680
    );
    oc554_gated!(
        oc554_fake_stop_all,
        OC554_PREV_STOP_ALL,
        oc554_install_stop_all,
        ee::s07::OVERRIDE_00282390,
        ee::s07::f_00282390__generated,
        0x0028_2390
    );
    oc554_gated!(
        oc554_fake_kill_all,
        OC554_PREV_KILL_ALL,
        oc554_install_kill_all,
        ee::s07::OVERRIDE_002823F0,
        ee::s07::f_002823f0__generated,
        0x0028_23f0
    );
    oc554_gated!(
        oc554_fake_remove_all,
        OC554_PREV_REMOVE_ALL,
        oc554_install_remove_all,
        ee::s07::OVERRIDE_0027D050,
        ee::s07::f_0027d050__generated,
        0x0027_d050
    );
    oc554_gated!(
        oc554_fake_in_seq_dtor,
        OC554_PREV_IN_SEQ_DTOR,
        oc554_install_in_seq_dtor,
        ee::s07::OVERRIDE_002728D0,
        ee::s07::f_002728d0__generated,
        0x0027_28d0
    );
    oc554_gated!(
        oc554_fake_reset_table,
        OC554_PREV_RESET_TABLE,
        oc554_install_reset_table,
        ee::s07::OVERRIDE_002828B8,
        ee::s07::f_002828b8__generated,
        0x0028_28b8
    );
    oc554_gated!(
        oc554_fake_mem_free,
        OC554_PREV_MEM_FREE,
        oc554_install_mem_free,
        ee::s10::OVERRIDE_00317E98,
        ee::s10::f_00317e98__generated,
        0x0031_7e98
    );
    oc554_gated!(
        oc554_fake_delete,
        OC554_PREV_DELETE,
        oc554_install_delete,
        ee::s10::OVERRIDE_00317E50,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );
    oc554_gated!(
        oc554_fake_script_obj_dtor,
        OC554_PREV_SCRIPT_OBJ_DTOR,
        oc554_install_script_obj_dtor,
        ee::s07::OVERRIDE_00283228,
        ee::s07::f_00283228__generated,
        0x0028_3228
    );
    oc554_gated!(
        oc554_fake_pause_all,
        OC554_PREV_PAUSE_ALL,
        oc554_install_pause_all,
        ee::s07::OVERRIDE_00272BF8,
        ee::s07::f_00272bf8__generated,
        0x0027_2bf8
    );
    oc554_gated!(
        oc554_fake_resume_all,
        OC554_PREV_RESUME_ALL,
        oc554_install_resume_all,
        ee::s07::OVERRIDE_00272C70,
        ee::s07::f_00272c70__generated,
        0x0027_2c70
    );
    oc554_gated!(
        oc554_fake_set_speed,
        OC554_PREV_SET_SPEED,
        oc554_install_set_speed,
        ee::s07::OVERRIDE_00274808,
        ee::s07::f_00274808__generated,
        0x0027_4808
    );
    oc554_gated!(
        oc554_fake_async_play,
        OC554_PREV_ASYNC_PLAY,
        oc554_install_async_play,
        ee::s06::OVERRIDE_00253860,
        ee::s06::f_00253860__generated,
        0x0025_3860
    );
    oc554_gated!(
        oc554_fake_force_off,
        OC554_PREV_FORCE_OFF,
        oc554_install_force_off,
        ee::s09::OVERRIDE_002EA820,
        ee::s09::f_002ea820__generated,
        0x002e_a820
    );
    oc554_gated!(
        oc554_fake_pause_sfx,
        OC554_PREV_PAUSE_SFX,
        oc554_install_pause_sfx,
        ee::s08::OVERRIDE_0029CE28,
        ee::s08::f_0029ce28__generated,
        0x0029_ce28
    );
    oc554_gated!(
        oc554_fake_audio_instance,
        OC554_PREV_AUDIO_INSTANCE,
        oc554_install_audio_instance,
        ee::s07::OVERRIDE_0028B180,
        ee::s07::f_0028b180__generated,
        0x0028_b180
    );
    oc554_gated!(
        oc554_fake_ticks,
        OC554_PREV_TICKS,
        oc554_install_ticks,
        ee::s14::OVERRIDE_003E4E98,
        ee::s14::f_003e4e98__generated,
        0x003e_4e98
    );
    oc554_gated!(
        oc554_fake_operator_new,
        OC554_PREV_OPERATOR_NEW,
        oc554_install_operator_new,
        ee::s10::OVERRIDE_00317E30,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );

    /// `ScriptEngine_GetProcess` (0x282cb0) returns the process named by the
    /// fixture's cell when the gate is on.
    static OC554_PREV_GET_PROCESS: AtomicUsize = AtomicUsize::new(0);
    fn oc554_fake_get_process(c: &mut Cpu) {
        if c.bus.read32(OC554_GATE) != OC554_MAGIC {
            return fakes::fall_back(c, &OC554_PREV_GET_PROCESS, ee::s07::f_00282cb0__generated);
        }
        c.g[V0][0] = c.bus.read32(OC554_CELL) as u64;
        ret(c);
    }

    /// Hooks every gated fake of this module (idempotent).
    fn oc554_install_fakes() {
        oc554_install_set_binding();
        oc554_install_set_owner();
        oc554_install_stop_all();
        oc554_install_kill_all();
        oc554_install_remove_all();
        oc554_install_in_seq_dtor();
        oc554_install_reset_table();
        oc554_install_mem_free();
        oc554_install_delete();
        oc554_install_script_obj_dtor();
        oc554_install_pause_all();
        oc554_install_resume_all();
        oc554_install_set_speed();
        oc554_install_async_play();
        oc554_install_force_off();
        oc554_install_pause_sfx();
        oc554_install_audio_instance();
        oc554_install_ticks();
        oc554_install_operator_new();
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282CB0),
            oc554_fake_get_process,
            &OC554_PREV_GET_PROCESS,
        );
    }

    /// The machine with the fakes on, and the sound-bank fixture.
    fn oc554_gate_on(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc554_install_fakes();
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC554_GATE, OC554_MAGIC);
        r
    }

    #[test]
    fn rider_script_obj_get_sound_bank_entry_matches() {
        check(
            ee::s07::f_00280548__generated,
            rider_script_obj_get_sound_bank_entry,
            200,
            &|c, seed| {
                let mut r = Rng(0x2805_4800 ^ seed as u64);
                scramble(c, &mut r);
                let obj = DATA + 0x1000;
                let entry = DATA + 0x1100;
                c.bus.write32(obj + 0x20, entry);
                c.bus.write32(entry + 4, r.next_u32());
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = (DATA + 0x1200) as u64;
                c.g[A3][0] = (DATA + 0x1300) as u64;
            },
        );
    }

    #[test]
    fn script_audio_obj_get_sound_bank_entry_matches() {
        check(
            ee::s07::f_00281000__generated,
            script_audio_obj_get_sound_bank_entry,
            200,
            &|c, seed| {
                let mut r = Rng(0x2810_0000 ^ seed as u64);
                scramble(c, &mut r);
                let obj = DATA + 0x1000;
                let entry = DATA + 0x1400;
                c.bus.write32(obj + 0xf8, entry);
                c.bus.write32(entry + 4, r.next_u32());
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = (DATA + 0x1500) as u64;
                c.g[A3][0] = (DATA + 0x1504) as u64;
            },
        );
    }

    #[test]
    fn script_view_obj_remove_cam_matches() {
        check(
            ee::s07::f_002811e8__generated,
            script_view_obj_remove_cam,
            300,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0003);
                let view = DATA + 0x8000;
                let cams = [DATA + 0x8200, DATA + 0x8400];
                for (i, cam) in cams.iter().enumerate() {
                    let slot = if (seed >> i) & 1 == 0 { *cam } else { 0 };
                    c.bus.write32(view + 0xd0 + 4 * i as u32, slot);
                    c.bus.write32(*cam + 4, 1 + r.next_u32() % 3);
                    let entity = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        DATA + 0x8800
                    };
                    c.bus.write32(*cam + 0x14, entity);
                }
                c.bus
                    .write32(view + 0x8, [0, 1, u32::MAX, 2][(seed % 4) as usize]);
                c.bus
                    .write32(view + 0xdc, [u32::MAX, 0, 1][r.next_u32() as usize % 3]);
                c.bus
                    .write32(view + 0xe0, [u32::MAX, 0, 1][r.next_u32() as usize % 3]);
                let key = DATA + 0x8600;
                c.bus.write32(key + 4, 1 + r.next_u32() % 3);
                c.g[A0][0] = view as u64;
                c.g[A1][0] = key as u64;
            },
        );
    }

    #[test]
    fn script_engine_destroy_script_database_matches() {
        check(
            ee::s07::f_00282020__generated,
            script_engine_destroy_script_database,
            200,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0004);
                let this = DATA + 0x9000;
                let db = if seed % 3 == 0 { 0 } else { DATA + 0xa000 };
                let in_seq = if seed % 4 == 0 { 0 } else { DATA + 0xa100 };
                let heap = if seed % 2 == 0 { 0 } else { DATA + 0xa200 };
                c.bus.write32(this + 0x2b0, db);
                c.bus.write32(this + 0x2b4, r.next_u32());
                c.bus.write32(this + 0x2b8, in_seq);
                c.bus.write32(this + 0x2bc, heap);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn script_engine_set_process_speed_matches() {
        check(
            ee::s07::f_00282498__generated,
            script_engine_set_process_speed,
            300,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0005);
                c.bus.write32(GAME_GP - 0x4784, 0.01f32.to_bits());
                let rate = [0.001f32, 0.5, 0.01, 2.0][(seed % 4) as usize];
                c.f[12] = rate.to_bits();
                let process = DATA + 0xb000;
                c.bus
                    .write32(OC554_CELL, if seed % 5 == 0 { 0 } else { process });
                c.bus.write32(process, DATA + 0xb200);
                c.bus
                    .write32(process + 0x4, if seed % 2 == 0 { 1 } else { 2 });
                c.bus.write32(process + 0x18, r.next_u32());
                c.g[A0][0] = (DATA + 0xb100) as u64;
                c.g[A1][0] = 7;
            },
        );
    }

    #[test]
    fn script_engine_set_process_user_data_matches() {
        check(
            ee::s07::f_00282510__generated,
            script_engine_set_process_user_data,
            200,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0006);
                let process = DATA + 0xb300;
                c.bus.write32(OC554_CELL, process);
                c.bus.write32(process + 0xc, r.next_u32());
                c.g[A0][0] = (DATA + 0xb100) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn script_engine_pause_and_resume_all_seqs_match() {
        let setup = |c: &mut Cpu, seed: u32| {
            let _ = oc554_gate_on(c, seed, 0x554_0007);
            c.g[A0][0] = (DATA + 0xb400) as u64;
            c.bus.write32(DATA + 0xb400 + 0x2b8, DATA + 0xb500);
        };
        check(
            ee::s07::f_00282698__generated,
            script_engine_pause_all_seqs,
            100,
            &setup,
        );
        check(
            ee::s07::f_002826b8__generated,
            script_engine_resume_all_seqs,
            100,
            &setup,
        );
    }

    #[test]
    fn script_engine_get_process_script_id_matches() {
        check(
            ee::s07::f_002827f8__generated,
            script_engine_get_process_script_id,
            300,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0008);
                let process = DATA + 0xb600;
                c.bus
                    .write32(OC554_CELL, if seed % 4 == 0 { 0 } else { process });
                c.bus.write32(process + 0x4, [1, 2, 3][(seed % 3) as usize]);
                c.bus.write32(process + 0x8, r.next_u32());
                c.bus.write32(process + 0x14, r.next_u32());
                c.g[A1][0] = 5;
            },
        );
    }

    #[test]
    fn script_engine_get_process_user_data_matches() {
        check(
            ee::s07::f_00282bc0__generated,
            script_engine_get_process_user_data,
            200,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0009);
                let process = DATA + 0xb700;
                c.bus.write32(OC554_CELL, process);
                c.bus.write32(process + 0xc, r.next_u32());
                c.g[A1][0] = 5;
            },
        );
    }

    #[test]
    fn script_movie_start_matches() {
        check(ee::s07::f_002839a8__generated, script_movie_start, 300, &|c, seed| {
            let mut r = oc554_gate_on(c, seed, 0x554_0011);
            let movie = DATA + 0x7000;
            let grman = DATA + 0x5000;
            let vt = DATA + 0x6400;
            let world = DATA + 0x5400;
            let game = DATA + 0x5800;
            let cut = DATA + 0x5c00;
            c.bus
                .write32(movie, if seed % 5 == 0 { 0 } else { DATA + 0x7100 });
            c.bus
                .write32(movie + 0x4, if seed % 3 == 0 { 1 } else { 0 });
            c.bus.write32(movie + 0x14, r.next_u32());
            c.bus.write32(GAME_GP - 0x854, grman);
            c.bus.write32(grman + 0x10d8, vt);
            c.bus.write16(vt + 0x140, 0x20);
            c.bus.write32(vt + 0x144, fakes::FAKE);
            c.bus.write16(vt + 0x150, 0x10);
            c.bus.write32(vt + 0x154, fakes::FAKE);
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, game);
            c.bus.write32(game + 0x64, cut);
            c.bus.write32(cut + 0x10, seed % 4);
            c.bus.write32(GAME_GP - 0x69c, seed % 4);
            c.g[A0][0] = movie as u64;
        });
    }

    #[test]
    fn script_view_obj_dtor_matches() {
        check(
            ee::s07::f_002848a0__generated,
            script_view_obj_dtor,
            100,
            &|c, seed| {
                let mut r = oc554_gate_on(c, seed, 0x554_0012);
                c.g[A0][0] = (DATA + 0xd000) as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn script_obj_derived_dtors_match() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc554_gate_on(c, seed, 0x554_0013);
            c.g[A0][0] = (DATA + 0xd100) as u64;
            // bit 0 (free the object) on every other seed
            c.g[A1][0] = ((r.next_u32() & !1) | (seed & 1)) as u64;
        };
        check(ee::s07::f_002848d8__generated, script_dummy_obj_dtor, 100, &setup);
        check(ee::s07::f_00284950__generated, script_audio_obj_dtor, 100, &setup);
        check(ee::s07::f_00284ae8__generated, rider_script_obj_dtor, 100, &setup);
    }

    #[test]
    fn script_obj_base_dtors_match() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc554_gate_on(c, seed, 0x554_0014);
            c.g[A0][0] = (DATA + 0xd200) as u64;
            c.g[A1][0] = r.next_u32() as u64;
        };
        check(ee::s07::f_00284a28__generated, script_obj_cam_dtor, 100, &setup);
        check(
            ee::s07::f_00284aa8__generated,
            c_transition_man_script_dtor,
            100,
            &setup,
        );
    }

    #[test]
    fn script_engine_global_ctors_matches() {
        check(
            ee::s07::f_00284b80__generated,
            script_engine_global_ctors,
            20,
            &|c, seed| {
                let mut r = Rng(0x284b_8000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
            },
        );
    }

    #[test]
    fn game_link_get_ticks_matches() {
        check(ee::s08::f_002c28a0__generated, game_link_get_ticks, 100, &|c, seed| {
            let mut r = oc554_gate_on(c, seed, 0x554_0015);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn game_link_alloc_matches() {
        check(ee::s08::f_002c28c0__generated, game_link_alloc, 200, &|c, seed| {
            let mut r = oc554_gate_on(c, seed, 0x554_0016);
            c.g[A1][0] = (r.next_u32() % 0x1000) as u64;
            c.g[A2][0] = if seed % 2 == 0 {
                0
            } else {
                (r.next_u32() as u64) | 1
            };
        });
    }
}
