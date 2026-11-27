//! Staging module oc1125 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1125`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::events::back_edge_unwinds;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001a_5898,
    c_fe_state_net_messenger_build_buddy_list,
    "cFEStateNetMessenger_BuildBuddyList",
)];

/// `cFEStateNetMessenger::BuildBuddyList` (0x001a5898): rebuilds the messenger
/// screen's name list from the BuddyMan session. Returns at once while the
/// roster is not done (`session[0xe8] == 0`); otherwise clears the list
/// (`ClearNameList`), then appends, as `cBXString`s: the session's buddy
/// names that `0x262468` accepts (>= 0), then the online names with the flag
/// byte `name[167]` set (not blocked, not away), then the away ones, then the
/// remaining online names; finally resizes the `playerMenu` scroll list when
/// its item count differs from the list length. Every `GetSession` inline is
/// the `gp+0x238` singleton check with its three init calls. Converted from
/// the generated body (a `match pc` body: every register, cycle and call
/// kept); any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_messenger_build_buddy_list(c: &mut Cpu) {
    if c.pc != 0x001a5898 {
        return ee::s03::f_001a5898__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x001a5898;
    loop {
        match pc {
            0x001a5898 => {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] != 0u64; 
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t { pc = 0x001a58c8; continue; }
                pc = 0x001a58c0; continue;
            }
            0x001a58c0 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a58c8);
                call_ptr(c, 0x00260f80, 0x001a58c8)?;
                pc = 0x001a58c8; continue;
            }
            0x001a58c8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a58d0);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a58d0)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a58e4; continue; }
                pc = 0x001a58d8; continue;
            }
            0x001a58d8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a58e0);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a58e0)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a58e4; continue;
            }
            0x001a58e4 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a58f8; continue; }
                pc = 0x001a58ec; continue;
            }
            0x001a58ec => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a58f4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a58f4)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a58f8; continue;
            }
            0x001a58f8 => {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe8));
                let t = c.g[V1][0] == 0u64; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if t { pc = 0x001a5f9c; continue; }
                pc = 0x001a5904; continue;
            }
            0x001a5904 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a590c);
                c.g[A0][0] = c.g[S2][0];
                Callee { addr: 0x001a5450, f: ee::s03::cFEStateNetMessenger_ClearNameList }.call(c, 0x001a590c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5920; continue; }
                pc = 0x001a5918; continue;
            }
            0x001a5918 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5920);
                call_ptr(c, 0x00260f80, 0x001a5920)?;
                pc = 0x001a5920; continue;
            }
            0x001a5920 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5928);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5928)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a593c; continue; }
                pc = 0x001a5930; continue;
            }
            0x001a5930 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5938);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5938)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a593c; continue;
            }
            0x001a593c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a594c; continue; }
                pc = 0x001a5944; continue;
            }
            0x001a5944 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a594c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a594c)?;
                pc = 0x001a594c; continue;
            }
            0x001a594c => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a5954);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00262598, 0x001a5954)?;
                c.g[S3][0] = c.g[V0][0];
                let t = c.g[S3][0] == 0u64; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x001a5f98; continue; }
                pc = 0x001a5960; continue;
            }
            0x001a5960 => {
                c.cycles += 2;
                let t = true; 
                c.g[S4][0] = 0x460000;
                if t { pc = 0x001a5a68; continue; }
                pc = 0x001a5968; continue;
            }
            0x001a5968 => {
                c.cycles += 2;
                let t = c.g[A0][0] != 0u64; 
                
                if t { pc = 0x001a5978; continue; }
                pc = 0x001a5970; continue;
            }
            0x001a5970 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5978);
                call_ptr(c, 0x00260f80, 0x001a5978)?;
                pc = 0x001a5978; continue;
            }
            0x001a5978 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5980);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5980)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5994; continue; }
                pc = 0x001a5988; continue;
            }
            0x001a5988 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5990);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5990)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5994; continue;
            }
            0x001a5994 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a59a8; continue; }
                pc = 0x001a599c; continue;
            }
            0x001a599c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a59a4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a59a4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a59a8; continue;
            }
            0x001a59a8 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a59b0);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x00263308, 0x001a59b0)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5a64; continue; }
                pc = 0x001a59bc; continue;
            }
            0x001a59bc => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a59cc; continue; }
                pc = 0x001a59c4; continue;
            }
            0x001a59c4 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a59cc);
                call_ptr(c, 0x00260f80, 0x001a59cc)?;
                pc = 0x001a59cc; continue;
            }
            0x001a59cc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a59d4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a59d4)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a59e8; continue; }
                pc = 0x001a59dc; continue;
            }
            0x001a59dc => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a59e4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a59e4)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a59e8; continue;
            }
            0x001a59e8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a59fc; continue; }
                pc = 0x001a59f0; continue;
            }
            0x001a59f0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a59f8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a59f8)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a59fc; continue;
            }
            0x001a59fc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5a04);
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                call_ptr(c, 0x00262468, 0x001a5a04)?;
                let t = (c.g[V0][0] as i64) < 0; 
                c.g[A0][0] = 0x4;
                if t { pc = 0x001a5a64; continue; }
                pc = 0x001a5a0c; continue;
            }
            0x001a5a0c => {
                c.cycles += 13;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1798));
                c.g[A2][0] = 0x20000000;
                c.g[RA][0] = sx(0x001a5a1c);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x001a5a1c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a5a28);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00317fe8, 0x001a5a28)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6e4));
                let t = c.g[A1][0] == c.g[V0][0]; 
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                if t { pc = 0x001a5a5c; continue; }
                pc = 0x001a5a40; continue;
            }
            0x001a5a40 => {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                
                if c.g[A0][0] != 0u64 { c.sw((c.g[A0][0] as u32), c.g[V1][0]); pc = 0x001a5a4c; continue; }
                pc = 0x001a5a4c; continue;
            }
            0x001a5a4c => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
                if t { pc = 0x001a5a64; continue; }
                pc = 0x001a5a5c; continue;
            }
            0x001a5a5c => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5a64);
                c.g[A2][0] = c.g[SP][0];
                call_ptr(c, 0x001db778, 0x001a5a64)?;
                pc = 0x001a5a64; continue;
            }
            0x001a5a64 => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x001a5a68; continue;
            }
            0x001a5a68 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5a7c; continue; }
                pc = 0x001a5a74; continue;
            }
            0x001a5a74 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5a7c);
                call_ptr(c, 0x00260f80, 0x001a5a7c)?;
                pc = 0x001a5a7c; continue;
            }
            0x001a5a7c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5a84);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5a84)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5a98; continue; }
                pc = 0x001a5a8c; continue;
            }
            0x001a5a8c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5a94);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5a94)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5a98; continue;
            }
            0x001a5a98 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5aac; continue; }
                pc = 0x001a5aa0; continue;
            }
            0x001a5aa0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5aa8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5aa8)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5aac; continue;
            }
            0x001a5aac => {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc0));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xbc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                
                if t { if back_edge_unwinds(c, 0x001a5968) { return Ok(()); } pc = 0x001a5968; continue; }
                pc = 0x001a5ac8; continue;
            }
            0x001a5ac8 => {
                c.cycles += 2;
                let t = (c.g[S3][0] as i64) <= 0; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x001a5c4c; continue; }
                pc = 0x001a5ad0; continue;
            }
            0x001a5ad0 => {
                c.cycles += 2;
                c.g[S4][0] = 0x460000;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5ad8; continue;
            }
            0x001a5ad8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5ae8; continue; }
                pc = 0x001a5ae0; continue;
            }
            0x001a5ae0 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5ae8);
                call_ptr(c, 0x00260f80, 0x001a5ae8)?;
                pc = 0x001a5ae8; continue;
            }
            0x001a5ae8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5af0);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5af0)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5b04; continue; }
                pc = 0x001a5af8; continue;
            }
            0x001a5af8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5b00);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5b00)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5b04; continue;
            }
            0x001a5b04 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5b18; continue; }
                pc = 0x001a5b0c; continue;
            }
            0x001a5b0c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5b14);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5b14)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5b18; continue;
            }
            0x001a5b18 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a5b20);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x002625c8, 0x001a5b20)?;
                c.g[S0][0] = c.g[V0][0];
                
                if c.g[S0][0] == 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5c40; continue; }
                pc = 0x001a5b2c; continue;
            }
            0x001a5b2c => {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xa7));
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5c3c; continue; }
                pc = 0x001a5b38; continue;
            }
            0x001a5b38 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5b48; continue; }
                pc = 0x001a5b40; continue;
            }
            0x001a5b40 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5b48);
                call_ptr(c, 0x00260f80, 0x001a5b48)?;
                pc = 0x001a5b48; continue;
            }
            0x001a5b48 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5b50);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5b50)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5b64; continue; }
                pc = 0x001a5b58; continue;
            }
            0x001a5b58 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5b60);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5b60)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5b64; continue;
            }
            0x001a5b64 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5b78; continue; }
                pc = 0x001a5b6c; continue;
            }
            0x001a5b6c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5b74);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5b74)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5b78; continue;
            }
            0x001a5b78 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5b80);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00263128, 0x001a5b80)?;
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5c40; continue; }
                pc = 0x001a5b88; continue;
            }
            0x001a5b88 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5b9c; continue; }
                pc = 0x001a5b94; continue;
            }
            0x001a5b94 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5b9c);
                call_ptr(c, 0x00260f80, 0x001a5b9c)?;
                pc = 0x001a5b9c; continue;
            }
            0x001a5b9c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5ba4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5ba4)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5bb8; continue; }
                pc = 0x001a5bac; continue;
            }
            0x001a5bac => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5bb4);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5bb4)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5bb8; continue;
            }
            0x001a5bb8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5bcc; continue; }
                pc = 0x001a5bc0; continue;
            }
            0x001a5bc0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5bc8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5bc8)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5bcc; continue;
            }
            0x001a5bcc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5bd4);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x00262400, 0x001a5bd4)?;
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5c40; continue; }
                pc = 0x001a5bdc; continue;
            }
            0x001a5bdc => {
                c.cycles += 15;
                c.g[A2][0] = 0x20000000;
                c.g[A0][0] = 0x4;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1798));
                c.g[RA][0] = sx(0x001a5bf0);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x001a5bf0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a5bfc);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003182d8, 0x001a5bfc)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6e4));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                let t = c.g[A1][0] == c.g[V0][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                if t { pc = 0x001a5c34; continue; }
                pc = 0x001a5c18; continue;
            }
            0x001a5c18 => {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                
                if c.g[A0][0] != 0u64 { c.sw((c.g[A0][0] as u32), c.g[V1][0]); pc = 0x001a5c24; continue; }
                pc = 0x001a5c24; continue;
            }
            0x001a5c24 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
                if t { pc = 0x001a5c3c; continue; }
                pc = 0x001a5c34; continue;
            }
            0x001a5c34 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5c3c);
                call_ptr(c, 0x001db778, 0x001a5c3c)?;
                pc = 0x001a5c3c; continue;
            }
            0x001a5c3c => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x001a5c40; continue;
            }
            0x001a5c40 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { if back_edge_unwinds(c, 0x001a5ad8) { return Ok(()); } pc = 0x001a5ad8; continue; }
                pc = 0x001a5c4c; continue;
            }
            0x001a5c4c => {
                c.cycles += 2;
                let t = (c.g[S3][0] as i64) <= 0; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x001a5dc4; continue; }
                pc = 0x001a5c54; continue;
            }
            0x001a5c54 => {
                c.cycles += 3;
                c.g[S4][0] = 0x460000;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5c60; continue;
            }
            0x001a5c60 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5c70; continue; }
                pc = 0x001a5c68; continue;
            }
            0x001a5c68 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5c70);
                call_ptr(c, 0x00260f80, 0x001a5c70)?;
                pc = 0x001a5c70; continue;
            }
            0x001a5c70 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5c78);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5c78)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5c8c; continue; }
                pc = 0x001a5c80; continue;
            }
            0x001a5c80 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5c88);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5c88)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5c8c; continue;
            }
            0x001a5c8c => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5ca0; continue; }
                pc = 0x001a5c94; continue;
            }
            0x001a5c94 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5c9c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5c9c)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5ca0; continue;
            }
            0x001a5ca0 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a5ca8);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x002625c8, 0x001a5ca8)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5db4; continue; }
                pc = 0x001a5cb4; continue;
            }
            0x001a5cb4 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5cc4; continue; }
                pc = 0x001a5cbc; continue;
            }
            0x001a5cbc => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5cc4);
                call_ptr(c, 0x00260f80, 0x001a5cc4)?;
                pc = 0x001a5cc4; continue;
            }
            0x001a5cc4 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5ccc);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5ccc)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5ce0; continue; }
                pc = 0x001a5cd4; continue;
            }
            0x001a5cd4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5cdc);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5cdc)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5ce0; continue;
            }
            0x001a5ce0 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5cf4; continue; }
                pc = 0x001a5ce8; continue;
            }
            0x001a5ce8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5cf0);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5cf0)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5cf4; continue;
            }
            0x001a5cf4 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5cfc);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x00262400, 0x001a5cfc)?;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5db4; continue; }
                pc = 0x001a5d04; continue;
            }
            0x001a5d04 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5d14; continue; }
                pc = 0x001a5d0c; continue;
            }
            0x001a5d0c => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5d14);
                call_ptr(c, 0x00260f80, 0x001a5d14)?;
                pc = 0x001a5d14; continue;
            }
            0x001a5d14 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5d1c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5d1c)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5d30; continue; }
                pc = 0x001a5d24; continue;
            }
            0x001a5d24 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5d2c);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5d2c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5d30; continue;
            }
            0x001a5d30 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5d44; continue; }
                pc = 0x001a5d38; continue;
            }
            0x001a5d38 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5d40);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5d40)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5d44; continue;
            }
            0x001a5d44 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5d4c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00263128, 0x001a5d4c)?;
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5db8; continue; }
                pc = 0x001a5d54; continue;
            }
            0x001a5d54 => {
                c.cycles += 15;
                c.g[A2][0] = 0x20000000;
                c.g[A0][0] = 0x4;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1798));
                c.g[RA][0] = sx(0x001a5d68);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x001a5d68)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a5d74);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003182d8, 0x001a5d74)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6e4));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                let t = c.g[A1][0] == c.g[V0][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                if t { pc = 0x001a5dac; continue; }
                pc = 0x001a5d90; continue;
            }
            0x001a5d90 => {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                
                if c.g[A0][0] != 0u64 { c.sw((c.g[A0][0] as u32), c.g[V1][0]); pc = 0x001a5d9c; continue; }
                pc = 0x001a5d9c; continue;
            }
            0x001a5d9c => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
                if t { pc = 0x001a5db4; continue; }
                pc = 0x001a5dac; continue;
            }
            0x001a5dac => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5db4);
                call_ptr(c, 0x001db778, 0x001a5db4)?;
                pc = 0x001a5db4; continue;
            }
            0x001a5db4 => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x001a5db8; continue;
            }
            0x001a5db8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { if back_edge_unwinds(c, 0x001a5c60) { return Ok(()); } pc = 0x001a5c60; continue; }
                pc = 0x001a5dc4; continue;
            }
            0x001a5dc4 => {
                c.cycles += 2;
                let t = (c.g[S3][0] as i64) <= 0; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x001a5f50; continue; }
                pc = 0x001a5dcc; continue;
            }
            0x001a5dcc => {
                c.cycles += 3;
                c.g[S4][0] = 0x460000;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5dd8; continue;
            }
            0x001a5dd8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5de8; continue; }
                pc = 0x001a5de0; continue;
            }
            0x001a5de0 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5de8);
                call_ptr(c, 0x00260f80, 0x001a5de8)?;
                pc = 0x001a5de8; continue;
            }
            0x001a5de8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5df0);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5df0)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5e04; continue; }
                pc = 0x001a5df8; continue;
            }
            0x001a5df8 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5e00);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5e00)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5e04; continue;
            }
            0x001a5e04 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5e18; continue; }
                pc = 0x001a5e0c; continue;
            }
            0x001a5e0c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5e14);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5e14)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5e18; continue;
            }
            0x001a5e18 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001a5e20);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x002625c8, 0x001a5e20)?;
                c.g[S0][0] = c.g[V0][0];
                
                if c.g[S0][0] == 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5f44; continue; }
                pc = 0x001a5e2c; continue;
            }
            0x001a5e2c => {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0xa7));
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5f44; continue; }
                pc = 0x001a5e38; continue;
            }
            0x001a5e38 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5e4c; continue; }
                pc = 0x001a5e44; continue;
            }
            0x001a5e44 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5e4c);
                call_ptr(c, 0x00260f80, 0x001a5e4c)?;
                pc = 0x001a5e4c; continue;
            }
            0x001a5e4c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5e54);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5e54)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5e68; continue; }
                pc = 0x001a5e5c; continue;
            }
            0x001a5e5c => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5e64);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5e64)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5e68; continue;
            }
            0x001a5e68 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5e7c; continue; }
                pc = 0x001a5e70; continue;
            }
            0x001a5e70 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5e78);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5e78)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5e7c; continue;
            }
            0x001a5e7c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5e84);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x00263128, 0x001a5e84)?;
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5f44; continue; }
                pc = 0x001a5e8c; continue;
            }
            0x001a5e8c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x001a5ea0; continue; }
                pc = 0x001a5e98; continue;
            }
            0x001a5e98 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5ea0);
                call_ptr(c, 0x00260f80, 0x001a5ea0)?;
                pc = 0x001a5ea0; continue;
            }
            0x001a5ea0 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5ea8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x00261460, 0x001a5ea8)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5ebc; continue; }
                pc = 0x001a5eb0; continue;
            }
            0x001a5eb0 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5eb8);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                call_ptr(c, 0x002613c8, 0x001a5eb8)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5ebc; continue;
            }
            0x001a5ebc => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { pc = 0x001a5ed0; continue; }
                pc = 0x001a5ec4; continue;
            }
            0x001a5ec4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x001a5ecc);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                call_ptr(c, 0x0025b800, 0x001a5ecc)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                pc = 0x001a5ed0; continue;
            }
            0x001a5ed0 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001a5ed8);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x00262400, 0x001a5ed8)?;
                
                if c.g[V0][0] != 0u64 { c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1)); pc = 0x001a5f44; continue; }
                pc = 0x001a5ee0; continue;
            }
            0x001a5ee0 => {
                c.cycles += 15;
                c.g[A2][0] = 0x20000000;
                c.g[A0][0] = 0x4;
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1798));
                c.g[RA][0] = sx(0x001a5ef4);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317d70, 0x001a5ef4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001a5f00);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003182d8, 0x001a5f00)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6e4));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc));
                let t = c.g[A1][0] == c.g[V0][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                if t { pc = 0x001a5f38; continue; }
                pc = 0x001a5f1c; continue;
            }
            0x001a5f1c => {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                
                if c.g[A0][0] != 0u64 { c.sw((c.g[A0][0] as u32), c.g[V1][0]); pc = 0x001a5f28; continue; }
                pc = 0x001a5f28; continue;
            }
            0x001a5f28 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
                if t { pc = 0x001a5f40; continue; }
                pc = 0x001a5f38; continue;
            }
            0x001a5f38 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5f40);
                call_ptr(c, 0x001db778, 0x001a5f40)?;
                pc = 0x001a5f40; continue;
            }
            0x001a5f40 => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x001a5f44; continue;
            }
            0x001a5f44 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                if t { if back_edge_unwinds(c, 0x001a5dd8) { return Ok(()); } pc = 0x001a5dd8; continue; }
                pc = 0x001a5f50; continue;
            }
            0x001a5f50 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001a5f98; continue; }
                pc = 0x001a5f5c; continue;
            }
            0x001a5f5c => {
                c.cycles += 13;
                c.g[RA][0] = sx(0x001a5f64);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1590));
                call_ptr(c, 0x00317670, 0x001a5f64)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001a5f70);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001a5f70)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6e8));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6ec));
                c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x96));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                let t = c.g[A0][0] == c.g[V0][0]; 
                c.g[A0][0] = c.g[A1][0];
                if t { pc = 0x001a5f98; continue; }
                pc = 0x001a5f90; continue;
            }
            0x001a5f90 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x001a5f98);
                c.g[A1][0] = c.g[V0][0] & 0xff;
                call_ptr(c, 0x0039b760, 0x001a5f98)?;
                pc = 0x001a5f98; continue;
            }
            0x001a5f98 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                pc = 0x001a5f9c; continue;
            }
            0x001a5f9c => {
                c.cycles += 7;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001a5898, 0x001a5fb8); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (in the machine's RAM): with it
    /// unset every fake runs the body it replaced.
    pub(crate) const OC1125_CTRL: u32 = DATA + 0xf_a000;
    pub(crate) const OC1125_MAGIC: u32 = 0x1125_c0de;
    /// Mailbox of fake `k`: call count, 8 results, 8 side words.
    pub(crate) const OC1125_MB: u32 = OC1125_CTRL + 0x100;
    pub(crate) const OC1125_MB_SIZE: u32 = 0x80;

    /// The messenger (`self`), its name vector storage, the buddy session,
    /// the scroll list, and the name cells and strings the fakes hand out.
    pub(crate) const OC1125_SELF: u32 = DATA + 0x2000;
    pub(crate) const OC1125_BUF: u32 = DATA + 0x3000;
    pub(crate) const OC1125_SESSION: u32 = DATA + 0x3800;
    pub(crate) const OC1125_LIST: u32 = DATA + 0x3c00;
    pub(crate) const OC1125_CELLS: u32 = DATA + 0x4000;
    pub(crate) const OC1125_STR_A: u32 = DATA + 0x4100;
    pub(crate) const OC1125_STR_B: u32 = DATA + 0x5000;
    pub(crate) const OC1125_OBJ: u32 = DATA + 0x6000;
    pub(crate) const OC1125_STR: u32 = DATA + 0x7000;

    const K_INIT: u32 = 0;
    const K_CHECK: u32 = 1;
    const K_COUNT: u32 = 4;
    const K_NAME_CELL: u32 = 5;
    const K_ACCEPT: u32 = 6;
    const K_NAME: u32 = 7;
    const K_FLAG_A: u32 = 8;
    const K_FLAG_B: u32 = 9;
    const K_ALLOC: u32 = 10;
    const K_STR1: u32 = 11;
    const K_STR2: u32 = 12;
    const K_HASH: u32 = 14;
    const K_LIST: u32 = 15;

    /// A callee fake: logs the call (every register, cycles, target), then
    /// answers with the mailbox's next result (sign-extended, so -1 works);
    /// the init fake also publishes the session singleton at `gp+0x238` when
    /// its side word is set.
    pub(crate) fn oc1125_fake(c: &mut Cpu, addr: u32, k: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let mb = OC1125_MB + OC1125_MB_SIZE * k;
        let n = c.bus.read32(mb);
        c.bus.write32(mb, n + 1);
        let i = 4 * (n % 8);
        let result = c.bus.read32(mb + 4 + i);
        c.g[V0][0] = sx(result);
        if k == K_INIT && c.bus.read32(mb + 0x24 + i) != 0 {
            c.bus.write32(GAME_GP + 0x238, OC1125_SESSION);
        }
    }

    macro_rules! oc1125_fakes {
        ($(($fake:ident, $addr:expr, $shard:ident, $slot:ident, $gen:ident, $k:expr)),* $(,)?) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1125_CTRL) != OC1125_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), ee::$shard::$gen);
                    }
                    oc1125_fake(c, $addr, $k)
                }
            )*

            /// Put every fake in its override slot (each once per process).
            pub(crate) fn oc1125_install_fakes() {
                $( fakes::hook(addr_of_mut!(ee::$shard::$slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc1125_fakes!(
        (oc1125_fake_init, 0x00260f80, s06, OVERRIDE_00260F80, f_00260f80__generated, 0),
        (oc1125_fake_check, 0x00261460, s06, OVERRIDE_00261460, f_00261460__generated, 1),
        (oc1125_fake_sess2, 0x002613c8, s06, OVERRIDE_002613C8, f_002613c8__generated, 2),
        (oc1125_fake_sess3, 0x0025b800, s06, OVERRIDE_0025B800, f_0025b800__generated, 3),
        (oc1125_fake_count, 0x00262598, s07, OVERRIDE_00262598, f_00262598__generated, 4),
        (oc1125_fake_cell, 0x00263308, s07, OVERRIDE_00263308, f_00263308__generated, 5),
        (oc1125_fake_accept, 0x00262468, s07, OVERRIDE_00262468, f_00262468__generated, 6),
        (oc1125_fake_name, 0x002625c8, s07, OVERRIDE_002625C8, f_002625c8__generated, 7),
        (oc1125_fake_flag_a, 0x00263128, s07, OVERRIDE_00263128, f_00263128__generated, 8),
        (oc1125_fake_flag_b, 0x00262400, s07, OVERRIDE_00262400, f_00262400__generated, 9),
        (oc1125_fake_alloc, 0x00317d70, s10, OVERRIDE_00317D70, f_00317d70__generated, 10),
        (oc1125_fake_str1, 0x00317fe8, s10, OVERRIDE_00317FE8, f_00317fe8__generated, 11),
        (oc1125_fake_str2, 0x003182d8, s10, OVERRIDE_003182D8, f_003182d8__generated, 12),
        (oc1125_fake_grow, 0x001db778, s04, OVERRIDE_001DB778, f_001db778__generated, 13),
        (oc1125_fake_hash, 0x00317670, s10, OVERRIDE_00317670, f_00317670__generated, 14),
        (oc1125_fake_find, 0x0039d860, s13, OVERRIDE_0039D860, f_0039d860__generated, 15),
        (oc1125_fake_scroll, 0x0039b760, s13, OVERRIDE_0039B760, f_0039b760__generated, 16),
        (oc1125_fake_clear, 0x001a5450, s03, OVERRIDE_001A5450, f_001a5450__generated, 17),
    );

    /// Set up one seed: a roster that is done or not, the session singleton
    /// present or published by the init call, the session's own name list
    /// (`[0xbc..0xc0]`), 0-4 online names with random flags, a name vector
    /// that is full or has room, and a screen whose scroll list has the
    /// right item count or not.
    pub(crate) fn oc1125_setup(c: &mut Cpu, seed: u32) {
        oc1125_install_fakes();
        let mut r = machine(c, seed, 0x1a5898, 40);
        c.bus.write32(OC1125_CTRL, OC1125_MAGIC);
        for k in 0..18 {
            for w in 0..0x20 {
                c.bus.write32(OC1125_MB + OC1125_MB_SIZE * k + 4 * w, 0);
            }
        }
        fill(c, &mut r, OC1125_SELF, 0x1c0);
        fill(c, &mut r, OC1125_SESSION, 0x100);
        fill(c, &mut r, OC1125_LIST, 0x80);
        fill(c, &mut r, OC1125_STR_A, 0x200);
        fill(c, &mut r, OC1125_STR_B, 0x200);
        let roster = if r.next_u32() % 6 == 0 { 0 } else { 1 + r.next_u32() % 3 };
        c.bus.write32(OC1125_SESSION + 0xe8, roster);
        let present = r.next_u32() % 4 != 0;
        c.bus
            .write32(GAME_GP + 0x238, if present { OC1125_SESSION } else { 0 });
        let publish = r.next_u32() % 4 != 0;
        let buddies = r.next_u32() % 4;
        c.bus.write32(OC1125_SESSION + 0xbc, 0x1000);
        c.bus.write32(OC1125_SESSION + 0xc0, 0x1000 + 4 * buddies);
        let mbw = |c: &mut Cpu, k: u32, i: u32, v: u32| {
            c.bus.write32(OC1125_MB + OC1125_MB_SIZE * k + 4 + 4 * i, v)
        };
        let count = if r.next_u32() % 7 == 0 { 0 } else { 1 + r.next_u32() % 4 };
        for i in 0..8 {
            c.bus
                .write32(OC1125_MB + OC1125_MB_SIZE * K_INIT + 0x24 + 4 * i, publish as u32);
            mbw(c, K_CHECK, i, r.next_u32() % 2);
            mbw(c, K_COUNT, i, count);
            let cell = if r.next_u32() % 5 == 0 { 0 } else { OC1125_CELLS + 8 * i };
            mbw(c, K_NAME_CELL, i, cell);
            c.bus.write32(OC1125_CELLS + 8 * i, OC1125_STR_A + 0x40 * i);
            let a = [0, 0, 1, 0xffff_ffff][(r.next_u32() % 4) as usize];
            mbw(c, K_ACCEPT, i, a);
            let name = if r.next_u32() % 6 == 0 { 0 } else { OC1125_STR_B + 0x100 * i };
            mbw(c, K_NAME, i, name);
            c.bus
                .write8(OC1125_STR_B + 0x100 * i + 167, (r.next_u32() % 2) as u8);
            mbw(c, K_FLAG_A, i, (r.next_u32() % 3 == 0) as u32);
            mbw(c, K_FLAG_B, i, (r.next_u32() % 3 == 0) as u32);
            mbw(c, K_ALLOC, i, OC1125_OBJ + 0x10 * i);
            mbw(c, K_STR1, i, OC1125_STR + 0x10 * i);
            mbw(c, K_STR2, i, OC1125_STR + 0x10 * i + 8);
            mbw(c, K_HASH, i, r.next_u32());
            mbw(c, K_LIST, i, OC1125_LIST);
        }
        c.bus.write8(OC1125_LIST + 0x96, (r.next_u32() % 13) as u8);
        let screen = if r.next_u32() % 5 == 0 { 0 } else { OC1125_LIST + 0x100 };
        c.bus.write32(OC1125_SELF + 0x40, screen);
        let finish = OC1125_BUF + 4 * (r.next_u32() % 4);
        c.bus.write32(OC1125_SELF + 0x6e8, OC1125_BUF);
        c.bus.write32(OC1125_SELF + 0x6ec, finish);
        c.bus.write32(OC1125_SELF + 0x6f0, finish + 4 * (r.next_u32() % 6));
        c.g[A0][0] = OC1125_SELF as u64;
        c.pc = 0x001a_5898;
    }

    /// Every branch of the list build: the roster not done, the session
    /// singleton missing (init calls), an empty session, rejected, accepted,
    /// blocked, away and plain names, a full vector (grow call) or one with
    /// room, no screen, a scroll list with the right count or not, and
    /// callee unwinds.
    #[test]
    fn c_fe_state_net_messenger_build_buddy_list_matches() {
        check(
            ee::s03::f_001a5898__generated,
            c_fe_state_net_messenger_build_buddy_list,
            800,
            &oc1125_setup,
        );
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..800 {
            let mut c = crate::difftest::fresh();
            oc1125_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s03::f_001a5898__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 20, "only {} paths", seen.len());
    }
}
