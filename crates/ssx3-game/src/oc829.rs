//! Staging module oc829 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc829`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_4828,
        cfe_state_uber_trick_show_execute_info,
        "cFEStateUberTrick_ShowExecuteInfo",
    ),
    (
        0x0018_49b0,
        cfe_state_uber_trick_init,
        "cFEStateUberTrick_Init",
    ),
    (
        0x0018_4c60,
        cfe_state_uber_trick_update,
        "cFEStateUberTrick_Update",
    ),
    (
        0x0018_4f40,
        cfe_state_uber_trick_on_child_event,
        "cFEStateUberTrick_OnChildEvent",
    ),
    (
        0x0018_5080,
        cfe_state_uber_trick_on_widget_created,
        "cFEStateUberTrick_OnWidgetCreated",
    ),
];

/// `void cFEStateUberTrick_ShowExecuteInfo(cFEStateUberTrick* self, int on)` (0x184828): show or hide the
/// six widgets of the left panel (`To_Execute_info`, `Checkboxes`, `MenuTrick`, `InstBtn0..2`), found by hash name
/// in the first screen of the state, with the widget's `setVisible(on)` virtual.
pub fn cfe_state_uber_trick_show_execute_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_184880: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00184844);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x00397840, 0x00184844)?;
            c.g[A0][0] = 0x460000;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184854);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2608));
            call_ptr(c, 0x00317670, 0x00184854)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184860);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184860)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184880;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184880);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184880)?;
                break 'b_184880;
            }
        }
        'b_1848b8: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x0018488c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25f8));
            call_ptr(c, 0x00317670, 0x0018488c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184898);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184898)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1848b8;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001848b8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001848b8)?;
                break 'b_1848b8;
            }
        }
        'b_1848f0: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001848c4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25e8));
            call_ptr(c, 0x00317670, 0x001848c4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001848d0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x001848d0)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1848f0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001848f0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001848f0)?;
                break 'b_1848f0;
            }
        }
        'b_184928: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001848fc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26a0));
            call_ptr(c, 0x00317670, 0x001848fc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184908);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184908)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184928;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184928);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184928)?;
                break 'b_184928;
            }
        }
        'b_184960: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00184934);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2690));
            call_ptr(c, 0x00317670, 0x00184934)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184940);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184940)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184960;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184960);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184960)?;
                break 'b_184960;
            }
        }
        'b_184998: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x0018496c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2680));
            call_ptr(c, 0x00317670, 0x0018496c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00184978);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00184978)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184998;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184998);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184998)?;
                break 'b_184998;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_Init(cFEStateUberTrick* self)` (0x1849b0): add the `btnmap` screen, set the rider's
/// character in the front-end rider manager and the preview camera, look up the checkbox, checkmark and dollar
/// textures, then hide cost and buy buttons.
pub fn cfe_state_uber_trick_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_184a04: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25d8));
            c.g[RA][0] = sx(0x001849d8);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001849d8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001849ec);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001849ec)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_184a04;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00184a04);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00184a04)?;
                break 'b_184a04;
            }
        }
        c.cycles += 91;
        c.g[RA][0] = sx(0x00184a0c);
        call_ptr(c, 0x0014dc80, 0x00184a0c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00184a18);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x00184a18)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[RA][0] = sx(0x00184a28);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00147488, 0x00184a28)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x00184a44);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00184a44)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A3][0] = 0u64;
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
        c.g[RA][0] = sx(0x00184a60);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a0508, 0x00184a60)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A2][0] = 0u64;
        c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[RA][0] = sx(0x00184a78);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        call_ptr(c, 0x001a0570, 0x00184a78)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = c.g[SP][0];
        c.g[AT][0] = 0x42a00000;
        c.mtc1(1, 2);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x44250000;
        c.mtc1(1, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.sw(c.g[SP][0] as u32, 0u64);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        c.g[RA][0] = sx(0x00184ac8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        call_ptr(c, 0x0015e050, 0x00184ac8)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59f4));
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25c8));
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x00184ae0);
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x00317670, 0x00184ae0)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[RA][0] = sx(0x00184aec);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00398380, 0x00184aec)?;
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[V0][0]);
        c.g[RA][0] = sx(0x00184b00);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25b8));
        call_ptr(c, 0x00317670, 0x00184b00)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[RA][0] = sx(0x00184b0c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00398380, 0x00184b0c)?;
        c.g[A0][0] = 0x460000;
        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[RA][0] = sx(0x00184b20);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25a8));
        call_ptr(c, 0x00317670, 0x00184b20)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
        c.g[RA][0] = sx(0x00184b2c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00398380, 0x00184b2c)?;
        c.g[RA][0] = sx(0x00184b34);
        c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
        call_ptr(c, 0x0028b180, 0x00184b34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00184b40);
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x0028f140, 0x00184b40)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00184b4c);
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x00184668,
            f: ee::s02::f_00184668,
        }
        .call(c, 0x00184b4c)?; // cFEStateUberTrick_ShowCost
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x00184b58);
        c.g[A1][0] = 0u64;
        Callee {
            addr: 0x00184780,
            f: ee::s02::f_00184780,
        }
        .call(c, 0x00184b58)?; // cFEStateUberTrick_ShowBuyButtons
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_Update(cFEStateUberTrick* self)` (0x184c60): the first update with an active rider
/// record (`+0x48` clear): place the three preview camera points (rotated by the rider's quaternion, via VU0) and
/// make the preview visible, then run the base state's update.
pub fn cfe_state_uber_trick_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_184f08: {
            c.cycles += 15;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x48));
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_184f08;
            } else {
                'b_184cd4: {
                    c.cycles += 9;
                    c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x00184cac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0548, 0x00184cac)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0u64;
                    if t {
                        break 'b_184cd4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb8));
                        let t = c.g[V0][0] == 0u64;

                        if t {
                            break 'b_184cd4;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb4));
                            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                            break 'b_184cd4;
                        }
                    }
                }
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64;
                c.g[FP][0] = 0x1;
                if t {
                    break 'b_184f08;
                } else {
                    'b_184d08: {
                        c.cycles += 3;
                        c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                        let t = c.g[S0][0] == 0u64;
                        c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), c.g[FP][0]);
                        if t {
                            break 'b_184d08;
                        } else {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x00184cf0);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00311a50, 0x00184cf0)?;
                            c.g[AT][0] = 0xffffffffbf800000;
                            c.mtc1(1, 12);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0x1b4;
                            c.g[RA][0] = sx(0x00184d08);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x003128e8, 0x00184d08)?;
                            break 'b_184d08;
                        }
                    }
                    c.cycles += 128;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[AT][0] = 0xffffffffc3640000;
                    c.mtc1(1, 0);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                    c.g[AT][0] = 0xffffffffc34d0000;
                    c.mtc1(1, 1);
                    c.g[S0][0] = 0x500000;
                    c.g[AT][0] = 0x427c0000;
                    c.mtc1(1, 2);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = 0x500000;
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xeb0));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.g[S4][0] = 0x500000;
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
                    c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xea0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x48), c.g[FP][0]);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x2c));
                    c.sq(2, c.g[SP][0] as u32);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                    c.g[RA][0] = sx(0x00184d80);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                    call_ptr(c, 0x0031be50, 0x00184d80)?;
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x78));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7c));
                    c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xec0));
                    c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
                    c.mul_s(1, 0, 1);
                    c.mul_s(2, 0, 2);
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                    c.mul_s(0, 0, 4);
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x5c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                    c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.mtc1(0, 12);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00184dfc);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x0031be50, 0x00184dfc)?;
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                    c.lwc1(2, (c.g[S2][0] as u32).wrapping_sub(0xeb0));
                    c.lwc1(4, (c.g[S3][0] as u32).wrapping_add(0x4));
                    c.mul_s(1, 0, 1);
                    c.mul_s(2, 0, 2);
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                    c.mul_s(0, 0, 4);
                    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x59f0));
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                    c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00184e78);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    call_ptr(c, 0x0031be50, 0x00184e78)?;
                    c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S6][0];
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(2, (c.g[S4][0] as u32).wrapping_sub(0xea0));
                    c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x4));
                    c.mul_s(1, 0, 1);
                    c.mul_s(2, 0, 2);
                    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
                    c.mul_s(0, 0, 4);
                    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                    c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x00184ef0);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    call_ptr(c, 0x0019f3e8, 0x00184ef0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A2][0] = 0x1;
                    c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                    c.g[RA][0] = sx(0x00184f08);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                    call_ptr(c, 0x001a0570, 0x00184f08)?;
                    break 'b_184f08;
                }
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x00184f10);
        c.g[A0][0] = c.g[S7][0];
        call_ptr(c, 0x0039e510, 0x00184f10)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_OnChildEvent(self, child, event)` (0x184f40): events 0x15 and 0x16 refresh the trick
/// list; 0x16 also re-applies the selected trick (rider preview, mute/trick screens) and the cost line.
pub fn cfe_state_uber_trick_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_185068: {
            'b_185064: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V0][0] = 0x15;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S3][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[A2][0] == c.g[V0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_185064;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x16;
                    let t = c.g[A2][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_185068;
                    } else {
                        'b_18505c: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_18505c;
                            } else {
                                'b_185054: {
                                    c.cycles += 48;
                                    c.g[RA][0] = sx(0x00184f88);
                                    call_ptr(c, 0x0014dc80, 0x00184f88)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00184f94);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x0014dd58, 0x00184f94)?;
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[RA][0] = sx(0x00184fa0);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00147488, 0x00184fa0)?;
                                    c.g[RA][0] = sx(0x00184fa8);
                                    c.g[S0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x00184fa8)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00184fb4);
                                    c.g[A1][0] = 0xb;
                                    call_ptr(c, 0x0014dd58, 0x00184fb4)?;
                                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[RA][0] = sx(0x00184fc8);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00150b48, 0x00184fc8)?;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4c));
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                    c.g[S1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.g[RA][0] = sx(0x00184fe4);
                                    c.g[S2][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0014dc80, 0x00184fe4)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00184ff0);
                                    c.g[A1][0] = 0x6;
                                    call_ptr(c, 0x0014dd58, 0x00184ff0)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x00185008);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0014fe08, 0x00185008)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0018501c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018501c)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x00185028);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26d0));
                                    call_ptr(c, 0x00317670, 0x00185028)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x00185034);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x00185034)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    let t = c.g[S0][0] == 0u64;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_185054;
                                    } else {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x00185048);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2590));
                                        call_ptr(c, 0x00317670, 0x00185048)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00185054);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x00185054)?;
                                        break 'b_185054;
                                    }
                                }
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x0018505c);
                                c.g[A0][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x00184520,
                                    f: ee::s02::f_00184520,
                                }
                                .call(c, 0x0018505c)?; // cFEStateUberTrick_UpdateCost
                                break 'b_18505c;
                            }
                        }
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x00185064);
                        c.g[A0][0] = c.g[S3][0];
                        Callee {
                            addr: 0x00183de0,
                            f: ee::s02::f_00183de0,
                        }
                        .call(c, 0x00185064)?; // cFEStateUberTrick_RefreshTrickList
                        break 'b_185064;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_185068;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateUberTrick_OnWidgetCreated(self, widget)` (0x185080): give the trick-name text widgets their
/// label and trick id (`mMiscData`), and remember the two menus.
pub fn cfe_state_uber_trick_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = 0u64;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S4][0] = 0x4a0000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S5][0] = 0x4a0000;
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S6][0] = 0x4a0000;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S7][0] = 0x460000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[FP][0] = 0x4a0000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[V0][0] = 0x4a0000;
        'l_1850d0: loop {
            'b_185238: {
                'b_185224: {
                    'b_185208: {
                        'b_1851f0: {
                            'b_1851dc: {
                                'b_1851c4: {
                                    'b_1851ac: {
                                        'b_185194: {
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[RA][0] = sx(0x001850dc);
                                            c.g[A0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x1468));
                                            call_ptr(c, 0x00317670, 0x001850dc)?;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[A1][0] = 0x4a0000;
                                            if t {
                                                break 'b_185194;
                                            } else {
                                                c.cycles += 5;
                                                c.g[S0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.g[RA][0] = sx(0x001850f0);
                                                c.g[A0][0] =
                                                    sx((c.g[FP][0] as u32).wrapping_add(0x1478));
                                                call_ptr(c, 0x00317670, 0x001850f0)?;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.g[A1][0] = 0x4a0000;
                                                if t {
                                                    break 'b_1851ac;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x00185104);
                                                    c.g[A0][0] =
                                                        sx((c.g[S7][0] as u32)
                                                            .wrapping_sub(0x2578));
                                                    call_ptr(c, 0x00317670, 0x00185104)?;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.g[A1][0] = 0x460000;
                                                    if t {
                                                        break 'b_1851c4;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x00185118);
                                                        c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                            .wrapping_add(0x1488));
                                                        call_ptr(c, 0x00317670, 0x00185118)?;
                                                        let t = c.g[S0][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1851dc;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[S0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[RA][0] = sx(0x0018512c);
                                                            c.g[A0][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x1498));
                                                            call_ptr(c, 0x00317670, 0x0018512c)?;
                                                            let t = c.g[S0][0] == c.g[V0][0];
                                                            c.g[A1][0] = 0x460000;
                                                            if t {
                                                                break 'b_1851f0;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x38));
                                                                c.g[RA][0] = sx(0x00185140);
                                                                c.g[A0][0] = sx((c.g[S4][0]
                                                                    as u32)
                                                                    .wrapping_add(0x14a0));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x00185140,
                                                                )?;
                                                                let t = c.g[S0][0] == c.g[V0][0];
                                                                c.g[S2][0] = sx((c.g[S2][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                if t {
                                                                    break 'b_185208;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S2][0]
                                                                        as i64)
                                                                        < 6i64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = 0x4a0000;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x001850d0,
                                                                            );
                                                                        }
                                                                        continue 'l_1850d0;
                                                                    } else {
                                                                        'b_185178: {
                                                                            c.cycles += 6;
                                                                            c.g[A0][0] = 0x4a0000;
                                                                            c.g[S0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x38,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x00185164);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x14a8,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x00317670,
                                                                                0x00185164,
                                                                            )?;
                                                                            if c.g[S0][0]
                                                                                != c.g[V0][0]
                                                                            {
                                                                                c.g[A0][0] =
                                                                                    0x460000;
                                                                                break 'b_185178;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = 0xffffffffffffffff;
                                                                                let t = true;
                                                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_185238;
                                                                                } else {
                                                                                    break 'b_185178;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[S0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        c.g[RA][0] = sx(0x00185184);
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x2538,
                                                                                ));
                                                                        call_ptr(
                                                                            c, 0x00317670,
                                                                            0x00185184,
                                                                        )?;
                                                                        if c.g[S0][0] != c.g[V0][0]
                                                                        {
                                                                            c.g[A0][0] = 0x460000;
                                                                            break 'b_185224;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.sw(
                                                                                (c.g[S3][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4c,
                                                                                    ),
                                                                                c.g[S1][0],
                                                                            );
                                                                            if t {
                                                                                break 'b_185238;
                                                                            } else {
                                                                                break 'b_185194;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001851a0);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1470));
                                        call_ptr(c, 0x003a0c50, 0x001851a0)?;
                                        c.g[V0][0] = 0x1;
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                        if t {
                                            break 'b_185238;
                                        } else {
                                            break 'b_1851ac;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001851b8);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1480));
                                    call_ptr(c, 0x003a0c50, 0x001851b8)?;
                                    c.g[V0][0] = 0x3;
                                    let t = true;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                    if t {
                                        break 'b_185238;
                                    } else {
                                        break 'b_1851c4;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001851d0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2568));
                                call_ptr(c, 0x003a0c50, 0x001851d0)?;
                                c.g[V0][0] = 0x2;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                if t {
                                    break 'b_185238;
                                } else {
                                    break 'b_1851dc;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001851e8);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1490));
                            call_ptr(c, 0x003a0c50, 0x001851e8)?;
                            let t = true;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                            if t {
                                break 'b_185238;
                            } else {
                                break 'b_1851f0;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001851fc);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2558));
                        call_ptr(c, 0x003a0c50, 0x001851fc)?;
                        c.g[V0][0] = 0x4;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                        if t {
                            break 'b_185238;
                        } else {
                            break 'b_185208;
                        }
                    }
                    c.cycles += 7;
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00185218);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2548));
                    call_ptr(c, 0x003a0c50, 0x00185218)?;
                    c.g[V0][0] = 0x9;
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    if t {
                        break 'b_185238;
                    } else {
                        break 'b_185224;
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x00185230);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25e8));
                call_ptr(c, 0x00317670, 0x00185230)?;
                if c.g[S0][0] == c.g[V0][0] {
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                    break 'b_185238;
                } else {
                    break 'b_185238;
                }
            }
            c.cycles += 12;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
            c.pc = t;
            return Ok(());
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::Rng;
    use crate::difftest::{DATA, GAME_GP};
    use crate::game::main::loadscreens::tests::{oc736_fake_call, oc736_force, oc736_force_zero};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and its magic (both machines of a diff get their own).
    const OC829_CTRL: u32 = DATA + 0x2f_0000;
    const OC829_MAGIC: u32 = 0x4f43_3832;
    /// Hash table of the `GetHashValue32` fake: the result for the string at
    /// `a0` is the word at `OC829_HASH_TAB + (a0 & 0xffffc)` (0: a pointer).
    const OC829_HASH_TAB: u32 = DATA + 0x50_0000;

    const OC829_SELF: u32 = DATA + 0x21_0000;
    const OC829_WORLD: u32 = DATA + 0x22_0000;
    const OC829_RM: u32 = DATA + 0x23_0000;
    const OC829_PLAYER: u32 = DATA + 0x24_0000;
    const OC829_ENG: u32 = DATA + 0x25_0000;
    const OC829_IFACE: u32 = DATA + 0x26_0000;
    const OC829_IFVT: u32 = DATA + 0x27_0000;
    const OC829_WIDGET: u32 = DATA + 0x28_0000;
    const OC829_WVT: u32 = DATA + 0x29_0000;
    const OC829_CHILD: u32 = DATA + 0x2a_0000;
    const OC829_PANEL_A: u32 = DATA + 0x2b_0000;
    const OC829_PANEL_B: u32 = DATA + 0x2c_0000;
    const OC829_REC_A: u32 = DATA + 0x2d_0000;
    const OC829_REC_B: u32 = DATA + 0x2e_0000;
    const OC829_SCREEN: u32 = DATA + 0x2e_8000;

    macro_rules! oc829_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC829_CTRL) != OC829_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc829_fakes!(
        oc829_install_generic;
        (oc829_fake_397840, OC829_PREV_397840, ee::s13::OVERRIDE_00397840, ee::s13::f_00397840__generated, 0x0039_7840),
        (oc829_fake_39d860, OC829_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc829_fake_397cc0, OC829_PREV_397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x0039_7cc0),
        (oc829_fake_39c870, OC829_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc829_fake_398380, OC829_PREV_398380, ee::s13::OVERRIDE_00398380, ee::s13::f_00398380__generated, 0x0039_8380),
        (oc829_fake_39e510, OC829_PREV_39E510, ee::s13::OVERRIDE_0039E510, ee::s13::f_0039e510__generated, 0x0039_e510),
        (oc829_fake_3a1000, OC829_PREV_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (oc829_fake_3a0c50, OC829_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (oc829_fake_311a50, OC829_PREV_311A50, ee::s10::OVERRIDE_00311A50, ee::s10::f_00311a50__generated, 0x0031_1a50),
        (oc829_fake_3128e8, OC829_PREV_3128E8, ee::s10::OVERRIDE_003128E8, ee::s10::f_003128e8__generated, 0x0031_28e8),
        (oc829_fake_31be50, OC829_PREV_31BE50, ee::s10::OVERRIDE_0031BE50, ee::s10::f_0031be50__generated, 0x0031_be50),
        (oc829_fake_14dc80, OC829_PREV_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014_dc80),
        (oc829_fake_14dd58, OC829_PREV_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014_dd58),
        (oc829_fake_147488, OC829_PREV_147488, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated, 0x0014_7488),
        (oc829_fake_15e050, OC829_PREV_15E050, ee::s01::OVERRIDE_0015E050, ee::s01::f_0015e050__generated, 0x0015_e050),
        (oc829_fake_150b48, OC829_PREV_150B48, ee::s01::OVERRIDE_00150B48, ee::s01::f_00150b48__generated, 0x0015_0b48),
        (oc829_fake_14fe08, OC829_PREV_14FE08, ee::s01::OVERRIDE_0014FE08, ee::s01::f_0014fe08__generated, 0x0014_fe08),
        (oc829_fake_1a0508, OC829_PREV_1A0508, ee::s03::OVERRIDE_001A0508, ee::s03::f_001a0508__generated, 0x001a_0508),
        (oc829_fake_1a0570, OC829_PREV_1A0570, ee::s03::OVERRIDE_001A0570, ee::s03::f_001a0570__generated, 0x001a_0570),
        (oc829_fake_19f3e8, OC829_PREV_19F3E8, ee::s03::OVERRIDE_0019F3E8, ee::s03::f_0019f3e8__generated, 0x0019_f3e8),
        (oc829_fake_1a0548, OC829_PREV_1A0548, ee::s03::OVERRIDE_001A0548, ee::s03::f_001a0548__generated, 0x001a_0548),
        (oc829_fake_28b180, OC829_PREV_28B180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated, 0x0028_b180),
        (oc829_fake_28f140, OC829_PREV_28F140, ee::s07::OVERRIDE_0028F140, ee::s07::f_0028f140__generated, 0x0028_f140),
        (oc829_fake_183de0, OC829_PREV_183DE0, ee::s02::OVERRIDE_00183DE0, ee::s02::f_00183de0__generated, 0x0018_3de0),
        (oc829_fake_184520, OC829_PREV_184520, ee::s02::OVERRIDE_00184520, ee::s02::f_00184520__generated, 0x0018_4520),
    );

    static OC829_PREV_317670: AtomicUsize = AtomicUsize::new(0);

    /// `GetHashValue32`: the hash table's value for the string, else the fake's pointer.
    fn oc829_fake_317670(c: &mut Cpu) {
        if c.bus.read32(OC829_CTRL) != OC829_MAGIC {
            return fakes::fall_back(c, &OC829_PREV_317670, ee::s10::f_00317670__generated);
        }
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        oc736_fake_call(c, 0x0031_7670);
        if c.pc == ra {
            let h = c.bus.read32(OC829_HASH_TAB + (a0 & 0xf_fffc));
            if h != 0 {
                c.g[V0][0] = h as u64;
            }
        }
    }

    fn oc829_install_fakes() {
        oc829_install_generic();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc829_fake_317670,
            &OC829_PREV_317670,
        );
    }

    /// Scrambled machine with the fakes' gate open and the world (`gp-0x848`)
    /// with its rider manager (`+0x7c`).
    fn oc829_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC829_CTRL, OC829_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC829_WORLD);
        c.bus.write32(OC829_WORLD + 0x7c, OC829_RM);
        r
    }

    /// A widget whose virtual `setVisible` slot (`+0x48` delta, `+0x4c` function) is a fake call.
    fn oc829_widget(c: &mut Cpu, seed: u32) {
        c.bus.write32(OC829_WIDGET + 8, OC829_WVT);
        c.bus.write16(OC829_WVT + 0x48, 0x10 * (seed % 3) as u16);
        c.bus.write32(OC829_WVT + 0x4c, FAKE);
    }

    /// The back-end interface the fakes of `0x14dd58` return: vtable at `+0xc`
    /// whose slots `+0x8/+0xc` and `+0x10/+0x14` are fake calls.
    fn oc829_iface(c: &mut Cpu, seed: u32) {
        oc736_force(c, 0x0014_dd58, OC829_IFACE);
        c.bus.write32(OC829_IFACE + 0xc, OC829_IFVT);
        for off in [0x8u32, 0x10] {
            c.bus.write16(OC829_IFVT + off, 0x10 * (seed % 3) as u16);
            c.bus.write32(OC829_IFVT + off + 4, FAKE + 0x10);
        }
    }

    /// `ShowExecuteInfo(self, on)`: the six widgets of the first screen, all
    /// missing (seed % 4 == 0) or present with a `setVisible` fake call.
    #[test]
    fn cfe_state_uber_trick_show_execute_info_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_00184828__generated,
            cfe_state_uber_trick_show_execute_info,
            60,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_4828, 14);
                oc829_widget(c, seed);
                if seed % 4 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC829_WIDGET);
                }
                c.g[A0][0] = sx(OC829_SELF);
                c.g[A1][0] = (seed % 3) as u64;
                c.pc = 0x0018_4828;
            },
        );
    }

    /// `Init(self)`: the `btnmap` screen (missing or present), the rider's
    /// character, the camera set-up, the three texture lookups and the real
    /// `ShowCost`/`ShowBuyButtons` hiding their widgets.
    #[test]
    fn cfe_state_uber_trick_init_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_001849b0__generated,
            cfe_state_uber_trick_init,
            60,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_49b0, 30);
                oc829_widget(c, seed);
                oc829_iface(c, seed);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                } else {
                    oc736_force(c, 0x0039_7cc0, OC829_SCREEN);
                }
                if seed % 2 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC829_WIDGET);
                }
                c.bus.write32(OC829_SELF + 0x10, OC829_ENG);
                c.bus.write8(OC829_SELF + 0x44, (seed % 4) as u8);
                c.g[A0][0] = sx(OC829_SELF);
                c.pc = 0x0018_49b0;
            },
        );
    }

    /// `Update(self)`: the first update (`+0x48` clear) with a rider record
    /// that is, or is not, ready (`+0` below 10, `+0xcb8` set, `+0xcb4` > 0),
    /// with or without a camera object (`+0xc`); then the base update.
    #[test]
    fn cfe_state_uber_trick_update_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_00184c60__generated,
            cfe_state_uber_trick_update,
            160,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_4c60, 16);
                oc736_force(c, 0x001a_0548, OC829_PLAYER);
                c.bus
                    .write32(OC829_SELF + 0x48, if (seed / 32) % 3 == 0 { 1 } else { 0 });
                c.bus.write8(OC829_SELF + 0x44, (seed % 4) as u8);
                c.bus
                    .write32(OC829_PLAYER, if seed % 2 == 0 { 3 } else { 20 });
                c.bus.write32(OC829_PLAYER + 0xcb8, (seed / 2) % 2);
                let cb4 = match (seed / 4) % 3 {
                    0 => 0,
                    1 => 5,
                    _ => u32::MAX,
                };
                c.bus.write32(OC829_PLAYER + 0xcb4, cb4);
                c.bus.write32(
                    OC829_PLAYER + 0xc,
                    if (seed / 16) % 2 == 0 { 0 } else { OC829_ENG },
                );
                c.g[A0][0] = sx(OC829_SELF);
                c.pc = 0x0018_4c60;
            },
        );
    }

    /// `OnChildEvent(self, child, event)`: events 0x15, 0x16 (with the child's
    /// `+0x6c` set or clear, the `btnmap` widget present or missing) and one
    /// other event.
    #[test]
    fn cfe_state_uber_trick_on_child_event_matches() {
        oc829_install_fakes();
        check(
            ee::s02::f_00184f40__generated,
            cfe_state_uber_trick_on_child_event,
            64,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_4f40, 30);
                oc829_iface(c, seed);
                oc829_widget(c, seed);
                if (seed / 8) % 2 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC829_WIDGET);
                }
                c.bus.write32(OC829_SELF + 0x40, OC829_SCREEN);
                c.bus.write8(OC829_SELF + 0x44, (seed % 4) as u8);
                c.bus.write32(OC829_SELF + 0x4c, OC829_PANEL_A);
                c.bus.write32(OC829_SELF + 0x50, OC829_PANEL_B);
                c.bus.write32(OC829_PANEL_A + 0xa0, OC829_REC_A);
                c.bus.write32(OC829_PANEL_B + 0xa0, OC829_REC_B);
                c.bus.write8(OC829_REC_A + 0x18, (seed % 7) as u8);
                c.bus.write8(OC829_REC_B + 0x18, (seed % 5) as u8);
                c.bus.write32(OC829_CHILD + 0x48, 0x1000 + seed);
                c.bus.write32(OC829_CHILD + 0x6c, (seed / 4) % 2);
                c.g[A0][0] = sx(OC829_SELF);
                c.g[A1][0] = sx(OC829_CHILD);
                c.g[A2][0] = [0x15u64, 0x16, 0x17, 0x16][(seed % 4) as usize];
                c.pc = 0x0018_4f40;
            },
        );
    }

    /// `OnWidgetCreated(self, widget)`: the widget's hash name matches one of
    /// the nine names (six trick labels, `done`, `MenuMain`, `MenuTrick`) or
    /// none. Each name's string address hashes to its own value.
    #[test]
    fn cfe_state_uber_trick_on_widget_created_matches() {
        oc829_install_fakes();
        const NAMES: [u32; 9] = [
            0x004a_1468,
            0x004a_1478,
            0x0045_da88,
            0x004a_1488,
            0x004a_1498,
            0x004a_14a0,
            0x004a_14a8,
            0x0045_dac8,
            0x0045_da18,
        ];
        check(
            ee::s02::f_00185080__generated,
            cfe_state_uber_trick_on_widget_created,
            33,
            &|c, seed| {
                let _ = oc829_fixture(c, seed, 0x0018_5080, 20);
                for (i, s) in NAMES.into_iter().enumerate() {
                    c.bus
                        .write32(OC829_HASH_TAB + (s & 0xf_fffc), 0x1000 + 0x10 * i as u32);
                }
                let k = (seed % 11) as usize;
                let name = if k < 9 {
                    0x1000 + 0x10 * k as u32
                } else {
                    0x7777
                };
                c.bus.write32(OC829_WIDGET + 0x38, name);
                c.g[A0][0] = sx(OC829_SELF);
                c.g[A1][0] = sx(OC829_WIDGET);
                c.pc = 0x0018_5080;
            },
        );
    }
}
