//! Staging module oc735 (wave wave241): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc735`.

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
    (
        0x0030_4c50,
        wscript_get_rider_height_above_object,
        "WScriptFn_GetRiderHeightAboveObject",
    ),
    (
        0x0030_4e38,
        wscript_call_object_slot41,
        "WScriptFn_CallObjectSlot41",
    ),
    (0x0030_5660, wscript_screen_flash, "WScriptFn_ScreenFlash"),
    (
        0x0030_59a0,
        world_obj_factory_avalanche,
        "WorldObjFactory_Avalanche",
    ),
    (
        0x0030_5c88,
        wscript_start_avalanche,
        "WScriptFn_StartAvalanche",
    ),
    (
        0x0030_5d90,
        wscript_add_ava_spline_modifier,
        "WScriptFn_AddAvaSplineModifier",
    ),
];

/// `gp` of the game: the world, script-manager and flag globals are gp-relative.
fn wsf_gp(c: &Cpu) -> u32 {
    c.g[GP][0] as u32
}

/// The `memcpy` the builders copy their parameter buffer with.
const WSF_MEMCPY: u32 = 0x003e_6574;

/// The parameter loop of a builder: its cycle charge before the loop, the
/// `V0` it sets first (if any), and the event point of its back edge.
struct WsfFill {
    pre: u64,
    v0: Option<u64>,
    head: u32,
}

const RIDER_FILL: WsfFill = WsfFill {
    pre: 3,
    v0: None,
    head: 0x0030_4cd0,
};
const CALL_SLOT_FILL: WsfFill = WsfFill {
    pre: 4,
    v0: None,
    head: 0x0030_4ed8,
};
const SCREEN_FILL: WsfFill = WsfFill {
    pre: 4,
    v0: None,
    head: 0x0030_56e0,
};
const AVA_FILL: WsfFill = WsfFill {
    pre: 5,
    v0: Some(0x0044_0000),
    head: 0x0030_5b38,
};
const START_AVA_FILL: WsfFill = WsfFill {
    pre: 3,
    v0: None,
    head: 0x0030_5cf0,
};
const ADD_AVA_FILL: WsfFill = WsfFill {
    pre: 4,
    v0: None,
    head: 0x0030_5e10,
};

/// The record loop of the builders: each `(idx, val, type)` record at `A2`
/// (`count` of them, `A1` counts down) stores its value into `buf[idx]` (the
/// stack buffer, a word per index), as a float when the type table `types`
/// asks for one and the record's type does not match. Leaves `A1`, `A2`, `T0`
/// and `A3` the way the generated loop does; nothing runs for `count <= 0`.
fn wsf_fill(c: &mut Cpu, count: u64, buf: u64, types: u64, fill: &WsfFill) {
    if (count as i64) <= 0 {
        return;
    }
    c.cycles += fill.pre;
    if let Some(v) = fill.v0 {
        c.g[V0][0] = v;
    }
    c.g[A1][0] = count;
    c.g[T0][0] = types;
    c.g[A3][0] = 2;
    loop {
        c.cycles += 8;
        let ent = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(ent).lw(c);
        c.g[V1][0] = Addr(ent.wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(buf as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let slot = c.g[A0][0] as u32;
        let mut store = true;
        if c.g[V1][0] == c.g[V0][0] {
            c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
        } else {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A3][0] {
                c.g[V0][0] = Addr(ent.wrapping_add(4)).lw(c);
            } else {
                c.cycles += 4;
                c.lwc1(0, ent.wrapping_add(4));
                c.cvt_s_w(0, 0);
                c.swc1(0, slot);
                store = false;
            }
        }
        if store {
            c.cycles += 1;
            let v0 = c.g[V0][0];
            Addr(slot).sw(c, v0);
        }
        c.cycles += 3;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = c.g[A1][0] != 0;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, fill.head);
        }
    }
}

/// The epilogue's 12-byte copy of the `luno` nil value (source `V1 - 0x6f68`)
/// into the hidden return slot `dst`. Leaves `A3` and `A1` as the generated
/// code does.
fn wsf_nil_copy(c: &mut Cpu, dst: u32) {
    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A3][0] as u32;
    c.ldl(4, src.wrapping_add(7));
    c.ldr(4, src);
    c.g[A1][0] = Addr(src.wrapping_add(8)).lw(c);
    c.sdl(4, dst.wrapping_add(7));
    c.sdr(4, dst);
    let a1 = c.g[A1][0];
    Addr(dst.wrapping_add(8)).sw(c, a1);
}

/// The section lookup of the key in `A2` (the table pointer is already in
/// `A1`): record `key & 0xff` of the table, then entry `key >> 8` of the
/// record's list. Leaves the entry, scaled to a pointer-sized offset, in `V0`
/// (0 when there is none). Charges the cycles of the generated `'b_..._78`
/// block: 7 when a record is found, 1 more when the entry is zero or missing.
fn wsf_section_chain(c: &mut Cpu) {
    c.g[V0][0] = c.g[A2][0] & 0xff;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    let found = c.g[V1][0] != 0;
    c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
    if !found {
        c.cycles += 1;
        c.g[V0][0] = 0;
        return;
    }
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
    let nonzero = c.g[V0][0] != 0;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    if !nonzero {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
}

/// `WScriptFn_GetRiderHeightAboveObject` (0x304c50), the C++ `RiderHeightFunc`:
/// builds the rider-height parameter record from the script's parameter list
/// (the template copy with the one-time init) and returns the rider's height
/// above the section's object, in the first argument's slot. Saves `s0`-`s3`
/// and `f20`.
fn rider_height_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x90);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2740)).lw(c);
    frame.save(c, S1, 0x60);
    frame.save(c, S2, 0x50);
    frame.save(c, S3, 0x40);
    frame.save(c, S0, 0x70);
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = c.g[A2][0];
    frame.save_ra(c, 0x30);
    let init = c.g[V0][0] == 0;
    frame.save_fpr(c, 20, 0x80);

    if init {
        c.cycles += 7;
        c.g[V0][0] = Addr(gp.wrapping_add(0x273c)).lbu(c);
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 1;
        let v1 = c.g[V1][0];
        Addr(gp.wrapping_add(0x2738)).sw(c, v1);
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2740)).sw(c, a0);
        let v0 = c.g[V0][0];
        Addr(sp(c)).sb(c, v0);
    }

    // The parameter buffer: the record's id word, then the template copy.
    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(gp.wrapping_add(0x2738));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_4cbc)?;

    c.g[A2][0] = c.g[S3][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xc88));
    wsf_fill(c, count, buf, types, &RIDER_FILL);

    // The key (buf[0]) and its section record; the -1 key takes the script
    // manager's section.
    c.cycles += 5;
    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V1][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    let not_neg1 = c.g[V0][0] != 0;
    c.g[A3][0] = Addr(c.g[V1][0] as u32).lw(c);
    if not_neg1 {
        c.cycles += 9;
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        wsf_section_chain(c);
        c.cycles += 1;
        c.g[S0][0] = c.g[V0][0];
    } else {
        c.cycles += 2;
        c.g[S0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x290)).lw(c);
    }

    // The instance of the section: its rider list entry gives the height
    // callee's result (the `f20` the epilogue returns through the frame).
    c.cycles += 2;
    let s0_zero = c.g[S0][0] == 0;
    c.g[A0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    if s0_zero {
        c.cycles += 5;
        c.g[V0][0] = 2;
        c.g[A0][0] = c.g[S2][0];
        let v0 = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, v0);
        call_ptr(c, 0x0022_6618, 0x0030_4e10)?;
        let v0 = c.g[V0][0];
        Addr(v0 as u32).sw(c, 0);
    } else {
        c.cycles += 12;
        c.g[A1][0] = sx((c.g[A3][0] as u32) << 2);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        c.g[A1][0] = 0;
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        if c.g[A0][0] == 0 {
            c.g[V1][0] = 0;
        }
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c0));
        if c.g[V1][0] != 0 {
            c.g[A1][0] = c.g[V0][0];
        }
        if c.g[A1][0] != 0 {
            c.cycles += 12;
            c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x6c0)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x2c)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0030_4dcc)?;
            c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x40));
            let v0 = c.g[V0][0] as u32;
            c.lwc1(1, v0.wrapping_add(0x8));
            c.sq(3, sp(c).wrapping_add(0x20));
            c.lwc1(0, sp(c).wrapping_add(0x28));
            c.sub_s(20, 1, 0);
        } else {
            c.cycles += 1;
            c.mtc1(0, 20);
        }
        c.cycles += 6;
        c.g[V0][0] = 2;
        c.g[A0][0] = c.g[S2][0];
        let v0 = c.g[V0][0];
        Addr((c.g[S2][0] as u32).wrapping_add(8)).sw(c, v0);
        call_ptr(c, 0x0022_6618, 0x0030_4df8)?;
        let v0 = c.g[V0][0] as u32;
        c.swc1(20, v0);
    }

    c.cycles += 9;
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore_ra(c, 0x30);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_GetRiderHeightAboveObject` (0x304c50): see [`rider_height_body`].
pub fn wscript_get_rider_height_above_object(c: &mut Cpu) {
    guest(c, rider_height_body);
}

/// `WScriptFn_CallObjectSlot41` (0x304e38): the object's slot-41 call with the
/// script's parameters. Builds the record from the template copy (with the
/// one-time init), runs the parameter loop, looks the section up and calls its
/// object. Returns its first argument; saves `s0`-`s4`.
fn call_slot41_body(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    let frame = Frame::open(c, 0x80);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2744)).lw(c);
    frame.save(c, S3, 0x40);
    frame.save(c, S0, 0x70);
    frame.save(c, S2, 0x50);
    frame.save(c, S1, 0x60);
    frame.save(c, S4, 0x30);
    c.g[S3][0] = c.g[A2][0];
    c.g[S0][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A0][0];
    c.g[A2][0] = 0x0050_0000;
    frame.save_ra(c, 0x20);
    let init = c.g[V0][0] == 0;

    if init {
        c.cycles += 12;
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42e0));
        c.g[V0][0] = u64::MAX;
        c.g[A1][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lbu(c);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = 1;
        let v0 = c.g[V0][0];
        Addr((c.g[A2][0] as u32).wrapping_sub(0x42e0)).sw(c, v0);
        let a1 = c.g[A1][0];
        Addr((c.g[V1][0] as u32).wrapping_add(8)).sb(c, a1);
        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(4));
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2744)).sw(c, a0);
        Addr(sp(c)).sb(c, a1);
    }

    // The parameter record: copied from the template at 0x4fbd20.
    c.cycles += 11;
    c.g[V0][0] = u64::MAX;
    c.g[S1][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x42e0));
    c.g[A2][0] = 0xc;
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    c.g[A0][0] = c.g[S1][0];
    c.g[S4][0] = 0x004d_0000;
    call_ptr(c, WSF_MEMCPY, 0x0030_4ec0)?;

    c.g[A2][0] = c.g[S3][0];
    let count = c.g[S0][0];
    let buf = c.g[S1][0];
    let types = sx(gp.wrapping_add(0xc90));
    wsf_fill(c, count, buf, types, &CALL_SLOT_FILL);

    // The key (the record's id word) and its section.
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    let not_neg1 = c.g[V0][0] != 0;
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if not_neg1 {
        c.cycles += 8;
        wsf_section_chain(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
    }

    // The section's object: its call with the parameter record.
    c.cycles += 3;
    c.g[S0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
    if c.g[S0][0] != 0 {
        c.cycles += 7;
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_4f9c)?;
        let v0_zero = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !v0_zero {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0035_5888, 0x0030_4fac)?;
        }
    }

    // The epilogue copies the nil value through the saved `S2` slot.
    c.cycles += 16;
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x6f68));
    let src = c.g[A1][0] as u32;
    c.ldl(2, src.wrapping_add(7));
    c.ldr(2, src);
    c.g[V1][0] = Addr(src.wrapping_add(8)).lw(c);
    let dst = c.g[S2][0] as u32;
    c.sdl(2, dst.wrapping_add(7));
    c.sdr(2, dst);
    let v1 = c.g[V1][0];
    Addr(dst.wrapping_add(8)).sw(c, v1);
    c.g[V0][0] = c.g[S2][0];
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_CallObjectSlot41` (0x304e38): see [`call_slot41_body`].
pub fn wscript_call_object_slot41(c: &mut Cpu) {
    guest(c, call_slot41_body);
}

/// `WScriptFn_ScreenFlash` (0x305660): the screen-flash parameter record
/// (template copy, one-time init) and the flash callee on the section's object.
fn screen_flash_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2760)).lw(c);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    frame.save(c, S3, 0x30);
    frame.save(c, S0, 0x60);
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A2][0];
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);

    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(gp.wrapping_add(0x275c)).lbu(c);
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 1;
        let v1 = c.g[V1][0];
        Addr(gp.wrapping_add(0x2758)).sw(c, v1);
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2760)).sw(c, a0);
        let v0 = c.g[V0][0];
        Addr(sp(c)).sb(c, v0);
    }

    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(gp.wrapping_add(0x2758));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_56c8)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xca0));
    wsf_fill(c, count, buf, types, &SCREEN_FILL);

    // The flash callee of the section's object (none for the -1 section).
    c.cycles += 5;
    c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
    c.g[V1][0] = u64::MAX;
    c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
    let is_neg1 = c.g[V0][0] == c.g[V1][0];
    c.g[A0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    let mut tail = true;
    if !is_neg1 {
        c.cycles += 14;
        c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c0));
        if c.g[V1][0] == 0 {
            c.g[A0][0] = 0;
        }
        c.g[A1][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x40)).lh(c);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x44)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, target, 0x0030_576c)?;
        let v0_zero = c.g[V0][0] == 0;
        c.g[V1][0] = 0x004d_0000;
        if v0_zero {
            tail = false;
        } else {
            c.cycles += 3;
            c.mtc1(0, 12);
            call_ptr(c, 0x0039_0ec8, 0x0030_5780)?;
        }
    }
    if tail {
        c.cycles += 1;
    }
    c.g[V1][0] = 0x004d_0000;

    c.cycles += 15;
    c.g[V0][0] = c.g[S3][0];
    let dst = c.g[S3][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_ScreenFlash` (0x305660): see [`screen_flash_body`].
pub fn wscript_screen_flash(c: &mut Cpu) {
    guest(c, screen_flash_body);
}

/// `WorldObjFactory_Avalanche` (0x3059a0): builds the avalanche object (the
/// one-time init of its template, the parameter copy, the parameter loop), then
/// walks the eight slots of its node list and calls each live one's update.
fn avalanche_factory_body(c: &mut Cpu) -> Flow {
    c.cycles += 17;
    let frame = Frame::open(c, 0x190);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2768)).lw(c);
    frame.save(c, S1, 0x170);
    frame.save(c, S2, 0x160);
    frame.save(c, S0, 0x180);
    frame.save(c, S3, 0x150);
    frame.save(c, S4, 0x140);
    frame.save(c, S5, 0x130);
    frame.save(c, S6, 0x120);
    frame.save(c, S7, 0x110);
    frame.save(c, FP, 0x100);
    c.g[S1][0] = c.g[A1][0];
    c.g[S2][0] = c.g[A2][0];
    c.g[A1][0] = 0x0050_0000;
    frame.save_ra(c, 0xf0);
    let init = c.g[V0][0] == 0;
    let a0 = c.g[A0][0];
    Addr(sp(c).wrapping_add(0xe0)).sw(c, a0);

    if init {
        avalanche_init(c);
    }

    // The parameter copy (200 bytes from the template) and its loop.
    c.cycles += 17;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 0xc8;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4270));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x30)).sw(c, v0);
    c.g[A0][0] = c.g[S0][0];
    for off in [0x14u32, 0x18, 0x1c, 0x20, 0x24, 0x28] {
        Addr(sp(c).wrapping_add(off)).sw(c, v0);
    }
    Addr(sp(c).wrapping_add(0x2c)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_5b1c)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    wsf_fill(c, count, buf, sx(0x0044_66d8), &AVA_FILL);

    // The node walk: eight slots of the copied list, each with its update.
    c.cycles += 10;
    c.g[S6][0] = u64::MAX;
    c.g[FP][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[S5][0] = c.g[S0][0] | 0x4;
    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa4));
    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x84));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x44));
    c.g[S7][0] = 7;
    loop {
        c.cycles += 3;
        c.g[A2][0] = Addr(c.g[S5][0] as u32).lw(c);
        let skip = c.g[A2][0] == c.g[S6][0];
        c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
        if !skip {
            c.cycles += 8;
            c.g[A0][0] = c.g[A2][0] & 0xff;
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let a0_zero = c.g[A0][0] == 0;
            c.g[A1][0] = 0;
            if !a0_zero {
                c.cycles += 8;
                c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
                c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                }
            }
            c.cycles += 7;
            let s0 = c.g[S0][0] as u32;
            c.lwc1(12, s0);
            c.g[A0][0] = c.g[FP][0];
            let s1 = c.g[S1][0] as u32;
            c.lwc1(13, s1);
            let s2 = c.g[S2][0] as u32;
            c.lwc1(14, s2);
            let s3 = c.g[S3][0] as u32;
            c.lwc1(15, s3);
            c.g[A2][0] = Addr(c.g[S4][0] as u32).lw(c);
            call_ptr(c, 0x002d_92d0, 0x0030_5c10)?;
        }
        c.cycles += 8;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(4));
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(4));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
        let more = (c.g[S7][0] as i64) >= 0;
        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0030_5ba8);
        }
    }

    c.cycles += 21;
    c.g[V0][0] = Addr(sp(c).wrapping_add(0xe0)).lw(c);
    c.g[V1][0] = 0x004d_0000;
    frame.restore(c, S0, 0x180);
    let dst = c.g[V0][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S1, 0x170);
    frame.restore(c, S2, 0x160);
    frame.restore(c, S3, 0x150);
    frame.restore(c, S4, 0x140);
    frame.restore(c, S5, 0x130);
    frame.restore(c, S6, 0x120);
    frame.restore(c, S7, 0x110);
    frame.restore(c, FP, 0x100);
    frame.restore_ra(c, 0xf0);
    frame.close(c);
    Ok(())
}

/// The one-time init of the avalanche template (the flag clear): the float
/// fields (10.0f, 3.0f, 1.0f, 0.0f) and the node slots, then the flag.
fn avalanche_init(c: &mut Cpu) {
    let gp = wsf_gp(c);
    c.cycles += 63;
    c.g[AT][0] = 0x4120_0000;
    c.mtc1(1, 0);
    c.g[V1][0] = 0xffff_ffff_ffff_0000;
    c.mtc1(0, 1);
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4270));
    c.g[V1][0] |= 0xffff;
    let base = c.g[V0][0] as u32;
    c.swc1(0, base.wrapping_add(0x60));
    let v1 = c.g[V1][0];
    Addr(base.wrapping_add(0x20)).sw(c, v1);
    c.g[A0][0] = 1;
    c.swc1(1, base.wrapping_add(0x80));
    Addr((c.g[A1][0] as u32).wrapping_sub(0x4270)).sw(c, 0);
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(1, 2);
    for off in [0x44u32, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c] {
        c.swc1(0, base.wrapping_add(off));
    }
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    for off in [0x4u32, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
        Addr(base.wrapping_add(off)).sw(c, v1);
    }
    for off in [0x24u32, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40] {
        Addr(base.wrapping_add(off)).sw(c, 0);
    }
    for off in [0x64u32, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c] {
        c.swc1(1, base.wrapping_add(off));
    }
    c.swc1(2, base.wrapping_add(0x84));
    c.g[V1][0] = Addr(base.wrapping_add(0xc4)).lbu(c);
    c.swc1(2, base.wrapping_add(0xa0));
    c.swc1(0, base.wrapping_add(0xc0));
    let a0 = c.g[A0][0];
    Addr(gp.wrapping_add(0x2768)).sw(c, a0);
    for off in [0x88u32, 0x8c, 0x90, 0x94, 0x98, 0x9c] {
        c.swc1(2, base.wrapping_add(off));
    }
    for off in [0xa4u32, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xbc] {
        c.swc1(0, base.wrapping_add(off));
    }
    let v1 = c.g[V1][0];
    Addr(sp(c)).sb(c, v1);
}

/// `WorldObjFactory_Avalanche` (0x3059a0): see [`avalanche_factory_body`].
pub fn world_obj_factory_avalanche(c: &mut Cpu) {
    guest(c, avalanche_factory_body);
}

/// `WScriptFn_StartAvalanche` (0x305c88): sets the avalanche's start record
/// (one-time init, the parameter copy and loop), then calls the start callee
/// of the section's object unless the global at `gp+0x9d4` is set.
fn start_avalanche_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2778)).lw(c);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x60);
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);

    if c.g[V0][0] == 0 {
        c.cycles += 5;
        c.g[V1][0] = Addr(gp.wrapping_add(0x2774)).lbu(c);
        c.g[V0][0] = 1;
        let v0 = c.g[V0][0];
        Addr(gp.wrapping_add(0x2778)).sw(c, v0);
        Addr(gp.wrapping_add(0x2770)).sw(c, 0);
        let v1 = c.g[V1][0];
        Addr(sp(c)).sb(c, v1);
    }

    c.cycles += 7;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A1][0] = sx(gp.wrapping_add(0x2770));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, WSF_MEMCPY, 0x0030_5cdc)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xca8));
    wsf_fill(c, count, buf, types, &START_AVA_FILL);

    c.cycles += 3;
    c.g[V0][0] = Addr(gp.wrapping_add(0x9d4)).lw(c);
    let set = c.g[V0][0] != 0;
    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
    if !set {
        c.cycles += 2;
        call_ptr(c, 0x002d_97a8, 0x0030_5d4c)?;
    }

    c.cycles += 16;
    c.g[V1][0] = 0x004d_0000;
    c.g[V0][0] = c.g[S3][0];
    let dst = c.g[S3][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `WScriptFn_StartAvalanche` (0x305c88): see [`start_avalanche_body`].
pub fn wscript_start_avalanche(c: &mut Cpu) {
    guest(c, start_avalanche_body);
}

/// `WScriptFn_AddAvaSplineModifier` (0x305d90): adds a spline modifier to the
/// avalanche of the section named by the first parameter: the record copy, the
/// parameter loop, the section lookup, then the node's update callee (when the
/// node is live) and the node-add callee.
fn add_ava_spline_body(c: &mut Cpu) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    let gp = wsf_gp(c);
    c.g[V0][0] = Addr(gp.wrapping_add(0x2788)).lw(c);
    frame.save(c, S1, 0x50);
    frame.save(c, S2, 0x40);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S3, 0x30);
    c.g[S2][0] = c.g[A2][0];
    frame.save(c, S0, 0x60);
    c.g[S3][0] = c.g[A0][0];
    frame.save_ra(c, 0x20);

    if c.g[V0][0] == 0 {
        c.cycles += 7;
        c.g[V0][0] = Addr(gp.wrapping_add(0x2784)).lbu(c);
        c.g[V1][0] = u64::MAX;
        c.g[A0][0] = 1;
        let v1 = c.g[V1][0];
        Addr(gp.wrapping_add(0x2780)).sw(c, v1);
        let a0 = c.g[A0][0];
        Addr(gp.wrapping_add(0x2788)).sw(c, a0);
        let v0 = c.g[V0][0];
        Addr(sp(c)).sb(c, v0);
    }

    c.cycles += 10;
    c.g[V0][0] = u64::MAX;
    c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A2][0] = 8;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(gp.wrapping_add(0x2780));
    let v0 = c.g[V0][0];
    Addr(sp(c).wrapping_add(0x10)).sw(c, v0);
    call_ptr(c, WSF_MEMCPY, 0x0030_5df8)?;

    c.g[A2][0] = c.g[S2][0];
    let count = c.g[S1][0];
    let buf = c.g[S0][0];
    let types = sx(gp.wrapping_add(0xcb0));
    wsf_fill(c, count, buf, types, &ADD_AVA_FILL);

    // The section lookup of the key (buf[0]): the table pointer, then the
    // record and its entry; S1 gets the entry (0 when there is none).
    c.cycles += 4;
    c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[V0][0] = !c.g[A2][0];
    let not_neg1 = c.g[V0][0] != 0;
    c.g[A1][0] = Addr(gp.wrapping_add(0x16c8)).lw(c);
    if not_neg1 {
        c.cycles += 8;
        c.g[V0][0] = c.g[A2][0] & 0xff;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[V1][0] != 0 {
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[A2][0] as u32) >> 8);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            if !nonzero {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 1;
            c.g[A1][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            c.g[A1][0] = 0;
        }
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(gp.wrapping_add(0xce8)).lw(c);
        c.g[S1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x290)).lw(c);
        // The -1 key takes the script manager's section and skips the tail.
        return add_ava_finish(c, frame);
    }
    c.cycles += 1;
    c.g[S1][0] = c.g[A1][0];
    add_ava_finish(c, frame)
}

/// The node walk and the epilogue of `WScriptFn_AddAvaSplineModifier`: the
/// node's callees when the section has a record, then the nil copy.
fn add_ava_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let s1_zero = c.g[S1][0] == 0;
    c.g[V1][0] = 0x004d_0000;
    if s1_zero {
        c.cycles += 1;
        c.g[V0][0] = c.g[S3][0];
    } else {
        c.cycles += 3;
        c.g[S0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
        let s0_zero = c.g[S0][0] == 0;
        c.g[V0][0] = c.g[S3][0];
        if !s0_zero {
            ada_node_callees(c)?;
            c.cycles += 1;
            c.g[V1][0] = 0x004d_0000;
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
        }
    }

    c.cycles += 14;
    let dst = c.g[S3][0] as u32;
    wsf_nil_copy(c, dst);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// The node's two callees of `AddAvaSplineModifier`: the node's update (its
/// pointer at `+0x84` of the node's `+0xc` record), then the node-add on the
/// section's list (`S1`) when the update answers non-zero.
fn ada_node_callees(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x80)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0030_5eec)?;
    let zero = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S0][0];
    if !zero {
        c.cycles += 2;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_5a78, 0x0030_5efc)?;
    }
    Ok(())
}

/// `WScriptFn_AddAvaSplineModifier` (0x305d90): see [`add_ava_spline_body`].
pub fn wscript_add_ava_spline_modifier(c: &mut Cpu) {
    guest(c, add_ava_spline_body);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The world the builders' lookups read: `gp+0x16c8` -> table -> records
    /// (`key & 0xff`) -> entries (`key >> 8`), the world at `gp-0x848` with its
    /// section table and the script manager's section index (`gp+0xce8`), the
    /// per-section instance and its callee (the leaf at `0x1406c0`, returns 0).
    fn wsf_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.g[GP][0] = sx(GAME_GP);
        c.bus.write32(GAME_GP + 0x16c8, DATA + 0x3000);
        c.bus.write32(DATA + 0x3000, DATA + 0x4000);
        c.bus.write32(DATA + 0x4008, DATA + 0x5000);
        for j in 0..4u32 {
            let sec = DATA + 0x6000 + 0x100 * j;
            c.bus.write32(
                DATA + 0x5000 + 4 * j,
                if (seed + j) % 5 == 0 { 0 } else { sec },
            );
            c.bus.write32(sec + 0x1c, DATA + 0x7000 + 0x100 * j);
            for m in 0..4u32 {
                let e = if (seed + m + j) % 3 == 0 {
                    0
                } else {
                    (1 + r.next_u32() % 255) << 8
                };
                c.bus.write32(DATA + 0x7000 + 0x100 * j + 4 * m, e);
            }
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x3800);
        c.bus.write32(DATA + 0x3800 + 0x84, DATA + 0x3900);
        c.bus.write32(DATA + 0x3900 + 0xc, DATA + 0x3a00);
        c.bus.write32(GAME_GP + 0xce8, DATA + 0x3b00);
        c.bus.write32(DATA + 0x3b00, seed % 4);
        c.bus.write32(
            DATA + 0x3b00 + 0x290,
            if seed % 2 == 0 { 0 } else { DATA + 0x3c00 },
        );
        for k in 0..4u32 {
            let inst = if (seed / 4 + k) % 3 == 0 {
                0
            } else {
                DATA + 0x3c00
            };
            c.bus.write32(DATA + 0x3a00 + 4 * k + 0x28, inst);
        }
        c.bus.write32(DATA + 0x3c00 + 0x6c0, DATA + 0x3d00);
        c.bus.write32(DATA + 0x3d00 + 0x2c, 0x0014_06c0);
        c.bus.write32(DATA + 0x3d00 + 0x44, 0x0014_06c0);
    }

    /// `count` parameter records at `DATA+0x9100` and the type table at `types`.
    fn wsf_records(c: &mut Cpu, r: &mut Rng, seed: u32, types: u32) {
        for i in 0..4u32 {
            c.bus.write32(types + 4 * i, r.next_u32() % 3);
        }
        for i in 0..(seed % 4) {
            let e = DATA + 0x9100 + 16 * i;
            c.bus.write32(e, i % 4);
            c.bus.write32(e + 4, r.next_u32());
            c.bus.write32(e + 0xc, r.next_u32() % 3);
        }
    }

    #[test]
    fn wscript_get_rider_height_above_object_matches() {
        check(
            ee::s10::f_00304c50__generated,
            wscript_get_rider_height_above_object,
            300,
            &|c, seed| {
                let mut r = Rng(0x304c_50 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2740, seed % 2);
                c.bus.write32(GAME_GP + 0x2738, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xc88);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_call_object_slot41_matches() {
        check(
            ee::s10::f_00304e38__generated,
            wscript_call_object_slot41,
            300,
            &|c, seed| {
                let mut r = Rng(0x304e_38 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2744, seed % 2);
                c.bus.write32(0x004f_bd20, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xc90);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_screen_flash_matches() {
        check(
            ee::s10::f_00305660__generated,
            wscript_screen_flash,
            300,
            &|c, seed| {
                let mut r = Rng(0x3056_60 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                c.bus.write32(
                    DATA + 0x3b00,
                    if seed % 5 == 4 { u32::MAX } else { seed % 4 },
                );
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2760, seed % 2);
                c.bus.write32(GAME_GP + 0x2758, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xca0);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn world_obj_factory_avalanche_matches() {
        check(
            ee::s10::f_003059a0__generated,
            world_obj_factory_avalanche,
            300,
            &|c, seed| {
                let mut r = Rng(0x3059_a0 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                c.bus.write32(GAME_GP + 0x2768, seed % 2);
                // Every node slot of the copied list is empty (-1), so the walk
                // never reaches the live-node callee at 0x2d92d0 (its real body
                // runs away on zeroed test data). Count 0: no parameter records.
                for i in 0..50u32 {
                    c.bus.write32(0x004f_bd90 + 4 * i, u32::MAX);
                }
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = 0;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_start_avalanche_matches() {
        check(
            ee::s10::f_00305c88__generated,
            wscript_start_avalanche,
            300,
            &|c, seed| {
                let mut r = Rng(0x305c_88 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                c.bus.write32(GAME_GP + 0x2778, seed % 2);
                c.bus.write32(GAME_GP + 0x2770, r.next_u32());
                c.bus.write32(GAME_GP + 0x9d4, 1);
                wsf_records(c, &mut r, seed, GAME_GP + 0xca8);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }

    #[test]
    fn wscript_add_ava_spline_modifier_matches() {
        check(
            ee::s10::f_00305d90__generated,
            wscript_add_ava_spline_modifier,
            300,
            &|c, seed| {
                let mut r = Rng(0x305d_90 ^ seed as u64);
                scramble(c, &mut r);
                wsf_world(c, &mut r, seed);
                let key = (((seed / 2) % 4) << 8) | ((seed / 8) % 4);
                c.bus.write32(GAME_GP + 0x2788, seed % 2);
                c.bus.write32(GAME_GP + 0x2780, key);
                wsf_records(c, &mut r, seed, GAME_GP + 0xcb0);
                c.g[A0][0] = sx(DATA + 0x9000);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x9100);
            },
        );
    }
}
