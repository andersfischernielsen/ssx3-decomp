//! Staging module oc932 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc932`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::back_edge_unwinds;
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void cFEPopupConfirm_SetFade(cFEPopupConfirm* self, float t)` (0x1c5f20): set the alpha of the popup's text and background widgets (`+0x70/+0x74/+0x78/+0x7c/+0x88`, the three widgets looked up by hash name, and the five option widgets at `+0x90`) to `t`: each non-null widget gets its colour block (16 bytes at widget+0x1c) copied with `t` as the first float and its vtable slot 0x58 called with it.
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_set_fade(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c5f80: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x70));
            let t = c.g[A2][0] == 0u64;
            c.mov_s(20, 12);
            if t {
                break 'b_1c5f80;
            } else {
                c.cycles += 15;
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c5f80);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c5f80)?;
                break 'b_1c5f80;
            }
        }
        'b_1c5fc4: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[SP][0];
            if t {
                break 'b_1c5fc4;
            } else {
                c.cycles += 14;
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(7, (c.g[SP][0] as u32));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c5fc4);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c5fc4)?;
                break 'b_1c5fc4;
            }
        }
        'b_1c6010: {
            c.cycles += 3;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
            if c.g[A2][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c6010;
            } else {
                c.cycles += 16;
                c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[SP][0] as u32));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c600c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c600c)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                break 'b_1c6010;
            }
        }
        'b_1c6058: {
            c.cycles += 2;
            if c.g[A2][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c6058;
            } else {
                c.cycles += 16;
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(4, (c.g[SP][0] as u32));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c6054);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c6054)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
                break 'b_1c6058;
            }
        }
        'b_1c60a0: {
            c.cycles += 2;
            if c.g[A2][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c60a0;
            } else {
                c.cycles += 16;
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(3, (c.g[SP][0] as u32));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c609c);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c609c)?;
                c.g[A0][0] = 0x4a0000;
                break 'b_1c60a0;
            }
        }
        'b_1c6100: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x001c60a8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c50));
            call_ptr(c, 0x00317670, 0x001c60a8)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c60b4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c60b4)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c6100;
            } else {
                c.cycles += 16;
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(2, (c.g[SP][0] as u32));
                c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c60fc);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c60fc)?;
                c.g[A0][0] = 0x4a0000;
                break 'b_1c6100;
            }
        }
        'b_1c6158: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x001c6108);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c58));
            call_ptr(c, 0x00317670, 0x001c6108)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6114);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6114)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[SP][0];
            if t {
                break 'b_1c6158;
            } else {
                c.cycles += 14;
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(2, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(7, (c.g[SP][0] as u32));
                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c6158);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c6158)?;
                break 'b_1c6158;
            }
        }
        'b_1c61bc: {
            c.cycles += 9;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001c6164);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15e0));
            call_ptr(c, 0x00317670, 0x001c6164)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6170);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6170)?;
            c.g[A2][0] = c.g[V0][0];
            if c.g[A2][0] == 0u64 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
                break 'b_1c61bc;
            } else {
                c.cycles += 16;
                c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x23));
                c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x1c));
                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x2b));
                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x24));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[SP][0] as u32));
                c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32));
                c.g[A1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c61b8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c61b8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
                break 'b_1c61bc;
            }
        }
        c.cycles += 3;
        c.g[S1][0] = 0x4;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
        'l_1c61c8: loop {
            'b_1c6210: {
                c.cycles += 4;
                c.g[A1][0] = c.g[SP][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                let t = c.g[A2][0] == 0u64;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                if t {
                    break 'b_1c6210;
                } else {
                    c.cycles += 14;
                    c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x23));
                    c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x1c));
                    c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x2b));
                    c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x24));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(4, (c.g[SP][0] as u32));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(20, (c.g[SP][0] as u32));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6210);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6210)?;
                    break 'b_1c6210;
                }
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) >= 0 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                if event_due(c) {
                    deliver_event_here(c, 0x001c61c8);
                }
                continue 'l_1c61c8;
            } else {
                c.cycles += 6;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_FindWidgets(cFEPopupConfirm* self)` (0x1c6230): look up the popup screen's widgets by hash name (`UIScreen_FindObject(self->screen +0x40, StrHash(name))`), keep their pointers in `+0x48..+0x8c`, copy the positions into `+0x188..` and get the rects of three of them; the five option widgets are found by `sprintf(name, fmt, i)` into `+0x90..`.
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_find_widgets(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c62a8: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x4a0000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x001c6258);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c60));
            call_ptr(c, 0x00317670, 0x001c6258)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6264);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6264)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c62a8;
            } else {
                c.cycles += 14;
                c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x18f));
                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x188));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x190), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c62a0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c62a0)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[S1][0]);
                c.g[A0][0] = 0x4a0000;
                break 'b_1c62a8;
            }
        }
        'b_1c62e4: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c62b0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c68));
            call_ptr(c, 0x00317670, 0x001c62b0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c62bc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c62bc)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c62e4;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x19b));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x194));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x19c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c62e4;
            }
        }
        'b_1c6300: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c62ec);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fa8));
            call_ptr(c, 0x00317670, 0x001c62ec)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c62f8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c62f8)?;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                break 'b_1c6300;
            } else {
                break 'b_1c6300;
            }
        }
        'b_1c6340: {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c630c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fb8));
            call_ptr(c, 0x00317670, 0x001c630c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6318);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6318)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6340;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x237));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x230));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x238), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6340;
            }
        }
        'b_1c637c: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6348);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fc8));
            call_ptr(c, 0x00317670, 0x001c6348)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6354);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6354)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c637c;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x22b));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x224));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x22c), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c637c;
            }
        }
        'b_1c63b8: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6384);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fd8));
            call_ptr(c, 0x00317670, 0x001c6384)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6390);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6390)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c63b8;
            } else {
                c.cycles += 8;
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x24f));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x248));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x250), c.g[V1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c63b8;
            }
        }
        'b_1c6408: {
            c.cycles += 8;
            c.g[RA][0] = sx(0x001c63c0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5fe8));
            call_ptr(c, 0x00317670, 0x001c63c0)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c63cc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c63cc)?;
            c.g[S1][0] = c.g[V0][0];
            let t = c.g[S1][0] == 0u64;
            c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x218));
            if t {
                break 'b_1c6408;
            } else {
                c.cycles += 12;
                c.ldl(6, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.ldr(6, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x243));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x23c));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x244), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c6404);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c6404)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x58), c.g[S1][0]);
                break 'b_1c6408;
            }
        }
        'b_1c6448: {
            c.cycles += 8;
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x001c6414);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c70));
            call_ptr(c, 0x00317670, 0x001c6414)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6420);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6420)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6448;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x25b));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x254));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x25c), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6448;
            }
        }
        'b_1c6464: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6450);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5ff8));
            call_ptr(c, 0x00317670, 0x001c6450)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c645c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c645c)?;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_1c6464;
            } else {
                break 'b_1c6464;
            }
        }
        'b_1c64a4: {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c6470);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2470));
            call_ptr(c, 0x00317670, 0x001c6470)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c647c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c647c)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x4a0000;
                break 'b_1c64a4;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1a7));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1a0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
                c.g[A0][0] = 0x4a0000;
                break 'b_1c64a4;
            }
        }
        'b_1c64e0: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c64ac);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c78));
            call_ptr(c, 0x00317670, 0x001c64ac)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c64b8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c64b8)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c64e0;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1b3));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1ac));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1b4), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c64e0;
            }
        }
        'b_1c651c: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c64e8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6008));
            call_ptr(c, 0x00317670, 0x001c64e8)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c64f4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c64f4)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c651c;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1cb));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1c4));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1cc), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c651c;
            }
        }
        'b_1c6558: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6524);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6018));
            call_ptr(c, 0x00317670, 0x001c6524)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6530);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6530)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6558;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1bf));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1b8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x7c), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6558;
            }
        }
        'b_1c6594: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6560);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6028));
            call_ptr(c, 0x00317670, 0x001c6560)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c656c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c656c)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c6594;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1d7));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1d0));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1d8), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x78), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c6594;
            }
        }
        'b_1c65d0: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c659c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6038));
            call_ptr(c, 0x00317670, 0x001c659c)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c65a8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c65a8)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c65d0;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1e4), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c65d0;
            }
        }
        'b_1c660c: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c65d8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6048));
            call_ptr(c, 0x00317670, 0x001c65d8)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c65e4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c65e4)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1c660c;
            } else {
                c.cycles += 8;
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1ef));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x1e8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1f0), c.g[A2][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x88), c.g[V0][0]);
                c.g[A0][0] = 0x460000;
                break 'b_1c660c;
            }
        }
        'b_1c6628: {
            c.cycles += 7;
            c.g[RA][0] = sx(0x001c6614);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6058));
            call_ptr(c, 0x00317670, 0x001c6614)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6620);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6620)?;
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                break 'b_1c6628;
            } else {
                break 'b_1c6628;
            }
        }
        'b_1c6684: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001c6634);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6070));
            call_ptr(c, 0x00317670, 0x001c6634)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001c6640);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001c6640)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] == 0u64 {
                c.g[S2][0] = 0u64;
                break 'b_1c6684;
            } else {
                c.cycles += 14;
                c.ldl(5, (c.g[S1][0] as u32).wrapping_add(0x4b));
                c.ldr(5, (c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x207));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x200));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x208), c.g[A2][0]);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20c));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001c667c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001c667c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8c), c.g[S1][0]);
                c.g[S2][0] = 0u64;
                break 'b_1c6684;
            }
        }
        c.cycles += 3;
        c.g[S3][0] = 0x460000;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x90));
        'l_1c6690: loop {
            'b_1c66c4: {
                c.cycles += 12;
                c.g[A0][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6080));
                c.g[RA][0] = sx(0x001c66a4);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x00417828, 0x001c66a4)?;
                c.g[RA][0] = sx(0x001c66ac);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001c66ac)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001c66b8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001c66b8)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                if t {
                    break 'b_1c66c4;
                } else {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                    break 'b_1c66c4;
                }
            }
            c.cycles += 2;
            let t = c.g[V1][0] != 0u64;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001c6690);
                }
                continue 'l_1c6690;
            } else {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void cFEPopupConfirm_ApplyBackgroundStyle(cFEPopupConfirm* self, int style)` (0x1c66e8): show/hide the background widgets for the popup style (0: online/white, 1: blue, 2: black; stored at `+0x31c`) and recolour the four text widgets (`+0x74/+0x78/+0x7c/+0x88`).
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_apply_background_style(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c6af4: {
            'b_1c69b8: {
                'b_1c6874: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = 0x1;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x31c));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                    if t {
                        break 'b_1c6874;
                    } else {
                        'b_1c6734: {
                            'b_1c6724: {
                                c.cycles += 2;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V0][0] = 0x2;
                                    break 'b_1c6724;
                                } else {
                                    c.cycles += 2;
                                    if c.g[V1][0] == 0u64 {
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                                        break 'b_1c6734;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1c6af4;
                                        } else {
                                            break 'b_1c6724;
                                        }
                                    }
                                }
                            }
                            c.cycles += 2;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                                break 'b_1c69b8;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1c6af4;
                                } else {
                                    break 'b_1c6734;
                                }
                            }
                        }
                        'b_1c67c0: {
                            c.cycles += 30;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001c6740);
                            c.g[S1][0] = 0x4c0000;
                            Callee {
                                addr: 0x001ca4c0,
                                f: ee::s03::cFEPopupConfirm_ShowWidget,
                            }
                            .call(c, 0x001c6740)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                            c.g[RA][0] = sx(0x001c674c);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca4c0,
                                f: ee::s03::cFEPopupConfirm_ShowWidget,
                            }
                            .call(c, 0x001c674c)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                            c.g[RA][0] = sx(0x001c6758);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca4c0,
                                f: ee::s03::cFEPopupConfirm_ShowWidget,
                            }
                            .call(c, 0x001c6758)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                            c.g[RA][0] = sx(0x001c6764);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6764)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                            c.g[RA][0] = sx(0x001c6770);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6770)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[RA][0] = sx(0x001c677c);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c677c)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[RA][0] = sx(0x001c6788);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6788)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                            c.g[RA][0] = sx(0x001c6794);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c6794)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                            c.g[RA][0] = sx(0x001c67a0);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001ca488,
                                f: ee::s03::cFEPopupConfirm_HideWidget,
                            }
                            .call(c, 0x001c67a0)?;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c67c0;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c67c0);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c67c0)?;
                                break 'b_1c67c0;
                            }
                        }
                        'b_1c67e0: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c67e0;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c67e0);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c67e0)?;
                                break 'b_1c67e0;
                            }
                        }
                        'b_1c6800: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c6800;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c6800);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c6800)?;
                                break 'b_1c6800;
                            }
                        }
                        'b_1c6820: {
                            c.cycles += 3;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                            if t {
                                break 'b_1c6820;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c6820);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c6820)?;
                                break 'b_1c6820;
                            }
                        }
                        c.cycles += 21;
                        c.g[V0][0] = 0x4c0000;
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6788));
                        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(3, (c.g[A2][0] as u32));
                        c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x333));
                        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x32c));
                        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x33b));
                        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x334));
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                        c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.ldr(2, (c.g[A1][0] as u32));
                        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x343));
                        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x33c));
                        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x34b));
                        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x344));
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1c6af4;
                        } else {
                            break 'b_1c6874;
                        }
                    }
                }
                'b_1c6904: {
                    c.cycles += 31;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001c6884);
                    c.g[S1][0] = 0x4c0000;
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c6884)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x001c6890);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c6890)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                    c.g[RA][0] = sx(0x001c689c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c689c)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[RA][0] = sx(0x001c68a8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c68a8)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x001c68b4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c68b4)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                    c.g[RA][0] = sx(0x001c68c0);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca4c0,
                        f: ee::s03::cFEPopupConfirm_ShowWidget,
                    }
                    .call(c, 0x001c68c0)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x001c68cc);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca488,
                        f: ee::s03::cFEPopupConfirm_HideWidget,
                    }
                    .call(c, 0x001c68cc)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                    c.g[RA][0] = sx(0x001c68d8);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca488,
                        f: ee::s03::cFEPopupConfirm_HideWidget,
                    }
                    .call(c, 0x001c68d8)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                    c.g[RA][0] = sx(0x001c68e4);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001ca488,
                        f: ee::s03::cFEPopupConfirm_HideWidget,
                    }
                    .call(c, 0x001c68e4)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6904;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6904);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001c6904)?;
                        break 'b_1c6904;
                    }
                }
                'b_1c6924: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6924;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6924);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001c6924)?;
                        break 'b_1c6924;
                    }
                }
                'b_1c6944: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6944;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6944);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001c6944)?;
                        break 'b_1c6944;
                    }
                }
                'b_1c6964: {
                    c.cycles += 3;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                    let t = c.g[A2][0] == 0u64;
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                    if t {
                        break 'b_1c6964;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001c6964);
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001c6964)?;
                        break 'b_1c6964;
                    }
                }
                c.cycles += 21;
                c.g[V0][0] = 0x4c0000;
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6788));
                c.ldl(6, (c.g[A1][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[A1][0] as u32));
                c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x333));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x32c));
                c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x33b));
                c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x334));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6798));
                c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0x7));
                c.ldr(6, (c.g[A0][0] as u32));
                c.ldl(2, (c.g[A0][0] as u32).wrapping_add(0xf));
                c.ldr(2, (c.g[A0][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x343));
                c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x33c));
                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x34b));
                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x344));
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1c6af4;
                } else {
                    break 'b_1c69b8;
                }
            }
            'b_1c6a44: {
                c.cycles += 30;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001c69c4);
                c.g[S1][0] = 0x4c0000;
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69c4)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
                c.g[RA][0] = sx(0x001c69d0);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69d0)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                c.g[RA][0] = sx(0x001c69dc);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69dc)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x001c69e8);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69e8)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
                c.g[RA][0] = sx(0x001c69f4);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c69f4)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[RA][0] = sx(0x001c6a00);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c6a00)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
                c.g[RA][0] = sx(0x001c6a0c);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c6a0c)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                c.g[RA][0] = sx(0x001c6a18);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca488,
                    f: ee::s03::cFEPopupConfirm_HideWidget,
                }
                .call(c, 0x001c6a18)?;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
                c.g[RA][0] = sx(0x001c6a24);
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001ca4c0,
                    f: ee::s03::cFEPopupConfirm_ShowWidget,
                }
                .call(c, 0x001c6a24)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6a44;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6a44);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6a44)?;
                    break 'b_1c6a44;
                }
            }
            'b_1c6a64: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6a64;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6a64);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6a64)?;
                    break 'b_1c6a64;
                }
            }
            'b_1c6a84: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6a84;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6a84);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6a84)?;
                    break 'b_1c6a84;
                }
            }
            'b_1c6aa4: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
                if t {
                    break 'b_1c6aa4;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6aa4);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6aa4)?;
                    break 'b_1c6aa4;
                }
            }
            c.cycles += 20;
            c.g[V0][0] = 0x4c0000;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x67a8));
            c.ldl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
            c.ldr(5, (c.g[A0][0] as u32));
            c.ldl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x333));
            c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x32c));
            c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x33b));
            c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x334));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6788));
            c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(5, (c.g[V1][0] as u32));
            c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x343));
            c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x33c));
            c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x34b));
            c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x344));
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1c6af4;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEPopupConfirm_ApplyConfig(cFEPopupConfirm* self, Config* cfg)` (0x1c6b08): find the widgets, copy the configuration (fields, title/body/option text slots, size and fade) into the popup, lay out title, body, options, menu, buttons and post text, apply the background style and frame, and lay out the text entry box when the config has no selection (`cfg[0] == -1`).
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_apply_config(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c6c04: {
            'b_1c6ba0: {
                c.cycles += 27;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x001c6b2c);
                c.g[S3][0] = c.g[A1][0];
                Callee {
                    addr: 0x001c6230,
                    f: ee::s03::cFEPopupConfirm_FindWidgets,
                }
                .call(c, 0x001c6b2c)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x94));
                c.mtc1(0, 1);
                c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x98));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2c0), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x310), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x30c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xb8));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2c8), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xbc));
                c.sw((c.g[S2][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
                c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0xb0));
                c.c_eq_s(0, 1);
                let t = c.fcc();
                c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x318));
                if t {
                    break 'b_1c6ba0;
                } else {
                    c.cycles += 11;
                    c.swc1(0, (c.g[SP][0] as u32));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(2, (c.g[SP][0] as u32));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x267));
                    c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x260));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x268), c.g[V1][0]);
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_1c6c04;
                    } else {
                        break 'b_1c6ba0;
                    }
                }
            }
            'b_1c6bd8: {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                if t {
                    break 'b_1c6bd8;
                } else {
                    c.cycles += 12;
                    c.g[AT][0] = 0x43ec0000;
                    c.mtc1(1, 0);
                    c.swc1(0, (c.g[SP][0] as u32));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(6, (c.g[SP][0] as u32));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x267));
                    c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0x260));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x268), c.g[A3][0]);
                    let t = true;
                    c.g[S1][0] = 0u64;
                    if t {
                        break 'b_1c6c04;
                    } else {
                        break 'b_1c6bd8;
                    }
                }
            }
            c.cycles += 11;
            c.g[AT][0] = 0x43960000;
            c.mtc1(1, 0);
            c.swc1(0, (c.g[SP][0] as u32));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
            c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(4, (c.g[SP][0] as u32));
            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
            c.sdl(4, (c.g[S2][0] as u32).wrapping_add(0x267));
            c.sdr(4, (c.g[S2][0] as u32).wrapping_add(0x260));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x268), c.g[A1][0]);
            c.g[S1][0] = 0u64;
            break 'b_1c6c04;
        }
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2d0));
        'l_1c6c08: loop {
            c.cycles += 13;
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001c6c14);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x001c58d0,
                f: ee::s03::cFEPopupConfirm_CfgGetOptionConfirm,
            }
            .call(c, 0x001c6c14)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
            c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x7));
            c.sdr(3, (c.g[S0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001c6c08);
                }
                continue 'l_1c6c08;
            } else {
                'b_1c6cc0: {
                    c.cycles += 31;
                    c.ldl(7, (c.g[S3][0] as u32).wrapping_add(0xa3));
                    c.ldr(7, (c.g[S3][0] as u32).wrapping_add(0x9c));
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa4));
                    c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.sdr(7, (c.g[SP][0] as u32));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A1][0] = c.g[S3][0];
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                    c.ldr(6, (c.g[SP][0] as u32));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.sdl(6, (c.g[S2][0] as u32).wrapping_add(0x273));
                    c.sdr(6, (c.g[S2][0] as u32).wrapping_add(0x26c));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x274), c.g[A3][0]);
                    c.g[RA][0] = sx(0x001c6c78);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c8678,
                        f: ee::s03::cFEPopupConfirm_LayoutTitle,
                    }
                    .call(c, 0x001c6c78)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001c6c84);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x001c7258,
                        f: ee::s03::cFEPopupConfirm_LayoutBodyText,
                    }
                    .call(c, 0x001c6c84)?;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001c6c90);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c7388,
                        f: ee::s03::cFEPopupConfirm_LayoutOptions,
                    }
                    .call(c, 0x001c6c90)?;
                    c.g[RA][0] = sx(0x001c6c98);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c8758,
                        f: ee::s03::cFEPopupConfirm_LayoutMenu,
                    }
                    .call(c, 0x001c6c98)?;
                    c.g[RA][0] = sx(0x001c6ca0);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c88c8,
                        f: ee::s03::cFEPopupConfirm_PositionButtons,
                    }
                    .call(c, 0x001c6ca0)?;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001c6cac);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001c7040,
                        f: ee::s03::cFEPopupConfirm_LayoutPostText,
                    }
                    .call(c, 0x001c6cac)?;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x90));
                    let t = c.g[A1][0] == 0u64;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x314), c.g[A1][0]);
                    if t {
                        break 'b_1c6cc0;
                    } else {
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001c6cc0);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001c9cd0,
                            f: ee::s03::cFEPopupConfirm_AdjustDrawLayer,
                        }
                        .call(c, 0x001c6cc0)?;
                        break 'b_1c6cc0;
                    }
                }
                'b_1c6d18: {
                    c.cycles += 15;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc0));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001c6cd4);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x31c), c.g[V0][0]);
                    Callee {
                        addr: 0x001c66e8,
                        f: ee::s03::cFEPopupConfirm_ApplyBackgroundStyle,
                    }
                    .call(c, 0x001c6cd4)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xac));
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x2c4), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc4));
                    c.g[RA][0] = sx(0x001c6cec);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x328), c.g[V0][0]);
                    Callee {
                        addr: 0x001c7c20,
                        f: ee::s03::cFEPopupConfirm_LayoutFrame,
                    }
                    .call(c, 0x001c6cec)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32));
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        break 'b_1c6d18;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001c6d04);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001c7eb0,
                            f: ee::s03::cFEPopupConfirm_LayoutTextEntryBox,
                        }
                        .call(c, 0x001c6d04)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x80));
                        c.g[RA][0] = sx(0x001c6d14);
                        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2b0));
                        Callee {
                            addr: 0x001ca6a0,
                            f: ee::s03::cFEPopupConfirm_CheckAndFixTextEntry,
                        }
                        .call(c, 0x001c6d14)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1c6d18;
                    }
                }
                c.cycles += 6;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `int cFEPopupConfirm_LayoutTextEntry(self, info)` (0x1c6d30): lay out the text-entry widgets (`+0x80/+0x84/+0x88/+0x8c`) on the first call (`info[0] == -1`, then set to 1, returns 1), else hide them again (returns 0).
/// Converted from the generated body: every register, cycle and call kept.
pub fn c_fe_popup_confirm_layout_text_entry(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c7024: {
            'b_1c6f28: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = 0xffffffffffffffff;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S2][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S1][0] = c.g[A0][0];
                if t {
                    break 'b_1c6f28;
                } else {
                    'b_1c6e10: {
                        c.cycles += 12;
                        c.g[V0][0] = 0x500000;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                        c.mtc1(0, 20);
                        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
                        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(3, (c.g[A3][0] as u32));
                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                        let t = c.g[A1][0] == 0u64;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                        if t {
                            break 'b_1c6e10;
                        } else {
                            c.cycles += 33;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001c6d98);
                            c.g[A2][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001c8050,
                                f: ee::s03::cFEPopupConfirm_SetTextFromDesc,
                            }
                            .call(c, 0x001c6d98)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A2][0] = c.g[SP][0];
                            c.g[RA][0] = sx(0x001c6da8);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001c8628,
                                f: ee::s03::cFEPopupConfirm_GetWidgetSize,
                            }
                            .call(c, 0x001c6da8)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[A2][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001c6dbc);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001c8320,
                                f: ee::s03::cFEPopupConfirm_MeasureDesc,
                            }
                            .call(c, 0x001c6dbc)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A1][0] = 0u64;
                            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x28));
                            c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x4b));
                            c.ldr(8, (c.g[A0][0] as u32).wrapping_add(0x44));
                            c.g[T1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
                            c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
                            c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x001c6df4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            call_ptr(c, t, 0x001c6df4)?;
                            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6e10);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c6e10)?;
                            break 'b_1c6e10;
                        }
                    }
                    'b_1c6ebc: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b0));
                        if t {
                            break 'b_1c6ebc;
                        } else {
                            c.cycles += 40;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001c6e28);
                            c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1c));
                            Callee {
                                addr: 0x001c80e8,
                                f: ee::s03::cFEPopupConfirm_DescToString,
                            }
                            .call(c, 0x001c6e28)?;
                            c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
                            c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
                            c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                            c.g[AT][0] = 0x41200000;
                            c.mtc1(1, 1);
                            c.g[A0][0] = c.g[S1][0];
                            c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[A2][0] = c.g[SP][0];
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.add_s(0, 0, 20);
                            c.add_s(0, 0, 1);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x27));
                            c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                            c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[T1][0]);
                            c.g[RA][0] = sx(0x001c6e84);
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x260));
                            Callee {
                                addr: 0x001c9b68,
                                f: ee::s03::cFEPopupConfirm_AccumulateSize,
                            }
                            .call(c, 0x001c6e84)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6ea0);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c6ea0)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6ebc);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c6ebc)?;
                            break 'b_1c6ebc;
                        }
                    }
                    'b_1c6ef8: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_1c6ef8;
                        } else {
                            c.cycles += 12;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6edc);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c6edc)?;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                            c.g[A1][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6ef8);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c6ef8)?;
                            break 'b_1c6ef8;
                        }
                    }
                    'b_1c6f18: {
                        c.cycles += 3;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8c));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1c6f18;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001c6f18);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001c6f18)?;
                            break 'b_1c6f18;
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[S2][0] as u32), c.g[V1][0]);
                    if t {
                        break 'b_1c7024;
                    } else {
                        break 'b_1c6f28;
                    }
                }
            }
            'b_1c6f64: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1c6f64;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6f48);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6f48)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x88));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6f64);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6f64)?;
                    break 'b_1c6f64;
                }
            }
            'b_1c6fa0: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1c6fa0;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6f84);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6f84)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6fa0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6fa0)?;
                    break 'b_1c6fa0;
                }
            }
            'b_1c6fdc: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                let t = c.g[A2][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1c6fdc;
                } else {
                    c.cycles += 12;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6fc0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6fc0)?;
                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c6fdc);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c6fdc)?;
                    break 'b_1c6fdc;
                }
            }
            'b_1c7004: {
                c.cycles += 3;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8c));
                if c.g[A2][0] == 0u64 {
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    break 'b_1c7004;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001c7000);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001c7000)?;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                    break 'b_1c7004;
                }
            }
            c.cycles += 8;
            c.g[A2][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001c7010);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x001c8628,
                f: ee::s03::cFEPopupConfirm_GetWidgetSize,
            }
            .call(c, 0x001c7010)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001c7020);
            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x260));
            Callee {
                addr: 0x001c9b68,
                f: ee::s03::cFEPopupConfirm_AccumulateSize,
            }
            .call(c, 0x001c7020)?;
            c.g[V0][0] = 0u64;
            break 'b_1c7024;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_5f20,
        c_fe_popup_confirm_set_fade,
        "cFEPopupConfirm_SetFade",
    ),
    (
        0x001c_6230,
        c_fe_popup_confirm_find_widgets,
        "cFEPopupConfirm_FindWidgets",
    ),
    (
        0x001c_66e8,
        c_fe_popup_confirm_apply_background_style,
        "cFEPopupConfirm_ApplyBackgroundStyle",
    ),
    (
        0x001c_6b08,
        c_fe_popup_confirm_apply_config,
        "cFEPopupConfirm_ApplyConfig",
    ),
    (
        0x001c_6d30,
        c_fe_popup_confirm_layout_text_entry,
        "cFEPopupConfirm_LayoutTextEntry",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (in the machine's RAM) and its magic;
    /// the mask word after it says which `UIScreen_FindObject` calls find a widget.
    const OC932_CTRL: u32 = DATA + 0x20_4000;
    const OC932_MAGIC: u32 = 0x0932_f00d;
    /// The popup object, its widgets, the shared widget vtable, the config/info block.
    const OC932_SELF: u32 = DATA + 0x20_0000;
    const OC932_WID: u32 = DATA + 0x20_1000;
    const OC932_VT: u32 = DATA + 0x20_2000;
    const OC932_CFG: u32 = DATA + 0x20_3000;

    /// A gated fake callee at `addr`: logs the call like `fakes::fake_call`;
    /// `UIScreen_FindObject` (0x39d860) then answers a widget or null from
    /// the mask, by the index of the call.
    fn oc932_fake_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if addr == 0x0039_d860 && c.pc == ra {
            let mask = c.bus.read32(OC932_CTRL + 4);
            c.g[V0][0] = if (mask >> (n % 32)) & 1 != 0 {
                oc932_widget(n) as u64
            } else {
                0
            };
        }
    }

    fn oc932_widget(i: u32) -> u32 {
        OC932_WID + 0x100 * (i % 8)
    }

    macro_rules! oc932_fakes {
        ($($fake:ident, $prev:ident, $slot:path, $gen:path, $addr:literal;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC932_CTRL) != OC932_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc932_fake_call(c, $addr);
                }
            )*
            fn oc932_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc932_fakes! {
        oc932_f_317670, OC932_P_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670;
        oc932_f_39d860, OC932_P_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860;
        oc932_f_417828, OC932_P_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828;
        oc932_f_1ca488, OC932_P_1CA488, ee::s03::OVERRIDE_001CA488, ee::s03::f_001ca488__generated, 0x001c_a488;
        oc932_f_1ca4c0, OC932_P_1CA4C0, ee::s03::OVERRIDE_001CA4C0, ee::s03::f_001ca4c0__generated, 0x001c_a4c0;
        oc932_f_1c6230, OC932_P_1C6230, ee::s03::OVERRIDE_001C6230, ee::s03::f_001c6230__generated, 0x001c_6230;
        oc932_f_1c66e8, OC932_P_1C66E8, ee::s03::OVERRIDE_001C66E8, ee::s03::f_001c66e8__generated, 0x001c_66e8;
        oc932_f_1c58d0, OC932_P_1C58D0, ee::s03::OVERRIDE_001C58D0, ee::s03::f_001c58d0__generated, 0x001c_58d0;
        oc932_f_1c8678, OC932_P_1C8678, ee::s03::OVERRIDE_001C8678, ee::s03::f_001c8678__generated, 0x001c_8678;
        oc932_f_1c7258, OC932_P_1C7258, ee::s03::OVERRIDE_001C7258, ee::s03::f_001c7258__generated, 0x001c_7258;
        oc932_f_1c7388, OC932_P_1C7388, ee::s03::OVERRIDE_001C7388, ee::s03::f_001c7388__generated, 0x001c_7388;
        oc932_f_1c8758, OC932_P_1C8758, ee::s03::OVERRIDE_001C8758, ee::s03::f_001c8758__generated, 0x001c_8758;
        oc932_f_1c88c8, OC932_P_1C88C8, ee::s03::OVERRIDE_001C88C8, ee::s03::f_001c88c8__generated, 0x001c_88c8;
        oc932_f_1c7040, OC932_P_1C7040, ee::s03::OVERRIDE_001C7040, ee::s03::f_001c7040__generated, 0x001c_7040;
        oc932_f_1c9cd0, OC932_P_1C9CD0, ee::s03::OVERRIDE_001C9CD0, ee::s03::f_001c9cd0__generated, 0x001c_9cd0;
        oc932_f_1c7c20, OC932_P_1C7C20, ee::s03::OVERRIDE_001C7C20, ee::s03::f_001c7c20__generated, 0x001c_7c20;
        oc932_f_1c7eb0, OC932_P_1C7EB0, ee::s03::OVERRIDE_001C7EB0, ee::s03::f_001c7eb0__generated, 0x001c_7eb0;
        oc932_f_1ca6a0, OC932_P_1CA6A0, ee::s03::OVERRIDE_001CA6A0, ee::s03::f_001ca6a0__generated, 0x001c_a6a0;
        oc932_f_1c8050, OC932_P_1C8050, ee::s03::OVERRIDE_001C8050, ee::s03::f_001c8050__generated, 0x001c_8050;
        oc932_f_1c80e8, OC932_P_1C80E8, ee::s03::OVERRIDE_001C80E8, ee::s03::f_001c80e8__generated, 0x001c_80e8;
        oc932_f_1c8320, OC932_P_1C8320, ee::s03::OVERRIDE_001C8320, ee::s03::f_001c8320__generated, 0x001c_8320;
        oc932_f_1c8628, OC932_P_1C8628, ee::s03::OVERRIDE_001C8628, ee::s03::f_001c8628__generated, 0x001c_8628;
        oc932_f_1c9b68, OC932_P_1C9B68, ee::s03::OVERRIDE_001C9B68, ee::s03::f_001c9b68__generated, 0x001c_9b68;
    }

    /// The popup world: scrambled machine with the fakes' gate open, eight
    /// widgets (random colour/position data, one shared vtable whose slots
    /// are all fake calls), and a popup object of random words. `mask`
    /// (seed-driven: all, none or random bits) says which widget fields of
    /// the popup and which `FindObject` calls are non-null.
    fn oc932_world(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> (Rng, u32) {
        oc932_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        let mask = match seed % 4 {
            0 => !0,
            1 => 0,
            _ => r.next_u32(),
        };
        c.bus.write32(OC932_CTRL, OC932_MAGIC);
        c.bus.write32(OC932_CTRL + 4, mask);
        for i in 0..0x40u32 {
            c.bus
                .write16(OC932_VT + 8 * i, (0x10 * (r.next_u32() % 3)) as u16);
            c.bus.write32(OC932_VT + 8 * i + 4, FAKE + 0x10 * (i % 4));
        }
        for k in 0..8u32 {
            let w = oc932_widget(k);
            fill(c, &mut r, w, 0x40);
            c.bus.write32(w + 8, OC932_VT);
        }
        fill(c, &mut r, OC932_SELF, 0x100);
        // Globals the popup code copies (colours, default position).
        fill(c, &mut r, 0x004c_6700, 0x80);
        fill(c, &mut r, 0x004f_f0d8, 4);
        c.g[A0][0] = sx(OC932_SELF);
        c.pc = 0;
        (r, mask)
    }

    /// Store widget pointer `i` (or null by the mask) at `OC932_SELF + off`.
    fn oc932_field(c: &mut Cpu, mask: u32, off: u32, i: u32) {
        let v = if (mask >> (i % 32)) & 1 != 0 {
            oc932_widget(i)
        } else {
            0
        };
        c.bus.write32(OC932_SELF + off, v);
    }

    /// `SetFade(self, t)`: the five widgets, three looked-up ones and the
    /// five option widgets each get alpha `t` (null ones are skipped).
    #[test]
    fn c_fe_popup_confirm_set_fade_matches() {
        covered(
            ee::s03::f_001c5f20__generated,
            c_fe_popup_confirm_set_fade,
            96,
            &|c, seed| {
                let (mut r, mask) = oc932_world(c, seed, 0x1c5f20, 22);
                for (i, off) in [
                    0x70u32, 0x74, 0x78, 0x7c, 0x88, 0x90, 0x94, 0x98, 0x9c, 0xa0,
                ]
                .into_iter()
                .enumerate()
                {
                    oc932_field(c, mask.rotate_left(seed), off, i as u32);
                }
                c.f[12] = r.f32(2.0);
                c.pc = 0x001c_5f20;
            },
        );
    }

    /// `FindWidgets(self)`: every named widget found or missing (by call
    /// index), with the hash, find and rect calls unwinding at random.
    #[test]
    fn c_fe_popup_confirm_find_widgets_matches() {
        covered(
            ee::s03::f_001c6230__generated,
            c_fe_popup_confirm_find_widgets,
            120,
            &|c, seed| {
                let _ = oc932_world(c, seed, 0x1c6230, 70);
                c.pc = 0x001c_6230;
            },
        );
    }

    /// `ApplyBackgroundStyle(self, style)`: the style word (+0x31c) takes
    /// each of 0..3, with text widgets present or missing.
    #[test]
    fn c_fe_popup_confirm_apply_background_style_matches() {
        covered(
            ee::s03::f_001c66e8__generated,
            c_fe_popup_confirm_apply_background_style,
            96,
            &|c, seed| {
                let (mut r, mask) = oc932_world(c, seed, 0x1c66e8, 20);
                c.bus.write32(OC932_SELF + 0x31c, (seed / 4) % 4);
                for (i, off) in [0x74u32, 0x78, 0x7c, 0x88].into_iter().enumerate() {
                    oc932_field(c, mask.rotate_left(seed), off, i as u32);
                }
                c.g[A1][0] = r.next_u32() as u64;
                c.pc = 0x001c_66e8;
            },
        );
    }

    /// `ApplyConfig(self, cfg)`: cfg[0] == -1 or not, fade float zero or not,
    /// +0xbc zero or not, +0x90 zero or not (bits of the seed).
    #[test]
    fn c_fe_popup_confirm_apply_config_matches() {
        covered(
            ee::s03::f_001c6b08__generated,
            c_fe_popup_confirm_apply_config,
            128,
            &|c, seed| {
                let (mut r, _) = oc932_world(c, seed, 0x1c6b08, 26);
                fill(c, &mut r, OC932_CFG, 0x40);
                if seed & 1 != 0 {
                    c.bus.write32(OC932_CFG, 0xffff_ffff);
                }
                if seed & 2 != 0 {
                    c.bus.write32(OC932_CFG + 0xb0, 0);
                }
                if seed & 4 != 0 {
                    c.bus.write32(OC932_CFG + 0xbc, 0);
                }
                if seed & 8 != 0 {
                    c.bus.write32(OC932_CFG + 0x90, 0);
                }
                c.g[A1][0] = sx(OC932_CFG);
                c.pc = 0x001c_6b08;
            },
        );
    }

    /// `LayoutTextEntry(self, info)`: first call (info[0] == -1: lay out and
    /// show the four entry widgets) and later calls (hide them), widgets
    /// present or missing.
    #[test]
    fn c_fe_popup_confirm_layout_text_entry_matches() {
        covered(
            ee::s03::f_001c6d30__generated,
            c_fe_popup_confirm_layout_text_entry,
            128,
            &|c, seed| {
                let (mut r, mask) = oc932_world(c, seed, 0x1c6d30, 24);
                fill(c, &mut r, OC932_CFG, 0x40);
                c.bus
                    .write32(OC932_CFG, if seed & 1 != 0 { 0xffff_ffff } else { 0 });
                for (i, off) in [0x80u32, 0x84, 0x88, 0x8c].into_iter().enumerate() {
                    oc932_field(c, mask.rotate_left(seed / 2), off, i as u32);
                }
                c.g[A1][0] = sx(OC932_CFG);
                c.pc = 0x001c_6d30;
            },
        );
    }
}
