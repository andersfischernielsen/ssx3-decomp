//! Staging module oc1126 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1126`.

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
    0x0018_5268,
    c_fe_state_uber_trick_on_widget_action,
    "cFEStateUberTrick_OnWidgetAction",
)];

/// `cFEStateUberTrick_OnWidgetAction` (0x00185268): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_uber_trick_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00185268 {
        return ee::s02::f_00185268__generated(c);
    }
    guest(c, |c| {

'l_185268: loop {
'b_1859b8: {
'b_1859b4: {
'b_1859b0: {
c.cycles += 14;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S0][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[V0][0] == 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
if t {
break 'b_1859b0;
} else {
'b_185998: {
'b_1856f4: {
'b_185654: {
'b_185618: {
'b_185614: {
'b_185558: {
c.cycles += 7;
c.g[V0][0] = 0x460000;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x24a0));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
let t = c.g[A0][0] as u32;
match t {
0x00185268 => {
if event_due(c) { deliver_event_here(c, 0x00185268); } continue 'l_185268;
}
0x001852bc => {
'b_18534c: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
if c.g[V1][0] != c.g[A0][0] {
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
break 'b_18534c;
} else {
c.cycles += 6;
c.g[A0][0] = 0x4a0000;
c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x001852dc);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14a8));
call_ptr(c, 0x00317670, 0x001852dc)?;
let t = c.g[S0][0] == c.g[V0][0];
c.g[A1][0] = 0u64;
if t {
break 'b_185614;
} else {
c.cycles += 26;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001852fc);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001852fc)?;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
c.g[RA][0] = sx(0x00185308);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0039b960, 0x00185308)?;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
c.g[A1][0] = 0x1;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x00185324);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00185324)?;
c.g[RA][0] = sx(0x0018532c);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00183c08, f: ee::s02::cFEStateUberTrick_SelectRightPanel }.call(c, 0x0018532c)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00185338);
c.g[A1][0] = 0x1;
Callee { addr: 0x00184668, f: ee::s02::cFEStateUberTrick_ShowCost }.call(c, 0x00185338)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00185344);
c.g[A1][0] = 0x1;
Callee { addr: 0x00184780, f: ee::s02::cFEStateUberTrick_ShowBuyButtons }.call(c, 0x00185344)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_18534c;
}
}
}
}
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
let t = c.g[A2][0] == 0u64;
c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
if t {
break 'b_1859b4;
} else {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_1859b8;
} else {
'b_185404: {
'b_185400: {
c.cycles += 18;
c.g[S4][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x18));
c.g[RA][0] = sx(0x00185374);
c.g[S5][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x18));
call_ptr(c, 0x0014dc80, 0x00185374)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185380);
c.g[A1][0] = 0x6;
call_ptr(c, 0x0014dd58, 0x00185380)?;
c.g[RA][0] = sx(0x00185388);
c.g[S3][0] = c.g[V0][0];
call_ptr(c, 0x0014dc80, 0x00185388)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185394);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0014dd58, 0x00185394)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x001853a8);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x00150048, 0x001853a8)?;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S3][0];
if t {
break 'b_185400;
} else {
c.cycles += 6;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x001853c0);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x0014ffb8, 0x001853c0)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x530000;
if t {
break 'b_185404;
} else {
c.cycles += 14;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x001853dc);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x0014fd80, 0x001853dc)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001853f0);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001853f0)?;
c.g[RA][0] = sx(0x001853f8);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00183de0, f: ee::s02::cFEStateUberTrick_RefreshTrickList }.call(c, 0x001853f8)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_185400;
}
}
}
}
c.cycles += 1;
c.g[V0][0] = 0x530000;
break 'b_185404;
}
'b_185528: {
c.cycles += 3;
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
if c.g[V1][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
break 'b_185528;
} else {
c.cycles += 7;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x00185424);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x0014ffb8, 0x00185424)?;
let t = c.g[V0][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
c.cycles += 40;
c.g[RA][0] = sx(0x00185434);
call_ptr(c, 0x0014dc80, 0x00185434)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185440);
c.g[A1][0] = 0x1;
call_ptr(c, 0x0014dd58, 0x00185440)?;
c.g[S0][0] = c.g[V0][0];
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[RA][0] = sx(0x00185450);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00147488, 0x00185450)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[S1][0] = sx((c.g[V0][0] as u32) << 24);
c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0018546c);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0018546c)?;
c.g[RA][0] = sx(0x00185474);
call_ptr(c, 0x0014dc80, 0x00185474)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185480);
c.g[A1][0] = 0xb;
call_ptr(c, 0x0014dd58, 0x00185480)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A2][0] = c.g[S1][0];
c.g[RA][0] = sx(0x00185490);
c.g[A0][0] = c.g[V0][0];
call_ptr(c, 0x00150928, 0x00185490)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[S3][0];
c.g[S6][0] = c.g[V0][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x001854a8);
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
call_ptr(c, 0x0014ff90, 0x001854a8)?;
c.g[V1][0] = sx((c.g[S5][0] as u32) << 3);
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[RA][0] = sx(0x001854b8);
c.g[A0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x4));
call_ptr(c, 0x00116938, 0x001854b8)?;
c.g[S3][0] = c.g[V0][0];
c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
break 'b_185528;
} else {
c.cycles += 23;
c.g[A1][0] = 0x460000;
c.g[A0][0] = 0x70;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2528));
c.g[A2][0] = 0x100;
c.g[RA][0] = sx(0x001854e4);
c.g[A3][0] = 0u64;
call_ptr(c, 0x00317d70, 0x001854e4)?;
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[V0][0];
c.g[A3][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[RA][0] = sx(0x001854f8);
c.g[A2][0] = c.g[S2][0];
call_ptr(c, 0x001caba8, 0x001854f8)?;
c.g[S0][0] = c.g[V0][0];
c.g[A2][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0x6));
c.g[A0][0] = c.g[S0][0];
c.g[A1][0] = c.g[S3][0];
c.g[RA][0] = sx(0x00185510);
c.g[A3][0] = c.g[S6][0];
call_ptr(c, 0x001caf58, 0x00185510)?;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00185520);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
call_ptr(c, 0x0039f290, 0x00185520)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_185528;
}
}
}
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
if c.g[A2][0] == 0 { c.g[A2][0] = c.g[V1][0]; }
let t = c.g[A2][0] == 0u64;
c.g[A1][0] = 0x4;
if t {
break 'b_1859b0;
} else {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A2][0] as u32));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x00185550);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00185550)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_185558;
}
}
}
}
}
}
0x00185558 => {
break 'b_185558;
}
0x00185654 => {
break 'b_185654;
}
0x001856f4 => {
break 'b_1856f4;
}
0x00185998 => {
break 'b_185998;
}
0x001859b0 => {
break 'b_1859b0;
}
_ => { c.jump_out(t, 0x00185268, 0x001859d8); return Ok(()); }
}
}
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
if c.g[V1][0] != c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
break 'b_185618;
} else {
'b_1855e4: {
c.cycles += 18;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
c.g[A1][0] = 0x1;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x00185584);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00185584)?;
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
c.g[A1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x001855a0);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001855a0)?;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
if c.g[V0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
break 'b_1855e4;
} else {
c.cycles += 13;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
c.swc1(0, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001855e0);
c.g[A1][0] = c.g[SP][0];
call_ptr(c, t, 0x001855e0)?;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
break 'b_1855e4;
}
}
c.cycles += 12;
c.g[RA][0] = sx(0x001855ec);
c.g[A1][0] = 0xff;
call_ptr(c, 0x0039b960, 0x001855ec)?;
c.g[RA][0] = sx(0x001855f4);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00183b08, f: ee::s02::cFEStateUberTrick_SelectLeftPanel }.call(c, 0x001855f4)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00185600);
c.g[A1][0] = 0u64;
Callee { addr: 0x00184668, f: ee::s02::cFEStateUberTrick_ShowCost }.call(c, 0x00185600)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x0018560c);
c.g[A1][0] = 0u64;
Callee { addr: 0x00184780, f: ee::s02::cFEStateUberTrick_ShowBuyButtons }.call(c, 0x0018560c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_185614;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
break 'b_185618;
}
c.cycles += 10;
c.g[A1][0] = c.g[S2][0];
c.g[A2][0] = 0u64;
c.g[A3][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00185638);
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00185638)?;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.g[V0][0];
if t {
break 'b_1859b0;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
c.g[RA][0] = sx(0x0018564c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
call_ptr(c, 0x0039f400, 0x0018564c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_185654;
}
}
}
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
let t = c.g[A1][0] != c.g[V0][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
let t = c.g[S1][0] == 0u64;
c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xa0));
if t {
break 'b_1859b0;
} else {
c.cycles += 2;
if c.g[S0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1859b4;
} else {
c.cycles += 16;
c.g[RA][0] = sx(0x00185684);
call_ptr(c, 0x0014dc80, 0x00185684)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185690);
c.g[A1][0] = 0x6;
call_ptr(c, 0x0014dd58, 0x00185690)?;
c.g[S3][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x18));
c.g[A0][0] = c.g[V0][0];
c.g[A2][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x18));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[RA][0] = sx(0x001856ac);
c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
call_ptr(c, 0x0014ff90, 0x001856ac)?;
c.g[S0][0] = c.g[V0][0];
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_1859b0;
} else {
c.cycles += 14;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
c.g[RA][0] = sx(0x001856cc);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
call_ptr(c, 0x001a0548, 0x001856cc)?;
c.g[V1][0] = sx((c.g[S3][0] as u32) << 3);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 12);
c.g[A1][0] = c.lhu((c.g[V1][0] as u32));
c.g[RA][0] = sx(0x001856ec);
c.g[A2][0] = 0u64;
call_ptr(c, 0x003128e8, 0x001856ec)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_1856f4;
}
}
}
}
}
}
'b_1857cc: {
c.cycles += 11;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
c.g[RA][0] = sx(0x00185708);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
call_ptr(c, 0x001a0548, 0x00185708)?;
c.g[V1][0] = 0x1;
c.sw((c.g[V0][0] as u32).wrapping_add(0xcd0), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
if t {
break 'b_1857cc;
} else {
'b_1857b0: {
'b_18577c: {
'b_185778: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0xffffffffffffffff;
if t {
break 'b_185778;
} else {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
if c.g[V1][0] == c.g[V0][0] {
c.g[A1][0] = 0u64;
break 'b_18577c;
} else {
c.cycles += 14;
c.g[A1][0] = 0x1;
c.g[RA][0] = sx(0x00185740);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00184828, f: ee::s02::cFEStateUberTrick_ShowExecuteInfo }.call(c, 0x00185740)?;
c.g[RA][0] = sx(0x00185748);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00183de0, f: ee::s02::cFEStateUberTrick_RefreshTrickList }.call(c, 0x00185748)?;
c.g[A0][0] = 0x460000;
c.g[RA][0] = sx(0x00185754);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26e0));
call_ptr(c, 0x00317670, 0x00185754)?;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
c.g[RA][0] = sx(0x00185760);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039d860, 0x00185760)?;
c.g[S0][0] = c.g[V0][0];
if c.g[S0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1859b4;
} else {
c.cycles += 3;
c.g[A0][0] = 0x460000;
let t = true;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2518));
if t {
break 'b_1857b0;
} else {
break 'b_185778;
}
}
}
}
}
c.cycles += 1;
c.g[A1][0] = 0u64;
break 'b_18577c;
}
c.cycles += 11;
c.g[RA][0] = sx(0x00185784);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00184828, f: ee::s02::cFEStateUberTrick_ShowExecuteInfo }.call(c, 0x00185784)?;
c.g[A0][0] = 0x460000;
c.g[RA][0] = sx(0x00185790);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26e0));
call_ptr(c, 0x00317670, 0x00185790)?;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
c.g[RA][0] = sx(0x0018579c);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039d860, 0x0018579c)?;
c.g[S0][0] = c.g[V0][0];
if c.g[S0][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1859b4;
} else {
c.cycles += 2;
c.g[A0][0] = 0x460000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2500));
break 'b_1857b0;
}
}
c.cycles += 7;
c.g[RA][0] = sx(0x001857b8);
call_ptr(c, 0x00317670, 0x001857b8)?;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x001857c4);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x003a1000, 0x001857c4)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_1857cc;
}
}
}
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
if t {
break 'b_1859b8;
} else {
c.cycles += 12;
c.g[A0][0] = 0x460000;
c.g[RA][0] = sx(0x001857e8);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x26d0));
call_ptr(c, 0x00317670, 0x001857e8)?;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
c.g[RA][0] = sx(0x001857f4);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x0039d860, 0x001857f4)?;
c.g[S6][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4c));
c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
let t = c.g[A2][0] == 0u64;
c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
if t {
break 'b_1859b0;
} else {
c.cycles += 2;
if c.g[V1][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1859b4;
} else {
c.cycles += 2;
if c.g[S6][0] == 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1859b4;
} else {
'b_185988: {
'b_185974: {
'b_185958: {
'b_18588c: {
'b_185888: {
c.cycles += 18;
c.g[S4][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x18));
c.g[RA][0] = sx(0x00185828);
c.g[S5][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x18));
call_ptr(c, 0x0014dc80, 0x00185828)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185834);
c.g[A1][0] = 0x6;
call_ptr(c, 0x0014dd58, 0x00185834)?;
c.g[RA][0] = sx(0x0018583c);
c.g[S3][0] = c.g[V0][0];
call_ptr(c, 0x0014dc80, 0x0018583c)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185848);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0014dd58, 0x00185848)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x0018585c);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x00150048, 0x0018585c)?;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S3][0];
if t {
break 'b_185888;
} else {
c.cycles += 6;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x00185874);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x0014ffb8, 0x00185874)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x530000;
if t {
break 'b_18588c;
} else {
c.cycles += 3;
c.g[A0][0] = 0x460000;
let t = true;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2590));
if t {
break 'b_185958;
} else {
break 'b_185888;
}
}
}
}
c.cycles += 1;
c.g[V0][0] = 0x530000;
break 'b_18588c;
}
c.cycles += 3;
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
let t = c.g[V1][0] != 0u64;
c.g[A0][0] = 0x460000;
if t {
break 'b_185974;
} else {
c.cycles += 7;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x001858ac);
c.g[A3][0] = c.g[S5][0];
call_ptr(c, 0x0014ffb8, 0x001858ac)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_185988;
} else {
'b_185954: {
c.cycles += 37;
c.g[RA][0] = sx(0x001858bc);
call_ptr(c, 0x0014dc80, 0x001858bc)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x001858c8);
c.g[A1][0] = 0x1;
call_ptr(c, 0x0014dd58, 0x001858c8)?;
c.g[S0][0] = c.g[V0][0];
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[RA][0] = sx(0x001858d8);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00147488, 0x001858d8)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[S1][0] = sx((c.g[V0][0] as u32) << 24);
c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001858f4);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001858f4)?;
c.g[RA][0] = sx(0x001858fc);
call_ptr(c, 0x0014dc80, 0x001858fc)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185908);
c.g[A1][0] = 0xb;
call_ptr(c, 0x0014dd58, 0x00185908)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00185918);
c.g[A2][0] = c.g[S1][0];
call_ptr(c, 0x00150928, 0x00185918)?;
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[S0][0] = c.g[V0][0];
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x00185930);
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
call_ptr(c, 0x0014ff90, 0x00185930)?;
c.g[V1][0] = sx((c.g[S5][0] as u32) << 3);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x6));
c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[S0][0] != 0u64 {
c.g[A0][0] = 0x460000;
break 'b_185954;
} else {
c.cycles += 3;
c.g[A0][0] = 0x460000;
let t = true;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x24e8));
if t {
break 'b_185958;
} else {
break 'b_185954;
}
}
}
c.cycles += 1;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x24d0));
break 'b_185958;
}
}
}
c.cycles += 7;
c.g[RA][0] = sx(0x00185960);
call_ptr(c, 0x00317670, 0x00185960)?;
c.g[A0][0] = c.g[S6][0];
c.g[RA][0] = sx(0x0018596c);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x003a1000, 0x0018596c)?;
let t = true;

if t {
break 'b_185988;
} else {
break 'b_185974;
}
}
c.cycles += 5;
c.g[RA][0] = sx(0x0018597c);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x24b8));
call_ptr(c, 0x00317670, 0x0018597c)?;
c.g[A0][0] = c.g[S6][0];
c.g[RA][0] = sx(0x00185988);
c.g[A1][0] = c.g[V0][0];
call_ptr(c, 0x003a1000, 0x00185988)?;
break 'b_185988;
}
c.cycles += 4;
c.g[RA][0] = sx(0x00185990);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x00184520, f: ee::s02::cFEStateUberTrick_UpdateCost }.call(c, 0x00185990)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_1859b4;
} else {
break 'b_185998;
}
}
}
}
}
}
}
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
c.g[RA][0] = sx(0x001859ac);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
call_ptr(c, 0x001a0548, 0x001859ac)?;
c.sw((c.g[V0][0] as u32).wrapping_add(0xcd0), 0u64);
break 'b_1859b0;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_1859b4;
}
c.cycles += 1;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
break 'b_1859b8;
}
c.cycles += 8;
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
}
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine_rt, LOG_COUNT, LOG_START};
    use crate::difftest::{elf_image, fill, fresh, run_generated, DATA};

    const UBER_SELF: u32 = DATA + 0x1000;
    const UBER_ITEM: u32 = DATA + 0x3000;

    /// Fixture: `msg` cycles 0..=8 (0 and 8 are out of the jump table range),
    /// the shop state and the item are random words with the pointer fields
    /// (main menu, trick menu, parent) pointing at objects in the data area;
    /// even seeds make the item's parent the main menu.
    fn uber_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x1855_268, 12);
        elf_image(c);
        fill(c, &mut r, DATA, 0x2000);
        c.bus.write32(UBER_SELF + 0x4c, DATA + 0x5000);
        c.bus.write32(UBER_SELF + 0x50, DATA + 0x5800);
        let parent = match seed % 3 {
            0 => DATA + 0x5000,
            1 => DATA + 0x5800,
            _ => DATA + 0x6000,
        };
        c.bus.write32(UBER_ITEM + 0x40, parent);
        c.bus.write32(UBER_ITEM + 0x38, 0x0080_0000 + (seed % 5) * 0x10);
        c.g[A0][0] = sx(UBER_SELF);
        c.g[A1][0] = sx(UBER_ITEM);
        c.g[A2][0] = (seed % 9) as u64;
        c.pc = 0x0018_5268;
    }

    /// Port against the generated body (fake callees, unwinds included).
    #[test]
    fn c_fe_state_uber_trick_on_widget_action_matches() {
        covered(
            ee::s02::f_00185268__generated,
            c_fe_state_uber_trick_on_widget_action,
            400,
            &uber_setup,
        );
    }

    /// The fixture reaches several distinct call sequences.
    #[test]
    fn c_fe_state_uber_trick_on_widget_action_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400u32 {
            let mut c = fresh();
            uber_setup(&mut c, seed);
            run_generated(&mut c, ee::s02::f_00185268__generated);
            let n = c.bus.read32(LOG_COUNT).min(64);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(LOG_START + 0x220 * i))
                .collect();
            seen.insert(seq);
        }
        assert!(seen.len() >= 5, "only {} distinct call sequences", seen.len());
    }
}
