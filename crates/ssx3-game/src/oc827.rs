//! Staging module oc827 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc827`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_32a8, rider_detail_init, "cFEStateRiderDetail_Init"),
    (
        0x0018_33d0,
        rider_detail_on_activate,
        "cFEStateRiderDetail_OnActivate",
    ),
    (
        0x0018_35a8,
        rider_detail_render,
        "cFEStateRiderDetail_Render",
    ),
    (
        0x0018_35f8,
        rider_detail_on_child_event,
        "cFEStateRiderDetail_OnChildEvent",
    ),
    (
        0x0018_3710,
        rider_detail_on_widget_action,
        "cFEStateRiderDetail_OnWidgetAction",
    ),
    (
        0x0018_4668,
        uber_trick_show_cost,
        "cFEStateUberTrick_ShowCost",
    ),
];

/// `void cFEStateRiderDetail_Init(cFEStateRiderDetail* self)`: add the rider-detail screen to the
/// UI engine and play its first frame, set up the rider preview, make the human rider visible with
/// the current character, enable the audio and flag the cheat-character panel (`+0x50`) when the
/// player owns cheat characters.
/// (0x001832a8; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1832fc: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2790));
            c.g[RA][0] = sx(0x001832d0);
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001832d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001832e4);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001832e4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1832fc;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001832fc);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001832fc)?;
                break 'b_1832fc;
            }
        }
        'b_1833b8: {
            c.cycles += 47;
            c.g[RA][0] = sx(0x00183304);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x00182db8,
                f: ee::s02::cFEStateCharSetup_InitRiderPreview,
            }
            .call(c, 0x00183304)?;
            c.g[RA][0] = sx(0x0018330c);
            call_ptr(c, 0x0014dc80, 0x0018330c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00183318);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00183318)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[RA][0] = sx(0x00183328);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00147488, 0x00183328)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x00183340);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x00183340)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = 0u64;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
            c.g[RA][0] = sx(0x00183358);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0570, 0x00183358)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A3][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[RA][0] = sx(0x00183374);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0508, 0x00183374)?;
            c.g[RA][0] = sx(0x0018337c);
            call_ptr(c, 0x0028b180, 0x0018337c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00183388);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0028f140, 0x00183388)?;
            c.g[V0][0] = 0x1;
            c.g[RA][0] = sx(0x00183394);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
            call_ptr(c, 0x0014dc80, 0x00183394)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001833a0);
            c.g[A1][0] = 0xd;
            call_ptr(c, 0x0014dd58, 0x001833a0)?;
            c.g[A1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001833b0);
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x001577a0, 0x001833b0)?;
            if c.g[V0][0] == 0u64 {
                c.sw((c.g[S2][0] as u32).wrapping_add(0x50), 0u64);
                break 'b_1833b8;
            } else {
                break 'b_1833b8;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_OnActivate(self, arg)`: tag the five rider-detail widgets with
/// states 5..9 (show them), then run the base `cFEState_OnActivate`.
/// (0x001833d0; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_18342c: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = 0x460000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S2][0] = c.g[A1][0];
            c.g[RA][0] = sx(0x001833f8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2778));
            call_ptr(c, 0x00317670, 0x001833f8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183404);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183404)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_18342c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00183424);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00183424)?;
                c.g[V0][0] = 0x5;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18342c;
            }
        }
        'b_18346c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00183438);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2768));
            call_ptr(c, 0x00317670, 0x00183438)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183444);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183444)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_18346c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00183464);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00183464)?;
                c.g[V0][0] = 0x6;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18346c;
            }
        }
        'b_1834ac: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00183478);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2758));
            call_ptr(c, 0x00317670, 0x00183478)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183484);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183484)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_1834ac;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001834a4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001834a4)?;
                c.g[V0][0] = 0x7;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_1834ac;
            }
        }
        'b_1834ec: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001834b8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2748));
            call_ptr(c, 0x00317670, 0x001834b8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x001834c4);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001834c4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_1834ec;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001834e4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001834e4)?;
                c.g[V0][0] = 0x8;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_1834ec;
            }
        }
        'b_18352c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001834f8);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2738));
            call_ptr(c, 0x00317670, 0x001834f8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x00183504);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00183504)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[A1][0] = 0x1;
            if t {
                break 'b_18352c;
            } else {
                c.cycles += 7;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00183524);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00183524)?;
                c.g[V0][0] = 0x9;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                break 'b_18352c;
            }
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x00183538);
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x00186518,
            f: ee::s02::cFEState_OnActivate,
        }
        .call(c, 0x00183538)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_Render(self)`: tail wrapper around `UIState_Render`.
/// (0x001835a8; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[RA][0] = sx(0x001835b8);
        call_ptr(c, 0x0039e6b8, 0x001835b8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_OnChildEvent(self, item, msg)`: popup closed (msg 0x16). The cheat-char
/// popup (name hash `D_0045D780`) sets the player's cheat character when done; the keyboard popup
/// (hash 0) sets the player name when accepted and not empty. Then re-show the 3D character.
/// (0x001835f8; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1836f8: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A2][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A1][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00183620);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x00183620)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0018362c);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x0018362c)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 0x16;
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_1836f8;
            } else {
                'b_1836ec: {
                    'b_1836e8: {
                        'b_1836a4: {
                            c.cycles += 6;
                            c.g[A0][0] = 0x460000;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                            c.g[RA][0] = sx(0x0018364c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2880));
                            call_ptr(c, 0x00317670, 0x0018364c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                                break 'b_1836a4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                let t = c.g[V0][0] == 0u64;
                                c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                if t {
                                    break 'b_1836e8;
                                } else {
                                    c.cycles += 17;
                                    c.g[RA][0] = sx(0x00183668);
                                    call_ptr(c, 0x0014dc80, 0x00183668)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00183674);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x0014dd58, 0x00183674)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00183688);
                                    c.g[A2][0] = c.g[S1][0];
                                    call_ptr(c, 0x001474a8, 0x00183688)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0018369c);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0018369c)?;
                                    let t = true;
                                    c.g[A0][0] = c.g[S3][0];
                                    if t {
                                        break 'b_1836ec;
                                    } else {
                                        break 'b_1836a4;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_1836ec;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] == 0u64;
                            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                            if t {
                                break 'b_1836ec;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001836c0);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x00416810, 0x001836c0)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1836e8;
                                } else {
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x44));
                                    c.g[RA][0] = sx(0x001836d4);
                                    c.g[A2][0] = c.g[S0][0];
                                    call_ptr(c, 0x00147138, 0x001836d4)?;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001836e8);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001836e8)?;
                                    break 'b_1836e8;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                    break 'b_1836ec;
                }
                c.cycles += 3;
                c.g[RA][0] = sx(0x001836f4);
                c.g[A1][0] = 0x1;
                Callee {
                    addr: 0x001831d0,
                    f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive,
                }
                .call(c, 0x001836f4)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_1836f8;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRiderDetail_OnWidgetAction(self, item, msg)`: widget actions 5/6/7 of the
/// rider-detail screen: open the keyboard or cheat-character popup, switch the state, or open the
/// options state.
/// (0x00183710; every register, cycle and call of the generated body is kept.)
pub fn rider_detail_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_183a88: {
            'b_183a84: {
                'b_183a80: {
                    c.cycles += 7;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[V1][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_183a80;
                    } else {
                        'b_183a2c: {
                            'b_183a1c: {
                                'b_183a18: {
                                    'b_1839e0: {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[A0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        if t {
                                            break 'b_1839e0;
                                        } else {
                                            'b_183764: {
                                                'b_183750: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_183750;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_183764;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            if t {
                                                                break 'b_183a88;
                                                            } else {
                                                                break 'b_183750;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x7;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_183a2c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_183a88;
                                                    } else {
                                                        break 'b_183764;
                                                    }
                                                }
                                            }
                                            'b_1839a8: {
                                                'b_183998: {
                                                    'b_1838ec: {
                                                        c.cycles += 4;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.g[V0][0] = 0x8;
                                                        let t = c.g[A2][0] != c.g[V0][0];
                                                        c.g[A1][0] = 0x460000;
                                                        if t {
                                                            break 'b_1838ec;
                                                        } else {
                                                            c.cycles += 94;
                                                            c.g[A0][0] = 0x444;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x2728));
                                                            c.g[A2][0] = 0x100;
                                                            c.g[RA][0] = sx(0x00183788);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x00183788)?;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[T0][0] = 0xf;
                                                            c.g[A2][0] = c.g[S1][0];
                                                            c.g[A3][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001837a0);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb030, 0x001837a0)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0x8;
                                                            c.g[V0][0] = 0x37;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001837b8);
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x140),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x001cd088, 0x001837b8)?;
                                                            c.g[A2][0] = 0x1;
                                                            c.g[A3][0] = 0x2;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001837cc);
                                                            c.g[A1][0] = 0x4b;
                                                            call_ptr(c, 0x001ce3c8, 0x001837cc)?;
                                                            c.g[V0][0] = 0x1;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x58),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[RA][0] = sx(0x001837e0);
                                                            c.g[A1][0] = 0x1;
                                                            call_ptr(c, 0x001ce408, 0x001837e0)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0xe;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001837f4);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x001837f4)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x27;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183808);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183808)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x28;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x0018381c);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x0018381c)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x34;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183830);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183830)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x1a;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183844);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183844)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x19;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183858);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183858)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x35;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x0018386c);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x0018386c)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x40;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183880);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183880)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[A1][0] = 0x41;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183894);
                                                            c.g[A3][0] = 0x2;
                                                            call_ptr(c, 0x001ce3c8, 0x00183894)?;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[A3][0] = 0x2;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001838a8);
                                                            c.g[A1][0] = 0x42;
                                                            call_ptr(c, 0x001ce3c8, 0x001838a8)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001838b8);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x001838b8)?;
                                                            c.g[RA][0] = sx(0x001838c0);
                                                            call_ptr(c, 0x0014dc80, 0x001838c0)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[RA][0] = sx(0x001838cc);
                                                            c.g[A1][0] = 0x1;
                                                            call_ptr(c, 0x0014dd58, 0x001838cc)?;
                                                            c.g[A1][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            c.g[RA][0] = sx(0x001838d8);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x00147170, 0x001838d8)?;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x001838e4);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb418, 0x001838e4)?;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_183998;
                                                            } else {
                                                                break 'b_1838ec;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    if c.g[A2][0] != c.g[A0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        break 'b_1839a8;
                                                    } else {
                                                        c.cycles += 19;
                                                        c.g[RA][0] = sx(0x001838fc);
                                                        call_ptr(c, 0x0014dc80, 0x001838fc)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00183908);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x0014dd58, 0x00183908)?;
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[RA][0] = sx(0x00183914);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x00147488, 0x00183914)?;
                                                        c.g[RA][0] = sx(0x0018391c);
                                                        c.g[S0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0014dc80, 0x0018391c)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00183928);
                                                        c.g[A1][0] = 0xd;
                                                        call_ptr(c, 0x0014dd58, 0x00183928)?;
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00183938);
                                                        c.g[A2][0] = c.g[S0][0];
                                                        call_ptr(c, 0x001577a0, 0x00183938)?;
                                                        let t = (c.g[V0][0] as i64) <= 0;
                                                        c.g[A1][0] = 0x460000;
                                                        if t {
                                                            break 'b_183a18;
                                                        } else {
                                                            c.cycles += 22;
                                                            c.g[A0][0] = 0x1e4;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x2718));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00183954);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x00183954)?;
                                                            c.g[A3][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            c.g[T0][0] = c.g[S0][0];
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x0018396c);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            Callee { addr: 0x00182220, f: ee::s02::cFEStateCheatCharSelect_ctor }.call(c, 0x0018396c)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            c.g[RA][0] = sx(0x00183980);
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            call_ptr(c, 0x001a1cd0, 0x00183980)?;
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x15),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[RA][0] = sx(0x00183994);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x00183994)?;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            break 'b_183998;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001839a0);
                                                c.g[A1][0] = 0u64;
                                                Callee { addr: 0x001831d0, f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive }.call(c, 0x001839a0)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_183a1c;
                                                } else {
                                                    break 'b_1839a8;
                                                }
                                            }
                                            c.cycles += 9;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001839c4);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001839c4)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = c.g[V0][0];
                                            if t {
                                                break 'b_183a18;
                                            } else {
                                                c.cycles += 5;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001839d8);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f400, 0x001839d8)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_183a1c;
                                                } else {
                                                    break 'b_1839e0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00183a04);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00183a04)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_183a80;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x00183a18);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f400, 0x00183a18)?;
                                        break 'b_183a18;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_183a1c;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00183a24);
                            c.g[A1][0] = 0u64;
                            Callee {
                                addr: 0x001831d0,
                                f: ee::s02::cFEStateCharSetup_SetRiderPreviewActive,
                            }
                            .call(c, 0x00183a24)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_183a84;
                            } else {
                                break 'b_183a2c;
                            }
                        }
                        c.cycles += 21;
                        c.g[A1][0] = 0x460000;
                        c.g[A3][0] = 0u64;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                        c.g[A0][0] = 0x54;
                        c.g[RA][0] = sx(0x00183a44);
                        c.g[A2][0] = 0x100;
                        call_ptr(c, 0x00317d70, 0x00183a44)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00183a54);
                        c.g[A2][0] = 0x1;
                        Callee {
                            addr: 0x001887a0,
                            f: ee::s02::cFEStateOptions_ctor,
                        }
                        .call(c, 0x00183a54)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        c.g[A0][0] = 0xffffffffffffc0ff;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = 0u64;
                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                        c.g[A3][0] = 0u64;
                        c.g[V1][0] = c.g[V1][0] | 0x100;
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x00183a80);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f320, 0x00183a80)?;
                        break 'b_183a80;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_183a84;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_183a88;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateUberTrick_ShowCost(self, on)`: set four cost widgets of the screen (the first
/// screen of the state's list) to `on`.
/// (0x00184668; every register, cycle and call of the generated body is kept.)
pub fn uber_trick_show_cost(c: &mut Cpu) {
    guest(c, |c| {
        'b_1846c0: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00184684);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x00397840, 0x00184684)?;
            c.g[A0][0] = 0x460000;
            c.g[S0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184694);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2648));
            call_ptr(c, 0x00317670, 0x00184694)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001846a0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x001846a0)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1846c0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001846c0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001846c0)?;
                break 'b_1846c0;
            }
        }
        'b_1846f8: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x001846cc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2628));
            call_ptr(c, 0x00317670, 0x001846cc)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001846d8);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x001846d8)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_1846f8;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001846f8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001846f8)?;
                break 'b_1846f8;
            }
        }
        'b_184730: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x00184704);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2638));
            call_ptr(c, 0x00317670, 0x00184704)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00184710);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0039d860, 0x00184710)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184730;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184730);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184730)?;
                break 'b_184730;
            }
        }
        'b_184768: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[RA][0] = sx(0x0018473c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2618));
            call_ptr(c, 0x00317670, 0x0018473c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x00184748);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x00184748)?;
            c.g[A2][0] = c.g[V0][0];
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = c.g[S1][0];
            if t {
                break 'b_184768;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00184768);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00184768)?;
                break 'b_184768;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::DATA;
    use crate::game::main::loadscreens::tests::{
        oc736_fake_call, oc736_fixture, oc736_force, oc736_force_zero, FX_CTRL, FX_IFVT, FX_MAGIC,
    };
    use std::sync::atomic::AtomicUsize;

    /// The rider-detail state (`this`), the popup / widget `item`, the UI
    /// manager chain and the screen object the lookups find.
    pub(crate) const OC827_THIS: u32 = DATA + 0x40_0000;
    pub(crate) const OC827_ITEM: u32 = DATA + 0x40_1000;
    pub(crate) const OC827_MGR: u32 = DATA + 0x40_2000;
    pub(crate) const OC827_OBJ: u32 = DATA + 0x40_3000;
    pub(crate) const OC827_VT: u32 = DATA + 0x40_4000;
    pub(crate) const OC827_SCR: u32 = DATA + 0x40_5000;
    pub(crate) const OC827_SVT: u32 = DATA + 0x40_6000;
    /// What the fake `StrHash` answers for the cheat-char popup name.
    pub(crate) const OC827_HASH: u32 = 0x1234_5678;

    /// Gated fakes (gate: `FX_CTRL == FX_MAGIC`, written by `oc736_fixture`)
    /// for every callee with an override slot; with the gate off they run the
    /// body they replaced.
    macro_rules! oc827_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FX_CTRL) != FX_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc827_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc827_fakes!(
        (
            oc827_fake_317670,
            OC827_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc827_fake_397cc0,
            OC827_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc827_fake_39c870,
            OC827_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc827_fake_182db8,
            OC827_PREV_182DB8,
            ee::s02::OVERRIDE_00182DB8,
            ee::s02::f_00182db8__generated,
            0x0018_2db8
        ),
        (
            oc827_fake_14dc80,
            OC827_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc827_fake_14dd58,
            OC827_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc827_fake_147488,
            OC827_PREV_147488,
            ee::s01::OVERRIDE_00147488,
            ee::s01::f_00147488__generated,
            0x0014_7488
        ),
        (
            oc827_fake_1a0570,
            OC827_PREV_1A0570,
            ee::s03::OVERRIDE_001A0570,
            ee::s03::f_001a0570__generated,
            0x001a_0570
        ),
        (
            oc827_fake_1a0508,
            OC827_PREV_1A0508,
            ee::s03::OVERRIDE_001A0508,
            ee::s03::f_001a0508__generated,
            0x001a_0508
        ),
        (
            oc827_fake_28b180,
            OC827_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc827_fake_28f140,
            OC827_PREV_28F140,
            ee::s07::OVERRIDE_0028F140,
            ee::s07::f_0028f140__generated,
            0x0028_f140
        ),
        (
            oc827_fake_1577a0,
            OC827_PREV_1577A0,
            ee::s01::OVERRIDE_001577A0,
            ee::s01::f_001577a0__generated,
            0x0015_77a0
        ),
        (
            oc827_fake_39e6b8,
            OC827_PREV_39E6B8,
            ee::s13::OVERRIDE_0039E6B8,
            ee::s13::f_0039e6b8__generated,
            0x0039_e6b8
        ),
        (
            oc827_fake_39d860,
            OC827_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc827_fake_186518,
            OC827_PREV_186518,
            ee::s02::OVERRIDE_00186518,
            ee::s02::f_00186518__generated,
            0x0018_6518
        ),
        (
            oc827_fake_1474a8,
            OC827_PREV_1474A8,
            ee::s01::OVERRIDE_001474A8,
            ee::s01::f_001474a8__generated,
            0x0014_74a8
        ),
        (
            oc827_fake_147138,
            OC827_PREV_147138,
            ee::s01::OVERRIDE_00147138,
            ee::s01::f_00147138__generated,
            0x0014_7138
        ),
        (
            oc827_fake_416810,
            OC827_PREV_416810,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810
        ),
        (
            oc827_fake_1831d0,
            OC827_PREV_1831D0,
            ee::s02::OVERRIDE_001831D0,
            ee::s02::f_001831d0__generated,
            0x0018_31d0
        ),
        (
            oc827_fake_317d70,
            OC827_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc827_fake_1cd088,
            OC827_PREV_1CD088,
            ee::s04::OVERRIDE_001CD088,
            ee::s04::f_001cd088__generated,
            0x001c_d088
        ),
        (
            oc827_fake_1ce3c8,
            OC827_PREV_1CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc827_fake_1ce408,
            OC827_PREV_1CE408,
            ee::s04::OVERRIDE_001CE408,
            ee::s04::f_001ce408__generated,
            0x001c_e408
        ),
        (
            oc827_fake_147170,
            OC827_PREV_147170,
            ee::s01::OVERRIDE_00147170,
            ee::s01::f_00147170__generated,
            0x0014_7170
        ),
        (
            oc827_fake_1cb418,
            OC827_PREV_1CB418,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418
        ),
        (
            oc827_fake_1a1cd0,
            OC827_PREV_1A1CD0,
            ee::s03::OVERRIDE_001A1CD0,
            ee::s03::f_001a1cd0__generated,
            0x001a_1cd0
        ),
        (
            oc827_fake_39f400,
            OC827_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc827_fake_1887a0,
            OC827_PREV_1887A0,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x0018_87a0
        ),
        (
            oc827_fake_39f320,
            OC827_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039_f320
        ),
        (
            oc827_fake_39e318,
            OC827_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039_e318
        ),
        (
            oc827_fake_3180b8,
            OC827_PREV_3180B8,
            ee::s10::OVERRIDE_003180B8,
            ee::s10::f_003180b8__generated,
            0x0031_80b8
        ),
        (
            oc827_fake_398380,
            OC827_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc827_fake_15a478,
            OC827_PREV_15A478,
            ee::s01::OVERRIDE_0015A478,
            ee::s01::f_0015a478__generated,
            0x0015_a478
        ),
        (
            oc827_fake_397840,
            OC827_PREV_397840,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc827_fake_3978f0,
            OC827_PREV_3978F0,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
    );

    /// The common machine: fake hooks, the fakes' gate, the world, the
    /// back-end interface (`0x14dd58` answers `FX_IFACE`, whose vtable has the
    /// methods at +8 and +0x10 as fake calls) and a clean object page.
    pub(crate) fn oc827_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc827_install_fakes();
        let _ = oc736_fixture(c, seed, salt, calls);
        c.bus.write16(FX_IFVT + 0x10, 0x10 * (seed % 3) as u16);
        c.bus.write32(FX_IFVT + 0x14, fakes::FAKE + 0x30);
        c.bus.write8(OC827_THIS + 0x44, seed as u8);
        c.bus.write32(OC827_THIS + 0x10, OC827_MGR);
        c.bus.write32(OC827_MGR, OC827_OBJ);
        c.bus.write32(OC827_OBJ + 4, OC827_VT);
        c.bus.write32(OC827_SCR + 8, OC827_SVT);
        c.bus.write16(OC827_SVT + 0x48, 0x10 * (seed % 3) as u16);
        c.bus.write32(OC827_SVT + 0x4c, fakes::FAKE + 0x40);
    }

    /// `cFEStateRiderDetail_Init`: the screen is added or not (`0x397cc0`
    /// answers 0), the cheat-character count is zero or not.
    #[test]
    fn rider_detail_init_matches() {
        check(
            ee::s02::f_001832a8__generated,
            rider_detail_init,
            200,
            &|c, seed| {
                oc827_base(c, seed, 0x1832a8, 15);
                if seed % 2 == 0 {
                    oc736_force_zero(c, 0x0039_7cc0);
                }
                if (seed / 2) % 2 == 0 {
                    oc736_force_zero(c, 0x0015_77a0);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.pc = 0x0018_32a8;
            },
        );
    }

    /// `cFEStateRiderDetail_OnActivate`: the five widget lookups all find the
    /// screen object or all miss.
    #[test]
    fn rider_detail_on_activate_matches() {
        check(
            ee::s02::f_001833d0__generated,
            rider_detail_on_activate,
            200,
            &|c, seed| {
                oc827_base(c, seed, 0x1833d0, 16);
                if seed % 2 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC827_SCR);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = sx(seed.wrapping_mul(0x9e37_79b1));
                c.pc = 0x0018_33d0;
            },
        );
    }

    /// `cFEStateRiderDetail_Render`: one call to `UIState_Render`.
    #[test]
    fn rider_detail_render_matches() {
        check(
            ee::s02::f_001835a8__generated,
            rider_detail_render,
            60,
            &|c, seed| {
                oc827_base(c, seed, 0x1835a8, 1);
                c.g[A0][0] = sx(OC827_THIS);
                c.pc = 0x0018_35a8;
            },
        );
    }

    /// `cFEStateRiderDetail_OnChildEvent`: another message; the cheat-char
    /// popup (not done / done); the keyboard popup (name hash 0; not
    /// accepted, empty name, name); an unknown popup.
    #[test]
    fn rider_detail_on_child_event_matches() {
        check(
            ee::s02::f_001835f8__generated,
            rider_detail_on_child_event,
            300,
            &|c, seed| {
                oc827_base(c, seed, 0x1835f8, 9);
                oc736_force(c, 0x0031_7670, OC827_HASH);
                let msg = if seed % 7 == 0 { 0x15 } else { 0x16 };
                let item = OC827_ITEM;
                match (seed / 7) % 6 {
                    0 => {
                        c.bus.write32(item + 0xc, OC827_HASH);
                        c.bus.write32(item + 0x74, 0);
                    }
                    1 => {
                        c.bus.write32(item + 0xc, OC827_HASH);
                        c.bus.write32(item + 0x74, 1);
                        c.bus.write32(item + 0x70, 3);
                    }
                    2 => {
                        c.bus.write32(item + 0xc, 0);
                        c.bus.write32(item + 0x5c, 0);
                    }
                    3 => {
                        c.bus.write32(item + 0xc, 0);
                        c.bus.write32(item + 0x5c, 1);
                        oc736_force_zero(c, 0x0041_6810);
                    }
                    4 => {
                        c.bus.write32(item + 0xc, 0);
                        c.bus.write32(item + 0x5c, 1);
                        c.bus.write32(item + 0x74, 0x4142_4300);
                    }
                    _ => c.bus.write32(item + 0xc, 0x77),
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = sx(item);
                c.g[A2][0] = msg;
                c.pc = 0x0018_35f8;
            },
        );
    }

    /// `cFEStateRiderDetail_OnWidgetAction` by `seed % 8`: no item, msg 6,
    /// msg 5 with item state 8 / 6 / other, msg 7, msg below 5, msg above 7;
    /// the virtual calls answer 0 (the real leaf `0x1406c0`) or a pointer;
    /// `0x1577a0` answers 0 or non-zero.
    #[test]
    fn rider_detail_on_widget_action_matches() {
        check(
            ee::s02::f_00183710__generated,
            rider_detail_on_widget_action,
            320,
            &|c, seed| {
                oc827_base(c, seed, 0x183710, 25);
                let item = OC827_ITEM;
                let sel = seed % 8;
                let (msg, state) = match sel {
                    0 | 1 => (6, 3),
                    2 => (5, 8),
                    3 => (5, 6),
                    4 => (5, 3),
                    5 => (7, 3),
                    6 => (3, 3),
                    _ => (9, 3),
                };
                c.bus.write32(item + 0x18, state);
                let method = if (seed / 8) % 2 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE + 0x50
                };
                for off in [0x20u32, 0x28] {
                    c.bus.write16(OC827_VT + off, 0x10 * (seed % 3) as u16);
                    c.bus.write32(OC827_VT + off + 4, method);
                }
                if (seed / 16) % 2 == 0 {
                    oc736_force_zero(c, 0x0015_77a0);
                } else {
                    oc736_force(c, 0x0015_77a0, 1);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = if sel == 0 { 0 } else { sx(item) };
                c.g[A2][0] = msg;
                c.pc = 0x0018_3710;
            },
        );
    }

    /// `cFEStateUberTrick_ShowCost`: the screen has the four widgets or none.
    #[test]
    fn uber_trick_show_cost_matches() {
        check(
            ee::s02::f_00184668__generated,
            uber_trick_show_cost,
            200,
            &|c, seed| {
                oc827_base(c, seed, 0x184668, 16);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x0039_d860);
                } else {
                    oc736_force(c, 0x0039_d860, OC827_SCR);
                }
                c.g[A0][0] = sx(OC827_THIS);
                c.g[A1][0] = sx(seed.wrapping_mul(0x9e37_79b1) & 3);
                c.pc = 0x0018_4668;
            },
        );
    }
}
