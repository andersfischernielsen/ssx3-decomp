//! Native Freestyle finish-overlay initialization1e8200, retained uninstalled.
//!
//! Creates the retail finishov screen, resets its timer, selects heat/medal
//! and placement widgets, formats the score and marks record/DNF presentation.
//! Required widgets are dereferenced as in the original. No automated test,
//! build or runtime installation is added under the user's instruction.

use crate::call::{call_ptr, guest, Frame, Flow};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x001e_8200;
const END: u32 = 0x001e_87b8;
const FRAME: u32 = 0x270;
const SCREEN: u32 = 0x40;
const TIMER: u32 = 0x9c;
const SCORE_IFACE: u32 = 0x1c0;
const MODE_MANAGER: u32 = 0x1c4;
const PLAYER_OFFSET: u32 = 0x1c8;
const HASH: u32 = 0x0031_7670;
const FIND_WIDGET: u32 = 0x0039_d860;
const HIDE: u32 = 0x0039_e8b8;
const SHOW: u32 = 0x0039_e948;
const BE_GET: u32 = 0x0014_dc80;
const BE_INTERFACE: u32 = 0x0014_dd58;

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }
#[inline]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 { r(c, reg).wrapping_add(off) }

/// The required widget in S2 receives its native visibility slot48.
fn widget_visible(c: &mut Cpu, visible: u64, ret: u32) -> Flow {
    c.g[V0][0] = c.lw(at(c, S2, 8));
    c.g[A1][0] = visible;
    c.g[A0][0] = c.lh(at(c, V0, 0x48));
    c.g[V1][0] = c.lw(at(c, V0, 0x4c));
    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
    let target = r(c, V1);
    call_ptr(c, target, ret)
}

/// `void OVScreenFinishOv2_Init(OVScreenFinishOv2* self)`.
/// S3 retains self; S4 is the current heat; S2 initially holds rewards and
/// later each widget. Native call-return PCs preserve the0x270-byte frame.
pub fn freestyle_finish_ui_init(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: FRAME };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 23;
                    Frame::open(c, FRAME);
                    frame.save(c, S3, 0x230);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, S1, 0x250);
                    frame.save(c, S2, 0x240);
                    c.g[A0][0] = 0x0047_0000;
                    frame.save(c, S4, 0x220);
                    c.g[A0][0] = sx(at(c, A0, 0u32.wrapping_sub(0x1cc0)));
                    frame.save(c, S5, 0x210);
                    frame.save(c, S6, 0x200);
                    frame.save(c, S7, 0x1f0);
                    frame.save(c, FP, 0x1e0);
                    frame.save_ra(c, 0x1d0);
                    frame.save(c, S0, 0x260);
                    c.g[S0][0] = c.lw(at(c, S3, 0x10));
                    call_ptr(c, HASH, 0x001e_8240)?;
                    pc = 0x001e_8240;
                }
                0x001e_8240 => {
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0039_7cc0, 0x001e_8254)?;
                    pc = 0x001e_8254;
                }
                0x001e_8254 => {
                    let absent = c.g[V0][0] == 0;
                    c.sw(at(c, S3, SCREEN), c.g[V0][0]);
                    if !absent {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x0039_c870, 0x001e_826c)?;
                    }
                    pc = 0x001e_826c;
                }
                0x001e_826c => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0020_a380, 0x001e_8274)?;
                    pc = 0x001e_8274;
                }
                0x001e_8274 => {
                    c.sw(at(c, S3, TIMER), 0);
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx(at(c, A0, 0u32.wrapping_sub(0x17e8)));
                    call_ptr(c, HASH, 0x001e_8284)?;
                    pc = 0x001e_8284;
                }
                0x001e_8284 => {
                    c.g[A0][0] = c.lw(at(c, S3, SCREEN));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e_8290)?;
                    pc = 0x001e_8290;
                }
                0x001e_8290 => {
                    c.g[V1][0] = c.g[V0][0];
                    if c.g[V1][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, V1, 0x90));
                        c.g[V0][0] |= 8;
                        c.sw(at(c, V1, 0x90), c.g[V0][0]);
                    }
                    pc = 0x001e_82a8;
                }
                0x001e_82a8 => {
                    c.cycles += 49;
                    c.g[S0][0] = 0x0053_0000;
                    call_ptr(c, BE_GET, 0x001e_82b0)?;
                    pc = 0x001e_82b0;
                }
                0x001e_82b0 | 0x001e_82c4 | 0x001e_82d8 | 0x001e_82ec => {
                    c.g[A0][0] = c.g[V0][0];
                    let (kind, ret) = match pc {
                        0x001e_82b0 => (0, 0x001e_82bc),
                        0x001e_82c4 => (1, 0x001e_82d0),
                        0x001e_82d8 => (8, 0x001e_82e4),
                        _ => (0xd, 0x001e_82f8),
                    };
                    c.g[A1][0] = kind;
                    call_ptr(c, BE_INTERFACE, ret)?;
                    pc = ret;
                }
                0x001e_82bc => {
                    c.g[S0][0] = sx(at(c, S0, 0x5bc8));
                    call_ptr(c, BE_GET, 0x001e_82c4)?;
                    pc = 0x001e_82c4;
                }
                0x001e_82d0 => {
                    c.g[S7][0] = c.g[V0][0];
                    call_ptr(c, BE_GET, 0x001e_82d8)?;
                    pc = 0x001e_82d8;
                }
                0x001e_82e4 => {
                    c.sw(frame.at(c, SCORE_IFACE), c.g[V0][0]);
                    call_ptr(c, BE_GET, 0x001e_82ec)?;
                    pc = 0x001e_82ec;
                }
                0x001e_82f8 => {
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V1][0] = 0x0047_0000;
                    c.g[V0][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
                    c.g[S6][0] = sx(at(c, V1, 0u32.wrapping_sub(0x17d8)));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = c.lw(at(c, V0, 0xc0));
                    c.g[A1][0] = c.g[S6][0];
                    c.sw(frame.at(c, MODE_MANAGER), c.g[V0][0]);
                    c.g[S1][0] = c.lb(at(c, S0, 0x48));
                    call_ptr(c, HIDE, 0x001e_8320)?;
                    pc = 0x001e_8320;
                }
                0x001e_8320 => {
                    c.g[V0][0] = 0x0047_0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S5][0] = sx(at(c, V0, 0u32.wrapping_sub(0x17c8)));
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, HIDE, 0x001e_8334)?;
                    pc = 0x001e_8334;
                }
                0x001e_8334 => {
                    c.g[A1][0] = 0x0047_0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0u32.wrapping_sub(0x17b8)));
                    call_ptr(c, HIDE, 0x001e_8344)?;
                    pc = 0x001e_8344;
                }
                0x001e_8344 => {
                    c.g[V0][0] = 0x0047_0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[S0][0] = sx(at(c, V0, 0u32.wrapping_sub(0x17a8)));
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, HIDE, 0x001e_8358)?;
                    pc = 0x001e_8358;
                }
                0x001e_8358 => {
                    c.g[V0][0] = c.lw(frame.at(c, MODE_MANAGER));
                    c.g[S4][0] = c.lw(r(c, V0));
                    c.g[V0][0] = 6;
                    let event6 = c.g[S1][0] == c.g[V0][0];
                    c.g[V0][0] = 2;
                    if event6 { pc = 0x001e_83bc; continue; }
                    c.cycles += 2;
                    let second = c.g[S4][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 3) as u64;
                    if second { pc = 0x001e_839c; continue; }
                    c.cycles += 2;
                    let earlier = c.g[V0][0] != 0;
                    c.g[V0][0] = 3;
                    if earlier { pc = 0x001e_838c; }
                    else {
                        c.cycles += 2;
                        let final_heat = c.g[S4][0] == c.g[V0][0];
                        c.g[A0][0] = c.g[S7][0];
                        if final_heat { pc = 0x001e_83b0; }
                        else { c.cycles += 2; pc = 0x001e_83c0; }
                    }
                }
                0x001e_838c => {
                    c.cycles += 2;
                    let negative = (c.g[S4][0] as i64) < 0;
                    c.g[A1][0] = c.g[S6][0];
                    if negative { pc = 0x001e_83bc; }
                    else { c.cycles += 2; pc = 0x001e_83a0; }
                }
                0x001e_839c => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S5][0];
                    pc = 0x001e_83a0;
                }
                0x001e_83a0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, SHOW, 0x001e_83a8)?;
                    pc = 0x001e_83a8;
                }
                0x001e_83a8 => { c.g[A0][0] = c.g[S7][0]; pc = 0x001e_83c0; }
                0x001e_83b0 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, SHOW, 0x001e_83bc)?;
                    pc = 0x001e_83bc;
                }
                0x001e_83bc => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S7][0];
                    pc = 0x001e_83c0;
                }
                0x001e_83c0 => {
                    c.cycles += 10;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_6e98, 0x001e_83c8)?;
                    pc = 0x001e_83c8;
                }
                0x001e_83c8 => {
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw(at(c, S2, 0x14));
                    c.g[V0][0] = 0x0053_0000;
                    c.g[V1][0] = sx(r(c, S0) << 2);
                    c.g[V0][0] = sx(at(c, V0, 0x6730));
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    let rewarded = c.g[A0][0] != 0;
                    c.g[S1][0] = c.lw(r(c, V1));
                    if rewarded { pc = 0x001e_8430; continue; }
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    let final_heat = c.g[S4][0] == c.g[V0][0];
                    c.g[V1][0] = 0x0053_0000;
                    if final_heat { pc = 0x001e_8410; continue; }
                    c.cycles += 5;
                    c.g[V0][0] = sx(at(c, V1, 0x5bc8));
                    c.g[A0][0] = c.lb(at(c, V0, 0x4a));
                    c.g[V1][0] = 4;
                    let fourth_event = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = 5;
                    if fourth_event { pc = 0x001e_8410; }
                    else {
                        c.cycles += 2;
                        let other_event = c.g[A0][0] != c.g[V0][0];
                        c.g[A1][0] = 0x0047_0000;
                        pc = if other_event { 0x001e_8434 } else { 0x001e_8410 };
                    }
                }
                0x001e_8410 => {
                    c.cycles += 8;
                    c.g[V0][0] = 1;
                    c.g[A1][0] = 0x0047_0000;
                    c.sw(at(c, GP, 0u32.wrapping_sub(0xfb4)), c.g[V0][0]);
                    c.g[A1][0] = sx(at(c, A1, 0u32.wrapping_sub(0x1860)));
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, SHOW, 0x001e_8428)?;
                    pc = 0x001e_8428;
                }
                0x001e_8428 | 0x001e_8444 => {
                    c.g[V1][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x001e_8448;
                }
                0x001e_8430 => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x0047_0000;
                    pc = 0x001e_8434;
                }
                0x001e_8434 => {
                    c.cycles += 5;
                    c.sw(at(c, GP, 0u32.wrapping_sub(0xfb4)), 0);
                    c.g[A1][0] = sx(at(c, A1, 0u32.wrapping_sub(0x1860)));
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, HIDE, 0x001e_8444)?;
                    pc = 0x001e_8444;
                }
                0x001e_8448 => {
                    c.cycles += 3;
                    c.g[V0][0] = 1;
                    let not_gold = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 0x004a_0000;
                    if not_gold { pc = 0x001e_848c; continue; }
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    let final_heat = c.g[S4][0] == c.g[V0][0];
                    c.g[A0][0] = 0x0053_0000;
                    if final_heat { pc = 0x001e_8478; continue; }
                    c.cycles += 5;
                    c.g[V0][0] = sx(at(c, A0, 0x5bc8));
                    c.g[V1][0] = c.lb(at(c, V0, 0x4a));
                    c.g[A0][0] = 5;
                    let other = c.g[V1][0] != c.g[A0][0];
                    c.g[A0][0] = c.g[S3][0];
                    if other { pc = 0x001e_8490; }
                    else { c.cycles += 1; c.g[A1][0] = 0x004a_0000; pc = 0x001e_8478; }
                }
                0x001e_8478 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0x2180));
                    call_ptr(c, SHOW, 0x001e_8484)?;
                    pc = 0x001e_8484;
                }
                0x001e_8484 | 0x001e_8498 => {
                    c.g[V1][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x001e_849c;
                }
                0x001e_848c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                    pc = 0x001e_8490;
                }
                0x001e_8490 => {
                    c.cycles += 3;
                    c.g[A1][0] = sx(at(c, A1, 0x2180));
                    call_ptr(c, HIDE, 0x001e_8498)?;
                    pc = 0x001e_8498;
                }
                0x001e_849c => {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = 0x004a_0000;
                        pc = 0x001e_84e0;
                        continue;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    let final_heat = c.g[S4][0] == c.g[V0][0];
                    c.g[V1][0] = 0x0053_0000;
                    if final_heat { pc = 0x001e_84c8; continue; }
                    c.cycles += 5;
                    c.g[A0][0] = 5;
                    c.g[V0][0] = sx(at(c, V1, 0x5bc8));
                    c.g[V1][0] = c.lb(at(c, V0, 0x4a));
                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[A1][0] = 0x004a_0000;
                        pc = 0x001e_84e0;
                    } else { pc = 0x001e_84c8; }
                }
                0x001e_84c8 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x004a_0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0x2188));
                    call_ptr(c, SHOW, 0x001e_84d8)?;
                    pc = 0x001e_84d8;
                }
                0x001e_84d8 | 0x001e_84ec => {
                    c.g[V1][0] = c.lw(at(c, S2, 0x14));
                    pc = 0x001e_84f0;
                }
                0x001e_84e0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0x2188));
                    call_ptr(c, HIDE, 0x001e_84ec)?;
                    pc = 0x001e_84ec;
                }
                0x001e_84f0 => {
                    c.cycles += 3;
                    c.g[V0][0] = 3;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A1][0] = 0x004a_0000;
                        pc = 0x001e_8530;
                        continue;
                    }
                    c.cycles += 2;
                    let final_heat = c.g[S4][0] == c.g[V1][0];
                    c.g[A1][0] = 0x004a_0000;
                    if final_heat { pc = 0x001e_851c; continue; }
                    c.cycles += 6;
                    c.g[A0][0] = 0x0053_0000;
                    c.g[V0][0] = sx(at(c, A0, 0x5bc8));
                    c.g[V1][0] = c.lb(at(c, V0, 0x4a));
                    c.g[A0][0] = 5;
                    pc = if c.g[V1][0] != c.g[A0][0] { 0x001e_8530 } else { 0x001e_851c };
                }
                0x001e_851c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0x2190));
                    call_ptr(c, SHOW, 0x001e_8528)?;
                    pc = 0x001e_8528;
                }
                0x001e_8530 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0x2190));
                    call_ptr(c, HIDE, 0x001e_853c)?;
                    pc = 0x001e_853c;
                }
                0x001e_8528 | 0x001e_853c => {
                    c.g[S0][0] = sx(r(c, S0) << 2);
                    pc = 0x001e_8540;
                }
                0x001e_8540 => {
                    c.cycles += 8;
                    c.g[S5][0] = (c.g[S1][0] < 6) as u64;
                    c.sw(frame.at(c, PLAYER_OFFSET), c.g[S0][0]);
                    c.g[FP][0] = 0x004a_0000;
                    c.g[S0][0] = 1;
                    c.g[S1][0] = sx(at(c, S1, 1));
                    c.g[S6][0] = sx(frame.at(c, 0x20));
                    c.g[S7][0] = sx(frame.at(c, 0xf0));
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x001e_8560;
                }
                0x001e_8560 => {
                    c.cycles += 17;
                    c.g[A1][0] = sx(at(c, FP, 0x2198));
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_7828, 0x001e_856c)?;
                    pc = 0x001e_856c;
                }
                0x001e_856c => {
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, HASH, 0x001e_8574)?;
                    pc = 0x001e_8574;
                }
                0x001e_8574 => {
                    c.g[A0][0] = c.lw(at(c, S3, SCREEN));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e_8580)?;
                    pc = 0x001e_8580;
                }
                0x001e_8580 => {
                    c.g[S2][0] = c.g[V0][0];
                    widget_visible(c, 0, 0x001e_859c)?;
                    pc = 0x001e_859c;
                }
                0x001e_859c => {
                    if c.g[S0][0] != c.g[S1][0] {
                        c.g[S0][0] = sx(at(c, S0, 1));
                        pc = 0x001e_85c0;
                    } else {
                        c.cycles += 7;
                        widget_visible(c, 1, 0x001e_85bc)?;
                        pc = 0x001e_85bc;
                    }
                }
                0x001e_85bc => { c.g[S0][0] = sx(at(c, S0, 1)); pc = 0x001e_85c0; }
                0x001e_85c0 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 7) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = c.g[SP][0];
                        if back_edge_unwinds(c, 0x001e_8560) { return Err(crate::call::Unwound); }
                        pc = 0x001e_8560;
                    } else {
                        c.cycles += 2;
                        if c.g[S5][0] != 0 { c.g[A0][0] = 0x0047_0000; pc = 0x001e_85e8; }
                        else {
                            c.cycles += 5;
                            c.g[A1][0] = 0x004a_0000;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = sx(at(c, A1, 0x21a8));
                            call_ptr(c, SHOW, 0x001e_85e4)?;
                            pc = 0x001e_85e4;
                        }
                    }
                }
                0x001e_85e4 => { c.g[A0][0] = 0x0047_0000; pc = 0x001e_85e8; }
                0x001e_85e8 => {
                    c.cycles += 43;
                    c.g[A0][0] = sx(at(c, A0, 0u32.wrapping_sub(0x1798)));
                    call_ptr(c, HASH, 0x001e_85f0)?;
                    pc = 0x001e_85f0;
                }
                0x001e_85f0 => {
                    c.g[A0][0] = c.lw(at(c, S3, SCREEN));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e_85fc)?;
                    pc = 0x001e_85fc;
                }
                0x001e_85fc => {
                    c.g[V1][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x0047_0000;
                    c.g[S0][0] = c.lw(at(c, V1, 0x8c));
                    c.g[A0][0] = sx(at(c, A0, 0u32.wrapping_sub(0x1768)));
                    c.g[S1][0] = c.lw(at(c, S0, 4));
                    c.g[V0][0] = c.lh(at(c, S1, 0x20));
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V0)));
                    call_ptr(c, HASH, 0x001e_8620)?;
                    pc = 0x001e_8620;
                }
                0x001e_8620 => {
                    c.g[V1][0] = c.lw(at(c, S1, 0x24));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    let target = r(c, V1);
                    call_ptr(c, target, 0x001e_8630)?;
                    pc = 0x001e_8630;
                }
                0x001e_8630 => {
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002c_2508, 0x001e_863c)?;
                    pc = 0x001e_863c;
                }
                0x001e_863c => {
                    c.g[V0][0] = c.lw(frame.at(c, PLAYER_OFFSET));
                    c.g[V1][0] = 0x0053_0000;
                    c.g[V1][0] = sx(at(c, V1, 0x6640));
                    c.g[A1][0] = c.g[S6][0];
                    c.g[V1][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A2][0] = c.lw(r(c, V1));
                    call_ptr(c, 0x002c_26d0, 0x001e_865c)?;
                    pc = 0x001e_865c;
                }
                0x001e_865c => {
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S7][0];
                    call_ptr(c, 0x003a_0e90, 0x001e_8668)?;
                    pc = 0x001e_8668;
                }
                0x001e_8668 => {
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx(at(c, A0, 0u32.wrapping_sub(0x1788)));
                    call_ptr(c, HASH, 0x001e_8674)?;
                    pc = 0x001e_8674;
                }
                0x001e_8674 => {
                    c.g[A0][0] = c.lw(at(c, S3, SCREEN));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, FIND_WIDGET, 0x001e_8680)?;
                    pc = 0x001e_8680;
                }
                0x001e_8680 => {
                    c.g[A0][0] = c.lw(frame.at(c, SCORE_IFACE));
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(at(c, A0, 0x18));
                    let record18 = (c.g[V1][0] as i64) >= 0;
                    c.g[A0][0] = 0;
                    if record18 { pc = 0x001e_86a4; }
                    else {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(frame.at(c, SCORE_IFACE));
                        c.g[V0][0] = c.lw(at(c, V1, 0x1c));
                        pc = if (c.g[V0][0] as i64) < 0 { 0x001e_86a8 } else { 0x001e_86a4 };
                    }
                }
                0x001e_86a4 => { c.cycles += 1; c.g[A0][0] = 1; pc = 0x001e_86a8; }
                0x001e_86a8 => {
                    c.cycles += 2;
                    let hidden = c.g[A0][0] == 0;
                    c.g[A1][0] = 1;
                    if hidden { pc = 0x001e_86cc; }
                    else {
                        c.cycles += 7;
                        widget_visible(c, 1, 0x001e_86c4)?;
                        pc = 0x001e_86c4;
                    }
                }
                0x001e_86cc => {
                    c.cycles += 7;
                    widget_visible(c, 0, 0x001e_86e4)?;
                    pc = 0x001e_86e4;
                }
                0x001e_86c4 | 0x001e_86e4 => {
                    c.g[A0][0] = c.lw(at(c, GP, 0u32.wrapping_sub(0x848)));
                    pc = 0x001e_86e8;
                }
                0x001e_86e8 => {
                    c.cycles += 10;
                    c.g[V0][0] = 0x0053_0000;
                    c.g[S0][0] = sx(at(c, V0, 0x5bc8));
                    c.g[A1][0] = 2;
                    c.g[V1][0] = c.lw(at(c, A0, 0x84));
                    c.g[A2][0] = c.lb(at(c, S0, 0x49));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[V1][0] = c.lw(frame.at(c, PLAYER_OFFSET));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    let two_players = c.g[A2][0] == c.g[A1][0];
                    c.g[V0][0] = c.lw(at(c, V0, 0x28));
                    if two_players { pc = 0x001e_8754; continue; }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(at(c, V0, 0x480));
                    if c.g[V0][0] != 0 { c.g[A1][0] = 0x0047_0000; pc = 0x001e_8758; continue; }
                    c.cycles += 4;
                    c.g[S0][0] = c.lb(at(c, S0, 0x4a));
                    c.g[V0][0] = 9;
                    let ninth = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 10;
                    if ninth { pc = 0x001e_8754; continue; }
                    c.cycles += 2;
                    let tenth = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 11;
                    if tenth { pc = 0x001e_8754; continue; }
                    c.cycles += 2;
                    let eleventh = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 1;
                    if eleventh { pc = 0x001e_8754; continue; }
                    c.cycles += 2;
                    let not_first_heat = c.g[S4][0] != c.g[V0][0];
                    c.g[A0][0] = c.lw(frame.at(c, MODE_MANAGER));
                    if not_first_heat { pc = 0x001e_8774; continue; }
                    c.cycles += 4;
                    c.g[V0][0] = 3;
                    c.g[V1][0] = c.lw(at(c, A0, 0x70));
                    pc = if c.g[V1][0] != c.g[V0][0] { 0x001e_8774 } else { 0x001e_8754 };
                }
                0x001e_8754 => { c.cycles += 1; c.g[A1][0] = 0x0047_0000; pc = 0x001e_8758; }
                0x001e_8758 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0u32.wrapping_sub(0x1778)));
                    call_ptr(c, HIDE, 0x001e_8764)?;
                    pc = 0x001e_8764;
                }
                0x001e_8764 => {
                    c.g[A1][0] = 0x004a_0000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx(at(c, A1, 0x21a0));
                    call_ptr(c, HIDE, 0x001e_8774)?;
                    pc = 0x001e_8774;
                }
                0x001e_8774 => {
                    c.cycles += 17;
                    call_ptr(c, 0x0028_b180, 0x001e_877c)?;
                    pc = 0x001e_877c;
                }
                0x001e_877c => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xe;
                    call_ptr(c, 0x0029_4f78, 0x001e_8788)?;
                    pc = 0x001e_8788;
                }
                0x001e_8788 => {
                    for (reg, off) in [(S0,0x260), (S1,0x250), (S2,0x240), (S3,0x230),
                                       (S4,0x220), (S5,0x210), (S6,0x200), (S7,0x1f0), (FP,0x1e0)] {
                        frame.restore(c, reg, off);
                    }
                    frame.restore_ra(c, 0x1d0);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
