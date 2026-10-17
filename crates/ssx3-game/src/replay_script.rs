//! Work package E7 replay, scripts, game module (docs/investigation/phase2-candidates-3.md).
//!
//! Hand-written, typed replacements for replay / WScript / game-module
//! functions, each with a differential test against the generated body.

use crate::abi;
use crate::call::{call_ptr, guest, Callee, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_1250, game_module_request_state, "GameModule_RequestState"),
    (0x0023_12d8, game_module_is_overlay_state_done, "GameModule_IsOverlayStateDone"),
    (0x0026_e448, replay_find_snapshot, "Replay_FindSnapshot"),
    (0x0027_0238, replay_free_snapshot, "Replay_FreeSnapshot"),
    (0x0027_02f8, replay_seek_to_snapshot, "Replay_SeekToSnapshot"),
    (0x0027_06f0, replay_finish, "Replay_Finish"),
    (0x0027_0970, replay_on_rider_reset, "Replay_OnRiderReset"),
    (0x0030_a2e8, wscript_man_has_obj_script3, "WScriptMan_HasObjScript3"),
    (0x0030_a548, wscript_man_run_obj_script3, "WScriptMan_RunObjScript3"),
    (0x0030_a688, wscript_man_run_obj_script5, "WScriptMan_RunObjScript5"),
];

// ---------------------------------------------------------------------------
// Callees used by the replacements, by their generated entry points.

mod callee {
    use super::Callee;

    pub const PROCESS_STATE_CHANGE: Callee = Callee { addr: 0x0023_0f40, f: ssx3_ee::s06::f_00230f40 };
    pub const SNAPSHOT_LIST_FIND_BY_FRAME: Callee = Callee { addr: 0x0026_e6a0, f: ssx3_ee::s07::f_0026e6a0 };
    pub const SNAPSHOT_LIST_REMOVE: Callee = Callee { addr: 0x0026_e608, f: ssx3_ee::s07::f_0026e608 };
    pub const SNAPSHOT_LIST_PUSH_BACK: Callee = Callee { addr: 0x0026_e5d0, f: ssx3_ee::s07::f_0026e5d0 };
    pub const WORLD_STATE_STREAM_RESET: Callee = Callee { addr: 0x0026_d628, f: ssx3_ee::s07::f_0026d628 };
    pub const REPLAY_SET_STATE: Callee = Callee { addr: 0x0026_f4a0, f: ssx3_ee::s07::f_0026f4a0 };
    pub const REPLAY_END_PLAYBACK: Callee = Callee { addr: 0x0026_f980, f: ssx3_ee::s07::f_0026f980 };
    pub const REPLAY_FIND_RIDER_INDEX: Callee = Callee { addr: 0x0027_0730, f: ssx3_ee::s07::f_00270730 };
    pub const REPLAY_CANCEL_HIGHLIGHT: Callee = Callee { addr: 0x0027_0de0, f: ssx3_ee::s07::f_00270de0 };
    pub const REPLAY_CANCEL_INSTANT_REPLAY: Callee = Callee { addr: 0x0027_0628, f: ssx3_ee::s07::f_00270628 };
    pub const REPLAY_SAVE_RESUME_SNAPSHOT: Callee = Callee { addr: 0x0026_f7f8, f: ssx3_ee::s07::f_0026f7f8 };
    pub const REPLAY_SEEK_TO_SNAPSHOT: Callee = Callee { addr: 0x0027_02f8, f: ssx3_ee::s07::f_002702f8 };
    pub const REPLAY_STEP_FRAME: Callee = Callee { addr: 0x0027_03f0, f: ssx3_ee::s07::f_002703f0 };
    pub const F_0026EF30: Callee = Callee { addr: 0x0026_ef30, f: ssx3_ee::s07::f_0026ef30 };
    pub const F_0026EF80: Callee = Callee { addr: 0x0026_ef80, f: ssx3_ee::s07::f_0026ef80 };
    pub const REPLAY_ENABLE_REPLAY_CAMERAS: Callee = Callee { addr: 0x0026_ee68, f: ssx3_ee::s07::f_0026ee68 };
    pub const REPLAY_RESTORE_GAME_CAMERAS: Callee = Callee { addr: 0x0026_eea0, f: ssx3_ee::s07::f_0026eea0 };
    pub const REPLAY_RESTORE_RESUME_SNAPSHOT: Callee = Callee { addr: 0x0026_f850, f: ssx3_ee::s07::f_0026f850 };
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee { addr: 0x0028_b180, f: ssx3_ee::s07::f_0028b180 };
    pub const SSX_AUDIO_SYSTEM_ON_REPLAY_BEGIN: Callee = Callee { addr: 0x0028_f200, f: ssx3_ee::s07::f_0028f200 };
    pub const SSX_AUDIO_SYSTEM_ON_REPLAY_END: Callee = Callee { addr: 0x0028_f2c0, f: ssx3_ee::s07::f_0028f2c0 };
    pub const F_0026D4D8: Callee = Callee { addr: 0x0026_d4d8, f: ssx3_ee::s07::f_0026d4d8 };
    pub const WSCRIPT_MAN_CALL_FUNC: Callee = Callee { addr: 0x0030_9c88, f: ssx3_ee::s10::f_00309c88 };
    pub const WSCRIPT_PROCESS_LIST_FIND_BY_ID: Callee = Callee { addr: 0x0030_b320, f: ssx3_ee::s10::f_0030b320 };
    pub const WSCRIPT_MAN_START_PROCESS: Callee = Callee { addr: 0x0030_9750, f: ssx3_ee::s10::f_00309750 };
    pub const WSCRIPT_MAN_STOP_PROCESS: Callee = Callee { addr: 0x0030_9798, f: ssx3_ee::s10::f_00309798 };
    pub const WSCRIPT_PROCESS_NEXT_TASK: Callee = Callee { addr: 0x0030_7e10, f: ssx3_ee::s10::f_00307e10 };
    pub const WSCRIPT_PROCESS_TERMINATE: Callee = Callee { addr: 0x0030_7d78, f: ssx3_ee::s10::f_00307d78 };
}

// ---------------------------------------------------------------------------
// Game module.

/// `void* GameModule_GetTransitionMan(GameModule *m, int index)` (0x230698):
/// `m->transition_men[index]` — a pointer array of stride 4 at +0x5c.
pub fn game_module_get_transition_man(c: &mut Cpu) {
    c.cycles += 4;
    let index = abi::arg(c, 1) << 2;
    c.g[A1][0] = sx(index);
    let at = abi::arg(c, 0).wrapping_add(index);
    c.g[A0][0] = sx(at);
    c.g[V0][0] = Addr(at).off(0x5c).lw(c);
    abi::ret(c);
}

/// `void* GameModule_GetSplashManager(GameModule *m, int index)` (0x2306a8):
/// like [`game_module_get_transition_man`] but with the array at +0x68.
pub fn game_module_get_splash_manager(c: &mut Cpu) {
    c.cycles += 4;
    let index = abi::arg(c, 1) << 2;
    c.g[A1][0] = sx(index);
    let at = abi::arg(c, 0).wrapping_add(index);
    c.g[A0][0] = sx(at);
    c.g[V0][0] = Addr(at).off(0x68).lw(c);
    abi::ret(c);
}

/// `void GameModule_RequestState(GameModule *m, int state, int a2, int notify)`
/// (0x231250): store the requested state (+0x210) and argument (+0x21c) and,
/// when `notify` is non-zero, run the state-change callback.
pub fn game_module_request_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        Addr(this).off(0x210).sw(c, abi::arg64(c, 1));
        let notify = abi::arg64(c, 3) != 0;
        Addr(this).off(0x21c).sw(c, abi::arg64(c, 2));
        if notify {
            c.cycles += 2;
            callee::PROCESS_STATE_CHANGE.call(c, 0x0023_126c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool GameModule_IsOverlayStateDone(GameModule *m)` (0x2312d8): false when
/// an overlay is pending (+0x208) or there is none (+0x204), otherwise the
/// result of the overlay interface's virtual "is done" method (vtable +0x44).
pub fn game_module_is_overlay_state_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[V0][0] = Addr(this).off(0x208).lw(c);
        let pending = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !pending {
            c.cycles += 3;
            let obj = Addr(this).off(0x204).lw(c);
            c.g[A1][0] = obj;
            c.g[V0][0] = 1;
            if obj != 0 {
                c.cycles += 7;
                let vt = Addr(obj as u32).off(0xc).lw(c);
                c.g[V1][0] = vt;
                let delta = Addr(vt as u32).off(0x40).lh(c);
                c.g[A0][0] = delta;
                let func = Addr(vt as u32).off(0x44).lw(c);
                c.g[V0][0] = func;
                c.g[A0][0] = abi::addu(obj, delta);
                call_ptr(c, func as u32, 0x0023_130c)?;
                c.g[RA][0] = c.ld(frame.at(c, 0));
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 1;
        c.g[RA][0] = c.ld(frame.at(c, 0));
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay snapshots.

/// `Snapshot* Replay_FindSnapshot(Replay *r, int frame)` (0x26e448): search the
/// snapshot list at +0x3b0 for `frame`.
pub fn replay_find_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(0x3b0));
        callee::SNAPSHOT_LIST_FIND_BY_FRAME.call(c, 0x0026_e458)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_HighlightSlotClear(ReplayHighlight *h)` (0x26e800): clear the
/// bit `h->slot` (+0xb0 -> record +0x1e) and forget the record.
pub fn replay_highlight_slot_clear(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[A2][0] = abi::arg64(c, 0);
    let slot = Addr(this).off(0xb0).lw(c);
    c.g[A1][0] = slot;
    c.g[V1][0] = 1;
    if slot != 0 {
        c.cycles += 7;
        let idx = Addr(this).lw(c);
        c.g[V0][0] = idx;
        c.g[A0][0] = Addr(slot as u32).off(0x1e).lbu(c);
        c.g[V1][0] = !sx(1u32 << (idx & 31));
        c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
        Addr(slot as u32).off(0x1e).sb(c, c.g[A0][0]);
        Addr(this).off(0xb0).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void Replay_FreeSnapshot(Replay *r, Snapshot *s)` (0x270238): remove the
/// snapshot from the live list (+0x3b0), reset its stream and push it back
/// onto the free list (+0x3bc).
pub fn replay_free_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        let snap = abi::arg64(c, 1);
        c.g[S0][0] = snap;
        c.g[S1][0] = this;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x3b0));
        callee::SNAPSHOT_LIST_REMOVE.call(c, 0x0027_0258)?;
        c.g[A0][0] = c.g[S0][0];
        callee::WORLD_STATE_STREAM_RESET.call(c, 0x0027_0260)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = sx((this as u32).wrapping_add(0x3bc));
        callee::SNAPSHOT_LIST_PUSH_BACK.call(c, 0x0027_026c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekToSnapshot(Replay *r, Snapshot *s)` (0x2702f8): select the
/// snapshot, clear the pending-seek marker (+0x604), then process the two
/// snapshot slots at +0x48c/+0x490.
pub fn replay_seek_to_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        let arg = abi::arg64(c, 1);
        Addr(this as u32).off(0x604).sw(c, 0);
        c.cycles += 2;
        if arg != 0 {
            Addr(this as u32).off(0x488).sw(c, arg);
        } else {
            let v = Addr(this as u32).off(0x3b0).lw(c);
            Addr(this as u32).off(0x488).sw(c, v);
        }
        c.cycles += 6;
        let snap = Addr(this as u32).off(0x488).lw(c);
        c.g[V1][0] = snap;
        c.g[S1][0] = sx((this as u32).wrapping_add(0x48c));
        c.g[S2][0] = 1;
        let v = Addr(snap as u32).off(0x30).lw(c);
        c.g[V0][0] = v;
        Addr(this as u32).off(0x484).sw(c, v);
        let mut cur = Addr(this as u32).off(0x48c).lw(c);
        c.g[V0][0] = cur;
        loop {
            c.cycles += 4;
            let count = (c.g[S2][0] as u32).wrapping_sub(1) as i32;
            c.g[S2][0] = sx(count as u32);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            let zero = cur == 0;
            c.g[A0][0] = cur;
            if !zero {
                c.cycles += 2;
                c.g[A1][0] = Addr(this as u32).off(0x484).lw(c);
                callee::F_0026D4D8.call(c, 0x0027_0358)?;
            }
            c.cycles += 2;
            if count >= 0 {
                cur = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = cur;
                if crate::events::event_due(c) {
                    crate::events::deliver_event_here(c, 0x0027_0340);
                }
                continue;
            }
            c.cycles += 6;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void Replay_Finish(Replay *r)` (0x2706f0): end playback if it is active
/// (+0x61c) and move the replay machine to state 0xd.
pub fn replay_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(this as u32).off(0x61c).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            callee::REPLAY_END_PLAYBACK.call(c, 0x0027_0714)?;
        }
        c.g[A0][0] = c.g[S0][0];
        c.cycles += 6;
        c.g[A1][0] = 0xd;
        callee::REPLAY_SET_STATE.call(c, 0x0027_0720)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_OnRiderReset(Replay *r, int rider)` (0x270970): if no instant
/// replay is running, cancel any pending highlight/instant-replay for `rider`.
pub fn replay_on_rider_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.g[A0][0] = abi::arg64(c, 1);
        c.g[V0][0] = Addr(this as u32).off(0x61c).lw(c);
        // The original's `cycles += 1` before the epilogue runs only on the
        // paths that fall out of the inner block (no pending replay, or a
        // rider index was cancelled); the other paths `break` past it.
        let mut fall_through = true;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(this as u32).off(0x620).lw(c);
            if c.g[V0][0] != 0 {
                fall_through = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(this as u32).lw(c);
                if c.g[V0][0] != 0 {
                    fall_through = false;
                } else {
                    c.cycles += 4;
                    callee::REPLAY_FIND_RIDER_INDEX.call(c, 0x0027_09ac)?;
                    if (c.g[V0][0] as i64) < 0 {
                        fall_through = false;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        callee::REPLAY_CANCEL_HIGHLIGHT.call(c, 0x0027_09c0)?;
                        c.g[A0][0] = c.g[S0][0];
                        callee::REPLAY_CANCEL_INSTANT_REPLAY.call(c, 0x0027_09c8)?;
                    }
                }
            }
        }
        if fall_through {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 3;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScript scripts.

/// `void* WScriptMan_HasObjScript3(WScriptMan *m, Obj *o)` (0x30a2e8): call the
/// object's script-3 predicate through the manager's cache.
pub fn wscript_man_has_obj_script3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 3;
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(abi::arg(c, 0)).off(0x28c).lw(c);
        call_ptr(c, 0x003a_6b78, 0x0030_a2fc)?;
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_RunObjScript3(WScriptMan *m, Obj *o)` (0x30a548): store the
/// script handle, look up script 3 and, if present, run it.
pub fn wscript_man_run_obj_script3(c: &mut Cpu) {
    wscript_man_run_obj_script(c, 3, 0x0030_a56c, 0x0030_a584)
}

/// `void WScriptMan_RunObjScript5(WScriptMan *m, Obj *o)` (0x30a688): as
/// [`wscript_man_run_obj_script3`] but for script 5.
pub fn wscript_man_run_obj_script5(c: &mut Cpu) {
    wscript_man_run_obj_script(c, 5, 0x0030_a6ac, 0x0030_a6c4)
}

/// Shared body of `WScriptMan_RunObjScript3`/`5`.
fn wscript_man_run_obj_script(c: &mut Cpu, script: u64, lookup_ret: u32, call_ret: u32) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = abi::arg64(c, 1);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let man = abi::arg64(c, 0);
        c.g[S0][0] = man;
        Addr(man as u32).off(0x290).sw(c, c.g[V0][0]);
        c.g[A2][0] = script;
        c.g[A0][0] = Addr(man as u32).off(0x28c).lw(c);
        call_ptr(c, 0x003a_6b78, lookup_ret)?;
        c.g[V1][0] = !(0u64 | c.g[V0][0]);
        let handled = c.g[V1][0] != 0;
        c.sw(crate::call::sp(c), c.g[V0][0]);
        if handled {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(crate::call::sp(c));
            callee::WSCRIPT_MAN_CALL_FUNC.call(c, call_ret)?;
        }
        c.cycles += 5;
        Addr(man as u32).off(0x290).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_CallFunc(WScriptMan *m, ScriptStack *s)` (0x309c88):
/// dispatch a script command through the script cache/VM.
pub fn wscript_man_call_func(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save_ra(c, 0x30);
        let man = abi::arg64(c, 0);
        let arg = abi::arg64(c, 1);
        c.g[S0][0] = man;
        c.g[S1][0] = arg;
        c.g[A0][0] = Addr(man as u32).off(0x28c).lw(c);
        call_ptr(c, 0x003a_6d18, 0x0030_9cac)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0022_6600, 0x0030_9cbc)?;
        c.g[V0][0] = Addr(arg as u32).lbu(c);
        c.g[S2][0] = sx(crate::call::sp(c).wrapping_add(0x20));
        Addr(crate::call::sp(c)).off(0x18).sw(c, 0);
        c.g[A0][0] = c.g[S2][0];
        Addr(crate::call::sp(c)).off(0x10).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x3cc).lw(c);
        call_ptr(c, 0x0022_24a8, 0x0030_9ce0)?;
        c.g[S0][0] = sx(crate::call::sp(c).wrapping_add(0x10));
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0022_24b8, 0x0030_9cf8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0022_5b90, 0x0030_9d04)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void WScriptMan_ProcessCommand(WScriptMan *m, int id, int command)`
/// (0x30a728): start/stop/advance/terminate a script process by command
/// (1 = start, 2 = stop, 0x12 = next task, 0 = terminate).
pub fn wscript_man_process_command(c: &mut Cpu) {
    #[derive(Clone, Copy)]
    enum Kind {
        Start,
        Stop,
        Next,
        Terminate,
        Return,
    }
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let id = abi::arg64(c, 1);
        c.g[V1][0] = id;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        let man = abi::arg64(c, 0);
        c.g[S0][0] = man;
        let cmd = abi::arg64(c, 2);
        let s0 = man as u32;
        let mut proc = Addr(s0).off(0x2a4).lw(c);
        c.g[A1][0] = proc;

        let kind = if cmd == 1 {
            Kind::Start
        } else {
            c.cycles += 3;
            if (cmd as i64) < 2 {
                c.cycles += 2;
                if cmd == 0 {
                    Kind::Terminate
                } else {
                    c.cycles += 2;
                    Kind::Return
                }
            } else {
                c.cycles += 2;
                if cmd == 2 {
                    Kind::Stop
                } else {
                    c.cycles += 2;
                    if cmd == 0x12 {
                        Kind::Next
                    } else {
                        c.cycles += 2;
                        Kind::Return
                    }
                }
            }
        };

        // The process list base and the entry point differ per command.
        let (list_off, find_ret, action, action_ret, action_extra) = match kind {
            Kind::Start => (0x2b4, 0x0030_a798, callee::WSCRIPT_MAN_START_PROCESS, 0x0030_a7ac, 4),
            Kind::Stop => (0x2b8, 0x0030_a7d0, callee::WSCRIPT_MAN_STOP_PROCESS, 0x0030_a7e4, 4),
            Kind::Next => (0x2b8, 0x0030_a808, callee::WSCRIPT_PROCESS_NEXT_TASK, 0x0030_a81c, 4),
            Kind::Terminate => (0x2b8, 0x0030_a840, callee::WSCRIPT_PROCESS_TERMINATE, 0x0030_a854, 3),
            Kind::Return => (0, 0, callee::WSCRIPT_PROCESS_TERMINATE, 0, 0),
        };
        if !matches!(kind, Kind::Return) {
            c.cycles += 4;
            if id != u64::MAX {
                c.cycles += 4;
                c.g[A1][0] = id;
                c.g[A0][0] = sx(s0.wrapping_add(list_off));
                callee::WSCRIPT_PROCESS_LIST_FIND_BY_ID.call(c, find_ret)?;
                proc = c.g[V0][0];
                c.g[A1][0] = proc;
            }
            c.cycles += 2;
            if proc != 0 {
                c.cycles += action_extra;
                if matches!(kind, Kind::Start | Kind::Stop) {
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.g[A0][0] = c.g[A1][0];
                }
                action.call(c, action_ret)?;
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, ptr, vtable};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 300;

    #[test]
    fn game_module_get_transition_man_matches() {
        check(ssx3_ee::s06::f_00230698, game_module_get_transition_man, SEEDS, &|c, seed| {
            let mut r = Rng(0x1000_0001 ^ seed as u64);
            machine(c, seed, 1, 0);
            let i = (r.next_u32() % 8) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = i as u64 | ((r.next_u32() as u64) << 32);
            c.bus.write32(DATA + i * 4 + 0x5c, r.next_u32());
        });
    }

    #[test]
    fn game_module_get_splash_manager_matches() {
        check(ssx3_ee::s06::f_002306a8, game_module_get_splash_manager, SEEDS, &|c, seed| {
            let mut r = Rng(0x1000_0002 ^ seed as u64);
            machine(c, seed, 2, 0);
            let i = (r.next_u32() % 8) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = i as u64;
            c.bus.write32(DATA + i * 4 + 0x68, r.next_u32());
        });
    }

    #[test]
    fn game_module_is_overlay_state_done_matches() {
        covered(ssx3_ee::s06::f_002312d8, game_module_is_overlay_state_done, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 3, 1);
            c.g[A0][0] = DATA as u64;
            let pending = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
            c.bus.write32(DATA + 0x208, pending);
            let obj = DATA + 0x2000;
            let have_obj = seed % 3 != 1;
            c.bus.write32(DATA + 0x204, if have_obj { obj } else { 0 });
            if have_obj {
                let vt = vtable(c, &mut r, &[0x40]);
                c.bus.write32(obj + 0xc, vt);
            }
        });
    }

    #[test]
    fn replay_find_snapshot_matches() {
        check(ssx3_ee::s07::f_0026e448, replay_find_snapshot, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 4, 0);
            c.g[A0][0] = DATA as u64;
            let want = match seed % 4 {
                0 => 0,
                1 => 5,
                2 => 10,
                _ => r.next_u32() % 20,
            };
            c.g[A1][0] = want as u64;
            let head = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
            c.bus.write32(DATA + 0x3b0, head);
            c.bus.write32(DATA + 0x3000 + 0x14, DATA + 0x3100);
            c.bus.write32(DATA + 0x3000 + 0x30, 5);
            c.bus.write32(DATA + 0x3100 + 0x14, 0);
            c.bus.write32(DATA + 0x3100 + 0x30, 10);
        });
    }

    #[test]
    fn replay_highlight_slot_clear_matches() {
        check(ssx3_ee::s07::f_0026e800, replay_highlight_slot_clear, SEEDS, &|c, seed| {
            let mut r = Rng(0x1000_0005 ^ seed as u64);
            machine(c, seed, 5, 0);
            c.g[A0][0] = DATA as u64;
            let idx = match seed % 4 {
                0 => 31,
                1 => 0,
                _ => r.next_u32() % 32,
            };
            c.bus.write32(DATA, idx);
            let slot = if seed % 2 == 0 { 0 } else { DATA + 0x3000 };
            c.bus.write32(DATA + 0xb0, slot);
            c.bus.write8(DATA + 0x3000 + 0x1e, (r.next_u32() & 0xff) as u8);
        });
    }

    #[test]
    fn replay_free_snapshot_matches() {
        check(ssx3_ee::s07::f_00270238, replay_free_snapshot, SEEDS, &|c, seed| {
            machine(c, seed, 6, 0);
            c.g[A0][0] = DATA as u64;
            let snap = DATA + 0x3000;
            c.g[A1][0] = snap as u64;
            // live list at +0x3b0, free list at +0x3bc
            c.bus.write32(DATA + 0x3b0, if seed % 2 == 0 { 0 } else { DATA + 0x3100 });
            c.bus.write32(DATA + 0x3b0 + 4, if seed % 3 == 0 { 0 } else { DATA + 0x3100 });
            c.bus.write32(DATA + 0x3bc, if seed % 2 == 0 { snap } else { 0 });
            c.bus.write32(DATA + 0x3bc + 4, if seed % 3 == 0 { 0 } else { DATA + 0x3200 });
            c.bus.write32(snap + 0x14, if seed % 2 == 0 { 0 } else { DATA + 0x3300 });
            c.bus.write32(snap + 0x18, if seed % 3 == 0 { 0 } else { DATA + 0x3400 });
        });
    }

    #[test]
    fn replay_seek_to_snapshot_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 7, 0);
            c.g[A0][0] = DATA as u64;
            let snap = if seed % 3 == 0 { 0 } else { DATA + 0x3000 };
            c.g[A1][0] = snap as u64;
            c.bus.write32(DATA + 0x388, 0);
            c.bus.write32(DATA + 0x3b0, DATA + 0x3200);
            c.bus.write32(DATA + 0x484, r.next_u32());
            // Two snapshot slots; the first is a valid object with count <= 0
            // so the callee returns at once (and never dereferences a1).
            let slot0 = if seed % 2 == 0 { 0 } else { DATA + 0x4000 };
            c.bus.write32(DATA + 0x48c, slot0);
            c.bus.write32(DATA + 0x490, 0);
            c.bus.write32(DATA + 0x4000, 0);
            c.bus.write32(DATA + 0x4000 + 0xc, 0);
            schedule(c, &mut r, seed, 0x0027_02f8)
        };
        check_events(ssx3_ee::s07::f_002702f8, replay_seek_to_snapshot, SEEDS, &setup);
    }

    #[test]
    fn replay_on_rider_reset_matches() {
        check(ssx3_ee::s07::f_00270970, replay_on_rider_reset, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 8, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ptr(&mut r) as u64;
            c.bus.write32(DATA + 0x61c, if seed % 4 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(
                DATA + 0x620,
                if seed % 4 == 0 && seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
            );
            c.bus.write32(
                DATA,
                if seed % 4 == 0 && seed % 3 == 0 && seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
            );
            // No world: Replay_FindRiderIndex returns -1 immediately.
            c.bus.write32(GAME_GP - 0x848, 0);
        });
    }

    #[test]
    fn game_module_request_state_matches() {
        check(ssx3_ee::s06::f_00231250, game_module_request_state, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 9, 1);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = (seed % 2) as u64;
            // Make GameModule_ProcessStateChange bail out of its dispatch:
            // state field +0x200 = 0 -> straight to its exit path.
            c.bus.write32(DATA + 0x200, 0);
            c.bus.write32(DATA + 0x208, 0);
            c.bus.write32(DATA + 0x204, 0);
            c.bus.write32(DATA + 0x214, 0);
        });
    }

    #[test]
    fn replay_finish_matches() {
        check(ssx3_ee::s07::f_002706f0, replay_finish, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 10, 0);
            c.g[A0][0] = DATA as u64;
            // Replay_EndPlayback early-returns for states 0xe/0xf.
            c.bus.write32(DATA, 0xe);
            c.bus.write32(DATA + 0x61c, if seed % 2 == 0 { r.next_u32() | 1 } else { 0 });
        });
    }

    #[test]
    fn wscript_man_has_obj_script3_matches() {
        check(ssx3_ee::s10::f_0030a2e8, wscript_man_has_obj_script3, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 11, 0);
            c.g[A0][0] = DATA as u64;
            let obj = DATA + 0x100;
            c.g[A1][0] = obj as u64;
            setup_script_lookup(c, &mut r, obj);
        });
    }

    #[test]
    fn wscript_man_run_obj_script3_matches() {
        check(ssx3_ee::s10::f_0030a548, wscript_man_run_obj_script3, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 12, 0);
            c.g[A0][0] = DATA as u64;
            let obj = DATA + 0x100;
            c.g[A1][0] = obj as u64;
            setup_script_lookup(c, &mut r, obj);
        });
    }

    #[test]
    fn wscript_man_run_obj_script5_matches() {
        check(ssx3_ee::s10::f_0030a688, wscript_man_run_obj_script5, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 13, 0);
            c.g[A0][0] = DATA as u64;
            let obj = DATA + 0x100;
            c.g[A1][0] = obj as u64;
            setup_script_lookup(c, &mut r, obj);
        });
    }

    /// The manager cache and object fields `FUN_003a6b78`/`FUN_003ad120` read,
    /// arranged so the lookup returns "no script" (obj+0x88 == 0).
    fn setup_script_lookup(c: &mut Cpu, r: &mut Rng, obj: u32) {
        c.bus.write32(DATA + 0x28c, DATA + 0x200);
        c.bus.write32(DATA + 0x204, DATA + 0x300);
        c.bus.write32(DATA + 0x304, 0);
        c.bus.write32(obj + 0x78, r.next_u32() & 0xff);
        c.bus.write32(obj + 0x88, 0);
    }

    #[test]
    fn wscript_man_process_command_matches() {
        check(ssx3_ee::s10::f_0030a728, wscript_man_process_command, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 14, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 3 {
                0 => u64::MAX,
                1 => 0,
                _ => r.next_u32() as u64,
            };
            c.g[A2][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 0x12,
                _ => r.next_u32() as u64,
            };
            // Empty process lists and no default process: the find returns 0
            // and no action is taken.
            c.bus.write32(DATA + 0x2a4, 0);
            c.bus.write32(DATA + 0x2b4, 0);
            c.bus.write32(DATA + 0x2b8, 0);
        });
    }
}
