//! Exact native Rider_ProbeRail0x1086b8, retained uninstalled.
//!
//! No tests, cargo, replays or game runs: disabled by the user.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY, rider_probe_rail, "Rider_ProbeRail",
)];

const ENTRY: u32 = 0x0010_86b8;
const SEARCH_RETURN: u32 = 0x0010_89d0;
const END: u32 = 0x0010_8a44;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Reduce vf3*vf5 to vf4.x, preserving the original W lane and accumulator.
fn dot(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// The VU0 four-component norm of vf3, returning Q in V1/f0.
fn length(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
}

/// Build the body's forward axis and capsule from the current animation
/// node quaternion. t=Rider+25c interpolates near radius30..50cm and
/// half-length100..170cm, or50..170cm when Rider+330 is nonzero.
fn body_capsule(c: &mut Cpu) {
    c.g[T0][0] = c.g[A0][0];
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 9);
    c.g[A0][0] = c.lw(at(c, T0, 0x780));
    c.g[V0][0] = c.lw(at(c, T0, 0x8a0));
    c.g[V1][0] = c.lw(at(c, A0, 0x2c));
    c.g[V0][0] = sx(r(c, V0) << 5);
    c.g[AT][0] = 0x4248_0000;
    c.mtc1(AT, 10);
    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
    c.g[AT][0] = 0x41f0_0000;
    c.mtc1(AT, 8);
    c.lwc1(4, at(c, V1, 0x14));
    c.g[A0][0] = c.g[SP][0];
    c.lwc1(5, at(c, V1, 0x18));
    c.lwc1(0, at(c, V1, 0x10));
    c.mul_s(1, 4, 4);
    c.lwc1(3, at(c, V1, 0x1c));
    c.mul_s(7, 5, 5);
    c.mul_s(2, 0, 5);
    c.sw(at(c, SP, 0xc), 0);
    c.mul_s(6, 3, 4);
    c.add_s(1, 1, 7);
    c.mul_s(0, 0, 4);
    c.mul_s(3, 3, 5);
    c.sub_s(2, 2, 6);
    c.add_s(1, 1, 1);
    c.add_s(0, 0, 3);
    c.add_s(2, 2, 2);
    c.sub_s(1, 9, 1);
    c.add_s(0, 0, 0);
    c.swc1(2, at(c, SP, 8));
    c.swc1(1, r(c, SP));
    c.swc1(0, at(c, SP, 4));
    c.lqc2(3, r(c, V1));
    c.lqc2(4, at(c, T0, 0x9d0));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x20));
    c.lwc1(2, at(c, T0, 0x25c));
    c.lq(V0, at(c, SP, 0x20));
    c.sub_s(9, 9, 2);
    c.g[V1][0] = c.lw(at(c, T0, 0x330));
    c.mul_s(0, 2, 10);
    c.sq(V0, at(c, SP, 0x10));
    c.mul_s(8, 9, 8);
    let alternate_capsule = c.g[V1][0] != 0;
    c.add_s(20, 0, 8);
    if alternate_capsule {
        c.cycles += 5;
        c.g[AT][0] = 0x432a_0000;
        c.mtc1(AT, 0);
        c.mul_s(1, 9, 10);
        c.mul_s(0, 2, 0);
        c.add_s(0, 0, 1);
    } else {
        c.cycles += 8;
        c.g[AT][0] = 0x432a_0000;
        c.mtc1(AT, 1);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(AT, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(0, 9, 0);
        c.add_s(0, 1, 0);
    }
}

/// Closest point of the body segment to A1's candidate; true when already
/// within the near radius. f4/f8 retain the lower/upper axial clamps.
fn initial_distance(c: &mut Cpu) -> bool {
    c.cycles += 22;
    c.lqc2(3, r(c, A1));
    c.lqc2(4, at(c, SP, 0x10));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x30));
    c.sub_s(4, 20, 0);
    c.lq(V0, at(c, SP, 0x30));
    c.sub_s(8, 0, 20);
    c.sq(V0, at(c, SP, 0x20));
    c.mov_s(1, 4);
    c.lqc2(3, r(c, A0));
    c.lqc2(5, at(c, SP, 0x20));
    dot(c);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    c.c_le_s(4, 0);
    if c.fcc() { c.min_s(1, 0, 8); }
    c.cycles += 33;
    c.mfc1(V1, 1);
    c.lqc2(4, r(c, A0));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(c, SP, 0x50));
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x40));
    c.lqc2(3, at(c, SP, 0x10));
    c.lqc2(4, at(c, SP, 0x40));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x50));
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x30));
    c.lqc2(3, at(c, SP, 0x30));
    c.lqc2(4, r(c, A1));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x50));
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x20));
    c.lqc2(3, at(c, SP, 0x20));
    length(c);
    c.c_le_s(0, 20);
    let close = c.fcc();
    c.g[V0][0] = 1;
    close
}

/// Reject a nearly parallel tangent when the first distance failed.
fn nonparallel(c: &mut Cpu) -> bool {
    c.cycles += 18;
    c.lqc2(3, r(c, A2));
    c.lqc2(5, r(c, A0));
    dot(c);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 3);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2);
    c.mul_s(0, 3, 3);
    c.lwc1(1, r(c, GP).wrapping_sub(0x7e3c));
    c.sub_s(2, 2, 0);
    c.c_lt_s(2, 1);
    let parallel = c.fcc();
    c.g[V0][0] = 0;
    !parallel
}

/// Project the body segment against the candidate tangent and set up the
/// ordinary second rail query. No collision, surface or bounce gate occurs.
fn secondary_query(c: &mut Cpu) {
    c.cycles += 42;
    c.lqc2(3, at(c, SP, 0x10));
    c.lqc2(4, r(c, A1));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x50));
    c.mov_s(1, 4);
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x40));
    c.mfc1(V1, 2);
    c.qmtc2(V1, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, at(c, SP, 0x40));
    c.vu0().waitq();
    c.vu0().mul(0b1111, 4, 4, Src::Q);
    c.sqc2(4, at(c, SP, 0x50));
    c.lq(V0, at(c, SP, 0x50));
    c.sq(V0, at(c, SP, 0x30));
    c.mfc1(V1, 3);
    c.lqc2(4, r(c, A2));
    c.qmtc2(V1, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(c, SP, 0x60));
    c.lq(V0, at(c, SP, 0x60));
    c.sq(V0, at(c, SP, 0x50));
    c.lqc2(3, at(c, SP, 0x50));
    c.lqc2(4, r(c, A0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x60));
    c.lq(V0, at(c, SP, 0x60));
    c.sq(V0, at(c, SP, 0x40));
    c.lqc2(3, at(c, SP, 0x30));
    c.lqc2(5, at(c, SP, 0x40));
    dot(c);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_le_s(4, 0);
    if c.fcc() { c.min_s(1, 0, 8); }
    c.cycles += 22;
    c.mfc1(V0, 1);
    c.lqc2(4, r(c, A0));
    c.qmtc2(V0, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, at(c, SP, 0x60));
    c.g[AT][0] = 0x4396_0000;
    c.mtc1(AT, 12);
    c.lq(V0, at(c, SP, 0x60));
    c.g[A1][0] = sx(at(c, SP, 0x40));
    c.g[A2][0] = sx(at(c, SP, 0x70));
    c.g[A3][0] = 1;
    c.sq(V0, at(c, SP, 0x50));
    c.lqc2(3, at(c, SP, 0x10));
    c.lqc2(4, at(c, SP, 0x50));
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x60));
    c.lq(V0, at(c, SP, 0x60));
    c.g[A0][0] = c.lw(at(c, T0, 0x860));
    c.g[RA][0] = sx(SEARCH_RETURN);
    c.sq(V0, at(c, SP, 0x40));
}

fn search_result(c: &mut Cpu) {
    let absent = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if absent { return }
    c.cycles += 22;
    c.lqc2(3, at(c, SP, 0x40));
    c.lqc2(4, at(c, SP, 0x70));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, 0x60));
    c.g[A0][0] = 1;
    c.lq(V0, at(c, SP, 0x60));
    c.sq(V0, at(c, SP, 0x20));
    c.sq(V0, at(c, SP, 0x50));
    c.lqc2(3, at(c, SP, 0x20));
    length(c);
    c.c_le_s(0, 20);
    if !c.fcc() { c.g[A0][0] = 0; }
    c.cycles += 1;
    c.g[V0][0] = c.g[A0][0];
}

/// `bool Rider_ProbeRail(Rider* self, Vec4* candidate, Vec4* tangent)`.
/// Accept a candidate inside the animated body capsule, or re-query at the
/// closest body/tangent point and check that distance. There are no terrain
/// collision, object bounce, surface, minimum speed or world-section tests.
pub fn rider_probe_rail(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x110 };
        match c.pc {
            ENTRY => {
                c.cycles += 50;
                Frame::open(c, 0x110);
                // The original saves f20 and RA between T0=A0 and its
                // first quaternion load; these stack writes do not alias
                // any subsequently read source field.
                frame.save_fpr(c, 20, 0x100);
                frame.save_ra(c, 0xf0);
                body_capsule(c);
                if !initial_distance(c) && nonparallel(c) {
                    secondary_query(c);
                    call_ptr(c, 0x0033_4680, SEARCH_RETURN)?;
                    search_result(c);
                }
            }
            SEARCH_RETURN => search_result(c),
            pc => { c.jump_out(pc, ENTRY, END); return Ok(()) }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0xf0);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    });
}
