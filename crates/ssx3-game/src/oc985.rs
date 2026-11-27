//! Staging module oc985 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc985`.
//!
//! The career stats screen (`cFEStateCareerStats`, `festatebuyattrib`): input
//! handling, tab focus and the highlights / race / freestyle pages. The bodies
//! are the generated bodies (every register, cycle and call kept); the calls go
//! through the function table and the readable aliases of the callees.

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
        0x001f_56c8,
        cfe_state_career_stats_handle_input,
        "cFEStateCareerStats_HandleInput",
    ),
    (
        0x001f_5ba8,
        cfe_state_career_stats_select_page,
        "cFEStateCareerStats_SelectPage",
    ),
    (
        0x001f_5da0,
        cfe_state_career_stats_show_highlights_page,
        "cFEStateCareerStats_ShowHighlightsPage",
    ),
    (
        0x001f_60e0,
        cfe_state_career_stats_show_race_page,
        "cFEStateCareerStats_ShowRacePage",
    ),
    (
        0x001f_6490,
        cfe_state_career_stats_show_freestyle_page,
        "cFEStateCareerStats_ShowFreestylePage",
    ),
];

/// `cFEStateCareerStats` fields (offsets from `self`).
#[cfg(test)]
pub(crate) mod cfe_state_career_stats_field {
    pub const ENGINE: u32 = 0x10;
    pub const SCREEN: u32 = 0x40;
    pub const STATS_STATE: u32 = 0x48;
    pub const TOP_HIGHLIGHT: u32 = 0x4c;
    pub const TOP_RACE: u32 = 0x50;
    pub const TOP_FREESTYLE: u32 = 0x54;
    pub const TOTAL_RACE: u32 = 0x58;
    pub const RACE_MEDALS: u32 = 0x5c;
    pub const RACE_DONE: u32 = 0x60;
    pub const TOTAL_FREESTYLE: u32 = 0x68;
    pub const FREESTYLE_MEDALS: u32 = 0x6c;
    pub const FREESTYLE_DONE: u32 = 0x70;
}

/// `bool cFEStateCareerStats_HandleInput(self, cInputEvent* w)` (0x1f56c8): the career
/// stats screen's input. Left (19 on the widget's `want` slot at +0x98) / right (20, +0xa0)
/// step the page `self->mStatsState` (+0x48) through the four pages (wrapping 0..3), play
/// the "hll"/"hlr" frame and redraw (`SelectPage`, `ShowPage`). Up (17, +0x88) / down (18,
/// +0x90) scroll the highlights (+0x4c, 24 rows, 3 visible), race (+0x50, `count` +0x58, 5
/// visible) or freestyle (+0x54, `count` +0x68) list one row, replay the matching frame and
/// redraw that page. Finally the UI sound object (`engine+0x14`, else the null sound at
/// `gp+0x2968`) plays sound 1 (page), 2 (scrolled), 4 (at the end) or 0. Returns whether
/// anything changed.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f57cc: {
            'b_1f5764: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S3][0] = 0u64;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001f5704);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5704)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_1f5764;
                } else {
                    'b_1f5744: {
                        c.cycles += 10;
                        c.g[A0][0] = 0x4a0000;
                        c.g[RA][0] = sx(0x001f5718);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2358));
                        call_ptr(c, 0x00317670, 0x001f5718)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f5724);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039c778, 0x001f5724)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        if c.g[A1][0] == c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            break 'b_1f5744;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f5740);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x0039c870, 0x001f5740)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            break 'b_1f5744;
                        }
                    }
                    c.cycles += 5;
                    c.g[S2][0] = 0x1;
                    c.g[S3][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                    if t {
                        break 'b_1f57cc;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                        if t {
                            break 'b_1f57cc;
                        } else {
                            break 'b_1f5764;
                        }
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001f5774);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001f5774)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x4a0000;
            if t {
                break 'b_1f57cc;
            } else {
                'b_1f57b0: {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001f5784);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2360));
                    call_ptr(c, 0x00317670, 0x001f5784)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f5790);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001f5790)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    if c.g[A1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                        break 'b_1f57b0;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f57ac);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039c870, 0x001f57ac)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                        break 'b_1f57b0;
                    }
                }
                c.cycles += 6;
                c.g[S2][0] = 0x1;
                c.g[S3][0] = 0x1;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                if t {
                    break 'b_1f57cc;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                    break 'b_1f57cc;
                }
            }
        }
        'b_1f57ec: {
            c.cycles += 2;
            if c.g[S3][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                break 'b_1f57ec;
            } else {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x001f57e0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001f5ba8,
                    f: ee::s04::cFEStateCareerStats_SelectPage,
                }
                .call(c, 0x001f57e0)?;
                c.g[RA][0] = sx(0x001f57e8);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001f5a38,
                    f: ee::s04::cFEStateCareerStats_ShowPage,
                }
                .call(c, 0x001f57e8)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                break 'b_1f57ec;
            }
        }
        'b_1f59ec: {
            'b_1f59e8: {
                'b_1f59e4: {
                    'b_1f59ac: {
                        'b_1f5880: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != 0u64;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1f5880;
                            } else {
                                'b_1f5838: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001f5808);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001f5808)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1f5838;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_1f59e4;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x4c),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001f582c);
                                            c.g[S3][0] = 0x1;
                                            Callee {
                                                addr: 0x001f5da0,
                                                f: ee::s04::cFEStateCareerStats_ShowHighlightsPage,
                                            }
                                            .call(c, 0x001f582c)?;
                                            c.g[A0][0] = 0x4a0000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2368));
                                            if t {
                                                break 'b_1f59ac;
                                            } else {
                                                break 'b_1f5838;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f5848);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f5848)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f59ec;
                                } else {
                                    c.cycles += 5;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 24i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_1f59e4;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f5874);
                                        c.g[S3][0] = 0x1;
                                        Callee {
                                            addr: 0x001f5da0,
                                            f: ee::s04::cFEStateCareerStats_ShowHighlightsPage,
                                        }
                                        .call(c, 0x001f5874)?;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe88));
                                        if t {
                                            break 'b_1f59ac;
                                        } else {
                                            break 'b_1f5880;
                                        }
                                    }
                                }
                            }
                        }
                        'b_1f5918: {
                            c.cycles += 2;
                            let t = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_1f5918;
                            } else {
                                'b_1f58cc: {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001f589c);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001f589c)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1f58cc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        if t {
                                            break 'b_1f59e4;
                                        } else {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x50),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001f58c0);
                                            c.g[S3][0] = 0x1;
                                            Callee {
                                                addr: 0x001f60e0,
                                                f: ee::s04::cFEStateCareerStats_ShowRacePage,
                                            }
                                            .call(c, 0x001f58c0)?;
                                            c.g[A0][0] = 0x4a0000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2370));
                                            if t {
                                                break 'b_1f59ac;
                                            } else {
                                                break 'b_1f58cc;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f58dc);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f58dc)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    break 'b_1f59ec;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                    if t {
                                        break 'b_1f59e4;
                                    } else {
                                        c.cycles += 7;
                                        c.g[S3][0] = 0x1;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001f590c);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                                        Callee {
                                            addr: 0x001f60e0,
                                            f: ee::s04::cFEStateCareerStats_ShowRacePage,
                                        }
                                        .call(c, 0x001f590c)?;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe78));
                                        if t {
                                            break 'b_1f59ac;
                                        } else {
                                            break 'b_1f5918;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_1f59ec;
                        } else {
                            'b_1f5964: {
                                c.cycles += 7;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001f5934);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f5934)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_1f5964;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    if t {
                                        break 'b_1f59e4;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001f5958);
                                        c.g[S3][0] = 0x1;
                                        Callee {
                                            addr: 0x001f6490,
                                            f: ee::s04::cFEStateCareerStats_ShowFreestylePage,
                                        }
                                        .call(c, 0x001f5958)?;
                                        c.g[A0][0] = 0x4a0000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2378));
                                        if t {
                                            break 'b_1f59ac;
                                        } else {
                                            break 'b_1f5964;
                                        }
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001f5974);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001f5974)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                break 'b_1f59ec;
                            } else {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f59e4;
                                } else {
                                    c.cycles += 6;
                                    c.g[S3][0] = 0x1;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001f59a4);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                    Callee {
                                        addr: 0x001f6490,
                                        f: ee::s04::cFEStateCareerStats_ShowFreestylePage,
                                    }
                                    .call(c, 0x001f59a4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe68));
                                    break 'b_1f59ac;
                                }
                            }
                        }
                    }
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001f59b4);
                    call_ptr(c, 0x00317670, 0x001f59b4)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f59c0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039c778, 0x001f59c0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let t = c.g[A1][0] == c.g[V0][0];
                    c.g[S2][0] = 0x2;
                    if t {
                        break 'b_1f59e8;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f59dc);
                        c.g[A2][0] = 0x1;
                        call_ptr(c, 0x0039c870, 0x001f59dc)?;
                        let t = true;
                        c.g[S2][0] = 0x2;
                        if t {
                            break 'b_1f59e8;
                        } else {
                            break 'b_1f59e4;
                        }
                    }
                }
                c.cycles += 1;
                c.g[S2][0] = 0x4;
                break 'b_1f59e8;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
            break 'b_1f59ec;
        }
        'b_1f5a14: {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            if c.g[A2][0] == 0 {
                c.g[A2][0] = c.g[V1][0];
            }
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S2][0];
            if t {
                break 'b_1f5a14;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5a14);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5a14)?;
                break 'b_1f5a14;
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S3][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateCareerStats_SelectPage(self, int menu)` (0x1f5ba8, `setupMenuFocus`):
/// stores the page (`self+0x48`), looks the four tab widgets up by hash name and takes
/// the focus off each (virtual `+0x38/+0x3c` with 0), then gives the focus to the
/// selected page's tab and builds its page (highlights, race, freestyle or records).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_select_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f5c04: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x470000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdf0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001f5bd8);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[A1][0]);
            call_ptr(c, 0x00317670, 0x001f5bd8)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5be4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5be4)?;
            c.g[S4][0] = c.g[V0][0];
            let t = c.g[S4][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5c04;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5c04);
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5c04)?;
                break 'b_1f5c04;
            }
        }
        'b_1f5c3c: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5c10);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xde0));
            call_ptr(c, 0x00317670, 0x001f5c10)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5c1c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5c1c)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5c3c;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5c3c);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5c3c)?;
                break 'b_1f5c3c;
            }
        }
        'b_1f5c74: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5c48);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdc8));
            call_ptr(c, 0x00317670, 0x001f5c48)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5c54);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5c54)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5c74;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5c74);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5c74)?;
                break 'b_1f5c74;
            }
        }
        'b_1f5cac: {
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5c80);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xdb0));
            call_ptr(c, 0x00317670, 0x001f5c80)?;
            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001f5c8c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001f5c8c)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1f5cac;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5cac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5cac)?;
                break 'b_1f5cac;
            }
        }
        'b_1f5d88: {
            'b_1f5d84: {
                'b_1f5d64: {
                    'b_1f5d3c: {
                        'b_1f5d14: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_1f5d14;
                            } else {
                                'b_1f5cec: {
                                    'b_1f5cd4: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_1f5cd4;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1f5cec;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1f5d88;
                                                } else {
                                                    break 'b_1f5cd4;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_1f5d3c;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            break 'b_1f5d64;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1f5d84;
                                            } else {
                                                break 'b_1f5cec;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001f5d04);
                                c.g[A0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001f5d04)?;
                                c.g[RA][0] = sx(0x001f5d0c);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001f5da0,
                                    f: ee::s04::cFEStateCareerStats_ShowHighlightsPage,
                                }
                                .call(c, 0x001f5d0c)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1f5d84;
                                } else {
                                    break 'b_1f5d14;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001f5d2c);
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001f5d2c)?;
                        c.g[RA][0] = sx(0x001f5d34);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001f60e0,
                            f: ee::s04::cFEStateCareerStats_ShowRacePage,
                        }
                        .call(c, 0x001f5d34)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_1f5d84;
                        } else {
                            break 'b_1f5d3c;
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001f5d54);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001f5d54)?;
                    c.g[RA][0] = sx(0x001f5d5c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001f6490,
                        f: ee::s04::cFEStateCareerStats_ShowFreestylePage,
                    }
                    .call(c, 0x001f5d5c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1f5d84;
                    } else {
                        break 'b_1f5d64;
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001f5d78);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001f5d78)?;
                c.g[RA][0] = sx(0x001f5d80);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001f6890,
                    f: ee::s04::cFEStateCareerStats_ShowRecordsPage,
                }
                .call(c, 0x001f5d80)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1f5d84;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1f5d88;
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

/// `void cFEStateCareerStats_ShowHighlightsPage(self)` (0x1f5da0, `setupHighlightsList`):
/// shows or hides the scroll arrows (`self+0x4c` > 0, `+3 < 24`) and fills the three
/// visible rows (`i + scroll`, level = row / 3, step = row % 3): the step's name, the
/// on/off marker from the score interface's current highlight level, the trick name and
/// the required-trick text (or the locked text).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_show_highlights_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f5e14: {
            'b_1f5e04: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x210));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x170), c.g[RA][0]);
                c.g[RA][0] = sx(0x001f5dd4);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x001f5dd4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f5de0);
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001f5de0)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f5e04;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f5dfc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd98));
                    call_ptr(c, 0x0039e948, 0x001f5dfc)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                    if t {
                        break 'b_1f5e14;
                    } else {
                        break 'b_1f5e04;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5e10);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd98));
            call_ptr(c, 0x0039e8b8, 0x001f5e10)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
            break 'b_1f5e14;
        }
        'b_1f5e48: {
            'b_1f5e38: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 24i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f5e38;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f5e30);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd88));
                    call_ptr(c, 0x0039e948, 0x001f5e30)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_1f5e48;
                    } else {
                        break 'b_1f5e38;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f5e44);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd88));
            call_ptr(c, 0x0039e8b8, 0x001f5e44)?;
            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1f5e48;
        }
        c.cycles += 8;
        c.g[V1][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[V0][0]);
        c.g[FP][0] = 0x3;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
        'l_1f5e68: loop {
            'b_1f5e74: {
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                if c.g[FP][0] == 0u64 {
                    break 'b_1f5e74;
                } else {
                    break 'b_1f5e74;
                }
            }
            'b_1f5f7c: {
                'b_1f5f78: {
                    c.cycles += 19;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S7][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S5][0] = c.g[A2][0];
                    c.div(0, 23, 30);
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2380));
                    c.g[S3][0] = c.lo[0];
                    c.g[S4][0] = c.hi[0];
                    c.g[RA][0] = sx(0x001f5e9c);
                    call_ptr(c, 0x00417828, 0x001f5e9c)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001f5ea8);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x164), 0u64);
                    call_ptr(c, 0x00317670, 0x001f5ea8)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f5eb4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f5eb4)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V1][0] = 0x440000;
                    if t {
                        break 'b_1f5f78;
                    } else {
                        'b_1f5f64: {
                            c.cycles += 32;
                            c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1ba0));
                            c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[A0][0] = c.g[S6][0];
                            c.g[V1][0] = 0x4a0000;
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                            c.g[RA][0] = sx(0x001f5ee4);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2388));
                            call_ptr(c, 0x00417828, 0x001f5ee4)?;
                            c.g[RA][0] = sx(0x001f5eec);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00317670, 0x001f5eec)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f5ef8);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x003a1000, 0x001f5ef8)?;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[V0][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd78));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f5f10);
                            c.g[A2][0] = c.g[S5][0];
                            call_ptr(c, 0x00417828, 0x001f5f10)?;
                            c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[V1][0] = 0x470000;
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd68));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f5f28);
                            c.g[A2][0] = c.g[S5][0];
                            call_ptr(c, 0x00417828, 0x001f5f28)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                            c.g[RA][0] = sx(0x001f5f34);
                            c.g[A1][0] = c.g[S3][0];
                            call_ptr(c, 0x001552b0, 0x001f5f34)?;
                            c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1f5f64;
                            } else {
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x001f5f48);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001f5f48)?;
                                c.g[V0][0] = 0x1;
                                c.g[A1][0] = c.g[S1][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001f5f5c);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e8b8, 0x001f5f5c)?;
                                let t = true;
                                c.g[V1][0] = 0x4a0000;
                                if t {
                                    break 'b_1f5f7c;
                                } else {
                                    break 'b_1f5f64;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001f5f6c);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x001f5f6c)?;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001f5f78);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e948, 0x001f5f78)?;
                        break 'b_1f5f78;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = 0x4a0000;
                break 'b_1f5f7c;
            }
            'b_1f6020: {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2390));
                c.g[RA][0] = sx(0x001f5f8c);
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001f5f8c)?;
                c.g[RA][0] = sx(0x001f5f94);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001f5f94)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001f5fa0);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001f5fa0)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[V1][0] = 0x470000;
                if t {
                    break 'b_1f6020;
                } else {
                    c.cycles += 29;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x001f5fbc);
                    c.g[A2][0] = 0x8;
                    call_ptr(c, 0x003e6448, 0x001f5fbc)?;
                    c.g[V1][0] = 0x440000;
                    c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1b40));
                    c.g[T0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A3][0] = c.lwu((c.g[V0][0] as u32));
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.g[V1][0] = c.g[V1][0] >> 5;
                    c.g[T0][0] = c.g[T0][0] & c.g[V1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A3][0] = c.g[A3][0] << 59;
                    c.g[A2][0] = 0x64;
                    c.g[T0][0] = c.g[T0][0] | c.g[A3][0];
                    c.g[RA][0] = sx(0x001f5ffc);
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T0][0]);
                    call_ptr(c, 0x00116950, 0x001f5ffc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[V0][0] = 0x470000;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[RA][0] = sx(0x001f6010);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xd58));
                    call_ptr(c, 0x00417828, 0x001f6010)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    c.g[RA][0] = sx(0x001f601c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0c50, 0x001f601c)?;
                    c.g[V1][0] = 0x470000;
                    break 'b_1f6020;
                }
            }
            'b_1f60a4: {
                'b_1f60a0: {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xd40));
                    c.g[RA][0] = sx(0x001f6030);
                    c.g[A2][0] = c.g[S5][0];
                    call_ptr(c, 0x00417828, 0x001f6030)?;
                    c.g[RA][0] = sx(0x001f6038);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001f6038)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f6044);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f6044)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                    if t {
                        break 'b_1f60a0;
                    } else {
                        'b_1f6088: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x440000;
                            if t {
                                break 'b_1f6088;
                            } else {
                                c.cycles += 12;
                                c.g[V0][0] = sx((c.g[S7][0] as u32) << 2);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1b40));
                                c.g[A0][0] = c.g[S6][0];
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[A2][0] = 0x64;
                                c.g[RA][0] = sx(0x001f6074);
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                                call_ptr(c, 0x00116e08, 0x001f6074)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001f6080);
                                c.g[A1][0] = c.g[S6][0];
                                call_ptr(c, 0x003a0c50, 0x001f6080)?;
                                let t = true;
                                c.g[V1][0] = c.g[S5][0];
                                if t {
                                    break 'b_1f60a4;
                                } else {
                                    break 'b_1f6088;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001f6094);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd30));
                        call_ptr(c, 0x00317670, 0x001f6094)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001f60a0);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001f60a0)?;
                        break 'b_1f60a0;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.g[S5][0];
                break 'b_1f60a4;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
                if event_due(c) {
                    deliver_event_here(c, 0x001f5e68);
                }
                continue 'l_1f5e68;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x170));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateCareerStats_ShowRacePage(self)` (0x1f60e0, `setupRacePlatinumMedalsList`):
/// shows or hides the scroll arrows (`self+0x50`, `+5 < count (+0x58)`), prints the medal
/// count, and fills five rows (peak name, track, time, medal marker from the done mask at
/// +0x60) for the events in range; rows past the end are hidden.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_show_race_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f6154: {
            'b_1f6144: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.g[RA][0] = sx(0x001f6114);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x001f6114)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f6120);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001f6120)?;
                c.g[FP][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f6144;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f613c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd18));
                    call_ptr(c, 0x0039e948, 0x001f613c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1f6154;
                    } else {
                        break 'b_1f6144;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f6150);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd18));
            call_ptr(c, 0x0039e8b8, 0x001f6150)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
            break 'b_1f6154;
        }
        'b_1f618c: {
            'b_1f617c: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1f617c;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f6174);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd08));
                    call_ptr(c, 0x0039e948, 0x001f6174)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f618c;
                    } else {
                        break 'b_1f617c;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f6188);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd08));
            call_ptr(c, 0x0039e8b8, 0x001f6188)?;
            c.g[A0][0] = 0x470000;
            break 'b_1f618c;
        }
        c.cycles += 27;
        c.g[S5][0] = 0u64;
        c.g[RA][0] = sx(0x001f6198);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xcf8));
        call_ptr(c, 0x00317670, 0x001f6198)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x001f61a4);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001f61a4)?;
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
        c.g[A0][0] = c.g[SP][0];
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f61c0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2398));
        call_ptr(c, 0x00417828, 0x001f61c0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f61cc);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a0c50, 0x001f61cc)?;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xec), c.g[A0][0]);
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
        'l_1f61f8: loop {
            'b_1f6450: {
                'b_1f63d0: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                    c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_1f63d0;
                    } else {
                        'b_1f62a4: {
                            c.cycles += 38;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xce8));
                            c.g[RA][0] = sx(0x001f621c);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x00417828, 0x001f621c)?;
                            c.g[S0][0] = sx((c.g[S4][0] as u32) << 2);
                            c.g[RA][0] = sx(0x001f6228);
                            c.g[A0][0] = c.g[S7][0];
                            call_ptr(c, 0x00317670, 0x001f6228)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6234);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6234)?;
                            c.g[A0][0] = 0x480000;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7f10));
                            c.g[V0][0] = 0x4a0000;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32));
                            c.g[A0][0] = c.g[S6][0];
                            c.g[RA][0] = sx(0x001f625c);
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                            call_ptr(c, 0x00417828, 0x001f625c)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f6268);
                            c.g[A1][0] = c.g[S6][0];
                            call_ptr(c, 0x003a0c50, 0x001f6268)?;
                            c.g[A0][0] = 0x470000;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xcd8));
                            c.g[RA][0] = sx(0x001f627c);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            call_ptr(c, 0x00417828, 0x001f627c)?;
                            c.g[RA][0] = sx(0x001f6284);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            call_ptr(c, 0x00317670, 0x001f6284)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6290);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6290)?;
                            c.g[A0][0] = 0x3;
                            c.g[S1][0] = c.g[V0][0];
                            c.div(0, 20, 4);
                            if c.g[A0][0] == 0u64 {
                                break 'b_1f62a4;
                            } else {
                                break 'b_1f62a4;
                            }
                        }
                        'b_1f6344: {
                            'b_1f6308: {
                                c.cycles += 3;
                                c.g[V1][0] = c.hi[0];
                                if c.g[V1][0] != 0u64 {
                                    c.g[A0][0] = 0x480000;
                                    break 'b_1f6308;
                                } else {
                                    c.cycles += 22;
                                    c.g[V0][0] = 0x480000;
                                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7f30));
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[RA][0] = sx(0x001f62cc);
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    call_ptr(c, 0x00144c60, 0x001f62cc)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001f62e0);
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a0));
                                    call_ptr(c, 0x00417828, 0x001f62e0)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[RA][0] = sx(0x001f62f0);
                                    c.g[A2][0] = 0x4;
                                    call_ptr(c, 0x001456a0, 0x001f62f0)?;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001f6300);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x003a0c50, 0x001f6300)?;
                                    let t = true;
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_1f6344;
                                    } else {
                                        break 'b_1f6308;
                                    }
                                }
                            }
                            c.cycles += 15;
                            c.g[A2][0] = 0u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7f30));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            c.g[RA][0] = sx(0x001f6320);
                            c.g[A0][0] = c.g[FP][0];
                            call_ptr(c, 0x001456a0, 0x001f6320)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A0][0] = c.g[FP][0];
                            c.g[RA][0] = sx(0x001f6330);
                            c.g[S0][0] = c.g[V0][0];
                            call_ptr(c, 0x00144c60, 0x001f6330)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001f633c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001f633c)?;
                            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                            c.g[V0][0] = 0x4a0000;
                            break 'b_1f6344;
                        }
                        'b_1f63c0: {
                            c.cycles += 27;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a8));
                            c.g[RA][0] = sx(0x001f6354);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x001f6354)?;
                            c.g[RA][0] = sx(0x001f635c);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x00317670, 0x001f635c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6368);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6368)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f6374);
                            c.g[S1][0] = c.g[V0][0];
                            call_ptr(c, 0x0020a1a8, 0x001f6374)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f6380);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x001f6380)?;
                            c.g[A0][0] = 0x470000;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xcc8));
                            c.g[A2][0] = c.g[S5][0];
                            c.g[RA][0] = sx(0x001f6398);
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x00417828, 0x001f6398)?;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S4][0] as u32) & 31));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1f63c0;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001f63b8);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001f63b8)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f6450;
                                } else {
                                    break 'b_1f63c0;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001f63c8);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x001f63c8)?;
                        let t = true;
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_1f6450;
                        } else {
                            break 'b_1f63d0;
                        }
                    }
                }
                c.cycles += 32;
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S7][0];
                c.g[RA][0] = sx(0x001f63e0);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xce8));
                call_ptr(c, 0x00417828, 0x001f63e0)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f63ec);
                c.g[A1][0] = c.g[S7][0];
                call_ptr(c, 0x0039e8b8, 0x001f63ec)?;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S6][0];
                c.g[RA][0] = sx(0x001f6400);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xcd8));
                call_ptr(c, 0x00417828, 0x001f6400)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f640c);
                c.g[A1][0] = c.g[S6][0];
                call_ptr(c, 0x0039e8b8, 0x001f640c)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.g[V0][0] = 0x4a0000;
                c.g[A2][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x001f6420);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x23a8));
                call_ptr(c, 0x00417828, 0x001f6420)?;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                c.g[RA][0] = sx(0x001f642c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f642c)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xcc8));
                c.g[RA][0] = sx(0x001f6440);
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001f6440)?;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[RA][0] = sx(0x001f644c);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f644c)?;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                break 'b_1f6450;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S5][0] as i64) < 5i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
                if event_due(c) {
                    deliver_event_here(c, 0x001f61f8);
                }
                continue 'l_1f61f8;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEStateCareerStats_ShowFreestylePage(self)` (0x1f6490, `setupFSPlatinumMedalsList`):
/// like the race page for the freestyle events (`self+0x54` scroll, `count` +0x68, medals
/// +0x6c, done mask +0x70).
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn cfe_state_career_stats_show_freestyle_page(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f6504: {
            'b_1f64f4: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.g[RA][0] = sx(0x001f64c4);
                c.g[S2][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x001f64c4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001f64d0);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001f64d0)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_1f64f4;
                } else {
                    c.cycles += 5;
                    c.g[A1][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f64ec);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xcb8));
                    call_ptr(c, 0x0039e948, 0x001f64ec)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    if t {
                        break 'b_1f6504;
                    } else {
                        break 'b_1f64f4;
                    }
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0x470000;
            c.g[RA][0] = sx(0x001f6500);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xcb8));
            call_ptr(c, 0x0039e8b8, 0x001f6500)?;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
            break 'b_1f6504;
        }
        'b_1f653c: {
            'b_1f652c: {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1f652c;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001f6524);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xca8));
                    call_ptr(c, 0x0039e948, 0x001f6524)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1f653c;
                    } else {
                        break 'b_1f652c;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001f6538);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xca8));
            call_ptr(c, 0x0039e8b8, 0x001f6538)?;
            c.g[A0][0] = 0x470000;
            break 'b_1f653c;
        }
        c.cycles += 27;
        c.g[S5][0] = 0u64;
        c.g[RA][0] = sx(0x001f6548);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xc98));
        call_ptr(c, 0x00317670, 0x001f6548)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x001f6554);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001f6554)?;
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6c));
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f6570);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2398));
        call_ptr(c, 0x00417828, 0x001f6570)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001f657c);
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a0c50, 0x001f657c)?;
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[V0][0]);
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0xec), c.g[V0][0]);
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
        'l_1f65a8: loop {
            'b_1f6804: {
                'b_1f6780: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        break 'b_1f6780;
                    } else {
                        'b_1f66ec: {
                            'b_1f66a4: {
                                c.cycles += 37;
                                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                c.g[V1][0] = 0x470000;
                                if c.g[V0][0] != 0 {
                                    c.g[S0][0] = c.g[S3][0];
                                }
                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc88));
                                c.g[A2][0] = c.g[S5][0];
                                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 2) as u32);
                                c.g[RA][0] = sx(0x001f65e0);
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x00417828, 0x001f65e0)?;
                                c.g[RA][0] = sx(0x001f65e8);
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x00317670, 0x001f65e8)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001f65f4);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001f65f4)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V0][0] = 0x4a0000;
                                c.g[A0][0] = c.g[S7][0];
                                c.g[RA][0] = sx(0x001f660c);
                                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                                call_ptr(c, 0x00417828, 0x001f660c)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001f6618);
                                c.g[A1][0] = c.g[S7][0];
                                call_ptr(c, 0x003a0c50, 0x001f6618)?;
                                c.g[V1][0] = 0x470000;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc78));
                                c.g[RA][0] = sx(0x001f662c);
                                c.g[A2][0] = c.g[S5][0];
                                call_ptr(c, 0x00417828, 0x001f662c)?;
                                c.g[RA][0] = sx(0x001f6634);
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                call_ptr(c, 0x00317670, 0x001f6634)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001f6640);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001f6640)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[V0][0] = c.g[S3][0] & 0x3;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x480000;
                                if t {
                                    break 'b_1f66a4;
                                } else {
                                    c.cycles += 21;
                                    c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.g[RA][0] = sx(0x001f666c);
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    call_ptr(c, 0x00144c60, 0x001f666c)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.g[V1][0] = 0x4a0000;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001f6680);
                                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x23b0));
                                    call_ptr(c, 0x00417828, 0x001f6680)?;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe8));
                                    c.g[RA][0] = sx(0x001f668c);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x003a0c50, 0x001f668c)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[A2][0] = 0x5;
                                    c.g[RA][0] = sx(0x001f669c);
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                    call_ptr(c, 0x001456a0, 0x001f669c)?;
                                    let t = true;
                                    c.g[V1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1f66ec;
                                    } else {
                                        break 'b_1f66a4;
                                    }
                                }
                            }
                            c.cycles += 18;
                            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ef0));
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[RA][0] = sx(0x001f66bc);
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            call_ptr(c, 0x00144c60, 0x001f66bc)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f66c8);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x001f66c8)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            c.g[RA][0] = sx(0x001f66d4);
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            call_ptr(c, 0x00145870, 0x001f66d4)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f66e4);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                            call_ptr(c, 0x001456a0, 0x001f66e4)?;
                            c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                            c.g[V1][0] = 0x4a0000;
                            break 'b_1f66ec;
                        }
                        'b_1f6770: {
                            c.cycles += 29;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2138));
                            c.g[RA][0] = sx(0x001f66fc);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x00417828, 0x001f66fc)?;
                            c.g[V0][0] = 0x470000;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc68));
                            c.g[RA][0] = sx(0x001f6710);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00417828, 0x001f6710)?;
                            c.g[RA][0] = sx(0x001f6718);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x00317670, 0x001f6718)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001f6724);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001f6724)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001f6730);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x003a0c50, 0x001f6730)?;
                            c.g[V1][0] = 0x470000;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc58));
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f6748);
                            c.g[A2][0] = c.g[S5][0];
                            call_ptr(c, 0x00417828, 0x001f6748)?;
                            c.g[V1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x70));
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << ((c.g[S3][0] as u32) & 31));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1f6770;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001f6768);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x0039e948, 0x001f6768)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f6804;
                                } else {
                                    break 'b_1f6770;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001f6778);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039e8b8, 0x001f6778)?;
                        let t = true;
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                        if t {
                            break 'b_1f6804;
                        } else {
                            break 'b_1f6780;
                        }
                    }
                }
                c.cycles += 33;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[A0][0] = c.g[FP][0];
                c.g[RA][0] = sx(0x001f6794);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc88));
                call_ptr(c, 0x00417828, 0x001f6794)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f67a0);
                c.g[A1][0] = c.g[FP][0];
                call_ptr(c, 0x0039e8b8, 0x001f67a0)?;
                c.g[V1][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc78));
                c.g[RA][0] = sx(0x001f67b4);
                c.g[A0][0] = c.g[S7][0];
                call_ptr(c, 0x00417828, 0x001f67b4)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f67c0);
                c.g[A1][0] = c.g[S7][0];
                call_ptr(c, 0x0039e8b8, 0x001f67c0)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x001f67d4);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc68));
                call_ptr(c, 0x00417828, 0x001f67d4)?;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xec));
                c.g[RA][0] = sx(0x001f67e0);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001f67e0)?;
                c.g[V1][0] = 0x470000;
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xc58));
                c.g[RA][0] = sx(0x001f67f4);
                c.g[A2][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001f67f4)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001f6800);
                c.g[A1][0] = c.g[S6][0];
                call_ptr(c, 0x0039e8b8, 0x001f6800)?;
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                break 'b_1f6804;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S5][0] as i64) < 5i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                if event_due(c) {
                    deliver_event_here(c, 0x001f65a8);
                }
                continue 'l_1f65a8;
            } else {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::cfe_state_career_stats_field as field;
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine, FAKE};
    use crate::difftest::{Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes in the machine's RAM (both sides of a
    /// diff get their own machine); the fakes fall back when it is not set.
    const OC985_CTRL: u32 = DATA + 0x20_0000;
    const OC985_MAGIC: u32 = 0x0985_f56c;
    /// Bit n set: the n-th `UIScreen_FindObject` call finds a widget.
    const OC985_FIND_MASK: u32 = OC985_CTRL + 4;
    const OC985_FIND_N: u32 = OC985_CTRL + 8;
    /// Forced results (result + 1; 0 = the fake's default) per mailbox.
    const OC985_MAIL: u32 = OC985_CTRL + 0x10;
    const OC985_MAIL_LOOKUP: u32 = 0;
    const OC985_MAIL_LEVEL: u32 = 1;
    const OC985_NO_MAIL: u32 = u32::MAX;
    /// Eight widgets (0x100 apart) sharing one vtable whose `+0x38/+0x3c`
    /// slot (`setFocus`) is a fake call.
    const OC985_OBJ: u32 = DATA + 0x20_1000;
    const OC985_OBJ_VT: u32 = DATA + 0x20_2000;
    const OC985_THIS: u32 = DATA + 0x1000;
    const OC985_SCREEN: u32 = DATA + 0x2000;
    const OC985_ENG: u32 = DATA + 0x3000;
    const OC985_SOUND: u32 = DATA + 0x3100;
    const OC985_SOUND_VT: u32 = DATA + 0x3200;
    const OC985_W: u32 = DATA + 0x3300;
    const OC985_WVT: u32 = DATA + 0x3400;
    /// A real stub that returns 0 (a vtable slot answering false).
    const OC985_RET_ZERO: u32 = 0x0014_06c0;

    /// The fake callee: logs the call like `fakes::fake_call`, then applies the
    /// forced result of its mailbox, or (for `UIScreen_FindObject`) finds a
    /// widget or nothing according to the mask.
    fn oc985_fake_call(c: &mut Cpu, addr: u32, mail: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if mail != OC985_NO_MAIL {
            let m = c.bus.read32(OC985_MAIL + 4 * mail);
            if m != 0 {
                c.g[V0][0] = (m - 1) as u64;
            }
        }
        if addr == 0x0039_d860 {
            let n = c.bus.read32(OC985_FIND_N);
            c.bus.write32(OC985_FIND_N, n.wrapping_add(1));
            let mask = c.bus.read32(OC985_FIND_MASK);
            c.g[V0][0] = if (mask >> (n % 32)) & 1 != 0 {
                sx(OC985_OBJ + 0x100 * (n % 8))
            } else {
                0
            };
        }
    }

    macro_rules! oc985_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mail:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC985_CTRL) != OC985_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc985_fake_call(c, $addr, $mail);
                }
            )*
            fn oc985_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc985_fakes!(
        (
            oc985_fake_317670,
            OC985_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39d860,
            OC985_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39e8b8,
            OC985_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39e948,
            OC985_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_3a0c50,
            OC985_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_3a1000,
            OC985_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_39c778,
            OC985_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778,
            OC985_MAIL_LOOKUP
        ),
        (
            oc985_fake_39c870,
            OC985_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_417828,
            OC985_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_3e6448,
            OC985_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_14dc80,
            OC985_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_14dd58,
            OC985_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1552b0,
            OC985_PREV_1552B0,
            ee::s01::OVERRIDE_001552B0,
            ee::s01::f_001552b0__generated,
            0x0015_52b0,
            OC985_MAIL_LEVEL
        ),
        (
            oc985_fake_144c60,
            OC985_PREV_144C60,
            ee::s01::OVERRIDE_00144C60,
            ee::s01::f_00144c60__generated,
            0x0014_4c60,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1456a0,
            OC985_PREV_1456A0,
            ee::s01::OVERRIDE_001456A0,
            ee::s01::f_001456a0__generated,
            0x0014_56a0,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_145870,
            OC985_PREV_145870,
            ee::s01::OVERRIDE_00145870,
            ee::s01::f_00145870__generated,
            0x0014_5870,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_116950,
            OC985_PREV_116950,
            ee::s00::OVERRIDE_00116950,
            ee::s00::f_00116950__generated,
            0x0011_6950,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_116e08,
            OC985_PREV_116E08,
            ee::s00::OVERRIDE_00116E08,
            ee::s00::f_00116e08__generated,
            0x0011_6e08,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_20a1a8,
            OC985_PREV_20A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020_a1a8,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f5a38,
            OC985_PREV_1F5A38,
            ee::s04::OVERRIDE_001F5A38,
            ee::s04::f_001f5a38__generated,
            0x001f_5a38,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f5ba8,
            OC985_PREV_1F5BA8,
            ee::s04::OVERRIDE_001F5BA8,
            ee::s04::f_001f5ba8__generated,
            0x001f_5ba8,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f5da0,
            OC985_PREV_1F5DA0,
            ee::s04::OVERRIDE_001F5DA0,
            ee::s04::f_001f5da0__generated,
            0x001f_5da0,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f60e0,
            OC985_PREV_1F60E0,
            ee::s04::OVERRIDE_001F60E0,
            ee::s04::f_001f60e0__generated,
            0x001f_60e0,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f6490,
            OC985_PREV_1F6490,
            ee::s04::OVERRIDE_001F6490,
            ee::s04::f_001f6490__generated,
            0x001f_6490,
            OC985_NO_MAIL
        ),
        (
            oc985_fake_1f6890,
            OC985_PREV_1F6890,
            ee::s04::OVERRIDE_001F6890,
            ee::s04::f_001f6890__generated,
            0x001f_6890,
            OC985_NO_MAIL
        ),
    );

    /// A scrambled machine with the fakes' gate open, widgets for the finder
    /// (found per the mask), and a career stats object with random list
    /// positions at `OC985_THIS`: the highlights scroll (0..23), the race and
    /// freestyle scrolls with a total around `scroll + 5`, medal counts and
    /// done masks.
    fn oc985_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc985_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC985_CTRL, OC985_MAGIC);
        c.bus.write32(OC985_FIND_N, 0);
        let mask = r.next_u32() | r.next_u32();
        c.bus.write32(OC985_FIND_MASK, mask);
        for k in 0..8 {
            c.bus.write32(OC985_OBJ + 0x100 * k + 8, OC985_OBJ_VT);
        }
        c.bus
            .write16(OC985_OBJ_VT + 0x38, (r.next_u32() & 0x30) as u16);
        c.bus.write32(OC985_OBJ_VT + 0x3c, FAKE);
        // 0x39c778 finds the frame (5) or not (0xffff); the highlight level 0..5.
        let lookup = if r.next_u32() % 2 == 0 { 0x1_0000 } else { 6 };
        c.bus.write32(OC985_MAIL + 4 * OC985_MAIL_LOOKUP, lookup);
        c.bus
            .write32(OC985_MAIL + 4 * OC985_MAIL_LEVEL, 1 + r.next_u32() % 6);

        let this = OC985_THIS;
        c.bus.write32(this + field::ENGINE, OC985_ENG);
        c.bus.write32(this + field::SCREEN, OC985_SCREEN);
        c.bus.write32(this + field::STATS_STATE, r.next_u32() % 4);
        const HIGHLIGHT_SCROLLS: [u32; 7] = [0, 0, 1, 5, 20, 21, 23];
        let hl = HIGHLIGHT_SCROLLS[(r.next_u32() % 7) as usize];
        c.bus.write32(this + field::TOP_HIGHLIGHT, hl);
        for (top, total, medals, done) in [
            (
                field::TOP_RACE,
                field::TOTAL_RACE,
                field::RACE_MEDALS,
                field::RACE_DONE,
            ),
            (
                field::TOP_FREESTYLE,
                field::TOTAL_FREESTYLE,
                field::FREESTYLE_MEDALS,
                field::FREESTYLE_DONE,
            ),
        ] {
            let scroll = if r.next_u32() % 4 == 0 {
                0
            } else {
                r.next_u32() % 12
            };
            let count = if r.next_u32() % 6 == 0 {
                r.next_u32() % 4
            } else {
                scroll + 3 + r.next_u32() % 5
            };
            c.bus.write32(this + top, scroll);
            c.bus.write32(this + total, count);
            c.bus.write32(this + medals, r.next_u32() % 20);
            c.bus.write32(this + done, r.next_u32());
        }
        r
    }

    /// `HandleInput`: left/right (neither, one) x up/down (none, up, down) in
    /// each of the four pages; the fake vtable slots answer true (a fake
    /// pointer) or false (a real leaf returning 0); the UI sound object is
    /// there or the null sound is used; fakes unwind at random calls.
    #[test]
    fn cfe_state_career_stats_handle_input_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f56c8__generated,
            cfe_state_career_stats_handle_input,
            320,
            &[
                0x001f_57e0, // left/right: SelectPage
                0x001f_582c, // highlights up
                0x001f_5874, // highlights down
                0x001f_58c0, // race up
                0x001f_590c, // race down
                0x001f_5958, // freestyle up
                0x001f_59a4, // freestyle down
            ],
            &|c, seed| {
                let mut r = oc985_fixture(c, seed, 0x001f_56c8, 14);
                let this = OC985_THIS;
                c.bus.write32(this + field::STATS_STATE, (seed / 20) % 4);
                c.bus.write32(OC985_W + 8, OC985_WVT);
                let lr = seed % 5; // 0: left, 1: right, else neither
                let ud = (seed / 5) % 4; // bit 0: up, bit 1: down
                let want = [
                    (0x98u32, lr == 0),
                    (0xa0, lr == 1),
                    (0x88, ud & 1 != 0),
                    (0x90, ud & 2 != 0),
                ];
                for (off, yes) in want {
                    c.bus.write16(OC985_WVT + off, 0);
                    c.bus
                        .write32(OC985_WVT + off + 4, if yes { FAKE } else { OC985_RET_ZERO });
                }
                // The UI sound object, or none (the null sound at gp+0x2968).
                if r.next_u32() % 4 != 0 {
                    c.bus.write32(OC985_ENG + 0x14, OC985_SOUND);
                    c.bus.write32(OC985_SOUND, OC985_SOUND_VT);
                    c.bus.write16(OC985_SOUND_VT + 0x10, 0);
                    c.bus.write32(OC985_SOUND_VT + 0x14, FAKE);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(OC985_W);
            },
        );
    }

    /// `SelectPage`: the page (0..5: four pages and out of range) is stored,
    /// the four tab widgets (found or not) lose the focus, the selected one
    /// gains it and its page is built.
    #[test]
    fn cfe_state_career_stats_select_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f5ba8__generated,
            cfe_state_career_stats_select_page,
            200,
            &[0x001f_5d0c, 0x001f_5d34, 0x001f_5d5c, 0x001f_5d80],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_5ba8, 12);
                c.g[A0][0] = sx(OC985_THIS);
                c.g[A1][0] = (seed % 6) as u64;
            },
        );
    }

    /// `ShowHighlightsPage`: scroll arrows, three rows of step name, on/off
    /// marker (level 0..5 against the step), trick name and required-trick or
    /// locked text, with widgets found or missing.
    #[test]
    fn cfe_state_career_stats_show_highlights_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f5da0__generated,
            cfe_state_career_stats_show_highlights_page,
            300,
            &[
                0x001f_5f5c,
                0x001f_5f6c,
                0x001f_601c,
                0x001f_6074,
                0x001f_60a0,
            ],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_5da0, 40);
                c.g[A0][0] = sx(OC985_THIS);
            },
        );
    }

    /// `ShowRacePage`: scroll arrows, medal count and five rows (events in
    /// range with the two track-name variants and the done marker, rows past
    /// the end hidden).
    #[test]
    fn cfe_state_career_stats_show_race_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f60e0__generated,
            cfe_state_career_stats_show_race_page,
            300,
            &[
                0x001f_62cc,
                0x001f_6320,
                0x001f_63b8,
                0x001f_63c8,
                0x001f_63e0,
            ],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_60e0, 40);
                c.g[A0][0] = sx(OC985_THIS);
            },
        );
    }

    /// `ShowFreestylePage`: the same for the freestyle events.
    #[test]
    fn cfe_state_career_stats_show_freestyle_page_matches() {
        oc985_install_fakes();
        check_reaching(
            ee::s04::f_001f6490__generated,
            cfe_state_career_stats_show_freestyle_page,
            300,
            &[
                0x001f_666c,
                0x001f_66bc,
                0x001f_6768,
                0x001f_6778,
                0x001f_67a0,
            ],
            &|c, seed| {
                oc985_fixture(c, seed, 0x001f_6490, 40);
                c.g[A0][0] = sx(OC985_THIS);
            },
        );
    }
}
