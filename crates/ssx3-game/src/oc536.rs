//! Staging module oc536 (wave wave231): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc536`.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0027_71a8, script_list_is_full, "ScriptList_IsFull"),
    (
        0x0027_7248,
        script_list_is_playing_movie,
        "ScriptList_IsPlayingMovie",
    ),
    (0x0027_8590, script_engine_purge, "ScriptEngine_Shutdown"),
    (
        0x0027_8718,
        script_engine_clear_skip_delay_on_exit,
        "ScriptEngine_ClearSkipDelayOnExit",
    ),
    (
        0x0027_8de8,
        script_engine_set_skip_delay,
        "ScriptEngine_SetSkipDelay",
    ),
    (
        0x0027_8ff8,
        script_engine_list_transfer,
        "ScriptEngine_ListTransfer",
    ),
    (
        0x0027_9178,
        script_engine_list_get_entry_ext_type,
        "ScriptEngine_ListGetEntryExtType",
    ),
    (
        0x0027_91a8,
        script_engine_list_get_entry_script_index,
        "ScriptEngine_ListGetEntryScriptIndex",
    ),
    (
        0x0027_9270,
        script_engine_list_is_next_ready,
        "ScriptEngine_ListIsNextReady",
    ),
    (
        0x0027_93b8,
        script_engine_aux_list_add,
        "ScriptEngine_AuxListAdd",
    ),
    (
        0x0027_93f8,
        script_engine_aux_list_prepare,
        "ScriptEngine_AuxListPrepare",
    ),
    (
        0x0027_9428,
        script_engine_aux_list_start,
        "ScriptEngine_AuxListStart",
    ),
    (
        0x0027_9458,
        script_engine_aux_list_stop,
        "ScriptEngine_AuxListStop",
    ),
    (
        0x0027_94e0,
        script_engine_aux_list_get_state,
        "ScriptEngine_AuxListGetState",
    ),
    (
        0x0027_96b8,
        script_engine_spawn_script_by_id,
        "ScriptEngine_SpawnScriptById",
    ),
    (0x0027_9da8, script_engine_v03, "ScriptEngine_v03"),
];

/// `bool ScriptList_IsFull(ScriptList* list)` (0x2771a8): whether the command
/// queue embedded at `list + 0xc` (a `cCircularBuffer`) is full.
pub fn script_list_is_full(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0xc));
        Callee {
            addr: 0x0028_3c58,
            f: ee::s07::f_00283c58,
        }
        .call(c, 0x0027_71b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool ScriptList_IsPlayingMovie(ScriptList* list)` (0x277248): false unless
/// the list's state word at `+0xa4` is 3 (a movie is playing); then whether the
/// entry at the queue head (`ScriptCmdQueue_At(list + 0xc, 0)`) is a movie entry.
pub fn script_list_is_playing_movie(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 3;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xa4)).lw(c);
        let playing = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = 0;
        if playing {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0;
            Callee {
                addr: 0x0028_3d70,
                f: ee::s07::f_00283d70,
            }
            .call(c, 0x0027_7274)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Callee {
                addr: 0x0027_7dd8,
                f: ee::s07::f_00277dd8,
            }
            .call(c, 0x0027_7280)?;
        } else {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_ClearSkipDelayOnExit(ScriptEngine* e)` (0x278718): zeroes
/// the skip-delay word at `+0x554`.
pub fn script_engine_clear_skip_delay_on_exit(c: &mut Cpu) {
    c.cycles += 2;
    Addr((c.g[A0][0] as u32).wrapping_add(0x554)).sw(c, 0);
    abi::ret(c);
}

/// `void ScriptEngine_SetSkipDelay(ScriptEngine* e, int delay)` (0x278de8): stores
/// `delay` at `+0x554`.
pub fn script_engine_set_skip_delay(c: &mut Cpu) {
    c.cycles += 2;
    let delay = c.g[A1][0];
    Addr((c.g[A0][0] as u32).wrapping_add(0x554)).sw(c, delay);
    abi::ret(c);
}

/// `ScriptEngine_ListTransfer(e, from, to)` (0x278ff8): nothing to move when
/// `from == to` (returns 1); otherwise moves the entries of the list at
/// `e + 0x54c` with `ScriptList_TakeFrom` (the entries are 0xcc bytes each).
pub fn script_engine_list_transfer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A1][0];
        let same = c.g[V0][0] == c.g[A2][0];
        frame.save_ra(c, 0);
        if same {
            c.cycles += 2;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 9;
            let list = c.g[A0][0] as u32;
            c.g[A3][0] = Addr(list.wrapping_add(0x54c)).lw(c);
            c.g[A0][0] = 0xcc;
            let to = c.g[A2][0] as u32;
            c.g[V1][0] = sx(abi::mult(c, to, 0xcc));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            let from = c.g[V0][0] as u32;
            c.g[V1][0] = sx(abi::mult(c, from, 0xcc));
            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
            Callee {
                addr: 0x0027_6198,
                f: ee::s07::f_00276198,
            }
            .call(c, 0x0027_9024)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// The argument setup of the list-entry wrappers: `v1 = a1 * 0xcc` (the byte
/// offset of entry `a1`), then `a0 = *(list + table) + v1` (the entry).
fn oc536_entry_args(c: &mut Cpu, table: u32) {
    c.g[V0][0] = 0xcc;
    let index = c.g[A1][0] as u32;
    c.g[V1][0] = sx(abi::mult(c, index, 0xcc));
    let list = c.g[A0][0] as u32;
    c.g[A0][0] = Addr(list.wrapping_add(table)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
}

/// Shared shape of the `ScriptEngine_ListGetEntry*` wrappers: the entry of
/// `e + 0x54c` at index `a1` is passed on with `a1 = a2`.
fn oc536_list_entry_wrapper(c: &mut Cpu, callee: Callee, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        oc536_entry_args(c, 0x54c);
        c.g[A1][0] = c.g[A2][0];
        callee.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_ListGetEntryExtType(e, index, a2)` (0x279178): the list entry's
/// extended type, through `ScriptList_GetEntryExtType`.
pub fn script_engine_list_get_entry_ext_type(c: &mut Cpu) {
    oc536_list_entry_wrapper(
        c,
        Callee {
            addr: 0x0027_6a08,
            f: ee::s07::f_00276a08,
        },
        0x0027_9198,
    )
}

/// `ScriptEngine_ListGetEntryScriptIndex(e, index, a2)` (0x2791a8): the list
/// entry's script index, through `ScriptList_GetEntryScriptIndex`.
pub fn script_engine_list_get_entry_script_index(c: &mut Cpu) {
    oc536_list_entry_wrapper(
        c,
        Callee {
            addr: 0x0027_6a78,
            f: ee::s07::f_00276a78,
        },
        0x0027_91c8,
    )
}

/// `ScriptEngine_ListIsNextReady(e, index)` (0x279270): `ScriptList_IsNextReady`
/// on the list entry at index `a1` (`*(e + 0x54c) + a1 * 0xcc`).
pub fn script_engine_list_is_next_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xcc;
        frame.save_ra(c, 0);
        let index = c.g[A1][0] as u32;
        c.g[A1][0] = sx(abi::mult(c, index, 0xcc));
        let list = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(list.wrapping_add(0x54c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        Callee {
            addr: 0x0027_7100,
            f: ee::s07::f_00277100,
        }
        .call(c, 0x0027_928c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_AuxListAdd(e, index, choice, flag)` (0x2793b8): adds a choice
/// to the aux list at `*(e + 0x624) + index * 0xcc` (`ScriptList_AddChoice`);
/// a non-zero `flag` passes 8 as the choice's last argument.
pub fn script_engine_aux_list_add(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xcc;
        frame.save_ra(c, 0);
        let index = c.g[A1][0] as u32;
        c.g[T3][0] = sx(abi::mult(c, index, 0xcc));
        c.g[A1][0] = c.g[A2][0];
        c.g[V1][0] = 8;
        let aux = c.g[A0][0] as u32;
        c.g[A0][0] = Addr(aux.wrapping_add(0x624)).lw(c);
        c.g[A2][0] = 0;
        if c.g[A3][0] != 0 {
            c.g[A2][0] = c.g[V1][0];
        }
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T3][0] as u32));
        Callee {
            addr: 0x0027_5ed0,
            f: ee::s07::f_00275ed0,
        }
        .call(c, 0x0027_93e8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The head shared by the `ScriptEngine_AuxList*` wrappers: the entry at
/// `index * 0xcc` of the aux list `*(e + 0x624)` goes to `a0` (`v1` is the offset).
fn oc536_aux_entry_args(c: &mut Cpu) {
    c.g[V0][0] = 0xcc;
    let index = c.g[A1][0] as u32;
    c.g[V1][0] = sx(abi::mult(c, index, 0xcc));
    let aux = c.g[A0][0] as u32;
    c.g[A0][0] = Addr(aux.wrapping_add(0x624)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
}

/// `ScriptEngine_AuxListPrepare(e, index, flag)` (0x2793f8): `ScriptList_Prepare`
/// on the aux entry; `a1 = 2`, or 0 when `flag` is zero.
pub fn script_engine_aux_list_prepare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        oc536_aux_entry_args(c);
        c.g[A1][0] = 2;
        if c.g[A2][0] == 0 {
            c.g[A1][0] = 0;
        }
        Callee {
            addr: 0x0027_6270,
            f: ee::s07::f_00276270,
        }
        .call(c, 0x0027_941c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_AuxListStart(e, index)` (0x279428): `ScriptList_Start` on the
/// aux entry with `a1 = a2 = 0`.
pub fn script_engine_aux_list_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        oc536_aux_entry_args(c);
        c.g[A2][0] = 0;
        c.g[A1][0] = 0;
        Callee {
            addr: 0x0027_6388,
            f: ee::s07::f_00276388,
        }
        .call(c, 0x0027_944c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_AuxListStop(e, index)` (0x279458): `ScriptList_Stop` on the
/// aux entry with `a1 = 1`.
pub fn script_engine_aux_list_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        oc536_aux_entry_args(c);
        c.g[A1][0] = 1;
        Callee {
            addr: 0x0027_6868,
            f: ee::s07::f_00276868,
        }
        .call(c, 0x0027_9478)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_AuxListGetState(e, index)` (0x2794e0): polls the async loads,
/// then `ScriptList_GetState` on the aux entry (`index` kept in `s0`).
pub fn script_engine_aux_list_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        Callee {
            addr: 0x0027_8918,
            f: ee::s07::f_00278918,
        }
        .call(c, 0x0027_94fc)?;
        c.g[V0][0] = 0xcc;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x624)).lw(c);
        let index = c.g[S0][0] as u32;
        c.g[S0][0] = sx(abi::mult(c, index, 0xcc));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
        Callee {
            addr: 0x0027_6b98,
            f: ee::s07::f_00276b98,
        }
        .call(c, 0x0027_9510)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_SpawnScriptById(e, id)` (0x2796b8): spawns the first entry
/// of the table at `*(e + 0x530)` (count at `*(e + 0x52c)`, 0xc-byte entries)
/// whose id word (`+4`) equals `id`, returning what `ScriptEngine_SpawnScript`
/// returns; -1 when none matches.
pub fn script_engine_spawn_script_by_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[T2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x52c)).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        let has_entries = (c.g[V0][0] as i64) > 0;
        c.g[A1][0] = 0;
        if has_entries {
            c.cycles += 1;
            c.g[T1][0] = 0;
            loop {
                c.cycles += 5;
                let base = Addr((c.g[A0][0] as u32).wrapping_add(0x530)).lw(c);
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(base as u32));
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(4)).lw(c);
                if c.g[V1][0] == c.g[T2][0] {
                    c.cycles += 4;
                    Callee {
                        addr: 0x0027_95d0,
                        f: ee::s07::f_002795d0,
                    }
                    .call(c, 0x0027_96f4)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    frame.close(c);
                    return Ok(());
                }
                c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x52c)).lw(c);
                c.cycles += 5;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_96d8);
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void ScriptEngine_v03(e, obj)` (0x279da8): the removal of a view object.
/// Kind 1 (`obj->0`) removes its camera (`ScriptViewObj_RemoveCam`); kind 2
/// runs the object's callback at `+0x4c` and then `ScriptViewObj_SetTransition`.
/// Both end by removing the script object (`ScriptEngine_RemoveScriptObj`).
pub fn script_engine_v03(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Callee {
            addr: 0x0027_d320,
            f: ee::s07::f_0027d320,
        }
        .call(c, 0x0027_9dc4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V0][0] = 1;
        let kind_is_1 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        if kind_is_1 {
            c.cycles += 8;
            c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x7c)).lw(c);
            c.g[V0][0] = 0x120;
            let slot = c.g[A0][0] as u32;
            c.g[A0][0] = sx(abi::mult(c, slot, 0x120));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Callee {
                addr: 0x0028_11e8,
                f: ee::s07::f_002811e8,
            }
            .call(c, 0x0027_9df0)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 12;
                c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x48)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x4c)).lw(c);
                let callback = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, callback, 0x0027_9e14)?;
                c.g[V1][0] = 0x120;
                c.g[A1][0] = 0;
                let v0 = c.g[V0][0] as u32;
                let scale = c.g[V1][0] as u32;
                c.g[V0][0] = sx(abi::mult(c, v0, scale));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6d0));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Callee {
                    addr: 0x0028_12e0,
                    f: ee::s07::f_002812e0,
                }
                .call(c, 0x0027_9e2c)?;
                c.g[A0][0] = c.g[S0][0];
            }
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        Callee {
            addr: 0x0027_cf88,
            f: ee::s07::f_0027cf88,
        }
        .call(c, 0x0027_9e38)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The table teardown of `ScriptEngine_Purge` (run for the tables at `e+0x54c`
/// and `e+0x624`): when the table at `a1` has entries, `ScriptList_dtor` runs on
/// them from the last one down until the table's head (`e + field`) is reached;
/// returns with `a0 = *(e + field)` for the free that follows.
fn oc536_purge_table(c: &mut Cpu, field: u32, head: u32, dtor_ret: u32) -> Flow {
    c.cycles += 5;
    let table = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(table.wrapping_sub(0x10)).lw(c);
    let count_scaled = c.g[V1][0] as u32;
    c.g[A0][0] = sx(abi::mult(c, count_scaled, 0xcc));
    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(table));
    if c.g[A1][0] == c.g[S0][0] {
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(field)).lw(c);
    } else {
        c.cycles += 2;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xcc));
        loop {
            c.cycles += 6;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x0027_5d90,
                f: ee::s07::f_00275d90,
            }
            .call(c, dtor_ret)?;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(field)).lw(c);
            let more = c.g[V0][0] != c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xcc));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, head);
            }
        }
        c.cycles += 1;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(field)).lw(c);
    }
    Ok(())
}

/// `void ScriptEngine_Purge(ScriptEngine* e)` (0x278590; the dev-build
/// `cSSXScriptEngine::Purge`): stops and unregisters every script object, unloads
/// the world's scripts and banks, removes the data instances, stops the movie,
/// tears down both script tables (the frees go through the function table at
/// 0x317e98), and clears the table pointers and counters.
pub fn script_engine_purge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6d0));
        Callee {
            addr: 0x0027_8a98,
            f: ee::s07::f_00278a98,
        }
        .call(c, 0x0027_85b8)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0028_23f0,
            f: ee::s07::f_002823f0,
        }
        .call(c, 0x0027_85c0)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            Callee {
                addr: 0x0028_32d8,
                f: ee::s07::f_002832d8,
            }
            .call(c, 0x0027_85cc)?;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_85c0);
            }
        }
        c.cycles += 11;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x91c));
        c.g[S0][0] = 0;
        Callee {
            addr: 0x0028_32d8,
            f: ee::s07::f_002832d8,
        }
        .call(c, 0x0027_85e0)?;
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x51c));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x938));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x500));
        Callee {
            addr: 0x0028_32d8,
            f: ee::s07::f_002832d8,
        }
        .call(c, 0x0027_85f0)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0027_8cd8,
            f: ee::s07::f_00278cd8,
        }
        .call(c, 0x0027_85f8)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0027_b410,
                f: ee::s07::f_0027b410,
            }
            .call(c, 0x0027_8608)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 10) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0027_8600);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S2][0];
        Callee {
            addr: 0x0027_5b08,
            f: ee::s07::f_00275b08,
        }
        .call(c, 0x0027_8620)?;
        c.g[A0][0] = c.g[S3][0];
        Callee {
            addr: 0x0028_3818,
            f: ee::s07::f_00283818,
        }
        .call(c, 0x0027_8628)?;
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x54c)).lw(c);
        c.g[V0][0] = 0xcc;
        if c.g[A1][0] != 0 {
            oc536_purge_table(c, 0x54c, 0x0027_8650, 0x0027_865c)?;
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0027_8674)?;
        }
        // The block at 0x27868c: the table at 0x544 is freed, then `a1` takes the
        // second table's pointer.
        c.cycles += 3;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x544)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0027_8688)?;
        }
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x624)).lw(c);
        c.cycles += 2;
        c.g[V0][0] = 0xcc;
        if c.g[A1][0] != 0 {
            oc536_purge_table(c, 0x624, 0x0027_86b0, 0x0027_86bc)?;
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0027_86d4)?;
        }
        c.cycles += 15;
        c.g[A0][0] = 0;
        call_ptr(c, 0x002e_f368, 0x0027_86dc)?;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0027_bf90,
            f: ee::s07::f_0027bf90,
        }
        .call(c, 0x0027_86e4)?;
        Addr((c.g[S1][0] as u32).wrapping_add(0x624)).sw(c, 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0x54c)).sw(c, 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0x544)).sw(c, 0);
        Addr((c.g[S1][0] as u32).wrapping_add(0x548)).sw(c, 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word for the test-only fakes (set in this test's machine only).
    const GATE: u32 = DATA + 0x20_0000;
    const GATE_ON: u32 = 0x0536_c0de;
    /// The script object `ScriptEngine_GetScriptObj` returns while the gate is on.
    const OBJ: u32 = DATA + 0x20_0010;

    /// Gated fake of the free the teardown calls through 0x317e98: a no-op.
    fn oc536_fake_free(c: &mut Cpu) {
        if c.bus.read32(GATE) != GATE_ON {
            return fakes::fall_back(c, fakes::prev_cell(oc536_fake_free), ee::s10::f_00317e98);
        }
        abi::ret(c);
    }

    /// Gated fake of `ScriptEngine_GetScriptObj`: returns `OBJ`.
    fn oc536_fake_script_obj(c: &mut Cpu) {
        if c.bus.read32(GATE) != GATE_ON {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc536_fake_script_obj),
                ee::s07::f_0027d320,
            );
        }
        c.g[V0][0] = OBJ as u64;
        c.cycles += 3;
        abi::ret(c);
    }

    fn install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc536_fake_free,
            fakes::prev_cell(oc536_fake_free),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027D320),
            oc536_fake_script_obj,
            fakes::prev_cell(oc536_fake_script_obj),
        );
    }

    /// Scrambled registers and `a0 = DATA` (the object or list under test).
    fn fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x5360_0536_0000_0001 ^ (seed as u64) << 9 ^ salt);
        scramble(c, &mut r);
        c.g[A0][0] = sx(DATA);
        r
    }

    #[test]
    fn script_list_is_full_matches() {
        check(ee::s07::f_002771a8__generated, script_list_is_full, 200, &|c, seed| {
            let mut r = fixture(c, seed, 1);
            fill(c, &mut r, DATA, 8);
        });
    }

    #[test]
    fn script_list_is_playing_movie_matches() {
        check(
            ee::s07::f_00277248__generated,
            script_list_is_playing_movie,
            200,
            &|c, seed| {
                fixture(c, seed, 2);
                c.bus
                    .write32(DATA + 0xa4, if seed % 3 == 0 { 0 } else { 3 });
            },
        );
    }

    #[test]
    fn script_engine_clear_skip_delay_on_exit_matches() {
        check(
            ee::s07::f_00278718__generated,
            script_engine_clear_skip_delay_on_exit,
            200,
            &|c, seed| {
                let mut r = fixture(c, seed, 3);
                fill(c, &mut r, DATA, 0x200);
            },
        );
    }

    #[test]
    fn script_engine_set_skip_delay_matches() {
        check(
            ee::s07::f_00278de8__generated,
            script_engine_set_skip_delay,
            200,
            &|c, seed| {
                let mut r = fixture(c, seed, 4);
                fill(c, &mut r, DATA, 0x200);
            },
        );
    }

    #[test]
    fn script_engine_list_transfer_matches() {
        check(
            ee::s07::f_00278ff8__generated,
            script_engine_list_transfer,
            200,
            &|c, seed| {
                fixture(c, seed, 5);
                c.bus.write32(DATA + 0x54c, DATA + 0x2000);
                c.g[A1][0] = sx(seed % 5);
                c.g[A2][0] = if seed % 2 == 0 {
                    c.g[A1][0]
                } else {
                    sx(seed % 7 + 1)
                };
            },
        );
    }

    /// Shared fixture of the list-entry wrappers.
    fn list_entry_setup(c: &mut Cpu, seed: u32, salt: u64) {
        fixture(c, seed, salt);
        c.bus.write32(DATA + 0x54c, DATA + 0x2000);
        c.g[A1][0] = sx(seed % 7);
        c.g[A2][0] = sx(seed % 3);
    }

    #[test]
    fn script_engine_list_get_entry_ext_type_matches() {
        check(
            ee::s07::f_00279178__generated,
            script_engine_list_get_entry_ext_type,
            200,
            &|c, seed| list_entry_setup(c, seed, 6),
        );
    }

    #[test]
    fn script_engine_list_get_entry_script_index_matches() {
        check(
            ee::s07::f_002791a8__generated,
            script_engine_list_get_entry_script_index,
            200,
            &|c, seed| list_entry_setup(c, seed, 7),
        );
    }

    #[test]
    fn script_engine_list_is_next_ready_matches() {
        check(
            ee::s07::f_00279270__generated,
            script_engine_list_is_next_ready,
            200,
            &|c, seed| {
                fixture(c, seed, 8);
                c.bus.write32(DATA + 0x54c, DATA + 0x2000);
                c.g[A1][0] = sx(seed % 7);
            },
        );
    }

    /// Shared fixture of the aux-list wrappers.
    fn aux_setup(c: &mut Cpu, seed: u32, salt: u64) {
        fixture(c, seed, salt);
        c.bus.write32(DATA + 0x624, DATA + 0x2000);
        c.g[A1][0] = sx(seed % 7);
        c.g[A2][0] = (seed % 2) as u64;
        c.g[A3][0] = (seed % 3) as u64;
    }

    #[test]
    fn script_engine_aux_list_add_matches() {
        check(
            ee::s07::f_002793b8__generated,
            script_engine_aux_list_add,
            200,
            &|c, seed| aux_setup(c, seed, 9),
        );
    }

    #[test]
    fn script_engine_aux_list_prepare_matches() {
        check(
            ee::s07::f_002793f8__generated,
            script_engine_aux_list_prepare,
            200,
            &|c, seed| aux_setup(c, seed, 10),
        );
    }

    #[test]
    fn script_engine_aux_list_start_matches() {
        check(
            ee::s07::f_00279428__generated,
            script_engine_aux_list_start,
            200,
            &|c, seed| aux_setup(c, seed, 11),
        );
    }

    #[test]
    fn script_engine_aux_list_stop_matches() {
        check(
            ee::s07::f_00279458__generated,
            script_engine_aux_list_stop,
            200,
            &|c, seed| aux_setup(c, seed, 12),
        );
    }

    #[test]
    fn script_engine_aux_list_get_state_matches() {
        check(
            ee::s07::f_002794e0__generated,
            script_engine_aux_list_get_state,
            200,
            &|c, seed| aux_setup(c, seed, 13),
        );
    }

    #[test]
    fn script_engine_spawn_script_by_id_matches() {
        check(
            ee::s07::f_002796b8__generated,
            script_engine_spawn_script_by_id,
            200,
            &|c, seed| {
                fixture(c, seed, 14);
                c.bus.write32(DATA + 0x52c, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, seed % 3);
                c.bus.write32(DATA + 0x530, DATA + 0x3000);
                for i in 0..3u32 {
                    let id = if (seed + i) % 2 == 0 { 5 } else { 9 };
                    c.bus.write32(DATA + 0x3004 + 0xc * i, id);
                }
                c.g[A1][0] = 5;
            },
        );
    }

    #[test]
    fn script_engine_v03_matches() {
        install();
        check(ee::s07::f_00279da8__generated, script_engine_v03, 200, &|c, seed| {
            fixture(c, seed, 15);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GATE, GATE_ON);
            c.g[A1][0] = sx(DATA + 0x4000);
            c.bus.write32(OBJ, seed % 3);
            c.bus.write32(OBJ + 0x7c, seed % 4);
            c.bus.write32(OBJ + 0xc, DATA + 0x5000);
            c.bus.write16(DATA + 0x5048, 0x10);
            c.bus.write32(DATA + 0x504c, 0x0014_06c0);
        });
    }

    #[test]
    fn script_engine_purge_matches() {
        install();
        check(ee::s07::f_00278590__generated, script_engine_purge, 200, &|c, seed| {
            fixture(c, seed, 16);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GATE, GATE_ON);
            let count = seed % 3;
            if seed % 2 == 1 {
                c.bus.write32(DATA + 0x54c, DATA + 0x3000);
                c.bus.write32(DATA + 0x2ff0, count);
            }
            if seed % 4 == 0 {
                c.bus.write32(DATA + 0x624, DATA + 0x4000);
                c.bus.write32(DATA + 0x3ff0, count);
            }
            if seed % 5 == 0 {
                c.bus.write32(DATA + 0x544, 0x0080_1000);
            }
        });
    }
}
