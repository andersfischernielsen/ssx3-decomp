//! Staging module oc853 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc853`.

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
        0x0019_6540,
        cfe_state_audio_options_on_widget_created,
        "cFEStateAudioOptions_OnWidgetCreated",
    ),
    (
        0x0019_66e0,
        cfe_state_audio_options_on_widget_action,
        "cFEStateAudioOptions_OnWidgetAction",
    ),
    (
        0x0019_6b90,
        cfe_state_request_line_ctor,
        "cFEStateRequestLine_ctor",
    ),
    (
        0x0019_6db0,
        cfe_state_request_line_init,
        "cFEStateRequestLine_Init",
    ),
    (
        0x0019_7500,
        cfe_state_request_line_on_widget_action,
        "cFEStateRequestLine_OnWidgetAction",
    ),
];

/// `void cFEStateAudioOptions_OnWidgetCreated(self, widget)` (0x196540): the
/// audio options screen caches the widgets it finds by hash name (`widget+0x38`):
/// the four check icons (`+0x4c/+0x50/+0x54/+0x58`), the radio buttons
/// (`+0x64/+0x68/+0x6c`) and the three menu entries, and sets each cached
/// icon's text to the checked or the empty box according to the radio mode
/// (`+0x78`). A widget matching none of the names is ignored.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_audio_options_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1966c8: {
            'b_1966c4: {
                'b_196640: {
                    'b_196630: {
                        'b_19662c: {
                            'b_19658c: {
                                c.cycles += 14;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[S1][0] = c.g[A0][0];
                                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[S0][0] = c.g[A1][0];
                                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                                c.g[A0][0] = 0x4a0000;
                                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1830));
                                c.g[RA][0] = sx(0x00196570);
                                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                                call_ptr(c, 0x00317670, 0x00196570)?;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[A0][0] = 0x4a0000;
                                if t {
                                    break 'b_19658c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    let t = c.g[V0][0] != 0u64;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4c), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_19658c;
                                        }
                                    }
                                }
                            }
                            'b_1965b4: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x00196594);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1838));
                                call_ptr(c, 0x00317670, 0x00196594)?;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[V0][0] = 0x4a0000;
                                if t {
                                    break 'b_1965b4;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    c.g[V0][0] = 0x2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_1965b4;
                                        }
                                    }
                                }
                            }
                            'b_1965dc: {
                                c.cycles += 5;
                                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1840));
                                c.g[RA][0] = sx(0x001965c0);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x00317670, 0x001965c0)?;
                                let t = c.g[S2][0] != c.g[V0][0];
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_1965dc;
                                } else {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_1965dc;
                                        }
                                    }
                                }
                            }
                            'b_196604: {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001965e4);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x00317670, 0x001965e4)?;
                                if c.g[S2][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x4a0000;
                                    break 'b_196604;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x50), c.g[S0][0]);
                                    if t {
                                        break 'b_19662c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                        if t {
                                            break 'b_196630;
                                        } else {
                                            break 'b_196604;
                                        }
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0019660c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1848));
                            call_ptr(c, 0x00317670, 0x0019660c)?;
                            if c.g[S2][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_196640;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x58), c.g[S0][0]);
                                if t {
                                    break 'b_19662c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                    if t {
                                        break 'b_196630;
                                    } else {
                                        break 'b_19662c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                        break 'b_196630;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 0xffffffffffffffff;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                    if t {
                        break 'b_1966c4;
                    } else {
                        break 'b_196640;
                    }
                }
                'b_196658: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00196648);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x420));
                    call_ptr(c, 0x00317670, 0x00196648)?;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        break 'b_196658;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1966c4;
                        } else {
                            break 'b_196658;
                        }
                    }
                }
                'b_1966c0: {
                    'b_196670: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196660);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x430));
                        call_ptr(c, 0x00317670, 0x00196660)?;
                        if c.g[S2][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_196670;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1966c0;
                            } else {
                                break 'b_196670;
                            }
                        }
                    }
                    'b_19668c: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196678);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x440));
                        call_ptr(c, 0x00317670, 0x00196678)?;
                        if c.g[S2][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_19668c;
                        } else {
                            c.cycles += 3;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x68), c.g[S0][0]);
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1966c0;
                            } else {
                                break 'b_19668c;
                            }
                        }
                    }
                    'b_1966a8: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196694);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x450));
                        call_ptr(c, 0x00317670, 0x00196694)?;
                        if c.g[S2][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x460000;
                            break 'b_1966a8;
                        } else {
                            c.cycles += 3;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x64), c.g[S0][0]);
                            let t = true;
                            c.g[V0][0] = 0x4;
                            if t {
                                break 'b_1966c0;
                            } else {
                                break 'b_1966a8;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001966b0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x460));
                    call_ptr(c, 0x00317670, 0x001966b0)?;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1966c8;
                    } else {
                        c.cycles += 2;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[S0][0]);
                        c.g[V0][0] = 0x3;
                        break 'b_1966c0;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_1966c4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1966c8;
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

/// `void cFEStateAudioOptions_OnWidgetAction(self, widget, event)` (0x1966e0):
/// the audio options screen's widget actions. Event 5 and 6 (a jump table at
/// `0x460560`) select the radio mode of the activated entry (`widget+0x18`, the
/// radio mode at `self+0x78`), re-draw the checked/empty boxes and notify the
/// screen's widgets through their vtables; event 1 opens a sub-screen. A resume
/// (`pc != entry`) goes to the generated twin.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_audio_options_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001966e0 {
        ee::s02::f_001966e0__entries(c);
        return;
    }
    guest(c, |c| 'l_1966e0: loop {
        'b_196ae8: {
            'b_196ae4: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S2][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_196ae4;
                } else {
                    'b_19692c: {
                        'b_19691c: {
                            'b_19688c: {
                                'b_196740: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                    if t {
                                        break 'b_196740;
                                    } else {
                                        'b_19672c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_19672c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_19692c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_196ae8;
                                                    } else {
                                                        break 'b_19672c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];

                                        if t {
                                            break 'b_19688c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_196ae8;
                                            } else {
                                                break 'b_196740;
                                            }
                                        }
                                    }
                                }
                                'b_1967f4: {
                                    'b_196770: {
                                        c.cycles += 3;
                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_196770;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_196770;
                                            } else {
                                                'b_196764: {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_196764;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[V0][0];
                                                        c.g[V0][0] = 0x4;
                                                        if t {
                                                            break 'b_1967f4;
                                                        } else {
                                                            break 'b_196764;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                                                if c.g[V0][0] == 0u64 {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    break 'b_196ae8;
                                                } else {
                                                    break 'b_196770;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 33;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                                    c.g[A2][0] = 0xffffffffffffffff;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[A2][0]);
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[A1][0]);
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[A2][0]);
                                    c.g[RA][0] = sx(0x001967b8);
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[A1][0]);
                                    call_ptr(c, 0x0014dc80, 0x001967b8)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001967c4);
                                    c.g[A1][0] = 0xd;
                                    call_ptr(c, 0x0014dd58, 0x001967c4)?;
                                    c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                                    c.g[RA][0] = sx(0x001967d8);
                                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                    call_ptr(c, 0x001587f8, 0x001967d8)?;
                                    c.g[RA][0] = sx(0x001967e0);
                                    call_ptr(c, 0x0028b180, 0x001967e0)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                    c.g[RA][0] = sx(0x001967ec);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0028bf78, 0x001967ec)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_196ae8;
                                    } else {
                                        break 'b_1967f4;
                                    }
                                }
                                c.cycles += 2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_196ae8;
                                } else {
                                    'b_196814: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A1][0] = 0x460000;
                                        if t {
                                            break 'b_196814;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                                            if c.g[V0][0] == 0u64 {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                break 'b_196ae8;
                                            } else {
                                                break 'b_196814;
                                            }
                                        }
                                    }
                                    'b_196880: {
                                        c.cycles += 15;
                                        c.g[A0][0] = 0x210;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x00196828);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x00196828)?;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                        c.g[RA][0] = sx(0x0019683c);
                                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                        call_ptr(c, 0x00196b90, 0x0019683c)?;
                                        c.g[T0][0] = c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V1][0] = 0xffffffffffffc0ff;
                                        if t {
                                            break 'b_196880;
                                        } else {
                                            c.cycles += 12;
                                            c.g[V0][0] =
                                                c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                            c.g[A1][0] = c.g[T0][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                            c.g[A3][0] = 0u64;
                                            c.g[V0][0] = c.g[V0][0] | 0x100;
                                            c.sw(
                                                (c.g[T0][0] as u32).wrapping_add(0x1c),
                                                c.g[V0][0],
                                            );
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[RA][0] = sx(0x00196878);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f320, 0x00196878)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_196ae8;
                                            } else {
                                                break 'b_196880;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    let t = true;
                                    c.g[A1][0] = c.g[T0][0];
                                    if t {
                                        break 'b_19691c;
                                    } else {
                                        break 'b_19688c;
                                    }
                                }
                            }
                            'b_1968ec: {
                                c.cycles += 19;
                                c.g[RA][0] = sx(0x00196894);
                                call_ptr(c, 0x0014dc80, 0x00196894)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001968a0);
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x001968a0)?;
                                c.g[A1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
                                c.g[RA][0] = sx(0x001968b4);
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, 0x001587f8, 0x001968b4)?;
                                c.g[RA][0] = sx(0x001968bc);
                                call_ptr(c, 0x0028b180, 0x001968bc)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                                c.g[RA][0] = sx(0x001968c8);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0028bf78, 0x001968c8)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_1968ec;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001968e4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x001968e4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_196ae8;
                                    } else {
                                        break 'b_1968ec;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00196910);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196910)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[V0][0];
                            if t {
                                break 'b_196ae4;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                break 'b_19691c;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x00196924);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f400, 0x00196924)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_196ae8;
                        } else {
                            break 'b_19692c;
                        }
                    }
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x00196934);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    call_ptr(c, 0x00317670, 0x00196934)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00196940);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00196940)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[S4][0] = 0u64;
                    if t {
                        break 'b_196ae4;
                    } else {
                        'b_196a88: {
                            'b_196a84: {
                                c.cycles += 5;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                c.g[S2][0] = 0u64;
                                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[S3][0] = 0u64;
                                if t {
                                    break 'b_196a84;
                                } else {
                                    'b_196a70: {
                                        'b_196a54: {
                                            'b_196a4c: {
                                                'b_196a1c: {
                                                    'b_1969d8: {
                                                        'b_196994: {
                                                            'b_196988: {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = 0x460000;
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x560));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x001966e0 => {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x001966e0,
                                                                            );
                                                                        }
                                                                        continue 'l_1966e0;
                                                                    }
                                                                    0x0019697c => {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x498,
                                                                                ));
                                                                        if t {
                                                                            break 'b_196a54;
                                                                        } else {
                                                                            break 'b_196988;
                                                                        }
                                                                    }
                                                                    0x00196988 => {
                                                                        break 'b_196988;
                                                                    }
                                                                    0x00196994 => {
                                                                        break 'b_196994;
                                                                    }
                                                                    0x001969d8 => {
                                                                        break 'b_1969d8;
                                                                    }
                                                                    0x00196a1c => {
                                                                        break 'b_196a1c;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x001966e0,
                                                                            0x00196b04,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4b0));
                                                            if t {
                                                                break 'b_196a54;
                                                            } else {
                                                                break 'b_196994;
                                                            }
                                                        }
                                                        'b_1969d0: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x7c));
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_1969d0;
                                                            } else {
                                                                'b_1969bc: {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x84));
                                                                    let t = c.g[V1][0] == 0u64;
                                                                    c.g[S3][0] =
                                                                        (0u64 < c.g[V1][0]) as u64;
                                                                    if t {
                                                                        break 'b_1969bc;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x4c0,
                                                                                ));
                                                                        if t {
                                                                            break 'b_196a54;
                                                                        } else {
                                                                            break 'b_1969bc;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x74));
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[A0][0] = 0x460000;
                                                                if t {
                                                                    break 'b_196a4c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4e0));
                                                                    if t {
                                                                        break 'b_196a54;
                                                                    } else {
                                                                        break 'b_1969d0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x510));
                                                        if t {
                                                            break 'b_196a54;
                                                        } else {
                                                            break 'b_1969d8;
                                                        }
                                                    }
                                                    'b_196a14: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x7c));
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A0][0] = 0x460000;
                                                        if t {
                                                            break 'b_196a14;
                                                        } else {
                                                            'b_196a00: {
                                                                c.cycles += 4;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                let t = c.g[V1][0] == 0u64;
                                                                c.g[S4][0] =
                                                                    (0u64 < c.g[V1][0]) as u64;
                                                                if t {
                                                                    break 'b_196a00;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x460000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4c0));
                                                                    if t {
                                                                        break 'b_196a54;
                                                                    } else {
                                                                        break 'b_196a00;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x74));
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_196a4c;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4e0));
                                                                if t {
                                                                    break 'b_196a54;
                                                                } else {
                                                                    break 'b_196a14;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x528));
                                                    if t {
                                                        break 'b_196a54;
                                                    } else {
                                                        break 'b_196a1c;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_196a70;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_196a70;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        let t = c.g[V1][0] == 0u64;
                                                        c.g[S2][0] = (0u64 < c.g[V1][0]) as u64;
                                                        if t {
                                                            break 'b_196a4c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4c0));
                                                            if t {
                                                                break 'b_196a54;
                                                            } else {
                                                                break 'b_196a4c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x4f8));
                                            break 'b_196a54;
                                        }
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00196a5c);
                                        call_ptr(c, 0x00317670, 0x00196a5c)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x00196a68);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x00196a68)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_196a88;
                                        } else {
                                            break 'b_196a70;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[RA][0] = sx(0x00196a78);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x540));
                                    call_ptr(c, 0x00317670, 0x00196a78)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00196a84);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x00196a84)?;
                                    break 'b_196a84;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            break 'b_196a88;
                        }
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        let t = c.g[V1][0] == 0u64;
                        c.g[A1][0] = c.g[S2][0];
                        if t {
                            break 'b_196ae4;
                        } else {
                            c.cycles += 20;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196aac);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196aac)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                            c.g[A1][0] = c.g[S3][0];
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196ac8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196ac8)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                            c.g[A1][0] = c.g[S4][0];
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00196ae4);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x00196ae4)?;
                            break 'b_196ae4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_196ae8;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

/// `cFEStateRequestLine *cFEStateRequestLine_ctor(self, int a1, int idx, int inStore)`
/// (0x196b90): the UIState base constructor, then the request-line state's
/// fields: player index (`+0x44`), the player's control mask (`+0x15`), the
/// 0x100-byte buffer cleared, the song lists and counters zeroed, and the
/// three widgets found by hash name (`+0x204/+0x208/+0x20c`).
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_request_line_ctor(c: &mut Cpu) {
    guest(c, |c| {
        'b_196bf0: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 24);
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00196bb8);
            c.g[S2][0] = c.g[A3][0];
            call_ptr(c, 0x0039e2a0, 0x00196bb8)?;
            c.g[V0][0] = 0x11;
            c.g[V1][0] = 0x470000;
            c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c88));
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sb((c.g[S1][0] as u32).wrapping_add(0x44), c.g[S0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
                break 'b_196bf0;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00196be8);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x001a1cd0, 0x00196be8)?;
                c.sb((c.g[S1][0] as u32).wrapping_add(0x15), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
                break 'b_196bf0;
            }
        }
        c.cycles += 10;
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00196bfc);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x003e6448, 0x00196bfc)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x14c), 0u64);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1bc));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x150), 0u64);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x194));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x170), 0u64);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x174));
        c.g[A0][0] = 0x7;
        'l_196c18: loop {
            c.cycles += 7;
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            let t = (c.g[A0][0] as i64) >= 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00196c18);
                }
                continue 'l_196c18;
            } else {
                c.cycles += 5;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1b4), 0u64);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1b8), 0u64);
                c.g[V1][0] = 0x5;
                'l_196c48: loop {
                    c.cycles += 7;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00196c48);
                        }
                        continue 'l_196c48;
                    } else {
                        c.cycles += 82;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x16c), c.g[S2][0]);
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25b8));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1dc), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d8), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e4), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1e8), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1ec), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1f0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1f4), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1f8), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1fc), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x200), 0u64);
                        c.sd((c.g[S1][0] as u32).wrapping_add(0x158), 0u64);
                        c.g[RA][0] = sx(0x00196cac);
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        call_ptr(c, 0x00317670, 0x00196cac)?;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[RA][0] = sx(0x00196cb8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00398380, 0x00196cb8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25c8));
                        c.g[RA][0] = sx(0x00196ccc);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x204), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00196ccc)?;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[RA][0] = sx(0x00196cd8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00398380, 0x00196cd8)?;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x25a8));
                        c.g[RA][0] = sx(0x00196cec);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x208), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x00196cec)?;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                        c.g[RA][0] = sx(0x00196cf8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x00398380, 0x00196cf8)?;
                        c.g[RA][0] = sx(0x00196d00);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x20c), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x00196d00)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d0c);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x00196d0c)?;
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[RA][0] = sx(0x00196d18);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x00147488, 0x00196d18)?;
                        c.g[RA][0] = sx(0x00196d20);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x00196d20)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d2c);
                        c.g[A1][0] = 0xb;
                        call_ptr(c, 0x0014dd58, 0x00196d2c)?;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d3c);
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        call_ptr(c, 0x00150928, 0x00196d3c)?;
                        c.g[RA][0] = sx(0x00196d44);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x168), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x00196d44)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00196d50);
                        c.g[A1][0] = 0xd;
                        call_ptr(c, 0x0014dd58, 0x00196d50)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x00196d64);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00158728, 0x00196d64)?;
                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                        c.g[RA][0] = sx(0x00196d78);
                        c.sd((c.g[S1][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                        call_ptr(c, 0x00158750, 0x00196d78)?;
                        c.g[RA][0] = sx(0x00196d80);
                        c.sd((c.g[S1][0] as u32).wrapping_add(0x158), c.g[V0][0]);
                        call_ptr(c, 0x0028b180, 0x00196d80)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00196d90);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
                        call_ptr(c, 0x00197ad8, 0x00196d90)?;
                        c.g[V0][0] = c.g[S1][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
    })
}

/// `cFEStateRequestLine_Init` (0x196db0): the request-line state's screen
/// setup: the economy, reward and audio interfaces are queried for the
/// player's money, owned song list and play list, then the play list is
/// generated and the list/now-playing/help text refreshed.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_request_line_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_196e28: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = 0x4a0000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
            let t = c.g[V0][0] == 0u64;
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18b0));
            if t {
                break 'b_196e28;
            } else {
                c.cycles += 13;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = 0x460000;
                c.g[A0][0] = 0x4a0000;
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x578));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                c.g[RA][0] = sx(0x00196df8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a8));
                call_ptr(c, 0x00317670, 0x00196df8)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00196e04);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039f9d8, 0x00196e04)?;
                c.g[A3][0] = c.g[V0][0];
                let t = c.g[A3][0] == 0u64;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_196e28;
                } else {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x00196e28);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x00196e28)?;
                    break 'b_196e28;
                }
            }
        }
        'b_196e5c: {
            c.cycles += 10;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x00196e34);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x00196e34)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00196e48);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00196e48)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_196e5c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00196e5c);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x00196e5c)?;
                break 'b_196e5c;
            }
        }
        c.cycles += 9;
        c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x14c));
        c.g[RA][0] = sx(0x00196e68);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x170));
        call_ptr(c, 0x0039b760, 0x00196e68)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    })
}

/// `int cFEStateRequestLine_TransitionIn(self, int first_time)` (0x196f50):
/// on the first transition in, refreshes the scroll arrows, the button text
/// and the help text; returns 0.
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
#[cfg(test)] // not installed: its test runs away in unbounded real callees
#[allow(dead_code)]
pub fn cfe_state_request_line_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_196f84: {
            c.cycles += 5;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            let t = c.g[A1][0] == 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_196f84;
            } else {
                c.cycles += 8;
                c.g[RA][0] = sx(0x00196f6c);
                call_ptr(c, 0x00197db8, 0x00196f6c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00196f78);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00198340, 0x00196f78)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00196f84);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001981f8, 0x00196f84)?;
                break 'b_196f84;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateRequestLine_OnWidgetAction(self, widget, event)` (0x197500):
/// the request-line screen's widget actions (event 6: a jump through the
/// widget's vtable, scrolling the song list, buying/playing songs).
#[allow(unreachable_code, unused_assignments, unused_mut, unused_variables)]
pub fn cfe_state_request_line_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1977c4: {
            'b_1977c0: {
                'b_1977bc: {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[A1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1977bc;
                    } else {
                        'b_197780: {
                            'b_19768c: {
                                'b_197550: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[A2][0] == c.g[V0][0];
                                    c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                    if t {
                                        break 'b_197550;
                                    } else {
                                        'b_19753c: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_19753c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_197780;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_1977c4;
                                                    } else {
                                                        break 'b_19753c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_19768c;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            if t {
                                                break 'b_1977c4;
                                            } else {
                                                break 'b_197550;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14c));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                if t {
                                    break 'b_1977bc;
                                } else {
                                    'b_19767c: {
                                        'b_197678: {
                                            'b_1975fc: {
                                                c.cycles += 9;
                                                c.g[V0][0] =
                                                    c.ld((c.g[S1][0] as u32).wrapping_add(0x160));
                                                c.g[V1][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                                c.g[V0][0] = c.g[V0][0] >> (c.g[S0][0] & 63);
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                c.g[V0][0] = c.g[V0][0] << 32;
                                                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[V1][0] =
                                                        c.ld(
                                                            (c.g[S1][0] as u32).wrapping_add(0x158)
                                                        );
                                                    break 'b_1975fc;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x16c)
                                                        );
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_197678;
                                                    } else {
                                                        'b_1975e8: {
                                                            c.cycles += 6;
                                                            c.g[RA][0] = sx(0x001975a4);
                                                            c.g[A1][0] = c.g[S0][0];
                                                            call_ptr(c, 0x00198ae8, 0x001975a4)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x168));
                                                            c.g[V1][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[V1][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_1975e8;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A2][0] = c
                                                                    .ld((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x160));
                                                                c.g[A0][0] = 0u64;
                                                                'l_1975c0: loop {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = c.g[A2][0]
                                                                        >> (c.g[A0][0] & 63);
                                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1));
                                                                    c.g[V1][0] = ((c.g[A0][0]
                                                                        as i64)
                                                                        < 64i64)
                                                                        as u64;
                                                                    let t = c.g[V1][0] != 0u64;
                                                                    c.g[A1][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A1][0] as u32,
                                                                        ));
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x001975c0,
                                                                            );
                                                                        }
                                                                        continue 'l_1975c0;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] = ((c.g[A1][0]
                                                                            as i64)
                                                                            < 6i64)
                                                                            as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        if t {
                                                                            break 'b_19767c;
                                                                        } else {
                                                                            break 'b_1975e8;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x001975f4);
                                                        c.g[A1][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00198988, 0x001975f4)?;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            break 'b_19767c;
                                                        } else {
                                                            break 'b_1975fc;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_197638: {
                                                c.cycles += 5;
                                                c.g[V0][0] = (c.g[V1][0] < 0x1u64) as u64;
                                                c.g[V0][0] = c.g[V0][0] << 32;
                                                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] = c.g[V1][0] >> (c.g[S0][0] & 63);
                                                if t {
                                                    break 'b_197638;
                                                } else {
                                                    c.cycles += 9;
                                                    c.g[V0][0] = 0x1;
                                                    c.g[V0][0] = c.g[V0][0] << (c.g[S0][0] & 63);
                                                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                    c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                                                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                                    c.g[V0][0] = c.g[V0][0] << 32;
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    if t {
                                                        break 'b_19767c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] =
                                                            c.g[V1][0] >> (c.g[S0][0] & 63);
                                                        break 'b_197638;
                                                    }
                                                }
                                            }
                                            'b_197664: {
                                                'b_19765c: {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[V0][0] = c.g[V0][0] << 32;
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_19765c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] =
                                                            c.g[V0][0] << (c.g[S0][0] & 63);
                                                        c.g[V0][0] = !(0u64 | c.g[V0][0]);
                                                        let t = true;
                                                        c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
                                                        if t {
                                                            break 'b_197664;
                                                        } else {
                                                            break 'b_19765c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[V0][0] = c.g[V0][0] << (c.g[S0][0] & 63);
                                                c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
                                                break 'b_197664;
                                            }
                                            c.cycles += 5;
                                            c.sd(
                                                (c.g[S1][0] as u32).wrapping_add(0x158),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x00197670);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x00197ca0, 0x00197670)?;
                                            c.g[RA][0] = sx(0x00197678);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x00198118, 0x00197678)?;
                                            break 'b_197678;
                                        }
                                        c.cycles += 1;
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_19767c;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00197684);
                                    c.g[A1][0] = c.g[S0][0];
                                    call_ptr(c, 0x00198340, 0x00197684)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1977c0;
                                    } else {
                                        break 'b_19768c;
                                    }
                                }
                            }
                            'b_197714: {
                                c.cycles += 24;
                                c.g[RA][0] = sx(0x00197694);
                                call_ptr(c, 0x0014dc80, 0x00197694)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001976a0);
                                c.g[A1][0] = 0xd;
                                call_ptr(c, 0x0014dd58, 0x001976a0)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001976b8);
                                c.g[A3][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x158));
                                call_ptr(c, 0x00158848, 0x001976b8)?;
                                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                c.g[RA][0] = sx(0x001976cc);
                                c.g[A3][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x160));
                                call_ptr(c, 0x00158820, 0x001976cc)?;
                                c.g[RA][0] = sx(0x001976d4);
                                call_ptr(c, 0x0028b180, 0x001976d4)?;
                                c.g[A1][0] = c.ld((c.g[S1][0] as u32).wrapping_add(0x158));
                                c.g[RA][0] = sx(0x001976e0);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0028c2d0, 0x001976e0)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_197714;
                                } else {
                                    c.cycles += 10;
                                    c.g[RA][0] = sx(0x001976f4);
                                    call_ptr(c, 0x0028b180, 0x001976f4)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5558));
                                    c.g[A1][0] = 0xa;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[RA][0] = sx(0x00197710);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00197710)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    break 'b_197714;
                                }
                            }
                            'b_197734: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_197734;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x0019772c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x0019772c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1977c0;
                                    } else {
                                        break 'b_197734;
                                    }
                                }
                            }
                            c.cycles += 13;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x80;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x650));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x0019774c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x0019774c)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                            c.g[RA][0] = sx(0x00197760);
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x16c));
                            call_ptr(c, 0x00195ff0, 0x00197760)?;
                            let t = c.g[V0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1977c0;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00197778);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x00197778)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1977c0;
                                } else {
                                    break 'b_197780;
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        c.g[RA][0] = sx(0x001977a4);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x001986d0, 0x001977a4)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001977b0);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x001981f8, 0x001977b0)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001977bc);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00198340, 0x001977bc)?;
                        break 'b_1977bc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1977c0;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1977c4;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;
    use std::sync::OnceLock;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const OC853_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC853_MAGIC: u32 = 0x4f43_3835;
    /// The state object (`self`).
    pub(crate) const OC853_SELF: u32 = DATA + 0x21_0000;
    /// The widget handed to the action/created handlers.
    pub(crate) const OC853_WIDGET: u32 = DATA + 0x22_0000;
    /// A scattered pool the object pointers point into (0x4000 bytes).
    pub(crate) const OC853_POOL: u32 = DATA + 0x23_0000;
    /// The world object at `gp-0x848`.
    pub(crate) const OC853_WORLD: u32 = DATA + 0x24_0000;

    /// The forced-result word of the fake for `addr` (1 = return 0).
    pub(crate) fn oc853_mail(addr: u32) -> u32 {
        OC853_CTRL + 0x100 + (addr & 0xffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns 0 if
    /// its mailbox asks for it.
    pub(crate) fn oc853_fake_call(c: &mut Cpu, addr: u32) {
        let mail = c.bus.read32(oc853_mail(addr));
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && mail == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc853_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC853_CTRL) != OC853_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc853_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc853_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc853_fakes!(
        (
            oc853_fake_14dc80,
            OC853_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc853_fake_14dd58,
            OC853_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc853_fake_1587f8,
            OC853_PREV_1587F8,
            ee::s01::OVERRIDE_001587F8,
            ee::s01::f_001587f8__generated,
            0x001587f8
        ),
        (
            oc853_fake_147488,
            OC853_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x00147488
        ),
        (
            oc853_fake_150928,
            OC853_PREV_150928,
            ee::s01::OVERRIDE_00150928,
            ee::s01::f_00150928__generated,
            0x00150928
        ),
        (
            oc853_fake_158728,
            OC853_PREV_158728,
            ee::s01::OVERRIDE_00158728,
            ee::s01::f_00158728__generated,
            0x00158728
        ),
        (
            oc853_fake_158750,
            OC853_PREV_158750,
            ee::s01::OVERRIDE_00158750,
            ee::s01::f_00158750__generated,
            0x00158750
        ),
        (
            oc853_fake_158848,
            OC853_PREV_158848,
            ee::s01::OVERRIDE_00158848,
            ee::s01::f_00158848__generated,
            0x00158848
        ),
        (
            oc853_fake_158820,
            OC853_PREV_158820,
            ee::s01::OVERRIDE_00158820,
            ee::s01::f_00158820__generated,
            0x00158820
        ),
        (
            oc853_fake_28b180,
            OC853_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc853_fake_28bf78,
            OC853_PREV_28BF78,
            ee::s07::OVERRIDE_0028BF78,
            ee::s07::f_0028bf78__generated,
            0x0028bf78
        ),
        (
            oc853_fake_28c2d0,
            OC853_PREV_28C2D0,
            ee::s07::OVERRIDE_0028C2D0,
            ee::s07::f_0028c2d0__generated,
            0x0028c2d0
        ),
        (
            oc853_fake_317d70,
            OC853_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc853_fake_39f320,
            OC853_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc853_fake_39f190,
            OC853_PREV_39F190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039f190
        ),
        (
            oc853_fake_39f400,
            OC853_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039f400
        ),
        (
            oc853_fake_39d860,
            OC853_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc853_fake_3a1000,
            OC853_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc853_fake_39e2a0,
            OC853_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039e2a0
        ),
        (
            oc853_fake_398380,
            OC853_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x00398380
        ),
        (
            oc853_fake_39f9d8,
            OC853_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039f9d8
        ),
        (
            oc853_fake_397cc0,
            OC853_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc853_fake_39c870,
            OC853_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc853_fake_39b760,
            OC853_PREV_39B760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039b760
        ),
        (
            oc853_fake_1a1cd0,
            OC853_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a1cd0
        ),
        (
            oc853_fake_198988,
            OC853_PREV_198988,
            ee::s03::OVERRIDE_00198988,
            ee::s03::f_00198988__generated,
            0x00198988
        ),
        (
            oc853_fake_3e6448,
            OC853_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e6448
        ),
        (
            oc853_fake_197ad8,
            OC853_PREV_197AD8,
            ee::s02::OVERRIDE_00197AD8,
            ee::s02::f_00197ad8__generated,
            0x00197ad8
        ),
        (
            oc853_fake_198118,
            OC853_PREV_198118,
            ee::s02::OVERRIDE_00198118,
            ee::s02::f_00198118__generated,
            0x00198118
        ),
    );

    /// Words that are 0, a small 16-aligned count/address, or a pointer into the
    /// pool (so real callees reading the object find shallow data and virtual
    /// calls land in the fakes).
    pub(crate) fn oc853_scatter(c: &mut Cpu, r: &mut Rng, base: u32, words: u32) {
        for i in 0..words {
            let v = match r.next_u32() % 5 {
                0 | 1 => 0,
                2 => (r.next_u32() % 8) * 16,
                _ => OC853_POOL + (r.next_u32() & 0x3ff0),
            };
            c.bus.write32(base + 4 * i, v);
        }
    }

    pub(crate) fn oc853_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The machine for one seed: fakes hooked and gated on, the pool, `self`
    /// and the widget scattered, the world at `gp-0x848`.
    pub(crate) fn oc853_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc853_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        oc853_w(c, OC853_CTRL, OC853_MAGIC);
        oc853_scatter(c, &mut r, OC853_POOL, 0x1000);
        oc853_scatter(c, &mut r, OC853_SELF, 0x100);
        oc853_scatter(c, &mut r, OC853_WIDGET, 0x40);
        oc853_w(c, GAME_GP.wrapping_sub(0x848), OC853_WORLD);
        c.g[A0][0] = sx(OC853_SELF);
        c.g[A1][0] = sx(OC853_WIDGET);
        r
    }

    /// `StrHash` of the strings `cFEStateAudioOptions_OnWidgetCreated` compares
    /// the widget's hash name with, in the order it tests them.
    pub(crate) fn oc853_hashes() -> &'static [u32; 9] {
        static H: OnceLock<[u32; 9]> = OnceLock::new();
        H.get_or_init(|| {
            let ids = [
                0x004a_1830u32,
                0x004a_1838,
                0x004a_1840,
                0x004a_1848,
                0x0046_0420,
                0x0046_0430,
                0x0046_0440,
                0x0046_0450,
                0x0046_0460,
            ];
            ids.map(|a| {
                let mut c = crate::difftest::fresh();
                c.g[A0][0] = a as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0031_7670;
                ee::s10::f_00317670(&mut c);
                c.g[V0][0] as u32
            })
        })
    }

    /// Whether the widget matches each id string in turn, a non-matching hash
    /// and the zero hash; the radio mode (`self+0x78`) takes 0..=4.
    #[test]
    fn cfe_state_audio_options_on_widget_created_matches() {
        check(
            ee::s02::f_00196540__generated,
            cfe_state_audio_options_on_widget_created,
            220,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_6540, 0);
                c.pc = 0x0019_6540;
                let pick = (seed % 11) as usize;
                let hash = match pick {
                    0..=8 => oc853_hashes()[pick],
                    9 => r.next_u32() | 1,
                    _ => 0,
                };
                oc853_w(c, OC853_WIDGET + 0x38, hash);
                oc853_w(c, OC853_SELF + 0x78, (seed / 11) % 5);
            },
        );
    }

    /// The chain `cFEStateAudioOptions_OnWidgetAction` follows for its virtual
    /// call: `self+0x10 -> obj -> vtable (+4) -> +0x28 (this delta), +0x2c`.
    /// The slot holds a real leaf returning 0, a fake slot or null.
    pub(crate) fn oc853_chain(c: &mut Cpu, seed: u32) {
        let mgr = OC853_POOL + 0x100;
        let obj = OC853_POOL + 0x200;
        let vt = OC853_POOL + 0x300;
        oc853_w(c, OC853_SELF + 0x10, mgr);
        oc853_w(c, mgr, obj);
        oc853_w(c, obj + 4, vt);
        c.bus.write16(vt + 0x28, 0);
        let target = [0x0014_06c0u32, fakes::FAKE, 0, 0x10][((seed / 7) % 4) as usize];
        oc853_w(c, vt + 0x2c, target);
    }

    /// Events 5 and 6 (the jump table by `widget+0x18`, 0..=7) and the
    /// others, a null widget, the world flag at `+0x84`, the sub-state fields.
    #[test]
    fn cfe_state_audio_options_on_widget_action_matches() {
        check(
            ee::s02::f_001966e0__generated,
            cfe_state_audio_options_on_widget_action,
            400,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_66e0, 14);
                c.pc = 0x0019_66e0;
                oc853_chain(c, seed);
                c.g[A1][0] = if seed % 9 == 0 { 0 } else { sx(OC853_WIDGET) };
                c.g[A2][0] = [5u64, 6, 1, 0, 2, 9, 5, 6, 1, 3][(seed % 10) as usize];
                oc853_w(c, OC853_WIDGET + 0x18, r.next_u32() % 8);
                oc853_w(c, OC853_SELF + 0x40, OC853_POOL + 0x400);
                oc853_w(c, OC853_SELF + 0x70, r.next_u32() % 4);
                oc853_w(c, OC853_SELF + 0x74, r.next_u32() % 2);
                oc853_w(c, OC853_SELF + 0x78, r.next_u32() % 5);
                oc853_w(c, OC853_SELF + 0x7c, [0u32, 1, 3][(seed % 3) as usize]);
                oc853_w(
                    c,
                    OC853_WORLD + 0x84,
                    if seed % 4 == 0 { 0 } else { OC853_POOL },
                );
                oc853_w(c, oc853_mail(0x0039_d860), (seed % 5 == 0) as u32);
            },
        );
    }

    /// The constructor: with and without a `**(self+0x10)` (the control-mask
    /// lookup), negative player indices, in-store flag.
    #[test]
    fn cfe_state_request_line_ctor_matches() {
        check(
            ee::s02::f_00196b90__generated,
            cfe_state_request_line_ctor,
            200,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_6b90, 16);
                c.pc = 0x0019_6b90;
                c.g[A2][0] =
                    (r.next_u32() % 5) as u64 | if seed % 7 == 0 { 0xffff_ff00 } else { 0 };
                c.g[A3][0] = (seed % 2) as u64;
                let mgr = OC853_POOL + 0x100;
                oc853_w(c, OC853_SELF + 0x10, mgr);
                oc853_w(c, mgr, if seed % 3 == 0 { 0 } else { OC853_POOL + 0x200 });
            },
        );
    }

    /// The setup: the world flag (`+0x84`) off/on, the screen's lookups
    /// returning nothing or a found object.
    #[test]
    fn cfe_state_request_line_init_matches() {
        check(
            ee::s02::f_00196db0__generated,
            cfe_state_request_line_init,
            200,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_6db0, 5);
                c.pc = 0x0019_6db0;
                oc853_w(
                    c,
                    OC853_WORLD + 0x84,
                    if seed % 4 == 0 { 0 } else { OC853_POOL },
                );
                oc853_w(c, OC853_SELF + 0x10, OC853_POOL + 0x100);
                c.bus.write8(OC853_SELF + 0x14c, r.next_u32() as u8);
                oc853_w(c, oc853_mail(0x0039_f9d8), (seed % 3 == 0) as u32);
                oc853_w(c, oc853_mail(0x0039_7cc0), (seed % 2 == 0) as u32);
            },
        );
    }

    /// Song-list fields the request-line action handler reads: visible range,
    /// song ids at `+0x4c`, owned/play-list masks, money, in-store flag.
    #[test]
    fn cfe_state_request_line_on_widget_action_matches() {
        check(
            ee::s02::f_00197500__generated,
            cfe_state_request_line_on_widget_action,
            300,
            &|c, seed| {
                let mut r = oc853_setup(c, seed, 0x19_7500, 12);
                c.pc = 0x0019_7500;
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(OC853_WIDGET) };
                c.g[A2][0] = [5u64, 6, 6, 5, 0, 6, 5, 9, 6, 5][(seed % 10) as usize];
                oc853_w(c, OC853_WIDGET + 0x18, r.next_u32() % 6);
                oc853_w(c, OC853_SELF + 0x14c, 0);
                oc853_w(c, OC853_SELF + 0x150, r.next_u32() % 5);
                for i in 0..16u32 {
                    oc853_w(c, OC853_SELF + 0x4c + 4 * i, r.next_u32() % 64);
                }
                c.bus.write64(
                    OC853_SELF + 0x158,
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                );
                c.bus.write64(
                    OC853_SELF + 0x160,
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                );
                oc853_w(c, OC853_SELF + 0x16c, r.next_u32() % 2);
                oc853_w(c, OC853_SELF + 0x168, r.next_u32() % 400);
                c.bus.write8(OC853_SELF + 0x44, (r.next_u32() % 4) as u8);
                oc853_w(c, OC853_SELF + 0x10, OC853_POOL + 0x100);
                // Events 1 and 5 (with a song in range) reach the unported song-list
                // helpers, which run away on scattered data (`DLHeap` at 0x31ef40):
                // only the exits of event 5 (no song in range) and event 6 are tested.
                // The new-state path (world flag 0) reaches the unported
                // `cFEStateAudioOptions_ctor`; keep the flag set.
                oc853_w(c, OC853_WORLD + 0x84, OC853_POOL);
                oc853_w(c, OC853_WORLD + 0x5558, OC853_POOL + 0x500);
            },
        );
    }
}
