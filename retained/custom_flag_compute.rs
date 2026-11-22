//! Retained port of native `cFlagManager_GroupComputeVerts` (0x0034bca0).
//!
//! This is deliberately uninstalled: the user has disabled automated tests.
//! The two column waves include cumulative Y bending unless both column
//! edges are pinned. Merely copying a source X-wave's amplitude and speed
//! into the native descriptor therefore does not reproduce that source law.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0034_bca0;
const AFTER_WIND: u32 = 0x0034_bcdc;
const END: u32 = 0x0034_c2e0;
const GET_FLAG_MANAGER: u32 = 0x002d_1cb0;

const PIN_LEFT: u32 = 0;
const PIN_RIGHT: u32 = 1;
const PIN_FIRST_ROW: u32 = 2;
const AMPLITUDES: u32 = 0x14;
const SCALES: u32 = 0x24;
const BEND_STILL: u32 = 0x34;
const BEND_WIND: u32 = 0x40;
const BLEND: u32 = 0x4c;
const COLS: u32 = 0x64;
const ROWS: u32 = 0x68;
const COL_DIVISOR: u32 = 0x6c;
const ROW_DIVISOR: u32 = 0x70;
const BASE_VERTICES: u32 = 0x74;
const PHASES: u32 = 0x78;

fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}
fn literal(c: &Cpu, offset: u32) -> u32 {
    r(c, GP).wrapping_sub(offset)
}

/// Integer angle in v0 -> sine and cosine through the original 512-step
/// table, with its extra quarter-turn tail. Preserve the pointer scratch
/// registers rather than replacing this with host trigonometry.
fn lookup(c: &mut Cpu, table: usize, sine: usize, cosine: usize) {
    c.g[V0][0] &= 0x1ff;
    c.g[V1][0] = sx(r(c, V0).wrapping_add(0x80));
    c.g[V0][0] = sx(r(c, V0) << 2);
    c.g[V1][0] = sx(r(c, V1) << 2);
    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, table)));
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, table)));
    c.lwc1(sine, r(c, V0));
    c.lwc1(cosine, r(c, V1));
}

/// Form four phase angles and four wind-scaled amplitudes in stack scratch.
fn prepare_waves(c: &mut Cpu) {
    c.lwc1(2, at(c, S0, BLEND));
    c.g[S1][0] = (c.g[S1][0] < 1) as u64;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 1);
    c.g[A1][0] = sx(at(c, SP, 0x10));
    c.lwc1(0, at(c, V0, 0x10));
    c.g[A0][0] = sx(at(c, S0, AMPLITUDES));
    c.sub_s(1, 1, 2);
    c.lwc1(3, literal(c, 0x2afc)); // -2*pi
    c.g[V1][0] = c.g[SP][0];
    c.g[V0][0] = sx(at(c, S0, PHASES));
    c.g[A2][0] = 3;
    c.mul_s(0, 0, 1);
    c.add_s(12, 2, 0); // blend + wind.rate*(1-blend)
    loop {
        c.cycles += 12;
        c.lwc1(1, r(c, V0));
        c.g[A2][0] = sx(r(c, A2).wrapping_sub(1));
        c.lwc1(0, r(c, A0));
        c.g[V0][0] = sx(at(c, V0, 4));
        c.mul_s(1, 1, 3);
        c.g[A0][0] = sx(at(c, A0, 4));
        c.mul_s(0, 0, 12);
        c.swc1(1, r(c, V1));
        c.swc1(0, r(c, A1));
        c.g[V1][0] = sx(at(c, V1, 4));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[A1][0] = sx(at(c, A1, 4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_bd18);
        }
    }
}

/// Column envelope in f3, unmirrored phase fraction in f4, destination column
/// in a0. Right-only pinning reverses destination order; both pinned edges
/// use the triangular envelope and disable cumulative Y bending.
fn column_envelope(c: &mut Cpu) {
    c.cycles += 5;
    c.lwc1(0, at(c, S0, COL_DIVISOR));
    c.g[A0][0] = c.g[A2][0];
    c.div_s(3, 13, 0);
    let has_pin = c.g[S1][0] == 0;
    c.mov_s(4, 3);
    if !has_pin {
        c.cycles += 4;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 3);
        return;
    }
    c.cycles += 2;
    if c.g[S2][0] != 0 {
        c.cycles += 6;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(AT, 0);
        c.c_lt_s(0, 3);
        let upper_half = c.fcc();
        if upper_half {
            c.sub_s(3, 14, 3);
            return;
        }
        c.cycles += 2;
        return;
    }
    c.g[V1][0] = c.lb(at(c, S0, PIN_RIGHT));
    c.cycles += 3;
    c.g[V0][0] = 1;
    if c.g[V1][0] != c.g[V0][0] {
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(at(c, S0, COLS));
    c.g[V0][0] = sx(r(c, V0).wrapping_sub(r(c, A2)));
    c.g[A0][0] = sx(r(c, V0).wrapping_sub(1));
}

/// Add the wind/still bend vectors to the column scratch vector. f12 is the
/// wind blend; these descriptor vectors are in the mesh's model coordinates.
fn column_bend(c: &mut Cpu) {
    c.cycles += 64;
    c.lwc1(1, at(c, SP, 0x14));
    c.sub_s(5, 14, 12);
    c.swc1(15, at(c, SP, 0xa4));
    c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
    c.mul_s(1, 1, 3);
    c.sw(at(c, SP, 0xa8), 0);
    c.lwc1(10, at(c, S0, BEND_WIND));
    c.mul_s(1, 1, 2);
    c.lwc1(9, at(c, S0, BEND_WIND + 4));
    c.lwc1(7, at(c, S0, BEND_WIND + 8));
    c.mul_s(10, 10, 3);
    c.lwc1(8, at(c, S0, BEND_STILL));
    c.mul_s(9, 9, 3);
    c.lwc1(6, at(c, S0, BEND_STILL + 4));
    c.mul_s(7, 7, 3);
    c.swc1(1, at(c, SP, 0xa0));
    c.mul_s(8, 8, 3);
    c.mul_s(6, 6, 3);
    c.lwc1(0, at(c, A0, 0x20));
    c.mul_s(10, 10, 12);
    c.lwc1(4, at(c, S0, BEND_STILL + 8));
    c.mul_s(8, 8, 5);
    c.add_s(0, 0, 1);
    c.lwc1(2, at(c, A0, 0x24));
    c.mul_s(4, 4, 3);
    c.lwc1(3, at(c, A0, 0x28));
    c.mul_s(9, 9, 12);
    c.swc1(0, at(c, A0, 0x20));
    c.mul_s(7, 7, 12);
    c.mul_s(4, 4, 5);
    c.lwc1(0, at(c, SP, 0xa4));
    c.mul_s(6, 6, 5);
    c.add_s(11, 10, 8);
    c.add_s(2, 2, 0);
    c.add_s(1, 7, 4);
    c.add_s(5, 9, 6);
    c.swc1(2, at(c, A0, 0x24));
    c.lwc1(0, at(c, SP, 0xa8));
    c.add_s(3, 3, 0);
    c.swc1(3, at(c, A0, 0x28));
    c.swc1(5, at(c, SP, 0xc4));
    c.swc1(1, at(c, SP, 0xc8));
    c.swc1(10, at(c, SP, 0xa0));
    c.swc1(9, at(c, SP, 0xa4));
    c.swc1(7, at(c, SP, 0xa8));
    c.swc1(8, at(c, SP, 0xb0));
    c.swc1(6, at(c, SP, 0xb4));
    c.swc1(4, at(c, SP, 0xb8));
    c.swc1(11, at(c, SP, 0xc0));
    c.lwc1(1, at(c, A0, 0x20));
    c.lwc1(2, at(c, A0, 0x24));
    c.add_s(1, 1, 11);
    c.lwc1(3, at(c, A0, 0x28));
    c.swc1(1, at(c, A0, 0x20));
    c.lwc1(0, at(c, SP, 0xc4));
    c.add_s(2, 2, 0);
    c.swc1(2, at(c, A0, 0x24));
    c.lwc1(0, at(c, SP, 0xc8));
    c.add_s(3, 3, 0);
    c.swc1(3, at(c, A0, 0x28));
    c.g[V0][0] = c.lw(at(c, S0, COLS));
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    c.add_s(13, 13, 14);
}

/// Build one displacement per column at stack+20. Native waves0/1 displace
/// X sinusoidally and also bend Y cumulatively when the ends are not both
/// pinned. Z is zero until the authored bend vector is added.
fn compute_columns(c: &mut Cpu) {
    c.cycles += 9;
    c.g[A0][0] = c.lw(at(c, S0, COLS));
    c.g[V0][0] = 9;
    c.g[V1][0] = u64::MAX;
    c.mtc1(ZERO, 13);
    c.g[A2][0] = 0;
    c.mov_s(16, 13);
    let empty = (c.g[A0][0] as i64) <= 0;
    c.mov_s(15, 13);
    if empty {
        return;
    }
    c.cycles += 3;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 14);
    loop {
        column_envelope(c);
        c.cycles += 22;
        c.lwc1(2, literal(c, 0x2af8)); // 2*pi
        c.g[V0][0] = 0x0050_0000;
        c.lwc1(1, at(c, S0, SCALES));
        c.g[A1][0] = sx(at(c, V0, 0x4fb8));
        c.mul_s(5, 4, 2);
        c.lwc1(0, r(c, SP));
        c.lwc1(6, literal(c, 0x2af4)); // 512/(2*pi)
        c.lwc1(4, at(c, SP, 0x10));
        c.mul_s(1, 5, 1);
        c.add_s(0, 0, 1);
        c.mul_s(0, 0, 6);
        c.cvt_w_s(1, 0);
        c.mfc1(V0, 1);
        lookup(c, A1, 2, 1);
        if c.g[S2][0] == 0 {
            c.cycles += 3;
            c.mul_s(0, 4, 3);
            c.mul_s(0, 0, 1);
            c.add_s(16, 16, 0);
        }
        c.cycles += 30;
        c.mul_s(0, 4, 3);
        c.g[V0][0] = 12;
        c.mult(0, V1, A0, V0);
        c.swc1(16, at(c, SP, 0xa4));
        c.sw(at(c, SP, 0xa8), 0);
        c.mul_s(0, 0, 2);
        c.lwc1(1, at(c, S0, SCALES + 4));
        c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, SP)));
        c.swc1(0, at(c, SP, 0xa0));
        c.mul_s(1, 5, 1);
        c.ldl(A3, at(c, SP, 0xa7));
        c.ldr(A3, at(c, SP, 0xa0));
        c.g[T0][0] = c.lw(at(c, SP, 0xa8));
        c.sdl(A3, at(c, A0, 0x27));
        c.sdr(A3, at(c, A0, 0x20));
        c.sw(at(c, A0, 0x28), c.g[T0][0]);
        c.lwc1(0, at(c, SP, 4));
        c.add_s(0, 0, 1);
        c.mul_s(0, 0, 6);
        c.cvt_w_s(1, 0);
        c.mfc1(V0, 1);
        lookup(c, A1, 2, 1);
        if c.g[S2][0] == 0 {
            c.cycles += 4;
            c.lwc1(0, at(c, SP, 0x14));
            c.mul_s(0, 0, 3);
            c.mul_s(0, 0, 1);
            c.add_s(15, 15, 0);
        }
        column_bend(c);
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_bd78);
        }
    }
}

/// f8 is the pin envelope, f9 the row fraction, f10 the phase fraction.
fn vertex_envelope(c: &mut Cpu) {
    c.cycles += 6;
    c.lwc1(0, at(c, S0, COL_DIVISOR));
    c.lwc1(1, at(c, S0, ROW_DIVISOR));
    c.div_s(8, 13, 0);
    c.div_s(9, 18, 1);
    let both_pins = c.g[S2][0] != 0;
    c.mov_s(10, 8);
    if both_pins {
        c.cycles += 4;
        c.c_lt_s(11, 8);
        let upper_half = c.fcc();
        if upper_half {
            c.sub_s(8, 12, 8);
            c.cycles += 1;
            c.g[V0][0] = c.lb(at(c, S0, PIN_FIRST_ROW));
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.lb(at(c, S0, PIN_FIRST_ROW));
        }
        return;
    }
    c.cycles += 4;
    c.g[V1][0] = c.lb(at(c, S0, PIN_RIGHT));
    c.g[V0][0] = 1;
    if c.g[V1][0] == c.g[V0][0] {
        c.cycles += 3;
        c.sub_s(8, 12, 8);
        c.mov_s(10, 8);
        c.cycles += 1;
        c.g[V0][0] = c.lb(at(c, S0, PIN_FIRST_ROW));
        return;
    }
    c.cycles += 2;
    if c.g[S1][0] == 0 {
        c.g[V0][0] = c.lb(at(c, S0, PIN_FIRST_ROW));
        return;
    }
    c.cycles += 2;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 8);
    c.cycles += 1;
    c.g[V0][0] = c.lb(at(c, S0, PIN_FIRST_ROW));
}

/// Wave2 adds a three-axis diagonal ripple weighted toward the last row.
fn row_wave_two(c: &mut Cpu) {
    c.cycles += 37;
    c.add_s(0, 10, 9);
    c.lwc1(2, at(c, S0, SCALES + 8));
    c.add_s(3, 8, 9);
    c.mul_s(2, 2, 16);
    c.lwc1(5, r(c, A0));
    c.mul_s(0, 0, 11);
    c.mul_s(3, 3, 11);
    c.mul_s(4, 6, 9);
    c.mul_s(0, 0, 2);
    c.mul_s(3, 6, 3);
    c.mul_s(2, 6, 8);
    c.add_s(1, 1, 0);
    c.mul_s(1, 1, 15);
    c.cvt_w_s(0, 1);
    c.mfc1(V0, 0);
    lookup(c, T0, 0, 1);
    c.mul_s(4, 4, 0);
    c.mul_s(3, 3, 1);
    c.mul_s(2, 2, 0);
    c.swc1(4, at(c, SP, 0xa8));
    c.swc1(3, at(c, SP, 0xa4));
    c.swc1(2, at(c, SP, 0xa0));
    c.add_s(5, 5, 2);
    c.lwc1(0, at(c, A0, 4));
    c.lwc1(1, at(c, A0, 8));
    c.add_s(0, 0, 3);
    c.swc1(5, r(c, A0));
    c.add_s(1, 1, 4);
    c.swc1(0, at(c, A0, 4));
    c.swc1(1, at(c, A0, 8));
}

/// Wave3 is the complementary diagonal ripple, weighted toward row zero.
fn row_wave_three(c: &mut Cpu) {
    c.cycles += 39;
    c.add_s(0, 10, 9);
    c.lwc1(2, at(c, S0, SCALES + 12));
    c.add_s(4, 8, 9);
    c.mul_s(2, 2, 16);
    c.lwc1(6, r(c, A0));
    c.mul_s(0, 0, 11);
    c.mul_s(4, 4, 11);
    c.sub_s(3, 12, 9);
    c.mul_s(0, 0, 2);
    c.mul_s(5, 7, 8);
    c.mul_s(3, 7, 3);
    c.add_s(1, 1, 0);
    c.mul_s(4, 7, 4);
    c.mul_s(1, 1, 15);
    c.cvt_w_s(0, 1);
    c.mfc1(V0, 0);
    lookup(c, T0, 0, 1);
    c.mul_s(5, 5, 0);
    c.mul_s(4, 4, 1);
    c.mul_s(3, 3, 0);
    c.swc1(5, at(c, SP, 0xa0));
    c.add_s(6, 6, 5);
    c.swc1(4, at(c, SP, 0xa4));
    c.swc1(3, at(c, SP, 0xa8));
    c.swc1(6, r(c, A0));
    c.lwc1(1, at(c, A0, 4));
    c.lwc1(2, at(c, A0, 8));
    c.add_s(1, 1, 4);
    c.swc1(1, at(c, A0, 4));
    c.lwc1(0, at(c, SP, 0xa8));
    c.add_s(2, 2, 0);
    c.swc1(2, at(c, A0, 8));
}

/// Copy the unchanged first row for flag2; otherwise add its column offset
/// and the optional diagonal ripples to the reference model-space vertex.
fn write_vertex(c: &mut Cpu) {
    c.cycles += 2;
    let pin_row = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(at(c, S0, BASE_VERTICES));
    if pin_row {
        c.cycles += 3;
        c.c_eq_s(9, 14);
        if c.fcc() {
            c.cycles += 8;
            c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, V0)));
            c.ldl(V1, at(c, V0, 7));
            c.ldr(V1, r(c, V0));
            c.g[T5][0] = c.lw(at(c, V0, 8));
            c.sdl(V1, at(c, A0, 7));
            c.sdr(V1, r(c, A0));
            c.sw(at(c, A0, 8), c.g[T5][0]);
            return;
        }
    }
    c.cycles += 22;
    c.c_lt_s(14, 10);
    c.lwc1(5, r(c, A1));
    c.g[V0][0] = sx(r(c, A2).wrapping_add(r(c, V0)));
    c.lwc1(4, at(c, A1, 4));
    c.lwc1(2, at(c, V0, 8));
    c.lwc1(0, r(c, V0));
    c.lwc1(1, at(c, V0, 4));
    c.lwc1(3, at(c, A1, 8));
    c.add_s(0, 0, 5);
    c.add_s(1, 1, 4);
    c.add_s(2, 2, 3);
    c.swc1(0, at(c, SP, 0xa0));
    c.swc1(1, at(c, SP, 0xa4));
    c.swc1(2, at(c, SP, 0xa8));
    c.ldl(V0, at(c, SP, 0xa7));
    c.ldr(V0, at(c, SP, 0xa0));
    c.g[V1][0] = c.lw(at(c, SP, 0xa8));
    c.sdl(V0, at(c, A0, 7));
    c.sdr(V0, r(c, A0));
    c.sw(at(c, A0, 8), c.g[V1][0]);
    let positive_phase_fraction = c.fcc();
    c.lwc1(6, at(c, SP, 0x18));
    if positive_phase_fraction {
        c.cycles += 4;
        c.c_lt_s(17, 6);
        let wave_two = c.fcc();
        c.lwc1(1, at(c, SP, 8));
        if wave_two {
            row_wave_two(c);
        }
        c.cycles += 2;
        c.c_lt_s(14, 10);
    }
    c.cycles += 2;
    let wave_three_region = c.fcc();
    c.lwc1(7, at(c, SP, 0x1c));
    if wave_three_region {
        c.cycles += 4;
        c.c_lt_s(17, 7);
        let wave_three = c.fcc();
        c.lwc1(1, at(c, SP, 12));
        if wave_three {
            row_wave_three(c);
        }
    }
    c.cycles += 4;
    c.g[A0][0] = sx(at(c, A0, 12));
    c.g[A2][0] = sx(at(c, A2, 12));
    c.g[T1][0] = sx(at(c, T1, 12));
    c.g[T2][0] = sx(r(c, T2).wrapping_add(1));
}

fn compute_rows(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw(at(c, S0, ROWS));
    c.g[T2][0] = 0;
    c.mtc1(ZERO, 18);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[V1][0] = 0;
    if empty {
        return;
    }
    c.cycles += 3;
    c.g[T4][0] = 12;
    c.g[T1][0] = 0;
    loop {
        c.cycles += 5;
        c.g[V0][0] = c.lw(at(c, S0, COLS));
        c.g[T3][0] = sx(r(c, V1).wrapping_add(1));
        c.mtc1(ZERO, 13);
        let empty_row = (c.g[V0][0] as i64) <= 0;
        c.g[A3][0] = 0;
        if !empty_row {
            c.cycles += 13;
            c.mult(0, A2, T2, T4);
            c.g[V0][0] = 0x0050_0000;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(AT, 11);
            c.g[T0][0] = sx(at(c, V0, 0x4fb8));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 12);
            c.mov_s(14, 13);
            c.lwc1(17, literal(c, 0x2af0)); // 0.01
            c.g[A1][0] = sx(at(c, SP, 0x20));
            c.lwc1(16, literal(c, 0x2aec)); // 2*pi
            c.g[A0][0] = sx(r(c, T1).wrapping_add(r(c, S3)));
            c.lwc1(15, literal(c, 0x2ae8)); // 512/(2*pi)
            loop {
                vertex_envelope(c);
                write_vertex(c);
                c.cycles += 6;
                c.g[V0][0] = c.lw(at(c, S0, COLS));
                c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                c.g[A1][0] = sx(at(c, A1, 12));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.add_s(13, 13, 12);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0034_c030);
                }
            }
        }
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[V1][0] = c.g[T3][0];
        c.g[V0][0] = c.lw(at(c, S0, ROWS));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.add_s(18, 18, 0);
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0034_bfe8);
        }
    }
}

/// `void GroupComputeVerts(Group* group, Vec3* output)` (0x34bca0).
/// Build four source-parameter wave components in the group's model frame.
/// Geometry construction chooses the sampling divisors elsewhere; this
/// function neither normalizes a model scale nor changes texture coordinates.
pub fn flag_manager_group_compute_vertices(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x120 };
        match c.pc {
            ENTRY => {
                c.cycles += 11;
                Frame::open(c, 0x120);
                frame.save(c, S0, 0x110);
                frame.save(c, S2, 0xf0);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S3, 0xe0);
                c.g[S2][0] = 0;
                frame.save(c, S1, 0x100);
                frame.save_ra(c, 0xd0);
                c.g[V0][0] = c.lb(at(c, S0, PIN_LEFT));
                let left_pin = c.g[V0][0] != 0;
                c.g[S3][0] = c.g[A1][0];
                if left_pin {
                    c.cycles += 2;
                    c.g[V0][0] = c.lb(at(c, S0, PIN_RIGHT));
                    c.g[S2][0] = (0 < c.g[V0][0]) as u64;
                }
                c.cycles += 17;
                c.g[RA][0] = sx(AFTER_WIND);
                c.g[S1][0] = c.lhu(at(c, S0, PIN_LEFT));
                call_ptr(c, GET_FLAG_MANAGER, AFTER_WIND)?;
            }
            AFTER_WIND => {}
            other => {
                c.jump_out(other, ENTRY, END);
                return Ok(());
            }
        }
        prepare_waves(c);
        compute_columns(c);
        compute_rows(c);
        c.cycles += 7;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    });
}
