//! Native one-player HUD value/clock panel (0x21f660).
//!
//! Despite its generated name, this routine does not read the event deadline
//! or its timer-enabled flag. It formats the per-rider HUD value table as a
//! time in Race/Rival Race and as an integer in the other event modes.
//! Retained uninstalled; no tests or game/build runs were performed.

use crate::call::{call_ptr, guest, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0021_f660, draw_race_time, "cOVStateHUD1P_DrawRaceTime")];

const WORLD_GP_NEG: u32 = 0x848;
const HUD_CONTEXT_GP_NEG: u32 = 0x854;
const EVENT_MODE: u32 = 0x0053_5c10;
const RIDER_VALUES: u32 = 0x0053_6640;
const TIMER_FONT: u32 = 0x4b4;
const SMALL_FONT: u32 = 0x49c;
const TEXT_RENDERER: u32 = 0x42c;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

#[inline]
fn gp(c: &Cpu, negative: u32) -> u32 {
    r(c, GP).wrapping_sub(negative)
}

/// Draw the selected element's backdrop. The original keeps all its working
/// layout addresses in s0..s5, which remain live for the following text pass.
fn draw_backdrop(c: &mut Cpu) -> Flow {
    c.cycles += 60;
    c.g[S0][0] = 0x24;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 0);
    c.mult(0, V0, A0, S0);
    c.g[S2][0] = sx(at(c, SP, 0x20));
    c.swc1(0, at(c, SP, 4));
    c.swc1(0, r(c, SP));
    c.g[S0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001e_91f8, 0x0021_f6f8)?;
    c.g[S1][0] = sx(at(c, SP, 0x30));
    c.g[A2][0] = c.g[S5][0];
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    call_ptr(c, 0x001e_9220, 0x0021_f70c)?;
    c.g[A1][0] = sx(at(c, SP, 0x40));
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x001e_9290, 0x0021_f718)?;
    c.lwc1(0, at(c, SP, 0x30));
    c.g[T3][0] = c.g[S3][0];
    c.lwc1(1, at(c, SP, 0x34));
    c.g[S3][0] = sx(at(c, SP, 0x10));
    c.lwc1(2, at(c, SP, 0x40));
    c.g[A2][0] = c.g[S2][0];
    c.lwc1(3, at(c, SP, 0x44));
    c.g[T0][0] = c.g[SP][0];
    c.mul_s(0, 0, 2);
    c.g[A3][0] = c.g[S1][0];
    c.mul_s(1, 1, 3);
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S3][0];
    c.swc1(0, at(c, SP, 0x30));
    c.swc1(1, at(c, SP, 0x34));
    c.g[T2][0] = c.lb(at(c, S0, 0x21));
    c.g[T1][0] = c.lb(at(c, S0, 0x20));
    call_ptr(c, 0x001f_10f8, 0x0021_f760)?;
    c.g[S2][0] = sx(at(c, SP, 0x50));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, 0x001e_91a8, 0x0021_f770)?;
    c.g[V0][0] = c.lw(gp(c, HUD_CONTEXT_GP_NEG));
    c.g[T0][0] = 0xffff_ffff_ffff_fc1f;
    c.g[V1][0] = c.lb(at(c, S0, 0x22));
    c.g[A1][0] = c.g[S5][0];
    c.g[T2][0] = c.lw(at(c, V0, 0xe84));
    c.g[A2][0] = c.g[S3][0];
    c.g[V1][0] = sx(r(c, V1) << 5);
    c.g[A3][0] = c.g[S1][0];
    c.g[V0][0] = c.lw(at(c, T2, 8));
    c.g[V1][0] &= 0x3e0;
    c.g[T1][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[V0][0] &= c.g[T0][0];
    c.g[V0][0] |= c.g[V1][0];
    c.g[T0][0] = c.g[SP][0];
    c.sw(at(c, T2, 8), c.g[V0][0]);
    call_ptr(c, 0x001f_1190, 0x0021_f7b4)
}

/// Format the player's table value. Modes 0/5 display hours/minutes/seconds;
/// other modes add the descriptor's value at +0x7c and display an integer.
fn format_value(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    call_ptr(c, 0x0014_dc80, 0x0021_f7c4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0021_f7d0)?;
    c.g[A1][0] = c.lw(at(c, S7, 0x6c0));
    c.g[V0][0] = 0x0053_0000;
    c.g[S0][0] = c.lb(EVENT_MODE);
    c.g[V1][0] = sx(at(c, S7, 0x6c0));
    c.g[A0][0] = c.lh(at(c, A1, 0x38));
    c.g[V0][0] = c.lw(at(c, A1, 0x3c));
    let rider_index = r(c, V0);
    c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
    call_ptr(c, rider_index, 0x0021_f7f0)?;
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[V1][0] = sx(RIDER_VALUES);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
    let mut time_mode = c.g[S0][0] == 0;
    c.g[V1][0] = c.lw(r(c, V0));
    if !time_mode {
        c.cycles += 3;
        c.g[V0][0] = 5;
        time_mode = c.g[S0][0] == c.g[V0][0];
        if !time_mode {
            c.g[A2][0] = c.lw(at(c, FP, 0x7c));
        }
    }
    if time_mode {
        c.cycles += 7;
        c.mtc1(V1, 0);
        c.cvt_s_w(0, 0);
        c.lwc1(1, gp(c, 0x5344));
        c.g[V1][0] = 0xe10;
        c.g[V0][0] = 0xe10;
        c.cycles += 18;
        c.g[A3][0] = 0x3c;
        c.mul_s(0, 0, 1);
        c.g[V0][0] = 0x3c;
        c.g[S0][0] = sx(at(c, SP, 0x60));
        c.cvt_w_s(1, 0);
        c.mfc1(A2, 1);
        c.g[A1][0] = 0x0047_0000;
        c.g[A1][0] = sx(0x0046_ed68);
        c.div(0, A2, V1);
        c.g[A2][0] = c.lo[0];
        c.g[V0][0] = c.hi[0];
        c.div(0, V0, A3);
        c.g[A3][0] = c.lo[0];
        c.g[T0][0] = c.hi[0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_7828, 0x0021_f870)?;
        c.g[A1][0] = c.g[S0][0];
    } else {
        c.cycles += 7;
        c.g[S0][0] = sx(at(c, SP, 0x60));
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(0x004a_2138);
        c.g[A2][0] = sx(r(c, V1).wrapping_add(r(c, A2)));
        call_ptr(c, 0x0041_7828, 0x0021_f890)?;
        c.g[A1][0] = c.g[S0][0];
    }
    Ok(())
}

fn draw_value(c: &mut Cpu) -> Flow {
    c.cycles += 59;
    c.lwc1(0, at(c, SP, 0x30));
    c.g[V1][0] = 0x004d_0000;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 1);
    c.lwc1(2, at(c, SP, 0x10));
    c.mul_s(0, 0, 1);
    c.lwc1(3, at(c, SP, 0x14));
    c.lwc1(4, at(c, SP, 0x40));
    c.lwc1(1, at(c, SP, 0x44));
    c.lwc1(5, at(c, SP, 0x34));
    c.add_s(2, 2, 0);
    c.lwc1(6, gp(c, 0x5340));
    c.add_s(3, 3, 5);
    c.g[V0][0] = c.lw(at(c, S4, TEXT_RENDERER));
    c.mul_s(1, 1, 6);
    c.swc1(2, at(c, SP, 0x10));
    c.mul_s(4, 4, 6);
    c.swc1(3, at(c, SP, 0x14));
    c.swc1(1, at(c, SP, 0x44));
    c.swc1(4, at(c, SP, 0x40));
    c.lwc1(0, at(c, V0, 0x30));
    c.lwc1(2, at(c, V0, 0x34));
    c.mul_s(0, 0, 4);
    c.swc1(0, at(c, V0, 0x38));
    c.lwc1(1, at(c, SP, 0x44));
    c.mul_s(2, 2, 1);
    c.swc1(2, at(c, V0, 0x3c));
    c.g[A3][0] = sx(0x004c_8808);
    // Keep the original unaligned loads/stores and their register results.
    c.ldl(V0, at(c, A3, 7));
    c.ldr(V0, r(c, A3));
    c.ldl(A0, at(c, A3, 0xf));
    c.ldr(A0, at(c, A3, 8));
    c.sdl(V0, at(c, SP, 0xa7));
    c.sdr(V0, at(c, SP, 0xa0));
    c.sdl(A0, at(c, SP, 0xaf));
    c.sdr(A0, at(c, SP, 0xa8));
    c.g[V0][0] = c.lw(at(c, S4, TEXT_RENDERER));
    c.ldl(V1, at(c, SP, 0xa7));
    c.ldr(V1, at(c, SP, 0xa0));
    c.ldl(A0, at(c, SP, 0xaf));
    c.ldr(A0, at(c, SP, 0xa8));
    c.sdl(V1, at(c, V0, 0x47));
    c.sdr(V1, at(c, V0, 0x40));
    c.sdl(A0, at(c, V0, 0x4f));
    c.sdr(A0, at(c, V0, 0x48));
    c.g[A0][0] = c.lw(at(c, S4, TEXT_RENDERER));
    c.lwc1(12, at(c, SP, 0x10));
    c.lwc1(13, at(c, SP, 0x14));
    call_ptr(c, 0x0039_2908, 0x0021_f958)?;
    c.g[V0][0] = c.lw(at(c, S4, TEXT_RENDERER));
    c.lwc1(1, r(c, SP));
    c.lwc1(0, at(c, V0, 0x30));
    c.lwc1(2, at(c, V0, 0x34));
    c.mul_s(0, 0, 1);
    c.swc1(0, at(c, V0, 0x38));
    c.lwc1(1, at(c, SP, 4));
    c.mul_s(2, 2, 1);
    c.swc1(2, at(c, V0, 0x3c));
    c.lq(S0, at(c, SP, 0x140));
    Ok(())
}

/// `void DrawRaceTime(HUD* self, Element* elements, Descriptor* rider,
///                    int alignment, int backdrop_only)` (0x21f660).
pub fn draw_race_time(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = c.lw(gp(c, WORLD_GP_NEG));
        frame.save(c, S3, 0x110);
        frame.save(c, S4, 0x100);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0xe0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, FP, 0xc0);
        c.g[S6][0] = c.g[T0][0];
        frame.save(c, S0, 0x140);
        c.g[FP][0] = c.g[A2][0];
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        frame.save(c, S5, 0xf0);
        frame.save(c, S7, 0xd0);
        frame.save_ra(c, 0xb0);
        c.g[A0][0] = c.lw(at(c, V0, 0x84));
        c.g[V1][0] = c.lw(r(c, FP));
        c.g[V0][0] = c.lw(at(c, A0, 0xc));
        c.g[V1][0] = sx(r(c, V1) << 2);
        c.g[A0][0] = 0x42;
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[S5][0] = c.lw(at(c, S4, TIMER_FONT));
        let small = c.g[S6][0] != 0;
        c.g[S7][0] = c.lw(at(c, V0, 0x28));
        if small {
            c.cycles += 2;
            c.g[S5][0] = c.lw(at(c, S4, SMALL_FONT));
            c.g[A0][0] = 0x1b;
        }
        draw_backdrop(c)?;
        let backdrop_only = c.g[S6][0] != 0;
        frame.restore(c, S0, 0x140);
        if !backdrop_only {
            format_value(c)?;
            draw_value(c)?;
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore(c, S3, 0x110);
        frame.restore(c, S4, 0x100);
        frame.restore(c, S5, 0xf0);
        frame.restore(c, S6, 0xe0);
        frame.restore(c, S7, 0xd0);
        frame.restore(c, FP, 0xc0);
        frame.restore_ra(c, 0xb0);
        frame.close(c);
        Ok(())
    });
}
