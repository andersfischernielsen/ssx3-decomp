//! Staging module oc854 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc854`.

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
        0x0019_77d0,
        cfe_state_request_line_on_widget_notify,
        "cFEStateRequestLine_OnWidgetNotify",
    ),
    (
        0x0019_7ca0,
        cfe_state_request_line_update_song_icons,
        "cFEStateRequestLine_UpdateSongIcons",
    ),
    // Already hand-written in festateaudiooptions.rs (not installed there);
    // tested here against the generated body.
    (
        0x0019_7db8,
        crate::game::fe::festateaudiooptions::cfe_request_line_update_scroll_arrows,
        "cFEStateRequestLine_UpdateScrollArrows",
    ),
    (
        0x0019_7e70,
        cfe_state_request_line_update_now_playing,
        "cFEStateRequestLine_UpdateNowPlaying",
    ),
    (
        0x0019_81f8,
        cfe_state_request_line_update_help_text,
        "cFEStateRequestLine_UpdateHelpText",
    ),
    (
        0x0019_8340,
        cfe_state_request_line_update_buttons,
        "cFEStateRequestLine_UpdateButtons",
    ),
];

/// `int cFEStateRequestLine_OnWidgetNotify(self, void*, int msg, int k)` (0x1977d0): the jukebox request list widget messages: 6 asks whether row `k` may be toggled (0x10 refuse, 0x101 ok), 8 plays/queues the song, 4 scrolls the list and refreshes it, 9 returns 0.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_1979c8: {
            'b_1979c4: {
                'b_1979c0: {
                    'b_1979b4: {
                        'b_197998: {
                            'b_197960: {
                                'b_197958: {
                                    'b_197910: {
                                        'b_197908: {
                                            'b_197824: {
                                                c.cycles += 7;
                                                c.g[SP][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                                                c.g[V0][0] = 0x6;
                                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.g[S0][0] = c.g[A0][0];
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                                                if t {
                                                    break 'b_197824;
                                                } else {
                                                    'b_197808: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x4;
                                                        if t {
                                                            break 'b_197808;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = c.g[A2][0] == c.g[V0][0];
                                                            c.g[V0][0] = 0x101;
                                                            if t {
                                                                break 'b_1979b4;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_1979c8;
                                                                } else {
                                                                    break 'b_197808;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x8;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.g[V0][0] = 0x9;
                                                    if t {
                                                        break 'b_197910;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0x101;
                                                        if t {
                                                            break 'b_197908;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_1979c8;
                                                            } else {
                                                                break 'b_197824;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_19788c: {
                                                c.cycles += 15;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                                                c.g[A0][0] = 0x1;
                                                c.g[V1][0] =
                                                    c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A3][0] as u32)));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                c.g[A0][0] = c.g[A0][0] << (c.g[A1][0] & 63);
                                                c.g[A0][0] = !(0u64 | c.g[A0][0]);
                                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
                                                c.g[V1][0] = c.g[V1][0] << 32;
                                                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                                                if t {
                                                    break 'b_19788c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x10;
                                                    if t {
                                                        break 'b_1979c4;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[V1][0] = c
                                                            .ld((c.g[S0][0] as u32)
                                                                .wrapping_add(0x160));
                                                        c.g[V1][0] =
                                                            c.g[V1][0] >> (c.g[A1][0] & 63);
                                                        c.g[V1][0] = c.g[V1][0] & 0x1;
                                                        c.g[V1][0] = c.g[V1][0] << 32;
                                                        c.g[V1][0] =
                                                            ((c.g[V1][0] as i64) >> 32) as u64;
                                                        let t = c.g[V1][0] == 0u64;
                                                        c.g[V0][0] = 0x101;
                                                        if t {
                                                            break 'b_1979c4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V0][0] = 0x10;
                                                            if t {
                                                                break 'b_1979c4;
                                                            } else {
                                                                break 'b_19788c;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            'b_1978b8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1978b8;
                                                } else {
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        c.ld(
                                                            (c.g[S0][0] as u32).wrapping_add(0x160)
                                                        );
                                                    c.g[V0][0] = c.g[V0][0] >> (c.g[A1][0] & 63);
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[V0][0] = c.g[V0][0] << 32;
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x101;
                                                    if t {
                                                        break 'b_197958;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_1979c8;
                                                        } else {
                                                            break 'b_1978b8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[RA][0] = sx(0x001978c0);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x00198ae8, 0x001978c0)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                                            c.g[V1][0] =
                                                ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A2][0] = 0u64;
                                            if t {
                                                break 'b_1979c0;
                                            } else {
                                                c.cycles += 2;
                                                c.g[A0][0] =
                                                    c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                                c.g[A1][0] = 0u64;
                                                'l_1978d8: loop {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = c.g[A0][0] >> (c.g[A1][0] & 63);
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                    c.g[V1][0] =
                                                        ((c.g[A1][0] as i64) < 64i64) as u64;
                                                    let t = c.g[V1][0] != 0u64;
                                                    c.g[A2][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A2][0] as u32)));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001978d8);
                                                        }
                                                        continue 'l_1978d8;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            ((c.g[A2][0] as i64) < 6i64) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x101;
                                                        if t {
                                                            break 'b_197958;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_1979c8;
                                                            } else {
                                                                break 'b_197908;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1979c4;
                                        } else {
                                            break 'b_197910;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14c));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1979c0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        if t {
                                            break 'b_197998;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                            c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] << 32;
                                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                            let t = c.g[V0][0] != 0u64;

                                            if t {
                                                break 'b_197960;
                                            } else {
                                                break 'b_197958;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x10;
                                if t {
                                    break 'b_1979c4;
                                } else {
                                    break 'b_197960;
                                }
                            }
                            c.cycles += 14;
                            c.g[RA][0] = sx(0x00197968);
                            call_ptr(c, 0x0028b180, 0x00197968)?;
                            c.g[RA][0] = sx(0x00197970);
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x508), c.g[S1][0]);
                            call_ptr(c, 0x0028b180, 0x00197970)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0xffffffffffffffff;
                            c.g[T0][0] = 0x1;
                            c.g[RA][0] = sx(0x00197988);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0028cf98, 0x00197988)?;
                            c.g[RA][0] = sx(0x00197990);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x00197e70,
                                f: ee::s02::cFEStateRequestLine_UpdateNowPlaying,
                            }
                            .call(c, 0x00197990)?;
                            let t = true;
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_1979c4;
                            } else {
                                break 'b_197998;
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001979a0);
                        call_ptr(c, 0x0028b180, 0x001979a0)?;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                        c.g[RA][0] = sx(0x001979ac);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x002b4978, 0x001979ac)?;
                        let t = true;
                        c.g[V0][0] = 0x101;
                        if t {
                            break 'b_1979c4;
                        } else {
                            break 'b_1979b4;
                        }
                    }
                    c.cycles += 3;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A3][0]);
                    c.g[RA][0] = sx(0x001979c0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00197b88,
                        f: ee::s02::cFEStateRequestLine_RefreshList,
                    }
                    .call(c, 0x001979c0)?;
                    break 'b_1979c0;
                }
                c.cycles += 1;
                c.g[V0][0] = 0x101;
                break 'b_1979c4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1979c8;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRequestLine_UpdateSongIcons(self)` (0x197ca0): hide the eight check-mark icons, then show and set the icon (check box, empty box or dollar sign) of every visible song row.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_song_icons(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S3][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S4][0] = 0xffffffffffffffff;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4c));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        'l_197cd8: loop {
            'b_197d84: {
                c.cycles += 13;
                c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00197cf4);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00197cf4)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14c));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                if t {
                    break 'b_197d84;
                } else {
                    'b_197d7c: {
                        'b_197d58: {
                            c.cycles += 16;
                            c.g[A3][0] = c.lw((c.g[S2][0] as u32));
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A1][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[S1][0] = c.lw((c.g[V0][0] as u32));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x00197d30);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00197d30)?;
                            c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                            c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                break 'b_197d58;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x204));
                                if t {
                                    break 'b_197d7c;
                                } else {
                                    break 'b_197d58;
                                }
                            }
                        }
                        'b_197d78: {
                            c.cycles += 6;
                            c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                            if t {
                                break 'b_197d78;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x208));
                                if t {
                                    break 'b_197d7c;
                                } else {
                                    break 'b_197d78;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20c));
                        break 'b_197d7c;
                    }
                    c.cycles += 2;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), c.g[S4][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[V1][0]);
                    break 'b_197d84;
                }
            }
            c.cycles += 4;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 8i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00197cd8);
                }
                continue 'l_197cd8;
            } else {
                c.cycles += 9;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateRequestLine_UpdateNowPlaying(self)` (0x197e70): format the "now playing" title and artist lines from the music system, hiding the lines when the text is empty.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_now_playing(c: &mut Cpu) {
    guest(c, |c| {
        'b_198100: {
            'b_1980fc: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                let t = c.g[V0][0] == 0u64;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1980fc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_198100;
                    } else {
                        'b_1980c4: {
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x00197eb0);
                            call_ptr(c, 0x0028b180, 0x00197eb0)?;
                            c.g[RA][0] = sx(0x00197eb8);
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x508));
                            call_ptr(c, 0x0028b180, 0x00197eb8)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
                            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V1][0] == 0u64 {
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                break 'b_1980c4;
                            } else {
                                'b_197fd8: {
                                    'b_197fbc: {
                                        c.cycles += 15;
                                        c.g[RA][0] = sx(0x00197ed0);
                                        call_ptr(c, 0x0028b180, 0x00197ed0)?;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                        c.g[RA][0] = sx(0x00197edc);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x002b40f0, 0x00197edc)?;
                                        c.g[RA][0] = sx(0x00197ee4);
                                        c.g[S3][0] = c.g[V0][0];
                                        call_ptr(c, 0x0028b180, 0x00197ee4)?;
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                                        c.g[RA][0] = sx(0x00197ef0);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x002b4120, 0x00197ef0)?;
                                        c.g[S4][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00197efc);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x00416810, 0x00197efc)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = 0x1;
                                        if t {
                                            break 'b_197fbc;
                                        } else {
                                            c.cycles += 42;
                                            c.g[A2][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00197f1c);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x00197f1c)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x668));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x00197f3c);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x00197f3c)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A1][0] = c.g[V0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00197f4c);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, t, 0x00197f4c)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00197f58);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x003dbca0, 0x00197f58)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00197f64);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x00416810, 0x00197f64)?;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A1][0] = 0x460000;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x470));
                                            c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                                            c.g[A2][0] = 0x100;
                                            c.g[RA][0] = sx(0x00197f84);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317e30, 0x00197f84)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x00197f98);
                                            c.g[A2][0] = c.g[S3][0];
                                            call_ptr(c, 0x002c26d0, 0x00197f98)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                            c.g[RA][0] = sx(0x00197fa4);
                                            c.g[A1][0] = c.g[S0][0];
                                            call_ptr(c, 0x003a0e90, 0x00197fa4)?;
                                            let t = c.g[S0][0] == 0u64;

                                            if t {
                                                break 'b_197fd8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x00197fb4);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, 0x00317e98, 0x00197fb4)?;
                                                let t = true;

                                                if t {
                                                    break 'b_197fd8;
                                                } else {
                                                    break 'b_197fbc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b4));
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00197fd8);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x00197fd8)?;
                                    break 'b_197fd8;
                                }
                                'b_1980a0: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00197fe0);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x00416810, 0x00197fe0)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1980a0;
                                    } else {
                                        c.cycles += 42;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00198000);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x00198000)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x678));
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x00198020);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        call_ptr(c, 0x00317670, 0x00198020)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A1][0] = c.g[V0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00198030);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, t, 0x00198030)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x0019803c);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x003dbca0, 0x0019803c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00198048);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x00416810, 0x00198048)?;
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A1][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                                        c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                                        c.g[A2][0] = 0x100;
                                        c.g[RA][0] = sx(0x00198068);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317e30, 0x00198068)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x0019807c);
                                        c.g[A2][0] = c.g[S4][0];
                                        call_ptr(c, 0x002c26d0, 0x0019807c)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                                        c.g[RA][0] = sx(0x00198088);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a0e90, 0x00198088)?;
                                        if c.g[S0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            break 'b_1980fc;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x00198098);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x00317e98, 0x00198098)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1980fc;
                                            } else {
                                                break 'b_1980a0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 9;
                                c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                                c.g[A1][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001980bc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001980bc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1980fc;
                                } else {
                                    break 'b_1980c4;
                                }
                            }
                        }
                        c.cycles += 14;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001980dc);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001980dc)?;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1b8));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001980f8);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001980f8)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1980fc;
                    }
                }
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_198100;
        }
        c.cycles += 6;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRequestLine_UpdateHelpText(self, int idx)` (0x1981f8): choose the help string for song `idx` (owned or not, in the store or not).
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_help_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_198334: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_198334;
            } else {
                'b_19831c: {
                    'b_198300: {
                        'b_198268: {
                            c.cycles += 7;
                            c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                            c.g[V0][0] = c.g[V0][0] >> (c.g[A1][0] & 63);
                            c.g[V0][0] = c.g[V0][0] & 0x1;
                            c.g[V0][0] = c.g[V0][0] << 32;
                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x1;
                            if t {
                                break 'b_198268;
                            } else {
                                'b_198260: {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                                    c.g[V1][0] = c.g[V1][0] << (c.g[A1][0] & 63);
                                    c.g[V1][0] = !(0u64 | c.g[V1][0]);
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                    c.g[V0][0] = c.g[V0][0] << 32;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_198260;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b0));
                                        if t {
                                            break 'b_198300;
                                        } else {
                                            break 'b_198260;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                                if t {
                                    break 'b_198300;
                                } else {
                                    break 'b_198268;
                                }
                            }
                        }
                        'b_1982d4: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_1982d4;
                            } else {
                                'b_1982cc: {
                                    'b_1982c0: {
                                        c.cycles += 6;
                                        c.g[RA][0] = sx(0x0019827c);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x00198ae8, 0x0019827c)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                                        c.g[V1][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[A1][0] = 0u64;
                                        if t {
                                            break 'b_1982c0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A2][0] =
                                                c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                            c.g[A0][0] = 0u64;
                                            'l_198298: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                                                let t = c.g[V1][0] != 0u64;
                                                c.g[A1][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00198298);
                                                    }
                                                    continue 'l_198298;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 6i64) as u64;
                                                    if c.g[V0][0] == 0u64 {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1982cc;
                                                    } else {
                                                        break 'b_1982c0;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e0));
                                    if t {
                                        break 'b_198300;
                                    } else {
                                        break 'b_1982cc;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f8));
                                if t {
                                    break 'b_198300;
                                } else {
                                    break 'b_1982d4;
                                }
                            }
                        }
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x001982dc);
                        call_ptr(c, 0x0014dc80, 0x001982dc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001982e8);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001982e8)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_19831c;
                        } else {
                            c.cycles += 2;
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x710));
                            break 'b_198300;
                        }
                    }
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00198308);
                    call_ptr(c, 0x00317670, 0x00198308)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                    c.g[RA][0] = sx(0x00198314);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x00198314)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_198334;
                    } else {
                        break 'b_19831c;
                    }
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x00198324);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x720));
                call_ptr(c, 0x00317670, 0x00198324)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1e8));
                c.g[RA][0] = sx(0x00198330);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x00198330)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                break 'b_198334;
            }
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRequestLine_UpdateButtons(self, int idx)` (0x198340): set the add/remove/buy button caption for song `idx` and the visibility of the button icons.
#[allow(
    unused_labels,
    unused_assignments,
    unreachable_code,
    unused_parens,
    unused_mut
)]
pub fn cfe_state_request_line_update_buttons(c: &mut Cpu) {
    guest(c, |c| {
        'b_1984f8: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            let t = c.g[A0][0] == 0u64;
            c.g[S1][0] = c.g[A1][0];
            if t {
                break 'b_1984f8;
            } else {
                'b_1984bc: {
                    'b_19844c: {
                        c.cycles += 25;
                        c.g[A1][0] = 0x4a0000;
                        c.g[S2][0] = 0u64;
                        c.g[RA][0] = sx(0x00198374);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                        call_ptr(c, 0x003a0c50, 0x00198374)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00198390);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00198390)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001983ac);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001983ac)?;
                        c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                        c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[V0][0] = c.g[V0][0] << 32;
                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0x1;
                        if t {
                            break 'b_19844c;
                        } else {
                            'b_198430: {
                                'b_198424: {
                                    'b_198410: {
                                        c.cycles += 10;
                                        c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x158));
                                        c.g[V1][0] = c.g[V1][0] << (c.g[S1][0] & 63);
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V1][0] = !(0u64 | c.g[V1][0]);
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                                        c.g[V0][0] = c.g[V0][0] << 32;
                                        c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = c.g[A0][0] >> (c.g[S1][0] & 63);
                                        if t {
                                            break 'b_198410;
                                        } else {
                                            c.cycles += 5;
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[V0][0] = c.g[V0][0] << 32;
                                            c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_198424;
                                            } else {
                                                c.cycles += 3;
                                                c.g[S2][0] = 0x1;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x738));
                                                if t {
                                                    break 'b_198430;
                                                } else {
                                                    break 'b_198410;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    c.g[V0][0] = c.g[V0][0] << 32;
                                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1984bc;
                                    } else {
                                        break 'b_198424;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x460000;
                                c.g[S2][0] = 0x1;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x748));
                                break 'b_198430;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x00198438);
                            call_ptr(c, 0x00317670, 0x00198438)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                            c.g[RA][0] = sx(0x00198444);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00198444)?;
                            let t = true;

                            if t {
                                break 'b_1984bc;
                            } else {
                                break 'b_19844c;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x16c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_1984bc;
                    } else {
                        'b_1984a0: {
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x00198460);
                            c.g[A1][0] = c.g[S1][0];
                            Callee {
                                addr: 0x00198ae8,
                                f: ee::s03::cFEStateRequestLine_GetSongPrice,
                            }
                            .call(c, 0x00198460)?;
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x168));
                            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1984a0;
                            } else {
                                c.cycles += 2;
                                c.g[A2][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                                c.g[A0][0] = 0u64;
                                'l_198478: loop {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                                    let t = c.g[V1][0] != 0u64;
                                    c.g[A1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00198478);
                                        }
                                        continue 'l_198478;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[A1][0] as i64) < 6i64) as u64;
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_1984bc;
                                        } else {
                                            break 'b_1984a0;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A0][0] = 0x460000;
                        c.g[S2][0] = 0x1;
                        c.g[RA][0] = sx(0x001984b0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x758));
                        call_ptr(c, 0x00317670, 0x001984b0)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                        c.g[RA][0] = sx(0x001984bc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001984bc)?;
                        break 'b_1984bc;
                    }
                }
                c.cycles += 2;
                let t = c.g[S2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1984f8;
                } else {
                    c.cycles += 13;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001984dc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001984dc)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001984f8);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001984f8)?;
                    break 'b_1984f8;
                }
            }
        }
        'b_198598: {
            'b_198560: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                let t = c.g[V1][0] == 0u64;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1fc));
                if t {
                    break 'b_198560;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.ld((c.g[S0][0] as u32).wrapping_add(0x160));
                    c.g[V0][0] = c.g[V0][0] >> (c.g[S1][0] & 63);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.g[V0][0] = c.g[V0][0] << 32;
                    c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_198560;
                    } else {
                        c.cycles += 15;
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x0019853c);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0019853c)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00198558);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00198558)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_198598;
                        } else {
                            break 'b_198560;
                        }
                    }
                }
            }
            c.cycles += 14;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00198578);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00198578)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00198594);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00198594)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_198598;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::sync::atomic::AtomicUsize;

    // The request line object (`cFEStateRequestLine`) and what it points at.
    const OC854_SELF: u32 = DATA + 0x400;
    const OC854_WORLD: u32 = DATA + 0x1800;
    const OC854_LOCALE: u32 = DATA + 0x1900;
    const OC854_LOCALE_VT: u32 = DATA + 0x1a00;
    const OC854_WIDGET_VT: u32 = DATA + 0x1b00;
    const OC854_BE: u32 = DATA + 0x1c00;
    const OC854_WIDGETS: u32 = DATA + 0x2000;
    const OC854_AUDIO: u32 = DATA + 0x3000;
    const OC854_NAME: u32 = DATA + 0x3400;
    const OC854_ARTIST: u32 = DATA + 0x3500;
    // Fakes are active only while this word of the machine's RAM holds the magic.
    const OC854_CTRL: u32 = DATA + 0x20_0000;
    const OC854_MAGIC: u32 = 0x4f43_3835;

    // Offsets in the request line.
    const SONG_ORDER: u32 = 0x4c;
    const NUM_SONGS: u32 = 0x14c;
    const FIRST_VISIBLE: u32 = 0x150;
    const OWNED_OR_SEL: u32 = 0x158;
    const PLAYLIST: u32 = 0x160;
    const MONEY: u32 = 0x168;
    const IN_STORE: u32 = 0x16c;
    const CHECK_MARKS: u32 = 0x194;
    const TITLE: u32 = 0x1b4;
    const ARTIST: u32 = 0x1b8;
    const HELP_TEXT: u32 = 0x1e8;
    const ARROW_UP: u32 = 0x1ec;
    const ARROW_DOWN: u32 = 0x1f0;
    const BTN_TEXT: u32 = 0x1f4;
    const BTN_BACK: u32 = 0x1f8;
    const BTN_A: u32 = 0x1fc;
    const BTN_B: u32 = 0x200;
    const IMG_CHECK: u32 = 0x204;
    const IMG_EMPTY: u32 = 0x208;
    const IMG_DOLLAR: u32 = 0x20c;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Mailbox word of a faked callee: a nonzero value is returned in `v0`.
    fn mail(addr: u32) -> u32 {
        OC854_CTRL + 0x100 + (addr & 0xffff)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// mailbox value (if set) unless it unwound.
    fn oc854_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let m = c.bus.read32(mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && m != 0 {
            c.g[V0][0] = m as u64;
        }
    }

    macro_rules! oc854_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC854_CTRL) != OC854_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc854_fake_call(c, $addr);
                }
            )*
            fn oc854_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc854_fakes!(
        (
            oc854_fake_28b180,
            OC854_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc854_fake_28cf98,
            OC854_PREV_28CF98,
            ee::s07::OVERRIDE_0028CF98,
            ee::s07::f_0028cf98__generated,
            0x0028_cf98
        ),
        (
            oc854_fake_2b4978,
            OC854_PREV_2B4978,
            ee::s08::OVERRIDE_002B4978,
            ee::s08::f_002b4978__generated,
            0x002b_4978
        ),
        (
            oc854_fake_2b40f0,
            OC854_PREV_2B40F0,
            ee::s08::OVERRIDE_002B40F0,
            ee::s08::f_002b40f0__generated,
            0x002b_40f0
        ),
        (
            oc854_fake_2b4120,
            OC854_PREV_2B4120,
            ee::s08::OVERRIDE_002B4120,
            ee::s08::f_002b4120__generated,
            0x002b_4120
        ),
        (
            oc854_fake_317670,
            OC854_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc854_fake_3dbca0,
            OC854_PREV_3DBCA0,
            ee::s14::OVERRIDE_003DBCA0,
            ee::s14::f_003dbca0__generated,
            0x003d_bca0
        ),
        (
            oc854_fake_317e30,
            OC854_PREV_317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc854_fake_2c26d0,
            OC854_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc854_fake_3a0e90,
            OC854_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc854_fake_317e98,
            OC854_PREV_317E98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc854_fake_3a0c50,
            OC854_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc854_fake_3a1000,
            OC854_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc854_fake_14dc80,
            OC854_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc854_fake_14dd58,
            OC854_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc854_fake_197b88,
            OC854_PREV_197B88,
            ee::s02::OVERRIDE_00197B88,
            ee::s02::f_00197b88__generated,
            0x0019_7b88
        ),
    );

    /// A mask of song bits: `mode` picks none, only `idx`, `idx` and a
    /// neighbour, a dense or a sparse random set, or everything but `idx`.
    fn oc854_mask(r: &mut Rng, mode: u32, idx: u32) -> u64 {
        let bit = 1u64 << (idx & 63);
        let dense = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        match mode % 7 {
            0 => 0,
            1 => bit,
            2 => bit | (1u64 << ((idx + 1 + r.next_u32() % 5) & 63)),
            3 => dense,
            4 => (1u64 << (r.next_u32() & 63)) | (1u64 << (r.next_u32() & 63)),
            5 => dense & !bit,
            _ => dense | bit,
        }
    }

    /// Machine state shared by the request line tests: fakes on, the world with
    /// the locale database, the widget vtable, the music system, the names.
    fn oc854_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        oc854_install_fakes();
        w(c, OC854_CTRL, OC854_MAGIC);
        w(c, GAME_GP - 0x848, OC854_WORLD);
        let game = r.next_u32() % 2;
        w(c, OC854_WORLD + 0x84, game);
        w(c, OC854_WORLD + 0x8c, OC854_LOCALE);
        w(c, OC854_LOCALE + 4, OC854_LOCALE_VT);
        c.bus.write16(OC854_LOCALE_VT + 0x20, 0);
        w(c, OC854_LOCALE_VT + 0x24, FAKE);
        c.bus.write16(OC854_WIDGET_VT + 0x48, 0);
        w(c, OC854_WIDGET_VT + 0x4c, FAKE);
        w(c, mail(0x0028_b180), OC854_AUDIO);
        w(c, mail(0x002b_40f0), OC854_NAME);
        w(c, mail(0x002b_4120), OC854_ARTIST);
        w(c, mail(0x0014_dc80), OC854_BE);
        let cur = r.next_u32() % 6;
        w(c, OC854_AUDIO + 0x508, cur);
        w(c, OC854_AUDIO + 0x504, r.next_u32() % 6);
        for a in [OC854_NAME, OC854_ARTIST] {
            let text = if r.next_u32() % 3 == 0 {
                0
            } else {
                0x4142_4300
            };
            w(c, a, text);
        }
        // cBENewRaceInterface_newRace.cGamePath
        c.bus.write8(0x0053_5c11, (r.next_u32() % 2) as u8);
        r
    }

    /// The request line at `OC854_SELF`: counts, the song order, the song masks
    /// and every widget (`null_ok` ones are sometimes missing).
    fn oc854_self(
        c: &mut Cpu,
        r: &mut Rng,
        idx_of: &dyn Fn(&mut Cpu, u32, u32) -> u32,
        dense: bool,
    ) -> u32 {
        let s = OC854_SELF;
        let num = r.next_u32() % 21;
        let first = r.next_u32() % 13;
        w(c, s + NUM_SONGS, num);
        w(c, s + FIRST_VISIBLE, first);
        for i in 0..40 {
            let v = r.next_u32() % 64;
            w(c, s + SONG_ORDER + 4 * i, v);
        }
        let idx = idx_of(c, first, r.next_u32() % 8);
        let (m1, m2) = if dense {
            (3, 3)
        } else {
            (r.next_u32(), r.next_u32())
        };
        let owned = oc854_mask(r, m1, idx);
        let playlist = oc854_mask(r, m2, idx);
        c.bus.write64(s + OWNED_OR_SEL, owned);
        c.bus.write64(s + PLAYLIST, playlist);
        let money = [0, 0x1387, 0x1388, 0x2000, r.next_u32() % 0x3000][(r.next_u32() % 5) as usize];
        w(c, s + MONEY, money);
        w(c, s + IN_STORE, r.next_u32() % 2);
        let mut next = 0;
        let mut put = |c: &mut Cpu, off: u32, nullable: bool, r: &mut Rng| {
            let widget = OC854_WIDGETS + 0x40 * next;
            next += 1;
            w(c, widget + 8, OC854_WIDGET_VT);
            let p = if nullable && r.next_u32() % 4 == 0 {
                0
            } else {
                widget
            };
            w(c, s + off, p);
        };
        for i in 0..8 {
            put(c, CHECK_MARKS + 4 * i, false, r);
        }
        for off in [TITLE, ARTIST, HELP_TEXT, BTN_TEXT] {
            put(c, off, true, r);
        }
        for off in [
            ARROW_UP, ARROW_DOWN, BTN_BACK, BTN_A, BTN_B, IMG_CHECK, IMG_EMPTY, IMG_DOLLAR,
        ] {
            put(c, off, false, r);
        }
        idx
    }

    /// The song of row `k` of the visible list (`mSongOrder[first + k]`).
    fn oc854_row_song(c: &mut Cpu, first: u32, k: u32) -> u32 {
        let a = OC854_SELF + SONG_ORDER + 4 * (first + k);
        c.bus.read32(a)
    }

    #[test]
    fn cfe_state_request_line_update_song_icons_matches() {
        check(
            ee::s02::f_00197ca0__generated,
            cfe_state_request_line_update_song_icons,
            120,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x197ca0, 20);
                oc854_self(c, &mut r, &|_, _, _| 0, true);
                c.g[A0][0] = OC854_SELF as u64;
                c.pc = 0x0019_7ca0;
            },
        );
    }

    #[test]
    fn cfe_request_line_update_scroll_arrows_matches() {
        check(
            ee::s02::f_00197db8__generated,
            crate::game::fe::festateaudiooptions::cfe_request_line_update_scroll_arrows,
            120,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x197db8, 4);
                oc854_self(c, &mut r, &|_, _, _| 0, true);
                c.g[A0][0] = OC854_SELF as u64;
                c.pc = 0x0019_7db8;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_update_now_playing_matches() {
        check(
            ee::s02::f_00197e70__generated,
            cfe_state_request_line_update_now_playing,
            240,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x197e70, 24);
                oc854_self(c, &mut r, &|_, _, _| 0, true);
                c.g[A0][0] = OC854_SELF as u64;
                c.pc = 0x0019_7e70;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_update_help_text_matches() {
        check(
            ee::s02::f_001981f8__generated,
            cfe_state_request_line_update_help_text,
            300,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x1981f8, 6);
                let idx = oc854_self(c, &mut r, &|_, _, _| 0, false);
                let idx = (idx + r.next_u32() % 64) % 64;
                // the masks were built for idx 0: rebuild them for the real one
                let owned = oc854_mask(&mut r, seed, idx);
                let playlist = oc854_mask(&mut r, seed / 7, idx);
                c.bus.write64(OC854_SELF + OWNED_OR_SEL, owned);
                c.bus.write64(OC854_SELF + PLAYLIST, playlist);
                c.g[A0][0] = OC854_SELF as u64;
                c.g[A1][0] = idx as u64;
                c.pc = 0x0019_81f8;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_update_buttons_matches() {
        check(
            ee::s03::f_00198340__generated,
            cfe_state_request_line_update_buttons,
            300,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x198340, 12);
                let idx = oc854_self(c, &mut r, &|_, _, _| 0, false);
                let idx = (idx + r.next_u32() % 64) % 64;
                let owned = oc854_mask(&mut r, seed, idx);
                let playlist = oc854_mask(&mut r, seed / 7, idx);
                c.bus.write64(OC854_SELF + OWNED_OR_SEL, owned);
                c.bus.write64(OC854_SELF + PLAYLIST, playlist);
                c.g[A0][0] = OC854_SELF as u64;
                c.g[A1][0] = idx as u64;
                c.pc = 0x0019_8340;
            },
        );
    }

    #[test]
    fn cfe_state_request_line_on_widget_notify_matches() {
        check(
            ee::s02::f_001977d0__generated,
            cfe_state_request_line_on_widget_notify,
            400,
            &|c, seed| {
                let mut r = oc854_setup(c, seed, 0x1977d0, 10);
                let k = r.next_u32() % 8;
                let idx = oc854_self(c, &mut r, &oc854_row_song, false);
                // `idx_of` above used a random row; rebuild masks for the row `k`
                let first = c.bus.read32(OC854_SELF + FIRST_VISIBLE);
                let song = oc854_row_song(c, first, k);
                let _ = idx;
                let owned = oc854_mask(&mut r, seed, song);
                let playlist = oc854_mask(&mut r, seed / 7, song);
                c.bus.write64(OC854_SELF + OWNED_OR_SEL, owned);
                c.bus.write64(OC854_SELF + PLAYLIST, playlist);
                let msg = [6u64, 8, 9, 4, 5, 7, 6, 8][(r.next_u32() % 8) as usize];
                c.g[A0][0] = OC854_SELF as u64;
                c.g[A2][0] = msg;
                c.g[A3][0] = k as u64;
                c.pc = 0x0019_77d0;
            },
        );
    }
}
