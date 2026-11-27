//! Staging module oc945 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc945`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, tail_wrapper, Callee, Flow, Frame, Target, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_3c80,
        cfe_state_peak_room_on_widget_action,
        "cFEStatePeakRoom_OnWidgetAction",
    ),
    (
        0x001d_3ec8,
        cfe_state_peak_room_update_help_text,
        "cFEStatePeakRoom_UpdateHelpText",
    ),
    (
        0x001d_4268,
        cfe_state_trophy_room_ctor,
        "cFEStateTrophyRoom_ctor",
    ),
    (
        0x001d_43d0,
        cfe_state_trophy_room_on_activate,
        "cFEStateTrophyRoom_OnActivate",
    ),
    (
        0x001d_47a0,
        cfe_state_trophy_room_show_trophy_image,
        "cFEStateTrophyRoom_ShowTrophyImage",
    ),
    (
        0x001d_4918,
        cfe_state_trophy_room_request_trophy_images,
        "cFEStateTrophyRoom_RequestTrophyImages",
    ),
    (
        0x001d_4a20,
        cfe_state_trophy_room_load_trophy_textures,
        "cFEStateTrophyRoom_LoadTrophyTextures",
    ),
    (
        0x001d_4cd8,
        cfe_state_rewards_room_on_widget_created,
        "cFEStateRewardsRoom_OnWidgetCreated",
    ),
];

/// `r32` plus an offset: the address `off(reg)`.
#[inline(always)]
fn oc945_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `lw off(reg)`.
#[inline(always)]
fn oc945_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc945_at(c, reg, off);
    c.lw(a)
}

/// `lh off(reg)`.
#[inline(always)]
fn oc945_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc945_at(c, reg, off);
    c.lh(a)
}

/// `lb off(reg)`.
#[inline(always)]
fn oc945_lb(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = oc945_at(c, reg, off);
    c.lb(a)
}

/// `lw -off(gp)` (the world and BE pointers the game keeps at gp).
#[inline(always)]
fn oc945_lw_gp(c: &mut Cpu, off: u32) -> u64 {
    let a = r32(c, GP).wrapping_sub(off);
    c.lw(a)
}

/// `lw off(obj); call` through the object's vtable: `v0 = obj->vt`,
/// `a0 = sx(obj + vt->delta)`, `v1 = vt->fn`, call `v1` returning to `ret`.
fn oc945_vcall(c: &mut Cpu, obj_reg: usize, ret: u32) -> Flow {
    c.g[V0][0] = oc945_lw(c, obj_reg, 8);
    c.g[A0][0] = oc945_lh(c, V0, 0x48);
    c.g[V1][0] = oc945_lw(c, V0, 0x4c);
    let target = r32(c, V1);
    c.g[A0][0] = sx(r32(c, obj_reg).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

// ---------------------------------------------------------------------------
// cFEStatePeakRoom::OnWidgetAction (0x1d3c80, 388 bytes)
// ---------------------------------------------------------------------------

/// Where the widget-action body leaves to: the restores it still owes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PeakTail {
    /// `cycles += 1; lq s0`, then `B`.
    A,
    /// `cycles += 1; lq s1`, then `C`.
    B,
    /// The epilogue (`lq s2, s3, ra`, `sp`).
    C,
}

type PeakFlow = Result<PeakTail, Unwound>;

/// The shared epilogue of the widget-action paths.
fn peak_finish(c: &mut Cpu, frame: Frame, from: PeakTail) {
    if from == PeakTail::A {
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
    }
    if from != PeakTail::C {
        c.cycles += 1;
        frame.restore(c, S1, 0x30);
    }
    c.cycles += 5;
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cFEStatePeakRoom::OnWidgetAction(self, widget, kind)` (0x1d3c80): with a
/// widget, the action `kind` picks the room's path: 5 opens the popup (BE
/// lookup, the popup's virtual call, then the slot update), 1 refreshes the help
/// text and goal list, 6 walks the four goal entries of `self+0x48`, any other
/// kind leaves at once. `self` lives in s1, the widget in s2.
pub fn cfe_state_peak_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let tail = if c.g[S2][0] == 0 {
            PeakTail::A
        } else {
            peak_action_body(c, frame)?
        };
        peak_finish(c, frame, tail);
        Ok(())
    })
}

/// Dispatch on the action kind (`a2`).
fn peak_action_body(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_popup = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[A2][0] < 6) as u64;
    if is_popup {
        return peak_popup_path(c, frame);
    }
    c.cycles += 2;
    let at_least_six = c.g[V0][0] == 0;
    c.g[V0][0] = 1;
    if at_least_six {
        c.cycles += 3;
        c.g[V0][0] = 6;
        let is_goals = c.g[A2][0] == c.g[V0][0];
        frame.restore(c, S0, 0x40);
        if is_goals {
            return peak_goal_path(c, frame);
        }
        c.cycles += 2;
        frame.restore(c, S1, 0x30);
        return Ok(PeakTail::C);
    }
    c.cycles += 2;
    let is_help = c.g[A2][0] == c.g[V0][0];
    frame.restore(c, S0, 0x40);
    if is_help {
        return peak_help_path(c);
    }
    c.cycles += 2;
    frame.restore(c, S1, 0x30);
    Ok(PeakTail::C)
}

/// Kind 5: the BE lookup and the popup object's virtual call, then the slot
/// update through `self+0x10`.
fn peak_popup_path(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 13;
    call_ptr(c, 0x0014_dc80, 0x001d_3ce4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001d_3cf0)?;
    c.g[A1][0] = oc945_lb(c, S1, 0x44);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = oc945_lw(c, S1, 0xdc);
    c.g[A3][0] = oc945_lw(c, S1, 0xe0);
    c.g[T0][0] = oc945_lw(c, S2, 0x18);
    call_ptr(c, 0x0015_7bf0, 0x001d_3d08)?;
    let found = c.g[V0][0] != 0;
    c.g[A1][0] = c.g[S1][0];
    if !found {
        return Ok(PeakTail::A);
    }
    c.cycles += 13;
    c.g[V0][0] = oc945_lw(c, S1, 0x10);
    c.g[A2][0] = oc945_lw(c, S1, 0xe0);
    c.g[A3][0] = oc945_lw(c, V0, 0);
    c.g[A0][0] = oc945_lw(c, S2, 0x18);
    c.g[A2][0] = sx(r32(c, A2) << 16);
    c.g[V1][0] = oc945_lw(c, A3, 4);
    c.g[A2][0] = c.g[A2][0] | c.g[A0][0];
    c.g[A0][0] = oc945_lh(c, V1, 0x20);
    c.g[V0][0] = oc945_lw(c, V1, 0x24);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001d_3d3c)?;
    c.g[A0][0] = oc945_lw(c, S1, 0x10);
    peak_slot_path(c, frame)
}

/// The shared tail of the popup and goal paths: `self+0x10`'s slot call
/// (`a1` = `v0`), then the epilogue through [`PeakTail::A`].
fn peak_slot_path(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 5;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f400, 0x001d_3dc8)?;
    frame.restore(c, S0, 0x40);
    Ok(PeakTail::B)
}

/// Kind 1: refresh the help text and the goal list.
fn peak_help_path(c: &mut Cpu) -> PeakFlow {
    c.cycles += 6;
    c.g[A1][0] = oc945_lw(c, S2, 0x18);
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x001d_3ec8,
        f: ee::s04::f_001d3ec8,
    }
    .call(c, 0x001d_3ddc)?;
    c.g[A1][0] = oc945_lw(c, S2, 0x18);
    c.g[A0][0] = c.g[S1][0];
    Callee {
        addr: 0x001d_3f80,
        f: ee::s04::f_001d3f80,
    }
    .call(c, 0x001d_3de8)?;
    Ok(PeakTail::A)
}

/// Kind 6: the four goal entries at `self+0x48` (a loop that calls each
/// entry's virtual method), then the slot call on `self+0x10`.
fn peak_goal_path(c: &mut Cpu, frame: Frame) -> PeakFlow {
    c.cycles += 7;
    c.g[V0][0] = oc945_lw(c, S1, 0xe0);
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x48));
    c.g[V1][0] = oc945_lw(c, S2, 0x18);
    c.g[V0][0] = sx(r32(c, V0) << 16);
    c.g[S2][0] = 3;
    c.g[S3][0] = c.g[V0][0] | c.g[V1][0];
    loop {
        c.cycles += 3;
        c.g[A2][0] = oc945_lw(c, S0, 0);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !empty {
            c.cycles += 5;
            oc945_vcall(c, A2, 0x001d_3d80)?;
        }
        c.cycles += 3;
        c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
        let more = (c.g[S2][0] as i64) >= 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001d_3d60);
        }
    }
    c.cycles += 11;
    c.g[V0][0] = oc945_lw(c, S1, 0x10);
    c.g[A2][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = oc945_lw(c, V0, 0);
    c.g[V1][0] = oc945_lw(c, A3, 4);
    c.g[A0][0] = oc945_lh(c, V1, 0x28);
    c.g[V0][0] = oc945_lw(c, V1, 0x2c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x001d_3db0)?;
    let no_result = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x40);
    if no_result {
        return Ok(PeakTail::B);
    }
    c.cycles += 1;
    c.g[A0][0] = oc945_lw(c, S1, 0x10);
    peak_slot_path(c, frame)
}

// ---------------------------------------------------------------------------
// cFEStatePeakRoom::UpdateHelpText (0x1d3ec8, 184 bytes)
// ---------------------------------------------------------------------------

/// `void cFEStatePeakRoom::UpdateHelpText(self, id)` (0x1d3ec8): when the room
/// has its help object (`self+0xd0`), look up the text for `id` (the BE table
/// and two strings at 0x467688 and 0x4676a0) and hand it to the object's
/// virtual method.
pub fn cfe_state_peak_room_update_help_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = oc945_lw(c, S0, 0xd0);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            help_text_lookup(c)?;
            c.cycles += 6;
            c.g[A1][0] = 1;
            oc945_vcall(c, A2, 0x001d_3f6c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The text lookup of [`cfe_state_peak_room_update_help_text`]: sets `a2` to the
/// help object.
fn help_text_lookup(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    call_ptr(c, 0x0014_dc80, 0x001d_3ef0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001d_3efc)?;
    c.g[A1][0] = oc945_lb(c, S0, 0x44);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = oc945_lw(c, S0, 0xdc);
    c.g[T0][0] = c.g[S1][0];
    c.g[A3][0] = oc945_lw(c, S0, 0xe0);
    call_ptr(c, 0x0015_7bf0, 0x001d_3f14)?;
    if c.g[V0][0] == 0 {
        c.g[A0][0] = 0x0046_0000;
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x76a0));
        call_ptr(c, 0x0031_7670, 0x001d_3f44)?;
        c.g[A0][0] = oc945_lw(c, S0, 0xd0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001d_3f50)?;
        c.g[A2][0] = oc945_lw(c, S0, 0xd0);
    } else {
        c.cycles += 8;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x7688));
        call_ptr(c, 0x0031_7670, 0x001d_3f28)?;
        c.g[A0][0] = oc945_lw(c, S0, 0xd0);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001d_3f34)?;
        c.g[A2][0] = oc945_lw(c, S0, 0xd0);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateTrophyRoom (0x1d4268 .. 0x1d4a20)
// ---------------------------------------------------------------------------

/// `cFEStateTrophyRoom::cFEStateTrophyRoom` (0x1d4268, 252 bytes): the UI state
/// base constructor, then the trophy widget's text fields, the eight slots at
/// `self+0x74` set to -1, and the trophy image request.
pub fn cfe_state_trophy_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039_e2a0, 0x001d_4298)?;
        c.g[V0][0] = 0x2d;
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x6948));
        let a = oc945_at(c, S3, 0xc);
        c.sw(a, c.g[V0][0]);
        let a = oc945_at(c, S3, 0x8);
        c.sw(a, c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        let a = oc945_at(c, S3, 0x44);
        c.sb(a, c.g[S0][0]);
        c.g[V0][0] = oc945_lw(c, S3, 0x10);
        c.g[A0][0] = oc945_lw(c, V0, 0);
        call_ptr(c, 0x001a_1cd0, 0x001d_42c0)?;
        let a = oc945_at(c, S3, 0x48);
        c.sw(a, 0);
        let a = oc945_at(c, S3, 0x15);
        c.sb(a, c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x001d_42cc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x001d_42d8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = oc945_lb(c, S3, 0x44);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0014_7488, 0x001d_42e8)?;
        let a = oc945_at(c, S3, 0x50);
        c.sw(a, c.g[V0][0]);
        c.g[V0][0] = oc945_lw(c, S0, 0xc);
        c.g[A0][0] = oc945_lh(c, V0, 0x10);
        c.g[V1][0] = oc945_lw(c, V0, 0x14);
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x001d_4300)?;
        let a = oc945_at(c, S3, 0x54);
        c.sw(a, c.g[S1][0]);
        c.g[A0][0] = u64::MAX;
        let a = oc945_at(c, S3, 0x58);
        c.sw(a, c.g[S2][0]);
        c.g[V0][0] = sx(oc945_at(c, S3, 0x74));
        let a = oc945_at(c, S3, 0x5c);
        c.sw(a, 0);
        c.g[V1][0] = 4;
        loop {
            c.cycles += 7;
            let a = r32(c, V0).wrapping_sub(0x14);
            c.sw(a, c.g[A0][0]);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
            let a = r32(c, V0);
            c.sw(a, c.g[A0][0]);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_4318);
            }
        }
        c.cycles += 12;
        c.g[V0][0] = u64::MAX;
        c.g[A0][0] = c.g[S3][0];
        let a = oc945_at(c, S3, 0x9c);
        c.sw(a, c.g[V0][0]);
        Callee {
            addr: 0x001d_4918,
            f: ee::s04::f_001d4918,
        }
        .call(c, 0x001d_4344)?;
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateTrophyRoom::OnActivate` (0x1d43d0): forwards to the UI state's
/// activate through the table.
pub fn cfe_state_trophy_room_on_activate(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Table(0x0039_e4c0), 0x001d_43e0)
}

/// `cFEStateTrophyRoom::ShowTrophyImage(self, index)` (0x1d47a0): the trophy
/// slot for `index` (negative: the image of the last trophy), then the item's
/// virtual method, and the slot's image index written at `item+0x78`.
pub fn cfe_state_trophy_room_show_trophy_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let negative = (c.g[A1][0] as i64) < 0;
        frame.save_ra(c, 0);
        // A negative index, or no slot found, takes the "no slot" exit.
        let has_slot = if negative {
            false
        } else {
            trophy_slot(c)?;
            c.cycles += 2;
            (c.g[S1][0] as i64) >= 0
        };
        if has_slot {
            trophy_item_slot(c, frame)?;
        } else {
            trophy_item_no_slot(c, frame)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The slot search of [`cfe_state_trophy_room_show_trophy_image`]: with an
/// index, `QueryEventReward` gives the slot; without one, the first trophy
/// name is hashed and looked up. Leaves the slot in s1 (-1 if none).
fn trophy_slot(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    let has_index = c.g[A1][0] != 0;
    c.g[S1][0] = u64::MAX;
    if has_index {
        c.cycles += 8;
        c.g[T0][0] = sx(r32(c, A1).wrapping_sub(1));
        c.g[A0][0] = oc945_lb(c, S0, 0x44);
        c.g[A1][0] = oc945_lw(c, S0, 0x50);
        c.g[A2][0] = oc945_lw(c, S0, 0x54);
        c.g[A3][0] = oc945_lw(c, S0, 0x58);
        Callee {
            addr: 0x001c_ed90,
            f: ee::s04::f_001ced90,
        }
        .call(c, 0x001d_4814)?;
        let same = c.g[V0][0] == c.g[S1][0];
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        if !same {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
            c.g[S1][0] = oc945_lw(c, V0, 0x60);
        }
        return Ok(());
    }
    c.cycles += 13;
    call_ptr(c, 0x0014_dc80, 0x001d_47c8)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001d_47d4)?;
    c.g[A1][0] = oc945_lb(c, S0, 0x44);
    c.g[A0][0] = c.g[V0][0];
    c.g[A2][0] = oc945_lw(c, S0, 0x50);
    c.g[A3][0] = oc945_lw(c, S0, 0x54);
    c.g[T0][0] = oc945_lw(c, S0, 0x58);
    call_ptr(c, 0x0015_7bf0, 0x001d_47ec)?;
    if c.g[V0][0] != 0 {
        c.g[S1][0] = oc945_lw(c, S0, 0x60);
    } else {
        c.cycles += 2;
    }
    Ok(())
}

/// The slot is known (`s1 >= 0`): the item's method is called with `a1 = 1`,
/// then the slot and a zero are stored in the item.
fn trophy_item_slot(c: &mut Cpu, frame: Frame) -> Flow {
    c.g[A2][0] = oc945_lw(c, S0, 0x48);
    c.cycles += 10;
    c.g[A1][0] = 1;
    oc945_vcall(c, A2, 0x001d_486c)?;
    c.g[V0][0] = oc945_lw(c, S0, 0x48);
    let a = oc945_at(c, V0, 0x78);
    c.sw(a, c.g[S1][0]);
    let a = oc945_at(c, V0, 0x7c);
    c.sw(a, 0);
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// No slot (`s1 < 0`): the item's method is called with `a1 = 0`.
fn trophy_item_no_slot(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    c.g[A2][0] = oc945_lw(c, S0, 0x48);
    c.g[A1][0] = 0;
    oc945_vcall(c, A2, 0x001d_484c)?;
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// `cFEStateTrophyRoom::RequestTrophyImages(self)` (0x1d4918): hash the five
/// trophy kinds into the list at `self+0x74`, one entry per tile, through the
/// world's texture table. Each entry asks for its image.
pub fn cfe_state_trophy_room_request_trophy_images(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0);
        c.g[S5][0] = sx(oc945_at(c, S0, 0x74));
        call_ptr(c, 0x0014_dc80, 0x001d_4948)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d_4954)?;
        c.g[S4][0] = c.g[V0][0];
        c.g[A1][0] = oc945_lw(c, S0, 0x54);
        c.g[V0][0] = oc945_lw_gp(c, 0x848);
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = oc945_lw(c, S0, 0x58);
        c.g[V1][0] = oc945_lw(c, V0, 0x7c);
        c.g[S3][0] = sx(r32(c, V1).wrapping_add(0x1a70));
        call_ptr(c, 0x0015_6a90, 0x001d_4974)?;
        c.g[A2][0] = oc945_lw(c, V0, 4);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = oc945_lw(c, S0, 0x50);
        c.g[A3][0] = 0xa;
        c.g[T0][0] = 1;
        c.g[T1][0] = 0;
        call_ptr(c, 0x0019_da10, 0x001d_4990)?;
        c.g[V1][0] = 1;
        let a = oc945_at(c, S0, 0x74);
        c.sw(a, c.g[V0][0]);
        let a = oc945_at(c, S0, 0x5c);
        c.sw(a, c.g[V1][0]);
        c.g[A1][0] = oc945_lw(c, S0, 0x58);
        loop {
            // One tile per pass: the tile's record, then the next tile.
            c.cycles += 12;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S4][0];
            c.g[S1][0] = sx(r32(c, S2).wrapping_add(1));
            call_ptr(c, 0x0015_6ae0, 0x001d_49b0)?;
            c.g[S2][0] = c.g[S1][0];
            c.g[V1][0] = oc945_lw(c, V0, 4);
            c.g[A0][0] = c.g[S3][0];
            c.g[A3][0] = 0xa;
            c.g[T0][0] = 1;
            c.g[T1][0] = 0;
            let empty = c.g[V1][0] == 0;
            c.g[A2][0] = c.g[V1][0];
            if !empty {
                c.cycles += 8;
                c.g[A1][0] = oc945_lw(c, S0, 0x50);
                call_ptr(c, 0x0019_da10, 0x001d_49d8)?;
                c.g[V1][0] = sx(r32(c, S1) << 2);
                c.g[V1][0] = sx(r32(c, S5).wrapping_add(r32(c, V1)));
                let a = r32(c, V1);
                c.sw(a, c.g[V0][0]);
                c.g[V0][0] = oc945_lw(c, S0, 0x5c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                let a = oc945_at(c, S0, 0x5c);
                c.sw(a, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 4) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A1][0] = oc945_lw(c, S0, 0x58);
            if event_due(c) {
                deliver_event_here(c, 0x001d_49a0);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateTrophyRoom::LoadTrophyTextures(self)` (0x1d4a20): for each of the
/// five list entries with an index, load its texture (the world's texture
/// table) and show the trophy image for it.
pub fn cfe_state_trophy_room_load_trophy_textures(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        c.g[V1][0] = oc945_lw_gp(c, 0x848);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = sx(oc945_at(c, S0, 0x74));
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = 4;
        frame.save(c, S5, 0x10);
        c.g[S4][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[S5][0] = 0x004a_0000;
        c.g[V0][0] = oc945_lw(c, V1, 0x7c);
        c.g[S2][0] = sx(r32(c, V0).wrapping_add(0x1a70));
        loop {
            c.cycles += 5;
            c.g[V0][0] = oc945_lw(c, S1, 0);
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            c.g[A0][0] = c.g[S2][0];
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[A2][0] = c.g[V0][0];
            if !negative {
                c.cycles += 4;
                c.g[A1][0] = oc945_lw(c, S0, 0x50);
                call_ptr(c, 0x0019_e238, 0x001d_4a7c)?;
                let no_entry = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                if !no_entry {
                    c.cycles += 27;
                    c.g[A1][0] = oc945_lw(c, S0, 0x50);
                    c.g[A2][0] = oc945_lw(c, S1, 0);
                    call_ptr(c, 0x0019_e2b0, 0x001d_4a90)?;
                    c.g[T2][0] = oc945_lw_gp(c, 0x854);
                    c.g[A2][0] = sx(r32(c, S5).wrapping_add(0x1ff8));
                    c.g[A1][0] = oc945_lw(c, V0, 0x14);
                    c.g[A3][0] = 0;
                    c.g[V1][0] = oc945_lw(c, T2, 0x10d8);
                    c.g[T0][0] = 1;
                    c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[T1][0] = u64::MAX;
                    c.g[A0][0] = oc945_lh(c, V1, 0x170);
                    c.g[V0][0] = oc945_lw(c, V1, 0x174);
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x001d_4ac0)?;
                    let a = r32(c, S1).wrapping_sub(0x14);
                    c.sw(a, c.g[V0][0]);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = oc945_lw(c, S1, 0);
                    c.g[A1][0] = oc945_lw(c, S0, 0x50);
                    call_ptr(c, 0x0019_dc20, 0x001d_4ad4)?;
                    let a = r32(c, S1);
                    c.sw(a, c.g[S4][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[V0][0] = oc945_lw(c, S0, 0x5c);
                    c.g[A1][0] = oc945_lw(c, S0, 0x9c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                    let a = oc945_at(c, S0, 0x5c);
                    c.sw(a, c.g[V0][0]);
                    Callee {
                        addr: 0x001d_47a0,
                        f: ee::s04::f_001d47a0,
                    }
                    .call(c, 0x001d_4af0)?;
                }
            }
            c.cycles += 2;
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_4a60);
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateRewardsRoom::OnWidgetCreated (0x1d4cd8, 460 bytes)
// ---------------------------------------------------------------------------

/// Where the rewards chain leaves to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RewardsExit {
    /// `cycles += 1; lq s0`, then the epilogue.
    Epilogue8c,
    /// The epilogue (the restores were done on the way).
    Epilogue90,
}

/// One string of the rewards chain: `a0 = sx(a0 + off)` (the table base is
/// already in `a0`), hash it, and report whether it is the widget's `s0`.
fn rewards_hash(c: &mut Cpu, off: u32, ret: u32) -> Result<bool, Unwound> {
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(off));
    call_ptr(c, 0x0031_7670, ret)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// The count-text call of a matched rewards entry (`a1` = the count).
fn rewards_set_count(c: &mut Cpu, count: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A1][0] = count;
    Callee {
        addr: 0x001d_4f90,
        f: ee::s04::f_001d4f90,
    }
    .call(c, ret)
}

/// The rewards chain of [`cfe_state_rewards_room_on_widget_created`]: each
/// string of the table (hashed in turn, the hash compared with the widget's
/// name in `s0`) decides which entry of the room the widget is.
fn rewards_chain(c: &mut Cpu, frame: Frame) -> Result<RewardsExit, Unwound> {
    // Entries that store their kind at `self+0x18`.
    const KINDS: [(u32, u32, u64, u64); 6] = [
        (0x2000, 0x001d_4d04, 5, 0x004a_0000),
        (0x2008, 0x001d_4d20, 6, 0x004a_0000),
        (0x2010, 0x001d_4d3c, 3, 0x004a_0000),
        (0x2018, 0x001d_4d58, 7, 0x0046_0000),
        (0x7748, 0x001d_4d74, 4, 0x0046_0000),
        (0x7758, 0x001d_4d90, 8, 0x0046_0000),
    ];
    for (i, &(off, ret, kind, base)) in KINDS.iter().enumerate() {
        if i > 0 {
            c.cycles += 4;
        }
        let hit = rewards_hash(c, off, ret)?;
        c.g[A0][0] = base;
        if hit {
            c.cycles += 3;
            c.g[V0][0] = kind;
            let a = oc945_at(c, S1, 0x18);
            c.sw(a, c.g[V0][0]);
            return Ok(RewardsExit::Epilogue8c);
        }
    }
    // Entries that set the count text; the base of their strings is 0x460000.
    const COUNTS: [(u32, u32, u64, u32); 5] = [
        (0x74a8, 0x001d_4dac, 5, 0x001d_4dc4),
        (0x74b8, 0x001d_4dd4, 3, 0x001d_4dec),
        (0x74c8, 0x001d_4dfc, 7, 0x001d_4e14),
        (0x74d8, 0x001d_4e24, 4, 0x001d_4e3c),
        (0x74e8, 0x001d_4e4c, 8, 0x001d_4e64),
    ];
    for &(off, ret, count, text_ret) in COUNTS.iter() {
        c.cycles += 4;
        let hit = rewards_hash(c, off, ret)?;
        c.g[A0][0] = 0x0046_0000;
        if hit {
            c.cycles += 6;
            rewards_set_count(c, count, text_ret)?;
            frame.restore(c, S0, 0x30);
            return Ok(RewardsExit::Epilogue90);
        }
    }
    // The last string: the restore comes before the test.
    c.cycles += 4;
    let hit = rewards_hash(c, 0x7768, 0x001d_4e74)?;
    frame.restore(c, S0, 0x30);
    if !hit {
        return Ok(RewardsExit::Epilogue90);
    }
    c.cycles += 4;
    rewards_set_count(c, 6, 0x001d_4e8c)?;
    Ok(RewardsExit::Epilogue8c)
}

/// `cFEStateRewardsRoom::OnWidgetCreated(self, widget)` (0x1d4cd8): hash the
/// widget's name (`widget+0x38`) against the rewards strings and store the
/// matching kind at `self+0x18` (or set the count text of that entry).
pub fn cfe_state_rewards_room_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x004a_0000;
        c.g[S0][0] = oc945_lw(c, S1, 0x38);
        let exit = rewards_chain(c, frame)?;
        if exit == RewardsExit::Epilogue8c {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's fakes (in the machine's own RAM), its
    /// magic, and the forced results (one word per fake call: 0 keeps the
    /// fake's own value, otherwise the result is `word - 1`).
    pub(crate) const OC945_GATE: u32 = DATA + 0x1f_f000;
    pub(crate) const OC945_MAGIC: u32 = 0x0945_0a11;
    pub(crate) const OC945_RESULTS: u32 = OC945_GATE + 4;
    pub(crate) const OC945_CALLS: u32 = 16;

    /// Gated fake: logs the call like `fakes::fake`, then returns the forced
    /// result of this call index; with the gate off, runs the body it replaced.
    pub(crate) fn oc945_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: GuestFn) {
        if c.bus.read32(OC945_GATE) != OC945_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        let call = c.bus.read32(fakes::LOG_COUNT) - 1;
        if call < OC945_CALLS && c.pc == ra {
            let word = c.bus.read32(OC945_RESULTS + 4 * call);
            if word != 0 {
                c.g[V0][0] = (word - 1) as u64;
            }
        }
    }

    /// Fakes for every callee of this module's replacements.
    macro_rules! oc945_fakes {
        ($( $name:ident => $target:expr, $slot:path, $generated:path; )*) => {
            $(
                pub(crate) fn $name(c: &mut Cpu) {
                    oc945_gated(c, $target, fakes::prev_cell($name), $generated)
                }
            )*

            /// Hooks the fakes into the override slots of their callees (once per process).
            pub(crate) fn oc945_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    oc945_fakes! {
        oc945_fake_14dc80 => 0x0014_dc80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc945_fake_14dd58 => 0x0014_dd58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc945_fake_157bf0 => 0x0015_7bf0, ee::s01::OVERRIDE_00157BF0, ee::s01::f_00157bf0__generated;
        oc945_fake_147488 => 0x0014_7488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc945_fake_156a90 => 0x0015_6a90, ee::s01::OVERRIDE_00156A90, ee::s01::f_00156a90__generated;
        oc945_fake_156ae0 => 0x0015_6ae0, ee::s01::OVERRIDE_00156AE0, ee::s01::f_00156ae0__generated;
        oc945_fake_39f400 => 0x0039_f400, ee::s13::OVERRIDE_0039F400, ee::s13::f_0039f400__generated;
        oc945_fake_3a1000 => 0x003a_1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated;
        oc945_fake_39e2a0 => 0x0039_e2a0, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
        oc945_fake_39e4c0 => 0x0039_e4c0, ee::s13::OVERRIDE_0039E4C0, ee::s13::f_0039e4c0__generated;
        oc945_fake_317670 => 0x0031_7670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc945_fake_19da10 => 0x0019_da10, ee::s03::OVERRIDE_0019DA10, ee::s03::f_0019da10__generated;
        oc945_fake_19dc20 => 0x0019_dc20, ee::s03::OVERRIDE_0019DC20, ee::s03::f_0019dc20__generated;
        oc945_fake_19e238 => 0x0019_e238, ee::s03::OVERRIDE_0019E238, ee::s03::f_0019e238__generated;
        oc945_fake_19e2b0 => 0x0019_e2b0, ee::s03::OVERRIDE_0019E2B0, ee::s03::f_0019e2b0__generated;
        oc945_fake_1a1cd0 => 0x001a_1cd0, ee::s03::OVERRIDE_001A1CD0, ee::s03::f_001a1cd0__generated;
        oc945_fake_1d3ec8 => 0x001d_3ec8, ee::s04::OVERRIDE_001D3EC8, ee::s04::f_001d3ec8__generated;
        oc945_fake_1d3f80 => 0x001d_3f80, ee::s04::OVERRIDE_001D3F80, ee::s04::f_001d3f80__generated;
        oc945_fake_1d47a0 => 0x001d_47a0, ee::s04::OVERRIDE_001D47A0, ee::s04::f_001d47a0__generated;
        oc945_fake_1d4918 => 0x001d_4918, ee::s04::OVERRIDE_001D4918, ee::s04::f_001d4918__generated;
        oc945_fake_1d4f90 => 0x001d_4f90, ee::s04::OVERRIDE_001D4F90, ee::s04::f_001d4f90__generated;
    }

    /// The machine for one seed: scrambled registers, the gate set, no forced
    /// results (every fake returns its own value).
    pub(crate) fn oc945_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc945_install_fakes();
        let r = fakes::machine(c, seed, salt, 0);
        c.bus.write32(OC945_GATE, OC945_MAGIC);
        for k in 0..OC945_CALLS {
            c.bus.write32(OC945_RESULTS + 4 * k, 0);
        }
        r
    }

    /// Forces the result of the `k`-th fake call to `value`.
    pub(crate) fn oc945_force(c: &mut Cpu, k: u32, value: u32) {
        c.bus.write32(OC945_RESULTS + 4 * k, value.wrapping_add(1));
    }

    /// Forces the results of a few random calls to zero.
    pub(crate) fn oc945_zeros(c: &mut Cpu, r: &mut Rng) {
        for k in 0..OC945_CALLS {
            if r.next_u32() % 4 == 0 {
                oc945_force(c, k, 0);
            }
        }
    }

    #[test]
    fn cfe_state_peak_room_on_widget_action_matches() {
        check(
            ee::s04::f_001d3c80__generated,
            cfe_state_peak_room_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x3c80);
                let obj = DATA + 0xa_0000;
                let widget = DATA + 0xa_1000;
                let kinds = [5u64, 1, 6, 0, 2, 7, 3, 4, 5, 1, 6];
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = if seed % 9 == 0 { 0 } else { sx(widget) };
                c.g[A2][0] = kinds[(seed % 11) as usize];
                c.pc = 0x001d_3c80;
                let (p1, p2, p3) = (DATA + 0xa_2000, DATA + 0xa_3000, DATA + 0xa_4000);
                c.bus.write32(obj + 0x10, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2 + 4, p3);
                c.bus.write16(p3 + 0x20, 0x10);
                c.bus.write32(p3 + 0x24, 0x0019_da10);
                c.bus.write16(p3 + 0x28, 0x20);
                c.bus.write32(p3 + 0x2c, 0x0019_dc20);
                c.bus.write32(obj + 0xe0, r.next_u32());
                c.bus.write32(obj + 0xdc, r.next_u32());
                c.bus.write8(obj + 0x44, r.next_u32() as u8);
                c.bus.write32(widget + 0x18, r.next_u32());
                let vt = DATA + 0xa_6000;
                c.bus.write16(vt + 0x48, 8);
                c.bus.write32(vt + 0x4c, 0x0039_e4c0);
                for k in 0..4 {
                    let item = DATA + 0xa_5000 + 0x20 * k;
                    let entry = if (seed >> k) & 1 == 0 { item } else { 0 };
                    c.bus.write32(obj + 0x48 + 4 * k, entry);
                    c.bus.write32(item + 8, vt);
                }
                oc945_zeros(c, &mut r);
            },
        );
    }

    #[test]
    fn cfe_state_peak_room_update_help_text_matches() {
        check(
            ee::s04::f_001d3ec8__generated,
            cfe_state_peak_room_update_help_text,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x3ec8);
                let obj = DATA + 0xa_7000;
                let help = DATA + 0xa_8000;
                let vt = DATA + 0xa_9000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x001d_3ec8;
                c.bus
                    .write32(obj + 0xd0, if seed % 5 == 0 { 0 } else { help });
                c.bus.write32(help + 8, vt);
                c.bus.write16(vt + 0x48, 4);
                c.bus.write32(vt + 0x4c, 0x0039_e4c0);
                c.bus.write8(obj + 0x44, r.next_u32() as u8);
                c.bus.write32(obj + 0xdc, r.next_u32());
                c.bus.write32(obj + 0xe0, r.next_u32());
                oc945_zeros(c, &mut r);
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_ctor_matches() {
        check(
            ee::s04::f_001d4268__generated,
            cfe_state_trophy_room_ctor,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4268);
                let obj = DATA + 0xa_a000;
                let p = DATA + 0xa_b000;
                let text = DATA + 0xa_c000;
                let q = DATA + 0xa_d000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.pc = 0x001d_4268;
                c.bus.write32(obj + 0x10, p);
                c.bus.write32(p, r.next_u32());
                // The string object returned by the BE lookup (call 3).
                c.bus.write32(text + 0xc, q);
                c.bus.write16(q + 0x10, 0x10);
                c.bus.write32(q + 0x14, 0x0019_da10);
                oc945_zeros(c, &mut r);
                oc945_force(c, 3, text);
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_on_activate_matches() {
        check(
            ee::s04::f_001d43d0__generated,
            cfe_state_trophy_room_on_activate,
            200,
            &|c, seed| {
                let _ = oc945_machine(c, seed, 0x43d0);
                c.g[A0][0] = sx(DATA + 0xa_e000);
                c.pc = 0x001d_43d0;
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_show_trophy_image_matches() {
        check(
            ee::s04::f_001d47a0__generated,
            cfe_state_trophy_room_show_trophy_image,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x47a0);
                let obj = DATA + 0xb_0000;
                let item = DATA + 0xb_1000;
                let vt = DATA + 0xb_2000;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = [u64::MAX, 0, 3, 1][(seed % 4) as usize];
                c.pc = 0x001d_47a0;
                c.bus.write8(obj + 0x44, r.next_u32() as u8);
                c.bus.write32(obj + 0x48, item);
                c.bus.write32(item + 8, vt);
                c.bus.write16(vt + 0x48, 8);
                c.bus.write32(vt + 0x4c, 0x0019_dc20);
                c.bus.write32(obj + 0x50, r.next_u32());
                c.bus.write32(obj + 0x54, r.next_u32());
                c.bus.write32(obj + 0x58, r.next_u32());
                c.bus.write32(obj + 0x60, r.next_u32() % 4);
                oc945_zeros(c, &mut r);
                // QueryEventReward (call 0) returns a small slot index.
                oc945_force(c, 0, r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_request_trophy_images_matches() {
        check(
            ee::s04::f_001d4918__generated,
            cfe_state_trophy_room_request_trophy_images,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4918);
                let obj = DATA + 0xb_3000;
                let world = DATA + 0xb_4000;
                let rec = DATA + 0xb_5000;
                c.g[A0][0] = sx(obj);
                c.pc = 0x001d_4918;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, (seed % 4) * 0x10);
                c.bus.write32(obj + 0x54, r.next_u32());
                c.bus.write32(obj + 0x58, r.next_u32() % 6);
                c.bus.write32(obj + 0x50, r.next_u32());
                c.bus
                    .write32(rec + 4, if seed % 3 == 0 { 0 } else { DATA + 0xb_6000 });
                // The record pointer comes back from the table lookups (not
                // zeroed: those results are dereferenced).
                oc945_force(c, 2, rec);
                for k in (4..16).step_by(2) {
                    oc945_force(c, k, rec);
                }
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_load_trophy_textures_matches() {
        check(
            ee::s04::f_001d4a20__generated,
            cfe_state_trophy_room_load_trophy_textures,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4a20);
                let obj = DATA + 0xb_7000;
                let world = DATA + 0xb_8000;
                let world2 = DATA + 0xb_9000;
                let table = DATA + 0xb_a000;
                c.g[A0][0] = sx(obj);
                c.pc = 0x001d_4a20;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x7c, (seed % 3) * 0x10);
                c.bus.write32(GAME_GP - 0x854, world2);
                c.bus.write32(world2 + 0x10d8, table);
                c.bus.write16(table + 0x170, 0x10);
                c.bus.write32(table + 0x174, 0x0019_dc20);
                for i in 0..5 {
                    let entry = if (seed >> i) & 1 == 0 { i } else { u32::MAX };
                    c.bus.write32(obj + 0x74 + 4 * i, entry);
                }
                c.bus.write32(obj + 0x5c, 2);
                c.bus.write32(obj + 0x50, r.next_u32());
                c.bus.write32(obj + 0x9c, r.next_u32());
                oc945_zeros(c, &mut r);
            },
        );
    }

    #[test]
    fn cfe_state_rewards_room_on_widget_created_matches() {
        check(
            ee::s04::f_001d4cd8__generated,
            cfe_state_rewards_room_on_widget_created,
            300,
            &|c, seed| {
                let mut r = oc945_machine(c, seed, 0x4cd8);
                let obj = DATA + 0xb_d000;
                let widget = DATA + 0xb_c000;
                let hash = 0x5a5a_0000 + seed;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(widget);
                c.pc = 0x001d_4cd8;
                c.bus.write32(widget + 0x38, hash);
                oc945_zeros(c, &mut r);
                // The hash of the matching string (call `m`) is `hash`.
                let m = seed % 13;
                if m < 12 {
                    oc945_force(c, m, hash);
                }
            },
        );
    }
}
