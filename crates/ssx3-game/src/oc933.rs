//! Staging module oc933 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc933`.

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
        0x001c_7388,
        c_fe_popup_confirm_layout_options,
        "cFEPopupConfirm_LayoutOptions",
    ),
    (
        0x001c_7c20,
        c_fe_popup_confirm_layout_frame,
        "cFEPopupConfirm_LayoutFrame",
    ),
    (
        0x001c_8a38,
        c_fe_popup_confirm_reset_layout,
        "cFEPopupConfirm_ResetLayout",
    ),
    (
        0x001c_9cd0,
        c_fe_popup_confirm_adjust_draw_layer,
        "cFEPopupConfirm_AdjustDrawLayer",
    ),
];

/// `void cFEPopupConfirm_LayoutOptions(self, info)` (0x1c7388): lay out the
/// option buttons of a confirm popup. `info[0]` is the option count: the
/// background object's flag 8 (`self+0x6c -> +0x90`) is set for 0 or 1 options
/// and cleared otherwise; the text entry is laid out
/// (`cFEPopupConfirm_LayoutTextEntry`), the `+0x70` widget is shown only when
/// there are options, unused option widgets (`self+0x90 + 4*i`, i >= count) are
/// made static and hidden, and every used one gets its text, size and measured
/// width (the widest goes to `+0x29c`, the sizes are accumulated into
/// `+0x260`) before it becomes non-static and visible. Finally `+0x264` gets
/// 10.0 added and, when the text entry layout asked for it, `info[0] = -1`.
pub fn c_fe_popup_confirm_layout_options(c: &mut Cpu) {
    if c.pc != 0x001c7388 {
        return ee::s03::f_001c7388__generated(c);
    }
    guest(c, |c| {
        'b_1c73ec: {
            'b_1c73dc: {
                'b_1c73c8: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S5][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.g[S2][0] = c.g[A0][0];
                    if t {
                        break 'b_1c73c8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                            break 'b_1c73dc;
                        } else {
                            break 'b_1c73c8;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x90));
                c.g[V0][0] = c.g[V0][0] | 0x8;
                let t = true;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x90), c.g[V0][0]);
                if t {
                    break 'b_1c73ec;
                } else {
                    break 'b_1c73dc;
                }
            }
            c.cycles += 4;
            c.g[V1][0] = 0xfffffffffffffff7;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x90));
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            c.sw((c.g[A0][0] as u32).wrapping_add(0x90), c.g[V0][0]);
            break 'b_1c73ec;
        }
        'b_1c744c: {
            'b_1c7430: {
                c.cycles += 9;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x290), 0u64);
                c.g[A0][0] = c.g[S2][0];
                c.sw((c.g[S2][0] as u32).wrapping_add(0x29c), 0u64);
                c.g[RA][0] = sx(0x001c7400);
                c.g[A1][0] = c.g[S5][0];
                Callee {
                    addr: 0x001c6d30,
                    f: ee::s03::cFEPopupConfirm_LayoutTextEntry,
                }
                .call(c, 0x001c7400)?;
                c.g[S6][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S5][0] as u32));
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_1c7430;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c7428);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c7428)?;
                    let t = true;
                    c.g[S1][0] = c.lw((c.g[S5][0] as u32));
                    if t {
                        break 'b_1c744c;
                    } else {
                        break 'b_1c7430;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001c7448);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001c7448)?;
            c.g[S1][0] = c.lw((c.g[S5][0] as u32));
            break 'b_1c744c;
        }
        'b_1c74ac: {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
            if t {
                break 'b_1c74ac;
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x90));
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                'l_1c7460: loop {
                    'b_1c749c: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1c749c;
                        } else {
                            c.cycles += 12;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c7480);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c7480)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c749c);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c749c)?;
                            break 'b_1c749c;
                        }
                    }
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c7460);
                        }
                        continue 'l_1c7460;
                    } else {
                        break 'b_1c74ac;
                    }
                }
            }
        }
        'b_1c757c: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S5][0] as u32));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = 0u64;
            if t {
                break 'b_1c757c;
            } else {
                c.cycles += 2;
                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x90));
                'l_1c74c0: loop {
                    'b_1c756c: {
                        c.cycles += 3;
                        c.g[S1][0] = c.lw((c.g[S4][0] as u32));
                        if c.g[S1][0] == 0u64 {
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32));
                            break 'b_1c756c;
                        } else {
                            'b_1c7528: {
                                c.cycles += 23;
                                c.g[A0][0] = c.g[S5][0];
                                c.g[RA][0] = sx(0x001c74d8);
                                c.g[A1][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x001c5800,
                                    f: ee::s03::cFEPopupConfirm_CfgGetOption,
                                }
                                .call(c, 0x001c74d8)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001c74ec);
                                c.g[A2][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001c8050,
                                    f: ee::s03::cFEPopupConfirm_SetTextFromDesc,
                                }
                                .call(c, 0x001c74ec)?;
                                c.g[A2][0] = c.g[SP][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001c74fc);
                                c.g[A1][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001c8628,
                                    f: ee::s03::cFEPopupConfirm_GetWidgetSize,
                                }
                                .call(c, 0x001c74fc)?;
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001c7510);
                                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                Callee {
                                    addr: 0x001c8320,
                                    f: ee::s03::cFEPopupConfirm_MeasureDesc,
                                }
                                .call(c, 0x001c7510)?;
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x29c));
                                c.c_lt_s(0, 1);
                                if c.fcc() {
                                    c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x29c));
                                    break 'b_1c7528;
                                } else {
                                    break 'b_1c7528;
                                }
                            }
                            c.cycles += 17;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x260));
                            c.g[RA][0] = sx(0x001c7538);
                            c.g[A2][0] = c.g[SP][0];
                            Callee {
                                addr: 0x001c9b68,
                                f: ee::s03::cFEPopupConfirm_AccumulateSize,
                            }
                            .call(c, 0x001c7538)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c7550);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c7550)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c7568);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c7568)?;
                            c.g[V0][0] = c.lw((c.g[S5][0] as u32));
                            break 'b_1c756c;
                        }
                    }
                    c.cycles += 4;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c74c0);
                        }
                        continue 'l_1c74c0;
                    } else {
                        break 'b_1c757c;
                    }
                }
            }
        }
        'b_1c759c: {
            c.cycles += 6;
            c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x264));
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 1);
            c.add_s(0, 0, 1);
            let t = c.g[S6][0] == 0u64;
            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x264));
            if t {
                break 'b_1c759c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffffffffffffffff;
                c.sw((c.g[S5][0] as u32), c.g[V0][0]);
                break 'b_1c759c;
            }
        }
        c.cycles += 10;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_LayoutFrame(self)` (0x1c7c20): position the popup's
/// frame: compute the centred x (`GetCenteredX + self+0x26c`) and y
/// (`self+0x270`), rescale and position the background, then shift the stored
/// positions (`+0x44` vec3) of the widgets at `+0x6c`/`+0x70` by (x, y) and of
/// the text widgets at `+0x74`, `+0x7c`, `+0x78` by (x, y) (x set to
/// `centred + 320.0` when `+0x2cc` is set, else added).
pub fn c_fe_popup_confirm_layout_frame(c: &mut Cpu) {
    if c.pc != 0x001c7c20 {
        return ee::s03::f_001c7c20__generated(c);
    }
    guest(c, |c| {
        'b_1c7cf4: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[RA][0] = sx(0x001c7c3c);
            c.g[S0][0] = c.g[A0][0];
            Callee {
                addr: 0x001c7be0,
                f: ee::s03::cFEPopupConfirm_GetCenteredX,
            }
            .call(c, 0x001c7c3c)?;
            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x26c));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x270));
            c.g[A0][0] = c.g[S0][0];
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x267));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x260));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x268));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[SP][0] as u32));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.g[RA][0] = sx(0x001c7c6c);
            c.add_s(20, 0, 1);
            Callee {
                addr: 0x001c7620,
                f: ee::s03::cFEPopupConfirm_ComputeBackgroundScale,
            }
            .call(c, 0x001c7c6c)?;
            c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x26c));
            c.g[A0][0] = c.g[S0][0];
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x267));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x260));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x268));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[SP][0] as u32));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001c7c98);
            c.mov_s(13, 21);
            Callee {
                addr: 0x001c78c0,
                f: ee::s03::cFEPopupConfirm_PositionBackground,
            }
            .call(c, 0x001c7c98)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c7cf4;
            } else {
                c.cycles += 20;
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(3, (c.g[SP][0] as u32));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x26c));
                c.lwc1(1, (c.g[SP][0] as u32));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.add_s(1, 1, 0);
                c.add_s(2, 2, 21);
                c.swc1(1, (c.g[SP][0] as u32));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[SP][0] as u32));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c7cf4;
            }
        }
        'b_1c7d4c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c7d4c;
            } else {
                c.cycles += 20;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(6, (c.g[SP][0] as u32));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x26c));
                c.lwc1(1, (c.g[SP][0] as u32));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.add_s(1, 1, 0);
                c.add_s(2, 2, 21);
                c.swc1(1, (c.g[SP][0] as u32));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[SP][0] as u32));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c7d4c;
            }
        }
        'b_1c7dbc: {
            c.cycles += 2;
            if c.g[V1][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c7dbc;
            } else {
                'b_1c7d8c: {
                    'b_1c7d88: {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x4b));
                        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x44));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(4, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        if c.g[V0][0] == 0u64 {
                            c.lwc1(0, (c.g[SP][0] as u32));
                            break 'b_1c7d88;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x43a00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.add_s(0, 20, 0);
                            if t {
                                break 'b_1c7d8c;
                            } else {
                                break 'b_1c7d88;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.add_s(0, 0, 20);
                    break 'b_1c7d8c;
                }
                c.cycles += 12;
                c.swc1(0, (c.g[SP][0] as u32));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.add_s(0, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[SP][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c7dbc;
            }
        }
        'b_1c7e2c: {
            c.cycles += 2;
            if c.g[V1][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c7e2c;
            } else {
                'b_1c7dfc: {
                    'b_1c7df8: {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                        c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x4b));
                        c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(7, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                        if c.g[V0][0] == 0u64 {
                            c.lwc1(0, (c.g[SP][0] as u32));
                            break 'b_1c7df8;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x43a00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.add_s(0, 20, 0);
                            if t {
                                break 'b_1c7dfc;
                            } else {
                                break 'b_1c7df8;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.add_s(0, 0, 20);
                    break 'b_1c7dfc;
                }
                c.cycles += 12;
                c.swc1(0, (c.g[SP][0] as u32));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                c.add_s(0, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[SP][0] as u32));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c7e2c;
            }
        }
        'b_1c7e9c: {
            c.cycles += 2;
            if c.g[V1][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1c7e9c;
            } else {
                'b_1c7e6c: {
                    'b_1c7e68: {
                        c.cycles += 9;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2cc));
                        c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x4b));
                        c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x44));
                        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(6, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        if c.g[V0][0] == 0u64 {
                            c.lwc1(0, (c.g[SP][0] as u32));
                            break 'b_1c7e68;
                        } else {
                            c.cycles += 4;
                            c.g[AT][0] = 0x43a00000;
                            c.mtc1(1, 0);
                            let t = true;
                            c.add_s(0, 20, 0);
                            if t {
                                break 'b_1c7e6c;
                            } else {
                                break 'b_1c7e68;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.add_s(0, 0, 20);
                    break 'b_1c7e6c;
                }
                c.cycles += 12;
                c.swc1(0, (c.g[SP][0] as u32));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                c.add_s(0, 0, 21);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[SP][0] as u32));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1c7e9c;
            }
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_ResetLayout(self)` (0x1c8a38): put the popup's
/// widgets back to their stored layout positions: widgets at `+0x48..+0x60` and
/// `+0x8c` get scale (1,1,1) and the position at `+0x188..`/`+0x200`; `+0x6c`,
/// `+0x70`, `+0x80`, `+0x88` get only their position; the text widgets at
/// `+0x74`, `+0x7c`, `+0x78` get their position and `SetWidgetAlign(9)` (the
/// generated code passes `+0x74` for all three); the option widgets at
/// `+0x90..` are aligned (10) at half `+0x29c`. Then `ResizeTextWidgets` with
/// the global viewport vec (x = 472.0), the scale at `+0x278` is reset to 1 and
/// `AdjustDrawLayer(self, -self+0x314)` runs.
pub fn c_fe_popup_confirm_reset_layout(c: &mut Cpu) {
    if c.pc != 0x001c8a38 {
        return ee::s03::f_001c8a38__generated(c);
    }
    guest(c, |c| {
        'b_1c8aac: {
            c.cycles += 10;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                break 'b_1c8aac;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[SP][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x18f));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x188));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x190));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                break 'b_1c8aac;
            }
        }
        'b_1c8b00: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                break 'b_1c8b00;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[SP][0] as u32));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x19b));
                c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x194));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x19c));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                break 'b_1c8b00;
            }
        }
        'b_1c8b54: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                break 'b_1c8b54;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(4, (c.g[SP][0] as u32));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x22b));
                c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x224));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x22c));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                break 'b_1c8b54;
            }
        }
        'b_1c8ba8: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                break 'b_1c8ba8;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[SP][0] as u32));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x237));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x230));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x238));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                break 'b_1c8ba8;
            }
        }
        'b_1c8bfc: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                break 'b_1c8bfc;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[SP][0] as u32));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x243));
                c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x23c));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x244));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                break 'b_1c8bfc;
            }
        }
        'b_1c8c50: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                break 'b_1c8c50;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(3, (c.g[SP][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x24f));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x248));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x250));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                break 'b_1c8c50;
            }
        }
        'b_1c8ca4: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                break 'b_1c8ca4;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[SP][0] as u32));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x25b));
                c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x254));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x25c));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                break 'b_1c8ca4;
            }
        }
        'b_1c8cf8: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                break 'b_1c8cf8;
            } else {
                c.cycles += 19;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(4, (c.g[SP][0] as u32));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x57));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
                c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x207));
                c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x200));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x208));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                break 'b_1c8cf8;
            }
        }
        'b_1c8d1c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c8d1c;
            } else {
                c.cycles += 7;
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1a7));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1a0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                break 'b_1c8d1c;
            }
        }
        'b_1c8d40: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c8d40;
            } else {
                c.cycles += 7;
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x1b3));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x1ac));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                break 'b_1c8d40;
            }
        }
        'b_1c8d70: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_1c8d70;
            } else {
                c.cycles += 10;
                c.ldl(5, (c.g[S0][0] as u32).wrapping_add(0x1cb));
                c.ldr(5, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1cc));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                c.mtc1(0, 12);
                c.g[A2][0] = 0x9;
                c.g[RA][0] = sx(0x001c8d70);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                Callee {
                    addr: 0x001c8930,
                    f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                }
                .call(c, 0x001c8d70)?;
                break 'b_1c8d70;
            }
        }
        'b_1c8dac: {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c8dac;
            } else {
                c.cycles += 12;
                c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1bf));
                c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1b8));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[RA][0] = sx(0x001c8da8);
                c.g[A2][0] = 0x9;
                Callee {
                    addr: 0x001c8930,
                    f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                }
                .call(c, 0x001c8da8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                break 'b_1c8dac;
            }
        }
        'b_1c8de4: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                break 'b_1c8de4;
            } else {
                c.cycles += 12;
                c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0x1d7));
                c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x1d0));
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1d8));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[RA][0] = sx(0x001c8de0);
                c.g[A2][0] = 0x9;
                Callee {
                    addr: 0x001c8930,
                    f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                }
                .call(c, 0x001c8de0)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                break 'b_1c8de4;
            }
        }
        'b_1c8e08: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c8e08;
            } else {
                c.cycles += 7;
                c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e4));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c8e08;
            }
        }
        'b_1c8e28: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1c8e28;
            } else {
                c.cycles += 6;
                c.ldl(6, (c.g[S0][0] as u32).wrapping_add(0x1ef));
                c.ldr(6, (c.g[S0][0] as u32).wrapping_add(0x1e8));
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A3][0]);
                break 'b_1c8e28;
            }
        }
        c.cycles += 4;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 20);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        c.g[S2][0] = 0x4;
        'l_1c8e38: loop {
            'b_1c8e54: {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32));
                let t = c.g[A1][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_1c8e54;
                } else {
                    c.cycles += 4;
                    c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x29c));
                    c.g[A2][0] = 0xa;
                    c.g[RA][0] = sx(0x001c8e54);
                    c.mul_s(12, 12, 20);
                    Callee {
                        addr: 0x001c8930,
                        f: ee::s03::cFEPopupConfirm_SetWidgetAlign,
                    }
                    .call(c, 0x001c8e54)?;
                    break 'b_1c8e54;
                }
            }
            c.cycles += 3;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
            let t = (c.g[S2][0] as i64) >= 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001c8e38);
                }
                continue 'l_1c8e38;
            } else {
                c.cycles += 36;
                c.g[V0][0] = 0x500000;
                c.g[AT][0] = 0x43ec0000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[V1][0] as u32));
                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[SP][0] as u32));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001c8e98);
                c.swc1(0, (c.g[SP][0] as u32));
                Callee {
                    addr: 0x001c70f0,
                    f: ee::s03::cFEPopupConfirm_ResizeTextWidgets,
                }
                .call(c, 0x001c8e98)?;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A0][0] = c.g[S0][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x17));
                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x27f));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x278));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x280), c.g[A2][0]);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x314));
                c.g[RA][0] = sx(0x001c8ed4);
                c.g[A1][0] = sx(0u32.wrapping_sub((c.g[A1][0] as u32)));
                Callee {
                    addr: 0x001c9cd0,
                    f: ee::s03::cFEPopupConfirm_AdjustDrawLayer,
                }
                .call(c, 0x001c8ed4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_AdjustDrawLayer(self, delta)` (0x1c9cd0): add `delta`
/// to the 5-bit draw layer (bits 8..12 of the word at `obj+0x14`, wrapping) of
/// each popup widget (`+0x48..+0x68`, `+0x74/+0x7c/+0x78/+0x80/+0x84/+0x8c`),
/// of the three screen objects found by hash name (`screen = self+0x40`) and
/// of the five option widgets at `+0x90`.
pub fn c_fe_popup_confirm_adjust_draw_layer(c: &mut Cpu) {
    if c.pc != 0x001c9cd0 {
        return ee::s03::f_001c9cd0__generated(c);
    }
    guest(c, |c| {
        'b_1c9d18: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A1][0];
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9d18;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9d18;
            }
        }
        'b_1c9d48: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9d48;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9d48;
            }
        }
        'b_1c9d78: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9d78;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9d78;
            }
        }
        'b_1c9da8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9da8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9da8;
            }
        }
        'b_1c9dd8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9dd8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9dd8;
            }
        }
        'b_1c9e08: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e08;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e08;
            }
        }
        'b_1c9e38: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e38;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e38;
            }
        }
        'b_1c9e68: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e68;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e68;
            }
        }
        'b_1c9e98: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9e98;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9e98;
            }
        }
        'b_1c9ec8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9ec8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9ec8;
            }
        }
        'b_1c9ef8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9ef8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9ef8;
            }
        }
        'b_1c9f28: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9f28;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9f28;
            }
        }
        'b_1c9f58: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9f58;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9f58;
            }
        }
        'b_1c9f88: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9f88;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9f88;
            }
        }
        'b_1c9fb8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8c));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1c9fb8;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1c9fb8;
            }
        }
        'b_1ca000: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001c9fc4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c50));
            call_ptr(c, 0x00317670, 0x001c9fc4)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c9fd0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c9fd0)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1ca000;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1ca000;
            }
        }
        'b_1ca048: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001ca00c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c58));
            call_ptr(c, 0x00317670, 0x001ca00c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001ca018);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001ca018)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1ca048;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1ca048;
            }
        }
        'b_1ca090: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001ca054);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15e0));
            call_ptr(c, 0x00317670, 0x001ca054)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001ca060);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001ca060)?;
            c.g[A1][0] = c.g[V0][0];
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0xffffffffffffe0ff;
            if t {
                break 'b_1ca090;
            } else {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = sx((c.g[V1][0] as u32) >> 8);
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1ca090;
            }
        }
        c.cycles += 4;
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        c.g[T0][0] = 0xffffffffffffe0ff;
        c.g[A3][0] = 0x4;
        c.g[A1][0] = c.lw((c.g[A2][0] as u32));
        'l_1ca0a0: loop {
            'b_1ca0d0: {
                c.cycles += 3;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                let t = c.g[A1][0] == 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                if t {
                    break 'b_1ca0d0;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[A0][0] = c.g[A0][0] & 0x1f;
                    c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[V1][0] = c.g[V1][0] & 0x1f;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 8);
                    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    break 'b_1ca0d0;
                }
            }
            c.cycles += 2;
            if (c.g[A3][0] as i64) >= 0 {
                c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                if event_due(c) {
                    deliver_event_here(c, 0x001ca0a0);
                }
                continue 'l_1ca0a0;
            } else {
                c.cycles += 5;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_CheckAndFixTextEntry(self, text, str)` (0x1ca6a0):
/// make a text entry fit its widget: measure `str` and an ellipsis string with
/// the text object's font scale; while the string (minus the ellipsis width and
/// 1.0) is wider than the widget (`text` vtable 0xa0 size), cut one character
/// off a private copy. If anything was cut, append the ellipsis and set the
/// copy as the text, else set the original; free the copy.
///
/// Not in `OVERRIDES`, no test: its callees (`UIText_GetFont`, `Font_MeasureText`,
/// `operator_new_named`, `strcpy`, `strcat`, `UIText_SetString`, `EA_MemFree`)
/// are ported or have no slot, so they run for real in a unit test and walk
/// the allocator (runaway at pc 0x31ef40) on fixture data. Exact otherwise.
#[allow(dead_code)]
pub fn c_fe_popup_confirm_check_and_fix_text_entry(c: &mut Cpu) {
    if c.pc != 0x001ca6a0 {
        return ee::s03::f_001ca6a0__generated(c);
    }
    guest(c, |c| {
        'b_1ca838: {
            c.cycles += 80;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A1][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A2][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
            c.g[S5][0] = 0u64;
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
            c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x57));
            c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[SP][0] as u32));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001ca704);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001ca704)?;
            c.g[RA][0] = sx(0x001ca70c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a04f0, 0x001ca70c)?;
            c.lwc1(2, (c.g[SP][0] as u32));
            c.g[A1][0] = 0x460000;
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = 0x100;
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x30));
            c.g[A3][0] = 0u64;
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x61a0));
            c.mul_s(1, 1, 2);
            c.mul_s(0, 0, 3);
            c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x38));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x328));
            c.g[RA][0] = sx(0x001ca748);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00317e30, 0x001ca748)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001ca754);
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x00416810, 0x001ca754)?;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001ca764);
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            call_ptr(c, 0x004166f4, 0x001ca764)?;
            c.g[V1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ca774);
            c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1bd0));
            call_ptr(c, 0x003a04f0, 0x001ca774)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[A1][0] = c.g[S4][0];
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            c.g[RA][0] = sx(0x001ca790);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00391fb0, 0x001ca790)?;
            c.g[RA][0] = sx(0x001ca798);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x003a04f0, 0x001ca798)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S0][0];
            c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
            c.g[RA][0] = sx(0x001ca7b4);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00391fb0, 0x001ca7b4)?;
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 3);
            c.sub_s(0, 0, 1);
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
            c.sub_s(0, 0, 3);
            c.c_lt_s(0, 2);
            let t = !c.fcc();
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
            if t {
                break 'b_1ca838;
            } else {
                c.cycles += 2;
                c.mov_s(20, 3);
                c.g[S5][0] = 0x1;
                'l_1ca7e8: loop {
                    c.cycles += 20;
                    c.g[A0][0] = c.g[S1][0];
                    c.sb((c.g[V0][0] as u32), 0u64);
                    c.g[RA][0] = sx(0x001ca7f8);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x003a04f0, 0x001ca7f8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
                    c.g[RA][0] = sx(0x001ca814);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00391fb0, 0x001ca814)?;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.sub_s(1, 1, 0);
                    c.sub_s(1, 1, 20);
                    c.c_lt_s(1, 2);
                    let t = c.fcc();
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001ca7e8);
                        }
                        continue 'l_1ca7e8;
                    } else {
                        break 'b_1ca838;
                    }
                }
            }
        }
        'b_1ca868: {
            'b_1ca85c: {
                c.cycles += 2;
                let t = c.g[S5][0] == 0u64;
                c.g[A1][0] = c.g[S4][0];
                if t {
                    break 'b_1ca85c;
                } else {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001ca848);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x004162d0, 0x001ca848)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001ca854);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0c50, 0x001ca854)?;
                    let t = true;

                    if t {
                        break 'b_1ca868;
                    } else {
                        break 'b_1ca85c;
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ca868);
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x003a0c50, 0x001ca868)?;
            break 'b_1ca868;
        }
        'b_1ca87c: {
            c.cycles += 2;
            if c.g[S0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1ca87c;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001ca878);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e98, 0x001ca878)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                break 'b_1ca87c;
            }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's own RAM) and
    /// its magic value; mailbox words follow at `+4`, `+8`, ...
    const OC933_CTRL: u32 = DATA + 0x7f00;
    const OC933_MAGIC: u32 = 0x0933_c0de;

    /// A gated fake of a direct callee: logs the call like `fakes::fake_call`
    /// and, when `$mailbox` (an offset from `OC933_CTRL`, 0 for none) is
    /// given, returns the mailbox word in `v0`.
    macro_rules! oc933_fake {
        ($fake:ident, $prev:ident, $addr:expr, $gen:path, $mailbox:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC933_CTRL) != OC933_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                let mailbox: u32 = $mailbox;
                if mailbox != 0 {
                    let word = c.bus.read32(OC933_CTRL + mailbox);
                    c.g[V0][0] = sx(word);
                }
            }
        };
    }

    oc933_fake!(
        oc933_fake_layout_text_entry,
        OC933_PREV_LTE,
        0x001c_6d30,
        ee::s03::f_001c6d30__generated,
        4
    );
    oc933_fake!(
        oc933_fake_set_text_from_desc,
        OC933_PREV_STFD,
        0x001c_8050,
        ee::s03::f_001c8050__generated,
        0
    );
    oc933_fake!(
        oc933_fake_measure_desc,
        OC933_PREV_MD,
        0x001c_8320,
        ee::s03::f_001c8320__generated,
        0
    );
    oc933_fake!(
        oc933_fake_compute_bg_scale,
        OC933_PREV_CBS,
        0x001c_7620,
        ee::s03::f_001c7620__generated,
        0
    );
    oc933_fake!(
        oc933_fake_position_bg,
        OC933_PREV_PBG,
        0x001c_78c0,
        ee::s03::f_001c78c0__generated,
        0
    );
    oc933_fake!(
        oc933_fake_set_widget_align,
        OC933_PREV_SWA,
        0x001c_8930,
        ee::s03::f_001c8930__generated,
        0
    );
    oc933_fake!(
        oc933_fake_resize_text,
        OC933_PREV_RTW,
        0x001c_70f0,
        ee::s03::f_001c70f0__generated,
        0
    );
    oc933_fake!(
        oc933_fake_adjust_layer,
        OC933_PREV_ADL,
        0x001c_9cd0,
        ee::s03::f_001c9cd0__generated,
        0
    );

    fn oc933_install() {
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C6D30),
            oc933_fake_layout_text_entry,
            &OC933_PREV_LTE,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8050),
            oc933_fake_set_text_from_desc,
            &OC933_PREV_STFD,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8320),
            oc933_fake_measure_desc,
            &OC933_PREV_MD,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C7620),
            oc933_fake_compute_bg_scale,
            &OC933_PREV_CBS,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C78C0),
            oc933_fake_position_bg,
            &OC933_PREV_PBG,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8930),
            oc933_fake_set_widget_align,
            &OC933_PREV_SWA,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C70F0),
            oc933_fake_resize_text,
            &OC933_PREV_RTW,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C9CD0),
            oc933_fake_adjust_layer,
            &OC933_PREV_ADL,
        );
    }

    /// A vtable at `vt` whose slots `offs` hold `{delta, FAKE + 0x10 * k}`.
    fn oc933_vtable(c: &mut Cpu, vt: u32, offs: &[u32], delta: u16) {
        for (k, off) in offs.iter().enumerate() {
            c.bus.write16(vt + off, delta);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * k as u32);
        }
    }

    /// A widget at `at` with its own vtable (slots 0x40 setStatic, 0x48
    /// setVisible, 0xa0 size) and random words in `+0..0x60`.
    fn oc933_widget(c: &mut Cpu, r: &mut Rng, at: u32, delta: u16) {
        fill(c, r, at, 0x18);
        let vt = at + 0x1000;
        c.bus.write32(at + 8, vt);
        oc933_vtable(c, vt, &[0x40, 0x48, 0xa0], delta);
    }

    // -- 0x001c7388 cFEPopupConfirm_LayoutOptions ----------------------

    #[test]
    fn c_fe_popup_confirm_layout_options_matches() {
        // Option counts 0..6 (flag 8 set / cleared, +0x70 shown or hidden,
        // used and unused widgets), absent widgets, wider / narrower
        // measured widths, and both answers of the text-entry layout.
        check(
            ee::s03::f_001c7388__generated,
            c_fe_popup_confirm_layout_options,
            140,
            &|c, seed| {
                oc933_install();
                let mut r = fakes::machine(c, seed, 0x1c7388, 16);
                c.bus.write32(OC933_CTRL, OC933_MAGIC);
                c.bus
                    .write32(OC933_CTRL + 4, if seed % 2 == 0 { 0 } else { 5 });
                let this = DATA + 0x1000;
                let info = DATA + 0x2000;
                let bg = DATA + 0x2400;
                let counts = [0u32, 1, 2, 3, 4, 5, 6];
                let count = counts[(seed as usize / 2) % counts.len()];
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(info);
                c.bus.write32(info, count);
                fill(c, &mut r, info + 4, 0x40);
                fill(c, &mut r, this + 0x260, 4);
                c.bus.write32(this + 0x264, r.f32(20.0));
                c.bus.write32(this + 0x29c, r.f32(100.0));
                c.bus.write32(this + 0x6c, bg);
                c.bus.write32(bg + 0x90, r.next_u32());
                let shown = DATA + 0x2800;
                oc933_widget(c, &mut r, shown, 0);
                c.bus.write32(this + 0x70, shown);
                for i in 0..5u32 {
                    let present = (seed >> (i + 1)) & 3 != 0;
                    let w = DATA + 0x3000 + 0x100 * i;
                    c.bus
                        .write32(this + 0x90 + 4 * i, if present { w } else { 0 });
                    if present {
                        oc933_widget(c, &mut r, w, (4 * (seed % 3)) as u16);
                    }
                }
                // The measured item size the faked `MeasureDesc` leaves on the stack.
                let frame = c.g[SP][0] as u32 - 0xa0;
                c.bus.write32(frame + 0x18, r.f32(100.0));
                fill(c, &mut r, frame, 4);
            },
        );
    }

    // -- 0x001c7c20 cFEPopupConfirm_LayoutFrame ------------------------

    #[test]
    fn c_fe_popup_confirm_layout_frame_matches() {
        // Each of the five widgets present or absent, `+0x2cc` set (x = centred
        // + 320) or clear (centred x added); the background scaling and
        // positioning callees are faked (gated), `GetCenteredX` runs for real.
        check(
            ee::s03::f_001c7c20__generated,
            c_fe_popup_confirm_layout_frame,
            120,
            &|c, seed| {
                oc933_install();
                let mut r = fakes::machine(c, seed, 0x1c7c20, 3);
                c.bus.write32(OC933_CTRL, OC933_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this + 0x1b8, 4);
                for off in [0x260u32, 0x264, 0x268, 0x26c, 0x270] {
                    c.bus.write32(this + off, r.f32(300.0));
                }
                c.bus.write32(this + 0x2cc, seed % 2);
                for (k, off) in [0x6cu32, 0x70, 0x74, 0x7c, 0x78].into_iter().enumerate() {
                    let w = DATA + 0x3000 + 0x80 * k as u32;
                    let present = (seed >> (k + 1)) % 4 != 0;
                    c.bus.write32(this + off, if present { w } else { 0 });
                    fill(c, &mut r, w + 0x44, 3);
                }
            },
        );
    }

    // -- 0x001c8a38 cFEPopupConfirm_ResetLayout ------------------------

    #[test]
    fn c_fe_popup_confirm_reset_layout_matches() {
        // Every widget slot present or absent (the scale / position copies and
        // the `SetWidgetAlign` calls of the three text widgets and of the
        // option widgets); the aligning, resizing and layer callees are faked.
        check(
            ee::s03::f_001c8a38__generated,
            c_fe_popup_confirm_reset_layout,
            140,
            &|c, seed| {
                oc933_install();
                let mut r = fakes::machine(c, seed, 0x1c8a38, 6);
                c.bus.write32(OC933_CTRL, OC933_MAGIC);
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this + 0x180, 0x80);
                c.bus.write32(this + 0x29c, r.f32(80.0));
                c.bus.write32(this + 0x314, r.next_u32());
                let slots = [
                    0x48u32, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x8c, 0x6c, 0x70, 0x74, 0x7c,
                    0x78, 0x80, 0x88, 0x90, 0x94, 0x98, 0x9c, 0xa0,
                ];
                let mask = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b9);
                for (k, off) in slots.into_iter().enumerate() {
                    // Seeds 0 and 1 run with everything present / absent.
                    let present = match seed {
                        0 => true,
                        1 => false,
                        _ => (mask >> k) & 1 != 0,
                    };
                    let w = DATA + 0x3000 + 0x80 * k as u32;
                    c.bus.write32(this + off, if present { w } else { 0 });
                    fill(c, &mut r, w + 0x44, 0x10);
                }
            },
        );
    }

    // -- 0x001c9cd0 cFEPopupConfirm_AdjustDrawLayer --------------------

    /// `miss` hook of the AdjustDrawLayer test: the screen's object finder
    /// (`0x39d860`, call index 1, 3, 5) answers with the mailbox word.
    fn oc933_adjust_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, target);
        if target == 0x0039_d860 {
            let found = c.bus.read32(OC933_CTRL + 0x10 + 4 * n);
            c.g[V0][0] = sx(found);
        }
    }

    #[test]
    fn c_fe_popup_confirm_adjust_draw_layer_matches() {
        // Each widget present or absent, each hash-named screen object found
        // or not, positive and negative layer deltas (the 5-bit layer wraps).
        check(
            ee::s03::f_001c9cd0__generated,
            c_fe_popup_confirm_adjust_draw_layer,
            140,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1c9cd0, 6);
                c.hooks.miss = oc933_adjust_miss;
                let this = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                let deltas = [1u32, 5, 0, 0xffff_ffff, 31, 0xffff_ffe1, 17, 3];
                c.g[A1][0] = sx(deltas[seed as usize % deltas.len()]);
                c.bus.write32(this + 0x40, DATA + 0x2000);
                let slots = [
                    0x48u32, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x74, 0x7c, 0x78,
                    0x80, 0x84, 0x8c, 0x90, 0x94, 0x98, 0x9c, 0xa0,
                ];
                let mask = r.next_u32() ^ seed.wrapping_mul(0x9e37_79b9);
                for (k, off) in slots.into_iter().enumerate() {
                    let present = match seed {
                        0 => true,
                        1 => false,
                        _ => (mask >> k) & 1 != 0,
                    };
                    let w = DATA + 0x3000 + 0x40 * k as u32;
                    c.bus.write32(this + off, if present { w } else { 0 });
                    c.bus.write32(w + 0x14, r.next_u32());
                }
                for k in 0..3u32 {
                    let found = seed == 0 || (seed != 1 && (mask >> (20 + k)) & 1 != 0);
                    let obj = DATA + 0x2800 + 0x40 * k;
                    c.bus.write32(obj + 0x14, r.next_u32());
                    c.bus.write32(
                        OC933_CTRL + 0x10 + 4 * (1 + 2 * k),
                        if found { obj } else { 0 },
                    );
                }
            },
        );
    }
}
