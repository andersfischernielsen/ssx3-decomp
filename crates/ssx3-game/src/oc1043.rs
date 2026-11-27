//! Staging module oc1043 (wave wave264): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1043`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002a_ec58,
        path_plat_get_track_status,
        "PATH_PlatGetTrackStatus",
    ),
    (
        0x002b_1780,
        music_player_create_stream,
        "MusicPlayer_CreateStream",
    ),
    (0x002b_3398, music_player_dtor, "MusicPlayer_dtor"),
    (0x002c_1da8, quad_tree_compact, "QuadTree_Compact"),
    (
        0x002d_83b8,
        avalanche_write_history_file,
        "Avalanche_WriteHistoryFile",
    ),
];

/// `int PATH_PlatGetTrackStatus(int slot, int* elapsed_ms, int* remaining_ms)`
/// (0x2aec58): reads one of the 24 sound-bank monitor slots (`D_004D3E98[slot]`),
/// locks the sound system and refreshes the slot's stream state (in-memory
/// streams: position from the stream header; streaming ones: each sub-request's
/// state), stores the two out values, and returns 0, -1 (no slot) or -2 (no
/// stream handle). Straight transcription of the generated body (labels kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn path_plat_get_track_status(c: &mut Cpu) {
    guest(c, |c| {
        'b_2aef80: {
            'b_2aeca8: {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
                c.g[V0][0] = 0x4d0000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[S4][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S2][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[RA][0]);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.g[S0][0] = c.lw((c.g[A0][0] as u32));
                c.sw((c.g[S4][0] as u32), 0u64);
                let t = c.g[S0][0] != 0u64;
                c.sw((c.g[S2][0] as u32), 0u64);
                if t {
                    break 'b_2aeca8;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_2aef80;
                    } else {
                        break 'b_2aeca8;
                    }
                }
            }
            'b_2aecc4: {
                c.cycles += 5;
                c.g[V1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                c.g[S5][0] = 0xfffffffffffffff8;
                let t = (c.g[V0][0] as i64) >= 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                if t {
                    break 'b_2aecc4;
                } else {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0xfffffffffffffffe;
                    if t {
                        break 'b_2aef80;
                    } else {
                        break 'b_2aecc4;
                    }
                }
            }
            'b_2aef6c: {
                'b_2aef68: {
                    'b_2aeddc: {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x002aeccc);
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x65), c.g[V1][0]);
                        call_ptr(c, 0x003b58a0, 0x002aeccc)?;
                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x64));
                        if c.g[V0][0] == 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            break 'b_2aeddc;
                        } else {
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            if (c.g[A1][0] as i64) < 0 {
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S4][0]);
                                break 'b_2aef6c;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                let t = (c.g[V1][0] as i64) < 0;
                                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_2aef68;
                                } else {
                                    'b_2aedac: {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                        c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                        c.g[RA][0] = sx(0x002aed04);
                                        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                                        call_ptr(c, 0x003b6670, 0x002aed04)?;
                                        c.g[S5][0] = c.g[V0][0];
                                        if (c.g[S5][0] as i64) < 0 {
                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                            break 'b_2aedac;
                                        } else {
                                            'b_2aed3c: {
                                                c.cycles += 6;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                c.g[RA][0] = sx(0x002aed1c);
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                                call_ptr(c, 0x003b8ad8, 0x002aed1c)?;
                                                c.g[V1][0] = c.g[V0][0];
                                                let t = (c.g[V1][0] as i64) >= 0;
                                                c.sw((c.g[S2][0] as u32), c.g[V1][0]);
                                                if t {
                                                    break 'b_2aed3c;
                                                } else {
                                                    c.cycles += 5;
                                                    c.sw((c.g[S2][0] as u32), 0u64);
                                                    c.g[V1][0] = 0xffffffffffffffff;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_2aef68;
                                                    } else {
                                                        break 'b_2aed3c;
                                                    }
                                                }
                                            }
                                            'b_2aed7c: {
                                                'b_2aed60: {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = 0xa;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0xa0));
                                                    c.mult(0, 2, 3, 2);
                                                    let t = (c.g[A0][0] as i64) < 0;
                                                    c.sw((c.g[S2][0] as u32), c.g[V0][0]);
                                                    if t {
                                                        break 'b_2aed60;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.mtc1(4, 2);
                                                        c.cvt_s_w(2, 2);
                                                        let t = true;
                                                        c.g[V0][0] = c.lhu(
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_2aed7c;
                                                        } else {
                                                            break 'b_2aed60;
                                                        }
                                                    }
                                                }
                                                c.cycles += 7;
                                                c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                                                c.g[V0][0] = c.g[A0][0] & 0x1;
                                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                                c.mtc1(2, 2);
                                                c.cvt_s_w(2, 2);
                                                c.add_s(2, 2, 2);
                                                c.g[V0][0] =
                                                    c.lhu((c.g[SP][0] as u32).wrapping_add(0x20));
                                                break 'b_2aed7c;
                                            }
                                            c.cycles += 12;
                                            c.g[AT][0] = 0x447a0000;
                                            c.mtc1(1, 1);
                                            c.mtc1(2, 0);
                                            c.cvt_s_w(0, 0);
                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                            c.div_s(0, 2, 0);
                                            c.mul_s(0, 0, 1);
                                            c.cvt_w_s(1, 0);
                                            c.mfc1(2, 1);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_sub((c.g[V1][0] as u32)));
                                            c.sw((c.g[S4][0] as u32), c.g[V0][0]);
                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                            break 'b_2aedac;
                                        }
                                    }
                                    c.cycles += 12;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x002aedc8);
                                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                    call_ptr(c, 0x003b86c0, 0x002aedc8)?;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                                    if t {
                                        break 'b_2aef68;
                                    } else {
                                        break 'b_2aeddc;
                                    }
                                }
                            }
                        }
                    }
                    'b_2aee00: {
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x002aede4);
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003b7b48, 0x002aede4)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
                        c.g[S5][0] = c.g[V0][0];
                        c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                        if c.g[V0][0] == 0u64 {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                            break 'b_2aee00;
                        } else {
                            break 'b_2aee00;
                        }
                    }
                    'b_2aef5c: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
                        let t = (c.g[V0][0] as i64) <= 0;
                        c.g[S3][0] = 0u64;
                        if t {
                            break 'b_2aef5c;
                        } else {
                            c.cycles += 1;
                            c.g[S1][0] = 0u64;
                            'l_2aee10: loop {
                                'b_2aef4c: {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                    c.g[V1][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    if (c.g[A0][0] as i64) < 0 {
                                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
                                        break 'b_2aef4c;
                                    } else {
                                        'b_2aef48: {
                                            'b_2aef3c: {
                                                'b_2aeeb8: {
                                                    'b_2aee70: {
                                                        c.cycles += 6;
                                                        c.g[RA][0] = sx(0x002aee38);
                                                        c.g[A1][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        call_ptr(c, 0x003b7c40, 0x002aee38)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V0][0] = 0x1;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[V0][0] =
                                                            ((c.g[V1][0] as i64) < 2i64) as u64;
                                                        if t {
                                                            break 'b_2aee70;
                                                        } else {
                                                            'b_2aee60: {
                                                                c.cycles += 2;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = 0x2;
                                                                if t {
                                                                    break 'b_2aee60;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = c.g[V1][0] == 0u64;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x14));
                                                                    if t {
                                                                        break 'b_2aeeb8;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x6c));
                                                                        if t {
                                                                            break 'b_2aef3c;
                                                                        } else {
                                                                            break 'b_2aee60;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = c.g[V1][0] == c.g[V0][0];
                                                            c.g[V0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x14));
                                                            if t {
                                                                break 'b_2aeeb8;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6c));
                                                                if t {
                                                                    break 'b_2aef3c;
                                                                } else {
                                                                    break 'b_2aee70;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 18;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                    c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x8),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                                    c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x54),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x58),
                                                        0u64,
                                                    );
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x50),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x5c),
                                                        c.g[V0][0],
                                                    );
                                                    if t {
                                                        break 'b_2aef48;
                                                    } else {
                                                        break 'b_2aeeb8;
                                                    }
                                                }
                                                'b_2aeee8: {
                                                    'b_2aeee4: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.sw((c.g[S4][0] as u32), c.g[V0][0]);
                                                        let t = c.g[S4][0] != 0u64;
                                                        c.sw((c.g[S2][0] as u32), c.g[V1][0]);
                                                        if t {
                                                            break 'b_2aeee4;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[S2][0] != 0u64 {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6c));
                                                                break 'b_2aeee8;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.sw((c.g[S2][0] as u32), 0u64);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6c));
                                                                c.g[V0][0] = sx((c.g[S1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[V1][0] =
                                                                    c.lw((c.g[V0][0] as u32));
                                                                c.sw(
                                                                    (c.g[S2][0] as u32),
                                                                    c.g[V1][0],
                                                                );
                                                                break 'b_2aeee4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                    break 'b_2aeee8;
                                                }
                                                'b_2aef28: {
                                                    c.cycles += 16;
                                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                                    c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x8),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x54),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x50),
                                                        c.g[A0][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x1c));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0x18));
                                                    c.g[V0][0] = ((c.g[V0][0] as i64)
                                                        < (c.g[V1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x58),
                                                            0u64,
                                                        );
                                                        break 'b_2aef28;
                                                    } else {
                                                        break 'b_2aef28;
                                                    }
                                                }
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                                                c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x5c),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_2aef48;
                                                } else {
                                                    break 'b_2aef3c;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V1][0] = 0xffffffffffffffff;
                                            c.g[V0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                            break 'b_2aef48;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x66));
                                        break 'b_2aef4c;
                                    }
                                }
                                c.cycles += 4;
                                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002aee10);
                                    }
                                    continue 'l_2aee10;
                                } else {
                                    break 'b_2aef5c;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x002aef64);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                    call_ptr(c, 0x003b7f18, 0x002aef64)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    break 'b_2aef68;
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S4][0]);
                break 'b_2aef6c;
            }
            c.cycles += 5;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[S2][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[S5][0]);
            c.g[RA][0] = sx(0x002aef7c);
            c.sb((c.g[S0][0] as u32).wrapping_add(0x65), 0u64);
            call_ptr(c, 0x003b58d8, 0x002aef7c)?;
            c.g[V0][0] = 0u64;
            break 'b_2aef80;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        return Ok(());
    })
}

/// `int MusicPlayer_CreateStream(SNDPLAYOPTS* opts, a1, a2, a3, t0)` (0x2b1780): pushes a
/// new bank-monitor instance on the music system's bank manager (default play
/// options, zeroed per-instance fields, delay 100/90/50, max volume 127, close on
/// end), copies `*opts` into its level slot, creates stream 2 and returns its
/// handle. Straight-line transcription of the generated body.
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn music_player_create_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 188;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S2][0] = 0x18;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S3][0] = c.g[A3][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S4][0] = c.g[T0][0];
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S5][0] = c.g[A2][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S6][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S7][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x8;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[S0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 3, 3, 18);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[V1][0] as u32));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 18);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[RA][0] = sx(0x002b1898);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x002b1898)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = 0x64;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T3][0] = 0x5a;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[T4][0] = 0x32;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[T5][0] = 0x7f;
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[T6][0] = 0x1;
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T1][0] = c.g[S6][0];
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[T2][0] = c.g[S5][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A2][0] = 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A1][0] = 0x2;
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[T4][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[T5][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[T6][0]);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f8));
        c.mult(0, 2, 2, 18);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(11, (c.g[S7][0] as u32).wrapping_add(0x7));
        c.ldr(11, (c.g[S7][0] as u32));
        c.ldl(12, (c.g[S7][0] as u32).wrapping_add(0xf));
        c.ldr(12, (c.g[S7][0] as u32).wrapping_add(0x8));
        c.ldl(2, (c.g[S7][0] as u32).wrapping_add(0x17));
        c.ldr(2, (c.g[S7][0] as u32).wrapping_add(0x10));
        c.sdl(11, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(11, (c.g[V1][0] as u32));
        c.sdl(12, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(12, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.g[RA][0] = sx(0x002b1a20);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        Callee {
            addr: 0x002aa870,
            f: ee::s08::StreamArray_CreateStreamEx,
        }
        .call(c, 0x002b1a20)?;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6fc));
        c.g[A1][0] = 0x2;
        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[RA][0] = sx(0x002b1a44);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32));
        Callee {
            addr: 0x002ab128,
            f: ee::s08::StreamArray_GetStreamHandle,
        }
        .call(c, 0x002b1a44)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    })
}

/// `void MusicPlayer_dtor(MusicPlayer* self, int flags)` (0x2b3398): resets the vtables of
/// the stream system's bank manager and sound system (rebased on `self` when
/// `flags == 0`), destroys the song configurations (virtual dtor, flag 3), frees
/// the MPF file and song instance, clears `gMusicSys`, then by `flags` runs the
/// sub-object destructors (bit 1) and `operator_delete` (bit 0). Transcription of
/// the generated body (labels kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn music_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        'b_2b34bc: {
            c.cycles += 29;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
            c.g[V0][0] = 0x480000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x38f8));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[A3][0] = 0x480000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3928));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = 0x480000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3b48));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x5440), c.g[V0][0]);
            c.g[T0][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
            c.g[T5][0] = sx((c.g[T0][0] as u32).wrapping_add(0x3948));
            c.g[T6][0] = c.g[A1][0];
            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[A0][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
            let t = c.g[T6][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T5][0]);
            if t {
                break 'b_2b34bc;
            } else {
                c.cycles += 44;
                c.g[A1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3928));
                c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sd((c.g[SP][0] as u32), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                c.g[T1][0] = c.ld((c.g[T0][0] as u32).wrapping_add(0x3948));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T3][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32));
                c.g[T0][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A2][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[V1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5448));
                c.g[A1][0] = c.ld((c.g[T5][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[V1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
                c.g[A0][0] = c.lhu((c.g[T5][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T4][0]);
                c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5450));
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A0][0]);
                break 'b_2b34bc;
            }
        }
        'b_2b350c: {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
            c.g[S4][0] = c.g[T6][0] & 0x1;
            c.g[S1][0] = 0u64;
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = c.g[T6][0] & 0x2;
            if t {
                break 'b_2b350c;
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                'l_2b34d8: loop {
                    'b_2b34f8: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x3;
                        if t {
                            break 'b_2b34f8;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002b34f8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002b34f8)?;
                            break 'b_2b34f8;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3e4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002b34d8);
                        }
                        continue 'l_2b34d8;
                    } else {
                        break 'b_2b350c;
                    }
                }
            }
        }
        'b_2b3524: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5434));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x408));
                break 'b_2b3524;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002b3520);
                call_ptr(c, 0x00317e98, 0x002b3520)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x408));
                break 'b_2b3524;
            }
        }
        'b_2b3534: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;

            if t {
                break 'b_2b3534;
            } else {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002b3534);
                c.g[A1][0] = 0x3;
                Callee {
                    addr: 0x002b1f78,
                    f: ee::s08::SongInstance_dtor,
                }
                .call(c, 0x002b3534)?;
                break 'b_2b3534;
            }
        }
        'b_2b356c: {
            c.cycles += 2;
            let t = c.g[S3][0] == 0u64;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x6fc), 0u64);
            if t {
                break 'b_2b356c;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                c.g[RA][0] = sx(0x002b3548);
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x002aa648,
                    f: ee::s08::StreamArray_dtor,
                }
                .call(c, 0x002b3548)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x002b3558);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x0028bb10, 0x002b3558)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[RA][0] = sx(0x002b356c);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
                Callee {
                    addr: 0x002adee8,
                    f: ee::s08::SoundManagerRef_dtor,
                }
                .call(c, 0x002b356c)?;
                break 'b_2b356c;
            }
        }
        'b_2b3580: {
            c.cycles += 2;
            let t = c.g[S4][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            if t {
                break 'b_2b3580;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002b357c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00317e50, 0x002b357c)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                break 'b_2b3580;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    })
}

/// `void QuadTree_Compact(QuadTree* self, void** out, int* size, ctx)` (0x2c1da8,
/// `cQuadTree_exportTree`): allocates the node map and the output block, flattens the
/// tree into it, renumbers each node's children and byte-swaps the header and nodes
/// through the context's fix functions. Transcription of the generated body (labels
/// kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn quad_tree_compact(c: &mut Cpu) {
    guest(c, |c| {
        'b_2c1e54: {
            c.cycles += 33;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S1][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A1][0] = 0x480000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[T2][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S6][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S7][0] = c.g[T1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[FP][0] = c.g[T0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5fb0));
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28));
            c.sw((c.g[S0][0] as u32), c.g[V0][0]);
            c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x002c1e1c);
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            call_ptr(c, 0x00317e30, 0x002c1e1c)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S2][0] = c.g[V0][0];
            if t {
                break 'b_2c1e54;
            } else {
                c.cycles += 3;
                c.g[V1][0] = 0xffffffffffffffff;
                c.g[V0][0] = c.g[S4][0];
                'l_2c1e38: loop {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S2][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002c1e38);
                        }
                        continue 'l_2c1e38;
                    } else {
                        break 'b_2c1e54;
                    }
                }
            }
        }
        'b_2c1ff4: {
            c.cycles += 48;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
            c.g[A1][0] = 0x480000;
            c.g[A3][0] = 0u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5fc8));
            c.g[A2][0] = 0u64;
            c.g[RA][0] = sx(0x002c1e70);
            c.g[S2][0] = 0u64;
            call_ptr(c, 0x00317e30, 0x002c1e70)?;
            c.sw((c.g[S3][0] as u32), c.g[V0][0]);
            c.g[A1][0] = 0xffffffffdead0000;
            c.g[A1][0] = c.g[A1][0] | 0xbeef;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002c1e88);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
            call_ptr(c, 0x00416210, 0x002c1e88)?;
            c.g[S3][0] = c.lw((c.g[S3][0] as u32));
            c.g[A0][0] = c.g[S6][0];
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = c.g[S4][0];
            c.ldl(2, (c.g[S6][0] as u32).wrapping_add(0x7));
            c.ldr(2, (c.g[S6][0] as u32));
            c.ldl(3, (c.g[S6][0] as u32).wrapping_add(0xf));
            c.ldr(3, (c.g[S6][0] as u32).wrapping_add(0x8));
            c.ldl(5, (c.g[S6][0] as u32).wrapping_add(0x17));
            c.ldr(5, (c.g[S6][0] as u32).wrapping_add(0x10));
            c.ldl(8, (c.g[S6][0] as u32).wrapping_add(0x1f));
            c.ldr(8, (c.g[S6][0] as u32).wrapping_add(0x18));
            c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[S3][0] as u32));
            c.sdl(3, (c.g[S3][0] as u32).wrapping_add(0xf));
            c.sdr(3, (c.g[S3][0] as u32).wrapping_add(0x8));
            c.sdl(5, (c.g[S3][0] as u32).wrapping_add(0x17));
            c.sdr(5, (c.g[S3][0] as u32).wrapping_add(0x10));
            c.sdl(8, (c.g[S3][0] as u32).wrapping_add(0x1f));
            c.sdr(8, (c.g[S3][0] as u32).wrapping_add(0x18));
            c.ldl(2, (c.g[S6][0] as u32).wrapping_add(0x27));
            c.ldr(2, (c.g[S6][0] as u32).wrapping_add(0x20));
            c.sdl(2, (c.g[S3][0] as u32).wrapping_add(0x27));
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x28));
            c.sdr(2, (c.g[S3][0] as u32).wrapping_add(0x20));
            c.sw((c.g[SP][0] as u32), 0u64);
            c.g[A1][0] = c.g[S0][0];
            c.g[T0][0] = c.lhu((c.g[S6][0] as u32).wrapping_add(0x14));
            c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
            c.g[T0][0] = sx((c.g[T0][0] as u32) << 3);
            c.g[RA][0] = sx(0x002c1f08);
            c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            Callee {
                addr: 0x002c2088,
                f: ee::s08::QuadTree_FlattenNode,
            }
            .call(c, 0x002c1f08)?;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32));
            if (c.g[V0][0] as i64) <= 0 {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                break 'b_2c1ff4;
            } else {
                c.cycles += 1;
                'l_2c1f18: loop {
                    'b_2c1fdc: {
                        'b_2c1fbc: {
                            c.cycles += 4;
                            c.g[T1][0] = c.lhu((c.g[S0][0] as u32));
                            c.g[V0][0] = c.g[T1][0] & 0x1;
                            let t = c.g[V0][0] == 0u64;
                            c.g[T1][0] = sx((c.g[T1][0] as u32) >> 1);
                            if t {
                                break 'b_2c1fbc;
                            } else {
                                c.cycles += 37;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                                c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[T1][0] = sx((c.g[T1][0] as u32) << 2);
                                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
                                c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[T1][0] =
                                    sx((c.g[T1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                c.g[A0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                c.g[A2][0] = c.lhu((c.g[A0][0] as u32));
                                c.g[A1][0] = c.lhu((c.g[T1][0] as u32));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A3][0] = c.lhu((c.g[V0][0] as u32));
                                c.g[RA][0] = sx(0x002c1f78);
                                c.g[T0][0] = c.lhu((c.g[V1][0] as u32));
                                Callee {
                                    addr: 0x002c16b8,
                                    f: ee::s08::QuadTreeNode_SetChildren,
                                }
                                .call(c, 0x002c1f78)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32));
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1f84);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, t, 0x002c1f84)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2));
                                c.g[A1][0] = c.g[S1][0];
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1f94);
                                c.sh((c.g[S0][0] as u32), c.g[V0][0]);
                                call_ptr(c, t, 0x002c1f94)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = c.g[S1][0];
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1fa4);
                                c.sh((c.g[S0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                                call_ptr(c, t, 0x002c1fa4)?;
                                c.g[A0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x6));
                                c.g[A1][0] = c.g[S1][0];
                                let t = c.g[S5][0] as u32;
                                c.g[RA][0] = sx(0x002c1fb4);
                                c.sh((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                call_ptr(c, t, 0x002c1fb4)?;
                                let t = true;
                                c.sh((c.g[S0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
                                if t {
                                    break 'b_2c1fdc;
                                } else {
                                    break 'b_2c1fbc;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                        let t = c.g[S7][0] as u32;
                        c.g[RA][0] = sx(0x002c1fc8);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, t, 0x002c1fc8)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[A1][0] = c.g[S1][0];
                        let t = c.g[S7][0] as u32;
                        c.g[RA][0] = sx(0x002c1fd8);
                        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                        call_ptr(c, t, 0x002c1fd8)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        break 'b_2c1fdc;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002c1f18);
                        }
                        continue 'l_2c1f18;
                    } else {
                        c.cycles += 1;
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                        break 'b_2c1ff4;
                    }
                }
            }
        }
        c.cycles += 37;
        c.g[A1][0] = c.g[S1][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(0x24), 0u64);
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2004);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x1c));
        call_ptr(c, t, 0x002c2004)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2014);
        c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x18));
        call_ptr(c, t, 0x002c2014)?;
        c.lwc1(12, (c.g[S3][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2024);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, t, 0x002c2024)?;
        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2034);
        c.swc1(0, (c.g[S3][0] as u32));
        call_ptr(c, t, 0x002c2034)?;
        c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S1][0];
        let t = c.g[FP][0] as u32;
        c.g[RA][0] = sx(0x002c2044);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        call_ptr(c, t, 0x002c2044)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        let t = c.g[S7][0] as u32;
        c.g[RA][0] = sx(0x002c2054);
        c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x8));
        call_ptr(c, t, 0x002c2054)?;
        c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    })
}

/// `void Avalanche_WriteHistoryFile(bool direct, const char* name)` (0x2d83b8,
/// `cAvalanche::SaveData`): when the avalanche list exists, opens the course's
/// history file, writes the magic, the size and then every avalanche's nodes,
/// collisions and ground collisions, closes it and clears `cAvalanche_m_Save`.
/// Transcription of the generated body (labels kept).
#[allow(
    unused_labels,
    unused_assignments,
    unused_mut,
    unreachable_code,
    unused_variables
)]
pub fn avalanche_write_history_file(c: &mut Cpu) {
    guest(c, |c| {
        'b_2d87ac: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[S4][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
            let t = c.g[V0][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
            if t {
                break 'b_2d87ac;
            } else {
                'b_2d87a8: {
                    c.cycles += 25;
                    c.g[RA][0] = sx(0x002d83f0);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
                    call_ptr(c, 0x0014dc80, 0x002d83f0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x002d83fc);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x002d83fc)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3af0));
                    c.g[RA][0] = sx(0x002d8410);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00144be0, 0x002d8410)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002d841c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00144c48, 0x002d841c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002d8430);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x002d8430)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x26;
                    c.g[A2][0] = 0x64;
                    c.g[RA][0] = sx(0x002d8444);
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x104));
                    call_ptr(c, 0x003debf0, 0x002d8444)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
                    if t {
                        break 'b_2d87a8;
                    } else {
                        'b_2d84f4: {
                            c.cycles += 3;
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x108));
                            let t = c.g[S1][0] == 0u64;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10c));
                            if t {
                                break 'b_2d84f4;
                            } else {
                                c.cycles += 2;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                'l_2d8460: loop {
                                    'b_2d8494: {
                                        c.cycles += 2;
                                        if c.g[A0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                            break 'b_2d8494;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A1][0] = 0xa;
                                            'l_2d8470: loop {
                                                c.cycles += 8;
                                                c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x100));
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0xf4));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(2, 1);
                                                c.mult(0, 2, 2, 5);
                                                let t = c.g[A0][0] != 0u64;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d8470);
                                                    }
                                                    continue 'l_2d8470;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    break 'b_2d8494;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d84bc: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                        if t {
                                            break 'b_2d84bc;
                                        } else {
                                            c.cycles += 1;
                                            'l_2d84a0: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d84a0);
                                                    }
                                                    continue 'l_2d84a0;
                                                } else {
                                                    break 'b_2d84bc;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d84e8: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_2d84e8;
                                        } else {
                                            'l_2d84c8: loop {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V1][0] =
                                                    sx((c.g[V1][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d84c8);
                                                    }
                                                    continue 'l_2d84c8;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[S1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_2d84e8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[S1][0] != 0u64 {
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002d8460);
                                        }
                                        continue 'l_2d8460;
                                    } else {
                                        c.cycles += 1;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                                        break 'b_2d84f4;
                                    }
                                }
                            }
                        }
                        'b_2d879c: {
                            c.cycles += 20;
                            c.g[V0][0] = 0x2be0000;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                            c.g[V0][0] = c.g[V0][0] | 0xef00;
                            c.g[A0][0] = c.g[S4][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                            c.g[A2][0] = 0u64;
                            c.g[T0][0] = 0x4;
                            c.g[RA][0] = sx(0x002d8518);
                            c.g[T1][0] = 0x1;
                            Callee {
                                addr: 0x002d82a0,
                                f: ee::s09::Avalanche_WriteData,
                            }
                            .call(c, 0x002d8518)?;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                            c.g[A3][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A2][0] = 0x4;
                            c.g[T0][0] = 0x4;
                            c.g[RA][0] = sx(0x002d8534);
                            c.g[T1][0] = 0x1;
                            Callee {
                                addr: 0x002d82a0,
                                f: ee::s09::Avalanche_WriteData,
                            }
                            .call(c, 0x002d8534)?;
                            c.g[V1][0] = 0x8;
                            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
                            let t = c.g[S1][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                            if t {
                                break 'b_2d879c;
                            } else {
                                c.cycles += 3;
                                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                'l_2d8550: loop {
                                    'b_2d8630: {
                                        c.cycles += 2;
                                        if c.g[S0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                            break 'b_2d8630;
                                        } else {
                                            c.cycles += 2;
                                            c.g[S2][0] = c.g[S5][0];
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                            'l_2d8560: loop {
                                                c.cycles += 51;
                                                c.g[V0][0] = 0xbeef;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.sh(
                                                    (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    c.g[V0][0],
                                                );
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[T0][0] = 0x2;
                                                c.g[RA][0] = sx(0x002d8580);
                                                c.g[T1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8580)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = 0xa;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A3][0] = c.g[S2][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[A2][0],
                                                );
                                                c.g[T0][0] = 0x2;
                                                c.g[T1][0] = 0x1;
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x100));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(2, 1);
                                                c.mult(0, 2, 2, 3);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0xc));
                                                c.g[RA][0] = sx(0x002d85c0);
                                                c.sh(
                                                    (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    c.g[V0][0],
                                                );
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d85c0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A3][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xe0));
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[T0][0] = 0x4;
                                                c.g[A2][0] = c.g[V1][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[RA][0] = sx(0x002d85e8);
                                                c.g[T1][0] = 0x3;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d85e8)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = 0x1;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[T0][0] = 0xa;
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[A2][0],
                                                );
                                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x100));
                                                c.cvt_w_s(1, 0);
                                                c.mfc1(9, 1);
                                                c.g[RA][0] = sx(0x002d8614);
                                                c.g[A3][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xfc));
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8614)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[S0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
                                                let t = c.g[S0][0] != 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d8560);
                                                    }
                                                    continue 'l_2d8560;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                                    break 'b_2d8630;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d8714: {
                                        c.cycles += 26;
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                        c.g[A3][0] = c.g[S5][0];
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[T0][0] = 0x2;
                                        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x002d8650);
                                        c.g[T1][0] = 0x1;
                                        Callee {
                                            addr: 0x002d82a0,
                                            f: ee::s09::Avalanche_WriteData,
                                        }
                                        .call(c, 0x002d8650)?;
                                        c.g[S3][0] = 0u64;
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[A0][0] = c.g[S4][0];
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                        c.g[A3][0] = c.g[S5][0];
                                        c.g[A2][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[T0][0] = 0x2;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                                        c.g[T1][0] = 0x1;
                                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                        c.g[RA][0] = sx(0x002d8680);
                                        c.sh((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                                        Callee {
                                            addr: 0x002d82a0,
                                            f: ee::s09::Avalanche_WriteData,
                                        }
                                        .call(c, 0x002d8680)?;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                                        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                            break 'b_2d8714;
                                        } else {
                                            c.cycles += 4;
                                            c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                            'l_2d86a8: loop {
                                                c.cycles += 26;
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[T0][0] = 0x4;
                                                c.g[RA][0] = sx(0x002d86c0);
                                                c.g[T1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d86c0)?;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[T0][0] = 0x4;
                                                c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[T1][0] = 0x1;
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[A2][0],
                                                );
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A3][0] = c.lw((c.g[S2][0] as u32));
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[RA][0] = sx(0x002d86f4);
                                                c.g[A3][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x78));
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d86f4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S1][0] as u32).wrapping_add(0xc));
                                                c.g[V0][0] = ((c.g[S3][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d86a8);
                                                    }
                                                    continue 'l_2d86a8;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V0][0] = c
                                                        .lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                                    break 'b_2d8714;
                                                }
                                            }
                                        }
                                    }
                                    'b_2d8790: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = 0u64;
                                        if t {
                                            break 'b_2d8790;
                                        } else {
                                            c.cycles += 3;
                                            c.g[S2][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x112));
                                            c.g[S0][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x110));
                                            'l_2d8728: loop {
                                                c.cycles += 26;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[A2][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[T0][0] = 0x2;
                                                c.g[RA][0] = sx(0x002d8744);
                                                c.g[T1][0] = 0x1;
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8744)?;
                                                c.g[S3][0] =
                                                    sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[T0][0] = 0x2;
                                                c.g[A2][0] = c.g[V1][0];
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[T1][0] = 0x1;
                                                c.g[RA][0] = sx(0x002d8774);
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x4));
                                                Callee {
                                                    addr: 0x002d82a0,
                                                    f: ee::s09::Avalanche_WriteData,
                                                }
                                                .call(c, 0x002d8774)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x10c));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lhu((c.g[S1][0] as u32).wrapping_add(0xe));
                                                c.g[V0][0] = ((c.g[S3][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002d8728);
                                                    }
                                                    continue 'l_2d8728;
                                                } else {
                                                    break 'b_2d8790;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    if c.g[S1][0] != 0u64 {
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                        if event_due(c) {
                                            deliver_event_here(c, 0x002d8550);
                                        }
                                        continue 'l_2d8550;
                                    } else {
                                        break 'b_2d879c;
                                    }
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                        c.g[RA][0] = sx(0x002d87a8);
                        c.g[A1][0] = 0x64;
                        call_ptr(c, 0x003deca0, 0x002d87a8)?;
                        break 'b_2d87a8;
                    }
                }
                c.cycles += 1;
                c.sw((c.g[GP][0] as u32).wrapping_add(0x958), 0u64);
                break 'b_2d87ac;
            }
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (their own, unique to oc1043):
    /// a fake answers only when its machine holds `OC1043_MAGIC` here.
    pub(crate) const OC1043_CTRL: u32 = DATA + 0x27_1040;
    pub(crate) const OC1043_MAGIC: u32 = 0x1043_7e57;
    /// Mailbox words the setups fill and the fakes read.
    pub(crate) const OC1043_BOX: u32 = DATA + 0x27_1100;

    fn oc1043_box_w(c: &mut Cpu, k: u32, v: u32) {
        c.bus.write32(OC1043_BOX + 4 * k, v);
    }

    fn oc1043_box_r(c: &mut Cpu, k: u32) -> u32 {
        c.bus.read32(OC1043_BOX + 4 * k)
    }

    /// A gated callee fake: with the gate off it runs whatever the slot held
    /// before; with the gate on it logs the call like `fakes::fake` (every
    /// register the caller set up), then `$post` sets results from the
    /// arguments it saw (`[a0, a1, a2, a3, t0, t1]`).
    macro_rules! oc1043_fake {
        ($name:ident, $prev:ident, $gen:path, $post:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1043_CTRL) != OC1043_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                let ret = c.g[RA][0] as u32;
                let args = [
                    c.g[A0][0] as u32,
                    c.g[A1][0] as u32,
                    c.g[A2][0] as u32,
                    c.g[A3][0] as u32,
                    c.g[T0][0] as u32,
                    c.g[T1][0] as u32,
                ];
                fakes::fake(c, target, ret);
                if c.pc == ret {
                    let post: fn(&mut Cpu, [u32; 6]) = $post;
                    post(c, args);
                }
            }
        };
    }

    // ----------------------------------------------------------------------
    // PATH_PlatGetTrackStatus.

    pub(crate) const OC1043_TRACK_TABLE: u32 = 0x004d_3e98;

    // 0x3b6670: stream position: header rate (u16) and sample count.
    oc1043_fake!(
        oc1043_fake_pos,
        OC1043_PREV_POS,
        ee::s13::f_003b6670__generated,
        |c, a| {
            let rate = oc1043_box_r(c, 1);
            let samples = oc1043_box_r(c, 2);
            let res = oc1043_box_r(c, 0);
            c.bus.write16(a[2], rate as u16);
            c.bus.write32(a[4], samples);
            c.g[V0][0] = sx(res);
        }
    );
    // 0x3b8ad8: time remaining of a sub-stream.
    oc1043_fake!(
        oc1043_fake_time,
        OC1043_PREV_TIME,
        ee::s13::f_003b8ad8__generated,
        |c, _a| {
            let v = oc1043_box_r(c, 4);
            c.g[V0][0] = sx(v);
        }
    );
    // 0x3b86c0: sub-stream volume.
    oc1043_fake!(
        oc1043_fake_vol,
        OC1043_PREV_VOL,
        ee::s13::f_003b86c0__generated,
        |c, _a| {
            let v = oc1043_box_r(c, 5);
            c.g[V0][0] = sx(v);
        }
    );
    // 0x3b7b48: stream info: the request count, status result.
    oc1043_fake!(
        oc1043_fake_info,
        OC1043_PREV_INFO,
        ee::s13::f_003b7b48__generated,
        |c, a| {
            let count = oc1043_box_r(c, 6);
            let res = oc1043_box_r(c, 7);
            c.bus.write32(a[1], count);
            c.g[V0][0] = sx(res);
        }
    );
    // 0x3b7c40: request status (state, a, b, c), alternating between two
    // prepared answers.
    oc1043_fake!(
        oc1043_fake_req,
        OC1043_PREV_REQ,
        ee::s13::f_003b7c40__generated,
        |c, a| {
            let n = oc1043_box_r(c, 0x20);
            oc1043_box_w(c, 0x20, n + 1);
            for k in 0..4 {
                let v = oc1043_box_r(c, 8 + 4 * (n % 2) + k);
                c.bus.write32(a[1] + 4 * k, v);
            }
        }
    );
    // 0x3b7f18: stream volume.
    oc1043_fake!(
        oc1043_fake_stvol,
        OC1043_PREV_STVOL,
        ee::s13::f_003b7f18__generated,
        |c, _a| {
            let v = oc1043_box_r(c, 0x10);
            c.g[V0][0] = sx(v);
        }
    );

    fn oc1043_install_track_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B6670),
            oc1043_fake_pos,
            &OC1043_PREV_POS,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8AD8),
            oc1043_fake_time,
            &OC1043_PREV_TIME,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B86C0),
            oc1043_fake_vol,
            &OC1043_PREV_VOL,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7B48),
            oc1043_fake_info,
            &OC1043_PREV_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7C40),
            oc1043_fake_req,
            &OC1043_PREV_REQ,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B7F18),
            oc1043_fake_stvol,
            &OC1043_PREV_STVOL,
        );
    }

    /// A monitor slot of every shape: no slot, no handle, an in-memory stream
    /// (negative sub, gone sub, live sub with positive or negative sample
    /// counts, failing position or time) and streaming ones (no subs, subs
    /// that are gone, queued, playing, parsing or dead), with out pointers
    /// that are sometimes null.
    #[test]
    fn path_plat_get_track_status_matches() {
        oc1043_install_track_fakes();
        covered(
            ee::s08::f_002aec58__generated,
            path_plat_get_track_status,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ec58, 3);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let idx = seed % 24;
                for i in 0..24 {
                    c.bus.write32(OC1043_TRACK_TABLE + 4 * i, 0);
                }
                let m = DATA + 0x28_0000;
                let subs = DATA + 0x28_1000;
                let kind = seed % 12;
                let streaming = kind >= 8;
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = if streaming && seed % 5 == 0 {
                    0
                } else {
                    (DATA + 0x28_2000) as u64
                };
                c.g[A2][0] = if streaming && seed % 7 == 0 {
                    0
                } else {
                    (DATA + 0x28_2010) as u64
                };
                if kind != 0 {
                    c.bus.write32(OC1043_TRACK_TABLE + 4 * idx, m);
                }
                // handle, sub index
                c.bus.write32(
                    m,
                    if kind == 1 {
                        u32::MAX
                    } else {
                        1 + r.next_u32() % 100
                    },
                );
                c.bus.write32(
                    m + 0x4,
                    if kind == 2 {
                        u32::MAX
                    } else {
                        r.next_u32() % 3
                    },
                );
                c.bus.write32(m + 0x50, r.next_u32() % 1000);
                c.bus.write32(m + 0x54, r.next_u32() % 1000);
                c.bus.write32(m + 0x6c, subs);
                c.bus.write8(m + 0x64, ((2..=7).contains(&kind)) as u8);
                let count = match kind {
                    8 => 0,
                    9 | 10 => 2,
                    11 => 1 + r.next_u32() % 3,
                    _ => 0,
                };
                c.bus.write8(m + 0x66, count as u8);
                for i in 0..3u32 {
                    let s = subs + 12 * i;
                    c.bus.write32(s, 1 + r.next_u32() % 1000);
                    let id = if (kind == 3 && i == 0) || (kind == 10 && i == 0) {
                        u32::MAX
                    } else {
                        5 + i
                    };
                    c.bus.write32(s + 4, id);
                    c.bus.write32(s + 8, r.next_u32() % 100);
                }
                // fakes' answers
                oc1043_box_w(
                    c,
                    0,
                    if kind == 6 {
                        u32::MAX
                    } else {
                        r.next_u32() % 3
                    },
                );
                oc1043_box_w(c, 1, r.next_u32() % 48000 + (seed % 9 != 0) as u32);
                oc1043_box_w(
                    c,
                    2,
                    if kind == 5 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() % 100_000
                    },
                );
                oc1043_box_w(
                    c,
                    4,
                    if kind == 7 {
                        u32::MAX - 4
                    } else {
                        r.next_u32() % 500
                    },
                );
                oc1043_box_w(c, 5, r.next_u32());
                oc1043_box_w(c, 6, r.next_u32() % 4);
                oc1043_box_w(
                    c,
                    7,
                    if r.next_u32() % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 3
                    },
                );
                oc1043_box_w(c, 0x10, r.next_u32());
                let states = [0u32, 1, 2, 3, u32::MAX, 5];
                for k in 0..2 {
                    oc1043_box_w(c, 8 + 4 * k, states[(r.next_u32() % 6) as usize]);
                    for j in 1..4 {
                        let v = if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            r.next_u32() % 100
                        };
                        oc1043_box_w(c, 8 + 4 * k + j, v);
                    }
                }
            },
        );
    }

    // ----------------------------------------------------------------------
    // MusicPlayer_CreateStream.

    oc1043_fake!(
        oc1043_fake_create_instance,
        OC1043_PREV_CREATE_INSTANCE,
        ee::s08::f_002aa870__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_get_handle,
        OC1043_PREV_GET_HANDLE,
        ee::s08::f_002ab128__generated,
        |_c, _a| {}
    );

    /// A new music-bank instance on a bank manager with `inst` instances so
    /// far; the play options are copied unaligned from `opts`.
    #[test]
    fn music_player_create_stream_matches() {
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AA870),
            oc1043_fake_create_instance,
            &OC1043_PREV_CREATE_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AB128),
            oc1043_fake_get_handle,
            &OC1043_PREV_GET_HANDLE,
        );
        covered(
            ee::s08::f_002b1780__generated,
            music_player_create_stream,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_1780, 2);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let (pp, p1, bm) = (DATA + 0x2a_0000, DATA + 0x2a_0100, DATA + 0x2a_1000);
                c.bus.write32(GAME_GP + 0x6fc, pp);
                c.bus.write32(pp, p1);
                c.bus.write32(p1, bm);
                c.bus.write32(bm + 0x1f4, r.next_u32() % 3);
                for k in 0..6 {
                    c.bus.write32(bm + 0x1dc + 4 * k, r.next_u32());
                }
                let arr = DATA + 0x2a_4000;
                for (k, off) in [
                    0x20u32, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c,
                    0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    let skew = if off == 0x20 { seed % 8 } else { 0 };
                    c.bus
                        .write32(bm + 0x1d8 + off, arr + 0x400 * k as u32 + skew);
                    fill(c, &mut r, arr + 0x400 * k as u32, 0x20);
                }
                let opts = DATA + 0x2a_8000;
                fill(c, &mut r, opts, 8);
                c.g[A0][0] = (opts + (seed % 8)) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // MusicPlayer_dtor.

    oc1043_fake!(
        oc1043_fake_mem_free,
        OC1043_PREV_MEM_FREE,
        ee::s10::f_00317e98__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_operator_delete,
        OC1043_PREV_OPERATOR_DELETE,
        ee::s10::f_00317e50__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_song_instance_dtor,
        OC1043_PREV_SONG_INSTANCE_DTOR,
        ee::s08::f_002b1f78__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_stream_sys_dtor,
        OC1043_PREV_STREAM_SYS_DTOR,
        ee::s08::f_002aa648__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_bank_manager_dtor,
        OC1043_PREV_BANK_MANAGER_DTOR,
        ee::s07::f_0028bb10__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_sound_sys_dtor,
        OC1043_PREV_SOUND_SYS_DTOR,
        ee::s08::f_002adee8__generated,
        |_c, _a| {}
    );

    /// Every `flags` value (vtable rebasing, sub-object dtors, delete) with
    /// 0-3 song configurations (null or with a virtual dtor), an MPF file and
    /// a song instance that are present or not.
    #[test]
    fn music_player_dtor_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            oc1043_fake_mem_free,
            &OC1043_PREV_MEM_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc1043_fake_operator_delete,
            &OC1043_PREV_OPERATOR_DELETE,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B1F78),
            oc1043_fake_song_instance_dtor,
            &OC1043_PREV_SONG_INSTANCE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AA648),
            oc1043_fake_stream_sys_dtor,
            &OC1043_PREV_STREAM_SYS_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028BB10),
            oc1043_fake_bank_manager_dtor,
            &OC1043_PREV_BANK_MANAGER_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADEE8),
            oc1043_fake_sound_sys_dtor,
            &OC1043_PREV_SOUND_SYS_DTOR,
        );
        covered(
            ee::s08::f_002b3398__generated,
            music_player_dtor,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_3398, 8);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let me = DATA + 0x2c_0000;
                let (p1, p2, p3) = (DATA + 0x2c_8000, DATA + 0x2c_9000, DATA + 0x2c_a000);
                c.bus.write32(me, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2 + 0x1d8, p3);
                c.bus.write32(me + 0x3e4, r.next_u32() % 5);
                for i in 0..4u32 {
                    let song = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x2d_0000 + 0x100 * i
                    };
                    c.bus.write32(me + 4 + 4 * i, song);
                    if song != 0 {
                        let vt = DATA + 0x2e_0000 + 0x20 * i;
                        c.bus.write32(song + 0xa8, vt);
                        c.bus.write16(vt + 8, r.next_u32() as u16);
                        c.bus.write32(vt + 0xc, FAKE + 0x10 * (i + 1));
                    }
                }
                let mpf = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    DATA + 0x2c_c000
                };
                c.bus.write32(me + 0x5434, mpf);
                let inst = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    DATA + 0x2c_d000
                };
                c.bus.write32(me + 0x408, inst);
                c.g[A0][0] = me as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // QuadTree_Compact.

    oc1043_fake!(
        oc1043_fake_qt_alloc,
        OC1043_PREV_QT_ALLOC,
        ee::s10::f_00317e30__generated,
        |c, a| {
            let p = oc1043_box_r(c, 0x30);
            oc1043_box_w(c, 0x30, (p + a[0] + 15) & !15);
            c.g[V0][0] = sx(p);
        }
    );
    oc1043_fake!(
        oc1043_fake_qt_flatten,
        OC1043_PREV_QT_FLATTEN,
        ee::s08::f_002c2088__generated,
        |c, a| {
            let n = oc1043_box_r(c, 0x40);
            c.bus.write32(a[2], n);
            for k in 0..n * 2 {
                let v = oc1043_box_r(c, 0x48 + k);
                c.bus.write32(a[1] + 4 * k, v);
            }
        }
    );
    oc1043_fake!(
        oc1043_fake_qt_set_children,
        OC1043_PREV_QT_SET_CHILDREN,
        ee::s08::f_002c16b8__generated,
        |_c, _a| {}
    );

    /// A tree of 0-4 nodes (leaf and inner records) exported through three
    /// callbacks (fixInt, fixShort, fixFloat at fake addresses).
    #[test]
    fn quad_tree_compact_matches() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            oc1043_fake_qt_alloc,
            &OC1043_PREV_QT_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2088),
            oc1043_fake_qt_flatten,
            &OC1043_PREV_QT_FLATTEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C16B8),
            oc1043_fake_qt_set_children,
            &OC1043_PREV_QT_SET_CHILDREN,
        );
        covered(
            ee::s08::f_002c1da8__generated,
            quad_tree_compact,
            160,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c_1da8, 12);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                let me = DATA + 0x2f_8000;
                fill(c, &mut r, me, 10);
                c.bus.write32(me + 0xc, r.next_u32() % 8);
                c.bus.write32(me + 0x10, r.next_u32() % 5);
                c.bus.write16(me + 0x14, (r.next_u32() % 4) as u16);
                c.bus.write32(me + 0x20, DATA + 0x2f_a000);
                oc1043_box_w(c, 0x30, DATA + 0x30_0000);
                let n = r.next_u32() % 5;
                oc1043_box_w(c, 0x40, n);
                for k in 0..8 {
                    // nodes alternate between inner (bit 0 set) and leaf records
                    let v = r.next_u32();
                    oc1043_box_w(
                        c,
                        0x48 + k,
                        if k % 2 == 0 && seed % 3 != 0 {
                            v | 1
                        } else {
                            v
                        },
                    );
                }
                c.g[A0][0] = me as u64;
                c.g[A1][0] = (DATA + 0x2f_9000) as u64;
                c.g[A2][0] = (DATA + 0x2f_9010) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = (FAKE + 0x100) as u64;
                c.g[T1][0] = (FAKE + 0x110) as u64;
                c.g[T2][0] = (FAKE + 0x120) as u64;
            },
        );
    }

    // ----------------------------------------------------------------------
    // Avalanche_WriteHistoryFile.

    oc1043_fake!(
        oc1043_fake_av_get_be,
        OC1043_PREV_AV_GET_BE,
        ee::s01::f_0014dc80__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_get_interface,
        OC1043_PREV_AV_GET_INTERFACE,
        ee::s01::f_0014dd58__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_location_id,
        OC1043_PREV_AV_LOCATION_ID,
        ee::s01::f_00144be0__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_location_file,
        OC1043_PREV_AV_LOCATION_FILE,
        ee::s01::f_00144c48__generated,
        |_c, _a| {}
    );
    oc1043_fake!(
        oc1043_fake_av_sprintf,
        OC1043_PREV_AV_SPRINTF,
        ee::s15::f_00417828__generated,
        |_c, _a| {}
    );
    // 0x3debf0 opensync: success flag in v0, the handle through a3.
    oc1043_fake!(
        oc1043_fake_av_open,
        OC1043_PREV_AV_OPEN,
        ee::s14::f_003debf0__generated,
        |c, a| {
            let ok = oc1043_box_r(c, 0);
            let fd = oc1043_box_r(c, 1);
            c.bus.write32(a[3], fd);
            c.g[V0][0] = ok as u64;
        }
    );
    oc1043_fake!(
        oc1043_fake_av_close,
        OC1043_PREV_AV_CLOSE,
        ee::s14::f_003deca0__generated,
        |_c, _a| {}
    );
    // 0x2d82a0 writeData: returns the bytes it wrote (size * count).
    oc1043_fake!(
        oc1043_fake_av_write,
        OC1043_PREV_AV_WRITE,
        ee::s09::f_002d82a0__generated,
        |c, a| {
            c.g[V0][0] = (a[4].wrapping_mul(a[5])) as u64;
        }
    );

    /// The avalanche history file: no list, a failing open, and 1-2
    /// avalanches with 0-2 nodes (life from 0 to 10, negative too) and 0-3
    /// collisions and ground collisions each.
    #[test]
    fn avalanche_write_history_file_matches() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc1043_fake_av_get_be,
            &OC1043_PREV_AV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc1043_fake_av_get_interface,
            &OC1043_PREV_AV_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BE0),
            oc1043_fake_av_location_id,
            &OC1043_PREV_AV_LOCATION_ID,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144C48),
            oc1043_fake_av_location_file,
            &OC1043_PREV_AV_LOCATION_FILE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc1043_fake_av_sprintf,
            &OC1043_PREV_AV_SPRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEBF0),
            oc1043_fake_av_open,
            &OC1043_PREV_AV_OPEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DECA0),
            oc1043_fake_av_close,
            &OC1043_PREV_AV_CLOSE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002D82A0),
            oc1043_fake_av_write,
            &OC1043_PREV_AV_WRITE,
        );
        covered(
            ee::s09::f_002d83b8__generated,
            avalanche_write_history_file,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d_83b8, 12);
                c.bus.write32(OC1043_CTRL, OC1043_MAGIC);
                oc1043_box_w(c, 0, (seed % 6 != 0) as u32);
                oc1043_box_w(c, 1, r.next_u32());
                c.g[A0][0] = (seed % 2) as u64;
                c.g[A1][0] = (DATA + 0x2e_f000) as u64;
                let base = DATA + 0x31_0000;
                let n_av = 1 + r.next_u32() % 2;
                c.bus
                    .write32(GAME_GP + 0x9c0, if seed % 8 == 0 { 0 } else { base });
                let lives = [0u32, 0x40a0_0000, 0x4120_0000, 0xc000_0000, 0x3f80_0000];
                let mut node_at = DATA + 0x33_0000;
                for i in 0..n_av {
                    let av = base + 0x1000 * i;
                    c.bus
                        .write32(av + 8, if i + 1 < n_av { av + 0x1000 } else { 0 });
                    // nodes
                    let nn = r.next_u32() % 3;
                    let mut head = 0;
                    for _ in 0..nn {
                        let node = node_at;
                        node_at += 0x200;
                        c.bus.write32(node + 0xf4, head);
                        c.bus.write32(node + 0xfc, DATA + 0x34_0000);
                        c.bus
                            .write32(node + 0x100, lives[(r.next_u32() % 5) as usize]);
                        fill(c, &mut r, node + 0xe0, 3);
                        head = node;
                    }
                    c.bus.write32(av + 4, head);
                    let ncoll = r.next_u32() % 4;
                    let ngr = r.next_u32() % 4;
                    c.bus.write16(av + 0xc, ncoll as u16);
                    c.bus.write16(av + 0xe, ngr as u16);
                    for k in 0..ncoll {
                        c.bus.write32(av + 0x10 + 8 * k, r.next_u32());
                        c.bus
                            .write32(av + 0x14 + 8 * k, DATA + 0x32_0000 + 0x100 * k);
                    }
                    for k in 0..ngr {
                        c.bus.write16(av + 0x110 + 4 * k, r.next_u32() as u16);
                        c.bus.write16(av + 0x112 + 4 * k, r.next_u32() as u16);
                    }
                }
                for k in 0..4 {
                    c.bus
                        .write32(DATA + 0x32_0000 + 0x100 * k + 0x78, r.next_u32());
                }
            },
        );
    }
}
