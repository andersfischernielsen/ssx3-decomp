//! Staging module oc840 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc840`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_d108,
        c_fe_state_options_controller_set_vibration,
        "cFEStateOptionsController_SetVibration",
    ),
    (
        0x0018_d2e8,
        c_fe_state_options_save_load_on_widget_created,
        "cFEStateOptionsSaveLoad_OnWidgetCreated",
    ),
    (
        0x0018_d460,
        c_fe_state_options_save_load_on_child_event,
        "cFEStateOptionsSaveLoad_OnChildEvent",
    ),
    (
        0x0018_d648,
        c_fe_state_options_save_load_on_widget_action,
        "cFEStateOptionsSaveLoad_OnWidgetAction",
    ),
    (
        0x0018_dea0,
        c_fe_state_options_device_select_on_widget_action,
        "cFEStateOptionsDeviceSelect_OnWidgetAction",
    ),
    (
        0x0018_e368,
        c_fe_state_options_device_select_v39,
        "cFEStateOptionsDeviceSelect_v39",
    ),
];

/// `void cFEStateOptionsController_SetVibration(self, int pad, float x,
/// float y)` (0x18d108): sets the two rumble motors of pad `pad` from the
/// options sliders. `x` (f12) and `y` (f13) are rescaled and clamped; if the
/// pad's device reports two actuators (`+0x18/+0x1c` vtable slot) the large
/// (`setActuator(0, ..)`) and small (`setActuator(1, ..)`) motors are set
/// from the larger of two linear ramps (never below 0). f20-f22 are saved
/// and restored like the original.
pub fn c_fe_state_options_controller_set_vibration(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d154: {
            c.cycles += 18;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59e8));
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59e4));
            c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
            c.mul_s(12, 12, 0);
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.mtc1(0, 21);
            c.g[V0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sub_s(12, 12, 1);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.mov_s(20, 13);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.c_le_s(21, 12);
            let t = !c.fcc();
            c.mov_s(22, 21);
            if t {
                break 'b_18d154;
            } else {
                c.cycles += 1;
                c.mov_s(22, 12);
                break 'b_18d154;
            }
        }
        'b_18d174: {
            c.cycles += 5;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x59e0));
            c.g[S0][0] = 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x850));
            let t = c.g[V1][0] == 0u64;
            c.mul_s(20, 20, 0);
            if t {
                break 'b_18d174;
            } else {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2eec));
                break 'b_18d174;
            }
        }
        'b_18d228: {
            c.cycles += 8;
            c.g[V1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_d188)?;
            c.g[V1][0] = 0x2;
            if c.g[V0][0] != c.g[V1][0] {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_18d228;
            } else {
                c.cycles += 37;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0u64;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x59dc));
                c.sub_s(0, 22, 0);
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 12);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59d8));
                c.sub_s(12, 20, 12);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.div_s(0, 0, 2);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.mul_s(12, 12, 1);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.max_s(12, 12, 0);
                let t = c.g[V1][0] as u32;
                c.max_s(12, 12, 21);
                call_ptr(c, t, 0x0018_d1dc)?;
                c.g[AT][0] = 0x42c8_0000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x1;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x59d4));
                c.sub_s(0, 22, 0);
                c.g[AT][0] = 0x428c_0000;
                c.mtc1(1, 12);
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x59d0));
                c.sub_s(12, 20, 12);
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                c.div_s(0, 0, 2);
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.mul_s(12, 12, 1);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.max_s(12, 12, 0);
                let t = c.g[V1][0] as u32;
                c.max_s(12, 12, 21);
                call_ptr(c, t, 0x0018_d224)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_18d228;
            }
        }
        c.cycles += 6;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnWidgetCreated(self, widget)` (0x18d2e8):
/// tags a freshly created menu widget (`+0x38` name hash) with its menu-item
/// code in `+0x18` (`1saveoptions` 0, `2loadoptions` 2, `3loadreplay` 1,
/// `4new_game` 3, two more ids 0x300 and 0x200); other widgets are left alone.
pub fn c_fe_state_options_save_load_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d3ac: {
            'b_18d3a8: {
                'b_18d31c: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[A0][0] = 0x460000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f30));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x0031_7670, 0x0018_d30c)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_18d31c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_18d3a8;
                        } else {
                            break 'b_18d31c;
                        }
                    }
                }
                'b_18d3a4: {
                    'b_18d338: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f20));
                        call_ptr(c, 0x0031_7670, 0x0018_d328)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_18d338;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d338;
                            }
                        }
                    }
                    'b_18d354: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f10));
                        call_ptr(c, 0x0031_7670, 0x0018_d344)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_18d354;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d354;
                            }
                        }
                    }
                    'b_18d370: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f00));
                        call_ptr(c, 0x0031_7670, 0x0018_d360)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_18d370;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d370;
                            }
                        }
                    }
                    'b_18d38c: {
                        c.cycles += 5;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1508));
                        call_ptr(c, 0x0031_7670, 0x0018_d37c)?;
                        if c.g[S0][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_18d38c;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x300;
                            if t {
                                break 'b_18d3a4;
                            } else {
                                break 'b_18d38c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
                    call_ptr(c, 0x0031_7670, 0x0018_d398)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_18d3ac;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0x200;
                        break 'b_18d3a4;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18d3a8;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_18d3ac;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnChildEvent(self, widget, event)`
/// (0x18d460): event 0xf / 0x10 store 1 / 0 in `widget+0x18` and pop the
/// state; event 0x16 (with `widget+0x18 != 0`) starts a new profile: names
/// player `mUserNum` (`+0x44`, clamped to 0) empty, resets both front-end
/// riders, asks the custom interface (`engine+0` vtable `+0x20/+0x24`) for the
/// next state (`1 << self+0x48 | 0x100`), pushes it and plays the
/// `TransitionOut` frame of the screen (`self+0x40`). Other events do
/// nothing.
pub fn c_fe_state_options_save_load_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d638: {
            'b_18d634: {
                'b_18d630: {
                    'b_18d4e8: {
                        'b_18d4d4: {
                            'b_18d4cc: {
                                c.cycles += 9;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                c.g[V1][0] = c.g[A1][0];
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = 0x10;
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[S2][0] = c.g[A0][0];
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                if t {
                                    break 'b_18d4cc;
                                } else {
                                    'b_18d4bc: {
                                        'b_18d4a0: {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xf;
                                            if t {
                                                break 'b_18d4a0;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_18d4bc;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x20),
                                                    );
                                                    if t {
                                                        break 'b_18d638;
                                                    } else {
                                                        break 'b_18d4a0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x15;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x16;
                                        if t {
                                            break 'b_18d630;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            if t {
                                                break 'b_18d4e8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_18d638;
                                                } else {
                                                    break 'b_18d4bc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = 0x1;
                                    c.g[A1][0] = 0x1;
                                    let t = true;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                    if t {
                                        break 'b_18d4d4;
                                    } else {
                                        break 'b_18d4cc;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                            c.g[A1][0] = 0x1;
                            break 'b_18d4d4;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f190, 0x0018_d4e0)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_18d634;
                        } else {
                            break 'b_18d4e8;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_18d630;
                    } else {
                        'b_18d5fc: {
                            c.cycles += 63;
                            c.g[S1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[S1][0] = 0u64;
                            }
                            call_ptr(c, 0x0014_dc80, 0x0018_d504)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014_dd58, 0x0018_d510)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A2][0] = 0x4a0000;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1408));
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0014_7138, 0x0018_d528)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_d53c)?;
                            call_ptr(c, 0x0014_dc80, 0x0018_d544)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x0014_de28, 0x0018_d550)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                            call_ptr(c, 0x001a_0548, 0x0018_d564)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0019_eba0, 0x0018_d570)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0x4;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0019_e960, 0x0018_d584)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                            call_ptr(c, 0x001a_0548, 0x0018_d598)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0019_eba0, 0x0018_d5a4)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = 0u64;
                            c.g[A1][0] = 0x1;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x0019_e960, 0x0018_d5b8)?;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A2][0] = 0x1;
                            c.g[V0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A3][0] = c.lw(c.g[V1][0] as u32);
                            c.g[A2][0] = sx((c.g[A2][0] as u32) << ((c.g[V0][0] as u32) & 31));
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A2][0] = c.g[A2][0] | 0x100;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_d5e8)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_18d5fc;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039_f400, 0x0018_d5fc)?;
                                break 'b_18d5fc;
                            }
                        }
                        c.cycles += 10;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
                        call_ptr(c, 0x0031_7670, 0x0018_d608)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x0018_d614)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let t = c.g[A1][0] == c.g[V0][0];
                        c.g[A1][0] = c.g[A1][0] & 0xffff;
                        if t {
                            break 'b_18d630;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039_c870, 0x0018_d630)?;
                            break 'b_18d630;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_18d634;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_18d638;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsSaveLoad_OnWidgetAction(self, widget, event)`
/// (0x18d648): event 5 (select) switches on the widget's menu-item code
/// (`widget+0x18`, set to 3 on the way): the "new game" item (3) opens the
/// create-new-file confirm popup, others push the next state the custom
/// interface (`engine+0` vtable `+0x20/+0x24`) returns for `item | 1 <<
/// self+0x14`; event 6 (back) asks the previous state (vtable `+0x28/+0x2c`)
/// for the widget's item and pushes it. A null widget does nothing.
pub fn c_fe_state_options_save_load_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18d7e0: {
            'b_18d7dc: {
                'b_18d7d8: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V1][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[V1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    if t {
                        break 'b_18d7d8;
                    } else {
                        'b_18d7a0: {
                            'b_18d684: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                if t {
                                    break 'b_18d684;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_18d7a0;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_18d7e0;
                                        } else {
                                            break 'b_18d684;
                                        }
                                    }
                                }
                            }
                            'b_18d748: {
                                c.cycles += 4;
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                c.g[V0][0] = 0x3;
                                let t = c.g[A2][0] != c.g[V0][0];
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                if t {
                                    break 'b_18d748;
                                } else {
                                    c.cycles += 45;
                                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ef0));
                                    c.sb((c.g[S2][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x0031_7670, 0x0018_d6bc)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x0018_d6cc)?;
                                    c.g[A1][0] = 0x460000;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ed0));
                                    c.g[A0][0] = 0xbfc;
                                    c.g[A2][0] = 0x100;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x0031_7d70, 0x0018_d6e8)?;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A3][0] = c.g[S0][0];
                                    c.g[T0][0] = 0u64;
                                    c.g[T1][0] = 0u64;
                                    c.g[T2][0] = 0u64;
                                    c.g[T3][0] = 0u64;
                                    c.sw(c.g[SP][0] as u32, 0u64);
                                    c.g[A2][0] = c.g[S2][0];
                                    call_ptr(c, 0x001d_99c8, 0x0018_d710)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018_d72c)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039_f290, 0x0018_d740)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_18d7dc;
                                    } else {
                                        break 'b_18d748;
                                    }
                                }
                            }
                            'b_18d768: {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x4u64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    break 'b_18d768;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = 0x1;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32) << ((c.g[V1][0] as u32) & 31));
                                    c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    break 'b_18d768;
                                }
                            }
                            c.cycles += 9;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_d784)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_18d7d8;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039_f400, 0x0018_d798)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_18d7dc;
                                } else {
                                    break 'b_18d7a0;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0018_d7c4)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.g[V0][0];
                        if t {
                            break 'b_18d7d8;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f400, 0x0018_d7d8)?;
                            break 'b_18d7d8;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_18d7dc;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_18d7e0;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_OnWidgetAction(self, widget, event)`
/// (0x18dea0): the memory-card device screen's button handler. Event 5
/// (select) and event 7 (X button) pick the widget's slot (`widget+0x18`
/// into `self+0x1c4` and the overlay manager `+0xf8`) and start a save/load
/// or open a status popup depending on `self+0x1bc/+0x1c0/+0x1a8`; event 6
/// (back/cancel) closes the screen (`self+0x1c0 = 6`).
pub fn c_fe_state_options_device_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18e154: {
            'b_18e150: {
                'b_18e094: {
                    'b_18e028: {
                        c.cycles += 16;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.g[S0][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S1][0] = c.g[A2][0];
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S3][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[S4][0] = 0x6;
                        call_ptr(c, 0x0022_7f80, 0x0018_ded8)?;
                        let t = c.g[S1][0] == c.g[S4][0];
                        c.g[S2][0] = c.g[V0][0];
                        if t {
                            break 'b_18e028;
                        } else {
                            'b_18df10: {
                                'b_18defc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x5;
                                    if t {
                                        break 'b_18defc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S1][0] == c.g[V0][0];

                                        if t {
                                            break 'b_18df10;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_18e154;
                                            } else {
                                                break 'b_18defc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x7;
                                if c.g[S1][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                    break 'b_18e094;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    if t {
                                        break 'b_18e154;
                                    } else {
                                        break 'b_18df10;
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[S3][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                break 'b_18e154;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    break 'b_18e154;
                                } else {
                                    c.cycles += 5;
                                    c.g[S1][0] = 0x1;
                                    c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x214));
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[S4][0] != c.g[V0][0];
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x238), c.g[S1][0]);
                                    if t {
                                        break 'b_18e150;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x224));
                                        if c.g[V0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            break 'b_18e154;
                                        } else {
                                            'b_18e018: {
                                                'b_18e010: {
                                                    'b_18e00c: {
                                                        'b_18df8c: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.g[A0][0] = 0x2;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1c4),
                                                                c.g[V0][0],
                                                            );
                                                            c.sw(
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0xf8),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1bc));
                                                            let t = c.g[V1][0] != c.g[A0][0];
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_18df8c;
                                                            } else {
                                                                'b_18df88: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x22c));
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    if t {
                                                                        break 'b_18df88;
                                                                    } else {
                                                                        c.cycles += 7;
                                                                        c.g[A1][0] = 0x2;
                                                                        c.g[A2][0] =
                                                                            0xffffffffffffffff;
                                                                        call_ptr(
                                                                            c,
                                                                            0x0023_cae8,
                                                                            0x0018_df78,
                                                                        )?;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1a8,
                                                                                ),
                                                                            c.g[S1][0],
                                                                        );
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1c0,
                                                                                ),
                                                                            c.g[S4][0],
                                                                        );
                                                                        let t = true;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1dc,
                                                                                ),
                                                                            c.g[S1][0],
                                                                        );
                                                                        if t {
                                                                            break 'b_18e00c;
                                                                        } else {
                                                                            break 'b_18df88;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V0][0] = 0x1;
                                                                break 'b_18df8c;
                                                            }
                                                        }
                                                        'b_18dfd0: {
                                                            c.cycles += 2;
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A2][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x434));
                                                            if t {
                                                                break 'b_18dfd0;
                                                            } else {
                                                                c.cycles += 8;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x1c4));
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[A2][0] as u32);
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x218));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x21c));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x0018_dfac)?;
                                                                if c.g[V0][0] == 0u64 {
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    break 'b_18e010;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    c.g[A1][0] = 0u64;
                                                                    call_ptr(
                                                                        c,
                                                                        0x0023_c8f0,
                                                                        0x0018_dfc0,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x160));
                                                                    let t = true;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x164));
                                                                    if t {
                                                                        break 'b_18e018;
                                                                    } else {
                                                                        break 'b_18dfd0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 8;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1c4));
                                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x218));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x21c));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0018_dfe8)?;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_18e010;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[A1][0] = 0x2;
                                                            call_ptr(c, 0x0023_c8f0, 0x0018_dffc)?;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x160));
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x164));
                                                            if t {
                                                                break 'b_18e018;
                                                            } else {
                                                                break 'b_18e00c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    break 'b_18e010;
                                                }
                                                c.cycles += 2;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x178));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x17c));
                                                break 'b_18e018;
                                            }
                                            c.cycles += 4;
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018_e020)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_18e154;
                                            } else {
                                                break 'b_18e028;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x238), 0u64);
                    call_ptr(c, 0x0022_7f80, 0x0018_e034)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                    if c.g[V0][0] == c.g[A0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_18e154;
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                        if c.g[V1][0] == c.g[S4][0] {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            break 'b_18e154;
                        } else {
                            'b_18e08c: {
                                'b_18e074: {
                                    'b_18e070: {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[A0][0]);
                                        if t {
                                            break 'b_18e070;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            if c.g[V1][0] == c.g[V0][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                                break 'b_18e074;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[A0][0];
                                                c.g[V0][0] = 0x6;
                                                if t {
                                                    break 'b_18e08c;
                                                } else {
                                                    break 'b_18e070;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                    break 'b_18e074;
                                }
                                c.cycles += 6;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018_e088)?;
                                c.g[V0][0] = 0x6;
                                break 'b_18e08c;
                            }
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                            if t {
                                break 'b_18e150;
                            } else {
                                break 'b_18e094;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[S5][0] = 0x1;
                if c.g[V0][0] == c.g[S5][0] {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'b_18e154;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    if c.g[V0][0] == c.g[S4][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_18e154;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x22c));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            break 'b_18e154;
                        } else {
                            c.cycles += 12;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            call_ptr(c, 0x0022_7f80, 0x0018_e0c0)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018_e0e0)?;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                break 'b_18e154;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                if c.g[S2][0] != c.g[S5][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    break 'b_18e154;
                                } else {
                                    c.cycles += 9;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018_e11c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_18e150;
                                    } else {
                                        c.cycles += 11;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S2][0]);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x0023_d570, 0x0018_e138)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x748));
                                        c.g[A1][0] = 0x18;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018_e150)?;
                                        break 'b_18e150;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_18e154;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateOptionsDeviceSelect_v39(self, int a1, int a2)` (0x18e368):
/// switches the device screen's highlight. Looks up a UI object of the
/// screen (`self+0x40`) by the hash of `0x4a15e0`; with `self+0x22c != 0` it
/// calls vtable `+0x48/+0x4c` on the `0x4a15d0` object with `a2 ^ 1`,
/// otherwise on the `0x4a15d8` object with `a1 ^ 1`; the value is stored in
/// `self+0x224` and also sent to the first object.
pub fn c_fe_state_options_device_select_v39(c: &mut Cpu) {
    guest(c, |c| {
        'b_18e45c: {
            'b_18e408: {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = 0x4a0000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A2][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15e0));
                call_ptr(c, 0x0031_7670, 0x0018_e398)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x0018_e3a4)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x22c));
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[S0][0] ^ 0x1;
                if t {
                    break 'b_18e408;
                } else {
                    c.cycles += 21;
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15d0));
                    call_ptr(c, 0x0031_7670, 0x0018_e3c0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x0018_e3cc)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018_e3e4)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[S0][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018_e400)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_18e45c;
                    } else {
                        break 'b_18e408;
                    }
                }
            }
            c.cycles += 21;
            c.g[A0][0] = 0x4a0000;
            c.g[S0][0] = c.g[S3][0] ^ 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15d8));
            call_ptr(c, 0x0031_7670, 0x0018_e418)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e424)?;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            let t = c.g[A2][0] as u32;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e43c)?;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x224), c.g[S0][0]);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0018_e458)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_18e45c;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (a machine without it runs the
    /// callees' real bodies) and its magic.
    const OC840_CTRL: u32 = DATA + 0x20_0000;
    const OC840_MAGIC: u32 = 0x0c84_0a49;
    /// A real leaf that returns 0 (a virtual call that answers 0).
    const OC840_LEAF_ZERO: u32 = 0x0014_06c0;

    // Fixture layout (above the area the fakes' result pointers cover).
    const OC840_SELF: u32 = DATA + 0x4_0000;
    const OC840_WIDGET: u32 = DATA + 0x4_1000;
    const OC840_ENGINE: u32 = DATA + 0x4_2000;
    const OC840_CUSTOM: u32 = DATA + 0x4_3000;
    const OC840_VT_CUSTOM: u32 = DATA + 0x4_4000;
    const OC840_WORLD: u32 = DATA + 0x4_5000;
    const OC840_SUB: u32 = DATA + 0x4_6000;
    const OC840_VT_SUB: u32 = DATA + 0x4_7000;
    const OC840_MGR: u32 = DATA + 0x4_8000;
    const OC840_OBJ434: u32 = DATA + 0x4_9000;
    const OC840_VT434: u32 = DATA + 0x4_a000;
    const OC840_OBJ748: u32 = DATA + 0x4_b000;
    const OC840_VT_SELF8: u32 = DATA + 0x4_c000;
    const OC840_OBJ: u32 = DATA + 0x4_d000;
    const OC840_VT_OBJ: u32 = DATA + 0x4_e000;
    const OC840_VT_X: u32 = DATA + 0x4_f000;
    const OC840_PAD: u32 = DATA + 0x5_0000;
    const OC840_DEV: u32 = DATA + 0x5_8000;
    const OC840_VT_DEV: u32 = DATA + 0x5_9000;

    /// Mailbox of the fake for `addr`: 0 = the fake's own pointer, else the
    /// call returns `mail - 1`.
    fn oc840_mail(addr: u32) -> u32 {
        OC840_CTRL + 0x100 + (addr & 0x3f_fffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the mailbox value (minus 1) unless the call unwound.
    fn oc840_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc840_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc840_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC840_CTRL) != OC840_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc840_fake_call(c, $addr);
                }
            )*
            fn oc840_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc840_fakes!(
        (
            oc840_fake_14dc80,
            OC840_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc840_fake_14dd58,
            OC840_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc840_fake_147138,
            OC840_PREV_147138,
            ee::s01::OVERRIDE_00147138,
            ee::s01::f_00147138__generated,
            0x0014_7138
        ),
        (
            oc840_fake_14de28,
            OC840_PREV_14DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc840_fake_1a0548,
            OC840_PREV_1A0548,
            ee::s03::OVERRIDE_001A0548,
            ee::s03::f_001a0548__generated,
            0x001a_0548
        ),
        (
            oc840_fake_19eba0,
            OC840_PREV_19EBA0,
            ee::s03::OVERRIDE_0019EBA0,
            ee::s03::f_0019eba0__generated,
            0x0019_eba0
        ),
        (
            oc840_fake_19e960,
            OC840_PREV_19E960,
            ee::s03::OVERRIDE_0019E960,
            ee::s03::f_0019e960__generated,
            0x0019_e960
        ),
        (
            oc840_fake_39f190,
            OC840_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc840_fake_39f400,
            OC840_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc840_fake_39c778,
            OC840_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc840_fake_39c870,
            OC840_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc840_fake_39d860,
            OC840_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc840_fake_397840,
            OC840_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc840_fake_3978f0,
            OC840_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
        (
            oc840_fake_317d70,
            OC840_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc840_fake_1d99c8,
            OC840_PREV_1D99C8,
            ee::s04::OVERRIDE_001D99C8,
            ee::s04::f_001d99c8__generated,
            0x001d_99c8
        ),
        (
            oc840_fake_227f80,
            OC840_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc840_fake_23cae8,
            OC840_PREV_23CAE8,
            ee::s06::OVERRIDE_0023CAE8,
            ee::s06::f_0023cae8__generated,
            0x0023_cae8
        ),
        (
            oc840_fake_23c8f0,
            OC840_PREV_23C8F0,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0
        ),
        (
            oc840_fake_23d570,
            OC840_PREV_23D570,
            ee::s06::OVERRIDE_0023D570,
            ee::s06::f_0023d570__generated,
            0x0023_d570
        ),
    );

    fn oc840_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    fn oc840_gp(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// A scrambled machine with this module's fakes installed and switched on.
    fn oc840_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc840_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC840_CTRL, OC840_MAGIC);
        r
    }

    /// A vtable entry `{ delta, func }` at `vt + off`: a call through it
    /// reaches fake number `k`, or (`leaf`) a real leaf that returns 0.
    fn oc840_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, k: u32, leaf: bool) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + off, delta as u16);
        let func = if leaf {
            OC840_LEAF_ZERO
        } else {
            FAKE + 0x10 * k
        };
        c.bus.write32(vt + off + 4, func);
    }

    /// `StrHash` of the string at `addr`, by the generated hasher.
    fn oc840_hash_of(addr: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = 0x0012_3450;
        c.g[GP][0] = GAME_GP as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut c);
        c.g[V0][0] as u32
    }

    /// The options-state object `OC840_SELF`: engine and custom interface
    /// (vtable slots `+0x20/+0x24` next state, `+0x28/+0x2c` previous
    /// state), the screen/rider bytes and the world.
    fn oc840_state_fixture(c: &mut Cpu, r: &mut Rng, seed: u32, leaf: bool) {
        oc840_w(c, OC840_SELF + 0x10, OC840_ENGINE);
        oc840_w(c, OC840_ENGINE, OC840_CUSTOM);
        oc840_w(c, OC840_CUSTOM + 4, OC840_VT_CUSTOM);
        oc840_slot(c, r, OC840_VT_CUSTOM, 0x20, 1, leaf);
        oc840_slot(c, r, OC840_VT_CUSTOM, 0x28, 2, leaf);
        oc840_w(c, OC840_SELF + 0x40, r.next_u32());
        c.bus
            .write8(OC840_SELF + 0x44, [0u8, 2, 0xff, 0x80][(seed % 4) as usize]);
        c.bus.write8(OC840_SELF + 0x48, (seed % 4) as u8);
        c.bus.write8(OC840_SELF + 0x14, ((seed / 4) % 4) as u8);
        oc840_w(c, oc840_gp(-0x848), OC840_WORLD);
        oc840_w(c, OC840_WORLD + 0x7c, r.next_u32());
        oc840_w(c, OC840_WORLD + 0x8c, OC840_SUB);
        oc840_w(c, OC840_SUB + 4, OC840_VT_SUB);
        oc840_slot(c, r, OC840_VT_SUB, 0x20, 3, false);
    }

    /// A UI object `OC840_OBJ` that the fakes of `0x14dd58`, `0x1d99c8` and
    /// `0x39d860` hand out: virtual slots `+0x48`, `+0xc8` and an embedded
    /// `{delta, func}` at `+0xc -> +8`.
    fn oc840_obj_fixture(c: &mut Cpu, r: &mut Rng) {
        for addr in [0x0014_dd58u32, 0x001d_99c8, 0x0039_d860] {
            oc840_w(c, oc840_mail(addr), OC840_OBJ + 1);
        }
        oc840_w(c, OC840_OBJ + 8, OC840_VT_OBJ);
        oc840_w(c, OC840_OBJ + 0xc, OC840_VT_X);
        oc840_slot(c, r, OC840_VT_OBJ, 0x48, 4, false);
        oc840_slot(c, r, OC840_VT_OBJ, 0xc8, 5, false);
        oc840_slot(c, r, OC840_VT_X, 8, 6, false);
    }

    /// Float bits of the sliders `SetVibration` rescales.
    #[test]
    fn c_fe_state_options_controller_set_vibration_matches() {
        check(
            ee::s02::f_0018d108__generated,
            c_fe_state_options_controller_set_vibration,
            160,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d108, 3);
                let idx = [0u64, 1, 2, 3, 0xfe][(seed % 5) as usize];
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 8) | idx;
                let xs = [0.0f32, 5.0, 20.0, 50.0, 100.0, 255.0, 1000.0, -3.0];
                let ys = [0.0f32, 0.5, 10.0, 60.0, 70.0, 100.0, 200.0];
                c.f[12] = xs[(seed % 8) as usize].to_bits();
                c.f[13] = ys[((seed / 8) % 7) as usize].to_bits();
                let no_pads = seed % 9 == 8;
                oc840_w(c, oc840_gp(-0x850), if no_pads { 0 } else { OC840_PAD });
                if no_pads {
                    // A null pad cache: the device read at address 0 is a
                    // zero vtable.
                    for a in (0..0x40u32).step_by(4) {
                        oc840_w(c, a, 0);
                    }
                }
                for i in -2i32..=3 {
                    oc840_w(c, OC840_PAD + 0x2eec + (i * 4) as u32, OC840_DEV);
                }
                oc840_w(c, OC840_DEV, OC840_VT_DEV);
                oc840_slot(c, &mut r, OC840_VT_DEV, 0x18, 0, false);
                oc840_slot(c, &mut r, OC840_VT_DEV, 0x20, 1, false);
                // numActuators(): 2 (both motors) or something else.
                c.bus
                    .write32(LOG_RESULTS, if seed % 3 == 2 { 3 } else { 2 });
            },
        );
    }

    #[test]
    fn c_fe_state_options_save_load_on_widget_created_matches() {
        check(
            ee::s02::f_0018d2e8__generated,
            c_fe_state_options_save_load_on_widget_created,
            64,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d2e8, 0);
                let ids = [
                    0x0045_e0d0u32,
                    0x0045_e0e0,
                    0x0045_e0f0,
                    0x0045_e100,
                    0x004a_1508,
                    0x004a_1500,
                ];
                let k = (seed % 8) as usize;
                let hash = if k < 6 {
                    oc840_hash_of(ids[k])
                } else {
                    r.next_u32()
                };
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = OC840_WIDGET as u64;
                oc840_w(c, OC840_WIDGET + 0x38, hash);
                oc840_w(c, OC840_WIDGET + 0x18, r.next_u32());
            },
        );
    }

    /// Event 0xf / 0x10 pop the state, 0x16 starts the new-profile chain.
    #[test]
    fn c_fe_state_options_save_load_on_child_event_matches() {
        check(
            ee::s02::f_0018d460__generated,
            c_fe_state_options_save_load_on_child_event,
            240,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d460, 20);
                oc840_state_fixture(c, &mut r, seed, seed % 5 == 4);
                oc840_obj_fixture(c, &mut r);
                let ev = [0xfu64, 0x10, 0x15, 0x16, 0x16, 0x16, 0x14, 0x30][(seed % 8) as usize];
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = OC840_WIDGET as u64;
                c.g[A2][0] = ev;
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                oc840_w(c, OC840_WIDGET + 0x18, flag);
                // GetFrameByLabel: "no such frame" (0xffff) or a frame.
                if seed % 2 == 0 {
                    oc840_w(c, oc840_mail(0x0039_c778), 0x1_0000);
                }
            },
        );
    }

    /// Select (5) on each menu item, back (6), a null widget, other events.
    #[test]
    fn c_fe_state_options_save_load_on_widget_action_matches() {
        check(
            ee::s02::f_0018d648__generated,
            c_fe_state_options_save_load_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_d648, 14);
                oc840_state_fixture(c, &mut r, seed, seed % 5 == 4);
                oc840_obj_fixture(c, &mut r);
                let ev = [5u64, 5, 5, 5, 6, 6, 0, 5][(seed % 8) as usize];
                let item = [3u32, 3, 0, 1, 2, 4, 5, 0x300, 0x200][((seed / 8) % 9) as usize];
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = if seed % 13 == 12 {
                    0
                } else {
                    OC840_WIDGET as u64
                };
                c.g[A2][0] = ev;
                oc840_w(c, OC840_WIDGET + 0x18, item);
                // The popup push: the list has a head (UIList_First != 0) or not.
                if seed % 2 == 1 {
                    oc840_w(c, oc840_mail(0x0039_7840), 1);
                }
            },
        );
    }

    /// The memory-card device screen's buttons: select (5), back (6), X (7).
    #[test]
    fn c_fe_state_options_device_select_on_widget_action_matches() {
        check(
            ee::s02::f_0018dea0__generated,
            c_fe_state_options_device_select_on_widget_action,
            400,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_dea0, 8);
                let pick = |r: &mut Rng, n: u32| r.next_u32() % n;
                let s = OC840_SELF;
                oc840_w(c, s + 8, OC840_VT_SELF8);
                oc840_slot(c, &mut r, OC840_VT_SELF8, 0x160, 1, false);
                oc840_slot(c, &mut r, OC840_VT_SELF8, 0x178, 2, false);
                let v = [0u32, 0, 1, 2][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x1a8, v);
                let v = [2u32, 2, 0, 1][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x1bc, v);
                let v = [0u32, 1, 2, 3, 6, 4][pick(&mut r, 6) as usize];
                oc840_w(c, s + 0x1c0, v);
                let v = [3u32, 3, 3, 1][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x214, v);
                let v = [0u32, 1, 1, 1][pick(&mut r, 4) as usize];
                oc840_w(c, s + 0x224, v);
                let v = [0u32, 1, 1][pick(&mut r, 3) as usize];
                oc840_w(c, s + 0x22c, v);
                oc840_w(c, OC840_WIDGET + 0x18, r.next_u32() & 7);
                // The memory-card overlay manager every call returns.
                oc840_w(c, oc840_gp(-0x848), OC840_WORLD);
                oc840_w(c, oc840_mail(0x0022_7f80), OC840_MGR + 1);
                oc840_w(c, OC840_MGR + 0x434, OC840_OBJ434);
                oc840_w(c, OC840_MGR + 0x428, r.next_u32());
                oc840_w(c, OC840_OBJ434, OC840_VT434);
                let leaf = seed % 2 == 0;
                oc840_slot(c, &mut r, OC840_VT434, 0x218, 3, leaf);
                let leaf = seed % 3 == 0;
                oc840_slot(c, &mut r, OC840_VT434, 0x1b0, 4, leaf);
                oc840_w(c, OC840_MGR + 0x748, OC840_OBJ748);
                oc840_slot(c, &mut r, OC840_OBJ748, 8, 5, false);
                let ev = [5u64, 5, 5, 5, 6, 7, 7, 7, 3][(seed % 9) as usize];
                c.g[A0][0] = s as u64;
                c.g[A1][0] = if ev == 5 && seed % 11 == 0 {
                    0
                } else {
                    OC840_WIDGET as u64
                };
                c.g[A2][0] = ev;
            },
        );
    }

    #[test]
    fn c_fe_state_options_device_select_v39_matches() {
        check(
            ee::s02::f_0018e368__generated,
            c_fe_state_options_device_select_v39,
            64,
            &|c, seed| {
                let mut r = oc840_base(c, seed, 0x18_e368, 3);
                oc840_obj_fixture(c, &mut r);
                c.g[A0][0] = OC840_SELF as u64;
                c.g[A1][0] = [0u64, 1, 5][(seed % 3) as usize];
                c.g[A2][0] = [1u64, 0, 7, 0][((seed / 3) % 4) as usize];
                oc840_w(c, OC840_SELF + 0x40, r.next_u32());
                oc840_w(c, OC840_SELF + 0x22c, (seed % 2) as u32);
                oc840_w(c, OC840_SELF + 0x224, r.next_u32());
            },
        );
    }
}
