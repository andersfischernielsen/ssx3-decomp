//! `src/fe/ovstaterewardslist.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::ovstatepause::ov_base_fill_fields;
pub(crate) use crate::game::fe::ovstatepause::OV_UI_STATE_CTOR;
pub(crate) use crate::game::fe::ovstatetrophy::oc1015_dtor;
pub(crate) use crate::game::fe::ovstatetrophy::OC1015_UI_DTOR;
pub(crate) use crate::game::fe::ovstatetrophy::OC1015_VT_BASE;
pub(crate) use crate::game::main::ssxappdtor::p as p_2;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0020_0218,
        ov_screen_rewards_list_popup_init,
        "OVScreenRewardsListPopup_Init",
    ),
    (0x0020_06b8, event_select_ctor, "EventSelect_ctor"),
    (
        0x0020_0388,
        event_select_get_event_record,
        "EventSelect_GetEventRecord",
    ),
    (
        0x0020_0288,
        ov_screen_rewards_list_popup_on_widget_action,
        "OVScreenRewardsListPopup_OnWidgetAction",
    ),
    (
        0x0022_2100,
        ov_screen_rewards_list_popup_ctor,
        "OVScreenRewardsListPopup_ctor",
    ),
    (
        0x0022_2160,
        ov_screen_rewards_list_popup_ctor_2,
        "OVScreenRewardsListPopup_ctor_2",
    ),
    (
        0x0022_21c8,
        ov_screen_rewards_list_popup_dtor,
        "OVScreenRewardsListPopup_dtor",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_GET_INTERFACE_RIDER;
}

/// `EventSelect* EventSelect_ctor(EventSelect* self)` (0x2006b8): when the
/// reward screen is in play, cache its selected index for the widget layer.
pub fn event_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        c.g[V0][0] = c.lw(gp(c, -0xb5c));
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[V0][0] < 2u64) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0020_06d4)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0020_06e0)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_INTERFACE_RIDER, 0x0020_06e8)?;
            c.g[V1][0] = c.lw(at(c, V0, 0x54));
            sw(c, gp(c, -0xb4c), 0, c.g[V1][0]);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenRewardsListPopup_Init(OVScreenRewardsListPopup* self)`
/// (0x200218): create the rewards-list popup screen and start its thread.
pub fn ov_screen_rewards_list_popup_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x4704_b8;
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0020_023c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0020_0250)?;
        let t = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0020_0268)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x0020_0270)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of `OVScreenTrophy_OnWidgetAction` (0x20e9a0) and
/// `OVScreenRewardsListPopup_OnWidgetAction` (0x200288): the same screen
/// widget-action handler compiled twice. `ret0` is the return address of its
/// first nested call; the others follow at fixed offsets.
pub(crate) fn ov_screen_widget_action(c: &mut Cpu, ret0: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x40);
    let widget = c.g[A1][0];
    frame.save(c, 16, 0x30);
    frame.save(c, 17, 0x20);
    c.g[S0][0] = widget;
    frame.save_ra(c, 0x10);
    let no_widget = c.g[S0][0] == 0;
    c.g[S1][0] = c.g[A0][0];
    if !no_widget {
        c.cycles += 3;
        c.g[V0][0] = 0x5;
        let action5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 0x6;
        if action5 {
            // a "clicked" action: tell the widget (slot 0x58) the new state
            c.cycles += 16;
            c.g[V1][0] = c.lw(p_2(c.g[S0][0] as u32, 8));
            c.g[A1][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x58));
            c.swc1(0, p_2(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.sw(p_2(c.g[SP][0] as u32, 8), 0);
            c.sw(p_2(c.g[SP][0] as u32, 0xc), 0);
            c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x5c));
            let func = c.g[V0][0] as u32;
            c.g[RA][0] = sx(ret0);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, ret0)?;
            c.g[V1][0] = c.lw(p_2(c.g[S0][0] as u32, 0x18));
            c.g[V0][0] = 0x1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = 0;
            if !not_first {
                c.cycles += 10;
                c.g[V0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0x1;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p_2(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x20));
                c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x24));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x34);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x34)?;
                c.g[A1][0] = c.g[V0][0];
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
            c.g[RA][0] = sx(ret0 + 0x44);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f400, ret0 + 0x44)?;
            c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
        } else {
            c.cycles += 2;
            if c.g[A2][0] == c.g[V0][0] {
                // the "second" action: hand the widget's state to the screen
                c.g[V0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                c.cycles += 10;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.lw(p_2(c.g[S0][0] as u32, 0x18));
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[V1][0] = c.lw(p_2(c.g[A3][0] as u32, 4));
                c.g[A0][0] = c.lh(p_2(c.g[V1][0] as u32, 0x28));
                c.g[V0][0] = c.lw(p_2(c.g[V1][0] as u32, 0x2c));
                let func = c.g[V0][0] as u32;
                c.g[RA][0] = sx(ret0 + 0x6c);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, ret0 + 0x6c)?;
                let unchanged = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[V0][0];
                if !unchanged {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(p_2(c.g[S1][0] as u32, 0x10));
                    c.g[RA][0] = sx(ret0 + 0x80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0039_f400, ret0 + 0x80)?;
                }
            } else {
                c.cycles += 2;
                c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
            }
        }
    }
    c.cycles += 1;
    c.lq(16, p_2(c.g[SP][0] as u32, 0x30));
    c.cycles += 4;
    c.lq(17, p_2(c.g[SP][0] as u32, 0x20));
    c.g[RA][0] = c.ld(p_2(c.g[SP][0] as u32, 0x10));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    Ok(())
}

/// `void func_00200288(OVScreen*, Widget* w, int action)` (0x200288): the
/// rewards-list popup's widget-action handler (the same code).
pub fn ovscreen_rewards_list_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_02f0))
}

/// `void* EventSelect_GetEventRecord(void* dst, int peak, int goal, int kind)` (0x200388): copies the 0x6c-byte event record `[peak][goal]` of the race (kind 0, stride 0x1b0 per peak), freestyle (1, 0x21c) or freeride (2, 0x360) location table into `dst` (any other kind copies race record [0][0]) and returns `dst`. Word-copy loop when source and destination are 8-aligned, `ldl/ldr` copy otherwise.
/// Transcribed from the generated body (registers, cycles and call order kept).
#[allow(
    unused_variables,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut,
    unused_labels
)]
pub fn event_select_get_event_record(c: &mut Cpu) {
    'b_2006b0: {
        'b_2005fc: {
            'b_2005e0: {
                'b_200454: {
                    c.cycles += 3;
                    c.g[T1][0] = c.g[A0][0];
                    let t = c.g[A3][0] != 0u64;
                    c.g[V1][0] = c.g[A2][0];
                    if t {
                        break 'b_200454;
                    } else {
                        'b_20041c: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x1b0;
                            c.g[V0][0] = 0x6c;
                            c.mult(0, 2, 3, 2);
                            c.mult(0, 4, 5, 4);
                            c.g[V1][0] = 0x480000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = c.g[A0][0] | c.g[T1][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A3][0] = c.g[T1][0];
                            if t {
                                break 'b_20041c;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                                'l_2003c8: loop {
                                    c.cycles += 19;
                                    c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.ldr(3, (c.g[A0][0] as u32));
                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                    c.sdr(3, (c.g[A3][0] as u32));
                                    c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                                    c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                    c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                    c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if c.cycles >= c.next_event {
                                            c.pc = 0x002003c8;
                                            c.event();
                                        }
                                        continue 'l_2003c8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_2005e0;
                                        } else {
                                            break 'b_20041c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                        'l_200420: loop {
                            c.cycles += 11;
                            c.g[V1][0] = c.ld((c.g[A0][0] as u32));
                            c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            if t {
                                if c.cycles >= c.next_event {
                                    c.pc = 0x00200420;
                                    c.event();
                                }
                                continue 'l_200420;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_2005e0;
                                } else {
                                    break 'b_200454;
                                }
                            }
                        }
                    }
                }
                'b_200520: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = c.g[A3][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2;
                    if t {
                        break 'b_200520;
                    } else {
                        'b_2004e8: {
                            c.cycles += 12;
                            c.g[A0][0] = 0x21c;
                            c.g[V0][0] = 0x6c;
                            c.mult(0, 2, 3, 2);
                            c.mult(0, 4, 5, 4);
                            c.g[V1][0] = 0x480000;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7920));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = c.g[A0][0] | c.g[T1][0];
                            c.g[V0][0] = c.g[V0][0] & 0x7;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A3][0] = c.g[T1][0];
                            if t {
                                break 'b_2004e8;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                                'l_200494: loop {
                                    c.cycles += 19;
                                    c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                                    c.ldr(3, (c.g[A0][0] as u32));
                                    c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                    c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                    c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                    c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                    c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                    c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                    c.sdr(3, (c.g[A3][0] as u32));
                                    c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                                    c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                    c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                    c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                    c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                    let t = c.g[A0][0] != c.g[V0][0];
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                    if t {
                                        if c.cycles >= c.next_event {
                                            c.pc = 0x00200494;
                                            c.event();
                                        }
                                        continue 'l_200494;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_2005e0;
                                        } else {
                                            break 'b_2004e8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                        'l_2004ec: loop {
                            c.cycles += 11;
                            c.g[V1][0] = c.ld((c.g[A0][0] as u32));
                            c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                            c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                            c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                            c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                            if t {
                                if c.cycles >= c.next_event {
                                    c.pc = 0x002004ec;
                                    c.event();
                                }
                                continue 'l_2004ec;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_2005e0;
                                } else {
                                    break 'b_200520;
                                }
                            }
                        }
                    }
                }
                c.cycles += 2;
                if c.g[A3][0] != c.g[V0][0] {
                    c.g[V1][0] = 0x480000;
                    break 'b_2005fc;
                } else {
                    'b_2005b0: {
                        c.cycles += 12;
                        c.g[A0][0] = 0x360;
                        c.g[V0][0] = 0x6c;
                        c.mult(0, 2, 3, 2);
                        c.mult(0, 4, 5, 4);
                        c.g[V1][0] = 0x480000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72c8));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[V0][0] = c.g[A0][0] | c.g[T1][0];
                        c.g[V0][0] = c.g[V0][0] & 0x7;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A3][0] = c.g[T1][0];
                        if t {
                            break 'b_2005b0;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                            'l_20055c: loop {
                                c.cycles += 19;
                                c.ldl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
                                c.ldr(3, (c.g[A0][0] as u32));
                                c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
                                c.ldr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
                                c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x17));
                                c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x10));
                                c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                                c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                                c.sdl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                                c.sdr(3, (c.g[A3][0] as u32));
                                c.sdl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                                c.sdr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                                c.sdl(6, (c.g[A3][0] as u32).wrapping_add(0x17));
                                c.sdr(6, (c.g[A3][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[A3][0] as u32).wrapping_add(0x1f));
                                c.sdr(8, (c.g[A3][0] as u32).wrapping_add(0x18));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                                if t {
                                    if c.cycles >= c.next_event {
                                        c.pc = 0x0020055c;
                                        c.event();
                                    }
                                    continue 'l_20055c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_2005e0;
                                    } else {
                                        break 'b_2005b0;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                    'l_2005b4: loop {
                        c.cycles += 11;
                        c.g[V1][0] = c.ld((c.g[A0][0] as u32));
                        c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[A2][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x10));
                        c.g[T0][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x18));
                        c.sd((c.g[A3][0] as u32), c.g[V1][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                        c.sd((c.g[A3][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        let t = c.g[A0][0] != c.g[V0][0];
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                        if t {
                            if c.cycles >= c.next_event {
                                c.pc = 0x002005b4;
                                c.event();
                            }
                            continue 'l_2005b4;
                        } else {
                            break 'b_2005e0;
                        }
                    }
                }
            }
            c.cycles += 7;
            c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[A0][0] as u32));
            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[A3][0] as u32));
            let t = true;
            c.sw((c.g[A3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            if t {
                break 'b_2006b0;
            } else {
                break 'b_2005fc;
            }
        }
        'b_200698: {
            'b_200668: {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
                c.g[V0][0] = c.g[T1][0] | c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] & 0x7;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[T1][0];
                if t {
                    break 'b_200668;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
                    'l_200614: loop {
                        c.cycles += 19;
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(7, (c.g[V1][0] as u32));
                        c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x17));
                        c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x1f));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                        c.sdr(7, (c.g[A0][0] as u32));
                        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                        c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x17));
                        c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                        c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x1f));
                        c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x18));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                        if t {
                            if c.cycles >= c.next_event {
                                c.pc = 0x00200614;
                                c.event();
                            }
                            continue 'l_200614;
                        } else {
                            c.cycles += 2;
                            let t = true;

                            if t {
                                break 'b_200698;
                            } else {
                                break 'b_200668;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x60));
            'l_20066c: loop {
                c.cycles += 11;
                c.g[A3][0] = c.ld((c.g[V1][0] as u32));
                c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                c.sd((c.g[A0][0] as u32), c.g[A3][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A2][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                if t {
                    if c.cycles >= c.next_event {
                        c.pc = 0x0020066c;
                        c.event();
                    }
                    continue 'l_20066c;
                } else {
                    break 'b_200698;
                }
            }
        }
        c.cycles += 6;
        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.ldr(7, (c.g[V1][0] as u32));
        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
        c.sdr(7, (c.g[A0][0] as u32));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        break 'b_2006b0;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[T1][0];
    c.pc = t;
    return;
}

/// `OVScreenRewardsListPopup_OnWidgetAction(this, data, kind)` (0x200288):
/// the popup's action dispatch. Returns whether the body leaves through the
/// one-cycle tail (`'b_200370`) rather than straight to the epilogue.
pub fn ov_screen_rewards_list_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let no_data = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        let tail = no_data || widget_action(c, frame)?;
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The kind dispatch of the widget action (`'b_20033c`). Returns true when
/// the body takes the `'b_200370` tail.
pub(crate) fn widget_action(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 0x5;
    let kind_five = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x6;
    if kind_five {
        widget_action_redraw(c, frame)?;
        c.cycles += 5;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x0020_0334)?;
        return Ok(false);
    }
    c.cycles += 2;
    if c.g[A2][0] == c.g[V0][0] {
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        return widget_action_notify(c);
    }
    c.cycles += 2;
    Ok(false)
}

/// Kind 5 (`'b_200328`): two float fields (1.0f) and a zeroed pair in the
/// frame, the vtable call with them, then the optional second call.
pub(crate) fn widget_action_redraw(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 16;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x8).lw(c);
    c.g[A1][0] = c.g[SP][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 0);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x58).lh(c);
    frame.save_fpr(c, 0, 0x4);
    frame.save_fpr(c, 0, 0x0);
    Addr(frame.at(c, 0x8)).sw(c, 0);
    Addr(frame.at(c, 0xc)).sw(c, 0);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5c).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0020_02f0)?;
    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
    c.g[V0][0] = 0x1;
    let other = c.g[V1][0] != c.g[V0][0];
    c.g[A1][0] = 0;
    if !other {
        c.cycles += 10;
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 0x1;
        c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x4).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0020_0324)?;
        c.g[A1][0] = c.g[V0][0];
    }
    Ok(())
}

/// Kind 6 (`'b_20033c` exit): the notify call on the object at `this->+0x10`.
/// Always leaves through the tail.
pub(crate) fn widget_action_notify(c: &mut Cpu) -> Result<bool, Unwound> {
    c.cycles += 10;
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0x18).lw(c);
    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(0x4).lw(c);
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
    let target = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, 0x0020_035c)?;
    let absent = c.g[V0][0] == 0;
    c.g[A1][0] = c.g[V0][0];
    if !absent {
        c.cycles += 3;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f400, 0x0020_0370)?;
    }
    Ok(true)
}

/// `OVScreenRewardsListPopup_ctor` (0x222100): the rewards-list popup state in
/// `a0` with overlay id 0, after `UIState_ctor(this, 0)`.
pub fn ov_screen_rewards_list_popup_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let f = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        f.save(c, 16, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OV_UI_STATE_CTOR, 0x0022_2118)?;
        ov_base_fill_fields(c, 0, 0x0047_21e0);
        f.restore(c, 16, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenRewardsListPopup_ctor_2` (0x222160): as the constructor above, with
/// the overlay id taken from `a2`.
pub fn ov_screen_rewards_list_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_UI_STATE_CTOR, 0x0022_217c)?;
        let ov_id = c.g[S1][0];
        ov_base_fill_fields(c, ov_id, 0x0047_21e0);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenRewardsListPopup_dtor` (0x2221c8): the rewards-list popup's
/// destructor. Sets the base `cOVState` vtable at +0x8, then `UIState_dtor`.
pub fn ov_screen_rewards_list_popup_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_BASE, OC1015_UI_DTOR, 0x0022_21e4)
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::fakes::machine;

    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;

    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    use ssx3_ee as ee;

    #[test]
    fn event_select_ctor_matches() {
        check(
            ssx3_ee::s05::f_002006b8__generated,
            event_select_ctor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2006b8, 0);
                be_library(c);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { 2 },
                );
            },
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_init_matches() {
        check(
            ee::s05::f_00200218__generated,
            ov_screen_rewards_list_popup_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x2002_18);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    use crate::difftest::check_lohi;

    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;

    /// `EventSelect_GetEventRecord`: all four kinds (race, freestyle,
    /// freeride, anything else), peaks and goals that make the source 8-byte
    /// aligned or not, destinations of every alignment.
    #[test]
    fn event_select_get_event_record_matches() {
        check_lohi(
            ee::s05::f_00200388__generated,
            event_select_get_event_record,
            400,
            &|c, seed| {
                let mut r = Rng(0x0020_0388 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                fill(c, &mut r, 0x0047_8000, 0x880);
                let dst = DATA + 0x100 + [0u32, 8, 4, 1, 3, 12, 16, 5][seed as usize / 6 % 8];
                let kind = [0u32, 1, 2, 3, 0xffff_ffff, 7][seed as usize % 6];
                c.g[A0][0] = sx(dst);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = sx(kind);
            },
        );
    }

    pub(crate) use crate::game::fe::ovstatepause::tests::oc1014_hook;
    pub(crate) use crate::game::fe::ovstatepause::tests::oc1014_setup;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc991_machine;

    pub(crate) use crate::game::fe::ovstatepause::tests::oc991_vtable;

    pub(crate) use crate::game::fe::ovstatepause::tests::OC991_MISS;
    pub(crate) use crate::game::fe::ovstatepause::tests::OC991_THIS;
    pub(crate) use crate::game::fe::ovstatetrophy::tests::oc1015_object_setup;

    pub(crate) fn oc991_setup_widget_action(c: &mut Cpu, seed: u32) {
        let mut r = oc991_machine(c, seed, 0x2002, 6);
        let vt = DATA + 0x2100;
        oc991_vtable(c, &mut r, vt, &[0x20, 0x28, 0x58]);
        // A zero data pointer takes the early exit.
        let arg: u32 = if seed % 7 == 0 { 0 } else { DATA + 0x1000 };
        if arg != 0 {
            c.bus.write32(arg + 8, vt);
            c.bus.write32(arg + 0x18, if seed % 2 == 0 { 1 } else { 0 });
        }
        let p = DATA + 0x2400;
        let obj = DATA + 0x2800;
        c.bus.write32(OC991_THIS + 0x10, p);
        c.bus.write32(p, obj);
        c.bus.write32(obj + 4, vt);
        // The notify call's result decides the tail (zero or not).
        c.bus.write32(
            OC991_MISS,
            if (seed / 3) % 2 == 0 { 0 } else { 0x0081_2340 },
        );
        c.g[A0][0] = sx(OC991_THIS);
        c.g[A1][0] = arg as u64;
        c.g[A2][0] = match seed % 3 {
            0 => 5,
            1 => 6,
            _ => 7 + (seed % 4) as u64,
        };
    }

    #[test]
    fn ov_screen_rewards_list_popup_on_widget_action_matches() {
        check(
            ee::s05::f_00200288__generated,
            ov_screen_rewards_list_popup_on_widget_action,
            200,
            &|c, seed| oc991_setup_widget_action(c, seed),
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_ctor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222100__generated,
            ov_screen_rewards_list_popup_ctor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2100),
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_ctor_2_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222160__generated,
            ov_screen_rewards_list_popup_ctor_2,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2160),
        );
    }

    #[test]
    fn ov_screen_rewards_list_popup_dtor_matches() {
        check(
            ee::s05::f_002221c8__generated,
            ov_screen_rewards_list_popup_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }
}
