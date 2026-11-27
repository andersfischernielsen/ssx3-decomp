//! Staging module oc1023 (wave wave262): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1023`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `reg + off` as a guest address.
#[inline(always)]
fn oc1023_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_52a0,
        oc1023_buy_attrib_update_money,
        "cFEStateBuyAttrib_UpdateMoney",
    ),
    (
        0x001f_5300,
        oc1023_buy_attrib_commit_purchase,
        "cFEStateBuyAttrib_CommitPurchase",
    ),
    (
        0x001f_5508,
        oc1023_career_stats_init,
        "cFEStateCareerStats_Init",
    ),
    (
        0x001f_8448,
        oc1023_pause_freeride_transition_in,
        "OVScreenPauseFreeride_TransitionIn",
    ),
    (
        0x001f_8728,
        oc1023_pause_freeride_update,
        "OVScreenPauseFreeride_Update",
    ),
    (
        0x001f_a100,
        oc1023_be_options_on_render,
        "OVScreenBEOptions_OnRender",
    ),
    (
        0x001f_c768,
        oc1023_freestyle_pre_on_activate,
        "OVScreenFreestylePre_OnActivate",
    ),
    (
        0x001f_de60,
        oc1023_save_replay_shutdown,
        "OVScreenSaveReplay_Shutdown",
    ),
];

/// `void cFEStateBuyAttrib_UpdateMoney(cFEStateBuyAttrib* self)` (0x1f52a0):
/// looks the money text object (hash of the string at 0x46f0a0) up under the
/// screen at `self+0x48`; if it exists, sets its text to the bank value
/// `self+0xa4` formatted as money.
pub fn oc1023_buy_attrib_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f0a0);
        call_ptr(c, 0x0031_7670, 0x001f_52c0)?;
        c.g[A0][0] = Addr(oc1023_at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_52cc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(oc1023_at(c, S1, 0xa4)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_52e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_52ec)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_CommitPurchase(cFEStateBuyAttrib* self)`
/// (0x1f5300): spend the bank on the pending purchase. Charges the cost
/// (`self+0xa0`) to the bank (`self+0xa4`), clears the cost, refreshes the
/// total, plays a message on the screen object (`self+0xac`), then for each
/// of the seven attributes bought (`self+0x84+4i` > 0) calls the BE
/// interface once per level; finally adds the bought levels into the owned
/// levels (`self+0x4c`, 7 words) while clearing them, and refreshes the
/// level, cost, bar and money texts.
pub fn oc1023_buy_attrib_commit_purchase(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S3, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x50);
        c.g[V0][0] = sx(oc1023_at(c, S3, 0x4c));
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        c.g[S6][0] = sx(oc1023_at(c, S3, 0x84));
        frame.save(c, S0, 0xa0);
        c.g[FP][0] = c.g[S6][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S2, 0x80);
        frame.save(c, S4, 0x60);
        frame.save_ra(c, 0x10);
        let owned = c.g[V0][0];
        let slot = frame.at(c, 0);
        c.sw(slot, owned);
        c.g[V0][0] = Addr(oc1023_at(c, S3, 0xa4)).lw(c);
        c.g[V1][0] = Addr(oc1023_at(c, S3, 0xa0)).lw(c);
        Addr(oc1023_at(c, S3, 0xa0)).sw(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let bank = c.g[V0][0];
        Addr(oc1023_at(c, S3, 0xa4)).sw(c, bank);
        call_ptr(c, 0x001f_5240, 0x001f_5358)?;
        Addr(oc1023_at(c, S3, 0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = sx(0x0046_f060);
        call_ptr(c, 0x0039_e8b8, 0x001f_536c)?;
        c.g[A2][0] = Addr(oc1023_at(c, S3, 0xac)).lw(c);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
        c.g[A0][0] = Addr(oc1023_at(c, V0, 0x38)).lh(c);
        c.g[V1][0] = Addr(oc1023_at(c, V0, 0x3c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_5388)?;
        call_ptr(c, 0x0014_dc80, 0x001f_5390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x0014_dd58, 0x001f_539c)?;
        c.g[S5][0] = c.g[V0][0];
        c.g[V1][0] = 0;
        c.g[V0][0] = 0x480000;
        c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f50));
        c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
        'l_1f53b0: loop {
            'b_1f53fc: {
                c.cycles += 5;
                c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let none = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                if none {
                    break 'b_1f53fc;
                }
                c.cycles += 3;
                c.g[S2][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[A3][0] as u32));
                c.g[S1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                c.g[A0][0] = 0;
                'l_1f53d0: loop {
                    c.cycles += 11;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    call_ptr(c, 0x0014_a080, 0x001f_53d8)?;
                    c.g[A3][0] = Addr(oc1023_at(c, S1, 0)).lw(c);
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0015_0c20, 0x001f_53ec)?;
                    c.g[V1][0] = Addr(oc1023_at(c, S2, 0)).lw(c);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[A0][0] = 0;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f_53d0);
                        }
                        continue 'l_1f53d0;
                    }
                    break 'b_1f53fc;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S4][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A3][0] = sx((c.g[V1][0] as u32) << 2);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_53b0);
                }
                continue 'l_1f53b0;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(oc1023_at(c, S5, 0xc)).lw(c);
            c.g[A0][0] = Addr(oc1023_at(c, V0, 0x8)).lh(c);
            c.g[V1][0] = Addr(oc1023_at(c, V0, 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001f_5420)?;
            c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
            c.g[A1][0] = c.g[S6][0];
            c.g[A2][0] = 6;
            'l_1f5430: loop {
                c.cycles += 9;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let sum = c.g[V1][0];
                Addr(c.g[A0][0] as u32).sw(c, sum);
                Addr(c.g[A1][0] as u32).sw(c, 0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
                let more = (c.g[A2][0] as i64) >= 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_5430);
                    }
                    continue 'l_1f5430;
                }
                c.cycles += 20;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_4ea8, 0x001f_545c)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_4fc0, 0x001f_5464)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_50b0, 0x001f_546c)?;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x001f_52a0, 0x001f_5474)?;
                frame.restore(c, S0, 0xa0);
                frame.restore(c, S1, 0x90);
                frame.restore(c, S2, 0x80);
                frame.restore(c, S3, 0x70);
                frame.restore(c, S4, 0x60);
                frame.restore(c, S5, 0x50);
                frame.restore(c, S6, 0x40);
                frame.restore(c, S7, 0x30);
                frame.restore(c, FP, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void cFEStateCareerStats_Init(cFEStateCareerStats* self)` (0x1f5508):
/// creates the career-stats screen under the engine (`self+0x10`) and plays
/// its first frame when it exists (`self+0x40`), clears the medal and page
/// counters (`self+0x48..0x74`), counts the medals, shows the first page
/// and finishes the setup with two engine calls.
pub fn oc1023_career_stats_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_f168);
        c.g[S0][0] = Addr(oc1023_at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_552c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_5540)?;
        let screen = c.g[V0][0];
        Addr(oc1023_at(c, S1, 0x40)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_5558)?;
        }
        c.cycles += 26;
        Addr(oc1023_at(c, S1, 0x4c)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        for off in [0x50, 0x54, 0x58, 0x5c, 0x64, 0x60, 0x68, 0x70, 0x6c, 0x74] {
            Addr(oc1023_at(c, S1, off)).sw(c, 0);
        }
        Addr(oc1023_at(c, S1, 0x48)).sw(c, 0);
        call_ptr(c, 0x001f_6f30, 0x001f_5590)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_5a38, 0x001f_5598)?;
        call_ptr(c, 0x0028_b180, 0x001f_55a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, 0x0028_f140, 0x001f_55ac)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool OVScreenPauseFreeride_TransitionIn(self)` (0x1f8448): on the first
/// transition in (`self+0xd4` set, then cleared) shows thirteen-plus-one
/// objects named by `sprintf` of table strings (`0x441c30`), each through a
/// vtable call with 0, then the first eight entries of the index list
/// (`self+0x9c`, `self+0xbc` of them) through the table: each found object
/// gets a vtable call with 1 and its position set to `index * 40.0`.
/// Returns 0.
pub fn oc1023_pause_freeride_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0x90);
        frame.save(c, S0, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save_ra(c, 0x50);
        frame.save_fpr(c, 20, 0xc0);
        c.g[V0][0] = Addr(oc1023_at(c, S2, 0xd4)).lw(c);
        let skip = c.g[V0][0] == 0;
        c.g[V0][0] = 0x440000;
        if !skip {
            c.cycles += 5;
            Addr(oc1023_at(c, S2, 0xd4)).sw(c, 0);
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c30));
            c.g[S4][0] = 0x470000;
            c.g[S0][0] = 0xd;
            'l_1f8490: loop {
                'b_1f84d4: {
                    c.cycles += 12;
                    c.g[A2][0] = Addr(oc1023_at(c, S1, 0)).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x7c0));
                    call_ptr(c, 0x0041_7828, 0x001f_84a0)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_7670, 0x001f_84a8)?;
                    c.g[A0][0] = Addr(oc1023_at(c, S2, 0xc0)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001f_84b4)?;
                    c.g[A2][0] = c.g[V0][0];
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if none {
                        break 'b_1f84d4;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
                    c.g[A0][0] = Addr(oc1023_at(c, V0, 0x48)).lh(c);
                    c.g[V1][0] = Addr(oc1023_at(c, V0, 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_84d4)?;
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                let more = (c.g[S0][0] as i64) >= 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_8490);
                    }
                    continue 'l_1f8490;
                }
                break;
            }
            c.cycles += 6;
            c.g[V0][0] = 0x440000;
            c.g[AT][0] = 0x4220_0000;
            c.mtc1(1, 20);
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c30));
            c.g[S1][0] = 0;
            c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x9c));
            'l_1f84f8: loop {
                'b_1f8584: {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(oc1023_at(c, S2, 0xbc)).lw(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    if done {
                        break 'b_1f8584;
                    }
                    c.cycles += 14;
                    c.g[V0][0] = Addr(oc1023_at(c, S3, 0)).lw(c);
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x7c0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    call_ptr(c, 0x0041_7828, 0x001f_8520)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_7670, 0x001f_8528)?;
                    c.g[A0][0] = Addr(oc1023_at(c, S2, 0xc0)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001f_8534)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[A1][0] = 1;
                    if none {
                        break 'b_1f8584;
                    }
                    c.cycles += 17;
                    c.g[V0][0] = Addr(oc1023_at(c, S0, 0x8)).lw(c);
                    c.g[A0][0] = Addr(oc1023_at(c, V0, 0x48)).lh(c);
                    c.g[V1][0] = Addr(oc1023_at(c, V0, 0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_8554)?;
                    c.mtc1(17, 0);
                    c.cvt_s_w(0, 0);
                    let at = frame.at(c, 0x40);
                    c.sw(at, 0);
                    let at = frame.at(c, 0x48);
                    c.sw(at, 0);
                    c.mul_s(0, 0, 20);
                    let at = frame.at(c, 0x44);
                    c.swc1(0, at);
                    let at = frame.at(c, 0x47);
                    c.ldl(2, at);
                    let at = frame.at(c, 0x40);
                    c.ldr(2, at);
                    c.g[V1][0] = Addr(frame.at(c, 0x48)).lw(c);
                    let at = oc1023_at(c, S0, 0x4b);
                    c.sdl(2, at);
                    let at = oc1023_at(c, S0, 0x44);
                    c.sdr(2, at);
                    let word = c.g[V1][0];
                    Addr(oc1023_at(c, S0, 0x4c)).sw(c, word);
                }
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 8) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(4));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001f_84f8);
                    }
                    continue 'l_1f84f8;
                }
                break;
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0xb0);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore_ra(c, 0x50);
        frame.restore_fpr(c, 20, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPauseFreeride_Update(self)` (0x1f8728): while the state
/// (`self+0x110`) is 1, 2 or 4 and the message center has unread messages,
/// pulses the blink phase (`self+0xc8` += `self+0xcc`, bouncing the step at
/// 0 and 1) and passes it to the message icon object (vtable `0x58`). Then
/// runs the common update (`0x20e900`); if the HUD object (`gp-0x204`) has
/// a pending request (`+0x68` set, `+0x64` zero) it clears it, plays the
/// UI sound, steps the screen counter (`gp-0x69c`) back one and loads the
/// next screen id from the tables.
pub fn oc1023_pause_freeride_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = Addr(oc1023_at(c, S0, 0x110)).lw(c);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let blink = c.g[V0][0] != 0;
        c.g[V0][0] = 4;
        let run_blink = if blink {
            true
        } else {
            c.cycles += 2;
            c.g[V1][0] == c.g[V0][0]
        };
        if run_blink {
            c.cycles += 4;
            call_ptr(c, 0x001e_3408, 0x001f_875c)?;
            if (c.g[V0][0] as i64) > 0 {
                'b_1f87ac: {
                    'b_1f879c: {
                        c.cycles += 9;
                        c.lwc1(0, oc1023_at(c, S0, 0xc8));
                        c.lwc1(1, oc1023_at(c, S0, 0xcc));
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 2);
                        c.add_s(0, 0, 1);
                        c.c_le_s(2, 0);
                        let at_top = c.fcc();
                        c.swc1(0, oc1023_at(c, S0, 0xc8));
                        if at_top {
                            break 'b_1f879c;
                        }
                        c.cycles += 5;
                        c.mtc1(0, 2);
                        c.c_lt_s(0, 2);
                        if !c.fcc() {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_1f87ac;
                        }
                    }
                    c.cycles += 4;
                    c.neg_s(0, 1);
                    c.swc1(2, oc1023_at(c, S0, 0xc8));
                    c.swc1(0, oc1023_at(c, S0, 0xcc));
                    c.g[A0][0] = 0x4a0000;
                }
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2478));
                call_ptr(c, 0x0031_7670, 0x001f_87b4)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0xc0)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_87c0)?;
                c.g[A2][0] = c.g[V0][0];
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = c.g[SP][0];
                if !none {
                    c.cycles += 15;
                    c.lwc1(0, oc1023_at(c, S0, 0xc8));
                    let at = oc1023_at(c, A2, 0x23);
                    c.ldl(2, at);
                    let at = oc1023_at(c, A2, 0x1c);
                    c.ldr(2, at);
                    let at = oc1023_at(c, A2, 0x2b);
                    c.ldl(3, at);
                    let at = oc1023_at(c, A2, 0x24);
                    c.ldr(3, at);
                    let at = frame.at(c, 0x7);
                    c.sdl(2, at);
                    let at = frame.at(c, 0);
                    c.sdr(2, at);
                    let at = frame.at(c, 0xf);
                    c.sdl(3, at);
                    let at = frame.at(c, 0x8);
                    c.sdr(3, at);
                    let at = frame.at(c, 0);
                    c.swc1(0, at);
                    c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
                    c.g[A0][0] = Addr(oc1023_at(c, V0, 0x58)).lh(c);
                    c.g[V1][0] = Addr(oc1023_at(c, V0, 0x5c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001f_8808)?;
                }
            }
        }
        // 0x1f8808
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0020_e900, 0x001f_8810)?;
        c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x204))).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(oc1023_at(c, V1, 0x68)).lw(c);
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            let mut pending = false;
            if none {
                // 0x1f8834
                c.cycles += 1;
                pending = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(oc1023_at(c, V1, 0x64)).lw(c);
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    pending = true;
                }
            }
            if pending {
                c.g[A0][0] = 1;
            }
            c.cycles += 2;
            let idle = c.g[A0][0] == 0;
            c.g[A0][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x204))).lw(c);
            if !idle {
                c.cycles += 37;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0025_8790, 0x001f_8848)?;
                c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x204))).lw(c);
                c.g[V0][0] = u64::MAX;
                c.g[A0][0] = 0x470000;
                let minus_one = c.g[V0][0];
                Addr(oc1023_at(c, V1, 0x64)).sw(c, minus_one);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7b0));
                Addr(oc1023_at(c, V1, 0x68)).sw(c, minus_one);
                call_ptr(c, 0x0031_7670, 0x001f_8864)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0xc0)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_8870)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0xc0)).lw(c);
                c.g[A2][0] = 1;
                c.g[A1][0] = c.g[V0][0] & 0xffff;
                call_ptr(c, 0x0039_c870, 0x001f_8880)?;
                c.g[V1][0] = 1;
                Addr(oc1023_at(c, S0, 0xd0)).sw(c, 1);
                call_ptr(c, 0x0020_d190, 0x001f_888c)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_889c)?;
                c.g[A0][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).lw(c);
                c.g[A1][0] = 0x530000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x66e8));
                c.g[A2][0] = 0x440000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                let counter = c.g[A0][0];
                Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).sw(c, counter);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
                let screen_id = c.g[V1][0];
                Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x6a0))).sw(c, screen_id);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBEOptions_OnRender(self)` (0x1fa100): per frame, makes the
/// options screen follow the BE state. Gets the BE interface (selector 0)
/// and the world's interface (selector 7), then, unless the flag at
/// `0x534b30` is set or the byte at `0x535c11` is 2, shows the object under
/// the screen (`self+0x40`) named at `0x4a24e0` (vtable call `0x44` with 1)
/// and notifies the ones named at `0x46f970` and `0x4a24a8` (`0x194498`);
/// finally, if the byte at `0x535c10` is not 4, notifies the one named at
/// `0x4a24e8`.
pub fn oc1023_be_options_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x530000;
        call_ptr(c, 0x0014_dc80, 0x001f_a11c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001f_a128)?;
        c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[A1][0] = 7;
        c.g[A0][0] = Addr(oc1023_at(c, V1, 0x78)).lw(c);
        call_ptr(c, 0x0014_dd58, 0x001f_a138)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(oc1023_at(c, V1, 0x4b30)).lw(c);
        // `skip_to_flags`: the branch straight to the final byte test (0x1fa1ec).
        let mut direct = false;
        if c.g[V0][0] != 0 {
            c.g[A0][0] = 0x4a0000;
        } else {
            c.cycles += 5;
            c.g[V1][0] = sx(oc1023_at(c, S1, 0x5bc8));
            c.g[V0][0] = Addr(oc1023_at(c, V1, 0x49)).lb(c);
            c.g[V0][0] ^= 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = Addr(oc1023_at(c, V1, 0x48)).lb(c);
                direct = true;
            } else {
                c.cycles += 1;
                c.g[A0][0] = 0x4a0000;
            }
        }
        if !direct {
            // 0x1fa160: the object named at 0x4a24e0 gets a vtable call (1).
            'b_1fa194: {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24e0));
                call_ptr(c, 0x0031_7670, 0x001f_a168)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_a174)?;
                c.g[A2][0] = c.g[V0][0];
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 1;
                if none {
                    break 'b_1fa194;
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(oc1023_at(c, A2, 0x8)).lw(c);
                c.g[A0][0] = Addr(oc1023_at(c, V0, 0x40)).lh(c);
                c.g[V1][0] = Addr(oc1023_at(c, V0, 0x44)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001f_a194)?;
            }
            'b_1fa1c0: {
                c.cycles += 8;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x690));
                call_ptr(c, 0x0031_7670, 0x001f_a1a0)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_a1ac)?;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = 0x4a0000;
                    break 'b_1fa1c0;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0019_4498, 0x001f_a1bc)?;
                c.g[A0][0] = 0x4a0000;
            }
            'b_1fa1e8: {
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24a8));
                call_ptr(c, 0x0031_7670, 0x001f_a1c8)?;
                c.g[A0][0] = Addr(oc1023_at(c, S0, 0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_a1d4)?;
                let none = c.g[V0][0] == 0;
                c.g[V1][0] = sx(oc1023_at(c, S1, 0x5bc8));
                if none {
                    break 'b_1fa1e8;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0019_4498, 0x001f_a1e4)?;
                c.g[V1][0] = sx(oc1023_at(c, S1, 0x5bc8));
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(oc1023_at(c, V1, 0x48)).lb(c);
        }
        // 0x1fa1ec
        'b_1fa224: {
            c.cycles += 3;
            c.g[V0][0] ^= 4;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x20);
                break 'b_1fa224;
            }
            c.cycles += 8;
            c.g[A0][0] = 0x4a0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24e8));
            call_ptr(c, 0x0031_7670, 0x001f_a204)?;
            c.g[A0][0] = Addr(oc1023_at(c, S0, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_a210)?;
            let none = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x20);
            if none {
                break 'b_1fa224;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_4498, 0x001f_a220)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenFreestylePre_OnActivate(self)` (0x1fc768): sets up the
/// pre-race screen's strings. Resets the screen (`0x39e4c0`), asks the BE
/// interface (selector 0), then applies string tables through `0x20a6f0`:
/// two entries chosen by whether the world's `+0xc0` object starts with 2,
/// and (if the byte at `0x535c11` is 2) two more; finally fires the screen
/// events through the application object (`0x28b180`).
pub fn oc1023_freestyle_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1b18));
        call_ptr(c, 0x0039_e4c0, 0x001f_c790)?;
        c.g[S3][0] = c.g[S0][0];
        call_ptr(c, 0x0014_dc80, 0x001f_c798)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001f_c7a4)?;
        c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2508));
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = Addr(oc1023_at(c, V1, 0xc0)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x0020_a6f0, 0x001f_c7c0)?;
        c.g[V1][0] = Addr(oc1023_at(c, S0, 0)).lw(c);
        c.g[V0][0] = 2;
        let other = c.g[V1][0] != c.g[V0][0];
        c.g[A1][0] = 0x4a0000;
        if other {
            c.cycles += 5;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2510));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020_a6f0, 0x001f_c7fc)?;
            c.g[V0][0] = 0x530000;
        } else {
            c.cycles += 7;
            c.g[A1][0] = 0x470000;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x208));
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020_a6f0, 0x001f_c7e4)?;
            c.g[V0][0] = 0x530000;
        }
        'b_1fc838: {
            c.cycles += 4;
            c.g[A0][0] = 2;
            c.g[V1][0] = Addr(oc1023_at(c, V0, 0x5c11)).lb(c);
            let other = c.g[V1][0] != c.g[A0][0];
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1b18));
            if other {
                break 'b_1fc838;
            }
            c.cycles += 10;
            c.g[A1][0] = 0x4a0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2518));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0020_a6f0, 0x001f_c824)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x228));
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0020_a6f0, 0x001f_c838)?;
        }
        c.cycles += 16;
        call_ptr(c, 0x0028_b180, 0x001f_c840)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x0029_4f78, 0x001f_c84c)?;
        call_ptr(c, 0x0028_b180, 0x001f_c854)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a_31c0, 0x001f_c85c)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSaveReplay_Shutdown(self)` (0x1fde60): if the replay-saved
/// flag (`self+0x1c8`) is 1, tells the world's save module
/// (`world+0x84 -> +0x28`, `0x270ed8`) and clears the flag. Then steps the
/// screen counter (`gp-0x69c`) back one, loads the screen id from the tables
/// into `gp-0x6a0`, refreshes the screen (`0x1d58b8`) and starts the screen
/// change through the save module (`0x26f8a0`).
pub fn oc1023_save_replay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(oc1023_at(c, S0, 0x1c8)).lw(c);
        let other = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).lw(c);
        if !other {
            c.cycles += 6;
            c.g[V0][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x848))).lw(c);
            c.g[V1][0] = Addr(oc1023_at(c, V0, 0x84)).lw(c);
            c.g[A0][0] = Addr(oc1023_at(c, V1, 0x28)).lw(c);
            call_ptr(c, 0x0027_0ed8, 0x001f_de90)?;
            Addr(oc1023_at(c, S0, 0x1c8)).sw(c, 0);
            c.g[A1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).lw(c);
        }
        c.cycles += 26;
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
        c.g[A2][0] = 0x440000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let counter = c.g[A1][0];
        Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x69c))).sw(c, counter);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let screen_id = c.g[V1][0];
        Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x6a0))).sw(c, screen_id);
        call_ptr(c, 0x001d_58b8, 0x001f_ded4)?;
        c.g[V1][0] = Addr(oc1023_at(c, GP, 0u32.wrapping_sub(0x848))).lw(c);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(oc1023_at(c, V1, 0x84)).lw(c);
        c.g[A0][0] = Addr(oc1023_at(c, V0, 0x28)).lw(c);
        call_ptr(c, 0x0026_f8a0, 0x001f_def0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Gated fakes for every real callee of this module's ports: with the gate
    // word set in the machine's RAM a callee logs its call like the machine
    // fakes do (`fakes::fake`: registers, cycles, unwinds) and returns, with
    // the result the fixture forced for that call index (sign-extended,
    // zero and negative results included); with the gate off the slot runs
    // the body it held before.
    const OC1023_GATE: u32 = DATA + 0x2a_0000;
    const OC1023_MAGIC: u32 = 0x1023_0001;
    /// One bit per call index (first 64 calls): the result is forced.
    const OC1023_MASK: u32 = DATA + 0x2a_0008;
    const OC1023_RES: u32 = DATA + 0x2a_0100;

    fn oc1023_no_prev(_c: &mut Cpu) {
        unreachable!("oc1023 fake called before it was hooked");
    }

    /// The gated fake of the callee at `T`.
    fn oc1023_gated<const T: u32>(c: &mut Cpu) {
        if c.bus.read32(OC1023_GATE) != OC1023_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1023_gated::<T>), oc1023_no_prev);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, T, ra);
        if n < 64 && c.pc == ra && (c.bus.read64(OC1023_MASK) >> n) & 1 != 0 {
            let forced = c.bus.read32(OC1023_RES + 4 * n);
            c.g[V0][0] = sx(forced);
        }
    }

    fn oc1023_hook<const T: u32>(slot: *mut fn(&mut Cpu)) {
        fakes::hook(slot, oc1023_gated::<T>, fakes::prev_cell(oc1023_gated::<T>));
    }

    pub(crate) fn oc1023_install() {
        oc1023_hook::<0x0031_7670>(addr_of_mut!(ee::s10::OVERRIDE_00317670));
        oc1023_hook::<0x0039_d860>(addr_of_mut!(ee::s13::OVERRIDE_0039D860));
        oc1023_hook::<0x0019_8af0>(addr_of_mut!(ee::s03::OVERRIDE_00198AF0));
        oc1023_hook::<0x003a_0c50>(addr_of_mut!(ee::s13::OVERRIDE_003A0C50));
        oc1023_hook::<0x001f_5240>(addr_of_mut!(ee::s04::OVERRIDE_001F5240));
        oc1023_hook::<0x0039_e8b8>(addr_of_mut!(ee::s13::OVERRIDE_0039E8B8));
        oc1023_hook::<0x0014_dc80>(addr_of_mut!(ee::s01::OVERRIDE_0014DC80));
        oc1023_hook::<0x0014_dd58>(addr_of_mut!(ee::s01::OVERRIDE_0014DD58));
        oc1023_hook::<0x0014_a080>(addr_of_mut!(ee::s01::OVERRIDE_0014A080));
        oc1023_hook::<0x0015_0c20>(addr_of_mut!(ee::s01::OVERRIDE_00150C20));
        oc1023_hook::<0x001f_4ea8>(addr_of_mut!(ee::s04::OVERRIDE_001F4EA8));
        oc1023_hook::<0x001f_4fc0>(addr_of_mut!(ee::s04::OVERRIDE_001F4FC0));
        oc1023_hook::<0x001f_50b0>(addr_of_mut!(ee::s04::OVERRIDE_001F50B0));
        oc1023_hook::<0x001f_52a0>(addr_of_mut!(ee::s04::OVERRIDE_001F52A0));
        oc1023_hook::<0x0039_7cc0>(addr_of_mut!(ee::s13::OVERRIDE_00397CC0));
        oc1023_hook::<0x0039_c870>(addr_of_mut!(ee::s13::OVERRIDE_0039C870));
        oc1023_hook::<0x001f_6f30>(addr_of_mut!(ee::s04::OVERRIDE_001F6F30));
        oc1023_hook::<0x001f_5a38>(addr_of_mut!(ee::s04::OVERRIDE_001F5A38));
        oc1023_hook::<0x0028_b180>(addr_of_mut!(ee::s07::OVERRIDE_0028B180));
        oc1023_hook::<0x0028_f140>(addr_of_mut!(ee::s07::OVERRIDE_0028F140));
        oc1023_hook::<0x0041_7828>(addr_of_mut!(ee::s15::OVERRIDE_00417828));
        oc1023_hook::<0x001e_3408>(addr_of_mut!(ee::s04::OVERRIDE_001E3408));
        oc1023_hook::<0x0020_e900>(addr_of_mut!(ee::s05::OVERRIDE_0020E900));
        oc1023_hook::<0x0025_8790>(addr_of_mut!(ee::s06::OVERRIDE_00258790));
        oc1023_hook::<0x0039_c778>(addr_of_mut!(ee::s13::OVERRIDE_0039C778));
        oc1023_hook::<0x0020_d190>(addr_of_mut!(ee::s05::OVERRIDE_0020D190));
        oc1023_hook::<0x0039_f190>(addr_of_mut!(ee::s13::OVERRIDE_0039F190));
        oc1023_hook::<0x0019_4498>(addr_of_mut!(ee::s02::OVERRIDE_00194498));
        oc1023_hook::<0x0039_e4c0>(addr_of_mut!(ee::s13::OVERRIDE_0039E4C0));
        oc1023_hook::<0x0020_a6f0>(addr_of_mut!(ee::s05::OVERRIDE_0020A6F0));
        oc1023_hook::<0x0029_4f78>(addr_of_mut!(ee::s08::OVERRIDE_00294F78));
        oc1023_hook::<0x002a_31c0>(addr_of_mut!(ee::s08::OVERRIDE_002A31C0));
        oc1023_hook::<0x0027_0ed8>(addr_of_mut!(ee::s07::OVERRIDE_00270ED8));
        oc1023_hook::<0x001d_58b8>(addr_of_mut!(ee::s04::OVERRIDE_001D58B8));
        oc1023_hook::<0x0026_f8a0>(addr_of_mut!(ee::s07::OVERRIDE_0026F8A0));
    }

    /// A machine with the gated fakes armed and no forced results.
    fn oc1023_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1023_install();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1023_GATE, OC1023_MAGIC);
        c.bus.write64(OC1023_MASK, 0);
        r
    }

    /// Force the result of fake call `n`.
    fn oc1023_force(c: &mut Cpu, n: u32, value: u32) {
        let mask = c.bus.read64(OC1023_MASK) | 1 << n;
        c.bus.write64(OC1023_MASK, mask);
        c.bus.write32(OC1023_RES + 4 * n, value);
    }

    /// Force every call's result to 0 (one in four) or `obj`.
    fn oc1023_force_all(c: &mut Cpu, r: &mut Rng, obj: u32) {
        for n in 0..64 {
            let v = if r.next_u32() % 4 == 0 { 0 } else { obj };
            oc1023_force(c, n, v);
        }
    }

    const OC1023_OBJ: u32 = DATA + 0x2000;

    /// A UI object whose vtable (`+8`) has method slots at 0x38..0x5c (a
    /// `short` delta and a pointer into the fake range).
    fn oc1023_object(c: &mut Cpu, obj: u32) {
        let vt = obj + 0x200;
        c.bus.write32(obj + 8, vt);
        for (k, off) in [0x38u32, 0x40, 0x48, 0x58].into_iter().enumerate() {
            c.bus.write16(vt + off, 0x10 * (k as u16 + 1));
            c.bus
                .write32(vt + off + 4, fakes::FAKE + 0x20 * (k as u32 + 1));
        }
        for i in 0..16 {
            c.bus.write32(obj + 0x1c + 4 * i, 0x0101_0101 * (i + 1));
        }
    }

    /// The screen-counter tables the screen steps read: counter at
    /// `gp-0x69c` (1..=3), index table `0x5366e8`, id table `0x4428f0`.
    fn oc1023_screen_tables(c: &mut Cpu, counter: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x69c), counter);
        for k in 0..4u32 {
            c.bus.write32(0x0053_66e8 + 4 * k, k + 1);
        }
        for j in 0..8u32 {
            c.bus.write32(0x0044_28f0 + 4 * j, 0x1000 + j);
        }
    }

    #[test]
    fn oc1023_buy_attrib_update_money_matches() {
        fakes::covered(
            ee::s04::f_001f52a0__generated,
            oc1023_buy_attrib_update_money,
            40,
            &|c, seed| {
                oc1023_machine(c, seed, 0x001f_52a0, 4);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x48, DATA + 0x1000);
                c.bus.write32(this + 0xa4, seed * 1000 + 5);
                // The money text object: found (seed % 3 != 0) or not.
                oc1023_force(c, 1, if seed % 3 == 0 { 0 } else { OC1023_OBJ });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_buy_attrib_commit_purchase_matches() {
        fakes::covered(
            ee::s04::f_001f5300__generated,
            oc1023_buy_attrib_commit_purchase,
            60,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_5300, 14);
                let this = DATA + 0x100;
                c.bus.write32(this + 0xa0, seed % 5 * 3);
                c.bus.write32(this + 0xa4, seed * 7);
                for i in 0..7u32 {
                    c.bus.write32(this + 0x4c + 4 * i, r.next_u32() % 20);
                    // Levels bought: none, one, two, or (rarely) negative.
                    let bought = match r.next_u32() % 6 {
                        0 | 1 => 0,
                        2 | 3 => 1,
                        4 => 2,
                        _ => u32::MAX,
                    };
                    c.bus.write32(this + 0x84 + 4 * i, bought);
                    c.bus.write32(0x0047_80b0 + 4 * i, r.next_u32() % 9);
                }
                // The message screen object (vtable 0x38 / 0x3c).
                let screen = DATA + 0x400;
                c.bus.write32(this + 0xac, screen);
                let vt = DATA + 0x500;
                c.bus.write32(screen + 8, vt);
                c.bus.write16(vt + 0x38, 0x20);
                c.bus.write32(vt + 0x3c, fakes::FAKE + 0x80);
                // The BE interface (call 4): vtable 8 / 0xc at +0xc.
                let iface = DATA + 0x600;
                let iface_vt = DATA + 0x700;
                c.bus.write32(iface + 0xc, iface_vt);
                c.bus.write16(iface_vt + 8, 0x10);
                c.bus.write32(iface_vt + 0xc, fakes::FAKE + 0xa0);
                oc1023_force(c, 4, iface);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_career_stats_init_matches() {
        fakes::covered(
            ee::s04::f_001f5508__generated,
            oc1023_career_stats_init,
            40,
            &|c, seed| {
                oc1023_machine(c, seed, 0x001f_5508, 7);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x10, DATA + 0x300);
                for off in (0x40..0x78).step_by(4) {
                    c.bus.write32(this + off, 0x5555_0000 + off);
                }
                // The new screen: created (non-zero) or not.
                oc1023_force(c, 1, if seed % 3 == 0 { 0 } else { OC1023_OBJ });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_pause_freeride_transition_in_matches() {
        fakes::covered(
            ee::s04::f_001f8448__generated,
            oc1023_pause_freeride_transition_in,
            48,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_8448, 40);
                let this = DATA + 0x100;
                // First transition (flag set) or a later one.
                c.bus.write32(this + 0xd4, (seed % 4 != 0) as u32);
                c.bus.write32(this + 0xc0, DATA + 0x300);
                c.bus.write32(this + 0xbc, seed % 10);
                for i in 0..8u32 {
                    c.bus.write32(this + 0x9c + 4 * i, (seed + 3 * i) % 14);
                }
                for j in 0..16u32 {
                    c.bus.write32(0x0044_1c30 + 4 * j, 0x004a_0000 + 0x10 * j);
                }
                oc1023_object(c, OC1023_OBJ);
                oc1023_force_all(c, &mut r, OC1023_OBJ);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_pause_freeride_update_matches() {
        fakes::covered(
            ee::s04::f_001f8728__generated,
            oc1023_pause_freeride_update,
            96,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_8728, 12);
                let this = DATA + 0x100;
                // States 1, 2 and 4 blink when mail is waiting; 0, 3, 5 do not.
                c.bus
                    .write32(this + 0x110, [1, 2, 4, 0, 3, 5][seed as usize % 6]);
                // Blink phase and step: stays, wraps above 1, wraps below 0.
                let phase = [0.2f32, 0.95, -0.3, 0.5][(seed / 6) as usize % 4];
                let step = [0.1f32, 0.2, -0.1, 0.05][(seed / 24) as usize % 4];
                c.bus.write32(this + 0xc8, phase.to_bits());
                c.bus.write32(this + 0xcc, step.to_bits());
                c.bus.write32(this + 0xc0, DATA + 0x300);
                c.bus.write32(this + 0x10, DATA + 0x380);
                oc1023_object(c, OC1023_OBJ);
                oc1023_force_all(c, &mut r, OC1023_OBJ);
                // Unread messages: some, none, or a negative count.
                oc1023_force(c, 0, [3, 0, 0xffff_ffff, 1][(seed / 3) as usize % 4]);
                // The HUD request object: absent, pending, or busy (both set).
                let hud = DATA + 0x3000;
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 7 == 0 { 0 } else { hud },
                );
                c.bus
                    .write32(hud + 0x68, [0, 0, 5, 5][(seed / 5) as usize % 4]);
                c.bus
                    .write32(hud + 0x64, [0, 7, 0, 7][(seed / 2) as usize % 4]);
                oc1023_screen_tables(c, 1 + seed % 3);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_be_options_on_render_matches() {
        fakes::covered(
            ee::s04::f_001fa100__generated,
            oc1023_be_options_on_render,
            64,
            &|c, seed| {
                let mut r = oc1023_machine(c, seed, 0x001f_a100, 12);
                let this = DATA + 0x100;
                c.bus.write32(this + 0x40, DATA + 0x300);
                let world = DATA + 0x500;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x78, DATA + 0x580);
                // 0x534b30 flag, the bytes at 0x535c10 / 0x535c11.
                c.bus.write32(0x0053_4b30, seed & 1);
                c.bus.write8(0x0053_5c11, if seed & 2 != 0 { 2 } else { 0 });
                c.bus.write8(0x0053_5c10, if seed & 4 != 0 { 4 } else { 1 });
                oc1023_object(c, OC1023_OBJ);
                oc1023_force_all(c, &mut r, OC1023_OBJ);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_freestyle_pre_on_activate_matches() {
        fakes::covered(
            ee::s04::f_001fc768__generated,
            oc1023_freestyle_pre_on_activate,
            48,
            &|c, seed| {
                oc1023_machine(c, seed, 0x001f_c768, 12);
                let this = DATA + 0x100;
                let world = DATA + 0x500;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                // The world's mode object starts with 2 or not.
                let mode = DATA + 0x600;
                c.bus.write32(world + 0xc0, mode);
                c.bus.write32(mode, if seed & 1 != 0 { 2 } else { 0 });
                c.bus.write8(0x0053_5c11, if seed & 2 != 0 { 2 } else { 0 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn oc1023_save_replay_shutdown_matches() {
        fakes::covered(
            ee::s05::f_001fde60__generated,
            oc1023_save_replay_shutdown,
            48,
            &|c, seed| {
                oc1023_machine(c, seed, 0x001f_de60, 5);
                let this = DATA + 0x100;
                // The replay-saved flag: 1 (tell the save module) or not.
                c.bus
                    .write32(this + 0x1c8, if seed % 3 == 0 { 0 } else { 1 });
                let world = DATA + 0x500;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                let module = DATA + 0x700;
                c.bus.write32(world + 0x84, module);
                c.bus.write32(module + 0x28, DATA + 0x780);
                oc1023_screen_tables(c, 1 + seed % 3);
                c.g[A0][0] = sx(this);
            },
        );
    }
}
