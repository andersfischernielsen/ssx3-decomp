//! Native AddParticleData factory (0x002fe840), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds, game runs or replays
//! were added or run: the user explicitly disabled automated validation.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    world_obj_factory_add_particle_data,
    "WorldObjFactory_AddParticleData",
)];

const ENTRY: u32 = 0x002f_e840;
const END: u32 = 0x002f_ee98;
const PARAM_TYPES: u32 = 0x0044_62a0;
const DEFAULTS_READY_GP: u32 = 0x2538;
const INSTANCE_RID: u32 = 0xd8;
const INSTANCE_NODE: u32 = 0xc;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// Issue the original four-component VU distance calculation. The native
/// factory has two copies: an existing actor uses stack+10 as its position,
/// while an unowned instance supplies its copied position through S2.
fn viewer_distance(c: &mut Cpu, owned: bool) {
    let position = if owned { at(c, SP, 0x10) } else { r(c, S2) };
    let difference = if owned { 0x30 } else { 0x20 };
    let copy = if owned { 0x20 } else { 0x10 };
    c.lqc2(3, position);
    c.lqc2(4, r(c, V0));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, at(c, SP, difference));
    c.lq(V0, at(c, SP, difference));
    c.sq(V0, at(c, SP, copy));
    c.lqc2(3, at(c, SP, copy));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    c.c_lt_s(0, 20);
    c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
}

/// Set the native RiderInterface virtual position getter arguments for the
/// next viewer. Both list variants leave identical virtual-call registers;
/// the main-rider list is at AI+40, the actor list at AI+70.
fn viewer_arguments(c: &mut Cpu, actors: bool) {
    if actors {
        c.g[A1][0] = sx(r(c, S0) << 2);
        c.g[V1][0] = c.lw(at(c, A0, 0x84));
        c.g[V0][0] = c.lw(at(c, V1, 0xc));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A1)));
    } else {
        c.g[V1][0] = c.lw(at(c, A1, 0x84));
        c.g[A0][0] = sx(r(c, S0) << 2);
        c.g[V0][0] = c.lw(at(c, V1, 0xc));
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
    }
    c.g[V1][0] = c.lw(at(c, V0, if actors { 0x70 } else { 0x40 }));
    c.g[A0][0] = c.lw(at(c, V1, 0x18));
    c.g[A1][0] = c.lw(at(c, A0, 0x6c0));
    c.g[A0][0] = sx(at(c, A0, 0x6c0));
    c.g[V1][0] = c.lh(at(c, A1, 0x28));
    c.g[V0][0] = c.lw(at(c, A1, 0x2c));
    c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V1)));
}

/// `Vec3* AddParticleData(Vec3* out, int count, NamedArg* args)`.
/// Allocate/copy the 0xe0-byte native descriptor and apply its unchecked named
/// arguments. Native types: 0,1,49,50,52,54 are int; all other indices0..54
/// are float. Param54 is the instance RID (-1 selects the script owner).
/// The factory clamps float7 to at most float5, but otherwise forwards the
/// descriptor intact to the native particle modifier or standalone actor.
/// Extra field semantics are owned by that consumer, not this wrapper.
/// In editor mode2 it also requires an existing viewer within30000cm.
pub fn world_obj_factory_add_particle_data(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xd0 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 16;
                    Frame::open(c, 0xd0);
                    c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
                    frame.save(c, S0, 0xb0);
                    frame.save(c, S1, 0xa0);
                    c.g[S0][0] = 0x0050_0000;
                    frame.save(c, S2, 0x90);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save(c, S5, 0x60);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, S3, 0x80);
                    c.g[S5][0] = c.g[A0][0];
                    frame.save(c, S4, 0x70);
                    frame.save(c, S6, 0x50);
                    frame.save_ra(c, 0x40);
                    frame.save_fpr(c, 20, 0xc0);
                    pc = if c.g[V0][0] != 0 { 0x002f_e9a0 } else { 0x002f_e880 };
                }
                0x002f_e880 => {
                    c.cycles += 72;
                    c.g[AT][0] = sx(0xbf80_0000);
                    c.mtc1(AT, 1);
                    c.g[V0][0] = sx(r(c, S0).wrapping_sub(0x47a8));
                    c.g[AT][0] = 0x4080_0000;
                    c.mtc1(AT, 2);
                    c.g[A0][0] = 1;
                    c.mtc1(ZERO, 0);
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(AT, 3);
                    c.g[A2][0] = 0x10;
                    c.swc1(1, at(c, V0, 8));
                    c.g[V1][0] |= 0xffff;
                    c.swc1(2, at(c, V0, 0x10));
                    c.sw(r(c, S0).wrapping_sub(0x47a8), c.g[A0][0]);
                    c.sw(at(c, V0, 4), 0);
                    c.swc1(3, at(c, V0, 0xc));
                    for offset in (0x14..=0x80).step_by(4) {
                        c.swc1(0, at(c, V0, offset));
                    }
                    c.g[AT][0] = 0x41a0_0000;
                    c.mtc1(AT, 1);
                    c.swc1(3, at(c, V0, 0x84));
                    c.g[A1][0] = c.lbu(at(c, V0, 0xdc));
                    c.swc1(3, at(c, V0, 0xb4));
                    c.sw(at(c, V0, 0xc4), c.g[A2][0]);
                    c.swc1(0, at(c, V0, 0xcc));
                    c.swc1(1, at(c, V0, 0xd4));
                    c.sw(at(c, V0, INSTANCE_RID), c.g[V1][0]);
                    c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
                    for offset in [0x88, 0x8c, 0x90] {
                        c.swc1(0, at(c, V0, offset));
                    }
                    for offset in (0x94..=0xa4).step_by(4) {
                        c.swc1(3, at(c, V0, offset));
                    }
                    for offset in [0xa8, 0xac, 0xb0, 0xb8, 0xbc, 0xc0] {
                        c.swc1(0, at(c, V0, offset));
                    }
                    c.sw(at(c, V0, 0xc8), 0);
                    c.sw(at(c, V0, 0xd0), c.g[A0][0]);
                    c.sb(r(c, SP), c.g[A1][0]);
                    pc = 0x002f_e9a0;
                }
                0x002f_e9a0 => {
                    c.cycles += 17;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A0][0] = 0xe0;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x68c0));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[S6][0] = 0x004d_0000;
                    call_ptr(c, 0x0031_7d70, 0x002f_e9bc)?;
                    pc = 0x002f_e9bc;
                }
                0x002f_e9bc => {
                    c.g[S3][0] = c.g[V0][0];
                    c.g[A2][0] = 0xe0;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[A1][0] = sx(r(c, S0).wrapping_sub(0x47a8));
                    c.g[V0][0] |= 0xffff;
                    c.g[A0][0] = c.g[S3][0];
                    c.sw(at(c, S3, INSTANCE_RID), c.g[V0][0]);
                    call_ptr(c, 0x003e_6574, 0x002f_e9dc)?;
                    pc = 0x002f_e9dc;
                }
                0x002f_e9dc => {
                    c.g[A2][0] = c.g[S2][0];
                    if (c.g[S1][0] as i64) <= 0 {
                        pc = 0x002f_ea40;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[T0][0] = sx(PARAM_TYPES);
                        c.g[A3][0] = 2;
                        pc = 0x002f_e9f8;
                    }
                }
                0x002f_e9f8 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S3)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                        c.cycles += 1;
                        c.sw(r(c, A0), c.g[V0][0]);
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(at(c, A2, 4));
                            c.cycles += 1;
                            c.sw(r(c, A0), c.g[V0][0]);
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, at(c, A2, 4));
                            c.cvt_s_w(0, 0);
                            c.swc1(0, r(c, A0));
                        }
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 0x10));
                    if more {
                        if back_edge_unwinds(c, 0x002f_e9f8) { return Err(Unwound) }
                    } else { pc = 0x002f_ea40; }
                }
                0x002f_ea40 => {
                    c.cycles += 6;
                    c.lwc1(0, at(c, S3, 0x1c));
                    c.lwc1(1, at(c, S3, 0x14));
                    c.c_lt_s(1, 0);
                    if c.fcc() { c.swc1(1, at(c, S3, 0x1c)); }
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(at(c, S3, INSTANCE_RID));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, 0x16c8));
                    if explicit {
                        pc = 0x002f_ea74;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, 0xce8));
                        c.g[S4][0] = c.lw(at(c, V0, 0x290));
                        pc = 0x002f_eac4;
                    }
                }
                0x002f_ea74 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.g[A2][0] & 255;
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[V1][0] = c.lw(r(c, A1));
                    c.g[A0][0] = c.lw(at(c, V1, 8));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                    c.g[V1][0] = c.lw(r(c, V0));
                    if c.g[V1][0] != 0 {
                        c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                        c.cycles += 7;
                        c.g[V0][0] = sx(r(c, A2) >> 8);
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        c.g[V0][0] = sx(r(c, V1) >> 8);
                        let reference = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        if !reference { c.cycles += 1; c.g[V0][0] = 0; }
                        c.cycles += 1;
                        c.g[A2][0] = c.g[V0][0];
                    } else {
                        c.cycles += 2;
                        c.g[A2][0] = 0;
                    }
                    c.cycles += 1;
                    c.g[S4][0] = c.g[A2][0];
                    pc = 0x002f_eac4;
                }
                0x002f_eac4 => {
                    c.cycles += 3;
                    c.g[S2][0] = c.lw(at(c, S4, INSTANCE_NODE));
                    if c.g[S2][0] == 0 {
                        c.lq(V0, at(c, S4, 0x40));
                        pc = 0x002f_ec90;
                    } else {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw(at(c, S2, 0xc));
                        c.g[A0][0] = c.lh(at(c, V1, 0x80));
                        c.g[V0][0] = c.lw(at(c, V1, 0x84));
                        let target = r(c, V0);
                        c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                        call_ptr(c, target, 0x002f_eae4)?;
                        pc = 0x002f_eae4;
                    }
                }
                0x002f_eae4 => {
                    if c.g[V0][0] == 0 { pc = 0x002f_ee44; continue }
                    c.cycles += 12;
                    c.g[V1][0] = c.lw(at(c, S2, 0xc));
                    c.g[A0][0] = c.lh(at(c, V1, 0xc0));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc4));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S2).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x002f_eb00)?;
                    pc = 0x002f_eb00;
                }
                0x002f_eb00 => {
                    c.lq(V1, at(c, V0, 0x30));
                    c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x84c));
                    c.sq(V1, at(c, SP, 0x10));
                    c.g[V0][0] = c.lw(at(c, A0, 0x550));
                    c.g[V0][0] ^= 2;
                    let editor = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                    pc = if editor { 0x002f_eb24 } else { 0x002f_eb1c };
                }
                0x002f_eb1c => {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    pc = 0x002f_ec78;
                }
                0x002f_eb24 => {
                    c.cycles += 6;
                    c.g[S0][0] = 0;
                    c.g[A0][0] = c.lw(at(c, V0, 0x84));
                    c.g[V1][0] = c.lw(at(c, A0, 0xc));
                    c.g[S1][0] = c.lw(at(c, V1, 0x7c));
                    let empty = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = c.g[V0][0];
                    if empty { pc = 0x002f_ebcc; } else {
                        c.cycles += 1;
                        c.lwc1(20, r(c, GP).wrapping_sub(0x337c));
                        pc = 0x002f_eb40;
                    }
                }
                0x002f_eb40 => {
                    c.cycles += 32;
                    viewer_arguments(c, false);
                    let target = r(c, V0);
                    call_ptr(c, target, 0x002f_eb70)?;
                    pc = 0x002f_eb70;
                }
                0x002f_eb70 => {
                    viewer_distance(c, true);
                    if c.fcc() {
                        if back_edge_unwinds(c, 0x002f_eb1c) { return Err(Unwound) }
                        pc = 0x002f_eb1c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A1][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                        if more {
                            if back_edge_unwinds(c, 0x002f_eb40) { return Err(Unwound) }
                            pc = 0x002f_eb40;
                        } else { pc = 0x002f_ebcc; }
                    }
                }
                0x002f_ebcc => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, A1, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[S1][0] = c.lw(at(c, V0, 0x88));
                    let empty = (c.g[S1][0] as i64) <= 0;
                    c.g[S0][0] = 0;
                    if empty {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        pc = 0x002f_ec78;
                    } else {
                        c.cycles += 2;
                        c.lwc1(20, r(c, GP).wrapping_sub(0x3378));
                        c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                        pc = 0x002f_ebe8;
                    }
                }
                0x002f_ebe8 => {
                    c.cycles += 32;
                    viewer_arguments(c, true);
                    let target = r(c, V0);
                    call_ptr(c, target, 0x002f_ec18)?;
                    pc = 0x002f_ec18;
                }
                0x002f_ec18 => {
                    viewer_distance(c, true);
                    if c.fcc() {
                        if back_edge_unwinds(c, 0x002f_eb1c) { return Err(Unwound) }
                        pc = 0x002f_eb1c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                        if more {
                            if back_edge_unwinds(c, 0x002f_ebe8) { return Err(Unwound) }
                            pc = 0x002f_ebe8;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                            pc = 0x002f_ec78;
                        }
                    }
                }
                0x002f_ec78 => {
                    c.cycles += 2;
                    let unavailable = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if unavailable { pc = 0x002f_ee44; } else {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, 0x0035_5cb0, 0x002f_ec88)?;
                        pc = 0x002f_ec88;
                    }
                }
                0x002f_ec88 => { pc = 0x002f_ee4c; }
                0x002f_ec90 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x84c));
                    c.sq(V0, r(c, SP));
                    c.g[V1][0] = c.lw(at(c, A0, 0x550));
                    c.g[V1][0] ^= 2;
                    let editor = c.g[V1][0] == 0;
                    c.g[S2][0] = c.g[SP][0];
                    pc = if editor { 0x002f_ecb0 } else { 0x002f_eca8 };
                }
                0x002f_eca8 => {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    pc = 0x002f_ee08;
                }
                0x002f_ecb0 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                    c.g[S0][0] = 0;
                    c.g[A0][0] = c.lw(at(c, V0, 0x84));
                    c.g[V1][0] = c.lw(at(c, A0, 0xc));
                    c.g[S1][0] = c.lw(at(c, V1, 0x7c));
                    let empty = (c.g[S1][0] as i64) <= 0;
                    c.g[A1][0] = c.g[V0][0];
                    if empty { pc = 0x002f_ed5c; } else {
                        c.cycles += 1;
                        c.lwc1(20, r(c, GP).wrapping_sub(0x3374));
                        pc = 0x002f_ecd0;
                    }
                }
                0x002f_ecd0 => {
                    c.cycles += 32;
                    viewer_arguments(c, false);
                    let target = r(c, V0);
                    call_ptr(c, target, 0x002f_ed00)?;
                    pc = 0x002f_ed00;
                }
                0x002f_ed00 => {
                    viewer_distance(c, false);
                    if c.fcc() {
                        if back_edge_unwinds(c, 0x002f_eca8) { return Err(Unwound) }
                        pc = 0x002f_eca8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A1][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                        if more {
                            if back_edge_unwinds(c, 0x002f_ecd0) { return Err(Unwound) }
                            pc = 0x002f_ecd0;
                        } else { pc = 0x002f_ed5c; }
                    }
                }
                0x002f_ed5c => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(at(c, A1, 0x84));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc));
                    c.g[S1][0] = c.lw(at(c, V0, 0x88));
                    let empty = (c.g[S1][0] as i64) <= 0;
                    c.g[S0][0] = 0;
                    if empty {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        pc = 0x002f_ee08;
                    } else {
                        c.cycles += 2;
                        c.lwc1(20, r(c, GP).wrapping_sub(0x3370));
                        c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                        pc = 0x002f_ed78;
                    }
                }
                0x002f_ed78 => {
                    c.cycles += 32;
                    viewer_arguments(c, true);
                    let target = r(c, V0);
                    call_ptr(c, target, 0x002f_eda8)?;
                    pc = 0x002f_eda8;
                }
                0x002f_eda8 => {
                    viewer_distance(c, false);
                    if c.fcc() {
                        if back_edge_unwinds(c, 0x002f_eca8) { return Err(Unwound) }
                        pc = 0x002f_eca8;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = c.lw(r(c, GP).wrapping_sub(0x848));
                        if more {
                            if back_edge_unwinds(c, 0x002f_ed78) { return Err(Unwound) }
                            pc = 0x002f_ed78;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                            pc = 0x002f_ee08;
                        }
                    }
                }
                0x002f_ee08 => {
                    c.cycles += 2;
                    let unavailable = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x0049_0000;
                    if unavailable { pc = 0x002f_ee44; } else {
                        c.cycles += 13;
                        c.g[A0][0] = 0x2c;
                        c.g[A1][0] = sx(r(c, A1).wrapping_sub(0x68d0));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x002f_ee24)?;
                        pc = 0x002f_ee24;
                    }
                }
                0x002f_ee24 => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[T0][0] = 0;
                    call_ptr(c, 0x0035_78a8, 0x002f_ee3c)?;
                    pc = 0x002f_ee3c;
                }
                0x002f_ee3c => { pc = 0x002f_ee4c; }
                0x002f_ee44 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0031_7e50, 0x002f_ee4c)?;
                    pc = 0x002f_ee4c;
                }
                0x002f_ee4c => {
                    c.cycles += 19;
                    c.g[A1][0] = sx(r(c, S6).wrapping_sub(0x6f68));
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.g[V1][0] = c.lw(at(c, A1, 8));
                    c.sdl(V0, at(c, S5, 7));
                    c.sdr(V0, r(c, S5));
                    c.sw(at(c, S5, 8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S5][0];
                    frame.restore(c, S0, 0xb0);
                    frame.restore(c, S1, 0xa0);
                    frame.restore(c, S2, 0x90);
                    frame.restore(c, S3, 0x80);
                    frame.restore(c, S4, 0x70);
                    frame.restore(c, S5, 0x60);
                    frame.restore(c, S6, 0x50);
                    frame.restore_ra(c, 0x40);
                    frame.restore_fpr(c, 20, 0xc0);
                    frame.close(c);
                    return Ok(());
                }
                _ => { c.jump_out(pc, ENTRY, END); return Ok(()); }
            }
        }
    });
}
