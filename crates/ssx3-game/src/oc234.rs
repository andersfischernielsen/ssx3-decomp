//! Staging wave 204 (oc234): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_f560, ai_update_rider_relations, "AI_UpdateRiderRelations"),
    (0x001f_1840, hud_draw_time, "HUD_DrawTime"),
    (0x0040_e550, ieee754_logf, "__ieee754_logf"),
    (0x002e_5da0, snowfall_update, "SnowFall_Update"),
    (0x003b_71d8, sndstrm_service, "SNDSTRM_Service"),
    (0x0022_c1b0, game_module_begin_bezier_pass, "GameModule_BeginBezierPass"),
    (0x0035_3b10, skybox_render, "SkyBox_Render"),
    (0x003c_7010, sndstrm_release_bytes, "SNDSTRM_ReleaseBytes"),
    (0x0010_aa70, computer_actor_rider_think_prewind, "ComputerActorRider_ThinkPrewind"),
];

/// `reg + off` (32-bit address arithmetic).
#[inline(always)]
fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `gp - off` (the small-data constants).
#[inline(always)]
fn gp_lit(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

// ---------------------------------------------------------------------------
// AI_UpdateRiderRelations (0x10f560, src/ai/computer.cpp).

/// Fields of the AI race manager and its riders used by the relation pass.
pub mod rider_relations {
    use super::{ee, Callee};
    /// `AI`: frame counter.
    pub const FRAME: u32 = 0x8;
    /// `AI`: `Rider*[]`.
    pub const RIDERS: u32 = 0x28;
    /// `AI`: riders in the race, and riders not taking part (subtracted).
    pub const COUNT: u32 = 0x78;
    pub const SKIP: u32 = 0x84;
    /// `Rider`: per-other-rider info, 0x24 bytes each, at offset 0.
    pub const INFO_SIZE: u32 = 0x24;
    /// Info entry: tracked (non-zero), distance, angle, aggressive flag.
    pub const INFO_ACTIVE: u32 = 0x0;
    pub const INFO_DIST: u32 = 0x8;
    pub const INFO_ANGLE: u32 = 0xc;
    pub const INFO_AGGRESSIVE: u32 = 0x1c;
    /// `Rider`: the distance window reset each pass (-50000 / 50000).
    pub const MIN_DIST: u32 = 0xdc;
    pub const MAX_DIST: u32 = 0xe0;
    /// `Rider`: the embedded object whose virtual slot 0x28 returns the
    /// position (`sVec4*`).
    pub const POS_OBJ: u32 = 0x6c0;
    pub const GET_POS_SLOT: u32 = 0x28;
    /// gp-relative constants: -50000, 50000, pi, -pi/2, pi/2, 1/(2 pi), 2 pi.
    pub const LIT_MIN_DIST: u32 = 0x7cac;
    pub const LIT_MAX_DIST: u32 = 0x7ca8;
    pub const LIT_PI: u32 = 0x7ca4;
    pub const LIT_NEG_HALF_PI: u32 = 0x7ca0;
    pub const LIT_HALF_PI: u32 = 0x7c9c;
    pub const LIT_INV_TWO_PI: u32 = 0x7c98;
    pub const LIT_TWO_PI: u32 = 0x7c94;

    pub const UPDATE_RACE_POSITIONS: Callee = Callee {
        addr: 0x0010_f998,
        f: ee::s00::AI_UpdateRacePositions,
    };
    pub const UPDATE_RIVALS: Callee = Callee {
        addr: 0x0010_f878,
        f: ee::s00::AI_UpdateRivals,
    };
    /// `Math_Atan(x)` (cross-shard).
    pub const ATAN: u32 = 0x0031_c228;
    /// `BEAggressionInt_GetInstance()`, `BEAggressionInt_GetRivalLevel(this, a, b)`.
    pub const AGGRESSION_GET_INSTANCE: u32 = 0x0015_5a50;
    pub const AGGRESSION_GET_RIVAL_LEVEL: u32 = 0x0015_5b50;
}

/// `void AI_UpdateRiderRelations(AI* ai)` (0x10f560): every sixth frame,
/// update the race positions, then for every pair of riders (i < j) that
/// rider i tracks: the distance between them in the ground plane
/// (`sqrt(dx^2 + dy^2)`, stored both ways), the angle from j to i
/// (`atan2(dy, dx)`, stored at j) and its opposite wrapped into
/// [-pi, pi) (stored at i), and whether i's rivalry level towards j is at
/// least 2. Also resets each rider's distance window, then updates rivals.
pub fn ai_update_rider_relations(c: &mut Cpu) {
    use rider_relations::*;
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = 6;
        for (reg, off) in [
            (S0, 0xc0),
            (S1, 0xb0),
            (S2, 0xa0),
            (S3, 0x90),
            (S4, 0x80),
            (S5, 0x70),
            (S6, 0x60),
            (S7, 0x50),
            (FP, 0x40),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x30);
        for (f, off) in [(24, 0xf0), (23, 0xe8), (22, 0xe0), (21, 0xd8), (20, 0xd0)] {
            frame.save_fpr(c, f, off);
        }
        // Stack slots: 0x20 the AI, 0x24/0x28 its rider array.
        let slot = |c: &Cpu, off: u32| sp(c).wrapping_add(off);
        c.sw(slot(c, 0x20), c.g[A0][0]);
        'out: {
            // 0x10f5b0: only every sixth frame.
            c.cycles += 5;
            c.g[A0][0] = c.lw(at(c, A0, FRAME));
            c.div(0, A0, V0);
            c.g[V1][0] = c.hi[0];
            frame.restore(c, S0, 0xc0);
            if c.g[V1][0] != 0 {
                break 'out;
            }
            c.cycles += 9;
            c.g[A0][0] = c.lw(slot(c, 0x20));
            UPDATE_RACE_POSITIONS.call(c, 0x0010_f5cc)?;
            c.g[V1][0] = c.lw(slot(c, 0x20));
            c.g[V0][0] = c.lw(at(c, V1, COUNT));
            c.g[V1][0] = c.lw(at(c, V1, SKIP));
            c.g[S6][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
            let few = (c.g[S6][0] as i64) < 2;
            c.g[A0][0] = few as u64;
            frame.restore(c, S0, 0xc0);
            if few {
                break 'out;
            }
            c.cycles += 2;
            c.g[S4][0] = 0;
            if (c.g[S6][0] as i64) > 0 {
                c.cycles += 4;
                c.g[A0][0] = c.lw(slot(c, 0x20));
                c.g[A0][0] = sx(at(c, A0, RIDERS));
                c.sw(slot(c, 0x24), c.g[A0][0]);
                c.g[V0][0] = c.lw(slot(c, 0x24));
                // 0x10f600: rider i = s4 (fp = 4 i); v0 = the rider array.
                loop {
                    c.cycles += 13;
                    c.g[V1][0] = sx(r32(c, S4) << 2);
                    c.g[FP][0] = c.g[V1][0];
                    c.lwc1(1, gp_lit(c, LIT_MIN_DIST));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.lwc1(0, gp_lit(c, LIT_MAX_DIST));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.g[S2][0] = sx(r32(c, S4).wrapping_add(1));
                    c.g[S7][0] = c.g[S2][0];
                    let pairs = (c.g[S2][0] as i64) < (c.g[S6][0] as i64);
                    c.g[A0][0] = pairs as u64;
                    c.swc1(1, at(c, V0, MIN_DIST));
                    c.g[V0][0] = c.lw(r32(c, V1));
                    c.swc1(0, at(c, V0, MAX_DIST));
                    if pairs {
                        // s2 = j, s1 = j * 0x24 (i's info on j), s3/s5 =
                        // &riders[j].
                        c.cycles += 9;
                        c.g[V1][0] = c.lw(slot(c, 0x24));
                        c.g[A0][0] = INFO_SIZE as u64;
                        c.mult(0, S1, S7, A0);
                        c.g[V0][0] = sx(r32(c, S7) << 2);
                        c.sw(slot(c, 0x28), c.g[V1][0]);
                        c.lwc1(24, gp_lit(c, LIT_PI));
                        c.g[S3][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[S5][0] = c.g[S3][0];
                        loop {
                            let (riders_a, riders_b) = (slot(c, 0x24), slot(c, 0x28));
                            relate_pair(c, riders_a, riders_b)?;
                            c.cycles += 5;
                            c.g[S3][0] = sx(at(c, S3, 4));
                            c.g[S1][0] = sx(at(c, S1, INFO_SIZE));
                            let more = (c.g[S2][0] as i64) < (c.g[S6][0] as i64);
                            c.g[V0][0] = more as u64;
                            c.g[S5][0] = sx(at(c, S5, 4));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x0010_f658);
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[S4][0] = c.g[S7][0];
                    let more = (c.g[S4][0] as i64) < (c.g[S6][0] as i64);
                    c.g[V0][0] = c.lw(slot(c, 0x24));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0010_f600);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.lw(slot(c, 0x20));
            UPDATE_RIVALS.call(c, 0x0010_f834)?;
            frame.restore(c, S0, 0xc0);
        }
        c.cycles += 16;
        for (reg, off) in [
            (S1, 0xb0),
            (S2, 0xa0),
            (S3, 0x90),
            (S4, 0x80),
            (S5, 0x70),
            (S6, 0x60),
            (S7, 0x50),
            (FP, 0x40),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x30);
        for (f, off) in [(24, 0xf0), (23, 0xe8), (22, 0xe0), (21, 0xd8), (20, 0xd0)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// One (i, j) pair of [`ai_update_rider_relations`] (0x10f658): rider i
/// is `riders[fp / 4]`, rider j `*s3`/`*s5`, s1 = j * 0x24, s4 = i, s2 = j
/// (advanced here). `riders_a`/`riders_b` are the stack slots holding the
/// rider array (0x24, 0x28).
fn relate_pair(c: &mut Cpu, riders_a: u32, riders_b: u32) -> crate::call::Flow {
    use rider_relations::*;
    c.cycles += 7;
    c.g[V1][0] = c.lw(riders_a);
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, FP)));
    c.g[A1][0] = c.lw(r32(c, V0));
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, S1)));
    c.g[V0][0] = c.lw(at(c, V1, INFO_ACTIVE));
    if c.g[V0][0] == 0 {
        c.g[S2][0] = sx(at(c, S2, 1));
        return Ok(());
    }

    // 0x10f670: both positions (virtual getPos), their difference.
    c.cycles += 30;
    c.g[V1][0] = c.lw(at(c, A1, POS_OBJ));
    c.g[A1][0] = sx(at(c, A1, POS_OBJ));
    c.mtc1(0, 22);
    c.g[A0][0] = c.lh(at(c, V1, GET_POS_SLOT));
    c.g[V0][0] = c.lw(at(c, V1, GET_POS_SLOT + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x0010_f690)?;
    c.g[A0][0] = c.lw(r32(c, S5));
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = c.lw(at(c, A0, POS_OBJ));
    c.g[A0][0] = sx(at(c, A0, POS_OBJ));
    c.g[A1][0] = c.lh(at(c, V1, GET_POS_SLOT));
    c.g[V0][0] = c.lw(at(c, V1, GET_POS_SLOT + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    call_ptr(c, method, 0x0010_f6b0)?;
    c.lqc2(3, r32(c, S0));
    c.lqc2(4, r32(c, V0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, sp(c).wrapping_add(0x10));
    c.lq(2, sp(c).wrapping_add(0x10));
    c.sq(2, sp(c));
    // f20 = dx, f21 = dy, f23 = distance.
    c.lwc1(20, sp(c));
    c.lwc1(21, sp(c).wrapping_add(4));
    c.mul_s(0, 20, 20);
    c.mul_s(1, 21, 21);
    c.c_eq_s(20, 22);
    c.add_s(0, 0, 1);
    c.sqrt_s(23, 0);
    let dx_zero = c.fcc();

    // The angle atan2(dy, dx) into f12, f2 = angle + pi, a0 = the riders.
    'angle: {
        if !dx_zero {
            // 0x10f718: atan(dy / dx), corrected by pi for dx < 0.
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, ATAN, 0x0010_f724)?;
            c.c_lt_s(20, 22);
            let left = c.fcc();
            c.mov_s(12, 0);
            if left {
                c.cycles += 4;
                c.c_lt_s(22, 21);
                if !c.fcc() {
                    c.sub_s(0, 0, 24);
                } else {
                    c.cycles += 1;
                    c.add_s(0, 0, 24);
                }
                c.cycles += 1;
                c.mov_s(12, 0);
            }
        } else {
            // dx == 0: 0 (dy == 0), pi/2 (dy > 0), -pi/2 (dy < 0).
            c.cycles += 4;
            c.c_eq_s(21, 22);
            let dy_zero = c.fcc();
            c.mov_s(12, 21);
            if !dy_zero {
                c.cycles += 4;
                c.c_le_s(22, 21);
                c.lwc1(12, gp_lit(c, LIT_NEG_HALF_PI));
                let up = c.fcc();
                c.g[A0][0] = c.lw(riders_b);
                if up {
                    c.cycles += 3;
                    c.lwc1(12, gp_lit(c, LIT_HALF_PI));
                    c.add_s(2, 12, 24);
                    break 'angle;
                }
            }
        }
        c.cycles += 2;
        c.g[A0][0] = c.lw(riders_b);
        c.add_s(2, 12, 24);
    }

    // 0x10f754: distance both ways; f1 = floor((angle + pi) / 2 pi).
    c.cycles += 18;
    c.lwc1(0, gp_lit(c, LIT_INV_TWO_PI));
    c.g[V1][0] = INFO_SIZE as u64;
    c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, FP)));
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    c.g[V0][0] = c.lw(r32(c, S0));
    c.mul_s(0, 2, 0);
    c.mult(0, A0, S4, V1);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
    c.swc1(23, at(c, V0, INFO_DIST));
    c.add_s(0, 0, 1);
    c.cvt_w_s(1, 0);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = c.lw(r32(c, S3));
    c.c_lt_s(0, 1);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    let rounded_up = c.fcc();
    c.swc1(23, at(c, V0, INFO_DIST));
    if rounded_up {
        c.cycles += 3;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.sub_s(1, 1, 0);
    }

    // 0x10f7a8: the angles (i's wrapped, j's raw), then the rivalry level.
    c.cycles += 17;
    c.lwc1(0, gp_lit(c, LIT_TWO_PI));
    c.g[V1][0] = c.lw(r32(c, S0));
    c.mul_s(0, 1, 0);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
    c.sub_s(0, 2, 0);
    c.swc1(0, at(c, V1, INFO_ANGLE));
    c.g[V0][0] = c.lw(r32(c, S3));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.swc1(12, at(c, V0, INFO_ANGLE));
    call_ptr(c, AGGRESSION_GET_INSTANCE, 0x0010_f7d0)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S2][0];
    call_ptr(c, AGGRESSION_GET_RIVAL_LEVEL, 0x0010_f7e0)?;
    let mild = (c.g[V0][0] as i64) < 2;
    c.g[V0][0] = c.lw(r32(c, S0));
    if mild {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.sw(at(c, V0, INFO_AGGRESSIVE), 0);
    } else {
        c.cycles += 4;
        c.g[V1][0] = 1;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
        c.sw(at(c, V0, INFO_AGGRESSIVE), 1);
    }
    c.cycles += 1;
    c.g[S2][0] = sx(at(c, S2, 1));
    Ok(())
}

// ---------------------------------------------------------------------------
// HUD_DrawTime (0x1f1840, src/fe/ovstatehud.cpp `cOVStateHUD1P::renderTime`).

/// Fields and strings of the HUD time display.
pub mod hud_time {
    use super::{ee, Callee};
    /// `HUD`: separator width, digit width, '1' narrowing, sign advance,
    /// the font, the separator string (":").
    pub const SEP_W: u32 = 0x16c;
    pub const DIGIT_W: u32 = 0x170;
    pub const SIGN_ADVANCE: u32 = 0x174;
    pub const ONE_W: u32 = 0x178;
    pub const FONT: u32 = 0x42c;
    pub const SEPARATOR: u32 = 0x430;
    /// `Font`: line count (int), base size x/y, scaled size x/y, colour.
    pub const FONT_LINES: u32 = 0x14;
    pub const FONT_BASE_W: u32 = 0x30;
    pub const FONT_BASE_H: u32 = 0x34;
    pub const FONT_SIZE_W: u32 = 0x38;
    pub const FONT_SIZE_H: u32 = 0x3c;
    pub const FONT_COLOUR: u32 = 0x40;
    /// Strings: "+", "-", "%02d", ".%02d".
    pub const STR_PLUS: u32 = 0x004a_2100;
    pub const STR_MINUS: u32 = 0x004a_2108;
    pub const FMT_TWO_DIGITS: u32 = 0x004a_22f0;
    pub const FMT_FRACTION: u32 = 0x004a_22f8;
    /// Flags (t1): bit 0 draws the ".xx" fraction, bits 1/2 a sign ('-' if
    /// bit 1).
    pub const FLAG_FRACTION: u64 = 1;
    pub const FLAG_MINUS: u64 = 2;
    pub const FLAG_SIGN: u64 = 6;

    pub const HUD_ALIGN_ELEM_POS: Callee = Callee {
        addr: 0x001f_10f8,
        f: ee::s04::HUD_AlignElemPos,
    };
    pub const FONT_DRAW_TEXT_VARARGS: Callee = Callee {
        addr: 0x001d_c990,
        f: ee::s04::Font_DrawTextVarargs,
    };
    /// `Font_DrawText(font, text, x, y)`, `sprintf` (cross-shard).
    pub const FONT_DRAW_TEXT: u32 = 0x0039_1cb0;
    pub const SPRINTF: u32 = 0x0041_7828;
}

/// `void HUD_DrawTime(HUD* hud, int a, int b, int c, int frac (t0), int
/// flags (t1), const float pos[2] (t2), const float scale[2] (t3), const
/// sColor* colour, int hAlign, int vAlign, int screen)` (0x1f1840): draws a
/// race time "aa:bb:cc" (each "%02d") with an optional sign in front and
/// ".frac" behind. Lays the box out (`HUD_AlignElemPos`, width from three
/// or four digit groups), scales and colours the font, then draws group by
/// group; a group starting with '1' is shifted right by the narrow-'1'
/// width per leading '1'. Frame 0xe0 (s0-s7, f20-f22; the size at +0, the
/// pen position at +0x10, the three strings at +0x20/+0x24/+0x28).
pub fn hud_draw_time(c: &mut Cpu) {
    use hud_time::*;
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xe0);
        c.g[AT][0] = 0x4040_0000; // 3.0: digit groups
        c.mtc1(AT, 5);
        for (reg, off) in [
            (S3, 0x80),
            (S5, 0x60),
            (S0, 0xb0),
            (S1, 0xa0),
            (S4, 0x70),
            (S6, 0x50),
            (S7, 0x40),
            (S2, 0x90),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x30);
        c.g[S3][0] = c.g[T1][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[A2][0] = c.g[T2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S6][0] = c.g[A3][0];
        c.g[S7][0] = c.g[T0][0];
        c.g[S0][0] = c.g[T3][0];
        for (f, off) in [(22, 0xd0), (21, 0xc8), (20, 0xc0)] {
            frame.save_fpr(c, f, off);
        }
        c.g[V0][0] = c.g[S3][0] & FLAG_FRACTION;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[AT][0] = 0x4080_0000; // 4.0
            c.mtc1(AT, 5);
        }
        let hud = r32(c, S1);
        let stk = |c: &Cpu, off: u32| sp(c).wrapping_add(off);

        // 0x1f18ac: the box size {groups * sep + (digit + 4) * (groups - 1),
        // lines * base height}, aligned into the pen position.
        c.cycles += 57;
        c.g[AT][0] = 0x4000_0000; // 2.0
        c.mtc1(AT, 20);
        c.g[A0][0] = c.g[S1][0];
        c.lwc1(0, hud.wrapping_add(SEP_W));
        c.g[A1][0] = sx(stk(c, 0x10));
        c.g[AT][0] = 0x3f80_0000; // 1.0
        c.mtc1(AT, 3);
        c.g[A3][0] = c.g[SP][0];
        c.add_s(0, 0, 20);
        c.g[V0][0] = c.lw(hud.wrapping_add(FONT));
        c.sub_s(3, 5, 3);
        c.lwc1(1, hud.wrapping_add(DIGIT_W));
        c.lwc1(2, at(c, V0, FONT_LINES));
        c.cvt_s_w(2, 2);
        c.lwc1(4, at(c, V0, FONT_BASE_H));
        c.add_s(0, 0, 20);
        c.g[T1][0] = c.lw(stk(c, 0xe8));
        c.mul_s(1, 1, 5);
        c.g[T2][0] = c.lw(stk(c, 0xf0));
        c.mul_s(2, 2, 4);
        c.g[T3][0] = c.lw(stk(c, 0xf8));
        c.mul_s(0, 0, 3);
        c.g[T0][0] = c.g[S0][0];
        c.swc1(2, stk(c, 0x4));
        c.add_s(1, 1, 0);
        c.swc1(1, stk(c, 0));
        HUD_ALIGN_ELEM_POS.call(c, 0x001f_1918)?;
        // Font size = base * scale; colour; f21/f20/f22 = scaled digit
        // advance, '1' narrowing, separator advance.
        c.g[V0][0] = c.lw(hud.wrapping_add(FONT));
        c.g[A0][0] = c.g[S3][0] & FLAG_SIGN;
        c.lwc1(1, r32(c, S0));
        c.lwc1(0, at(c, V0, FONT_BASE_W));
        c.lwc1(2, at(c, V0, FONT_BASE_H));
        c.mul_s(0, 0, 1);
        c.swc1(0, at(c, V0, FONT_SIZE_W));
        c.lwc1(1, at(c, S0, 4));
        c.mul_s(2, 2, 1);
        c.swc1(2, at(c, V0, FONT_SIZE_H));
        c.g[V0][0] = c.lw(stk(c, 0xe0));
        c.g[V1][0] = c.lw(hud.wrapping_add(FONT));
        c.ldl(A1, at(c, V0, 0x7));
        c.ldr(A1, at(c, V0, 0));
        c.ldl(A2, at(c, V0, 0xf));
        c.ldr(A2, at(c, V0, 0x8));
        c.sdl(A1, at(c, V1, FONT_COLOUR + 7));
        c.sdr(A1, at(c, V1, FONT_COLOUR));
        c.sdl(A2, at(c, V1, FONT_COLOUR + 0xf));
        c.sdr(A2, at(c, V1, FONT_COLOUR + 8));
        c.lwc1(3, hud.wrapping_add(DIGIT_W));
        c.lwc1(2, hud.wrapping_add(SEP_W));
        c.add_s(3, 3, 20);
        c.lwc1(1, r32(c, S0));
        c.add_s(2, 2, 20);
        c.lwc1(0, hud.wrapping_add(ONE_W));
        c.mul_s(21, 3, 1);
        c.mul_s(20, 0, 1);
        c.mul_s(22, 2, 1);
        if c.g[A0][0] != 0 {
            // 0x1f1994: the sign.
            c.cycles += 3;
            c.g[V0][0] = c.g[S3][0] & FLAG_MINUS;
            c.lwc1(12, stk(c, 0x10));
            let (text, ret_to) = if c.g[V0][0] != 0 {
                c.cycles += 7;
                (STR_MINUS, 0x001f_19b0)
            } else {
                c.cycles += 6;
                (STR_PLUS, 0x001f_19cc)
            };
            c.g[A1][0] = sx(text);
            c.g[A0][0] = c.lw(hud.wrapping_add(FONT));
            c.lwc1(13, stk(c, 0x14));
            call_ptr(c, FONT_DRAW_TEXT, ret_to)?;
            c.lwc1(0, stk(c, 0x10));
            c.cycles += 3;
            c.lwc1(1, hud.wrapping_add(SIGN_ADVANCE));
            c.add_s(0, 0, 1);
            c.swc1(0, stk(c, 0x10));
        }

        // 0x1f19dc: the three groups as text.
        c.cycles += 20;
        c.g[S0][0] = sx(FMT_TWO_DIGITS);
        c.g[S2][0] = sx(stk(c, 0x20));
        c.g[A2][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, SPRINTF, 0x001f_19f8)?;
        c.g[A0][0] = sx(stk(c, 0x24));
        c.g[A2][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, SPRINTF, 0x001f_1a08)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S6][0];
        c.g[A0][0] = sx(stk(c, 0x28));
        call_ptr(c, SPRINTF, 0x001f_1a18)?;
        // f0 = '1' narrowing per leading '1' of the group (two digits).
        c.g[V0][0] = c.lb(stk(c, 0x20));
        c.g[V1][0] = b'1' as u64;
        c.mtc1(0, 0);
        if c.g[V0][0] == c.g[V1][0] {
            c.mov_s(0, 20);
        }
        c.cycles += 3;
        c.g[V0][0] = c.lb(stk(c, 0x21));
        if c.g[V0][0] == c.g[V1][0] {
            c.add_s(0, 0, 20);
        }
        c.cycles += 14;
        c.lwc1(12, stk(c, 0x10));
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.lw(hud.wrapping_add(FONT));
        c.g[S4][0] = c.g[S3][0] & FLAG_FRACTION;
        c.add_s(12, 12, 0);
        c.lwc1(13, stk(c, 0x14));
        c.g[S3][0] = b'1' as u64;
        c.g[S2][0] = 1;
        call_ptr(c, FONT_DRAW_TEXT, 0x001f_1a5c)?;
        c.lwc1(0, stk(c, 0x10));
        c.g[S0][0] = sx(stk(c, 0x24));
        c.add_s(0, 0, 21);
        c.swc1(0, stk(c, 0x10));
        // 0x1f1a70: separator, then the next group (s2 counts down 1, 0).
        loop {
            c.cycles += 12;
            c.g[A0][0] = c.lw(hud.wrapping_add(FONT));
            c.lwc1(12, stk(c, 0x10));
            c.lwc1(13, stk(c, 0x14));
            c.g[A1][0] = c.lw(hud.wrapping_add(SEPARATOR));
            call_ptr(c, FONT_DRAW_TEXT, 0x001f_1a84)?;
            c.lwc1(0, stk(c, 0x10));
            c.mtc1(0, 1);
            c.add_s(0, 0, 22);
            c.swc1(0, stk(c, 0x10));
            c.g[V0][0] = c.lb(r32(c, S0));
            if c.g[V0][0] == c.g[S3][0] {
                c.mov_s(1, 20);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lb(at(c, S0, 1));
            if c.g[V0][0] == c.g[S3][0] {
                c.add_s(1, 1, 20);
            }
            c.cycles += 11;
            c.add_s(12, 0, 1);
            c.g[A0][0] = c.lw(hud.wrapping_add(FONT));
            c.lwc1(13, stk(c, 0x14));
            c.g[A1][0] = c.g[S0][0];
            c.g[S0][0] = sx(at(c, S0, 4));
            c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
            call_ptr(c, FONT_DRAW_TEXT, 0x001f_1ac8)?;
            c.lwc1(0, stk(c, 0x10));
            c.add_s(12, 0, 21);
            let more = (c.g[S2][0] as i64) >= 0;
            c.swc1(12, stk(c, 0x10));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001f_1a70);
            }
        }
        c.cycles += 2;
        c.g[A1][0] = 0x4a_0000;
        if c.g[S4][0] != 0 {
            // ".%02d" with the fraction.
            c.cycles += 5;
            c.g[A0][0] = c.lw(hud.wrapping_add(FONT));
            c.lwc1(13, stk(c, 0x14));
            c.g[A1][0] = sx(FMT_FRACTION);
            c.g[A2][0] = c.g[S7][0];
            FONT_DRAW_TEXT_VARARGS.call(c, 0x001f_1af4)?;
        }
        c.cycles += 14;
        for (reg, off) in [
            (S0, 0xb0),
            (S1, 0xa0),
            (S2, 0x90),
            (S3, 0x80),
            (S4, 0x70),
            (S5, 0x60),
            (S6, 0x50),
            (S7, 0x40),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x30);
        for (f, off) in [(22, 0xd0), (21, 0xc8), (20, 0xc0)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// __ieee754_logf (0x40e550, newlib e_logf.c).

/// `at = bits; mtc1 at, f` (a float literal through `at`).
#[inline(always)]
fn lit(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(AT, f);
}

/// ln2_hi, ln2_lo (newlib's split of ln 2).
const LN2_HI: u32 = 0x3f31_7180;
const LN2_LO: u32 = 0x3717_f7d1;

/// `float __ieee754_logf(float x)` (0x40e550): newlib's single-precision
/// natural logarithm. x = 2^k (1 + f): subnormals are scaled by 2^25
/// first; log(+-0) is the constant at 0x499224 (-huge), log(x < 0) =
/// (x - x) / 0, log(inf/NaN) = x + x. For tiny f a short polynomial; else
/// s = f / (2 + f) and the Remez series in s^2, combined with k ln 2
/// (split hi/lo). A leaf; every scratch register (a0-a3, t0, v0, v1, at,
/// f0-f10) left as the compiled code leaves it.
pub fn ieee754_logf(c: &mut Cpu) {
    c.cycles += 7;
    c.mfc1(A0, 12);
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x7f_ffff;
    let normal = 0x7f_ffff < (c.g[A3][0] as i64);
    c.g[V0][0] = normal as u64;
    c.g[T0][0] = 0;
    if !normal {
        // 0x40e574: zero, negative or subnormal.
        c.cycles += 5;
        let nonzero = c.g[A3][0] & 0x7fff_ffff != 0;
        c.g[V0][0] = 0x4a_0000;
        if !nonzero {
            c.cycles += 2;
            c.lwc1(0, 0x004a_0000u32.wrapping_sub(0x6ddc));
            return ret(c);
        }
        c.cycles += 2;
        c.g[T0][0] = sx(-25i32 as u32);
        if (c.g[A3][0] as i64) < 0 {
            // (x - x) / 0.
            c.cycles += 8;
            c.mtc1(A0, 0);
            c.sub_s(1, 0, 0);
            c.mtc1(0, 0);
            c.div_s(0, 1, 0);
            return ret(c);
        }
        // Subnormal: x * 2^25.
        c.cycles += 6;
        lit(c, 0, 0x4c00_0000);
        c.mtc1(A0, 1);
        c.mul_s(1, 1, 0);
        c.mfc1(A0, 1);
        c.g[A3][0] = c.g[A0][0];
    }
    // 0x40e5c8
    c.cycles += 5;
    let finite = (c.g[A3][0] as i64) <= 0x7f7f_ffff;
    c.g[V0][0] = (!finite) as u64;
    c.g[A1][0] = 0x7f_0000;
    if !finite {
        c.cycles += 3;
        c.mtc1(A0, 2);
        c.add_s(0, 2, 2);
        return ret(c);
    }

    // 0x40e5e0: k (t0), the mantissa normalised into [sqrt(2)/2, sqrt(2))
    // (a0), f = x - 1 (f12).
    c.cycles += 23;
    let hx = r32(c, A3);
    c.g[A2][0] = sx(((hx as i32) >> 23) as u32);
    c.g[A1][0] = 0x7f_ffff;
    let mant = hx & 0x7f_ffff;
    c.g[A3][0] = mant as u64;
    c.g[V0][0] = sx(r32(c, T0).wrapping_sub(0x7f));
    let i = mant.wrapping_add(0x4a_fb20) & 0x80_0000;
    c.g[A0][0] = 0x80_0000;
    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
    c.g[V0][0] = (i ^ 0x3f80_0000) as u64;
    c.g[A0][0] = (mant | i ^ 0x3f80_0000) as u64;
    c.g[V1][0] = sx(((i as i32) >> 23) as u32);
    lit(c, 0, 0x3f80_0000);
    let tiny = (mant.wrapping_add(0xf) & 0x7f_ffff) < 16;
    c.mtc1(A0, 1);
    c.g[T0][0] = sx(r32(c, T0).wrapping_add(r32(c, V1)));
    c.g[V0][0] = tiny as u64;
    c.sub_s(12, 1, 0);
    let k_zero = c.g[T0][0] == 0;
    'tail: {
        if tiny {
            // 0x40e7f4: |f| < 2^-20.
            c.cycles += 5;
            c.mtc1(0, 0);
            c.c_eq_s(12, 0);
            if c.fcc() {
                // f == 0: k ln 2 (0 when k == 0).
                c.cycles += 2;
                if k_zero {
                    break 'tail;
                }
                c.cycles += 12;
                c.mtc1(T0, 9);
                c.cvt_s_w(9, 9);
                lit(c, 1, LN2_HI);
                lit(c, 0, LN2_LO);
                c.mul_s(1, 9, 1);
                c.mul_s(0, 9, 0);
                c.add_s(0, 1, 0);
                return ret(c);
            }
            // R = f^2 (0.5 - f / 3).
            c.mul_s(2, 12, 12);
            c.cycles += 9;
            lit(c, 0, 0x3eaa_aaab);
            lit(c, 1, 0x3f00_0000);
            c.mul_s(0, 12, 0);
            c.sub_s(1, 1, 0);
            c.mul_s(2, 2, 1);
            if !k_zero {
                c.cycles += 14;
                c.mtc1(T0, 9);
                c.cvt_s_w(9, 9);
                lit(c, 0, LN2_LO);
                lit(c, 1, LN2_HI);
                c.mul_s(0, 9, 0);
                c.mul_s(1, 9, 1);
                c.sub_s(0, 2, 0);
                c.sub_s(0, 0, 12);
                c.sub_s(0, 1, 0);
            } else {
                c.cycles += 2;
                c.sub_s(0, 12, 2);
            }
            return ret(c);
        }

        // 0x40e660: s = f / (2 + f), R = the series in s^2 (f2).
        c.cycles += 52;
        lit(c, 0, 0x4000_0000);
        c.g[V1][0] = 0xffff_ffff_ffcf_0000;
        lit(c, 1, 0x3e17_8897); // Lg7
        c.g[V0][0] = 0x35_0000;
        c.add_s(0, 12, 0);
        lit(c, 8, 0x3e3a_3325); // Lg5
        lit(c, 2, 0x3e1c_d04f); // Lg6
        c.g[V1][0] |= 0x5c30;
        lit(c, 4, 0x3e63_8e29); // Lg4
        c.g[V0][0] |= 0xc288;
        c.div_s(10, 12, 0);
        lit(c, 5, 0x3e92_4925); // Lg3
        lit(c, 6, 0x3ecc_cccd); // Lg2
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(mant));
        lit(c, 7, 0x3f2a_aaab); // Lg1
        c.g[V1][0] = sx(mant.wrapping_add(r32(c, V1)));
        c.g[V1][0] |= c.g[V0][0];
        c.mtc1(T0, 9);
        c.cvt_s_w(9, 9);
        c.mul_s(3, 10, 10);
        c.mul_s(0, 3, 3);
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.add_s(1, 1, 8);
        c.add_s(2, 2, 4);
        c.mul_s(1, 0, 1);
        c.mul_s(2, 0, 2);
        c.add_s(1, 1, 5);
        c.add_s(2, 2, 6);
        c.mul_s(1, 0, 1);
        c.mul_s(0, 0, 2);
        c.add_s(1, 1, 7);
        c.mul_s(3, 3, 1);
        let small = (c.g[V1][0] as i64) <= 0;
        c.add_s(2, 3, 0);
        if small {
            // f - s (f - R).
            c.cycles += 2;
            c.sub_s(0, 12, 2);
            if k_zero {
                c.cycles += 3;
                c.mul_s(0, 10, 0);
                c.sub_s(0, 12, 0);
                return ret(c);
            }
            c.cycles += 12;
            lit(c, 1, LN2_LO);
            lit(c, 2, LN2_HI);
            c.mul_s(1, 9, 1);
            c.mul_s(0, 10, 0);
            c.mul_s(2, 9, 2);
            c.sub_s(0, 0, 1);
            c.sub_s(0, 0, 12);
            c.sub_s(0, 2, 0);
            break 'tail;
        }
        // hfsq = f^2 / 2 (f4).
        c.cycles += 5;
        lit(c, 0, 0x3f00_0000);
        c.mul_s(0, 12, 0);
        c.mul_s(4, 0, 12);
        if !k_zero {
            c.cycles += 15;
            c.add_s(0, 4, 2);
            lit(c, 1, LN2_LO);
            lit(c, 2, LN2_HI);
            c.mul_s(1, 9, 1);
            c.mul_s(0, 10, 0);
            c.mul_s(2, 9, 2);
            c.add_s(0, 0, 1);
            c.sub_s(0, 4, 0);
            c.sub_s(0, 0, 12);
            c.sub_s(0, 2, 0);
        } else {
            c.cycles += 5;
            c.add_s(0, 4, 2);
            c.mul_s(0, 10, 0);
            c.sub_s(0, 4, 0);
            c.sub_s(0, 12, 0);
        }
        return ret(c);
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SnowFall_Update (0x2e5da0, src/visualfx/boardwakefx.cpp).

/// Globals, records and callees of the snowfall update.
pub mod snowfall {
    use super::{ee, Callee};
    /// `gp-0x848`: the app (`+0x84` game module, `+0x84` its weather
    /// object, whose `+0x10` is the number of splash managers / views).
    pub const GP_APP: u32 = 0x848;
    /// gp+0x164c: non-zero when the weather comes from the override block
    /// (gp+0x1650..0x166c and the colours at 0x5059b8 / 0x5059c8) instead of
    /// the world painter.
    pub const GP_WEATHER_OVERRIDE: u32 = 0x164c;
    pub const GP_OVERRIDE_F: [u32; 8] = [0x1650, 0x1654, 0x1668, 0x166c, 0x1658, 0x165c, 0x1660, 0x1664];
    pub const OVERRIDE_C0: u32 = 0x0050_59b8;
    pub const OVERRIDE_C1: u32 = 0x0050_59c8;
    /// One weather record per view on the stack (0x40 bytes: f[8], c0, c1).
    pub const REC_SIZE: u32 = 0x40;
    /// `SnowFall`: twelve `SnowBox*` from +0x10 (a box's first word is its
    /// view index).
    pub const BOXES: u32 = 0x10;

    /// `WorldPainter_GetWeatherNN(kind)`, in record order.
    pub const GETTERS: [Callee; 16] = [
        Callee {
            addr: 0x002e_e448,
            f: ee::s09::WorldPainter_GetWeather08,
        },
        Callee {
            addr: 0x002e_e508,
            f: ee::s09::WorldPainter_GetWeather18,
        },
        Callee {
            addr: 0x002e_e600,
            f: ee::s09::WorldPainter_GetWeather48,
        },
        Callee {
            addr: 0x002e_e660,
            f: ee::s09::WorldPainter_GetWeather50,
        },
        Callee {
            addr: 0x002e_e4c0,
            f: ee::s09::WorldPainter_GetWeather10,
        },
        Callee {
            addr: 0x002e_e570,
            f: ee::s09::WorldPainter_GetWeather20,
        },
        Callee {
            addr: 0x002e_e5b8,
            f: ee::s09::WorldPainter_GetWeather38,
        },
        Callee {
            addr: 0x002e_e6a8,
            f: ee::s09::WorldPainter_GetWeather40,
        },
        Callee {
            addr: 0x002e_e780,
            f: ee::s09::WorldPainter_GetWeather60,
        },
        Callee {
            addr: 0x002e_e7c8,
            f: ee::s09::WorldPainter_GetWeather68,
        },
        Callee {
            addr: 0x002e_e810,
            f: ee::s09::WorldPainter_GetWeather70,
        },
        Callee {
            addr: 0x002e_e858,
            f: ee::s09::WorldPainter_GetWeather78,
        },
        Callee {
            addr: 0x002e_e8a0,
            f: ee::s09::WorldPainter_GetWeather80,
        },
        Callee {
            addr: 0x002e_e8e8,
            f: ee::s09::WorldPainter_GetWeather88,
        },
        Callee {
            addr: 0x002e_e930,
            f: ee::s09::WorldPainter_GetWeather90,
        },
        Callee {
            addr: 0x002e_e978,
            f: ee::s09::WorldPainter_GetWeather98,
        },
    ];
    /// `GameModule_GetSplashManager(gm, i)` (cross-shard).
    pub const GET_SPLASH_MANAGER: u32 = 0x0023_06a8;
    pub const SET_SNOW_AMOUNT: Callee = Callee {
        addr: 0x002f_4330,
        f: ee::s09::cSplashManager_SetSnowAmount,
    };
    pub const SNOW_BOX_UPDATE: Callee = Callee {
        addr: 0x002e_4f50,
        f: ee::s09::SnowBox_Update,
    };
}

/// `void SnowFall_Update(SnowFall* self)` (0x2e5da0): for each view, build
/// the weather record on the stack (from the override block when gp+0x164c
/// is set, else from the world painter's weather of kind 6 for view 0 and
/// 7 for the others) and pass its first value to that view's splash manager
/// as the snow amount; then update each of the twelve snow boxes with the
/// record of its view. Frame 0x120 (records at +0, s0-s7, fp).
pub fn snowfall_update(c: &mut Cpu) {
    use snowfall::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x120);
        c.g[A1][0] = c.lw(gp_lit(c, GP_APP));
        for (reg, off) in [
            (S3, 0xe0),
            (FP, 0x90),
            (S0, 0x110),
            (S1, 0x100),
            (S2, 0xf0),
            (S4, 0xd0),
            (S5, 0xc0),
            (S6, 0xb0),
            (S7, 0xa0),
        ] {
            frame.save(c, reg, off);
        }
        c.g[FP][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        c.g[V0][0] = c.lw(at(c, A1, 0x84));
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x10));
        c.g[S3][0] = 0;
        if c.g[A0][0] != 0 {
            // s3 = view, s2 = its record, s1 = record + 0x3c.
            c.cycles += 8;
            c.g[S7][0] = 0x50_0000;
            c.g[S6][0] = 0x50_0000;
            c.g[S5][0] = sx(OVERRIDE_C0);
            c.g[S4][0] = sx(OVERRIDE_C1);
            c.g[S2][0] = c.g[SP][0];
            c.g[S1][0] = sx(sp(c).wrapping_add(0x3c));
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_WEATHER_OVERRIDE));
            loop {
                // 0x2e5e08
                c.cycles += 2;
                let gp = r32(c, GP);
                c.lwc1(0, gp.wrapping_add(GP_OVERRIDE_F[0]));
                if c.g[V0][0] != 0 {
                    c.cycles += 32;
                    for (f, &off) in GP_OVERRIDE_F.iter().enumerate().skip(1) {
                        c.lwc1(f, gp.wrapping_add(off));
                    }
                    for k in 0..4 {
                        c.lwc1(8 + k, OVERRIDE_C0 + 4 * k as u32);
                    }
                    for k in 0..4 {
                        c.lwc1(12 + k, OVERRIDE_C1 + 4 * k as u32);
                    }
                    let rec = r32(c, S2);
                    for f in 0..16 {
                        c.swc1(f, rec.wrapping_add(4 * f as u32));
                    }
                } else {
                    c.cycles += 51;
                    c.g[V0][0] = 6;
                    c.g[S0][0] = if c.g[S3][0] == 0 { 6 } else { 7 };
                    for (n, getter) in GETTERS.iter().enumerate() {
                        c.g[A0][0] = c.g[S0][0];
                        getter.call(c, 0x002e_5ea4 + 0xc * n as u32)?;
                        let slot = r32(c, S2).wrapping_add(4 * n as u32);
                        c.swc1(0, slot);
                    }
                }
                // 0x2e5f5c: the view's splash manager gets f[0].
                c.cycles += 17;
                c.g[V0][0] = c.lw(gp_lit(c, GP_APP));
                c.g[A1][0] = c.g[S3][0];
                c.g[S3][0] = sx(at(c, S3, 1));
                c.g[S1][0] = sx(at(c, S1, REC_SIZE));
                c.g[A0][0] = c.lw(at(c, V0, 0x84));
                call_ptr(c, GET_SPLASH_MANAGER, 0x002e_5f74)?;
                c.lwc1(12, r32(c, S2));
                c.g[A0][0] = c.g[V0][0];
                c.g[S2][0] = sx(at(c, S2, REC_SIZE));
                SET_SNOW_AMOUNT.call(c, 0x002e_5f84)?;
                c.g[V0][0] = c.lw(gp_lit(c, GP_APP));
                c.g[V1][0] = c.lw(at(c, V0, 0x84));
                c.g[A0][0] = c.lw(at(c, V1, 0x84));
                c.g[V0][0] = c.lw(at(c, A0, 0x10));
                let more = c.g[S3][0] < c.g[V0][0];
                c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_WEATHER_OVERRIDE));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002e_5e08);
                }
            }
        }

        // 0x2e5fa0: the twelve boxes (s1 counts 11..0).
        c.cycles += 4;
        c.g[S0][0] = sx(at(c, FP, BOXES));
        c.g[S1][0] = 0xb;
        c.g[A1][0] = c.lw(r32(c, S0));
        loop {
            c.cycles += 4;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[S0][0] = sx(at(c, S0, 4));
            c.g[A0][0] = c.g[A1][0];
            if c.g[A1][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = c.lw(r32(c, A1));
                c.g[A1][0] = sx(r32(c, A1) << 6);
                c.g[A1][0] = sx(sp(c).wrapping_add(r32(c, A1)));
                SNOW_BOX_UPDATE.call(c, 0x002e_5fd0)?;
            }
            c.cycles += 2;
            if (c.g[S1][0] as i64) < 0 {
                break;
            }
            c.g[A1][0] = c.lw(r32(c, S0));
            if event_due(c) {
                deliver_event_here(c, 0x002e_5fb0);
            }
        }
        c.cycles += 12;
        for (reg, off) in [
            (S0, 0x110),
            (S1, 0x100),
            (S2, 0xf0),
            (S3, 0xe0),
            (S4, 0xd0),
            (S5, 0xc0),
            (S6, 0xb0),
            (S7, 0xa0),
            (FP, 0x90),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_Service (0x3b71d8, EA sound library: streams).

/// Stream table, stream fields and callees of the stream service.
pub mod sndstrm_service {
    use super::{ee, Callee};
    /// `Stream*[]` and the sound state (`+0x46` the stream count).
    pub const STREAMS: u32 = 0x0050_a0a0;
    pub const SND_STATE: u32 = 0x0050_a8e8;
    pub const STREAM_COUNT: u32 = 0x46;
    /// `Stream`: its stream file, playback slot, state (signed byte: 1
    /// feeding, 2 playing), the open flag, the prebuffer record.
    pub const FILE: u32 = 0x0;
    pub const SLOT: u32 = 0x8;
    pub const STATE: u32 = 0x10;
    pub const OPEN: u32 = 0x124;
    pub const STATE_FEEDING: u64 = 1;
    pub const STATE_PLAYING: u64 = 2;
    /// Chunks taken per pass when not feeding.
    pub const CHUNKS_PER_PASS: u64 = 8;

    pub const SND_LOCK: Callee = Callee { addr: 0x003b_58a0, f: ee::s13::SND_Lock };
    pub const SND_UNLOCK: Callee = Callee { addr: 0x003b_58d8, f: ee::s13::SND_Unlock };
    pub const GET_BUFFERED_BYTES: Callee = Callee {
        addr: 0x003c_6c98,
        f: ee::s13::SNDSTRM_GetBufferedBytes,
    };
    pub const STOP_SLOT: Callee = Callee { addr: 0x003c_6ce8, f: ee::s13::SNDSTRM_StopSlot };
    pub const START_PLAYBACK: Callee = Callee {
        addr: 0x003b_68b8,
        f: ee::s13::SNDSTRM_StartPlayback,
    };
    pub const IS_PREBUFFERING: Callee = Callee {
        addr: 0x003b_70f8,
        f: ee::s13::SNDSTRM_IsPrebuffering,
    };
    pub const GET_FREE_BLOCKS: Callee = Callee {
        addr: 0x003c_6c28,
        f: ee::s13::SNDSTRM_GetFreeBlocks,
    };
    pub const HANDLE_CHUNK: Callee = Callee {
        addr: 0x003b_7080,
        f: ee::s13::SNDSTRM_HandleChunk,
    };
    /// `StreamFile_GetChunk(file)` (cross-shard).
    pub const STREAM_FILE_GET_CHUNK: u32 = 0x003e_12e0;
}

/// `void SNDSTRM_Service(void)` (0x3b71d8): under the sound lock, for each
/// open stream: a playing stream whose buffer ran dry restarts (the queued
/// request at +0x84..+0xec moves down to +0x1c, +0x18 to +0x14; stop the
/// slot, start playback); then, unless it is still prebuffering, feed it
/// chunks from its stream file: as many as the slot has free blocks when
/// feeding, else up to 8, stopping at the first missing or unhandled one.
/// Frame 0x90 (s0-s7).
pub fn sndstrm_service(c: &mut Cpu) {
    use sndstrm_service::*;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x90);
        for (reg, off) in [
            (S3, 0x50),
            (S7, 0x10),
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0);
        c.g[S3][0] = 0;
        SND_LOCK.call(c, 0x003b_7208)?;
        c.g[S7][0] = 0x51_0000;
        c.g[V1][0] = c.lbu(SND_STATE + STREAM_COUNT);
        c.g[V0][0] = 0x51_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[S6][0] = STATE_PLAYING;
            c.g[S5][0] = sx(STREAMS);
            c.g[V0][0] = sx(r32(c, S3) << 2);
            // 0x3b7230: stream s3 (s4 = s3 + 1).
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                c.g[S0][0] = c.lw(r32(c, V0));
                c.g[S4][0] = sx(r32(c, S3).wrapping_add(1));
                service_stream(c)?;
                // 0x3b73c4: next stream.
                c.cycles += 5;
                c.g[S3][0] = c.g[S4][0];
                c.g[V0][0] = c.lbu(at(c, V1, STREAM_COUNT));
                let more = (c.g[S3][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = sx(r32(c, S3) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_7230);
                }
            }
        }
        c.cycles += 13;
        SND_UNLOCK.call(c, 0x003b_73e0)?;
        for (reg, off) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`sndstrm_service`] for the stream in s0 (s3 its index);
/// ends with v1 = the sound state, as every path of the original does.
fn service_stream(c: &mut Cpu) -> crate::call::Flow {
    use sndstrm_service::*;
    let state_addr = sx(r32(c, S7).wrapping_sub(0x5718));
    let stream = r32(c, S0);
    if stream == 0 {
        c.cycles += 1;
        c.g[V1][0] = state_addr;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(stream.wrapping_add(OPEN));
    c.g[V1][0] = state_addr;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = sx(c.lbu(stream.wrapping_add(STATE)) as u8 as i8 as i32 as u32);
    if c.g[V0][0] == c.g[S6][0] {
        // Playing: restart once the buffer ran dry.
        c.cycles += 5;
        c.g[A0][0] = c.lw(stream.wrapping_add(SLOT));
        GET_BUFFERED_BYTES.call(c, 0x003b_726c)?;
        let buffered = (c.g[V0][0] as i64) > 0;
        c.g[V1][0] = state_addr;
        if buffered {
            return Ok(());
        }
        restart_stream(c, stream)?;
    }

    // 0x3b7340
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    IS_PREBUFFERING.call(c, 0x003b_7348)?;
    let prebuffering = c.g[V0][0] != 0;
    c.g[V1][0] = state_addr;
    if prebuffering {
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = sx(c.lbu(stream.wrapping_add(STATE)) as u8 as i8 as i32 as u32);
    c.g[V1][0] = STATE_FEEDING;
    c.g[S1][0] = CHUNKS_PER_PASS;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 6;
        c.g[A0][0] = c.lw(stream.wrapping_add(SLOT));
        GET_FREE_BLOCKS.call(c, 0x003b_7374)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = state_addr;
        if (c.g[S1][0] as i64) <= 0 {
            return Ok(());
        }
        c.cycles += 2;
    } else {
        c.cycles += 2;
    }
    c.g[S2][0] = 0;
    c.cycles += 1;
    c.g[A0][0] = c.lw(stream.wrapping_add(FILE));
    // 0x3b7394: s1 chunks at most.
    loop {
        c.cycles += 4;
        c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
        call_ptr(c, STREAM_FILE_GET_CHUNK, 0x003b_739c)?;
        c.g[A1][0] = c.g[V0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            HANDLE_CHUNK.call(c, 0x003b_73ac)?;
            c.g[S2][0] = c.g[V0][0];
        }
        c.cycles += 2;
        c.g[V1][0] = state_addr;
        if c.g[S2][0] == 0 {
            return Ok(());
        }
        c.cycles += 2;
        if (c.g[S1][0] as i64) <= 0 {
            c.cycles += 1;
            c.g[V1][0] = state_addr;
            return Ok(());
        }
        c.g[A0][0] = c.lw(stream.wrapping_add(FILE));
        if event_due(c) {
            deliver_event_here(c, 0x003b_7394);
        }
    }
}

/// 0x3b7280: a playing stream ran dry: move the queued request (0x60
/// bytes at +0x84, then 8 at +0xe4) down to +0x1c and +0x18 to +0x14
/// (unaligned copies when either end is not 8-byte aligned), clear +0x9c,
/// stop the slot and start playback again.
fn restart_stream(c: &mut Cpu, stream: u32) -> crate::call::Flow {
    use sndstrm_service::*;
    c.cycles += 10;
    c.g[A0][0] = sx(stream.wrapping_add(0x1c));
    c.g[V1][0] = sx(stream.wrapping_add(0x84));
    c.g[V0][0] = c.g[V1][0] | c.g[A0][0];
    c.lwl(A1, stream.wrapping_add(0x1b));
    c.lwr(A1, stream.wrapping_add(0x18));
    c.swl(A1, stream.wrapping_add(0x17));
    c.swr(A1, stream.wrapping_add(0x14));
    let aligned = c.g[V0][0] & 7 == 0;
    c.g[V0][0] = sx(stream.wrapping_add(0xe4));
    if !aligned {
        // 0x3b729c
        loop {
            c.cycles += 19;
            let (src, dst) = (r32(c, V1), r32(c, A0));
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                let o = 8 * k as u32;
                c.ldl(reg, src.wrapping_add(o + 7));
                c.ldr(reg, src.wrapping_add(o));
            }
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                let o = 8 * k as u32;
                c.sdl(reg, dst.wrapping_add(o + 7));
                c.sdr(reg, dst.wrapping_add(o));
            }
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_729c);
            }
        }
    } else {
        // 0x3b72f0
        loop {
            c.cycles += 11;
            let (src, dst) = (r32(c, V1), r32(c, A0));
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                c.g[reg][0] = c.ld(src.wrapping_add(8 * k as u32));
            }
            for (k, reg) in [A1, A2, A3, T0].into_iter().enumerate() {
                c.sd(dst.wrapping_add(8 * k as u32), c.g[reg][0]);
            }
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003b_72f0);
            }
        }
    }
    // 0x3b731c
    c.cycles += 9;
    c.ldl(V0, at(c, V1, 7));
    c.ldr(V0, r32(c, V1));
    c.sdl(V0, at(c, A0, 7));
    c.sdr(V0, r32(c, A0));
    c.g[A0][0] = c.lw(stream.wrapping_add(SLOT));
    c.sw(stream.wrapping_add(0x9c), 0);
    STOP_SLOT.call(c, 0x003b_7338)?;
    c.g[A0][0] = c.g[S0][0];
    START_PLAYBACK.call(c, 0x003b_7340)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// GameModule_BeginBezierPass (0x22c1b0, src/main/game.cpp).

/// `this(a1)->vtable(vt_reg)[slot]` called with `a0 = a1 + delta`, the
/// method address in `fn_reg` (the compiled register choice).
fn vcall_a1(c: &mut Cpu, vt_reg: usize, fn_reg: usize, slot: u32, ret_to: u32) -> crate::call::Flow {
    let this = r32(c, A1);
    c.g[A0][0] = c.lh(at(c, vt_reg, slot));
    c.g[fn_reg][0] = c.lw(at(c, vt_reg, slot + 4));
    let method = r32(c, fn_reg);
    c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret_to)
}

/// Fields of the bezier pass context.
pub mod bezier_pass {
    /// gp+0x1158: the camera; gp-0x854: the renderer (`+0x10d8` its
    /// vtable, slot 0x118 the view matrix).
    pub const GP_CAMERA: u32 = 0x1158;
    pub const GP_RENDERER: u32 = 0x854;
    pub const RENDERER_VT: u32 = 0x10d8;
    pub const SLOT_VIEW_MATRIX: u32 = 0x118;
    /// Context: camera, eye position (view space), depth fit (k, c),
    /// near/far (1, 3).
    pub const CAMERA: u32 = 0x0;
    pub const EYE: u32 = 0x10;
    pub const DEPTH_K: u32 = 0x20;
    pub const DEPTH_C: u32 = 0x24;
    pub const NEAR: u32 = 0x28;
    pub const FAR: u32 = 0x2c;
    /// Camera (vtable at +4): near plane, far plane, begin pass.
    pub const SLOT_NEAR: u32 = 0x40;
    pub const SLOT_FAR: u32 = 0x48;
    pub const SLOT_BEGIN: u32 = 0x88;
}

/// `Ctx* GameModule_BeginBezierPass(Ctx* ctx)` (0x22c1b0): take the
/// camera, compute the eye position (-translation of the view matrix,
/// transformed by the view matrix, w = 1), set near/far = 1/3 and fit the
/// depth mapping `k = (near - far) / (2 f^2 - 2 n^2)`, `c = far - k 2 n^2`
/// from the camera's near (n) and far (f) planes, then begin the camera's
/// pass. Returns ctx. One 130-cycle block; frame 0x60 (s0, f20).
pub fn game_module_begin_bezier_pass(c: &mut Cpu) {
    use bezier_pass::*;
    guest(c, |c| {
        c.cycles += 130;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_CAMERA));
        frame.save(c, S0, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x50);
        let ctx = r32(c, S0);
        c.g[A1][0] = c.lw(gp_lit(c, GP_RENDERER));
        c.sw(ctx.wrapping_add(CAMERA), c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, A1, RENDERER_VT));
        vcall_a1(c, V1, V0, SLOT_VIEW_MATRIX, 0x0022_c1e4)?;
        // -translation (VU0 scale by -1).
        c.lq(3, at(c, V0, 0x30));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 0);
        let s = sp(c);
        c.sq(3, s.wrapping_add(0x10));
        c.mfc1(A0, 0);
        c.lqc2(4, s.wrapping_add(0x10));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s.wrapping_add(0x20));
        c.lq(2, s.wrapping_add(0x20));
        c.g[A1][0] = c.lw(gp_lit(c, GP_RENDERER));
        c.sq(2, ctx.wrapping_add(EYE));
        c.sq(2, s);
        c.g[V1][0] = c.lw(at(c, A1, RENDERER_VT));
        vcall_a1(c, V1, V0, SLOT_VIEW_MATRIX, 0x0022_c22c)?;
        // eye = M * eye (row-major dot products, the compiled order).
        let m = r32(c, V0);
        c.lwc1(1, ctx.wrapping_add(EYE));
        c.lwc1(5, m);
        c.lwc1(8, m.wrapping_add(0x4));
        c.lwc1(4, m.wrapping_add(0x10));
        c.mul_s(5, 1, 5);
        c.lwc1(7, m.wrapping_add(0x14));
        c.lwc1(3, m.wrapping_add(0x20));
        c.mul_s(4, 1, 4);
        c.lwc1(6, m.wrapping_add(0x34));
        c.lwc1(0, ctx.wrapping_add(EYE + 4));
        c.mul_s(3, 1, 3);
        c.lwc1(12, m.wrapping_add(0x24));
        c.lwc1(2, m.wrapping_add(0x30));
        c.mul_s(6, 0, 6);
        c.mul_s(8, 0, 8);
        c.lwc1(10, m.wrapping_add(0x8));
        c.mul_s(7, 0, 7);
        c.lwc1(11, m.wrapping_add(0x18));
        c.mul_s(0, 0, 12);
        c.lwc1(9, m.wrapping_add(0x38));
        c.mul_s(1, 1, 2);
        c.lwc1(15, m.wrapping_add(0x28));
        c.lwc1(2, ctx.wrapping_add(EYE + 8));
        c.add_s(5, 5, 8);
        c.add_s(4, 4, 7);
        c.lwc1(12, m.wrapping_add(0x3c));
        c.mul_s(9, 2, 9);
        c.lwc1(8, m.wrapping_add(0xc));
        c.mul_s(10, 2, 10);
        c.lwc1(7, m.wrapping_add(0x1c));
        c.mul_s(11, 2, 11);
        c.lwc1(14, m.wrapping_add(0x2c));
        c.add_s(3, 3, 0);
        lit(c, 13, 0x3f80_0000);
        c.lwc1(0, ctx.wrapping_add(EYE + 0xc));
        c.mul_s(2, 2, 15);
        c.add_s(1, 1, 6);
        c.swc1(13, ctx.wrapping_add(NEAR));
        c.mul_s(12, 0, 12);
        lit(c, 6, 0x4040_0000);
        c.mul_s(8, 0, 8);
        c.g[A1][0] = c.lw(ctx.wrapping_add(CAMERA));
        c.mul_s(7, 0, 7);
        c.swc1(6, ctx.wrapping_add(FAR));
        c.add_s(5, 5, 10);
        c.add_s(4, 4, 11);
        c.add_s(1, 1, 9);
        c.mul_s(0, 0, 14);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 12);
        c.add_s(5, 5, 8);
        c.add_s(4, 4, 7);
        c.add_s(3, 3, 0);
        let s = sp(c);
        c.swc1(1, s.wrapping_add(0xc));
        c.swc1(5, s);
        c.swc1(4, s.wrapping_add(0x4));
        c.swc1(3, s.wrapping_add(0x8));
        c.lq(2, s);
        c.sq(2, ctx.wrapping_add(EYE));
        c.swc1(13, ctx.wrapping_add(EYE + 0xc));
        // The camera's near and far planes.
        c.g[V0][0] = c.lw(at(c, A1, 4));
        vcall_a1(c, V0, V1, SLOT_NEAR, 0x0022_c338)?;
        c.g[A1][0] = c.lw(ctx.wrapping_add(CAMERA));
        c.mov_s(20, 0);
        c.g[V0][0] = c.lw(at(c, A1, 4));
        vcall_a1(c, V0, V1, SLOT_FAR, 0x0022_c354)?;
        c.mul_s(3, 20, 20);
        c.lwc1(2, ctx.wrapping_add(FAR));
        c.mul_s(0, 0, 0);
        c.lwc1(1, ctx.wrapping_add(NEAR));
        c.g[A1][0] = c.lw(ctx.wrapping_add(CAMERA));
        c.add_s(20, 3, 3);
        c.add_s(0, 0, 0);
        c.sub_s(1, 1, 2);
        c.sub_s(0, 0, 20);
        c.div_s(1, 1, 0);
        c.mul_s(0, 1, 20);
        c.swc1(1, ctx.wrapping_add(DEPTH_K));
        c.sub_s(2, 2, 0);
        c.swc1(2, ctx.wrapping_add(DEPTH_C));
        c.g[V0][0] = c.lw(at(c, A1, 4));
        vcall_a1(c, V0, V1, SLOT_BEGIN, 0x0022_c3a0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore_fpr(c, 20, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SkyBox_Render (0x353b10, src/object/modifierblock.cpp).

/// Globals, render-state fields and callees of the sky box draw.
pub mod skybox {
    /// gp+0x143c: sky drawing disabled; gp+0x2a90: the renderer.
    pub const GP_SKY_OFF: u32 = 0x143c;
    pub const GP_RENDERER: u32 = 0x2a90;
    /// Renderer: render-state stack top (0x14-byte records), vtable.
    pub const RS_TOP: u32 = 0xe84;
    pub const RENDERER_VT: u32 = 0x10d8;
    /// `DrawModel(renderer, model, 1.0, textures, matrix, 0, 0, 0x5420)`.
    pub const SLOT_DRAW_MODEL: u32 = 0x2f8;
    pub const RS_SIZE: u32 = 0x14;
    /// Record word 2: layer bits 5..9.
    pub const RS_LAYER_WORD: u32 = 0x8;
    pub const LAYER_KEEP: u64 = 0xffff_ffff_ffff_fc1f;
    /// The view matrix the sky is drawn with (eye replaced by the camera's).
    pub const VIEW_MATRIX: u32 = 0x004f_f1a0;
    /// `SkyBox`: the model (`+8` flags, `+0x80` its texture set at +0xc).
    pub const MODEL: u32 = 0x14;
    /// `World_GetCurrentViewIndex()`, `World_GetCameraEye(view)`.
    pub const GET_VIEW_INDEX: u32 = 0x002d_1c58;
    pub const GET_CAMERA_EYE: u32 = 0x002d_1c20;
}

/// `void SkyBox_Render(SkyBox* sky)` (0x353b10): unless disabled or
/// without a model, draw the sky model with the view matrix moved to the
/// camera eye, under a pushed render state (layer 0, z test on, blend 2,
/// z write off, alpha ref 0x14 in the state's mode word) restored with
/// the old layer afterwards; the model's flag bit 1 is cleared and bit 2
/// set. Frame 0xa0 (matrix at +0, s0-s3).
pub fn skybox_render(c: &mut Cpu) {
    use skybox::*;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_add(GP_SKY_OFF));
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save_ra(c, 0x50);
        let sky = r32(c, S1);
        'out: {
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(sky.wrapping_add(MODEL));
            c.g[S0][0] = c.lw(r32(c, GP).wrapping_add(GP_RENDERER));
            if c.g[V0][0] == 0 {
                break 'out;
            }
            let rend = r32(c, S0);
            c.cycles += 74;
            // The view matrix, eye replaced by the camera's.
            c.g[V0][0] = sx(VIEW_MATRIX);
            let s = sp(c);
            for k in 0..4u8 {
                c.lqc2(1 + k as usize, VIEW_MATRIX + 0x10 * k as u32);
            }
            for k in 0..4u8 {
                c.sqc2(1 + k as usize, s + 0x10 * k as u32);
            }
            c.g[S3][0] = LAYER_KEEP;
            call_ptr(c, GET_VIEW_INDEX, 0x0035_3b70)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, GET_CAMERA_EYE, 0x0035_3b78)?;
            let s = sp(c);
            c.lq(3, r32(c, V0));
            c.g[A1][0] = 0xffff_ffff_ffbf_ffff;
            c.g[T2][0] = 0x40_0000;
            c.sq(3, s.wrapping_add(0x30));
            c.g[A2][0] = 0xffff_ffff_fe7f_ffff;
            c.sq(3, s.wrapping_add(0x40));
            c.g[T3][0] = 0x100_0000;
            c.g[A3][0] = 0xffff_ffff_ffcf_ffff;
            // Push: copy the top record over the next one.
            let top = c.lw(rend.wrapping_add(RS_TOP));
            c.g[V0][0] = top;
            let top = top as u32;
            c.g[T0][0] = 0xffff_ffff_fff0_0000;
            c.g[T1][0] = 0x1_0000;
            c.ldl(V1, top.wrapping_add(0x7));
            c.ldr(V1, top);
            c.ldl(A0, top.wrapping_add(0xf));
            c.ldr(A0, top.wrapping_add(0x8));
            c.g[T4][0] = c.lw(top.wrapping_add(0x10));
            c.sdl(V1, top.wrapping_add(0x1b));
            c.sdr(V1, top.wrapping_add(0x14));
            c.sdl(A0, top.wrapping_add(0x23));
            c.sdr(A0, top.wrapping_add(0x1c));
            c.sw(top.wrapping_add(0x24), c.g[T4][0]);
            c.g[T0][0] |= 0xfff;
            c.g[T1][0] |= 0x4000;
            c.g[A0][0] = c.lw(rend.wrapping_add(RS_TOP));
            c.g[V1][0] = sx(at(c, A0, RS_SIZE));
            c.sw(rend.wrapping_add(RS_TOP), c.g[V1][0]);
            // Layer 0, remembering the old one (s2).
            c.g[V0][0] = c.lw(at(c, A0, RS_SIZE + RS_LAYER_WORD));
            c.g[V1][0] = c.g[V0][0] & c.g[S3][0];
            c.sw(at(c, A0, RS_SIZE + RS_LAYER_WORD), c.g[V1][0]);
            c.g[V0][0] &= 0x3e0;
            c.g[S2][0] = sx(r32(c, V0) >> 5);
            // Word 0 |= 0xc; word 1: z test on, blend 2, z write off,
            // alpha ref 0x14.
            c.g[V1][0] = c.lw(rend.wrapping_add(RS_TOP));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] |= 0xc;
            c.sw(r32(c, V1), c.g[V0][0]);
            for (base, keep, set) in [(A0, A1, Some(T2)), (A1, A2, Some(T3)), (V1, A3, None), (A0, T0, Some(T1))] {
                c.g[base][0] = c.lw(rend.wrapping_add(RS_TOP));
                c.g[V0][0] = c.lw(at(c, base, 4));
                c.g[V0][0] &= c.g[keep][0];
                if let Some(set) = set {
                    c.g[V0][0] |= c.g[set][0];
                }
                c.sw(at(c, base, 4), c.g[V0][0]);
            }
            c.g[A0][0] = c.lw(sky.wrapping_add(MODEL));
            c.g[A3][0] = c.g[SP][0];
            if c.g[A0][0] != 0 {
                c.cycles += 19;
                c.g[V1][0] = c.lw(at(c, A0, 8));
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                lit(c, 12, 0x3f80_0000);
                c.g[V1][0] &= c.g[V0][0];
                c.g[V1][0] |= 0x4;
                c.g[T0][0] = 0;
                c.sw(at(c, A0, 8), c.g[V1][0]);
                c.g[T1][0] = 0;
                c.g[T2][0] = 0x5420;
                c.g[A1][0] = c.lw(sky.wrapping_add(MODEL));
                c.g[V0][0] = c.lw(rend.wrapping_add(RENDERER_VT));
                c.g[T3][0] = c.lw(at(c, A1, 0x80));
                c.g[A0][0] = c.lh(at(c, V0, SLOT_DRAW_MODEL));
                c.g[V1][0] = c.lw(at(c, V0, SLOT_DRAW_MODEL + 4));
                c.g[A2][0] = c.lw(at(c, T3, 0xc));
                let method = r32(c, V1);
                c.g[A0][0] = sx(rend.wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x0035_3cb4)?;
            }
            // Pop, restoring the old layer.
            c.cycles += 8;
            c.g[A0][0] = c.lw(rend.wrapping_add(RS_TOP));
            c.g[A1][0] = sx(r32(c, S2) << 5);
            c.g[V1][0] = sx(r32(c, A0).wrapping_sub(RS_SIZE));
            c.sw(rend.wrapping_add(RS_TOP), c.g[V1][0]);
            let word = r32(c, A0).wrapping_sub(RS_SIZE - RS_LAYER_WORD);
            c.g[V0][0] = c.lw(word);
            c.g[V0][0] &= c.g[S3][0];
            c.g[V0][0] |= c.g[A1][0];
            c.sw(word, c.g[V0][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SNDSTRM_ReleaseBytes (0x3c7010, EA sound library: stream slots).

/// Stream slot table and release-queue fields.
pub mod sndstrm_release {
    /// Release queue: count at 0x5157b8, 8-byte entries {0, slot, bytes}
    /// from 0x5157c0; slot buffers `Buffer*[]` at 0x5157b8 + 0x304.
    pub const QUEUE: u32 = 0x0051_57b8;
    pub const BUFFERS: u32 = QUEUE + 0x304;
    /// Buffer: the reader id (signed byte), bytes held, release
    /// notification wanted.
    pub const READER: u32 = 0x42;
    pub const HELD: u32 = 0x4c;
    pub const NOTIFY: u32 = 0x58;
}

/// `void SNDSTRM_ReleaseBytes(int slot, int reader, int bytes)`
/// (0x3c7010): when `reader` owns slot `slot`'s buffer, take `bytes` off
/// the bytes it holds and, if the buffer wants notifications, queue a
/// release entry {0, slot, bytes}. A leaf.
pub fn sndstrm_release_bytes(c: &mut Cpu) {
    use sndstrm_release::*;
    c.cycles += 10;
    c.g[T0][0] = 0x51_0000;
    c.g[V1][0] = sx(r32(c, A0) << 2);
    c.g[T1][0] = sx(QUEUE);
    c.g[V1][0] = sx(QUEUE.wrapping_add(r32(c, V1)));
    let buf = c.lw(at(c, V1, 0x304));
    c.g[A3][0] = buf;
    let buf = buf as u32;
    c.g[V0][0] = sx(c.lbu(buf.wrapping_add(READER)) as u8 as i8 as i32 as u32);
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 6;
        c.g[V0][0] = c.lw(buf.wrapping_add(HELD));
        c.g[V1][0] = c.lw(buf.wrapping_add(NOTIFY));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A2)));
        c.sw(buf.wrapping_add(HELD), c.g[V0][0]);
        let notify = c.g[V1][0] != 0;
        c.g[V0][0] = c.lw(QUEUE);
        if notify {
            c.cycles += 9;
            c.g[V1][0] = sx(QUEUE + 4);
            let entry = (r32(c, V0) << 3).wrapping_add(QUEUE + 4);
            c.g[V0][0] = sx(entry);
            c.sw(entry.wrapping_add(4), c.g[A2][0]);
            c.sh(entry.wrapping_add(2), c.g[A0][0]);
            c.sh(entry, 0);
            c.g[V1][0] = c.lw(QUEUE);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
            c.sw(QUEUE, c.g[V1][0]);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// ComputerActorRider_ThinkPrewind (0x10aa70, src/seg/seg_1218.cpp).

/// `VF4.x = |VF3|` (the compiled VU0 length: dot product, sqrt into Q).
fn vu0_length_vf3(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
}

/// Fields of the CPU rider used by the prewind think.
pub mod prewind {
    use super::{ee, Callee};
    /// `ComputerActorRider`: its rider; the control block (+0xdf0: speed
    /// threshold, +0x48/+0x4c analog values); the scripted-start flag and
    /// its two points.
    pub const RIDER: u32 = 0x18;
    pub const CONTROL: u32 = 0xdf0;
    pub const SCRIPTED: u32 = 0xe1c;
    pub const SCRIPT_FROM: u32 = 0xe50;
    pub const SCRIPT_TO: u32 = 0xe60;
    /// `Rider`: position, look-ahead point, velocity, track distance and
    /// lateral offset, the track.
    pub const POS: u32 = 0x110;
    pub const VELOCITY: u32 = 0x1e0;
    pub const AHEAD: u32 = 0x490;
    pub const TRACK_DIST: u32 = 0x4c0;
    pub const TRACK_SIDE: u32 = 0x4c4;
    pub const TRACK: u32 = 0xab8;
    /// Track: vtable at +0x34, `Query(out, max, from, to)` at slot 8.
    pub const TRACK_VT: u32 = 0x34;
    pub const SLOT_QUERY: u32 = 0x8;
    /// A track event of this type ends the prewind.
    pub const EVENT_PREWIND_END: u64 = 0x10;
    /// gp-0x7e30: subtracted from the speed threshold.
    pub const GP_SPEED_MARGIN: u32 = 0x7e30;
    /// `Path_GetPointAtDistance(out, track, in_range, t)` (cross-shard).
    pub const PATH_GET_POINT: u32 = 0x0026_ab20;
    pub const COMPUTE_STEER: Callee = Callee {
        addr: 0x0010_0348,
        f: ee::s00::ComputerActorRider_ComputeSteer,
    };
}

/// `void ComputerActorRider_ThinkPrewind(ComputerActorRider* self, Pad*
/// pad)` (0x10aa70): the CPU rider's input before the start. Queries the
/// track events in [dist, dist + 50]; presses bit 14 while slower than the
/// threshold; on a scripted start presses bit 13 once the rider is within
/// 50 of the goal distance and stops there. Otherwise a prewind-end event
/// (type 0x10) presses bit 13 and steers (pad word 1, 6 bits, from
/// `ComputeSteer` towards the event's point relative to half the
/// look-ahead), and the two analog values go to pad bits 15..20 and 21..26
/// (times 31). Frame 0x160 (the event buffer at +0, s0-s3, f20).
pub fn computer_actor_rider_think_prewind(c: &mut Cpu) {
    use prewind::*;
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x160);
        c.g[A2][0] = 0xc;
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S0, 0x140);
        frame.save(c, S3, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x150);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x100);
        let me = r32(c, S2);
        let pad = r32(c, S1);
        lit(c, 20, 0x4248_0000); // 50.0
        c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
        c.g[S0][0] = c.lw(at(c, V0, TRACK));
        c.lwc1(13, at(c, V0, TRACK_SIDE));
        c.g[V1][0] = c.lw(at(c, S0, TRACK_VT));
        c.lwc1(12, at(c, V0, TRACK_DIST));
        c.add_s(13, 13, 20);
        vcall_a0_s0(c, SLOT_QUERY, 0x0010_aacc)?;
        // Slower than the threshold: bit 14.
        c.lwc1(1, me.wrapping_add(CONTROL));
        c.g[A3][0] = c.g[V0][0];
        c.lwc1(2, gp_lit(c, GP_SPEED_MARGIN));
        c.g[T0][0] = u64::MAX;
        c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
        c.sub_s(1, 1, 2);
        c.lqc2(3, at(c, V0, VELOCITY));
        vu0_length_vf3(c);
        c.cfc2(V1, 22);
        c.mtc1(V1, 0);
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 4;
            c.g[V0][0] = c.lw(pad) | 0x4000;
            c.sw(pad, c.g[V0][0]);
        }
        c.g[V0][0] = c.lw(me.wrapping_add(SCRIPTED));
        c.cycles += 2;
        c.g[A2][0] = 0;
        if c.g[V0][0] != 0 {
            // 0x10ab34: |pos - from| < |to - from| - 50: bit 13.
            c.cycles += 38;
            let s = sp(c);
            c.lqc2(3, me.wrapping_add(SCRIPT_TO));
            c.lqc2(4, me.wrapping_add(SCRIPT_FROM));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xd0));
            c.lq(2, s.wrapping_add(0xd0));
            c.g[V1][0] = c.lw(me.wrapping_add(RIDER));
            c.sq(2, s.wrapping_add(0xc0));
            c.lqc2(3, at(c, V1, POS));
            c.lqc2(4, me.wrapping_add(SCRIPT_FROM));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, s.wrapping_add(0xe0));
            c.lq(2, s.wrapping_add(0xe0));
            c.sq(2, s.wrapping_add(0xd0));
            c.lqc2(3, s.wrapping_add(0xc0));
            vu0_length_vf3(c);
            c.cfc2(V1, 22);
            c.mtc1(V1, 0);
            c.lqc2(3, s.wrapping_add(0xd0));
            vu0_length_vf3(c);
            c.cfc2(V0, 22);
            c.mtc1(V0, 1);
            c.sub_s(0, 0, 20);
            c.c_lt_s(1, 0);
            let close = c.fcc();
            frame.restore(c, S0, 0x140);
            if close {
                c.cycles += 4;
                c.g[V0][0] = c.lw(pad) | 0x2000;
                c.sw(pad, c.g[V0][0]);
            }
        } else {
            // 0x10abdc: look for a prewind-end event (t0 = its index).
            c.cycles += 2;
            c.g[S3][0] = sx(me.wrapping_add(CONTROL));
            if (c.g[A3][0] as i64) > 0 {
                c.cycles += 4;
                c.g[V1][0] = c.lw(sp(c));
                c.g[V0][0] = EVENT_PREWIND_END;
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                let mut found = c.g[V1][0] == EVENT_PREWIND_END;
                if found {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(pad);
                    c.g[T0][0] = 0;
                } else {
                    // 0x10ac00
                    loop {
                        c.cycles += 3;
                        let more = (c.g[A2][0] as i64) < (c.g[A3][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[A0][0] = sx(r32(c, A2) << 4);
                        if !more {
                            break;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = EVENT_PREWIND_END;
                        c.g[V0][0] = sx(sp(c).wrapping_add(r32(c, A0)));
                        c.g[V1][0] = c.lw(r32(c, V0));
                        if c.g[V1][0] == c.g[A1][0] {
                            c.cycles += 2;
                            c.g[V0][0] = c.lw(pad);
                            c.g[T0][0] = c.g[A2][0];
                            found = true;
                            break;
                        }
                        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
                        if event_due(c) {
                            deliver_event_here(c, 0x0010_ac00);
                        }
                    }
                }
                if found {
                    c.cycles += 2;
                    c.g[V0][0] |= 0x2000;
                    c.sw(pad, c.g[V0][0]);
                }
            }
            // 0x10ac30
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            c.g[A0][0] = sx(r32(c, T0) << 4);
            if c.g[T0][0] != u64::MAX {
                // Steer towards the event point (relative to half the
                // look-ahead).
                c.cycles += 45;
                let s = sp(c);
                c.g[V0][0] = c.lw(me.wrapping_add(RIDER));
                c.lqc2(3, at(c, V0, AHEAD));
                c.lqc2(4, at(c, V0, POS));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xd0));
                lit(c, 0, 0x3f00_0000);
                c.lq(2, s.wrapping_add(0xd0));
                c.g[V1][0] = sx(s.wrapping_add(0xc).wrapping_add(r32(c, A0)));
                c.g[A1][0] = c.g[S0][0];
                c.sq(2, s.wrapping_add(0xc0));
                c.g[S0][0] = sx(s.wrapping_add(0xe0));
                c.mfc1(V0, 0);
                c.lqc2(4, s.wrapping_add(0xc0));
                c.qmtc2(V0, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, s.wrapping_add(0xe0));
                c.g[A0][0] = sx(s.wrapping_add(0xd0));
                c.lq(2, s.wrapping_add(0xe0));
                c.g[A2][0] = 0;
                c.sq(2, s.wrapping_add(0xc0));
                c.sq(2, s.wrapping_add(0xd0));
                c.lwc1(12, r32(c, V1));
                call_ptr(c, PATH_GET_POINT, 0x0010_aca0)?;
                let s = sp(c);
                c.lqc2(3, s.wrapping_add(0xd0));
                c.lqc2(4, s.wrapping_add(0xc0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, s.wrapping_add(0xf0));
                c.g[A0][0] = c.g[S2][0];
                c.lq(2, s.wrapping_add(0xf0));
                c.g[A1][0] = c.g[S0][0];
                c.sq(2, s.wrapping_add(0xe0));
                COMPUTE_STEER.call(c, 0x0010_acc4)?;
                lit(c, 1, 0x41f8_0000); // 31.0
                c.g[A0][0] = 0xffff_ffff_ffff_ffc0;
                c.g[V1][0] = c.lw(pad.wrapping_add(4));
                c.mul_s(0, 0, 1);
                c.g[V1][0] &= c.g[A0][0];
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] &= 0x3f;
                c.g[V1][0] |= c.g[V0][0];
                c.sw(pad.wrapping_add(4), c.g[V1][0]);
            }
            // 0x10acf0: the analog values (6 bits each, times 31).
            c.cycles += 26;
            let ctl = r32(c, S3);
            c.lwc1(0, ctl.wrapping_add(0x48));
            c.g[A0][0] = 0xffff_ffff_ffe0_7fff;
            lit(c, 1, 0x41f8_0000);
            c.g[V1][0] = c.lw(pad);
            c.g[A1][0] = 0xffff_ffff_f81f_ffff;
            c.mul_s(0, 0, 1);
            c.g[V1][0] &= c.g[A0][0];
            c.cvt_w_s(2, 0);
            c.mfc1(V0, 2);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 15);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(pad, c.g[V1][0]);
            c.g[V1][0] &= c.g[A1][0];
            c.lwc1(0, ctl.wrapping_add(0x4c));
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(V0, 1);
            c.g[V0][0] &= 0x3f;
            c.g[V0][0] = sx(r32(c, V0) << 21);
            c.g[V1][0] |= c.g[V0][0];
            c.sw(pad, c.g[V1][0]);
            frame.restore(c, S0, 0x140);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore_ra(c, 0x100);
        frame.restore_fpr(c, 20, 0x150);
        frame.close(c);
        Ok(())
    })
}

/// `s0->vtable(v1)[slot]` with `a0 = s0 + delta`, the method in v0.
fn vcall_a0_s0(c: &mut Cpu, slot: u32, ret_to: u32) -> crate::call::Flow {
    c.g[A0][0] = c.lh(at(c, V1, slot));
    c.g[V0][0] = c.lw(at(c, V1, slot + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret_to)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, schedule, Events};
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, fill, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    // ------------------------------------------------------ AI_UpdateRiderRelations

    /// The back end's tables the rivalry lookup reads (a copy of the `ai`
    /// tests' fixture): player slots, the player table, rider records
    /// (characters mostly 0..9, some profiles -1), the rivalry table, the
    /// track, and the BE singletons.
    fn be_tables(c: &mut Cpu, r: &mut Rng) {
        use crate::ai::{player_table, RIVALRY};
        for p in 0..8 {
            c.bus.write32(0x0053_05b0 + 4 * p, r.next_u32() % 8);
        }
        fill(c, r, player_table::BASE, 8 * 0x1c / 4);
        let all_cpu = r.next_u32() % 5 == 0;
        for s in 0..8 {
            let v = if all_cpu || r.next_u32() % 2 == 0 {
                r.next_u32() | 0x8000_0000
            } else {
                r.next_u32() % 4
            };
            c.bus.write32(player_table::BASE + s * 0x1c + 0xc, v);
        }
        fill(c, r, 0x0053_5b20, 16 * 0x1c / 4);
        for s in 0..16 {
            let rec = 0x0053_5b20 + s * 0x1c;
            if r.next_u32() % 2 == 0 {
                c.bus.write32(rec + 0xc, u32::MAX);
            }
            let ch = if r.next_u32() % 8 == 0 {
                r.next_u32() as u8
            } else {
                (r.next_u32() % 10) as u8
            };
            c.bus.write8(rec + 0x11, ch);
        }
        for p in 0..3u32 {
            for b in 0..10u32 {
                for a in 0..10u32 {
                    c.bus.write8(
                        RIVALRY + p * 0x9b50 + b * 0xf88 + a * 3,
                        (r.next_u32() % 5) as u8,
                    );
                }
            }
        }
        c.bus.write32(0x0053_5c08, r.next_u32() % 40);
        fill(c, r, 0x0043_d950, 40 * 0x64 / 4);
        for t in 0..40 {
            c.bus.write32(0x0043_d950 + t * 0x64 + 0x54, r.next_u32() % 4);
        }
        // NewRace, NewPlayer, Aggression, Char singletons.
        for g in [0x1f3c, 0x1f30, 0x1ea4, 0x1f2c] {
            c.bus.write32(GAME_GP - g, DATA + 0xe_0000 + g * 0x10);
        }
    }

    /// The relation pass over up to six riders: the frame counter is a
    /// multiple of 6 for most seeds; rider count minus skipped is 0..6;
    /// riders track some others; getPos is a fake whose first eight
    /// results are positions with equal x / zero y often enough to reach
    /// every atan2 case (dx = 0 with dy = 0, > 0, < 0; dx < 0 with dy
    /// either sign); the rivalry levels come from the real lookup; the
    /// rivals pass runs for some seeds. Fakes may unwind; events are
    /// delivered at the loop heads.
    #[test]
    fn ai_update_rider_relations_matches() {
        use rider_relations::*;
        const AI: u32 = 0x00a0_0000;
        const VT: u32 = 0x00a0_1000;
        const POS: u32 = 0x00a0_2000;
        const RIDER0: u32 = 0x00a1_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x2341, 8);
            be_tables(c, &mut r);
            for (off, v) in [
                (LIT_MIN_DIST, -50000.0f32),
                (LIT_MAX_DIST, 50000.0),
                (LIT_PI, std::f32::consts::PI),
                (LIT_NEG_HALF_PI, -std::f32::consts::FRAC_PI_2),
                (LIT_HALF_PI, std::f32::consts::FRAC_PI_2),
                (LIT_INV_TWO_PI, 0.159_154_94),
                (LIT_TWO_PI, std::f32::consts::TAU),
            ] {
                c.bus.write32(GAME_GP - off, v.to_bits());
            }
            // Positions the unforced fakes return (DATA + 0..0x3fff0).
            for k in 0..0x4000u32 {
                let a = DATA + 0x10 * k;
                c.bus.write32(a, r.f32(500.0));
                c.bus.write32(a + 4, r.f32(500.0));
            }
            // The first eight getPos results.
            for n in 0..8u32 {
                let x = [0.0f32, 1.0, -2.0][(r.next_u32() % 3) as usize];
                let y = [0.0f32, 3.0, -1.5][(r.next_u32() % 3) as usize];
                put_vec(c, POS + 0x10 * n, [x.to_bits(), y.to_bits(), r.f32(9.0), 0]);
                c.bus.write32(LOG_RESULTS + 4 * n, POS + 0x10 * n);
            }
            c.bus.write16(VT + GET_POS_SLOT, (r.next_u32() % 8 * 4) as u16);
            c.bus.write32(VT + GET_POS_SLOT + 4, FAKE);
            let frame = 6 * (r.next_u32() % 100) + if seed % 7 == 3 { 1 + r.next_u32() % 5 } else { 0 };
            c.bus.write32(AI + FRAME, frame);
            c.bus.write32(AI + 0x74, 0); // race positions: nothing to do
            c.bus.write32(AI + COUNT, r.next_u32() % 7);
            c.bus.write32(AI + SKIP, if r.next_u32() % 4 == 0 { 1 } else { 0 });
            c.bus.write32(AI + 0x80, (r.next_u32() % 3 == 0) as u32);
            for i in 0..6 {
                let rider = RIDER0 + 0x1000 * i;
                c.bus.write32(AI + RIDERS + 4 * i, rider);
                fill(c, &mut r, rider, 0x100 / 4);
                for j in 0..6 {
                    let tracked = if r.next_u32() % 4 == 0 { 0 } else { 1 + r.next_u32() % 3 };
                    c.bus.write32(rider + j * INFO_SIZE + INFO_ACTIVE, tracked);
                }
                c.bus.write32(rider + POS_OBJ, VT);
            }
            c.g[A0][0] = AI as u64;
            schedule(c, &mut r, seed, 0x0010_f560)
        };
        check_events(ee::s00::f_0010f560__generated, ai_update_rider_relations, 300, &setup);
    }

    // ------------------------------------------------------------ HUD_DrawTime

    /// A positive float in [lo, hi) as bits.
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        (lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)).to_bits()
    }

    fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// HUD_DrawTime: the font renderer (gp+0x2a90) whose DrawText is a
    /// fake (may unwind), a font with random sizes and shadow offsets
    /// (often zero), glyph tables, the HUD's widths and separator ":";
    /// times whose groups start with '1' (one or two) or not; flags 0..7
    /// (fraction, '+' / '-' sign); real HUD_AlignElemPos, Font_DrawText,
    /// sprintf and the varargs draw.
    #[test]
    fn hud_draw_time_matches() {
        use crate::difftest::fakes::covered;
        use hud_time::*;
        const HUD: u32 = DATA + 0x1_0000;
        const FONT_AT: u32 = DATA + 0x1_1000;
        const VT: u32 = DATA + 0x1_2000;
        const ARGS: u32 = DATA + 0x1_3000;
        covered(ee::s04::f_001f1840__generated, hud_draw_time, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x1f_1840, 6);
            c.pc = 0x001f_1840;
            for off in (0..0x100).step_by(8) {
                c.bus.write16(VT + off, ((r.next_u32() % 16) * 4) as u16);
                c.bus.write32(VT + off + 4, FAKE + 2 * off);
            }
            let rend = DATA + 0x1_8000;
            c.bus.write32(GAME_GP + 0x2a90, rend);
            c.bus.write32(rend + 0x10d8, VT - 0x280 + 8 * (r.next_u32() % 8));
            let state = DATA + 0x1_c000 + 4 * (r.next_u32() % 16);
            c.bus.write32(rend + 0xe84, state);
            fill(c, &mut r, state & !0xf, 0x20);
            // The font.
            fill(c, &mut r, FONT_AT, 0x20);
            for i in 0..8 {
                c.bus.write32(FONT_AT + 0x18 + 4 * i, uf(&mut r, 0.5, 30.0));
            }
            for i in 0..2 {
                let v = if r.next_u32() % 2 == 0 { 0 } else { r.f32(3.0) };
                c.bus.write32(FONT_AT + 0x28 + 4 * i, v);
            }
            c.bus.write32(FONT_AT + FONT_LINES, 1 + r.next_u32() % 3);
            let codes = DATA + 0x3000;
            let n = 1 + r.next_u32() % 40;
            let mut code = 0;
            for i in 0..n {
                code += 1 + r.next_u32() % 6;
                c.bus.write16(codes + 2 * i, code as u16);
            }
            fill(c, &mut r, DATA + 0x4000, 0x400);
            c.bus.write32(FONT_AT, n);
            c.bus.write32(FONT_AT + 4, codes);
            c.bus.write32(FONT_AT + 8, DATA + 0x4000 + 12 * (r.next_u32() % 8));
            c.bus.write32(FONT_AT + 0xc, r.next_u32() % 64);
            c.bus.write32(FONT_AT + 0x10, 64 + r.next_u32() % 128);
            // The HUD.
            fill(c, &mut r, HUD + 0x160, 0x8);
            for off in [SEP_W, DIGIT_W, SIGN_ADVANCE, ONE_W] {
                c.bus.write32(HUD + off, uf(&mut r, 1.0, 24.0));
            }
            c.bus.write32(HUD + FONT, FONT_AT);
            c.bus.write32(HUD + SEPARATOR, ARGS + 0x100);
            put_cstr(c, ARGS + 0x100, b":");
            put_cstr(c, STR_PLUS, b"+");
            put_cstr(c, STR_MINUS, b"-");
            put_cstr(c, FMT_TWO_DIGITS, b"%02d");
            put_cstr(c, FMT_FRACTION, b".%02d");
            // pos, scale, colour; the stack arguments.
            for k in 0..2 {
                c.bus.write32(ARGS + 4 * k, uf(&mut r, 0.0, 600.0));
                c.bus.write32(ARGS + 0x10 + 4 * k, uf(&mut r, 0.25, 3.0));
            }
            let colour = ARGS + 0x20 + r.next_u32() % 8;
            for k in 0..16 {
                c.bus.write8(colour + k, r.next_u32() as u8);
            }
            c.bus.write32(crate::difftest::SP, colour);
            for k in 1..4 {
                c.bus.write32(crate::difftest::SP + 8 * k, r.next_u32() % 4);
            }
            let group = |r: &mut Rng| -> u64 {
                let v = match r.next_u32() % 5 {
                    0 => 11,
                    1 => 10 + r.next_u32() % 10,
                    2 => 1 + 10 * (r.next_u32() % 10),
                    3 => r.next_u32() % 100,
                    _ => r.next_u32() % 10,
                };
                v as u64
            };
            c.g[A0][0] = HUD as u64;
            c.g[A1][0] = group(&mut r);
            c.g[A2][0] = group(&mut r);
            c.g[A3][0] = group(&mut r);
            c.g[T0][0] = group(&mut r);
            c.g[T1][0] = (r.next_u32() % 8) as u64;
            c.g[T2][0] = ARGS as u64;
            c.g[T3][0] = (ARGS + 0x10) as u64;
        });
    }

    // ---------------------------------------------------------- __ieee754_logf

    /// __ieee754_logf on every input class: +-0 (the -huge constant written
    /// at 0x499224), negatives, subnormals, exponent 255, exactly 1 and
    /// powers of two (f == 0, k == 0 or not), mantissas within 2^-20 of 1
    /// (the tiny polynomial), mantissas just above/below the series' split
    /// (k == 0 or not), and random positives.
    #[test]
    fn ieee754_logf_matches() {
        check(ee::s15::f_0040e550__generated, ieee754_logf, 400, &|c, seed| {
            let mut r = Rng(0x40_e550 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            c.pc = 0x0040_e550;
            c.bus.write32(0x0049_9224, 0xff7f_ffff);
            let exp = |r: &mut Rng| (1 + r.next_u32() % 253) << 23;
            let x = match seed % 12 {
                0 => (r.next_u32() & 1) << 31,
                1 => 0x8000_0000 | r.next_u32(),
                2 => r.next_u32() & 0x007f_ffff,
                3 => 0x7f80_0000 | (r.next_u32() & 0x807f_ffff),
                4 => 0x3f80_0000,
                5 => exp(&mut r),
                6 => 0x3f80_0000 + r.next_u32() % 16,
                7 => exp(&mut r) | (r.next_u32() % 16),
                8 => exp(&mut r) | (0x7f_fff0 + r.next_u32() % 16),
                9 => (0x7e_u32 << 23) | (0x7f_fff0 + r.next_u32() % 16),
                10 => 0x3f80_0000 + (r.next_u32() % 0x8_0000) | ((r.next_u32() & 1) << 22),
                _ => r.next_u32() & 0x7fff_ffff,
            };
            c.f[12] = x;
        });
    }

    // ---------------------------------------------------------- SnowFall_Update

    /// SnowFall_Update: zero to three views; the weather from the override
    /// block or from the world painter's getters (their quick path, or the
    /// slow path through a fake method); each view's splash manager (amount
    /// below or above the threshold); twelve box slots, some empty, the
    /// others real snow boxes (modes 0, 1, other; drift timer not expiring)
    /// of a view. Fakes may unwind; events are delivered at both loop heads.
    #[test]
    fn snowfall_update_matches() {
        use crate::game::visualfx::boardwakefx::snow_update as snow;
        use snowfall::*;
        const SELF: u32 = 0x00a0_0000;
        const APP: u32 = 0x00a0_1000;
        const GM: u32 = 0x00a0_2000;
        const WX: u32 = 0x00a0_3000;
        const RATE_OBJ: u32 = 0x00a0_3800;
        const SPLASH: u32 = 0x00a1_0000;
        const BOX0: u32 = 0x00a2_0000;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x2e_5da0, 8);
            let gp = |o: u32| GAME_GP.wrapping_add(o);
            let f = |x: f32| x.to_bits();
            c.bus.write32(GAME_GP - GP_APP, APP);
            c.bus.write32(APP + 0x84, GM);
            c.bus.write32(GM + 0x84, WX);
            let views = [0, 1, 2, 2, 3][(seed % 5) as usize];
            c.bus.write32(WX + 0x10, views);
            // Weather: override block or the painter (quick path or not).
            c.bus.write32(gp(GP_WEATHER_OVERRIDE), (seed % 3 == 0) as u32);
            for off in GP_OVERRIDE_F {
                c.bus.write32(gp(off), r.f32(2.0));
            }
            for k in 0..8 {
                c.bus.write32(OVERRIDE_C0 + 4 * k, r.f32(200.0));
            }
            c.bus.write32(gp(0xa70), (r.next_u32() % 4 != 0) as u32);
            // Splash managers: amount threshold and constants.
            c.bus.write32(gp(0x1190), r.f32(1.0));
            c.bus.write32(gp(0x1194), r.f32(2.0));
            c.bus.write32(GAME_GP - 0x38f4, r.f32(2.0));
            for i in 0..3 {
                let sm = SPLASH + 0x2000 * i;
                c.bus.write32(GM + 0x68 + 4 * i, sm);
                c.bus.write32(sm + 0x1020, r.f32(5.0));
                c.bus.write32(sm + 0x1024, r.f32(5.0));
            }
            // Snow boxes.
            c.bus.write32(gp(snow::GP_RATE_OBJ as u32), RATE_OBJ);
            c.bus.write32(RATE_OBJ + snow::RATE, 60);
            for k in 0..4 {
                c.bus.write32(snow::WIND + 4 * k, r.f32(1.0));
            }
            for i in 0..12 {
                let b = BOX0 + 0x100 * i;
                let used = r.next_u32() % 3 != 0;
                c.bus.write32(SELF + BOXES + 4 * i, if used { b } else { 0 });
                c.bus.write32(b, r.next_u32() % views.max(1));
                c.bus.write32(b + snow::MODE, r.next_u32() % 3);
                c.bus.write32(b + snow::SIZE, f(100.0));
                c.bus.write32(b + snow::SPEED, r.f32(3.0));
                c.bus.write32(b + snow::WIND_SCALE, r.f32(1.0));
                c.bus.write32(b + snow::TIMER, f(1000.0));
                c.bus.write32(b + snow::PERIOD, f(2000.0));
                c.bus.write32(b + snow::SNAP, 0);
                for k in 0..4 {
                    c.bus.write32(b + snow::POS + 4 * k, r.f32(10.0));
                    c.bus.write32(b + snow::VEL + 4 * k, r.f32(2.0));
                    c.bus.write32(b + snow::PREV_VEL + 4 * k, r.f32(2.0));
                }
            }
            c.g[A0][0] = SELF as u64;
            schedule(c, &mut r, seed, 0x002e_5da0)
        };
        check_events(ee::s09::f_002e5da0__generated, snowfall_update, 200, &setup);
    }

    // ---------------------------------------------------------- SNDSTRM_Service

    /// SNDSTRM_Service: zero to four streams (empty slots, closed, open);
    /// states playing (buffer not dry), feeding (no / some free blocks) and
    /// other; prebuffering (no record, record idle, record waiting) or not;
    /// the stream file null (no chunk). The lock and unlock hooks are fakes
    /// (may unwind). Not reached: the dry-buffer restart (StopSlot /
    /// StartPlayback run the whole channel machinery) and HandleChunk (a
    /// chunk needs a live stream file).
    #[test]
    fn sndstrm_service_matches() {
        use sndstrm_service::*;
        const STREAM0: u32 = 0x00a0_0000;
        const BUF0: u32 = 0x00a1_0000;
        const BUFFER_TABLE: u32 = 0x0051_57b8 + 0x304;
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x3b_71d8, 6);
            c.bus.write32(0x0044_c46c, FAKE);
            c.bus.write32(0x0044_c470, FAKE + 0x10);
            let n = [0, 1, 2, 3, 4, 4][(seed % 6) as usize];
            c.bus.write8(SND_STATE + STREAM_COUNT, n);
            c.bus.write8(SND_STATE + 0x176, r.next_u32() as u8);
            for i in 0..4u32 {
                let st = STREAM0 + 0x200 * i;
                let present = r.next_u32() % 5 != 0;
                c.bus.write32(STREAMS + 4 * i, if present { st } else { 0 });
                fill(c, &mut r, st, 0x200 / 4);
                c.bus.write32(st + FILE, 0);
                c.bus.write32(st + OPEN, (r.next_u32() % 4 != 0) as u32);
                let state = [1u8, 1, 2, 0, 3, 0xff][(r.next_u32() % 6) as usize];
                c.bus.write8(st + STATE, state);
                let slot = r.next_u32() % 4;
                c.bus.write32(st + SLOT, slot);
                let buf = BUF0 + 0x100 * slot;
                c.bus.write32(BUFFER_TABLE + 4 * slot, buf);
                // Buffered bytes always positive (no restart).
                c.bus.write32(buf + 0x48, 1 + r.next_u32() % 0x1000);
                c.bus.write32(buf + 0x4c, r.next_u32() % 0x1000);
                c.bus.write8(buf + 0x62, (r.next_u32() % 3) as u8);
                for k in 0..4 {
                    c.bus.write16(buf + 0x34 + 2 * k, r.next_u32() as u16);
                }
                // The prebuffer record: none, idle, or waiting (-1).
                let rec = st + 0x180;
                let (ptr, busy, wait) = match r.next_u32() % 3 {
                    0 => (0, 0, 0),
                    1 => (rec, 0, 0),
                    _ => (rec, 1, u32::MAX),
                };
                c.bus.write32(st + 0x134, ptr);
                c.bus.write32(rec + 0x10, busy);
                c.bus.write32(rec + 0x20, wait);
            }
            schedule(c, &mut r, seed, 0x003b_71d8)
        };
        check_events(ee::s13::f_003b71d8__generated, sndstrm_service, 300, &setup);
    }

    // ------------------------------------------------ GameModule_BeginBezierPass

    /// GameModule_BeginBezierPass: the renderer's view-matrix method and
    /// the camera's near / far / begin methods are fakes (the matrix
    /// pointers forced to random matrices; may unwind).
    #[test]
    fn game_module_begin_bezier_pass_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use bezier_pass::*;
        const CTX: u32 = 0x00a0_0000;
        const REND: u32 = 0x00a0_1000;
        const CAM: u32 = 0x00a0_3000;
        const MAT: u32 = 0x00a0_4000;
        covered(ee::s05::f_0022c1b0__generated, game_module_begin_bezier_pass, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x22_c1b0, 5);
            c.pc = 0x0022_c1b0;
            c.bus.write32(GAME_GP - GP_RENDERER, REND);
            let vt = vtable(c, &mut r, &[SLOT_VIEW_MATRIX]);
            c.bus.write32(REND + RENDERER_VT, vt);
            c.bus.write32(GAME_GP + GP_CAMERA, CAM);
            let cvt = vtable(c, &mut r, &[SLOT_NEAR, SLOT_FAR, SLOT_BEGIN]);
            c.bus.write32(CAM + 4, cvt);
            for k in 0..2u32 {
                let m = MAT + 0x100 * k;
                for i in 0..16 {
                    c.bus.write32(m + 4 * i, r.edge_f32());
                }
                c.bus.write32(LOG_RESULTS + 4 * k, m);
            }
            fill(c, &mut r, CTX, 0x10);
            c.g[A0][0] = CTX as u64;
        });
    }

    // ------------------------------------------------------------ SkyBox_Render

    /// SkyBox_Render: sky drawing disabled, no model, or drawn (the draw
    /// method a fake that may unwind); random render-state records and
    /// layers, view index 0..3 with its camera eye, a random view matrix.
    #[test]
    fn skybox_render_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use skybox::*;
        const SKY: u32 = 0x00a0_0000;
        const REND: u32 = 0x00a0_1000;
        const RS: u32 = 0x00a0_3000;
        const MODEL_AT: u32 = 0x00a0_4000;
        const APP: u32 = 0x00a0_5000;
        const GM: u32 = 0x00a0_5100;
        const WX: u32 = 0x00a0_5200;
        const CAMS: u32 = 0x00a0_6000;
        covered(ee::s11::f_00353b10__generated, skybox_render, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x35_3b10, 1);
            c.pc = 0x0035_3b10;
            c.bus.write32(GAME_GP + GP_SKY_OFF, (seed % 9 == 4) as u32);
            c.bus.write32(GAME_GP + GP_RENDERER, REND);
            let top = RS + RS_SIZE * (r.next_u32() % 4);
            c.bus.write32(REND + RS_TOP, top);
            fill(c, &mut r, RS, 0x40);
            let vt = vtable(c, &mut r, &[SLOT_DRAW_MODEL]);
            c.bus.write32(REND + RENDERER_VT, vt);
            c.bus.write32(SKY + MODEL, if seed % 9 == 7 { 0 } else { MODEL_AT });
            fill(c, &mut r, MODEL_AT, 0x40);
            c.bus.write32(MODEL_AT + 0x80, MODEL_AT + 0x40);
            for i in 0..16 {
                c.bus.write32(VIEW_MATRIX + 4 * i, r.f32(100.0));
            }
            c.bus.write32(GAME_GP - 0x848, APP);
            c.bus.write32(APP + 0x84, GM);
            c.bus.write32(GM + 0x84, WX);
            c.bus.write32(WX + 0x14, r.next_u32() % 4);
            for v in 0..4 {
                c.bus.write32(WX + 4 + 4 * v, CAMS + 0x100 * v);
                fill(c, &mut r, CAMS + 0x100 * v + 0x20, 4);
            }
            c.g[A0][0] = SKY as u64;
        });
    }

    // ------------------------------------------------------ SNDSTRM_ReleaseBytes

    /// SNDSTRM_ReleaseBytes: the reader owning the slot or not (negative
    /// ids too), notification wanted or not, a queue already holding 0..7
    /// entries.
    #[test]
    fn sndstrm_release_bytes_matches() {
        use sndstrm_release::*;
        check(ee::s13::f_003c7010__generated, sndstrm_release_bytes, 300, &|c, seed| {
            let mut r = Rng(0x3c_7010 ^ (seed as u64) << 16);
            scramble(c, &mut r);
            c.pc = 0x003c_7010;
            let slot = r.next_u32() % 4;
            let buf = DATA + 0x100 * slot;
            c.bus.write32(BUFFERS + 4 * slot, buf);
            fill(c, &mut r, buf, 0x40);
            let reader = [0u8, 1, 2, 0xff][(r.next_u32() % 4) as usize];
            c.bus.write8(buf + READER, reader);
            c.bus.write32(buf + NOTIFY, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(QUEUE, r.next_u32() % 8);
            fill(c, &mut r, QUEUE + 4, 0x10);
            let asked = if seed % 4 == 0 { r.next_u32() % 3 } else { reader as i8 as u32 };
            c.g[A0][0] = slot as u64;
            c.g[A1][0] = sx(asked);
            c.g[A2][0] = sx(r.next_u32() % 0x10000);
        });
    }

    // ------------------------------------------- ComputerActorRider_ThinkPrewind

    /// ComputerActorRider_ThinkPrewind: the track query a fake returning
    /// 1..12 events (written on the stack where the buffer lies), with a
    /// prewind-end event first, later or not at all; slower or faster than
    /// the threshold; the scripted start (inside / outside the 50 margin);
    /// a real Path_GetPointAtDistance on a small track and a real
    /// ComputeSteer whose two virtual methods are fakes (may unwind).
    #[test]
    fn computer_actor_rider_think_prewind_matches() {
        use crate::difftest::fakes::{covered, vtable};
        use crate::difftest::{elf_image, SP};
        use prewind::*;
        const CAR: u32 = 0x00a0_0000;
        const PAD: u32 = 0x00a0_8000;
        const RIDER_AT: u32 = 0x00a1_0000;
        const TRK: u32 = 0x00a2_0000;
        const SEGS: u32 = 0x00a2_1000;
        const VECS: u32 = 0x00a3_0000;
        covered(ee::s00::f_0010aa70__generated, computer_actor_rider_think_prewind, 200, &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x10_aa70, 3);
            c.pc = 0x0010_aa70;
            let f = |x: f32| x.to_bits();
            fill(c, &mut r, CAR + CONTROL, 0x20);
            c.bus.write32(CAR + CONTROL, f((r.next_u32() % 60) as f32));
            c.bus.write32(CAR + CONTROL + 0x48, uf(&mut r, 0.0, 1.0));
            c.bus.write32(CAR + CONTROL + 0x4c, uf(&mut r, 0.0, 1.0));
            c.bus.write32(CAR + RIDER, RIDER_AT);
            c.bus.write32(CAR + SCRIPTED, (seed % 4 == 0) as u32);
            for off in [SCRIPT_FROM, SCRIPT_TO] {
                put_vec(c, CAR + off, [r.f32(200.0), r.f32(200.0), r.f32(200.0), 0]);
            }
            fill(c, &mut r, PAD, 2);
            // The rider.
            for off in [POS, VELOCITY, AHEAD] {
                put_vec(c, RIDER_AT + off, [r.f32(60.0), r.f32(60.0), r.f32(60.0), 0]);
            }
            c.bus.write32(RIDER_AT + TRACK_DIST, uf(&mut r, 0.0, 200.0));
            c.bus.write32(RIDER_AT + TRACK_SIDE, r.f32(20.0));
            c.bus.write32(RIDER_AT + TRACK, TRK);
            let mvt = vtable(c, &mut r, &[0x10, 0x28]);
            c.bus.write32(RIDER_AT + 0x6c0, mvt);
            // The track: Query (fake) and the path segments.
            let tvt = vtable(c, &mut r, &[SLOT_QUERY]);
            c.bus.write32(TRK + TRACK_VT, tvt);
            c.bus.write32(TRK + 8, r.next_u32() % 4);
            for k in 0..3 {
                c.bus.write32(TRK + 0xc + 4 * k, r.f32(100.0));
            }
            c.bus.write32(TRK + 0x18, SEGS);
            for i in 0..4 {
                put_vec(c, SEGS + 16 * i, [r.f32(2.0), r.f32(2.0), r.f32(2.0), uf(&mut r, 10.0, 60.0)]);
            }
            // Query's result and the events it "wrote".
            let n = 1 + r.next_u32() % 12;
            c.bus.write32(LOG_RESULTS, n);
            let sel = match seed % 3 {
                0 => 0,
                1 => r.next_u32() % n,
                _ => u32::MAX,
            };
            let buf = SP - 0x160;
            for i in 0..12 {
                let e = buf + 16 * i;
                fill(c, &mut r, e, 4);
                let ty = if i == sel { 0x10 } else { r.next_u32() % 0x10 };
                c.bus.write32(e, ty);
                c.bus.write32(e + 0xc, uf(&mut r, 0.0, 250.0));
            }
            // ComputeSteer's axis and motion vectors.
            put_vec(c, VECS, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            put_vec(c, VECS + 0x10, [0, 0, 0, 0]);
            c.bus.write32(LOG_RESULTS + 4, VECS);
            c.bus.write32(LOG_RESULTS + 8, VECS + 0x10);
            c.g[A0][0] = CAR as u64;
            c.g[A1][0] = PAD as u64;
        });
    }
}
