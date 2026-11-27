//! Staging module oc988 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc988`.

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
        0x0020_8f88,
        ov_screen_session_update_confirm_popup,
        "OVScreenSession_UpdateConfirmPopup",
    ),
    (
        0x0020_96a8,
        ov_screen_session_create_player_indicator,
        "OVScreenSession_CreatePlayerIndicator",
    ),
    (
        0x0020_9970,
        ov_screen_session_create_map_indicators,
        "OVScreenSession_CreateMapIndicators",
    ),
    (
        0x0021_0940,
        ov_screen_read_private_message_init,
        "OVScreenReadPrivateMessage_Init",
    ),
    (
        0x0021_1380,
        ov_screen_profile_save_pda_refresh_card_info,
        "OVScreenProfileSavePDA_RefreshCardInfo",
    ),
];

/// `void OVScreenSession_UpdateConfirmPopup(OVScreenSession* self)` (0x208f88): the map screen's
/// confirm popup. With `self->+0xbc` set it shows the popup widgets, fills the
/// text and money fields (`UStr_SprintfAsciiFmt_2`, `sprintf`, the string manager's
/// lookup through the world), hides the others and shows `0x4706c8`; otherwise it
/// hides the popup and toggles the menu and list widgets.
pub fn ov_screen_session_update_confirm_popup(c: &mut Cpu) {
    guest(c, |c| {
        'b_2092e4: {
            'b_20929c: {
                c.cycles += 24;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A0][0] = 0x470000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
                c.g[RA][0] = sx(0x00208fb0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xa20));
                call_ptr(c, 0x00317670, 0x00208fb0)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                c.g[RA][0] = sx(0x00208fbc);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00208fbc)?;
                c.g[A0][0] = 0x470000;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x00208fcc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1850));
                call_ptr(c, 0x00317670, 0x00208fcc)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                c.g[RA][0] = sx(0x00208fd8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x00208fd8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xbc));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if t {
                    break 'b_20929c;
                } else {
                    'b_209004: {
                        c.cycles += 5;
                        c.g[A1][0] = 0x470000;
                        c.g[RA][0] = sx(0x00208ff4);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa60));
                        call_ptr(c, 0x0039e948, 0x00208ff4)?;
                        let t = c.g[S1][0] == 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        if t {
                            break 'b_209004;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x00209004);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0039b960, 0x00209004)?;
                            break 'b_209004;
                        }
                    }
                    'b_209048: {
                        c.cycles += 11;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2638));
                        c.g[RA][0] = sx(0x00209018);
                        c.g[S3][0] = 0u64;
                        call_ptr(c, 0x0039e948, 0x00209018)?;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x00209028);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2640));
                        call_ptr(c, 0x0039e8b8, 0x00209028)?;
                        let t = c.g[S0][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_209048;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209044);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00209044)?;
                            c.g[S3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                            break 'b_209048;
                        }
                    }
                    'b_20907c: {
                        c.cycles += 2;
                        let t = c.g[S1][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_20907c;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209064);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00209064)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x0020907c);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0020907c)?;
                            break 'b_20907c;
                        }
                    }
                    'b_2090f4: {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1818));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x0020909c);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x0020909c)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002090ac);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x002090ac)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.g[V0][0];
                        if t {
                            break 'b_2090f4;
                        } else {
                            c.cycles += 13;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x26f0));
                            c.g[RA][0] = sx(0x002090c8);
                            c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            call_ptr(c, 0x002c27c0, 0x002090c8)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x002090d4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1890));
                            call_ptr(c, 0x00317670, 0x002090d4)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                            c.g[RA][0] = sx(0x002090e0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x002090e0)?;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_2090f4;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x002090f4);
                                c.g[A1][0] = c.g[SP][0];
                                call_ptr(c, 0x003a0e90, 0x002090f4)?;
                                break 'b_2090f4;
                            }
                        }
                    }
                    'b_209174: {
                        c.cycles += 28;
                        c.g[RA][0] = sx(0x002090fc);
                        call_ptr(c, 0x0014dc80, 0x002090fc)?;
                        c.g[A1][0] = 0xb;
                        c.g[RA][0] = sx(0x00209108);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00209108)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00209114);
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014a080, 0x00209114)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00209124);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00150928, 0x00209124)?;
                        c.g[V1][0] = 0x4a0000;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x26f8));
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00209140);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00417828, 0x00209140)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x0020914c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18a0));
                        call_ptr(c, 0x00317670, 0x0020914c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209158);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209158)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A1][0] = c.g[S3][0];
                            break 'b_209174;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00209170);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x00209170)?;
                            c.g[A1][0] = c.g[S3][0];
                            break 'b_209174;
                        }
                    }
                    'b_2091b4: {
                        c.cycles += 12;
                        c.g[A2][0] = 0x1f4;
                        c.g[RA][0] = sx(0x00209180);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00417828, 0x00209180)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x0020918c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18b8));
                        call_ptr(c, 0x00317670, 0x0020918c)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209198);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209198)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x470000;
                            break 'b_2091b4;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x002091b0);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x002091b0)?;
                            c.g[A0][0] = 0x470000;
                            break 'b_2091b4;
                        }
                    }
                    'b_2091f0: {
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x002091bc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c8));
                        call_ptr(c, 0x00317670, 0x002091bc)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x002091c8);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x002091c8)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A1][0] = 0x470000;
                            break 'b_2091f0;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x002091e0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18d8));
                            call_ptr(c, 0x00317670, 0x002091e0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x002091ec);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x002091ec)?;
                            c.g[A1][0] = 0x470000;
                            break 'b_2091f0;
                        }
                    }
                    'b_209254: {
                        c.cycles += 19;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x002091fc);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                        call_ptr(c, 0x0039e8b8, 0x002091fc)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x00209208);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6b8));
                        call_ptr(c, 0x00317670, 0x00209208)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209214);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209214)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V0][0] = 0x470000;
                        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c8));
                        c.g[RA][0] = sx(0x00209228);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00317670, 0x00209228)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xd4));
                        c.g[RA][0] = sx(0x00209234);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00209234)?;
                        let t = c.g[S1][0] == 0u64;
                        c.g[S0][0] = c.g[V0][0];
                        if t {
                            break 'b_209254;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00209248);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xaa8));
                            call_ptr(c, 0x00317670, 0x00209248)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00209254);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x00209254)?;
                            break 'b_209254;
                        }
                    }
                    'b_209288: {
                        c.cycles += 2;
                        let t = c.g[S0][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_209288;
                        } else {
                            c.cycles += 11;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209270);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00209270)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209288);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00209288)?;
                            break 'b_209288;
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x00209294);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0039e948, 0x00209294)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    if t {
                        break 'b_2092e4;
                    } else {
                        break 'b_20929c;
                    }
                }
            }
            'b_2092c4: {
                c.cycles += 5;
                c.g[A1][0] = 0x470000;
                c.g[RA][0] = sx(0x002092a8);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa60));
                call_ptr(c, 0x0039e8b8, 0x002092a8)?;
                let t = c.g[S0][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_2092c4;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002092c4);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x002092c4)?;
                    break 'b_2092c4;
                }
            }
            'b_2092e0: {
                c.cycles += 2;
                let t = c.g[S1][0] == 0u64;
                c.g[A1][0] = 0u64;
                if t {
                    break 'b_2092e0;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002092e0);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x002092e0)?;
                    break 'b_2092e0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            break 'b_2092e4;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSession_CreatePlayerIndicator(OVScreenSession* self)` (0x2096a8): allocate
/// the player's marker icon, place it on the map image from the rider's position and the
/// track bounds (`BEMissionInt_GetSessionInfo`), scale it 17x17, give it its texture, hide
/// it and append it to the screen's object list.
pub fn ov_screen_session_create_player_indicator(c: &mut Cpu) {
    guest(c, |c| {
        'b_209780: {
            c.cycles += 54;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
            c.g[RA][0] = sx(0x002096c8);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x002096c8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002096d4);
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x002096d4)?;
            c.g[RA][0] = sx(0x002096dc);
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x002096dc)?;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002096e8);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x002096e8)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd0));
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[SP][0] as u32));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x2c));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00209730);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            call_ptr(c, t, 0x00209730)?;
            c.lq(3, (c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00209744);
            c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00144bc0, 0x00209744)?;
            c.g[A2][0] = c.lw((c.g[V0][0] as u32));
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00209754);
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x00154678, 0x00209754)?;
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
            c.sub_s(1, 1, 2);
            c.sub_s(0, 0, 2);
            c.mtc1(0, 5);
            c.div_s(4, 1, 0);
            c.c_lt_s(4, 5);
            if c.fcc() {
                c.neg_s(4, 4);
                break 'b_209780;
            } else {
                break 'b_209780;
            }
        }
        'b_2097bc: {
            c.cycles += 15;
            c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
            c.sub_s(2, 2, 3);
            c.g[AT][0] = 0x43b20000;
            c.mtc1(1, 0);
            c.sub_s(1, 1, 3);
            c.mul_s(0, 4, 0);
            c.div_s(2, 2, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(17, 1);
            c.c_lt_s(2, 5);
            if c.fcc() {
                c.neg_s(2, 2);
                break 'b_2097bc;
            } else {
                break 'b_2097bc;
            }
        }
        'b_209850: {
            'b_209838: {
                c.cycles += 27;
                c.g[AT][0] = 0x43850000;
                c.mtc1(1, 0);
                c.g[A1][0] = 0x470000;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x18f0));
                c.mul_s(0, 2, 0);
                c.g[A0][0] = 0x88;
                c.cvt_w_s(1, 0);
                c.mfc1(18, 1);
                c.g[RA][0] = sx(0x002097e8);
                c.g[A2][0] = 0x20000000;
                call_ptr(c, 0x00317d70, 0x002097e8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd4));
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x002097fc);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039fb30, 0x002097fc)?;
                c.g[V1][0] = 0x490000;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4aa8));
                c.g[RA][0] = sx(0x00209810);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                Callee {
                    addr: 0x0021ccc8,
                    f: ee::s05::UIIconFlags_ctor,
                }
                .call(c, 0x00209810)?;
                c.g[V0][0] = 0xffffffffffffffff;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0xe0), c.g[S0][0]);
                let t = (c.g[A0][0] as i64) < 0;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                if t {
                    break 'b_209838;
                } else {
                    c.cycles += 4;
                    c.mtc1(4, 1);
                    c.cvt_s_w(1, 1);
                    let t = true;

                    if t {
                        break 'b_209850;
                    } else {
                        break 'b_209838;
                    }
                }
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
            c.g[V0][0] = c.g[A0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.add_s(1, 1, 1);
            break 'b_209850;
        }
        'b_209898: {
            'b_209874: {
                c.cycles += 5;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 0);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x61));
                let t = (c.g[V0][0] as i64) < 0;
                c.sub_s(3, 1, 0);
                if t {
                    break 'b_209874;
                } else {
                    c.cycles += 4;
                    c.mtc1(2, 2);
                    c.cvt_s_w(2, 2);
                    let t = true;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    if t {
                        break 'b_209898;
                    } else {
                        break 'b_209874;
                    }
                }
            }
            c.cycles += 9;
            c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
            c.g[V0][0] = c.g[V0][0] & 0x1;
            c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
            c.mtc1(2, 2);
            c.cvt_s_w(2, 2);
            c.add_s(2, 2, 2);
            c.g[AT][0] = 0x40a00000;
            c.mtc1(1, 0);
            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
            break 'b_209898;
        }
        c.cycles += 53;
        c.sub_s(0, 2, 0);
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x57));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
        c.g[AT][0] = 0x41880000;
        c.mtc1(1, 0);
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x002098f4);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        call_ptr(c, t, 0x002098f4)?;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x00209900);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1908));
        call_ptr(c, 0x00317670, 0x00209900)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = sx(0x0020990c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x00399730, 0x0020990c)?;
        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00209928);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00209928)?;
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[V1][0] = 0xffffffffffffe0ff;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[V0][0] = c.g[V0][0] | 0xd00;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xd4));
        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe0));
        c.g[RA][0] = sx(0x00209950);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
        call_ptr(c, 0x00397910, 0x00209950)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSession_CreateMapIndicators(OVScreenSession* self)` (0x209970): create one
/// marker icon per path marker of the track (`PathData_HasMarker`/`FindMarker`), each
/// placed on the map image from the track bounds, plus the second set of markers, and
/// append them to the screen's object list.
pub fn ov_screen_session_create_map_indicators(c: &mut Cpu) {
    guest(c, |c| {
        'b_209e40: {
            c.cycles += 40;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S5][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S7][0] = 0u64;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x118));
            c.g[RA][0] = sx(0x002099b0);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
            call_ptr(c, 0x0014dc80, 0x002099b0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002099bc);
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0014dd58, 0x002099bc)?;
            c.g[RA][0] = sx(0x002099c4);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x002099c4)?;
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x002099d0);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x002099d0)?;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x64), c.g[V0][0]);
            c.g[RA][0] = sx(0x002099dc);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x00144bc0, 0x002099dc)?;
            c.g[A1][0] = c.lw((c.g[V0][0] as u32));
            c.g[RA][0] = sx(0x002099e8);
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
            call_ptr(c, 0x001545f8, 0x002099e8)?;
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd0));
            c.sw((c.g[S5][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x4b));
            c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x44));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.sdr(4, (c.g[SP][0] as u32));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
            let t = (c.g[V0][0] as i64) <= 0;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            if t {
                break 'b_209e40;
            } else {
                c.cycles += 6;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 21);
                c.g[AT][0] = 0x41900000;
                c.mtc1(1, 20);
                c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                'l_209a28: loop {
                    'b_209e28: {
                        c.cycles += 7;
                        c.g[A0][0] = 0x4d0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[A2][0]);
                        c.g[RA][0] = sx(0x00209a3c);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0026b598, 0x00209a3c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
                        if t {
                            break 'b_209e28;
                        } else {
                            'b_209ab4: {
                                c.cycles += 28;
                                c.g[A0][0] = 0x4d0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x33a0));
                                c.g[RA][0] = sx(0x00209a54);
                                c.g[A1][0] = 0x2;
                                call_ptr(c, 0x0026b5e0, 0x00209a54)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
                                c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0xf));
                                c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
                                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                c.g[RA][0] = sx(0x00209a78);
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                call_ptr(c, 0x00144bc0, 0x00209a78)?;
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00209a88);
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                                call_ptr(c, 0x00154678, 0x00209a88)?;
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x3c));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                                c.sub_s(1, 1, 2);
                                c.sub_s(0, 0, 2);
                                c.mtc1(0, 5);
                                c.div_s(4, 1, 0);
                                c.c_lt_s(4, 5);
                                if c.fcc() {
                                    c.neg_s(4, 4);
                                    break 'b_209ab4;
                                } else {
                                    break 'b_209ab4;
                                }
                            }
                            'b_209af0: {
                                c.cycles += 15;
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                                c.sub_s(2, 2, 3);
                                c.g[AT][0] = 0x43b20000;
                                c.mtc1(1, 0);
                                c.sub_s(1, 1, 3);
                                c.mul_s(0, 4, 0);
                                c.div_s(2, 2, 1);
                                c.cvt_w_s(1, 0);
                                c.mfc1(20, 1);
                                c.c_lt_s(2, 5);
                                if c.fcc() {
                                    c.neg_s(2, 2);
                                    break 'b_209af0;
                                } else {
                                    break 'b_209af0;
                                }
                            }
                            'b_209b9c: {
                                'b_209b80: {
                                    c.cycles += 32;
                                    c.g[AT][0] = 0x43850000;
                                    c.mtc1(1, 0);
                                    c.g[A1][0] = 0x470000;
                                    c.g[A3][0] = 0u64;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1918));
                                    c.mul_s(0, 2, 0);
                                    c.g[A0][0] = 0x88;
                                    c.g[A2][0] = 0x20000000;
                                    c.g[S1][0] = 0x490000;
                                    c.g[S2][0] = sx((c.g[S7][0] as u32) << 2);
                                    c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add(0xe4));
                                    c.cvt_w_s(1, 0);
                                    c.mfc1(22, 1);
                                    c.g[RA][0] = sx(0x00209b2c);
                                    c.g[FP][0] = c.g[S2][0];
                                    call_ptr(c, 0x00317d70, 0x00209b2c)?;
                                    c.g[S2][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00209b44);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0039fb30, 0x00209b44)?;
                                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xfc));
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4aa8));
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                                    c.g[RA][0] = sx(0x00209b58);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                                    Callee {
                                        addr: 0x0021ccc8,
                                        f: ee::s05::UIIconFlags_ctor,
                                    }
                                    .call(c, 0x00209b58)?;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                    c.sw((c.g[S2][0] as u32), c.g[S0][0]);
                                    let t = (c.g[S4][0] as i64) < 0;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                                    if t {
                                        break 'b_209b80;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(20, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(2, 0, 21);
                                        if t {
                                            break 'b_209b9c;
                                        } else {
                                            break 'b_209b80;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V0][0] = c.g[S4][0] & 0x1;
                                c.g[V1][0] = sx((c.g[S4][0] as u32) >> 1);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.sub_s(2, 0, 21);
                                break 'b_209b9c;
                            }
                            'b_209bd0: {
                                'b_209bb8: {
                                    c.cycles += 3;
                                    c.g[S3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x61));
                                    let t = (c.g[S3][0] as i64) < 0;
                                    c.g[V0][0] = c.g[S3][0] & 0x1;
                                    if t {
                                        break 'b_209bb8;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(19, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(0, 0, 21);
                                        if t {
                                            break 'b_209bd0;
                                        } else {
                                            break 'b_209bb8;
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) >> 1);
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.sub_s(0, 0, 21);
                                break 'b_209bd0;
                            }
                            'b_209d20: {
                                'b_209d04: {
                                    c.cycles += 73;
                                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                    c.g[S0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                                    c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x104));
                                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x57));
                                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
                                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x4b));
                                    c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x44));
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4c), c.g[A0][0]);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00209c30);
                                    c.g[S2][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                                    call_ptr(c, t, 0x00209c30)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x00209c3c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1930));
                                    call_ptr(c, 0x00317670, 0x00209c3c)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[RA][0] = sx(0x00209c48);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x00399730, 0x00209c48)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00209c64);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x00209c64)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[V1][0] = 0xffffffffffffe0ff;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                    c.g[V0][0] = c.g[V0][0] | 0xb00;
                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                    c.g[RA][0] = sx(0x00209c8c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
                                    call_ptr(c, 0x00397910, 0x00209c8c)?;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A3][0] = 0u64;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1940));
                                    c.g[A0][0] = 0x88;
                                    c.g[RA][0] = sx(0x00209ca4);
                                    c.g[A2][0] = 0x20000000;
                                    call_ptr(c, 0x00317d70, 0x00209ca4)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00209cb8);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0039fb30, 0x00209cb8)?;
                                    c.g[T0][0] = 0x490000;
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                                    c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x4aa8));
                                    c.g[RA][0] = sx(0x00209ccc);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                                    Callee {
                                        addr: 0x0021ccc8,
                                        f: ee::s05::UIIconFlags_ctor,
                                    }
                                    .call(c, 0x00209ccc)?;
                                    c.g[V0][0] = 0xc;
                                    c.sw((c.g[S2][0] as u32), c.g[S0][0]);
                                    c.mult(0, 4, 23, 2);
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), 0u64);
                                    c.g[V0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                                    let t = (c.g[S4][0] as i64) < 0;
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x124));
                                    if t {
                                        break 'b_209d04;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(20, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(2, 0, 21);
                                        if t {
                                            break 'b_209d20;
                                        } else {
                                            break 'b_209d04;
                                        }
                                    }
                                }
                                c.cycles += 7;
                                c.g[V1][0] = sx((c.g[S4][0] as u32) >> 1);
                                c.g[V0][0] = c.g[S4][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.sub_s(2, 0, 21);
                                break 'b_209d20;
                            }
                            'b_209d58: {
                                'b_209d38: {
                                    c.cycles += 2;
                                    let t = (c.g[S3][0] as i64) < 0;
                                    c.g[V1][0] = 0xc;
                                    if t {
                                        break 'b_209d38;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(19, 0);
                                        c.cvt_s_w(0, 0);
                                        let t = true;
                                        c.sub_s(1, 0, 21);
                                        if t {
                                            break 'b_209d58;
                                        } else {
                                            break 'b_209d38;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[V1][0] = sx((c.g[S3][0] as u32) >> 1);
                                c.g[V0][0] = c.g[S3][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.g[V1][0] = 0xc;
                                c.sub_s(1, 0, 21);
                                break 'b_209d58;
                            }
                            c.cycles += 52;
                            c.mult(0, 2, 23, 3);
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                            c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x57));
                            c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
                            c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                            c.sdr(6, (c.g[A0][0] as u32));
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x12b));
                            c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x124));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x12c));
                            c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa8));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x54));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xac));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00209dcc);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00209dcc)?;
                            c.g[A0][0] = 0x470000;
                            c.g[RA][0] = sx(0x00209dd8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1958));
                            call_ptr(c, 0x00317670, 0x00209dd8)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            c.g[RA][0] = sx(0x00209de4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x00399730, 0x00209de4)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209e00);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00209e00)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                            c.g[V1][0] = 0xffffffffffffe0ff;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            c.g[V0][0] = c.g[V0][0] | 0xc00;
                            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xd4));
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                            c.g[RA][0] = sx(0x00209e28);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb4));
                            call_ptr(c, 0x00397910, 0x00209e28)?;
                            break 'b_209e28;
                        }
                    }
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc0));
                    c.g[S7][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00209a28);
                        }
                        continue 'l_209a28;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                        break 'b_209e40;
                    }
                }
            }
        }
        c.cycles += 13;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x118));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenReadPrivateMessage_Init(OVStateOverlay* self)` (0x210940): create the
/// ReadPrivateMessage screen, show the sender name, set up the agreement scroll bar and
/// the chat text scroll widget, then run `cOVStateOverlay_Init`.
pub fn ov_screen_read_private_message_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_210b3c: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = 0x470000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ce8));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[RA][0] = sx(0x0021096c);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0021096c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x00210980);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x00210980)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_210b3c;
            } else {
                'b_2109f0: {
                    c.cycles += 13;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x00210998);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0039c870, 0x00210998)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002109a4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d00));
                    call_ptr(c, 0x00317670, 0x002109a4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x002109b0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002109b0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_2109f0;
                    } else {
                        c.cycles += 13;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x002109d0);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002109d0)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x7;
                        c.g[RA][0] = sx(0x002109e0);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                        call_ptr(c, 0x0014dd58, 0x002109e0)?;
                        c.g[A1][0] = 0x530000;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002109f0);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4fc8));
                        call_ptr(c, 0x003a0c50, 0x002109f0)?;
                        break 'b_2109f0;
                    }
                }
                'b_210b34: {
                    c.cycles += 44;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002109fc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d10));
                    call_ptr(c, 0x00317670, 0x002109fc)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a08);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a08)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d20));
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.g[RA][0] = sx(0x00210a24);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00210a24)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a30);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a30)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00210a40);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d30));
                    call_ptr(c, 0x00317670, 0x00210a40)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a4c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a4c)?;
                    c.g[A0][0] = 0x470000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00210a5c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d40));
                    call_ptr(c, 0x00317670, 0x00210a5c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a68);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a68)?;
                    c.g[V1][0] = 0x2;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[S0][0]);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x17f8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x88), c.g[V1][0]);
                    c.g[RA][0] = sx(0x00210a88);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                    call_ptr(c, 0x00317670, 0x00210a88)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00210a94);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00210a94)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        break 'b_210b34;
                    } else {
                        c.cycles += 37;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.g[V0][0] | 0x1;
                        c.g[RA][0] = sx(0x00210ab4);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                        call_ptr(c, 0x003a1310, 0x00210ab4)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00210acc);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00210acc)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = c.g[SP][0];
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.g[V0][0] | 0x80;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                        c.swc1(0, (c.g[SP][0] as u32));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00210b08);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00210b08)?;
                        c.g[V1][0] = 0x4d0000;
                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72f8));
                        c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(2, (c.g[A2][0] as u32));
                        c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0xaf));
                        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0xa8));
                        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0xb7));
                        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0xb0));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), c.g[S2][0]);
                        break 'b_210b34;
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x00210b3c);
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x0020a380,
                    f: ee::s05::cOVStateOverlay_Init,
                }
                .call(c, 0x00210b3c)?;
                break 'b_210b3c;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenProfileSavePDA_RefreshCardInfo(OVScreen* self)` (0x211380): refresh the
/// memory-card page of the profile save screen: device name, the six slot names with
/// their enabled/static states, and the two directory info icons.
pub fn ov_screen_profile_save_pda_refresh_card_info(c: &mut Cpu) {
    guest(c, |c| {
        'b_211820: {
            'b_211438: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x720));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
                c.g[RA][0] = sx(0x002113bc);
                Callee {
                    addr: 0x00227f80,
                    f: ee::s05::MCManager_Get,
                }
                .call(c, 0x002113bc)?;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x002113d4);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x002113d4)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    break 'b_211438;
                } else {
                    'b_211414: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002113f4);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x002113f4)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x214));
                            break 'b_211414;
                        } else {
                            c.cycles += 6;
                            c.g[RA][0] = sx(0x00211404);
                            call_ptr(c, 0x001d8240, 0x00211404)?;
                            c.g[RA][0] = sx(0x0021140c);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x00211270,
                                f: ee::s05::OVScreenProfileSavePDA_ResetSlotList,
                            }
                            .call(c, 0x0021140c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
                            if t {
                                break 'b_211820;
                            } else {
                                break 'b_211414;
                            }
                        }
                    }
                    'b_211434: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        if t {
                            break 'b_211434;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00211434);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00211434)?;
                            break 'b_211434;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    break 'b_211438;
                }
            }
            'b_21181c: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00211448);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00211448)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_21181c;
                } else {
                    c.cycles += 60;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                    c.g[A3][0] = 0x320;
                    c.g[RA][0] = sx(0x00211460);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00241710, 0x00211460)?;
                    c.g[S7][0] = 0u64;
                    c.g[A0][0] = 0x470000;
                    c.g[S5][0] = 0x1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d78));
                    c.g[RA][0] = sx(0x00211478);
                    c.g[FP][0] = 0x470000;
                    call_ptr(c, 0x00317670, 0x00211478)?;
                    c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x234));
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0021148c);
                    c.g[S3][0] = 0u64;
                    call_ptr(c, 0x0039d860, 0x0021148c)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00211498);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0d00, 0x00211498)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x21c), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x002114b4);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x002114b4)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x002114c0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                    call_ptr(c, 0x00317670, 0x002114c0)?;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x002114cc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x002114cc)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4c0000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66c8));
                    c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.ldr(3, (c.g[A2][0] as u32));
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x8f));
                    c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x88));
                    c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x97));
                    c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x200));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x204));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211514);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00211514)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00211520);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x00211520)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x00211538);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x00211538)?;
                    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    'l_211540: loop {
                        'b_211708: {
                            'b_2116d4: {
                                'b_2116c4: {
                                    'b_21163c: {
                                        c.cycles += 30;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1d68));
                                        c.g[RA][0] = sx(0x00211550);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x00417828, 0x00211550)?;
                                        c.g[RA][0] = sx(0x00211558);
                                        c.g[A0][0] = c.g[S4][0];
                                        call_ptr(c, 0x00317670, 0x00211558)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x00211564);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x00211564)?;
                                        c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S6][0];
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00211588);
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x00211588)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00211594);
                                        c.g[A1][0] = c.g[S6][0];
                                        call_ptr(c, 0x003a0e90, 0x00211594)?;
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x002115b0);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x002115b0)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                        if t {
                                            break 'b_21163c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x002115d0);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x002115d0)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                                break 'b_21163c;
                                            } else {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x002115f0);
                                                c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x002115f0)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x434)
                                                        );
                                                    break 'b_21163c;
                                                } else {
                                                    'b_211614: {
                                                        c.cycles += 2;
                                                        let t = c.g[S7][0] != 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        if t {
                                                            break 'b_211614;
                                                        } else {
                                                            c.cycles += 5;
                                                            c.g[S7][0] = 0x1;
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            if t {
                                                                break 'b_2116c4;
                                                            } else {
                                                                break 'b_211614;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00211628);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x00211628)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_2116c4;
                                                    } else {
                                                        break 'b_21163c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00211654);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x00211654)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[S3][0];
                                    if t {
                                        break 'b_2116d4;
                                    } else {
                                        'b_2116bc: {
                                            'b_21169c: {
                                                c.cycles += 8;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x00211674);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x00211674)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_21169c;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x00211690);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x00211690)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    let t = true;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_2116bc;
                                                    } else {
                                                        break 'b_21169c;
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x002116b4);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x002116b4)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            break 'b_2116bc;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                        break 'b_2116c4;
                                    }
                                }
                                c.cycles += 4;
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x002116cc);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x002116cc)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_211708;
                                } else {
                                    break 'b_2116d4;
                                }
                            }
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x002116ec);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x002116ec)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00211704);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x00211704)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            break 'b_211708;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S5][0] as i64) < 7i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00211540);
                            }
                            continue 'l_211540;
                        } else {
                            'b_21179c: {
                                'b_21176c: {
                                    c.cycles += 8;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00211728);
                                    c.g[A3][0] = 0x1;
                                    call_ptr(c, 0x001d97e0, 0x00211728)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V1][0] = 0x470000;
                                    if t {
                                        break 'b_21176c;
                                    } else {
                                        c.cycles += 14;
                                        c.g[V0][0] = 0x470000;
                                        c.g[RA][0] = sx(0x00211740);
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d90));
                                        call_ptr(c, 0x00317670, 0x00211740)?;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x0021174c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0021174c)?;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[A2][0] as u32;
                                        c.g[RA][0] = sx(0x00211764);
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x00211764)?;
                                        let t = true;
                                        c.g[A1][0] = c.g[S1][0];
                                        if t {
                                            break 'b_21179c;
                                        } else {
                                            break 'b_21176c;
                                        }
                                    }
                                }
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x00211774);
                                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d90));
                                call_ptr(c, 0x00317670, 0x00211774)?;
                                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x00211780);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00211780)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[RA][0] = sx(0x00211798);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x00211798)?;
                                c.g[A1][0] = c.g[S1][0];
                                break 'b_21179c;
                            }
                            'b_2117f0: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                                c.g[RA][0] = sx(0x002117ac);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x001d97e0, 0x002117ac)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[A2][0] = 0x470000;
                                if t {
                                    break 'b_2117f0;
                                } else {
                                    c.cycles += 14;
                                    c.g[A1][0] = 0x470000;
                                    c.g[RA][0] = sx(0x002117c4);
                                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1da0));
                                    call_ptr(c, 0x00317670, 0x002117c4)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x002117d0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x002117d0)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[RA][0] = sx(0x002117e8);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x002117e8)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
                                    if t {
                                        break 'b_211820;
                                    } else {
                                        break 'b_2117f0;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[RA][0] = sx(0x002117f8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1da0));
                            call_ptr(c, 0x00317670, 0x002117f8)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00211804);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00211804)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x0021181c);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0021181c)?;
                            break 'b_21181c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
            break 'b_211820;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x720));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP, RA as TEST_RA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in the machine's own RAM).
    const OC988_CTRL: u32 = DATA + 0x2f_0000;
    const OC988_MAGIC: u32 = 0x0c98_8a5e;
    /// A real leaf that returns 0 (`jr ra; li v0, 0`).
    const OC988_ZERO_LEAF: u32 = 0x0014_06c0;
    /// A real leaf that returns `a0` (`jr ra; move v0, a0`).
    const OC988_THIS_LEAF: u32 = 0x0014_0ce0;
    /// Kinds of the gated fakes: log only, `sprintf` (writes `s<a2>`), or a
    /// result sequence (kind `OC988_K_SEQ + n`: `v0 = word[ctr & 3]` of
    /// mailbox `n` at `CTRL + 0x30 + 0x10 * n`).
    const OC988_K_PLAIN: u32 = 0;
    const OC988_K_SPRINTF: u32 = 1;
    const OC988_K_SEQ: u32 = 2;

    /// The logging fake of a gated callee: logs the call like every fake
    /// machine call, then (by kind) writes the name or sets `v0`.
    fn oc988_fake(c: &mut Cpu, addr: u32, kind: u32) {
        let a0 = c.g[A0][0] as u32;
        let a2 = c.g[A2][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match kind {
            OC988_K_PLAIN => {}
            OC988_K_SPRINTF => {
                let text = format!("s{a2}");
                for (i, b) in text.bytes().enumerate() {
                    c.bus.write8(a0 + i as u32, b);
                }
                c.bus.write8(a0 + text.len() as u32, 0);
            }
            k => {
                let n = k - OC988_K_SEQ;
                let ctr = OC988_CTRL + 0x20 + 4 * n;
                let i = c.bus.read32(ctr);
                c.bus.write32(ctr, i + 1);
                let v = c.bus.read32(OC988_CTRL + 0x30 + 0x10 * n + 4 * (i & 3));
                c.g[V0][0] = sx(v);
            }
        }
    }

    macro_rules! oc988_fakes {
        ($($name:ident $prev:ident $addr:literal $shard:ident $slot:ident $gen:ident $kind:expr;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC988_CTRL) != OC988_MAGIC {
                        return fakes::fall_back(c, &$prev, ee::$shard::$gen);
                    }
                    oc988_fake(c, $addr, $kind)
                }
            )*
            fn oc988_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $name, &$prev); )*
            }
        };
    }

    oc988_fakes! {
        oc988_f_create_screen OC988_P_CREATE_SCREEN 0x0039_7cc0 s13 OVERRIDE_00397CC0 f_00397cc0__generated OC988_K_SEQ;
        oc988_f_start_thread OC988_P_START_THREAD 0x0039_c870 s13 OVERRIDE_0039C870 f_0039c870__generated OC988_K_PLAIN;
        oc988_f_overlay_init OC988_P_OVERLAY_INIT 0x0020_a380 s05 OVERRIDE_0020A380 f_0020a380__generated OC988_K_PLAIN;
        oc988_f_step_slot_dir OC988_P_STEP_SLOT_DIR 0x001d_97e0 s04 OVERRIDE_001D97E0 f_001d97e0__generated OC988_K_SEQ + 1;
        oc988_f_card_error_text OC988_P_CARD_ERROR_TEXT 0x0024_1710 s06 OVERRIDE_00241710 f_00241710__generated OC988_K_PLAIN;
        oc988_f_no_cards OC988_P_NO_CARDS 0x001d_8240 s04 OVERRIDE_001D8240 f_001d8240__generated OC988_K_PLAIN;
        oc988_f_reset_slot_list OC988_P_RESET_SLOT_LIST 0x0021_1270 s05 OVERRIDE_00211270 f_00211270__generated OC988_K_PLAIN;
        oc988_f_set_wide OC988_P_SET_WIDE 0x003a_0d00 s13 OVERRIDE_003A0D00 f_003a0d00__generated OC988_K_PLAIN;
        oc988_f_set_wide_string OC988_P_SET_WIDE_STRING 0x003a_0e90 s13 OVERRIDE_003A0E90 f_003a0e90__generated OC988_K_PLAIN;
        oc988_f_sprintf OC988_P_SPRINTF 0x0041_7828 s15 OVERRIDE_00417828 f_00417828__generated OC988_K_SPRINTF;
        oc988_f_sprintf_wide OC988_P_SPRINTF_WIDE 0x002c_27c0 s08 OVERRIDE_002C27C0 f_002c27c0__generated OC988_K_PLAIN;
        oc988_f_set_string OC988_P_SET_STRING 0x003a_0c50 s13 OVERRIDE_003A0C50 f_003a0c50__generated OC988_K_PLAIN;
        oc988_f_set_text_id OC988_P_SET_TEXT_ID 0x003a_1000 s13 OVERRIDE_003A1000 f_003a1000__generated OC988_K_PLAIN;
        oc988_f_nav_buttons OC988_P_NAV_BUTTONS 0x003a_1310 s13 OVERRIDE_003A1310 f_003a1310__generated OC988_K_PLAIN;
    }

    fn oc988_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The gated fakes on, with their result mailboxes (word `i` of mailbox
    /// `n`) set by the caller afterwards.
    fn oc988_gate(c: &mut Cpu) {
        oc988_install_fakes();
        c.bus.write32(OC988_CTRL, OC988_MAGIC);
        for i in 0..0x20 {
            c.bus.write32(OC988_CTRL + 4 + 4 * i, 0);
        }
    }

    fn oc988_mailbox(c: &mut Cpu, n: u32, words: [u32; 4]) {
        for (i, w) in words.into_iter().enumerate() {
            c.bus
                .write32(OC988_CTRL + 0x30 + 0x10 * n + 4 * i as u32, w);
        }
    }

    fn oc988_put_str(c: &mut Cpu, addr: u32, text: &str) {
        for (i, b) in text.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + text.len() as u32, 0);
    }

    /// `StrHash(text)` run for real on the fixture machine.
    fn oc988_hash(c: &mut Cpu, addr: u32) -> u32 {
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = TEST_RA as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, TEST_RA, "StrHash did not return");
        c.g[V0][0] as u32
    }

    /// A vtable of 0x48 slots (`{delta 0, method}` at `8 * n`); a slot's
    /// method is a logging fake, or (1 in `zero_chance`) a real leaf that
    /// returns 0.
    fn oc988_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, zero_chance: u32) {
        for n in 0..0x48u32 {
            c.bus.write16(vt + 8 * n, 0);
            let f = if zero_chance != 0 && r.next_u32() % zero_chance == 0 {
                OC988_ZERO_LEAF
            } else {
                FAKE + 0x10 * n
            };
            c.bus.write32(vt + 8 * n + 4, f);
        }
    }

    /// A UI screen at `screen` (needs `0x3000` bytes): the children named by
    /// `names` (`(address of the name string, its text, present)`) with the
    /// hash of their string in `+0x38` and a vtable at `+8`. Lookups by
    /// `StrHash(name)` find exactly the present ones.
    fn oc988_screen(c: &mut Cpu, r: &mut Rng, screen: u32, names: &[(u32, String, bool)]) {
        let vt = screen + 0x1000;
        oc988_vtable(c, r, vt, 3);
        let count = screen + 0x10;
        let array = screen + 0x20;
        let mut n = 0;
        for (i, (addr, text, present)) in names.iter().enumerate() {
            oc988_put_str(c, *addr, text);
            let h = oc988_hash(c, *addr);
            if *present {
                let obj = screen + 0x2000 + 0x100 * i as u32;
                c.bus.write32(obj + 8, vt);
                c.bus.write32(obj + 0x38, h);
                c.bus.write32(obj + 0x94, r.next_u32());
                c.bus.write32(array + 4 * n, obj);
                n += 1;
            }
        }
        c.bus.write32(count, n);
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(screen + 0x3c, array);
        // The list head the indicators are appended to (`screen + 0xb4`).
        c.bus.write32(screen + 0xb4 + 0xc, screen + 0xe0);
    }

    /// Names of `addrs`, present with probability 3 in 4.
    fn oc988_names(r: &mut Rng, addrs: &[u32], tag: &str) -> Vec<(u32, String, bool)> {
        addrs
            .iter()
            .enumerate()
            .map(|(i, &a)| (a, format!("{tag}{i}"), r.next_u32() % 4 != 0))
            .collect()
    }

    /// The first track-info record (`0x440770`: key 0 matches the key the
    /// zeroed track table gives) with random bounds floats; `+0x18` is the
    /// session-type count `BEMissionInt_GetSessionType` returns.
    fn oc988_bounds(c: &mut Cpu, r: &mut Rng, count: u32) {
        for off in (4..0x2c).step_by(4) {
            c.bus.write32(0x0044_0770 + off, r.f32(300.0));
        }
        c.bus.write32(0x0044_0770, 0);
        c.bus.write32(0x0044_0770 + 0x18, count);
    }

    /// `world -> +0x84 -> +0xc -> +0x28` = rider whose object at `+0x6c0` has
    /// the vtable method `vt+0x2c` (the real "return this" leaf, with a `+0x10`
    /// delta, so the position vector is read from the rider at `+0x6d0`).
    fn oc988_rider(c: &mut Cpu, r: &mut Rng) {
        let world = DATA + 0x2000;
        let race = DATA + 0x2100;
        let mgr = DATA + 0x2200;
        let rider = DATA + 0x2400;
        let vt = DATA + 0x2300;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, race);
        c.bus.write32(race + 0xc, mgr);
        c.bus.write32(mgr + 0x28, rider);
        c.bus.write32(rider + 0x6c0, vt);
        c.bus.write16(vt + 0x28, 0x10);
        c.bus.write32(vt + 0x2c, OC988_THIS_LEAF);
        for i in 0..4 {
            c.bus.write32(rider + 0x6d0 + 4 * i, r.f32(300.0));
        }
    }

    /// `OVScreenSession_UpdateConfirmPopup`: with the flag at `+0xbc` set the
    /// popup widgets are shown and filled from the string manager (reached
    /// through the world; its lookup method returns 0 or a pointer), without
    /// it they are hidden. Widgets are present or missing.
    #[test]
    fn ov_screen_session_update_confirm_popup_matches() {
        check(
            ee::s05::f_00208f88__generated,
            ov_screen_session_update_confirm_popup,
            90,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x208f88, 36);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let screen = DATA + 0x8000;
                let names = oc988_names(
                    &mut r,
                    &[
                        0x0047_0a20,
                        0x0047_1850,
                        0x0047_1890,
                        0x0047_18a0,
                        0x0047_18b8,
                        0x0047_18c8,
                        0x0047_18d8,
                        0x0047_06b8,
                        0x0047_06c8,
                        0x0047_0aa8,
                    ],
                    "a",
                );
                oc988_screen(c, &mut r, screen, &names);
                oc988_w(c, this + 0xd4, if seed % 11 == 0 { 0 } else { screen });
                oc988_w(c, this + 0xbc, if seed % 3 == 0 { 0 } else { 1 });
                // world -> +0x8c: the string manager and its lookup method.
                let world = DATA + 0x2000;
                let mgr = DATA + 0x2100;
                let vt = DATA + 0x2200;
                oc988_w(c, GAME_GP - 0x848, world);
                oc988_w(c, world + 0x8c, mgr);
                oc988_w(c, mgr + 4, vt);
                c.bus.write16(vt + 0x20, 0);
                oc988_w(
                    c,
                    vt + 0x24,
                    if r.next_u32() % 3 == 0 {
                        OC988_ZERO_LEAF
                    } else {
                        FAKE + 0x500
                    },
                );
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `OVScreenSession_CreatePlayerIndicator`: the rider's map position
    /// (random bounds, negative and positive) becomes the icon's place; the
    /// interface results `S1`/`S2` are sometimes negative (the unsigned
    /// float conversions).
    #[test]
    fn ov_screen_session_create_player_indicator_matches() {
        check(
            ee::s05::f_002096a8__generated,
            ov_screen_session_create_player_indicator,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x2096a8, 12);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let img = DATA + 0x6800;
                let screen = DATA + 0x8000;
                oc988_rider(c, &mut r);
                oc988_bounds(c, &mut r, 0);
                for i in 0..3 {
                    c.bus.write32(img + 0x44 + 4 * i, r.f32(400.0));
                }
                oc988_w(c, this + 0xd0, img);
                oc988_w(c, this + 0xd4, screen);
                oc988_w(c, screen + 0xb4 + 0xc, screen + 0xe0);
                // The two interface lookups are the first two fake calls.
                let neg = |bit: u32| {
                    if (seed >> bit) & 1 == 1 {
                        0xffff_ff00u32
                    } else {
                        0
                    }
                };
                c.bus.write32(LOG_RESULTS, neg(0));
                c.bus.write32(LOG_RESULTS + 4, neg(1));
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `OVScreenSession_CreateMapIndicators`: one icon per path marker of
    /// type 2 that exists, for `count` session entries (0..=4), markers
    /// present or absent.
    #[test]
    fn ov_screen_session_create_map_indicators_matches() {
        check(
            ee::s05::f_00209970__generated,
            ov_screen_session_create_map_indicators,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x209970, 12);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let img = DATA + 0x6800;
                let screen = DATA + 0x8000;
                let count = if seed % 5 == 0 { 0 } else { 1 + seed % 4 };
                oc988_bounds(c, &mut r, count);
                for i in 0..3 {
                    c.bus.write32(img + 0x44 + 4 * i, r.f32(400.0));
                }
                oc988_w(c, this + 0xd0, img);
                oc988_w(c, this + 0xd4, screen);
                oc988_w(c, screen + 0xb4 + 0xc, screen + 0xe0);
                // Path data at 0x4d33a0: `{count, entries}`, 0x28 bytes per
                // entry `{id, type, position..}`.
                let path = 0x004d_33a0;
                let entries = DATA + 0x7000;
                let n = 1 + seed % 3;
                oc988_w(c, path, n);
                oc988_w(c, path + 4, entries);
                for i in 0..n {
                    let e = entries + 0x28 * i;
                    oc988_w(c, e, 1 + i);
                    oc988_w(c, e + 4, if r.next_u32() % 4 == 0 { 3 } else { 2 });
                    for k in 0..3 {
                        oc988_w(c, e + 8 + 4 * k, r.f32(300.0));
                    }
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `OVScreenReadPrivateMessage_Init`: the screen is created (or not: the
    /// create-screen fake answers 0), widgets are present or missing.
    #[test]
    fn ov_screen_read_private_message_init_matches() {
        check(
            ee::s05::f_00210940__generated,
            ov_screen_read_private_message_init,
            60,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x210940, 14);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let screen = DATA + 0x8000;
                let names = oc988_names(
                    &mut r,
                    &[
                        0x0047_1d00,
                        0x0047_1d10,
                        0x0047_1d20,
                        0x0047_1d30,
                        0x0047_1d40,
                        0x0046_e808,
                    ],
                    "d",
                );
                oc988_screen(c, &mut r, screen, &names);
                oc988_mailbox(c, 0, [if seed % 5 == 0 { 0 } else { screen }, 0, 0, 0]);
                oc988_w(c, this + 0x10, DATA + 0x6400);
                // world -> +0x78 (an interface owner).
                let world = DATA + 0x2000;
                oc988_w(c, GAME_GP - 0x848, world);
                oc988_w(c, world + 0x78, DATA + 0x2100);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `OVScreenProfileSavePDA_RefreshCardInfo`: the card manager (through
    /// `gp-0x47c`) and the screen's methods answer 0 or non-zero per slot, the
    /// six slot widgets and the directory icons are present or missing, and
    /// `StepSlotDir` answers -1 or 0.
    #[test]
    fn ov_screen_profile_save_pda_refresh_card_info_matches() {
        check(
            ee::s05::f_00211380__generated,
            ov_screen_profile_save_pda_refresh_card_info,
            90,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x211380, 40);
                oc988_gate(c);
                let this = DATA + 0x6000;
                let screen = DATA + 0x8000;
                let mgr = DATA + 0x5000;
                let card = DATA + 0x5600;
                let mut names = oc988_names(&mut r, &[0x0047_1d78, 0x0047_00f0], "e");
                for i in 1..7u32 {
                    names.push((
                        DATA + 0x5f00 + 8 * i,
                        format!("s{i}"),
                        r.next_u32() % 5 != 0,
                    ));
                }
                names.extend(oc988_names(&mut r, &[0x0047_1d90, 0x0047_1da0], "f"));
                oc988_screen(c, &mut r, screen, &names);
                // The screen's own vtable (`this + 8`) and the card
                // manager's (`card + 0`): per-slot results 0 or non-zero.
                oc988_vtable(c, &mut r, DATA + 0x5800, 2);
                oc988_vtable(c, &mut r, DATA + 0x5b00, 2);
                oc988_w(c, this + 8, DATA + 0x5800);
                oc988_w(c, this + 0x40, screen);
                oc988_w(c, this + 0x214, if seed % 3 == 0 { 3 } else { 1 });
                oc988_w(c, GAME_GP - 0x47c, mgr);
                oc988_w(c, mgr + 0x428, r.next_u32() % 4);
                oc988_w(c, mgr + 0x434, card);
                oc988_w(c, card, DATA + 0x5b00);
                let neg = 0xffff_ffffu32;
                oc988_mailbox(
                    c,
                    1,
                    [
                        if seed % 2 == 0 { neg } else { 0 },
                        if seed % 4 < 2 { neg } else { 0 },
                        0,
                        0,
                    ],
                );
                c.g[A0][0] = this as u64;
            },
        );
    }
}
