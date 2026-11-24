//! Staging module oc538 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc538`.

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
    (0x0027_4578, in_seq_instance_dtor, "InSeq_Instance_dtor"),
    (
        0x0027_4868,
        in_seq_instance_run_to_end,
        "InSeq_InstanceRunToEnd",
    ),
    (0x0027_6048, script_list_add_movie, "ScriptList_AddMovie"),
    (
        0x0027_6a78,
        script_list_get_entry_script_index,
        "ScriptList_GetEntryScriptIndex",
    ),
    (
        0x0027_6b08,
        script_list_get_entry_script,
        "ScriptList_GetEntryScript",
    ),
    (
        0x0027_7100,
        script_list_is_next_ready,
        "ScriptList_IsNextReady",
    ),
];

/// The address `off(reg)`, for a register read before a call or a store.
fn oc538_field(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// `cScriptPlayerMain::~cScriptPlayerMain(this, flags)` (0x274578, `InSeq_Instance_dtor`):
/// releases the instance (`InSeq_InstanceRelease`), then frees `this` through
/// `operator delete` (0x317e50) when bit 0 of `flags` is set.
pub fn in_seq_instance_dtor(c: &mut Cpu) {
    const RELEASE: Callee = Callee {
        addr: 0x0027_46f8,
        f: ee::s07::f_002746f8,
    };
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        RELEASE.call(c, 0x0027_4594)?;
        c.g[S0][0] &= 1;
        let free_this = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if free_this {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0027_45a8)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cScriptPlayerMain_Preview(this, flag)` (0x274868, `InSeq_InstanceRunToEnd`): stores
/// `flag` as the preview comm interface (+0x44), starts the instance at speed 1.0 and
/// updates it until it stops, then restores the saved speed (+0x24) and loop script
/// (+0x1c) and clears the interface. The update loop checks for due events at its back
/// edge.
pub fn in_seq_instance_run_to_end(c: &mut Cpu) {
    const START: Callee = Callee {
        addr: 0x0027_4830,
        f: ee::s07::f_00274830,
    };
    const UPDATE: Callee = Callee {
        addr: 0x0027_4a30,
        f: ee::s07::f_00274a30,
    };
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_fpr(c, 20, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x38);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x24));
        let flag = c.g[A1][0];
        oc538_field(c, S0, 0x44).sw(c, flag);
        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x24));
        c.g[A1][0] = 0;
        c.g[S1][0] = oc538_field(c, S0, 0x1c).lw(c);
        START.call(c, 0x0027_48a4)?;
        c.g[V0][0] = oc538_field(c, S0, 0x20).lw(c);
        if c.g[V0][0] == 0 {
            c.swc1(21, (c.g[S0][0] as u32).wrapping_add(0x24));
        } else {
            c.cycles += 2;
            c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x24));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                UPDATE.call(c, 0x0027_48c0)?;
                c.g[V0][0] = oc538_field(c, S0, 0x20).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    c.swc1(21, (c.g[S0][0] as u32).wrapping_add(0x24));
                    break;
                }
                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x24));
                if event_due(c) {
                    deliver_event_here(c, 0x0027_48b8);
                }
            }
        }
        c.cycles += 9;
        let saved_script = c.g[S1][0];
        oc538_field(c, S0, 0x1c).sw(c, saved_script);
        oc538_field(c, S0, 0x44).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_AddMovie(self, controlBits, scriptIndex, a2, a3, t0, t1)` (0x276048,
/// `tNISList_AddByScriptIndex`): queues a script command record (category 0x1c) on the
/// list's command queue (+0xc). When the list is idle (state 0), the call returns 1. When
/// it has a pending entry (`nextPending >= 0`), returns 1 too. Otherwise the entry is
/// prepared, and when the list is in state 3 the entry is checked (movie or request
/// player) and `ScriptList_CheckNextReady` decides whether it is ready. Returns 0 when the
/// queue is full.
pub fn script_list_add_movie(c: &mut Cpu) {
    const CMD_QUEUE_IS_FULL: Callee = Callee {
        addr: 0x0028_3c58,
        f: ee::s07::f_00283c58,
    };
    const CMD_QUEUE_PUSH: Callee = Callee {
        addr: 0x0028_3c80,
        f: ee::s07::f_00283c80,
    };
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const PREPARE_ENTRY: Callee = Callee {
        addr: 0x0027_7450,
        f: ee::s07::f_00277450,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    const MOVIE_GET_ID: Callee = Callee {
        addr: 0x0028_3bb8,
        f: ee::s07::f_00283bb8,
    };
    const CHECK_NEXT_READY: Callee = Callee {
        addr: 0x0027_76e0,
        f: ee::s07::f_002776e0,
    };
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[T1][0];
        frame.save_ra(c, 0x20);
        c.g[S6][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S1][0];
        CMD_QUEUE_IS_FULL.call(c, 0x0027_6090)?;
        let full = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !full {
            c.cycles += 15;
            c.g[V1][0] = u64::MAX;
            c.g[V0][0] = 0x1c;
            let v1 = c.g[V1][0];
            Addr(sp(c).wrapping_add(0xc)).sw(c, v1);
            c.g[A0][0] = c.g[S1][0];
            let category = c.g[V0][0];
            Addr(sp(c)).sw(c, category);
            c.g[A1][0] = c.g[SP][0];
            let (s2, s6, s3, s4, s5) = (c.g[S2][0], c.g[S6][0], c.g[S3][0], c.g[S4][0], c.g[S5][0]);
            Addr(sp(c).wrapping_add(0x8)).sw(c, s2);
            Addr(sp(c).wrapping_add(0x4)).sw(c, s6);
            Addr(sp(c).wrapping_add(0x10)).sw(c, s3);
            Addr(sp(c).wrapping_add(0x14)).sw(c, s4);
            Addr(sp(c).wrapping_add(0x18)).sw(c, s5);
            CMD_QUEUE_PUSH.call(c, 0x0027_60c8)?;
            c.g[V1][0] = oc538_field(c, S0, 0xa4).lw(c);
            let idle = c.g[V1][0] == 0;
            c.g[V0][0] = 1;
            if !idle {
                c.cycles += 3;
                c.g[V0][0] = oc538_field(c, S0, 0xbc).lw(c);
                let pending = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 1;
                if !pending {
                    // Prepare the next entry; state 3 checks it.
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    CMD_QUEUE_COUNT.call(c, 0x0027_60e8)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.g[A2][0] = 0;
                    PREPARE_ENTRY.call(c, 0x0027_60f8)?;
                    c.g[A0][0] = oc538_field(c, S0, 0xa4).lw(c);
                    c.g[V1][0] = 3;
                    let not_three = c.g[A0][0] != c.g[V1][0];
                    let prepared = c.g[V0][0];
                    oc538_field(c, S0, 0xbc).sw(c, prepared);
                    if not_three {
                        c.cycles += 1;
                        c.g[V0][0] = 1;
                    } else {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = 0;
                        CMD_QUEUE_AT.call(c, 0x0027_6114)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S0][0];
                        IS_MOVIE_ENTRY.call(c, 0x0027_6120)?;
                        let movie = c.g[V0][0] != 0;
                        let mut get_id = true;
                        if movie {
                            c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = 0;
                            CMD_QUEUE_AT.call(c, 0x0027_6134)?;
                            c.g[A1][0] = oc538_field(c, V0, 0xc).lw(c);
                            c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                            PROCESS_IN_SEQ.call(c, 0x0027_6140)?;
                            c.g[V1][0] = oc538_field(c, V0, 0x20).lw(c);
                            c.g[V1][0] ^= 4;
                            c.g[V0][0] = (c.g[V1][0] < 1) as u64;
                            get_id = false;
                        }
                        if get_id {
                            c.cycles += 2;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                            MOVIE_GET_ID.call(c, 0x0027_6158)?;
                        }
                        c.cycles += 2;
                        let no_request = c.g[V0][0] == 0;
                        c.g[V0][0] = 1;
                        if !no_request {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S0][0];
                            CHECK_NEXT_READY.call(c, 0x0027_6168)?;
                            c.cycles += 1;
                            c.g[V0][0] = 1;
                        }
                    }
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_Skip` (0x2766d0, `tNISList_SkipForward`): skips `delta` queue entries from
/// the skip count (+0xc0). Returns the list state. A skip that reaches the end of the queue
/// resets the list (`ScriptList_Stop(self, 1)`). In state 3 the pending entry is released
/// and replaced, and the transition to the next entry is started.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn script_list_skip(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const POP_ENTRIES: Callee = Callee {
        addr: 0x0027_7598,
        f: ee::s07::f_00277598,
    };
    const PREPARE: Callee = Callee {
        addr: 0x0027_6270,
        f: ee::s07::f_00276270,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const KILL_PROCESS: Callee = Callee {
        addr: 0x0028_2540,
        f: ee::s07::f_00282540,
    };
    const MOVIE_STOP: Callee = Callee {
        addr: 0x0028_3818,
        f: ee::s07::f_00283818,
    };
    const SET_PAUSED: Callee = Callee {
        addr: 0x0027_7c08,
        f: ee::s07::f_00277c08,
    };
    const PREPARE_ENTRY: Callee = Callee {
        addr: 0x0027_7450,
        f: ee::s07::f_00277450,
    };
    const START_TRANSITION: Callee = Callee {
        addr: 0x0027_7980,
        f: ee::s07::f_00277980,
    };
    const TRY_ADVANCE: Callee = Callee {
        addr: 0x0027_7800,
        f: ee::s07::f_00277800,
    };
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc538_field(c, S0, 0xc0).lw(c);
        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let unchanged = c.g[S1][0] == c.g[V0][0];
        c.g[S3][0] = c.g[A2][0];
        'exit: {
            if unchanged {
                // Same skip count: return the state.
                c.cycles += 1;
                c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
                break 'exit;
            }
            c.cycles += 11;
            c.g[V1][0] = oc538_field(c, S0, 0xbc).lw(c);
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = c.g[V1][0];
            }
            CMD_QUEUE_COUNT.call(c, 0x0027_6714)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = oc538_field(c, S0, 0xa4).lw(c);
            c.g[V0][0] = 3;
            let state_is_3 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if !state_is_3 {
                // Not playing a pending entry: pop the skipped entries.
                c.cycles += 2;
                let in_range = c.g[V0][0] != 0;
                oc538_field(c, S0, 0xc0).sw(c, 0);
                if !in_range {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S0][0];
                    skip_reset(c)?;
                    break 'exit;
                }
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                POP_ENTRIES.call(c, 0x0027_673c)?;
                c.g[V1][0] = oc538_field(c, S0, 0xa4).lw(c);
                c.g[V0][0] = u64::MAX;
                let minus_one = c.g[V0][0];
                oc538_field(c, S0, 0xbc).sw(c, minus_one);
                let idle = c.g[V1][0] == 0;
                oc538_field(c, S0, 0xa4).sw(c, 0);
                if idle {
                    c.cycles += 1;
                    c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
                    break 'exit;
                }
                c.cycles += 5;
                c.g[A1][0] = oc538_field(c, S0, 0xb4).lw(c);
                c.g[A0][0] = c.g[S0][0];
                PREPARE.call(c, 0x0027_675c)?;
                c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
                break 'exit;
            }
            // State 3: the skip count becomes the new position.
            c.cycles += 3;
            c.g[A1][0] = oc538_field(c, S0, 0xbc).lw(c);
            let no_pending = (c.g[A1][0] as i64) <= 0;
            let pos = c.g[S1][0];
            oc538_field(c, S0, 0xc0).sw(c, pos);
            let mut skip_h_end = false;
            if !no_pending {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = oc538_field(c, S0, 0xa8).lw(c);
                    skip_h_end = true;
                } else {
                    // The pending entry is released, then the next one is set up.
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    CMD_QUEUE_AT.call(c, 0x0027_6784)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    IS_MOVIE_ENTRY.call(c, 0x0027_6794)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = oc538_field(c, S2, 0xc).lw(c);
                        c.cycles += 2;
                        let no_handle = c.g[V0][0] == 0;
                        c.g[V1][0] = u64::MAX;
                        if !no_handle {
                            c.cycles += 4;
                            c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x500));
                            MOVIE_STOP.call(c, 0x0027_67c4)?;
                            c.g[V1][0] = u64::MAX;
                        }
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                        c.g[A1][0] = oc538_field(c, S2, 0xc).lw(c);
                        KILL_PROCESS.call(c, 0x0027_67a8)?;
                        c.g[V1][0] = u64::MAX;
                    }
                    c.cycles += 5;
                    let minus_one = c.g[V1][0];
                    oc538_field(c, S2, 0xc).sw(c, minus_one);
                    oc538_field(c, S0, 0xbc).sw(c, minus_one);
                    c.g[V0][0] = oc538_field(c, S0, 0xa8).lw(c);
                    let paused = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if paused {
                        c.cycles += 3;
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 1;
                        SET_PAUSED.call(c, 0x0027_67e8)?;
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    PREPARE_ENTRY.call(c, 0x0027_67f8)?;
                    let prepared = c.g[V0][0];
                    oc538_field(c, S0, 0xbc).sw(c, prepared);
                }
            }
            if !skip_h_end {
                c.cycles += 1;
                c.g[V0][0] = oc538_field(c, S0, 0xa8).lw(c);
            }
            // Unless the transition is paused, advance to the next entry.
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
                break 'exit;
            }
            c.cycles += 2;
            if c.g[S3][0] == 0 {
                c.g[V0][0] = oc538_field(c, S0, 0xbc).lw(c);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                START_TRANSITION.call(c, 0x0027_6818)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
                    break 'exit;
                }
                c.cycles += 1;
                c.g[V0][0] = oc538_field(c, S0, 0xbc).lw(c);
            }
            c.cycles += 2;
            let no_entry = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S0][0];
            if no_entry {
                skip_reset(c)?;
                break 'exit;
            }
            c.cycles += 4;
            TRY_ADVANCE.call(c, 0x0027_6834)?;
            c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The reset branch of `ScriptList_Skip` (`tNISList_Reset(self, 1)`): stop the list with
/// `ScriptList_Stop(self, 1)`, then return the state. `a0` is the list.
fn skip_reset(c: &mut Cpu) -> Flow {
    const STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };
    c.cycles += 2;
    c.g[A1][0] = 1;
    STOP.call(c, 0x0027_6848)?;
    c.cycles += 1;
    c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
    Ok(())
}

/// `ScriptList_GetEntryScriptIndex(self, i)` (0x276a78): the script index of queue entry
/// `i`, or -1 when `i` is out of range or the entry is a movie. An entry whose own index
/// (+4) is negative takes the index of its request (`ScriptEngine_GetProcessScriptId`)
/// when it has a handle (+0xc).
pub fn script_list_get_entry_script_index(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_SCRIPT_ID: Callee = Callee {
        addr: 0x0028_27f8,
        f: ee::s07::f_002827f8,
    };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        CMD_QUEUE_COUNT.call(c, 0x0027_6aa0)?;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        let mut script_entry = false;
        if in_range {
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            CMD_QUEUE_AT.call(c, 0x0027_6ab4)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            IS_MOVIE_ENTRY.call(c, 0x0027_6ac4)?;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = oc538_field(c, S0, 0x4).lw(c);
                script_entry = true;
            }
        }
        if script_entry {
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 3;
                c.g[A1][0] = oc538_field(c, S0, 0xc).lw(c);
                let no_handle = (c.g[A1][0] as i64) < 0;
                c.g[V0][0] = u64::MAX;
                if !no_handle {
                    c.cycles += 2;
                    c.g[A0][0] = oc538_field(c, S1, 0).lw(c);
                    PROCESS_SCRIPT_ID.call(c, 0x0027_6af0)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        } else {
            // Out of range or a movie entry: -1.
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
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

/// `ScriptList_GetEntryScript(self, i)` (0x276b08): the request player of queue entry `i`
/// (`ScriptEngine_GetProcessInSeq` of its handle) when `i` is in range, the entry is not a
/// movie and its handle (+0xc) is set. Otherwise 0.
pub fn script_list_get_entry_script(c: &mut Cpu) {
    const CMD_QUEUE_COUNT: Callee = Callee {
        addr: 0x0028_3d28,
        f: ee::s07::f_00283d28,
    };
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::f_00282bf0,
    };
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        CMD_QUEUE_COUNT.call(c, 0x0027_6b30)?;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let in_range = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.g[S2][0];
            CMD_QUEUE_AT.call(c, 0x0027_6b44)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            IS_MOVIE_ENTRY.call(c, 0x0027_6b54)?;
            let movie = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if movie {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 3;
                c.g[A1][0] = oc538_field(c, S0, 0xc).lw(c);
                let no_handle = (c.g[A1][0] as i64) < 0;
                frame.restore(c, S0, 0x30);
                if !no_handle {
                    c.cycles += 4;
                    c.g[A0][0] = oc538_field(c, S1, 0).lw(c);
                    PROCESS_IN_SEQ.call(c, 0x0027_6b70)?;
                    frame.restore(c, S0, 0x30);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ScriptList_IsNextReady(self)` (0x277100): 0 while the list is in state 1, 1 when there
/// is no pending entry (`nextPending < 0`). Otherwise the pending entry is checked: a movie
/// is ready when its handle (+0xc) is non-negative, anything else when its request is in
/// state 2 (`ScriptEngine_GetProcessState`).
pub fn script_list_is_next_ready(c: &mut Cpu) {
    const CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::f_00283d70,
    };
    const IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::f_00277dd8,
    };
    const PROCESS_STATE: Callee = Callee {
        addr: 0x0028_2798,
        f: ee::s07::f_00282798,
    };
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = oc538_field(c, S0, 0xa4).lw(c);
        if c.g[V0][0] == c.g[V1][0] {
            // State 1: not ready.
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[A1][0] = oc538_field(c, S0, 0xbc).lw(c);
            c.cycles += 2;
            let pending = (c.g[A1][0] as i64) >= 0;
            c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if !pending {
                c.cycles += 2;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.g[S2][0];
                CMD_QUEUE_AT.call(c, 0x0027_7148)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                IS_MOVIE_ENTRY.call(c, 0x0027_7158)?;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = oc538_field(c, S1, 0xc).lw(c);
                    c.cycles += 2;
                    c.g[V0][0] = !c.g[V0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 31);
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = oc538_field(c, S0, 0xbc).lw(c);
                    c.g[A0][0] = c.g[S2][0];
                    CMD_QUEUE_AT.call(c, 0x0027_716c)?;
                    c.g[A0][0] = oc538_field(c, S0, 0).lw(c);
                    c.g[A1][0] = oc538_field(c, V0, 0xc).lw(c);
                    PROCESS_STATE.call(c, 0x0027_7178)?;
                    c.g[V0][0] ^= 2;
                    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the callee fakes below act only when it holds
    /// `OC538_MAGIC` (the second word is the start state the instance fake writes).
    const OC538_GATE: u32 = DATA + 0x2f_0538;
    const OC538_MAGIC: u32 = 0x0538_0001;

    /// A gated fake: falls back to the previous body unless the gate is set, else it logs
    /// a call to `$addr` and returns through the logged return address.
    macro_rules! gated_fake {
        ($fake:ident, $prev:ident, $addr:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC538_GATE) != OC538_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    gated_fake!(
        oc538_fake_release,
        OC538_PREV_RELEASE,
        0x0027_46f8,
        ee::s07::f_002746f8__generated
    );
    gated_fake!(
        oc538_fake_delete,
        OC538_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    gated_fake!(
        oc538_fake_pop,
        OC538_PREV_POP,
        0x0027_7598,
        ee::s07::f_00277598__generated
    );
    gated_fake!(
        oc538_fake_prepare,
        OC538_PREV_PREPARE,
        0x0027_6270,
        ee::s07::f_00276270__generated
    );
    gated_fake!(
        oc538_fake_stop,
        OC538_PREV_STOP,
        0x0027_6868,
        ee::s07::f_00276868__generated
    );
    gated_fake!(
        oc538_fake_prepare_entry,
        OC538_PREV_PREPARE_ENTRY,
        0x0027_7450,
        ee::s07::f_00277450__generated
    );
    gated_fake!(
        oc538_fake_kill,
        OC538_PREV_KILL,
        0x0028_2540,
        ee::s07::f_00282540__generated
    );
    gated_fake!(
        oc538_fake_movie_stop,
        OC538_PREV_MOVIE_STOP,
        0x0028_3818,
        ee::s07::f_00283818__generated
    );
    gated_fake!(
        oc538_fake_try_advance,
        OC538_PREV_TRY_ADVANCE,
        0x0027_7800,
        ee::s07::f_00277800__generated
    );

    static OC538_PREV_START: AtomicUsize = AtomicUsize::new(0);
    static OC538_PREV_UPDATE: AtomicUsize = AtomicUsize::new(0);

    /// `InSeq_InstanceStart` fake: the instance's state word (+0x20) takes the gate's
    /// second word, so the run-to-end test covers both the idle and the looping path.
    fn oc538_fake_start(c: &mut Cpu) {
        if c.bus.read32(OC538_GATE) != OC538_MAGIC {
            return fakes::fall_back(c, &OC538_PREV_START, ee::s07::f_00274830__generated);
        }
        let inst = c.g[A0][0] as u32;
        let state = c.bus.read32(OC538_GATE + 4);
        c.bus.write32(inst + 0x20, state);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0027_4830, ra);
    }

    /// `InSeq_InstanceUpdate` fake: the instance stops at once (state word 0).
    fn oc538_fake_update(c: &mut Cpu) {
        if c.bus.read32(OC538_GATE) != OC538_MAGIC {
            return fakes::fall_back(c, &OC538_PREV_UPDATE, ee::s07::f_00274a30__generated);
        }
        let inst = c.g[A0][0] as u32;
        c.bus.write32(inst + 0x20, 0);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0027_4a30, ra);
    }

    /// Installs the fakes (once per process; each one acts only behind the gate).
    fn install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002746F8),
            oc538_fake_release,
            &OC538_PREV_RELEASE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc538_fake_delete,
            &OC538_PREV_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274830),
            oc538_fake_start,
            &OC538_PREV_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00274A30),
            oc538_fake_update,
            &OC538_PREV_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277598),
            oc538_fake_pop,
            &OC538_PREV_POP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276270),
            oc538_fake_prepare,
            &OC538_PREV_PREPARE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            oc538_fake_stop,
            &OC538_PREV_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277450),
            oc538_fake_prepare_entry,
            &OC538_PREV_PREPARE_ENTRY,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00282540),
            oc538_fake_kill,
            &OC538_PREV_KILL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00283818),
            oc538_fake_movie_stop,
            &OC538_PREV_MOVIE_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00277800),
            oc538_fake_try_advance,
            &OC538_PREV_TRY_ADVANCE,
        );
    }

    /// Scrambled machine with the callee gate on (start state from the seed).
    fn gated(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC538_GATE, OC538_MAGIC);
        c.bus.write32(OC538_GATE + 4, seed % 2);
        r
    }

    /// A script list at `list`: engine pointer at +0, command queue at +0xc (full flag at
    /// +0x94, head/tail at +0x8c/+0x90, five 28-byte entries with a type word and a
    /// handle at +0xc).
    fn queue_fixture(c: &mut Cpu, r: &mut Rng, list: u32, seed: u32) {
        let queue = list + 0xc;
        c.bus.write32(list, DATA + 0x1000);
        c.bus
            .write32(queue + 0x94, if seed % 5 == 0 { 1 } else { 0 });
        c.bus.write32(queue + 0x8c, r.next_u32() % 6);
        c.bus.write32(queue + 0x90, r.next_u32() % 6);
        for k in 0..5 {
            let v = if (seed + k) % 2 == 0 {
                r.next_u32() % 29
            } else {
                29 + r.next_u32() % 100
            };
            c.bus.write32(queue + k * 28, v);
            let handle = if (seed + k) % 3 == 0 {
                u32::MAX
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(queue + k * 28 + 0xc, handle);
        }
    }

    #[test]
    fn in_seq_instance_dtor_matches() {
        install_fakes();
        check(
            ee::s07::f_00274578__generated,
            in_seq_instance_dtor,
            200,
            &|c, seed| {
                gated(c, seed, 0x274578);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = (seed % 4) as u64;
                c.pc = 0x0027_4578;
            },
        );
    }

    #[test]
    fn in_seq_instance_run_to_end_matches() {
        install_fakes();
        check(
            ee::s07::f_00274868__generated,
            in_seq_instance_run_to_end,
            200,
            &|c, seed| {
                let mut r = gated(c, seed, 0x274868);
                let inst = DATA + 0x600;
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(inst + 0x1c, r.next_u32());
                c.bus.write32(inst + 0x24, r.next_u32());
                c.f[20] = r.next_u32();
                c.f[21] = r.next_u32();
                c.pc = 0x0027_4868;
            },
        );
    }

    #[test]
    fn script_list_add_movie_matches() {
        install_fakes();
        check(
            ee::s07::f_00276048__generated,
            script_list_add_movie,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x276048);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                // States 0 (idle) to 3; pending entry -1 to 3.
                let states = [0u32, 1, 2, 3];
                c.bus.write32(list + 0xa4, states[(seed % 4) as usize]);
                c.bus.write32(list + 0xbc, (seed % 5).wrapping_sub(1));
                c.pc = 0x0027_6048;
            },
        );
    }

    #[test]
    fn script_list_get_entry_script_index_matches() {
        install_fakes();
        check(
            ee::s07::f_00276a78__generated,
            script_list_get_entry_script_index,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x276a78);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (seed % 6) as u64;
                c.pc = 0x0027_6a78;
            },
        );
    }

    #[test]
    fn script_list_get_entry_script_matches() {
        install_fakes();
        check(
            ee::s07::f_00276b08__generated,
            script_list_get_entry_script,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x276b08);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                c.g[A1][0] = (seed % 6) as u64;
                c.pc = 0x0027_6b08;
            },
        );
    }

    #[test]
    fn script_list_is_next_ready_matches() {
        install_fakes();
        check(
            ee::s07::f_00277100__generated,
            script_list_is_next_ready,
            300,
            &|c, seed| {
                let mut r = gated(c, seed, 0x277100);
                let list = DATA + 0x200;
                queue_fixture(c, &mut r, list, seed);
                c.g[A0][0] = sx(list);
                let states = [0u32, 1, 2, 3];
                c.bus.write32(list + 0xa4, states[(seed % 4) as usize]);
                c.bus.write32(list + 0xbc, (seed % 5).wrapping_sub(1));
                c.pc = 0x0027_7100;
            },
        );
    }
}
