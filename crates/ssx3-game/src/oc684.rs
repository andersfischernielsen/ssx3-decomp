//! Staging module oc684 (wave wave236): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc684`.
//!
//! Hard leftovers. Dead ports of Replay_StepFrame (`replay_step_frame_2`),
//! Replay_OnRiderEvent, Replay_LoadSaved, SphereTree_CollidePlane and
//! SphereTree_SortChildOrder exist in the tree (`replaycache.rs`,
//! `riderspheretree.rs`, same names with or without a `_2` suffix); the ports
//! here replace them, delete the dead copies when moving these in.
//! Not ported here: ScriptEngine_QueueFinishScripts (0x27a860: its music event
//! `SSXAudioSystem_HandleMusicEvent` has no override slot and runs the real
//! audio singleton code, unbounded on test data; dead ports exist in
//! `ssxscriptengine.rs`) and WScriptMan_LoadState (0x30bd20: a resumable
//! `match pc` dispatcher, for `tools/port_dispatcher.py`).

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_03f0, replay_step_frame, "Replay_StepFrame"),
    (0x0027_09d8, replay_on_rider_event, "Replay_OnRiderEvent"),
    (0x0027_1758, replay_load_saved, "Replay_LoadSaved"),
    (
        0x0030_bfc0,
        wscript_man_check_rider_gate,
        "WScriptMan_CheckRiderGate",
    ),
    (0x0031_4050, anim_sequence_load, "AnimSequence_Load"),
    (0x0031_4880, anim_seq_list_load, "AnimSeqList_Load"),
];

/// `void Replay_StepFrame(Replay* self, int reset)` (0x2703f0): step the
/// replay player one frame. With `reset` it re-finds the current snapshot
/// (`+0x488`), skips a finished one, re-seeks every run list at `+0x48c` and
/// restores the snapshot; without it the tick (`+0x484`) advances, and past
/// the last tick (`+0x3cc`) the replay either goes to state 3 or rewinds,
/// steps once more with `reset = 1` and redraws the camera and views.
pub fn replay_step_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        let no_step = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        'b_27041c: {
            if no_step {
                break 'b_27041c;
            }
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
            Callee {
                addr: 0x0026_e448,
                f: ee::s07::f_0026e448,
            }
            .call(c, 0x0027_0418)?;
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x488)).sw(c, v);
        }
        'b_27043c: {
            c.cycles += 3;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x488)).lw(c);
            if c.g[V1][0] == 0 {
                break 'b_27043c;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x3d0)).lw(c);
            if c.g[V1][0] != c.g[V0][0] {
                break 'b_27043c;
            }
            c.cycles += 2;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x18)).lw(c);
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x488)).sw(c, v);
        }
        'b_270520: {
            'b_27048c: {
                c.cycles += 2;
                let flag_zero = c.g[S1][0] == 0;
                c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x48c));
                if flag_zero {
                    break 'b_27048c;
                }
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x488)).lw(c);
                c.g[S2][0] = 1;
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x30)).lw(c);
                let v = c.g[V0][0];
                Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
                'l_270458: loop {
                    'b_270470: {
                        c.cycles += 3;
                        c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                        if c.g[A0][0] == 0 {
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                            break 'b_270470;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
                        Callee {
                            addr: 0x0026_d4d8,
                            f: ee::s07::f_0026d4d8,
                        }
                        .call(c, 0x0027_046c)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                    }
                    c.cycles += 2;
                    let go_on = (c.g[S2][0] as i64) >= 0;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    if go_on {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_0458);
                        }
                        continue 'l_270458;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x488)).lw(c);
                    Callee {
                        addr: 0x0026_db88,
                        f: ee::s07::f_0026db88,
                    }
                    .call(c, 0x0027_0484)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270520;
                }
            }
            'b_270518: {
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x484)).lw(c);
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x3cc)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                let below = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                if below {
                    break 'b_270518;
                }
                'b_270504: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x61c)).lw(c);
                    if c.g[V0][0] == 0 {
                        let v = c.g[A0][0];
                        Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
                        break 'b_270504;
                    }
                    c.cycles += 22;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    Callee {
                        addr: 0x0027_02f8,
                        f: ee::s07::f_002702f8,
                    }
                    .call(c, 0x0027_04b8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    Callee {
                        addr: 0x0027_03f0,
                        f: ee::s07::f_002703f0,
                    }
                    .call(c, 0x0027_04c4)?;
                    c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[S1][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0016_20d0, 0x0027_04e0)?;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x14)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x20)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x24)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0027_04f4)?;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0022_e840, 0x0027_04fc)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270520;
                }
                c.cycles += 5;
                c.g[A1][0] = 3;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x0026_f4a0,
                    f: ee::s07::f_0026f4a0,
                }
                .call(c, 0x0027_0510)?;
                frame.restore(c, S0, 0x30);
                break 'b_270520;
            }
            c.cycles += 2;
            let v = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(0x484)).sw(c, v);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_OnRiderEvent(Replay* self, int rider, int kind)` (0x2709d8): a
/// rider event while a replay is recorded. Ignored while the replay is busy
/// (`+0x61c`, `+0x620`, `+0x610`, `+0x60c` or the state word at 0 set);
/// otherwise the rider's index is looked up and kind 0 runs the empty rider
/// hook and kind 1 begins a highlight (with the rider's `+0x790` field).
pub fn replay_on_rider_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        'b_270a9c: {
            'b_270a98: {
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x61c));
                let busy = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[A2][0];
                if busy {
                    break 'b_270a98;
                }
                for off in [0x620u32, 0x610, 0x60c, 0] {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(off));
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x30);
                        break 'b_270a9c;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0027_0730,
                    f: ee::s07::f_00270730,
                }
                .call(c, 0x0027_0a38)?;
                c.g[A3][0] = c.g[V0][0];
                let negative = (c.g[A3][0] as i64) < 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if negative {
                    break 'b_270a98;
                }
                'b_270a88: {
                    'b_270a70: {
                        c.cycles += 6;
                        c.g[A1][0] = sx(r32(c, S1) << 2);
                        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x84));
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                        let kind_zero = c.g[S2][0] == 0;
                        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
                        if kind_zero {
                            break 'b_270a70;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 1;
                        if c.g[S2][0] == c.g[V0][0] {
                            c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(0x790));
                            break 'b_270a88;
                        }
                        c.cycles += 2;
                        frame.restore(c, S0, 0x30);
                        break 'b_270a9c;
                    }
                    // kind 0: the empty rider hook.
                    c.cycles += 6;
                    c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x790));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xfc));
                    Callee {
                        addr: 0x0027_05a8,
                        f: ee::s07::f_002705a8,
                    }
                    .call(c, 0x0027_0a80)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_270a9c;
                }
                // kind 1: begin the rider's highlight.
                c.cycles += 4;
                c.g[A1][0] = c.g[A3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0xfc));
                Callee {
                    addr: 0x0027_0b88,
                    f: ee::s07::f_00270b88,
                }
                .call(c, 0x0027_0a98)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int Replay_LoadSaved(Replay* self, ReplaySave* save)` (0x271758): rebuild
/// the replay from a saved one. Returns 0 when the save's world mode
/// (`save+0x18`) is not the game's; else resets the snapshot list (`+0x3b0`),
/// reads the save's snapshots, then each cache slot (`+0x48c`..) that is set,
/// enters state 8 and returns 1.
pub fn replay_load_saved(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[SP][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x84));
        c.g[A2][0] = c.lw(r32(c, S3).wrapping_add(0x18));
        let a1 = c.g[A1][0];
        c.sw(frame.at(c, 0), a1);
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x7c));
        let mismatch = c.g[A2][0] != c.g[V1][0];
        c.g[V0][0] = 0;
        'b_271880: {
            if mismatch {
                break 'b_271880;
            }
            'b_271838: {
                c.cycles += 22;
                c.g[V0][0] = 1;
                c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x3b0));
                let v = c.g[V0][0];
                c.sw(r32(c, S2).wrapping_add(0x608), v);
                c.g[S5][0] = c.g[A0][0];
                c.g[S1][0] = 0;
                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x8));
                let v = c.g[V1][0];
                c.sw(r32(c, S2).wrapping_add(0x18), v);
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0xc));
                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                let v = c.g[V0][0];
                c.sw(r32(c, S2).wrapping_add(0x610), v);
                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x10));
                let v = c.g[V1][0];
                c.sw(r32(c, S2).wrapping_add(0x3c8), v);
                c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x14));
                c.sw(r32(c, S2).wrapping_add(0x3d0), 0);
                c.sw(r32(c, S2).wrapping_add(0x484), 0);
                let v = c.g[V0][0];
                c.sw(r32(c, S2).wrapping_add(0x3cc), v);
                Callee {
                    addr: 0x0026_e5c0,
                    f: ee::s07::f_0026e5c0,
                }
                .call(c, 0x0027_17e8)?;
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x28));
                let v = c.g[V0][0];
                c.sw(frame.at(c, 0), v);
                c.g[V1][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                if (c.g[V1][0] as i64) <= 0 {
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                    break 'b_271838;
                }
                c.cycles += 1;
                c.g[A0][0] = c.g[S2][0];
                // The saved snapshots, one allocation each.
                loop {
                    c.cycles += 13;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    Callee {
                        addr: 0x0027_0130,
                        f: ee::s07::f_00270130,
                    }
                    .call(c, 0x0027_1808)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0026_d678,
                        f: ee::s07::f_0026d678,
                    }
                    .call(c, 0x0027_1818)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0026_e5d0,
                        f: ee::s07::f_0026e5d0,
                    }
                    .call(c, 0x0027_1824)?;
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[S2][0];
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_1800);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                    break 'b_271838;
                }
            }
            'b_271870: {
                c.cycles += 2;
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = 0;
                if none {
                    break 'b_271870;
                }
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x48c));
                // The cache slots that are set.
                loop {
                    'b_271860: {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(r32(c, S0));
                        if c.g[A0][0] == 0 {
                            c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                            break 'b_271860;
                        }
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S4][0];
                        Callee {
                            addr: 0x0026_d558,
                            f: ee::s07::f_0026d558,
                        }
                        .call(c, 0x0027_185c)?;
                        c.g[V0][0] = c.lw(r32(c, S3).wrapping_add(0x18));
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0027_1848);
                        }
                        continue;
                    }
                    break 'b_271870;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = 1;
            c.g[V1][0] = 8;
            let v = c.g[V0][0];
            c.sw(r32(c, S2).wrapping_add(0x618), v);
            let v = c.g[V1][0];
            c.sw(r32(c, S2), v);
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `|a - b|` on VU0: `Vec(a) - Vec(b)` stored at `dst` (and through `tmp`),
/// then the square root of the squared length into Q (`cfc2 v1, vi22`). The
/// distance helper `vu0Length(vu0Sub(a, b))` of the rider gate.
fn gate_distance(c: &mut Cpu, a: u32, b: u32, dst: u32, tmp: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, dst);
    c.lq(2, dst);
    c.sq(2, tmp);
    c.lqc2(3, tmp);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
}

/// `int WScriptMan_CheckRiderGate(int* rider, int id1, int id2, int id3)`
/// (0x30bfc0): looks up up to three script objects (`id`: low byte indexes
/// the table behind `gp+0x16c8`, `id >> 8` the entry's table, `-1` = none) and
/// decides whether the rider passes the gate between them. Returns 1 when the
/// rider's own first query (vtable +0x18) is set, else compares the rider's
/// distance to the first object (scaled by `gp-0x3364`, else 1.5) with the
/// distance to the other two: 0, or -1 when it is outside.
pub fn wscript_man_check_rider_gate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xa0);
        c.g[T0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[T1][0] = c.g[A0][0];
        frame.save(c, S3, 0x60);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x50);
        c.g[S3][0] = 0;
        frame.save(c, S0, 0x90);
        c.g[S4][0] = 0;
        frame.save(c, S1, 0x80);
        c.g[V0][0] = !(0u64 | c.g[T0][0]);
        let none = c.g[V0][0] == 0;
        frame.save_ra(c, 0x40);
        'b_30c040: {
            if none {
                break 'b_30c040;
            }
            'b_30c03c: {
                'b_30c038: {
                    c.cycles += 9;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
                    c.g[V0][0] = c.g[T0][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    let null = c.g[V1][0] == 0;
                    c.g[V0][0] = sx(r32(c, T0) >> 8);
                    if null {
                        break 'b_30c038;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if found {
                        break 'b_30c03c;
                    }
                    break 'b_30c038;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'b_30c03c;
            }
            c.cycles += 1;
            c.g[S3][0] = c.g[V0][0];
        }
        'b_30c09c: {
            c.cycles += 3;
            c.g[V0][0] = !(0u64 | c.g[A2][0]);
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            if none {
                break 'b_30c09c;
            }
            'b_30c098: {
                'b_30c074: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                        break 'b_30c074;
                    }
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    break 'b_30c098;
                }
                'b_30c094: {
                    c.cycles += 7;
                    c.g[V0][0] = sx(r32(c, A2) >> 8);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if found {
                        break 'b_30c094;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                c.cycles += 1;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[S2][0] = c.g[A1][0];
        }
        'b_30c0ec: {
            c.cycles += 3;
            c.g[V0][0] = !(0u64 | c.g[A3][0]);
            let none = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
            if none {
                break 'b_30c0ec;
            }
            'b_30c0e8: {
                'b_30c0e4: {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A3][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    let null = c.g[V1][0] == 0;
                    c.g[V0][0] = sx(r32(c, A3) >> 8);
                    if null {
                        break 'b_30c0e4;
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(r32(c, V0));
                    c.g[V0][0] = sx(r32(c, V1) >> 8);
                    let found = c.g[V0][0] != 0;
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    if found {
                        break 'b_30c0e8;
                    }
                    break 'b_30c0e4;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[S4][0] = c.g[V0][0];
        }
        'b_30c36c: {
            c.cycles += 16;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[S2][0];
            c.g[V1][0] = c.lw(r32(c, T1));
            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x84));
            c.g[V1][0] = sx(r32(c, V1) << 2);
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
            c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x6c0));
            c.g[S1][0] = sx(r32(c, S0).wrapping_add(0x6c0));
            c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x18));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0030_c124)?;
            let gated = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
            if gated {
                break 'b_30c36c;
            }
            let spv = frame.at(c, 0);
            'b_30c200: {
                'b_30c1a4: {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(0x6c0));
                    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x28));
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x2c));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x0030_c140)?;
                    let no_first = c.g[S3][0] == 0;
                    c.g[A0][0] = c.g[V0][0];
                    if no_first {
                        break 'b_30c1a4;
                    }
                    // Rider position vs the first object, scaled by gp-0x3364.
                    c.cycles += 23;
                    c.lq(2, r32(c, S3).wrapping_add(0x40));
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3364));
                    c.sq(2, spv + 0x10);
                    let pos = r32(c, A0);
                    gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                    c.mtc1(3, 2);
                    c.lq(2, r32(c, S2).wrapping_add(0x40));
                    c.sq(2, spv + 0x10);
                    c.lq(3, r32(c, S3).wrapping_add(0x40));
                    break 'b_30c200;
                }
                // No first object: scale 1.5, second object instead.
                c.cycles += 23;
                c.lq(2, r32(c, S2).wrapping_add(0x40));
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 1);
                c.sq(2, spv + 0x10);
                let pos = r32(c, A0);
                gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                c.mtc1(3, 2);
                c.lq(2, r32(c, S4).wrapping_add(0x40));
                c.sq(2, spv + 0x10);
                c.lq(3, r32(c, S2).wrapping_add(0x40));
            }
            c.cycles += 22;
            c.sq(3, spv + 0x20);
            gate_distance(c, spv + 0x10, spv + 0x20, spv + 0x30, spv);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 1);
            c.c_le_s(2, 0);
            let inside = c.fcc();
            c.g[V0][0] = 0;
            if inside {
                break 'b_30c36c;
            }
            'b_30c310: {
                'b_30c2b8: {
                    c.cycles += 2;
                    let no_third = c.g[S4][0] == 0;
                    c.lq(2, r32(c, S2).wrapping_add(0x40));
                    if no_third {
                        break 'b_30c2b8;
                    }
                    c.cycles += 22;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3360));
                    c.sq(2, spv + 0x10);
                    let pos = r32(c, A0);
                    gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                    c.mtc1(3, 2);
                    c.lq(2, r32(c, S4).wrapping_add(0x40));
                    c.sq(2, spv + 0x10);
                    c.lq(3, r32(c, S2).wrapping_add(0x40));
                    break 'b_30c310;
                }
                c.cycles += 22;
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 1);
                c.sq(2, spv + 0x10);
                let pos = r32(c, A0);
                gate_distance(c, pos, spv + 0x10, spv + 0x20, spv);
                c.mtc1(3, 2);
                c.lq(2, r32(c, S2).wrapping_add(0x40));
                c.sq(2, spv + 0x10);
                c.lq(3, r32(c, S3).wrapping_add(0x40));
            }
            c.cycles += 22;
            c.sq(3, spv + 0x20);
            gate_distance(c, spv + 0x10, spv + 0x20, spv + 0x30, spv);
            c.mtc1(3, 0);
            c.mul_s(0, 0, 1);
            c.c_le_s(2, 0);
            let inside = c.fcc();
            c.g[V0][0] = 0;
            if !inside {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

/// One stream read of the anim loaders: `stream->read(dst, len)` through the
/// stream's vtable (`lh +0x10` this-delta, `lw +0x14` method).
fn anim_stream_read(c: &mut Cpu, stream: u32, dst: u32, len: u32, ret_addr: u32) -> Flow {
    c.g[V1][0] = c.lw(stream);
    c.g[A1][0] = sx(dst);
    c.g[A2][0] = len as u64;
    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x10));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x14));
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret_addr)
}

/// `void AnimSequence_Load(AnimSequence* seq, Stream* in)` (0x314050): reads a
/// sequence from a replay stream: the fixed fields (17 stream reads, the last
/// one the track count), then one 16-byte track per count, each allocated and
/// chained from `seq+0xac` / the previous track's `+0xc`.
pub fn anim_sequence_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 133;
        let frame = Frame::open(c, 0x80);
        c.g[A2][0] = 4;
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S0, 0x70);
        c.g[S4][0] = 0;
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        // (field offset in the sequence, length, return address)
        const READS: [(u32, u32, u32); 16] = [
            (0x0, 4, 0x0031_4098),
            (0x4, 0x54, 0x0031_40b4),
            (0x60, 0x20, 0x0031_40d0),
            (0x80, 4, 0x0031_40ec),
            (0x84, 4, 0x0031_4108),
            (0x88, 8, 0x0031_4124),
            (0x90, 4, 0x0031_4140),
            (0x94, 4, 0x0031_415c),
            (0x98, 4, 0x0031_4178),
            (0x9c, 4, 0x0031_4194),
            (0xa0, 4, 0x0031_41b0),
            (0xa4, 4, 0x0031_41cc),
            (0xa8, 4, 0x0031_41e8),
            (0xb0, 0x10, 0x0031_4204),
            (0xc0, 4, 0x0031_4220),
            (0xc4, 4, 0x0031_423c),
        ];
        for (off, len, ret_addr) in READS {
            let (stream, seq) = (r32(c, S1), r32(c, S2));
            anim_stream_read(c, stream, seq.wrapping_add(off), len, ret_addr)?;
        }
        // The track count lands on the stack.
        let (stream, count_slot) = (r32(c, S1), frame.at(c, 0));
        anim_stream_read(c, stream, count_slot, 4, 0x0031_4258)?;
        c.g[V1][0] = c.lw(frame.at(c, 0));
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[S3][0] = c.lw(r32(c, S2).wrapping_add(0xac));
        'b_3142c0: {
            if none {
                break 'b_3142c0;
            }
            c.cycles += 1;
            c.g[S5][0] = 0x49_0000;
            loop {
                'b_3142ac: {
                    c.cycles += 16;
                    c.g[A0][0] = 0x10;
                    c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x5a50));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    Callee {
                        addr: 0x0031_7d70,
                        f: ee::s10::f_00317d70,
                    }
                    .call(c, 0x0031_427c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = 0x10;
                    c.g[V0][0] = c.lw(r32(c, S1));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x10));
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x14));
                    let method = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0031_429c)?;
                    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0xac));
                    if c.g[V0][0] == 0 {
                        let track = c.g[S0][0];
                        c.sw(r32(c, S2).wrapping_add(0xac), track);
                        break 'b_3142ac;
                    }
                    c.cycles += 1;
                    let track = c.g[S0][0];
                    c.sw(r32(c, S3).wrapping_add(0xc), track);
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw(frame.at(c, 0));
                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0031_4268);
                    }
                    continue;
                }
                break 'b_3142c0;
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void AnimSeqList_Load(AnimSeqList* list, Stream* in)` (0x314880): reads
/// the sequence count, then for each sequence allocates 0xd0 bytes, clears
/// them, loads the sequence from the stream and appends it to the list.
pub fn anim_seq_list_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        c.g[A2][0] = 4;
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S4, 0x30);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = c.lw(r32(c, S2));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x10));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x14));
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0031_48c8)?;
        c.g[V1][0] = c.lw(frame.at(c, 0));
        let none = (c.g[V1][0] as i64) <= 0;
        c.sw(r32(c, S3), 0);
        'b_314950: {
            if none {
                break 'b_314950;
            }
            c.cycles += 3;
            c.g[S5][0] = 0x49_0000;
            c.g[S4][0] = 0xffff_ffff_ffff_ffff;
            c.g[A0][0] = 0xd0;
            'l_3148e0: loop {
                c.cycles += 8;
                c.g[A1][0] = sx(r32(c, S5).wrapping_sub(0x5a40));
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                Callee {
                    addr: 0x0031_7d70,
                    f: ee::s10::f_00317d70,
                }
                .call(c, 0x0031_48f0)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 2;
                // Three spins of a cleared-slot countdown (v0 2, 1, 0, -1).
                loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    let spin = c.g[V0][0] != c.g[S4][0];
                    c.g[A2][0] = 0xd0;
                    if spin {
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_4900);
                        }
                        continue;
                    }
                    c.cycles += 13;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003e_6448, 0x0031_4928)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x0031_4050,
                        f: ee::s10::f_00314050,
                    }
                    .call(c, 0x0031_4934)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0031_4368,
                        f: ee::s10::f_00314368,
                    }
                    .call(c, 0x0031_4940)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = 0xd0;
                        if event_due(c) {
                            deliver_event_here(c, 0x0031_48e0);
                        }
                        continue 'l_3148e0;
                    }
                    break 'b_314950;
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `base + off` as a 32-bit guest address.
fn oc684_addr(base: u64, off: u32) -> u32 {
    (base as u32).wrapping_add(off)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, fall_back, hook, machine};
    use crate::difftest::{check, put_vec, put_words, scramble, Rng, DATA, GAME_GP};
    use crate::game::scripter::ssxscriptengine::tests::{
        oc423_fake, oc423_fake_call, OC423_CTRL, OC423_MAGIC, OC423_RET,
    };
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The test stack top (`SP` would shadow the register index).
    const OC684_STACK: u32 = crate::difftest::SP;

    /// Where the bump allocator fake keeps its next block, and the value the
    /// plane-check fake stores as `worstCase`.
    const OC684_BUMP: u32 = OC423_CTRL + 0x100;
    const OC684_WORST: u32 = OC423_CTRL + 0x80;

    fn oc684_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// Gate on, the scripted result words and the allocator heap start.
    fn oc684_gate(c: &mut Cpu) {
        oc684_w(c, OC423_CTRL, OC423_MAGIC);
        oc684_w(c, OC684_BUMP, DATA + 0x20_0000);
    }

    oc423_fake!(
        oc684_fake_find_snapshot,
        OC684_PREV_FIND_SNAPSHOT,
        0x0026_e448,
        ee::s07::f_0026e448__generated
    );
    oc423_fake!(
        oc684_fake_run_list_seek,
        OC684_PREV_RUN_LIST_SEEK,
        0x0026_d4d8,
        ee::s07::f_0026d4d8__generated
    );
    oc423_fake!(
        oc684_fake_seek_to_snapshot,
        OC684_PREV_SEEK_TO_SNAPSHOT,
        0x0027_02f8,
        ee::s07::f_002702f8__generated
    );
    oc423_fake!(
        oc684_fake_set_state,
        OC684_PREV_SET_STATE,
        0x0026_f4a0,
        ee::s07::f_0026f4a0__generated
    );
    oc423_fake!(
        oc684_fake_find_rider_index,
        OC684_PREV_FIND_RIDER_INDEX,
        0x0027_0730,
        ee::s07::f_00270730__generated
    );
    oc423_fake!(
        oc684_fake_empty_rider_hook,
        OC684_PREV_EMPTY_RIDER_HOOK,
        0x0027_05a8,
        ee::s07::f_002705a8__generated
    );
    oc423_fake!(
        oc684_fake_begin_highlight,
        OC684_PREV_BEGIN_HIGHLIGHT,
        0x0027_0b88,
        ee::s07::f_00270b88__generated
    );
    oc423_fake!(
        oc684_fake_snapshot_clear,
        OC684_PREV_SNAPSHOT_CLEAR,
        0x0026_e5c0,
        ee::s07::f_0026e5c0__generated
    );
    oc423_fake!(
        oc684_fake_cache_deserialize,
        OC684_PREV_CACHE_DESERIALIZE,
        0x0026_d558,
        ee::s07::f_0026d558__generated
    );
    oc423_fake!(
        oc684_fake_alloc_buf,
        OC684_PREV_ALLOC_BUF,
        0x0032_df28,
        ee::s11::f_0032df28__generated
    );

    static OC684_PREV_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static OC684_PREV_CHECK_PLANE: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemAllocNamed`: a fresh 0x100-byte block of the test heap per call
    /// (so appended list nodes never alias).
    fn oc684_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(OC423_CTRL) != OC423_MAGIC {
            return fall_back(c, &OC684_PREV_ALLOC, ee::s10::f_00317d70__generated);
        }
        let ra = c.g[RA][0] as u32;
        let block = c.bus.read32(OC684_BUMP);
        c.bus.write32(OC684_BUMP, block + 0x100);
        oc423_fake_call(c, 0x0031_7d70);
        if c.pc == ra {
            c.g[V0][0] = block as u64;
        }
    }

    /// `SphereTree_checkPlaneCollision`: stores the seed's `worstCase` in the
    /// tree (`+0x94`).
    fn oc684_fake_check_plane(c: &mut Cpu) {
        if c.bus.read32(OC423_CTRL) != OC423_MAGIC {
            return fall_back(c, &OC684_PREV_CHECK_PLANE, ee::s11::f_0032d208__generated);
        }
        let tree = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        oc423_fake_call(c, 0x0032_d208);
        if c.pc == ra {
            let worst = c.bus.read32(OC684_WORST);
            c.bus.write32(tree + 0x94, worst);
        }
    }

    fn oc684_hook_replay_fakes() {
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E448),
            oc684_fake_find_snapshot,
            &OC684_PREV_FIND_SNAPSHOT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D4D8),
            oc684_fake_run_list_seek,
            &OC684_PREV_RUN_LIST_SEEK,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002702F8),
            oc684_fake_seek_to_snapshot,
            &OC684_PREV_SEEK_TO_SNAPSHOT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F4A0),
            oc684_fake_set_state,
            &OC684_PREV_SET_STATE,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00270730),
            oc684_fake_find_rider_index,
            &OC684_PREV_FIND_RIDER_INDEX,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_002705A8),
            oc684_fake_empty_rider_hook,
            &OC684_PREV_EMPTY_RIDER_HOOK,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00270B88),
            oc684_fake_begin_highlight,
            &OC684_PREV_BEGIN_HIGHLIGHT,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026E5C0),
            oc684_fake_snapshot_clear,
            &OC684_PREV_SNAPSHOT_CLEAR,
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026D558),
            oc684_fake_cache_deserialize,
            &OC684_PREV_CACHE_DESERIALIZE,
        );
    }

    fn oc684_hook_anim_fakes() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc684_fake_alloc,
            &OC684_PREV_ALLOC,
        );
    }

    fn oc684_hook_sphere_fakes() {
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0032DF28),
            oc684_fake_alloc_buf,
            &OC684_PREV_ALLOC_BUF,
        );
        hook(
            addr_of_mut!(ee::s11::OVERRIDE_0032D208),
            oc684_fake_check_plane,
            &OC684_PREV_CHECK_PLANE,
        );
    }

    /// `Replay_StepFrame`: reset = 0 with the tick before the last tick (tick
    /// advances; current snapshot unchanged, equal to the saved one, or
    /// another), at the last tick with `+0x61c` clear (state 3 via the fake)
    /// or set (the first fake, SeekToSnapshot, unwinds; the recursive step
    /// and the redraw behind it need the real snapshot restore, which the
    /// fixture cannot bound), and reset = 1 (FindSnapshot unwinds at once:
    /// the run-list loop and `Replay_RestoreSnapshot`, the real
    /// `WorldState_Read`, are not reached).
    #[test]
    fn replay_step_frame_matches() {
        oc684_hook_replay_fakes();
        fakes::covered(
            ee::s07::f_002703f0__generated,
            replay_step_frame,
            240,
            &|c, seed| {
                let _ = machine(c, seed, 0x0027_03f0, 1);
                oc684_gate(c);
                let mut r = Rng(0x2703_f0 ^ (seed as u64) << 20);
                let (replay, save) = (DATA, DATA + 0x2000);
                let kind = seed % 6;
                let tick = r.next_u32() % 1000;
                let (last, f61c) = match kind {
                    0 | 5 => (tick + 1 + seed % 2, 0),
                    1 => (tick - tick.min(seed % 2), 0),
                    _ => (tick - tick.min(seed % 2), 1),
                };
                let cur = match seed / 6 % 3 {
                    0 => 0,
                    1 => save,
                    _ => DATA + 0x3000,
                };
                oc684_w(c, replay + 0x3d0, save);
                oc684_w(c, replay + 0x488, cur);
                oc684_w(c, save + 0x18, DATA + 0x4000);
                oc684_w(c, replay + 0x484, tick);
                oc684_w(c, replay + 0x3cc, last);
                oc684_w(c, replay + 0x61c, f61c);
                let unwind = matches!(kind, 2 | 3 | 4);
                oc684_w(c, fakes::LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
                c.g[A0][0] = replay as u64;
                c.g[A1][0] = matches!(kind, 3 | 4) as u64;
            },
        );
    }

    /// `Replay_OnRiderEvent`: each busy flag alone, no flag with a negative
    /// or valid rider index, and kinds 0 (empty hook), 1 (highlight) and 2
    /// (ignored).
    #[test]
    fn replay_on_rider_event_matches() {
        oc684_hook_replay_fakes();
        fakes::covered(
            ee::s07::f_002709d8__generated,
            replay_on_rider_event,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_09d8, 2);
                oc684_gate(c);
                let replay = DATA;
                // Which busy word is set: 0..=4, or none (5..=7).
                let busy = [0x61cu32, 0x620, 0x610, 0x60c, 0];
                let which = (seed % 8) as usize;
                for off in busy {
                    oc684_w(c, replay + off, 0);
                }
                if which < busy.len() {
                    oc684_w(c, replay + busy[which], 1 + r.next_u32() % 3);
                }
                let rider = seed / 8 % 3;
                let kind = seed / 24 % 3;
                // FindRiderIndex result: negative for one seed in three.
                let index = if seed / 72 % 3 == 0 { u32::MAX } else { rider };
                oc684_w(c, OC423_RET, index);
                // World -> manager -> rider table at +0x28 + 4 * rider.
                let (world, manager, table) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                oc684_w(c, GAME_GP - 0x848, world);
                oc684_w(c, world + 0x84, manager);
                oc684_w(c, manager + 0xc, table);
                for i in 0..3u32 {
                    let obj = DATA + 0x4400 + 0x1000 * i;
                    oc684_w(c, table + 0x28 + 4 * i, obj);
                    oc684_w(c, obj + 0x790, r.next_u32());
                }
                c.g[A0][0] = replay as u64;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = kind as u64;
            },
        );
    }

    /// `Replay_LoadSaved`: world mode mismatch (returns 0), 0..=2 cache slots
    /// each set or not, no saved snapshots (the snapshot loop calls the
    /// unfakeable `Replay_DeserializeSnapshot` and is not reached).
    #[test]
    fn replay_load_saved_matches() {
        oc684_hook_replay_fakes();
        fakes::covered(
            ee::s07::f_00271758__generated,
            replay_load_saved,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0027_1758, 3);
                oc684_gate(c);
                let (replay, save) = (DATA, DATA + 0x1000);
                let caches = seed % 3;
                let mismatch = seed % 6 == 5;
                let (world, manager, ai) = (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200);
                oc684_w(c, GAME_GP - 0x848, world);
                oc684_w(c, world + 0x84, manager);
                oc684_w(c, manager + 0xc, ai);
                oc684_w(c, ai + 0x7c, caches + mismatch as u32);
                oc684_w(c, save + 0x4, 0);
                oc684_w(c, save + 0x8, r.next_u32());
                oc684_w(c, save + 0xc, r.next_u32() % 3);
                oc684_w(c, save + 0x10, r.next_u32());
                oc684_w(c, save + 0x14, r.next_u32());
                oc684_w(c, save + 0x18, caches);
                for i in 0..2u32 {
                    let set = (seed / 3 >> i) & 1 == 1;
                    oc684_w(
                        c,
                        replay + 0x48c + 4 * i,
                        if set { DATA + 0x3000 } else { 0 },
                    );
                }
                c.g[A0][0] = replay as u64;
                c.g[A1][0] = save as u64;
            },
        );
    }

    /// A stream object at `stream` whose vtable (at `stream + 0x100`) has the
    /// read method (`+0x14`) at the fake address and a `this` delta (`+0x10`).
    fn oc684_stream(c: &mut Cpu, stream: u32, delta: u32) {
        let vt = stream + 0x100;
        oc684_w(c, stream, vt);
        c.bus.write16(vt + 0x10, delta as u16);
        oc684_w(c, vt + 0x14, fakes::FAKE);
    }

    /// `AnimSequence_Load`: 0, 1..3 tracks (and a negative count), the
    /// sequence's track chain empty or already started.
    #[test]
    fn anim_sequence_load_matches() {
        oc684_hook_anim_fakes();
        fakes::covered(
            ee::s10::f_00314050__generated,
            anim_sequence_load,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_4050, 24);
                oc684_gate(c);
                let (seq, stream) = (DATA + 0x400, DATA + 0x800);
                oc684_stream(c, stream, (r.next_u32() % 4) * 4);
                let count = [0u32, 1, 2, 3, u32::MAX][(seed % 5) as usize];
                oc684_w(c, OC684_STACK - 0x80, count);
                let head = if seed / 5 % 2 == 0 { 0 } else { DATA + 0x600 };
                oc684_w(c, seq + 0xac, head);
                c.g[A0][0] = seq as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    /// `AnimSeqList_Load`: 0, 1 or 2 sequences, each one with 0..2 tracks
    /// (real `AnimSequence_Load`, `AnimSeqList_Append` and memset behind the
    /// allocator and stream fakes).
    #[test]
    fn anim_seq_list_load_matches() {
        oc684_hook_anim_fakes();
        fakes::covered(
            ee::s10::f_00314880__generated,
            anim_seq_list_load,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0031_4880, 40);
                oc684_gate(c);
                let (list, stream) = (DATA + 0x400, DATA + 0x800);
                oc684_stream(c, stream, (r.next_u32() % 4) * 4);
                let count = [0u32, 1, 2, u32::MAX][(seed % 4) as usize];
                oc684_w(c, OC684_STACK - 0x80, count);
                oc684_w(c, OC684_STACK - 0x100, seed / 4 % 3);
                oc684_w(c, list, r.next_u32());
                oc684_w(c, list + 4, 0);
                c.g[A0][0] = list as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    /// `SphereTree_SortChildOrder`: random child centres and direction (some
    /// seeds with equal centres, so the keys tie) through the real shell sort.

    /// `SphereTree_CollidePlane`: compressed and plain tree info (AllocBuf
    /// fake), `worstCase` negative, zero, positive or -0 from the faked plane
    /// check, the real sort-axes call in between; fakes unwind at any call.

    /// `WScriptMan_CheckRiderGate`: the three ids each -1 (none), valid, or
    /// pointing at a null entry / null node (6^3 combinations), with the
    /// rider's first query either set (fake call, returns 1) or clear (real
    /// leaf returning 0, then the position from a second fake).
    #[test]
    fn wscript_man_check_rider_gate_matches() {
        fakes::covered(
            ee::s10::f_0030bfc0__generated,
            wscript_man_check_rider_gate,
            432,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0030_bfc0, 1);
                // The first fake call unwinds on a third of the seeds.
                let unwind = (seed / 7) % 3 == 0;
                oc684_w(c, fakes::LOG_UNWIND_AT, if unwind { 0 } else { u32::MAX });
                const NODES: u32 = DATA + 0x7000;
                const POS: u32 = DATA + 0x7800;
                // Ref table: ctx -> A -> [+8] B[lo] -> entry -> [+0x1c] C[hi]
                // -> (node >> 2) << 8.
                let (ctx, ta, tb, entry, tc) = (
                    DATA + 0x5000,
                    DATA + 0x5100,
                    DATA + 0x5200,
                    DATA + 0x5300,
                    DATA + 0x5400,
                );
                oc684_w(c, GAME_GP + 0x16c8, ctx);
                oc684_w(c, ctx, ta);
                oc684_w(c, ta + 8, tb);
                for lo in 0..4u32 {
                    oc684_w(c, tb + 4 * lo, entry);
                }
                oc684_w(c, tb + 16, 0);
                oc684_w(c, entry + 0x1c, tc);
                for hi in 0..4u32 {
                    let node = NODES + 0x100 * hi;
                    let word = if hi == 3 {
                        0
                    } else {
                        ((node >> 2) << 8) | 0x55
                    };
                    oc684_w(c, tc + 4 * hi, word);
                    for k in 0..4 {
                        oc684_w(c, node + 0x40 + 4 * k, r.f32(10.0));
                    }
                }
                for k in 0..4 {
                    oc684_w(c, POS + 4 * k, r.f32(10.0));
                }
                // The rider: index at `idx`, object with the vtable at +0x6c0.
                let (world, manager, table, idx) =
                    (DATA + 0x4000, DATA + 0x4100, DATA + 0x4200, DATA + 0x4400);
                let (rider, vt) = (DATA + 0x6900, DATA + 0x6800);
                let slot = seed % 3;
                oc684_w(c, GAME_GP - 0x848, world);
                oc684_w(c, world + 0x84, manager);
                oc684_w(c, manager + 0xc, table);
                oc684_w(c, idx, slot);
                oc684_w(c, table + 0x28 + 4 * slot, rider);
                oc684_w(c, rider + 0x6c0, vt);
                let delta = (r.next_u32() % 4) * 4;
                c.bus.write16(vt + 0x18, delta as u16);
                c.bus.write16(vt + 0x28, delta as u16);
                let first_set = seed / 216 == 0;
                // Slot 0x1c: the fake (non-zero result) or a leaf returning 0.
                oc684_w(
                    c,
                    vt + 0x1c,
                    if first_set { fakes::FAKE } else { 0x0014_06c0 },
                );
                oc684_w(c, vt + 0x2c, fakes::FAKE);
                // The second query's result: the rider position (fake call 0).
                oc684_w(c, fakes::LOG_RESULTS, POS);
                put_words(
                    c,
                    GAME_GP - 0x3364,
                    &[(1.0f32 + (seed % 3) as f32).to_bits()],
                );
                put_words(
                    c,
                    GAME_GP - 0x3360,
                    &[(1.0f32 + (seed / 3 % 3) as f32).to_bits()],
                );
                // Ids: -1, valid (hi 0..2, lo 0..2), null node (hi 3), null entry (lo 4).
                let ids = [u32::MAX, 0x000, 0x101, 0x202, 0x303, 0x004];
                let pick = |k: u32| sx(ids[((seed % 216) / 6u32.pow(k) % 6) as usize]);
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = pick(0);
                c.g[A2][0] = pick(1);
                c.g[A3][0] = pick(2);
            },
        );
    }
}
