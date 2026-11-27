//! Staging module oc1124 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1124`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0019_6f98,
    cfe_state_request_line_on_widget_created,
    "cFEStateRequestLine_OnWidgetCreated",
)];

/// `cFEStateRequestLine_OnWidgetCreated(self, widget)` (0x00196f98): the
/// song-request line screen's widget-created hook. Hashes the widget's name
/// (`widget+0x38`) against the names it knows (current song/artist text,
/// help text, arrows, song list, cost/you-have lines, buttons, the 8+8 slot
/// widgets and the 6 option widgets), stores the widget in the matching
/// `self` field and hides or shows it through its virtual `setVisible`
/// (slot `+0x48`/`+0x4c`); the song-count line formats the locale string
/// with the bit count of `self+0x158`.
///
/// The normal entry, converted from the generated body (a `match pc` body:
/// every register, cycle and call kept). Any other `pc` (a resume after an
/// unwound callee or event) runs the generated body.
pub fn cfe_state_request_line_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x00196f98 {
        return ee::s02::f_00196f98__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x00196f98;
    loop {
        match pc {
            0x00196f98 => {
                c.cycles += 19;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = 0x460000;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x588));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[RA][0] = sx(0x00196fdc);
                c.g[S4][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                call_ptr(c, 0x00317670, 0x00196fdc)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x00196ff0; continue; }
                pc = 0x00196fe4; continue;
            }
            0x00196fe4 => {
                c.cycles += 3;
                c.g[A1][0] = 0x4a0000;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[S2][0]);
                if t { pc = 0x00197008; continue; }
                pc = 0x00196ff0; continue;
            }
            0x00196ff0 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00196ff8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x598));
                call_ptr(c, 0x00317670, 0x00196ff8)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x0019701c; continue; }
                pc = 0x00197000; continue;
            }
            0x00197000 => {
                c.cycles += 2;
                c.g[A1][0] = 0x4a0000;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S2][0]);
                pc = 0x00197008; continue;
            }
            0x00197008 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x00197014);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                call_ptr(c, 0x003a0c50, 0x00197014)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x0019701c; continue;
            }
            0x0019701c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197024);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x488));
                call_ptr(c, 0x00317670, 0x00197024)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x00197034; continue; }
                pc = 0x0019702c; continue;
            }
            0x0019702c => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1e8), c.g[S2][0]);
                if t { pc = 0x001974d0; continue; }
                pc = 0x00197034; continue;
            }
            0x00197034 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0019703c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18b8));
                call_ptr(c, 0x00317670, 0x0019703c)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x0019704c; continue; }
                pc = 0x00197044; continue;
            }
            0x00197044 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1ec), c.g[S2][0]);
                if t { pc = 0x001974d0; continue; }
                pc = 0x0019704c; continue;
            }
            0x0019704c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197054);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5a8));
                call_ptr(c, 0x00317670, 0x00197054)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x00197064; continue; }
                pc = 0x0019705c; continue;
            }
            0x0019705c => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1f0), c.g[S2][0]);
                if t { pc = 0x001974d0; continue; }
                pc = 0x00197064; continue;
            }
            0x00197064 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0019706c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5b8));
                call_ptr(c, 0x00317670, 0x0019706c)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x00197080; continue; }
                pc = 0x00197074; continue;
            }
            0x00197074 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197358; continue; }
                pc = 0x00197080; continue;
            }
            0x00197080 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197088);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c8));
                call_ptr(c, 0x00317670, 0x00197088)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x00197148; continue; }
                pc = 0x00197090; continue;
            }
            0x00197090 => {
                c.cycles += 26;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = sx(0x001970b0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001970b0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.g[V0][0];
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001970c0);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, t, 0x001970c0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001970cc);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x003dbca0, 0x001970cc)?;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5));
                c.g[A1][0] = 0x460000;
                c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x470));
                c.g[A2][0] = 0x100;
                c.g[RA][0] = sx(0x001970e8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x001970e8)?;
                c.g[A1][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x158));
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = 0u64;
                c.g[A0][0] = 0u64;
                pc = 0x001970f8; continue;
            }
            0x001970f8 => {
                c.cycles += 7;
                c.g[V0][0] = c.g[A1][0] >> (c.g[A0][0] & 63);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                let t = c.g[V1][0] != 0u64; 
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                if t { if back_edge_unwinds(c, 0x001970f8) { return Ok(()); } pc = 0x001970f8; continue; }
                pc = 0x00197114; continue;
            }
            0x00197114 => {
                c.cycles += 9;
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x00197120);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c26d0, 0x00197120)?;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1e4), c.g[S2][0]);
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x00197130);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x00197130)?;
                
                if c.g[S0][0] == 0u64 { c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90)); pc = 0x001974d4; continue; }
                pc = 0x00197138; continue;
            }
            0x00197138 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197140);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317e98, 0x00197140)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x00197148; continue;
            }
            0x00197148 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197150);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c0));
                call_ptr(c, 0x00317670, 0x00197150)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x00197164; continue; }
                pc = 0x00197158; continue;
            }
            0x00197158 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197358; continue; }
                pc = 0x00197164; continue;
            }
            0x00197164 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0019716c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18c8));
                call_ptr(c, 0x00317670, 0x0019716c)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x00197180; continue; }
                pc = 0x00197174; continue;
            }
            0x00197174 => {
                c.cycles += 3;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1d4), c.g[S2][0]);
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197354; continue; }
                pc = 0x00197180; continue;
            }
            0x00197180 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197188);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18d0));
                call_ptr(c, 0x00317670, 0x00197188)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001971bc; continue; }
                pc = 0x00197190; continue;
            }
            0x00197190 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                
                if c.g[V0][0] != 0u64 { c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[S2][0]); pc = 0x001974d0; continue; }
                pc = 0x0019719c; continue;
            }
            0x0019719c => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001971b4);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001971b4)?;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1dc), c.g[S2][0]);
                if t { pc = 0x001974d0; continue; }
                pc = 0x001971bc; continue;
            }
            0x001971bc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001971c4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5f0));
                call_ptr(c, 0x00317670, 0x001971c4)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x001971f8; continue; }
                pc = 0x001971cc; continue;
            }
            0x001971cc => {
                c.cycles += 8;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1d8), c.g[S2][0]);
                c.g[A1][0] = 0x4a0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                c.g[RA][0] = sx(0x001971e0);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x003a0c50, 0x001971e0)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                let t = c.g[V0][0] != 0u64; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x001971ec; continue;
            }
            0x001971ec => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197358; continue; }
                pc = 0x001971f8; continue;
            }
            0x001971f8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197200);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x600));
                call_ptr(c, 0x00317670, 0x00197200)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x00197220; continue; }
                pc = 0x00197208; continue;
            }
            0x00197208 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                let t = c.g[V0][0] != 0u64; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x00197214; continue;
            }
            0x00197214 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197358; continue; }
                pc = 0x00197220; continue;
            }
            0x00197220 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197228);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x610));
                call_ptr(c, 0x00317670, 0x00197228)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x460000;
                if t { pc = 0x00197260; continue; }
                pc = 0x00197230; continue;
            }
            0x00197230 => {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x168));
                c.g[RA][0] = sx(0x0019723c);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1e0), c.g[S2][0]);
                call_ptr(c, 0x00198af0, 0x0019723c)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1e0));
                c.g[RA][0] = sx(0x00197248);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a0c50, 0x00197248)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                let t = c.g[V0][0] != 0u64; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x00197254; continue;
            }
            0x00197254 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197358; continue; }
                pc = 0x00197260; continue;
            }
            0x00197260 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197268);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x620));
                call_ptr(c, 0x00317670, 0x00197268)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001972bc; continue; }
                pc = 0x00197270; continue;
            }
            0x00197270 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                let t = c.g[V0][0] == 0u64; 
                c.g[A2][0] = 0u64;
                if t { pc = 0x001972b0; continue; }
                pc = 0x0019727c; continue;
            }
            0x0019727c => {
                c.cycles += 3;
                c.g[A0][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x160));
                c.g[A1][0] = 0u64;
                pc = 0x00197288; continue;
            }
            0x00197288 => {
                c.cycles += 7;
                c.g[V0][0] = c.g[A0][0] >> (c.g[A1][0] & 63);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A1][0] as i64) < 64i64) as u64;
                let t = c.g[V1][0] != 0u64; 
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                if t { if back_edge_unwinds(c, 0x00197288) { return Ok(()); } pc = 0x00197288; continue; }
                pc = 0x001972a4; continue;
            }
            0x001972a4 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A2][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x001972b0; continue;
            }
            0x001972b0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197358; continue; }
                pc = 0x001972bc; continue;
            }
            0x001972bc => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001972c4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
                call_ptr(c, 0x00317670, 0x001972c4)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001972d4; continue; }
                pc = 0x001972cc; continue;
            }
            0x001972cc => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x170), c.g[S2][0]);
                if t { pc = 0x001974d0; continue; }
                pc = 0x001972d4; continue;
            }
            0x001972d4 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001972dc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x630));
                call_ptr(c, 0x00317670, 0x001972dc)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001972f0; continue; }
                pc = 0x001972e4; continue;
            }
            0x001972e4 => {
                c.cycles += 3;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1f4), c.g[S2][0]);
                let t = true; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x00197354; continue; }
                pc = 0x001972f0; continue;
            }
            0x001972f0 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001972f8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x640));
                call_ptr(c, 0x00317670, 0x001972f8)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                
                if t { pc = 0x00197308; continue; }
                pc = 0x00197300; continue;
            }
            0x00197300 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1fc), c.g[S2][0]);
                if t { pc = 0x00197350; continue; }
                pc = 0x00197308; continue;
            }
            0x00197308 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197310);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1850));
                call_ptr(c, 0x00317670, 0x00197310)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[A1][0] = 0u64;
                if t { pc = 0x0019733c; continue; }
                pc = 0x00197318; continue;
            }
            0x00197318 => {
                c.cycles += 9;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x1f8), c.g[S2][0]);
                c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00197334);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00197334)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x0019733c; continue;
            }
            0x0019733c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00197344);
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x184c));
                call_ptr(c, 0x00317670, 0x00197344)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[V1][0] = 0x440000;
                if t { pc = 0x00197398; continue; }
                pc = 0x0019734c; continue;
            }
            0x0019734c => {
                c.cycles += 1;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x200), c.g[S2][0]);
                pc = 0x00197350; continue;
            }
            0x00197350 => {
                c.cycles += 1;
                c.g[A1][0] = 0x1;
                pc = 0x00197354; continue;
            }
            0x00197354 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                pc = 0x00197358; continue;
            }
            0x00197358 => {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00197368);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00197368)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                if t { pc = 0x001974d4; continue; }
                pc = 0x00197370; continue;
            }
            0x00197370 => {
                c.cycles += 10;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[A1][0] = 0u64;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x194), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00197390);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00197390)?;
                let t = true; 
                
                if t { pc = 0x00197404; continue; }
                pc = 0x00197398; continue;
            }
            0x00197398 => {
                c.cycles += 6;
                c.g[V0][0] = 0x440000;
                c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11c0));
                c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0x11a0));
                c.g[S1][0] = 0u64;
                let t = true; 
                c.g[S7][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                if t { pc = 0x001973c4; continue; }
                pc = 0x001973b0; continue;
            }
            0x001973b0 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001973b8);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x001973b8)?;
                let t = c.g[S4][0] == c.g[V0][0]; 
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x00197370) { return Ok(()); } pc = 0x00197370; continue; }
                pc = 0x001973c0; continue;
            }
            0x001973c0 => {
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                pc = 0x001973c4; continue;
            }
            0x001973c4 => {
                c.cycles += 3;
                c.g[S6][0] = ((c.g[S1][0] as i64) < 8i64) as u64;
                let t = c.g[S6][0] == 0u64; 
                c.g[V0][0] = 0x440000;
                if t { pc = 0x00197414; continue; }
                pc = 0x001973d0; continue;
            }
            0x001973d0 => {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32));
                c.g[RA][0] = sx(0x001973dc);
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                call_ptr(c, 0x00317670, 0x001973dc)?;
                let t = c.g[S4][0] != c.g[V0][0]; 
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                if t { if back_edge_unwinds(c, 0x001973b0) { return Ok(()); } pc = 0x001973b0; continue; }
                pc = 0x001973e4; continue;
            }
            0x001973e4 => {
                c.cycles += 8;
                c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.g[A1][0] = 0u64;
                c.sw((c.g[V1][0] as u32), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00197404);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00197404)?;
                pc = 0x00197404; continue;
            }
            0x00197404 => {
                c.cycles += 2;
                let t = c.g[S6][0] == 0u64; 
                c.g[V0][0] = 0x440000;
                if t { pc = 0x00197414; continue; }
                pc = 0x0019740c; continue;
            }
            0x0019740c => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S1][0]);
                if t { pc = 0x001974d0; continue; }
                pc = 0x00197414; continue;
            }
            0x00197414 => {
                c.cycles += 5;
                c.g[S1][0] = 0u64;
                c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1200));
                c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1bc));
                c.g[S7][0] = 0x6;
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                pc = 0x00197428; continue;
            }
            0x00197428 => {
                c.cycles += 6;
                c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
                c.g[RA][0] = sx(0x00197438);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x00197438)?;
                
                if c.g[S4][0] != c.g[V0][0] { c.g[S1][0] = c.g[S5][0]; pc = 0x001974c4; continue; }
                pc = 0x00197440; continue;
            }
            0x00197440 => {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[S2][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x16c));
                let t = c.g[V1][0] == 0u64; 
                c.g[T0][0] = sx((c.g[S7][0] as u32).wrapping_sub((c.g[S1][0] as u32)));
                if t { pc = 0x001974a8; continue; }
                pc = 0x00197454; continue;
            }
            0x00197454 => {
                c.cycles += 5;
                c.g[A2][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x160));
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = 0u64;
                pc = 0x00197468; continue;
            }
            0x00197468 => {
                c.cycles += 7;
                c.g[V0][0] = c.g[A2][0] >> (c.g[A0][0] & 63);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A0][0] as i64) < 64i64) as u64;
                let t = c.g[V1][0] != 0u64; 
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                if t { if back_edge_unwinds(c, 0x00197468) { return Ok(()); } pc = 0x00197468; continue; }
                pc = 0x00197484; continue;
            }
            0x00197484 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = 0x1;
                if t { pc = 0x001974a8; continue; }
                pc = 0x00197490; continue;
            }
            0x00197490 => {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001974a0);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001974a0)?;
                let t = true; 
                c.g[S1][0] = c.g[S5][0];
                if t { pc = 0x001974c4; continue; }
                pc = 0x001974a8; continue;
            }
            0x001974a8 => {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001974c0);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001974c0)?;
                c.g[S1][0] = c.g[S5][0];
                pc = 0x001974c4; continue;
            }
            0x001974c4 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
                if t { if back_edge_unwinds(c, 0x00197428) { return Ok(()); } pc = 0x00197428; continue; }
                pc = 0x001974d0; continue;
            }
            0x001974d0 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                pc = 0x001974d4; continue;
            }
            0x001974d4 => {
                c.cycles += 11;
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
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00196f98, 0x00197500); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const OC1124_ENTRY: u32 = 0x0019_6f98;
    /// Gate word (and magic) of this test's callee fakes; `+4`: 1 = the
    /// allocator fake returns null.
    pub(crate) const OC1124_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1124_MAGIC: u32 = 0x1124_c0de;
    const OC1124_THIS: u32 = DATA + 0x1000;
    const OC1124_WIDGET: u32 = DATA + 0x2000;
    const OC1124_WVT: u32 = DATA + 0x3000;
    const OC1124_WORLD: u32 = DATA + 0x4000;
    const OC1124_LOC: u32 = DATA + 0x5000;
    const OC1124_LVT: u32 = DATA + 0x6000;
    /// The hash fake returns its argument, so a widget whose name hash is the
    /// string address matches that entry of the chain.
    const OC1124_HASHES: [u32; 19] = [
        0x0046_0588,
        0x0046_0598,
        0x0046_0488,
        0x004a_18b8,
        0x0046_05a8,
        0x0046_05b8,
        0x0046_05c8,
        0x004a_18c0,
        0x004a_18c8,
        0x004a_18d0,
        0x0046_05f0,
        0x0046_0600,
        0x0046_0610,
        0x0046_0620,
        0x0045_e1b0,
        0x0046_0630,
        0x0046_0640,
        0x004a_1900,
        0x004a_1910,
    ];
    /// Targets: the 19 fixed names, 8 slot-A, 8 slot-B and 6 option names,
    /// and one name that matches nothing.
    const OC1124_NT: u32 = 19 + 8 + 8 + 6 + 1;

    /// The fake callee: logs like `fakes::fake_call`; the hash fake echoes
    /// its argument and the allocator fake can return null.
    fn oc1124_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let null_alloc = c.bus.read32(OC1124_CTRL + 4) == 1;
        fakes::fake_call(c, addr);
        if c.pc == ra {
            if addr == 0x0031_7670 {
                c.g[V0][0] = a0;
            } else if addr == 0x0031_7e30 && null_alloc {
                c.g[V0][0] = 0;
            }
        }
    }

    macro_rules! oc1124_fakes {
        ($($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1124_CTRL) != OC1124_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1124_fake_call(c, $addr);
                }
            )*
            fn oc1124_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1124_fakes! {
        oc1124_fake_317670, OC1124_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670;
        oc1124_fake_3dbca0, OC1124_PREV_3DBCA0, ee::s14::OVERRIDE_003DBCA0, ee::s14::f_003dbca0__generated, 0x003d_bca0;
        oc1124_fake_2c26d0, OC1124_PREV_2C26D0, ee::s08::OVERRIDE_002C26D0, ee::s08::f_002c26d0__generated, 0x002c_26d0;
        oc1124_fake_317e30, OC1124_PREV_317E30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated, 0x0031_7e30;
        oc1124_fake_317e98, OC1124_PREV_317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98;
        oc1124_fake_3a0e90, OC1124_PREV_3A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90;
        oc1124_fake_3a0c50, OC1124_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50;
        oc1124_fake_198af0, OC1124_PREV_198AF0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated, 0x0019_8af0;
    }

    /// Scene for seed `seed`: the widget's name hash is entry `seed % NT` of
    /// the chain; the variant (`seed / NT`) sets the cost-line flag, the bit
    /// counts of the done/selected masks and a null allocation.
    fn oc1124_scene(c: &mut Cpu, seed: u32) {
        let target = seed % OC1124_NT;
        let v = seed / OC1124_NT;
        c.bus.write32(OC1124_CTRL, OC1124_MAGIC);
        c.bus.write32(OC1124_CTRL + 4, (v % 3 == 2) as u32);
        // Name tables of the slot-A, slot-B and option loops.
        for i in 0..8u32 {
            c.bus.write32(0x0044_11a0 + 4 * i, 0x0050_0000 + 0x10 * i);
            c.bus.write32(0x0044_11c0 + 4 * i, 0x0050_0100 + 0x10 * i);
        }
        for i in 0..6u32 {
            c.bus.write32(0x0044_1200 + 4 * i, 0x0050_0200 + 0x10 * i);
        }
        c.bus.write32(GAME_GP - 0x1850, 0x004a_1900);
        c.bus.write32(GAME_GP - 0x184c, 0x004a_1910);
        let hash = match target {
            0..=18 => OC1124_HASHES[target as usize],
            19..=26 => 0x0050_0000 + 0x10 * (target - 19),
            27..=34 => 0x0050_0100 + 0x10 * (target - 27),
            35..=40 => 0x0050_0200 + 0x10 * (target - 35),
            _ => 0x1234_5670,
        };
        c.bus.write32(OC1124_WIDGET + 0x38, hash);
        c.bus.write32(OC1124_WIDGET + 8, OC1124_WVT);
        c.bus.write16(OC1124_WVT + 0x48, (4 * (v % 4)) as u16);
        c.bus.write32(OC1124_WVT + 0x4c, FAKE);
        // The locale object behind the world's `+0x8c` (virtual slot +0x20/+0x24).
        c.bus.write32(GAME_GP - 0x848, OC1124_WORLD);
        c.bus.write32(OC1124_WORLD + 0x8c, OC1124_LOC);
        c.bus.write32(OC1124_LOC + 4, OC1124_LVT);
        c.bus.write16(OC1124_LVT + 0x20, (4 * (v % 3)) as u16);
        c.bus.write32(OC1124_LVT + 0x24, FAKE + 0x10);
        // The line: done mask, selection mask, cost flag, price pointer.
        let k = (v * 5) % 9;
        c.bus.write64(
            OC1124_THIS + 0x158,
            0x9e37_79b9_7f4a_7c15u64.rotate_left(seed) | 1,
        );
        c.bus.write64(OC1124_THIS + 0x160, (1u64 << k) - 1);
        c.bus.write32(OC1124_THIS + 0x16c, (v >> 1) & 1);
        c.bus.write32(OC1124_THIS + 0x168, DATA + 0x7000);
        c.g[A0][0] = sx(OC1124_THIS);
        c.g[A1][0] = sx(OC1124_WIDGET);
        c.pc = OC1124_ENTRY;
    }

    /// `cFEStateRequestLine_OnWidgetCreated`: every name of the hash chain
    /// (and a miss) with 8 variants of the flag/bit-count masks; some seeds
    /// unwind in a callee (the resume runs the generated body); every call
    /// site is reached by some seed. Then events due at the bit-count loops.
    #[test]
    fn cfe_state_request_line_on_widget_created_matches() {
        oc1124_install();
        fakes::check_reaching(
            ee::s02::f_00196f98__generated,
            cfe_state_request_line_on_widget_created,
            OC1124_NT * 8,
            &[
                0x0019_7014,
                0x0019_70c0,
                0x0019_70e8,
                0x0019_7130,
                0x0019_7140,
                0x0019_71e0,
                0x0019_723c,
                0x0019_7248,
                0x0019_7334,
                0x0019_7368,
                0x0019_7390,
                0x0019_7404,
                0x0019_74a0,
                0x0019_74c0,
            ],
            &|c, seed| {
                machine(c, seed, 0x196f98, 40);
                oc1124_scene(c, seed);
            },
        );
        driven!(oc1124_gen_driven, ee::s02::f_00196f98__generated);
        driven!(oc1124_hand_driven, cfe_state_request_line_on_widget_created);
        check_events(oc1124_gen_driven, oc1124_hand_driven, 96, &|c, seed| {
            // Names that reach a bit-count loop (song list, free songs,
            // options) with the cost flag set where the loop needs it.
            let name = [6, 13, 35, 36, 37, 38][(seed % 6) as usize];
            let variant = [2, 3, 6, 7][((seed / 6) % 4) as usize];
            let mut r = machine(c, seed, 0x196f99, 0);
            oc1124_scene(c, name + OC1124_NT * variant);
            c.g[RA][0] = crate::difftest::RA as u64;
            schedule(c, &mut r, seed, OC1124_ENTRY)
        });
    }
}
