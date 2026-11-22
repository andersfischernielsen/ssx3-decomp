//! Staging module oc442 (wave wave223): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc442`.

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
    0x0020_3d40,
    event_select_refresh_peak_list,
    "EventSelect_RefreshPeakList",
)];

// ---------------------------------------------------------------------------
// EventSelect_RefreshPeakList (`cUITemplate_MAP_setupPeakInfo`).

/// `self` (the map template, `s3`) field offsets: unlocked flags of the three
/// peaks (`+4 + 4 * peak`), popup flag, screen and the state owner.
pub(crate) const PK_UNLOCKED: u32 = 0x4;
pub(crate) const PK_POPUP: u32 = 0x2cc;
pub(crate) const PK_SCREEN: u32 = 0x2e8;
pub(crate) const PK_OWNER: u32 = 0x2ec;
/// Peak selection object: the selected peak at `+0x18`; the list object keeps
/// it at `+0xa0`.
pub(crate) const PK_SEL_PEAK: u32 = 0x18;
pub(crate) const PK_LIST_SEL: u32 = 0xa0;

/// Stack frame of the function (`sp` relative).
pub(crate) const PK_FRAME: u32 = 0x3b0;
/// `name1`/`name2` string buffers (`sp+0`, `sp+0x20`), the wide/ascii text
/// buffer (`sp+0x40`), the copied goal records (`sp+0x1d0`, `sp+0x240`; 0x6c
/// bytes each: word 0 is the track, word 2 the goal kind).
pub(crate) const PK_NAME2: u32 = 0x20;
pub(crate) const PK_BUF: u32 = 0x40;
pub(crate) const PK_GOAL_A: u32 = 0x1d0;
pub(crate) const PK_GOAL_B: u32 = 0x240;
/// Spill slots: the mission, economy and score interfaces, goal count, done
/// count, the next peak, the goal-B pointer and `t1`.
pub(crate) const PK_MIS: u32 = 0x2b0;
pub(crate) const PK_ECON: u32 = 0x2b4;
pub(crate) const PK_SCORE: u32 = 0x2b8;
pub(crate) const PK_CNT: u32 = 0x2bc;
pub(crate) const PK_DONE: u32 = 0x2c0;
pub(crate) const PK_NEXT: u32 = 0x2c4;
pub(crate) const PK_GOAL_B_PTR: u32 = 0x2c8;
pub(crate) const PK_SPILL_T1: u32 = 0x2d0;

const PK_HIDE: u32 = 0x0039_e8b8;
const PK_SHOW: u32 = 0x0039_e948;
const PK_HASH: u32 = 0x0031_7670;
const PK_FIND: u32 = 0x0039_d860;
const PK_SET_TEXT_ID: u32 = 0x003a_1000;
const PK_SET_STRING: u32 = 0x003a_0c50;
const PK_SET_WIDE: u32 = 0x003a_0e90;
const PK_SPRINTF: u32 = 0x0041_7828;
const PK_SPRINTF1: u32 = 0x002c_26d0;
const PK_TRACK_MEDAL: u32 = 0x0015_8910;
const PK_TROPHY: u32 = 0x0015_7bf0;

/// `s3` (self); callee-saved, read from the register each time.
#[inline(always)]
fn pk_this(c: &Cpu) -> u32 {
    c.g[S3][0] as u32
}

/// `a0 = self->owner`.
#[inline(always)]
fn pk_owner_a0(c: &mut Cpu) {
    let a = pk_this(c).wrapping_add(PK_OWNER);
    c.g[A0][0] = c.lw(a);
}

/// `a0 = self->screen`.
#[inline(always)]
fn pk_screen_a0(c: &mut Cpu) {
    let a = pk_this(c).wrapping_add(PK_SCREEN);
    c.g[A0][0] = c.lw(a);
}

#[inline(always)]
fn pk_lw_sp(c: &mut Cpu, off: u32) -> u64 {
    let a = sp(c).wrapping_add(off);
    c.lw(a)
}

#[inline(always)]
fn pk_sw_sp(c: &mut Cpu, off: u32, v: u64) {
    let a = sp(c).wrapping_add(off);
    c.sw(a, v)
}

/// `sw reg, off(sp)`.
#[inline(always)]
fn pk_sw_sp_reg(c: &mut Cpu, off: u32, reg: usize) {
    let a = sp(c).wrapping_add(off);
    let v = c.g[reg][0];
    c.sw(a, v)
}

/// `lw (gp - off)`.
#[inline(always)]
fn pk_lw_gp(c: &mut Cpu, off: u32) -> u64 {
    let a = (c.g[GP][0] as u32).wrapping_sub(off);
    c.lw(a)
}

/// `a0 = sp + off`-style address register value.
#[inline(always)]
fn pk_sp_addr(c: &Cpu, off: u32) -> u64 {
    sx(sp(c).wrapping_add(off))
}

/// Call of a guest function through the function table.
#[inline(always)]
fn pk_call(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    call_ptr(c, target, ret)
}

/// `obj->vtable[slot](obj, ...)`: `v0 = *(obj+8)`, `a0 = lh(v0+slot)`,
/// `v1 = lw(v0+slot+4)`, `a0 = obj + a0`, call `v1` (a1 is set by the caller).
fn pk_vcall(c: &mut Cpu, obj: usize, slot: u32, ret: u32) -> Flow {
    let o = c.g[obj][0] as u32;
    c.g[V0][0] = c.lw(o.wrapping_add(8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(slot));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(slot + 4));
    let t = c.g[V1][0] as u32;
    c.g[A0][0] = sx(o.wrapping_add(c.g[A0][0] as u32));
    pk_call(c, t, ret)
}

/// The inlined copy of one 0x6c-byte goal record from `v1` to `a0`: 0x60 bytes
/// in 32-byte strides (`v0` is the end address of the strides), either with
/// aligned doublewords (`ld`/`sd`, 11 cycles a stride) or unaligned (`ldl`/
/// `ldr`, 19 cycles a stride and 2 more cycles at the exit). `regs` are the
/// four registers carrying the doublewords. The last 12 bytes follow in the
/// caller ([`pk_goal_tail`]).
fn pk_goal_strides(c: &mut Cpu, aligned: bool, regs: [usize; 4], head: u32) {
    loop {
        let src = c.g[V1][0] as u32;
        let dst = c.g[A0][0] as u32;
        if aligned {
            c.cycles += 11;
            for (i, r) in regs.iter().enumerate() {
                c.g[*r][0] = c.ld(src.wrapping_add(8 * i as u32));
            }
            for (i, r) in regs.iter().enumerate() {
                let v = c.g[*r][0];
                c.sd(dst.wrapping_add(8 * i as u32), v);
            }
        } else {
            c.cycles += 19;
            for (i, r) in regs.iter().enumerate() {
                let o = 8 * i as u32;
                c.ldl(*r, src.wrapping_add(o + 7));
                c.ldr(*r, src.wrapping_add(o));
            }
            for (i, r) in regs.iter().enumerate() {
                let o = 8 * i as u32;
                c.sdl(*r, dst.wrapping_add(o + 7));
                c.sdr(*r, dst.wrapping_add(o));
            }
        }
        c.g[V1][0] = sx(src.wrapping_add(0x20));
        let more = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx(dst.wrapping_add(0x20));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
    if !aligned {
        c.cycles += 2;
    }
}

/// The last 12 bytes of a goal copy: a doubleword through `ldl`/`ldr` in
/// `r_dw`, then a word into `r_w` (stored with `sw`).
fn pk_goal_tail(c: &mut Cpu, r_dw: usize, r_w: usize) {
    let src = c.g[V1][0] as u32;
    c.ldl(r_dw, src.wrapping_add(7));
    c.ldr(r_dw, src);
    c.g[r_w][0] = c.lw(src.wrapping_add(8));
    let dst = c.g[A0][0] as u32;
    c.sdl(r_dw, dst.wrapping_add(7));
    c.sdr(r_dw, dst);
    let w = c.g[r_w][0];
    c.sw(dst.wrapping_add(8), w);
}

#[cfg(not(feature = "ee-unwind"))]
fn pk_resume(c: &mut Cpu) {
    ee::s05::f_00203d40__entries(c)
}

#[cfg(feature = "ee-unwind")]
fn pk_resume(c: &mut Cpu) {
    ee::s05::f_00203d40__generated(c)
}

/// Epilogue: reload the callee-saved registers (`s0` first, as the early
/// exits do), `ra` and the float registers, and return.
fn pk_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 17;
    for (r, off) in [
        (S1, 0x360),
        (S2, 0x350),
        (S3, 0x340),
        (S4, 0x330),
        (S5, 0x320),
        (S6, 0x310),
        (S7, 0x300),
        (FP, 0x2f0),
    ] {
        frame.restore(c, r, off);
    }
    frame.restore_ra(c, 0x2e0);
    for (f, off) in [
        (25, 0x3a8),
        (24, 0x3a0),
        (23, 0x398),
        (22, 0x390),
        (21, 0x388),
        (20, 0x380),
    ] {
        frame.restore_fpr(c, f, off);
    }
    frame.close(c);
}

/// `lq s0` (the early exits reload `s0` in a branch delay slot) and leave.
fn pk_leave(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x370);
    pk_epilogue(c, frame);
}

/// `void EventSelect_RefreshPeakList(MapTemplate* self)` (0x203d40;
/// `cUITemplate_MAP_setupPeakInfo`): refresh the peak panel of the map screen.
/// Shows/hides the popup frame, the three trophy marks and the per-peak
/// "unlocked" markers (by map style and state), then looks at the selected
/// peak. For peaks 0..2 it sets the help text and, on map style >= 2, the goal
/// counters (slopestyle/race medal goals found, challenges and collectibles,
/// earnings) with their check marks. For the overall entry (peak >= 3) it
/// counts goals, gold medals, missions, collectibles and highlights and shows
/// the completion percentage.
///
/// Everything lives in registers across the calls (the generated body is
/// resumable at its call returns), so the loop counters stay in `s0`..`s7`.
pub fn event_select_refresh_peak_list(c: &mut Cpu) {
    if c.pc != 0x0020_3d40 {
        return pk_resume(c);
    }
    guest(c, pk_refresh_body)
}

fn pk_refresh_body(c: &mut Cpu) -> Flow {
    c.cycles += 44;
    let frame = Frame::open(c, PK_FRAME);
    frame.save(c, S3, 0x340);
    frame.save(c, S5, 0x320);
    c.g[S3][0] = c.g[A0][0];
    frame.save(c, S7, 0x300);
    frame.save(c, S0, 0x370);
    frame.save(c, S1, 0x360);
    frame.save(c, S2, 0x350);
    frame.save(c, S4, 0x330);
    frame.save(c, S6, 0x310);
    frame.save(c, FP, 0x2f0);
    frame.save_ra(c, 0x2e0);
    for (f, off) in [
        (25, 0x3a8),
        (24, 0x3a0),
        (23, 0x398),
        (22, 0x390),
        (21, 0x388),
        (20, 0x380),
    ] {
        frame.save_fpr(c, f, off);
    }

    // The back-end interfaces: reward (s5), mission, economy, score; and the
    // rider character (s7).
    pk_call(c, 0x0014_dc80, 0x0020_3d8c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    pk_call(c, 0x0014_dd58, 0x0020_3d98)?;
    c.g[S5][0] = c.g[V0][0];
    pk_call(c, 0x0014_dc80, 0x0020_3da0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xa;
    pk_call(c, 0x0014_dd58, 0x0020_3dac)?;
    pk_sw_sp_reg(c, PK_MIS, V0);
    pk_call(c, 0x0014_dc80, 0x0020_3db4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xb;
    pk_call(c, 0x0014_dd58, 0x0020_3dc0)?;
    pk_sw_sp_reg(c, PK_ECON, V0);
    pk_call(c, 0x0014_dc80, 0x0020_3dc8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0x8;
    pk_call(c, 0x0014_dd58, 0x0020_3dd4)?;
    pk_sw_sp_reg(c, PK_SCORE, V0);
    c.g[A0][0] = 0;
    pk_call(c, 0x0014_a080, 0x0020_3de0)?;
    c.g[S7][0] = c.g[V0][0];

    // Popup frame: hidden while the popup flag is set, else shown.
    let popup = pk_this(c).wrapping_add(PK_POPUP);
    c.g[V0][0] = c.lw(popup);
    let show_frame = c.g[V0][0] == 0;
    pk_owner_a0(c);
    if !show_frame {
        c.cycles += 13;
        c.g[A1][0] = 0x0047_0ae8;
        pk_call(c, PK_HIDE, 0x0020_3dfc)?;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2648;
        pk_call(c, PK_HIDE, 0x0020_3e0c)?;
        c.g[A1][0] = 0x004a_0000;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2610;
        pk_call(c, PK_HIDE, 0x0020_3e1c)?;
        pk_owner_a0(c);
    } else {
        c.cycles += 12;
        c.g[A1][0] = 0x0047_0ae8;
        pk_call(c, PK_SHOW, 0x0020_3e30)?;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2648;
        pk_call(c, PK_SHOW, 0x0020_3e40)?;
        c.g[A1][0] = 0x004a_0000;
        pk_owner_a0(c);
        c.g[A1][0] = 0x004a_2610;
        pk_call(c, PK_SHOW, 0x0020_3e50)?;
        pk_owner_a0(c);
    }

    c.cycles += 17;
    c.g[A1][0] = 0x0047_0af8;
    c.g[S0][0] = 0;
    pk_call(c, PK_HIDE, 0x0020_3e64)?;
    c.g[S2][0] = 0x0047_0000;
    pk_owner_a0(c);
    c.g[A1][0] = 0x0047_0b08;
    c.g[FP][0] = 0x0047_0000;
    pk_call(c, PK_HIDE, 0x0020_3e7c)?;
    c.g[A1][0] = 0x004a_0000;
    pk_owner_a0(c);
    c.g[A1][0] = 0x004a_2650;
    pk_call(c, PK_HIDE, 0x0020_3e8c)?;
    c.g[S4][0] = pk_sp_addr(c, PK_NAME2);
    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));

    // Trophy marks 1..3: shown when all four trophies of the peak are owned.
    loop {
        c.cycles += 12;
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb20));
        pk_call(c, PK_SPRINTF, 0x0020_3ea8)?;
        // `isTrophyOwned(rew, 0, ch, peak, i)` for i = 0..3. `None`: one of
        // the first three failed, `Some(false)`: the fourth failed.
        let mut owned: Option<bool> = None;
        'trophies: {
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 0;
            pk_call(c, PK_TROPHY, 0x0020_3ec0)?;
            let ok = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S5][0];
            if !ok {
                break 'trophies;
            }
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 1;
            pk_call(c, PK_TROPHY, 0x0020_3edc)?;
            let ok = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S5][0];
            if !ok {
                break 'trophies;
            }
            c.cycles += 7;
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[T0][0] = 2;
            pk_call(c, PK_TROPHY, 0x0020_3ef8)?;
            let ok = c.g[V0][0] != 0;
            c.g[A3][0] = c.g[S0][0];
            if !ok {
                break 'trophies;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S7][0];
            c.g[T0][0] = 3;
            pk_call(c, PK_TROPHY, 0x0020_3f14)?;
            let ok = c.g[V0][0] != 0;
            pk_owner_a0(c);
            owned = Some(ok);
        }
        match owned {
            Some(true) => {
                c.cycles += 4;
                c.g[A1][0] = c.g[SP][0];
                pk_call(c, PK_SHOW, 0x0020_3f24)?;
                c.g[S0][0] = c.g[S1][0];
            }
            other => {
                if other.is_none() {
                    c.cycles += 1;
                    pk_owner_a0(c);
                }
                c.cycles += 3;
                c.g[A1][0] = c.g[SP][0];
                pk_call(c, PK_HIDE, 0x0020_3f38)?;
                c.g[S0][0] = c.g[S1][0];
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_3e98);
        }
    }

    // Map style < 2: the "Mountain" object is made static and invisible.
    c.cycles += 4;
    c.g[V0][0] = pk_lw_gp(c, 0xb5c);
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let low_style = c.g[V0][0] != 0;
    c.g[A0][0] = 0x0047_0000;
    if low_style {
        c.cycles += 18;
        c.g[A0][0] = 0x0047_0610;
        pk_call(c, PK_HASH, 0x0020_3f60)?;
        pk_screen_a0(c);
        c.g[A1][0] = c.g[V0][0];
        pk_call(c, PK_FIND, 0x0020_3f6c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        pk_vcall(c, S0, 0x40, 0x0020_3f88)?;
        c.g[A1][0] = 0;
        pk_vcall(c, S0, 0x48, 0x0020_3fa0)?;
    }

    c.cycles += 16;
    pk_owner_a0(c);
    c.g[A1][0] = 0x004a_25d8;
    c.g[S1][0] = 0;
    pk_call(c, PK_HIDE, 0x0020_3fb4)?;
    c.g[S6][0] = sx(pk_this(c).wrapping_add(PK_UNLOCKED));
    pk_owner_a0(c);
    c.g[A1][0] = 0x0047_0548;
    pk_call(c, PK_HIDE, 0x0020_3fc8)?;
    c.g[A1][0] = 0x0047_0000;
    pk_owner_a0(c);
    c.g[A1][0] = 0x0047_0558;
    pk_call(c, PK_HIDE, 0x0020_3fd8)?;
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));

    // The per-peak markers "PEAKn" (name1) and "...n" (name2).
    loop {
        c.cycles += 13;
        c.g[V0][0] = 0x004a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2658));
        c.g[A2][0] = c.g[S0][0];
        c.g[S2][0] = c.g[S0][0];
        pk_call(c, PK_SPRINTF, 0x0020_3ff8)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0xb38));
        c.g[A2][0] = c.g[S0][0];
        pk_call(c, PK_SPRINTF, 0x0020_4008)?;
        c.g[V1][0] = pk_lw_gp(c, 0xb54);
        let state_set = c.g[V1][0] != 0;
        pk_owner_a0(c);
        let mut hide_name2_only = false;
        if !state_set {
            c.cycles += 5;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = c.g[S4][0];
                pk_call(c, PK_HIDE, 0x0020_4030)?;
                c.g[S1][0] = c.g[S2][0];
                hide_name2_only = true;
            }
        }
        if !hide_name2_only {
            c.cycles += 6;
            c.g[A1][0] = c.g[SP][0];
            pk_call(c, PK_HIDE, 0x0020_4040)?;
            c.g[V0][0] = pk_lw_gp(c, 0xb5c);
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let high_style = c.g[V0][0] == 0;
            c.g[S1][0] = c.g[S2][0];
            if !high_style {
                c.cycles += 4;
                pk_owner_a0(c);
                c.g[A1][0] = c.g[S4][0];
                pk_call(c, PK_HIDE, 0x0020_405c)?;
                c.g[S1][0] = c.g[S2][0];
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S1][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_3fe0);
        }
    }

    // Help text object hidden; then the peak list and its selection.
    c.cycles += 21;
    c.g[A0][0] = 0x0047_0578;
    pk_call(c, PK_HASH, 0x0020_4078)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4084)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    pk_vcall(c, S1, 0x48, 0x0020_40a0)?;
    c.g[A0][0] = 0x0047_0600;
    pk_call(c, PK_HASH, 0x0020_40ac)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_40b8)?;
    let no_list = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x370);
    if no_list {
        pk_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 3;
    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(PK_LIST_SEL));
    if c.g[S0][0] == 0 {
        pk_leave(c, frame);
        return Ok(());
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PK_SEL_PEAK));
    c.g[V0][0] = ((c.g[V0][0] as i64) < 3) as u64;
    let single_peak = c.g[V0][0] != 0;
    pk_owner_a0(c);
    if single_peak {
        pk_peak_info(c, frame)
    } else {
        pk_total_info(c, frame)
    }
}

/// The panel of one peak (0..2): help text, then (map style >= 2) the goal
/// counters of that peak.
fn pk_peak_info(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 15;
    c.g[A1][0] = 0x0047_0568;
    pk_call(c, PK_HIDE, 0x0020_40e8)?;
    c.g[A1][0] = 1;
    c.g[FP][0] = c.lw((c.g[S0][0] as u32).wrapping_add(PK_SEL_PEAK));
    pk_vcall(c, S1, 0x48, 0x0020_4104)?;
    c.g[V0][0] = sx((c.g[FP][0] as u32) << 2);
    c.g[V0][0] = sx(pk_this(c).wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(PK_UNLOCKED));
    let locked = c.g[V1][0] == 0;
    c.g[V0][0] = pk_lw_gp(c, 0xb5c);
    // Which help string, as the original's block ladder: `Some(string)` with
    // the cycles of the string choice, then `SetTextId(title, hash(string))`
    // charging `set_cycles`.
    let set_cycles;
    if !locked {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = 0x0047_0000;
            c.cycles += 2;
            c.g[A0][0] = 0x0047_0b68;
        } else {
            c.cycles += 3;
            c.g[A0][0] = 0x0047_0b50;
        }
        set_cycles = 7;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let not_one = c.g[FP][0] != c.g[V0][0];
        c.g[V0][0] = pk_lw_gp(c, 0xb5c);
        if !not_one {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = 0x0047_0000;
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0b98;
            } else {
                c.cycles += 3;
                c.g[A0][0] = 0x0047_0b80;
            }
            set_cycles = 7;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            c.g[A0][0] = 0x0047_0000;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = 0x0047_0bc8;
                set_cycles = 6;
            } else {
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0bb0;
                set_cycles = 7;
            }
        }
    }
    c.cycles += set_cycles;
    pk_call(c, PK_HASH, if set_cycles == 7 { 0x0020_4180 } else { 0x0020_419c })?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, if set_cycles == 7 { 0x0020_418c } else { 0x0020_41a8 })?;
    c.g[V0][0] = pk_lw_gp(c, 0xb5c);

    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    if c.g[V0][0] != 0 {
        pk_leave(c, frame);
        return Ok(());
    }

    // Map style >= 2: goal counters of the peak.
    c.cycles += 41;
    c.g[V1][0] = 4;
    c.g[A0][0] = 0x0047_0be0;
    pk_sw_sp_reg(c, PK_CNT, V1);
    pk_call(c, PK_HASH, 0x0020_41cc)?;
    c.g[S4][0] = 0;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_41dc)?;
    c.g[V1][0] = pk_lw_gp(c, 0x848);
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = 0x0047_0000;
    c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbf8));
    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    pk_call(c, PK_HASH, 0x0020_4200)?;
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let t = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, t, 0x0020_4210)?;
    c.g[S0][0] = pk_sp_addr(c, PK_BUF);
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(1));
    c.g[A0][0] = c.g[S0][0];
    pk_call(c, PK_SPRINTF1, 0x0020_4224)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    pk_call(c, PK_SET_WIDE, 0x0020_4230)?;
    c.g[A1][0] = pk_lw_sp(c, PK_CNT);
    c.g[V1][0] = 3;
    c.g[V0][0] = c.g[FP][0] ^ 2;
    c.g[A0][0] = pk_sp_addr(c, PK_GOAL_B);
    if c.g[V0][0] == 0 {
        c.g[A1][0] = c.g[V1][0];
    }
    c.g[S6][0] = pk_sp_addr(c, PK_GOAL_A);
    pk_sw_sp_reg(c, PK_CNT, A1);
    pk_sw_sp_reg(c, PK_GOAL_B_PTR, A0);
    c.g[A2][0] = pk_lw_sp(c, PK_CNT);
    let no_goals = c.g[A2][0] == 0;
    c.g[A1][0] = 0;
    if !no_goals {
        // Goal records of the first table (0x4781d0, 0x1b0 a peak, 0x6c a
        // goal): count the medals found.
        c.cycles += 3;
        c.g[V0][0] = 0x1b0;
        c.mult(0, S2, FP, V0);
        c.g[V0][0] = 0x6c;
        loop {
            c.cycles += 8;
            c.g[A2][0] = 0x0048_0000;
            c.mult(0, V0, A1, V0);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7e30));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S6][0];
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
            if aligned {
                pk_goal_strides(c, true, [A3, T0, A1, A2], 0x0020_42f4);
            } else {
                pk_goal_strides(c, false, [A3, T0, A1, A2], 0x0020_4294);
            }
            c.cycles += 19;
            pk_goal_tail(c, A3, T0);
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = pk_lw_sp(c, PK_GOAL_A + 8);
            c.g[A1][0] = 0;
            c.g[T0][0] = pk_lw_sp(c, PK_GOAL_A);
            c.g[A0][0] = c.g[S5][0];
            pk_call(c, PK_TRACK_MEDAL, 0x0020_4350)?;
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            c.g[A2][0] = pk_lw_sp(c, PK_CNT);
            c.g[A1][0] = c.g[S1][0];
            if c.g[V0][0] != 0 {
                c.g[S4][0] = c.g[S0][0];
            }
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x6c;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_4268);
            }
        }
    }

    // The medal-goal count for this peak: shown with its "x of n" text.
    c.cycles += 20;
    c.g[A0][0] = 0x0047_0c10;
    pk_call(c, PK_HASH, 0x0020_4378)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4384)?;
    c.g[A3][0] = 0x004a_0000;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2398));
    c.g[A0][0] = c.g[S6][0];
    c.g[A3][0] = pk_lw_sp(c, PK_CNT);
    c.g[A2][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_43a0)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_43ac)?;
    c.g[T0][0] = pk_lw_sp(c, PK_CNT);
    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
    let short = c.g[V0][0] != 0;
    c.g[V1][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc20));
        pk_call(c, PK_HIDE, 0x0020_43e0)?;
    } else {
        c.cycles += 6;
        c.g[V0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc20));
        pk_call(c, PK_SHOW, 0x0020_43cc)?;
    }
    c.g[V0][0] = 0x21c;

    // Goal records of the second table (0x4786e0, 0x21c a peak): count.
    c.cycles += 5;
    c.g[S4][0] = 0;
    c.mult(0, S2, FP, V0);
    c.g[A1][0] = 0;
    c.g[V0][0] = 0x6c;
    loop {
        c.cycles += 8;
        c.g[A2][0] = 0x0048_0000;
        c.mult(0, V0, A1, V0);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7920));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = c.g[V1][0] & 7;
        let aligned = c.g[V0][0] == 0;
        c.g[A0][0] = pk_lw_sp(c, PK_GOAL_B_PTR);
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
        if aligned {
            pk_goal_strides(c, true, [A3, T0, A1, A2], 0x0020_4484);
        } else {
            pk_goal_strides(c, false, [A3, T0, A1, A2], 0x0020_4424);
        }
        c.cycles += 18;
        pk_goal_tail(c, A3, T0);
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = pk_lw_sp(c, PK_GOAL_B + 8);
        c.g[A1][0] = 0;
        c.g[T0][0] = pk_lw_sp(c, PK_GOAL_B);
        c.g[A0][0] = c.g[S5][0];
        pk_call(c, PK_TRACK_MEDAL, 0x0020_44e0)?;
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        if c.g[V0][0] != 0 {
            c.g[S4][0] = c.g[S0][0];
        }
        c.g[V0][0] = ((c.g[A1][0] as i64) < 5) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V0][0] = 0x6c;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_43f8);
        }
    }

    c.cycles += 18;
    c.g[A0][0] = 0x0047_0c30;
    pk_call(c, PK_HASH, 0x0020_4504)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4510)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2660));
    c.g[A0][0] = c.g[S6][0];
    c.g[A2][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_4528)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_4534)?;
    c.g[V0][0] = ((c.g[S4][0] as i64) < 5) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc48));
        pk_call(c, PK_HIDE, 0x0020_4564)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc48));
        pk_call(c, PK_SHOW, 0x0020_4550)?;
    }
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);

    // Challenges and collectibles: two medals at most.
    c.cycles += 39;
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, 0x0015_43e0, 0x0020_4570)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = c.g[FP][0];
    c.g[A2][0] = 3;
    pk_call(c, 0x0015_8890, 0x0020_4584)?;
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);
    c.g[S1][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[S1][0] ^= 1;
    c.g[A1][0] = 0;
    c.g[A2][0] = c.g[FP][0];
    pk_call(c, 0x0015_35c0, 0x0020_459c)?;
    c.g[A2][0] = 3;
    c.g[A1][0] = c.g[FP][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[S0][0] = c.g[V0][0];
    pk_call(c, 0x0015_8870, 0x0020_45b0)?;
    c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.g[A0][0] = 0x0047_0000;
    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc60));
    if c.g[S0][0] == 0 {
        c.g[S1][0] = c.g[V0][0];
    }
    pk_call(c, PK_HASH, 0x0020_45c8)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_45d4)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[A2][0] = c.g[S1][0];
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2668));
    c.g[A0][0] = c.g[S6][0];
    pk_call(c, PK_SPRINTF, 0x0020_45ec)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_45f8)?;
    c.g[S1][0] = ((c.g[S1][0] as i64) < 2) as u64;
    let few = c.g[S1][0] != 0;
    c.g[V0][0] = 0x0047_0000;
    if few {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc70));
        pk_call(c, PK_HIDE, 0x0020_4628)?;
    } else {
        c.cycles += 6;
        c.g[T0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc70));
        pk_call(c, PK_SHOW, 0x0020_4614)?;
    }
    c.g[A0][0] = pk_lw_sp(c, PK_ECON);

    // Earnings: total money earned against the peak's target.
    c.cycles += 24;
    c.g[A1][0] = 0;
    pk_call(c, 0x0015_09f8, 0x0020_4634)?;
    c.g[A1][0] = c.g[FP][0];
    c.g[A0][0] = c.g[S5][0];
    c.g[S0][0] = c.g[V0][0];
    pk_call(c, 0x0015_88b0, 0x0020_4644)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc88));
    pk_call(c, PK_HASH, 0x0020_4654)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4660)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2670));
    c.g[A0][0] = c.g[S6][0];
    c.g[A2][0] = c.g[S1][0];
    pk_call(c, PK_SPRINTF, 0x0020_4678)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    pk_call(c, PK_SET_STRING, 0x0020_4684)?;
    let not_met = (c.g[S1][0] as i64) <= 0;
    c.g[V1][0] = 0x0047_0000;
    if not_met {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xc98));
        pk_call(c, PK_HIDE, 0x0020_46b0)?;
    } else {
        c.cycles += 5;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc98));
        pk_call(c, PK_SHOW, 0x0020_4698)?;
    }
    pk_leave(c, frame);
    Ok(())
}

/// The overall entry of the peak list (peak >= 3): the game-wide completion
/// panel. Counts the goals done over all peaks, the gold medals, the missions
/// and collectibles and the highlight levels, shows each with its check mark
/// and ends with the completion percentage.
fn pk_total_info(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 24;
    c.g[A1][0] = 0x0047_0568;
    pk_sw_sp(c, PK_DONE, 0);
    pk_call(c, PK_SHOW, 0x0020_46c8)?;
    c.g[S6][0] = 0;
    pk_owner_a0(c);
    c.g[A1][0] = 0x004a_2608;
    pk_call(c, PK_HIDE, 0x0020_46dc)?;
    c.g[A0][0] = 0x0047_0cb0;
    pk_call(c, PK_HASH, 0x0020_46e8)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_46f4)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xcc8));
    pk_call(c, PK_HASH, 0x0020_4704)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, 0x0020_4710)?;
    c.g[FP][0] = pk_sp_addr(c, PK_BUF);
    c.g[S1][0] = 4;

    // Goals done over the three peaks (s6): table A (4 goals, 3 on the last
    // peak) and table B (5 goals), skipping the medal-only kinds.
    loop {
        c.cycles += 7;
        c.g[V1][0] = 3;
        c.g[V0][0] = c.g[S6][0] ^ 2;
        c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(1));
        c.g[A1][0] = 0;
        if c.g[V0][0] == 0 {
            c.g[S1][0] = c.g[V1][0];
        }
        let skip = c.g[S1][0] == 0;
        pk_sw_sp_reg(c, PK_NEXT, A3);
        if !skip {
            c.cycles += 7;
            c.g[V0][0] = 0x1b0;
            c.g[V1][0] = 0x0048_0000;
            c.mult(0, S4, S6, V0);
            c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
            c.g[S2][0] = 1;
            c.g[V0][0] = 0x6c;
            loop {
                c.cycles += 6;
                c.mult(0, V0, A1, V0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T1][0] as u32));
                c.g[V0][0] = c.g[V1][0] & 7;
                let aligned = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[FP][0];
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                if aligned {
                    pk_goal_strides(c, true, [A1, A2, A3, T0], 0x0020_47cc);
                } else {
                    pk_goal_strides(c, false, [A1, A2, A3, T0], 0x0020_4770);
                }
                c.cycles += 10;
                pk_goal_tail(c, V0, A1);
                c.g[V1][0] = pk_lw_sp(c, PK_BUF + 8);
                c.g[V0][0] = c.g[V1][0] ^ 4;
                let t = c.g[V0][0] == 0;
                c.g[A1][0] = 0;
                if !t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V1][0] ^ 6;
                    let t = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[V1][0];
                    if !t {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[A0][0] ^ 7;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[A0][0] ^ 8;
                        if !t {
                            c.cycles += 1;
                            if c.g[V0][0] != 0 {
                                c.g[A1][0] = c.g[S2][0];
                            }
                        }
                    }
                }
                c.cycles += 2;
                let skip_goal = c.g[A1][0] == 0;
                c.g[T0][0] = pk_lw_sp(c, PK_BUF);
                if !skip_goal {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A3][0] = pk_lw_sp(c, PK_BUF + 8);
                    c.g[A1][0] = 0;
                    frame.save(c, T1, PK_SPILL_T1);
                    c.g[A2][0] = c.g[S7][0];
                    pk_call(c, PK_TRACK_MEDAL, 0x0020_485c)?;
                    c.g[T0][0] = pk_lw_sp(c, PK_DONE);
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    frame.restore(c, T1, PK_SPILL_T1);
                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                    if c.g[V0][0] == 0 {
                        c.g[T0][0] = c.g[V1][0];
                    }
                    pk_sw_sp_reg(c, PK_DONE, T0);
                }
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = 0x6c;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_4750);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = 0x21c;
        c.g[V1][0] = 0x0048_0000;
        c.mult(0, S2, S6, V0);
        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7920));
        c.g[A1][0] = 0;
        c.g[S1][0] = 1;
        c.g[V0][0] = 0x6c;
        loop {
            c.cycles += 6;
            c.mult(0, V0, A1, V0);
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
            c.g[V0][0] = c.g[V1][0] & 7;
            let aligned = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[FP][0];
            c.cycles += 2;
            c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            if aligned {
                pk_goal_strides(c, true, [A1, A2, A3, T0], 0x0020_491c);
            } else {
                pk_goal_strides(c, false, [A1, A2, A3, T0], 0x0020_48c0);
            }
            c.cycles += 10;
            pk_goal_tail(c, V0, A1);
            c.g[V1][0] = pk_lw_sp(c, PK_BUF + 8);
            c.g[V0][0] = c.g[V1][0] ^ 5;
            let t = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] ^ 9;
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[V1][0];
                if !t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] ^ 0xa;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = c.g[A0][0] ^ 0xb;
                    if !t {
                        c.cycles += 1;
                        if c.g[V0][0] != 0 {
                            c.g[A1][0] = c.g[S1][0];
                        }
                    }
                }
            }
            c.cycles += 2;
            let skip_goal = c.g[A1][0] == 0;
            c.g[T0][0] = pk_lw_sp(c, PK_BUF);
            if !skip_goal {
                c.cycles += 10;
                c.g[A0][0] = c.g[S5][0];
                c.g[A3][0] = pk_lw_sp(c, PK_BUF + 8);
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S7][0];
                pk_call(c, PK_TRACK_MEDAL, 0x0020_49a8)?;
                c.g[T0][0] = pk_lw_sp(c, PK_DONE);
                c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                if c.g[V0][0] == 0 {
                    c.g[T0][0] = c.g[V1][0];
                }
                pk_sw_sp_reg(c, PK_DONE, T0);
            }
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = ((c.g[A1][0] as i64) < 5) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x6c;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_48a0);
            }
        }
        c.cycles += 4;
        c.g[S6][0] = pk_lw_sp(c, PK_NEXT);
        c.g[V0][0] = ((c.g[S6][0] as i64) < 3) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = 4;
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0020_4718);
        }
    }

    // "done / 14" and its check mark.
    c.cycles += 19;
    c.g[A0][0] = 0x0047_0cd8;
    pk_call(c, PK_HASH, 0x0020_49e8)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_49f4)?;
    c.g[A2][0] = pk_lw_sp(c, PK_DONE);
    c.g[A1][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2678));
    c.g[A0][0] = c.g[FP][0];
    pk_call(c, PK_SPRINTF, 0x0020_4a0c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4a18)?;
    c.g[V1][0] = pk_lw_sp(c, PK_DONE);
    c.g[V0][0] = ((c.g[V1][0] as i64) < 14) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xce8));
        pk_call(c, PK_HIDE, 0x0020_4a4c)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xce8));
        pk_call(c, PK_SHOW, 0x0020_4a38)?;
    }
    c.g[A0][0] = 0x0047_0000;

    // Gold medals: twelve track medals (kind, track), "gold / 12".
    c.cycles += 124;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd00));
    pk_call(c, PK_HASH, 0x0020_4a58)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4a64)?;
    c.g[S1][0] = c.g[V0][0];
    const GOLD: [(u64, u64, u32); 12] = [
        (4, 0xe, 0x0020_4a80),
        (6, 0xe, 0x0020_4aa0),
        (5, 0xe, 0x0020_4ac4),
        (9, 0xe, 0x0020_4ae8),
        (4, 0xf, 0x0020_4b0c),
        (7, 0xf, 0x0020_4b30),
        (5, 0xf, 0x0020_4b54),
        (0xa, 0xf, 0x0020_4b78),
        (4, 0x10, 0x0020_4b9c),
        (8, 0x10, 0x0020_4bc0),
        (5, 0x10, 0x0020_4be4),
        (0xb, 0x10, 0x0020_4c08),
    ];
    for (i, (a3, t0, ret)) in GOLD.into_iter().enumerate() {
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = a3;
        c.g[T0][0] = t0;
        pk_call(c, PK_TRACK_MEDAL, ret)?;
        if i == 0 {
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
        } else {
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = !(0u64 | c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V1][0];
            }
        }
    }
    c.g[A1][0] = 0x004a_0000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2680));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S0][0];
    pk_call(c, PK_SPRINTF, 0x0020_4c28)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4c34)?;
    c.g[V0][0] = ((c.g[S0][0] as i64) < 12) as u64;
    let short = c.g[V0][0] != 0;
    c.g[V0][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd10));
        pk_call(c, PK_HIDE, 0x0020_4c64)?;
    } else {
        c.cycles += 6;
        c.g[T0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0xd10));
        pk_call(c, PK_SHOW, 0x0020_4c50)?;
    }
    c.g[A0][0] = 0x0047_0000;

    // Missions done / total over the three peaks.
    c.cycles += 20;
    c.lwc1(24, sp(c).wrapping_add(PK_DONE));
    c.cvt_s_w(24, 24);
    c.mtc1(S0, 25);
    c.cvt_s_w(25, 25);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd28));
    pk_call(c, PK_HASH, 0x0020_4c80)?;
    c.g[S2][0] = 0;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[S4][0] = 0;
    c.g[S0][0] = 0;
    pk_call(c, PK_FIND, 0x0020_4c98)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xd40));
    pk_call(c, PK_HASH, 0x0020_4ca8)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, 0x0020_4cb4)?;
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);
    loop {
        c.cycles += 11;
        c.g[A1][0] = c.g[S0][0];
        pk_call(c, 0x0015_43e0, 0x0020_4cc0)?;
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        c.g[A1][0] = c.g[S0][0];
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        pk_call(c, 0x0015_4368, 0x0020_4cd4)?;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        if event_due(c) {
            deliver_event_here(c, 0x0020_4cb8);
        }
    }
    c.cycles += 19;
    c.g[A0][0] = 0x0047_0d58;
    pk_call(c, PK_HASH, 0x0020_4cf0)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4cfc)?;
    c.g[V1][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2398));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_4d18)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4d24)?;
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 3;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xd68));
        pk_call(c, PK_HIDE, 0x0020_4d54)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xd68));
        pk_call(c, PK_SHOW, 0x0020_4d40)?;
    }

    // Collectibles found / total.
    c.cycles += 9;
    c.mtc1(S2, 23);
    c.cvt_s_w(23, 23);
    c.g[S0][0] = 0;
    c.mtc1(S4, 22);
    c.cvt_s_w(22, 22);
    c.g[S2][0] = 0;
    c.g[S4][0] = 0;
    c.g[A0][0] = pk_lw_sp(c, PK_MIS);
    loop {
        c.cycles += 12;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        pk_call(c, 0x0015_35c0, 0x0020_4d84)?;
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        c.g[A1][0] = c.g[S0][0];
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        pk_call(c, 0x0015_3390, 0x0020_4d98)?;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 3) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = pk_lw_sp(c, PK_MIS);
        if event_due(c) {
            deliver_event_here(c, 0x0020_4d78);
        }
    }
    c.cycles += 19;
    c.g[A0][0] = 0x0047_0d80;
    pk_call(c, PK_HASH, 0x0020_4db4)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4dc0)?;
    c.g[T0][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x2398));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S4][0];
    pk_call(c, PK_SPRINTF, 0x0020_4ddc)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4de8)?;
    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S4][0] as i64)) as u64;
    let short = c.g[V0][0] != 0;
    c.g[V1][0] = 0x0047_0000;
    if short {
        c.cycles += 4;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xd90));
        pk_call(c, PK_HIDE, 0x0020_4e18)?;
    } else {
        c.cycles += 6;
        c.g[V0][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd90));
        pk_call(c, PK_SHOW, 0x0020_4e04)?;
    }
    c.g[A0][0] = 0x0047_0000;

    // Highlight levels (8 slots), "highlights / 24".
    c.cycles += 19;
    c.mtc1(S2, 21);
    c.cvt_s_w(21, 21);
    c.mtc1(S4, 20);
    c.cvt_s_w(20, 20);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xda8));
    pk_call(c, PK_HASH, 0x0020_4e34)?;
    c.g[S2][0] = 0;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[S0][0] = 0;
    pk_call(c, PK_FIND, 0x0020_4e48)?;
    c.g[A0][0] = 0x0047_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdc0));
    pk_call(c, PK_HASH, 0x0020_4e58)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_SET_TEXT_ID, 0x0020_4e64)?;
    c.g[A0][0] = pk_lw_sp(c, PK_SCORE);
    loop {
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
        pk_call(c, 0x0015_52b0, 0x0020_4e74)?;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = ((c.g[S0][0] as i64) < 8) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = pk_lw_sp(c, PK_SCORE);
        if event_due(c) {
            deliver_event_here(c, 0x0020_4e68);
        }
    }
    c.cycles += 17;
    c.g[A1][0] = 0x004a_0000;
    c.g[A2][0] = c.g[S2][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2688));
    c.g[A0][0] = c.g[FP][0];
    pk_call(c, PK_SPRINTF, 0x0020_4e98)?;
    c.g[A0][0] = 0x0047_0dd8;
    pk_call(c, PK_HASH, 0x0020_4ea4)?;
    pk_screen_a0(c);
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4eb0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4ebc)?;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 24) as u64;
    let short = c.g[V0][0] != 0;
    c.g[A3][0] = 0x0047_0000;
    if short {
        c.cycles += 3;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xde8));
        pk_call(c, PK_HIDE, 0x0020_4eec)?;
    } else {
        c.cycles += 6;
        c.g[A2][0] = 0x0047_0000;
        pk_owner_a0(c);
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xde8));
        pk_call(c, PK_SHOW, 0x0020_4ed8)?;
    }

    // Completion percentage: (done/14 + gold/12 + missions + collectibles +
    // highlights/24) * 20, 100 only when exactly 100.0, else at most 99.
    c.cycles += 23;
    let a = (c.g[GP][0] as u32).wrapping_sub(0x552c);
    c.lwc1(0, a);
    c.div_s(7, 23, 22);
    c.lwc1(2, a.wrapping_add(4));
    c.div_s(6, 21, 20);
    c.mul_s(0, 24, 0);
    c.lwc1(3, a.wrapping_add(8));
    c.mul_s(2, 25, 2);
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(AT, 5);
    c.mtc1(S2, 1);
    c.cvt_s_w(1, 1);
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 4);
    c.add_s(0, 0, 2);
    c.mul_s(1, 1, 3);
    c.add_s(0, 0, 7);
    c.add_s(0, 0, 6);
    c.add_s(0, 0, 1);
    c.mul_s(1, 0, 5);
    c.c_eq_s(1, 4);
    let exactly_100 = c.fcc();
    c.g[S0][0] = 0x64;
    if !exactly_100 {
        c.cycles += 8;
        c.g[AT][0] = 0x42c6_0000;
        c.mtc1(AT, 0);
        c.cvt_w_s(2, 1);
        c.mfc1(S0, 2);
        c.c_le_s(0, 1);
        if c.fcc() {
            c.g[S0][0] = 0x63;
        }
    }
    c.cycles += 16;
    c.g[A0][0] = 0x0047_0e00;
    pk_call(c, PK_HASH, 0x0020_4f74)?;
    c.g[A0][0] = c.lw((pk_this(c)).wrapping_add(PK_SCREEN));
    c.g[A1][0] = c.g[V0][0];
    pk_call(c, PK_FIND, 0x0020_4f80)?;
    c.g[A1][0] = 0x004a_0000;
    c.g[S1][0] = c.g[V0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2690));
    c.g[A0][0] = c.g[FP][0];
    c.g[A2][0] = c.g[S0][0];
    pk_call(c, PK_SPRINTF, 0x0020_4f98)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[FP][0];
    pk_call(c, PK_SET_STRING, 0x0020_4fa4)?;
    pk_leave(c, frame);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{elf_image, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of the fakes (magic word in the machine's RAM) and the knobs the
    /// setup writes for them.
    pub(crate) const OC442_CTRL: u32 = DATA + 0x8000;
    pub(crate) const OC442_MAGIC: u32 = 0x4f43_3434;
    pub(crate) const OC442_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC442_SCREEN: u32 = DATA + 0x2000;
    pub(crate) const OC442_OWNER: u32 = DATA + 0x3000;
    pub(crate) const OC442_NODE1: u32 = DATA + 0x3100;
    pub(crate) const OC442_NODE2: u32 = DATA + 0x3200;
    /// The one object every `UIScreen_FindObject(screen, ..)` returns (list,
    /// text, mountain: its vtable has a fake method in every slot).
    pub(crate) const OC442_OBJ: u32 = DATA + 0x4000;
    pub(crate) const OC442_VT: u32 = DATA + 0x5000;
    pub(crate) const OC442_SEL: u32 = DATA + 0x6000;
    pub(crate) const OC442_WORLD: u32 = DATA + 0x7000;
    pub(crate) const OC442_LOC: u32 = DATA + 0x7100;
    pub(crate) const OC442_LOCVT: u32 = DATA + 0x7200;

    /// A fake callee: logs the call (all registers, cycles) through
    /// `fakes::fake`, then sets the result the callee would return from a
    /// seeded hash and the knobs in the control block.
    pub(crate) fn oc442_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let seed = c.bus.read32(OC442_CTRL + 4) as u64;
        let trophy_pct = c.bus.read32(OC442_CTRL + 8) as u64;
        let medal_pct = c.bus.read32(OC442_CTRL + 0xc) as u64;
        let h = fakes::mix((seed << 32) ^ ((addr as u64) << 8) ^ n as u64 ^ 0x9e37_79b9);
        let v = match addr {
            // The rider character.
            0x0014_a080 => Some(h % 6),
            // isTrophyOwned.
            0x0015_7bf0 => Some((h % 100 < trophy_pct) as u64),
            // getTrackMedal: -1 (none) or a medal 0..3.
            0x0015_8910 => Some(if h % 100 < medal_pct {
                (h >> 8) % 4
            } else {
                u64::MAX
            }),
            // Mission, collectible, medal-target and money counters.
            0x0015_43e0 | 0x0015_4368 | 0x0015_35c0 | 0x0015_3390 | 0x0015_8890 | 0x0015_8870
            | 0x0015_09f8 => Some((h >> 8) % 5),
            // FindObject: the screen's objects (one lookup may fail), nothing
            // in the state owner's list nodes.
            0x0039_d860 => {
                if a0 == OC442_SCREEN {
                    let k = c.bus.read32(OC442_CTRL + 0x10);
                    c.bus.write32(OC442_CTRL + 0x10, k + 1);
                    let zero_at = c.bus.read32(OC442_CTRL + 0x14);
                    Some(if k == zero_at { 0 } else { OC442_OBJ as u64 })
                } else {
                    Some(0)
                }
            }
            _ => None,
        };
        if let Some(v) = v {
            c.g[V0][0] = v;
        }
    }

    macro_rules! oc442_fakes {
        ($(($fake:ident, $addr:expr, $gen:path, $slot:path)),* $(,)?) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC442_CTRL) != OC442_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    oc442_fake_call(c, $addr)
                }
            )*

            /// Hooks the fakes into the callees' override slots (once per
            /// process; with the gate clear they run what the slots held).
            pub(crate) fn oc442_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc442_fakes! {
        (oc442_fake_be, 0x0014_dc80, ee::s01::f_0014dc80__generated, ee::s01::OVERRIDE_0014DC80),
        (oc442_fake_interface, 0x0014_dd58, ee::s01::f_0014dd58__generated, ee::s01::OVERRIDE_0014DD58),
        (oc442_fake_rider_char, 0x0014_a080, ee::s01::f_0014a080__generated, ee::s01::OVERRIDE_0014A080),
        (oc442_fake_show, 0x0039_e948, ee::s13::f_0039e948__generated, ee::s13::OVERRIDE_0039E948),
        (oc442_fake_trophy, 0x0015_7bf0, ee::s01::f_00157bf0__generated, ee::s01::OVERRIDE_00157BF0),
        (oc442_fake_find, 0x0039_d860, ee::s13::f_0039d860__generated, ee::s13::OVERRIDE_0039D860),
        (oc442_fake_set_wide, 0x003a_0e90, ee::s13::f_003a0e90__generated, ee::s13::OVERRIDE_003A0E90),
        (oc442_fake_set_ascii, 0x003a_0c70, ee::s13::f_003a0c70__generated, ee::s13::OVERRIDE_003A0C70),
        (oc442_fake_medal, 0x0015_8910, ee::s01::f_00158910__generated, ee::s01::OVERRIDE_00158910),
        (oc442_fake_done_missions, 0x0015_43e0, ee::s01::f_001543e0__generated, ee::s01::OVERRIDE_001543E0),
        (oc442_fake_challenge_target, 0x0015_8890, ee::s01::f_00158890__generated, ee::s01::OVERRIDE_00158890),
        (oc442_fake_collect, 0x0015_35c0, ee::s01::f_001535c0__generated, ee::s01::OVERRIDE_001535C0),
        (oc442_fake_collect_target, 0x0015_8870, ee::s01::f_00158870__generated, ee::s01::OVERRIDE_00158870),
        (oc442_fake_money, 0x0015_09f8, ee::s01::f_001509f8__generated, ee::s01::OVERRIDE_001509F8),
        (oc442_fake_num_missions, 0x0015_4368, ee::s01::f_00154368__generated, ee::s01::OVERRIDE_00154368),
        (oc442_fake_total_collect, 0x0015_3390, ee::s01::f_00153390__generated, ee::s01::OVERRIDE_00153390),
    }

    /// One seed of `EventSelect_RefreshPeakList`: map style 0..3, map state,
    /// popup flag, unlocked peaks, the selected peak 0..4 (the overall entry
    /// from 3), missing list/selection, and the counters the fakes return
    /// (trophies, medals, missions) from a seeded hash. Random goal records
    /// (kinds and tracks, both 4-aligned and 8-aligned copies), earnings
    /// targets and highlight levels in RAM; the strings and format strings
    /// come from the executable image. Real callees: hash, sprintf, hide,
    /// SetTextId, SetString (the text vtable slots are fake methods), the
    /// earnings and highlight accessors.
    pub(crate) fn oc442_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x0020_3d40, 200);
        elf_image(c);
        c.pc = 0x0020_3d40;
        c.g[A0][0] = sx(OC442_SELF);

        let style = seed % 4;
        let state = (seed / 4) % 2;
        let popup = (seed / 8) % 2;
        let peak = r.next_u32() % 5;
        c.bus.write32(GAME_GP - 0xb5c, style);
        c.bus.write32(GAME_GP - 0xb54, state);
        c.bus.write32(GAME_GP - 0x848, OC442_WORLD);

        // The map template.
        c.bus.write32(OC442_SELF + PK_SCREEN, OC442_SCREEN);
        c.bus.write32(OC442_SELF + PK_OWNER, OC442_OWNER);
        c.bus.write32(OC442_SELF + PK_POPUP, popup);
        for i in 0..3 {
            c.bus
                .write32(OC442_SELF + PK_UNLOCKED + 4 * i, r.next_u32() % 2);
        }
        // The state owner's object list: two nodes, so `UIState_HideObject`
        // looks the object up twice.
        c.bus.write32(OC442_OWNER + 0x28, OC442_NODE1);
        c.bus.write32(OC442_NODE1 + 4, OC442_NODE2);
        // The text/list object and its vtable: a fake method in every slot.
        c.bus.write32(OC442_OBJ + 8, OC442_VT);
        let no_sel = r.next_u32() % 8 == 0;
        c.bus
            .write32(OC442_OBJ + PK_LIST_SEL, if no_sel { 0 } else { OC442_SEL });
        c.bus.write32(OC442_SEL + PK_SEL_PEAK, peak);
        for off in (0..0x200).step_by(8) {
            c.bus.write16(OC442_VT + off, 0);
            c.bus
                .write32(OC442_VT + off + 4, fakes::FAKE + 0x10 * (off / 8));
        }
        // The localisation lookup through the world (`world+0x8c`).
        c.bus.write32(OC442_WORLD + 0x8c, OC442_LOC);
        c.bus.write32(OC442_LOC + 4, OC442_LOCVT);
        c.bus.write16(OC442_LOCVT + 0x20, 0);
        c.bus.write32(OC442_LOCVT + 0x24, fakes::FAKE + 0x1000);

        // Goal records: table A (3 peaks x 4 goals, 0x6c each; 0x1b0 a peak)
        // and table B (3 x 5; 0x21c a peak): random track and kind.
        for (base, stride, n) in [(0x0047_81d0u32, 0x1b0u32, 4u32), (0x0047_86e0, 0x21c, 5)] {
            for p in 0..3 {
                for k in 0..n {
                    let at = base + p * stride + k * 0x6c;
                    for w in 0..27 {
                        c.bus.write32(at + 4 * w, r.next_u32());
                    }
                    c.bus.write32(at, r.next_u32() % 20);
                    c.bus.write32(at + 8, r.next_u32() % 14);
                }
            }
        }
        // Earnings targets and highlight levels.
        for p in 0..6 {
            c.bus.write32(0x0045_b048 + 4 * p, r.next_u32() % 5);
        }
        let fill = r.next_u32() % 3;
        for o in 0..0x5d60u32 {
            let b = match fill {
                0 => 0,
                1 => 3,
                _ => r.next_u32() % 4,
            };
            c.bus.write8(0x004a_7860 + o, b as u8);
        }

        // The fakes' gate and answers.
        let trophy_pct = [0, 60, 100, 95][(r.next_u32() % 4) as usize];
        let medal_pct = [0, 30, 80, 100][(r.next_u32() % 4) as usize];
        c.bus.write32(OC442_CTRL, OC442_MAGIC);
        c.bus.write32(OC442_CTRL + 4, seed);
        c.bus.write32(OC442_CTRL + 8, trophy_pct);
        c.bus.write32(OC442_CTRL + 0xc, medal_pct);
        c.bus.write32(OC442_CTRL + 0x10, 0);
        // The k-th screen lookup that fails: the list lookup in 1 of 8 seeds
        // (third lookup with map style < 2, else the second).
        let list_lookup = if style < 2 { 2 } else { 1 };
        c.bus.write32(
            OC442_CTRL + 0x14,
            if r.next_u32() % 8 == 0 {
                list_lookup
            } else {
                u32::MAX
            },
        );
    }

    /// `EventSelect_RefreshPeakList` (0x203d40) against the generated body:
    /// map styles 0..3 and both map states, popup on/off, locked and unlocked
    /// peaks, trophies and medals (every combination of found and missing),
    /// the selected peak 0..2 (help text; goal counters on style >= 2) and the
    /// overall entry (peaks 3 and 4: done/gold/missions/collectibles/highlight
    /// counters and the percentage), a missing list or selection, and thread
    /// unwinds in the callees. The back-end accessors and the screen lookups
    /// are fakes; the formatting, hashing and state hide/show run real.
    #[test]
    fn event_select_refresh_peak_list_matches() {
        oc442_install_fakes();
        covered(
            ee::s05::f_00203d40__generated,
            event_select_refresh_peak_list,
            300,
            &oc442_setup,
        );
    }
}
