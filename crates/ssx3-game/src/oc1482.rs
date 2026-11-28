//! Staging module oc1482 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1482`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0031_6f00, app_run_module_loop, "App_RunModuleLoop"),
    (0x0036_c790, ps2gfx_render_bloom, "PS2GraphicsMan_RenderBloom"),
    (0x0037_7a10, ps2gfx_end_frame, "PS2GraphicsMan_EndFrame"),
    (0x0038_1310, ps2gfx_draw_trail, "PS2GraphicsMan_v85"),
    (0x0038_83b8, draw_board_wake, "PS2GraphicsMan_v90"),
];

/// `App_RunModuleLoop` (0x00316f00): the application main loop, a resumable dispatcher: the whole generated
/// `match pc` is kept (every register, cycle and call), calls go through `call_ptr` / `Callee`, so the
/// function also resumes natively at any block head after an unwound callee or event.
pub fn app_run_module_loop(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x00316f00 => {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S1][0] = 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S2][0] = 0x1;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00316f34);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00316f34)?;
                pc = 0x00316f34; continue;
            }
            0x00316f34 => {
                c.cycles += 1;
                pc = 0x00316f38; continue;
            }
            0x00316f38 => {
                c.cycles += 1;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                pc = 0x00316f3c; continue;
            }
            0x00316f3c => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00316f50);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00316f50)?;
                pc = 0x00316f50; continue;
            }
            0x00316f50 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00316f58);
                c.g[A0][0] = 0u64;
                call_ptr(c, 0x003e5928, 0x00316f58)?;
                pc = 0x00316f58; continue;
            }
            0x00316f58 => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00316f70);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00316f70)?;
                pc = 0x00316f70; continue;
            }
            0x00316f70 => {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                let t = c.g[V1][0] == c.g[S2][0]; 
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                if t { pc = 0x00316fac; continue; }
                pc = 0x00316f7c; continue;
            }
            0x00316f7c => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00316f94; continue; }
                pc = 0x00316f84; continue;
            }
            0x00316f84 => {
                c.cycles += 2;
                
                if c.g[V1][0] == 0u64 { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4)); pc = 0x00317140; continue; }
                pc = 0x00316f8c; continue;
            }
            0x00316f8c => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x003171c8; continue; }
                pc = 0x00316f94; continue;
            }
            0x00316f94 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x3;
                if t { pc = 0x00316ff4; continue; }
                pc = 0x00316f9c; continue;
            }
            0x00316f9c => {
                c.cycles += 2;
                
                if c.g[V1][0] == c.g[V0][0] { c.g[A1][0] = c.lw((c.g[S0][0] as u32)); pc = 0x003170bc; continue; }
                pc = 0x00316fa4; continue;
            }
            0x00316fa4 => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x003171c8; continue; }
                pc = 0x00316fac; continue;
            }
            0x00316fac => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00316fc4);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00316fc4)?;
                pc = 0x00316fc4; continue;
            }
            0x00316fc4 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00316fe4; continue; }
                pc = 0x00316fcc; continue;
            }
            0x00316fcc => {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.g[RA][0] = sx(0x00316fdc);
                Callee { addr: 0x00317600, f: ee::s10::App_LoadProgressComplete }.call(c, 0x00316fdc)?;
                pc = 0x00316fdc; continue;
            }
            0x00316fdc => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x003171c8; continue; }
                pc = 0x00316fe4; continue;
            }
            0x00316fe4 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00316fec);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                Callee { addr: 0x003175a0, f: ee::s10::App_LoadProgressUpdate }.call(c, 0x00316fec)?;
                pc = 0x00316fec; continue;
            }
            0x00316fec => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x003171c8; continue; }
                pc = 0x00316ff4; continue;
            }
            0x00316ff4 => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0031700c);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0031700c)?;
                pc = 0x0031700c; continue;
            }
            0x0031700c => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 { c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c)); pc = 0x003171c8; continue; }
                pc = 0x00317014; continue;
            }
            0x00317014 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0031701c);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x0031701c)?;
                pc = 0x0031701c; continue;
            }
            0x0031701c => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00317034);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00317034)?;
                pc = 0x00317034; continue;
            }
            0x00317034 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0031703c);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x0031703c)?;
                pc = 0x0031703c; continue;
            }
            0x0031703c => {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                let t = c.g[A2][0] == 0u64; 
                c.g[A1][0] = 0x3;
                if t { pc = 0x0031705c; continue; }
                pc = 0x00317048; continue;
            }
            0x00317048 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0031705c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0031705c)?;
                pc = 0x0031705c; continue;
            }
            0x0031705c => {
                c.cycles += 3;
                c.g[A1][0] = 0x1;
                c.g[RA][0] = sx(0x00317068);
                c.g[A0][0] = 0x1;
                Callee { addr: 0x00319d18, f: ee::s10::MEM_FrameCheckStub_ReturnZero }.call(c, 0x00317068)?;
                pc = 0x00317068; continue;
            }
            0x00317068 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00317070);
                c.g[S1][0] = 0u64;
                Callee { addr: 0x00319d10, f: ee::s10::MEM_FrameBeginStub }.call(c, 0x00317070)?;
                pc = 0x00317070; continue;
            }
            0x00317070 => {
                c.cycles += 8;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[S0][0] as u32), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                let t = c.g[A1][0] as u32;
                c.g[RA][0] = sx(0x00317090);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                call_ptr(c, t, 0x00317090)?;
                pc = 0x00317090; continue;
            }
            0x00317090 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00317098);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x00317098)?;
                pc = 0x00317098; continue;
            }
            0x00317098 => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003170ac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003170ac)?;
                pc = 0x003170ac; continue;
            }
            0x003170ac => {
                c.cycles += 4;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x003171c8; continue; }
                pc = 0x003170bc; continue;
            }
            0x003170bc => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003170d0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003170d0)?;
                pc = 0x003170d0; continue;
            }
            0x003170d0 => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 { c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c)); pc = 0x003171c8; continue; }
                pc = 0x003170d8; continue;
            }
            0x003170d8 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003170e0);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x003170e0)?;
                pc = 0x003170e0; continue;
            }
            0x003170e0 => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003170f8);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003170f8)?;
                pc = 0x003170f8; continue;
            }
            0x003170f8 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00317100);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x00317100)?;
                pc = 0x00317100; continue;
            }
            0x00317100 => {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                let t = c.g[A2][0] == 0u64; 
                c.g[A1][0] = 0x3;
                if t { pc = 0x00317120; continue; }
                pc = 0x0031710c; continue;
            }
            0x0031710c => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00317120);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00317120)?;
                pc = 0x00317120; continue;
            }
            0x00317120 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S0][0] as u32), 0u64);
                if t { pc = 0x003172f0; continue; }
                pc = 0x00317128; continue;
            }
            0x00317128 => {
                c.cycles += 4;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00317138);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00317138)?;
                pc = 0x00317138; continue;
            }
            0x00317138 => {
                c.cycles += 2;
                let t = true; 
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if t { pc = 0x003171e0; continue; }
                pc = 0x00317140; continue;
            }
            0x00317140 => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 { c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c)); pc = 0x003171c8; continue; }
                pc = 0x00317148; continue;
            }
            0x00317148 => {
                c.cycles += 4;
                c.g[AT][0] = 0x41200000;
                c.mtc1(1, 12);
                c.g[RA][0] = sx(0x00317158);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                Callee { addr: 0x00317530, f: ee::s10::App_LoadProgressInit }.call(c, 0x00317158)?;
                pc = 0x00317158; continue;
            }
            0x00317158 => {
                c.cycles += 7;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00317174);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00317174)?;
                pc = 0x00317174; continue;
            }
            0x00317174 => {
                c.cycles += 3;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[S2][0]);
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                if t { pc = 0x003171c8; continue; }
                pc = 0x00317180; continue;
            }
            0x00317180 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00317188);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x00317188)?;
                pc = 0x00317188; continue;
            }
            0x00317188 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c)); if back_edge_unwinds(c, 0x00317128) { return Ok(()); } pc = 0x00317128; continue; }
                pc = 0x00317198; continue;
            }
            0x00317198 => {
                c.cycles += 9;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003171bc);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003171bc)?;
                pc = 0x003171bc; continue;
            }
            0x003171bc => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003171c4);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x003171c4)?;
                pc = 0x003171c4; continue;
            }
            0x003171c4 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                pc = 0x003171c8; continue;
            }
            0x003171c8 => {
                c.cycles += 4;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x003171d8);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003171d8)?;
                pc = 0x003171d8; continue;
            }
            0x003171d8 => {
                c.cycles += 2;
                
                if c.g[V0][0] != 0u64 { c.g[A0][0] = c.g[S0][0]; if back_edge_unwinds(c, 0x00317180) { return Ok(()); } pc = 0x00317180; continue; }
                pc = 0x003171e0; continue;
            }
            0x003171e0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                let t = c.g[V0][0] != c.g[S2][0]; 
                
                if t { pc = 0x0031721c; continue; }
                pc = 0x003171ec; continue;
            }
            0x003171ec => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34)); pc = 0x0031721c; continue; }
                pc = 0x003171f8; continue;
            }
            0x003171f8 => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00317210);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00317210)?;
                pc = 0x00317210; continue;
            }
            0x00317210 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                
                if t { pc = 0x0031724c; continue; }
                pc = 0x00317218; continue;
            }
            0x00317218 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
                pc = 0x0031721c; continue;
            }
            0x0031721c => {
                c.cycles += 2;
                
                if c.g[V0][0] != 0u64 { c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8)); pc = 0x003172cc; continue; }
                pc = 0x00317224; continue;
            }
            0x00317224 => {
                c.cycles += 2;
                
                if c.g[S1][0] == 0u64 { c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8)); pc = 0x003172cc; continue; }
                pc = 0x0031722c; continue;
            }
            0x0031722c => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00317244);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00317244)?;
                pc = 0x00317244; continue;
            }
            0x00317244 => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 { c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8)); pc = 0x003172cc; continue; }
                pc = 0x0031724c; continue;
            }
            0x0031724c => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00317254);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x00317254)?;
                pc = 0x00317254; continue;
            }
            0x00317254 => {
                c.cycles += 6;
                c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x2c));
                c.mtc1(0, 0);
                c.c_lt_s(0, 3);
                let t = !c.fcc(); 
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x24));
                if t { pc = 0x003172a8; continue; }
                pc = 0x0031726c; continue;
            }
            0x0031726c => {
                c.cycles += 15;
                c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x30));
                c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.cvt_s_w(5, 5);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.mtc1(17, 4);
                c.cvt_s_w(4, 4);
                c.mul_s(3, 2, 3);
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 1);
                c.mul_s(0, 0, 5);
                c.div_s(0, 0, 4);
                c.add_s(0, 0, 3);
                let t = true; 
                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
                if t { pc = 0x003172c4; continue; }
                pc = 0x003172a8; continue;
            }
            0x003172a8 => {
                c.cycles += 7;
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x10));
                c.cvt_s_w(0, 0);
                c.mtc1(17, 2);
                c.cvt_s_w(2, 2);
                c.mul_s(1, 1, 0);
                c.div_s(1, 1, 2);
                c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x2c));
                pc = 0x003172c4; continue;
            }
            0x003172c4 => {
                c.cycles += 2;
                let t = true; 
                c.g[S1][0] = 0u64;
                if t { if back_edge_unwinds(c, 0x00316f38) { return Ok(()); } pc = 0x00316f38; continue; }
                pc = 0x003172cc; continue;
            }
            0x003172cc => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x003172e0);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x003172e0)?;
                pc = 0x003172e0; continue;
            }
            0x003172e0 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x003172e8);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x00317328, f: ee::s10::App_InvokeExecCallback }.call(c, 0x003172e8)?;
                pc = 0x003172e8; continue;
            }
            0x003172e8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                if t { if back_edge_unwinds(c, 0x00316f3c) { return Ok(()); } pc = 0x00316f3c; continue; }
                pc = 0x003172f0; continue;
            }
            0x003172f0 => {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x00316f00, 0x00317308); return Ok(()); }
        }
    }
    });
}

/// `PS2GraphicsMan_RenderBloom` (0x0036c790): converted from the generated labelled-block body (every register, cycle, store and call
/// order kept; calls through `Callee` / `call_ptr`, unwinds propagate with `?`).
pub fn ps2gfx_render_bloom(c: &mut Cpu) {
    guest(c, |c| {
'b_36cbc4: {
'b_36cbc0: {
c.cycles += 19;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x12d8));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S0][0] = c.g[A1][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S3][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[S6][0] = c.g[A3][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x118));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x110));
let t = c.g[V0][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
if t {
break 'b_36cbc0;
} else {
'b_36c7f4: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x12dc));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1c;
if t {
break 'b_36c7f4;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x0036c7f0);
Callee { addr: 0x0036c740, f: ee::s12::PS2GraphicsMan_ResetBloomParams }.call(c, 0x0036c7f0)?;
c.g[V0][0] = 0x1c;
break 'b_36c7f4;
}
}
'b_36c89c: {
c.cycles += 32;
c.g[AT][0] = 0x42ff0000;
c.mtc1(1, 3);
c.mult(0, 2, 16, 2);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x12f8));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x12fc));
c.mul_s(0, 0, 3);
c.mul_s(2, 2, 3);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6cd4));
c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x14));
c.cvt_w_s(0, 2);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x18));
c.mul_s(1, 1, 3);
c.mul_s(0, 0, 3);
c.cvt_w_s(2, 1);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4c));
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x4f));
c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x48));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x3f));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x38));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a80));
break 'b_36c89c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a80));
break 'b_36c89c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a80));
break 'b_36c89c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
if c.g[V0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
break 'b_36cbc4;
} else {
c.cycles += 1;
c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a80));
break 'b_36c89c;
}
}
}
}
}
c.cycles += 75;
c.g[V1][0] = sx((c.g[S0][0] as u32) << 3);
c.g[AT][0] = 0x41800000;
c.mtc1(1, 1);
c.g[T1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x1308));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x400));
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[A2][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
c.mul_s(0, 0, 1);
c.g[A1][0] = c.g[T1][0];
c.g[V1][0] = c.g[T1][0];
c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[S0][0] = 0x1;
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x12e0));
c.g[V0][0] = c.g[T1][0];
c.g[S2][0] = c.g[A2][0];
c.g[S1][0] = sx((c.g[S0][0] as u32) << ((c.g[A3][0] as u32) & 31));
c.g[AT][0] = 0x42ff0000;
c.mtc1(1, 20);
c.g[A1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x59b0));
c.g[S4][0] = c.g[S1][0];
c.g[A0][0] = c.lh((c.g[T0][0] as u32).wrapping_add(0x50));
c.mov_s(21, 20);
c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x59b2));
c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
c.g[V1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x59b4));
c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a84));
c.g[FP][0] = c.lhu((c.g[T1][0] as u32).wrapping_add(0x59b6));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x54));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0036c930);
c.sw((c.g[SP][0] as u32).wrapping_add(0x64), c.g[V1][0]);
call_ptr(c, t, 0x0036c930)?;
c.lwc1(12, (c.g[S5][0] as u32).wrapping_add(0xc));
c.g[A2][0] = c.g[V0][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
c.g[T2][0] = c.g[FP][0];
c.mul_s(12, 12, 20);
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x5c));
c.g[S7][0] = sx((c.g[A0][0] as u32) << 5);
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A0][0] = c.g[S3][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S0][0]);
c.g[A1][0] = c.g[S7][0];
c.sw((c.g[SP][0] as u32), c.g[S1][0]);
c.g[T3][0] = c.g[S2][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S1][0]);
c.g[S0][0] = 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
c.g[RA][0] = sx(0x0036c97c);
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S6][0]);
Callee { addr: 0x0036b9d8, f: ee::s12::PS2GraphicsMan_EmitBlurDownsample }.call(c, 0x0036c97c)?;
c.lwc1(1, (c.g[S5][0] as u32));
c.g[A0][0] = c.g[S3][0];
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x12e8));
c.g[A1][0] = c.g[S2][0];
c.mul_s(1, 1, 20);
c.g[A2][0] = c.g[S1][0];
c.mul_s(0, 0, 20);
c.g[A3][0] = c.g[S1][0];
c.cvt_w_s(2, 1);
c.mfc1(8, 2);
c.cvt_w_s(1, 0);
c.mfc1(9, 1);
c.g[RA][0] = sx(0x0036c9b4);
c.g[T2][0] = c.g[S6][0];
Callee { addr: 0x0036c188, f: ee::s12::PS2GraphicsMan_EmitBlendedFill }.call(c, 0x0036c9b4)?;
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), c.g[V1][0]);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
'l_36c9c8: loop {
'b_36ca18: {
c.cycles += 5;
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = c.lw((c.g[V0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S3][0];
if t {
break 'b_36ca18;
} else {
c.cycles += 15;
c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A1][0] = c.g[S2][0];
c.mul_s(0, 0, 21);
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x5c));
c.g[A2][0] = c.g[S4][0];
c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A3][0] = c.g[S1][0];
c.sw((c.g[SP][0] as u32), c.g[FP][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S6][0]);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = sx(0x0036ca18);
c.g[T0][0] = c.g[S7][0];
Callee { addr: 0x0036c398, f: ee::s12::PS2GraphicsMan_EmitBloomComposite }.call(c, 0x0036ca18)?;
break 'b_36ca18;
}
}
c.cycles += 30;
c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x6c));
c.g[V0][0] = c.g[V1][0] & 0x1;
c.g[A1][0] = c.g[S2][0];
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.lwc1(12, (c.g[S5][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A2][0] = c.g[S4][0];
c.g[S2][0] = c.lw((c.g[V0][0] as u32));
c.g[T2][0] = c.g[S1][0];
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
c.mul_s(12, 12, 21);
c.g[S4][0] = sx(((c.g[S4][0] as u32) as i32 >> 1) as u32);
c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 1) as u32);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.g[S0][0] = c.g[V1][0];
c.g[V0][0] = 0x4;
c.sw((c.g[SP][0] as u32), c.g[S4][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[A0][0] = c.g[S3][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S1][0]);
c.g[A3][0] = 0u64;
c.g[T0][0] = 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[S6][0]);
c.g[T1][0] = c.g[A2][0];
c.g[RA][0] = sx(0x0036ca84);
c.g[T3][0] = c.g[S2][0];
Callee { addr: 0x0036b9d8, f: ee::s12::PS2GraphicsMan_EmitBlurDownsample }.call(c, 0x0036ca84)?;
c.g[V0][0] = ((c.g[S0][0] as i64) < 3i64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x68));
if t {
if event_due(c) { deliver_event_here(c, 0x0036c9c8); } continue 'l_36c9c8;
} else {
'b_36cae0: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.g[S2][0];
if t {
break 'b_36cae0;
} else {
c.cycles += 17;
c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x10));
c.g[AT][0] = 0x42ff0000;
c.mtc1(1, 1);
c.g[A2][0] = c.g[S4][0];
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
c.g[A3][0] = c.g[S1][0];
c.mul_s(0, 0, 1);
c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x5c));
c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[T0][0] = c.g[S7][0];
c.sw((c.g[SP][0] as u32), c.g[FP][0]);
c.g[A0][0] = c.g[S3][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = sx(0x0036cae0);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[S6][0]);
Callee { addr: 0x0036c398, f: ee::s12::PS2GraphicsMan_EmitBloomComposite }.call(c, 0x0036cae0)?;
break 'b_36cae0;
}
}
c.cycles += 56;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[A1][0] = 0x30000000;
c.g[S0][0] = c.lw((c.g[S6][0] as u32));
c.g[S1][0] = 0x1000;
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
c.g[S0][0] = c.g[S0][0] | c.g[A1][0];
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0036cb04);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0036cb04)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32) >> 1);
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
c.g[S2][0] = sx((c.g[S2][0] as u32) << 4);
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0036cb24);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0036cb24)?;
c.g[A1][0] = 0xe;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x3;
c.g[A0][0] = 0x10000000;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[A0][0] | 0x8002;
c.sd((c.g[S0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.sd((c.g[S0][0] as u32), c.g[V1][0]);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.sd((c.g[S0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[S1][0] = sx((c.g[S1][0] as u32) >> 1);
c.sd((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
c.g[S1][0] = sx((c.g[S1][0] as u32) << 4);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0036cb6c);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0036cb6c)?;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 6);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x64));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[S1][0] = c.g[S1][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 16;
c.g[S1][0] = c.g[S2][0] | c.g[S1][0];
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = 0x4c;
c.sd((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[A0][0] = 0x18;
c.sd((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
c.g[V0][0] = 0xfff0000;
c.sd((c.g[S0][0] as u32).wrapping_add(0x30), c.g[S1][0]);
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
c.sd((c.g[S0][0] as u32).wrapping_add(0x38), c.g[A0][0]);
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[S0][0] = c.g[S0][0] & c.g[V0][0];
c.sw((c.g[S6][0] as u32), c.g[S0][0]);
c.g[RA][0] = sx(0x0036cbc0);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f4));
Callee { addr: 0x00368138, f: ee::s12::TexMan_EmitPath3Window }.call(c, 0x0036cbc0)?;
break 'b_36cbc0;
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
break 'b_36cbc4;
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
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_EndFrame` (0x00377a10): converted from the generated labelled-block body (every register, cycle, store and call
/// order kept; calls through `Callee` / `call_ptr`, unwinds propagate with `?`).
pub fn ps2gfx_end_frame(c: &mut Cpu) {
    guest(c, |c| {
'b_377a80: {
'b_377a68: {
c.cycles += 18;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S3][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377a50);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00377a50)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_377a68;
} else {
c.cycles += 4;
c.mtc1(2, 20);
c.cvt_s_w(20, 20);
let t = true;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
if t {
break 'b_377a80;
} else {
break 'b_377a68;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 20);
c.cvt_s_w(20, 20);
c.add_s(20, 20, 20);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
break 'b_377a80;
}
'b_377ac0: {
'b_377aa8: {
c.cycles += 6;
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377a90);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00377a90)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_377aa8;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
if t {
break 'b_377ac0;
} else {
break 'b_377aa8;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
break 'b_377ac0;
}
'b_377c60: {
'b_377c48: {
c.cycles += 94;
c.g[A1][0] = c.g[SP][0];
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.g[A0][0] = c.g[S0][0];
c.sw((c.g[SP][0] as u32), 0u64);
c.g[A2][0] = 0x1;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.g[S0][0] = 0x20000;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377ae8);
c.g[S0][0] = c.g[S0][0] | 0x6600;
call_ptr(c, t, 0x00377ae8)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377afc);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00377afc)?;
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
c.g[A3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5a14));
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5a00));
c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a00));
c.g[RA][0] = sx(0x00377b1c);
c.g[S2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a14));
Callee { addr: 0x00363490, f: ee::s12::PS2GraphicsMan_RenderFrame }.call(c, 0x00377b1c)?;
c.g[RA][0] = sx(0x00377b24);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5acc));
call_ptr(c, 0x00423de0, 0x00377b24)?;
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a10));
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5a90));
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A3][0] = 0x2;
c.sw((c.g[S3][0] as u32).wrapping_add(0x5aa4), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5aa4));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.sw((c.g[S3][0] as u32).wrapping_add(0x5aac), c.g[A1][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[S3][0] as u32).wrapping_add(0x5ab4), c.g[A2][0]);
c.sw((c.g[S3][0] as u32).wrapping_add(0x5aa8), c.g[S1][0]);
c.sw((c.g[S3][0] as u32).wrapping_add(0x5ab0), c.g[S2][0]);
c.sw((c.g[V1][0] as u32), c.g[A3][0]);
c.g[RA][0] = sx(0x00377b64);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5acc));
call_ptr(c, 0x00423dc0, 0x00377b64)?;
c.g[RA][0] = sx(0x00377b6c);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5ac8));
call_ptr(c, 0x00423dc0, 0x00377b6c)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a10));
c.g[A1][0] = 0xc0000;
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a24));
c.g[A1][0] = c.g[A1][0] | 0xcc80;
c.g[V1][0] = (c.g[V1][0] < 0x1u64) as u64;
c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a0c));
c.mult(0, 4, 3, 5);
c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
c.mult(0, 8, 2, 16);
c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a20));
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a10), c.g[V1][0]);
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a24), c.g[V0][0]);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[A0][0] = c.g[A3][0];
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a14), c.g[A2][0]);
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a04), c.g[A3][0]);
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a00), c.g[A3][0]);
c.g[RA][0] = sx(0x00377bc0);
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a18), c.g[A2][0]);
call_ptr(c, 0x004247d8, 0x00377bc0)?;
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a14));
c.g[RA][0] = sx(0x00377bcc);
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
call_ptr(c, 0x004247d8, 0x00377bcc)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = 0x500000;
c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1420));
c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
c.ldr(4, (c.g[A3][0] as u32));
c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
c.sdr(4, (c.g[V1][0] as u32));
c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0xf));
c.sdr(5, (c.g[V1][0] as u32).wrapping_add(0x8));
c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.g[RA][0] = sx(0x00377c08);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
Callee { addr: 0x00362cc8, f: ee::s12::DeferMan_ResetFrame }.call(c, 0x00377c08)?;
c.g[RA][0] = sx(0x00377c10);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f4));
Callee { addr: 0x003673d0, f: ee::s12::TexMan_EndFrameStub }.call(c, 0x00377c10)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377c30);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00377c30)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_377c48;
} else {
c.cycles += 4;
c.mtc1(2, 20);
c.cvt_s_w(20, 20);
let t = true;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
if t {
break 'b_377c60;
} else {
break 'b_377c48;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 20);
c.cvt_s_w(20, 20);
c.add_s(20, 20, 20);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10d8));
break 'b_377c60;
}
'b_377ca0: {
'b_377c88: {
c.cycles += 6;
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377c70);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00377c70)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_377c88;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
if t {
break 'b_377ca0;
} else {
break 'b_377c88;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
break 'b_377ca0;
}
c.cycles += 19;
c.g[A0][0] = c.g[S0][0];
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.g[A1][0] = c.g[SP][0];
c.sw((c.g[SP][0] as u32), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00377cc0);
c.g[A2][0] = 0x1;
call_ptr(c, t, 0x00377cc0)?;
c.g[A0][0] = 0x500000;
c.g[RA][0] = sx(0x00377ccc);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x59d8));
Callee { addr: 0x00361f90, f: ee::s12::IntcTable_EndFrameStub }.call(c, 0x00377ccc)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_DrawTrail` (0x00381310): converted from the generated labelled-block body (every register, cycle, store and call
/// order kept; calls through `Callee` / `call_ptr`, unwinds propagate with `?`).
pub fn ps2gfx_draw_trail(c: &mut Cpu) {
    guest(c, |c| {
'b_3813a4: {
c.cycles += 34;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
c.g[V1][0] = 0xfffffffffffffc3f;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A3][0] = 0xffffffffc0000000;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S1][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S2][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[T0][0] = 0xfffffffffffffffc;
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
c.g[A2][0] = 0xffffffffe0000000;
c.g[A2][0] = c.g[A2][0] | 0x3ff;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x140;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.g[V0][0] = c.g[V0][0] | 0x2;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
let t = c.g[V1][0] != 0u64;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
if t {
break 'b_3813a4;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003813a0);
c.g[A0][0] = c.g[S1][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x003813a0)?;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_3813a4;
}
}
'b_3814f4: {
c.cycles += 76;
c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5860));
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x50));
c.lqc2(4, (c.g[V1][0] as u32));
c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[V0][0] as u32));
c.lqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.vu0().mula(0b1111, 4, Src::Bc(9, X));
c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
c.vu0().mula(0b1111, 4, Src::Bc(10, X));
c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
c.vu0().mula(0b1111, 4, Src::Bc(11, X));
c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
c.sqc2(12, (c.g[S0][0] as u32));
c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
c.lqc2(1, (c.g[S0][0] as u32));
c.lqc2(2, (c.g[S0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[SP][0] as u32));
c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x00381454);
c.g[A1][0] = c.g[S3][0];
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x00381454)?;
c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xd;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A0][0] = 0xd817;
c.g[A0][0] = c.g[A0][0] << 47;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A3][0] = c.g[A2][0];
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.g[SP][0];
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60));
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x48), 0u64);
c.g[V1][0] = c.g[V0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
let t = c.g[V1][0] == c.g[A0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), 0u64);
if t {
break 'b_3814f4;
} else {
c.cycles += 1;
'l_3814d8: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x003814d8); } continue 'l_3814d8;
} else {
break 'b_3814f4;
}
}
}
}
'b_381528: {
c.cycles += 5;
c.sw((c.g[A2][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if c.g[V1][0] == c.g[A0][0] {
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
break 'b_381528;
} else {
'l_381508: loop {
c.cycles += 7;
c.lq(2, (c.g[S0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381508); } continue 'l_381508;
} else {
c.cycles += 1;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
break 'b_381528;
}
}
}
}
'b_381554: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == c.g[A1][0];
c.g[A0][0] = c.g[T0][0];
if t {
break 'b_381554;
} else {
'l_381538: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381538); } continue 'l_381538;
} else {
break 'b_381554;
}
}
}
}
'b_381584: {
c.cycles += 5;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
let t = c.g[V1][0] == c.g[A0][0];
c.g[A1][0] = c.g[S2][0];
if t {
break 'b_381584;
} else {
'l_381568: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381568); } continue 'l_381568;
} else {
break 'b_381584;
}
}
}
}
'b_381674: {
c.cycles += 27;
c.g[V0][0] = 0x0;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x0));
c.g[V1][0] = 0x14000000;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = 0x8800;
c.g[A1][0] = c.g[A1][0] << 45;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A2][0] = 0x4;
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x003815d4);
c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x003815d4)?;
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
if t {
break 'b_381674;
} else {
c.cycles += 33;
c.g[A1][0] = 0x60000;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[V1][0] = 0x30000000;
c.g[V0][0] = c.g[V0][0] & 0x1f;
c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[V0][0] as u32));
c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_381674;
}
}
c.cycles += 31;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[V1][0] = 0xfffffffffffffffc;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[A0][0] = 0xffffffffe0000000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[A0][0] | 0x3ff;
c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[A2][0] = 0xfffffffffffffc3f;
c.g[A3][0] = 0xffffffffc0000000;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.pc = t; return Ok(());
    });
}

/// `DrawBoardWake` (0x003883b8): converted from the generated labelled-block body (every register, cycle, store and call
/// order kept; calls through `Callee` / `call_ptr`, unwinds propagate with `?`).
pub fn draw_board_wake(c: &mut Cpu) {
    guest(c, |c| {
'b_388444: {
c.cycles += 33;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[T2][0] = 0xffffffffc0000000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S0][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S1][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[A0][0] = 0xfffffffffffffc3f;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S4][0] = c.g[T0][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S5][0] = c.g[A3][0];
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x98));
c.g[S6][0] = c.g[A1][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S3][0] = c.g[T1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.mov_s(21, 12);
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | 0xc0;
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
let t = c.g[V1][0] != 0u64;
c.mov_s(20, 13);
if t {
break 'b_388444;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00388444);
c.g[A0][0] = c.g[S1][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00388444)?;
break 'b_388444;
}
}
'b_3884a4: {
c.cycles += 9;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
c.g[A1][0] = 0u64;
c.g[S7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[RA][0] = sx(0x00388458);
c.g[A2][0] = 0x30;
call_ptr(c, 0x003e6448, 0x00388458)?;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
c.g[T1][0] = 0u64;
let t = (c.g[V0][0] as i64) <= 0;
c.g[S2][0] = c.g[V0][0];
if t {
break 'b_3884a4;
} else {
c.cycles += 2;
c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
'l_388470: loop {
c.cycles += 13;
c.g[A0][0] = c.g[S1][0];
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.g[S5][0];
c.g[A3][0] = c.g[S4][0];
c.g[T0][0] = c.g[S3][0];
c.g[T2][0] = c.g[S0][0];
c.mov_s(12, 21);
c.g[RA][0] = sx(0x00388494);
c.mov_s(13, 20);
Callee { addr: 0x003885e0, f: ee::s12::PS2GraphicsMan_EmitBoardWakeStrips }.call(c, 0x00388494)?;
c.g[T1][0] = c.g[S0][0];
c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[S2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x00388470); } continue 'l_388470;
} else {
break 'b_3884a4;
}
}
}
}
'b_388540: {
c.cycles += 6;
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
c.g[T1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
if t {
break 'b_388540;
} else {
c.cycles += 33;
c.g[A1][0] = 0x60000;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[V1][0] = 0x30000000;
c.g[V0][0] = c.g[V0][0] & 0x1f;
c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[V0][0] as u32));
c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S7][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_388540;
}
}
c.cycles += 39;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[A1][0] = 0xfffffffffffffc3f;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0xffffffffc0000000;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V0][0] = 0x500000;
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x98));
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1420));
c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[A3][0] as u32));
c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[A0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[A0][0] as u32));
c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0xf));
c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x8));
c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of the oc1482 fakes (its own word and magic), the per-call
    /// "result is zero" mask and the per-call "result is negative" mask the
    /// virtual-call miss hook applies (bit n = the n-th logged call).
    const OC1482_CTRL: u32 = DATA + 0xe_0000;
    const OC1482_MAGIC: u32 = 0x1482_0274;
    const OC1482_ZERO: u32 = OC1482_CTRL + 4;
    const OC1482_NEG: u32 = OC1482_CTRL + 8;

    /// Fixture addresses (all outside the fakes' pointer range DATA..DATA+0x40000).
    const OC1482_OBJ: u32 = DATA + 0x5_0000;
    const OC1482_MOD1: u32 = DATA + 0x6_0000;
    const OC1482_MOD2: u32 = DATA + 0x6_1000;
    const OC1482_EXEC: u32 = DATA + 0x6_2000;
    const OC1482_COUNTER: u32 = DATA + 0x7_0000;
    const OC1482_VT: u32 = DATA + 0x8_0000;
    const OC1482_GS: u32 = DATA + 0x8_1000;
    const OC1482_DMA: u32 = DATA + 0x8_2000;
    const OC1482_PKT: u32 = DATA + 0x8_3000;
    const OC1482_MAT: u32 = DATA + 0x8_4000;

    /// Gated fakes for every callee that has an override slot: with the
    /// gate word set they log the registers (`fakes::fake`) and return; with
    /// it clear they run what the slot held before.
    macro_rules! oc1482_fakes {
        ($($fake:ident, $slot:path, $gen:path, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1482_CTRL) != OC1482_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                }
            )*

            fn oc1482_hooks() {
                $(
                    fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
                )*
            }
        };
    }

    oc1482_fakes! {
        oc1482_fake_synctask, ee::s14::OVERRIDE_003E5928, ee::s14::f_003e5928__generated, 0x003e_5928;
        oc1482_fake_memset, ee::s14::OVERRIDE_003E6448, ee::s14::f_003e6448__generated, 0x003e_6448;
        oc1482_fake_exec_cb, ee::s10::OVERRIDE_00317328, ee::s10::f_00317328__generated, 0x0031_7328;
        oc1482_fake_load_complete, ee::s10::OVERRIDE_00317600, ee::s10::f_00317600__generated, 0x0031_7600;
        oc1482_fake_load_update, ee::s10::OVERRIDE_003175A0, ee::s10::f_003175a0__generated, 0x0031_75a0;
        oc1482_fake_load_init, ee::s10::OVERRIDE_00317530, ee::s10::f_00317530__generated, 0x0031_7530;
        oc1482_fake_frame_check, ee::s10::OVERRIDE_00319D18, ee::s10::f_00319d18__generated, 0x0031_9d18;
        oc1482_fake_frame_begin, ee::s10::OVERRIDE_00319D10, ee::s10::f_00319d10__generated, 0x0031_9d10;
        oc1482_fake_reset_bloom, ee::s12::OVERRIDE_0036C740, ee::s12::f_0036c740__generated, 0x0036_c740;
        oc1482_fake_blur_down, ee::s12::OVERRIDE_0036B9D8, ee::s12::f_0036b9d8__generated, 0x0036_b9d8;
        oc1482_fake_blend_fill, ee::s12::OVERRIDE_0036C188, ee::s12::f_0036c188__generated, 0x0036_c188;
        oc1482_fake_bloom_comp, ee::s12::OVERRIDE_0036C398, ee::s12::f_0036c398__generated, 0x0036_c398;
        oc1482_fake_path3, ee::s12::OVERRIDE_00368138, ee::s12::f_00368138__generated, 0x0036_8138;
        oc1482_fake_render_frame, ee::s12::OVERRIDE_00363490, ee::s12::f_00363490__generated, 0x0036_3490;
        oc1482_fake_defer_reset, ee::s12::OVERRIDE_00362CC8, ee::s12::f_00362cc8__generated, 0x0036_2cc8;
        oc1482_fake_tex_end, ee::s12::OVERRIDE_003673D0, ee::s12::f_003673d0__generated, 0x0036_73d0;
        oc1482_fake_intc_end, ee::s12::OVERRIDE_00361F90, ee::s12::f_00361f90__generated, 0x0036_1f90;
        oc1482_fake_wait_sema, ee::s15::OVERRIDE_00423DE0, ee::s15::f_00423de0__generated, 0x0042_3de0;
        oc1482_fake_signal_sema, ee::s15::OVERRIDE_00423DC0, ee::s15::f_00423dc0__generated, 0x0042_3dc0;
        oc1482_fake_inval_dcache, ee::s15::OVERRIDE_004247D8, ee::s15::f_004247d8__generated, 0x0042_47d8;
        oc1482_fake_view_mats, ee::s12::OVERRIDE_0037D968, ee::s12::f_0037d968__generated, 0x0037_d968;
        oc1482_fake_sprs_begin, ee::s12::OVERRIDE_0038F460, ee::s12::f_0038f460__generated, 0x0038_f460;
        oc1482_fake_sprs_end, ee::s12::OVERRIDE_0038F668, ee::s12::f_0038f668__generated, 0x0038_f668;
        oc1482_fake_wake_strips, ee::s12::OVERRIDE_003885E0, ee::s12::f_003885e0__generated, 0x0038_85e0;
    }

    /// The miss hook for virtual calls into FAKE: the shared fake, then the
    /// per-call result mask (zero / negative v0) of this fixture.
    fn oc1482_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, target);
        if n < 32 {
            if (c.bus.read32(OC1482_ZERO) >> n) & 1 != 0 {
                c.g[V0][0] = 0;
            } else if (c.bus.read32(OC1482_NEG) >> n) & 1 != 0 {
                c.g[V0][0] = sx(c.g[V0][0] as u32 | 0x8000_0000);
            }
        }
    }

    /// `fakes::machine` plus the gated fakes, the gate word and the masks.
    fn oc1482_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = fakes::machine(c, seed, salt, calls);
        oc1482_hooks();
        c.hooks.miss = oc1482_miss;
        c.bus.write32(OC1482_CTRL, OC1482_MAGIC);
        c.bus.write32(OC1482_ZERO, r.next_u32() & r.next_u32());
        c.bus.write32(OC1482_NEG, r.next_u32());
        r
    }

    /// Random words at `addr..addr + 4 * n`.
    fn oc1482_words(c: &mut Cpu, r: &mut Rng, addr: u32, n: u32) {
        for k in 0..n {
            c.bus.write32(addr + 4 * k, r.next_u32());
        }
    }

    /// A vtable at `vt` whose entries `{delta: i16, fn: FAKE}` sit at the
    /// given offsets (delta at `off`, function at `off + 4`).
    fn oc1482_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, offs: &[u32]) {
        for &off in offs {
            c.bus.write16(vt + off, (r.next_u32() & 0xf8) as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE);
        }
    }

    /// The GS-state block at `OBJ+0xe84` (read and rewritten by the packet
    /// builders) and the packet counter object at `OBJ+0x18f0`: small
    /// counters take the "record the DMA tag" path, large ones skip it.
    fn oc1482_gs_and_counter(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(OC1482_OBJ + 0xe84, OC1482_GS);
        oc1482_words(c, r, OC1482_GS, 6);
        c.bus.write32(OC1482_OBJ + 0x6b90, (seed % 2) as u32);
        c.bus.write32(OC1482_OBJ + 0x18f0, OC1482_COUNTER);
        let count = if seed % 5 == 0 { 2600 + seed } else { seed % 40 };
        c.bus.write32(OC1482_COUNTER, count);
        oc1482_words(c, r, OC1482_COUNTER + 0x68000 + 0x1cc0, 2);
        c.bus.write32(GAME_GP + 0x2a94, OC1482_DMA);
        // The constant 16-byte block the functions copy into the GS state.
        oc1482_words(c, r, 0x0050_1420, 5);
    }

    // ---- DrawBoardWake (PS2GraphicsMan_v90) ------------------------------

    /// Fixture: the manager at `OBJ`; seeds vary the view-matrix flag, the
    /// strip count (0..3: no loop / one / two passes) and the packet counter.
    fn oc1482_wake_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1482_machine(c, seed, 0x3883b8, 5);
        c.pc = 0x0038_83b8;
        c.g[A0][0] = sx(OC1482_OBJ);
        c.g[A2][0] = (seed % 4) as u64;
        oc1482_gs_and_counter(c, &mut r, seed);
        c.bus.write32(OC1482_OBJ + 0x5a00, r.next_u32());
    }

    /// `DrawBoardWake` (0x3883b8): clears the GS-state blend bits, memsets
    /// the DMA block, emits the wake strips in a loop and records the packet
    /// when the counter is below 2600. Callees are gated fakes.
    #[test]
    fn draw_board_wake_matches() {
        fakes::covered(
            ee::s12::f_003883b8__generated,
            draw_board_wake,
            200,
            &|c, seed| oc1482_wake_setup(c, seed),
        );
    }

    // ---- DrawTrail (PS2GraphicsMan_v85) -----------------------------------

    /// Fixture: manager at `OBJ`, a 0x90-byte matrix block at `MAT`, the
    /// view matrix at `OBJ+0x5860`; seeds vary the view flag and counter.
    fn oc1482_trail_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1482_machine(c, seed, 0x381310, 3);
        c.pc = 0x0038_1310;
        c.g[A0][0] = sx(OC1482_OBJ);
        c.g[A1][0] = sx(OC1482_MAT);
        oc1482_gs_and_counter(c, &mut r, seed);
        oc1482_words(c, &mut r, OC1482_MAT, 0x24);
        oc1482_words(c, &mut r, OC1482_OBJ + 0x5860, 16);
        c.bus.write32(OC1482_OBJ + 0x5a00, r.next_u32());
        c.bus.write32(GAME_GP + 0x1384, r.next_u32());
    }

    /// `PS2GraphicsMan_v85` (0x381310): builds a GIF packet for one trail
    /// (view * model matrix through VU0, copies of the matrix blocks) with a
    /// SPR stream block around it; view update and block calls are fakes.
    #[test]
    fn ps2gfx_draw_trail_matches() {
        fakes::covered(
            ee::s12::f_00381310__generated,
            ps2gfx_draw_trail,
            200,
            &|c, seed| oc1482_trail_setup(c, seed),
        );
    }

    // ---- EndFrame ---------------------------------------------------------

    /// Fixture: manager at `OBJ` with the device vtable at `OBJ+0x10d8` (all
    /// virtual calls land in the miss hook, whose negative-result mask picks
    /// the "odd size, halve and round" arms); seeds vary the semaphore
    /// state words and the unwind point.
    fn oc1482_end_frame_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1482_machine(c, seed, 0x377a10, 16);
        c.pc = 0x0037_7a10;
        c.g[A0][0] = sx(OC1482_OBJ);
        let vt = OC1482_VT;
        c.bus.write32(OC1482_OBJ + 0x10d8, vt);
        oc1482_vtable(c, &mut r, vt, &[0x50, 0x58, 0x1b0, 0xc0]);
        oc1482_words(c, &mut r, OC1482_OBJ + 0x5a00, 0x40);
        oc1482_words(c, &mut r, OC1482_OBJ + 0x18f0, 2);
        c.bus.write32(OC1482_OBJ + 0x5a10, seed % 4);
        c.bus.write32(OC1482_OBJ + 0x5a24, (seed / 4) % 3);
        c.bus.write32(OC1482_OBJ + 0xe84, OC1482_GS);
        oc1482_words(c, &mut r, OC1482_GS, 6);
        oc1482_words(c, &mut r, 0x0050_1420, 5);
    }

    /// `PS2GraphicsMan_EndFrame` (0x377a10): reads the display size through
    /// the device vtable, renders the frame, flips the semaphore-guarded
    /// double buffer, flushes the data cache and resets the frame helpers.
    #[test]
    fn ps2gfx_end_frame_matches() {
        fakes::covered(
            ee::s12::f_00377a10__generated,
            ps2gfx_end_frame,
            300,
            &|c, seed| oc1482_end_frame_setup(c, seed),
        );
    }

    // ---- RenderBloom ------------------------------------------------------

    /// A float word: zero, or a value in [0.3, 1.0) so that `* 255` truncates
    /// to a non-zero integer.
    fn oc1482_unit(r: &mut Rng, nonzero: bool) -> u32 {
        if nonzero {
            (0.3 + (r.next_u32() % 700) as f32 / 1000.0).to_bits()
        } else {
            0
        }
    }

    /// Fixture: manager at `OBJ` with bloom parameters at
    /// `OBJ + 0x6cd4 + 0x1c * a1`. The four float parameters (two from gp,
    /// two from the parameter block) are zero or non-zero by bits of the
    /// seed: all zero leaves early, otherwise the blur/composite loop runs.
    fn oc1482_bloom_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1482_machine(c, seed, 0x36c790, 15);
        c.pc = 0x0036_c790;
        let idx = seed % 4;
        c.g[A0][0] = sx(OC1482_OBJ);
        c.g[A1][0] = idx as u64;
        c.g[A3][0] = sx(OC1482_PKT);
        c.bus.write32(OC1482_PKT, OC1482_PKT + 0x100);
        c.bus.write32(GAME_GP + 0x12d8, if seed % 7 == 0 { 0 } else { 1 });
        c.bus.write32(GAME_GP + 0x12dc, (seed / 4) % 2);
        let mask = (seed / 8) % 16;
        let p = OC1482_OBJ + 0x6cd4 + 0x1c * idx;
        let w = oc1482_unit(&mut r, mask & 1 != 0);
        c.bus.write32(GAME_GP + 0x12f8, w);
        let w = oc1482_unit(&mut r, mask & 2 != 0);
        c.bus.write32(GAME_GP + 0x12fc, w);
        let w = oc1482_unit(&mut r, mask & 4 != 0);
        c.bus.write32(p + 0x14, w);
        let w = oc1482_unit(&mut r, mask & 8 != 0);
        c.bus.write32(p + 0x18, w);
        for off in [0u32, 8, 0xc, 0x10] {
            let w = oc1482_unit(&mut r, true);
            c.bus.write32(p + off, w);
        }
        c.bus.write32(GAME_GP + 0x12e0, r.next_u32() % 4);
        let w = oc1482_unit(&mut r, true);
        c.bus.write32(GAME_GP + 0x1308, w);
        let w = oc1482_unit(&mut r, true);
        c.bus.write32(GAME_GP + 0x12e8, w);
        c.bus.write32(OC1482_OBJ + 0x5a80, r.next_u32() % 64);
        c.bus.write32(OC1482_OBJ + 0x5a84, r.next_u32() % 64);
        oc1482_words(c, &mut r, OC1482_OBJ + 0x59b0, 0x20);
        c.bus.write32(OC1482_OBJ + 0x10d8, OC1482_VT);
        oc1482_vtable(c, &mut r, OC1482_VT, &[0x50, 0x58]);
        oc1482_words(c, &mut r, OC1482_OBJ + 0x18f4, 1);
    }

    /// `PS2GraphicsMan_RenderBloom` (0x36c790): turns the bloom colour
    /// parameters into bytes (`* 255`), emits the blur down-sample chain and
    /// the composite passes, and ends with a PATH3 window packet. Callees
    /// are gated fakes; the device vtable calls land in the miss hook.
    #[test]
    fn ps2gfx_render_bloom_matches() {
        fakes::covered(
            ee::s12::f_0036c790__generated,
            ps2gfx_render_bloom,
            300,
            &|c, seed| oc1482_bloom_setup(c, seed),
        );
    }

    // ---- App_RunModuleLoop ------------------------------------------------

    /// Fixture: the application manager at `OBJ` (module words +0/+4, execution
    /// manager +8 whose vtable pointer is at its +8, state +0x58, its own
    /// vtable +0x5c), one shared vtable of FAKE entries. State 3 ("ending")
    /// with a non-zero first result leaves the loop for real; every other
    /// seed unwinds out of the endless loop at a chosen fake call.
    fn oc1482_app_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1482_machine(c, seed, 0x316f00, 40);
        c.pc = 0x0031_6f00;
        c.g[A0][0] = sx(OC1482_OBJ);
        let state = [3u32, 1, 2, 0, 3, 5, 2, 0xffff_fffe][(seed % 8) as usize];
        let ends = seed % 8 == 0;
        let offs: Vec<u32> = (0..0x70).step_by(8).collect();
        oc1482_vtable(c, &mut r, OC1482_VT, &offs);
        c.bus.write32(OC1482_OBJ, OC1482_MOD1);
        c.bus.write32(OC1482_OBJ + 4, if seed % 3 == 0 { 0 } else { OC1482_MOD2 });
        c.bus.write32(OC1482_OBJ + 8, OC1482_EXEC);
        c.bus.write32(OC1482_OBJ + 0x58, state);
        c.bus.write32(OC1482_OBJ + 0x5c, OC1482_VT);
        c.bus.write32(OC1482_MOD1, OC1482_VT);
        c.bus.write32(OC1482_MOD2, OC1482_VT);
        c.bus.write32(OC1482_EXEC + 8, OC1482_VT);
        c.bus.write32(OC1482_OBJ + 0x1c, r.next_u32() % 100);
        c.bus.write32(OC1482_OBJ + 0x20, r.next_u32() % 4);
        c.bus.write32(OC1482_OBJ + 0x34, (seed / 8) % 2);
        c.bus.write32(OC1482_OBJ + 0x10, 60 + r.next_u32() % 5);
        for off in [0x24u32, 0x2c, 0x30] {
            let w = r.f32(2.0);
            c.bus.write32(OC1482_OBJ + off, w);
        }
        if ends {
            // The first state-3 virtual call (fake call 4) must return non-zero.
            let z = c.bus.read32(OC1482_ZERO);
            c.bus.write32(OC1482_ZERO, z & !(1 << 4));
            c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
        } else {
            c.bus.write32(fakes::LOG_UNWIND_AT, 5 + (seed / 8) % 30);
        }
    }

    /// `App_RunModuleLoop` (0x316f00): the application main loop (execution
    /// manager tick, load state machine, fixed-step module updates, frame
    /// rate smoothing). A resumable dispatcher: the whole `match pc` is kept.
    #[test]
    fn app_run_module_loop_matches() {
        fakes::covered(
            ee::s10::f_00316f00__generated,
            app_run_module_loop,
            300,
            &|c, seed| oc1482_app_setup(c, seed),
        );
    }

    /// The fixtures reach many distinct paths of the dispatcher (cycle
    /// totals of the generated run over the seeds).
    #[test]
    fn app_run_module_loop_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc1482_app_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s10::f_00316f00__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 12, "only {} distinct paths: {seen:?}", seen.len());
    }
}
