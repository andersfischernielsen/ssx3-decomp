//! Staging module oc737 (wave wave242): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc737`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0018_2ec0,
        cfe_state_char_setup_update_rider_preview,
        "cFEStateCharSetup_UpdateRiderPreview",
    ),
    (
        0x0018_4520,
        cfe_state_uber_trick_update_cost,
        "cFEStateUberTrick_UpdateCost",
    ),
    (
        0x0019_d578,
        bolt_file_ensure_budget,
        "BoltFile_EnsureBudget",
    ),
    (0x0019_da10, bolt_file_acquire, "BoltFile_Acquire"),
    (0x001e_95a0, hud_draw_icon_text, "HUD_DrawIconText"),
    (0x001f_14b0, hud_draw_wobble_text, "HUD_DrawWobbleText"),
];

/// `cFEStateCharSetup_UpdateRiderPreview(self)` (0x182ec0): looks up the
/// human rider of the setup screen (`self+0x44`, through the world's
/// rider manager at `world+0x7c`), and when its textures are loaded and the
/// 3D preview is on (`self+0x4c`), sets the rider's pose and board pose
/// (three quaternion multiplies of the fixed preview rotation), and on the
/// first frame (`self+0x48`) resets and plays the intro animation and
/// speaks; finally shows the rider.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn cfe_state_char_setup_update_rider_preview(c: &mut Cpu) {
    guest(c, |c| {
        'b_182f2c: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S7][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
            c.g[A1][0] = c.lb((c.g[S7][0] as u32).wrapping_add(0x44));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
            call_ptr(c, 0x001a0548, 0x00182f04)?;
            c.g[S6][0] = c.g[V0][0];
            c.g[V0][0] = c.lw(c.g[S6][0] as u32);
            c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V1][0] = 0u64;
            if t {
                break 'b_182f2c;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb8));
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_182f2c;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xcb4));
                    c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_182f2c;
                }
            }
        }
        'b_1831a4: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
            if t {
                break 'b_1831a4;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x4c));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                if t {
                    break 'b_1831a4;
                } else {
                    'b_183198: {
                        c.cycles += 134;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 20);
                        c.g[AT][0] = 0x43080000;
                        c.mtc1(1, 0);
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                        c.g[AT][0] = 0xffffffffc37a0000;
                        c.mtc1(1, 1);
                        c.g[S0][0] = 0x500000;
                        c.g[AT][0] = 0xffffffffc2a40000;
                        c.mtc1(1, 2);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xec0));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = 0x500000;
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xeb0));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                        c.g[S4][0] = 0x500000;
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x3c));
                        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_sub(0xea0));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(2, c.g[SP][0] as u32);
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2c));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
                        call_ptr(c, 0x0031be50, 0x00182fb0)?;
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x78));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x7c));
                        c.lwc1(2, (c.g[S0][0] as u32).wrapping_sub(0xec0));
                        c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(2, 0, 2);
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.mul_s(0, 0, 4);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x5c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.mtc1(0, 12);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        call_ptr(c, 0x0031be50, 0x0018302c)?;
                        c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x78));
                        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x84));
                        c.lwc1(2, (c.g[S2][0] as u32).wrapping_sub(0xeb0));
                        c.lwc1(4, (c.g[S3][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(2, 0, 2);
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x7c));
                        c.mul_s(0, 0, 4);
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5a08));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x0031be50, 0x001830a8)?;
                        c.lwc1(1, (c.g[S5][0] as u32).wrapping_add(0x8));
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[A0][0] = c.g[S6][0];
                        c.lwc1(3, (c.g[S4][0] as u32).wrapping_sub(0xea0));
                        c.g[A1][0] = c.g[S0][0];
                        c.lwc1(4, (c.g[S5][0] as u32).wrapping_add(0x4));
                        c.mul_s(1, 0, 1);
                        c.mul_s(3, 0, 3);
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                        c.mul_s(0, 0, 4);
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x4c));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
                        c.vu0().opmula(4, 5);
                        c.vu0().opmsub(6, 5, 4);
                        c.vu0().mula(0b1110, 4, Src::Bc(5, W));
                        c.vu0().madda(0b1110, 5, Src::Bc(4, W));
                        c.vu0().suba(0b0001, 7, Src::Bc(7, X));
                        c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
                        c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
                        c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
                        c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        call_ptr(c, 0x0019f3e8, 0x00183124)?;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5a04));
                        c.g[A1][0] = c.g[S0][0];
                        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x3c));
                        c.g[A0][0] = c.g[S6][0];
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        call_ptr(c, 0x0019f548, 0x0018314c)?;
                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x48));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S6][0];
                        if t {
                            break 'b_183198;
                        } else {
                            'b_18318c: {
                                c.cycles += 3;
                                c.g[S0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0xc));
                                if c.g[S0][0] == 0u64 {
                                    c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), 0u64);
                                    break 'b_18318c;
                                } else {
                                    c.cycles += 10;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00311a50, 0x0018316c)?;
                                    c.g[AT][0] = 0xffffffffbf800000;
                                    c.mtc1(1, 12);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x1b3;
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x003128e8, 0x00183184)?;
                                    c.sw((c.g[S6][0] as u32).wrapping_add(0xccc), 0u64);
                                    c.g[A0][0] = c.g[S6][0];
                                    break 'b_18318c;
                                }
                            }
                            c.cycles += 3;
                            c.sw((c.g[S7][0] as u32).wrapping_add(0x48), 0u64);
                            call_ptr(c, 0x001a0358, 0x00183194)?;
                            c.g[A0][0] = c.g[S6][0];
                            break 'b_183198;
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0019e538, 0x001831a0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                    break 'b_1831a4;
                }
            }
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x120));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
        c.pc = t;
        Ok(())
    });
}

/// `cFEStateUberTrick_UpdateCost(self)` (0x184520): refreshes the two text
/// objects of the uber-trick screen: the cost of the selected trick
/// (`getDollarString(table[trick].cost)` into "cost_num") and the player's
/// money (into "have_num"), each only when the screen has that object.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn cfe_state_uber_trick_update_cost(c: &mut Cpu) {
    guest(c, |c| {
        'b_1845c8: {
            c.cycles += 36;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4c));
            c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
            c.g[S4][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x18));
            c.g[S0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0014dc80, 0x0018455c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x6;
            call_ptr(c, 0x0014dd58, 0x00184568)?;
            c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[SP][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014ff90, 0x0018457c)?;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x00397840, 0x00184588)?;
            c.g[A0][0] = 0x460000;
            c.g[S3][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2648));
            call_ptr(c, 0x00317670, 0x00184598)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0039d860, 0x001845a4)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = sx((c.g[S4][0] as u32) << 3);
            if t {
                break 'b_1845c8;
            } else {
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x6));
                call_ptr(c, 0x00198af0, 0x001845bc)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a0c50, 0x001845c8)?;
                break 'b_1845c8;
            }
        }
        'b_18464c: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2638));
            call_ptr(c, 0x00317670, 0x001845d4)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001845e0)?;
            c.g[S2][0] = c.g[V0][0];
            if c.g[S2][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_18464c;
            } else {
                c.cycles += 24;
                call_ptr(c, 0x0014dc80, 0x001845f4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00184600)?;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00147488, 0x0018460c)?;
                c.g[S0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
                call_ptr(c, 0x0014dc80, 0x00184618)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xb;
                call_ptr(c, 0x0014dd58, 0x00184624)?;
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x00150928, 0x00184634)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x00198af0, 0x0018463c)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a0c50, 0x00184648)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_18464c;
            }
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    });
}

/// `BoltFile_EnsureBudget(mgr, bank, size)` (0x19d578): keeps the bolt file
/// cache under its 9999999-byte budget before `size` more bytes are loaded:
/// when `mem_used + size` is over, frees unreferenced idle slots of the other
/// banks first, then the oldest (smallest stamp) idle slots of `bank` until
/// the budget holds or none is left. Slots are 0x120 bytes, 256 per bank, 10
/// banks 0x12000 apart.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn bolt_file_ensure_budget(c: &mut Cpu) {
    guest(c, |c| {
        'b_19d704: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.g[V0][0] = 0xb0000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[V1][0] = 0x980000;
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S4][0] = c.g[A0][0];
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S7][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = c.g[V1][0] | 0x967f;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4000));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S7][0] as u32));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            let t = c.g[V1][0] == 0u64;
            c.g[FP][0] = c.g[A1][0];
            if t {
                break 'b_19d704;
            } else {
                c.cycles += 2;
                c.g[S3][0] = 0u64;
                c.g[S6][0] = 0u64;
                'l_19d5d8: loop {
                    'b_19d64c: {
                        c.cycles += 2;
                        let t = c.g[S3][0] == c.g[FP][0];
                        c.g[V0][0] = 0xb0000;
                        if t {
                            break 'b_19d64c;
                        } else {
                            c.cycles += 6;
                            c.g[S1][0] = 0x980000;
                            c.g[S2][0] = 0u64;
                            c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[S1][0] = c.g[S1][0] | 0x967f;
                            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S6][0] as u32));
                            'l_19d5f8: loop {
                                'b_19d640: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x104));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                        break 'b_19d640;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                                        if c.g[V0][0] != 0u64 {
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                            break 'b_19d640;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                            if c.g[V0][0] != 0u64 {
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                break 'b_19d640;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A1][0] = c.g[S3][0];
                                                c.g[A2][0] = c.g[S2][0];
                                                call_ptr(c, 0x0019d4b8, 0x0019d62c)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S5][0] as u32).wrapping_add(0x4000));
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                                c.g[V0][0] = ((c.g[S1][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[S2][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                if t {
                                                    break 'b_19d704;
                                                } else {
                                                    break 'b_19d640;
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[S2][0] as i64) < 256i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x120));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019d5f8);
                                    }
                                    continue 'l_19d5f8;
                                } else {
                                    break 'b_19d64c;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[V1][0] = 0x10000;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = c.g[V1][0] | 0x2000;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x0019d5d8);
                        }
                        continue 'l_19d5d8;
                    } else {
                        c.cycles += 7;
                        c.mult(0, 18, 30, 3);
                        c.g[V0][0] = 0xb0000;
                        c.g[S0][0] = 0x980000;
                        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[S0][0] = c.g[S0][0] | 0x967f;
                        c.g[A2][0] = 0xffffffffffffffff;
                        'l_19d680: loop {
                            c.cycles += 4;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S2][0] as u32));
                            'l_19d690: loop {
                                'b_19d6d4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x104));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        break 'b_19d6d4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_19d6d4;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x114));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                break 'b_19d6d4;
                                            } else {
                                                'b_19d6c8: {
                                                    c.cycles += 2;
                                                    let t = (c.g[A2][0] as i64) < 0;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x11c)
                                                        );
                                                    if t {
                                                        break 'b_19d6c8;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            (c.g[V1][0] < c.g[A3][0]) as u64;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_19d6d4;
                                                        } else {
                                                            break 'b_19d6c8;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A3][0] = c.g[V1][0];
                                                c.g[A2][0] = c.g[A1][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                                break 'b_19d6d4;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[A1][0] as i64) < 256i64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x120));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x0019d690);
                                    }
                                    continue 'l_19d690;
                                } else {
                                    c.cycles += 2;
                                    let t = (c.g[A2][0] as i64) < 0;
                                    c.g[A0][0] = c.g[S4][0];
                                    if t {
                                        break 'b_19d704;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A1][0] = c.g[FP][0];
                                        call_ptr(c, 0x0019d4b8, 0x0019d6f0)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4000));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                                        c.g[V0][0] =
                                            ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0019d680);
                                            }
                                            continue 'l_19d680;
                                        } else {
                                            break 'b_19d704;
                                        }
                                    }
                                }
                            }
                        }
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
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    });
}

/// `BoltFile_Acquire(mgr, bank, name, file_type, async, deferred)` (0x19da10):
/// finds the cached slot of `name` in `bank` (or takes a free one, freeing it
/// first and copying the name), stamps it (0 when deferred, else the bank's
/// running counter), bumps its use count and starts or schedules the load
/// (async loads wait as state 3 when load threads exist). Returns the slot
/// index.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn bolt_file_acquire(c: &mut Cpu) {
    guest(c, |c| {
        'b_19dbf4: {
            'b_19dbf0: {
                'b_19db28: {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S6][0] = c.g[A2][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S4][0] = c.g[T1][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S5][0] = c.g[T0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S7][0] = c.g[A3][0];
                    call_ptr(c, 0x0019d428, 0x0019da54)?;
                    c.g[S3][0] = c.g[V0][0];
                    let t = (c.g[S3][0] as i64) < 0;
                    c.g[V0][0] = 0x10000;
                    if t {
                        break 'b_19db28;
                    } else {
                        'b_19daa8: {
                            'b_19da84: {
                                c.cycles += 7;
                                c.g[V1][0] = 0x120;
                                c.g[V0][0] = c.g[V0][0] | 0x2000;
                                c.mult(0, 3, 19, 3);
                                c.mult(0, 4, 18, 2);
                                c.g[V0][0] =
                                    sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                let t = c.g[S4][0] == 0u64;
                                c.g[S0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                if t {
                                    break 'b_19da84;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                                    if t {
                                        break 'b_19daa8;
                                    } else {
                                        break 'b_19da84;
                                    }
                                }
                            }
                            c.cycles += 9;
                            c.g[V0][0] = 0xb0000;
                            c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                            c.g[V0][0] = c.g[V0][0] | 0x4008;
                            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                            break 'b_19daa8;
                        }
                        'b_19db1c: {
                            'b_19db18: {
                                'b_19dac8: {
                                    c.cycles += 2;
                                    if c.g[S5][0] == 0u64 {
                                        c.g[V1][0] = c.lw(c.g[S0][0] as u32);
                                        break 'b_19dac8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                                        if c.g[V0][0] != 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                                            break 'b_19db1c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x1;
                                            let t = true;
                                            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                            if t {
                                                break 'b_19db18;
                                            } else {
                                                break 'b_19dac8;
                                            }
                                        }
                                    }
                                }
                                'b_19db0c: {
                                    'b_19daec: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10c), 0u64);
                                        if t {
                                            break 'b_19daec;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x108));
                                            c.g[A1][0] = 0u64;
                                            c.g[A2][0] = c.g[SP][0];
                                            call_ptr(c, 0x003df9d8, 0x0019dae4)?;
                                            let t = true;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_19db0c;
                                            } else {
                                                break 'b_19daec;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_19db0c;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A2][0] = c.g[S3][0];
                                        call_ptr(c, 0x0019d8b8, 0x0019db08)?;
                                        c.g[V0][0] = 0x1;
                                        break 'b_19db0c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[V1][0] = 0xffffffffffffffff;
                                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x108), c.g[V1][0]);
                                break 'b_19db18;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                            break 'b_19db1c;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x114), c.g[V0][0]);
                        if t {
                            break 'b_19dbf0;
                        } else {
                            break 'b_19db28;
                        }
                    }
                }
                'b_19db94: {
                    'b_19db74: {
                        c.cycles += 17;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S2][0];
                        call_ptr(c, 0x0019d3b0, 0x0019db34)?;
                        c.g[V1][0] = 0x10000;
                        c.g[S3][0] = c.g[V0][0];
                        c.g[V1][0] = c.g[V1][0] | 0x2000;
                        c.g[V0][0] = 0x120;
                        c.mult(0, 4, 18, 3);
                        c.mult(0, 2, 19, 2);
                        c.g[A1][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x0019d4b8, 0x0019db64)?;
                        let t = c.g[S4][0] == 0u64;
                        c.g[V0][0] = 0xb0000;
                        if t {
                            break 'b_19db74;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), 0u64);
                            if t {
                                break 'b_19db94;
                            } else {
                                break 'b_19db74;
                            }
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = c.g[V0][0] | 0x4008;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x11c), c.g[V1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
                    break 'b_19db94;
                }
                'b_19dbec: {
                    'b_19dbe8: {
                        c.cycles += 9;
                        c.g[A1][0] = c.g[S6][0];
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x004166f4, 0x0019dba0)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x110), c.g[S7][0]);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x10c), c.g[S5][0]);
                        let t = c.g[S4][0] != 0u64;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x114), c.g[V1][0]);
                        if t {
                            break 'b_19dbe8;
                        } else {
                            'b_19dbd0: {
                                c.cycles += 2;
                                let t = c.g[S5][0] == 0u64;
                                c.g[V0][0] = 0xb0000;
                                if t {
                                    break 'b_19dbd0;
                                } else {
                                    c.cycles += 4;
                                    c.g[V0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4004));
                                    let t = (c.g[V1][0] as i64) > 0;
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_19dbec;
                                    } else {
                                        break 'b_19dbd0;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x0019d8b8, 0x0019dbe0)?;
                            let t = true;
                            c.g[V0][0] = c.g[S3][0];
                            if t {
                                break 'b_19dbf4;
                            } else {
                                break 'b_19dbe8;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x3;
                    break 'b_19dbec;
                }
                c.cycles += 1;
                c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                break 'b_19dbf0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
            break 'b_19dbf4;
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    });
}

/// `HUD_DrawIconText(text, pos, scale, color, align_x, align_y)` (0x1e95a0):
/// draws a line of mixed text and icon elements (even elements are strings,
/// odd ones HUD pictures): scales and tints the font, aligns the line around
/// `pos`, then draws each string with `cFont_text` and each picture as a
/// textured quad through the graphics manager's vtable. Restores the font.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn hud_draw_icon_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e9a00: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.g[S0][0] = c.g[A1][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
            let t = c.g[V0][0] == 0u64;
            c.g[S4][0] = c.g[A2][0];
            if t {
                break 'b_1e9a00;
            } else {
                'b_1e9708: {
                    'b_1e96e8: {
                        c.cycles += 56;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.lwc1(5, c.g[A3][0] as u32);
                        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x57));
                        c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x50));
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x5f));
                        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x58));
                        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(2, c.g[SP][0] as u32);
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.ldr(2, c.g[SP][0] as u32);
                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x1f));
                        c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x4));
                        c.lwc1(4, c.g[S4][0] as u32);
                        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x10));
                        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.mul_s(3, 3, 4);
                        c.mul_s(1, 1, 2);
                        c.mul_s(0, 0, 5);
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
                        c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x34));
                        c.mul_s(0, 0, 3);
                        c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.mul_s(2, 2, 1);
                        c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x3c));
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.ldl(3, (c.g[A3][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[A3][0] as u32);
                        c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x47));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x40));
                        c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x4f));
                        c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
                        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x1f));
                        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x57));
                        c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x50));
                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x5f));
                        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x58));
                        let t = c.g[T0][0] != 0u64;

                        if t {
                            break 'b_1e96e8;
                        } else {
                            c.cycles += 10;
                            c.lwc1(2, c.g[S4][0] as u32);
                            c.lwc1(1, c.g[S1][0] as u32);
                            c.g[AT][0] = 0x3f000000;
                            c.mtc1(1, 3);
                            c.mul_s(1, 1, 2);
                            c.lwc1(0, c.g[S0][0] as u32);
                            c.mul_s(1, 1, 3);
                            c.sub_s(0, 0, 1);
                            let t = true;
                            c.swc1(0, c.g[S0][0] as u32);
                            if t {
                                break 'b_1e9708;
                            } else {
                                break 'b_1e96e8;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = (c.g[T0][0] as i64) <= 0;

                    if t {
                        break 'b_1e9708;
                    } else {
                        c.cycles += 6;
                        c.lwc1(0, c.g[S4][0] as u32);
                        c.lwc1(1, c.g[S1][0] as u32);
                        c.lwc1(2, c.g[S0][0] as u32);
                        c.mul_s(1, 1, 0);
                        c.sub_s(2, 2, 1);
                        c.swc1(2, c.g[S0][0] as u32);
                        break 'b_1e9708;
                    }
                }
                'b_1e975c: {
                    'b_1e9758: {
                        'b_1e9738: {
                            c.cycles += 2;
                            let t = c.g[T1][0] != 0u64;

                            if t {
                                break 'b_1e9738;
                            } else {
                                c.cycles += 10;
                                c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0x4));
                                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                                c.g[AT][0] = 0x3f000000;
                                c.mtc1(1, 3);
                                c.mul_s(1, 1, 2);
                                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                                c.mul_s(1, 1, 3);
                                c.sub_s(0, 0, 1);
                                let t = true;
                                c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                                if t {
                                    break 'b_1e9758;
                                } else {
                                    break 'b_1e9738;
                                }
                            }
                        }
                        c.cycles += 2;
                        if (c.g[T1][0] as i64) <= 0 {
                            c.lwc1(3, (c.g[S4][0] as u32).wrapping_add(0x4));
                            break 'b_1e975c;
                        } else {
                            c.cycles += 6;
                            c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
                            c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
                            c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                            c.mul_s(1, 1, 0);
                            c.sub_s(2, 2, 1);
                            c.swc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                            break 'b_1e9758;
                        }
                    }
                    c.cycles += 1;
                    c.lwc1(3, (c.g[S4][0] as u32).wrapping_add(0x4));
                    break 'b_1e975c;
                }
                c.cycles += 13;
                c.g[V0][0] = 0x3;
                c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 4);
                c.g[T1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.mul_s(0, 0, 3);
                c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0xc));
                c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0xffffffffffffffff;
                c.mul_s(22, 1, 3);
                c.mul_s(0, 0, 4);
                c.add_s(21, 2, 0);
                'l_1e9790: loop {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[T0][0] = sx((c.g[S5][0] as u32).wrapping_add(0xc0));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001e9790);
                        }
                        continue 'l_1e9790;
                    } else {
                        c.cycles += 23;
                        c.g[AT][0] = 0x43000000;
                        c.mtc1(1, 1);
                        c.g[V1][0] = c.g[S5][0];
                        c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                        c.lwc1(4, (c.g[A3][0] as u32).wrapping_add(0xc));
                        c.lwc1(2, c.g[A3][0] as u32);
                        c.mul_s(0, 0, 1);
                        c.lwc1(3, (c.g[A3][0] as u32).wrapping_add(0x4));
                        c.mul_s(4, 4, 1);
                        c.mul_s(2, 2, 1);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 5);
                        c.mul_s(3, 3, 1);
                        c.cvt_w_s(1, 0);
                        c.mfc1(7, 1);
                        c.cvt_w_s(1, 2);
                        c.mfc1(6, 1);
                        c.cvt_w_s(1, 3);
                        c.mfc1(5, 1);
                        c.cvt_w_s(0, 4);
                        c.mfc1(4, 0);
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                        'l_1e9808: loop {
                            c.cycles += 8;
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A3][0]);
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                            c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x8));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                                if event_due(c) {
                                    deliver_event_here(c, 0x001e9808);
                                }
                                continue 'l_1e9808;
                            } else {
                                'b_1e99b0: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.g[S2][0] = c.g[T1][0];
                                    let t = (c.g[V0][0] as i64) <= 0;
                                    c.g[S3][0] = 0u64;
                                    if t {
                                        break 'b_1e99b0;
                                    } else {
                                        c.cycles += 2;
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 20);
                                        'l_1e9840: loop {
                                            'b_1e99a0: {
                                                'b_1e999c: {
                                                    'b_1e9890: {
                                                        c.cycles += 4;
                                                        c.g[V0][0] = c.g[S3][0] ^ 0x1;
                                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[T0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            break 'b_1e9890;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A1][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.lhu(c.g[A1][0] as u32);
                                                            if c.g[V0][0] == 0u64 {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x18));
                                                                break 'b_1e99a0;
                                                            } else {
                                                                c.cycles += 12;
                                                                c.lwc1(
                                                                    13,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.add_s(13, 13, 22);
                                                                c.lwc1(12, c.g[S0][0] as u32);
                                                                call_ptr(
                                                                    c, 0x00391e30, 0x001e9874,
                                                                )?;
                                                                c.lwc1(2, c.g[S4][0] as u32);
                                                                c.lwc1(0, c.g[S2][0] as u32);
                                                                c.lwc1(1, c.g[S0][0] as u32);
                                                                c.mul_s(0, 0, 2);
                                                                c.add_s(1, 1, 0);
                                                                let t = true;
                                                                c.swc1(1, c.g[S0][0] as u32);
                                                                if t {
                                                                    break 'b_1e999c;
                                                                } else {
                                                                    break 'b_1e9890;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 67;
                                                    c.g[A1][0] = 0x4;
                                                    c.lwc1(1, c.g[S4][0] as u32);
                                                    c.g[A2][0] = c.g[S5][0];
                                                    c.lwc1(
                                                        4,
                                                        (c.g[T0][0] as u32).wrapping_add(0xc),
                                                    );
                                                    c.g[A3][0] = 0u64;
                                                    c.lwc1(
                                                        3,
                                                        (c.g[T0][0] as u32).wrapping_add(0x10),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[S4][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.mul_s(4, 4, 1);
                                                    c.lwc1(5, c.g[S0][0] as u32);
                                                    c.mul_s(3, 3, 0);
                                                    c.g[AT][0] = 0x3f000000;
                                                    c.mtc1(1, 2);
                                                    c.swc1(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                    );
                                                    c.add_s(4, 5, 4);
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0xf4),
                                                    );
                                                    c.mul_s(2, 3, 2);
                                                    c.lwc1(
                                                        0,
                                                        (c.g[T0][0] as u32).wrapping_add(0x14),
                                                    );
                                                    c.sub_s(2, 21, 2);
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[T0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.add_s(3, 2, 3);
                                                    c.swc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x34),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[T0][0] as u32).wrapping_add(0x1c),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[T0][0] as u32).wrapping_add(0x18),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x64),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[T0][0] as u32).wrapping_add(0x14),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x90),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[T0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x94),
                                                    );
                                                    c.lwc1(
                                                        0,
                                                        (c.g[T0][0] as u32).wrapping_add(0x1c),
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc0),
                                                    );
                                                    c.lwc1(
                                                        1,
                                                        (c.g[T0][0] as u32).wrapping_add(0x20),
                                                    );
                                                    c.swc1(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x108),
                                                        0u64,
                                                    );
                                                    c.swc1(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x104),
                                                    );
                                                    c.swc1(
                                                        20,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10c),
                                                    );
                                                    c.swc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0xc4),
                                                    );
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.swc1(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x108),
                                                        0u64,
                                                    );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.swc1(
                                                        5,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x108),
                                                        0u64,
                                                    );
                                                    c.swc1(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x104),
                                                    );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                    c.lq(
                                                        3,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.swc1(
                                                        4,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x108),
                                                        0u64,
                                                    );
                                                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                                    c.lq(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x100),
                                                    );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x854)
                                                        );
                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0xe84)
                                                        );
                                                    c.sh(
                                                        (c.g[V0][0] as u32).wrapping_add(0x10),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[T0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x854)
                                                        );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[T0][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V0][0] as u32).wrapping_add(0x238)
                                                        );
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x23c)
                                                        );
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001e998c)?;
                                                    c.lwc1(0, c.g[S0][0] as u32);
                                                    c.lwc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0xf0),
                                                    );
                                                    c.add_s(0, 0, 1);
                                                    c.swc1(0, c.g[S0][0] as u32);
                                                    break 'b_1e999c;
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                break 'b_1e99a0;
                                            }
                                            c.cycles += 4;
                                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                            c.g[V0][0] =
                                                ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001e9840);
                                                }
                                                continue 'l_1e9840;
                                            } else {
                                                break 'b_1e99b0;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 20;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 0);
                                c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(3, c.g[SP][0] as u32);
                                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x57));
                                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x50));
                                c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x5f));
                                c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x58));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x30));
                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x34));
                                c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x38));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xf4));
                                c.mul_s(0, 0, 1);
                                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x3c));
                                break 'b_1e9a00;
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.pc = t;
        Ok(())
    });
}

/// `HUD_DrawWobbleText(a0, str, font, x, y, amp)` (0x1f14b0): draws `str`
/// one character at a time, each offset by an amount derived from its glyph
/// and distance from the centre (the "exploding" HUD text), advancing by the
/// glyph's width and offset.
#[allow(unused_labels, unused_variables, unused_assignments)]
pub fn hud_draw_wobble_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f1690: {
            c.cycles += 29;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S3][0] = c.g[A1][0];
            c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
            c.mov_s(22, 14);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.mov_s(21, 12);
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.mov_s(20, 13);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[A0][0] = c.g[S3][0];
            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x38));
            c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x3c));
            c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x34));
            c.div_s(1, 1, 3);
            c.div_s(0, 0, 2);
            c.swc1(1, c.g[SP][0] as u32);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
            call_ptr(c, 0x00416810, 0x001f1518)?;
            c.g[S2][0] = c.g[V0][0];
            let t = (c.g[S2][0] as i64) <= 0;
            c.sb((c.g[SP][0] as u32).wrapping_add(0x11), 0u64);
            if t {
                break 'b_1f1690;
            } else {
                c.cycles += 1;
                c.g[S4][0] = 0xc;
                'l_1f1528: loop {
                    'b_1f1560: {
                        'b_1f155c: {
                            'b_1f154c: {
                                c.cycles += 7;
                                c.g[V0][0] =
                                    sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                                c.g[V1][0] = sx(((c.g[S2][0] as u32) as i32 >> 1) as u32);
                                c.g[A0][0] = c.lbu(c.g[V0][0] as u32);
                                c.g[A1][0] =
                                    sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                                c.g[A3][0] = 0u64;
                                let t = c.g[A1][0] != 0u64;
                                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                                if t {
                                    break 'b_1f154c;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1f155c;
                                    } else {
                                        break 'b_1f154c;
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = (c.g[A1][0] as i64) >= 0;
                            c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_1f1560;
                            } else {
                                c.cycles += 2;
                                c.g[A3][0] = 0x1;
                                c.g[A1][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                                break 'b_1f155c;
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_1f1560;
                    }
                    'b_1f15a8: {
                        c.cycles += 17;
                        c.g[A0][0] = sx((c.g[A1][0] as u32) << 1);
                        c.g[V1][0] = c.g[V0][0] & 0x7f;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
                        c.g[V0][0] = c.g[V0][0] & 0xf;
                        c.mult(0, 4, 4, 3);
                        c.mult(0, 2, 5, 2);
                        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        c.g[V0][0] = c.g[V0][0] & 0x7f;
                        c.g[V1][0] = c.g[V1][0] & 0x7f;
                        c.mtc1(2, 1);
                        c.cvt_s_w(1, 1);
                        c.mtc1(3, 0);
                        c.cvt_s_w(0, 0);
                        let t = c.g[A3][0] == 0u64;
                        c.neg_s(0, 0);
                        if t {
                            break 'b_1f15a8;
                        } else {
                            c.cycles += 1;
                            c.neg_s(1, 1);
                            break 'b_1f15a8;
                        }
                    }
                    'b_1f166c: {
                        'b_1f1668: {
                            'b_1f15e4: {
                                c.cycles += 12;
                                c.mul_s(12, 21, 1);
                                c.g[A0][0] = c.g[S0][0];
                                c.mul_s(13, 21, 0);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.add_s(12, 20, 12);
                                c.add_s(13, 22, 13);
                                call_ptr(c, 0x00391cb0, 0x001f15c4)?;
                                c.g[T1][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1f15e4;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0u64;
                                    let t = true;
                                    c.lwc1(1, c.g[SP][0] as u32);
                                    if t {
                                        break 'b_1f1668;
                                    } else {
                                        break 'b_1f15e4;
                                    }
                                }
                            }
                            'b_1f1614: {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T1][0] as i64)) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                                    break 'b_1f1614;
                                } else {
                                    c.cycles += 8;
                                    c.mult(0, 9, 9, 20);
                                    c.mult(0, 10, 10, 20);
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                    c.lwc1(1, c.g[SP][0] as u32);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[T1][0] as u32));
                                    let t = true;
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                                    if t {
                                        break 'b_1f1668;
                                    } else {
                                        break 'b_1f1614;
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[T2][0] as u32));
                            c.lwc1(1, c.g[SP][0] as u32);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                            c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[A3][0] = 0xc;
                            c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                            'l_1f1630: loop {
                                c.cycles += 6;
                                c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
                                c.mult(0, 3, 5, 7);
                                c.g[V0][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                                c.g[V1][0] = c.lhu(c.g[V0][0] as u32);
                                if c.g[T1][0] == c.g[V1][0] {
                                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x2));
                                    break 'b_1f166c;
                                } else {
                                    'b_1f1658: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_1f1658;
                                        } else {
                                            c.cycles += 1;
                                            c.g[A2][0] = c.g[A1][0];
                                            break 'b_1f1658;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] =
                                        sx((c.g[T2][0] as u32).wrapping_add(c.g[A2][0] as u32));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001f1630);
                                        }
                                        continue 'l_1f1630;
                                    } else {
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1f1668;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x2));
                        break 'b_1f166c;
                    }
                    c.cycles += 9;
                    c.g[S1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x9));
                    c.g[A0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.mtc1(2, 0);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 1);
                    let t = c.g[A0][0] != 0u64;
                    c.add_s(20, 20, 0);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001f1528);
                        }
                        continue 'l_1f1528;
                    } else {
                        break 'b_1f1690;
                    }
                }
            }
        }
        c.cycles += 11;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x88));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{fill, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up (clear of the fake log).
    const OC737_FX_CTRL: u32 = DATA + 0x20_0000;
    const OC737_FX_MAGIC: u32 = 0x4f43_3737;
    const OC737_FX_SELF: u32 = DATA + 0x21_0000;
    const OC737_FX_WORLD: u32 = DATA + 0x22_0000;
    const OC737_FX_OBJ: u32 = DATA + 0x23_0000;
    const OC737_FX_RIDER: u32 = DATA + 0x24_0000;
    const OC737_FX_MM: u32 = DATA + 0x25_0000;
    const OC737_FX_TM: u32 = DATA + 0x26_0000;
    const OC737_FX_P1: u32 = DATA + 0x27_0000;
    const OC737_FX_P2: u32 = DATA + 0x28_0000;
    const OC737_FX_TBL: u32 = DATA + 0x29_0000;
    const OC737_FX_TXT1: u32 = DATA + 0x2a_0000;
    const OC737_FX_TXT2: u32 = DATA + 0x2b_0000;
    const OC737_FX_MGR: u32 = DATA + 0x30_0000;
    const OC737_FX_NAME: u32 = DATA + 0x3c_0000;
    const OC737_FX_HTXT: u32 = DATA + 0x40_0000;
    const OC737_FX_HFONT: u32 = DATA + 0x41_0000;
    const OC737_FX_HPOS: u32 = DATA + 0x42_0000;
    const OC737_FX_HSCALE: u32 = DATA + 0x43_0000;
    const OC737_FX_HCOL: u32 = DATA + 0x44_0000;
    const OC737_FX_HSTR: u32 = DATA + 0x45_0000;
    const OC737_FX_HIMG: u32 = DATA + 0x46_0000;
    const OC737_FX_GR: u32 = DATA + 0x47_0000;
    const OC737_FX_DS: u32 = DATA + 0x48_0000;
    const OC737_FX_GVT: u32 = DATA + 0x49_0000;
    const OC737_FX_WSTR: u32 = DATA + 0x4a_0000;
    const OC737_FX_WFONT: u32 = DATA + 0x4b_0000;
    const OC737_FX_WTBL: u32 = DATA + 0x4c_0000;
    const OC737_FX_WGLYPH: u32 = DATA + 0x4d_0000;

    // Result queues of the fakes, inside the control block: `[next, len,
    // results...]`.
    const OC737_Q_RIDER: u32 = 0x100;
    const OC737_Q_FIND: u32 = 0x140;
    const OC737_Q_FREE: u32 = 0x180;
    const OC737_Q_TABLE: u32 = 0x1c0;
    const OC737_Q_TEXT: u32 = 0x200;

    macro_rules! oc737_fake {
        ($fake:ident, $prev:ident, $gen:path, $addr:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC737_FX_CTRL) != OC737_FX_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
        ($fake:ident, $prev:ident, $gen:path, $addr:expr, queue $q:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC737_FX_CTRL) != OC737_FX_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
                // The next queued result, if any (else the fake's own).
                let base = OC737_FX_CTRL + $q;
                let next = c.bus.read32(base);
                c.bus.write32(base, next + 1);
                if next < c.bus.read32(base + 4) {
                    let v = c.bus.read32(base + 8 + 4 * next);
                    c.g[V0][0] = sx(v);
                }
            }
        };
    }

    oc737_fake!(oc737_fake_rider, OC737_PREV_RIDER, ee::s03::f_001a0548__generated, 0x001a_0548, queue OC737_Q_RIDER);
    oc737_fake!(
        oc737_fake_set_pose,
        OC737_PREV_SET_POSE,
        ee::s03::f_0019f3e8__generated,
        0x0019_f3e8
    );
    oc737_fake!(
        oc737_fake_set_board,
        OC737_PREV_SET_BOARD,
        ee::s03::f_0019f548__generated,
        0x0019_f548
    );
    oc737_fake!(
        oc737_fake_set_visible,
        OC737_PREV_SET_VISIBLE,
        ee::s03::f_0019e538__generated,
        0x0019_e538
    );
    oc737_fake!(oc737_fake_find_name, OC737_PREV_FIND_NAME, ee::s03::f_0019d428__generated, 0x0019_d428, queue OC737_Q_FIND);
    oc737_fake!(oc737_fake_find_free, OC737_PREV_FIND_FREE, ee::s03::f_0019d3b0__generated, 0x0019_d3b0, queue OC737_Q_FREE);
    oc737_fake!(
        oc737_fake_quat,
        OC737_PREV_QUAT,
        ee::s10::f_0031be50__generated,
        0x0031_be50
    );
    oc737_fake!(
        oc737_fake_anim_reset,
        OC737_PREV_ANIM_RESET,
        ee::s10::f_00311a50__generated,
        0x0031_1a50
    );
    oc737_fake!(
        oc737_fake_anim_play,
        OC737_PREV_ANIM_PLAY,
        ee::s10::f_003128e8__generated,
        0x0031_28e8
    );
    oc737_fake!(
        oc737_fake_hash,
        OC737_PREV_HASH,
        ee::s10::f_00317670__generated,
        0x0031_7670
    );
    oc737_fake!(
        oc737_fake_alloc,
        OC737_PREV_ALLOC,
        ee::s10::f_00317f60__generated,
        0x0031_7f60
    );
    oc737_fake!(
        oc737_fake_open,
        OC737_PREV_OPEN,
        ee::s10::f_00317e30__generated,
        0x0031_7e30
    );
    oc737_fake!(
        oc737_fake_be,
        OC737_PREV_BE,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80
    );
    oc737_fake!(
        oc737_fake_iface,
        OC737_PREV_IFACE,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58
    );
    oc737_fake!(oc737_fake_uber, OC737_PREV_UBER, ee::s01::f_0014ff90__generated, 0x0014_ff90, queue OC737_Q_TABLE);
    oc737_fake!(
        oc737_fake_money,
        OC737_PREV_MONEY,
        ee::s01::f_00150928__generated,
        0x0015_0928
    );
    oc737_fake!(
        oc737_fake_char_id,
        OC737_PREV_CHAR_ID,
        ee::s01::f_00147488__generated,
        0x0014_7488
    );
    oc737_fake!(
        oc737_fake_list_first,
        OC737_PREV_LIST_FIRST,
        ee::s13::f_00397840__generated,
        0x0039_7840
    );
    oc737_fake!(oc737_fake_by_hash, OC737_PREV_BY_HASH, ee::s13::f_0039d860__generated, 0x0039_d860, queue OC737_Q_TEXT);
    oc737_fake!(
        oc737_fake_set_text,
        OC737_PREV_SET_TEXT,
        ee::s13::f_003a0c70__generated,
        0x003a_0c70
    );
    oc737_fake!(
        oc737_fake_release,
        OC737_PREV_RELEASE,
        ee::s14::f_003df9d8__generated,
        0x003d_f9d8
    );
    oc737_fake!(
        oc737_fake_read,
        OC737_PREV_READ,
        ee::s14::f_003df748__generated,
        0x003d_f748
    );
    oc737_fake!(
        oc737_fake_wait,
        OC737_PREV_WAIT,
        ee::s14::f_003e1bb8__generated,
        0x003e_1bb8
    );
    oc737_fake!(
        oc737_fake_strcpy,
        OC737_PREV_STRCPY,
        ee::s15::f_004166f4__generated,
        0x0041_66f4
    );
    oc737_fake!(
        oc737_fake_text_draw,
        OC737_PREV_TEXT_DRAW,
        ee::s12::f_00391e30__generated,
        0x0039_1e30
    );
    oc737_fake!(
        oc737_fake_font_text,
        OC737_PREV_FONT_TEXT,
        ee::s12::f_00391cb0__generated,
        0x0039_1cb0
    );

    static OC737_PREV_FREE_SLOT: AtomicUsize = AtomicUsize::new(0);

    /// `BoltFile_FreeSlot` fake: logs the call like the others, then does
    /// what the real one's callers rely on: the slot loses its data pointer
    /// and the cache's byte count drops by the control block's `+4` word.
    fn oc737_fake_free_slot(c: &mut Cpu) {
        if c.bus.read32(OC737_FX_CTRL) != OC737_FX_MAGIC {
            return fakes::fall_back(c, &OC737_PREV_FREE_SLOT, ee::s03::f_0019d4b8__generated);
        }
        let mgr = c.g[A0][0] as u32;
        let bank = c.g[A1][0] as u32;
        let index = c.g[A2][0] as u32;
        let slot = mgr
            .wrapping_add(bank.wrapping_mul(0x12000))
            .wrapping_add(index.wrapping_mul(0x120));
        fakes::fake_call(c, 0x0019_d4b8);
        c.bus.write32(slot.wrapping_add(0x104), 0);
        let used = c.bus.read32(mgr.wrapping_add(0xb4000));
        let drop = c.bus.read32(OC737_FX_CTRL + 4);
        c.bus
            .write32(mgr.wrapping_add(0xb4000), used.wrapping_sub(drop));
    }

    macro_rules! oc737_hook {
        ($shard:ident, $slot:ident, $fake:ident, $prev:ident) => {
            fakes::hook(std::ptr::addr_of_mut!(ee::$shard::$slot), $fake, &$prev);
        };
    }

    fn oc737_install_fakes() {
        oc737_hook!(s03, OVERRIDE_001A0548, oc737_fake_rider, OC737_PREV_RIDER);
        oc737_hook!(
            s03,
            OVERRIDE_0019F3E8,
            oc737_fake_set_pose,
            OC737_PREV_SET_POSE
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019F548,
            oc737_fake_set_board,
            OC737_PREV_SET_BOARD
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019E538,
            oc737_fake_set_visible,
            OC737_PREV_SET_VISIBLE
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019D428,
            oc737_fake_find_name,
            OC737_PREV_FIND_NAME
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019D3B0,
            oc737_fake_find_free,
            OC737_PREV_FIND_FREE
        );
        oc737_hook!(
            s03,
            OVERRIDE_0019D4B8,
            oc737_fake_free_slot,
            OC737_PREV_FREE_SLOT
        );
        oc737_hook!(s10, OVERRIDE_0031BE50, oc737_fake_quat, OC737_PREV_QUAT);
        oc737_hook!(
            s10,
            OVERRIDE_00311A50,
            oc737_fake_anim_reset,
            OC737_PREV_ANIM_RESET
        );
        oc737_hook!(
            s10,
            OVERRIDE_003128E8,
            oc737_fake_anim_play,
            OC737_PREV_ANIM_PLAY
        );
        oc737_hook!(s10, OVERRIDE_00317670, oc737_fake_hash, OC737_PREV_HASH);
        oc737_hook!(s10, OVERRIDE_00317F60, oc737_fake_alloc, OC737_PREV_ALLOC);
        oc737_hook!(s10, OVERRIDE_00317E30, oc737_fake_open, OC737_PREV_OPEN);
        oc737_hook!(s01, OVERRIDE_0014DC80, oc737_fake_be, OC737_PREV_BE);
        oc737_hook!(s01, OVERRIDE_0014DD58, oc737_fake_iface, OC737_PREV_IFACE);
        oc737_hook!(s01, OVERRIDE_0014FF90, oc737_fake_uber, OC737_PREV_UBER);
        oc737_hook!(s01, OVERRIDE_00150928, oc737_fake_money, OC737_PREV_MONEY);
        oc737_hook!(
            s01,
            OVERRIDE_00147488,
            oc737_fake_char_id,
            OC737_PREV_CHAR_ID
        );
        oc737_hook!(
            s13,
            OVERRIDE_00397840,
            oc737_fake_list_first,
            OC737_PREV_LIST_FIRST
        );
        oc737_hook!(
            s13,
            OVERRIDE_0039D860,
            oc737_fake_by_hash,
            OC737_PREV_BY_HASH
        );
        oc737_hook!(
            s13,
            OVERRIDE_003A0C70,
            oc737_fake_set_text,
            OC737_PREV_SET_TEXT
        );
        oc737_hook!(
            s14,
            OVERRIDE_003DF9D8,
            oc737_fake_release,
            OC737_PREV_RELEASE
        );
        oc737_hook!(s14, OVERRIDE_003DF748, oc737_fake_read, OC737_PREV_READ);
        oc737_hook!(s14, OVERRIDE_003E1BB8, oc737_fake_wait, OC737_PREV_WAIT);
        oc737_hook!(s15, OVERRIDE_004166F4, oc737_fake_strcpy, OC737_PREV_STRCPY);
        oc737_hook!(
            s12,
            OVERRIDE_00391E30,
            oc737_fake_text_draw,
            OC737_PREV_TEXT_DRAW
        );
        oc737_hook!(
            s12,
            OVERRIDE_00391CB0,
            oc737_fake_font_text,
            OC737_PREV_FONT_TEXT
        );
    }

    /// A machine with the fakes installed, gated on, and `calls` as the range
    /// of fake-call indices at which a call may unwind.
    fn oc737_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc737_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC737_FX_CTRL, OC737_FX_MAGIC);
        r
    }

    /// Queues the results the next calls of a queued fake return.
    fn oc737_queue(c: &mut Cpu, q: u32, results: &[u32]) {
        c.bus.write32(OC737_FX_CTRL + q, 0);
        c.bus.write32(OC737_FX_CTRL + q + 4, results.len() as u32);
        for (i, v) in results.iter().enumerate() {
            c.bus.write32(OC737_FX_CTRL + q + 8 + 4 * i as u32, *v);
        }
    }

    /// `cFEStateCharSetup_UpdateRiderPreview`: the rider the manager returns
    /// (or none), whether it is loaded and compiled, the draw-3D and
    /// first-frame flags, and whether the rider has an animation object.
    #[test]
    fn cfe_state_char_setup_update_rider_preview_matches() {
        fakes::covered(
            ee::s02::f_00182ec0__generated,
            cfe_state_char_setup_update_rider_preview,
            200,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0018_2ec0, 9);
                let gp = GAME_GP;
                c.bus.write32(gp.wrapping_sub(0x848), OC737_FX_WORLD);
                c.bus.write32(OC737_FX_WORLD + 0x7c, OC737_FX_OBJ);
                c.bus.write8(OC737_FX_SELF + 0x44, (r.next_u32() % 4) as u8);
                c.bus.write32(OC737_FX_SELF + 0x48, r.next_u32() & 1);
                c.bus.write32(OC737_FX_SELF + 0x4c, (seed % 3 != 0) as u32);
                let kind = if r.next_u32() % 6 == 0 {
                    12
                } else {
                    r.next_u32() % 10
                };
                c.bus.write32(OC737_FX_RIDER, kind);
                let loaded = if r.next_u32() % 4 == 0 {
                    0
                } else {
                    OC737_FX_OBJ
                };
                c.bus.write32(OC737_FX_RIDER + 0xcb8, loaded);
                let models = if r.next_u32() % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 100
                };
                c.bus.write32(OC737_FX_RIDER + 0xcb4, models);
                let anim = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    OC737_FX_OBJ
                };
                c.bus.write32(OC737_FX_RIDER + 0xc, anim);
                // The preview rotation axes and the gp-relative constants.
                for i in 0..12u32 {
                    let v = r.f32(2.0);
                    c.bus.write32(0x004f_f140 + 4 * i, v);
                }
                let (k1, k2) = (r.f32(2.0), r.f32(2.0));
                c.bus.write32(gp.wrapping_sub(0x5a08), k1);
                c.bus.write32(gp.wrapping_sub(0x5a04), k2);
                let rider = if seed % 7 == 0 { 0 } else { OC737_FX_RIDER };
                oc737_queue(c, OC737_Q_RIDER, &[rider]);
                c.g[A0][0] = sx(OC737_FX_SELF);
                c.pc = 0x0018_2ec0;
            },
        );
    }

    /// `cFEStateUberTrick_UpdateCost`: the selected category and trick, the
    /// cost table, and which of the two text objects the screen has.
    #[test]
    fn cfe_state_uber_trick_update_cost_matches() {
        fakes::covered(
            ee::s02::f_00184520__generated,
            cfe_state_uber_trick_update_cost,
            200,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0018_4520, 16);
                c.bus.write32(OC737_FX_SELF + 0x4c, OC737_FX_MM);
                c.bus.write32(OC737_FX_SELF + 0x50, OC737_FX_TM);
                c.bus.write8(OC737_FX_SELF + 0x44, (r.next_u32() % 4) as u8);
                c.bus.write32(OC737_FX_MM + 0xa0, OC737_FX_P1);
                c.bus.write32(OC737_FX_TM + 0xa0, OC737_FX_P2);
                c.bus.write8(OC737_FX_P1 + 0x18, (r.next_u32() % 16) as u8);
                let trick = (r.next_u32() % 81) as i32 - 40;
                c.bus.write8(OC737_FX_P2 + 0x18, trick as u8);
                fill(c, &mut r, OC737_FX_TBL - 0x400, 0x200);
                let first = if seed % 3 == 1 { 0 } else { OC737_FX_TXT1 };
                let second = if seed % 5 == 2 { 0 } else { OC737_FX_TXT2 };
                oc737_queue(c, OC737_Q_TABLE, &[OC737_FX_TBL]);
                // Both lookups of the hash-named text objects.
                oc737_queue(c, OC737_Q_TEXT, &[first, second]);
                c.g[A0][0] = sx(OC737_FX_SELF);
                c.pc = 0x0018_4520;
            },
        );
    }

    /// The bolt file manager of the cache tests: slot `index` of `bank`.
    fn oc737_slot(bank: u32, index: u32) -> u32 {
        OC737_FX_MGR + bank * 0x12000 + index * 0x120
    }

    /// `BoltFile_EnsureBudget`: bytes in use around the budget, a handful of
    /// slots in the banks (idle ones with data, and decoys that are
    /// referenced, busy or empty), and a free that lowers the byte count by
    /// nothing, a little or everything.
    #[test]
    fn bolt_file_ensure_budget_matches() {
        fakes::covered(
            ee::s03::f_0019d578__generated,
            bolt_file_ensure_budget,
            200,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0019_d578, 8);
                let bank = r.next_u32() % 10;
                let drop = [0u32, 700_000, 100_000_000][(r.next_u32() % 3) as usize];
                c.bus.write32(OC737_FX_CTRL + 4, drop);
                let used = match seed % 4 {
                    0 => 0,
                    1 => 9_999_000,
                    _ => 10_000_000 + r.next_u32() % 5000,
                };
                c.bus.write32(OC737_FX_MGR + 0xb4000, used);
                for k in 0..10u32 {
                    let b = if k < 4 { bank } else { r.next_u32() % 10 };
                    let slot = oc737_slot(b, r.next_u32() % 256);
                    c.bus.write32(slot + 0x104, 1 + r.next_u32() % 1000);
                    c.bus.write32(slot + 0x11c, r.next_u32() >> 4);
                    match r.next_u32() % 4 {
                        0 => c.bus.write32(slot, 1),
                        1 => c.bus.write32(slot + 0x114, 1),
                        2 => c.bus.write32(slot + 0x104, 0),
                        _ => {}
                    }
                }
                c.g[A0][0] = sx(OC737_FX_MGR);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = (r.next_u32() % 3000) as u64;
                c.pc = 0x0019_d578;
            },
        );
    }

    /// `BoltFile_Acquire`: a cached slot of that name in each state, or a new
    /// one; async and deferred requests; load threads or none.
    #[test]
    fn bolt_file_acquire_matches() {
        fakes::covered(
            ee::s03::f_0019da10__generated,
            bolt_file_acquire,
            300,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x0019_da10, 12);
                let bank = r.next_u32() % 10;
                let found = r.next_u32() % 256;
                let free = r.next_u32() % 256;
                c.bus.write32(OC737_FX_CTRL + 4, 0);
                c.bus
                    .write32(OC737_FX_MGR + 0xb4000, r.next_u32() % 1_000_000);
                let threads = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write32(OC737_FX_MGR + 0xb4004, threads);
                c.bus
                    .write32(OC737_FX_MGR + 0xb4008 + 4 * bank, r.next_u32() % 100);
                for slot in [oc737_slot(bank, found), oc737_slot(bank, free)] {
                    c.bus.write32(slot, r.next_u32() % 4);
                    c.bus.write32(slot + 0x114, r.next_u32() % 4);
                    c.bus.write32(slot + 0x108, r.next_u32());
                    c.bus.write32(slot + 0x10c, r.next_u32() % 3);
                    c.bus.write32(slot + 0x11c, r.next_u32());
                }
                let hit = if (seed / 4) % 3 != 0 { found } else { u32::MAX };
                oc737_queue(c, OC737_Q_FIND, &[hit]);
                oc737_queue(c, OC737_Q_FREE, &[free]);
                c.g[A0][0] = sx(OC737_FX_MGR);
                c.g[A1][0] = bank as u64;
                c.g[A2][0] = sx(OC737_FX_NAME);
                c.g[A3][0] = (r.next_u32() % 8) as u64;
                c.g[T0][0] = (seed & 1) as u64;
                c.g[T1][0] = ((seed >> 1) & 1) as u64;
                c.pc = 0x0019_da10;
            },
        );
    }

    /// `HUD_DrawIconText`: a line of 0..4 elements (strings and pictures),
    /// every alignment, strings that are empty or not.
    #[test]
    fn hud_draw_icon_text_matches() {
        fakes::covered(
            ee::s04::f_001e95a0__generated,
            hud_draw_icon_text,
            300,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x001e_95a0, 5);
                let gp = GAME_GP;
                let n = seed % 5;
                let (w, h, off) = (r.f32(50.0), r.f32(50.0), r.f32(5.0));
                c.bus.write32(OC737_FX_HTXT, w);
                c.bus.write32(OC737_FX_HTXT + 4, h);
                c.bus.write32(OC737_FX_HTXT + 8, OC737_FX_HFONT);
                c.bus.write32(OC737_FX_HTXT + 0xc, off);
                let (b0, b1) = (r.f32(2.0), r.f32(2.0));
                c.bus.write32(OC737_FX_HTXT + 0x10, b0);
                c.bus.write32(OC737_FX_HTXT + 0x14, b1);
                c.bus.write32(OC737_FX_HTXT + 0x18, n);
                for i in 0..n {
                    let e = OC737_FX_HTXT + 0x1c + 8 * i;
                    let ew = r.f32(30.0);
                    c.bus.write32(e, ew);
                    if i % 2 == 0 {
                        let s = OC737_FX_HSTR + 0x100 * i;
                        let first = if r.next_u32() % 4 == 0 {
                            0
                        } else {
                            0x41 + r.next_u32() % 20
                        };
                        c.bus.write16(s, first as u16);
                        c.bus.write32(e + 4, s);
                    } else {
                        let img = OC737_FX_HIMG + 0x100 * i;
                        fill(c, &mut r, img, 9);
                        for off in [0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
                            let v = r.f32(4.0);
                            c.bus.write32(img + off, v);
                        }
                        c.bus.write32(e + 4, img);
                    }
                }
                let (s0, s1) = (r.f32(3.0), r.f32(3.0));
                c.bus.write32(OC737_FX_HFONT + 0x30, s0);
                c.bus.write32(OC737_FX_HFONT + 0x34, s1);
                fill(c, &mut r, OC737_FX_HFONT + 0x40, 8);
                for i in 0..2u32 {
                    let v = r.f32(100.0);
                    c.bus.write32(OC737_FX_HPOS + 4 * i, v);
                    let v = r.f32(3.0);
                    c.bus.write32(OC737_FX_HSCALE + 4 * i, v);
                }
                for i in 0..4u32 {
                    let v = r.f32(1.0);
                    c.bus.write32(OC737_FX_HCOL + 4 * i, v);
                }
                c.bus.write32(gp.wrapping_sub(0x854), OC737_FX_GR);
                c.bus.write32(OC737_FX_GR + 0xe84, OC737_FX_DS);
                c.bus.write32(OC737_FX_GR + 0x10d8, OC737_FX_GVT);
                let delta = (r.next_u32() as i16 as i32 & !3) as u16;
                c.bus.write16(OC737_FX_GVT + 0x238, delta);
                c.bus.write32(OC737_FX_GVT + 0x23c, fakes::FAKE);
                let align = [u64::MAX, 0, 1];
                c.g[A0][0] = sx(OC737_FX_HTXT);
                c.g[A1][0] = sx(OC737_FX_HPOS);
                c.g[A2][0] = sx(OC737_FX_HSCALE);
                c.g[A3][0] = sx(OC737_FX_HCOL);
                c.g[T0][0] = align[(r.next_u32() % 3) as usize];
                c.g[T1][0] = align[(r.next_u32() % 3) as usize];
                c.pc = 0x001e_95a0;
            },
        );
    }

    /// `HUD_DrawWobbleText`: strings of 0..7 characters below, inside and
    /// above the font's direct glyph range, in the sparse part (found or
    /// not) and negative ones.
    #[test]
    fn hud_draw_wobble_text_matches() {
        fakes::covered(
            ee::s04::f_001f14b0__generated,
            hud_draw_wobble_text,
            300,
            &|c, seed| {
                let mut r = oc737_machine(c, seed, 0x001f_14b0, 7);
                let pool = [
                    0x10u8, 0x21, 0x30, 0x40, 0x50, 0x55, 0x57, 0x5a, 0x60, 0x65, 0x70, 0x71, 0x90,
                    0xc3,
                ];
                let len = seed % 8;
                for i in 0..len {
                    let ch = pool[(r.next_u32() % pool.len() as u32) as usize];
                    c.bus.write8(OC737_FX_WSTR + i, ch);
                }
                c.bus.write8(OC737_FX_WSTR + len, 0);
                // The font: 0x26 glyphs, a direct range 0x20..=0x40 and six
                // sparse ones after it, a fallback glyph.
                c.bus.write32(OC737_FX_WFONT, 0x26);
                c.bus.write32(OC737_FX_WFONT + 4, OC737_FX_WTBL);
                c.bus.write32(OC737_FX_WFONT + 8, OC737_FX_WGLYPH);
                c.bus.write32(OC737_FX_WFONT + 0xc, 0x20);
                c.bus.write32(OC737_FX_WFONT + 0x10, 0x40);
                for off in [0x30u32, 0x34] {
                    let v = (1.0f32 + (r.next_u32() % 7) as f32).to_bits();
                    c.bus.write32(OC737_FX_WFONT + off, v);
                }
                for off in [0x38u32, 0x3c] {
                    let v = r.f32(4.0);
                    c.bus.write32(OC737_FX_WFONT + off, v);
                }
                fill(c, &mut r, OC737_FX_WTBL, 0x26 * 3);
                for (k, code) in [0x50u16, 0x55, 0x5a, 0x60, 0x65, 0x70].iter().enumerate() {
                    c.bus.write16(OC737_FX_WTBL + 12 * (0x20 + k as u32), *code);
                }
                fill(c, &mut r, OC737_FX_WGLYPH, 4);
                let (x, y, amp) = (r.f32(100.0), r.f32(100.0), r.f32(2.0));
                c.f[12] = x;
                c.f[13] = y;
                c.f[14] = amp;
                c.g[A1][0] = sx(OC737_FX_WSTR);
                c.g[A2][0] = sx(OC737_FX_WFONT);
                c.pc = 0x001f_14b0;
            },
        );
    }
}
