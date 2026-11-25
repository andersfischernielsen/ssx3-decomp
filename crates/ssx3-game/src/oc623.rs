//! Staging module oc623 (wave 234, list oc623): hand-written replacements, each
//! with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc623`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0013_04e0, rider_board_press_sub_update_end, "RiderBoardPressSub_UpdateEnd"),
    (0x0013_06b0, rider_board_press_sub_check_switch, "RiderBoardPressSub_CheckSwitch"),
    (0x0013_07b8, rider_board_press_sub_check_jump, "RiderBoardPressSub_CheckJump"),
    (0x0013_08d8, rider_board_press_sub_check_spin, "RiderBoardPressSub_CheckSpin"),
];

/// `cBoardPressControl_handleExitState(self, in)` (0x1304e0): the board press's
/// per-frame exit step. On the two press anims (33 and 25) with the slew past
/// zero it plays the flip anim for a press beyond +-0.5 and resets the slew
/// target; otherwise it ends a press that is still pending, leaves the
/// railslide orientation and picks the control state from the motion state.
#[allow(unreachable_code)]
pub fn rider_board_press_sub_update_end(c: &mut Cpu) {
    guest(c, |c| {
        'b_13069c: {
        'b_130698: {
        'b_130620: {
        'b_130520: {
        c.cycles += 14;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x2;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x00130508);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x00312aa0, 0x00130508)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x21;
        let t = c.g[A0][0] == c.g[A1][0];
        c.g[V0][0] = 0x19;
        if t {
        break 'b_130520;
        } else {
        c.cycles += 2;
        let t = c.g[A0][0] != c.g[V0][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_130620;
        } else {
        break 'b_130520;
        }
        }
        }
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x268));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[V1][0] = c.g[V0][0];
        if t {
        break 'b_130620;
        } else {
        'b_130594: {
        'b_130574: {
        c.cycles += 12;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76a4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
        c.g[AT][0] = 0xffffffffbf000000;
        c.mtc1(1, 1);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(2, 0, 2);
        c.c_lt_s(2, 1);
        let t = !c.fcc();

        if t {
        break 'b_130574;
        } else {
        c.cycles += 2;
        let t = c.g[A0][0] == c.g[A1][0];

        if t {
        break 'b_130594;
        } else {
        break 'b_130574;
        }
        }
        }
        c.cycles += 6;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 0);
        c.c_lt_s(0, 2);
        let t = !c.fcc();
        c.g[V0][0] = 0x19;
        if t {
        break 'b_130698;
        } else {
        c.cycles += 2;
        if c.g[A0][0] != c.g[V0][0] {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_13069c;
        } else {
        break 'b_130594;
        }
        }
        }
        'b_1305f0: {
        'b_1305cc: {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(2, 0);
        let t = !c.fcc();
        c.g[A1][0] = 0x20;
        if t {
        break 'b_1305cc;
        } else {
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001305c0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x003128e8, 0x001305c0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        let t = true;
        c.g[V0][0] = 0x2;
        if t {
        break 'b_1305f0;
        } else {
        break 'b_1305cc;
        }
        }
        }
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x18;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001305e8);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x003128e8, 0x001305e8)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = 0x1;
        break 'b_1305f0;
        }
        c.cycles += 12;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x330), c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x76a0));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 1);
        c.add_s(0, 0, 0);
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x270));
        c.mul_s(0, 0, 2);
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x26c));
        let t = true;
        c.sw((c.g[S0][0] as u32), 0u64);
        if t {
        break 'b_130698;
        } else {
        break 'b_130620;
        }
        }
        }
        'b_13064c: {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x330));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        let t = c.g[V0][0] == 0u64;

        if t {
        break 'b_13064c;
        } else {
        c.cycles += 6;
        c.g[RA][0] = sx(0x0013063c);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x790));
        Callee { addr: 0x00119a38, f: ee::s00::f_00119a38 }.call(c, 0x0013063c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.mov_s(12, 0);
        c.g[RA][0] = sx(0x0013064c);
        c.g[A1][0] = 0x1;
        Callee { addr: 0x0010e098, f: ee::s00::f_0010e098 }.call(c, 0x0013064c)?;
        break 'b_13064c;
        }
        }
        'b_130684: {
        c.cycles += 5;
        c.g[RA][0] = sx(0x00130654);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee { addr: 0x0011fe98, f: ee::s00::f_0011fe98 }.call(c, 0x00130654)?;
        c.g[V1][0] = 0x4;
        let t = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = 0u64;
        if t {
        break 'b_130684;
        } else {
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x77c));
        c.g[RA][0] = sx(0x00130670);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2b0));
        call_ptr(c, 0x001326c8, 0x00130670)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x0013067c);
        c.g[A1][0] = 0x7;
        Callee { addr: 0x0011fec8, f: ee::s00::f_0011fec8 }.call(c, 0x0013067c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_13069c;
        } else {
        break 'b_130684;
        }
        }
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0013068c);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee { addr: 0x00115640, f: ee::s00::f_00115640 }.call(c, 0x0013068c)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x00130698);
        c.g[A1][0] = 0u64;
        Callee { addr: 0x0011fec8, f: ee::s00::f_0011fec8 }.call(c, 0x00130698)?;
        break 'b_130698;
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_13069c;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `cBoardPressControl_checkForFakie(self)` (0x1306b0): the fakie check. Ends a
/// press on a ground fakie switch, plays the matching anim and sets the press
/// mode, or returns 0 when the rider's state does not allow it.
#[allow(unreachable_code)]
pub fn rider_board_press_sub_check_switch(c: &mut Cpu) {
    guest(c, |c| {
        'b_1307a8: {
        'b_1307a4: {
        'b_1307a0: {
        'b_130798: {
        c.cycles += 13;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[A1][0] = 0x2;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x001306d4);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x00312aa0, 0x001306d4)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 0x17;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1f;
        if t {
        break 'b_130798;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x25));
        if t {
        break 'b_130798;
        } else {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if t {
        break 'b_1307a4;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1d;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1e;
        if t {
        break 'b_1307a0;
        } else {
        c.cycles += 2;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0u64;
        if t {
        break 'b_1307a4;
        } else {
        c.cycles += 4;
        c.g[RA][0] = sx(0x00130714);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee { addr: 0x0011fe98, f: ee::s00::f_0011fe98 }.call(c, 0x00130714)?;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = 0u64;
        if t {
        break 'b_1307a4;
        } else {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.mtc1(0, 1);
        c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x2dc));
        c.c_eq_s(0, 1);
        if !c.fcc() {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_1307a8;
        } else {
        c.cycles += 4;
        c.g[RA][0] = sx(0x00130740);
        Callee { addr: 0x00114cc0, f: ee::s00::f_00114cc0 }.call(c, 0x00130740)?;
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = 0x1;
        if t {
        break 'b_1307a0;
        } else {
        'b_13077c: {
        c.cycles += 4;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x330));
        let t = c.g[V0][0] != c.g[S1][0];
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x784));
        if t {
        break 'b_13077c;
        } else {
        c.cycles += 9;
        c.g[A1][0] = 0x17;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x0013076c);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x003128e8, 0x0013076c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = 0x2;
        let t = true;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x330), c.g[V0][0]);
        if t {
        break 'b_130798;
        } else {
        break 'b_13077c;
        }
        }
        }
        c.cycles += 7;
        c.g[A1][0] = 0x1f;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x00130790);
        c.g[A2][0] = 0u64;
        call_ptr(c, 0x003128e8, 0x00130790)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x330), c.g[S1][0]);
        break 'b_130798;
        }
        }
        }
        }
        }
        }
        }
        }
        }
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_1307a4;
        } else {
        break 'b_1307a0;
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0u64;
        break 'b_1307a4;
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_1307a8;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `cBoardPressControl_checkForOllie(self, cmd)` (0x1307b8): latch the ollie
/// button (bit 0x20000 of the command word) and, when it is released while the
/// press is held, end the press and launch the rider into the air.
#[allow(unreachable_code)]
pub fn rider_board_press_sub_check_jump(c: &mut Cpu) {
    guest(c, |c| {
        'b_1307e4: {
        c.cycles += 9;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[A2][0] = 0x20000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        let t = c.g[V0][0] == 0u64;
        c.g[S0][0] = c.g[A0][0];
        if t {
        break 'b_1307e4;
        } else {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        break 'b_1307e4;
        }
        }
        'b_1307fc: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = 0u64;
        if t {
        break 'b_1307fc;
        } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V1][0] = (c.g[V0][0] < 0x1u64) as u64;
        break 'b_1307fc;
        }
        }
        'b_1308c0: {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
        c.abs_s(20, 0);
        let t = c.g[V1][0] == 0u64;
        c.g[V0][0] = 0u64;
        if t {
        break 'b_1308c0;
        } else {
        'b_130870: {
        'b_130850: {
        c.cycles += 8;
        c.g[RA][0] = sx(0x00130818);
        c.g[A0][0] = c.g[S0][0];
        Callee { addr: 0x00131428, f: ee::s00::f_00131428 }.call(c, 0x00130818)?;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 0);
        c.c_lt_s(0, 20);
        let t = !c.fcc();
        c.g[A1][0] = 0x23;
        if t {
        break 'b_130850;
        } else {
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00130848);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x003128e8, 0x00130848)?;
        let t = true;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_130870;
        } else {
        break 'b_130850;
        }
        }
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = 0x1b;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x0013086c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x003128e8, 0x0013086c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        break 'b_130870;
        }
        'b_130888: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x328));
        let t = c.g[V0][0] == 0u64;

        if t {
        break 'b_130888;
        } else {
        c.cycles += 3;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x328), 0u64);
        c.g[RA][0] = sx(0x00130888);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee { addr: 0x00116930, f: ee::s00::f_00116930 }.call(c, 0x00130888)?;
        break 'b_130888;
        }
        }
        c.cycles += 14;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 12);
        c.g[RA][0] = sx(0x00130898);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        Callee { addr: 0x00114298, f: ee::s00::f_00114298 }.call(c, 0x00130898)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x001308a4);
        c.g[A1][0] = 0x1;
        Callee { addr: 0x0011fe78, f: ee::s00::f_0011fe78 }.call(c, 0x001308a4)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x001308b0);
        c.g[A1][0] = 0x5;
        Callee { addr: 0x0011fec8, f: ee::s00::f_0011fec8 }.call(c, 0x001308b0)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[RA][0] = sx(0x001308bc);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x410));
        call_ptr(c, 0x00294170, 0x001308bc)?;
        c.g[V0][0] = 0x1;
        break 'b_1308c0;
        }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    });
}

/// `cBoardPressControl_checkForPivot(self, in)` (0x1308d8): the pivot check.
/// Turns the stick into a board-press angle, slews it toward the pivot target
/// and starts the pivot anim (or the spin, with the Q-register VU0 dot products)
/// when the press passes the threshold.
#[allow(unreachable_code)]
pub fn rider_board_press_sub_check_spin(c: &mut Cpu) {
    guest(c, |c| {
        'b_130948: {
        c.cycles += 27;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x769c));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.g[S1][0] = c.g[A0][0];
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 20);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 26);
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 26) as u32);
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 26) as u32);
        c.mtc1(3, 1);
        c.cvt_s_w(1, 1);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.mul_s(20, 1, 2);
        c.mul_s(21, 0, 2);
        c.abs_s(0, 20);
        c.abs_s(3, 21);
        c.c_lt_s(0, 3);
        let t = !c.fcc();
        c.mov_s(23, 0);
        if t {
        break 'b_130948;
        } else {
        c.cycles += 1;
        c.mov_s(23, 3);
        break 'b_130948;
        }
        }
        'b_130da8: {
        'b_130d9c: {
        'b_1309f4: {
        'b_13099c: {
        c.cycles += 4;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7698));
        c.c_lt_s(23, 0);
        let t = c.fcc();

        if t {
        break 'b_13099c;
        } else {
        c.cycles += 14;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.mtc1(0, 22);
        c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
        c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x3a0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 0);
        c.c_lt_s(0, 22);
        let t = c.fcc();

        if t {
        break 'b_13099c;
        } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        if c.g[V0][0] == 0u64 {
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x320));
        break 'b_1309f4;
        } else {
        break 'b_13099c;
        }
        }
        }
        }
        'b_1309d4: {
        c.cycles += 9;
        c.g[RA][0] = sx(0x001309a4);
        c.g[A0][0] = c.g[S1][0];
        Callee { addr: 0x001313a8, f: ee::s00::f_001313a8 }.call(c, 0x001309a4)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7694));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
        c.abs_s(2, 0);
        c.c_le_s(2, 1);
        let t = c.fcc();

        if t {
        break 'b_1309d4;
        } else {
        c.cycles += 5;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7690));
        c.c_le_s(0, 2);
        let t = !c.fcc();
        c.g[V0][0] = 0x1;
        if t {
        break 'b_130da8;
        } else {
        break 'b_1309d4;
        }
        }
        }
        c.cycles += 4;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x768c));
        c.c_lt_s(23, 0);
        let t = !c.fcc();

        if t {
        break 'b_130d9c;
        } else {
        c.cycles += 4;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        let t = true;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
        if t {
        break 'b_130d9c;
        } else {
        break 'b_1309f4;
        }
        }
        }
        'b_1309fc: {
        c.cycles += 2;
        if c.g[V0][0] != 0u64 {
        c.neg_s(20, 20);
        break 'b_1309fc;
        } else {
        break 'b_1309fc;
        }
        }
        'b_130a78: {
        'b_130a74: {
        'b_130a34: {
        c.cycles += 3;
        c.c_eq_s(21, 22);
        let t = !c.fcc();

        if t {
        break 'b_130a34;
        } else {
        c.cycles += 4;
        c.c_eq_s(20, 22);
        let t = c.fcc();
        c.mov_s(12, 20);
        if t {
        break 'b_130a74;
        } else {
        c.cycles += 4;
        c.c_le_s(22, 20);
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x7688));
        let t = !c.fcc();
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_130a78;
        } else {
        c.cycles += 3;
        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x7684));
        let t = true;

        if t {
        break 'b_130a78;
        } else {
        break 'b_130a34;
        }
        }
        }
        }
        }
        'b_130a70: {
        c.cycles += 7;
        c.div_s(12, 20, 21);
        c.g[RA][0] = sx(0x00130a40);
        call_ptr(c, 0x0031c228, 0x00130a40)?;
        c.c_lt_s(21, 22);
        let t = !c.fcc();
        c.mov_s(1, 0);
        if t {
        break 'b_130a70;
        } else {
        'b_130a68: {
        c.cycles += 3;
        c.c_lt_s(22, 20);
        let t = !c.fcc();

        if t {
        break 'b_130a68;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7680));
        let t = true;
        c.add_s(1, 1, 0);
        if t {
        break 'b_130a70;
        } else {
        break 'b_130a68;
        }
        }
        }
        c.cycles += 2;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x767c));
        c.sub_s(1, 1, 0);
        break 'b_130a70;
        }
        }
        c.cycles += 1;
        c.mov_s(12, 1);
        break 'b_130a74;
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        break 'b_130a78;
        }
        'b_130abc: {
        c.cycles += 14;
        c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7678));
        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x280));
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7674));
        c.mul_s(21, 1, 2);
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 3);
        c.sub_s(1, 12, 21);
        c.mul_s(0, 1, 0);
        c.add_s(0, 0, 3);
        c.cvt_w_s(2, 0);
        c.cvt_s_w(2, 2);
        c.c_lt_s(0, 2);
        let t = !c.fcc();

        if t {
        break 'b_130abc;
        } else {
        c.cycles += 3;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sub_s(2, 2, 0);
        break 'b_130abc;
        }
        }
        'b_130b00: {
        'b_130afc: {
        c.cycles += 11;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7670));
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        c.mul_s(0, 2, 0);
        c.g[RA][0] = sx(0x00130ad4);
        c.sub_s(20, 1, 0);
        call_ptr(c, 0x00312aa0, 0x00130ad4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x25));
        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A2][0] = 0u64;
        if t {
        break 'b_130afc;
        } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1d;
        let t = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 0x1e;
        if t {
        break 'b_130afc;
        } else {
        c.cycles += 2;
        let t = c.g[A1][0] != c.g[V0][0];

        if t {
        break 'b_130b00;
        } else {
        break 'b_130afc;
        }
        }
        }
        }
        c.cycles += 1;
        c.g[A2][0] = 0x1;
        break 'b_130b00;
        }
        'b_130cc8: {
        'b_130b8c: {
        c.cycles += 2;
        if c.g[A2][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        break 'b_130b8c;
        } else {
        c.cycles += 6;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x766c));
        c.abs_s(0, 20);
        c.c_le_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_130b8c;
        } else {
        'b_130b64: {
        c.cycles += 7;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
        c.abs_s(0, 0);
        c.c_lt_s(1, 0);
        let t = !c.fcc();

        if t {
        break 'b_130b64;
        } else {
        'b_130b58: {
        c.cycles += 4;
        c.mtc1(0, 1);
        c.c_lt_s(1, 20);
        let t = !c.fcc();

        if t {
        break 'b_130b58;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7668));
        let t = true;
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x284));
        if t {
        break 'b_130cc8;
        } else {
        break 'b_130b58;
        }
        }
        }
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7664));
        let t = true;
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x284));
        if t {
        break 'b_130cc8;
        } else {
        break 'b_130b64;
        }
        }
        }
        'b_130b80: {
        c.cycles += 4;
        c.mtc1(0, 1);
        c.c_lt_s(20, 1);
        let t = !c.fcc();

        if t {
        break 'b_130b80;
        } else {
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7660));
        let t = true;
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x284));
        if t {
        break 'b_130cc8;
        } else {
        break 'b_130b80;
        }
        }
        }
        c.cycles += 3;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x765c));
        let t = true;
        c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x284));
        if t {
        break 'b_130cc8;
        } else {
        break 'b_130b8c;
        }
        }
        }
        }
        'b_130cb4: {
        'b_130c08: {
        c.cycles += 9;
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7658));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
        c.mtc1(0, 2);
        c.mul_s(1, 20, 1);
        c.add_s(0, 0, 0);
        c.c_lt_s(21, 2);
        let t = !c.fcc();
        c.mul_s(0, 1, 0);
        if t {
        break 'b_130c08;
        } else {
        c.cycles += 3;
        c.c_lt_s(2, 0);
        let t = !c.fcc();

        if t {
        break 'b_130c08;
        } else {
        c.cycles += 14;
        c.add_s(0, 21, 0);
        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x7654));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7650));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.sub_s(12, 0, 3);
        c.mul_s(1, 12, 1);
        c.add_s(1, 1, 2);
        c.cvt_w_s(2, 1);
        c.cvt_s_w(2, 2);
        c.c_lt_s(1, 2);
        if !c.fcc() {
        c.mul_s(0, 2, 3);
        break 'b_130cb4;
        } else {
        c.cycles += 5;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sub_s(2, 2, 0);
        let t = true;
        c.mul_s(0, 2, 3);
        if t {
        break 'b_130cb4;
        } else {
        break 'b_130c08;
        }
        }
        }
        }
        }
        'b_130c78: {
        c.cycles += 5;
        c.mtc1(0, 1);
        c.c_lt_s(1, 21);
        if !c.fcc() {
        c.add_s(12, 21, 0);
        break 'b_130c78;
        } else {
        c.cycles += 4;
        c.c_lt_s(0, 1);
        if !c.fcc() {
        c.add_s(12, 21, 0);
        break 'b_130c78;
        } else {
        c.cycles += 14;
        c.add_s(0, 21, 0);
        c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x764c));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7648));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.add_s(12, 0, 3);
        c.mul_s(1, 12, 1);
        c.add_s(1, 1, 2);
        c.cvt_w_s(2, 1);
        c.cvt_s_w(2, 2);
        c.c_lt_s(1, 2);
        if !c.fcc() {
        c.mul_s(0, 2, 3);
        break 'b_130cb4;
        } else {
        c.cycles += 5;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sub_s(2, 2, 0);
        let t = true;
        c.mul_s(0, 2, 3);
        if t {
        break 'b_130cb4;
        } else {
        break 'b_130c78;
        }
        }
        }
        }
        }
        'b_130cac: {
        c.cycles += 10;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7644));
        c.mul_s(0, 12, 0);
        c.add_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.cvt_s_w(1, 1);
        c.c_lt_s(0, 1);
        let t = !c.fcc();

        if t {
        break 'b_130cac;
        } else {
        c.cycles += 3;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.sub_s(1, 1, 0);
        break 'b_130cac;
        }
        }
        c.cycles += 2;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7640));
        c.mul_s(0, 1, 0);
        break 'b_130cb4;
        }
        c.cycles += 5;
        c.sub_s(1, 12, 0);
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x763c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.mul_s(0, 1, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x284), 0u64);
        break 'b_130cc8;
        }
        c.cycles += 9;
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x288));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7638));
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x280));
        c.abs_s(2, 0);
        c.c_le_s(1, 2);
        let t = !c.fcc();

        if t {
        break 'b_130d9c;
        } else {
        c.cycles += 5;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7634));
        c.c_le_s(2, 0);
        let t = !c.fcc();
        c.g[V1][0] = 0x1;
        if t {
        break 'b_130d9c;
        } else {
        'b_130d60: {
        'b_130d48: {
        c.cycles += 11;
        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x300));
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7630));
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 2);
        c.mul_s(0, 0, 1);
        c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x27c));
        c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x278));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x330));
        let t = c.g[V0][0] != c.g[V1][0];
        c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x280));
        if t {
        break 'b_130d48;
        } else {
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 1);
        let t = !c.fcc();
        c.g[S0][0] = 0x1e;
        if t {
        break 'b_130d60;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[S0][0] = 0x1d;
        if t {
        break 'b_130d60;
        } else {
        break 'b_130d48;
        }
        }
        }
        }
        c.cycles += 5;
        c.mtc1(0, 0);
        c.c_lt_s(0, 1);
        let t = !c.fcc();
        c.g[S0][0] = 0x26;
        if t {
        break 'b_130d60;
        } else {
        c.cycles += 1;
        c.g[S0][0] = 0x25;
        break 'b_130d60;
        }
        }
        c.cycles += 2;
        let t = c.g[A1][0] == c.g[S0][0];
        c.g[V0][0] = 0x1;
        if t {
        break 'b_130da8;
        } else {
        'b_130d7c: {
        c.cycles += 2;
        if c.g[A2][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        break 'b_130d7c;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x00130d78);
        c.g[A0][0] = c.g[S1][0];
        Callee { addr: 0x0012fec8, f: ee::s00::f_0012fec8 }.call(c, 0x00130d78)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        break 'b_130d7c;
        }
        }
        c.cycles += 8;
        c.g[A1][0] = c.g[S0][0];
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 12);
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00130d94);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x784));
        call_ptr(c, 0x003128e8, 0x00130d94)?;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_130da8;
        } else {
        break 'b_130d9c;
        }
        }
        }
        }
        }
        c.cycles += 3;
        c.g[RA][0] = sx(0x00130da4);
        c.g[A0][0] = c.g[S1][0];
        Callee { addr: 0x00131428, f: ee::s00::f_00131428 }.call(c, 0x00130da4)?;
        c.g[V0][0] = 0u64;
        break 'b_130da8;
        }
        c.cycles += 9;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x48));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    /// A board press control at `DATA` (its rider at `DATA + 0x1000`, the
    /// rider's anim at `+0x2000`, its metrix at `+0x3000`), the command word
    /// at `DATA + 0x100`, and the fields the four checks read. `seed` picks the
    /// press mode, the ollie bit and the sign of the slew.
    pub(crate) fn bp_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let mut r = Rng(0x0623_0000 ^ seed as u64 ^ (entry as u64) << 24);
        scramble(c, &mut r);
        c.pc = entry;
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x100) as u64;
        fill(c, &mut r, DATA, 0x100);
        let rider = DATA + 0x1000;
        let anim = DATA + 0x2000;
        let metrix = DATA + 0x3000;
        c.bus.write32(DATA + 0x14, rider);
        fill(c, &mut r, rider, 0x400);
        c.bus.write32(rider + 0x784, anim);
        c.bus.write32(rider + 0x790, metrix);
        fill(c, &mut r, anim, 0x100);
        fill(c, &mut r, metrix, 0x100);
        c.bus.write32(rider + 0x330, seed % 4);
        c.bus.write32(rider + 0x320, seed / 4 % 2);
        c.bus.write32(rider + 0x328, seed / 8 % 2);
        c.bus.write32(rider + 0x268, r.edge_f32());
        c.bus.write32(rider + 0x280, r.edge_f32());
        c.bus.write32(rider + 0x2dc, r.edge_f32());
        c.bus.write32(rider + 0x300, r.edge_f32());
        let ollie = if seed % 2 == 0 { 0x20000 } else { 0 };
        c.bus.write32(DATA + 0x100, (r.next_u32() & !0x20000) | ollie);
    }

    #[test]
    fn rider_board_press_sub_update_end_matches() {
        check(ssx3_ee::s00::f_001304e0__generated, rider_board_press_sub_update_end, 200, &|c, seed| {
            bp_fixture(c, seed, 0x0013_04e0)
        });
    }

    #[test]
    fn rider_board_press_sub_check_switch_matches() {
        check(ssx3_ee::s00::f_001306b0__generated, rider_board_press_sub_check_switch, 200, &|c, seed| {
            bp_fixture(c, seed, 0x0013_06b0)
        });
    }

    #[test]
    fn rider_board_press_sub_check_jump_matches() {
        check(ssx3_ee::s00::f_001307b8__generated, rider_board_press_sub_check_jump, 200, &|c, seed| {
            bp_fixture(c, seed, 0x0013_07b8)
        });
    }

    #[test]
    fn rider_board_press_sub_check_spin_matches() {
        check(ssx3_ee::s00::f_001308d8__generated, rider_board_press_sub_check_spin, 200, &|c, seed| {
            bp_fixture(c, seed, 0x0013_08d8)
        });
    }
}
