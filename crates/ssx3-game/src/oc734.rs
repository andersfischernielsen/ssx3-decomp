//! Staging module oc734 (wave wave241): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc734`.

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
    (
        0x0030_2ef8,
        wscript_fn_is_rider_in_state,
        "WScriptFn_IsRiderInState",
    ),
    (
        0x0030_34f8,
        wscript_fn_get_rider_state_info,
        "WScriptFn_GetRiderStateInfo",
    ),
    (
        0x0030_36b0,
        wscript_fn_get_event_arg,
        "WScriptFn_GetEventArg",
    ),
    (
        0x0030_37b0,
        wscript_fn_is_rider_in_state_2,
        "WScriptFn_IsRiderInState_2",
    ),
];

/// One of the two rider-query switches (`WScriptFn_IsRiderInState` and its
/// `_2` twin). Offsets are gp-relative where the generated code uses gp;
/// `table`, `copy_ret`, the return addresses and `targets` are absolute.
pub(crate) struct RiderQuery {
    /// The function's entry (the self case restarts from it).
    entry: u32,
    /// The `jump_out` end of the body.
    end: u32,
    /// The static-init flag word, its flag byte, the 8-byte block copied to
    /// `vp` and the per-index kind table.
    flag_off: u32,
    byte_off: u32,
    src_off: u32,
    kinds_off: u32,
    /// The case jump table (one word per `vp[0]`) and its 19 case targets.
    table: u32,
    targets: [u32; 19],
    /// The extra arm that charges one more cycle and lands on case 4.
    quirk: Option<u32>,
    /// Return addresses of the block copy, the call into the query table and
    /// of the int-view call.
    copy_ret: u32,
    call_ret: u32,
    tail_ret: u32,
    /// The arg loop's head and its entry charge.
    loop_top: u32,
    entry_cycles: u64,
    /// The static-init body (the only difference between the two switches).
    init: fn(&mut Cpu, &RiderQuery),
}

/// Whether a query pass restarted the body (the self case) or finished it
/// (through the epilogue or a `jump_out`).
enum RiderPass {
    Restart,
    Done,
}

/// The rider-query static init of `WScriptFn_IsRiderInState` (0x302ef8).
fn rider_query_init_a(c: &mut Cpu, q: &RiderQuery) {
    c.cycles += 5;
    let gp = c.g[GP][0] as u32;
    c.g[V1][0] = c.lbu(gp.wrapping_add(q.byte_off));
    c.g[V0][0] = 1;
    c.sw(gp.wrapping_add(q.flag_off), c.g[V0][0]);
    c.sw(gp.wrapping_add(q.src_off), 0);
    let s = sp(c);
    c.sb(s, c.g[V1][0]);
}

/// The rider-query static init of `WScriptFn_IsRiderInState_2` (0x3037b0).
fn rider_query_init_b(c: &mut Cpu, q: &RiderQuery) {
    c.cycles += 6;
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lbu(gp.wrapping_add(q.byte_off));
    c.g[V1][0] = u64::MAX;
    c.g[A0][0] = 1;
    c.sw(gp.wrapping_add(q.src_off), c.g[V1][0]);
    c.sw(gp.wrapping_add(q.flag_off), c.g[A0][0]);
    let s = sp(c);
    c.sb(s, c.g[V0][0]);
}

/// The `(params, n)` loop shared by the WScript bodies: each argument is
/// stored into the block at `vp` by its index, as a float when the kind
/// table says float and the argument's kind differs, else as an int. The
/// caller has set `a2` to the parameter list and checked `count > 0`; `entry`
/// is the loop's entry charge, `top` its back-edge head, `kinds` the kind table.
fn rider_fill_args(c: &mut Cpu, vp: u64, count: u64, kinds: u32, entry: u64, top: u32) {
    c.cycles += entry;
    c.g[A1][0] = count;
    c.g[T0][0] = sx(kinds);
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(vp as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = c.lw(c.g[V0][0] as u32);
        let mut int_store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
            } else {
                c.cycles += 4;
                c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
                c.cvt_s_w(0, 0);
                c.swc1(0, c.g[A0][0] as u32);
                int_store = false;
            }
        }
        if int_store {
            c.cycles += 1;
            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, top);
        }
    }
}

/// The case the loaded jump-table target `target` names: the mission index
/// `n` it passes to the query table and the cycles its arm charges, or None
/// for a target outside the switch.
fn rider_case(q: &RiderQuery, target: u32) -> Option<(u64, u64)> {
    if q.quirk == Some(target) {
        return Some((4, 4));
    }
    let i = q.targets.iter().position(|&t| t == target)?;
    Some((i as u64, if i == 18 { 2 } else { 3 }))
}

/// The static-init-flag prologue of a rider-query switch: saves the
/// registers, and runs the init when the flag is clear.
fn rider_query_prologue(c: &mut Cpu, q: &RiderQuery) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
    let gp = c.g[GP][0] as u32;
    c.g[V0][0] = c.lw(gp.wrapping_add(q.flag_off));
    let s = sp(c);
    c.sq(17, s.wrapping_add(0x50));
    c.sq(18, s.wrapping_add(0x40));
    c.g[S1][0] = c.g[A1][0];
    c.sq(19, s.wrapping_add(0x30));
    c.g[S2][0] = c.g[A2][0];
    c.sq(16, s.wrapping_add(0x60));
    c.g[S3][0] = c.g[A0][0];
    let t = c.g[V0][0] != 0;
    let ra = c.g[RA][0];
    c.sd(s.wrapping_add(0x20), ra);
    if !t {
        (q.init)(c, q);
    }
}

/// The body of a rider-query switch after its prologue: the `vp` block copy
/// and arg loop, the case dispatch (the self case asks for a restart), the
/// `Rider_DispatchQueryTable` call and the int store, then the epilogue.
fn rider_query_rest(c: &mut Cpu, q: &RiderQuery) -> Result<RiderPass, Unwound> {
    // The 8-byte block copy into vp (sp+0x10, kept in s0).
    c.cycles += 7;
    let vp = sx(sp(c).wrapping_add(0x10));
    c.g[S0][0] = vp;
    c.g[A2][0] = 8;
    let gp = c.g[GP][0] as u32;
    c.g[A1][0] = sx(gp.wrapping_add(q.src_off));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003e_6574, q.copy_ret)?;
    let count = c.g[S1][0];
    let no_args = (count as i64) <= 0;
    c.g[A2][0] = c.g[S2][0];
    if !no_args {
        let kinds = gp.wrapping_add(q.kinds_off);
        rider_fill_args(c, vp, count, kinds, q.entry_cycles, q.loop_top);
    }

    // The rider the manager's index names, and the case index (vp[0]).
    c.cycles += 16;
    c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
    c.g[A2][0] = 0;
    c.g[A0][0] = c.lw(gp.wrapping_add(0xce8));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A3][0] = c.lw(vp as u32);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
    c.g[A1][0] = (c.g[A3][0] < 0x13) as u64;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
    if c.g[A0][0] == 0 {
        c.g[V0][0] = 0;
    }
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
    let in_range = c.g[A1][0] != 0;
    if c.g[V0][0] != 0 {
        c.g[A2][0] = c.g[V1][0];
    }

    let res = if !in_range {
        c.cycles += 1;
        0
    } else {
        // The jump table: one target per vp[0], dispatched on its value.
        c.cycles += 7;
        c.g[V0][0] = sx(q.table);
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        let target = c.g[A0][0] as u32;
        if target == q.entry {
            // The self case: the body runs again from its entry.
            if event_due(c) {
                deliver_event_here(c, q.entry);
            }
            return Ok(RiderPass::Restart);
        }
        match rider_case(q, target) {
            Some((n, cycles)) => {
                c.cycles += cycles;
                c.g[A0][0] = c.g[A2][0];
                c.g[A1][0] = n;
            }
            None => {
                c.jump_out(target, q.entry, q.end);
                return Ok(RiderPass::Done);
            }
        }
        c.cycles += 4;
        call_ptr(c, 0x0012_2ee8, q.call_ret)?;
        c.g[V0][0]
    };
    c.g[S0][0] = res;

    // The tail: mark the object as type 1, store the int, return it.
    c.cycles += 13;
    c.g[V0][0] = 1;
    c.g[A0][0] = c.g[S3][0];
    let s3 = c.g[S3][0] as u32;
    c.sw(s3.wrapping_add(0x8), 1);
    call_ptr(c, 0x0022_6610, q.tail_ret)?;
    let s0 = c.g[S0][0];
    c.sw(c.g[V0][0] as u32, s0);
    c.g[V0][0] = c.g[S3][0];
    let s = sp(c);
    c.lq(16, s.wrapping_add(0x60));
    c.lq(17, s.wrapping_add(0x50));
    c.lq(18, s.wrapping_add(0x40));
    c.lq(19, s.wrapping_add(0x30));
    c.g[RA][0] = c.ld(s.wrapping_add(0x20));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
    ret(c);
    Ok(RiderPass::Done)
}

/// Runs a rider-query switch: the prologue, then the body, again from the
/// prologue when the self case restarts it.
fn rider_query_run(c: &mut Cpu, q: &RiderQuery) -> Flow {
    loop {
        rider_query_prologue(c, q);
        match rider_query_rest(c, q)? {
            RiderPass::Restart => continue,
            RiderPass::Done => return Ok(()),
        }
    }
}

/// The `IsRiderInState` case table, indexed by `vp[0]` (0..=18).
const IS_RIDER_IN_STATE_TARGETS: [u32; 19] = [
    0x0030_3004,
    0x0030_3010,
    0x0030_301c,
    0x0030_3028,
    0x0030_3034,
    0x0030_3040,
    0x0030_304c,
    0x0030_3058,
    0x0030_3064,
    0x0030_3070,
    0x0030_307c,
    0x0030_3088,
    0x0030_3094,
    0x0030_30a0,
    0x0030_30ac,
    0x0030_30b8,
    0x0030_30c4,
    0x0030_30d0,
    0x0030_30dc,
];

/// The `IsRiderInState_2` case table, indexed by `vp[0]` (0..=18).
const IS_RIDER_IN_STATE_2_TARGETS: [u32; 19] = [
    0x0030_38c4,
    0x0030_38d0,
    0x0030_38dc,
    0x0030_38e8,
    0x0030_38f4,
    0x0030_3900,
    0x0030_390c,
    0x0030_3918,
    0x0030_3924,
    0x0030_3930,
    0x0030_393c,
    0x0030_3948,
    0x0030_3954,
    0x0030_3960,
    0x0030_396c,
    0x0030_3978,
    0x0030_3984,
    0x0030_3990,
    0x0030_399c,
];

/// The gp-relative and absolute layout of `WScriptFn_IsRiderInState` (0x302ef8).
pub(crate) const IS_RIDER_IN_STATE: RiderQuery = RiderQuery {
    entry: 0x0030_2ef8,
    end: 0x0030_312c,
    flag_off: 0x26c0,
    byte_off: 0x26bc,
    src_off: 0x26b8,
    kinds_off: 0xc20,
    table: 0x0048_97b0,
    targets: IS_RIDER_IN_STATE_TARGETS,
    quirk: Some(0x0030_3030),
    copy_ret: 0x0030_2f4c,
    call_ret: 0x0030_30ec,
    tail_ret: 0x0030_3108,
    loop_top: 0x0030_2f60,
    entry_cycles: 3,
    init: rider_query_init_a,
};

/// The gp-relative and absolute layout of `WScriptFn_IsRiderInState_2` (0x3037b0).
pub(crate) const IS_RIDER_IN_STATE_2: RiderQuery = RiderQuery {
    entry: 0x0030_37b0,
    end: 0x0030_39ec,
    flag_off: 0x26e8,
    byte_off: 0x26e4,
    src_off: 0x26e0,
    kinds_off: 0xc40,
    table: 0x0048_9800,
    targets: IS_RIDER_IN_STATE_2_TARGETS,
    quirk: None,
    copy_ret: 0x0030_3808,
    call_ret: 0x0030_39ac,
    tail_ret: 0x0030_39c8,
    loop_top: 0x0030_3820,
    entry_cycles: 4,
    init: rider_query_init_b,
};

/// `bool WScriptFn_IsRiderInState(self, params, n)` (0x302ef8): stores into
/// the calling object (type 1) whether the rider at the manager's index is in
/// the state the case index `vp[0]` names, through `Rider_DispatchQueryTable`
/// (0x122ee8). The params overwrite the block; unknown indices give 0.
pub fn wscript_fn_is_rider_in_state(c: &mut Cpu) {
    guest(c, |c| rider_query_run(c, &IS_RIDER_IN_STATE));
}

/// `WScriptFn_IsRiderInState_2` (0x3037b0): the same switch as
/// `WScriptFn_IsRiderInState` with its own static block, kind table and
/// case table.
pub fn wscript_fn_is_rider_in_state_2(c: &mut Cpu) {
    guest(c, |c| rider_query_run(c, &IS_RIDER_IN_STATE_2));
}

/// `WScriptFn_GetRiderStateInfo` (0x3034f8, `CurrentRailFunc`): the spline id
/// of the rider's railslide motion when the rider's mission int 4 is set,
/// else -1, stored into the calling object (type 1) and returned through
/// `LunoVMRegister_AsInt` (0x226610).
pub fn wscript_fn_get_rider_state_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        let s = sp(c);
        c.sq(16, s.wrapping_add(0x20));
        c.g[A1][0] = 0x4;
        c.sq(17, s.wrapping_add(0x10));
        c.g[S0][0] = 0;
        let ra = c.g[RA][0];
        c.sd(s, ra);
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
        }
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c0));
        if c.g[V0][0] != 0 {
            c.g[S0][0] = c.g[V1][0];
        }
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0012_2ee8, 0x0030_354c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[S0][0] = u64::MAX;
        } else {
            c.cycles += 3;
            let handler = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
            c.g[V0][0] = handler;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd4));
        }
        // The tail: mark the object as type 1, store the spline id.
        c.cycles += 11;
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S1][0];
        let s1 = c.g[S1][0] as u32;
        c.sw(s1.wrapping_add(0x8), 1);
        call_ptr(c, 0x0022_6610, 0x0030_3578)?;
        let s0 = c.g[S0][0];
        c.sw(c.g[V0][0] as u32, s0);
        c.g[V0][0] = c.g[S1][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x20));
        c.lq(17, s.wrapping_add(0x10));
        c.g[RA][0] = c.ld(s);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        ret(c);
        Ok(())
    });
}

/// `WScriptFn_GetEventArg` (0x3036b0, `CurrentTrickFunc`): the current trick
/// of the manager's rider (`WScriptMan_GetEventArg`, 0x30af08) for the case
/// index `vp[0]`, stored into the calling object (type 1) and returned
/// through `LunoVMRegister_AsInt` (0x226610).
pub fn wscript_fn_get_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x26d8));
        let s = sp(c);
        c.sq(17, s.wrapping_add(0x50));
        c.sq(18, s.wrapping_add(0x40));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, s.wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, s.wrapping_add(0x60));
        c.g[S3][0] = c.g[A2][0];
        let t = c.g[V0][0] != 0;
        let ra = c.g[RA][0];
        c.sd(s.wrapping_add(0x20), ra);
        if !t {
            c.cycles += 6;
            c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x26d4));
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = 1;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x26d0), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x26d8), c.g[A0][0]);
            let s = sp(c);
            c.sb(s, c.g[V0][0]);
        }
        // The 8-byte block copy into vp, then the arg loop.
        c.cycles += 7;
        let vp = sx(sp(c).wrapping_add(0x10));
        c.g[S0][0] = vp;
        c.g[A2][0] = 8;
        c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x26d0));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6574, 0x0030_3708)?;
        let count = c.g[S1][0];
        let no_args = (count as i64) <= 0;
        c.g[A2][0] = c.g[S3][0];
        if !no_args {
            let kinds = (c.g[GP][0] as u32).wrapping_add(0xc38);
            rider_fill_args(c, vp, count, kinds, 4, 0x0030_3720);
        }
        // The trick lookup, then the int store.
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
        c.g[A1][0] = c.lw(c.g[A0][0] as u32);
        Callee {
            addr: 0x0030_af08,
            f: ee::s10::f_0030af08,
        }
        .call(c, 0x0030_3778)?;
        c.g[V1][0] = 1;
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0022_6610, 0x0030_378c)?;
        let s0 = c.g[S0][0];
        c.sw(c.g[V0][0] as u32, s0);
        c.g[V0][0] = c.g[S2][0];
        let s = sp(c);
        c.lq(16, s.wrapping_add(0x60));
        c.lq(17, s.wrapping_add(0x50));
        c.lq(18, s.wrapping_add(0x40));
        c.lq(19, s.wrapping_add(0x30));
        c.g[RA][0] = c.ld(s.wrapping_add(0x20));
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        ret(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fill, Rng, DATA, GAME_GP};

    /// The script object every fixture passes as `this`.
    const OC734_OBJ: u32 = DATA + 0x4000;
    /// The script manager (`gp+0xce8` holds its address).
    const OC734_MGR: u32 = DATA + 0x1400;
    /// The world (`gp-0x848` holds its address) and its rider table.
    const OC734_WORLD: u32 = DATA + 0x1000;
    const OC734_RIDERS: u32 = DATA + 0x1100;
    const OC734_BASE: u32 = DATA + 0x1200;
    /// The rider and the handler behind its `+0x77c` word.
    const OC734_RIDER: u32 = DATA + 0x1600;
    const OC734_HANDLER: u32 = DATA + 0x1a00;
    /// The argument list the loop reads (16-byte entries).
    const OC734_PARAMS: u32 = DATA + 0x2000;

    /// The rider chain the fixtures build: world -> rider table (+0x84) ->
    /// base (+0xc) -> rider (+0x28), rider index 0. Returns the rider (0 for
    /// the no-rider cases).
    fn oc734_world(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC734_WORLD);
        c.bus.write32(OC734_WORLD + 0x84, OC734_RIDERS);
        c.bus.write32(OC734_RIDERS + 0xc, OC734_BASE);
        c.bus.write32(GAME_GP + 0xce8, OC734_MGR);
        c.bus.write32(OC734_MGR, 0);
        let rider = if seed % 4 == 0 { 0 } else { OC734_RIDER };
        c.bus.write32(OC734_BASE + 0x28, rider);
        if rider != 0 {
            fill(c, r, rider, 0x800);
            c.bus.write32(rider + 0x77c, OC734_HANDLER);
            c.bus.write32(OC734_HANDLER + 0xd4, r.next_u32());
        }
        rider
    }

    /// The static block, its flag and byte, the kind table and an argument
    /// list of up to two entries (indices 0 and 1, random values and kinds).
    fn oc734_args(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        flag_off: u32,
        byte_off: u32,
        src_off: u32,
        kinds_off: u32,
    ) {
        let count = seed % 3;
        c.g[A1][0] = count as u64;
        c.g[A2][0] = sx(OC734_PARAMS);
        c.bus.write32(GAME_GP + src_off, (seed / 2) % 21);
        c.bus
            .write32(GAME_GP + flag_off, if seed % 2 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + byte_off, r.next_u32());
        for k in 0..2u32 {
            c.bus.write32(GAME_GP + kinds_off + 4 * k, r.next_u32() % 3);
        }
        for j in 0..count {
            let e = OC734_PARAMS + 16 * j;
            c.bus.write32(e, j % 2);
            c.bus.write32(e + 0x4, r.next_u32());
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
    }

    /// The fixture of the two rider-query switches: the case table of the
    /// given layout (index 4 may take the quirk arm), the static block and
    /// the arguments.
    fn oc734_query_setup(c: &mut Cpu, seed: u32, q: &RiderQuery) {
        let mut r = fakes::machine(c, seed, q.entry as u64, 0);
        c.pc = q.entry;
        c.g[A0][0] = sx(OC734_OBJ);
        oc734_world(c, &mut r, seed);
        oc734_args(
            c,
            &mut r,
            seed,
            q.flag_off,
            q.byte_off,
            q.src_off,
            q.kinds_off,
        );
        for (i, &target) in q.targets.iter().enumerate() {
            c.bus.write32(q.table + 4 * i as u32, target);
        }
        if let Some(quirk) = q.quirk {
            if seed % 5 == 0 {
                c.bus.write32(q.table + 16, quirk);
            }
        }
    }

    #[test]
    fn wscript_fn_is_rider_in_state_matches() {
        check(
            ee::s10::f_00302ef8__generated,
            wscript_fn_is_rider_in_state,
            300,
            &|c, seed| {
                oc734_query_setup(c, seed, &IS_RIDER_IN_STATE);
            },
        );
    }

    #[test]
    fn wscript_fn_is_rider_in_state_2_matches() {
        check(
            ee::s10::f_003037b0__generated,
            wscript_fn_is_rider_in_state_2,
            300,
            &|c, seed| {
                oc734_query_setup(c, seed, &IS_RIDER_IN_STATE_2);
            },
        );
    }

    #[test]
    fn wscript_fn_get_rider_state_info_matches() {
        check(
            ee::s10::f_003034f8__generated,
            wscript_fn_get_rider_state_info,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x34f8, 0);
                c.pc = 0x0030_34f8;
                c.g[A0][0] = sx(OC734_OBJ);
                oc734_world(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn wscript_fn_get_event_arg_matches() {
        check(
            ee::s10::f_003036b0__generated,
            wscript_fn_get_event_arg,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x36b0, 0);
                c.pc = 0x0030_36b0;
                c.g[A0][0] = sx(OC734_OBJ);
                oc734_world(c, &mut r, seed);
                oc734_args(c, &mut r, seed, 0x26d8, 0x26d4, 0x26d0, 0xc38);
            },
        );
    }
}
