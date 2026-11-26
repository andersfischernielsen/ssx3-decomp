//! Staging module oc860 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc860`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x001a16c0, feui_custom_v05, "FEUICustom_v05")];

/// `FEUICustom_v05` (0x001a16c0): the normal entry, converted from the generated body
/// (a labelled-block body with a jump table: every register, cycle and call kept).
/// Picks the front-end state object for the flow state `*(a1 + 0xc)` (`a2` = state data,
/// `*(a1 + 0x44)` = user number): allocates it and calls its constructor.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_labels, unused_assignments, unused_variables)]
pub fn feui_custom_v05(c: &mut Cpu) {
    if c.pc != 0x001a16c0 {
        // At install: `f_001a16c0__generated` (the `__entries` twin exists now).
        return ee::s03::f_001a16c0__entries(c);
    }
    guest(c, |c| 'l_1a16c0: loop {
        'b_1a1ca4: {
            'b_1a1ca0: {
                'b_1a1c9c: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[S2][0] = c.g[A1][0];
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                    c.g[V0][0] = (c.g[V1][0] < 0x2eu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A2][0];
                    if t {
                        break 'b_1a1c9c;
                    } else {
                        'b_1a1c68: {
                            'b_1a1c34: {
                                'b_1a1c04: {
                                    'b_1a1bd8: {
                                        'b_1a1bb0: {
                                            'b_1a1bac: {
                                                'b_1a1ba0: {
                                                    'b_1a1b74: {
                                                        'b_1a1b64: {
                                                            'b_1a1b3c: {
                                                                'b_1a1b38: {
                                                                    'b_1a1b0c: {
                                                                        'b_1a1ad0: {
                                                                            'b_1a1a9c: {
                                                                                'b_1a1a74: {
                                                                                    'b_1a1a70: {
                                                                                        'b_1a1a40: {
                                                                                            'b_1a19fc: {
                                                                                                'b_1a19ec: {
                                                                                                    'b_1a19bc: {
                                                                                                        'b_1a19a8: {
                                                                                                            'b_1a19a4: {
                                                                                                                'b_1a1980: {
                                                                                                                    'b_1a1978: {
                                                                                                                        'b_1a1948: {
                                                                                                                            'b_1a190c: {
                                                                                                                                'b_1a18cc: {
                                                                                                                                    'b_1a189c: {
                                                                                                                                        'b_1a1870: {
                                                                                                                                            'b_1a186c: {
                                                                                                                                                'b_1a182c: {
                                                                                                                                                    'b_1a17f8: {
                                                                                                                                                        'b_1a17e4: {
                                                                                                                                                            'b_1a17d0: {
                                                                                                                                                                'b_1a17a0: {
                                                                                                                                                                    c.cycles += 7;
                                                                                                                                                                    c.g[V0][0] = 0x460000;
                                                                                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x11c0));
                                                                                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                                                                                                                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                                                                                                                                    let t = c.g[A0][0] as u32;
                                                                                                                                                                    match t {
0x001a16c0 => {
if event_due(c) { deliver_event_here(c, 0x001a16c0); } continue 'l_1a16c0;
}
0x001a170c => {
'b_1a1744: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
c.g[V0][0] = c.g[V0][0] ^ 0x1;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = 0x460000;
if t {
break 'b_1a1744;
} else {
c.cycles += 10;
c.g[A0][0] = 0x6d0;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
c.g[A2][0] = 0u64;
c.g[RA][0] = sx(0x001a1730);
c.g[A3][0] = 0u64;
call_ptr(c, 0x00317d70, 0x001a1730)?;
c.g[A1][0] = c.lw(c.g[S0][0] as u32);
c.g[RA][0] = sx(0x001a173c);
c.g[A0][0] = c.g[V0][0];
Callee { addr: 0x001bfbb8, f: ee::s03::cFEStateOnlineMainMenu_ctor }.call(c, 0x001a173c)?;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_1a1ca4;
} else {
break 'b_1a1744;
}
}
}
c.cycles += 10;
c.g[RA][0] = sx(0x001a174c);
call_ptr(c, 0x0014dc80, 0x001a174c)?;
c.g[A0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x001a1758);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0014dd58, 0x001a1758)?;
c.g[V0][0] = 0x530000;
c.g[A0][0] = 0x1;
c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
let t = c.g[V1][0] == c.g[A0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
if t {
break 'b_1a1978;
} else {
'b_1a1784: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_1a1784;
} else {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_1a1978;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_1a1ca4;
} else {
break 'b_1a1784;
}
}
}
}
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0u64;
if t {
break 'b_1a1ca0;
} else {
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
let t = c.g[V0][0] == c.g[A0][0];
c.g[A1][0] = 0x460000;
if t {
break 'b_1a186c;
} else {
c.cycles += 2;
let t = true;
c.g[A0][0] = 0x58;
if t {
break 'b_1a1980;
} else {
break 'b_1a17a0;
}
}
}
}
}
0x001a17a0 => {
break 'b_1a17a0;
}
0x001a17d0 => {
break 'b_1a17d0;
}
0x001a17e4 => {
break 'b_1a17e4;
}
0x001a17f8 => {
break 'b_1a17f8;
}
0x001a182c => {
break 'b_1a182c;
}
0x001a18cc => {
break 'b_1a18cc;
}
0x001a190c => {
break 'b_1a190c;
}
0x001a1948 => {
break 'b_1a1948;
}
0x001a1978 => {
break 'b_1a1978;
}
0x001a19a4 => {
break 'b_1a19a4;
}
0x001a19bc => {
break 'b_1a19bc;
}
0x001a19fc => {
break 'b_1a19fc;
}
0x001a1a40 => {
break 'b_1a1a40;
}
0x001a1b0c => {
break 'b_1a1b0c;
}
0x001a1b38 => {
break 'b_1a1b38;
}
0x001a1b64 => {
break 'b_1a1b64;
}
0x001a1ba0 => {
break 'b_1a1ba0;
}
0x001a1bac => {
break 'b_1a1bac;
}
0x001a1c04 => {
break 'b_1a1c04;
}
0x001a1c34 => {
break 'b_1a1c34;
}
0x001a1c68 => {
break 'b_1a1c68;
}
0x001a1c9c => {
break 'b_1a1c9c;
}
_ => { c.jump_out(t, 0x001a16c0, 0x001a1cb8); return Ok(()); }
}
                                                                                                                                                                }
                                                                                                                                                                c.cycles += 12;
                                                                                                                                                                c.g[A1][0] = 0x460000;
                                                                                                                                                                c.g[A0][0] = 0x90;
                                                                                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbc0));
                                                                                                                                                                c.g[A2][0] = 0u64;
                                                                                                                                                                c.g[RA][0] = sx(0x001a17b8);
                                                                                                                                                                c.g[A3][0] = 0u64;
                                                                                                                                                                call_ptr(c, 0x00317d70, 0x001a17b8)?;
                                                                                                                                                                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                                                                c.g[A0][0] = c.g[V0][0];
                                                                                                                                                                c.g[RA][0] = sx(0x001a17c8);
                                                                                                                                                                c.g[A2][0] = 0x1;
                                                                                                                                                                call_ptr(c, 0x00181090, 0x001a17c8)?;
                                                                                                                                                                let t = true;
                                                                                                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                                                                                if t {
break 'b_1a1ca4;
} else {
break 'b_1a17d0;
}
                                                                                                                                                            }
                                                                                                                                                            c.cycles += 3;
                                                                                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                                                                                                                                                            let t = c.g[V0][0] != 0u64;
                                                                                                                                                            c.g[A1][0] = 0x460000;
                                                                                                                                                            if t {
break 'b_1a1b3c;
} else {
c.cycles += 2;
let t = true;
c.g[A1][0] = 0x460000;
if t {
break 'b_1a1b74;
} else {
break 'b_1a17e4;
}
}
                                                                                                                                                        }
                                                                                                                                                        c.cycles += 3;
                                                                                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                                                                                                                                                        let t = c.g[V0][0] != 0u64;
                                                                                                                                                        c.g[A1][0] = 0x460000;
                                                                                                                                                        if t {
break 'b_1a1bb0;
} else {
c.cycles += 2;
let t = true;
c.g[A1][0] = 0x460000;
if t {
break 'b_1a1bd8;
} else {
break 'b_1a17f8;
}
}
                                                                                                                                                    }
                                                                                                                                                    c.cycles += 13;
                                                                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                                                                    c.g[A0][0] = 0x80;
                                                                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x650));
                                                                                                                                                    c.g[A2][0] = 0u64;
                                                                                                                                                    c.g[RA][0] = sx(0x001a1810);
                                                                                                                                                    c.g[A3][0] = 0u64;
                                                                                                                                                    call_ptr(c, 0x00317d70, 0x001a1810)?;
                                                                                                                                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                                                    c.g[A0][0] = c.g[V0][0];
                                                                                                                                                    c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                                                                                    c.g[RA][0] = sx(0x001a1824);
                                                                                                                                                    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                                                                                                                                                    call_ptr(c, 0x00195ff0, 0x001a1824)?;
                                                                                                                                                    let t = true;
                                                                                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                                                                    if t {
break 'b_1a1ca4;
} else {
break 'b_1a182c;
}
                                                                                                                                                }
                                                                                                                                                c.cycles += 9;
                                                                                                                                                c.g[RA][0] = sx(0x001a1834);
                                                                                                                                                call_ptr(c, 0x0014dc80, 0x001a1834)?;
                                                                                                                                                c.g[A0][0] = c.g[V0][0];
                                                                                                                                                c.g[RA][0] = sx(0x001a1840);
                                                                                                                                                c.g[A1][0] = 0u64;
                                                                                                                                                call_ptr(c, 0x0014dd58, 0x001a1840)?;
                                                                                                                                                c.g[V1][0] = 0x530000;
                                                                                                                                                c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                                                                                                                                                let t = (c.g[V1][0] as i64) < 0;
                                                                                                                                                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                                                                                                                                                if t {
break 'b_1a1c9c;
} else {
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.g[A1][0] = 0x460000;
break 'b_1a1870;
} else {
c.cycles += 3;
c.g[V0][0] = 0x2;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0u64;
if t {
break 'b_1a189c;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_1a1ca4;
} else {
break 'b_1a186c;
}
}
}
}
                                                                                                                                            }
                                                                                                                                            c.cycles += 1;
                                                                                                                                            c.g[A1][0] = 0x460000;
                                                                                                                                            break 'b_1a1870;
                                                                                                                                        }
                                                                                                                                        c.cycles += 11;
                                                                                                                                        c.g[A0][0] = 0x50;
                                                                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc08));
                                                                                                                                        c.g[A2][0] = 0u64;
                                                                                                                                        c.g[RA][0] = sx(0x001a1884);
                                                                                                                                        c.g[A3][0] = 0u64;
                                                                                                                                        call_ptr(c, 0x00317d70, 0x001a1884)?;
                                                                                                                                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                                        c.g[A0][0] = c.g[V0][0];
                                                                                                                                        c.g[RA][0] = sx(0x001a1894);
                                                                                                                                        c.g[A2][0] = 0u64;
                                                                                                                                        call_ptr(c, 0x001828c0, 0x001a1894)?;
                                                                                                                                        let t = true;
                                                                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                                                        if t {
break 'b_1a1ca4;
} else {
break 'b_1a189c;
}
                                                                                                                                    }
                                                                                                                                    c.cycles += 12;
                                                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                                                    c.g[A0][0] = 0x50;
                                                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc08));
                                                                                                                                    c.g[A2][0] = 0u64;
                                                                                                                                    c.g[RA][0] = sx(0x001a18b4);
                                                                                                                                    c.g[A3][0] = 0u64;
                                                                                                                                    call_ptr(c, 0x00317d70, 0x001a18b4)?;
                                                                                                                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                                    c.g[A0][0] = c.g[V0][0];
                                                                                                                                    c.g[RA][0] = sx(0x001a18c4);
                                                                                                                                    c.g[A2][0] = 0x1;
                                                                                                                                    call_ptr(c, 0x001828c0, 0x001a18c4)?;
                                                                                                                                    let t = true;
                                                                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                                                    if t {
break 'b_1a1ca4;
} else {
break 'b_1a18cc;
}
                                                                                                                                }
                                                                                                                                c.cycles += 16;
                                                                                                                                c.g[V0][0] = 0x1;
                                                                                                                                c.g[A1][0] = 0x460000;
                                                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc38));
                                                                                                                                c.g[A0][0] = 0x33c;
                                                                                                                                c.g[A2][0] = 0u64;
                                                                                                                                c.g[A3][0] = 0u64;
                                                                                                                                c.sw((c.g[GP][0] as u32).wrapping_sub(0xb58), c.g[V0][0]);
                                                                                                                                c.g[RA][0] = sx(0x001a18f0);
                                                                                                                                c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), 0u64);
                                                                                                                                call_ptr(c, 0x00317d70, 0x001a18f0)?;
                                                                                                                                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                                c.g[A0][0] = c.g[V0][0];
                                                                                                                                c.g[RA][0] = sx(0x001a1900);
                                                                                                                                c.g[A2][0] = 0u64;
                                                                                                                                call_ptr(c, 0x00186610, 0x001a1900)?;
                                                                                                                                c.g[V1][0] = c.g[V0][0];
                                                                                                                                let t = true;
                                                                                                                                c.sb((c.g[V1][0] as u32).wrapping_add(0x44), 0u64);
                                                                                                                                if t {
break 'b_1a1ca0;
} else {
break 'b_1a190c;
}
                                                                                                                            }
                                                                                                                            c.cycles += 15;
                                                                                                                            c.g[V0][0] = 0x1;
                                                                                                                            c.g[A1][0] = 0x460000;
                                                                                                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc38));
                                                                                                                            c.g[A0][0] = 0x33c;
                                                                                                                            c.g[A2][0] = 0u64;
                                                                                                                            c.sw((c.g[GP][0] as u32).wrapping_sub(0xb58), c.g[V0][0]);
                                                                                                                            c.g[A3][0] = 0u64;
                                                                                                                            c.g[RA][0] = sx(0x001a1930);
                                                                                                                            c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), 0u64);
                                                                                                                            call_ptr(c, 0x00317d70, 0x001a1930)?;
                                                                                                                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                            c.g[A0][0] = c.g[V0][0];
                                                                                                                            c.g[RA][0] = sx(0x001a1940);
                                                                                                                            c.g[A2][0] = 0u64;
                                                                                                                            call_ptr(c, 0x00186610, 0x001a1940)?;
                                                                                                                            let t = true;
                                                                                                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                                            if t {
break 'b_1a1ca4;
} else {
break 'b_1a1948;
}
                                                                                                                        }
                                                                                                                        c.cycles += 10;
                                                                                                                        c.g[RA][0] = sx(0x001a1950);
                                                                                                                        call_ptr(c, 0x0014dc80, 0x001a1950)?;
                                                                                                                        c.g[A0][0] = c.g[V0][0];
                                                                                                                        c.g[RA][0] = sx(0x001a195c);
                                                                                                                        c.g[A1][0] = 0u64;
                                                                                                                        call_ptr(c, 0x0014dd58, 0x001a195c)?;
                                                                                                                        c.g[V1][0] = 0x530000;
                                                                                                                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
                                                                                                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                                                                                                        if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
break 'b_1a19a8;
} else {
c.cycles += 2;
if (c.g[V1][0] as i64) < 0 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
break 'b_1a19a8;
} else {
break 'b_1a1978;
}
}
                                                                                                                    }
                                                                                                                    c.cycles += 2;
                                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                                    c.g[A0][0] = 0x58;
                                                                                                                    break 'b_1a1980;
                                                                                                                }
                                                                                                                c.cycles += 9;
                                                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xba8));
                                                                                                                c.g[A2][0] = 0u64;
                                                                                                                c.g[RA][0] = sx(0x001a1990);
                                                                                                                c.g[A3][0] = 0u64;
                                                                                                                call_ptr(c, 0x00317d70, 0x001a1990)?;
                                                                                                                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                                c.g[RA][0] = sx(0x001a199c);
                                                                                                                c.g[A0][0] = c.g[V0][0];
                                                                                                                call_ptr(c, 0x00194a60, 0x001a199c)?;
                                                                                                                let t = true;
                                                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                                if t {
break 'b_1a1ca4;
} else {
break 'b_1a19a4;
}
                                                                                                            }
                                                                                                            c.cycles += 1;
                                                                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                                            break 'b_1a19a8;
                                                                                                        }
                                                                                                        c.cycles += 3;
                                                                                                        c.g[V0][0] = 0x18;
                                                                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                                                                        c.g[V0][0] = 0x9;
                                                                                                        if t {
break 'b_1a1a70;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x27;
if t {
break 'b_1a19ec;
} else {
break 'b_1a19bc;
}
}
                                                                                                    }
                                                                                                    c.cycles += 12;
                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                    c.g[A0][0] = 0x90;
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbc0));
                                                                                                    c.g[A2][0] = 0u64;
                                                                                                    c.g[RA][0] = sx(0x001a19d4);
                                                                                                    c.g[A3][0] = 0u64;
                                                                                                    call_ptr(c, 0x00317d70, 0x001a19d4)?;
                                                                                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                                    c.g[A0][0] = c.g[V0][0];
                                                                                                    c.g[RA][0] = sx(0x001a19e4);
                                                                                                    c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                                                                    call_ptr(c, 0x00181090, 0x001a19e4)?;
                                                                                                    let t = true;
                                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                    if t {
break 'b_1a1ca4;
} else {
break 'b_1a19ec;
}
                                                                                                }
                                                                                                c.cycles += 2;
                                                                                                let t = c.g[V1][0] != c.g[V0][0];
                                                                                                c.g[V0][0] = 0u64;
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1a1ca0;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    let t = true;
                                                                                                    c.g[A1][0] = 0x460000;
                                                                                                    if t {
break 'b_1a1b3c;
} else {
break 'b_1a19fc;
}
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 3;
                                                                                            c.g[V0][0] = 0x1;
                                                                                            let t = c.g[S1][0] == c.g[V0][0];
                                                                                            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
                                                                                            if t {
                                                                                                break 'b_1a1a9c;
                                                                                            } else {
                                                                                                'b_1a1a30: {
                                                                                                    'b_1a1a20: {
                                                                                                        c.cycles += 2;
                                                                                                        let t = c.g[V0][0] == 0u64;
                                                                                                        c.g[V0][0] = 0x2;
                                                                                                        if t {
break 'b_1a1a20;
} else {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_1a1a30;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_1a1ca4;
} else {
break 'b_1a1a20;
}
}
}
                                                                                                    }
                                                                                                    c.cycles += 2;
                                                                                                    let t = c.g[S1][0] == c.g[V0][0];
                                                                                                    c.g[V0][0] = 0u64;
                                                                                                    if t {
break 'b_1a1ad0;
} else {
c.cycles += 2;
let t = true;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_1a1ca4;
} else {
break 'b_1a1a30;
}
}
                                                                                                }
                                                                                                c.cycles += 4;
                                                                                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                                c.g[V0][0] = 0x22;
                                                                                                let t = c.g[V1][0] != c.g[V0][0];
                                                                                                c.g[A1][0] = 0x460000;
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1a1a74;
                                                                                                } else {
                                                                                                    break 'b_1a1a40;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            12;
                                                                                        c.g[A1][0] = 0x460000;
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0x54;
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1198));
                                                                                        c.g[A2]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        c.g[RA][0] = sx(0x001a1a58);
                                                                                        c.g[A3]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        call_ptr(c, 0x00317d70, 0x001a1a58)?;
                                                                                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0];
                                                                                        c.g[RA][0] = sx(0x001a1a68);
                                                                                        c.g[A2]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        call_ptr(c, 0x001887a0, 0x001a1a68)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                        if t {
                                                                                            break 'b_1a1ca4;
                                                                                        } else {
                                                                                            break 'b_1a1a70;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[A1][0] =
                                                                                        0x460000;
                                                                                    break 'b_1a1a74;
                                                                                }
                                                                                c.cycles += 10;
                                                                                c.g[A0][0] = 0x4c;
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xd60));
                                                                                c.g[A2][0] = 0u64;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001a1a88);
                                                                                c.g[A3][0] = 0u64;
                                                                                call_ptr(
                                                                                    c, 0x00317d70,
                                                                                    0x001a1a88,
                                                                                )?;
                                                                                c.g[A1][0] = c
                                                                                    .lw(c.g[S0][0]
                                                                                        as u32);
                                                                                c.g[RA][0] =
                                                                                    sx(0x001a1a94);
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                call_ptr(
                                                                                    c, 0x0018d240,
                                                                                    0x001a1a94,
                                                                                )?;
                                                                                let t = true;
                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                if t {
                                                                                    break 'b_1a1ca4;
                                                                                } else {
                                                                                    break 'b_1a1a9c;
                                                                                }
                                                                            }
                                                                            c.cycles += 13;
                                                                            c.g[A1][0] = 0x460000;
                                                                            c.g[A0][0] = 0x8;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x31a0,
                                                                                    ));
                                                                            c.g[A2][0] = 0x100;
                                                                            c.g[RA][0] =
                                                                                sx(0x001a1ab4);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x001a1ab4,
                                                                            )?;
                                                                            c.g[RA][0] =
                                                                                sx(0x001a1abc);
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, 0x00232720,
                                                                                0x001a1abc,
                                                                            )?;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x001a1ac8);
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, 0x00316ef8,
                                                                                0x001a1ac8,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[V0][0] = 0u64;
                                                                            if t {
                                                                                break 'b_1a1ca0;
                                                                            } else {
                                                                                break 'b_1a1ad0;
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x1680));
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] ^ 0x1;
                                                                        let t = c.g[V0][0] != 0u64;
                                                                        c.g[V0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_1a1ca0;
                                                                        } else {
                                                                            c.cycles += 11;
                                                                            c.g[A1][0] = 0x460000;
                                                                            c.g[A0][0] = 0x6d0;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x11a8,
                                                                                    ));
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001a1af8);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x001a1af8,
                                                                            )?;
                                                                            c.g[A1][0] = c
                                                                                .lw(c.g[S0][0]
                                                                                    as u32);
                                                                            c.g[RA][0] =
                                                                                sx(0x001a1b04);
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            Callee { addr: 0x001c1c50, f: ee::s03::cFEStateNetSSXWorld_ctor }.call(c, 0x001a1b04)?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x30,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1a1ca4;
                                                                            } else {
                                                                                break 'b_1a1b0c;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 11;
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.g[A0][0] = 0x4c;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1d70));
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x001a1b24);
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x001a1b24,
                                                                    )?;
                                                                    c.g[A1][0] =
                                                                        c.lw(c.g[S0][0] as u32);
                                                                    c.g[RA][0] = sx(0x001a1b30);
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    call_ptr(
                                                                        c, 0x00194738, 0x001a1b30,
                                                                    )?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x30),
                                                                    );
                                                                    if t {
                                                                        break 'b_1a1ca4;
                                                                    } else {
                                                                        break 'b_1a1b38;
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[A1][0] = 0x460000;
                                                                break 'b_1a1b3c;
                                                            }
                                                            c.cycles += 10;
                                                            c.g[A0][0] = 0x58;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1188));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001a1b50);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x001a1b50)?;
                                                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                            c.g[RA][0] = sx(0x001a1b5c);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001f3700, 0x001a1b5c)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_1a1ca4;
                                                            } else {
                                                                break 'b_1a1b64;
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1718));
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A1][0] = 0x460000;
                                                        if t {
                                                            break 'b_1a1bb0;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[A1][0] = 0x460000;
                                                            break 'b_1a1b74;
                                                        }
                                                    }
                                                    c.cycles += 11;
                                                    c.g[A0][0] = 0x50;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0xc08));
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001a1b88);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x001a1b88)?;
                                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001a1b98);
                                                    c.g[A2][0] = c
                                                        .lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                                    call_ptr(c, 0x001828c0, 0x001a1b98)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1a1ca4;
                                                    } else {
                                                        break 'b_1a1ba0;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x1718));
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x460000;
                                                if t {
                                                    break 'b_1a1bd8;
                                                } else {
                                                    break 'b_1a1bac;
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = 0x460000;
                                            break 'b_1a1bb0;
                                        }
                                        c.cycles += 10;
                                        c.g[A0][0] = 0x4c;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xdd8));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001a1bc4);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001a1bc4)?;
                                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                        c.g[RA][0] = sx(0x001a1bd0);
                                        c.g[A0][0] = c.g[V0][0];
                                        call_ptr(c, 0x001f3ff8, 0x001a1bd0)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1a1ca4;
                                        } else {
                                            break 'b_1a1bd8;
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[A0][0] = 0x54;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc68));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001a1bec);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001a1bec)?;
                                    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001a1bfc);
                                    c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                    call_ptr(c, 0x00183238, 0x001a1bfc)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1a1ca4;
                                    } else {
                                        break 'b_1a1c04;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0xc8;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xdf8));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x001a1c1c);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001a1c1c)?;
                                c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001a1c2c);
                                c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                                call_ptr(c, 0x001d2678, 0x001a1c2c)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1a1ca4;
                                } else {
                                    break 'b_1a1c34;
                                }
                            }
                            c.cycles += 13;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0xe4;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe28));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x001a1c4c);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001a1c4c)?;
                            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                            c.g[RA][0] = sx(0x001a1c60);
                            c.g[A3][0] = c.g[S1][0];
                            call_ptr(c, 0x001d3780, 0x001a1c60)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1a1ca4;
                            } else {
                                break 'b_1a1c68;
                            }
                        }
                        c.cycles += 13;
                        c.g[A1][0] = 0x460000;
                        c.g[A0][0] = 0x50;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc80));
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x001a1c80);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x001a1c80)?;
                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
                        c.g[RA][0] = sx(0x001a1c94);
                        c.g[A3][0] = (0u64 < c.g[S1][0]) as u64;
                        call_ptr(c, 0x001d4bc8, 0x001a1c94)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1a1ca4;
                        } else {
                            break 'b_1a1c9c;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_1a1ca0;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1a1ca4;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes (word `OC860_MAGIC` at `OC860_CTRL`).
    pub(crate) const OC860_CTRL: u32 = DATA + 0x28_0000;
    pub(crate) const OC860_MAGIC: u32 = 0x0860_f00d;
    pub(crate) const OC860_SELF: u32 = DATA + 0x29_0000;
    pub(crate) const OC860_DESC: u32 = DATA + 0x2a_0000;
    pub(crate) const OC860_WORLD: u32 = DATA + 0x2b_0000;
    /// `cBENewRaceInterface_newRace.cGamePath` (a signed byte).
    pub(crate) const OC860_GAME_PATH: u32 = 0x0053_5c11;

    macro_rules! oc860_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC860_CTRL) != OC860_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc860_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc860_fakes!(
        (
            oc860_fake_1bfbb8,
            OC860_PREV_1BFBB8,
            ee::s03::OVERRIDE_001BFBB8,
            ee::s03::f_001bfbb8__generated,
            0x001bfbb8
        ),
        (
            oc860_fake_181090,
            OC860_PREV_181090,
            ee::s02::OVERRIDE_00181090,
            ee::s02::f_00181090__generated,
            0x00181090
        ),
        (
            oc860_fake_1828c0,
            OC860_PREV_1828C0,
            ee::s02::OVERRIDE_001828C0,
            ee::s02::f_001828c0__generated,
            0x001828c0
        ),
        (
            oc860_fake_194a60,
            OC860_PREV_194A60,
            ee::s02::OVERRIDE_00194A60,
            ee::s02::f_00194a60__generated,
            0x00194a60
        ),
        (
            oc860_fake_1f3700,
            OC860_PREV_1F3700,
            ee::s04::OVERRIDE_001F3700,
            ee::s04::f_001f3700__generated,
            0x001f3700
        ),
        (
            oc860_fake_1f3ff8,
            OC860_PREV_1F3FF8,
            ee::s04::OVERRIDE_001F3FF8,
            ee::s04::f_001f3ff8__generated,
            0x001f3ff8
        ),
        (
            oc860_fake_183238,
            OC860_PREV_183238,
            ee::s02::OVERRIDE_00183238,
            ee::s02::f_00183238__generated,
            0x00183238
        ),
        (
            oc860_fake_186610,
            OC860_PREV_186610,
            ee::s02::OVERRIDE_00186610,
            ee::s02::f_00186610__generated,
            0x00186610
        ),
        (
            oc860_fake_18d240,
            OC860_PREV_18D240,
            ee::s02::OVERRIDE_0018D240,
            ee::s02::f_0018d240__generated,
            0x0018d240
        ),
        (
            oc860_fake_1887a0,
            OC860_PREV_1887A0,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x001887a0
        ),
        (
            oc860_fake_1c1c50,
            OC860_PREV_1C1C50,
            ee::s03::OVERRIDE_001C1C50,
            ee::s03::f_001c1c50__generated,
            0x001c1c50
        ),
        (
            oc860_fake_194738,
            OC860_PREV_194738,
            ee::s02::OVERRIDE_00194738,
            ee::s02::f_00194738__generated,
            0x00194738
        ),
        (
            oc860_fake_1d4bc8,
            OC860_PREV_1D4BC8,
            ee::s04::OVERRIDE_001D4BC8,
            ee::s04::f_001d4bc8__generated,
            0x001d4bc8
        ),
        (
            oc860_fake_317d70,
            OC860_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc860_fake_14dc80,
            OC860_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc860_fake_14dd58,
            OC860_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc860_fake_232720,
            OC860_PREV_232720,
            ee::s06::OVERRIDE_00232720,
            ee::s06::f_00232720__generated,
            0x00232720
        ),
        (
            oc860_fake_316ef8,
            OC860_PREV_316EF8,
            ee::s10::OVERRIDE_00316EF8,
            ee::s10::f_00316ef8__generated,
            0x00316ef8
        ),
    );

    /// `FEUICustom_v05` picks the next front-end state object by flow state
    /// (jump table over states 2..=0x2f, then a few sub-switches on the game path,
    /// `gInLodge`, the mode flag, the user number and the state data). Every callee
    /// (allocator, state constructors, back-end getters) is a logged fake. States 17,
    /// 44 and 45 (AudioOptions, MountainRoom, PeakRoom constructors, which have no
    /// override slot to fake) are not reached; the others, and the out-of-range states
    /// 0, 1, 0x30 and 0x31, are, with every sub-switch value.
    #[test]
    fn feui_custom_v05_matches() {
        oc860_install_fakes();
        check(
            ee::s03::f_001a16c0__generated,
            feui_custom_v05,
            1200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_16c0, 4);
                c.bus.write32(OC860_CTRL, OC860_MAGIC);
                c.pc = 0x001a_16c0;
                let mut state = seed % 0x32;
                if matches!(state, 17 | 44 | 45) {
                    state = 0x30;
                }
                let sub = seed / 0x32;
                c.g[A0][0] = sx(OC860_SELF);
                c.g[A1][0] = sx(OC860_DESC);
                c.g[A2][0] = sx([0u32, 1, 2, 3, 0xffff_ffff, r.next_u32()][(sub % 6) as usize]);
                fill(c, &mut r, OC860_SELF, 8);
                fill(c, &mut r, OC860_DESC, 0x20);
                c.bus.write32(OC860_DESC + 0xc, state);
                c.bus.write8(
                    OC860_DESC + 0x44,
                    [0u8, 1, 2, 0xff][(r.next_u32() % 4) as usize],
                );
                // The creating object's own state code (the sub-switch of states 8, 33, 35).
                let own = [0x18u32, 9, 0x27, 0x22, 0x21, r.next_u32()][(r.next_u32() % 6) as usize];
                c.bus.write32(OC860_SELF + 0xc, own);
                c.bus.write8(
                    OC860_GAME_PATH,
                    [0xffu8, 0, 1, 2, 3, 5][((sub / 6 + seed) % 6) as usize],
                );
                // gp-0x1680 (online mode flag) and gp-0x1718 (gInLodge).
                c.bus.write32(GAME_GP - 0x1680, r.next_u32() % 2);
                c.bus.write32(GAME_GP - 0x1718, r.next_u32() % 2);
                c.bus.write32(GAME_GP - 0x848, OC860_WORLD);
                fill(c, &mut r, OC860_WORLD, 0x20);
            },
        );
    }
}
