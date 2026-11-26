//! Staging module oc843 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc843`.

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
    0x0018_f2b8,
    fe_profile_load_refresh_profile_list,
    "cFEStateProfileLoad_RefreshProfileList",
)];

/// `cFEStateProfileLoad::displayOn`: redraws the profile-load screen of the
/// memory-card front end. With no card it shows the "no cards" message and
/// refreshes the slot list instead. Otherwise it writes the device name, then
/// for each of the six profile slots sets its title, the static/greyed state
/// and the date/time texts, and finally shows the left/right device arrows
/// when another usable memory card exists in that direction.
///
/// Frame: `s1` = self, `s3` = the memory-card manager, `s4` = slot index
/// (0..6), `s5` = slot number (1..7), `s2` = the widget-name buffer,
/// `s6` = self.mTitle.
pub fn fe_profile_load_refresh_profile_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_18f9ac: {
            'b_18f370: {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x730));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x710));
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6f0));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x700));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x6e0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x6d0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x6c0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x6b0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x6a0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x690), c.g[RA][0]);
                call_ptr(c, 0x00227f80, 0x0018f2f4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[S3][0] = c.g[V0][0];
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018f30c)?;
                if c.g[V0][0] == 0u64 {
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_18f370;
                } else {
                    'b_18f34c: {
                        c.cycles += 8;
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x0018f32c)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x214));
                            break 'b_18f34c;
                        } else {
                            c.cycles += 6;
                            call_ptr(c, 0x001d8240, 0x0018f33c)?;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x0018f9d8, 0x0018f344)?;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
                            break 'b_18f9ac;
                        }
                    }
                    'b_18f36c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x3;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A1][0] = 0x3;
                        if t {
                            break 'b_18f36c;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f36c)?;
                            break 'b_18f36c;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    break 'b_18f370;
                }
            }
            'b_18f9a8: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x0018f380)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A2][0] = c.g[SP][0];
                if t {
                    break 'b_18f9a8;
                } else {
                    c.cycles += 40;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                    c.g[A3][0] = 0x320;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x00241710, 0x0018f398)?;
                    c.g[S5][0] = 0x1;
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x680), 0u64);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e18));
                    c.g[FP][0] = 0x4a0000;
                    call_ptr(c, 0x00317670, 0x0018f3b0)?;
                    c.g[S7][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x238));
                    c.g[S4][0] = 0u64;
                    call_ptr(c, 0x0039d860, 0x0018f3c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x003a0d00, 0x0018f3d4)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x21c), c.g[V0][0]);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018f3f0)?;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e28));
                    call_ptr(c, 0x00317670, 0x0018f3fc)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0018f408)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0018f424)?;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    'l_18f428: loop {
                        'b_18f894: {
                            'b_18f7b8: {
                                'b_18f7a0: {
                                    'b_18f608: {
                                        c.cycles += 30;
                                        c.g[V0][0] = 0x460000;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1da0));
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00417828, 0x0018f43c)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00317670, 0x0018f444)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f450)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[V1][0] = 0x4c0000;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66c8));
                                        c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
                                        c.ldr(4, c.g[V0][0] as u32);
                                        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                                        c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x8f));
                                        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x88));
                                        c.sdl(6, (c.g[S0][0] as u32).wrapping_add(0x97));
                                        c.sdr(6, (c.g[S0][0] as u32).wrapping_add(0x90));
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f498)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        if t {
                                            break 'b_18f608;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018f4b8)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                break 'b_18f608;
                                            } else {
                                                'b_18f604: {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x434)
                                                        );
                                                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1b8)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1bc)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x0018f4d8)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A1][0] = c.g[S4][0];
                                                    if t {
                                                        break 'b_18f604;
                                                    } else {
                                                        'b_18f594: {
                                                            'b_18f564: {
                                                                'b_18f560: {
                                                                    c.cycles += 14;
                                                                    c.g[A3][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x434));
                                                                    c.g[A2][0] = c.g[S6][0];
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[A3][0] as u32);
                                                                    c.g[A0][0] = c
                                                                        .lh((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x70));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x74));
                                                                    let t = c.g[V1][0] as u32;
                                                                    c.g[A0][0] = sx((c.g[A3][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A0][0] as u32,
                                                                        ));
                                                                    call_ptr(c, t, 0x0018f4fc)?;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[A1][0] = c.g[S6][0];
                                                                    call_ptr(
                                                                        c, 0x003a0e90, 0x0018f508,
                                                                    )?;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x1b08));
                                                                    c.g[V0][0] = 0x2;
                                                                    let t =
                                                                        c.g[V1][0] == c.g[V0][0];
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x680));
                                                                    if t {
                                                                        break 'b_18f560;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        if c.g[V1][0] != 0u64 {
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            break 'b_18f564;
                                                                        } else {
                                                                            c.cycles += 16;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[A0][0] = 0x1;
                                                                            c.sw(
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x680,
                                                                                    ),
                                                                                c.g[A0][0],
                                                                            );
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[A0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x40,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x44,
                                                                                ));
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, t, 0x0018f540,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[A0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xc8,
                                                                                ));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xcc,
                                                                                ));
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[A0][0]
                                                                                            as u32,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, t, 0x0018f558,
                                                                            )?;
                                                                            c.g[A2][0] = c.g[S5][0];
                                                                            break 'b_18f594;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 1;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                break 'b_18f564;
                                                            }
                                                            c.cycles += 12;
                                                            c.g[A1][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xcc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0018f578)?;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x44));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x0018f590)?;
                                                            c.g[A2][0] = c.g[S5][0];
                                                            break 'b_18f594;
                                                        }
                                                        c.cycles += 28;
                                                        c.g[A1][0] = sx((c.g[FP][0] as u32)
                                                            .wrapping_add(0x15f0));
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00417828, 0x0018f5a0)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00317670, 0x0018f5a8)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x0018f5b4)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0018f5d0)?;
                                                        c.g[A1][0] = sx((c.g[S7][0] as u32)
                                                            .wrapping_add(0x15f8));
                                                        c.g[A2][0] = c.g[S5][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00417828, 0x0018f5e0)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x00317670, 0x0018f5e8)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x0018f5f4)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        break 'b_18f7a0;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                break 'b_18f608;
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018f620)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A2][0] = c.g[S6][0];
                                    if t {
                                        break 'b_18f7b8;
                                    } else {
                                        'b_18f6dc: {
                                            'b_18f6a8: {
                                                c.cycles += 19;
                                                c.g[A3][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                c.g[A1][0] = c.g[S4][0];
                                                c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0018f644)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[S6][0];
                                                call_ptr(c, 0x003a0e90, 0x0018f650)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                                c.g[A1][0] = c.g[S4][0];
                                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0018f66c)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_18f6a8;
                                                } else {
                                                    c.cycles += 13;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x0018f688)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x0018f6a0)?;
                                                    c.g[A2][0] = c.g[S5][0];
                                                    break 'b_18f6dc;
                                                }
                                            }
                                            c.cycles += 13;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018f6c0)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018f6d8)?;
                                            c.g[A2][0] = c.g[S5][0];
                                            break 'b_18f6dc;
                                        }
                                        c.cycles += 49;
                                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x15f0));
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00417828, 0x0018f6e8)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00317670, 0x0018f6f0)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f6fc)?;
                                        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[SP][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x78));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f720)?;
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x003a0e90, 0x0018f72c)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f744)?;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00417828, 0x0018f754)?;
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x00317670, 0x0018f75c)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f768)?;
                                        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[A2][0] = c.g[SP][0];
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f78c)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[SP][0];
                                        call_ptr(c, 0x003a0e90, 0x0018f798)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        break 'b_18f7a0;
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018f7b0)?;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                break 'b_18f894;
                            }
                            c.cycles += 55;
                            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x434));
                            c.g[A1][0] = c.g[S4][0];
                            c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f7d4)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S6][0];
                            call_ptr(c, 0x003a0e90, 0x0018f7e0)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f7f8)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f810)?;
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x15f0));
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00417828, 0x0018f820)?;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00317670, 0x0018f828)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0018f834)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f850)?;
                            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x15f8));
                            c.g[A2][0] = c.g[S5][0];
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00417828, 0x0018f860)?;
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00317670, 0x0018f868)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0018f874)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f890)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            break 'b_18f894;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S5][0] as i64) < 7i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0018f428);
                            }
                            continue 'l_18f428;
                        } else {
                            'b_18f928: {
                                'b_18f8f8: {
                                    c.cycles += 8;
                                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x428));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A3][0] = 0x1;
                                    call_ptr(c, 0x001d97e0, 0x0018f8b4)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[A3][0] = 0x460000;
                                    if t {
                                        break 'b_18f8f8;
                                    } else {
                                        c.cycles += 14;
                                        c.g[A2][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1d90));
                                        call_ptr(c, 0x00317670, 0x0018f8cc)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0018f8d8)?;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[A2][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x0018f8f0)?;
                                        c.g[A1][0] = c.g[S3][0];
                                        break 'b_18f928;
                                    }
                                }
                                c.cycles += 12;
                                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1d90));
                                call_ptr(c, 0x00317670, 0x0018f900)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0018f90c)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0018f924)?;
                                c.g[A1][0] = c.g[S3][0];
                                break 'b_18f928;
                            }
                            'b_18f97c: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x001d97e0, 0x0018f938)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V1][0] = 0x460000;
                                if t {
                                    break 'b_18f97c;
                                } else {
                                    c.cycles += 14;
                                    c.g[V0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1d80));
                                    call_ptr(c, 0x00317670, 0x0018f950)?;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x0018f95c)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018f974)?;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
                                    break 'b_18f9ac;
                                }
                            }
                            c.cycles += 11;
                            c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1d80));
                            call_ptr(c, 0x00317670, 0x0018f984)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0018f990)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x0018f9a8)?;
                            break 'b_18f9a8;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x720));
            break 'b_18f9ac;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x710));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x700));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x690));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x730));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x240000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC843_CTRL: u32 = DATA + 0x24_0000;
    pub(crate) const OC843_MAGIC: u32 = 0x4f43_3834;
    pub(crate) const OC843_SELF: u32 = DATA + 0x24_1000;
    pub(crate) const OC843_SELF_VT: u32 = DATA + 0x24_2000;
    pub(crate) const OC843_UI: u32 = DATA + 0x24_3000;
    pub(crate) const OC843_MP: u32 = DATA + 0x24_4000;
    pub(crate) const OC843_MC: u32 = DATA + 0x24_5000;
    pub(crate) const OC843_MC_VT: u32 = DATA + 0x24_6000;
    pub(crate) const OC843_WIDGET: u32 = DATA + 0x24_7000;
    pub(crate) const OC843_WIDGET_VT: u32 = DATA + 0x24_8000;
    pub(crate) const OC843_APP: u32 = DATA + 0x24_9000;
    /// Per-callee result lists: 0x80 bytes per `target & 0xfff`.
    pub(crate) const OC843_MAIL: u32 = DATA + 0x28_0000;

    /// Self's vtable: `displayFileOK`, `triggerDisplayState`, `setMenuHighlight`.
    const OC843_SELF_SLOTS: [u32; 3] = [0x150, 0x118, 0x158];
    /// The memory card's vtable: `IsAvailable`, `IsFormatted`, `IsActionable`,
    /// `IsFileSlotDisabled`, `HasFileName`, `GetTitleString`, `GetFileDate`,
    /// `GetFileTime`.
    const OC843_MC_SLOTS: [u32; 8] = [0x1a8, 0x1b8, 0x1b0, 0x210, 0x218, 0x70, 0x78, 0x80];
    /// A text widget's vtable: `setStatic`, `setVisible`, `setUseStaticColour`.
    const OC843_WIDGET_SLOTS: [u32; 3] = [0x40, 0x48, 0xc8];

    /// The mailbox of the callee at `target`: `[count, next, v0 values...]`.
    pub(crate) fn oc843_mail(target: u32) -> u32 {
        OC843_MAIL + (target & 0xfff) * 0x80
    }

    /// Make the callee at `target` return `vals` in turn (cycling) in `v0`.
    pub(crate) fn oc843_force(c: &mut Cpu, target: u32, vals: &[u64]) {
        assert!(vals.len() <= 14);
        let m = oc843_mail(target);
        c.bus.write64(m, vals.len() as u64);
        c.bus.write64(m + 8, 0);
        for (i, v) in vals.iter().enumerate() {
            c.bus.write64(m + 16 + 8 * i as u32, *v);
        }
    }

    /// A fake callee: logs the call like `fakes::fake_call`, then returns the
    /// next forced result of its mailbox, unless the call unwound.
    pub(crate) fn oc843_fake_call(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        let m = oc843_mail(target);
        let n = c.bus.read64(m);
        if c.pc == ra && n != 0 {
            let i = c.bus.read64(m + 8);
            c.g[V0][0] = c.bus.read64(m + 16 + 8 * (i % n) as u32);
            c.bus.write64(m + 8, i + 1);
        }
    }

    /// The machine's miss hook (vtable slots pointing at `FAKE`).
    pub(crate) fn oc843_miss(c: &mut Cpu, target: u32) {
        oc843_fake_call(c, target);
    }

    macro_rules! oc843_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC843_CTRL) != OC843_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc843_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc843_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc843_fakes!(
        (
            oc843_fake_227f80,
            OC843_PREV_227F80,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80
        ),
        (
            oc843_fake_1d8240,
            OC843_PREV_1D8240,
            ee::s04::OVERRIDE_001D8240,
            ee::s04::f_001d8240__generated,
            0x001d_8240
        ),
        (
            oc843_fake_241710,
            OC843_PREV_241710,
            ee::s06::OVERRIDE_00241710,
            ee::s06::f_00241710__generated,
            0x0024_1710
        ),
        (
            oc843_fake_317670,
            OC843_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc843_fake_39d860,
            OC843_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc843_fake_3a0d00,
            OC843_PREV_3A0D00,
            ee::s13::OVERRIDE_003A0D00,
            ee::s13::f_003a0d00__generated,
            0x003a_0d00
        ),
        (
            oc843_fake_3a0e90,
            OC843_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc843_fake_417828,
            OC843_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc843_fake_1d97e0,
            OC843_PREV_1D97E0,
            ee::s04::OVERRIDE_001D97E0,
            ee::s04::f_001d97e0__generated,
            0x001d_97e0
        ),
    );

    /// Vtable slot `off` of `vt`: a random `this` delta, the method a fake
    /// whose mailbox key is its own address.
    fn oc843_slot(c: &mut Cpu, r: &mut Rng, vt: u32, off: u32) {
        let delta = (r.next_u32() % 9) * 4;
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE + off + 4);
    }

    /// The fake address of the method in vtable slot `off`.
    fn oc843_method(off: u32) -> u32 {
        fakes::FAKE + off + 4
    }

    /// `n` random 0/1 results, 1 with the chance `(m - 1) / m`.
    fn oc843_bits(r: &mut Rng, n: usize, m: u32) -> Vec<u64> {
        (0..n).map(|_| (r.next_u32() % m != 0) as u64).collect()
    }

    /// The screen, the card manager and its card, all methods fakes with
    /// forced results; every seed a different mix of card states.
    fn oc843_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x18_f2b8, 48);
        c.hooks.miss = oc843_miss;
        oc843_install_fakes();
        c.bus.write32(OC843_CTRL, OC843_MAGIC);
        c.pc = 0x0018_f2b8;
        c.g[A0][0] = sx(OC843_SELF);

        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC843_APP);
        let action = match r.next_u32() % 3 {
            0 => 2,
            n => n - 1,
        };
        c.bus.write32(GAME_GP.wrapping_sub(0x1b08), action);
        for i in 0..4 {
            c.bus.write32(0x004c_66c8 + 4 * i, r.next_u32());
        }

        c.bus.write32(OC843_SELF + 8, OC843_SELF_VT);
        c.bus.write32(OC843_SELF + 0x40, OC843_UI);
        let status = if r.next_u32() % 2 == 0 {
            3
        } else {
            r.next_u32() % 5
        };
        c.bus.write32(OC843_SELF + 0x214, status);
        for off in OC843_SELF_SLOTS {
            oc843_slot(c, &mut r, OC843_SELF_VT, off);
        }
        c.bus.write32(OC843_MP + 0x428, r.next_u32() % 4);
        c.bus.write32(OC843_MP + 0x434, OC843_MC);
        c.bus.write32(OC843_MC, OC843_MC_VT);
        for off in OC843_MC_SLOTS {
            oc843_slot(c, &mut r, OC843_MC_VT, off);
        }
        c.bus.write32(OC843_WIDGET + 8, OC843_WIDGET_VT);
        for off in OC843_WIDGET_SLOTS {
            oc843_slot(c, &mut r, OC843_WIDGET_VT, off);
        }

        oc843_force(c, 0x0022_7f80, &[OC843_MP as u64]);
        oc843_force(c, 0x0039_d860, &[OC843_WIDGET as u64]);
        // displayFileOK: a few answers in turn (RefreshSlots reads it too).
        let n = 1 + (r.next_u32() % 3) as usize;
        let ok = oc843_bits(&mut r, n, 3);
        oc843_force(c, oc843_method(0x150), &ok);
        // IsAvailable: constant, so that RefreshSlots (called when there is
        // no card) cannot recurse back into the function.
        let avail = (r.next_u32() % 4 != 0) as u64;
        oc843_force(c, oc843_method(0x1a8), &[avail]);
        // checkNextAvailableDevice: -1 (none) or a device, left then right.
        let next: Vec<u64> = (0..2)
            .map(|_| if r.next_u32() % 2 == 0 { u64::MAX } else { 3 })
            .collect();
        oc843_force(c, 0x001d_97e0, &next);
        let has = oc843_bits(&mut r, 6, 2);
        oc843_force(c, oc843_method(0x218), &has);
        let disabled = oc843_bits(&mut r, 6, 2);
        oc843_force(c, oc843_method(0x210), &disabled);
        let formatted = oc843_bits(&mut r, 6, 4);
        oc843_force(c, oc843_method(0x1b8), &formatted);
        let actionable = oc843_bits(&mut r, 6, 3);
        oc843_force(c, oc843_method(0x1b0), &actionable);
    }

    /// `cFEStateProfileLoad::displayOn` (the profile list of the load screen):
    /// no card (message and slot refresh), screen state change, the six slots
    /// in all their card states, and the device arrows, with calls unwinding
    /// at any point.
    #[test]
    fn fe_profile_load_refresh_profile_list_matches() {
        check(
            ee::s02::f_0018f2b8__generated,
            fe_profile_load_refresh_profile_list,
            300,
            &oc843_setup,
        );
    }

    /// The fixture reaches the function's branches: the no-card message, the
    /// screen state change, titles, static colours, dates and times, both
    /// arrows, and unwinds.
    #[test]
    fn fe_profile_load_refresh_profile_list_fixture_paths() {
        let generated: GuestFn = ee::s02::f_0018f2b8__generated;
        let mut targets = BTreeSet::new();
        let mut counts = BTreeSet::new();
        let mut unwinds = 0;
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc843_setup(&mut c, seed);
            generated(&mut c);
            unwinds += (c.pc & 0xffff_0000 == 0xdead_0000) as u32;
            let n = c.bus.read32(fakes::LOG_COUNT);
            counts.insert(n);
            for i in 0..n {
                targets.insert(c.bus.read32(fakes::LOG_START + 0x220 * i));
            }
        }
        for t in [
            0x001d_8240,
            0x0024_1710,
            0x001d_97e0,
            0x003a_0e90,
            0x0041_7828,
            oc843_method(0x118),
            oc843_method(0x158),
            oc843_method(0x40),
            oc843_method(0x48),
            oc843_method(0xc8),
            oc843_method(0x70),
            oc843_method(0x78),
            oc843_method(0x80),
        ] {
            assert!(targets.contains(&t), "never called: {t:#x}");
        }
        assert!(counts.len() >= 15, "call counts {counts:?}");
        assert!(unwinds > 20, "unwinds {unwinds}");
    }
}
