//! Staging module oc980 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc980`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_bab8,
    game_ui_push_popup_screen,
    "GameUI_PushPopupScreen",
)];

/// `GameUI_PushPopupScreen` (0x0020bab8): builds the overlay/popup state object
/// for popup type `a0` (1..0x28: allocate, run its constructor, set vtable and
/// fields) and pushes it on the UI state stack. The generated body, with its
/// calls through `call_ptr` and unwinds through `?`.
pub fn game_ui_push_popup_screen(c: &mut Cpu) {
    guest(c, |c| {
        'b_20bb60: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A0][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x0020baf4);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039fa88, 0x0020baf4)?;
            c.g[S5][0] = c.g[V0][0];
            c.g[V0][0] = 0x1;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_20bb60;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bb1c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bb1c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bb30);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bb30)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3098));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x2;
                break 'b_20bb60;
            }
        }
        'b_20bbc4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                break 'b_20bbc4;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bb80);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bb80)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bb94);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bb94)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2fc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x3;
                break 'b_20bbc4;
            }
        }
        'b_20bc28: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x4;
            if t {
                break 'b_20bc28;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bbe4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bbe4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bbf8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bbf8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ee8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x4;
                break 'b_20bc28;
            }
        }
        'b_20bc8c: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x5;
            if t {
                break 'b_20bc8c;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bc48);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bc48)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bc5c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bc5c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e10));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x5;
                break 'b_20bc8c;
            }
        }
        'b_20bcf0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x6;
            if t {
                break 'b_20bcf0;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bcac);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bcac)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bcc0);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bcc0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d38));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x6;
                break 'b_20bcf0;
            }
        }
        'b_20bd54: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x7;
            if t {
                break 'b_20bd54;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x114;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bd10);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bd10)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bd24);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bd24)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2c60));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), 0u64);
                c.g[V0][0] = 0x7;
                break 'b_20bd54;
            }
        }
        'b_20bdb4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x8;
            if t {
                break 'b_20bdb4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bd74);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bd74)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bd88);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bd88)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2b88));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x8;
                break 'b_20bdb4;
            }
        }
        'b_20be14: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x9;
            if t {
                break 'b_20be14;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bdd4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bdd4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bde8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bde8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2850));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x9;
                break 'b_20be14;
            }
        }
        'b_20be74: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xa;
            if t {
                break 'b_20be74;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020be34);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020be34)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020be48);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020be48)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2778));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xa;
                break 'b_20be74;
            }
        }
        'b_20bed4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xb;
            if t {
                break 'b_20bed4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020be94);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020be94)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bea8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bea8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4088));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xb;
                break 'b_20bed4;
            }
        }
        'b_20bf34: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xc;
            if t {
                break 'b_20bf34;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020bef4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020bef4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bf08);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bf08)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3fb0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xc;
                break 'b_20bf34;
            }
        }
        'b_20bfac: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xd;
            if t {
                break 'b_20bfac;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020bf54);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020bf54)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bf68);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bf68)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e00));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020bfa4);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020bfa4)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[V0][0] = 0xd;
                break 'b_20bfac;
            }
        }
        'b_20c024: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xe;
            if t {
                break 'b_20c024;
            } else {
                c.cycles += 28;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[RA][0] = sx(0x0020bfcc);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020bfcc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020bfe0);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020bfe0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3d28));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1cc0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c01c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x00317670, 0x0020c01c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[V0][0] = 0xe;
                break 'b_20c024;
            }
        }
        'b_20c084: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0xf;
            if t {
                break 'b_20c084;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c044);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c044)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c058);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c058)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b80));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0xf;
                break 'b_20c084;
            }
        }
        'b_20c0e4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x10;
            if t {
                break 'b_20c0e4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c0a4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c0a4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c0b8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c0b8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x10;
                break 'b_20c0e4;
            }
        }
        'b_20c144: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x11;
            if t {
                break 'b_20c144;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c104);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c104)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c118);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c118)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2108));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x11;
                break 'b_20c144;
            }
        }
        'b_20c1a4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x12;
            if t {
                break 'b_20c1a4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xb8;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c164);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c164)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c178);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c178)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4aa8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x12;
                break 'b_20c1a4;
            }
        }
        'b_20c1dc: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x13;
            if t {
                break 'b_20c1dc;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c1c4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c1c4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c1d4);
                c.g[A2][0] = 0x12;
                Callee {
                    addr: 0x001fdbf0,
                    f: ee::s05::OVScreenSaveReplay_ctor,
                }
                .call(c, 0x0020c1d4)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x13;
                break 'b_20c1dc;
            }
        }
        'b_20c214: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x14;
            if t {
                break 'b_20c214;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x274;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c1fc);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c1fc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c20c);
                c.g[A2][0] = 0x13;
                Callee {
                    addr: 0x001fe968,
                    f: ee::s05::OVScreenSaveRecords_ctor,
                }
                .call(c, 0x0020c20c)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x14;
                break 'b_20c214;
            }
        }
        'b_20c274: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x15;
            if t {
                break 'b_20c274;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c234);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c234)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c248);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c248)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x25c8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x15;
                break 'b_20c274;
            }
        }
        'b_20c2d4: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x16;
            if t {
                break 'b_20c2d4;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c294);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c294)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c2a8);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c2a8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x26a0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x16;
                break 'b_20c2d4;
            }
        }
        'b_20c334: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x17;
            if t {
                break 'b_20c334;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c2f4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c2f4)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c308);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c308)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3ed8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x17;
                break 'b_20c334;
            }
        }
        'b_20c398: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x18;
            if t {
                break 'b_20c398;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c354);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c354)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c368);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c368)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x18;
                break 'b_20c398;
            }
        }
        'b_20c400: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x19;
            if t {
                break 'b_20c400;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c3b8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c3b8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c3cc);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c3cc)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48f8));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x19;
                break 'b_20c400;
            }
        }
        'b_20c468: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1a;
            if t {
                break 'b_20c468;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c420);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c420)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c434);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c434)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4820));
                c.g[V1][0] = 0x2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1a;
                break 'b_20c468;
            }
        }
        'b_20c4d0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1b;
            if t {
                break 'b_20c4d0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c488);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c488)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c49c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c49c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4748));
                c.g[V1][0] = 0x3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1b;
                break 'b_20c4d0;
            }
        }
        'b_20c538: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1c;
            if t {
                break 'b_20c538;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c4f0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c4f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c504);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c504)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4670));
                c.g[V1][0] = 0x4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1c;
                break 'b_20c538;
            }
        }
        'b_20c5a0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1d;
            if t {
                break 'b_20c5a0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c558);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c558)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c56c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c56c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4598));
                c.g[V1][0] = 0x5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x1d;
                break 'b_20c5a0;
            }
        }
        'b_20c608: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1e;
            if t {
                break 'b_20c608;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0020c5c0);
                c.g[A0][0] = 0xa0;
                call_ptr(c, 0x00317d70, 0x0020c5c0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c5d4);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c5d4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c604);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7548, 0x0020c604)?;
                c.g[V0][0] = 0x1e;
                break 'b_20c608;
            }
        }
        'b_20c670: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x1f;
            if t {
                break 'b_20c670;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0020c628);
                c.g[A0][0] = 0xa4;
                call_ptr(c, 0x00317d70, 0x0020c628)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c63c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c63c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.g[S0][0];
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c66c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                call_ptr(c, 0x001f7738, 0x0020c66c)?;
                c.g[V0][0] = 0x1f;
                break 'b_20c670;
            }
        }
        'b_20c6d0: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x20;
            if t {
                break 'b_20c6d0;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c690);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c690)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c6a4);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c6a4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3760));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x20;
                break 'b_20c6d0;
            }
        }
        'b_20c734: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            if t {
                break 'b_20c734;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x1d4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c6f0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c6f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c700);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c700)?;
                c.g[V0][0] = 0x4a0000;
                c.g[V1][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x2088));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3320));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.g[A1][0] = 0xffffffffffffffff;
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.g[V0][0] = 0x7;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[S3][0] = c.g[S0][0];
                break 'b_20c734;
            }
        }
        'b_20c7a0: {
            c.cycles += 3;
            c.g[V0][0] = 0x21;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x22;
            if t {
                break 'b_20c7a0;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x0020c758);
                c.g[A0][0] = 0x390;
                call_ptr(c, 0x00317d70, 0x0020c758)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c76c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c76c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x33f8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[RA][0] = sx(0x0020c79c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                Callee {
                    addr: 0x002006b8,
                    f: ee::s05::EventSelect_ctor,
                }
                .call(c, 0x0020c79c)?;
                c.g[V0][0] = 0x22;
                break 'b_20c7a0;
            }
        }
        'b_20c7d8: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x23;
            if t {
                break 'b_20c7d8;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x180;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c7c0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c7c0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c7d0);
                c.g[A2][0] = 0x22;
                call_ptr(c, 0x001e3c00, 0x0020c7d0)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x23;
                break 'b_20c7d8;
            }
        }
        'b_20c838: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x24;
            if t {
                break 'b_20c838;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x9c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c7f8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c7f8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c80c);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c80c)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c50));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x24;
                break 'b_20c838;
            }
        }
        'b_20c874: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x25;
            if t {
                break 'b_20c874;
            } else {
                c.cycles += 13;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x27c;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c858);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c858)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0x24;
                c.g[RA][0] = sx(0x0020c86c);
                c.g[A3][0] = 0x2a;
                Callee {
                    addr: 0x00211088,
                    f: ee::s05::OVScreenProfileSavePDA_ctor,
                }
                .call(c, 0x0020c86c)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x25;
                break 'b_20c874;
            }
        }
        'b_20c8ac: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x26;
            if t {
                break 'b_20c8ac;
            } else {
                c.cycles += 12;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0x234;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c894);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c894)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x0020c8a4);
                c.g[A2][0] = 0x25;
                Callee {
                    addr: 0x00212138,
                    f: ee::s05::OVScreenAutosave_ctor,
                }
                .call(c, 0x0020c8a4)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = 0x26;
                break 'b_20c8ac;
            }
        }
        'b_20c914: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x27;
            if t {
                break 'b_20c914;
            } else {
                c.cycles += 24;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c8cc);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c8cc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c8e0);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c8e0)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44c0));
                c.g[V1][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.g[V0][0] = 0x27;
                break 'b_20c914;
            }
        }
        'b_20c980: {
            c.cycles += 2;
            let t = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0x28;
            if t {
                break 'b_20c980;
            } else {
                c.cycles += 25;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xa4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c934);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c934)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c948);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c948)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ab0));
                c.g[V1][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x9c), 0u64);
                c.g[V0][0] = 0x28;
                break 'b_20c980;
            }
        }
        'b_20c9e0: {
            c.cycles += 2;
            if c.g[S1][0] != c.g[V0][0] {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[S5][0]);
                break 'b_20c9e0;
            } else {
                c.cycles += 22;
                c.g[A1][0] = 0x470000;
                c.g[A0][0] = 0xe4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a30));
                c.g[A2][0] = 0u64;
                c.g[RA][0] = sx(0x0020c9a0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x0020c9a0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020c9b4);
                c.g[S3][0] = c.g[S0][0];
                call_ptr(c, 0x0039e2a0, 0x0020c9b4)?;
                c.g[V1][0] = 0x4a0000;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x2088));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3248));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
                c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[S5][0]);
                break 'b_20c9e0;
            }
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x0020c9ec);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0039f290, 0x0020c9ec)?;
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
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine_rt as machine};
    use crate::difftest::DATA;

    /// Every popup type 0..=0x2b (cases 1..0x28 plus the fall-through), the
    /// constructors and allocator faked, some runs unwinding mid-call.
    #[test]
    fn game_ui_push_popup_screen_matches() {
        covered(
            ee::s05::f_0020bab8__generated,
            game_ui_push_popup_screen,
            440,
            &|c, seed| {
                machine(c, seed, 0x0020_bab8, 4);
                c.pc = 0x0020_bab8;
                // world (gp-0x848) -> +0x84 -> +0x48 = UI engine object
                let world = DATA + 0x1000;
                let a = DATA + 0x2000;
                let ui = DATA + 0x3000;
                let gp = c.g[GP][0] as u32;
                c.bus.write32(gp.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, a);
                c.bus.write32(a + 0x48, ui);
                c.g[A0][0] = (seed % 0x2c) as u64;
            },
        );
    }
}
