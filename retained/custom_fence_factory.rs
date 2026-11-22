//! Native builtin10 / WScriptFn_Modifier9, retained uninstalled.
//!
//! Generated EE owns the register/cycle/call contract. Automated tests and
//! game runs are disabled by the user; no override is installed here.

use crate::call::{call_ptr, guest, Frame, Unwound};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const ENTRY: u32 = 0x002f_cc20;
const COPY_PARAMETERS: u32 = 0x002f_cc88;
const AFTER_COPY: u32 = 0x002f_ccac;
const ARGUMENT_LOOP: u32 = 0x002f_ccc8;
const RESOLVE_INSTANCE: u32 = 0x002f_cd10;
const ATTACH: u32 = 0x002f_cd6c;
const FINISH: u32 = 0x002f_cd88;
const END: u32 = 0x002f_cdc8;

const DEFAULTS: u32 = 0x004f_b5b0;
const DEFAULTS_READY_GP: u32 = 0x2504;
const PARAMETER_TYPES: u32 = 0x0044_6028;
const SCRIPT_MANAGER_GP: u32 = 0x0ce8;
const WORLD_GP: u32 = 0x16c8;
const CURRENT_INSTANCE: u32 = 0x290;
const INSTANCE_NODE: u32 = 0x0c;

fn r(c: &Cpu, register: usize) -> u32 { c.g[register][0] as u32 }
fn at(c: &Cpu, register: usize, offset: u32) -> u32 {
    r(c, register).wrapping_add(offset)
}

/// `Vec3* WScriptFn_Modifier9(Vec3* out, int count, NamedArg* args)`.
/// Native schema `[int,int,float]` at446028, defaults `[-1,0,0.5]`.
/// RID0 selects a resident instance (-1 selects the script owner). Only
/// an existing node receives AttachModifier(node,9,parameterBlock). This
/// factory allocates no node and reads no contact record. The copied
/// fourth word remains opaque; initialization reads/rewrites its first byte
/// unchanged and mirrors that byte into the temporary stack record.
pub fn wscript_fn_modifier9(c: &mut Cpu) {
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
                    frame.save(c, S1, 0x60);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S2, 0x50);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x70);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save(c, S4, 0x30);
                    c.g[A2][0] = 0x0050_0000;
                    let initialized = c.g[V0][0] != 0;
                    frame.save_ra(c, 0x20);
                    if !initialized {
                        c.cycles += 13;
                        c.g[V1][0] = sx(DEFAULTS);
                        c.g[V0][0] = 0xffff_ffff_ffff_0000;
                        c.g[A1][0] = c.lbu(at(c, V1, 0xc));
                        c.g[V0][0] |= 0xffff;
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(AT, 0);
                        c.g[A0][0] = 1;
                        c.sw(DEFAULTS, c.g[V0][0]);
                        c.sb(at(c, V1, 0xc), c.g[A1][0]);
                        c.swc1(0, at(c, V1, 8));
                        c.sw(at(c, GP, DEFAULTS_READY_GP), c.g[A0][0]);
                        c.sw(at(c, V1, 4), 0);
                        c.sb(r(c, SP), c.g[A1][0]);
                    }
                    pc = COPY_PARAMETERS;
                }
                COPY_PARAMETERS => {
                    c.cycles += 11;
                    c.g[V0][0] = 0xffff_ffff_ffff_0000;
                    c.g[S0][0] = sx(at(c, SP, 0x10));
                    c.g[V0][0] |= 0xffff;
                    c.g[A1][0] = sx(r(c, A2).wrapping_sub(0x4a50));
                    c.g[A2][0] = 0x10;
                    c.sw(at(c, SP, 0x10), c.g[V0][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S4][0] = 0x004d_0000;
                    call_ptr(c, 0x003e_6574, AFTER_COPY)?;
                    pc = AFTER_COPY;
                }
                AFTER_COPY => {
                    let no_arguments = (c.g[S2][0] as i64) <= 0;
                    c.g[A2][0] = c.g[S3][0];
                    if no_arguments {
                        pc = RESOLVE_INSTANCE;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = 0x0044_0000;
                        c.g[A1][0] = c.g[S2][0];
                        c.g[T0][0] = sx(PARAMETER_TYPES);
                        c.g[A3][0] = 2;
                        pc = ARGUMENT_LOOP;
                    }
                }
                ARGUMENT_LOOP => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(r(c, A2));
                    c.g[V1][0] = c.lw(at(c, A2, 0xc));
                    c.g[V0][0] = sx(r(c, V0) << 2);
                    c.g[A0][0] = sx(r(c, V0).wrapping_add(r(c, S0)));
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, T0)));
                    c.g[V0][0] = c.lw(r(c, V0));
                    let mut integer_store = true;
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
                            integer_store = false;
                        }
                    }
                    if integer_store {
                        c.cycles += 1;
                        c.sw(r(c, A0), c.g[V0][0]);
                    }
                    c.cycles += 3;
                    c.g[A1][0] = sx(r(c, A1).wrapping_sub(1));
                    let more = c.g[A1][0] != 0;
                    c.g[A2][0] = sx(at(c, A2, 0x10));
                    if more {
                        if back_edge_unwinds(c, ARGUMENT_LOOP) { return Err(Unwound) }
                    } else {
                        pc = RESOLVE_INSTANCE;
                    }
                }
                RESOLVE_INSTANCE => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(r(c, S0));
                    c.g[V0][0] = !c.g[A2][0];
                    let explicit = c.g[V0][0] != 0;
                    c.g[A1][0] = c.lw(at(c, GP, WORLD_GP));
                    if !explicit {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c, GP, SCRIPT_MANAGER_GP));
                        c.g[V0][0] = c.lw(at(c, V0, CURRENT_INSTANCE));
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.g[A2][0] & 0xff;
                        c.g[V0][0] = sx(r(c, V0) << 2);
                        c.g[V1][0] = c.lw(r(c, A1));
                        c.g[A0][0] = c.lw(at(c, V1, 8));
                        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                        c.g[V1][0] = c.lw(r(c, V0));
                        let missing_bank = c.g[V1][0] == 0;
                        c.g[V0][0] = sx(r(c, A2) >> 8);
                        let mut live = false;
                        if !missing_bank {
                            c.cycles += 7;
                            c.g[A0][0] = c.lw(at(c, V1, 0x1c));
                            c.g[V0][0] = sx(r(c, V0) << 2);
                            c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, A0)));
                            c.g[V1][0] = c.lw(r(c, V0));
                            c.g[V0][0] = sx(r(c, V1) >> 8);
                            live = c.g[V0][0] != 0;
                            c.g[V0][0] = sx(r(c, V0) << 2);
                        }
                        if !live {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                        }
                    }
                    pc = ATTACH;
                }
                ATTACH => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw(at(c, V0, INSTANCE_NODE));
                        let no_node = c.g[A0][0] == 0;
                        c.g[A2][0] = c.g[S0][0];
                        if !no_node {
                            c.cycles += 2;
                            c.g[A1][0] = 9;
                            call_ptr(c, 0x002f_ae38, FINISH)?;
                        }
                    }
                    pc = FINISH;
                }
                FINISH => {
                    c.cycles += 16;
                    c.g[A1][0] = sx(r(c, S4).wrapping_sub(0x6f68));
                    c.ldl(V0, at(c, A1, 7));
                    c.ldr(V0, r(c, A1));
                    c.g[V1][0] = c.lw(at(c, A1, 8));
                    c.sdl(V0, at(c, S1, 7));
                    c.sdr(V0, r(c, S1));
                    c.sw(at(c, S1, 8), c.g[V1][0]);
                    c.g[V0][0] = c.g[S1][0];
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
    })
}
