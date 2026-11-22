//! Native WScriptFn_AddRollerModifier (0x002fd250), retained uninstalled.
//!
//! Generated EE is authoritative. No tests, cargo, game or replays were
//! added or run, in accordance with the user's instruction.

use crate::call::{call_ptr, guest, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    ENTRY,
    add_roller_modifier,
    "WScriptFn_AddRollerModifier",
)];

const ENTRY: u32 = 0x002f_d250;
const AFTER_COPY: u32 = 0x002f_d2e4;
const ARGUMENT_LOOP: u32 = 0x002f_d300;
const RESOLVE: u32 = 0x002f_d348;
const AFTER_PREDICATE: u32 = 0x002f_d3c4;
const AFTER_ADD: u32 = 0x002f_d3e0;
const FINISH: u32 = 0x002f_d3e4;
const END: u32 = 0x002f_d420;
const DEFAULTS: u32 = 0x004f_b628;
const PARAM_TYPES: u32 = 0x0044_60a0;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }

#[inline]
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* AddRollerModifier(Vec3* out, int count, NamedArg* args)`.
/// Named fields 0..3 have native tags [int,float,float,float], initialized
/// to [-1,1.0,0.7,0.5]. Copy all 20 bytes, including the trailing opaque
/// byte, then apply the native unchecked argument indices/conversions.
/// The RID selects an existing instance or -1 selects manager+0x290.
/// Its node must answer true to adjusted virtual slot +0x80. This wrapper
/// does not create a node. It calls ModObj_AddRollerModifier(node,params,
/// manager.contactA,manager.contactB), preserving null contact pointers.
/// No parameter units or semantics beyond these four fields are inferred.
pub fn add_roller_modifier(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    Frame::open(c, 0x80);
                    c.g[V0][0] = c.lw(at(c, GP, 0x2510));
                    frame.save(c, S2, 0x50);
                    frame.save(c, S0, 0x70);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, S3, 0x40);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0x60);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save_ra(c, 0x30);
                    let ready = c.g[V0][0] != 0;
                    c.g[A2][0] = 0x0050_0000;
                    if !ready {
                        c.cycles += 17;
                        c.g[V1][0] = sx(DEFAULTS);
                        c.g[V0][0] = 0xffff_ffff_ffff_0000;
                        c.g[A0][0] = c.lbu(at(c, V1, 0x10));
                        c.g[V0][0] |= 0xffff;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(AT, 2);
                        c.g[A1][0] = 1;
                        c.lwc1(0, r(c, GP).wrapping_sub(0x3388));
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 1);
                        c.sw(r(c, A2).wrapping_sub(0x49d8), c.g[V0][0]);
                        c.sb(at(c, V1, 0x10), c.g[A0][0]);
                        c.swc1(2, at(c, V1, 4));
                        c.swc1(0, at(c, V1, 8));
                        c.swc1(1, at(c, V1, 12));
                        c.sw(at(c, GP, 0x2510), c.g[A1][0]);
                        c.sb(r(c, SP), c.g[A0][0]);
                    }
                    c.cycles += 10;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S1][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x49d8));
                    c.g[A2][0] = 0x14;
                    c.sw(at(c, SP, 0x10), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x003e_6574, AFTER_COPY)?;
                    pc = AFTER_COPY;
                }
                AFTER_COPY => {
                    let no_arguments = (c.g[S0][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S2][0];
                    if no_arguments {
                        pc = RESOLVE;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[T0][0] = sx(PARAM_TYPES);
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
                        pc = ARGUMENT_LOOP;
                    } else {
                        pc = RESOLVE;
                    }
                }
                RESOLVE => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(r(c, S1));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, 0x16c8));
                    if !explicit {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, 0xce8));
                        c.g[V0][0] = c.lw(at(c, V0, 0x290));
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.g[A2][0] & 0xff;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V1][0] = c.lw(r(c, A1));
                        c.g[A0][0] = c.lw(at(c, V1, 8));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        let absent_bank = c.g[V1][0] == 0;
                        c.g[V0][0] = sx(r(c, A2) >> 8);
                        let mut present = false;
                        if !absent_bank {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            present = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                        }
                        if !present {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                    }
                    c.cycles += 3;
                    c.g[S0][0] = c.lw(at(c, V0, 12));
                    let no_node = c.g[S0][0] == 0;
                    c.g[V1][0] = 0x004d_0000;
                    if no_node {
                        pc = FINISH;
                    } else {
                        c.cycles += 7;
                        c.g[V1][0] = c.lw(at(c, S0, 12));
                        c.g[A0][0] = c.lh(at(c, V1, 0x80));
                        c.g[V0][0] = c.lw(at(c, V1, 0x84));
                        let predicate = r(c, V0);
                        c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                        call_ptr(c, predicate, AFTER_PREDICATE)?;
                        pc = AFTER_PREDICATE;
                    }
                }
                AFTER_PREDICATE => {
                    let unsupported = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(at(c, GP, 0xce8));
                    if !unsupported {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A3][0] = c.lw(at(c, V0, 0x294));
                        c.g[A2][0] = c.lw(at(c, V0, 0x298));
                        call_ptr(c, 0x0035_5db8, AFTER_ADD)?;
                    }
                    pc = AFTER_ADD;
                }
                AFTER_ADD => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x004d_0000;
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 15;
                    c.g[V0][0] = c.g[S3][0];
                    c.g[A3][0] = sx(r(c, V1).wrapping_sub(0x6f68));
                    c.ldl(A0, at(c, A3, 7));
                    c.ldr(A0, r(c, A3));
                    c.g[A1][0] = c.lw(at(c, A3, 8));
                    c.sdl(A0, at(c, S3, 7));
                    c.sdr(A0, r(c, S3));
                    c.sw(at(c, S3, 8), c.g[A1][0]);
                    frame.restore(c, S0, 0x70);
                    frame.restore(c, S1, 0x60);
                    frame.restore(c, S2, 0x50);
                    frame.restore(c, S3, 0x40);
                    frame.restore_ra(c, 0x30);
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
