//! Native ModObj_TakeOverObject0x355f10, retained uninstalled.
//!
//! Generated EE is authoritative. No tests, builds, game runs or replays
//! were added or executed, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    mod_obj_take_over_object,
    "ModObj_TakeOverObject",
)];

const ENTRY: u32 = 0x0035_5f10;
const AFTER_MODIFIER_QUERY: u32 = 0x0035_5f40;
const AFTER_PARTICLE_TRANSFER: u32 = 0x0035_5f54;
const GET_SOURCE_MATRIX: u32 = 0x0035_5f58;
const AFTER_SOURCE_MATRIX: u32 = 0x0035_5f68;
const AFTER_STATIC_DESTROY: u32 = 0x0035_5f90;
const AFTER_MOVING_DESTROY: u32 = 0x0035_5fd8;
const AFTER_POSITION_MODIFIER: u32 = 0x0035_5fec;
const RESTORE_SOURCE: u32 = 0x0035_6008;
const EPILOGUE: u32 = 0x0035_600c;
const END: u32 = 0x0035_601c;
const VTABLE: u32 = 0xc;
const INSTANCE: u32 = 0x18;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `void ModObj_TakeOverObject(ModObj* self, WorldObj* source)`.
/// A null source is a no-op. Otherwise transfer any native particle list,
/// obtain the source's current world matrix, then destroy that source. If
/// the matrix is the destination instance's static frame there is nothing
/// more to do. For a separate moving frame, preserve the destination's flags
/// across destruction, install a native position modifier from a VU copy of
/// the frame, mark ownership at +0x28 and invoke destination virtual slot50.
/// No Instance is cloned and no new visibility/collision flag is set here.
pub fn mod_obj_take_over_object(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 7;
                    Frame::open(c, 0x70);
                    frame.save(c, S0, 0x60);
                    frame.save(c, S1, 0x50);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save_ra(c, 0x40);
                    let no_source = c.g[S0][0] == 0;
                    c.g[S1][0] = c.g[A0][0];
                    if no_source {
                        pc = RESTORE_SOURCE;
                    } else {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw(at(c, S0, VTABLE));
                        c.g[A0][0] = c.lh(at(c, V1, 0x80));
                        c.g[V0][0] = c.lw(at(c, V1, 0x84));
                        let target = r(c, V0);
                        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                        call_ptr(c, target, AFTER_MODIFIER_QUERY)?;
                        pc = AFTER_MODIFIER_QUERY;
                    }
                }
                AFTER_MODIFIER_QUERY => {
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.lw(at(c, S0, VTABLE));
                        pc = GET_SOURCE_MATRIX;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0035_6020, AFTER_PARTICLE_TRANSFER)?;
                        pc = AFTER_PARTICLE_TRANSFER;
                    }
                }
                AFTER_PARTICLE_TRANSFER => {
                    c.g[V1][0] = c.lw(at(c, S0, VTABLE));
                    pc = GET_SOURCE_MATRIX;
                }
                GET_SOURCE_MATRIX => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lh(at(c, V1, 0xc0));
                    c.g[V0][0] = c.lw(at(c, V1, 0xc4));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, AFTER_SOURCE_MATRIX)?;
                    pc = AFTER_SOURCE_MATRIX;
                }
                AFTER_SOURCE_MATRIX => {
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCE));
                    c.g[V0][0] = sx(at(c, V0, 0x10));
                    let moving_frame = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = 3;
                    if !moving_frame {
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(at(c, S0, VTABLE));
                        c.g[A0][0] = c.lh(at(c, V0, 8));
                        c.g[V1][0] = c.lw(at(c, V0, 12));
                        let target = r(c, V1);
                        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                        call_ptr(c, target, AFTER_STATIC_DESTROY)?;
                        pc = AFTER_STATIC_DESTROY;
                    } else {
                        c.cycles += 28;
                        c.lqc2(1, r(c, V1));
                        c.lqc2(2, at(c, V1, 0x10));
                        c.lqc2(3, at(c, V1, 0x20));
                        c.lqc2(4, at(c, V1, 0x30));
                        c.sqc2(1, r(c, SP));
                        c.sqc2(2, at(c, SP, 0x10));
                        c.sqc2(3, at(c, SP, 0x20));
                        c.sqc2(4, at(c, SP, 0x30));
                        c.g[V0][0] = c.lw(at(c, S0, VTABLE));
                        c.g[A1][0] = 3;
                        c.g[A2][0] = c.lw(at(c, S1, INSTANCE));
                        c.g[A0][0] = c.lh(at(c, V0, 8));
                        c.g[V1][0] = c.lw(at(c, V0, 12));
                        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                        let target = r(c, V1);
                        c.g[S0][0] = c.lw(at(c, A2, 8));
                        call_ptr(c, target, AFTER_MOVING_DESTROY)?;
                        pc = AFTER_MOVING_DESTROY;
                    }
                }
                AFTER_STATIC_DESTROY => {
                    // This branch omits the common one-cycle S0 restore.
                    frame.restore(c, S0, 0x60);
                    pc = EPILOGUE;
                }
                AFTER_MOVING_DESTROY => {
                    c.g[V0][0] = c.lw(at(c, S1, INSTANCE));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.sw(at(c, V0, 8), c.g[S0][0]);
                    call_ptr(c, 0x0035_5918, AFTER_POSITION_MODIFIER)?;
                    pc = AFTER_POSITION_MODIFIER;
                }
                AFTER_POSITION_MODIFIER => {
                    c.g[V0][0] = 1;
                    c.g[V1][0] = c.lw(at(c, S1, VTABLE));
                    c.sw(at(c, S1, 0x28), c.g[V0][0]);
                    c.g[A0][0] = c.lh(at(c, V1, 0x190));
                    c.g[V0][0] = c.lw(at(c, V1, 0x194));
                    let target = r(c, V0);
                    c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
                    call_ptr(c, target, RESTORE_SOURCE)?;
                    pc = RESTORE_SOURCE;
                }
                RESTORE_SOURCE => {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    pc = EPILOGUE;
                }
                EPILOGUE => {
                    c.cycles += 4;
                    frame.restore(c, S1, 0x50);
                    frame.restore_ra(c, 0x40);
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
