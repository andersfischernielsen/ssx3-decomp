//! Exact native Rider_ResetAtMatrix0x123210, retained uninstalled.
//!
//! No tests, builds or game runs were added or executed. The user's explicit
//! development instruction forbids automated tests and replays.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0012_3210, rider_reset_at_matrix, "Rider_ResetAtMatrix",
)];

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Transform one native local vec4 by the supplied marker matrix. Keep the
/// original VU0 accumulator/lane operations, including the W coordinate.
#[inline]
fn marker_vector(c: &mut Cpu, vector: u32, output: u32) {
    c.lqc2(8, vector);
    c.lqc2(4, r(c, S0));
    c.lqc2(5, at(c, S0, 0x10));
    c.lqc2(6, at(c, S0, 0x20));
    c.lqc2(7, at(c, S0, 0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, output);
}

/// Build the native six-rider metre offsets. The human rider0 is two metres
/// forward of the marker. This function does not search for a safe offset.
fn lane_offset(c: &mut Cpu) {
    c.lq(V0, at(c, S0, 0x30));
    c.g[V1][0] = c.lw(at(c, S2, 0x86c));
    c.sw(at(c, SP, 0x10), 0);
    c.sq(V0, r(c, SP));
    c.sw(at(c, SP, 0x14), 0);
    c.sw(at(c, SP, 0x18), 0);
    let lane_zero = c.g[V1][0] == 0;
    c.sw(at(c, SP, 0x1c), 0);
    let mut write_y_float = false;
    let mut clear_z = true;
    if lane_zero {
        c.cycles += 5;
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(AT, 0);
        c.sw(at(c, SP, 0x24), 0);
        c.swc1(0, at(c, SP, 0x20));
    } else {
        c.cycles += 3;
        c.g[V0][0] = 1;
        let is_one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        if is_one {
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.sw(at(c, SP, 0x24), 0);
            c.swc1(0, at(c, SP, 0x20));
        } else {
            c.cycles += 2;
            let is_two = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is_two {
                c.cycles += 6;
                c.g[AT][0] = 0x4000_0000;
                c.mtc1(AT, 0);
                c.g[AT][0] = sx(0xc000_0000);
                c.mtc1(AT, 1);
                c.swc1(0, at(c, SP, 0x20));
                write_y_float = true;
            } else {
                c.cycles += 2;
                let is_three = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 4;
                if is_three {
                    c.cycles += 6;
                    c.g[AT][0] = 0x4000_0000;
                    c.mtc1(AT, 0);
                    c.sw(at(c, SP, 0x28), 0);
                    c.swc1(0, at(c, SP, 0x24));
                    c.swc1(0, at(c, SP, 0x20));
                    clear_z = false;
                } else {
                    c.cycles += 2;
                    let is_four = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 5;
                    if is_four {
                        c.cycles += 6;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 0);
                        c.g[AT][0] = sx(0xc000_0000);
                        c.mtc1(AT, 1);
                        c.swc1(0, at(c, SP, 0x20));
                        write_y_float = true;
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] { return }
                        c.cycles += 5;
                        c.g[AT][0] = 0x4000_0000;
                        c.mtc1(AT, 0);
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 1);
                        c.swc1(0, at(c, SP, 0x20));
                        write_y_float = true;
                    }
                }
            }
        }
    }
    if write_y_float {
        c.cycles += 1;
        c.swc1(1, at(c, SP, 0x24));
    }
    if clear_z {
        c.cycles += 1;
        c.sw(at(c, SP, 0x28), 0);
    }
    c.cycles += 3;
    c.sw(at(c, SP, 0x2c), 0);
    c.lq(V0, at(c, SP, 0x20));
    c.sq(V0, at(c, SP, 0x10));
}

/// `void Rider_ResetAtMatrix(Rider*, Matrix*)`.
///
/// Select Ground/substate0 and stop the transition, ground-place at the
/// marker plus native lane offset (animation5, clearance0), then restore the
/// pre-reset speed along the new rider forward vector. Replay, metrix,
/// camera-facing interface and FX notifications remain native. No path
/// cursor, checkpoint, reset-immunity or safe-placement state is written.
pub fn rider_reset_at_matrix(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xb0 };
        let mut pc = c.pc;
        loop {
            match pc {
                0x0012_3210 => {
                    c.cycles += 21;
                    Frame::open(c, 0xb0);
                    c.g[V1][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                    frame.save(c, S0, 0x90);
                    frame.save(c, S2, 0x70);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0x80);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save_ra(c, 0x60);
                    frame.save_fpr(c, 20, 0xa0);
                    c.g[V0][0] = c.lw(at(c, V1, 0x84));
                    c.g[A1][0] = c.lw(at(c, S2, 0x86c));
                    c.g[A0][0] = c.lw(at(c, V0, 0x28));
                    call_ptr(c, 0x0027_0970, 0x0012_3244)?;
                    pc = 0x0012_3244;
                }
                0x0012_3244 => {
                    lane_offset(c);
                    c.cycles += 106;
                    marker_vector(c, at(c, SP, 0x10), at(c, SP, 0x40));
                    c.g[AT][0] = 0x42c8_0000;
                    c.mtc1(AT, 0);
                    c.lq(V0, at(c, SP, 0x40));
                    c.g[A1][0] = 1;
                    c.sq(V0, at(c, SP, 0x30));
                    c.mfc1(V1, 0);
                    c.lqc2(4, at(c, SP, 0x30));
                    c.qmtc2(V1, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x40));
                    c.lq(V0, at(c, SP, 0x40));
                    c.sq(V0, at(c, SP, 0x20));
                    c.sq(V0, at(c, SP, 0x10));
                    c.g[V0][0] = 0x0050_0000;
                    c.lqc2(3, r(c, SP));
                    c.lqc2(4, at(c, SP, 0x10));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, r(c, SP));
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0xec0));
                    marker_vector(c, r(c, V0), at(c, SP, 0x30));
                    c.lq(V0, at(c, SP, 0x30));
                    c.g[A0][0] = c.lw(at(c, S2, 0x790));
                    c.sq(V0, at(c, SP, 0x20));
                    call_ptr(c, 0x0011_9368, 0x0012_33d4)?;
                    pc = 0x0012_33d4;
                }
                0x0012_33d4 => {
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0011_fe78, 0x0012_33e0)?;
                    pc = 0x0012_33e0;
                }
                0x0012_33e0 => {
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0011_fec8, 0x0012_33ec)?;
                    pc = 0x0012_33ec;
                }
                0x0012_33ec => {
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0012_5038, 0x0012_33f4)?;
                    pc = 0x0012_33f4;
                }
                0x0012_33f4 => {
                    c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                    c.g[A2][0] = sx(at(c, S2, 0x6c0));
                    c.g[V1][0] = c.lw(at(c, S2, 0x6c0));
                    c.g[V0][0] = c.lw(at(c, A0, 0x84));
                    c.g[A0][0] = c.lh(at(c, V1, 0x38));
                    c.g[A1][0] = c.lw(at(c, V0, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0x3c));
                    c.g[A0][0] = sx(r(c, A2).wrapping_add(r(c, A0)));
                    c.g[V1][0] = c.lw(at(c, A1, 4));
                    c.g[S0][0] = c.lw(at(c, V1, 0xa0));
                    c.g[S1][0] = c.lw(at(c, S0, 0x14));
                    c.g[V1][0] = c.lh(at(c, S1, 0x18));
                    let target = r(c, V0);
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
                    call_ptr(c, target, 0x0012_342c)?;
                    pc = 0x0012_342c;
                }
                0x0012_342c => {
                    c.g[V1][0] = c.lw(at(c, S1, 0x1c));
                    c.g[A1][0] = c.g[V0][0];
                    let target = r(c, V1);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, target, 0x0012_343c)?;
                    pc = 0x0012_343c;
                }
                0x0012_343c => {
                    c.mtc1(0, 12);
                    c.lqc2(3, at(c, S2, 0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(V0, 22);
                    c.mtc1(V0, 20);
                    c.g[A2][0] = sx(at(c, SP, 0x20));
                    c.g[A3][0] = 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x0011_d660, 0x0012_347c)?;
                    pc = 0x0012_347c;
                }
                0x0012_347c => {
                    c.lq(V0, at(c, S2, 0x1b0));
                    c.sq(V0, at(c, SP, 0x40));
                    c.mfc1(V1, 20);
                    c.lqc2(4, at(c, SP, 0x40));
                    c.qmtc2(V1, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, SP, 0x50));
                    c.lq(V0, at(c, SP, 0x50));
                    c.g[A0][0] = c.lw(at(c, S2, 0x77c));
                    c.sq(V0, at(c, S2, 0x1e0));
                    c.sq(V0, at(c, SP, 0x30));
                    call_ptr(c, 0x0011_1890, 0x0012_34ac)?;
                    pc = 0x0012_34ac;
                }
                0x0012_34ac => {
                    c.g[A0][0] = c.lw(at(c, S2, 0x88c));
                    call_ptr(c, 0x002f_1a00, 0x0012_34b4)?;
                    pc = 0x0012_34b4;
                }
                0x0012_34b4 => {
                    frame.restore(c, S0, 0x90);
                    frame.restore(c, S1, 0x80);
                    frame.restore(c, S2, 0x70);
                    frame.restore_ra(c, 0x60);
                    frame.restore_fpr(c, 20, 0xa0);
                    frame.close(c);
                    return Ok(());
                }
                _ => unreachable!("Rider_ResetAtMatrix resume at {pc:08x}"),
            }
        }
    })
}
