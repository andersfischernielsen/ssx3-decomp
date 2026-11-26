//! Staging module oc841 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc841`.

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
    0x0018_e510,
    oc841_device_select_display_status,
    "cFEStateOptionsDeviceSelect_v26",
)];

/// Virtual call `obj->vtable[slot]()` the way gcc 2.9x emits it: `a0 =
/// lh(vt + doff)` (the `this` delta), `fr = lw(vt + foff)` (the method), `ra`,
/// `a0 = obj + a0`, `jalr`. `obj` and `vt` are register numbers.
#[allow(clippy::too_many_arguments)]
fn oc841_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    doff: u32,
    fr: usize,
    foff: u32,
    ret: u32,
) -> Flow {
    c.g[A0][0] = c.lh((c.g[vt][0] as u32).wrapping_add(doff));
    c.g[fr][0] = c.lw((c.g[vt][0] as u32).wrapping_add(foff));
    let t = c.g[fr][0] as u32;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `0x460000 - off`: the address of a string literal (`lui` + `addiu`).
fn oc841_str(off: u32) -> u64 {
    sx(0x0046_0000u32.wrapping_sub(off))
}

/// Shared tail of the per-slot loop: call the text's method (`v1`, `this =
/// s0 + a0`) and count the slot (`s4 += 1`).
fn oc841_slot_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x0018_e86c);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0018_e86c)?;
    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
    Ok(())
}

/// The slot is neither the save target nor a blank one: grey it out or leave it
/// live depending on the device (the `DEVI 54` / `66` branches).
fn oc841_slot_device_flags(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    oc841_vcall(c, A2, V1, 0x1b0, V0, 0x1b4, 0x0018_e7f4)?;
    let t = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[S3][0];
    if t {
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e88c);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e88c)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e8a4);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e8a4)?;
        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
        return Ok(());
    }
    'b_85c: {
        c.cycles += 8;
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        oc841_vcall(c, A2, V1, 0x210, V0, 0x214, 0x0018_e814)?;
        let t = c.g[V0][0] == 0;
        c.g[A1][0] = 1;
        if !t {
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x0018_e830);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e830)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[A1][0] = 1;
            break 'b_85c;
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e854);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e854)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 0;
    }
    c.cycles += 2;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
    oc841_slot_tail(c)
}

/// One slot's text object (`s0`) is set up: the device texts, then static /
/// static-colour flags. Ends with `s4 += 1` on every path.
fn oc841_slot_flags(c: &mut Cpu) -> Flow {
    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
    c.g[A1][0] = c.g[S3][0];
    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
    oc841_vcall(c, A2, V1, 0x218, V0, 0x21c, 0x0018_e6e8)?;
    let mut to_static = c.g[V0][0] != 0;
    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
    if !to_static {
        c.cycles += 8;
        c.g[A1][0] = c.g[S3][0];
        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
        oc841_vcall(c, A2, V1, 0x210, V0, 0x214, 0x0018_e708)?;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
            to_static = true;
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            oc841_vcall(c, A1, V1, 0x1b8, V0, 0x1bc, 0x0018_e728)?;
            if c.g[V0][0] == 0 {
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                to_static = true;
            }
        }
    }
    if to_static {
        return oc841_slot_device_flags(c);
    }
    // Blank slot of a writable card: type 1 (all slots) greys it, otherwise
    // only the first one stays live.
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
    let t = c.g[V0][0] != c.g[S7][0];
    c.g[A1][0] = 1;
    if !t {
        c.cycles += 10;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018_e750);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_e750)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
        return oc841_slot_tail(c);
    }
    c.cycles += 2;
    if c.g[FP][0] != 0 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x22c));
        let t = c.g[V0][0] == 0;
        c.g[A1][0] = 0;
        if !t {
            c.cycles += 14;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[FP][0] = 1;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0018_e790);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e790)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            c.g[A1][0] = 0;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0018_e7a8);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e7a8)?;
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
            return Ok(());
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
    }
    c.cycles += 10;
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x0018_e7c8);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, 0x0018_e7c8)?;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
    c.g[A1][0] = 1;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
    oc841_slot_tail(c)
}

/// An arrow text: `FindObject(screen, StrHash(name)).setVisible(vis)`
/// (virtual `+0x48`); `rets` are the three return addresses.
fn oc841_arrow(c: &mut Cpu, name_off: u32, rets: [u32; 3], vis: u64) -> Flow {
    c.g[A0][0] = oc841_str(name_off);
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, rets[1])?;
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(8));
    c.g[A1][0] = vis;
    oc841_vcall(c, V0, V1, 0x48, A2, 0x4c, rets[2])
}

/// `cFEStateOptionsDeviceSelect::displayStatus(self)` (0x18e510): the memory
/// card device picker. Keeps the screen state in step with the card (no cards:
/// the no-cards callback; otherwise state 3), then fills the device name text,
/// the four "memory card N" slot texts (live or greyed per slot) and the two
/// arrows; finally greys the slots again unless the screen state is 3.
pub fn oc841_device_select_display_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x770);
        frame.save(c, S2, 0x740);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x750);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x760);
        frame.save(c, S3, 0x730);
        frame.save(c, S4, 0x720);
        frame.save(c, S5, 0x710);
        frame.save(c, S6, 0x700);
        frame.save(c, S7, 0x6f0);
        frame.save(c, FP, 0x6e0);
        frame.save_ra(c, 0x6d0);
        call_ptr(c, 0x0022_7f80, 0x0018_e54c)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
        c.g[S1][0] = c.g[V0][0];
        oc841_vcall(c, S2, V1, 0x150, V0, 0x154, 0x0018_e564)?;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
        } else {
            c.cycles += 8;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
            c.g[V1][0] = c.lw(c.g[A1][0] as u32);
            oc841_vcall(c, A1, V1, 0x1a8, V0, 0x1ac, 0x0018_e584)?;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
                c.cycles += 3;
                c.g[V0][0] = 3;
                let same = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = 3;
                if !same {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
                    oc841_vcall(c, S2, V0, 0x118, V1, 0x11c, 0x0018_e5bc)?;
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
            } else {
                c.cycles += 4;
                call_ptr(c, 0x001d_8240, 0x0018_e594)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
            }
        }
        // Second displayFileOK: the real work.
        c.cycles += 6;
        oc841_vcall(c, S2, V1, 0x150, V0, 0x154, 0x0018_e5d0)?;
        let skip = c.g[V0][0] == 0;
        c.g[FP][0] = 0;
        if skip {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
        } else {
            c.cycles += 40;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(8));
            c.g[S4][0] = 1;
            c.g[S7][0] = 1;
            c.g[S3][0] = 0;
            oc841_vcall(c, S2, V0, 0x158, V1, 0x15c, 0x0018_e5f8)?;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0x320;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0024_1710, 0x0018_e60c)?;
            c.g[A0][0] = oc841_str(0x1e18);
            call_ptr(c, 0x0031_7670, 0x0018_e618)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e624)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0d00, 0x0018_e630)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
            c.g[A0][0] = oc841_str(0x1e28);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x21c), c.g[V0][0]);
            call_ptr(c, 0x0031_7670, 0x0018_e644)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e650)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
            oc841_vcall(c, S0, V0, 0x48, V1, 0x4c, 0x0018_e66c)?;
            c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x690));
            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
            c.g[V0][0] = 0x0046_0000;
            // The four "memory card N" slot texts.
            'l_18e678: loop {
                c.cycles += 30;
                c.g[A2][0] = c.g[S4][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1df0));
                c.g[A0][0] = c.g[S6][0];
                call_ptr(c, 0x0041_7828, 0x0018_e688)?;
                c.g[A0][0] = c.g[S6][0];
                call_ptr(c, 0x0031_7670, 0x0018_e690)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_e69c)?;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S5][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                oc841_vcall(c, A3, V0, 0x70, V1, 0x74, 0x0018_e6c0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x003a_0e90, 0x0018_e6cc)?;
                oc841_slot_flags(c)?;
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S4][0] as i64) < 5) as u64;
                let done = c.g[V0][0] == 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                    let again = c.g[V0][0] == c.g[S7][0];
                    c.g[V0][0] = 0x0046_0000;
                    if again {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018_e678);
                        }
                        continue 'l_18e678;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 2) as u64;
                    let again = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x0046_0000;
                    if again {
                        if event_due(c) {
                            deliver_event_here(c, 0x0018_e678);
                        }
                        continue 'l_18e678;
                    }
                }
                break;
            }
            // Left arrow: shown when a next device exists.
            c.cycles += 8;
            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 1;
            call_ptr(c, 0x001d_97e0, 0x0018_e8e0)?;
            c.g[V1][0] = u64::MAX;
            let found = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0x0046_0000;
            if found {
                c.cycles += 12;
                oc841_arrow(c, 0x1de0, [0x0018_e928, 0x0018_e934, 0x0018_e94c], 1)?;
            } else {
                c.cycles += 13;
                oc841_arrow(c, 0x1de0, [0x0018_e8f4, 0x0018_e900, 0x0018_e918], 0)?;
            }
            c.g[A1][0] = c.g[S1][0];
            // Right arrow.
            c.cycles += 7;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
            c.g[A3][0] = 0;
            call_ptr(c, 0x001d_97e0, 0x0018_e960)?;
            c.g[V1][0] = u64::MAX;
            let found = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0x0046_0000;
            if found {
                c.cycles += 11;
                oc841_arrow(c, 0x1dd0, [0x0018_e9a8, 0x0018_e9b4, 0x0018_e9cc], 1)?;
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
            } else {
                c.cycles += 13;
                oc841_arrow(c, 0x1dd0, [0x0018_e974, 0x0018_e980, 0x0018_e998], 0)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
            }
        }
        // Not in state 3: grey every slot text again.
        c.cycles += 3;
        c.g[V0][0] = 3;
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = 1;
        if same {
            c.cycles += 1;
            frame.restore(c, S0, 0x760);
        } else {
            c.cycles += 3;
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
            c.g[S4][0] = 1;
            c.g[V0][0] = 0x0046_0000;
            loop {
                c.cycles += 26;
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1df0));
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0041_7828, 0x0018_e9f8)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x0031_7670, 0x0018_ea04)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_ea10)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                oc841_vcall(c, S0, V0, 0xc8, V1, 0xcc, 0x0018_ea2c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(8));
                c.g[A1][0] = 1;
                oc841_vcall(c, S0, V0, 0x40, V1, 0x44, 0x0018_ea44)?;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 5) as u64;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x760);
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1bc));
                let again = c.g[V0][0] == c.g[S4][0];
                c.g[V0][0] = 0x0046_0000;
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_e9e8);
                    }
                    continue;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                let again = c.g[V0][0] != 0;
                c.g[V0][0] = 0x0046_0000;
                if again {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_e9e8);
                    }
                    continue;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x760);
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x750);
        frame.restore(c, S2, 0x740);
        frame.restore(c, S3, 0x730);
        frame.restore(c, S4, 0x720);
        frame.restore(c, S5, 0x710);
        frame.restore(c, S6, 0x700);
        frame.restore(c, S7, 0x6f0);
        frame.restore(c, FP, 0x6e0);
        frame.restore_ra(c, 0x6d0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and magic of this module's fakes; the seed word and the two
    /// "no next device" flags (left, right) follow it.
    pub(crate) const OC841_CTRL: u32 = DATA + 0x2e_0000;
    pub(crate) const OC841_MAGIC: u32 = 0x4f43_3834;
    /// The screen, the memory card manager, the card object and one text.
    pub(crate) const OC841_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC841_MC: u32 = DATA + 0x2000;
    pub(crate) const OC841_CARD: u32 = DATA + 0x3000;
    pub(crate) const OC841_TEXT: u32 = DATA + 0x4000;
    pub(crate) const OC841_VT_SELF: u32 = DATA + 0x5000;
    pub(crate) const OC841_VT_CARD: u32 = DATA + 0x6000;
    pub(crate) const OC841_VT_TEXT: u32 = DATA + 0x7000;

    macro_rules! oc841_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $post:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC841_CTRL) != OC841_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let a3 = c.g[A3][0];
                    let ra = c.g[RA][0] as u32;
                    fakes::fake_call(c, $addr);
                    if c.pc == ra {
                        let post: fn(&mut Cpu, u64) = $post;
                        post(c, a3);
                    }
                }
            )*
            pub(crate) fn oc841_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    fn oc841_none(_: &mut Cpu, _: u64) {}
    fn oc841_post_mc(c: &mut Cpu, _: u64) {
        c.g[V0][0] = OC841_MC as u64;
    }
    fn oc841_post_text(c: &mut Cpu, _: u64) {
        c.g[V0][0] = OC841_TEXT as u64;
    }
    /// `checkNextAvailableDevice(.., a3)`: -1 (none) when the seed says so.
    fn oc841_post_next(c: &mut Cpu, a3: u64) {
        if c.bus.read32(OC841_CTRL + 8 + 4 * (a3 as u32 & 1)) != 0 {
            c.g[V0][0] = u64::MAX;
        }
    }

    oc841_fakes!(
        (
            oc841_fake_227f80,
            OC841_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80,
            oc841_post_mc
        ),
        (
            oc841_fake_1d8240,
            OC841_PREV_1D8240,
            ee::s04::OVERRIDE_001D8240,
            ee::s04::f_001d8240__generated,
            0x001d_8240,
            oc841_none
        ),
        (
            oc841_fake_241710,
            OC841_PREV_241710,
            ee::s06::OVERRIDE_00241710,
            ee::s06::f_00241710__generated,
            0x0024_1710,
            oc841_none
        ),
        (
            oc841_fake_317670,
            OC841_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            oc841_none
        ),
        (
            oc841_fake_39d860,
            OC841_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            oc841_post_text
        ),
        (
            oc841_fake_3a0d00,
            OC841_PREV_3A0D00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a_0d00,
            oc841_none
        ),
        (
            oc841_fake_417828,
            OC841_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            oc841_none
        ),
        (
            oc841_fake_3a0e90,
            OC841_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90,
            oc841_none
        ),
        (
            oc841_fake_1d97e0,
            OC841_PREV_1D97E0,
            ee::s04::OVERRIDE_001D97E0,
            ee::s04::f_001d97e0__generated,
            0x001d_97e0,
            oc841_post_next
        ),
    );

    /// Virtual methods (fake addresses): the answers of the "is it so"
    /// methods come from a hash of the call, so each seed walks other branches.
    const OC841_DISPLAY_FILE_OK: u32 = fakes::FAKE + 0x10;
    const OC841_IS_AVAILABLE: u32 = fakes::FAKE + 0x40;

    fn oc841_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT) as u64;
        let seed = c.bus.read32(OC841_CTRL + 4) as u64;
        let h = fakes::mix(target as u64 ^ c.g[A1][0] << 8 ^ seed << 32 ^ n << 20);
        fakes::fake_call(c, target);
        let mostly = target == OC841_DISPLAY_FILE_OK || target == OC841_IS_AVAILABLE;
        let yes = if mostly { h & 7 != 0 } else { h & 1 != 0 };
        c.g[V0][0] = yes as u64;
    }

    fn oc841_slot(c: &mut Cpu, r: &mut crate::difftest::Rng, vt: u32, off: u32, k: u32) {
        let delta = (r.next_u32() % 8) * 4;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * k);
    }

    /// Reference: `cFEStateOptionsDeviceSelect::displayStatus`: both
    /// `displayFileOK` answers, a card present or not, screen state 3 or not,
    /// screen types 1 / 0 / 2 (all four slots or just the first), the slot
    /// flag branches (device flags and save screen), arrows with and without a
    /// next device, and callee unwinds at random calls.
    #[test]
    fn device_select_display_status_matches() {
        oc841_install_fakes();
        check(
            |c| {
                c.pc = 0x0018_e510;
                ee::s02::f_0018e510__generated(c)
            },
            oc841_device_select_display_status,
            600,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8410, 40);
                c.hooks.miss = oc841_miss;
                c.bus.write32(OC841_CTRL, OC841_MAGIC);
                c.bus.write32(OC841_CTRL + 4, r.next_u32());
                c.bus.write32(OC841_CTRL + 8, r.next_u32() & 1);
                c.bus.write32(OC841_CTRL + 12, r.next_u32() & 1);
                c.bus.write32(OC841_SELF + 8, OC841_VT_SELF);
                c.bus.write32(OC841_SELF + 0x40, DATA + 0x8000);
                c.bus
                    .write32(OC841_SELF + 0x1bc, [1, 1, 0, 2][(seed % 4) as usize]);
                c.bus
                    .write32(OC841_SELF + 0x214, [3, 0, 1, 3, 2][(seed / 4 % 5) as usize]);
                c.bus.write32(OC841_SELF + 0x22c, r.next_u32() & 1);
                c.bus.write32(OC841_MC + 0x428, r.next_u32() % 4);
                c.bus.write32(OC841_MC + 0x434, OC841_CARD);
                c.bus.write32(OC841_CARD, OC841_VT_CARD);
                c.bus.write32(OC841_TEXT + 8, OC841_VT_TEXT);
                for (off, k) in [(0x118, 0), (0x150, 1), (0x158, 2)] {
                    oc841_slot(c, &mut r, OC841_VT_SELF, off, k);
                }
                for (off, k) in [
                    (0x70, 3),
                    (0x1a8, 4),
                    (0x1b0, 5),
                    (0x1b8, 6),
                    (0x210, 7),
                    (0x218, 8),
                ] {
                    oc841_slot(c, &mut r, OC841_VT_CARD, off, k);
                }
                for (off, k) in [(0x40, 9), (0x48, 10), (0xc8, 11)] {
                    oc841_slot(c, &mut r, OC841_VT_TEXT, off, k);
                }
                c.g[A0][0] = sx(OC841_SELF);
                c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
                c.pc = 0x0018_e510;
            },
        );
    }
}
