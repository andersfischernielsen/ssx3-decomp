//! Native cBoardWakeFX_BuildTrail0x2ddd30, retained uninstalled.
//!
//! No tests, builds, replays or game runs: the user disabled them.
//! Generated EE defines register, scratch-memory and cycle behavior.

use crate::call::{guest, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const NOISE_RETURN: u32 = 0x002d_dd5c;
const POINT_LOOP: u32 = 0x002d_de88;
const RING_LOOP: u32 = 0x002d_dfe8;
const NOISE: Callee = Callee {
    addr: 0x002d_1928,
    f: ssx3_ee::s09::Noise_Fractal1D,
};

#[inline]
fn r(c: &Cpu, reg: usize) -> u32 { c.g[reg][0] as u32 }

#[inline]
fn at(c: &Cpu, reg: usize, offset: u32) -> u32 {
    r(c, reg).wrapping_add(offset)
}

/// `void cBoardWakeFX_BuildTrail(cBoardWakeFX*, Vec4* pos, float width)`.
///
/// Build one 0x70-byte wake element from the direction vectors at
/// self+0x60/+0x70/+0x80, and seed the associated 0x30-byte ring vertices.
/// The only color component written here is integer alpha at ring+0x1c,
/// converted from self+0x94*128. RGB at ring+0x10/+0x14/+0x18 and texture
/// state belong to other routines and remain untouched by this function.
pub fn board_wake_fx_build_trail(c: &mut Cpu) {
    guest(c, |c| {
        // The generated entry charges the entire first block before Noise.
        c.cycles += 83;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x80);
        c.mov_s(20, 12);
        c.g[A0][0] = 3;
        c.g[RA][0] = sx(NOISE_RETURN);
        c.lwc1(12, at(c, S0, 0x38));
        NOISE.call(c, NOISE_RETURN)?;

        // Element=head*0x70. Reset its age and form its drift vector from
        // rider velocity and the stored self+0x80 direction.
        c.g[V0][0] = c.lw(at(c, S0, 8));
        c.g[V1][0] = 0x70;
        c.g[A1][0] = c.lw(at(c, S0, 0xc));
        c.g[A0][0] = 0x500000;
        c.mult(0, V0, V0, V1);
        c.lwc1(3, at(c, S0, 0x90));
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(AT, 1);
        c.g[A0][0] = sx(r(c, A0).wrapping_sub(0xee0));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(AT, 2);
        c.g[A2][0] = 0;
        c.mul_s(0, 0, 1);
        c.lwc1(4, r(c, GP).wrapping_sub(0x3be4));
        c.g[T1][0] = sx(r(c, A1).wrapping_add(r(c, V0)));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.sw(at(c, T1, 0x64), 0);
        c.add_s(0, 0, 1);
        c.lq(V0, r(c, A0));
        c.sq(V0, r(c, T1));
        c.mul_s(3, 3, 0);
        c.mfc1(A0, 2);
        c.lqc2(4, at(c, S0, 0x80));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x20));
        c.lq(V1, frame.at(c, 0x20));
        c.g[V0][0] = c.lw(r(c, S0));
        c.sq(V1, frame.at(c, 0x10));
        c.lqc2(3, at(c, V0, 0x1e0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(A0, 22);
        c.mtc1(A0, 0);
        c.mfc1(V1, 0);
        c.lqc2(4, frame.at(c, 0x10));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x20));
        c.lq(V0, frame.at(c, 0x20));
        c.g[V1][0] = c.lw(r(c, S0));
        c.sq(V0, frame.at(c, 0));
        c.mfc1(A0, 2);
        c.lqc2(4, at(c, V1, 0x1e0));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x20));
        c.lq(V0, frame.at(c, 0x20));
        c.sq(V0, frame.at(c, 0x10));
        c.lqc2(3, frame.at(c, 0));
        c.lqc2(4, frame.at(c, 0x10));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x40));
        c.lq(V0, frame.at(c, 0x40));
        c.sq(V0, frame.at(c, 0x30));
        c.mfc1(V1, 4);
        c.lqc2(4, frame.at(c, 0x30));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x40));
        c.lq(V0, frame.at(c, 0x40));
        c.sq(V0, frame.at(c, 0x20));
        c.sq(V0, at(c, T1, 0x50));
        c.g[V1][0] = c.lw(at(c, S0, 0xb4));
        let no_points = (c.g[V1][0] as i64) <= 0;
        c.g[A0][0] = c.g[T1][0];
        if !no_points {
            c.cycles += 3;
            c.lwc1(4, r(c, GP).wrapping_sub(0x3be0));
            c.g[AT][0] = 0x3f40_0000;
            c.mtc1(AT, 2);
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(1));
                c.mtc1(A2, 1);
                c.cvt_s_w(1, 1);
                c.mtc1(V0, 0);
                c.cvt_s_w(0, 0);
                c.div_s(1, 1, 0);
                let last_point = c.g[A2][0] == c.g[V0][0];
                c.mul_s(0, 1, 1);
                if last_point {
                    c.cycles += 1;
                    c.mul_s(1, 1, 4);
                }

                c.cycles += 40;
                c.mfc1(V0, 0);
                c.lqc2(4, at(c, S0, 0x70));
                c.qmtc2(V0, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x60));
                c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
                c.lq(V0, frame.at(c, 0x60));
                c.sq(V0, frame.at(c, 0x50));
                c.mfc1(V1, 2);
                c.lqc2(4, frame.at(c, 0x50));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(V0, frame.at(c, 0x60));
                c.sq(V0, frame.at(c, 0x40));
                c.mfc1(V1, 1);
                c.lqc2(4, at(c, S0, 0x60));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x70));
                c.lq(V0, frame.at(c, 0x70));
                c.sq(V0, frame.at(c, 0x60));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x60));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x70));
                c.lq(V0, frame.at(c, 0x70));
                c.sq(V0, frame.at(c, 0x30));
                c.mfc1(V1, 3);
                c.lqc2(4, frame.at(c, 0x30));
                c.qmtc2(V1, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x70));
                c.lq(V0, frame.at(c, 0x70));
                c.sq(V0, frame.at(c, 0x20));
                c.sq(V0, r(c, A0));
                c.g[V1][0] = c.lw(at(c, S0, 0xb4));
                c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx(at(c, A0, 0x10));
                if !more { break; }
                if event_due(c) { deliver_event_here(c, POINT_LOOP); }
            }
        }

        // Seed ring0 at the input position, offset from rider+0x370 by5.
        c.cycles += 37;
        c.g[V1][0] = c.lw(at(c, S0, 0x10));
        c.g[A1][0] = 0x30;
        c.g[A0][0] = c.lw(at(c, S0, 8));
        c.g[A3][0] = 1;
        c.g[V0][0] = c.lw(r(c, S0));
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 0);
        c.mult(0, A0, A0, A1);
        c.mfc1(A2, 0);
        c.lqc2(4, at(c, V0, 0x370));
        c.qmtc2(A2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, 0x40));
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(AT, 1);
        c.lq(V0, frame.at(c, 0x40));
        c.sq(V0, frame.at(c, 0x30));
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
        c.lqc2(3, r(c, S1));
        c.lqc2(4, frame.at(c, 0x30));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x40));
        c.lq(V0, frame.at(c, 0x40));
        c.sq(V0, frame.at(c, 0x20));
        c.sq(V0, at(c, A0, 0x20));
        c.g[V1][0] = c.lw(at(c, S0, 8));
        c.lwc1(0, at(c, S0, 0x94));
        c.mult(0, V1, V1, A1);
        c.g[V0][0] = c.lw(at(c, S0, 0x10));
        c.mul_s(0, 0, 1);
        c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
        c.cvt_w_s(1, 0);
        c.swc1(1, at(c, V1, 0x1c));
        c.g[V0][0] = c.lw(at(c, S0, 0xb4));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[T0][0] = 0x30;
            c.g[A2][0] = sx(at(c, S0, 0x14));
            loop {
                // Duplicate position and alpha to every remaining ring.
                c.cycles += 21;
                c.g[V1][0] = c.lw(at(c, S0, 8));
                c.g[A3][0] = sx(r(c, A3).wrapping_add(1));
                c.g[A0][0] = c.lw(at(c, S0, 0x10));
                c.mult(0, V1, V1, T0);
                c.g[A1][0] = c.lw(r(c, A2));
                c.g[A0][0] = sx(r(c, V1).wrapping_add(r(c, A0)));
                c.lq(V0, at(c, A0, 0x20));
                c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, A1)));
                c.sq(V0, at(c, V1, 0x20));
                c.g[A0][0] = c.lw(at(c, S0, 8));
                c.g[V0][0] = c.lw(at(c, S0, 0x10));
                c.mult(0, A0, A0, T0);
                c.g[A1][0] = c.lw(r(c, A2));
                c.g[V0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
                c.g[V1][0] = c.lw(at(c, V0, 0x1c));
                c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, A1)));
                c.sw(at(c, A0, 0x1c), c.g[V1][0]);
                c.g[V0][0] = c.lw(at(c, S0, 0xb4));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(at(c, A2, 4));
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) { deliver_event_here(c, RING_LOOP); }
            }
        }
        c.swc1(20, at(c, T1, 0x60));

        c.cycles += 6;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}
