//! Staging module oc868 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc868`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
///
/// Not ported here: `cFEStateReadMessagePopup_LoadMessages` (0x1a7f78, a
/// `match pc` dispatcher) and `NetFEState_OnNetEvent` (0x1a8b88, has an
/// `__entries` twin); both go through `tools/port_dispatcher.py`.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001a_81f0,
        c_fe_state_read_message_popup_refresh_text,
        "cFEStateReadMessagePopup_RefreshText",
    ),
    (
        0x001a_83d8,
        c_ui_text_scroll_trim_and_scroll,
        "cUITextScroll_TrimAndScroll",
    ),
    (0x001a_8500, net_fe_state_base_ctor, "NetFEState_ctor"),
    (0x001a_85d0, net_fe_state_base_dtor, "NetFEState_dtor"),
    (0x001a_8768, net_fe_state_init, "NetFEState_Init"),
    (
        0x001a_8a40,
        net_fe_state_goto_next_state,
        "NetFEState_GotoNextState",
    ),
];

/// `cFEStateReadMessagePopup_RefreshText(this)` (0x1a81f0): rebuild the
/// popup's text from its message list. Reads the popup's text object at +0x40
/// (nothing to do when it is null), formats each message line into a stack
/// buffer, and hands the result to `cUITextScroll_TrimAndScroll` (0x1a83d8).
pub fn c_fe_state_read_message_popup_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x190);
        frame.save(c, S4, 0x140);
        frame.save(c, S0, 0x180);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x170);
        frame.save(c, S2, 0x160);
        frame.save(c, S3, 0x150);
        frame.save(c, S5, 0x130);
        frame.save(c, S6, 0x120);
        frame.save(c, S7, 0x110);
        frame.save_ra(c, 0x100);
        'exit: {
            c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x460000;
            if t {
                break 'exit;
            }
            'b82d8: {
                c.cycles += 8;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a18));
                call_ptr(c, 0x0031_7670, 0x001a_8230)?;
                c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001a_823c)?;
                c.g[S5][0] = c.g[V0][0];
                let t = c.g[S5][0] == 0u64;
                c.g[A0][0] = 0x460000;
                if t {
                    break 'b82d8;
                }
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6e0));
                c.g[S2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6dc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b82d8;
                }
                c.cycles += 30;
                c.g[A0][0] = 0x460000;
                c.g[S2][0] = c.lw(c.g[S2][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a28));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6d0));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001a_8284)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x001a_8294)?;
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = 0x4a0000;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S3][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1408));
                call_ptr(c, 0x002c_26d0, 0x001a_82b0)?;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a_0e90, 0x001a_82bc)?;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001a_82d4)?;
                c.g[A0][0] = 0x460000;
            }
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19f8));
            call_ptr(c, 0x0031_7670, 0x001a_82e0)?;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001a_82ec)?;
            c.g[S2][0] = c.g[V0][0];
            let t = c.g[S2][0] == 0u64;
            c.g[A0][0] = c.g[S2][0];
            if t {
                break 'exit;
            }
            'b8384: {
                c.cycles += 11;
                c.g[S7][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0xc8));
                c.g[S6][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0xc4));
                c.g[S1][0] = 0u64;
                c.g[S5][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0xc6));
                call_ptr(c, 0x003a_18c0, 0x001a_830c)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6e0));
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6dc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = 0x4a0000;
                if t {
                    break 'b8384;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6dc));
                'l8328: loop {
                    c.cycles += 23;
                    c.g[V1][0] = sx(((c.g[S1][0] as u32) << 2) as u32);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                    call_ptr(c, 0x0041_66f4, 0x001a_8344)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1a50));
                    call_ptr(c, 0x0041_62d0, 0x001a_8350)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0041_62d0, 0x001a_835c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a_19f8, 0x001a_8368)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6e0));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6dc));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6dc));
                        if event_due(c) {
                            deliver_event_here(c, 0x001a_8328);
                        }
                        continue 'l8328;
                    }
                    break 'b8384;
                }
            }
            c.cycles += 10;
            c.g[T2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6e8));
            c.g[A0][0] = c.g[S2][0];
            c.g[A3][0] = c.g[S7][0];
            c.g[T0][0] = c.g[S6][0];
            c.g[T1][0] = c.g[S5][0];
            c.g[A1][0] = 0x28;
            c.g[A2][0] = 0x8;
            c.g[T3][0] = 0u64;
            call_ptr(c, 0x001a_83d8, 0x001a_83a8)?;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6e8), 0u64);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x180);
        frame.restore(c, S1, 0x170);
        frame.restore(c, S2, 0x160);
        frame.restore(c, S3, 0x150);
        frame.restore(c, S4, 0x140);
        frame.restore(c, S5, 0x130);
        frame.restore(c, S6, 0x120);
        frame.restore(c, S7, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `cUITextScroll_TrimAndScroll(this, a1, a2, a3, t0, t1, t2, t3)` (0x1a83d8):
/// clamp a text scroller's offset (+0xc4) and visible span (+0xc6) to the
/// text length (+0xc8). Scrolls by a1 steps through `0x3a1f90` when the
/// scroller is asked to move.
pub fn c_ui_text_scroll_trim_and_scroll(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S5, 0x30);
        c.g[S3][0] = c.g[T0][0];
        frame.save(c, S7, 0x10);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x70);
        c.g[S7][0] = c.g[T1][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        frame.save(c, S6, 0x20);
        let t = c.g[S0][0] == 0u64;
        frame.save_ra(c, 0);
        'epi: {
            if t {
                break 'epi;
            }
            // Section 1a8430: the first-line flag (S6) from a3 == t0 + 1.
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            let t = c.g[A3][0] == c.g[V0][0];
            c.g[S6][0] = 0u64;
            let mut skip_flag = false;
            if !t {
                c.cycles += 2;
                if c.g[T2][0] == 0u64 {
                    c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
                    skip_flag = true;
                }
            }
            if !skip_flag {
                c.cycles += 2;
                c.g[S6][0] = 0x1;
                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
            }
            // Section 1a8468: the scroll distance, and the optional step loop.
            'b8468: {
                c.cycles += 4;
                c.g[S4][0] = (c.g[S3][0] < 0x1u64) as u64;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                let t = (c.g[S2][0] as i64) <= 0;
                c.g[S1][0] = 0u64;
                if t {
                    break 'b8468;
                }
                c.cycles += 2;
                let t = c.g[T3][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b8468;
                }
                c.cycles += 2;
                c.g[S1][0] = c.g[S2][0];
                'l8450: loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x003a_1f90, 0x001a_8458)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S1][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001a_8450);
                        }
                        continue 'l8450;
                    }
                    c.cycles += 1;
                    c.g[S1][0] = c.g[S2][0];
                    break 'b8468;
                }
            }
            // Section 1a8498 (S6 != 0 takes the wrap-around path below).
            c.cycles += 2;
            if c.g[S6][0] != 0u64 {
                c.cycles += 4;
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
                c.g[V0][0] = (c.g[S5][0] < c.g[V1][0]) as u64;
                if c.g[V0][0] != 0u64 {
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S5][0] as u32));
                    c.cycles += 1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc6), c.g[V0][0]);
                } else {
                    c.cycles += 2;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc6), 0u64);
                }
                c.cycles += 3;
                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0xc8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.cycles += 1;
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                break 'epi;
            }
            // Section 1a84ac: no step count, the offset is the remaining span.
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            if c.g[S4][0] != 0u64 {
                c.cycles += 3;
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc6), 0u64);
                break 'epi;
            }
            // Section 1a84c0.
            c.cycles += 2;
            c.g[T1][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[S1][0] as u32));
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 2;
                if (c.g[T1][0] as i64) > 0 {
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc6), c.g[T1][0]);
                } else {
                    c.cycles += 1;
                    c.sh((c.g[S0][0] as u32).wrapping_add(0xc6), 0u64);
                }
            } else {
                c.cycles += 3;
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc6), 0u64);
                c.sh((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                break 'epi;
            }
            c.cycles += 1;
            c.sh((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetFEState_ctor(this)` (0x1a8500): set up the net FE state's fields, its
/// two sub-objects (the first call at 0x39e318, the allocation at 0x317d70)
/// and the back link from the sub-object at +0x6c8.
pub fn net_fe_state_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 51;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e318, 0x001a_8518)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c8));
        c.g[V0][0] = 0x470000;
        c.g[V1][0] = 0x2;
        c.g[A2][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x47f8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
        c.g[A1][0] = 0x4a0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6c0));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x69c), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6a0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6a4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6ac), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6b0), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6b4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6b8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6bc), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6a8), c.g[A2][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x001a_8578)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6c4), 0u64);
        c.g[A1][0] = 0x460000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3218));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x0031_7d70, 0x001a_8594)?;
        c.g[A1][0] = c.g[V0][0];
        frame.restore_ra(c, 0x10);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x6cc), c.g[A1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.sw(c.g[V1][0] as u32, c.g[A1][0]);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6cc));
        frame.restore(c, S1, 0x20);
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x44), 0u64);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `NetFEState_dtor(this, a1)` (0x1a85d0): unlink and free the net FE state's
/// message list (0x6cc is the list head; each node's +8 is a sub-object
/// destroyed through its vtable), then tear down the two sub-objects.
pub fn net_fe_state_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x470000;
        frame.save(c, S2, 0x60);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x47f8));
        frame.save(c, S4, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x80);
        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c8));
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save_ra(c, 0x30);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6cc));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        let t = c.g[V1][0] == c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        'list: {
            if t {
                break 'list;
            }
            c.cycles += 5;
            c.g[S3][0] = c.g[A2][0];
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw(sp(c), c.g[V1][0]);
            'l8628: loop {
                {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw(sp(c).wrapping_add(0x20), c.g[V1][0]);
                    c.g[S0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A0][0] = c.lw(sp(c).wrapping_add(0x20));
                    call_ptr(c, 0x0031_7e50, 0x001a_8650)?;
                    let t = c.g[S1][0] == 0u64;
                    c.sw(sp(c).wrapping_add(0x10), c.g[S0][0]);
                    if !t {
                        c.cycles += 6;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = 0x3;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x001a_8670)?;
                    }
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6cc));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] != c.g[A0][0] {
                    c.sw(sp(c), c.g[V1][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_8628);
                    }
                    continue 'l8628;
                }
                break 'list;
            }
        }
        'b869c: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6b8));
            let t = c.g[A0][0] == 0u64;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c8));
            if t {
                break 'b869c;
            }
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x001a_8698)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x6b8), 0u64);
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_b718, 0x001a_86a4)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        call_ptr(c, 0x0031_7e50, 0x001a_86ac)?;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x0031_8260, 0x001a_86b8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x0039_e390, 0x001a_86c4)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `NetFEState_Init` (0x1a8768): nothing to do, return.
pub fn net_fe_state_init(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `NetFEState_GotoNextState(this)` (0x1a8a40): follow the chain at +0x20 to
/// its last state, then run that state's `OnEnter` (through the vtable slot at
/// +0xd4) and report whether it moved on (1) or not (0).
pub fn net_fe_state_goto_next_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        'lp: loop {
            'b8a68: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    break 'b8a68;
                }
                c.cycles += 2;
                c.g[S0][0] = c.g[V0][0];
                if event_due(c) {
                    deliver_event_here(c, 0x001a_8a54);
                }
                continue 'lp;
            }
            'b8ad0: {
                'b8acc: {
                    c.cycles += 7;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd4));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001a_8a78)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b8acc;
                    }
                    {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
                        call_ptr(c, 0x0031_7670, 0x001a_8a8c)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001a_8a98)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let t = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if !t {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x001a_8ab4)?;
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f4c0, 0x001a_8ac4)?;
                    c.g[V0][0] = 0x1;
                    break 'b8ad0;
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
            }
            break 'lp;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// The globals the replacements read: the BELibrary dummy at `gp-0x1f04`
    /// and the world at `gp-0x848` (whose +0x8c is a text object).
    fn oc868_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x6_8000);
        c.bus.write32(DATA + 0x6_8000 + 0x8c, DATA + 0x6_9000);
        c.bus.write32(DATA + 0x6_9000 + 0x4, DATA + 0x6_9100);
        c.bus.write16(DATA + 0x6_9100 + 0x20, 0x10);
        c.bus
            .write32(DATA + 0x6_9100 + 0x24, crate::difftest::fakes::FAKE + 0x10);
    }

    #[test]
    fn c_fe_state_read_message_popup_refresh_text_matches() {
        check(
            ee::s03::f_001a81f0__generated,
            c_fe_state_read_message_popup_refresh_text,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x8681, 3);
                oc868_globals(c);
                let this = DATA + 0x1_0000;
                c.pc = 0x001a_81f0;
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 0x40, if seed % 4 == 0 { 0 } else { DATA + 0x1_1000 });
                c.bus.write32(this + 0x6d0, DATA + 0x1_3000);
                c.bus.write32(this + 0x6dc, DATA + 0x1_2000);
                c.bus
                    .write32(this + 0x6e0, DATA + 0x1_2000 + 0x10 * (seed % 5));
                c.bus.write32(this + 0x6e8, DATA + 0x1_4000);
                c.bus.write32(DATA + 0x1_2000, DATA + 0x1_5000);
                c.bus.write32(DATA + 0x1_1000 + 0x8, DATA + 0x1_6000);
            },
        );
    }

    #[test]
    fn c_ui_text_scroll_trim_and_scroll_matches() {
        check(
            ee::s03::f_001a83d8__generated,
            c_ui_text_scroll_trim_and_scroll,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x83d8, 2);
                c.pc = 0x001a_83d8;
                let obj = if seed % 6 == 0 { 0 } else { DATA + 0x1_7000 };
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                c.g[T1][0] = (seed % 9) as u64;
                c.g[T2][0] = (seed % 2) as u64;
                c.g[T3][0] = (seed % 3 == 1) as u64;
                c.bus.write16(DATA + 0x1_7000 + 0xc8, (seed % 8) as u16);
                c.bus.write16(DATA + 0x1_7000 + 0xc4, (seed % 6) as u16);
                c.bus.write16(DATA + 0x1_7000 + 0xc6, (seed % 4) as u16);
            },
        );
    }

    /// A net FE state at `DATA + 0x1_0000` with its sub-object links set up.
    fn oc868_net_state(c: &mut Cpu, this: u32) {
        c.bus.write32(this + 0x6cc, DATA + 0x2_0000);
    }

    #[test]
    fn net_fe_state_base_ctor_matches() {
        check(
            ee::s03::f_001a8500__generated,
            net_fe_state_base_ctor,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x8500, 3);
                oc868_globals(c);
                let this = DATA + 0x1_0000;
                c.pc = 0x001a_8500;
                c.g[A0][0] = sx(this);
                c.bus.write32(DATA + 0x2_0000 + 0x4, DATA + 0x3_0000);
                c.bus.write32(this + 0x6c8 + 0x4, DATA + 0x3_0000);
            },
        );
    }

    #[test]
    fn net_fe_state_base_dtor_matches() {
        check(
            ee::s03::f_001a85d0__generated,
            net_fe_state_base_dtor,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x85d0, 3);
                oc868_globals(c);
                let this = DATA + 0x1_0000;
                c.pc = 0x001a_85d0;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                oc868_net_state(c, this);
                // One or two list nodes: node[0] = next, node[4] = link back,
                // node[8] = a sub-object whose +0xc slot is a fake method.
                let head = DATA + 0x2_0000;
                let n1 = DATA + 0x2_1000;
                let n2 = DATA + 0x2_1100;
                c.bus.write32(head, if seed % 2 == 0 { head } else { n1 });
                c.bus.write32(n1, if seed % 2 == 0 { head } else { n2 });
                c.bus.write32(n1 + 4, head);
                c.bus.write32(n1 + 8, DATA + 0x2_2000);
                c.bus.write32(n2, head);
                c.bus.write32(n2 + 4, n1);
                c.bus.write32(n2 + 8, 0);
                c.bus.write32(DATA + 0x2_2000 + 0x4, DATA + 0x2_2100);
                c.bus.write16(DATA + 0x2_2100 + 0x8, 0x10);
                c.bus
                    .write32(DATA + 0x2_2100 + 0xc, crate::difftest::fakes::FAKE + 0x20);
            },
        );
    }

    #[test]
    fn net_fe_state_init_matches() {
        check(
            ee::s03::f_001a8768__generated,
            net_fe_state_init,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x8768, 0);
                c.pc = 0x001a_8768;
            },
        );
    }

    #[test]
    fn net_fe_state_goto_next_state_matches() {
        check(
            ee::s03::f_001a8a40__generated,
            net_fe_state_goto_next_state,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x8a40, 2);
                c.pc = 0x001a_8a40;
                let this = DATA + 0x1_0000;
                c.g[A0][0] = sx(this);
                // Chain of one or two states through +0x20.
                let next = DATA + 0x1_1000;
                c.bus
                    .write32(this + 0x20, if seed % 2 == 0 { 0 } else { next });
                c.bus.write32(next + 0x20, 0);
                c.bus.write32(this + 0x8, DATA + 0x1_2000);
                c.bus.write16(DATA + 0x1_2000 + 0xd0, 0x8);
                c.bus
                    .write32(DATA + 0x1_2000 + 0xd4, crate::difftest::fakes::FAKE + 0x30);
                c.bus.write32(this + 0x40, DATA + 0x1_3000);
                c.bus.write32(this + 0x10, DATA + 0x1_4000);
                c.bus.write32(next + 0x8, DATA + 0x1_2000);
            },
        );
    }
}
