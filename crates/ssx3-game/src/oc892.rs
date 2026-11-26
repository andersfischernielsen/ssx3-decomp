//! Staging module oc892 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc892`.

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
        0x001b_bf18,
        fe_state_create_duplicate_on_widget_action,
        "cFEStateCreateDuplicate_OnWidgetAction",
    ),
    (
        0x001b_c190,
        fe_state_create_duplicate_help_popup,
        "cFEStateCreateDuplicate_v25",
    ),
    (
        0x001b_c300,
        fe_state_create_duplicate_submit_persona_name,
        "cFEStateCreateDuplicate_SubmitPersonaName",
    ),
    (
        0x001b_c400,
        fe_state_create_duplicate_fill_name_suggestions,
        "cFEStateCreateDuplicate_FillNameSuggestions",
    ),
    (
        0x001b_c4f0,
        fe_state_create_duplicate_v31,
        "cFEStateCreateDuplicate_v31",
    ),
    (
        0x001b_c728,
        fe_state_create_duplicate_on_error_popup_event,
        "cFEStateCreateDuplicate_OnErrorPopupEvent",
    ),
    (
        0x001b_c818,
        fe_state_create_duplicate_on_save_account_answer,
        "cFEStateCreateDuplicate_OnSaveAccountAnswer",
    ),
];

/// `cFEStateCreateDuplicate_OnWidgetAction(this, widget, msg)` (0x001bbf18): the duplicate-name
/// screen's widget handler. Msg 5 (activate) opens the keyboard or submits the persona name,
/// 6 opens the account or persona popup, 13 submits the edited name; anything else goes to
/// the base `cFEStateNet_OnWidgetAction`.
pub fn fe_state_create_duplicate_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bc180: {
            'b_1bc174: {
                'b_1bc160: {
                    'b_1bc150: {
                        'b_1bc13c: {
                            'b_1bc050: {
                                c.cycles += 6;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                                c.g[V0][0] = 0x6;
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[S0][0] = c.g[A0][0];
                                if t {
                                    break 'b_1bc050;
                                } else {
                                    'b_1bbf60: {
                                        'b_1bbf4c: {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1bbf4c;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[A2][0] == c.g[V0][0] {
                                                    c.g[A1][0] = c
                                                        .lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                                    break 'b_1bbf60;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1bc174;
                                                    } else {
                                                        break 'b_1bbf4c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0xd;
                                        if c.g[A2][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                                            break 'b_1bc13c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;

                                            if t {
                                                break 'b_1bc174;
                                            } else {
                                                break 'b_1bbf60;
                                            }
                                        }
                                    }
                                    'b_1bc010: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x4;
                                        let t = c.g[A1][0] != c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                                        if t {
                                            break 'b_1bc010;
                                        } else {
                                            'b_1bbfcc: {
                                                'b_1bbfa4: {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1bbfa4;
                                                    } else {
                                                        c.cycles += 12;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4cc8));
                                                        call_ptr(c, 0x00317670, 0x001bbf80)?;
                                                        c.g[A3][0] = 0x4a0000;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        c.g[A3][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x1408));
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[T0][0] = 0u64;
                                                        c.g[T1][0] = 0xffffffffffffffff;
                                                        let t = true;
                                                        c.g[T2][0] = 0x10;
                                                        if t {
                                                            break 'b_1bbfcc;
                                                        } else {
                                                            break 'b_1bbfa4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 10;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x4c78));
                                                call_ptr(c, 0x00317670, 0x001bbfac)?;
                                                c.g[A3][0] = 0x4a0000;
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x1408));
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[T0][0] = 0u64;
                                                c.g[T1][0] = 0xffffffffffffffff;
                                                c.g[T2][0] = 0xc;
                                                break 'b_1bbfcc;
                                            }
                                            c.cycles += 17;
                                            call_ptr(c, 0x001a9150, 0x001bbfd4)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x69c));
                                            c.g[V0][0] = 0x1;
                                            c.g[A1][0] = 0x4b;
                                            c.g[A2][0] = 0x1;
                                            c.sw(
                                                (c.g[V1][0] as u32).wrapping_add(0x58),
                                                c.g[V0][0],
                                            );
                                            c.g[A3][0] = 0x2;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x69c));
                                            call_ptr(c, 0x001ce3c8, 0x001bbff4)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x69c));
                                            c.g[A1][0] = 0x35;
                                            c.g[A2][0] = 0u64;
                                            c.g[A3][0] = 0x2;
                                            call_ptr(c, 0x001ce3c8, 0x001bc008)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            if t {
                                                break 'b_1bc180;
                                            } else {
                                                break 'b_1bc010;
                                            }
                                        }
                                    }
                                    'b_1bc034: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                                            break 'b_1bc034;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                                            c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                                            c.g[A1][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[A1][0] as u32));
                                            call_ptr(c, 0x00318448, 0x001bc02c)?;
                                            let t = true;

                                            if t {
                                                break 'b_1bc150;
                                            } else {
                                                break 'b_1bc034;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                    call_ptr(c, 0x001bc300, 0x001bc048)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    if t {
                                        break 'b_1bc180;
                                    } else {
                                        break 'b_1bc050;
                                    }
                                }
                            }
                            'b_1bc104: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if t {
                                    break 'b_1bc104;
                                } else {
                                    'b_1bc06c: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1bc06c;
                                        } else {
                                            c.cycles += 2;
                                            call_ptr(c, 0x00260f80, 0x001bc06c)?;
                                            break 'b_1bc06c;
                                        }
                                    }
                                    'b_1bc088: {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x00261460, 0x001bc074)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        if t {
                                            break 'b_1bc088;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x002613c8, 0x001bc084)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            break 'b_1bc088;
                                        }
                                    }
                                    'b_1bc098: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1bc098;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025b800, 0x001bc098)?;
                                            break 'b_1bc098;
                                        }
                                    }
                                    c.cycles += 27;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x00261408, 0x001bc0a0)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    call_ptr(c, 0x0025b800, 0x001bc0a8)?;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = 0xb10;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                                    c.g[A2][0] = 0u64;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001bc0c0)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[A3][0] = 0u64;
                                    c.g[T0][0] = 0u64;
                                    c.g[T1][0] = 0u64;
                                    call_ptr(c, 0x001b6ed0, 0x001bc0dc)?;
                                    c.g[V1][0] = 0x832;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f290, 0x001bc0fc)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    if t {
                                        break 'b_1bc180;
                                    } else {
                                        break 'b_1bc104;
                                    }
                                }
                            }
                            c.cycles += 14;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x6d8;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d68));
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001bc11c)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x001ad130, 0x001bc128)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x001a8a40, 0x001bc134)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_1bc180;
                            } else {
                                break 'b_1bc13c;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x69c));
                        if t {
                            break 'b_1bc160;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                            call_ptr(c, 0x003184e8, 0x001bc150)?;
                            break 'b_1bc150;
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001bc290, 0x001bc158)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1bc180;
                    } else {
                        break 'b_1bc160;
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                call_ptr(c, 0x001bc300, 0x001bc16c)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    break 'b_1bc180;
                } else {
                    break 'b_1bc174;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a8918, 0x001bc17c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1bc180;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateDuplicate_v25(this, menu)` (0x001bc190): fills the help popup for the
/// duplicate-name screen (three options, a title and a description line chosen by
/// `mbAccount`).
pub fn fe_state_create_duplicate_help_popup(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bc220: {
            'b_1bc1f4: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x001da528, 0x001bc1b4)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                if c.g[V0][0] == 0u64 {
                    c.g[A0][0] = 0x460000;
                    break 'b_1bc1f4;
                } else {
                    c.cycles += 13;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4ce8));
                    call_ptr(c, 0x00317670, 0x001bc1cc)?;
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4d08));
                    call_ptr(c, 0x00317670, 0x001bc1dc)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x001da530, 0x001bc1ec)?;
                    let t = true;
                    c.g[A0][0] = 0x460000;
                    if t {
                        break 'b_1bc220;
                    } else {
                        break 'b_1bc1f4;
                    }
                }
            }
            c.cycles += 11;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4d30));
            call_ptr(c, 0x00317670, 0x001bc1fc)?;
            c.g[A0][0] = 0x460000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4d50));
            call_ptr(c, 0x00317670, 0x001bc20c)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x001da530, 0x001bc21c)?;
            c.g[A0][0] = 0x460000;
            break 'b_1bc220;
        }
        c.cycles += 28;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a0));
        call_ptr(c, 0x00317670, 0x001bc228)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[A3][0] = 0x3;
        call_ptr(c, 0x001da510, 0x001bc23c)?;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c50));
        call_ptr(c, 0x00317670, 0x001bc248)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x001da510, 0x001bc25c)?;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14e8));
        call_ptr(c, 0x00317670, 0x001bc268)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x2;
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x001da510, 0x001bc27c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateDuplicate_SubmitPersonaName(this, name)` (0x001bc300): if the name is
/// shorter than 3 characters (`strlen`) it shows the short-name popup, else it pops up the
/// submitting popup and resends the persona.
pub fn fe_state_create_duplicate_submit_persona_name(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bc3e8: {
            'b_1bc398: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = c.g[A1][0];
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00416810, 0x001bc324)?;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_1bc398;
                } else {
                    c.cycles += 26;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x001a9c50, 0x001bc338)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15f8));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                    call_ptr(c, 0x00317670, 0x001bc354)?;
                    c.g[V1][0] = 0x3;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x001bc36c)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x001c57c0, 0x001bc37c)?;
                    c.g[V0][0] = 0x814;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x001a9bc0, 0x001bc390)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1bc3e8;
                    } else {
                        break 'b_1bc398;
                    }
                }
            }
            c.cycles += 20;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x001a9c50, 0x001bc3a0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x460000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2de0));
            call_ptr(c, 0x00317670, 0x001bc3b4)?;
            c.g[V1][0] = 0x3;
            c.g[A2][0] = 0x815;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A2][0]);
            call_ptr(c, 0x001a9bc0, 0x001bc3d8)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0025d1b8, 0x001bc3e4)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1bc3e8;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateDuplicate_FillNameSuggestions(this)` (0x001bc400): formats four name
/// suggestions with `sprintf`, looks each one up and sends it to the name list box, then
/// clears the list entries that are not used.
pub fn fe_state_create_duplicate_fill_name_suggestions(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S1][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S3][0] = 0x4a0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S4][0] = 0x460000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[A0][0] = c.g[SP][0];
        'l_1bc430: loop {
            'b_1bc4c4: {
                'b_1bc4c0: {
                    c.cycles += 11;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4cb8));
                    call_ptr(c, 0x00417828, 0x001bc43c)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001bc444)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001bc450)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
                    if t {
                        break 'b_1bc4c0;
                    } else {
                        'b_1bc4a0: {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1408));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            if t {
                                break 'b_1bc4a0;
                            } else {
                                c.cycles += 10;
                                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x14));
                                call_ptr(c, 0x003a0c50, 0x001bc480)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001bc498)?;
                                let t = true;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1bc4c4;
                                } else {
                                    break 'b_1bc4a0;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003a0c50, 0x001bc4a8)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001bc4c0)?;
                        break 'b_1bc4c0;
                    }
                }
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                break 'b_1bc4c4;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 4i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.g[SP][0];
                if event_due(c) {
                    deliver_event_here(c, 0x001bc430);
                }
                continue 'l_1bc430;
            } else {
                c.cycles += 8;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `cFEStateCreateDuplicate_v31(this, event)` (0x001bc4f0): the popup-event handler: a
/// popup result (0xd4, 0xcb..0x102) opens the account or persona follow-up popups, else the
/// event goes on to the child handler.
pub fn fe_state_create_duplicate_v31(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bc688: {
            'b_1bc684: {
                'b_1bc674: {
                    'b_1bc610: {
                        'b_1bc5d8: {
                            'b_1bc5d0: {
                                c.cycles += 8;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                                c.g[V0][0] = 0xd4;
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[S2][0] = c.g[A0][0];
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                if t {
                                    break 'b_1bc5d0;
                                } else {
                                    'b_1bc548: {
                                        'b_1bc52c: {
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[A1][0] as i64) < 213i64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xcb;
                                            if t {
                                                break 'b_1bc52c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == c.g[V0][0];

                                                if t {
                                                    break 'b_1bc548;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1bc674;
                                                    } else {
                                                        break 'b_1bc52c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0xd6;
                                        let t = c.g[A1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x102;
                                        if t {
                                            break 'b_1bc5d8;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x64750000;
                                            if t {
                                                break 'b_1bc610;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1bc674;
                                                } else {
                                                    break 'b_1bc548;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 34;
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x001a9c50, 0x001bc550)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.g[V0][0] = 0x2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4ad0));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                    call_ptr(c, 0x00317670, 0x001bc56c)?;
                                    c.g[V1][0] = 0x3;
                                    c.g[A0][0] = 0x460000;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                    call_ptr(c, 0x00317670, 0x001bc584)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x001c57c0, 0x001bc594)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x00317670, 0x001bc5a0)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x001c57c0, 0x001bc5b0)?;
                                    c.g[V0][0] = 0x81f;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x001a9bc0, 0x001bc5c8)?;
                                    let t = true;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_1bc684;
                                    } else {
                                        break 'b_1bc5d0;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            call_ptr(c, 0x00260f10, 0x001bc5d8)?;
                            break 'b_1bc5d8;
                        }
                        c.cycles += 14;
                        c.g[A1][0] = 0x460000;
                        c.g[A0][0] = 0x6d8;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d68));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x001bc5f0)?;
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x001ad130, 0x001bc5fc)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x001a8a40, 0x001bc608)?;
                        let t = true;
                        c.g[V0][0] = 0xffffffffffffffff;
                        if t {
                            break 'b_1bc684;
                        } else {
                            break 'b_1bc610;
                        }
                    }
                    'b_1bc664: {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] | 0x706c;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        if t {
                            break 'b_1bc664;
                        } else {
                            'b_1bc654: {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6a0));
                                let t = c.g[V0][0] == 0u64;

                                if t {
                                    break 'b_1bc654;
                                } else {
                                    'b_1bc644: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                        c.g[V0][0] = 0x820;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x815;
                                        if t {
                                            break 'b_1bc644;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];

                                            if t {
                                                break 'b_1bc654;
                                            } else {
                                                break 'b_1bc644;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x001bc654)?;
                                    break 'b_1bc654;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x001bc400, 0x001bc65c)?;
                            let t = true;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_1bc684;
                            } else {
                                break 'b_1bc664;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A1][0] = 0x102;
                    call_ptr(c, 0x001a8e40, 0x001bc66c)?;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_1bc684;
                    } else {
                        break 'b_1bc674;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a8e40, 0x001bc67c)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1bc688;
                } else {
                    break 'b_1bc684;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1bc688;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateDuplicate_OnErrorPopupEvent(this, msg)` (0x001bc728): handles the error
/// popup's answer (msg 0xf or 0x14): the child event, then the account memory card or the
/// persona select state.
pub fn fe_state_create_duplicate_on_error_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bc80c: {
            'b_1bc800: {
                'b_1bc74c: {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[V0][0] = 0xf;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'b_1bc74c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x14;
                        let t = c.g[A2][0] != c.g[V0][0];

                        if t {
                            break 'b_1bc800;
                        } else {
                            break 'b_1bc74c;
                        }
                    }
                }
                'b_1bc768: {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x001a97b8, 0x001bc754)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1bc768;
                    } else {
                        c.cycles += 2;
                        call_ptr(c, 0x00260f80, 0x001bc768)?;
                        break 'b_1bc768;
                    }
                }
                'b_1bc784: {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    call_ptr(c, 0x00261460, 0x001bc770)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                    if t {
                        break 'b_1bc784;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x002613c8, 0x001bc780)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        break 'b_1bc784;
                    }
                }
                'b_1bc794: {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1bc794;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025b800, 0x001bc794)?;
                        break 'b_1bc794;
                    }
                }
                c.cycles += 27;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261408, 0x001bc79c)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001bc7a4)?;
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = 0xb10;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x001bc7bc)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[V0][0];
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x001b6ed0, 0x001bc7d8)?;
                c.g[V1][0] = 0x832;
                c.g[A1][0] = c.g[V0][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039f290, 0x001bc7f8)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                if t {
                    break 'b_1bc80c;
                } else {
                    break 'b_1bc800;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a97b8, 0x001bc808)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1bc80c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateDuplicate_OnSaveAccountAnswer(this, answer)` (0x001bc818): answer 0x10 or
/// 0x11..0x14 (other than 0xf and 0x14) shows the save-account popups; the rest goes to the
/// child event.
pub fn fe_state_create_duplicate_on_save_account_answer(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bc97c: {
            'b_1bc970: {
                'b_1bc8d0: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V0][0] = 0x10;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1bc8d0;
                    } else {
                        'b_1bc868: {
                            'b_1bc854: {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0xf;
                                if t {
                                    break 'b_1bc854;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    if t {
                                        break 'b_1bc868;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_1bc970;
                                        } else {
                                            break 'b_1bc854;
                                        }
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x14;
                            let t = c.g[A2][0] == c.g[V0][0];

                            if t {
                                break 'b_1bc8d0;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_1bc970;
                                } else {
                                    break 'b_1bc868;
                                }
                            }
                        }
                        c.cycles += 26;
                        c.g[A2][0] = 0xf;
                        call_ptr(c, 0x001a97b8, 0x001bc870)?;
                        c.g[A1][0] = 0x460000;
                        c.g[A0][0] = 0xb10;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x001bc888)?;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[A2][0] = c.g[S2][0];
                        c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.g[A3][0] = 0x1;
                        c.g[T0][0] = c.lw(c.g[V1][0] as u32);
                        call_ptr(c, 0x001b6ed0, 0x001bc8a8)?;
                        c.g[V1][0] = 0x82c;
                        c.g[A1][0] = c.g[V0][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f290, 0x001bc8c8)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1bc97c;
                        } else {
                            break 'b_1bc8d0;
                        }
                    }
                }
                c.cycles += 40;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a97b8, 0x001bc8d8)?;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x001a9c50, 0x001bc8e0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x460000;
                c.g[V0][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c70));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                call_ptr(c, 0x00317670, 0x001bc8fc)?;
                c.g[V1][0] = 0x3;
                c.g[A0][0] = 0x460000;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                call_ptr(c, 0x00317670, 0x001bc914)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001c57c0, 0x001bc924)?;
                c.g[V0][0] = 0x80e;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                call_ptr(c, 0x001a9bc0, 0x001bc938)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
                call_ptr(c, 0x003184e8, 0x001bc94c)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
                call_ptr(c, 0x003184e8, 0x001bc960)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025c610, 0x001bc968)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1bc97c;
                } else {
                    break 'b_1bc970;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x001a97b8, 0x001bc978)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1bc97c;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    /// The duplicate-name screen fixture: the self object at DATA+0x10000
    /// with its account/persona fields, the world at gp-0xc8 and BuddyMan at
    /// gp+0x238 point into DATA. `mbAccount` (+0x6d0) is set from the seed.
    fn dup_fixture(c: &mut Cpu, r: &mut Rng, entry: u32, seed: u32) -> u32 {
        scramble(c, r);
        elf_image(c);
        let this = DATA + 0x1_0000;
        c.pc = entry;
        c.g[SP][0] = sx(crate::difftest::SP);
        c.g[RA][0] = sx(crate::difftest::RA);
        c.g[GP][0] = sx(GAME_GP);
        c.g[A0][0] = sx(this);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x1_5000);
        c.bus.write32(GAME_GP + 0x238, DATA + 0x1_6000);
        c.bus.write32(this + 0x40, DATA + 0x1_3000);
        c.bus.write32(DATA + 0x1_3000 + 0xd0, DATA + 0x1_3100);
        c.bus.write32(DATA + 0x1_3100 + 0x10, DATA + 0x1_3200);
        c.bus.write32(this + 0x10, DATA + 0x1_4000);
        c.bus.write32(this + 0x69c, DATA + 0x1_1000);
        c.bus.write32(this + 0x6d0, (seed / 2) % 2);
        c.bus.write32(this + 0x6d8, DATA + 0x1_2000);
        c.bus.write32(DATA + 0x1_2000, DATA + 0x1_2100);
        c.bus.write32(DATA + 0x1_2000 + 0x4, DATA + 0x1_2200);
        this
    }

    #[test]
    fn fe_state_create_duplicate_on_widget_action_matches() {
        check(
            ee::s03::f_001bbf18__generated,
            fe_state_create_duplicate_on_widget_action,
            300,
            &|c, seed| {
                let mut r = Rng(0x001b_bf18 ^ seed as u64);
                dup_fixture(c, &mut r, 0x001b_bf18, seed);
                // a1 is the widget: its misc data (+0x18) selects the item.
                c.g[A1][0] = sx(DATA + 0x1_7000);
                c.bus.write32(DATA + 0x1_7000 + 0x18, seed % 6);
                c.g[A2][0] = [5u64, 6, 13, 9][(seed % 4) as usize];
            },
        );
    }

    #[test]
    fn fe_state_create_duplicate_help_popup_matches() {
        check(
            ee::s03::f_001bc190__generated,
            fe_state_create_duplicate_help_popup,
            200,
            &|c, seed| {
                let mut r = Rng(0x001b_c190 ^ seed as u64);
                dup_fixture(c, &mut r, 0x001b_c190, seed);
                c.g[A1][0] = sx(DATA + 0x1_8000);
            },
        );
    }

    #[test]
    fn fe_state_create_duplicate_submit_persona_name_matches() {
        check(
            ee::s03::f_001bc300__generated,
            fe_state_create_duplicate_submit_persona_name,
            200,
            &|c, seed| {
                let mut r = Rng(0x001b_c300 ^ seed as u64);
                let this = dup_fixture(c, &mut r, 0x001b_c300, seed);
                // The name is shorter than 3 characters on some seeds.
                let name = DATA + 0x1_9000;
                for i in 0..(seed % 6) {
                    c.bus.write8(name + i, b'A');
                }
                c.bus.write8(name + seed % 6, 0);
                c.g[A1][0] = sx(name);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn fe_state_create_duplicate_fill_name_suggestions_matches() {
        check(
            ee::s03::f_001bc400__generated,
            fe_state_create_duplicate_fill_name_suggestions,
            200,
            &|c, seed| {
                let mut r = Rng(0x001b_c400 ^ seed as u64);
                dup_fixture(c, &mut r, 0x001b_c400, seed);
                c.bus.write32(DATA + 0x1_2000, DATA + 0x1_2100);
            },
        );
    }

    #[test]
    fn fe_state_create_duplicate_v31_matches() {
        check(
            ee::s03::f_001bc4f0__generated,
            fe_state_create_duplicate_v31,
            300,
            &|c, seed| {
                let mut r = Rng(0x001b_c4f0 ^ seed as u64);
                dup_fixture(c, &mut r, 0x001b_c4f0, seed);
                c.g[A1][0] =
                    [0xd4u64, 0xcb, 0xd6, 0x102, 0x20, 0x81f, 0x80e, 0x82c][(seed % 8) as usize];
            },
        );
    }

    #[test]
    fn fe_state_create_duplicate_on_error_popup_event_matches() {
        check(
            ee::s03::f_001bc728__generated,
            fe_state_create_duplicate_on_error_popup_event,
            200,
            &|c, seed| {
                // The error popup path allocates (cMemMan_alloc, 0x317d70): give it
                // the real pool allocator before the registers are set.
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                let mut r = Rng(0x001b_c728 ^ seed as u64);
                dup_fixture(c, &mut r, 0x001b_c728, seed);
                c.g[A2][0] = [0xfu64, 0x14, 0x3][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn fe_state_create_duplicate_on_save_account_answer_matches() {
        check(
            ee::s03::f_001bc818__generated,
            fe_state_create_duplicate_on_save_account_answer,
            300,
            &|c, seed| {
                let mut r = Rng(0x001b_c818 ^ seed as u64);
                dup_fixture(c, &mut r, 0x001b_c818, seed);
                c.g[A2][0] = [0x10u64, 0x11, 0xf, 0x14, 0x13, 0x3][(seed % 6) as usize];
            },
        );
    }
}
