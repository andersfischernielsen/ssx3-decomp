//! Staging module oc838 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc838`.

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
        0x0018_c478,
        oc838_options_hud_refresh_selection,
        "cFEStateOptionsHUD_RefreshSelection",
    ),
    (
        0x0018_c6a0,
        oc838_options_controller_init,
        "cFEStateOptionsController_Init",
    ),
    (
        0x0018_c910,
        oc838_options_controller_on_child_event,
        "cFEStateOptionsController_OnChildEvent",
    ),
    (
        0x0018_cce0,
        oc838_options_controller_on_widget_notify,
        "cFEStateOptionsController_OnWidgetNotify",
    ),
    (
        0x0018_cdc0,
        oc838_options_controller_refresh_list_boxes,
        "cFEStateOptionsController_RefreshListBoxes",
    ),
    (
        0x0018_cf50,
        oc838_options_controller_show_config_text,
        "cFEStateOptionsController_ShowConfigText",
    ),
];

/// `cFEStateOptionsController_SetVibration` (0x18d108, not ported).
const OC838_SET_VIBRATION: Callee = Callee {
    addr: 0x0018_d108,
    f: ee::s02::f_0018d108,
};

/// `cFEStateOptionsController_RefreshListBoxes` (0x18cdc0).
const OC838_REFRESH_LIST_BOXES: Callee = Callee {
    addr: 0x0018_cdc0,
    f: ee::s02::f_0018cdc0,
};

/// `cFEStateOptionsController_ShowConfigText` (0x18cf50).
const OC838_SHOW_CONFIG_TEXT: Callee = Callee {
    addr: 0x0018_cf50,
    f: ee::s02::f_0018cf50,
};

/// `lw off(reg)` / `lh off(reg)` with the register's low 32 bits.
macro_rules! lwr {
    ($c:expr, $reg:expr, $off:expr) => {{
        let base = $c.g[$reg][0] as u32;
        $c.lw(base.wrapping_add($off))
    }};
}

macro_rules! lhr {
    ($c:expr, $reg:expr, $off:expr) => {{
        let base = $c.g[$reg][0] as u32;
        $c.lh(base.wrapping_add($off))
    }};
}

/// `sx(base + off)` (`addiu`).
fn add_at(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `SetVibration(self, pad, 0.0f, 0.0f)` for pad 0 and pad 1 (the generated
/// pair of calls with `f20 = 0`); `self` is in `self_reg`, the first call
/// returns to `ret0`, the second to `ret1` (f13 taken from f12 the second
/// time, as the generated body does).
fn oc838_vibrate_both(c: &mut Cpu, self_reg: usize, ret0: u32, ret1: u32) -> Flow {
    c.mtc1(0, 20);
    c.g[A0][0] = c.g[self_reg][0];
    c.g[A1][0] = 0;
    c.mov_s(12, 20);
    c.mov_s(13, 20);
    OC838_SET_VIBRATION.call(c, ret0)?;
    c.mov_s(12, 20);
    c.g[A0][0] = c.g[self_reg][0];
    c.g[A1][0] = 1;
    c.mov_s(13, 12);
    OC838_SET_VIBRATION.call(c, ret1)
}

/// `GetHashValue32(a0 + add)` then `UIScreen_FindObject(self->mpScreen, hash)`
/// (screen at `s1 + 0x40`); the object lands in `s0`. True when it exists.
fn oc838_find_widget(
    c: &mut Cpu,
    pre: u64,
    add: u32,
    ret_hash: u32,
    ret_find: u32,
) -> Result<bool, Unwound> {
    c.cycles += pre;
    c.g[A0][0] = add_at(c.g[A0][0], add);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A0][0] = lwr!(c, S1, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)?;
    c.g[S0][0] = c.g[V0][0];
    Ok(c.g[S0][0] != 0)
}

/// Add the two entries (`ListBox_AddEntryByStringID(s0, hash(str), 0 / 1)`)
/// whose strings are at `0x460000 - lo1` and `0x460000 - lo2`.
fn oc838_add_two_entries(c: &mut Cpu, pre: u64, lo1: u32, lo2: u32, rets: [u32; 4]) -> Flow {
    c.cycles += pre;
    c.g[A0][0] = 0x46_0000;
    c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(lo1));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a568, rets[1])?;
    c.g[A0][0] = 0x46_0000;
    c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(lo2));
    call_ptr(c, 0x0031_7670, rets[2])?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = 1;
    call_ptr(c, 0x0039_a568, rets[3])
}

/// A widget's virtual `setVisible`-style call: `vt = *(obj + 8)`, call
/// `vt[+0x4c]` with `this = obj + *(short*)(vt + 0x48)` and `a1` as set.
fn oc838_widget_vcall(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V0][0] = lwr!(c, A2, 8);
    c.g[A0][0] = lhr!(c, V0, 0x48);
    c.g[V1][0] = lwr!(c, V0, 0x4c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = add_at(c.g[A2][0], c.g[A0][0] as u32);
    call_ptr(c, target, ret)
}

/// `void cFEStateOptionsHUD_RefreshSelection(cFEStateOptionsHUD* self)`
/// (0x18c478): for the three rows "1x", "2x", "3x" of the HUD's "Menu"
/// widget, store the row index in the item, then highlight the item whose
/// index equals the current option mode (the top two bits of the first word
/// of the option block at 0x535610; that block is copied to the stack by an
/// inlined struct copy) and select it in the menu; the others get the
/// normal (0) state. The menu's "busy" bit (`+0x14 & 1`) is cleared at the end.
pub fn oc838_options_hud_refresh_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x300);
        frame.save(c, S0, 0x2f0);
        frame.save(c, S3, 0x2c0);
        frame.save(c, S1, 0x2e0);
        frame.save(c, S2, 0x2d0);
        frame.save(c, S4, 0x2b0);
        frame.save_ra(c, 0x2a0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_c49c)?;
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_c4a8)?;
        c.g[V1][0] = 0x4a_0000;
        c.g[A0][0] = 0x4a_0000;
        c.g[A3][0] = add_at(c.g[V1][0], 0x14f8);
        let name = c.g[A3][0] as u32;
        c.g[V0][0] = c.lb(name);
        c.g[A1][0] = c.lb(name.wrapping_add(1));
        let s = frame.at(c, 0);
        c.sb(s, c.g[V0][0]);
        c.sb(s.wrapping_add(1), c.g[A1][0]);
        c.g[A0][0] = add_at(c.g[A0][0], 0x1398);
        call_ptr(c, 0x0031_7670, 0x0018_c4cc)?;
        c.g[A0][0] = lwr!(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_c4d8)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[S2][0] = 0;
        if c.g[S3][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = add_at(c.g[S2][0], 0x31);
            loop {
                c.cycles += 14;
                c.g[A0][0] = c.g[SP][0];
                let s = frame.at(c, 0);
                c.sb(s, c.g[V0][0]);
                c.g[S4][0] = add_at(c.g[S2][0], 1);
                c.g[S1][0] = lwr!(c, S3, 8);
                c.g[S0][0] = lhr!(c, S1, 0x70);
                c.g[S0][0] = add_at(c.g[S3][0], c.g[S0][0] as u32);
                call_ptr(c, 0x0031_7670, 0x0018_c504)?;
                c.g[V1][0] = lwr!(c, S1, 0x74);
                c.g[A0][0] = c.g[S0][0];
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, target, 0x0018_c514)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = 0x53_0000;
                if c.g[A2][0] == 0 {
                    c.cycles += 1;
                    c.g[S2][0] = c.g[S4][0];
                } else {
                    c.cycles += 5;
                    let item = c.g[A2][0] as u32;
                    c.sw(item.wrapping_add(0x18), c.g[S2][0]);
                    c.g[A0][0] = add_at(c.g[V0][0], 0x5610);
                    c.g[V0][0] = c.g[A0][0] & 7;
                    c.g[V1][0] = add_at(c.g[SP][0], 0x10);
                    if c.g[V0][0] != 0 {
                        // Unaligned copy (never taken: the source is constant).
                        c.cycles += 1;
                        c.g[V0][0] = add_at(c.g[A0][0], 0x280);
                        loop {
                            c.cycles += 19;
                            let a0 = c.g[A0][0] as u32;
                            let v1 = c.g[V1][0] as u32;
                            c.ldl(8, a0.wrapping_add(7));
                            c.ldr(8, a0);
                            c.ldl(9, a0.wrapping_add(0xf));
                            c.ldr(9, a0.wrapping_add(8));
                            c.ldl(5, a0.wrapping_add(0x17));
                            c.ldr(5, a0.wrapping_add(0x10));
                            c.ldl(7, a0.wrapping_add(0x1f));
                            c.ldr(7, a0.wrapping_add(0x18));
                            c.sdl(8, v1.wrapping_add(7));
                            c.sdr(8, v1);
                            c.sdl(9, v1.wrapping_add(0xf));
                            c.sdr(9, v1.wrapping_add(8));
                            c.sdl(5, v1.wrapping_add(0x17));
                            c.sdr(5, v1.wrapping_add(0x10));
                            c.sdl(7, v1.wrapping_add(0x1f));
                            c.sdr(7, v1.wrapping_add(0x18));
                            c.g[A0][0] = add_at(c.g[A0][0], 0x20);
                            let more = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = add_at(c.g[V1][0], 0x20);
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018_c538);
                                }
                                continue;
                            }
                            c.cycles += 2;
                            break;
                        }
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = add_at(c.g[A0][0], 0x280);
                        loop {
                            c.cycles += 11;
                            let a0 = c.g[A0][0] as u32;
                            let v1 = c.g[V1][0] as u32;
                            c.g[T0][0] = c.ld(a0);
                            c.g[T1][0] = c.ld(a0.wrapping_add(8));
                            c.g[A1][0] = c.ld(a0.wrapping_add(0x10));
                            c.g[A3][0] = c.ld(a0.wrapping_add(0x18));
                            c.sd(v1, c.g[T0][0]);
                            c.sd(v1.wrapping_add(8), c.g[T1][0]);
                            c.sd(v1.wrapping_add(0x10), c.g[A1][0]);
                            c.sd(v1.wrapping_add(0x18), c.g[A3][0]);
                            c.g[A0][0] = add_at(c.g[A0][0], 0x20);
                            let more = c.g[A0][0] != c.g[V0][0];
                            c.g[V1][0] = add_at(c.g[V1][0], 0x20);
                            if more {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0018_c590);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                    // The last eight bytes of the option block.
                    c.cycles += 8;
                    let a0 = c.g[A0][0] as u32;
                    let v1 = c.g[V1][0] as u32;
                    c.ldl(8, a0.wrapping_add(7));
                    c.ldr(8, a0);
                    c.sdl(8, v1.wrapping_add(7));
                    c.sdr(8, v1);
                    let copy = frame.at(c, 0x10);
                    c.g[V0][0] = c.lw(copy);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 30);
                    let other = c.g[S2][0] != c.g[V0][0];
                    c.g[V0][0] = lwr!(c, A2, 8);
                    if other {
                        c.cycles += 5;
                        c.g[A1][0] = 0;
                        c.g[A0][0] = lhr!(c, V0, 0x48);
                        c.g[V1][0] = lwr!(c, V0, 0x4c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = add_at(c.g[A2][0], c.g[A0][0] as u32);
                        call_ptr(c, target, 0x0018_c618)?;
                        c.cycles += 1;
                        c.g[S2][0] = c.g[S4][0];
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 1;
                        c.g[A0][0] = lhr!(c, V0, 0x48);
                        c.g[V1][0] = lwr!(c, V0, 0x4c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = add_at(c.g[A2][0], c.g[A0][0] as u32);
                        call_ptr(c, target, 0x0018_c5f0)?;
                        c.g[A1][0] = c.g[S2][0] & 0xff;
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0039_b960, 0x0018_c5fc)?;
                        c.g[S2][0] = c.g[S4][0];
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 3) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = add_at(c.g[S2][0], 0x31);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0018_c4e8);
                    }
                    continue;
                }
                c.cycles += 4;
                c.g[V0][0] = lwr!(c, S3, 0x14);
                c.g[V1][0] = 0xffff_ffff_ffff_fffe;
                c.g[V0][0] &= c.g[V1][0];
                let flags = (c.g[S3][0] as u32).wrapping_add(0x14);
                c.sw(flags, c.g[V0][0]);
                break;
            }
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x2f0);
        frame.restore(c, S1, 0x2e0);
        frame.restore(c, S2, 0x2d0);
        frame.restore(c, S3, 0x2c0);
        frame.restore(c, S4, 0x2b0);
        frame.restore_ra(c, 0x2a0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsController_Init(self)` (0x18c6a0): create the
/// "22control" screen, clear the two vibration timers, fill the four list
/// boxes ("1lb"/"2lb": Off/On, "3lb"/"4lb": Default/Pro), switch both pads'
/// vibration off and refresh the list boxes from the current settings.
pub fn oc838_options_controller_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x30);
        c.g[A0][0] = 0x46_0000;
        frame.save(c, S0, 0x20);
        c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x1f98));
        c.g[S0][0] = lwr!(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x0018_c6c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_c6dc)?;
        let created = c.g[V0][0] != 0;
        let this = (c.g[S1][0] as u32).wrapping_add(0x40);
        c.sw(this, c.g[V0][0]);
        if created {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_c6f4)?;
        }
        // "1lb"
        c.cycles += 11;
        let this = c.g[S1][0] as u32;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[A0][0] = 0x4a_0000;
        c.sw(this.wrapping_add(0x48), 0);
        if oc838_find_widget(c, 0, 0x1568, 0x0018_c708, 0x0018_c714)? {
            oc838_add_two_entries(
                c,
                15,
                0x20c0,
                0x1fd8,
                [0x0018_c72c, 0x0018_c73c, 0x0018_c748, 0x0018_c758],
            )?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "2lb"
        if oc838_find_widget(c, 8, 0x1570, 0x0018_c764, 0x0018_c770)? {
            oc838_add_two_entries(
                c,
                15,
                0x20c0,
                0x1fd8,
                [0x0018_c788, 0x0018_c798, 0x0018_c7a4, 0x0018_c7b4],
            )?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "3lb"
        if oc838_find_widget(c, 8, 0x15c8, 0x0018_c7c0, 0x0018_c7cc)? {
            oc838_add_two_entries(
                c,
                15,
                0x1f88,
                0x1f70,
                [0x0018_c7e4, 0x0018_c7f4, 0x0018_c800, 0x0018_c810],
            )?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "4lb"
        if oc838_find_widget(c, 8, 0x1578, 0x0018_c81c, 0x0018_c828)? {
            oc838_add_two_entries(
                c,
                14,
                0x1f88,
                0x1f70,
                [0x0018_c840, 0x0018_c850, 0x0018_c85c, 0x0018_c86c],
            )?;
        }
        c.cycles += 19;
        oc838_vibrate_both(c, S1, 0x0018_c884, 0x0018_c898)?;
        c.g[A0][0] = c.g[S1][0];
        OC838_REFRESH_LIST_BOXES.call(c, 0x0018_c8a0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsController_OnChildEvent(self, child, int event)`
/// (0x18c910): event 0x15 switches both pads' vibration off; event 0x16 on
/// the "resetopts" child with a non-null `getData(2)` resets the controller
/// mapping to the defaults, refreshes the list boxes and flags the change
/// (`gp-0x1c14 = 1`).
pub fn oc838_options_controller_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x50);
        call_ptr(c, 0x0014_dc80, 0x0018_c93c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_c948)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = 0x15;
        let is_15 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = 0x16;
        'done: {
            if is_15 {
                // Event 0x15: both pads' vibration off.
                c.cycles += 13;
                oc838_vibrate_both(c, S2, 0x0018_c980, 0x0018_c994)?;
                frame.restore(c, S0, 0x40);
                break 'done;
            }
            c.cycles += 2;
            let is_16 = c.g[S0][0] == c.g[V0][0];
            frame.restore(c, S0, 0x40);
            if !is_16 {
                c.cycles += 2;
                frame.restore(c, S1, 0x30);
                c.cycles += 6;
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x50);
                frame.close(c);
                return Ok(());
            }
            // Event 0x16.
            c.cycles += 6;
            c.g[A0][0] = 0x46_0000;
            c.g[S0][0] = lwr!(c, S1, 0xc);
            c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x22e0));
            call_ptr(c, 0x0031_7670, 0x0018_c9ac)?;
            let differs = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x40);
            if differs {
                break 'done;
            }
            c.cycles += 8;
            c.g[V1][0] = lwr!(c, S1, 8);
            c.g[A1][0] = 2;
            c.g[A0][0] = lhr!(c, V1, 0xb8);
            c.g[V0][0] = lwr!(c, V1, 0xbc);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = add_at(c.g[S1][0], c.g[A0][0] as u32);
            call_ptr(c, target, 0x0018_c9cc)?;
            let empty = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x40);
            if empty {
                break 'done;
            }
            c.cycles += 7;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0014_5bd8, 0x0018_c9dc)?;
            c.g[A0][0] = c.g[S2][0];
            OC838_REFRESH_LIST_BOXES.call(c, 0x0018_c9e4)?;
            c.g[V0][0] = 1;
            let flag = (c.g[GP][0] as u32).wrapping_sub(0x1c14);
            c.sw(flag, c.g[V0][0]);
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 1;
        frame.restore(c, S1, 0x30);
        c.cycles += 6;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateOptionsController_OnWidgetNotify(self, menu, key)`
/// (0x18cce0): key 6 returns 0, key 9 returns 0x100, other keys 0x101; key 5
/// on the menu row 2 or 3 also shows the config text for pad 0 or 1 (default
/// text when that pad's controller mapping is 0).
pub fn oc838_options_controller_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_cd04)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_cd10)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = 6;
        let key = c.g[S0][0];
        let is_6 = key == c.g[V0][0];
        c.g[V0][0] = (key < 7) as u64;
        // The shared exits of the generated body: `b_18cd98` (+1 cycle,
        // restore s0) and the epilogue behind it (+5 cycles).
        let restore_s0 = 'decide: {
            if is_6 {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'decide true;
            }
            c.cycles += 2;
            let high = c.g[V0][0] == 0;
            c.g[V0][0] = 5;
            if high {
                // key >= 7: 9 returns 0x100, the rest 0x101.
                c.cycles += 3;
                c.g[V0][0] = 9;
                let not_9 = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = 0x101;
                if !not_9 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
                break 'decide true;
            }
            c.cycles += 2;
            let is_5 = c.g[S0][0] == c.g[V0][0];
            c.g[V0][0] = 0x101;
            if !is_5 {
                c.cycles += 2;
                break 'decide false;
            }
            // Key 5: which pad, from the menu's current row.
            c.cycles += 4;
            c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
            c.g[V0][0] = 2;
            let row2 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if row2 {
                c.cycles += 2;
                c.g[A1][0] = 0;
            } else {
                c.cycles += 2;
                let row3 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0x101;
                if !row3 {
                    c.cycles += 2;
                    break 'decide false;
                }
                c.cycles += 1;
                c.g[A1][0] = 1;
            }
            c.cycles += 7;
            call_ptr(c, 0x0014_7618, 0x0018_cd80)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = (c.g[V0][0] < 1) as u64;
            OC838_SHOW_CONFIG_TEXT.call(c, 0x0018_cd8c)?;
            c.g[V0][0] = 0x101;
            true
        };
        if restore_s0 {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateOptionsController_RefreshListBoxes(self)` (0x18cdc0): set
/// each of the four list boxes' current entry from the player interface:
/// "1lb"/"2lb" the dual-shock flag of pad 0/1, "3lb"/"4lb" the controller
/// mapping of pad 0/1 (and the config text for pad 0's mapping).
pub fn oc838_options_controller_refresh_list_boxes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0018_cddc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0018_cde8)?;
        c.g[A0][0] = 0x4a_0000;
        c.g[S2][0] = c.g[V0][0];
        // "1lb"
        if oc838_find_widget(c, 0, 0x1568, 0x0018_cdf8, 0x0018_ce04)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_74e8, 0x0018_ce1c)?;
            oc838_set_current_entry(c, 0, 0x0018_ce2c, 0x0018_ce38)?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "2lb"
        if oc838_find_widget(c, 8, 0x1570, 0x0018_ce44, 0x0018_ce50)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_74e8, 0x0018_ce68)?;
            oc838_set_current_entry(c, 1, 0x0018_ce78, 0x0018_ce84)?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "3lb"
        if oc838_find_widget(c, 8, 0x15c8, 0x0018_ce90, 0x0018_ce9c)? {
            c.cycles += 17;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_7618, 0x0018_ceb4)?;
            oc838_set_current_entry(c, 2, 0x0018_cec4, 0x0018_ced0)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_7618, 0x0018_cedc)?;
            c.g[A1][0] = (c.g[V0][0] < 1) as u64;
            c.g[A0][0] = c.g[S1][0];
            OC838_SHOW_CONFIG_TEXT.call(c, 0x0018_cee8)?;
        }
        c.g[A0][0] = 0x4a_0000;
        // "4lb"
        if oc838_find_widget(c, 8, 0x1578, 0x0018_cef4, 0x0018_cf00)? {
            c.cycles += 11;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x0014_7618, 0x0018_cf18)?;
            oc838_set_current_entry(c, 3, 0x0018_cf28, 0x0018_cf34)?;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ListBox_GetEntryByData(s0, a1 = v0, &vals[i])` then
/// `ListBox_SetCurrentEntryByIndex(s0, vals[i])`; `vals` is the four bytes at
/// the stack pointer.
fn oc838_set_current_entry(c: &mut Cpu, index: u32, ret_get: u32, ret_set: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[SP][0] | index as u64;
    call_ptr(c, 0x0039_a768, ret_get)?;
    let slot = (c.g[SP][0] as u32).wrapping_add(index);
    c.g[A1][0] = c.lbu(slot);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a7a8, ret_set)
}

/// `void cFEStateOptionsController_ShowConfigText(self, int on)` (0x18cf50):
/// show the "default" picture and text when `on`, the "pro" ones when not
/// (the four widgets are looked up by hash in the screen at `self + 0x40`;
/// the first two names come from `gp-0x1b34` and `gp-0x1b30`).
pub fn oc838_options_controller_show_config_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b34));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        oc838_set_visible(c, false, [0x0018_cf70, 0x0018_cf7c, 0x0018_cf9c])?;
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b30));
        oc838_set_visible(c, true, [0x0018_cfa4, 0x0018_cfb0, 0x0018_cfd0])?;
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x1f60));
        oc838_set_visible(c, false, [0x0018_cfdc, 0x0018_cfe8, 0x0018_d008])?;
        c.cycles += 9;
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = add_at(c.g[A0][0], 0u32.wrapping_sub(0x1f50));
        oc838_set_visible(c, true, [0x0018_d014, 0x0018_d020, 0x0018_d040])?;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One widget of `ShowConfigText`: hash the name in `a0`, find the object in
/// the screen at `s0 + 0x40`, and when it exists call its virtual
/// `setVisible(on ^ invert)` (`a1` is `s1`, or `s1 ^ 1`, in the delay slot
/// of the null test either way).
fn oc838_set_visible(c: &mut Cpu, invert: bool, rets: [u32; 3]) -> Flow {
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[A0][0] = lwr!(c, S0, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, rets[1])?;
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = c.g[S1][0] ^ invert as u64;
    if !missing {
        c.cycles += 5;
        oc838_widget_vcall(c, rets[2])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::Rng;
    use crate::difftest::{DATA, GAME_GP};
    use crate::game::fe::feasyncfile::tests::run_8;
    use crate::game::main::loadscreens::tests::{oc736_fake_call, oc736_force_zero};
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's callee fakes (clear of the fake log at
    /// `DATA + 0x100000` and of the loadscreens fakes' gate).
    const CTRL: u32 = DATA + 0x2a_0000;
    const MAGIC: u32 = 0x4f43_3838;

    macro_rules! oc838_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(CTRL) != MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            fn oc838_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc838_fakes!(
        (
            oc838_fk_14dc80,
            OC838_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc838_fk_14dd58,
            OC838_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc838_fk_147618,
            OC838_PREV_147618,
            ee::s01::OVERRIDE_00147618,
            ee::s01::f_00147618__generated,
            0x0014_7618
        ),
        (
            oc838_fk_1474e8,
            OC838_PREV_1474E8,
            ee::s01::OVERRIDE_001474E8,
            ee::s01::f_001474e8__generated,
            0x0014_74e8
        ),
        (
            oc838_fk_145bd8,
            OC838_PREV_145BD8,
            ee::s01::OVERRIDE_00145BD8,
            ee::s01::f_00145bd8__generated,
            0x0014_5bd8
        ),
        (
            oc838_fk_397cc0,
            OC838_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc838_fk_39c870,
            OC838_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc838_fk_39a568,
            OC838_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc838_fk_39a768,
            OC838_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc838_fk_39a7a8,
            OC838_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc838_fk_39b960,
            OC838_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
    );

    static OC838_PREV_39D860: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake of `UIScreen_FindObject` (0x39d860): logs the call like the
    /// other fakes, then returns the fixture screen's object whose hash
    /// (`+0x38`) equals `a1`, or 0 (the real loop runs away on test data).
    fn oc838_fk_39d860(c: &mut Cpu) {
        if c.bus.read32(CTRL) != MAGIC {
            return fakes::fall_back(c, &OC838_PREV_39D860, ee::s13::f_0039d860__generated);
        }
        let screen = c.g[A0][0] as u32;
        let hash = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let count = c.bus.read32(screen.wrapping_add(0x3c)).min(16);
        let array = c.bus.read32(screen.wrapping_add(0x38));
        let mut found = 0u32;
        for i in 0..count {
            let obj = c.bus.read32(array.wrapping_add(4 * i));
            if obj != 0 && c.bus.read32(obj.wrapping_add(0x38)) == hash {
                found = obj;
                break;
            }
        }
        fakes::fake_call(c, 0x0039_d860);
        if c.pc == ra {
            c.g[V0][0] = found as u64;
        }
    }

    /// The screen of the controller / HUD under test.
    const SCR: u32 = DATA + 0x4_0000;
    /// The state object under test.
    const THIS: u32 = DATA + 0x100;
    /// A widget with a vtable whose methods are fakes (the "item" of the HUD).
    const ITEM: u32 = DATA + 0x6_0000;
    /// A real leaf that returns 0.
    const ZERO_LEAF: u32 = 0x0014_06c0;

    /// The four list box names, and the names `ShowConfigText` looks up.
    const LB_NAMES: [u32; 4] = [0x004a_1568, 0x004a_1570, 0x004a_15c8, 0x004a_1578];
    const CFG_NAMES: [u32; 4] = [0x004a_15bc, 0x004a_15c0, 0x0045_e0a0, 0x0045_e0b0];

    fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run_8(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }

    /// Machine with the callee fakes gated on, and the fixture's per-call
    /// unwind index.
    fn fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc838_install_fakes();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc838_fk_39d860,
            &OC838_PREV_39D860,
        );
        let r = machine(c, seed, salt, calls);
        c.bus.write32(CTRL, MAGIC);
        r
    }

    /// A screen at `SCR` with one object per hash (count at `+0x3c`, pointer
    /// array at `+0x38`, hash at `object + 0x38`), all sharing one vtable whose
    /// methods at `+0x4c`, `+0xbc` and `+0x74` are `fns` (deltas 0).
    fn screen(c: &mut Cpu, hashes: &[u32], fns: [u32; 3]) {
        let array = SCR + 0x100;
        let vt = SCR + 0x1000;
        c.bus.write32(SCR + 0x3c, hashes.len() as u32);
        c.bus.write32(SCR + 0x38, array);
        for (i, &h) in hashes.iter().enumerate() {
            let obj = SCR + 0x200 + 0x100 * i as u32;
            c.bus.write32(array + 4 * i as u32, obj);
            c.bus.write32(obj + 0x38, h);
            c.bus.write32(obj + 8, vt);
            c.bus.write32(obj + 0x14, 0x5);
            c.bus.write8(obj + 0x95, i as u8);
        }
        for (off, f) in [(0x4cu32, fns[0]), (0xbc, fns[1]), (0x74, fns[2])] {
            c.bus.write16(vt + off - 4, 0);
            c.bus.write32(vt + off, f);
        }
        c.bus.write16(vt + 0x70, 0);
    }

    /// The names of `mask`'s set bits, hashed by the real hash function.
    fn present(c: &mut Cpu, names: &[u32], mask: u32) -> Vec<u32> {
        let mut v = Vec::new();
        for (i, &n) in names.iter().enumerate() {
            if mask >> i & 1 != 0 {
                v.push(str_hash(c, n));
            }
        }
        v
    }

    /// `gp-0x1b34` / `gp-0x1b30`: the pointers to the picture names.
    fn config_names(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1b34), CFG_NAMES[0]);
        c.bus.write32(GAME_GP.wrapping_sub(0x1b30), CFG_NAMES[1]);
    }

    /// Which vtable method a seed uses: a fake, or a real leaf returning 0.
    fn vfn(seed: u32, shift: u32) -> u32 {
        if (seed >> shift) % 3 == 0 {
            ZERO_LEAF
        } else {
            FAKE + 0x10
        }
    }

    /// An item (the HUD row) the fakes' forced results hand out.
    fn item(c: &mut Cpu) {
        c.bus.write32(ITEM + 8, ITEM + 0x400);
        c.bus.write16(ITEM + 0x400 + 0x48, 0);
        c.bus.write32(ITEM + 0x400 + 0x4c, FAKE + 0x20);
        for n in 2..8u32 {
            c.bus.write32(LOG_RESULTS + 4 * n, ITEM);
        }
    }

    /// `cFEStateOptionsHUD_RefreshSelection`: the "Menu" widget missing or
    /// present, every option mode 0..2 (the row index matches for one row), the
    /// row lookup returning a fake item or nothing (the real leaf).
    #[test]
    fn options_hud_refresh_selection_matches() {
        check(
            ee::s02::f_0018c478__generated,
            oc838_options_hud_refresh_selection,
            90,
            &|c, seed| {
                let mut r = fixture(c, seed, 0x18c478, 8);
                item(c);
                let menu = str_hash(c, 0x004a_1398);
                let hashes = match seed % 5 {
                    0 => vec![],
                    1 => vec![menu ^ 1],
                    _ => vec![r.next_u32(), menu],
                };
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, vfn(seed, 2)]);
                c.bus.write32(THIS + 0x40, SCR);
                // The option block: mode in the top two bits of its first word.
                for i in 0..0xa4u32 {
                    c.bus.write32(0x0053_5610 + 4 * i, r.next_u32());
                }
                c.bus
                    .write32(0x0053_5610, (seed % 3) << 30 | r.next_u32() >> 2);
                c.pc = 0x0018_c478;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `cFEStateOptionsController_Init`: the screen creation failing or not
    /// (a forced fake result), each subset of the four list boxes found.
    #[test]
    fn options_controller_init_matches() {
        check(
            ee::s02::f_0018c6a0__generated,
            oc838_options_controller_init,
            160,
            &|c, seed| {
                let mut r = fixture(c, seed, 0x18c6a0, 14);
                if seed % 7 == 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                } else {
                    crate::game::main::loadscreens::tests::oc736_force(c, 0x0039_7cc0, SCR);
                }
                if seed % 3 == 1 {
                    oc736_force_zero(c, 0x0014_7618);
                }
                let hashes = present(c, &LB_NAMES, seed >> 1);
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x10, DATA + 0x200);
                c.bus.write32(THIS + 0x48, r.next_u32());
                c.bus.write32(THIS + 0x4c, r.next_u32());
                c.pc = 0x0018_c6a0;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `cFEStateOptionsController_OnChildEvent`: events 0x15, 0x16 (the
    /// "resetopts" child or another, `getData(2)` zero or not) and others.
    #[test]
    fn options_controller_on_child_event_matches() {
        check(
            ee::s02::f_0018c910__generated,
            oc838_options_controller_on_child_event,
            120,
            &|c, seed| {
                let mut r = fixture(c, seed, 0x18c910, 6);
                let reset = str_hash(c, 0x0045_dd20);
                let hashes = present(c, &LB_NAMES, seed >> 3);
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                let child = DATA + 0x800;
                let vt = DATA + 0x900;
                c.bus.write32(child + 8, vt);
                c.bus.write16(vt + 0xb8, 0);
                c.bus.write32(
                    vt + 0xbc,
                    if seed % 4 == 0 {
                        ZERO_LEAF
                    } else {
                        FAKE + 0x40
                    },
                );
                c.bus.write32(
                    child + 0xc,
                    if seed % 3 == 0 { r.next_u32() } else { reset },
                );
                let event = [0x15u32, 0x16, 0x16, 0x16, 0x17, 0x0][(seed % 6) as usize];
                c.pc = 0x0018_c910;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = child as u64;
                c.g[A2][0] = event as u64;
            },
        );
    }

    /// `cFEStateOptionsController_OnWidgetNotify`: keys 5 (menu rows 2, 3 and
    /// others; the controller mapping zero or not), 6, 9, the small keys and
    /// the large ones.
    #[test]
    fn options_controller_on_widget_notify_matches() {
        check(
            ee::s02::f_0018cce0__generated,
            oc838_options_controller_on_widget_notify,
            160,
            &|c, seed| {
                let _r = fixture(c, seed, 0x18cce0, 6);
                if seed % 2 == 1 {
                    oc736_force_zero(c, 0x0014_7618);
                }
                config_names(c);
                let hashes = present(c, &CFG_NAMES, seed >> 2);
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                let menu = DATA + 0x800;
                c.bus
                    .write8(menu + 0x95, [2u8, 3, 0, 4, 3, 2][((seed / 2) % 6) as usize]);
                let key = [5u32, 5, 5, 6, 9, 0, 1, 3, 4, 7, 8, 0x101][(seed % 12) as usize];
                c.pc = 0x0018_cce0;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = menu as u64;
                c.g[A2][0] = key as u64;
            },
        );
    }

    /// `cFEStateOptionsController_RefreshListBoxes`: each subset of the four
    /// list boxes present, the pad mapping zero or not.
    #[test]
    fn options_controller_refresh_list_boxes_matches() {
        check(
            ee::s02::f_0018cdc0__generated,
            oc838_options_controller_refresh_list_boxes,
            160,
            &|c, seed| {
                let _r = fixture(c, seed, 0x18cdc0, 10);
                if seed % 3 == 1 {
                    oc736_force_zero(c, 0x0014_7618);
                }
                config_names(c);
                let mut hashes = present(c, &LB_NAMES, seed);
                hashes.extend(present(c, &CFG_NAMES, seed >> 4));
                screen(c, &hashes, [FAKE + 0x10, FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                c.pc = 0x0018_cdc0;
                c.g[A0][0] = THIS as u64;
            },
        );
    }

    /// `cFEStateOptionsController_ShowConfigText`: each subset of the four
    /// widgets present, `on` 0, 1 and other values.
    #[test]
    fn options_controller_show_config_text_matches() {
        check(
            ee::s02::f_0018cf50__generated,
            oc838_options_controller_show_config_text,
            160,
            &|c, seed| {
                let mut r = fixture(c, seed, 0x18cf50, 5);
                config_names(c);
                let hashes = present(c, &CFG_NAMES, seed);
                screen(c, &hashes, [vfn(seed, 4), FAKE + 0x20, FAKE + 0x30]);
                c.bus.write32(THIS + 0x40, SCR);
                let on = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32(),
                };
                c.pc = 0x0018_cf50;
                c.g[A0][0] = THIS as u64;
                c.g[A1][0] = on as u64;
            },
        );
    }
}
