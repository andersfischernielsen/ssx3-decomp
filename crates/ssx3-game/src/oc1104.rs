//! Staging module oc1104 (wave wave268): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1104`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_0cc8, path_track_get_status, "PATH_TrackGetStatus"),
    (0x003d_0ed0, path_broadcast_event, "PATH_BroadcastEvent"),
    (0x003d_1658, path_debug_printf, "PATH_DebugPrintf"),
    (0x003d_1690, path_debug_stub, "PATH_DebugStub"),
    (0x003d_16c8, path_nop_stub, "PATH_NopStub"),
    (0x003d_16d0, path_get_time_value, "PATH_GetTimeValue"),
];

/// `D_0044F42C`: the byte that says the path system is set up.
const PATH_FLAG_OC1104: u32 = 0x0044_f42c;
/// `D_0044F430`: the selected map id (0 unset, -1 none).
const CURRENT_ID_OC1104: u32 = 0x0044_f430;
/// `D_0044F420`: the selected path map.
const CURRENT_MAP_OC1104: u32 = 0x0044_f420;
/// `D_0044F438`: the current tick the track start ticks are compared with.
const TICK_NOW_OC1104: u32 = 0x0044_f438;
/// `D_0044F43C`: the time value `PATH_GetTimeValue` returns.
const TIME_VALUE_OC1104: u32 = 0x0044_f43c;

/// `PATH_TrackGetBeatPos` (0x3d32b8).
const TRACK_BEAT_POS_OC1104: Callee = Callee {
    addr: 0x003d_32b8,
    f: ee::s14::f_003d32b8,
};

/// `PATH_QueueAppend` (0x3d0b18).
const QUEUE_APPEND_OC1104: Callee = Callee {
    addr: 0x003d_0b18,
    f: ee::s14::f_003d0b18,
};

/// `PATH_SelectMap` (0x3d5450).
const SELECT_MAP_OC1104: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::f_003d5450,
};

/// `PATHI_copyevent` (0x3d2068).
const COPY_EVENT_OC1104: Callee = Callee {
    addr: 0x003d_2068,
    f: ee::s14::f_003d2068,
};

/// `PATH_AddEventToMaps` (0x3d1f80).
const ADD_EVENT_OC1104: Callee = Callee {
    addr: 0x003d_1f80,
    f: ee::s14::f_003d1f80,
};

/// Spill one general register into the frame with `sd`.
fn spill_gpr_oc1104(c: &mut Cpu, frame: Frame, off: u32, reg: usize) {
    let v = c.g[reg][0];
    let a = frame.at(c, off);
    c.sd(a, v);
}

/// `int PATH_TrackGetStatus` state machine for a non-null voice (the body of
/// 0x3d0cc8 after its null check). Leaves the state in `+0x100` and in v0.
fn track_status_states(c: &mut Cpu, s0: u64) -> Flow {
    let voice = s0 as u32;
    c.cycles += 4;
    c.g[V0][0] = 2;
    c.g[A0][0] = Addr(voice.wrapping_add(0x4c)).lw(c);
    let two = c.g[V0][0];
    Addr(voice.wrapping_add(0x100)).sw(c, two);
    if c.g[A0][0] == 0 {
        return track_status_short(c, s0);
    }
    c.cycles += 5;
    c.g[V1][0] = 0x0045_0000;
    c.g[V0][0] = Addr(TICK_NOW_OC1104).lw(c);
    let before_start = c.g[V0][0] < c.g[A0][0];
    if before_start {
        c.g[V0][0] = 3;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 4;
    }
    track_status_store_tail(c, s0)
}

/// Store the state in v0 to `+0x100`, then the load tail.
fn track_status_store_tail(c: &mut Cpu, s0: u64) -> Flow {
    let voice = s0 as u32;
    c.cycles += 1;
    let v = c.g[V0][0];
    Addr(voice.wrapping_add(0x100)).sw(c, v);
    track_status_load_tail(c, s0)
}

/// Return the state stored at `+0x100`.
fn track_status_load_tail(c: &mut Cpu, s0: u64) -> Flow {
    let voice = s0 as u32;
    c.cycles += 1;
    c.g[V0][0] = Addr(voice.wrapping_add(0x100)).lw(c);
    Ok(())
}

/// The path of the status function for a zero start tick: the voice's
/// short-path check, then the track's beat position against the map's
/// window.
fn track_status_short(c: &mut Cpu, s0: u64) -> Flow {
    let voice = s0 as u32;
    c.cycles += 3;
    c.g[V0][0] = Addr(voice.wrapping_add(0x36)).lh(c);
    let negative = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = 7;
    if negative {
        return track_status_load_tail(c, s0);
    }
    c.cycles += 6;
    c.g[A0][0] = s0;
    let seven = c.g[V0][0];
    Addr(voice.wrapping_add(0x100)).sw(c, seven);
    c.g[A1][0] = c.g[SP][0];
    TRACK_BEAT_POS_OC1104.call(c, 0x003d_0d28)?;
    let beat_negative = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = 0x0045_0000;
    if beat_negative {
        return track_status_load_tail(c, s0);
    }
    c.cycles += 8;
    c.g[V0][0] = Addr(voice.wrapping_add(0x24)).lw(c);
    c.g[A0][0] = Addr(CURRENT_MAP_OC1104).lw(c);
    c.g[A1][0] = Addr(sp(c).wrapping_add(0x18)).lw(c);
    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    let below_window = c.g[V0][0] < c.g[A1][0];
    c.g[V0][0] = below_window as u64;
    if below_window {
        // The state stays what the call path stored (7).
        c.g[V0][0] = Addr(voice.wrapping_add(0x100)).lw(c);
        return Ok(());
    }
    c.cycles += 1;
    c.g[V0][0] = 5;
    track_status_store_tail(c, s0)
}

/// `int PATH_TrackGetStatus(PathVoice* voice)` (0x3d0cc8): the state of a
/// path voice (0 when null, 3..5 for a pending or running track, 7 when the
/// track's beat position is not yet reached, 2 when no start tick is set).
/// The state is kept at `+0x100`.
pub fn path_track_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = 1;
        if c.g[S0][0] != 0 {
            let s0 = c.g[S0][0];
            track_status_states(c, s0)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The gate of `PATH_BroadcastEvent`: the path system must be set up, no map
/// selected, and the query for map -1 must answer. Leaves the header address
/// in `S0` terms (the caller computes it from the frame). True to insert.
fn broadcast_gate(c: &mut Cpu) -> Result<bool, Unwound> {
    c.g[V0][0] = 0x0045_0000;
    c.g[V1][0] = Addr(PATH_FLAG_OC1104).lbu(c);
    if c.g[V1][0] == 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[V1][0] = 0x0045_0000;
    c.g[V0][0] = Addr(CURRENT_ID_OC1104).lw(c);
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = u64::MAX;
    SELECT_MAP_OC1104.call(c, 0x003d_0f24)?;
    Ok(c.g[V0][0] != 0)
}

/// The insertion body of `PATH_BroadcastEvent` (after the gate): clears the
/// event header and the item, fills the item, queues it, then adds the
/// event to every map whose player mask fits (4 slots, 0x928 bytes each).
fn broadcast_insert(c: &mut Cpu, frame: Frame, mask: u64, val: u64) -> Flow {
    let header = frame.at(c, 0x10);
    let item = frame.at(c, 0);
    c.cycles += 25;
    c.g[A1][0] = 0;
    c.g[A0][0] = sx(header);
    c.g[A2][0] = 0x10;
    call_ptr(c, 0x0041_6210, 0x003d_0f44)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    call_ptr(c, 0x0041_6210, 0x003d_0f5c)?;
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = 4;
    let four = c.g[V0][0];
    Addr(item.wrapping_add(9)).sb(c, four);
    c.g[A0][0] = sx(header);
    let minus_one = c.g[V1][0];
    Addr(item.wrapping_add(0xe)).sb(c, minus_one);
    c.g[A1][0] = c.g[SP][0];
    Addr(item.wrapping_add(0xc)).sh(c, val);
    Addr(item).sw(c, mask);
    let minus_one = c.g[V1][0];
    Addr(item.wrapping_add(8)).sb(c, minus_one);
    QUEUE_APPEND_OC1104.call(c, 0x003d_0f8c)?;

    c.g[V0][0] = 0x0051_7610;
    // The map slots: the player mask at +0x30 and the loaded flag at +0x34
    // of each 0x928-byte map, starting at the first map's +0x30.
    let mut slot: u32 = 0x0051_7640;
    let mut i: u64 = 0;
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(slot.wrapping_add(4)).lw(c);
        if c.g[V0][0] == 0 {
            i += 1;
        } else {
            c.cycles += 5;
            c.g[V1][0] = Addr(slot).lw(c);
            let player = c.g[V1][0];
            c.g[V0][0] = player & !0x0fff_ffffu64;
            c.g[V0][0] &= mask;
            let high_hit = c.g[V0][0] != 0;
            c.g[V0][0] = player & 0x0f00_0000;
            if !high_hit {
                c.cycles += 1;
                i += 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] &= mask;
                if c.g[V0][0] == 0 {
                    i += 1;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = i & 0xff;
                    SELECT_MAP_OC1104.call(c, 0x003d_0fcc)?;
                    if c.g[V0][0] == 0 {
                        i += 1;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = sx(header);
                        COPY_EVENT_OC1104.call(c, 0x003d_0fdc)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = mask;
                        ADD_EVENT_OC1104.call(c, 0x003d_0fe8)?;
                        c.cycles += 1;
                        i += 1;
                    }
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = (i < 4) as u64;
        let more = c.g[V0][0] != 0;
        slot = slot.wrapping_add(0x928);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_0f98);
            }
            continue;
        }
        break;
    }
    c.cycles += 4;
    c.g[V1][0] = 0x0045_0000;
    Addr(CURRENT_ID_OC1104).sw(c, 0);
    Addr(CURRENT_MAP_OC1104).sw(c, 0);
    c.g[V0][0] = 0;
    Ok(())
}

/// `int PATH_BroadcastEvent(uint mask, int val)` (0x3d0ed0): queues an event
/// with player mask `mask` and value `val` and adds it to the matching maps.
/// Returns 0, or -18 when the path system is not ready (no map selected
/// afterwards, or the flag is clear).
pub fn path_broadcast_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x20);
        let mask = c.g[A0][0];
        let val = c.g[A1][0];
        if broadcast_gate(c)? {
            broadcast_insert(c, frame, mask, val)?;
        } else {
            c.cycles += 2;
            c.g[V0][0] = (-18i64) as u64;
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// One row of `PATH_StopAllTracks`: for the map `i` (the 0x928-byte slot at
/// `i`), when its player mask and loaded flag fit and the map is selected,
/// stops every track on its list. Ends with the 1578 tail (+1 cycle) unless
/// it returns early (the caller's 157c tail follows either way).
fn stop_tracks_row(c: &mut Cpu, mask: u64, i: u64) -> Flow {
    c.cycles += 5;
    c.mult(0, 5, 4, 2);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x0051_7644));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    c.cycles += 8;
    c.g[V0][0] = 0x0051_7640;
    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[V1][0] as u32).lw(c);
    let player = c.g[V1][0];
    c.g[V0][0] = player & !0x0fff_ffffu64;
    c.g[V0][0] &= mask;
    if c.g[V0][0] == 0 {
        c.cycles += 1;
        return Ok(());
    }
    c.g[V0][0] = player & 0x0f00_0000;
    c.cycles += 3;
    c.g[V0][0] &= mask;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = i;
    SELECT_MAP_OC1104.call(c, 0x003d_1530)?;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(CURRENT_MAP_OC1104).lw(c);
    let mut j: u64 = 0;
    loop {
        c.cycles += 5;
        c.g[V1][0] = sx((j as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let voice = Addr((c.g[V0][0] as u32).wrapping_add(0x58)).lw(c);
        if voice == 0 {
            j += 1;
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr((voice as u32).wrapping_add(0x54)).lw(c);
            call_ptr(c, 0x002a_f838, 0x003d_1564)?;
            Addr((voice as u32).wrapping_add(0x36)).sh(c, u64::MAX);
            j += 1;
        }
        c.cycles += 3;
        c.g[V0][0] = (j < 24) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = Addr(CURRENT_MAP_OC1104).lw(c);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_1548);
            }
            continue;
        }
        break;
    }
    c.cycles += 1;
    Ok(())
}

/// The map loop of `PATH_StopAllTracks`: rows 0..4, then the exit that clears
/// the selection.
fn stop_tracks_loop(c: &mut Cpu, mask: u64) -> Flow {
    c.g[V0][0] = 0x0928;
    let mut i: u64 = 0;
    loop {
        c.g[A0][0] = i;
        stop_tracks_row(c, mask, i)?;
        i += 1;
        c.g[A0][0] = i;
        c.cycles += 3;
        c.g[V0][0] = ((i as i64) < 4) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x0928;
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x003d_14e8);
            }
            continue;
        }
        break;
    }
    c.cycles += 3;
    c.g[V0][0] = 0x0045_0000;
    Addr(CURRENT_MAP_OC1104).sw(c, 0);
    Addr(CURRENT_ID_OC1104).sw(c, 0);
    Ok(())
}

/// `void PATH_StopAllTracks(uint mask)` (0x3d1470): stops every track on the
/// selected path maps whose player mask fits, then clears the selection.
/// Does nothing when the path system is not set up or no map is selected.
#[allow(dead_code)]
pub fn path_stop_all_tracks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0x0045_0000;
        frame.save(c, S4, 0x50);
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        c.g[V0][0] = Addr(PATH_FLAG_OC1104).lbu(c);
        let mask = c.g[A0][0];
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = 0x0045_0000;
            c.g[V0][0] = Addr(CURRENT_ID_OC1104).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                SELECT_MAP_OC1104.call(c, 0x003d_14c4)?;
                let found = c.g[V0][0] != 0;
                c.g[V0][0] = 0x0051_0000;
                if found {
                    c.cycles += 7;
                    stop_tracks_loop(c, mask)?;
                }
                c.cycles += 1;
            }
        }
        c.cycles += 11;
        frame.restore(c, S4, 0x50);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PATH_FatalExit` (0x3d1608): sets the path result to -1, calls `exit(0)`
/// and falls into `PATH_DebugPrintf`. Not registered: the callee is the real
/// `exit` (no override slot to gate a fake on), so it has no differential
/// test. Kept as the port for when `exit` is replaced.
#[allow(dead_code)]
pub fn path_fatal_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[V0][0] = u64::MAX;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = 0x0045_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        spill_gpr_oc1104(c, frame, 0x58, A1);
        Addr(CURRENT_ID_OC1104).sw(c, u64::MAX);
        spill_gpr_oc1104(c, frame, 0x60, A2);
        spill_gpr_oc1104(c, frame, 0x68, A3);
        spill_gpr_oc1104(c, frame, 0x70, T0);
        spill_gpr_oc1104(c, frame, 0x78, T1);
        spill_gpr_oc1104(c, frame, 0x80, T2);
        spill_gpr_oc1104(c, frame, 0x88, T3);
        frame.save_fpr(c, 12, 0x48);
        frame.save_fpr(c, 14, 0x4c);
        frame.save_fpr(c, 16, 0x50);
        frame.save_fpr(c, 18, 0x54);
        call_ptr(c, 0x0041_1310, 0x003d_1654)?;
        c.pc = 0x003d_1658;
        path_debug_printf(c);
        Ok(())
    })
}

/// The argument spill of `PATH_DebugPrintf` and `PATH_DebugStub`: the
/// variadic registers (a1..t3) and the float arguments (f12..f18) go to the
/// frame, then the frame is popped and it returns to `ra`. 14 cycles.
fn debug_spill_oc1104(c: &mut Cpu) {
    c.cycles += 14;
    let frame = Frame::open(c, 0x80);
    spill_gpr_oc1104(c, frame, 0x48, A1);
    spill_gpr_oc1104(c, frame, 0x50, A2);
    spill_gpr_oc1104(c, frame, 0x58, A3);
    spill_gpr_oc1104(c, frame, 0x60, T0);
    spill_gpr_oc1104(c, frame, 0x68, T1);
    spill_gpr_oc1104(c, frame, 0x70, T2);
    spill_gpr_oc1104(c, frame, 0x78, T3);
    frame.save_fpr(c, 12, 0x38);
    frame.save_fpr(c, 14, 0x3c);
    frame.save_fpr(c, 16, 0x40);
    frame.save_fpr(c, 18, 0x44);
    frame.close(c);
}

/// `PATH_DebugPrintf` (0x3d1658): the debug printf's argument spill.
pub fn path_debug_printf(c: &mut Cpu) {
    debug_spill_oc1104(c);
}

/// `PATH_DebugStub` (0x3d1690): the same argument spill as `PATH_DebugPrintf`.
pub fn path_debug_stub(c: &mut Cpu) {
    debug_spill_oc1104(c);
}

/// `PATH_NopStub` (0x3d16c8): returns at once. 2 cycles.
pub fn path_nop_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PATH_GetTimeValue` (0x3d16d0): the time value global. 3 cycles.
pub fn path_get_time_value(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = 0x0045_0000;
    c.g[V0][0] = Addr(TIME_VALUE_OC1104).lw(c);
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{machine, machine_rt};
    use crate::difftest::fill;
    use crate::difftest::DATA;

    /// A path voice at `DATA+0x1000` with junk around its fields, a map at
    /// `DATA+0x3000` selected as the current map, and a start tick that is
    /// zero for a third of the seeds. The voice pointer is zero for every
    /// eleventh seed (the null return).
    fn track_status_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let voice = DATA + 0x1000;
        let map = DATA + 0x3000;
        fill(c, &mut r, voice, 0x44);
        fill(c, &mut r, map, 0x10);
        c.bus.write32(TICK_NOW_OC1104, r.next_u32() % 0x2000);
        c.bus.write32(CURRENT_MAP_OC1104, map);
        c.bus.write32(map + 0xc, r.next_u32() % 0x400);
        c.bus.write32(
            voice + 0x4c,
            if seed % 3 == 0 {
                0
            } else {
                r.next_u32() % 0x2000
            },
        );
        c.bus.write32(voice + 0x24, r.next_u32() % 0x400);
        let h = if seed % 5 == 0 { 0x8000u32 } else { 0x0100 };
        c.bus.write8(voice + 0x36, h as u8);
        c.bus.write8(voice + 0x37, (h >> 8) as u8);
        c.g[A0][0] = if seed % 11 == 0 { 0 } else { sx(voice) };
    }

    #[test]
    fn path_track_get_status_matches() {
        check(
            ee::s14::f_003d0cc8__generated,
            path_track_get_status,
            300,
            &|c, seed| track_status_setup(c, seed, 0x3d_0cc8),
        );
    }

    /// The map rows and the event arguments: the four maps have player masks
    /// that fit the mask argument for some seeds, and loaded flags on some.
    fn broadcast_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine_rt(c, seed, salt, 0);
        let flags = [0x1000_0000u32, 0x0100_0000, 0x1100_0000, 0xf000_0000];
        c.g[A0][0] = sx(flags[(seed % 4) as usize]);
        c.g[A1][0] = sx(r.next_u32() & 0xffff);
        for m in 0..4u32 {
            let base = 0x0051_7610 + m * 0x928;
            let player = if seed % 2 == 0 {
                flags[((seed + m) % 4) as usize]
            } else {
                r.next_u32()
            };
            c.bus.write32(base + 0x30, player);
            c.bus.write32(base + 0x34, (seed >> m) & 1);
        }
        c.bus
            .write8(PATH_FLAG_OC1104, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(
            CURRENT_ID_OC1104,
            if seed % 5 == 0 { 0 } else { r.next_u32() },
        );
        c.bus.write32(CURRENT_MAP_OC1104, 0x0051_7610);
    }

    #[test]
    fn path_broadcast_event_matches() {
        check(
            ee::s14::f_003d0ed0__generated,
            path_broadcast_event,
            200,
            &|c, seed| broadcast_setup(c, seed, 0x3d_0ed0),
        );
    }

    /// The map rows with loaded maps and player masks that fit, lists with
    /// empty and filled slots, and the selection set or clear.
    fn stop_tracks_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine_rt(c, seed, salt, 0);
        let flags = [0x1000_0000u32, 0x0100_0000, 0x1100_0000, 0xf000_0000];
        c.g[A0][0] = sx(flags[(seed % 4) as usize]);
        for m in 0..4u32 {
            let base = 0x0051_7610 + m * 0x928;
            let player = if seed % 2 == 0 {
                flags[((seed + m) % 4) as usize]
            } else {
                r.next_u32()
            };
            c.bus.write32(base + 0x30, player);
            c.bus
                .write32(base + 0x34, if (seed >> m) & 3 == 0 { 0 } else { 1 });
            for j in 0..24u32 {
                let voice = DATA + 0x8000 + (m * 24 + j) * 0x40;
                let slot = if (seed + j + m) % 3 == 0 { 0 } else { voice };
                c.bus.write32(base + 0x58 + 4 * j, slot);
                c.bus.write32(voice + 0x54, 0);
                c.bus.write8(voice + 0x36, 0);
                c.bus.write8(voice + 0x37, 0);
            }
        }
        c.bus
            .write8(PATH_FLAG_OC1104, if seed % 6 == 0 { 0 } else { 1 });
        c.bus.write32(
            CURRENT_ID_OC1104,
            if seed % 4 == 0 { r.next_u32() | 1 } else { 0 },
        );
        c.bus.write32(CURRENT_MAP_OC1104, 0x0051_7610);
    }

    #[test]
    fn path_debug_printf_matches() {
        check(
            ee::s14::f_003d1658__generated,
            path_debug_printf,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x3d_1658, 0);
            },
        );
    }

    #[test]
    fn path_debug_stub_matches() {
        check(
            ee::s14::f_003d1690__generated,
            path_debug_stub,
            200,
            &|c, seed| {
                let _ = machine(c, seed, 0x3d_1690, 0);
            },
        );
    }

    #[test]
    fn path_nop_stub_matches() {
        check(
            ee::s14::f_003d16c8__generated,
            path_nop_stub,
            50,
            &|c, seed| {
                let _ = machine(c, seed, 0x3d_16c8, 0);
            },
        );
    }

    #[test]
    fn path_get_time_value_matches() {
        check(
            ee::s14::f_003d16d0__generated,
            path_get_time_value,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_16d0, 0);
                c.bus.write32(TIME_VALUE_OC1104, r.next_u32());
            },
        );
    }
}
