//! Native WorldObjFactory_Counter (0x002fb8d0), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game runs or replays
//! were added or run, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    world_obj_factory_counter,
    "WorldObjFactory_Counter",
)];

const ENTRY: u32 = 0x002f_b8d0;
const AFTER_DEFAULT_COPY: u32 = 0x002f_b954;
const ARGUMENT_LOOP: u32 = 0x002f_b968;
const RESOLVE_INSTANCE: u32 = 0x002f_b9b0;
const CREATE_COUNTER: u32 = 0x002f_ba1c;
const AFTER_ATTACH: u32 = 0x002f_ba38;
const ALLOCATE_COUNTER: u32 = 0x002f_ba40;
const AFTER_ALLOCATE: u32 = 0x002f_ba58;
const FINISH: u32 = 0x002f_ba6c;
const END: u32 = 0x002f_bab0;
const DEFAULTS: u32 = 0x004f_b468;
const DEFAULTS_READY_GP: u32 = 0x24e0;
const PARAM_TYPES_GP: u32 = 0x0af0;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;
const WORLD_GP: u32 = 0x16c8;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* WorldObjFactory_Counter(Vec3* out, int count, NamedArg* args)`.
/// Param0 is an instance RID (-1 selects the script's current instance),
/// param1 the counter seed. Native types are [int,int], defaults [-1,3].
/// Copy all 12 bytes, including the original opaque trailing byte, before
/// applying the unchecked named argument indices. Existing nodes first get
/// AttachModifier(kind 7,params); otherwise allocate 0x24 and construct the
/// native Counter with WorldObj kind 7. Return the shared native Vec3 through
/// out. Keep each native continuation and the argument-loop event boundary.
pub fn world_obj_factory_counter(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    Frame::open(c, 0x80);
                    c.g[V0][0] = c.lw(at(c, GP, DEFAULTS_READY_GP));
                    frame.save(c, S3, 0x40);
                    frame.save(c, S0, 0x70);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S2, 0x50);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0x60);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S4, 0x30);
                    c.g[A2][0] = 0x0050_0000;
                    frame.save_ra(c, 0x20);
                    if c.g[V0][0] == 0 {
                        c.cycles += 11;
                        c.g[V1][0] = sx(DEFAULTS);
                        c.g[V0][0] = 0xffff_ffff_ffff_0000;
                        c.g[A1][0] = c.lbu(at(c, V1, 8));
                        c.g[V0][0] |= 0xffff;
                        c.sw(DEFAULTS, c.g[V0][0]);
                        c.g[A0][0] = 3;
                        c.sb(at(c, V1, 8), c.g[A1][0]);
                        c.g[V0][0] = 1;
                        c.sw(at(c, V1, 4), c.g[A0][0]);
                        c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[V0][0]);
                        c.sb(r(c, SP), c.g[A1][0]);
                    }
                    c.cycles += 11;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S1][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4b98));
                    c.g[A2][0] = 12;
                    c.sw(at(c, SP, 0x10), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S4][0] = 0x004d_0000;
                    call_ptr(c, 0x003e_6574, AFTER_DEFAULT_COPY)?;
                    pc = AFTER_DEFAULT_COPY;
                }
                AFTER_DEFAULT_COPY => {
                    let no_arguments = (c.g[S0][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S3][0];
                    if no_arguments {
                        pc = RESOLVE_INSTANCE;
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[T0][0] = sx(at(c, GP, PARAM_TYPES_GP));
                        c.g[A3][0] = 2;
                        pc = ARGUMENT_LOOP;
                    }
                }
                ARGUMENT_LOOP => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 12));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S1)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    let mut converted = false;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw(at(c, A2, 4));
                    } else {
                        c.cycles += 2;
                        if c.g[V0][0] != c.g[A3][0] {
                            c.g[V0][0] = c.lw(at(c, A2, 4));
                        } else {
                            c.cycles += 4;
                            c.lwc1(0, at(c, A2, 4));
                            c.cvt_s_w(0, 0);
                            c.swc1(0, r(c, A0));
                            converted = true;
                        }
                    }
                    if !converted {
                        c.cycles += 1;
                        c.sw(r(c, A0), c.g[V0][0]);
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 16));
                    if more {
                        if event_due(c) { deliver_event_here(c, ARGUMENT_LOOP); }
                    } else {
                        pc = RESOLVE_INSTANCE;
                    }
                }
                RESOLVE_INSTANCE => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(r(c, S1));
                    c.g[V0][0] = !c.g[A2][0];
                    c.g[A1][0] = c.lw(at(c, GP, WORLD_GP));
                    if c.g[V0][0] == 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
                        c.g[S0][0] = c.lw(at(c, V0, 0x290));
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.g[A2][0] & 255;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V1][0] = c.lw(r(c, A1));
                        c.g[A0][0] = c.lw(at(c, V1, 8));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        if c.g[V1][0] == 0 {
                            c.cycles += 2;
                            c.g[A2][0] = 0;
                        } else {
                            c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                            c.cycles += 7;
                            c.g[V0][0] = sx(r(c, A2) >> 8);
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            let present = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            if !present {
                                c.cycles += 1;
                                c.g[V0][0] = 0;
                            }
                            c.cycles += 1;
                            c.g[A2][0] = c.g[V0][0];
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[A2][0];
                    }
                    pc = CREATE_COUNTER;
                }
                CREATE_COUNTER => {
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        pc = FINISH;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(at(c, S0, 12));
                        let no_node = c.g[A0][0] == 0;
                        c.g[A1][0] = 7;
                        if no_node {
                            pc = ALLOCATE_COUNTER;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x002f_ae38, AFTER_ATTACH)?;
                            pc = AFTER_ATTACH;
                        }
                    }
                }
                AFTER_ATTACH => {
                    pc = if c.g[V0][0] != 0 { FINISH } else { ALLOCATE_COUNTER };
                }
                ALLOCATE_COUNTER => {
                    c.cycles += 11;
                    c.g[A1][0] = 0x004a_0000;
                    c.g[A0][0] = 0x24;
                    c.g[A1][0] = sx(at(c, A1, 0x3be8));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, 0x0031_7d70, AFTER_ALLOCATE)?;
                    pc = AFTER_ALLOCATE;
                }
                AFTER_ALLOCATE => {
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[S1][0];
                    call_ptr(c, 0x0034_3768, FINISH)?;
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 16;
                    c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.g[V1][0] = c.lw(at(c, A1, 8));
                    c.sdl(V0, at(c, S2, 7));
                    c.sdr(V0, r(c, S2));
                    c.sw(at(c, S2, 8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S2][0];
                    frame.restore(c, S0, 0x70);
                    frame.restore(c, S1, 0x60);
                    frame.restore(c, S2, 0x50);
                    frame.restore(c, S3, 0x40);
                    frame.restore(c, S4, 0x30);
                    frame.restore_ra(c, 0x20);
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
