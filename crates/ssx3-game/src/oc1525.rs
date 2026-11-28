//! Staging module oc1525 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1525`.

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
    0x002c_f1c8,
    menu_sys_color_channel_item_draw,
    "MenuSysColorChannelItem_Draw",
)];

/// `void MenuSysColorChannelItem_Draw(Item* self, const float* rect)`
/// (0x2cf1c8; `cColorMenuItem::render` in the On Tour symbols): one colour
/// channel row of the colour editor. Draws the page row, the channel name
/// and its value (`%s`-style text through sprintf), a 4-vertex gradient bar
/// (80 wide, 10 high) from the colour with the channel at 0 to the colour
/// with the channel at 1, and a 3-vertex marker triangle at the channel's
/// current value. The selected row uses the highlight colour (0x4d53a0),
/// the others the plain colour (0x4d53b0).
pub fn menu_sys_color_channel_item_draw(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x1a0);
        let sp0 = sp(c);
        let s = |o: u32| sp0.wrapping_add(o);
        frame.save(c, 18, 0x150);
        frame.save(c, 19, 0x140);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x198);
        c.g[S3][0] = c.g[A1][0];
        frame.save_fpr(c, 22, 0x190);
        frame.save(c, 17, 0x160);
        frame.save_ra(c, 0x130);
        frame.save_fpr(c, 21, 0x188);
        frame.save_fpr(c, 20, 0x180);
        frame.save(c, 16, 0x170);
        // Page vtable call: vt[9](menu + delta, rect, self == selected item).
        c.g[S0][0] = c.lw(r32(c, S2));
        c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(0x12c));
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = c.lh(r32(c, S1).wrapping_add(0x48));
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        call_ptr(c, 0x002c_bf30, 0x002c_f210)?; // MenuSysPage_GetSelectedItem
        c.g[V0][0] = c.g[S2][0] ^ c.g[V0][0];
        c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x4c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = (c.g[V0][0] < 1) as u64;
        let method = r32(c, V1);
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, method, 0x002c_f228)?;
        // Bar geometry: f22 = rect.x + rect.w - 16 - 80 (bar x), f23 = f22 - 16 (text right edge).
        c.lwc1(0, r32(c, S3));
        c.lwc1(1, r32(c, S3).wrapping_add(0x8));
        c.g[AT][0] = 0x4180_0000;
        c.mtc1(1, 3);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x42a0_0000;
        c.mtc1(1, 2);
        c.g[A0][0] = c.lw(r32(c, S2));
        c.sub_s(0, 0, 3);
        c.sub_s(22, 0, 2);
        c.sub_s(23, 22, 3);
        call_ptr(c, 0x002c_bf30, 0x002c_f258)?; // MenuSysPage_GetSelectedItem
                                                // Highlight colour for the selected row, plain colour otherwise.
        if c.g[S2][0] != c.g[V0][0] {
            c.g[V0][0] = 0x4d_0000;
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x53b0));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4d_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x53a0));
        }
        c.cycles += 128;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[A2][0] = 0x480000;
        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x7470));
        c.swc1(20, sp0);
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 2);
        c.lwc1(0, c.g[V0][0] as u32);
        c.g[AT][0] = 0x40c00000;
        c.mtc1(1, 21);
        c.swc1(0, s(0x4));
        c.g[AT][0] = 0x41800000;
        c.mtc1(1, 3);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
        c.swc1(0, s(0x8));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.swc1(1, s(0xc));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.ldl(2, s(0x7));
        c.ldr(2, sp0);
        c.ldl(4, s(0xf));
        c.ldr(4, s(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw(c.g[S2][0] as u32);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
        c.swc1(2, s(0x14));
        c.swc1(2, s(0x10));
        c.ldl(8, s(0x17));
        c.ldr(8, s(0x10));
        c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x2f));
        c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x28));
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.add_s(13, 13, 21);
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.lwc1(12, c.g[S3][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        c.sub_s(13, 13, 20);
        c.add_s(12, 12, 3);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x58));
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        call_ptr(c, 0x00391cb0, 0x002cf338)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[S0][0] = 0x480000;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[S1][0] = sx(s(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6710));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.lwc1(12, c.g[V0][0] as u32);
        call_ptr(c, 0x00413af8, 0x002cf35c)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fa4));
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
        c.lwc1(0, c.g[A3][0] as u32);
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(7, 1);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00417828, 0x002cf390)?;
        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x3c));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x38));
        call_ptr(c, 0x00391fb0, 0x002cf3b8)?;
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.sub_s(12, 23, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.add_s(13, 13, 21);
        c.sub_s(13, 13, 20);
        call_ptr(c, 0x00391cb0, 0x002cf3d4)?;
        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[S0][0] = sx(s(0x60));
        c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = 0x3;
        c.add_s(0, 0, 1);
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(3, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(3, s(0x37));
        c.sdr(3, s(0x30));
        c.sdl(4, s(0x3f));
        c.sdr(4, s(0x38));
        c.sub_s(0, 0, 21);
        c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x7));
        c.ldr(10, c.g[A1][0] as u32);
        c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
        c.sdl(10, s(0x47));
        c.sdr(10, s(0x40));
        c.sdl(4, s(0x4f));
        c.sdr(4, s(0x48));
        c.g[V1][0] = sx(sp0.wrapping_add(c.g[V0][0] as u32));
        c.g[AT][0] = 0x42a00000;
        c.mtc1(1, 1);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x30), 0u64);
        c.g[A0][0] = c.g[V1][0];
        c.sub_s(0, 0, 2);
        c.swc1(20, (c.g[A0][0] as u32).wrapping_add(0x40));
        c.swc1(22, s(0x50));
        c.swc1(1, s(0x58));
        c.swc1(0, s(0x54));
        c.swc1(2, s(0x5c));

        // Vertex setup is done; this loop only counts a2 from 3 down to -1.
        loop {
            c.cycles += 7;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            let more = c.g[A2][0] != c.g[A3][0];
            c.lwc1(0, s(0x34));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002c_f470);
            }
        }
        c.cycles += 85;
        c.g[AT][0] = 0x43000000;
        c.mtc1(1, 5);
        c.g[A3][0] = 0x80;
        c.lwc1(1, s(0x38));
        c.mul_s(0, 0, 5);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 4);
        c.mul_s(1, 1, 5);
        c.lwc1(6, s(0x50));
        c.mtc1(0, 3);
        c.cvt_w_s(9, 0);
        c.mfc1(6, 9);
        c.lwc1(8, s(0x54));
        c.cvt_w_s(0, 1);
        c.mfc1(5, 0);
        c.lwc1(7, s(0x5c));
        c.lwc1(2, s(0x3c));
        c.lwc1(0, s(0x44));
        c.add_s(7, 8, 7);
        c.swc1(6, s(0x120));
        c.mul_s(2, 2, 5);
        c.swc1(8, s(0x124));
        c.mul_s(0, 0, 5);
        c.swc1(3, s(0x128));
        c.swc1(4, s(0x12c));
        c.cvt_w_s(1, 2);
        c.mfc1(4, 1);
        c.cvt_w_s(1, 0);
        c.mfc1(8, 1);
        c.sw(s(0xa4), c.g[A1][0]);
        c.lq(3, s(0x120));
        c.swc1(7, s(0x124));
        c.sq(3, s(0x80));
        c.lq(2, s(0x120));
        c.sw(s(0xa8), c.g[A0][0]);
        c.sq(2, s(0xb0));
        c.sw(s(0x74), c.g[A1][0]);
        c.sw(s(0x78), c.g[A0][0]);
        c.sw(s(0xa0), c.g[A2][0]);
        c.sw(s(0x70), c.g[A2][0]);
        c.sw(s(0x7c), c.g[A3][0]);
        c.swc1(3, s(0x64));
        c.swc1(3, s(0x60));
        c.swc1(4, s(0x68));
        c.sw(s(0xac), c.g[A3][0]);
        c.swc1(3, s(0x94));
        c.swc1(3, s(0x90));
        c.swc1(4, s(0x98));
        c.sw(s(0xd0), c.g[T0][0]);
        c.lwc1(0, s(0x58));
        c.lwc1(2, s(0x48));
        c.add_s(6, 6, 0);
        c.lwc1(1, s(0x4c));
        c.swc1(8, s(0x124));
        c.mul_s(2, 2, 5);
        c.mul_s(1, 1, 5);
        c.sw(s(0x100), c.g[T0][0]);
        c.swc1(6, s(0x120));
        c.cvt_w_s(0, 2);
        c.mfc1(4, 0);
        c.sw(s(0x10c), c.g[A3][0]);
        c.lq(3, s(0x120));
        c.cvt_w_s(0, 1);
        c.mfc1(5, 0);
        c.swc1(7, s(0x124));
        c.sq(3, s(0xe0));
        c.lq(2, s(0x120));
        c.sw(s(0x104), c.g[A0][0]);
        c.sw(s(0x108), c.g[A1][0]);
        c.sw(s(0xd4), c.g[A0][0]);
        c.sw(s(0xd8), c.g[A1][0]);
        c.sw(s(0xdc), c.g[A3][0]);
        c.swc1(3, s(0xc4));
        c.swc1(3, s(0xc0));
        c.swc1(4, s(0xc8));
        c.swc1(3, s(0xf4));
        c.swc1(3, s(0xf0));
        c.swc1(4, s(0xf8));
        c.sq(2, s(0x110));
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.lwc1(0, c.g[V0][0] as u32);
        c.c_eq_s(0, 4);

        // Selection state: the item's value at +0x14 equals f4 (1.0): flags 4, else 0x14.
        let fcc = c.fcc();
        c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x854));
        let flag = if fcc {
            c.cycles += 5;
            0x4
        } else {
            c.cycles += 6;
            0x14
        };
        c.g[A1][0] = 0xffff_ffff_ffff_ff83;
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0xe84));
        c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0x4));
        c.g[V0][0] &= c.g[A1][0];
        c.g[V0][0] |= flag;
        c.cycles += 95;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = 0xffffffffffffffff;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x4;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[A3][0] = 0u64;
        c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x002cf644)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
        c.g[A2][0] = c.g[S0][0];
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x3;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.lwc1(0, s(0x58));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.lwc1(4, s(0x50));
        c.lwc1(1, c.g[V1][0] as u32);
        c.g[A3][0] = 0u64;
        c.lwc1(5, s(0x54));
        c.mul_s(0, 0, 1);
        c.g[AT][0] = 0x40a00000;
        c.mtc1(1, 7);
        c.g[AT][0] = 0x40e00000;
        c.mtc1(1, 8);
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 1);
        c.add_s(4, 4, 0);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 9);
        c.sub_s(8, 5, 8);
        c.g[AT][0] = 0x43000000;
        c.mtc1(1, 0);
        c.sub_s(5, 5, 1);
        c.lwc1(3, s(0x4));
        c.sub_s(6, 4, 7);
        c.lwc1(1, s(0x8));
        c.lwc1(2, s(0xc));
        c.mul_s(3, 3, 0);
        c.swc1(8, s(0x124));
        c.mul_s(1, 1, 0);
        c.swc1(6, s(0x120));
        c.mul_s(2, 2, 0);
        c.sw(s(0x128), 0u64);
        c.add_s(7, 4, 7);
        c.swc1(9, s(0x12c));
        c.cvt_w_s(0, 3);
        c.mfc1(8, 0);
        c.cvt_w_s(0, 2);
        c.mfc1(4, 0);
        c.lq(3, s(0x120));
        c.cvt_w_s(0, 1);
        c.mfc1(9, 0);
        c.swc1(5, s(0x124));
        c.swc1(4, s(0x120));
        c.sw(s(0x128), 0u64);
        c.sw(s(0xd0), c.g[T0][0]);
        c.lq(2, s(0x120));
        c.swc1(7, s(0x120));
        c.swc1(8, s(0x124));
        c.sw(s(0x128), 0u64);
        c.sq(2, s(0xb0));
        c.sw(s(0x70), c.g[T0][0]);
        c.sw(s(0xa0), c.g[T0][0]);
        c.lq(2, s(0x120));
        c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.sq(3, s(0x80));
        c.sw(s(0xd8), c.g[A0][0]);
        c.sw(s(0x78), c.g[A0][0]);
        c.sw(s(0xa8), c.g[A0][0]);
        c.sw(s(0xd4), c.g[T1][0]);
        c.sw(s(0x74), c.g[T1][0]);
        c.sw(s(0xa4), c.g[T1][0]);
        c.sq(2, s(0xe0));
        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
        call_ptr(c, t, 0x002cf75c)?;

        frame.restore(c, 16, 0x170);
        frame.restore(c, 17, 0x160);
        frame.restore(c, 18, 0x150);
        frame.restore(c, 19, 0x140);
        frame.restore_ra(c, 0x130);
        frame.restore_fpr(c, 23, 0x198);
        frame.restore_fpr(c, 22, 0x190);
        frame.restore_fpr(c, 21, 0x188);
        frame.restore_fpr(c, 20, 0x180);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of this module's callee fakes: the magic word at `OC1525_CTRL`.
    const OC1525_CTRL: u32 = DATA + 0xf_f000;
    const OC1525_MAGIC: u32 = 0x1525_c0de;

    /// Gated fakes of the callees (hooked once per process; with the gate off
    /// they run the body they replaced).
    macro_rules! oc1525_fakes {
        ($($fake:ident, $prev:ident, $addr:literal, $slot:path, $gen:path;)*) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1525_CTRL) != OC1525_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr)
                }
            )*
            fn oc1525_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1525_fakes! {
        oc1525_fake_2cbf30, OC1525_PREV_2CBF30, 0x002c_bf30, ee::s09::OVERRIDE_002CBF30, ee::s09::f_002cbf30__generated;
        oc1525_fake_391cb0, OC1525_PREV_391CB0, 0x0039_1cb0, ee::s12::OVERRIDE_00391CB0, ee::s12::f_00391cb0__generated;
        oc1525_fake_391fb0, OC1525_PREV_391FB0, 0x0039_1fb0, ee::s12::OVERRIDE_00391FB0, ee::s12::f_00391fb0__generated;
        oc1525_fake_413af8, OC1525_PREV_413AF8, 0x0041_3af8, ee::s15::OVERRIDE_00413AF8, ee::s15::f_00413af8__generated;
        oc1525_fake_417828, OC1525_PREV_417828, 0x0041_7828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
    }

    /// A float in `[0, 1]` as raw bits.
    fn oc1525_unit(r: &mut crate::difftest::Rng) -> u32 {
        ((r.next_u32() % 1000) as f32 / 999.0).to_bits()
    }

    /// `MenuSysColorChannelItem_Draw` against the generated body: the page
    /// vtable call, both colour choices (selected row or not), the highlight
    /// flag of the item (value 1.0 or not), the four channels and every
    /// callee (fakes) unwinding at each call.
    #[test]
    fn menu_sys_color_channel_item_draw_matches() {
        covered(
            ee::s09::f_002cf1c8__generated,
            menu_sys_color_channel_item_draw,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cf1c8, 11);
                oc1525_install_fakes();
                c.bus.write32(OC1525_CTRL, OC1525_MAGIC);
                let this = DATA + 0x1000;
                let menu = DATA + 0x2000;
                let vt = DATA + 0x3000;
                let rect = DATA + 0x4000;
                let vals = DATA + 0x5000;
                let font = DATA + 0x6000;
                let hold = DATA + 0x7000;
                let gfx = DATA + 0x8000;
                let target = DATA + 0x9000;
                let sys = DATA + 0xa000;
                let idx = seed % 4;
                c.bus.write32(this, menu);
                c.bus.write32(this + 0x14, vals);
                c.bus.write32(this + 0x18, idx);
                c.bus.write32(menu + 0x12c, vt);
                c.bus.write32(menu + 0x124, hold);
                c.bus.write32(hold + 0x58, font);
                c.bus.write16(vt + 0x48, 0x10);
                c.bus.write32(vt + 0x4c, fakes::FAKE + 0x10);
                for i in 0..4 {
                    let v = if i == 0 && (seed / 4) % 2 == 0 {
                        1.0f32.to_bits()
                    } else {
                        oc1525_unit(&mut r)
                    };
                    c.bus.write32(vals + 4 * i, v);
                    let v = ((r.next_u32() % 4000) as f32 / 10.0).to_bits();
                    c.bus.write32(rect + 4 * i, v);
                }
                for i in 0..3 {
                    let v = oc1525_unit(&mut r);
                    c.bus.write32(0x004d_53a0 + 4 * i, v);
                    let v = oc1525_unit(&mut r);
                    c.bus.write32(0x004d_53b0 + 4 * i, v);
                }
                let v = oc1525_unit(&mut r);
                c.bus.write32(font + 0x38, v);
                let v = oc1525_unit(&mut r);
                c.bus.write32(font + 0x3c, v);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), gfx);
                c.bus.write32(gfx + 0xe84, target);
                c.bus.write32(gfx + 0x10d8, sys);
                c.bus.write16(sys + 0x238, 0x20);
                c.bus.write32(sys + 0x23c, fakes::FAKE + 0x20);
                // Calls 0 and 2 are MenuSysPage_GetSelectedItem: the item
                // itself or a pointer from the fake's hash.
                if seed % 2 == 0 {
                    c.bus.write32(fakes::LOG_RESULTS, this);
                }
                if (seed / 2) % 2 == 0 {
                    c.bus.write32(fakes::LOG_RESULTS + 8, this);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = rect as u64;
            },
        );
    }
}
