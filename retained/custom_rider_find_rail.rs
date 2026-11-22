//! Native Rider_FindRail0x108a48, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds, replays or game runs
//! were added or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY, rider_find_rail, "Rider_FindRail",
)];

const ENTRY: u32 = 0x0010_8a48;
const END: u32 = 0x0010_8c24;
const STATE_CONTEXT: u32 = 0x784;
const OVERLAP_LIST: u32 = 0x860;
const VELOCITY: u32 = 0x1e0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `bool Rider_FindRail(Rider* rider, RailHit* out)`.
/// Accept native main state0/1, search the rider's overlap list with mask1
/// and a300cm radius, reject a candidate behind the moving rider or blocked
/// by task2's flag policy, then invoke Rider_ProbeRail. This entry itself
/// neither selects a world section nor changes spline eligibility/geometry.
pub fn rider_find_rail(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 9;
                    Frame::open(c, 0x60);
                    frame.save(c, S0, 0x50);
                    frame.save(c, S1, 0x40);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x30);
                    c.g[S1][0] = c.g[A1][0];
                    call_ptr(c, 0x0011_fe98, 0x0010_8a64)?;
                    pc = 0x0010_8a64;
                }
                0x0010_8a64 => {
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.lw(at(c, S0, 0x780));
                        pc = 0x0010_8a84;
                    } else { pc = 0x0010_8a6c; }
                }
                0x0010_8a6c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0011_fe98, 0x0010_8a74)?;
                    pc = 0x0010_8a74;
                }
                0x0010_8a74 => {
                    c.g[V1][0] = 1;
                    let rejected = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0;
                    pc = if rejected { 0x0010_8c10 } else { 0x0010_8a80 };
                }
                0x0010_8a80 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(at(c, S0, 0x780));
                    pc = 0x0010_8a84;
                }
                0x0010_8a84 => {
                    c.cycles += 19;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw(at(c, S0, 0x8a0));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V1][0] = c.lw(at(c, A0, 0x2c));
                    c.g[A3][0] = 1;
                    c.g[V0][0] = sx(r(c, V0) << 5);
                    c.g[AT][0] = 0x4396_0000;
                    c.mtc1(AT, 12);
                    c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                    c.lqc2(3, r(c, V1));
                    c.lqc2(4, at(c, S0, 0x9d0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x10));
                    c.lq(V0, at(c, SP, 0x10));
                    c.g[A0][0] = c.lw(at(c, S0, OVERLAP_LIST));
                    c.sq(V0, r(c, SP));
                    call_ptr(c, 0x0033_4680, 0x0010_8ac8)?;
                    pc = 0x0010_8ac8;
                }
                0x0010_8ac8 => {
                    let missing = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    pc = if missing { 0x0010_8c10 } else { 0x0010_8ad0 };
                }
                0x0010_8ad0 => {
                    c.cycles += 15;
                    c.lwc1(0, r(c, GP).wrapping_sub(0x7e38));
                    c.lqc2(3, at(c, S0, VELOCITY));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(V0, 22);
                    c.mtc1(V0, 2);
                    c.c_lt_s(0, 2);
                    if !c.fcc() {
                        c.g[A0][0] = c.lw(at(c, S0, STATE_CONTEXT));
                        pc = 0x0010_8b64;
                    } else { pc = 0x0010_8b0c; }
                }
                0x0010_8b0c => {
                    c.cycles += 21;
                    c.lqc2(3, r(c, S1));
                    c.lqc2(4, r(c, SP));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, at(c, SP, 0x20));
                    c.lwc1(1, r(c, GP).wrapping_sub(0x7e34));
                    c.lq(V0, at(c, SP, 0x20));
                    c.mul_s(1, 2, 1);
                    c.sq(V0, at(c, SP, 0x10));
                    c.lqc2(3, at(c, SP, 0x10));
                    c.lqc2(5, at(c, S0, VELOCITY));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(V1, 4);
                    c.mtc1(V1, 0);
                    c.c_lt_s(0, 1);
                    let behind = c.fcc();
                    c.g[V0][0] = 0;
                    pc = if behind { 0x0010_8c10 } else { 0x0010_8b60 };
                }
                0x0010_8b60 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(at(c, S0, STATE_CONTEXT));
                    pc = 0x0010_8b64;
                }
                0x0010_8b64 => {
                    c.cycles += 5;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_1ae8, 0x0010_8b6c)?;
                    pc = 0x0010_8b6c;
                }
                0x0010_8b6c => {
                    c.g[V1][0] = 0x12;
                    let other_task = c.g[V0][0] != c.g[V1][0];
                    c.g[A0][0] = c.lw(at(c, S0, STATE_CONTEXT));
                    pc = if other_task { 0x0010_8bbc } else { 0x0010_8b78 };
                }
                0x0010_8b78 => {
                    c.cycles += 7;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_1b20, 0x0010_8b80)?;
                    pc = 0x0010_8b80;
                }
                0x0010_8b80 => {
                    c.g[A0][0] = sx(at(c, V0, 0xb0));
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0014_46a0, 0x0010_8b8c)?;
                    pc = 0x0010_8b8c;
                }
                0x0010_8b8c => {
                    let allowed = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    pc = if allowed { 0x0010_8c04 } else { 0x0010_8b94 };
                }
                0x0010_8b94 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw(at(c, S0, STATE_CONTEXT));
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_1b20, 0x0010_8ba0)?;
                    pc = 0x0010_8ba0;
                }
                0x0010_8ba0 => {
                    c.g[A0][0] = sx(at(c, V0, 0xb0));
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_46a0, 0x0010_8bac)?;
                    pc = 0x0010_8bac;
                }
                0x0010_8bac => {
                    let unblocked = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    pc = if unblocked { 0x0010_8c04 } else { 0x0010_8bb4 };
                }
                0x0010_8bb4 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0010_8c10;
                }
                0x0010_8bbc => {
                    c.cycles += 5;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_1ae8, 0x0010_8bc4)?;
                    pc = 0x0010_8bc4;
                }
                0x0010_8bc4 => {
                    c.g[V1][0] = 0x13;
                    let special = c.g[V0][0] == c.g[V1][0];
                    c.g[A0][0] = c.lw(at(c, S0, STATE_CONTEXT));
                    pc = if special { 0x0010_8be8 } else { 0x0010_8bd0 };
                }
                0x0010_8bd0 => {
                    c.cycles += 5;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_1ae8, 0x0010_8bd8)?;
                    pc = 0x0010_8bd8;
                }
                0x0010_8bd8 => {
                    c.g[V1][0] = 0x14;
                    let ordinary = c.g[V0][0] != c.g[V1][0];
                    c.g[A1][0] = c.g[S1][0];
                    pc = if ordinary { 0x0010_8c04 } else { 0x0010_8be4 };
                }
                0x0010_8be4 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(at(c, S0, STATE_CONTEXT));
                    pc = 0x0010_8be8;
                }
                0x0010_8be8 => {
                    c.cycles += 7;
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_1b20, 0x0010_8bf0)?;
                    pc = 0x0010_8bf0;
                }
                0x0010_8bf0 => {
                    c.g[A0][0] = sx(at(c, V0, 0xb0));
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0014_46a0, 0x0010_8bfc)?;
                    pc = 0x0010_8bfc;
                }
                0x0010_8bfc => {
                    let blocked = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    if blocked {
                        if back_edge_unwinds(c, 0x0010_8bb4) { return Err(Unwound) }
                        pc = 0x0010_8bb4;
                    } else { pc = 0x0010_8c04; }
                }
                0x0010_8c04 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx(at(c, A1, 0x10));
                    call_ptr(c, 0x0010_86b8, 0x0010_8c10)?;
                    pc = 0x0010_8c10;
                }
                0x0010_8c10 => {
                    c.cycles += 5;
                    frame.restore(c, S0, 0x50);
                    frame.restore(c, S1, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
