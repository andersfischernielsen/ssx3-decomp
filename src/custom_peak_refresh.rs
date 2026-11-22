//! Native Event Select refresh of the three retail peaks and Mountain.
//!
//! The selected row is addressed through UIMenu.selectedObject, and the
//! per-peak arrays use UIObject.id rather than the child's list position.
//! Reordering the peak list therefore does not change this contract.

use crate::call::{call_ptr, guest};
use crate::events;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_3d40,
    event_select_refresh_peak_list,
    "EventSelect_RefreshPeakList",
)];

const SELECTED_OBJECT: u32 = 0xa0;
const OBJECT_ID: u32 = 0x18;
const GET_CHARACTER_ID: u32 = 0x0014a080;
const GET_BE_LIBRARY: u32 = 0x0014dc80;
const GET_BE_INTERFACE: u32 = 0x0014dd58;
const STR_HASH: u32 = 0x00317670;
const FIND_SCREEN_OBJECT: u32 = 0x0039d860;
const HIDE_STATE_OBJECT: u32 = 0x0039e8b8;
const SHOW_STATE_OBJECT: u32 = 0x0039e948;
const SET_ASCII_TEXT: u32 = 0x003a0c50;
const SET_TEXT_ID: u32 = 0x003a1000;
const FORMAT_TEXT: u32 = 0x00417828;
const PEAK_GOAL_COMPLETE: u32 = 0x00157bf0;
const TRACK_MEDAL: u32 = 0x00158910;
const PEAK_COLLECTIBLE_COUNT: u32 = 0x001535c0;
const PEAK_HIGHLIGHT_LEVEL: u32 = 0x001552b0;

/// `void EventSelect_RefreshPeakList(EventSelect* self)` (0x203d40).
/// Refresh the peak's locks and mode hints. In overlay flows, also format
/// medal, collectible and highlight progress; Mountain aggregates all peaks.
/// Retail IDs 0..2 name the three peaks. IDs >=3 take the Mountain path.
/// No widget ID, menu order or menu selection is written here.
///
/// Register-backed branch state and block charges preserve the native call
/// ABI, including the progress calculation's FPU operations and event edges.
pub fn event_select_refresh_peak_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_203e54: {
            'b_203e24: {
                c.cycles += 44;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3b0));
                c.sq(S3, (c.g[SP][0] as u32).wrapping_add(0x340));
                c.sq(S5, (c.g[SP][0] as u32).wrapping_add(0x320));
                c.g[S3][0] = c.g[A0][0];
                c.sq(S7, (c.g[SP][0] as u32).wrapping_add(0x300));
                c.sq(S0, (c.g[SP][0] as u32).wrapping_add(0x370));
                c.sq(S1, (c.g[SP][0] as u32).wrapping_add(0x360));
                c.sq(S2, (c.g[SP][0] as u32).wrapping_add(0x350));
                c.sq(S4, (c.g[SP][0] as u32).wrapping_add(0x330));
                c.sq(S6, (c.g[SP][0] as u32).wrapping_add(0x310));
                c.sq(FP, (c.g[SP][0] as u32).wrapping_add(0x2f0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[RA][0]);
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x3a8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x398));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x390));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x388));
                c.g[RA][0] = sx(0x00203d8c);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x380));
                call_ptr(c, GET_BE_LIBRARY, 0x00203d8c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00203d98);
                c.g[A1][0] = 0xd;
                call_ptr(c, GET_BE_INTERFACE, 0x00203d98)?;
                c.g[RA][0] = sx(0x00203da0);
                c.g[S5][0] = c.g[V0][0];
                call_ptr(c, GET_BE_LIBRARY, 0x00203da0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00203dac);
                c.g[A1][0] = 0xa;
                call_ptr(c, GET_BE_INTERFACE, 0x00203dac)?;
                c.g[RA][0] = sx(0x00203db4);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2b0), c.g[V0][0]);
                call_ptr(c, GET_BE_LIBRARY, 0x00203db4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00203dc0);
                c.g[A1][0] = 0xb;
                call_ptr(c, GET_BE_INTERFACE, 0x00203dc0)?;
                c.g[RA][0] = sx(0x00203dc8);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2b4), c.g[V0][0]);
                call_ptr(c, GET_BE_LIBRARY, 0x00203dc8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00203dd4);
                c.g[A1][0] = 0x8;
                call_ptr(c, GET_BE_INTERFACE, 0x00203dd4)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2b8), c.g[V0][0]);
                c.g[RA][0] = sx(0x00203de0);
                c.g[A0][0] = 0u64;
                call_ptr(c, GET_CHARACTER_ID, 0x00203de0)?;
                c.g[S7][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2cc));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                if t {
                    break 'b_203e24;
                } else {
                    c.cycles += 13;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x00203dfc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xae8));
                    call_ptr(c, HIDE_STATE_OBJECT, 0x00203dfc)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x00203e0c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2648));
                    call_ptr(c, HIDE_STATE_OBJECT, 0x00203e0c)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                    c.g[RA][0] = sx(0x00203e1c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2610));
                    call_ptr(c, HIDE_STATE_OBJECT, 0x00203e1c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));

                    break 'b_203e54;
                }
            }
            c.cycles += 12;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x00203e30);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xae8));
            call_ptr(c, SHOW_STATE_OBJECT, 0x00203e30)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
            c.g[A1][0] = 0x4a0000;
            c.g[RA][0] = sx(0x00203e40);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2648));
            call_ptr(c, SHOW_STATE_OBJECT, 0x00203e40)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
            c.g[RA][0] = sx(0x00203e50);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2610));
            call_ptr(c, SHOW_STATE_OBJECT, 0x00203e50)?;
            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
            break 'b_203e54;
        }
        c.cycles += 17;
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xaf8));
        c.g[RA][0] = sx(0x00203e64);
        c.g[S0][0] = 0u64;
        call_ptr(c, HIDE_STATE_OBJECT, 0x00203e64)?;
        c.g[S2][0] = 0x470000;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
        c.g[A1][0] = 0x470000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb08));
        c.g[RA][0] = sx(0x00203e7c);
        c.g[FP][0] = 0x470000;
        call_ptr(c, HIDE_STATE_OBJECT, 0x00203e7c)?;
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
        c.g[RA][0] = sx(0x00203e8c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2650));
        call_ptr(c, HIDE_STATE_OBJECT, 0x00203e8c)?;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        // The native three-peak completion badges are found by name.
        'l_203e98: loop {
            'b_203f3c: {
                'b_203f30: {
                    'b_203f2c: {
                        c.cycles += 12;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[S1][0] = c.g[A2][0];
                        c.g[RA][0] = sx(0x00203ea8);
                        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb20));
                        call_ptr(c, FORMAT_TEXT, 0x00203ea8)?;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = c.g[S7][0];
                        c.g[A3][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x00203ec0);
                        c.g[T0][0] = 0u64;
                        call_ptr(c, PEAK_GOAL_COMPLETE, 0x00203ec0)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S5][0];
                        if t {
                            break 'b_203f2c;
                        } else {
                            c.cycles += 7;
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S7][0];
                            c.g[A3][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00203edc);
                            c.g[T0][0] = 0x1;
                            call_ptr(c, PEAK_GOAL_COMPLETE, 0x00203edc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S5][0];
                            if t {
                                break 'b_203f2c;
                            } else {
                                c.cycles += 7;
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = c.g[S7][0];
                                c.g[A3][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00203ef8);
                                c.g[T0][0] = 0x2;
                                call_ptr(c, PEAK_GOAL_COMPLETE, 0x00203ef8)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A3][0] = c.g[S0][0];
                                if t {
                                    break 'b_203f2c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[A2][0] = c.g[S7][0];
                                    c.g[RA][0] = sx(0x00203f14);
                                    c.g[T0][0] = 0x3;
                                    call_ptr(c, PEAK_GOAL_COMPLETE, 0x00203f14)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                                    if t {
                                        break 'b_203f30;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x00203f24);
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, SHOW_STATE_OBJECT, 0x00203f24)?;
                                        c.g[S0][0] = c.g[S1][0];

                                        break 'b_203f3c;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                    break 'b_203f30;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x00203f38);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, HIDE_STATE_OBJECT, 0x00203f38)?;
                c.g[S0][0] = c.g[S1][0];
                break 'b_203f3c;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
            if t {
                if events::event_due(c) {
                    events::deliver_event_here(c, 0x00203e98);
                }
                continue 'l_203e98;
            } else {
                'b_203fa0: {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_203fa0;
                    } else {
                        c.cycles += 18;
                        c.g[RA][0] = sx(0x00203f60);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x610));
                        call_ptr(c, STR_HASH, 0x00203f60)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                        c.g[RA][0] = sx(0x00203f6c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, FIND_SCREEN_OBJECT, 0x00203f6c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00203f88);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00203f88)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00203fa0);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00203fa0)?;
                        break 'b_203fa0;
                    }
                }
                c.cycles += 16;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x25d8));
                c.g[RA][0] = sx(0x00203fb4);
                c.g[S1][0] = 0u64;
                call_ptr(c, HIDE_STATE_OBJECT, 0x00203fb4)?;
                c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                c.g[A1][0] = 0x470000;
                c.g[RA][0] = sx(0x00203fc8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x548));
                call_ptr(c, HIDE_STATE_OBJECT, 0x00203fc8)?;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                c.g[RA][0] = sx(0x00203fd8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x558));
                call_ptr(c, HIDE_STATE_OBJECT, 0x00203fd8)?;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                // Refresh the three peak lock/availability indicators.
                'l_203fe0: loop {
                    'b_204060: {
                        'b_204038: {
                            c.cycles += 13;
                            c.g[V0][0] = 0x4a0000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2658));
                            c.g[A2][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00203ff8);
                            c.g[S2][0] = c.g[S0][0];
                            call_ptr(c, FORMAT_TEXT, 0x00203ff8)?;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0xb38));
                            c.g[RA][0] = sx(0x00204008);
                            c.g[A2][0] = c.g[S0][0];
                            call_ptr(c, FORMAT_TEXT, 0x00204008)?;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb54));
                            let t = c.g[V1][0] != 0u64;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                            if t {
                                break 'b_204038;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                let t = c.g[V1][0] != 0u64;

                                if t {
                                    break 'b_204038;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00204030);
                                    c.g[A1][0] = c.g[S4][0];
                                    call_ptr(c, HIDE_STATE_OBJECT, 0x00204030)?;
                                    c.g[S1][0] = c.g[S2][0];

                                    break 'b_204060;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x00204040);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, HIDE_STATE_OBJECT, 0x00204040)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S1][0] = c.g[S2][0];
                        if t {
                            break 'b_204060;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                            c.g[RA][0] = sx(0x0020405c);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, HIDE_STATE_OBJECT, 0x0020405c)?;
                            c.g[S1][0] = c.g[S2][0];
                            break 'b_204060;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    if t {
                        if events::event_due(c) {
                            events::deliver_event_here(c, 0x00203fe0);
                        }
                        continue 'l_203fe0;
                    } else {
                        // Common exit after the selected row's information has been refreshed.
                        'b_204fa8: {
                            c.cycles += 21;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00204078);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x578));
                            call_ptr(c, STR_HASH, 0x00204078)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x00204084);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, FIND_SCREEN_OBJECT, 0x00204084)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002040a0);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002040a0)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x002040ac);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
                            call_ptr(c, STR_HASH, 0x002040ac)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                            c.g[RA][0] = sx(0x002040b8);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, FIND_SCREEN_OBJECT, 0x002040b8)?;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(S0, (c.g[SP][0] as u32).wrapping_add(0x370));
                            if t {
                                break 'b_204fa8;
                            } else {
                                c.cycles += 3;
                                // UIMenu.selectedObject; no child-list index is used below.
                                c.g[S0][0] =
                                    c.lw((c.g[V0][0] as u32).wrapping_add(SELECTED_OBJECT));
                                if c.g[S0][0] == 0u64 {
                                    c.lq(S0, (c.g[SP][0] as u32).wrapping_add(0x370));
                                    break 'b_204fa8;
                                } else {
                                    // Stable row IDs below three use per-peak progress; all other IDs
                                    // take the Mountain summary path, independently of visual row order.
                                    'b_2046b8: {
                                        c.cycles += 4;
                                        c.g[V0][0] =
                                            c.lw((c.g[S0][0] as u32).wrapping_add(OBJECT_ID));
                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                                        if t {
                                            break 'b_2046b8;
                                        } else {
                                            'b_2041ac: {
                                                'b_204194: {
                                                    'b_204178: {
                                                        'b_204138: {
                                                            c.cycles += 15;
                                                            c.g[A1][0] = 0x470000;
                                                            c.g[RA][0] = sx(0x002040e8);
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x568));
                                                            call_ptr(
                                                                c,
                                                                HIDE_STATE_OBJECT,
                                                                0x002040e8,
                                                            )?;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0x1;
                                                            c.g[FP][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x00204104);
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x00204104)?;
                                                            c.g[V0][0] =
                                                                sx((c.g[FP][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            let t = c.g[V1][0] == 0u64;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xb5c));
                                                            if t {
                                                                break 'b_204138;
                                                            } else {
                                                                'b_204130: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x2u64)
                                                                        as u64;
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[A0][0] = 0x470000;
                                                                        break 'b_204130;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x470000;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xb50,
                                                                                ));

                                                                        break 'b_204178;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0xb68));

                                                                break 'b_204178;
                                                            }
                                                        }
                                                        'b_204164: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x1;
                                                            let t = c.g[FP][0] != c.g[V0][0];
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xb5c));
                                                            if t {
                                                                break 'b_204164;
                                                            } else {
                                                                'b_20415c: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x2u64)
                                                                        as u64;
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[A0][0] = 0x470000;
                                                                        break 'b_20415c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x470000;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xb80,
                                                                                ));

                                                                        break 'b_204178;
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0xb98));

                                                                break 'b_204178;
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = 0x470000;
                                                        if t {
                                                            break 'b_204194;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[A0][0] = 0x470000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xbb0));
                                                            break 'b_204178;
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[RA][0] = sx(0x00204180);
                                                    call_ptr(c, STR_HASH, 0x00204180)?;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[RA][0] = sx(0x0020418c);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, SET_TEXT_ID, 0x0020418c)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0xb5c)
                                                        );

                                                    break 'b_2041ac;
                                                }
                                                c.cycles += 6;
                                                c.g[RA][0] = sx(0x0020419c);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0xbc8));
                                                call_ptr(c, STR_HASH, 0x0020419c)?;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x002041a8);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, SET_TEXT_ID, 0x002041a8)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                                                break 'b_2041ac;
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                            if c.g[V0][0] != 0u64 {
                                                c.lq(S0, (c.g[SP][0] as u32).wrapping_add(0x370));
                                                break 'b_204fa8;
                                            } else {
                                                'b_20436c: {
                                                    c.cycles += 41;
                                                    c.g[V1][0] = 0x4;
                                                    c.g[A0][0] = 0x470000;
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2bc),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[RA][0] = sx(0x002041cc);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0xbe0));
                                                    call_ptr(c, STR_HASH, 0x002041cc)?;
                                                    c.g[S4][0] = 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x2e8)
                                                        );
                                                    c.g[RA][0] = sx(0x002041dc);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, FIND_SCREEN_OBJECT, 0x002041dc)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[S2][0] = c.g[V0][0];
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0xbf8));
                                                    c.g[S1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                    c.g[V0][0] = c
                                                        .lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                    c.g[RA][0] = sx(0x00204200);
                                                    c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    call_ptr(c, STR_HASH, 0x00204200)?;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00204210);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, t, 0x00204210)?;
                                                    c.g[S0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[A2][0] =
                                                        sx((c.g[FP][0] as u32).wrapping_add(0x1));
                                                    c.g[RA][0] = sx(0x00204224);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x002c26d0, 0x00204224)?;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[RA][0] = sx(0x00204230);
                                                    c.g[A1][0] = c.g[S0][0];
                                                    call_ptr(c, 0x003a0e90, 0x00204230)?;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2bc)
                                                        );
                                                    c.g[V1][0] = 0x3;
                                                    c.g[V0][0] = c.g[FP][0] ^ 0x2;
                                                    c.g[A0][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x240));
                                                    if c.g[V0][0] == 0 {
                                                        c.g[A1][0] = c.g[V1][0];
                                                    }
                                                    c.g[S6][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2bc),
                                                        c.g[A1][0],
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2c8),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2bc)
                                                        );
                                                    let t = c.g[A2][0] == 0u64;
                                                    c.g[A1][0] = 0u64;
                                                    if t {
                                                        break 'b_20436c;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x1b0;
                                                        c.mult(0, 18, 30, 2);
                                                        c.g[V0][0] = 0x6c;
                                                        'l_204268: loop {
                                                            'b_204320: {
                                                                'b_2042e8: {
                                                                    c.cycles += 8;
                                                                    c.g[A2][0] = 0x480000;
                                                                    c.mult(0, 2, 5, 2);
                                                                    c.g[A2][0] = sx((c.g[A2][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x7e30));
                                                                    c.g[V1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[S2][0] as u32),
                                                                        ));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[A2][0] as u32),
                                                                        ));
                                                                    c.g[V0][0] = c.g[V1][0] & 0x7;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S6][0];
                                                                    if t {
                                                                        break 'b_2042e8;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[S1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x60,
                                                                                ));
                                                                        c.g[S0][0] =
                                                                            sx((c.g[S4][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        'l_204294: loop {
                                                                            c.cycles += 19;
                                                                            c.ldl(
                                                                                7,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x7,
                                                                                    ),
                                                                            );
                                                                            c.ldr(
                                                                                7,
                                                                                (c.g[V1][0] as u32),
                                                                            );
                                                                            c.ldl(
                                                                                8,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf,
                                                                                    ),
                                                                            );
                                                                            c.ldr(
                                                                                8,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.ldl(
                                                                                5,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x17,
                                                                                    ),
                                                                            );
                                                                            c.ldr(
                                                                                5,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            c.ldl(
                                                                                6,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1f,
                                                                                    ),
                                                                            );
                                                                            c.ldr(
                                                                                6,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            c.sdl(
                                                                                7,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x7,
                                                                                    ),
                                                                            );
                                                                            c.sdr(
                                                                                7,
                                                                                (c.g[A0][0] as u32),
                                                                            );
                                                                            c.sdl(
                                                                                8,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xf,
                                                                                    ),
                                                                            );
                                                                            c.sdr(
                                                                                8,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.sdl(
                                                                                5,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x17,
                                                                                    ),
                                                                            );
                                                                            c.sdr(
                                                                                5,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            c.sdl(
                                                                                6,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1f,
                                                                                    ),
                                                                            );
                                                                            c.sdr(
                                                                                6,
                                                                                (c.g[A0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ));
                                                                            let t = c.g[V1][0]
                                                                                != c.g[V0][0];
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ));
                                                                            if t {
                                                                                if events::event_due(
                                                                                    c,
                                                                                ) {
                                                                                    events::deliver_event_here(c, 0x00204294);
                                                                                }
                                                                                continue 'l_204294;
                                                                            } else {
                                                                                c.cycles += 2;

                                                                                break 'b_204320;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[S1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.g[V0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x60));
                                                                c.g[S0][0] = sx((c.g[S4][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                'l_2042f4: loop {
                                                                    c.cycles += 11;
                                                                    c.g[A3][0] =
                                                                        c.ld((c.g[V1][0] as u32));
                                                                    c.g[T0][0] = c
                                                                        .ld((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    c.g[A1][0] = c
                                                                        .ld((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[A2][0] = c
                                                                        .ld((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    c.sd(
                                                                        (c.g[A0][0] as u32),
                                                                        c.g[A3][0],
                                                                    );
                                                                    c.sd(
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                        c.g[T0][0],
                                                                    );
                                                                    c.sd(
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                        c.g[A1][0],
                                                                    );
                                                                    c.sd(
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[A2][0],
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x20));
                                                                    let t =
                                                                        c.g[V1][0] != c.g[V0][0];
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x20));
                                                                    if t {
                                                                        if events::event_due(c) {
                                                                            events::deliver_event_here(c, 0x002042f4);
                                                                        }
                                                                        continue 'l_2042f4;
                                                                    } else {
                                                                        break 'b_204320;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 19;
                                                            c.ldl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(7, (c.g[V1][0] as u32));
                                                            c.g[T0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.sdl(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(7, (c.g[A0][0] as u32));
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[T0][0],
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x1d8));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[T0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x1d0));
                                                            c.g[RA][0] = sx(0x00204350);
                                                            c.g[A0][0] = c.g[S5][0];
                                                            call_ptr(c, TRACK_MEDAL, 0x00204350)?;
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2bc));
                                                            c.g[A1][0] = c.g[S1][0];
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S4][0] = c.g[S0][0];
                                                            }
                                                            c.g[V0][0] = ((c.g[A1][0] as i64)
                                                                < (c.g[A2][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x6c;
                                                            if t {
                                                                if events::event_due(c) {
                                                                    events::deliver_event_here(
                                                                        c, 0x00204268,
                                                                    );
                                                                }
                                                                continue 'l_204268;
                                                            } else {
                                                                break 'b_20436c;
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_2043e4: {
                                                    'b_2043d4: {
                                                        c.cycles += 20;
                                                        c.g[A0][0] = 0x470000;
                                                        c.g[RA][0] = sx(0x00204378);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xc10));
                                                        call_ptr(c, STR_HASH, 0x00204378)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2e8));
                                                        c.g[RA][0] = sx(0x00204384);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(
                                                            c,
                                                            FIND_SCREEN_OBJECT,
                                                            0x00204384,
                                                        )?;
                                                        c.g[A3][0] = 0x4a0000;
                                                        c.g[S2][0] = c.g[V0][0];
                                                        c.g[A1][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x2398));
                                                        c.g[A0][0] = c.g[S6][0];
                                                        c.g[A3][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2bc));
                                                        c.g[RA][0] = sx(0x002043a0);
                                                        c.g[A2][0] = c.g[S4][0];
                                                        call_ptr(c, FORMAT_TEXT, 0x002043a0)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[RA][0] = sx(0x002043ac);
                                                        c.g[A1][0] = c.g[S6][0];
                                                        call_ptr(c, SET_ASCII_TEXT, 0x002043ac)?;
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2bc));
                                                        c.g[V0][0] = ((c.g[S4][0] as i64)
                                                            < (c.g[T0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V1][0] = 0x470000;
                                                        if t {
                                                            break 'b_2043d4;
                                                        } else {
                                                            c.cycles += 6;
                                                            c.g[V0][0] = 0x470000;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                            c.g[RA][0] = sx(0x002043cc);
                                                            c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc20));
                                                            call_ptr(
                                                                c,
                                                                SHOW_STATE_OBJECT,
                                                                0x002043cc,
                                                            )?;
                                                            c.g[V0][0] = 0x21c;

                                                            break 'b_2043e4;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x2ec)
                                                        );
                                                    c.g[RA][0] = sx(0x002043e0);
                                                    c.g[A1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0xc20));
                                                    call_ptr(c, HIDE_STATE_OBJECT, 0x002043e0)?;
                                                    c.g[V0][0] = 0x21c;
                                                    break 'b_2043e4;
                                                }
                                                c.cycles += 5;
                                                c.g[S4][0] = 0u64;
                                                c.mult(0, 18, 30, 2);
                                                c.g[A1][0] = 0u64;
                                                c.g[V0][0] = 0x6c;
                                                'l_2043f8: loop {
                                                    'b_2044b0: {
                                                        'b_204478: {
                                                            c.cycles += 8;
                                                            c.g[A2][0] = 0x480000;
                                                            c.mult(0, 2, 5, 2);
                                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x7920));
                                                            c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[S2][0] as u32)));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[A2][0] as u32)));
                                                            c.g[V0][0] = c.g[V1][0] & 0x7;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2c8));
                                                            if t {
                                                                break 'b_204478;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[S1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.g[V0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x60));
                                                                c.g[S0][0] = sx((c.g[S4][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                'l_204424: loop {
                                                                    c.cycles += 19;
                                                                    c.ldl(
                                                                        7,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x7),
                                                                    );
                                                                    c.ldr(7, (c.g[V1][0] as u32));
                                                                    c.ldl(
                                                                        8,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0xf),
                                                                    );
                                                                    c.ldr(
                                                                        8,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.ldl(
                                                                        5,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x17),
                                                                    );
                                                                    c.ldr(
                                                                        5,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                    );
                                                                    c.ldl(
                                                                        6,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x1f),
                                                                    );
                                                                    c.ldr(
                                                                        6,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                    );
                                                                    c.sdl(
                                                                        7,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x7),
                                                                    );
                                                                    c.sdr(7, (c.g[A0][0] as u32));
                                                                    c.sdl(
                                                                        8,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0xf),
                                                                    );
                                                                    c.sdr(
                                                                        8,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.sdl(
                                                                        5,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x17),
                                                                    );
                                                                    c.sdr(
                                                                        5,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                    );
                                                                    c.sdl(
                                                                        6,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x1f),
                                                                    );
                                                                    c.sdr(
                                                                        6,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x20));
                                                                    let t =
                                                                        c.g[V1][0] != c.g[V0][0];
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x20));
                                                                    if t {
                                                                        if events::event_due(c) {
                                                                            events::deliver_event_here(c, 0x00204424);
                                                                        }
                                                                        continue 'l_204424;
                                                                    } else {
                                                                        c.cycles += 2;

                                                                        break 'b_2044b0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[S1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x60));
                                                        c.g[S0][0] =
                                                            sx((c.g[S4][0] as u32)
                                                                .wrapping_add(0x1));
                                                        'l_204484: loop {
                                                            c.cycles += 11;
                                                            c.g[A3][0] = c.ld((c.g[V1][0] as u32));
                                                            c.g[T0][0] = c.ld((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = c.ld((c.g[V1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.ld((c.g[V1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.sd((c.g[A0][0] as u32), c.g[A3][0]);
                                                            c.sd(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[T0][0],
                                                            );
                                                            c.sd(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A1][0],
                                                            );
                                                            c.sd(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if events::event_due(c) {
                                                                    events::deliver_event_here(
                                                                        c, 0x00204484,
                                                                    );
                                                                }
                                                                continue 'l_204484;
                                                            } else {
                                                                break 'b_2044b0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 18;
                                                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                    c.ldr(7, (c.g[V1][0] as u32));
                                                    c.g[T0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                    c.sdr(7, (c.g[A0][0] as u32));
                                                    c.g[A2][0] = c.g[S7][0];
                                                    c.sw(
                                                        (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        c.g[T0][0],
                                                    );
                                                    c.g[A3][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x248)
                                                        );
                                                    c.g[A1][0] = 0u64;
                                                    c.g[T0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x240)
                                                        );
                                                    c.g[RA][0] = sx(0x002044e0);
                                                    c.g[A0][0] = c.g[S5][0];
                                                    call_ptr(c, TRACK_MEDAL, 0x002044e0)?;
                                                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                    c.g[A1][0] = c.g[S1][0];
                                                    if c.g[V0][0] != 0 {
                                                        c.g[S4][0] = c.g[S0][0];
                                                    }
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 5i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x6c;
                                                    if t {
                                                        if events::event_due(c) {
                                                            events::deliver_event_here(
                                                                c, 0x002043f8,
                                                            );
                                                        }
                                                        continue 'l_2043f8;
                                                    } else {
                                                        'b_204568: {
                                                            'b_204558: {
                                                                c.cycles += 18;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[RA][0] = sx(0x00204504);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0xc30));
                                                                call_ptr(c, STR_HASH, 0x00204504)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x2e8));
                                                                c.g[RA][0] = sx(0x00204510);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c,
                                                                    FIND_SCREEN_OBJECT,
                                                                    0x00204510,
                                                                )?;
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[S2][0] = c.g[V0][0];
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2660));
                                                                c.g[A0][0] = c.g[S6][0];
                                                                c.g[RA][0] = sx(0x00204528);
                                                                c.g[A2][0] = c.g[S4][0];
                                                                call_ptr(
                                                                    c,
                                                                    FORMAT_TEXT,
                                                                    0x00204528,
                                                                )?;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[RA][0] = sx(0x00204534);
                                                                c.g[A1][0] = c.g[S6][0];
                                                                call_ptr(
                                                                    c,
                                                                    SET_ASCII_TEXT,
                                                                    0x00204534,
                                                                )?;
                                                                c.g[V0][0] = ((c.g[S4][0] as i64)
                                                                    < 5i64)
                                                                    as u64;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[A3][0] = 0x470000;
                                                                if t {
                                                                    break 'b_204558;
                                                                } else {
                                                                    c.cycles += 6;
                                                                    c.g[A2][0] = 0x470000;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x2ec));
                                                                    c.g[RA][0] = sx(0x00204550);
                                                                    c.g[A1][0] = sx((c.g[A2][0]
                                                                        as u32)
                                                                        .wrapping_add(0xc48));
                                                                    call_ptr(
                                                                        c,
                                                                        SHOW_STATE_OBJECT,
                                                                        0x00204550,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x2b0));

                                                                    break 'b_204568;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                            c.g[RA][0] = sx(0x00204564);
                                                            c.g[A1][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0xc48));
                                                            call_ptr(
                                                                c,
                                                                HIDE_STATE_OBJECT,
                                                                0x00204564,
                                                            )?;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2b0));
                                                            break 'b_204568;
                                                        }
                                                        'b_20462c: {
                                                            'b_20461c: {
                                                                c.cycles += 39;
                                                                c.g[RA][0] = sx(0x00204570);
                                                                c.g[A1][0] = c.g[FP][0];
                                                                call_ptr(
                                                                    c, 0x001543e0, 0x00204570,
                                                                )?;
                                                                c.g[S1][0] = c.g[V0][0];
                                                                c.g[A0][0] = c.g[S5][0];
                                                                c.g[A1][0] = c.g[FP][0];
                                                                c.g[RA][0] = sx(0x00204584);
                                                                c.g[A2][0] = 0x3;
                                                                call_ptr(
                                                                    c, 0x00158890, 0x00204584,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x2b0));
                                                                c.g[S1][0] = ((c.g[S1][0] as i64)
                                                                    < (c.g[V0][0] as i64))
                                                                    as u64;
                                                                c.g[S1][0] = c.g[S1][0] ^ 0x1;
                                                                c.g[A1][0] = 0u64;
                                                                c.g[RA][0] = sx(0x0020459c);
                                                                c.g[A2][0] = c.g[FP][0];
                                                                call_ptr(
                                                                    c,
                                                                    PEAK_COLLECTIBLE_COUNT,
                                                                    0x0020459c,
                                                                )?;
                                                                c.g[A2][0] = 0x3;
                                                                c.g[A1][0] = c.g[FP][0];
                                                                c.g[A0][0] = c.g[S5][0];
                                                                c.g[RA][0] = sx(0x002045b0);
                                                                c.g[S0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00158870, 0x002045b0,
                                                                )?;
                                                                c.g[S0][0] = ((c.g[S0][0] as i64)
                                                                    < (c.g[V0][0] as i64))
                                                                    as u64;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[V0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0xc60));
                                                                c.g[RA][0] = sx(0x002045c8);
                                                                if c.g[S0][0] == 0 {
                                                                    c.g[S1][0] = c.g[V0][0];
                                                                }
                                                                call_ptr(c, STR_HASH, 0x002045c8)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x2e8));
                                                                c.g[RA][0] = sx(0x002045d4);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c,
                                                                    FIND_SCREEN_OBJECT,
                                                                    0x002045d4,
                                                                )?;
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[A2][0] = c.g[S1][0];
                                                                c.g[S2][0] = c.g[V0][0];
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2668));
                                                                c.g[RA][0] = sx(0x002045ec);
                                                                c.g[A0][0] = c.g[S6][0];
                                                                call_ptr(
                                                                    c,
                                                                    FORMAT_TEXT,
                                                                    0x002045ec,
                                                                )?;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[RA][0] = sx(0x002045f8);
                                                                c.g[A1][0] = c.g[S6][0];
                                                                call_ptr(
                                                                    c,
                                                                    SET_ASCII_TEXT,
                                                                    0x002045f8,
                                                                )?;
                                                                c.g[S1][0] = ((c.g[S1][0] as i64)
                                                                    < 2i64)
                                                                    as u64;
                                                                let t = c.g[S1][0] != 0u64;
                                                                c.g[V0][0] = 0x470000;
                                                                if t {
                                                                    break 'b_20461c;
                                                                } else {
                                                                    c.cycles += 6;
                                                                    c.g[T0][0] = 0x470000;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x2ec));
                                                                    c.g[RA][0] = sx(0x00204614);
                                                                    c.g[A1][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        .wrapping_add(0xc70));
                                                                    call_ptr(
                                                                        c,
                                                                        SHOW_STATE_OBJECT,
                                                                        0x00204614,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x2b4));

                                                                    break 'b_20462c;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                            c.g[RA][0] = sx(0x00204628);
                                                            c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc70));
                                                            call_ptr(
                                                                c,
                                                                HIDE_STATE_OBJECT,
                                                                0x00204628,
                                                            )?;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2b4));
                                                            break 'b_20462c;
                                                        }
                                                        'b_2046a0: {
                                                            c.cycles += 24;
                                                            c.g[RA][0] = sx(0x00204634);
                                                            c.g[A1][0] = 0u64;
                                                            call_ptr(c, 0x001509f8, 0x00204634)?;
                                                            c.g[A1][0] = c.g[FP][0];
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[RA][0] = sx(0x00204644);
                                                            c.g[S0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001588b0, 0x00204644)?;
                                                            c.g[A0][0] = 0x470000;
                                                            c.g[S1][0] =
                                                                (c.g[V0][0] < c.g[S0][0]) as u64;
                                                            c.g[RA][0] = sx(0x00204654);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xc88));
                                                            call_ptr(c, STR_HASH, 0x00204654)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2e8));
                                                            c.g[RA][0] = sx(0x00204660);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(
                                                                c,
                                                                FIND_SCREEN_OBJECT,
                                                                0x00204660,
                                                            )?;
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.g[S2][0] = c.g[V0][0];
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x2670));
                                                            c.g[A0][0] = c.g[S6][0];
                                                            c.g[RA][0] = sx(0x00204678);
                                                            c.g[A2][0] = c.g[S1][0];
                                                            call_ptr(c, FORMAT_TEXT, 0x00204678)?;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[RA][0] = sx(0x00204684);
                                                            c.g[A1][0] = c.g[S6][0];
                                                            call_ptr(
                                                                c,
                                                                SET_ASCII_TEXT,
                                                                0x00204684,
                                                            )?;
                                                            let t = (c.g[S1][0] as i64) <= 0;
                                                            c.g[V1][0] = 0x470000;
                                                            if t {
                                                                break 'b_2046a0;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x2ec));
                                                                c.g[RA][0] = sx(0x00204698);
                                                                c.g[A1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0xc98));
                                                                call_ptr(
                                                                    c,
                                                                    SHOW_STATE_OBJECT,
                                                                    0x00204698,
                                                                )?;
                                                                c.lq(
                                                                    S0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x370),
                                                                );

                                                                break 'b_204fa8;
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[A2][0] = 0x470000;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                        c.g[RA][0] = sx(0x002046b0);
                                                        c.g[A1][0] =
                                                            sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0xc98));
                                                        call_ptr(c, HIDE_STATE_OBJECT, 0x002046b0)?;
                                                        c.lq(
                                                            S0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x370),
                                                        );

                                                        break 'b_204fa8;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 24;
                                    c.g[A1][0] = 0x470000;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x2c0), 0u64);
                                    c.g[RA][0] = sx(0x002046c8);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x568));
                                    call_ptr(c, SHOW_STATE_OBJECT, 0x002046c8)?;
                                    c.g[S6][0] = 0u64;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x002046dc);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2608));
                                    call_ptr(c, HIDE_STATE_OBJECT, 0x002046dc)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x002046e8);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xcb0));
                                    call_ptr(c, STR_HASH, 0x002046e8)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                                    c.g[RA][0] = sx(0x002046f4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, FIND_SCREEN_OBJECT, 0x002046f4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[S1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00204704);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xcc8));
                                    call_ptr(c, STR_HASH, 0x00204704)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00204710);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, SET_TEXT_ID, 0x00204710)?;
                                    c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                    c.g[S1][0] = 0x4;
                                    'l_204718: loop {
                                        'b_204884: {
                                            c.cycles += 7;
                                            c.g[V1][0] = 0x3;
                                            c.g[V0][0] = c.g[S6][0] ^ 0x2;
                                            c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                                            c.g[A1][0] = 0u64;
                                            if c.g[V0][0] == 0 {
                                                c.g[S1][0] = c.g[V1][0];
                                            }
                                            let t = c.g[S1][0] == 0u64;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x2c4),
                                                c.g[A3][0],
                                            );
                                            if t {
                                                break 'b_204884;
                                            } else {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x1b0;
                                                c.g[V1][0] = 0x480000;
                                                c.mult(0, 20, 22, 2);
                                                c.g[T1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_sub(0x7e30));
                                                c.g[S2][0] = 0x1;
                                                c.g[V0][0] = 0x6c;
                                                'l_204750: loop {
                                                    'b_2047f8: {
                                                        'b_2047c4: {
                                                            c.cycles += 6;
                                                            c.mult(0, 2, 5, 2);
                                                            c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[S4][0] as u32)));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[T1][0] as u32)));
                                                            c.g[V0][0] = c.g[V1][0] & 0x7;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.g[FP][0];
                                                            if t {
                                                                break 'b_2047c4;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[S0][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                c.g[V0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x60));
                                                                'l_204770: loop {
                                                                    c.cycles += 19;
                                                                    c.ldl(
                                                                        5,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x7),
                                                                    );
                                                                    c.ldr(5, (c.g[V1][0] as u32));
                                                                    c.ldl(
                                                                        6,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0xf),
                                                                    );
                                                                    c.ldr(
                                                                        6,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.ldl(
                                                                        7,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x17),
                                                                    );
                                                                    c.ldr(
                                                                        7,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                    );
                                                                    c.ldl(
                                                                        8,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x1f),
                                                                    );
                                                                    c.ldr(
                                                                        8,
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                    );
                                                                    c.sdl(
                                                                        5,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x7),
                                                                    );
                                                                    c.sdr(5, (c.g[A0][0] as u32));
                                                                    c.sdl(
                                                                        6,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0xf),
                                                                    );
                                                                    c.sdr(
                                                                        6,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x8),
                                                                    );
                                                                    c.sdl(
                                                                        7,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x17),
                                                                    );
                                                                    c.sdr(
                                                                        7,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                    );
                                                                    c.sdl(
                                                                        8,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x1f),
                                                                    );
                                                                    c.sdr(
                                                                        8,
                                                                        (c.g[A0][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x20));
                                                                    let t =
                                                                        c.g[V1][0] != c.g[V0][0];
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x20));
                                                                    if t {
                                                                        if events::event_due(c) {
                                                                            events::deliver_event_here(c, 0x00204770);
                                                                        }
                                                                        continue 'l_204770;
                                                                    } else {
                                                                        c.cycles += 2;

                                                                        break 'b_2047f8;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        c.g[S0][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x60));
                                                        'l_2047cc: loop {
                                                            c.cycles += 11;
                                                            c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                                                            c.g[A2][0] = c.ld((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A3][0] = c.ld((c.g[V1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[T0][0] = c.ld((c.g[V1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                                                            c.sd(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[A2][0],
                                                            );
                                                            c.sd(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A3][0],
                                                            );
                                                            c.sd(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                                c.g[T0][0],
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if events::event_due(c) {
                                                                    events::deliver_event_here(
                                                                        c, 0x002047cc,
                                                                    );
                                                                }
                                                                continue 'l_2047cc;
                                                            } else {
                                                                break 'b_2047f8;
                                                            }
                                                        }
                                                    }
                                                    'b_20483c: {
                                                        c.cycles += 10;
                                                        c.ldl(
                                                            2,
                                                            (c.g[V1][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.ldr(2, (c.g[V1][0] as u32));
                                                        c.g[A1][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.sdl(
                                                            2,
                                                            (c.g[A0][0] as u32).wrapping_add(0x7),
                                                        );
                                                        c.sdr(2, (c.g[A0][0] as u32));
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                            c.g[A1][0],
                                                        );
                                                        c.g[V1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x48));
                                                        c.g[V0][0] = c.g[V1][0] ^ 0x4;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = 0u64;
                                                        if t {
                                                            break 'b_20483c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.g[V1][0] ^ 0x6;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = c.g[V1][0];
                                                            if t {
                                                                break 'b_20483c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c.g[A0][0] ^ 0x7;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = c.g[A0][0] ^ 0x8;
                                                                if t {
                                                                    break 'b_20483c;
                                                                } else {
                                                                    c.cycles += 1;
                                                                    if c.g[V0][0] != 0 {
                                                                        c.g[A1][0] = c.g[S2][0];
                                                                    }
                                                                    break 'b_20483c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_204874: {
                                                        c.cycles += 2;
                                                        let t = c.g[A1][0] == 0u64;
                                                        c.g[T0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x40));
                                                        if t {
                                                            break 'b_204874;
                                                        } else {
                                                            c.cycles += 12;
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A3][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[A1][0] = 0u64;
                                                            c.sq(
                                                                9,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x2d0),
                                                            );
                                                            c.g[RA][0] = sx(0x0020485c);
                                                            c.g[A2][0] = c.g[S7][0];
                                                            call_ptr(c, TRACK_MEDAL, 0x0020485c)?;
                                                            c.g[T0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2c0));
                                                            c.g[V0][0] =
                                                                (c.g[S2][0] < c.g[V0][0]) as u64;
                                                            c.lq(
                                                                9,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x2d0),
                                                            );
                                                            c.g[V1][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if c.g[V0][0] == 0 {
                                                                c.g[T0][0] = c.g[V1][0];
                                                            }
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x2c0),
                                                                c.g[T0][0],
                                                            );
                                                            break 'b_204874;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[V0][0] = ((c.g[A1][0] as i64)
                                                        < (c.g[S1][0] as i64))
                                                        as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x6c;
                                                    if t {
                                                        if events::event_due(c) {
                                                            events::deliver_event_here(
                                                                c, 0x00204750,
                                                            );
                                                        }
                                                        continue 'l_204750;
                                                    } else {
                                                        break 'b_204884;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x21c;
                                        c.g[V1][0] = 0x480000;
                                        c.mult(0, 18, 22, 2);
                                        c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7920));
                                        c.g[A1][0] = 0u64;
                                        c.g[S1][0] = 0x1;
                                        c.g[V0][0] = 0x6c;
                                        'l_2048a0: loop {
                                            'b_204948: {
                                                'b_204914: {
                                                    c.cycles += 6;
                                                    c.mult(0, 2, 5, 2);
                                                    c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[S2][0] as u32)));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[S4][0] as u32)));
                                                    c.g[V0][0] = c.g[V1][0] & 0x7;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[FP][0];
                                                    if t {
                                                        break 'b_204914;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[S0][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x60));
                                                        'l_2048c0: loop {
                                                            c.cycles += 19;
                                                            c.ldl(
                                                                5,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, (c.g[V1][0] as u32));
                                                            c.ldl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                5,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(5, (c.g[A0][0] as u32));
                                                            c.sdl(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if events::event_due(c) {
                                                                    events::deliver_event_here(
                                                                        c, 0x002048c0,
                                                                    );
                                                                }
                                                                continue 'l_2048c0;
                                                            } else {
                                                                c.cycles += 2;

                                                                break 'b_204948;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[S0][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x60));
                                                'l_20491c: loop {
                                                    c.cycles += 11;
                                                    c.g[A1][0] = c.ld((c.g[V1][0] as u32));
                                                    c.g[A2][0] =
                                                        c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                                                    c.g[A3][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[T0][0] = c
                                                        .ld((c.g[V1][0] as u32).wrapping_add(0x18));
                                                    c.sd((c.g[A0][0] as u32), c.g[A1][0]);
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x8),
                                                        c.g[A2][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x10),
                                                        c.g[A3][0],
                                                    );
                                                    c.sd(
                                                        (c.g[A0][0] as u32).wrapping_add(0x18),
                                                        c.g[T0][0],
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[V1][0] != c.g[V0][0];
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if events::event_due(c) {
                                                            events::deliver_event_here(
                                                                c, 0x0020491c,
                                                            );
                                                        }
                                                        continue 'l_20491c;
                                                    } else {
                                                        break 'b_204948;
                                                    }
                                                }
                                            }
                                            'b_20498c: {
                                                c.cycles += 10;
                                                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                c.ldr(2, (c.g[V1][0] as u32));
                                                c.g[A1][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                                c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                c.sdr(2, (c.g[A0][0] as u32));
                                                c.sw(
                                                    (c.g[A0][0] as u32).wrapping_add(0x8),
                                                    c.g[A1][0],
                                                );
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
                                                c.g[V0][0] = c.g[V1][0] ^ 0x5;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0u64;
                                                if t {
                                                    break 'b_20498c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c.g[V1][0] ^ 0x9;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[V1][0];
                                                    if t {
                                                        break 'b_20498c;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c.g[A0][0] ^ 0xa;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = c.g[A0][0] ^ 0xb;
                                                        if t {
                                                            break 'b_20498c;
                                                        } else {
                                                            c.cycles += 1;
                                                            if c.g[V0][0] != 0 {
                                                                c.g[A1][0] = c.g[S1][0];
                                                            }
                                                            break 'b_20498c;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_2049bc: {
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == 0u64;
                                                c.g[T0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_2049bc;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[A0][0] = c.g[S5][0];
                                                    c.g[A3][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x48));
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x002049a8);
                                                    c.g[A2][0] = c.g[S7][0];
                                                    call_ptr(c, TRACK_MEDAL, 0x002049a8)?;
                                                    c.g[T0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2c0)
                                                        );
                                                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                                                    c.g[V1][0] =
                                                        sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                                    if c.g[V0][0] == 0 {
                                                        c.g[T0][0] = c.g[V1][0];
                                                    }
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x2c0),
                                                        c.g[T0][0],
                                                    );
                                                    break 'b_2049bc;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[V0][0] = ((c.g[A1][0] as i64) < 5i64) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[V0][0] = 0x6c;
                                            if t {
                                                if events::event_due(c) {
                                                    events::deliver_event_here(c, 0x002048a0);
                                                }
                                                continue 'l_2048a0;
                                            } else {
                                                c.cycles += 4;
                                                c.g[S6][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x2c4));
                                                c.g[V0][0] = ((c.g[S6][0] as i64) < 3i64) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[S1][0] = 0x4;
                                                if t {
                                                    if events::event_due(c) {
                                                        events::deliver_event_here(c, 0x00204718);
                                                    }
                                                    continue 'l_204718;
                                                } else {
                                                    'b_204a50: {
                                                        'b_204a40: {
                                                            c.cycles += 19;
                                                            c.g[A0][0] = 0x470000;
                                                            c.g[RA][0] = sx(0x002049e8);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xcd8));
                                                            call_ptr(c, STR_HASH, 0x002049e8)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2e8));
                                                            c.g[RA][0] = sx(0x002049f4);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(
                                                                c,
                                                                FIND_SCREEN_OBJECT,
                                                                0x002049f4,
                                                            )?;
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2c0));
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.g[S1][0] = c.g[V0][0];
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x2678));
                                                            c.g[RA][0] = sx(0x00204a0c);
                                                            c.g[A0][0] = c.g[FP][0];
                                                            call_ptr(c, FORMAT_TEXT, 0x00204a0c)?;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x00204a18);
                                                            c.g[A1][0] = c.g[FP][0];
                                                            call_ptr(
                                                                c,
                                                                SET_ASCII_TEXT,
                                                                0x00204a18,
                                                            )?;
                                                            c.g[V1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2c0));
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < 14i64)
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[A3][0] = 0x470000;
                                                            if t {
                                                                break 'b_204a40;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[A2][0] = 0x470000;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x2ec));
                                                                c.g[RA][0] = sx(0x00204a38);
                                                                c.g[A1][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(0xce8));
                                                                call_ptr(
                                                                    c,
                                                                    SHOW_STATE_OBJECT,
                                                                    0x00204a38,
                                                                )?;
                                                                c.g[A0][0] = 0x470000;

                                                                break 'b_204a50;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                        c.g[RA][0] = sx(0x00204a4c);
                                                        c.g[A1][0] =
                                                            sx((c.g[A3][0] as u32)
                                                                .wrapping_add(0xce8));
                                                        call_ptr(c, HIDE_STATE_OBJECT, 0x00204a4c)?;
                                                        c.g[A0][0] = 0x470000;
                                                        break 'b_204a50;
                                                    }
                                                    'b_204c68: {
                                                        'b_204c58: {
                                                            c.cycles += 124;
                                                            c.g[RA][0] = sx(0x00204a58);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xd00));
                                                            call_ptr(c, STR_HASH, 0x00204a58)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2e8));
                                                            c.g[RA][0] = sx(0x00204a64);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(
                                                                c,
                                                                FIND_SCREEN_OBJECT,
                                                                0x00204a64,
                                                            )?;
                                                            c.g[S1][0] = c.g[V0][0];
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x4;
                                                            c.g[RA][0] = sx(0x00204a80);
                                                            c.g[T0][0] = 0xe;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204a80)?;
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[S0][0] = (0u64 < c.g[V0][0]) as u64;
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x6;
                                                            c.g[RA][0] = sx(0x00204aa0);
                                                            c.g[T0][0] = 0xe;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204aa0)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x5;
                                                            c.g[RA][0] = sx(0x00204ac4);
                                                            c.g[T0][0] = 0xe;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204ac4)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x9;
                                                            c.g[RA][0] = sx(0x00204ae8);
                                                            c.g[T0][0] = 0xe;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204ae8)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x4;
                                                            c.g[RA][0] = sx(0x00204b0c);
                                                            c.g[T0][0] = 0xf;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204b0c)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x7;
                                                            c.g[RA][0] = sx(0x00204b30);
                                                            c.g[T0][0] = 0xf;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204b30)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x5;
                                                            c.g[RA][0] = sx(0x00204b54);
                                                            c.g[T0][0] = 0xf;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204b54)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0xa;
                                                            c.g[RA][0] = sx(0x00204b78);
                                                            c.g[T0][0] = 0xf;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204b78)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x4;
                                                            c.g[RA][0] = sx(0x00204b9c);
                                                            c.g[T0][0] = 0x10;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204b9c)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x8;
                                                            c.g[RA][0] = sx(0x00204bc0);
                                                            c.g[T0][0] = 0x10;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204bc0)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A2][0] = c.g[S7][0];
                                                            c.g[A3][0] = 0x5;
                                                            c.g[RA][0] = sx(0x00204be4);
                                                            c.g[T0][0] = 0x10;
                                                            call_ptr(c, TRACK_MEDAL, 0x00204be4)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A3][0] = 0xb;
                                                            c.g[T0][0] = 0x10;
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.g[S5][0];
                                                            c.g[RA][0] = sx(0x00204c08);
                                                            c.g[A2][0] = c.g[S7][0];
                                                            call_ptr(c, TRACK_MEDAL, 0x00204c08)?;
                                                            c.g[V1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                            if c.g[V0][0] != 0 {
                                                                c.g[S0][0] = c.g[V1][0];
                                                            }
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x2680));
                                                            c.g[A0][0] = c.g[FP][0];
                                                            c.g[RA][0] = sx(0x00204c28);
                                                            c.g[A2][0] = c.g[S0][0];
                                                            call_ptr(c, FORMAT_TEXT, 0x00204c28)?;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x00204c34);
                                                            c.g[A1][0] = c.g[FP][0];
                                                            call_ptr(
                                                                c,
                                                                SET_ASCII_TEXT,
                                                                0x00204c34,
                                                            )?;
                                                            c.g[V0][0] = ((c.g[S0][0] as i64)
                                                                < 12i64)
                                                                as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x470000;
                                                            if t {
                                                                break 'b_204c58;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[T0][0] = 0x470000;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x2ec));
                                                                c.g[RA][0] = sx(0x00204c50);
                                                                c.g[A1][0] = sx((c.g[T0][0]
                                                                    as u32)
                                                                    .wrapping_add(0xd10));
                                                                call_ptr(
                                                                    c,
                                                                    SHOW_STATE_OBJECT,
                                                                    0x00204c50,
                                                                )?;
                                                                c.g[A0][0] = 0x470000;

                                                                break 'b_204c68;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x2ec));
                                                        c.g[RA][0] = sx(0x00204c64);
                                                        c.g[A1][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0xd10));
                                                        call_ptr(c, HIDE_STATE_OBJECT, 0x00204c64)?;
                                                        c.g[A0][0] = 0x470000;
                                                        break 'b_204c68;
                                                    }
                                                    c.cycles += 20;
                                                    c.lwc1(
                                                        24,
                                                        (c.g[SP][0] as u32).wrapping_add(0x2c0),
                                                    );
                                                    c.cvt_s_w(24, 24);
                                                    c.mtc1(S0, 25);
                                                    c.cvt_s_w(25, 25);
                                                    c.g[RA][0] = sx(0x00204c80);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0xd28));
                                                    call_ptr(c, STR_HASH, 0x00204c80)?;
                                                    c.g[S2][0] = 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x2e8)
                                                        );
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[S4][0] = 0u64;
                                                    c.g[RA][0] = sx(0x00204c98);
                                                    c.g[S0][0] = 0u64;
                                                    call_ptr(c, FIND_SCREEN_OBJECT, 0x00204c98)?;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00204ca8);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0xd40));
                                                    call_ptr(c, STR_HASH, 0x00204ca8)?;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    c.g[RA][0] = sx(0x00204cb4);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, SET_TEXT_ID, 0x00204cb4)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x2b0)
                                                        );
                                                    'l_204cb8: loop {
                                                        c.cycles += 11;
                                                        c.g[RA][0] = sx(0x00204cc0);
                                                        c.g[A1][0] = c.g[S0][0];
                                                        call_ptr(c, 0x001543e0, 0x00204cc0)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2b0));
                                                        c.g[A1][0] = c.g[S0][0];
                                                        c.g[S2][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[RA][0] = sx(0x00204cd4);
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        call_ptr(c, 0x00154368, 0x00204cd4)?;
                                                        c.g[S4][0] = sx((c.g[S4][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[V0][0] =
                                                            ((c.g[S0][0] as i64) < 3i64) as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2b0));
                                                            if events::event_due(c) {
                                                                events::deliver_event_here(
                                                                    c, 0x00204cb8,
                                                                );
                                                            }
                                                            continue 'l_204cb8;
                                                        } else {
                                                            'b_204d54: {
                                                                'b_204d48: {
                                                                    c.cycles += 19;
                                                                    c.g[A0][0] = 0x470000;
                                                                    c.g[RA][0] = sx(0x00204cf0);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0xd58));
                                                                    call_ptr(
                                                                        c, STR_HASH, 0x00204cf0,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x2e8));
                                                                    c.g[RA][0] = sx(0x00204cfc);
                                                                    c.g[A1][0] = c.g[V0][0];
                                                                    call_ptr(
                                                                        c,
                                                                        FIND_SCREEN_OBJECT,
                                                                        0x00204cfc,
                                                                    )?;
                                                                    c.g[V1][0] = 0x4a0000;
                                                                    c.g[S1][0] = c.g[V0][0];
                                                                    c.g[A1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x2398));
                                                                    c.g[A0][0] = c.g[FP][0];
                                                                    c.g[A2][0] = c.g[S2][0];
                                                                    c.g[RA][0] = sx(0x00204d18);
                                                                    c.g[A3][0] = c.g[S4][0];
                                                                    call_ptr(
                                                                        c,
                                                                        FORMAT_TEXT,
                                                                        0x00204d18,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[RA][0] = sx(0x00204d24);
                                                                    c.g[A1][0] = c.g[FP][0];
                                                                    call_ptr(
                                                                        c,
                                                                        SET_ASCII_TEXT,
                                                                        0x00204d24,
                                                                    )?;
                                                                    c.g[V0][0] = ((c.g[S2][0]
                                                                        as i64)
                                                                        < (c.g[S4][0] as i64))
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[A3][0] = 0x470000;
                                                                    if t {
                                                                        break 'b_204d48;
                                                                    } else {
                                                                        c.cycles += 6;
                                                                        c.g[A2][0] = 0x470000;
                                                                        c.g[A0][0] = c.lw((c.g[S3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2ec));
                                                                        c.g[RA][0] = sx(0x00204d40);
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A2][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xd68,
                                                                                ));
                                                                        call_ptr(
                                                                            c,
                                                                            SHOW_STATE_OBJECT,
                                                                            0x00204d40,
                                                                        )?;

                                                                        break 'b_204d54;
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x2ec));
                                                                c.g[RA][0] = sx(0x00204d54);
                                                                c.g[A1][0] = sx((c.g[A3][0]
                                                                    as u32)
                                                                    .wrapping_add(0xd68));
                                                                call_ptr(
                                                                    c,
                                                                    HIDE_STATE_OBJECT,
                                                                    0x00204d54,
                                                                )?;
                                                                break 'b_204d54;
                                                            }
                                                            c.cycles += 9;
                                                            c.mtc1(S2, 23);
                                                            c.cvt_s_w(23, 23);
                                                            c.g[S0][0] = 0u64;
                                                            c.mtc1(S4, 22);
                                                            c.cvt_s_w(22, 22);
                                                            c.g[S2][0] = 0u64;
                                                            c.g[S4][0] = 0u64;
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x2b0));
                                                            'l_204d78: loop {
                                                                c.cycles += 12;
                                                                c.g[A2][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x00204d84);
                                                                c.g[A1][0] = 0u64;
                                                                call_ptr(
                                                                    c,
                                                                    PEAK_COLLECTIBLE_COUNT,
                                                                    0x00204d84,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x2b0));
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[S2][0] = sx((c.g[S2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[RA][0] = sx(0x00204d98);
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                call_ptr(
                                                                    c, 0x00153390, 0x00204d98,
                                                                )?;
                                                                c.g[S4][0] = sx((c.g[S4][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[V0][0] = ((c.g[S0][0] as i64)
                                                                    < 3i64)
                                                                    as u64;
                                                                if c.g[V0][0] != 0u64 {
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x2b0));
                                                                    if events::event_due(c) {
                                                                        events::deliver_event_here(
                                                                            c, 0x00204d78,
                                                                        );
                                                                    }
                                                                    continue 'l_204d78;
                                                                } else {
                                                                    'b_204e1c: {
                                                                        'b_204e0c: {
                                                                            c.cycles += 19;
                                                                            c.g[A0][0] = 0x470000;
                                                                            c.g[RA][0] =
                                                                                sx(0x00204db4);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xd80,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, STR_HASH,
                                                                                0x00204db4,
                                                                            )?;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S3][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x2e8,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x00204dc0);
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c,
                                                                                FIND_SCREEN_OBJECT,
                                                                                0x00204dc0,
                                                                            )?;
                                                                            c.g[T0][0] = 0x4a0000;
                                                                            c.g[S1][0] = c.g[V0][0];
                                                                            c.g[A1][0] =
                                                                                sx((c.g[T0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2398,
                                                                                    ));
                                                                            c.g[A0][0] = c.g[FP][0];
                                                                            c.g[A2][0] = c.g[S2][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00204ddc);
                                                                            c.g[A3][0] = c.g[S4][0];
                                                                            call_ptr(
                                                                                c,
                                                                                FORMAT_TEXT,
                                                                                0x00204ddc,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00204de8);
                                                                            c.g[A1][0] = c.g[FP][0];
                                                                            call_ptr(
                                                                                c,
                                                                                SET_ASCII_TEXT,
                                                                                0x00204de8,
                                                                            )?;
                                                                            c.g[V0][0] = ((c.g[S2]
                                                                                [0]
                                                                                as i64)
                                                                                < (c.g[S4][0]
                                                                                    as i64))
                                                                                as u64;
                                                                            let t =
                                                                                c.g[V0][0] != 0u64;
                                                                            c.g[V1][0] = 0x470000;
                                                                            if t {
                                                                                break 'b_204e0c;
                                                                            } else {
                                                                                c.cycles += 6;
                                                                                c.g[V0][0] =
                                                                                    0x470000;
                                                                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                                                                                c.g[RA][0] =
                                                                                    sx(0x00204e04);
                                                                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0xd90));
                                                                                call_ptr(c, SHOW_STATE_OBJECT, 0x00204e04)?;
                                                                                c.g[A0][0] =
                                                                                    0x470000;

                                                                                break 'b_204e1c;
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.g[A0][0] = c.lw((c.g[S3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2ec));
                                                                        c.g[RA][0] = sx(0x00204e18);
                                                                        c.g[A1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xd90,
                                                                                ));
                                                                        call_ptr(
                                                                            c,
                                                                            HIDE_STATE_OBJECT,
                                                                            0x00204e18,
                                                                        )?;
                                                                        c.g[A0][0] = 0x470000;
                                                                        break 'b_204e1c;
                                                                    }
                                                                    c.cycles += 19;
                                                                    c.mtc1(S2, 21);
                                                                    c.cvt_s_w(21, 21);
                                                                    c.mtc1(S4, 20);
                                                                    c.cvt_s_w(20, 20);
                                                                    c.g[RA][0] = sx(0x00204e34);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0xda8));
                                                                    call_ptr(
                                                                        c, STR_HASH, 0x00204e34,
                                                                    )?;
                                                                    c.g[S2][0] = 0u64;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x2e8));
                                                                    c.g[A1][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x00204e48);
                                                                    c.g[S0][0] = 0u64;
                                                                    call_ptr(
                                                                        c,
                                                                        FIND_SCREEN_OBJECT,
                                                                        0x00204e48,
                                                                    )?;
                                                                    c.g[A0][0] = 0x470000;
                                                                    c.g[S1][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x00204e58);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0xdc0));
                                                                    call_ptr(
                                                                        c, STR_HASH, 0x00204e58,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[RA][0] = sx(0x00204e64);
                                                                    c.g[A1][0] = c.g[V0][0];
                                                                    call_ptr(
                                                                        c,
                                                                        SET_TEXT_ID,
                                                                        0x00204e64,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x2b8));
                                                                    'l_204e68: loop {
                                                                        c.cycles += 7;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        c.g[RA][0] = sx(0x00204e74);
                                                                        c.g[S0][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        call_ptr(
                                                                            c,
                                                                            PEAK_HIGHLIGHT_LEVEL,
                                                                            0x00204e74,
                                                                        )?;
                                                                        c.g[S2][0] =
                                                                            sx((c.g[S2][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[V0][0] = ((c.g[S0][0]
                                                                            as i64)
                                                                            < 8i64)
                                                                            as u64;
                                                                        if c.g[V0][0] != 0u64 {
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x2b8,
                                                                                ));
                                                                            if events::event_due(c)
                                                                            {
                                                                                events::deliver_event_here(c, 0x00204e68);
                                                                            }
                                                                            continue 'l_204e68;
                                                                        } else {
                                                                            'b_204eec: {
                                                                                'b_204ee0: {
                                                                                    c.cycles += 17;
                                                                                    c.g[A1][0] =
                                                                                        0x4a0000;
                                                                                    c.g[A2][0] =
                                                                                        c.g[S2][0];
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2688));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x00204e98,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[FP][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        FORMAT_TEXT,
                                                                                        0x00204e98,
                                                                                    )?;
                                                                                    c.g[A0][0] =
                                                                                        0x470000;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x00204ea4,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xdd8));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        STR_HASH,
                                                                                        0x00204ea4,
                                                                                    )?;
                                                                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2e8));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x00204eb0,
                                                                                    );
                                                                                    c.g[A1][0] =
                                                                                        c.g[V0][0];
                                                                                    call_ptr(c, FIND_SCREEN_OBJECT, 0x00204eb0)?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x00204ebc,
                                                                                    );
                                                                                    c.g[A1][0] =
                                                                                        c.g[FP][0];
                                                                                    call_ptr(c, SET_ASCII_TEXT, 0x00204ebc)?;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[S2][0]
                                                                                        as i64)
                                                                                        < 24i64)
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != 0u64;
                                                                                    c.g[A3][0] =
                                                                                        0x470000;
                                                                                    if t {
                                                                                        break 'b_204ee0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            6;
                                                                                        c.g[A2][0] = 0x470000;
                                                                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                                                                                        c.g[RA][0] = sx(0x00204ed8);
                                                                                        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0xde8));
                                                                                        call_ptr(c, SHOW_STATE_OBJECT, 0x00204ed8)?;
                                                                                        let t =
                                                                                            true;

                                                                                        if t {
                                                                                            break 'b_204eec;
                                                                                        } else {
                                                                                            break 'b_204ee0;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 3;
                                                                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2ec));
                                                                                c.g[RA][0] =
                                                                                    sx(0x00204eec);
                                                                                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xde8));
                                                                                call_ptr(c, HIDE_STATE_OBJECT, 0x00204eec)?;
                                                                                break 'b_204eec;
                                                                            }
                                                                            'b_204f68: {
                                                                                c.cycles += 23;
                                                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x552c));
                                                                                c.div_s(7, 23, 22);
                                                                                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5528));
                                                                                c.div_s(6, 21, 20);
                                                                                c.mul_s(0, 24, 0);
                                                                                c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x5524));
                                                                                c.mul_s(2, 25, 2);
                                                                                c.g[AT][0] =
                                                                                    0x41a00000;
                                                                                c.mtc1(1, 5);
                                                                                c.mtc1(S2, 1);
                                                                                c.cvt_s_w(1, 1);
                                                                                c.g[AT][0] =
                                                                                    0x42c80000;
                                                                                c.mtc1(1, 4);
                                                                                c.add_s(0, 0, 2);
                                                                                c.mul_s(1, 1, 3);
                                                                                c.add_s(0, 0, 7);
                                                                                c.add_s(0, 0, 6);
                                                                                c.add_s(0, 0, 1);
                                                                                c.mul_s(1, 0, 5);
                                                                                c.c_eq_s(1, 4);
                                                                                let t = c.fcc();
                                                                                c.g[S0][0] = 0x64;
                                                                                if t {
                                                                                    break 'b_204f68;
                                                                                } else {
                                                                                    c.cycles += 8;
                                                                                    c.g[AT][0] =
                                                                                        0x42c60000;
                                                                                    c.mtc1(1, 0);
                                                                                    c.cvt_w_s(2, 1);
                                                                                    c.mfc1(S0, 2);
                                                                                    c.c_le_s(0, 1);
                                                                                    if c.fcc() {
                                                                                        c.g[S0]
                                                                                            [0] =
                                                                                            0x63;
                                                                                        break 'b_204f68;
                                                                                    } else {
                                                                                        break 'b_204f68;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 16;
                                                                            c.g[A0][0] = 0x470000;
                                                                            c.g[RA][0] =
                                                                                sx(0x00204f74);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xe00,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, STR_HASH,
                                                                                0x00204f74,
                                                                            )?;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S3][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x2e8,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x00204f80);
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c,
                                                                                FIND_SCREEN_OBJECT,
                                                                                0x00204f80,
                                                                            )?;
                                                                            c.g[A1][0] = 0x4a0000;
                                                                            c.g[S1][0] = c.g[V0][0];
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x2690,
                                                                                    ));
                                                                            c.g[A0][0] = c.g[FP][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00204f98);
                                                                            c.g[A2][0] = c.g[S0][0];
                                                                            call_ptr(
                                                                                c,
                                                                                FORMAT_TEXT,
                                                                                0x00204f98,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x00204fa4);
                                                                            c.g[A1][0] = c.g[FP][0];
                                                                            call_ptr(
                                                                                c,
                                                                                SET_ASCII_TEXT,
                                                                                0x00204fa4,
                                                                            )?;
                                                                            c.lq(
                                                                                S0,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x370,
                                                                                    ),
                                                                            );
                                                                            break 'b_204fa8;
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
                                }
                            }
                        }
                        c.cycles += 17;
                        c.lq(S1, (c.g[SP][0] as u32).wrapping_add(0x360));
                        c.lq(S2, (c.g[SP][0] as u32).wrapping_add(0x350));
                        c.lq(S3, (c.g[SP][0] as u32).wrapping_add(0x340));
                        c.lq(S4, (c.g[SP][0] as u32).wrapping_add(0x330));
                        c.lq(S5, (c.g[SP][0] as u32).wrapping_add(0x320));
                        c.lq(S6, (c.g[SP][0] as u32).wrapping_add(0x310));
                        c.lq(S7, (c.g[SP][0] as u32).wrapping_add(0x300));
                        c.lq(FP, (c.g[SP][0] as u32).wrapping_add(0x2f0));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2e0));
                        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x3a8));
                        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x398));
                        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x390));
                        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x388));
                        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x380));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3b0));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}
