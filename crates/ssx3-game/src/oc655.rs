//! Staging module oc655 (wave 234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc655`.

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
    (0x0027_2aa0, in_seq_run_to_end, "InSeq_RunToEnd"),
    (0x0027_3f68, in_seq_channel_restart, "InSeq_ChannelRestart"),
    (0x0027_66d0, script_list_skip, "ScriptList_Skip"),
    (0x0030_8118, wscript_process_load_state, "WScriptProcess_LoadState"),
    (0x0030_94c8, wscript_man_create_process, "WScriptMan_CreateProcess"),
    (
        0x0030_9618,
        wscript_man_delete_process_by_id,
        "WScriptMan_DeleteProcessById",
    ),
    (
        0x0030_b590,
        wscript_man_request_mission_playback,
        "WScriptMan_RequestMissionPlayback",
    ),
];

// Callees, by their generated shard functions (each goes through its override
// slot, so a replaced callee is reached too).

/// `InSeq_GetInstance` (0x272cc0): the singleton sequencer.
const IS_GET_INSTANCE: Callee = Callee {
    addr: 0x0027_2cc0,
    f: ee::s07::f_00272cc0,
};
/// `InSeq_InstanceRunToEnd` (0x274868).
const IS_INSTANCE_RUN_TO_END: Callee = Callee {
    addr: 0x0027_4868,
    f: ee::s07::f_00274868,
};

/// `ScriptCmdQueue_Count` (0x283d28).
const SLK_CMDQ_COUNT: Callee = Callee {
    addr: 0x0028_3d28,
    f: ee::s07::f_00283d28,
};
/// `ScriptList_PopEntries` (0x277598).
const SLK_POP_ENTRIES: Callee = Callee {
    addr: 0x0027_7598,
    f: ee::s07::f_00277598,
};
/// `ScriptList_Prepare` (0x276270).
const SLK_LIST_PREPARE: Callee = Callee {
    addr: 0x0027_6270,
    f: ee::s07::f_00276270,
};
/// `ScriptCmdQueue_At` (0x283d70).
const SLK_CMDQ_AT: Callee = Callee {
    addr: 0x0028_3d70,
    f: ee::s07::f_00283d70,
};
/// `ScriptList_IsMovieEntry` (0x277dd8).
const SLK_IS_MOVIE: Callee = Callee {
    addr: 0x0027_7dd8,
    f: ee::s07::f_00277dd8,
};
/// `ScriptEngine_KillProcess` (0x282540).
const SLK_KILL_PROCESS: Callee = Callee {
    addr: 0x0028_2540,
    f: ee::s07::f_00282540,
};
/// `ScriptMovie_Stop` (0x283818).
const SLK_MOVIE_STOP: Callee = Callee {
    addr: 0x0028_3818,
    f: ee::s07::f_00283818,
};
/// `ScriptList_SetPaused` (0x277c08).
const SLK_SET_PAUSED: Callee = Callee {
    addr: 0x0027_7c08,
    f: ee::s07::f_00277c08,
};
/// `ScriptList_PrepareEntry` (0x277450).
const SLK_PREPARE_ENTRY: Callee = Callee {
    addr: 0x0027_7450,
    f: ee::s07::f_00277450,
};
/// `ScriptList_Stop` (0x276868).
const SLK_LIST_STOP: Callee = Callee {
    addr: 0x0027_6868,
    f: ee::s07::f_00276868,
};
/// `ScriptList_StartTransition` (0x277980).
const SLK_START_TRANSITION: Callee = Callee {
    addr: 0x0027_7980,
    f: ee::s07::f_00277980,
};
/// `ScriptList_TryAdvance` (0x277800).
const SLK_TRY_ADVANCE: Callee = Callee {
    addr: 0x0027_7800,
    f: ee::s07::f_00277800,
};

/// `ScriptEngine_ListAddChoice` (0x278e50).
const QFS_ADD_CHOICE: Callee = Callee {
    addr: 0x0027_8e50,
    f: ee::s07::f_00278e50,
};
/// `ScriptEngine_ListPrepare` (0x278f38).
const QFS_NIS_PREPARE: Callee = Callee {
    addr: 0x0027_8f38,
    f: ee::s07::f_00278f38,
};
/// `ScriptEngine_ListStart` (0x278f68).
const QFS_NIS_START: Callee = Callee {
    addr: 0x0027_8f68,
    f: ee::s07::f_00278f68,
};
/// `SSXAudioSystem_GetInstance` (0x28b180).
const QFS_AUDIO_GET: Callee = Callee {
    addr: 0x0028_b180,
    f: ee::s07::f_0028b180,
};
/// `SSXAudioSystem_HandleMusicEvent` (0x28e8c0).
const QFS_MUSIC_EVENT: Callee = Callee {
    addr: 0x0028_e8c0,
    f: ee::s07::f_0028e8c0,
};

/// `WScriptTask_LoadState` (0x308798).
const WPS_TASK_LOAD: Callee = Callee {
    addr: 0x0030_8798,
    f: ee::s10::f_00308798,
};
/// `EA_MemAllocNamed` (0x317d70).
const WSM_EA_ALLOC: Callee = Callee {
    addr: 0x0031_7d70,
    f: ee::s10::f_00317d70,
};
/// `WScriptProcess_ctor` (0x307598).
const WSM_PROCESS_CTOR: Callee = Callee {
    addr: 0x0030_7598,
    f: ee::s10::f_00307598,
};
/// `WScriptProcessList_PushFront` (0x30b260).
const WSM_PUSH_FRONT: Callee = Callee {
    addr: 0x0030_b260,
    f: ee::s10::f_0030b260,
};
/// `WScriptProcessList_FindById` (0x30b320).
const WSM_FIND_BY_ID: Callee = Callee {
    addr: 0x0030_b320,
    f: ee::s10::f_0030b320,
};
/// `WScriptProcessList_Remove` (0x30b2d0).
const WSM_LIST_REMOVE: Callee = Callee {
    addr: 0x0030_b2d0,
    f: ee::s10::f_0030b2d0,
};

/// `InSeq_RunToEnd(a0, a1, flag)` (0x272aa0): runs the singleton's `RunToEnd`
/// on `InSeq_GetInstance(a0, a1)`, with `flag` (a2) passed through as the
/// last argument. Frame 0x20: saves ra and s0.
pub fn in_seq_run_to_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        IS_GET_INSTANCE.call(c, 0x0027_2ab4)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        IS_INSTANCE_RUN_TO_END.call(c, 0x0027_2ac0)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The entity's `mIsPlaying` flag (0x28), `mpSystemHeader` (0x1c),
/// `mpChannelPlayers` (0x24) and `mpEntityComm` (0x2c) of the channel restart.
const ICR_HEADER: u32 = 0x1c;
const ICR_PLAYERS: u32 = 0x24;
const ICR_PLAYING: u32 = 0x28;
const ICR_COMM: u32 = 0x2c;

/// The sequencer `self` pointer, as the register holds it (s1, read after
/// each call).
#[inline(always)]
fn icr_self(c: &Cpu) -> u32 {
    c.g[S1][0] as u32
}

/// `u32 InSeq_ChannelRestart(self)` (0x273f68): restarts a playing entity.
/// Stops each channel player, plays its channel record again, and tells the
/// entity comm about the events the channels raised. Returns whether any
/// channel changed (0 if the entity is not playing). Frame 0xa0.
pub fn in_seq_channel_restart(c: &mut Cpu) {
    guest(c, icr_body)
}

fn icr_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0xa0);
    frame.save(c, 17, 0x80);
    frame.save(c, 16, 0x90);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, 18, 0x70);
    frame.save(c, 19, 0x60);
    frame.save(c, 20, 0x50);
    frame.save(c, 21, 0x40);
    frame.save(c, 22, 0x30);
    frame.save(c, 23, 0x20);
    frame.save_ra(c, 0x10);

    c.g[V0][0] = Addr(icr_self(c).wrapping_add(ICR_PLAYING)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
        icr_channels(c)?;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }

    c.cycles += 11;
    frame.restore(c, 16, 0x90);
    frame.restore(c, 17, 0x80);
    frame.restore(c, 18, 0x70);
    frame.restore(c, 19, 0x60);
    frame.restore(c, 20, 0x50);
    frame.restore(c, 21, 0x40);
    frame.restore(c, 22, 0x30);
    frame.restore(c, 23, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// The channel loop and the entity callbacks, from the system header in V0.
/// The loop stops and replays each channel (`S3` = index, `S2` = channel
/// record, `S6` = event mask, `S7` = changed); then `OnInSeqKeyEvent` when
/// there are events and `OnInSeqUpdate` for the entity comm.
fn icr_channels(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[S6][0] = 0;
    c.g[S3][0] = 0;
    c.g[S7][0] = 0;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lhu(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(2)).lbu(c);
    let no_channels = c.g[A0][0] == 0;
    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));

    if !no_channels {
        c.cycles += 1;
        loop {
            c.cycles += 26;
            c.g[V1][0] = Addr(icr_self(c).wrapping_add(ICR_PLAYERS)).lw(c);
            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[S4][0] = c.g[S2][0];
            c.g[S5][0] = 0;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
            c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x10)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
            let stop = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, stop, 0x0027_3ff8)?; // player->Stop()

            c.g[V0][0] = Addr(icr_self(c).wrapping_add(ICR_PLAYERS)).lw(c);
            c.g[A1][0] = c.g[S2][0];
            Addr(sp(c)).sw(c, 0); // channelEvents = 0
            c.g[A2][0] = c.g[S1][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A3][0] = c.g[SP][0];
            c.g[T0][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
            let play = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, play, 0x0027_4028)?; // player->Play(data, self, &events)

            if c.g[V0][0] != 0 {
                c.g[S5][0] = 1;
                c.cycles += 1;
                c.g[A0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
            } else {
                c.cycles += 2;
                if c.g[S7][0] == 0 {
                    c.g[A0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
                } else {
                    c.cycles += 1;
                    c.g[S5][0] = 1;
                    c.cycles += 1;
                    c.g[A0][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
                }
            }

            c.cycles += 9;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
            c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(2)).lhu(c);
            c.g[S7][0] = c.g[S5][0];
            c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(2)).lbu(c);
            c.g[V1][0] = Addr(sp(c)).lw(c);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S6][0] |= c.g[V1][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0027_3fc8);
                }
                continue;
            }
            break;
        }
    }

    c.cycles += 3;
    c.g[T1][0] = Addr(icr_self(c).wrapping_add(ICR_COMM)).lw(c);
    let no_comm = c.g[T1][0] == 0;
    c.g[V0][0] = c.g[S7][0];
    if !no_comm {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        if c.g[S6][0] != 0 {
            c.cycles += 9;
            c.g[V0][0] = Addr(c.g[T1][0] as u32).lw(c);
            c.g[V1][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
            c.g[A3][0] = c.g[S6][0];
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
            c.g[T0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
            c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(3)).lbu(c);
            c.mtc1(0, 12);
            let key_event = c.g[T0][0] as u32;
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, key_event, 0x0027_409c)?; // comm->OnInSeqKeyEvent
        }
        c.cycles += 13;
        c.g[T0][0] = Addr(icr_self(c).wrapping_add(ICR_COMM)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[T1][0] = Addr(icr_self(c).wrapping_add(ICR_HEADER)).lw(c);
        c.g[A3][0] = c.g[S6][0];
        c.g[V0][0] = Addr(c.g[T0][0] as u32).lw(c);
        c.mtc1(0, 12);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
        c.mov_s(13, 12);
        c.g[A2][0] = Addr((c.g[T1][0] as u32).wrapping_add(3)).lbu(c);
        let update = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, update, 0x0027_40cc)?; // comm->OnInSeqUpdate
        c.g[V0][0] = c.g[S7][0];
    }
    Ok(())
}

/// `void ScriptList_Skip(list, delta, force)` (0x2766d0): moves the list's
/// skip position by `delta` and advances the list to the new position: the
/// queue entries before it are removed, the next pending script is set up,
/// and the transition or the play-on-available path runs. Returns the state.
/// Frame 0x50: saves ra and s0-s3.
pub fn script_list_skip(c: &mut Cpu) {
    guest(c, slk_body)
}

fn slk_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 16, 0x40);
    frame.save(c, 17, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, 19, 0x10);
    frame.save(c, 18, 0x20);
    frame.save_ra(c, 0);
    'b_27684c: {
        'b_276848: {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let t = c.g[S1][0] == c.g[V0][0];
            c.g[S3][0] = c.g[A2][0];
            if t {
                break 'b_276848;
            }
            'b_276840: {
                'b_27683c: {
                    'b_276764: {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S1][0] = c.g[V1][0];
                        }
                        SLK_CMDQ_COUNT.call(c, 0x0027_6714)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        if t {
                            break 'b_276764;
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), 0u64);
                        if t {
                            break 'b_27683c;
                        }
                        c.cycles += 8;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S0][0];
                        SLK_POP_ENTRIES.call(c, 0x0027_673c)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        let t = c.g[V1][0] == 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa4), 0u64);
                        if t {
                            break 'b_276848;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
                        c.g[A0][0] = c.g[S0][0];
                        SLK_LIST_PREPARE.call(c, 0x0027_675c)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        break 'b_27684c;
                    }
                    'b_276800: {
                        'b_2767fc: {
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            let t = (c.g[A1][0] as i64) <= 0;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S1][0]);
                            if t {
                                break 'b_2767fc;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                                break 'b_276800;
                            }
                            'b_2767c8: {
                                'b_2767b0: {
                                    c.cycles += 8;
                                    c.g[A0][0] = c.g[S2][0];
                                    SLK_CMDQ_AT.call(c, 0x0027_6784)?;
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    SLK_IS_MOVIE.call(c, 0x0027_6794)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                        break 'b_2767b0;
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    SLK_KILL_PROCESS.call(c, 0x0027_67a8)?;
                                    c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                                    break 'b_2767c8;
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                                if t {
                                    break 'b_2767c8;
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                                SLK_MOVIE_STOP.call(c, 0x0027_67c4)?;
                                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                            }
                            'b_2767e8: {
                                c.cycles += 5;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_2767e8;
                                }
                                c.cycles += 3;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x1;
                                SLK_SET_PAUSED.call(c, 0x0027_67e8)?;
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0u64;
                            SLK_PREPARE_ENTRY.call(c, 0x0027_67f8)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa8));
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                        break 'b_27684c;
                    }
                    'b_276824: {
                        c.cycles += 2;
                        if c.g[S3][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            break 'b_276824;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        SLK_START_TRANSITION.call(c, 0x0027_6818)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                            break 'b_27684c;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    }
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_276840;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    SLK_TRY_ADVANCE.call(c, 0x0027_6834)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    break 'b_27684c;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                break 'b_276840;
            }
            c.cycles += 2;
            c.g[A1][0] = 0x1;
            SLK_LIST_STOP.call(c, 0x0027_6848)?;
            break 'b_276848;
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
        break 'b_27684c;
    }
    c.cycles += 7;
    frame.restore(c, 16, 0x40);
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void ScriptEngine_QueueFinishScripts(engine, online, intro)` (0x27a860):
/// queues the transport sequence's NIS scripts (the intro first when `intro`
/// is set; the online and offline variants differ), readies and starts the
/// list, and plays the music event 0x14. The world mode check
/// (`world->game->0x0c->0x7c == 1`) picks the second script of each branch.
/// Frame 0x20: saves ra and s0.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn script_engine_queue_finish_scripts(c: &mut Cpu) {
    guest(c, qfs_body)
}

fn qfs_body(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    let frame = Frame::open(c, 0x20);
    'b_27a9ac: {
        'b_27a98c: {
            'b_27a974: {
                'b_27a8f8: {
                    frame.save(c, 16, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A1][0] == 0u64;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'b_27a8f8;
                    }
                    'b_27a898: {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_27a898;
                        }
                        c.cycles += 7;
                        c.g[A2][0] = 0x12;
                        c.g[A3][0] = 0x1;
                        c.g[T0][0] = 0x1;
                        c.g[T1][0] = 0xffff_ffff_ffff_ffff;
                        c.g[T2][0] = 0xffff_ffff_ffff_ffff;
                        c.g[T3][0] = 0xffff_ffff_ffff_ffff;
                        QFS_ADD_CHOICE.call(c, 0x0027_a898)?;
                    }
                    'b_27a8e8: {
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x1;
                        c.g[A2][0] = 0x13;
                        c.g[A3][0] = 0u64;
                        c.g[T0][0] = 0u64;
                        c.g[T1][0] = 0xffff_ffff_ffff_ffff;
                        c.g[T2][0] = 0xffff_ffff_ffff_ffff;
                        c.g[T3][0] = 0xffff_ffff_ffff_ffff;
                        QFS_ADD_CHOICE.call(c, 0x0027_a8bc)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                        let t = c.g[A0][0] != c.g[A1][0];
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_27a8e8;
                        }
                        c.cycles += 4;
                        c.g[A2][0] = 0x14;
                        c.g[A3][0] = 0x8;
                        c.g[T0][0] = 0u64;
                        break 'b_27a974;
                    }
                    c.cycles += 4;
                    c.g[A2][0] = 0x15;
                    c.g[A3][0] = 0x8;
                    c.g[T0][0] = 0u64;
                    break 'b_27a974;
                }
                'b_27a920: {
                    c.cycles += 2;
                    let t = c.g[A2][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_27a920;
                    }
                    c.cycles += 8;
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0xc;
                    c.g[A3][0] = 0x1;
                    c.g[T0][0] = 0x1;
                    c.g[T1][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T2][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T3][0] = 0xffff_ffff_ffff_ffff;
                    QFS_ADD_CHOICE.call(c, 0x0027_a920)?;
                }
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0xd;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0x1;
                c.g[T1][0] = 0xffff_ffff_ffff_ffff;
                c.g[T2][0] = 0xffff_ffff_ffff_ffff;
                c.g[T3][0] = 0xffff_ffff_ffff_ffff;
                QFS_ADD_CHOICE.call(c, 0x0027_a944)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sw((c.g[V1][0] as u32).wrapping_add(0xf44), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                let t = c.g[A0][0] != c.g[A1][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_27a98c;
                }
                c.cycles += 3;
                c.g[A2][0] = 0xe;
                c.g[A3][0] = 0x8;
                c.g[T0][0] = 0x1;
                break 'b_27a974;
            }
            c.cycles += 6;
            c.g[T1][0] = 0xffff_ffff_ffff_ffff;
            c.g[T2][0] = 0xffff_ffff_ffff_ffff;
            c.g[T3][0] = 0xffff_ffff_ffff_ffff;
            QFS_ADD_CHOICE.call(c, 0x0027_a984)?;
            c.g[A0][0] = c.g[S0][0];
            break 'b_27a9ac;
        }
        c.cycles += 8;
        c.g[A2][0] = 0xf;
        c.g[A3][0] = 0x8;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0xffff_ffff_ffff_ffff;
        c.g[T2][0] = 0xffff_ffff_ffff_ffff;
        c.g[T3][0] = 0xffff_ffff_ffff_ffff;
        QFS_ADD_CHOICE.call(c, 0x0027_a9a8)?;
        c.g[A0][0] = c.g[S0][0];
    }
    c.cycles += 17;
    c.g[A1][0] = 0x1;
    QFS_NIS_PREPARE.call(c, 0x0027_a9b4)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0x1;
    c.g[A2][0] = 0u64;
    c.g[A3][0] = 0x1;
    QFS_NIS_START.call(c, 0x0027_a9c8)?;
    QFS_AUDIO_GET.call(c, 0x0027_a9d0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x14;
    c.g[A2][0] = 0x1;
    QFS_MUSIC_EVENT.call(c, 0x0027_a9e0)?;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int WScriptProcess_LoadState(proc, stream)` (0x308118): reads the
/// process header (16 bytes) from the replay stream, reloads its local Luno
/// table (dropping the old one), reads the task count, and loads each task
/// (`WScriptTask_LoadState`). Frame 0x60: saves ra and s0-s3.
pub fn wscript_process_load_state(c: &mut Cpu) {
    guest(c, wps_body)
}

fn wps_body(c: &mut Cpu) -> Flow {
    c.cycles += 22;
    let frame = Frame::open(c, 0x60);
    c.g[A2][0] = 0x10;
    frame.save(c, 16, 0x50);
    frame.save(c, 18, 0x30);
    frame.save(c, 19, 0x20);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, 17, 0x40);
    c.g[S3][0] = c.g[A1][0];
    frame.save_ra(c, 0x10);

    // stream->read(proc, 0x10)
    c.g[A1][0] = c.g[S2][0];
    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
    let read = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, read, 0x0030_8154)?;

    // The Luno table of the section's script cache: the old one goes.
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    call_ptr(c, 0x003a_ca38, 0x0030_8164)?; // cWScriptCache_GetData
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
    let t = c.g[A0][0] == 0u64;
    c.g[S0][0] = c.g[V0][0];
    if !t {
        c.cycles += 2;
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x0022_4df0, 0x0030_8178)?; // Luno table dtor
    }

    'b_3081d8: {
        c.cycles += 11;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), 0u64);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[V0][0] = c.lw(c.g[S3][0] as u32);
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
        let read_has = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, read_has, 0x0030_8198)?; // stream->read(&has, 4)
        c.g[V0][0] = c.lw(sp(c));
        let t = c.g[V0][0] == 0u64;
        c.g[A2][0] = 0x2000_0000;
        if t {
            break 'b_3081d8;
        }
        // A process with a local table: a fresh one, read from the stream.
        c.cycles += 13;
        c.g[A1][0] = 0x0049_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6778));
        c.g[A0][0] = 0x4;
        c.g[A3][0] = 0u64;
        WSM_EA_ALLOC.call(c, 0x0030_81b8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x0022_4da0, 0x0030_81c4)?; // Luno table ctor
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0022_59a8, 0x0030_81d8)?; // Luno table readFromReplayFrame
    }

    'b_30820c: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0u64;
        if t {
            break 'b_30820c;
        }
        c.cycles += 1;
        c.g[S1][0] = 0u64;
        loop {
            c.cycles += 9;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
            c.g[A1][0] = c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            WPS_TASK_LOAD.call(c, 0x0030_81fc)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x44));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0030_81e8);
                }
                continue;
            }
            break 'b_30820c;
        }
    }

    c.cycles += 7;
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// `void* WScriptMan_CreateProcess(man, sectionId, data, front)` (0x3094c8):
/// allocates a 0x60-byte process, constructs it, and pushes it on the add
/// list (`front` set) or the idle list. Returns the process. Frame 0x50.
pub fn wscript_man_create_process(c: &mut Cpu) {
    guest(c, wsm_create_body)
}

fn wsm_create_body(c: &mut Cpu) -> Flow {
    c.cycles += 23;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 16, 0x40);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, 17, 0x30);
    frame.save(c, 18, 0x20);
    c.g[S1][0] = c.g[A2][0];
    frame.save(c, 19, 0x10);
    c.g[S2][0] = c.g[A3][0];
    c.g[S3][0] = c.g[A0][0];
    c.g[A1][0] = 0x0049_0000;
    frame.save_ra(c, 0);

    c.g[A0][0] = 0x60;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x66f8));
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0u64;
    WSM_EA_ALLOC.call(c, 0x0030_9508)?;
    c.g[A2][0] = c.g[S0][0];
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S3][0];
    WSM_PROCESS_CTOR.call(c, 0x0030_951c)?;
    let t = c.g[S2][0] == 0u64;
    c.g[S0][0] = c.g[V0][0];
    if !t {
        c.cycles += 5;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2bc));
        c.g[A1][0] = c.g[S0][0];
        WSM_PUSH_FRONT.call(c, 0x0030_9530)?;
    } else {
        c.cycles += 4;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2b4));
        c.g[A1][0] = c.g[S0][0];
        WSM_PUSH_FRONT.call(c, 0x0030_9544)?;
    }
    c.g[V0][0] = c.g[S0][0];

    c.cycles += 7;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 16, 0x40);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int WScriptMan_DeleteProcessById(man, id)` (0x309618): finds the process
/// with `id` on the active list, else the add list, else the idle list,
/// removes it, and calls its virtual destructor with flag 3 (`vt[+0x0c]`).
/// Frame 0x50.
pub fn wscript_man_delete_process_by_id(c: &mut Cpu) {
    guest(c, wsm_delete_body)
}

fn wsm_delete_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x50);
    frame.save(c, 17, 0x30);
    frame.save(c, 18, 0x20);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, 19, 0x10);
    c.g[S2][0] = c.g[A1][0];
    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b8));
    frame.save(c, 16, 0x40);
    frame.save_ra(c, 0);

    'b_3096d8: {
        'b_309694: {
            'b_30966c: {
                c.g[A0][0] = c.g[S3][0];
                WSM_FIND_BY_ID.call(c, 0x0030_9644)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] != 0u64;
                c.g[A0][0] = c.g[S3][0];
                if t {
                    break 'b_30966c;
                }
                c.cycles += 7;
                c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2bc));
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S3][0];
                WSM_FIND_BY_ID.call(c, 0x0030_9660)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A0][0] = c.g[S3][0];
                if t {
                    break 'b_309694;
                }
            }
            // Found on the active or the add list: remove it and destroy it.
            c.cycles += 10;
            c.g[A1][0] = c.g[S0][0];
            WSM_LIST_REMOVE.call(c, 0x0030_9674)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            c.g[A1][0] = 0x3;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let destroy = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, destroy, 0x0030_968c)?;
            frame.restore(c, 16, 0x40);
            break 'b_3096d8;
        }
        // Not on the active or add list: the idle list.
        c.cycles += 7;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        WSM_FIND_BY_ID.call(c, 0x0030_96a4)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
            frame.restore(c, 16, 0x40);
            break 'b_3096d8;
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        WSM_LIST_REMOVE.call(c, 0x0030_96bc)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        c.g[A1][0] = 0x3;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let destroy = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, destroy, 0x0030_96d4)?;
        frame.restore(c, 16, 0x40);
    }

    c.cycles += 6;
    frame.restore(c, 17, 0x30);
    frame.restore(c, 18, 0x20);
    frame.restore(c, 19, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `int WScriptMan_RequestMissionPlayback(man)` (0x30b590): stops the
/// playback, resets the replay, and sets up the preview of the local mission
/// (`LocalMissionId`). On success it queues command 5 for that mission and
/// returns 1; otherwise 0. Frame 0x20: saves ra and s0.
pub fn wscript_man_request_mission_playback(c: &mut Cpu) {
    guest(c, wsm_preview_body)
}

fn wsm_preview_body(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0x20);
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    frame.save(c, 16, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];

    // SSXApp->Game->PlaybackMan->Stop(); SSXApp->Game->Replay->Reset().
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
    call_ptr(c, 0x0026_cdf8, 0x0030_b5b0)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
    call_ptr(c, 0x0026_f228, 0x0030_b5c0)?;

    // PlaybackMan->setupPreview(LocalMissionId).
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a0));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
    call_ptr(c, 0x0026_caf8, 0x0030_b5d4)?;

    if c.g[V0][0] != 0u64 {
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a0));
        c.cycles += 4;
        c.g[V0][0] = 0x5;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2ac), c.g[V0][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x2b0), c.g[V1][0]);
        c.g[V0][0] = 0x1;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0u64;
    }

    c.cycles += 4;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, fall_back, hook, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the fakes below act only while it holds `OC655_MAGIC`.
    const OC655_CTRL: u32 = DATA + 0x4_f000;
    const OC655_MAGIC: u32 = 0x6f63_0655;
    /// Bit `n` set: the fake with bit `n` returns 0 (instead of its random value).
    const OC655_ZERO: u32 = DATA + 0x4_f008;
    /// The FindById hits: bit `k` set, the list at slot `k` (idle, active,
    /// add) holds the process.
    const OC655_HITS: u32 = DATA + 0x4_f020;
    /// The process manager, and the process object a FindById hit returns.
    const OC655_MAN: u32 = DATA + 0x5_0000;
    const OC655_OBJ: u32 = DATA + 0x5_3000;
    const OC655_OBJ_VT: u32 = DATA + 0x5_3100;
    /// The channel mask the channel player's `Play` writes to its events word.
    const OC655_CHANNEL_MASK: u32 = DATA + 0x4_f018;

    /// A gated fake of a callee: with the gate set it logs the call and
    /// returns (`fake_call`), with bit `$bit` of the zero word set it returns
    /// 0; with the gate off it runs the body it replaced.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path, $bit:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC655_CTRL) != OC655_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                fake_call(c, $target);
                if (c.bus.read32(OC655_ZERO) >> $bit) & 1 != 0 {
                    c.g[V0][0] = 0;
                }
            }
        };
    }

    gated_fake!(oc655_get_instance, OC655_PREV_GET_INSTANCE, 0x0027_2cc0, ee::s07::f_00272cc0__generated, 0);
    gated_fake!(oc655_run_to_end, OC655_PREV_RUN_TO_END, 0x0027_4868, ee::s07::f_00274868__generated, 1);
    gated_fake!(oc655_cmdq_count, OC655_PREV_CMDQ_COUNT, 0x0028_3d28, ee::s07::f_00283d28__generated, 2);
    gated_fake!(oc655_pop_entries, OC655_PREV_POP_ENTRIES, 0x0027_7598, ee::s07::f_00277598__generated, 3);
    gated_fake!(oc655_list_prepare, OC655_PREV_LIST_PREPARE, 0x0027_6270, ee::s07::f_00276270__generated, 4);
    gated_fake!(oc655_cmdq_at, OC655_PREV_CMDQ_AT, 0x0028_3d70, ee::s07::f_00283d70__generated, 5);
    gated_fake!(oc655_is_movie, OC655_PREV_IS_MOVIE, 0x0027_7dd8, ee::s07::f_00277dd8__generated, 6);
    gated_fake!(oc655_kill_process, OC655_PREV_KILL_PROCESS, 0x0028_2540, ee::s07::f_00282540__generated, 7);
    gated_fake!(oc655_movie_stop, OC655_PREV_MOVIE_STOP, 0x0028_3818, ee::s07::f_00283818__generated, 8);
    gated_fake!(oc655_set_paused, OC655_PREV_SET_PAUSED, 0x0027_7c08, ee::s07::f_00277c08__generated, 9);
    gated_fake!(oc655_prepare_entry, OC655_PREV_PREPARE_ENTRY, 0x0027_7450, ee::s07::f_00277450__generated, 10);
    gated_fake!(oc655_list_stop, OC655_PREV_LIST_STOP, 0x0027_6868, ee::s07::f_00276868__generated, 11);
    gated_fake!(oc655_start_transition, OC655_PREV_START_TRANSITION, 0x0027_7980, ee::s07::f_00277980__generated, 12);
    gated_fake!(oc655_try_advance, OC655_PREV_TRY_ADVANCE, 0x0027_7800, ee::s07::f_00277800__generated, 13);
    gated_fake!(oc655_add_choice, OC655_PREV_ADD_CHOICE, 0x0027_8e50, ee::s07::f_00278e50__generated, 14);
    gated_fake!(oc655_nis_prepare, OC655_PREV_NIS_PREPARE, 0x0027_8f38, ee::s07::f_00278f38__generated, 15);
    gated_fake!(oc655_nis_start, OC655_PREV_NIS_START, 0x0027_8f68, ee::s07::f_00278f68__generated, 16);
    gated_fake!(oc655_audio_get, OC655_PREV_AUDIO_GET, 0x0028_b180, ee::s07::f_0028b180__generated, 17);
    gated_fake!(oc655_playback_stop, OC655_PREV_PLAYBACK_STOP, 0x0026_cdf8, ee::s07::f_0026cdf8__generated, 18);
    gated_fake!(oc655_replay_reset, OC655_PREV_REPLAY_RESET, 0x0026_f228, ee::s07::f_0026f228__generated, 19);
    gated_fake!(oc655_load_playback, OC655_PREV_LOAD_PLAYBACK, 0x0026_caf8, ee::s07::f_0026caf8__generated, 20);
    gated_fake!(oc655_task_load, OC655_PREV_TASK_LOAD, 0x0030_8798, ee::s10::f_00308798__generated, 21);
    gated_fake!(oc655_process_ctor, OC655_PREV_PROCESS_CTOR, 0x0030_7598, ee::s10::f_00307598__generated, 22);
    gated_fake!(oc655_ea_alloc, OC655_PREV_EA_ALLOC, 0x0031_7d70, ee::s10::f_00317d70__generated, 23);
    gated_fake!(oc655_push_front, OC655_PREV_PUSH_FRONT, 0x0030_b260, ee::s10::f_0030b260__generated, 24);
    gated_fake!(oc655_list_remove, OC655_PREV_LIST_REMOVE, 0x0030_b2d0, ee::s10::f_0030b2d0__generated, 25);
    gated_fake!(oc655_cache_get, OC655_PREV_CACHE_GET, 0x003a_ca38, ee::s13::f_003aca38__generated, 26);
    gated_fake!(oc655_luno_dtor, OC655_PREV_LUNO_DTOR, 0x0022_4df0, ee::s05::f_00224df0__generated, 27);
    gated_fake!(oc655_luno_ctor, OC655_PREV_LUNO_CTOR, 0x0022_4da0, ee::s05::f_00224da0__generated, 28);

    static OC655_PREV_FIND_BY_ID: AtomicUsize = AtomicUsize::new(0);

    /// FindById: the process is found on the list whose slot bit is set in
    /// `OC655_HITS` (slots: idle +0x2b4, active +0x2b8, add +0x2bc).
    fn oc655_find_by_id(c: &mut Cpu) {
        if c.bus.read32(OC655_CTRL) != OC655_MAGIC {
            return fall_back(c, &OC655_PREV_FIND_BY_ID, ee::s10::f_0030b320__generated);
        }
        let list = c.g[A0][0] as u32;
        let slot = list.wrapping_sub(OC655_MAN + 0x2b4) / 4;
        let hit = (c.bus.read32(OC655_HITS) >> (slot & 31)) & 1 != 0;
        fake_call(c, 0x0030_b320);
        c.g[V0][0] = if hit { OC655_OBJ as u64 } else { 0 };
    }

    /// Puts the gated fakes of this module in their override slots (once per
    /// process; with the gate off each runs the body it replaced).
    fn oc655_hook_all() {
        hook(addr_of_mut!(ee::s07::OVERRIDE_00272CC0), oc655_get_instance, &OC655_PREV_GET_INSTANCE);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00274868), oc655_run_to_end, &OC655_PREV_RUN_TO_END);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00283D28), oc655_cmdq_count, &OC655_PREV_CMDQ_COUNT);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00277598), oc655_pop_entries, &OC655_PREV_POP_ENTRIES);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00276270), oc655_list_prepare, &OC655_PREV_LIST_PREPARE);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00283D70), oc655_cmdq_at, &OC655_PREV_CMDQ_AT);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00277DD8), oc655_is_movie, &OC655_PREV_IS_MOVIE);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00282540), oc655_kill_process, &OC655_PREV_KILL_PROCESS);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00283818), oc655_movie_stop, &OC655_PREV_MOVIE_STOP);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00277C08), oc655_set_paused, &OC655_PREV_SET_PAUSED);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00277450), oc655_prepare_entry, &OC655_PREV_PREPARE_ENTRY);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00276868), oc655_list_stop, &OC655_PREV_LIST_STOP);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00277980), oc655_start_transition, &OC655_PREV_START_TRANSITION);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00277800), oc655_try_advance, &OC655_PREV_TRY_ADVANCE);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00278E50), oc655_add_choice, &OC655_PREV_ADD_CHOICE);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00278F38), oc655_nis_prepare, &OC655_PREV_NIS_PREPARE);
        hook(addr_of_mut!(ee::s07::OVERRIDE_00278F68), oc655_nis_start, &OC655_PREV_NIS_START);
        hook(addr_of_mut!(ee::s07::OVERRIDE_0028B180), oc655_audio_get, &OC655_PREV_AUDIO_GET);
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026CDF8), oc655_playback_stop, &OC655_PREV_PLAYBACK_STOP);
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026F228), oc655_replay_reset, &OC655_PREV_REPLAY_RESET);
        hook(addr_of_mut!(ee::s07::OVERRIDE_0026CAF8), oc655_load_playback, &OC655_PREV_LOAD_PLAYBACK);
        hook(addr_of_mut!(ee::s10::OVERRIDE_00308798), oc655_task_load, &OC655_PREV_TASK_LOAD);
        hook(addr_of_mut!(ee::s10::OVERRIDE_00307598), oc655_process_ctor, &OC655_PREV_PROCESS_CTOR);
        hook(addr_of_mut!(ee::s10::OVERRIDE_00317D70), oc655_ea_alloc, &OC655_PREV_EA_ALLOC);
        hook(addr_of_mut!(ee::s10::OVERRIDE_0030B260), oc655_push_front, &OC655_PREV_PUSH_FRONT);
        hook(addr_of_mut!(ee::s10::OVERRIDE_0030B2D0), oc655_list_remove, &OC655_PREV_LIST_REMOVE);
        hook(addr_of_mut!(ee::s10::OVERRIDE_0030B320), oc655_find_by_id, &OC655_PREV_FIND_BY_ID);
        hook(addr_of_mut!(ee::s13::OVERRIDE_003ACA38), oc655_cache_get, &OC655_PREV_CACHE_GET);
        hook(addr_of_mut!(ee::s05::OVERRIDE_00224DF0), oc655_luno_dtor, &OC655_PREV_LUNO_DTOR);
        hook(addr_of_mut!(ee::s05::OVERRIDE_00224DA0), oc655_luno_ctor, &OC655_PREV_LUNO_CTOR);
    }

    /// The gate on, and random zero/hit words for this machine.
    fn oc655_gates(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC655_CTRL, OC655_MAGIC);
        c.bus.write32(OC655_ZERO, r.next_u32());
        c.bus.write32(OC655_HITS, r.next_u32() & 7);
    }

    /// The rider-independent vtable slot `{delta, fn}` at `off` of `vt`.
    fn oc655_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, func: u32) {
        c.bus.write16(vt + off, ((r.next_u32() as i16) & !3) as u16);
        c.bus.write32(vt + off + 4, func);
    }

    #[test]
    fn in_seq_run_to_end_matches() {
        oc655_hook_all();
        covered(ee::s07::f_00272aa0__generated, in_seq_run_to_end, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0027_2aa0, 2);
            oc655_gates(c, &mut r);
        });
    }

    #[test]
    fn in_seq_channel_restart_matches() {
        oc655_hook_all();
        covered(ee::s07::f_00273f68__generated, in_seq_channel_restart, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0027_3f68, 6);
            oc655_gates(c, &mut r);
            c.hooks.miss = oc655_channel_miss;
            let me = OC655_MAN;
            let hdr = me + 0x100;
            let arr = me + 0x200;
            let comm = me + 0x300;
            let player_vt = me + 0x400;
            let comm_vt = me + 0x500;
            let players = me + 0x600;
            c.bus.write32(me + ICR_PLAYING, if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(me + ICR_HEADER, hdr);
            c.bus.write32(me + ICR_PLAYERS, arr);
            c.bus.write32(me + ICR_COMM, if seed % 4 == 0 { 0 } else { comm });
            // System header: headerSize 0x10, numChannels 0..3, the entity type.
            let nch = seed % 4;
            c.bus.write16(hdr, 0x10);
            c.bus.write8(hdr + 2, nch as u8);
            c.bus.write8(hdr + 3, r.next_u32() as u8);
            let mut rec = hdr + 0x10;
            for _ in 0..nch {
                let size = (r.next_u32() % 6) * 2;
                c.bus.write16(rec + 2, size as u16);
                rec += 4 + size;
            }
            for i in 0..4 {
                let p = players + 0x40 * i;
                c.bus.write32(arr + 4 * i, p);
                c.bus.write32(p + 4, player_vt);
            }
            // Player vtable: Play at slot 0 (+8), Stop at slot 1 (+0x10).
            oc655_slot(c, &mut r, player_vt, 0x8, FAKE);
            oc655_slot(c, &mut r, player_vt, 0x10, FAKE + 0x10);
            // Entity comm vtable: key event at slot 2 (+0x18), update at slot 3 (+0x20).
            c.bus.write32(comm, comm_vt);
            oc655_slot(c, &mut r, comm_vt, 0x18, FAKE + 0x20);
            oc655_slot(c, &mut r, comm_vt, 0x20, FAKE + 0x30);
            c.bus.write32(OC655_CHANNEL_MASK, r.next_u32() & 7);
            c.g[A0][0] = me as u64;
        });
    }

    /// Channel players' `Play` (slot 0, `FAKE`) writes the test's event mask
    /// through its `events` argument (a3) before the usual fake return.
    fn oc655_channel_miss(c: &mut Cpu, target: u32) {
        if target == FAKE {
            let events = c.g[A3][0] as u32;
            let mask = c.bus.read32(OC655_CHANNEL_MASK);
            c.bus.write32(events, mask);
        }
        fake_call(c, target);
    }

    #[test]
    fn script_list_skip_matches() {
        oc655_hook_all();
        covered(ee::s07::f_002766d0__generated, script_list_skip, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0027_66d0, 3);
            oc655_gates(c, &mut r);
            let list = OC655_MAN;
            c.bus.write32(list + 0xc0, r.next_u32() % 4);
            c.bus.write32(list + 0xbc, (r.next_u32() % 6).wrapping_sub(1));
            c.bus.write32(list + 0xa4, r.next_u32() % 4);
            c.bus.write32(list + 0xa8, r.next_u32() % 2);
            c.bus.write32(list + 0xb4, r.next_u32() % 2);
            c.g[A0][0] = list as u64;
            c.g[A1][0] = (((r.next_u32() % 5) as i32) - 2) as u32 as u64;
            c.g[A2][0] = (r.next_u32() % 2) as u64;
        });
    }



    #[test]
    fn wscript_process_load_state_matches() {
        oc655_hook_all();
        covered(ee::s10::f_00308118__generated, wscript_process_load_state, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0030_8118, 6);
            oc655_gates(c, &mut r);
            let proc_ = OC655_MAN;
            let stream = OC655_MAN + 0x1000;
            let stream_vt = OC655_MAN + 0x1100;
            let holder = OC655_MAN + 0x1200;
            let tasks = OC655_MAN + 0x1400;
            // stream->read is slot 0 of its vtable (+0x10 / +0x14).
            c.bus.write32(stream, stream_vt);
            oc655_slot(c, &mut r, stream_vt, 0x10, FAKE);
            c.bus.write32(GAME_GP.wrapping_add(0x16c8), holder);
            c.bus.write32(holder + 4, OC655_MAN + 0x1500);
            // The `has` word stays 0: the Luno table reader of the stream has no
            // override slot, so the non-zero branch is left out of the fixture.
            c.bus.write32(crate::difftest::SP.wrapping_sub(0x60), 0);
            c.bus.write32(proc_ + 0x1c, if seed % 3 == 0 { 0 } else { OC655_MAN + 0x1600 });
            c.bus.write32(proc_ + 0x24, r.next_u32() % 8);
            c.bus.write32(proc_ + 4, seed % 4);
            c.bus.write32(proc_ + 0x14, tasks);
            c.g[A0][0] = proc_ as u64;
            c.g[A1][0] = stream as u64;
        });
    }

    #[test]
    fn wscript_man_create_process_matches() {
        oc655_hook_all();
        covered(ee::s10::f_003094c8__generated, wscript_man_create_process, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0030_94c8, 3);
            oc655_gates(c, &mut r);
            c.g[A0][0] = OC655_MAN as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = (seed % 2) as u64;
        });
    }

    #[test]
    fn wscript_man_delete_process_by_id_matches() {
        oc655_hook_all();
        covered(ee::s10::f_00309618__generated, wscript_man_delete_process_by_id, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x0030_9618, 4);
            oc655_gates(c, &mut r);
            // The found process: its vtable's destructor slot (+0x0c, delta +8).
            c.bus.write32(OC655_OBJ + 0x5c, OC655_OBJ_VT);
            oc655_slot(c, &mut r, OC655_OBJ_VT, 0x8, FAKE);
            c.g[A0][0] = OC655_MAN as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn wscript_man_request_mission_playback_matches() {
        oc655_hook_all();
        covered(
            ee::s10::f_0030b590__generated,
            wscript_man_request_mission_playback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_b590, 3);
                oc655_gates(c, &mut r);
                let world = OC655_MAN + 0x1000;
                let game = OC655_MAN + 0x1100;
                let pm = OC655_MAN + 0x1200;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x34, pm);
                c.bus.write32(game + 0x28, OC655_MAN + 0x1300);
                c.bus.write32(OC655_MAN + 0x2a0, r.next_u32());
                c.g[A0][0] = OC655_MAN as u64;
            },
        );
    }
}
