//! Native MultiSplineModifier_ctor0x00359f88, retained uninstalled.
//!
//! Generated EE owns registers, cycles, memory and call order. The user's
//! automated-test ban takes precedence over the normal port validation loop.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x0035_9f88;
const AFTER_INIT_PATH: u32 = 0x0035_9ff8;
const AFTER_SET_SPEED: u32 = 0x0035_a004;
const AFTER_ALLOC_ARRAYS: u32 = 0x0035_a0bc;
const AFTER_ALLOC_INSTANCES: u32 = 0x0035_a0c4;
const AFTER_CLONE_INSTANCES: u32 = 0x0035_a0cc;
const AFTER_UPDATE_NODES: u32 = 0x0035_a0d4;
const AFTER_INSERT_INSTANCES: u32 = 0x0035_a0dc;
const END: u32 = 0x0035_a118;

const COUNT: u32 = 0x04;
const MODE: u32 = 0x08;
const YAW_RADIANS: u32 = 0x0c;
const DISTANCE: u32 = 0x10;
const DRAW_ROUTE: u32 = 0x1c;
const ROUTE_COLOUR: u32 = 0x20;
const SOURCE_INSTANCE: u32 = 0x40;
const PATH: u32 = 0x48;
const PATH_LENGTH: u32 = 0x54;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `MultiSplineModifier* ctor(self, const Params* descriptor, Instance* source)`.
///
/// Keep the raw source instance at+40; copy descriptor+2c count and+8 mode,
/// convert+10 yaw degrees to radians, and load the path selected by+4.
/// SetSpeed receives descriptor+c. Positive/zero speed starts at distance0;
/// negative speed starts at the loaded path length, unless nonzero+14 supplies
/// a valid explicit distance. Copy+18..24 route RGBA and normalize+28's route
/// visibility flag. Native helpers allocate arrays and node instances, clone
/// the source, update their transforms and insert them into the native spatial
/// system. The constructor itself does not translate the descriptor's mode
/// enum or allocate its own modifier object; it retains a raw Instance pointer.
pub fn multi_spline_modifier_ctor(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 37;
                    Frame::open(c, frame.size);
                    c.g[V0][0] = 0x0049_0000;
                    frame.save(c, S0, 0x30);
                    c.g[V1][0] = 0xffff_ffff_ffff_0000;
                    frame.save(c, S1, 0x20);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save_ra(c, 0x10);
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(0xe98));
                    c.g[V1][0] |= 0xffff;
                    c.sw(at(c, S0, SOURCE_INSTANCE), c.g[A2][0]);
                    c.sw(r(c, S0), c.g[V0][0]);
                    c.g[S1][0] = c.g[A1][0];
                    c.sw(at(c, S0, PATH), c.g[V1][0]);
                    c.g[A0][0] = sx(at(c, S0, PATH));
                    c.sw(at(c, S0, 0x50), 0);
                    c.sw(at(c, S0, 0x4c), 0);
                    c.sw(at(c, S0, PATH_LENGTH), 0);
                    c.sw(at(c, S0, 0x18), 0);
                    c.lwc1(1, r(c, GP).wrapping_sub(0x2a2c));
                    c.g[V0][0] = c.lw(at(c, S1, 0x2c));
                    c.sw(at(c, S0, COUNT), c.g[V0][0]);
                    c.g[V1][0] = c.lw(at(c, S1, 0x08));
                    c.sw(at(c, S0, MODE), c.g[V1][0]);
                    c.lwc1(0, at(c, S1, 0x10));
                    c.mul_s(0, 0, 1);
                    c.swc1(0, at(c, S0, YAW_RADIANS));
                    c.g[A1][0] = c.lw(at(c, S1, 0x04));
                    call_ptr(c, 0x0034_51c0, AFTER_INIT_PATH)?;
                    pc = AFTER_INIT_PATH;
                }
                AFTER_INIT_PATH => {
                    c.lwc1(12, at(c, S1, 0x0c));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_a550, AFTER_SET_SPEED)?;
                    pc = AFTER_SET_SPEED;
                }
                AFTER_SET_SPEED => {
                    c.mtc1(ZERO, 1);
                    c.lwc1(0, at(c, S1, 0x0c));
                    c.c_le_s(1, 0);
                    if c.fcc() {
                        c.cycles += 2;
                        c.swc1(1, at(c, S0, DISTANCE));
                    } else {
                        c.lwc1(0, at(c, S0, PATH_LENGTH));
                        c.cycles += 1;
                        c.swc1(0, at(c, S0, DISTANCE));
                    }
                    c.cycles += 25;
                    c.g[V0][0] = c.lw(at(c, S1, 0x28));
                    c.mtc1(ZERO, 4);
                    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                    c.sw(at(c, S0, DRAW_ROUTE), c.g[V0][0]);
                    for register in 0..4 {
                        c.lwc1(register, at(c, S1, 0x18 + register as u32 * 4));
                    }
                    for register in 0..4 {
                        c.swc1(register, at(c, SP, register as u32 * 4));
                    }
                    c.ldl(V0, at(c, SP, 0x07));
                    c.ldr(V0, r(c, SP));
                    c.ldl(V1, at(c, SP, 0x0f));
                    c.ldr(V1, at(c, SP, 0x08));
                    c.sdl(V0, at(c, S0, ROUTE_COLOUR + 0x07));
                    c.sdr(V0, at(c, S0, ROUTE_COLOUR));
                    c.sdl(V1, at(c, S0, ROUTE_COLOUR + 0x0f));
                    c.sdr(V1, at(c, S0, ROUTE_COLOUR + 0x08));
                    c.lwc1(0, at(c, S1, 0x14));
                    c.c_eq_s(0, 4);
                    if !c.fcc() {
                        c.swc1(0, at(c, S0, DISTANCE));
                    }
                    c.cycles += 5;
                    c.lwc1(1, at(c, S0, DISTANCE));
                    c.c_lt_s(1, 4);
                    if c.fcc() {
                        c.swc1(4, at(c, S0, DISTANCE));
                    } else {
                        c.cycles += 5;
                        c.lwc1(0, at(c, S0, PATH_LENGTH));
                        c.c_lt_s(0, 1);
                        if c.fcc() {
                            c.swc1(4, at(c, S0, DISTANCE));
                        }
                    }
                    c.cycles += 15;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_a328, AFTER_ALLOC_ARRAYS)?;
                    pc = AFTER_ALLOC_ARRAYS;
                }
                AFTER_ALLOC_ARRAYS => {
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_a3f0, AFTER_ALLOC_INSTANCES)?;
                    pc = AFTER_ALLOC_INSTANCES;
                }
                AFTER_ALLOC_INSTANCES => {
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_a458, AFTER_CLONE_INSTANCES)?;
                    pc = AFTER_CLONE_INSTANCES;
                }
                AFTER_CLONE_INSTANCES => {
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_ac20, AFTER_UPDATE_NODES)?;
                    pc = AFTER_UPDATE_NODES;
                }
                AFTER_UPDATE_NODES => {
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0035_a5f8, AFTER_INSERT_INSTANCES)?;
                    pc = AFTER_INSERT_INSTANCES;
                }
                AFTER_INSERT_INSTANCES => {
                    c.g[V1][0] = c.lw(at(c, S0, SOURCE_INSTANCE));
                    c.g[V0][0] = c.lw(at(c, V1, 0x08));
                    c.g[V0][0] &= 0x100;
                    let source_flag_100 = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if source_flag_100 {
                        c.cycles += 2;
                        c.sw(at(c, S0, 0x34), c.g[V0][0]);
                    } else {
                        c.cycles += 1;
                        c.sw(at(c, S0, 0x34), 0);
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.g[S0][0];
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S0, 0x30);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, ENTRY, END);
                    return Ok(());
                }
            }
        }
    });
}
