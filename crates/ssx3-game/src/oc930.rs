//! Staging module oc930 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc930`.

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
        0x001c_c1e0,
        c_keyboard_popup_press_key,
        "cKeyboardPopup_PressKey",
    ),
    (
        0x001c_d0d8,
        c_keyboard_popup_get_key_widget_name,
        "cKeyboardPopup_GetKeyWidgetName",
    ),
    (
        0x001c_d9a0,
        c_keyboard_popup_set_special_key_label,
        "cKeyboardPopup_SetSpecialKeyLabel",
    ),
    (
        0x001c_db98,
        c_keyboard_popup_handle_input,
        "cKeyboardPopup_HandleInput",
    ),
];

/// `void cKeyboardPopup_PressKey(cKeyboardPopup* self, int flag)` (0x1cc1e0): the
/// on-screen keyboard presses the key under the cursor (`self->key`, +0x140).
/// A pending accept (+0x54) is first completed by pressing key 0x46 (recursion).
/// With `flag == 0` and the key enabled it runs the key's action: backspace
/// (0x1b), caps (0x2a), shift (0x38), clear (0x46), left (0x4c), right (0x53),
/// accept (0x37), quick messages (4..13) and the default typed character. It
/// then refreshes the entry text, the accept/clear key states (min length) and
/// re-highlights the clear key (colour through the key widget's vtable).
pub fn c_keyboard_popup_press_key(c: &mut Cpu) {
    if c.pc != 0x001cc1e0 {
        return ee::s04::f_001cc1e0__generated(c);
    }
    guest(c, |c| {
        'b_1cc244: {
            'b_1cc240: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                let t = c.g[V0][0] == 0u64;
                c.g[S4][0] = 0u64;
                if t {
                    break 'b_1cc240;
                } else {
                    c.cycles += 4;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                    c.g[V0][0] = 0x37;
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = 0x46;
                    if t {
                        break 'b_1cc240;
                    } else {
                        c.cycles += 2;
                        let t = c.g[S0][0] == c.g[V0][0];
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_1cc244;
                        } else {
                            c.cycles += 6;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                            c.g[RA][0] = sx(0x001cc238);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001cc1e0,
                                f: ee::s04::cKeyboardPopup_PressKey,
                            }
                            .call(c, 0x001cc238)?;
                            c.g[S4][0] = 0x1;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x140), c.g[S0][0]);
                            break 'b_1cc240;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[S3][0] = 0u64;
            break 'b_1cc244;
        }
        'b_1cc500: {
            'b_1cc4fc: {
                c.cycles += 2;
                let t = c.g[S2][0] != 0u64;
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                if t {
                    break 'b_1cc4fc;
                } else {
                    c.cycles += 6;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cc25c);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x001ce560,
                        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                    }
                    .call(c, 0x001cc25c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                    if t {
                        break 'b_1cc500;
                    } else {
                        'b_1cc4f0: {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[V1][0] < 0x54u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x460000;
                            if t {
                                break 'b_1cc4f0;
                            } else {
                                'b_1cc4d4: {
                                    'b_1cc4b8: {
                                        'b_1cc498: {
                                            'b_1cc46c: {
                                                'b_1cc438: {
                                                    'b_1cc3e8: {
                                                        'b_1cc3b0: {
                                                            'b_1cc388: {
                                                                'b_1cc368: {
                                                                    'b_1cc348: {
                                                                        c.cycles += 6;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x63e0,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[V1][0] as u32)
                                                                            );
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x001cc288 => {
                                                                                'b_1cc314: {
                                                                                    c.cycles += 7;
                                                                                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001cc298,
                                                                                    );
                                                                                    c.g[S0][0] =
                                                                                        c.g[V0][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00416810,
                                                                                        0x001cc298,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                    if c.g[V1][0]
                                                                                        == c.g[V0]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x138));
                                                                                        break 'b_1cc314;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        if c.g[V1]
                                                                                            [0]
                                                                                            == 0u64
                                                                                        {
                                                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                                                                            break 'b_1cc500;
                                                                                        } else {
                                                                                            'b_1cc2b8: {
                                                                                                c.cycles += 2;
                                                                                                let t = (c.g[V1][0] as i64) <= 0;
                                                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1cc2b8;
                                                                                                } else {
                                                                                                    c.cycles += 1;
                                                                                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                                                                                                    break 'b_1cc2b8;
                                                                                                }
                                                                                            }
                                                                                            'b_1cc2ec: {
                                                                                                c.cycles += 11;
                                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x75));
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x74));
                                                                                                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                                                c.g[RA][0] = sx(0x001cc2d0);
                                                                                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                                call_ptr(c, 0x004166f4, 0x001cc2d0)?;
                                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x138));
                                                                                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                                                                let t = c.g[V1][0] != c.g[V0][0];

                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1cc2ec;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x138), c.g[A0][0]);
                                                                                                    c.g[S3][0] = 0x1;
                                                                                                    break 'b_1cc2ec;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 6;
                                                                                            c.g[RA][0] = sx(0x001cc2f4);
                                                                                            c.g[A0][0] = c.g[S0][0];
                                                                                            call_ptr(c, 0x00416810, 0x001cc2f4)?;
                                                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x13c));
                                                                                            c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                                                                                            if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
break 'b_1cc500;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x001cc30c);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00416810, 0x001cc30c)?;
let t = true;
c.sw((c.g[S1][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
if t {
break 'b_1cc4fc;
} else {
break 'b_1cc314;
}
}
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_1cc324: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != c.g[V1]
                                                                                            [0];

                                                                                    if t {
                                                                                        break 'b_1cc324;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x138), 0u64);
                                                                                        if t {
                                                                                            break 'b_1cc4fc;
                                                                                        } else {
                                                                                            break 'b_1cc324;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_1cc330: {
                                                                                    c.cycles += 2;
                                                                                    let t = (c.g[V1]
                                                                                        [0]
                                                                                        as i64)
                                                                                        <= 0;
                                                                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                                                    if t {
                                                                                        break 'b_1cc330;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            1;
                                                                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x134), c.g[V0][0]);
                                                                                        break 'b_1cc330;
                                                                                    }
                                                                                }
                                                                                c.cycles += 6;
                                                                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                c.sb(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                    0u64,
                                                                                );
                                                                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x134));
                                                                                let t = true;
                                                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x13c), c.g[V1][0]);
                                                                                if t {
                                                                                    break 'b_1cc4fc;
                                                                                } else {
                                                                                    break 'b_1cc348;
                                                                                }
                                                                            }
                                                                            0x001cc348 => {
                                                                                break 'b_1cc348;
                                                                            }
                                                                            0x001cc368 => {
                                                                                break 'b_1cc368;
                                                                            }
                                                                            0x001cc388 => {
                                                                                break 'b_1cc388;
                                                                            }
                                                                            0x001cc3b0 => {
                                                                                break 'b_1cc3b0;
                                                                            }
                                                                            0x001cc3e8 => {
                                                                                break 'b_1cc3e8;
                                                                            }
                                                                            0x001cc438 => {
                                                                                break 'b_1cc438;
                                                                            }
                                                                            0x001cc46c => {
                                                                                break 'b_1cc46c;
                                                                            }
                                                                            0x001cc498 => {
                                                                                break 'b_1cc498;
                                                                            }
                                                                            0x001cc4d4 => {
                                                                                break 'b_1cc4d4;
                                                                            }
                                                                            0x001cc4f0 => {
                                                                                break 'b_1cc4f0;
                                                                            }
                                                                            0x001cc4fc => {
                                                                                break 'b_1cc4fc;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x001cc1e0,
                                                                                    0x001cc620,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 8;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x68));
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[S0][0] = sx((c.g[S1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x74));
                                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                    c.g[RA][0] = sx(0x001cc360);
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x68),
                                                                        c.g[V0][0],
                                                                    );
                                                                    Callee { addr: 0x001cd708, f: ee::s04::cKeyboardPopup_RefreshKeys }.call(c, 0x001cc360)?;
                                                                    let t = true;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x140));
                                                                    if t {
                                                                        break 'b_1cc500;
                                                                    } else {
                                                                        break 'b_1cc368;
                                                                    }
                                                                }
                                                                c.cycles += 8;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6c));
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[S0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x74));
                                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                c.g[RA][0] = sx(0x001cc380);
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6c),
                                                                    c.g[V0][0],
                                                                );
                                                                Callee { addr: 0x001cd708, f: ee::s04::cKeyboardPopup_RefreshKeys }.call(c, 0x001cc380)?;
                                                                let t = true;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x140));
                                                                if t {
                                                                    break 'b_1cc500;
                                                                } else {
                                                                    break 'b_1cc388;
                                                                }
                                                            }
                                                            c.cycles += 10;
                                                            c.g[S0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x74));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001cc39c);
                                                            c.g[A2][0] = 0x40;
                                                            call_ptr(c, 0x00416210, 0x001cc39c)?;
                                                            c.sb(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x74),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x134),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x13c),
                                                                0u64,
                                                            );
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x138),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_1cc4fc;
                                                            } else {
                                                                break 'b_1cc3b0;
                                                            }
                                                        }
                                                        'b_1cc3d4: {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x138));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x134));
                                                            let t = c.g[V1][0] != c.g[V0][0];

                                                            if t {
                                                                break 'b_1cc3d4;
                                                            } else {
                                                                'b_1cc3d0: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[V1][0] == 0u64;
                                                                    c.g[V0][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1));
                                                                    if t {
                                                                        break 'b_1cc3d0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x138,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x134));
                                                                        break 'b_1cc3d0;
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[S3][0] = 0x1;
                                                                break 'b_1cc3d4;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = (c.g[V1][0] as i64) <= 0;
                                                        c.g[S0][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x74));
                                                        if t {
                                                            break 'b_1cc4fc;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x134),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1cc4fc;
                                                            } else {
                                                                break 'b_1cc3e8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001cc3f8);
                                                    c.g[S0][0] = c.g[V0][0];
                                                    call_ptr(c, 0x00416810, 0x001cc3f8)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x134)
                                                        );
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x140));
                                                        break 'b_1cc500;
                                                    } else {
                                                        'b_1cc41c: {
                                                            c.cycles += 5;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x60));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                < (c.g[V0][0] as i64))
                                                                as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_1cc41c;
                                                            } else {
                                                                c.cycles += 1;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x134),
                                                                    c.g[V0][0],
                                                                );
                                                                break 'b_1cc41c;
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x13c));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x134));
                                                        if c.g[V1][0] != c.g[V0][0] {
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x140));
                                                            break 'b_1cc500;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x13c),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1cc4fc;
                                                            } else {
                                                                break 'b_1cc438;
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_1cc450: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x420)
                                                        );
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_1cc450;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[RA][0] = sx(0x001cc450);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        call_ptr(c, 0x0039f190, 0x001cc450)?;
                                                        break 'b_1cc450;
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x1;
                                                c.g[A3][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0x5c),
                                                    c.g[V0][0],
                                                );
                                                c.g[A1][0] = 0u64;
                                                c.g[A2][0] = 0xd;
                                                let t = true;
                                                c.g[S0][0] =
                                                    sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                                if t {
                                                    break 'b_1cc4b8;
                                                } else {
                                                    break 'b_1cc46c;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = 0x4b;
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001cc480);
                                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                            Callee {
                                                addr: 0x001ce560,
                                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                            }
                                            .call(c, 0x001cc480)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                                break 'b_1cc500;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001cc490);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee {
                                                    addr: 0x001cc7c8,
                                                    f: ee::s04::cKeyboardPopup_TypeCurrentKey,
                                                }
                                                .call(c, 0x001cc490)?;
                                                let t = true;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                                if t {
                                                    break 'b_1cc500;
                                                } else {
                                                    break 'b_1cc498;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x1;
                                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                        c.g[RA][0] = sx(0x001cc4ac);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f190, 0x001cc4ac)?;
                                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[A1][0] = 0u64;
                                        c.g[A2][0] = 0xe;
                                        break 'b_1cc4b8;
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001cc4cc);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cc4cc)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                    if t {
                                        break 'b_1cc500;
                                    } else {
                                        break 'b_1cc4d4;
                                    }
                                }
                                c.cycles += 7;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                c.g[A0][0] = c.g[S1][0];
                                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                                c.g[RA][0] = sx(0x001cc4e8);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x4));
                                Callee {
                                    addr: 0x001ce5e0,
                                    f: ee::s04::cKeyboardPopup_AppendQuickMessage,
                                }
                                .call(c, 0x001cc4e8)?;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1cc500;
                                } else {
                                    break 'b_1cc4f0;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x001cc4f8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001cc7c8,
                            f: ee::s04::cKeyboardPopup_TypeCurrentKey,
                        }
                        .call(c, 0x001cc4f8)?;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                        break 'b_1cc4fc;
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x140));
            break 'b_1cc500;
        }
        'b_1cc528: {
            c.cycles += 3;
            c.g[V0][0] = 0x38;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            if t {
                break 'b_1cc528;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                if t {
                    break 'b_1cc528;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cc524);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                    Callee {
                        addr: 0x001cd708,
                        f: ee::s04::cKeyboardPopup_RefreshKeys,
                    }
                    .call(c, 0x001cc524)?;
                    c.g[A1][0] = c.g[S3][0];
                    break 'b_1cc528;
                }
            }
        }
        'b_1cc558: {
            c.cycles += 5;
            c.g[RA][0] = sx(0x001cc530);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x001cc848,
                f: ee::s04::cKeyboardPopup_RefreshEntryText,
            }
            .call(c, 0x001cc530)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x58));
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1cc558;
            } else {
                c.cycles += 7;
                c.g[RA][0] = sx(0x001cc544);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416810, 0x001cc544)?;
                c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x4b;
                c.g[RA][0] = sx(0x001cc558);
                c.g[A3][0] = 0x2;
                Callee {
                    addr: 0x001ce3c8,
                    f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                }
                .call(c, 0x001cc558)?;
                break 'b_1cc558;
            }
        }
        'b_1cc598: {
            'b_1cc588: {
                c.cycles += 6;
                c.g[RA][0] = sx(0x001cc560);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00416810, 0x001cc560)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x37;
                if t {
                    break 'b_1cc588;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x001cc580);
                    c.g[A3][0] = 0x2;
                    Callee {
                        addr: 0x001ce3c8,
                        f: ee::s04::cKeyboardPopup_SetKeyEnabled,
                    }
                    .call(c, 0x001cc580)?;
                    let t = true;

                    if t {
                        break 'b_1cc598;
                    } else {
                        break 'b_1cc588;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0x1;
            c.g[RA][0] = sx(0x001cc598);
            c.g[A3][0] = 0x2;
            Callee {
                addr: 0x001ce3c8,
                f: ee::s04::cKeyboardPopup_SetKeyEnabled,
            }
            .call(c, 0x001cc598)?;
            break 'b_1cc598;
        }
        'b_1cc604: {
            'b_1cc600: {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001cc5a0);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x001cd708,
                    f: ee::s04::cKeyboardPopup_RefreshKeys,
                }
                .call(c, 0x001cc5a0)?;
                let t = c.g[S4][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_1cc600;
                } else {
                    c.cycles += 11;
                    c.g[A1][0] = 0x46;
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x001cc5b8);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001cd0d8,
                        f: ee::s04::cKeyboardPopup_GetKeyWidgetName,
                    }
                    .call(c, 0x001cc5b8)?;
                    c.g[RA][0] = sx(0x001cc5c0);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001cc5c0)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001cc5cc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001cc5cc)?;
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_1cc604;
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x58fc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cc600);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x001cc600)?;
                        break 'b_1cc600;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1cc604;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `int cKeyboardPopup_GetKeyWidgetName(self, int key, char* out, int shifted)`
/// (0x1cd0d8): writes the UI object name of a key (a name prefix, then the key's
/// suffix: special keys by table, F-keys by `sprintf`, characters from the key
/// character table) into `out`; returns 0 for keys whose character is `*`.
pub fn c_keyboard_popup_get_key_widget_name(c: &mut Cpu) {
    if c.pc != 0x001cd0d8 {
        return ee::s04::f_001cd0d8__generated(c);
    }
    guest(c, |c| {
        'b_1cd13c: {
            'b_1cd110: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[T0][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                let t = c.g[A3][0] == 0u64;
                c.g[S0][0] = c.g[A2][0];
                if t {
                    break 'b_1cd110;
                } else {
                    c.cycles += 8;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1368));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[A2][0] as u32));
                    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
                    c.sdr(3, (c.g[S0][0] as u32));
                    let t = true;
                    c.g[V0][0] = (c.g[T0][0] < 0x54u64) as u64;
                    if t {
                        break 'b_1cd13c;
                    } else {
                        break 'b_1cd110;
                    }
                }
            }
            c.cycles += 11;
            c.g[V0][0] = 0x460000;
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2ab0));
            c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[A2][0] as u32));
            c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x9));
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
            c.sdr(3, (c.g[S0][0] as u32));
            c.sb((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.sb((c.g[S0][0] as u32).wrapping_add(0x9), c.g[A1][0]);
            c.g[V0][0] = (c.g[T0][0] < 0x54u64) as u64;
            break 'b_1cd13c;
        }
        'b_1cd2e0: {
            'b_1cd2a4: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1cd2a4;
                } else {
                    'b_1cd27c: {
                        'b_1cd264: {
                            'b_1cd24c: {
                                'b_1cd234: {
                                    'b_1cd21c: {
                                        'b_1cd204: {
                                            'b_1cd1ec: {
                                                'b_1cd1d4: {
                                                    'b_1cd1bc: {
                                                        'b_1cd1a4: {
                                                            'b_1cd18c: {
                                                                'b_1cd174: {
                                                                    c.cycles += 6;
                                                                    c.g[V1][0] = sx((c.g[T0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x65b0));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            (c.g[V0][0] as u32),
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw((c.g[V1][0] as u32));
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x001cd15c => {
                                                                            c.cycles += 6;
                                                                            c.g[A1][0] = 0x4a0000;
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001cd16c);
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x12a8,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x004162d0,
                                                                                0x001cd16c,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[V0][0] = 0x1;
                                                                            if t {
                                                                                break 'b_1cd2e0;
                                                                            } else {
                                                                                break 'b_1cd174;
                                                                            }
                                                                        }
                                                                        0x001cd174 => {
                                                                            break 'b_1cd174;
                                                                        }
                                                                        0x001cd18c => {
                                                                            break 'b_1cd18c;
                                                                        }
                                                                        0x001cd1a4 => {
                                                                            break 'b_1cd1a4;
                                                                        }
                                                                        0x001cd1bc => {
                                                                            break 'b_1cd1bc;
                                                                        }
                                                                        0x001cd1d4 => {
                                                                            break 'b_1cd1d4;
                                                                        }
                                                                        0x001cd1ec => {
                                                                            break 'b_1cd1ec;
                                                                        }
                                                                        0x001cd204 => {
                                                                            break 'b_1cd204;
                                                                        }
                                                                        0x001cd21c => {
                                                                            break 'b_1cd21c;
                                                                        }
                                                                        0x001cd234 => {
                                                                            break 'b_1cd234;
                                                                        }
                                                                        0x001cd24c => {
                                                                            break 'b_1cd24c;
                                                                        }
                                                                        0x001cd264 => {
                                                                            break 'b_1cd264;
                                                                        }
                                                                        0x001cd27c => {
                                                                            break 'b_1cd27c;
                                                                        }
                                                                        0x001cd2a4 => {
                                                                            break 'b_1cd2a4;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x001cd0d8,
                                                                                0x001cd2f0,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 6;
                                                                c.g[A1][0] = 0x4a0000;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x001cd184);
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1c80));
                                                                call_ptr(
                                                                    c, 0x004162d0, 0x001cd184,
                                                                )?;
                                                                let t = true;
                                                                c.g[V0][0] = 0x1;
                                                                if t {
                                                                    break 'b_1cd2e0;
                                                                } else {
                                                                    break 'b_1cd18c;
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001cd19c);
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1c88));
                                                            call_ptr(c, 0x004162d0, 0x001cd19c)?;
                                                            let t = true;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_1cd2e0;
                                                            } else {
                                                                break 'b_1cd1a4;
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x001cd1b4);
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1c90));
                                                        call_ptr(c, 0x004162d0, 0x001cd1b4)?;
                                                        let t = true;
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_1cd2e0;
                                                        } else {
                                                            break 'b_1cd1bc;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x001cd1cc);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1350));
                                                    call_ptr(c, 0x004162d0, 0x001cd1cc)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_1cd2e0;
                                                    } else {
                                                        break 'b_1cd1d4;
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x001cd1e4);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x2aa0));
                                                call_ptr(c, 0x004162d0, 0x001cd1e4)?;
                                                let t = true;
                                                c.g[V0][0] = 0x1;
                                                if t {
                                                    break 'b_1cd2e0;
                                                } else {
                                                    break 'b_1cd1ec;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001cd1fc);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1c98));
                                            call_ptr(c, 0x004162d0, 0x001cd1fc)?;
                                            let t = true;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1cd2e0;
                                            } else {
                                                break 'b_1cd204;
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001cd214);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ca0));
                                        call_ptr(c, 0x004162d0, 0x001cd214)?;
                                        let t = true;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1cd2e0;
                                        } else {
                                            break 'b_1cd21c;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001cd22c);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ca8));
                                    call_ptr(c, 0x004162d0, 0x001cd22c)?;
                                    let t = true;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1cd2e0;
                                    } else {
                                        break 'b_1cd234;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A1][0] = 0x4a0000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001cd244);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cb0));
                                call_ptr(c, 0x004162d0, 0x001cd244)?;
                                let t = true;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_1cd2e0;
                                } else {
                                    break 'b_1cd24c;
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001cd25c);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cb8));
                            call_ptr(c, 0x004162d0, 0x001cd25c)?;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1cd2e0;
                            } else {
                                break 'b_1cd264;
                            }
                        }
                        c.cycles += 6;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001cd274);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                        call_ptr(c, 0x004162d0, 0x001cd274)?;
                        let t = true;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_1cd2e0;
                        } else {
                            break 'b_1cd27c;
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cc8));
                    c.g[RA][0] = sx(0x001cd290);
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x3));
                    call_ptr(c, 0x00417828, 0x001cd290)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001cd29c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x004162d0, 0x001cd29c)?;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_1cd2e0;
                    } else {
                        break 'b_1cd2a4;
                    }
                }
            }
            'b_1cd2c8: {
                c.cycles += 7;
                c.g[V0][0] = 0x440000;
                c.g[A0][0] = 0x2a;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1390));
                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32));
                let t = c.g[V1][0] != c.g[A0][0];
                c.g[A1][0] = c.lbu((c.g[V0][0] as u32));
                if t {
                    break 'b_1cd2c8;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_1cd2e0;
                    } else {
                        break 'b_1cd2c8;
                    }
                }
            }
            c.cycles += 6;
            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), 0u64);
            c.g[RA][0] = sx(0x001cd2dc);
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x004162d0, 0x001cd2dc)?;
            c.g[V0][0] = 0x1;
            break 'b_1cd2e0;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cKeyboardPopup_SetSpecialKeyLabel(self, cUIText* text, int key)` (0x1cd9a0):
/// colours the key's text (enabled or disabled colour), then sets the label of
/// the special keys (backspace, caps, shift, clear, left, right, accept...) from
/// a text id; the digit keys 4..13 are shown/hidden by the caps state and get
/// their number as an ASCII string.
pub fn c_keyboard_popup_set_special_key_label(c: &mut Cpu) {
    if c.pc != 0x001cd9a0 {
        return ee::s04::f_001cd9a0__generated(c);
    }
    guest(c, |c| {
        'b_1cdb84: {
            'b_1cdb80: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                let t = c.g[S0][0] == 0u64;
                c.g[S2][0] = c.g[A2][0];
                if t {
                    break 'b_1cdb80;
                } else {
                    'b_1cda14: {
                        'b_1cd9f8: {
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cd9d0);
                            c.g[A2][0] = 0u64;
                            Callee {
                                addr: 0x001ce560,
                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                            }
                            .call(c, 0x001cd9d0)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            if t {
                                break 'b_1cd9f8;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = 0x4c0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x67b8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001cd9f0);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cd9f0)?;
                                let t = true;
                                c.g[V0][0] = (c.g[S2][0] < 0x54u64) as u64;
                                if t {
                                    break 'b_1cda14;
                                } else {
                                    break 'b_1cd9f8;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A1][0] = 0x4c0000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x67c8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cda10);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cda10)?;
                        c.g[V0][0] = (c.g[S2][0] < 0x54u64) as u64;
                        break 'b_1cda14;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_1cdb80;
                    } else {
                        'b_1cdb20: {
                            'b_1cdb04: {
                                'b_1cdabc: {
                                    'b_1cdab0: {
                                        'b_1cdaa4: {
                                            'b_1cda98: {
                                                'b_1cda8c: {
                                                    'b_1cda54: {
                                                        'b_1cda4c: {
                                                            'b_1cda40: {
                                                                c.cycles += 6;
                                                                c.g[V1][0] =
                                                                    sx((c.g[S2][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x67f0));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x001cda34 => {
                                                                        c.cycles += 3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        let t = true;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x6720,
                                                                                ));
                                                                        if t {
                                                                            break 'b_1cdb04;
                                                                        } else {
                                                                            break 'b_1cda40;
                                                                        }
                                                                    }
                                                                    0x001cda40 => {
                                                                        break 'b_1cda40;
                                                                    }
                                                                    0x001cda4c => {
                                                                        break 'b_1cda4c;
                                                                    }
                                                                    0x001cda8c => {
                                                                        break 'b_1cda8c;
                                                                    }
                                                                    0x001cda98 => {
                                                                        break 'b_1cda98;
                                                                    }
                                                                    0x001cdaa4 => {
                                                                        break 'b_1cdaa4;
                                                                    }
                                                                    0x001cdab0 => {
                                                                        break 'b_1cdab0;
                                                                    }
                                                                    0x001cdabc => {
                                                                        break 'b_1cdabc;
                                                                    }
                                                                    0x001cdb20 => {
                                                                        break 'b_1cdb20;
                                                                    }
                                                                    0x001cdb80 => {
                                                                        break 'b_1cdb80;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x001cd9a0,
                                                                            0x001cdb98,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x460000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x6740));
                                                            if t {
                                                                break 'b_1cda54;
                                                            } else {
                                                                break 'b_1cda4c;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6760));
                                                        break 'b_1cda54;
                                                    }
                                                    c.cycles += 14;
                                                    c.g[RA][0] = sx(0x001cda5c);
                                                    call_ptr(c, 0x00317670, 0x001cda5c)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001cda68);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x003a1000, 0x001cda68)?;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x4c0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x67c8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001cda84);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001cda84)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_1cdb84;
                                                    } else {
                                                        break 'b_1cda8c;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6778));
                                                if t {
                                                    break 'b_1cdb04;
                                                } else {
                                                    break 'b_1cda98;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = 0x460000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x6790));
                                            if t {
                                                break 'b_1cdb04;
                                            } else {
                                                break 'b_1cdaa4;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x67a8));
                                        if t {
                                            break 'b_1cdb04;
                                        } else {
                                            break 'b_1cdab0;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x67c0));
                                    if t {
                                        break 'b_1cdb04;
                                    } else {
                                        break 'b_1cdabc;
                                    }
                                }
                                'b_1cdb00: {
                                    'b_1cdae8: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x43c));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1cdae8;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001cdae0);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001cdae0)?;
                                            let t = true;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_1cdb00;
                                            } else {
                                                break 'b_1cdae8;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001cdafc);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cdafc)?;
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1cdb00;
                                }
                                c.cycles += 1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x67d8));
                                break 'b_1cdb04;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001cdb0c);
                            call_ptr(c, 0x00317670, 0x001cdb0c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001cdb18);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001cdb18)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1cdb84;
                            } else {
                                break 'b_1cdb20;
                            }
                        }
                        'b_1cdb64: {
                            'b_1cdb4c: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x43c));
                                c.g[V0][0] = 0x1;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1cdb4c;
                                } else {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001cdb44);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cdb44)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1cdb64;
                                    } else {
                                        break 'b_1cdb4c;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001cdb60);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cdb60)?;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1cdb64;
                        }
                        c.cycles += 7;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cc8));
                        c.g[RA][0] = sx(0x001cdb74);
                        c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3));
                        call_ptr(c, 0x00417828, 0x001cdb74)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001cdb80);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0c50, 0x001cdb80)?;
                        break 'b_1cdb80;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1cdb84;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `int cKeyboardPopup_HandleInput(self, InputEvent* ev)` (0x1cdb98): the
/// keyboard popup's input handler. Moves the cursor key (left/right/up/down),
/// presses a key, opens the help popup, closes, toggles the two modifier flags,
/// or maps a typed (PC) key to a keyboard key; highlights the old and new key
/// widgets and plays the feedback sound. Always returns 1.
pub fn c_keyboard_popup_handle_input(c: &mut Cpu) {
    if c.pc != 0x001cdb98 {
        return ee::s04::f_001cdb98__generated(c);
    }
    guest(c, |c| 'l_1cdb98: loop {
        'b_1ce284: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S0][0] = c.g[A1][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001cdbd4);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001cdbd4)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_1ce284;
            } else {
                'b_1cdc44: {
                    c.cycles += 14;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x001cdbf0);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001cd0d8,
                        f: ee::s04::cKeyboardPopup_GetKeyWidgetName,
                    }
                    .call(c, 0x001cdbf0)?;
                    c.g[S3][0] = 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001cdc00);
                    c.g[S4][0] = 0x1;
                    call_ptr(c, 0x00317670, 0x001cdc00)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001cdc0c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001cdc0c)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_1cdc44;
                    } else {
                        c.cycles += 12;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x58f8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdc40);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x001cdc40)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        break 'b_1cdc44;
                    }
                }
                'b_1ce1f4: {
                    'b_1cdc74: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdc54);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cdc54)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdc74;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdc6c);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb500,
                                f: ee::s04::cKeyboardPopup_MoveLeft,
                            }
                            .call(c, 0x001cdc6c)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdc74;
                            }
                        }
                    }
                    'b_1cdca4: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdc84);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cdc84)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdca4;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdc9c);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb650,
                                f: ee::s04::cKeyboardPopup_MoveRight,
                            }
                            .call(c, 0x001cdc9c)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdca4;
                            }
                        }
                    }
                    'b_1cdcd4: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdcb4);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cdcb4)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdcd4;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdccc);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb7f0,
                                f: ee::s04::cKeyboardPopup_MoveUp,
                            }
                            .call(c, 0x001cdccc)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdcd4;
                            }
                        }
                    }
                    'b_1cdd04: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdce4);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cdce4)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdd04;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdcfc);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001cb9b0,
                                f: ee::s04::cKeyboardPopup_MoveDown,
                            }
                            .call(c, 0x001cdcfc)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdd04;
                            }
                        }
                    }
                    'b_1cdd34: {
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdd14);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cdd14)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cdd34;
                        } else {
                            c.cycles += 6;
                            c.g[S3][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001cdd2c);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001cc1e0,
                                f: ee::s04::cKeyboardPopup_PressKey,
                            }
                            .call(c, 0x001cdd2c)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1ce1f4;
                            } else {
                                break 'b_1cdd34;
                            }
                        }
                    }
                    'b_1ce1f0: {
                        'b_1cdea8: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdd44);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cdd44)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                            if t {
                                break 'b_1cdea8;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                    break 'b_1ce1f4;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[S4][0] = 0x5;
                                    if t {
                                        break 'b_1ce1f0;
                                    } else {
                                        c.cycles += 80;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A3][0] = 0u64;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a70));
                                        c.g[A0][0] = 0x140;
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x001cdd84);
                                        c.g[S3][0] = 0x1;
                                        call_ptr(c, 0x00317d70, 0x001cdd84)?;
                                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001cdd94);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001d9d48,
                                            f: ee::s04::cFEStateNetHelpPopup_ctor,
                                        }
                                        .call(c, 0x001cdd94)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x6;
                                        c.g[RA][0] = sx(0x001cdda4);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da528,
                                            f: ee::s04::cFEStateNetHelpPopup_SetNumButtons,
                                        }
                                        .call(c, 0x001cdda4)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cddb0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6940));
                                        call_ptr(c, 0x00317670, 0x001cddb0)?;
                                        c.g[A0][0] = 0x460000;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001cddc0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6950));
                                        call_ptr(c, 0x00317670, 0x001cddc0)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[RA][0] = sx(0x001cddd0);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da530,
                                            f: ee::s04::cFEStateNetHelpPopup_SetBodyText,
                                        }
                                        .call(c, 0x001cddd0)?;
                                        c.g[S0][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6968));
                                        c.g[RA][0] = sx(0x001cdde0);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00317670, 0x001cdde0)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[A3][0] = 0x4;
                                        c.g[RA][0] = sx(0x001cddf4);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cddf4)?;
                                        c.g[RA][0] = sx(0x001cddfc);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00317670, 0x001cddfc)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        c.g[A3][0] = 0x3;
                                        c.g[RA][0] = sx(0x001cde10);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde10)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde1c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6988));
                                        call_ptr(c, 0x00317670, 0x001cde1c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[A3][0] = 0x5;
                                        c.g[RA][0] = sx(0x001cde30);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde30)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde3c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x69a0));
                                        call_ptr(c, 0x00317670, 0x001cde3c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x3;
                                        c.g[A3][0] = 0x6;
                                        c.g[RA][0] = sx(0x001cde50);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde50)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde5c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x69b8));
                                        call_ptr(c, 0x00317670, 0x001cde5c)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x4;
                                        c.g[A3][0] = 0u64;
                                        c.g[RA][0] = sx(0x001cde70);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde70)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001cde7c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x69d8));
                                        call_ptr(c, 0x00317670, 0x001cde7c)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = 0x5;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001cde90);
                                        c.g[A3][0] = 0x1;
                                        Callee {
                                            addr: 0x001da510,
                                            f: ee::s04::cFEStateNetHelpPopup_SetButtonHelp,
                                        }
                                        .call(c, 0x001cde90)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001cdea0);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f290, 0x001cdea0)?;
                                        let t = true;
                                        c.g[S4][0] = 0x5;
                                        if t {
                                            break 'b_1ce1f0;
                                        } else {
                                            break 'b_1cdea8;
                                        }
                                    }
                                }
                            }
                        }
                        'b_1cdf00: {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdebc);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cdebc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1cdf00;
                            } else {
                                c.cycles += 15;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[S3][0] = 0x1;
                                c.g[S4][0] = 0x6;
                                c.g[RA][0] = sx(0x001cded8);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001cded8)?;
                                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0xe;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001cdef8);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cdef8)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1ce1f4;
                                } else {
                                    break 'b_1cdf00;
                                }
                            }
                        }
                        'b_1cdf3c: {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdf14);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cdf14)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1cdf3c;
                            } else {
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                                c.g[S3][0] = 0x1;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                c.g[RA][0] = sx(0x001cdf34);
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                                Callee {
                                    addr: 0x001cd708,
                                    f: ee::s04::cKeyboardPopup_RefreshKeys,
                                }
                                .call(c, 0x001cdf34)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1ce1f4;
                                } else {
                                    break 'b_1cdf3c;
                                }
                            }
                        }
                        'b_1cdf74: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001cdf4c);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cdf4c)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1cdf74;
                            } else {
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
                                c.g[S3][0] = 0x1;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                c.g[RA][0] = sx(0x001cdf6c);
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                                Callee {
                                    addr: 0x001cd708,
                                    f: ee::s04::cKeyboardPopup_RefreshKeys,
                                }
                                .call(c, 0x001cdf6c)?;
                                let t = true;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_1ce1f4;
                                } else {
                                    break 'b_1cdf74;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[S1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x148));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001cdf88);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cdf88)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S5][0] = 0u64;
                        if t {
                            break 'b_1ce1f0;
                        } else {
                            'b_1ce1d8: {
                                'b_1ce138: {
                                    c.cycles += 9;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001cdfa4);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cdfa4)?;
                                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8));
                                    c.g[V0][0] = (c.g[V1][0] < 0x72u64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x460000;
                                    if t {
                                        break 'b_1ce138;
                                    } else {
                                        'b_1ce130: {
                                            'b_1ce0f8: {
                                                'b_1ce0d8: {
                                                    'b_1ce0b8: {
                                                        'b_1ce098: {
                                                            'b_1ce078: {
                                                                'b_1ce058: {
                                                                    'b_1ce038: {
                                                                        'b_1ce018: {
                                                                            'b_1cdff8: {
                                                                                'b_1cdfec: {
                                                                                    c.cycles += 6;
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[V1][0]
                                                                                            as u32)
                                                                                            << 2,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x69f0));
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                    c.g[A0][0] = c
                                                                                        .lw(
                                                                                        (c.g[V1][0]
                                                                                            as u32),
                                                                                    );
                                                                                    let t = c.g[A0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    match t {
0x001cdb98 => {
if event_due(c) { deliver_event_here(c, 0x001cdb98); } continue 'l_1cdb98;
}
0x001cdfcc => {
c.cycles += 6;
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = 0x1b;
c.g[RA][0] = sx(0x001cdfdc);
c.g[A2][0] = 0u64;
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cdfdc)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1b;
if t {
break 'b_1ce130;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S2][0] as u32).wrapping_add(0x140), c.g[V0][0]);
if t {
break 'b_1ce1d8;
} else {
break 'b_1cdfec;
}
}
}
0x001cdfec => {
break 'b_1cdfec;
}
0x001cdff8 => {
break 'b_1cdff8;
}
0x001ce018 => {
break 'b_1ce018;
}
0x001ce038 => {
break 'b_1ce038;
}
0x001ce058 => {
break 'b_1ce058;
}
0x001ce078 => {
break 'b_1ce078;
}
0x001ce098 => {
break 'b_1ce098;
}
0x001ce0b8 => {
break 'b_1ce0b8;
}
0x001ce0d8 => {
break 'b_1ce0d8;
}
0x001ce0f8 => {
break 'b_1ce0f8;
}
0x001ce138 => {
break 'b_1ce138;
}
_ => { c.jump_out(t, 0x001cdb98, 0x001ce2a8); return Ok(()); }
}
                                                                                }
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = 0x37;
                                                                                let t = true;
                                                                                c.sw((c.g[S2][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_1ce1d8;
                                                                                } else {
                                                                                    break 'b_1cdff8;
                                                                                }
                                                                            }
                                                                            c.cycles += 6;
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            c.g[A1][0] = 0x38;
                                                                            c.g[RA][0] =
                                                                                sx(0x001ce008);
                                                                            c.g[A2][0] = 0u64;
                                                                            Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce008)?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[V0][0] = 0x38;
                                                                            if t {
                                                                                break 'b_1ce130;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.sw((c.g[S2][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_1ce1d8;
                                                                                } else {
                                                                                    break 'b_1ce018;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 6;
                                                                        c.g[A0][0] = c.g[S2][0];
                                                                        c.g[A1][0] = 0x2a;
                                                                        c.g[RA][0] = sx(0x001ce028);
                                                                        c.g[A2][0] = 0u64;
                                                                        Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce028)?;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[V0][0] = 0x2a;
                                                                        if t {
                                                                            break 'b_1ce130;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.sw(
                                                                                (c.g[S2][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x140,
                                                                                    ),
                                                                                c.g[V0][0],
                                                                            );
                                                                            if t {
                                                                                break 'b_1ce1d8;
                                                                            } else {
                                                                                break 'b_1ce038;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 6;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    c.g[A1][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x001ce048);
                                                                    c.g[A2][0] = 0u64;
                                                                    Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce048)?;
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[S1][0] = 0x1;
                                                                        break 'b_1ce1d8;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.sw(
                                                                            (c.g[S2][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x140,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        if t {
                                                                            break 'b_1ce1d8;
                                                                        } else {
                                                                            break 'b_1ce058;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 6;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[A1][0] = 0x4c;
                                                                c.g[RA][0] = sx(0x001ce068);
                                                                c.g[A2][0] = 0u64;
                                                                Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce068)?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = 0x4c;
                                                                if t {
                                                                    break 'b_1ce130;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.sw(
                                                                        (c.g[S2][0] as u32)
                                                                            .wrapping_add(0x140),
                                                                        c.g[V0][0],
                                                                    );
                                                                    if t {
                                                                        break 'b_1ce1d8;
                                                                    } else {
                                                                        break 'b_1ce078;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            c.g[A1][0] = 0x53;
                                                            c.g[RA][0] = sx(0x001ce088);
                                                            c.g[A2][0] = 0u64;
                                                            Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001ce088)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x53;
                                                            if t {
                                                                break 'b_1ce130;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x140),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1ce1d8;
                                                                } else {
                                                                    break 'b_1ce098;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[A1][0] = 0x45;
                                                        c.g[RA][0] = sx(0x001ce0a8);
                                                        c.g[A2][0] = 0u64;
                                                        Callee {
                                                            addr: 0x001ce560,
                                                            f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                                        }
                                                        .call(c, 0x001ce0a8)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x45;
                                                        if t {
                                                            break 'b_1ce130;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x140),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_1ce1d8;
                                                            } else {
                                                                break 'b_1ce0b8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[A1][0] = 0x4f;
                                                    c.g[RA][0] = sx(0x001ce0c8);
                                                    c.g[A2][0] = 0u64;
                                                    Callee {
                                                        addr: 0x001ce560,
                                                        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                                    }
                                                    .call(c, 0x001ce0c8)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x4f;
                                                    if t {
                                                        break 'b_1ce130;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x140),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1ce1d8;
                                                        } else {
                                                            break 'b_1ce0d8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 6;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = 0x1c;
                                                c.g[RA][0] = sx(0x001ce0e8);
                                                c.g[A2][0] = 0u64;
                                                Callee {
                                                    addr: 0x001ce560,
                                                    f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                                }
                                                .call(c, 0x001ce0e8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x1c;
                                                if t {
                                                    break 'b_1ce130;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S2][0] as u32).wrapping_add(0x140),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_1ce1d8;
                                                    } else {
                                                        break 'b_1ce0f8;
                                                    }
                                                }
                                            }
                                            c.cycles += 12;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001ce10c);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001ce10c)?;
                                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001ce120);
                                            c.g[A2][0] = 0u64;
                                            Callee {
                                                addr: 0x001ce560,
                                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                            }
                                            .call(c, 0x001ce120)?;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[S1][0] = 0x1;
                                                break 'b_1ce1d8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x140),
                                                    c.g[S0][0],
                                                );
                                                if t {
                                                    break 'b_1ce1d8;
                                                } else {
                                                    break 'b_1ce130;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[S1][0] = 0x1;
                                        if t {
                                            break 'b_1ce1d8;
                                        } else {
                                            break 'b_1ce138;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001ce14c);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001ce14c)?;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 32i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S5][0] = 0x1;
                                if t {
                                    break 'b_1ce1d8;
                                } else {
                                    c.cycles += 13;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001ce16c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001ce16c)?;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                    c.g[RA][0] = sx(0x001ce180);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001cc6e8,
                                        f: ee::s04::cKeyboardPopup_HandleKeyChar,
                                    }
                                    .call(c, 0x001ce180)?;
                                    c.g[S1][0] = c.g[V0][0] ^ 0x1;
                                    let t = c.g[S1][0] != 0u64;
                                    c.g[S5][0] = 0x1;
                                    if t {
                                        break 'b_1ce1d8;
                                    } else {
                                        'b_1ce1d0: {
                                            c.cycles += 6;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001ce19c);
                                            c.g[A2][0] = 0u64;
                                            Callee {
                                                addr: 0x001ce560,
                                                f: ee::s04::cKeyboardPopup_IsKeyEnabled,
                                            }
                                            .call(c, 0x001ce19c)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x60;
                                            if t {
                                                break 'b_1ce1d0;
                                            } else {
                                                'b_1ce1c0: {
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A1][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_1ce1c0;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[RA][0] = sx(0x001ce1b8);
                                                        c.g[A1][0] = 0x7e;
                                                        Callee {
                                                            addr: 0x001cc620,
                                                            f: ee::s04::cKeyboardPopup_InsertChar,
                                                        }
                                                        .call(c, 0x001ce1b8)?;
                                                        let t = true;
                                                        c.g[S5][0] = 0x1;
                                                        if t {
                                                            break 'b_1ce1d8;
                                                        } else {
                                                            break 'b_1ce1c0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001ce1c8);
                                                c.g[A0][0] = c.g[S2][0];
                                                Callee {
                                                    addr: 0x001cc620,
                                                    f: ee::s04::cKeyboardPopup_InsertChar,
                                                }
                                                .call(c, 0x001ce1c8)?;
                                                let t = true;
                                                c.g[S5][0] = 0x1;
                                                if t {
                                                    break 'b_1ce1d8;
                                                } else {
                                                    break 'b_1ce1d0;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[S1][0] = 0x1;
                                        c.g[S5][0] = 0x1;
                                        break 'b_1ce1d8;
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[S1][0] != 0u64 {
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                                break 'b_1ce1f4;
                            } else {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S5][0];
                                c.g[S3][0] = 0x1;
                                c.g[RA][0] = sx(0x001ce1f0);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001cc1e0,
                                    f: ee::s04::cKeyboardPopup_PressKey,
                                }
                                .call(c, 0x001ce1f0)?;
                                break 'b_1ce1f0;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x140));
                    break 'b_1ce1f4;
                }
                'b_1ce254: {
                    c.cycles += 11;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x001ce204);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001cd0d8,
                        f: ee::s04::cKeyboardPopup_GetKeyWidgetName,
                    }
                    .call(c, 0x001ce204)?;
                    c.g[RA][0] = sx(0x001ce20c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001ce20c)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001ce218);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001ce218)?;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        break 'b_1ce254;
                    } else {
                        c.cycles += 13;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001ce250);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        call_ptr(c, t, 0x001ce250)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        break 'b_1ce254;
                    }
                }
                'b_1ce280: {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[S3][0] == 0u64;
                    if c.g[A2][0] == 0 {
                        c.g[A2][0] = c.g[V1][0];
                    }
                    if t {
                        break 'b_1ce280;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = c.g[S4][0];
                        if t {
                            break 'b_1ce280;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001ce280);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001ce280)?;
                            break 'b_1ce280;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_1ce284;
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// Test constants of the keyboard popup fixtures (clear of the fakes' call log
/// at `DATA+0x100000`).
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, elf_image, run_generated, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word of this module's fakes (in each machine's own RAM); the
    /// per-callee call counters, result mailboxes and the input-event result
    /// table sit behind it.
    pub(crate) const OC930_CTRL: u32 = DATA + 0x2e_0000;
    pub(crate) const OC930_MAGIC: u32 = 0x0930_0930;
    /// Result table of the input-event vtable calls, one word per event code.
    pub(crate) const OC930_EV_RES: u32 = OC930_CTRL + 0x1000;
    /// Fake targets of the input-event vtable slots (`+ 0x10 * code`).
    pub(crate) const OC930_EV_FAKE: u32 = FAKE + 0x1000;

    pub(crate) const OC930_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC930_OWNER: u32 = DATA + 0x2000;
    pub(crate) const OC930_PARENT: u32 = DATA + 0x2400;
    pub(crate) const OC930_PARENT_VT: u32 = DATA + 0x2600;
    pub(crate) const OC930_EV: u32 = DATA + 0x2800;
    pub(crate) const OC930_EV_VT: u32 = DATA + 0x2a00;
    pub(crate) const OC930_OBJ: u32 = DATA + 0x3000;
    pub(crate) const OC930_OBJ_VT: u32 = DATA + 0x3200;
    pub(crate) const OC930_SND: u32 = DATA + 0x3400;
    pub(crate) const OC930_SND_VT: u32 = DATA + 0x3600;
    pub(crate) const OC930_WORLD: u32 = DATA + 0x3800;
    pub(crate) const OC930_POPUP: u32 = DATA + 0x3a00;
    pub(crate) const OC930_SCREEN: u32 = DATA + 0x3c00;
    pub(crate) const OC930_TEXT: u32 = DATA + 0x3e00;
    pub(crate) const OC930_TEXT_VT: u32 = DATA + 0x4000;
    pub(crate) const OC930_OUT: u32 = DATA + 0x4200;

    // Fake slot indexes (the call counter / mailbox of each faked callee).
    const FK_MOVE_LEFT: u32 = 0;
    const FK_MOVE_RIGHT: u32 = 1;
    const FK_MOVE_UP: u32 = 2;
    const FK_MOVE_DOWN: u32 = 3;
    const FK_INSERT_CHAR: u32 = 4;
    const FK_HANDLE_KEY_CHAR: u32 = 5;
    const FK_TYPE_CURRENT_KEY: u32 = 6;
    const FK_REFRESH_ENTRY: u32 = 7;
    const FK_REFRESH_KEYS: u32 = 8;
    const FK_SET_KEY_ENABLED: u32 = 9;
    const FK_IS_KEY_ENABLED: u32 = 10;
    const FK_QUICK_MESSAGE: u32 = 11;
    const FK_HELP_CTOR: u32 = 12;
    const FK_HELP_BUTTON: u32 = 13;
    const FK_HELP_NUM_BUTTONS: u32 = 14;
    const FK_HELP_BODY: u32 = 15;
    const FK_ALLOC: u32 = 16;
    const FK_FIND_OBJECT: u32 = 17;
    const FK_CLOSE_TOP: u32 = 18;
    const FK_SET_ASCII: u32 = 19;
    const FK_PRESS_KEY: u32 = 20;

    /// A fake callee: logs the call like `fakes::fake`, then (unless it
    /// unwound) returns `mail - 1` when the fixture put `mail` (non-zero) in
    /// the mailbox of this fake for this call number (16 calls, then wraps).
    fn oc930_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC930_CTRL + 0x20 + 4 * idx);
        c.bus.write32(OC930_CTRL + 0x20 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC930_CTRL + 0x100 + 0x40 * idx + 4 * (n & 15));
        fakes::fake(c, addr, ra);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc930_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC930_CTRL) != OC930_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc930_fake_call(c, $addr, $idx);
                }
            )*
            pub(crate) fn oc930_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc930_fakes!(
        (
            oc930_fake_move_left,
            OC930_PREV_MOVE_LEFT,
            ee::s04::OVERRIDE_001CB500,
            ee::s04::f_001cb500__generated,
            0x001c_b500,
            FK_MOVE_LEFT
        ),
        (
            oc930_fake_move_right,
            OC930_PREV_MOVE_RIGHT,
            ee::s04::OVERRIDE_001CB650,
            ee::s04::f_001cb650__generated,
            0x001c_b650,
            FK_MOVE_RIGHT
        ),
        (
            oc930_fake_move_up,
            OC930_PREV_MOVE_UP,
            ee::s04::OVERRIDE_001CB7F0,
            ee::s04::f_001cb7f0__generated,
            0x001c_b7f0,
            FK_MOVE_UP
        ),
        (
            oc930_fake_move_down,
            OC930_PREV_MOVE_DOWN,
            ee::s04::OVERRIDE_001CB9B0,
            ee::s04::f_001cb9b0__generated,
            0x001c_b9b0,
            FK_MOVE_DOWN
        ),
        (
            oc930_fake_insert_char,
            OC930_PREV_INSERT_CHAR,
            ee::s04::OVERRIDE_001CC620,
            ee::s04::f_001cc620__generated,
            0x001c_c620,
            FK_INSERT_CHAR
        ),
        (
            oc930_fake_handle_key_char,
            OC930_PREV_HANDLE_KEY_CHAR,
            ee::s04::OVERRIDE_001CC6E8,
            ee::s04::f_001cc6e8__generated,
            0x001c_c6e8,
            FK_HANDLE_KEY_CHAR
        ),
        (
            oc930_fake_type_current_key,
            OC930_PREV_TYPE_CURRENT_KEY,
            ee::s04::OVERRIDE_001CC7C8,
            ee::s04::f_001cc7c8__generated,
            0x001c_c7c8,
            FK_TYPE_CURRENT_KEY
        ),
        (
            oc930_fake_refresh_entry,
            OC930_PREV_REFRESH_ENTRY,
            ee::s04::OVERRIDE_001CC848,
            ee::s04::f_001cc848__generated,
            0x001c_c848,
            FK_REFRESH_ENTRY
        ),
        (
            oc930_fake_refresh_keys,
            OC930_PREV_REFRESH_KEYS,
            ee::s04::OVERRIDE_001CD708,
            ee::s04::f_001cd708__generated,
            0x001c_d708,
            FK_REFRESH_KEYS
        ),
        (
            oc930_fake_set_key_enabled,
            OC930_PREV_SET_KEY_ENABLED,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8,
            FK_SET_KEY_ENABLED
        ),
        (
            oc930_fake_is_key_enabled,
            OC930_PREV_IS_KEY_ENABLED,
            ee::s04::OVERRIDE_001CE560,
            ee::s04::f_001ce560__generated,
            0x001c_e560,
            FK_IS_KEY_ENABLED
        ),
        (
            oc930_fake_quick_message,
            OC930_PREV_QUICK_MESSAGE,
            ee::s04::OVERRIDE_001CE5E0,
            ee::s04::f_001ce5e0__generated,
            0x001c_e5e0,
            FK_QUICK_MESSAGE
        ),
        (
            oc930_fake_help_ctor,
            OC930_PREV_HELP_CTOR,
            ee::s04::OVERRIDE_001D9D48,
            ee::s04::f_001d9d48__generated,
            0x001d_9d48,
            FK_HELP_CTOR
        ),
        (
            oc930_fake_help_button,
            OC930_PREV_HELP_BUTTON,
            ee::s04::OVERRIDE_001DA510,
            ee::s04::f_001da510__generated,
            0x001d_a510,
            FK_HELP_BUTTON
        ),
        (
            oc930_fake_help_num_buttons,
            OC930_PREV_HELP_NUM_BUTTONS,
            ee::s04::OVERRIDE_001DA528,
            ee::s04::f_001da528__generated,
            0x001d_a528,
            FK_HELP_NUM_BUTTONS
        ),
        (
            oc930_fake_help_body,
            OC930_PREV_HELP_BODY,
            ee::s04::OVERRIDE_001DA530,
            ee::s04::f_001da530__generated,
            0x001d_a530,
            FK_HELP_BODY
        ),
        (
            oc930_fake_alloc,
            OC930_PREV_ALLOC,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            FK_ALLOC
        ),
        (
            oc930_fake_find_object,
            OC930_PREV_FIND_OBJECT,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            FK_FIND_OBJECT
        ),
        (
            oc930_fake_close_top,
            OC930_PREV_CLOSE_TOP,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190,
            FK_CLOSE_TOP
        ),
        (
            oc930_fake_set_ascii,
            OC930_PREV_SET_ASCII,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70,
            FK_SET_ASCII
        ),
        (
            oc930_fake_press_key,
            OC930_PREV_PRESS_KEY,
            ee::s04::OVERRIDE_001CC1E0,
            ee::s04::f_001cc1e0__generated,
            0x001c_c1e0,
            FK_PRESS_KEY
        ),
    );

    /// Mailbox word of fake `idx` for call number `n`: the fake returns `value`.
    fn oc930_mail(c: &mut Cpu, idx: u32, n: u32, value: u32) {
        c.bus
            .write32(OC930_CTRL + 0x100 + 0x40 * idx + 4 * n, value + 1);
    }

    /// The `miss` hook: the input-event vtable slots (`OC930_EV_FAKE + 0x10 *
    /// code`) return the fixture's per-code result, everything else is the
    /// plain logging fake.
    fn oc930_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc == ra && (OC930_EV_FAKE..OC930_EV_FAKE + 0x400).contains(&target) {
            let code = (target - OC930_EV_FAKE) / 0x10;
            let res = c.bus.read32(OC930_EV_RES + 4 * code);
            c.g[V0][0] = res as u64;
        }
    }

    /// A vtable at `vt` whose slots `offs` hold `{delta, FAKE + 0x10 * i}`.
    fn oc930_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, offs: &[u32]) {
        for (i, &off) in offs.iter().enumerate() {
            let delta = (r.next_u32() % 8) * 4;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// A machine with the game image (jump tables, strings, constants), the
    /// fakes' gate on and the event-aware miss hook.
    fn oc930_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc930_install_fakes();
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        c.hooks.miss = oc930_miss;
        c.bus.write32(OC930_CTRL, OC930_MAGIC);
        r
    }

    /// The popup object with its owner (sound object, empty state list), the
    /// parent (vtable slot 0x98), the screen and the world flags.
    fn oc930_popup(c: &mut Cpu, r: &mut Rng, seed: u32, key: u32) {
        let s = OC930_SELF;
        c.bus.write32(s + 0x10, OC930_OWNER);
        c.bus.write32(s + 0x20, OC930_PARENT);
        c.bus.write32(s + 0x40, OC930_SCREEN);
        c.bus.write32(s + 0x140, key);
        c.bus.write32(OC930_PARENT + 8, OC930_PARENT_VT);
        oc930_vtable(c, r, OC930_PARENT_VT, &[0x98]);
        // The owner's state list, as `UIList_Reset` leaves it (empty).
        let l = OC930_OWNER + 0x18;
        c.bus.write32(l, l);
        c.bus.write32(l + 4, l + 0xc);
        c.bus.write32(l + 0xc, l);
        c.bus.write32(l + 0x10, l + 0xc);
        // The feedback sound object: the owner's (+0x14), or the game's own
        // at gp+0x2968 when the owner has none.
        c.bus.write32(
            OC930_OWNER + 0x14,
            if seed % 4 == 0 { 0 } else { OC930_SND },
        );
        c.bus.write32(OC930_SND, OC930_SND_VT);
        c.bus.write32(GAME_GP + 0x2968, OC930_SND_VT);
        oc930_vtable(c, r, OC930_SND_VT, &[0x10]);
        // The found key widget: vtable slot 0x58 (set colour).
        c.bus.write32(OC930_OBJ + 8, OC930_OBJ_VT);
        oc930_vtable(c, r, OC930_OBJ_VT, &[0x58]);
        c.bus.write32(OC930_WORLD + 0x84, seed % 2);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC930_WORLD);
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_GetKeyWidgetName
    // ---------------------------------------------------------------------

    /// Every key 0..0x60 (special keys by table, digit keys through
    /// `sprintf`, characters from the key character table, `*` keys that
    /// return 0, keys past the table) with and without the shifted prefix.
    #[test]
    fn c_keyboard_popup_get_key_widget_name_matches() {
        check(
            ee::s04::f_001cd0d8__generated,
            c_keyboard_popup_get_key_widget_name,
            194,
            &|c, seed| {
                let _ = machine(c, seed, 0x1cd0d8, 0);
                elf_image(c);
                c.pc = 0x001c_d0d8;
                c.g[A0][0] = sx(OC930_SELF);
                c.g[A1][0] = (seed % 97) as u64;
                c.g[A2][0] = sx(OC930_OUT);
                c.g[A3][0] = (seed / 97) as u64;
            },
        );
    }

    /// The generated body takes many different paths over the fixtures.
    #[test]
    fn c_keyboard_popup_get_key_widget_name_paths() {
        oc930_paths(ee::s04::f_001cd0d8__generated, 194, 15, &|c, seed| {
            let _ = machine(c, seed, 0x1cd0d8, 0);
            elf_image(c);
            c.pc = 0x001c_d0d8;
            c.g[A0][0] = sx(OC930_SELF);
            c.g[A1][0] = (seed % 97) as u64;
            c.g[A2][0] = sx(OC930_OUT);
            c.g[A3][0] = (seed / 97) as u64;
        });
    }

    /// Distinct cycle totals of the generated body over `seeds` fixtures
    /// (no unwinds): at least `min` paths must be taken.
    fn oc930_paths(generated: fn(&mut Cpu), seeds: u32, min: usize, setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "only {} paths: {seen:?}", seen.len());
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_SetSpecialKeyLabel
    // ---------------------------------------------------------------------

    const OC930_LABEL_KEYS: [u32; 30] = [
        27, 42, 56, 70, 28, 55, 75, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 83, 1, 2, 3, 14, 26, 29,
        0x54, 0x60, 76, 69, 79,
    ];

    fn oc930_label_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc930_machine(c, seed, 0x1cd9a0, calls);
        let key = OC930_LABEL_KEYS[(seed % 30) as usize];
        let enabled = (seed / 30) % 2;
        oc930_mail(c, FK_IS_KEY_ENABLED, 0, enabled);
        c.bus
            .write32(OC930_SELF + 0x43c, if (seed / 60) % 2 == 0 { 1 } else { 2 });
        c.bus.write32(OC930_TEXT + 8, OC930_TEXT_VT);
        oc930_vtable(c, &mut r, OC930_TEXT_VT, &[0x28, 0x48, 0x58]);
        c.pc = 0x001c_d9a0;
        c.g[A0][0] = sx(OC930_SELF);
        c.g[A1][0] = if seed % 47 == 3 { 0 } else { sx(OC930_TEXT) };
        c.g[A2][0] = key as u64;
    }

    /// Colour (enabled or disabled), then the label of every special key by
    /// text id, the digit keys by caps state and `sprintf`; null text exits.
    #[test]
    fn c_keyboard_popup_set_special_key_label_matches() {
        check(
            ee::s04::f_001cd9a0__generated,
            c_keyboard_popup_set_special_key_label,
            240,
            &|c, seed| oc930_label_fixture(c, seed, 6),
        );
    }

    #[test]
    fn c_keyboard_popup_set_special_key_label_paths() {
        oc930_paths(ee::s04::f_001cd9a0__generated, 240, 10, &|c, seed| {
            oc930_label_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_PressKey
    // ---------------------------------------------------------------------

    const OC930_PRESS_KEYS: [u32; 32] = [
        27, 28, 69, 79, 42, 56, 70, 76, 83, 55, 75, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 30, 1, 2,
        3, 14, 0x37, 0x46, 0x54, 0x60, 0x38,
    ];

    fn oc930_press_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc930_machine(c, seed, 0x1cc1e0, calls);
        let v = seed / 32;
        let key = OC930_PRESS_KEYS[(seed % 32) as usize];
        oc930_popup(c, &mut r, seed, key);
        let s = OC930_SELF;
        // The entry text and the cursor, selection mark and end.
        let len = (v + key) % 5;
        for i in 0..len {
            c.bus.write8(s + 0x74 + i, b'a' + i as u8);
        }
        let cursor = r.next_u32() % (len + 2);
        let mark = [cursor, cursor + 1, cursor.saturating_sub(1), 3][(r.next_u32() % 4) as usize];
        c.bus.write32(s + 0x134, cursor);
        c.bus.write32(s + 0x138, mark);
        c.bus.write32(s + 0x13c, r.next_u32() % 7);
        c.bus.write32(s + 0x60, 1 + r.next_u32() % 7);
        c.bus.write32(s + 0x64, r.next_u32() % 4);
        c.bus.write32(s + 0x68, r.next_u32() & 1);
        c.bus.write32(s + 0x6c, r.next_u32() & 1);
        c.bus.write32(s + 0x58, r.next_u32() & 1);
        c.bus.write32(s + 0x5c, r.next_u32() & 1);
        c.bus.write32(s + 0x420, r.next_u32() & 1);
        // A pending accept makes the key press itself first (key 0x46).
        c.bus.write32(s + 0x54, (v % 3 == 1) as u32);
        oc930_mail(c, FK_IS_KEY_ENABLED, 0, (v % 4 != 0) as u32);
        oc930_mail(c, FK_IS_KEY_ENABLED, 1, ((v / 2) % 2) as u32);
        oc930_mail(c, FK_FIND_OBJECT, 0, if v % 5 == 2 { 0 } else { OC930_OBJ });
        c.pc = 0x001c_c1e0;
        c.g[A0][0] = sx(s);
        c.g[A1][0] = (seed % 13 == 0) as u64;
    }

    /// Every key action of the keyboard (backspace, caps, shift, clear, left,
    /// right, accept, quick messages, typed characters), with the key
    /// enabled or not, a pending accept, and the entry-text refresh.
    #[test]
    fn c_keyboard_popup_press_key_matches() {
        check(
            ee::s04::f_001cc1e0__generated,
            c_keyboard_popup_press_key,
            320,
            &|c, seed| oc930_press_fixture(c, seed, 12),
        );
    }

    #[test]
    fn c_keyboard_popup_press_key_paths() {
        oc930_paths(ee::s04::f_001cc1e0__generated, 320, 20, &|c, seed| {
            oc930_press_fixture(c, seed, 0)
        });
    }

    // ---------------------------------------------------------------------
    // cKeyboardPopup_HandleInput
    // ---------------------------------------------------------------------

    /// `switch` values of event 42 (a PC key): the mapped keys, the F-key
    /// range, and unmapped values (default: typed characters).
    const OC930_PC_KEYS: [u32; 30] = [
        8, 0xd, 0x10, 0x14, 0x1b, 0x25, 0x27, 0x26, 0x28, 9, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75,
        0x76, 0x77, 0x78, 0x79, 0, 0x7a, 0x30, 0x7f, 0x41, 0x41, 0x41, 0x41, 0x41, 0x41,
    ];

    fn oc930_input_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = oc930_machine(c, seed, 0x1cdb98, calls);
        let scn = seed % 44;
        let v = seed / 44;
        let key = r.next_u32() % 0x54;
        oc930_popup(c, &mut r, seed, key);
        let s = OC930_SELF;
        c.bus.write32(s + 0x68, r.next_u32() & 1);
        c.bus.write32(s + 0x6c, r.next_u32() & 1);
        // The event: a vtable with one slot per event code (offset 8 * code).
        c.bus.write32(OC930_EV + 8, OC930_EV_VT);
        let codes = [8u32, 19, 20, 17, 18, 5, 7, 6, 25, 27, 41, 42, 43];
        for &code in &codes {
            let delta = (r.next_u32() % 4) * 4;
            c.bus.write16(OC930_EV_VT + 8 * code, delta as u16);
            c.bus
                .write32(OC930_EV_VT + 8 * code + 4, OC930_EV_FAKE + 0x10 * code);
        }
        let mut res = [0u32; 64];
        let mut isonflag = 0;
        match scn {
            0 => res[8] = 1,
            1 => res[19] = 1,
            2 => res[20] = 1,
            3 => res[17] = 1,
            4 => res[18] = 1,
            5 => res[5] = 1,
            6..=9 => {
                // Help: the isOn flag (gp-0x1680) and the world's +0x84 word.
                res[7] = 1;
                isonflag = (scn - 6) & 1;
                c.bus.write32(OC930_WORLD + 0x84, (scn - 6) >> 1);
            }
            10 => res[6] = 1,
            11 => res[25] = 1,
            12 => res[27] = 1,
            13 => {}
            _ => {
                res[41] = 1;
                res[42] = OC930_PC_KEYS[(scn - 14) as usize];
                res[43] = [0x41u32, 0x60, 0x1f, 0x80, 0x7e, 0x2a][((v + scn) % 6) as usize];
            }
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x1680), isonflag);
        for (i, w) in res.iter().enumerate() {
            c.bus.write32(OC930_EV_RES + 4 * i as u32, *w);
        }
        // Callee results: IsKeyEnabled by call number, HandleKeyChar, the
        // help popup object, the found key widget (before / after).
        for n in 0..4 {
            oc930_mail(c, FK_IS_KEY_ENABLED, n, (v >> n) & 1);
        }
        oc930_mail(c, FK_HANDLE_KEY_CHAR, 0, (v >> 1) & 1);
        oc930_mail(c, FK_HELP_CTOR, 0, OC930_POPUP);
        oc930_mail(c, FK_FIND_OBJECT, 0, if v % 2 == 0 { OC930_OBJ } else { 0 });
        oc930_mail(c, FK_FIND_OBJECT, 1, if v % 3 == 0 { 0 } else { OC930_OBJ });
        c.pc = 0x001c_db98;
        c.g[A0][0] = sx(s);
        c.g[A1][0] = sx(OC930_EV);
    }

    /// Cursor moves, press, help popup, close, modifier toggles, PC keys
    /// (mapped, F-keys, typed characters), with and without the owner's
    /// sound object; returns 1.
    #[test]
    fn c_keyboard_popup_handle_input_matches() {
        check(
            ee::s04::f_001cdb98__generated,
            c_keyboard_popup_handle_input,
            264,
            &|c, seed| oc930_input_fixture(c, seed, 24),
        );
    }

    #[test]
    fn c_keyboard_popup_handle_input_paths() {
        oc930_paths(ee::s04::f_001cdb98__generated, 264, 25, &|c, seed| {
            oc930_input_fixture(c, seed, 0)
        });
    }
}
