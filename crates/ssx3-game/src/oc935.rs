//! Staging module oc935 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc935`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_f820,
        c_fe_state_reward_gallery_on_widget_created,
        "cFEStateRewardGallery_OnWidgetCreated",
    ),
    (
        0x001d_0648,
        c_fe_state_reward_gallery_update_help_text,
        "cFEStateRewardGallery_UpdateHelpText",
    ),
    (
        0x001d_0890,
        c_fe_state_reward_gallery_refresh_row_icons,
        "cFEStateRewardGallery_RefreshRowIcons",
    ),
    (
        0x001d_1c98,
        c_fe_state_preview_reward_on_widget_created,
        "cFEStatePreviewReward_OnWidgetCreated",
    ),
    (
        0x001d_2678,
        c_fe_state_mountain_room_ctor,
        "cFEStateMountainRoom_ctor",
    ),
];

/// `void cFEStateRewardGallery::OnWidgetCreated(self, UIWidget* w)` (0x001cf820):
/// a widget of the reward gallery screen was created. Compare its name hash
/// (`w+0x38`) with the screen's widget names in order (help text, arrows, select
/// bar, max/num rewards, slash, Menu0000, poster icon, screen title, page text,
/// reward name/cost, you-have-amount, then the cell and frame grids) and store
/// it in the matching state field; titles, icons and texts depend on the
/// reward type (`self+0x4c`, jump tables in the game image).
///
/// Converted from the generated body (a `match pc` body; every register, cycle
/// and call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_reward_gallery_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001cf820 {
        return ee::s04::f_001cf820__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001cf820;
        loop {
            match pc {
                0x001cf820 => {
                    c.cycles += 19;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[A0][0] = 0x460000;
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001cf864);
                    c.g[S7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001cf864)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf874;
                        continue;
                    }
                    pc = 0x001cf86c;
                    continue;
                }
                0x001cf86c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x900), c.g[S1][0]);
                    if t {
                        pc = 0x001cf8b8;
                        continue;
                    }
                    pc = 0x001cf874;
                    continue;
                }
                0x001cf874 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf87c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6da0));
                    call_ptr(c, 0x00317670, 0x001cf87c)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf88c;
                        continue;
                    }
                    pc = 0x001cf884;
                    continue;
                }
                0x001cf884 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x91c), c.g[S1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf88c;
                    continue;
                }
                0x001cf88c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf894);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6db0));
                    call_ptr(c, 0x00317670, 0x001cf894)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf8a4;
                        continue;
                    }
                    pc = 0x001cf89c;
                    continue;
                }
                0x001cf89c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x920), c.g[S1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf8a4;
                    continue;
                }
                0x001cf8a4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf8ac);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6dc0));
                    call_ptr(c, 0x00317670, 0x001cf8ac)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf8d8;
                        continue;
                    }
                    pc = 0x001cf8b4;
                    continue;
                }
                0x001cf8b4 => {
                    c.cycles += 1;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8f8), c.g[S1][0]);
                    pc = 0x001cf8b8;
                    continue;
                }
                0x001cf8b8 => {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001cf8d0);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001cf8d0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cf8d8;
                    continue;
                }
                0x001cf8d8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf8e0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6dd8));
                    call_ptr(c, 0x00317670, 0x001cf8e0)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cf908;
                        continue;
                    }
                    pc = 0x001cf8e8;
                    continue;
                }
                0x001cf8e8 => {
                    c.cycles += 8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x918), c.g[S1][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                    c.g[RA][0] = sx(0x001cf900);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001cf900)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x918));
                    if t {
                        pc = 0x001cf934;
                        continue;
                    }
                    pc = 0x001cf908;
                    continue;
                }
                0x001cf908 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf910);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6de8));
                    call_ptr(c, 0x00317670, 0x001cf910)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        pc = 0x001cf944;
                        continue;
                    }
                    pc = 0x001cf918;
                    continue;
                }
                0x001cf918 => {
                    c.cycles += 7;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x914), c.g[S1][0]);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                    c.g[RA][0] = sx(0x001cf930);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001cf930)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x914));
                    pc = 0x001cf934;
                    continue;
                }
                0x001cf934 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf93c);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0c50, 0x001cf93c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cf944;
                    continue;
                }
                0x001cf944 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf94c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1ee8));
                    call_ptr(c, 0x00317670, 0x001cf94c)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cf96c;
                        continue;
                    }
                    pc = 0x001cf954;
                    continue;
                }
                0x001cf954 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cf964);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1ef0));
                    call_ptr(c, 0x003a0c50, 0x001cf964)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cf96c;
                    continue;
                }
                0x001cf96c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf974);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
                    call_ptr(c, 0x00317670, 0x001cf974)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001cf984;
                        continue;
                    }
                    pc = 0x001cf97c;
                    continue;
                }
                0x001cf97c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8f4), c.g[S1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf984;
                    continue;
                }
                0x001cf984 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cf98c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6df8));
                    call_ptr(c, 0x00317670, 0x001cf98c)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfa30;
                        continue;
                    }
                    pc = 0x001cf994;
                    continue;
                }
                0x001cf994 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cf9a8;
                    continue;
                }
                0x001cf9a8 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f40));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001cf9c0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e08));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cf9d0;
                    continue;
                }
                0x001cf9d0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e18));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cf9e0;
                    continue;
                }
                0x001cf9e0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e30));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cf9f0;
                    continue;
                }
                0x001cf9f0 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e40));
                    if t {
                        pc = 0x001cfa0c;
                        continue;
                    }
                    pc = 0x001cfa00;
                    continue;
                }
                0x001cfa00 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e50));
                    pc = 0x001cfa0c;
                    continue;
                }
                0x001cfa0c => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001cfa14);
                    call_ptr(c, 0x00317670, 0x001cfa14)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001cfa20);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x001cfa20)?;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cfa30;
                    continue;
                }
                0x001cfa30 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfa38);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                    call_ptr(c, 0x00317670, 0x001cfa38)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfab4;
                        continue;
                    }
                    pc = 0x001cfa40;
                    continue;
                }
                0x001cfa40 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cfa54;
                    continue;
                }
                0x001cfa54 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6f60));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001cfa6c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e70));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa78;
                    continue;
                }
                0x001cfa78 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e88));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa84;
                    continue;
                }
                0x001cfa84 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ea0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa90;
                    continue;
                }
                0x001cfa90 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6eb0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfa9c;
                    continue;
                }
                0x001cfa9c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ec0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfaa8;
                    continue;
                }
                0x001cfaa8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ed0));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfab4;
                    continue;
                }
                0x001cfab4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfabc);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ee8));
                    call_ptr(c, 0x00317670, 0x001cfabc)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfad0;
                        continue;
                    }
                    pc = 0x001cfac4;
                    continue;
                }
                0x001cfac4 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x4a0000;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x904), c.g[S1][0]);
                    if t {
                        pc = 0x001cfbac;
                        continue;
                    }
                    pc = 0x001cfad0;
                    continue;
                }
                0x001cfad0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfad8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ef8));
                    call_ptr(c, 0x00317670, 0x001cfad8)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfb78;
                        continue;
                    }
                    pc = 0x001cfae0;
                    continue;
                }
                0x001cfae0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[V0][0] = 0x3;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x4;
                    if t {
                        pc = 0x001cfafc;
                        continue;
                    }
                    pc = 0x001cfaf0;
                    continue;
                }
                0x001cfaf0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6210));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfafc;
                    continue;
                }
                0x001cfafc => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x5;
                    if t {
                        pc = 0x001cfb10;
                        continue;
                    }
                    pc = 0x001cfb04;
                    continue;
                }
                0x001cfb04 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6228));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb10;
                    continue;
                }
                0x001cfb10 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6;
                    if t {
                        pc = 0x001cfb24;
                        continue;
                    }
                    pc = 0x001cfb18;
                    continue;
                }
                0x001cfb18 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6240));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb24;
                    continue;
                }
                0x001cfb24 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        pc = 0x001cfb38;
                        continue;
                    }
                    pc = 0x001cfb2c;
                    continue;
                }
                0x001cfb2c => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6258));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb38;
                    continue;
                }
                0x001cfb38 => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V0][0] = 0x8;
                    if t {
                        pc = 0x001cfb4c;
                        continue;
                    }
                    pc = 0x001cfb40;
                    continue;
                }
                0x001cfb40 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6270));
                    if t {
                        pc = 0x001cfb5c;
                        continue;
                    }
                    pc = 0x001cfb4c;
                    continue;
                }
                0x001cfb4c => {
                    c.cycles += 2;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfb54;
                    continue;
                }
                0x001cfb54 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6288));
                    pc = 0x001cfb5c;
                    continue;
                }
                0x001cfb5c => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001cfb64);
                    call_ptr(c, 0x00317670, 0x001cfb64)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cfb70);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001cfb70)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfb78;
                    continue;
                }
                0x001cfb78 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfb80);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f10));
                    call_ptr(c, 0x00317670, 0x001cfb80)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfb94;
                        continue;
                    }
                    pc = 0x001cfb88;
                    continue;
                }
                0x001cfb88 => {
                    c.cycles += 3;
                    c.g[A1][0] = 0x4a0000;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x908), c.g[S1][0]);
                    if t {
                        pc = 0x001cfbac;
                        continue;
                    }
                    pc = 0x001cfb94;
                    continue;
                }
                0x001cfb94 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfb9c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f20));
                    call_ptr(c, 0x00317670, 0x001cfb9c)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001cfbc0;
                        continue;
                    }
                    pc = 0x001cfba4;
                    continue;
                }
                0x001cfba4 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x90c), c.g[S1][0]);
                    pc = 0x001cfbac;
                    continue;
                }
                0x001cfbac => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001cfbb8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                    call_ptr(c, 0x003a0c50, 0x001cfbb8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfbc0;
                    continue;
                }
                0x001cfbbc => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    pc = 0x001cfbc0;
                    continue;
                }
                0x001cfbc0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfbc8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f30));
                    call_ptr(c, 0x00317670, 0x001cfbc8)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                        pc = 0x001cfbf0;
                        continue;
                    }
                    pc = 0x001cfbd0;
                    continue;
                }
                0x001cfbd0 => {
                    c.cycles += 8;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x910), c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001cfbe8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001cfbe8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    if t {
                        pc = 0x001cfce8;
                        continue;
                    }
                    pc = 0x001cfbf0;
                    continue;
                }
                0x001cfbf0 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[FP][0] = 0u64;
                    if t {
                        pc = 0x001cfce4;
                        continue;
                    }
                    pc = 0x001cfbf8;
                    continue;
                }
                0x001cfbf8 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    pc = 0x001cfc00;
                    continue;
                }
                0x001cfc00 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                    if t {
                        pc = 0x001cfcd0;
                        continue;
                    }
                    pc = 0x001cfc10;
                    continue;
                }
                0x001cfc10 => {
                    c.cycles += 12;
                    c.g[V1][0] = sx((c.g[FP][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x924));
                    c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = 0x440000;
                    c.g[V0][0] = 0x440000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14c0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1440));
                    c.g[S5][0] = c.g[V1][0];
                    c.g[S4][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9a4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    pc = 0x001cfc40;
                    continue;
                }
                0x001cfc40 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfc48);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32));
                    call_ptr(c, 0x00317670, 0x001cfc48)?;
                    let t = c.g[S7][0] != c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001cfc7c;
                        continue;
                    }
                    pc = 0x001cfc50;
                    continue;
                }
                0x001cfc50 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.mult(0, 4, 30, 2);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001cfc74);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001cfc74)?;
                    let t = true;
                    c.sw((c.g[S6][0] as u32), c.g[S1][0]);
                    if t {
                        pc = 0x001cfcb0;
                        continue;
                    }
                    pc = 0x001cfc7c;
                    continue;
                }
                0x001cfc7c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001cfc84);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32));
                    call_ptr(c, 0x00317670, 0x001cfc84)?;

                    if c.g[S7][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        pc = 0x001cfcb4;
                        continue;
                    }
                    pc = 0x001cfc8c;
                    continue;
                }
                0x001cfc8c => {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0u64;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[S1][0]);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001cfcb0);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001cfcb0)?;
                    pc = 0x001cfcb0;
                    continue;
                }
                0x001cfcb0 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    pc = 0x001cfcb4;
                    continue;
                }
                0x001cfcb4 => {
                    c.cycles += 7;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x001cfc40) {
                            return Ok(());
                        }
                        pc = 0x001cfc40;
                        continue;
                    }
                    pc = 0x001cfcd0;
                    continue;
                }
                0x001cfcd0 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[FP][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        if back_edge_unwinds(c, 0x001cfc00) {
                            return Ok(());
                        }
                        pc = 0x001cfc00;
                        continue;
                    }
                    pc = 0x001cfce4;
                    continue;
                }
                0x001cfce4 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    pc = 0x001cfce8;
                    continue;
                }
                0x001cfce8 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001cf820, 0x001cfd14);
                    return Ok(());
                }
            }
        }
    });
}

/// `void cFEStateRewardGallery::UpdateHelpText(self, int idx)` (0x001d0648):
/// hide the help text widget (`self+0x900`), look up reward `idx` through the
/// state's vtable (item, its state, whether it is owned) and show the help text
/// that fits: not owned, wrong mode, can afford or not, etc. Fetches the text by
/// string hash and sets it on the widget.
///
/// Converted from the generated labelled-block body (every register, cycle and
/// call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_reward_gallery_update_help_text(c: &mut Cpu) {
    if c.pc != 0x001d0648 {
        return ee::s04::f_001d0648__generated(c);
    }
    guest(c, |c| {
        'b_1d0874: {
            'b_1d0870: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                let t = c.g[A2][0] == 0u64;
                c.g[S3][0] = c.g[A1][0];
                if t {
                    break 'b_1d0870;
                } else {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d0688);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d0688)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x110));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x114));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001d06a0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d06a0)?;
                    c.g[S2][0] = c.g[V0][0];
                    if c.g[S2][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1d0874;
                    } else {
                        'b_1d085c: {
                            'b_1d0840: {
                                'b_1d0838: {
                                    'b_1d07a0: {
                                        c.cycles += 18;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x120));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001d06c4);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d06c4)?;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[A1][0] = 0x1;
                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001d06e4);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d06e4)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                                        c.g[V0][0] = 0x8;
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1d07a0;
                                        } else {
                                            'b_1d0718: {
                                                c.cycles += 7;
                                                c.g[A1][0] = c.g[S3][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x130));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x134));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001d0708);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001d0708)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d0718;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6f90));
                                                    if t {
                                                        break 'b_1d0840;
                                                    } else {
                                                        break 'b_1d0718;
                                                    }
                                                }
                                            }
                                            'b_1d0728: {
                                                c.cycles += 2;
                                                if (c.g[S1][0] as i64) >= 0 {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x8f0)
                                                        );
                                                    break 'b_1d0728;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    if t {
                                                        break 'b_1d0840;
                                                    } else {
                                                        break 'b_1d0728;
                                                    }
                                                }
                                            }
                                            'b_1d0798: {
                                                c.cycles += 2;
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1d0798;
                                                } else {
                                                    'b_1d0768: {
                                                        c.cycles += 2;
                                                        let t = c.g[S1][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0x1;
                                                        if t {
                                                            break 'b_1d0768;
                                                        } else {
                                                            'b_1d074c: {
                                                                c.cycles += 2;
                                                                let t = c.g[S1][0] != c.g[V0][0];
                                                                c.g[V0][0] = 0x2;
                                                                if t {
                                                                    break 'b_1d074c;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x460000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6fb0));
                                                                    if t {
                                                                        break 'b_1d0840;
                                                                    } else {
                                                                        break 'b_1d074c;
                                                                    }
                                                                }
                                                            }
                                                            'b_1d0760: {
                                                                c.cycles += 2;
                                                                if c.g[S1][0] != c.g[V0][0] {
                                                                    c.g[A0][0] = 0x460000;
                                                                    break 'b_1d0760;
                                                                } else {
                                                                    c.cycles += 3;
                                                                    c.g[A0][0] = 0x460000;
                                                                    let t = true;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x6fc8));
                                                                    if t {
                                                                        break 'b_1d0840;
                                                                    } else {
                                                                        break 'b_1d0760;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x6fe0));
                                                            if t {
                                                                break 'b_1d0840;
                                                            } else {
                                                                break 'b_1d0768;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = c.g[S2][0];
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x118)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x11c)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001d0780);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001d0780)?;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                                    c.g[V1][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[V0][0] as i64))
                                                        as u64;
                                                    let t = c.g[V1][0] == 0u64;
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1d0838;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x958));
                                                        if t {
                                                            break 'b_1d0840;
                                                        } else {
                                                            break 'b_1d0798;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x7010));
                                            if t {
                                                break 'b_1d0840;
                                            } else {
                                                break 'b_1d07a0;
                                            }
                                        }
                                    }
                                    'b_1d07c4: {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.g[S3][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x130));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x134));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001d07b4);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d07b4)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A0][0] = 0x460000;
                                        if t {
                                            break 'b_1d07c4;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x7028));
                                            if t {
                                                break 'b_1d0840;
                                            } else {
                                                break 'b_1d07c4;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8f0));
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1d085c;
                                    } else {
                                        'b_1d0808: {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_1d0808;
                                            } else {
                                                'b_1d07ec: {
                                                    c.cycles += 2;
                                                    let t = c.g[S1][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1d07ec;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = 0x460000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6fb0));
                                                        if t {
                                                            break 'b_1d0840;
                                                        } else {
                                                            break 'b_1d07ec;
                                                        }
                                                    }
                                                }
                                                'b_1d0800: {
                                                    c.cycles += 2;
                                                    if c.g[S1][0] != c.g[V0][0] {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1d0800;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = 0x460000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6fc8));
                                                        if t {
                                                            break 'b_1d0840;
                                                        } else {
                                                            break 'b_1d0800;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6fe0));
                                                if t {
                                                    break 'b_1d0840;
                                                } else {
                                                    break 'b_1d0808;
                                                }
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x118));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001d0820);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001d0820)?;
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                                        c.g[V1][0] =
                                            ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[A0][0] = 0x460000;
                                        if t {
                                            break 'b_1d0838;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x958));
                                            if t {
                                                break 'b_1d0840;
                                            } else {
                                                break 'b_1d0838;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ff8));
                                break 'b_1d0840;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001d0848);
                            call_ptr(c, 0x00317670, 0x001d0848)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                            c.g[RA][0] = sx(0x001d0854);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001d0854)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1d0874;
                            } else {
                                break 'b_1d085c;
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001d0864);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7010));
                        call_ptr(c, 0x00317670, 0x001d0864)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x900));
                        c.g[RA][0] = sx(0x001d0870);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001d0870)?;
                        break 'b_1d0870;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1d0874;
        }
        c.cycles += 6;
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

/// `void cFEStateRewardGallery::RefreshRowIcons(self, int row)` (0x001d0890):
/// for each column of `row` show or hide the cell and frame widgets
/// (`self+0x924`, `+0x9a4`) depending on whether the reward index
/// `(scroll + row) * cols + col` is below the count, and set the cell's texture
/// from the reward (vtable queries) or the locked/selected icon.
///
/// Converted from the generated labelled-block body (every register, cycle and
/// call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_reward_gallery_refresh_row_icons(c: &mut Cpu) {
    if c.pc != 0x001d0890 {
        return ee::s04::f_001d0890__generated(c);
    }
    guest(c, |c| {
        'b_1d0ad0: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A0][0];
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S7][0] = 0u64;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[FP][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
            let t = (c.g[V1][0] as i64) <= 0;
            c.mult(0, 19, 2, 3);
            if t {
                break 'b_1d0ad0;
            } else {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x924));
                c.g[V0][0] = sx((c.g[FP][0] as u32) << 5);
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[S6][0] = 0u64;
                'l_1d08f0: loop {
                    'b_1d0ab8: {
                        'b_1d0a7c: {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1d0a7c;
                            } else {
                                'b_1d0a50: {
                                    'b_1d0a4c: {
                                        'b_1d09c0: {
                                            c.cycles += 23;
                                            c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                                            c.g[S4][0] = sx((c.g[FP][0] as u32) << 5);
                                            c.g[S5][0] =
                                                sx((c.g[S1][0] as u32).wrapping_add(0x924));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001d0920);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001d0920)?;
                                            c.g[A2][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                                            c.g[A1][0] = 0x1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001d093c);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001d093c)?;
                                            c.g[A2][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A0][0] =
                                                c.lh((c.g[A2][0] as u32).wrapping_add(0x130));
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x134));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001d0954);
                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001d0954)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[S3][0];
                                            if t {
                                                break 'b_1d09c0;
                                            } else {
                                                'b_1d098c: {
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0xd4));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001d0970);
                                                    c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001d0970)?;
                                                    c.g[V1][0] = c.g[V0][0];
                                                    if (c.g[V1][0] as i64) < 0 {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        break 'b_1d098c;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x78),
                                                            c.g[V1][0],
                                                        );
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                            0u64,
                                                        );
                                                        if t {
                                                            break 'b_1d0a4c;
                                                        } else {
                                                            break 'b_1d098c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.g[A1][0] = c.g[S3][0];
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001d09a0);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001d09a0)?;
                                                c.g[V1][0] = c.g[V0][0];
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[V0][0] = sx((c.g[S6][0] as u32)
                                                    .wrapping_add((c.g[S4][0] as u32)));
                                                if t {
                                                    break 'b_1d0a50;
                                                } else {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x7c),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V1][0] = 0xffffffffffffffff;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x78),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_1d0a4c;
                                                    } else {
                                                        break 'b_1d09c0;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1d0a1c: {
                                            'b_1d0a14: {
                                                c.cycles += 9;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x110));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x114));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001d09d4);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001d09d4)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8f0));
                                                if (c.g[V0][0] as i64) < 0 {
                                                    c.g[A0][0] = 0x460000;
                                                    break 'b_1d0a14;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x120)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x001d09f8);
                                                    c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001d09f8)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x8f0)
                                                        );
                                                    if c.g[V0][0] != c.g[V1][0] {
                                                        c.g[A0][0] = 0x460000;
                                                        break 'b_1d0a14;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x7040));
                                                        if t {
                                                            break 'b_1d0a1c;
                                                        } else {
                                                            break 'b_1d0a14;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x7050));
                                            break 'b_1d0a1c;
                                        }
                                        c.cycles += 12;
                                        c.g[RA][0] = sx(0x001d0a24);
                                        call_ptr(c, 0x00317670, 0x001d0a24)?;
                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                        c.g[RA][0] = sx(0x001d0a30);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x00398380, 0x001d0a30)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V0][0] =
                                            sx((c.g[S6][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                        c.g[V0][0] =
                                            sx((c.g[S5][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                        c.g[V0][0] = 0xffffffffffffffff;
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[A0][0]);
                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                                        break 'b_1d0a4c;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                                    break 'b_1d0a50;
                                }
                                c.cycles += 11;
                                c.g[V1][0] = 0x4c0000;
                                c.g[V0][0] =
                                    sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6818));
                                c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001d0a74);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001d0a74)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                                if t {
                                    break 'b_1d0ab8;
                                } else {
                                    break 'b_1d0a7c;
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001d0a98);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001d0a98)?;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001d0ab4);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001d0ab4)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                        break 'b_1d0ab8;
                    }
                    c.cycles += 6;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001d08f0);
                        }
                        continue 'l_1d08f0;
                    } else {
                        break 'b_1d0ad0;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStatePreviewReward::OnWidgetCreated(self, UIWidget* w)` (0x001d1c98):
/// null `w` returns at once. Otherwise match its name hash (`w+0x38`): the icon
/// texture or title text by reward type (`self+0x68`), the reward name text,
/// the three frame shapes (hidden, `self+0x54..`) and bitmaps (`self+0x48..`).
/// Afterwards show the frame of the selected shape (`self+0x74`).
///
/// Converted from the generated body (a `match pc` body; every register, cycle
/// and call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_preview_reward_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d1c98 {
        return ee::s04::f_001d1c98__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001d1c98;
        loop {
            match pc {
                0x001d1c98 => {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1cb8;
                    continue;
                }
                0x001d1cb8 => {
                    c.cycles += 6;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001d1cc8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6df8));
                    call_ptr(c, 0x00317670, 0x001d1cc8)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1d6c;
                        continue;
                    }
                    pc = 0x001d1cd0;
                    continue;
                }
                0x001d1cd0 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[V0][0] = 0x4;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                    if t {
                        pc = 0x001d1d20;
                        continue;
                    }
                    pc = 0x001d1ce0;
                    continue;
                }
                0x001d1ce0 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x3;
                    if t {
                        pc = 0x001d1cf8;
                        continue;
                    }
                    pc = 0x001d1ce8;
                    continue;
                }
                0x001d1ce8 => {
                    c.cycles += 2;

                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1d14;
                        continue;
                    }
                    pc = 0x001d1cf0;
                    continue;
                }
                0x001d1cf0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1cf8;
                    continue;
                }
                0x001d1cf8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7;
                    if t {
                        pc = 0x001d1d30;
                        continue;
                    }
                    pc = 0x001d1d04;
                    continue;
                }
                0x001d1d04 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001d1d40;
                        continue;
                    }
                    pc = 0x001d1d0c;
                    continue;
                }
                0x001d1d0c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1d14;
                    continue;
                }
                0x001d1d14 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e08));
                    if t {
                        pc = 0x001d1d48;
                        continue;
                    }
                    pc = 0x001d1d20;
                    continue;
                }
                0x001d1d20 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e18));
                    if t {
                        pc = 0x001d1d48;
                        continue;
                    }
                    pc = 0x001d1d30;
                    continue;
                }
                0x001d1d30 => {
                    c.cycles += 4;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e30));
                    if t {
                        pc = 0x001d1d48;
                        continue;
                    }
                    pc = 0x001d1d40;
                    continue;
                }
                0x001d1d40 => {
                    c.cycles += 2;
                    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e50));
                    pc = 0x001d1d48;
                    continue;
                }
                0x001d1d48 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x001d1d50);
                    call_ptr(c, 0x00317670, 0x001d1d50)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001d1d5c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x00398380, 0x001d1d5c)?;
                    c.g[V1][0] = 0xffffffffffffffff;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1d6c;
                    continue;
                }
                0x001d1d6c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1d74);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e60));
                    call_ptr(c, 0x00317670, 0x001d1d74)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001d1dfc;
                        continue;
                    }
                    pc = 0x001d1d7c;
                    continue;
                }
                0x001d1d7c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1d90;
                    continue;
                }
                0x001d1d90 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001d1da8 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e70));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1db4;
                    continue;
                }
                0x001d1db4 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6e88));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1dc0;
                    continue;
                }
                0x001d1dc0 => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ea0));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1dcc;
                    continue;
                }
                0x001d1dcc => {
                    c.cycles += 3;
                    c.g[A0][0] = 0x460000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7150));
                    if t {
                        pc = 0x001d1de0;
                        continue;
                    }
                    pc = 0x001d1dd8;
                    continue;
                }
                0x001d1dd8 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6ec0));
                    pc = 0x001d1de0;
                    continue;
                }
                0x001d1de0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x001d1de8);
                    call_ptr(c, 0x00317670, 0x001d1de8)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d1df4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001d1df4)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1dfc;
                    continue;
                }
                0x001d1dfc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e04);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6f10));
                    call_ptr(c, 0x00317670, 0x001d1e04)?;
                    let t = c.g[S0][0] != c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    if t {
                        pc = 0x001d1e5c;
                        continue;
                    }
                    pc = 0x001d1e0c;
                    continue;
                }
                0x001d1e0c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                    c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1e20;
                    continue;
                }
                0x001d1e20 => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x71f0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x001d1e38 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x001d1ea8;
                        continue;
                    }
                    pc = 0x001d1e44;
                    continue;
                }
                0x001d1e44 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001d1e54);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x003a0c50, 0x001d1e54)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1e5c;
                    continue;
                }
                0x001d1e5c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7160));
                    call_ptr(c, 0x00317670, 0x001d1e64)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1e74;
                        continue;
                    }
                    pc = 0x001d1e6c;
                    continue;
                }
                0x001d1e6c => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x54), c.g[S1][0]);
                    if t {
                        pc = 0x001d1ea0;
                        continue;
                    }
                    pc = 0x001d1e74;
                    continue;
                }
                0x001d1e74 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e7c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7170));
                    call_ptr(c, 0x00317670, 0x001d1e7c)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1e8c;
                        continue;
                    }
                    pc = 0x001d1e84;
                    continue;
                }
                0x001d1e84 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                    if t {
                        pc = 0x001d1ea0;
                        continue;
                    }
                    pc = 0x001d1e8c;
                    continue;
                }
                0x001d1e8c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1e94);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7188));
                    call_ptr(c, 0x00317670, 0x001d1e94)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1ec0;
                        continue;
                    }
                    pc = 0x001d1e9c;
                    continue;
                }
                0x001d1e9c => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x5c), c.g[S1][0]);
                    pc = 0x001d1ea0;
                    continue;
                }
                0x001d1ea0 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    pc = 0x001d1ea8;
                    continue;
                }
                0x001d1ea8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d1eb8);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d1eb8)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    if t {
                        pc = 0x001d1f04;
                        continue;
                    }
                    pc = 0x001d1ec0;
                    continue;
                }
                0x001d1ec0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1ec8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7198));
                    call_ptr(c, 0x00317670, 0x001d1ec8)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1ed8;
                        continue;
                    }
                    pc = 0x001d1ed0;
                    continue;
                }
                0x001d1ed0 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1ed8;
                    continue;
                }
                0x001d1ed8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1ee0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x71a8));
                    call_ptr(c, 0x00317670, 0x001d1ee0)?;

                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x460000;
                        pc = 0x001d1ef0;
                        continue;
                    }
                    pc = 0x001d1ee8;
                    continue;
                }
                0x001d1ee8 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[S1][0]);
                    if t {
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1ef0;
                    continue;
                }
                0x001d1ef0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001d1ef8);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x71c0));
                    call_ptr(c, 0x00317670, 0x001d1ef8)?;

                    if c.g[S0][0] == c.g[V0][0] {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[S1][0]);
                        pc = 0x001d1f00;
                        continue;
                    }
                    pc = 0x001d1f00;
                    continue;
                }
                0x001d1f00 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    pc = 0x001d1f04;
                    continue;
                }
                0x001d1f04 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1f0c;
                    continue;
                }
                0x001d1f0c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                    let t = c.g[A0][0] == 0u64;
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1f1c;
                    continue;
                }
                0x001d1f1c => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x001d1f5c;
                        continue;
                    }
                    pc = 0x001d1f2c;
                    continue;
                }
                0x001d1f2c => {
                    c.cycles += 12;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x70));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x7c), 0u64);
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x78), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x74));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001d1f5c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d1f5c)?;
                    pc = 0x001d1f5c;
                    continue;
                }
                0x001d1f5c => {
                    c.cycles += 6;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001d1c98, 0x001d1f74);
                    return Ok(());
                }
            }
        }
    });
}

/// `cFEStateMountainRoom* cFEStateMountainRoom::ctor(this, a1, signed char player)`
/// (0x001d2678): base-state constructor, vtable, player slot, zeroed member
/// arrays, then the first character slot with a rider event record
/// (`BENewPlayerInt_GetCharStateFlag` over the three slots), the reward lookups,
/// two string-hash texture lookups, and `RequestImage(this)`; returns `this`.
///
/// Converted from the generated labelled-block body (every register, cycle and
/// call kept). Any other `pc` (a resume) runs the generated body.
pub fn c_fe_state_mountain_room_ctor(c: &mut Cpu) {
    if c.pc != 0x001d2678 {
        return ee::s04::f_001d2678__generated(c);
    }
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = sx((c.g[A2][0] as u32) << 24);
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x001d26a4);
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, 0x0039e2a0, 0x001d26a4)?;
        c.g[V0][0] = 0x2b;
        c.g[V1][0] = 0x470000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x67a8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        c.sb((c.g[S1][0] as u32).wrapping_add(0x44), c.g[S0][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x001d26cc);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
        call_ptr(c, 0x001a1cd0, 0x001d26cc)?;
        c.sb((c.g[S1][0] as u32).wrapping_add(0x15), c.g[V0][0]);
        c.g[V1][0] = 0x4;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x48), 0u64);
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x68));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x50), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
        'l_1d26e8: loop {
            c.cycles += 7;
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            let t = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001d26e8);
                }
                continue 'l_1d26e8;
            } else {
                c.cycles += 7;
                c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x78));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x84));
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x90));
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x9c));
                c.g[V1][0] = 0x2;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                'l_1d2720: loop {
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[V1][0] as i64) >= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001d2720);
                        }
                        continue 'l_1d2720;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = 0x2;
                        'l_1d2748: loop {
                            c.cycles += 7;
                            c.sw((c.g[V0][0] as u32), 0u64);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V1][0] as i64) >= 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001d2748);
                                }
                                continue 'l_1d2748;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = 0x2;
                                'l_1d2770: loop {
                                    c.cycles += 7;
                                    c.sw((c.g[V0][0] as u32), 0u64);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[V1][0] as i64) >= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001d2770);
                                        }
                                        continue 'l_1d2770;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = 0x2;
                                        'l_1d2798: loop {
                                            c.cycles += 7;
                                            c.sw((c.g[V0][0] as u32), 0u64);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001d2798);
                                                }
                                                continue 'l_1d2798;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    sx((c.g[A3][0] as u32).wrapping_add(0x8));
                                                c.g[V1][0] = 0x2;
                                                'l_1d27c0: loop {
                                                    c.cycles += 7;
                                                    c.sw((c.g[V0][0] as u32), 0u64);
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                    let t = (c.g[V1][0] as i64) >= 0;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001d27c0);
                                                        }
                                                        continue 'l_1d27c0;
                                                    } else {
                                                        c.cycles += 13;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0xa8),
                                                            0u64,
                                                        );
                                                        c.g[S0][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001d27ec);
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0xac),
                                                            0u64,
                                                        );
                                                        call_ptr(c, 0x0014dc80, 0x001d27ec)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x001d27f8);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x0014dd58, 0x001d27f8)?;
                                                        c.g[S2][0] = c.g[V0][0];
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[RA][0] = sx(0x001d2808);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00147488, 0x001d2808)?;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0xb0),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        'l_1d2810: loop {
                                                            'b_1d2828: {
                                                                c.cycles += 6;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb0));
                                                                c.g[RA][0] = sx(0x001d2820);
                                                                c.g[A3][0] = c.g[S0][0];
                                                                call_ptr(
                                                                    c, 0x00145f90, 0x001d2820,
                                                                )?;
                                                                if c.g[V0][0] == 0u64 {
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0xb4),
                                                                        c.g[S0][0],
                                                                    );
                                                                    break 'b_1d2828;
                                                                } else {
                                                                    break 'b_1d2828;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] =
                                                                ((c.g[S0][0] as i64) < 3i64) as u64;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.g[A1][0] = c
                                                                    .lb((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x44));
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001d2810,
                                                                    );
                                                                }
                                                                continue 'l_1d2810;
                                                            } else {
                                                                c.cycles += 48;
                                                                c.g[RA][0] = sx(0x001d2840);
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x001d2840,
                                                                )?;
                                                                c.g[A1][0] = 0x3;
                                                                c.g[RA][0] = sx(0x001d284c);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x001d284c,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lb((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x44));
                                                                c.g[RA][0] = sx(0x001d2858);
                                                                c.g[S0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0014a130, 0x001d2858,
                                                                )?;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x001d2864);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00148950, 0x001d2864,
                                                                )?;
                                                                c.g[RA][0] = sx(0x001d286c);
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb8),
                                                                    c.g[V0][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x001d286c,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001d2878);
                                                                c.g[A1][0] = 0xd;
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x001d2878,
                                                                )?;
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb0));
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001d2888);
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0xb4));
                                                                call_ptr(
                                                                    c, 0x00156b28, 0x001d2888,
                                                                )?;
                                                                c.g[V1][0] = 0xffffffffffffffff;
                                                                c.g[A0][0] = 0x460000;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x7208));
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xbc),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[RA][0] = sx(0x001d28a4);
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xc0),
                                                                    c.g[V0][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001d28a4,
                                                                )?;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[RA][0] = sx(0x001d28b0);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00398380, 0x001d28b0,
                                                                )?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xac),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[RA][0] = sx(0x001d28c0);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x7218));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001d28c0,
                                                                )?;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[RA][0] = sx(0x001d28cc);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x00398380, 0x001d28cc,
                                                                )?;
                                                                c.sw(
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0xa8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[RA][0] = sx(0x001d28d8);
                                                                c.g[A0][0] = c.g[S1][0];
                                                                Callee { addr: 0x001d3160, f: ee::s04::cFEStateMountainRoom_RequestImage }.call(c, 0x001d28d8)?;
                                                                c.g[V0][0] = c.g[S1][0];
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.lq(
                                                                    17,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                c.lq(
                                                                    18,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                c.lq(
                                                                    19,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.g[RA][0] =
                                                                    c.ld((c.g[SP][0] as u32));
                                                                let t = c.g[RA][0] as u32;
                                                                c.g[SP][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0x50));
                                                                c.pc = t;
                                                                return Ok(());
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
        }
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_COUNT, LOG_START};
    use crate::difftest::{check, elf_image, fresh, run_generated, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (in the machine's RAM: off in every
    /// other test) and the magic that turns them on.
    pub(crate) const OC935_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC935_MAGIC: u32 = 0x0935_c0de;
    /// Number of `GetHashValue32` calls so far, and the call that matches.
    pub(crate) const OC935_HASH_N: u32 = OC935_CTRL + 4;
    pub(crate) const OC935_HASH_TARGET: u32 = OC935_CTRL + 8;
    /// Result mailboxes (0x20 bytes per fake callee index).
    pub(crate) const OC935_MAIL: u32 = OC935_CTRL + 0x100;
    /// The hash every widget carries at `+0x38` (what a matching call returns).
    pub(crate) const OC935_HASH: u32 = 0x8765_4321;
    pub(crate) const OC935_ZERO: u32 = 1;
    pub(crate) const OC935_NEG1: u32 = 2;

    pub(crate) const OC935_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC935_WIDGET: u32 = DATA + 0x3000;
    pub(crate) const OC935_VT_SELF: u32 = DATA + 0x6000;
    pub(crate) const OC935_VT_W: u32 = DATA + 0x6800;

    /// Mailbox word returning `v` (0 leaves the fake's own result).
    pub(crate) fn oc935_val(v: u32) -> u32 {
        v + 3
    }

    /// The fake callee `idx` answers its calls from these four words in turn.
    pub(crate) fn oc935_mail(c: &mut Cpu, idx: u32, words: [u32; 4]) {
        for (i, w) in words.iter().enumerate() {
            c.bus.write32(OC935_MAIL + 0x20 * idx + 4 * i as u32, *w);
        }
    }

    pub(crate) fn oc935_apply_mail(c: &mut Cpu, idx: u32) {
        let base = OC935_MAIL + 0x20 * idx;
        let k = c.bus.read32(base + 0x10);
        c.bus.write32(base + 0x10, k + 1);
        match c.bus.read32(base + 4 * (k % 4)) {
            0 => {}
            OC935_ZERO => c.g[V0][0] = 0,
            OC935_NEG1 => c.g[V0][0] = !0,
            m => c.g[V0][0] = sx(m - 3),
        }
    }

    /// `miss` hook: the vtable fakes (`FAKE + 0x10 * idx`) answer from their
    /// mailbox once the gate is on.
    pub(crate) fn oc935_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra
            && c.bus.read32(OC935_CTRL) == OC935_MAGIC
            && (FAKE..FAKE + 0x200).contains(&target)
        {
            oc935_apply_mail(c, (target - FAKE) / 0x10);
        }
    }

    /// `GetHashValue32` fake: call number `OC935_HASH_TARGET` returns the
    /// widgets' hash (a match), every other call something else.
    pub(crate) fn oc935_fake_hash(c: &mut Cpu) {
        if c.bus.read32(OC935_CTRL) != OC935_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc935_fake_hash),
                ee::s10::f_00317670__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC935_HASH_N);
        c.bus.write32(OC935_HASH_N, n + 1);
        fakes::fake(c, 0x0031_7670, ra);
        if c.pc == ra {
            let hit = n == c.bus.read32(OC935_HASH_TARGET);
            c.g[V0][0] = sx(if hit { OC935_HASH } else { OC935_HASH ^ 1 });
        }
    }

    /// Gated fakes in the override slots of the direct callees: each logs its
    /// call like `fakes::fake`, then answers from its mailbox; gate off, the
    /// slot runs the body it held.
    macro_rules! oc935_gated {
        ($($fake:ident => $addr:literal, $idx:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC935_CTRL) != OC935_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                    if c.pc == ra {
                        oc935_apply_mail(c, $idx);
                    }
                }
            )*

            pub(crate) fn oc935_install_fakes() {
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317670),
                    oc935_fake_hash,
                    fakes::prev_cell(oc935_fake_hash),
                );
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc935_gated! {
        oc935_fake_398380 => 0x0039_8380, 24, ee::s13::OVERRIDE_00398380, ee::s13::f_00398380__generated;
        oc935_fake_3a0c50 => 0x003a_0c50, 25, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated;
        oc935_fake_3a1000 => 0x003a_1000, 26, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated;
        oc935_fake_417828 => 0x0041_7828, 27, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc935_fake_145f90 => 0x0014_5f90, 28, ee::s01::OVERRIDE_00145F90, ee::s01::f_00145f90__generated;
        oc935_fake_147488 => 0x0014_7488, 29, ee::s01::OVERRIDE_00147488, ee::s01::f_00147488__generated;
        oc935_fake_148950 => 0x0014_8950, 29, ee::s01::OVERRIDE_00148950, ee::s01::f_00148950__generated;
        oc935_fake_14a130 => 0x0014_a130, 29, ee::s01::OVERRIDE_0014A130, ee::s01::f_0014a130__generated;
        oc935_fake_14dc80 => 0x0014_dc80, 29, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc935_fake_14dd58 => 0x0014_dd58, 29, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc935_fake_156b28 => 0x0015_6b28, 29, ee::s01::OVERRIDE_00156B28, ee::s01::f_00156b28__generated;
        oc935_fake_1a1cd0 => 0x001a_1cd0, 29, ee::s03::OVERRIDE_001A1CD0, ee::s03::f_001a1cd0__generated;
        oc935_fake_1d3160 => 0x001d_3160, 29, ee::s04::OVERRIDE_001D3160, ee::s04::f_001d3160__generated;
        oc935_fake_39e2a0 => 0x0039_e2a0, 29, ee::s13::OVERRIDE_0039E2A0, ee::s13::f_0039e2a0__generated;
    }

    /// Turn the fakes on: gate, mailbox-aware `miss` hook, and the call
    /// number of `GetHashValue32` that matches the widget's hash.
    pub(crate) fn oc935_arm(c: &mut Cpu, hash_target: u32) {
        oc935_install_fakes();
        c.hooks.miss = oc935_miss;
        c.bus.write32(OC935_CTRL, OC935_MAGIC);
        c.bus.write32(OC935_HASH_N, 0);
        c.bus.write32(OC935_HASH_TARGET, hash_target);
    }

    /// A vtable slot `{delta, FAKE + 0x10 * idx}`.
    pub(crate) fn oc935_vslot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32, idx: u32) {
        c.bus.write16(vt + off, ((r.next_u32() % 8) * 4) as u16);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * idx);
    }

    /// The widget vtable (set visible `+0x48`, set value `+0x58`) and the
    /// state vtable (`+0x158`, `+0x110`, `+0x120`, `+0x130`, `+0x118`, `+0xd0`,
    /// `+0xd8`), `self+8` and `widget+8` pointing at them.
    pub(crate) fn oc935_vtables(c: &mut Cpu, r: &mut Rng) {
        oc935_vslot(c, r, OC935_VT_W, 0x48, 0);
        oc935_vslot(c, r, OC935_VT_W, 0x58, 1);
        for (off, idx) in [
            (0x158, 2),
            (0x110, 3),
            (0x120, 4),
            (0x130, 5),
            (0x118, 6),
            (0xd0, 7),
            (0xd8, 8),
        ] {
            oc935_vslot(c, r, OC935_VT_SELF, off, idx);
        }
        c.bus.write32(OC935_SELF + 8, OC935_VT_SELF);
    }

    /// An object that has the widget vtable.
    pub(crate) fn oc935_widget(c: &mut Cpu, obj: u32) {
        c.bus.write32(obj + 8, OC935_VT_W);
        c.bus.write32(obj + 0x38, OC935_HASH);
    }

    /// Distinct call-target sequences the generated body makes over `seeds`
    /// (a path count: the setups must reach at least `at_least` of them).
    pub(crate) fn oc935_require_sequences(
        generated: GuestFn,
        seeds: u32,
        at_least: usize,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            let n = c.bus.read32(LOG_COUNT).min(64);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(LOG_START + 0x220 * i))
                .collect();
            seen.insert(seq);
        }
        assert!(
            seen.len() >= at_least,
            "only {} distinct call sequences (want {at_least})",
            seen.len()
        );
    }

    /// Reward types seen by the type switches (3..8 in the tables, others out of range).
    const OC935_TYPES: [u32; 10] = [3, 4, 5, 6, 7, 8, 9, 0, 2, 12];

    /// Fixture for `cFEStateRewardGallery_OnWidgetCreated`: the widget's hash
    /// matches the k-th `GetHashValue32` call (even seeds favour the three
    /// type-dependent branches, odd seeds walk the chain and the cell grid).
    fn oc935_gallery_created_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1cf820, 8);
        elf_image(c);
        let k = seed / 2;
        let (target, ty) = if seed % 2 == 0 {
            (
                [8, 9, 11][(k % 3) as usize],
                OC935_TYPES[(k / 3 % 10) as usize],
            )
        } else {
            (k % 41, OC935_TYPES[(r.next_u32() % 10) as usize])
        };
        oc935_arm(c, target);
        c.pc = 0x001c_f820;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        c.bus.write32(this + 0x4c, ty);
        c.bus.write32(this + 0x58, 1 + r.next_u32() % 3);
        c.bus.write32(this + 0x5c, 1 + r.next_u32() % 3);
        c.bus.write32(this + 0x68, r.next_u32() % 100);
        c.bus.write32(this + 0x6c, r.next_u32() % 100);
        c.bus.write32(this + 0x10, DATA + 0x4000);
        oc935_widget(c, OC935_WIDGET);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(OC935_WIDGET);
    }

    /// Every widget name of the screen, the type-dependent icons, titles and
    /// captions, the cell/frame grid, and unwinds of the calls.
    #[test]
    fn c_fe_state_reward_gallery_on_widget_created_matches() {
        check(
            ee::s04::f_001cf820__generated,
            c_fe_state_reward_gallery_on_widget_created,
            216,
            &oc935_gallery_created_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001cf820__generated,
            216,
            25,
            &oc935_gallery_created_setup,
        );
    }

    /// Fixture for `cFEStateRewardGallery_UpdateHelpText`: no help widget,
    /// no item, item state below/equal/above the mode, owned or not, both
    /// screen types.
    fn oc935_help_text_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d0648, 9);
        elf_image(c);
        oc935_arm(c, 99);
        c.pc = 0x001d_0648;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        let txt = if seed % 7 == 0 { 0 } else { OC935_WIDGET };
        c.bus.write32(this + 0x900, txt);
        oc935_widget(c, OC935_WIDGET);
        c.bus
            .write32(this + 0x4c, if (seed / 25) % 2 == 0 { 8 } else { 5 });
        let sel = [0xffff_ffff, 0, 1, 2, 5][((seed / 5) % 5) as usize];
        c.bus.write32(this + 0x8f0, sel);
        c.bus.write32(this + 0x54, 50);
        let item = if seed % 9 == 0 {
            OC935_ZERO
        } else {
            oc935_val(DATA + 0x2000)
        };
        oc935_mail(c, 3, [item; 4]);
        let state = [
            OC935_NEG1,
            oc935_val(0),
            oc935_val(1),
            oc935_val(2),
            oc935_val(5),
        ][(seed % 5) as usize];
        oc935_mail(c, 4, [state; 4]);
        let has = if (seed / 50) % 2 == 0 {
            OC935_ZERO
        } else {
            oc935_val(1)
        };
        oc935_mail(c, 5, [has; 4]);
        let cost = if (seed / 100) % 2 == 0 {
            oc935_val(0)
        } else {
            oc935_val(100)
        };
        oc935_mail(c, 6, [cost; 4]);
        c.bus.write32(DATA + 0x2008, r.next_u32());
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (seed % 12) as u64;
    }

    #[test]
    fn c_fe_state_reward_gallery_update_help_text_matches() {
        check(
            ee::s04::f_001d0648__generated,
            c_fe_state_reward_gallery_update_help_text,
            400,
            &oc935_help_text_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d0648__generated,
            400,
            8,
            &oc935_help_text_setup,
        );
    }

    /// Fixture for `cFEStateRewardGallery_RefreshRowIcons`: rows 0..2, one to
    /// four columns, scroll offsets, shown/hidden cells (count), owned and
    /// unowned rewards with the three icon sources, selected or not.
    fn oc935_row_icons_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d0890, 12);
        elf_image(c);
        oc935_arm(c, 99);
        c.pc = 0x001d_0890;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        let row = seed % 3;
        let cols = 1 + (seed / 9) % 4;
        c.bus.write32(this + 0x64, (seed / 3) % 3);
        c.bus.write32(this + 0x5c, cols);
        c.bus.write32(this + 0x68, r.next_u32() % 15);
        let sel = [0xffff_ffff, 5, 6][((seed / 36) % 3) as usize];
        c.bus.write32(this + 0x8f0, sel);
        c.bus.write32(this + 0x10, DATA + 0x4000);
        for rr in 0..3 {
            for col in 0..8 {
                let cell = DATA + 0x8000 + 0x100 * (rr * 8 + col);
                let frame = DATA + 0xa000 + 0x100 * (rr * 8 + col);
                oc935_widget(c, cell);
                oc935_widget(c, frame);
                c.bus.write32(this + 0x924 + 0x20 * rr + 4 * col, cell);
                c.bus.write32(this + 0x9a4 + 0x20 * rr + 4 * col, frame);
            }
        }
        let pick = |r: &mut Rng, f: &dyn Fn(u32) -> u32| -> [u32; 4] {
            [
                f(r.next_u32()),
                f(r.next_u32()),
                f(r.next_u32()),
                f(r.next_u32()),
            ]
        };
        let owned = pick(&mut r, &|x| {
            if x % 2 == 0 {
                OC935_ZERO
            } else {
                oc935_val(1)
            }
        });
        oc935_mail(c, 5, owned);
        let a = pick(&mut r, &|x| {
            if x % 2 == 0 {
                OC935_NEG1
            } else {
                oc935_val(x % 3)
            }
        });
        oc935_mail(c, 7, a);
        let b = pick(&mut r, &|x| {
            if x % 2 == 0 {
                OC935_ZERO
            } else {
                oc935_val(7 + x % 5)
            }
        });
        oc935_mail(c, 8, b);
        let id = pick(&mut r, &|x| oc935_val(x % 8));
        oc935_mail(c, 3, id);
        let state = pick(&mut r, &|x| oc935_val(5 + x % 2));
        oc935_mail(c, 4, state);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = row as u64;
    }

    #[test]
    fn c_fe_state_reward_gallery_refresh_row_icons_matches() {
        check(
            ee::s04::f_001d0890__generated,
            c_fe_state_reward_gallery_refresh_row_icons,
            324,
            &oc935_row_icons_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d0890__generated,
            324,
            15,
            &oc935_row_icons_setup,
        );
    }

    /// Fixture for `cFEStatePreviewReward_OnWidgetCreated`: null widget,
    /// every name of the chain with every reward type, frames and bitmaps
    /// stored, and the selected-frame update (missing items or frames,
    /// no selection).
    fn oc935_preview_created_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d1c98, 8);
        elf_image(c);
        let k = seed / 2;
        let (target, ty) = if seed % 2 == 0 {
            (
                [0, 1, 2][(k % 3) as usize],
                OC935_TYPES[(k / 3 % 10) as usize],
            )
        } else {
            (k % 12, OC935_TYPES[(r.next_u32() % 10) as usize])
        };
        oc935_arm(c, target);
        c.pc = 0x001d_1c98;
        oc935_vtables(c, &mut r);
        let this = OC935_SELF;
        c.bus.write32(this + 0x68, ty);
        let sel = [0xffff_ffff, 0, 1, 2][((seed / 3) % 4) as usize];
        c.bus.write32(this + 0x74, sel);
        c.bus.write32(this + 0x70, r.next_u32());
        c.bus.write32(this + 0x10, DATA + 0x4000);
        c.bus.write32(this + 0x60, DATA + 0x4400);
        c.bus.write32(DATA + 0x4400, DATA + 0x4500);
        for i in 0..3u32 {
            let item = DATA + 0x8000 + 0x100 * i;
            let frame = DATA + 0x9000 + 0x100 * i;
            oc935_widget(c, item);
            oc935_widget(c, frame);
            let have_item = (seed >> (i + 3)) % 4 != 0;
            let have_frame = (seed >> (i + 5)) % 4 != 0;
            c.bus
                .write32(this + 0x48 + 4 * i, if have_item { item } else { 0 });
            c.bus
                .write32(this + 0x54 + 4 * i, if have_frame { frame } else { 0 });
        }
        oc935_widget(c, OC935_WIDGET);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = if seed % 13 == 0 { 0 } else { sx(OC935_WIDGET) };
    }

    #[test]
    fn c_fe_state_preview_reward_on_widget_created_matches() {
        check(
            ee::s04::f_001d1c98__generated,
            c_fe_state_preview_reward_on_widget_created,
            180,
            &oc935_preview_created_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d1c98__generated,
            180,
            12,
            &oc935_preview_created_setup,
        );
    }

    /// Fixture for `cFEStateMountainRoom_ctor`: the three character-slot
    /// probes answer zero or not, a negative or positive player byte, and
    /// calls unwind at random points.
    fn oc935_mountain_room_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1d2678, 28);
        elf_image(c);
        oc935_arm(c, 99);
        c.pc = 0x001d_2678;
        let this = OC935_SELF;
        c.bus.write32(this + 0x10, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000, DATA + 0x4100);
        let probe: [u32; 4] = std::array::from_fn(|_| {
            if r.next_u32() % 2 == 0 {
                OC935_ZERO
            } else {
                oc935_val(1)
            }
        });
        oc935_mail(c, 28, probe);
        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(DATA + 0x2000);
        c.g[A2][0] = (r.next_u32() % 256) as u64;
    }

    #[test]
    fn c_fe_state_mountain_room_ctor_matches() {
        check(
            ee::s04::f_001d2678__generated,
            c_fe_state_mountain_room_ctor,
            200,
            &oc935_mountain_room_setup,
        );
        oc935_require_sequences(
            ee::s04::f_001d2678__generated,
            200,
            6,
            &oc935_mountain_room_setup,
        );
    }
}
